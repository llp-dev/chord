//! CPU feature detection via `CPUID`.
//!
//! At boot, the root task probes the CPU for its vendor string,
//! feature flags, and version information. This data is made
//! available through [`ArchInfo`] for diagnostic logging or
//! runtime feature gating.
//!
//! # Safety
//!
//! `CPUID` is an unprivileged instruction — safe to execute at any
//! ring level. The seL4 root task runs in user mode and can freely
//! access `CPUID`.

use core::fmt;

/// `x86_64` CPU information gathered via `CPUID` during boot.
#[derive(Debug, Clone)]
pub struct ArchInfo {
    /// CPU vendor string, e.g. `"GenuineIntel"`, `"AuthenticAMD"`.
    pub vendor: [u8; 12],
    /// Maximum basic `CPUID` leaf supported.
    #[expect(dead_code, reason = "available for downstream diagnostic use")]
    pub max_basic_leaf: u32,
    /// EAX from leaf `0x01`: stepping, model, family, and type.
    pub version: u32,
    /// EBX from leaf `0x01`: brand index, `CLFLUSH` line size,
    /// logical processor count.
    #[expect(dead_code, reason = "available for downstream diagnostic use")]
    pub misc: u32,
    /// ECX from leaf `0x01`: feature flags (SSE3, SSE4.1,
    /// AES-NI, AVX, etc.).
    #[expect(dead_code, reason = "available for downstream feature gating")]
    pub features_ecx: u32,
    /// EDX from leaf `0x01`: feature flags (FPU, MMX, SSE,
    /// SSE2, PSE, PAE, APIC, etc.).
    #[expect(dead_code, reason = "available for downstream feature gating")]
    pub features_edx: u32,
}

impl fmt::Display for ArchInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vendor = core::str::from_utf8(&self.vendor).unwrap_or("???");
        let family = (self.version >> 8) & 0xf;
        let model = (self.version >> 4) & 0xf;
        let stepping = self.version & 0xf;
        write!(
            f,
            "{vendor} family={family:#x} model={model:#x} stepping={stepping:#x}"
        )
    }
}

/// Runs `CPUID` and returns an [`ArchInfo`] with the processor
/// identification and feature flags.
///
/// Returns `None` if `CPUID` is not supported (only possible on
/// processors manufactured before ~1995).
#[must_use]
pub fn detect() -> Option<ArchInfo> {
    if !cpuid_supported() {
        return None;
    }

    let leaf0 = core::arch::x86_64::__cpuid(0);

    let mut vendor = [0u8; 12];
    vendor[0..4].copy_from_slice(&leaf0.ebx.to_le_bytes());
    vendor[4..8].copy_from_slice(&leaf0.edx.to_le_bytes());
    vendor[8..12].copy_from_slice(&leaf0.ecx.to_le_bytes());

    let leaf1 = core::arch::x86_64::__cpuid(1);

    Some(ArchInfo {
        vendor,
        max_basic_leaf: leaf0.eax,
        version: leaf1.eax,
        misc: leaf1.ebx,
        features_ecx: leaf1.ecx,
        features_edx: leaf1.edx,
    })
}

/// Checks for `CPUID` support by toggling the ID bit (bit 21) in
/// `EFLAGS`. If the bit can be changed, `CPUID` is available.
fn cpuid_supported() -> bool {
    let (original, toggled): (u64, u64);
    // SAFETY: `pushfq`/`popfq` are unprivileged instructions available
    // at any ring level. We restore the original flags after the check.
    //
    // Fixed registers (`rax`, `rcx`) prevent LLVM from aliasing both
    // `out` operands to the same register — if they shared a register,
    // the `mov`/`btr`/`btc` sequence would operate on a single register,
    // making `original ^ toggled == 0` and the function always return
    // `false` (CPUID appears unsupported).
    //
    // `rbx` is reserved by LLVM (PIC base register), so `rcx` is used
    // for the second output. The template uses Rust's default Intel
    // syntax on x86_64 (destination-first).
    unsafe {
        core::arch::asm!(
            "pushfq",
            "pop rax",
            "mov rcx, rax",
            "btr rcx, 21",
            "btc rcx, 21",
            "push rcx",
            "popfq",
            "pushfq",
            "pop rcx",
            "push rax",
            "popfq",
            out("rax") original,
            out("rcx") toggled,
        );
    }
    (original ^ toggled) & (1 << 21) != 0
}
