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
use super::pool::{Pool, PoolSize};

/// The root-task bootstrap state.
///
/// Holds the root [`CNode`](cap_type::CNode), all three pool
/// [`CNodes`](Pool) (Small / Large / Huge), and the list of remaining free
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
/// | `small_pool`              | Pool for 4 KiB untyped blocks |
/// | `large_pool`              | Pool for 2 MiB untyped blocks |
/// | `huge_pool`               | Pool for 1 GiB untyped blocks |
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
    /// All three pool `CNode`s are created with the same radix so that the
    /// `CPtr` encoding is uniform. This is the maximum of the three
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

    /// The untyped slot used to create the three pool `CNode`s.
    ///
    /// Kept for future allocator accounting — the frame allocator may need to
    /// know which untyped region backs the pool `CNode`s.
    #[expect(dead_code, reason = "kept for future bootstrap allocator accounting")]
    pub pool_cnode_untyped: Slot<cap_type::Untyped>,

    /// Remaining empty slots in the root [`CNode`](cap_type::CNode).
    ///
    /// After three slots are consumed for the pool `CNode`s, the rest are
    /// available for future root `CNode` scratch allocations.
    #[expect(dead_code, reason = "reserved for later root CNode scratch allocation")]
    pub free_slots: Vec<Slot<cap_type::Null>>,

    /// Pool for 4 KiB untyped blocks (`size_bits = 12`).
    ///
    /// Populated during bootstrap by
    /// [`populate_small_pool`](Self::populate_small_pool).
    pub small_pool: Pool,

    /// Pool for 2 MiB untyped blocks (`size_bits = 21`).
    ///
    /// Not yet populated — will be filled by the frame allocator.
    #[expect(
        dead_code,
        reason = "large pool will be populated by the frame allocator"
    )]
    pub large_pool: Pool,

    /// Pool for 1 GiB untyped blocks (`size_bits = 30`).
    ///
    /// Not yet populated — will be filled by the frame allocator.
    #[expect(
        dead_code,
        reason = "huge pool will be populated by the frame allocator"
    )]
    pub huge_pool: Pool,
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
    ///    list and count how many 4 KiB, 2 MiB, and 1 GiB blocks can be
    ///    produced from non-device regions.
    /// 2. **Compute pool radix** — derive the `CNode` radix for each pool from
    ///    the slot counts, then take the maximum so all pools use the same
    ///    radix.
    /// 3. **Find backing untyped** — locate the first 2 MiB non-device kernel
    ///    untyped region to back the three pool `CNode`s.
    /// 4. **Create pool `CNode`s** — retype the backing untyped into three
    ///    `CNode` capabilities (Small / Large / Huge).
    /// 5. **Populate Small pool** — retype kernel untyped regions smaller than
    ///    2 MiB into 4 KiB untyped blocks inside the Small pool.
    /// 6. **Expand `CSpace`** — configure a guard on the root `CNode` so pool
    ///    slots are directly addressable via `CPtr`.
    /// 7. **Verify pool** — spot-check three Small pool slots for
    ///    reachability.
    ///
    /// # Errors
    ///
    /// Returns [`BootstrapError`] if:
    /// - No suitable 2 MiB kernel untyped is found
    ///   ([`NoBackingUntyped`](BootstrapError::NoBackingUntyped)).
    /// - Fewer than 3 empty `CSlot`s remain in the root `CNode`
    ///   ([`NoEmptyCSlots`](BootstrapError::NoEmptyCSlots)).
    /// - A kernel operation fails
    ///   ([`Sel4`](BootstrapError::Sel4)).
    pub fn new(bootinfo: &BootInfoPtr) -> Result<Self, BootstrapError> {
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

        bootstrap.populate_small_pool(bootinfo)?;
        cspace::expand_current_cspace(&bootstrap)?;
        bootstrap.verify_small_pool();

        Ok(bootstrap)
    }

    /// Computes the shared pool radix from the untyped descriptor list.
    ///
    /// # How it works
    ///
    /// For each non-device untyped region, determines how many blocks it can
    /// produce for each pool size class:
    ///
    /// - `size_bits ≥ 30` → Huge pool: `2^(bits - 30)` blocks
    /// - `21 ≤ size_bits ≤ 29` → Large pool: `2^(bits - 21)` blocks
    /// - `12 ≤ size_bits ≤ 20` → Small pool: `2^(bits - 12)` blocks
    /// - `size_bits < 12` → too small for any pool; skipped
    ///
    /// Then computes `radix = ceil(log2(total_slots))` for each pool, and
    /// returns the maximum (so all pools use the same radix).
    fn derive_pool_radix(untyped_list: &[sel4::UntypedDesc]) -> usize {
        let mut small_slots: usize = 0;
        let mut large_slots: usize = 0;
        let mut huge_slots: usize = 0;

        for ut_desc in untyped_list.iter().filter(|ut_desc| !ut_desc.is_device()) {
            let bits = ut_desc.size_bits();
            match bits {
                30.. => huge_slots += 1 << (bits - PoolSize::Huge.frame_size_bits()),
                21..=29 => large_slots += 1 << (bits - PoolSize::Large.frame_size_bits()),
                12..=20 => small_slots += 1 << (bits - PoolSize::Small.frame_size_bits()),
                _ => {}
            }
        }

        // next_power_of_two rounds up to the nearest power of 2.
        // ilog2 gives log2 of that power, which is the minimum radix.
        // .max(1) ensures at least 2 slots even if no frames were counted.
        let small_radix = (small_slots.next_power_of_two().ilog2() as usize).max(1);
        let large_radix = (large_slots.next_power_of_two().ilog2() as usize).max(1);
        let huge_radix = (huge_slots.next_power_of_two().ilog2() as usize).max(1);

        small_radix.max(large_radix).max(huge_radix)
    }

    /// Searches the kernel untyped range for a 2 MiB non-device region.
    ///
    /// # Selection criteria
    ///
    /// - Must be in the kernel untyped range (not the device range).
    /// - Must have `size_bits == 21` (exactly 2 MiB).
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
                !ud.is_device() && ud.size_bits() == 21
            })
            .ok_or(BootstrapError::NoBackingUntyped)
    }

    /// Retypes the backing untyped into three pool [`CNode`](cap_type::CNode)s
    /// and assembles the initial `Bootstrap` struct.
    ///
    /// # Steps
    ///
    /// 1. Collect all empty root `CNode` slots (newest-first, so we consume
    ///    from the end of the range).
    /// 2. Pop three empty slots for the Small, Large, and Huge pool `CNode`s.
    /// 3. Call [`Pool::new`] three times to retype the backing untyped into
    ///    `CNode` capabilities.
    /// 4. Assemble the `Bootstrap` struct with all fields.
    ///
    /// # Errors
    ///
    /// Returns [`BootstrapError::NoEmptyCSlots`] if fewer than 3 empty slots
    /// remain, or propagates kernel errors from [`Pool::new`].
    fn create_pool_cnodes(
        root_cnode: Slot<cap_type::CNode>,
        root_cnode_radix: usize,
        pool_slot_radix: usize,
        pool_cnode_untyped_idx: usize,
        pool_cnode_untyped: Slot<cap_type::Untyped>,
        empty_slots: &SlotRegion<cap_type::Null>,
    ) -> Result<Self, BootstrapError> {
        // Collect empty slots newest-first so we consume from the end
        // of the range, keeping lower slots for other kernel objects.
        let mut free_slots: Vec<Slot<cap_type::Null>> =
            empty_slots.range().rev().map(Slot::from_index).collect();

        let small_cnode_slot = free_slots.pop().ok_or(BootstrapError::NoEmptyCSlots)?;
        let large_cnode_slot = free_slots.pop().ok_or(BootstrapError::NoEmptyCSlots)?;
        let huge_cnode_slot = free_slots.pop().ok_or(BootstrapError::NoEmptyCSlots)?;

        Ok(Self {
            root_cnode,
            root_cnode_radix,
            pool_slot_radix,
            pool_cnode_untyped_idx,
            pool_cnode_untyped,
            free_slots,
            small_pool: Pool::new(
                root_cnode,
                pool_cnode_untyped,
                PoolSize::Small,
                small_cnode_slot,
                pool_slot_radix,
            )?,
            large_pool: Pool::new(
                root_cnode,
                pool_cnode_untyped,
                PoolSize::Large,
                large_cnode_slot,
                pool_slot_radix,
            )?,
            huge_pool: Pool::new(
                root_cnode,
                pool_cnode_untyped,
                PoolSize::Huge,
                huge_cnode_slot,
                pool_slot_radix,
            )?,
        })
    }

    /// Populates the Small pool with 4 KiB untyped blocks.
    ///
    /// # How it works
    ///
    /// Iterates every kernel untyped region (not device regions) whose
    /// `size_bits` is in the range `[12, 21)` — that is, regions between
    /// 4 KiB (inclusive) and 2 MiB (exclusive). For each such region:
    ///
    /// 1. Computes the number of 4 KiB blocks: `2^(size_bits - 12)`.
    /// 2. Calls [`Pool::retype_frames`] to retype the region into that many
    ///    contiguous untyped blocks in the Small pool `CNode`.
    ///
    /// # Why only kernel untyped regions?
    ///
    /// Device untyped regions cannot be retyped into general-purpose untyped
    /// blocks — they represent memory-mapped I/O regions. Only kernel untyped
    /// regions (RAM) are eligible.
    ///
    /// # Errors
    ///
    /// Propagates kernel errors from [`Pool::retype_frames`].
    fn populate_small_pool(&mut self, bootinfo: &BootInfoPtr) -> Result<(), BootstrapError> {
        debug_println!("[chord] === Populating Small Pool ===");

        let untyped_list = bootinfo.untyped_list();

        for i in bootinfo.kernel_untyped_range() {
            let ut = &untyped_list[i];
            // Device memory must not be retyped into general-purpose blocks.
            if ut.is_device() {
                continue;
            }
            let bits = ut.size_bits();
            // Only regions >= 4 KiB and < 2 MiB contribute to the Small pool.
            if !(12..21).contains(&bits) {
                continue;
            }

            // Number of 4 KiB blocks this region produces.
            let num_frames = 1usize << (bits - 12);
            let untyped_slot = bootinfo.untyped().index(i);
            debug_println!(
                "[chord] retyping untyped idx={} size_bits={} → {} 4K untyped blocks",
                i,
                bits,
                num_frames,
            );
            self.small_pool.retype_frames(untyped_slot, num_frames)?;
        }

        debug_println!(
            "[chord] small pool populated: {} slots occupied",
            self.small_pool.frame_count,
        );

        Ok(())
    }

    /// Spot-checks three pool slots (first, middle, last) for reachability.
    ///
    /// # How it works
    ///
    /// Uses [`cap::Untyped::debug_identify`] (a non-mutating kernel debug
    /// call) to confirm the kernel can resolve each slot to a valid untyped
    /// capability through the expanded `CSpace`.
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
    fn verify_small_pool(&self) {
        debug_println!("[chord] === Verifying Small Pool ===");

        let total = self.small_pool.frame_count;

        if total == 0 {
            debug_println!("[verify] pool is empty, nothing to verify");
            return;
        }

        for slot in [0, total / 2, total - 1] {
            let selector = self.small_pool.cptr_bits_for_slot(slot);
            let cap_type = self.small_pool.untyped_cap_at(slot).debug_identify();
            debug_println!(
                "[verify] small pool slot={} selector={:#x} cap_type={}",
                slot,
                selector,
                cap_type,
            );
        }

        debug_println!("[verify] verification complete: {} slots reachable", total);
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
    /// let small = bootstrap.pool(PoolSize::Small);
    /// let large = bootstrap.pool(PoolSize::Large);
    /// let huge  = bootstrap.pool(PoolSize::Huge);
    /// ```
    #[expect(
        dead_code,
        reason = "allocator clients will select pools by frame size"
    )]
    pub const fn pool(&self, size: PoolSize) -> &Pool {
        match size {
            PoolSize::Small => &self.small_pool,
            PoolSize::Large => &self.large_pool,
            PoolSize::Huge => &self.huge_pool,
        }
    }
}
