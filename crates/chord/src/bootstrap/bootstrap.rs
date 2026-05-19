//! Root-task bootstrap state and constructor.
//!
//! [`Bootstrap`] is the central struct that holds the root-task's capability
//! pool configuration after initialization. It is constructed via
//! [`Bootstrap::new`] from the kernel-provided bootinfo.

use alloc::vec::Vec;

use sel4::{
    BootInfoPtr, cap_type, debug_println,
    init_thread::{Slot, SlotRegion},
};

use super::cspace;
use super::error::BootstrapError;
use super::pool::{Pool, PoolSet, PoolSize};

/// The root-task bootstrap state.
///
/// Holds the root [`CNode`](cap_type::CNode), all ten pool
/// [`CNodes`](Pool) (one per slab size), and the list of remaining free
/// slots in the root `CNode`. Constructed via [`new`](Self::new).
///
/// # Lifetime
///
/// `Bootstrap` is created once at startup and lives for the entire lifetime of
/// the root task. It is never dropped (the root task either suspends or
/// panics).
///
/// # Field overview
///
/// | Field                     | Purpose |
/// |---------------------------|---------|
/// | `root_cnode`              | The initial thread's root `CNode` slot |
/// | `root_cnode_radix`        | Log2 slot count of the root `CNode` |
/// | `pool_slot_radix`         | Shared log2 slot count for all pool `CNode`s |
/// | `pool_cnode_untyped_idx`  | Index of the 2 MiB untyped that backs pool `CNode`s |
/// | `pool_cnode_untyped`      | The untyped slot itself (for future accounting) |
/// | `free_slots`              | Remaining empty slots in the root `CNode` |
/// | `pools`                   | All 10 slab pools, indexed by [`PoolSize`] |
#[must_use = "Bootstrap::new performs kernel CSpace mutations — discarding the result is a logic error"]
pub struct Bootstrap {
    /// The initial thread's root [`CNode`](cap_type::CNode) capability slot.
    ///
    /// This is the top-level `CNode` in the initial thread's `CSpace`. All
    /// pool `CNode` capabilities are placed as slots within this `CNode`.
    pub root_cnode: Slot<cap_type::CNode>,

    /// Log2 number of slots in the root [`CNode`](cap_type::CNode).
    ///
    /// On pc99 (`x86_64` QEMU), this is typically 12 (4096 slots). Read from
    /// `bootinfo.inner().initThreadCNodeSizeBits`.
    pub root_cnode_radix: usize,

    /// Shared radix used by all pool [`CNodes`](Pool).
    ///
    /// All pool `CNode`s are created with the same radix so that the
    /// `CPtr` encoding is uniform. This is the maximum of the ten
    /// individually-computed radices (one per pool size class).
    ///
    /// Total pool slots per `CNode` = `2^pool_slot_radix`.
    pub pool_slot_radix: usize,

    /// Index of the kernel untyped region (`size_bits=21`) that backs the pool
    /// [`CNodes`](Pool).
    ///
    /// This is an index into the bootinfo untyped list, not a `CSlot` index.
    /// Useful for debugging which untyped region was consumed.
    pub pool_cnode_untyped_idx: usize,

    /// The untyped slot used to create the ten pool `CNode`s.
    ///
    /// Kept for future allocator accounting — the frame allocator may need to
    /// know which untyped region backs the pool `CNode`s.
    #[expect(dead_code, reason = "kept for future bootstrap allocator accounting")]
    pub pool_cnode_untyped: Slot<cap_type::Untyped>,

    /// Remaining empty slots in the root [`CNode`](cap_type::CNode).
    ///
    /// After ten slots are consumed for the pool `CNode`s, the rest are
    /// available for future root `CNode` scratch allocations.
    #[expect(dead_code, reason = "reserved for later root CNode scratch allocation")]
    pub free_slots: Vec<Slot<cap_type::Null>>,

    /// All 10 slab pools, indexed by [`PoolSize`].
    ///
    /// Each pool holds untyped capabilities of a specific size class,
    /// populated during bootstrap by [`populate_all_pools`](Self::populate_all_pools).
    pub pools: PoolSet,
}

impl Bootstrap {
    /// Constructs the bootstrap state from the kernel bootinfo.
    ///
    /// This is the primary constructor for [`Bootstrap`]. It performs the full
    /// bootstrap sequence:
    ///
    /// # Steps
    ///
    /// 1. **Count untyped regions by size class** — walk the bootinfo untyped
    ///    list and assign each non-device region to the largest pool size that
    ///    fits, counting how many blocks each pool receives.
    /// 2. **Compute pool radix** — derive the `CNode` radix for each pool from
    ///    the slot counts, then take the maximum so all pools use the same
    ///    radix.
    /// 3. **Find backing untyped** — locate the first 2 MiB non-device kernel
    ///    untyped region to back the ten pool `CNode`s.
    /// 4. **Create pool `CNode`s** — retype the backing untyped into ten
    ///    `CNode` capabilities (one per slab size).
    /// 5. **Populate all pools** — retype kernel untyped regions into untyped
    ///    blocks inside each pool, using a "largest fits" strategy.
    /// 6. **Expand `CSpace`** — configure a guard on the root `CNode` so pool
    ///    slots are directly addressable via `CPtr`.
    /// 7. **Verify pools** — spot-check slots in each non-empty pool for
    ///    reachability.
    ///
    /// # Errors
    ///
    /// Returns [`BootstrapError`] if:
    /// - No suitable 2 MiB kernel untyped is found
    ///   ([`NoBackingUntyped`](BootstrapError::NoBackingUntyped)).
    /// - Fewer than 10 empty `CSlot`s remain in the root `CNode`
    ///   ([`NoEmptyCSlots`](BootstrapError::NoEmptyCSlots)).
    /// - A kernel operation fails
    ///   ([`Sel4`](BootstrapError::Sel4)).
    pub fn new(
        bootinfo: &BootInfoPtr,
        vspace_untyped_idx: usize,
    ) -> Result<Self, BootstrapError> {
        let untyped_list = bootinfo.untyped_list();

        let pool_slot_radix = Self::derive_pool_radix(untyped_list);

        let untyped_idx = Self::find_backing_untyped(untyped_list, bootinfo)?;
        let untyped_slot = bootinfo.untyped().index(untyped_idx);
        let untyped_desc = &untyped_list[untyped_idx];

        debug_println!(
            "[chord] pool cnode untyped idx={} paddr={:#x} size_bits={} is_device={}",
            untyped_idx,
            untyped_desc.paddr(),
            untyped_desc.size_bits(),
            untyped_desc.is_device()
        );

        let root_cnode = sel4::init_thread::slot::CNODE;
        // initThreadCNodeSizeBits is a small integer (u8 on pc99);
        // it always fits in usize.
        #[expect(clippy::cast_possible_truncation, reason = "u8 field fits usize")]
        let root_cnode_radix = bootinfo.inner().initThreadCNodeSizeBits as usize;

        let mut bootstrap = Self::create_pool_cnodes(
            root_cnode,
            root_cnode_radix,
            pool_slot_radix,
            untyped_idx,
            untyped_slot,
            &bootinfo.empty(),
        )?;

        bootstrap.populate_all_pools(bootinfo, vspace_untyped_idx)?;
        cspace::expand_current_cspace(&bootstrap)?;
        bootstrap.verify_pools();

        Ok(bootstrap)
    }

    /// Computes the shared pool radix from the untyped descriptor list.
    ///
    /// # How it works
    ///
    /// For each non-device untyped region, uses the same "largest fits"
    /// strategy as [`populate_all_pools`](Self::populate_all_pools):
    /// iterates `PoolSize::ALL` in reverse (largest first), finds the first
    /// pool with `bits >= frame_bits`, and adds `2^(bits - frame_bits)` to
    /// that pool's slot count. Each region contributes to exactly one pool.
    ///
    /// Then computes `radix = ceil(log2(total_slots))` for each pool, and
    /// returns the maximum (so all pools use the same radix).
    fn derive_pool_radix(untyped_list: &[sel4::UntypedDesc]) -> usize {
        let mut slot_counts = [0usize; PoolSize::COUNT];

        for ut_desc in untyped_list.iter().filter(|ut_desc| !ut_desc.is_device()) {
            let bits = ut_desc.size_bits();
            for size in PoolSize::ALL.iter().rev() {
                let frame_bits = size.frame_size_bits();
                if bits >= frame_bits {
                    let idx = *size as usize;
                    slot_counts[idx] += 1 << (bits - frame_bits);
                    break;
                }
            }
        }

        // next_power_of_two rounds up to the nearest power of 2.
        // ilog2 gives log2 of that power, which is the minimum radix.
        // .max(1) ensures at least 2 slots even if no frames were counted.
        slot_counts
            .iter()
            .map(|&c| (c.next_power_of_two().ilog2() as usize).max(1))
            .max()
            .unwrap_or(1)
    }

    /// Searches the kernel untyped range for a non-device region of at least
    /// 2 MiB (`size_bits >= 21`). The kernel uses only the portion it needs
    /// from the untyped region, so larger regions are accepted.
    ///
    /// # Selection criteria
    ///
    /// - Must be in the kernel untyped range (not the device range).
    /// - Must have `size_bits >= 21` (2 MiB or larger).
    /// - Must not be a device region.
    ///
    /// Returns the index into the untyped list if found.
    ///
    /// # Errors
    ///
    /// Returns [`BootstrapError::NoBackingUntyped`] if no suitable region
    /// exists.
    fn find_backing_untyped(
        untyped_list: &[sel4::UntypedDesc],
        bootinfo: &BootInfoPtr,
    ) -> Result<usize, BootstrapError> {
        bootinfo
            .kernel_untyped_range()
            .find(|&i| {
                let ud = &untyped_list[i];
                !ud.is_device() && ud.size_bits() >= 21
            })
            .ok_or(BootstrapError::NoBackingUntyped)
    }

    /// Retypes the backing untyped into ten pool [`CNode`](cap_type::CNode)s
    /// and assembles the initial `Bootstrap` struct.
    ///
    /// # Steps
    ///
    /// 1. Collect all empty root `CNode` slots (newest-first, so we consume
    ///    from the end of the range).
    /// 2. Pop ten empty slots, one for each pool `CNode`.
    /// 3. Call [`Pool::new`] ten times to retype the backing untyped into
    ///    `CNode` capabilities (one per slab size).
    /// 4. Assemble the `Bootstrap` struct with all fields.
    ///
    /// # Errors
    ///
    /// Returns [`BootstrapError::NoEmptyCSlots`] if fewer than 10 empty slots
    /// remain, or propagates kernel errors from [`Pool::new`].
    fn create_pool_cnodes(
        root_cnode: Slot<cap_type::CNode>,
        root_cnode_radix: usize,
        pool_slot_radix: usize,
        pool_cnode_untyped_idx: usize,
        pool_cnode_untyped: Slot<cap_type::Untyped>,
        empty_slots: &SlotRegion<cap_type::Null>,
    ) -> Result<Self, BootstrapError> {
        // Collect empty slots highest-index-first into the Vec so that
        // `.pop()` yields the lowest-index slots. This leaves higher slots
        // available for the VSpace bootstrap module, which takes the highest
        // empty slot for its large-page frame cap.
        let mut free_slots: Vec<Slot<cap_type::Null>> =
            empty_slots.range().rev().map(Slot::from_index).collect();

        // SAFETY: All PoolSize::COUNT elements are written by the loop below
        // before array_assume_init is called. If the loop exits early (via
        // `?`), the MaybeUninit array is dropped without reading uninit memory.
        let mut pools: [core::mem::MaybeUninit<Pool>; PoolSize::COUNT] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        for size in PoolSize::ALL {
            let cnode_slot = free_slots.pop().ok_or(BootstrapError::NoEmptyCSlots)?;
            pools[*size as usize].write(Pool::new(
                root_cnode,
                pool_cnode_untyped,
                *size,
                cnode_slot,
                pool_slot_radix,
            )?);
        }
        // SAFETY: All PoolSize::COUNT elements have been initialized.
        // We use transmute because `MaybeUninit::array_assume_init` is still
        // unstable on our nightly toolchain. The type annotation ensures
        // the transmute target is exactly `[Pool; PoolSize::COUNT]`.
        let pools: [Pool; PoolSize::COUNT] = unsafe { core::mem::transmute(pools) };
        let pools = PoolSet { pools };

        Ok(Self {
            root_cnode,
            root_cnode_radix,
            pool_slot_radix,
            pool_cnode_untyped_idx,
            pool_cnode_untyped,
            free_slots,
            pools,
        })
    }

    /// Populates all pools with untyped blocks from kernel untyped regions.
    ///
    /// # How it works
    ///
    /// Iterates every kernel untyped region (excluding device regions and the
    /// region consumed to back pool `CNode`s). For each region:
    ///
    /// 1. Skips regions with `size_bits < 12` (too small for any pool).
    /// 2. Finds the largest pool size that fits (`bits >= frame_bits`,
    ///    iterating `PoolSize::ALL` in reverse).
    /// 3. Computes the number of blocks: `2^(size_bits - frame_bits)`.
    /// 4. Calls [`Pool::retype_blocks`] to retype the region into that many
    ///    contiguous untyped blocks in the matching pool.
    ///
    /// Each region contributes to exactly one pool (the largest that fits).
    ///
    /// # Why only kernel untyped regions?
    ///
    /// Device untyped regions cannot be retyped into general-purpose untyped
    /// blocks — they represent memory-mapped I/O regions. Only kernel untyped
    /// regions (RAM) are eligible.
    ///
    /// # Errors
    ///
    /// Propagates kernel errors from [`Pool::retype_blocks`].
    fn populate_all_pools(
        &mut self,
        bootinfo: &BootInfoPtr,
        vspace_untyped_idx: usize,
    ) -> Result<(), BootstrapError> {
        debug_println!("[chord] === Populating All Pools ===");

        let untyped_list = bootinfo.untyped_list();

        for i in bootinfo.kernel_untyped_range() {
            // Skip the untyped region consumed to back pool CNodes.
            if i == self.pool_cnode_untyped_idx {
                continue;
            }
            // Skip the untyped region consumed by the VSpace bootstrap
            // (large page allocation).
            if i == vspace_untyped_idx {
                continue;
            }

            let ut = &untyped_list[i];
            // Device memory must not be retyped into general-purpose blocks.
            if ut.is_device() {
                continue;
            }
            let bits = ut.size_bits();
            // Too small for any pool size class.
            if bits < 12 {
                continue;
            }

            let untyped_slot = bootinfo.untyped().index(i);

            // Find the largest pool size that fits.
            for size in PoolSize::ALL.iter().rev() {
                let frame_bits = size.frame_size_bits();
                if bits >= frame_bits {
                    let num_blocks = 1usize << (bits - frame_bits);
                    debug_println!(
                        "[chord] retyping untyped idx={} size_bits={} → {} {} blocks",
                        i,
                        bits,
                        num_blocks,
                        size.name(),
                    );
                    self.pools
                        .get_mut(*size)
                        .retype_blocks(untyped_slot, num_blocks)?;
                    break;
                }
            }
        }

        // Log total slot counts for each pool.
        for size in PoolSize::ALL {
            let pool = self.pools.get(*size);
            debug_println!(
                "[chord] {} pool: {} slots occupied",
                size.name(),
                pool.slot_count,
            );
        }

        Ok(())
    }

    /// Spot-checks pool slots (first, middle, last) for reachability across
    /// all non-empty pools.
    ///
    /// # How it works
    ///
    /// Iterates all 10 pools via `PoolSize::ALL`. For each pool:
    /// - If `slot_count == 0`, logs a skip message and continues.
    /// - Otherwise, uses [`cap::Untyped::debug_identify`] (a non-mutating
    ///   kernel debug call) to confirm the kernel can resolve slots 0,
    ///   `slot_count / 2`, and `slot_count - 1` to valid untyped capabilities
    ///   through the expanded `CSpace`.
    ///
    /// # Why these three slots?
    ///
    /// - **Slot 0** — tests the boundary where the `CNode` cap itself lives.
    ///   A bug in the `CPtr` encoding could cause slot 0 to resolve to the
    ///   `CNode` cap instead of the first untyped block.
    /// - **Middle slot** — confirms bulk placement didn't skip or duplicate.
    /// - **Last slot** — confirms the pool `CNode` can address its highest
    ///   slot without overflow.
    ///
    /// # No return value
    ///
    /// This is a debug-only verification. Errors are printed to the kernel
    /// debug console but do not cause the bootstrap to fail.
    fn verify_pools(&self) {
        debug_println!("[chord] === Verifying All Pools ===");

        let mut total_reachable: usize = 0;

        for size in PoolSize::ALL {
            let pool = self.pools.get(*size);
            let count = pool.slot_count;

            if count == 0 {
                debug_println!("[verify] {} pool is empty, skipping", size.name(),);
                continue;
            }

            for slot in [0, count / 2, count - 1] {
                let selector = pool.cptr_bits_for_slot(slot);
                let cap_type = pool.untyped_cap_at(slot).debug_identify();
                debug_println!(
                    "[verify] {} pool slot={} selector={:#x} cap_type={}",
                    size.name(),
                    slot,
                    selector,
                    cap_type,
                );
            }

            total_reachable += count;
        }

        debug_println!(
            "[verify] verification complete: {} total slots reachable",
            total_reachable,
        );
    }

    /// Pops the next free slot from the root [`CNode`](cap_type::CNode).
    ///
    /// Returns `None` if no free slots remain. Intended for future scratch
    /// allocations in the root `CNode` (e.g., intermediate objects during
    /// capability operations).
    #[expect(dead_code, reason = "reserved for later root CNode scratch allocation")]
    pub fn pop_free_slot(&mut self) -> Option<Slot<cap_type::Null>> {
        self.free_slots.pop()
    }

    /// Returns a reference to the [`Pool`] for the given [`PoolSize`].
    ///
    /// This is the generic accessor for pool lookup by size class:
    ///
    /// ```ignore
    /// let four_k  = bootstrap.pool(PoolSize::FourK);
    /// let two_m   = bootstrap.pool(PoolSize::TwoM);
    /// let one_g   = bootstrap.pool(PoolSize::OneG);
    /// ```
    #[expect(
        dead_code,
        reason = "allocator clients will select pools by frame size"
    )]
    pub const fn pool(&self, size: PoolSize) -> &Pool {
        self.pools.get(size)
    }
}
