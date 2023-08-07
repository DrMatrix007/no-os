release:
	cargo build --release --target x86_64-unknown-uefi
	mkdir -p esp/efi/boot
	cp target/x86_64-unknown-uefi/release/no_bootloader.efi esp/efi/boot/bootx64.efi
	qemu-system-x86_64 \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp
