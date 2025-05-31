#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(str_as_str)]

use ch32_hal::{bind_interrupts, println, usart::Uart};
use constant::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use esp8266_driver::Esp8266Driver;

mod constant;
mod esp8266_driver;
mod lang_items;
mod util;

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
    )
    .unwrap();

    let mut esp_driver = Esp8266Driver::new(uart);

    init_esp8266(&mut esp_driver).await;

    // Set the ESP8266 to TCP client mode and connect to a server
    let (response, len) = esp_driver
        .connect_to_server(esp8266_driver::Protocol::Tcp, b"192.168.1.111", 5000, 1000)
        .await
        .unwrap();
    println!("Connect to Server: {:?}", response[..len].as_str());
    Timer::after_millis(3000).await; // Wait for connection to establish

    // Send a GET request to the server
    let (response, len) = esp_driver
        .send_network_request(b"GET / HTTP/1.1\r\nHost: 192.168.1.111:5000\r\n", 1000)
        .await
        .unwrap();
    println!("Send Network Request: {:?}", response[..len].as_str());

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}

async fn init_esp8266(driver: &mut Esp8266Driver<'_, ch32_hal::peripherals::USART1>) {
    // Test ESP8266 AT commands
    let (response, len) = driver.at_test().await.unwrap();
    println!("Test AT: {:?}", response[..len].as_str());

    // Set the ESP8266 to station mode
    let (response, len) = driver
        .set_wifi_mode(esp8266_driver::WifiMode::Station)
        .await
        .unwrap();
    println!("Set Station Mode: {:?}", response[..len].as_str());

    // Set the ESP8266 to normal mode
    let (response, len) = driver
        .set_data_transfer_mode(esp8266_driver::DataTransferMode::Normal, 1000)
        .await
        .unwrap();
    println!("Set Data Transfer Mode: {:?}", response[..len].as_str());

    // Connect to a Wi-Fi network
    let (response, len) = driver
        .connect_to_wifi(WIFI_SSID, WIFI_PASSWORD)
        .await
        .unwrap();
    println!("Connect to Wi-Fi: {:?}", response[..len].as_str());

    // Wait for the ESP8266 to connect to the Wi-Fi network
    let (response, len) = driver.wait_for_wifi_connection(5000).await.unwrap();
    println!("Wi-Fi Status: {:?}", response[..len].as_str());
}
