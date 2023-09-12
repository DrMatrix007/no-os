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
    for x in 0..frame.width {
        for y in 0..frame.height {
            core::ptr::write_volatile(frame.get_pixel(x, y), 0xffff);
        }
    }

    #[allow(clippy::empty_loop)]
    loop {}
}
