//! TSC-based high-resolution clock.
//!
//! Uses the CPU Time Stamp Counter (`rdtsc`) together with the TSC frequency
//! extracted from the kernel's `BootInfo` extra region to provide wall-clock
//! time measurement and busy-wait delays.
//!
//! # Architecture
//!
//! The seL4 kernel measures the TSC frequency in MHz at boot and stores it as
//! a `u32` in an extra `BootInfo` chunk with id [`SEL4_BOOTINFO_HEADER_X86_TSC_FREQ`].
//! This module walks the extra region during [`init`] and caches the value.
//!
//! Once initialized, [`rdtsc`] and [`busy_wait`] are available for the
//! remainder of the root task's lifetime.
//!
//! # Safety
//!
//! `rdtsc` is an unprivileged instruction available at any ring level.
//! The kernel guarantees the `BootInfo` pointer is valid for the root task's
//! entire lifetime.
//!
//! # Single-core assumption
//!
//! The cached TSC frequency is stored in an [`AtomicU32`] (not `static mut`
//! — which is soft-deprecated). `Relaxed` ordering is sufficient since the
//! root task is single-core and single-threaded with no preemption on the
//! non-MCS kernel.

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use core::time::Duration;

use sel4::{BootInfoPtr, FrameObjectType};

// ---------------------------------------------------------------------------
// TSC frequency — cached from BootInfo, never changes after init
// ---------------------------------------------------------------------------

/// Cached TSC frequency in MHz.
///
/// Initialized once by [`init`] and thereafter only read. Uses [`AtomicU32`]
/// because `static mut` is soft-deprecated in Rust 2024. `Relaxed` ordering
/// is sufficient: single-core, single-threaded, no preemption on the
/// non-MCS kernel — no data-race risk.
static TSC_FREQ_MHZ: AtomicU32 = AtomicU32::new(0);

/// The TSC value at boot time — the zero-point of the timer.
///
/// [`timer_ticks`] and [`timer_ns`] compute elapsed time as
/// `rdtsc() - TIMER_BASE`. Captured once during [`init`] and never written
/// again.
///
/// `Relaxed` ordering is sufficient: single-core, no preemption.
static TIMER_BASE: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// BootInfo extra region walk
// ---------------------------------------------------------------------------

/// `BootInfo` extra chunk ID for the x86 TSC frequency (in MHz).
///
/// Defined in the seL4 kernel as `SEL4_BOOTINFO_HEADER_X86_TSC_FREQ = 5`.
/// The payload is a `uint32_t` in MHz.
const SEL4_BOOTINFO_HEADER_X86_TSC_FREQ: u64 = 5;

/// Offset from the base of [`BootInfo`] to the start of the extra region.
///
/// Equal to `seL4_BootInfoFrameSize`, which is one page on all architectures.
/// On `x86_64` this is [`FrameObjectType::GRANULE`] (the 4 KiB frame object).
const EXTRA_OFFSET: usize = FrameObjectType::GRANULE.bytes();

/// Initializes the clock by extracting the TSC frequency from `BootInfo`
/// and capturing the boot-time TSC as the timer zero-point.
///
/// # How it works
///
/// The seL4 kernel stores the TSC frequency as a `u32` (in MHz) in the
/// `BootInfo` extra region — a sequence of `(id: u64, len: u64, payload...)`
/// chunks immediately after the fixed-size `seL4_BootInfo` struct at
/// offset [`EXTRA_OFFSET`] (4096 bytes). This function walks the chunks
/// (order-independent) until it finds `id == 5`, stores the frequency,
/// then snapshots the current [`rdtsc`] value as [`TIMER_BASE`].
///
/// # Panics
///
/// Panics if:
/// - The TSC frequency chunk is missing (indicates kernel config or
///   platform mismatch, or an older kernel that doesn't populate this field).
/// - The stored frequency is outside the plausible range 10–10,000 MHz
///   (hardware or kernel bug).
///
/// # Idempotency
///
/// If called more than once, subsequent calls silently return (no-op).
/// This is safe: the frequency and boot TSC do not change after boot.
pub fn init(bootinfo: &BootInfoPtr) {
    // Guard: skip re-initialization if already set.
    if TSC_FREQ_MHZ.load(Ordering::Relaxed) != 0 {
        return;
    }

    let extra_len: usize = bootinfo
        .inner()
        .extraLen
        .try_into()
        .unwrap_or(0);

    // Compute the start of the extra region. The kernel maps the extra
    // bootinfo region contiguously after the main bootinfo frame.
    // SAFETY: extra_start points into this mapped region; all reads are
    // bounded by extraLen.
    let extra_start = unsafe { bootinfo.ptr().cast::<u8>().add(EXTRA_OFFSET) };

    let mut offset = 0usize;
    while offset + 16 <= extra_len {
        // Each chunk: [id: u64, len: u64] + payload.
        // The kernel writes these as `seL4_BootInfoHeader` structs. We use
        // `read_unaligned` because we're casting from `*const u8` (alignment 1)
        // to `*const u64` / `*const u32`. On x86_64 unaligned reads are safe
        // and the kernel does guarantee alignment in practice, but Clippy
        // requires the defensive cast.
        //
        // SAFETY: extra_start is within the bootinfo frame, offset is
        // bounded by extra_len, and the kernel populated valid headers here.
        let id =
            unsafe { extra_start.add(offset).cast::<u64>().read_unaligned() };
        let len =
            unsafe { extra_start.add(offset + 8).cast::<u64>().read_unaligned() };

        if id == SEL4_BOOTINFO_HEADER_X86_TSC_FREQ && len >= 20 {
            // Payload is a u32 (MHz) at offset + 16 (after the 16-byte header).
            // SAFETY: len >= 20 guarantees offset + 20 <= extra_len, and the
            // kernel populated a valid u32 here during boot.
            let freq_mhz = unsafe {
                extra_start
                    .add(offset + 16)
                    .cast::<u32>()
                    .read_unaligned()
            };
            assert_plausible_freq(freq_mhz);
            // Store the frequency and capture the boot-time TSC. Relaxed
            // ordering: single-core, no concurrent access, idempotency
            // guard above prevents double-write.
            TSC_FREQ_MHZ.store(freq_mhz, Ordering::Relaxed);
            TIMER_BASE.store(rdtsc(), Ordering::Relaxed);
            return;
        }

        // Advance to the next chunk. A header is at minimum 16 bytes;
        // a zero len would cause an infinite loop.
        assert!(
            len >= 16,
            "BootInfo extra chunk at offset {offset} has len={len} (minimum 16 for header)"
        );
        offset = offset
            .checked_add(usize::try_from(len).expect("BootInfo chunk len overflows usize"))
            .expect("BootInfo extra region offset overflow");
    }

    panic!(
        "TSC frequency not found in BootInfo extra region \
         (missing chunk id={SEL4_BOOTINFO_HEADER_X86_TSC_FREQ})"
    );
}

/// Sanity-checks that `freq` looks like a plausible TSC frequency.
///
/// The check is intentionally loose: any CPU frequency from 10 MHz
/// (retro hardware under QEMU) to 10 GHz (future CPUs) is accepted.
fn assert_plausible_freq(freq: u32) {
    assert!(
        (10..=10_000).contains(&freq),
        "implausible TSC frequency: {freq} MHz"
    );
}

// ---------------------------------------------------------------------------
// Time measurement
// ---------------------------------------------------------------------------

/// Returns the cached TSC frequency in MHz.
///
/// # Panics
///
/// Panics if [`init`] has not been called yet.
#[inline]
pub fn tsc_freq_mhz() -> u32 {
    let freq = TSC_FREQ_MHZ.load(Ordering::Relaxed);
    assert!(
        freq != 0,
        "time::init() must be called before using the clock"
    );
    freq
}

/// Reads the CPU Time Stamp Counter.
///
/// Returns the raw 64-bit TSC value. On modern `x86_64` CPUs this is invariant
/// (constant rate regardless of P-state changes) and monotonically increasing.
///
/// # Performance
///
/// ~20-30 cycles on modern hardware. The seL4 kernel itself uses `rdtsc`
/// (without `lfence` serialization) for its internal timekeeping — see
/// `include/arch/x86/arch/machine/timer.h` in the kernel source.
#[inline]
pub fn rdtsc() -> u64 {
    // SAFETY: rdtsc is an unprivileged instruction available at any ring level.
    // It has no side effects beyond reading the TSC register.
    unsafe { core::arch::x86_64::_rdtsc() }
}

/// Converts a TSC tick delta to nanoseconds.
///
/// Uses `u128` intermediate arithmetic to avoid overflow for deltas up to
/// ~580 years at typical TSC frequencies (~2-5 GHz).
///
/// # Panics
///
/// Panics if [`init`] has not been called yet.
#[inline]
fn ticks_to_ns(delta: u64) -> u64 {
    let freq = u128::from(tsc_freq_mhz());
    // ticks / MHz = microseconds
    // microseconds * 1000 = nanoseconds
    // = ticks * 1000 / MHz
    u64::try_from(u128::from(delta) * 1_000 / freq)
        .expect("TSC delta to nanoseconds overflow — delta too large")
}

/// Returns the elapsed [`Duration`] since the given TSC start value.
///
/// Uses [`rdtsc`] to get the current tick value and [`wrapping_sub`] to
/// compute the delta. The wrapping behavior is irrelevant in practice
/// because u64 TSC overflow at 5 GHz takes ~116 years.
///
/// # Panics
///
/// Panics if [`init`] has not been called yet.
///
/// # Example
///
/// ```ignore
/// let start = time::rdtsc();
/// // ... do work ...
/// let elapsed = time::elapsed(start);
/// ```
#[expect(dead_code, reason = "public API — used by downstream timer code")]
pub fn elapsed(start: u64) -> Duration {
    let delta = rdtsc().wrapping_sub(start);
    Duration::from_nanos(ticks_to_ns(delta))
}

// ---------------------------------------------------------------------------
// Monotonic timer — elapsed since init
// ---------------------------------------------------------------------------

/// Returns the number of CPU ticks elapsed since [`init`].
///
/// This is the canonical "timer" value — it increases on every CPU tick
/// because each call samples the live [`rdtsc`] counter and subtracts the
/// boot-time base stored in [`TIMER_BASE`]. No periodic update is needed;
/// the `AtomicU64` holds the zero-point, and the elapsed value is derived
/// from the hardware counter.
///
/// Zero at boot (immediately after [`init`]) and monotonically increasing
/// thereafter.
///
/// # Panics
///
/// Panics if [`init`] has not been called yet.
///
/// # Example
///
/// ```ignore
/// let ticks = time::timer_ticks();
/// // ... do work ...
/// let elapsed_ticks = time::timer_ticks() - ticks;
/// ```
#[inline]
pub fn timer_ticks() -> u64 {
    let base = TIMER_BASE.load(Ordering::Relaxed);
    assert!(
        base != 0,
        "time::init() must be called before reading the timer"
    );
    rdtsc().wrapping_sub(base)
}

/// Returns the number of nanoseconds elapsed since [`init`].
///
/// Convenience wrapper around [`timer_ticks`] that converts CPU ticks to
/// wall-clock nanoseconds using the TSC frequency.
///
/// # Panics
///
/// Panics if [`init`] has not been called yet.
#[expect(dead_code, reason = "public API — used by downstream timer code")]
#[inline]
pub fn timer_ns() -> u64 {
    ticks_to_ns(timer_ticks())
}

// ---------------------------------------------------------------------------
// Busy-wait
// ---------------------------------------------------------------------------

/// Spins the CPU for at least `duration`.
///
/// Uses [`core::hint::spin_loop`] (the `PAUSE` instruction on `x86_64`) on
/// every iteration to reduce power consumption and improve SMT fairness
/// when running on a hyper-threaded core.
///
/// # Panics
///
/// Panics if [`init`] has not been called yet, or if the tick-equivalent
/// duration overflows `u64` (~580 years at 5 GHz).
///
/// # Precision
///
/// Sub-microsecond durations may truncate to zero ticks (e.g., a 500 ns
/// delay with a 1 GHz TSC produces zero target ticks). The function
/// guarantees at least 1 tick of wait for any non-zero `duration`.
///
/// `rdtsc` is used without `lfence` serialization (consistent with the
/// seL4 kernel's own timekeeping — see `include/arch/x86/arch/machine/timer.h`).
/// This means the TSC read can be reordered slightly by the CPU's out-of-order
/// execution engine. The error is bounded (~1-2 cycles) and negligible for
/// any practical busy-wait duration.
///
/// # Example
///
/// ```ignore
/// // Wait for 1 millisecond.
/// time::busy_wait(core::time::Duration::from_millis(1));
/// ```
#[expect(dead_code, reason = "public API — used by downstream timer code")]
#[inline]
pub fn busy_wait(duration: Duration) {
    let freq = u128::from(tsc_freq_mhz());
    // Convert duration to TSC ticks:
    //   duration.as_nanos() / 1_000_000_000 = seconds
    //   seconds * MHz * 1_000_000 = ticks
    //   = nanos / 1_000 * MHz
    //   = nanos * MHz / 1_000
    let target_ticks = u128::from(rdtsc())
        .saturating_add(duration.as_nanos() * freq / 1_000);
    // Truncation: the u128 result is guaranteed to fit in u64 for any
    // practical duration (max ~580 years at 5 GHz).
    let mut target = u64::try_from(target_ticks)
        .expect("busy_wait duration overflow — duration too large");
    // Guard: ensure at least 1 tick of wait for any non-zero duration.
    // Prevents silent no-op when the tick conversion truncates to zero
    // (e.g., 500 ns at 1 GHz → 0 ticks).
    if !duration.is_zero() {
        target = target.max(rdtsc().wrapping_add(1));
    }
    while rdtsc() < target {
        core::hint::spin_loop();
    }
}
