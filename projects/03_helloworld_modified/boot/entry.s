.option norvc

.section .text.init

.global _entry
_entry:
.option push
.option norelax
		la		gp, _global_pointer
.option pop
        la sp, _stack_begin
        li a0, 1024 * 4         # 4KiB
        csrr a1, mhartid        # read current hardware thread
        addi a1, a1, 1
        mul a0, a0, a1          # a0 = a0 * (mhartid + 1)
        add sp, sp, a0
		j __start
spin:
        wfi
        j spin
