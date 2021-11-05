/*
 * File: boot.s
 * Project: RpiOS
 * File Created: Tuesday, 26th October 2021 10:33:49 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */

/*
Boot code for rpi3 - aarch64.
This is what happens after the bootloader transfer control to the kernel
i.e jumps to 0x8000.
*/

/*
This is a macro that loads an address of a symbol to a register.
The value is relative to the PC.
The adrp instruction loads the higher 23 bits of the symbol to the register,
Its puropse is to load the page (4kb) address of the symbol. by adding the 12 lower bits,
the offset of the symbol is calculated.
"lo12:\symbol" before the symbol to extract its lower 12 bits.
References:
https://developer.arm.com/documentation/dui0802/a/A64-General-Instructions/ADRP
https://stackoverflow.com/questions/41906688/what-are-the-semantics-of-adrp-and-adrl-instructions-in-arm-assembly
https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
*/
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

.equ _core_id_mask, 0b11

.section .text._start

// fn _start() -> do initialization work and call rust code
_start:
    // We have 4 cores. Only proceed with the boot core, core0.
    // move MPIDR_EL1 register content to general purpose register x1   
    mrs x1, MPIDR_EL1
    // mask the mpidr_el1 register to check what core is currently executing
    and x1, x1, _core_id_mask
    // load the boot core id (0) to x2
    ldr x2, BOOT_CORE_ID
    cmp x1, x2 // compare the boot id 
    b.ne _park_core

    // the core executing these lines is the boot core
    ADR_REL x0, __bss_start
    ADR_REL x1, __bss_end_exclusive

_initialize_bss:
    cmp x0, x1
    b.eq _prepare_rust // break the loop if we reached ens of bss
    stp xzr, xzr, [x0], #16 // store two zero values (xzr=0) in the bss section
    b _initialize_bss

_prepare_rust:
    // setting up stack:
    // sp start from pointing at the end of the bss section,
    // and it grows downwards (0x7999 - 0x0000)
    ADR_REL x0, __boot_core_stack_end_exclusive
    mov sp, x0
    // let's begin!
    b __start_rust

_park_core:
    wfe // wait for event
    b _park_core // jump to loop if event occured.

.size _start, . - _start // tells the linker the size of _start, doesn't look important
.type _start, function // start is a function
.global _start // _start is an external symbol ready to link