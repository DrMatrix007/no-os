#![no_std]
#![no_main]
use core::panic::PanicInfo;

pub mod vga_buffer;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[export_name = "no_kernel_main"]
pub extern "C" fn no_kernel_main() -> i32 {
    //println!("hello bozo");
    //WRITER.lock().write_byte(65);
    // vga_buffer::a();
    // loop{}
    42
}