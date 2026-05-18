*[sel4_config](../index.md) / [consts](index.md)*

---

# Module `consts`

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

## Contents

- [Constants](#constants)
  - [`AARCH64_SERROR_IGNORE`](#aarch64-serror-ignore)
  - [`ARCH`](#arch)
  - [`ARCH_AARCH32`](#arch-aarch32)
  - [`ARCH_AARCH64`](#arch-aarch64)
  - [`ARCH_ARM`](#arch-arm)
  - [`ARCH_ARM_HYP`](#arch-arm-hyp)
  - [`ARCH_ARM_V7A`](#arch-arm-v7a)
  - [`ARCH_ARM_V7VE`](#arch-arm-v7ve)
  - [`ARCH_ARM_V8A`](#arch-arm-v8a)
  - [`ARCH_IA32`](#arch-ia32)
  - [`ARCH_RISCV`](#arch-riscv)
  - [`ARCH_RISCV32`](#arch-riscv32)
  - [`ARCH_RISCV64`](#arch-riscv64)
  - [`ARCH_X86`](#arch-x86)
  - [`ARCH_X86_64`](#arch-x86-64)
  - [`ARCH_X86_BROADWELL`](#arch-x86-broadwell)
  - [`ARCH_X86_GENERIC`](#arch-x86-generic)
  - [`ARCH_X86_HASWELL`](#arch-x86-haswell)
  - [`ARCH_X86_IVY`](#arch-x86-ivy)
  - [`ARCH_X86_NEHALEM`](#arch-x86-nehalem)
  - [`ARCH_X86_SANDY`](#arch-x86-sandy)
  - [`ARCH_X86_SKYLAKE`](#arch-x86-skylake)
  - [`ARCH_X86_WESTMERE`](#arch-x86-westmere)
  - [`ARM_CORTEX_A15`](#arm-cortex-a15)
  - [`ARM_CORTEX_A35`](#arm-cortex-a35)
  - [`ARM_CORTEX_A53`](#arm-cortex-a53)
  - [`ARM_CORTEX_A55`](#arm-cortex-a55)
  - [`ARM_CORTEX_A57`](#arm-cortex-a57)
  - [`ARM_CORTEX_A7`](#arm-cortex-a7)
  - [`ARM_CORTEX_A72`](#arm-cortex-a72)
  - [`ARM_CORTEX_A8`](#arm-cortex-a8)
  - [`ARM_CORTEX_A9`](#arm-cortex-a9)
  - [`ARM_HIKEY_OUTSTANDING_PREFETCHERS`](#arm-hikey-outstanding-prefetchers)
  - [`ARM_HIKEY_PREFETCHER_NPFSTRM`](#arm-hikey-prefetcher-npfstrm)
  - [`ARM_HIKEY_PREFETCHER_STBPFDIS`](#arm-hikey-prefetcher-stbpfdis)
  - [`ARM_HIKEY_PREFETCHER_STBPFRS`](#arm-hikey-prefetcher-stbpfrs)
  - [`ARM_HIKEY_PREFETCHER_STRIDE`](#arm-hikey-prefetcher-stride)
  - [`BENCHMARK_GENERIC`](#benchmark-generic)
  - [`BENCHMARK_TRACEPOINTS`](#benchmark-tracepoints)
  - [`BENCHMARK_TRACK_KERNEL_ENTRIES`](#benchmark-track-kernel-entries)
  - [`BENCHMARK_TRACK_UTILISATION`](#benchmark-track-utilisation)
  - [`BINARY_VERIFICATION_BUILD`](#binary-verification-build)
  - [`CACHE_LN_SZ`](#cache-ln-sz)
  - [`CLZ_32`](#clz-32)
  - [`CLZ_64`](#clz-64)
  - [`CLZ_NO_BUILTIN`](#clz-no-builtin)
  - [`COLOUR_PRINTING`](#colour-printing)
  - [`CTZ_32`](#ctz-32)
  - [`CTZ_64`](#ctz-64)
  - [`CTZ_NO_BUILTIN`](#ctz-no-builtin)
  - [`DANGEROUS_CODE_INJECTION`](#dangerous-code-injection)
  - [`DEBUG_BUILD`](#debug-build)
  - [`DEBUG_DISABLE_PREFETCHERS`](#debug-disable-prefetchers)
  - [`ENABLE_BENCHMARKS`](#enable-benchmarks)
  - [`ENABLE_SMP_SUPPORT`](#enable-smp-support)
  - [`EXCEPTION_FASTPATH`](#exception-fastpath)
  - [`EXPORT_PMC_USER`](#export-pmc-user)
  - [`FASTPATH`](#fastpath)
  - [`FSGSBASE_GDT`](#fsgsbase-gdt)
  - [`FSGSBASE_INST`](#fsgsbase-inst)
  - [`FSGSBASE_MSR`](#fsgsbase-msr)
  - [`FXSAVE`](#fxsave)
  - [`HARDWARE_DEBUG_API`](#hardware-debug-api)
  - [`HAVE_FPU`](#have-fpu)
  - [`HUGE_PAGE`](#huge-page)
  - [`IOMMU`](#iommu)
  - [`IRQ_IOAPIC`](#irq-ioapic)
  - [`IRQ_PIC`](#irq-pic)
  - [`IRQ_REPORTING`](#irq-reporting)
  - [`KERNEL_BENCHMARK`](#kernel-benchmark)
  - [`KERNEL_FSGS_BASE`](#kernel-fsgs-base)
  - [`KERNEL_FWHOLE_PROGRAM`](#kernel-fwhole-program)
  - [`KERNEL_INVOCATION_REPORT_ERROR_IPC`](#kernel-invocation-report-error-ipc)
  - [`KERNEL_IRQ_CONTROLLER`](#kernel-irq-controller)
  - [`KERNEL_LAPIC_MODE`](#kernel-lapic-mode)
  - [`KERNEL_LOG_BUFFER`](#kernel-log-buffer)
  - [`KERNEL_MCS`](#kernel-mcs)
  - [`KERNEL_MUTLTIBOOT_GFX_MODE`](#kernel-mutltiboot-gfx-mode)
  - [`KERNEL_OPTIMISATION_CLONE_FUNCTIONS`](#kernel-optimisation-clone-functions)
  - [`KERNEL_OPT_LEVEL`](#kernel-opt-level)
  - [`KERNEL_OPT_LEVEL_O0`](#kernel-opt-level-o0)
  - [`KERNEL_OPT_LEVEL_O1`](#kernel-opt-level-o1)
  - [`KERNEL_OPT_LEVEL_O2`](#kernel-opt-level-o2)
  - [`KERNEL_OPT_LEVEL_O3`](#kernel-opt-level-o3)
  - [`KERNEL_OPT_LEVEL_OS`](#kernel-opt-level-os)
  - [`KERNEL_SKIM_WINDOW`](#kernel-skim-window)
  - [`KERNEL_STACK_BITS`](#kernel-stack-bits)
  - [`KERNEL_X86_DANGEROUS_MSR`](#kernel-x86-dangerous-msr)
  - [`KERNEL_X86_FPU`](#kernel-x86-fpu)
  - [`KERNEL_X86_IBPB_ON_CONTEXT_SWITCH`](#kernel-x86-ibpb-on-context-switch)
  - [`KERNEL_X86_IBRS`](#kernel-x86-ibrs)
  - [`KERNEL_X86_IBRS_ALL`](#kernel-x86-ibrs-all)
  - [`KERNEL_X86_IBRS_BASIC`](#kernel-x86-ibrs-basic)
  - [`KERNEL_X86_IBRS_NONE`](#kernel-x86-ibrs-none)
  - [`KERNEL_X86_IBRS_STIBP`](#kernel-x86-ibrs-stibp)
  - [`KERNEL_X86_MICRO_ARCH`](#kernel-x86-micro-arch)
  - [`KERNEL_X86_RSB_ON_CONTEXT_SWITCH`](#kernel-x86-rsb-on-context-switch)
  - [`KERNEL_X86_SYSCALL`](#kernel-x86-syscall)
  - [`KERNEL_XSAVE`](#kernel-xsave)
  - [`LIB_SEL4_DEFAULT_FUNCTION_ATTRIBUTES`](#lib-sel4-default-function-attributes)
  - [`LIB_SEL4_FUNCTION_ATTRIBUTE`](#lib-sel4-function-attribute)
  - [`LIB_SEL4_INLINE_INVOCATIONS`](#lib-sel4-inline-invocations)
  - [`LIB_SEL4_PRINT_INVOCATION_ERRORS`](#lib-sel4-print-invocation-errors)
  - [`LIB_SEL4_PUBLIC_SYMBOLS`](#lib-sel4-public-symbols)
  - [`LIB_SEL4_STUBS_USE_IPC_BUFFER_ONLY`](#lib-sel4-stubs-use-ipc-buffer-only)
  - [`LIB_SEL4_USE_THREAD_LOCALS`](#lib-sel4-use-thread-locals)
  - [`MAX_NUM_BOOTINFO_UNTYPED_CAPS`](#max-num-bootinfo-untyped-caps)
  - [`MAX_NUM_IOAPIC`](#max-num-ioapic)
  - [`MAX_NUM_NODES`](#max-num-nodes)
  - [`MAX_NUM_TRACE_POINTS`](#max-num-trace-points)
  - [`MAX_NUM_WORK_UNITS_PER_PREEMPTION`](#max-num-work-units-per-preemption)
  - [`MAX_RMRR_ENTRIES`](#max-rmrr-entries)
  - [`MAX_VPIDS`](#max-vpids)
  - [`MULTIBOOT1_HEADER`](#multiboot1-header)
  - [`MULTIBOOT2_HEADER`](#multiboot2-header)
  - [`MULTIBOOT_GRAPHICS_MODE_LINEAR`](#multiboot-graphics-mode-linear)
  - [`MULTIBOOT_GRAPHICS_MODE_NONE`](#multiboot-graphics-mode-none)
  - [`MULTIBOOT_GRAPHICS_MODE_TEXT`](#multiboot-graphics-mode-text)
  - [`NO_BENCHMARKS`](#no-benchmarks)
  - [`NUM_DOMAINS`](#num-domains)
  - [`NUM_DOMAIN_SCHEDULES`](#num-domain-schedules)
  - [`NUM_PRIORITIES`](#num-priorities)
  - [`PADDR_USER_DEVICE_TOP`](#paddr-user-device-top)
  - [`PC99_TSC_FREQUENCY`](#pc99-tsc-frequency)
  - [`PLAT`](#plat)
  - [`PLAT_ALLWINNERA20`](#plat-allwinnera20)
  - [`PLAT_AM335X`](#plat-am335x)
  - [`PLAT_APQ8064`](#plat-apq8064)
  - [`PLAT_ARIANE`](#plat-ariane)
  - [`PLAT_BANANAPIF3`](#plat-bananapif3)
  - [`PLAT_BCM2711`](#plat-bcm2711)
  - [`PLAT_BCM2837`](#plat-bcm2837)
  - [`PLAT_CHESHIRE`](#plat-cheshire)
  - [`PLAT_EXYNOS4`](#plat-exynos4)
  - [`PLAT_EXYNOS5`](#plat-exynos5)
  - [`PLAT_FVP`](#plat-fvp)
  - [`PLAT_HIFIVE`](#plat-hifive)
  - [`PLAT_HIFIVE_P550`](#plat-hifive-p550)
  - [`PLAT_HIKEY`](#plat-hikey)
  - [`PLAT_IMX6`](#plat-imx6)
  - [`PLAT_IMX7`](#plat-imx7)
  - [`PLAT_IMX7_SABRE`](#plat-imx7-sabre)
  - [`PLAT_IMX8MM_EVK`](#plat-imx8mm-evk)
  - [`PLAT_IMX8MP_EVK`](#plat-imx8mp-evk)
  - [`PLAT_IMX8MQ_EVK`](#plat-imx8mq-evk)
  - [`PLAT_IMX93`](#plat-imx93)
  - [`PLAT_MAAXBOARD`](#plat-maaxboard)
  - [`PLAT_ODROIDC2`](#plat-odroidc2)
  - [`PLAT_ODROIDC4`](#plat-odroidc4)
  - [`PLAT_OMAP3`](#plat-omap3)
  - [`PLAT_PC99`](#plat-pc99)
  - [`PLAT_POLARFIRE`](#plat-polarfire)
  - [`PLAT_QEMU_ARM_VIRT`](#plat-qemu-arm-virt)
  - [`PLAT_QEMU_RISCV_VIRT`](#plat-qemu-riscv-virt)
  - [`PLAT_QUARTZ64`](#plat-quartz64)
  - [`PLAT_RK3568`](#plat-rk3568)
  - [`PLAT_ROCKETCHIP`](#plat-rocketchip)
  - [`PLAT_ROCKPRO64`](#plat-rockpro64)
  - [`PLAT_SPIKE`](#plat-spike)
  - [`PLAT_STAR64`](#plat-star64)
  - [`PLAT_TK1`](#plat-tk1)
  - [`PLAT_TQMA8XQP1GB`](#plat-tqma8xqp1gb)
  - [`PLAT_TX1`](#plat-tx1)
  - [`PLAT_TX2`](#plat-tx2)
  - [`PLAT_ZYNQ7000`](#plat-zynq7000)
  - [`PLAT_ZYNQMP`](#plat-zynqmp)
  - [`PRINTING`](#printing)
  - [`RESET_CHUNK_BITS`](#reset-chunk-bits)
  - [`RETYPE_FAN_OUT_LIMIT`](#retype-fan-out-limit)
  - [`ROOT_CNODE_SIZE_BITS`](#root-cnode-size-bits)
  - [`SEL4_ARCH`](#sel4-arch)
  - [`SET_TLS_BASE_SELF`](#set-tls-base-self)
  - [`SIGNAL_FASTPATH`](#signal-fastpath)
  - [`SUPPORT_PCID`](#support-pcid)
  - [`SYSCALL`](#syscall)
  - [`SYSENTER`](#sysenter)
  - [`TIMER_TICK_MS`](#timer-tick-ms)
  - [`TIME_SLICE`](#time-slice)
  - [`USER_STACK_TRACE_LENGTH`](#user-stack-trace-length)
  - [`USE_LOGICAL_IDS`](#use-logical-ids)
  - [`VERIFICATION_BUILD`](#verification-build)
  - [`VTX`](#vtx)
  - [`WORD_SIZE`](#word-size)
  - [`X2APIC`](#x2apic)
  - [`X86_64_VTX_64BIT_GUESTS`](#x86-64-vtx-64bit-guests)
  - [`XAPIC`](#xapic)
  - [`XSAVE`](#xsave)
  - [`XSAVE_FEATURE_SET`](#xsave-feature-set)
  - [`XSAVE_SIZE`](#xsave-size)
  - [`XSAVE_XSAVE`](#xsave-xsave)
  - [`XSAVE_XSAVEC`](#xsave-xsavec)
  - [`XSAVE_XSAVEOPT`](#xsave-xsaveopt)
  - [`XSAVE_XSAVES`](#xsave-xsaves)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AARCH64_SERROR_IGNORE`](#aarch64-serror-ignore) | const |  |
| [`ARCH`](#arch) | const |  |
| [`ARCH_AARCH32`](#arch-aarch32) | const |  |
| [`ARCH_AARCH64`](#arch-aarch64) | const |  |
| [`ARCH_ARM`](#arch-arm) | const |  |
| [`ARCH_ARM_HYP`](#arch-arm-hyp) | const |  |
| [`ARCH_ARM_V7A`](#arch-arm-v7a) | const |  |
| [`ARCH_ARM_V7VE`](#arch-arm-v7ve) | const |  |
| [`ARCH_ARM_V8A`](#arch-arm-v8a) | const |  |
| [`ARCH_IA32`](#arch-ia32) | const |  |
| [`ARCH_RISCV`](#arch-riscv) | const |  |
| [`ARCH_RISCV32`](#arch-riscv32) | const |  |
| [`ARCH_RISCV64`](#arch-riscv64) | const |  |
| [`ARCH_X86`](#arch-x86) | const |  |
| [`ARCH_X86_64`](#arch-x86-64) | const |  |
| [`ARCH_X86_BROADWELL`](#arch-x86-broadwell) | const |  |
| [`ARCH_X86_GENERIC`](#arch-x86-generic) | const |  |
| [`ARCH_X86_HASWELL`](#arch-x86-haswell) | const |  |
| [`ARCH_X86_IVY`](#arch-x86-ivy) | const |  |
| [`ARCH_X86_NEHALEM`](#arch-x86-nehalem) | const |  |
| [`ARCH_X86_SANDY`](#arch-x86-sandy) | const |  |
| [`ARCH_X86_SKYLAKE`](#arch-x86-skylake) | const |  |
| [`ARCH_X86_WESTMERE`](#arch-x86-westmere) | const |  |
| [`ARM_CORTEX_A15`](#arm-cortex-a15) | const |  |
| [`ARM_CORTEX_A35`](#arm-cortex-a35) | const |  |
| [`ARM_CORTEX_A53`](#arm-cortex-a53) | const |  |
| [`ARM_CORTEX_A55`](#arm-cortex-a55) | const |  |
| [`ARM_CORTEX_A57`](#arm-cortex-a57) | const |  |
| [`ARM_CORTEX_A7`](#arm-cortex-a7) | const |  |
| [`ARM_CORTEX_A72`](#arm-cortex-a72) | const |  |
| [`ARM_CORTEX_A8`](#arm-cortex-a8) | const |  |
| [`ARM_CORTEX_A9`](#arm-cortex-a9) | const |  |
| [`ARM_HIKEY_OUTSTANDING_PREFETCHERS`](#arm-hikey-outstanding-prefetchers) | const |  |
| [`ARM_HIKEY_PREFETCHER_NPFSTRM`](#arm-hikey-prefetcher-npfstrm) | const |  |
| [`ARM_HIKEY_PREFETCHER_STBPFDIS`](#arm-hikey-prefetcher-stbpfdis) | const |  |
| [`ARM_HIKEY_PREFETCHER_STBPFRS`](#arm-hikey-prefetcher-stbpfrs) | const |  |
| [`ARM_HIKEY_PREFETCHER_STRIDE`](#arm-hikey-prefetcher-stride) | const |  |
| [`BENCHMARK_GENERIC`](#benchmark-generic) | const |  |
| [`BENCHMARK_TRACEPOINTS`](#benchmark-tracepoints) | const |  |
| [`BENCHMARK_TRACK_KERNEL_ENTRIES`](#benchmark-track-kernel-entries) | const |  |
| [`BENCHMARK_TRACK_UTILISATION`](#benchmark-track-utilisation) | const |  |
| [`BINARY_VERIFICATION_BUILD`](#binary-verification-build) | const |  |
| [`CACHE_LN_SZ`](#cache-ln-sz) | const |  |
| [`CLZ_32`](#clz-32) | const |  |
| [`CLZ_64`](#clz-64) | const |  |
| [`CLZ_NO_BUILTIN`](#clz-no-builtin) | const |  |
| [`COLOUR_PRINTING`](#colour-printing) | const |  |
| [`CTZ_32`](#ctz-32) | const |  |
| [`CTZ_64`](#ctz-64) | const |  |
| [`CTZ_NO_BUILTIN`](#ctz-no-builtin) | const |  |
| [`DANGEROUS_CODE_INJECTION`](#dangerous-code-injection) | const |  |
| [`DEBUG_BUILD`](#debug-build) | const |  |
| [`DEBUG_DISABLE_PREFETCHERS`](#debug-disable-prefetchers) | const |  |
| [`ENABLE_BENCHMARKS`](#enable-benchmarks) | const |  |
| [`ENABLE_SMP_SUPPORT`](#enable-smp-support) | const |  |
| [`EXCEPTION_FASTPATH`](#exception-fastpath) | const |  |
| [`EXPORT_PMC_USER`](#export-pmc-user) | const |  |
| [`FASTPATH`](#fastpath) | const |  |
| [`FSGSBASE_GDT`](#fsgsbase-gdt) | const |  |
| [`FSGSBASE_INST`](#fsgsbase-inst) | const |  |
| [`FSGSBASE_MSR`](#fsgsbase-msr) | const |  |
| [`FXSAVE`](#fxsave) | const |  |
| [`HARDWARE_DEBUG_API`](#hardware-debug-api) | const |  |
| [`HAVE_FPU`](#have-fpu) | const |  |
| [`HUGE_PAGE`](#huge-page) | const |  |
| [`IOMMU`](#iommu) | const |  |
| [`IRQ_IOAPIC`](#irq-ioapic) | const |  |
| [`IRQ_PIC`](#irq-pic) | const |  |
| [`IRQ_REPORTING`](#irq-reporting) | const |  |
| [`KERNEL_BENCHMARK`](#kernel-benchmark) | const |  |
| [`KERNEL_FSGS_BASE`](#kernel-fsgs-base) | const |  |
| [`KERNEL_FWHOLE_PROGRAM`](#kernel-fwhole-program) | const |  |
| [`KERNEL_INVOCATION_REPORT_ERROR_IPC`](#kernel-invocation-report-error-ipc) | const |  |
| [`KERNEL_IRQ_CONTROLLER`](#kernel-irq-controller) | const |  |
| [`KERNEL_LAPIC_MODE`](#kernel-lapic-mode) | const |  |
| [`KERNEL_LOG_BUFFER`](#kernel-log-buffer) | const |  |
| [`KERNEL_MCS`](#kernel-mcs) | const |  |
| [`KERNEL_MUTLTIBOOT_GFX_MODE`](#kernel-mutltiboot-gfx-mode) | const |  |
| [`KERNEL_OPTIMISATION_CLONE_FUNCTIONS`](#kernel-optimisation-clone-functions) | const |  |
| [`KERNEL_OPT_LEVEL`](#kernel-opt-level) | const |  |
| [`KERNEL_OPT_LEVEL_O0`](#kernel-opt-level-o0) | const |  |
| [`KERNEL_OPT_LEVEL_O1`](#kernel-opt-level-o1) | const |  |
| [`KERNEL_OPT_LEVEL_O2`](#kernel-opt-level-o2) | const |  |
| [`KERNEL_OPT_LEVEL_O3`](#kernel-opt-level-o3) | const |  |
| [`KERNEL_OPT_LEVEL_OS`](#kernel-opt-level-os) | const |  |
| [`KERNEL_SKIM_WINDOW`](#kernel-skim-window) | const |  |
| [`KERNEL_STACK_BITS`](#kernel-stack-bits) | const |  |
| [`KERNEL_X86_DANGEROUS_MSR`](#kernel-x86-dangerous-msr) | const |  |
| [`KERNEL_X86_FPU`](#kernel-x86-fpu) | const |  |
| [`KERNEL_X86_IBPB_ON_CONTEXT_SWITCH`](#kernel-x86-ibpb-on-context-switch) | const |  |
| [`KERNEL_X86_IBRS`](#kernel-x86-ibrs) | const |  |
| [`KERNEL_X86_IBRS_ALL`](#kernel-x86-ibrs-all) | const |  |
| [`KERNEL_X86_IBRS_BASIC`](#kernel-x86-ibrs-basic) | const |  |
| [`KERNEL_X86_IBRS_NONE`](#kernel-x86-ibrs-none) | const |  |
| [`KERNEL_X86_IBRS_STIBP`](#kernel-x86-ibrs-stibp) | const |  |
| [`KERNEL_X86_MICRO_ARCH`](#kernel-x86-micro-arch) | const |  |
| [`KERNEL_X86_RSB_ON_CONTEXT_SWITCH`](#kernel-x86-rsb-on-context-switch) | const |  |
| [`KERNEL_X86_SYSCALL`](#kernel-x86-syscall) | const |  |
| [`KERNEL_XSAVE`](#kernel-xsave) | const |  |
| [`LIB_SEL4_DEFAULT_FUNCTION_ATTRIBUTES`](#lib-sel4-default-function-attributes) | const |  |
| [`LIB_SEL4_FUNCTION_ATTRIBUTE`](#lib-sel4-function-attribute) | const |  |
| [`LIB_SEL4_INLINE_INVOCATIONS`](#lib-sel4-inline-invocations) | const |  |
| [`LIB_SEL4_PRINT_INVOCATION_ERRORS`](#lib-sel4-print-invocation-errors) | const |  |
| [`LIB_SEL4_PUBLIC_SYMBOLS`](#lib-sel4-public-symbols) | const |  |
| [`LIB_SEL4_STUBS_USE_IPC_BUFFER_ONLY`](#lib-sel4-stubs-use-ipc-buffer-only) | const |  |
| [`LIB_SEL4_USE_THREAD_LOCALS`](#lib-sel4-use-thread-locals) | const |  |
| [`MAX_NUM_BOOTINFO_UNTYPED_CAPS`](#max-num-bootinfo-untyped-caps) | const |  |
| [`MAX_NUM_IOAPIC`](#max-num-ioapic) | const |  |
| [`MAX_NUM_NODES`](#max-num-nodes) | const |  |
| [`MAX_NUM_TRACE_POINTS`](#max-num-trace-points) | const |  |
| [`MAX_NUM_WORK_UNITS_PER_PREEMPTION`](#max-num-work-units-per-preemption) | const |  |
| [`MAX_RMRR_ENTRIES`](#max-rmrr-entries) | const |  |
| [`MAX_VPIDS`](#max-vpids) | const |  |
| [`MULTIBOOT1_HEADER`](#multiboot1-header) | const |  |
| [`MULTIBOOT2_HEADER`](#multiboot2-header) | const |  |
| [`MULTIBOOT_GRAPHICS_MODE_LINEAR`](#multiboot-graphics-mode-linear) | const |  |
| [`MULTIBOOT_GRAPHICS_MODE_NONE`](#multiboot-graphics-mode-none) | const |  |
| [`MULTIBOOT_GRAPHICS_MODE_TEXT`](#multiboot-graphics-mode-text) | const |  |
| [`NO_BENCHMARKS`](#no-benchmarks) | const |  |
| [`NUM_DOMAINS`](#num-domains) | const |  |
| [`NUM_DOMAIN_SCHEDULES`](#num-domain-schedules) | const |  |
| [`NUM_PRIORITIES`](#num-priorities) | const |  |
| [`PADDR_USER_DEVICE_TOP`](#paddr-user-device-top) | const |  |
| [`PC99_TSC_FREQUENCY`](#pc99-tsc-frequency) | const |  |
| [`PLAT`](#plat) | const |  |
| [`PLAT_ALLWINNERA20`](#plat-allwinnera20) | const |  |
| [`PLAT_AM335X`](#plat-am335x) | const |  |
| [`PLAT_APQ8064`](#plat-apq8064) | const |  |
| [`PLAT_ARIANE`](#plat-ariane) | const |  |
| [`PLAT_BANANAPIF3`](#plat-bananapif3) | const |  |
| [`PLAT_BCM2711`](#plat-bcm2711) | const |  |
| [`PLAT_BCM2837`](#plat-bcm2837) | const |  |
| [`PLAT_CHESHIRE`](#plat-cheshire) | const |  |
| [`PLAT_EXYNOS4`](#plat-exynos4) | const |  |
| [`PLAT_EXYNOS5`](#plat-exynos5) | const |  |
| [`PLAT_FVP`](#plat-fvp) | const |  |
| [`PLAT_HIFIVE`](#plat-hifive) | const |  |
| [`PLAT_HIFIVE_P550`](#plat-hifive-p550) | const |  |
| [`PLAT_HIKEY`](#plat-hikey) | const |  |
| [`PLAT_IMX6`](#plat-imx6) | const |  |
| [`PLAT_IMX7`](#plat-imx7) | const |  |
| [`PLAT_IMX7_SABRE`](#plat-imx7-sabre) | const |  |
| [`PLAT_IMX8MM_EVK`](#plat-imx8mm-evk) | const |  |
| [`PLAT_IMX8MP_EVK`](#plat-imx8mp-evk) | const |  |
| [`PLAT_IMX8MQ_EVK`](#plat-imx8mq-evk) | const |  |
| [`PLAT_IMX93`](#plat-imx93) | const |  |
| [`PLAT_MAAXBOARD`](#plat-maaxboard) | const |  |
| [`PLAT_ODROIDC2`](#plat-odroidc2) | const |  |
| [`PLAT_ODROIDC4`](#plat-odroidc4) | const |  |
| [`PLAT_OMAP3`](#plat-omap3) | const |  |
| [`PLAT_PC99`](#plat-pc99) | const |  |
| [`PLAT_POLARFIRE`](#plat-polarfire) | const |  |
| [`PLAT_QEMU_ARM_VIRT`](#plat-qemu-arm-virt) | const |  |
| [`PLAT_QEMU_RISCV_VIRT`](#plat-qemu-riscv-virt) | const |  |
| [`PLAT_QUARTZ64`](#plat-quartz64) | const |  |
| [`PLAT_RK3568`](#plat-rk3568) | const |  |
| [`PLAT_ROCKETCHIP`](#plat-rocketchip) | const |  |
| [`PLAT_ROCKPRO64`](#plat-rockpro64) | const |  |
| [`PLAT_SPIKE`](#plat-spike) | const |  |
| [`PLAT_STAR64`](#plat-star64) | const |  |
| [`PLAT_TK1`](#plat-tk1) | const |  |
| [`PLAT_TQMA8XQP1GB`](#plat-tqma8xqp1gb) | const |  |
| [`PLAT_TX1`](#plat-tx1) | const |  |
| [`PLAT_TX2`](#plat-tx2) | const |  |
| [`PLAT_ZYNQ7000`](#plat-zynq7000) | const |  |
| [`PLAT_ZYNQMP`](#plat-zynqmp) | const |  |
| [`PRINTING`](#printing) | const |  |
| [`RESET_CHUNK_BITS`](#reset-chunk-bits) | const |  |
| [`RETYPE_FAN_OUT_LIMIT`](#retype-fan-out-limit) | const |  |
| [`ROOT_CNODE_SIZE_BITS`](#root-cnode-size-bits) | const |  |
| [`SEL4_ARCH`](#sel4-arch) | const |  |
| [`SET_TLS_BASE_SELF`](#set-tls-base-self) | const |  |
| [`SIGNAL_FASTPATH`](#signal-fastpath) | const |  |
| [`SUPPORT_PCID`](#support-pcid) | const |  |
| [`SYSCALL`](#syscall) | const |  |
| [`SYSENTER`](#sysenter) | const |  |
| [`TIMER_TICK_MS`](#timer-tick-ms) | const |  |
| [`TIME_SLICE`](#time-slice) | const |  |
| [`USER_STACK_TRACE_LENGTH`](#user-stack-trace-length) | const |  |
| [`USE_LOGICAL_IDS`](#use-logical-ids) | const |  |
| [`VERIFICATION_BUILD`](#verification-build) | const |  |
| [`VTX`](#vtx) | const |  |
| [`WORD_SIZE`](#word-size) | const |  |
| [`X2APIC`](#x2apic) | const |  |
| [`X86_64_VTX_64BIT_GUESTS`](#x86-64-vtx-64bit-guests) | const |  |
| [`XAPIC`](#xapic) | const |  |
| [`XSAVE`](#xsave) | const |  |
| [`XSAVE_FEATURE_SET`](#xsave-feature-set) | const |  |
| [`XSAVE_SIZE`](#xsave-size) | const |  |
| [`XSAVE_XSAVE`](#xsave-xsave) | const |  |
| [`XSAVE_XSAVEC`](#xsave-xsavec) | const |  |
| [`XSAVE_XSAVEOPT`](#xsave-xsaveopt) | const |  |
| [`XSAVE_XSAVES`](#xsave-xsaves) | const |  |

## Constants

### `AARCH64_SERROR_IGNORE`
```rust
const AARCH64_SERROR_IGNORE: bool = false;
```

### `ARCH`
```rust
const ARCH: &str;
```

### `ARCH_AARCH32`
```rust
const ARCH_AARCH32: bool = false;
```

### `ARCH_AARCH64`
```rust
const ARCH_AARCH64: bool = false;
```

### `ARCH_ARM`
```rust
const ARCH_ARM: bool = false;
```

### `ARCH_ARM_HYP`
```rust
const ARCH_ARM_HYP: bool = false;
```

### `ARCH_ARM_V7A`
```rust
const ARCH_ARM_V7A: bool = false;
```

### `ARCH_ARM_V7VE`
```rust
const ARCH_ARM_V7VE: bool = false;
```

### `ARCH_ARM_V8A`
```rust
const ARCH_ARM_V8A: bool = false;
```

### `ARCH_IA32`
```rust
const ARCH_IA32: bool = false;
```

### `ARCH_RISCV`
```rust
const ARCH_RISCV: bool = false;
```

### `ARCH_RISCV32`
```rust
const ARCH_RISCV32: bool = false;
```

### `ARCH_RISCV64`
```rust
const ARCH_RISCV64: bool = false;
```

### `ARCH_X86`
```rust
const ARCH_X86: bool = true;
```

### `ARCH_X86_64`
```rust
const ARCH_X86_64: bool = true;
```

### `ARCH_X86_BROADWELL`
```rust
const ARCH_X86_BROADWELL: bool = false;
```

### `ARCH_X86_GENERIC`
```rust
const ARCH_X86_GENERIC: bool = false;
```

### `ARCH_X86_HASWELL`
```rust
const ARCH_X86_HASWELL: bool = false;
```

### `ARCH_X86_IVY`
```rust
const ARCH_X86_IVY: bool = false;
```

### `ARCH_X86_NEHALEM`
```rust
const ARCH_X86_NEHALEM: bool = true;
```

### `ARCH_X86_SANDY`
```rust
const ARCH_X86_SANDY: bool = false;
```

### `ARCH_X86_SKYLAKE`
```rust
const ARCH_X86_SKYLAKE: bool = false;
```

### `ARCH_X86_WESTMERE`
```rust
const ARCH_X86_WESTMERE: bool = false;
```

### `ARM_CORTEX_A15`
```rust
const ARM_CORTEX_A15: bool = false;
```

### `ARM_CORTEX_A35`
```rust
const ARM_CORTEX_A35: bool = false;
```

### `ARM_CORTEX_A53`
```rust
const ARM_CORTEX_A53: bool = false;
```

### `ARM_CORTEX_A55`
```rust
const ARM_CORTEX_A55: bool = false;
```

### `ARM_CORTEX_A57`
```rust
const ARM_CORTEX_A57: bool = false;
```

### `ARM_CORTEX_A7`
```rust
const ARM_CORTEX_A7: bool = false;
```

### `ARM_CORTEX_A72`
```rust
const ARM_CORTEX_A72: bool = false;
```

### `ARM_CORTEX_A8`
```rust
const ARM_CORTEX_A8: bool = false;
```

### `ARM_CORTEX_A9`
```rust
const ARM_CORTEX_A9: bool = false;
```

### `ARM_HIKEY_OUTSTANDING_PREFETCHERS`
```rust
const ARM_HIKEY_OUTSTANDING_PREFETCHERS: &str;
```

### `ARM_HIKEY_PREFETCHER_NPFSTRM`
```rust
const ARM_HIKEY_PREFETCHER_NPFSTRM: &str;
```

### `ARM_HIKEY_PREFETCHER_STBPFDIS`
```rust
const ARM_HIKEY_PREFETCHER_STBPFDIS: bool = false;
```

### `ARM_HIKEY_PREFETCHER_STBPFRS`
```rust
const ARM_HIKEY_PREFETCHER_STBPFRS: bool = false;
```

### `ARM_HIKEY_PREFETCHER_STRIDE`
```rust
const ARM_HIKEY_PREFETCHER_STRIDE: &str;
```

### `BENCHMARK_GENERIC`
```rust
const BENCHMARK_GENERIC: bool = false;
```

### `BENCHMARK_TRACEPOINTS`
```rust
const BENCHMARK_TRACEPOINTS: bool = false;
```

### `BENCHMARK_TRACK_KERNEL_ENTRIES`
```rust
const BENCHMARK_TRACK_KERNEL_ENTRIES: bool = false;
```

### `BENCHMARK_TRACK_UTILISATION`
```rust
const BENCHMARK_TRACK_UTILISATION: bool = false;
```

### `BINARY_VERIFICATION_BUILD`
```rust
const BINARY_VERIFICATION_BUILD: bool = false;
```

### `CACHE_LN_SZ`
```rust
const CACHE_LN_SZ: &str;
```

### `CLZ_32`
```rust
const CLZ_32: bool = false;
```

### `CLZ_64`
```rust
const CLZ_64: bool = false;
```

### `CLZ_NO_BUILTIN`
```rust
const CLZ_NO_BUILTIN: bool = false;
```

### `COLOUR_PRINTING`
```rust
const COLOUR_PRINTING: bool = true;
```

### `CTZ_32`
```rust
const CTZ_32: bool = false;
```

### `CTZ_64`
```rust
const CTZ_64: bool = false;
```

### `CTZ_NO_BUILTIN`
```rust
const CTZ_NO_BUILTIN: bool = false;
```

### `DANGEROUS_CODE_INJECTION`
```rust
const DANGEROUS_CODE_INJECTION: bool = false;
```

### `DEBUG_BUILD`
```rust
const DEBUG_BUILD: bool = true;
```

### `DEBUG_DISABLE_PREFETCHERS`
```rust
const DEBUG_DISABLE_PREFETCHERS: bool = false;
```

### `ENABLE_BENCHMARKS`
```rust
const ENABLE_BENCHMARKS: bool = false;
```

### `ENABLE_SMP_SUPPORT`
```rust
const ENABLE_SMP_SUPPORT: bool = false;
```

### `EXCEPTION_FASTPATH`
```rust
const EXCEPTION_FASTPATH: bool = false;
```

### `EXPORT_PMC_USER`
```rust
const EXPORT_PMC_USER: bool = false;
```

### `FASTPATH`
```rust
const FASTPATH: bool = true;
```

### `FSGSBASE_GDT`
```rust
const FSGSBASE_GDT: bool = false;
```

### `FSGSBASE_INST`
```rust
const FSGSBASE_INST: bool = true;
```

### `FSGSBASE_MSR`
```rust
const FSGSBASE_MSR: bool = false;
```

### `FXSAVE`
```rust
const FXSAVE: bool = false;
```

### `HARDWARE_DEBUG_API`
```rust
const HARDWARE_DEBUG_API: bool = false;
```

### `HAVE_FPU`
```rust
const HAVE_FPU: bool = true;
```

### `HUGE_PAGE`
```rust
const HUGE_PAGE: bool = true;
```

### `IOMMU`
```rust
const IOMMU: bool = true;
```

### `IRQ_IOAPIC`
```rust
const IRQ_IOAPIC: bool = true;
```

### `IRQ_PIC`
```rust
const IRQ_PIC: bool = false;
```

### `IRQ_REPORTING`
```rust
const IRQ_REPORTING: bool = true;
```

### `KERNEL_BENCHMARK`
```rust
const KERNEL_BENCHMARK: &str;
```

### `KERNEL_FSGS_BASE`
```rust
const KERNEL_FSGS_BASE: &str;
```

### `KERNEL_FWHOLE_PROGRAM`
```rust
const KERNEL_FWHOLE_PROGRAM: bool = false;
```

### `KERNEL_INVOCATION_REPORT_ERROR_IPC`
```rust
const KERNEL_INVOCATION_REPORT_ERROR_IPC: bool = false;
```

### `KERNEL_IRQ_CONTROLLER`
```rust
const KERNEL_IRQ_CONTROLLER: &str;
```

### `KERNEL_LAPIC_MODE`
```rust
const KERNEL_LAPIC_MODE: &str;
```

### `KERNEL_LOG_BUFFER`
```rust
const KERNEL_LOG_BUFFER: bool = false;
```

### `KERNEL_MCS`
```rust
const KERNEL_MCS: bool = false;
```

### `KERNEL_MUTLTIBOOT_GFX_MODE`
```rust
const KERNEL_MUTLTIBOOT_GFX_MODE: &str;
```

### `KERNEL_OPTIMISATION_CLONE_FUNCTIONS`
```rust
const KERNEL_OPTIMISATION_CLONE_FUNCTIONS: bool = true;
```

### `KERNEL_OPT_LEVEL`
```rust
const KERNEL_OPT_LEVEL: &str;
```

### `KERNEL_OPT_LEVEL_O0`
```rust
const KERNEL_OPT_LEVEL_O0: bool = false;
```

### `KERNEL_OPT_LEVEL_O1`
```rust
const KERNEL_OPT_LEVEL_O1: bool = false;
```

### `KERNEL_OPT_LEVEL_O2`
```rust
const KERNEL_OPT_LEVEL_O2: bool = true;
```

### `KERNEL_OPT_LEVEL_O3`
```rust
const KERNEL_OPT_LEVEL_O3: bool = false;
```

### `KERNEL_OPT_LEVEL_OS`
```rust
const KERNEL_OPT_LEVEL_OS: bool = false;
```

### `KERNEL_SKIM_WINDOW`
```rust
const KERNEL_SKIM_WINDOW: bool = true;
```

### `KERNEL_STACK_BITS`
```rust
const KERNEL_STACK_BITS: &str;
```

### `KERNEL_X86_DANGEROUS_MSR`
```rust
const KERNEL_X86_DANGEROUS_MSR: bool = false;
```

### `KERNEL_X86_FPU`
```rust
const KERNEL_X86_FPU: &str;
```

### `KERNEL_X86_IBPB_ON_CONTEXT_SWITCH`
```rust
const KERNEL_X86_IBPB_ON_CONTEXT_SWITCH: bool = false;
```

### `KERNEL_X86_IBRS`
```rust
const KERNEL_X86_IBRS: &str;
```

### `KERNEL_X86_IBRS_ALL`
```rust
const KERNEL_X86_IBRS_ALL: bool = false;
```

### `KERNEL_X86_IBRS_BASIC`
```rust
const KERNEL_X86_IBRS_BASIC: bool = false;
```

### `KERNEL_X86_IBRS_NONE`
```rust
const KERNEL_X86_IBRS_NONE: bool = true;
```

### `KERNEL_X86_IBRS_STIBP`
```rust
const KERNEL_X86_IBRS_STIBP: bool = false;
```

### `KERNEL_X86_MICRO_ARCH`
```rust
const KERNEL_X86_MICRO_ARCH: &str;
```

### `KERNEL_X86_RSB_ON_CONTEXT_SWITCH`
```rust
const KERNEL_X86_RSB_ON_CONTEXT_SWITCH: bool = false;
```

### `KERNEL_X86_SYSCALL`
```rust
const KERNEL_X86_SYSCALL: &str;
```

### `KERNEL_XSAVE`
```rust
const KERNEL_XSAVE: &str;
```

### `LIB_SEL4_DEFAULT_FUNCTION_ATTRIBUTES`
```rust
const LIB_SEL4_DEFAULT_FUNCTION_ATTRIBUTES: bool = false;
```

### `LIB_SEL4_FUNCTION_ATTRIBUTE`
```rust
const LIB_SEL4_FUNCTION_ATTRIBUTE: &str;
```

### `LIB_SEL4_INLINE_INVOCATIONS`
```rust
const LIB_SEL4_INLINE_INVOCATIONS: bool = true;
```

### `LIB_SEL4_PRINT_INVOCATION_ERRORS`
```rust
const LIB_SEL4_PRINT_INVOCATION_ERRORS: &str;
```

### `LIB_SEL4_PUBLIC_SYMBOLS`
```rust
const LIB_SEL4_PUBLIC_SYMBOLS: bool = false;
```

### `LIB_SEL4_STUBS_USE_IPC_BUFFER_ONLY`
```rust
const LIB_SEL4_STUBS_USE_IPC_BUFFER_ONLY: bool = false;
```

### `LIB_SEL4_USE_THREAD_LOCALS`
```rust
const LIB_SEL4_USE_THREAD_LOCALS: bool = true;
```

### `MAX_NUM_BOOTINFO_UNTYPED_CAPS`
```rust
const MAX_NUM_BOOTINFO_UNTYPED_CAPS: &str;
```

### `MAX_NUM_IOAPIC`
```rust
const MAX_NUM_IOAPIC: &str;
```

### `MAX_NUM_NODES`
```rust
const MAX_NUM_NODES: &str;
```

### `MAX_NUM_TRACE_POINTS`
```rust
const MAX_NUM_TRACE_POINTS: &str;
```

### `MAX_NUM_WORK_UNITS_PER_PREEMPTION`
```rust
const MAX_NUM_WORK_UNITS_PER_PREEMPTION: &str;
```

### `MAX_RMRR_ENTRIES`
```rust
const MAX_RMRR_ENTRIES: &str;
```

### `MAX_VPIDS`
```rust
const MAX_VPIDS: &str;
```

### `MULTIBOOT1_HEADER`
```rust
const MULTIBOOT1_HEADER: bool = false;
```

### `MULTIBOOT2_HEADER`
```rust
const MULTIBOOT2_HEADER: bool = true;
```

### `MULTIBOOT_GRAPHICS_MODE_LINEAR`
```rust
const MULTIBOOT_GRAPHICS_MODE_LINEAR: bool = false;
```

### `MULTIBOOT_GRAPHICS_MODE_NONE`
```rust
const MULTIBOOT_GRAPHICS_MODE_NONE: bool = true;
```

### `MULTIBOOT_GRAPHICS_MODE_TEXT`
```rust
const MULTIBOOT_GRAPHICS_MODE_TEXT: bool = false;
```

### `NO_BENCHMARKS`
```rust
const NO_BENCHMARKS: bool = true;
```

### `NUM_DOMAINS`
```rust
const NUM_DOMAINS: &str;
```

### `NUM_DOMAIN_SCHEDULES`
```rust
const NUM_DOMAIN_SCHEDULES: &str;
```

### `NUM_PRIORITIES`
```rust
const NUM_PRIORITIES: &str;
```

### `PADDR_USER_DEVICE_TOP`
```rust
const PADDR_USER_DEVICE_TOP: &str;
```

### `PC99_TSC_FREQUENCY`
```rust
const PC99_TSC_FREQUENCY: &str;
```

### `PLAT`
```rust
const PLAT: &str;
```

### `PLAT_ALLWINNERA20`
```rust
const PLAT_ALLWINNERA20: bool = false;
```

### `PLAT_AM335X`
```rust
const PLAT_AM335X: bool = false;
```

### `PLAT_APQ8064`
```rust
const PLAT_APQ8064: bool = false;
```

### `PLAT_ARIANE`
```rust
const PLAT_ARIANE: bool = false;
```

### `PLAT_BANANAPIF3`
```rust
const PLAT_BANANAPIF3: bool = false;
```

### `PLAT_BCM2711`
```rust
const PLAT_BCM2711: bool = false;
```

### `PLAT_BCM2837`
```rust
const PLAT_BCM2837: bool = false;
```

### `PLAT_CHESHIRE`
```rust
const PLAT_CHESHIRE: bool = false;
```

### `PLAT_EXYNOS4`
```rust
const PLAT_EXYNOS4: bool = false;
```

### `PLAT_EXYNOS5`
```rust
const PLAT_EXYNOS5: bool = false;
```

### `PLAT_FVP`
```rust
const PLAT_FVP: bool = false;
```

### `PLAT_HIFIVE`
```rust
const PLAT_HIFIVE: bool = false;
```

### `PLAT_HIFIVE_P550`
```rust
const PLAT_HIFIVE_P550: bool = false;
```

### `PLAT_HIKEY`
```rust
const PLAT_HIKEY: bool = false;
```

### `PLAT_IMX6`
```rust
const PLAT_IMX6: bool = false;
```

### `PLAT_IMX7`
```rust
const PLAT_IMX7: bool = false;
```

### `PLAT_IMX7_SABRE`
```rust
const PLAT_IMX7_SABRE: bool = false;
```

### `PLAT_IMX8MM_EVK`
```rust
const PLAT_IMX8MM_EVK: bool = false;
```

### `PLAT_IMX8MP_EVK`
```rust
const PLAT_IMX8MP_EVK: bool = false;
```

### `PLAT_IMX8MQ_EVK`
```rust
const PLAT_IMX8MQ_EVK: bool = false;
```

### `PLAT_IMX93`
```rust
const PLAT_IMX93: bool = false;
```

### `PLAT_MAAXBOARD`
```rust
const PLAT_MAAXBOARD: bool = false;
```

### `PLAT_ODROIDC2`
```rust
const PLAT_ODROIDC2: bool = false;
```

### `PLAT_ODROIDC4`
```rust
const PLAT_ODROIDC4: bool = false;
```

### `PLAT_OMAP3`
```rust
const PLAT_OMAP3: bool = false;
```

### `PLAT_PC99`
```rust
const PLAT_PC99: bool = true;
```

### `PLAT_POLARFIRE`
```rust
const PLAT_POLARFIRE: bool = false;
```

### `PLAT_QEMU_ARM_VIRT`
```rust
const PLAT_QEMU_ARM_VIRT: bool = false;
```

### `PLAT_QEMU_RISCV_VIRT`
```rust
const PLAT_QEMU_RISCV_VIRT: bool = false;
```

### `PLAT_QUARTZ64`
```rust
const PLAT_QUARTZ64: bool = false;
```

### `PLAT_RK3568`
```rust
const PLAT_RK3568: bool = false;
```

### `PLAT_ROCKETCHIP`
```rust
const PLAT_ROCKETCHIP: bool = false;
```

### `PLAT_ROCKPRO64`
```rust
const PLAT_ROCKPRO64: bool = false;
```

### `PLAT_SPIKE`
```rust
const PLAT_SPIKE: bool = false;
```

### `PLAT_STAR64`
```rust
const PLAT_STAR64: bool = false;
```

### `PLAT_TK1`
```rust
const PLAT_TK1: bool = false;
```

### `PLAT_TQMA8XQP1GB`
```rust
const PLAT_TQMA8XQP1GB: bool = false;
```

### `PLAT_TX1`
```rust
const PLAT_TX1: bool = false;
```

### `PLAT_TX2`
```rust
const PLAT_TX2: bool = false;
```

### `PLAT_ZYNQ7000`
```rust
const PLAT_ZYNQ7000: bool = false;
```

### `PLAT_ZYNQMP`
```rust
const PLAT_ZYNQMP: bool = false;
```

### `PRINTING`
```rust
const PRINTING: bool = true;
```

### `RESET_CHUNK_BITS`
```rust
const RESET_CHUNK_BITS: &str;
```

### `RETYPE_FAN_OUT_LIMIT`
```rust
const RETYPE_FAN_OUT_LIMIT: &str;
```

### `ROOT_CNODE_SIZE_BITS`
```rust
const ROOT_CNODE_SIZE_BITS: &str;
```

### `SEL4_ARCH`
```rust
const SEL4_ARCH: &str;
```

### `SET_TLS_BASE_SELF`
```rust
const SET_TLS_BASE_SELF: bool = false;
```

### `SIGNAL_FASTPATH`
```rust
const SIGNAL_FASTPATH: bool = false;
```

### `SUPPORT_PCID`
```rust
const SUPPORT_PCID: bool = true;
```

### `SYSCALL`
```rust
const SYSCALL: bool = true;
```

### `SYSENTER`
```rust
const SYSENTER: bool = false;
```

### `TIMER_TICK_MS`
```rust
const TIMER_TICK_MS: &str;
```

### `TIME_SLICE`
```rust
const TIME_SLICE: &str;
```

### `USER_STACK_TRACE_LENGTH`
```rust
const USER_STACK_TRACE_LENGTH: &str;
```

### `USE_LOGICAL_IDS`
```rust
const USE_LOGICAL_IDS: bool = false;
```

### `VERIFICATION_BUILD`
```rust
const VERIFICATION_BUILD: bool = false;
```

### `VTX`
```rust
const VTX: bool = false;
```

### `WORD_SIZE`
```rust
const WORD_SIZE: &str;
```

### `X2APIC`
```rust
const X2APIC: bool = false;
```

### `X86_64_VTX_64BIT_GUESTS`
```rust
const X86_64_VTX_64BIT_GUESTS: bool = false;
```

### `XAPIC`
```rust
const XAPIC: bool = true;
```

### `XSAVE`
```rust
const XSAVE: bool = true;
```

### `XSAVE_FEATURE_SET`
```rust
const XSAVE_FEATURE_SET: &str;
```

### `XSAVE_SIZE`
```rust
const XSAVE_SIZE: &str;
```

### `XSAVE_XSAVE`
```rust
const XSAVE_XSAVE: bool = true;
```

### `XSAVE_XSAVEC`
```rust
const XSAVE_XSAVEC: bool = false;
```

### `XSAVE_XSAVEOPT`
```rust
const XSAVE_XSAVEOPT: bool = false;
```

### `XSAVE_XSAVES`
```rust
const XSAVE_XSAVES: bool = false;
```

