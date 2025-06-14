#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;
mod startup_stm32f405;
// Placed into CCMRAM for fast access
// #[unsafe(link_section = ".ccmram")]
// static mut CCM_BUFFER: [u8; 512] = [0; 512];

// Stored in backup SRAM so it survives standby mode

// #[unsafe(export_name = "main")]
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    loop {
        
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        
    }
}