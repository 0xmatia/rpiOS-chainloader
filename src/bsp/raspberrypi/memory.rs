/*
 * File: memory.rs
 * Project: RpiOS
 * File Created: Thursday, 30th December 2021 3:42:09 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

// Taken from: https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/05_drivers_gpio_uart/src/bsp/raspberrypi/memory.rs
// Explainations can be found in the BCM2837 and my personal onenote.
// This is just a way to define the start address of UART and the GPIO. The trick is to figure out that the specified addresses are bus addresses
// that need to be mapped physically.

pub mod map {
    pub const BOARD_DEFAULT_LOAD_ADDRESS: usize =        0x8_0000;

    pub const GPIO_OFFSET:         usize = 0x0020_0000;
    pub const UART_OFFSET:         usize = 0x0020_1000;

    /// Physical devices.
    #[cfg(feature = "bsp_rpi3")]
    pub mod mmio {
        use super::*;

        pub const START:            usize =         0x3F00_0000;
        pub const GPIO_START:       usize = START + GPIO_OFFSET;
        pub const PL011_UART_START: usize = START + UART_OFFSET;
    }

    /// Physical devices.
    #[cfg(feature = "bsp_rpi4")]
    pub mod mmio {
        use super::*;

        pub const START:            usize =         0xFE00_0000;
        pub const GPIO_START:       usize = START + GPIO_OFFSET;
        pub const PL011_UART_START: usize = START + UART_OFFSET;
    }
}

#[inline(always)]
pub fn board_default_load_address() -> *const u64 {
    map::BOARD_DEFAULT_LOAD_ADDRESS as _
}
