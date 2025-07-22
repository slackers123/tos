OUTPUT_ARCH("riscv")

ENTRY(boot)

MEMORY
{
	ram   (wxa) : ORIGIN = 0x80200000, LENGTH = 128M
}

PHDRS
{
	text PT_LOAD;
	data PT_LOAD;
	bss PT_LOAD;
}

SECTIONS {
  .text : {
    KEEP(*(.text.boot));

    *(.text .text.*)

  } >ram AT>ram :text

  .rodata : {
    *(.rodata .rodata.*);

  } >ram AT>ram :text

  .data : {
    . = ALIGN(4096);

    *(.data .data.*);

  } >ram AT>ram :data

  .bss : {
    __bss = .;
    *(.bss .bss.* .sbss .sbss.*);
    __bss_end = .;
  } >ram AT>ram :bss

  PROVIDE(__memory_start = ORIGIN(ram));

  PROVIDE(__stack_start = __bss_end);
  PROVIDE(__stack_top = __stack_start + 0x80000);
  PROVIDE(__memory_end = ORIGIN(ram) + LENGTH(ram));

  PROVIDE(__heap_start = __stack_top);
  PROVIDE(__heap_size = __memory_end - __heap_start);
}
