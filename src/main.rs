#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

mod startup_stm32f405;

static mut SCORES_GLOBAL: [i32; 5] = [1, 2, 3, 4, 5];
const NUMBERS: [i32; 5] = [1, 2, 3, 4, 5];
static mut BUFFER: [u8; 1024] = [0; 1024];

// Placed into CCMRAM for fast access
#[unsafe(link_section = ".ccmram")]
static mut CCM_BUFFER: [u8; 512] = [0; 512];

// Stored in backup SRAM so it survives standby mode
#[unsafe(link_section = ".bkpsram")]
static mut BKP_DATA: [u32; 8] = [0; 8];

#[unsafe(export_name = "main")]
pub extern "C" fn main() -> ! {
    let mut _total_score: i32 = 0;
    for score in unsafe { SCORES_GLOBAL } {
        _total_score += score;
    };

    for score in NUMBERS {
    _total_score += score;
}

    unsafe {
        BUFFER[0] = 100;
        CCM_BUFFER[0] = 1;
        BKP_DATA[0] = 42;
    }
    loop {
        
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        
    }
}