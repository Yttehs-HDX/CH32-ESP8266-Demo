# CH32-ESP8266-Demo

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

## Special Thanks

- [`ch32-rs/ch32-hal`](https://github.com/ch32-rs/ch32-hal): HAL implementation for CH32V307.

## License

MIT
