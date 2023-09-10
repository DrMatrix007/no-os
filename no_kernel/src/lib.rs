#![no_std]
#![no_main]

use core::{
    ffi::c_void,
    panic::{self, PanicInfo},
};

use no_kernel_args::FrameData;

pub mod vga_buffer;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[derive(Debug)]
struct A {}

/// # Safety
/// bro
#[export_name = "no_kernel_main"]
pub unsafe extern "C" fn no_kernel_main(frame: *mut FrameData) -> i32 {
    let mut frame = unsafe {*frame};
    for x in 0..100 {
        for y in 0..100 {
            core::ptr::write_volatile(frame.get_pixel(x, y), 0xffff);
        }
    }

    loop {}
    42
}
