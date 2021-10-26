/*
 * File: linker.ld
 * Project: RpiOS
 * File Created: Tuesday, 26th October 2021 5:49:15 pm
 * Author: Elad Matia (elad.matia@gmail.com)
 */
kernel_addr_in_memory = 0x80000;
ENTRY(kernel_addr_in_memory)

MEMORY
{
    RAM (rwx) : ORIGIN = kernel_addr_in_memory, LENGTH = 1023M
}

PHDRS
{
    segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
}

SECTIONS
{
    . = kernel_addr_in_memory;
    /*
    Code section. the first instruction is in kernel_addr_in_memory
    create a section called text where code will live
    */
    .text :
    {
        KEEP(*(.text*))
    } > RAM : segment_rx
}
