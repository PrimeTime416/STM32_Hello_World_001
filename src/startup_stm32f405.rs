// 1. Define the vector table for the mcu
static VECTOR_TABLE: [Option<unsafe fn()>; 95] = [
    Some(Reset_Handler()),
    Some(NMI_Handler()),
    Some(HardFault_Handler()),


];
// 2. Define the reset handler
#[unsafe(no_mangle)]
fn Reset_Handler(){
    crate::main()
}

// 3. Define the exception handler 
#[unsafe(no_mangle)]
fn NMI_Handler(){
    loop {
        
    }
}

#[unsafe(no_mangle)]
fn default_handler(){
    loop {
        
    }
}

#[unsafe(no_mangle)]
fn HardFault_Handler(){
    loop {
        
    }
}