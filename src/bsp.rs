/*
 * File: bsp.rs
 * Project: RpiOS
 * File Created: Tuesday, 26th October 2021 5:26:03 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */
//! board specific code
//! reexport board specific code (only RPi 3 for now)

#[cfg(feature="bsp_rpi3")]
mod raspberrypi;

#[cfg(feature="bsp_rpi3")]
#[allow(warnings)]
use raspberrypi::*;