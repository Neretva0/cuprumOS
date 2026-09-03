#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

static HELLOWORLD: &[u8] = b"Hello World!!!!!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!!!!");

    loop {}
}

//Denna function anropas vid panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
