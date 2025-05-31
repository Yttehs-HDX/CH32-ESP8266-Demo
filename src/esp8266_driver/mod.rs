use ch32_hal::{
    mode::Async,
    usart::{Instance, Uart, UartRx, UartTx},
};
use embassy_futures::select::{select, Either};
use embassy_time::Timer;
use heapless::{String, Vec};

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
    pub async fn send_raw_command(&mut self, command: &[u8]) -> Result<(), &'static str> {
        self.tx
            .write(command)
            .await
            .map_err(|_| "Failed to send byte")
    }

    pub async fn read_raw_response(&mut self, timeout_ms: u64) -> Result<(String<BUF_SIZE>, usize), &'static str> {
        let timeout = Timer::after_millis(timeout_ms);

        let mut buf = [0u8; BUF_SIZE];
        let read_future = self.rx.read_until_idle(&mut buf);

        let len = match select(timeout, read_future).await {
            Either::First(_) => return Err("Timeout while waiting for response"),
            Either::Second(res) => res.map_err(|_| "Failed to read response")?,
        };

        let vec = Vec::from_slice(&buf).map_err(|_| "Failed to convert buffer to Vec")?;
        let string = String::from_utf8(vec).map_err(|_| "Failed to convert response to string")?;

        Ok((string, len))
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn send_command(&mut self, command: &[u8]) -> Result<(), &'static str> {
        let mut cmd = String::<BUF_SIZE>::new();
        let cmd_str = core::str::from_utf8(command).map_err(|_| "Command is not valid UTF-8")?;
        cmd.push_str(cmd_str)
            .map_err(|_| "Failed to create command string")?;
        cmd.push_str("\r\n")
            .map_err(|_| "Failed to append CRLF to command")?;

        self.tx
            .write(cmd.as_bytes())
            .await
            .map_err(|_| "Failed to send command")
    }

    pub async fn read_response(&mut self, timeout_ms: u64) -> Result<(String<BUF_SIZE>, usize), &'static str> {
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
    ) -> Result<(String<BUF_SIZE>, usize), &'static str> {
        self.send_command(command).await?;
        self.read_response(timeout_ms).await
    }
}
