# PIC10F206 Voltage-Dependent LED Blinker

The LED blinks **slow (1 Hz)** when Vin < 1.0 V and **fast (5 Hz)** when Vin > 1.0 V.

## Why PIC10F206 instead of PIC10F202

The **PIC10F202 has no ADC and no comparator**, so it cannot sense an analog
voltage. The **PIC10F206** is the same part with an analog comparator added:
same 512-word flash, same 24 bytes of RAM, same SOT-23-6 / DIP-8 pinout. It drops
straight into the same footprint.

(The gputils header confirms this: address `0x07` is `CMCON0` on the 10F206
and is marked `__BADRAM` on the 10F202.)

## Language

The firmware is written in **assembly** using MPASM syntax and built with
**gputils `gpasm`**. On a baseline core with 512 words, a 2-level stack and
24 bytes of RAM, assembly gives exact cycle timing and full control over the
`OPTION`/`TRIS` special instructions. The whole program is 33 words.

## Wiring (VDD = 5 V regulated)

```
            VDD (5 V)
             |
            [R3 40.2k]                 PIC10F206 (DIP-8)
             |                        +-----------------+
             +---------- 1.00 V ----->| GP1 / CIN-   (4)|
             |                        |                 |
            [R4 10k]   Vin --[1k]--+->| GP0 / CIN+   (5)|
             |                     |  |                 |
            GND                [100nF]|         GP2  (3)|---[330R]--|>|-- GND
                                   |  |                 |          LED
                                  GND | VDD (2)  VSS (7)|
                                      +-----------------+
   Decouple VDD-VSS with 100 nF at the pins.
   DIP-8 pin numbers shown; check the datasheet for SOT-23-6 numbering.
```

* Vref = 5.0 V × 10k / (40.2k + 10k) = **0.996 V**. Use 1 % resistors.
* The 1k + 100 nF RC filter (fc ≈ 1.6 kHz) reduces chatter near the threshold.

### Alternative: internal 0.6 V reference (frees GP1)

Set `CNREF = 0` (`CMCON_VAL equ b'01111011'`) and feed Vin through a divider
into GP0: Vin → 10k → GP0 → 15k → GND. That puts 0.6 V on GP0 when Vin = 1.0 V.
The internal reference tolerance is wider than a 1 % divider on a regulated
rail, so the threshold is less accurate.

## Build

```bash
sudo apt install gputils
gpasm -p p10f206 main.asm      # -> main.hex
```

## Program

Use a PICkit 3/4/5, MPLAB Snap or ICD with MPLAB IPE and import `main.hex`.
**Preserve the OSCCAL calibration word at address 0x1FF.** The programmers do
this by default; don't override it.

ICSP pins: GP0 = ICSPDAT, GP1 = ICSPCLK, GP3 = MCLR/VPP. Disconnect or isolate
the RC filter and divider while programming if they load the lines.

## Timing

| Condition   | Half-period | Blink rate |
|-------------|-------------|------------|
| Vin < 1.0 V | ~502 ms     | ~1 Hz      |
| Vin > 1.0 V | ~100 ms     | ~5 Hz      |

The comparator is checked every 10 ms, so the blink rate changes within about
10 ms of Vin crossing the threshold.
