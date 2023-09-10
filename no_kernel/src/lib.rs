#![no_std]
#![no_main]

use core::{
    ffi::c_void,
    panic::{self, PanicInfo},
};

use no_kernel_args::{FrameData, PSF1_FONT};
mod basicRenderer;


#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[derive(Debug)]
struct A {}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BootInfo {
    // BootInfo contains boot data such as GOP ,font , EfiMemory ,etc...
    pub framebuffer: *mut FrameData,
    pub psf1_Font: *mut PSF1_FONT,
    // pub mMap: *mut PageFrameAllocator::EfiMemory::EFI_MEMORY_DESCRIPTOR,
    pub mMapSize: usize,
    pub mMapDescSize: usize,
}

/// # Safety
/// bro
#[export_name = "no_kernel_main"]
pub unsafe extern "C" fn no_kernel_main(bootInfo: *mut BootInfo) -> i32 {
    let mut frame = unsafe {(*(*bootInfo).framebuffer)};
    for x in 0..frame.width {
        for y in 0..frame.height {
            core::ptr::write_volatile(frame.get_pixel(x, y), 0xffff);
        }
    }

    loop {}
    42
}
