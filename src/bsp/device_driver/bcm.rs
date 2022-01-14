/*
 * File: bcm.rs
 * Project: RpiOS
 * File Created: Thursday, 30th December 2021 3:50:27 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

//! BCM2xxx drivers (RPI3 is BCM2837)

mod bcm2xxx_gpio;
mod bcm2xxx_pl011_uart;

pub use bcm2xxx_gpio::*;
pub use bcm2xxx_pl011_uart::*;