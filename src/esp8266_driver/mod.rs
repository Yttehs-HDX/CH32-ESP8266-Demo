use ch32_hal::{
    mode::Async,
    usart::{Instance, Uart, UartRx, UartTx},
};
use command::*;
use embassy_futures::select::{select, Either};
use embassy_time::Timer;
use heapless::{String, Vec};

mod command;
pub mod error;

const BUF_SIZE: usize = 256;

pub struct Esp8266Driver<'d, T: Instance> {
    rx: UartRx<'d, T, Async>,
    tx: UartTx<'d, T, Async>,
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub fn new(uart: Uart<'d, T, Async>) -> Self {
        let (tx, rx) = uart.split();
        Esp8266Driver { rx, tx }
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn send_raw_command(&mut self, command: &[u8]) -> Result<(), Error> {
        self.tx
            .write(command)
            .await
            .map_err(|e| Error::Tx(error::TxError::Write(e)))
    }

    pub async fn read_raw_response(
        &mut self,
        timeout_ms: u64,
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        let timeout = Timer::after_millis(timeout_ms);

        let mut buf = [0u8; BUF_SIZE];
        let read_future = self.rx.read_until_idle(&mut buf);

        let len = match select(timeout, read_future).await {
            Either::First(_) => return Err(Error::Rx(error::RxError::Timeout)),
            Either::Second(res) => res.map_err(|e| Error::Rx(error::RxError::Read(e)))?,
        };

        let vec = Vec::from_slice(&buf)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        let string = String::from_utf8(vec)
            .map_err(|_| Error::StringConversion(error::StringConversionError::Utf8Conversion))?;

        Ok((string, len))
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn send_command(&mut self, command: &[u8]) -> Result<(), Error> {
        let mut cmd = String::<BUF_SIZE>::new();
        let cmd_str = core::str::from_utf8(command)
            .map_err(|_| Error::StringConversion(error::StringConversionError::Utf8Conversion))?;
        cmd.push_str(cmd_str)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        cmd.push_str("\r\n")
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;

        let len = cmd.chars().filter(|&c| c != '\0').count();
        self.send_raw_command(cmd[..len].as_bytes()).await
    }

    pub async fn read_response(
        &mut self,
        timeout_ms: u64,
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        let (raw_response, raw_len) = self.read_raw_response(timeout_ms).await?;

        let response = &raw_response[..raw_len];
        let response = response.trim_matches(|c| c == '\r' || c == '\n');

        let mut string = String::<BUF_SIZE>::new();
        string
            .push_str(response)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        string = string.chars().filter(|&c| c != '\r').collect();

        let len = string.chars().filter(|&c| c != '\0').count();

        Ok((string, len))
    }

    pub async fn send_command_for_response(
        &mut self,
        command: &[u8],
        timeout_ms: u64,
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        self.send_command(command).await?;
        self.read_response(timeout_ms).await
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn at_test(&mut self) -> Result<(String<BUF_SIZE>, usize), Error> {
        self.send_command_for_response(AT, 1000).await
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiMode {
    Station,
    SoftAP,
    StationSoftAP,
}

impl WifiMode {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            WifiMode::Station => "1",
            WifiMode::SoftAP => "2",
            WifiMode::StationSoftAP => "3",
        }
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn set_wifi_mode(
        &mut self,
        mode: WifiMode,
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        let mut command = String::<BUF_SIZE>::new();
        command
            .push_str(core::str::from_utf8(AT_CWMODE).map_err(|_| {
                Error::StringConversion(error::StringConversionError::Utf8Conversion)
            })?)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str(mode.as_str())
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        let len = command.chars().filter(|&c| c != '\0').count();
        self.send_command_for_response(command[..len].as_bytes(), 1000)
            .await
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn connect_to_wifi(
        &mut self,
        ssid: &[u8],
        password: &[u8],
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        let ssid = core::str::from_utf8(ssid)
            .map_err(|_| Error::StringConversion(error::StringConversionError::Utf8Conversion))?;
        let password = core::str::from_utf8(password)
            .map_err(|_| Error::StringConversion(error::StringConversionError::Utf8Conversion))?;

        let mut command = String::<BUF_SIZE>::new();
        command
            .push_str(core::str::from_utf8(AT_CWJAP).map_err(|_| {
                Error::StringConversion(error::StringConversionError::Utf8Conversion)
            })?)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str("\"")
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str(ssid)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str("\",\"")
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str(password)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str("\"")
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;

        let len = command.chars().filter(|&c| c != '\0').count();
        self.send_command_for_response(command[..len].as_bytes(), 10000)
            .await
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn check_wifi_connection(
        &mut self,
        timeout_ms: u64,
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        self.send_command_for_response(AT_CIFSR, timeout_ms).await
    }

    pub async fn loop_until_wifi_connected(&mut self) -> Result<(String<BUF_SIZE>, usize), Error> {
        let (mut response, mut len);
        loop {
            (response, len) = self.check_wifi_connection(1000).await?;
            let response = response[..len].as_str();
            if response.contains("OK") {
                break;
            }
        }
        Ok((response, len))
    }

    pub async fn wait_for_wifi_connection(
        &mut self,
        timeout_ms: u64,
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        let timeout = Timer::after_millis(timeout_ms);
        let loop_future = self.loop_until_wifi_connected();

        match select(timeout, loop_future).await {
            Either::First(_) => Err(Error::Rx(error::RxError::Timeout)),
            Either::Second(res) => res,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Ssl,
}

impl Protocol {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Protocol::Tcp => "TCP",
            Protocol::Udp => "UDP",
            Protocol::Ssl => "SSL",
        }
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn connect_to_server(
        &mut self,
        protocol: Protocol,
        ip: &[u8],
        port: u16,
        timeout_ms: u64,
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        let ip = core::str::from_utf8(ip)
            .map_err(|_| Error::StringConversion(error::StringConversionError::Utf8Conversion))?;
        let port = crate::util::parse_to_str::<5, _>(port)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        let port = port.0[..port.1].as_str();

        let mut command = String::<BUF_SIZE>::new();
        command
            .push_str(core::str::from_utf8(AT_CIPSTART).map_err(|_| {
                Error::StringConversion(error::StringConversionError::Utf8Conversion)
            })?)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str("\"")
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str(protocol.as_str())
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str("\",\"")
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str(ip)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str("\",")
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str(port)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;

        let len = command.chars().filter(|&c| c != '\0').count();
        self.send_command_for_response(command[..len].as_bytes(), timeout_ms)
            .await
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataTransferMode {
    Normal,
    Transparent,
}

impl DataTransferMode {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            DataTransferMode::Normal => "0",
            DataTransferMode::Transparent => "1",
        }
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn set_data_transfer_mode(
        &mut self,
        mode: DataTransferMode,
        timeout_ms: u64,
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        let mut command = String::<BUF_SIZE>::new();
        command
            .push_str(core::str::from_utf8(AT_CIPMODE).map_err(|_| {
                Error::StringConversion(error::StringConversionError::Utf8Conversion)
            })?)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str(mode.as_str())
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;

        let len = command.chars().filter(|&c| c != '\0').count();
        self.send_command_for_response(command[..len].as_bytes(), timeout_ms)
            .await
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn send_network_request(
        &mut self,
        request: &[u8],
        timeout_ms: u64,
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        let request_len = request.len() + 2; // +2 for auto \r\n
        let request_len = crate::util::parse_to_str::<BUF_SIZE, _>(request_len)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        let request_len = request_len.0[..request_len.1].as_str();

        // Prepare the AT+CIPSEND command with the request length
        let mut command = String::<BUF_SIZE>::new();
        command
            .push_str(core::str::from_utf8(AT_CIPSEND).map_err(|_| {
                Error::StringConversion(error::StringConversionError::Utf8Conversion)
            })?)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;
        command
            .push_str(request_len)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;

        self.send_command(command.as_bytes()).await?;

        // Prepare the actual request command
        command.clear();
        command
            .push_str(core::str::from_utf8(request).map_err(|_| {
                Error::StringConversion(error::StringConversionError::Utf8Conversion)
            })?)
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;

        let mut response = String::<BUF_SIZE>::new();

        let len = command.chars().filter(|&c| c != '\0').count();
        let (res, l) = self
            .send_command_for_response(command[..len].as_bytes(), timeout_ms)
            .await?;
        response
            .push_str(&res[..l])
            .map_err(|_| Error::StringConversion(error::StringConversionError::BufferConversion))?;

        loop {
            match self.read_response(timeout_ms).await {
                Ok((res, l)) => {
                    response.push_str(&res[..l]).map_err(|_| {
                        Error::StringConversion(error::StringConversionError::BufferConversion)
                    })?;
                }
                Err(e) => {
                    if e == Error::Rx(error::RxError::Timeout) {
                        break; // Exit the loop on timeout
                    } else {
                        return Err(e); // Propagate other errors
                    }
                }
            }
        }

        let len = response.chars().filter(|&c| c != '\0').count();
        Ok((response, len))
    }
}

type Error = error::Esp8266Error;
