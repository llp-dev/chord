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
//! The boot stack uses [`Stack`](crate::arch::stack::Stack) storage in a Rust
//! static. Startup assembly derives the initial stack pointer from the exported
//! stack symbol, so no custom linker script is needed for stack setup.

use core::arch::global_asm;

use crate::arch::stack::Stack;

const STACK_SIZE: usize = 64 * 1024;

#[allow(non_upper_case_globals)]
#[used]
#[unsafe(no_mangle)]
static __chord_boot_stack: Stack<STACK_SIZE> = Stack::new();

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

// Compile-time check only: `Stack` aligns the base address, while `STACK_SIZE`
// must preserve that alignment at `__stack_top`, where `_start` installs `rsp`.
const _: () = assert!(STACK_SIZE.is_multiple_of(core::mem::align_of::<Stack<STACK_SIZE>>()));
