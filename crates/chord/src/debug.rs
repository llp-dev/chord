use sel4::{BootInfoPtr, debug_println};

pub fn dump_bootinfo(bootinfo: &BootInfoPtr) {
    debug_println!("========================================");
    debug_println!("        DETAILED BOOTINFO DUMP          ");
    debug_println!("========================================");

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

    debug_println!(
        "Device Untyped Index Range: {:?}",
        bootinfo.device_untyped_range()
    );
    debug_println!(
        "Kernel Untyped Index Range: {:?}",
        bootinfo.kernel_untyped_range()
    );

    debug_println!(
        "BootInfo Footprint Size:    {} bytes",
        bootinfo.footprint_size()
    );
    debug_println!("========================================");
    debug_println!("--- Untyped Memory List ---");
    for (i, ut) in bootinfo.untyped_list().iter().enumerate() {
        let bits = ut.size_bits();

        let (size, unit) = if bits >= 40 {
            (1usize << (bits - 40), "TB")
        } else if bits >= 30 {
            (1usize << (bits - 30), "GB")
        } else if bits >= 20 {
            (1usize << (bits - 20), "MB")
        } else if bits >= 10 {
            (1usize << (bits - 10), "KB")
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
