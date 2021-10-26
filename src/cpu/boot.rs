/*
 * File: boot.rs
 * Project: RpiOS
 * File Created: Tuesday, 26th October 2021 10:25:34 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

//! Boot code. This is different for every architecture
 
// look for the module in the cpu boot code specific to aarch64
#[cfg(target_arch="aarch64")]
#[path ="../_arch/aarch64/cpu/boot.rs"]
mod arm_boot;