use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    nasm_rs::Build::new()
        .file("./bootstrap.asm")
        .out_dir(&out_dir)
        .compile("bootstrap.o")
        .unwrap();

        println!("cargo:rustc-link-arg=-Tlinker.ld");
        println!("cargo:rustc-link-arg=-l {}{}",&out_dir,"booatstrap.o");

    // nasm_rs::compile_library_args(&output, &[], &[]).unwrap();
}
