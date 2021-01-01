OUTPUT_ARCH("riscv")
ENTRY(_entry)

SECTIONS 
{
    . = 0x80000000;

    .text :
    {
        PROVIDE(_text_begin = .);
        *(.text.init) *(.text .text.*)
        PROVIDE(_text_end = .);
    }
   PROVIDE(_global_pointer = .);

    .rodata :
    {
        . = ALIGN(16);
        *(.rodata .rodata.*)
    }

    .data :
    {
        . = ALIGN(16);
        *(.data .data.*)
    }

    .bss :
    {
        . = ALIGN(16);
        PROVIDE(_bss_begin = .);
        *(.sbss .sbss.*) *(.bss .bss.*)
        PROVIDE(_bss_end = .);
    }

    PROVIDE(_stack_begin = _bss_end);
    PROVIDE(_stack_end = _stack_begin + 0x80000);
    PROVIDE(_stack = _bss_end + 0x80000);
    PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));
    PROVIDE(_heap_begin = _stack_end);
    PROVIDE(_heap_size = _memory_end - _stack_end);
}