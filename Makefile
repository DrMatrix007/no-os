release:
	cd no_bootloader; \
		cargo build --release;
	cd no_kernel; \
		cargo build --release;

	cp no_bootloader/target/x86_64-unknown-uefi/release/no_bootloader.efi esp/efi/boot/bootx64.efi
	
	
	ld \
	 -e "no_kernel_main" \
	 no_kernel/target/x86_64-unknown-none/release/libno_kernel.a \
	 -o esp/no_kernel.elf \
	 -melf_x86_64
	
	

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
