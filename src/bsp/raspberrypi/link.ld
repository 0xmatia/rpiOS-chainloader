/*
 * File: linker.ld
 * Project: RpiOS
 * File Created: Tuesday, 26th October 2021 5:49:15 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */
kernel_addr_in_memory = 0x80000;
ENTRY(kernel_addr_in_memory)

/* Apparenly not needed :) */
/* MEMORY
{
    RAM (rwx) : ORIGIN = kernel_addr_in_memory, LENGTH = 1023M
} */

PHDRS
{
    segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
    segment_rw PT_LOAD FLAGS(6); /* 6 == RW */
}

SECTIONS
{
    . = kernel_addr_in_memory;

    /*
    This is the "start" of the stack section for the boot core.
    It grows downwards (0x7999-0x0000)
    */
    __boot_core_stack_end_exclusive = .;

    /*
    Code section. the first instruction is in kernel_addr_in_memory
    create a section called text where code will live
    */
    .text :
    {
        KEEP(*(.text._start)) /* found in boot.s */
        *(.text._start_arguments) /* constants */
        *(.text._start_rust)  /* rust entry point*/
        *(.text*) /* the rest */
    } :segment_rx

    .rodata : ALIGN(8)
    { 
        *(.rodata*)
    } :segment_rx

    .got : ALIGN(8) { *(.got) } :segment_rx

    .data : { *(.data) } :segment_rw

    /* bsses */
    .bss : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*)
        . = ALIGN(16);
        __bss_end_exclusive = .;
    } :NONE
}
