#![doc(html_logo_url = "https://git.io/JeGIp")]

//! Entery point of, well, everything
//! Well, not really, more general metadata, module definitions etc...
#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]

mod cpu;
mod bsp;
/// Dummy panic handler
mod panic_handler;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    panic!()
}