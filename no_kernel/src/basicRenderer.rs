#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use no_kernel_args::{FrameData, PsfFont};

pub struct Point {
    pub X: u32,
    pub Y: u32,
}

static mut CursorPosition: Point = Point { X: 0u32, Y: 0u32 };
static mut framebuffer: *mut FrameData = core::ptr::null_mut(); // framebuffer need not be mutable after BasicRenderer fn Initializes it
static mut psf1_font: *mut PsfFont = core::ptr::null_mut(); // psf1_font need not be mutable after BasicRenderer fn Initializes it
static mut colour: u32 = 0xFFFFFFu32;

pub fn BasicRenderer(FRAMEbuffer: *mut FrameData, PSF1_font: &mut PsfFont) {
    unsafe {
        framebuffer = FRAMEbuffer;
        psf1_font = PSF1_font;
    }
}

// BasicRenderer
pub fn PutPix(x: u32, y: u32, color: u32) {
    // PutPix Function pass framebuffer colour and screen cords
    unsafe {
        core::ptr::write_volatile(
            (*framebuffer)
                .ptr
                .add(((x * 4) + (y * ((*framebuffer).pixels_per_scan_line as u32) * 4)) as usize)
                as *mut u32,
            color,
        );
    }
}

pub fn Clear(clear_color: u32) {
    unsafe {
        let mut x = 0;
        let mut y = 0;
        for _i in 0..(*framebuffer).size_per_pixel {
            // i in framebuffer size so in 1080 x 1920 i will be incremented 2073600 times
            PutPix(x, y, clear_color); // calling PutPix passing framebuffer colour and screen cords
            if x < (*framebuffer).pixels_per_scan_line as u32 {
                // logic for passing on every pixel in screen
                x += 1;
            } else if y < (*framebuffer).height as u32 {
                y += 1;
                x = 0;
            }
        }
        x = 0;
        y = 0;
    }
}

fn PutChar(chr: char, mut xOff: u32, mut yOff: u32) {
    // PutChar prints a character to the screen using the psf v1 font glyphbuffer
    unsafe {
        let mut fontPtr = (*psf1_font).buffer as *mut u8; // cast pointer to glyphbuffer
        for y in yOff..yOff + 16 {
            // hight of character is 16
            if y < (*framebuffer).height as u32 {
                // check for cursor_position.Y (yOff) not going out of bounds and writing outside framebuffer into unspecified memory
                for x in xOff..xOff + 8 {
                    // width of character is 8
                    if x < (*framebuffer).width as u32 {
                        // check for cursor_position.X (xOff) not going out of bounds and writing outside framebuffer into unspecified memory
                        let glyphIndex = (chr as u32) * (*psf1_font).header.charsize as u32; // cast chr to u32 & dereference psf1_Header charsize and cast as u32
                        if *fontPtr.offset(glyphIndex.try_into().unwrap())
                            & (0b10000000 >> (x - xOff))
                            > 0
                        {
                            // bit shift andwise to iterate glyphbuffer for correct letter
                            if x < (*framebuffer).width as u32 && y < (*framebuffer).height as u32 {
                                // double check that cursor_position is not out of bounds
                                PutPix(x, y, colour);
                            }
                        }
                    }
                }
                fontPtr = fontPtr.offset(1); // add offset
            }
        }
    }
    xOff = 0; // clear values
    yOff = 0;
}

// should make it take str: impl Into<String>
pub fn Print(str: &str) {
    unsafe {
        // prints an &str by calling PutChar
        //let str = str.into();
        for c in str.chars() {
            // iterates characters of str pointer
            if CursorPosition.X < (*framebuffer).width as u32
                && CursorPosition.Y < (*framebuffer).height as u32
            {
                // checks for out of bounds
                PutChar(c, CursorPosition.X, CursorPosition.Y);
            }
            CursorPosition.X += 8;
            if CursorPosition.X + 8 > (*framebuffer).width as u32 {
                // checks for X out of bounds and wraps over
                CursorPosition.X = 0;
                CursorPosition.Y += 16;
            }
        }
    }
}

pub fn Next() {
    unsafe {
        // my version of newline
        CursorPosition.X = 0; // reset X
        if CursorPosition.Y + 16 < (*framebuffer).height as u32 {
            // checks for Y out of bounds and wraps over
            CursorPosition.Y += 16;
        } else {
            // scroll instead of wrapover when shell is implemented
        }
    }
}

pub fn Colour(color: u32) {
    unsafe {
        colour = color;
    }
}