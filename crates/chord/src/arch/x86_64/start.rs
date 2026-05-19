//! `_start` assembly entry point for `x86_64`.
//!
//! # Boot context
//!
//! The seL4 kernel invokes the root task with:
//! - `rdi` = pointer to [`BootInfo`](sel4::BootInfo)
//!
//! # Boot sequence
//!
//! 1. **Stack setup** — loads `rsp` from the ABI-aligned `__stack_top`.
//! 2. **Call Rust** — invokes `__chord_entry` with `rdi` intact.
//!
//! # Stack layout
//!
//! The boot stack uses a local [`BootStack`] static. Startup assembly derives
//! the initial stack pointer from the exported stack symbol, so no custom
//! linker script is needed for stack setup.

use core::arch::global_asm;
use core::cell::UnsafeCell;

const STACK_SIZE: usize = 64 * 1024;

/// Boot stack storage — 64 KiB, 16-byte aligned per `x86_64` `SysV` ABI.
///
/// Wrapped in [`UnsafeCell`] because the memory is mutated exclusively
/// through the CPU's stack pointer (`rsp`), never via Rust references.
#[repr(C, align(16))]
struct BootStack(UnsafeCell<[u8; STACK_SIZE]>);

// SAFETY: Stack memory is only accessed through the CPU's stack pointer.
// Rust code never creates references to this memory.
unsafe impl Sync for BootStack {}

impl BootStack {
    #[allow(
        clippy::large_stack_arrays,
        reason = "const initializer for static storage, not an actual stack allocation"
    )]
    const fn new() -> Self {
        Self(UnsafeCell::new([0; STACK_SIZE]))
    }
}

#[used]
#[unsafe(no_mangle)]
static __chord_boot_stack: BootStack = BootStack::new();

// SAFETY: This assembly block is the root task's entry point, invoked by the
// seL4 kernel with `rdi` pointing to the `BootInfo` structure. It:
//
// 1. Clears the direction flag (`cld`) as required by the SysV ABI on entry.
// 2. Sets `rsp` to `__stack_top`, which is derived from the `BootStack`
//    static. `BootStack` guarantees 16-byte alignment, and `STACK_SIZE` is a
//    multiple of that alignment, so `rsp` is correctly aligned.
// 3. Preserves `rdi` (the BootInfo pointer) across the stack setup so it
//    becomes the first argument to `__chord_entry` per the SysV calling
//    convention.
// 4. Builds a call frame (`sub rsp, 8` + `push rbp`) so that `__chord_entry`
//    enters with `rsp % 16 == 8`, matching the x86-64 SysV ABI requirement
//    that `call` pushes an 8-byte return address before the callee's prologue.
// 5. Jumps to a halt loop if `__chord_entry` (which is `-> !`) ever returns.
global_asm! {
    // -----------------------------------------------------------------------
    // Boot stack top used by startup assembly.
    // -----------------------------------------------------------------------
    ".extern __chord_boot_stack",
    ".global __stack_top",
    ".set __stack_top, __chord_boot_stack + {stack_size}",

    // -----------------------------------------------------------------------
    // Entry point
    // -----------------------------------------------------------------------
    ".text",
    ".global _start",
    "_start:",

    // The SysV ABI requires the direction flag to be clear on function entry.
    "    cld",

    // Set up the stack. `rdi` still contains the kernel-provided BootInfo
    // pointer, which becomes `__chord_entry`'s first argument.
    "    lea rsp, [rip + __stack_top]",
    "    mov rbp, rsp",

    // Build the same initial call frame shape used by rust-sel4's runtime:
    // `rsp` is 16-byte aligned before `call`, so the Rust function enters with
    // `rsp % 16 == 8`, as required by the x86-64 SysV ABI.
    "    sub rsp, 8",
    "    push rbp",
    "    call __chord_entry",
    // __chord_entry is -> !, but if it somehow returns, halt.
    "2:  jmp 2b",
    stack_size = const STACK_SIZE,
}

// Compile-time check only: `BootStack` aligns the base address, while
// `STACK_SIZE` must preserve that alignment at `__stack_top`, where `_start`
// installs `rsp`.
const _: () = assert!(STACK_SIZE.is_multiple_of(core::mem::align_of::<BootStack>()));
