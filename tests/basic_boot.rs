#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cuprum_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use cuprum_os::{exit_qemu, QemuExitCode};

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cuprum_os::test_panic_handler(info)
}
