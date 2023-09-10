#![no_main]
#![no_std]

use core::ffi::c_uint;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FrameData {
    pub ptr: *mut u8,
    pub width: usize,
    pub height: usize,
    pub size_per_pixel: usize,
    pub pixels_per_scan_line: usize,
}

impl FrameData {
    pub fn get_pixel(&mut self, x: usize, y: usize) -> *mut u32 {
        unsafe {self.ptr.add(4 * self.pixels_per_scan_line * y + 4 * x) as *mut u32}
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSF1_FONT {
    // PSF v1 font struct
    pub psf1_Header: *mut PSF1_HEADER,
    pub glyphBuffer: *mut (),
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSF1_HEADER {
    // PSF v1 header struct
    pub magic: [u8; 2],
    pub mode: u8,
    pub charsize: u8,
}