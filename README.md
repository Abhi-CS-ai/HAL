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


[CORRECTION GPIO] (Don't hesitate to remove this part)
I couldn't compile ! When you build your project for the first time, I recommand you to use the ```cargo new your_project``` command.
It is good that you use a generic function like "modify_reg()", but you could have try to subdivise your project into different module.
