/*
 * File: console.rs
 * Project: RpiOS
 * File Created: Saturday, 6th November 2021 5:17:59 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

pub mod interface {
    pub use core::fmt;

    /// Console write functions
    pub trait Write {
        /// Write a single character
        fn write_char(&self, c: char);
        /// write format string trait
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
        /// block until TX FIFO is not busy anymore
        fn flush(&self);
    }

    /// Console read functions
    pub trait Read {
        /// Read one character
        fn read_char(&self) -> char {
            ' '
        }
        /// Clear RX buffers
        fn clear_rx(&self);
    }
    /// console statistics
    pub trait Statistics {
        /// returns the number of characters written
        fn chars_written(&self) -> usize {
            0
        }
        /// returns the number of characters read
        fn chars_read(&self) -> usize {
            0
        }
    }

    /// trait alias: All for output interface that needs to implement
    pub trait All = Read + Write + Statistics;
}