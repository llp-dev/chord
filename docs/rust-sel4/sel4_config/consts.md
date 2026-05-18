**sel4_config > consts**

# Module: consts

## Contents

**Constants**

- [`AARCH64_SERROR_IGNORE`](#aarch64_serror_ignore)
- [`ARCH`](#arch)
- [`ARCH_AARCH32`](#arch_aarch32)
- [`ARCH_AARCH64`](#arch_aarch64)
- [`ARCH_ARM`](#arch_arm)
- [`ARCH_ARM_HYP`](#arch_arm_hyp)
- [`ARCH_ARM_V7A`](#arch_arm_v7a)
- [`ARCH_ARM_V7VE`](#arch_arm_v7ve)
- [`ARCH_ARM_V8A`](#arch_arm_v8a)
- [`ARCH_IA32`](#arch_ia32)
- [`ARCH_RISCV`](#arch_riscv)
- [`ARCH_RISCV32`](#arch_riscv32)
- [`ARCH_RISCV64`](#arch_riscv64)
- [`ARCH_X86`](#arch_x86)
- [`ARCH_X86_64`](#arch_x86_64)
- [`ARCH_X86_BROADWELL`](#arch_x86_broadwell)
- [`ARCH_X86_GENERIC`](#arch_x86_generic)
- [`ARCH_X86_HASWELL`](#arch_x86_haswell)
- [`ARCH_X86_IVY`](#arch_x86_ivy)
- [`ARCH_X86_NEHALEM`](#arch_x86_nehalem)
- [`ARCH_X86_SANDY`](#arch_x86_sandy)
- [`ARCH_X86_SKYLAKE`](#arch_x86_skylake)
- [`ARCH_X86_WESTMERE`](#arch_x86_westmere)
- [`ARM_CORTEX_A15`](#arm_cortex_a15)
- [`ARM_CORTEX_A35`](#arm_cortex_a35)
- [`ARM_CORTEX_A53`](#arm_cortex_a53)
- [`ARM_CORTEX_A55`](#arm_cortex_a55)
- [`ARM_CORTEX_A57`](#arm_cortex_a57)
- [`ARM_CORTEX_A7`](#arm_cortex_a7)
- [`ARM_CORTEX_A72`](#arm_cortex_a72)
- [`ARM_CORTEX_A8`](#arm_cortex_a8)
- [`ARM_CORTEX_A9`](#arm_cortex_a9)
- [`ARM_HIKEY_OUTSTANDING_PREFETCHERS`](#arm_hikey_outstanding_prefetchers)
- [`ARM_HIKEY_PREFETCHER_NPFSTRM`](#arm_hikey_prefetcher_npfstrm)
- [`ARM_HIKEY_PREFETCHER_STBPFDIS`](#arm_hikey_prefetcher_stbpfdis)
- [`ARM_HIKEY_PREFETCHER_STBPFRS`](#arm_hikey_prefetcher_stbpfrs)
- [`ARM_HIKEY_PREFETCHER_STRIDE`](#arm_hikey_prefetcher_stride)
- [`BENCHMARK_GENERIC`](#benchmark_generic)
- [`BENCHMARK_TRACEPOINTS`](#benchmark_tracepoints)
- [`BENCHMARK_TRACK_KERNEL_ENTRIES`](#benchmark_track_kernel_entries)
- [`BENCHMARK_TRACK_UTILISATION`](#benchmark_track_utilisation)
- [`BINARY_VERIFICATION_BUILD`](#binary_verification_build)
- [`CACHE_LN_SZ`](#cache_ln_sz)
- [`CLZ_32`](#clz_32)
- [`CLZ_64`](#clz_64)
- [`CLZ_NO_BUILTIN`](#clz_no_builtin)
- [`COLOUR_PRINTING`](#colour_printing)
- [`CTZ_32`](#ctz_32)
- [`CTZ_64`](#ctz_64)
- [`CTZ_NO_BUILTIN`](#ctz_no_builtin)
- [`DANGEROUS_CODE_INJECTION`](#dangerous_code_injection)
- [`DEBUG_BUILD`](#debug_build)
- [`DEBUG_DISABLE_PREFETCHERS`](#debug_disable_prefetchers)
- [`ENABLE_BENCHMARKS`](#enable_benchmarks)
- [`ENABLE_SMP_SUPPORT`](#enable_smp_support)
- [`EXCEPTION_FASTPATH`](#exception_fastpath)
- [`EXPORT_PMC_USER`](#export_pmc_user)
- [`FASTPATH`](#fastpath)
- [`FSGSBASE_GDT`](#fsgsbase_gdt)
- [`FSGSBASE_INST`](#fsgsbase_inst)
- [`FSGSBASE_MSR`](#fsgsbase_msr)
- [`FXSAVE`](#fxsave)
- [`HARDWARE_DEBUG_API`](#hardware_debug_api)
- [`HAVE_FPU`](#have_fpu)
- [`HUGE_PAGE`](#huge_page)
- [`IOMMU`](#iommu)
- [`IRQ_IOAPIC`](#irq_ioapic)
- [`IRQ_PIC`](#irq_pic)
- [`IRQ_REPORTING`](#irq_reporting)
- [`KERNEL_BENCHMARK`](#kernel_benchmark)
- [`KERNEL_FSGS_BASE`](#kernel_fsgs_base)
- [`KERNEL_FWHOLE_PROGRAM`](#kernel_fwhole_program)
- [`KERNEL_INVOCATION_REPORT_ERROR_IPC`](#kernel_invocation_report_error_ipc)
- [`KERNEL_IRQ_CONTROLLER`](#kernel_irq_controller)
- [`KERNEL_LAPIC_MODE`](#kernel_lapic_mode)
- [`KERNEL_LOG_BUFFER`](#kernel_log_buffer)
- [`KERNEL_MCS`](#kernel_mcs)
- [`KERNEL_MUTLTIBOOT_GFX_MODE`](#kernel_mutltiboot_gfx_mode)
- [`KERNEL_OPTIMISATION_CLONE_FUNCTIONS`](#kernel_optimisation_clone_functions)
- [`KERNEL_OPT_LEVEL`](#kernel_opt_level)
- [`KERNEL_OPT_LEVEL_O0`](#kernel_opt_level_o0)
- [`KERNEL_OPT_LEVEL_O1`](#kernel_opt_level_o1)
- [`KERNEL_OPT_LEVEL_O2`](#kernel_opt_level_o2)
- [`KERNEL_OPT_LEVEL_O3`](#kernel_opt_level_o3)
- [`KERNEL_OPT_LEVEL_OS`](#kernel_opt_level_os)
- [`KERNEL_SKIM_WINDOW`](#kernel_skim_window)
- [`KERNEL_STACK_BITS`](#kernel_stack_bits)
- [`KERNEL_X86_DANGEROUS_MSR`](#kernel_x86_dangerous_msr)
- [`KERNEL_X86_FPU`](#kernel_x86_fpu)
- [`KERNEL_X86_IBPB_ON_CONTEXT_SWITCH`](#kernel_x86_ibpb_on_context_switch)
- [`KERNEL_X86_IBRS`](#kernel_x86_ibrs)
- [`KERNEL_X86_IBRS_ALL`](#kernel_x86_ibrs_all)
- [`KERNEL_X86_IBRS_BASIC`](#kernel_x86_ibrs_basic)
- [`KERNEL_X86_IBRS_NONE`](#kernel_x86_ibrs_none)
- [`KERNEL_X86_IBRS_STIBP`](#kernel_x86_ibrs_stibp)
- [`KERNEL_X86_MICRO_ARCH`](#kernel_x86_micro_arch)
- [`KERNEL_X86_RSB_ON_CONTEXT_SWITCH`](#kernel_x86_rsb_on_context_switch)
- [`KERNEL_X86_SYSCALL`](#kernel_x86_syscall)
- [`KERNEL_XSAVE`](#kernel_xsave)
- [`LIB_SEL4_DEFAULT_FUNCTION_ATTRIBUTES`](#lib_sel4_default_function_attributes)
- [`LIB_SEL4_FUNCTION_ATTRIBUTE`](#lib_sel4_function_attribute)
- [`LIB_SEL4_INLINE_INVOCATIONS`](#lib_sel4_inline_invocations)
- [`LIB_SEL4_PRINT_INVOCATION_ERRORS`](#lib_sel4_print_invocation_errors)
- [`LIB_SEL4_PUBLIC_SYMBOLS`](#lib_sel4_public_symbols)
- [`LIB_SEL4_STUBS_USE_IPC_BUFFER_ONLY`](#lib_sel4_stubs_use_ipc_buffer_only)
- [`LIB_SEL4_USE_THREAD_LOCALS`](#lib_sel4_use_thread_locals)
- [`MAX_NUM_BOOTINFO_UNTYPED_CAPS`](#max_num_bootinfo_untyped_caps)
- [`MAX_NUM_IOAPIC`](#max_num_ioapic)
- [`MAX_NUM_NODES`](#max_num_nodes)
- [`MAX_NUM_TRACE_POINTS`](#max_num_trace_points)
- [`MAX_NUM_WORK_UNITS_PER_PREEMPTION`](#max_num_work_units_per_preemption)
- [`MAX_RMRR_ENTRIES`](#max_rmrr_entries)
- [`MAX_VPIDS`](#max_vpids)
- [`MULTIBOOT1_HEADER`](#multiboot1_header)
- [`MULTIBOOT2_HEADER`](#multiboot2_header)
- [`MULTIBOOT_GRAPHICS_MODE_LINEAR`](#multiboot_graphics_mode_linear)
- [`MULTIBOOT_GRAPHICS_MODE_NONE`](#multiboot_graphics_mode_none)
- [`MULTIBOOT_GRAPHICS_MODE_TEXT`](#multiboot_graphics_mode_text)
- [`NO_BENCHMARKS`](#no_benchmarks)
- [`NUM_DOMAINS`](#num_domains)
- [`NUM_DOMAIN_SCHEDULES`](#num_domain_schedules)
- [`NUM_PRIORITIES`](#num_priorities)
- [`PADDR_USER_DEVICE_TOP`](#paddr_user_device_top)
- [`PC99_TSC_FREQUENCY`](#pc99_tsc_frequency)
- [`PLAT`](#plat)
- [`PLAT_ALLWINNERA20`](#plat_allwinnera20)
- [`PLAT_AM335X`](#plat_am335x)
- [`PLAT_APQ8064`](#plat_apq8064)
- [`PLAT_ARIANE`](#plat_ariane)
- [`PLAT_BANANAPIF3`](#plat_bananapif3)
- [`PLAT_BCM2711`](#plat_bcm2711)
- [`PLAT_BCM2837`](#plat_bcm2837)
- [`PLAT_CHESHIRE`](#plat_cheshire)
- [`PLAT_EXYNOS4`](#plat_exynos4)
- [`PLAT_EXYNOS5`](#plat_exynos5)
- [`PLAT_FVP`](#plat_fvp)
- [`PLAT_HIFIVE`](#plat_hifive)
- [`PLAT_HIFIVE_P550`](#plat_hifive_p550)
- [`PLAT_HIKEY`](#plat_hikey)
- [`PLAT_IMX6`](#plat_imx6)
- [`PLAT_IMX7`](#plat_imx7)
- [`PLAT_IMX7_SABRE`](#plat_imx7_sabre)
- [`PLAT_IMX8MM_EVK`](#plat_imx8mm_evk)
- [`PLAT_IMX8MP_EVK`](#plat_imx8mp_evk)
- [`PLAT_IMX8MQ_EVK`](#plat_imx8mq_evk)
- [`PLAT_IMX93`](#plat_imx93)
- [`PLAT_MAAXBOARD`](#plat_maaxboard)
- [`PLAT_ODROIDC2`](#plat_odroidc2)
- [`PLAT_ODROIDC4`](#plat_odroidc4)
- [`PLAT_OMAP3`](#plat_omap3)
- [`PLAT_PC99`](#plat_pc99)
- [`PLAT_POLARFIRE`](#plat_polarfire)
- [`PLAT_QEMU_ARM_VIRT`](#plat_qemu_arm_virt)
- [`PLAT_QEMU_RISCV_VIRT`](#plat_qemu_riscv_virt)
- [`PLAT_QUARTZ64`](#plat_quartz64)
- [`PLAT_RK3568`](#plat_rk3568)
- [`PLAT_ROCKETCHIP`](#plat_rocketchip)
- [`PLAT_ROCKPRO64`](#plat_rockpro64)
- [`PLAT_SPIKE`](#plat_spike)
- [`PLAT_STAR64`](#plat_star64)
- [`PLAT_TK1`](#plat_tk1)
- [`PLAT_TQMA8XQP1GB`](#plat_tqma8xqp1gb)
- [`PLAT_TX1`](#plat_tx1)
- [`PLAT_TX2`](#plat_tx2)
- [`PLAT_ZYNQ7000`](#plat_zynq7000)
- [`PLAT_ZYNQMP`](#plat_zynqmp)
- [`PRINTING`](#printing)
- [`RESET_CHUNK_BITS`](#reset_chunk_bits)
- [`RETYPE_FAN_OUT_LIMIT`](#retype_fan_out_limit)
- [`ROOT_CNODE_SIZE_BITS`](#root_cnode_size_bits)
- [`SEL4_ARCH`](#sel4_arch)
- [`SET_TLS_BASE_SELF`](#set_tls_base_self)
- [`SIGNAL_FASTPATH`](#signal_fastpath)
- [`SUPPORT_PCID`](#support_pcid)
- [`SYSCALL`](#syscall)
- [`SYSENTER`](#sysenter)
- [`TIMER_TICK_MS`](#timer_tick_ms)
- [`TIME_SLICE`](#time_slice)
- [`USER_STACK_TRACE_LENGTH`](#user_stack_trace_length)
- [`USE_LOGICAL_IDS`](#use_logical_ids)
- [`VERIFICATION_BUILD`](#verification_build)
- [`VTX`](#vtx)
- [`WORD_SIZE`](#word_size)
- [`X2APIC`](#x2apic)
- [`X86_64_VTX_64BIT_GUESTS`](#x86_64_vtx_64bit_guests)
- [`XAPIC`](#xapic)
- [`XSAVE`](#xsave)
- [`XSAVE_FEATURE_SET`](#xsave_feature_set)
- [`XSAVE_SIZE`](#xsave_size)
- [`XSAVE_XSAVE`](#xsave_xsave)
- [`XSAVE_XSAVEC`](#xsave_xsavec)
- [`XSAVE_XSAVEOPT`](#xsave_xsaveopt)
- [`XSAVE_XSAVES`](#xsave_xsaves)

---

## sel4_config::consts::AARCH64_SERROR_IGNORE

*Constant*: `bool`



## sel4_config::consts::ARCH

*Constant*: `&str`



## sel4_config::consts::ARCH_AARCH32

*Constant*: `bool`



## sel4_config::consts::ARCH_AARCH64

*Constant*: `bool`



## sel4_config::consts::ARCH_ARM

*Constant*: `bool`



## sel4_config::consts::ARCH_ARM_HYP

*Constant*: `bool`



## sel4_config::consts::ARCH_ARM_V7A

*Constant*: `bool`



## sel4_config::consts::ARCH_ARM_V7VE

*Constant*: `bool`



## sel4_config::consts::ARCH_ARM_V8A

*Constant*: `bool`



## sel4_config::consts::ARCH_IA32

*Constant*: `bool`



## sel4_config::consts::ARCH_RISCV

*Constant*: `bool`



## sel4_config::consts::ARCH_RISCV32

*Constant*: `bool`



## sel4_config::consts::ARCH_RISCV64

*Constant*: `bool`



## sel4_config::consts::ARCH_X86

*Constant*: `bool`



## sel4_config::consts::ARCH_X86_64

*Constant*: `bool`



## sel4_config::consts::ARCH_X86_BROADWELL

*Constant*: `bool`



## sel4_config::consts::ARCH_X86_GENERIC

*Constant*: `bool`



## sel4_config::consts::ARCH_X86_HASWELL

*Constant*: `bool`



## sel4_config::consts::ARCH_X86_IVY

*Constant*: `bool`



## sel4_config::consts::ARCH_X86_NEHALEM

*Constant*: `bool`



## sel4_config::consts::ARCH_X86_SANDY

*Constant*: `bool`



## sel4_config::consts::ARCH_X86_SKYLAKE

*Constant*: `bool`



## sel4_config::consts::ARCH_X86_WESTMERE

*Constant*: `bool`



## sel4_config::consts::ARM_CORTEX_A15

*Constant*: `bool`



## sel4_config::consts::ARM_CORTEX_A35

*Constant*: `bool`



## sel4_config::consts::ARM_CORTEX_A53

*Constant*: `bool`



## sel4_config::consts::ARM_CORTEX_A55

*Constant*: `bool`



## sel4_config::consts::ARM_CORTEX_A57

*Constant*: `bool`



## sel4_config::consts::ARM_CORTEX_A7

*Constant*: `bool`



## sel4_config::consts::ARM_CORTEX_A72

*Constant*: `bool`



## sel4_config::consts::ARM_CORTEX_A8

*Constant*: `bool`



## sel4_config::consts::ARM_CORTEX_A9

*Constant*: `bool`



## sel4_config::consts::ARM_HIKEY_OUTSTANDING_PREFETCHERS

*Constant*: `&str`



## sel4_config::consts::ARM_HIKEY_PREFETCHER_NPFSTRM

*Constant*: `&str`



## sel4_config::consts::ARM_HIKEY_PREFETCHER_STBPFDIS

*Constant*: `bool`



## sel4_config::consts::ARM_HIKEY_PREFETCHER_STBPFRS

*Constant*: `bool`



## sel4_config::consts::ARM_HIKEY_PREFETCHER_STRIDE

*Constant*: `&str`



## sel4_config::consts::BENCHMARK_GENERIC

*Constant*: `bool`



## sel4_config::consts::BENCHMARK_TRACEPOINTS

*Constant*: `bool`



## sel4_config::consts::BENCHMARK_TRACK_KERNEL_ENTRIES

*Constant*: `bool`



## sel4_config::consts::BENCHMARK_TRACK_UTILISATION

*Constant*: `bool`



## sel4_config::consts::BINARY_VERIFICATION_BUILD

*Constant*: `bool`



## sel4_config::consts::CACHE_LN_SZ

*Constant*: `&str`



## sel4_config::consts::CLZ_32

*Constant*: `bool`



## sel4_config::consts::CLZ_64

*Constant*: `bool`



## sel4_config::consts::CLZ_NO_BUILTIN

*Constant*: `bool`



## sel4_config::consts::COLOUR_PRINTING

*Constant*: `bool`



## sel4_config::consts::CTZ_32

*Constant*: `bool`



## sel4_config::consts::CTZ_64

*Constant*: `bool`



## sel4_config::consts::CTZ_NO_BUILTIN

*Constant*: `bool`



## sel4_config::consts::DANGEROUS_CODE_INJECTION

*Constant*: `bool`



## sel4_config::consts::DEBUG_BUILD

*Constant*: `bool`



## sel4_config::consts::DEBUG_DISABLE_PREFETCHERS

*Constant*: `bool`



## sel4_config::consts::ENABLE_BENCHMARKS

*Constant*: `bool`



## sel4_config::consts::ENABLE_SMP_SUPPORT

*Constant*: `bool`



## sel4_config::consts::EXCEPTION_FASTPATH

*Constant*: `bool`



## sel4_config::consts::EXPORT_PMC_USER

*Constant*: `bool`



## sel4_config::consts::FASTPATH

*Constant*: `bool`



## sel4_config::consts::FSGSBASE_GDT

*Constant*: `bool`



## sel4_config::consts::FSGSBASE_INST

*Constant*: `bool`



## sel4_config::consts::FSGSBASE_MSR

*Constant*: `bool`



## sel4_config::consts::FXSAVE

*Constant*: `bool`



## sel4_config::consts::HARDWARE_DEBUG_API

*Constant*: `bool`



## sel4_config::consts::HAVE_FPU

*Constant*: `bool`



## sel4_config::consts::HUGE_PAGE

*Constant*: `bool`



## sel4_config::consts::IOMMU

*Constant*: `bool`



## sel4_config::consts::IRQ_IOAPIC

*Constant*: `bool`



## sel4_config::consts::IRQ_PIC

*Constant*: `bool`



## sel4_config::consts::IRQ_REPORTING

*Constant*: `bool`



## sel4_config::consts::KERNEL_BENCHMARK

*Constant*: `&str`



## sel4_config::consts::KERNEL_FSGS_BASE

*Constant*: `&str`



## sel4_config::consts::KERNEL_FWHOLE_PROGRAM

*Constant*: `bool`



## sel4_config::consts::KERNEL_INVOCATION_REPORT_ERROR_IPC

*Constant*: `bool`



## sel4_config::consts::KERNEL_IRQ_CONTROLLER

*Constant*: `&str`



## sel4_config::consts::KERNEL_LAPIC_MODE

*Constant*: `&str`



## sel4_config::consts::KERNEL_LOG_BUFFER

*Constant*: `bool`



## sel4_config::consts::KERNEL_MCS

*Constant*: `bool`



## sel4_config::consts::KERNEL_MUTLTIBOOT_GFX_MODE

*Constant*: `&str`



## sel4_config::consts::KERNEL_OPTIMISATION_CLONE_FUNCTIONS

*Constant*: `bool`



## sel4_config::consts::KERNEL_OPT_LEVEL

*Constant*: `&str`



## sel4_config::consts::KERNEL_OPT_LEVEL_O0

*Constant*: `bool`



## sel4_config::consts::KERNEL_OPT_LEVEL_O1

*Constant*: `bool`



## sel4_config::consts::KERNEL_OPT_LEVEL_O2

*Constant*: `bool`



## sel4_config::consts::KERNEL_OPT_LEVEL_O3

*Constant*: `bool`



## sel4_config::consts::KERNEL_OPT_LEVEL_OS

*Constant*: `bool`



## sel4_config::consts::KERNEL_SKIM_WINDOW

*Constant*: `bool`



## sel4_config::consts::KERNEL_STACK_BITS

*Constant*: `&str`



## sel4_config::consts::KERNEL_X86_DANGEROUS_MSR

*Constant*: `bool`



## sel4_config::consts::KERNEL_X86_FPU

*Constant*: `&str`



## sel4_config::consts::KERNEL_X86_IBPB_ON_CONTEXT_SWITCH

*Constant*: `bool`



## sel4_config::consts::KERNEL_X86_IBRS

*Constant*: `&str`



## sel4_config::consts::KERNEL_X86_IBRS_ALL

*Constant*: `bool`



## sel4_config::consts::KERNEL_X86_IBRS_BASIC

*Constant*: `bool`



## sel4_config::consts::KERNEL_X86_IBRS_NONE

*Constant*: `bool`



## sel4_config::consts::KERNEL_X86_IBRS_STIBP

*Constant*: `bool`



## sel4_config::consts::KERNEL_X86_MICRO_ARCH

*Constant*: `&str`



## sel4_config::consts::KERNEL_X86_RSB_ON_CONTEXT_SWITCH

*Constant*: `bool`



## sel4_config::consts::KERNEL_X86_SYSCALL

*Constant*: `&str`



## sel4_config::consts::KERNEL_XSAVE

*Constant*: `&str`



## sel4_config::consts::LIB_SEL4_DEFAULT_FUNCTION_ATTRIBUTES

*Constant*: `bool`



## sel4_config::consts::LIB_SEL4_FUNCTION_ATTRIBUTE

*Constant*: `&str`



## sel4_config::consts::LIB_SEL4_INLINE_INVOCATIONS

*Constant*: `bool`



## sel4_config::consts::LIB_SEL4_PRINT_INVOCATION_ERRORS

*Constant*: `&str`



## sel4_config::consts::LIB_SEL4_PUBLIC_SYMBOLS

*Constant*: `bool`



## sel4_config::consts::LIB_SEL4_STUBS_USE_IPC_BUFFER_ONLY

*Constant*: `bool`



## sel4_config::consts::LIB_SEL4_USE_THREAD_LOCALS

*Constant*: `bool`



## sel4_config::consts::MAX_NUM_BOOTINFO_UNTYPED_CAPS

*Constant*: `&str`



## sel4_config::consts::MAX_NUM_IOAPIC

*Constant*: `&str`



## sel4_config::consts::MAX_NUM_NODES

*Constant*: `&str`



## sel4_config::consts::MAX_NUM_TRACE_POINTS

*Constant*: `&str`



## sel4_config::consts::MAX_NUM_WORK_UNITS_PER_PREEMPTION

*Constant*: `&str`



## sel4_config::consts::MAX_RMRR_ENTRIES

*Constant*: `&str`



## sel4_config::consts::MAX_VPIDS

*Constant*: `&str`



## sel4_config::consts::MULTIBOOT1_HEADER

*Constant*: `bool`



## sel4_config::consts::MULTIBOOT2_HEADER

*Constant*: `bool`



## sel4_config::consts::MULTIBOOT_GRAPHICS_MODE_LINEAR

*Constant*: `bool`



## sel4_config::consts::MULTIBOOT_GRAPHICS_MODE_NONE

*Constant*: `bool`



## sel4_config::consts::MULTIBOOT_GRAPHICS_MODE_TEXT

*Constant*: `bool`



## sel4_config::consts::NO_BENCHMARKS

*Constant*: `bool`



## sel4_config::consts::NUM_DOMAINS

*Constant*: `&str`



## sel4_config::consts::NUM_DOMAIN_SCHEDULES

*Constant*: `&str`



## sel4_config::consts::NUM_PRIORITIES

*Constant*: `&str`



## sel4_config::consts::PADDR_USER_DEVICE_TOP

*Constant*: `&str`



## sel4_config::consts::PC99_TSC_FREQUENCY

*Constant*: `&str`



## sel4_config::consts::PLAT

*Constant*: `&str`



## sel4_config::consts::PLAT_ALLWINNERA20

*Constant*: `bool`



## sel4_config::consts::PLAT_AM335X

*Constant*: `bool`



## sel4_config::consts::PLAT_APQ8064

*Constant*: `bool`



## sel4_config::consts::PLAT_ARIANE

*Constant*: `bool`



## sel4_config::consts::PLAT_BANANAPIF3

*Constant*: `bool`



## sel4_config::consts::PLAT_BCM2711

*Constant*: `bool`



## sel4_config::consts::PLAT_BCM2837

*Constant*: `bool`



## sel4_config::consts::PLAT_CHESHIRE

*Constant*: `bool`



## sel4_config::consts::PLAT_EXYNOS4

*Constant*: `bool`



## sel4_config::consts::PLAT_EXYNOS5

*Constant*: `bool`



## sel4_config::consts::PLAT_FVP

*Constant*: `bool`



## sel4_config::consts::PLAT_HIFIVE

*Constant*: `bool`



## sel4_config::consts::PLAT_HIFIVE_P550

*Constant*: `bool`



## sel4_config::consts::PLAT_HIKEY

*Constant*: `bool`



## sel4_config::consts::PLAT_IMX6

*Constant*: `bool`



## sel4_config::consts::PLAT_IMX7

*Constant*: `bool`



## sel4_config::consts::PLAT_IMX7_SABRE

*Constant*: `bool`



## sel4_config::consts::PLAT_IMX8MM_EVK

*Constant*: `bool`



## sel4_config::consts::PLAT_IMX8MP_EVK

*Constant*: `bool`



## sel4_config::consts::PLAT_IMX8MQ_EVK

*Constant*: `bool`



## sel4_config::consts::PLAT_IMX93

*Constant*: `bool`



## sel4_config::consts::PLAT_MAAXBOARD

*Constant*: `bool`



## sel4_config::consts::PLAT_ODROIDC2

*Constant*: `bool`



## sel4_config::consts::PLAT_ODROIDC4

*Constant*: `bool`



## sel4_config::consts::PLAT_OMAP3

*Constant*: `bool`



## sel4_config::consts::PLAT_PC99

*Constant*: `bool`



## sel4_config::consts::PLAT_POLARFIRE

*Constant*: `bool`



## sel4_config::consts::PLAT_QEMU_ARM_VIRT

*Constant*: `bool`



## sel4_config::consts::PLAT_QEMU_RISCV_VIRT

*Constant*: `bool`



## sel4_config::consts::PLAT_QUARTZ64

*Constant*: `bool`



## sel4_config::consts::PLAT_RK3568

*Constant*: `bool`



## sel4_config::consts::PLAT_ROCKETCHIP

*Constant*: `bool`



## sel4_config::consts::PLAT_ROCKPRO64

*Constant*: `bool`



## sel4_config::consts::PLAT_SPIKE

*Constant*: `bool`



## sel4_config::consts::PLAT_STAR64

*Constant*: `bool`



## sel4_config::consts::PLAT_TK1

*Constant*: `bool`



## sel4_config::consts::PLAT_TQMA8XQP1GB

*Constant*: `bool`



## sel4_config::consts::PLAT_TX1

*Constant*: `bool`



## sel4_config::consts::PLAT_TX2

*Constant*: `bool`



## sel4_config::consts::PLAT_ZYNQ7000

*Constant*: `bool`



## sel4_config::consts::PLAT_ZYNQMP

*Constant*: `bool`



## sel4_config::consts::PRINTING

*Constant*: `bool`



## sel4_config::consts::RESET_CHUNK_BITS

*Constant*: `&str`



## sel4_config::consts::RETYPE_FAN_OUT_LIMIT

*Constant*: `&str`



## sel4_config::consts::ROOT_CNODE_SIZE_BITS

*Constant*: `&str`



## sel4_config::consts::SEL4_ARCH

*Constant*: `&str`



## sel4_config::consts::SET_TLS_BASE_SELF

*Constant*: `bool`



## sel4_config::consts::SIGNAL_FASTPATH

*Constant*: `bool`



## sel4_config::consts::SUPPORT_PCID

*Constant*: `bool`



## sel4_config::consts::SYSCALL

*Constant*: `bool`



## sel4_config::consts::SYSENTER

*Constant*: `bool`



## sel4_config::consts::TIMER_TICK_MS

*Constant*: `&str`



## sel4_config::consts::TIME_SLICE

*Constant*: `&str`



## sel4_config::consts::USER_STACK_TRACE_LENGTH

*Constant*: `&str`



## sel4_config::consts::USE_LOGICAL_IDS

*Constant*: `bool`



## sel4_config::consts::VERIFICATION_BUILD

*Constant*: `bool`



## sel4_config::consts::VTX

*Constant*: `bool`



## sel4_config::consts::WORD_SIZE

*Constant*: `&str`



## sel4_config::consts::X2APIC

*Constant*: `bool`



## sel4_config::consts::X86_64_VTX_64BIT_GUESTS

*Constant*: `bool`



## sel4_config::consts::XAPIC

*Constant*: `bool`



## sel4_config::consts::XSAVE

*Constant*: `bool`



## sel4_config::consts::XSAVE_FEATURE_SET

*Constant*: `&str`



## sel4_config::consts::XSAVE_SIZE

*Constant*: `&str`



## sel4_config::consts::XSAVE_XSAVE

*Constant*: `bool`



## sel4_config::consts::XSAVE_XSAVEC

*Constant*: `bool`



## sel4_config::consts::XSAVE_XSAVEOPT

*Constant*: `bool`



## sel4_config::consts::XSAVE_XSAVES

*Constant*: `bool`



