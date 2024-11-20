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
