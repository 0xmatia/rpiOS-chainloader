/*
 * File: cpu.rs
 * Project: RpiOS
 * File Created: Tuesday, 26th October 2021 10:21:41 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

//! Processor code

mod boot;

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;

pub use arch_cpu::{nop, wait_forever};
pub use arch_cpu::spin_for_cycles;