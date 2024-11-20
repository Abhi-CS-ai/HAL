pub trait USART {
    fn init(baud_rate: u32);
    fn transmit(data: u8);
    fn receive() -> u8;
}

#[cfg(feature = "atmega328p")]
pub mod atmega328p;
