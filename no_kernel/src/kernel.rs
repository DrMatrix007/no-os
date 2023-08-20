#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod vga_buffer;


#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[export_name = "no_kernel_main"]
pub extern "C" fn no_kernel_main() -> i32 {
    println!("hello bozo");
    42
}
