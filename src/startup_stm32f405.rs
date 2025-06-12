/*
When writing embedded Rust for ARM Cortex-M MCUs (e.g., STM32F405), 
naming interrupt and exception handlers must conform to hardware expectations and C toolchain conventions, 
not Rust's snake_case style. The suffix Handler is capitalized (PascalCase) by long-standing C/assembly convention.
*/

// 1. Define the vector table for the mcu
#[used]
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".isr_vector")]
static VECTOR_TABLE: [Option< unsafe extern "C" fn()>; 97] = [
    Some(Reset_Handler),
    Some(NMI_Handler),
    Some(HardFault_Handler),
    Some(MemManage_Handler),
    Some(BusFault_Handler),
    Some(UsageFault_Handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    None,
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    Some(WWDG_Handler),
    Some(PVD_Handler),
    Some(TAMP_STAMP_Handler),
    Some(RTC_WKUP_Handler),
    None,
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_Stream0_Handler),
    Some(DMA1_Stream1_Handler),
    Some(DMA1_Stream2_Handler),
    Some(DMA1_Stream3_Handler),
    Some(DMA1_Stream4_Handler),
    Some(DMA1_Stream5_Handler),
    Some(DMA1_Stream6_Handler),
    Some(ADC_Handler),
    Some(CAN1_TX_Handler),
    Some(CAN1_RX0_Handler),
    Some(CAN1_RX1_Handler),
    Some(CAN1_SCE_Handler),
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_TIM9_Handler),
    Some(TIM1_UP_TIM10_Handler),
    Some(TIM1_TRG_COM_TIM11_Handler),
    Some(TIM1_CC_Handler),
    Some(TIM2_Handler),
    Some(TIM3_Handler),
    Some(TIM4_Handler),
    Some(I2C1_EV_Handler),
    Some(I2C1_ER_Handler),
    Some(I2C2_EV_Handler),
    Some(I2C2_ER_Handler),
    Some(SPI1_Handler),
    Some(SPI2_Handler),
    Some(USART1_Handler),
    Some(USART2_Handler),
    Some(USART3_Handler),
    Some(EXTI15_10_Handler),
    Some(RTC_Alarm_Handler),
    Some(OTG_FS_WKUP_Handler),
    Some(TIM8_BRK_TIM12_Handler),
    Some(TIM8_UP_TIM13_Handler),
    Some(TIM8_TRG_COM_TIM14_Handler),
    Some(TIM8_CC_Handler),
    Some(DMA1_Stream7_Handler),
    Some(FSMC_Handler),
    Some(SDIO_Handler),
    Some(TIM5_Handler),
    Some(SPI3_Handler),
    Some(UART4_Handler),
    Some(UART5_Handler),
    Some(TIM6_DAC_Handler),
    Some(TIM7_Handler),
    Some(DMA2_Stream0_Handler),
    Some(DMA2_Stream1_Handler),
    Some(DMA2_Stream2_Handler),
    Some(DMA2_Stream3_Handler),
    Some(DMA2_Stream4_Handler),
    Some(ETH_Handler),
    Some(ETH_WKUP_Handler),
    Some(CAN2_TX_Handler),
    Some(CAN2_RX0_Handler),
    Some(CAN2_RX1_Handler),
    Some(CAN2_SCE_Handler),
    Some(OTG_FS_Handler),
    Some(DMA2_Stream5_Handler),
    Some(DMA2_Stream6_Handler),
    Some(DMA2_Stream7_Handler),
    Some(USART6_Handler),
    Some(I2C3_EV_Handler),
    Some(I2C3_ER_Handler),
    Some(OTG_HS_EP1_OUT_Handler),
    Some(OTG_HS_EP1_IN_Handler),
    Some(OTG_HS_WKUP_Handler),
    Some(OTG_HS_Handler),
    Some(DCMI_Handler),
    Some(CRYP_Handler),
    Some(HASH_RNG_Handler),
    Some(FPU_Handler),
];

// 2. Define the reset handler
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset_Handler(){
    crate::main()
}

// 3. Define the exception handler
#[allow(non_snake_case)] 
#[unsafe(no_mangle)]
pub unsafe extern "C" fn NMI_Handler(){
    loop {
        
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Default_Handler(){
    loop {
        
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn HardFault_Handler(){
    loop {
        
    }
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
SVCall_Handler, 
PendSV_Handler, 
SysTick_Handler,
WWDG_Handler, 
PVD_Handler, 
TAMP_STAMP_Handler, 
RTC_WKUP_Handler,
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

