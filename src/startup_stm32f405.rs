/*
When writing embedded Rust for ARM Cortex-M MCUs (e.g., STM32F405), 
naming interrupt and exception handlers must conform to hardware expectations and C toolchain conventions, 
not Rust's snake_case style. The suffix Handler is capitalized (PascalCase) by long-standing C/assembly convention.
*/

// 1. Define the vector table for the mcu
static VECTOR_TABLE: [Option<unsafe fn()>; 95] = [
    Some(Reset_Handler()),
    Some(NMI_Handler()),
    Some(HardFault_Handler()),


];
// 2. Define the reset handler
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
fn Reset_Handler(){
    crate::main()
}

// 3. Define the exception handler
#[allow(non_snake_case)] 
#[unsafe(no_mangle)]
fn NMI_Handler(){
    loop {
        
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn Default_Handler(){
    loop {
        
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
fn HardFault_Handler(){
    loop {
        
    }
}