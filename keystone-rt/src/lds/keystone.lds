OUTPUT_ARCH("riscv")

SECTIONS {
    . = 0xFFFFFFFFC0000000;
    PROVIDE(rt_base = .);
    .text : {
        *(.text.init)
        *(.text .text.*)
    }
    . = ALIGN(0x1000);
    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(8);
    .data : {
        *(.sdata .sdata.*)
        *(.data .data.*)
    }
    . = ALIGN(8);
    .bss : {
        _bss_start = .;
        *(.sbss .sbss.*) *(.bss .bss.*)
        _bss_end = .;
    }
    . = ALIGN(0x1000);
    .stack : {
        . += 0x8000;
        PROVIDE(_stack_end = .);
    }
    .page_table : {
        PROVIDE(rt_page_table = .);
        . += 0x10000;
    }

    _end = .;
}
