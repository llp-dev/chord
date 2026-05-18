//! Architecture-specific code.
//!
//! Conditionally compiles the correct architecture support based on
//! `target_arch`. Each submodule provides the low-level, arch-specific
//! primitives required by the root-task runtime: startup assembly, static stack
//! storage, and CPU feature detection.
//!
//! # Adding a new architecture
//!
//! 1. Create `arch/{target_arch}/mod.rs` with a `pub mod` declaration for each
//!    arch-specific primitive module.
//! 2. Add a `#[cfg(target_arch = "...")] pub mod {target_arch};` line here.
//! 3. Reuse [`stack`] when the architecture needs a static bootstrap stack.

pub mod stack;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
