#![no_std]
#![no_main]

extern crate alloc;

use core::{fmt::Write, time::Duration};

use uefi::{
    cstr16, entry,
    proto::{
        console::gop::{GraphicsOutput, BltOp, BltPixel},
        media::{
            file::{Directory, File, FileAttribute, RegularFile},
            fs::SimpleFileSystem,
        },
        ProtocolPointer,
    },
    table::{boot::MemoryType, Boot, SystemTable},
    CStr16, Handle, Status,
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
        "3...2...1...",
        Duration::from_secs_f32(0.05),
    );

    system_table.stdout().clear().unwrap();

    let gop_scoped = unsafe {
        {
            let handle = system_table
                .boot_services()
                .get_handle_for_protocol::<GraphicsOutput>()
                .unwrap();
            // system_table.boot_services()

            system_table
                .boot_services()
                .open_protocol::<GraphicsOutput>(
                    uefi::table::boot::OpenProtocolParams {
                        handle,
                        agent: image_handle,
                        controller: None,
                    },
                    uefi::table::boot::OpenProtocolAttributes::GetProtocol,
                )
                .unwrap()

        }
    };
    let gop: *mut _  = gop_scoped.get_mut().unwrap();
    core::mem::forget(gop_scoped); 

    // let frame_buffer = unsafe { gop.read() }.frame_buffer().as_mut_ptr();

    let f: extern "C" fn(gop: *mut GraphicsOutput) -> i32 =
        unsafe { core::mem::transmute(elf.entry) };
    system_table.stdout().write_str("exiting!").unwrap();

    // (unsafe{&mut *gop}).blt(BltOp::VideoFill {
    //     color: BltPixel::new(0x69, 0x69, 0x69),
    //     dest: (0, 0),
    //     dims: unsafe{&mut *gop}.current_mode_info().resolution(),
    // })
    // .unwrap();


    let (_runtime, _map) = system_table.exit_boot_services();

    let i = f(gop);

    Status::ABORTED
}
