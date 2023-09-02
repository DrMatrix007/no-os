#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;

use uefi::{
    cstr16, entry,
    proto::media::{
        file::{Directory, File, FileAttribute, RegularFile},
        fs::SimpleFileSystem,
    },
    table::{
        boot::{AllocateType, MemoryType},
        Boot, SystemTable,
    },
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
            println!("pages: {}", pages);
            println!("flsz: {}",i.p_filesz);
            println!("ppaddr: {}",i.p_paddr);
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
    
    println!("size: {}",data.len());
    println!("entry: {}", elf.header.e_entry);
    // println!("{}", i);
    // let bootstrap = system_table
    //     .boot_services()
    //     .load_image(
    //         image_handle,
    //         uefi::table::boot::LoadImageSource::FromBuffer {
    //             buffer: &data,
    //             file_path: None,
    //         },
    //     )
    //     .expect("cant load bootstrap!");
    unsafe {
        system_table
            .boot_services()
            .memmove((i) as _, data.as_ptr(), data.len());
    }

    let f: extern "C" fn() -> i32 = unsafe { core::mem::transmute(elf.entry+10000) };

    // let _ = system_table.exit_boot_services();

    let i = f();
    println!("{}",i as u8 as char);
    // system_table.boot_services().start_image(bootstrap).unwrap();
    loop{}
    Status::SUCCESS
}

//oldshit:
/*    let mut data = vec![0; size];

kernel.read(data.as_mut()).unwrap();
// println!("{}", data.len());
let elf = goblin::elf::Elf::parse(&data).unwrap();
let entry = elf.entry;

println!("{}", (data.len() + 0x0999) / 0x1000);
println!("{}",elf.program_headers.len());
for phdr in elf.program_headers.iter().filter(|a| a.p_type == 1) {
    println!("{:?}",phdr);
    let data_ptr = system_table
        .boot_services()
        .allocate_pages(
            uefi::table::boot::AllocateType::AnyPages,
            MemoryType::LOADER_DATA,
            (data.len() + 0x0999) / 0x1000,
        )
        .unwrap();

    kernel.set_position(phdr.p_offset).unwrap();
    kernel.read(&mut data).unwrap();
    unsafe {
        system_table
            .boot_services()
            .memmove(data_ptr as _, data.as_ptr(), phdr.p_filesz as _)
    }
}

let map_size = system_table.boot_services().memory_map_size();
let mut mem_map_vec = vec![0; (map_size.entry_size + map_size.map_size)];
let mem_map = system_table
    .boot_services()
    .memory_map(&mut mem_map_vec)
    .unwrap();
// println!("{:?}",mem_map);
// write here
let (a, mut mem_map) = system_table.exit_boot_services();

// // write here
mem_map.sort();

// // let i = (addr)();
// // println!("ans is {}", i);
// // let _ = system_table.exit_boot_services();
for i in mem_map.entries() {
    match i.ty {
        MemoryType::CONVENTIONAL
        | MemoryType::LOADER_CODE
        | MemoryType::LOADER_DATA
        | MemoryType::BOOT_SERVICES_CODE
        | MemoryType::BOOT_SERVICES_DATA => {
            let addr: fn() -> i32 = unsafe { core::mem::transmute(entry) };
            addr();
        }
        _ => {}
    }
} */
