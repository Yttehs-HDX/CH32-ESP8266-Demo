# CH32-ESP8266-Demo

[![Build test](https://github.com/Yttehs-HDX/CH32-ESP8266-Demo/actions/workflows/build-test.yml/badge.svg)](https://github.com/Yttehs-HDX/CH32-ESP8266-Demo/actions/workflows/build-test.yml)

Demo for ESP8266 under CH32V307VCT6 using ch32-hal

## Features

- UART: communicate with ESP8266, see struct [Esp8266Driver](src/esp8266_driver/mod.rs).

```rust
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
```

## Description

This project is build via [ch32-rs/ch32-hal-template](https://github.com/ch32-rs/ch32-hal-template).

This project is for CH32V307VCT6 only:

| Board        | Network Chip        |
|:------------:|:-------------------:|
| CH32V307VCT6 | ESP8266-01 (8 pins) |

Wrap AT-commands for a better experience.

### Output Example

```txt
2025-06-01 01:59:57.093: Test AT: "AT\n\nOK"
2025-06-01 01:59:57.116: Set Station Mode: "AT+CWMODE=1\n\nOK"
2025-06-01 01:59:57.125: Set Data Transfer Mode: "AT+CIPMODE=0\n\nOK"
2025-06-01 01:59:57.136: Connect to Wi-Fi: "AT+CWJAP=\"SSID\",\"PASSWORD\"
2025-06-01 01:59:57.157: Wi-Fi Status: "AT+CIFSR\n+CIFSR:STAIP,\"192.168.1.101\"\n+CIFSR:STAMAC,\"2c:3a:e8:40:bb:c8\"\n\nOK"
2025-06-01 01:59:57.175: Connect to Server: "AT+CIPSTART=\"TCP\",\"192.168.1.111\",5000"
2025-06-01 02:00:00.169: Sending command: "AT+CIPSEND=44"
2025-06-01 02:00:01.390: Send Network Request: "Recv 44 bytesSEND OK+IPD,164:HTTP/1.1 200 OK\nContent-Type: text/plain; charset=utf-8\nDate: Sat, 31 May 2025 17:59:59 GMT\nServer: Kestrel\nTransfer-Encoding: chunked\n\nc\nHello World!\n0"
2025-06-01 02:00:02.402: tick
2025-06-01 02:00:03.398: tick
2025-06-01 02:00:04.394: tick
```

Get the pure response from server:

```txt
c
Hello, world!
0
```

## Usage

1. Install [wlink](https://github.com/ch32-rs/wlink) tool.

2. Connect DuPont line.

From ESP8266 to board:

| ESP8266 | Board |
|:-------:|:-----:|
| 3V3     | 3V3   |
| EN      | 3V3   |
| GND     | GND   |
| RX      | PA9   |
| TX      | PA10  |

3. Customize config:

- `src/constant.rs`: for Wi-Fi info.
- `src/main.rs`: at main function, for server info.

4. Run command:

```bash
cargo run --release
# or
make run
```

## Discussion

[Discussions](https://github.com/ch32-rs/ch32-hal/discussions/100) at ch32-hal.

## Special Thanks

- [`ch32-rs/ch32-hal`](https://github.com/ch32-rs/ch32-hal): HAL implementation for CH32V307.

## License

MIT
