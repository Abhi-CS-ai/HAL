pub trait GPIO {
    fn set_pin_mode(pin: u8, mode: PinMode);
    fn write_pin(pin: u8, value: bool);
    fn read_pin(pin: u8) -> bool;
}

pub enum PinMode {
    Input,
    Output,
    InputPullUp,
}

#[cfg(feature = "atmega328p")]
pub mod atmega328p;
