//! `x86_64` `VSpace` bootstrap — large page allocation into the root task's
//! address space.
//!
//! Provides [`bootstrap()`] which allocates a 2 MiB large page from a kernel
//! untyped region and maps it into the root task's `VSpace` at the first
//! 2 MiB-aligned virtual address past the loaded ELF image
//! ([`crate::runtime::image_end`]).
//!
//! # Ordering requirement
//!
//! This module **must** be called before [`Bootstrap::new`](crate::bootstrap::Bootstrap::new)
//! for two reasons:
//!
//! 1. **`CPtr` encoding** — After `expand_current_cspace` (called inside
//!    `Bootstrap::new`), all `CPtr` resolution switches from flat to two-level
//!    encoding. Bootinfo slot indices are flat `CPtr`s and become invalid in
//!    the expanded `CSpace`.
//!
//! 2. **Slot allocation** — `Bootstrap::new` consumes slots from the end of the
//!    empty region for pool `CNode`s. This module takes the last remaining
//!    empty slot, so it must run before those slots are occupied.
//!
//! The untyped search uses a **last-match** strategy (iterating from the end of
//! the kernel untyped range) so that [`Bootstrap::new`] — which uses a
//! **first-match** strategy — does not collide with this module's untyped choice.
//!
//! # Why a separate large page?
//!
//! The root task's ELF image is mapped by the kernel at boot time using 4 KiB
//! pages. To demonstrate seL4 `VSpace` management on `x86_64`, this module
//! allocates a single 2 MiB large page (a *superpage*) and maps it into the
//! root task's `VSpace`. Large pages reduce TLB pressure for contiguous memory
//! regions and serve as the foundation for future heap or stack expansion.
//!
//! # `x86_64` virtual memory hierarchy
//!
//! ```text
//! Level 0: PML4    (512 GiB entries, 48-bit virtual space)
//! Level 1: PDPT    (1 GiB entries)
//! Level 2: PD      (2 MiB entries — this module maps at this level)
//! Level 3: PT      (4 KiB entries)
//! ```
//!
//! A large page is a 2 MiB entry placed directly in a page-directory (PD)
//! table. The kernel handles PD insertion implicitly via `seL4_X86_Page_Map`
//! — no explicit page-directory allocation is required when mapping at a
//! previously-unmapped PDPT slot.
//!
//! # Alignment
//!
//! Large pages on `x86_64` require 2 MiB alignment in both the virtual address
//! and the physical address. The kernel enforces this alignment when mapping.
//! This module rounds the virtual address up to the next 2 MiB boundary past
//! the ELF image end.

use sel4::{BootInfoPtr, CapRights, ObjectBlueprint, cap_type, debug_println, init_thread::Slot};

use crate::bootstrap::PoolSize;
use crate::runtime::image_end;

/// Size of a large page on `x86_64`: 2 MiB.
///
/// Large pages (also called *superpages*) on `x86_64` are 2 MiB pages that
/// map directly into a page-directory entry, skipping the page-table level.
/// This constant is used for virtual-address alignment when mapping a
/// [`LargePage`](cap_type::LargePage) frame and for sizing the heap region
/// in [`main`](crate::main).
pub const LARGE_PAGE_SIZE: usize = 2 * 1024 * 1024;

/// Allocates a 2 MiB large page and maps it into the root task's `VSpace`.
///
/// # How it works
///
/// 1. Scans the kernel untyped range **from the end** for a non-device region
///    with `size_bits >= 21` (2 MiB or larger, i.e. [`PoolSize::TwoM`]). This *last-match* strategy
///    avoids colliding with [`Bootstrap::new`](crate::bootstrap::Bootstrap::new)
///    which uses a *first-match* strategy on the same range.
/// 2. Retypes the untyped into a [`LargePage`](cap_type::LargePage) frame cap
///    placed in a free root `CNode` slot.
/// 3. Maps the frame into the root task's `VSpace` at the first 2 MiB-aligned
///    virtual address past the ELF image end, with read/write permissions and
///    default `x86_64` VM attributes (cacheable, write-back).
///
/// # Ordering
///
/// Must be called **before** [`Bootstrap::new`](crate::bootstrap::Bootstrap::new).
/// See [module-level docs](self) for details.
///
/// # Returns
///
/// `(virtual_address, page_size_in_bytes, untyped_idx)` — the virtual address
/// at which the large page was mapped, its size, and the index into the
/// bootinfo untyped list of the region consumed for this allocation.
///
/// # Errors
///
/// Panics if no suitable untyped region or free `CNode` slot is available.
/// Returns [`sel4::Error`] if the kernel rejects the retype or map operation.
pub fn bootstrap(bootinfo: &BootInfoPtr) -> sel4::Result<(usize, usize, usize)> {
    let untyped_list = bootinfo.untyped_list();

    // Search from the end of the kernel untyped range so we don't collide
    // with Bootstrap::new which picks the first suitable untyped.
    let untyped_idx = bootinfo
        .kernel_untyped_range()
        .rev()
        .find(|&i| {
            !untyped_list[i].is_device()
                && untyped_list[i].size_bits() >= PoolSize::TwoM.frame_size_bits()
        })
        .expect("no suitable untyped for large page");

    let untyped_desc = &untyped_list[untyped_idx];
    debug_println!(
        "[chord] vspace: using untyped idx={} paddr={:#x} size_bits={} for large page",
        untyped_idx,
        untyped_desc.paddr(),
        untyped_desc.size_bits(),
    );

    let frame_slot = free_slot(bootinfo);
    let untyped_slot = bootinfo.untyped().index(untyped_idx);

    retype_to_large_page(untyped_slot, frame_slot)?;
    let (vaddr, size) = map_large_page(frame_slot)?;

    Ok((vaddr, size, untyped_idx))
}

/// Finds the highest free slot in the root `CNode` empty slot region.
///
/// Uses the last slot in the empty region (highest index) so that lower slots
/// remain available for `Bootstrap` pool allocations.
///
/// # Panics
///
/// Panics if the empty slot region is exhausted (no free slots remain).
fn free_slot(bootinfo: &BootInfoPtr) -> Slot<cap_type::Null> {
    bootinfo
        .empty()
        .range()
        .rev()
        .map(Slot::from_index)
        .next()
        .expect("no free CSlot for large page frame")
}

/// Retypes a kernel untyped region into a [`LargePage`](cap_type::LargePage)
/// frame capability.
///
/// # How it works
///
/// Calls `seL4_Untyped_Retype` on the given untyped slot, producing one 2 MiB
/// `LargePage` frame capability placed at `frame_slot` in the root `CNode`.
///
/// # Kernel call
///
/// `ObjectBlueprint::Arch(ObjectBlueprintX86::LargePage)` rewrites to
/// `seL4_X86_LargePageObject` with `size_bits = 21` (2 MiB, [`PoolSize::TwoM`]).
fn retype_to_large_page(
    untyped_slot: Slot<cap_type::Untyped>,
    frame_slot: Slot<cap_type::Null>,
) -> sel4::Result<()> {
    let dst = sel4::init_thread::slot::CNODE
        .cap()
        .absolute_cptr_for_self();

    untyped_slot.cap().untyped_retype(
        &ObjectBlueprint::Arch(sel4::ObjectBlueprintX86::LargePage),
        &dst,
        frame_slot.index(),
        1,
    )?;

    debug_println!(
        "[chord] vspace: large page frame cap at slot={}",
        frame_slot.index(),
    );

    Ok(())
}

/// Maps a [`LargePage`](cap_type::LargePage) frame into the root task's `VSpace`.
///
/// # How it works
///
/// 1. Computes the first 2 MiB-aligned virtual address past the ELF image end.
/// 2. Invokes `seL4_X86_Page_Map` with read/write permissions and default
///    `x86_64` VM attributes (cacheable, write-back).
///
/// # Alignment
///
/// The `x86_64` architecture requires 2 MiB alignment for large page mappings.
/// [`image_end()`] returns a 4 KiB-aligned address; this function rounds it up
/// to the next 2 MiB boundary (`LARGE_PAGE_SIZE`).
fn map_large_page(frame_slot: Slot<cap_type::Null>) -> sel4::Result<(usize, usize)> {
    let vaddr = image_end().next_multiple_of(LARGE_PAGE_SIZE);

    let vspace = sel4::init_thread::slot::VSPACE;
    let frame_cap = frame_slot.cast::<cap_type::LargePage>().cap();

    frame_cap.frame_map(
        vspace.cap(),
        vaddr,
        CapRights::read_write(),
        sel4::VmAttributes::DEFAULT,
    )?;

    debug_println!(
        "[chord] vspace: large page mapped at vaddr={:#x} size=2MiB",
        vaddr,
    );

    Ok((vaddr, LARGE_PAGE_SIZE))
}
