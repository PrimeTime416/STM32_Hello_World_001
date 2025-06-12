/* memory.ld — Linker script for STM32F405RGT6 (Cortex-M4)
   Flash:   1 MB @ 0x08000000
   SRAM:    192 KB @ 0x20000000
   CCM RAM: 64 KB @ 0x10000000 (optional, not used here)
   BKPSRAM:  4 KB @ 0x40024000 
   
   System Memory       | 0x1FFF\_0000     | 30 KB     | ST bootloader (read-only, user-inaccessible)      |### STM32F405RGT6 Memory Map Summary (from RM0090 Reference Manual)

| Memory Region        | Start Address | Size     | Description                                        |
| -------------------- | ------------- | -------- | -------------------------------------------------- |
| System Memory        | 0x1FFF\_0000  | 30 KB    | ST bootloader and system ROM                       |
| Flash                | 0x0800\_0000  | 1 MB     | User program code and read-only data               |
| CCM SRAM             | 0x1000\_0000  | 64 KB    | Core Coupled SRAM (not DMA-accessible)             |
| SRAM1 + SRAM2        | 0x2000\_0000  | 112 KB   | General-purpose RAM                                |
| SRAM3 (if present)   | 0x2001\_C000  | 16 KB    | Optional additional RAM block (variant-specific)   |
| Peripheral Region    | 0x4000\_0000  | Variable | Memory-mapped peripheral registers (AHB/APB)       |
| Backup SRAM          | 0x4002\_4000  | 4 KB     | Retains data in standby mode (PWR\_CR.EN\_BKPSRAM) |
| System Control Space | 0xE000\_E000  | Variable | NVIC, SCB, SysTick, Debug, ITM, FPU, etc.          |

#### Notes:

* **Flash Memory** is the main program storage, typically mapped to `0x0800_0000`.
* **CCM SRAM** is high-speed SRAM tightly coupled to the CPU core—ideal for performance-critical data.
* **BKPSRAM** must be enabled via the PWR peripheral; can retain contents during standby mode.
* **System Memory** is where ST's internal bootloader resides; used for DFU/UART booting.

This map supports development of accurate linker scripts (`memory.ld`) and peripheral memory access in low-level embedded firmware (Rust `no_std`, C, or Assembly).

### 🧠 What is CCM SRAM good for?

**CCM SRAM** (Core Coupled Memory) is a 64 KB region at `0x1000_0000` that:

* **Is tightly coupled to the CPU core** (via the D-bus, not the system bus)
* **Is faster** than main SRAM for CPU accesses (zero wait states)
* **Is not accessible** by **DMA, peripherals, or the I-bus**
* **Cannot be used** for:

  * Interrupt vector tables
  * DMA buffers
  * ITCM-based code execution

---

### ✅ Use CCM SRAM for:

* **High-speed working buffers** for signal processing, math, or real-time control:

  * Audio buffers
  * FIR/IIR filter state
  * Temporary arrays in a control loop
* **Fast stacks or context memory** for real-time tasks (in cooperative systems)
* **Scratchpad** memory for calculations that are strictly CPU-only

---

### 🚫 Do not use CCM SRAM for:

* Interrupt vector tables (use Flash or SRAM1)
* DMA target or source buffers
* Memory-mapped peripherals
* Code execution via ITCM or via remapped Flash
   */

ENTRY(Reset_Handler)

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

  /* Uninitialized variables explicitly placed in CCMRAM */
  .ccmram (NOLOAD) : {
    _sccmram = .;
    *(.ccmram*)
    . = ALIGN(4);
    _eccmram = .;
  } > CCMRAM

  /* Uninitialized variables that should persist in backup SRAM */
  .bkpsram (NOLOAD) : {
    _sbkpsram = .;
    *(.bkpsram*)
    . = ALIGN(4);
    _ebkpsram = .;
  } > BKPSRAM

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
