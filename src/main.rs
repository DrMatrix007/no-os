#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi::{
    cstr16, entry,
    proto::{
        loaded_image::{self, LoadedImage},
        media::{
            file::{Directory, File, FileAttribute, RegularFile},
            fs::SimpleFileSystem,
        },
    },
    table::{Boot, SystemTable},
    CStr16, Error, Handle, Status,
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
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    system_table.stdout().clear().unwrap();

    let kernel = load_file(cstr16!("kernel.no"), &system_table, None).unwrap();

    

    println!("Hello world!");
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}
