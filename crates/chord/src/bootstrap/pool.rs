//! Capability pool types — [`PoolSize`] and [`Pool`].
//!
//! A [`Pool`] is a capability pool backed by a dedicated
//! [`CNode`](sel4::cap_type::CNode). Each pool holds untyped capabilities of a
//! specific size class (4 KiB, 2 MiB, or 1 GiB), organized contiguously inside
//! the pool's `CNode`.
//!
//! # Pool sizes
//!
//! | [`PoolSize`] | Frame size | `size_bits` | Typical use |
//! |-------------|-----------|------------|-------------|
//! | `Small`     | 4 KiB     | 12         | General-purpose 4K pages |
//! | `Large`     | 2 MiB     | 21         | Large-page mappings |
//! | `Huge`      | 1 GiB     | 30         | Huge-page mappings |
//!
//! # `CPtr` encoding
//!
//! After [`expand_current_cspace`](super::cspace::expand_current_cspace) has
//! configured the `CSpace` guard, pool slots are addressed by a two-level
//! `CPtr`:
//!
//! ```text
//! CPtr = (cnode_slot.index << cnode_radix) | internal_slot
//! ```
//!
//! See [`Pool::cptr_bits_for_slot`] for details.

use sel4::{CPtr, CPtrBits, ObjectBlueprint, cap, cap_type, debug_println, init_thread::Slot};

/// Frame size category for memory pools.
///
/// Each variant corresponds to a fixed power-of-two frame size used as the
/// allocation unit for that pool. The seL4 kernel represents all object sizes
/// as `2^size_bits` bytes, so the frame size is encoded as a `size_bits` value.
///
/// # Why these sizes?
///
/// - **Small (4 KiB)** — the standard page size on `x86_64` and most
///   architectures. Most frame allocations will use this size.
/// - **Large (2 MiB)** — a "superpage" on `x86_64`, used for large contiguous
///   mappings (e.g., framebuffers, large buffers).
/// - **Huge (1 GiB)** — a "giant page" on `x86_64`, used for very large
///   mappings (e.g., entire address spaces, huge DMA regions).
///
/// # Examples
///
/// ```ignore
/// assert_eq!(PoolSize::Small.frame_size_bits(), 12);
/// assert_eq!(PoolSize::Large.frame_size_bits(), 21);
/// assert_eq!(PoolSize::Huge.frame_size_bits(), 30);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PoolSize {
    /// 4 KiB frames (`size_bits = 12`). Standard page size.
    Small,
    /// 2 MiB frames (`size_bits = 21`). `x86_64` superpage.
    Large,
    /// 1 GiB frames (`size_bits = 30`). `x86_64` giant page.
    Huge,
}

impl PoolSize {
    /// Returns the log2 frame size (in bits) for this pool.
    ///
    /// This is the `size_bits` value passed to seL4 kernel operations that
    /// create or retype untyped objects. For example, `size_bits = 12` means
    /// the object is `2^12 = 4096` bytes.
    ///
    /// | Variant    | Return | Size     |
    /// |-----------|--------|----------|
    /// | `Small`   | 12     | 4 KiB    |
    /// | `Large`   | 21     | 2 MiB    |
    /// | `Huge`    | 30     | 1 GiB    |
    pub const fn frame_size_bits(self) -> usize {
        match self {
            Self::Small => 12,
            Self::Large => 21,
            Self::Huge => 30,
        }
    }

    /// Returns a human-readable label for this pool size, used in debug
    /// output.
    ///
    /// | Variant  | Label   |
    /// |---------|---------|
    /// | `Small` | "small" |
    /// | `Large` | "large" |
    /// | `Huge`  | "huge"  |
    pub const fn name(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Large => "large",
            Self::Huge => "huge",
        }
    }
}

/// A capability pool backed by a dedicated [`CNode`](cap_type::CNode).
///
/// A `Pool` owns a `CNode` capability that was retyped from a shared 2 MiB
/// backing untyped. The `CNode` has `2^cnode_radix` slots, each of which can
/// hold one capability. Pool slots are populated sequentially by
/// [`retype_frames`](Self::retype_frames).
///
/// # Field overview
///
/// - [`size`](Self::size) — which frame size class this pool serves.
/// - [`cnode_slot`](Self::cnode_slot) — where the pool's `CNode` cap lives
///   in the root `CNode`.
/// - [`cnode_radix`](Self::cnode_radix) — log2 of the pool `CNode`'s slot
///   count.
/// - [`frame_count`](Self::frame_count) — how many slots are currently
///   occupied.
///
/// # `CPtr` addressing
///
/// After the `CSpace` guard has been configured, pool slots are addressed by
/// a two-level `CPtr` encoding (see [`cptr_bits_for_slot`](Self::cptr_bits_for_slot)):
///
/// ```text
/// CPtr = (cnode_slot.index << cnode_radix) | internal_slot
/// ```
///
/// All three pools share the same `cnode_radix` (the
/// [`pool_slot_radix`](super::Bootstrap::pool_slot_radix)) so that slot
/// indices are uniform across pools.
#[derive(Debug, Copy, Clone)]
pub struct Pool {
    /// The frame-size category for this pool.
    ///
    /// Determines the `size_bits` passed to `untyped_retype` when populating
    /// the pool. See [`PoolSize`] for the mapping between variants and byte
    /// sizes.
    pub size: PoolSize,

    /// The root [`CNode`](cap_type::CNode) slot that holds this pool's
    /// [`CNode`](cap_type::CNode) capability.
    ///
    /// After [`expand_current_cspace`](super::cspace::expand_current_cspace),
    /// this slot index becomes the "root slot" portion of the two-level `CPtr`
    /// encoding for all capabilities in this pool.
    pub cnode_slot: Slot<cap_type::CNode>,

    /// Radix (log2 capacity) of this pool's [`CNode`](cap_type::CNode).
    ///
    /// The pool `CNode` has `2^cnode_radix` total slots. This value is the
    /// same for all three pools (Small, Large, Huge) and equals
    /// [`Bootstrap::pool_slot_radix`](super::Bootstrap::pool_slot_radix).
    pub cnode_radix: usize,

    /// Number of slots currently occupied by retyped untyped blocks.
    ///
    /// This is a monotonically increasing watermark: each call to
    /// [`retype_frames`](Self::retype_frames) advances it by the number of
    /// frames retyped. It doubles as the index of the next free slot.
    pub frame_count: usize,
}

impl Pool {
    /// Creates a new pool [`CNode`](cap_type::CNode) from the backing untyped
    /// and an empty slot.
    ///
    /// # What it does
    ///
    /// 1. Takes the shared backing untyped (`pool_cnode_untyped`) and retypes
    ///    a portion of it into a [`CNode`](cap_type::CNode) with
    ///    `2^cnode_radix` slots.
    /// 2. Places the resulting `CNode` capability at `cnode_slot` in the root
    ///    `CNode`.
    /// 3. Returns a [`Pool`] with `frame_count = 0` (empty pool).
    ///
    /// # Parameters
    ///
    /// - `root_cnode` — the root `CNode` capability slot, used to compute
    ///   the absolute `CPtr` for the retype destination.
    /// - `pool_cnode_untyped` — the 2 MiB untyped region that backs all three
    ///   pool `CNode`s. Each call to `new` consumes a portion of this region.
    /// - `size` — which frame size class this pool serves.
    /// - `cnode_slot` — an empty slot in the root `CNode` where the new
    ///   `CNode` cap will be placed.
    /// - `cnode_radix` — log2 of the number of slots in the new `CNode`.
    ///
    /// # Errors
    ///
    /// Returns [`sel4::Error`] if the kernel rejects the retype (e.g.,
    /// insufficient untyped size, invalid slot, or insufficient rights).
    ///
    /// # Kernel call
    ///
    /// Invokes `seL4_Untyped_Retype` with `ObjectBlueprint::CNode {
    /// size_bits }`, placing one `CNode` capability at the specified slot.
    pub fn new(
        root_cnode: Slot<cap_type::CNode>,
        pool_cnode_untyped: Slot<cap_type::Untyped>,
        size: PoolSize,
        cnode_slot: Slot<cap_type::Null>,
        cnode_radix: usize,
    ) -> sel4::Result<Self> {
        debug_println!(
            "[chord] creating {} pool CNode: frame_size_bits={} radix={} slot={}",
            size.name(),
            size.frame_size_bits(),
            cnode_radix,
            cnode_slot.index()
        );

        // Compute the absolute CPtr for the root CNode, which is where
        // the new CNode capability will be placed.
        let dst = root_cnode.cap().absolute_cptr_for_self();

        // Retype a portion of the backing untyped into a CNode.
        // The kernel allocates 2^cnode_radix * sizeof(seL4_CNodeSlot) bytes
        // from the untyped region.
        pool_cnode_untyped.cap().untyped_retype(
            &ObjectBlueprint::CNode {
                size_bits: cnode_radix,
            },
            &dst,
            cnode_slot.index(),
            1,
        )?;

        // After retype, the slot now holds a CNode cap instead of Null.
        // Cast the type marker accordingly.
        Ok(Self {
            size,
            cnode_slot: cnode_slot.cast::<cap_type::CNode>(),
            cnode_radix,
            frame_count: 0,
        })
    }

    /// Retypes `num_frames` untyped blocks from `untyped_slot` into this pool.
    ///
    /// # What it does
    ///
    /// Takes a source untyped capability and splits it into `num_frames`
    /// contiguous untyped blocks, each of this pool's native frame size
    /// (`size_bits = size.frame_size_bits()`). The blocks are placed
    /// sequentially starting at the current [`frame_count`](Self::frame_count)
    /// offset inside the pool `CNode`.
    ///
    /// After the call succeeds, `frame_count` is advanced by `num_frames`.
    ///
    /// # Example
    ///
    /// For a Small pool (`size_bits = 12`), retyping a 1 MiB untyped
    /// (`size_bits = 20`) produces `2^(20-12) = 256` blocks of 4 KiB each:
    ///
    /// ```ignore
    /// pool.retype_frames(untyped_slot, 256)?;
    /// assert_eq!(pool.frame_count, 256);
    /// ```
    ///
    /// # Parameters
    ///
    /// - `untyped_slot` — the source untyped capability to split. Must be in
    ///   the root `CNode` (the pool `CNode` cannot be used as a source).
    /// - `num_frames` — how many blocks to create. Must be > 0 (no-op if 0).
    ///
    /// # Errors
    ///
    /// Returns [`sel4::Error`] if the kernel rejects the retype. Common
    /// reasons:
    /// - Insufficient untyped size (source too small for `num_frames` blocks).
    /// - Destination [`CNode`](cap_type::CNode) full (would overflow the pool
    ///   slot range).
    /// - Source is a device untyped (cannot be retyped into general-purpose
    ///   objects).
    pub fn retype_frames(
        &mut self,
        untyped_slot: Slot<cap_type::Untyped>,
        num_frames: usize,
    ) -> sel4::Result<()> {
        if num_frames == 0 {
            return Ok(());
        }

        // Blueprint specifies the target object type and size.
        // Untyped { size_bits } creates a new untyped block of that size.
        // For the Small pool, size_bits=12 → 4 KiB blocks.
        let bp = ObjectBlueprint::Untyped {
            size_bits: self.size.frame_size_bits(),
        };

        // Compute the absolute CPtr for this pool's CNode, which is the
        // destination for the retype operation.
        let dst = self.cnode_slot.cap().absolute_cptr_for_self();

        // Retype: the kernel consumes num_frames * 2^size_bits bytes from
        // the source untyped and places the resulting capabilities contiguously
        // starting at offset frame_count in the pool CNode.
        untyped_slot
            .cap()
            .untyped_retype(&bp, &dst, self.frame_count, num_frames)?;

        debug_println!(
            "[chord] {} pool: retyped {} untyped blocks (size_bits={}) → slots {}-{}",
            self.size.name(),
            num_frames,
            self.size.frame_size_bits(),
            self.frame_count,
            self.frame_count + num_frames - 1,
        );

        // Advance the occupancy watermark so the next retype starts after
        // the blocks we just placed.
        self.frame_count += num_frames;
        Ok(())
    }

    /// Returns the raw [`CPtr`] bits for `slot` within this pool's
    /// [`CNode`](cap_type::CNode).
    ///
    /// # Encoding
    ///
    /// The two-level `CPtr` format (after the `CSpace` guard has been
    /// configured by [`expand_current_cspace`](super::cspace::expand_current_cspace)):
    ///
    /// ```text
    /// CPtr = (cnode_slot.index << cnode_radix) | slot
    ///        ────────────────────────────────   ────
    ///        root CNode slot selector            pool-internal slot index
    /// ```
    ///
    /// - **High bits** (`cnode_slot.index << cnode_radix`) — selects which
    ///   pool `CNode` in the root `CNode`.
    /// - **Low bits** (`slot`) — selects the internal slot within that pool
    ///   `CNode`.
    ///
    /// # Parameters
    ///
    /// - `slot` — the internal slot index within the pool `CNode` (0-based).
    ///
    /// # Note
    ///
    /// This encoding only produces correct `CPtr`s after the `CSpace` guard
    /// has been set. Before that, `CPtr` resolution follows the kernel's
    /// default single-level scheme.
    pub const fn cptr_bits_for_slot(&self, slot: usize) -> CPtrBits {
        ((self.cnode_slot.index() << self.cnode_radix) | slot) as CPtrBits
    }

    /// Returns a typed [`Untyped`](cap::Untyped) capability for the given pool
    /// slot.
    ///
    /// # How it works
    ///
    /// This is a lightweight handle constructed purely from `CPtr` bits — no
    /// kernel invocation is performed. The capability is only valid if the
    /// slot actually contains an untyped object (i.e., it has been populated
    /// by [`retype_frames`](Self::retype_frames)).
    ///
    /// # Use case
    ///
    /// Used by [`verify_small_pool`](super::Bootstrap::verify_small_pool) to
    /// call `debug_identify` on pool slots for reachability testing.
    pub const fn untyped_cap_at(&self, slot: usize) -> cap::Untyped {
        CPtr::from_bits(self.cptr_bits_for_slot(slot)).cast()
    }
}
