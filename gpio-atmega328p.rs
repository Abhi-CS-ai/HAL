use core::ptr;
use crate::gpio::{GPIO, PinMode};

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *const u8 = 0x23 as *const u8;

pub struct Atmega328pGPIO;

impl GPIO for Atmega328pGPIO {
    fn set_pin_mode(pin: u8, mode: PinMode) {
        unsafe {
            match mode {
                PinMode::Output => {
                    ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) | (1 << pin));
                }
                PinMode::Input => {
                    ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) & !(1 << pin));
                }
                PinMode::InputPullUp => {
                    ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) & !(1 << pin));
                    ptr::write_volatile(PORTB, ptr::read_volatile(PORTB) | (1 << pin));
                }
            }
        }
    }

    fn write_pin(pin: u8, value: bool) {
        unsafe {
            if value {
                ptr::write_volatile(PORTB, ptr::read_volatile(PORTB) | (1 << pin));
            } else {
                ptr::write_volatile(PORTB, ptr::read_volatile(PORTB) & !(1 << pin));
            }
        }
    }

    fn read_pin(pin: u8) -> bool {
        unsafe { ptr::read_volatile(PINB) & (1 << pin) != 0 }
    }
}
