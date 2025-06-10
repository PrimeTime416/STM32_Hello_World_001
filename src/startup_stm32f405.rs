// 1. Define the vectort table for the mcu

// 2. Define the reset handler
#[unsafe(no_mangle)]
fn reset_handler(){
    crate::main()
}

// 3. Define the exception handler 
