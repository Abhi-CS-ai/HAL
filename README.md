# Hardware Abstraction layer FOR ATMEGA328p

- We established distinct branches dedicated to the specific features of GPIO, USART, and SPI. Each feature has been allocated its own separate branch for better organization and clarity. For anyone interested in exploring these features more thoroughly, you can find detailed information regarding each one within their corresponding branches. These separate branches serve to enhance the clarity and manageability of our project.
This Rust code implements a basic Hardware Abstraction Layer (HAL) for controlling GPIO pins on the Atmega328p (Arduino Uno). It provides functions to configure a pin as input/output, write HIGH/LOW values, and read pin states using memory-mapped I/O.
# Access to branches
- [`GPIO`](https://github.com/Abhi-CS-ai/HAL-Atmega328p-/tree/GPIO-features)
- [`USART`](https://github.com/Abhi-CS-ai/HAL-Atmega328p-/tree/USART-feature)
- [`SPI`](https://github.com/Abhi-CS-ai/HAL-Atmega328p-/tree/SPI)

##  GPIO Feature - Hardware Abstraction Layer (HAL)

This branch focuses on the **GPIO Abstraction** feature. The GPIO feature allows you to configure and control the digital pins on the target microcontroller as inputs or outputs.

## Current Status
- **GPIO Pins**: Configure any digital pin as input or output.
- **Pin Read/Write**: Read and write values to digital pins.

## Usage Example


Key Components:

    Registers:
        DDRB: Configures pins as input or output.
        PORTB: Sets pins HIGH or LOW.
        PINB: Reads pin state (HIGH or LOW).
    Enums:
        PinMode: Defines Input or Output.
        PinState: Defines High or Low.

Functions:

    set_pin_mode(pin, mode): Configures pin as input or output.
    write_pin(pin, state): Writes HIGH or LOW to a pin.
    read_pin(pin): Reads the current pin state.

Memory Access:

    Uses unsafe blocks for direct register access via memory-mapped I/O.

This HAL simplifies GPIO control on the Atmega328p by abstracting hardware interactions into high-level Rust functions.

# ATmega328P USART Driver

This Rust crate provides a USART (Universal Synchronous/Asynchronous Receiver/Transmitter) driver for the ATmega328P microcontroller. The driver allows you to initialize the USART peripheral, transmit data, and receive data.

## Features

- Initialize USART with a specified baud rate.
- Transmit data via USART.
- Receive data via USART.

## Constants

The following constants represent memory-mapped I/O registers for the USART peripheral:

- `UBRR0H`: High byte of the USART Baud Rate Register.
- `UBRR0L`: Low byte of the USART Baud Rate Register.
- `UCSR0B`: USART Control and Status Register B.
- `UCSR0C`: USART Control and Status Register C.
- `UDR0`: USART I/O Data Register.
- `UCSRA`: USART Control and Status Register A.

## Struct

The `Atmega328pUSART` struct represents the USART peripheral for the ATmega328P microcontroller.

## USART Trait Implementation

The `USART` trait is implemented for the `Atmega328pUSART` struct, providing methods to initialize the USART, transmit data, and receive data.

### Code

```rust
use core::ptr;
use crate::usart::USART;

const UBRR0H: *mut u8 = 0xC5 as *mut u8;
const UBRR0L: *mut u8 = 0xC4 as *mut u8;
const UCSR0B: *mut u8 = 0xC1 as *mut u8;
const UCSR0C: *mut u8 = 0xC2 as *mut u8;
const UDR0: *mut u8 = 0xC6 as *mut u8;
const UCSRA: *const u8 = 0xC0 as *const u8;

pub struct Atmega328pUSART;

impl USART for Atmega328pUSART {
    fn init(baud_rate: u32) {
        let ubrr = 16_000_000 / (16 * baud_rate) - 1;

        unsafe {
            ptr::write_volatile(UBRR0H, (ubrr >> 8) as u8);
            ptr::write_volatile(UBRR0L, ubrr as u8);
            ptr::write_volatile(UCSR0C, (1 << 1) | (1 << 2));
            ptr::write_volatile(UCSR0B, (1 << 3) | (1 << 4));
        }
    }

    fn transmit(data: u8) {
        unsafe {
            while ptr::read_volatile(UCSRA) & (1 << 5) == 0 {}
            ptr::write_volatile(UDR0, data);
        }
    }

    fn receive() -> u8 {
        unsafe {
            while ptr::read_volatile(UCSRA) & (1 << 7) == 0 {}
            ptr::read_volatile(UDR0)
        }
    }
}
```
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
```


[CORRECTION GPIO] (Don't hesitate to remove this part)
I couldn't compile ! When you build your project for the first time, I recommand you to use the ```cargo new your_project``` command.
It is good that you use a generic function like "modify_reg()", but you could have try to subdivise your project into different module.

[CORRECTION USART] (Don't hesitate to remove this part)
Your code cannot work with an empty Cargo.toml (or an empty code). I invite you to follow the advice I gave you in the GPIO correction (about cargo new).
Don't hesitate to send me an email.

[CORRECTION SPI] (Don't hesitate to remove this part)
You didn't implement the SPI feature.
