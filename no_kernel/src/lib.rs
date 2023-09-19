#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod renderer;

use no_kernel_args::BootInfo;
use renderer::{Color, Writer};

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[derive(Debug)]
struct A {}

/// # Safety
/// bro
#[export_name = "no_kernel_main"]
pub unsafe extern "C" fn no_kernel_main(boot_info: *mut BootInfo) -> i32 {
    let frame = unsafe { *boot_info }.framebuffer;
    let mut w = Writer::new(frame);

    w.clear(Color::BLACK);
    w.color(Color::BLACK);
   
    w.println("Hello world!");
    w.color(Color::BLACK);
    w.println("Welcome to NO_OS!");

    w.color(Color::BLACK);
    w.println("Written By Nadav and Ofri");
    42
}
