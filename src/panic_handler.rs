use core::panic::PanicInfo;

#[panic_handler]
/// Dummy panic handler
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}