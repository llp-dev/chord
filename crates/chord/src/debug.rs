//! Kernel bootinfo debug dump.
//!
//! [`dump_bootinfo`] prints a human-readable breakdown of the seL4 bootinfo
//! structure to the kernel debug console. This includes IPC buffer location,
//! slot region ranges, untyped index sub-ranges, and a per-entry breakdown of
//! every untyped memory region.

use sel4::{BootInfoPtr, debug_println};

/// Threshold in `size_bits` for kilobyte scaling (2^10 = 1024 bytes).
const KB_THRESHOLD: u8 = 10;
/// Threshold in `size_bits` for megabyte scaling (2^20 = 1 MiB).
const MB_THRESHOLD: u8 = 20;
/// Threshold in `size_bits` for gigabyte scaling (2^30 = 1 GiB).
const GB_THRESHOLD: u8 = 30;
/// Threshold in `size_bits` for terabyte scaling (2^40 = 1 TiB).
const TB_THRESHOLD: u8 = 40;

/// Converts a `size_bits` value to a human-readable size and unit.
///
/// Scales from bytes up to terabytes based on the bit shift:
///
/// | `bits` range | Unit | Calculation            |
/// |-------------|------|------------------------|
/// | `≥ 40`      | TB   | `2^(bits - 40)`        |
/// | `≥ 30`      | GB   | `2^(bits - 30)`        |
/// | `≥ 20`      | MB   | `2^(bits - 20)`        |
/// | `≥ 10`      | KB   | `2^(bits - 10)`        |
/// | `< 10`      | B    | `2^bits`               |
///
/// Uses [`checked_shl`](usize::checked_shl) uniformly across all branches
/// to guard against shift overflow for extremely large `size_bits` values,
/// falling back to `usize::MAX`. The `< 10` branch uses `checked_shl` for
/// consistency, not overflow safety (shifts < 10 cannot overflow `usize`).
#[expect(
    clippy::cast_possible_truncation,
    reason = "size_bits from seL4 is always small (< 48), fits u32"
)]
fn human_size(bits: usize) -> (usize, &'static str) {
    if bits >= TB_THRESHOLD as usize {
        (
            1usize
                .checked_shl((bits - TB_THRESHOLD as usize) as u32)
                .unwrap_or(usize::MAX),
            "TB",
        )
    } else if bits >= GB_THRESHOLD as usize {
        (
            1usize
                .checked_shl((bits - GB_THRESHOLD as usize) as u32)
                .unwrap_or(usize::MAX),
            "GB",
        )
    } else if bits >= MB_THRESHOLD as usize {
        (
            1usize
                .checked_shl((bits - MB_THRESHOLD as usize) as u32)
                .unwrap_or(usize::MAX),
            "MB",
        )
    } else if bits >= KB_THRESHOLD as usize {
        (
            1usize
                .checked_shl((bits - KB_THRESHOLD as usize) as u32)
                .unwrap_or(usize::MAX),
            "KB",
        )
    } else {
        // Consistency: checked_shl used here even though bits < 10
        // cannot overflow usize on any platform.
        (1usize.checked_shl(bits as u32).unwrap_or(usize::MAX), "B")
    }
}

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
        let (size, unit) = human_size(ut.size_bits());

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
