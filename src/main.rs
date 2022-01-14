#![doc(html_logo_url = "https://git.io/JeGIp")]

//! Enter point of, well, everything
//! Well, not really, more general metadata, module definitions etc...
#![feature(const_fn_fn_ptr_basics)]
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

fn kernel_main() -> ! {
    use bsp::console::console;
    use console::interface::All;
    use driver::interface::DeviceManager;

    println!("RpiOS is booting...");
    println!("Communicating through PL011 UART");

    println!("[1] Booting on: {}", bsp::board_name());
    println!("[2] Drivers loaded:");
    for (i, driver) in bsp::driver::driver_manager().all_device_drivers().iter().enumerate(){
        println!("{}. {}", i+1, driver.compatible());
    }

    println!("[3] Chars written: {}", bsp::console::console().chars_written());

    println!("[4] Entering echo mode");
    console().clear_rx();
    loop {
        let c = bsp::console::console().read_char();
        bsp::console::console().write_char(c);
    } 
}