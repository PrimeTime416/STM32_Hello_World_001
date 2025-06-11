// 1. Define the vector table for the mcu
static VECTOR_TABLE: [Option<unsafe fn()>; 95] = [
    Some(reset_handler()),
    Some(nmi_handler()),
    Some(hardfault_handler()),


];
// 2. Define the reset handler
#[unsafe(no_mangle)]
fn reset_handler(){
    crate::main()
}

// 3. Define the exception handler 
#[unsafe(no_mangle)]
fn nmi_handler(){
    loop {
        
    }
}

#[unsafe(no_mangle)]
fn default_handler(){
    loop {
        
    }
}

#[unsafe(no_mangle)]
fn hardfault_handler(){
    loop {
        
    }
}