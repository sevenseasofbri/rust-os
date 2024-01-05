// main.rs
#![no_std] // do not link the std lib
#![no_main] // disable rust-level entrypoints

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // called on panic
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // do not mangle the name of this function
pub extern "C" fn _start() -> ! { // entry point, linker looks for _start by default
    let vga_buffer = 0xb8000 as *mut u8; // VGA buffer

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // colour: light cyan
        }
    }
    
    loop {}
}
