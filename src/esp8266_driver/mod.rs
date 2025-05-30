use ch32_hal::{mode::Async, println, usart::{Instance, Uart, UartRx, UartTx}};
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
    pub async fn send_raw_command(&mut self, command: &str) -> Result<(), &'static str> {
        let bytes = command.as_bytes();
        self.tx.write(bytes).await.map_err(|_| "Failed to send byte")?;
        Ok(())
    }

    pub async fn read_raw_response(&mut self) -> Result<(String<BUF_SIZE>, usize), &'static str> {
        let mut buf = [0u8; BUF_SIZE];

        let len = self.rx
            .read_until_idle(&mut buf)
            .await
            .map_err(|_| "Failed to read response")?;

        let vec = Vec::from_slice(&buf)
            .map_err(|_| "Failed to convert buffer to Vec")?;

        let string = String::from_utf8(vec)
            .map_err(|_| "Failed to convert response to string")?;

        Ok((string, len))
    }
}

impl<'d, T: Instance> Esp8266Driver<'d, T> {
    pub async fn send_command(&mut self, command: &str) -> Result<(), &'static str> {
        let mut cmd = String::<BUF_SIZE>::new();
        cmd.push_str(command).map_err(|_| "Failed to create command string")?;
        cmd.push_str("\r\n").map_err(|_| "Failed to append CRLF to command")?;

        self.tx
            .write(cmd.as_bytes())
            .await
            .map_err(|_| "Failed to send command")
    }

    pub async fn send_command_for_response(&mut self, command: &str) -> Result<(String<BUF_SIZE>, usize), &'static str> {
        self.send_command(command).await?;
        let (raw_response, raw_len) = self.read_raw_response().await?;

        // clip command loop rx message
        let mut response_cut = raw_response[..raw_len].split(command);
        let _ = response_cut.next();
        let response = response_cut.next().unwrap();

        // trim '\r' and '\n'
        let response = response.trim_matches(|c| c == '\r' || c == '\n');

        let mut string = String::<BUF_SIZE>::new();
        string.push_str(response).unwrap();

        // count length
        let len = string
            .chars()
            .filter(|&c| c != '\0')
            .count();

        Ok((string, len))
    }
}
