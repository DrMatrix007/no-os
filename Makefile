release:
	cd no_bootloader; \
		cargo build --release;
	cd no_kernel; \
		cargo build --release;
	cp no_bootloader/target/x86_64-unknown-uefi/release/no_bootloader.efi esp/efi/boot/bootx64.efi
	cp no_kernel/target/x86_64-unknown-none/release/no_kernel esp/no_kernel.no
	qemu-system-x86_64 \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp