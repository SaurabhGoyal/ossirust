#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_writer;

static HELLO: &[u8] = b"I am in space\n  and I see no God!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = vga_writer::Writer::new();
    let mut c = 0;
    loop {
        for byte in HELLO {
            writer.write_byte(*byte);
        }
        c += 1;
        if c == 2500 {
            break;
        }
    }
    // writer.write_byte(0x7f_u8);
    loop {}
}
