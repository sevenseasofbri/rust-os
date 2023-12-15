// main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! { //entry point
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // called on panic
    loop {}
}