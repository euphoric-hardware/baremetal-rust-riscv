.global _start

_start:
    /* Set up stack pointer. */
#define STKSHIFT 17
    la tp, _end + 63
    and tp, tp, -64

    li a0, 0
    add sp, a0, 1
    sll sp, sp, STKSHIFT
    add sp, sp, tp
    sll a2, a0, STKSHIFT
    add tp, tp, a2
    j       _init

.bss

stacks:
    .skip 1024

# .section ".tohost","aw",@progbits
# .align 6
# .globl tohost
# tohost: .dword 0
# .align 6
# .globl fromhost
# fromhost: .dword 0
