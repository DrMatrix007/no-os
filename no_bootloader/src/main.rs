#![no_std]
#![no_main]

extern crate alloc;

use core::{fmt::Write, time::Duration};

use no_kernel_args::{FrameData,PSF1_FONT, PSF1_HEADER};
use uefi::{
    cstr16, entry,
    proto::{
        console::gop::{GraphicsOutput, PixelFormat},
        media::{
            file::{Directory, File, FileAttribute, RegularFile},
            fs::SimpleFileSystem,
        },
    },
    table::{boot::MemoryType, Boot, SystemTable},
    CStr16, Handle, Identify, Status,
};
use uefi_services::println;


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


fn load_file(
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

fn prretty_print(
    system_table: &mut SystemTable<Boot>,
    data: impl AsRef<str>,
    char_time_spereator: Duration,
) {
    for i in data.as_ref().chars() {
        system_table.stdout().write_char(i).unwrap();
        system_table
            .boot_services()
            .stall(char_time_spereator.as_micros() as _)
    }
}

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    system_table.stdout().clear().unwrap();

    let mut kernel = load_file(cstr16!("no_kernel.elf"), &system_table, None).unwrap();
    kernel.set_position(0xFFFFFFFFFFFFFFFF).unwrap();
    let size = kernel.get_position().unwrap() as usize;
    kernel.set_position(0).unwrap();

    let data = unsafe {
        core::slice::from_raw_parts_mut(
            system_table
                .boot_services()
                .allocate_pool(MemoryType::LOADER_DATA, size)
                .unwrap(),
            size,
        )
    };

    kernel.read(data).unwrap();

    let elf = goblin::elf::Elf::parse(data).unwrap();

    let i = elf
        .program_headers
        .iter()
        .find(|a| a.p_type == 1)
        .map(|i| {
            let pages = i.p_memsz as usize + 0x1000 - 1;
            let pages = pages / 0x1000;
            system_table
                .boot_services()
                .allocate_pages(
                    uefi::table::boot::AllocateType::Address(i.p_paddr),
                    MemoryType::LOADER_DATA,
                    pages,
                )
                .unwrap()
        })
        .unwrap();

    unsafe {
        system_table
            .boot_services()
            .memmove((i) as _, data.as_ptr(), data.len());
    }

    prretty_print(
        &mut system_table,
        "starting in:\n",
        Duration::from_secs_f32(0.05),
    );
    prretty_print(
        &mut system_table,
        "3...2...1...\n",
        Duration::from_secs_f32(0.05),
    );

    let gop_scoped = unsafe {
        {
            let handle = *system_table
                .boot_services()
                .locate_handle_buffer(uefi::table::boot::SearchType::ByProtocol(
                    &GraphicsOutput::GUID,
                ))
                .unwrap()
                .last()
                .unwrap();
            // system_table
            //     .boot_services()
            //     .open_protocol_exclusive::<GraphicsOutput>(handle)
            //     .unwrap()
            system_table
                .boot_services()
                .test_protocol::<GraphicsOutput>(uefi::table::boot::OpenProtocolParams {
                    handle,
                    agent: image_handle,
                    controller: None,
                })
                .unwrap();
            system_table
                .boot_services()
                .open_protocol::<GraphicsOutput>(
                    uefi::table::boot::OpenProtocolParams {
                        handle,
                        agent: image_handle,
                        controller: None,
                    },
                    uefi::table::boot::OpenProtocolAttributes::Exclusive,
                )
                .unwrap()
        }
    };
    let mut gop = gop_scoped;

    let (width, height) = gop.current_mode_info().resolution();

    let mut frame = FrameData {
        ptr: gop.frame_buffer().as_mut_ptr(),
        width,
        height,
        size_per_pixel: gop.frame_buffer().size() / (width * height),
        pixels_per_scan_line: gop.current_mode_info().stride(),
    };

    
    // println!("{}",gop.modes().any(|f|f.info().pixel_format()==PixelFormat::Rgb));
    println!("starting!");
    // for index in 0..20 {
    // println!("{:?}",&gop.current_mode_info());
    // println!("{}. {:?} {:?}",index,gop.query_mode(index).unwrap().info().resolution(),gop.query_mode(index).unwrap().info().pixel_format());
    // }
    // println!("{:?}", frame.ptr);
    drop(gop);

    let f: extern "C" fn(bootInfo: *mut BootInfo) -> i32 = unsafe { core::mem::transmute(elf.entry) };

    let (_runtime, _map) = system_table.exit_boot_services();
    
    for x in 0..frame.width { 
        for y in 0..frame.height {
            unsafe {
                core::ptr::write_volatile(frame.get_pixel(x, y), 0);
            }
        }
    }
    let mut font = load_file(cstr16!("some file"), &system_table, None).unwrap();

    font.set_position(0xFFFFFFFFFFFFFFFF).unwrap();
    let size = kernel.get_position().unwrap() as usize;
    font.set_position(0).unwrap();
    let data = unsafe {
        core::slice::from_raw_parts_mut(
            system_table
                .boot_services()
                .allocate_pool(MemoryType::LOADER_DATA, size)
                .unwrap(),
            size,
        )
    };

    font.read(data).unwrap();


    let psf1_f: PSF1_FONT = PSF1_FONT { psf1_Header: (), glyphBuffer: None };
    let bootInfo = BootInfo{framebuffer: &mut frame, psf1_Font: psf1_f, mMapDescSize: 0, mMapSize: 0};

    let i = f(&mut bootInfo);

    Status::SUCCESS
}
