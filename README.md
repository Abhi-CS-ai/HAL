# SPI feature




[CORRECTION GPIO] (Don't hesitate to remove this part)
I couldn't compile ! When you build your project for the first time, I recommand you to use the ```cargo new your_project``` command.
It is good that you use a generic function like "modify_reg()", but you could have try to subdivise your project into different module.

[CORRECTION USART] (Don't hesitate to remove this part)
Your code cannot work with an empty Cargo.toml (or an empty code). I invite you to follow the advice I gave you in the GPIO correction (about cargo new).
Don't hesitate to send me an email.

[CORRECTION SPI] (Don't hesitate to remove this part)
You didn't implement the SPI feature.


# SPI Feature for Atmega328p

## Overview

The SPI (Serial Peripheral Interface) feature is a full-duplex protocol that allows simultaneous data transfer between a controller and peripherals. This implementation abstracts the Atmega328p's SPI module, providing high-level methods to initialize, transmit, and receive data, as well as control chip select (CS) lines for multiple peripherals.

## Features

- Full-duplex communication.
- Support for SPI modes 0, 1, 2, and 3.
- Configurable clock speed using clock dividers.
- Control of chip select (CS) pins for peripheral communication.

## File Structure


## How to Use

### 1. Initialization
Configure the SPI with a specific clock divisor and mode (0-3). For example, initialize with a clock divider of `0b01` and mode `0`:

```rust
use spi::SPI;
use spi::atmega328p::Atmega328pSPI;

Atmega328pSPI::init(0b01, 0);
