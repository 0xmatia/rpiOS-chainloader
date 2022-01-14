use core::{fmt,panic::PanicInfo};
use crate::{cpu, bsp};

fn _panic_print(args: fmt::Arguments) {
    use fmt::Write;
    unsafe {bsp::console::panic_console_out().write_fmt(args).unwrap() }
}

/// Prints with a newline - only use from the panic handler.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! panic_println {
    ($($args:tt)*) => ({
        _panic_print(format_args_nl!($($args)*));
    })
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(args) = _info.message() {
        panic_println!("\nKernel panic: {}", args);
    }
    else {
        panic_println!("\nKernel panic!")
    }
    cpu::wait_forever()
}