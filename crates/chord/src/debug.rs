//! Kernel bootinfo debug dump.
//!
//! [`dump_bootinfo`] prints a human-readable breakdown of the seL4 bootinfo
//! structure to the kernel debug console. This includes IPC buffer location,
//! slot region ranges, untyped index sub-ranges, and a per-entry breakdown of
//! every untyped memory region.

use sel4::{BootInfoPtr, debug_println};

/// Dumps the kernel bootinfo structure to the seL4 debug console.
///
/// # What it prints
///
/// 1. **Root task metadata** — IPC buffer pointer, empty `CSlot` range, user
///    image frames range, untyped `CSlot` range.
/// 2. **Untyped index sub-ranges** — device untyped index range and kernel
///    untyped index range (these are indices into `untyped_list`, not slot
///    indices).
/// 3. **`BootInfo` footprint** — total size of the bootinfo structure in bytes.
/// 4. **Untyped memory list** — one line per entry showing physical address,
///    `size_bits`, human-readable size, and device flag.
///
/// # Human-readable size scaling
///
/// For each untyped entry, the `size_bits` is converted to a human-readable
/// size with appropriate unit (B, KB, MB, GB, TB). The conversion uses
/// [`checked_shl`](usize::checked_shl) to avoid overflow for very large
/// `size_bits` values, falling back to `usize::MAX`.
///
/// # When to call
///
/// Call this early in the root task, before any `CSpace` mutations, to capture
/// the initial kernel-provided state for debugging.
pub fn dump_bootinfo(bootinfo: &BootInfoPtr) {
    debug_println!("========================================");
    debug_println!("        DETAILED BOOTINFO DUMP          ");
    debug_println!("========================================");

    // ---- Root task metadata ----
    // IPC buffer and slot region ranges help verify the initial CSpace layout.
    debug_println!("IPC Buffer Pointer: {:?}", bootinfo.ipc_buffer());
    debug_println!("Empty CSlots Range:        {:?}", bootinfo.empty().range());
    debug_println!(
        "User Image Frames Range:   {:?}",
        bootinfo.user_image_frames().range()
    );
    debug_println!(
        "Untyped CSlots Range:      {:?}",
        bootinfo.untyped().range()
    );

    // ---- Untyped index sub-ranges ----
    // The kernel partitions the untyped list into device and kernel regions.
    // These ranges are indices into `untyped_list`, not slot indices.
    debug_println!(
        "Device Untyped Index Range: {:?}",
        bootinfo.device_untyped_range()
    );
    debug_println!(
        "Kernel Untyped Index Range: {:?}",
        bootinfo.kernel_untyped_range()
    );

    // ---- BootInfo size ----
    debug_println!(
        "BootInfo Footprint Size:    {} bytes",
        bootinfo.footprint_size()
    );
    debug_println!("========================================");

    // ---- Untyped Memory List ----
    // Print every entry with human-readable size scaling.
    debug_println!("--- Untyped Memory List ---");
    for (i, ut) in bootinfo.untyped_list().iter().enumerate() {
        let bits = ut.size_bits();

        // Dynamically scale from bytes up to terabytes based on the bit shift.
        // size_bits is a small u8 from seL4, so the subtraction always fits u32.
        // checked_shl guards against shift overflow for extremely large values.
        #[expect(
            clippy::cast_possible_truncation,
            reason = "size_bits is u8, subtraction fits u32"
        )]
        let (size, unit) = if bits >= 40 {
            (
                1usize.checked_shl((bits - 40) as u32).unwrap_or(usize::MAX),
                "TB",
            )
        } else if bits >= 30 {
            (
                1usize.checked_shl((bits - 30) as u32).unwrap_or(usize::MAX),
                "GB",
            )
        } else if bits >= 20 {
            (
                1usize.checked_shl((bits - 20) as u32).unwrap_or(usize::MAX),
                "MB",
            )
        } else if bits >= 10 {
            (
                1usize.checked_shl((bits - 10) as u32).unwrap_or(usize::MAX),
                "KB",
            )
        } else {
            (1usize << bits, "B")
        };

        debug_println!(
            "  [{:3}] paddr: {:#016x}, size_bits: {:3} ({:3} {}), is_device: {}",
            i,
            ut.paddr(),
            ut.size_bits(),
            size,
            unit,
            ut.is_device()
        );
    }
    debug_println!("========================================");
}
