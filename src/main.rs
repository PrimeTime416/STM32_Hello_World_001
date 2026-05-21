#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;
mod startup_stm32f405;
mod usart;

const RCC_AHB1ENR: *mut u32 = (0x4002_3800 + 0x30) as *mut u32;
const GPIOC_MODER:  *mut u32 = (0x4002_0800 + 0x00) as *mut u32;
const GPIOC_ODR:    *mut u32 = (0x4002_0800 + 0x14) as *mut u32;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable GPIOC clock (AHB1ENR bit 2)
        RCC_AHB1ENR.write_volatile(RCC_AHB1ENR.read_volatile() | (1 << 2));

        // PC1 as output: MODER1[1:0] = 0b01 (bits 3:2)
        GPIOC_MODER.write_volatile((GPIOC_MODER.read_volatile() & !(3 << 2)) | (1 << 2));
    }

    usart::init();
    usart::write_str("Hello, World!\r\n");

    loop {
        unsafe { GPIOC_ODR.write_volatile(GPIOC_ODR.read_volatile() | (1 << 1)); }
        delay(100_000);
        unsafe { GPIOC_ODR.write_volatile(GPIOC_ODR.read_volatile() & !(1 << 1)); }
        delay(100_000);
    }
}

#[inline(never)]
fn delay(count: u32) {
    for _ in 0..count {
        core::hint::black_box(());
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
