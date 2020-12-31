OUTPUT_ARCH("riscv")
ENTRY(_entry)

SECTIONS 
{
    . = 0x80000000;

    .text :
    {
        *(.text.init) *(.text .text.*)
        PROVIDE(extent = .);
    }

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
        *(.bss .bss.*)
    }

    PROVIDE(end = .);
}