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
	
debug:
	cd no_bootloader; \
		cargo build;
	cd no_kernel; \
		cargo build;

	cp no_bootloader/target/x86_64-unknown-uefi/debug/no_bootloader.efi esp/efi/boot/bootx64.efi
	
	ld \
	 -e "no_kernel_main" \
	 no_kernel/target/x86_64-unknown-none/debug/libno_kernel.a \
	 -o esp/no_kernel.elf \
	 -melf_x86_64
	
	

run: 
	qemu-system-x86_64 -enable-kvm -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd -drive format=raw,file=fat:rw:esp
	# qemu-system-x86_64 --enable-kvm -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE-pure-efi.fd -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS-pure-efi.fd -drive format=raw,file=fat:rw:esp
	# qemu-system-x86_64 -enable-kvm --bios OVMF.fd -drive format=raw,file=fat:rw:esp
clean:
	cd no_bootloader; \
		cargo clean
	cd no_kernel; \
		cargo clean
	rm -rf esp/efi/boot/bootx64.efi
	rm -rf esp/no_krenel.elf
