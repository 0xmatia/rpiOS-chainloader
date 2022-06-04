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

// Load the address of a symbol into a register, absolute.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_ABS register, symbol
	movz	\register, #:abs_g2:\symbol
	movk	\register, #:abs_g1_nc:\symbol
	movk	\register, #:abs_g0_nc:\symbol
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
    ADR_ABS x0, __bss_start
	ADR_ABS x1, __bss_end_exclusive

_initialize_bss:
    cmp x0, x1
    b.eq _relocate_bootloader // break the loop if we reached end of bss
    stp xzr, xzr, [x0], #16 // store two zero values (xzr=0) in the bss section
    b _initialize_bss

_relocate_bootloader:
	ADR_REL x0, __binary_start // Where the binary was loaded (ex. 0x8000)
	ADR_ABS x1, __binary_start // Where the binary was linked (ex. 0x2000000)
	ADR_ABS x2, __binary_end_exclusive // End of the binary.

_copy_loop:
	ldr x3, [x0], #8 //  load to x3 whatever is in x0, advance x0 by 8
	str x3, [x1], #8 //  store whatever is in x3 in the address of x3, advance by 8 x1
	cmp x1, x2
	b.lo _copy_loop

// setting up stack:
ADR_ABS x0, __boot_core_stack_end_exclusive
mov sp, x0
// let's begin!
ADR_ABS x0, _start_rust
br x0

_park_core:
    wfe // wait for event
    b _park_core // jump to loop if event occured.

.size _start, . - _start // tells the linker the size of _start, doesn't look important
.type _start, function // start is a function
.global _start // _start is an external symbol ready to link
