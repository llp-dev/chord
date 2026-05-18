**object > macho**

# Module: macho

## Contents

**Structs**

- [`BuildToolVersion`](#buildtoolversion)
- [`BuildVersionCommand`](#buildversioncommand)
- [`DataInCodeEntry`](#dataincodeentry)
- [`DyldCacheHeader`](#dyldcacheheader) - The dyld cache header.
- [`DyldCacheImageInfo`](#dyldcacheimageinfo) - Corresponds to struct dyld_cache_image_info from dyld_cache_format.h.
- [`DyldCacheMappingAndSlideInfo`](#dyldcachemappingandslideinfo) - Corresponds to struct dyld_cache_mapping_and_slide_info from dyld_cache_format.h.
- [`DyldCacheMappingInfo`](#dyldcachemappinginfo) - Corresponds to struct dyld_cache_mapping_info from dyld_cache_format.h.
- [`DyldCacheSlideInfo2`](#dyldcacheslideinfo2) - Corresponds to struct dyld_cache_slide_info2 from dyld_cache_format.h.
- [`DyldCacheSlideInfo3`](#dyldcacheslideinfo3) - Corresponds to struct dyld_cache_slide_info3 from dyld_cache_format.h.
- [`DyldCacheSlideInfo5`](#dyldcacheslideinfo5) - Corresponds to struct dyld_cache_slide_info5 from dyld_cache_format.h.
- [`DyldCacheSlidePointer3`](#dyldcacheslidepointer3) - Corresponds to union dyld_cache_slide_pointer3 from dyld_cache_format.h.
- [`DyldCacheSlidePointer5`](#dyldcacheslidepointer5) - Corresponds to struct dyld_cache_slide_pointer5 from dyld_cache_format.h.
- [`DyldInfoCommand`](#dyldinfocommand)
- [`DyldSubCacheEntryV1`](#dyldsubcacheentryv1) - Added in dyld-940, which shipped with macOS 12 / iOS 15.
- [`DyldSubCacheEntryV2`](#dyldsubcacheentryv2) - Added in dyld-1042.1, which shipped with macOS 13 / iOS 16.
- [`Dylib`](#dylib)
- [`DylibCommand`](#dylibcommand)
- [`DylibModule32`](#dylibmodule32)
- [`DylibModule64`](#dylibmodule64)
- [`DylibReference`](#dylibreference)
- [`DylibTableOfContents`](#dylibtableofcontents)
- [`DylinkerCommand`](#dylinkercommand)
- [`DysymtabCommand`](#dysymtabcommand)
- [`EncryptionInfoCommand32`](#encryptioninfocommand32)
- [`EncryptionInfoCommand64`](#encryptioninfocommand64)
- [`EntryPointCommand`](#entrypointcommand)
- [`FatArch32`](#fatarch32)
- [`FatArch64`](#fatarch64)
- [`FatHeader`](#fatheader)
- [`FilesetEntryCommand`](#filesetentrycommand)
- [`FvmfileCommand`](#fvmfilecommand)
- [`Fvmlib`](#fvmlib)
- [`FvmlibCommand`](#fvmlibcommand)
- [`IdentCommand`](#identcommand)
- [`LcStr`](#lcstr) - A variable length string in a load command.
- [`LinkeditDataCommand`](#linkeditdatacommand)
- [`LinkerOptionCommand`](#linkeroptioncommand)
- [`LoadCommand`](#loadcommand) - Common fields at the start of every load command.
- [`MachHeader32`](#machheader32) - The 32-bit mach header.
- [`MachHeader64`](#machheader64) - The 64-bit mach header.
- [`Nlist32`](#nlist32)
- [`Nlist64`](#nlist64)
- [`NoteCommand`](#notecommand)
- [`PrebindCksumCommand`](#prebindcksumcommand)
- [`PreboundDylibCommand`](#prebounddylibcommand)
- [`Relocation`](#relocation) - A relocation entry.
- [`RelocationInfo`](#relocationinfo)
- [`RoutinesCommand32`](#routinescommand32)
- [`RoutinesCommand64`](#routinescommand64)
- [`RpathCommand`](#rpathcommand)
- [`ScatteredRelocationInfo`](#scatteredrelocationinfo)
- [`Section32`](#section32) - 32-bit section.
- [`Section64`](#section64) - 64-bit section.
- [`SegmentCommand32`](#segmentcommand32) - 32-bit segment load command.
- [`SegmentCommand64`](#segmentcommand64) - 64-bit segment load command.
- [`SourceVersionCommand`](#sourceversioncommand)
- [`SubClientCommand`](#subclientcommand)
- [`SubFrameworkCommand`](#subframeworkcommand)
- [`SubLibraryCommand`](#sublibrarycommand)
- [`SubUmbrellaCommand`](#subumbrellacommand)
- [`SymsegCommand`](#symsegcommand)
- [`SymtabCommand`](#symtabcommand)
- [`ThreadCommand`](#threadcommand)
- [`TwolevelHint`](#twolevelhint)
- [`TwolevelHintsCommand`](#twolevelhintscommand)
- [`UuidCommand`](#uuidcommand)
- [`VersionMinCommand`](#versionmincommand)

**Enums**

- [`PtrauthKey`](#ptrauthkey) - The key used to sign a pointer for authentication.

**Functions**

- [`cpu_subtype_intel`](#cpu_subtype_intel)
- [`cpu_subtype_intel_family`](#cpu_subtype_intel_family)
- [`cpu_subtype_intel_model`](#cpu_subtype_intel_model)

**Constants**

- [`ARM64_RELOC_ADDEND`](#arm64_reloc_addend) - must be followed by PAGE21 or PAGEOFF12
- [`ARM64_RELOC_AUTHENTICATED_POINTER`](#arm64_reloc_authenticated_pointer)
- [`ARM64_RELOC_BRANCH26`](#arm64_reloc_branch26) - a B/BL instruction with 26-bit displacement
- [`ARM64_RELOC_GOT_LOAD_PAGE21`](#arm64_reloc_got_load_page21) - pc-rel distance to page of GOT slot
- [`ARM64_RELOC_GOT_LOAD_PAGEOFF12`](#arm64_reloc_got_load_pageoff12) - offset within page of GOT slot, scaled by r_length
- [`ARM64_RELOC_PAGE21`](#arm64_reloc_page21) - pc-rel distance to page of target
- [`ARM64_RELOC_PAGEOFF12`](#arm64_reloc_pageoff12) - offset within page, scaled by r_length
- [`ARM64_RELOC_POINTER_TO_GOT`](#arm64_reloc_pointer_to_got) - for pointers to GOT slots
- [`ARM64_RELOC_SUBTRACTOR`](#arm64_reloc_subtractor) - must be followed by a ARM64_RELOC_UNSIGNED
- [`ARM64_RELOC_TLVP_LOAD_PAGE21`](#arm64_reloc_tlvp_load_page21) - pc-rel distance to page of TLVP slot
- [`ARM64_RELOC_TLVP_LOAD_PAGEOFF12`](#arm64_reloc_tlvp_load_pageoff12) - offset within page of TLVP slot, scaled by r_length
- [`ARM64_RELOC_UNSIGNED`](#arm64_reloc_unsigned) - for pointers
- [`ARM_RELOC_BR24`](#arm_reloc_br24) - 24 bit branch displacement (to a word address)
- [`ARM_RELOC_HALF`](#arm_reloc_half)
- [`ARM_RELOC_HALF_SECTDIFF`](#arm_reloc_half_sectdiff)
- [`ARM_RELOC_LOCAL_SECTDIFF`](#arm_reloc_local_sectdiff) - like ARM_RELOC_SECTDIFF, but the symbol referenced was local.
- [`ARM_RELOC_PAIR`](#arm_reloc_pair) - the second relocation entry of a pair
- [`ARM_RELOC_PB_LA_PTR`](#arm_reloc_pb_la_ptr) - prebound lazy pointer
- [`ARM_RELOC_SECTDIFF`](#arm_reloc_sectdiff) - a PAIR follows with subtract symbol value
- [`ARM_RELOC_VANILLA`](#arm_reloc_vanilla) - generic relocation as described above
- [`ARM_THUMB_32BIT_BRANCH`](#arm_thumb_32bit_branch) - obsolete - a thumb 32-bit branch instruction possibly needing page-spanning branch workaround
- [`ARM_THUMB_RELOC_BR22`](#arm_thumb_reloc_br22) - 22 bit branch displacement (to a half-word address)
- [`BIND_IMMEDIATE_MASK`](#bind_immediate_mask)
- [`BIND_OPCODE_ADD_ADDR_ULEB`](#bind_opcode_add_addr_uleb)
- [`BIND_OPCODE_DONE`](#bind_opcode_done)
- [`BIND_OPCODE_DO_BIND`](#bind_opcode_do_bind)
- [`BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED`](#bind_opcode_do_bind_add_addr_imm_scaled)
- [`BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB`](#bind_opcode_do_bind_add_addr_uleb)
- [`BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB`](#bind_opcode_do_bind_uleb_times_skipping_uleb)
- [`BIND_OPCODE_MASK`](#bind_opcode_mask)
- [`BIND_OPCODE_SET_ADDEND_SLEB`](#bind_opcode_set_addend_sleb)
- [`BIND_OPCODE_SET_DYLIB_ORDINAL_IMM`](#bind_opcode_set_dylib_ordinal_imm)
- [`BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB`](#bind_opcode_set_dylib_ordinal_uleb)
- [`BIND_OPCODE_SET_DYLIB_SPECIAL_IMM`](#bind_opcode_set_dylib_special_imm)
- [`BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB`](#bind_opcode_set_segment_and_offset_uleb)
- [`BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM`](#bind_opcode_set_symbol_trailing_flags_imm)
- [`BIND_OPCODE_SET_TYPE_IMM`](#bind_opcode_set_type_imm)
- [`BIND_OPCODE_THREADED`](#bind_opcode_threaded)
- [`BIND_SPECIAL_DYLIB_FLAT_LOOKUP`](#bind_special_dylib_flat_lookup)
- [`BIND_SPECIAL_DYLIB_MAIN_EXECUTABLE`](#bind_special_dylib_main_executable)
- [`BIND_SPECIAL_DYLIB_SELF`](#bind_special_dylib_self)
- [`BIND_SPECIAL_DYLIB_WEAK_LOOKUP`](#bind_special_dylib_weak_lookup)
- [`BIND_SUBOPCODE_THREADED_APPLY`](#bind_subopcode_threaded_apply)
- [`BIND_SUBOPCODE_THREADED_SET_BIND_ORDINAL_TABLE_SIZE_ULEB`](#bind_subopcode_threaded_set_bind_ordinal_table_size_uleb)
- [`BIND_SYMBOL_FLAGS_NON_WEAK_DEFINITION`](#bind_symbol_flags_non_weak_definition)
- [`BIND_SYMBOL_FLAGS_WEAK_IMPORT`](#bind_symbol_flags_weak_import)
- [`BIND_TYPE_POINTER`](#bind_type_pointer)
- [`BIND_TYPE_TEXT_ABSOLUTE32`](#bind_type_text_absolute32)
- [`BIND_TYPE_TEXT_PCREL32`](#bind_type_text_pcrel32)
- [`CPU_ARCH_ABI64`](#cpu_arch_abi64) - 64 bit ABI
- [`CPU_ARCH_ABI64_32`](#cpu_arch_abi64_32) - ABI for 64-bit hardware with 32-bit types; LP32
- [`CPU_ARCH_MASK`](#cpu_arch_mask) - mask for architecture bits
- [`CPU_SUBTYPE_386`](#cpu_subtype_386)
- [`CPU_SUBTYPE_486`](#cpu_subtype_486)
- [`CPU_SUBTYPE_486SX`](#cpu_subtype_486sx)
- [`CPU_SUBTYPE_586`](#cpu_subtype_586)
- [`CPU_SUBTYPE_ANY`](#cpu_subtype_any) - When selecting a slice, ANY will pick the slice with the best
- [`CPU_SUBTYPE_ARM64E`](#cpu_subtype_arm64e)
- [`CPU_SUBTYPE_ARM64_32_ALL`](#cpu_subtype_arm64_32_all)
- [`CPU_SUBTYPE_ARM64_32_V8`](#cpu_subtype_arm64_32_v8)
- [`CPU_SUBTYPE_ARM64_ALL`](#cpu_subtype_arm64_all)
- [`CPU_SUBTYPE_ARM64_V8`](#cpu_subtype_arm64_v8)
- [`CPU_SUBTYPE_ARM_ALL`](#cpu_subtype_arm_all)
- [`CPU_SUBTYPE_ARM_V4T`](#cpu_subtype_arm_v4t)
- [`CPU_SUBTYPE_ARM_V5TEJ`](#cpu_subtype_arm_v5tej)
- [`CPU_SUBTYPE_ARM_V6`](#cpu_subtype_arm_v6)
- [`CPU_SUBTYPE_ARM_V6M`](#cpu_subtype_arm_v6m) - Not meant to be run under xnu
- [`CPU_SUBTYPE_ARM_V7`](#cpu_subtype_arm_v7) - ARMv7-A and ARMv7-R
- [`CPU_SUBTYPE_ARM_V7EM`](#cpu_subtype_arm_v7em) - Not meant to be run under xnu
- [`CPU_SUBTYPE_ARM_V7F`](#cpu_subtype_arm_v7f) - Cortex A9
- [`CPU_SUBTYPE_ARM_V7K`](#cpu_subtype_arm_v7k)
- [`CPU_SUBTYPE_ARM_V7M`](#cpu_subtype_arm_v7m) - Not meant to be run under xnu
- [`CPU_SUBTYPE_ARM_V7S`](#cpu_subtype_arm_v7s) - Swift
- [`CPU_SUBTYPE_ARM_V8`](#cpu_subtype_arm_v8)
- [`CPU_SUBTYPE_ARM_V8M`](#cpu_subtype_arm_v8m) - Not meant to be run under xnu
- [`CPU_SUBTYPE_ARM_XSCALE`](#cpu_subtype_arm_xscale)
- [`CPU_SUBTYPE_BIG_ENDIAN`](#cpu_subtype_big_endian)
- [`CPU_SUBTYPE_CELERON`](#cpu_subtype_celeron)
- [`CPU_SUBTYPE_CELERON_MOBILE`](#cpu_subtype_celeron_mobile)
- [`CPU_SUBTYPE_HPPA_7100LC`](#cpu_subtype_hppa_7100lc)
- [`CPU_SUBTYPE_HPPA_ALL`](#cpu_subtype_hppa_all)
- [`CPU_SUBTYPE_I386_ALL`](#cpu_subtype_i386_all)
- [`CPU_SUBTYPE_I860_860`](#cpu_subtype_i860_860)
- [`CPU_SUBTYPE_I860_ALL`](#cpu_subtype_i860_all)
- [`CPU_SUBTYPE_INTEL_FAMILY_MAX`](#cpu_subtype_intel_family_max)
- [`CPU_SUBTYPE_INTEL_MODEL_ALL`](#cpu_subtype_intel_model_all)
- [`CPU_SUBTYPE_ITANIUM`](#cpu_subtype_itanium)
- [`CPU_SUBTYPE_ITANIUM_2`](#cpu_subtype_itanium_2)
- [`CPU_SUBTYPE_LIB64`](#cpu_subtype_lib64) - 64 bit libraries
- [`CPU_SUBTYPE_LITTLE_ENDIAN`](#cpu_subtype_little_endian)
- [`CPU_SUBTYPE_MASK`](#cpu_subtype_mask) - mask for feature flags
- [`CPU_SUBTYPE_MC68030`](#cpu_subtype_mc68030)
- [`CPU_SUBTYPE_MC68030_ONLY`](#cpu_subtype_mc68030_only)
- [`CPU_SUBTYPE_MC68040`](#cpu_subtype_mc68040)
- [`CPU_SUBTYPE_MC680X0_ALL`](#cpu_subtype_mc680x0_all)
- [`CPU_SUBTYPE_MC88000_ALL`](#cpu_subtype_mc88000_all)
- [`CPU_SUBTYPE_MC88100`](#cpu_subtype_mc88100)
- [`CPU_SUBTYPE_MC88110`](#cpu_subtype_mc88110)
- [`CPU_SUBTYPE_MC98000_ALL`](#cpu_subtype_mc98000_all)
- [`CPU_SUBTYPE_MC98601`](#cpu_subtype_mc98601)
- [`CPU_SUBTYPE_MIPS_ALL`](#cpu_subtype_mips_all)
- [`CPU_SUBTYPE_MIPS_R2000`](#cpu_subtype_mips_r2000)
- [`CPU_SUBTYPE_MIPS_R2000A`](#cpu_subtype_mips_r2000a) - pmax
- [`CPU_SUBTYPE_MIPS_R2300`](#cpu_subtype_mips_r2300)
- [`CPU_SUBTYPE_MIPS_R2600`](#cpu_subtype_mips_r2600)
- [`CPU_SUBTYPE_MIPS_R2800`](#cpu_subtype_mips_r2800)
- [`CPU_SUBTYPE_MIPS_R3000`](#cpu_subtype_mips_r3000)
- [`CPU_SUBTYPE_MIPS_R3000A`](#cpu_subtype_mips_r3000a) - 3max
- [`CPU_SUBTYPE_MULTIPLE`](#cpu_subtype_multiple)
- [`CPU_SUBTYPE_PENT`](#cpu_subtype_pent)
- [`CPU_SUBTYPE_PENTII_M3`](#cpu_subtype_pentii_m3)
- [`CPU_SUBTYPE_PENTII_M5`](#cpu_subtype_pentii_m5)
- [`CPU_SUBTYPE_PENTIUM_3`](#cpu_subtype_pentium_3)
- [`CPU_SUBTYPE_PENTIUM_3_M`](#cpu_subtype_pentium_3_m)
- [`CPU_SUBTYPE_PENTIUM_3_XEON`](#cpu_subtype_pentium_3_xeon)
- [`CPU_SUBTYPE_PENTIUM_4`](#cpu_subtype_pentium_4)
- [`CPU_SUBTYPE_PENTIUM_4_M`](#cpu_subtype_pentium_4_m)
- [`CPU_SUBTYPE_PENTIUM_M`](#cpu_subtype_pentium_m)
- [`CPU_SUBTYPE_PENTPRO`](#cpu_subtype_pentpro)
- [`CPU_SUBTYPE_POWERPC_601`](#cpu_subtype_powerpc_601)
- [`CPU_SUBTYPE_POWERPC_602`](#cpu_subtype_powerpc_602)
- [`CPU_SUBTYPE_POWERPC_603`](#cpu_subtype_powerpc_603)
- [`CPU_SUBTYPE_POWERPC_603E`](#cpu_subtype_powerpc_603e)
- [`CPU_SUBTYPE_POWERPC_603EV`](#cpu_subtype_powerpc_603ev)
- [`CPU_SUBTYPE_POWERPC_604`](#cpu_subtype_powerpc_604)
- [`CPU_SUBTYPE_POWERPC_604E`](#cpu_subtype_powerpc_604e)
- [`CPU_SUBTYPE_POWERPC_620`](#cpu_subtype_powerpc_620)
- [`CPU_SUBTYPE_POWERPC_7400`](#cpu_subtype_powerpc_7400)
- [`CPU_SUBTYPE_POWERPC_7450`](#cpu_subtype_powerpc_7450)
- [`CPU_SUBTYPE_POWERPC_750`](#cpu_subtype_powerpc_750)
- [`CPU_SUBTYPE_POWERPC_970`](#cpu_subtype_powerpc_970)
- [`CPU_SUBTYPE_POWERPC_ALL`](#cpu_subtype_powerpc_all)
- [`CPU_SUBTYPE_PTRAUTH_ABI`](#cpu_subtype_ptrauth_abi) - pointer authentication with versioned ABI
- [`CPU_SUBTYPE_SPARC_ALL`](#cpu_subtype_sparc_all)
- [`CPU_SUBTYPE_UVAXI`](#cpu_subtype_uvaxi)
- [`CPU_SUBTYPE_UVAXII`](#cpu_subtype_uvaxii)
- [`CPU_SUBTYPE_UVAXIII`](#cpu_subtype_uvaxiii)
- [`CPU_SUBTYPE_VAX730`](#cpu_subtype_vax730)
- [`CPU_SUBTYPE_VAX750`](#cpu_subtype_vax750)
- [`CPU_SUBTYPE_VAX780`](#cpu_subtype_vax780)
- [`CPU_SUBTYPE_VAX785`](#cpu_subtype_vax785)
- [`CPU_SUBTYPE_VAX8200`](#cpu_subtype_vax8200)
- [`CPU_SUBTYPE_VAX8500`](#cpu_subtype_vax8500)
- [`CPU_SUBTYPE_VAX8600`](#cpu_subtype_vax8600)
- [`CPU_SUBTYPE_VAX8650`](#cpu_subtype_vax8650)
- [`CPU_SUBTYPE_VAX8800`](#cpu_subtype_vax8800)
- [`CPU_SUBTYPE_VAX_ALL`](#cpu_subtype_vax_all)
- [`CPU_SUBTYPE_X86_64_ALL`](#cpu_subtype_x86_64_all)
- [`CPU_SUBTYPE_X86_64_H`](#cpu_subtype_x86_64_h) - Haswell feature subset
- [`CPU_SUBTYPE_X86_ALL`](#cpu_subtype_x86_all)
- [`CPU_SUBTYPE_X86_ARCH1`](#cpu_subtype_x86_arch1)
- [`CPU_SUBTYPE_XEON`](#cpu_subtype_xeon)
- [`CPU_SUBTYPE_XEON_MP`](#cpu_subtype_xeon_mp)
- [`CPU_TYPE_ALPHA`](#cpu_type_alpha)
- [`CPU_TYPE_ANY`](#cpu_type_any)
- [`CPU_TYPE_ARM`](#cpu_type_arm)
- [`CPU_TYPE_ARM64`](#cpu_type_arm64)
- [`CPU_TYPE_ARM64_32`](#cpu_type_arm64_32)
- [`CPU_TYPE_HPPA`](#cpu_type_hppa)
- [`CPU_TYPE_I860`](#cpu_type_i860)
- [`CPU_TYPE_MC680X0`](#cpu_type_mc680x0)
- [`CPU_TYPE_MC88000`](#cpu_type_mc88000)
- [`CPU_TYPE_MC98000`](#cpu_type_mc98000)
- [`CPU_TYPE_MIPS`](#cpu_type_mips)
- [`CPU_TYPE_POWERPC`](#cpu_type_powerpc)
- [`CPU_TYPE_POWERPC64`](#cpu_type_powerpc64)
- [`CPU_TYPE_SPARC`](#cpu_type_sparc)
- [`CPU_TYPE_VAX`](#cpu_type_vax)
- [`CPU_TYPE_X86`](#cpu_type_x86)
- [`CPU_TYPE_X86_64`](#cpu_type_x86_64)
- [`DICE_KIND_ABS_JUMP_TABLE32`](#dice_kind_abs_jump_table32)
- [`DICE_KIND_DATA`](#dice_kind_data)
- [`DICE_KIND_JUMP_TABLE16`](#dice_kind_jump_table16)
- [`DICE_KIND_JUMP_TABLE32`](#dice_kind_jump_table32)
- [`DICE_KIND_JUMP_TABLE8`](#dice_kind_jump_table8)
- [`DYLD_CACHE_DYNAMIC_CONFIG_DATA`](#dyld_cache_dynamic_config_data)
- [`DYLD_CACHE_MAPPING_AUTH_DATA`](#dyld_cache_mapping_auth_data)
- [`DYLD_CACHE_MAPPING_CONST_DATA`](#dyld_cache_mapping_const_data)
- [`DYLD_CACHE_MAPPING_DIRTY_DATA`](#dyld_cache_mapping_dirty_data)
- [`DYLD_CACHE_MAPPING_TEXT_STUBS`](#dyld_cache_mapping_text_stubs)
- [`DYLD_CACHE_SLIDE_PAGE_ATTRS`](#dyld_cache_slide_page_attrs)
- [`DYLD_CACHE_SLIDE_PAGE_ATTR_END`](#dyld_cache_slide_page_attr_end)
- [`DYLD_CACHE_SLIDE_PAGE_ATTR_EXTRA`](#dyld_cache_slide_page_attr_extra)
- [`DYLD_CACHE_SLIDE_PAGE_ATTR_NO_REBASE`](#dyld_cache_slide_page_attr_no_rebase)
- [`DYLD_CACHE_SLIDE_V3_PAGE_ATTR_NO_REBASE`](#dyld_cache_slide_v3_page_attr_no_rebase) - Page has no rebasing.
- [`DYLD_CACHE_SLIDE_V5_PAGE_ATTR_NO_REBASE`](#dyld_cache_slide_v5_page_attr_no_rebase) - Page has no rebasing.
- [`DYNAMIC_LOOKUP_ORDINAL`](#dynamic_lookup_ordinal)
- [`EXECUTABLE_ORDINAL`](#executable_ordinal)
- [`EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE`](#export_symbol_flags_kind_absolute)
- [`EXPORT_SYMBOL_FLAGS_KIND_MASK`](#export_symbol_flags_kind_mask)
- [`EXPORT_SYMBOL_FLAGS_KIND_REGULAR`](#export_symbol_flags_kind_regular)
- [`EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL`](#export_symbol_flags_kind_thread_local)
- [`EXPORT_SYMBOL_FLAGS_REEXPORT`](#export_symbol_flags_reexport)
- [`EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER`](#export_symbol_flags_stub_and_resolver)
- [`EXPORT_SYMBOL_FLAGS_WEAK_DEFINITION`](#export_symbol_flags_weak_definition)
- [`FAT_CIGAM`](#fat_cigam) - NXSwapLong(FAT_MAGIC)
- [`FAT_CIGAM_64`](#fat_cigam_64) - NXSwapLong(FAT_MAGIC_64)
- [`FAT_MAGIC`](#fat_magic)
- [`FAT_MAGIC_64`](#fat_magic_64)
- [`GENERIC_RELOC_LOCAL_SECTDIFF`](#generic_reloc_local_sectdiff)
- [`GENERIC_RELOC_PAIR`](#generic_reloc_pair) - Only follows a GENERIC_RELOC_SECTDIFF
- [`GENERIC_RELOC_PB_LA_PTR`](#generic_reloc_pb_la_ptr) - prebound lazy pointer
- [`GENERIC_RELOC_SECTDIFF`](#generic_reloc_sectdiff)
- [`GENERIC_RELOC_TLV`](#generic_reloc_tlv) - thread local variables
- [`GENERIC_RELOC_VANILLA`](#generic_reloc_vanilla) - generic relocation as described above
- [`INDIRECT_SYMBOL_ABS`](#indirect_symbol_abs)
- [`INDIRECT_SYMBOL_LOCAL`](#indirect_symbol_local)
- [`LC_BUILD_VERSION`](#lc_build_version) - build for platform min OS version
- [`LC_CODE_SIGNATURE`](#lc_code_signature) - local of code signature
- [`LC_DATA_IN_CODE`](#lc_data_in_code) - table of non-instructions in __text
- [`LC_DYLD_CHAINED_FIXUPS`](#lc_dyld_chained_fixups) - used with `LinkeditDataCommand`
- [`LC_DYLD_ENVIRONMENT`](#lc_dyld_environment) - string for dyld to treat like environment variable
- [`LC_DYLD_EXPORTS_TRIE`](#lc_dyld_exports_trie) - used with `LinkeditDataCommand`, payload is trie
- [`LC_DYLD_INFO`](#lc_dyld_info) - compressed dyld information
- [`LC_DYLD_INFO_ONLY`](#lc_dyld_info_only) - compressed dyld information only
- [`LC_DYLIB_CODE_SIGN_DRS`](#lc_dylib_code_sign_drs) - Code signing DRs copied from linked dylibs
- [`LC_DYSYMTAB`](#lc_dysymtab) - dynamic link-edit symbol table info
- [`LC_ENCRYPTION_INFO`](#lc_encryption_info) - encrypted segment information
- [`LC_ENCRYPTION_INFO_64`](#lc_encryption_info_64) - 64-bit encrypted segment information
- [`LC_FILESET_ENTRY`](#lc_fileset_entry) - used with `FilesetEntryCommand`
- [`LC_FUNCTION_STARTS`](#lc_function_starts) - compressed table of function start addresses
- [`LC_FVMFILE`](#lc_fvmfile) - fixed VM file inclusion (internal use)
- [`LC_IDENT`](#lc_ident) - object identification info (obsolete)
- [`LC_IDFVMLIB`](#lc_idfvmlib) - fixed VM shared library identification
- [`LC_ID_DYLIB`](#lc_id_dylib) - dynamically linked shared lib ident
- [`LC_ID_DYLINKER`](#lc_id_dylinker) - dynamic linker identification
- [`LC_LAZY_LOAD_DYLIB`](#lc_lazy_load_dylib) - delay load of dylib until first use
- [`LC_LINKER_OPTIMIZATION_HINT`](#lc_linker_optimization_hint) - optimization hints in MH_OBJECT files
- [`LC_LINKER_OPTION`](#lc_linker_option) - linker options in MH_OBJECT files
- [`LC_LOADFVMLIB`](#lc_loadfvmlib) - load a specified fixed VM shared library
- [`LC_LOAD_DYLIB`](#lc_load_dylib) - load a dynamically linked shared library
- [`LC_LOAD_DYLINKER`](#lc_load_dylinker) - load a dynamic linker
- [`LC_LOAD_UPWARD_DYLIB`](#lc_load_upward_dylib) - load upward dylib
- [`LC_LOAD_WEAK_DYLIB`](#lc_load_weak_dylib) - load a dynamically linked shared library that is allowed to be missing
- [`LC_MAIN`](#lc_main) - replacement for LC_UNIXTHREAD
- [`LC_NOTE`](#lc_note) - arbitrary data included within a Mach-O file
- [`LC_PREBIND_CKSUM`](#lc_prebind_cksum) - prebind checksum
- [`LC_PREBOUND_DYLIB`](#lc_prebound_dylib) - modules prebound for a dynamically linked shared library
- [`LC_PREPAGE`](#lc_prepage) - prepage command (internal use)
- [`LC_REEXPORT_DYLIB`](#lc_reexport_dylib) - load and re-export dylib
- [`LC_REQ_DYLD`](#lc_req_dyld)
- [`LC_ROUTINES`](#lc_routines) - image routines
- [`LC_ROUTINES_64`](#lc_routines_64) - 64-bit image routines
- [`LC_RPATH`](#lc_rpath) - runpath additions
- [`LC_SEGMENT`](#lc_segment) - segment of this file to be mapped
- [`LC_SEGMENT_64`](#lc_segment_64) - 64-bit segment of this file to be mapped
- [`LC_SEGMENT_SPLIT_INFO`](#lc_segment_split_info) - local of info to split segments
- [`LC_SOURCE_VERSION`](#lc_source_version) - source version used to build binary
- [`LC_SUB_CLIENT`](#lc_sub_client) - sub client
- [`LC_SUB_FRAMEWORK`](#lc_sub_framework) - sub framework
- [`LC_SUB_LIBRARY`](#lc_sub_library) - sub library
- [`LC_SUB_UMBRELLA`](#lc_sub_umbrella) - sub umbrella
- [`LC_SYMSEG`](#lc_symseg) - link-edit gdb symbol table info (obsolete)
- [`LC_SYMTAB`](#lc_symtab) - link-edit stab symbol table info
- [`LC_THREAD`](#lc_thread) - thread
- [`LC_TWOLEVEL_HINTS`](#lc_twolevel_hints) - two-level namespace lookup hints
- [`LC_UNIXTHREAD`](#lc_unixthread) - unix thread (includes a stack)
- [`LC_UUID`](#lc_uuid) - the uuid
- [`LC_VERSION_MIN_IPHONEOS`](#lc_version_min_iphoneos) - build for iPhoneOS min OS version
- [`LC_VERSION_MIN_MACOSX`](#lc_version_min_macosx) - build for MacOSX min OS version
- [`LC_VERSION_MIN_TVOS`](#lc_version_min_tvos) - build for AppleTV min OS version
- [`LC_VERSION_MIN_WATCHOS`](#lc_version_min_watchos) - build for Watch min OS version
- [`MAX_LIBRARY_ORDINAL`](#max_library_ordinal)
- [`MAX_SECT`](#max_sect) - 1 thru 255 inclusive
- [`MH_ALLMODSBOUND`](#mh_allmodsbound) - indicates that this binary binds to all two-level namespace modules of its dependent libraries. only used when MH_PREBINDABLE and MH_TWOLEVEL are both set.
- [`MH_ALLOW_STACK_EXECUTION`](#mh_allow_stack_execution) - When this bit is set, all stacks in the task will be given stack execution privilege.  Only used in MH_EXECUTE filetypes.
- [`MH_APP_EXTENSION_SAFE`](#mh_app_extension_safe) - The code was linked for use in an application extension.
- [`MH_BINDATLOAD`](#mh_bindatload) - the object file's undefined references are bound by the dynamic linker when loaded.
- [`MH_BINDS_TO_WEAK`](#mh_binds_to_weak) - the final linked image uses weak symbols
- [`MH_BUNDLE`](#mh_bundle) - dynamically bound bundle file
- [`MH_CANONICAL`](#mh_canonical) - the binary has been canonicalized via the unprebind operation
- [`MH_CIGAM`](#mh_cigam) - NXSwapInt(MH_MAGIC)
- [`MH_CIGAM_64`](#mh_cigam_64) - NXSwapInt(MH_MAGIC_64)
- [`MH_CORE`](#mh_core) - core file
- [`MH_DEAD_STRIPPABLE_DYLIB`](#mh_dead_strippable_dylib) - Only for use on dylibs.  When linking against a dylib that has this bit set, the static linker will automatically not create a LC_LOAD_DYLIB load command to the dylib if no symbols are being referenced from the dylib.
- [`MH_DSYM`](#mh_dsym) - companion file with only debug sections
- [`MH_DYLDLINK`](#mh_dyldlink) - the object file is input for the dynamic linker and can't be statically link edited again
- [`MH_DYLIB`](#mh_dylib) - dynamically bound shared library
- [`MH_DYLIB_IN_CACHE`](#mh_dylib_in_cache) - Only for use on dylibs. When this bit is set, the dylib is part of the dyld
- [`MH_DYLIB_STUB`](#mh_dylib_stub) - shared library stub for static linking only, no section contents
- [`MH_DYLINKER`](#mh_dylinker) - dynamic link editor
- [`MH_EXECUTE`](#mh_execute) - demand paged executable file
- [`MH_FILESET`](#mh_fileset) - set of mach-o's
- [`MH_FORCE_FLAT`](#mh_force_flat) - the executable is forcing all images to use flat name space bindings
- [`MH_FVMLIB`](#mh_fvmlib) - fixed VM shared library file
- [`MH_HAS_TLV_DESCRIPTORS`](#mh_has_tlv_descriptors) - Contains a section of type S_THREAD_LOCAL_VARIABLES
- [`MH_INCRLINK`](#mh_incrlink) - the object file is the output of an incremental link against a base file and can't be link edited again
- [`MH_KEXT_BUNDLE`](#mh_kext_bundle) - x86_64 kexts
- [`MH_LAZY_INIT`](#mh_lazy_init) - the shared library init routine is to be run lazily via catching memory faults to its writeable segments (obsolete)
- [`MH_MAGIC`](#mh_magic) - the mach magic number
- [`MH_MAGIC_64`](#mh_magic_64) - the 64-bit mach magic number
- [`MH_NLIST_OUTOFSYNC_WITH_DYLDINFO`](#mh_nlist_outofsync_with_dyldinfo) - The external symbols listed in the nlist symbol table do not include all the symbols listed in the dyld info.
- [`MH_NOFIXPREBINDING`](#mh_nofixprebinding) - do not have dyld notify the prebinding agent about this executable
- [`MH_NOMULTIDEFS`](#mh_nomultidefs) - this umbrella guarantees no multiple definitions of symbols in its sub-images so the two-level namespace hints can always be used.
- [`MH_NOUNDEFS`](#mh_noundefs) - the object file has no undefined references
- [`MH_NO_HEAP_EXECUTION`](#mh_no_heap_execution) - When this bit is set, the OS will run the main executable with a non-executable heap even on platforms (e.g. i386) that don't require it. Only used in MH_EXECUTE filetypes.
- [`MH_NO_REEXPORTED_DYLIBS`](#mh_no_reexported_dylibs) - When this bit is set on a dylib, the static linker does not need to examine dependent dylibs to see if any are re-exported
- [`MH_OBJECT`](#mh_object) - relocatable object file
- [`MH_PIE`](#mh_pie) - When this bit is set, the OS will load the main executable at a random address.  Only used in MH_EXECUTE filetypes.
- [`MH_PREBINDABLE`](#mh_prebindable) - the binary is not prebound but can have its prebinding redone. only used when MH_PREBOUND is not set.
- [`MH_PREBOUND`](#mh_prebound) - the file has its dynamic undefined references prebound.
- [`MH_PRELOAD`](#mh_preload) - preloaded executable file
- [`MH_ROOT_SAFE`](#mh_root_safe) - When this bit is set, the binary declares it is safe for use in processes with uid zero
- [`MH_SETUID_SAFE`](#mh_setuid_safe) - When this bit is set, the binary declares it is safe for use in processes when issetugid() is true
- [`MH_SIM_SUPPORT`](#mh_sim_support) - Allow LC_MIN_VERSION_MACOS and LC_BUILD_VERSION load commands with
- [`MH_SPLIT_SEGS`](#mh_split_segs) - the file has its read-only and read-write segments split
- [`MH_SUBSECTIONS_VIA_SYMBOLS`](#mh_subsections_via_symbols) - safe to divide up the sections into sub-sections via symbols for dead code stripping
- [`MH_TWOLEVEL`](#mh_twolevel) - the image is using two-level name space bindings
- [`MH_WEAK_DEFINES`](#mh_weak_defines) - the final linked image contains external weak symbols
- [`NO_SECT`](#no_sect) - symbol is not in any section
- [`N_ABS`](#n_abs) - absolute, n_sect == NO_SECT
- [`N_ALT_ENTRY`](#n_alt_entry)
- [`N_ARM_THUMB_DEF`](#n_arm_thumb_def) - symbol is a Thumb function (ARM)
- [`N_AST`](#n_ast) - AST file path: name,,NO_SECT,0,0
- [`N_BCOMM`](#n_bcomm) - begin common: name,,NO_SECT,0,0
- [`N_BINCL`](#n_bincl) - include file beginning: name,,NO_SECT,0,sum
- [`N_BNSYM`](#n_bnsym) - begin nsect sym: 0,,n_sect,0,address
- [`N_DESC_DISCARDED`](#n_desc_discarded) - symbol is discarded
- [`N_ECOML`](#n_ecoml) - end common (local name): 0,,n_sect,0,address
- [`N_ECOMM`](#n_ecomm) - end common: name,,n_sect,0,0
- [`N_EINCL`](#n_eincl) - include file end: name,,NO_SECT,0,0
- [`N_ENSYM`](#n_ensym) - end nsect sym: 0,,n_sect,0,address
- [`N_ENTRY`](#n_entry) - alternate entry: name,,n_sect,linenumber,address
- [`N_EXCL`](#n_excl) - deleted include file: name,,NO_SECT,0,sum
- [`N_EXT`](#n_ext) - external symbol bit, set for external symbols
- [`N_FNAME`](#n_fname) - procedure name (f77 kludge): name,,NO_SECT,0,0
- [`N_FUN`](#n_fun) - procedure: name,,n_sect,linenumber,address
- [`N_GSYM`](#n_gsym) - global symbol: name,,NO_SECT,type,0
- [`N_INDR`](#n_indr) - indirect
- [`N_LBRAC`](#n_lbrac) - left bracket: 0,,NO_SECT,nesting level,address
- [`N_LCSYM`](#n_lcsym) - .lcomm symbol: name,,n_sect,type,address
- [`N_LENG`](#n_leng) - second stab entry with length information
- [`N_LSYM`](#n_lsym) - local sym: name,,NO_SECT,type,offset
- [`N_NO_DEAD_STRIP`](#n_no_dead_strip) - symbol is not to be dead stripped
- [`N_OLEVEL`](#n_olevel) - compiler -O level: name,,NO_SECT,0,0
- [`N_OPT`](#n_opt) - emitted with gcc2_compiled and in gcc source
- [`N_OSO`](#n_oso) - object file name: name,,0,0,st_mtime
- [`N_PARAMS`](#n_params) - compiler parameters: name,,NO_SECT,0,0
- [`N_PBUD`](#n_pbud) - prebound undefined (defined in a dylib)
- [`N_PC`](#n_pc) - global pascal symbol: name,,NO_SECT,subtype,line
- [`N_PEXT`](#n_pext) - private external symbol bit
- [`N_PSYM`](#n_psym) - parameter: name,,NO_SECT,type,offset
- [`N_RBRAC`](#n_rbrac) - right bracket: 0,,NO_SECT,nesting level,address
- [`N_REF_TO_WEAK`](#n_ref_to_weak) - reference to a weak symbol
- [`N_RSYM`](#n_rsym) - register sym: name,,NO_SECT,type,register
- [`N_SECT`](#n_sect) - defined in section number n_sect
- [`N_SLINE`](#n_sline) - src line: 0,,n_sect,linenumber,address
- [`N_SO`](#n_so) - source file name: name,,n_sect,0,address
- [`N_SOL`](#n_sol) - #included file name: name,,n_sect,0,address
- [`N_SSYM`](#n_ssym) - structure elt: name,,NO_SECT,type,struct_offset
- [`N_STAB`](#n_stab) - if any of these bits set, a symbolic debugging entry
- [`N_STSYM`](#n_stsym) - static symbol: name,,n_sect,type,address
- [`N_SYMBOL_RESOLVER`](#n_symbol_resolver)
- [`N_TYPE`](#n_type) - mask for the type bits
- [`N_UNDF`](#n_undf) - undefined, n_sect == NO_SECT
- [`N_VERSION`](#n_version) - compiler version: name,,NO_SECT,0,0
- [`N_WEAK_DEF`](#n_weak_def) - coalesced symbol is a weak definition
- [`N_WEAK_REF`](#n_weak_ref) - symbol is weak referenced
- [`PLATFORM_BRIDGEOS`](#platform_bridgeos)
- [`PLATFORM_DRIVERKIT`](#platform_driverkit)
- [`PLATFORM_IOS`](#platform_ios)
- [`PLATFORM_IOSSIMULATOR`](#platform_iossimulator)
- [`PLATFORM_MACCATALYST`](#platform_maccatalyst)
- [`PLATFORM_MACOS`](#platform_macos)
- [`PLATFORM_TVOS`](#platform_tvos)
- [`PLATFORM_TVOSSIMULATOR`](#platform_tvossimulator)
- [`PLATFORM_WATCHOS`](#platform_watchos)
- [`PLATFORM_WATCHOSSIMULATOR`](#platform_watchossimulator)
- [`PLATFORM_XROS`](#platform_xros)
- [`PLATFORM_XROSSIMULATOR`](#platform_xrossimulator)
- [`PPC_RELOC_BR14`](#ppc_reloc_br14) - 14 bit branch displacement (to a word address)
- [`PPC_RELOC_BR24`](#ppc_reloc_br24) - 24 bit branch displacement (to a word address)
- [`PPC_RELOC_HA16`](#ppc_reloc_ha16) - Same as the RELOC_HI16 except the low 16 bits and the high 16 bits are added together
- [`PPC_RELOC_HA16_SECTDIFF`](#ppc_reloc_ha16_sectdiff)
- [`PPC_RELOC_HI16`](#ppc_reloc_hi16) - a PAIR follows with the low half
- [`PPC_RELOC_HI16_SECTDIFF`](#ppc_reloc_hi16_sectdiff) - section difference forms of above.  a PAIR
- [`PPC_RELOC_JBSR`](#ppc_reloc_jbsr)
- [`PPC_RELOC_LO14`](#ppc_reloc_lo14) - Same as the LO16 except that the low 2 bits are not stored in the instruction and are
- [`PPC_RELOC_LO14_SECTDIFF`](#ppc_reloc_lo14_sectdiff)
- [`PPC_RELOC_LO16`](#ppc_reloc_lo16) - a PAIR follows with the high half
- [`PPC_RELOC_LO16_SECTDIFF`](#ppc_reloc_lo16_sectdiff) - follows these with subtract symbol value
- [`PPC_RELOC_LOCAL_SECTDIFF`](#ppc_reloc_local_sectdiff) - like PPC_RELOC_SECTDIFF, but the symbol referenced was local.
- [`PPC_RELOC_PAIR`](#ppc_reloc_pair) - the second relocation entry of a pair
- [`PPC_RELOC_PB_LA_PTR`](#ppc_reloc_pb_la_ptr) - prebound lazy pointer
- [`PPC_RELOC_SECTDIFF`](#ppc_reloc_sectdiff) - a PAIR follows with subtract symbol value
- [`PPC_RELOC_VANILLA`](#ppc_reloc_vanilla) - generic relocation as described above
- [`REBASE_IMMEDIATE_MASK`](#rebase_immediate_mask)
- [`REBASE_OPCODE_ADD_ADDR_IMM_SCALED`](#rebase_opcode_add_addr_imm_scaled)
- [`REBASE_OPCODE_ADD_ADDR_ULEB`](#rebase_opcode_add_addr_uleb)
- [`REBASE_OPCODE_DONE`](#rebase_opcode_done)
- [`REBASE_OPCODE_DO_REBASE_ADD_ADDR_ULEB`](#rebase_opcode_do_rebase_add_addr_uleb)
- [`REBASE_OPCODE_DO_REBASE_IMM_TIMES`](#rebase_opcode_do_rebase_imm_times)
- [`REBASE_OPCODE_DO_REBASE_ULEB_TIMES`](#rebase_opcode_do_rebase_uleb_times)
- [`REBASE_OPCODE_DO_REBASE_ULEB_TIMES_SKIPPING_ULEB`](#rebase_opcode_do_rebase_uleb_times_skipping_uleb)
- [`REBASE_OPCODE_MASK`](#rebase_opcode_mask)
- [`REBASE_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB`](#rebase_opcode_set_segment_and_offset_uleb)
- [`REBASE_OPCODE_SET_TYPE_IMM`](#rebase_opcode_set_type_imm)
- [`REBASE_TYPE_POINTER`](#rebase_type_pointer)
- [`REBASE_TYPE_TEXT_ABSOLUTE32`](#rebase_type_text_absolute32)
- [`REBASE_TYPE_TEXT_PCREL32`](#rebase_type_text_pcrel32)
- [`REFERENCED_DYNAMICALLY`](#referenced_dynamically)
- [`REFERENCE_FLAG_DEFINED`](#reference_flag_defined)
- [`REFERENCE_FLAG_PRIVATE_DEFINED`](#reference_flag_private_defined)
- [`REFERENCE_FLAG_PRIVATE_UNDEFINED_LAZY`](#reference_flag_private_undefined_lazy)
- [`REFERENCE_FLAG_PRIVATE_UNDEFINED_NON_LAZY`](#reference_flag_private_undefined_non_lazy)
- [`REFERENCE_FLAG_UNDEFINED_LAZY`](#reference_flag_undefined_lazy)
- [`REFERENCE_FLAG_UNDEFINED_NON_LAZY`](#reference_flag_undefined_non_lazy)
- [`REFERENCE_TYPE`](#reference_type)
- [`R_ABS`](#r_abs) - absolute relocation type for Mach-O files
- [`R_SCATTERED`](#r_scattered) - Bit set in `Relocation::r_word0` for scattered relocations.
- [`SECTION_ATTRIBUTES`](#section_attributes) - 24 section attributes
- [`SECTION_ATTRIBUTES_SYS`](#section_attributes_sys) - system setable attributes
- [`SECTION_ATTRIBUTES_USR`](#section_attributes_usr) - User setable attributes
- [`SECTION_TYPE`](#section_type) - 256 section types
- [`SECT_BSS`](#sect_bss) - the real uninitialized data section no padding
- [`SECT_COMMON`](#sect_common) - the section common symbols are allocated in by the link editor
- [`SECT_DATA`](#sect_data) - the real initialized data section no padding, no bss overlap
- [`SECT_FVMLIB_INIT0`](#sect_fvmlib_init0) - the fvmlib initialization section
- [`SECT_FVMLIB_INIT1`](#sect_fvmlib_init1) - the section following the fvmlib initialization section
- [`SECT_ICON_HEADER`](#sect_icon_header) - the icon headers
- [`SECT_ICON_TIFF`](#sect_icon_tiff) - the icons in tiff format
- [`SECT_OBJC_MODULES`](#sect_objc_modules) - module information
- [`SECT_OBJC_REFS`](#sect_objc_refs) - string table
- [`SECT_OBJC_STRINGS`](#sect_objc_strings) - string table
- [`SECT_OBJC_SYMBOLS`](#sect_objc_symbols) - symbol table
- [`SECT_TEXT`](#sect_text) - the real text part of the text section no headers, and no padding
- [`SEG_DATA`](#seg_data) - the tradition UNIX data segment
- [`SEG_ICON`](#seg_icon) - the icon segment
- [`SEG_IMPORT`](#seg_import) - the segment for the self (dyld) modifying code stubs that has read, write and execute permissions
- [`SEG_LINKEDIT`](#seg_linkedit) - the segment containing all structs created and maintained by the link editor.  Created with -seglinkedit option to ld(1) for MH_EXECUTE and FVMLIB file types only
- [`SEG_LINKINFO`](#seg_linkinfo) - the segment overlapping with linkedit containing linking information
- [`SEG_OBJC`](#seg_objc) - objective-C runtime segment
- [`SEG_PAGEZERO`](#seg_pagezero) - the pagezero segment which has no protections and catches NULL references for MH_EXECUTE files
- [`SEG_TEXT`](#seg_text) - the tradition UNIX text segment
- [`SEG_UNIXSTACK`](#seg_unixstack) - the unix stack segment
- [`SELF_LIBRARY_ORDINAL`](#self_library_ordinal)
- [`SG_FVMLIB`](#sg_fvmlib) - this segment is the VM that is allocated by a fixed VM library, for overlap checking in the link editor
- [`SG_HIGHVM`](#sg_highvm) - the file contents for this segment is for the high part of the VM space, the low part is zero filled (for stacks in core files)
- [`SG_NORELOC`](#sg_noreloc) - this segment has nothing that was relocated in it and nothing relocated to it, that is it maybe safely replaced without relocation
- [`SG_PROTECTED_VERSION_1`](#sg_protected_version_1) - This segment is protected.  If the segment starts at file offset 0, the first page of the segment is not protected.  All other pages of the segment are protected.
- [`SG_READ_ONLY`](#sg_read_only) - This segment is made read-only after fixups
- [`S_16BYTE_LITERALS`](#s_16byte_literals) - section with only 16 byte literals
- [`S_4BYTE_LITERALS`](#s_4byte_literals) - section with only 4 byte literals
- [`S_8BYTE_LITERALS`](#s_8byte_literals) - section with only 8 byte literals
- [`S_ATTR_DEBUG`](#s_attr_debug) - a debug section
- [`S_ATTR_EXT_RELOC`](#s_attr_ext_reloc) - section has external relocation entries
- [`S_ATTR_LIVE_SUPPORT`](#s_attr_live_support) - blocks are live if they reference live blocks
- [`S_ATTR_LOC_RELOC`](#s_attr_loc_reloc) - section has local relocation entries
- [`S_ATTR_NO_DEAD_STRIP`](#s_attr_no_dead_strip) - no dead stripping
- [`S_ATTR_NO_TOC`](#s_attr_no_toc) - section contains coalesced symbols that are not to be in a ranlib table of contents
- [`S_ATTR_PURE_INSTRUCTIONS`](#s_attr_pure_instructions) - section contains only true machine instructions
- [`S_ATTR_SELF_MODIFYING_CODE`](#s_attr_self_modifying_code) - Used with i386 code stubs written on by dyld
- [`S_ATTR_SOME_INSTRUCTIONS`](#s_attr_some_instructions) - section contains some machine instructions
- [`S_ATTR_STRIP_STATIC_SYMS`](#s_attr_strip_static_syms) - ok to strip static symbols in this section in files with the MH_DYLDLINK flag
- [`S_COALESCED`](#s_coalesced) - section contains symbols that are to be coalesced
- [`S_CSTRING_LITERALS`](#s_cstring_literals) - section with only literal C strings
- [`S_DTRACE_DOF`](#s_dtrace_dof) - section contains DTrace Object Format
- [`S_GB_ZEROFILL`](#s_gb_zerofill) - zero fill on demand section (that can be larger than 4 gigabytes)
- [`S_INIT_FUNC_OFFSETS`](#s_init_func_offsets) - 32-bit offsets to initializers
- [`S_INTERPOSING`](#s_interposing) - section with only pairs of function pointers for interposing
- [`S_LAZY_DYLIB_SYMBOL_POINTERS`](#s_lazy_dylib_symbol_pointers) - section with only lazy symbol pointers to lazy loaded dylibs
- [`S_LAZY_SYMBOL_POINTERS`](#s_lazy_symbol_pointers) - section with only lazy symbol pointers
- [`S_LITERAL_POINTERS`](#s_literal_pointers) - section with only pointers to literals
- [`S_MOD_INIT_FUNC_POINTERS`](#s_mod_init_func_pointers) - section with only function pointers for initialization
- [`S_MOD_TERM_FUNC_POINTERS`](#s_mod_term_func_pointers) - section with only function pointers for termination
- [`S_NON_LAZY_SYMBOL_POINTERS`](#s_non_lazy_symbol_pointers) - section with only non-lazy symbol pointers
- [`S_REGULAR`](#s_regular) - regular section
- [`S_SYMBOL_STUBS`](#s_symbol_stubs) - section with only symbol stubs, byte size of stub in the reserved2 field
- [`S_THREAD_LOCAL_INIT_FUNCTION_POINTERS`](#s_thread_local_init_function_pointers) - functions to call to initialize TLV values
- [`S_THREAD_LOCAL_REGULAR`](#s_thread_local_regular) - template of initial values for TLVs
- [`S_THREAD_LOCAL_VARIABLES`](#s_thread_local_variables) - TLV descriptors
- [`S_THREAD_LOCAL_VARIABLE_POINTERS`](#s_thread_local_variable_pointers) - pointers to TLV descriptors
- [`S_THREAD_LOCAL_ZEROFILL`](#s_thread_local_zerofill) - template of initial values for TLVs
- [`S_ZEROFILL`](#s_zerofill) - zero fill on demand section
- [`TOOL_CLANG`](#tool_clang)
- [`TOOL_LD`](#tool_ld)
- [`TOOL_SWIFT`](#tool_swift)
- [`VM_PROT_EXECUTE`](#vm_prot_execute) - execute permission
- [`VM_PROT_READ`](#vm_prot_read) - read permission
- [`VM_PROT_WRITE`](#vm_prot_write) - write permission
- [`X86_64_RELOC_BRANCH`](#x86_64_reloc_branch) - a CALL/JMP instruction with 32-bit displacement
- [`X86_64_RELOC_GOT`](#x86_64_reloc_got) - other GOT references
- [`X86_64_RELOC_GOT_LOAD`](#x86_64_reloc_got_load) - a MOVQ load of a GOT entry
- [`X86_64_RELOC_SIGNED`](#x86_64_reloc_signed) - for signed 32-bit displacement
- [`X86_64_RELOC_SIGNED_1`](#x86_64_reloc_signed_1) - for signed 32-bit displacement with a -1 addend
- [`X86_64_RELOC_SIGNED_2`](#x86_64_reloc_signed_2) - for signed 32-bit displacement with a -2 addend
- [`X86_64_RELOC_SIGNED_4`](#x86_64_reloc_signed_4) - for signed 32-bit displacement with a -4 addend
- [`X86_64_RELOC_SUBTRACTOR`](#x86_64_reloc_subtractor) - must be followed by a X86_64_RELOC_UNSIGNED
- [`X86_64_RELOC_TLV`](#x86_64_reloc_tlv) - for thread local variables
- [`X86_64_RELOC_UNSIGNED`](#x86_64_reloc_unsigned) - for absolute addresses

---

## object::macho::ARM64_RELOC_ADDEND

*Constant*: `u8`

must be followed by PAGE21 or PAGEOFF12



## object::macho::ARM64_RELOC_AUTHENTICATED_POINTER

*Constant*: `u8`



## object::macho::ARM64_RELOC_BRANCH26

*Constant*: `u8`

a B/BL instruction with 26-bit displacement



## object::macho::ARM64_RELOC_GOT_LOAD_PAGE21

*Constant*: `u8`

pc-rel distance to page of GOT slot



## object::macho::ARM64_RELOC_GOT_LOAD_PAGEOFF12

*Constant*: `u8`

offset within page of GOT slot, scaled by r_length



## object::macho::ARM64_RELOC_PAGE21

*Constant*: `u8`

pc-rel distance to page of target



## object::macho::ARM64_RELOC_PAGEOFF12

*Constant*: `u8`

offset within page, scaled by r_length



## object::macho::ARM64_RELOC_POINTER_TO_GOT

*Constant*: `u8`

for pointers to GOT slots



## object::macho::ARM64_RELOC_SUBTRACTOR

*Constant*: `u8`

must be followed by a ARM64_RELOC_UNSIGNED



## object::macho::ARM64_RELOC_TLVP_LOAD_PAGE21

*Constant*: `u8`

pc-rel distance to page of TLVP slot



## object::macho::ARM64_RELOC_TLVP_LOAD_PAGEOFF12

*Constant*: `u8`

offset within page of TLVP slot, scaled by r_length



## object::macho::ARM64_RELOC_UNSIGNED

*Constant*: `u8`

for pointers



## object::macho::ARM_RELOC_BR24

*Constant*: `u8`

24 bit branch displacement (to a word address)



## object::macho::ARM_RELOC_HALF

*Constant*: `u8`



## object::macho::ARM_RELOC_HALF_SECTDIFF

*Constant*: `u8`



## object::macho::ARM_RELOC_LOCAL_SECTDIFF

*Constant*: `u8`

like ARM_RELOC_SECTDIFF, but the symbol referenced was local.



## object::macho::ARM_RELOC_PAIR

*Constant*: `u8`

the second relocation entry of a pair



## object::macho::ARM_RELOC_PB_LA_PTR

*Constant*: `u8`

prebound lazy pointer



## object::macho::ARM_RELOC_SECTDIFF

*Constant*: `u8`

a PAIR follows with subtract symbol value



## object::macho::ARM_RELOC_VANILLA

*Constant*: `u8`

generic relocation as described above



## object::macho::ARM_THUMB_32BIT_BRANCH

*Constant*: `u8`

obsolete - a thumb 32-bit branch instruction possibly needing page-spanning branch workaround



## object::macho::ARM_THUMB_RELOC_BR22

*Constant*: `u8`

22 bit branch displacement (to a half-word address)



## object::macho::BIND_IMMEDIATE_MASK

*Constant*: `u8`



## object::macho::BIND_OPCODE_ADD_ADDR_ULEB

*Constant*: `u8`



## object::macho::BIND_OPCODE_DONE

*Constant*: `u8`



## object::macho::BIND_OPCODE_DO_BIND

*Constant*: `u8`



## object::macho::BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED

*Constant*: `u8`



## object::macho::BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB

*Constant*: `u8`



## object::macho::BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB

*Constant*: `u8`



## object::macho::BIND_OPCODE_MASK

*Constant*: `u8`



## object::macho::BIND_OPCODE_SET_ADDEND_SLEB

*Constant*: `u8`



## object::macho::BIND_OPCODE_SET_DYLIB_ORDINAL_IMM

*Constant*: `u8`



## object::macho::BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB

*Constant*: `u8`



## object::macho::BIND_OPCODE_SET_DYLIB_SPECIAL_IMM

*Constant*: `u8`



## object::macho::BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB

*Constant*: `u8`



## object::macho::BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM

*Constant*: `u8`



## object::macho::BIND_OPCODE_SET_TYPE_IMM

*Constant*: `u8`



## object::macho::BIND_OPCODE_THREADED

*Constant*: `u8`



## object::macho::BIND_SPECIAL_DYLIB_FLAT_LOOKUP

*Constant*: `i8`



## object::macho::BIND_SPECIAL_DYLIB_MAIN_EXECUTABLE

*Constant*: `i8`



## object::macho::BIND_SPECIAL_DYLIB_SELF

*Constant*: `i8`



## object::macho::BIND_SPECIAL_DYLIB_WEAK_LOOKUP

*Constant*: `i8`



## object::macho::BIND_SUBOPCODE_THREADED_APPLY

*Constant*: `u8`



## object::macho::BIND_SUBOPCODE_THREADED_SET_BIND_ORDINAL_TABLE_SIZE_ULEB

*Constant*: `u8`



## object::macho::BIND_SYMBOL_FLAGS_NON_WEAK_DEFINITION

*Constant*: `u8`



## object::macho::BIND_SYMBOL_FLAGS_WEAK_IMPORT

*Constant*: `u8`



## object::macho::BIND_TYPE_POINTER

*Constant*: `u8`



## object::macho::BIND_TYPE_TEXT_ABSOLUTE32

*Constant*: `u8`



## object::macho::BIND_TYPE_TEXT_PCREL32

*Constant*: `u8`



## object::macho::BuildToolVersion

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `tool: crate::endian::U32<E>` - enum for the tool
- `version: crate::endian::U32<E>` - version number of the tool

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BuildToolVersion<E>`



## object::macho::BuildVersionCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_BUILD_VERSION
- `cmdsize: crate::endian::U32<E>` - sizeof(struct BuildVersionCommand) plus ntools * sizeof(struct BuildToolVersion)
- `platform: crate::endian::U32<E>` - platform
- `minos: crate::endian::U32<E>` - X.Y.Z is encoded in nibbles xxxx.yy.zz
- `sdk: crate::endian::U32<E>` - X.Y.Z is encoded in nibbles xxxx.yy.zz
- `ntools: crate::endian::U32<E>` - number of tool entries following this

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> BuildVersionCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::CPU_ARCH_ABI64

*Constant*: `u32`

64 bit ABI



## object::macho::CPU_ARCH_ABI64_32

*Constant*: `u32`

ABI for 64-bit hardware with 32-bit types; LP32



## object::macho::CPU_ARCH_MASK

*Constant*: `u32`

mask for architecture bits



## object::macho::CPU_SUBTYPE_386

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_486

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_486SX

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_586

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ANY

*Constant*: `u32`

When selecting a slice, ANY will pick the slice with the best
grading for the selected cpu_type_t, unlike the "ALL" subtypes,
which are the slices that can run on any hardware for that cpu type.



## object::macho::CPU_SUBTYPE_ARM64E

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM64_32_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM64_32_V8

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM64_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM64_V8

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM_V4T

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM_V5TEJ

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM_V6

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM_V6M

*Constant*: `u32`

Not meant to be run under xnu



## object::macho::CPU_SUBTYPE_ARM_V7

*Constant*: `u32`

ARMv7-A and ARMv7-R



## object::macho::CPU_SUBTYPE_ARM_V7EM

*Constant*: `u32`

Not meant to be run under xnu



## object::macho::CPU_SUBTYPE_ARM_V7F

*Constant*: `u32`

Cortex A9



## object::macho::CPU_SUBTYPE_ARM_V7K

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM_V7M

*Constant*: `u32`

Not meant to be run under xnu



## object::macho::CPU_SUBTYPE_ARM_V7S

*Constant*: `u32`

Swift



## object::macho::CPU_SUBTYPE_ARM_V8

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ARM_V8M

*Constant*: `u32`

Not meant to be run under xnu



## object::macho::CPU_SUBTYPE_ARM_XSCALE

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_BIG_ENDIAN

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_CELERON

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_CELERON_MOBILE

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_HPPA_7100LC

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_HPPA_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_I386_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_I860_860

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_I860_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_INTEL_FAMILY_MAX

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_INTEL_MODEL_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ITANIUM

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_ITANIUM_2

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_LIB64

*Constant*: `u32`

64 bit libraries



## object::macho::CPU_SUBTYPE_LITTLE_ENDIAN

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MASK

*Constant*: `u32`

mask for feature flags



## object::macho::CPU_SUBTYPE_MC68030

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MC68030_ONLY

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MC68040

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MC680X0_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MC88000_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MC88100

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MC88110

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MC98000_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MC98601

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MIPS_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MIPS_R2000

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MIPS_R2000A

*Constant*: `u32`

pmax



## object::macho::CPU_SUBTYPE_MIPS_R2300

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MIPS_R2600

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MIPS_R2800

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MIPS_R3000

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_MIPS_R3000A

*Constant*: `u32`

3max



## object::macho::CPU_SUBTYPE_MULTIPLE

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PENT

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PENTII_M3

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PENTII_M5

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PENTIUM_3

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PENTIUM_3_M

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PENTIUM_3_XEON

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PENTIUM_4

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PENTIUM_4_M

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PENTIUM_M

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PENTPRO

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_601

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_602

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_603

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_603E

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_603EV

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_604

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_604E

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_620

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_7400

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_7450

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_750

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_970

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_POWERPC_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_PTRAUTH_ABI

*Constant*: `u32`

pointer authentication with versioned ABI



## object::macho::CPU_SUBTYPE_SPARC_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_UVAXI

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_UVAXII

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_UVAXIII

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_VAX730

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_VAX750

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_VAX780

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_VAX785

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_VAX8200

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_VAX8500

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_VAX8600

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_VAX8650

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_VAX8800

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_VAX_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_X86_64_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_X86_64_H

*Constant*: `u32`

Haswell feature subset



## object::macho::CPU_SUBTYPE_X86_ALL

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_X86_ARCH1

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_XEON

*Constant*: `u32`



## object::macho::CPU_SUBTYPE_XEON_MP

*Constant*: `u32`



## object::macho::CPU_TYPE_ALPHA

*Constant*: `u32`



## object::macho::CPU_TYPE_ANY

*Constant*: `u32`



## object::macho::CPU_TYPE_ARM

*Constant*: `u32`



## object::macho::CPU_TYPE_ARM64

*Constant*: `u32`



## object::macho::CPU_TYPE_ARM64_32

*Constant*: `u32`



## object::macho::CPU_TYPE_HPPA

*Constant*: `u32`



## object::macho::CPU_TYPE_I860

*Constant*: `u32`



## object::macho::CPU_TYPE_MC680X0

*Constant*: `u32`



## object::macho::CPU_TYPE_MC88000

*Constant*: `u32`



## object::macho::CPU_TYPE_MC98000

*Constant*: `u32`



## object::macho::CPU_TYPE_MIPS

*Constant*: `u32`



## object::macho::CPU_TYPE_POWERPC

*Constant*: `u32`



## object::macho::CPU_TYPE_POWERPC64

*Constant*: `u32`



## object::macho::CPU_TYPE_SPARC

*Constant*: `u32`



## object::macho::CPU_TYPE_VAX

*Constant*: `u32`



## object::macho::CPU_TYPE_X86

*Constant*: `u32`



## object::macho::CPU_TYPE_X86_64

*Constant*: `u32`



## object::macho::DICE_KIND_ABS_JUMP_TABLE32

*Constant*: `u32`



## object::macho::DICE_KIND_DATA

*Constant*: `u32`



## object::macho::DICE_KIND_JUMP_TABLE16

*Constant*: `u32`



## object::macho::DICE_KIND_JUMP_TABLE32

*Constant*: `u32`



## object::macho::DICE_KIND_JUMP_TABLE8

*Constant*: `u32`



## object::macho::DYLD_CACHE_DYNAMIC_CONFIG_DATA

*Constant*: `u64`



## object::macho::DYLD_CACHE_MAPPING_AUTH_DATA

*Constant*: `u64`



## object::macho::DYLD_CACHE_MAPPING_CONST_DATA

*Constant*: `u64`



## object::macho::DYLD_CACHE_MAPPING_DIRTY_DATA

*Constant*: `u64`



## object::macho::DYLD_CACHE_MAPPING_TEXT_STUBS

*Constant*: `u64`



## object::macho::DYLD_CACHE_SLIDE_PAGE_ATTRS

*Constant*: `u16`



## object::macho::DYLD_CACHE_SLIDE_PAGE_ATTR_END

*Constant*: `u16`



## object::macho::DYLD_CACHE_SLIDE_PAGE_ATTR_EXTRA

*Constant*: `u16`



## object::macho::DYLD_CACHE_SLIDE_PAGE_ATTR_NO_REBASE

*Constant*: `u16`



## object::macho::DYLD_CACHE_SLIDE_V3_PAGE_ATTR_NO_REBASE

*Constant*: `u16`

Page has no rebasing.



## object::macho::DYLD_CACHE_SLIDE_V5_PAGE_ATTR_NO_REBASE

*Constant*: `u16`

Page has no rebasing.



## object::macho::DYNAMIC_LOOKUP_ORDINAL

*Constant*: `u8`



## object::macho::DataInCodeEntry

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `offset: crate::endian::U32<E>` - from mach_header to start of data range
- `length: crate::endian::U16<E>` - number of bytes in data range
- `kind: crate::endian::U16<E>` - a DICE_KIND_* value

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DataInCodeEntry<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::DyldCacheHeader

*Struct*

The dyld cache header.
Corresponds to struct dyld_cache_header from dyld_cache_format.h.
This header has grown over time. Only the fields up to and including dyld_base_address
are guaranteed to be present. For all other fields, check the header size before
accessing the field. The header size is stored in mapping_offset; the mappings start
right after the theader.

**Generic Parameters:**
- E

**Fields:**
- `magic: [u8; 16]` - e.g. "dyld_v0    i386"
- `mapping_offset: crate::endian::U32<E>` - file offset to first dyld_cache_mapping_info
- `mapping_count: crate::endian::U32<E>` - number of dyld_cache_mapping_info entries
- `images_offset_old: crate::endian::U32<E>` - UNUSED: moved to imagesOffset to prevent older dsc_extarctors from crashing
- `images_count_old: crate::endian::U32<E>` - UNUSED: moved to imagesCount to prevent older dsc_extarctors from crashing
- `dyld_base_address: crate::endian::U64<E>` - base address of dyld when cache was built
- `code_signature_offset: crate::endian::U64<E>` - file offset of code signature blob
- `code_signature_size: crate::endian::U64<E>` - size of code signature blob (zero means to end of file)
- `slide_info_offset_unused: crate::endian::U64<E>` - unused.  Used to be file offset of kernel slid info
- `slide_info_size_unused: crate::endian::U64<E>` - unused.  Used to be size of kernel slid info
- `local_symbols_offset: crate::endian::U64<E>` - file offset of where local symbols are stored
- `local_symbols_size: crate::endian::U64<E>` - size of local symbols information
- `uuid: [u8; 16]` - unique value for each shared cache file
- `cache_type: crate::endian::U64<E>` - 0 for development, 1 for production, 2 for multi-cache
- `branch_pools_offset: crate::endian::U32<E>` - file offset to table of uint64_t pool addresses
- `branch_pools_count: crate::endian::U32<E>` - number of uint64_t entries
- `dyld_in_cache_mh: crate::endian::U64<E>` - (unslid) address of mach_header of dyld in cache
- `dyld_in_cache_entry: crate::endian::U64<E>` - (unslid) address of entry point (_dyld_start) of dyld in cache
- `images_text_offset: crate::endian::U64<E>` - file offset to first dyld_cache_image_text_info
- `images_text_count: crate::endian::U64<E>` - number of dyld_cache_image_text_info entries
- `patch_info_addr: crate::endian::U64<E>` - (unslid) address of dyld_cache_patch_info
- `patch_info_size: crate::endian::U64<E>` - Size of all of the patch information pointed to via the dyld_cache_patch_info
- `other_image_group_addr_unused: crate::endian::U64<E>` - unused
- `other_image_group_size_unused: crate::endian::U64<E>` - unused
- `prog_closures_addr: crate::endian::U64<E>` - (unslid) address of list of program launch closures
- `prog_closures_size: crate::endian::U64<E>` - size of list of program launch closures
- `prog_closures_trie_addr: crate::endian::U64<E>` - (unslid) address of trie of indexes into program launch closures
- `prog_closures_trie_size: crate::endian::U64<E>` - size of trie of indexes into program launch closures
- `platform: crate::endian::U32<E>` - platform number (macOS=1, etc)
- `flags: crate::endian::U32<E>`
- `shared_region_start: crate::endian::U64<E>` - base load address of cache if not slid
- `shared_region_size: crate::endian::U64<E>` - overall size required to map the cache and all subCaches, if any
- `max_slide: crate::endian::U64<E>` - runtime slide of cache can be between zero and this value
- `dylibs_image_array_addr: crate::endian::U64<E>` - (unslid) address of ImageArray for dylibs in this cache
- `dylibs_image_array_size: crate::endian::U64<E>` - size of ImageArray for dylibs in this cache
- `dylibs_trie_addr: crate::endian::U64<E>` - (unslid) address of trie of indexes of all cached dylibs
- `dylibs_trie_size: crate::endian::U64<E>` - size of trie of cached dylib paths
- `other_image_array_addr: crate::endian::U64<E>` - (unslid) address of ImageArray for dylibs and bundles with dlopen closures
- `other_image_array_size: crate::endian::U64<E>` - size of ImageArray for dylibs and bundles with dlopen closures
- `other_trie_addr: crate::endian::U64<E>` - (unslid) address of trie of indexes of all dylibs and bundles with dlopen closures
- `other_trie_size: crate::endian::U64<E>` - size of trie of dylibs and bundles with dlopen closures
- `mapping_with_slide_offset: crate::endian::U32<E>` - file offset to first dyld_cache_mapping_and_slide_info
- `mapping_with_slide_count: crate::endian::U32<E>` - number of dyld_cache_mapping_and_slide_info entries
- `dylibs_pbl_state_array_addr_unused: crate::endian::U64<E>` - unused
- `dylibs_pbl_set_addr: crate::endian::U64<E>` - (unslid) address of PrebuiltLoaderSet of all cached dylibs
- `programs_pbl_set_pool_addr: crate::endian::U64<E>` - (unslid) address of pool of PrebuiltLoaderSet for each program
- `programs_pbl_set_pool_size: crate::endian::U64<E>` - size of pool of PrebuiltLoaderSet for each program
- `program_trie_addr: crate::endian::U64<E>` - (unslid) address of trie mapping program path to PrebuiltLoaderSet
- `os_version: crate::endian::U32<E>` - OS Version of dylibs in this cache for the main platform
- `alt_platform: crate::endian::U32<E>` - e.g. iOSMac on macOS
- `alt_os_version: crate::endian::U32<E>` - e.g. 14.0 for iOSMac
- `swift_opts_offset: crate::endian::U64<E>` - VM offset from cache_header* to Swift optimizations header
- `swift_opts_size: crate::endian::U64<E>` - size of Swift optimizations header
- `sub_cache_array_offset: crate::endian::U32<E>` - file offset to first dyld_subcache_entry
- `sub_cache_array_count: crate::endian::U32<E>` - number of subCache entries
- `symbol_file_uuid: [u8; 16]` - unique value for the shared cache file containing unmapped local symbols
- `rosetta_read_only_addr: crate::endian::U64<E>` - (unslid) address of the start of where Rosetta can add read-only/executable data
- `rosetta_read_only_size: crate::endian::U64<E>` - maximum size of the Rosetta read-only/executable region
- `rosetta_read_write_addr: crate::endian::U64<E>` - (unslid) address of the start of where Rosetta can add read-write data
- `rosetta_read_write_size: crate::endian::U64<E>` - maximum size of the Rosetta read-write region
- `images_offset: crate::endian::U32<E>` - file offset to first dyld_cache_image_info
- `images_count: crate::endian::U32<E>` - number of dyld_cache_image_info entries
- `cache_sub_type: crate::endian::U32<E>` - 0 for development, 1 for production, when cacheType is multi-cache(2)
- `objc_opts_offset: crate::endian::U64<E>` - VM offset from cache_header* to ObjC optimizations header
- `objc_opts_size: crate::endian::U64<E>` - size of ObjC optimizations header
- `cache_atlas_offset: crate::endian::U64<E>` - VM offset from cache_header* to embedded cache atlas for process introspection
- `cache_atlas_size: crate::endian::U64<E>` - size of embedded cache atlas
- `dynamic_data_offset: crate::endian::U64<E>` - VM offset from cache_header* to the location of dyld_cache_dynamic_data_header
- `dynamic_data_max_size: crate::endian::U64<E>` - maximum size of space reserved from dynamic data

**Methods:**

- `fn parse<'data, R>(data: R) -> Result<&'data Self>` - Read the dyld cache header.
- `fn parse_magic(self: &Self) -> Result<(Architecture, E)>` - Returns (arch, endian) based on the magic string.
- `fn mappings<'data, R>(self: &Self, endian: E, data: R) -> Result<DyldCacheMappingSlice<'data, E>>` - Return the mapping information table.
- `fn subcaches<'data, R>(self: &Self, endian: E, data: R) -> Result<Option<DyldSubCacheSlice<'data, E>>>` - Return the information about subcaches, if present.
- `fn symbols_subcache_uuid(self: &Self, endian: E) -> Option<[u8; 16]>` - Return the UUID for the .symbols subcache, if present.
- `fn images<'data, R>(self: &Self, endian: E, data: R) -> Result<&'data [macho::DyldCacheImageInfo<E>]>` - Return the image information table.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DyldCacheHeader<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::DyldCacheImageInfo

*Struct*

Corresponds to struct dyld_cache_image_info from dyld_cache_format.h.

**Generic Parameters:**
- E

**Fields:**
- `address: crate::endian::U64<E>`
- `mod_time: crate::endian::U64<E>`
- `inode: crate::endian::U64<E>`
- `path_file_offset: crate::endian::U32<E>`
- `pad: crate::endian::U32<E>`

**Methods:**

- `fn path<'data, R>(self: &Self, endian: E, data: R) -> Result<&'data [u8]>` - The file system path of this image.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldCacheImageInfo<E>`



## object::macho::DyldCacheMappingAndSlideInfo

*Struct*

Corresponds to struct dyld_cache_mapping_and_slide_info from dyld_cache_format.h.

**Generic Parameters:**
- E

**Fields:**
- `address: crate::endian::U64<E>`
- `size: crate::endian::U64<E>`
- `file_offset: crate::endian::U64<E>`
- `slide_info_file_offset: crate::endian::U64<E>`
- `slide_info_file_size: crate::endian::U64<E>`
- `flags: crate::endian::U64<E>`
- `max_prot: crate::endian::U32<E>`
- `init_prot: crate::endian::U32<E>`

**Methods:**

- `fn slide<'data, R>(self: &Self, endian: E, data: R) -> Result<DyldCacheSlideInfo<'data, E>>` - Return the (optional) array of slide information structs

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldCacheMappingAndSlideInfo<E>`



## object::macho::DyldCacheMappingInfo

*Struct*

Corresponds to struct dyld_cache_mapping_info from dyld_cache_format.h.

**Generic Parameters:**
- E

**Fields:**
- `address: crate::endian::U64<E>`
- `size: crate::endian::U64<E>`
- `file_offset: crate::endian::U64<E>`
- `max_prot: crate::endian::U32<E>`
- `init_prot: crate::endian::U32<E>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldCacheMappingInfo<E>`



## object::macho::DyldCacheSlideInfo2

*Struct*

Corresponds to struct dyld_cache_slide_info2 from dyld_cache_format.h.

**Generic Parameters:**
- E

**Fields:**
- `version: crate::endian::U32<E>`
- `page_size: crate::endian::U32<E>`
- `page_starts_offset: crate::endian::U32<E>`
- `page_starts_count: crate::endian::U32<E>`
- `page_extras_offset: crate::endian::U32<E>`
- `page_extras_count: crate::endian::U32<E>`
- `delta_mask: crate::endian::U64<E>`
- `value_add: crate::endian::U64<E>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DyldCacheSlideInfo2<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::DyldCacheSlideInfo3

*Struct*

Corresponds to struct dyld_cache_slide_info3 from dyld_cache_format.h.

**Generic Parameters:**
- E

**Fields:**
- `version: crate::endian::U32<E>`
- `page_size: crate::endian::U32<E>`
- `page_starts_count: crate::endian::U32<E>`
- `auth_value_add: crate::endian::U64<E>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DyldCacheSlideInfo3<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::DyldCacheSlideInfo5

*Struct*

Corresponds to struct dyld_cache_slide_info5 from dyld_cache_format.h.

**Generic Parameters:**
- E

**Fields:**
- `version: crate::endian::U32<E>`
- `page_size: crate::endian::U32<E>`
- `page_starts_count: crate::endian::U32<E>`
- `value_add: crate::endian::U64<E>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldCacheSlideInfo5<E>`



## object::macho::DyldCacheSlidePointer3

*Struct*

Corresponds to union dyld_cache_slide_pointer3 from dyld_cache_format.h.

**Tuple Struct**: `(u64)`

**Methods:**

- `fn is_auth(self: &Self) -> bool` - Whether the pointer is authenticated.
- `fn target(self: &Self) -> u64` - The target of the pointer.
- `fn high8(self: &Self) -> u64` - The high 8 bits of the pointer.
- `fn runtime_offset(self: &Self) -> u64` - The target of the pointer as an offset from the start of the shared cache.
- `fn diversity(self: &Self) -> u16` - The diversity value for authentication.
- `fn addr_div(self: &Self) -> bool` - Whether to use address diversity for authentication.
- `fn key(self: &Self) -> u8` - The key for authentication.
- `fn next(self: &Self) -> u64` - The offset to the next slide pointer in 8-byte units.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldCacheSlidePointer3`



## object::macho::DyldCacheSlidePointer5

*Struct*

Corresponds to struct dyld_cache_slide_pointer5 from dyld_cache_format.h.

**Tuple Struct**: `(u64)`

**Methods:**

- `fn is_auth(self: &Self) -> bool` - Whether the pointer is authenticated.
- `fn runtime_offset(self: &Self) -> u64` - The target of the pointer as an offset from the start of the shared cache.
- `fn high8(self: &Self) -> u64` - The high 8 bits of the pointer.
- `fn diversity(self: &Self) -> u16` - The diversity value for authentication.
- `fn addr_div(self: &Self) -> bool` - Whether to use address diversity for authentication.
- `fn key_is_data(self: &Self) -> bool` - Whether the key is IA or DA.
- `fn next(self: &Self) -> u64` - The offset to the next slide pointer in 8-byte units.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldCacheSlidePointer5`



## object::macho::DyldInfoCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_DYLD_INFO or LC_DYLD_INFO_ONLY
- `cmdsize: crate::endian::U32<E>` - sizeof(struct DyldInfoCommand)
- `rebase_off: crate::endian::U32<E>` - file offset to rebase info
- `rebase_size: crate::endian::U32<E>` - size of rebase info
- `bind_off: crate::endian::U32<E>` - file offset to binding info
- `bind_size: crate::endian::U32<E>` - size of binding info
- `weak_bind_off: crate::endian::U32<E>` - file offset to weak binding info
- `weak_bind_size: crate::endian::U32<E>` - size of weak binding info
- `lazy_bind_off: crate::endian::U32<E>` - file offset to lazy binding info
- `lazy_bind_size: crate::endian::U32<E>` - size of lazy binding infs
- `export_off: crate::endian::U32<E>` - file offset to lazy binding info
- `export_size: crate::endian::U32<E>` - size of lazy binding infs

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DyldInfoCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::DyldSubCacheEntryV1

*Struct*

Added in dyld-940, which shipped with macOS 12 / iOS 15.
Originally called `dyld_subcache_entry`, renamed to `dyld_subcache_entry_v1`
in dyld-1042.1.

**Generic Parameters:**
- E

**Fields:**
- `uuid: [u8; 16]` - The UUID of this subcache.
- `cache_vm_offset: crate::endian::U64<E>` - The offset of this subcache from the main cache base address.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldSubCacheEntryV1<E>`



## object::macho::DyldSubCacheEntryV2

*Struct*

Added in dyld-1042.1, which shipped with macOS 13 / iOS 16.
Called `dyld_subcache_entry` as of dyld-1042.1.

**Generic Parameters:**
- E

**Fields:**
- `uuid: [u8; 16]` - The UUID of this subcache.
- `cache_vm_offset: crate::endian::U64<E>` - The offset of this subcache from the main cache base address.
- `file_suffix: [u8; 32]` - The file name suffix of the subCache file, e.g. ".25.data" or ".03.development".

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldSubCacheEntryV2<E>`



## object::macho::Dylib

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `name: LcStr<E>` - library's path name
- `timestamp: crate::endian::U32<E>` - library's build time stamp
- `current_version: crate::endian::U32<E>` - library's current version number
- `compatibility_version: crate::endian::U32<E>` - library's compatibility vers number

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Dylib<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::DylibCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_ID_DYLIB, LC_LOAD_{,WEAK_}DYLIB, LC_REEXPORT_DYLIB
- `cmdsize: crate::endian::U32<E>` - includes pathname string
- `dylib: Dylib<E>` - the library identification

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DylibCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::DylibModule32

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `module_name: crate::endian::U32<E>` - the module name (index into string table)
- `iextdefsym: crate::endian::U32<E>` - index into externally defined symbols
- `nextdefsym: crate::endian::U32<E>` - number of externally defined symbols
- `irefsym: crate::endian::U32<E>` - index into reference symbol table
- `nrefsym: crate::endian::U32<E>` - number of reference symbol table entries
- `ilocalsym: crate::endian::U32<E>` - index into symbols for local symbols
- `nlocalsym: crate::endian::U32<E>` - number of local symbols
- `iextrel: crate::endian::U32<E>` - index into external relocation entries
- `nextrel: crate::endian::U32<E>` - number of external relocation entries
- `iinit_iterm: crate::endian::U32<E>` - low 16 bits are the index into the init section, high 16 bits are the index into the term section
- `ninit_nterm: crate::endian::U32<E>` - low 16 bits are the number of init section entries, high 16 bits are the number of term section entries
- `objc_module_info_addr: crate::endian::U32<E>` - for this module address of the start of the (__OBJC,__module_info) section
- `objc_module_info_size: crate::endian::U32<E>` - for this module size of the (__OBJC,__module_info) section

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DylibModule32<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::DylibModule64

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `module_name: crate::endian::U32<E>` - the module name (index into string table)
- `iextdefsym: crate::endian::U32<E>` - index into externally defined symbols
- `nextdefsym: crate::endian::U32<E>` - number of externally defined symbols
- `irefsym: crate::endian::U32<E>` - index into reference symbol table
- `nrefsym: crate::endian::U32<E>` - number of reference symbol table entries
- `ilocalsym: crate::endian::U32<E>` - index into symbols for local symbols
- `nlocalsym: crate::endian::U32<E>` - number of local symbols
- `iextrel: crate::endian::U32<E>` - index into external relocation entries
- `nextrel: crate::endian::U32<E>` - number of external relocation entries
- `iinit_iterm: crate::endian::U32<E>` - low 16 bits are the index into the init section, high 16 bits are the index into the term section
- `ninit_nterm: crate::endian::U32<E>` - low 16 bits are the number of init section entries, high 16 bits are the number of term section entries
- `objc_module_info_size: crate::endian::U32<E>` - for this module size of the (__OBJC,__module_info) section
- `objc_module_info_addr: crate::endian::U64<E>` - for this module address of the start of the (__OBJC,__module_info) section

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DylibModule64<E>`



## object::macho::DylibReference

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `bitfield: crate::endian::U32<E>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DylibReference<E>`



## object::macho::DylibTableOfContents

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `symbol_index: crate::endian::U32<E>` - the defined external symbol (index into the symbol table)
- `module_index: crate::endian::U32<E>` - index into the module table this symbol is defined in

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DylibTableOfContents<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::DylinkerCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_ID_DYLINKER, LC_LOAD_DYLINKER or LC_DYLD_ENVIRONMENT
- `cmdsize: crate::endian::U32<E>` - includes pathname string
- `name: LcStr<E>` - dynamic linker's path name

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DylinkerCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::DysymtabCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_DYSYMTAB
- `cmdsize: crate::endian::U32<E>` - sizeof(struct DysymtabCommand)
- `ilocalsym: crate::endian::U32<E>` - index to local symbols
- `nlocalsym: crate::endian::U32<E>` - number of local symbols
- `iextdefsym: crate::endian::U32<E>` - index to externally defined symbols
- `nextdefsym: crate::endian::U32<E>` - number of externally defined symbols
- `iundefsym: crate::endian::U32<E>` - index to undefined symbols
- `nundefsym: crate::endian::U32<E>` - number of undefined symbols
- `tocoff: crate::endian::U32<E>` - file offset to table of contents
- `ntoc: crate::endian::U32<E>` - number of entries in table of contents
- `modtaboff: crate::endian::U32<E>` - file offset to module table
- `nmodtab: crate::endian::U32<E>` - number of module table entries
- `extrefsymoff: crate::endian::U32<E>` - offset to referenced symbol table
- `nextrefsyms: crate::endian::U32<E>` - number of referenced symbol table entries
- `indirectsymoff: crate::endian::U32<E>` - file offset to the indirect symbol table
- `nindirectsyms: crate::endian::U32<E>` - number of indirect symbol table entries
- `extreloff: crate::endian::U32<E>` - offset to external relocation entries
- `nextrel: crate::endian::U32<E>` - number of external relocation entries
- `locreloff: crate::endian::U32<E>` - offset to local relocation entries
- `nlocrel: crate::endian::U32<E>` - number of local relocation entries

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DysymtabCommand<E>`



## object::macho::EXECUTABLE_ORDINAL

*Constant*: `u8`



## object::macho::EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE

*Constant*: `u8`



## object::macho::EXPORT_SYMBOL_FLAGS_KIND_MASK

*Constant*: `u8`



## object::macho::EXPORT_SYMBOL_FLAGS_KIND_REGULAR

*Constant*: `u8`



## object::macho::EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL

*Constant*: `u8`



## object::macho::EXPORT_SYMBOL_FLAGS_REEXPORT

*Constant*: `u8`



## object::macho::EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER

*Constant*: `u8`



## object::macho::EXPORT_SYMBOL_FLAGS_WEAK_DEFINITION

*Constant*: `u8`



## object::macho::EncryptionInfoCommand32

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_ENCRYPTION_INFO
- `cmdsize: crate::endian::U32<E>` - sizeof(struct EncryptionInfoCommand32)
- `cryptoff: crate::endian::U32<E>` - file offset of encrypted range
- `cryptsize: crate::endian::U32<E>` - file size of encrypted range
- `cryptid: crate::endian::U32<E>` - which enryption system, 0 means not-encrypted yet

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> EncryptionInfoCommand32<E>`



## object::macho::EncryptionInfoCommand64

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_ENCRYPTION_INFO_64
- `cmdsize: crate::endian::U32<E>` - sizeof(struct EncryptionInfoCommand64)
- `cryptoff: crate::endian::U32<E>` - file offset of encrypted range
- `cryptsize: crate::endian::U32<E>` - file size of encrypted range
- `cryptid: crate::endian::U32<E>` - which enryption system, 0 means not-encrypted yet
- `pad: crate::endian::U32<E>` - padding to make this struct's size a multiple of 8 bytes

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> EncryptionInfoCommand64<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::EntryPointCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_MAIN only used in MH_EXECUTE filetypes
- `cmdsize: crate::endian::U32<E>` - 24
- `entryoff: crate::endian::U64<E>` - file (__TEXT) offset of main()
- `stacksize: crate::endian::U64<E>` - if not zero, initial stack size

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> EntryPointCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::FAT_CIGAM

*Constant*: `u32`

NXSwapLong(FAT_MAGIC)



## object::macho::FAT_CIGAM_64

*Constant*: `u32`

NXSwapLong(FAT_MAGIC_64)



## object::macho::FAT_MAGIC

*Constant*: `u32`



## object::macho::FAT_MAGIC_64

*Constant*: `u32`



## object::macho::FatArch32

*Struct*

**Fields:**
- `cputype: crate::endian::U32<crate::endian::BigEndian>` - cpu specifier (int)
- `cpusubtype: crate::endian::U32<crate::endian::BigEndian>` - machine specifier (int)
- `offset: crate::endian::U32<crate::endian::BigEndian>` - file offset to this object file
- `size: crate::endian::U32<crate::endian::BigEndian>` - size of this object file
- `align: crate::endian::U32<crate::endian::BigEndian>` - alignment as a power of 2

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FatArch32`
- **FatArch**
  - `fn cputype(self: &Self) -> u32`
  - `fn cpusubtype(self: &Self) -> u32`
  - `fn offset(self: &Self) -> <Self as >::Word`
  - `fn size(self: &Self) -> <Self as >::Word`
  - `fn align(self: &Self) -> u32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::FatArch64

*Struct*

**Fields:**
- `cputype: crate::endian::U32<crate::endian::BigEndian>` - cpu specifier (int)
- `cpusubtype: crate::endian::U32<crate::endian::BigEndian>` - machine specifier (int)
- `offset: crate::endian::U64<crate::endian::BigEndian>` - file offset to this object file
- `size: crate::endian::U64<crate::endian::BigEndian>` - size of this object file
- `align: crate::endian::U32<crate::endian::BigEndian>` - alignment as a power of 2
- `reserved: crate::endian::U32<crate::endian::BigEndian>` - reserved

**Traits:** Pod, Copy

**Trait Implementations:**

- **FatArch**
  - `fn cputype(self: &Self) -> u32`
  - `fn cpusubtype(self: &Self) -> u32`
  - `fn offset(self: &Self) -> <Self as >::Word`
  - `fn size(self: &Self) -> <Self as >::Word`
  - `fn align(self: &Self) -> u32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FatArch64`



## object::macho::FatHeader

*Struct*

**Fields:**
- `magic: crate::endian::U32<crate::endian::BigEndian>` - FAT_MAGIC or FAT_MAGIC_64
- `nfat_arch: crate::endian::U32<crate::endian::BigEndian>` - number of structs that follow

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FatHeader`



## object::macho::FilesetEntryCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>`
- `cmdsize: crate::endian::U32<E>` - includes id string
- `vmaddr: crate::endian::U64<E>` - memory address of the dylib
- `fileoff: crate::endian::U64<E>` - file offset of the dylib
- `entry_id: LcStr<E>` - contained entry id
- `reserved: crate::endian::U32<E>` - entry_id is 32-bits long, so this is the reserved padding

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FilesetEntryCommand<E>`



## object::macho::FvmfileCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_FVMFILE
- `cmdsize: crate::endian::U32<E>` - includes pathname string
- `name: LcStr<E>` - files pathname
- `header_addr: crate::endian::U32<E>` - files virtual address

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FvmfileCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::Fvmlib

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `name: LcStr<E>` - library's target pathname
- `minor_version: crate::endian::U32<E>` - library's minor version number
- `header_addr: crate::endian::U32<E>` - library's header address

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Fvmlib<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::FvmlibCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_IDFVMLIB or LC_LOADFVMLIB
- `cmdsize: crate::endian::U32<E>` - includes pathname string
- `fvmlib: Fvmlib<E>` - the library identification

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FvmlibCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::GENERIC_RELOC_LOCAL_SECTDIFF

*Constant*: `u8`



## object::macho::GENERIC_RELOC_PAIR

*Constant*: `u8`

Only follows a GENERIC_RELOC_SECTDIFF



## object::macho::GENERIC_RELOC_PB_LA_PTR

*Constant*: `u8`

prebound lazy pointer



## object::macho::GENERIC_RELOC_SECTDIFF

*Constant*: `u8`



## object::macho::GENERIC_RELOC_TLV

*Constant*: `u8`

thread local variables



## object::macho::GENERIC_RELOC_VANILLA

*Constant*: `u8`

generic relocation as described above



## object::macho::INDIRECT_SYMBOL_ABS

*Constant*: `u32`



## object::macho::INDIRECT_SYMBOL_LOCAL

*Constant*: `u32`



## object::macho::IdentCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_IDENT
- `cmdsize: crate::endian::U32<E>` - strings that follow this command

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> IdentCommand<E>`



## object::macho::LC_BUILD_VERSION

*Constant*: `u32`

build for platform min OS version



## object::macho::LC_CODE_SIGNATURE

*Constant*: `u32`

local of code signature



## object::macho::LC_DATA_IN_CODE

*Constant*: `u32`

table of non-instructions in __text



## object::macho::LC_DYLD_CHAINED_FIXUPS

*Constant*: `u32`

used with `LinkeditDataCommand`



## object::macho::LC_DYLD_ENVIRONMENT

*Constant*: `u32`

string for dyld to treat like environment variable



## object::macho::LC_DYLD_EXPORTS_TRIE

*Constant*: `u32`

used with `LinkeditDataCommand`, payload is trie



## object::macho::LC_DYLD_INFO

*Constant*: `u32`

compressed dyld information



## object::macho::LC_DYLD_INFO_ONLY

*Constant*: `u32`

compressed dyld information only



## object::macho::LC_DYLIB_CODE_SIGN_DRS

*Constant*: `u32`

Code signing DRs copied from linked dylibs



## object::macho::LC_DYSYMTAB

*Constant*: `u32`

dynamic link-edit symbol table info



## object::macho::LC_ENCRYPTION_INFO

*Constant*: `u32`

encrypted segment information



## object::macho::LC_ENCRYPTION_INFO_64

*Constant*: `u32`

64-bit encrypted segment information



## object::macho::LC_FILESET_ENTRY

*Constant*: `u32`

used with `FilesetEntryCommand`



## object::macho::LC_FUNCTION_STARTS

*Constant*: `u32`

compressed table of function start addresses



## object::macho::LC_FVMFILE

*Constant*: `u32`

fixed VM file inclusion (internal use)



## object::macho::LC_IDENT

*Constant*: `u32`

object identification info (obsolete)



## object::macho::LC_IDFVMLIB

*Constant*: `u32`

fixed VM shared library identification



## object::macho::LC_ID_DYLIB

*Constant*: `u32`

dynamically linked shared lib ident



## object::macho::LC_ID_DYLINKER

*Constant*: `u32`

dynamic linker identification



## object::macho::LC_LAZY_LOAD_DYLIB

*Constant*: `u32`

delay load of dylib until first use



## object::macho::LC_LINKER_OPTIMIZATION_HINT

*Constant*: `u32`

optimization hints in MH_OBJECT files



## object::macho::LC_LINKER_OPTION

*Constant*: `u32`

linker options in MH_OBJECT files



## object::macho::LC_LOADFVMLIB

*Constant*: `u32`

load a specified fixed VM shared library



## object::macho::LC_LOAD_DYLIB

*Constant*: `u32`

load a dynamically linked shared library



## object::macho::LC_LOAD_DYLINKER

*Constant*: `u32`

load a dynamic linker



## object::macho::LC_LOAD_UPWARD_DYLIB

*Constant*: `u32`

load upward dylib



## object::macho::LC_LOAD_WEAK_DYLIB

*Constant*: `u32`

load a dynamically linked shared library that is allowed to be missing
(all symbols are weak imported).



## object::macho::LC_MAIN

*Constant*: `u32`

replacement for LC_UNIXTHREAD



## object::macho::LC_NOTE

*Constant*: `u32`

arbitrary data included within a Mach-O file



## object::macho::LC_PREBIND_CKSUM

*Constant*: `u32`

prebind checksum



## object::macho::LC_PREBOUND_DYLIB

*Constant*: `u32`

modules prebound for a dynamically linked shared library



## object::macho::LC_PREPAGE

*Constant*: `u32`

prepage command (internal use)



## object::macho::LC_REEXPORT_DYLIB

*Constant*: `u32`

load and re-export dylib



## object::macho::LC_REQ_DYLD

*Constant*: `u32`



## object::macho::LC_ROUTINES

*Constant*: `u32`

image routines



## object::macho::LC_ROUTINES_64

*Constant*: `u32`

64-bit image routines



## object::macho::LC_RPATH

*Constant*: `u32`

runpath additions



## object::macho::LC_SEGMENT

*Constant*: `u32`

segment of this file to be mapped



## object::macho::LC_SEGMENT_64

*Constant*: `u32`

64-bit segment of this file to be mapped



## object::macho::LC_SEGMENT_SPLIT_INFO

*Constant*: `u32`

local of info to split segments



## object::macho::LC_SOURCE_VERSION

*Constant*: `u32`

source version used to build binary



## object::macho::LC_SUB_CLIENT

*Constant*: `u32`

sub client



## object::macho::LC_SUB_FRAMEWORK

*Constant*: `u32`

sub framework



## object::macho::LC_SUB_LIBRARY

*Constant*: `u32`

sub library



## object::macho::LC_SUB_UMBRELLA

*Constant*: `u32`

sub umbrella



## object::macho::LC_SYMSEG

*Constant*: `u32`

link-edit gdb symbol table info (obsolete)



## object::macho::LC_SYMTAB

*Constant*: `u32`

link-edit stab symbol table info



## object::macho::LC_THREAD

*Constant*: `u32`

thread



## object::macho::LC_TWOLEVEL_HINTS

*Constant*: `u32`

two-level namespace lookup hints



## object::macho::LC_UNIXTHREAD

*Constant*: `u32`

unix thread (includes a stack)



## object::macho::LC_UUID

*Constant*: `u32`

the uuid



## object::macho::LC_VERSION_MIN_IPHONEOS

*Constant*: `u32`

build for iPhoneOS min OS version



## object::macho::LC_VERSION_MIN_MACOSX

*Constant*: `u32`

build for MacOSX min OS version



## object::macho::LC_VERSION_MIN_TVOS

*Constant*: `u32`

build for AppleTV min OS version



## object::macho::LC_VERSION_MIN_WATCHOS

*Constant*: `u32`

build for Watch min OS version



## object::macho::LcStr

*Struct*

A variable length string in a load command.

The strings are stored just after the load command structure and
the offset is from the start of the load command structure.  The size
of the string is reflected in the `cmdsize` field of the load command.
Once again any padded bytes to bring the `cmdsize` field to a multiple
of 4 bytes must be zero.

**Generic Parameters:**
- E

**Fields:**
- `offset: crate::endian::U32<E>` - offset to the string

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LcStr<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::LinkeditDataCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - `LC_CODE_SIGNATURE`, `LC_SEGMENT_SPLIT_INFO`, `LC_FUNCTION_STARTS`,
- `cmdsize: crate::endian::U32<E>` - sizeof(struct LinkeditDataCommand)
- `dataoff: crate::endian::U32<E>` - file offset of data in __LINKEDIT segment
- `datasize: crate::endian::U32<E>` - file size of data in __LINKEDIT segment

**Methods:**

- `fn function_starts<'data, R>(self: &Self, endian: E, data: R, text_segment_addr: u64) -> Result<FunctionStartsIterator<'data>>` - Return an iterator over the function start addresses.
- `fn exports_trie<'data, R>(self: &Self, endian: E, data: R) -> Result<ExportsTrieIterator<'data>>` - Return an iterator over the exports trie.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> LinkeditDataCommand<E>`



## object::macho::LinkerOptionCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_LINKER_OPTION only used in MH_OBJECT filetypes
- `cmdsize: crate::endian::U32<E>`
- `count: crate::endian::U32<E>` - number of strings

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LinkerOptionCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::LoadCommand

*Struct*

Common fields at the start of every load command.

The load commands directly follow the mach_header.  The total size of all
of the commands is given by the sizeofcmds field in the mach_header.  All
load commands must have as their first two fields `cmd` and `cmdsize`.  The `cmd`
field is filled in with a constant for that command type.  Each command type
has a structure specifically for it.  The `cmdsize` field is the size in bytes
of the particular load command structure plus anything that follows it that
is a part of the load command (i.e. section structures, strings, etc.).  To
advance to the next load command the `cmdsize` can be added to the offset or
pointer of the current load command.  The `cmdsize` for 32-bit architectures
MUST be a multiple of 4 bytes and for 64-bit architectures MUST be a multiple
of 8 bytes (these are forever the maximum alignment of any load commands).
The padded bytes must be zero.  All tables in the object file must also
follow these rules so the file can be memory mapped.  Otherwise the pointers
to these tables will not work well or at all on some machines.  With all
padding zeroed like objects will compare byte for byte.

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - Type of load command.
- `cmdsize: crate::endian::U32<E>` - Total size of command in bytes.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> LoadCommand<E>`



## object::macho::MAX_LIBRARY_ORDINAL

*Constant*: `u8`



## object::macho::MAX_SECT

*Constant*: `u8`

1 thru 255 inclusive



## object::macho::MH_ALLMODSBOUND

*Constant*: `u32`

indicates that this binary binds to all two-level namespace modules of its dependent libraries. only used when MH_PREBINDABLE and MH_TWOLEVEL are both set.



## object::macho::MH_ALLOW_STACK_EXECUTION

*Constant*: `u32`

When this bit is set, all stacks in the task will be given stack execution privilege.  Only used in MH_EXECUTE filetypes.



## object::macho::MH_APP_EXTENSION_SAFE

*Constant*: `u32`

The code was linked for use in an application extension.



## object::macho::MH_BINDATLOAD

*Constant*: `u32`

the object file's undefined references are bound by the dynamic linker when loaded.



## object::macho::MH_BINDS_TO_WEAK

*Constant*: `u32`

the final linked image uses weak symbols



## object::macho::MH_BUNDLE

*Constant*: `u32`

dynamically bound bundle file



## object::macho::MH_CANONICAL

*Constant*: `u32`

the binary has been canonicalized via the unprebind operation



## object::macho::MH_CIGAM

*Constant*: `u32`

NXSwapInt(MH_MAGIC)



## object::macho::MH_CIGAM_64

*Constant*: `u32`

NXSwapInt(MH_MAGIC_64)



## object::macho::MH_CORE

*Constant*: `u32`

core file



## object::macho::MH_DEAD_STRIPPABLE_DYLIB

*Constant*: `u32`

Only for use on dylibs.  When linking against a dylib that has this bit set, the static linker will automatically not create a LC_LOAD_DYLIB load command to the dylib if no symbols are being referenced from the dylib.



## object::macho::MH_DSYM

*Constant*: `u32`

companion file with only debug sections



## object::macho::MH_DYLDLINK

*Constant*: `u32`

the object file is input for the dynamic linker and can't be statically link edited again



## object::macho::MH_DYLIB

*Constant*: `u32`

dynamically bound shared library



## object::macho::MH_DYLIB_IN_CACHE

*Constant*: `u32`

Only for use on dylibs. When this bit is set, the dylib is part of the dyld
shared cache, rather than loose in the filesystem.



## object::macho::MH_DYLIB_STUB

*Constant*: `u32`

shared library stub for static linking only, no section contents



## object::macho::MH_DYLINKER

*Constant*: `u32`

dynamic link editor



## object::macho::MH_EXECUTE

*Constant*: `u32`

demand paged executable file



## object::macho::MH_FILESET

*Constant*: `u32`

set of mach-o's



## object::macho::MH_FORCE_FLAT

*Constant*: `u32`

the executable is forcing all images to use flat name space bindings



## object::macho::MH_FVMLIB

*Constant*: `u32`

fixed VM shared library file



## object::macho::MH_HAS_TLV_DESCRIPTORS

*Constant*: `u32`

Contains a section of type S_THREAD_LOCAL_VARIABLES



## object::macho::MH_INCRLINK

*Constant*: `u32`

the object file is the output of an incremental link against a base file and can't be link edited again



## object::macho::MH_KEXT_BUNDLE

*Constant*: `u32`

x86_64 kexts



## object::macho::MH_LAZY_INIT

*Constant*: `u32`

the shared library init routine is to be run lazily via catching memory faults to its writeable segments (obsolete)



## object::macho::MH_MAGIC

*Constant*: `u32`

the mach magic number



## object::macho::MH_MAGIC_64

*Constant*: `u32`

the 64-bit mach magic number



## object::macho::MH_NLIST_OUTOFSYNC_WITH_DYLDINFO

*Constant*: `u32`

The external symbols listed in the nlist symbol table do not include all the symbols listed in the dyld info.



## object::macho::MH_NOFIXPREBINDING

*Constant*: `u32`

do not have dyld notify the prebinding agent about this executable



## object::macho::MH_NOMULTIDEFS

*Constant*: `u32`

this umbrella guarantees no multiple definitions of symbols in its sub-images so the two-level namespace hints can always be used.



## object::macho::MH_NOUNDEFS

*Constant*: `u32`

the object file has no undefined references



## object::macho::MH_NO_HEAP_EXECUTION

*Constant*: `u32`

When this bit is set, the OS will run the main executable with a non-executable heap even on platforms (e.g. i386) that don't require it. Only used in MH_EXECUTE filetypes.



## object::macho::MH_NO_REEXPORTED_DYLIBS

*Constant*: `u32`

When this bit is set on a dylib, the static linker does not need to examine dependent dylibs to see if any are re-exported



## object::macho::MH_OBJECT

*Constant*: `u32`

relocatable object file



## object::macho::MH_PIE

*Constant*: `u32`

When this bit is set, the OS will load the main executable at a random address.  Only used in MH_EXECUTE filetypes.



## object::macho::MH_PREBINDABLE

*Constant*: `u32`

the binary is not prebound but can have its prebinding redone. only used when MH_PREBOUND is not set.



## object::macho::MH_PREBOUND

*Constant*: `u32`

the file has its dynamic undefined references prebound.



## object::macho::MH_PRELOAD

*Constant*: `u32`

preloaded executable file



## object::macho::MH_ROOT_SAFE

*Constant*: `u32`

When this bit is set, the binary declares it is safe for use in processes with uid zero



## object::macho::MH_SETUID_SAFE

*Constant*: `u32`

When this bit is set, the binary declares it is safe for use in processes when issetugid() is true



## object::macho::MH_SIM_SUPPORT

*Constant*: `u32`

Allow LC_MIN_VERSION_MACOS and LC_BUILD_VERSION load commands with
the platforms macOS, iOSMac, iOSSimulator, tvOSSimulator and watchOSSimulator.



## object::macho::MH_SPLIT_SEGS

*Constant*: `u32`

the file has its read-only and read-write segments split



## object::macho::MH_SUBSECTIONS_VIA_SYMBOLS

*Constant*: `u32`

safe to divide up the sections into sub-sections via symbols for dead code stripping



## object::macho::MH_TWOLEVEL

*Constant*: `u32`

the image is using two-level name space bindings



## object::macho::MH_WEAK_DEFINES

*Constant*: `u32`

the final linked image contains external weak symbols



## object::macho::MachHeader32

*Struct*

The 32-bit mach header.

Appears at the very beginning of the object file for 32-bit architectures.

**Generic Parameters:**
- E

**Fields:**
- `magic: crate::endian::U32<crate::endian::BigEndian>` - mach magic number identifier
- `cputype: crate::endian::U32<E>` - cpu specifier
- `cpusubtype: crate::endian::U32<E>` - machine specifier
- `filetype: crate::endian::U32<E>` - type of file
- `ncmds: crate::endian::U32<E>` - number of load commands
- `sizeofcmds: crate::endian::U32<E>` - the size of all the load commands
- `flags: crate::endian::U32<E>` - flags

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MachHeader32<E>`
- **MachHeader**
  - `fn is_type_64(self: &Self) -> bool`
  - `fn is_big_endian(self: &Self) -> bool`
  - `fn is_little_endian(self: &Self) -> bool`
  - `fn magic(self: &Self) -> u32`
  - `fn cputype(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn cpusubtype(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn filetype(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn ncmds(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn sizeofcmds(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`



## object::macho::MachHeader64

*Struct*

The 64-bit mach header.

Appears at the very beginning of object files for 64-bit architectures.

**Generic Parameters:**
- E

**Fields:**
- `magic: crate::endian::U32<crate::endian::BigEndian>` - mach magic number identifier
- `cputype: crate::endian::U32<E>` - cpu specifier
- `cpusubtype: crate::endian::U32<E>` - machine specifier
- `filetype: crate::endian::U32<E>` - type of file
- `ncmds: crate::endian::U32<E>` - number of load commands
- `sizeofcmds: crate::endian::U32<E>` - the size of all the load commands
- `flags: crate::endian::U32<E>` - flags
- `reserved: crate::endian::U32<E>` - reserved

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> MachHeader64<E>`
- **MachHeader**
  - `fn is_type_64(self: &Self) -> bool`
  - `fn is_big_endian(self: &Self) -> bool`
  - `fn is_little_endian(self: &Self) -> bool`
  - `fn magic(self: &Self) -> u32`
  - `fn cputype(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn cpusubtype(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn filetype(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn ncmds(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn sizeofcmds(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::NO_SECT

*Constant*: `u8`

symbol is not in any section



## object::macho::N_ABS

*Constant*: `u8`

absolute, n_sect == NO_SECT



## object::macho::N_ALT_ENTRY

*Constant*: `u16`



## object::macho::N_ARM_THUMB_DEF

*Constant*: `u16`

symbol is a Thumb function (ARM)



## object::macho::N_AST

*Constant*: `u8`

AST file path: name,,NO_SECT,0,0



## object::macho::N_BCOMM

*Constant*: `u8`

begin common: name,,NO_SECT,0,0



## object::macho::N_BINCL

*Constant*: `u8`

include file beginning: name,,NO_SECT,0,sum



## object::macho::N_BNSYM

*Constant*: `u8`

begin nsect sym: 0,,n_sect,0,address



## object::macho::N_DESC_DISCARDED

*Constant*: `u16`

symbol is discarded



## object::macho::N_ECOML

*Constant*: `u8`

end common (local name): 0,,n_sect,0,address



## object::macho::N_ECOMM

*Constant*: `u8`

end common: name,,n_sect,0,0



## object::macho::N_EINCL

*Constant*: `u8`

include file end: name,,NO_SECT,0,0



## object::macho::N_ENSYM

*Constant*: `u8`

end nsect sym: 0,,n_sect,0,address



## object::macho::N_ENTRY

*Constant*: `u8`

alternate entry: name,,n_sect,linenumber,address



## object::macho::N_EXCL

*Constant*: `u8`

deleted include file: name,,NO_SECT,0,sum



## object::macho::N_EXT

*Constant*: `u8`

external symbol bit, set for external symbols



## object::macho::N_FNAME

*Constant*: `u8`

procedure name (f77 kludge): name,,NO_SECT,0,0



## object::macho::N_FUN

*Constant*: `u8`

procedure: name,,n_sect,linenumber,address



## object::macho::N_GSYM

*Constant*: `u8`

global symbol: name,,NO_SECT,type,0



## object::macho::N_INDR

*Constant*: `u8`

indirect



## object::macho::N_LBRAC

*Constant*: `u8`

left bracket: 0,,NO_SECT,nesting level,address



## object::macho::N_LCSYM

*Constant*: `u8`

.lcomm symbol: name,,n_sect,type,address



## object::macho::N_LENG

*Constant*: `u8`

second stab entry with length information



## object::macho::N_LSYM

*Constant*: `u8`

local sym: name,,NO_SECT,type,offset



## object::macho::N_NO_DEAD_STRIP

*Constant*: `u16`

symbol is not to be dead stripped



## object::macho::N_OLEVEL

*Constant*: `u8`

compiler -O level: name,,NO_SECT,0,0



## object::macho::N_OPT

*Constant*: `u8`

emitted with gcc2_compiled and in gcc source



## object::macho::N_OSO

*Constant*: `u8`

object file name: name,,0,0,st_mtime



## object::macho::N_PARAMS

*Constant*: `u8`

compiler parameters: name,,NO_SECT,0,0



## object::macho::N_PBUD

*Constant*: `u8`

prebound undefined (defined in a dylib)



## object::macho::N_PC

*Constant*: `u8`

global pascal symbol: name,,NO_SECT,subtype,line



## object::macho::N_PEXT

*Constant*: `u8`

private external symbol bit



## object::macho::N_PSYM

*Constant*: `u8`

parameter: name,,NO_SECT,type,offset



## object::macho::N_RBRAC

*Constant*: `u8`

right bracket: 0,,NO_SECT,nesting level,address



## object::macho::N_REF_TO_WEAK

*Constant*: `u16`

reference to a weak symbol



## object::macho::N_RSYM

*Constant*: `u8`

register sym: name,,NO_SECT,type,register



## object::macho::N_SECT

*Constant*: `u8`

defined in section number n_sect



## object::macho::N_SLINE

*Constant*: `u8`

src line: 0,,n_sect,linenumber,address



## object::macho::N_SO

*Constant*: `u8`

source file name: name,,n_sect,0,address



## object::macho::N_SOL

*Constant*: `u8`

#included file name: name,,n_sect,0,address



## object::macho::N_SSYM

*Constant*: `u8`

structure elt: name,,NO_SECT,type,struct_offset



## object::macho::N_STAB

*Constant*: `u8`

if any of these bits set, a symbolic debugging entry



## object::macho::N_STSYM

*Constant*: `u8`

static symbol: name,,n_sect,type,address



## object::macho::N_SYMBOL_RESOLVER

*Constant*: `u16`



## object::macho::N_TYPE

*Constant*: `u8`

mask for the type bits



## object::macho::N_UNDF

*Constant*: `u8`

undefined, n_sect == NO_SECT



## object::macho::N_VERSION

*Constant*: `u8`

compiler version: name,,NO_SECT,0,0



## object::macho::N_WEAK_DEF

*Constant*: `u16`

coalesced symbol is a weak definition



## object::macho::N_WEAK_REF

*Constant*: `u16`

symbol is weak referenced



## object::macho::Nlist32

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `n_strx: crate::endian::U32<E>` - index into the string table
- `n_type: u8` - type flag, see below
- `n_sect: u8` - section number or NO_SECT
- `n_desc: crate::endian::U16<E>` - see <mach-o/stab.h>
- `n_value: crate::endian::U32<E>` - value of this symbol (or stab offset)

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Nlist**
  - `fn n_strx(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn n_type(self: &Self) -> u8`
  - `fn n_sect(self: &Self) -> u8`
  - `fn n_desc(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn n_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
- **Clone**
  - `fn clone(self: &Self) -> Nlist32<E>`



## object::macho::Nlist64

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `n_strx: crate::endian::U32<E>` - index into the string table
- `n_type: u8` - type flag, see below
- `n_sect: u8` - section number or NO_SECT
- `n_desc: crate::endian::U16<E>` - see <mach-o/stab.h>
- `n_value: crate::endian::U64Bytes<E>` - value of this symbol (or stab offset)

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Nlist64<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Nlist**
  - `fn n_strx(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn n_type(self: &Self) -> u8`
  - `fn n_sect(self: &Self) -> u8`
  - `fn n_desc(self: &Self, endian: <Self as >::Endian) -> u16`
  - `fn n_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`



## object::macho::NoteCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_NOTE
- `cmdsize: crate::endian::U32<E>` - sizeof(struct NoteCommand)
- `data_owner: [u8; 16]` - owner name for this LC_NOTE
- `offset: crate::endian::U64<E>` - file offset of this data
- `size: crate::endian::U64<E>` - length of data region

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> NoteCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::PLATFORM_BRIDGEOS

*Constant*: `u32`



## object::macho::PLATFORM_DRIVERKIT

*Constant*: `u32`



## object::macho::PLATFORM_IOS

*Constant*: `u32`



## object::macho::PLATFORM_IOSSIMULATOR

*Constant*: `u32`



## object::macho::PLATFORM_MACCATALYST

*Constant*: `u32`



## object::macho::PLATFORM_MACOS

*Constant*: `u32`



## object::macho::PLATFORM_TVOS

*Constant*: `u32`



## object::macho::PLATFORM_TVOSSIMULATOR

*Constant*: `u32`



## object::macho::PLATFORM_WATCHOS

*Constant*: `u32`



## object::macho::PLATFORM_WATCHOSSIMULATOR

*Constant*: `u32`



## object::macho::PLATFORM_XROS

*Constant*: `u32`



## object::macho::PLATFORM_XROSSIMULATOR

*Constant*: `u32`



## object::macho::PPC_RELOC_BR14

*Constant*: `u8`

14 bit branch displacement (to a word address)



## object::macho::PPC_RELOC_BR24

*Constant*: `u8`

24 bit branch displacement (to a word address)



## object::macho::PPC_RELOC_HA16

*Constant*: `u8`

Same as the RELOC_HI16 except the low 16 bits and the high 16 bits are added together
with the low 16 bits sign extended first.  This means if bit 15 of the low 16 bits is
set the high 16 bits stored in the instruction will be adjusted.



## object::macho::PPC_RELOC_HA16_SECTDIFF

*Constant*: `u8`



## object::macho::PPC_RELOC_HI16

*Constant*: `u8`

a PAIR follows with the low half



## object::macho::PPC_RELOC_HI16_SECTDIFF

*Constant*: `u8`

section difference forms of above.  a PAIR



## object::macho::PPC_RELOC_JBSR

*Constant*: `u8`



## object::macho::PPC_RELOC_LO14

*Constant*: `u8`

Same as the LO16 except that the low 2 bits are not stored in the instruction and are
always zero.  This is used in double word load/store instructions.



## object::macho::PPC_RELOC_LO14_SECTDIFF

*Constant*: `u8`



## object::macho::PPC_RELOC_LO16

*Constant*: `u8`

a PAIR follows with the high half



## object::macho::PPC_RELOC_LO16_SECTDIFF

*Constant*: `u8`

follows these with subtract symbol value



## object::macho::PPC_RELOC_LOCAL_SECTDIFF

*Constant*: `u8`

like PPC_RELOC_SECTDIFF, but the symbol referenced was local.



## object::macho::PPC_RELOC_PAIR

*Constant*: `u8`

the second relocation entry of a pair



## object::macho::PPC_RELOC_PB_LA_PTR

*Constant*: `u8`

prebound lazy pointer



## object::macho::PPC_RELOC_SECTDIFF

*Constant*: `u8`

a PAIR follows with subtract symbol value



## object::macho::PPC_RELOC_VANILLA

*Constant*: `u8`

generic relocation as described above



## object::macho::PrebindCksumCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_PREBIND_CKSUM
- `cmdsize: crate::endian::U32<E>` - sizeof(struct PrebindCksumCommand)
- `cksum: crate::endian::U32<E>` - the check sum or zero

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PrebindCksumCommand<E>`



## object::macho::PreboundDylibCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_PREBOUND_DYLIB
- `cmdsize: crate::endian::U32<E>` - includes strings
- `name: LcStr<E>` - library's path name
- `nmodules: crate::endian::U32<E>` - number of modules in library
- `linked_modules: LcStr<E>` - bit vector of linked modules

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PreboundDylibCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::PtrauthKey

*Enum*

The key used to sign a pointer for authentication.

The variant values correspond to the values used in the
`ptrauth_key` enum in `ptrauth.h`.

**Variants:**
- `IA` - Instruction key A.
- `IB` - Instruction key B.
- `DA` - Data key A.
- `DB` - Data key B.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PtrauthKey) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> PtrauthKey`



## object::macho::REBASE_IMMEDIATE_MASK

*Constant*: `u8`



## object::macho::REBASE_OPCODE_ADD_ADDR_IMM_SCALED

*Constant*: `u8`



## object::macho::REBASE_OPCODE_ADD_ADDR_ULEB

*Constant*: `u8`



## object::macho::REBASE_OPCODE_DONE

*Constant*: `u8`



## object::macho::REBASE_OPCODE_DO_REBASE_ADD_ADDR_ULEB

*Constant*: `u8`



## object::macho::REBASE_OPCODE_DO_REBASE_IMM_TIMES

*Constant*: `u8`



## object::macho::REBASE_OPCODE_DO_REBASE_ULEB_TIMES

*Constant*: `u8`



## object::macho::REBASE_OPCODE_DO_REBASE_ULEB_TIMES_SKIPPING_ULEB

*Constant*: `u8`



## object::macho::REBASE_OPCODE_MASK

*Constant*: `u8`



## object::macho::REBASE_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB

*Constant*: `u8`



## object::macho::REBASE_OPCODE_SET_TYPE_IMM

*Constant*: `u8`



## object::macho::REBASE_TYPE_POINTER

*Constant*: `u8`



## object::macho::REBASE_TYPE_TEXT_ABSOLUTE32

*Constant*: `u8`



## object::macho::REBASE_TYPE_TEXT_PCREL32

*Constant*: `u8`



## object::macho::REFERENCED_DYNAMICALLY

*Constant*: `u16`



## object::macho::REFERENCE_FLAG_DEFINED

*Constant*: `u16`



## object::macho::REFERENCE_FLAG_PRIVATE_DEFINED

*Constant*: `u16`



## object::macho::REFERENCE_FLAG_PRIVATE_UNDEFINED_LAZY

*Constant*: `u16`



## object::macho::REFERENCE_FLAG_PRIVATE_UNDEFINED_NON_LAZY

*Constant*: `u16`



## object::macho::REFERENCE_FLAG_UNDEFINED_LAZY

*Constant*: `u16`



## object::macho::REFERENCE_FLAG_UNDEFINED_NON_LAZY

*Constant*: `u16`



## object::macho::REFERENCE_TYPE

*Constant*: `u16`



## object::macho::R_ABS

*Constant*: `u8`

absolute relocation type for Mach-O files



## object::macho::R_SCATTERED

*Constant*: `u32`

Bit set in `Relocation::r_word0` for scattered relocations.



## object::macho::Relocation

*Struct*

A relocation entry.

Mach-O relocations have plain and scattered variants, with the
meaning of the fields depending on the variant.

This type provides functions for determining whether the relocation
is scattered, and for accessing the fields of each variant.

**Generic Parameters:**
- E

**Fields:**
- `r_word0: crate::endian::U32<E>`
- `r_word1: crate::endian::U32<E>`

**Methods:**

- `fn r_scattered(self: Self, endian: E, cputype: u32) -> bool` - Determine whether this is a scattered relocation.
- `fn info(self: Self, endian: E) -> RelocationInfo` - Return the fields of a plain relocation.
- `fn scattered_info(self: Self, endian: E) -> ScatteredRelocationInfo` - Return the fields of a scattered relocation.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Relocation<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::RelocationInfo

*Struct*

**Fields:**
- `r_address: u32` - offset in the section to what is being relocated
- `r_symbolnum: u32` - symbol index if r_extern == 1 or section ordinal if r_extern == 0
- `r_pcrel: bool` - was relocated pc relative already
- `r_length: u8` - 0=byte, 1=word, 2=long, 3=quad
- `r_extern: bool` - does not include value of sym referenced
- `r_type: u8` - if not 0, machine specific relocation type

**Methods:**

- `fn relocation<E>(self: Self, endian: E) -> Relocation<E>` - Combine the fields into a `Relocation`.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RelocationInfo`



## object::macho::RoutinesCommand32

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_ROUTINES
- `cmdsize: crate::endian::U32<E>` - total size of this command
- `init_address: crate::endian::U32<E>` - address of initialization routine
- `init_module: crate::endian::U32<E>` - index into the module table that the init routine is defined in
- `reserved1: crate::endian::U32<E>`
- `reserved2: crate::endian::U32<E>`
- `reserved3: crate::endian::U32<E>`
- `reserved4: crate::endian::U32<E>`
- `reserved5: crate::endian::U32<E>`
- `reserved6: crate::endian::U32<E>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RoutinesCommand32<E>`



## object::macho::RoutinesCommand64

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_ROUTINES_64
- `cmdsize: crate::endian::U32<E>` - total size of this command
- `init_address: crate::endian::U64<E>` - address of initialization routine
- `init_module: crate::endian::U64<E>` - index into the module table that the init routine is defined in
- `reserved1: crate::endian::U64<E>`
- `reserved2: crate::endian::U64<E>`
- `reserved3: crate::endian::U64<E>`
- `reserved4: crate::endian::U64<E>`
- `reserved5: crate::endian::U64<E>`
- `reserved6: crate::endian::U64<E>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RoutinesCommand64<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::RpathCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_RPATH
- `cmdsize: crate::endian::U32<E>` - includes string
- `path: LcStr<E>` - path to add to run path

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RpathCommand<E>`



## object::macho::SECTION_ATTRIBUTES

*Constant*: `u32`

24 section attributes



## object::macho::SECTION_ATTRIBUTES_SYS

*Constant*: `u32`

system setable attributes



## object::macho::SECTION_ATTRIBUTES_USR

*Constant*: `u32`

User setable attributes



## object::macho::SECTION_TYPE

*Constant*: `u32`

256 section types



## object::macho::SECT_BSS

*Constant*: `&str`

the real uninitialized data section no padding



## object::macho::SECT_COMMON

*Constant*: `&str`

the section common symbols are allocated in by the link editor



## object::macho::SECT_DATA

*Constant*: `&str`

the real initialized data section no padding, no bss overlap



## object::macho::SECT_FVMLIB_INIT0

*Constant*: `&str`

the fvmlib initialization section



## object::macho::SECT_FVMLIB_INIT1

*Constant*: `&str`

the section following the fvmlib initialization section



## object::macho::SECT_ICON_HEADER

*Constant*: `&str`

the icon headers



## object::macho::SECT_ICON_TIFF

*Constant*: `&str`

the icons in tiff format



## object::macho::SECT_OBJC_MODULES

*Constant*: `&str`

module information



## object::macho::SECT_OBJC_REFS

*Constant*: `&str`

string table



## object::macho::SECT_OBJC_STRINGS

*Constant*: `&str`

string table



## object::macho::SECT_OBJC_SYMBOLS

*Constant*: `&str`

symbol table



## object::macho::SECT_TEXT

*Constant*: `&str`

the real text part of the text section no headers, and no padding



## object::macho::SEG_DATA

*Constant*: `&str`

the tradition UNIX data segment



## object::macho::SEG_ICON

*Constant*: `&str`

the icon segment



## object::macho::SEG_IMPORT

*Constant*: `&str`

the segment for the self (dyld) modifying code stubs that has read, write and execute permissions



## object::macho::SEG_LINKEDIT

*Constant*: `&str`

the segment containing all structs created and maintained by the link editor.  Created with -seglinkedit option to ld(1) for MH_EXECUTE and FVMLIB file types only



## object::macho::SEG_LINKINFO

*Constant*: `&str`

the segment overlapping with linkedit containing linking information



## object::macho::SEG_OBJC

*Constant*: `&str`

objective-C runtime segment



## object::macho::SEG_PAGEZERO

*Constant*: `&str`

the pagezero segment which has no protections and catches NULL references for MH_EXECUTE files



## object::macho::SEG_TEXT

*Constant*: `&str`

the tradition UNIX text segment



## object::macho::SEG_UNIXSTACK

*Constant*: `&str`

the unix stack segment



## object::macho::SELF_LIBRARY_ORDINAL

*Constant*: `u8`



## object::macho::SG_FVMLIB

*Constant*: `u32`

this segment is the VM that is allocated by a fixed VM library, for overlap checking in the link editor



## object::macho::SG_HIGHVM

*Constant*: `u32`

the file contents for this segment is for the high part of the VM space, the low part is zero filled (for stacks in core files)



## object::macho::SG_NORELOC

*Constant*: `u32`

this segment has nothing that was relocated in it and nothing relocated to it, that is it maybe safely replaced without relocation



## object::macho::SG_PROTECTED_VERSION_1

*Constant*: `u32`

This segment is protected.  If the segment starts at file offset 0, the first page of the segment is not protected.  All other pages of the segment are protected.



## object::macho::SG_READ_ONLY

*Constant*: `u32`

This segment is made read-only after fixups



## object::macho::S_16BYTE_LITERALS

*Constant*: `u32`

section with only 16 byte literals



## object::macho::S_4BYTE_LITERALS

*Constant*: `u32`

section with only 4 byte literals



## object::macho::S_8BYTE_LITERALS

*Constant*: `u32`

section with only 8 byte literals



## object::macho::S_ATTR_DEBUG

*Constant*: `u32`

a debug section



## object::macho::S_ATTR_EXT_RELOC

*Constant*: `u32`

section has external relocation entries



## object::macho::S_ATTR_LIVE_SUPPORT

*Constant*: `u32`

blocks are live if they reference live blocks



## object::macho::S_ATTR_LOC_RELOC

*Constant*: `u32`

section has local relocation entries



## object::macho::S_ATTR_NO_DEAD_STRIP

*Constant*: `u32`

no dead stripping



## object::macho::S_ATTR_NO_TOC

*Constant*: `u32`

section contains coalesced symbols that are not to be in a ranlib table of contents



## object::macho::S_ATTR_PURE_INSTRUCTIONS

*Constant*: `u32`

section contains only true machine instructions



## object::macho::S_ATTR_SELF_MODIFYING_CODE

*Constant*: `u32`

Used with i386 code stubs written on by dyld



## object::macho::S_ATTR_SOME_INSTRUCTIONS

*Constant*: `u32`

section contains some machine instructions



## object::macho::S_ATTR_STRIP_STATIC_SYMS

*Constant*: `u32`

ok to strip static symbols in this section in files with the MH_DYLDLINK flag



## object::macho::S_COALESCED

*Constant*: `u32`

section contains symbols that are to be coalesced



## object::macho::S_CSTRING_LITERALS

*Constant*: `u32`

section with only literal C strings



## object::macho::S_DTRACE_DOF

*Constant*: `u32`

section contains DTrace Object Format



## object::macho::S_GB_ZEROFILL

*Constant*: `u32`

zero fill on demand section (that can be larger than 4 gigabytes)



## object::macho::S_INIT_FUNC_OFFSETS

*Constant*: `u32`

32-bit offsets to initializers



## object::macho::S_INTERPOSING

*Constant*: `u32`

section with only pairs of function pointers for interposing



## object::macho::S_LAZY_DYLIB_SYMBOL_POINTERS

*Constant*: `u32`

section with only lazy symbol pointers to lazy loaded dylibs



## object::macho::S_LAZY_SYMBOL_POINTERS

*Constant*: `u32`

section with only lazy symbol pointers



## object::macho::S_LITERAL_POINTERS

*Constant*: `u32`

section with only pointers to literals



## object::macho::S_MOD_INIT_FUNC_POINTERS

*Constant*: `u32`

section with only function pointers for initialization



## object::macho::S_MOD_TERM_FUNC_POINTERS

*Constant*: `u32`

section with only function pointers for termination



## object::macho::S_NON_LAZY_SYMBOL_POINTERS

*Constant*: `u32`

section with only non-lazy symbol pointers



## object::macho::S_REGULAR

*Constant*: `u32`

regular section



## object::macho::S_SYMBOL_STUBS

*Constant*: `u32`

section with only symbol stubs, byte size of stub in the reserved2 field



## object::macho::S_THREAD_LOCAL_INIT_FUNCTION_POINTERS

*Constant*: `u32`

functions to call to initialize TLV values



## object::macho::S_THREAD_LOCAL_REGULAR

*Constant*: `u32`

template of initial values for TLVs



## object::macho::S_THREAD_LOCAL_VARIABLES

*Constant*: `u32`

TLV descriptors



## object::macho::S_THREAD_LOCAL_VARIABLE_POINTERS

*Constant*: `u32`

pointers to TLV descriptors



## object::macho::S_THREAD_LOCAL_ZEROFILL

*Constant*: `u32`

template of initial values for TLVs



## object::macho::S_ZEROFILL

*Constant*: `u32`

zero fill on demand section



## object::macho::ScatteredRelocationInfo

*Struct*

**Fields:**
- `r_address: u32` - offset in the section to what is being relocated
- `r_type: u8` - if not 0, machine specific relocation type
- `r_length: u8` - 0=byte, 1=word, 2=long, 3=quad
- `r_pcrel: bool` - was relocated pc relative already
- `r_value: u32` - the value the item to be relocated is referring to (without any offset added)

**Methods:**

- `fn relocation<E>(self: Self, endian: E) -> Relocation<E>` - Combine the fields into a `Relocation`.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ScatteredRelocationInfo`



## object::macho::Section32

*Struct*

32-bit section.

**Generic Parameters:**
- E

**Fields:**
- `sectname: [u8; 16]` - name of this section
- `segname: [u8; 16]` - segment this section goes in
- `addr: crate::endian::U32<E>` - memory address of this section
- `size: crate::endian::U32<E>` - size in bytes of this section
- `offset: crate::endian::U32<E>` - file offset of this section
- `align: crate::endian::U32<E>` - section alignment (power of 2)
- `reloff: crate::endian::U32<E>` - file offset of relocation entries
- `nreloc: crate::endian::U32<E>` - number of relocation entries
- `flags: crate::endian::U32<E>` - flags (section type and attributes)
- `reserved1: crate::endian::U32<E>` - reserved (for offset or index)
- `reserved2: crate::endian::U32<E>` - reserved (for count or sizeof)

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Section**
  - `fn sectname(self: &Self) -> &[u8; 16]`
  - `fn segname(self: &Self) -> &[u8; 16]`
  - `fn addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn offset(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn align(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn reloff(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn nreloc(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`
- **Clone**
  - `fn clone(self: &Self) -> Section32<E>`



## object::macho::Section64

*Struct*

64-bit section.

**Generic Parameters:**
- E

**Fields:**
- `sectname: [u8; 16]` - name of this section
- `segname: [u8; 16]` - segment this section goes in
- `addr: crate::endian::U64<E>` - memory address of this section
- `size: crate::endian::U64<E>` - size in bytes of this section
- `offset: crate::endian::U32<E>` - file offset of this section
- `align: crate::endian::U32<E>` - section alignment (power of 2)
- `reloff: crate::endian::U32<E>` - file offset of relocation entries
- `nreloc: crate::endian::U32<E>` - number of relocation entries
- `flags: crate::endian::U32<E>` - flags (section type and attributes)
- `reserved1: crate::endian::U32<E>` - reserved (for offset or index)
- `reserved2: crate::endian::U32<E>` - reserved (for count or sizeof)
- `reserved3: crate::endian::U32<E>` - reserved

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Section64<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Section**
  - `fn sectname(self: &Self) -> &[u8; 16]`
  - `fn segname(self: &Self) -> &[u8; 16]`
  - `fn addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn offset(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn align(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn reloff(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn nreloc(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`



## object::macho::SegmentCommand32

*Struct*

32-bit segment load command.

The segment load command indicates that a part of this file is to be
mapped into the task's address space.  The size of this segment in memory,
vmsize, maybe equal to or larger than the amount to map from this file,
filesize.  The file is mapped starting at fileoff to the beginning of
the segment in memory, vmaddr.  The rest of the memory of the segment,
if any, is allocated zero fill on demand.  The segment's maximum virtual
memory protection and initial virtual memory protection are specified
by the maxprot and initprot fields.  If the segment has sections then the
`Section32` structures directly follow the segment command and their size is
reflected in `cmdsize`.

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_SEGMENT
- `cmdsize: crate::endian::U32<E>` - includes sizeof section structs
- `segname: [u8; 16]` - segment name
- `vmaddr: crate::endian::U32<E>` - memory address of this segment
- `vmsize: crate::endian::U32<E>` - memory size of this segment
- `fileoff: crate::endian::U32<E>` - file offset of this segment
- `filesize: crate::endian::U32<E>` - amount to map from the file
- `maxprot: crate::endian::U32<E>` - maximum VM protection
- `initprot: crate::endian::U32<E>` - initial VM protection
- `nsects: crate::endian::U32<E>` - number of sections in segment
- `flags: crate::endian::U32<E>` - flags

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SegmentCommand32<E>`
- **Segment**
  - `fn from_command(command: LoadCommandData<<Self as >::Endian>) -> Result<Option<(&Self, &[u8])>>`
  - `fn cmd(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn cmdsize(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn segname(self: &Self) -> &[u8; 16]`
  - `fn vmaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn vmsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn fileoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn filesize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn maxprot(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn initprot(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn nsects(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`



## object::macho::SegmentCommand64

*Struct*

64-bit segment load command.

The 64-bit segment load command indicates that a part of this file is to be
mapped into a 64-bit task's address space.  If the 64-bit segment has
sections then `Section64` structures directly follow the 64-bit segment
command and their size is reflected in `cmdsize`.

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_SEGMENT_64
- `cmdsize: crate::endian::U32<E>` - includes sizeof section_64 structs
- `segname: [u8; 16]` - segment name
- `vmaddr: crate::endian::U64<E>` - memory address of this segment
- `vmsize: crate::endian::U64<E>` - memory size of this segment
- `fileoff: crate::endian::U64<E>` - file offset of this segment
- `filesize: crate::endian::U64<E>` - amount to map from the file
- `maxprot: crate::endian::U32<E>` - maximum VM protection
- `initprot: crate::endian::U32<E>` - initial VM protection
- `nsects: crate::endian::U32<E>` - number of sections in segment
- `flags: crate::endian::U32<E>` - flags

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SegmentCommand64<E>`
- **Segment**
  - `fn from_command(command: LoadCommandData<<Self as >::Endian>) -> Result<Option<(&Self, &[u8])>>`
  - `fn cmd(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn cmdsize(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn segname(self: &Self) -> &[u8; 16]`
  - `fn vmaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn vmsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn fileoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn filesize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`
  - `fn maxprot(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn initprot(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn nsects(self: &Self, endian: <Self as >::Endian) -> u32`
  - `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::SourceVersionCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_SOURCE_VERSION
- `cmdsize: crate::endian::U32<E>` - 16
- `version: crate::endian::U64<E>` - A.B.C.D.E packed as a24.b10.c10.d10.e10

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SourceVersionCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::SubClientCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_SUB_CLIENT
- `cmdsize: crate::endian::U32<E>` - includes client string
- `client: LcStr<E>` - the client name

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SubClientCommand<E>`



## object::macho::SubFrameworkCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_SUB_FRAMEWORK
- `cmdsize: crate::endian::U32<E>` - includes umbrella string
- `umbrella: LcStr<E>` - the umbrella framework name

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SubFrameworkCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::SubLibraryCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_SUB_LIBRARY
- `cmdsize: crate::endian::U32<E>` - includes sub_library string
- `sub_library: LcStr<E>` - the sub_library name

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SubLibraryCommand<E>`



## object::macho::SubUmbrellaCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_SUB_UMBRELLA
- `cmdsize: crate::endian::U32<E>` - includes sub_umbrella string
- `sub_umbrella: LcStr<E>` - the sub_umbrella framework name

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SubUmbrellaCommand<E>`



## object::macho::SymsegCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_SYMSEG
- `cmdsize: crate::endian::U32<E>` - sizeof(struct SymsegCommand)
- `offset: crate::endian::U32<E>` - symbol segment offset
- `size: crate::endian::U32<E>` - symbol segment size in bytes

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SymsegCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::SymtabCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_SYMTAB
- `cmdsize: crate::endian::U32<E>` - sizeof(struct SymtabCommand)
- `symoff: crate::endian::U32<E>` - symbol table offset
- `nsyms: crate::endian::U32<E>` - number of symbol table entries
- `stroff: crate::endian::U32<E>` - string table offset
- `strsize: crate::endian::U32<E>` - string table size in bytes

**Methods:**

- `fn symbols<'data, Mach, R>(self: &Self, endian: E, data: R) -> Result<SymbolTable<'data, Mach, R>>` - Return the symbol table that this command references.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SymtabCommand<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::TOOL_CLANG

*Constant*: `u32`



## object::macho::TOOL_LD

*Constant*: `u32`



## object::macho::TOOL_SWIFT

*Constant*: `u32`



## object::macho::ThreadCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_THREAD or  LC_UNIXTHREAD
- `cmdsize: crate::endian::U32<E>` - total size of this command

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ThreadCommand<E>`



## object::macho::TwolevelHint

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `bitfield: crate::endian::U32<E>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TwolevelHint<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::macho::TwolevelHintsCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_TWOLEVEL_HINTS
- `cmdsize: crate::endian::U32<E>` - sizeof(struct TwolevelHintsCommand)
- `offset: crate::endian::U32<E>` - offset to the hint table
- `nhints: crate::endian::U32<E>` - number of hints in the hint table

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TwolevelHintsCommand<E>`



## object::macho::UuidCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_UUID
- `cmdsize: crate::endian::U32<E>` - sizeof(struct UuidCommand)
- `uuid: [u8; 16]` - the 128-bit uuid

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> UuidCommand<E>`



## object::macho::VM_PROT_EXECUTE

*Constant*: `u32`

execute permission



## object::macho::VM_PROT_READ

*Constant*: `u32`

read permission



## object::macho::VM_PROT_WRITE

*Constant*: `u32`

write permission



## object::macho::VersionMinCommand

*Struct*

**Generic Parameters:**
- E

**Fields:**
- `cmd: crate::endian::U32<E>` - LC_VERSION_MIN_MACOSX or LC_VERSION_MIN_IPHONEOS or LC_VERSION_MIN_WATCHOS or LC_VERSION_MIN_TVOS
- `cmdsize: crate::endian::U32<E>` - sizeof(struct VersionMinCommand)
- `version: crate::endian::U32<E>` - X.Y.Z is encoded in nibbles xxxx.yy.zz
- `sdk: crate::endian::U32<E>` - X.Y.Z is encoded in nibbles xxxx.yy.zz

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VersionMinCommand<E>`



## object::macho::X86_64_RELOC_BRANCH

*Constant*: `u8`

a CALL/JMP instruction with 32-bit displacement



## object::macho::X86_64_RELOC_GOT

*Constant*: `u8`

other GOT references



## object::macho::X86_64_RELOC_GOT_LOAD

*Constant*: `u8`

a MOVQ load of a GOT entry



## object::macho::X86_64_RELOC_SIGNED

*Constant*: `u8`

for signed 32-bit displacement



## object::macho::X86_64_RELOC_SIGNED_1

*Constant*: `u8`

for signed 32-bit displacement with a -1 addend



## object::macho::X86_64_RELOC_SIGNED_2

*Constant*: `u8`

for signed 32-bit displacement with a -2 addend



## object::macho::X86_64_RELOC_SIGNED_4

*Constant*: `u8`

for signed 32-bit displacement with a -4 addend



## object::macho::X86_64_RELOC_SUBTRACTOR

*Constant*: `u8`

must be followed by a X86_64_RELOC_UNSIGNED



## object::macho::X86_64_RELOC_TLV

*Constant*: `u8`

for thread local variables



## object::macho::X86_64_RELOC_UNSIGNED

*Constant*: `u8`

for absolute addresses



## object::macho::cpu_subtype_intel

*Function*

```rust
fn cpu_subtype_intel(f: u32, m: u32) -> u32
```



## object::macho::cpu_subtype_intel_family

*Function*

```rust
fn cpu_subtype_intel_family(x: u32) -> u32
```



## object::macho::cpu_subtype_intel_model

*Function*

```rust
fn cpu_subtype_intel_model(x: u32) -> u32
```



