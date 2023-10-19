// main.rs

// A freestanding executable - no link to std library
#![no_std]
// Tell Rust compiler that we are not using normal entry point chain
#![no_main]


use core::panic::PanicInfo;

#[no_mangle] // Disable name mangling for _start function
// Tell compiler to use C calling convention 
// instead of default Rust calling convention
pub extern "C" fn _start() -> !{
    // the entry point for linker, named '_start' by default
    loop {}
}

// function used to handle panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    // loop for now
    loop {}
}
