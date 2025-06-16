/*
Startup code for STM32F405 in embedded Rust.
- Defines the interrupt vector table
- Implements the Reset handler
- Provides default implementations for all other exception/interrupt handlers
*/


unsafe extern "C" {
    unsafe static _estack: u32; // End of RAM, used to initialize MSP
    unsafe static _sidata: u32; // Start of .data section in flash
    unsafe static mut _sdata: u32; // Start of .data section in RAM
    unsafe static mut _edata: u32; // End of .data section in RAM
    unsafe static mut _sbss: u32; // Start of .bss section in RAM
    unsafe static mut _ebss: u32; // End of .bss section in RAM
}

const NONE: VectorEntry = VectorEntry {
    pointer: 0 as *const u32,
};

#[repr(C)]
pub union VectorEntry {
    pub handler: unsafe extern "C" fn(),
    pub pointer: *const u32,
}

unsafe impl Sync for VectorEntry {}
#[unsafe(link_section = ".isr_vector")]
#[unsafe(no_mangle)]
#[used]
pub static VECTOR_TABLE: [VectorEntry; 98] = [
    VectorEntry {
        pointer: unsafe { &_estack },
    }, // Initial Stack Pointer
    VectorEntry {
        handler: Reset_Handler,
    },
    VectorEntry {
        handler: NMI_Handler,
    },
    VectorEntry {
        handler: HardFault_Handler,
    },
    VectorEntry {
        handler: MemManage_Handler,
    },
    VectorEntry {
        handler: BusFault_Handler,
    },
    VectorEntry {
        handler: UsageFault_Handler,
    },
    NONE,
    NONE,
    NONE,
    NONE,
    VectorEntry {
        handler: SVCall_Handler,
    },
    VectorEntry {
        handler: Debug_Monitor_Handler,
    },
    NONE,
    VectorEntry {
        handler: PendSV_Handler,
    },
    VectorEntry {
        handler: SysTick_Handler,
    },
    VectorEntry {
        handler: WWDG_Handler,
    },
    VectorEntry {
        handler: PVD_Handler,
    },
    VectorEntry {
        handler: TAMP_STAMP_Handler,
    },
    VectorEntry {
        handler: RTC_WKUP_Handler,
    },
    VectorEntry {
        handler: FLASH_Handler,
    },
    VectorEntry {
        handler: RCC_Handler,
    },
    VectorEntry {
        handler: EXTI0_Handler,
    },
    VectorEntry {
        handler: EXTI1_Handler,
    },
    VectorEntry {
        handler: EXTI2_Handler,
    },
    VectorEntry {
        handler: EXTI3_Handler,
    },
    VectorEntry {
        handler: EXTI4_Handler,
    },
    VectorEntry {
        handler: DMA1_Stream0_Handler,
    },
    VectorEntry {
        handler: DMA1_Stream1_Handler,
    },
    VectorEntry {
        handler: DMA1_Stream2_Handler,
    },
    VectorEntry {
        handler: DMA1_Stream3_Handler,
    },
    VectorEntry {
        handler: DMA1_Stream4_Handler,
    },
    VectorEntry {
        handler: DMA1_Stream5_Handler,
    },
    VectorEntry {
        handler: DMA1_Stream6_Handler,
    },
    VectorEntry {
        handler: ADC_Handler,
    },
    VectorEntry {
        handler: CAN1_TX_Handler,
    },
    VectorEntry {
        handler: CAN1_RX0_Handler,
    },
    VectorEntry {
        handler: CAN1_RX1_Handler,
    },
    VectorEntry {
        handler: CAN1_SCE_Handler,
    },
    VectorEntry {
        handler: EXTI9_5_Handler,
    },
    VectorEntry {
        handler: TIM1_BRK_TIM9_Handler,
    },
    VectorEntry {
        handler: TIM1_UP_TIM10_Handler,
    },
    VectorEntry {
        handler: TIM1_TRG_COM_TIM11_Handler,
    },
    VectorEntry {
        handler: TIM1_CC_Handler,
    },
    VectorEntry {
        handler: TIM2_Handler,
    },
    VectorEntry {
        handler: TIM3_Handler,
    },
    VectorEntry {
        handler: TIM4_Handler,
    },
    VectorEntry {
        handler: I2C1_EV_Handler,
    },
    VectorEntry {
        handler: I2C1_ER_Handler,
    },
    VectorEntry {
        handler: I2C2_EV_Handler,
    },
    VectorEntry {
        handler: I2C2_ER_Handler,
    },
    VectorEntry {
        handler: SPI1_Handler,
    },
    VectorEntry {
        handler: SPI2_Handler,
    },
    VectorEntry {
        handler: USART1_Handler,
    },
    VectorEntry {
        handler: USART2_Handler,
    },
    VectorEntry {
        handler: USART3_Handler,
    },
    VectorEntry {
        handler: EXTI15_10_Handler,
    },
    VectorEntry {
        handler: RTC_Alarm_Handler,
    },
    VectorEntry {
        handler: OTG_FS_WKUP_Handler,
    },
    VectorEntry {
        handler: TIM8_BRK_TIM12_Handler,
    },
    VectorEntry {
        handler: TIM8_UP_TIM13_Handler,
    },
    VectorEntry {
        handler: TIM8_TRG_COM_TIM14_Handler,
    },
    VectorEntry {
        handler: TIM8_CC_Handler,
    },
    VectorEntry {
        handler: DMA1_Stream7_Handler,
    },
    VectorEntry {
        handler: FSMC_Handler,
    },
    VectorEntry {
        handler: SDIO_Handler,
    },
    VectorEntry {
        handler: TIM5_Handler,
    },
    VectorEntry {
        handler: SPI3_Handler,
    },
    VectorEntry {
        handler: UART4_Handler,
    },
    VectorEntry {
        handler: UART5_Handler,
    },
    VectorEntry {
        handler: TIM6_DAC_Handler,
    },
    VectorEntry {
        handler: TIM7_Handler,
    },
    VectorEntry {
        handler: DMA2_Stream0_Handler,
    },
    VectorEntry {
        handler: DMA2_Stream1_Handler,
    },
    VectorEntry {
        handler: DMA2_Stream2_Handler,
    },
    VectorEntry {
        handler: DMA2_Stream3_Handler,
    },
    VectorEntry {
        handler: DMA2_Stream4_Handler,
    },
    VectorEntry {
        handler: ETH_Handler,
    },
    VectorEntry {
        handler: ETH_WKUP_Handler,
    },
    VectorEntry {
        handler: CAN2_TX_Handler,
    },
    VectorEntry {
        handler: CAN2_RX0_Handler,
    },
    VectorEntry {
        handler: CAN2_RX1_Handler,
    },
    VectorEntry {
        handler: CAN2_SCE_Handler,
    },
    VectorEntry {
        handler: OTG_FS_Handler,
    },
    VectorEntry {
        handler: DMA2_Stream5_Handler,
    },
    VectorEntry {
        handler: DMA2_Stream6_Handler,
    },
    VectorEntry {
        handler: DMA2_Stream7_Handler,
    },
    VectorEntry {
        handler: USART6_Handler,
    },
    VectorEntry {
        handler: I2C3_EV_Handler,
    },
    VectorEntry {
        handler: I2C3_ER_Handler,
    },
    VectorEntry {
        handler: OTG_HS_EP1_OUT_Handler,
    },
    VectorEntry {
        handler: OTG_HS_EP1_IN_Handler,
    },
    VectorEntry {
        handler: OTG_HS_WKUP_Handler,
    },
    VectorEntry {
        handler: OTG_HS_Handler,
    },
    VectorEntry {
        handler: DCMI_Handler,
    },
    VectorEntry {
        handler: CRYP_Handler,
    },
    VectorEntry {
        handler: HASH_RNG_Handler,
    },
    VectorEntry {
        handler: FPU_Handler,
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset_Handler() {
    // Copy .data from flash to RAM
    let mut src = unsafe {&_sidata as *const u32};
    let mut dest = &raw mut _sdata as *mut u32;
while dest <  &raw mut _edata as *mut u32{
        unsafe {
           *dest = *src;
            dest = dest.add(1);
            src = src.add(1);
        }
    }

    // Zero out .bss
    let mut bss = &raw mut _sbss as *mut u32;
    while bss < &raw mut _ebss as *mut u32 {
        unsafe {
            *bss = 0;
            bss = bss.add(1);
        }
    }

    // Call main
    crate::main();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Default_Handler() {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reserved_Handler() {
    unsafe { Default_Handler() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn HardFault_Handler() {
    loop {}
}

macro_rules! stub_handlers {
    ($($name:ident),*) => {
        $(
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn $name() {
                unsafe {Default_Handler()};
            }
        )*
    };
}

stub_handlers!(
    BusFault_Handler,
    MemManage_Handler,
    UsageFault_Handler,
    NMI_Handler,
    SVCall_Handler,
    Debug_Monitor_Handler,
    PendSV_Handler,
    SysTick_Handler,
    WWDG_Handler,
    PVD_Handler,
    TAMP_STAMP_Handler,
    RTC_WKUP_Handler,
    FLASH_Handler,
    RCC_Handler,
    EXTI0_Handler,
    EXTI1_Handler,
    EXTI2_Handler,
    EXTI3_Handler,
    EXTI4_Handler,
    DMA1_Stream0_Handler,
    DMA1_Stream1_Handler,
    DMA1_Stream2_Handler,
    DMA1_Stream3_Handler,
    DMA1_Stream4_Handler,
    DMA1_Stream5_Handler,
    DMA1_Stream6_Handler,
    ADC_Handler,
    CAN1_TX_Handler,
    CAN1_RX0_Handler,
    CAN1_RX1_Handler,
    CAN1_SCE_Handler,
    EXTI9_5_Handler,
    TIM1_BRK_TIM9_Handler,
    TIM1_UP_TIM10_Handler,
    TIM1_TRG_COM_TIM11_Handler,
    TIM1_CC_Handler,
    TIM2_Handler,
    TIM3_Handler,
    TIM4_Handler,
    I2C1_EV_Handler,
    I2C1_ER_Handler,
    I2C2_EV_Handler,
    I2C2_ER_Handler,
    SPI1_Handler,
    SPI2_Handler,
    USART1_Handler,
    USART2_Handler,
    USART3_Handler,
    EXTI15_10_Handler,
    RTC_Alarm_Handler,
    OTG_FS_WKUP_Handler,
    TIM8_BRK_TIM12_Handler,
    TIM8_UP_TIM13_Handler,
    TIM8_TRG_COM_TIM14_Handler,
    TIM8_CC_Handler,
    DMA1_Stream7_Handler,
    FSMC_Handler,
    SDIO_Handler,
    TIM5_Handler,
    SPI3_Handler,
    UART4_Handler,
    UART5_Handler,
    TIM6_DAC_Handler,
    TIM7_Handler,
    DMA2_Stream0_Handler,
    DMA2_Stream1_Handler,
    DMA2_Stream2_Handler,
    DMA2_Stream3_Handler,
    DMA2_Stream4_Handler,
    ETH_Handler,
    ETH_WKUP_Handler,
    CAN2_TX_Handler,
    CAN2_RX0_Handler,
    CAN2_RX1_Handler,
    CAN2_SCE_Handler,
    OTG_FS_Handler,
    DMA2_Stream5_Handler,
    DMA2_Stream6_Handler,
    DMA2_Stream7_Handler,
    USART6_Handler,
    I2C3_EV_Handler,
    I2C3_ER_Handler,
    OTG_HS_EP1_OUT_Handler,
    OTG_HS_EP1_IN_Handler,
    OTG_HS_WKUP_Handler,
    OTG_HS_Handler,
    DCMI_Handler,
    CRYP_Handler,
    HASH_RNG_Handler,
    FPU_Handler
);
