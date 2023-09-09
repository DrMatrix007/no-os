#![no_main]
#![no_std]

use core::ffi::c_uint;
#[derive(Debug, Clone, Copy)]
pub struct FrameData {
    pub ptr: *mut u8,
    pub width: usize,
    pub height: usize,
    pub size_per_pixel: usize,
    pub pixels_per_scan_line: usize,
}

impl FrameData {
    pub fn get_pixel(&mut self, x: usize, y: usize) -> &mut c_uint {
        unsafe { &mut *(self.ptr.add(4 * self.pixels_per_scan_line * y + 4 * x) as *mut c_uint)}
    }
}
