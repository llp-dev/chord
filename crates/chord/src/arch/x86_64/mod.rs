//! `x86_64` architecture support.
//!
//! Provides the x86_64-specific low-level primitives:
//!
//! - [`start`] — assembly `_start` entry point that sets up the kernel
//!   stack and calls into the Rust runtime ([`crate::runtime::__chord_entry`]).
//! - [`cpu`] — `CPUID`-based processor identification and feature
//!   detection.
//!
//! # ABI
//!
//! Follows the `SystemV` AMD64 ABI:
//! - `rdi` carries the seL4 `BootInfo` pointer into the root task.
//! - The stack is 16-byte aligned before `call` instructions.

pub mod cpu;
pub mod start;
