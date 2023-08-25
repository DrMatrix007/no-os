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
extern crate no_bootloader;
extern crate no_kernel;

use core::panic::PanicInfo;

use alloc::vec;
use alloc::vec::Vec;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::Result;

#[panic_handler]
fn panic_handler(data: &PanicInfo) -> ! {
    uefi_services::println!("fuck {}", data);
    loop {}
}

use core::mem;
use core::ptr;
use uefi::prelude::*;
use uefi::proto::rng::Rng;
use uefi::table::boot::{BootServices, MemoryMap, AllocateType};
use uefi::prelude::*;
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

struct Framebuffer{
	BaseAddress: *mut core::ffi::c_void,
	BufferSize: u64,
	Width: u32,
	Height: u32,
	PixelsPerScanLine: u32,
}

const PSF1_MAGIC0: u8 = 0x36;
const PSF1_MAGIC1: u8 = 0x04;

struct PSF1_HEADER{
	magic: [u8; 2],
	mode: u8,
	charsize: u8,
}

struct PSF1_FONT{
	psf1_Header: *mut PSF1_HEADER,
	glyphBuffer: *mut core::ffi::c_void,
}

struct BootInfo<'a> {
    framebuffer: *mut Framebuffer,
    psf1_Font: *mut PSF1_FONT,
    mMap: *mut MemoryMap<'a>,
    mMapSize: usize,
    mMapDescSize: usize,
}

fn initialize_gop(system_table: &SystemTable<Boot>) -> Option<Framebuffer> {
    //let gop_guid = ; // i have no idea how to get the gop's guid

    let gop: *mut GraphicsOutput = system_table
        .boot_services()
        .locate_protocol::<GraphicsOutput>(&gop_guid, ptr::null_mut())
        .expect_success("Failed to locate GOP")
        .get();

    let mode_info = unsafe { (*gop).current_mode_info() };
    let framebuffer = Framebuffer {
        BaseAddress: mode_info.frame_buffer_base as usize,
        BufferSize: mode_info.frame_buffer_size as u64,
        Width: mode_info.horizontal_resolution as u32,
        Height: mode_info.vertical_resolution as u32,
        PixelsPerScanLine: mode_info.pixels_per_scan_line as u32,
    };

    Some(framebuffer)
}

fn load_psf1_font(
    system_table: &SystemTable<Boot>,
) -> Option<*mut PSF1_FONT> {
    let font = no_bootloader::load_file(cstr16!("font"), system_table, None); // TODO: change path
    
    let mut font_header: *mut PSF1_HEADER = system_table
        .boot_services()
        .allocate_pool(AllocateType::LoaderData, core::mem::size_of::<PSF1_HEADER>())?
        .cast::<PSF1_HEADER>();

    let mut size: usize = core::mem::size_of::<PSF1_HEADER>();
    font.read(&mut size, font_header);
    unsafe {
        if (*font_header).magic[0] != PSF1_MAGIC0 || (*font_header).magic[1] != PSF1_MAGIC1 {
            return None;
        }
    }
    let mut glyph_buffer_size = unsafe {(*font_header).charsize * 256 };
    unsafe {
        if (*font_header).mode == 1 {
            glyph_buffer_size = (*font_header).charsize * 512;
        }
    }

    let mut glyph_buffer = system_table
        .boot_services()
        .allocate_pool(AllocateType::LoaderData, glyph_buffer_size)?
        .cast::<usize>();

    font.set_position(core::mem::size_of::<PSF1_HEADER>() as u64);
    font.read(&mut glyph_buffer_size, glyph_buffer);

    let mut finished_font: *mut PSF1_FONT = system_table
        .boot_services()
        .allocate_pool(AllocateType::LoaderData, core::mem::size_of::<PSF1_FONT>())?
        .cast::<PSF1_FONT>();

    unsafe {
        (*finished_font).psf1_Header = font_header;
        (*finished_font).glyphBuffer = glyph_buffer;
    }

    Some(finished_font)
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let bt = system_table.boot_services();
    let mut newBuffer = initialize_gop().unwrap();
    let mut font = load_psf1_font(&system_table).unwrap();
    let mut boot_info: BootInfo = BootInfo { 
        framebuffer: newBuffer,
        psf1_Font: font, mMap: (), // not so sure how to get the memory map
        mMapSize: (),
        mMapDescSize: () 
    };

    // TOOD: call no_kernel_main and pass the boot_info
    // and pray for it to work _/\_

    Status::SUCCESS
}
