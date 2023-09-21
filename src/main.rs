use std::{env, path::Path, process::Command};

fn main() {
    let mut args = env::args();
    args.next();
    let t = args.next();

    match t.as_deref() {
        Some("uefi") => {
            println!("running UEFI!");
            let boot = bootloader::UefiBoot::new(Path::new("./kernel_build/kernel.no"));

            boot.create_disk_image(Path::new("./kernel_build/kernel_uefi.img"))
                .unwrap();

            Command::new("qemu-system-x86_64")
                .arg("-drive")
                .arg("format=raw,file=./kernel_build/kernel_uefi.img")
                .args(["-drive", "if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd"])
                .args(["-drive", "if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd"])
                .status()
                .expect("Failed to start qemu-system-x86_64");
        }
        _ => {
            println!("running BIOS!");
            let boot = bootloader::BiosBoot::new(Path::new("./kernel_build/kernel.no"));

            boot.create_disk_image(Path::new("./kernel_build/kernel_bios.img"))
                .unwrap();

            Command::new("qemu-system-x86_64")
                .arg("-drive")
                .arg("format=raw,file=./kernel_build/kernel_bios.img")
                .status()
                .expect("Failed to start qemu-system-x86_64");
        }
    }
}
