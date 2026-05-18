#![no_std]
#![no_main]
#![warn(missing_docs)]

//! Bare-metal seL4 root task that bootstraps capability pools and `CSpace`
//! guard configuration for direct pool-slot addressing.
//!
//! # What this crate does
//!
//! After the seL4 kernel boots the root task, the initial thread has a flat,
//! single-level `CSpace`. This root task transforms that layout into a
//! two-level `CSpace` where:
//!
//! - A **root `CNode`** holds the pool `CNode` capabilities and other
//!   kernel-provided caps.
//! - Three **pool `CNode`s** (Small / Large / Huge) hold untyped capabilities
//!   organized by frame size.
//! - A **`CSpace` guard** is configured so pool slots are directly addressable
//!   via a compact `CPtr` encoding.
//!
//! # Boot sequence
//!
//! 1. Detect CPU information via `CPUID` and print to the debug console.
//! 2. Dump bootinfo to the kernel debug console.
//! 3. Construct the [`Bootstrap`] state (pool `CNode` creation, Small pool
//!    population, `CSpace` guard configuration).
//! 4. Suspend the root task (no further work in this `PoC`).
//!
//! # `CPtr` encoding after bootstrap
//!
//! ```text
//! CPtr bits (WORD_SIZE wide):
//! ┌──────────────┬──────────────┬──────────────────┐
//! │  guard (0s)  │  root slot   │  pool-internal   │
//! │  guard_size  │  root_radix  │  pool_slot_radix │
//! └──────────────┴──────────────┴──────────────────┘
//! ```

extern crate alloc;

mod arch;
mod bootstrap;
mod debug;
mod runtime;

use bootstrap::Bootstrap;
use bootstrap::cspace;
use sel4::{BootInfoPtr, debug_println};

/// seL4 root-task entry point.
///
/// This function is invoked by [`runtime::__chord_entry`] after the custom
/// runtime has set up the stack and initialized the IPC buffer.
///
/// # Steps
///
/// 1. Detect CPU information and print it to the kernel debug console.
/// 2. Dump bootinfo to the kernel debug console for inspection.
/// 3. Construct [`Bootstrap`] via [`Bootstrap::new`] — this creates the pool
///    `CNode`s, populates the Small pool, configures the `CSpace` guard, and
///    verifies pool reachability.
/// 4. Suspend the root task via [`cspace::suspend_self`]. The root task will
///    never be scheduled again.
///
/// # Panics
///
/// Panics if [`Bootstrap::new`] fails (should not happen with a valid bootinfo
/// from the seL4 kernel).
#[unsafe(no_mangle)]
extern "C" fn main(bootinfo: &BootInfoPtr) -> ! {
    match arch::x86_64::cpu::detect() {
        Some(info) => debug_println!("[chord] CPU: {info}"),
        None => debug_println!("[chord] CPU: CPUID not supported"),
    }

    debug::dump_bootinfo(bootinfo);

    let bootstrap = Bootstrap::new(bootinfo).expect("Failed to create bootstrap state");

    debug_println!(
        "[chord] bootstrap cnode backing untyped idx={}",
        bootstrap.pool_cnode_untyped_idx
    );

    cspace::suspend_self(&bootstrap)
}
