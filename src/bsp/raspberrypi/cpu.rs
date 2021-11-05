/*
 * File: cpu.rs
 * Project: RpiOS
 * File Created: Friday, 5th November 2021 12:21:21 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */


//! Board specific processor code

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;