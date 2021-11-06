use core::panic::PanicInfo;
use crate::{cpu, println};

#[panic_handler]
/// Dummy panic handler
fn panic(_info: &PanicInfo) -> ! {
    if let Some(args) = _info.message() {
        println!("\nKernel panic: {}", args);
    }
    else {
        println!("\nKernel panic!")
    }
    cpu::wait_forever();
}