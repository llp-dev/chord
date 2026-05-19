//! Architecture abstraction layer.
//!
//! Defines arch-neutral traits ([`ArchBootstrap`], [`CpuDetect`]) so that
//! [`main`](crate::main) never references architecture-specific modules.
//! The concrete [`Arch`] type alias dispatches via `cfg(target_arch = ...)`.
//!
//! Also exports [`abort`] for architecture-independent CPU halt.

use core::fmt::Display;
use sel4::BootInfoPtr;

// ---------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------

/// `VSpace` bootstrap — allocate and map a large page into the root task.
pub trait ArchBootstrap {
    /// Error type (bootstrap can fail on kernel rejection of retype/map).
    type Error: core::fmt::Debug;

    /// Allocates a large page from kernel untyped and maps it into `VSpace`.
    ///
    /// Returns `(virtual_address, page_size_in_bytes, untyped_idx)`.
    /// The `untyped_idx` is the index into the bootinfo untyped list of the
    /// region consumed for the large page, so that [`Bootstrap::new`]
    /// can skip it during pool population.
    fn bootstrap_vspace(bootinfo: &BootInfoPtr) -> Result<(usize, usize, usize), Self::Error>;
}

/// CPU detection — probe processor identification and feature flags.
pub trait CpuDetect {
    /// Architecture-specific CPU information type (must be `Display`).
    type Info: Display;

    /// Probes the CPU. Returns `None` when detection is unsupported.
    fn detect() -> Option<Self::Info>;
}

// ---------------------------------------------------------------------------
// Concrete architecture dispatch
// ---------------------------------------------------------------------------

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
mod imp {
    use super::{ArchBootstrap, BootInfoPtr, CpuDetect};

    /// Marker struct implementing `x86_64` arch traits.
    pub struct X86_64;

    impl ArchBootstrap for X86_64 {
        type Error = sel4::Error;

        fn bootstrap_vspace(bootinfo: &BootInfoPtr) -> Result<(usize, usize, usize), Self::Error> {
            super::x86_64::bootstrap::bootstrap(bootinfo)
        }
    }

    impl CpuDetect for X86_64 {
        type Info = super::x86_64::cpu::ArchInfo;

        fn detect() -> Option<Self::Info> {
            super::x86_64::cpu::detect()
        }
    }
}

/// The architecture type for the current target.
///
/// `main.rs` uses this type alias — it never imports architecture-specific
/// modules directly. Unsupported architectures trigger a `compile_error!`.
#[cfg(target_arch = "x86_64")]
pub type Arch = imp::X86_64;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("unsupported target architecture — only x86_64 is implemented");

// ---------------------------------------------------------------------------
// Abort — architecture-independent CPU halt
// ---------------------------------------------------------------------------

/// Halts the CPU immediately via the architecture-specific abort primitive.
///
/// On `x86_64` this executes `ud2`; on unsupported architectures it loops
/// forever. This function never returns.
#[cold]
#[inline(never)]
pub fn abort() -> ! {
    #[cfg(target_arch = "x86_64")]
    {
        self::x86_64::abort::abort_now()
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        // Unreachable — compile_error! gates this branch.
        loop {}
    }
}
