#![doc(html_logo_url = "https://git.io/JeGIp")]

//! Enter point of, well, everything
//! Well, not really, more general metadata, module definitions etc...
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]


mod cpu;
mod bsp;
mod console;
mod print;
mod synchronization;
mod driver;
mod panic_handler;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    use crate::driver::interface::DeviceManager;

    for i in bsp::driver::driver_manager().all_device_drivers().iter() {
        if let Err(e) = i.init() {
            panic!("Error initializing {} driver: {}", i.compatible(), e);
        }
    }
    bsp::driver::driver_manager().post_device_driver_init();

    kernel_main();
}

const LOADER_LOGO: &str = r#"
.____                     .___            
|    |    _________     __| _/___________ 
|    |   /  _ \__  \   / __ |/ __ \_  __ \
|    |__(  <_> ) __ \_/ /_/ \  ___/|  | \/
|_______ \____(____  /\____ |\___  >__|   
        \/         \/      \/    \/
"#;

fn kernel_main() -> ! {
    use bsp::console::console;
    use console::interface::All;

    println!("{}", LOADER_LOGO);
    println!("Running on: {}", bsp::board_name());
    println!();
    println!("Requesting binary!");
    console().flush();

    console().clear_rx();

    // send three times '3' through UART to notify the pusher to send the kernel / binary
    for _ in 0..3 {
        console().write_char(3 as char);
    }

    // Read the binary's size.
    let mut size: u32 = u32::from(console().read_char() as u8);
    size |= u32::from(console().read_char() as u8) << 8;
    size |= u32::from(console().read_char() as u8) << 16;
    size |= u32::from(console().read_char() as u8) << 24;

    console().write_char('O');
    console().write_char('K');

    let kernel_addr = bsp::memory::board_default_load_address() as *mut u8;

    unsafe {
        for i in 0..size {
            core::ptr::write_volatile(kernel_addr.offset(i as isize), console().read_char() as u8);
        }
    }

    println!("Received kernel, executing now!");
    console().flush();

    let kernel: fn() -> ! = unsafe { core::mem::transmute(kernel_addr) };
    kernel();
}
