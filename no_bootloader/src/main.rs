#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use uefi::{
    cstr16, entry,
    proto::media::{
        file::{Directory, File, FileAttribute, RegularFile},
        fs::SimpleFileSystem,
    },
    table::{Boot, SystemTable},
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

    return f.into_regular_file();
}

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    system_table.stdout().clear().unwrap();


    let mut kernel = load_file(cstr16!("no_kernel.elf"), &system_table, None).unwrap();
    kernel.set_position(0xFFFFFFFFFFFFFFFF).unwrap();
    let size = kernel.get_position().unwrap() as usize;
    kernel.set_position(0).unwrap();

    let mut data = Vec::with_capacity(size + 1);
    data.resize(size + 1 as usize, 0);

    kernel.read(data.as_mut()).unwrap();
    println!("{}",data.len());
    // let elf = Elf::parse(kernel.);
    let loaded = system_table
        .boot_services()
        .load_image(
            image_handle,
            uefi::table::boot::LoadImageSource::FromBuffer {
                buffer: &data,
                file_path: None,
            },
        )
        .unwrap();
    system_table.boot_services().start_image(loaded).unwrap();

    let _ = system_table.exit_boot_services();
    Status::SUCCESS
}
