/* memory.ld — Linker script for STM32F405RGT6 (Cortex-M4)
   Flash:   1 MB @ 0x08000000
   SRAM:    192 KB @ 0x20000000
   CCM RAM: 64 KB @ 0x10000000 (optional, not used here)
   BKPSRAM:  4 KB @ 0x40024000 */

ENTRY(reset_handler)

_Min_Heap_Size = 0x400;     /* 1KB */
_Min_Stack_Size = 0x800;    /* 2KB */

MEMORY {
  FLASH    (rx)  : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM      (rwx) : ORIGIN = 0x20000000, LENGTH = 192K
  CCMRAM   (rwx) : ORIGIN = 0x10000000, LENGTH = 64K 
  BKPSRAM  (rwx) : ORIGIN = 0x40024000, LENGTH = 4K
}

_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS {
  .isr_vector : {
    KEEP(*(.isr_vector))
    . = ALIGN(4);
  } > FLASH

/* Here is where all your executable code should go */
  .text : {
    *(.text*)
    . = ALIGN(4);
  } > FLASH

/* Constant data will go into flash */
  .rodata : {
    *(.rodata*)
    . = ALIGN(4);
  } > FLASH

/* Initialized data will be in the FLASH and it will also be in the RAM during runtime */
  .data : {
    _sidata = LOADADDR(.data); /*This returns the flash (LMA) address*/
    _sdata = . ; /* Start of data section in RAM (VMA)*/
    *(.data*)
    . = ALIGN(4);
    _edata = . ; /* End of data section in RAM (VMA)*/
  } > RAM AT > FLASH

  .bss : {
    _sbss = . ;
    *(.bss*)
    . = ALIGN(4);
    _ebss = . ;
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

  .ram_usage_check :
  {
    . = ALIGN(8);
    . = . + _Min_Stack_Size;
    . = . + _Min_Heap_Size;
    . = ALIGN(8);
  } > RAM
}
