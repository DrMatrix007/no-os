#![no_std]
#![no_main]

use core::panic::PanicInfo;

use no_kernel_args::BootInfo;
mod basicRenderer;

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
    let mut frame = unsafe { *(*boot_info).framebuffer };
    // basicRenderer::BasicRenderer(unsafe {&mut *(*boot_info).framebuffer}, unsafe { &mut (*boot_info).font });
    // basicRenderer::Print(r"hello");
    for x in 0..frame.width {
        for y in 0..frame.height {
            core::ptr::write_volatile(frame.get_pixel(x, y), 0x0);
        }
    }

    for x in 0..200 {
        for y in 0..200 {
            core::ptr::write_volatile(frame.get_pixel(x, y), 0xff0000);
        }
    }
     
    #[allow(clippy::empty_loop)]
    loop {}
}
