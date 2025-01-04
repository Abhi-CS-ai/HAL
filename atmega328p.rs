// src/spi/atmega328p.rs
use core::ptr;
use crate::spi::SPI;

const SPCR: *mut u8 = 0x4C as *mut u8; // SPI Control Register
const SPSR: *mut u8 = 0x4D as *mut u8; // SPI Status Register
const SPDR: *mut u8 = 0x4E as *mut u8; // SPI Data Register

pub struct Atmega328pSPI;

impl SPI for Atmega328pSPI {
    fn init(clock_div: u8, mode: u8) {
        unsafe {
            // Configure SPI: Enable SPI, set mode, and clock divider
            ptr::write_volatile(SPCR, (1 << 6) | (1 << 4) | (mode << 2) | clock_div);
        }
    }

    fn transfer(data: u8) -> u8 {
        unsafe {
            // Write data to SPDR to start transmission
            ptr::write_volatile(SPDR, data);

            // Wait for transmission to complete (SPIF = 1 in SPSR)
            while ptr::read_volatile(SPSR) & (1 << 7) == 0 {}

            // Read and return the received data
            ptr::read_volatile(SPDR)
        }
    }

    fn select_peripheral(cs_pin: u8) {
        unsafe {
            // Set CS pin low (assumes GPIO is configured for the pin)
            let port = 0x25 as *mut u8; // Example PORTB register
            ptr::write_volatile(port, ptr::read_volatile(port) & !(1 << cs_pin));
        }
    }

    fn deselect_peripheral(cs_pin: u8) {
        unsafe {
            // Set CS pin high
            let port = 0x25 as *mut u8; // Example PORTB register
            ptr::write_volatile(port, ptr::read_volatile(port) | (1 << cs_pin));
        }
    }
}
