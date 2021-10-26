#![doc(html_logo_url = "https://git.io/JeGIp")]

//! Entery point of, well, everything
//! Well, not really, more general metadata, module definitions etc...
#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]

/// Dummy panic handler
mod cpu;
mod bsp;
mod panic_handler;