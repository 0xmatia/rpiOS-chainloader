/*
 * File: cpu.rs
 * Project: RpiOS
 * File Created: Friday, 5th November 2021 1:02:24 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

//! Architectural processor code.
//!
//! # Orientation
//!
//! Since arch modules are imported into generic modules using the path attribute, the path of this
//! file is:
//!
//! crate::cpu::arch_cpu

use cortex_a::asm;

pub use asm::nop; // export cpu::nop() for waiting

/// Pause execution on the core.
#[inline(always)]
pub fn wait_forever() -> ! {
    loop{
        asm::wfe();
    }
}

#[cfg(feature = "bsp_rpi3")]
#[inline(always)]
pub fn spin_for_cycles(cycles: usize) {
    for _ in 0..cycles {
        asm::nop();
    }
}