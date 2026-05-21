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
//! - Ten **pool `CNode`s** (one per slab size: 4 KiB through 1 GiB) hold
//!   untyped capabilities organized by frame size.
//! - A **`CSpace` guard** is configured so pool slots are directly addressable
//!   via a compact `CPtr` encoding.
//!
//! # Boot sequence
//!
//! 1. Detect CPU information via `CPUID` and print to the debug console.
//! 2. Dump bootinfo to the kernel debug console.
//! 3. Allocate a 2 MiB large page into the root task's `VSpace`.
//! 4. Initialize the talc heap with the bootstrapped large page.
//! 5. Construct the [`Bootstrap`] state (pool `CNode` creation, all pools
//!    populated, `CSpace` guard configuration).
//! 6. Allocate a 1 MiB [`Vec`] to verify the talc heap handles large allocations.
//! 7. Suspend the root task (no further work in this `PoC`).
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

unsafe extern "C" {
    /// Returns 42. Declared in `mach/ipc/test.c`.
    fn chord_test() -> i32;
}

use bootstrap::Bootstrap;
use bootstrap::cspace;
use sel4::{BootInfoPtr, debug_println};

use arch::{Arch, ArchBootstrap, CpuDetect};
use arch::x86_64::time;

/// seL4 root-task entry point.
///
/// This function is invoked by [`runtime::__chord_entry`] after the custom
/// runtime has set up the stack and initialized the IPC buffer.
///
/// # Steps
///
/// 1. Detect CPU information and print it to the kernel debug console.
/// 2. Dump bootinfo to the kernel debug console for inspection.
/// 3. Allocate a 2 MiB large page into the root task's `VSpace` (must happen
///    before `CSpace` expansion so flat `CPtr` encoding is still valid).
/// 4. Initialize the talc heap with the bootstrapped large page (must happen
///    before step 5 because [`Bootstrap::new`] allocates via `Vec`).
/// 5. Construct [`Bootstrap`] via [`Bootstrap::new`] — this creates the pool
///    `CNode`s, populates all pools, configures the `CSpace` guard, and
///    verifies pool reachability.
/// 6. Allocate a 1 MiB [`Vec`] to verify the talc heap handles large allocations.
/// 7. Suspend the root task via [`cspace::suspend_self`]. The root task will
///    never be scheduled again.
///
/// # Panics
///
/// Panics if [`Bootstrap::new`] fails (should not happen with a valid bootinfo
/// from the seL4 kernel).
#[unsafe(no_mangle)]
extern "C" fn main(bootinfo: &BootInfoPtr) -> ! {
    match <Arch as CpuDetect>::detect() {
        Some(info) => debug_println!("[chord] CPU: {info}"),
        None => debug_println!("[chord] CPU: CPUID not supported"),
    }

    // --- C function test ---
    // SAFETY: chord_test is a pure C function with no side effects
    // beyond its return value. It is safe to call at any point.
    let c_result = unsafe { chord_test() };
    debug_println!("[chord] C function test: chord_test() returned {c_result}");

    debug::dump_bootinfo(bootinfo);

    time::init(bootinfo);
    debug_println!("[chord] TSC frequency: {} MHz", time::tsc_freq_mhz());
    debug_println!("[chord] timer base captured (ticks=0)");

    // VSpace bootstrap must run before Bootstrap::new because:
    // 1. Bootstrap::new calls expand_current_cspace, which switches CPtr
    //    encoding from flat to two-level. Bootinfo slot indices are flat CPtrs
    //    and become invalid in the expanded CSpace.
    // 2. Bootstrap::new consumes empty CNode slots from the same region this
    //    module uses. Running first ensures slots are truly empty.
    let (large_page_vaddr, large_page_size, vspace_untyped_idx) =
        <Arch as ArchBootstrap>::bootstrap_vspace(bootinfo)
            .inspect_err(|_| debug_println!("[chord] ERROR: VSpace bootstrap failed"))
            .expect("Failed to allocate large page into VSpace");

    debug_println!("[chord] large page mapped at vaddr={:#x}", large_page_vaddr);

    // Initialize the talc heap with the bootstrapped large page.
    // Must happen before Bootstrap::new — it allocates via Vec.
    // SAFETY: The large page is valid, writable, and exclusively owned
    // by the allocator for the remainder of the program.
    unsafe {
        runtime::init_heap(large_page_vaddr as *mut u8, large_page_size);
    }

    let bootstrap = Bootstrap::new(bootinfo, vspace_untyped_idx)
        .inspect_err(|_| debug_println!("[chord] ERROR: Bootstrap::new failed"))
        .expect("Failed to create bootstrap state");

    debug_println!(
        "[chord] bootstrap cnode backing untyped idx={}",
        bootstrap.pool_cnode_untyped_idx
    );

    // --- 1 MiB allocation test ---
    // Uses Vec::with_capacity to allocate directly on the heap (no stack
    // overhead — the boot stack is only 64 KiB).
    let mut meg: alloc::vec::Vec<u8> = alloc::vec::Vec::with_capacity(1024 * 1024);
    meg.resize(1024 * 1024, 0x42u8);
    meg[0] = 0xAB;
    meg[512 * 1024] = 0xCD;
    meg[1024 * 1024 - 1] = 0xEF;
    assert_eq!(meg[0], 0xAB);
    assert_eq!(meg[512 * 1024], 0xCD);
    assert_eq!(meg[1024 * 1024 - 1], 0xEF);
    debug_println!("[chord] 1 MiB Vec allocation test passed");

    debug_println!("[chord] TEST_PASS");
    cspace::suspend_self(&bootstrap)
}
