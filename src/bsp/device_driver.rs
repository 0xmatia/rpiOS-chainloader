/*
 * File: device_driver.rs
 * Project: RpiOS
 * File Created: Thursday, 30th December 2021 2:44:06 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */


#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod bcm;
mod common;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use bcm::*;