#![no_std]
#![no_main]

use core::panic::PanicInfo;
static HELLO: &[u8] = b"I am in space and I see no God!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    {
        let vga_buffer = 0xb8000 as *mut u8;
        for (i, &byte) in HELLO.iter().enumerate() {
            unsafe {
                *vga_buffer.offset(i as isize * 2) = byte; // Chraracter byte
                *vga_buffer.offset(i as isize * 2 + 1) = 0xc; // Colour byte - Red
            }
        }
    };
    loop {}
}
