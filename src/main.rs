#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(str_as_str)]

use ch32_hal::{bind_interrupts, println, usart::Uart};
use embassy_executor::Spawner;
use embassy_time::Timer;
use esp8266_driver::Esp8266Driver;

mod lang_items;
mod esp8266_driver;

bind_interrupts!(
    struct Irqs {
        USART1 => ch32_hal::usart::InterruptHandler<ch32_hal::peripherals::USART1>;
    }
);

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    ch32_hal::debug::SDIPrint::enable();
    let p = ch32_hal::init(ch32_hal::Config::default());

    let uart_config = ch32_hal::usart::Config::default();
    let uart = Uart::new(
        p.USART1,
        p.PA10, // RX
        p.PA9,  // TX
        Irqs,
        p.DMA1_CH4,
        p.DMA1_CH5,
        uart_config,
    ).unwrap();

    let mut esp_driver = Esp8266Driver::new(uart);

    esp_driver.send_command("AT").await.unwrap();
    let (response, len) = esp_driver.read_raw_response().await.unwrap();
    println!("Response len: {}", len);
    println!("Response: {:?}", response[..len].as_str());

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}
