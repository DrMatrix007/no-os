release:
	cd no_bootloader; \
		cargo build --release;
	cd no_kernel; \
		cargo build --release;

	cp no_bootloader/target/x86_64-unknown-uefi/release/no_bootloader.efi esp/efi/boot/bootx64.efi
	# cp no_bootstrap/target/x86_64-unknown-uefi/release/no_bootstrap.efi esp/no_bootstrap.efi
	
	#  -T no_kernel/linker.ld \
	
	ld \
	 -e no_kernel_main \
	 no_kernel/target/x86_64-unknown-none/release/libno_kernel.rlib \
	 -o esp/no_kernel.elf \
	 -melf_x86_64
	
	# cp no_kernel/target/x86_64-unknown-none/release/no_kernel esp/no_kernel.elf
	

run: 
	qemu-system-x86_64 \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp
clean:
	cd no_bootloader; \
		rm -rf target
	cd no_kernel; \
		rm -rf target
