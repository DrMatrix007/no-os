use std::{path::Path, process::Command};

use bootloader::BootConfig;

fn main() {
    let boot= bootloader::BiosBoot::new(Path::new("./kernel_build/kernel.no"));

    boot.create_disk_image(Path::new("./kernel_build/kernel.img")).unwrap();

 let status = Command::new("qemu-system-x86_64")
        .arg("-drive")
        .arg("format=raw,file=./kernel_build/kernel.img")
        .status()
        .expect("Failed to start qemu-system-x86_64");


    }
