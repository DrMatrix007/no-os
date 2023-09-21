release run:
	cargo bootimage --release
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/no-os

