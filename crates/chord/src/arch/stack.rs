//! Architecture-neutral boot stack storage.
//!
//! This module provides the small `UnsafeCell` wrapper used for statically
//! allocated stacks. It follows the stack-storage shape used by `rust-sel4`,
//! while leaving symbol layout and initial stack-pointer derivation to
//! architecture-specific startup code.

use core::cell::UnsafeCell;

/// Statically allocated stack storage.
///
/// The array is wrapped in [`UnsafeCell`] because stack memory is mutated
/// through the CPU stack pointer, not through Rust references. The type is
/// `repr(C)` so the wrapped byte array begins at the same address as the
/// `Stack` value, which lets assembly code treat the symbol address as the
/// start of the storage.
#[repr(C)]
#[cfg_attr(any(target_arch = "aarch64", target_arch = "x86_64"), repr(align(16)))]
#[cfg_attr(target_arch = "arm", repr(align(4)))]
pub struct Stack<const N: usize>(UnsafeCell<[u8; N]>);

// SAFETY: Static stack storage is not accessed through shared Rust references.
// Startup assembly installs the initial stack pointer, and normal Rust code then
// mutates the bytes only through stack operations.
unsafe impl<const N: usize> Sync for Stack<N> {}

impl<const N: usize> Stack<N> {
    /// Creates zero-initialized stack storage.
    ///
    /// This is `const` so stack storage can live in a `static`.
    pub const fn new() -> Self {
        Self(UnsafeCell::new([0; N]))
    }
}

impl<const N: usize> Default for Stack<N> {
    fn default() -> Self {
        Self::new()
    }
}
