#![no_main]
#![no_std]

use core::ffi::c_void;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FrameData {
    pub ptr: *mut c_void,
    pub width: usize,
    pub height: usize,
    pub size_per_pixel: usize,
    pub size: usize,
    pub pixels_per_scan_line: usize,
}

impl FrameData {
    pub fn get_pixel(&mut self, x: usize, y: usize) -> *mut u32 {
        unsafe { self.ptr.add(4 * self.pixels_per_scan_line * y + 4 * x) as *mut u32 }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PsfFont {
    // PSF v1 font struct
    pub header: PsfHeader,
    pub buffer: *mut usize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PsfHeader {
    // PSF v1 header struct
    pub magic: [u8; 2],
    pub mode: u8,
    pub charsize: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BootInfo {
    // BootInfo contains boot data such as GOP ,font , EfiMemory ,etc...
    pub framebuffer: *mut FrameData,
    pub font: PsfFont,
    // pub mMap: *mut PageFrameAllocator::EfiMemory::EFI_MEMORY_DESCRIPTOR,
    pub map_size: usize,
    pub map_desc_size: usize,
}
