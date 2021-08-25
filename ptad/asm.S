    .data
    .global a
    .global b
    .align 0x1000   /* 4KiB */
a:
    .word 0x0
    .space 0x1000   /* 4KiB */
b:
    .word 0x0
    .space 0x1000   /* 4KiB */
