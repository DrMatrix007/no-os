#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn no_kernel_main() {
    loop{}
}
