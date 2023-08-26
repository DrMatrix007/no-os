#![no_main]
#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// extern crate no_kernel;

// use no_kernel::no_kernel_main;
// use uefi::{prelude::*, CStr16};
// use uefi_services::println;

// #[entry]
// fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
//     uefi_services::init(&mut system_table).unwrap();
//     println!("Hello world! lol bozo");
//     let ans = unsafe { no_kernel_main() };

//     println!("{}",ans);

//     system_table.boot_services().stall(10_000_000);
//     Status::SUCCESS
// }

/////////////////

extern crate alloc;


use alloc::vec;
use alloc::vec::Vec;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, FrameBuffer, GraphicsOutput};
use uefi::proto::media::file::{Directory, File, FileAttribute, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::{CStr16, Result};
use uefi_services::println;


use core::mem;
use core::ptr;
use uefi::prelude::*;
use uefi::proto::rng::Rng;
use uefi::table::boot::{
    AllocateType, BootServices, MemoryMap, MemoryType, OpenProtocolAttributes, ScopedProtocol,
};
use uefi::ResultExt;

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

struct Buffer {
    width: usize,
    height: usize,
    pixels: Vec<BltPixel>,
}

impl Buffer {
    /// Create a new `Buffer`.
    fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            pixels: vec![BltPixel::new(0, 0, 0); width * height],
        }
    }

    /// Get a single pixel.
    fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        self.pixels.get_mut(y * self.width + x)
    }

    /// Blit the buffer to the framebuffer.
    fn blit(&self, gop: &mut GraphicsOutput) -> Result {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }
}

const PSF1_MAGIC0: u8 = 0x36;
const PSF1_MAGIC1: u8 = 0x04;

struct PSF1_HEADER {
    magic: [u8; 2],
    mode: u8,
    charsize: u8,
}

struct PSF1_FONT {
    psf1_Header: *mut PSF1_HEADER,
    glyphBuffer: *mut core::ffi::c_void,
}

struct BootInfo<'a> {
    framebuffer: FrameBuffer<'a>,
    psf1_Font: PSF1_FONT,
}

pub fn load_file(
    path: &CStr16,
    table: &SystemTable<Boot>,
    dir: Option<Directory>,
) -> Option<RegularFile> {
    let fs = table
        .boot_services()
        .get_handle_for_protocol::<SimpleFileSystem>()
        .ok()?;
    let mut fs = table
        .boot_services()
        .open_protocol_exclusive::<SimpleFileSystem>(fs)
        .ok()?;

    let mut dir = match dir {
        Some(a) => Some(a),
        None => fs.open_volume().ok(),
    }?;

    let f = dir
        .open(
            path,
            uefi::proto::media::file::FileMode::Read,
            FileAttribute::READ_ONLY,
        )
        .ok()?;

    f.into_regular_file()
}

fn initialize_gop(system_table: &SystemTable<Boot>) -> Option<ScopedProtocol<'_, GraphicsOutput>> {
    let handle = system_table
        .boot_services()
        .get_handle_for_protocol::<GraphicsOutput>()
        .ok().unwrap();
    println!("before a");
    let gop = system_table
        .boot_services()
        .open_protocol_exclusive::<GraphicsOutput>(handle)
        .ok().unwrap();
    println!("after a");

    Some(gop)
}

fn load_psf1_font(system_table: &SystemTable<Boot>) -> Option<PSF1_FONT> {
    let mut font = load_file(cstr16!("font"), system_table, None)?; // TODO: change path

    let size: usize = core::mem::size_of::<PSF1_HEADER>();
    let font_header_data = unsafe {
        core::slice::from_raw_parts_mut(
            system_table
                .boot_services()
                .allocate_pool(MemoryType::LOADER_DATA, size)
                .ok()?,
            size,
        )
    };

    font.read(font_header_data).unwrap();

    let font_header: *mut PSF1_HEADER = font_header_data.as_mut_ptr() as *mut PSF1_HEADER;

    unsafe {
        if (*font_header).magic[0] != PSF1_MAGIC0 || (*font_header).magic[1] != PSF1_MAGIC1 {
            return None;
        }
    }
    let mut glyph_buffer_size = unsafe { (*font_header).charsize as usize * 256 };
    unsafe {
        if (*font_header).mode == 1 {
            glyph_buffer_size = (*font_header).charsize as usize * 512;
        }
    }

    let glyph_buffer = unsafe {
        core::slice::from_raw_parts_mut(
            system_table
                .boot_services()
                .allocate_pool(MemoryType::LOADER_DATA, glyph_buffer_size)
                .ok()?,
            glyph_buffer_size,
        )
    };
    font.set_position(core::mem::size_of::<PSF1_HEADER>() as u64)
        .unwrap();
    font.read(glyph_buffer).unwrap();

    let finished_font = PSF1_FONT {
        glyphBuffer: glyph_buffer.as_mut_ptr() as _,
        psf1_Header: font_header,
    };

    Some(finished_font)
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    println!("before init gop");
    let mut newBuffer = initialize_gop(&system_table).unwrap();
    println!("after init gop");
    let font = load_psf1_font(&system_table).unwrap();
    let _boot_info = BootInfo {
        framebuffer: newBuffer.frame_buffer(),
        psf1_Font: font,
    };

    // TOOD: call no_kernel_main and pass the boot_info
    // and pray for it to work _/\_

    Status::SUCCESS
}
