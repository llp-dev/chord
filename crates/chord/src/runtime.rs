//! Bare-metal runtime for the seL4 root task.
//!
//! Replaces `sel4-root-task` with minimal, no-magic startup code:
//!
//! 1. [`__chord_entry`] — initializes the IPC buffer, then calls [`crate::main`].
//! 2. talc allocator — `#[global_allocator]` backed by talc (dlmalloc-style),
//!    initialized at runtime via [`init_heap`].
//! 3. Panic handler — aborts immediately.
//! 4. [`image_end`] — returns the first safe virtual address past the ELF image.
//!
//! Architecture-specific startup assembly lives in [`arch`](crate::arch).

use core::panic::PanicInfo;

use sel4::debug_println;
use talc::{TalcLock, source::Manual};

// ---------------------------------------------------------------------------
// Rust entry point — called by arch-specific _start with bootinfo in rdi
// ---------------------------------------------------------------------------

/// Rust entry point called by `_start`.
///
/// # Safety
///
/// This function may only be called from the `_start` assembly entry point.
/// The `bootinfo` pointer must be a valid kernel-provided `BootInfo` address.
#[unsafe(no_mangle)]
unsafe extern "C" fn __chord_entry(bootinfo: *const sel4::BootInfo) -> ! {
    let bootinfo = unsafe { sel4::BootInfoPtr::new(bootinfo) };

    // The IPC buffer pointer is the virtual address where the kernel mapped
    // the root task's IPC buffer page. All seL4 syscalls use this buffer
    // to send/receive message registers.
    //
    // SAFETY: The kernel maps the IPC buffer page for the entire lifetime of
    // the root task, so the `'static` lifetime required by `set_ipc_buffer`
    // is justified. The pointer is guaranteed non-null by the seL4 boot
    // protocol — a null pointer would indicate a kernel bug.
    let ipc_buffer =
        unsafe { bootinfo.ipc_buffer().as_mut() }.expect("IPC buffer pointer was null");
    sel4::set_ipc_buffer(ipc_buffer);

    crate::main(&bootinfo)
}

// ---------------------------------------------------------------------------
// talc allocator
// ---------------------------------------------------------------------------

/// Global allocator backed by talc (a dlmalloc-style allocator).
///
/// Uses `spinning_top::RawSpinlock` for thread safety (single-core,
/// no contention in practice). The `Manual` source means the heap
/// region is provided at runtime via [`init_heap`].
///
/// Initialized by [`init_heap`] before any allocation occurs.
#[global_allocator]
static ALLOCATOR: TalcLock<spinning_top::RawSpinlock, Manual> = TalcLock::new(Manual);

/// Initializes the global allocator with an exclusive memory region.
///
/// # Safety
///
/// - Must be called **once**, before any allocation (`Box`, `Vec`,
///   `String`, `format!`, etc.).
/// - The region `[base, base+size)` must be valid, writable, and
///   exclusively owned by the allocator for the remainder of the program.
/// - No other code may read from or write to this region after the call.
///
/// # Panics
///
/// Panics if the region is too small for talc's internal metadata
/// (typically a few hundred bytes on `x86_64`) or is misaligned.
pub unsafe fn init_heap(base: *mut u8, size: usize) {
    // SAFETY: Caller guarantees the region is valid, writable, and exclusively
    // owned by the allocator. claim() requires the same invariants.
    unsafe {
        let result = ALLOCATOR.lock().claim(base, size);
        if result.is_none() {
            debug_println!(
                "[chord] ERROR: talc heap claim failed — region too small or misaligned"
            );
        }
        result.expect("talc heap claim failed — region too small or misaligned");
    }
}

// ---------------------------------------------------------------------------
// Panic handler
// ---------------------------------------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    crate::arch::abort()
}

// ---------------------------------------------------------------------------
// Linker symbol access
// ---------------------------------------------------------------------------

/// Returns the first virtual address past the root task's loaded ELF image.
///
/// Uses the default linker's `_end` symbol, then rounds it up to the next
/// 4 KiB page. This avoids requiring a project-owned linker script while still
/// giving later `VSpace` code a conservative first candidate page for new
/// mappings.
///
/// # Use case
///
/// When mapping new pages into the root task's `VSpace`, start at this address
/// to avoid conflicting with the existing image mapping.
///
/// # Example
///
/// ```ignore
/// let vaddr = image_end();
/// // vaddr is safe to use as the target of frame_map(...)
/// ```
pub fn image_end() -> usize {
    unsafe extern "C" {
        static _end: u8;
    }
    (core::ptr::addr_of!(_end) as usize).next_multiple_of(4096)
}
