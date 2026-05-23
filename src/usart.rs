// USART3 bare-metal driver — TX on PB10 (AF7), RX on PB11 (AF7)
// Baud: 115200 @ 16 MHz HSI (default clock after reset)
// Connect: Feather PB10 → adapter RX, Feather PB11 → adapter TX, GND → GND

const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
const RCC_APB1ENR: *mut u32 = 0x4002_3840 as *mut u32;

const GPIOB_MODER: *mut u32 = 0x4002_0400 as *mut u32;
const GPIOB_AFRH:  *mut u32 = 0x4002_0424 as *mut u32;

const USART3_SR:  *mut u32 = 0x4000_4800 as *mut u32;
const USART3_DR:  *mut u32 = 0x4000_4804 as *mut u32;
const USART3_BRR: *mut u32 = 0x4000_4808 as *mut u32;
const USART3_CR1: *mut u32 = 0x4000_480C as *mut u32;

pub fn init() {
    unsafe {
        // Enable GPIOB clock (AHB1ENR bit 1)
        RCC_AHB1ENR.write_volatile(RCC_AHB1ENR.read_volatile() | (1 << 1));

        // Enable USART3 clock (APB1ENR bit 18)
        RCC_APB1ENR.write_volatile(RCC_APB1ENR.read_volatile() | (1 << 18));

        // PB10 and PB11 → Alternate Function mode (MODER bits = 0b10)
        let moder = GPIOB_MODER.read_volatile();
        GPIOB_MODER.write_volatile(
            (moder & !(3 << 20) & !(3 << 22)) | (2 << 20) | (2 << 22),
        );

        // PB10 = AF7, PB11 = AF7 in AFRH (pins 8-15, each 4 bits)
        // PB10 → AFRH bits [11:8], PB11 → AFRH bits [15:12]
        let afrh = GPIOB_AFRH.read_volatile();
        GPIOB_AFRH.write_volatile(
            (afrh & !(0xF << 8) & !(0xF << 12)) | (7 << 8) | (7 << 12),
        );

        // BRR: 16 MHz HSI / (16 * 115200) = USARTDIV 8.680
        // Mantissa = 8, Fraction = round(0.680 * 16) = 11
        USART3_BRR.write_volatile((8 << 4) | 11);

        // CR1: UE (bit 13) enable USART, TE (bit 3) enable transmitter
        USART3_CR1.write_volatile((1 << 13) | (1 << 3));
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        // Poll TXE (bit 7) — wait until transmit data register is empty
        while USART3_SR.read_volatile() & (1 << 7) == 0 {}
        USART3_DR.write_volatile(byte as u32);
    }
}

pub fn write_str(s: &str) {
    for byte in s.bytes() {
        write_byte(byte);
    }
}
