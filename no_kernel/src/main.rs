#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::BootInfo;


#[no_mangle]
fn no_kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    boot_info.framebuffer.as_mut().unwrap().buffer_mut().iter_mut().map(|a|*a = 0xFF).count();
    panic!()

}
bootloader_api::entry_point!(no_kernel_main);

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
