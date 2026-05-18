**sel4_config**

# Module: sel4_config

## Contents

**Modules**

- [`consts`](#consts) - The kernel configuration as `const` items.

---

## Module: consts

The kernel configuration as `const` items.

While this module can be used as an alternative to the `sel4_cfg_*!` macros for accessing
the kernel configuration at the value level, its primary purpose is to provide a reference
within Rustdoc for the active configuration. Towards that end, the generated source of this
module is also provided in this module's Rustdoc to make browsing easier.
```rust
pub const AARCH64_SERROR_IGNORE: bool = false;
pub const ARCH: &str = "x86";
pub const ARCH_AARCH32: bool = false;
pub const ARCH_AARCH64: bool = false;
pub const ARCH_ARM: bool = false;
pub const ARCH_ARM_HYP: bool = false;
pub const ARCH_ARM_V7A: bool = false;
pub const ARCH_ARM_V7VE: bool = false;
pub const ARCH_ARM_V8A: bool = false;
pub const ARCH_IA32: bool = false;
pub const ARCH_RISCV: bool = false;
pub const ARCH_RISCV32: bool = false;
pub const ARCH_RISCV64: bool = false;
pub const ARCH_X86: bool = true;
pub const ARCH_X86_64: bool = true;
pub const ARCH_X86_BROADWELL: bool = false;
pub const ARCH_X86_GENERIC: bool = false;
pub const ARCH_X86_HASWELL: bool = false;
pub const ARCH_X86_IVY: bool = false;
pub const ARCH_X86_NEHALEM: bool = true;
pub const ARCH_X86_SANDY: bool = false;
pub const ARCH_X86_SKYLAKE: bool = false;
pub const ARCH_X86_WESTMERE: bool = false;
pub const ARM_CORTEX_A15: bool = false;
pub const ARM_CORTEX_A35: bool = false;
pub const ARM_CORTEX_A53: bool = false;
pub const ARM_CORTEX_A55: bool = false;
pub const ARM_CORTEX_A57: bool = false;
pub const ARM_CORTEX_A7: bool = false;
pub const ARM_CORTEX_A72: bool = false;
pub const ARM_CORTEX_A8: bool = false;
pub const ARM_CORTEX_A9: bool = false;
pub const ARM_HIKEY_OUTSTANDING_PREFETCHERS: &str = "0";
pub const ARM_HIKEY_PREFETCHER_NPFSTRM: &str = "0";
pub const ARM_HIKEY_PREFETCHER_STBPFDIS: bool = false;
pub const ARM_HIKEY_PREFETCHER_STBPFRS: bool = false;
pub const ARM_HIKEY_PREFETCHER_STRIDE: &str = "0";
pub const BENCHMARK_GENERIC: bool = false;
pub const BENCHMARK_TRACEPOINTS: bool = false;
pub const BENCHMARK_TRACK_KERNEL_ENTRIES: bool = false;
pub const BENCHMARK_TRACK_UTILISATION: bool = false;
pub const BINARY_VERIFICATION_BUILD: bool = false;
pub const CACHE_LN_SZ: &str = "64";
pub const CLZ_32: bool = false;
pub const CLZ_64: bool = false;
pub const CLZ_NO_BUILTIN: bool = false;
pub const COLOUR_PRINTING: bool = true;
pub const CTZ_32: bool = false;
pub const CTZ_64: bool = false;
pub const CTZ_NO_BUILTIN: bool = false;
pub const DANGEROUS_CODE_INJECTION: bool = false;
pub const DEBUG_BUILD: bool = true;
pub const DEBUG_DISABLE_PREFETCHERS: bool = false;
pub const ENABLE_BENCHMARKS: bool = false;
pub const ENABLE_SMP_SUPPORT: bool = false;
pub const EXCEPTION_FASTPATH: bool = false;
pub const EXPORT_PMC_USER: bool = false;
pub const FASTPATH: bool = true;
pub const FSGSBASE_GDT: bool = false;
pub const FSGSBASE_INST: bool = true;
pub const FSGSBASE_MSR: bool = false;
pub const FXSAVE: bool = false;
pub const HARDWARE_DEBUG_API: bool = false;
pub const HAVE_FPU: bool = true;
pub const HUGE_PAGE: bool = true;
pub const IOMMU: bool = true;
pub const IRQ_IOAPIC: bool = true;
pub const IRQ_PIC: bool = false;
pub const IRQ_REPORTING: bool = true;
pub const KERNEL_BENCHMARK: &str = "none";
pub const KERNEL_FSGS_BASE: &str = "inst";
pub const KERNEL_FWHOLE_PROGRAM: bool = false;
pub const KERNEL_INVOCATION_REPORT_ERROR_IPC: bool = false;
pub const KERNEL_IRQ_CONTROLLER: &str = "IOAPIC";
pub const KERNEL_LAPIC_MODE: &str = "XAPIC";
pub const KERNEL_LOG_BUFFER: bool = false;
pub const KERNEL_MCS: bool = false;
pub const KERNEL_MUTLTIBOOT_GFX_MODE: &str = "none";
pub const KERNEL_OPTIMISATION_CLONE_FUNCTIONS: bool = true;
pub const KERNEL_OPT_LEVEL: &str = "-O2";
pub const KERNEL_OPT_LEVEL_O0: bool = false;
pub const KERNEL_OPT_LEVEL_O1: bool = false;
pub const KERNEL_OPT_LEVEL_O2: bool = true;
pub const KERNEL_OPT_LEVEL_O3: bool = false;
pub const KERNEL_OPT_LEVEL_OS: bool = false;
pub const KERNEL_SKIM_WINDOW: bool = true;
pub const KERNEL_STACK_BITS: &str = "12";
pub const KERNEL_X86_DANGEROUS_MSR: bool = false;
pub const KERNEL_X86_FPU: &str = "XSAVE";
pub const KERNEL_X86_IBPB_ON_CONTEXT_SWITCH: bool = false;
pub const KERNEL_X86_IBRS: &str = "ibrs_none";
pub const KERNEL_X86_IBRS_ALL: bool = false;
pub const KERNEL_X86_IBRS_BASIC: bool = false;
pub const KERNEL_X86_IBRS_NONE: bool = true;
pub const KERNEL_X86_IBRS_STIBP: bool = false;
pub const KERNEL_X86_MICRO_ARCH: &str = "nehalem";
pub const KERNEL_X86_RSB_ON_CONTEXT_SWITCH: bool = false;
pub const KERNEL_X86_SYSCALL: &str = "syscall";
pub const KERNEL_XSAVE: &str = "XSAVE";
pub const LIB_SEL4_DEFAULT_FUNCTION_ATTRIBUTES: bool = false;
pub const LIB_SEL4_FUNCTION_ATTRIBUTE: &str = "inline";
pub const LIB_SEL4_INLINE_INVOCATIONS: bool = true;
pub const LIB_SEL4_PRINT_INVOCATION_ERRORS: &str = "0";
pub const LIB_SEL4_PUBLIC_SYMBOLS: bool = false;
pub const LIB_SEL4_STUBS_USE_IPC_BUFFER_ONLY: bool = false;
pub const LIB_SEL4_USE_THREAD_LOCALS: bool = true;
pub const MAX_NUM_BOOTINFO_UNTYPED_CAPS: &str = "230";
pub const MAX_NUM_IOAPIC: &str = "1";
pub const MAX_NUM_NODES: &str = "1";
pub const MAX_NUM_TRACE_POINTS: &str = "0";
pub const MAX_NUM_WORK_UNITS_PER_PREEMPTION: &str = "100";
pub const MAX_RMRR_ENTRIES: &str = "32";
pub const MAX_VPIDS: &str = "0";
pub const MULTIBOOT1_HEADER: bool = false;
pub const MULTIBOOT2_HEADER: bool = true;
pub const MULTIBOOT_GRAPHICS_MODE_LINEAR: bool = false;
pub const MULTIBOOT_GRAPHICS_MODE_NONE: bool = true;
pub const MULTIBOOT_GRAPHICS_MODE_TEXT: bool = false;
pub const NO_BENCHMARKS: bool = true;
pub const NUM_DOMAINS: &str = "1";
pub const NUM_DOMAIN_SCHEDULES: &str = "2";
pub const NUM_PRIORITIES: &str = "256";
pub const PADDR_USER_DEVICE_TOP: &str = "140737488355328";
pub const PC99_TSC_FREQUENCY: &str = "0";
pub const PLAT: &str = "pc99";
pub const PLAT_ALLWINNERA20: bool = false;
pub const PLAT_AM335X: bool = false;
pub const PLAT_APQ8064: bool = false;
pub const PLAT_ARIANE: bool = false;
pub const PLAT_BANANAPIF3: bool = false;
pub const PLAT_BCM2711: bool = false;
pub const PLAT_BCM2837: bool = false;
pub const PLAT_CHESHIRE: bool = false;
pub const PLAT_EXYNOS4: bool = false;
pub const PLAT_EXYNOS5: bool = false;
pub const PLAT_FVP: bool = false;
pub const PLAT_HIFIVE: bool = false;
pub const PLAT_HIFIVE_P550: bool = false;
pub const PLAT_HIKEY: bool = false;
pub const PLAT_IMX6: bool = false;
pub const PLAT_IMX7: bool = false;
pub const PLAT_IMX7_SABRE: bool = false;
pub const PLAT_IMX8MM_EVK: bool = false;
pub const PLAT_IMX8MP_EVK: bool = false;
pub const PLAT_IMX8MQ_EVK: bool = false;
pub const PLAT_IMX93: bool = false;
pub const PLAT_MAAXBOARD: bool = false;
pub const PLAT_ODROIDC2: bool = false;
pub const PLAT_ODROIDC4: bool = false;
pub const PLAT_OMAP3: bool = false;
pub const PLAT_PC99: bool = true;
pub const PLAT_POLARFIRE: bool = false;
pub const PLAT_QEMU_ARM_VIRT: bool = false;
pub const PLAT_QEMU_RISCV_VIRT: bool = false;
pub const PLAT_QUARTZ64: bool = false;
pub const PLAT_RK3568: bool = false;
pub const PLAT_ROCKETCHIP: bool = false;
pub const PLAT_ROCKPRO64: bool = false;
pub const PLAT_SPIKE: bool = false;
pub const PLAT_STAR64: bool = false;
pub const PLAT_TK1: bool = false;
pub const PLAT_TQMA8XQP1GB: bool = false;
pub const PLAT_TX1: bool = false;
pub const PLAT_TX2: bool = false;
pub const PLAT_ZYNQ7000: bool = false;
pub const PLAT_ZYNQMP: bool = false;
pub const PRINTING: bool = true;
pub const RESET_CHUNK_BITS: &str = "8";
pub const RETYPE_FAN_OUT_LIMIT: &str = "256";
pub const ROOT_CNODE_SIZE_BITS: &str = "12";
pub const SEL4_ARCH: &str = "x86_64";
pub const SET_TLS_BASE_SELF: bool = false;
pub const SIGNAL_FASTPATH: bool = false;
pub const SUPPORT_PCID: bool = true;
pub const SYSCALL: bool = true;
pub const SYSENTER: bool = false;
pub const TIMER_TICK_MS: &str = "2";
pub const TIME_SLICE: &str = "5";
pub const USER_STACK_TRACE_LENGTH: &str = "16";
pub const USE_LOGICAL_IDS: bool = false;
pub const VERIFICATION_BUILD: bool = false;
pub const VTX: bool = false;
pub const WORD_SIZE: &str = "64";
pub const X2APIC: bool = false;
pub const X86_64_VTX_64BIT_GUESTS: bool = false;
pub const XAPIC: bool = true;
pub const XSAVE: bool = true;
pub const XSAVE_FEATURE_SET: &str = "3";
pub const XSAVE_SIZE: &str = "576";
pub const XSAVE_XSAVE: bool = true;
pub const XSAVE_XSAVEC: bool = false;
pub const XSAVE_XSAVEOPT: bool = false;
pub const XSAVE_XSAVES: bool = false;
```



