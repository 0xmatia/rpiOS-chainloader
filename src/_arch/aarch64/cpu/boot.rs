/*
 * File: boot.rs
 * Project: RpiOS
 * File Created: Tuesday, 26th October 2021 10:31:22 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

 //! Include the assembly file that is responsible for booting the kernel
 //! for the aarch64 architecture.

use crate::kernel_init;
 
core::arch::global_asm!(include_str!("boot.s"));


#[no_mangle]
pub unsafe fn _start_rust() {
    kernel_init()
}

