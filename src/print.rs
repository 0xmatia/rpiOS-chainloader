/*
 * File: print.rs
 * Project: RpiOS
 * File Created: Saturday, 6th November 2021 5:42:51 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

//! Print functions

use crate::{bsp, console};
use core::fmt;

// private, helper function
pub fn __print(args: fmt::Arguments) {
    // This is just fmt::Write, but more readable (or is it?)
    use console::interface::Write;
    bsp::console::console().write_fmt(args).unwrap();
}

// public usable macros: print, println

/// Regular print, no endline
#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        ($crate::print::__print(format_args!($($args)*)));
    };
}

/// Print with newline at the end
#[macro_export]
macro_rules! println {
    () => {
        print!("\n");
    };
    ($($args:tt)*) => {
        ($crate::print::__print(format_args_nl!($($args)*)))
    };
}