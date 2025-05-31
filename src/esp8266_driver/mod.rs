use ch32_hal::{
    mode::Async,
    usart::{Instance, Uart, UartRx, UartTx},
};
use command::*;
use embassy_futures::select::{select, Either};
use embassy_time::Timer;
use error::Esp8266Error;
use error::*;
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
            .map_err(|e| Error::TxError(TxError::WriteError(e)))
    }

    pub async fn read_raw_response(
        &mut self,
        timeout_ms: u64,
    ) -> Result<(String<BUF_SIZE>, usize), Error> {
        let timeout = Timer::after_millis(timeout_ms);

        let mut buf = [0u8; BUF_SIZE];
        let read_future = self.rx.read_until_idle(&mut buf);

        let len = match select(timeout, read_future).await {
            Either::First(_) => return Err(Error::RxError(RxError::Timeout)),
            Either::Second(res) => res.map_err(|e| Error::RxError(RxError::ReadError(e)))?,
        };

        let vec = Vec::from_slice(&buf).map_err(|_| {
            Error::StringConversionError(StringConversionError::BufferConversionError)
        })?;
        let string = String::from_utf8(vec)
            .map_err(|_| Error::StringConversionError(StringConversionError::Utf8Error))?;

        Ok((string, len))
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn send_command(&mut self, command: &[u8]) -> Result<(), Error> {
        let mut cmd = String::<BUF_SIZE>::new();
        let cmd_str = core::str::from_utf8(command)
            .map_err(|_| Error::StringConversionError(StringConversionError::Utf8Error))?;
        cmd.push_str(cmd_str).map_err(|_| {
            Error::StringConversionError(StringConversionError::BufferConversionError)
        })?;
        cmd.push_str("\r\n").map_err(|_| {
            Error::StringConversionError(StringConversionError::BufferConversionError)
        })?;

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
        string.push_str(response).unwrap();
        string = string.chars().filter(|&c| c != '\r').collect();

        // count length
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
    pub fn as_str(&self) -> &'static str {
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
            .push_str(core::str::from_utf8(AT_CWMODE).unwrap())
            .map_err(|_| {
                Error::StringConversionError(StringConversionError::BufferConversionError)
            })?;
        command.push_str(mode.as_str()).map_err(|_| {
            Error::StringConversionError(StringConversionError::BufferConversionError)
        })?;
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
            .map_err(|_| Error::StringConversionError(StringConversionError::Utf8Error))?;
        let password = core::str::from_utf8(password)
            .map_err(|_| Error::StringConversionError(StringConversionError::Utf8Error))?;
        let mut command = String::<BUF_SIZE>::new();
        command
            .push_str(core::str::from_utf8(AT_CWJAP).unwrap())
            .map_err(|_| {
                Error::StringConversionError(StringConversionError::BufferConversionError)
            })?;
        command.push_str("\"").map_err(|_| {
            Error::StringConversionError(StringConversionError::BufferConversionError)
        })?;
        command.push_str(ssid).map_err(|_| {
            Error::StringConversionError(StringConversionError::BufferConversionError)
        })?;
        command.push_str("\",\"").map_err(|_| {
            Error::StringConversionError(StringConversionError::BufferConversionError)
        })?;
        command.push_str(password).map_err(|_| {
            Error::StringConversionError(StringConversionError::BufferConversionError)
        })?;
        command.push_str("\"").map_err(|_| {
            Error::StringConversionError(StringConversionError::BufferConversionError)
        })?;

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
            (response, len) = self.check_wifi_connection(1000).await.unwrap();
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
            Either::First(_) => Err(Error::RxError(RxError::Timeout)),
            Either::Second(res) => res,
        }
    }
}

type Error = Esp8266Error;
