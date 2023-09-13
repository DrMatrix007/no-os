#![no_std]
#![no_main]

extern crate alloc;

use core::{fmt::Write, time::Duration};

use alloc::vec;
use no_kernel_args::{BootInfo, FrameData, PsfFont, PsfHeader};
use uefi::{
    cstr16, entry,
    proto::{
        console::gop::GraphicsOutput,
        media::{
            file::{Directory, File, FileAttribute, FileInfo, RegularFile},
            fs::SimpleFileSystem,
        },
    },
    table::{boot::MemoryType, Boot, SystemTable},
    CStr16, Handle, Identify, Status,
};
use uefi_services::println;

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

fn create_frame_buffer(system_table: &mut SystemTable<Boot>) -> FrameData {
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
                    agent: system_table.boot_services().image_handle(),
                    controller: None,
                })
                .unwrap();
            system_table
                .boot_services()
                .open_protocol::<GraphicsOutput>(
                    uefi::table::boot::OpenProtocolParams {
                        handle,
                        agent: system_table.boot_services().image_handle(),
                        controller: None,
                    },
                    uefi::table::boot::OpenProtocolAttributes::Exclusive,
                )
                .unwrap()
        }
    };
    let mut gop = gop_scoped;

    let (width, height) = gop.current_mode_info().resolution();

    let frame = FrameData {
        ptr: gop.frame_buffer().as_mut_ptr() as _,
        width,
        height,
        size: gop.frame_buffer().size(),
        size_per_pixel: gop.frame_buffer().size() / (width * height),
        pixels_per_scan_line: gop.current_mode_info().stride(),
    };
    // unsafe {
    //     system_table
    //         .boot_services()
    //         .set_mem(frame.ptr, gop.frame_buffer().size(), 0);
    // }

    frame
}

fn get_font(system_table: &mut SystemTable<Boot>) -> PsfFont {
    let mut font = load_file(cstr16!("zap-light16.psf"), system_table, None).unwrap();

    let mut font_header = PsfHeader {
        magic: [0, 0],
        mode: 0,
        charsize: 0,
    };
    let _ = system_table
        .boot_services()
        .allocate_pool(MemoryType::LOADER_DATA, 4);

    let mut small_buffer = vec![0u8; 0];
    let size = font
        .get_info::<FileInfo>(&mut small_buffer)
        .err()
        .unwrap()
        .data()
        .unwrap();
    let mut file_info = vec![0u8; size];
    font.get_info::<FileInfo>(&mut file_info).unwrap();
    font_header.magic[0] = file_info[0];
    font_header.magic[1] = file_info[1];
    font_header.mode = file_info[2];
    font_header.charsize = file_info[3];

    let mut buffer: usize = (font_header.charsize as usize) * 256;
    PsfFont {
        header: font_header,
        buffer: &mut buffer,
    }
}

fn get_entry(system_table: &mut SystemTable<Boot>) -> fn(*mut BootInfo) -> i32 {
    let mut kernel = load_file(cstr16!("no_kernel.elf"), system_table, None).unwrap();
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
    let entry = elf.entry;
    drop(elf);

    system_table
        .boot_services()
        .free_pool(data.as_mut_ptr())
        .unwrap();

    unsafe { core::mem::transmute(entry) }
}

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    system_table.stdout().clear().unwrap();

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

    // println!("{}",gop.modes().any(|f|f.info().pixel_format()==PixelFormat::Rgb));
    // for index in 0..20 {
    // println!("{:?}",&gop.current_mode_info());
    // println!("{}. {:?} {:?}",index,gop.query_mode(index).unwrap().info().resolution(),gop.query_mode(index).unwrap().info().pixel_format());
    // }
    // println!("{:?}", frame.ptr);
    let mut frame = create_frame_buffer(&mut system_table);

    let font = get_font(&mut system_table);

    let f = get_entry(&mut system_table);

    // for x in 0..frame.width {
    //     for y in 0..frame.height {
    //         unsafe {
    //             *frame.get_pixel(x, y) = 0;
    //             // print!("{},",*)
    //         }
    //     }
    // }
    println!("{:?}", frame);
    // unsafe {
    //     system_table
    //         .boot_services()
    //         .set_mem(frame.ptr as _, frame.size, 0);
    // }
    // println!("{}",(0..frame.size).all(|x|unsafe{(frame.ptr.add(x) as *const u8).read()}==0));
    // let (_runtime, _mazp) = system_table.exit_boot_services();

    let mut boot_info = BootInfo {
        framebuffer: &mut frame,
        font,
        map_desc_size: 0,
        map_size: 0,
    };

    let _i = f(&mut boot_info);

    Status::SUCCESS
}
