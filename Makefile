CARGO := cargo
TARGET_SPEC := target-specs/x86_64-sel4-roottask-minimal.json
BUILD_ARGS := -Zjson-target-spec --target $(TARGET_SPEC)

build_profile ?= debug
chord_path = target/x86_64-sel4-roottask-minimal/$(build_profile)/chord.elf

.DEFAULT_GOAL := help

.PHONY: debug
debug:
	$(CARGO) build $(BUILD_ARGS)

.PHONY: release
release:
	$(CARGO) build $(BUILD_ARGS) --release

.PHONY: check
check:
	$(CARGO) check $(BUILD_ARGS)

.PHONY: clippy
clippy:
	$(CARGO) clippy $(BUILD_ARGS) -- -D warnings

.PHONY: fmt
fmt:
	$(CARGO) fmt

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: _iso
_iso: stubs/grub.cfg
	$(eval TMP_DIR := $(shell mktemp -d))
	@mkdir -p $(TMP_DIR)/boot/grub
	@cp sel4/bin/kernel.elf $(TMP_DIR)/boot/kernel.elf
	@cp $(chord_path) $(TMP_DIR)/boot/chord.elf
	@cp stubs/grub.cfg $(TMP_DIR)/boot/grub/grub.cfg
	grub2-mkrescue -o target/chord.iso $(TMP_DIR)
	@rm -rf $(TMP_DIR)

.PHONY: _qemu
_qemu:
	@qemu-system-x86_64 \
		-machine q35,accel=kvm -cpu host -smp 2 -m 2G \
		-serial mon:stdio \
		-nographic \
		-no-reboot \
		-d guest_errors \
		-cdrom target/chord.iso \
		-boot d

.PHONY: image-debug
image-debug: build_profile=debug
image-debug: debug _iso

.PHONY: image-release
image-release: build_profile=release
image-release: release _iso

.PHONY: run-debug
run-debug: build_profile=debug
run-debug: debug _iso _qemu

.PHONY: run-release
run-release: build_profile=release
run-release: release _iso _qemu

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make debug          - Build the seL4 roottask (debug)"
	@echo "  make release        - Build the seL4 roottask (release, optimized)"
	@echo "  make check          - Compile-check only (fast, no binary)"
	@echo "  make clippy         - Run the clippy linter on the codebase"
	@echo "  make fmt            - Format the Rust source code"
	@echo "  make clean          - Remove the target/ build directory"
	@echo "  make image-debug    - Build debug chord + ISO"
	@echo "  make image-release  - Build release chord + ISO"
	@echo "  make run-debug      - Build debug chord + ISO + boot in QEMU"
	@echo "  make run-release    - Build release chord + ISO + boot in QEMU"
	@echo "  make help           - Show this help"
