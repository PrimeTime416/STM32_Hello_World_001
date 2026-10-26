;==============================================================================
; PIC10F206 - Voltage-dependent LED blinker
;
;   Vin < 1.0 V  -> LED blinks SLOW (500 ms on / 500 ms off, 1 Hz)
;   Vin > 1.0 V  -> LED blinks FAST (100 ms on / 100 ms off, 5 Hz)
;
; Why PIC10F206 and not PIC10F202:
;   The PIC10F202 has no ADC and no comparator, so it cannot measure an analog
;   voltage. The PIC10F206 is the same die/pinout/memory (512 words, 24 bytes
;   RAM) plus an analog comparator, so it is a drop-in replacement.
;
; Pinout (SOT-23-6 / DIP-8 identical to PIC10F202):
;   GP0 / CIN+  <- analog input Vin (via optional 1k + 100 nF RC filter)
;   GP1 / CIN-  <- 1.0 V reference (resistor divider from regulated VDD)
;   GP2         -> LED (through 330 R to GND, active high)
;   GP3 / MCLR  -  digital input (MCLR disabled), unused
;
; Clock: internal 4 MHz -> 1 us per instruction cycle.
; Build: gpasm -p p10f206 main.asm   (gputils, MPASM syntax)
;==============================================================================

        list        p=10f206
        #include    <p10f206.inc>
        errorlevel  -302

        __CONFIG    _MCLRE_OFF & _CP_OFF & _WDT_OFF

;------------------------------------------------------------------------------
; Constants
;------------------------------------------------------------------------------
LED_MASK    equ     b'00000100'     ; GP2
TRIS_VAL    equ     b'00001011'     ; GP0,GP1,GP3 inputs; GP2 output
OPTION_VAL  equ     b'11011111'     ; /GPWU=1, /GPPU=1, T0CS=0 (GP2 = I/O)
; CMCON0: /CWU=1  CPREF=1 (+in = CIN+)  CNREF=1 (-in = CIN- pin)  CMPON=1
;         /CMPT0CS=1  POL=1 (non-inverted)  /COUTEN=1 (output NOT on GP2)
CMCON_VAL   equ     b'01111111'

FAST_TICKS  equ     .10             ; 10 x 10 ms = 100 ms per half period
SLOW_TICKS  equ     .50             ; 50 x 10 ms = 500 ms per half period

;------------------------------------------------------------------------------
; RAM (PIC10F206 general purpose registers start at 0x08)
;------------------------------------------------------------------------------
        cblock  0x08
            shadow                  ; GPIO output shadow (avoids R-M-W issues)
            ticks                   ; 10 ms ticks elapsed this half period
            d1                      ; delay inner counter
            d2                      ; delay outer counter
        endc

;------------------------------------------------------------------------------
; Reset vector. Execution begins at 0x1FF, which holds the factory
; "movlw <cal>" instruction, then the PC wraps to 0x000 with W = cal value.
; DO NOT erase the calibration word at 0x1FF.
;------------------------------------------------------------------------------
        org     0x000
        andlw   0xFE                ; keep calibration bits, force FOSC4 = 0
        movwf   OSCCAL              ; so GP2 is not the Fosc/4 clock output
        goto    init

;------------------------------------------------------------------------------
; delay_10ms - ~10.03 ms @ 4 MHz (10 028 cycles incl. call/retlw)
; Baseline core: CALL can only reach addresses 0x00-0xFF, so subroutines
; must live in the first 256 words. Only retlw exists (no return).
;------------------------------------------------------------------------------
delay_10ms
        movlw   .13
        movwf   d2
dly_outer
        clrf    d1                  ; 256 inner passes
dly_inner
        decfsz  d1, f               ; 3 cycles/pass -> 767 cycles
        goto    dly_inner
        decfsz  d2, f
        goto    dly_outer
        retlw   0

;------------------------------------------------------------------------------
; Initialisation
;------------------------------------------------------------------------------
init
        movlw   OPTION_VAL
        option                      ; T0CS=0 frees GP2 from T0CKI
        movlw   CMCON_VAL
        movwf   CMCON0              ; comparator on, inputs on GP0/GP1
        clrf    shadow
        clrf    GPIO
        movlw   TRIS_VAL
        tris    GPIO

;------------------------------------------------------------------------------
; Main loop: toggle LED, then wait until 'ticks' reaches the limit chosen by
; the comparator. The comparator is re-read every 10 ms so a change of Vin
; takes effect within one tick instead of after a full slow half-period.
;------------------------------------------------------------------------------
main_loop
        movlw   LED_MASK
        xorwf   shadow, f           ; toggle LED bit in shadow
        movf    shadow, w
        movwf   GPIO
        clrf    ticks
wait_tick
        call    delay_10ms
        incf    ticks, f
        movlw   FAST_TICKS          ; assume Vin > 1 V
        btfss   CMCON0, CMPOUT      ; CMPOUT=1 when CIN+ > CIN-
        movlw   SLOW_TICKS          ; Vin < 1 V -> slow
        subwf   ticks, w            ; W = ticks - limit, C=1 if ticks >= limit
        btfss   STATUS, C
        goto    wait_tick
        goto    main_loop

        end
