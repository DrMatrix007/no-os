#![no_std]
#![no_main]

use core::{ffi::c_void, panic::PanicInfo};

use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};

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
pub unsafe extern "C" fn no_kernel_main(gop: *mut GraphicsOutput) -> i32 {
    let gop = unsafe { &mut *gop };
    // *core::ptr::null_mut() = 0;

    gop.blt(BltOp::VideoFill {
        color: BltPixel::new(0x69, 0x69, 0x69),
        dest: (0, 0),
        dims: gop.current_mode_info().resolution(),
    })
    .unwrap();

    loop{}

    42
}
