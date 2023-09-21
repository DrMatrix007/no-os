use std::process::Command;

fn main() {
    Command::new("echo")
        .arg(std::env::var_os("CARGO_BIN_FILE_NO_KERNEL_no_kernel").unwrap())
        .spawn()
        .unwrap();

    std::fs::copy(std::env::var_os("CARGO_BIN_FILE_NO_KERNEL_no_kernel").unwrap(), "./kernel_build/kernel.no").unwrap();
}
