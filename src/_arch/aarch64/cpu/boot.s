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

 .section .text

 _start: 

loop:
    wfe // wait for event
    b loop // jump to loop if event occured.

.size _start, . - _start // tells the linker the size of _start, doesn't look important
.type _start, function // start is a function
.global _start // _start is an external symbol ready to link