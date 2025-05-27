#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_time::Timer;
use ch32_hal::println;
use panic_halt as _;

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    ch32_hal::debug::SDIPrint::enable();
    let p = ch32_hal::init(ch32_hal::Config::default());

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}
