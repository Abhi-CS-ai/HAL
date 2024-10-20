This Rust code implements a basic Hardware Abstraction Layer (HAL) for controlling GPIO pins on the Atmega328p (Arduino Uno). It provides functions to configure a pin as input/output, write HIGH/LOW values, and read pin states using memory-mapped I/O.
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
