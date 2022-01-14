/*
 * File: raspberrypi.rs
 * Project: RpiOS
 * File Created: Tuesday, 26th October 2021 5:45:10 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

//! board specific code for Raspberry Pi (currently 3) 

pub mod cpu;
pub mod console;
pub mod driver;
pub mod memory;

use super::device_driver;

//-----------------------------------------------
//      Global instances
//-----------------------------------------------

static GPIO: device_driver::GPIO =
            unsafe {device_driver::GPIO::new(memory::map::mmio::GPIO_START) };

static PL011_UART: device_driver::PL011Uart =
            unsafe {device_driver::PL011Uart::new(memory::map::mmio::PL011_UART_START) };

/// Returns the board's name (rpi3, rpi4)
pub fn board_name() -> &'static str {
    #[cfg(feature="bsp_rpi3")]
    {
        "Raspberry pi 3"
    }
    
    #[cfg(feature="bsp_rpi4")]
    {

        "Raspberry pi 4"
    }
}