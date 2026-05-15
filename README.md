# CHORD

A proof-of-concept for seL4 userland development in Rust.

## Prerequisites

- **Rust nightly-2026-04-04** (pinned via `rust-toolchain.toml`) with the
  `x86_64-unknown-none` target
- **QEMU** (for `make run-debug` / `make run-release`)
- **grub2-mkrescue** (for ISO generation)

## Build

```sh
make debug         # debug build
make release       # optimized release build
make check         # compile-check only (fast, no binary)
```

## Run

```sh
make run-debug     # build debug chord + ISO + boot under QEMU
make run-release   # build release chord + ISO + boot under QEMU
```

Expected output:

```
Hello, World!
```

## Other targets

| Command              | Description                           |
| -------------------- | ------------------------------------- |
| `make image-debug`   | Build debug chord + ISO               |
| `make image-release` | Build release chord + ISO             |
| `make clippy`        | Run Clippy with `-D warnings`         |
| `make fmt`           | Format Rust source code               |
| `make clean`         | Remove `target/` build directory      |
| `make help`          | Show available targets (also default) |

## Project structure

```
.
├── .cargo/config.toml          # Rust build settings (target specs, env)
├── Cargo.toml                  # Workspace root (rust-sel4 v4.0.0)
├── Makefile                    # Build, ISO, and QEMU targets
├── rust-toolchain.toml         # Pinned nightly toolchain
├── crates/chord/src/main.rs    # Minimal seL4 root task
├── sel4/                       # Pre-built seL4 kernel + C headers
├── stubs/grub.cfg              # GRUB2 boot config
└── target-specs/               # Custom Rust target for bare-metal x86_64
```

## How it works

The root task uses the [`rust-sel4`](https://github.com/seL4/rust-sel4) crates:

- `sel4` — Rust bindings to the seL4 kernel API
- `sel4-root-task` — `#[root_task]` attribute macro that sets up the entry
  point and boot stack for a bare-metal seL4 root task

The kernel (`sel4/bin/kernel.elf`) is a pre-built seL4 binary. At boot, GRUB
chainloads the kernel via multiboot2 and passes the root task ELF as a module.
The kernel starts the root task, which prints to the kernel's debug console and
suspends.

## License

MIT
