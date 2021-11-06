/*
 * File: console.rs
 * Project: RpiOS
 * File Created: Saturday, 6th November 2021 5:21:01 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

//! Write interface to UART data register
 
use crate::console::interface;
use core::fmt;

const UART_DR_ADDRESS: u64 = 0x3F20_1000;

// Dummy object
struct QEMUOutput;

impl fmt::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for character in s.chars() {
            unsafe {
                core::ptr::write_volatile(UART_DR_ADDRESS as *mut u8, character as u8);
            }
        }
        Ok(())
    }
}

// public
pub fn console() -> impl interface::Write {
    QEMUOutput {}
}