// src/spi/mod.rs
pub trait SPI {
    /// Initialize the SPI with clock divisor and mode
    /// `clock_div`: Clock divider for SCK frequency
    /// `mode`: SPI mode (0-3)
    fn init(clock_div: u8, mode: u8);

    /// Transfer a single byte of data
    /// Returns the byte received during the transfer
    fn transfer(data: u8) -> u8;

    /// Select a specific peripheral by pulling the CS pin low
    fn select_peripheral(cs_pin: u8);

    /// Deselect a specific peripheral by pulling the CS pin high
    fn deselect_peripheral(cs_pin: u8);
}
