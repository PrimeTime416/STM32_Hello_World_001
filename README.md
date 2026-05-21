# STM32F405 Feather — Bare-Metal Rust

A from-scratch bare-metal Rust project for the **Adafruit Feather STM32F405 Express**.  
No HAL crates. No `cortex-m-rt`. Everything hand-written.

---

## Versions

| Version | Description |
|---------|-------------|
| `hello_world_001` | LED blink — bare register access, no dependencies |
| `hello_world_002` | USART3 "Hello, World!" — polled TX over PB10, 115200 baud, external USB-to-UART adapter |

---

## Planning

### Current Focus
- Hardware test: flash hello_world_002, wire USB-to-UART adapter, verify "Hello, World!" in PuTTY

### Chosen Approach: USART3 via external USB-to-UART adapter ✅
Option 2 was selected. ST-LINK/V2 (standalone dongle) does not have a VCP bridge — that feature is only on V2-1 (Nucleo) and V3. USART3 via an external FTDI/CP2102 adapter is the correct path for this hardware.

#### Wiring
| Feather pin | Adapter pin |
|---|---|
| PB10 (TX / GPIO 1) | RX |
| PB11 (RX / GPIO 0) | TX |
| GND | GND |

#### PuTTY / Tera Term settings
- Port: COM port assigned to adapter (e.g. COM5 / /dev/ttyUSB0)
- Baud: 115200, 8N1

### Logging Options Considered

| Feature | Option 1: ITM/SWO | Option 2: USART3 VCP ✅ |
|---|---|---|
| Extra wires | 1 (PB3) | 2 (PB10, PB11) |
| Direction | Output only | Full duplex |
| MCU overhead | Extremely low | Standard (polled) |
| PuTTY support | No | Yes |
| Works with ST-LINK/V2 | Yes (SWV) | Yes (external adapter) |

### Next Steps
- [ ] Wire USB-to-UART adapter to Feather PB10/PB11/GND
- [ ] Flash hello_world_002 with `cargo flash --chip STM32F405RG`
- [ ] Open PuTTY at 115200 baud and verify "Hello, World!" output
- [ ] Expand logging: add formatted output, multiple log lines, or RX input handling
- [ ] Update Session Log with hardware test results

### Open Questions
- None — implementation complete, pending hardware verification

---

## Hardware

| Item | Detail |
|------|--------|
| Board | Adafruit Feather STM32F405 Express |
| MCU | STM32F405RGT6 (Cortex-M4F, 168 MHz) |
| Flash | 1 MB |
| SRAM | 128 KB (SRAM1 + SRAM2) |
| CCM RAM | 64 KB (CPU-only, no DMA) |
| Onboard LED | PC1 (red) |

---

## Known Gotchas

**NRST on the ribbon cable holds the MCU in permanent reset.**  
Pin 15 of the ST-Link 20-pin header is NRST. When the ST-Link USB is unplugged but the ribbon cable remains connected, the unpowered ST-Link circuitry pulls NRST low — holding the STM32 in reset indefinitely. The MCU cannot boot, and pressing the Reset button on the Feather makes no difference because NRST is immediately pulled low again by the cable. Fix: remove or leave Pin 15 (NRST) unconnected in the ribbon cable. NRST is not required for SWD flashing with probe-rs. Always physically disconnect the SWD ribbon cable when testing standalone (USB-C only) behaviour.

**Pin 1 (VAPP) is a target voltage sense INPUT, not a power supply.**  
The Feather's 3.3V feeds into the ST-Link via Pin 1 so the ST-Link can calibrate its level-shifters (confirmed in ST UM1075). The ST-Link does not power the target through this pin. When the ST-Link USB is unplugged, its unpowered internals can also load the 3.3V rail through this pin. Fix: leave Pin 1 unconnected — the ST-Link defaults to 3.3V and probe-rs works without it. Alternatively add a 10kΩ series resistor on the VAPP line to limit current when unpowered.

**Orange charging LED flickers with no LiPo battery.**  
The MCP73831 charger's status LED flickers when no battery is connected. This is normal — it does not indicate a fault.

---

## Toolchain

| Tool | Purpose |
|------|---------|
| `rustup` target `thumbv7em-none-eabihf` | Cross-compile for Cortex-M4F |
| `probe-rs` / `cargo-flash` | Flash via ST-Link |
| ST-Link V2 | Debug probe (SWD) |

### Install the target and probe-rs

```powershell
rustup target add thumbv7em-none-eabihf
cargo install probe-rs-tools --locked
```

---

## Build and Flash

```powershell
# Compile only
cargo build

# Compile + flash (debug)
cargo flash --chip STM32F405RG

# Compile + flash (release — optimised, faster execution)
cargo flash --chip STM32F405RG --release
```

> **Note:** `cargo run` also flashes but may show harmless `SwdDpError` warnings on  
> detach. This is because the Feather's NRST pin is not wired to the ST-Link.  
> Use `cargo flash` for a clean output.

---

## Project Structure

```
hello_world_001/
├── src/
│   ├── main.rs                  # Application entry point
│   └── startup_stm32f405.rs     # Vector table, Reset_Handler, stub ISRs
├── memory.ld                    # Linker script (memory regions + sections)
├── .cargo/
│   └── config.toml              # Target, linker flags, probe-rs runner
└── Cargo.toml
```

---

## How the Code Works (`src/main.rs`)

### Crate-level attributes

```rust
#![no_std]   // no standard library — no heap, no OS, no println!
#![no_main]  // we supply our own entry point, not the normal Rust main()
```

Mandatory for bare-metal. Without them Rust links against OS facilities that don't exist on the chip.

---

### Memory-mapped register addresses

```rust
const RCC_AHB1ENR: *mut u32 = (0x4002_3800 + 0x30) as *mut u32;
const GPIOC_MODER: *mut u32 = (0x4002_0800 + 0x00) as *mut u32;
const GPIOC_ODR:   *mut u32 = (0x4002_0800 + 0x14) as *mut u32;
```

These are raw pointers to **memory-mapped hardware registers** — addresses where the CPU reads and writes to control peripherals directly, without any OS or driver layer.

| Constant | Address | Purpose |
|----------|---------|---------|
| `RCC_AHB1ENR` | `0x40023830` | Enable/disable peripheral clocks |
| `GPIOC_MODER` | `0x40020800` | Set each pin: input / output / alternate / analog |
| `GPIOC_ODR` | `0x40020814` | Drive each pin high or low |

---

### Step 1 — Enable the GPIOC clock

```rust
RCC_AHB1ENR.write_volatile(RCC_AHB1ENR.read_volatile() | (1 << 2));
```

On STM32, every peripheral starts with its clock **gated off** to save power. Bit 2 of `AHB1ENR` is the GPIOC clock enable. Without this, all reads and writes to GPIOC registers are silently ignored.

```
AHB1ENR register:
  bit 0 = GPIOAEN
  bit 1 = GPIOBEN
  bit 2 = GPIOCEN  ← set this to enable GPIOC
  bit 3 = GPIODEN
  ...
```

`write_volatile` / `read_volatile` tell the compiler this is a hardware register — never cache it, never reorder it.

---

### Step 2 — Configure PC1 as an output

```rust
GPIOC_MODER.write_volatile((GPIOC_MODER.read_volatile() & !(3 << 2)) | (1 << 2));
```

`MODER` assigns 2 bits to each pin. PC1 uses bits [3:2]:

```
& !(3 << 2)  →  clear bits 3:2  (safe — sets to 00 = input first)
|  (1 << 2)  →  set  bits 3:2 to 01 = output mode
```

```
MODER bit layout (port C):
  bits [1:0]  = PC0
  bits [3:2]  = PC1  ← set to 01 (output)
  bits [5:4]  = PC2
  ...
```

The read-modify-write pattern preserves the state of all other pins.

---

### Step 3 — Blink loop

```rust
loop {
    GPIOC_ODR.write_volatile(GPIOC_ODR.read_volatile() | (1 << 1));  // PC1 HIGH → LED on
    delay(100_000);
    GPIOC_ODR.write_volatile(GPIOC_ODR.read_volatile() & !(1 << 1)); // PC1 LOW  → LED off
    delay(100_000);
}
```

`ODR` bit 1 = PC1. Setting it drives the pin to 3.3 V (LED on); clearing it drives it to 0 V (LED off).

---

### Delay function

```rust
#[inline(never)]
fn delay(count: u32) {
    for _ in 0..count {
        core::hint::black_box(());
    }
}
```

A simple busy-wait loop. `black_box(())` prevents the compiler from optimising the loop body away. `#[inline(never)]` prevents the compiler from collapsing the whole function at the call site. Not cycle-accurate — execution time depends on the build profile (`dev` vs `release`).

---

### Panic handler

```rust
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

Required by `#![no_std]`. If the firmware hits a panic condition (e.g. out-of-bounds slice), execution lands here and spins forever. Future versions will flash an error LED or log over UART.

---

## Memory Map

### Linker regions (`memory.ld`)

```
FLASH    (rx)  : ORIGIN = 0x08000000, LENGTH = 1024K
RAM      (rwx) : ORIGIN = 0x20000000, LENGTH = 128K
CCMRAM   (rwx) : ORIGIN = 0x10000000, LENGTH = 64K
BKPSRAM  (rwx) : ORIGIN = 0x40024000, LENGTH = 4K
```

### Section layout

```
FLASH  0x08000000 ┌─────────────────────────┐
                  │  .isr_vector             │  98-entry vector table
                  │  .text                   │  executable code
                  │  .rodata                 │  constants / string literals
                  │  .data (load image)      │  copied to RAM by Reset_Handler
       0x080FFFFF └─────────────────────────┘

CCM    0x10000000 ┌─────────────────────────┐
                  │  .ccmram  (NOLOAD)       │  CPU-only fast SRAM, no DMA
       0x1000FFFF └─────────────────────────┘

RAM    0x20000000 ┌─────────────────────────┐
                  │  .data  (runtime copy)   │
                  │  .bss   (zeroed)         │
                  │  heap   (1 KB)           │
                  │  stack  (2 KB)           │
       0x2001FFFF └─────────────────────────┘
                    ↑ _estack = 0x20020000 (initial MSP)

BKPSM  0x40024000 ┌─────────────────────────┐
                  │  .bkpsram (NOLOAD)       │  survives standby mode
       0x40024FFF └─────────────────────────┘
```

### Key linker symbols (used by `startup_stm32f405.rs`)

| Symbol | Value | Meaning |
|--------|-------|---------|
| `_estack` | `0x20020000` | Initial stack pointer (top of RAM) |
| `_sidata` | (in Flash) | Load address of `.data` section |
| `_sdata` / `_edata` | (in RAM) | Runtime address range of `.data` |
| `_sbss` / `_ebss` | (in RAM) | Range of `.bss` to zero-fill |

### STM32F405 peripheral address space

```
0x08000000  Flash          — firmware
0x20000000  SRAM1+2        — stack, heap, globals
0x10000000  CCM RAM        — fast CPU-only scratchpad

0x40000000  APB1            TIM2–7, USART2–3, SPI2–3, I2C1–3 ...
0x40010000  APB2            TIM1/8/9–11, USART1/6, SPI1, ADC1–3 ...
0x40020000  AHB1
            ├─ GPIOA  0x40020000
            ├─ GPIOB  0x40020400
            ├─ GPIOC  0x40020800  ← LED on PC1
            ├─ GPIOD  0x40020C00
            └─ RCC    0x40023800

0xE0000000  Cortex-M4 core — NVIC, SysTick, SCB, FPU, ITM ...
```

### GPIOC register map (from RM0090)

| Offset | Register | Purpose |
|--------|----------|---------|
| `+0x00` | `MODER` | Pin mode: input / output / alternate / analog |
| `+0x04` | `OTYPER` | Output type: push-pull / open-drain |
| `+0x08` | `OSPEEDR` | Output slew rate |
| `+0x0C` | `PUPDR` | Pull-up / pull-down |
| `+0x10` | `IDR` | Input data register (read pin states) |
| `+0x14` | `ODR` | Output data register ← **used in this project** |
| `+0x18` | `BSRR` | Atomic bit set/reset (preferred over ODR for multi-pin) |

---

## Startup Flow (`src/startup_stm32f405.rs`)

```
Power on / reset
      │
      ▼
VECTOR_TABLE[1] → Reset_Handler
      │
      ├── Copy .data from Flash → RAM
      ├── Zero-fill .bss in RAM
      │
      ▼
   main()
      │
      ├── Enable GPIOC clock
      ├── Set PC1 as output
      ├── usart::init() — enable GPIOB + USART3 clocks, configure PB10/PB11 AF7, set BRR, enable TE+UE
      ├── usart::write_str("Hello, World!\r\n") — polled TX
      │
      ▼
   loop { LED on → delay → LED off → delay }
```

The vector table (`VECTOR_TABLE`) is a 98-entry array placed in Flash at `0x08000000`. Entry 0 is the initial stack pointer (`_estack`), entry 1 is `Reset_Handler`. All unused interrupt slots default to `Default_Handler` (infinite loop).

---

## Session Log

### Session 001 — 2026-05-21
**Changes:** None (exploration session)
**Summary:** Explored project structure. Confirmed target board is Adafruit Feather STM32F405 Express. Identified PC1 as onboard red LED. Discussed adding UART Hello World as next feature. Established README.md as persistent session memory.
**Lessons learned:**
- NRST pin on SWD ribbon cable must be left unconnected to avoid holding MCU in reset
- probe-rs is used for flashing via ST-Link over SWD
- README.md will be used as persistent memory — Planning section updated each session, Session Log appended after each session

### Session 002 — 2026-05-21
**Changes:** Added `src/usart.rs`, updated `src/main.rs`
**Summary:** Researched and evaluated two console logging options (ITM/SWO vs USART3 VCP). Determined ST-LINK/V2 standalone dongle has no VCP bridge (V2-1/Nucleo only), so USART3 via external USB-to-UART adapter was chosen. Implemented bare-metal USART3 driver from scratch: clock enable, GPIO alternate function config (PB10/PB11 AF7), baud rate register, polled TX. "Hello, World!\r\n" is sent once on boot before the LED blink loop. Code cross-compiled cleanly in cloud container.
**Lessons learned:**
- ST-LINK/V2 standalone supports SWV (ITM/SWO) but has NO virtual COM port bridge — that requires V2-1 or V3
- SWO is output-only (MCU → PC); no data path back to MCU
- ST-LINK/V2 pin 13 (TDO_SWO) carries TRACESWO — adding this wire enables ITM in future
- BRR for 115200 baud at 16 MHz HSI: mantissa=138, fraction=14 → `(138 << 4) | 14`
- Cloud container can cross-compile bare-metal Rust (confirms code correctness before flashing)
- Pending: hardware test with USB-to-UART adapter on PB10/PB11
