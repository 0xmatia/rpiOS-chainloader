#![doc(html_logo_url = "https://git.io/JeGIp")]

//! Entery point of, well, everything
//! Well, not really, more general metadata, module definitions etc...
#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]


mod cpu;
mod bsp;
mod console;
mod print;
mod synchronization;
/// Dummy panic handler
mod panic_handler;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    use crate::console::interface::Statisitcs;
    println!("[0] Hello hello!");
    println!("[1] Chars written: {}", bsp::console::console().chars_written());
    println!("[2] NT 2.0");
    panic!()
}