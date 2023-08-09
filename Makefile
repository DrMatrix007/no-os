clean:
	cd no_bootloader; \
		cargo clean;
	cd no_kernel; \
		cargo clean;
	

release:
	cd no_bootloader; \
		cargo build --release;
	cd no_kernel; \
		cargo build --release;
	
	nasm no_kernel/bootstrap.asm -felf64 -o no_kernel/target/x86_64-unknown-none/release/bootstrap.o
	
	ld -T no_kernel/linker.ld \
	 no_kernel/target/x86_64-unknown-none/release/bootstrap.o \
	 no_kernel/target/x86_64-unknown-none/release/libno_kernel.a \
	 -o esp/no_kernel.elf \
	 -melf_x86_64 -static -Bsymbolic -nostdlib -z noexecstack

	cp no_bootloader/target/x86_64-unknown-uefi/release/no_bootloader.efi esp/efi/boot/bootx64.efi
	##### cp no_kernel/target/x86_64-unknown-none/release/no_kernel esp/no_kernel.no
	

debug:
	cd no_bootloader; \
		cargo build;
	cd no_kernel; \
		cargo build;
	cp no_bootloader/target/x86_64-unknown-uefi/debug/no_bootloader.efi esp/efi/boot/bootx64.efi
	cp no_kernel/target/x86_64-unknown-none/debug/no_kernel esp/no_kernel.no
run: 
	qemu-system-x86_64 \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp