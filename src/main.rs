#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use ch32_hal::{bind_interrupts, println, usart::Uart};
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_halt as _;

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

    let (mut tx, mut rx) = uart.split();

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}
