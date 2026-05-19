//! Architecture-specific abort for `x86_64`.
//!
//! Provides [`abort_now`] which executes the `ud2` instruction to
//! trigger an invalid-opcode exception, halting the CPU.

/// Halts the CPU immediately via `ud2` (undefined instruction trap).
///
/// This function never returns. Safe to call from any context.
#[cold]
#[inline(never)]
pub fn abort_now() -> ! {
    unsafe {
        core::arch::asm!("ud2", options(noreturn, nomem, nostack));
    }
}
