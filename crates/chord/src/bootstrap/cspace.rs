//! `CSpace` guard configuration and `CPtr` encoding for pool-slot addressing.
//!
//! After [`expand_current_cspace`] sets a guard on the root `CNode`, pool slots
//! become directly addressable via a two-level `CPtr` encoding:
//!
//! ```text
//! CPtr bits:
//! ┌──────────────┬──────────────┬──────────────────┐
//! │  guard (0s)  │  root slot   │  pool-internal   │
//! │  guard_size  │  root_radix  │  pool_slot_radix │
//! └──────────────┴──────────────┴──────────────────┘
//! ```
//!
//! The guard is always zero. The kernel skips `guard_size` high bits, uses
//! `root_radix` bits to index into the root `CNode`, then uses
//! `pool_slot_radix` bits to index into the pool `CNode`.
//!
//! # `CSpace` layout transformation
//!
//! ## Before: flat single-level `CSpace` (kernel-provided)
//!
//! ```text
//! root CNode (4096 slots, radix=12)
//! ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
//! │  0  │  1  │  2  │ ... │ TCB │ ... │ VSP │ ... │  ← flat CPtr = slot index
//! └─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘
//!   IPC   ...   ...       TCB       VSPACE   ...
//! ```
//!
//! All capabilities are addressed by their slot index directly. The kernel
//! places well-known caps (TCB, `VSpace`, IPC buffer, untyped regions) at
//! fixed indices.
//!
//! ## After: two-level `CSpace` (post-bootstrap)
//!
//! ```text
//! root CNode (4096 slots, radix=12)
//! ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
//! │  0  │  1  │  2  │ ... │ TCB │ ... │ VSP │ ... │
//! └─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘
//!   IPC   ...   ...       TCB       VSPACE   ...
//!                     ┌─────┬─────┬─────┐
//!                     │ S   │ L   │ H   │  ← pool CNode slots
//!                     └─────┴─────┴─────┘
//!                       │     │     │
//!                       ▼     ▼     ▼
//!                     ┌───┐ ┌───┐ ┌───┐
//!                     │CNode│ │CNode│ │CNode│  ← pool CNodes (radix=N)
//!                     └───┘ └───┘ └───┘
//!                       │     │     │
//!                       ▼     ▼     ▼
//!                     [4K]  [2M]  [1G]  ← untyped blocks
//! ```
//!
//! Pool slots are addressed by a two-level `CPtr`:
//! `(cnode_slot.index << pool_slot_radix) | internal_slot`. The guard
//! ensures the kernel strips the high bits before walking the tree.
//!
//! # Key functions
//!
//! - [`expand_current_cspace`] — configures the guard via `tcb_set_space`.
//! - [`root_slot_cptr_bits`] — encodes a root `CNode` slot into `CPtr` bits.
//! - [`current_tcb`] — resolves the TCB cap through the expanded `CSpace`.
//! - [`suspend_self`] — suspends the root task (never returns).

use sel4::{CNodeCapData, CPtr, CPtrBits, WORD_SIZE, cap, debug_println, init_thread::Slot};

use super::bootstrap::Bootstrap;
use super::error::BootstrapError;

/// Reconfigures the current thread's [`CSpace`](cap_type::CNode) so pool
/// [`CNode`](cap_type::CNode) slots are directly addressable via [`CPtr`].
///
/// # How it works
///
/// The seL4 `CSpace` guard mechanism works as follows: when the kernel resolves
/// a `CPtr`, it first checks that the top `guard_size` bits match the guard
/// value (which we set to 0). If they match, the kernel strips those bits and
/// uses the remaining bits to walk the `CSpace` tree.
///
/// By setting `guard_size = WORD_SIZE - (root_radix + pool_slot_radix)`, we
/// ensure that:
///
/// 1. All valid pool `CPtr`s have zeros in the high bits (the guard).
/// 2. The next `root_radix` bits select a slot in the root `CNode`.
/// 3. The remaining `pool_slot_radix` bits select a slot inside the pool
///    `CNode`.
///
/// # Parameters
///
/// The function reads from `bootstrap`:
/// - `root_cnode_radix` — log2 of the root `CNode` slot count.
/// - `pool_slot_radix` — log2 of the pool `CNode` slot count.
/// - `root_cnode` — the root `CNode` capability slot.
///
/// # Errors
///
/// Returns [`BootstrapError::Sel4`] if `tcb_set_space` fails (e.g., invalid
/// capability, insufficient rights).
///
/// # Kernel call
///
/// Invokes `seL4_TCB_SetSpace` with:
/// - `fault_ep` = `NULL` (no fault endpoint).
/// - `cspace_root` = root `CNode` (unchanged).
/// - `cspace_data` = `CNodeCapData::new(guard=0, guard_size)`.
/// - `vspace_root` = `VSpace` (unchanged).
pub(super) fn expand_current_cspace(bootstrap: &Bootstrap) -> Result<(), BootstrapError> {
    // Total addressable depth: root_radix bits select the pool CNode slot
    // in the root CNode, then pool_slot_radix bits select the internal slot.
    let depth = bootstrap.root_cnode_radix + bootstrap.pool_slot_radix;

    // The guard consumes all bits above the addressable depth.
    // Since our CPtrs always have zeros there, the guard always matches.
    let guard_size = WORD_SIZE - depth;

    debug_println!(
        "[chord] expanding current CSpace: root_radix={} pool_radix={} guard_size={}",
        bootstrap.root_cnode_radix,
        bootstrap.pool_slot_radix,
        guard_size,
    );

    sel4::init_thread::slot::TCB
        .cap()
        .tcb_set_space(
            sel4::init_thread::slot::NULL.cptr(),
            bootstrap.root_cnode.cap(),
            CNodeCapData::new(0, guard_size),
            sel4::init_thread::slot::VSPACE.cap(),
        )
        .map_err(BootstrapError::from)
}

/// Encodes a root [`CNode`](cap_type::CNode) slot index into the post-guard
/// [`CPtr`] format.
///
/// After [`expand_current_cspace`] has been called, a root `CNode` slot is
/// addressed by shifting its index left by `pool_slot_radix` bits, leaving
/// room for the pool-internal slot index in the low bits.
///
/// # Encoding
///
/// ```text
/// CPtr = (slot.index << pool_slot_radix)
/// ```
///
/// The pool-internal slot index is OR'd in later by
/// [`Pool::cptr_bits_for_slot`](super::Pool::cptr_bits_for_slot).
pub(super) const fn root_slot_cptr_bits(bootstrap: &Bootstrap, slot: Slot) -> CPtrBits {
    (slot.index() << bootstrap.pool_slot_radix) as CPtrBits
}

/// Returns a [`Tcb`](cap::Tcb) capability resolved through the expanded
/// [`CSpace`](cap_type::CNode).
///
/// Constructs the [`CPtr`] for the TCB root slot (the slot where the kernel
/// placed the initial thread's TCB capability) and casts it to a typed TCB
/// capability handle.
///
/// # How it works
///
/// 1. Looks up the well-known TCB slot index via
///    `sel4::init_thread::slot::TCB`.
/// 2. Encodes it with [`root_slot_cptr_bits`] to produce the post-guard
///    `CPtr` bits.
/// 3. Constructs a [`CPtr`] from those bits and casts to [`cap::Tcb`].
///
/// This function only works correctly *after* [`expand_current_cspace`] has
/// been called, because the `CPtr` encoding assumes the guard is in place.
pub const fn current_tcb(bootstrap: &Bootstrap) -> cap::Tcb {
    CPtr::from_bits(root_slot_cptr_bits(
        bootstrap,
        sel4::init_thread::slot::TCB.upcast(),
    ))
    .cast()
}

/// Suspends the current root task thread using the expanded
/// [`CSpace`](cap_type::CNode).
///
/// Resolves the TCB capability through the new `CSpace` layout and invokes
/// `seL4_TCB_Suspend`. The root task will never be scheduled again after this
/// call — in a real system, other threads or processes would take over.
///
/// # Difference from `sel4::init_thread::suspend_self`
///
/// The upstream `sel4::init_thread::suspend_self()` uses the pre-expansion
/// flat `CPtr` encoding (well-known slot indices). After
/// [`expand_current_cspace`] configures the guard, that encoding is no longer
/// valid — all `CPtr` resolution goes through the two-level scheme
/// (guard → root `CNode` slot → pool-internal slot). This function resolves
/// the TCB through the expanded `CSpace` using [`current_tcb`], which is the
/// only correct approach after bootstrap.
///
/// # Panics
///
/// Panics if `tcb_suspend` returns an error. This should be unreachable for a
/// root task with a valid TCB capability and proper `CSpace` configuration.
///
/// # Never returns
///
/// This function has return type [`!`] (never). After `tcb_suspend` succeeds,
/// the calling thread is permanently descheduled by the kernel.
pub fn suspend_self(bootstrap: &Bootstrap) -> ! {
    current_tcb(bootstrap)
        .tcb_suspend()
        .expect("tcb_suspend should not fail for root task");
    unreachable!()
}
