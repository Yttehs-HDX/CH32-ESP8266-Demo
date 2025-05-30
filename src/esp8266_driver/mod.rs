use ch32_hal::{mode::Async, usart::{Instance, Uart, UartRx, UartTx}};
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
    pub async fn send_command(&mut self, command: &str) -> Result<(), &'static str> {
        let bytes = command.as_bytes();
        self.tx.write(bytes).await.map_err(|_| "Failed to send byte")?;
        Ok(())
    }

    pub async fn read_response(&mut self) -> Result<(String<BUF_SIZE>, usize), &'static str> {
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
