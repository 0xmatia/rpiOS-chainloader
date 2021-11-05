use core::panic::PanicInfo;
use crate::cpu;

#[panic_handler]
/// Dummy panic handler
fn panic(_info: &PanicInfo) -> ! {
    cpu::wait_forever();
}