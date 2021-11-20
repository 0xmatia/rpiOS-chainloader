/*
 * File: console.rs
 * Project: RpiOS
 * File Created: Saturday, 6th November 2021 5:21:01 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

//! Write interface to UART data register
 
use crate::{console, synchronization::{NullLock, interface::Mutex}};
use core::fmt;

const UART_DR_ADDRESS: u64 = 0x3F20_1000;

/// This is the inner struct of the qemu output device.
/// it is lock-protected and thus can hold / save a state safely
struct QEMUOutputInner {
    chars_written: usize
}

/// The QEMU output device, containing an inner, mutex protected device,
/// that allows it to share it's state and even modify it globally.
struct QEMUOutput {
    inner: NullLock<QEMUOutputInner>
}

/// Global, mutable and shareable instance of the QEMU output device
static QEMU_OUTPUT_DEVICE: QEMUOutput = QEMUOutput::new();


/// Inner qemu device implementation
impl QEMUOutputInner {
    const fn new() -> Self {
        Self {
            chars_written: 0
        }
    }
    fn write_char(&mut self, c: char) {
        unsafe {
            core::ptr::write_volatile(UART_DR_ADDRESS as *mut u8, c as u8);
        }
        self.chars_written += 1;
    }
    
}

/// inner QEMU device write format trait implementation
impl fmt::Write for QEMUOutputInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for character in s.chars() {
            if character == '\n' {
                self.write_char('\r');
            }
            self.write_char(character);
        } 
        Ok(())    
    }
}

impl QEMUOutput {
    pub const fn new() -> Self {
        Self {
            inner: NullLock::new(QEMUOutputInner::new())
        }
    }
}

/// Now instead of creating a console instance every time we want to print,
/// return a reference to the console device (Anything that implements console::interface::All).
/// Because the console device is static, we can say the lifetime of the reference is also static because 
/// we know the device will live as long as the os is running :).
pub fn console() -> &'static impl console::interface::All {
    &QEMU_OUTPUT_DEVICE
}

// OS interface implementation

impl console::interface::Write for QEMUOutput {
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::write(inner, args))
    }
}

impl console::interface::Statisitcs for QEMUOutput {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner|inner.chars_written)
    }
}