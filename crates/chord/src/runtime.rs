//! Bare-metal runtime for the seL4 root task.
//!
//! Replaces `sel4-root-task` with minimal, no-magic startup code:
//!
//! 1. [`__chord_entry`] — initializes the IPC buffer, then calls [`crate::main`].
//! 2. Bump allocator — simple `#[global_allocator]` over a static 5 MiB heap.
//! 3. Panic handler — aborts immediately.
//! 4. [`image_end`] — returns the first safe virtual address past the ELF image.
//!
//! Architecture-specific startup assembly lives in [`arch`](crate::arch).

use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const HEAP_SIZE: usize = 5 * 1024 * 1024;

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
    let ipc_buffer = unsafe { bootinfo.ipc_buffer().as_mut().unwrap() };
    sel4::set_ipc_buffer(ipc_buffer);

    crate::main(&bootinfo)
}

// ---------------------------------------------------------------------------
// Bump allocator
// ---------------------------------------------------------------------------

/// A simple bump allocator that never frees.
///
/// Allocates sequentially from a static heap. `dealloc` is a no-op — memory is
/// reclaimed only when the heap pointer is reset (which we never do in this
/// `PoC`). This is sufficient for a single-threaded root task that only grows
/// its data structures.
///
/// # Why not a real allocator?
///
/// For a `PoC` learning project, the bump allocator is the simplest correct
/// choice. It has zero fragmentation, zero overhead, and ~30 lines. If we
/// later need proper deallocation (e.g., dynamic process spawning with
/// cleanup), we can swap in `linked_list_allocator` or `sel4-dlmalloc`.
struct BumpAllocator {
    heap: *mut u8,
    size: usize,
    pos: AtomicUsize,
}

// SAFETY: Single-threaded root task, no concurrent access.
unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pos = self.pos.load(Ordering::Relaxed);
        let heap_start = self.heap as usize;

        let aligned = (heap_start + pos).next_multiple_of(layout.align());
        let next = aligned + layout.size();
        let heap_end = heap_start + self.size;

        if next > heap_end {
            return core::ptr::null_mut();
        }

        self.pos.store(next - heap_start, Ordering::Relaxed);
        aligned as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

/// Static heap memory in BSS.
///
/// Treated as a zero-initialized memory region, not as actual data.
/// Placed in `.bss` explicitly so the kernel maps it writable.
#[unsafe(link_section = ".bss")]
static HEAP_STORAGE: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator {
    heap: core::ptr::addr_of!(HEAP_STORAGE) as *mut u8,
    size: HEAP_SIZE,
    pos: AtomicUsize::new(0),
};

// ---------------------------------------------------------------------------
// Panic handler
// ---------------------------------------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    abort()
}

#[cold]
#[inline(never)]
fn abort() -> ! {
    unsafe {
        core::arch::asm!("ud2", options(noreturn, nomem, nostack));
    }
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
#[expect(dead_code, reason = "used by VSpace module once implemented")]
pub fn image_end() -> usize {
    unsafe extern "C" {
        static _end: u8;
    }
    (core::ptr::addr_of!(_end) as usize).next_multiple_of(4096)
}
