#![no_std]
#![no_main]

extern crate alloc;

use core::arch::asm;

use alloc::{string::ToString, vec::Vec};
use uefi::{
    cstr16, entry,
    proto::media::{
        file::{Directory, File, FileAttribute, RegularFile},
        fs::SimpleFileSystem,
    },
    table::{Boot, SystemTable},
    CStr16, Handle, Status,
};
use uefi_services::{print, println};

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

    println!("hello world!");

    let mut kernel = load_file(cstr16!("no_kernel.elf"), &system_table, None).unwrap();
    kernel.set_position(0xFFFFFFFFFFFFFFFF).unwrap();
    let size = kernel.get_position().unwrap() as usize;
    kernel.set_position(0).unwrap();

    let mut data = Vec::with_capacity(size);
    data.resize(size, 0);

    kernel.read(data.as_mut()).unwrap();
    // println!("{}", data.len());
    let elf = goblin::elf::Elf::parse(&data).unwrap();
    let entry = elf.entry;

    // let off = elf.program_headers.first().unwrap().p_offset as usize;
    // let ph = elf.program_headers.first().unwrap();
    // for i in &data {
    //     print!("{:x} ",i)
    // }
    // println!(
    //     "entry is {}, {:?}",
    //     elf.entry, &elf as *const goblin::elf::Elf as usize
    // );

    // let f: extern "C" fn() -> i32 = unsafe { core::mem::transmute(data.as_ptr().add(ph.p_vaddr as _) as *const ()) };
    unsafe {
        system_table
            .boot_services()
            .set_mem(data.as_mut_ptr(), data.len(), 0);
    }
    let addr: extern "C" fn() -> i32 = unsafe { core::mem::transmute(entry as *const ()) };
    // unsafe { asm!("call 32") };
    let i = (addr)();

    // println!("ans is {}", i);
    // let _ = system_table.exit_boot_services();
    Status::SUCCESS
}
