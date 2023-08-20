#![no_main]
#![no_std]

extern crate no_kernel;

use no_kernel::no_kernel_main;
use uefi::{prelude::*, CStr16};
use uefi_services::println;



#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    println!("Hello world! lol bozo");
    let ans = unsafe { no_kernel_main() };

    println!("{}",ans);

    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}
