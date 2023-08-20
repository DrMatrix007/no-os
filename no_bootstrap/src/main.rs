#![no_main]
#![no_std]

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
///
///
///
///
///
extern crate alloc;

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
use uefi::prelude::*;
use uefi::proto::rng::Rng;
use uefi::table::boot::BootServices;

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

/// Get a random `usize` value.
fn get_random_usize(rng: &mut Rng) -> usize {
    let mut buf = [0; mem::size_of::<usize>()];
    rng.get_rng(None, &mut buf).expect("get_rng failed");
    usize::from_le_bytes(buf)
}

fn draw_sierpinski(bt: &BootServices) -> Result {
    // Open graphics output protocol.
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = bt
        .open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .unwrap();

    // Open random number generator protocol.

    // Create a buffer to draw into.
    let (width, height) = gop.current_mode_info().resolution();
    let mut buffer = Buffer::new(width, height);

    // Initialize the buffer with a simple gradient background.
    for y in 0..height {
        let r = ((y as f32) / ((height - 1) as f32)) * 255.0;
        for x in 0..width {
            let g = ((x as f32) / ((width - 1) as f32)) * 255.0;
            let pixel = buffer.pixel(x, y).unwrap();
            pixel.red = r as u8;
            pixel.green = g as u8;
            pixel.blue = 255;
        }
    }

    let size = Point::new(width as f32, height as f32);

    // Define the vertices of a big triangle.
    let border = 20.0;
    let triangle = [
        Point::new(size.x / 2.0, border),
        Point::new(border, size.y - border),
        Point::new(size.x - border, size.y - border),
    ];

    // `p` is the point to draw. Start at the center of the triangle.
    let mut p = Point::new(size.x / 2.0, size.y / 2.0);
    let mut c = 0;
    // Loop forever, drawing the frame after each new point is changed.
    loop {
        // Choose one of the triangle's vertices at random.
        let v = triangle[c % 3];
        c += 1;
        // Move `p` halfway to the chosen vertex.
        p.x = (p.x + v.x) * 0.5;
        p.y = (p.y + v.y) * 0.5;

        // Set `p` to black.
        let pixel = buffer.pixel(p.x as usize, p.y as usize).unwrap();
        pixel.red = 0;
        pixel.green = 100;
        pixel.blue = 0;

        // Draw the buffer to the screen.
        buffer.blit(&mut gop).unwrap();
    }
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let bt = system_table.boot_services();
    draw_sierpinski(bt).unwrap();
    Status::SUCCESS
}
