**object > elf**

# Module: elf

## Contents

**Structs**

- [`CompressionHeader32`](#compressionheader32) - Section compression header.
- [`CompressionHeader64`](#compressionheader64) - Section compression header.
- [`Dyn32`](#dyn32) - Dynamic section entry.
- [`Dyn64`](#dyn64) - Dynamic section entry.
- [`FileHeader32`](#fileheader32) - The header at the start of every 32-bit ELF file.
- [`FileHeader64`](#fileheader64) - The header at the start of every 64-bit ELF file.
- [`GnuHashHeader`](#gnuhashheader) - Header of `SHT_GNU_HASH` section.
- [`HashHeader`](#hashheader) - Header of `SHT_HASH` section.
- [`Ident`](#ident) - Magic number and other information.
- [`NoteHeader32`](#noteheader32) - Note section entry header.
- [`NoteHeader64`](#noteheader64) - Note section entry header.
- [`ProgramHeader32`](#programheader32) - Program segment header.
- [`ProgramHeader64`](#programheader64) - Program segment header.
- [`Rel32`](#rel32) - Relocation table entry without explicit addend.
- [`Rel64`](#rel64) - Relocation table entry without explicit addend.
- [`Rela32`](#rela32) - Relocation table entry with explicit addend.
- [`Rela64`](#rela64) - Relocation table entry with explicit addend.
- [`Relr32`](#relr32) - 32-bit relative relocation table entry.
- [`Relr64`](#relr64) - 64-bit relative relocation table entry.
- [`SectionHeader32`](#sectionheader32) - Section header.
- [`SectionHeader64`](#sectionheader64) - Section header.
- [`Sym32`](#sym32) - Symbol table entry.
- [`Sym64`](#sym64) - Symbol table entry.
- [`Syminfo32`](#syminfo32) - Additional information about a `Sym32`.
- [`Syminfo64`](#syminfo64) - Additional information about a `Sym64`.
- [`Verdaux`](#verdaux) - Auxiliary version information.
- [`Verdef`](#verdef) - Version definition sections
- [`Vernaux`](#vernaux) - Auxiliary needed version information.
- [`Verneed`](#verneed) - Version dependency.
- [`Versym`](#versym) - Version symbol information

**Functions**

- [`ef_e2k_flag_to_mach`](#ef_e2k_flag_to_mach) - Decode `E_E2K_MACH_*` from `FileHeader*::e_flags`.
- [`ef_e2k_mach_to_flag`](#ef_e2k_mach_to_flag) - Encode `E_E2K_MACH_*` into `FileHeader*::e_flags`.
- [`gnu_hash`](#gnu_hash) - Calculate the GNU hash for a symbol name.
- [`hash`](#hash) - Calculate the SysV hash for a symbol name.

**Constants**

- [`DF_1_CONFALT`](#df_1_confalt) - Configuration alternative created.
- [`DF_1_DIRECT`](#df_1_direct) - Direct binding enabled.
- [`DF_1_DISPRELDNE`](#df_1_dispreldne) - Disp reloc applied at build time.
- [`DF_1_DISPRELPND`](#df_1_disprelpnd) - Disp reloc applied at run-time.
- [`DF_1_EDITED`](#df_1_edited) - Object is modified after built.
- [`DF_1_ENDFILTEE`](#df_1_endfiltee) - Filtee terminates filters search.
- [`DF_1_GLOBAL`](#df_1_global) - Set RTLD_GLOBAL for this object.
- [`DF_1_GLOBAUDIT`](#df_1_globaudit) - Global auditing required.
- [`DF_1_GROUP`](#df_1_group) - Set RTLD_GROUP for this object.
- [`DF_1_IGNMULDEF`](#df_1_ignmuldef)
- [`DF_1_INITFIRST`](#df_1_initfirst) - Set RTLD_INITFIRST for this object.
- [`DF_1_INTERPOSE`](#df_1_interpose) - Object is used to interpose.
- [`DF_1_LOADFLTR`](#df_1_loadfltr) - Trigger filtee loading at runtime.
- [`DF_1_NODEFLIB`](#df_1_nodeflib) - Ignore default lib search path.
- [`DF_1_NODELETE`](#df_1_nodelete) - Set RTLD_NODELETE for this object.
- [`DF_1_NODIRECT`](#df_1_nodirect) - Object has no-direct binding.
- [`DF_1_NODUMP`](#df_1_nodump) - Object can't be dldump'ed.
- [`DF_1_NOHDR`](#df_1_nohdr)
- [`DF_1_NOKSYMS`](#df_1_noksyms)
- [`DF_1_NOOPEN`](#df_1_noopen) - Set RTLD_NOOPEN for this object.
- [`DF_1_NORELOC`](#df_1_noreloc)
- [`DF_1_NOW`](#df_1_now) - Set RTLD_NOW for this object.
- [`DF_1_ORIGIN`](#df_1_origin) - $ORIGIN must be handled.
- [`DF_1_PIE`](#df_1_pie)
- [`DF_1_SINGLETON`](#df_1_singleton) - Singleton symbols are used.
- [`DF_1_STUB`](#df_1_stub)
- [`DF_1_SYMINTPOSE`](#df_1_symintpose) - Object has individual interposers.
- [`DF_1_TRANS`](#df_1_trans)
- [`DF_BIND_NOW`](#df_bind_now) - No lazy binding for this object
- [`DF_ORIGIN`](#df_origin) - Object may use DF_ORIGIN
- [`DF_STATIC_TLS`](#df_static_tls) - Module uses the static TLS model
- [`DF_SYMBOLIC`](#df_symbolic) - Symbol resolutions starts here
- [`DF_TEXTREL`](#df_textrel) - Object contains text relocations
- [`DT_AARCH64_BTI_PLT`](#dt_aarch64_bti_plt)
- [`DT_AARCH64_NUM`](#dt_aarch64_num)
- [`DT_AARCH64_PAC_PLT`](#dt_aarch64_pac_plt)
- [`DT_AARCH64_VARIANT_PCS`](#dt_aarch64_variant_pcs)
- [`DT_ADDRRNGHI`](#dt_addrrnghi)
- [`DT_ADDRRNGLO`](#dt_addrrnglo)
- [`DT_ALPHA_PLTRO`](#dt_alpha_pltro)
- [`DT_AUDIT`](#dt_audit) - Object auditing.
- [`DT_AUXILIARY`](#dt_auxiliary) - Shared object to load before self
- [`DT_BIND_NOW`](#dt_bind_now) - Process relocations of object
- [`DT_CHECKSUM`](#dt_checksum)
- [`DT_CONFIG`](#dt_config) - Configuration information.
- [`DT_DEBUG`](#dt_debug) - For debugging; unspecified
- [`DT_DEPAUDIT`](#dt_depaudit) - Dependency auditing.
- [`DT_E2K_EXPORT_PL`](#dt_e2k_export_pl)
- [`DT_E2K_EXPORT_PLSZ`](#dt_e2k_export_plsz)
- [`DT_E2K_INIT_GOT`](#dt_e2k_init_got)
- [`DT_E2K_LAZY`](#dt_e2k_lazy)
- [`DT_E2K_LAZY_GOT`](#dt_e2k_lazy_got)
- [`DT_E2K_NO_SELFINIT`](#dt_e2k_no_selfinit)
- [`DT_E2K_NUM`](#dt_e2k_num)
- [`DT_E2K_REAL_PLTGOT`](#dt_e2k_real_pltgot)
- [`DT_ENCODING`](#dt_encoding) - Start of encoded range
- [`DT_FEATURE_1`](#dt_feature_1) - Feature selection (DTF_*).
- [`DT_FILTER`](#dt_filter) - Shared object to get values from
- [`DT_FINI`](#dt_fini) - Address of termination function
- [`DT_FINI_ARRAY`](#dt_fini_array) - Array with addresses of fini fct
- [`DT_FINI_ARRAYSZ`](#dt_fini_arraysz) - Size in bytes of DT_FINI_ARRAY
- [`DT_FLAGS`](#dt_flags) - Flags for the object being loaded
- [`DT_FLAGS_1`](#dt_flags_1) - State flags, see DF_1_* below.
- [`DT_GNU_CONFLICT`](#dt_gnu_conflict) - Start of conflict section
- [`DT_GNU_CONFLICTSZ`](#dt_gnu_conflictsz) - Size of conflict section
- [`DT_GNU_HASH`](#dt_gnu_hash) - GNU-style hash table.
- [`DT_GNU_LIBLIST`](#dt_gnu_liblist) - Library list
- [`DT_GNU_LIBLISTSZ`](#dt_gnu_liblistsz) - Size of library list
- [`DT_GNU_PRELINKED`](#dt_gnu_prelinked) - Prelinking timestamp
- [`DT_HASH`](#dt_hash) - Address of symbol hash table
- [`DT_HIOS`](#dt_hios) - End of OS-specific
- [`DT_HIPROC`](#dt_hiproc) - End of processor-specific
- [`DT_IA_64_PLT_RESERVE`](#dt_ia_64_plt_reserve)
- [`DT_INIT`](#dt_init) - Address of init function
- [`DT_INIT_ARRAY`](#dt_init_array) - Array with addresses of init fct
- [`DT_INIT_ARRAYSZ`](#dt_init_arraysz) - Size in bytes of DT_INIT_ARRAY
- [`DT_JMPREL`](#dt_jmprel) - Address of PLT relocs
- [`DT_LOOS`](#dt_loos) - Start of OS-specific
- [`DT_LOPROC`](#dt_loproc) - Start of processor-specific
- [`DT_MIPS_AUX_DYNAMIC`](#dt_mips_aux_dynamic) - Address of aux .dynamic.
- [`DT_MIPS_BASE_ADDRESS`](#dt_mips_base_address) - Base address
- [`DT_MIPS_COMPACT_SIZE`](#dt_mips_compact_size) - (O32)Size of compact rel section.
- [`DT_MIPS_CONFLICT`](#dt_mips_conflict) - Address of CONFLICT section
- [`DT_MIPS_CONFLICTNO`](#dt_mips_conflictno) - Number of CONFLICT entries
- [`DT_MIPS_CXX_FLAGS`](#dt_mips_cxx_flags) - Flags indicating for C++ flavor.
- [`DT_MIPS_DELTA_CLASS`](#dt_mips_delta_class) - Delta C++ class definition.
- [`DT_MIPS_DELTA_CLASSSYM`](#dt_mips_delta_classsym) - Delta symbols that hold the class declaration.
- [`DT_MIPS_DELTA_CLASSSYM_NO`](#dt_mips_delta_classsym_no) - Number of entries in DT_MIPS_DELTA_CLASSSYM.
- [`DT_MIPS_DELTA_CLASS_NO`](#dt_mips_delta_class_no) - Number of entries in DT_MIPS_DELTA_CLASS.
- [`DT_MIPS_DELTA_INSTANCE`](#dt_mips_delta_instance) - Delta C++ class instances.
- [`DT_MIPS_DELTA_INSTANCE_NO`](#dt_mips_delta_instance_no) - Number of entries in DT_MIPS_DELTA_INSTANCE.
- [`DT_MIPS_DELTA_RELOC`](#dt_mips_delta_reloc) - Delta relocations.
- [`DT_MIPS_DELTA_RELOC_NO`](#dt_mips_delta_reloc_no) - Number of entries in DT_MIPS_DELTA_RELOC.
- [`DT_MIPS_DELTA_SYM`](#dt_mips_delta_sym) - Delta symbols that Delta relocations refer to.
- [`DT_MIPS_DELTA_SYM_NO`](#dt_mips_delta_sym_no) - Number of entries in DT_MIPS_DELTA_SYM.
- [`DT_MIPS_DYNSTR_ALIGN`](#dt_mips_dynstr_align)
- [`DT_MIPS_FLAGS`](#dt_mips_flags) - Flags
- [`DT_MIPS_GOTSYM`](#dt_mips_gotsym) - First GOT entry in DYNSYM
- [`DT_MIPS_GP_VALUE`](#dt_mips_gp_value) - GP value for aux GOTs.
- [`DT_MIPS_HIDDEN_GOTIDX`](#dt_mips_hidden_gotidx)
- [`DT_MIPS_HIPAGENO`](#dt_mips_hipageno) - Number of GOT page table entries
- [`DT_MIPS_ICHECKSUM`](#dt_mips_ichecksum) - Checksum
- [`DT_MIPS_INTERFACE`](#dt_mips_interface) - Address of .interface.
- [`DT_MIPS_INTERFACE_SIZE`](#dt_mips_interface_size) - Size of the .interface section.
- [`DT_MIPS_IVERSION`](#dt_mips_iversion) - Version string (string tbl index)
- [`DT_MIPS_LIBLIST`](#dt_mips_liblist) - Address of LIBLIST section
- [`DT_MIPS_LIBLISTNO`](#dt_mips_liblistno) - Number of LIBLIST entries
- [`DT_MIPS_LOCALPAGE_GOTIDX`](#dt_mips_localpage_gotidx)
- [`DT_MIPS_LOCAL_GOTIDX`](#dt_mips_local_gotidx)
- [`DT_MIPS_LOCAL_GOTNO`](#dt_mips_local_gotno) - Number of local GOT entries
- [`DT_MIPS_MSYM`](#dt_mips_msym)
- [`DT_MIPS_OPTIONS`](#dt_mips_options) - Address of .options.
- [`DT_MIPS_PERF_SUFFIX`](#dt_mips_perf_suffix) - Default suffix of dso to be added by rld on dlopen() calls.
- [`DT_MIPS_PIXIE_INIT`](#dt_mips_pixie_init)
- [`DT_MIPS_PLTGOT`](#dt_mips_pltgot) - The address of .got.plt in an executable using the new non-PIC ABI.
- [`DT_MIPS_PROTECTED_GOTIDX`](#dt_mips_protected_gotidx)
- [`DT_MIPS_RLD_MAP`](#dt_mips_rld_map) - Address of run time loader map.
- [`DT_MIPS_RLD_MAP_REL`](#dt_mips_rld_map_rel) - An alternative description of the classic MIPS RLD_MAP that is usable in a PIE as it stores a relative offset from the address of the tag rather than an absolute address.
- [`DT_MIPS_RLD_TEXT_RESOLVE_ADDR`](#dt_mips_rld_text_resolve_addr) - Address of rld_text_rsolve function stored in GOT.
- [`DT_MIPS_RLD_VERSION`](#dt_mips_rld_version) - Runtime linker interface version
- [`DT_MIPS_RWPLT`](#dt_mips_rwplt) - The base of the PLT in an executable using the new non-PIC ABI if that PLT is writable.  For a non-writable PLT, this is omitted or has a zero value.
- [`DT_MIPS_SYMBOL_LIB`](#dt_mips_symbol_lib)
- [`DT_MIPS_SYMTABNO`](#dt_mips_symtabno) - Number of DYNSYM entries
- [`DT_MIPS_TIME_STAMP`](#dt_mips_time_stamp) - Timestamp
- [`DT_MIPS_UNREFEXTNO`](#dt_mips_unrefextno) - First external DYNSYM
- [`DT_MOVEENT`](#dt_moveent)
- [`DT_MOVESZ`](#dt_movesz)
- [`DT_MOVETAB`](#dt_movetab) - Move table.
- [`DT_NEEDED`](#dt_needed) - Name of needed library
- [`DT_NIOS2_GP`](#dt_nios2_gp) - Address of _gp.
- [`DT_NULL`](#dt_null) - Marks end of dynamic section
- [`DT_PLTGOT`](#dt_pltgot) - Processor defined value
- [`DT_PLTPAD`](#dt_pltpad) - PLT padding.
- [`DT_PLTPADSZ`](#dt_pltpadsz)
- [`DT_PLTREL`](#dt_pltrel) - Type of reloc in PLT
- [`DT_PLTRELSZ`](#dt_pltrelsz) - Size in bytes of PLT relocs
- [`DT_POSFLAG_1`](#dt_posflag_1) - Flags for DT_* entries, affecting the following DT_* entry.
- [`DT_PPC64_GLINK`](#dt_ppc64_glink)
- [`DT_PPC64_OPD`](#dt_ppc64_opd)
- [`DT_PPC64_OPDSZ`](#dt_ppc64_opdsz)
- [`DT_PPC64_OPT`](#dt_ppc64_opt)
- [`DT_PPC_GOT`](#dt_ppc_got)
- [`DT_PPC_OPT`](#dt_ppc_opt)
- [`DT_PREINIT_ARRAY`](#dt_preinit_array) - Array with addresses of preinit fct
- [`DT_PREINIT_ARRAYSZ`](#dt_preinit_arraysz) - size in bytes of DT_PREINIT_ARRAY
- [`DT_REL`](#dt_rel) - Address of Rel relocs
- [`DT_RELA`](#dt_rela) - Address of Rela relocs
- [`DT_RELACOUNT`](#dt_relacount)
- [`DT_RELAENT`](#dt_relaent) - Size of one Rela reloc
- [`DT_RELASZ`](#dt_relasz) - Total size of Rela relocs
- [`DT_RELCOUNT`](#dt_relcount)
- [`DT_RELENT`](#dt_relent) - Size of one Rel reloc
- [`DT_RELSZ`](#dt_relsz) - Total size of Rel relocs
- [`DT_RISCV_VARIANT_CC`](#dt_riscv_variant_cc)
- [`DT_RPATH`](#dt_rpath) - Library search path (deprecated)
- [`DT_RUNPATH`](#dt_runpath) - Library search path
- [`DT_SONAME`](#dt_soname) - Name of shared object
- [`DT_SPARC_REGISTER`](#dt_sparc_register)
- [`DT_STRSZ`](#dt_strsz) - Size of string table
- [`DT_STRTAB`](#dt_strtab) - Address of string table
- [`DT_SYMBOLIC`](#dt_symbolic) - Start symbol search here
- [`DT_SYMENT`](#dt_syment) - Size of one symbol table entry
- [`DT_SYMINENT`](#dt_syminent) - Entry size of syminfo
- [`DT_SYMINFO`](#dt_syminfo) - Syminfo table.
- [`DT_SYMINSZ`](#dt_syminsz) - Size of syminfo table (in bytes)
- [`DT_SYMTAB`](#dt_symtab) - Address of symbol table
- [`DT_SYMTAB_SHNDX`](#dt_symtab_shndx) - Address of SYMTAB_SHNDX section
- [`DT_TEXTREL`](#dt_textrel) - Reloc might modify .text
- [`DT_TLSDESC_GOT`](#dt_tlsdesc_got)
- [`DT_TLSDESC_PLT`](#dt_tlsdesc_plt)
- [`DT_VALRNGHI`](#dt_valrnghi)
- [`DT_VALRNGLO`](#dt_valrnglo)
- [`DT_VERDEF`](#dt_verdef) - Address of version definition table
- [`DT_VERDEFNUM`](#dt_verdefnum) - Number of version definitions
- [`DT_VERNEED`](#dt_verneed) - Address of table with needed versions
- [`DT_VERNEEDNUM`](#dt_verneednum) - Number of needed versions
- [`DT_VERSYM`](#dt_versym)
- [`EFA_PARISC_1_0`](#efa_parisc_1_0) - PA-RISC 1.0 big-endian.
- [`EFA_PARISC_1_1`](#efa_parisc_1_1) - PA-RISC 1.1 big-endian.
- [`EFA_PARISC_2_0`](#efa_parisc_2_0) - PA-RISC 2.0 big-endian.
- [`EF_ALPHA_32BIT`](#ef_alpha_32bit) - All addresses must be < 2GB.
- [`EF_ALPHA_CANRELAX`](#ef_alpha_canrelax) - Relocations for relaxing exist.
- [`EF_ARM_ABI_FLOAT_HARD`](#ef_arm_abi_float_hard) - NB conflicts with EF_ARM_VFP_FLOAT
- [`EF_ARM_ABI_FLOAT_SOFT`](#ef_arm_abi_float_soft) - NB conflicts with EF_ARM_SOFT_FLOAT
- [`EF_ARM_ALIGN8`](#ef_arm_align8) - 8-bit structure alignment is in use
- [`EF_ARM_APCS_26`](#ef_arm_apcs_26)
- [`EF_ARM_APCS_FLOAT`](#ef_arm_apcs_float)
- [`EF_ARM_BE8`](#ef_arm_be8)
- [`EF_ARM_DYNSYMSUSESEGIDX`](#ef_arm_dynsymsusesegidx)
- [`EF_ARM_EABIMASK`](#ef_arm_eabimask)
- [`EF_ARM_EABI_UNKNOWN`](#ef_arm_eabi_unknown)
- [`EF_ARM_EABI_VER1`](#ef_arm_eabi_ver1)
- [`EF_ARM_EABI_VER2`](#ef_arm_eabi_ver2)
- [`EF_ARM_EABI_VER3`](#ef_arm_eabi_ver3)
- [`EF_ARM_EABI_VER4`](#ef_arm_eabi_ver4)
- [`EF_ARM_EABI_VER5`](#ef_arm_eabi_ver5)
- [`EF_ARM_HASENTRY`](#ef_arm_hasentry)
- [`EF_ARM_INTERWORK`](#ef_arm_interwork)
- [`EF_ARM_LE8`](#ef_arm_le8)
- [`EF_ARM_MAPSYMSFIRST`](#ef_arm_mapsymsfirst)
- [`EF_ARM_MAVERICK_FLOAT`](#ef_arm_maverick_float)
- [`EF_ARM_NEW_ABI`](#ef_arm_new_abi)
- [`EF_ARM_OLD_ABI`](#ef_arm_old_abi)
- [`EF_ARM_PIC`](#ef_arm_pic)
- [`EF_ARM_RELEXEC`](#ef_arm_relexec)
- [`EF_ARM_SOFT_FLOAT`](#ef_arm_soft_float)
- [`EF_ARM_SYMSARESORTED`](#ef_arm_symsaresorted)
- [`EF_ARM_VFP_FLOAT`](#ef_arm_vfp_float)
- [`EF_AVR_ARCH`](#ef_avr_arch) - Bitmask for `EF_AVR_ARCH_*`.
- [`EF_AVR_ARCH_AVR1`](#ef_avr_arch_avr1)
- [`EF_AVR_ARCH_AVR2`](#ef_avr_arch_avr2)
- [`EF_AVR_ARCH_AVR25`](#ef_avr_arch_avr25)
- [`EF_AVR_ARCH_AVR3`](#ef_avr_arch_avr3)
- [`EF_AVR_ARCH_AVR31`](#ef_avr_arch_avr31)
- [`EF_AVR_ARCH_AVR35`](#ef_avr_arch_avr35)
- [`EF_AVR_ARCH_AVR4`](#ef_avr_arch_avr4)
- [`EF_AVR_ARCH_AVR5`](#ef_avr_arch_avr5)
- [`EF_AVR_ARCH_AVR51`](#ef_avr_arch_avr51)
- [`EF_AVR_ARCH_AVR6`](#ef_avr_arch_avr6)
- [`EF_AVR_ARCH_AVRTINY`](#ef_avr_arch_avrtiny)
- [`EF_AVR_ARCH_XMEGA1`](#ef_avr_arch_xmega1)
- [`EF_AVR_ARCH_XMEGA2`](#ef_avr_arch_xmega2)
- [`EF_AVR_ARCH_XMEGA3`](#ef_avr_arch_xmega3)
- [`EF_AVR_ARCH_XMEGA4`](#ef_avr_arch_xmega4)
- [`EF_AVR_ARCH_XMEGA5`](#ef_avr_arch_xmega5)
- [`EF_AVR_ARCH_XMEGA6`](#ef_avr_arch_xmega6)
- [`EF_AVR_ARCH_XMEGA7`](#ef_avr_arch_xmega7)
- [`EF_AVR_LINKRELAX_PREPARED`](#ef_avr_linkrelax_prepared) - If set, it is assumed that the elf file uses local symbols as reference
- [`EF_CSKY_ABIMASK`](#ef_csky_abimask)
- [`EF_CSKY_ABIV1`](#ef_csky_abiv1)
- [`EF_CSKY_ABIV2`](#ef_csky_abiv2)
- [`EF_CSKY_OTHER`](#ef_csky_other)
- [`EF_CSKY_PROCESSOR`](#ef_csky_processor)
- [`EF_E2K_4MB_PAGES`](#ef_e2k_4mb_pages)
- [`EF_E2K_INCOMPAT`](#ef_e2k_incompat)
- [`EF_E2K_IPD`](#ef_e2k_ipd)
- [`EF_E2K_PACK_SEGMENTS`](#ef_e2k_pack_segments)
- [`EF_E2K_PM`](#ef_e2k_pm)
- [`EF_E2K_X86APP`](#ef_e2k_x86app)
- [`EF_IA_64_ABI64`](#ef_ia_64_abi64) - 64-bit ABI
- [`EF_IA_64_ARCH`](#ef_ia_64_arch) - arch. version mask
- [`EF_IA_64_MASKOS`](#ef_ia_64_maskos) - os-specific flags
- [`EF_LARCH_ABI_DOUBLE_FLOAT`](#ef_larch_abi_double_float) - Uses GPRs, 64-bit FPRs and the stack for parameter passing
- [`EF_LARCH_ABI_MODIFIER_MASK`](#ef_larch_abi_modifier_mask) - Additional properties of the base ABI type, including the FP calling
- [`EF_LARCH_ABI_SINGLE_FLOAT`](#ef_larch_abi_single_float) - Uses GPRs, 32-bit FPRs and the stack for parameter passing
- [`EF_LARCH_ABI_SOFT_FLOAT`](#ef_larch_abi_soft_float) - Uses GPRs and the stack for parameter passing
- [`EF_LARCH_OBJABI_V1`](#ef_larch_objabi_v1) - Uses relocation types directly writing to immediate slots
- [`EF_MIPS_64BIT_WHIRL`](#ef_mips_64bit_whirl)
- [`EF_MIPS_ABI`](#ef_mips_abi) - Mask for selecting EF_MIPS_ABI_ variant
- [`EF_MIPS_ABI2`](#ef_mips_abi2)
- [`EF_MIPS_ABI_EABI32`](#ef_mips_abi_eabi32) - EABI in 32-bit mode
- [`EF_MIPS_ABI_EABI64`](#ef_mips_abi_eabi64) - EABI in 64-bit mode
- [`EF_MIPS_ABI_O32`](#ef_mips_abi_o32) - The first MIPS 32 bit ABI
- [`EF_MIPS_ABI_O64`](#ef_mips_abi_o64) - O32 ABI extended for 64-bit architectures
- [`EF_MIPS_ABI_ON32`](#ef_mips_abi_on32)
- [`EF_MIPS_ARCH`](#ef_mips_arch) - MIPS architecture level.
- [`EF_MIPS_ARCH_1`](#ef_mips_arch_1) - -mips1 code.
- [`EF_MIPS_ARCH_2`](#ef_mips_arch_2) - -mips2 code.
- [`EF_MIPS_ARCH_3`](#ef_mips_arch_3) - -mips3 code.
- [`EF_MIPS_ARCH_32`](#ef_mips_arch_32) - MIPS32 code.
- [`EF_MIPS_ARCH_32R2`](#ef_mips_arch_32r2) - MIPS32r2 code.
- [`EF_MIPS_ARCH_32R6`](#ef_mips_arch_32r6) - MIPS32r6 code
- [`EF_MIPS_ARCH_4`](#ef_mips_arch_4) - -mips4 code.
- [`EF_MIPS_ARCH_5`](#ef_mips_arch_5) - -mips5 code.
- [`EF_MIPS_ARCH_64`](#ef_mips_arch_64) - MIPS64 code.
- [`EF_MIPS_ARCH_64R2`](#ef_mips_arch_64r2) - MIPS64r2 code.
- [`EF_MIPS_ARCH_64R6`](#ef_mips_arch_64r6) - MIPS64r6 code
- [`EF_MIPS_CPIC`](#ef_mips_cpic) - Uses PIC calling sequence.
- [`EF_MIPS_FP64`](#ef_mips_fp64) - Uses FP64 (12 callee-saved).
- [`EF_MIPS_NAN2008`](#ef_mips_nan2008) - Uses IEEE 754-2008 NaN encoding.
- [`EF_MIPS_NOREORDER`](#ef_mips_noreorder) - A .noreorder directive was used.
- [`EF_MIPS_PIC`](#ef_mips_pic) - Contains PIC code.
- [`EF_MIPS_XGOT`](#ef_mips_xgot)
- [`EF_PARISC_ARCH`](#ef_parisc_arch) - Architecture version.
- [`EF_PARISC_EXT`](#ef_parisc_ext) - Program uses arch. extensions.
- [`EF_PARISC_LAZYSWAP`](#ef_parisc_lazyswap) - Allow lazy swapping.
- [`EF_PARISC_LSB`](#ef_parisc_lsb) - Program expects little endian.
- [`EF_PARISC_NO_KABP`](#ef_parisc_no_kabp) - No kernel assisted branch prediction.
- [`EF_PARISC_TRAPNIL`](#ef_parisc_trapnil) - Trap nil pointer dereference.
- [`EF_PARISC_WIDE`](#ef_parisc_wide) - Program expects wide mode.
- [`EF_PPC64_ABI`](#ef_ppc64_abi) - PowerPC64 bits specifying ABI.
- [`EF_PPC_EMB`](#ef_ppc_emb) - PowerPC embedded flag
- [`EF_PPC_RELOCATABLE`](#ef_ppc_relocatable) - PowerPC -mrelocatable flag
- [`EF_PPC_RELOCATABLE_LIB`](#ef_ppc_relocatable_lib) - PowerPC -mrelocatable-lib flag
- [`EF_RISCV_FLOAT_ABI`](#ef_riscv_float_abi)
- [`EF_RISCV_FLOAT_ABI_DOUBLE`](#ef_riscv_float_abi_double)
- [`EF_RISCV_FLOAT_ABI_QUAD`](#ef_riscv_float_abi_quad)
- [`EF_RISCV_FLOAT_ABI_SINGLE`](#ef_riscv_float_abi_single)
- [`EF_RISCV_FLOAT_ABI_SOFT`](#ef_riscv_float_abi_soft)
- [`EF_RISCV_RV64ILP32`](#ef_riscv_rv64ilp32)
- [`EF_RISCV_RVC`](#ef_riscv_rvc)
- [`EF_RISCV_RVE`](#ef_riscv_rve)
- [`EF_RISCV_TSO`](#ef_riscv_tso)
- [`EF_S390_HIGH_GPRS`](#ef_s390_high_gprs) - High GPRs kernel facility needed.
- [`EF_SH1`](#ef_sh1)
- [`EF_SH2`](#ef_sh2)
- [`EF_SH2A`](#ef_sh2a)
- [`EF_SH2A_NOFPU`](#ef_sh2a_nofpu)
- [`EF_SH2A_SH3E`](#ef_sh2a_sh3e)
- [`EF_SH2A_SH3_NOFPU`](#ef_sh2a_sh3_nofpu)
- [`EF_SH2A_SH4`](#ef_sh2a_sh4)
- [`EF_SH2A_SH4_NOFPU`](#ef_sh2a_sh4_nofpu)
- [`EF_SH2E`](#ef_sh2e)
- [`EF_SH3`](#ef_sh3)
- [`EF_SH3E`](#ef_sh3e)
- [`EF_SH3_DSP`](#ef_sh3_dsp)
- [`EF_SH3_NOMMU`](#ef_sh3_nommu)
- [`EF_SH4`](#ef_sh4)
- [`EF_SH4A`](#ef_sh4a)
- [`EF_SH4AL_DSP`](#ef_sh4al_dsp)
- [`EF_SH4A_NOFPU`](#ef_sh4a_nofpu)
- [`EF_SH4_NOFPU`](#ef_sh4_nofpu)
- [`EF_SH4_NOMMU_NOFPU`](#ef_sh4_nommu_nofpu)
- [`EF_SH_DSP`](#ef_sh_dsp)
- [`EF_SH_MACH_MASK`](#ef_sh_mach_mask)
- [`EF_SH_UNKNOWN`](#ef_sh_unknown)
- [`EF_SPARCV9_MM`](#ef_sparcv9_mm)
- [`EF_SPARCV9_PSO`](#ef_sparcv9_pso)
- [`EF_SPARCV9_RMO`](#ef_sparcv9_rmo)
- [`EF_SPARCV9_TSO`](#ef_sparcv9_tso)
- [`EF_SPARC_32PLUS`](#ef_sparc_32plus) - generic V8+ features
- [`EF_SPARC_EXT_MASK`](#ef_sparc_ext_mask)
- [`EF_SPARC_HAL_R1`](#ef_sparc_hal_r1) - HAL R1 extensions
- [`EF_SPARC_LEDATA`](#ef_sparc_ledata) - little endian data
- [`EF_SPARC_SUN_US1`](#ef_sparc_sun_us1) - Sun UltraSPARC1 extensions
- [`EF_SPARC_SUN_US3`](#ef_sparc_sun_us3) - Sun UltraSPARCIII extensions
- [`ELFCLASS32`](#elfclass32) - 32-bit object.
- [`ELFCLASS64`](#elfclass64) - 64-bit object.
- [`ELFCLASSNONE`](#elfclassnone) - Invalid class.
- [`ELFCOMPRESS_HIOS`](#elfcompress_hios) - End of OS-specific compression types.
- [`ELFCOMPRESS_HIPROC`](#elfcompress_hiproc) - End of processor-specific compression types.
- [`ELFCOMPRESS_LOOS`](#elfcompress_loos) - Start of OS-specific compression types.
- [`ELFCOMPRESS_LOPROC`](#elfcompress_loproc) - Start of processor-specific compression types.
- [`ELFCOMPRESS_ZLIB`](#elfcompress_zlib) - ZLIB/DEFLATE algorithm.
- [`ELFCOMPRESS_ZSTD`](#elfcompress_zstd) - Zstandard algorithm.
- [`ELFDATA2LSB`](#elfdata2lsb) - 2's complement, little endian.
- [`ELFDATA2MSB`](#elfdata2msb) - 2's complement, big endian.
- [`ELFDATANONE`](#elfdatanone) - Invalid data encoding.
- [`ELFMAG`](#elfmag) - File identification bytes stored in `Ident::magic`.
- [`ELFOSABI_AIX`](#elfosabi_aix) - IBM AIX.
- [`ELFOSABI_ARM`](#elfosabi_arm) - ARM.
- [`ELFOSABI_ARM_AEABI`](#elfosabi_arm_aeabi) - ARM EABI.
- [`ELFOSABI_AROS`](#elfosabi_aros) - AROS
- [`ELFOSABI_CLOUDABI`](#elfosabi_cloudabi) - Nuxi CloudABI
- [`ELFOSABI_FENIXOS`](#elfosabi_fenixos) - FenixOS
- [`ELFOSABI_FREEBSD`](#elfosabi_freebsd) - FreeBSD.
- [`ELFOSABI_GNU`](#elfosabi_gnu) - Object uses GNU ELF extensions.
- [`ELFOSABI_HPUX`](#elfosabi_hpux) - HP-UX.
- [`ELFOSABI_HURD`](#elfosabi_hurd) - GNU/Hurd.
- [`ELFOSABI_IRIX`](#elfosabi_irix) - SGI Irix.
- [`ELFOSABI_LINUX`](#elfosabi_linux) - Object uses GNU ELF extensions.
- [`ELFOSABI_MODESTO`](#elfosabi_modesto) - Novell Modesto.
- [`ELFOSABI_NETBSD`](#elfosabi_netbsd) - NetBSD.
- [`ELFOSABI_NONE`](#elfosabi_none) - UNIX System V ABI.
- [`ELFOSABI_NSK`](#elfosabi_nsk) - Hewlett-Packard Non-Stop Kernel.
- [`ELFOSABI_OPENBSD`](#elfosabi_openbsd) - OpenBSD.
- [`ELFOSABI_OPENVMS`](#elfosabi_openvms) - OpenVMS.
- [`ELFOSABI_SOLARIS`](#elfosabi_solaris) - Sun Solaris.
- [`ELFOSABI_STANDALONE`](#elfosabi_standalone) - Standalone (embedded) application.
- [`ELFOSABI_SYSV`](#elfosabi_sysv) - UNIX System V ABI.
- [`ELFOSABI_TRU64`](#elfosabi_tru64) - Compaq TRU64 UNIX.
- [`ELF_NOTE_CORE`](#elf_note_core) - Note name for core files.
- [`ELF_NOTE_GNU`](#elf_note_gnu) - GNU entries in the note section have this name.
- [`ELF_NOTE_GO`](#elf_note_go) - Go entries in the note section have this name.
- [`ELF_NOTE_LINUX`](#elf_note_linux) - Note name for linux core files.
- [`ELF_NOTE_OS_FREEBSD`](#elf_note_os_freebsd) - OS descriptor for `NT_GNU_ABI_TAG`.
- [`ELF_NOTE_OS_GNU`](#elf_note_os_gnu) - OS descriptor for `NT_GNU_ABI_TAG`.
- [`ELF_NOTE_OS_LINUX`](#elf_note_os_linux) - OS descriptor for `NT_GNU_ABI_TAG`.
- [`ELF_NOTE_OS_SOLARIS2`](#elf_note_os_solaris2) - OS descriptor for `NT_GNU_ABI_TAG`.
- [`ELF_NOTE_SOLARIS`](#elf_note_solaris) - Solaris entries in the note section have this name.
- [`EM_386`](#em_386) - Intel 80386
- [`EM_56800EX`](#em_56800ex) - Freescale 56800EX DSC
- [`EM_68HC05`](#em_68hc05) - Motorola MC68HC05 microcontroller
- [`EM_68HC08`](#em_68hc08) - Motorola MC68HC08 microcontroller
- [`EM_68HC11`](#em_68hc11) - Motorola MC68HC11 microcontroller
- [`EM_68HC12`](#em_68hc12) - Motorola M68HC12
- [`EM_68HC16`](#em_68hc16) - Motorola MC68HC16 microcontroller
- [`EM_68K`](#em_68k) - Motorola m68k family
- [`EM_78KOR`](#em_78kor) - Renesas 78KOR
- [`EM_8051`](#em_8051) - Intel 8051 and variants
- [`EM_860`](#em_860) - Intel 80860
- [`EM_88K`](#em_88k) - Motorola m88k family
- [`EM_960`](#em_960) - Intel 80960
- [`EM_AARCH64`](#em_aarch64) - ARM AARCH64
- [`EM_ALPHA`](#em_alpha) - Digital Alpha
- [`EM_ALTERA_NIOS2`](#em_altera_nios2) - Altera Nios II
- [`EM_AMDGPU`](#em_amdgpu) - AMD GPU
- [`EM_ARC`](#em_arc) - Argonaut RISC Core
- [`EM_ARCA`](#em_arca) - Arca RISC
- [`EM_ARC_COMPACT`](#em_arc_compact) - ARC International ARCompact
- [`EM_ARC_COMPACT2`](#em_arc_compact2) - Synopsys ARCompact V2
- [`EM_ARM`](#em_arm) - ARM
- [`EM_AVR`](#em_avr) - Atmel AVR 8-bit microcontroller
- [`EM_AVR32`](#em_avr32) - Amtel 32-bit microprocessor
- [`EM_BA1`](#em_ba1) - Beyond BA1
- [`EM_BA2`](#em_ba2) - Beyond BA2
- [`EM_BLACKFIN`](#em_blackfin) - Analog Devices Blackfin DSP
- [`EM_BPF`](#em_bpf) - Linux BPF -- in-kernel virtual machine
- [`EM_C166`](#em_c166) - Infineon C16x/XC16x
- [`EM_CDP`](#em_cdp) - Paneve CDP
- [`EM_CE`](#em_ce) - Freescale Communication Engine RISC
- [`EM_CLOUDSHIELD`](#em_cloudshield) - CloudShield
- [`EM_COGE`](#em_coge) - Cognitive Smart Memory Processor
- [`EM_COLDFIRE`](#em_coldfire) - Motorola Coldfire
- [`EM_COOL`](#em_cool) - Bluechip CoolEngine
- [`EM_COREA_1ST`](#em_corea_1st) - KIPO-KAIST Core-A 1st gen.
- [`EM_COREA_2ND`](#em_corea_2nd) - KIPO-KAIST Core-A 2nd gen.
- [`EM_CR`](#em_cr) - National Semi. CompactRISC
- [`EM_CR16`](#em_cr16) - National Semi. CompactRISC CR16
- [`EM_CRAYNV2`](#em_craynv2) - Cray NV2 vector architecture
- [`EM_CRIS`](#em_cris) - Axis Communications 32-bit emb.proc
- [`EM_CRX`](#em_crx) - National Semi. CompactRISC CRX
- [`EM_CSKY`](#em_csky) - C-SKY
- [`EM_CSR_KALIMBA`](#em_csr_kalimba) - CSR Kalimba
- [`EM_CUDA`](#em_cuda) - NVIDIA CUDA
- [`EM_CYPRESS_M8C`](#em_cypress_m8c) - Cypress M8C
- [`EM_D10V`](#em_d10v) - Mitsubishi D10V
- [`EM_D30V`](#em_d30v) - Mitsubishi D30V
- [`EM_DSP24`](#em_dsp24) - New Japan Radio (NJR) 24-bit DSP
- [`EM_DSPIC30F`](#em_dspic30f) - Microchip Technology dsPIC30F
- [`EM_DXP`](#em_dxp) - Icera Semi. Deep Execution Processor
- [`EM_ECOG16`](#em_ecog16) - Cyan Technology eCOG16
- [`EM_ECOG1X`](#em_ecog1x) - Cyan Technology eCOG1X
- [`EM_ECOG2`](#em_ecog2) - Cyan Technology eCOG2
- [`EM_EMX16`](#em_emx16) - KM211 KMX16
- [`EM_EMX8`](#em_emx8) - KM211 KMX8
- [`EM_ETPU`](#em_etpu) - Freescale Extended Time Processing Unit
- [`EM_EXCESS`](#em_excess) - eXcess configurable cpu
- [`EM_F2MC16`](#em_f2mc16) - Fujitsu F2MC16
- [`EM_FAKE_ALPHA`](#em_fake_alpha) - Digital Alpha
- [`EM_FIREPATH`](#em_firepath) - Element 14 64-bit DSP Processor
- [`EM_FR20`](#em_fr20) - Fujitsu FR20
- [`EM_FR30`](#em_fr30) - Fujitsu FR30
- [`EM_FT32`](#em_ft32) - FTDI Chip FT32
- [`EM_FX66`](#em_fx66) - Siemens FX66 microcontroller
- [`EM_H8S`](#em_h8s) - Hitachi H8S
- [`EM_H8_300`](#em_h8_300) - Hitachi H8/300
- [`EM_H8_300H`](#em_h8_300h) - Hitachi H8/300H
- [`EM_H8_500`](#em_h8_500) - Hitachi H8/500
- [`EM_HEXAGON`](#em_hexagon) - QUALCOMM Hexagon
- [`EM_HUANY`](#em_huany) - Harvard University machine-independent object files
- [`EM_IAMCU`](#em_iamcu) - Intel MCU
- [`EM_IA_64`](#em_ia_64) - Intel Merced
- [`EM_IP2K`](#em_ip2k) - Ubicom IP2xxx
- [`EM_JAVELIN`](#em_javelin) - Infineon Technologies 32-bit emb.proc
- [`EM_K10M`](#em_k10m) - Intel K10M
- [`EM_KM32`](#em_km32) - KM211 KM32
- [`EM_KMX32`](#em_kmx32) - KM211 KMX32
- [`EM_KVARC`](#em_kvarc) - KM211 KVARC
- [`EM_L10M`](#em_l10m) - Intel L10M
- [`EM_LATTICEMICO32`](#em_latticemico32) - RISC for Lattice FPGA
- [`EM_LOONGARCH`](#em_loongarch) - Loongson LoongArch
- [`EM_M16C`](#em_m16c) - Renesas M16C
- [`EM_M32`](#em_m32) - AT&T WE 32100
- [`EM_M32C`](#em_m32c) - Renesas M32C
- [`EM_M32R`](#em_m32r) - Mitsubishi M32R
- [`EM_MANIK`](#em_manik) - M2000 Reconfigurable RISC
- [`EM_MAX`](#em_max) - MAX processor
- [`EM_MAXQ30`](#em_maxq30) - Dallas Semi. MAXQ30 mc
- [`EM_MCHP_PIC`](#em_mchp_pic) - Microchip 8-bit PIC(r)
- [`EM_MCST_ELBRUS`](#em_mcst_elbrus) - MCST Elbrus
- [`EM_ME16`](#em_me16) - Toyota ME16 processor
- [`EM_METAG`](#em_metag) - Imagination Tech. META
- [`EM_MICROBLAZE`](#em_microblaze) - Xilinx MicroBlaze
- [`EM_MIPS`](#em_mips) - MIPS R3000 big-endian
- [`EM_MIPS_RS3_LE`](#em_mips_rs3_le) - MIPS R3000 little-endian
- [`EM_MIPS_X`](#em_mips_x) - Stanford MIPS-X
- [`EM_MMA`](#em_mma) - Fujitsu MMA Multimedia Accelerator
- [`EM_MMDSP_PLUS`](#em_mmdsp_plus) - STMicroelectronics 64bit VLIW DSP
- [`EM_MMIX`](#em_mmix) - Donald Knuth's educational 64-bit proc
- [`EM_MN10200`](#em_mn10200) - Matsushita MN10200
- [`EM_MN10300`](#em_mn10300) - Matsushita MN10300
- [`EM_MOXIE`](#em_moxie) - Moxie processor
- [`EM_MSP430`](#em_msp430) - Texas Instruments msp430
- [`EM_NCPU`](#em_ncpu) - Sony nCPU embeeded RISC
- [`EM_NDR1`](#em_ndr1) - Denso NDR1 microprocessor
- [`EM_NDS32`](#em_nds32) - Andes Tech. compact code emb. RISC
- [`EM_NONE`](#em_none) - No machine
- [`EM_NORC`](#em_norc) - Nanoradio Optimized RISC
- [`EM_NS32K`](#em_ns32k) - National Semi. 32000
- [`EM_OPEN8`](#em_open8) - Open8 RISC
- [`EM_OPENRISC`](#em_openrisc) - OpenRISC 32-bit embedded processor
- [`EM_PARISC`](#em_parisc) - HPPA
- [`EM_PCP`](#em_pcp) - Siemens PCP
- [`EM_PDP10`](#em_pdp10) - Digital PDP-10
- [`EM_PDP11`](#em_pdp11) - Digital PDP-11
- [`EM_PDSP`](#em_pdsp) - Sony DSP Processor
- [`EM_PJ`](#em_pj) - picoJava
- [`EM_PPC`](#em_ppc) - PowerPC
- [`EM_PPC64`](#em_ppc64) - PowerPC 64-bit
- [`EM_PRISM`](#em_prism) - SiTera Prism
- [`EM_R32C`](#em_r32c) - Renesas R32C
- [`EM_RCE`](#em_rce) - Motorola RCE
- [`EM_RH32`](#em_rh32) - TRW RH-32
- [`EM_RISCV`](#em_riscv) - RISC-V
- [`EM_RL78`](#em_rl78) - Renesas RL78
- [`EM_RS08`](#em_rs08) - Freescale RS08
- [`EM_RX`](#em_rx) - Renesas RX
- [`EM_S370`](#em_s370) - IBM System/370
- [`EM_S390`](#em_s390) - IBM S390
- [`EM_SBF`](#em_sbf) - Solana Binary Format
- [`EM_SCORE7`](#em_score7) - Sunplus S+core7 RISC
- [`EM_SEP`](#em_sep) - Sharp embedded microprocessor
- [`EM_SE_C17`](#em_se_c17) - Seiko Epson C17
- [`EM_SE_C33`](#em_se_c33) - Seiko Epson S1C33 family
- [`EM_SH`](#em_sh) - Hitachi SH
- [`EM_SHARC`](#em_sharc) - Analog Devices SHARC family
- [`EM_SLE9X`](#em_sle9x) - Infineon Tech. SLE9X
- [`EM_SNP1K`](#em_snp1k) - Trebia SNP 1000
- [`EM_SPARC`](#em_sparc) - SUN SPARC
- [`EM_SPARC32PLUS`](#em_sparc32plus) - Sun's "v8plus"
- [`EM_SPARCV9`](#em_sparcv9) - SPARC v9 64-bit
- [`EM_SPU`](#em_spu) - IBM SPU/SPC
- [`EM_ST100`](#em_st100) - STMicroelectronic ST100 processor
- [`EM_ST19`](#em_st19) - STMicroelectronics ST19 8 bit mc
- [`EM_ST200`](#em_st200) - STMicroelectronics ST200
- [`EM_ST7`](#em_st7) - STmicroelectronics ST7 8 bit mc
- [`EM_ST9PLUS`](#em_st9plus) - STMicroelectronics ST9+ 8/16 mc
- [`EM_STARCORE`](#em_starcore) - Motorola Start*Core processor
- [`EM_STM8`](#em_stm8) - STMicroelectronics STM8
- [`EM_STXP7X`](#em_stxp7x) - STMicroelectronics STxP7x
- [`EM_SVX`](#em_svx) - Silicon Graphics SVx
- [`EM_TILE64`](#em_tile64) - Tileta TILE64
- [`EM_TILEGX`](#em_tilegx) - Tilera TILE-Gx
- [`EM_TILEPRO`](#em_tilepro) - Tilera TILEPro
- [`EM_TINYJ`](#em_tinyj) - Advanced Logic Corp. Tinyj emb.fam
- [`EM_TI_ARP32`](#em_ti_arp32) - Texas Instruments App. Specific RISC
- [`EM_TI_C2000`](#em_ti_c2000) - Texas Instruments TMS320C2000 DSP
- [`EM_TI_C5500`](#em_ti_c5500) - Texas Instruments TMS320C55x DSP
- [`EM_TI_C6000`](#em_ti_c6000) - Texas Instruments TMS320C6000 DSP
- [`EM_TI_PRU`](#em_ti_pru) - Texas Instruments Prog. Realtime Unit
- [`EM_TMM_GPP`](#em_tmm_gpp) - Thompson Multimedia General Purpose Proc
- [`EM_TPC`](#em_tpc) - Tenor Network TPC
- [`EM_TRICORE`](#em_tricore) - Siemens Tricore
- [`EM_TRIMEDIA`](#em_trimedia) - NXP Semi. TriMedia
- [`EM_TSK3000`](#em_tsk3000) - Altium TSK3000
- [`EM_UNICORE`](#em_unicore) - PKU-Unity & MPRC Peking Uni. mc series
- [`EM_V800`](#em_v800) - NEC V800 series
- [`EM_V850`](#em_v850) - NEC v850
- [`EM_VAX`](#em_vax) - Digital VAX
- [`EM_VIDEOCORE`](#em_videocore) - Alphamosaic VideoCore
- [`EM_VIDEOCORE3`](#em_videocore3) - Broadcom VideoCore III
- [`EM_VIDEOCORE5`](#em_videocore5) - Broadcom VideoCore V
- [`EM_VISIUM`](#em_visium) - Controls and Data Services VISIUMcore
- [`EM_VPP500`](#em_vpp500) - Fujitsu VPP500
- [`EM_X86_64`](#em_x86_64) - AMD x86-64 architecture
- [`EM_XCORE`](#em_xcore) - XMOS xCORE
- [`EM_XGATE`](#em_xgate) - Motorola XGATE
- [`EM_XIMO16`](#em_ximo16) - New Japan Radio (NJR) 16-bit DSP
- [`EM_XTENSA`](#em_xtensa) - Tensilica Xtensa Architecture
- [`EM_Z80`](#em_z80) - Zilog Z80
- [`EM_ZSP`](#em_zsp) - LSI Logic 16-bit DSP Processor
- [`ET_CORE`](#et_core) - Core file.
- [`ET_DYN`](#et_dyn) - Shared object file.
- [`ET_EXEC`](#et_exec) - Executable file.
- [`ET_HIOS`](#et_hios) - OS-specific range end.
- [`ET_HIPROC`](#et_hiproc) - Processor-specific range end.
- [`ET_LOOS`](#et_loos) - OS-specific range start.
- [`ET_LOPROC`](#et_loproc) - Processor-specific range start.
- [`ET_NONE`](#et_none) - No file type.
- [`ET_REL`](#et_rel) - Relocatable file.
- [`EV_CURRENT`](#ev_current) - Current ELF version.
- [`EV_NONE`](#ev_none) - Invalid ELF version.
- [`E_E2K_MACH_12C`](#e_e2k_mach_12c) - -mtune=elbrus-12c code.
- [`E_E2K_MACH_16C`](#e_e2k_mach_16c) - -mtune=elbrus-16c code.
- [`E_E2K_MACH_1CPLUS`](#e_e2k_mach_1cplus) - -mtune=elbrus-1c+ code.
- [`E_E2K_MACH_2C3`](#e_e2k_mach_2c3) - -mtune=elbrus-2c3 code.
- [`E_E2K_MACH_48C`](#e_e2k_mach_48c) - -mtune=elbrus-48c code.
- [`E_E2K_MACH_8C`](#e_e2k_mach_8c) - -mtune=elbrus-8c code.
- [`E_E2K_MACH_8V7`](#e_e2k_mach_8v7) - -mtune=elbrus-8v7 code.
- [`E_E2K_MACH_BASE`](#e_e2k_mach_base) - -march=generic code.
- [`E_E2K_MACH_EV1`](#e_e2k_mach_ev1) - -march=elbrus-v1 code.
- [`E_E2K_MACH_EV2`](#e_e2k_mach_ev2) - -march=elbrus-v2 code.
- [`E_E2K_MACH_EV3`](#e_e2k_mach_ev3) - -march=elbrus-v3 code.
- [`E_E2K_MACH_EV4`](#e_e2k_mach_ev4) - -march=elbrus-v4 code.
- [`E_E2K_MACH_EV5`](#e_e2k_mach_ev5) - -march=elbrus-v5 code.
- [`E_E2K_MACH_EV6`](#e_e2k_mach_ev6) - -march=elbrus-v6 code.
- [`E_E2K_MACH_EV7`](#e_e2k_mach_ev7) - -march=elbrus-v7 code.
- [`GNU_PROPERTY_1_NEEDED`](#gnu_property_1_needed) - The needed properties by the object file.  */
- [`GNU_PROPERTY_1_NEEDED_INDIRECT_EXTERN_ACCESS`](#gnu_property_1_needed_indirect_extern_access) - Set if the object file requires canonical function pointers and
- [`GNU_PROPERTY_AARCH64_FEATURE_1_AND`](#gnu_property_aarch64_feature_1_and) - AArch64 specific GNU properties.
- [`GNU_PROPERTY_AARCH64_FEATURE_1_BTI`](#gnu_property_aarch64_feature_1_bti)
- [`GNU_PROPERTY_AARCH64_FEATURE_1_PAC`](#gnu_property_aarch64_feature_1_pac)
- [`GNU_PROPERTY_AARCH64_FEATURE_PAUTH`](#gnu_property_aarch64_feature_pauth)
- [`GNU_PROPERTY_HIPROC`](#gnu_property_hiproc) - Processor-specific semantics, hi
- [`GNU_PROPERTY_HIUSER`](#gnu_property_hiuser) - Application-specific semantics, hi
- [`GNU_PROPERTY_LOPROC`](#gnu_property_loproc) - Processor-specific semantics, lo
- [`GNU_PROPERTY_LOUSER`](#gnu_property_louser) - Application-specific semantics, lo
- [`GNU_PROPERTY_NO_COPY_ON_PROTECTED`](#gnu_property_no_copy_on_protected) - No copy relocation on protected data symbol.
- [`GNU_PROPERTY_STACK_SIZE`](#gnu_property_stack_size) - Stack size.
- [`GNU_PROPERTY_UINT32_AND_HI`](#gnu_property_uint32_and_hi)
- [`GNU_PROPERTY_UINT32_AND_LO`](#gnu_property_uint32_and_lo)
- [`GNU_PROPERTY_UINT32_OR_HI`](#gnu_property_uint32_or_hi)
- [`GNU_PROPERTY_UINT32_OR_LO`](#gnu_property_uint32_or_lo)
- [`GNU_PROPERTY_X86_FEATURE_1_AND`](#gnu_property_x86_feature_1_and) - X86 processor-specific features used in program.
- [`GNU_PROPERTY_X86_FEATURE_1_IBT`](#gnu_property_x86_feature_1_ibt) - This indicates that all executable sections are compatible with IBT.
- [`GNU_PROPERTY_X86_FEATURE_1_SHSTK`](#gnu_property_x86_feature_1_shstk) - This indicates that all executable sections are compatible with SHSTK.
- [`GNU_PROPERTY_X86_ISA_1_BASELINE`](#gnu_property_x86_isa_1_baseline) - GNU_PROPERTY_X86_ISA_1_BASELINE: CMOV, CX8 (cmpxchg8b), FPU (fld),
- [`GNU_PROPERTY_X86_ISA_1_NEEDED`](#gnu_property_x86_isa_1_needed) - The x86 instruction sets indicated by the corresponding bits are
- [`GNU_PROPERTY_X86_ISA_1_USED`](#gnu_property_x86_isa_1_used) - The x86 instruction sets indicated by the corresponding bits are
- [`GNU_PROPERTY_X86_ISA_1_V2`](#gnu_property_x86_isa_1_v2) - GNU_PROPERTY_X86_ISA_1_V2: GNU_PROPERTY_X86_ISA_1_BASELINE,
- [`GNU_PROPERTY_X86_ISA_1_V3`](#gnu_property_x86_isa_1_v3) - GNU_PROPERTY_X86_ISA_1_V3: GNU_PROPERTY_X86_ISA_1_V2, AVX, AVX2, BMI1,
- [`GNU_PROPERTY_X86_ISA_1_V4`](#gnu_property_x86_isa_1_v4) - GNU_PROPERTY_X86_ISA_1_V4: GNU_PROPERTY_X86_ISA_1_V3, AVX512F,
- [`GNU_PROPERTY_X86_UINT32_AND_HI`](#gnu_property_x86_uint32_and_hi)
- [`GNU_PROPERTY_X86_UINT32_AND_LO`](#gnu_property_x86_uint32_and_lo)
- [`GNU_PROPERTY_X86_UINT32_OR_AND_HI`](#gnu_property_x86_uint32_or_and_hi)
- [`GNU_PROPERTY_X86_UINT32_OR_AND_LO`](#gnu_property_x86_uint32_or_and_lo)
- [`GNU_PROPERTY_X86_UINT32_OR_HI`](#gnu_property_x86_uint32_or_hi)
- [`GNU_PROPERTY_X86_UINT32_OR_LO`](#gnu_property_x86_uint32_or_lo)
- [`GRP_COMDAT`](#grp_comdat) - Mark group as COMDAT.
- [`LITUSE_ALPHA_ADDR`](#lituse_alpha_addr)
- [`LITUSE_ALPHA_BASE`](#lituse_alpha_base)
- [`LITUSE_ALPHA_BYTOFF`](#lituse_alpha_bytoff)
- [`LITUSE_ALPHA_JSR`](#lituse_alpha_jsr)
- [`LITUSE_ALPHA_TLS_GD`](#lituse_alpha_tls_gd)
- [`LITUSE_ALPHA_TLS_LDM`](#lituse_alpha_tls_ldm)
- [`LL_DELAY_LOAD`](#ll_delay_load)
- [`LL_DELTA`](#ll_delta)
- [`LL_EXACT_MATCH`](#ll_exact_match) - Require exact match
- [`LL_EXPORTS`](#ll_exports)
- [`LL_IGNORE_INT_VER`](#ll_ignore_int_ver) - Ignore interface version
- [`LL_NONE`](#ll_none)
- [`LL_REQUIRE_MINOR`](#ll_require_minor)
- [`NT_386_IOPERM`](#nt_386_ioperm) - x86 io permission bitmap (1=deny).
- [`NT_386_TLS`](#nt_386_tls) - i386 TLS slots (struct user_desc).
- [`NT_ARM_HW_BREAK`](#nt_arm_hw_break) - ARM hardware breakpoint registers.
- [`NT_ARM_HW_WATCH`](#nt_arm_hw_watch) - ARM hardware watchpoint registers.
- [`NT_ARM_SVE`](#nt_arm_sve) - ARM Scalable Vector Extension registers.
- [`NT_ARM_SYSTEM_CALL`](#nt_arm_system_call) - ARM system call number.
- [`NT_ARM_TLS`](#nt_arm_tls) - ARM TLS register.
- [`NT_ARM_VFP`](#nt_arm_vfp) - ARM VFP/NEON registers.
- [`NT_ASRS`](#nt_asrs) - Contains copy of asrset struct.
- [`NT_AUXV`](#nt_auxv) - Contains copy of auxv array.
- [`NT_FILE`](#nt_file) - Contains information about mapped files.
- [`NT_FPREGSET`](#nt_fpregset) - Contains copy of fpregset struct.
- [`NT_GNU_ABI_TAG`](#nt_gnu_abi_tag) - ABI information.
- [`NT_GNU_BUILD_ID`](#nt_gnu_build_id) - Build ID bits as generated by `ld --build-id`.
- [`NT_GNU_GOLD_VERSION`](#nt_gnu_gold_version) - Version note generated by GNU gold containing a version string.
- [`NT_GNU_HWCAP`](#nt_gnu_hwcap) - Synthetic hwcap information.
- [`NT_GNU_PROPERTY_TYPE_0`](#nt_gnu_property_type_0) - Program property.
- [`NT_GO_BUILD_ID`](#nt_go_build_id) - Build ID bits as generated by Go's gc compiler.
- [`NT_GWINDOWS`](#nt_gwindows) - Contains copy of gwindows struct.
- [`NT_LWPSINFO`](#nt_lwpsinfo) - Contains copy of lwpinfo struct.
- [`NT_LWPSTATUS`](#nt_lwpstatus) - Contains copy of lwpstatus struct.
- [`NT_MIPS_DSP`](#nt_mips_dsp) - MIPS DSP ASE registers.
- [`NT_MIPS_FP_MODE`](#nt_mips_fp_mode) - MIPS floating-point mode.
- [`NT_PLATFORM`](#nt_platform) - String from sysinfo(SI_PLATFORM).
- [`NT_PPC_DSCR`](#nt_ppc_dscr) - Data Stream Control Register.
- [`NT_PPC_EBB`](#nt_ppc_ebb) - Event Based Branch Registers.
- [`NT_PPC_PKEY`](#nt_ppc_pkey) - Memory Protection Keys registers.
- [`NT_PPC_PMU`](#nt_ppc_pmu) - Performance Monitor Registers.
- [`NT_PPC_PPR`](#nt_ppc_ppr) - Program Priority Register.
- [`NT_PPC_SPE`](#nt_ppc_spe) - PowerPC SPE/EVR registers.
- [`NT_PPC_TAR`](#nt_ppc_tar) - Target Address Register.
- [`NT_PPC_TM_CDSCR`](#nt_ppc_tm_cdscr) - TM checkpointed Data Stream Control Register.
- [`NT_PPC_TM_CFPR`](#nt_ppc_tm_cfpr) - TM checkpointed FPR Registers.
- [`NT_PPC_TM_CGPR`](#nt_ppc_tm_cgpr) - TM checkpointed GPR Registers.
- [`NT_PPC_TM_CPPR`](#nt_ppc_tm_cppr) - TM checkpointed Program Priority Register.
- [`NT_PPC_TM_CTAR`](#nt_ppc_tm_ctar) - TM checkpointed Target Address Register.
- [`NT_PPC_TM_CVMX`](#nt_ppc_tm_cvmx) - TM checkpointed VMX Registers.
- [`NT_PPC_TM_CVSX`](#nt_ppc_tm_cvsx) - TM checkpointed VSX Registers.
- [`NT_PPC_TM_SPR`](#nt_ppc_tm_spr) - TM Special Purpose Registers.
- [`NT_PPC_VMX`](#nt_ppc_vmx) - PowerPC Altivec/VMX registers.
- [`NT_PPC_VSX`](#nt_ppc_vsx) - PowerPC VSX registers.
- [`NT_PRCRED`](#nt_prcred) - Contains copy of prcred struct.
- [`NT_PRFPREG`](#nt_prfpreg) - Contains copy of fpregset struct.
- [`NT_PRFPXREG`](#nt_prfpxreg) - Contains copy of fprxregset struct.
- [`NT_PRPSINFO`](#nt_prpsinfo) - Contains copy of prpsinfo struct.
- [`NT_PRSTATUS`](#nt_prstatus) - Contains copy of prstatus struct.
- [`NT_PRXFPREG`](#nt_prxfpreg) - Contains copy of user_fxsr_struct.
- [`NT_PRXREG`](#nt_prxreg) - Contains copy of prxregset struct.
- [`NT_PSINFO`](#nt_psinfo) - Contains copy of psinfo struct.
- [`NT_PSTATUS`](#nt_pstatus) - Contains copy of pstatus struct.
- [`NT_S390_CTRS`](#nt_s390_ctrs) - s390 control registers.
- [`NT_S390_GS_BC`](#nt_s390_gs_bc) - s390 guarded storage broadcast control block.
- [`NT_S390_GS_CB`](#nt_s390_gs_cb) - s390 guarded storage registers.
- [`NT_S390_HIGH_GPRS`](#nt_s390_high_gprs) - s390 upper register halves.
- [`NT_S390_LAST_BREAK`](#nt_s390_last_break) - s390 breaking event address.
- [`NT_S390_PREFIX`](#nt_s390_prefix) - s390 prefix register.
- [`NT_S390_RI_CB`](#nt_s390_ri_cb) - s390 runtime instrumentation.
- [`NT_S390_SYSTEM_CALL`](#nt_s390_system_call) - s390 system call restart data.
- [`NT_S390_TDB`](#nt_s390_tdb) - s390 transaction diagnostic block.
- [`NT_S390_TIMER`](#nt_s390_timer) - s390 timer register.
- [`NT_S390_TODCMP`](#nt_s390_todcmp) - s390 TOD clock comparator register.
- [`NT_S390_TODPREG`](#nt_s390_todpreg) - s390 TOD programmable register.
- [`NT_S390_VXRS_HIGH`](#nt_s390_vxrs_high) - s390 vector registers 16-31.
- [`NT_S390_VXRS_LOW`](#nt_s390_vxrs_low) - s390 vector registers 0-15 upper half.
- [`NT_SIGINFO`](#nt_siginfo) - Contains copy of siginfo_t, size might increase.
- [`NT_SOLARIS_PAGESIZE_HINT`](#nt_solaris_pagesize_hint) - Desired pagesize for the binary.
- [`NT_TASKSTRUCT`](#nt_taskstruct) - Contains copy of task structure.
- [`NT_UTSNAME`](#nt_utsname) - Contains copy of utsname struct.
- [`NT_VERSION`](#nt_version) - Note type for version string.
- [`NT_VMCOREDD`](#nt_vmcoredd) - Vmcore Device Dump Note.
- [`NT_X86_XSTATE`](#nt_x86_xstate) - x86 extended state using xsave.
- [`ODK_EXCEPTIONS`](#odk_exceptions) - Exception processing options.
- [`ODK_FILL`](#odk_fill) - record the fill value used by the linker.
- [`ODK_HWAND`](#odk_hwand) - HW workarounds.  'AND' bits when merging.
- [`ODK_HWOR`](#odk_hwor) - HW workarounds.  'OR' bits when merging.
- [`ODK_HWPATCH`](#odk_hwpatch) - Hardware workarounds performed
- [`ODK_NULL`](#odk_null) - Undefined.
- [`ODK_PAD`](#odk_pad) - Section padding options.
- [`ODK_REGINFO`](#odk_reginfo) - Register usage information.
- [`ODK_TAGS`](#odk_tags) - reserve space for desktop tools to write.
- [`OEX_DISMISS`](#oex_dismiss) - Dismiss invalid address faults?
- [`OEX_FPDBUG`](#oex_fpdbug) - Force floating point debug mode?
- [`OEX_FPU_DIV0`](#oex_fpu_div0)
- [`OEX_FPU_INEX`](#oex_fpu_inex)
- [`OEX_FPU_INVAL`](#oex_fpu_inval)
- [`OEX_FPU_MAX`](#oex_fpu_max) - FPE's which MAY be enabled.
- [`OEX_FPU_MIN`](#oex_fpu_min) - FPE's which MUST be enabled.
- [`OEX_FPU_OFLO`](#oex_fpu_oflo)
- [`OEX_FPU_UFLO`](#oex_fpu_uflo)
- [`OEX_PAGE0`](#oex_page0) - page zero must be mapped.
- [`OEX_PRECISEFP`](#oex_precisefp)
- [`OEX_SMM`](#oex_smm) - Force sequential memory mode?
- [`OHWA0_R4KEOP_CHECKED`](#ohwa0_r4keop_checked)
- [`OHWA1_R4KEOP_CLEAN`](#ohwa1_r4keop_clean)
- [`OHW_R4KEOP`](#ohw_r4keop) - R4000 end-of-page patch.
- [`OHW_R5KCVTL`](#ohw_r5kcvtl) - R5000 cvt.\[ds\].l bug.  clean=1.
- [`OHW_R5KEOP`](#ohw_r5keop) - R5000 end-of-page patch.
- [`OHW_R8KPFETCH`](#ohw_r8kpfetch) - may need R8000 prefetch patch.
- [`OPAD_POSTFIX`](#opad_postfix)
- [`OPAD_PREFIX`](#opad_prefix)
- [`OPAD_SYMBOL`](#opad_symbol)
- [`PF_ARM_ABS`](#pf_arm_abs) - Absolute segment.
- [`PF_ARM_PI`](#pf_arm_pi) - Position-independent segment.
- [`PF_ARM_SB`](#pf_arm_sb) - Segment contains the location addressed by the static base.
- [`PF_HP_CODE`](#pf_hp_code)
- [`PF_HP_FAR_SHARED`](#pf_hp_far_shared)
- [`PF_HP_LAZYSWAP`](#pf_hp_lazyswap)
- [`PF_HP_MODIFY`](#pf_hp_modify)
- [`PF_HP_NEAR_SHARED`](#pf_hp_near_shared)
- [`PF_HP_PAGE_SIZE`](#pf_hp_page_size)
- [`PF_HP_SBP`](#pf_hp_sbp)
- [`PF_IA_64_NORECOV`](#pf_ia_64_norecov) - spec insns w/o recovery
- [`PF_MASKOS`](#pf_maskos) - OS-specific segment flags.
- [`PF_MASKPROC`](#pf_maskproc) - Processor-specific segment flags.
- [`PF_MIPS_LOCAL`](#pf_mips_local)
- [`PF_PARISC_SBP`](#pf_parisc_sbp)
- [`PF_R`](#pf_r) - Segment is readable.
- [`PF_W`](#pf_w) - Segment is writable.
- [`PF_X`](#pf_x) - Segment is executable.
- [`PN_XNUM`](#pn_xnum) - Special value for `FileHeader*::e_phnum`.
- [`PPC64_OPT_LOCALENTRY`](#ppc64_opt_localentry)
- [`PPC64_OPT_MULTI_TOC`](#ppc64_opt_multi_toc)
- [`PPC64_OPT_TLS`](#ppc64_opt_tls)
- [`PPC_OPT_TLS`](#ppc_opt_tls)
- [`PT_ARM_EXIDX`](#pt_arm_exidx) - ARM unwind segment.
- [`PT_DYNAMIC`](#pt_dynamic) - Dynamic linking information.
- [`PT_GNU_EH_FRAME`](#pt_gnu_eh_frame) - GCC `.eh_frame_hdr` segment.
- [`PT_GNU_PROPERTY`](#pt_gnu_property) - Segment containing `.note.gnu.property` section.
- [`PT_GNU_RELRO`](#pt_gnu_relro) - Read-only after relocation.
- [`PT_GNU_SFRAME`](#pt_gnu_sframe) - GNU SFrame stack trace format.
- [`PT_GNU_STACK`](#pt_gnu_stack) - Indicates stack executability.
- [`PT_HIOS`](#pt_hios) - End of OS-specific segment types.
- [`PT_HIPROC`](#pt_hiproc) - End of processor-specific segment types.
- [`PT_HP_CORE_COMM`](#pt_hp_core_comm)
- [`PT_HP_CORE_KERNEL`](#pt_hp_core_kernel)
- [`PT_HP_CORE_LOADABLE`](#pt_hp_core_loadable)
- [`PT_HP_CORE_MMF`](#pt_hp_core_mmf)
- [`PT_HP_CORE_NONE`](#pt_hp_core_none)
- [`PT_HP_CORE_PROC`](#pt_hp_core_proc)
- [`PT_HP_CORE_SHM`](#pt_hp_core_shm)
- [`PT_HP_CORE_STACK`](#pt_hp_core_stack)
- [`PT_HP_CORE_VERSION`](#pt_hp_core_version)
- [`PT_HP_FASTBIND`](#pt_hp_fastbind)
- [`PT_HP_HSL_ANNOT`](#pt_hp_hsl_annot)
- [`PT_HP_OPT_ANNOT`](#pt_hp_opt_annot)
- [`PT_HP_PARALLEL`](#pt_hp_parallel)
- [`PT_HP_STACK`](#pt_hp_stack)
- [`PT_HP_TLS`](#pt_hp_tls)
- [`PT_IA_64_ARCHEXT`](#pt_ia_64_archext) - arch extension bits
- [`PT_IA_64_HP_HSL_ANOT`](#pt_ia_64_hp_hsl_anot)
- [`PT_IA_64_HP_OPT_ANOT`](#pt_ia_64_hp_opt_anot)
- [`PT_IA_64_HP_STACK`](#pt_ia_64_hp_stack)
- [`PT_IA_64_UNWIND`](#pt_ia_64_unwind) - ia64 unwind bits
- [`PT_INTERP`](#pt_interp) - Program interpreter.
- [`PT_LOAD`](#pt_load) - Loadable program segment.
- [`PT_LOOS`](#pt_loos) - Start of OS-specific segment types.
- [`PT_LOPROC`](#pt_loproc) - Start of processor-specific segment types.
- [`PT_MIPS_ABIFLAGS`](#pt_mips_abiflags) - FP mode requirement.
- [`PT_MIPS_OPTIONS`](#pt_mips_options)
- [`PT_MIPS_REGINFO`](#pt_mips_reginfo) - Register usage information.
- [`PT_MIPS_RTPROC`](#pt_mips_rtproc) - Runtime procedure table.
- [`PT_NOTE`](#pt_note) - Auxiliary information.
- [`PT_NULL`](#pt_null) - Program header table entry is unused.
- [`PT_PARISC_ARCHEXT`](#pt_parisc_archext)
- [`PT_PARISC_UNWIND`](#pt_parisc_unwind)
- [`PT_PHDR`](#pt_phdr) - Segment contains the program header table.
- [`PT_RISCV_ATTRIBUTES`](#pt_riscv_attributes)
- [`PT_SHLIB`](#pt_shlib) - Reserved.
- [`PT_TLS`](#pt_tls) - Thread-local storage segment.
- [`RHF_CORD`](#rhf_cord)
- [`RHF_DEFAULT_DELAY_LOAD`](#rhf_default_delay_load)
- [`RHF_DELTA_C_PLUS_PLUS`](#rhf_delta_c_plus_plus)
- [`RHF_GUARANTEE_INIT`](#rhf_guarantee_init)
- [`RHF_GUARANTEE_START_INIT`](#rhf_guarantee_start_init)
- [`RHF_NONE`](#rhf_none) - No flags
- [`RHF_NOTPOT`](#rhf_notpot) - Hash size not power of 2
- [`RHF_NO_LIBRARY_REPLACEMENT`](#rhf_no_library_replacement) - Ignore LD_LIBRARY_PATH
- [`RHF_NO_MOVE`](#rhf_no_move)
- [`RHF_NO_UNRES_UNDEF`](#rhf_no_unres_undef)
- [`RHF_PIXIE`](#rhf_pixie)
- [`RHF_QUICKSTART`](#rhf_quickstart) - Use quickstart
- [`RHF_REQUICKSTART`](#rhf_requickstart)
- [`RHF_REQUICKSTARTED`](#rhf_requickstarted)
- [`RHF_RLD_ORDER_SAFE`](#rhf_rld_order_safe)
- [`RHF_SGI_ONLY`](#rhf_sgi_only)
- [`R_386_16`](#r_386_16) - Direct 16 bit
- [`R_386_32`](#r_386_32) - Direct 32 bit
- [`R_386_32PLT`](#r_386_32plt) - Direct 32 bit PLT address
- [`R_386_8`](#r_386_8) - Direct 8 bit
- [`R_386_COPY`](#r_386_copy) - Copy symbol at runtime
- [`R_386_GLOB_DAT`](#r_386_glob_dat) - Create GOT entry
- [`R_386_GOT32`](#r_386_got32) - 32 bit GOT entry
- [`R_386_GOT32X`](#r_386_got32x) - Load from 32 bit GOT entry, relaxable.
- [`R_386_GOTOFF`](#r_386_gotoff) - 32 bit offset to GOT
- [`R_386_GOTPC`](#r_386_gotpc) - 32 bit PC relative offset to GOT
- [`R_386_IRELATIVE`](#r_386_irelative) - Adjust indirectly by program base
- [`R_386_JMP_SLOT`](#r_386_jmp_slot) - Create PLT entry
- [`R_386_NONE`](#r_386_none) - No reloc
- [`R_386_PC16`](#r_386_pc16) - PC relative 16 bit
- [`R_386_PC32`](#r_386_pc32) - PC relative 32 bit
- [`R_386_PC8`](#r_386_pc8) - PC relative 8 bit
- [`R_386_PLT32`](#r_386_plt32) - 32 bit PLT address
- [`R_386_RELATIVE`](#r_386_relative) - Adjust by program base
- [`R_386_SIZE32`](#r_386_size32) - 32-bit symbol size
- [`R_386_TLS_DESC`](#r_386_tls_desc) - TLS descriptor containing pointer to code and to argument, returning the TLS offset for the symbol.
- [`R_386_TLS_DESC_CALL`](#r_386_tls_desc_call) - Marker of call through TLS descriptor for relaxation.
- [`R_386_TLS_DTPMOD32`](#r_386_tls_dtpmod32) - ID of module containing symbol
- [`R_386_TLS_DTPOFF32`](#r_386_tls_dtpoff32) - Offset in TLS block
- [`R_386_TLS_GD`](#r_386_tls_gd) - Direct 32 bit for GNU version of general dynamic thread local data
- [`R_386_TLS_GD_32`](#r_386_tls_gd_32) - Direct 32 bit for general dynamic thread local data
- [`R_386_TLS_GD_CALL`](#r_386_tls_gd_call) - Relocation for call to __tls_get_addr()
- [`R_386_TLS_GD_POP`](#r_386_tls_gd_pop) - Tag for popl in GD TLS code
- [`R_386_TLS_GD_PUSH`](#r_386_tls_gd_push) - Tag for pushl in GD TLS code
- [`R_386_TLS_GOTDESC`](#r_386_tls_gotdesc) - GOT offset for TLS descriptor.
- [`R_386_TLS_GOTIE`](#r_386_tls_gotie) - GOT entry for static TLS block offset
- [`R_386_TLS_IE`](#r_386_tls_ie) - Address of GOT entry for static TLS block offset
- [`R_386_TLS_IE_32`](#r_386_tls_ie_32) - GOT entry for negated static TLS block offset
- [`R_386_TLS_LDM`](#r_386_tls_ldm) - Direct 32 bit for GNU version of local dynamic thread local data in LE code
- [`R_386_TLS_LDM_32`](#r_386_tls_ldm_32) - Direct 32 bit for local dynamic thread local data in LE code
- [`R_386_TLS_LDM_CALL`](#r_386_tls_ldm_call) - Relocation for call to __tls_get_addr() in LDM code
- [`R_386_TLS_LDM_POP`](#r_386_tls_ldm_pop) - Tag for popl in LDM TLS code
- [`R_386_TLS_LDM_PUSH`](#r_386_tls_ldm_push) - Tag for pushl in LDM TLS code
- [`R_386_TLS_LDO_32`](#r_386_tls_ldo_32) - Offset relative to TLS block
- [`R_386_TLS_LE`](#r_386_tls_le) - Offset relative to static TLS block
- [`R_386_TLS_LE_32`](#r_386_tls_le_32) - Negated offset relative to static TLS block
- [`R_386_TLS_TPOFF`](#r_386_tls_tpoff) - Offset in static TLS block
- [`R_386_TLS_TPOFF32`](#r_386_tls_tpoff32) - Negated offset in static TLS block
- [`R_390_12`](#r_390_12) - Direct 12 bit.
- [`R_390_16`](#r_390_16) - Direct 16 bit.
- [`R_390_20`](#r_390_20) - Direct 20 bit.
- [`R_390_32`](#r_390_32) - Direct 32 bit.
- [`R_390_64`](#r_390_64) - Direct 64 bit.
- [`R_390_8`](#r_390_8) - Direct 8 bit.
- [`R_390_COPY`](#r_390_copy) - Copy symbol at runtime.
- [`R_390_GLOB_DAT`](#r_390_glob_dat) - Create GOT entry.
- [`R_390_GOT12`](#r_390_got12) - 12 bit GOT offset.
- [`R_390_GOT16`](#r_390_got16) - 16 bit GOT offset.
- [`R_390_GOT20`](#r_390_got20) - 20 bit GOT offset.
- [`R_390_GOT32`](#r_390_got32) - 32 bit GOT offset.
- [`R_390_GOT64`](#r_390_got64) - 64 bit GOT offset.
- [`R_390_GOTENT`](#r_390_gotent) - 32 bit PC rel. to GOT entry >> 1.
- [`R_390_GOTOFF16`](#r_390_gotoff16) - 16 bit offset to GOT.
- [`R_390_GOTOFF32`](#r_390_gotoff32) - 32 bit offset to GOT.
- [`R_390_GOTOFF64`](#r_390_gotoff64) - 64 bit offset to GOT.
- [`R_390_GOTPC`](#r_390_gotpc) - 32 bit PC relative offset to GOT.
- [`R_390_GOTPCDBL`](#r_390_gotpcdbl) - 32 bit PC rel. GOT shifted by 1.
- [`R_390_GOTPLT12`](#r_390_gotplt12) - 12 bit offset to jump slot.
- [`R_390_GOTPLT16`](#r_390_gotplt16) - 16 bit offset to jump slot.
- [`R_390_GOTPLT20`](#r_390_gotplt20) - 20 bit offset to jump slot.
- [`R_390_GOTPLT32`](#r_390_gotplt32) - 32 bit offset to jump slot.
- [`R_390_GOTPLT64`](#r_390_gotplt64) - 64 bit offset to jump slot.
- [`R_390_GOTPLTENT`](#r_390_gotpltent) - 32 bit rel. offset to jump slot.
- [`R_390_IRELATIVE`](#r_390_irelative) - STT_GNU_IFUNC relocation.
- [`R_390_JMP_SLOT`](#r_390_jmp_slot) - Create PLT entry.
- [`R_390_NONE`](#r_390_none) - No reloc.
- [`R_390_PC16`](#r_390_pc16) - PC relative 16 bit.
- [`R_390_PC16DBL`](#r_390_pc16dbl) - PC relative 16 bit shifted by 1.
- [`R_390_PC32`](#r_390_pc32) - PC relative 32 bit.
- [`R_390_PC32DBL`](#r_390_pc32dbl) - PC relative 32 bit shifted by 1.
- [`R_390_PC64`](#r_390_pc64) - PC relative 64 bit.
- [`R_390_PLT16DBL`](#r_390_plt16dbl) - 16 bit PC rel. PLT shifted by 1.
- [`R_390_PLT32`](#r_390_plt32) - 32 bit PC relative PLT address.
- [`R_390_PLT32DBL`](#r_390_plt32dbl) - 32 bit PC rel. PLT shifted by 1.
- [`R_390_PLT64`](#r_390_plt64) - 64 bit PC relative PLT address.
- [`R_390_PLTOFF16`](#r_390_pltoff16) - 16 bit offset from GOT to PLT.
- [`R_390_PLTOFF32`](#r_390_pltoff32) - 32 bit offset from GOT to PLT.
- [`R_390_PLTOFF64`](#r_390_pltoff64) - 16 bit offset from GOT to PLT.
- [`R_390_RELATIVE`](#r_390_relative) - Adjust by program base.
- [`R_390_TLS_DTPMOD`](#r_390_tls_dtpmod) - ID of module containing symbol.
- [`R_390_TLS_DTPOFF`](#r_390_tls_dtpoff) - Offset in TLS block.
- [`R_390_TLS_GD32`](#r_390_tls_gd32) - Direct 32 bit for general dynamic thread local data.
- [`R_390_TLS_GD64`](#r_390_tls_gd64) - Direct 64 bit for general dynamic thread local data.
- [`R_390_TLS_GDCALL`](#r_390_tls_gdcall) - Tag for function call in general dynamic TLS code.
- [`R_390_TLS_GOTIE12`](#r_390_tls_gotie12) - 12 bit GOT offset for static TLS block offset.
- [`R_390_TLS_GOTIE20`](#r_390_tls_gotie20) - 20 bit GOT offset for static TLS block offset.
- [`R_390_TLS_GOTIE32`](#r_390_tls_gotie32) - 32 bit GOT offset for static TLS block offset.
- [`R_390_TLS_GOTIE64`](#r_390_tls_gotie64) - 64 bit GOT offset for static TLS block offset.
- [`R_390_TLS_IE32`](#r_390_tls_ie32) - 32 bit address of GOT entry for negated static TLS block offset.
- [`R_390_TLS_IE64`](#r_390_tls_ie64) - 64 bit address of GOT entry for negated static TLS block offset.
- [`R_390_TLS_IEENT`](#r_390_tls_ieent) - 32 bit rel. offset to GOT entry for negated static TLS block offset.
- [`R_390_TLS_LDCALL`](#r_390_tls_ldcall) - Tag for function call in local dynamic TLS code.
- [`R_390_TLS_LDM32`](#r_390_tls_ldm32) - Direct 32 bit for local dynamic thread local data in LE code.
- [`R_390_TLS_LDM64`](#r_390_tls_ldm64) - Direct 64 bit for local dynamic thread local data in LE code.
- [`R_390_TLS_LDO32`](#r_390_tls_ldo32) - 32 bit offset relative to TLS block.
- [`R_390_TLS_LDO64`](#r_390_tls_ldo64) - 64 bit offset relative to TLS block.
- [`R_390_TLS_LE32`](#r_390_tls_le32) - 32 bit negated offset relative to static TLS block.
- [`R_390_TLS_LE64`](#r_390_tls_le64) - 64 bit negated offset relative to static TLS block.
- [`R_390_TLS_LOAD`](#r_390_tls_load) - Tag for load insn in TLS code.
- [`R_390_TLS_TPOFF`](#r_390_tls_tpoff) - Negated offset in static TLS block.
- [`R_68K_16`](#r_68k_16) - Direct 16 bit
- [`R_68K_32`](#r_68k_32) - Direct 32 bit
- [`R_68K_8`](#r_68k_8) - Direct 8 bit
- [`R_68K_COPY`](#r_68k_copy) - Copy symbol at runtime
- [`R_68K_GLOB_DAT`](#r_68k_glob_dat) - Create GOT entry
- [`R_68K_GOT16`](#r_68k_got16) - 16 bit PC relative GOT entry
- [`R_68K_GOT16O`](#r_68k_got16o) - 16 bit GOT offset
- [`R_68K_GOT32`](#r_68k_got32) - 32 bit PC relative GOT entry
- [`R_68K_GOT32O`](#r_68k_got32o) - 32 bit GOT offset
- [`R_68K_GOT8`](#r_68k_got8) - 8 bit PC relative GOT entry
- [`R_68K_GOT8O`](#r_68k_got8o) - 8 bit GOT offset
- [`R_68K_JMP_SLOT`](#r_68k_jmp_slot) - Create PLT entry
- [`R_68K_NONE`](#r_68k_none) - No reloc
- [`R_68K_PC16`](#r_68k_pc16) - PC relative 16 bit
- [`R_68K_PC32`](#r_68k_pc32) - PC relative 32 bit
- [`R_68K_PC8`](#r_68k_pc8) - PC relative 8 bit
- [`R_68K_PLT16`](#r_68k_plt16) - 16 bit PC relative PLT address
- [`R_68K_PLT16O`](#r_68k_plt16o) - 16 bit PLT offset
- [`R_68K_PLT32`](#r_68k_plt32) - 32 bit PC relative PLT address
- [`R_68K_PLT32O`](#r_68k_plt32o) - 32 bit PLT offset
- [`R_68K_PLT8`](#r_68k_plt8) - 8 bit PC relative PLT address
- [`R_68K_PLT8O`](#r_68k_plt8o) - 8 bit PLT offset
- [`R_68K_RELATIVE`](#r_68k_relative) - Adjust by program base
- [`R_68K_TLS_DTPMOD32`](#r_68k_tls_dtpmod32) - 32 bit module number
- [`R_68K_TLS_DTPREL32`](#r_68k_tls_dtprel32) - 32 bit module-relative offset
- [`R_68K_TLS_GD16`](#r_68k_tls_gd16) - 16 bit GOT offset for GD
- [`R_68K_TLS_GD32`](#r_68k_tls_gd32) - 32 bit GOT offset for GD
- [`R_68K_TLS_GD8`](#r_68k_tls_gd8) - 8 bit GOT offset for GD
- [`R_68K_TLS_IE16`](#r_68k_tls_ie16) - 16 bit GOT offset for IE
- [`R_68K_TLS_IE32`](#r_68k_tls_ie32) - 32 bit GOT offset for IE
- [`R_68K_TLS_IE8`](#r_68k_tls_ie8) - 8 bit GOT offset for IE
- [`R_68K_TLS_LDM16`](#r_68k_tls_ldm16) - 16 bit GOT offset for LDM
- [`R_68K_TLS_LDM32`](#r_68k_tls_ldm32) - 32 bit GOT offset for LDM
- [`R_68K_TLS_LDM8`](#r_68k_tls_ldm8) - 8 bit GOT offset for LDM
- [`R_68K_TLS_LDO16`](#r_68k_tls_ldo16) - 16 bit module-relative offset
- [`R_68K_TLS_LDO32`](#r_68k_tls_ldo32) - 32 bit module-relative offset
- [`R_68K_TLS_LDO8`](#r_68k_tls_ldo8) - 8 bit module-relative offset
- [`R_68K_TLS_LE16`](#r_68k_tls_le16) - 16 bit offset relative to static TLS block
- [`R_68K_TLS_LE32`](#r_68k_tls_le32) - 32 bit offset relative to static TLS block
- [`R_68K_TLS_LE8`](#r_68k_tls_le8) - 8 bit offset relative to static TLS block
- [`R_68K_TLS_TPREL32`](#r_68k_tls_tprel32) - 32 bit TP-relative offset
- [`R_AARCH64_ABS16`](#r_aarch64_abs16) - Direct 16-bit.
- [`R_AARCH64_ABS32`](#r_aarch64_abs32) - Direct 32 bit.
- [`R_AARCH64_ABS64`](#r_aarch64_abs64) - Direct 64 bit.
- [`R_AARCH64_ADD_ABS_LO12_NC`](#r_aarch64_add_abs_lo12_nc) - Dir. ADD imm. from bits 11:0.
- [`R_AARCH64_ADR_GOT_PAGE`](#r_aarch64_adr_got_page) - P-page-rel. GOT off. ADRP 32:12.
- [`R_AARCH64_ADR_PREL_LO21`](#r_aarch64_adr_prel_lo21) - PC-rel. ADR imm. from bits 20:0.
- [`R_AARCH64_ADR_PREL_PG_HI21`](#r_aarch64_adr_prel_pg_hi21) - Page-rel. ADRP imm. from 32:12.
- [`R_AARCH64_ADR_PREL_PG_HI21_NC`](#r_aarch64_adr_prel_pg_hi21_nc) - Likewise; no overflow check.
- [`R_AARCH64_CALL26`](#r_aarch64_call26) - Likewise for CALL.
- [`R_AARCH64_CONDBR19`](#r_aarch64_condbr19) - PC-rel. cond. br. imm. from 20:2.
- [`R_AARCH64_COPY`](#r_aarch64_copy) - Copy symbol at runtime.
- [`R_AARCH64_GLOB_DAT`](#r_aarch64_glob_dat) - Create GOT entry.
- [`R_AARCH64_GOTREL32`](#r_aarch64_gotrel32) - GOT-relative 32-bit.
- [`R_AARCH64_GOTREL64`](#r_aarch64_gotrel64) - GOT-relative 64-bit.
- [`R_AARCH64_GOT_LD_PREL19`](#r_aarch64_got_ld_prel19) - PC-rel. GOT off. load imm. 20:2.
- [`R_AARCH64_IRELATIVE`](#r_aarch64_irelative) - STT_GNU_IFUNC relocation.
- [`R_AARCH64_JUMP26`](#r_aarch64_jump26) - PC-rel. B imm. from bits 27:2.
- [`R_AARCH64_JUMP_SLOT`](#r_aarch64_jump_slot) - Create PLT entry.
- [`R_AARCH64_LD64_GOTOFF_LO15`](#r_aarch64_ld64_gotoff_lo15) - GOT-rel. off. LD/ST imm. 14:3.
- [`R_AARCH64_LD64_GOTPAGE_LO15`](#r_aarch64_ld64_gotpage_lo15) - GOT-page-rel. GOT off. LD/ST 14:3
- [`R_AARCH64_LD64_GOT_LO12_NC`](#r_aarch64_ld64_got_lo12_nc) - Dir. GOT off. LD/ST imm. 11:3.
- [`R_AARCH64_LDST128_ABS_LO12_NC`](#r_aarch64_ldst128_abs_lo12_nc) - Dir. ADD imm. from bits 11:4.
- [`R_AARCH64_LDST16_ABS_LO12_NC`](#r_aarch64_ldst16_abs_lo12_nc) - Dir. ADD imm. from bits 11:1.
- [`R_AARCH64_LDST32_ABS_LO12_NC`](#r_aarch64_ldst32_abs_lo12_nc) - Likewise for bits 11:2.
- [`R_AARCH64_LDST64_ABS_LO12_NC`](#r_aarch64_ldst64_abs_lo12_nc) - Likewise for bits 11:3.
- [`R_AARCH64_LDST8_ABS_LO12_NC`](#r_aarch64_ldst8_abs_lo12_nc) - Likewise for LD/ST; no check.
- [`R_AARCH64_LD_PREL_LO19`](#r_aarch64_ld_prel_lo19) - PC-rel. LD imm. from bits 20:2.
- [`R_AARCH64_MOVW_GOTOFF_G0`](#r_aarch64_movw_gotoff_g0) - GOT-rel. off. MOV{N,Z} imm. 15:0.
- [`R_AARCH64_MOVW_GOTOFF_G0_NC`](#r_aarch64_movw_gotoff_g0_nc) - Likewise for MOVK; no check.
- [`R_AARCH64_MOVW_GOTOFF_G1`](#r_aarch64_movw_gotoff_g1) - GOT-rel. o. MOV{N,Z} imm. 31:16.
- [`R_AARCH64_MOVW_GOTOFF_G1_NC`](#r_aarch64_movw_gotoff_g1_nc) - Likewise for MOVK; no check.
- [`R_AARCH64_MOVW_GOTOFF_G2`](#r_aarch64_movw_gotoff_g2) - GOT-rel. o. MOV{N,Z} imm. 47:32.
- [`R_AARCH64_MOVW_GOTOFF_G2_NC`](#r_aarch64_movw_gotoff_g2_nc) - Likewise for MOVK; no check.
- [`R_AARCH64_MOVW_GOTOFF_G3`](#r_aarch64_movw_gotoff_g3) - GOT-rel. o. MOV{N,Z} imm. 63:48.
- [`R_AARCH64_MOVW_PREL_G0`](#r_aarch64_movw_prel_g0) - PC-rel. MOV{N,Z} imm. from 15:0.
- [`R_AARCH64_MOVW_PREL_G0_NC`](#r_aarch64_movw_prel_g0_nc) - Likewise for MOVK; no check.
- [`R_AARCH64_MOVW_PREL_G1`](#r_aarch64_movw_prel_g1) - PC-rel. MOV{N,Z} imm. from 31:16.
- [`R_AARCH64_MOVW_PREL_G1_NC`](#r_aarch64_movw_prel_g1_nc) - Likewise for MOVK; no check.
- [`R_AARCH64_MOVW_PREL_G2`](#r_aarch64_movw_prel_g2) - PC-rel. MOV{N,Z} imm. from 47:32.
- [`R_AARCH64_MOVW_PREL_G2_NC`](#r_aarch64_movw_prel_g2_nc) - Likewise for MOVK; no check.
- [`R_AARCH64_MOVW_PREL_G3`](#r_aarch64_movw_prel_g3) - PC-rel. MOV{N,Z} imm. from 63:48.
- [`R_AARCH64_MOVW_SABS_G0`](#r_aarch64_movw_sabs_g0) - Dir. MOV{N,Z} imm. from 15:0.
- [`R_AARCH64_MOVW_SABS_G1`](#r_aarch64_movw_sabs_g1) - Dir. MOV{N,Z} imm. from 31:16.
- [`R_AARCH64_MOVW_SABS_G2`](#r_aarch64_movw_sabs_g2) - Dir. MOV{N,Z} imm. from 47:32.
- [`R_AARCH64_MOVW_UABS_G0`](#r_aarch64_movw_uabs_g0) - Dir. MOVZ imm. from bits 15:0.
- [`R_AARCH64_MOVW_UABS_G0_NC`](#r_aarch64_movw_uabs_g0_nc) - Likewise for MOVK; no check.
- [`R_AARCH64_MOVW_UABS_G1`](#r_aarch64_movw_uabs_g1) - Dir. MOVZ imm. from bits 31:16.
- [`R_AARCH64_MOVW_UABS_G1_NC`](#r_aarch64_movw_uabs_g1_nc) - Likewise for MOVK; no check.
- [`R_AARCH64_MOVW_UABS_G2`](#r_aarch64_movw_uabs_g2) - Dir. MOVZ imm. from bits 47:32.
- [`R_AARCH64_MOVW_UABS_G2_NC`](#r_aarch64_movw_uabs_g2_nc) - Likewise for MOVK; no check.
- [`R_AARCH64_MOVW_UABS_G3`](#r_aarch64_movw_uabs_g3) - Dir. MOV{K,Z} imm. from 63:48.
- [`R_AARCH64_NONE`](#r_aarch64_none) - No relocation.
- [`R_AARCH64_P32_ABS32`](#r_aarch64_p32_abs32) - Direct 32 bit.
- [`R_AARCH64_P32_COPY`](#r_aarch64_p32_copy) - Copy symbol at runtime.
- [`R_AARCH64_P32_GLOB_DAT`](#r_aarch64_p32_glob_dat) - Create GOT entry.
- [`R_AARCH64_P32_IRELATIVE`](#r_aarch64_p32_irelative) - STT_GNU_IFUNC relocation.
- [`R_AARCH64_P32_JUMP_SLOT`](#r_aarch64_p32_jump_slot) - Create PLT entry.
- [`R_AARCH64_P32_RELATIVE`](#r_aarch64_p32_relative) - Adjust by program base.
- [`R_AARCH64_P32_TLSDESC`](#r_aarch64_p32_tlsdesc) - TLS Descriptor.
- [`R_AARCH64_P32_TLS_DTPMOD`](#r_aarch64_p32_tls_dtpmod) - Module number, 32 bit.
- [`R_AARCH64_P32_TLS_DTPREL`](#r_aarch64_p32_tls_dtprel) - Module-relative offset, 32 bit.
- [`R_AARCH64_P32_TLS_TPREL`](#r_aarch64_p32_tls_tprel) - TP-relative offset, 32 bit.
- [`R_AARCH64_PREL16`](#r_aarch64_prel16) - PC-relative 16-bit.
- [`R_AARCH64_PREL32`](#r_aarch64_prel32) - PC-relative 32-bit.
- [`R_AARCH64_PREL64`](#r_aarch64_prel64) - PC-relative 64-bit.
- [`R_AARCH64_RELATIVE`](#r_aarch64_relative) - Adjust by program base.
- [`R_AARCH64_TLSDESC`](#r_aarch64_tlsdesc) - TLS Descriptor.
- [`R_AARCH64_TLSDESC_ADD`](#r_aarch64_tlsdesc_add) - Relax ADD.
- [`R_AARCH64_TLSDESC_ADD_LO12`](#r_aarch64_tlsdesc_add_lo12) - Direct ADD imm. from 11:0.
- [`R_AARCH64_TLSDESC_ADR_PAGE21`](#r_aarch64_tlsdesc_adr_page21) - Page-rel. ADRP imm. 32:12.
- [`R_AARCH64_TLSDESC_ADR_PREL21`](#r_aarch64_tlsdesc_adr_prel21) - PC-rel. ADR immediate 20:0.
- [`R_AARCH64_TLSDESC_CALL`](#r_aarch64_tlsdesc_call) - Relax BLR.
- [`R_AARCH64_TLSDESC_LD64_LO12`](#r_aarch64_tlsdesc_ld64_lo12) - Direct LD off. from 11:3.
- [`R_AARCH64_TLSDESC_LDR`](#r_aarch64_tlsdesc_ldr) - Relax LDR.
- [`R_AARCH64_TLSDESC_LD_PREL19`](#r_aarch64_tlsdesc_ld_prel19) - PC-rel. load immediate 20:2.
- [`R_AARCH64_TLSDESC_OFF_G0_NC`](#r_aarch64_tlsdesc_off_g0_nc) - GOT-rel. MOVK imm. 15:0; no ck.
- [`R_AARCH64_TLSDESC_OFF_G1`](#r_aarch64_tlsdesc_off_g1) - GOT-rel. MOV{N,Z} imm. 31:16.
- [`R_AARCH64_TLSGD_ADD_LO12_NC`](#r_aarch64_tlsgd_add_lo12_nc) - direct ADD imm. from 11:0.
- [`R_AARCH64_TLSGD_ADR_PAGE21`](#r_aarch64_tlsgd_adr_page21) - page-rel. ADRP imm. 32:12.
- [`R_AARCH64_TLSGD_ADR_PREL21`](#r_aarch64_tlsgd_adr_prel21) - PC-relative ADR imm. 20:0.
- [`R_AARCH64_TLSGD_MOVW_G0_NC`](#r_aarch64_tlsgd_movw_g0_nc) - GOT-rel. MOVK imm. 15:0.
- [`R_AARCH64_TLSGD_MOVW_G1`](#r_aarch64_tlsgd_movw_g1) - GOT-rel. MOV{N,Z} 31:16.
- [`R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21`](#r_aarch64_tlsie_adr_gottprel_page21) - Page-rel. ADRP 32:12.
- [`R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC`](#r_aarch64_tlsie_ld64_gottprel_lo12_nc) - Direct LD off. 11:3.
- [`R_AARCH64_TLSIE_LD_GOTTPREL_PREL19`](#r_aarch64_tlsie_ld_gottprel_prel19) - PC-rel. load imm. 20:2.
- [`R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC`](#r_aarch64_tlsie_movw_gottprel_g0_nc) - GOT-rel. MOVK 15:0.
- [`R_AARCH64_TLSIE_MOVW_GOTTPREL_G1`](#r_aarch64_tlsie_movw_gottprel_g1) - GOT-rel. MOV{N,Z} 31:16.
- [`R_AARCH64_TLSLD_ADD_DTPREL_HI12`](#r_aarch64_tlsld_add_dtprel_hi12) - DTP-rel. ADD imm. from 23:12.
- [`R_AARCH64_TLSLD_ADD_DTPREL_LO12`](#r_aarch64_tlsld_add_dtprel_lo12) - DTP-rel. ADD imm. from 11:0.
- [`R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC`](#r_aarch64_tlsld_add_dtprel_lo12_nc) - Likewise; no ovfl. check.
- [`R_AARCH64_TLSLD_ADD_LO12_NC`](#r_aarch64_tlsld_add_lo12_nc) - Like 514; local dynamic model.
- [`R_AARCH64_TLSLD_ADR_PAGE21`](#r_aarch64_tlsld_adr_page21) - Like 513; local dynamic model.
- [`R_AARCH64_TLSLD_ADR_PREL21`](#r_aarch64_tlsld_adr_prel21) - Like 512; local dynamic model.
- [`R_AARCH64_TLSLD_LDST128_DTPREL_LO12`](#r_aarch64_tlsld_ldst128_dtprel_lo12) - DTP-rel. LD/ST imm. 11:4.
- [`R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst128_dtprel_lo12_nc) - Likewise; no check.
- [`R_AARCH64_TLSLD_LDST16_DTPREL_LO12`](#r_aarch64_tlsld_ldst16_dtprel_lo12) - DTP-rel. LD/ST imm. 11:1.
- [`R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst16_dtprel_lo12_nc) - Likewise; no check.
- [`R_AARCH64_TLSLD_LDST32_DTPREL_LO12`](#r_aarch64_tlsld_ldst32_dtprel_lo12) - DTP-rel. LD/ST imm. 11:2.
- [`R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst32_dtprel_lo12_nc) - Likewise; no check.
- [`R_AARCH64_TLSLD_LDST64_DTPREL_LO12`](#r_aarch64_tlsld_ldst64_dtprel_lo12) - DTP-rel. LD/ST imm. 11:3.
- [`R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst64_dtprel_lo12_nc) - Likewise; no check.
- [`R_AARCH64_TLSLD_LDST8_DTPREL_LO12`](#r_aarch64_tlsld_ldst8_dtprel_lo12) - DTP-rel. LD/ST imm. 11:0.
- [`R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC`](#r_aarch64_tlsld_ldst8_dtprel_lo12_nc) - Likewise; no check.
- [`R_AARCH64_TLSLD_LD_PREL19`](#r_aarch64_tlsld_ld_prel19) - TLS PC-rel. load imm. 20:2.
- [`R_AARCH64_TLSLD_MOVW_DTPREL_G0`](#r_aarch64_tlsld_movw_dtprel_g0) - TLS DTP-rel. MOV{N,Z} 15:0.
- [`R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC`](#r_aarch64_tlsld_movw_dtprel_g0_nc) - Likewise; MOVK; no check.
- [`R_AARCH64_TLSLD_MOVW_DTPREL_G1`](#r_aarch64_tlsld_movw_dtprel_g1) - TLS DTP-rel. MOV{N,Z} 31:16.
- [`R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC`](#r_aarch64_tlsld_movw_dtprel_g1_nc) - Likewise; MOVK; no check.
- [`R_AARCH64_TLSLD_MOVW_DTPREL_G2`](#r_aarch64_tlsld_movw_dtprel_g2) - TLS DTP-rel. MOV{N,Z} 47:32.
- [`R_AARCH64_TLSLD_MOVW_G0_NC`](#r_aarch64_tlsld_movw_g0_nc) - Like 516; local dynamic model.
- [`R_AARCH64_TLSLD_MOVW_G1`](#r_aarch64_tlsld_movw_g1) - Like 515; local dynamic model.
- [`R_AARCH64_TLSLE_ADD_TPREL_HI12`](#r_aarch64_tlsle_add_tprel_hi12) - TP-rel. ADD imm. 23:12.
- [`R_AARCH64_TLSLE_ADD_TPREL_LO12`](#r_aarch64_tlsle_add_tprel_lo12) - TP-rel. ADD imm. 11:0.
- [`R_AARCH64_TLSLE_ADD_TPREL_LO12_NC`](#r_aarch64_tlsle_add_tprel_lo12_nc) - Likewise; no ovfl. check.
- [`R_AARCH64_TLSLE_LDST128_TPREL_LO12`](#r_aarch64_tlsle_ldst128_tprel_lo12) - TP-rel. LD/ST off. 11:4.
- [`R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst128_tprel_lo12_nc) - Likewise; no check.
- [`R_AARCH64_TLSLE_LDST16_TPREL_LO12`](#r_aarch64_tlsle_ldst16_tprel_lo12) - TP-rel. LD/ST off. 11:1.
- [`R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst16_tprel_lo12_nc) - Likewise; no check.
- [`R_AARCH64_TLSLE_LDST32_TPREL_LO12`](#r_aarch64_tlsle_ldst32_tprel_lo12) - TP-rel. LD/ST off. 11:2.
- [`R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst32_tprel_lo12_nc) - Likewise; no check.
- [`R_AARCH64_TLSLE_LDST64_TPREL_LO12`](#r_aarch64_tlsle_ldst64_tprel_lo12) - TP-rel. LD/ST off. 11:3.
- [`R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst64_tprel_lo12_nc) - Likewise; no check.
- [`R_AARCH64_TLSLE_LDST8_TPREL_LO12`](#r_aarch64_tlsle_ldst8_tprel_lo12) - TP-rel. LD/ST off. 11:0.
- [`R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC`](#r_aarch64_tlsle_ldst8_tprel_lo12_nc) - Likewise; no ovfl. check.
- [`R_AARCH64_TLSLE_MOVW_TPREL_G0`](#r_aarch64_tlsle_movw_tprel_g0) - TLS TP-rel. MOV{N,Z} 15:0.
- [`R_AARCH64_TLSLE_MOVW_TPREL_G0_NC`](#r_aarch64_tlsle_movw_tprel_g0_nc) - Likewise; MOVK; no check.
- [`R_AARCH64_TLSLE_MOVW_TPREL_G1`](#r_aarch64_tlsle_movw_tprel_g1) - TLS TP-rel. MOV{N,Z} 31:16.
- [`R_AARCH64_TLSLE_MOVW_TPREL_G1_NC`](#r_aarch64_tlsle_movw_tprel_g1_nc) - Likewise; MOVK; no check.
- [`R_AARCH64_TLSLE_MOVW_TPREL_G2`](#r_aarch64_tlsle_movw_tprel_g2) - TLS TP-rel. MOV{N,Z} 47:32.
- [`R_AARCH64_TLS_DTPMOD`](#r_aarch64_tls_dtpmod) - Module number, 64 bit.
- [`R_AARCH64_TLS_DTPREL`](#r_aarch64_tls_dtprel) - Module-relative offset, 64 bit.
- [`R_AARCH64_TLS_TPREL`](#r_aarch64_tls_tprel) - TP-relative offset, 64 bit.
- [`R_AARCH64_TSTBR14`](#r_aarch64_tstbr14) - PC-rel. TBZ/TBNZ imm. from 15:2.
- [`R_ALPHA_BRADDR`](#r_alpha_braddr) - PC+4 relative 23 bit shifted
- [`R_ALPHA_COPY`](#r_alpha_copy) - Copy symbol at runtime
- [`R_ALPHA_DTPMOD64`](#r_alpha_dtpmod64)
- [`R_ALPHA_DTPREL16`](#r_alpha_dtprel16)
- [`R_ALPHA_DTPREL64`](#r_alpha_dtprel64)
- [`R_ALPHA_DTPRELHI`](#r_alpha_dtprelhi)
- [`R_ALPHA_DTPRELLO`](#r_alpha_dtprello)
- [`R_ALPHA_GLOB_DAT`](#r_alpha_glob_dat) - Create GOT entry
- [`R_ALPHA_GOTDTPREL`](#r_alpha_gotdtprel)
- [`R_ALPHA_GOTTPREL`](#r_alpha_gottprel)
- [`R_ALPHA_GPDISP`](#r_alpha_gpdisp) - Add displacement to GP
- [`R_ALPHA_GPREL16`](#r_alpha_gprel16) - GP relative 16 bit
- [`R_ALPHA_GPREL32`](#r_alpha_gprel32) - GP relative 32 bit
- [`R_ALPHA_GPRELHIGH`](#r_alpha_gprelhigh) - GP relative 32 bit, high 16 bits
- [`R_ALPHA_GPRELLOW`](#r_alpha_gprellow) - GP relative 32 bit, low 16 bits
- [`R_ALPHA_HINT`](#r_alpha_hint) - PC+4 relative 16 bit shifted
- [`R_ALPHA_JMP_SLOT`](#r_alpha_jmp_slot) - Create PLT entry
- [`R_ALPHA_LITERAL`](#r_alpha_literal) - GP relative 16 bit w/optimization
- [`R_ALPHA_LITUSE`](#r_alpha_lituse) - Optimization hint for LITERAL
- [`R_ALPHA_NONE`](#r_alpha_none) - No reloc
- [`R_ALPHA_REFLONG`](#r_alpha_reflong) - Direct 32 bit
- [`R_ALPHA_REFQUAD`](#r_alpha_refquad) - Direct 64 bit
- [`R_ALPHA_RELATIVE`](#r_alpha_relative) - Adjust by program base
- [`R_ALPHA_SREL16`](#r_alpha_srel16) - PC relative 16 bit
- [`R_ALPHA_SREL32`](#r_alpha_srel32) - PC relative 32 bit
- [`R_ALPHA_SREL64`](#r_alpha_srel64) - PC relative 64 bit
- [`R_ALPHA_TLSGD`](#r_alpha_tlsgd)
- [`R_ALPHA_TLS_GD_HI`](#r_alpha_tls_gd_hi)
- [`R_ALPHA_TLS_LDM`](#r_alpha_tls_ldm)
- [`R_ALPHA_TPREL16`](#r_alpha_tprel16)
- [`R_ALPHA_TPREL64`](#r_alpha_tprel64)
- [`R_ALPHA_TPRELHI`](#r_alpha_tprelhi)
- [`R_ALPHA_TPRELLO`](#r_alpha_tprello)
- [`R_ARM_ABS12`](#r_arm_abs12) - Direct 12 bit
- [`R_ARM_ABS16`](#r_arm_abs16) - Direct 16 bit
- [`R_ARM_ABS32`](#r_arm_abs32) - Direct 32 bit
- [`R_ARM_ABS32_NOI`](#r_arm_abs32_noi) - Direct 32-bit.
- [`R_ARM_ABS8`](#r_arm_abs8) - Direct 8 bit
- [`R_ARM_ALU_PCREL_15_8`](#r_arm_alu_pcrel_15_8) - Obsolete.
- [`R_ARM_ALU_PCREL_23_15`](#r_arm_alu_pcrel_23_15) - Obsolete.
- [`R_ARM_ALU_PCREL_7_0`](#r_arm_alu_pcrel_7_0) - Obsolete.
- [`R_ARM_ALU_PC_G0`](#r_arm_alu_pc_g0) - PC relative (`ADD`, `SUB`).
- [`R_ARM_ALU_PC_G0_NC`](#r_arm_alu_pc_g0_nc) - PC relative (`ADD`, `SUB`).
- [`R_ARM_ALU_PC_G1`](#r_arm_alu_pc_g1) - PC relative (`ADD`, `SUB`).
- [`R_ARM_ALU_PC_G1_NC`](#r_arm_alu_pc_g1_nc) - PC relative (`ADD`, `SUB`).
- [`R_ARM_ALU_PC_G2`](#r_arm_alu_pc_g2) - PC relative (`ADD`, `SUB`).
- [`R_ARM_ALU_SBREL_19_12`](#r_arm_alu_sbrel_19_12) - Deprecated, prog. base relative.
- [`R_ARM_ALU_SBREL_27_20`](#r_arm_alu_sbrel_27_20) - Deprecated, prog. base relative.
- [`R_ARM_ALU_SB_G0`](#r_arm_alu_sb_g0) - Program base relative (`ADD`,`SUB`).
- [`R_ARM_ALU_SB_G0_NC`](#r_arm_alu_sb_g0_nc) - Program base relative (`ADD`,`SUB`).
- [`R_ARM_ALU_SB_G1`](#r_arm_alu_sb_g1) - Program base relative (`ADD`,`SUB`).
- [`R_ARM_ALU_SB_G1_NC`](#r_arm_alu_sb_g1_nc) - Program base relative (`ADD`,`SUB`).
- [`R_ARM_ALU_SB_G2`](#r_arm_alu_sb_g2) - Program base relative (`ADD`,`SUB`).
- [`R_ARM_AMP_VCALL9`](#r_arm_amp_vcall9)
- [`R_ARM_BASE_ABS`](#r_arm_base_abs) - Adjust by program base.
- [`R_ARM_CALL`](#r_arm_call) - PC relative 24 bit (`BL`, `BLX`).
- [`R_ARM_COPY`](#r_arm_copy) - Copy symbol at runtime
- [`R_ARM_GLOB_DAT`](#r_arm_glob_dat) - Create GOT entry
- [`R_ARM_GNU_VTENTRY`](#r_arm_gnu_vtentry)
- [`R_ARM_GNU_VTINHERIT`](#r_arm_gnu_vtinherit)
- [`R_ARM_GOT32`](#r_arm_got32) - 32 bit GOT entry
- [`R_ARM_GOTOFF`](#r_arm_gotoff) - 32 bit offset to GOT
- [`R_ARM_GOTOFF12`](#r_arm_gotoff12) - 12 bit, GOT entry relative to GOT origin (`LDR`, `STR`).
- [`R_ARM_GOTPC`](#r_arm_gotpc) - 32 bit PC relative offset to GOT
- [`R_ARM_GOTRELAX`](#r_arm_gotrelax)
- [`R_ARM_GOT_ABS`](#r_arm_got_abs) - GOT entry.
- [`R_ARM_GOT_BREL12`](#r_arm_got_brel12) - GOT entry relative to GOT origin (`LDR`).
- [`R_ARM_GOT_PREL`](#r_arm_got_prel) - PC relative GOT entry.
- [`R_ARM_IRELATIVE`](#r_arm_irelative)
- [`R_ARM_JUMP24`](#r_arm_jump24) - PC relative 24 bit (`B`, `BL<cond>`).
- [`R_ARM_JUMP_SLOT`](#r_arm_jump_slot) - Create PLT entry
- [`R_ARM_LDC_PC_G0`](#r_arm_ldc_pc_g0) - PC relative (`LDC`, `STC`).
- [`R_ARM_LDC_PC_G1`](#r_arm_ldc_pc_g1) - PC relative (`LDC`, `STC`).
- [`R_ARM_LDC_PC_G2`](#r_arm_ldc_pc_g2) - PC relative (`LDC`, `STC`).
- [`R_ARM_LDC_SB_G0`](#r_arm_ldc_sb_g0) - Program base relative (`LDC`,`STC`).
- [`R_ARM_LDC_SB_G1`](#r_arm_ldc_sb_g1) - Program base relative (`LDC`,`STC`).
- [`R_ARM_LDC_SB_G2`](#r_arm_ldc_sb_g2) - Program base relative (`LDC`,`STC`).
- [`R_ARM_LDRS_PC_G0`](#r_arm_ldrs_pc_g0) - PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).
- [`R_ARM_LDRS_PC_G1`](#r_arm_ldrs_pc_g1) - PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).
- [`R_ARM_LDRS_PC_G2`](#r_arm_ldrs_pc_g2) - PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).
- [`R_ARM_LDRS_SB_G0`](#r_arm_ldrs_sb_g0) - Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).
- [`R_ARM_LDRS_SB_G1`](#r_arm_ldrs_sb_g1) - Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).
- [`R_ARM_LDRS_SB_G2`](#r_arm_ldrs_sb_g2) - Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).
- [`R_ARM_LDR_PC_G1`](#r_arm_ldr_pc_g1) - PC relative (`LDR`,`STR`,`LDRB`,`STRB`).
- [`R_ARM_LDR_PC_G2`](#r_arm_ldr_pc_g2) - PC relative (`LDR`,`STR`,`LDRB`,`STRB`).
- [`R_ARM_LDR_SBREL_11_0`](#r_arm_ldr_sbrel_11_0) - Deprecated, prog. base relative.
- [`R_ARM_LDR_SB_G0`](#r_arm_ldr_sb_g0) - Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).
- [`R_ARM_LDR_SB_G1`](#r_arm_ldr_sb_g1) - Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).
- [`R_ARM_LDR_SB_G2`](#r_arm_ldr_sb_g2) - Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).
- [`R_ARM_ME_TOO`](#r_arm_me_too) - Obsolete.
- [`R_ARM_MOVT_ABS`](#r_arm_movt_abs) - Direct high 16-bit (`MOVT`).
- [`R_ARM_MOVT_BREL`](#r_arm_movt_brel) - Program base relative high 16 bit (`MOVT`).
- [`R_ARM_MOVT_PREL`](#r_arm_movt_prel) - PC relative (MOVT).
- [`R_ARM_MOVW_ABS_NC`](#r_arm_movw_abs_nc) - Direct 16-bit (`MOVW`).
- [`R_ARM_MOVW_BREL`](#r_arm_movw_brel) - Program base relative 16 bit (`MOVW`).
- [`R_ARM_MOVW_BREL_NC`](#r_arm_movw_brel_nc) - Program base relative 16 bit (`MOVW`).
- [`R_ARM_MOVW_PREL_NC`](#r_arm_movw_prel_nc) - PC relative 16-bit (`MOVW`).
- [`R_ARM_NONE`](#r_arm_none) - No reloc
- [`R_ARM_PC13`](#r_arm_pc13)
- [`R_ARM_PC24`](#r_arm_pc24) - Deprecated PC relative 26 bit branch.
- [`R_ARM_PLT32`](#r_arm_plt32) - Deprecated, 32 bit PLT address.
- [`R_ARM_PLT32_ABS`](#r_arm_plt32_abs)
- [`R_ARM_PREL31`](#r_arm_prel31) - 32 bit PC relative.
- [`R_ARM_RABS22`](#r_arm_rabs22)
- [`R_ARM_RBASE`](#r_arm_rbase)
- [`R_ARM_REL32`](#r_arm_rel32) - PC relative 32 bit
- [`R_ARM_REL32_NOI`](#r_arm_rel32_noi) - PC relative 32-bit.
- [`R_ARM_RELATIVE`](#r_arm_relative) - Adjust by program base
- [`R_ARM_RPC24`](#r_arm_rpc24)
- [`R_ARM_RREL32`](#r_arm_rrel32)
- [`R_ARM_RSBREL32`](#r_arm_rsbrel32)
- [`R_ARM_RXPC25`](#r_arm_rxpc25)
- [`R_ARM_SBREL31`](#r_arm_sbrel31) - Program base relative.
- [`R_ARM_SBREL32`](#r_arm_sbrel32)
- [`R_ARM_SWI24`](#r_arm_swi24) - Obsolete static relocation.
- [`R_ARM_TARGET1`](#r_arm_target1)
- [`R_ARM_TARGET2`](#r_arm_target2)
- [`R_ARM_THM_ABS5`](#r_arm_thm_abs5) - Direct & 0x7C (`LDR`, `STR`).
- [`R_ARM_THM_ALU_PREL_11_0`](#r_arm_thm_alu_prel_11_0) - PC relative 12 bit (Thumb32 `ADR.W`).
- [`R_ARM_THM_GOT_BREL12`](#r_arm_thm_got_brel12) - GOT entry relative to GOT origin, 12 bit (Thumb32 `LDR`).
- [`R_ARM_THM_JUMP19`](#r_arm_thm_jump19) - PC relative 20 bit (Thumb32 `B<cond>.W`).
- [`R_ARM_THM_JUMP24`](#r_arm_thm_jump24) - PC relative 24 bit (Thumb32 `B.W`).
- [`R_ARM_THM_JUMP6`](#r_arm_thm_jump6) - PC relative X & 0x7E (Thumb16 `CBZ`, `CBNZ`).
- [`R_ARM_THM_MOVT_ABS`](#r_arm_thm_movt_abs) - Direct high 16 bit (Thumb32 `MOVT`).
- [`R_ARM_THM_MOVT_BREL`](#r_arm_thm_movt_brel) - Program base relative high 16 bit (Thumb32 `MOVT`).
- [`R_ARM_THM_MOVT_PREL`](#r_arm_thm_movt_prel) - PC relative high 16 bit (Thumb32 `MOVT`).
- [`R_ARM_THM_MOVW_ABS_NC`](#r_arm_thm_movw_abs_nc) - Direct 16 bit (Thumb32 `MOVW`).
- [`R_ARM_THM_MOVW_BREL`](#r_arm_thm_movw_brel) - Program base relative 16 bit (Thumb32 `MOVW`).
- [`R_ARM_THM_MOVW_BREL_NC`](#r_arm_thm_movw_brel_nc) - Program base relative 16 bit (Thumb32 `MOVW`).
- [`R_ARM_THM_MOVW_PREL_NC`](#r_arm_thm_movw_prel_nc) - PC relative 16 bit (Thumb32 `MOVW`).
- [`R_ARM_THM_PC11`](#r_arm_thm_pc11) - PC relative & 0xFFE (Thumb16 `B`).
- [`R_ARM_THM_PC12`](#r_arm_thm_pc12) - PC relative 12 bit (Thumb32 `LDR{D,SB,H,SH}`).
- [`R_ARM_THM_PC22`](#r_arm_thm_pc22) - PC relative 24 bit (Thumb32 `BL`).
- [`R_ARM_THM_PC8`](#r_arm_thm_pc8) - PC relative & 0x3FC (Thumb16 `LDR`, `ADD`, `ADR`).
- [`R_ARM_THM_PC9`](#r_arm_thm_pc9) - PC relative & 0x1FE (Thumb16 `B`/`B<cond>`).
- [`R_ARM_THM_RPC22`](#r_arm_thm_rpc22)
- [`R_ARM_THM_SWI8`](#r_arm_thm_swi8) - Reserved.
- [`R_ARM_THM_TLS_CALL`](#r_arm_thm_tls_call)
- [`R_ARM_THM_TLS_DESCSEQ`](#r_arm_thm_tls_descseq)
- [`R_ARM_THM_TLS_DESCSEQ16`](#r_arm_thm_tls_descseq16)
- [`R_ARM_THM_TLS_DESCSEQ32`](#r_arm_thm_tls_descseq32)
- [`R_ARM_THM_XPC22`](#r_arm_thm_xpc22) - Reserved.
- [`R_ARM_TLS_CALL`](#r_arm_tls_call)
- [`R_ARM_TLS_DESC`](#r_arm_tls_desc) - Dynamic relocation.
- [`R_ARM_TLS_DESCSEQ`](#r_arm_tls_descseq) - TLS relaxation.
- [`R_ARM_TLS_DTPMOD32`](#r_arm_tls_dtpmod32) - ID of module containing symbol
- [`R_ARM_TLS_DTPOFF32`](#r_arm_tls_dtpoff32) - Offset in TLS block
- [`R_ARM_TLS_GD32`](#r_arm_tls_gd32) - PC-rel 32 bit for global dynamic thread local data
- [`R_ARM_TLS_GOTDESC`](#r_arm_tls_gotdesc)
- [`R_ARM_TLS_IE12GP`](#r_arm_tls_ie12gp) - 12 bit GOT entry relative to GOT origin (`LDR`).
- [`R_ARM_TLS_IE32`](#r_arm_tls_ie32) - PC-rel 32 bit for GOT entry of static TLS block offset
- [`R_ARM_TLS_LDM32`](#r_arm_tls_ldm32) - PC-rel 32 bit for local dynamic thread local data
- [`R_ARM_TLS_LDO12`](#r_arm_tls_ldo12) - 12 bit relative to TLS block (`LDR`, `STR`).
- [`R_ARM_TLS_LDO32`](#r_arm_tls_ldo32) - 32 bit offset relative to TLS block
- [`R_ARM_TLS_LE12`](#r_arm_tls_le12) - 12 bit relative to static TLS block (`LDR`, `STR`).
- [`R_ARM_TLS_LE32`](#r_arm_tls_le32) - 32 bit offset relative to static TLS block
- [`R_ARM_TLS_TPOFF32`](#r_arm_tls_tpoff32) - Offset in static TLS block
- [`R_ARM_V4BX`](#r_arm_v4bx)
- [`R_ARM_XPC25`](#r_arm_xpc25) - Reserved.
- [`R_AVR_13_PCREL`](#r_avr_13_pcrel)
- [`R_AVR_16`](#r_avr_16) - Direct 16 bit
- [`R_AVR_16_PM`](#r_avr_16_pm)
- [`R_AVR_32`](#r_avr_32) - Direct 32 bit
- [`R_AVR_32_PCREL`](#r_avr_32_pcrel)
- [`R_AVR_6`](#r_avr_6)
- [`R_AVR_6_ADIW`](#r_avr_6_adiw)
- [`R_AVR_7_PCREL`](#r_avr_7_pcrel)
- [`R_AVR_8`](#r_avr_8)
- [`R_AVR_8_HI8`](#r_avr_8_hi8)
- [`R_AVR_8_HLO8`](#r_avr_8_hlo8)
- [`R_AVR_8_LO8`](#r_avr_8_lo8)
- [`R_AVR_CALL`](#r_avr_call)
- [`R_AVR_DIFF16`](#r_avr_diff16)
- [`R_AVR_DIFF32`](#r_avr_diff32)
- [`R_AVR_DIFF8`](#r_avr_diff8)
- [`R_AVR_HH8_LDI`](#r_avr_hh8_ldi)
- [`R_AVR_HH8_LDI_NEG`](#r_avr_hh8_ldi_neg)
- [`R_AVR_HH8_LDI_PM`](#r_avr_hh8_ldi_pm)
- [`R_AVR_HH8_LDI_PM_NEG`](#r_avr_hh8_ldi_pm_neg)
- [`R_AVR_HI8_LDI`](#r_avr_hi8_ldi)
- [`R_AVR_HI8_LDI_GS`](#r_avr_hi8_ldi_gs)
- [`R_AVR_HI8_LDI_NEG`](#r_avr_hi8_ldi_neg)
- [`R_AVR_HI8_LDI_PM`](#r_avr_hi8_ldi_pm)
- [`R_AVR_HI8_LDI_PM_NEG`](#r_avr_hi8_ldi_pm_neg)
- [`R_AVR_LDI`](#r_avr_ldi)
- [`R_AVR_LDS_STS_16`](#r_avr_lds_sts_16)
- [`R_AVR_LO8_LDI`](#r_avr_lo8_ldi)
- [`R_AVR_LO8_LDI_GS`](#r_avr_lo8_ldi_gs)
- [`R_AVR_LO8_LDI_NEG`](#r_avr_lo8_ldi_neg)
- [`R_AVR_LO8_LDI_PM`](#r_avr_lo8_ldi_pm)
- [`R_AVR_LO8_LDI_PM_NEG`](#r_avr_lo8_ldi_pm_neg)
- [`R_AVR_MS8_LDI`](#r_avr_ms8_ldi)
- [`R_AVR_MS8_LDI_NEG`](#r_avr_ms8_ldi_neg)
- [`R_AVR_NONE`](#r_avr_none)
- [`R_AVR_PORT5`](#r_avr_port5)
- [`R_AVR_PORT6`](#r_avr_port6)
- [`R_BPF_64_32`](#r_bpf_64_32)
- [`R_BPF_64_64`](#r_bpf_64_64)
- [`R_BPF_NONE`](#r_bpf_none) - No reloc
- [`R_CKCORE_ADDR32`](#r_ckcore_addr32) - direct 32 bit (S + A)
- [`R_CKCORE_ADDRGOT`](#r_ckcore_addrgot) - GOT entry in GLOB_DAT (GOT + G)
- [`R_CKCORE_ADDRGOT_HI16`](#r_ckcore_addrgot_hi16) - high & low 16 bit ADDRGOT, (GOT + G * 4) & 0xffff
- [`R_CKCORE_ADDRGOT_LO16`](#r_ckcore_addrgot_lo16) - (GOT + G * 4) & 0xffff
- [`R_CKCORE_ADDRPLT`](#r_ckcore_addrplt) - PLT entry in GLOB_DAT (GOT + G)
- [`R_CKCORE_ADDRPLT_HI16`](#r_ckcore_addrplt_hi16) - high & low 16 bit ADDRPLT, ((GOT + G * 4) >> 16) & 0xFFFF
- [`R_CKCORE_ADDRPLT_LO16`](#r_ckcore_addrplt_lo16) - (GOT+G*4) & 0xffff
- [`R_CKCORE_ADDR_HI16`](#r_ckcore_addr_hi16) - high & low 16 bit ADDR, ((S + A) >> 16) & 0xffff
- [`R_CKCORE_ADDR_LO16`](#r_ckcore_addr_lo16) - (S + A) & 0xffff
- [`R_CKCORE_COPY`](#r_ckcore_copy) - 32 bit adjust by program base
- [`R_CKCORE_DOFFSET_IMM18`](#r_ckcore_doffset_imm18) - disp (S+A-BDATA) & 0x3ffff
- [`R_CKCORE_DOFFSET_IMM18BY2`](#r_ckcore_doffset_imm18by2) - disp ((S+A-BDATA)>>1) & 0x3ffff
- [`R_CKCORE_DOFFSET_IMM18BY4`](#r_ckcore_doffset_imm18by4) - disp ((S+A-BDATA)>>2) & 0x3ffff
- [`R_CKCORE_DOFFSET_LO16`](#r_ckcore_doffset_lo16) - (S+A-BTEXT) & 0xffff
- [`R_CKCORE_GLOB_DAT`](#r_ckcore_glob_dat) - off between got and sym (S)
- [`R_CKCORE_GOT12`](#r_ckcore_got12) - 12 bit disp GOT entry (G)
- [`R_CKCORE_GOT32`](#r_ckcore_got32) - 32 bit GOT entry (G)
- [`R_CKCORE_GOTOFF`](#r_ckcore_gotoff) - offset to GOT (S + A - GOT)
- [`R_CKCORE_GOTOFF_HI16`](#r_ckcore_gotoff_hi16) - high & low 16 bit GOTOFF, ((S + A - GOT) >> 16) & 0xffff
- [`R_CKCORE_GOTOFF_LO16`](#r_ckcore_gotoff_lo16) - (S + A - GOT) & 0xffff
- [`R_CKCORE_GOTPC`](#r_ckcore_gotpc) - PC offset to GOT (GOT + A - P)
- [`R_CKCORE_GOTPC_HI16`](#r_ckcore_gotpc_hi16) - high & low 16 bit GOTPC, ((GOT + A - P) >> 16) & 0xffff
- [`R_CKCORE_GOTPC_LO16`](#r_ckcore_gotpc_lo16) - (GOT + A - P) & 0xffff
- [`R_CKCORE_GOT_HI16`](#r_ckcore_got_hi16) - high & low 16 bit GOT, (G >> 16) & 0xffff
- [`R_CKCORE_GOT_IMM18BY4`](#r_ckcore_got_imm18by4) - disp (G >> 2)
- [`R_CKCORE_GOT_LO16`](#r_ckcore_got_lo16) - (G & 0xffff)
- [`R_CKCORE_JUMP_SLOT`](#r_ckcore_jump_slot) - PLT entry (S)
- [`R_CKCORE_NONE`](#r_ckcore_none) - no reloc
- [`R_CKCORE_PCREL32`](#r_ckcore_pcrel32) - 32-bit rel (S + A - P)
- [`R_CKCORE_PCRELIMM11BY2`](#r_ckcore_pcrelimm11by2) - disp ((S + A - P) >> 1) & 0x7ff
- [`R_CKCORE_PCRELIMM8BY4`](#r_ckcore_pcrelimm8by4) - disp ((S + A - P) >> 2) & 0xff
- [`R_CKCORE_PCRELJSR_IMM11BY2`](#r_ckcore_pcreljsr_imm11by2) - disp ((S + A - P) >>1) & 0x7ff
- [`R_CKCORE_PCREL_IMM10BY2`](#r_ckcore_pcrel_imm10by2) - disp ((S + A - P) >> 1) & 0x3ff
- [`R_CKCORE_PCREL_IMM10BY4`](#r_ckcore_pcrel_imm10by4) - disp ((S + A - P) >> 2) & 0x3ff
- [`R_CKCORE_PCREL_IMM16BY2`](#r_ckcore_pcrel_imm16by2) - disp ((S + A - P) >> 1) & 0xffff
- [`R_CKCORE_PCREL_IMM16BY4`](#r_ckcore_pcrel_imm16by4) - disp ((S + A - P) >> 2) & 0xffff
- [`R_CKCORE_PCREL_IMM18BY2`](#r_ckcore_pcrel_imm18by2) - disp ((S+A-P) >>1) & 0x3ffff
- [`R_CKCORE_PCREL_IMM26BY2`](#r_ckcore_pcrel_imm26by2) - ((S + A - P) >> 1) & 0x3ff_ffff
- [`R_CKCORE_PCREL_IMM7BY4`](#r_ckcore_pcrel_imm7by4) - disp ((S+A-P) >>2) & 0x7f
- [`R_CKCORE_PCREL_JSR_IMM26BY2`](#r_ckcore_pcrel_jsr_imm26by2) - disp ((S+A-P) >>1) & x3ff_ffff
- [`R_CKCORE_PLT12`](#r_ckcore_plt12) - 12 bit disp PLT entry (G)
- [`R_CKCORE_PLT32`](#r_ckcore_plt32) - 32 bit PLT entry (G)
- [`R_CKCORE_PLT_HI16`](#r_ckcore_plt_hi16) - high & low 16 bit PLT, (G >> 16) & 0xffff
- [`R_CKCORE_PLT_IMM18BY4`](#r_ckcore_plt_imm18by4) - disp (G >> 2)
- [`R_CKCORE_PLT_LO16`](#r_ckcore_plt_lo16) - G & 0xffff
- [`R_CKCORE_RELATIVE`](#r_ckcore_relative) - 32 bit adjust program base(B + A)
- [`R_CKCORE_TLS_DTPMOD32`](#r_ckcore_tls_dtpmod32)
- [`R_CKCORE_TLS_DTPOFF32`](#r_ckcore_tls_dtpoff32)
- [`R_CKCORE_TLS_GD32`](#r_ckcore_tls_gd32)
- [`R_CKCORE_TLS_IE32`](#r_ckcore_tls_ie32)
- [`R_CKCORE_TLS_LDM32`](#r_ckcore_tls_ldm32)
- [`R_CKCORE_TLS_LDO32`](#r_ckcore_tls_ldo32)
- [`R_CKCORE_TLS_LE32`](#r_ckcore_tls_le32) - 32 bit offset to TLS block
- [`R_CKCORE_TLS_TPOFF32`](#r_ckcore_tls_tpoff32)
- [`R_CKCORE_TOFFSET_LO16`](#r_ckcore_toffset_lo16) - (S+A-BTEXT) & 0xffff
- [`R_CRIS_16`](#r_cris_16)
- [`R_CRIS_16_GOT`](#r_cris_16_got)
- [`R_CRIS_16_GOTPLT`](#r_cris_16_gotplt)
- [`R_CRIS_16_PCREL`](#r_cris_16_pcrel)
- [`R_CRIS_32`](#r_cris_32)
- [`R_CRIS_32_GOT`](#r_cris_32_got)
- [`R_CRIS_32_GOTPLT`](#r_cris_32_gotplt)
- [`R_CRIS_32_GOTREL`](#r_cris_32_gotrel)
- [`R_CRIS_32_PCREL`](#r_cris_32_pcrel)
- [`R_CRIS_32_PLT_GOTREL`](#r_cris_32_plt_gotrel)
- [`R_CRIS_32_PLT_PCREL`](#r_cris_32_plt_pcrel)
- [`R_CRIS_8`](#r_cris_8)
- [`R_CRIS_8_PCREL`](#r_cris_8_pcrel)
- [`R_CRIS_COPY`](#r_cris_copy)
- [`R_CRIS_GLOB_DAT`](#r_cris_glob_dat)
- [`R_CRIS_GNU_VTENTRY`](#r_cris_gnu_vtentry)
- [`R_CRIS_GNU_VTINHERIT`](#r_cris_gnu_vtinherit)
- [`R_CRIS_JUMP_SLOT`](#r_cris_jump_slot)
- [`R_CRIS_NONE`](#r_cris_none)
- [`R_CRIS_RELATIVE`](#r_cris_relative)
- [`R_E2K_32_ABS`](#r_e2k_32_abs) - Direct 32 bit.
- [`R_E2K_32_COPY`](#r_e2k_32_copy) - Copy relocation, 32-bit case.
- [`R_E2K_32_DYNOPT`](#r_e2k_32_dynopt) - Symbol value if resolved by the definition in the same
- [`R_E2K_32_IRELATIVE`](#r_e2k_32_irelative) - Adjust indirectly by program base, 32-bit case.
- [`R_E2K_32_JMP_SLOT`](#r_e2k_32_jmp_slot) - Create PLT entry.
- [`R_E2K_32_PC`](#r_e2k_32_pc) - PC relative 32 bit.
- [`R_E2K_32_RELATIVE`](#r_e2k_32_relative) - Adjust by program base, 32-bit case.
- [`R_E2K_32_SIZE`](#r_e2k_32_size) - Size of symbol plus 32-bit addend.
- [`R_E2K_32_TLS_LE`](#r_e2k_32_tls_le) - Offset relative to static TLS block, 32-bit case.
- [`R_E2K_64_ABS`](#r_e2k_64_abs) - Direct 64 bit.
- [`R_E2K_64_ABS_LIT`](#r_e2k_64_abs_lit) - Direct 64 bit for literal.
- [`R_E2K_64_COPY`](#r_e2k_64_copy) - Copy relocation, 64-bit case.
- [`R_E2K_64_DYNOPT`](#r_e2k_64_dynopt) - Symbol value if resolved by the definition in the same
- [`R_E2K_64_GOTOFF`](#r_e2k_64_gotoff) - 64-bit offset of the symbol from GOT.
- [`R_E2K_64_GOTOFF_LIT`](#r_e2k_64_gotoff_lit) - The symbol's offset from GOT encoded within a 64-bit literal.
- [`R_E2K_64_IRELATIVE`](#r_e2k_64_irelative) - Adjust indirectly by program base, 64-bit case.
- [`R_E2K_64_JMP_SLOT`](#r_e2k_64_jmp_slot) - Create PLT entry, 64-bit case.
- [`R_E2K_64_PC`](#r_e2k_64_pc) - PC relative 64 bit in data.
- [`R_E2K_64_PC_LIT`](#r_e2k_64_pc_lit) - PC relative 64 bit for literal.
- [`R_E2K_64_RELATIVE`](#r_e2k_64_relative) - Adjust by program base, 64-bit case.
- [`R_E2K_64_RELATIVE_LIT`](#r_e2k_64_relative_lit) - Adjust by program base for literal, 64-bit case.
- [`R_E2K_64_SIZE`](#r_e2k_64_size) - Size of symbol plus 64-bit addend.
- [`R_E2K_64_TLS_LE`](#r_e2k_64_tls_le) - Offset relative to static TLS block, 64-bit case.
- [`R_E2K_AP`](#r_e2k_ap) - Direct AP.
- [`R_E2K_AP_GOT`](#r_e2k_ap_got) - 32-bit offset of AP GOT entry.
- [`R_E2K_DISP`](#r_e2k_disp) - PC relative 28 bit for DISP.
- [`R_E2K_GOT`](#r_e2k_got) - 32-bit offset of the symbol's entry in GOT.
- [`R_E2K_GOTOFF`](#r_e2k_gotoff) - 32-bit offset of the symbol from GOT.
- [`R_E2K_GOTPLT`](#r_e2k_gotplt) - 32-bit offset of the symbol's entry in .got.plt.
- [`R_E2K_ISLOCAL`](#r_e2k_islocal) - Is symbol resolved locally during the link.
- [`R_E2K_ISLOCAL32`](#r_e2k_islocal32) - Is symbol resloved locally during the link.
- [`R_E2K_NONE`](#r_e2k_none) - No reloc.
- [`R_E2K_PL`](#r_e2k_pl) - Direct PL.
- [`R_E2K_PL_GOT`](#r_e2k_pl_got) - 32-bit offset of PL GOT entry.
- [`R_E2K_PREF`](#r_e2k_pref) - Prefetch insn line containing the label (symbol).
- [`R_E2K_TLS_32_DTPMOD`](#r_e2k_tls_32_dtpmod) - ID of module containing symbol, 32-bit case.
- [`R_E2K_TLS_32_DTPREL`](#r_e2k_tls_32_dtprel) - Offset in module TLS block, 32-bit case.
- [`R_E2K_TLS_32_TPREL`](#r_e2k_tls_32_tprel) - Offset in static TLS block, 32-bit case.
- [`R_E2K_TLS_64_DTPMOD`](#r_e2k_tls_64_dtpmod) - ID of module containing symbol, 64-bit case.
- [`R_E2K_TLS_64_DTPREL`](#r_e2k_tls_64_dtprel) - Offset in module TLS block, 64-bit case.
- [`R_E2K_TLS_64_TPREL`](#r_e2k_tls_64_tprel) - Offset in static TLS block, 64-bit case.
- [`R_E2K_TLS_GDMOD`](#r_e2k_tls_gdmod) - GOT entry for ID of module containing symbol.
- [`R_E2K_TLS_GDREL`](#r_e2k_tls_gdrel) - GOT entry for offset in module TLS block.
- [`R_E2K_TLS_IE`](#r_e2k_tls_ie) - Static TLS block offset GOT entry.
- [`R_HEX_32`](#r_hex_32) - Direct 32 bit
- [`R_HEX_NONE`](#r_hex_none) - No reloc
- [`R_IA64_COPY`](#r_ia64_copy) - copy relocation
- [`R_IA64_DIR32LSB`](#r_ia64_dir32lsb) - symbol + addend, data4 LSB
- [`R_IA64_DIR32MSB`](#r_ia64_dir32msb) - symbol + addend, data4 MSB
- [`R_IA64_DIR64LSB`](#r_ia64_dir64lsb) - symbol + addend, data8 LSB
- [`R_IA64_DIR64MSB`](#r_ia64_dir64msb) - symbol + addend, data8 MSB
- [`R_IA64_DTPMOD64LSB`](#r_ia64_dtpmod64lsb) - @dtpmod(sym + add), data8 LSB
- [`R_IA64_DTPMOD64MSB`](#r_ia64_dtpmod64msb) - @dtpmod(sym + add), data8 MSB
- [`R_IA64_DTPREL14`](#r_ia64_dtprel14) - @dtprel(sym + add), imm14
- [`R_IA64_DTPREL22`](#r_ia64_dtprel22) - @dtprel(sym + add), imm22
- [`R_IA64_DTPREL32LSB`](#r_ia64_dtprel32lsb) - @dtprel(sym + add), data4 LSB
- [`R_IA64_DTPREL32MSB`](#r_ia64_dtprel32msb) - @dtprel(sym + add), data4 MSB
- [`R_IA64_DTPREL64I`](#r_ia64_dtprel64i) - @dtprel(sym + add), imm64
- [`R_IA64_DTPREL64LSB`](#r_ia64_dtprel64lsb) - @dtprel(sym + add), data8 LSB
- [`R_IA64_DTPREL64MSB`](#r_ia64_dtprel64msb) - @dtprel(sym + add), data8 MSB
- [`R_IA64_FPTR32LSB`](#r_ia64_fptr32lsb) - @fptr(sym + add), data4 LSB
- [`R_IA64_FPTR32MSB`](#r_ia64_fptr32msb) - @fptr(sym + add), data4 MSB
- [`R_IA64_FPTR64I`](#r_ia64_fptr64i) - @fptr(sym + add), mov imm64
- [`R_IA64_FPTR64LSB`](#r_ia64_fptr64lsb) - @fptr(sym + add), data8 LSB
- [`R_IA64_FPTR64MSB`](#r_ia64_fptr64msb) - @fptr(sym + add), data8 MSB
- [`R_IA64_GPREL22`](#r_ia64_gprel22) - @gprel(sym + add), add imm22
- [`R_IA64_GPREL32LSB`](#r_ia64_gprel32lsb) - @gprel(sym + add), data4 LSB
- [`R_IA64_GPREL32MSB`](#r_ia64_gprel32msb) - @gprel(sym + add), data4 MSB
- [`R_IA64_GPREL64I`](#r_ia64_gprel64i) - @gprel(sym + add), mov imm64
- [`R_IA64_GPREL64LSB`](#r_ia64_gprel64lsb) - @gprel(sym + add), data8 LSB
- [`R_IA64_GPREL64MSB`](#r_ia64_gprel64msb) - @gprel(sym + add), data8 MSB
- [`R_IA64_IMM14`](#r_ia64_imm14) - symbol + addend, add imm14
- [`R_IA64_IMM22`](#r_ia64_imm22) - symbol + addend, add imm22
- [`R_IA64_IMM64`](#r_ia64_imm64) - symbol + addend, mov imm64
- [`R_IA64_IPLTLSB`](#r_ia64_ipltlsb) - dynamic reloc, imported PLT, LSB
- [`R_IA64_IPLTMSB`](#r_ia64_ipltmsb) - dynamic reloc, imported PLT, MSB
- [`R_IA64_LDXMOV`](#r_ia64_ldxmov) - Use of LTOFF22X.
- [`R_IA64_LTOFF22`](#r_ia64_ltoff22) - @ltoff(sym + add), add imm22
- [`R_IA64_LTOFF22X`](#r_ia64_ltoff22x) - LTOFF22, relaxable.
- [`R_IA64_LTOFF64I`](#r_ia64_ltoff64i) - @ltoff(sym + add), mov imm64
- [`R_IA64_LTOFF_DTPMOD22`](#r_ia64_ltoff_dtpmod22) - @ltoff(@dtpmod(sym + add)), imm22
- [`R_IA64_LTOFF_DTPREL22`](#r_ia64_ltoff_dtprel22) - @ltoff(@dtprel(s+a)), imm22
- [`R_IA64_LTOFF_FPTR22`](#r_ia64_ltoff_fptr22) - @ltoff(@fptr(s+a)), imm22
- [`R_IA64_LTOFF_FPTR32LSB`](#r_ia64_ltoff_fptr32lsb) - @ltoff(@fptr(s+a)), data4 LSB
- [`R_IA64_LTOFF_FPTR32MSB`](#r_ia64_ltoff_fptr32msb) - @ltoff(@fptr(s+a)), data4 MSB
- [`R_IA64_LTOFF_FPTR64I`](#r_ia64_ltoff_fptr64i) - @ltoff(@fptr(s+a)), imm64
- [`R_IA64_LTOFF_FPTR64LSB`](#r_ia64_ltoff_fptr64lsb) - @ltoff(@fptr(s+a)), data8 LSB
- [`R_IA64_LTOFF_FPTR64MSB`](#r_ia64_ltoff_fptr64msb) - @ltoff(@fptr(s+a)), data8 MSB
- [`R_IA64_LTOFF_TPREL22`](#r_ia64_ltoff_tprel22) - @ltoff(@tprel(s+a)), imm2
- [`R_IA64_LTV32LSB`](#r_ia64_ltv32lsb) - symbol + addend, data4 LSB
- [`R_IA64_LTV32MSB`](#r_ia64_ltv32msb) - symbol + addend, data4 MSB
- [`R_IA64_LTV64LSB`](#r_ia64_ltv64lsb) - symbol + addend, data8 LSB
- [`R_IA64_LTV64MSB`](#r_ia64_ltv64msb) - symbol + addend, data8 MSB
- [`R_IA64_NONE`](#r_ia64_none) - none
- [`R_IA64_PCREL21B`](#r_ia64_pcrel21b) - @pcrel(sym + add), ptb, call
- [`R_IA64_PCREL21BI`](#r_ia64_pcrel21bi) - @pcrel(sym + add), 21bit inst
- [`R_IA64_PCREL21F`](#r_ia64_pcrel21f) - @pcrel(sym + add), fchkf
- [`R_IA64_PCREL21M`](#r_ia64_pcrel21m) - @pcrel(sym + add), chk.s
- [`R_IA64_PCREL22`](#r_ia64_pcrel22) - @pcrel(sym + add), 22bit inst
- [`R_IA64_PCREL32LSB`](#r_ia64_pcrel32lsb) - @pcrel(sym + add), data4 LSB
- [`R_IA64_PCREL32MSB`](#r_ia64_pcrel32msb) - @pcrel(sym + add), data4 MSB
- [`R_IA64_PCREL60B`](#r_ia64_pcrel60b) - @pcrel(sym + add), brl
- [`R_IA64_PCREL64I`](#r_ia64_pcrel64i) - @pcrel(sym + add), 64bit inst
- [`R_IA64_PCREL64LSB`](#r_ia64_pcrel64lsb) - @pcrel(sym + add), data8 LSB
- [`R_IA64_PCREL64MSB`](#r_ia64_pcrel64msb) - @pcrel(sym + add), data8 MSB
- [`R_IA64_PLTOFF22`](#r_ia64_pltoff22) - @pltoff(sym + add), add imm22
- [`R_IA64_PLTOFF64I`](#r_ia64_pltoff64i) - @pltoff(sym + add), mov imm64
- [`R_IA64_PLTOFF64LSB`](#r_ia64_pltoff64lsb) - @pltoff(sym + add), data8 LSB
- [`R_IA64_PLTOFF64MSB`](#r_ia64_pltoff64msb) - @pltoff(sym + add), data8 MSB
- [`R_IA64_REL32LSB`](#r_ia64_rel32lsb) - data 4 + REL
- [`R_IA64_REL32MSB`](#r_ia64_rel32msb) - data 4 + REL
- [`R_IA64_REL64LSB`](#r_ia64_rel64lsb) - data 8 + REL
- [`R_IA64_REL64MSB`](#r_ia64_rel64msb) - data 8 + REL
- [`R_IA64_SECREL32LSB`](#r_ia64_secrel32lsb) - @secrel(sym + add), data4 LSB
- [`R_IA64_SECREL32MSB`](#r_ia64_secrel32msb) - @secrel(sym + add), data4 MSB
- [`R_IA64_SECREL64LSB`](#r_ia64_secrel64lsb) - @secrel(sym + add), data8 LSB
- [`R_IA64_SECREL64MSB`](#r_ia64_secrel64msb) - @secrel(sym + add), data8 MSB
- [`R_IA64_SEGREL32LSB`](#r_ia64_segrel32lsb) - @segrel(sym + add), data4 LSB
- [`R_IA64_SEGREL32MSB`](#r_ia64_segrel32msb) - @segrel(sym + add), data4 MSB
- [`R_IA64_SEGREL64LSB`](#r_ia64_segrel64lsb) - @segrel(sym + add), data8 LSB
- [`R_IA64_SEGREL64MSB`](#r_ia64_segrel64msb) - @segrel(sym + add), data8 MSB
- [`R_IA64_SUB`](#r_ia64_sub) - Addend and symbol difference
- [`R_IA64_TPREL14`](#r_ia64_tprel14) - @tprel(sym + add), imm14
- [`R_IA64_TPREL22`](#r_ia64_tprel22) - @tprel(sym + add), imm22
- [`R_IA64_TPREL64I`](#r_ia64_tprel64i) - @tprel(sym + add), imm64
- [`R_IA64_TPREL64LSB`](#r_ia64_tprel64lsb) - @tprel(sym + add), data8 LSB
- [`R_IA64_TPREL64MSB`](#r_ia64_tprel64msb) - @tprel(sym + add), data8 MSB
- [`R_LARCH_32`](#r_larch_32) - Runtime address resolving
- [`R_LARCH_32_PCREL`](#r_larch_32_pcrel) - 32-bit PC relative
- [`R_LARCH_64`](#r_larch_64) - Runtime address resolving
- [`R_LARCH_64_PCREL`](#r_larch_64_pcrel) - 64-bit PC relative
- [`R_LARCH_ABS64_HI12`](#r_larch_abs64_hi12) - 52..=63 bits of 64-bit absolute address
- [`R_LARCH_ABS64_LO20`](#r_larch_abs64_lo20) - 32..=51 bits of 64-bit absolute address
- [`R_LARCH_ABS_HI20`](#r_larch_abs_hi20) - 12..=31 bits of 32/64-bit absolute address
- [`R_LARCH_ABS_LO12`](#r_larch_abs_lo12) - 0..=11 bits of 32/64-bit absolute address
- [`R_LARCH_ADD16`](#r_larch_add16) - 16-bit in-place addition
- [`R_LARCH_ADD24`](#r_larch_add24) - 24-bit in-place addition
- [`R_LARCH_ADD32`](#r_larch_add32) - 32-bit in-place addition
- [`R_LARCH_ADD6`](#r_larch_add6) - 6-bit in-place addition
- [`R_LARCH_ADD64`](#r_larch_add64) - 64-bit in-place addition
- [`R_LARCH_ADD8`](#r_larch_add8) - 8-bit in-place addition
- [`R_LARCH_ADD_ULEB128`](#r_larch_add_uleb128) - LEB128 in-place addition
- [`R_LARCH_ALIGN`](#r_larch_align) - Delete some bytes to ensure the instruction at PC + A aligned to
- [`R_LARCH_B16`](#r_larch_b16) - 18-bit PC-relative jump offset with two trailing zeros
- [`R_LARCH_B21`](#r_larch_b21) - 23-bit PC-relative jump offset with two trailing zeros
- [`R_LARCH_B26`](#r_larch_b26) - 28-bit PC-relative jump offset with two trailing zeros
- [`R_LARCH_CALL30`](#r_larch_call30) - 12..=31 bits of `S + A - PC` into the `pcaddu12i` instruction at `PC`,
- [`R_LARCH_CALL36`](#r_larch_call36) - 18..=37 bits of `S + A - PC` into the `pcaddu18i` instruction at `PC`,
- [`R_LARCH_CFA`](#r_larch_cfa) - Reserved
- [`R_LARCH_COPY`](#r_larch_copy) - Runtime memory copy in executable
- [`R_LARCH_DELETE`](#r_larch_delete) - Reserved
- [`R_LARCH_GNU_VTENTRY`](#r_larch_gnu_vtentry) - GNU C++ vtable member usage
- [`R_LARCH_GNU_VTINHERIT`](#r_larch_gnu_vtinherit) - GNU C++ vtable hierarchy
- [`R_LARCH_GOT64_HI12`](#r_larch_got64_hi12) - 52..=63 bits of 64-bit GOT entry absolute address
- [`R_LARCH_GOT64_LO20`](#r_larch_got64_lo20) - 32..=51 bits of 64-bit GOT entry absolute address
- [`R_LARCH_GOT64_PC_HI12`](#r_larch_got64_pc_hi12) - 52..=63 bits of the 64-bit offset from the
- [`R_LARCH_GOT64_PC_LO20`](#r_larch_got64_pc_lo20) - 32..=51 bits of the 64-bit offset from the
- [`R_LARCH_GOT_HI20`](#r_larch_got_hi20) - 12..=31 bits of 32/64-bit GOT entry absolute address
- [`R_LARCH_GOT_LO12`](#r_larch_got_lo12) - 0..=11 bits of 32/64-bit GOT entry absolute address
- [`R_LARCH_GOT_PCADD_HI20`](#r_larch_got_pcadd_hi20) - The signed 32-bit offset `offs` from `PC` to
- [`R_LARCH_GOT_PCADD_LO12`](#r_larch_got_pcadd_lo12) - 0..=11 bits of the 32-bit offset from the
- [`R_LARCH_GOT_PC_HI20`](#r_larch_got_pc_hi20) - The signed 32-bit offset `offs` from `PC & 0xfffff000` to
- [`R_LARCH_GOT_PC_LO12`](#r_larch_got_pc_lo12) - 0..=11 bits of the 32/64-bit offset from the
- [`R_LARCH_IRELATIVE`](#r_larch_irelative) - Runtime local indirect function resolving
- [`R_LARCH_JUMP_SLOT`](#r_larch_jump_slot) - Runtime PLT supporting
- [`R_LARCH_MARK_LA`](#r_larch_mark_la) - Mark la.abs: load absolute address for static link.
- [`R_LARCH_MARK_PCREL`](#r_larch_mark_pcrel) - Mark external label branch: access PC relative address for static link.
- [`R_LARCH_NONE`](#r_larch_none) - No reloc
- [`R_LARCH_PCADD_HI20`](#r_larch_pcadd_hi20) - The signed 32-bit offset `offs` from `PC` to `(S + A + 0x800) & 0xfffff000`.
- [`R_LARCH_PCADD_LO12`](#r_larch_pcadd_lo12) - 0..=11 bits of the 32-bit offset from the
- [`R_LARCH_PCALA64_HI12`](#r_larch_pcala64_hi12) - 52..=63 bits of the 64-bit offset from the
- [`R_LARCH_PCALA64_LO20`](#r_larch_pcala64_lo20) - 32..=51 bits of the 64-bit offset from the
- [`R_LARCH_PCALA_HI20`](#r_larch_pcala_hi20) - The signed 32-bit offset `offs` from `PC & 0xfffff000` to
- [`R_LARCH_PCALA_LO12`](#r_larch_pcala_lo12) - Same as R_LARCH_ABS_LO12.  0..=11 bits of the 32/64-bit offset from the
- [`R_LARCH_PCREL20_S2`](#r_larch_pcrel20_s2) - 22-bit PC-relative offset with two trailing zeros
- [`R_LARCH_RELATIVE`](#r_larch_relative) - Runtime fixup for load-address
- [`R_LARCH_RELAX`](#r_larch_relax) - Paired with a normal relocation at the same address to indicate the
- [`R_LARCH_SOP_ADD`](#r_larch_sop_add) - Stack top addition (binary)
- [`R_LARCH_SOP_AND`](#r_larch_sop_and) - Stack top bitwise and (binary)
- [`R_LARCH_SOP_ASSERT`](#r_larch_sop_assert) - Assert stack top
- [`R_LARCH_SOP_IF_ELSE`](#r_larch_sop_if_else) - Stack top selection (tertiary)
- [`R_LARCH_SOP_NOT`](#r_larch_sop_not) - Stack top logical not (unary)
- [`R_LARCH_SOP_POP_32_S_0_10_10_16_S2`](#r_larch_sop_pop_32_s_0_10_10_16_s2) - Pop stack top to fill 28-bit signed immediate operand with two trailing
- [`R_LARCH_SOP_POP_32_S_0_5_10_16_S2`](#r_larch_sop_pop_32_s_0_5_10_16_s2) - Pop stack top to fill 23-bit signed immediate operand with two trailing
- [`R_LARCH_SOP_POP_32_S_10_12`](#r_larch_sop_pop_32_s_10_12) - Pop stack top to fill 12-bit signed immediate operand
- [`R_LARCH_SOP_POP_32_S_10_16`](#r_larch_sop_pop_32_s_10_16) - Pop stack top to fill 16-bit signed immediate operand
- [`R_LARCH_SOP_POP_32_S_10_16_S2`](#r_larch_sop_pop_32_s_10_16_s2) - Pop stack top to fill 18-bit signed immediate operand with two trailing
- [`R_LARCH_SOP_POP_32_S_10_5`](#r_larch_sop_pop_32_s_10_5) - Pop stack top to fill 5-bit signed immediate operand
- [`R_LARCH_SOP_POP_32_S_5_20`](#r_larch_sop_pop_32_s_5_20) - Pop stack top to fill 20-bit signed immediate operand
- [`R_LARCH_SOP_POP_32_U`](#r_larch_sop_pop_32_u) - Pop stack top to fill an instruction
- [`R_LARCH_SOP_POP_32_U_10_12`](#r_larch_sop_pop_32_u_10_12) - Pop stack top to fill 12-bit unsigned immediate operand
- [`R_LARCH_SOP_PUSH_ABSOLUTE`](#r_larch_sop_push_absolute) - Push constant or absolute address
- [`R_LARCH_SOP_PUSH_DUP`](#r_larch_sop_push_dup) - Duplicate stack top
- [`R_LARCH_SOP_PUSH_GPREL`](#r_larch_sop_push_gprel) - Push for access GOT entry
- [`R_LARCH_SOP_PUSH_PCREL`](#r_larch_sop_push_pcrel) - Push PC-relative offset
- [`R_LARCH_SOP_PUSH_PLT_PCREL`](#r_larch_sop_push_plt_pcrel) - Push for external function calling
- [`R_LARCH_SOP_PUSH_TLS_GD`](#r_larch_sop_push_tls_gd) - Push for TLS-GD
- [`R_LARCH_SOP_PUSH_TLS_GOT`](#r_larch_sop_push_tls_got) - Push for TLS-IE
- [`R_LARCH_SOP_PUSH_TLS_TPREL`](#r_larch_sop_push_tls_tprel) - Push for TLS-LE
- [`R_LARCH_SOP_SL`](#r_larch_sop_sl) - Stack top left shift (binary)
- [`R_LARCH_SOP_SR`](#r_larch_sop_sr) - Stack top right shift (binary)
- [`R_LARCH_SOP_SUB`](#r_larch_sop_sub) - Stack top subtraction (binary)
- [`R_LARCH_SUB16`](#r_larch_sub16) - 16-bit in-place subtraction
- [`R_LARCH_SUB24`](#r_larch_sub24) - 24-bit in-place subtraction
- [`R_LARCH_SUB32`](#r_larch_sub32) - 32-bit in-place subtraction
- [`R_LARCH_SUB6`](#r_larch_sub6) - 6-bit in-place subtraction
- [`R_LARCH_SUB64`](#r_larch_sub64) - 64-bit in-place subtraction
- [`R_LARCH_SUB8`](#r_larch_sub8) - 8-bit in-place subtraction
- [`R_LARCH_SUB_ULEB128`](#r_larch_sub_uleb128) - LEB128 in-place subtraction
- [`R_LARCH_TLS_DESC32`](#r_larch_tls_desc32) - Runtime relocation for TLS descriptors
- [`R_LARCH_TLS_DESC64`](#r_larch_tls_desc64) - Runtime relocation for TLS descriptors
- [`R_LARCH_TLS_DESC64_HI12`](#r_larch_tls_desc64_hi12) - 52..=63 bits of 64-bit TLS DESC GOT entry absolute address
- [`R_LARCH_TLS_DESC64_LO20`](#r_larch_tls_desc64_lo20) - 32..=51 bits of 64-bit TLS DESC GOT entry absolute address
- [`R_LARCH_TLS_DESC64_PC_HI12`](#r_larch_tls_desc64_pc_hi12) - 52..=63 bits of 64-bit PC-relative offset to TLS DESC GOT entry
- [`R_LARCH_TLS_DESC64_PC_LO20`](#r_larch_tls_desc64_pc_lo20) - 32..=51 bits of 64-bit PC-relative offset to TLS DESC GOT entry
- [`R_LARCH_TLS_DESC_CALL`](#r_larch_tls_desc_call) - Used on jirl for TLS DESC to call the resolve function
- [`R_LARCH_TLS_DESC_HI20`](#r_larch_tls_desc_hi20) - 12..=31 bits of 32/64-bit TLS DESC GOT entry absolute address
- [`R_LARCH_TLS_DESC_LD`](#r_larch_tls_desc_ld) - Used on ld.{w,d} for TLS DESC to get the resolve function address
- [`R_LARCH_TLS_DESC_LO12`](#r_larch_tls_desc_lo12) - 0..=11 bits of 32/64-bit TLS DESC GOT entry absolute address
- [`R_LARCH_TLS_DESC_PCADD_HI20`](#r_larch_tls_desc_pcadd_hi20) - The signed 32-bit offset `offs` from `PC` to
- [`R_LARCH_TLS_DESC_PCADD_LO12`](#r_larch_tls_desc_pcadd_lo12) - 0..=11 bits of the 32-bit offset from the
- [`R_LARCH_TLS_DESC_PCREL20_S2`](#r_larch_tls_desc_pcrel20_s2) - 22-bit PC-relative offset to TLS DESC GOT entry
- [`R_LARCH_TLS_DESC_PC_HI20`](#r_larch_tls_desc_pc_hi20) - 12..=31 bits of 32/64-bit PC-relative offset to TLS DESC GOT entry
- [`R_LARCH_TLS_DESC_PC_LO12`](#r_larch_tls_desc_pc_lo12) - 0..=11 bits of 32/64-bit TLS DESC GOT entry address
- [`R_LARCH_TLS_DTPMOD32`](#r_larch_tls_dtpmod32) - Runtime relocation for TLS-GD
- [`R_LARCH_TLS_DTPMOD64`](#r_larch_tls_dtpmod64) - Runtime relocation for TLS-GD
- [`R_LARCH_TLS_DTPREL32`](#r_larch_tls_dtprel32) - Runtime relocation for TLS-GD
- [`R_LARCH_TLS_DTPREL64`](#r_larch_tls_dtprel64) - Runtime relocation for TLS-GD
- [`R_LARCH_TLS_GD_HI20`](#r_larch_tls_gd_hi20) - 12..=31 bits of TLS GD GOT entry 32/64-bit absolute address
- [`R_LARCH_TLS_GD_PCADD_HI20`](#r_larch_tls_gd_pcadd_hi20) - The signed 32-bit offset `offs` from `PC` to
- [`R_LARCH_TLS_GD_PCADD_LO12`](#r_larch_tls_gd_pcadd_lo12) - 0..=11 bits of the 32-bit offset from the
- [`R_LARCH_TLS_GD_PCREL20_S2`](#r_larch_tls_gd_pcrel20_s2) - 22-bit PC-relative offset to TLS GD GOT entry
- [`R_LARCH_TLS_GD_PC_HI20`](#r_larch_tls_gd_pc_hi20) - 12..=31 bits of the 32/64-bit PC-relative offset to the PC-relative
- [`R_LARCH_TLS_IE64_HI12`](#r_larch_tls_ie64_hi12) - 51..=63 bits of TLS IE GOT entry 64-bit absolute address
- [`R_LARCH_TLS_IE64_LO20`](#r_larch_tls_ie64_lo20) - 32..=51 bits of TLS IE GOT entry 64-bit absolute address
- [`R_LARCH_TLS_IE64_PC_HI12`](#r_larch_tls_ie64_pc_hi12) - 52..=63 bits of the 64-bit offset from the
- [`R_LARCH_TLS_IE64_PC_LO20`](#r_larch_tls_ie64_pc_lo20) - 32..=51 bits of the 64-bit offset from the
- [`R_LARCH_TLS_IE_HI20`](#r_larch_tls_ie_hi20) - 12..=31 bits of TLS IE GOT entry 32/64-bit absolute address
- [`R_LARCH_TLS_IE_LO12`](#r_larch_tls_ie_lo12) - 0..=11 bits of TLS IE GOT entry 32/64-bit absolute address
- [`R_LARCH_TLS_IE_PCADD_HI20`](#r_larch_tls_ie_pcadd_hi20) - The signed 32-bit offset `offs` from `PC` to
- [`R_LARCH_TLS_IE_PCADD_LO12`](#r_larch_tls_ie_pcadd_lo12) - 0..=11 bits of the 32-bit offset from the
- [`R_LARCH_TLS_IE_PC_HI20`](#r_larch_tls_ie_pc_hi20) - The signed 32-bit offset `offs` from `PC & 0xfffff000` to
- [`R_LARCH_TLS_IE_PC_LO12`](#r_larch_tls_ie_pc_lo12) - 0..=12 bits of the 32/64-bit offset from the
- [`R_LARCH_TLS_LD_HI20`](#r_larch_tls_ld_hi20) - 12..=31 bits of TLS LD GOT entry 32/64-bit absolute address
- [`R_LARCH_TLS_LD_PCADD_HI20`](#r_larch_tls_ld_pcadd_hi20) - The signed 32-bit offset `offs` from `PC` to
- [`R_LARCH_TLS_LD_PCADD_LO12`](#r_larch_tls_ld_pcadd_lo12) - 0..=11 bits of the 32-bit offset from the
- [`R_LARCH_TLS_LD_PCREL20_S2`](#r_larch_tls_ld_pcrel20_s2) - 22-bit PC-relative offset to TLS LD GOT entry
- [`R_LARCH_TLS_LD_PC_HI20`](#r_larch_tls_ld_pc_hi20) - 12..=31 bits of the offset from `PC` to `GP + GD + 0x800`, where
- [`R_LARCH_TLS_LE64_HI12`](#r_larch_tls_le64_hi12) - 52..=63 bits of TLS LE 64-bit offset from thread pointer
- [`R_LARCH_TLS_LE64_LO20`](#r_larch_tls_le64_lo20) - 32..=51 bits of TLS LE 64-bit offset from thread pointer
- [`R_LARCH_TLS_LE_ADD_R`](#r_larch_tls_le_add_r) - TLS LE thread pointer usage, can be relaxed
- [`R_LARCH_TLS_LE_HI20`](#r_larch_tls_le_hi20) - 12..=31 bits of TLS LE 32/64-bit offset from thread pointer
- [`R_LARCH_TLS_LE_HI20_R`](#r_larch_tls_le_hi20_r) - 12..=31 bits of TLS LE 32/64-bit offset from TP register, can be relaxed
- [`R_LARCH_TLS_LE_LO12`](#r_larch_tls_le_lo12) - 0..=11 bits of TLS LE 32/64-bit offset from thread pointer
- [`R_LARCH_TLS_LE_LO12_R`](#r_larch_tls_le_lo12_r) - 0..=11 bits of TLS LE 32/64-bit offset from TP register, sign-extended,
- [`R_LARCH_TLS_TPREL32`](#r_larch_tls_tprel32) - Runtime relocation for TLE-IE
- [`R_LARCH_TLS_TPREL64`](#r_larch_tls_tprel64) - Runtime relocation for TLE-IE
- [`R_M32R_10_PCREL`](#r_m32r_10_pcrel) - PC relative 10 bit shifted.
- [`R_M32R_10_PCREL_RELA`](#r_m32r_10_pcrel_rela) - PC relative 10 bit shifted.
- [`R_M32R_16`](#r_m32r_16) - Direct 16 bit.
- [`R_M32R_16_RELA`](#r_m32r_16_rela) - Direct 16 bit.
- [`R_M32R_18_PCREL`](#r_m32r_18_pcrel) - PC relative 18 bit shifted.
- [`R_M32R_18_PCREL_RELA`](#r_m32r_18_pcrel_rela) - PC relative 18 bit shifted.
- [`R_M32R_24`](#r_m32r_24) - Direct 24 bit.
- [`R_M32R_24_RELA`](#r_m32r_24_rela) - Direct 24 bit.
- [`R_M32R_26_PCREL`](#r_m32r_26_pcrel) - PC relative 26 bit shifted.
- [`R_M32R_26_PCREL_RELA`](#r_m32r_26_pcrel_rela) - PC relative 26 bit shifted.
- [`R_M32R_26_PLTREL`](#r_m32r_26_pltrel) - 26 bit PC relative to PLT shifted
- [`R_M32R_32`](#r_m32r_32) - Direct 32 bit.
- [`R_M32R_32_RELA`](#r_m32r_32_rela) - Direct 32 bit.
- [`R_M32R_COPY`](#r_m32r_copy) - Copy symbol at runtime
- [`R_M32R_GLOB_DAT`](#r_m32r_glob_dat) - Create GOT entry
- [`R_M32R_GNU_VTENTRY`](#r_m32r_gnu_vtentry)
- [`R_M32R_GNU_VTINHERIT`](#r_m32r_gnu_vtinherit)
- [`R_M32R_GOT16_HI_SLO`](#r_m32r_got16_hi_slo) - High 16 bit GOT entry with signed low
- [`R_M32R_GOT16_HI_ULO`](#r_m32r_got16_hi_ulo) - High 16 bit GOT entry with unsigned low
- [`R_M32R_GOT16_LO`](#r_m32r_got16_lo) - Low 16 bit GOT entry
- [`R_M32R_GOT24`](#r_m32r_got24) - 24 bit GOT entry
- [`R_M32R_GOTOFF`](#r_m32r_gotoff) - 24 bit offset to GOT
- [`R_M32R_GOTOFF_HI_SLO`](#r_m32r_gotoff_hi_slo) - High 16 bit offset to GOT with signed low
- [`R_M32R_GOTOFF_HI_ULO`](#r_m32r_gotoff_hi_ulo) - High 16 bit offset to GOT with unsigned low
- [`R_M32R_GOTOFF_LO`](#r_m32r_gotoff_lo) - Low 16 bit offset to GOT
- [`R_M32R_GOTPC24`](#r_m32r_gotpc24) - 24 bit PC relative offset to GOT
- [`R_M32R_GOTPC_HI_SLO`](#r_m32r_gotpc_hi_slo) - High 16 bit PC relative offset to GOT with signed low
- [`R_M32R_GOTPC_HI_ULO`](#r_m32r_gotpc_hi_ulo) - High 16 bit PC relative offset to GOT with unsigned low
- [`R_M32R_GOTPC_LO`](#r_m32r_gotpc_lo) - Low 16 bit PC relative offset to GOT
- [`R_M32R_HI16_SLO`](#r_m32r_hi16_slo) - High 16 bit with signed low.
- [`R_M32R_HI16_SLO_RELA`](#r_m32r_hi16_slo_rela) - High 16 bit with signed low
- [`R_M32R_HI16_ULO`](#r_m32r_hi16_ulo) - High 16 bit with unsigned low.
- [`R_M32R_HI16_ULO_RELA`](#r_m32r_hi16_ulo_rela) - High 16 bit with unsigned low
- [`R_M32R_JMP_SLOT`](#r_m32r_jmp_slot) - Create PLT entry
- [`R_M32R_LO16`](#r_m32r_lo16) - Low 16 bit.
- [`R_M32R_LO16_RELA`](#r_m32r_lo16_rela) - Low 16 bit
- [`R_M32R_NONE`](#r_m32r_none) - No reloc.
- [`R_M32R_NUM`](#r_m32r_num) - Keep this the last entry.
- [`R_M32R_REL32`](#r_m32r_rel32) - PC relative 32 bit.
- [`R_M32R_RELATIVE`](#r_m32r_relative) - Adjust by program base
- [`R_M32R_RELA_GNU_VTENTRY`](#r_m32r_rela_gnu_vtentry)
- [`R_M32R_RELA_GNU_VTINHERIT`](#r_m32r_rela_gnu_vtinherit)
- [`R_M32R_SDA16`](#r_m32r_sda16) - 16 bit offset in SDA.
- [`R_M32R_SDA16_RELA`](#r_m32r_sda16_rela) - 16 bit offset in SDA
- [`R_METAG_ADDR32`](#r_metag_addr32) - 32bit absolute address
- [`R_METAG_COPY`](#r_metag_copy)
- [`R_METAG_GETSETOFF`](#r_metag_getsetoff)
- [`R_METAG_GETSET_GOT`](#r_metag_getset_got)
- [`R_METAG_GETSET_GOTOFF`](#r_metag_getset_gotoff)
- [`R_METAG_GLOB_DAT`](#r_metag_glob_dat)
- [`R_METAG_GNU_VTENTRY`](#r_metag_gnu_vtentry)
- [`R_METAG_GNU_VTINHERIT`](#r_metag_gnu_vtinherit)
- [`R_METAG_GOTOFF`](#r_metag_gotoff)
- [`R_METAG_HI16_GOTOFF`](#r_metag_hi16_gotoff)
- [`R_METAG_HI16_GOTPC`](#r_metag_hi16_gotpc)
- [`R_METAG_HI16_PLT`](#r_metag_hi16_plt)
- [`R_METAG_HIADDR16`](#r_metag_hiaddr16)
- [`R_METAG_HIOG`](#r_metag_hiog)
- [`R_METAG_JMP_SLOT`](#r_metag_jmp_slot)
- [`R_METAG_LO16_GOTOFF`](#r_metag_lo16_gotoff)
- [`R_METAG_LO16_GOTPC`](#r_metag_lo16_gotpc)
- [`R_METAG_LO16_PLT`](#r_metag_lo16_plt)
- [`R_METAG_LOADDR16`](#r_metag_loaddr16)
- [`R_METAG_LOOG`](#r_metag_loog)
- [`R_METAG_NONE`](#r_metag_none) - No reloc
- [`R_METAG_PLT`](#r_metag_plt)
- [`R_METAG_REG16OP1`](#r_metag_reg16op1)
- [`R_METAG_REG16OP2`](#r_metag_reg16op2)
- [`R_METAG_REG16OP3`](#r_metag_reg16op3)
- [`R_METAG_REG32OP1`](#r_metag_reg32op1)
- [`R_METAG_REG32OP2`](#r_metag_reg32op2)
- [`R_METAG_REG32OP3`](#r_metag_reg32op3)
- [`R_METAG_REG32OP4`](#r_metag_reg32op4)
- [`R_METAG_REL16`](#r_metag_rel16)
- [`R_METAG_REL8`](#r_metag_rel8)
- [`R_METAG_RELATIVE`](#r_metag_relative)
- [`R_METAG_RELBRANCH`](#r_metag_relbranch)
- [`R_METAG_RELBRANCH_PLT`](#r_metag_relbranch_plt)
- [`R_METAG_TLS_DTPMOD`](#r_metag_tls_dtpmod)
- [`R_METAG_TLS_DTPOFF`](#r_metag_tls_dtpoff)
- [`R_METAG_TLS_GD`](#r_metag_tls_gd)
- [`R_METAG_TLS_IE`](#r_metag_tls_ie)
- [`R_METAG_TLS_IENONPIC`](#r_metag_tls_ienonpic)
- [`R_METAG_TLS_IENONPIC_HI16`](#r_metag_tls_ienonpic_hi16)
- [`R_METAG_TLS_IENONPIC_LO16`](#r_metag_tls_ienonpic_lo16)
- [`R_METAG_TLS_LDM`](#r_metag_tls_ldm)
- [`R_METAG_TLS_LDO`](#r_metag_tls_ldo)
- [`R_METAG_TLS_LDO_HI16`](#r_metag_tls_ldo_hi16)
- [`R_METAG_TLS_LDO_LO16`](#r_metag_tls_ldo_lo16)
- [`R_METAG_TLS_LE`](#r_metag_tls_le)
- [`R_METAG_TLS_LE_HI16`](#r_metag_tls_le_hi16)
- [`R_METAG_TLS_LE_LO16`](#r_metag_tls_le_lo16)
- [`R_METAG_TLS_TPOFF`](#r_metag_tls_tpoff)
- [`R_MICROBLAZE_32`](#r_microblaze_32) - Direct 32 bit.
- [`R_MICROBLAZE_32_LO`](#r_microblaze_32_lo) - Low 16 bit.
- [`R_MICROBLAZE_32_PCREL`](#r_microblaze_32_pcrel) - PC relative 32 bit.
- [`R_MICROBLAZE_32_PCREL_LO`](#r_microblaze_32_pcrel_lo) - Low 16 bits of PCREL32.
- [`R_MICROBLAZE_32_SYM_OP_SYM`](#r_microblaze_32_sym_op_sym) - Symbol Op Symbol relocation.
- [`R_MICROBLAZE_64`](#r_microblaze_64) - Direct 64 bit.
- [`R_MICROBLAZE_64_NONE`](#r_microblaze_64_none) - No reloc.
- [`R_MICROBLAZE_64_PCREL`](#r_microblaze_64_pcrel) - PC relative 64 bit.
- [`R_MICROBLAZE_COPY`](#r_microblaze_copy) - Runtime copy.
- [`R_MICROBLAZE_GLOB_DAT`](#r_microblaze_glob_dat) - Create GOT entry.
- [`R_MICROBLAZE_GNU_VTENTRY`](#r_microblaze_gnu_vtentry) - GNU C++ vtable member usage.
- [`R_MICROBLAZE_GNU_VTINHERIT`](#r_microblaze_gnu_vtinherit) - GNU C++ vtable hierarchy.
- [`R_MICROBLAZE_GOTOFF_32`](#r_microblaze_gotoff_32) - 32 bit offset to GOT.
- [`R_MICROBLAZE_GOTOFF_64`](#r_microblaze_gotoff_64) - 64 bit offset to GOT.
- [`R_MICROBLAZE_GOTPC_64`](#r_microblaze_gotpc_64) - PC-relative GOT offset.
- [`R_MICROBLAZE_GOT_64`](#r_microblaze_got_64) - GOT entry offset.
- [`R_MICROBLAZE_JUMP_SLOT`](#r_microblaze_jump_slot) - Create PLT entry.
- [`R_MICROBLAZE_NONE`](#r_microblaze_none) - No reloc.
- [`R_MICROBLAZE_PLT_64`](#r_microblaze_plt_64) - PLT offset (PC-relative).
- [`R_MICROBLAZE_REL`](#r_microblaze_rel) - Adjust by program base.
- [`R_MICROBLAZE_SRO32`](#r_microblaze_sro32) - Read-only small data area.
- [`R_MICROBLAZE_SRW32`](#r_microblaze_srw32) - Read-write small data area.
- [`R_MICROBLAZE_TLS`](#r_microblaze_tls) - TLS Reloc.
- [`R_MICROBLAZE_TLSDTPMOD32`](#r_microblaze_tlsdtpmod32) - TLS Module ID.
- [`R_MICROBLAZE_TLSDTPREL32`](#r_microblaze_tlsdtprel32) - TLS Offset Within TLS Block.
- [`R_MICROBLAZE_TLSDTPREL64`](#r_microblaze_tlsdtprel64) - TLS Offset Within TLS Block.
- [`R_MICROBLAZE_TLSGD`](#r_microblaze_tlsgd) - TLS General Dynamic.
- [`R_MICROBLAZE_TLSGOTTPREL32`](#r_microblaze_tlsgottprel32) - TLS Offset From Thread Pointer.
- [`R_MICROBLAZE_TLSLD`](#r_microblaze_tlsld) - TLS Local Dynamic.
- [`R_MICROBLAZE_TLSTPREL32`](#r_microblaze_tlstprel32) - TLS Offset From Thread Pointer.
- [`R_MIPS_16`](#r_mips_16) - Direct 16 bit
- [`R_MIPS_26`](#r_mips_26) - Direct 26 bit shifted
- [`R_MIPS_32`](#r_mips_32) - Direct 32 bit
- [`R_MIPS_64`](#r_mips_64)
- [`R_MIPS_ADD_IMMEDIATE`](#r_mips_add_immediate)
- [`R_MIPS_CALL16`](#r_mips_call16) - 16 bit GOT entry for function
- [`R_MIPS_CALL_HI16`](#r_mips_call_hi16)
- [`R_MIPS_CALL_LO16`](#r_mips_call_lo16)
- [`R_MIPS_COPY`](#r_mips_copy)
- [`R_MIPS_DELETE`](#r_mips_delete)
- [`R_MIPS_GLOB_DAT`](#r_mips_glob_dat)
- [`R_MIPS_GOT16`](#r_mips_got16) - 16 bit GOT entry
- [`R_MIPS_GOT_DISP`](#r_mips_got_disp)
- [`R_MIPS_GOT_HI16`](#r_mips_got_hi16)
- [`R_MIPS_GOT_LO16`](#r_mips_got_lo16)
- [`R_MIPS_GOT_OFST`](#r_mips_got_ofst)
- [`R_MIPS_GOT_PAGE`](#r_mips_got_page)
- [`R_MIPS_GPREL16`](#r_mips_gprel16) - GP relative 16 bit
- [`R_MIPS_GPREL32`](#r_mips_gprel32) - GP relative 32 bit
- [`R_MIPS_HI16`](#r_mips_hi16) - High 16 bit
- [`R_MIPS_HIGHER`](#r_mips_higher)
- [`R_MIPS_HIGHEST`](#r_mips_highest)
- [`R_MIPS_INSERT_A`](#r_mips_insert_a)
- [`R_MIPS_INSERT_B`](#r_mips_insert_b)
- [`R_MIPS_JALR`](#r_mips_jalr)
- [`R_MIPS_JUMP_SLOT`](#r_mips_jump_slot)
- [`R_MIPS_LITERAL`](#r_mips_literal) - 16 bit literal entry
- [`R_MIPS_LO16`](#r_mips_lo16) - Low 16 bit
- [`R_MIPS_NONE`](#r_mips_none) - No reloc
- [`R_MIPS_PC16`](#r_mips_pc16) - PC relative 16 bit
- [`R_MIPS_PJUMP`](#r_mips_pjump)
- [`R_MIPS_REL16`](#r_mips_rel16)
- [`R_MIPS_REL32`](#r_mips_rel32) - PC relative 32 bit
- [`R_MIPS_RELGOT`](#r_mips_relgot)
- [`R_MIPS_SCN_DISP`](#r_mips_scn_disp)
- [`R_MIPS_SHIFT5`](#r_mips_shift5)
- [`R_MIPS_SHIFT6`](#r_mips_shift6)
- [`R_MIPS_SUB`](#r_mips_sub)
- [`R_MIPS_TLS_DTPMOD32`](#r_mips_tls_dtpmod32) - Module number 32 bit
- [`R_MIPS_TLS_DTPMOD64`](#r_mips_tls_dtpmod64) - Module number 64 bit
- [`R_MIPS_TLS_DTPREL32`](#r_mips_tls_dtprel32) - Module-relative offset 32 bit
- [`R_MIPS_TLS_DTPREL64`](#r_mips_tls_dtprel64) - Module-relative offset 64 bit
- [`R_MIPS_TLS_DTPREL_HI16`](#r_mips_tls_dtprel_hi16) - Module-relative offset, high 16 bits
- [`R_MIPS_TLS_DTPREL_LO16`](#r_mips_tls_dtprel_lo16) - Module-relative offset, low 16 bits
- [`R_MIPS_TLS_GD`](#r_mips_tls_gd) - 16 bit GOT offset for GD
- [`R_MIPS_TLS_GOTTPREL`](#r_mips_tls_gottprel) - 16 bit GOT offset for IE
- [`R_MIPS_TLS_LDM`](#r_mips_tls_ldm) - 16 bit GOT offset for LDM
- [`R_MIPS_TLS_TPREL32`](#r_mips_tls_tprel32) - TP-relative offset, 32 bit
- [`R_MIPS_TLS_TPREL64`](#r_mips_tls_tprel64) - TP-relative offset, 64 bit
- [`R_MIPS_TLS_TPREL_HI16`](#r_mips_tls_tprel_hi16) - TP-relative offset, high 16 bits
- [`R_MIPS_TLS_TPREL_LO16`](#r_mips_tls_tprel_lo16) - TP-relative offset, low 16 bits
- [`R_MN10300_16`](#r_mn10300_16) - Direct 16 bit.
- [`R_MN10300_24`](#r_mn10300_24) - Direct 24 bit.
- [`R_MN10300_32`](#r_mn10300_32) - Direct 32 bit.
- [`R_MN10300_8`](#r_mn10300_8) - Direct 8 bit.
- [`R_MN10300_ALIGN`](#r_mn10300_align) - Alignment requirement for linker relaxation.
- [`R_MN10300_COPY`](#r_mn10300_copy) - Copy symbol at runtime.
- [`R_MN10300_GLOB_DAT`](#r_mn10300_glob_dat) - Create GOT entry.
- [`R_MN10300_GNU_VTENTRY`](#r_mn10300_gnu_vtentry) - ... collection annotation.
- [`R_MN10300_GNU_VTINHERIT`](#r_mn10300_gnu_vtinherit) - Ancient C++ vtable garbage...
- [`R_MN10300_GOT16`](#r_mn10300_got16) - 16-bit offset to GOT entry.
- [`R_MN10300_GOT24`](#r_mn10300_got24) - 24-bit offset to GOT entry.
- [`R_MN10300_GOT32`](#r_mn10300_got32) - 32-bit offset to GOT entry.
- [`R_MN10300_GOTOFF16`](#r_mn10300_gotoff16) - 16-bit offset from GOT.
- [`R_MN10300_GOTOFF24`](#r_mn10300_gotoff24) - 24-bit offset from GOT.
- [`R_MN10300_GOTOFF32`](#r_mn10300_gotoff32) - 32-bit offset from GOT.
- [`R_MN10300_GOTPC16`](#r_mn10300_gotpc16) - 16-bit PCrel offset to GOT.
- [`R_MN10300_GOTPC32`](#r_mn10300_gotpc32) - 32-bit PCrel offset to GOT.
- [`R_MN10300_JMP_SLOT`](#r_mn10300_jmp_slot) - Create PLT entry.
- [`R_MN10300_NONE`](#r_mn10300_none) - No reloc.
- [`R_MN10300_PCREL16`](#r_mn10300_pcrel16) - PC-relative 16-bit signed.
- [`R_MN10300_PCREL32`](#r_mn10300_pcrel32) - PC-relative 32-bit.
- [`R_MN10300_PCREL8`](#r_mn10300_pcrel8) - PC-relative 8-bit signed.
- [`R_MN10300_PLT16`](#r_mn10300_plt16) - 16-bit PCrel to PLT entry.
- [`R_MN10300_PLT32`](#r_mn10300_plt32) - 32-bit PCrel to PLT entry.
- [`R_MN10300_RELATIVE`](#r_mn10300_relative) - Adjust by program base.
- [`R_MN10300_SYM_DIFF`](#r_mn10300_sym_diff) - Adjustment for next reloc as needed by linker relaxation.
- [`R_MN10300_TLS_DTPMOD`](#r_mn10300_tls_dtpmod) - ID of module containing symbol.
- [`R_MN10300_TLS_DTPOFF`](#r_mn10300_tls_dtpoff) - Offset in module TLS block.
- [`R_MN10300_TLS_GD`](#r_mn10300_tls_gd) - 32-bit offset for global dynamic.
- [`R_MN10300_TLS_GOTIE`](#r_mn10300_tls_gotie) - GOT offset for static TLS block offset.
- [`R_MN10300_TLS_IE`](#r_mn10300_tls_ie) - GOT address for static TLS block offset.
- [`R_MN10300_TLS_LD`](#r_mn10300_tls_ld) - 32-bit offset for local dynamic.
- [`R_MN10300_TLS_LDO`](#r_mn10300_tls_ldo) - Module-relative offset.
- [`R_MN10300_TLS_LE`](#r_mn10300_tls_le) - Offset relative to static TLS block.
- [`R_MN10300_TLS_TPOFF`](#r_mn10300_tls_tpoff) - Offset in static TLS block.
- [`R_MSP430_16_BYTE`](#r_msp430_16_byte) - Direct 16 bit
- [`R_MSP430_32`](#r_msp430_32) - Direct 32 bit
- [`R_MSP430_NONE`](#r_msp430_none) - No reloc
- [`R_NDS32_32_RELA`](#r_nds32_32_rela)
- [`R_NDS32_COPY`](#r_nds32_copy)
- [`R_NDS32_GLOB_DAT`](#r_nds32_glob_dat)
- [`R_NDS32_JMP_SLOT`](#r_nds32_jmp_slot)
- [`R_NDS32_NONE`](#r_nds32_none)
- [`R_NDS32_RELATIVE`](#r_nds32_relative)
- [`R_NDS32_TLS_DESC`](#r_nds32_tls_desc)
- [`R_NDS32_TLS_TPOFF`](#r_nds32_tls_tpoff)
- [`R_NIOS2_ALIGN`](#r_nios2_align) - Alignment requirement for linker relaxation.
- [`R_NIOS2_BFD_RELOC_16`](#r_nios2_bfd_reloc_16) - 16 bit symbol value + addend.
- [`R_NIOS2_BFD_RELOC_32`](#r_nios2_bfd_reloc_32) - 32 bit symbol value + addend.
- [`R_NIOS2_BFD_RELOC_8`](#r_nios2_bfd_reloc_8) - 8 bit symbol value + addend.
- [`R_NIOS2_CACHE_OPX`](#r_nios2_cache_opx) - 5 bit expression, shift 22.
- [`R_NIOS2_CALL16`](#r_nios2_call16) - 16 bit GOT entry for function.
- [`R_NIOS2_CALL26`](#r_nios2_call26) - Direct call.
- [`R_NIOS2_CALL26_NOAT`](#r_nios2_call26_noat) - Direct call in .noat section.
- [`R_NIOS2_CALLR`](#r_nios2_callr) - Indirect call through register.
- [`R_NIOS2_CALL_HA`](#r_nios2_call_ha) - %hiadj() of function GOT entry.
- [`R_NIOS2_CALL_LO`](#r_nios2_call_lo) - %lo() of function GOT entry.
- [`R_NIOS2_CJMP`](#r_nios2_cjmp) - Conditional branch.
- [`R_NIOS2_COPY`](#r_nios2_copy) - Copy symbol at runtime.
- [`R_NIOS2_GLOB_DAT`](#r_nios2_glob_dat) - Create GOT entry.
- [`R_NIOS2_GNU_VTENTRY`](#r_nios2_gnu_vtentry) - GNU C++ vtable member usage.
- [`R_NIOS2_GNU_VTINHERIT`](#r_nios2_gnu_vtinherit) - GNU C++ vtable hierarchy.
- [`R_NIOS2_GOT16`](#r_nios2_got16) - 16 bit GOT entry.
- [`R_NIOS2_GOTOFF`](#r_nios2_gotoff) - 16 bit offset to GOT pointer.
- [`R_NIOS2_GOTOFF_HA`](#r_nios2_gotoff_ha) - %hiadj of offset to GOT pointer.
- [`R_NIOS2_GOTOFF_LO`](#r_nios2_gotoff_lo) - %lo of offset to GOT pointer.
- [`R_NIOS2_GOT_HA`](#r_nios2_got_ha) - %hiadj() of GOT entry.
- [`R_NIOS2_GOT_LO`](#r_nios2_got_lo) - %lo() of GOT entry.
- [`R_NIOS2_GPREL`](#r_nios2_gprel) - 16 bit GP pointer offset.
- [`R_NIOS2_HI16`](#r_nios2_hi16) - High 16 bit.
- [`R_NIOS2_HIADJ16`](#r_nios2_hiadj16) - High 16 bit, adjusted.
- [`R_NIOS2_IMM5`](#r_nios2_imm5) - 5 bit constant expression.
- [`R_NIOS2_IMM6`](#r_nios2_imm6) - 6 bit constant expression.
- [`R_NIOS2_IMM8`](#r_nios2_imm8) - 8 bit constant expression.
- [`R_NIOS2_JUMP_SLOT`](#r_nios2_jump_slot) - Create PLT entry.
- [`R_NIOS2_LO16`](#r_nios2_lo16) - Low 16 bit.
- [`R_NIOS2_NONE`](#r_nios2_none) - No reloc.
- [`R_NIOS2_PCREL16`](#r_nios2_pcrel16) - PC relative 16 bit.
- [`R_NIOS2_PCREL_HA`](#r_nios2_pcrel_ha) - %hiadj of PC relative offset.
- [`R_NIOS2_PCREL_LO`](#r_nios2_pcrel_lo) - %lo of PC relative offset.
- [`R_NIOS2_RELATIVE`](#r_nios2_relative) - Adjust by program base.
- [`R_NIOS2_S16`](#r_nios2_s16) - Direct signed 16 bit.
- [`R_NIOS2_TLS_DTPMOD`](#r_nios2_tls_dtpmod) - Module number.
- [`R_NIOS2_TLS_DTPREL`](#r_nios2_tls_dtprel) - Module-relative offset.
- [`R_NIOS2_TLS_GD16`](#r_nios2_tls_gd16) - 16 bit GOT offset for TLS GD.
- [`R_NIOS2_TLS_IE16`](#r_nios2_tls_ie16) - 16 bit GOT offset for TLS IE.
- [`R_NIOS2_TLS_LDM16`](#r_nios2_tls_ldm16) - 16 bit GOT offset for TLS LDM.
- [`R_NIOS2_TLS_LDO16`](#r_nios2_tls_ldo16) - 16 bit module relative offset.
- [`R_NIOS2_TLS_LE16`](#r_nios2_tls_le16) - 16 bit LE TP-relative offset.
- [`R_NIOS2_TLS_TPREL`](#r_nios2_tls_tprel) - TP-relative offset.
- [`R_NIOS2_U16`](#r_nios2_u16) - Direct unsigned 16 bit.
- [`R_NIOS2_UJMP`](#r_nios2_ujmp) - Unconditional branch.
- [`R_PARISC_COPY`](#r_parisc_copy) - Copy relocation.
- [`R_PARISC_DIR14DR`](#r_parisc_dir14dr) - 14 bits of eff. address.
- [`R_PARISC_DIR14R`](#r_parisc_dir14r) - Right 14 bits of eff. address.
- [`R_PARISC_DIR14WR`](#r_parisc_dir14wr) - 14 bits of eff. address.
- [`R_PARISC_DIR16DF`](#r_parisc_dir16df) - 16 bits of eff. address.
- [`R_PARISC_DIR16F`](#r_parisc_dir16f) - 16 bits of eff. address.
- [`R_PARISC_DIR16WF`](#r_parisc_dir16wf) - 16 bits of eff. address.
- [`R_PARISC_DIR17F`](#r_parisc_dir17f) - 17 bits of eff. address.
- [`R_PARISC_DIR17R`](#r_parisc_dir17r) - Right 17 bits of eff. address.
- [`R_PARISC_DIR21L`](#r_parisc_dir21l) - Left 21 bits of eff. address.
- [`R_PARISC_DIR32`](#r_parisc_dir32) - Direct 32-bit reference.
- [`R_PARISC_DIR64`](#r_parisc_dir64) - 64 bits of eff. address.
- [`R_PARISC_DPREL14R`](#r_parisc_dprel14r) - Right 14 bits of rel. address.
- [`R_PARISC_DPREL21L`](#r_parisc_dprel21l) - Left 21 bits of rel. address.
- [`R_PARISC_EPLT`](#r_parisc_eplt) - Dynamic reloc, exported PLT
- [`R_PARISC_FPTR64`](#r_parisc_fptr64) - 64 bits function address.
- [`R_PARISC_GNU_VTENTRY`](#r_parisc_gnu_vtentry)
- [`R_PARISC_GNU_VTINHERIT`](#r_parisc_gnu_vtinherit)
- [`R_PARISC_GPREL14DR`](#r_parisc_gprel14dr) - GP-rel. address, right 14 bits.
- [`R_PARISC_GPREL14R`](#r_parisc_gprel14r) - GP-relative, right 14 bits.
- [`R_PARISC_GPREL14WR`](#r_parisc_gprel14wr) - GP-rel. address, right 14 bits.
- [`R_PARISC_GPREL16DF`](#r_parisc_gprel16df) - 16 bits GP-rel. address.
- [`R_PARISC_GPREL16F`](#r_parisc_gprel16f) - 16 bits GP-rel. address.
- [`R_PARISC_GPREL16WF`](#r_parisc_gprel16wf) - 16 bits GP-rel. address.
- [`R_PARISC_GPREL21L`](#r_parisc_gprel21l) - GP-relative, left 21 bits.
- [`R_PARISC_GPREL64`](#r_parisc_gprel64) - 64 bits of GP-rel. address.
- [`R_PARISC_HIRESERVE`](#r_parisc_hireserve)
- [`R_PARISC_IPLT`](#r_parisc_iplt) - Dynamic reloc, imported PLT
- [`R_PARISC_LORESERVE`](#r_parisc_loreserve)
- [`R_PARISC_LTOFF14DR`](#r_parisc_ltoff14dr) - LT-rel. address, right 14 bits.
- [`R_PARISC_LTOFF14R`](#r_parisc_ltoff14r) - LT-relative, right 14 bits.
- [`R_PARISC_LTOFF14WR`](#r_parisc_ltoff14wr) - LT-rel. address, right 14 bits.
- [`R_PARISC_LTOFF16DF`](#r_parisc_ltoff16df) - 16 bits LT-rel. address.
- [`R_PARISC_LTOFF16F`](#r_parisc_ltoff16f) - 16 bits LT-rel. address.
- [`R_PARISC_LTOFF16WF`](#r_parisc_ltoff16wf) - 16 bits LT-rel. address.
- [`R_PARISC_LTOFF21L`](#r_parisc_ltoff21l) - LT-relative, left 21 bits.
- [`R_PARISC_LTOFF64`](#r_parisc_ltoff64) - 64 bits LT-rel. address.
- [`R_PARISC_LTOFF_FPTR14DR`](#r_parisc_ltoff_fptr14dr) - LT-rel. fct. ptr., right 14 bits.
- [`R_PARISC_LTOFF_FPTR14R`](#r_parisc_ltoff_fptr14r) - LT-rel. fct ptr, right 14 bits.
- [`R_PARISC_LTOFF_FPTR14WR`](#r_parisc_ltoff_fptr14wr) - LT-rel. fct. ptr., right 14 bits.
- [`R_PARISC_LTOFF_FPTR16DF`](#r_parisc_ltoff_fptr16df) - 16 bits LT-rel. function ptr.
- [`R_PARISC_LTOFF_FPTR16F`](#r_parisc_ltoff_fptr16f) - 16 bits LT-rel. function ptr.
- [`R_PARISC_LTOFF_FPTR16WF`](#r_parisc_ltoff_fptr16wf) - 16 bits LT-rel. function ptr.
- [`R_PARISC_LTOFF_FPTR21L`](#r_parisc_ltoff_fptr21l) - LT-rel. fct ptr, left 21 bits.
- [`R_PARISC_LTOFF_FPTR32`](#r_parisc_ltoff_fptr32) - 32 bits LT-rel. function pointer.
- [`R_PARISC_LTOFF_FPTR64`](#r_parisc_ltoff_fptr64) - 64 bits LT-rel. function ptr.
- [`R_PARISC_LTOFF_TP14DR`](#r_parisc_ltoff_tp14dr) - LT-TP-rel. address, right 14 bits.
- [`R_PARISC_LTOFF_TP14F`](#r_parisc_ltoff_tp14f) - 14 bits LT-TP-rel. address.
- [`R_PARISC_LTOFF_TP14R`](#r_parisc_ltoff_tp14r) - LT-TP-rel. address, right 14 bits.
- [`R_PARISC_LTOFF_TP14WR`](#r_parisc_ltoff_tp14wr) - LT-TP-rel. address, right 14 bits.
- [`R_PARISC_LTOFF_TP16DF`](#r_parisc_ltoff_tp16df) - 16 bits LT-TP-rel. address.
- [`R_PARISC_LTOFF_TP16F`](#r_parisc_ltoff_tp16f) - 16 bits LT-TP-rel. address.
- [`R_PARISC_LTOFF_TP16WF`](#r_parisc_ltoff_tp16wf) - 16 bits LT-TP-rel. address.
- [`R_PARISC_LTOFF_TP21L`](#r_parisc_ltoff_tp21l) - LT-TP-rel. address, left 21 bits.
- [`R_PARISC_LTOFF_TP64`](#r_parisc_ltoff_tp64) - 64 bits LT-TP-rel. address.
- [`R_PARISC_NONE`](#r_parisc_none) - No reloc.
- [`R_PARISC_PCREL14DR`](#r_parisc_pcrel14dr) - PC rel. address, right 14 bits.
- [`R_PARISC_PCREL14R`](#r_parisc_pcrel14r) - Right 14 bits of rel. address.
- [`R_PARISC_PCREL14WR`](#r_parisc_pcrel14wr) - PC-rel. address, right 14 bits.
- [`R_PARISC_PCREL16DF`](#r_parisc_pcrel16df) - 16 bits PC-rel. address.
- [`R_PARISC_PCREL16F`](#r_parisc_pcrel16f) - 16 bits PC-rel. address.
- [`R_PARISC_PCREL16WF`](#r_parisc_pcrel16wf) - 16 bits PC-rel. address.
- [`R_PARISC_PCREL17F`](#r_parisc_pcrel17f) - 17 bits of rel. address.
- [`R_PARISC_PCREL17R`](#r_parisc_pcrel17r) - Right 17 bits of rel. address.
- [`R_PARISC_PCREL21L`](#r_parisc_pcrel21l) - Left 21 bits of rel. address.
- [`R_PARISC_PCREL22F`](#r_parisc_pcrel22f) - 22 bits PC-rel. address.
- [`R_PARISC_PCREL32`](#r_parisc_pcrel32) - 32-bit rel. address.
- [`R_PARISC_PCREL64`](#r_parisc_pcrel64) - 64 bits PC-rel. address.
- [`R_PARISC_PLABEL14R`](#r_parisc_plabel14r) - Right 14 bits of fdesc address.
- [`R_PARISC_PLABEL21L`](#r_parisc_plabel21l) - Left 21 bits of fdesc address.
- [`R_PARISC_PLABEL32`](#r_parisc_plabel32) - 32 bits function address.
- [`R_PARISC_PLTOFF14DR`](#r_parisc_pltoff14dr) - PLT-rel. address, right 14 bits.
- [`R_PARISC_PLTOFF14R`](#r_parisc_pltoff14r) - PLT rel. address, right 14 bits.
- [`R_PARISC_PLTOFF14WR`](#r_parisc_pltoff14wr) - PLT-rel. address, right 14 bits.
- [`R_PARISC_PLTOFF16DF`](#r_parisc_pltoff16df) - 16 bits PLT-rel. address.
- [`R_PARISC_PLTOFF16F`](#r_parisc_pltoff16f) - 16 bits LT-rel. address.
- [`R_PARISC_PLTOFF16WF`](#r_parisc_pltoff16wf) - 16 bits PLT-rel. address.
- [`R_PARISC_PLTOFF21L`](#r_parisc_pltoff21l) - PLT rel. address, left 21 bits.
- [`R_PARISC_SECREL32`](#r_parisc_secrel32) - 32 bits section rel. address.
- [`R_PARISC_SECREL64`](#r_parisc_secrel64) - 64 bits section rel. address.
- [`R_PARISC_SEGBASE`](#r_parisc_segbase) - No relocation, set segment base.
- [`R_PARISC_SEGREL32`](#r_parisc_segrel32) - 32 bits segment rel. address.
- [`R_PARISC_SEGREL64`](#r_parisc_segrel64) - 64 bits segment rel. address.
- [`R_PARISC_TLS_DTPMOD32`](#r_parisc_tls_dtpmod32) - DTP module 32-bit.
- [`R_PARISC_TLS_DTPMOD64`](#r_parisc_tls_dtpmod64) - DTP module 64-bit.
- [`R_PARISC_TLS_DTPOFF32`](#r_parisc_tls_dtpoff32) - DTP offset 32-bit.
- [`R_PARISC_TLS_DTPOFF64`](#r_parisc_tls_dtpoff64) - DTP offset 32-bit.
- [`R_PARISC_TLS_GD14R`](#r_parisc_tls_gd14r) - GD 14-bit right.
- [`R_PARISC_TLS_GD21L`](#r_parisc_tls_gd21l) - GD 21-bit left.
- [`R_PARISC_TLS_GDCALL`](#r_parisc_tls_gdcall) - GD call to __t_g_a.
- [`R_PARISC_TLS_IE14R`](#r_parisc_tls_ie14r)
- [`R_PARISC_TLS_IE21L`](#r_parisc_tls_ie21l)
- [`R_PARISC_TLS_LDM14R`](#r_parisc_tls_ldm14r) - LD module 14-bit right.
- [`R_PARISC_TLS_LDM21L`](#r_parisc_tls_ldm21l) - LD module 21-bit left.
- [`R_PARISC_TLS_LDMCALL`](#r_parisc_tls_ldmcall) - LD module call to __t_g_a.
- [`R_PARISC_TLS_LDO14R`](#r_parisc_tls_ldo14r) - LD offset 14-bit right.
- [`R_PARISC_TLS_LDO21L`](#r_parisc_tls_ldo21l) - LD offset 21-bit left.
- [`R_PARISC_TLS_LE14R`](#r_parisc_tls_le14r)
- [`R_PARISC_TLS_LE21L`](#r_parisc_tls_le21l)
- [`R_PARISC_TLS_TPREL32`](#r_parisc_tls_tprel32)
- [`R_PARISC_TLS_TPREL64`](#r_parisc_tls_tprel64)
- [`R_PARISC_TPREL14DR`](#r_parisc_tprel14dr) - TP-rel. address, right 14 bits.
- [`R_PARISC_TPREL14R`](#r_parisc_tprel14r) - TP-rel. address, right 14 bits.
- [`R_PARISC_TPREL14WR`](#r_parisc_tprel14wr) - TP-rel. address, right 14 bits.
- [`R_PARISC_TPREL16DF`](#r_parisc_tprel16df) - 16 bits TP-rel. address.
- [`R_PARISC_TPREL16F`](#r_parisc_tprel16f) - 16 bits TP-rel. address.
- [`R_PARISC_TPREL16WF`](#r_parisc_tprel16wf) - 16 bits TP-rel. address.
- [`R_PARISC_TPREL21L`](#r_parisc_tprel21l) - TP-rel. address, left 21 bits.
- [`R_PARISC_TPREL32`](#r_parisc_tprel32) - 32 bits TP-rel. address.
- [`R_PARISC_TPREL64`](#r_parisc_tprel64) - 64 bits TP-rel. address.
- [`R_PPC64_ADDR14`](#r_ppc64_addr14) - 16bit address, word aligned
- [`R_PPC64_ADDR14_BRNTAKEN`](#r_ppc64_addr14_brntaken)
- [`R_PPC64_ADDR14_BRTAKEN`](#r_ppc64_addr14_brtaken)
- [`R_PPC64_ADDR16`](#r_ppc64_addr16) - 16bit absolute address
- [`R_PPC64_ADDR16_DS`](#r_ppc64_addr16_ds) - half16ds* (S + A) >> 2
- [`R_PPC64_ADDR16_HA`](#r_ppc64_addr16_ha) - adjusted high 16bits.
- [`R_PPC64_ADDR16_HI`](#r_ppc64_addr16_hi) - high 16bits of address.
- [`R_PPC64_ADDR16_HIGH`](#r_ppc64_addr16_high)
- [`R_PPC64_ADDR16_HIGHA`](#r_ppc64_addr16_higha)
- [`R_PPC64_ADDR16_HIGHER`](#r_ppc64_addr16_higher) - half16 #higher(S + A)
- [`R_PPC64_ADDR16_HIGHERA`](#r_ppc64_addr16_highera) - half16 #highera(S + A)
- [`R_PPC64_ADDR16_HIGHEST`](#r_ppc64_addr16_highest) - half16 #highest(S + A)
- [`R_PPC64_ADDR16_HIGHESTA`](#r_ppc64_addr16_highesta) - half16 #highesta(S + A)
- [`R_PPC64_ADDR16_LO`](#r_ppc64_addr16_lo) - lower 16bits of address
- [`R_PPC64_ADDR16_LO_DS`](#r_ppc64_addr16_lo_ds) - half16ds  #lo(S + A) >> 2
- [`R_PPC64_ADDR24`](#r_ppc64_addr24) - 26bit address, word aligned
- [`R_PPC64_ADDR30`](#r_ppc64_addr30) - word30 (S + A - P) >> 2
- [`R_PPC64_ADDR32`](#r_ppc64_addr32) - 32bit absolute address
- [`R_PPC64_ADDR64`](#r_ppc64_addr64) - doubleword64 S + A
- [`R_PPC64_COPY`](#r_ppc64_copy)
- [`R_PPC64_DTPMOD64`](#r_ppc64_dtpmod64) - doubleword64 (sym+add)@dtpmod
- [`R_PPC64_DTPREL16`](#r_ppc64_dtprel16) - half16* (sym+add)@dtprel
- [`R_PPC64_DTPREL16_DS`](#r_ppc64_dtprel16_ds) - half16ds* (sym+add)@dtprel
- [`R_PPC64_DTPREL16_HA`](#r_ppc64_dtprel16_ha) - half16  (sym+add)@dtprel@ha
- [`R_PPC64_DTPREL16_HI`](#r_ppc64_dtprel16_hi) - half16  (sym+add)@dtprel@h
- [`R_PPC64_DTPREL16_HIGH`](#r_ppc64_dtprel16_high)
- [`R_PPC64_DTPREL16_HIGHA`](#r_ppc64_dtprel16_higha)
- [`R_PPC64_DTPREL16_HIGHER`](#r_ppc64_dtprel16_higher) - half16  (sym+add)@dtprel@higher
- [`R_PPC64_DTPREL16_HIGHERA`](#r_ppc64_dtprel16_highera) - half16  (sym+add)@dtprel@highera
- [`R_PPC64_DTPREL16_HIGHEST`](#r_ppc64_dtprel16_highest) - half16  (sym+add)@dtprel@highest
- [`R_PPC64_DTPREL16_HIGHESTA`](#r_ppc64_dtprel16_highesta) - half16  (sym+add)@dtprel@highesta
- [`R_PPC64_DTPREL16_LO`](#r_ppc64_dtprel16_lo) - half16  (sym+add)@dtprel@l
- [`R_PPC64_DTPREL16_LO_DS`](#r_ppc64_dtprel16_lo_ds) - half16ds (sym+add)@dtprel@l
- [`R_PPC64_DTPREL64`](#r_ppc64_dtprel64) - doubleword64 (sym+add)@dtprel
- [`R_PPC64_GLOB_DAT`](#r_ppc64_glob_dat)
- [`R_PPC64_GOT16`](#r_ppc64_got16)
- [`R_PPC64_GOT16_DS`](#r_ppc64_got16_ds) - half16ds* (G + A) >> 2
- [`R_PPC64_GOT16_HA`](#r_ppc64_got16_ha)
- [`R_PPC64_GOT16_HI`](#r_ppc64_got16_hi)
- [`R_PPC64_GOT16_LO`](#r_ppc64_got16_lo)
- [`R_PPC64_GOT16_LO_DS`](#r_ppc64_got16_lo_ds) - half16ds  #lo(G + A) >> 2
- [`R_PPC64_GOT_DTPREL16_DS`](#r_ppc64_got_dtprel16_ds) - half16ds* (sym+add)@got@dtprel
- [`R_PPC64_GOT_DTPREL16_HA`](#r_ppc64_got_dtprel16_ha) - half16  (sym+add)@got@dtprel@ha
- [`R_PPC64_GOT_DTPREL16_HI`](#r_ppc64_got_dtprel16_hi) - half16  (sym+add)@got@dtprel@h
- [`R_PPC64_GOT_DTPREL16_LO_DS`](#r_ppc64_got_dtprel16_lo_ds) - half16ds (sym+add)@got@dtprel@l
- [`R_PPC64_GOT_TLSGD16`](#r_ppc64_got_tlsgd16) - half16* (sym+add)@got@tlsgd
- [`R_PPC64_GOT_TLSGD16_HA`](#r_ppc64_got_tlsgd16_ha) - half16  (sym+add)@got@tlsgd@ha
- [`R_PPC64_GOT_TLSGD16_HI`](#r_ppc64_got_tlsgd16_hi) - half16  (sym+add)@got@tlsgd@h
- [`R_PPC64_GOT_TLSGD16_LO`](#r_ppc64_got_tlsgd16_lo) - half16  (sym+add)@got@tlsgd@l
- [`R_PPC64_GOT_TLSLD16`](#r_ppc64_got_tlsld16) - half16* (sym+add)@got@tlsld
- [`R_PPC64_GOT_TLSLD16_HA`](#r_ppc64_got_tlsld16_ha) - half16  (sym+add)@got@tlsld@ha
- [`R_PPC64_GOT_TLSLD16_HI`](#r_ppc64_got_tlsld16_hi) - half16  (sym+add)@got@tlsld@h
- [`R_PPC64_GOT_TLSLD16_LO`](#r_ppc64_got_tlsld16_lo) - half16  (sym+add)@got@tlsld@l
- [`R_PPC64_GOT_TPREL16_DS`](#r_ppc64_got_tprel16_ds) - half16ds* (sym+add)@got@tprel
- [`R_PPC64_GOT_TPREL16_HA`](#r_ppc64_got_tprel16_ha) - half16  (sym+add)@got@tprel@ha
- [`R_PPC64_GOT_TPREL16_HI`](#r_ppc64_got_tprel16_hi) - half16  (sym+add)@got@tprel@h
- [`R_PPC64_GOT_TPREL16_LO_DS`](#r_ppc64_got_tprel16_lo_ds) - half16ds (sym+add)@got@tprel@l
- [`R_PPC64_IRELATIVE`](#r_ppc64_irelative) - GNU extension to support local ifunc.
- [`R_PPC64_JMP_IREL`](#r_ppc64_jmp_irel) - GNU extension to support local ifunc.
- [`R_PPC64_JMP_SLOT`](#r_ppc64_jmp_slot)
- [`R_PPC64_NONE`](#r_ppc64_none)
- [`R_PPC64_PLT16_HA`](#r_ppc64_plt16_ha)
- [`R_PPC64_PLT16_HI`](#r_ppc64_plt16_hi)
- [`R_PPC64_PLT16_LO`](#r_ppc64_plt16_lo)
- [`R_PPC64_PLT16_LO_DS`](#r_ppc64_plt16_lo_ds) - half16ds  #lo(L + A) >> 2
- [`R_PPC64_PLT32`](#r_ppc64_plt32)
- [`R_PPC64_PLT64`](#r_ppc64_plt64) - doubleword64 L + A
- [`R_PPC64_PLTGOT16`](#r_ppc64_pltgot16) - half16* M + A
- [`R_PPC64_PLTGOT16_DS`](#r_ppc64_pltgot16_ds) - half16ds* (M + A) >> 2
- [`R_PPC64_PLTGOT16_HA`](#r_ppc64_pltgot16_ha) - half16 #ha(M + A)
- [`R_PPC64_PLTGOT16_HI`](#r_ppc64_pltgot16_hi) - half16 #hi(M + A)
- [`R_PPC64_PLTGOT16_LO`](#r_ppc64_pltgot16_lo) - half16 #lo(M + A)
- [`R_PPC64_PLTGOT16_LO_DS`](#r_ppc64_pltgot16_lo_ds) - half16ds  #lo(M + A) >> 2
- [`R_PPC64_PLTREL32`](#r_ppc64_pltrel32)
- [`R_PPC64_PLTREL64`](#r_ppc64_pltrel64) - doubleword64 L + A - P
- [`R_PPC64_REL14`](#r_ppc64_rel14) - PC relative 16 bit
- [`R_PPC64_REL14_BRNTAKEN`](#r_ppc64_rel14_brntaken)
- [`R_PPC64_REL14_BRTAKEN`](#r_ppc64_rel14_brtaken)
- [`R_PPC64_REL16`](#r_ppc64_rel16) - half16   (sym+add-.)
- [`R_PPC64_REL16_HA`](#r_ppc64_rel16_ha) - half16   (sym+add-.)@ha
- [`R_PPC64_REL16_HI`](#r_ppc64_rel16_hi) - half16   (sym+add-.)@h
- [`R_PPC64_REL16_LO`](#r_ppc64_rel16_lo) - half16   (sym+add-.)@l
- [`R_PPC64_REL24`](#r_ppc64_rel24) - PC-rel. 26 bit, word aligned
- [`R_PPC64_REL32`](#r_ppc64_rel32)
- [`R_PPC64_REL64`](#r_ppc64_rel64) - doubleword64 S + A - P
- [`R_PPC64_RELATIVE`](#r_ppc64_relative)
- [`R_PPC64_SECTOFF`](#r_ppc64_sectoff)
- [`R_PPC64_SECTOFF_DS`](#r_ppc64_sectoff_ds) - half16ds* (R + A) >> 2
- [`R_PPC64_SECTOFF_HA`](#r_ppc64_sectoff_ha)
- [`R_PPC64_SECTOFF_HI`](#r_ppc64_sectoff_hi)
- [`R_PPC64_SECTOFF_LO`](#r_ppc64_sectoff_lo)
- [`R_PPC64_SECTOFF_LO_DS`](#r_ppc64_sectoff_lo_ds) - half16ds  #lo(R + A) >> 2
- [`R_PPC64_TLS`](#r_ppc64_tls) - none    (sym+add)@tls
- [`R_PPC64_TLSGD`](#r_ppc64_tlsgd) - none    (sym+add)@tlsgd
- [`R_PPC64_TLSLD`](#r_ppc64_tlsld) - none    (sym+add)@tlsld
- [`R_PPC64_TOC`](#r_ppc64_toc) - doubleword64 .TOC
- [`R_PPC64_TOC16`](#r_ppc64_toc16) - half16* S + A - .TOC
- [`R_PPC64_TOC16_DS`](#r_ppc64_toc16_ds) - half16ds* (S + A - .TOC.) >> 2
- [`R_PPC64_TOC16_HA`](#r_ppc64_toc16_ha) - half16 #ha(S + A - .TOC.)
- [`R_PPC64_TOC16_HI`](#r_ppc64_toc16_hi) - half16 #hi(S + A - .TOC.)
- [`R_PPC64_TOC16_LO`](#r_ppc64_toc16_lo) - half16 #lo(S + A - .TOC.)
- [`R_PPC64_TOC16_LO_DS`](#r_ppc64_toc16_lo_ds) - half16ds  #lo(S + A - .TOC.) >> 2
- [`R_PPC64_TOCSAVE`](#r_ppc64_tocsave) - none
- [`R_PPC64_TPREL16`](#r_ppc64_tprel16) - half16* (sym+add)@tprel
- [`R_PPC64_TPREL16_DS`](#r_ppc64_tprel16_ds) - half16ds* (sym+add)@tprel
- [`R_PPC64_TPREL16_HA`](#r_ppc64_tprel16_ha) - half16  (sym+add)@tprel@ha
- [`R_PPC64_TPREL16_HI`](#r_ppc64_tprel16_hi) - half16  (sym+add)@tprel@h
- [`R_PPC64_TPREL16_HIGH`](#r_ppc64_tprel16_high)
- [`R_PPC64_TPREL16_HIGHA`](#r_ppc64_tprel16_higha)
- [`R_PPC64_TPREL16_HIGHER`](#r_ppc64_tprel16_higher) - half16  (sym+add)@tprel@higher
- [`R_PPC64_TPREL16_HIGHERA`](#r_ppc64_tprel16_highera) - half16  (sym+add)@tprel@highera
- [`R_PPC64_TPREL16_HIGHEST`](#r_ppc64_tprel16_highest) - half16  (sym+add)@tprel@highest
- [`R_PPC64_TPREL16_HIGHESTA`](#r_ppc64_tprel16_highesta) - half16  (sym+add)@tprel@highesta
- [`R_PPC64_TPREL16_LO`](#r_ppc64_tprel16_lo) - half16  (sym+add)@tprel@l
- [`R_PPC64_TPREL16_LO_DS`](#r_ppc64_tprel16_lo_ds) - half16ds (sym+add)@tprel@l
- [`R_PPC64_TPREL64`](#r_ppc64_tprel64) - doubleword64 (sym+add)@tprel
- [`R_PPC64_UADDR16`](#r_ppc64_uaddr16)
- [`R_PPC64_UADDR32`](#r_ppc64_uaddr32)
- [`R_PPC64_UADDR64`](#r_ppc64_uaddr64) - doubleword64 S + A
- [`R_PPC_ADDR14`](#r_ppc_addr14) - 16bit address, 2 bits ignored
- [`R_PPC_ADDR14_BRNTAKEN`](#r_ppc_addr14_brntaken)
- [`R_PPC_ADDR14_BRTAKEN`](#r_ppc_addr14_brtaken)
- [`R_PPC_ADDR16`](#r_ppc_addr16) - 16bit absolute address
- [`R_PPC_ADDR16_HA`](#r_ppc_addr16_ha) - adjusted high 16bit
- [`R_PPC_ADDR16_HI`](#r_ppc_addr16_hi) - high 16bit of absolute address
- [`R_PPC_ADDR16_LO`](#r_ppc_addr16_lo) - lower 16bit of absolute address
- [`R_PPC_ADDR24`](#r_ppc_addr24) - 26bit address, 2 bits ignored.
- [`R_PPC_ADDR32`](#r_ppc_addr32) - 32bit absolute address
- [`R_PPC_COPY`](#r_ppc_copy)
- [`R_PPC_DIAB_RELSDA_HA`](#r_ppc_diab_relsda_ha) - like EMB_RELSDA, adjusted high 16
- [`R_PPC_DIAB_RELSDA_HI`](#r_ppc_diab_relsda_hi) - like EMB_RELSDA, but high 16 bit
- [`R_PPC_DIAB_RELSDA_LO`](#r_ppc_diab_relsda_lo) - like EMB_RELSDA, but lower 16 bit
- [`R_PPC_DIAB_SDA21_HA`](#r_ppc_diab_sda21_ha) - like EMB_SDA21, adjusted high 16
- [`R_PPC_DIAB_SDA21_HI`](#r_ppc_diab_sda21_hi) - like EMB_SDA21, but high 16 bit
- [`R_PPC_DIAB_SDA21_LO`](#r_ppc_diab_sda21_lo) - like EMB_SDA21, but lower 16 bit
- [`R_PPC_DTPMOD32`](#r_ppc_dtpmod32) - word32  (sym+add)@dtpmod
- [`R_PPC_DTPREL16`](#r_ppc_dtprel16) - half16*(sym+add)@dtprel
- [`R_PPC_DTPREL16_HA`](#r_ppc_dtprel16_ha) - half16  (sym+add)@dtprel@ha
- [`R_PPC_DTPREL16_HI`](#r_ppc_dtprel16_hi) - half16  (sym+add)@dtprel@h
- [`R_PPC_DTPREL16_LO`](#r_ppc_dtprel16_lo) - half16  (sym+add)@dtprel@l
- [`R_PPC_DTPREL32`](#r_ppc_dtprel32) - word32  (sym+add)@dtprel
- [`R_PPC_EMB_BIT_FLD`](#r_ppc_emb_bit_fld)
- [`R_PPC_EMB_MRKREF`](#r_ppc_emb_mrkref)
- [`R_PPC_EMB_NADDR16`](#r_ppc_emb_naddr16)
- [`R_PPC_EMB_NADDR16_HA`](#r_ppc_emb_naddr16_ha)
- [`R_PPC_EMB_NADDR16_HI`](#r_ppc_emb_naddr16_hi)
- [`R_PPC_EMB_NADDR16_LO`](#r_ppc_emb_naddr16_lo)
- [`R_PPC_EMB_NADDR32`](#r_ppc_emb_naddr32)
- [`R_PPC_EMB_RELSDA`](#r_ppc_emb_relsda) - 16 bit relative offset in SDA
- [`R_PPC_EMB_RELSEC16`](#r_ppc_emb_relsec16)
- [`R_PPC_EMB_RELST_HA`](#r_ppc_emb_relst_ha)
- [`R_PPC_EMB_RELST_HI`](#r_ppc_emb_relst_hi)
- [`R_PPC_EMB_RELST_LO`](#r_ppc_emb_relst_lo)
- [`R_PPC_EMB_SDA21`](#r_ppc_emb_sda21) - 16 bit offset in SDA
- [`R_PPC_EMB_SDA2I16`](#r_ppc_emb_sda2i16)
- [`R_PPC_EMB_SDA2REL`](#r_ppc_emb_sda2rel)
- [`R_PPC_EMB_SDAI16`](#r_ppc_emb_sdai16)
- [`R_PPC_GLOB_DAT`](#r_ppc_glob_dat)
- [`R_PPC_GOT16`](#r_ppc_got16)
- [`R_PPC_GOT16_HA`](#r_ppc_got16_ha)
- [`R_PPC_GOT16_HI`](#r_ppc_got16_hi)
- [`R_PPC_GOT16_LO`](#r_ppc_got16_lo)
- [`R_PPC_GOT_DTPREL16`](#r_ppc_got_dtprel16) - half16* (sym+add)@got@dtprel
- [`R_PPC_GOT_DTPREL16_HA`](#r_ppc_got_dtprel16_ha) - half16* (sym+add)@got@dtprel@ha
- [`R_PPC_GOT_DTPREL16_HI`](#r_ppc_got_dtprel16_hi) - half16* (sym+add)@got@dtprel@h
- [`R_PPC_GOT_DTPREL16_LO`](#r_ppc_got_dtprel16_lo) - half16* (sym+add)@got@dtprel@l
- [`R_PPC_GOT_TLSGD16`](#r_ppc_got_tlsgd16) - half16* (sym+add)@got@tlsgd
- [`R_PPC_GOT_TLSGD16_HA`](#r_ppc_got_tlsgd16_ha) - half16  (sym+add)@got@tlsgd@ha
- [`R_PPC_GOT_TLSGD16_HI`](#r_ppc_got_tlsgd16_hi) - half16  (sym+add)@got@tlsgd@h
- [`R_PPC_GOT_TLSGD16_LO`](#r_ppc_got_tlsgd16_lo) - half16  (sym+add)@got@tlsgd@l
- [`R_PPC_GOT_TLSLD16`](#r_ppc_got_tlsld16) - half16* (sym+add)@got@tlsld
- [`R_PPC_GOT_TLSLD16_HA`](#r_ppc_got_tlsld16_ha) - half16  (sym+add)@got@tlsld@ha
- [`R_PPC_GOT_TLSLD16_HI`](#r_ppc_got_tlsld16_hi) - half16  (sym+add)@got@tlsld@h
- [`R_PPC_GOT_TLSLD16_LO`](#r_ppc_got_tlsld16_lo) - half16  (sym+add)@got@tlsld@l
- [`R_PPC_GOT_TPREL16`](#r_ppc_got_tprel16) - half16* (sym+add)@got@tprel
- [`R_PPC_GOT_TPREL16_HA`](#r_ppc_got_tprel16_ha) - half16  (sym+add)@got@tprel@ha
- [`R_PPC_GOT_TPREL16_HI`](#r_ppc_got_tprel16_hi) - half16  (sym+add)@got@tprel@h
- [`R_PPC_GOT_TPREL16_LO`](#r_ppc_got_tprel16_lo) - half16  (sym+add)@got@tprel@l
- [`R_PPC_IRELATIVE`](#r_ppc_irelative) - GNU extension to support local ifunc.
- [`R_PPC_JMP_SLOT`](#r_ppc_jmp_slot)
- [`R_PPC_LOCAL24PC`](#r_ppc_local24pc)
- [`R_PPC_NONE`](#r_ppc_none)
- [`R_PPC_PLT16_HA`](#r_ppc_plt16_ha)
- [`R_PPC_PLT16_HI`](#r_ppc_plt16_hi)
- [`R_PPC_PLT16_LO`](#r_ppc_plt16_lo)
- [`R_PPC_PLT32`](#r_ppc_plt32)
- [`R_PPC_PLTREL24`](#r_ppc_pltrel24)
- [`R_PPC_PLTREL32`](#r_ppc_pltrel32)
- [`R_PPC_REL14`](#r_ppc_rel14) - PC relative 16 bit
- [`R_PPC_REL14_BRNTAKEN`](#r_ppc_rel14_brntaken)
- [`R_PPC_REL14_BRTAKEN`](#r_ppc_rel14_brtaken)
- [`R_PPC_REL16`](#r_ppc_rel16) - half16   (sym+add-.)
- [`R_PPC_REL16_HA`](#r_ppc_rel16_ha) - half16   (sym+add-.)@ha
- [`R_PPC_REL16_HI`](#r_ppc_rel16_hi) - half16   (sym+add-.)@h
- [`R_PPC_REL16_LO`](#r_ppc_rel16_lo) - half16   (sym+add-.)@l
- [`R_PPC_REL24`](#r_ppc_rel24) - PC relative 26 bit
- [`R_PPC_REL32`](#r_ppc_rel32)
- [`R_PPC_RELATIVE`](#r_ppc_relative)
- [`R_PPC_SDAREL16`](#r_ppc_sdarel16)
- [`R_PPC_SECTOFF`](#r_ppc_sectoff)
- [`R_PPC_SECTOFF_HA`](#r_ppc_sectoff_ha)
- [`R_PPC_SECTOFF_HI`](#r_ppc_sectoff_hi)
- [`R_PPC_SECTOFF_LO`](#r_ppc_sectoff_lo)
- [`R_PPC_TLS`](#r_ppc_tls) - none    (sym+add)@tls
- [`R_PPC_TLSGD`](#r_ppc_tlsgd) - none    (sym+add)@tlsgd
- [`R_PPC_TLSLD`](#r_ppc_tlsld) - none    (sym+add)@tlsld
- [`R_PPC_TOC16`](#r_ppc_toc16) - This is a phony reloc to handle any old fashioned TOC16 references that may
- [`R_PPC_TPREL16`](#r_ppc_tprel16) - half16* (sym+add)@tprel
- [`R_PPC_TPREL16_HA`](#r_ppc_tprel16_ha) - half16  (sym+add)@tprel@ha
- [`R_PPC_TPREL16_HI`](#r_ppc_tprel16_hi) - half16  (sym+add)@tprel@h
- [`R_PPC_TPREL16_LO`](#r_ppc_tprel16_lo) - half16  (sym+add)@tprel@l
- [`R_PPC_TPREL32`](#r_ppc_tprel32) - word32  (sym+add)@tprel
- [`R_PPC_UADDR16`](#r_ppc_uaddr16)
- [`R_PPC_UADDR32`](#r_ppc_uaddr32)
- [`R_RISCV_32`](#r_riscv_32)
- [`R_RISCV_32_PCREL`](#r_riscv_32_pcrel)
- [`R_RISCV_64`](#r_riscv_64)
- [`R_RISCV_ADD16`](#r_riscv_add16)
- [`R_RISCV_ADD32`](#r_riscv_add32)
- [`R_RISCV_ADD64`](#r_riscv_add64)
- [`R_RISCV_ADD8`](#r_riscv_add8)
- [`R_RISCV_ALIGN`](#r_riscv_align)
- [`R_RISCV_BRANCH`](#r_riscv_branch)
- [`R_RISCV_CALL`](#r_riscv_call)
- [`R_RISCV_CALL_PLT`](#r_riscv_call_plt)
- [`R_RISCV_COPY`](#r_riscv_copy)
- [`R_RISCV_GOT32_PCREL`](#r_riscv_got32_pcrel)
- [`R_RISCV_GOT_HI20`](#r_riscv_got_hi20)
- [`R_RISCV_GPREL_I`](#r_riscv_gprel_i)
- [`R_RISCV_GPREL_S`](#r_riscv_gprel_s)
- [`R_RISCV_HI20`](#r_riscv_hi20)
- [`R_RISCV_IRELATIVE`](#r_riscv_irelative)
- [`R_RISCV_JAL`](#r_riscv_jal)
- [`R_RISCV_JUMP_SLOT`](#r_riscv_jump_slot)
- [`R_RISCV_LO12_I`](#r_riscv_lo12_i)
- [`R_RISCV_LO12_S`](#r_riscv_lo12_s)
- [`R_RISCV_NONE`](#r_riscv_none)
- [`R_RISCV_PCREL_HI20`](#r_riscv_pcrel_hi20)
- [`R_RISCV_PCREL_LO12_I`](#r_riscv_pcrel_lo12_i)
- [`R_RISCV_PCREL_LO12_S`](#r_riscv_pcrel_lo12_s)
- [`R_RISCV_PLT32`](#r_riscv_plt32)
- [`R_RISCV_RELATIVE`](#r_riscv_relative)
- [`R_RISCV_RELAX`](#r_riscv_relax)
- [`R_RISCV_RVC_BRANCH`](#r_riscv_rvc_branch)
- [`R_RISCV_RVC_JUMP`](#r_riscv_rvc_jump)
- [`R_RISCV_RVC_LUI`](#r_riscv_rvc_lui)
- [`R_RISCV_SET16`](#r_riscv_set16)
- [`R_RISCV_SET32`](#r_riscv_set32)
- [`R_RISCV_SET6`](#r_riscv_set6)
- [`R_RISCV_SET8`](#r_riscv_set8)
- [`R_RISCV_SET_ULEB128`](#r_riscv_set_uleb128)
- [`R_RISCV_SUB16`](#r_riscv_sub16)
- [`R_RISCV_SUB32`](#r_riscv_sub32)
- [`R_RISCV_SUB6`](#r_riscv_sub6)
- [`R_RISCV_SUB64`](#r_riscv_sub64)
- [`R_RISCV_SUB8`](#r_riscv_sub8)
- [`R_RISCV_SUB_ULEB128`](#r_riscv_sub_uleb128)
- [`R_RISCV_TLSDESC`](#r_riscv_tlsdesc)
- [`R_RISCV_TLSDESC_ADD_LO12`](#r_riscv_tlsdesc_add_lo12)
- [`R_RISCV_TLSDESC_CALL`](#r_riscv_tlsdesc_call)
- [`R_RISCV_TLSDESC_HI20`](#r_riscv_tlsdesc_hi20)
- [`R_RISCV_TLSDESC_LOAD_LO12`](#r_riscv_tlsdesc_load_lo12)
- [`R_RISCV_TLS_DTPMOD32`](#r_riscv_tls_dtpmod32)
- [`R_RISCV_TLS_DTPMOD64`](#r_riscv_tls_dtpmod64)
- [`R_RISCV_TLS_DTPREL32`](#r_riscv_tls_dtprel32)
- [`R_RISCV_TLS_DTPREL64`](#r_riscv_tls_dtprel64)
- [`R_RISCV_TLS_GD_HI20`](#r_riscv_tls_gd_hi20)
- [`R_RISCV_TLS_GOT_HI20`](#r_riscv_tls_got_hi20)
- [`R_RISCV_TLS_TPREL32`](#r_riscv_tls_tprel32)
- [`R_RISCV_TLS_TPREL64`](#r_riscv_tls_tprel64)
- [`R_RISCV_TPREL_ADD`](#r_riscv_tprel_add)
- [`R_RISCV_TPREL_HI20`](#r_riscv_tprel_hi20)
- [`R_RISCV_TPREL_I`](#r_riscv_tprel_i)
- [`R_RISCV_TPREL_LO12_I`](#r_riscv_tprel_lo12_i)
- [`R_RISCV_TPREL_LO12_S`](#r_riscv_tprel_lo12_s)
- [`R_RISCV_TPREL_S`](#r_riscv_tprel_s)
- [`R_SBF_64_32`](#r_sbf_64_32)
- [`R_SBF_64_64`](#r_sbf_64_64)
- [`R_SBF_NONE`](#r_sbf_none) - No reloc
- [`R_SHARC_ADDR24_V3`](#r_sharc_addr24_v3) - 24-bit absolute address in bits 23:0 of a 48-bit instr
- [`R_SHARC_ADDR32_V3`](#r_sharc_addr32_v3) - 32-bit absolute address in bits 31:0 of a 48-bit instr
- [`R_SHARC_ADDR_VAR16_V3`](#r_sharc_addr_var16_v3) - 16-bit absolute address into bits 15:0 of a 16-bit location.
- [`R_SHARC_ADDR_VAR_V3`](#r_sharc_addr_var_v3) - 32-bit absolute address in bits 31:0 of a 32-bit data location
- [`R_SHARC_CALC_ADD`](#r_sharc_calc_add)
- [`R_SHARC_CALC_AND`](#r_sharc_calc_and)
- [`R_SHARC_CALC_DIV`](#r_sharc_calc_div)
- [`R_SHARC_CALC_LSHIFT`](#r_sharc_calc_lshift)
- [`R_SHARC_CALC_MOD`](#r_sharc_calc_mod)
- [`R_SHARC_CALC_MUL`](#r_sharc_calc_mul)
- [`R_SHARC_CALC_NOT`](#r_sharc_calc_not)
- [`R_SHARC_CALC_OR`](#r_sharc_calc_or)
- [`R_SHARC_CALC_PUSH_ADDEND`](#r_sharc_calc_push_addend)
- [`R_SHARC_CALC_PUSH_ADDR`](#r_sharc_calc_push_addr)
- [`R_SHARC_CALC_PUSH_LEN`](#r_sharc_calc_push_len)
- [`R_SHARC_CALC_RSHIFT`](#r_sharc_calc_rshift)
- [`R_SHARC_CALC_SUB`](#r_sharc_calc_sub)
- [`R_SHARC_CALC_XOR`](#r_sharc_calc_xor)
- [`R_SHARC_DATA16_V3`](#r_sharc_data16_v3) - 16-bit absolute address in bits 39:24 of a 48-bit instr
- [`R_SHARC_DATA16_VISA_V3`](#r_sharc_data16_visa_v3) - 16-bit absolute address into bits 15:0 of a 32-bit instr
- [`R_SHARC_DATA6_V3`](#r_sharc_data6_v3) - 6-bit absolute address in bits 32:27 of a 48-bit instr
- [`R_SHARC_DATA6_VISA_V3`](#r_sharc_data6_visa_v3) - 6-bit absolute address into bits 16:11 of a 32-bit instr
- [`R_SHARC_DATA7_VISA_V3`](#r_sharc_data7_visa_v3) - 7-bit absolute address into bits 6:0 of a 32-bit instr
- [`R_SHARC_PCR6_VISA_V3`](#r_sharc_pcr6_visa_v3) - 6-bit PC-relative address into bits 16:11 of a Type B
- [`R_SHARC_PCRLONG_V3`](#r_sharc_pcrlong_v3) - 24-bit PC-relative address in bits 23:0 of a 48-bit instr
- [`R_SHARC_PCRSHORT_V3`](#r_sharc_pcrshort_v3) - 6-bit PC-relative address in bits 32:27 of a 48-bit instr
- [`R_SH_ALIGN`](#r_sh_align)
- [`R_SH_CODE`](#r_sh_code)
- [`R_SH_COPY`](#r_sh_copy)
- [`R_SH_COUNT`](#r_sh_count)
- [`R_SH_DATA`](#r_sh_data)
- [`R_SH_DIR32`](#r_sh_dir32)
- [`R_SH_DIR8BP`](#r_sh_dir8bp)
- [`R_SH_DIR8L`](#r_sh_dir8l)
- [`R_SH_DIR8W`](#r_sh_dir8w)
- [`R_SH_DIR8WPL`](#r_sh_dir8wpl)
- [`R_SH_DIR8WPN`](#r_sh_dir8wpn)
- [`R_SH_DIR8WPZ`](#r_sh_dir8wpz)
- [`R_SH_GLOB_DAT`](#r_sh_glob_dat)
- [`R_SH_GNU_VTENTRY`](#r_sh_gnu_vtentry)
- [`R_SH_GNU_VTINHERIT`](#r_sh_gnu_vtinherit)
- [`R_SH_GOT32`](#r_sh_got32)
- [`R_SH_GOTOFF`](#r_sh_gotoff)
- [`R_SH_GOTPC`](#r_sh_gotpc)
- [`R_SH_IND12W`](#r_sh_ind12w)
- [`R_SH_JMP_SLOT`](#r_sh_jmp_slot)
- [`R_SH_LABEL`](#r_sh_label)
- [`R_SH_NONE`](#r_sh_none)
- [`R_SH_PLT32`](#r_sh_plt32)
- [`R_SH_REL32`](#r_sh_rel32)
- [`R_SH_RELATIVE`](#r_sh_relative)
- [`R_SH_SWITCH16`](#r_sh_switch16)
- [`R_SH_SWITCH32`](#r_sh_switch32)
- [`R_SH_SWITCH8`](#r_sh_switch8)
- [`R_SH_TLS_DTPMOD32`](#r_sh_tls_dtpmod32)
- [`R_SH_TLS_DTPOFF32`](#r_sh_tls_dtpoff32)
- [`R_SH_TLS_GD_32`](#r_sh_tls_gd_32)
- [`R_SH_TLS_IE_32`](#r_sh_tls_ie_32)
- [`R_SH_TLS_LDO_32`](#r_sh_tls_ldo_32)
- [`R_SH_TLS_LD_32`](#r_sh_tls_ld_32)
- [`R_SH_TLS_LE_32`](#r_sh_tls_le_32)
- [`R_SH_TLS_TPOFF32`](#r_sh_tls_tpoff32)
- [`R_SH_USES`](#r_sh_uses)
- [`R_SPARC_10`](#r_sparc_10) - Direct 10 bit
- [`R_SPARC_11`](#r_sparc_11) - Direct 11 bit
- [`R_SPARC_13`](#r_sparc_13) - Direct 13 bit
- [`R_SPARC_16`](#r_sparc_16) - Direct 16 bit
- [`R_SPARC_22`](#r_sparc_22) - Direct 22 bit
- [`R_SPARC_32`](#r_sparc_32) - Direct 32 bit
- [`R_SPARC_5`](#r_sparc_5) - Direct 5 bit
- [`R_SPARC_6`](#r_sparc_6) - Direct 6 bit
- [`R_SPARC_64`](#r_sparc_64) - Direct 64 bit
- [`R_SPARC_7`](#r_sparc_7) - Direct 7 bit
- [`R_SPARC_8`](#r_sparc_8) - Direct 8 bit
- [`R_SPARC_COPY`](#r_sparc_copy) - Copy symbol at runtime
- [`R_SPARC_DISP16`](#r_sparc_disp16) - PC relative 16 bit
- [`R_SPARC_DISP32`](#r_sparc_disp32) - PC relative 32 bit
- [`R_SPARC_DISP64`](#r_sparc_disp64) - PC relative 64 bit
- [`R_SPARC_DISP8`](#r_sparc_disp8) - PC relative 8 bit
- [`R_SPARC_GLOB_DAT`](#r_sparc_glob_dat) - Create GOT entry
- [`R_SPARC_GLOB_JMP`](#r_sparc_glob_jmp) - was part of v9 ABI but was removed
- [`R_SPARC_GNU_VTENTRY`](#r_sparc_gnu_vtentry)
- [`R_SPARC_GNU_VTINHERIT`](#r_sparc_gnu_vtinherit)
- [`R_SPARC_GOT10`](#r_sparc_got10) - Truncated 10 bit GOT entry
- [`R_SPARC_GOT13`](#r_sparc_got13) - 13 bit GOT entry
- [`R_SPARC_GOT22`](#r_sparc_got22) - 22 bit GOT entry shifted
- [`R_SPARC_GOTDATA_HIX22`](#r_sparc_gotdata_hix22)
- [`R_SPARC_GOTDATA_LOX10`](#r_sparc_gotdata_lox10)
- [`R_SPARC_GOTDATA_OP`](#r_sparc_gotdata_op)
- [`R_SPARC_GOTDATA_OP_HIX22`](#r_sparc_gotdata_op_hix22)
- [`R_SPARC_GOTDATA_OP_LOX10`](#r_sparc_gotdata_op_lox10)
- [`R_SPARC_H34`](#r_sparc_h34)
- [`R_SPARC_H44`](#r_sparc_h44) - Direct high 12 of 44 bit
- [`R_SPARC_HH22`](#r_sparc_hh22) - Top 22 bits of direct 64 bit
- [`R_SPARC_HI22`](#r_sparc_hi22) - High 22 bit
- [`R_SPARC_HIPLT22`](#r_sparc_hiplt22) - High 22 bit PLT entry
- [`R_SPARC_HIX22`](#r_sparc_hix22) - High 22 bit complemented
- [`R_SPARC_HM10`](#r_sparc_hm10) - High middle 10 bits of ...
- [`R_SPARC_IRELATIVE`](#r_sparc_irelative)
- [`R_SPARC_JMP_IREL`](#r_sparc_jmp_irel)
- [`R_SPARC_JMP_SLOT`](#r_sparc_jmp_slot) - Create PLT entry
- [`R_SPARC_L44`](#r_sparc_l44) - Direct low 10 of 44 bit
- [`R_SPARC_LM22`](#r_sparc_lm22) - Low middle 22 bits of ...
- [`R_SPARC_LO10`](#r_sparc_lo10) - Truncated 10 bit
- [`R_SPARC_LOPLT10`](#r_sparc_loplt10) - Truncated 10 bit PLT entry
- [`R_SPARC_LOX10`](#r_sparc_lox10) - Truncated 11 bit complemented
- [`R_SPARC_M44`](#r_sparc_m44) - Direct mid 22 of 44 bit
- [`R_SPARC_NONE`](#r_sparc_none) - No reloc
- [`R_SPARC_OLO10`](#r_sparc_olo10) - 10bit with secondary 13bit addend
- [`R_SPARC_PC10`](#r_sparc_pc10) - PC relative 10 bit truncated
- [`R_SPARC_PC22`](#r_sparc_pc22) - PC relative 22 bit shifted
- [`R_SPARC_PCPLT10`](#r_sparc_pcplt10) - PC rel trunc 10 bit PLT entry
- [`R_SPARC_PCPLT22`](#r_sparc_pcplt22) - PC rel high 22 bit PLT entry
- [`R_SPARC_PCPLT32`](#r_sparc_pcplt32) - PC rel 32 bit ref to PLT entry
- [`R_SPARC_PC_HH22`](#r_sparc_pc_hh22) - Top 22 bits of pc rel 64 bit
- [`R_SPARC_PC_HM10`](#r_sparc_pc_hm10) - High middle 10 bit of ...
- [`R_SPARC_PC_LM22`](#r_sparc_pc_lm22) - Low miggle 22 bits of ...
- [`R_SPARC_PLT32`](#r_sparc_plt32) - Direct 32 bit ref to PLT entry
- [`R_SPARC_PLT64`](#r_sparc_plt64) - Direct 64 bit ref to PLT entry
- [`R_SPARC_REGISTER`](#r_sparc_register) - Global register usage
- [`R_SPARC_RELATIVE`](#r_sparc_relative) - Adjust by program base
- [`R_SPARC_REV32`](#r_sparc_rev32)
- [`R_SPARC_SIZE32`](#r_sparc_size32)
- [`R_SPARC_SIZE64`](#r_sparc_size64)
- [`R_SPARC_TLS_DTPMOD32`](#r_sparc_tls_dtpmod32)
- [`R_SPARC_TLS_DTPMOD64`](#r_sparc_tls_dtpmod64)
- [`R_SPARC_TLS_DTPOFF32`](#r_sparc_tls_dtpoff32)
- [`R_SPARC_TLS_DTPOFF64`](#r_sparc_tls_dtpoff64)
- [`R_SPARC_TLS_GD_ADD`](#r_sparc_tls_gd_add)
- [`R_SPARC_TLS_GD_CALL`](#r_sparc_tls_gd_call)
- [`R_SPARC_TLS_GD_HI22`](#r_sparc_tls_gd_hi22)
- [`R_SPARC_TLS_GD_LO10`](#r_sparc_tls_gd_lo10)
- [`R_SPARC_TLS_IE_ADD`](#r_sparc_tls_ie_add)
- [`R_SPARC_TLS_IE_HI22`](#r_sparc_tls_ie_hi22)
- [`R_SPARC_TLS_IE_LD`](#r_sparc_tls_ie_ld)
- [`R_SPARC_TLS_IE_LDX`](#r_sparc_tls_ie_ldx)
- [`R_SPARC_TLS_IE_LO10`](#r_sparc_tls_ie_lo10)
- [`R_SPARC_TLS_LDM_ADD`](#r_sparc_tls_ldm_add)
- [`R_SPARC_TLS_LDM_CALL`](#r_sparc_tls_ldm_call)
- [`R_SPARC_TLS_LDM_HI22`](#r_sparc_tls_ldm_hi22)
- [`R_SPARC_TLS_LDM_LO10`](#r_sparc_tls_ldm_lo10)
- [`R_SPARC_TLS_LDO_ADD`](#r_sparc_tls_ldo_add)
- [`R_SPARC_TLS_LDO_HIX22`](#r_sparc_tls_ldo_hix22)
- [`R_SPARC_TLS_LDO_LOX10`](#r_sparc_tls_ldo_lox10)
- [`R_SPARC_TLS_LE_HIX22`](#r_sparc_tls_le_hix22)
- [`R_SPARC_TLS_LE_LOX10`](#r_sparc_tls_le_lox10)
- [`R_SPARC_TLS_TPOFF32`](#r_sparc_tls_tpoff32)
- [`R_SPARC_TLS_TPOFF64`](#r_sparc_tls_tpoff64)
- [`R_SPARC_UA16`](#r_sparc_ua16) - Direct 16 bit unaligned
- [`R_SPARC_UA32`](#r_sparc_ua32) - Direct 32 bit unaligned
- [`R_SPARC_UA64`](#r_sparc_ua64) - Direct 64 bit unaligned
- [`R_SPARC_WDISP10`](#r_sparc_wdisp10)
- [`R_SPARC_WDISP16`](#r_sparc_wdisp16) - PC relative 16 bit shifted
- [`R_SPARC_WDISP19`](#r_sparc_wdisp19) - PC relative 19 bit shifted
- [`R_SPARC_WDISP22`](#r_sparc_wdisp22) - PC relative 22 bit shifted
- [`R_SPARC_WDISP30`](#r_sparc_wdisp30) - PC relative 30 bit shifted
- [`R_SPARC_WPLT30`](#r_sparc_wplt30) - 30 bit PC relative PLT address
- [`R_TILEGX_16`](#r_tilegx_16) - Direct 16 bit
- [`R_TILEGX_16_PCREL`](#r_tilegx_16_pcrel) - PC relative 16 bit
- [`R_TILEGX_32`](#r_tilegx_32) - Direct 32 bit
- [`R_TILEGX_32_PCREL`](#r_tilegx_32_pcrel) - PC relative 32 bit
- [`R_TILEGX_64`](#r_tilegx_64) - Direct 64 bit
- [`R_TILEGX_64_PCREL`](#r_tilegx_64_pcrel) - PC relative 64 bit
- [`R_TILEGX_8`](#r_tilegx_8) - Direct 8 bit
- [`R_TILEGX_8_PCREL`](#r_tilegx_8_pcrel) - PC relative 8 bit
- [`R_TILEGX_BROFF_X1`](#r_tilegx_broff_x1) - X1 pipe branch offset
- [`R_TILEGX_COPY`](#r_tilegx_copy) - Copy relocation
- [`R_TILEGX_DEST_IMM8_X1`](#r_tilegx_dest_imm8_x1) - X1 pipe destination 8-bit
- [`R_TILEGX_GLOB_DAT`](#r_tilegx_glob_dat) - Create GOT entry
- [`R_TILEGX_GNU_VTENTRY`](#r_tilegx_gnu_vtentry) - GNU C++ vtable member usage
- [`R_TILEGX_GNU_VTINHERIT`](#r_tilegx_gnu_vtinherit) - GNU C++ vtable hierarchy
- [`R_TILEGX_HW0`](#r_tilegx_hw0) - hword 0 16-bit
- [`R_TILEGX_HW0_LAST`](#r_tilegx_hw0_last) - last hword 0 16-bit
- [`R_TILEGX_HW1`](#r_tilegx_hw1) - hword 1 16-bit
- [`R_TILEGX_HW1_LAST`](#r_tilegx_hw1_last) - last hword 1 16-bit
- [`R_TILEGX_HW2`](#r_tilegx_hw2) - hword 2 16-bit
- [`R_TILEGX_HW2_LAST`](#r_tilegx_hw2_last) - last hword 2 16-bit
- [`R_TILEGX_HW3`](#r_tilegx_hw3) - hword 3 16-bit
- [`R_TILEGX_IMM16_X0_HW0`](#r_tilegx_imm16_x0_hw0) - X0 pipe hword 0
- [`R_TILEGX_IMM16_X0_HW0_GOT`](#r_tilegx_imm16_x0_hw0_got) - X0 pipe hword 0 GOT offset
- [`R_TILEGX_IMM16_X0_HW0_LAST`](#r_tilegx_imm16_x0_hw0_last) - X0 pipe last hword 0
- [`R_TILEGX_IMM16_X0_HW0_LAST_GOT`](#r_tilegx_imm16_x0_hw0_last_got) - X0 pipe last hword 0 GOT offset
- [`R_TILEGX_IMM16_X0_HW0_LAST_PCREL`](#r_tilegx_imm16_x0_hw0_last_pcrel) - X0 pipe PC-rel last hword 0
- [`R_TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL`](#r_tilegx_imm16_x0_hw0_last_plt_pcrel) - X0 pipe PC-rel PLT last hword 0
- [`R_TILEGX_IMM16_X0_HW0_LAST_TLS_GD`](#r_tilegx_imm16_x0_hw0_last_tls_gd) - X0 pipe last hword 0 GD off
- [`R_TILEGX_IMM16_X0_HW0_LAST_TLS_IE`](#r_tilegx_imm16_x0_hw0_last_tls_ie) - X0 pipe last hword 0 IE off
- [`R_TILEGX_IMM16_X0_HW0_LAST_TLS_LE`](#r_tilegx_imm16_x0_hw0_last_tls_le) - X0 pipe last hword 0 LE off
- [`R_TILEGX_IMM16_X0_HW0_PCREL`](#r_tilegx_imm16_x0_hw0_pcrel) - X0 pipe PC relative hword 0
- [`R_TILEGX_IMM16_X0_HW0_PLT_PCREL`](#r_tilegx_imm16_x0_hw0_plt_pcrel) - X0 pipe PC-rel PLT hword 0
- [`R_TILEGX_IMM16_X0_HW0_TLS_GD`](#r_tilegx_imm16_x0_hw0_tls_gd) - X0 pipe hword 0 TLS GD offset
- [`R_TILEGX_IMM16_X0_HW0_TLS_IE`](#r_tilegx_imm16_x0_hw0_tls_ie) - X0 pipe hword 0 TLS IE offset
- [`R_TILEGX_IMM16_X0_HW0_TLS_LE`](#r_tilegx_imm16_x0_hw0_tls_le) - X0 pipe hword 0 TLS LE offset
- [`R_TILEGX_IMM16_X0_HW1`](#r_tilegx_imm16_x0_hw1) - X0 pipe hword 1
- [`R_TILEGX_IMM16_X0_HW1_LAST`](#r_tilegx_imm16_x0_hw1_last) - X0 pipe last hword 1
- [`R_TILEGX_IMM16_X0_HW1_LAST_GOT`](#r_tilegx_imm16_x0_hw1_last_got) - X0 pipe last hword 1 GOT offset
- [`R_TILEGX_IMM16_X0_HW1_LAST_PCREL`](#r_tilegx_imm16_x0_hw1_last_pcrel) - X0 pipe PC-rel last hword 1
- [`R_TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL`](#r_tilegx_imm16_x0_hw1_last_plt_pcrel) - X0 pipe PC-rel PLT last hword 1
- [`R_TILEGX_IMM16_X0_HW1_LAST_TLS_GD`](#r_tilegx_imm16_x0_hw1_last_tls_gd) - X0 pipe last hword 1 GD off
- [`R_TILEGX_IMM16_X0_HW1_LAST_TLS_IE`](#r_tilegx_imm16_x0_hw1_last_tls_ie) - X0 pipe last hword 1 IE off
- [`R_TILEGX_IMM16_X0_HW1_LAST_TLS_LE`](#r_tilegx_imm16_x0_hw1_last_tls_le) - X0 pipe last hword 1 LE off
- [`R_TILEGX_IMM16_X0_HW1_PCREL`](#r_tilegx_imm16_x0_hw1_pcrel) - X0 pipe PC relative hword 1
- [`R_TILEGX_IMM16_X0_HW1_PLT_PCREL`](#r_tilegx_imm16_x0_hw1_plt_pcrel) - X0 pipe PC-rel PLT hword 1
- [`R_TILEGX_IMM16_X0_HW2`](#r_tilegx_imm16_x0_hw2) - X0 pipe hword 2
- [`R_TILEGX_IMM16_X0_HW2_LAST`](#r_tilegx_imm16_x0_hw2_last) - X0 pipe last hword 2
- [`R_TILEGX_IMM16_X0_HW2_LAST_PCREL`](#r_tilegx_imm16_x0_hw2_last_pcrel) - X0 pipe PC-rel last hword 2
- [`R_TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL`](#r_tilegx_imm16_x0_hw2_last_plt_pcrel) - X0 pipe PC-rel PLT last hword 2
- [`R_TILEGX_IMM16_X0_HW2_PCREL`](#r_tilegx_imm16_x0_hw2_pcrel) - X0 pipe PC relative hword 2
- [`R_TILEGX_IMM16_X0_HW2_PLT_PCREL`](#r_tilegx_imm16_x0_hw2_plt_pcrel) - X0 pipe PC-rel PLT hword 2
- [`R_TILEGX_IMM16_X0_HW3`](#r_tilegx_imm16_x0_hw3) - X0 pipe hword 3
- [`R_TILEGX_IMM16_X0_HW3_PCREL`](#r_tilegx_imm16_x0_hw3_pcrel) - X0 pipe PC relative hword 3
- [`R_TILEGX_IMM16_X0_HW3_PLT_PCREL`](#r_tilegx_imm16_x0_hw3_plt_pcrel) - X0 pipe PC-rel PLT hword 3
- [`R_TILEGX_IMM16_X1_HW0`](#r_tilegx_imm16_x1_hw0) - X1 pipe hword 0
- [`R_TILEGX_IMM16_X1_HW0_GOT`](#r_tilegx_imm16_x1_hw0_got) - X1 pipe hword 0 GOT offset
- [`R_TILEGX_IMM16_X1_HW0_LAST`](#r_tilegx_imm16_x1_hw0_last) - X1 pipe last hword 0
- [`R_TILEGX_IMM16_X1_HW0_LAST_GOT`](#r_tilegx_imm16_x1_hw0_last_got) - X1 pipe last hword 0 GOT offset
- [`R_TILEGX_IMM16_X1_HW0_LAST_PCREL`](#r_tilegx_imm16_x1_hw0_last_pcrel) - X1 pipe PC-rel last hword 0
- [`R_TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL`](#r_tilegx_imm16_x1_hw0_last_plt_pcrel) - X1 pipe PC-rel PLT last hword 0
- [`R_TILEGX_IMM16_X1_HW0_LAST_TLS_GD`](#r_tilegx_imm16_x1_hw0_last_tls_gd) - X1 pipe last hword 0 GD off
- [`R_TILEGX_IMM16_X1_HW0_LAST_TLS_IE`](#r_tilegx_imm16_x1_hw0_last_tls_ie) - X1 pipe last hword 0 IE off
- [`R_TILEGX_IMM16_X1_HW0_LAST_TLS_LE`](#r_tilegx_imm16_x1_hw0_last_tls_le) - X1 pipe last hword 0 LE off
- [`R_TILEGX_IMM16_X1_HW0_PCREL`](#r_tilegx_imm16_x1_hw0_pcrel) - X1 pipe PC relative hword 0
- [`R_TILEGX_IMM16_X1_HW0_PLT_PCREL`](#r_tilegx_imm16_x1_hw0_plt_pcrel) - X1 pipe PC-rel PLT hword 0
- [`R_TILEGX_IMM16_X1_HW0_TLS_GD`](#r_tilegx_imm16_x1_hw0_tls_gd) - X1 pipe hword 0 TLS GD offset
- [`R_TILEGX_IMM16_X1_HW0_TLS_IE`](#r_tilegx_imm16_x1_hw0_tls_ie) - X1 pipe hword 0 TLS IE offset
- [`R_TILEGX_IMM16_X1_HW0_TLS_LE`](#r_tilegx_imm16_x1_hw0_tls_le) - X1 pipe hword 0 TLS LE offset
- [`R_TILEGX_IMM16_X1_HW1`](#r_tilegx_imm16_x1_hw1) - X1 pipe hword 1
- [`R_TILEGX_IMM16_X1_HW1_LAST`](#r_tilegx_imm16_x1_hw1_last) - X1 pipe last hword 1
- [`R_TILEGX_IMM16_X1_HW1_LAST_GOT`](#r_tilegx_imm16_x1_hw1_last_got) - X1 pipe last hword 1 GOT offset
- [`R_TILEGX_IMM16_X1_HW1_LAST_PCREL`](#r_tilegx_imm16_x1_hw1_last_pcrel) - X1 pipe PC-rel last hword 1
- [`R_TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL`](#r_tilegx_imm16_x1_hw1_last_plt_pcrel) - X1 pipe PC-rel PLT last hword 1
- [`R_TILEGX_IMM16_X1_HW1_LAST_TLS_GD`](#r_tilegx_imm16_x1_hw1_last_tls_gd) - X1 pipe last hword 1 GD off
- [`R_TILEGX_IMM16_X1_HW1_LAST_TLS_IE`](#r_tilegx_imm16_x1_hw1_last_tls_ie) - X1 pipe last hword 1 IE off
- [`R_TILEGX_IMM16_X1_HW1_LAST_TLS_LE`](#r_tilegx_imm16_x1_hw1_last_tls_le) - X1 pipe last hword 1 LE off
- [`R_TILEGX_IMM16_X1_HW1_PCREL`](#r_tilegx_imm16_x1_hw1_pcrel) - X1 pipe PC relative hword 1
- [`R_TILEGX_IMM16_X1_HW1_PLT_PCREL`](#r_tilegx_imm16_x1_hw1_plt_pcrel) - X1 pipe PC-rel PLT hword 1
- [`R_TILEGX_IMM16_X1_HW2`](#r_tilegx_imm16_x1_hw2) - X1 pipe hword 2
- [`R_TILEGX_IMM16_X1_HW2_LAST`](#r_tilegx_imm16_x1_hw2_last) - X1 pipe last hword 2
- [`R_TILEGX_IMM16_X1_HW2_LAST_PCREL`](#r_tilegx_imm16_x1_hw2_last_pcrel) - X1 pipe PC-rel last hword 2
- [`R_TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL`](#r_tilegx_imm16_x1_hw2_last_plt_pcrel) - X1 pipe PC-rel PLT last hword 2
- [`R_TILEGX_IMM16_X1_HW2_PCREL`](#r_tilegx_imm16_x1_hw2_pcrel) - X1 pipe PC relative hword 2
- [`R_TILEGX_IMM16_X1_HW2_PLT_PCREL`](#r_tilegx_imm16_x1_hw2_plt_pcrel) - X1 pipe PC-rel PLT hword 2
- [`R_TILEGX_IMM16_X1_HW3`](#r_tilegx_imm16_x1_hw3) - X1 pipe hword 3
- [`R_TILEGX_IMM16_X1_HW3_PCREL`](#r_tilegx_imm16_x1_hw3_pcrel) - X1 pipe PC relative hword 3
- [`R_TILEGX_IMM16_X1_HW3_PLT_PCREL`](#r_tilegx_imm16_x1_hw3_plt_pcrel) - X1 pipe PC-rel PLT hword 3
- [`R_TILEGX_IMM8_X0`](#r_tilegx_imm8_x0) - X0 pipe 8-bit
- [`R_TILEGX_IMM8_X0_TLS_ADD`](#r_tilegx_imm8_x0_tls_add) - X0 pipe "addi" for TLS GD/IE
- [`R_TILEGX_IMM8_X0_TLS_GD_ADD`](#r_tilegx_imm8_x0_tls_gd_add) - X0 pipe "addi" for TLS GD
- [`R_TILEGX_IMM8_X1`](#r_tilegx_imm8_x1) - X1 pipe 8-bit
- [`R_TILEGX_IMM8_X1_TLS_ADD`](#r_tilegx_imm8_x1_tls_add) - X1 pipe "addi" for TLS GD/IE
- [`R_TILEGX_IMM8_X1_TLS_GD_ADD`](#r_tilegx_imm8_x1_tls_gd_add) - X1 pipe "addi" for TLS GD
- [`R_TILEGX_IMM8_Y0`](#r_tilegx_imm8_y0) - Y0 pipe 8-bit
- [`R_TILEGX_IMM8_Y0_TLS_ADD`](#r_tilegx_imm8_y0_tls_add) - Y0 pipe "addi" for TLS GD/IE
- [`R_TILEGX_IMM8_Y0_TLS_GD_ADD`](#r_tilegx_imm8_y0_tls_gd_add) - Y0 pipe "addi" for TLS GD
- [`R_TILEGX_IMM8_Y1`](#r_tilegx_imm8_y1) - Y1 pipe 8-bit
- [`R_TILEGX_IMM8_Y1_TLS_ADD`](#r_tilegx_imm8_y1_tls_add) - Y1 pipe "addi" for TLS GD/IE
- [`R_TILEGX_IMM8_Y1_TLS_GD_ADD`](#r_tilegx_imm8_y1_tls_gd_add) - Y1 pipe "addi" for TLS GD
- [`R_TILEGX_JMP_SLOT`](#r_tilegx_jmp_slot) - Create PLT entry
- [`R_TILEGX_JUMPOFF_X1`](#r_tilegx_jumpoff_x1) - X1 pipe jump offset
- [`R_TILEGX_JUMPOFF_X1_PLT`](#r_tilegx_jumpoff_x1_plt) - X1 pipe jump offset to PLT
- [`R_TILEGX_MF_IMM14_X1`](#r_tilegx_mf_imm14_x1) - X1 pipe mfspr
- [`R_TILEGX_MMEND_X0`](#r_tilegx_mmend_x0) - X0 pipe mm "end"
- [`R_TILEGX_MMSTART_X0`](#r_tilegx_mmstart_x0) - X0 pipe mm "start"
- [`R_TILEGX_MT_IMM14_X1`](#r_tilegx_mt_imm14_x1) - X1 pipe mtspr
- [`R_TILEGX_NONE`](#r_tilegx_none) - No reloc
- [`R_TILEGX_RELATIVE`](#r_tilegx_relative) - Adjust by program base
- [`R_TILEGX_SHAMT_X0`](#r_tilegx_shamt_x0) - X0 pipe shift amount
- [`R_TILEGX_SHAMT_X1`](#r_tilegx_shamt_x1) - X1 pipe shift amount
- [`R_TILEGX_SHAMT_Y0`](#r_tilegx_shamt_y0) - Y0 pipe shift amount
- [`R_TILEGX_SHAMT_Y1`](#r_tilegx_shamt_y1) - Y1 pipe shift amount
- [`R_TILEGX_TLS_DTPMOD32`](#r_tilegx_tls_dtpmod32) - 32-bit ID of symbol's module
- [`R_TILEGX_TLS_DTPMOD64`](#r_tilegx_tls_dtpmod64) - 64-bit ID of symbol's module
- [`R_TILEGX_TLS_DTPOFF32`](#r_tilegx_tls_dtpoff32) - 32-bit offset in TLS block
- [`R_TILEGX_TLS_DTPOFF64`](#r_tilegx_tls_dtpoff64) - 64-bit offset in TLS block
- [`R_TILEGX_TLS_GD_CALL`](#r_tilegx_tls_gd_call) - "jal" for TLS GD
- [`R_TILEGX_TLS_IE_LOAD`](#r_tilegx_tls_ie_load) - "ld_tls" for TLS IE
- [`R_TILEGX_TLS_TPOFF32`](#r_tilegx_tls_tpoff32) - 32-bit offset in static TLS block
- [`R_TILEGX_TLS_TPOFF64`](#r_tilegx_tls_tpoff64) - 64-bit offset in static TLS block
- [`R_TILEPRO_16`](#r_tilepro_16) - Direct 16 bit
- [`R_TILEPRO_16_PCREL`](#r_tilepro_16_pcrel) - PC relative 16 bit
- [`R_TILEPRO_32`](#r_tilepro_32) - Direct 32 bit
- [`R_TILEPRO_32_PCREL`](#r_tilepro_32_pcrel) - PC relative 32 bit
- [`R_TILEPRO_8`](#r_tilepro_8) - Direct 8 bit
- [`R_TILEPRO_8_PCREL`](#r_tilepro_8_pcrel) - PC relative 8 bit
- [`R_TILEPRO_BROFF_X1`](#r_tilepro_broff_x1) - X1 pipe branch offset
- [`R_TILEPRO_COPY`](#r_tilepro_copy) - Copy relocation
- [`R_TILEPRO_DEST_IMM8_X1`](#r_tilepro_dest_imm8_x1) - X1 pipe destination 8-bit
- [`R_TILEPRO_GLOB_DAT`](#r_tilepro_glob_dat) - Create GOT entry
- [`R_TILEPRO_GNU_VTENTRY`](#r_tilepro_gnu_vtentry) - GNU C++ vtable member usage
- [`R_TILEPRO_GNU_VTINHERIT`](#r_tilepro_gnu_vtinherit) - GNU C++ vtable hierarchy
- [`R_TILEPRO_HA16`](#r_tilepro_ha16) - High 16 bit, adjusted
- [`R_TILEPRO_HI16`](#r_tilepro_hi16) - High 16 bit
- [`R_TILEPRO_IMM16_X0`](#r_tilepro_imm16_x0) - X0 pipe 16-bit
- [`R_TILEPRO_IMM16_X0_GOT`](#r_tilepro_imm16_x0_got) - X0 pipe 16-bit GOT offset
- [`R_TILEPRO_IMM16_X0_GOT_HA`](#r_tilepro_imm16_x0_got_ha) - X0 pipe ha() 16-bit GOT offset
- [`R_TILEPRO_IMM16_X0_GOT_HI`](#r_tilepro_imm16_x0_got_hi) - X0 pipe high 16-bit GOT offset
- [`R_TILEPRO_IMM16_X0_GOT_LO`](#r_tilepro_imm16_x0_got_lo) - X0 pipe low 16-bit GOT offset
- [`R_TILEPRO_IMM16_X0_HA`](#r_tilepro_imm16_x0_ha) - X0 pipe high 16-bit, adjusted
- [`R_TILEPRO_IMM16_X0_HA_PCREL`](#r_tilepro_imm16_x0_ha_pcrel) - X0 pipe PC relative ha() 16 bit
- [`R_TILEPRO_IMM16_X0_HI`](#r_tilepro_imm16_x0_hi) - X0 pipe high 16-bit
- [`R_TILEPRO_IMM16_X0_HI_PCREL`](#r_tilepro_imm16_x0_hi_pcrel) - X0 pipe PC relative high 16 bit
- [`R_TILEPRO_IMM16_X0_LO`](#r_tilepro_imm16_x0_lo) - X0 pipe low 16-bit
- [`R_TILEPRO_IMM16_X0_LO_PCREL`](#r_tilepro_imm16_x0_lo_pcrel) - X0 pipe PC relative low 16 bit
- [`R_TILEPRO_IMM16_X0_PCREL`](#r_tilepro_imm16_x0_pcrel) - X0 pipe PC relative 16 bit
- [`R_TILEPRO_IMM16_X0_TLS_GD`](#r_tilepro_imm16_x0_tls_gd) - X0 pipe 16-bit TLS GD offset
- [`R_TILEPRO_IMM16_X0_TLS_GD_HA`](#r_tilepro_imm16_x0_tls_gd_ha) - X0 pipe ha() 16-bit TLS GD offset
- [`R_TILEPRO_IMM16_X0_TLS_GD_HI`](#r_tilepro_imm16_x0_tls_gd_hi) - X0 pipe high 16-bit TLS GD offset
- [`R_TILEPRO_IMM16_X0_TLS_GD_LO`](#r_tilepro_imm16_x0_tls_gd_lo) - X0 pipe low 16-bit TLS GD offset
- [`R_TILEPRO_IMM16_X0_TLS_IE`](#r_tilepro_imm16_x0_tls_ie) - X0 pipe 16-bit TLS IE offset
- [`R_TILEPRO_IMM16_X0_TLS_IE_HA`](#r_tilepro_imm16_x0_tls_ie_ha) - X0 pipe ha() 16-bit TLS IE offset
- [`R_TILEPRO_IMM16_X0_TLS_IE_HI`](#r_tilepro_imm16_x0_tls_ie_hi) - X0 pipe high 16-bit TLS IE offset
- [`R_TILEPRO_IMM16_X0_TLS_IE_LO`](#r_tilepro_imm16_x0_tls_ie_lo) - X0 pipe low 16-bit TLS IE offset
- [`R_TILEPRO_IMM16_X0_TLS_LE`](#r_tilepro_imm16_x0_tls_le) - X0 pipe 16-bit TLS LE offset
- [`R_TILEPRO_IMM16_X0_TLS_LE_HA`](#r_tilepro_imm16_x0_tls_le_ha) - X0 pipe ha() 16-bit TLS LE offset
- [`R_TILEPRO_IMM16_X0_TLS_LE_HI`](#r_tilepro_imm16_x0_tls_le_hi) - X0 pipe high 16-bit TLS LE offset
- [`R_TILEPRO_IMM16_X0_TLS_LE_LO`](#r_tilepro_imm16_x0_tls_le_lo) - X0 pipe low 16-bit TLS LE offset
- [`R_TILEPRO_IMM16_X1`](#r_tilepro_imm16_x1) - X1 pipe 16-bit
- [`R_TILEPRO_IMM16_X1_GOT`](#r_tilepro_imm16_x1_got) - X1 pipe 16-bit GOT offset
- [`R_TILEPRO_IMM16_X1_GOT_HA`](#r_tilepro_imm16_x1_got_ha) - X1 pipe ha() 16-bit GOT offset
- [`R_TILEPRO_IMM16_X1_GOT_HI`](#r_tilepro_imm16_x1_got_hi) - X1 pipe high 16-bit GOT offset
- [`R_TILEPRO_IMM16_X1_GOT_LO`](#r_tilepro_imm16_x1_got_lo) - X1 pipe low 16-bit GOT offset
- [`R_TILEPRO_IMM16_X1_HA`](#r_tilepro_imm16_x1_ha) - X1 pipe high 16-bit, adjusted
- [`R_TILEPRO_IMM16_X1_HA_PCREL`](#r_tilepro_imm16_x1_ha_pcrel) - X1 pipe PC relative ha() 16 bit
- [`R_TILEPRO_IMM16_X1_HI`](#r_tilepro_imm16_x1_hi) - X1 pipe high 16-bit
- [`R_TILEPRO_IMM16_X1_HI_PCREL`](#r_tilepro_imm16_x1_hi_pcrel) - X1 pipe PC relative high 16 bit
- [`R_TILEPRO_IMM16_X1_LO`](#r_tilepro_imm16_x1_lo) - X1 pipe low 16-bit
- [`R_TILEPRO_IMM16_X1_LO_PCREL`](#r_tilepro_imm16_x1_lo_pcrel) - X1 pipe PC relative low 16 bit
- [`R_TILEPRO_IMM16_X1_PCREL`](#r_tilepro_imm16_x1_pcrel) - X1 pipe PC relative 16 bit
- [`R_TILEPRO_IMM16_X1_TLS_GD`](#r_tilepro_imm16_x1_tls_gd) - X1 pipe 16-bit TLS GD offset
- [`R_TILEPRO_IMM16_X1_TLS_GD_HA`](#r_tilepro_imm16_x1_tls_gd_ha) - X1 pipe ha() 16-bit TLS GD offset
- [`R_TILEPRO_IMM16_X1_TLS_GD_HI`](#r_tilepro_imm16_x1_tls_gd_hi) - X1 pipe high 16-bit TLS GD offset
- [`R_TILEPRO_IMM16_X1_TLS_GD_LO`](#r_tilepro_imm16_x1_tls_gd_lo) - X1 pipe low 16-bit TLS GD offset
- [`R_TILEPRO_IMM16_X1_TLS_IE`](#r_tilepro_imm16_x1_tls_ie) - X1 pipe 16-bit TLS IE offset
- [`R_TILEPRO_IMM16_X1_TLS_IE_HA`](#r_tilepro_imm16_x1_tls_ie_ha) - X1 pipe ha() 16-bit TLS IE offset
- [`R_TILEPRO_IMM16_X1_TLS_IE_HI`](#r_tilepro_imm16_x1_tls_ie_hi) - X1 pipe high 16-bit TLS IE offset
- [`R_TILEPRO_IMM16_X1_TLS_IE_LO`](#r_tilepro_imm16_x1_tls_ie_lo) - X1 pipe low 16-bit TLS IE offset
- [`R_TILEPRO_IMM16_X1_TLS_LE`](#r_tilepro_imm16_x1_tls_le) - X1 pipe 16-bit TLS LE offset
- [`R_TILEPRO_IMM16_X1_TLS_LE_HA`](#r_tilepro_imm16_x1_tls_le_ha) - X1 pipe ha() 16-bit TLS LE offset
- [`R_TILEPRO_IMM16_X1_TLS_LE_HI`](#r_tilepro_imm16_x1_tls_le_hi) - X1 pipe high 16-bit TLS LE offset
- [`R_TILEPRO_IMM16_X1_TLS_LE_LO`](#r_tilepro_imm16_x1_tls_le_lo) - X1 pipe low 16-bit TLS LE offset
- [`R_TILEPRO_IMM8_X0`](#r_tilepro_imm8_x0) - X0 pipe 8-bit
- [`R_TILEPRO_IMM8_X0_TLS_GD_ADD`](#r_tilepro_imm8_x0_tls_gd_add) - X0 pipe "addi" for TLS GD
- [`R_TILEPRO_IMM8_X1`](#r_tilepro_imm8_x1) - X1 pipe 8-bit
- [`R_TILEPRO_IMM8_X1_TLS_GD_ADD`](#r_tilepro_imm8_x1_tls_gd_add) - X1 pipe "addi" for TLS GD
- [`R_TILEPRO_IMM8_Y0`](#r_tilepro_imm8_y0) - Y0 pipe 8-bit
- [`R_TILEPRO_IMM8_Y0_TLS_GD_ADD`](#r_tilepro_imm8_y0_tls_gd_add) - Y0 pipe "addi" for TLS GD
- [`R_TILEPRO_IMM8_Y1`](#r_tilepro_imm8_y1) - Y1 pipe 8-bit
- [`R_TILEPRO_IMM8_Y1_TLS_GD_ADD`](#r_tilepro_imm8_y1_tls_gd_add) - Y1 pipe "addi" for TLS GD
- [`R_TILEPRO_JMP_SLOT`](#r_tilepro_jmp_slot) - Create PLT entry
- [`R_TILEPRO_JOFFLONG_X1`](#r_tilepro_jofflong_x1) - X1 pipe jump offset
- [`R_TILEPRO_JOFFLONG_X1_PLT`](#r_tilepro_jofflong_x1_plt) - X1 pipe jump offset to PLT
- [`R_TILEPRO_LO16`](#r_tilepro_lo16) - Low 16 bit
- [`R_TILEPRO_MF_IMM15_X1`](#r_tilepro_mf_imm15_x1) - X1 pipe mfspr
- [`R_TILEPRO_MMEND_X0`](#r_tilepro_mmend_x0) - X0 pipe mm "end"
- [`R_TILEPRO_MMEND_X1`](#r_tilepro_mmend_x1) - X1 pipe mm "end"
- [`R_TILEPRO_MMSTART_X0`](#r_tilepro_mmstart_x0) - X0 pipe mm "start"
- [`R_TILEPRO_MMSTART_X1`](#r_tilepro_mmstart_x1) - X1 pipe mm "start"
- [`R_TILEPRO_MT_IMM15_X1`](#r_tilepro_mt_imm15_x1) - X1 pipe mtspr
- [`R_TILEPRO_NONE`](#r_tilepro_none) - No reloc
- [`R_TILEPRO_RELATIVE`](#r_tilepro_relative) - Adjust by program base
- [`R_TILEPRO_SHAMT_X0`](#r_tilepro_shamt_x0) - X0 pipe shift amount
- [`R_TILEPRO_SHAMT_X1`](#r_tilepro_shamt_x1) - X1 pipe shift amount
- [`R_TILEPRO_SHAMT_Y0`](#r_tilepro_shamt_y0) - Y0 pipe shift amount
- [`R_TILEPRO_SHAMT_Y1`](#r_tilepro_shamt_y1) - Y1 pipe shift amount
- [`R_TILEPRO_TLS_DTPMOD32`](#r_tilepro_tls_dtpmod32) - ID of module containing symbol
- [`R_TILEPRO_TLS_DTPOFF32`](#r_tilepro_tls_dtpoff32) - Offset in TLS block
- [`R_TILEPRO_TLS_GD_CALL`](#r_tilepro_tls_gd_call) - "jal" for TLS GD
- [`R_TILEPRO_TLS_IE_LOAD`](#r_tilepro_tls_ie_load) - "lw_tls" for TLS IE
- [`R_TILEPRO_TLS_TPOFF32`](#r_tilepro_tls_tpoff32) - Offset in static TLS block
- [`R_X86_64_16`](#r_x86_64_16) - Direct 16 bit zero extended
- [`R_X86_64_32`](#r_x86_64_32) - Direct 32 bit zero extended
- [`R_X86_64_32S`](#r_x86_64_32s) - Direct 32 bit sign extended
- [`R_X86_64_64`](#r_x86_64_64) - Direct 64 bit
- [`R_X86_64_8`](#r_x86_64_8) - Direct 8 bit sign extended
- [`R_X86_64_COPY`](#r_x86_64_copy) - Copy symbol at runtime
- [`R_X86_64_DTPMOD64`](#r_x86_64_dtpmod64) - ID of module containing symbol
- [`R_X86_64_DTPOFF32`](#r_x86_64_dtpoff32) - Offset in TLS block
- [`R_X86_64_DTPOFF64`](#r_x86_64_dtpoff64) - Offset in module's TLS block
- [`R_X86_64_GLOB_DAT`](#r_x86_64_glob_dat) - Create GOT entry
- [`R_X86_64_GOT32`](#r_x86_64_got32) - 32 bit GOT entry
- [`R_X86_64_GOT64`](#r_x86_64_got64) - 64-bit GOT entry offset
- [`R_X86_64_GOTOFF64`](#r_x86_64_gotoff64) - 64 bit offset to GOT
- [`R_X86_64_GOTPC32`](#r_x86_64_gotpc32) - 32 bit signed pc relative offset to GOT
- [`R_X86_64_GOTPC32_TLSDESC`](#r_x86_64_gotpc32_tlsdesc) - GOT offset for TLS descriptor.
- [`R_X86_64_GOTPC64`](#r_x86_64_gotpc64) - 64-bit PC relative offset to GOT
- [`R_X86_64_GOTPCREL`](#r_x86_64_gotpcrel) - 32 bit signed PC relative offset to GOT
- [`R_X86_64_GOTPCREL64`](#r_x86_64_gotpcrel64) - 64-bit PC relative offset to GOT entry
- [`R_X86_64_GOTPCRELX`](#r_x86_64_gotpcrelx) - Load from 32 bit signed pc relative offset to GOT entry without REX prefix, relaxable.
- [`R_X86_64_GOTPLT64`](#r_x86_64_gotplt64) - like GOT64, says PLT entry needed
- [`R_X86_64_GOTTPOFF`](#r_x86_64_gottpoff) - 32 bit signed PC relative offset to GOT entry for IE symbol
- [`R_X86_64_IRELATIVE`](#r_x86_64_irelative) - Adjust indirectly by program base
- [`R_X86_64_JUMP_SLOT`](#r_x86_64_jump_slot) - Create PLT entry
- [`R_X86_64_NONE`](#r_x86_64_none) - No reloc
- [`R_X86_64_PC16`](#r_x86_64_pc16) - 16 bit sign extended pc relative
- [`R_X86_64_PC32`](#r_x86_64_pc32) - PC relative 32 bit signed
- [`R_X86_64_PC64`](#r_x86_64_pc64) - PC relative 64 bit
- [`R_X86_64_PC8`](#r_x86_64_pc8) - 8 bit sign extended pc relative
- [`R_X86_64_PLT32`](#r_x86_64_plt32) - 32 bit PLT address
- [`R_X86_64_PLTOFF64`](#r_x86_64_pltoff64) - 64-bit GOT relative offset to PLT entry
- [`R_X86_64_RELATIVE`](#r_x86_64_relative) - Adjust by program base
- [`R_X86_64_RELATIVE64`](#r_x86_64_relative64) - 64-bit adjust by program base
- [`R_X86_64_REX_GOTPCRELX`](#r_x86_64_rex_gotpcrelx) - Load from 32 bit signed pc relative offset to GOT entry with REX prefix, relaxable.
- [`R_X86_64_SIZE32`](#r_x86_64_size32) - Size of symbol plus 32-bit addend
- [`R_X86_64_SIZE64`](#r_x86_64_size64) - Size of symbol plus 64-bit addend
- [`R_X86_64_TLSDESC`](#r_x86_64_tlsdesc) - TLS descriptor.
- [`R_X86_64_TLSDESC_CALL`](#r_x86_64_tlsdesc_call) - Marker for call through TLS descriptor.
- [`R_X86_64_TLSGD`](#r_x86_64_tlsgd) - 32 bit signed PC relative offset to two GOT entries for GD symbol
- [`R_X86_64_TLSLD`](#r_x86_64_tlsld) - 32 bit signed PC relative offset to two GOT entries for LD symbol
- [`R_X86_64_TPOFF32`](#r_x86_64_tpoff32) - Offset in initial TLS block
- [`R_X86_64_TPOFF64`](#r_x86_64_tpoff64) - Offset in initial TLS block
- [`R_XTENSA_32`](#r_xtensa_32)
- [`R_XTENSA_32_PCREL`](#r_xtensa_32_pcrel)
- [`R_XTENSA_ASM_EXPAND`](#r_xtensa_asm_expand)
- [`R_XTENSA_ASM_SIMPLIFY`](#r_xtensa_asm_simplify)
- [`R_XTENSA_DIFF16`](#r_xtensa_diff16)
- [`R_XTENSA_DIFF32`](#r_xtensa_diff32)
- [`R_XTENSA_DIFF8`](#r_xtensa_diff8)
- [`R_XTENSA_GLOB_DAT`](#r_xtensa_glob_dat)
- [`R_XTENSA_GNU_VTENTRY`](#r_xtensa_gnu_vtentry)
- [`R_XTENSA_GNU_VTINHERIT`](#r_xtensa_gnu_vtinherit)
- [`R_XTENSA_JMP_SLOT`](#r_xtensa_jmp_slot)
- [`R_XTENSA_NDIFF16`](#r_xtensa_ndiff16)
- [`R_XTENSA_NDIFF32`](#r_xtensa_ndiff32)
- [`R_XTENSA_NDIFF8`](#r_xtensa_ndiff8)
- [`R_XTENSA_NONE`](#r_xtensa_none)
- [`R_XTENSA_OP0`](#r_xtensa_op0)
- [`R_XTENSA_OP1`](#r_xtensa_op1)
- [`R_XTENSA_OP2`](#r_xtensa_op2)
- [`R_XTENSA_PDIFF16`](#r_xtensa_pdiff16)
- [`R_XTENSA_PDIFF32`](#r_xtensa_pdiff32)
- [`R_XTENSA_PDIFF8`](#r_xtensa_pdiff8)
- [`R_XTENSA_PLT`](#r_xtensa_plt)
- [`R_XTENSA_RELATIVE`](#r_xtensa_relative)
- [`R_XTENSA_RTLD`](#r_xtensa_rtld)
- [`R_XTENSA_SLOT0_ALT`](#r_xtensa_slot0_alt)
- [`R_XTENSA_SLOT0_OP`](#r_xtensa_slot0_op)
- [`R_XTENSA_SLOT10_ALT`](#r_xtensa_slot10_alt)
- [`R_XTENSA_SLOT10_OP`](#r_xtensa_slot10_op)
- [`R_XTENSA_SLOT11_ALT`](#r_xtensa_slot11_alt)
- [`R_XTENSA_SLOT11_OP`](#r_xtensa_slot11_op)
- [`R_XTENSA_SLOT12_ALT`](#r_xtensa_slot12_alt)
- [`R_XTENSA_SLOT12_OP`](#r_xtensa_slot12_op)
- [`R_XTENSA_SLOT13_ALT`](#r_xtensa_slot13_alt)
- [`R_XTENSA_SLOT13_OP`](#r_xtensa_slot13_op)
- [`R_XTENSA_SLOT14_ALT`](#r_xtensa_slot14_alt)
- [`R_XTENSA_SLOT14_OP`](#r_xtensa_slot14_op)
- [`R_XTENSA_SLOT1_ALT`](#r_xtensa_slot1_alt)
- [`R_XTENSA_SLOT1_OP`](#r_xtensa_slot1_op)
- [`R_XTENSA_SLOT2_ALT`](#r_xtensa_slot2_alt)
- [`R_XTENSA_SLOT2_OP`](#r_xtensa_slot2_op)
- [`R_XTENSA_SLOT3_ALT`](#r_xtensa_slot3_alt)
- [`R_XTENSA_SLOT3_OP`](#r_xtensa_slot3_op)
- [`R_XTENSA_SLOT4_ALT`](#r_xtensa_slot4_alt)
- [`R_XTENSA_SLOT4_OP`](#r_xtensa_slot4_op)
- [`R_XTENSA_SLOT5_ALT`](#r_xtensa_slot5_alt)
- [`R_XTENSA_SLOT5_OP`](#r_xtensa_slot5_op)
- [`R_XTENSA_SLOT6_ALT`](#r_xtensa_slot6_alt)
- [`R_XTENSA_SLOT6_OP`](#r_xtensa_slot6_op)
- [`R_XTENSA_SLOT7_ALT`](#r_xtensa_slot7_alt)
- [`R_XTENSA_SLOT7_OP`](#r_xtensa_slot7_op)
- [`R_XTENSA_SLOT8_ALT`](#r_xtensa_slot8_alt)
- [`R_XTENSA_SLOT8_OP`](#r_xtensa_slot8_op)
- [`R_XTENSA_SLOT9_ALT`](#r_xtensa_slot9_alt)
- [`R_XTENSA_SLOT9_OP`](#r_xtensa_slot9_op)
- [`R_XTENSA_TLSDESC_ARG`](#r_xtensa_tlsdesc_arg)
- [`R_XTENSA_TLSDESC_FN`](#r_xtensa_tlsdesc_fn)
- [`R_XTENSA_TLS_ARG`](#r_xtensa_tls_arg)
- [`R_XTENSA_TLS_CALL`](#r_xtensa_tls_call)
- [`R_XTENSA_TLS_DTPOFF`](#r_xtensa_tls_dtpoff)
- [`R_XTENSA_TLS_FUNC`](#r_xtensa_tls_func)
- [`R_XTENSA_TLS_TPOFF`](#r_xtensa_tls_tpoff)
- [`SHF_ALLOC`](#shf_alloc) - Section occupies memory during execution.
- [`SHF_ALPHA_GPREL`](#shf_alpha_gprel)
- [`SHF_ARM_COMDEF`](#shf_arm_comdef) - Section may be multiply defined in the input to a link step.
- [`SHF_ARM_ENTRYSECT`](#shf_arm_entrysect) - Section contains an entry point
- [`SHF_COMPRESSED`](#shf_compressed) - Section is compressed.
- [`SHF_EXCLUDE`](#shf_exclude) - This section is excluded from the final executable or shared library.
- [`SHF_EXECINSTR`](#shf_execinstr) - Section is executable.
- [`SHF_GNU_MBIND`](#shf_gnu_mbind) - Mbind section.
- [`SHF_GNU_RETAIN`](#shf_gnu_retain) - Section should not be garbage collected by the linker.
- [`SHF_GROUP`](#shf_group) - Section is a member of a group.
- [`SHF_IA_64_NORECOV`](#shf_ia_64_norecov) - spec insns w/o recovery
- [`SHF_IA_64_SHORT`](#shf_ia_64_short) - section near gp
- [`SHF_INFO_LINK`](#shf_info_link) - The `sh_info` field contains a section header table index.
- [`SHF_LINK_ORDER`](#shf_link_order) - Section has special ordering requirements when combining sections.
- [`SHF_MASKOS`](#shf_maskos) - OS-specific section flags.
- [`SHF_MASKPROC`](#shf_maskproc) - Processor-specific section flags.
- [`SHF_MERGE`](#shf_merge) - Section may be be merged to eliminate duplication.
- [`SHF_MIPS_ADDR`](#shf_mips_addr)
- [`SHF_MIPS_GPREL`](#shf_mips_gprel) - Must be in global data area.
- [`SHF_MIPS_LOCAL`](#shf_mips_local)
- [`SHF_MIPS_MERGE`](#shf_mips_merge)
- [`SHF_MIPS_NAMES`](#shf_mips_names)
- [`SHF_MIPS_NODUPE`](#shf_mips_nodupe)
- [`SHF_MIPS_NOSTRIP`](#shf_mips_nostrip)
- [`SHF_MIPS_STRINGS`](#shf_mips_strings)
- [`SHF_OS_NONCONFORMING`](#shf_os_nonconforming) - Section requires special OS-specific handling.
- [`SHF_PARISC_HUGE`](#shf_parisc_huge) - Section far from gp.
- [`SHF_PARISC_SBP`](#shf_parisc_sbp) - Static branch prediction code.
- [`SHF_PARISC_SHORT`](#shf_parisc_short) - Section with short addressing.
- [`SHF_STRINGS`](#shf_strings) - Section contains nul-terminated strings.
- [`SHF_TLS`](#shf_tls) - Section holds thread-local storage.
- [`SHF_WRITE`](#shf_write) - Section is writable.
- [`SHN_ABS`](#shn_abs) - Associated symbol is absolute.
- [`SHN_COMMON`](#shn_common) - Associated symbol is common.
- [`SHN_HIOS`](#shn_hios) - End of OS-specific section indices.
- [`SHN_HIPROC`](#shn_hiproc) - End of processor-specific section indices.
- [`SHN_HIRESERVE`](#shn_hireserve) - End of reserved section indices.
- [`SHN_LOOS`](#shn_loos) - Start of OS-specific section indices.
- [`SHN_LOPROC`](#shn_loproc) - Start of processor-specific section indices.
- [`SHN_LORESERVE`](#shn_loreserve) - OS-specific range start.
- [`SHN_MIPS_ACOMMON`](#shn_mips_acommon) - Allocated common symbols.
- [`SHN_MIPS_DATA`](#shn_mips_data) - Allocated data symbols.
- [`SHN_MIPS_SCOMMON`](#shn_mips_scommon) - Small common symbols.
- [`SHN_MIPS_SUNDEFINED`](#shn_mips_sundefined) - Small undefined symbols.
- [`SHN_MIPS_TEXT`](#shn_mips_text) - Allocated test symbols.
- [`SHN_PARISC_ANSI_COMMON`](#shn_parisc_ansi_common) - Section for tentatively declared symbols in ANSI C.
- [`SHN_PARISC_HUGE_COMMON`](#shn_parisc_huge_common) - Common blocks in huge model.
- [`SHN_UNDEF`](#shn_undef) - Undefined section.
- [`SHN_XINDEX`](#shn_xindex) - Section index is in the `SHT_SYMTAB_SHNDX` section.
- [`SHT_AARCH64_ATTRIBUTES`](#sht_aarch64_attributes) - AArch64 attributes section.
- [`SHT_ALPHA_DEBUG`](#sht_alpha_debug)
- [`SHT_ALPHA_REGINFO`](#sht_alpha_reginfo)
- [`SHT_ARM_ATTRIBUTES`](#sht_arm_attributes) - ARM attributes section.
- [`SHT_ARM_EXIDX`](#sht_arm_exidx) - ARM unwind section.
- [`SHT_ARM_PREEMPTMAP`](#sht_arm_preemptmap) - Preemption details.
- [`SHT_CHECKSUM`](#sht_checksum) - Checksum for DSO content.
- [`SHT_CREL`](#sht_crel) - Experimental CREL relocations. LLVM will change the value and
- [`SHT_CSKY_ATTRIBUTES`](#sht_csky_attributes) - C-SKY attributes section.
- [`SHT_DYNAMIC`](#sht_dynamic) - Dynamic linking information.
- [`SHT_DYNSYM`](#sht_dynsym) - Dynamic linker symbol table.
- [`SHT_FINI_ARRAY`](#sht_fini_array) - Array of destructors.
- [`SHT_GNU_ATTRIBUTES`](#sht_gnu_attributes) - Object attributes.
- [`SHT_GNU_HASH`](#sht_gnu_hash) - GNU-style hash table.
- [`SHT_GNU_LIBLIST`](#sht_gnu_liblist) - Prelink library list
- [`SHT_GNU_SFRAME`](#sht_gnu_sframe) - GNU SFrame stack trace format.
- [`SHT_GNU_VERDEF`](#sht_gnu_verdef) - Version definition section.
- [`SHT_GNU_VERNEED`](#sht_gnu_verneed) - Version needs section.
- [`SHT_GNU_VERSYM`](#sht_gnu_versym) - Version symbol table.
- [`SHT_GROUP`](#sht_group) - Section group.
- [`SHT_HASH`](#sht_hash) - Symbol hash table.
- [`SHT_HIOS`](#sht_hios) - End of OS-specific section types.
- [`SHT_HIPROC`](#sht_hiproc) - End of processor-specific section types.
- [`SHT_HISUNW`](#sht_hisunw) - Sun-specific high bound.
- [`SHT_HIUSER`](#sht_hiuser) - End of application-specific section types.
- [`SHT_IA_64_EXT`](#sht_ia_64_ext) - extension bits
- [`SHT_IA_64_UNWIND`](#sht_ia_64_unwind) - unwind bits
- [`SHT_INIT_ARRAY`](#sht_init_array) - Array of constructors.
- [`SHT_LLVM_DEPENDENT_LIBRARIES`](#sht_llvm_dependent_libraries) - LLVM-style dependent libraries.
- [`SHT_LOOS`](#sht_loos) - Start of OS-specific section types.
- [`SHT_LOPROC`](#sht_loproc) - Start of processor-specific section types.
- [`SHT_LOSUNW`](#sht_losunw) - Sun-specific low bound.
- [`SHT_LOUSER`](#sht_louser) - Start of application-specific section types.
- [`SHT_MIPS_AUXSYM`](#sht_mips_auxsym)
- [`SHT_MIPS_CONFLICT`](#sht_mips_conflict) - Conflicting symbols.
- [`SHT_MIPS_CONTENT`](#sht_mips_content)
- [`SHT_MIPS_DEBUG`](#sht_mips_debug) - MIPS ECOFF debugging info.
- [`SHT_MIPS_DELTACLASS`](#sht_mips_deltaclass)
- [`SHT_MIPS_DELTADECL`](#sht_mips_deltadecl)
- [`SHT_MIPS_DELTAINST`](#sht_mips_deltainst)
- [`SHT_MIPS_DELTASYM`](#sht_mips_deltasym)
- [`SHT_MIPS_DENSE`](#sht_mips_dense)
- [`SHT_MIPS_DWARF`](#sht_mips_dwarf) - DWARF debugging information.
- [`SHT_MIPS_EH_REGION`](#sht_mips_eh_region)
- [`SHT_MIPS_EVENTS`](#sht_mips_events) - Event section.
- [`SHT_MIPS_EXTSYM`](#sht_mips_extsym)
- [`SHT_MIPS_FDESC`](#sht_mips_fdesc)
- [`SHT_MIPS_GPTAB`](#sht_mips_gptab) - Global data area sizes.
- [`SHT_MIPS_IFACE`](#sht_mips_iface)
- [`SHT_MIPS_LIBLIST`](#sht_mips_liblist) - Shared objects used in link.
- [`SHT_MIPS_LINE`](#sht_mips_line)
- [`SHT_MIPS_LOCSTR`](#sht_mips_locstr)
- [`SHT_MIPS_LOCSYM`](#sht_mips_locsym)
- [`SHT_MIPS_MSYM`](#sht_mips_msym)
- [`SHT_MIPS_OPTIONS`](#sht_mips_options) - Miscellaneous options.
- [`SHT_MIPS_OPTSYM`](#sht_mips_optsym)
- [`SHT_MIPS_PACKAGE`](#sht_mips_package)
- [`SHT_MIPS_PACKSYM`](#sht_mips_packsym)
- [`SHT_MIPS_PDESC`](#sht_mips_pdesc)
- [`SHT_MIPS_PDR_EXCEPTION`](#sht_mips_pdr_exception)
- [`SHT_MIPS_PIXIE`](#sht_mips_pixie)
- [`SHT_MIPS_REGINFO`](#sht_mips_reginfo) - Register usage information.
- [`SHT_MIPS_RELD`](#sht_mips_reld)
- [`SHT_MIPS_RFDESC`](#sht_mips_rfdesc)
- [`SHT_MIPS_SHDR`](#sht_mips_shdr)
- [`SHT_MIPS_SYMBOL_LIB`](#sht_mips_symbol_lib)
- [`SHT_MIPS_TRANSLATE`](#sht_mips_translate)
- [`SHT_MIPS_UCODE`](#sht_mips_ucode) - Reserved for SGI/MIPS compilers
- [`SHT_MIPS_WHIRL`](#sht_mips_whirl)
- [`SHT_MIPS_XLATE`](#sht_mips_xlate)
- [`SHT_MIPS_XLATE_DEBUG`](#sht_mips_xlate_debug)
- [`SHT_MIPS_XLATE_OLD`](#sht_mips_xlate_old)
- [`SHT_NOBITS`](#sht_nobits) - Program space with no data (bss).
- [`SHT_NOTE`](#sht_note) - Notes.
- [`SHT_NULL`](#sht_null) - Section header table entry is unused.
- [`SHT_PARISC_DOC`](#sht_parisc_doc) - Debug info for optimized code.
- [`SHT_PARISC_EXT`](#sht_parisc_ext) - Contains product specific ext.
- [`SHT_PARISC_UNWIND`](#sht_parisc_unwind) - Unwind information.
- [`SHT_PREINIT_ARRAY`](#sht_preinit_array) - Array of pre-constructors.
- [`SHT_PROGBITS`](#sht_progbits) - Program data.
- [`SHT_REL`](#sht_rel) - Relocation entries without explicit addends.
- [`SHT_RELA`](#sht_rela) - Relocation entries with explicit addends.
- [`SHT_RELR`](#sht_relr) - Relocation entries; only offsets.
- [`SHT_RISCV_ATTRIBUTES`](#sht_riscv_attributes) - RISC-V attributes section.
- [`SHT_SHARC_ADI_ATTRIBUTES`](#sht_sharc_adi_attributes) - .adi.attributes
- [`SHT_SHLIB`](#sht_shlib) - Reserved section type.
- [`SHT_STRTAB`](#sht_strtab) - String table.
- [`SHT_SUNW_COMDAT`](#sht_sunw_comdat)
- [`SHT_SUNW_move`](#sht_sunw_move)
- [`SHT_SUNW_syminfo`](#sht_sunw_syminfo)
- [`SHT_SYMTAB`](#sht_symtab) - Symbol table.
- [`SHT_SYMTAB_SHNDX`](#sht_symtab_shndx) - Extended section indices for a symbol table.
- [`SHT_X86_64_UNWIND`](#sht_x86_64_unwind) - Unwind information.
- [`STB_GLOBAL`](#stb_global) - Global symbol.
- [`STB_GNU_UNIQUE`](#stb_gnu_unique) - Unique symbol.
- [`STB_HIOS`](#stb_hios) - End of OS-specific symbol binding.
- [`STB_HIPROC`](#stb_hiproc) - End of processor-specific symbol binding.
- [`STB_LOCAL`](#stb_local) - Local symbol.
- [`STB_LOOS`](#stb_loos) - Start of OS-specific symbol binding.
- [`STB_LOPROC`](#stb_loproc) - Start of processor-specific symbol binding.
- [`STB_MIPS_SPLIT_COMMON`](#stb_mips_split_common)
- [`STB_WEAK`](#stb_weak) - Weak symbol.
- [`STO_AARCH64_VARIANT_PCS`](#sto_aarch64_variant_pcs)
- [`STO_ALPHA_NOPV`](#sto_alpha_nopv) - No PV required.
- [`STO_ALPHA_STD_GPLOAD`](#sto_alpha_std_gpload) - PV only used for initial ldgp.
- [`STO_MIPS_PLT`](#sto_mips_plt)
- [`STO_MIPS_SC_ALIGN_UNUSED`](#sto_mips_sc_align_unused) - Only valid for `STB_MIPS_SPLIT_COMMON`.
- [`STO_PPC64_LOCAL_BIT`](#sto_ppc64_local_bit)
- [`STO_PPC64_LOCAL_MASK`](#sto_ppc64_local_mask)
- [`STO_RISCV_VARIANT_CC`](#sto_riscv_variant_cc) - Function uses variant calling convention.
- [`STT_ARM_16BIT`](#stt_arm_16bit) - A Thumb label.
- [`STT_ARM_TFUNC`](#stt_arm_tfunc) - A Thumb function.
- [`STT_COMMON`](#stt_common) - Symbol is a common data object.
- [`STT_FILE`](#stt_file) - Symbol's name is a file name.
- [`STT_FUNC`](#stt_func) - Symbol is a code object.
- [`STT_GNU_IFUNC`](#stt_gnu_ifunc) - Symbol is an indirect code object.
- [`STT_HIOS`](#stt_hios) - End of OS-specific symbol types.
- [`STT_HIPROC`](#stt_hiproc) - End of processor-specific symbol types.
- [`STT_HP_OPAQUE`](#stt_hp_opaque)
- [`STT_HP_STUB`](#stt_hp_stub)
- [`STT_LOOS`](#stt_loos) - Start of OS-specific symbol types.
- [`STT_LOPROC`](#stt_loproc) - Start of processor-specific symbol types.
- [`STT_NOTYPE`](#stt_notype) - Symbol type is unspecified.
- [`STT_OBJECT`](#stt_object) - Symbol is a data object.
- [`STT_PARISC_MILLICODE`](#stt_parisc_millicode) - Millicode function entry point.
- [`STT_SECTION`](#stt_section) - Symbol is associated with a section.
- [`STT_SPARC_REGISTER`](#stt_sparc_register) - Global register reserved to app.
- [`STT_TLS`](#stt_tls) - Symbol is a thread-local storage object.
- [`STV_DEFAULT`](#stv_default) - Default symbol visibility rules.
- [`STV_HIDDEN`](#stv_hidden) - Symbol is not visible to other components.
- [`STV_INTERNAL`](#stv_internal) - Processor specific hidden class.
- [`STV_PROTECTED`](#stv_protected) - Symbol is visible to other components, but is not preemptible.
- [`SYMINFO_BT_LOWRESERVE`](#syminfo_bt_lowreserve) - Beginning of reserved entries
- [`SYMINFO_BT_PARENT`](#syminfo_bt_parent) - Symbol bound to parent
- [`SYMINFO_BT_SELF`](#syminfo_bt_self) - Symbol bound to self
- [`SYMINFO_CURRENT`](#syminfo_current)
- [`SYMINFO_FLG_COPY`](#syminfo_flg_copy) - Symbol is a copy-reloc
- [`SYMINFO_FLG_DIRECT`](#syminfo_flg_direct) - Direct bound symbol
- [`SYMINFO_FLG_LAZYLOAD`](#syminfo_flg_lazyload) - Symbol bound to object to be lazy loaded
- [`SYMINFO_FLG_PASSTHRU`](#syminfo_flg_passthru) - Pass-thru symbol for translator
- [`SYMINFO_NONE`](#syminfo_none)
- [`SYMINFO_NUM`](#syminfo_num)
- [`Tag_File`](#tag_file)
- [`Tag_Section`](#tag_section)
- [`Tag_Symbol`](#tag_symbol)
- [`VERSYM_HIDDEN`](#versym_hidden) - Symbol is hidden.
- [`VERSYM_VERSION`](#versym_version) - Symbol version index.
- [`VER_DEF_CURRENT`](#ver_def_current) - Current version
- [`VER_DEF_NONE`](#ver_def_none) - No version
- [`VER_FLG_BASE`](#ver_flg_base) - Version definition of file itself
- [`VER_FLG_WEAK`](#ver_flg_weak) - Weak version identifier
- [`VER_NDX_GLOBAL`](#ver_ndx_global) - Symbol is global.
- [`VER_NDX_LOCAL`](#ver_ndx_local) - Symbol is local.
- [`VER_NEED_CURRENT`](#ver_need_current) - Current version
- [`VER_NEED_NONE`](#ver_need_none) - No version

---

## object::elf::CompressionHeader32

*Struct*

Section compression header.

Used when `SHF_COMPRESSED` is set.

Note: this type currently allows for misaligned headers, but that may be
changed in a future version.

**Generic Parameters:**
- E

**Fields:**
- `ch_type: crate::endian::U32Bytes<E>` - Compression format. One of the `ELFCOMPRESS_*` values.
- `ch_size: crate::endian::U32Bytes<E>` - Uncompressed data size.
- `ch_addralign: crate::endian::U32Bytes<E>` - Uncompressed data alignment.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> CompressionHeader32<E>`
- **Clone**
  - `fn clone(self: &Self) -> CompressionHeader32<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CompressionHeader**
  - `fn ch_type(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn ch_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn ch_addralign(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`



## object::elf::CompressionHeader64

*Struct*

Section compression header.

Used when `SHF_COMPRESSED` is set.

Note: this type currently allows for misaligned headers, but that may be
changed in a future version.

**Generic Parameters:**
- E

**Fields:**
- `ch_type: crate::endian::U32Bytes<E>` - Compression format. One of the `ELFCOMPRESS_*` values.
- `ch_reserved: crate::endian::U32Bytes<E>` - Reserved.
- `ch_size: crate::endian::U64Bytes<E>` - Uncompressed data size.
- `ch_addralign: crate::endian::U64Bytes<E>` - Uncompressed data alignment.

**Traits:** Pod, Copy

**Trait Implementations:**

- **CompressionHeader**
  - `fn ch_type(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn ch_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn ch_addralign(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> CompressionHeader64<E>`
- **Clone**
  - `fn clone(self: &Self) -> CompressionHeader64<E>`



## object::elf::DF_1_CONFALT

*Constant*: `u32`

Configuration alternative created.



## object::elf::DF_1_DIRECT

*Constant*: `u32`

Direct binding enabled.



## object::elf::DF_1_DISPRELDNE

*Constant*: `u32`

Disp reloc applied at build time.



## object::elf::DF_1_DISPRELPND

*Constant*: `u32`

Disp reloc applied at run-time.



## object::elf::DF_1_EDITED

*Constant*: `u32`

Object is modified after built.



## object::elf::DF_1_ENDFILTEE

*Constant*: `u32`

Filtee terminates filters search.



## object::elf::DF_1_GLOBAL

*Constant*: `u32`

Set RTLD_GLOBAL for this object.



## object::elf::DF_1_GLOBAUDIT

*Constant*: `u32`

Global auditing required.



## object::elf::DF_1_GROUP

*Constant*: `u32`

Set RTLD_GROUP for this object.



## object::elf::DF_1_IGNMULDEF

*Constant*: `u32`



## object::elf::DF_1_INITFIRST

*Constant*: `u32`

Set RTLD_INITFIRST for this object.



## object::elf::DF_1_INTERPOSE

*Constant*: `u32`

Object is used to interpose.



## object::elf::DF_1_LOADFLTR

*Constant*: `u32`

Trigger filtee loading at runtime.



## object::elf::DF_1_NODEFLIB

*Constant*: `u32`

Ignore default lib search path.



## object::elf::DF_1_NODELETE

*Constant*: `u32`

Set RTLD_NODELETE for this object.



## object::elf::DF_1_NODIRECT

*Constant*: `u32`

Object has no-direct binding.



## object::elf::DF_1_NODUMP

*Constant*: `u32`

Object can't be dldump'ed.



## object::elf::DF_1_NOHDR

*Constant*: `u32`



## object::elf::DF_1_NOKSYMS

*Constant*: `u32`



## object::elf::DF_1_NOOPEN

*Constant*: `u32`

Set RTLD_NOOPEN for this object.



## object::elf::DF_1_NORELOC

*Constant*: `u32`



## object::elf::DF_1_NOW

*Constant*: `u32`

Set RTLD_NOW for this object.



## object::elf::DF_1_ORIGIN

*Constant*: `u32`

$ORIGIN must be handled.



## object::elf::DF_1_PIE

*Constant*: `u32`



## object::elf::DF_1_SINGLETON

*Constant*: `u32`

Singleton symbols are used.



## object::elf::DF_1_STUB

*Constant*: `u32`



## object::elf::DF_1_SYMINTPOSE

*Constant*: `u32`

Object has individual interposers.



## object::elf::DF_1_TRANS

*Constant*: `u32`



## object::elf::DF_BIND_NOW

*Constant*: `u32`

No lazy binding for this object



## object::elf::DF_ORIGIN

*Constant*: `u32`

Object may use DF_ORIGIN



## object::elf::DF_STATIC_TLS

*Constant*: `u32`

Module uses the static TLS model



## object::elf::DF_SYMBOLIC

*Constant*: `u32`

Symbol resolutions starts here



## object::elf::DF_TEXTREL

*Constant*: `u32`

Object contains text relocations



## object::elf::DT_AARCH64_BTI_PLT

*Constant*: `u32`



## object::elf::DT_AARCH64_NUM

*Constant*: `u32`



## object::elf::DT_AARCH64_PAC_PLT

*Constant*: `u32`



## object::elf::DT_AARCH64_VARIANT_PCS

*Constant*: `u32`



## object::elf::DT_ADDRRNGHI

*Constant*: `u32`



## object::elf::DT_ADDRRNGLO

*Constant*: `u32`



## object::elf::DT_ALPHA_PLTRO

*Constant*: `u32`



## object::elf::DT_AUDIT

*Constant*: `u32`

Object auditing.



## object::elf::DT_AUXILIARY

*Constant*: `u32`

Shared object to load before self



## object::elf::DT_BIND_NOW

*Constant*: `u32`

Process relocations of object



## object::elf::DT_CHECKSUM

*Constant*: `u32`



## object::elf::DT_CONFIG

*Constant*: `u32`

Configuration information.



## object::elf::DT_DEBUG

*Constant*: `u32`

For debugging; unspecified



## object::elf::DT_DEPAUDIT

*Constant*: `u32`

Dependency auditing.



## object::elf::DT_E2K_EXPORT_PL

*Constant*: `u32`



## object::elf::DT_E2K_EXPORT_PLSZ

*Constant*: `u32`



## object::elf::DT_E2K_INIT_GOT

*Constant*: `u32`



## object::elf::DT_E2K_LAZY

*Constant*: `u32`



## object::elf::DT_E2K_LAZY_GOT

*Constant*: `u32`



## object::elf::DT_E2K_NO_SELFINIT

*Constant*: `u32`



## object::elf::DT_E2K_NUM

*Constant*: `u32`



## object::elf::DT_E2K_REAL_PLTGOT

*Constant*: `u32`



## object::elf::DT_ENCODING

*Constant*: `u32`

Start of encoded range



## object::elf::DT_FEATURE_1

*Constant*: `u32`

Feature selection (DTF_*).



## object::elf::DT_FILTER

*Constant*: `u32`

Shared object to get values from



## object::elf::DT_FINI

*Constant*: `u32`

Address of termination function



## object::elf::DT_FINI_ARRAY

*Constant*: `u32`

Array with addresses of fini fct



## object::elf::DT_FINI_ARRAYSZ

*Constant*: `u32`

Size in bytes of DT_FINI_ARRAY



## object::elf::DT_FLAGS

*Constant*: `u32`

Flags for the object being loaded



## object::elf::DT_FLAGS_1

*Constant*: `u32`

State flags, see DF_1_* below.



## object::elf::DT_GNU_CONFLICT

*Constant*: `u32`

Start of conflict section



## object::elf::DT_GNU_CONFLICTSZ

*Constant*: `u32`

Size of conflict section



## object::elf::DT_GNU_HASH

*Constant*: `u32`

GNU-style hash table.



## object::elf::DT_GNU_LIBLIST

*Constant*: `u32`

Library list



## object::elf::DT_GNU_LIBLISTSZ

*Constant*: `u32`

Size of library list



## object::elf::DT_GNU_PRELINKED

*Constant*: `u32`

Prelinking timestamp



## object::elf::DT_HASH

*Constant*: `u32`

Address of symbol hash table



## object::elf::DT_HIOS

*Constant*: `u32`

End of OS-specific



## object::elf::DT_HIPROC

*Constant*: `u32`

End of processor-specific



## object::elf::DT_IA_64_PLT_RESERVE

*Constant*: `u32`



## object::elf::DT_INIT

*Constant*: `u32`

Address of init function



## object::elf::DT_INIT_ARRAY

*Constant*: `u32`

Array with addresses of init fct



## object::elf::DT_INIT_ARRAYSZ

*Constant*: `u32`

Size in bytes of DT_INIT_ARRAY



## object::elf::DT_JMPREL

*Constant*: `u32`

Address of PLT relocs



## object::elf::DT_LOOS

*Constant*: `u32`

Start of OS-specific



## object::elf::DT_LOPROC

*Constant*: `u32`

Start of processor-specific



## object::elf::DT_MIPS_AUX_DYNAMIC

*Constant*: `u32`

Address of aux .dynamic.



## object::elf::DT_MIPS_BASE_ADDRESS

*Constant*: `u32`

Base address



## object::elf::DT_MIPS_COMPACT_SIZE

*Constant*: `u32`

(O32)Size of compact rel section.



## object::elf::DT_MIPS_CONFLICT

*Constant*: `u32`

Address of CONFLICT section



## object::elf::DT_MIPS_CONFLICTNO

*Constant*: `u32`

Number of CONFLICT entries



## object::elf::DT_MIPS_CXX_FLAGS

*Constant*: `u32`

Flags indicating for C++ flavor.



## object::elf::DT_MIPS_DELTA_CLASS

*Constant*: `u32`

Delta C++ class definition.



## object::elf::DT_MIPS_DELTA_CLASSSYM

*Constant*: `u32`

Delta symbols that hold the class declaration.



## object::elf::DT_MIPS_DELTA_CLASSSYM_NO

*Constant*: `u32`

Number of entries in DT_MIPS_DELTA_CLASSSYM.



## object::elf::DT_MIPS_DELTA_CLASS_NO

*Constant*: `u32`

Number of entries in DT_MIPS_DELTA_CLASS.



## object::elf::DT_MIPS_DELTA_INSTANCE

*Constant*: `u32`

Delta C++ class instances.



## object::elf::DT_MIPS_DELTA_INSTANCE_NO

*Constant*: `u32`

Number of entries in DT_MIPS_DELTA_INSTANCE.



## object::elf::DT_MIPS_DELTA_RELOC

*Constant*: `u32`

Delta relocations.



## object::elf::DT_MIPS_DELTA_RELOC_NO

*Constant*: `u32`

Number of entries in DT_MIPS_DELTA_RELOC.



## object::elf::DT_MIPS_DELTA_SYM

*Constant*: `u32`

Delta symbols that Delta relocations refer to.



## object::elf::DT_MIPS_DELTA_SYM_NO

*Constant*: `u32`

Number of entries in DT_MIPS_DELTA_SYM.



## object::elf::DT_MIPS_DYNSTR_ALIGN

*Constant*: `u32`



## object::elf::DT_MIPS_FLAGS

*Constant*: `u32`

Flags



## object::elf::DT_MIPS_GOTSYM

*Constant*: `u32`

First GOT entry in DYNSYM



## object::elf::DT_MIPS_GP_VALUE

*Constant*: `u32`

GP value for aux GOTs.



## object::elf::DT_MIPS_HIDDEN_GOTIDX

*Constant*: `u32`



## object::elf::DT_MIPS_HIPAGENO

*Constant*: `u32`

Number of GOT page table entries



## object::elf::DT_MIPS_ICHECKSUM

*Constant*: `u32`

Checksum



## object::elf::DT_MIPS_INTERFACE

*Constant*: `u32`

Address of .interface.



## object::elf::DT_MIPS_INTERFACE_SIZE

*Constant*: `u32`

Size of the .interface section.



## object::elf::DT_MIPS_IVERSION

*Constant*: `u32`

Version string (string tbl index)



## object::elf::DT_MIPS_LIBLIST

*Constant*: `u32`

Address of LIBLIST section



## object::elf::DT_MIPS_LIBLISTNO

*Constant*: `u32`

Number of LIBLIST entries



## object::elf::DT_MIPS_LOCALPAGE_GOTIDX

*Constant*: `u32`



## object::elf::DT_MIPS_LOCAL_GOTIDX

*Constant*: `u32`



## object::elf::DT_MIPS_LOCAL_GOTNO

*Constant*: `u32`

Number of local GOT entries



## object::elf::DT_MIPS_MSYM

*Constant*: `u32`



## object::elf::DT_MIPS_OPTIONS

*Constant*: `u32`

Address of .options.



## object::elf::DT_MIPS_PERF_SUFFIX

*Constant*: `u32`

Default suffix of dso to be added by rld on dlopen() calls.



## object::elf::DT_MIPS_PIXIE_INIT

*Constant*: `u32`



## object::elf::DT_MIPS_PLTGOT

*Constant*: `u32`

The address of .got.plt in an executable using the new non-PIC ABI.



## object::elf::DT_MIPS_PROTECTED_GOTIDX

*Constant*: `u32`



## object::elf::DT_MIPS_RLD_MAP

*Constant*: `u32`

Address of run time loader map.



## object::elf::DT_MIPS_RLD_MAP_REL

*Constant*: `u32`

An alternative description of the classic MIPS RLD_MAP that is usable in a PIE as it stores a relative offset from the address of the tag rather than an absolute address.



## object::elf::DT_MIPS_RLD_TEXT_RESOLVE_ADDR

*Constant*: `u32`

Address of rld_text_rsolve function stored in GOT.



## object::elf::DT_MIPS_RLD_VERSION

*Constant*: `u32`

Runtime linker interface version



## object::elf::DT_MIPS_RWPLT

*Constant*: `u32`

The base of the PLT in an executable using the new non-PIC ABI if that PLT is writable.  For a non-writable PLT, this is omitted or has a zero value.



## object::elf::DT_MIPS_SYMBOL_LIB

*Constant*: `u32`



## object::elf::DT_MIPS_SYMTABNO

*Constant*: `u32`

Number of DYNSYM entries



## object::elf::DT_MIPS_TIME_STAMP

*Constant*: `u32`

Timestamp



## object::elf::DT_MIPS_UNREFEXTNO

*Constant*: `u32`

First external DYNSYM



## object::elf::DT_MOVEENT

*Constant*: `u32`



## object::elf::DT_MOVESZ

*Constant*: `u32`



## object::elf::DT_MOVETAB

*Constant*: `u32`

Move table.



## object::elf::DT_NEEDED

*Constant*: `u32`

Name of needed library



## object::elf::DT_NIOS2_GP

*Constant*: `u32`

Address of _gp.



## object::elf::DT_NULL

*Constant*: `u32`

Marks end of dynamic section



## object::elf::DT_PLTGOT

*Constant*: `u32`

Processor defined value



## object::elf::DT_PLTPAD

*Constant*: `u32`

PLT padding.



## object::elf::DT_PLTPADSZ

*Constant*: `u32`



## object::elf::DT_PLTREL

*Constant*: `u32`

Type of reloc in PLT



## object::elf::DT_PLTRELSZ

*Constant*: `u32`

Size in bytes of PLT relocs



## object::elf::DT_POSFLAG_1

*Constant*: `u32`

Flags for DT_* entries, affecting the following DT_* entry.



## object::elf::DT_PPC64_GLINK

*Constant*: `u32`



## object::elf::DT_PPC64_OPD

*Constant*: `u32`



## object::elf::DT_PPC64_OPDSZ

*Constant*: `u32`



## object::elf::DT_PPC64_OPT

*Constant*: `u32`



## object::elf::DT_PPC_GOT

*Constant*: `u32`



## object::elf::DT_PPC_OPT

*Constant*: `u32`



## object::elf::DT_PREINIT_ARRAY

*Constant*: `u32`

Array with addresses of preinit fct



## object::elf::DT_PREINIT_ARRAYSZ

*Constant*: `u32`

size in bytes of DT_PREINIT_ARRAY



## object::elf::DT_REL

*Constant*: `u32`

Address of Rel relocs



## object::elf::DT_RELA

*Constant*: `u32`

Address of Rela relocs



## object::elf::DT_RELACOUNT

*Constant*: `u32`



## object::elf::DT_RELAENT

*Constant*: `u32`

Size of one Rela reloc



## object::elf::DT_RELASZ

*Constant*: `u32`

Total size of Rela relocs



## object::elf::DT_RELCOUNT

*Constant*: `u32`



## object::elf::DT_RELENT

*Constant*: `u32`

Size of one Rel reloc



## object::elf::DT_RELSZ

*Constant*: `u32`

Total size of Rel relocs



## object::elf::DT_RISCV_VARIANT_CC

*Constant*: `u32`



## object::elf::DT_RPATH

*Constant*: `u32`

Library search path (deprecated)



## object::elf::DT_RUNPATH

*Constant*: `u32`

Library search path



## object::elf::DT_SONAME

*Constant*: `u32`

Name of shared object



## object::elf::DT_SPARC_REGISTER

*Constant*: `u32`



## object::elf::DT_STRSZ

*Constant*: `u32`

Size of string table



## object::elf::DT_STRTAB

*Constant*: `u32`

Address of string table



## object::elf::DT_SYMBOLIC

*Constant*: `u32`

Start symbol search here



## object::elf::DT_SYMENT

*Constant*: `u32`

Size of one symbol table entry



## object::elf::DT_SYMINENT

*Constant*: `u32`

Entry size of syminfo



## object::elf::DT_SYMINFO

*Constant*: `u32`

Syminfo table.



## object::elf::DT_SYMINSZ

*Constant*: `u32`

Size of syminfo table (in bytes)



## object::elf::DT_SYMTAB

*Constant*: `u32`

Address of symbol table



## object::elf::DT_SYMTAB_SHNDX

*Constant*: `u32`

Address of SYMTAB_SHNDX section



## object::elf::DT_TEXTREL

*Constant*: `u32`

Reloc might modify .text



## object::elf::DT_TLSDESC_GOT

*Constant*: `u32`



## object::elf::DT_TLSDESC_PLT

*Constant*: `u32`



## object::elf::DT_VALRNGHI

*Constant*: `u32`



## object::elf::DT_VALRNGLO

*Constant*: `u32`



## object::elf::DT_VERDEF

*Constant*: `u32`

Address of version definition table



## object::elf::DT_VERDEFNUM

*Constant*: `u32`

Number of version definitions



## object::elf::DT_VERNEED

*Constant*: `u32`

Address of table with needed versions



## object::elf::DT_VERNEEDNUM

*Constant*: `u32`

Number of needed versions



## object::elf::DT_VERSYM

*Constant*: `u32`



## object::elf::Dyn32

*Struct*

Dynamic section entry.

**Generic Parameters:**
- E

**Fields:**
- `d_tag: crate::endian::U32<E>` - Dynamic entry type.
- `d_val: crate::endian::U32<E>` - Value (integer or address).

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Dyn32<E>`
- **Dyn**
  - `fn d_tag(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn d_val(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::Dyn64

*Struct*

Dynamic section entry.

**Generic Parameters:**
- E

**Fields:**
- `d_tag: crate::endian::U64<E>` - Dynamic entry type.
- `d_val: crate::endian::U64<E>` - Value (integer or address).

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Dyn64<E>`
- **Dyn**
  - `fn d_tag(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn d_val(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`



## object::elf::EFA_PARISC_1_0

*Constant*: `u32`

PA-RISC 1.0 big-endian.



## object::elf::EFA_PARISC_1_1

*Constant*: `u32`

PA-RISC 1.1 big-endian.



## object::elf::EFA_PARISC_2_0

*Constant*: `u32`

PA-RISC 2.0 big-endian.



## object::elf::EF_ALPHA_32BIT

*Constant*: `u32`

All addresses must be < 2GB.



## object::elf::EF_ALPHA_CANRELAX

*Constant*: `u32`

Relocations for relaxing exist.



## object::elf::EF_ARM_ABI_FLOAT_HARD

*Constant*: `u32`

NB conflicts with EF_ARM_VFP_FLOAT



## object::elf::EF_ARM_ABI_FLOAT_SOFT

*Constant*: `u32`

NB conflicts with EF_ARM_SOFT_FLOAT



## object::elf::EF_ARM_ALIGN8

*Constant*: `u32`

8-bit structure alignment is in use



## object::elf::EF_ARM_APCS_26

*Constant*: `u32`



## object::elf::EF_ARM_APCS_FLOAT

*Constant*: `u32`



## object::elf::EF_ARM_BE8

*Constant*: `u32`



## object::elf::EF_ARM_DYNSYMSUSESEGIDX

*Constant*: `u32`



## object::elf::EF_ARM_EABIMASK

*Constant*: `u32`



## object::elf::EF_ARM_EABI_UNKNOWN

*Constant*: `u32`



## object::elf::EF_ARM_EABI_VER1

*Constant*: `u32`



## object::elf::EF_ARM_EABI_VER2

*Constant*: `u32`



## object::elf::EF_ARM_EABI_VER3

*Constant*: `u32`



## object::elf::EF_ARM_EABI_VER4

*Constant*: `u32`



## object::elf::EF_ARM_EABI_VER5

*Constant*: `u32`



## object::elf::EF_ARM_HASENTRY

*Constant*: `u32`



## object::elf::EF_ARM_INTERWORK

*Constant*: `u32`



## object::elf::EF_ARM_LE8

*Constant*: `u32`



## object::elf::EF_ARM_MAPSYMSFIRST

*Constant*: `u32`



## object::elf::EF_ARM_MAVERICK_FLOAT

*Constant*: `u32`



## object::elf::EF_ARM_NEW_ABI

*Constant*: `u32`



## object::elf::EF_ARM_OLD_ABI

*Constant*: `u32`



## object::elf::EF_ARM_PIC

*Constant*: `u32`



## object::elf::EF_ARM_RELEXEC

*Constant*: `u32`



## object::elf::EF_ARM_SOFT_FLOAT

*Constant*: `u32`



## object::elf::EF_ARM_SYMSARESORTED

*Constant*: `u32`



## object::elf::EF_ARM_VFP_FLOAT

*Constant*: `u32`



## object::elf::EF_AVR_ARCH

*Constant*: `u32`

Bitmask for `EF_AVR_ARCH_*`.



## object::elf::EF_AVR_ARCH_AVR1

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_AVR2

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_AVR25

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_AVR3

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_AVR31

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_AVR35

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_AVR4

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_AVR5

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_AVR51

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_AVR6

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_AVRTINY

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_XMEGA1

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_XMEGA2

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_XMEGA3

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_XMEGA4

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_XMEGA5

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_XMEGA6

*Constant*: `u32`



## object::elf::EF_AVR_ARCH_XMEGA7

*Constant*: `u32`



## object::elf::EF_AVR_LINKRELAX_PREPARED

*Constant*: `u32`

If set, it is assumed that the elf file uses local symbols as reference
for the relocations so that linker relaxation is possible.



## object::elf::EF_CSKY_ABIMASK

*Constant*: `u32`



## object::elf::EF_CSKY_ABIV1

*Constant*: `u32`



## object::elf::EF_CSKY_ABIV2

*Constant*: `u32`



## object::elf::EF_CSKY_OTHER

*Constant*: `u32`



## object::elf::EF_CSKY_PROCESSOR

*Constant*: `u32`



## object::elf::EF_E2K_4MB_PAGES

*Constant*: `u32`



## object::elf::EF_E2K_INCOMPAT

*Constant*: `u32`



## object::elf::EF_E2K_IPD

*Constant*: `u32`



## object::elf::EF_E2K_PACK_SEGMENTS

*Constant*: `u32`



## object::elf::EF_E2K_PM

*Constant*: `u32`



## object::elf::EF_E2K_X86APP

*Constant*: `u32`



## object::elf::EF_IA_64_ABI64

*Constant*: `u32`

64-bit ABI



## object::elf::EF_IA_64_ARCH

*Constant*: `u32`

arch. version mask



## object::elf::EF_IA_64_MASKOS

*Constant*: `u32`

os-specific flags



## object::elf::EF_LARCH_ABI_DOUBLE_FLOAT

*Constant*: `u32`

Uses GPRs, 64-bit FPRs and the stack for parameter passing



## object::elf::EF_LARCH_ABI_MODIFIER_MASK

*Constant*: `u32`

Additional properties of the base ABI type, including the FP calling
convention.



## object::elf::EF_LARCH_ABI_SINGLE_FLOAT

*Constant*: `u32`

Uses GPRs, 32-bit FPRs and the stack for parameter passing



## object::elf::EF_LARCH_ABI_SOFT_FLOAT

*Constant*: `u32`

Uses GPRs and the stack for parameter passing



## object::elf::EF_LARCH_OBJABI_V1

*Constant*: `u32`

Uses relocation types directly writing to immediate slots



## object::elf::EF_MIPS_64BIT_WHIRL

*Constant*: `u32`



## object::elf::EF_MIPS_ABI

*Constant*: `u32`

Mask for selecting EF_MIPS_ABI_ variant



## object::elf::EF_MIPS_ABI2

*Constant*: `u32`



## object::elf::EF_MIPS_ABI_EABI32

*Constant*: `u32`

EABI in 32-bit mode



## object::elf::EF_MIPS_ABI_EABI64

*Constant*: `u32`

EABI in 64-bit mode



## object::elf::EF_MIPS_ABI_O32

*Constant*: `u32`

The first MIPS 32 bit ABI



## object::elf::EF_MIPS_ABI_O64

*Constant*: `u32`

O32 ABI extended for 64-bit architectures



## object::elf::EF_MIPS_ABI_ON32

*Constant*: `u32`



## object::elf::EF_MIPS_ARCH

*Constant*: `u32`

MIPS architecture level.



## object::elf::EF_MIPS_ARCH_1

*Constant*: `u32`

-mips1 code.



## object::elf::EF_MIPS_ARCH_2

*Constant*: `u32`

-mips2 code.



## object::elf::EF_MIPS_ARCH_3

*Constant*: `u32`

-mips3 code.



## object::elf::EF_MIPS_ARCH_32

*Constant*: `u32`

MIPS32 code.



## object::elf::EF_MIPS_ARCH_32R2

*Constant*: `u32`

MIPS32r2 code.



## object::elf::EF_MIPS_ARCH_32R6

*Constant*: `u32`

MIPS32r6 code



## object::elf::EF_MIPS_ARCH_4

*Constant*: `u32`

-mips4 code.



## object::elf::EF_MIPS_ARCH_5

*Constant*: `u32`

-mips5 code.



## object::elf::EF_MIPS_ARCH_64

*Constant*: `u32`

MIPS64 code.



## object::elf::EF_MIPS_ARCH_64R2

*Constant*: `u32`

MIPS64r2 code.



## object::elf::EF_MIPS_ARCH_64R6

*Constant*: `u32`

MIPS64r6 code



## object::elf::EF_MIPS_CPIC

*Constant*: `u32`

Uses PIC calling sequence.



## object::elf::EF_MIPS_FP64

*Constant*: `u32`

Uses FP64 (12 callee-saved).



## object::elf::EF_MIPS_NAN2008

*Constant*: `u32`

Uses IEEE 754-2008 NaN encoding.



## object::elf::EF_MIPS_NOREORDER

*Constant*: `u32`

A .noreorder directive was used.



## object::elf::EF_MIPS_PIC

*Constant*: `u32`

Contains PIC code.



## object::elf::EF_MIPS_XGOT

*Constant*: `u32`



## object::elf::EF_PARISC_ARCH

*Constant*: `u32`

Architecture version.



## object::elf::EF_PARISC_EXT

*Constant*: `u32`

Program uses arch. extensions.



## object::elf::EF_PARISC_LAZYSWAP

*Constant*: `u32`

Allow lazy swapping.



## object::elf::EF_PARISC_LSB

*Constant*: `u32`

Program expects little endian.



## object::elf::EF_PARISC_NO_KABP

*Constant*: `u32`

No kernel assisted branch prediction.



## object::elf::EF_PARISC_TRAPNIL

*Constant*: `u32`

Trap nil pointer dereference.



## object::elf::EF_PARISC_WIDE

*Constant*: `u32`

Program expects wide mode.



## object::elf::EF_PPC64_ABI

*Constant*: `u32`

PowerPC64 bits specifying ABI.

1 for original function descriptor using ABI,
2 for revised ABI without function descriptors,
0 for unspecified or not using any features affected by the differences.



## object::elf::EF_PPC_EMB

*Constant*: `u32`

PowerPC embedded flag



## object::elf::EF_PPC_RELOCATABLE

*Constant*: `u32`

PowerPC -mrelocatable flag



## object::elf::EF_PPC_RELOCATABLE_LIB

*Constant*: `u32`

PowerPC -mrelocatable-lib flag



## object::elf::EF_RISCV_FLOAT_ABI

*Constant*: `u32`



## object::elf::EF_RISCV_FLOAT_ABI_DOUBLE

*Constant*: `u32`



## object::elf::EF_RISCV_FLOAT_ABI_QUAD

*Constant*: `u32`



## object::elf::EF_RISCV_FLOAT_ABI_SINGLE

*Constant*: `u32`



## object::elf::EF_RISCV_FLOAT_ABI_SOFT

*Constant*: `u32`



## object::elf::EF_RISCV_RV64ILP32

*Constant*: `u32`



## object::elf::EF_RISCV_RVC

*Constant*: `u32`



## object::elf::EF_RISCV_RVE

*Constant*: `u32`



## object::elf::EF_RISCV_TSO

*Constant*: `u32`



## object::elf::EF_S390_HIGH_GPRS

*Constant*: `u32`

High GPRs kernel facility needed.



## object::elf::EF_SH1

*Constant*: `u32`



## object::elf::EF_SH2

*Constant*: `u32`



## object::elf::EF_SH2A

*Constant*: `u32`



## object::elf::EF_SH2A_NOFPU

*Constant*: `u32`



## object::elf::EF_SH2A_SH3E

*Constant*: `u32`



## object::elf::EF_SH2A_SH3_NOFPU

*Constant*: `u32`



## object::elf::EF_SH2A_SH4

*Constant*: `u32`



## object::elf::EF_SH2A_SH4_NOFPU

*Constant*: `u32`



## object::elf::EF_SH2E

*Constant*: `u32`



## object::elf::EF_SH3

*Constant*: `u32`



## object::elf::EF_SH3E

*Constant*: `u32`



## object::elf::EF_SH3_DSP

*Constant*: `u32`



## object::elf::EF_SH3_NOMMU

*Constant*: `u32`



## object::elf::EF_SH4

*Constant*: `u32`



## object::elf::EF_SH4A

*Constant*: `u32`



## object::elf::EF_SH4AL_DSP

*Constant*: `u32`



## object::elf::EF_SH4A_NOFPU

*Constant*: `u32`



## object::elf::EF_SH4_NOFPU

*Constant*: `u32`



## object::elf::EF_SH4_NOMMU_NOFPU

*Constant*: `u32`



## object::elf::EF_SH_DSP

*Constant*: `u32`



## object::elf::EF_SH_MACH_MASK

*Constant*: `u32`



## object::elf::EF_SH_UNKNOWN

*Constant*: `u32`



## object::elf::EF_SPARCV9_MM

*Constant*: `u32`



## object::elf::EF_SPARCV9_PSO

*Constant*: `u32`



## object::elf::EF_SPARCV9_RMO

*Constant*: `u32`



## object::elf::EF_SPARCV9_TSO

*Constant*: `u32`



## object::elf::EF_SPARC_32PLUS

*Constant*: `u32`

generic V8+ features



## object::elf::EF_SPARC_EXT_MASK

*Constant*: `u32`



## object::elf::EF_SPARC_HAL_R1

*Constant*: `u32`

HAL R1 extensions



## object::elf::EF_SPARC_LEDATA

*Constant*: `u32`

little endian data



## object::elf::EF_SPARC_SUN_US1

*Constant*: `u32`

Sun UltraSPARC1 extensions



## object::elf::EF_SPARC_SUN_US3

*Constant*: `u32`

Sun UltraSPARCIII extensions



## object::elf::ELFCLASS32

*Constant*: `u8`

32-bit object.



## object::elf::ELFCLASS64

*Constant*: `u8`

64-bit object.



## object::elf::ELFCLASSNONE

*Constant*: `u8`

Invalid class.



## object::elf::ELFCOMPRESS_HIOS

*Constant*: `u32`

End of OS-specific compression types.



## object::elf::ELFCOMPRESS_HIPROC

*Constant*: `u32`

End of processor-specific compression types.



## object::elf::ELFCOMPRESS_LOOS

*Constant*: `u32`

Start of OS-specific compression types.



## object::elf::ELFCOMPRESS_LOPROC

*Constant*: `u32`

Start of processor-specific compression types.



## object::elf::ELFCOMPRESS_ZLIB

*Constant*: `u32`

ZLIB/DEFLATE algorithm.



## object::elf::ELFCOMPRESS_ZSTD

*Constant*: `u32`

Zstandard algorithm.



## object::elf::ELFDATA2LSB

*Constant*: `u8`

2's complement, little endian.



## object::elf::ELFDATA2MSB

*Constant*: `u8`

2's complement, big endian.



## object::elf::ELFDATANONE

*Constant*: `u8`

Invalid data encoding.



## object::elf::ELFMAG

*Constant*: `[u8; 4]`

File identification bytes stored in `Ident::magic`.



## object::elf::ELFOSABI_AIX

*Constant*: `u8`

IBM AIX.



## object::elf::ELFOSABI_ARM

*Constant*: `u8`

ARM.



## object::elf::ELFOSABI_ARM_AEABI

*Constant*: `u8`

ARM EABI.



## object::elf::ELFOSABI_AROS

*Constant*: `u8`

AROS



## object::elf::ELFOSABI_CLOUDABI

*Constant*: `u8`

Nuxi CloudABI



## object::elf::ELFOSABI_FENIXOS

*Constant*: `u8`

FenixOS



## object::elf::ELFOSABI_FREEBSD

*Constant*: `u8`

FreeBSD.



## object::elf::ELFOSABI_GNU

*Constant*: `u8`

Object uses GNU ELF extensions.



## object::elf::ELFOSABI_HPUX

*Constant*: `u8`

HP-UX.



## object::elf::ELFOSABI_HURD

*Constant*: `u8`

GNU/Hurd.



## object::elf::ELFOSABI_IRIX

*Constant*: `u8`

SGI Irix.



## object::elf::ELFOSABI_LINUX

*Constant*: `u8`

Object uses GNU ELF extensions.

Compatibility alias.



## object::elf::ELFOSABI_MODESTO

*Constant*: `u8`

Novell Modesto.



## object::elf::ELFOSABI_NETBSD

*Constant*: `u8`

NetBSD.



## object::elf::ELFOSABI_NONE

*Constant*: `u8`

UNIX System V ABI.



## object::elf::ELFOSABI_NSK

*Constant*: `u8`

Hewlett-Packard Non-Stop Kernel.



## object::elf::ELFOSABI_OPENBSD

*Constant*: `u8`

OpenBSD.



## object::elf::ELFOSABI_OPENVMS

*Constant*: `u8`

OpenVMS.



## object::elf::ELFOSABI_SOLARIS

*Constant*: `u8`

Sun Solaris.



## object::elf::ELFOSABI_STANDALONE

*Constant*: `u8`

Standalone (embedded) application.



## object::elf::ELFOSABI_SYSV

*Constant*: `u8`

UNIX System V ABI.

Alias.



## object::elf::ELFOSABI_TRU64

*Constant*: `u8`

Compaq TRU64 UNIX.



## object::elf::ELF_NOTE_CORE

*Constant*: `&[u8]`

Note name for core files.



## object::elf::ELF_NOTE_GNU

*Constant*: `&[u8]`

GNU entries in the note section have this name.



## object::elf::ELF_NOTE_GO

*Constant*: `&[u8]`

Go entries in the note section have this name.



## object::elf::ELF_NOTE_LINUX

*Constant*: `&[u8]`

Note name for linux core files.

Notes in linux core files may also use `ELF_NOTE_CORE`.



## object::elf::ELF_NOTE_OS_FREEBSD

*Constant*: `u32`

OS descriptor for `NT_GNU_ABI_TAG`.



## object::elf::ELF_NOTE_OS_GNU

*Constant*: `u32`

OS descriptor for `NT_GNU_ABI_TAG`.



## object::elf::ELF_NOTE_OS_LINUX

*Constant*: `u32`

OS descriptor for `NT_GNU_ABI_TAG`.



## object::elf::ELF_NOTE_OS_SOLARIS2

*Constant*: `u32`

OS descriptor for `NT_GNU_ABI_TAG`.



## object::elf::ELF_NOTE_SOLARIS

*Constant*: `&[u8]`

Solaris entries in the note section have this name.



## object::elf::EM_386

*Constant*: `u16`

Intel 80386



## object::elf::EM_56800EX

*Constant*: `u16`

Freescale 56800EX DSC



## object::elf::EM_68HC05

*Constant*: `u16`

Motorola MC68HC05 microcontroller



## object::elf::EM_68HC08

*Constant*: `u16`

Motorola MC68HC08 microcontroller



## object::elf::EM_68HC11

*Constant*: `u16`

Motorola MC68HC11 microcontroller



## object::elf::EM_68HC12

*Constant*: `u16`

Motorola M68HC12



## object::elf::EM_68HC16

*Constant*: `u16`

Motorola MC68HC16 microcontroller



## object::elf::EM_68K

*Constant*: `u16`

Motorola m68k family



## object::elf::EM_78KOR

*Constant*: `u16`

Renesas 78KOR



## object::elf::EM_8051

*Constant*: `u16`

Intel 8051 and variants



## object::elf::EM_860

*Constant*: `u16`

Intel 80860



## object::elf::EM_88K

*Constant*: `u16`

Motorola m88k family



## object::elf::EM_960

*Constant*: `u16`

Intel 80960



## object::elf::EM_AARCH64

*Constant*: `u16`

ARM AARCH64



## object::elf::EM_ALPHA

*Constant*: `u16`

Digital Alpha



## object::elf::EM_ALTERA_NIOS2

*Constant*: `u16`

Altera Nios II



## object::elf::EM_AMDGPU

*Constant*: `u16`

AMD GPU



## object::elf::EM_ARC

*Constant*: `u16`

Argonaut RISC Core



## object::elf::EM_ARCA

*Constant*: `u16`

Arca RISC



## object::elf::EM_ARC_COMPACT

*Constant*: `u16`

ARC International ARCompact



## object::elf::EM_ARC_COMPACT2

*Constant*: `u16`

Synopsys ARCompact V2



## object::elf::EM_ARM

*Constant*: `u16`

ARM



## object::elf::EM_AVR

*Constant*: `u16`

Atmel AVR 8-bit microcontroller



## object::elf::EM_AVR32

*Constant*: `u16`

Amtel 32-bit microprocessor



## object::elf::EM_BA1

*Constant*: `u16`

Beyond BA1



## object::elf::EM_BA2

*Constant*: `u16`

Beyond BA2



## object::elf::EM_BLACKFIN

*Constant*: `u16`

Analog Devices Blackfin DSP



## object::elf::EM_BPF

*Constant*: `u16`

Linux BPF -- in-kernel virtual machine



## object::elf::EM_C166

*Constant*: `u16`

Infineon C16x/XC16x



## object::elf::EM_CDP

*Constant*: `u16`

Paneve CDP



## object::elf::EM_CE

*Constant*: `u16`

Freescale Communication Engine RISC



## object::elf::EM_CLOUDSHIELD

*Constant*: `u16`

CloudShield



## object::elf::EM_COGE

*Constant*: `u16`

Cognitive Smart Memory Processor



## object::elf::EM_COLDFIRE

*Constant*: `u16`

Motorola Coldfire



## object::elf::EM_COOL

*Constant*: `u16`

Bluechip CoolEngine



## object::elf::EM_COREA_1ST

*Constant*: `u16`

KIPO-KAIST Core-A 1st gen.



## object::elf::EM_COREA_2ND

*Constant*: `u16`

KIPO-KAIST Core-A 2nd gen.



## object::elf::EM_CR

*Constant*: `u16`

National Semi. CompactRISC



## object::elf::EM_CR16

*Constant*: `u16`

National Semi. CompactRISC CR16



## object::elf::EM_CRAYNV2

*Constant*: `u16`

Cray NV2 vector architecture



## object::elf::EM_CRIS

*Constant*: `u16`

Axis Communications 32-bit emb.proc



## object::elf::EM_CRX

*Constant*: `u16`

National Semi. CompactRISC CRX



## object::elf::EM_CSKY

*Constant*: `u16`

C-SKY



## object::elf::EM_CSR_KALIMBA

*Constant*: `u16`

CSR Kalimba



## object::elf::EM_CUDA

*Constant*: `u16`

NVIDIA CUDA



## object::elf::EM_CYPRESS_M8C

*Constant*: `u16`

Cypress M8C



## object::elf::EM_D10V

*Constant*: `u16`

Mitsubishi D10V



## object::elf::EM_D30V

*Constant*: `u16`

Mitsubishi D30V



## object::elf::EM_DSP24

*Constant*: `u16`

New Japan Radio (NJR) 24-bit DSP



## object::elf::EM_DSPIC30F

*Constant*: `u16`

Microchip Technology dsPIC30F



## object::elf::EM_DXP

*Constant*: `u16`

Icera Semi. Deep Execution Processor



## object::elf::EM_ECOG16

*Constant*: `u16`

Cyan Technology eCOG16



## object::elf::EM_ECOG1X

*Constant*: `u16`

Cyan Technology eCOG1X



## object::elf::EM_ECOG2

*Constant*: `u16`

Cyan Technology eCOG2



## object::elf::EM_EMX16

*Constant*: `u16`

KM211 KMX16



## object::elf::EM_EMX8

*Constant*: `u16`

KM211 KMX8



## object::elf::EM_ETPU

*Constant*: `u16`

Freescale Extended Time Processing Unit



## object::elf::EM_EXCESS

*Constant*: `u16`

eXcess configurable cpu



## object::elf::EM_F2MC16

*Constant*: `u16`

Fujitsu F2MC16



## object::elf::EM_FAKE_ALPHA

*Constant*: `u16`

Digital Alpha



## object::elf::EM_FIREPATH

*Constant*: `u16`

Element 14 64-bit DSP Processor



## object::elf::EM_FR20

*Constant*: `u16`

Fujitsu FR20



## object::elf::EM_FR30

*Constant*: `u16`

Fujitsu FR30



## object::elf::EM_FT32

*Constant*: `u16`

FTDI Chip FT32



## object::elf::EM_FX66

*Constant*: `u16`

Siemens FX66 microcontroller



## object::elf::EM_H8S

*Constant*: `u16`

Hitachi H8S



## object::elf::EM_H8_300

*Constant*: `u16`

Hitachi H8/300



## object::elf::EM_H8_300H

*Constant*: `u16`

Hitachi H8/300H



## object::elf::EM_H8_500

*Constant*: `u16`

Hitachi H8/500



## object::elf::EM_HEXAGON

*Constant*: `u16`

QUALCOMM Hexagon



## object::elf::EM_HUANY

*Constant*: `u16`

Harvard University machine-independent object files



## object::elf::EM_IAMCU

*Constant*: `u16`

Intel MCU



## object::elf::EM_IA_64

*Constant*: `u16`

Intel Merced



## object::elf::EM_IP2K

*Constant*: `u16`

Ubicom IP2xxx



## object::elf::EM_JAVELIN

*Constant*: `u16`

Infineon Technologies 32-bit emb.proc



## object::elf::EM_K10M

*Constant*: `u16`

Intel K10M



## object::elf::EM_KM32

*Constant*: `u16`

KM211 KM32



## object::elf::EM_KMX32

*Constant*: `u16`

KM211 KMX32



## object::elf::EM_KVARC

*Constant*: `u16`

KM211 KVARC



## object::elf::EM_L10M

*Constant*: `u16`

Intel L10M



## object::elf::EM_LATTICEMICO32

*Constant*: `u16`

RISC for Lattice FPGA



## object::elf::EM_LOONGARCH

*Constant*: `u16`

Loongson LoongArch



## object::elf::EM_M16C

*Constant*: `u16`

Renesas M16C



## object::elf::EM_M32

*Constant*: `u16`

AT&T WE 32100



## object::elf::EM_M32C

*Constant*: `u16`

Renesas M32C



## object::elf::EM_M32R

*Constant*: `u16`

Mitsubishi M32R



## object::elf::EM_MANIK

*Constant*: `u16`

M2000 Reconfigurable RISC



## object::elf::EM_MAX

*Constant*: `u16`

MAX processor



## object::elf::EM_MAXQ30

*Constant*: `u16`

Dallas Semi. MAXQ30 mc



## object::elf::EM_MCHP_PIC

*Constant*: `u16`

Microchip 8-bit PIC(r)



## object::elf::EM_MCST_ELBRUS

*Constant*: `u16`

MCST Elbrus



## object::elf::EM_ME16

*Constant*: `u16`

Toyota ME16 processor



## object::elf::EM_METAG

*Constant*: `u16`

Imagination Tech. META



## object::elf::EM_MICROBLAZE

*Constant*: `u16`

Xilinx MicroBlaze



## object::elf::EM_MIPS

*Constant*: `u16`

MIPS R3000 big-endian



## object::elf::EM_MIPS_RS3_LE

*Constant*: `u16`

MIPS R3000 little-endian



## object::elf::EM_MIPS_X

*Constant*: `u16`

Stanford MIPS-X



## object::elf::EM_MMA

*Constant*: `u16`

Fujitsu MMA Multimedia Accelerator



## object::elf::EM_MMDSP_PLUS

*Constant*: `u16`

STMicroelectronics 64bit VLIW DSP



## object::elf::EM_MMIX

*Constant*: `u16`

Donald Knuth's educational 64-bit proc



## object::elf::EM_MN10200

*Constant*: `u16`

Matsushita MN10200



## object::elf::EM_MN10300

*Constant*: `u16`

Matsushita MN10300



## object::elf::EM_MOXIE

*Constant*: `u16`

Moxie processor



## object::elf::EM_MSP430

*Constant*: `u16`

Texas Instruments msp430



## object::elf::EM_NCPU

*Constant*: `u16`

Sony nCPU embeeded RISC



## object::elf::EM_NDR1

*Constant*: `u16`

Denso NDR1 microprocessor



## object::elf::EM_NDS32

*Constant*: `u16`

Andes Tech. compact code emb. RISC



## object::elf::EM_NONE

*Constant*: `u16`

No machine



## object::elf::EM_NORC

*Constant*: `u16`

Nanoradio Optimized RISC



## object::elf::EM_NS32K

*Constant*: `u16`

National Semi. 32000



## object::elf::EM_OPEN8

*Constant*: `u16`

Open8 RISC



## object::elf::EM_OPENRISC

*Constant*: `u16`

OpenRISC 32-bit embedded processor



## object::elf::EM_PARISC

*Constant*: `u16`

HPPA



## object::elf::EM_PCP

*Constant*: `u16`

Siemens PCP



## object::elf::EM_PDP10

*Constant*: `u16`

Digital PDP-10



## object::elf::EM_PDP11

*Constant*: `u16`

Digital PDP-11



## object::elf::EM_PDSP

*Constant*: `u16`

Sony DSP Processor



## object::elf::EM_PJ

*Constant*: `u16`

picoJava



## object::elf::EM_PPC

*Constant*: `u16`

PowerPC



## object::elf::EM_PPC64

*Constant*: `u16`

PowerPC 64-bit



## object::elf::EM_PRISM

*Constant*: `u16`

SiTera Prism



## object::elf::EM_R32C

*Constant*: `u16`

Renesas R32C



## object::elf::EM_RCE

*Constant*: `u16`

Motorola RCE



## object::elf::EM_RH32

*Constant*: `u16`

TRW RH-32



## object::elf::EM_RISCV

*Constant*: `u16`

RISC-V



## object::elf::EM_RL78

*Constant*: `u16`

Renesas RL78



## object::elf::EM_RS08

*Constant*: `u16`

Freescale RS08



## object::elf::EM_RX

*Constant*: `u16`

Renesas RX



## object::elf::EM_S370

*Constant*: `u16`

IBM System/370



## object::elf::EM_S390

*Constant*: `u16`

IBM S390



## object::elf::EM_SBF

*Constant*: `u16`

Solana Binary Format



## object::elf::EM_SCORE7

*Constant*: `u16`

Sunplus S+core7 RISC



## object::elf::EM_SEP

*Constant*: `u16`

Sharp embedded microprocessor



## object::elf::EM_SE_C17

*Constant*: `u16`

Seiko Epson C17



## object::elf::EM_SE_C33

*Constant*: `u16`

Seiko Epson S1C33 family



## object::elf::EM_SH

*Constant*: `u16`

Hitachi SH



## object::elf::EM_SHARC

*Constant*: `u16`

Analog Devices SHARC family



## object::elf::EM_SLE9X

*Constant*: `u16`

Infineon Tech. SLE9X



## object::elf::EM_SNP1K

*Constant*: `u16`

Trebia SNP 1000



## object::elf::EM_SPARC

*Constant*: `u16`

SUN SPARC



## object::elf::EM_SPARC32PLUS

*Constant*: `u16`

Sun's "v8plus"



## object::elf::EM_SPARCV9

*Constant*: `u16`

SPARC v9 64-bit



## object::elf::EM_SPU

*Constant*: `u16`

IBM SPU/SPC



## object::elf::EM_ST100

*Constant*: `u16`

STMicroelectronic ST100 processor



## object::elf::EM_ST19

*Constant*: `u16`

STMicroelectronics ST19 8 bit mc



## object::elf::EM_ST200

*Constant*: `u16`

STMicroelectronics ST200



## object::elf::EM_ST7

*Constant*: `u16`

STmicroelectronics ST7 8 bit mc



## object::elf::EM_ST9PLUS

*Constant*: `u16`

STMicroelectronics ST9+ 8/16 mc



## object::elf::EM_STARCORE

*Constant*: `u16`

Motorola Start*Core processor



## object::elf::EM_STM8

*Constant*: `u16`

STMicroelectronics STM8



## object::elf::EM_STXP7X

*Constant*: `u16`

STMicroelectronics STxP7x



## object::elf::EM_SVX

*Constant*: `u16`

Silicon Graphics SVx



## object::elf::EM_TILE64

*Constant*: `u16`

Tileta TILE64



## object::elf::EM_TILEGX

*Constant*: `u16`

Tilera TILE-Gx



## object::elf::EM_TILEPRO

*Constant*: `u16`

Tilera TILEPro



## object::elf::EM_TINYJ

*Constant*: `u16`

Advanced Logic Corp. Tinyj emb.fam



## object::elf::EM_TI_ARP32

*Constant*: `u16`

Texas Instruments App. Specific RISC



## object::elf::EM_TI_C2000

*Constant*: `u16`

Texas Instruments TMS320C2000 DSP



## object::elf::EM_TI_C5500

*Constant*: `u16`

Texas Instruments TMS320C55x DSP



## object::elf::EM_TI_C6000

*Constant*: `u16`

Texas Instruments TMS320C6000 DSP



## object::elf::EM_TI_PRU

*Constant*: `u16`

Texas Instruments Prog. Realtime Unit



## object::elf::EM_TMM_GPP

*Constant*: `u16`

Thompson Multimedia General Purpose Proc



## object::elf::EM_TPC

*Constant*: `u16`

Tenor Network TPC



## object::elf::EM_TRICORE

*Constant*: `u16`

Siemens Tricore



## object::elf::EM_TRIMEDIA

*Constant*: `u16`

NXP Semi. TriMedia



## object::elf::EM_TSK3000

*Constant*: `u16`

Altium TSK3000



## object::elf::EM_UNICORE

*Constant*: `u16`

PKU-Unity & MPRC Peking Uni. mc series



## object::elf::EM_V800

*Constant*: `u16`

NEC V800 series



## object::elf::EM_V850

*Constant*: `u16`

NEC v850



## object::elf::EM_VAX

*Constant*: `u16`

Digital VAX



## object::elf::EM_VIDEOCORE

*Constant*: `u16`

Alphamosaic VideoCore



## object::elf::EM_VIDEOCORE3

*Constant*: `u16`

Broadcom VideoCore III



## object::elf::EM_VIDEOCORE5

*Constant*: `u16`

Broadcom VideoCore V



## object::elf::EM_VISIUM

*Constant*: `u16`

Controls and Data Services VISIUMcore



## object::elf::EM_VPP500

*Constant*: `u16`

Fujitsu VPP500



## object::elf::EM_X86_64

*Constant*: `u16`

AMD x86-64 architecture



## object::elf::EM_XCORE

*Constant*: `u16`

XMOS xCORE



## object::elf::EM_XGATE

*Constant*: `u16`

Motorola XGATE



## object::elf::EM_XIMO16

*Constant*: `u16`

New Japan Radio (NJR) 16-bit DSP



## object::elf::EM_XTENSA

*Constant*: `u16`

Tensilica Xtensa Architecture



## object::elf::EM_Z80

*Constant*: `u16`

Zilog Z80



## object::elf::EM_ZSP

*Constant*: `u16`

LSI Logic 16-bit DSP Processor



## object::elf::ET_CORE

*Constant*: `u16`

Core file.



## object::elf::ET_DYN

*Constant*: `u16`

Shared object file.



## object::elf::ET_EXEC

*Constant*: `u16`

Executable file.



## object::elf::ET_HIOS

*Constant*: `u16`

OS-specific range end.



## object::elf::ET_HIPROC

*Constant*: `u16`

Processor-specific range end.



## object::elf::ET_LOOS

*Constant*: `u16`

OS-specific range start.



## object::elf::ET_LOPROC

*Constant*: `u16`

Processor-specific range start.



## object::elf::ET_NONE

*Constant*: `u16`

No file type.



## object::elf::ET_REL

*Constant*: `u16`

Relocatable file.



## object::elf::EV_CURRENT

*Constant*: `u8`

Current ELF version.



## object::elf::EV_NONE

*Constant*: `u8`

Invalid ELF version.



## object::elf::E_E2K_MACH_12C

*Constant*: `u32`

-mtune=elbrus-12c code.



## object::elf::E_E2K_MACH_16C

*Constant*: `u32`

-mtune=elbrus-16c code.



## object::elf::E_E2K_MACH_1CPLUS

*Constant*: `u32`

-mtune=elbrus-1c+ code.



## object::elf::E_E2K_MACH_2C3

*Constant*: `u32`

-mtune=elbrus-2c3 code.



## object::elf::E_E2K_MACH_48C

*Constant*: `u32`

-mtune=elbrus-48c code.



## object::elf::E_E2K_MACH_8C

*Constant*: `u32`

-mtune=elbrus-8c code.



## object::elf::E_E2K_MACH_8V7

*Constant*: `u32`

-mtune=elbrus-8v7 code.



## object::elf::E_E2K_MACH_BASE

*Constant*: `u32`

-march=generic code.

Legacy. Shouldn't be created nowadays.



## object::elf::E_E2K_MACH_EV1

*Constant*: `u32`

-march=elbrus-v1 code.

Legacy. Shouldn't be created nowadays.



## object::elf::E_E2K_MACH_EV2

*Constant*: `u32`

-march=elbrus-v2 code.



## object::elf::E_E2K_MACH_EV3

*Constant*: `u32`

-march=elbrus-v3 code.



## object::elf::E_E2K_MACH_EV4

*Constant*: `u32`

-march=elbrus-v4 code.



## object::elf::E_E2K_MACH_EV5

*Constant*: `u32`

-march=elbrus-v5 code.



## object::elf::E_E2K_MACH_EV6

*Constant*: `u32`

-march=elbrus-v6 code.



## object::elf::E_E2K_MACH_EV7

*Constant*: `u32`

-march=elbrus-v7 code.



## object::elf::FileHeader32

*Struct*

The header at the start of every 32-bit ELF file.

**Generic Parameters:**
- E

**Fields:**
- `e_ident: Ident` - Magic number and other information.
- `e_type: crate::endian::U16<E>` - Object file type. One of the `ET_*` constants.
- `e_machine: crate::endian::U16<E>` - Architecture. One of the `EM_*` constants.
- `e_version: crate::endian::U32<E>` - Object file version. Must be `EV_CURRENT`.
- `e_entry: crate::endian::U32<E>` - Entry point virtual address.
- `e_phoff: crate::endian::U32<E>` - Program header table file offset.
- `e_shoff: crate::endian::U32<E>` - Section header table file offset.
- `e_flags: crate::endian::U32<E>` - Processor-specific flags.
- `e_ehsize: crate::endian::U16<E>` - Size in bytes of this header.
- `e_phentsize: crate::endian::U16<E>` - Program header table entry size.
- `e_phnum: crate::endian::U16<E>` - Program header table entry count.
- `e_shentsize: crate::endian::U16<E>` - Section header table entry size.
- `e_shnum: crate::endian::U16<E>` - Section header table entry count.
- `e_shstrndx: crate::endian::U16<E>` - Section header string table index.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FileHeader32<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FileHeader**
  - `fn is_type_64(self: &Self) -> bool`
  - `fn is_type_64_sized() -> bool`
  - `fn e_ident(self: &Self) -> &elf::Ident`
  - `fn e_type(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_machine(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_version(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn e_entry(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn e_phoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn e_shoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn e_flags(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn e_ehsize(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_phentsize(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_phnum(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_shentsize(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_shnum(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_shstrndx(self: &Self, endian: <Self as >::Endian) -> u16`



## object::elf::FileHeader64

*Struct*

The header at the start of every 64-bit ELF file.

**Generic Parameters:**
- E

**Fields:**
- `e_ident: Ident` - Magic number and other information.
- `e_type: crate::endian::U16<E>` - Object file type. One of the `ET_*` constants.
- `e_machine: crate::endian::U16<E>` - Architecture. One of the `EM_*` constants.
- `e_version: crate::endian::U32<E>` - Object file version. Must be `EV_CURRENT`.
- `e_entry: crate::endian::U64<E>` - Entry point virtual address.
- `e_phoff: crate::endian::U64<E>` - Program header table file offset.
- `e_shoff: crate::endian::U64<E>` - Section header table file offset.
- `e_flags: crate::endian::U32<E>` - Processor-specific flags.
- `e_ehsize: crate::endian::U16<E>` - Size in bytes of this header.
- `e_phentsize: crate::endian::U16<E>` - Program header table entry size.
- `e_phnum: crate::endian::U16<E>` - Program header table entry count.
- `e_shentsize: crate::endian::U16<E>` - Section header table entry size.
- `e_shnum: crate::endian::U16<E>` - Section header table entry count.
- `e_shstrndx: crate::endian::U16<E>` - Section header string table index.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FileHeader**
  - `fn is_type_64(self: &Self) -> bool`
  - `fn is_type_64_sized() -> bool`
  - `fn e_ident(self: &Self) -> &elf::Ident`
  - `fn e_type(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_machine(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_version(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn e_entry(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn e_phoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn e_shoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn e_flags(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn e_ehsize(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_phentsize(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_phnum(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_shentsize(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_shnum(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn e_shstrndx(self: &Self, endian: <Self as >::Endian) -> u16`
- **Clone**
  - `fn clone(self: &Self) -> FileHeader64<E>`



## object::elf::GNU_PROPERTY_1_NEEDED

*Constant*: `u32`

The needed properties by the object file.  */



## object::elf::GNU_PROPERTY_1_NEEDED_INDIRECT_EXTERN_ACCESS

*Constant*: `u32`

Set if the object file requires canonical function pointers and
cannot be used with copy relocation.



## object::elf::GNU_PROPERTY_AARCH64_FEATURE_1_AND

*Constant*: `u32`

AArch64 specific GNU properties.



## object::elf::GNU_PROPERTY_AARCH64_FEATURE_1_BTI

*Constant*: `u32`



## object::elf::GNU_PROPERTY_AARCH64_FEATURE_1_PAC

*Constant*: `u32`



## object::elf::GNU_PROPERTY_AARCH64_FEATURE_PAUTH

*Constant*: `u32`



## object::elf::GNU_PROPERTY_HIPROC

*Constant*: `u32`

Processor-specific semantics, hi



## object::elf::GNU_PROPERTY_HIUSER

*Constant*: `u32`

Application-specific semantics, hi



## object::elf::GNU_PROPERTY_LOPROC

*Constant*: `u32`

Processor-specific semantics, lo



## object::elf::GNU_PROPERTY_LOUSER

*Constant*: `u32`

Application-specific semantics, lo



## object::elf::GNU_PROPERTY_NO_COPY_ON_PROTECTED

*Constant*: `u32`

No copy relocation on protected data symbol.



## object::elf::GNU_PROPERTY_STACK_SIZE

*Constant*: `u32`

Stack size.



## object::elf::GNU_PROPERTY_UINT32_AND_HI

*Constant*: `u32`



## object::elf::GNU_PROPERTY_UINT32_AND_LO

*Constant*: `u32`



## object::elf::GNU_PROPERTY_UINT32_OR_HI

*Constant*: `u32`



## object::elf::GNU_PROPERTY_UINT32_OR_LO

*Constant*: `u32`



## object::elf::GNU_PROPERTY_X86_FEATURE_1_AND

*Constant*: `u32`

X86 processor-specific features used in program.



## object::elf::GNU_PROPERTY_X86_FEATURE_1_IBT

*Constant*: `u32`

This indicates that all executable sections are compatible with IBT.



## object::elf::GNU_PROPERTY_X86_FEATURE_1_SHSTK

*Constant*: `u32`

This indicates that all executable sections are compatible with SHSTK.



## object::elf::GNU_PROPERTY_X86_ISA_1_BASELINE

*Constant*: `u32`

GNU_PROPERTY_X86_ISA_1_BASELINE: CMOV, CX8 (cmpxchg8b), FPU (fld),
MMX, OSFXSR (fxsave), SCE (syscall), SSE and SSE2.



## object::elf::GNU_PROPERTY_X86_ISA_1_NEEDED

*Constant*: `u32`

The x86 instruction sets indicated by the corresponding bits are
used in program and they must be supported by the hardware.



## object::elf::GNU_PROPERTY_X86_ISA_1_USED

*Constant*: `u32`

The x86 instruction sets indicated by the corresponding bits are
used in program.  Their support in the hardware is optional.



## object::elf::GNU_PROPERTY_X86_ISA_1_V2

*Constant*: `u32`

GNU_PROPERTY_X86_ISA_1_V2: GNU_PROPERTY_X86_ISA_1_BASELINE,
CMPXCHG16B (cmpxchg16b), LAHF-SAHF (lahf), POPCNT (popcnt), SSE3,
SSSE3, SSE4.1 and SSE4.2.



## object::elf::GNU_PROPERTY_X86_ISA_1_V3

*Constant*: `u32`

GNU_PROPERTY_X86_ISA_1_V3: GNU_PROPERTY_X86_ISA_1_V2, AVX, AVX2, BMI1,
BMI2, F16C, FMA, LZCNT, MOVBE, XSAVE.



## object::elf::GNU_PROPERTY_X86_ISA_1_V4

*Constant*: `u32`

GNU_PROPERTY_X86_ISA_1_V4: GNU_PROPERTY_X86_ISA_1_V3, AVX512F,
AVX512BW, AVX512CD, AVX512DQ and AVX512VL.



## object::elf::GNU_PROPERTY_X86_UINT32_AND_HI

*Constant*: `u32`



## object::elf::GNU_PROPERTY_X86_UINT32_AND_LO

*Constant*: `u32`



## object::elf::GNU_PROPERTY_X86_UINT32_OR_AND_HI

*Constant*: `u32`



## object::elf::GNU_PROPERTY_X86_UINT32_OR_AND_LO

*Constant*: `u32`



## object::elf::GNU_PROPERTY_X86_UINT32_OR_HI

*Constant*: `u32`



## object::elf::GNU_PROPERTY_X86_UINT32_OR_LO

*Constant*: `u32`



## object::elf::GRP_COMDAT

*Constant*: `u32`

Mark group as COMDAT.



## object::elf::GnuHashHeader

*Struct*

Header of `SHT_GNU_HASH` section.

**Generic Parameters:**
- E

**Fields:**
- `bucket_count: crate::endian::U32<E>` - The number of hash buckets.
- `symbol_base: crate::endian::U32<E>` - The symbol table index of the first symbol in the hash.
- `bloom_count: crate::endian::U32<E>` - The number of words in the bloom filter.
- `bloom_shift: crate::endian::U32<E>` - The bit shift count for the bloom filter.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GnuHashHeader<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::HashHeader

*Struct*

Header of `SHT_HASH` section.

**Generic Parameters:**
- E

**Fields:**
- `bucket_count: crate::endian::U32<E>` - The number of hash buckets.
- `chain_count: crate::endian::U32<E>` - The number of chain values.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> HashHeader<E>`



## object::elf::Ident

*Struct*

Magic number and other information.

Contained in the file header.

**Fields:**
- `magic: [u8; 4]` - Magic number. Must be `ELFMAG`.
- `class: u8` - File class. One of the `ELFCLASS*` constants.
- `data: u8` - Data encoding. One of the `ELFDATA*` constants.
- `version: u8` - ELF version. Must be `EV_CURRENT`.
- `os_abi: u8` - OS ABI identification. One of the `ELFOSABI*` constants.
- `abi_version: u8` - ABI version.
- `padding: [u8; 7]` - Padding bytes.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Ident`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::LITUSE_ALPHA_ADDR

*Constant*: `u32`



## object::elf::LITUSE_ALPHA_BASE

*Constant*: `u32`



## object::elf::LITUSE_ALPHA_BYTOFF

*Constant*: `u32`



## object::elf::LITUSE_ALPHA_JSR

*Constant*: `u32`



## object::elf::LITUSE_ALPHA_TLS_GD

*Constant*: `u32`



## object::elf::LITUSE_ALPHA_TLS_LDM

*Constant*: `u32`



## object::elf::LL_DELAY_LOAD

*Constant*: `u32`



## object::elf::LL_DELTA

*Constant*: `u32`



## object::elf::LL_EXACT_MATCH

*Constant*: `u32`

Require exact match



## object::elf::LL_EXPORTS

*Constant*: `u32`



## object::elf::LL_IGNORE_INT_VER

*Constant*: `u32`

Ignore interface version



## object::elf::LL_NONE

*Constant*: `u32`



## object::elf::LL_REQUIRE_MINOR

*Constant*: `u32`



## object::elf::NT_386_IOPERM

*Constant*: `u32`

x86 io permission bitmap (1=deny).



## object::elf::NT_386_TLS

*Constant*: `u32`

i386 TLS slots (struct user_desc).



## object::elf::NT_ARM_HW_BREAK

*Constant*: `u32`

ARM hardware breakpoint registers.



## object::elf::NT_ARM_HW_WATCH

*Constant*: `u32`

ARM hardware watchpoint registers.



## object::elf::NT_ARM_SVE

*Constant*: `u32`

ARM Scalable Vector Extension registers.



## object::elf::NT_ARM_SYSTEM_CALL

*Constant*: `u32`

ARM system call number.



## object::elf::NT_ARM_TLS

*Constant*: `u32`

ARM TLS register.



## object::elf::NT_ARM_VFP

*Constant*: `u32`

ARM VFP/NEON registers.



## object::elf::NT_ASRS

*Constant*: `u32`

Contains copy of asrset struct.



## object::elf::NT_AUXV

*Constant*: `u32`

Contains copy of auxv array.



## object::elf::NT_FILE

*Constant*: `u32`

Contains information about mapped files.



## object::elf::NT_FPREGSET

*Constant*: `u32`

Contains copy of fpregset struct.



## object::elf::NT_GNU_ABI_TAG

*Constant*: `u32`

ABI information.

The descriptor consists of words:
- word 0: OS descriptor
- word 1: major version of the ABI
- word 2: minor version of the ABI
- word 3: subminor version of the ABI



## object::elf::NT_GNU_BUILD_ID

*Constant*: `u32`

Build ID bits as generated by `ld --build-id`.

The descriptor consists of any nonzero number of bytes.



## object::elf::NT_GNU_GOLD_VERSION

*Constant*: `u32`

Version note generated by GNU gold containing a version string.



## object::elf::NT_GNU_HWCAP

*Constant*: `u32`

Synthetic hwcap information.

The descriptor begins with two words:
- word 0: number of entries
- word 1: bitmask of enabled entries

Then follow variable-length entries, one byte followed by a
'\0'-terminated hwcap name string.  The byte gives the bit
number to test if enabled, (1U << bit) & bitmask.  */



## object::elf::NT_GNU_PROPERTY_TYPE_0

*Constant*: `u32`

Program property.



## object::elf::NT_GO_BUILD_ID

*Constant*: `u32`

Build ID bits as generated by Go's gc compiler.

The descriptor consists of any nonzero number of bytes.



## object::elf::NT_GWINDOWS

*Constant*: `u32`

Contains copy of gwindows struct.



## object::elf::NT_LWPSINFO

*Constant*: `u32`

Contains copy of lwpinfo struct.



## object::elf::NT_LWPSTATUS

*Constant*: `u32`

Contains copy of lwpstatus struct.



## object::elf::NT_MIPS_DSP

*Constant*: `u32`

MIPS DSP ASE registers.



## object::elf::NT_MIPS_FP_MODE

*Constant*: `u32`

MIPS floating-point mode.



## object::elf::NT_PLATFORM

*Constant*: `u32`

String from sysinfo(SI_PLATFORM).



## object::elf::NT_PPC_DSCR

*Constant*: `u32`

Data Stream Control Register.



## object::elf::NT_PPC_EBB

*Constant*: `u32`

Event Based Branch Registers.



## object::elf::NT_PPC_PKEY

*Constant*: `u32`

Memory Protection Keys registers.



## object::elf::NT_PPC_PMU

*Constant*: `u32`

Performance Monitor Registers.



## object::elf::NT_PPC_PPR

*Constant*: `u32`

Program Priority Register.



## object::elf::NT_PPC_SPE

*Constant*: `u32`

PowerPC SPE/EVR registers.



## object::elf::NT_PPC_TAR

*Constant*: `u32`

Target Address Register.



## object::elf::NT_PPC_TM_CDSCR

*Constant*: `u32`

TM checkpointed Data Stream Control Register.



## object::elf::NT_PPC_TM_CFPR

*Constant*: `u32`

TM checkpointed FPR Registers.



## object::elf::NT_PPC_TM_CGPR

*Constant*: `u32`

TM checkpointed GPR Registers.



## object::elf::NT_PPC_TM_CPPR

*Constant*: `u32`

TM checkpointed Program Priority Register.



## object::elf::NT_PPC_TM_CTAR

*Constant*: `u32`

TM checkpointed Target Address Register.



## object::elf::NT_PPC_TM_CVMX

*Constant*: `u32`

TM checkpointed VMX Registers.



## object::elf::NT_PPC_TM_CVSX

*Constant*: `u32`

TM checkpointed VSX Registers.



## object::elf::NT_PPC_TM_SPR

*Constant*: `u32`

TM Special Purpose Registers.



## object::elf::NT_PPC_VMX

*Constant*: `u32`

PowerPC Altivec/VMX registers.



## object::elf::NT_PPC_VSX

*Constant*: `u32`

PowerPC VSX registers.



## object::elf::NT_PRCRED

*Constant*: `u32`

Contains copy of prcred struct.



## object::elf::NT_PRFPREG

*Constant*: `u32`

Contains copy of fpregset struct.



## object::elf::NT_PRFPXREG

*Constant*: `u32`

Contains copy of fprxregset struct.



## object::elf::NT_PRPSINFO

*Constant*: `u32`

Contains copy of prpsinfo struct.



## object::elf::NT_PRSTATUS

*Constant*: `u32`

Contains copy of prstatus struct.



## object::elf::NT_PRXFPREG

*Constant*: `u32`

Contains copy of user_fxsr_struct.



## object::elf::NT_PRXREG

*Constant*: `u32`

Contains copy of prxregset struct.



## object::elf::NT_PSINFO

*Constant*: `u32`

Contains copy of psinfo struct.



## object::elf::NT_PSTATUS

*Constant*: `u32`

Contains copy of pstatus struct.



## object::elf::NT_S390_CTRS

*Constant*: `u32`

s390 control registers.



## object::elf::NT_S390_GS_BC

*Constant*: `u32`

s390 guarded storage broadcast control block.



## object::elf::NT_S390_GS_CB

*Constant*: `u32`

s390 guarded storage registers.



## object::elf::NT_S390_HIGH_GPRS

*Constant*: `u32`

s390 upper register halves.



## object::elf::NT_S390_LAST_BREAK

*Constant*: `u32`

s390 breaking event address.



## object::elf::NT_S390_PREFIX

*Constant*: `u32`

s390 prefix register.



## object::elf::NT_S390_RI_CB

*Constant*: `u32`

s390 runtime instrumentation.



## object::elf::NT_S390_SYSTEM_CALL

*Constant*: `u32`

s390 system call restart data.



## object::elf::NT_S390_TDB

*Constant*: `u32`

s390 transaction diagnostic block.



## object::elf::NT_S390_TIMER

*Constant*: `u32`

s390 timer register.



## object::elf::NT_S390_TODCMP

*Constant*: `u32`

s390 TOD clock comparator register.



## object::elf::NT_S390_TODPREG

*Constant*: `u32`

s390 TOD programmable register.



## object::elf::NT_S390_VXRS_HIGH

*Constant*: `u32`

s390 vector registers 16-31.



## object::elf::NT_S390_VXRS_LOW

*Constant*: `u32`

s390 vector registers 0-15 upper half.



## object::elf::NT_SIGINFO

*Constant*: `u32`

Contains copy of siginfo_t, size might increase.



## object::elf::NT_SOLARIS_PAGESIZE_HINT

*Constant*: `u32`

Desired pagesize for the binary.



## object::elf::NT_TASKSTRUCT

*Constant*: `u32`

Contains copy of task structure.



## object::elf::NT_UTSNAME

*Constant*: `u32`

Contains copy of utsname struct.



## object::elf::NT_VERSION

*Constant*: `u32`

Note type for version string.

This note may appear in object files.

It must be handled as a special case because it has no descriptor, and instead
uses the note name as the version string.



## object::elf::NT_VMCOREDD

*Constant*: `u32`

Vmcore Device Dump Note.



## object::elf::NT_X86_XSTATE

*Constant*: `u32`

x86 extended state using xsave.



## object::elf::NoteHeader32

*Struct*

Note section entry header.

A note consists of a header followed by a variable length name and descriptor.

**Generic Parameters:**
- E

**Fields:**
- `n_namesz: crate::endian::U32<E>` - Length of the note's name.
- `n_descsz: crate::endian::U32<E>` - Length of the note's descriptor.
- `n_type: crate::endian::U32<E>` - Type of the note.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **NoteHeader**
  - `fn n_namesz(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn n_descsz(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn n_type(self: &Self, endian: <Self as >::Endian) -> u32`
- **Clone**
  - `fn clone(self: &Self) -> NoteHeader32<E>`



## object::elf::NoteHeader64

*Struct*

Note section entry header.

**Generic Parameters:**
- E

**Fields:**
- `n_namesz: crate::endian::U32<E>` - Length of the note's name.
- `n_descsz: crate::endian::U32<E>` - Length of the note's descriptor.
- `n_type: crate::endian::U32<E>` - Type of the note.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> NoteHeader64<E>`
- **NoteHeader**
  - `fn n_namesz(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn n_descsz(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn n_type(self: &Self, endian: <Self as >::Endian) -> u32`



## object::elf::ODK_EXCEPTIONS

*Constant*: `u32`

Exception processing options.



## object::elf::ODK_FILL

*Constant*: `u32`

record the fill value used by the linker.



## object::elf::ODK_HWAND

*Constant*: `u32`

HW workarounds.  'AND' bits when merging.



## object::elf::ODK_HWOR

*Constant*: `u32`

HW workarounds.  'OR' bits when merging.



## object::elf::ODK_HWPATCH

*Constant*: `u32`

Hardware workarounds performed



## object::elf::ODK_NULL

*Constant*: `u32`

Undefined.



## object::elf::ODK_PAD

*Constant*: `u32`

Section padding options.



## object::elf::ODK_REGINFO

*Constant*: `u32`

Register usage information.



## object::elf::ODK_TAGS

*Constant*: `u32`

reserve space for desktop tools to write.



## object::elf::OEX_DISMISS

*Constant*: `u32`

Dismiss invalid address faults?



## object::elf::OEX_FPDBUG

*Constant*: `u32`

Force floating point debug mode?



## object::elf::OEX_FPU_DIV0

*Constant*: `u32`



## object::elf::OEX_FPU_INEX

*Constant*: `u32`



## object::elf::OEX_FPU_INVAL

*Constant*: `u32`



## object::elf::OEX_FPU_MAX

*Constant*: `u32`

FPE's which MAY be enabled.



## object::elf::OEX_FPU_MIN

*Constant*: `u32`

FPE's which MUST be enabled.



## object::elf::OEX_FPU_OFLO

*Constant*: `u32`



## object::elf::OEX_FPU_UFLO

*Constant*: `u32`



## object::elf::OEX_PAGE0

*Constant*: `u32`

page zero must be mapped.



## object::elf::OEX_PRECISEFP

*Constant*: `u32`



## object::elf::OEX_SMM

*Constant*: `u32`

Force sequential memory mode?



## object::elf::OHWA0_R4KEOP_CHECKED

*Constant*: `u32`



## object::elf::OHWA1_R4KEOP_CLEAN

*Constant*: `u32`



## object::elf::OHW_R4KEOP

*Constant*: `u32`

R4000 end-of-page patch.



## object::elf::OHW_R5KCVTL

*Constant*: `u32`

R5000 cvt.\[ds\].l bug.  clean=1.



## object::elf::OHW_R5KEOP

*Constant*: `u32`

R5000 end-of-page patch.



## object::elf::OHW_R8KPFETCH

*Constant*: `u32`

may need R8000 prefetch patch.



## object::elf::OPAD_POSTFIX

*Constant*: `u32`



## object::elf::OPAD_PREFIX

*Constant*: `u32`



## object::elf::OPAD_SYMBOL

*Constant*: `u32`



## object::elf::PF_ARM_ABS

*Constant*: `u32`

Absolute segment.



## object::elf::PF_ARM_PI

*Constant*: `u32`

Position-independent segment.



## object::elf::PF_ARM_SB

*Constant*: `u32`

Segment contains the location addressed by the static base.



## object::elf::PF_HP_CODE

*Constant*: `u32`



## object::elf::PF_HP_FAR_SHARED

*Constant*: `u32`



## object::elf::PF_HP_LAZYSWAP

*Constant*: `u32`



## object::elf::PF_HP_MODIFY

*Constant*: `u32`



## object::elf::PF_HP_NEAR_SHARED

*Constant*: `u32`



## object::elf::PF_HP_PAGE_SIZE

*Constant*: `u32`



## object::elf::PF_HP_SBP

*Constant*: `u32`



## object::elf::PF_IA_64_NORECOV

*Constant*: `u32`

spec insns w/o recovery



## object::elf::PF_MASKOS

*Constant*: `u32`

OS-specific segment flags.



## object::elf::PF_MASKPROC

*Constant*: `u32`

Processor-specific segment flags.



## object::elf::PF_MIPS_LOCAL

*Constant*: `u32`



## object::elf::PF_PARISC_SBP

*Constant*: `u32`



## object::elf::PF_R

*Constant*: `u32`

Segment is readable.



## object::elf::PF_W

*Constant*: `u32`

Segment is writable.



## object::elf::PF_X

*Constant*: `u32`

Segment is executable.



## object::elf::PN_XNUM

*Constant*: `u16`

Special value for `FileHeader*::e_phnum`.

This indicates that the real number of program headers is too large to fit into e_phnum.
Instead the real value is in the field `sh_info` of section 0.



## object::elf::PPC64_OPT_LOCALENTRY

*Constant*: `u32`



## object::elf::PPC64_OPT_MULTI_TOC

*Constant*: `u32`



## object::elf::PPC64_OPT_TLS

*Constant*: `u32`



## object::elf::PPC_OPT_TLS

*Constant*: `u32`



## object::elf::PT_ARM_EXIDX

*Constant*: `u32`

ARM unwind segment.



## object::elf::PT_DYNAMIC

*Constant*: `u32`

Dynamic linking information.



## object::elf::PT_GNU_EH_FRAME

*Constant*: `u32`

GCC `.eh_frame_hdr` segment.



## object::elf::PT_GNU_PROPERTY

*Constant*: `u32`

Segment containing `.note.gnu.property` section.



## object::elf::PT_GNU_RELRO

*Constant*: `u32`

Read-only after relocation.



## object::elf::PT_GNU_SFRAME

*Constant*: `u32`

GNU SFrame stack trace format.



## object::elf::PT_GNU_STACK

*Constant*: `u32`

Indicates stack executability.



## object::elf::PT_HIOS

*Constant*: `u32`

End of OS-specific segment types.



## object::elf::PT_HIPROC

*Constant*: `u32`

End of processor-specific segment types.



## object::elf::PT_HP_CORE_COMM

*Constant*: `u32`



## object::elf::PT_HP_CORE_KERNEL

*Constant*: `u32`



## object::elf::PT_HP_CORE_LOADABLE

*Constant*: `u32`



## object::elf::PT_HP_CORE_MMF

*Constant*: `u32`



## object::elf::PT_HP_CORE_NONE

*Constant*: `u32`



## object::elf::PT_HP_CORE_PROC

*Constant*: `u32`



## object::elf::PT_HP_CORE_SHM

*Constant*: `u32`



## object::elf::PT_HP_CORE_STACK

*Constant*: `u32`



## object::elf::PT_HP_CORE_VERSION

*Constant*: `u32`



## object::elf::PT_HP_FASTBIND

*Constant*: `u32`



## object::elf::PT_HP_HSL_ANNOT

*Constant*: `u32`



## object::elf::PT_HP_OPT_ANNOT

*Constant*: `u32`



## object::elf::PT_HP_PARALLEL

*Constant*: `u32`



## object::elf::PT_HP_STACK

*Constant*: `u32`



## object::elf::PT_HP_TLS

*Constant*: `u32`



## object::elf::PT_IA_64_ARCHEXT

*Constant*: `u32`

arch extension bits



## object::elf::PT_IA_64_HP_HSL_ANOT

*Constant*: `u32`



## object::elf::PT_IA_64_HP_OPT_ANOT

*Constant*: `u32`



## object::elf::PT_IA_64_HP_STACK

*Constant*: `u32`



## object::elf::PT_IA_64_UNWIND

*Constant*: `u32`

ia64 unwind bits



## object::elf::PT_INTERP

*Constant*: `u32`

Program interpreter.



## object::elf::PT_LOAD

*Constant*: `u32`

Loadable program segment.



## object::elf::PT_LOOS

*Constant*: `u32`

Start of OS-specific segment types.



## object::elf::PT_LOPROC

*Constant*: `u32`

Start of processor-specific segment types.



## object::elf::PT_MIPS_ABIFLAGS

*Constant*: `u32`

FP mode requirement.



## object::elf::PT_MIPS_OPTIONS

*Constant*: `u32`



## object::elf::PT_MIPS_REGINFO

*Constant*: `u32`

Register usage information.



## object::elf::PT_MIPS_RTPROC

*Constant*: `u32`

Runtime procedure table.



## object::elf::PT_NOTE

*Constant*: `u32`

Auxiliary information.



## object::elf::PT_NULL

*Constant*: `u32`

Program header table entry is unused.



## object::elf::PT_PARISC_ARCHEXT

*Constant*: `u32`



## object::elf::PT_PARISC_UNWIND

*Constant*: `u32`



## object::elf::PT_PHDR

*Constant*: `u32`

Segment contains the program header table.



## object::elf::PT_RISCV_ATTRIBUTES

*Constant*: `u32`



## object::elf::PT_SHLIB

*Constant*: `u32`

Reserved.



## object::elf::PT_TLS

*Constant*: `u32`

Thread-local storage segment.



## object::elf::ProgramHeader32

*Struct*

Program segment header.

**Generic Parameters:**
- E

**Fields:**
- `p_type: crate::endian::U32<E>` - Segment type. One of the `PT_*` constants.
- `p_offset: crate::endian::U32<E>` - Segment file offset.
- `p_vaddr: crate::endian::U32<E>` - Segment virtual address.
- `p_paddr: crate::endian::U32<E>` - Segment physical address.
- `p_filesz: crate::endian::U32<E>` - Segment size in the file.
- `p_memsz: crate::endian::U32<E>` - Segment size in memory.
- `p_flags: crate::endian::U32<E>` - Segment flags. A combination of the `PF_*` constants.
- `p_align: crate::endian::U32<E>` - Segment alignment.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ProgramHeader32<E>`
- **ProgramHeader**
  - `fn p_type(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn p_flags(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn p_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn p_vaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn p_paddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn p_filesz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn p_memsz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn p_align(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::ProgramHeader64

*Struct*

Program segment header.

**Generic Parameters:**
- E

**Fields:**
- `p_type: crate::endian::U32<E>` - Segment type. One of the `PT_*` constants.
- `p_flags: crate::endian::U32<E>` - Segment flags. A combination of the `PF_*` constants.
- `p_offset: crate::endian::U64<E>` - Segment file offset.
- `p_vaddr: crate::endian::U64<E>` - Segment virtual address.
- `p_paddr: crate::endian::U64<E>` - Segment physical address.
- `p_filesz: crate::endian::U64<E>` - Segment size in the file.
- `p_memsz: crate::endian::U64<E>` - Segment size in memory.
- `p_align: crate::endian::U64<E>` - Segment alignment.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ProgramHeader64<E>`
- **ProgramHeader**
  - `fn p_type(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn p_flags(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn p_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn p_vaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn p_paddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn p_filesz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn p_memsz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn p_align(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`



## object::elf::RHF_CORD

*Constant*: `u32`



## object::elf::RHF_DEFAULT_DELAY_LOAD

*Constant*: `u32`



## object::elf::RHF_DELTA_C_PLUS_PLUS

*Constant*: `u32`



## object::elf::RHF_GUARANTEE_INIT

*Constant*: `u32`



## object::elf::RHF_GUARANTEE_START_INIT

*Constant*: `u32`



## object::elf::RHF_NONE

*Constant*: `u32`

No flags



## object::elf::RHF_NOTPOT

*Constant*: `u32`

Hash size not power of 2



## object::elf::RHF_NO_LIBRARY_REPLACEMENT

*Constant*: `u32`

Ignore LD_LIBRARY_PATH



## object::elf::RHF_NO_MOVE

*Constant*: `u32`



## object::elf::RHF_NO_UNRES_UNDEF

*Constant*: `u32`



## object::elf::RHF_PIXIE

*Constant*: `u32`



## object::elf::RHF_QUICKSTART

*Constant*: `u32`

Use quickstart



## object::elf::RHF_REQUICKSTART

*Constant*: `u32`



## object::elf::RHF_REQUICKSTARTED

*Constant*: `u32`



## object::elf::RHF_RLD_ORDER_SAFE

*Constant*: `u32`



## object::elf::RHF_SGI_ONLY

*Constant*: `u32`



## object::elf::R_386_16

*Constant*: `u32`

Direct 16 bit



## object::elf::R_386_32

*Constant*: `u32`

Direct 32 bit



## object::elf::R_386_32PLT

*Constant*: `u32`

Direct 32 bit PLT address



## object::elf::R_386_8

*Constant*: `u32`

Direct 8 bit



## object::elf::R_386_COPY

*Constant*: `u32`

Copy symbol at runtime



## object::elf::R_386_GLOB_DAT

*Constant*: `u32`

Create GOT entry



## object::elf::R_386_GOT32

*Constant*: `u32`

32 bit GOT entry



## object::elf::R_386_GOT32X

*Constant*: `u32`

Load from 32 bit GOT entry, relaxable.



## object::elf::R_386_GOTOFF

*Constant*: `u32`

32 bit offset to GOT



## object::elf::R_386_GOTPC

*Constant*: `u32`

32 bit PC relative offset to GOT



## object::elf::R_386_IRELATIVE

*Constant*: `u32`

Adjust indirectly by program base



## object::elf::R_386_JMP_SLOT

*Constant*: `u32`

Create PLT entry



## object::elf::R_386_NONE

*Constant*: `u32`

No reloc



## object::elf::R_386_PC16

*Constant*: `u32`

PC relative 16 bit



## object::elf::R_386_PC32

*Constant*: `u32`

PC relative 32 bit



## object::elf::R_386_PC8

*Constant*: `u32`

PC relative 8 bit



## object::elf::R_386_PLT32

*Constant*: `u32`

32 bit PLT address



## object::elf::R_386_RELATIVE

*Constant*: `u32`

Adjust by program base



## object::elf::R_386_SIZE32

*Constant*: `u32`

32-bit symbol size



## object::elf::R_386_TLS_DESC

*Constant*: `u32`

TLS descriptor containing pointer to code and to argument, returning the TLS offset for the symbol.



## object::elf::R_386_TLS_DESC_CALL

*Constant*: `u32`

Marker of call through TLS descriptor for relaxation.



## object::elf::R_386_TLS_DTPMOD32

*Constant*: `u32`

ID of module containing symbol



## object::elf::R_386_TLS_DTPOFF32

*Constant*: `u32`

Offset in TLS block



## object::elf::R_386_TLS_GD

*Constant*: `u32`

Direct 32 bit for GNU version of general dynamic thread local data



## object::elf::R_386_TLS_GD_32

*Constant*: `u32`

Direct 32 bit for general dynamic thread local data



## object::elf::R_386_TLS_GD_CALL

*Constant*: `u32`

Relocation for call to __tls_get_addr()



## object::elf::R_386_TLS_GD_POP

*Constant*: `u32`

Tag for popl in GD TLS code



## object::elf::R_386_TLS_GD_PUSH

*Constant*: `u32`

Tag for pushl in GD TLS code



## object::elf::R_386_TLS_GOTDESC

*Constant*: `u32`

GOT offset for TLS descriptor.



## object::elf::R_386_TLS_GOTIE

*Constant*: `u32`

GOT entry for static TLS block offset



## object::elf::R_386_TLS_IE

*Constant*: `u32`

Address of GOT entry for static TLS block offset



## object::elf::R_386_TLS_IE_32

*Constant*: `u32`

GOT entry for negated static TLS block offset



## object::elf::R_386_TLS_LDM

*Constant*: `u32`

Direct 32 bit for GNU version of local dynamic thread local data in LE code



## object::elf::R_386_TLS_LDM_32

*Constant*: `u32`

Direct 32 bit for local dynamic thread local data in LE code



## object::elf::R_386_TLS_LDM_CALL

*Constant*: `u32`

Relocation for call to __tls_get_addr() in LDM code



## object::elf::R_386_TLS_LDM_POP

*Constant*: `u32`

Tag for popl in LDM TLS code



## object::elf::R_386_TLS_LDM_PUSH

*Constant*: `u32`

Tag for pushl in LDM TLS code



## object::elf::R_386_TLS_LDO_32

*Constant*: `u32`

Offset relative to TLS block



## object::elf::R_386_TLS_LE

*Constant*: `u32`

Offset relative to static TLS block



## object::elf::R_386_TLS_LE_32

*Constant*: `u32`

Negated offset relative to static TLS block



## object::elf::R_386_TLS_TPOFF

*Constant*: `u32`

Offset in static TLS block



## object::elf::R_386_TLS_TPOFF32

*Constant*: `u32`

Negated offset in static TLS block



## object::elf::R_390_12

*Constant*: `u32`

Direct 12 bit.



## object::elf::R_390_16

*Constant*: `u32`

Direct 16 bit.



## object::elf::R_390_20

*Constant*: `u32`

Direct 20 bit.



## object::elf::R_390_32

*Constant*: `u32`

Direct 32 bit.



## object::elf::R_390_64

*Constant*: `u32`

Direct 64 bit.



## object::elf::R_390_8

*Constant*: `u32`

Direct 8 bit.



## object::elf::R_390_COPY

*Constant*: `u32`

Copy symbol at runtime.



## object::elf::R_390_GLOB_DAT

*Constant*: `u32`

Create GOT entry.



## object::elf::R_390_GOT12

*Constant*: `u32`

12 bit GOT offset.



## object::elf::R_390_GOT16

*Constant*: `u32`

16 bit GOT offset.



## object::elf::R_390_GOT20

*Constant*: `u32`

20 bit GOT offset.



## object::elf::R_390_GOT32

*Constant*: `u32`

32 bit GOT offset.



## object::elf::R_390_GOT64

*Constant*: `u32`

64 bit GOT offset.



## object::elf::R_390_GOTENT

*Constant*: `u32`

32 bit PC rel. to GOT entry >> 1.



## object::elf::R_390_GOTOFF16

*Constant*: `u32`

16 bit offset to GOT.



## object::elf::R_390_GOTOFF32

*Constant*: `u32`

32 bit offset to GOT.



## object::elf::R_390_GOTOFF64

*Constant*: `u32`

64 bit offset to GOT.



## object::elf::R_390_GOTPC

*Constant*: `u32`

32 bit PC relative offset to GOT.



## object::elf::R_390_GOTPCDBL

*Constant*: `u32`

32 bit PC rel. GOT shifted by 1.



## object::elf::R_390_GOTPLT12

*Constant*: `u32`

12 bit offset to jump slot.



## object::elf::R_390_GOTPLT16

*Constant*: `u32`

16 bit offset to jump slot.



## object::elf::R_390_GOTPLT20

*Constant*: `u32`

20 bit offset to jump slot.



## object::elf::R_390_GOTPLT32

*Constant*: `u32`

32 bit offset to jump slot.



## object::elf::R_390_GOTPLT64

*Constant*: `u32`

64 bit offset to jump slot.



## object::elf::R_390_GOTPLTENT

*Constant*: `u32`

32 bit rel. offset to jump slot.



## object::elf::R_390_IRELATIVE

*Constant*: `u32`

STT_GNU_IFUNC relocation.



## object::elf::R_390_JMP_SLOT

*Constant*: `u32`

Create PLT entry.



## object::elf::R_390_NONE

*Constant*: `u32`

No reloc.



## object::elf::R_390_PC16

*Constant*: `u32`

PC relative 16 bit.



## object::elf::R_390_PC16DBL

*Constant*: `u32`

PC relative 16 bit shifted by 1.



## object::elf::R_390_PC32

*Constant*: `u32`

PC relative 32 bit.



## object::elf::R_390_PC32DBL

*Constant*: `u32`

PC relative 32 bit shifted by 1.



## object::elf::R_390_PC64

*Constant*: `u32`

PC relative 64 bit.



## object::elf::R_390_PLT16DBL

*Constant*: `u32`

16 bit PC rel. PLT shifted by 1.



## object::elf::R_390_PLT32

*Constant*: `u32`

32 bit PC relative PLT address.



## object::elf::R_390_PLT32DBL

*Constant*: `u32`

32 bit PC rel. PLT shifted by 1.



## object::elf::R_390_PLT64

*Constant*: `u32`

64 bit PC relative PLT address.



## object::elf::R_390_PLTOFF16

*Constant*: `u32`

16 bit offset from GOT to PLT.



## object::elf::R_390_PLTOFF32

*Constant*: `u32`

32 bit offset from GOT to PLT.



## object::elf::R_390_PLTOFF64

*Constant*: `u32`

16 bit offset from GOT to PLT.



## object::elf::R_390_RELATIVE

*Constant*: `u32`

Adjust by program base.



## object::elf::R_390_TLS_DTPMOD

*Constant*: `u32`

ID of module containing symbol.



## object::elf::R_390_TLS_DTPOFF

*Constant*: `u32`

Offset in TLS block.



## object::elf::R_390_TLS_GD32

*Constant*: `u32`

Direct 32 bit for general dynamic thread local data.



## object::elf::R_390_TLS_GD64

*Constant*: `u32`

Direct 64 bit for general dynamic thread local data.



## object::elf::R_390_TLS_GDCALL

*Constant*: `u32`

Tag for function call in general dynamic TLS code.



## object::elf::R_390_TLS_GOTIE12

*Constant*: `u32`

12 bit GOT offset for static TLS block offset.



## object::elf::R_390_TLS_GOTIE20

*Constant*: `u32`

20 bit GOT offset for static TLS block offset.



## object::elf::R_390_TLS_GOTIE32

*Constant*: `u32`

32 bit GOT offset for static TLS block offset.



## object::elf::R_390_TLS_GOTIE64

*Constant*: `u32`

64 bit GOT offset for static TLS block offset.



## object::elf::R_390_TLS_IE32

*Constant*: `u32`

32 bit address of GOT entry for negated static TLS block offset.



## object::elf::R_390_TLS_IE64

*Constant*: `u32`

64 bit address of GOT entry for negated static TLS block offset.



## object::elf::R_390_TLS_IEENT

*Constant*: `u32`

32 bit rel. offset to GOT entry for negated static TLS block offset.



## object::elf::R_390_TLS_LDCALL

*Constant*: `u32`

Tag for function call in local dynamic TLS code.



## object::elf::R_390_TLS_LDM32

*Constant*: `u32`

Direct 32 bit for local dynamic thread local data in LE code.



## object::elf::R_390_TLS_LDM64

*Constant*: `u32`

Direct 64 bit for local dynamic thread local data in LE code.



## object::elf::R_390_TLS_LDO32

*Constant*: `u32`

32 bit offset relative to TLS block.



## object::elf::R_390_TLS_LDO64

*Constant*: `u32`

64 bit offset relative to TLS block.



## object::elf::R_390_TLS_LE32

*Constant*: `u32`

32 bit negated offset relative to static TLS block.



## object::elf::R_390_TLS_LE64

*Constant*: `u32`

64 bit negated offset relative to static TLS block.



## object::elf::R_390_TLS_LOAD

*Constant*: `u32`

Tag for load insn in TLS code.



## object::elf::R_390_TLS_TPOFF

*Constant*: `u32`

Negated offset in static TLS block.



## object::elf::R_68K_16

*Constant*: `u32`

Direct 16 bit



## object::elf::R_68K_32

*Constant*: `u32`

Direct 32 bit



## object::elf::R_68K_8

*Constant*: `u32`

Direct 8 bit



## object::elf::R_68K_COPY

*Constant*: `u32`

Copy symbol at runtime



## object::elf::R_68K_GLOB_DAT

*Constant*: `u32`

Create GOT entry



## object::elf::R_68K_GOT16

*Constant*: `u32`

16 bit PC relative GOT entry



## object::elf::R_68K_GOT16O

*Constant*: `u32`

16 bit GOT offset



## object::elf::R_68K_GOT32

*Constant*: `u32`

32 bit PC relative GOT entry



## object::elf::R_68K_GOT32O

*Constant*: `u32`

32 bit GOT offset



## object::elf::R_68K_GOT8

*Constant*: `u32`

8 bit PC relative GOT entry



## object::elf::R_68K_GOT8O

*Constant*: `u32`

8 bit GOT offset



## object::elf::R_68K_JMP_SLOT

*Constant*: `u32`

Create PLT entry



## object::elf::R_68K_NONE

*Constant*: `u32`

No reloc



## object::elf::R_68K_PC16

*Constant*: `u32`

PC relative 16 bit



## object::elf::R_68K_PC32

*Constant*: `u32`

PC relative 32 bit



## object::elf::R_68K_PC8

*Constant*: `u32`

PC relative 8 bit



## object::elf::R_68K_PLT16

*Constant*: `u32`

16 bit PC relative PLT address



## object::elf::R_68K_PLT16O

*Constant*: `u32`

16 bit PLT offset



## object::elf::R_68K_PLT32

*Constant*: `u32`

32 bit PC relative PLT address



## object::elf::R_68K_PLT32O

*Constant*: `u32`

32 bit PLT offset



## object::elf::R_68K_PLT8

*Constant*: `u32`

8 bit PC relative PLT address



## object::elf::R_68K_PLT8O

*Constant*: `u32`

8 bit PLT offset



## object::elf::R_68K_RELATIVE

*Constant*: `u32`

Adjust by program base



## object::elf::R_68K_TLS_DTPMOD32

*Constant*: `u32`

32 bit module number



## object::elf::R_68K_TLS_DTPREL32

*Constant*: `u32`

32 bit module-relative offset



## object::elf::R_68K_TLS_GD16

*Constant*: `u32`

16 bit GOT offset for GD



## object::elf::R_68K_TLS_GD32

*Constant*: `u32`

32 bit GOT offset for GD



## object::elf::R_68K_TLS_GD8

*Constant*: `u32`

8 bit GOT offset for GD



## object::elf::R_68K_TLS_IE16

*Constant*: `u32`

16 bit GOT offset for IE



## object::elf::R_68K_TLS_IE32

*Constant*: `u32`

32 bit GOT offset for IE



## object::elf::R_68K_TLS_IE8

*Constant*: `u32`

8 bit GOT offset for IE



## object::elf::R_68K_TLS_LDM16

*Constant*: `u32`

16 bit GOT offset for LDM



## object::elf::R_68K_TLS_LDM32

*Constant*: `u32`

32 bit GOT offset for LDM



## object::elf::R_68K_TLS_LDM8

*Constant*: `u32`

8 bit GOT offset for LDM



## object::elf::R_68K_TLS_LDO16

*Constant*: `u32`

16 bit module-relative offset



## object::elf::R_68K_TLS_LDO32

*Constant*: `u32`

32 bit module-relative offset



## object::elf::R_68K_TLS_LDO8

*Constant*: `u32`

8 bit module-relative offset



## object::elf::R_68K_TLS_LE16

*Constant*: `u32`

16 bit offset relative to static TLS block



## object::elf::R_68K_TLS_LE32

*Constant*: `u32`

32 bit offset relative to static TLS block



## object::elf::R_68K_TLS_LE8

*Constant*: `u32`

8 bit offset relative to static TLS block



## object::elf::R_68K_TLS_TPREL32

*Constant*: `u32`

32 bit TP-relative offset



## object::elf::R_AARCH64_ABS16

*Constant*: `u32`

Direct 16-bit.



## object::elf::R_AARCH64_ABS32

*Constant*: `u32`

Direct 32 bit.



## object::elf::R_AARCH64_ABS64

*Constant*: `u32`

Direct 64 bit.



## object::elf::R_AARCH64_ADD_ABS_LO12_NC

*Constant*: `u32`

Dir. ADD imm. from bits 11:0.



## object::elf::R_AARCH64_ADR_GOT_PAGE

*Constant*: `u32`

P-page-rel. GOT off. ADRP 32:12.



## object::elf::R_AARCH64_ADR_PREL_LO21

*Constant*: `u32`

PC-rel. ADR imm. from bits 20:0.



## object::elf::R_AARCH64_ADR_PREL_PG_HI21

*Constant*: `u32`

Page-rel. ADRP imm. from 32:12.



## object::elf::R_AARCH64_ADR_PREL_PG_HI21_NC

*Constant*: `u32`

Likewise; no overflow check.



## object::elf::R_AARCH64_CALL26

*Constant*: `u32`

Likewise for CALL.



## object::elf::R_AARCH64_CONDBR19

*Constant*: `u32`

PC-rel. cond. br. imm. from 20:2.



## object::elf::R_AARCH64_COPY

*Constant*: `u32`

Copy symbol at runtime.



## object::elf::R_AARCH64_GLOB_DAT

*Constant*: `u32`

Create GOT entry.



## object::elf::R_AARCH64_GOTREL32

*Constant*: `u32`

GOT-relative 32-bit.



## object::elf::R_AARCH64_GOTREL64

*Constant*: `u32`

GOT-relative 64-bit.



## object::elf::R_AARCH64_GOT_LD_PREL19

*Constant*: `u32`

PC-rel. GOT off. load imm. 20:2.



## object::elf::R_AARCH64_IRELATIVE

*Constant*: `u32`

STT_GNU_IFUNC relocation.



## object::elf::R_AARCH64_JUMP26

*Constant*: `u32`

PC-rel. B imm. from bits 27:2.



## object::elf::R_AARCH64_JUMP_SLOT

*Constant*: `u32`

Create PLT entry.



## object::elf::R_AARCH64_LD64_GOTOFF_LO15

*Constant*: `u32`

GOT-rel. off. LD/ST imm. 14:3.



## object::elf::R_AARCH64_LD64_GOTPAGE_LO15

*Constant*: `u32`

GOT-page-rel. GOT off. LD/ST 14:3



## object::elf::R_AARCH64_LD64_GOT_LO12_NC

*Constant*: `u32`

Dir. GOT off. LD/ST imm. 11:3.



## object::elf::R_AARCH64_LDST128_ABS_LO12_NC

*Constant*: `u32`

Dir. ADD imm. from bits 11:4.



## object::elf::R_AARCH64_LDST16_ABS_LO12_NC

*Constant*: `u32`

Dir. ADD imm. from bits 11:1.



## object::elf::R_AARCH64_LDST32_ABS_LO12_NC

*Constant*: `u32`

Likewise for bits 11:2.



## object::elf::R_AARCH64_LDST64_ABS_LO12_NC

*Constant*: `u32`

Likewise for bits 11:3.



## object::elf::R_AARCH64_LDST8_ABS_LO12_NC

*Constant*: `u32`

Likewise for LD/ST; no check.



## object::elf::R_AARCH64_LD_PREL_LO19

*Constant*: `u32`

PC-rel. LD imm. from bits 20:2.



## object::elf::R_AARCH64_MOVW_GOTOFF_G0

*Constant*: `u32`

GOT-rel. off. MOV{N,Z} imm. 15:0.



## object::elf::R_AARCH64_MOVW_GOTOFF_G0_NC

*Constant*: `u32`

Likewise for MOVK; no check.



## object::elf::R_AARCH64_MOVW_GOTOFF_G1

*Constant*: `u32`

GOT-rel. o. MOV{N,Z} imm. 31:16.



## object::elf::R_AARCH64_MOVW_GOTOFF_G1_NC

*Constant*: `u32`

Likewise for MOVK; no check.



## object::elf::R_AARCH64_MOVW_GOTOFF_G2

*Constant*: `u32`

GOT-rel. o. MOV{N,Z} imm. 47:32.



## object::elf::R_AARCH64_MOVW_GOTOFF_G2_NC

*Constant*: `u32`

Likewise for MOVK; no check.



## object::elf::R_AARCH64_MOVW_GOTOFF_G3

*Constant*: `u32`

GOT-rel. o. MOV{N,Z} imm. 63:48.



## object::elf::R_AARCH64_MOVW_PREL_G0

*Constant*: `u32`

PC-rel. MOV{N,Z} imm. from 15:0.



## object::elf::R_AARCH64_MOVW_PREL_G0_NC

*Constant*: `u32`

Likewise for MOVK; no check.



## object::elf::R_AARCH64_MOVW_PREL_G1

*Constant*: `u32`

PC-rel. MOV{N,Z} imm. from 31:16.



## object::elf::R_AARCH64_MOVW_PREL_G1_NC

*Constant*: `u32`

Likewise for MOVK; no check.



## object::elf::R_AARCH64_MOVW_PREL_G2

*Constant*: `u32`

PC-rel. MOV{N,Z} imm. from 47:32.



## object::elf::R_AARCH64_MOVW_PREL_G2_NC

*Constant*: `u32`

Likewise for MOVK; no check.



## object::elf::R_AARCH64_MOVW_PREL_G3

*Constant*: `u32`

PC-rel. MOV{N,Z} imm. from 63:48.



## object::elf::R_AARCH64_MOVW_SABS_G0

*Constant*: `u32`

Dir. MOV{N,Z} imm. from 15:0.



## object::elf::R_AARCH64_MOVW_SABS_G1

*Constant*: `u32`

Dir. MOV{N,Z} imm. from 31:16.



## object::elf::R_AARCH64_MOVW_SABS_G2

*Constant*: `u32`

Dir. MOV{N,Z} imm. from 47:32.



## object::elf::R_AARCH64_MOVW_UABS_G0

*Constant*: `u32`

Dir. MOVZ imm. from bits 15:0.



## object::elf::R_AARCH64_MOVW_UABS_G0_NC

*Constant*: `u32`

Likewise for MOVK; no check.



## object::elf::R_AARCH64_MOVW_UABS_G1

*Constant*: `u32`

Dir. MOVZ imm. from bits 31:16.



## object::elf::R_AARCH64_MOVW_UABS_G1_NC

*Constant*: `u32`

Likewise for MOVK; no check.



## object::elf::R_AARCH64_MOVW_UABS_G2

*Constant*: `u32`

Dir. MOVZ imm. from bits 47:32.



## object::elf::R_AARCH64_MOVW_UABS_G2_NC

*Constant*: `u32`

Likewise for MOVK; no check.



## object::elf::R_AARCH64_MOVW_UABS_G3

*Constant*: `u32`

Dir. MOV{K,Z} imm. from 63:48.



## object::elf::R_AARCH64_NONE

*Constant*: `u32`

No relocation.



## object::elf::R_AARCH64_P32_ABS32

*Constant*: `u32`

Direct 32 bit.



## object::elf::R_AARCH64_P32_COPY

*Constant*: `u32`

Copy symbol at runtime.



## object::elf::R_AARCH64_P32_GLOB_DAT

*Constant*: `u32`

Create GOT entry.



## object::elf::R_AARCH64_P32_IRELATIVE

*Constant*: `u32`

STT_GNU_IFUNC relocation.



## object::elf::R_AARCH64_P32_JUMP_SLOT

*Constant*: `u32`

Create PLT entry.



## object::elf::R_AARCH64_P32_RELATIVE

*Constant*: `u32`

Adjust by program base.



## object::elf::R_AARCH64_P32_TLSDESC

*Constant*: `u32`

TLS Descriptor.



## object::elf::R_AARCH64_P32_TLS_DTPMOD

*Constant*: `u32`

Module number, 32 bit.



## object::elf::R_AARCH64_P32_TLS_DTPREL

*Constant*: `u32`

Module-relative offset, 32 bit.



## object::elf::R_AARCH64_P32_TLS_TPREL

*Constant*: `u32`

TP-relative offset, 32 bit.



## object::elf::R_AARCH64_PREL16

*Constant*: `u32`

PC-relative 16-bit.



## object::elf::R_AARCH64_PREL32

*Constant*: `u32`

PC-relative 32-bit.



## object::elf::R_AARCH64_PREL64

*Constant*: `u32`

PC-relative 64-bit.



## object::elf::R_AARCH64_RELATIVE

*Constant*: `u32`

Adjust by program base.



## object::elf::R_AARCH64_TLSDESC

*Constant*: `u32`

TLS Descriptor.



## object::elf::R_AARCH64_TLSDESC_ADD

*Constant*: `u32`

Relax ADD.



## object::elf::R_AARCH64_TLSDESC_ADD_LO12

*Constant*: `u32`

Direct ADD imm. from 11:0.



## object::elf::R_AARCH64_TLSDESC_ADR_PAGE21

*Constant*: `u32`

Page-rel. ADRP imm. 32:12.



## object::elf::R_AARCH64_TLSDESC_ADR_PREL21

*Constant*: `u32`

PC-rel. ADR immediate 20:0.



## object::elf::R_AARCH64_TLSDESC_CALL

*Constant*: `u32`

Relax BLR.



## object::elf::R_AARCH64_TLSDESC_LD64_LO12

*Constant*: `u32`

Direct LD off. from 11:3.



## object::elf::R_AARCH64_TLSDESC_LDR

*Constant*: `u32`

Relax LDR.



## object::elf::R_AARCH64_TLSDESC_LD_PREL19

*Constant*: `u32`

PC-rel. load immediate 20:2.



## object::elf::R_AARCH64_TLSDESC_OFF_G0_NC

*Constant*: `u32`

GOT-rel. MOVK imm. 15:0; no ck.



## object::elf::R_AARCH64_TLSDESC_OFF_G1

*Constant*: `u32`

GOT-rel. MOV{N,Z} imm. 31:16.



## object::elf::R_AARCH64_TLSGD_ADD_LO12_NC

*Constant*: `u32`

direct ADD imm. from 11:0.



## object::elf::R_AARCH64_TLSGD_ADR_PAGE21

*Constant*: `u32`

page-rel. ADRP imm. 32:12.



## object::elf::R_AARCH64_TLSGD_ADR_PREL21

*Constant*: `u32`

PC-relative ADR imm. 20:0.



## object::elf::R_AARCH64_TLSGD_MOVW_G0_NC

*Constant*: `u32`

GOT-rel. MOVK imm. 15:0.



## object::elf::R_AARCH64_TLSGD_MOVW_G1

*Constant*: `u32`

GOT-rel. MOV{N,Z} 31:16.



## object::elf::R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21

*Constant*: `u32`

Page-rel. ADRP 32:12.



## object::elf::R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC

*Constant*: `u32`

Direct LD off. 11:3.



## object::elf::R_AARCH64_TLSIE_LD_GOTTPREL_PREL19

*Constant*: `u32`

PC-rel. load imm. 20:2.



## object::elf::R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC

*Constant*: `u32`

GOT-rel. MOVK 15:0.



## object::elf::R_AARCH64_TLSIE_MOVW_GOTTPREL_G1

*Constant*: `u32`

GOT-rel. MOV{N,Z} 31:16.



## object::elf::R_AARCH64_TLSLD_ADD_DTPREL_HI12

*Constant*: `u32`

DTP-rel. ADD imm. from 23:12.



## object::elf::R_AARCH64_TLSLD_ADD_DTPREL_LO12

*Constant*: `u32`

DTP-rel. ADD imm. from 11:0.



## object::elf::R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC

*Constant*: `u32`

Likewise; no ovfl. check.



## object::elf::R_AARCH64_TLSLD_ADD_LO12_NC

*Constant*: `u32`

Like 514; local dynamic model.



## object::elf::R_AARCH64_TLSLD_ADR_PAGE21

*Constant*: `u32`

Like 513; local dynamic model.



## object::elf::R_AARCH64_TLSLD_ADR_PREL21

*Constant*: `u32`

Like 512; local dynamic model.



## object::elf::R_AARCH64_TLSLD_LDST128_DTPREL_LO12

*Constant*: `u32`

DTP-rel. LD/ST imm. 11:4.



## object::elf::R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC

*Constant*: `u32`

Likewise; no check.



## object::elf::R_AARCH64_TLSLD_LDST16_DTPREL_LO12

*Constant*: `u32`

DTP-rel. LD/ST imm. 11:1.



## object::elf::R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC

*Constant*: `u32`

Likewise; no check.



## object::elf::R_AARCH64_TLSLD_LDST32_DTPREL_LO12

*Constant*: `u32`

DTP-rel. LD/ST imm. 11:2.



## object::elf::R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC

*Constant*: `u32`

Likewise; no check.



## object::elf::R_AARCH64_TLSLD_LDST64_DTPREL_LO12

*Constant*: `u32`

DTP-rel. LD/ST imm. 11:3.



## object::elf::R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC

*Constant*: `u32`

Likewise; no check.



## object::elf::R_AARCH64_TLSLD_LDST8_DTPREL_LO12

*Constant*: `u32`

DTP-rel. LD/ST imm. 11:0.



## object::elf::R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC

*Constant*: `u32`

Likewise; no check.



## object::elf::R_AARCH64_TLSLD_LD_PREL19

*Constant*: `u32`

TLS PC-rel. load imm. 20:2.



## object::elf::R_AARCH64_TLSLD_MOVW_DTPREL_G0

*Constant*: `u32`

TLS DTP-rel. MOV{N,Z} 15:0.



## object::elf::R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC

*Constant*: `u32`

Likewise; MOVK; no check.



## object::elf::R_AARCH64_TLSLD_MOVW_DTPREL_G1

*Constant*: `u32`

TLS DTP-rel. MOV{N,Z} 31:16.



## object::elf::R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC

*Constant*: `u32`

Likewise; MOVK; no check.



## object::elf::R_AARCH64_TLSLD_MOVW_DTPREL_G2

*Constant*: `u32`

TLS DTP-rel. MOV{N,Z} 47:32.



## object::elf::R_AARCH64_TLSLD_MOVW_G0_NC

*Constant*: `u32`

Like 516; local dynamic model.



## object::elf::R_AARCH64_TLSLD_MOVW_G1

*Constant*: `u32`

Like 515; local dynamic model.



## object::elf::R_AARCH64_TLSLE_ADD_TPREL_HI12

*Constant*: `u32`

TP-rel. ADD imm. 23:12.



## object::elf::R_AARCH64_TLSLE_ADD_TPREL_LO12

*Constant*: `u32`

TP-rel. ADD imm. 11:0.



## object::elf::R_AARCH64_TLSLE_ADD_TPREL_LO12_NC

*Constant*: `u32`

Likewise; no ovfl. check.



## object::elf::R_AARCH64_TLSLE_LDST128_TPREL_LO12

*Constant*: `u32`

TP-rel. LD/ST off. 11:4.



## object::elf::R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC

*Constant*: `u32`

Likewise; no check.



## object::elf::R_AARCH64_TLSLE_LDST16_TPREL_LO12

*Constant*: `u32`

TP-rel. LD/ST off. 11:1.



## object::elf::R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC

*Constant*: `u32`

Likewise; no check.



## object::elf::R_AARCH64_TLSLE_LDST32_TPREL_LO12

*Constant*: `u32`

TP-rel. LD/ST off. 11:2.



## object::elf::R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC

*Constant*: `u32`

Likewise; no check.



## object::elf::R_AARCH64_TLSLE_LDST64_TPREL_LO12

*Constant*: `u32`

TP-rel. LD/ST off. 11:3.



## object::elf::R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC

*Constant*: `u32`

Likewise; no check.



## object::elf::R_AARCH64_TLSLE_LDST8_TPREL_LO12

*Constant*: `u32`

TP-rel. LD/ST off. 11:0.



## object::elf::R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC

*Constant*: `u32`

Likewise; no ovfl. check.



## object::elf::R_AARCH64_TLSLE_MOVW_TPREL_G0

*Constant*: `u32`

TLS TP-rel. MOV{N,Z} 15:0.



## object::elf::R_AARCH64_TLSLE_MOVW_TPREL_G0_NC

*Constant*: `u32`

Likewise; MOVK; no check.



## object::elf::R_AARCH64_TLSLE_MOVW_TPREL_G1

*Constant*: `u32`

TLS TP-rel. MOV{N,Z} 31:16.



## object::elf::R_AARCH64_TLSLE_MOVW_TPREL_G1_NC

*Constant*: `u32`

Likewise; MOVK; no check.



## object::elf::R_AARCH64_TLSLE_MOVW_TPREL_G2

*Constant*: `u32`

TLS TP-rel. MOV{N,Z} 47:32.



## object::elf::R_AARCH64_TLS_DTPMOD

*Constant*: `u32`

Module number, 64 bit.



## object::elf::R_AARCH64_TLS_DTPREL

*Constant*: `u32`

Module-relative offset, 64 bit.



## object::elf::R_AARCH64_TLS_TPREL

*Constant*: `u32`

TP-relative offset, 64 bit.



## object::elf::R_AARCH64_TSTBR14

*Constant*: `u32`

PC-rel. TBZ/TBNZ imm. from 15:2.



## object::elf::R_ALPHA_BRADDR

*Constant*: `u32`

PC+4 relative 23 bit shifted



## object::elf::R_ALPHA_COPY

*Constant*: `u32`

Copy symbol at runtime



## object::elf::R_ALPHA_DTPMOD64

*Constant*: `u32`



## object::elf::R_ALPHA_DTPREL16

*Constant*: `u32`



## object::elf::R_ALPHA_DTPREL64

*Constant*: `u32`



## object::elf::R_ALPHA_DTPRELHI

*Constant*: `u32`



## object::elf::R_ALPHA_DTPRELLO

*Constant*: `u32`



## object::elf::R_ALPHA_GLOB_DAT

*Constant*: `u32`

Create GOT entry



## object::elf::R_ALPHA_GOTDTPREL

*Constant*: `u32`



## object::elf::R_ALPHA_GOTTPREL

*Constant*: `u32`



## object::elf::R_ALPHA_GPDISP

*Constant*: `u32`

Add displacement to GP



## object::elf::R_ALPHA_GPREL16

*Constant*: `u32`

GP relative 16 bit



## object::elf::R_ALPHA_GPREL32

*Constant*: `u32`

GP relative 32 bit



## object::elf::R_ALPHA_GPRELHIGH

*Constant*: `u32`

GP relative 32 bit, high 16 bits



## object::elf::R_ALPHA_GPRELLOW

*Constant*: `u32`

GP relative 32 bit, low 16 bits



## object::elf::R_ALPHA_HINT

*Constant*: `u32`

PC+4 relative 16 bit shifted



## object::elf::R_ALPHA_JMP_SLOT

*Constant*: `u32`

Create PLT entry



## object::elf::R_ALPHA_LITERAL

*Constant*: `u32`

GP relative 16 bit w/optimization



## object::elf::R_ALPHA_LITUSE

*Constant*: `u32`

Optimization hint for LITERAL



## object::elf::R_ALPHA_NONE

*Constant*: `u32`

No reloc



## object::elf::R_ALPHA_REFLONG

*Constant*: `u32`

Direct 32 bit



## object::elf::R_ALPHA_REFQUAD

*Constant*: `u32`

Direct 64 bit



## object::elf::R_ALPHA_RELATIVE

*Constant*: `u32`

Adjust by program base



## object::elf::R_ALPHA_SREL16

*Constant*: `u32`

PC relative 16 bit



## object::elf::R_ALPHA_SREL32

*Constant*: `u32`

PC relative 32 bit



## object::elf::R_ALPHA_SREL64

*Constant*: `u32`

PC relative 64 bit



## object::elf::R_ALPHA_TLSGD

*Constant*: `u32`



## object::elf::R_ALPHA_TLS_GD_HI

*Constant*: `u32`



## object::elf::R_ALPHA_TLS_LDM

*Constant*: `u32`



## object::elf::R_ALPHA_TPREL16

*Constant*: `u32`



## object::elf::R_ALPHA_TPREL64

*Constant*: `u32`



## object::elf::R_ALPHA_TPRELHI

*Constant*: `u32`



## object::elf::R_ALPHA_TPRELLO

*Constant*: `u32`



## object::elf::R_ARM_ABS12

*Constant*: `u32`

Direct 12 bit



## object::elf::R_ARM_ABS16

*Constant*: `u32`

Direct 16 bit



## object::elf::R_ARM_ABS32

*Constant*: `u32`

Direct 32 bit



## object::elf::R_ARM_ABS32_NOI

*Constant*: `u32`

Direct 32-bit.



## object::elf::R_ARM_ABS8

*Constant*: `u32`

Direct 8 bit



## object::elf::R_ARM_ALU_PCREL_15_8

*Constant*: `u32`

Obsolete.



## object::elf::R_ARM_ALU_PCREL_23_15

*Constant*: `u32`

Obsolete.



## object::elf::R_ARM_ALU_PCREL_7_0

*Constant*: `u32`

Obsolete.



## object::elf::R_ARM_ALU_PC_G0

*Constant*: `u32`

PC relative (`ADD`, `SUB`).



## object::elf::R_ARM_ALU_PC_G0_NC

*Constant*: `u32`

PC relative (`ADD`, `SUB`).



## object::elf::R_ARM_ALU_PC_G1

*Constant*: `u32`

PC relative (`ADD`, `SUB`).



## object::elf::R_ARM_ALU_PC_G1_NC

*Constant*: `u32`

PC relative (`ADD`, `SUB`).



## object::elf::R_ARM_ALU_PC_G2

*Constant*: `u32`

PC relative (`ADD`, `SUB`).



## object::elf::R_ARM_ALU_SBREL_19_12

*Constant*: `u32`

Deprecated, prog. base relative.



## object::elf::R_ARM_ALU_SBREL_27_20

*Constant*: `u32`

Deprecated, prog. base relative.



## object::elf::R_ARM_ALU_SB_G0

*Constant*: `u32`

Program base relative (`ADD`,`SUB`).



## object::elf::R_ARM_ALU_SB_G0_NC

*Constant*: `u32`

Program base relative (`ADD`,`SUB`).



## object::elf::R_ARM_ALU_SB_G1

*Constant*: `u32`

Program base relative (`ADD`,`SUB`).



## object::elf::R_ARM_ALU_SB_G1_NC

*Constant*: `u32`

Program base relative (`ADD`,`SUB`).



## object::elf::R_ARM_ALU_SB_G2

*Constant*: `u32`

Program base relative (`ADD`,`SUB`).



## object::elf::R_ARM_AMP_VCALL9

*Constant*: `u32`



## object::elf::R_ARM_BASE_ABS

*Constant*: `u32`

Adjust by program base.



## object::elf::R_ARM_CALL

*Constant*: `u32`

PC relative 24 bit (`BL`, `BLX`).



## object::elf::R_ARM_COPY

*Constant*: `u32`

Copy symbol at runtime



## object::elf::R_ARM_GLOB_DAT

*Constant*: `u32`

Create GOT entry



## object::elf::R_ARM_GNU_VTENTRY

*Constant*: `u32`



## object::elf::R_ARM_GNU_VTINHERIT

*Constant*: `u32`



## object::elf::R_ARM_GOT32

*Constant*: `u32`

32 bit GOT entry



## object::elf::R_ARM_GOTOFF

*Constant*: `u32`

32 bit offset to GOT



## object::elf::R_ARM_GOTOFF12

*Constant*: `u32`

12 bit, GOT entry relative to GOT origin (`LDR`, `STR`).



## object::elf::R_ARM_GOTPC

*Constant*: `u32`

32 bit PC relative offset to GOT



## object::elf::R_ARM_GOTRELAX

*Constant*: `u32`



## object::elf::R_ARM_GOT_ABS

*Constant*: `u32`

GOT entry.



## object::elf::R_ARM_GOT_BREL12

*Constant*: `u32`

GOT entry relative to GOT origin (`LDR`).



## object::elf::R_ARM_GOT_PREL

*Constant*: `u32`

PC relative GOT entry.



## object::elf::R_ARM_IRELATIVE

*Constant*: `u32`



## object::elf::R_ARM_JUMP24

*Constant*: `u32`

PC relative 24 bit (`B`, `BL<cond>`).



## object::elf::R_ARM_JUMP_SLOT

*Constant*: `u32`

Create PLT entry



## object::elf::R_ARM_LDC_PC_G0

*Constant*: `u32`

PC relative (`LDC`, `STC`).



## object::elf::R_ARM_LDC_PC_G1

*Constant*: `u32`

PC relative (`LDC`, `STC`).



## object::elf::R_ARM_LDC_PC_G2

*Constant*: `u32`

PC relative (`LDC`, `STC`).



## object::elf::R_ARM_LDC_SB_G0

*Constant*: `u32`

Program base relative (`LDC`,`STC`).



## object::elf::R_ARM_LDC_SB_G1

*Constant*: `u32`

Program base relative (`LDC`,`STC`).



## object::elf::R_ARM_LDC_SB_G2

*Constant*: `u32`

Program base relative (`LDC`,`STC`).



## object::elf::R_ARM_LDRS_PC_G0

*Constant*: `u32`

PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).



## object::elf::R_ARM_LDRS_PC_G1

*Constant*: `u32`

PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).



## object::elf::R_ARM_LDRS_PC_G2

*Constant*: `u32`

PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).



## object::elf::R_ARM_LDRS_SB_G0

*Constant*: `u32`

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).



## object::elf::R_ARM_LDRS_SB_G1

*Constant*: `u32`

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).



## object::elf::R_ARM_LDRS_SB_G2

*Constant*: `u32`

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).



## object::elf::R_ARM_LDR_PC_G1

*Constant*: `u32`

PC relative (`LDR`,`STR`,`LDRB`,`STRB`).



## object::elf::R_ARM_LDR_PC_G2

*Constant*: `u32`

PC relative (`LDR`,`STR`,`LDRB`,`STRB`).



## object::elf::R_ARM_LDR_SBREL_11_0

*Constant*: `u32`

Deprecated, prog. base relative.



## object::elf::R_ARM_LDR_SB_G0

*Constant*: `u32`

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).



## object::elf::R_ARM_LDR_SB_G1

*Constant*: `u32`

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).



## object::elf::R_ARM_LDR_SB_G2

*Constant*: `u32`

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).



## object::elf::R_ARM_ME_TOO

*Constant*: `u32`

Obsolete.



## object::elf::R_ARM_MOVT_ABS

*Constant*: `u32`

Direct high 16-bit (`MOVT`).



## object::elf::R_ARM_MOVT_BREL

*Constant*: `u32`

Program base relative high 16 bit (`MOVT`).



## object::elf::R_ARM_MOVT_PREL

*Constant*: `u32`

PC relative (MOVT).



## object::elf::R_ARM_MOVW_ABS_NC

*Constant*: `u32`

Direct 16-bit (`MOVW`).



## object::elf::R_ARM_MOVW_BREL

*Constant*: `u32`

Program base relative 16 bit (`MOVW`).



## object::elf::R_ARM_MOVW_BREL_NC

*Constant*: `u32`

Program base relative 16 bit (`MOVW`).



## object::elf::R_ARM_MOVW_PREL_NC

*Constant*: `u32`

PC relative 16-bit (`MOVW`).



## object::elf::R_ARM_NONE

*Constant*: `u32`

No reloc



## object::elf::R_ARM_PC13

*Constant*: `u32`



## object::elf::R_ARM_PC24

*Constant*: `u32`

Deprecated PC relative 26 bit branch.



## object::elf::R_ARM_PLT32

*Constant*: `u32`

Deprecated, 32 bit PLT address.



## object::elf::R_ARM_PLT32_ABS

*Constant*: `u32`



## object::elf::R_ARM_PREL31

*Constant*: `u32`

32 bit PC relative.



## object::elf::R_ARM_RABS22

*Constant*: `u32`



## object::elf::R_ARM_RBASE

*Constant*: `u32`



## object::elf::R_ARM_REL32

*Constant*: `u32`

PC relative 32 bit



## object::elf::R_ARM_REL32_NOI

*Constant*: `u32`

PC relative 32-bit.



## object::elf::R_ARM_RELATIVE

*Constant*: `u32`

Adjust by program base



## object::elf::R_ARM_RPC24

*Constant*: `u32`



## object::elf::R_ARM_RREL32

*Constant*: `u32`



## object::elf::R_ARM_RSBREL32

*Constant*: `u32`



## object::elf::R_ARM_RXPC25

*Constant*: `u32`



## object::elf::R_ARM_SBREL31

*Constant*: `u32`

Program base relative.



## object::elf::R_ARM_SBREL32

*Constant*: `u32`



## object::elf::R_ARM_SWI24

*Constant*: `u32`

Obsolete static relocation.



## object::elf::R_ARM_TARGET1

*Constant*: `u32`



## object::elf::R_ARM_TARGET2

*Constant*: `u32`



## object::elf::R_ARM_THM_ABS5

*Constant*: `u32`

Direct & 0x7C (`LDR`, `STR`).



## object::elf::R_ARM_THM_ALU_PREL_11_0

*Constant*: `u32`

PC relative 12 bit (Thumb32 `ADR.W`).



## object::elf::R_ARM_THM_GOT_BREL12

*Constant*: `u32`

GOT entry relative to GOT origin, 12 bit (Thumb32 `LDR`).



## object::elf::R_ARM_THM_JUMP19

*Constant*: `u32`

PC relative 20 bit (Thumb32 `B<cond>.W`).



## object::elf::R_ARM_THM_JUMP24

*Constant*: `u32`

PC relative 24 bit (Thumb32 `B.W`).



## object::elf::R_ARM_THM_JUMP6

*Constant*: `u32`

PC relative X & 0x7E (Thumb16 `CBZ`, `CBNZ`).



## object::elf::R_ARM_THM_MOVT_ABS

*Constant*: `u32`

Direct high 16 bit (Thumb32 `MOVT`).



## object::elf::R_ARM_THM_MOVT_BREL

*Constant*: `u32`

Program base relative high 16 bit (Thumb32 `MOVT`).



## object::elf::R_ARM_THM_MOVT_PREL

*Constant*: `u32`

PC relative high 16 bit (Thumb32 `MOVT`).



## object::elf::R_ARM_THM_MOVW_ABS_NC

*Constant*: `u32`

Direct 16 bit (Thumb32 `MOVW`).



## object::elf::R_ARM_THM_MOVW_BREL

*Constant*: `u32`

Program base relative 16 bit (Thumb32 `MOVW`).



## object::elf::R_ARM_THM_MOVW_BREL_NC

*Constant*: `u32`

Program base relative 16 bit (Thumb32 `MOVW`).



## object::elf::R_ARM_THM_MOVW_PREL_NC

*Constant*: `u32`

PC relative 16 bit (Thumb32 `MOVW`).



## object::elf::R_ARM_THM_PC11

*Constant*: `u32`

PC relative & 0xFFE (Thumb16 `B`).



## object::elf::R_ARM_THM_PC12

*Constant*: `u32`

PC relative 12 bit (Thumb32 `LDR{D,SB,H,SH}`).



## object::elf::R_ARM_THM_PC22

*Constant*: `u32`

PC relative 24 bit (Thumb32 `BL`).



## object::elf::R_ARM_THM_PC8

*Constant*: `u32`

PC relative & 0x3FC (Thumb16 `LDR`, `ADD`, `ADR`).



## object::elf::R_ARM_THM_PC9

*Constant*: `u32`

PC relative & 0x1FE (Thumb16 `B`/`B<cond>`).



## object::elf::R_ARM_THM_RPC22

*Constant*: `u32`



## object::elf::R_ARM_THM_SWI8

*Constant*: `u32`

Reserved.



## object::elf::R_ARM_THM_TLS_CALL

*Constant*: `u32`



## object::elf::R_ARM_THM_TLS_DESCSEQ

*Constant*: `u32`



## object::elf::R_ARM_THM_TLS_DESCSEQ16

*Constant*: `u32`



## object::elf::R_ARM_THM_TLS_DESCSEQ32

*Constant*: `u32`



## object::elf::R_ARM_THM_XPC22

*Constant*: `u32`

Reserved.



## object::elf::R_ARM_TLS_CALL

*Constant*: `u32`



## object::elf::R_ARM_TLS_DESC

*Constant*: `u32`

Dynamic relocation.



## object::elf::R_ARM_TLS_DESCSEQ

*Constant*: `u32`

TLS relaxation.



## object::elf::R_ARM_TLS_DTPMOD32

*Constant*: `u32`

ID of module containing symbol



## object::elf::R_ARM_TLS_DTPOFF32

*Constant*: `u32`

Offset in TLS block



## object::elf::R_ARM_TLS_GD32

*Constant*: `u32`

PC-rel 32 bit for global dynamic thread local data



## object::elf::R_ARM_TLS_GOTDESC

*Constant*: `u32`



## object::elf::R_ARM_TLS_IE12GP

*Constant*: `u32`

12 bit GOT entry relative to GOT origin (`LDR`).



## object::elf::R_ARM_TLS_IE32

*Constant*: `u32`

PC-rel 32 bit for GOT entry of static TLS block offset



## object::elf::R_ARM_TLS_LDM32

*Constant*: `u32`

PC-rel 32 bit for local dynamic thread local data



## object::elf::R_ARM_TLS_LDO12

*Constant*: `u32`

12 bit relative to TLS block (`LDR`, `STR`).



## object::elf::R_ARM_TLS_LDO32

*Constant*: `u32`

32 bit offset relative to TLS block



## object::elf::R_ARM_TLS_LE12

*Constant*: `u32`

12 bit relative to static TLS block (`LDR`, `STR`).



## object::elf::R_ARM_TLS_LE32

*Constant*: `u32`

32 bit offset relative to static TLS block



## object::elf::R_ARM_TLS_TPOFF32

*Constant*: `u32`

Offset in static TLS block



## object::elf::R_ARM_V4BX

*Constant*: `u32`



## object::elf::R_ARM_XPC25

*Constant*: `u32`

Reserved.



## object::elf::R_AVR_13_PCREL

*Constant*: `u32`



## object::elf::R_AVR_16

*Constant*: `u32`

Direct 16 bit



## object::elf::R_AVR_16_PM

*Constant*: `u32`



## object::elf::R_AVR_32

*Constant*: `u32`

Direct 32 bit



## object::elf::R_AVR_32_PCREL

*Constant*: `u32`



## object::elf::R_AVR_6

*Constant*: `u32`



## object::elf::R_AVR_6_ADIW

*Constant*: `u32`



## object::elf::R_AVR_7_PCREL

*Constant*: `u32`



## object::elf::R_AVR_8

*Constant*: `u32`



## object::elf::R_AVR_8_HI8

*Constant*: `u32`



## object::elf::R_AVR_8_HLO8

*Constant*: `u32`



## object::elf::R_AVR_8_LO8

*Constant*: `u32`



## object::elf::R_AVR_CALL

*Constant*: `u32`



## object::elf::R_AVR_DIFF16

*Constant*: `u32`



## object::elf::R_AVR_DIFF32

*Constant*: `u32`



## object::elf::R_AVR_DIFF8

*Constant*: `u32`



## object::elf::R_AVR_HH8_LDI

*Constant*: `u32`



## object::elf::R_AVR_HH8_LDI_NEG

*Constant*: `u32`



## object::elf::R_AVR_HH8_LDI_PM

*Constant*: `u32`



## object::elf::R_AVR_HH8_LDI_PM_NEG

*Constant*: `u32`



## object::elf::R_AVR_HI8_LDI

*Constant*: `u32`



## object::elf::R_AVR_HI8_LDI_GS

*Constant*: `u32`



## object::elf::R_AVR_HI8_LDI_NEG

*Constant*: `u32`



## object::elf::R_AVR_HI8_LDI_PM

*Constant*: `u32`



## object::elf::R_AVR_HI8_LDI_PM_NEG

*Constant*: `u32`



## object::elf::R_AVR_LDI

*Constant*: `u32`



## object::elf::R_AVR_LDS_STS_16

*Constant*: `u32`



## object::elf::R_AVR_LO8_LDI

*Constant*: `u32`



## object::elf::R_AVR_LO8_LDI_GS

*Constant*: `u32`



## object::elf::R_AVR_LO8_LDI_NEG

*Constant*: `u32`



## object::elf::R_AVR_LO8_LDI_PM

*Constant*: `u32`



## object::elf::R_AVR_LO8_LDI_PM_NEG

*Constant*: `u32`



## object::elf::R_AVR_MS8_LDI

*Constant*: `u32`



## object::elf::R_AVR_MS8_LDI_NEG

*Constant*: `u32`



## object::elf::R_AVR_NONE

*Constant*: `u32`



## object::elf::R_AVR_PORT5

*Constant*: `u32`



## object::elf::R_AVR_PORT6

*Constant*: `u32`



## object::elf::R_BPF_64_32

*Constant*: `u32`



## object::elf::R_BPF_64_64

*Constant*: `u32`



## object::elf::R_BPF_NONE

*Constant*: `u32`

No reloc



## object::elf::R_CKCORE_ADDR32

*Constant*: `u32`

direct 32 bit (S + A)



## object::elf::R_CKCORE_ADDRGOT

*Constant*: `u32`

GOT entry in GLOB_DAT (GOT + G)



## object::elf::R_CKCORE_ADDRGOT_HI16

*Constant*: `u32`

high & low 16 bit ADDRGOT, (GOT + G * 4) & 0xffff



## object::elf::R_CKCORE_ADDRGOT_LO16

*Constant*: `u32`

(GOT + G * 4) & 0xffff



## object::elf::R_CKCORE_ADDRPLT

*Constant*: `u32`

PLT entry in GLOB_DAT (GOT + G)



## object::elf::R_CKCORE_ADDRPLT_HI16

*Constant*: `u32`

high & low 16 bit ADDRPLT, ((GOT + G * 4) >> 16) & 0xFFFF



## object::elf::R_CKCORE_ADDRPLT_LO16

*Constant*: `u32`

(GOT+G*4) & 0xffff



## object::elf::R_CKCORE_ADDR_HI16

*Constant*: `u32`

high & low 16 bit ADDR, ((S + A) >> 16) & 0xffff



## object::elf::R_CKCORE_ADDR_LO16

*Constant*: `u32`

(S + A) & 0xffff



## object::elf::R_CKCORE_COPY

*Constant*: `u32`

32 bit adjust by program base



## object::elf::R_CKCORE_DOFFSET_IMM18

*Constant*: `u32`

disp (S+A-BDATA) & 0x3ffff



## object::elf::R_CKCORE_DOFFSET_IMM18BY2

*Constant*: `u32`

disp ((S+A-BDATA)>>1) & 0x3ffff



## object::elf::R_CKCORE_DOFFSET_IMM18BY4

*Constant*: `u32`

disp ((S+A-BDATA)>>2) & 0x3ffff



## object::elf::R_CKCORE_DOFFSET_LO16

*Constant*: `u32`

(S+A-BTEXT) & 0xffff



## object::elf::R_CKCORE_GLOB_DAT

*Constant*: `u32`

off between got and sym (S)



## object::elf::R_CKCORE_GOT12

*Constant*: `u32`

12 bit disp GOT entry (G)



## object::elf::R_CKCORE_GOT32

*Constant*: `u32`

32 bit GOT entry (G)



## object::elf::R_CKCORE_GOTOFF

*Constant*: `u32`

offset to GOT (S + A - GOT)



## object::elf::R_CKCORE_GOTOFF_HI16

*Constant*: `u32`

high & low 16 bit GOTOFF, ((S + A - GOT) >> 16) & 0xffff



## object::elf::R_CKCORE_GOTOFF_LO16

*Constant*: `u32`

(S + A - GOT) & 0xffff



## object::elf::R_CKCORE_GOTPC

*Constant*: `u32`

PC offset to GOT (GOT + A - P)



## object::elf::R_CKCORE_GOTPC_HI16

*Constant*: `u32`

high & low 16 bit GOTPC, ((GOT + A - P) >> 16) & 0xffff



## object::elf::R_CKCORE_GOTPC_LO16

*Constant*: `u32`

(GOT + A - P) & 0xffff



## object::elf::R_CKCORE_GOT_HI16

*Constant*: `u32`

high & low 16 bit GOT, (G >> 16) & 0xffff



## object::elf::R_CKCORE_GOT_IMM18BY4

*Constant*: `u32`

disp (G >> 2)



## object::elf::R_CKCORE_GOT_LO16

*Constant*: `u32`

(G & 0xffff)



## object::elf::R_CKCORE_JUMP_SLOT

*Constant*: `u32`

PLT entry (S)



## object::elf::R_CKCORE_NONE

*Constant*: `u32`

no reloc



## object::elf::R_CKCORE_PCREL32

*Constant*: `u32`

32-bit rel (S + A - P)



## object::elf::R_CKCORE_PCRELIMM11BY2

*Constant*: `u32`

disp ((S + A - P) >> 1) & 0x7ff



## object::elf::R_CKCORE_PCRELIMM8BY4

*Constant*: `u32`

disp ((S + A - P) >> 2) & 0xff



## object::elf::R_CKCORE_PCRELJSR_IMM11BY2

*Constant*: `u32`

disp ((S + A - P) >>1) & 0x7ff



## object::elf::R_CKCORE_PCREL_IMM10BY2

*Constant*: `u32`

disp ((S + A - P) >> 1) & 0x3ff



## object::elf::R_CKCORE_PCREL_IMM10BY4

*Constant*: `u32`

disp ((S + A - P) >> 2) & 0x3ff



## object::elf::R_CKCORE_PCREL_IMM16BY2

*Constant*: `u32`

disp ((S + A - P) >> 1) & 0xffff



## object::elf::R_CKCORE_PCREL_IMM16BY4

*Constant*: `u32`

disp ((S + A - P) >> 2) & 0xffff



## object::elf::R_CKCORE_PCREL_IMM18BY2

*Constant*: `u32`

disp ((S+A-P) >>1) & 0x3ffff



## object::elf::R_CKCORE_PCREL_IMM26BY2

*Constant*: `u32`

((S + A - P) >> 1) & 0x3ff_ffff



## object::elf::R_CKCORE_PCREL_IMM7BY4

*Constant*: `u32`

disp ((S+A-P) >>2) & 0x7f



## object::elf::R_CKCORE_PCREL_JSR_IMM26BY2

*Constant*: `u32`

disp ((S+A-P) >>1) & x3ff_ffff



## object::elf::R_CKCORE_PLT12

*Constant*: `u32`

12 bit disp PLT entry (G)



## object::elf::R_CKCORE_PLT32

*Constant*: `u32`

32 bit PLT entry (G)



## object::elf::R_CKCORE_PLT_HI16

*Constant*: `u32`

high & low 16 bit PLT, (G >> 16) & 0xffff



## object::elf::R_CKCORE_PLT_IMM18BY4

*Constant*: `u32`

disp (G >> 2)



## object::elf::R_CKCORE_PLT_LO16

*Constant*: `u32`

G & 0xffff



## object::elf::R_CKCORE_RELATIVE

*Constant*: `u32`

32 bit adjust program base(B + A)



## object::elf::R_CKCORE_TLS_DTPMOD32

*Constant*: `u32`



## object::elf::R_CKCORE_TLS_DTPOFF32

*Constant*: `u32`



## object::elf::R_CKCORE_TLS_GD32

*Constant*: `u32`



## object::elf::R_CKCORE_TLS_IE32

*Constant*: `u32`



## object::elf::R_CKCORE_TLS_LDM32

*Constant*: `u32`



## object::elf::R_CKCORE_TLS_LDO32

*Constant*: `u32`



## object::elf::R_CKCORE_TLS_LE32

*Constant*: `u32`

32 bit offset to TLS block



## object::elf::R_CKCORE_TLS_TPOFF32

*Constant*: `u32`



## object::elf::R_CKCORE_TOFFSET_LO16

*Constant*: `u32`

(S+A-BTEXT) & 0xffff



## object::elf::R_CRIS_16

*Constant*: `u32`



## object::elf::R_CRIS_16_GOT

*Constant*: `u32`



## object::elf::R_CRIS_16_GOTPLT

*Constant*: `u32`



## object::elf::R_CRIS_16_PCREL

*Constant*: `u32`



## object::elf::R_CRIS_32

*Constant*: `u32`



## object::elf::R_CRIS_32_GOT

*Constant*: `u32`



## object::elf::R_CRIS_32_GOTPLT

*Constant*: `u32`



## object::elf::R_CRIS_32_GOTREL

*Constant*: `u32`



## object::elf::R_CRIS_32_PCREL

*Constant*: `u32`



## object::elf::R_CRIS_32_PLT_GOTREL

*Constant*: `u32`



## object::elf::R_CRIS_32_PLT_PCREL

*Constant*: `u32`



## object::elf::R_CRIS_8

*Constant*: `u32`



## object::elf::R_CRIS_8_PCREL

*Constant*: `u32`



## object::elf::R_CRIS_COPY

*Constant*: `u32`



## object::elf::R_CRIS_GLOB_DAT

*Constant*: `u32`



## object::elf::R_CRIS_GNU_VTENTRY

*Constant*: `u32`



## object::elf::R_CRIS_GNU_VTINHERIT

*Constant*: `u32`



## object::elf::R_CRIS_JUMP_SLOT

*Constant*: `u32`



## object::elf::R_CRIS_NONE

*Constant*: `u32`



## object::elf::R_CRIS_RELATIVE

*Constant*: `u32`



## object::elf::R_E2K_32_ABS

*Constant*: `u32`

Direct 32 bit.



## object::elf::R_E2K_32_COPY

*Constant*: `u32`

Copy relocation, 32-bit case.



## object::elf::R_E2K_32_DYNOPT

*Constant*: `u32`

Symbol value if resolved by the definition in the same
compilation unit or NULL otherwise, 32-bit case.



## object::elf::R_E2K_32_IRELATIVE

*Constant*: `u32`

Adjust indirectly by program base, 32-bit case.



## object::elf::R_E2K_32_JMP_SLOT

*Constant*: `u32`

Create PLT entry.



## object::elf::R_E2K_32_PC

*Constant*: `u32`

PC relative 32 bit.



## object::elf::R_E2K_32_RELATIVE

*Constant*: `u32`

Adjust by program base, 32-bit case.



## object::elf::R_E2K_32_SIZE

*Constant*: `u32`

Size of symbol plus 32-bit addend.



## object::elf::R_E2K_32_TLS_LE

*Constant*: `u32`

Offset relative to static TLS block, 32-bit case.



## object::elf::R_E2K_64_ABS

*Constant*: `u32`

Direct 64 bit.



## object::elf::R_E2K_64_ABS_LIT

*Constant*: `u32`

Direct 64 bit for literal.



## object::elf::R_E2K_64_COPY

*Constant*: `u32`

Copy relocation, 64-bit case.



## object::elf::R_E2K_64_DYNOPT

*Constant*: `u32`

Symbol value if resolved by the definition in the same
compilation unit or NULL otherwise, 64-bit case.



## object::elf::R_E2K_64_GOTOFF

*Constant*: `u32`

64-bit offset of the symbol from GOT.



## object::elf::R_E2K_64_GOTOFF_LIT

*Constant*: `u32`

The symbol's offset from GOT encoded within a 64-bit literal.



## object::elf::R_E2K_64_IRELATIVE

*Constant*: `u32`

Adjust indirectly by program base, 64-bit case.



## object::elf::R_E2K_64_JMP_SLOT

*Constant*: `u32`

Create PLT entry, 64-bit case.



## object::elf::R_E2K_64_PC

*Constant*: `u32`

PC relative 64 bit in data.



## object::elf::R_E2K_64_PC_LIT

*Constant*: `u32`

PC relative 64 bit for literal.



## object::elf::R_E2K_64_RELATIVE

*Constant*: `u32`

Adjust by program base, 64-bit case.



## object::elf::R_E2K_64_RELATIVE_LIT

*Constant*: `u32`

Adjust by program base for literal, 64-bit case.



## object::elf::R_E2K_64_SIZE

*Constant*: `u32`

Size of symbol plus 64-bit addend.



## object::elf::R_E2K_64_TLS_LE

*Constant*: `u32`

Offset relative to static TLS block, 64-bit case.



## object::elf::R_E2K_AP

*Constant*: `u32`

Direct AP.



## object::elf::R_E2K_AP_GOT

*Constant*: `u32`

32-bit offset of AP GOT entry.



## object::elf::R_E2K_DISP

*Constant*: `u32`

PC relative 28 bit for DISP.



## object::elf::R_E2K_GOT

*Constant*: `u32`

32-bit offset of the symbol's entry in GOT.



## object::elf::R_E2K_GOTOFF

*Constant*: `u32`

32-bit offset of the symbol from GOT.



## object::elf::R_E2K_GOTPLT

*Constant*: `u32`

32-bit offset of the symbol's entry in .got.plt.



## object::elf::R_E2K_ISLOCAL

*Constant*: `u32`

Is symbol resolved locally during the link.
The result is encoded in 5-bit ALS.src1.



## object::elf::R_E2K_ISLOCAL32

*Constant*: `u32`

Is symbol resloved locally during the link.
The result is encoded in a long 32-bit LTS.



## object::elf::R_E2K_NONE

*Constant*: `u32`

No reloc.



## object::elf::R_E2K_PL

*Constant*: `u32`

Direct PL.



## object::elf::R_E2K_PL_GOT

*Constant*: `u32`

32-bit offset of PL GOT entry.



## object::elf::R_E2K_PREF

*Constant*: `u32`

Prefetch insn line containing the label (symbol).



## object::elf::R_E2K_TLS_32_DTPMOD

*Constant*: `u32`

ID of module containing symbol, 32-bit case.



## object::elf::R_E2K_TLS_32_DTPREL

*Constant*: `u32`

Offset in module TLS block, 32-bit case.



## object::elf::R_E2K_TLS_32_TPREL

*Constant*: `u32`

Offset in static TLS block, 32-bit case.



## object::elf::R_E2K_TLS_64_DTPMOD

*Constant*: `u32`

ID of module containing symbol, 64-bit case.



## object::elf::R_E2K_TLS_64_DTPREL

*Constant*: `u32`

Offset in module TLS block, 64-bit case.



## object::elf::R_E2K_TLS_64_TPREL

*Constant*: `u32`

Offset in static TLS block, 64-bit case.



## object::elf::R_E2K_TLS_GDMOD

*Constant*: `u32`

GOT entry for ID of module containing symbol.



## object::elf::R_E2K_TLS_GDREL

*Constant*: `u32`

GOT entry for offset in module TLS block.



## object::elf::R_E2K_TLS_IE

*Constant*: `u32`

Static TLS block offset GOT entry.



## object::elf::R_HEX_32

*Constant*: `u32`

Direct 32 bit



## object::elf::R_HEX_NONE

*Constant*: `u32`

No reloc



## object::elf::R_IA64_COPY

*Constant*: `u32`

copy relocation



## object::elf::R_IA64_DIR32LSB

*Constant*: `u32`

symbol + addend, data4 LSB



## object::elf::R_IA64_DIR32MSB

*Constant*: `u32`

symbol + addend, data4 MSB



## object::elf::R_IA64_DIR64LSB

*Constant*: `u32`

symbol + addend, data8 LSB



## object::elf::R_IA64_DIR64MSB

*Constant*: `u32`

symbol + addend, data8 MSB



## object::elf::R_IA64_DTPMOD64LSB

*Constant*: `u32`

@dtpmod(sym + add), data8 LSB



## object::elf::R_IA64_DTPMOD64MSB

*Constant*: `u32`

@dtpmod(sym + add), data8 MSB



## object::elf::R_IA64_DTPREL14

*Constant*: `u32`

@dtprel(sym + add), imm14



## object::elf::R_IA64_DTPREL22

*Constant*: `u32`

@dtprel(sym + add), imm22



## object::elf::R_IA64_DTPREL32LSB

*Constant*: `u32`

@dtprel(sym + add), data4 LSB



## object::elf::R_IA64_DTPREL32MSB

*Constant*: `u32`

@dtprel(sym + add), data4 MSB



## object::elf::R_IA64_DTPREL64I

*Constant*: `u32`

@dtprel(sym + add), imm64



## object::elf::R_IA64_DTPREL64LSB

*Constant*: `u32`

@dtprel(sym + add), data8 LSB



## object::elf::R_IA64_DTPREL64MSB

*Constant*: `u32`

@dtprel(sym + add), data8 MSB



## object::elf::R_IA64_FPTR32LSB

*Constant*: `u32`

@fptr(sym + add), data4 LSB



## object::elf::R_IA64_FPTR32MSB

*Constant*: `u32`

@fptr(sym + add), data4 MSB



## object::elf::R_IA64_FPTR64I

*Constant*: `u32`

@fptr(sym + add), mov imm64



## object::elf::R_IA64_FPTR64LSB

*Constant*: `u32`

@fptr(sym + add), data8 LSB



## object::elf::R_IA64_FPTR64MSB

*Constant*: `u32`

@fptr(sym + add), data8 MSB



## object::elf::R_IA64_GPREL22

*Constant*: `u32`

@gprel(sym + add), add imm22



## object::elf::R_IA64_GPREL32LSB

*Constant*: `u32`

@gprel(sym + add), data4 LSB



## object::elf::R_IA64_GPREL32MSB

*Constant*: `u32`

@gprel(sym + add), data4 MSB



## object::elf::R_IA64_GPREL64I

*Constant*: `u32`

@gprel(sym + add), mov imm64



## object::elf::R_IA64_GPREL64LSB

*Constant*: `u32`

@gprel(sym + add), data8 LSB



## object::elf::R_IA64_GPREL64MSB

*Constant*: `u32`

@gprel(sym + add), data8 MSB



## object::elf::R_IA64_IMM14

*Constant*: `u32`

symbol + addend, add imm14



## object::elf::R_IA64_IMM22

*Constant*: `u32`

symbol + addend, add imm22



## object::elf::R_IA64_IMM64

*Constant*: `u32`

symbol + addend, mov imm64



## object::elf::R_IA64_IPLTLSB

*Constant*: `u32`

dynamic reloc, imported PLT, LSB



## object::elf::R_IA64_IPLTMSB

*Constant*: `u32`

dynamic reloc, imported PLT, MSB



## object::elf::R_IA64_LDXMOV

*Constant*: `u32`

Use of LTOFF22X.



## object::elf::R_IA64_LTOFF22

*Constant*: `u32`

@ltoff(sym + add), add imm22



## object::elf::R_IA64_LTOFF22X

*Constant*: `u32`

LTOFF22, relaxable.



## object::elf::R_IA64_LTOFF64I

*Constant*: `u32`

@ltoff(sym + add), mov imm64



## object::elf::R_IA64_LTOFF_DTPMOD22

*Constant*: `u32`

@ltoff(@dtpmod(sym + add)), imm22



## object::elf::R_IA64_LTOFF_DTPREL22

*Constant*: `u32`

@ltoff(@dtprel(s+a)), imm22



## object::elf::R_IA64_LTOFF_FPTR22

*Constant*: `u32`

@ltoff(@fptr(s+a)), imm22



## object::elf::R_IA64_LTOFF_FPTR32LSB

*Constant*: `u32`

@ltoff(@fptr(s+a)), data4 LSB



## object::elf::R_IA64_LTOFF_FPTR32MSB

*Constant*: `u32`

@ltoff(@fptr(s+a)), data4 MSB



## object::elf::R_IA64_LTOFF_FPTR64I

*Constant*: `u32`

@ltoff(@fptr(s+a)), imm64



## object::elf::R_IA64_LTOFF_FPTR64LSB

*Constant*: `u32`

@ltoff(@fptr(s+a)), data8 LSB



## object::elf::R_IA64_LTOFF_FPTR64MSB

*Constant*: `u32`

@ltoff(@fptr(s+a)), data8 MSB



## object::elf::R_IA64_LTOFF_TPREL22

*Constant*: `u32`

@ltoff(@tprel(s+a)), imm2



## object::elf::R_IA64_LTV32LSB

*Constant*: `u32`

symbol + addend, data4 LSB



## object::elf::R_IA64_LTV32MSB

*Constant*: `u32`

symbol + addend, data4 MSB



## object::elf::R_IA64_LTV64LSB

*Constant*: `u32`

symbol + addend, data8 LSB



## object::elf::R_IA64_LTV64MSB

*Constant*: `u32`

symbol + addend, data8 MSB



## object::elf::R_IA64_NONE

*Constant*: `u32`

none



## object::elf::R_IA64_PCREL21B

*Constant*: `u32`

@pcrel(sym + add), ptb, call



## object::elf::R_IA64_PCREL21BI

*Constant*: `u32`

@pcrel(sym + add), 21bit inst



## object::elf::R_IA64_PCREL21F

*Constant*: `u32`

@pcrel(sym + add), fchkf



## object::elf::R_IA64_PCREL21M

*Constant*: `u32`

@pcrel(sym + add), chk.s



## object::elf::R_IA64_PCREL22

*Constant*: `u32`

@pcrel(sym + add), 22bit inst



## object::elf::R_IA64_PCREL32LSB

*Constant*: `u32`

@pcrel(sym + add), data4 LSB



## object::elf::R_IA64_PCREL32MSB

*Constant*: `u32`

@pcrel(sym + add), data4 MSB



## object::elf::R_IA64_PCREL60B

*Constant*: `u32`

@pcrel(sym + add), brl



## object::elf::R_IA64_PCREL64I

*Constant*: `u32`

@pcrel(sym + add), 64bit inst



## object::elf::R_IA64_PCREL64LSB

*Constant*: `u32`

@pcrel(sym + add), data8 LSB



## object::elf::R_IA64_PCREL64MSB

*Constant*: `u32`

@pcrel(sym + add), data8 MSB



## object::elf::R_IA64_PLTOFF22

*Constant*: `u32`

@pltoff(sym + add), add imm22



## object::elf::R_IA64_PLTOFF64I

*Constant*: `u32`

@pltoff(sym + add), mov imm64



## object::elf::R_IA64_PLTOFF64LSB

*Constant*: `u32`

@pltoff(sym + add), data8 LSB



## object::elf::R_IA64_PLTOFF64MSB

*Constant*: `u32`

@pltoff(sym + add), data8 MSB



## object::elf::R_IA64_REL32LSB

*Constant*: `u32`

data 4 + REL



## object::elf::R_IA64_REL32MSB

*Constant*: `u32`

data 4 + REL



## object::elf::R_IA64_REL64LSB

*Constant*: `u32`

data 8 + REL



## object::elf::R_IA64_REL64MSB

*Constant*: `u32`

data 8 + REL



## object::elf::R_IA64_SECREL32LSB

*Constant*: `u32`

@secrel(sym + add), data4 LSB



## object::elf::R_IA64_SECREL32MSB

*Constant*: `u32`

@secrel(sym + add), data4 MSB



## object::elf::R_IA64_SECREL64LSB

*Constant*: `u32`

@secrel(sym + add), data8 LSB



## object::elf::R_IA64_SECREL64MSB

*Constant*: `u32`

@secrel(sym + add), data8 MSB



## object::elf::R_IA64_SEGREL32LSB

*Constant*: `u32`

@segrel(sym + add), data4 LSB



## object::elf::R_IA64_SEGREL32MSB

*Constant*: `u32`

@segrel(sym + add), data4 MSB



## object::elf::R_IA64_SEGREL64LSB

*Constant*: `u32`

@segrel(sym + add), data8 LSB



## object::elf::R_IA64_SEGREL64MSB

*Constant*: `u32`

@segrel(sym + add), data8 MSB



## object::elf::R_IA64_SUB

*Constant*: `u32`

Addend and symbol difference



## object::elf::R_IA64_TPREL14

*Constant*: `u32`

@tprel(sym + add), imm14



## object::elf::R_IA64_TPREL22

*Constant*: `u32`

@tprel(sym + add), imm22



## object::elf::R_IA64_TPREL64I

*Constant*: `u32`

@tprel(sym + add), imm64



## object::elf::R_IA64_TPREL64LSB

*Constant*: `u32`

@tprel(sym + add), data8 LSB



## object::elf::R_IA64_TPREL64MSB

*Constant*: `u32`

@tprel(sym + add), data8 MSB



## object::elf::R_LARCH_32

*Constant*: `u32`

Runtime address resolving



## object::elf::R_LARCH_32_PCREL

*Constant*: `u32`

32-bit PC relative



## object::elf::R_LARCH_64

*Constant*: `u32`

Runtime address resolving



## object::elf::R_LARCH_64_PCREL

*Constant*: `u32`

64-bit PC relative



## object::elf::R_LARCH_ABS64_HI12

*Constant*: `u32`

52..=63 bits of 64-bit absolute address



## object::elf::R_LARCH_ABS64_LO20

*Constant*: `u32`

32..=51 bits of 64-bit absolute address



## object::elf::R_LARCH_ABS_HI20

*Constant*: `u32`

12..=31 bits of 32/64-bit absolute address



## object::elf::R_LARCH_ABS_LO12

*Constant*: `u32`

0..=11 bits of 32/64-bit absolute address



## object::elf::R_LARCH_ADD16

*Constant*: `u32`

16-bit in-place addition



## object::elf::R_LARCH_ADD24

*Constant*: `u32`

24-bit in-place addition



## object::elf::R_LARCH_ADD32

*Constant*: `u32`

32-bit in-place addition



## object::elf::R_LARCH_ADD6

*Constant*: `u32`

6-bit in-place addition



## object::elf::R_LARCH_ADD64

*Constant*: `u32`

64-bit in-place addition



## object::elf::R_LARCH_ADD8

*Constant*: `u32`

8-bit in-place addition



## object::elf::R_LARCH_ADD_ULEB128

*Constant*: `u32`

LEB128 in-place addition



## object::elf::R_LARCH_ALIGN

*Constant*: `u32`

Delete some bytes to ensure the instruction at PC + A aligned to
`A.next_power_of_two()`-byte boundary



## object::elf::R_LARCH_B16

*Constant*: `u32`

18-bit PC-relative jump offset with two trailing zeros



## object::elf::R_LARCH_B21

*Constant*: `u32`

23-bit PC-relative jump offset with two trailing zeros



## object::elf::R_LARCH_B26

*Constant*: `u32`

28-bit PC-relative jump offset with two trailing zeros



## object::elf::R_LARCH_CALL30

*Constant*: `u32`

12..=31 bits of `S + A - PC` into the `pcaddu12i` instruction at `PC`,
and 2..=11 bits of `S + A - PC` into the `jirl` instruction at `PC + 4`



## object::elf::R_LARCH_CALL36

*Constant*: `u32`

18..=37 bits of `S + A - PC` into the `pcaddu18i` instruction at `PC`,
and 2..=17 bits of `S + A - PC` into the `jirl` instruction at `PC + 4`



## object::elf::R_LARCH_CFA

*Constant*: `u32`

Reserved



## object::elf::R_LARCH_COPY

*Constant*: `u32`

Runtime memory copy in executable



## object::elf::R_LARCH_DELETE

*Constant*: `u32`

Reserved



## object::elf::R_LARCH_GNU_VTENTRY

*Constant*: `u32`

GNU C++ vtable member usage



## object::elf::R_LARCH_GNU_VTINHERIT

*Constant*: `u32`

GNU C++ vtable hierarchy



## object::elf::R_LARCH_GOT64_HI12

*Constant*: `u32`

52..=63 bits of 64-bit GOT entry absolute address



## object::elf::R_LARCH_GOT64_LO20

*Constant*: `u32`

32..=51 bits of 64-bit GOT entry absolute address



## object::elf::R_LARCH_GOT64_PC_HI12

*Constant*: `u32`

52..=63 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_GOT_PC_HI20] to the GOT entry.



## object::elf::R_LARCH_GOT64_PC_LO20

*Constant*: `u32`

32..=51 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_GOT_PC_HI20] to the GOT entry.



## object::elf::R_LARCH_GOT_HI20

*Constant*: `u32`

12..=31 bits of 32/64-bit GOT entry absolute address



## object::elf::R_LARCH_GOT_LO12

*Constant*: `u32`

0..=11 bits of 32/64-bit GOT entry absolute address



## object::elf::R_LARCH_GOT_PCADD_HI20

*Constant*: `u32`

The signed 32-bit offset `offs` from `PC` to
`(GP + G + 0x800) & 0xfffff000`.

We define the *PC relative anchor* for the GOT entry at `GP + G` as
`PC + offs` (`offs` is sign-extended to VA bits).



## object::elf::R_LARCH_GOT_PCADD_LO12

*Constant*: `u32`

0..=11 bits of the 32-bit offset from the
[PC relative anchor][R_LARCH_GOT_PCADD_HI20] to the GOT entry.



## object::elf::R_LARCH_GOT_PC_HI20

*Constant*: `u32`

The signed 32-bit offset `offs` from `PC & 0xfffff000` to
`(GP + G + 0x800) & 0xfffff000`, with 12 trailing zeros removed.

We define the *PC relative anchor* for the GOT entry at `GP + G` as
`PC + offs` (`offs` is sign-extended to VA bits).



## object::elf::R_LARCH_GOT_PC_LO12

*Constant*: `u32`

0..=11 bits of the 32/64-bit offset from the
[PC relative anchor][R_LARCH_GOT_PC_HI20] to the GOT entry.



## object::elf::R_LARCH_IRELATIVE

*Constant*: `u32`

Runtime local indirect function resolving



## object::elf::R_LARCH_JUMP_SLOT

*Constant*: `u32`

Runtime PLT supporting



## object::elf::R_LARCH_MARK_LA

*Constant*: `u32`

Mark la.abs: load absolute address for static link.



## object::elf::R_LARCH_MARK_PCREL

*Constant*: `u32`

Mark external label branch: access PC relative address for static link.



## object::elf::R_LARCH_NONE

*Constant*: `u32`

No reloc



## object::elf::R_LARCH_PCADD_HI20

*Constant*: `u32`

The signed 32-bit offset `offs` from `PC` to `(S + A + 0x800) & 0xfffff000`.

We define the *PC relative anchor* for `S + A` as `PC + offs` (`offs`
is sign-extended to VA bits).



## object::elf::R_LARCH_PCADD_LO12

*Constant*: `u32`

0..=11 bits of the 32-bit offset from the
[PC relative anchor][R_LARCH_PCADD_HI20].



## object::elf::R_LARCH_PCALA64_HI12

*Constant*: `u32`

52..=63 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_PCALA_HI20].



## object::elf::R_LARCH_PCALA64_LO20

*Constant*: `u32`

32..=51 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_PCALA_HI20].



## object::elf::R_LARCH_PCALA_HI20

*Constant*: `u32`

The signed 32-bit offset `offs` from `PC & 0xfffff000` to
`(S + A + 0x800) & 0xfffff000`, with 12 trailing zeros removed.

We define the *PC relative anchor* for `S + A` as `PC + offs` (`offs`
is sign-extended to VA bits).



## object::elf::R_LARCH_PCALA_LO12

*Constant*: `u32`

Same as R_LARCH_ABS_LO12.  0..=11 bits of the 32/64-bit offset from the
[PC relative anchor][R_LARCH_PCALA_HI20].



## object::elf::R_LARCH_PCREL20_S2

*Constant*: `u32`

22-bit PC-relative offset with two trailing zeros



## object::elf::R_LARCH_RELATIVE

*Constant*: `u32`

Runtime fixup for load-address



## object::elf::R_LARCH_RELAX

*Constant*: `u32`

Paired with a normal relocation at the same address to indicate the
instruction can be relaxed



## object::elf::R_LARCH_SOP_ADD

*Constant*: `u32`

Stack top addition (binary)



## object::elf::R_LARCH_SOP_AND

*Constant*: `u32`

Stack top bitwise and (binary)



## object::elf::R_LARCH_SOP_ASSERT

*Constant*: `u32`

Assert stack top



## object::elf::R_LARCH_SOP_IF_ELSE

*Constant*: `u32`

Stack top selection (tertiary)



## object::elf::R_LARCH_SOP_NOT

*Constant*: `u32`

Stack top logical not (unary)



## object::elf::R_LARCH_SOP_POP_32_S_0_10_10_16_S2

*Constant*: `u32`

Pop stack top to fill 28-bit signed immediate operand with two trailing
zeros implied



## object::elf::R_LARCH_SOP_POP_32_S_0_5_10_16_S2

*Constant*: `u32`

Pop stack top to fill 23-bit signed immediate operand with two trailing
zeros implied



## object::elf::R_LARCH_SOP_POP_32_S_10_12

*Constant*: `u32`

Pop stack top to fill 12-bit signed immediate operand



## object::elf::R_LARCH_SOP_POP_32_S_10_16

*Constant*: `u32`

Pop stack top to fill 16-bit signed immediate operand



## object::elf::R_LARCH_SOP_POP_32_S_10_16_S2

*Constant*: `u32`

Pop stack top to fill 18-bit signed immediate operand with two trailing
zeros implied



## object::elf::R_LARCH_SOP_POP_32_S_10_5

*Constant*: `u32`

Pop stack top to fill 5-bit signed immediate operand



## object::elf::R_LARCH_SOP_POP_32_S_5_20

*Constant*: `u32`

Pop stack top to fill 20-bit signed immediate operand



## object::elf::R_LARCH_SOP_POP_32_U

*Constant*: `u32`

Pop stack top to fill an instruction



## object::elf::R_LARCH_SOP_POP_32_U_10_12

*Constant*: `u32`

Pop stack top to fill 12-bit unsigned immediate operand



## object::elf::R_LARCH_SOP_PUSH_ABSOLUTE

*Constant*: `u32`

Push constant or absolute address



## object::elf::R_LARCH_SOP_PUSH_DUP

*Constant*: `u32`

Duplicate stack top



## object::elf::R_LARCH_SOP_PUSH_GPREL

*Constant*: `u32`

Push for access GOT entry



## object::elf::R_LARCH_SOP_PUSH_PCREL

*Constant*: `u32`

Push PC-relative offset



## object::elf::R_LARCH_SOP_PUSH_PLT_PCREL

*Constant*: `u32`

Push for external function calling



## object::elf::R_LARCH_SOP_PUSH_TLS_GD

*Constant*: `u32`

Push for TLS-GD



## object::elf::R_LARCH_SOP_PUSH_TLS_GOT

*Constant*: `u32`

Push for TLS-IE



## object::elf::R_LARCH_SOP_PUSH_TLS_TPREL

*Constant*: `u32`

Push for TLS-LE



## object::elf::R_LARCH_SOP_SL

*Constant*: `u32`

Stack top left shift (binary)



## object::elf::R_LARCH_SOP_SR

*Constant*: `u32`

Stack top right shift (binary)



## object::elf::R_LARCH_SOP_SUB

*Constant*: `u32`

Stack top subtraction (binary)



## object::elf::R_LARCH_SUB16

*Constant*: `u32`

16-bit in-place subtraction



## object::elf::R_LARCH_SUB24

*Constant*: `u32`

24-bit in-place subtraction



## object::elf::R_LARCH_SUB32

*Constant*: `u32`

32-bit in-place subtraction



## object::elf::R_LARCH_SUB6

*Constant*: `u32`

6-bit in-place subtraction



## object::elf::R_LARCH_SUB64

*Constant*: `u32`

64-bit in-place subtraction



## object::elf::R_LARCH_SUB8

*Constant*: `u32`

8-bit in-place subtraction



## object::elf::R_LARCH_SUB_ULEB128

*Constant*: `u32`

LEB128 in-place subtraction



## object::elf::R_LARCH_TLS_DESC32

*Constant*: `u32`

Runtime relocation for TLS descriptors



## object::elf::R_LARCH_TLS_DESC64

*Constant*: `u32`

Runtime relocation for TLS descriptors



## object::elf::R_LARCH_TLS_DESC64_HI12

*Constant*: `u32`

52..=63 bits of 64-bit TLS DESC GOT entry absolute address



## object::elf::R_LARCH_TLS_DESC64_LO20

*Constant*: `u32`

32..=51 bits of 64-bit TLS DESC GOT entry absolute address



## object::elf::R_LARCH_TLS_DESC64_PC_HI12

*Constant*: `u32`

52..=63 bits of 64-bit PC-relative offset to TLS DESC GOT entry



## object::elf::R_LARCH_TLS_DESC64_PC_LO20

*Constant*: `u32`

32..=51 bits of 64-bit PC-relative offset to TLS DESC GOT entry



## object::elf::R_LARCH_TLS_DESC_CALL

*Constant*: `u32`

Used on jirl for TLS DESC to call the resolve function



## object::elf::R_LARCH_TLS_DESC_HI20

*Constant*: `u32`

12..=31 bits of 32/64-bit TLS DESC GOT entry absolute address



## object::elf::R_LARCH_TLS_DESC_LD

*Constant*: `u32`

Used on ld.{w,d} for TLS DESC to get the resolve function address
from GOT entry



## object::elf::R_LARCH_TLS_DESC_LO12

*Constant*: `u32`

0..=11 bits of 32/64-bit TLS DESC GOT entry absolute address



## object::elf::R_LARCH_TLS_DESC_PCADD_HI20

*Constant*: `u32`

The signed 32-bit offset `offs` from `PC` to
`(GP + GD + 0x800) & 0xfffff000`.

We define the *PC relative anchor* for the TLS DESC GOT entry at
`GP + GD` as `PC + offs` (`offs` is sign-extended to VA bits).



## object::elf::R_LARCH_TLS_DESC_PCADD_LO12

*Constant*: `u32`

0..=11 bits of the 32-bit offset from the
[PC-relative anchor][R_LARCH_TLS_DESC_PCADD_HI20] to the TLS DESC GOT entry.



## object::elf::R_LARCH_TLS_DESC_PCREL20_S2

*Constant*: `u32`

22-bit PC-relative offset to TLS DESC GOT entry



## object::elf::R_LARCH_TLS_DESC_PC_HI20

*Constant*: `u32`

12..=31 bits of 32/64-bit PC-relative offset to TLS DESC GOT entry



## object::elf::R_LARCH_TLS_DESC_PC_LO12

*Constant*: `u32`

0..=11 bits of 32/64-bit TLS DESC GOT entry address



## object::elf::R_LARCH_TLS_DTPMOD32

*Constant*: `u32`

Runtime relocation for TLS-GD



## object::elf::R_LARCH_TLS_DTPMOD64

*Constant*: `u32`

Runtime relocation for TLS-GD



## object::elf::R_LARCH_TLS_DTPREL32

*Constant*: `u32`

Runtime relocation for TLS-GD



## object::elf::R_LARCH_TLS_DTPREL64

*Constant*: `u32`

Runtime relocation for TLS-GD



## object::elf::R_LARCH_TLS_GD_HI20

*Constant*: `u32`

12..=31 bits of TLS GD GOT entry 32/64-bit absolute address



## object::elf::R_LARCH_TLS_GD_PCADD_HI20

*Constant*: `u32`

The signed 32-bit offset `offs` from `PC` to
`(GP + GD + 0x800) & 0xfffff000`.

We define the *PC relative anchor* for the TLS GD GOT entry at
`GP + GD` as `PC + offs` (`offs` is sign-extended to VA bits).



## object::elf::R_LARCH_TLS_GD_PCADD_LO12

*Constant*: `u32`

0..=11 bits of the 32-bit offset from the
[PC-relative anchor][R_LARCH_TLS_GD_PCADD_HI20] to the TLS GD GOT entry.



## object::elf::R_LARCH_TLS_GD_PCREL20_S2

*Constant*: `u32`

22-bit PC-relative offset to TLS GD GOT entry



## object::elf::R_LARCH_TLS_GD_PC_HI20

*Constant*: `u32`

12..=31 bits of the 32/64-bit PC-relative offset to the PC-relative
anchor for the TLE GD GOT entry.



## object::elf::R_LARCH_TLS_IE64_HI12

*Constant*: `u32`

51..=63 bits of TLS IE GOT entry 64-bit absolute address



## object::elf::R_LARCH_TLS_IE64_LO20

*Constant*: `u32`

32..=51 bits of TLS IE GOT entry 64-bit absolute address



## object::elf::R_LARCH_TLS_IE64_PC_HI12

*Constant*: `u32`

52..=63 bits of the 64-bit offset from the
[PC-relative anchor][R_LARCH_TLS_IE_PC_HI20] to the TLS IE GOT entry.



## object::elf::R_LARCH_TLS_IE64_PC_LO20

*Constant*: `u32`

32..=51 bits of the 64-bit offset from the
[PC-relative anchor][R_LARCH_TLS_IE_PC_HI20] to the TLS IE GOT entry.



## object::elf::R_LARCH_TLS_IE_HI20

*Constant*: `u32`

12..=31 bits of TLS IE GOT entry 32/64-bit absolute address



## object::elf::R_LARCH_TLS_IE_LO12

*Constant*: `u32`

0..=11 bits of TLS IE GOT entry 32/64-bit absolute address



## object::elf::R_LARCH_TLS_IE_PCADD_HI20

*Constant*: `u32`

The signed 32-bit offset `offs` from `PC` to
`(GP + IE + 0x800) & 0xfffff000`.

We define the *PC relative anchor* for the TLS IE GOT entry at
`GP + IE` as `PC + offs` (`offs` is sign-extended to VA bits).



## object::elf::R_LARCH_TLS_IE_PCADD_LO12

*Constant*: `u32`

0..=11 bits of the 32-bit offset from the
[PC-relative anchor][R_LARCH_TLS_IE_PCADD_HI20] to the TLS IE GOT entry.



## object::elf::R_LARCH_TLS_IE_PC_HI20

*Constant*: `u32`

The signed 32-bit offset `offs` from `PC & 0xfffff000` to
`(GP + IE + 0x800) & 0xfffff000`, with 12 trailing zeros removed.

We define the *PC relative anchor* for the TLS IE GOT entry at
`GP + IE` as `PC + offs` (`offs` is sign-extended to VA bits).



## object::elf::R_LARCH_TLS_IE_PC_LO12

*Constant*: `u32`

0..=12 bits of the 32/64-bit offset from the
[PC-relative anchor][R_LARCH_TLS_IE_PC_HI20] to the TLS IE GOT entry.



## object::elf::R_LARCH_TLS_LD_HI20

*Constant*: `u32`

12..=31 bits of TLS LD GOT entry 32/64-bit absolute address



## object::elf::R_LARCH_TLS_LD_PCADD_HI20

*Constant*: `u32`

The signed 32-bit offset `offs` from `PC` to
`(GP + GD + 0x800) & 0xfffff000`.

We define the *PC relative anchor* for the TLS LD GOT entry at
`GP + GD` as `PC + offs` (`offs` is sign-extended to VA bits).



## object::elf::R_LARCH_TLS_LD_PCADD_LO12

*Constant*: `u32`

0..=11 bits of the 32-bit offset from the
[PC-relative anchor][R_LARCH_TLS_LD_PCADD_HI20] to the TLS LD GOT entry.



## object::elf::R_LARCH_TLS_LD_PCREL20_S2

*Constant*: `u32`

22-bit PC-relative offset to TLS LD GOT entry



## object::elf::R_LARCH_TLS_LD_PC_HI20

*Constant*: `u32`

12..=31 bits of the offset from `PC` to `GP + GD + 0x800`, where
`GP + GD` is a TLS LD GOT entry



## object::elf::R_LARCH_TLS_LE64_HI12

*Constant*: `u32`

52..=63 bits of TLS LE 64-bit offset from thread pointer



## object::elf::R_LARCH_TLS_LE64_LO20

*Constant*: `u32`

32..=51 bits of TLS LE 64-bit offset from thread pointer



## object::elf::R_LARCH_TLS_LE_ADD_R

*Constant*: `u32`

TLS LE thread pointer usage, can be relaxed



## object::elf::R_LARCH_TLS_LE_HI20

*Constant*: `u32`

12..=31 bits of TLS LE 32/64-bit offset from thread pointer



## object::elf::R_LARCH_TLS_LE_HI20_R

*Constant*: `u32`

12..=31 bits of TLS LE 32/64-bit offset from TP register, can be relaxed



## object::elf::R_LARCH_TLS_LE_LO12

*Constant*: `u32`

0..=11 bits of TLS LE 32/64-bit offset from thread pointer



## object::elf::R_LARCH_TLS_LE_LO12_R

*Constant*: `u32`

0..=11 bits of TLS LE 32/64-bit offset from TP register, sign-extended,
can be relaxed.



## object::elf::R_LARCH_TLS_TPREL32

*Constant*: `u32`

Runtime relocation for TLE-IE



## object::elf::R_LARCH_TLS_TPREL64

*Constant*: `u32`

Runtime relocation for TLE-IE



## object::elf::R_M32R_10_PCREL

*Constant*: `u32`

PC relative 10 bit shifted.



## object::elf::R_M32R_10_PCREL_RELA

*Constant*: `u32`

PC relative 10 bit shifted.



## object::elf::R_M32R_16

*Constant*: `u32`

Direct 16 bit.



## object::elf::R_M32R_16_RELA

*Constant*: `u32`

Direct 16 bit.



## object::elf::R_M32R_18_PCREL

*Constant*: `u32`

PC relative 18 bit shifted.



## object::elf::R_M32R_18_PCREL_RELA

*Constant*: `u32`

PC relative 18 bit shifted.



## object::elf::R_M32R_24

*Constant*: `u32`

Direct 24 bit.



## object::elf::R_M32R_24_RELA

*Constant*: `u32`

Direct 24 bit.



## object::elf::R_M32R_26_PCREL

*Constant*: `u32`

PC relative 26 bit shifted.



## object::elf::R_M32R_26_PCREL_RELA

*Constant*: `u32`

PC relative 26 bit shifted.



## object::elf::R_M32R_26_PLTREL

*Constant*: `u32`

26 bit PC relative to PLT shifted



## object::elf::R_M32R_32

*Constant*: `u32`

Direct 32 bit.



## object::elf::R_M32R_32_RELA

*Constant*: `u32`

Direct 32 bit.



## object::elf::R_M32R_COPY

*Constant*: `u32`

Copy symbol at runtime



## object::elf::R_M32R_GLOB_DAT

*Constant*: `u32`

Create GOT entry



## object::elf::R_M32R_GNU_VTENTRY

*Constant*: `u32`



## object::elf::R_M32R_GNU_VTINHERIT

*Constant*: `u32`



## object::elf::R_M32R_GOT16_HI_SLO

*Constant*: `u32`

High 16 bit GOT entry with signed low



## object::elf::R_M32R_GOT16_HI_ULO

*Constant*: `u32`

High 16 bit GOT entry with unsigned low



## object::elf::R_M32R_GOT16_LO

*Constant*: `u32`

Low 16 bit GOT entry



## object::elf::R_M32R_GOT24

*Constant*: `u32`

24 bit GOT entry



## object::elf::R_M32R_GOTOFF

*Constant*: `u32`

24 bit offset to GOT



## object::elf::R_M32R_GOTOFF_HI_SLO

*Constant*: `u32`

High 16 bit offset to GOT with signed low



## object::elf::R_M32R_GOTOFF_HI_ULO

*Constant*: `u32`

High 16 bit offset to GOT with unsigned low



## object::elf::R_M32R_GOTOFF_LO

*Constant*: `u32`

Low 16 bit offset to GOT



## object::elf::R_M32R_GOTPC24

*Constant*: `u32`

24 bit PC relative offset to GOT



## object::elf::R_M32R_GOTPC_HI_SLO

*Constant*: `u32`

High 16 bit PC relative offset to GOT with signed low



## object::elf::R_M32R_GOTPC_HI_ULO

*Constant*: `u32`

High 16 bit PC relative offset to GOT with unsigned low



## object::elf::R_M32R_GOTPC_LO

*Constant*: `u32`

Low 16 bit PC relative offset to GOT



## object::elf::R_M32R_HI16_SLO

*Constant*: `u32`

High 16 bit with signed low.



## object::elf::R_M32R_HI16_SLO_RELA

*Constant*: `u32`

High 16 bit with signed low



## object::elf::R_M32R_HI16_ULO

*Constant*: `u32`

High 16 bit with unsigned low.



## object::elf::R_M32R_HI16_ULO_RELA

*Constant*: `u32`

High 16 bit with unsigned low



## object::elf::R_M32R_JMP_SLOT

*Constant*: `u32`

Create PLT entry



## object::elf::R_M32R_LO16

*Constant*: `u32`

Low 16 bit.



## object::elf::R_M32R_LO16_RELA

*Constant*: `u32`

Low 16 bit



## object::elf::R_M32R_NONE

*Constant*: `u32`

No reloc.



## object::elf::R_M32R_NUM

*Constant*: `u32`

Keep this the last entry.



## object::elf::R_M32R_REL32

*Constant*: `u32`

PC relative 32 bit.



## object::elf::R_M32R_RELATIVE

*Constant*: `u32`

Adjust by program base



## object::elf::R_M32R_RELA_GNU_VTENTRY

*Constant*: `u32`



## object::elf::R_M32R_RELA_GNU_VTINHERIT

*Constant*: `u32`



## object::elf::R_M32R_SDA16

*Constant*: `u32`

16 bit offset in SDA.



## object::elf::R_M32R_SDA16_RELA

*Constant*: `u32`

16 bit offset in SDA



## object::elf::R_METAG_ADDR32

*Constant*: `u32`

32bit absolute address



## object::elf::R_METAG_COPY

*Constant*: `u32`



## object::elf::R_METAG_GETSETOFF

*Constant*: `u32`



## object::elf::R_METAG_GETSET_GOT

*Constant*: `u32`



## object::elf::R_METAG_GETSET_GOTOFF

*Constant*: `u32`



## object::elf::R_METAG_GLOB_DAT

*Constant*: `u32`



## object::elf::R_METAG_GNU_VTENTRY

*Constant*: `u32`



## object::elf::R_METAG_GNU_VTINHERIT

*Constant*: `u32`



## object::elf::R_METAG_GOTOFF

*Constant*: `u32`



## object::elf::R_METAG_HI16_GOTOFF

*Constant*: `u32`



## object::elf::R_METAG_HI16_GOTPC

*Constant*: `u32`



## object::elf::R_METAG_HI16_PLT

*Constant*: `u32`



## object::elf::R_METAG_HIADDR16

*Constant*: `u32`



## object::elf::R_METAG_HIOG

*Constant*: `u32`



## object::elf::R_METAG_JMP_SLOT

*Constant*: `u32`



## object::elf::R_METAG_LO16_GOTOFF

*Constant*: `u32`



## object::elf::R_METAG_LO16_GOTPC

*Constant*: `u32`



## object::elf::R_METAG_LO16_PLT

*Constant*: `u32`



## object::elf::R_METAG_LOADDR16

*Constant*: `u32`



## object::elf::R_METAG_LOOG

*Constant*: `u32`



## object::elf::R_METAG_NONE

*Constant*: `u32`

No reloc



## object::elf::R_METAG_PLT

*Constant*: `u32`



## object::elf::R_METAG_REG16OP1

*Constant*: `u32`



## object::elf::R_METAG_REG16OP2

*Constant*: `u32`



## object::elf::R_METAG_REG16OP3

*Constant*: `u32`



## object::elf::R_METAG_REG32OP1

*Constant*: `u32`



## object::elf::R_METAG_REG32OP2

*Constant*: `u32`



## object::elf::R_METAG_REG32OP3

*Constant*: `u32`



## object::elf::R_METAG_REG32OP4

*Constant*: `u32`



## object::elf::R_METAG_REL16

*Constant*: `u32`



## object::elf::R_METAG_REL8

*Constant*: `u32`



## object::elf::R_METAG_RELATIVE

*Constant*: `u32`



## object::elf::R_METAG_RELBRANCH

*Constant*: `u32`



## object::elf::R_METAG_RELBRANCH_PLT

*Constant*: `u32`



## object::elf::R_METAG_TLS_DTPMOD

*Constant*: `u32`



## object::elf::R_METAG_TLS_DTPOFF

*Constant*: `u32`



## object::elf::R_METAG_TLS_GD

*Constant*: `u32`



## object::elf::R_METAG_TLS_IE

*Constant*: `u32`



## object::elf::R_METAG_TLS_IENONPIC

*Constant*: `u32`



## object::elf::R_METAG_TLS_IENONPIC_HI16

*Constant*: `u32`



## object::elf::R_METAG_TLS_IENONPIC_LO16

*Constant*: `u32`



## object::elf::R_METAG_TLS_LDM

*Constant*: `u32`



## object::elf::R_METAG_TLS_LDO

*Constant*: `u32`



## object::elf::R_METAG_TLS_LDO_HI16

*Constant*: `u32`



## object::elf::R_METAG_TLS_LDO_LO16

*Constant*: `u32`



## object::elf::R_METAG_TLS_LE

*Constant*: `u32`



## object::elf::R_METAG_TLS_LE_HI16

*Constant*: `u32`



## object::elf::R_METAG_TLS_LE_LO16

*Constant*: `u32`



## object::elf::R_METAG_TLS_TPOFF

*Constant*: `u32`



## object::elf::R_MICROBLAZE_32

*Constant*: `u32`

Direct 32 bit.



## object::elf::R_MICROBLAZE_32_LO

*Constant*: `u32`

Low 16 bit.



## object::elf::R_MICROBLAZE_32_PCREL

*Constant*: `u32`

PC relative 32 bit.



## object::elf::R_MICROBLAZE_32_PCREL_LO

*Constant*: `u32`

Low 16 bits of PCREL32.



## object::elf::R_MICROBLAZE_32_SYM_OP_SYM

*Constant*: `u32`

Symbol Op Symbol relocation.



## object::elf::R_MICROBLAZE_64

*Constant*: `u32`

Direct 64 bit.



## object::elf::R_MICROBLAZE_64_NONE

*Constant*: `u32`

No reloc.



## object::elf::R_MICROBLAZE_64_PCREL

*Constant*: `u32`

PC relative 64 bit.



## object::elf::R_MICROBLAZE_COPY

*Constant*: `u32`

Runtime copy.



## object::elf::R_MICROBLAZE_GLOB_DAT

*Constant*: `u32`

Create GOT entry.



## object::elf::R_MICROBLAZE_GNU_VTENTRY

*Constant*: `u32`

GNU C++ vtable member usage.



## object::elf::R_MICROBLAZE_GNU_VTINHERIT

*Constant*: `u32`

GNU C++ vtable hierarchy.



## object::elf::R_MICROBLAZE_GOTOFF_32

*Constant*: `u32`

32 bit offset to GOT.



## object::elf::R_MICROBLAZE_GOTOFF_64

*Constant*: `u32`

64 bit offset to GOT.



## object::elf::R_MICROBLAZE_GOTPC_64

*Constant*: `u32`

PC-relative GOT offset.



## object::elf::R_MICROBLAZE_GOT_64

*Constant*: `u32`

GOT entry offset.



## object::elf::R_MICROBLAZE_JUMP_SLOT

*Constant*: `u32`

Create PLT entry.



## object::elf::R_MICROBLAZE_NONE

*Constant*: `u32`

No reloc.



## object::elf::R_MICROBLAZE_PLT_64

*Constant*: `u32`

PLT offset (PC-relative).



## object::elf::R_MICROBLAZE_REL

*Constant*: `u32`

Adjust by program base.



## object::elf::R_MICROBLAZE_SRO32

*Constant*: `u32`

Read-only small data area.



## object::elf::R_MICROBLAZE_SRW32

*Constant*: `u32`

Read-write small data area.



## object::elf::R_MICROBLAZE_TLS

*Constant*: `u32`

TLS Reloc.



## object::elf::R_MICROBLAZE_TLSDTPMOD32

*Constant*: `u32`

TLS Module ID.



## object::elf::R_MICROBLAZE_TLSDTPREL32

*Constant*: `u32`

TLS Offset Within TLS Block.



## object::elf::R_MICROBLAZE_TLSDTPREL64

*Constant*: `u32`

TLS Offset Within TLS Block.



## object::elf::R_MICROBLAZE_TLSGD

*Constant*: `u32`

TLS General Dynamic.



## object::elf::R_MICROBLAZE_TLSGOTTPREL32

*Constant*: `u32`

TLS Offset From Thread Pointer.



## object::elf::R_MICROBLAZE_TLSLD

*Constant*: `u32`

TLS Local Dynamic.



## object::elf::R_MICROBLAZE_TLSTPREL32

*Constant*: `u32`

TLS Offset From Thread Pointer.



## object::elf::R_MIPS_16

*Constant*: `u32`

Direct 16 bit



## object::elf::R_MIPS_26

*Constant*: `u32`

Direct 26 bit shifted



## object::elf::R_MIPS_32

*Constant*: `u32`

Direct 32 bit



## object::elf::R_MIPS_64

*Constant*: `u32`



## object::elf::R_MIPS_ADD_IMMEDIATE

*Constant*: `u32`



## object::elf::R_MIPS_CALL16

*Constant*: `u32`

16 bit GOT entry for function



## object::elf::R_MIPS_CALL_HI16

*Constant*: `u32`



## object::elf::R_MIPS_CALL_LO16

*Constant*: `u32`



## object::elf::R_MIPS_COPY

*Constant*: `u32`



## object::elf::R_MIPS_DELETE

*Constant*: `u32`



## object::elf::R_MIPS_GLOB_DAT

*Constant*: `u32`



## object::elf::R_MIPS_GOT16

*Constant*: `u32`

16 bit GOT entry



## object::elf::R_MIPS_GOT_DISP

*Constant*: `u32`



## object::elf::R_MIPS_GOT_HI16

*Constant*: `u32`



## object::elf::R_MIPS_GOT_LO16

*Constant*: `u32`



## object::elf::R_MIPS_GOT_OFST

*Constant*: `u32`



## object::elf::R_MIPS_GOT_PAGE

*Constant*: `u32`



## object::elf::R_MIPS_GPREL16

*Constant*: `u32`

GP relative 16 bit



## object::elf::R_MIPS_GPREL32

*Constant*: `u32`

GP relative 32 bit



## object::elf::R_MIPS_HI16

*Constant*: `u32`

High 16 bit



## object::elf::R_MIPS_HIGHER

*Constant*: `u32`



## object::elf::R_MIPS_HIGHEST

*Constant*: `u32`



## object::elf::R_MIPS_INSERT_A

*Constant*: `u32`



## object::elf::R_MIPS_INSERT_B

*Constant*: `u32`



## object::elf::R_MIPS_JALR

*Constant*: `u32`



## object::elf::R_MIPS_JUMP_SLOT

*Constant*: `u32`



## object::elf::R_MIPS_LITERAL

*Constant*: `u32`

16 bit literal entry



## object::elf::R_MIPS_LO16

*Constant*: `u32`

Low 16 bit



## object::elf::R_MIPS_NONE

*Constant*: `u32`

No reloc



## object::elf::R_MIPS_PC16

*Constant*: `u32`

PC relative 16 bit



## object::elf::R_MIPS_PJUMP

*Constant*: `u32`



## object::elf::R_MIPS_REL16

*Constant*: `u32`



## object::elf::R_MIPS_REL32

*Constant*: `u32`

PC relative 32 bit



## object::elf::R_MIPS_RELGOT

*Constant*: `u32`



## object::elf::R_MIPS_SCN_DISP

*Constant*: `u32`



## object::elf::R_MIPS_SHIFT5

*Constant*: `u32`



## object::elf::R_MIPS_SHIFT6

*Constant*: `u32`



## object::elf::R_MIPS_SUB

*Constant*: `u32`



## object::elf::R_MIPS_TLS_DTPMOD32

*Constant*: `u32`

Module number 32 bit



## object::elf::R_MIPS_TLS_DTPMOD64

*Constant*: `u32`

Module number 64 bit



## object::elf::R_MIPS_TLS_DTPREL32

*Constant*: `u32`

Module-relative offset 32 bit



## object::elf::R_MIPS_TLS_DTPREL64

*Constant*: `u32`

Module-relative offset 64 bit



## object::elf::R_MIPS_TLS_DTPREL_HI16

*Constant*: `u32`

Module-relative offset, high 16 bits



## object::elf::R_MIPS_TLS_DTPREL_LO16

*Constant*: `u32`

Module-relative offset, low 16 bits



## object::elf::R_MIPS_TLS_GD

*Constant*: `u32`

16 bit GOT offset for GD



## object::elf::R_MIPS_TLS_GOTTPREL

*Constant*: `u32`

16 bit GOT offset for IE



## object::elf::R_MIPS_TLS_LDM

*Constant*: `u32`

16 bit GOT offset for LDM



## object::elf::R_MIPS_TLS_TPREL32

*Constant*: `u32`

TP-relative offset, 32 bit



## object::elf::R_MIPS_TLS_TPREL64

*Constant*: `u32`

TP-relative offset, 64 bit



## object::elf::R_MIPS_TLS_TPREL_HI16

*Constant*: `u32`

TP-relative offset, high 16 bits



## object::elf::R_MIPS_TLS_TPREL_LO16

*Constant*: `u32`

TP-relative offset, low 16 bits



## object::elf::R_MN10300_16

*Constant*: `u32`

Direct 16 bit.



## object::elf::R_MN10300_24

*Constant*: `u32`

Direct 24 bit.



## object::elf::R_MN10300_32

*Constant*: `u32`

Direct 32 bit.



## object::elf::R_MN10300_8

*Constant*: `u32`

Direct 8 bit.



## object::elf::R_MN10300_ALIGN

*Constant*: `u32`

Alignment requirement for linker relaxation.



## object::elf::R_MN10300_COPY

*Constant*: `u32`

Copy symbol at runtime.



## object::elf::R_MN10300_GLOB_DAT

*Constant*: `u32`

Create GOT entry.



## object::elf::R_MN10300_GNU_VTENTRY

*Constant*: `u32`

... collection annotation.



## object::elf::R_MN10300_GNU_VTINHERIT

*Constant*: `u32`

Ancient C++ vtable garbage...



## object::elf::R_MN10300_GOT16

*Constant*: `u32`

16-bit offset to GOT entry.



## object::elf::R_MN10300_GOT24

*Constant*: `u32`

24-bit offset to GOT entry.



## object::elf::R_MN10300_GOT32

*Constant*: `u32`

32-bit offset to GOT entry.



## object::elf::R_MN10300_GOTOFF16

*Constant*: `u32`

16-bit offset from GOT.



## object::elf::R_MN10300_GOTOFF24

*Constant*: `u32`

24-bit offset from GOT.



## object::elf::R_MN10300_GOTOFF32

*Constant*: `u32`

32-bit offset from GOT.



## object::elf::R_MN10300_GOTPC16

*Constant*: `u32`

16-bit PCrel offset to GOT.



## object::elf::R_MN10300_GOTPC32

*Constant*: `u32`

32-bit PCrel offset to GOT.



## object::elf::R_MN10300_JMP_SLOT

*Constant*: `u32`

Create PLT entry.



## object::elf::R_MN10300_NONE

*Constant*: `u32`

No reloc.



## object::elf::R_MN10300_PCREL16

*Constant*: `u32`

PC-relative 16-bit signed.



## object::elf::R_MN10300_PCREL32

*Constant*: `u32`

PC-relative 32-bit.



## object::elf::R_MN10300_PCREL8

*Constant*: `u32`

PC-relative 8-bit signed.



## object::elf::R_MN10300_PLT16

*Constant*: `u32`

16-bit PCrel to PLT entry.



## object::elf::R_MN10300_PLT32

*Constant*: `u32`

32-bit PCrel to PLT entry.



## object::elf::R_MN10300_RELATIVE

*Constant*: `u32`

Adjust by program base.



## object::elf::R_MN10300_SYM_DIFF

*Constant*: `u32`

Adjustment for next reloc as needed by linker relaxation.



## object::elf::R_MN10300_TLS_DTPMOD

*Constant*: `u32`

ID of module containing symbol.



## object::elf::R_MN10300_TLS_DTPOFF

*Constant*: `u32`

Offset in module TLS block.



## object::elf::R_MN10300_TLS_GD

*Constant*: `u32`

32-bit offset for global dynamic.



## object::elf::R_MN10300_TLS_GOTIE

*Constant*: `u32`

GOT offset for static TLS block offset.



## object::elf::R_MN10300_TLS_IE

*Constant*: `u32`

GOT address for static TLS block offset.



## object::elf::R_MN10300_TLS_LD

*Constant*: `u32`

32-bit offset for local dynamic.



## object::elf::R_MN10300_TLS_LDO

*Constant*: `u32`

Module-relative offset.



## object::elf::R_MN10300_TLS_LE

*Constant*: `u32`

Offset relative to static TLS block.



## object::elf::R_MN10300_TLS_TPOFF

*Constant*: `u32`

Offset in static TLS block.



## object::elf::R_MSP430_16_BYTE

*Constant*: `u32`

Direct 16 bit



## object::elf::R_MSP430_32

*Constant*: `u32`

Direct 32 bit



## object::elf::R_MSP430_NONE

*Constant*: `u32`

No reloc



## object::elf::R_NDS32_32_RELA

*Constant*: `u32`



## object::elf::R_NDS32_COPY

*Constant*: `u32`



## object::elf::R_NDS32_GLOB_DAT

*Constant*: `u32`



## object::elf::R_NDS32_JMP_SLOT

*Constant*: `u32`



## object::elf::R_NDS32_NONE

*Constant*: `u32`



## object::elf::R_NDS32_RELATIVE

*Constant*: `u32`



## object::elf::R_NDS32_TLS_DESC

*Constant*: `u32`



## object::elf::R_NDS32_TLS_TPOFF

*Constant*: `u32`



## object::elf::R_NIOS2_ALIGN

*Constant*: `u32`

Alignment requirement for linker relaxation.



## object::elf::R_NIOS2_BFD_RELOC_16

*Constant*: `u32`

16 bit symbol value + addend.



## object::elf::R_NIOS2_BFD_RELOC_32

*Constant*: `u32`

32 bit symbol value + addend.



## object::elf::R_NIOS2_BFD_RELOC_8

*Constant*: `u32`

8 bit symbol value + addend.



## object::elf::R_NIOS2_CACHE_OPX

*Constant*: `u32`

5 bit expression, shift 22.



## object::elf::R_NIOS2_CALL16

*Constant*: `u32`

16 bit GOT entry for function.



## object::elf::R_NIOS2_CALL26

*Constant*: `u32`

Direct call.



## object::elf::R_NIOS2_CALL26_NOAT

*Constant*: `u32`

Direct call in .noat section.



## object::elf::R_NIOS2_CALLR

*Constant*: `u32`

Indirect call through register.



## object::elf::R_NIOS2_CALL_HA

*Constant*: `u32`

%hiadj() of function GOT entry.



## object::elf::R_NIOS2_CALL_LO

*Constant*: `u32`

%lo() of function GOT entry.



## object::elf::R_NIOS2_CJMP

*Constant*: `u32`

Conditional branch.



## object::elf::R_NIOS2_COPY

*Constant*: `u32`

Copy symbol at runtime.



## object::elf::R_NIOS2_GLOB_DAT

*Constant*: `u32`

Create GOT entry.



## object::elf::R_NIOS2_GNU_VTENTRY

*Constant*: `u32`

GNU C++ vtable member usage.



## object::elf::R_NIOS2_GNU_VTINHERIT

*Constant*: `u32`

GNU C++ vtable hierarchy.



## object::elf::R_NIOS2_GOT16

*Constant*: `u32`

16 bit GOT entry.



## object::elf::R_NIOS2_GOTOFF

*Constant*: `u32`

16 bit offset to GOT pointer.



## object::elf::R_NIOS2_GOTOFF_HA

*Constant*: `u32`

%hiadj of offset to GOT pointer.



## object::elf::R_NIOS2_GOTOFF_LO

*Constant*: `u32`

%lo of offset to GOT pointer.



## object::elf::R_NIOS2_GOT_HA

*Constant*: `u32`

%hiadj() of GOT entry.



## object::elf::R_NIOS2_GOT_LO

*Constant*: `u32`

%lo() of GOT entry.



## object::elf::R_NIOS2_GPREL

*Constant*: `u32`

16 bit GP pointer offset.



## object::elf::R_NIOS2_HI16

*Constant*: `u32`

High 16 bit.



## object::elf::R_NIOS2_HIADJ16

*Constant*: `u32`

High 16 bit, adjusted.



## object::elf::R_NIOS2_IMM5

*Constant*: `u32`

5 bit constant expression.



## object::elf::R_NIOS2_IMM6

*Constant*: `u32`

6 bit constant expression.



## object::elf::R_NIOS2_IMM8

*Constant*: `u32`

8 bit constant expression.



## object::elf::R_NIOS2_JUMP_SLOT

*Constant*: `u32`

Create PLT entry.



## object::elf::R_NIOS2_LO16

*Constant*: `u32`

Low 16 bit.



## object::elf::R_NIOS2_NONE

*Constant*: `u32`

No reloc.



## object::elf::R_NIOS2_PCREL16

*Constant*: `u32`

PC relative 16 bit.



## object::elf::R_NIOS2_PCREL_HA

*Constant*: `u32`

%hiadj of PC relative offset.



## object::elf::R_NIOS2_PCREL_LO

*Constant*: `u32`

%lo of PC relative offset.



## object::elf::R_NIOS2_RELATIVE

*Constant*: `u32`

Adjust by program base.



## object::elf::R_NIOS2_S16

*Constant*: `u32`

Direct signed 16 bit.



## object::elf::R_NIOS2_TLS_DTPMOD

*Constant*: `u32`

Module number.



## object::elf::R_NIOS2_TLS_DTPREL

*Constant*: `u32`

Module-relative offset.



## object::elf::R_NIOS2_TLS_GD16

*Constant*: `u32`

16 bit GOT offset for TLS GD.



## object::elf::R_NIOS2_TLS_IE16

*Constant*: `u32`

16 bit GOT offset for TLS IE.



## object::elf::R_NIOS2_TLS_LDM16

*Constant*: `u32`

16 bit GOT offset for TLS LDM.



## object::elf::R_NIOS2_TLS_LDO16

*Constant*: `u32`

16 bit module relative offset.



## object::elf::R_NIOS2_TLS_LE16

*Constant*: `u32`

16 bit LE TP-relative offset.



## object::elf::R_NIOS2_TLS_TPREL

*Constant*: `u32`

TP-relative offset.



## object::elf::R_NIOS2_U16

*Constant*: `u32`

Direct unsigned 16 bit.



## object::elf::R_NIOS2_UJMP

*Constant*: `u32`

Unconditional branch.



## object::elf::R_PARISC_COPY

*Constant*: `u32`

Copy relocation.



## object::elf::R_PARISC_DIR14DR

*Constant*: `u32`

14 bits of eff. address.



## object::elf::R_PARISC_DIR14R

*Constant*: `u32`

Right 14 bits of eff. address.



## object::elf::R_PARISC_DIR14WR

*Constant*: `u32`

14 bits of eff. address.



## object::elf::R_PARISC_DIR16DF

*Constant*: `u32`

16 bits of eff. address.



## object::elf::R_PARISC_DIR16F

*Constant*: `u32`

16 bits of eff. address.



## object::elf::R_PARISC_DIR16WF

*Constant*: `u32`

16 bits of eff. address.



## object::elf::R_PARISC_DIR17F

*Constant*: `u32`

17 bits of eff. address.



## object::elf::R_PARISC_DIR17R

*Constant*: `u32`

Right 17 bits of eff. address.



## object::elf::R_PARISC_DIR21L

*Constant*: `u32`

Left 21 bits of eff. address.



## object::elf::R_PARISC_DIR32

*Constant*: `u32`

Direct 32-bit reference.



## object::elf::R_PARISC_DIR64

*Constant*: `u32`

64 bits of eff. address.



## object::elf::R_PARISC_DPREL14R

*Constant*: `u32`

Right 14 bits of rel. address.



## object::elf::R_PARISC_DPREL21L

*Constant*: `u32`

Left 21 bits of rel. address.



## object::elf::R_PARISC_EPLT

*Constant*: `u32`

Dynamic reloc, exported PLT



## object::elf::R_PARISC_FPTR64

*Constant*: `u32`

64 bits function address.



## object::elf::R_PARISC_GNU_VTENTRY

*Constant*: `u32`



## object::elf::R_PARISC_GNU_VTINHERIT

*Constant*: `u32`



## object::elf::R_PARISC_GPREL14DR

*Constant*: `u32`

GP-rel. address, right 14 bits.



## object::elf::R_PARISC_GPREL14R

*Constant*: `u32`

GP-relative, right 14 bits.



## object::elf::R_PARISC_GPREL14WR

*Constant*: `u32`

GP-rel. address, right 14 bits.



## object::elf::R_PARISC_GPREL16DF

*Constant*: `u32`

16 bits GP-rel. address.



## object::elf::R_PARISC_GPREL16F

*Constant*: `u32`

16 bits GP-rel. address.



## object::elf::R_PARISC_GPREL16WF

*Constant*: `u32`

16 bits GP-rel. address.



## object::elf::R_PARISC_GPREL21L

*Constant*: `u32`

GP-relative, left 21 bits.



## object::elf::R_PARISC_GPREL64

*Constant*: `u32`

64 bits of GP-rel. address.



## object::elf::R_PARISC_HIRESERVE

*Constant*: `u32`



## object::elf::R_PARISC_IPLT

*Constant*: `u32`

Dynamic reloc, imported PLT



## object::elf::R_PARISC_LORESERVE

*Constant*: `u32`



## object::elf::R_PARISC_LTOFF14DR

*Constant*: `u32`

LT-rel. address, right 14 bits.



## object::elf::R_PARISC_LTOFF14R

*Constant*: `u32`

LT-relative, right 14 bits.



## object::elf::R_PARISC_LTOFF14WR

*Constant*: `u32`

LT-rel. address, right 14 bits.



## object::elf::R_PARISC_LTOFF16DF

*Constant*: `u32`

16 bits LT-rel. address.



## object::elf::R_PARISC_LTOFF16F

*Constant*: `u32`

16 bits LT-rel. address.



## object::elf::R_PARISC_LTOFF16WF

*Constant*: `u32`

16 bits LT-rel. address.



## object::elf::R_PARISC_LTOFF21L

*Constant*: `u32`

LT-relative, left 21 bits.



## object::elf::R_PARISC_LTOFF64

*Constant*: `u32`

64 bits LT-rel. address.



## object::elf::R_PARISC_LTOFF_FPTR14DR

*Constant*: `u32`

LT-rel. fct. ptr., right 14 bits.



## object::elf::R_PARISC_LTOFF_FPTR14R

*Constant*: `u32`

LT-rel. fct ptr, right 14 bits.



## object::elf::R_PARISC_LTOFF_FPTR14WR

*Constant*: `u32`

LT-rel. fct. ptr., right 14 bits.



## object::elf::R_PARISC_LTOFF_FPTR16DF

*Constant*: `u32`

16 bits LT-rel. function ptr.



## object::elf::R_PARISC_LTOFF_FPTR16F

*Constant*: `u32`

16 bits LT-rel. function ptr.



## object::elf::R_PARISC_LTOFF_FPTR16WF

*Constant*: `u32`

16 bits LT-rel. function ptr.



## object::elf::R_PARISC_LTOFF_FPTR21L

*Constant*: `u32`

LT-rel. fct ptr, left 21 bits.



## object::elf::R_PARISC_LTOFF_FPTR32

*Constant*: `u32`

32 bits LT-rel. function pointer.



## object::elf::R_PARISC_LTOFF_FPTR64

*Constant*: `u32`

64 bits LT-rel. function ptr.



## object::elf::R_PARISC_LTOFF_TP14DR

*Constant*: `u32`

LT-TP-rel. address, right 14 bits.



## object::elf::R_PARISC_LTOFF_TP14F

*Constant*: `u32`

14 bits LT-TP-rel. address.



## object::elf::R_PARISC_LTOFF_TP14R

*Constant*: `u32`

LT-TP-rel. address, right 14 bits.



## object::elf::R_PARISC_LTOFF_TP14WR

*Constant*: `u32`

LT-TP-rel. address, right 14 bits.



## object::elf::R_PARISC_LTOFF_TP16DF

*Constant*: `u32`

16 bits LT-TP-rel. address.



## object::elf::R_PARISC_LTOFF_TP16F

*Constant*: `u32`

16 bits LT-TP-rel. address.



## object::elf::R_PARISC_LTOFF_TP16WF

*Constant*: `u32`

16 bits LT-TP-rel. address.



## object::elf::R_PARISC_LTOFF_TP21L

*Constant*: `u32`

LT-TP-rel. address, left 21 bits.



## object::elf::R_PARISC_LTOFF_TP64

*Constant*: `u32`

64 bits LT-TP-rel. address.



## object::elf::R_PARISC_NONE

*Constant*: `u32`

No reloc.



## object::elf::R_PARISC_PCREL14DR

*Constant*: `u32`

PC rel. address, right 14 bits.



## object::elf::R_PARISC_PCREL14R

*Constant*: `u32`

Right 14 bits of rel. address.



## object::elf::R_PARISC_PCREL14WR

*Constant*: `u32`

PC-rel. address, right 14 bits.



## object::elf::R_PARISC_PCREL16DF

*Constant*: `u32`

16 bits PC-rel. address.



## object::elf::R_PARISC_PCREL16F

*Constant*: `u32`

16 bits PC-rel. address.



## object::elf::R_PARISC_PCREL16WF

*Constant*: `u32`

16 bits PC-rel. address.



## object::elf::R_PARISC_PCREL17F

*Constant*: `u32`

17 bits of rel. address.



## object::elf::R_PARISC_PCREL17R

*Constant*: `u32`

Right 17 bits of rel. address.



## object::elf::R_PARISC_PCREL21L

*Constant*: `u32`

Left 21 bits of rel. address.



## object::elf::R_PARISC_PCREL22F

*Constant*: `u32`

22 bits PC-rel. address.



## object::elf::R_PARISC_PCREL32

*Constant*: `u32`

32-bit rel. address.



## object::elf::R_PARISC_PCREL64

*Constant*: `u32`

64 bits PC-rel. address.



## object::elf::R_PARISC_PLABEL14R

*Constant*: `u32`

Right 14 bits of fdesc address.



## object::elf::R_PARISC_PLABEL21L

*Constant*: `u32`

Left 21 bits of fdesc address.



## object::elf::R_PARISC_PLABEL32

*Constant*: `u32`

32 bits function address.



## object::elf::R_PARISC_PLTOFF14DR

*Constant*: `u32`

PLT-rel. address, right 14 bits.



## object::elf::R_PARISC_PLTOFF14R

*Constant*: `u32`

PLT rel. address, right 14 bits.



## object::elf::R_PARISC_PLTOFF14WR

*Constant*: `u32`

PLT-rel. address, right 14 bits.



## object::elf::R_PARISC_PLTOFF16DF

*Constant*: `u32`

16 bits PLT-rel. address.



## object::elf::R_PARISC_PLTOFF16F

*Constant*: `u32`

16 bits LT-rel. address.



## object::elf::R_PARISC_PLTOFF16WF

*Constant*: `u32`

16 bits PLT-rel. address.



## object::elf::R_PARISC_PLTOFF21L

*Constant*: `u32`

PLT rel. address, left 21 bits.



## object::elf::R_PARISC_SECREL32

*Constant*: `u32`

32 bits section rel. address.



## object::elf::R_PARISC_SECREL64

*Constant*: `u32`

64 bits section rel. address.



## object::elf::R_PARISC_SEGBASE

*Constant*: `u32`

No relocation, set segment base.



## object::elf::R_PARISC_SEGREL32

*Constant*: `u32`

32 bits segment rel. address.



## object::elf::R_PARISC_SEGREL64

*Constant*: `u32`

64 bits segment rel. address.



## object::elf::R_PARISC_TLS_DTPMOD32

*Constant*: `u32`

DTP module 32-bit.



## object::elf::R_PARISC_TLS_DTPMOD64

*Constant*: `u32`

DTP module 64-bit.



## object::elf::R_PARISC_TLS_DTPOFF32

*Constant*: `u32`

DTP offset 32-bit.



## object::elf::R_PARISC_TLS_DTPOFF64

*Constant*: `u32`

DTP offset 32-bit.



## object::elf::R_PARISC_TLS_GD14R

*Constant*: `u32`

GD 14-bit right.



## object::elf::R_PARISC_TLS_GD21L

*Constant*: `u32`

GD 21-bit left.



## object::elf::R_PARISC_TLS_GDCALL

*Constant*: `u32`

GD call to __t_g_a.



## object::elf::R_PARISC_TLS_IE14R

*Constant*: `u32`



## object::elf::R_PARISC_TLS_IE21L

*Constant*: `u32`



## object::elf::R_PARISC_TLS_LDM14R

*Constant*: `u32`

LD module 14-bit right.



## object::elf::R_PARISC_TLS_LDM21L

*Constant*: `u32`

LD module 21-bit left.



## object::elf::R_PARISC_TLS_LDMCALL

*Constant*: `u32`

LD module call to __t_g_a.



## object::elf::R_PARISC_TLS_LDO14R

*Constant*: `u32`

LD offset 14-bit right.



## object::elf::R_PARISC_TLS_LDO21L

*Constant*: `u32`

LD offset 21-bit left.



## object::elf::R_PARISC_TLS_LE14R

*Constant*: `u32`



## object::elf::R_PARISC_TLS_LE21L

*Constant*: `u32`



## object::elf::R_PARISC_TLS_TPREL32

*Constant*: `u32`



## object::elf::R_PARISC_TLS_TPREL64

*Constant*: `u32`



## object::elf::R_PARISC_TPREL14DR

*Constant*: `u32`

TP-rel. address, right 14 bits.



## object::elf::R_PARISC_TPREL14R

*Constant*: `u32`

TP-rel. address, right 14 bits.



## object::elf::R_PARISC_TPREL14WR

*Constant*: `u32`

TP-rel. address, right 14 bits.



## object::elf::R_PARISC_TPREL16DF

*Constant*: `u32`

16 bits TP-rel. address.



## object::elf::R_PARISC_TPREL16F

*Constant*: `u32`

16 bits TP-rel. address.



## object::elf::R_PARISC_TPREL16WF

*Constant*: `u32`

16 bits TP-rel. address.



## object::elf::R_PARISC_TPREL21L

*Constant*: `u32`

TP-rel. address, left 21 bits.



## object::elf::R_PARISC_TPREL32

*Constant*: `u32`

32 bits TP-rel. address.



## object::elf::R_PARISC_TPREL64

*Constant*: `u32`

64 bits TP-rel. address.



## object::elf::R_PPC64_ADDR14

*Constant*: `u32`

16bit address, word aligned



## object::elf::R_PPC64_ADDR14_BRNTAKEN

*Constant*: `u32`



## object::elf::R_PPC64_ADDR14_BRTAKEN

*Constant*: `u32`



## object::elf::R_PPC64_ADDR16

*Constant*: `u32`

16bit absolute address



## object::elf::R_PPC64_ADDR16_DS

*Constant*: `u32`

half16ds* (S + A) >> 2



## object::elf::R_PPC64_ADDR16_HA

*Constant*: `u32`

adjusted high 16bits.



## object::elf::R_PPC64_ADDR16_HI

*Constant*: `u32`

high 16bits of address.



## object::elf::R_PPC64_ADDR16_HIGH

*Constant*: `u32`



## object::elf::R_PPC64_ADDR16_HIGHA

*Constant*: `u32`



## object::elf::R_PPC64_ADDR16_HIGHER

*Constant*: `u32`

half16 #higher(S + A)



## object::elf::R_PPC64_ADDR16_HIGHERA

*Constant*: `u32`

half16 #highera(S + A)



## object::elf::R_PPC64_ADDR16_HIGHEST

*Constant*: `u32`

half16 #highest(S + A)



## object::elf::R_PPC64_ADDR16_HIGHESTA

*Constant*: `u32`

half16 #highesta(S + A)



## object::elf::R_PPC64_ADDR16_LO

*Constant*: `u32`

lower 16bits of address



## object::elf::R_PPC64_ADDR16_LO_DS

*Constant*: `u32`

half16ds  #lo(S + A) >> 2



## object::elf::R_PPC64_ADDR24

*Constant*: `u32`

26bit address, word aligned



## object::elf::R_PPC64_ADDR30

*Constant*: `u32`

word30 (S + A - P) >> 2



## object::elf::R_PPC64_ADDR32

*Constant*: `u32`

32bit absolute address



## object::elf::R_PPC64_ADDR64

*Constant*: `u32`

doubleword64 S + A



## object::elf::R_PPC64_COPY

*Constant*: `u32`



## object::elf::R_PPC64_DTPMOD64

*Constant*: `u32`

doubleword64 (sym+add)@dtpmod



## object::elf::R_PPC64_DTPREL16

*Constant*: `u32`

half16* (sym+add)@dtprel



## object::elf::R_PPC64_DTPREL16_DS

*Constant*: `u32`

half16ds* (sym+add)@dtprel



## object::elf::R_PPC64_DTPREL16_HA

*Constant*: `u32`

half16  (sym+add)@dtprel@ha



## object::elf::R_PPC64_DTPREL16_HI

*Constant*: `u32`

half16  (sym+add)@dtprel@h



## object::elf::R_PPC64_DTPREL16_HIGH

*Constant*: `u32`



## object::elf::R_PPC64_DTPREL16_HIGHA

*Constant*: `u32`



## object::elf::R_PPC64_DTPREL16_HIGHER

*Constant*: `u32`

half16  (sym+add)@dtprel@higher



## object::elf::R_PPC64_DTPREL16_HIGHERA

*Constant*: `u32`

half16  (sym+add)@dtprel@highera



## object::elf::R_PPC64_DTPREL16_HIGHEST

*Constant*: `u32`

half16  (sym+add)@dtprel@highest



## object::elf::R_PPC64_DTPREL16_HIGHESTA

*Constant*: `u32`

half16  (sym+add)@dtprel@highesta



## object::elf::R_PPC64_DTPREL16_LO

*Constant*: `u32`

half16  (sym+add)@dtprel@l



## object::elf::R_PPC64_DTPREL16_LO_DS

*Constant*: `u32`

half16ds (sym+add)@dtprel@l



## object::elf::R_PPC64_DTPREL64

*Constant*: `u32`

doubleword64 (sym+add)@dtprel



## object::elf::R_PPC64_GLOB_DAT

*Constant*: `u32`



## object::elf::R_PPC64_GOT16

*Constant*: `u32`



## object::elf::R_PPC64_GOT16_DS

*Constant*: `u32`

half16ds* (G + A) >> 2



## object::elf::R_PPC64_GOT16_HA

*Constant*: `u32`



## object::elf::R_PPC64_GOT16_HI

*Constant*: `u32`



## object::elf::R_PPC64_GOT16_LO

*Constant*: `u32`



## object::elf::R_PPC64_GOT16_LO_DS

*Constant*: `u32`

half16ds  #lo(G + A) >> 2



## object::elf::R_PPC64_GOT_DTPREL16_DS

*Constant*: `u32`

half16ds* (sym+add)@got@dtprel



## object::elf::R_PPC64_GOT_DTPREL16_HA

*Constant*: `u32`

half16  (sym+add)@got@dtprel@ha



## object::elf::R_PPC64_GOT_DTPREL16_HI

*Constant*: `u32`

half16  (sym+add)@got@dtprel@h



## object::elf::R_PPC64_GOT_DTPREL16_LO_DS

*Constant*: `u32`

half16ds (sym+add)@got@dtprel@l



## object::elf::R_PPC64_GOT_TLSGD16

*Constant*: `u32`

half16* (sym+add)@got@tlsgd



## object::elf::R_PPC64_GOT_TLSGD16_HA

*Constant*: `u32`

half16  (sym+add)@got@tlsgd@ha



## object::elf::R_PPC64_GOT_TLSGD16_HI

*Constant*: `u32`

half16  (sym+add)@got@tlsgd@h



## object::elf::R_PPC64_GOT_TLSGD16_LO

*Constant*: `u32`

half16  (sym+add)@got@tlsgd@l



## object::elf::R_PPC64_GOT_TLSLD16

*Constant*: `u32`

half16* (sym+add)@got@tlsld



## object::elf::R_PPC64_GOT_TLSLD16_HA

*Constant*: `u32`

half16  (sym+add)@got@tlsld@ha



## object::elf::R_PPC64_GOT_TLSLD16_HI

*Constant*: `u32`

half16  (sym+add)@got@tlsld@h



## object::elf::R_PPC64_GOT_TLSLD16_LO

*Constant*: `u32`

half16  (sym+add)@got@tlsld@l



## object::elf::R_PPC64_GOT_TPREL16_DS

*Constant*: `u32`

half16ds* (sym+add)@got@tprel



## object::elf::R_PPC64_GOT_TPREL16_HA

*Constant*: `u32`

half16  (sym+add)@got@tprel@ha



## object::elf::R_PPC64_GOT_TPREL16_HI

*Constant*: `u32`

half16  (sym+add)@got@tprel@h



## object::elf::R_PPC64_GOT_TPREL16_LO_DS

*Constant*: `u32`

half16ds (sym+add)@got@tprel@l



## object::elf::R_PPC64_IRELATIVE

*Constant*: `u32`

GNU extension to support local ifunc.



## object::elf::R_PPC64_JMP_IREL

*Constant*: `u32`

GNU extension to support local ifunc.



## object::elf::R_PPC64_JMP_SLOT

*Constant*: `u32`



## object::elf::R_PPC64_NONE

*Constant*: `u32`



## object::elf::R_PPC64_PLT16_HA

*Constant*: `u32`



## object::elf::R_PPC64_PLT16_HI

*Constant*: `u32`



## object::elf::R_PPC64_PLT16_LO

*Constant*: `u32`



## object::elf::R_PPC64_PLT16_LO_DS

*Constant*: `u32`

half16ds  #lo(L + A) >> 2



## object::elf::R_PPC64_PLT32

*Constant*: `u32`



## object::elf::R_PPC64_PLT64

*Constant*: `u32`

doubleword64 L + A



## object::elf::R_PPC64_PLTGOT16

*Constant*: `u32`

half16* M + A



## object::elf::R_PPC64_PLTGOT16_DS

*Constant*: `u32`

half16ds* (M + A) >> 2



## object::elf::R_PPC64_PLTGOT16_HA

*Constant*: `u32`

half16 #ha(M + A)



## object::elf::R_PPC64_PLTGOT16_HI

*Constant*: `u32`

half16 #hi(M + A)



## object::elf::R_PPC64_PLTGOT16_LO

*Constant*: `u32`

half16 #lo(M + A)



## object::elf::R_PPC64_PLTGOT16_LO_DS

*Constant*: `u32`

half16ds  #lo(M + A) >> 2



## object::elf::R_PPC64_PLTREL32

*Constant*: `u32`



## object::elf::R_PPC64_PLTREL64

*Constant*: `u32`

doubleword64 L + A - P



## object::elf::R_PPC64_REL14

*Constant*: `u32`

PC relative 16 bit



## object::elf::R_PPC64_REL14_BRNTAKEN

*Constant*: `u32`



## object::elf::R_PPC64_REL14_BRTAKEN

*Constant*: `u32`



## object::elf::R_PPC64_REL16

*Constant*: `u32`

half16   (sym+add-.)



## object::elf::R_PPC64_REL16_HA

*Constant*: `u32`

half16   (sym+add-.)@ha



## object::elf::R_PPC64_REL16_HI

*Constant*: `u32`

half16   (sym+add-.)@h



## object::elf::R_PPC64_REL16_LO

*Constant*: `u32`

half16   (sym+add-.)@l



## object::elf::R_PPC64_REL24

*Constant*: `u32`

PC-rel. 26 bit, word aligned



## object::elf::R_PPC64_REL32

*Constant*: `u32`



## object::elf::R_PPC64_REL64

*Constant*: `u32`

doubleword64 S + A - P



## object::elf::R_PPC64_RELATIVE

*Constant*: `u32`



## object::elf::R_PPC64_SECTOFF

*Constant*: `u32`



## object::elf::R_PPC64_SECTOFF_DS

*Constant*: `u32`

half16ds* (R + A) >> 2



## object::elf::R_PPC64_SECTOFF_HA

*Constant*: `u32`



## object::elf::R_PPC64_SECTOFF_HI

*Constant*: `u32`



## object::elf::R_PPC64_SECTOFF_LO

*Constant*: `u32`



## object::elf::R_PPC64_SECTOFF_LO_DS

*Constant*: `u32`

half16ds  #lo(R + A) >> 2



## object::elf::R_PPC64_TLS

*Constant*: `u32`

none    (sym+add)@tls



## object::elf::R_PPC64_TLSGD

*Constant*: `u32`

none    (sym+add)@tlsgd



## object::elf::R_PPC64_TLSLD

*Constant*: `u32`

none    (sym+add)@tlsld



## object::elf::R_PPC64_TOC

*Constant*: `u32`

doubleword64 .TOC



## object::elf::R_PPC64_TOC16

*Constant*: `u32`

half16* S + A - .TOC



## object::elf::R_PPC64_TOC16_DS

*Constant*: `u32`

half16ds* (S + A - .TOC.) >> 2



## object::elf::R_PPC64_TOC16_HA

*Constant*: `u32`

half16 #ha(S + A - .TOC.)



## object::elf::R_PPC64_TOC16_HI

*Constant*: `u32`

half16 #hi(S + A - .TOC.)



## object::elf::R_PPC64_TOC16_LO

*Constant*: `u32`

half16 #lo(S + A - .TOC.)



## object::elf::R_PPC64_TOC16_LO_DS

*Constant*: `u32`

half16ds  #lo(S + A - .TOC.) >> 2



## object::elf::R_PPC64_TOCSAVE

*Constant*: `u32`

none



## object::elf::R_PPC64_TPREL16

*Constant*: `u32`

half16* (sym+add)@tprel



## object::elf::R_PPC64_TPREL16_DS

*Constant*: `u32`

half16ds* (sym+add)@tprel



## object::elf::R_PPC64_TPREL16_HA

*Constant*: `u32`

half16  (sym+add)@tprel@ha



## object::elf::R_PPC64_TPREL16_HI

*Constant*: `u32`

half16  (sym+add)@tprel@h



## object::elf::R_PPC64_TPREL16_HIGH

*Constant*: `u32`



## object::elf::R_PPC64_TPREL16_HIGHA

*Constant*: `u32`



## object::elf::R_PPC64_TPREL16_HIGHER

*Constant*: `u32`

half16  (sym+add)@tprel@higher



## object::elf::R_PPC64_TPREL16_HIGHERA

*Constant*: `u32`

half16  (sym+add)@tprel@highera



## object::elf::R_PPC64_TPREL16_HIGHEST

*Constant*: `u32`

half16  (sym+add)@tprel@highest



## object::elf::R_PPC64_TPREL16_HIGHESTA

*Constant*: `u32`

half16  (sym+add)@tprel@highesta



## object::elf::R_PPC64_TPREL16_LO

*Constant*: `u32`

half16  (sym+add)@tprel@l



## object::elf::R_PPC64_TPREL16_LO_DS

*Constant*: `u32`

half16ds (sym+add)@tprel@l



## object::elf::R_PPC64_TPREL64

*Constant*: `u32`

doubleword64 (sym+add)@tprel



## object::elf::R_PPC64_UADDR16

*Constant*: `u32`



## object::elf::R_PPC64_UADDR32

*Constant*: `u32`



## object::elf::R_PPC64_UADDR64

*Constant*: `u32`

doubleword64 S + A



## object::elf::R_PPC_ADDR14

*Constant*: `u32`

16bit address, 2 bits ignored



## object::elf::R_PPC_ADDR14_BRNTAKEN

*Constant*: `u32`



## object::elf::R_PPC_ADDR14_BRTAKEN

*Constant*: `u32`



## object::elf::R_PPC_ADDR16

*Constant*: `u32`

16bit absolute address



## object::elf::R_PPC_ADDR16_HA

*Constant*: `u32`

adjusted high 16bit



## object::elf::R_PPC_ADDR16_HI

*Constant*: `u32`

high 16bit of absolute address



## object::elf::R_PPC_ADDR16_LO

*Constant*: `u32`

lower 16bit of absolute address



## object::elf::R_PPC_ADDR24

*Constant*: `u32`

26bit address, 2 bits ignored.



## object::elf::R_PPC_ADDR32

*Constant*: `u32`

32bit absolute address



## object::elf::R_PPC_COPY

*Constant*: `u32`



## object::elf::R_PPC_DIAB_RELSDA_HA

*Constant*: `u32`

like EMB_RELSDA, adjusted high 16



## object::elf::R_PPC_DIAB_RELSDA_HI

*Constant*: `u32`

like EMB_RELSDA, but high 16 bit



## object::elf::R_PPC_DIAB_RELSDA_LO

*Constant*: `u32`

like EMB_RELSDA, but lower 16 bit



## object::elf::R_PPC_DIAB_SDA21_HA

*Constant*: `u32`

like EMB_SDA21, adjusted high 16



## object::elf::R_PPC_DIAB_SDA21_HI

*Constant*: `u32`

like EMB_SDA21, but high 16 bit



## object::elf::R_PPC_DIAB_SDA21_LO

*Constant*: `u32`

like EMB_SDA21, but lower 16 bit



## object::elf::R_PPC_DTPMOD32

*Constant*: `u32`

word32  (sym+add)@dtpmod



## object::elf::R_PPC_DTPREL16

*Constant*: `u32`

half16*(sym+add)@dtprel



## object::elf::R_PPC_DTPREL16_HA

*Constant*: `u32`

half16  (sym+add)@dtprel@ha



## object::elf::R_PPC_DTPREL16_HI

*Constant*: `u32`

half16  (sym+add)@dtprel@h



## object::elf::R_PPC_DTPREL16_LO

*Constant*: `u32`

half16  (sym+add)@dtprel@l



## object::elf::R_PPC_DTPREL32

*Constant*: `u32`

word32  (sym+add)@dtprel



## object::elf::R_PPC_EMB_BIT_FLD

*Constant*: `u32`



## object::elf::R_PPC_EMB_MRKREF

*Constant*: `u32`



## object::elf::R_PPC_EMB_NADDR16

*Constant*: `u32`



## object::elf::R_PPC_EMB_NADDR16_HA

*Constant*: `u32`



## object::elf::R_PPC_EMB_NADDR16_HI

*Constant*: `u32`



## object::elf::R_PPC_EMB_NADDR16_LO

*Constant*: `u32`



## object::elf::R_PPC_EMB_NADDR32

*Constant*: `u32`



## object::elf::R_PPC_EMB_RELSDA

*Constant*: `u32`

16 bit relative offset in SDA



## object::elf::R_PPC_EMB_RELSEC16

*Constant*: `u32`



## object::elf::R_PPC_EMB_RELST_HA

*Constant*: `u32`



## object::elf::R_PPC_EMB_RELST_HI

*Constant*: `u32`



## object::elf::R_PPC_EMB_RELST_LO

*Constant*: `u32`



## object::elf::R_PPC_EMB_SDA21

*Constant*: `u32`

16 bit offset in SDA



## object::elf::R_PPC_EMB_SDA2I16

*Constant*: `u32`



## object::elf::R_PPC_EMB_SDA2REL

*Constant*: `u32`



## object::elf::R_PPC_EMB_SDAI16

*Constant*: `u32`



## object::elf::R_PPC_GLOB_DAT

*Constant*: `u32`



## object::elf::R_PPC_GOT16

*Constant*: `u32`



## object::elf::R_PPC_GOT16_HA

*Constant*: `u32`



## object::elf::R_PPC_GOT16_HI

*Constant*: `u32`



## object::elf::R_PPC_GOT16_LO

*Constant*: `u32`



## object::elf::R_PPC_GOT_DTPREL16

*Constant*: `u32`

half16* (sym+add)@got@dtprel



## object::elf::R_PPC_GOT_DTPREL16_HA

*Constant*: `u32`

half16* (sym+add)@got@dtprel@ha



## object::elf::R_PPC_GOT_DTPREL16_HI

*Constant*: `u32`

half16* (sym+add)@got@dtprel@h



## object::elf::R_PPC_GOT_DTPREL16_LO

*Constant*: `u32`

half16* (sym+add)@got@dtprel@l



## object::elf::R_PPC_GOT_TLSGD16

*Constant*: `u32`

half16* (sym+add)@got@tlsgd



## object::elf::R_PPC_GOT_TLSGD16_HA

*Constant*: `u32`

half16  (sym+add)@got@tlsgd@ha



## object::elf::R_PPC_GOT_TLSGD16_HI

*Constant*: `u32`

half16  (sym+add)@got@tlsgd@h



## object::elf::R_PPC_GOT_TLSGD16_LO

*Constant*: `u32`

half16  (sym+add)@got@tlsgd@l



## object::elf::R_PPC_GOT_TLSLD16

*Constant*: `u32`

half16* (sym+add)@got@tlsld



## object::elf::R_PPC_GOT_TLSLD16_HA

*Constant*: `u32`

half16  (sym+add)@got@tlsld@ha



## object::elf::R_PPC_GOT_TLSLD16_HI

*Constant*: `u32`

half16  (sym+add)@got@tlsld@h



## object::elf::R_PPC_GOT_TLSLD16_LO

*Constant*: `u32`

half16  (sym+add)@got@tlsld@l



## object::elf::R_PPC_GOT_TPREL16

*Constant*: `u32`

half16* (sym+add)@got@tprel



## object::elf::R_PPC_GOT_TPREL16_HA

*Constant*: `u32`

half16  (sym+add)@got@tprel@ha



## object::elf::R_PPC_GOT_TPREL16_HI

*Constant*: `u32`

half16  (sym+add)@got@tprel@h



## object::elf::R_PPC_GOT_TPREL16_LO

*Constant*: `u32`

half16  (sym+add)@got@tprel@l



## object::elf::R_PPC_IRELATIVE

*Constant*: `u32`

GNU extension to support local ifunc.



## object::elf::R_PPC_JMP_SLOT

*Constant*: `u32`



## object::elf::R_PPC_LOCAL24PC

*Constant*: `u32`



## object::elf::R_PPC_NONE

*Constant*: `u32`



## object::elf::R_PPC_PLT16_HA

*Constant*: `u32`



## object::elf::R_PPC_PLT16_HI

*Constant*: `u32`



## object::elf::R_PPC_PLT16_LO

*Constant*: `u32`



## object::elf::R_PPC_PLT32

*Constant*: `u32`



## object::elf::R_PPC_PLTREL24

*Constant*: `u32`



## object::elf::R_PPC_PLTREL32

*Constant*: `u32`



## object::elf::R_PPC_REL14

*Constant*: `u32`

PC relative 16 bit



## object::elf::R_PPC_REL14_BRNTAKEN

*Constant*: `u32`



## object::elf::R_PPC_REL14_BRTAKEN

*Constant*: `u32`



## object::elf::R_PPC_REL16

*Constant*: `u32`

half16   (sym+add-.)



## object::elf::R_PPC_REL16_HA

*Constant*: `u32`

half16   (sym+add-.)@ha



## object::elf::R_PPC_REL16_HI

*Constant*: `u32`

half16   (sym+add-.)@h



## object::elf::R_PPC_REL16_LO

*Constant*: `u32`

half16   (sym+add-.)@l



## object::elf::R_PPC_REL24

*Constant*: `u32`

PC relative 26 bit



## object::elf::R_PPC_REL32

*Constant*: `u32`



## object::elf::R_PPC_RELATIVE

*Constant*: `u32`



## object::elf::R_PPC_SDAREL16

*Constant*: `u32`



## object::elf::R_PPC_SECTOFF

*Constant*: `u32`



## object::elf::R_PPC_SECTOFF_HA

*Constant*: `u32`



## object::elf::R_PPC_SECTOFF_HI

*Constant*: `u32`



## object::elf::R_PPC_SECTOFF_LO

*Constant*: `u32`



## object::elf::R_PPC_TLS

*Constant*: `u32`

none    (sym+add)@tls



## object::elf::R_PPC_TLSGD

*Constant*: `u32`

none    (sym+add)@tlsgd



## object::elf::R_PPC_TLSLD

*Constant*: `u32`

none    (sym+add)@tlsld



## object::elf::R_PPC_TOC16

*Constant*: `u32`

This is a phony reloc to handle any old fashioned TOC16 references that may
still be in object files.



## object::elf::R_PPC_TPREL16

*Constant*: `u32`

half16* (sym+add)@tprel



## object::elf::R_PPC_TPREL16_HA

*Constant*: `u32`

half16  (sym+add)@tprel@ha



## object::elf::R_PPC_TPREL16_HI

*Constant*: `u32`

half16  (sym+add)@tprel@h



## object::elf::R_PPC_TPREL16_LO

*Constant*: `u32`

half16  (sym+add)@tprel@l



## object::elf::R_PPC_TPREL32

*Constant*: `u32`

word32  (sym+add)@tprel



## object::elf::R_PPC_UADDR16

*Constant*: `u32`



## object::elf::R_PPC_UADDR32

*Constant*: `u32`



## object::elf::R_RISCV_32

*Constant*: `u32`



## object::elf::R_RISCV_32_PCREL

*Constant*: `u32`



## object::elf::R_RISCV_64

*Constant*: `u32`



## object::elf::R_RISCV_ADD16

*Constant*: `u32`



## object::elf::R_RISCV_ADD32

*Constant*: `u32`



## object::elf::R_RISCV_ADD64

*Constant*: `u32`



## object::elf::R_RISCV_ADD8

*Constant*: `u32`



## object::elf::R_RISCV_ALIGN

*Constant*: `u32`



## object::elf::R_RISCV_BRANCH

*Constant*: `u32`



## object::elf::R_RISCV_CALL

*Constant*: `u32`



## object::elf::R_RISCV_CALL_PLT

*Constant*: `u32`



## object::elf::R_RISCV_COPY

*Constant*: `u32`



## object::elf::R_RISCV_GOT32_PCREL

*Constant*: `u32`



## object::elf::R_RISCV_GOT_HI20

*Constant*: `u32`



## object::elf::R_RISCV_GPREL_I

*Constant*: `u32`



## object::elf::R_RISCV_GPREL_S

*Constant*: `u32`



## object::elf::R_RISCV_HI20

*Constant*: `u32`



## object::elf::R_RISCV_IRELATIVE

*Constant*: `u32`



## object::elf::R_RISCV_JAL

*Constant*: `u32`



## object::elf::R_RISCV_JUMP_SLOT

*Constant*: `u32`



## object::elf::R_RISCV_LO12_I

*Constant*: `u32`



## object::elf::R_RISCV_LO12_S

*Constant*: `u32`



## object::elf::R_RISCV_NONE

*Constant*: `u32`



## object::elf::R_RISCV_PCREL_HI20

*Constant*: `u32`



## object::elf::R_RISCV_PCREL_LO12_I

*Constant*: `u32`



## object::elf::R_RISCV_PCREL_LO12_S

*Constant*: `u32`



## object::elf::R_RISCV_PLT32

*Constant*: `u32`



## object::elf::R_RISCV_RELATIVE

*Constant*: `u32`



## object::elf::R_RISCV_RELAX

*Constant*: `u32`



## object::elf::R_RISCV_RVC_BRANCH

*Constant*: `u32`



## object::elf::R_RISCV_RVC_JUMP

*Constant*: `u32`



## object::elf::R_RISCV_RVC_LUI

*Constant*: `u32`



## object::elf::R_RISCV_SET16

*Constant*: `u32`



## object::elf::R_RISCV_SET32

*Constant*: `u32`



## object::elf::R_RISCV_SET6

*Constant*: `u32`



## object::elf::R_RISCV_SET8

*Constant*: `u32`



## object::elf::R_RISCV_SET_ULEB128

*Constant*: `u32`



## object::elf::R_RISCV_SUB16

*Constant*: `u32`



## object::elf::R_RISCV_SUB32

*Constant*: `u32`



## object::elf::R_RISCV_SUB6

*Constant*: `u32`



## object::elf::R_RISCV_SUB64

*Constant*: `u32`



## object::elf::R_RISCV_SUB8

*Constant*: `u32`



## object::elf::R_RISCV_SUB_ULEB128

*Constant*: `u32`



## object::elf::R_RISCV_TLSDESC

*Constant*: `u32`



## object::elf::R_RISCV_TLSDESC_ADD_LO12

*Constant*: `u32`



## object::elf::R_RISCV_TLSDESC_CALL

*Constant*: `u32`



## object::elf::R_RISCV_TLSDESC_HI20

*Constant*: `u32`



## object::elf::R_RISCV_TLSDESC_LOAD_LO12

*Constant*: `u32`



## object::elf::R_RISCV_TLS_DTPMOD32

*Constant*: `u32`



## object::elf::R_RISCV_TLS_DTPMOD64

*Constant*: `u32`



## object::elf::R_RISCV_TLS_DTPREL32

*Constant*: `u32`



## object::elf::R_RISCV_TLS_DTPREL64

*Constant*: `u32`



## object::elf::R_RISCV_TLS_GD_HI20

*Constant*: `u32`



## object::elf::R_RISCV_TLS_GOT_HI20

*Constant*: `u32`



## object::elf::R_RISCV_TLS_TPREL32

*Constant*: `u32`



## object::elf::R_RISCV_TLS_TPREL64

*Constant*: `u32`



## object::elf::R_RISCV_TPREL_ADD

*Constant*: `u32`



## object::elf::R_RISCV_TPREL_HI20

*Constant*: `u32`



## object::elf::R_RISCV_TPREL_I

*Constant*: `u32`



## object::elf::R_RISCV_TPREL_LO12_I

*Constant*: `u32`



## object::elf::R_RISCV_TPREL_LO12_S

*Constant*: `u32`



## object::elf::R_RISCV_TPREL_S

*Constant*: `u32`



## object::elf::R_SBF_64_32

*Constant*: `u32`



## object::elf::R_SBF_64_64

*Constant*: `u32`



## object::elf::R_SBF_NONE

*Constant*: `u32`

No reloc



## object::elf::R_SHARC_ADDR24_V3

*Constant*: `u32`

24-bit absolute address in bits 23:0 of a 48-bit instr

Targets:

* Type 25a (PC_DIRECT)



## object::elf::R_SHARC_ADDR32_V3

*Constant*: `u32`

32-bit absolute address in bits 31:0 of a 48-bit instr

Targets:

* Type 14a
* Type 14d
* Type 15a
* Type 16a
* Type 17a
* Type 18a
* Type 19a



## object::elf::R_SHARC_ADDR_VAR16_V3

*Constant*: `u32`

16-bit absolute address into bits 15:0 of a 16-bit location.

Represented with `RelocationEncoding::Generic`



## object::elf::R_SHARC_ADDR_VAR_V3

*Constant*: `u32`

32-bit absolute address in bits 31:0 of a 32-bit data location

Represented with `RelocationEncoding::Generic`



## object::elf::R_SHARC_CALC_ADD

*Constant*: `u32`



## object::elf::R_SHARC_CALC_AND

*Constant*: `u32`



## object::elf::R_SHARC_CALC_DIV

*Constant*: `u32`



## object::elf::R_SHARC_CALC_LSHIFT

*Constant*: `u32`



## object::elf::R_SHARC_CALC_MOD

*Constant*: `u32`



## object::elf::R_SHARC_CALC_MUL

*Constant*: `u32`



## object::elf::R_SHARC_CALC_NOT

*Constant*: `u32`



## object::elf::R_SHARC_CALC_OR

*Constant*: `u32`



## object::elf::R_SHARC_CALC_PUSH_ADDEND

*Constant*: `u32`



## object::elf::R_SHARC_CALC_PUSH_ADDR

*Constant*: `u32`



## object::elf::R_SHARC_CALC_PUSH_LEN

*Constant*: `u32`



## object::elf::R_SHARC_CALC_RSHIFT

*Constant*: `u32`



## object::elf::R_SHARC_CALC_SUB

*Constant*: `u32`



## object::elf::R_SHARC_CALC_XOR

*Constant*: `u32`



## object::elf::R_SHARC_DATA16_V3

*Constant*: `u32`

16-bit absolute address in bits 39:24 of a 48-bit instr

Targets:

* Type 12a



## object::elf::R_SHARC_DATA16_VISA_V3

*Constant*: `u32`

16-bit absolute address into bits 15:0 of a 32-bit instr



## object::elf::R_SHARC_DATA6_V3

*Constant*: `u32`

6-bit absolute address in bits 32:27 of a 48-bit instr

Targets:

* Type 4a
* Type 4b
* Type 4d



## object::elf::R_SHARC_DATA6_VISA_V3

*Constant*: `u32`

6-bit absolute address into bits 16:11 of a 32-bit instr

Targets:

* Type 4b



## object::elf::R_SHARC_DATA7_VISA_V3

*Constant*: `u32`

7-bit absolute address into bits 6:0 of a 32-bit instr



## object::elf::R_SHARC_PCR6_VISA_V3

*Constant*: `u32`

6-bit PC-relative address into bits 16:11 of a Type B

Targets:

* Type 9b



## object::elf::R_SHARC_PCRLONG_V3

*Constant*: `u32`

24-bit PC-relative address in bits 23:0 of a 48-bit instr

Targets:

* Type 8a
* Type 12a (truncated to 23 bits after relocation)
* Type 13a (truncated to 23 bits after relocation)
* Type 25a (PC Relative)



## object::elf::R_SHARC_PCRSHORT_V3

*Constant*: `u32`

6-bit PC-relative address in bits 32:27 of a 48-bit instr

Targets:

* Type 9a
* Type 10a



## object::elf::R_SH_ALIGN

*Constant*: `u32`



## object::elf::R_SH_CODE

*Constant*: `u32`



## object::elf::R_SH_COPY

*Constant*: `u32`



## object::elf::R_SH_COUNT

*Constant*: `u32`



## object::elf::R_SH_DATA

*Constant*: `u32`



## object::elf::R_SH_DIR32

*Constant*: `u32`



## object::elf::R_SH_DIR8BP

*Constant*: `u32`



## object::elf::R_SH_DIR8L

*Constant*: `u32`



## object::elf::R_SH_DIR8W

*Constant*: `u32`



## object::elf::R_SH_DIR8WPL

*Constant*: `u32`



## object::elf::R_SH_DIR8WPN

*Constant*: `u32`



## object::elf::R_SH_DIR8WPZ

*Constant*: `u32`



## object::elf::R_SH_GLOB_DAT

*Constant*: `u32`



## object::elf::R_SH_GNU_VTENTRY

*Constant*: `u32`



## object::elf::R_SH_GNU_VTINHERIT

*Constant*: `u32`



## object::elf::R_SH_GOT32

*Constant*: `u32`



## object::elf::R_SH_GOTOFF

*Constant*: `u32`



## object::elf::R_SH_GOTPC

*Constant*: `u32`



## object::elf::R_SH_IND12W

*Constant*: `u32`



## object::elf::R_SH_JMP_SLOT

*Constant*: `u32`



## object::elf::R_SH_LABEL

*Constant*: `u32`



## object::elf::R_SH_NONE

*Constant*: `u32`



## object::elf::R_SH_PLT32

*Constant*: `u32`



## object::elf::R_SH_REL32

*Constant*: `u32`



## object::elf::R_SH_RELATIVE

*Constant*: `u32`



## object::elf::R_SH_SWITCH16

*Constant*: `u32`



## object::elf::R_SH_SWITCH32

*Constant*: `u32`



## object::elf::R_SH_SWITCH8

*Constant*: `u32`



## object::elf::R_SH_TLS_DTPMOD32

*Constant*: `u32`



## object::elf::R_SH_TLS_DTPOFF32

*Constant*: `u32`



## object::elf::R_SH_TLS_GD_32

*Constant*: `u32`



## object::elf::R_SH_TLS_IE_32

*Constant*: `u32`



## object::elf::R_SH_TLS_LDO_32

*Constant*: `u32`



## object::elf::R_SH_TLS_LD_32

*Constant*: `u32`



## object::elf::R_SH_TLS_LE_32

*Constant*: `u32`



## object::elf::R_SH_TLS_TPOFF32

*Constant*: `u32`



## object::elf::R_SH_USES

*Constant*: `u32`



## object::elf::R_SPARC_10

*Constant*: `u32`

Direct 10 bit



## object::elf::R_SPARC_11

*Constant*: `u32`

Direct 11 bit



## object::elf::R_SPARC_13

*Constant*: `u32`

Direct 13 bit



## object::elf::R_SPARC_16

*Constant*: `u32`

Direct 16 bit



## object::elf::R_SPARC_22

*Constant*: `u32`

Direct 22 bit



## object::elf::R_SPARC_32

*Constant*: `u32`

Direct 32 bit



## object::elf::R_SPARC_5

*Constant*: `u32`

Direct 5 bit



## object::elf::R_SPARC_6

*Constant*: `u32`

Direct 6 bit



## object::elf::R_SPARC_64

*Constant*: `u32`

Direct 64 bit



## object::elf::R_SPARC_7

*Constant*: `u32`

Direct 7 bit



## object::elf::R_SPARC_8

*Constant*: `u32`

Direct 8 bit



## object::elf::R_SPARC_COPY

*Constant*: `u32`

Copy symbol at runtime



## object::elf::R_SPARC_DISP16

*Constant*: `u32`

PC relative 16 bit



## object::elf::R_SPARC_DISP32

*Constant*: `u32`

PC relative 32 bit



## object::elf::R_SPARC_DISP64

*Constant*: `u32`

PC relative 64 bit



## object::elf::R_SPARC_DISP8

*Constant*: `u32`

PC relative 8 bit



## object::elf::R_SPARC_GLOB_DAT

*Constant*: `u32`

Create GOT entry



## object::elf::R_SPARC_GLOB_JMP

*Constant*: `u32`

was part of v9 ABI but was removed



## object::elf::R_SPARC_GNU_VTENTRY

*Constant*: `u32`



## object::elf::R_SPARC_GNU_VTINHERIT

*Constant*: `u32`



## object::elf::R_SPARC_GOT10

*Constant*: `u32`

Truncated 10 bit GOT entry



## object::elf::R_SPARC_GOT13

*Constant*: `u32`

13 bit GOT entry



## object::elf::R_SPARC_GOT22

*Constant*: `u32`

22 bit GOT entry shifted



## object::elf::R_SPARC_GOTDATA_HIX22

*Constant*: `u32`



## object::elf::R_SPARC_GOTDATA_LOX10

*Constant*: `u32`



## object::elf::R_SPARC_GOTDATA_OP

*Constant*: `u32`



## object::elf::R_SPARC_GOTDATA_OP_HIX22

*Constant*: `u32`



## object::elf::R_SPARC_GOTDATA_OP_LOX10

*Constant*: `u32`



## object::elf::R_SPARC_H34

*Constant*: `u32`



## object::elf::R_SPARC_H44

*Constant*: `u32`

Direct high 12 of 44 bit



## object::elf::R_SPARC_HH22

*Constant*: `u32`

Top 22 bits of direct 64 bit



## object::elf::R_SPARC_HI22

*Constant*: `u32`

High 22 bit



## object::elf::R_SPARC_HIPLT22

*Constant*: `u32`

High 22 bit PLT entry



## object::elf::R_SPARC_HIX22

*Constant*: `u32`

High 22 bit complemented



## object::elf::R_SPARC_HM10

*Constant*: `u32`

High middle 10 bits of ...



## object::elf::R_SPARC_IRELATIVE

*Constant*: `u32`



## object::elf::R_SPARC_JMP_IREL

*Constant*: `u32`



## object::elf::R_SPARC_JMP_SLOT

*Constant*: `u32`

Create PLT entry



## object::elf::R_SPARC_L44

*Constant*: `u32`

Direct low 10 of 44 bit



## object::elf::R_SPARC_LM22

*Constant*: `u32`

Low middle 22 bits of ...



## object::elf::R_SPARC_LO10

*Constant*: `u32`

Truncated 10 bit



## object::elf::R_SPARC_LOPLT10

*Constant*: `u32`

Truncated 10 bit PLT entry



## object::elf::R_SPARC_LOX10

*Constant*: `u32`

Truncated 11 bit complemented



## object::elf::R_SPARC_M44

*Constant*: `u32`

Direct mid 22 of 44 bit



## object::elf::R_SPARC_NONE

*Constant*: `u32`

No reloc



## object::elf::R_SPARC_OLO10

*Constant*: `u32`

10bit with secondary 13bit addend



## object::elf::R_SPARC_PC10

*Constant*: `u32`

PC relative 10 bit truncated



## object::elf::R_SPARC_PC22

*Constant*: `u32`

PC relative 22 bit shifted



## object::elf::R_SPARC_PCPLT10

*Constant*: `u32`

PC rel trunc 10 bit PLT entry



## object::elf::R_SPARC_PCPLT22

*Constant*: `u32`

PC rel high 22 bit PLT entry



## object::elf::R_SPARC_PCPLT32

*Constant*: `u32`

PC rel 32 bit ref to PLT entry



## object::elf::R_SPARC_PC_HH22

*Constant*: `u32`

Top 22 bits of pc rel 64 bit



## object::elf::R_SPARC_PC_HM10

*Constant*: `u32`

High middle 10 bit of ...



## object::elf::R_SPARC_PC_LM22

*Constant*: `u32`

Low miggle 22 bits of ...



## object::elf::R_SPARC_PLT32

*Constant*: `u32`

Direct 32 bit ref to PLT entry



## object::elf::R_SPARC_PLT64

*Constant*: `u32`

Direct 64 bit ref to PLT entry



## object::elf::R_SPARC_REGISTER

*Constant*: `u32`

Global register usage



## object::elf::R_SPARC_RELATIVE

*Constant*: `u32`

Adjust by program base



## object::elf::R_SPARC_REV32

*Constant*: `u32`



## object::elf::R_SPARC_SIZE32

*Constant*: `u32`



## object::elf::R_SPARC_SIZE64

*Constant*: `u32`



## object::elf::R_SPARC_TLS_DTPMOD32

*Constant*: `u32`



## object::elf::R_SPARC_TLS_DTPMOD64

*Constant*: `u32`



## object::elf::R_SPARC_TLS_DTPOFF32

*Constant*: `u32`



## object::elf::R_SPARC_TLS_DTPOFF64

*Constant*: `u32`



## object::elf::R_SPARC_TLS_GD_ADD

*Constant*: `u32`



## object::elf::R_SPARC_TLS_GD_CALL

*Constant*: `u32`



## object::elf::R_SPARC_TLS_GD_HI22

*Constant*: `u32`



## object::elf::R_SPARC_TLS_GD_LO10

*Constant*: `u32`



## object::elf::R_SPARC_TLS_IE_ADD

*Constant*: `u32`



## object::elf::R_SPARC_TLS_IE_HI22

*Constant*: `u32`



## object::elf::R_SPARC_TLS_IE_LD

*Constant*: `u32`



## object::elf::R_SPARC_TLS_IE_LDX

*Constant*: `u32`



## object::elf::R_SPARC_TLS_IE_LO10

*Constant*: `u32`



## object::elf::R_SPARC_TLS_LDM_ADD

*Constant*: `u32`



## object::elf::R_SPARC_TLS_LDM_CALL

*Constant*: `u32`



## object::elf::R_SPARC_TLS_LDM_HI22

*Constant*: `u32`



## object::elf::R_SPARC_TLS_LDM_LO10

*Constant*: `u32`



## object::elf::R_SPARC_TLS_LDO_ADD

*Constant*: `u32`



## object::elf::R_SPARC_TLS_LDO_HIX22

*Constant*: `u32`



## object::elf::R_SPARC_TLS_LDO_LOX10

*Constant*: `u32`



## object::elf::R_SPARC_TLS_LE_HIX22

*Constant*: `u32`



## object::elf::R_SPARC_TLS_LE_LOX10

*Constant*: `u32`



## object::elf::R_SPARC_TLS_TPOFF32

*Constant*: `u32`



## object::elf::R_SPARC_TLS_TPOFF64

*Constant*: `u32`



## object::elf::R_SPARC_UA16

*Constant*: `u32`

Direct 16 bit unaligned



## object::elf::R_SPARC_UA32

*Constant*: `u32`

Direct 32 bit unaligned



## object::elf::R_SPARC_UA64

*Constant*: `u32`

Direct 64 bit unaligned



## object::elf::R_SPARC_WDISP10

*Constant*: `u32`



## object::elf::R_SPARC_WDISP16

*Constant*: `u32`

PC relative 16 bit shifted



## object::elf::R_SPARC_WDISP19

*Constant*: `u32`

PC relative 19 bit shifted



## object::elf::R_SPARC_WDISP22

*Constant*: `u32`

PC relative 22 bit shifted



## object::elf::R_SPARC_WDISP30

*Constant*: `u32`

PC relative 30 bit shifted



## object::elf::R_SPARC_WPLT30

*Constant*: `u32`

30 bit PC relative PLT address



## object::elf::R_TILEGX_16

*Constant*: `u32`

Direct 16 bit



## object::elf::R_TILEGX_16_PCREL

*Constant*: `u32`

PC relative 16 bit



## object::elf::R_TILEGX_32

*Constant*: `u32`

Direct 32 bit



## object::elf::R_TILEGX_32_PCREL

*Constant*: `u32`

PC relative 32 bit



## object::elf::R_TILEGX_64

*Constant*: `u32`

Direct 64 bit



## object::elf::R_TILEGX_64_PCREL

*Constant*: `u32`

PC relative 64 bit



## object::elf::R_TILEGX_8

*Constant*: `u32`

Direct 8 bit



## object::elf::R_TILEGX_8_PCREL

*Constant*: `u32`

PC relative 8 bit



## object::elf::R_TILEGX_BROFF_X1

*Constant*: `u32`

X1 pipe branch offset



## object::elf::R_TILEGX_COPY

*Constant*: `u32`

Copy relocation



## object::elf::R_TILEGX_DEST_IMM8_X1

*Constant*: `u32`

X1 pipe destination 8-bit



## object::elf::R_TILEGX_GLOB_DAT

*Constant*: `u32`

Create GOT entry



## object::elf::R_TILEGX_GNU_VTENTRY

*Constant*: `u32`

GNU C++ vtable member usage



## object::elf::R_TILEGX_GNU_VTINHERIT

*Constant*: `u32`

GNU C++ vtable hierarchy



## object::elf::R_TILEGX_HW0

*Constant*: `u32`

hword 0 16-bit



## object::elf::R_TILEGX_HW0_LAST

*Constant*: `u32`

last hword 0 16-bit



## object::elf::R_TILEGX_HW1

*Constant*: `u32`

hword 1 16-bit



## object::elf::R_TILEGX_HW1_LAST

*Constant*: `u32`

last hword 1 16-bit



## object::elf::R_TILEGX_HW2

*Constant*: `u32`

hword 2 16-bit



## object::elf::R_TILEGX_HW2_LAST

*Constant*: `u32`

last hword 2 16-bit



## object::elf::R_TILEGX_HW3

*Constant*: `u32`

hword 3 16-bit



## object::elf::R_TILEGX_IMM16_X0_HW0

*Constant*: `u32`

X0 pipe hword 0



## object::elf::R_TILEGX_IMM16_X0_HW0_GOT

*Constant*: `u32`

X0 pipe hword 0 GOT offset



## object::elf::R_TILEGX_IMM16_X0_HW0_LAST

*Constant*: `u32`

X0 pipe last hword 0



## object::elf::R_TILEGX_IMM16_X0_HW0_LAST_GOT

*Constant*: `u32`

X0 pipe last hword 0 GOT offset



## object::elf::R_TILEGX_IMM16_X0_HW0_LAST_PCREL

*Constant*: `u32`

X0 pipe PC-rel last hword 0



## object::elf::R_TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL

*Constant*: `u32`

X0 pipe PC-rel PLT last hword 0



## object::elf::R_TILEGX_IMM16_X0_HW0_LAST_TLS_GD

*Constant*: `u32`

X0 pipe last hword 0 GD off



## object::elf::R_TILEGX_IMM16_X0_HW0_LAST_TLS_IE

*Constant*: `u32`

X0 pipe last hword 0 IE off



## object::elf::R_TILEGX_IMM16_X0_HW0_LAST_TLS_LE

*Constant*: `u32`

X0 pipe last hword 0 LE off



## object::elf::R_TILEGX_IMM16_X0_HW0_PCREL

*Constant*: `u32`

X0 pipe PC relative hword 0



## object::elf::R_TILEGX_IMM16_X0_HW0_PLT_PCREL

*Constant*: `u32`

X0 pipe PC-rel PLT hword 0



## object::elf::R_TILEGX_IMM16_X0_HW0_TLS_GD

*Constant*: `u32`

X0 pipe hword 0 TLS GD offset



## object::elf::R_TILEGX_IMM16_X0_HW0_TLS_IE

*Constant*: `u32`

X0 pipe hword 0 TLS IE offset



## object::elf::R_TILEGX_IMM16_X0_HW0_TLS_LE

*Constant*: `u32`

X0 pipe hword 0 TLS LE offset



## object::elf::R_TILEGX_IMM16_X0_HW1

*Constant*: `u32`

X0 pipe hword 1



## object::elf::R_TILEGX_IMM16_X0_HW1_LAST

*Constant*: `u32`

X0 pipe last hword 1



## object::elf::R_TILEGX_IMM16_X0_HW1_LAST_GOT

*Constant*: `u32`

X0 pipe last hword 1 GOT offset



## object::elf::R_TILEGX_IMM16_X0_HW1_LAST_PCREL

*Constant*: `u32`

X0 pipe PC-rel last hword 1



## object::elf::R_TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL

*Constant*: `u32`

X0 pipe PC-rel PLT last hword 1



## object::elf::R_TILEGX_IMM16_X0_HW1_LAST_TLS_GD

*Constant*: `u32`

X0 pipe last hword 1 GD off



## object::elf::R_TILEGX_IMM16_X0_HW1_LAST_TLS_IE

*Constant*: `u32`

X0 pipe last hword 1 IE off



## object::elf::R_TILEGX_IMM16_X0_HW1_LAST_TLS_LE

*Constant*: `u32`

X0 pipe last hword 1 LE off



## object::elf::R_TILEGX_IMM16_X0_HW1_PCREL

*Constant*: `u32`

X0 pipe PC relative hword 1



## object::elf::R_TILEGX_IMM16_X0_HW1_PLT_PCREL

*Constant*: `u32`

X0 pipe PC-rel PLT hword 1



## object::elf::R_TILEGX_IMM16_X0_HW2

*Constant*: `u32`

X0 pipe hword 2



## object::elf::R_TILEGX_IMM16_X0_HW2_LAST

*Constant*: `u32`

X0 pipe last hword 2



## object::elf::R_TILEGX_IMM16_X0_HW2_LAST_PCREL

*Constant*: `u32`

X0 pipe PC-rel last hword 2



## object::elf::R_TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL

*Constant*: `u32`

X0 pipe PC-rel PLT last hword 2



## object::elf::R_TILEGX_IMM16_X0_HW2_PCREL

*Constant*: `u32`

X0 pipe PC relative hword 2



## object::elf::R_TILEGX_IMM16_X0_HW2_PLT_PCREL

*Constant*: `u32`

X0 pipe PC-rel PLT hword 2



## object::elf::R_TILEGX_IMM16_X0_HW3

*Constant*: `u32`

X0 pipe hword 3



## object::elf::R_TILEGX_IMM16_X0_HW3_PCREL

*Constant*: `u32`

X0 pipe PC relative hword 3



## object::elf::R_TILEGX_IMM16_X0_HW3_PLT_PCREL

*Constant*: `u32`

X0 pipe PC-rel PLT hword 3



## object::elf::R_TILEGX_IMM16_X1_HW0

*Constant*: `u32`

X1 pipe hword 0



## object::elf::R_TILEGX_IMM16_X1_HW0_GOT

*Constant*: `u32`

X1 pipe hword 0 GOT offset



## object::elf::R_TILEGX_IMM16_X1_HW0_LAST

*Constant*: `u32`

X1 pipe last hword 0



## object::elf::R_TILEGX_IMM16_X1_HW0_LAST_GOT

*Constant*: `u32`

X1 pipe last hword 0 GOT offset



## object::elf::R_TILEGX_IMM16_X1_HW0_LAST_PCREL

*Constant*: `u32`

X1 pipe PC-rel last hword 0



## object::elf::R_TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL

*Constant*: `u32`

X1 pipe PC-rel PLT last hword 0



## object::elf::R_TILEGX_IMM16_X1_HW0_LAST_TLS_GD

*Constant*: `u32`

X1 pipe last hword 0 GD off



## object::elf::R_TILEGX_IMM16_X1_HW0_LAST_TLS_IE

*Constant*: `u32`

X1 pipe last hword 0 IE off



## object::elf::R_TILEGX_IMM16_X1_HW0_LAST_TLS_LE

*Constant*: `u32`

X1 pipe last hword 0 LE off



## object::elf::R_TILEGX_IMM16_X1_HW0_PCREL

*Constant*: `u32`

X1 pipe PC relative hword 0



## object::elf::R_TILEGX_IMM16_X1_HW0_PLT_PCREL

*Constant*: `u32`

X1 pipe PC-rel PLT hword 0



## object::elf::R_TILEGX_IMM16_X1_HW0_TLS_GD

*Constant*: `u32`

X1 pipe hword 0 TLS GD offset



## object::elf::R_TILEGX_IMM16_X1_HW0_TLS_IE

*Constant*: `u32`

X1 pipe hword 0 TLS IE offset



## object::elf::R_TILEGX_IMM16_X1_HW0_TLS_LE

*Constant*: `u32`

X1 pipe hword 0 TLS LE offset



## object::elf::R_TILEGX_IMM16_X1_HW1

*Constant*: `u32`

X1 pipe hword 1



## object::elf::R_TILEGX_IMM16_X1_HW1_LAST

*Constant*: `u32`

X1 pipe last hword 1



## object::elf::R_TILEGX_IMM16_X1_HW1_LAST_GOT

*Constant*: `u32`

X1 pipe last hword 1 GOT offset



## object::elf::R_TILEGX_IMM16_X1_HW1_LAST_PCREL

*Constant*: `u32`

X1 pipe PC-rel last hword 1



## object::elf::R_TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL

*Constant*: `u32`

X1 pipe PC-rel PLT last hword 1



## object::elf::R_TILEGX_IMM16_X1_HW1_LAST_TLS_GD

*Constant*: `u32`

X1 pipe last hword 1 GD off



## object::elf::R_TILEGX_IMM16_X1_HW1_LAST_TLS_IE

*Constant*: `u32`

X1 pipe last hword 1 IE off



## object::elf::R_TILEGX_IMM16_X1_HW1_LAST_TLS_LE

*Constant*: `u32`

X1 pipe last hword 1 LE off



## object::elf::R_TILEGX_IMM16_X1_HW1_PCREL

*Constant*: `u32`

X1 pipe PC relative hword 1



## object::elf::R_TILEGX_IMM16_X1_HW1_PLT_PCREL

*Constant*: `u32`

X1 pipe PC-rel PLT hword 1



## object::elf::R_TILEGX_IMM16_X1_HW2

*Constant*: `u32`

X1 pipe hword 2



## object::elf::R_TILEGX_IMM16_X1_HW2_LAST

*Constant*: `u32`

X1 pipe last hword 2



## object::elf::R_TILEGX_IMM16_X1_HW2_LAST_PCREL

*Constant*: `u32`

X1 pipe PC-rel last hword 2



## object::elf::R_TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL

*Constant*: `u32`

X1 pipe PC-rel PLT last hword 2



## object::elf::R_TILEGX_IMM16_X1_HW2_PCREL

*Constant*: `u32`

X1 pipe PC relative hword 2



## object::elf::R_TILEGX_IMM16_X1_HW2_PLT_PCREL

*Constant*: `u32`

X1 pipe PC-rel PLT hword 2



## object::elf::R_TILEGX_IMM16_X1_HW3

*Constant*: `u32`

X1 pipe hword 3



## object::elf::R_TILEGX_IMM16_X1_HW3_PCREL

*Constant*: `u32`

X1 pipe PC relative hword 3



## object::elf::R_TILEGX_IMM16_X1_HW3_PLT_PCREL

*Constant*: `u32`

X1 pipe PC-rel PLT hword 3



## object::elf::R_TILEGX_IMM8_X0

*Constant*: `u32`

X0 pipe 8-bit



## object::elf::R_TILEGX_IMM8_X0_TLS_ADD

*Constant*: `u32`

X0 pipe "addi" for TLS GD/IE



## object::elf::R_TILEGX_IMM8_X0_TLS_GD_ADD

*Constant*: `u32`

X0 pipe "addi" for TLS GD



## object::elf::R_TILEGX_IMM8_X1

*Constant*: `u32`

X1 pipe 8-bit



## object::elf::R_TILEGX_IMM8_X1_TLS_ADD

*Constant*: `u32`

X1 pipe "addi" for TLS GD/IE



## object::elf::R_TILEGX_IMM8_X1_TLS_GD_ADD

*Constant*: `u32`

X1 pipe "addi" for TLS GD



## object::elf::R_TILEGX_IMM8_Y0

*Constant*: `u32`

Y0 pipe 8-bit



## object::elf::R_TILEGX_IMM8_Y0_TLS_ADD

*Constant*: `u32`

Y0 pipe "addi" for TLS GD/IE



## object::elf::R_TILEGX_IMM8_Y0_TLS_GD_ADD

*Constant*: `u32`

Y0 pipe "addi" for TLS GD



## object::elf::R_TILEGX_IMM8_Y1

*Constant*: `u32`

Y1 pipe 8-bit



## object::elf::R_TILEGX_IMM8_Y1_TLS_ADD

*Constant*: `u32`

Y1 pipe "addi" for TLS GD/IE



## object::elf::R_TILEGX_IMM8_Y1_TLS_GD_ADD

*Constant*: `u32`

Y1 pipe "addi" for TLS GD



## object::elf::R_TILEGX_JMP_SLOT

*Constant*: `u32`

Create PLT entry



## object::elf::R_TILEGX_JUMPOFF_X1

*Constant*: `u32`

X1 pipe jump offset



## object::elf::R_TILEGX_JUMPOFF_X1_PLT

*Constant*: `u32`

X1 pipe jump offset to PLT



## object::elf::R_TILEGX_MF_IMM14_X1

*Constant*: `u32`

X1 pipe mfspr



## object::elf::R_TILEGX_MMEND_X0

*Constant*: `u32`

X0 pipe mm "end"



## object::elf::R_TILEGX_MMSTART_X0

*Constant*: `u32`

X0 pipe mm "start"



## object::elf::R_TILEGX_MT_IMM14_X1

*Constant*: `u32`

X1 pipe mtspr



## object::elf::R_TILEGX_NONE

*Constant*: `u32`

No reloc



## object::elf::R_TILEGX_RELATIVE

*Constant*: `u32`

Adjust by program base



## object::elf::R_TILEGX_SHAMT_X0

*Constant*: `u32`

X0 pipe shift amount



## object::elf::R_TILEGX_SHAMT_X1

*Constant*: `u32`

X1 pipe shift amount



## object::elf::R_TILEGX_SHAMT_Y0

*Constant*: `u32`

Y0 pipe shift amount



## object::elf::R_TILEGX_SHAMT_Y1

*Constant*: `u32`

Y1 pipe shift amount



## object::elf::R_TILEGX_TLS_DTPMOD32

*Constant*: `u32`

32-bit ID of symbol's module



## object::elf::R_TILEGX_TLS_DTPMOD64

*Constant*: `u32`

64-bit ID of symbol's module



## object::elf::R_TILEGX_TLS_DTPOFF32

*Constant*: `u32`

32-bit offset in TLS block



## object::elf::R_TILEGX_TLS_DTPOFF64

*Constant*: `u32`

64-bit offset in TLS block



## object::elf::R_TILEGX_TLS_GD_CALL

*Constant*: `u32`

"jal" for TLS GD



## object::elf::R_TILEGX_TLS_IE_LOAD

*Constant*: `u32`

"ld_tls" for TLS IE



## object::elf::R_TILEGX_TLS_TPOFF32

*Constant*: `u32`

32-bit offset in static TLS block



## object::elf::R_TILEGX_TLS_TPOFF64

*Constant*: `u32`

64-bit offset in static TLS block



## object::elf::R_TILEPRO_16

*Constant*: `u32`

Direct 16 bit



## object::elf::R_TILEPRO_16_PCREL

*Constant*: `u32`

PC relative 16 bit



## object::elf::R_TILEPRO_32

*Constant*: `u32`

Direct 32 bit



## object::elf::R_TILEPRO_32_PCREL

*Constant*: `u32`

PC relative 32 bit



## object::elf::R_TILEPRO_8

*Constant*: `u32`

Direct 8 bit



## object::elf::R_TILEPRO_8_PCREL

*Constant*: `u32`

PC relative 8 bit



## object::elf::R_TILEPRO_BROFF_X1

*Constant*: `u32`

X1 pipe branch offset



## object::elf::R_TILEPRO_COPY

*Constant*: `u32`

Copy relocation



## object::elf::R_TILEPRO_DEST_IMM8_X1

*Constant*: `u32`

X1 pipe destination 8-bit



## object::elf::R_TILEPRO_GLOB_DAT

*Constant*: `u32`

Create GOT entry



## object::elf::R_TILEPRO_GNU_VTENTRY

*Constant*: `u32`

GNU C++ vtable member usage



## object::elf::R_TILEPRO_GNU_VTINHERIT

*Constant*: `u32`

GNU C++ vtable hierarchy



## object::elf::R_TILEPRO_HA16

*Constant*: `u32`

High 16 bit, adjusted



## object::elf::R_TILEPRO_HI16

*Constant*: `u32`

High 16 bit



## object::elf::R_TILEPRO_IMM16_X0

*Constant*: `u32`

X0 pipe 16-bit



## object::elf::R_TILEPRO_IMM16_X0_GOT

*Constant*: `u32`

X0 pipe 16-bit GOT offset



## object::elf::R_TILEPRO_IMM16_X0_GOT_HA

*Constant*: `u32`

X0 pipe ha() 16-bit GOT offset



## object::elf::R_TILEPRO_IMM16_X0_GOT_HI

*Constant*: `u32`

X0 pipe high 16-bit GOT offset



## object::elf::R_TILEPRO_IMM16_X0_GOT_LO

*Constant*: `u32`

X0 pipe low 16-bit GOT offset



## object::elf::R_TILEPRO_IMM16_X0_HA

*Constant*: `u32`

X0 pipe high 16-bit, adjusted



## object::elf::R_TILEPRO_IMM16_X0_HA_PCREL

*Constant*: `u32`

X0 pipe PC relative ha() 16 bit



## object::elf::R_TILEPRO_IMM16_X0_HI

*Constant*: `u32`

X0 pipe high 16-bit



## object::elf::R_TILEPRO_IMM16_X0_HI_PCREL

*Constant*: `u32`

X0 pipe PC relative high 16 bit



## object::elf::R_TILEPRO_IMM16_X0_LO

*Constant*: `u32`

X0 pipe low 16-bit



## object::elf::R_TILEPRO_IMM16_X0_LO_PCREL

*Constant*: `u32`

X0 pipe PC relative low 16 bit



## object::elf::R_TILEPRO_IMM16_X0_PCREL

*Constant*: `u32`

X0 pipe PC relative 16 bit



## object::elf::R_TILEPRO_IMM16_X0_TLS_GD

*Constant*: `u32`

X0 pipe 16-bit TLS GD offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_GD_HA

*Constant*: `u32`

X0 pipe ha() 16-bit TLS GD offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_GD_HI

*Constant*: `u32`

X0 pipe high 16-bit TLS GD offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_GD_LO

*Constant*: `u32`

X0 pipe low 16-bit TLS GD offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_IE

*Constant*: `u32`

X0 pipe 16-bit TLS IE offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_IE_HA

*Constant*: `u32`

X0 pipe ha() 16-bit TLS IE offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_IE_HI

*Constant*: `u32`

X0 pipe high 16-bit TLS IE offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_IE_LO

*Constant*: `u32`

X0 pipe low 16-bit TLS IE offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_LE

*Constant*: `u32`

X0 pipe 16-bit TLS LE offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_LE_HA

*Constant*: `u32`

X0 pipe ha() 16-bit TLS LE offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_LE_HI

*Constant*: `u32`

X0 pipe high 16-bit TLS LE offset



## object::elf::R_TILEPRO_IMM16_X0_TLS_LE_LO

*Constant*: `u32`

X0 pipe low 16-bit TLS LE offset



## object::elf::R_TILEPRO_IMM16_X1

*Constant*: `u32`

X1 pipe 16-bit



## object::elf::R_TILEPRO_IMM16_X1_GOT

*Constant*: `u32`

X1 pipe 16-bit GOT offset



## object::elf::R_TILEPRO_IMM16_X1_GOT_HA

*Constant*: `u32`

X1 pipe ha() 16-bit GOT offset



## object::elf::R_TILEPRO_IMM16_X1_GOT_HI

*Constant*: `u32`

X1 pipe high 16-bit GOT offset



## object::elf::R_TILEPRO_IMM16_X1_GOT_LO

*Constant*: `u32`

X1 pipe low 16-bit GOT offset



## object::elf::R_TILEPRO_IMM16_X1_HA

*Constant*: `u32`

X1 pipe high 16-bit, adjusted



## object::elf::R_TILEPRO_IMM16_X1_HA_PCREL

*Constant*: `u32`

X1 pipe PC relative ha() 16 bit



## object::elf::R_TILEPRO_IMM16_X1_HI

*Constant*: `u32`

X1 pipe high 16-bit



## object::elf::R_TILEPRO_IMM16_X1_HI_PCREL

*Constant*: `u32`

X1 pipe PC relative high 16 bit



## object::elf::R_TILEPRO_IMM16_X1_LO

*Constant*: `u32`

X1 pipe low 16-bit



## object::elf::R_TILEPRO_IMM16_X1_LO_PCREL

*Constant*: `u32`

X1 pipe PC relative low 16 bit



## object::elf::R_TILEPRO_IMM16_X1_PCREL

*Constant*: `u32`

X1 pipe PC relative 16 bit



## object::elf::R_TILEPRO_IMM16_X1_TLS_GD

*Constant*: `u32`

X1 pipe 16-bit TLS GD offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_GD_HA

*Constant*: `u32`

X1 pipe ha() 16-bit TLS GD offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_GD_HI

*Constant*: `u32`

X1 pipe high 16-bit TLS GD offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_GD_LO

*Constant*: `u32`

X1 pipe low 16-bit TLS GD offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_IE

*Constant*: `u32`

X1 pipe 16-bit TLS IE offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_IE_HA

*Constant*: `u32`

X1 pipe ha() 16-bit TLS IE offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_IE_HI

*Constant*: `u32`

X1 pipe high 16-bit TLS IE offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_IE_LO

*Constant*: `u32`

X1 pipe low 16-bit TLS IE offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_LE

*Constant*: `u32`

X1 pipe 16-bit TLS LE offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_LE_HA

*Constant*: `u32`

X1 pipe ha() 16-bit TLS LE offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_LE_HI

*Constant*: `u32`

X1 pipe high 16-bit TLS LE offset



## object::elf::R_TILEPRO_IMM16_X1_TLS_LE_LO

*Constant*: `u32`

X1 pipe low 16-bit TLS LE offset



## object::elf::R_TILEPRO_IMM8_X0

*Constant*: `u32`

X0 pipe 8-bit



## object::elf::R_TILEPRO_IMM8_X0_TLS_GD_ADD

*Constant*: `u32`

X0 pipe "addi" for TLS GD



## object::elf::R_TILEPRO_IMM8_X1

*Constant*: `u32`

X1 pipe 8-bit



## object::elf::R_TILEPRO_IMM8_X1_TLS_GD_ADD

*Constant*: `u32`

X1 pipe "addi" for TLS GD



## object::elf::R_TILEPRO_IMM8_Y0

*Constant*: `u32`

Y0 pipe 8-bit



## object::elf::R_TILEPRO_IMM8_Y0_TLS_GD_ADD

*Constant*: `u32`

Y0 pipe "addi" for TLS GD



## object::elf::R_TILEPRO_IMM8_Y1

*Constant*: `u32`

Y1 pipe 8-bit



## object::elf::R_TILEPRO_IMM8_Y1_TLS_GD_ADD

*Constant*: `u32`

Y1 pipe "addi" for TLS GD



## object::elf::R_TILEPRO_JMP_SLOT

*Constant*: `u32`

Create PLT entry



## object::elf::R_TILEPRO_JOFFLONG_X1

*Constant*: `u32`

X1 pipe jump offset



## object::elf::R_TILEPRO_JOFFLONG_X1_PLT

*Constant*: `u32`

X1 pipe jump offset to PLT



## object::elf::R_TILEPRO_LO16

*Constant*: `u32`

Low 16 bit



## object::elf::R_TILEPRO_MF_IMM15_X1

*Constant*: `u32`

X1 pipe mfspr



## object::elf::R_TILEPRO_MMEND_X0

*Constant*: `u32`

X0 pipe mm "end"



## object::elf::R_TILEPRO_MMEND_X1

*Constant*: `u32`

X1 pipe mm "end"



## object::elf::R_TILEPRO_MMSTART_X0

*Constant*: `u32`

X0 pipe mm "start"



## object::elf::R_TILEPRO_MMSTART_X1

*Constant*: `u32`

X1 pipe mm "start"



## object::elf::R_TILEPRO_MT_IMM15_X1

*Constant*: `u32`

X1 pipe mtspr



## object::elf::R_TILEPRO_NONE

*Constant*: `u32`

No reloc



## object::elf::R_TILEPRO_RELATIVE

*Constant*: `u32`

Adjust by program base



## object::elf::R_TILEPRO_SHAMT_X0

*Constant*: `u32`

X0 pipe shift amount



## object::elf::R_TILEPRO_SHAMT_X1

*Constant*: `u32`

X1 pipe shift amount



## object::elf::R_TILEPRO_SHAMT_Y0

*Constant*: `u32`

Y0 pipe shift amount



## object::elf::R_TILEPRO_SHAMT_Y1

*Constant*: `u32`

Y1 pipe shift amount



## object::elf::R_TILEPRO_TLS_DTPMOD32

*Constant*: `u32`

ID of module containing symbol



## object::elf::R_TILEPRO_TLS_DTPOFF32

*Constant*: `u32`

Offset in TLS block



## object::elf::R_TILEPRO_TLS_GD_CALL

*Constant*: `u32`

"jal" for TLS GD



## object::elf::R_TILEPRO_TLS_IE_LOAD

*Constant*: `u32`

"lw_tls" for TLS IE



## object::elf::R_TILEPRO_TLS_TPOFF32

*Constant*: `u32`

Offset in static TLS block



## object::elf::R_X86_64_16

*Constant*: `u32`

Direct 16 bit zero extended



## object::elf::R_X86_64_32

*Constant*: `u32`

Direct 32 bit zero extended



## object::elf::R_X86_64_32S

*Constant*: `u32`

Direct 32 bit sign extended



## object::elf::R_X86_64_64

*Constant*: `u32`

Direct 64 bit



## object::elf::R_X86_64_8

*Constant*: `u32`

Direct 8 bit sign extended



## object::elf::R_X86_64_COPY

*Constant*: `u32`

Copy symbol at runtime



## object::elf::R_X86_64_DTPMOD64

*Constant*: `u32`

ID of module containing symbol



## object::elf::R_X86_64_DTPOFF32

*Constant*: `u32`

Offset in TLS block



## object::elf::R_X86_64_DTPOFF64

*Constant*: `u32`

Offset in module's TLS block



## object::elf::R_X86_64_GLOB_DAT

*Constant*: `u32`

Create GOT entry



## object::elf::R_X86_64_GOT32

*Constant*: `u32`

32 bit GOT entry



## object::elf::R_X86_64_GOT64

*Constant*: `u32`

64-bit GOT entry offset



## object::elf::R_X86_64_GOTOFF64

*Constant*: `u32`

64 bit offset to GOT



## object::elf::R_X86_64_GOTPC32

*Constant*: `u32`

32 bit signed pc relative offset to GOT



## object::elf::R_X86_64_GOTPC32_TLSDESC

*Constant*: `u32`

GOT offset for TLS descriptor.



## object::elf::R_X86_64_GOTPC64

*Constant*: `u32`

64-bit PC relative offset to GOT



## object::elf::R_X86_64_GOTPCREL

*Constant*: `u32`

32 bit signed PC relative offset to GOT



## object::elf::R_X86_64_GOTPCREL64

*Constant*: `u32`

64-bit PC relative offset to GOT entry



## object::elf::R_X86_64_GOTPCRELX

*Constant*: `u32`

Load from 32 bit signed pc relative offset to GOT entry without REX prefix, relaxable.



## object::elf::R_X86_64_GOTPLT64

*Constant*: `u32`

like GOT64, says PLT entry needed



## object::elf::R_X86_64_GOTTPOFF

*Constant*: `u32`

32 bit signed PC relative offset to GOT entry for IE symbol



## object::elf::R_X86_64_IRELATIVE

*Constant*: `u32`

Adjust indirectly by program base



## object::elf::R_X86_64_JUMP_SLOT

*Constant*: `u32`

Create PLT entry



## object::elf::R_X86_64_NONE

*Constant*: `u32`

No reloc



## object::elf::R_X86_64_PC16

*Constant*: `u32`

16 bit sign extended pc relative



## object::elf::R_X86_64_PC32

*Constant*: `u32`

PC relative 32 bit signed



## object::elf::R_X86_64_PC64

*Constant*: `u32`

PC relative 64 bit



## object::elf::R_X86_64_PC8

*Constant*: `u32`

8 bit sign extended pc relative



## object::elf::R_X86_64_PLT32

*Constant*: `u32`

32 bit PLT address



## object::elf::R_X86_64_PLTOFF64

*Constant*: `u32`

64-bit GOT relative offset to PLT entry



## object::elf::R_X86_64_RELATIVE

*Constant*: `u32`

Adjust by program base



## object::elf::R_X86_64_RELATIVE64

*Constant*: `u32`

64-bit adjust by program base



## object::elf::R_X86_64_REX_GOTPCRELX

*Constant*: `u32`

Load from 32 bit signed pc relative offset to GOT entry with REX prefix, relaxable.



## object::elf::R_X86_64_SIZE32

*Constant*: `u32`

Size of symbol plus 32-bit addend



## object::elf::R_X86_64_SIZE64

*Constant*: `u32`

Size of symbol plus 64-bit addend



## object::elf::R_X86_64_TLSDESC

*Constant*: `u32`

TLS descriptor.



## object::elf::R_X86_64_TLSDESC_CALL

*Constant*: `u32`

Marker for call through TLS descriptor.



## object::elf::R_X86_64_TLSGD

*Constant*: `u32`

32 bit signed PC relative offset to two GOT entries for GD symbol



## object::elf::R_X86_64_TLSLD

*Constant*: `u32`

32 bit signed PC relative offset to two GOT entries for LD symbol



## object::elf::R_X86_64_TPOFF32

*Constant*: `u32`

Offset in initial TLS block



## object::elf::R_X86_64_TPOFF64

*Constant*: `u32`

Offset in initial TLS block



## object::elf::R_XTENSA_32

*Constant*: `u32`



## object::elf::R_XTENSA_32_PCREL

*Constant*: `u32`



## object::elf::R_XTENSA_ASM_EXPAND

*Constant*: `u32`



## object::elf::R_XTENSA_ASM_SIMPLIFY

*Constant*: `u32`



## object::elf::R_XTENSA_DIFF16

*Constant*: `u32`



## object::elf::R_XTENSA_DIFF32

*Constant*: `u32`



## object::elf::R_XTENSA_DIFF8

*Constant*: `u32`



## object::elf::R_XTENSA_GLOB_DAT

*Constant*: `u32`



## object::elf::R_XTENSA_GNU_VTENTRY

*Constant*: `u32`



## object::elf::R_XTENSA_GNU_VTINHERIT

*Constant*: `u32`



## object::elf::R_XTENSA_JMP_SLOT

*Constant*: `u32`



## object::elf::R_XTENSA_NDIFF16

*Constant*: `u32`



## object::elf::R_XTENSA_NDIFF32

*Constant*: `u32`



## object::elf::R_XTENSA_NDIFF8

*Constant*: `u32`



## object::elf::R_XTENSA_NONE

*Constant*: `u32`



## object::elf::R_XTENSA_OP0

*Constant*: `u32`



## object::elf::R_XTENSA_OP1

*Constant*: `u32`



## object::elf::R_XTENSA_OP2

*Constant*: `u32`



## object::elf::R_XTENSA_PDIFF16

*Constant*: `u32`



## object::elf::R_XTENSA_PDIFF32

*Constant*: `u32`



## object::elf::R_XTENSA_PDIFF8

*Constant*: `u32`



## object::elf::R_XTENSA_PLT

*Constant*: `u32`



## object::elf::R_XTENSA_RELATIVE

*Constant*: `u32`



## object::elf::R_XTENSA_RTLD

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT0_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT0_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT10_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT10_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT11_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT11_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT12_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT12_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT13_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT13_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT14_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT14_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT1_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT1_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT2_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT2_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT3_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT3_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT4_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT4_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT5_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT5_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT6_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT6_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT7_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT7_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT8_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT8_OP

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT9_ALT

*Constant*: `u32`



## object::elf::R_XTENSA_SLOT9_OP

*Constant*: `u32`



## object::elf::R_XTENSA_TLSDESC_ARG

*Constant*: `u32`



## object::elf::R_XTENSA_TLSDESC_FN

*Constant*: `u32`



## object::elf::R_XTENSA_TLS_ARG

*Constant*: `u32`



## object::elf::R_XTENSA_TLS_CALL

*Constant*: `u32`



## object::elf::R_XTENSA_TLS_DTPOFF

*Constant*: `u32`



## object::elf::R_XTENSA_TLS_FUNC

*Constant*: `u32`



## object::elf::R_XTENSA_TLS_TPOFF

*Constant*: `u32`



## object::elf::Rel32

*Struct*

Relocation table entry without explicit addend.

**Generic Parameters:**
- E

**Fields:**
- `r_offset: crate::endian::U32<E>` - Relocation address.
- `r_info: crate::endian::U32<E>` - Relocation type and symbol index.

**Methods:**

- `fn r_sym(self: &Self, endian: E) -> u32` - Get the `r_sym` component of the `r_info` field.
- `fn r_type(self: &Self, endian: E) -> u32` - Get the `r_type` component of the `r_info` field.
- `fn r_info(endian: E, r_sym: u32, r_type: u8) -> U32<E>` - Calculate the `r_info` field given the `r_sym` and `r_type` components.
- `fn set_r_info(self: & mut Self, endian: E, r_sym: u32, r_type: u8)` - Set the `r_info` field given the `r_sym` and `r_type` components.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Rel32<E>`
- **Rel**
  - `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn r_info(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn r_sym(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn r_type(self: &Self, endian: <Self as >::Endian) -> u32`



## object::elf::Rel64

*Struct*

Relocation table entry without explicit addend.

**Generic Parameters:**
- E

**Fields:**
- `r_offset: crate::endian::U64<E>` - Relocation address.
- `r_info: crate::endian::U64<E>` - Relocation type and symbol index.

**Methods:**

- `fn r_sym(self: &Self, endian: E) -> u32` - Get the `r_sym` component of the `r_info` field.
- `fn r_type(self: &Self, endian: E) -> u32` - Get the `r_type` component of the `r_info` field.
- `fn r_info(endian: E, r_sym: u32, r_type: u32) -> U64<E>` - Calculate the `r_info` field given the `r_sym` and `r_type` components.
- `fn set_r_info(self: & mut Self, endian: E, r_sym: u32, r_type: u32)` - Set the `r_info` field given the `r_sym` and `r_type` components.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Rel**
  - `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn r_info(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn r_sym(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn r_type(self: &Self, endian: <Self as >::Endian) -> u32`
- **Clone**
  - `fn clone(self: &Self) -> Rel64<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::Rela32

*Struct*

Relocation table entry with explicit addend.

**Generic Parameters:**
- E

**Fields:**
- `r_offset: crate::endian::U32<E>` - Relocation address.
- `r_info: crate::endian::U32<E>` - Relocation type and symbol index.
- `r_addend: crate::endian::I32<E>` - Explicit addend.

**Methods:**

- `fn r_sym(self: &Self, endian: E) -> u32` - Get the `r_sym` component of the `r_info` field.
- `fn r_type(self: &Self, endian: E) -> u32` - Get the `r_type` component of the `r_info` field.
- `fn r_info(endian: E, r_sym: u32, r_type: u8) -> U32<E>` - Calculate the `r_info` field given the `r_sym` and `r_type` components.
- `fn set_r_info(self: & mut Self, endian: E, r_sym: u32, r_type: u8)` - Set the `r_info` field given the `r_sym` and `r_type` components.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Rela**
  - `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn r_info(self: &Self, endian: <Self as >::Endian, _is_mips64el: bool) -> <Self as >::Word`
  - `fn r_addend(self: &Self, endian: <Self as >::Endian) -> <Self as >::Sword`
  - `fn r_sym(self: &Self, endian: <Self as >::Endian, _is_mips64el: bool) -> u32`
  - `fn r_type(self: &Self, endian: <Self as >::Endian, _is_mips64el: bool) -> u32`
- **Clone**
  - `fn clone(self: &Self) -> Rela32<E>`
- **From**
  - `fn from(rel: Rel32<E>) -> Self`



## object::elf::Rela64

*Struct*

Relocation table entry with explicit addend.

**Generic Parameters:**
- E

**Fields:**
- `r_offset: crate::endian::U64<E>` - Relocation address.
- `r_info: crate::endian::U64<E>` - Relocation type and symbol index.
- `r_addend: crate::endian::I64<E>` - Explicit addend.

**Methods:**

- `fn r_sym(self: &Self, endian: E, is_mips64el: bool) -> u32` - Get the `r_sym` component of the `r_info` field.
- `fn r_type(self: &Self, endian: E, is_mips64el: bool) -> u32` - Get the `r_type` component of the `r_info` field.
- `fn r_info(endian: E, is_mips64el: bool, r_sym: u32, r_type: u32) -> U64<E>` - Calculate the `r_info` field given the `r_sym` and `r_type` components.
- `fn set_r_info(self: & mut Self, endian: E, is_mips64el: bool, r_sym: u32, r_type: u32)` - Set the `r_info` field given the `r_sym` and `r_type` components.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Rela64<E>`
- **From**
  - `fn from(rel: Rel64<E>) -> Self`
- **Rela**
  - `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn r_info(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> <Self as >::Word`
  - `fn r_addend(self: &Self, endian: <Self as >::Endian) -> <Self as >::Sword`
  - `fn r_sym(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`
  - `fn r_type(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::Relr32

*Struct*

32-bit relative relocation table entry.

**Generic Parameters:**
- E

**Tuple Struct**: `(crate::endian::U32<E>)`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Relr32<E>`
- **Relr**
  - `fn get(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn next(offset: & mut <Self as >::Word, bits: & mut <Self as >::Word) -> Option<<Self as >::Word>`



## object::elf::Relr64

*Struct*

64-bit relative relocation table entry.

**Generic Parameters:**
- E

**Tuple Struct**: `(crate::endian::U64<E>)`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Relr64<E>`
- **Relr**
  - `fn get(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn next(offset: & mut <Self as >::Word, bits: & mut <Self as >::Word) -> Option<<Self as >::Word>`



## object::elf::SHF_ALLOC

*Constant*: `u32`

Section occupies memory during execution.



## object::elf::SHF_ALPHA_GPREL

*Constant*: `u32`



## object::elf::SHF_ARM_COMDEF

*Constant*: `u32`

Section may be multiply defined in the input to a link step.



## object::elf::SHF_ARM_ENTRYSECT

*Constant*: `u32`

Section contains an entry point



## object::elf::SHF_COMPRESSED

*Constant*: `u32`

Section is compressed.

Compressed sections begin with one of the `CompressionHeader*` headers.



## object::elf::SHF_EXCLUDE

*Constant*: `u32`

This section is excluded from the final executable or shared library.



## object::elf::SHF_EXECINSTR

*Constant*: `u32`

Section is executable.



## object::elf::SHF_GNU_MBIND

*Constant*: `u32`

Mbind section.



## object::elf::SHF_GNU_RETAIN

*Constant*: `u32`

Section should not be garbage collected by the linker.



## object::elf::SHF_GROUP

*Constant*: `u32`

Section is a member of a group.



## object::elf::SHF_IA_64_NORECOV

*Constant*: `u32`

spec insns w/o recovery



## object::elf::SHF_IA_64_SHORT

*Constant*: `u32`

section near gp



## object::elf::SHF_INFO_LINK

*Constant*: `u32`

The `sh_info` field contains a section header table index.



## object::elf::SHF_LINK_ORDER

*Constant*: `u32`

Section has special ordering requirements when combining sections.



## object::elf::SHF_MASKOS

*Constant*: `u32`

OS-specific section flags.



## object::elf::SHF_MASKPROC

*Constant*: `u32`

Processor-specific section flags.



## object::elf::SHF_MERGE

*Constant*: `u32`

Section may be be merged to eliminate duplication.



## object::elf::SHF_MIPS_ADDR

*Constant*: `u32`



## object::elf::SHF_MIPS_GPREL

*Constant*: `u32`

Must be in global data area.



## object::elf::SHF_MIPS_LOCAL

*Constant*: `u32`



## object::elf::SHF_MIPS_MERGE

*Constant*: `u32`



## object::elf::SHF_MIPS_NAMES

*Constant*: `u32`



## object::elf::SHF_MIPS_NODUPE

*Constant*: `u32`



## object::elf::SHF_MIPS_NOSTRIP

*Constant*: `u32`



## object::elf::SHF_MIPS_STRINGS

*Constant*: `u32`



## object::elf::SHF_OS_NONCONFORMING

*Constant*: `u32`

Section requires special OS-specific handling.



## object::elf::SHF_PARISC_HUGE

*Constant*: `u32`

Section far from gp.



## object::elf::SHF_PARISC_SBP

*Constant*: `u32`

Static branch prediction code.



## object::elf::SHF_PARISC_SHORT

*Constant*: `u32`

Section with short addressing.



## object::elf::SHF_STRINGS

*Constant*: `u32`

Section contains nul-terminated strings.



## object::elf::SHF_TLS

*Constant*: `u32`

Section holds thread-local storage.



## object::elf::SHF_WRITE

*Constant*: `u32`

Section is writable.



## object::elf::SHN_ABS

*Constant*: `u16`

Associated symbol is absolute.



## object::elf::SHN_COMMON

*Constant*: `u16`

Associated symbol is common.



## object::elf::SHN_HIOS

*Constant*: `u16`

End of OS-specific section indices.



## object::elf::SHN_HIPROC

*Constant*: `u16`

End of processor-specific section indices.



## object::elf::SHN_HIRESERVE

*Constant*: `u16`

End of reserved section indices.



## object::elf::SHN_LOOS

*Constant*: `u16`

Start of OS-specific section indices.



## object::elf::SHN_LOPROC

*Constant*: `u16`

Start of processor-specific section indices.



## object::elf::SHN_LORESERVE

*Constant*: `u16`

OS-specific range start.
Start of reserved section indices.



## object::elf::SHN_MIPS_ACOMMON

*Constant*: `u16`

Allocated common symbols.



## object::elf::SHN_MIPS_DATA

*Constant*: `u16`

Allocated data symbols.



## object::elf::SHN_MIPS_SCOMMON

*Constant*: `u16`

Small common symbols.



## object::elf::SHN_MIPS_SUNDEFINED

*Constant*: `u16`

Small undefined symbols.



## object::elf::SHN_MIPS_TEXT

*Constant*: `u16`

Allocated test symbols.



## object::elf::SHN_PARISC_ANSI_COMMON

*Constant*: `u16`

Section for tentatively declared symbols in ANSI C.



## object::elf::SHN_PARISC_HUGE_COMMON

*Constant*: `u16`

Common blocks in huge model.



## object::elf::SHN_UNDEF

*Constant*: `u16`

Undefined section.



## object::elf::SHN_XINDEX

*Constant*: `u16`

Section index is in the `SHT_SYMTAB_SHNDX` section.



## object::elf::SHT_AARCH64_ATTRIBUTES

*Constant*: `u32`

AArch64 attributes section.



## object::elf::SHT_ALPHA_DEBUG

*Constant*: `u32`



## object::elf::SHT_ALPHA_REGINFO

*Constant*: `u32`



## object::elf::SHT_ARM_ATTRIBUTES

*Constant*: `u32`

ARM attributes section.



## object::elf::SHT_ARM_EXIDX

*Constant*: `u32`

ARM unwind section.



## object::elf::SHT_ARM_PREEMPTMAP

*Constant*: `u32`

Preemption details.



## object::elf::SHT_CHECKSUM

*Constant*: `u32`

Checksum for DSO content.



## object::elf::SHT_CREL

*Constant*: `u32`

Experimental CREL relocations. LLVM will change the value and
break compatibility in the future.



## object::elf::SHT_CSKY_ATTRIBUTES

*Constant*: `u32`

C-SKY attributes section.



## object::elf::SHT_DYNAMIC

*Constant*: `u32`

Dynamic linking information.



## object::elf::SHT_DYNSYM

*Constant*: `u32`

Dynamic linker symbol table.



## object::elf::SHT_FINI_ARRAY

*Constant*: `u32`

Array of destructors.



## object::elf::SHT_GNU_ATTRIBUTES

*Constant*: `u32`

Object attributes.



## object::elf::SHT_GNU_HASH

*Constant*: `u32`

GNU-style hash table.



## object::elf::SHT_GNU_LIBLIST

*Constant*: `u32`

Prelink library list



## object::elf::SHT_GNU_SFRAME

*Constant*: `u32`

GNU SFrame stack trace format.



## object::elf::SHT_GNU_VERDEF

*Constant*: `u32`

Version definition section.



## object::elf::SHT_GNU_VERNEED

*Constant*: `u32`

Version needs section.



## object::elf::SHT_GNU_VERSYM

*Constant*: `u32`

Version symbol table.



## object::elf::SHT_GROUP

*Constant*: `u32`

Section group.



## object::elf::SHT_HASH

*Constant*: `u32`

Symbol hash table.



## object::elf::SHT_HIOS

*Constant*: `u32`

End of OS-specific section types.



## object::elf::SHT_HIPROC

*Constant*: `u32`

End of processor-specific section types.



## object::elf::SHT_HISUNW

*Constant*: `u32`

Sun-specific high bound.



## object::elf::SHT_HIUSER

*Constant*: `u32`

End of application-specific section types.



## object::elf::SHT_IA_64_EXT

*Constant*: `u32`

extension bits



## object::elf::SHT_IA_64_UNWIND

*Constant*: `u32`

unwind bits



## object::elf::SHT_INIT_ARRAY

*Constant*: `u32`

Array of constructors.



## object::elf::SHT_LLVM_DEPENDENT_LIBRARIES

*Constant*: `u32`

LLVM-style dependent libraries.



## object::elf::SHT_LOOS

*Constant*: `u32`

Start of OS-specific section types.



## object::elf::SHT_LOPROC

*Constant*: `u32`

Start of processor-specific section types.



## object::elf::SHT_LOSUNW

*Constant*: `u32`

Sun-specific low bound.



## object::elf::SHT_LOUSER

*Constant*: `u32`

Start of application-specific section types.



## object::elf::SHT_MIPS_AUXSYM

*Constant*: `u32`



## object::elf::SHT_MIPS_CONFLICT

*Constant*: `u32`

Conflicting symbols.



## object::elf::SHT_MIPS_CONTENT

*Constant*: `u32`



## object::elf::SHT_MIPS_DEBUG

*Constant*: `u32`

MIPS ECOFF debugging info.



## object::elf::SHT_MIPS_DELTACLASS

*Constant*: `u32`



## object::elf::SHT_MIPS_DELTADECL

*Constant*: `u32`



## object::elf::SHT_MIPS_DELTAINST

*Constant*: `u32`



## object::elf::SHT_MIPS_DELTASYM

*Constant*: `u32`



## object::elf::SHT_MIPS_DENSE

*Constant*: `u32`



## object::elf::SHT_MIPS_DWARF

*Constant*: `u32`

DWARF debugging information.



## object::elf::SHT_MIPS_EH_REGION

*Constant*: `u32`



## object::elf::SHT_MIPS_EVENTS

*Constant*: `u32`

Event section.



## object::elf::SHT_MIPS_EXTSYM

*Constant*: `u32`



## object::elf::SHT_MIPS_FDESC

*Constant*: `u32`



## object::elf::SHT_MIPS_GPTAB

*Constant*: `u32`

Global data area sizes.



## object::elf::SHT_MIPS_IFACE

*Constant*: `u32`



## object::elf::SHT_MIPS_LIBLIST

*Constant*: `u32`

Shared objects used in link.



## object::elf::SHT_MIPS_LINE

*Constant*: `u32`



## object::elf::SHT_MIPS_LOCSTR

*Constant*: `u32`



## object::elf::SHT_MIPS_LOCSYM

*Constant*: `u32`



## object::elf::SHT_MIPS_MSYM

*Constant*: `u32`



## object::elf::SHT_MIPS_OPTIONS

*Constant*: `u32`

Miscellaneous options.



## object::elf::SHT_MIPS_OPTSYM

*Constant*: `u32`



## object::elf::SHT_MIPS_PACKAGE

*Constant*: `u32`



## object::elf::SHT_MIPS_PACKSYM

*Constant*: `u32`



## object::elf::SHT_MIPS_PDESC

*Constant*: `u32`



## object::elf::SHT_MIPS_PDR_EXCEPTION

*Constant*: `u32`



## object::elf::SHT_MIPS_PIXIE

*Constant*: `u32`



## object::elf::SHT_MIPS_REGINFO

*Constant*: `u32`

Register usage information.



## object::elf::SHT_MIPS_RELD

*Constant*: `u32`



## object::elf::SHT_MIPS_RFDESC

*Constant*: `u32`



## object::elf::SHT_MIPS_SHDR

*Constant*: `u32`



## object::elf::SHT_MIPS_SYMBOL_LIB

*Constant*: `u32`



## object::elf::SHT_MIPS_TRANSLATE

*Constant*: `u32`



## object::elf::SHT_MIPS_UCODE

*Constant*: `u32`

Reserved for SGI/MIPS compilers



## object::elf::SHT_MIPS_WHIRL

*Constant*: `u32`



## object::elf::SHT_MIPS_XLATE

*Constant*: `u32`



## object::elf::SHT_MIPS_XLATE_DEBUG

*Constant*: `u32`



## object::elf::SHT_MIPS_XLATE_OLD

*Constant*: `u32`



## object::elf::SHT_NOBITS

*Constant*: `u32`

Program space with no data (bss).



## object::elf::SHT_NOTE

*Constant*: `u32`

Notes.



## object::elf::SHT_NULL

*Constant*: `u32`

Section header table entry is unused.



## object::elf::SHT_PARISC_DOC

*Constant*: `u32`

Debug info for optimized code.



## object::elf::SHT_PARISC_EXT

*Constant*: `u32`

Contains product specific ext.



## object::elf::SHT_PARISC_UNWIND

*Constant*: `u32`

Unwind information.



## object::elf::SHT_PREINIT_ARRAY

*Constant*: `u32`

Array of pre-constructors.



## object::elf::SHT_PROGBITS

*Constant*: `u32`

Program data.



## object::elf::SHT_REL

*Constant*: `u32`

Relocation entries without explicit addends.



## object::elf::SHT_RELA

*Constant*: `u32`

Relocation entries with explicit addends.



## object::elf::SHT_RELR

*Constant*: `u32`

Relocation entries; only offsets.



## object::elf::SHT_RISCV_ATTRIBUTES

*Constant*: `u32`

RISC-V attributes section.



## object::elf::SHT_SHARC_ADI_ATTRIBUTES

*Constant*: `u32`

.adi.attributes



## object::elf::SHT_SHLIB

*Constant*: `u32`

Reserved section type.



## object::elf::SHT_STRTAB

*Constant*: `u32`

String table.



## object::elf::SHT_SUNW_COMDAT

*Constant*: `u32`



## object::elf::SHT_SUNW_move

*Constant*: `u32`



## object::elf::SHT_SUNW_syminfo

*Constant*: `u32`



## object::elf::SHT_SYMTAB

*Constant*: `u32`

Symbol table.



## object::elf::SHT_SYMTAB_SHNDX

*Constant*: `u32`

Extended section indices for a symbol table.



## object::elf::SHT_X86_64_UNWIND

*Constant*: `u32`

Unwind information.



## object::elf::STB_GLOBAL

*Constant*: `u8`

Global symbol.



## object::elf::STB_GNU_UNIQUE

*Constant*: `u8`

Unique symbol.



## object::elf::STB_HIOS

*Constant*: `u8`

End of OS-specific symbol binding.



## object::elf::STB_HIPROC

*Constant*: `u8`

End of processor-specific symbol binding.



## object::elf::STB_LOCAL

*Constant*: `u8`

Local symbol.



## object::elf::STB_LOOS

*Constant*: `u8`

Start of OS-specific symbol binding.



## object::elf::STB_LOPROC

*Constant*: `u8`

Start of processor-specific symbol binding.



## object::elf::STB_MIPS_SPLIT_COMMON

*Constant*: `u8`



## object::elf::STB_WEAK

*Constant*: `u8`

Weak symbol.



## object::elf::STO_AARCH64_VARIANT_PCS

*Constant*: `u8`



## object::elf::STO_ALPHA_NOPV

*Constant*: `u8`

No PV required.



## object::elf::STO_ALPHA_STD_GPLOAD

*Constant*: `u8`

PV only used for initial ldgp.



## object::elf::STO_MIPS_PLT

*Constant*: `u8`



## object::elf::STO_MIPS_SC_ALIGN_UNUSED

*Constant*: `u8`

Only valid for `STB_MIPS_SPLIT_COMMON`.



## object::elf::STO_PPC64_LOCAL_BIT

*Constant*: `u8`



## object::elf::STO_PPC64_LOCAL_MASK

*Constant*: `u8`



## object::elf::STO_RISCV_VARIANT_CC

*Constant*: `u8`

Function uses variant calling convention.



## object::elf::STT_ARM_16BIT

*Constant*: `u8`

A Thumb label.



## object::elf::STT_ARM_TFUNC

*Constant*: `u8`

A Thumb function.



## object::elf::STT_COMMON

*Constant*: `u8`

Symbol is a common data object.



## object::elf::STT_FILE

*Constant*: `u8`

Symbol's name is a file name.



## object::elf::STT_FUNC

*Constant*: `u8`

Symbol is a code object.



## object::elf::STT_GNU_IFUNC

*Constant*: `u8`

Symbol is an indirect code object.



## object::elf::STT_HIOS

*Constant*: `u8`

End of OS-specific symbol types.



## object::elf::STT_HIPROC

*Constant*: `u8`

End of processor-specific symbol types.



## object::elf::STT_HP_OPAQUE

*Constant*: `u8`



## object::elf::STT_HP_STUB

*Constant*: `u8`



## object::elf::STT_LOOS

*Constant*: `u8`

Start of OS-specific symbol types.



## object::elf::STT_LOPROC

*Constant*: `u8`

Start of processor-specific symbol types.



## object::elf::STT_NOTYPE

*Constant*: `u8`

Symbol type is unspecified.



## object::elf::STT_OBJECT

*Constant*: `u8`

Symbol is a data object.



## object::elf::STT_PARISC_MILLICODE

*Constant*: `u8`

Millicode function entry point.



## object::elf::STT_SECTION

*Constant*: `u8`

Symbol is associated with a section.



## object::elf::STT_SPARC_REGISTER

*Constant*: `u8`

Global register reserved to app.



## object::elf::STT_TLS

*Constant*: `u8`

Symbol is a thread-local storage object.



## object::elf::STV_DEFAULT

*Constant*: `u8`

Default symbol visibility rules.



## object::elf::STV_HIDDEN

*Constant*: `u8`

Symbol is not visible to other components.



## object::elf::STV_INTERNAL

*Constant*: `u8`

Processor specific hidden class.



## object::elf::STV_PROTECTED

*Constant*: `u8`

Symbol is visible to other components, but is not preemptible.



## object::elf::SYMINFO_BT_LOWRESERVE

*Constant*: `u16`

Beginning of reserved entries



## object::elf::SYMINFO_BT_PARENT

*Constant*: `u16`

Symbol bound to parent



## object::elf::SYMINFO_BT_SELF

*Constant*: `u16`

Symbol bound to self



## object::elf::SYMINFO_CURRENT

*Constant*: `u16`



## object::elf::SYMINFO_FLG_COPY

*Constant*: `u16`

Symbol is a copy-reloc



## object::elf::SYMINFO_FLG_DIRECT

*Constant*: `u16`

Direct bound symbol



## object::elf::SYMINFO_FLG_LAZYLOAD

*Constant*: `u16`

Symbol bound to object to be lazy loaded



## object::elf::SYMINFO_FLG_PASSTHRU

*Constant*: `u16`

Pass-thru symbol for translator



## object::elf::SYMINFO_NONE

*Constant*: `u16`



## object::elf::SYMINFO_NUM

*Constant*: `u16`



## object::elf::SectionHeader32

*Struct*

Section header.

**Generic Parameters:**
- E

**Fields:**
- `sh_name: crate::endian::U32<E>` - Section name.
- `sh_type: crate::endian::U32<E>` - Section type. One of the `SHT_*` constants.
- `sh_flags: crate::endian::U32<E>` - Section flags. A combination of the `SHF_*` constants.
- `sh_addr: crate::endian::U32<E>` - Section virtual address at execution.
- `sh_offset: crate::endian::U32<E>` - Section file offset.
- `sh_size: crate::endian::U32<E>` - Section size in bytes.
- `sh_link: crate::endian::U32<E>` - Link to another section.
- `sh_info: crate::endian::U32<E>` - Additional section information.
- `sh_addralign: crate::endian::U32<E>` - Section alignment.
- `sh_entsize: crate::endian::U32<E>` - Entry size if the section holds a table.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SectionHeader32<E>`
- **SectionHeader**
  - `fn sh_name(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn sh_type(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn sh_flags(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn sh_addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn sh_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn sh_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn sh_link(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn sh_info(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn sh_addralign(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn sh_entsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::SectionHeader64

*Struct*

Section header.

**Generic Parameters:**
- E

**Fields:**
- `sh_name: crate::endian::U32<E>` - Section name.
- `sh_type: crate::endian::U32<E>` - Section type. One of the `SHT_*` constants.
- `sh_flags: crate::endian::U64<E>` - Section flags. A combination of the `SHF_*` constants.
- `sh_addr: crate::endian::U64<E>` - Section virtual address at execution.
- `sh_offset: crate::endian::U64<E>` - Section file offset.
- `sh_size: crate::endian::U64<E>` - Section size in bytes.
- `sh_link: crate::endian::U32<E>` - Link to another section.
- `sh_info: crate::endian::U32<E>` - Additional section information.
- `sh_addralign: crate::endian::U64<E>` - Section alignment.
- `sh_entsize: crate::endian::U64<E>` - Entry size if the section holds a table.

**Traits:** Copy, Pod

**Trait Implementations:**

- **SectionHeader**
  - `fn sh_name(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn sh_type(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn sh_flags(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn sh_addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn sh_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn sh_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn sh_link(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn sh_info(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn sh_addralign(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn sh_entsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SectionHeader64<E>`



## object::elf::Sym32

*Struct*

Symbol table entry.

**Generic Parameters:**
- E

**Fields:**
- `st_name: crate::endian::U32<E>` - Symbol name.
- `st_value: crate::endian::U32<E>` - Symbol value.
- `st_size: crate::endian::U32<E>` - Symbol size.
- `st_info: u8` - Symbol type and binding.
- `st_other: u8` - Symbol visibility.
- `st_shndx: crate::endian::U16<E>` - Section index or one of the `SHN_*` values.

**Methods:**

- `fn st_bind(self: &Self) -> u8` - Get the `st_bind` component of the `st_info` field.
- `fn st_type(self: &Self) -> u8` - Get the `st_type` component of the `st_info` field.
- `fn set_st_info(self: & mut Self, st_bind: u8, st_type: u8)` - Set the `st_info` field given the `st_bind` and `st_type` components.
- `fn st_visibility(self: &Self) -> u8` - Get the `st_visibility` component of the `st_info` field.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Sym**
  - `fn st_name(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn st_info(self: &Self) -> u8`
  - `fn st_bind(self: &Self) -> u8`
  - `fn st_type(self: &Self) -> u8`
  - `fn st_other(self: &Self) -> u8`
  - `fn st_visibility(self: &Self) -> u8`
  - `fn st_shndx(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn st_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn st_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
- **Clone**
  - `fn clone(self: &Self) -> Sym32<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Sym32<E>`



## object::elf::Sym64

*Struct*

Symbol table entry.

**Generic Parameters:**
- E

**Fields:**
- `st_name: crate::endian::U32<E>` - Symbol name.
- `st_info: u8` - Symbol type and binding.
- `st_other: u8` - Symbol visibility.
- `st_shndx: crate::endian::U16<E>` - Section index or one of the `SHN_*` values.
- `st_value: crate::endian::U64<E>` - Symbol value.
- `st_size: crate::endian::U64<E>` - Symbol size.

**Methods:**

- `fn st_bind(self: &Self) -> u8` - Get the `st_bind` component of the `st_info` field.
- `fn st_type(self: &Self) -> u8` - Get the `st_type` component of the `st_info` field.
- `fn set_st_info(self: & mut Self, st_bind: u8, st_type: u8)` - Set the `st_info` field given the `st_bind` and `st_type` components.
- `fn st_visibility(self: &Self) -> u8` - Get the `st_visibility` component of the `st_info` field.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Sym**
  - `fn st_name(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn st_info(self: &Self) -> u8`
  - `fn st_bind(self: &Self) -> u8`
  - `fn st_type(self: &Self) -> u8`
  - `fn st_other(self: &Self) -> u8`
  - `fn st_visibility(self: &Self) -> u8`
  - `fn st_shndx(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn st_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn st_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
- **Clone**
  - `fn clone(self: &Self) -> Sym64<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Sym64<E>`



## object::elf::Syminfo32

*Struct*

Additional information about a `Sym32`.

**Generic Parameters:**
- E

**Fields:**
- `si_boundto: crate::endian::U16<E>` - Direct bindings, symbol bound to.
- `si_flags: crate::endian::U16<E>` - Per symbol flags.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Syminfo32<E>`



## object::elf::Syminfo64

*Struct*

Additional information about a `Sym64`.

**Generic Parameters:**
- E

**Fields:**
- `si_boundto: crate::endian::U16<E>` - Direct bindings, symbol bound to.
- `si_flags: crate::endian::U16<E>` - Per symbol flags.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Syminfo64<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::Tag_File

*Constant*: `u8`



## object::elf::Tag_Section

*Constant*: `u8`



## object::elf::Tag_Symbol

*Constant*: `u8`



## object::elf::VERSYM_HIDDEN

*Constant*: `u16`

Symbol is hidden.



## object::elf::VERSYM_VERSION

*Constant*: `u16`

Symbol version index.



## object::elf::VER_DEF_CURRENT

*Constant*: `u16`

Current version



## object::elf::VER_DEF_NONE

*Constant*: `u16`

No version



## object::elf::VER_FLG_BASE

*Constant*: `u16`

Version definition of file itself



## object::elf::VER_FLG_WEAK

*Constant*: `u16`

Weak version identifier



## object::elf::VER_NDX_GLOBAL

*Constant*: `u16`

Symbol is global.



## object::elf::VER_NDX_LOCAL

*Constant*: `u16`

Symbol is local.



## object::elf::VER_NEED_CURRENT

*Constant*: `u16`

Current version



## object::elf::VER_NEED_NONE

*Constant*: `u16`

No version



## object::elf::Verdaux

*Struct*

Auxiliary version information.

**Generic Parameters:**
- E

**Fields:**
- `vda_name: crate::endian::U32<E>` - Version or dependency names
- `vda_next: crate::endian::U32<E>` - Offset in bytes to next verdaux

**Methods:**

- `fn name<'data, R>(self: &Self, endian: Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>` - Parse the version name from the string table.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Verdaux<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::Verdef

*Struct*

Version definition sections

**Generic Parameters:**
- E

**Fields:**
- `vd_version: crate::endian::U16<E>` - Version revision
- `vd_flags: crate::endian::U16<E>` - Version information
- `vd_ndx: crate::endian::U16<E>` - Version Index
- `vd_cnt: crate::endian::U16<E>` - Number of associated aux entries
- `vd_hash: crate::endian::U32<E>` - Version name hash value
- `vd_aux: crate::endian::U32<E>` - Offset in bytes to verdaux array
- `vd_next: crate::endian::U32<E>` - Offset in bytes to next verdef entry

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Verdef<E>`



## object::elf::Vernaux

*Struct*

Auxiliary needed version information.

**Generic Parameters:**
- E

**Fields:**
- `vna_hash: crate::endian::U32<E>` - Hash value of dependency name
- `vna_flags: crate::endian::U16<E>` - Dependency specific information
- `vna_other: crate::endian::U16<E>` - Version Index
- `vna_name: crate::endian::U32<E>` - Dependency name string offset
- `vna_next: crate::endian::U32<E>` - Offset in bytes to next vernaux entry

**Methods:**

- `fn name<'data, R>(self: &Self, endian: Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>` - Parse the version name from the string table.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Vernaux<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::Verneed

*Struct*

Version dependency.

**Generic Parameters:**
- E

**Fields:**
- `vn_version: crate::endian::U16<E>` - Version of structure
- `vn_cnt: crate::endian::U16<E>` - Number of associated aux entries
- `vn_file: crate::endian::U32<E>` - Offset of filename for this dependency
- `vn_aux: crate::endian::U32<E>` - Offset in bytes to vernaux array
- `vn_next: crate::endian::U32<E>` - Offset in bytes to next verneed entry

**Methods:**

- `fn file<'data, R>(self: &Self, endian: Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>` - Parse the file from the string table.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Verneed<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::elf::Versym

*Struct*

Version symbol information

**Generic Parameters:**
- E

**Tuple Struct**: `(crate::endian::U16<E>)`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Versym<E>`



## object::elf::ef_e2k_flag_to_mach

*Function*

Decode `E_E2K_MACH_*` from `FileHeader*::e_flags`.

```rust
fn ef_e2k_flag_to_mach(e_flags: u32) -> u32
```



## object::elf::ef_e2k_mach_to_flag

*Function*

Encode `E_E2K_MACH_*` into `FileHeader*::e_flags`.

```rust
fn ef_e2k_mach_to_flag(e_flags: u32, x: u32) -> u32
```



## object::elf::gnu_hash

*Function*

Calculate the GNU hash for a symbol name.

Used for `SHT_GNU_HASH`.

```rust
fn gnu_hash(name: &[u8]) -> u32
```



## object::elf::hash

*Function*

Calculate the SysV hash for a symbol name.

Used for `SHT_HASH`.

```rust
fn hash(name: &[u8]) -> u32
```



