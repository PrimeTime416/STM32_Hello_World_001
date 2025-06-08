/* memory.ld — Linker script for STM32F405RGT6 (Cortex-M4)
   Flash:   1 MB @ 0x08000000
   SRAM:    192 KB @ 0x20000000
   CCM RAM: 64 KB @ 0x10000000 (optional, not used here)
   BKPSRAM:  4 KB @ 0x40024000 */

ENTRY(Reset_Handler)

_Min_Heap_Size = 0x400;     /* 1KB */
_Min_Stack_Size = 0x800;    /* 2KB */

MEMORY {
  FLASH    (rx)  : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM      (rwx) : ORIGIN = 0x20000000, LENGTH = 192K
  BKPSRAM  (rwx) : ORIGIN = 0x40024000, LENGTH = 4K
}

_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS {
  .isr_vector : {
    KEEP(*(.isr_vector))
    . = ALIGN(4);
  } > FLASH

  .text : {
    *(.text*)
    . = ALIGN(4);
  } > FLASH

  .rodata : {
    *(.rodata*)
    . = ALIGN(4);
  } > FLASH

  .data : {
    *(.data*)
    . = ALIGN(4);
  } > RAM AT > FLASH

  .bss : {
    *(.bss*)
    . = ALIGN(4);
  } > RAM

  ._user_heap_stack : {
    . = ALIGN(4);
    PROVIDE(_end = .);
    PROVIDE(_heap_start = .);
    . = . + _Min_Heap_Size;
    PROVIDE(_heap_end = .);
    . = . + _Min_Stack_Size;
    . = ALIGN(4);
    PROVIDE(_stack_end = .);
  } > RAM
}
