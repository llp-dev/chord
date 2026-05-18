**object > pe**

# Module: pe

## Contents

**Structs**

- [`AnonObjectHeader`](#anonobjectheader) - Non-COFF Object file header
- [`AnonObjectHeaderBigobj`](#anonobjectheaderbigobj)
- [`AnonObjectHeaderV2`](#anonobjectheaderv2)
- [`Guid`](#guid)
- [`ImageAlpha64RuntimeFunctionEntry`](#imagealpha64runtimefunctionentry)
- [`ImageAlphaRuntimeFunctionEntry`](#imagealpharuntimefunctionentry)
- [`ImageArchitectureEntry`](#imagearchitectureentry)
- [`ImageArchiveMemberHeader`](#imagearchivememberheader)
- [`ImageArm64RuntimeFunctionEntry`](#imagearm64runtimefunctionentry)
- [`ImageArmRuntimeFunctionEntry`](#imagearmruntimefunctionentry)
- [`ImageAuxSymbolCrc`](#imageauxsymbolcrc)
- [`ImageAuxSymbolFunction`](#imageauxsymbolfunction) - Auxiliary symbol format 1: function definitions.
- [`ImageAuxSymbolFunctionBeginEnd`](#imageauxsymbolfunctionbeginend) - Auxiliary symbol format 2: .bf and .ef symbols.
- [`ImageAuxSymbolSection`](#imageauxsymbolsection) - Auxiliary symbol format 5: sections.
- [`ImageAuxSymbolTokenDef`](#imageauxsymboltokendef)
- [`ImageAuxSymbolWeak`](#imageauxsymbolweak) - Auxiliary symbol format 3: weak externals.
- [`ImageBaseRelocation`](#imagebaserelocation)
- [`ImageBoundForwarderRef`](#imageboundforwarderref)
- [`ImageBoundImportDescriptor`](#imageboundimportdescriptor)
- [`ImageCoffSymbolsHeader`](#imagecoffsymbolsheader)
- [`ImageCor20Header`](#imagecor20header)
- [`ImageDataDirectory`](#imagedatadirectory)
- [`ImageDebugDirectory`](#imagedebugdirectory)
- [`ImageDebugMisc`](#imagedebugmisc)
- [`ImageDelayloadDescriptor`](#imagedelayloaddescriptor)
- [`ImageDosHeader`](#imagedosheader) - DOS .EXE header
- [`ImageDynamicRelocation32`](#imagedynamicrelocation32)
- [`ImageDynamicRelocation32V2`](#imagedynamicrelocation32v2)
- [`ImageDynamicRelocation64`](#imagedynamicrelocation64)
- [`ImageDynamicRelocation64V2`](#imagedynamicrelocation64v2)
- [`ImageDynamicRelocationTable`](#imagedynamicrelocationtable)
- [`ImageEnclaveConfig32`](#imageenclaveconfig32)
- [`ImageEnclaveConfig64`](#imageenclaveconfig64)
- [`ImageEnclaveImport`](#imageenclaveimport)
- [`ImageEpilogueDynamicRelocationHeader`](#imageepiloguedynamicrelocationheader)
- [`ImageExportDirectory`](#imageexportdirectory)
- [`ImageFileHeader`](#imagefileheader)
- [`ImageFunctionEntry`](#imagefunctionentry)
- [`ImageFunctionEntry64`](#imagefunctionentry64)
- [`ImageHotPatchBase`](#imagehotpatchbase)
- [`ImageHotPatchHashes`](#imagehotpatchhashes)
- [`ImageHotPatchInfo`](#imagehotpatchinfo)
- [`ImageImportByName`](#imageimportbyname)
- [`ImageImportDescriptor`](#imageimportdescriptor)
- [`ImageLinenumber`](#imagelinenumber)
- [`ImageLoadConfigCodeIntegrity`](#imageloadconfigcodeintegrity)
- [`ImageLoadConfigDirectory32`](#imageloadconfigdirectory32)
- [`ImageLoadConfigDirectory64`](#imageloadconfigdirectory64)
- [`ImageNtHeaders32`](#imagentheaders32)
- [`ImageNtHeaders64`](#imagentheaders64)
- [`ImageOptionalHeader32`](#imageoptionalheader32)
- [`ImageOptionalHeader64`](#imageoptionalheader64)
- [`ImageOs2Header`](#imageos2header) - OS/2 .EXE header
- [`ImagePrologueDynamicRelocationHeader`](#imageprologuedynamicrelocationheader)
- [`ImageRelocation`](#imagerelocation)
- [`ImageResourceDataEntry`](#imageresourcedataentry)
- [`ImageResourceDirStringU`](#imageresourcedirstringu)
- [`ImageResourceDirectory`](#imageresourcedirectory)
- [`ImageResourceDirectoryEntry`](#imageresourcedirectoryentry)
- [`ImageResourceDirectoryString`](#imageresourcedirectorystring)
- [`ImageRomHeaders`](#imageromheaders)
- [`ImageRomOptionalHeader`](#imageromoptionalheader)
- [`ImageRuntimeFunctionEntry`](#imageruntimefunctionentry)
- [`ImageSectionHeader`](#imagesectionheader)
- [`ImageSeparateDebugHeader`](#imageseparatedebugheader)
- [`ImageSymbol`](#imagesymbol)
- [`ImageSymbolBytes`](#imagesymbolbytes)
- [`ImageSymbolEx`](#imagesymbolex)
- [`ImageSymbolExBytes`](#imagesymbolexbytes)
- [`ImageThunkData32`](#imagethunkdata32)
- [`ImageThunkData64`](#imagethunkdata64)
- [`ImageTlsDirectory32`](#imagetlsdirectory32)
- [`ImageTlsDirectory64`](#imagetlsdirectory64)
- [`ImageVxdHeader`](#imagevxdheader) - Windows VXD header
- [`ImportObjectHeader`](#importobjectheader)
- [`MaskedRichHeaderEntry`](#maskedrichheaderentry) - A PE rich header entry.
- [`NonPagedDebugInfo`](#nonpageddebuginfo)

**Constants**

- [`ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID`](#anon_object_header_bigobj_class_id) - The required value of `AnonObjectHeaderBigobj::class_id`.
- [`COMIMAGE_FLAGS_32BITPREFERRED`](#comimage_flags_32bitpreferred)
- [`COMIMAGE_FLAGS_32BITREQUIRED`](#comimage_flags_32bitrequired)
- [`COMIMAGE_FLAGS_ILONLY`](#comimage_flags_ilonly)
- [`COMIMAGE_FLAGS_IL_LIBRARY`](#comimage_flags_il_library)
- [`COMIMAGE_FLAGS_NATIVE_ENTRYPOINT`](#comimage_flags_native_entrypoint)
- [`COMIMAGE_FLAGS_STRONGNAMESIGNED`](#comimage_flags_strongnamesigned)
- [`COMIMAGE_FLAGS_TRACKDEBUGDATA`](#comimage_flags_trackdebugdata)
- [`COR_DELETED_NAME_LENGTH`](#cor_deleted_name_length)
- [`COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE`](#cor_ilmethod_sect_small_max_datasize)
- [`COR_VERSION_MAJOR`](#cor_version_major)
- [`COR_VERSION_MAJOR_V2`](#cor_version_major_v2)
- [`COR_VERSION_MINOR`](#cor_version_minor)
- [`COR_VTABLEGAP_NAME_LENGTH`](#cor_vtablegap_name_length)
- [`COR_VTABLE_32BIT`](#cor_vtable_32bit) - V-table slots are 32-bits in size.
- [`COR_VTABLE_64BIT`](#cor_vtable_64bit) - V-table slots are 64-bits in size.
- [`COR_VTABLE_CALL_MOST_DERIVED`](#cor_vtable_call_most_derived) - Call most derived method described by
- [`COR_VTABLE_FROM_UNMANAGED`](#cor_vtable_from_unmanaged) - If set, transition from unmanaged.
- [`COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN`](#cor_vtable_from_unmanaged_retain_appdomain) - If set, transition from unmanaged with keeping the current appdomain.
- [`EMARCH_ENC_I17_IC_INST_WORD_POS_X`](#emarch_enc_i17_ic_inst_word_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IC_INST_WORD_X`](#emarch_enc_i17_ic_inst_word_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IC_SIZE_X`](#emarch_enc_i17_ic_size_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IC_VAL_POS_X`](#emarch_enc_i17_ic_val_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41A_INST_WORD_POS_X`](#emarch_enc_i17_imm41a_inst_word_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41A_INST_WORD_X`](#emarch_enc_i17_imm41a_inst_word_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41A_SIZE_X`](#emarch_enc_i17_imm41a_size_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41A_VAL_POS_X`](#emarch_enc_i17_imm41a_val_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41B_INST_WORD_POS_X`](#emarch_enc_i17_imm41b_inst_word_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41B_INST_WORD_X`](#emarch_enc_i17_imm41b_inst_word_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41B_SIZE_X`](#emarch_enc_i17_imm41b_size_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41B_VAL_POS_X`](#emarch_enc_i17_imm41b_val_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41C_INST_WORD_POS_X`](#emarch_enc_i17_imm41c_inst_word_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41C_INST_WORD_X`](#emarch_enc_i17_imm41c_inst_word_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41C_SIZE_X`](#emarch_enc_i17_imm41c_size_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM41C_VAL_POS_X`](#emarch_enc_i17_imm41c_val_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X`](#emarch_enc_i17_imm5c_inst_word_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM5C_INST_WORD_X`](#emarch_enc_i17_imm5c_inst_word_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM5C_SIZE_X`](#emarch_enc_i17_imm5c_size_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM5C_VAL_POS_X`](#emarch_enc_i17_imm5c_val_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X`](#emarch_enc_i17_imm7b_inst_word_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM7B_INST_WORD_X`](#emarch_enc_i17_imm7b_inst_word_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM7B_SIZE_X`](#emarch_enc_i17_imm7b_size_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM7B_VAL_POS_X`](#emarch_enc_i17_imm7b_val_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X`](#emarch_enc_i17_imm9d_inst_word_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM9D_INST_WORD_X`](#emarch_enc_i17_imm9d_inst_word_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM9D_SIZE_X`](#emarch_enc_i17_imm9d_size_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_IMM9D_VAL_POS_X`](#emarch_enc_i17_imm9d_val_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_SIGN_INST_WORD_POS_X`](#emarch_enc_i17_sign_inst_word_pos_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_SIGN_INST_WORD_X`](#emarch_enc_i17_sign_inst_word_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_SIGN_SIZE_X`](#emarch_enc_i17_sign_size_x) - Intel-IA64-Filler
- [`EMARCH_ENC_I17_SIGN_VAL_POS_X`](#emarch_enc_i17_sign_val_pos_x) - Intel-IA64-Filler
- [`FRAME_FPO`](#frame_fpo)
- [`FRAME_NONFPO`](#frame_nonfpo)
- [`FRAME_TRAP`](#frame_trap)
- [`FRAME_TSS`](#frame_tss)
- [`IMAGE_ARCHIVE_END`](#image_archive_end)
- [`IMAGE_ARCHIVE_HYBRIDMAP_MEMBER`](#image_archive_hybridmap_member)
- [`IMAGE_ARCHIVE_LINKER_MEMBER`](#image_archive_linker_member)
- [`IMAGE_ARCHIVE_LONGNAMES_MEMBER`](#image_archive_longnames_member)
- [`IMAGE_ARCHIVE_PAD`](#image_archive_pad)
- [`IMAGE_ARCHIVE_START`](#image_archive_start)
- [`IMAGE_ARCHIVE_START_SIZE`](#image_archive_start_size)
- [`IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF`](#image_aux_symbol_type_token_def)
- [`IMAGE_COMDAT_SELECT_ANY`](#image_comdat_select_any)
- [`IMAGE_COMDAT_SELECT_ASSOCIATIVE`](#image_comdat_select_associative)
- [`IMAGE_COMDAT_SELECT_EXACT_MATCH`](#image_comdat_select_exact_match)
- [`IMAGE_COMDAT_SELECT_LARGEST`](#image_comdat_select_largest)
- [`IMAGE_COMDAT_SELECT_NEWEST`](#image_comdat_select_newest)
- [`IMAGE_COMDAT_SELECT_NODUPLICATES`](#image_comdat_select_noduplicates)
- [`IMAGE_COMDAT_SELECT_SAME_SIZE`](#image_comdat_select_same_size)
- [`IMAGE_COR_EATJ_THUNK_SIZE`](#image_cor_eatj_thunk_size) - Size of a jump thunk reserved range.
- [`IMAGE_COR_MIH_BASICBLOCK`](#image_cor_mih_basicblock)
- [`IMAGE_COR_MIH_EHRVA`](#image_cor_mih_ehrva)
- [`IMAGE_COR_MIH_METHODRVA`](#image_cor_mih_methodrva)
- [`IMAGE_DEBUG_MISC_EXENAME`](#image_debug_misc_exename)
- [`IMAGE_DEBUG_TYPE_BORLAND`](#image_debug_type_borland)
- [`IMAGE_DEBUG_TYPE_CLSID`](#image_debug_type_clsid)
- [`IMAGE_DEBUG_TYPE_CODEVIEW`](#image_debug_type_codeview)
- [`IMAGE_DEBUG_TYPE_COFF`](#image_debug_type_coff)
- [`IMAGE_DEBUG_TYPE_EXCEPTION`](#image_debug_type_exception)
- [`IMAGE_DEBUG_TYPE_FIXUP`](#image_debug_type_fixup)
- [`IMAGE_DEBUG_TYPE_FPO`](#image_debug_type_fpo)
- [`IMAGE_DEBUG_TYPE_ILTCG`](#image_debug_type_iltcg)
- [`IMAGE_DEBUG_TYPE_MISC`](#image_debug_type_misc)
- [`IMAGE_DEBUG_TYPE_MPX`](#image_debug_type_mpx)
- [`IMAGE_DEBUG_TYPE_OMAP_FROM_SRC`](#image_debug_type_omap_from_src)
- [`IMAGE_DEBUG_TYPE_OMAP_TO_SRC`](#image_debug_type_omap_to_src)
- [`IMAGE_DEBUG_TYPE_POGO`](#image_debug_type_pogo)
- [`IMAGE_DEBUG_TYPE_REPRO`](#image_debug_type_repro)
- [`IMAGE_DEBUG_TYPE_RESERVED10`](#image_debug_type_reserved10)
- [`IMAGE_DEBUG_TYPE_UNKNOWN`](#image_debug_type_unknown)
- [`IMAGE_DEBUG_TYPE_VC_FEATURE`](#image_debug_type_vc_feature)
- [`IMAGE_DELAYLOAD_RVA_BASED`](#image_delayload_rva_based) - Delay load version 2 flag for `ImageDelayloadDescriptor::attributes`.
- [`IMAGE_DIRECTORY_ENTRY_ARCHITECTURE`](#image_directory_entry_architecture) - Architecture Specific Data
- [`IMAGE_DIRECTORY_ENTRY_BASERELOC`](#image_directory_entry_basereloc) - Base Relocation Table
- [`IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT`](#image_directory_entry_bound_import) - Bound Import Directory in headers
- [`IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR`](#image_directory_entry_com_descriptor) - COM Runtime descriptor
- [`IMAGE_DIRECTORY_ENTRY_DEBUG`](#image_directory_entry_debug) - Debug Directory
- [`IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT`](#image_directory_entry_delay_import) - Delay Load Import Descriptors
- [`IMAGE_DIRECTORY_ENTRY_EXCEPTION`](#image_directory_entry_exception) - Exception Directory
- [`IMAGE_DIRECTORY_ENTRY_EXPORT`](#image_directory_entry_export) - Export Directory
- [`IMAGE_DIRECTORY_ENTRY_GLOBALPTR`](#image_directory_entry_globalptr) - RVA of GP
- [`IMAGE_DIRECTORY_ENTRY_IAT`](#image_directory_entry_iat) - Import Address Table
- [`IMAGE_DIRECTORY_ENTRY_IMPORT`](#image_directory_entry_import) - Import Directory
- [`IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG`](#image_directory_entry_load_config) - Load Configuration Directory
- [`IMAGE_DIRECTORY_ENTRY_RESOURCE`](#image_directory_entry_resource) - Resource Directory
- [`IMAGE_DIRECTORY_ENTRY_SECURITY`](#image_directory_entry_security) - Security Directory
- [`IMAGE_DIRECTORY_ENTRY_TLS`](#image_directory_entry_tls) - TLS Directory
- [`IMAGE_DLLCHARACTERISTICS_APPCONTAINER`](#image_dllcharacteristics_appcontainer) - Image should execute in an AppContainer
- [`IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE`](#image_dllcharacteristics_dynamic_base) - DLL can move.
- [`IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY`](#image_dllcharacteristics_force_integrity) - Code Integrity Image
- [`IMAGE_DLLCHARACTERISTICS_GUARD_CF`](#image_dllcharacteristics_guard_cf) - Image supports Control Flow Guard.
- [`IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA`](#image_dllcharacteristics_high_entropy_va) - Image can handle a high entropy 64-bit virtual address space.
- [`IMAGE_DLLCHARACTERISTICS_NO_BIND`](#image_dllcharacteristics_no_bind) - Do not bind this image.
- [`IMAGE_DLLCHARACTERISTICS_NO_ISOLATION`](#image_dllcharacteristics_no_isolation) - Image understands isolation and doesn't want it
- [`IMAGE_DLLCHARACTERISTICS_NO_SEH`](#image_dllcharacteristics_no_seh) - Image does not use SEH.  No SE handler may reside in this image
- [`IMAGE_DLLCHARACTERISTICS_NX_COMPAT`](#image_dllcharacteristics_nx_compat) - Image is NX compatible
- [`IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE`](#image_dllcharacteristics_terminal_server_aware)
- [`IMAGE_DLLCHARACTERISTICS_WDM_DRIVER`](#image_dllcharacteristics_wdm_driver) - Driver uses WDM model
- [`IMAGE_DOS_SIGNATURE`](#image_dos_signature) - MZ
- [`IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER`](#image_dynamic_relocation_guard_import_control_transfer)
- [`IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER`](#image_dynamic_relocation_guard_indir_control_transfer)
- [`IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE`](#image_dynamic_relocation_guard_rf_epilogue)
- [`IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE`](#image_dynamic_relocation_guard_rf_prologue)
- [`IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH`](#image_dynamic_relocation_guard_switchtable_branch)
- [`IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE`](#image_enclave_flag_primary_image)
- [`IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID`](#image_enclave_import_match_author_id)
- [`IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID`](#image_enclave_import_match_family_id)
- [`IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID`](#image_enclave_import_match_image_id)
- [`IMAGE_ENCLAVE_IMPORT_MATCH_NONE`](#image_enclave_import_match_none)
- [`IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID`](#image_enclave_import_match_unique_id)
- [`IMAGE_ENCLAVE_LONG_ID_LENGTH`](#image_enclave_long_id_length)
- [`IMAGE_ENCLAVE_POLICY_DEBUGGABLE`](#image_enclave_policy_debuggable)
- [`IMAGE_ENCLAVE_SHORT_ID_LENGTH`](#image_enclave_short_id_length)
- [`IMAGE_FILE_32BIT_MACHINE`](#image_file_32bit_machine) - 32 bit word machine.
- [`IMAGE_FILE_AGGRESIVE_WS_TRIM`](#image_file_aggresive_ws_trim) - Aggressively trim working set
- [`IMAGE_FILE_BYTES_REVERSED_HI`](#image_file_bytes_reversed_hi) - Bytes of machine word are reversed.
- [`IMAGE_FILE_BYTES_REVERSED_LO`](#image_file_bytes_reversed_lo) - Bytes of machine word are reversed.
- [`IMAGE_FILE_DEBUG_STRIPPED`](#image_file_debug_stripped) - Debugging info stripped from file in .DBG file
- [`IMAGE_FILE_DLL`](#image_file_dll) - File is a DLL.
- [`IMAGE_FILE_EXECUTABLE_IMAGE`](#image_file_executable_image) - File is executable  (i.e. no unresolved external references).
- [`IMAGE_FILE_LARGE_ADDRESS_AWARE`](#image_file_large_address_aware) - App can handle >2gb addresses
- [`IMAGE_FILE_LINE_NUMS_STRIPPED`](#image_file_line_nums_stripped) - Line numbers stripped from file.
- [`IMAGE_FILE_LOCAL_SYMS_STRIPPED`](#image_file_local_syms_stripped) - Local symbols stripped from file.
- [`IMAGE_FILE_MACHINE_ALPHA`](#image_file_machine_alpha) - Alpha_AXP
- [`IMAGE_FILE_MACHINE_ALPHA64`](#image_file_machine_alpha64) - ALPHA64
- [`IMAGE_FILE_MACHINE_AM33`](#image_file_machine_am33)
- [`IMAGE_FILE_MACHINE_AMD64`](#image_file_machine_amd64) - AMD64 (K8)
- [`IMAGE_FILE_MACHINE_ARM`](#image_file_machine_arm) - ARM Little-Endian
- [`IMAGE_FILE_MACHINE_ARM64`](#image_file_machine_arm64) - ARM64 Little-Endian
- [`IMAGE_FILE_MACHINE_ARM64EC`](#image_file_machine_arm64ec) - ARM64EC ("Emulation Compatible")
- [`IMAGE_FILE_MACHINE_ARM64X`](#image_file_machine_arm64x) - ARM64X (Mixed ARM64 and ARM64EC)
- [`IMAGE_FILE_MACHINE_ARMNT`](#image_file_machine_armnt) - ARM Thumb-2 Little-Endian
- [`IMAGE_FILE_MACHINE_AXP64`](#image_file_machine_axp64)
- [`IMAGE_FILE_MACHINE_CEE`](#image_file_machine_cee)
- [`IMAGE_FILE_MACHINE_CEF`](#image_file_machine_cef)
- [`IMAGE_FILE_MACHINE_CHPE_X86`](#image_file_machine_chpe_x86) - CHPE x86 ("Compiled Hybrid Portable Executable")
- [`IMAGE_FILE_MACHINE_EBC`](#image_file_machine_ebc) - EFI Byte Code
- [`IMAGE_FILE_MACHINE_I386`](#image_file_machine_i386) - Intel 386.
- [`IMAGE_FILE_MACHINE_IA64`](#image_file_machine_ia64) - Intel 64
- [`IMAGE_FILE_MACHINE_M32R`](#image_file_machine_m32r) - M32R little-endian
- [`IMAGE_FILE_MACHINE_MIPS16`](#image_file_machine_mips16) - MIPS
- [`IMAGE_FILE_MACHINE_MIPSFPU`](#image_file_machine_mipsfpu) - MIPS
- [`IMAGE_FILE_MACHINE_MIPSFPU16`](#image_file_machine_mipsfpu16) - MIPS
- [`IMAGE_FILE_MACHINE_POWERPC`](#image_file_machine_powerpc) - IBM PowerPC Little-Endian
- [`IMAGE_FILE_MACHINE_POWERPCBE`](#image_file_machine_powerpcbe) - IBM PowerPC Big-Endian
- [`IMAGE_FILE_MACHINE_POWERPCFP`](#image_file_machine_powerpcfp)
- [`IMAGE_FILE_MACHINE_R10000`](#image_file_machine_r10000) - MIPS little-endian
- [`IMAGE_FILE_MACHINE_R3000`](#image_file_machine_r3000) - MIPS little-endian, 0x160 big-endian
- [`IMAGE_FILE_MACHINE_R4000`](#image_file_machine_r4000) - MIPS little-endian
- [`IMAGE_FILE_MACHINE_RISCV128`](#image_file_machine_riscv128) - RISCV128
- [`IMAGE_FILE_MACHINE_RISCV32`](#image_file_machine_riscv32) - RISCV32
- [`IMAGE_FILE_MACHINE_RISCV64`](#image_file_machine_riscv64) - RISCV64
- [`IMAGE_FILE_MACHINE_SH3`](#image_file_machine_sh3) - SH3 little-endian
- [`IMAGE_FILE_MACHINE_SH3DSP`](#image_file_machine_sh3dsp)
- [`IMAGE_FILE_MACHINE_SH3E`](#image_file_machine_sh3e) - SH3E little-endian
- [`IMAGE_FILE_MACHINE_SH4`](#image_file_machine_sh4) - SH4 little-endian
- [`IMAGE_FILE_MACHINE_SH5`](#image_file_machine_sh5) - SH5
- [`IMAGE_FILE_MACHINE_TARGET_HOST`](#image_file_machine_target_host) - Useful for indicating we want to interact with the host and not a WoW guest.
- [`IMAGE_FILE_MACHINE_THUMB`](#image_file_machine_thumb) - ARM Thumb/Thumb-2 Little-Endian
- [`IMAGE_FILE_MACHINE_TRICORE`](#image_file_machine_tricore) - Infineon
- [`IMAGE_FILE_MACHINE_UNKNOWN`](#image_file_machine_unknown)
- [`IMAGE_FILE_MACHINE_WCEMIPSV2`](#image_file_machine_wcemipsv2) - MIPS little-endian WCE v2
- [`IMAGE_FILE_NET_RUN_FROM_SWAP`](#image_file_net_run_from_swap) - If Image is on Net, copy and run from the swap file.
- [`IMAGE_FILE_RELOCS_STRIPPED`](#image_file_relocs_stripped) - Relocation info stripped from file.
- [`IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP`](#image_file_removable_run_from_swap) - If Image is on removable media, copy and run from the swap file.
- [`IMAGE_FILE_SYSTEM`](#image_file_system) - System File.
- [`IMAGE_FILE_UP_SYSTEM_ONLY`](#image_file_up_system_only) - File should only be run on a UP machine
- [`IMAGE_GUARD_CFW_INSTRUMENTED`](#image_guard_cfw_instrumented) - Module performs control flow and write integrity checks
- [`IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION`](#image_guard_cf_enable_export_suppression) - Module enables suppression of exports
- [`IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT`](#image_guard_cf_export_suppression_info_present) - Module contains suppressed export information.
- [`IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT`](#image_guard_cf_function_table_present) - Module contains valid control flow target metadata
- [`IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK`](#image_guard_cf_function_table_size_mask) - Stride of Guard CF function table encoded in these bits (additional count of bytes per element)
- [`IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT`](#image_guard_cf_function_table_size_shift) - Shift to right-justify Guard CF function table stride
- [`IMAGE_GUARD_CF_INSTRUMENTED`](#image_guard_cf_instrumented) - Module performs control flow integrity checks using system-supplied support
- [`IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT`](#image_guard_cf_longjump_table_present) - Module contains longjmp target information
- [`IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION`](#image_guard_delayload_iat_in_its_own_section) - Delayload import table in its own .didat section (with nothing else in it) that can be freely reprotected
- [`IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED`](#image_guard_flag_export_suppressed) - The containing GFID entry is export suppressed
- [`IMAGE_GUARD_FLAG_FID_SUPPRESSED`](#image_guard_flag_fid_suppressed) - The containing GFID entry is suppressed
- [`IMAGE_GUARD_PROTECT_DELAYLOAD_IAT`](#image_guard_protect_delayload_iat) - Module supports read only delay load IAT
- [`IMAGE_GUARD_RETPOLINE_PRESENT`](#image_guard_retpoline_present) - Module was built with retpoline support
- [`IMAGE_GUARD_RF_ENABLE`](#image_guard_rf_enable) - Module requests that the OS enable return flow protection
- [`IMAGE_GUARD_RF_INSTRUMENTED`](#image_guard_rf_instrumented) - Module contains return flow instrumentation and metadata
- [`IMAGE_GUARD_RF_STRICT`](#image_guard_rf_strict) - Module requests that the OS enable return flow protection in strict mode
- [`IMAGE_GUARD_SECURITY_COOKIE_UNUSED`](#image_guard_security_cookie_unused) - Module does not make use of the /GS security cookie
- [`IMAGE_HOT_PATCH_ABSOLUTE`](#image_hot_patch_absolute)
- [`IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK`](#image_hot_patch_base_can_roll_back)
- [`IMAGE_HOT_PATCH_BASE_OBLIGATORY`](#image_hot_patch_base_obligatory)
- [`IMAGE_HOT_PATCH_CALL_TARGET`](#image_hot_patch_call_target)
- [`IMAGE_HOT_PATCH_CHUNK_INVERSE`](#image_hot_patch_chunk_inverse)
- [`IMAGE_HOT_PATCH_CHUNK_OBLIGATORY`](#image_hot_patch_chunk_obligatory)
- [`IMAGE_HOT_PATCH_CHUNK_RESERVED`](#image_hot_patch_chunk_reserved)
- [`IMAGE_HOT_PATCH_CHUNK_SIZE`](#image_hot_patch_chunk_size)
- [`IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA`](#image_hot_patch_chunk_source_rva)
- [`IMAGE_HOT_PATCH_CHUNK_TARGET_RVA`](#image_hot_patch_chunk_target_rva)
- [`IMAGE_HOT_PATCH_CHUNK_TYPE`](#image_hot_patch_chunk_type)
- [`IMAGE_HOT_PATCH_DYNAMIC_VALUE`](#image_hot_patch_dynamic_value)
- [`IMAGE_HOT_PATCH_FUNCTION`](#image_hot_patch_function)
- [`IMAGE_HOT_PATCH_INDIRECT`](#image_hot_patch_indirect)
- [`IMAGE_HOT_PATCH_NONE`](#image_hot_patch_none)
- [`IMAGE_HOT_PATCH_NO_CALL_TARGET`](#image_hot_patch_no_call_target)
- [`IMAGE_HOT_PATCH_REL32`](#image_hot_patch_rel32)
- [`IMAGE_NT_OPTIONAL_HDR32_MAGIC`](#image_nt_optional_hdr32_magic)
- [`IMAGE_NT_OPTIONAL_HDR64_MAGIC`](#image_nt_optional_hdr64_magic)
- [`IMAGE_NT_SIGNATURE`](#image_nt_signature) - PE00
- [`IMAGE_NUMBEROF_DIRECTORY_ENTRIES`](#image_numberof_directory_entries)
- [`IMAGE_ORDINAL_FLAG32`](#image_ordinal_flag32)
- [`IMAGE_ORDINAL_FLAG64`](#image_ordinal_flag64)
- [`IMAGE_OS2_SIGNATURE`](#image_os2_signature) - NE
- [`IMAGE_OS2_SIGNATURE_LE`](#image_os2_signature_le) - LE
- [`IMAGE_REL_ALPHA_ABSOLUTE`](#image_rel_alpha_absolute)
- [`IMAGE_REL_ALPHA_BRADDR`](#image_rel_alpha_braddr)
- [`IMAGE_REL_ALPHA_GPDISP`](#image_rel_alpha_gpdisp)
- [`IMAGE_REL_ALPHA_GPREL32`](#image_rel_alpha_gprel32)
- [`IMAGE_REL_ALPHA_GPRELHI`](#image_rel_alpha_gprelhi) - High 16-bit GP relative reference
- [`IMAGE_REL_ALPHA_GPRELLO`](#image_rel_alpha_gprello) - Low 16-bit GP relative reference
- [`IMAGE_REL_ALPHA_HINT`](#image_rel_alpha_hint)
- [`IMAGE_REL_ALPHA_INLINE_REFLONG`](#image_rel_alpha_inline_reflong)
- [`IMAGE_REL_ALPHA_LITERAL`](#image_rel_alpha_literal)
- [`IMAGE_REL_ALPHA_LITUSE`](#image_rel_alpha_lituse)
- [`IMAGE_REL_ALPHA_MATCH`](#image_rel_alpha_match)
- [`IMAGE_REL_ALPHA_PAIR`](#image_rel_alpha_pair)
- [`IMAGE_REL_ALPHA_REFHI`](#image_rel_alpha_refhi)
- [`IMAGE_REL_ALPHA_REFLO`](#image_rel_alpha_reflo)
- [`IMAGE_REL_ALPHA_REFLONG`](#image_rel_alpha_reflong)
- [`IMAGE_REL_ALPHA_REFLONGNB`](#image_rel_alpha_reflongnb)
- [`IMAGE_REL_ALPHA_REFQ1`](#image_rel_alpha_refq1) - Low 16 bits of 48 bit reference
- [`IMAGE_REL_ALPHA_REFQ2`](#image_rel_alpha_refq2) - Middle 16 bits of 48 bit reference
- [`IMAGE_REL_ALPHA_REFQ3`](#image_rel_alpha_refq3) - High 16 bits of 48 bit reference
- [`IMAGE_REL_ALPHA_REFQUAD`](#image_rel_alpha_refquad)
- [`IMAGE_REL_ALPHA_SECREL`](#image_rel_alpha_secrel)
- [`IMAGE_REL_ALPHA_SECRELHI`](#image_rel_alpha_secrelhi) - High 16-bit section relative reference
- [`IMAGE_REL_ALPHA_SECRELLO`](#image_rel_alpha_secrello) - Low 16-bit section relative reference
- [`IMAGE_REL_ALPHA_SECTION`](#image_rel_alpha_section)
- [`IMAGE_REL_AMD64_ABSOLUTE`](#image_rel_amd64_absolute) - Reference is absolute, no relocation is necessary
- [`IMAGE_REL_AMD64_ADDR32`](#image_rel_amd64_addr32) - 32-bit address (VA).
- [`IMAGE_REL_AMD64_ADDR32NB`](#image_rel_amd64_addr32nb) - 32-bit address w/o image base (RVA).
- [`IMAGE_REL_AMD64_ADDR64`](#image_rel_amd64_addr64) - 64-bit address (VA).
- [`IMAGE_REL_AMD64_CFG_BR`](#image_rel_amd64_cfg_br) - Indirect branch to a CFG check
- [`IMAGE_REL_AMD64_CFG_BR_REX`](#image_rel_amd64_cfg_br_rex) - Indirect branch to a CFG check, with REX.W prefix
- [`IMAGE_REL_AMD64_CFG_CALL`](#image_rel_amd64_cfg_call) - Indirect call to a CFG check
- [`IMAGE_REL_AMD64_EHANDLER`](#image_rel_amd64_ehandler)
- [`IMAGE_REL_AMD64_IMPORT_BR`](#image_rel_amd64_import_br) - Indirect branch to an import
- [`IMAGE_REL_AMD64_IMPORT_CALL`](#image_rel_amd64_import_call) - Indirect call to an import
- [`IMAGE_REL_AMD64_INDIR_BR`](#image_rel_amd64_indir_br) - Indirect branch to a target in RAX (no CFG)
- [`IMAGE_REL_AMD64_INDIR_BR_REX`](#image_rel_amd64_indir_br_rex) - Indirect branch to a target in RAX, with REX.W prefix (no CFG)
- [`IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST`](#image_rel_amd64_indir_br_switchtable_first) - Indirect branch for a switch table using Reg 0 (RAX)
- [`IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST`](#image_rel_amd64_indir_br_switchtable_last) - Indirect branch for a switch table using Reg 15 (R15)
- [`IMAGE_REL_AMD64_INDIR_CALL`](#image_rel_amd64_indir_call) - Indirect call to a target in RAX (no CFG)
- [`IMAGE_REL_AMD64_PAIR`](#image_rel_amd64_pair)
- [`IMAGE_REL_AMD64_REL32`](#image_rel_amd64_rel32) - 32-bit relative address from byte following reloc
- [`IMAGE_REL_AMD64_REL32_1`](#image_rel_amd64_rel32_1) - 32-bit relative address from byte distance 1 from reloc
- [`IMAGE_REL_AMD64_REL32_2`](#image_rel_amd64_rel32_2) - 32-bit relative address from byte distance 2 from reloc
- [`IMAGE_REL_AMD64_REL32_3`](#image_rel_amd64_rel32_3) - 32-bit relative address from byte distance 3 from reloc
- [`IMAGE_REL_AMD64_REL32_4`](#image_rel_amd64_rel32_4) - 32-bit relative address from byte distance 4 from reloc
- [`IMAGE_REL_AMD64_REL32_5`](#image_rel_amd64_rel32_5) - 32-bit relative address from byte distance 5 from reloc
- [`IMAGE_REL_AMD64_SECREL`](#image_rel_amd64_secrel) - 32 bit offset from base of section containing target
- [`IMAGE_REL_AMD64_SECREL7`](#image_rel_amd64_secrel7) - 7 bit unsigned offset from base of section containing target
- [`IMAGE_REL_AMD64_SECTION`](#image_rel_amd64_section) - Section index
- [`IMAGE_REL_AMD64_SREL32`](#image_rel_amd64_srel32) - 32 bit signed span-dependent value emitted into object
- [`IMAGE_REL_AMD64_SSPAN32`](#image_rel_amd64_sspan32) - 32 bit signed span-dependent value applied at link time
- [`IMAGE_REL_AMD64_TOKEN`](#image_rel_amd64_token) - 32 bit metadata token
- [`IMAGE_REL_AM_ABSOLUTE`](#image_rel_am_absolute)
- [`IMAGE_REL_AM_ADDR32`](#image_rel_am_addr32)
- [`IMAGE_REL_AM_ADDR32NB`](#image_rel_am_addr32nb)
- [`IMAGE_REL_AM_CALL32`](#image_rel_am_call32)
- [`IMAGE_REL_AM_FUNCINFO`](#image_rel_am_funcinfo)
- [`IMAGE_REL_AM_REL32_1`](#image_rel_am_rel32_1)
- [`IMAGE_REL_AM_REL32_2`](#image_rel_am_rel32_2)
- [`IMAGE_REL_AM_SECREL`](#image_rel_am_secrel)
- [`IMAGE_REL_AM_SECTION`](#image_rel_am_section)
- [`IMAGE_REL_AM_TOKEN`](#image_rel_am_token)
- [`IMAGE_REL_ARM64_ABSOLUTE`](#image_rel_arm64_absolute) - No relocation required
- [`IMAGE_REL_ARM64_ADDR32`](#image_rel_arm64_addr32) - 32 bit address. Review! do we need it?
- [`IMAGE_REL_ARM64_ADDR32NB`](#image_rel_arm64_addr32nb) - 32 bit address w/o image base (RVA: for Data/PData/XData)
- [`IMAGE_REL_ARM64_ADDR64`](#image_rel_arm64_addr64) - 64 bit address
- [`IMAGE_REL_ARM64_BRANCH14`](#image_rel_arm64_branch14) - TBZ/TBNZ
- [`IMAGE_REL_ARM64_BRANCH19`](#image_rel_arm64_branch19) - 19 bit offset << 2 & sign ext. for conditional B
- [`IMAGE_REL_ARM64_BRANCH26`](#image_rel_arm64_branch26) - 26 bit offset << 2 & sign ext. for B & BL
- [`IMAGE_REL_ARM64_PAGEBASE_REL21`](#image_rel_arm64_pagebase_rel21) - ADRP
- [`IMAGE_REL_ARM64_PAGEOFFSET_12A`](#image_rel_arm64_pageoffset_12a) - ADD/ADDS (immediate) with zero shift, for page offset
- [`IMAGE_REL_ARM64_PAGEOFFSET_12L`](#image_rel_arm64_pageoffset_12l) - LDR (indexed, unsigned immediate), for page offset
- [`IMAGE_REL_ARM64_REL21`](#image_rel_arm64_rel21) - ADR
- [`IMAGE_REL_ARM64_REL32`](#image_rel_arm64_rel32) - 32-bit relative address from byte following reloc
- [`IMAGE_REL_ARM64_SECREL`](#image_rel_arm64_secrel) - Offset within section
- [`IMAGE_REL_ARM64_SECREL_HIGH12A`](#image_rel_arm64_secrel_high12a) - ADD/ADDS (immediate) with zero shift, for bit 12:23 of section offset
- [`IMAGE_REL_ARM64_SECREL_LOW12A`](#image_rel_arm64_secrel_low12a) - ADD/ADDS (immediate) with zero shift, for bit 0:11 of section offset
- [`IMAGE_REL_ARM64_SECREL_LOW12L`](#image_rel_arm64_secrel_low12l) - LDR (indexed, unsigned immediate), for bit 0:11 of section offset
- [`IMAGE_REL_ARM64_SECTION`](#image_rel_arm64_section) - Section table index
- [`IMAGE_REL_ARM64_TOKEN`](#image_rel_arm64_token)
- [`IMAGE_REL_ARM_ABSOLUTE`](#image_rel_arm_absolute) - No relocation required
- [`IMAGE_REL_ARM_ADDR32`](#image_rel_arm_addr32) - 32 bit address
- [`IMAGE_REL_ARM_ADDR32NB`](#image_rel_arm_addr32nb) - 32 bit address w/o image base
- [`IMAGE_REL_ARM_BLX11`](#image_rel_arm_blx11)
- [`IMAGE_REL_ARM_BLX23T`](#image_rel_arm_blx23t) - Thumb: BLX immediate
- [`IMAGE_REL_ARM_BLX24`](#image_rel_arm_blx24)
- [`IMAGE_REL_ARM_BRANCH11`](#image_rel_arm_branch11) - Thumb: 2 11 bit offsets
- [`IMAGE_REL_ARM_BRANCH20T`](#image_rel_arm_branch20t) - Thumb: 32-bit conditional B
- [`IMAGE_REL_ARM_BRANCH24`](#image_rel_arm_branch24) - 24 bit offset << 2 & sign ext.
- [`IMAGE_REL_ARM_BRANCH24T`](#image_rel_arm_branch24t) - Thumb: 32-bit B or BL
- [`IMAGE_REL_ARM_GPREL12`](#image_rel_arm_gprel12) - GP-relative addressing (ARM)
- [`IMAGE_REL_ARM_GPREL7`](#image_rel_arm_gprel7) - GP-relative addressing (Thumb)
- [`IMAGE_REL_ARM_MOV32`](#image_rel_arm_mov32) - ARM: MOVW/MOVT (deprecated)
- [`IMAGE_REL_ARM_MOV32A`](#image_rel_arm_mov32a) - ARM: MOVW/MOVT
- [`IMAGE_REL_ARM_MOV32T`](#image_rel_arm_mov32t) - Thumb: MOVW/MOVT
- [`IMAGE_REL_ARM_REL32`](#image_rel_arm_rel32) - 32-bit relative address from byte following reloc
- [`IMAGE_REL_ARM_SECREL`](#image_rel_arm_secrel) - Offset within section
- [`IMAGE_REL_ARM_SECTION`](#image_rel_arm_section) - Section table index
- [`IMAGE_REL_ARM_TOKEN`](#image_rel_arm_token) - clr token
- [`IMAGE_REL_BASED_ABSOLUTE`](#image_rel_based_absolute)
- [`IMAGE_REL_BASED_ARM_MOV32`](#image_rel_based_arm_mov32)
- [`IMAGE_REL_BASED_DIR64`](#image_rel_based_dir64)
- [`IMAGE_REL_BASED_HIGH`](#image_rel_based_high)
- [`IMAGE_REL_BASED_HIGHADJ`](#image_rel_based_highadj)
- [`IMAGE_REL_BASED_HIGHLOW`](#image_rel_based_highlow)
- [`IMAGE_REL_BASED_IA64_IMM64`](#image_rel_based_ia64_imm64)
- [`IMAGE_REL_BASED_LOW`](#image_rel_based_low)
- [`IMAGE_REL_BASED_MACHINE_SPECIFIC_5`](#image_rel_based_machine_specific_5)
- [`IMAGE_REL_BASED_MACHINE_SPECIFIC_7`](#image_rel_based_machine_specific_7)
- [`IMAGE_REL_BASED_MACHINE_SPECIFIC_8`](#image_rel_based_machine_specific_8)
- [`IMAGE_REL_BASED_MACHINE_SPECIFIC_9`](#image_rel_based_machine_specific_9)
- [`IMAGE_REL_BASED_MIPS_JMPADDR`](#image_rel_based_mips_jmpaddr)
- [`IMAGE_REL_BASED_MIPS_JMPADDR16`](#image_rel_based_mips_jmpaddr16)
- [`IMAGE_REL_BASED_RESERVED`](#image_rel_based_reserved)
- [`IMAGE_REL_BASED_RISCV_HIGH20`](#image_rel_based_riscv_high20)
- [`IMAGE_REL_BASED_RISCV_LOW12I`](#image_rel_based_riscv_low12i)
- [`IMAGE_REL_BASED_RISCV_LOW12S`](#image_rel_based_riscv_low12s)
- [`IMAGE_REL_BASED_THUMB_MOV32`](#image_rel_based_thumb_mov32)
- [`IMAGE_REL_CEE_ABSOLUTE`](#image_rel_cee_absolute) - Reference is absolute, no relocation is necessary
- [`IMAGE_REL_CEE_ADDR32`](#image_rel_cee_addr32) - 32-bit address (VA).
- [`IMAGE_REL_CEE_ADDR32NB`](#image_rel_cee_addr32nb) - 32-bit address w/o image base (RVA).
- [`IMAGE_REL_CEE_ADDR64`](#image_rel_cee_addr64) - 64-bit address (VA).
- [`IMAGE_REL_CEE_SECREL`](#image_rel_cee_secrel) - 32 bit offset from base of section containing target
- [`IMAGE_REL_CEE_SECTION`](#image_rel_cee_section) - Section index
- [`IMAGE_REL_CEE_TOKEN`](#image_rel_cee_token) - 32 bit metadata token
- [`IMAGE_REL_CEF_ABSOLUTE`](#image_rel_cef_absolute) - Reference is absolute, no relocation is necessary
- [`IMAGE_REL_CEF_ADDR32`](#image_rel_cef_addr32) - 32-bit address (VA).
- [`IMAGE_REL_CEF_ADDR32NB`](#image_rel_cef_addr32nb) - 32-bit address w/o image base (RVA).
- [`IMAGE_REL_CEF_ADDR64`](#image_rel_cef_addr64) - 64-bit address (VA).
- [`IMAGE_REL_CEF_SECREL`](#image_rel_cef_secrel) - 32 bit offset from base of section containing target
- [`IMAGE_REL_CEF_SECTION`](#image_rel_cef_section) - Section index
- [`IMAGE_REL_CEF_TOKEN`](#image_rel_cef_token) - 32 bit metadata token
- [`IMAGE_REL_EBC_ABSOLUTE`](#image_rel_ebc_absolute) - No relocation required
- [`IMAGE_REL_EBC_ADDR32NB`](#image_rel_ebc_addr32nb) - 32 bit address w/o image base
- [`IMAGE_REL_EBC_REL32`](#image_rel_ebc_rel32) - 32-bit relative address from byte following reloc
- [`IMAGE_REL_EBC_SECREL`](#image_rel_ebc_secrel) - Offset within section
- [`IMAGE_REL_EBC_SECTION`](#image_rel_ebc_section) - Section table index
- [`IMAGE_REL_I386_ABSOLUTE`](#image_rel_i386_absolute) - Reference is absolute, no relocation is necessary
- [`IMAGE_REL_I386_DIR16`](#image_rel_i386_dir16) - Direct 16-bit reference to the symbols virtual address
- [`IMAGE_REL_I386_DIR32`](#image_rel_i386_dir32) - Direct 32-bit reference to the symbols virtual address
- [`IMAGE_REL_I386_DIR32NB`](#image_rel_i386_dir32nb) - Direct 32-bit reference to the symbols virtual address, base not included
- [`IMAGE_REL_I386_REL16`](#image_rel_i386_rel16) - PC-relative 16-bit reference to the symbols virtual address
- [`IMAGE_REL_I386_REL32`](#image_rel_i386_rel32) - PC-relative 32-bit reference to the symbols virtual address
- [`IMAGE_REL_I386_SECREL`](#image_rel_i386_secrel)
- [`IMAGE_REL_I386_SECREL7`](#image_rel_i386_secrel7) - 7 bit offset from base of section containing target
- [`IMAGE_REL_I386_SECTION`](#image_rel_i386_section)
- [`IMAGE_REL_I386_SEG12`](#image_rel_i386_seg12) - Direct 16-bit reference to the segment-selector bits of a 32-bit virtual address
- [`IMAGE_REL_I386_TOKEN`](#image_rel_i386_token) - clr token
- [`IMAGE_REL_IA64_ABSOLUTE`](#image_rel_ia64_absolute)
- [`IMAGE_REL_IA64_ADDEND`](#image_rel_ia64_addend)
- [`IMAGE_REL_IA64_DIR32`](#image_rel_ia64_dir32)
- [`IMAGE_REL_IA64_DIR32NB`](#image_rel_ia64_dir32nb)
- [`IMAGE_REL_IA64_DIR64`](#image_rel_ia64_dir64)
- [`IMAGE_REL_IA64_GPREL22`](#image_rel_ia64_gprel22)
- [`IMAGE_REL_IA64_GPREL32`](#image_rel_ia64_gprel32)
- [`IMAGE_REL_IA64_IMM14`](#image_rel_ia64_imm14)
- [`IMAGE_REL_IA64_IMM22`](#image_rel_ia64_imm22)
- [`IMAGE_REL_IA64_IMM64`](#image_rel_ia64_imm64)
- [`IMAGE_REL_IA64_IMMGPREL64`](#image_rel_ia64_immgprel64)
- [`IMAGE_REL_IA64_LTOFF22`](#image_rel_ia64_ltoff22)
- [`IMAGE_REL_IA64_PCREL21B`](#image_rel_ia64_pcrel21b)
- [`IMAGE_REL_IA64_PCREL21F`](#image_rel_ia64_pcrel21f)
- [`IMAGE_REL_IA64_PCREL21M`](#image_rel_ia64_pcrel21m)
- [`IMAGE_REL_IA64_PCREL60B`](#image_rel_ia64_pcrel60b) - If possible, convert to MBB bundle with NOP.B in slot 1
- [`IMAGE_REL_IA64_PCREL60F`](#image_rel_ia64_pcrel60f) - If possible, convert to MFB bundle with NOP.F in slot 1
- [`IMAGE_REL_IA64_PCREL60I`](#image_rel_ia64_pcrel60i) - If possible, convert to MIB bundle with NOP.I in slot 1
- [`IMAGE_REL_IA64_PCREL60M`](#image_rel_ia64_pcrel60m) - If possible, convert to MMB bundle with NOP.M in slot 1
- [`IMAGE_REL_IA64_PCREL60X`](#image_rel_ia64_pcrel60x) - This is always a BRL and never converted
- [`IMAGE_REL_IA64_SECREL22`](#image_rel_ia64_secrel22)
- [`IMAGE_REL_IA64_SECREL32`](#image_rel_ia64_secrel32)
- [`IMAGE_REL_IA64_SECREL64I`](#image_rel_ia64_secrel64i)
- [`IMAGE_REL_IA64_SECTION`](#image_rel_ia64_section)
- [`IMAGE_REL_IA64_SREL14`](#image_rel_ia64_srel14)
- [`IMAGE_REL_IA64_SREL22`](#image_rel_ia64_srel22)
- [`IMAGE_REL_IA64_SREL32`](#image_rel_ia64_srel32)
- [`IMAGE_REL_IA64_TOKEN`](#image_rel_ia64_token) - clr token
- [`IMAGE_REL_IA64_UREL32`](#image_rel_ia64_urel32)
- [`IMAGE_REL_M32R_ABSOLUTE`](#image_rel_m32r_absolute) - No relocation required
- [`IMAGE_REL_M32R_ADDR24`](#image_rel_m32r_addr24) - 24 bit address
- [`IMAGE_REL_M32R_ADDR32`](#image_rel_m32r_addr32) - 32 bit address
- [`IMAGE_REL_M32R_ADDR32NB`](#image_rel_m32r_addr32nb) - 32 bit address w/o image base
- [`IMAGE_REL_M32R_GPREL16`](#image_rel_m32r_gprel16) - GP relative addressing
- [`IMAGE_REL_M32R_PAIR`](#image_rel_m32r_pair) - Link HI and LO
- [`IMAGE_REL_M32R_PCREL16`](#image_rel_m32r_pcrel16) - 16 bit offset << 2 & sign ext.
- [`IMAGE_REL_M32R_PCREL24`](#image_rel_m32r_pcrel24) - 24 bit offset << 2 & sign ext.
- [`IMAGE_REL_M32R_PCREL8`](#image_rel_m32r_pcrel8) - 8 bit offset << 2 & sign ext.
- [`IMAGE_REL_M32R_REFHALF`](#image_rel_m32r_refhalf) - 16 MSBs
- [`IMAGE_REL_M32R_REFHI`](#image_rel_m32r_refhi) - 16 MSBs; adj for LSB sign ext.
- [`IMAGE_REL_M32R_REFLO`](#image_rel_m32r_reflo) - 16 LSBs
- [`IMAGE_REL_M32R_SECREL32`](#image_rel_m32r_secrel32) - 32 bit section relative reference
- [`IMAGE_REL_M32R_SECTION`](#image_rel_m32r_section) - Section table index
- [`IMAGE_REL_M32R_TOKEN`](#image_rel_m32r_token) - clr token
- [`IMAGE_REL_MIPS_ABSOLUTE`](#image_rel_mips_absolute) - Reference is absolute, no relocation is necessary
- [`IMAGE_REL_MIPS_GPREL`](#image_rel_mips_gprel)
- [`IMAGE_REL_MIPS_JMPADDR`](#image_rel_mips_jmpaddr)
- [`IMAGE_REL_MIPS_JMPADDR16`](#image_rel_mips_jmpaddr16)
- [`IMAGE_REL_MIPS_LITERAL`](#image_rel_mips_literal)
- [`IMAGE_REL_MIPS_PAIR`](#image_rel_mips_pair)
- [`IMAGE_REL_MIPS_REFHALF`](#image_rel_mips_refhalf)
- [`IMAGE_REL_MIPS_REFHI`](#image_rel_mips_refhi)
- [`IMAGE_REL_MIPS_REFLO`](#image_rel_mips_reflo)
- [`IMAGE_REL_MIPS_REFWORD`](#image_rel_mips_refword)
- [`IMAGE_REL_MIPS_REFWORDNB`](#image_rel_mips_refwordnb)
- [`IMAGE_REL_MIPS_SECREL`](#image_rel_mips_secrel)
- [`IMAGE_REL_MIPS_SECRELHI`](#image_rel_mips_secrelhi) - High 16-bit section relative reference (used for >32k TLS)
- [`IMAGE_REL_MIPS_SECRELLO`](#image_rel_mips_secrello) - Low 16-bit section relative reference (used for >32k TLS)
- [`IMAGE_REL_MIPS_SECTION`](#image_rel_mips_section)
- [`IMAGE_REL_MIPS_TOKEN`](#image_rel_mips_token) - clr token
- [`IMAGE_REL_PPC_ABSOLUTE`](#image_rel_ppc_absolute) - NOP
- [`IMAGE_REL_PPC_ADDR14`](#image_rel_ppc_addr14) - 16-bit address, shifted left 2 (load doubleword)
- [`IMAGE_REL_PPC_ADDR16`](#image_rel_ppc_addr16) - 16-bit address
- [`IMAGE_REL_PPC_ADDR24`](#image_rel_ppc_addr24) - 26-bit address, shifted left 2 (branch absolute)
- [`IMAGE_REL_PPC_ADDR32`](#image_rel_ppc_addr32) - 32-bit address
- [`IMAGE_REL_PPC_ADDR32NB`](#image_rel_ppc_addr32nb) - 32-bit addr w/o image base
- [`IMAGE_REL_PPC_ADDR64`](#image_rel_ppc_addr64) - 64-bit address
- [`IMAGE_REL_PPC_BRNTAKEN`](#image_rel_ppc_brntaken) - fix branch prediction bit to predict branch not taken
- [`IMAGE_REL_PPC_BRTAKEN`](#image_rel_ppc_brtaken) - fix branch prediction bit to predict branch taken
- [`IMAGE_REL_PPC_GPREL`](#image_rel_ppc_gprel)
- [`IMAGE_REL_PPC_IFGLUE`](#image_rel_ppc_ifglue) - substitute TOC restore instruction iff symbol is glue code
- [`IMAGE_REL_PPC_IMGLUE`](#image_rel_ppc_imglue) - symbol is glue code; virtual address is TOC restore instruction
- [`IMAGE_REL_PPC_NEG`](#image_rel_ppc_neg) - subtract reloc value rather than adding it
- [`IMAGE_REL_PPC_PAIR`](#image_rel_ppc_pair)
- [`IMAGE_REL_PPC_REFHI`](#image_rel_ppc_refhi)
- [`IMAGE_REL_PPC_REFLO`](#image_rel_ppc_reflo)
- [`IMAGE_REL_PPC_REL14`](#image_rel_ppc_rel14) - 16-bit PC-relative offset, shifted left 2 (br cond relative)
- [`IMAGE_REL_PPC_REL24`](#image_rel_ppc_rel24) - 26-bit PC-relative offset, shifted left 2 (branch relative)
- [`IMAGE_REL_PPC_SECREL`](#image_rel_ppc_secrel) - va of containing section (as in an image sectionhdr)
- [`IMAGE_REL_PPC_SECREL16`](#image_rel_ppc_secrel16) - va of containing section (limited to 16 bits)
- [`IMAGE_REL_PPC_SECRELHI`](#image_rel_ppc_secrelhi) - High 16-bit section relative reference (used for >32k TLS)
- [`IMAGE_REL_PPC_SECRELLO`](#image_rel_ppc_secrello) - Low 16-bit section relative reference (used for >32k TLS)
- [`IMAGE_REL_PPC_SECTION`](#image_rel_ppc_section) - sectionheader number
- [`IMAGE_REL_PPC_TOCDEFN`](#image_rel_ppc_tocdefn) - toc slot defined in file (or, data in toc)
- [`IMAGE_REL_PPC_TOCREL14`](#image_rel_ppc_tocrel14) - 16-bit offset from TOC base, shifted left 2 (load doubleword)
- [`IMAGE_REL_PPC_TOCREL16`](#image_rel_ppc_tocrel16) - 16-bit offset from TOC base
- [`IMAGE_REL_PPC_TOKEN`](#image_rel_ppc_token) - clr token
- [`IMAGE_REL_PPC_TYPEMASK`](#image_rel_ppc_typemask) - mask to isolate above values in IMAGE_RELOCATION.Type
- [`IMAGE_REL_SH3_ABSOLUTE`](#image_rel_sh3_absolute) - No relocation
- [`IMAGE_REL_SH3_DIRECT16`](#image_rel_sh3_direct16) - 16 bit direct
- [`IMAGE_REL_SH3_DIRECT32`](#image_rel_sh3_direct32) - 32 bit direct
- [`IMAGE_REL_SH3_DIRECT32_NB`](#image_rel_sh3_direct32_nb) - 32 bit direct not based
- [`IMAGE_REL_SH3_DIRECT4`](#image_rel_sh3_direct4) - 4 bit direct (0 ext.)
- [`IMAGE_REL_SH3_DIRECT4_LONG`](#image_rel_sh3_direct4_long) - 4 bit direct .L (0 ext.)
- [`IMAGE_REL_SH3_DIRECT4_WORD`](#image_rel_sh3_direct4_word) - 4 bit direct .W (0 ext.)
- [`IMAGE_REL_SH3_DIRECT8`](#image_rel_sh3_direct8) - 8 bit direct, -128..255
- [`IMAGE_REL_SH3_DIRECT8_LONG`](#image_rel_sh3_direct8_long) - 8 bit direct .L (0 ext.)
- [`IMAGE_REL_SH3_DIRECT8_WORD`](#image_rel_sh3_direct8_word) - 8 bit direct .W (0 ext.)
- [`IMAGE_REL_SH3_GPREL4_LONG`](#image_rel_sh3_gprel4_long) - GP-relative addressing
- [`IMAGE_REL_SH3_PCREL12_WORD`](#image_rel_sh3_pcrel12_word) - 12 LSB PC relative .W
- [`IMAGE_REL_SH3_PCREL8_LONG`](#image_rel_sh3_pcrel8_long) - 8 bit PC relative .L
- [`IMAGE_REL_SH3_PCREL8_WORD`](#image_rel_sh3_pcrel8_word) - 8 bit PC relative .W
- [`IMAGE_REL_SH3_SECREL`](#image_rel_sh3_secrel) - Offset within section
- [`IMAGE_REL_SH3_SECTION`](#image_rel_sh3_section) - Section table index
- [`IMAGE_REL_SH3_SIZEOF_SECTION`](#image_rel_sh3_sizeof_section) - Size of EXE section
- [`IMAGE_REL_SH3_STARTOF_SECTION`](#image_rel_sh3_startof_section) - Start of EXE section
- [`IMAGE_REL_SH3_TOKEN`](#image_rel_sh3_token) - clr token
- [`IMAGE_REL_SHM_PAIR`](#image_rel_shm_pair) - offset operand for relocation
- [`IMAGE_REL_SHM_PCRELPT`](#image_rel_shm_pcrelpt) - Offset from current instruction in longwords
- [`IMAGE_REL_SHM_REFHALF`](#image_rel_shm_refhalf) - High bits of 32-bit address
- [`IMAGE_REL_SHM_REFLO`](#image_rel_shm_reflo) - Low bits of 32-bit address
- [`IMAGE_REL_SHM_RELHALF`](#image_rel_shm_relhalf) - High bits of relative reference
- [`IMAGE_REL_SHM_RELLO`](#image_rel_shm_rello) - Low bits of relative reference
- [`IMAGE_REL_SH_NOMODE`](#image_rel_sh_nomode) - relocation ignores section mode
- [`IMAGE_REL_THUMB_BLX23`](#image_rel_thumb_blx23) - Thumb: BLX immediate (deprecated)
- [`IMAGE_REL_THUMB_BRANCH20`](#image_rel_thumb_branch20) - Thumb: 32-bit conditional B (deprecated)
- [`IMAGE_REL_THUMB_BRANCH24`](#image_rel_thumb_branch24) - Thumb: 32-bit B or BL (deprecated)
- [`IMAGE_REL_THUMB_MOV32`](#image_rel_thumb_mov32) - Thumb: MOVW/MOVT (deprecated)
- [`IMAGE_RESOURCE_DATA_IS_DIRECTORY`](#image_resource_data_is_directory)
- [`IMAGE_RESOURCE_NAME_IS_STRING`](#image_resource_name_is_string)
- [`IMAGE_ROM_OPTIONAL_HDR_MAGIC`](#image_rom_optional_hdr_magic)
- [`IMAGE_SCN_ALIGN_1024BYTES`](#image_scn_align_1024bytes)
- [`IMAGE_SCN_ALIGN_128BYTES`](#image_scn_align_128bytes)
- [`IMAGE_SCN_ALIGN_16BYTES`](#image_scn_align_16bytes) - Default alignment if no others are specified.
- [`IMAGE_SCN_ALIGN_1BYTES`](#image_scn_align_1bytes)
- [`IMAGE_SCN_ALIGN_2048BYTES`](#image_scn_align_2048bytes)
- [`IMAGE_SCN_ALIGN_256BYTES`](#image_scn_align_256bytes)
- [`IMAGE_SCN_ALIGN_2BYTES`](#image_scn_align_2bytes)
- [`IMAGE_SCN_ALIGN_32BYTES`](#image_scn_align_32bytes)
- [`IMAGE_SCN_ALIGN_4096BYTES`](#image_scn_align_4096bytes)
- [`IMAGE_SCN_ALIGN_4BYTES`](#image_scn_align_4bytes)
- [`IMAGE_SCN_ALIGN_512BYTES`](#image_scn_align_512bytes)
- [`IMAGE_SCN_ALIGN_64BYTES`](#image_scn_align_64bytes)
- [`IMAGE_SCN_ALIGN_8192BYTES`](#image_scn_align_8192bytes)
- [`IMAGE_SCN_ALIGN_8BYTES`](#image_scn_align_8bytes)
- [`IMAGE_SCN_ALIGN_MASK`](#image_scn_align_mask)
- [`IMAGE_SCN_CNT_CODE`](#image_scn_cnt_code) - Section contains code.
- [`IMAGE_SCN_CNT_INITIALIZED_DATA`](#image_scn_cnt_initialized_data) - Section contains initialized data.
- [`IMAGE_SCN_CNT_UNINITIALIZED_DATA`](#image_scn_cnt_uninitialized_data) - Section contains uninitialized data.
- [`IMAGE_SCN_GPREL`](#image_scn_gprel) - Section content can be accessed relative to GP
- [`IMAGE_SCN_LNK_COMDAT`](#image_scn_lnk_comdat) - Section contents comdat.
- [`IMAGE_SCN_LNK_INFO`](#image_scn_lnk_info) - Section contains comments or some other type of information.
- [`IMAGE_SCN_LNK_NRELOC_OVFL`](#image_scn_lnk_nreloc_ovfl) - Section contains extended relocations.
- [`IMAGE_SCN_LNK_OTHER`](#image_scn_lnk_other) - Reserved.
- [`IMAGE_SCN_LNK_REMOVE`](#image_scn_lnk_remove) - Section contents will not become part of image.
- [`IMAGE_SCN_MEM_16BIT`](#image_scn_mem_16bit)
- [`IMAGE_SCN_MEM_DISCARDABLE`](#image_scn_mem_discardable) - Section can be discarded.
- [`IMAGE_SCN_MEM_EXECUTE`](#image_scn_mem_execute) - Section is executable.
- [`IMAGE_SCN_MEM_FARDATA`](#image_scn_mem_fardata)
- [`IMAGE_SCN_MEM_LOCKED`](#image_scn_mem_locked)
- [`IMAGE_SCN_MEM_NOT_CACHED`](#image_scn_mem_not_cached) - Section is not cacheable.
- [`IMAGE_SCN_MEM_NOT_PAGED`](#image_scn_mem_not_paged) - Section is not pageable.
- [`IMAGE_SCN_MEM_PRELOAD`](#image_scn_mem_preload)
- [`IMAGE_SCN_MEM_PURGEABLE`](#image_scn_mem_purgeable)
- [`IMAGE_SCN_MEM_READ`](#image_scn_mem_read) - Section is readable.
- [`IMAGE_SCN_MEM_SHARED`](#image_scn_mem_shared) - Section is shareable.
- [`IMAGE_SCN_MEM_WRITE`](#image_scn_mem_write) - Section is writeable.
- [`IMAGE_SCN_NO_DEFER_SPEC_EXC`](#image_scn_no_defer_spec_exc) - Reset speculative exceptions handling bits in the TLB entries for this section.
- [`IMAGE_SCN_SCALE_INDEX`](#image_scn_scale_index) - Tls index is scaled
- [`IMAGE_SCN_TYPE_NO_PAD`](#image_scn_type_no_pad) - Reserved.
- [`IMAGE_SEPARATE_DEBUG_FLAGS_MASK`](#image_separate_debug_flags_mask)
- [`IMAGE_SEPARATE_DEBUG_MISMATCH`](#image_separate_debug_mismatch) - when DBG was updated, the old checksum didn't match.
- [`IMAGE_SEPARATE_DEBUG_SIGNATURE`](#image_separate_debug_signature)
- [`IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR`](#image_sizeof_archive_member_hdr)
- [`IMAGE_SIZEOF_FILE_HEADER`](#image_sizeof_file_header)
- [`IMAGE_SIZEOF_SECTION_HEADER`](#image_sizeof_section_header)
- [`IMAGE_SIZEOF_SHORT_NAME`](#image_sizeof_short_name)
- [`IMAGE_SIZEOF_SYMBOL`](#image_sizeof_symbol)
- [`IMAGE_SIZEOF_SYMBOL_EX`](#image_sizeof_symbol_ex)
- [`IMAGE_SUBSYSTEM_EFI_APPLICATION`](#image_subsystem_efi_application)
- [`IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER`](#image_subsystem_efi_boot_service_driver)
- [`IMAGE_SUBSYSTEM_EFI_ROM`](#image_subsystem_efi_rom)
- [`IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER`](#image_subsystem_efi_runtime_driver)
- [`IMAGE_SUBSYSTEM_NATIVE`](#image_subsystem_native) - Image doesn't require a subsystem.
- [`IMAGE_SUBSYSTEM_NATIVE_WINDOWS`](#image_subsystem_native_windows) - image is a native Win9x driver.
- [`IMAGE_SUBSYSTEM_OS2_CUI`](#image_subsystem_os2_cui) - image runs in the OS/2 character subsystem.
- [`IMAGE_SUBSYSTEM_POSIX_CUI`](#image_subsystem_posix_cui) - image runs in the Posix character subsystem.
- [`IMAGE_SUBSYSTEM_UNKNOWN`](#image_subsystem_unknown) - Unknown subsystem.
- [`IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION`](#image_subsystem_windows_boot_application)
- [`IMAGE_SUBSYSTEM_WINDOWS_CE_GUI`](#image_subsystem_windows_ce_gui) - Image runs in the Windows CE subsystem.
- [`IMAGE_SUBSYSTEM_WINDOWS_CUI`](#image_subsystem_windows_cui) - Image runs in the Windows character subsystem.
- [`IMAGE_SUBSYSTEM_WINDOWS_GUI`](#image_subsystem_windows_gui) - Image runs in the Windows GUI subsystem.
- [`IMAGE_SUBSYSTEM_XBOX`](#image_subsystem_xbox)
- [`IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG`](#image_subsystem_xbox_code_catalog)
- [`IMAGE_SYM_ABSOLUTE`](#image_sym_absolute) - Symbol is an absolute value.
- [`IMAGE_SYM_CLASS_ARGUMENT`](#image_sym_class_argument)
- [`IMAGE_SYM_CLASS_AUTOMATIC`](#image_sym_class_automatic)
- [`IMAGE_SYM_CLASS_BIT_FIELD`](#image_sym_class_bit_field)
- [`IMAGE_SYM_CLASS_BLOCK`](#image_sym_class_block)
- [`IMAGE_SYM_CLASS_CLR_TOKEN`](#image_sym_class_clr_token)
- [`IMAGE_SYM_CLASS_END_OF_FUNCTION`](#image_sym_class_end_of_function)
- [`IMAGE_SYM_CLASS_END_OF_STRUCT`](#image_sym_class_end_of_struct)
- [`IMAGE_SYM_CLASS_ENUM_TAG`](#image_sym_class_enum_tag)
- [`IMAGE_SYM_CLASS_EXTERNAL`](#image_sym_class_external)
- [`IMAGE_SYM_CLASS_EXTERNAL_DEF`](#image_sym_class_external_def)
- [`IMAGE_SYM_CLASS_FAR_EXTERNAL`](#image_sym_class_far_external)
- [`IMAGE_SYM_CLASS_FILE`](#image_sym_class_file)
- [`IMAGE_SYM_CLASS_FUNCTION`](#image_sym_class_function)
- [`IMAGE_SYM_CLASS_LABEL`](#image_sym_class_label)
- [`IMAGE_SYM_CLASS_MEMBER_OF_ENUM`](#image_sym_class_member_of_enum)
- [`IMAGE_SYM_CLASS_MEMBER_OF_STRUCT`](#image_sym_class_member_of_struct)
- [`IMAGE_SYM_CLASS_MEMBER_OF_UNION`](#image_sym_class_member_of_union)
- [`IMAGE_SYM_CLASS_NULL`](#image_sym_class_null)
- [`IMAGE_SYM_CLASS_REGISTER`](#image_sym_class_register)
- [`IMAGE_SYM_CLASS_REGISTER_PARAM`](#image_sym_class_register_param)
- [`IMAGE_SYM_CLASS_SECTION`](#image_sym_class_section)
- [`IMAGE_SYM_CLASS_STATIC`](#image_sym_class_static)
- [`IMAGE_SYM_CLASS_STRUCT_TAG`](#image_sym_class_struct_tag)
- [`IMAGE_SYM_CLASS_TYPE_DEFINITION`](#image_sym_class_type_definition)
- [`IMAGE_SYM_CLASS_UNDEFINED_LABEL`](#image_sym_class_undefined_label)
- [`IMAGE_SYM_CLASS_UNDEFINED_STATIC`](#image_sym_class_undefined_static)
- [`IMAGE_SYM_CLASS_UNION_TAG`](#image_sym_class_union_tag)
- [`IMAGE_SYM_CLASS_WEAK_EXTERNAL`](#image_sym_class_weak_external)
- [`IMAGE_SYM_DEBUG`](#image_sym_debug) - Symbol is a special debug item.
- [`IMAGE_SYM_DTYPE_ARRAY`](#image_sym_dtype_array) - array.
- [`IMAGE_SYM_DTYPE_FUNCTION`](#image_sym_dtype_function) - function.
- [`IMAGE_SYM_DTYPE_NULL`](#image_sym_dtype_null) - no derived type.
- [`IMAGE_SYM_DTYPE_POINTER`](#image_sym_dtype_pointer) - pointer.
- [`IMAGE_SYM_DTYPE_SHIFT`](#image_sym_dtype_shift)
- [`IMAGE_SYM_SECTION_MAX`](#image_sym_section_max) - Values 0xFF00-0xFFFF are special
- [`IMAGE_SYM_SECTION_MAX_EX`](#image_sym_section_max_ex)
- [`IMAGE_SYM_TYPE_BYTE`](#image_sym_type_byte)
- [`IMAGE_SYM_TYPE_CHAR`](#image_sym_type_char) - type character.
- [`IMAGE_SYM_TYPE_DOUBLE`](#image_sym_type_double)
- [`IMAGE_SYM_TYPE_DWORD`](#image_sym_type_dword)
- [`IMAGE_SYM_TYPE_ENUM`](#image_sym_type_enum) - enumeration.
- [`IMAGE_SYM_TYPE_FLOAT`](#image_sym_type_float)
- [`IMAGE_SYM_TYPE_INT`](#image_sym_type_int)
- [`IMAGE_SYM_TYPE_LONG`](#image_sym_type_long)
- [`IMAGE_SYM_TYPE_MOE`](#image_sym_type_moe) - member of enumeration.
- [`IMAGE_SYM_TYPE_NULL`](#image_sym_type_null) - no type.
- [`IMAGE_SYM_TYPE_PCODE`](#image_sym_type_pcode)
- [`IMAGE_SYM_TYPE_SHORT`](#image_sym_type_short) - type short integer.
- [`IMAGE_SYM_TYPE_STRUCT`](#image_sym_type_struct)
- [`IMAGE_SYM_TYPE_UINT`](#image_sym_type_uint)
- [`IMAGE_SYM_TYPE_UNION`](#image_sym_type_union)
- [`IMAGE_SYM_TYPE_VOID`](#image_sym_type_void)
- [`IMAGE_SYM_TYPE_WORD`](#image_sym_type_word)
- [`IMAGE_SYM_UNDEFINED`](#image_sym_undefined) - Symbol is undefined or is common.
- [`IMAGE_VXD_SIGNATURE`](#image_vxd_signature) - LE
- [`IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY`](#image_weak_extern_anti_dependency)
- [`IMAGE_WEAK_EXTERN_SEARCH_ALIAS`](#image_weak_extern_search_alias)
- [`IMAGE_WEAK_EXTERN_SEARCH_LIBRARY`](#image_weak_extern_search_library)
- [`IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY`](#image_weak_extern_search_nolibrary)
- [`IMPORT_OBJECT_CODE`](#import_object_code)
- [`IMPORT_OBJECT_CONST`](#import_object_const)
- [`IMPORT_OBJECT_DATA`](#import_object_data)
- [`IMPORT_OBJECT_HDR_SIG2`](#import_object_hdr_sig2)
- [`IMPORT_OBJECT_NAME`](#import_object_name) - Import name == public symbol name.
- [`IMPORT_OBJECT_NAME_EXPORTAS`](#import_object_name_exportas) - Import name == a name is explicitly provided after the DLL name.
- [`IMPORT_OBJECT_NAME_MASK`](#import_object_name_mask)
- [`IMPORT_OBJECT_NAME_NO_PREFIX`](#import_object_name_no_prefix) - Import name == public symbol name skipping leading ?, @, or optionally _.
- [`IMPORT_OBJECT_NAME_SHIFT`](#import_object_name_shift)
- [`IMPORT_OBJECT_NAME_UNDECORATE`](#import_object_name_undecorate) - Import name == public symbol name skipping leading ?, @, or optionally _ and truncating at first @.
- [`IMPORT_OBJECT_ORDINAL`](#import_object_ordinal) - Import by ordinal
- [`IMPORT_OBJECT_TYPE_MASK`](#import_object_type_mask)
- [`IMPORT_OBJECT_TYPE_SHIFT`](#import_object_type_shift)
- [`MAX_CLASS_NAME`](#max_class_name)
- [`MAX_PACKAGE_NAME`](#max_package_name)
- [`NATIVE_TYPE_MAX_CB`](#native_type_max_cb)
- [`NON_PAGED_DEBUG_SIGNATURE`](#non_paged_debug_signature)
- [`N_BTMASK`](#n_btmask)
- [`N_BTSHFT`](#n_btshft)
- [`N_TMASK`](#n_tmask)
- [`N_TMASK1`](#n_tmask1)
- [`N_TMASK2`](#n_tmask2)
- [`N_TSHIFT`](#n_tshift)
- [`RT_ACCELERATOR`](#rt_accelerator) - ID for: Accelerator table.
- [`RT_ANICURSOR`](#rt_anicursor) - ID for: Animated cursor.
- [`RT_ANIICON`](#rt_aniicon) - ID for: Animated icon.
- [`RT_BITMAP`](#rt_bitmap) - ID for: Bitmap resource.
- [`RT_CURSOR`](#rt_cursor) - ID for: Hardware-dependent cursor resource.
- [`RT_DIALOG`](#rt_dialog) - ID for: Dialog box.
- [`RT_DLGINCLUDE`](#rt_dlginclude) - ID for: Allows a resource editing tool to associate a string with an .rc file.
- [`RT_FONT`](#rt_font) - ID for: Font resource.
- [`RT_FONTDIR`](#rt_fontdir) - ID for: Font directory resource.
- [`RT_GROUP_CURSOR`](#rt_group_cursor) - ID for: Hardware-independent cursor resource.
- [`RT_GROUP_ICON`](#rt_group_icon) - ID for: Hardware-independent icon resource.
- [`RT_HTML`](#rt_html) - ID for: HTML resource.
- [`RT_ICON`](#rt_icon) - ID for: Hardware-dependent icon resource.
- [`RT_MANIFEST`](#rt_manifest) - ID for: Side-by-Side Assembly Manifest.
- [`RT_MENU`](#rt_menu) - ID for: Menu resource.
- [`RT_MESSAGETABLE`](#rt_messagetable) - ID for: Message-table entry.
- [`RT_PLUGPLAY`](#rt_plugplay) - ID for: Plug and Play resource.
- [`RT_RCDATA`](#rt_rcdata) - ID for: Application-defined resource (raw data).
- [`RT_STRING`](#rt_string) - ID for: String-table entry.
- [`RT_VERSION`](#rt_version) - ID for: Version resource.
- [`RT_VXD`](#rt_vxd) - ID for: VXD.
- [`X3_BTYPE_QP_INST_VAL_POS_X`](#x3_btype_qp_inst_val_pos_x) - Intel-IA64-Filler
- [`X3_BTYPE_QP_INST_WORD_POS_X`](#x3_btype_qp_inst_word_pos_x) - Intel-IA64-Filler
- [`X3_BTYPE_QP_INST_WORD_X`](#x3_btype_qp_inst_word_x) - Intel-IA64-Filler
- [`X3_BTYPE_QP_SIZE_X`](#x3_btype_qp_size_x) - Intel-IA64-Filler
- [`X3_D_WH_INST_WORD_POS_X`](#x3_d_wh_inst_word_pos_x) - Intel-IA64-Filler
- [`X3_D_WH_INST_WORD_X`](#x3_d_wh_inst_word_x) - Intel-IA64-Filler
- [`X3_D_WH_SIGN_VAL_POS_X`](#x3_d_wh_sign_val_pos_x) - Intel-IA64-Filler
- [`X3_D_WH_SIZE_X`](#x3_d_wh_size_x) - Intel-IA64-Filler
- [`X3_EMPTY_INST_VAL_POS_X`](#x3_empty_inst_val_pos_x) - Intel-IA64-Filler
- [`X3_EMPTY_INST_WORD_POS_X`](#x3_empty_inst_word_pos_x) - Intel-IA64-Filler
- [`X3_EMPTY_INST_WORD_X`](#x3_empty_inst_word_x) - Intel-IA64-Filler
- [`X3_EMPTY_SIZE_X`](#x3_empty_size_x) - Intel-IA64-Filler
- [`X3_IMM20_INST_WORD_POS_X`](#x3_imm20_inst_word_pos_x) - Intel-IA64-Filler
- [`X3_IMM20_INST_WORD_X`](#x3_imm20_inst_word_x) - Intel-IA64-Filler
- [`X3_IMM20_SIGN_VAL_POS_X`](#x3_imm20_sign_val_pos_x) - Intel-IA64-Filler
- [`X3_IMM20_SIZE_X`](#x3_imm20_size_x) - Intel-IA64-Filler
- [`X3_IMM39_1_INST_WORD_POS_X`](#x3_imm39_1_inst_word_pos_x) - Intel-IA64-Filler
- [`X3_IMM39_1_INST_WORD_X`](#x3_imm39_1_inst_word_x) - Intel-IA64-Filler
- [`X3_IMM39_1_SIGN_VAL_POS_X`](#x3_imm39_1_sign_val_pos_x) - Intel-IA64-Filler
- [`X3_IMM39_1_SIZE_X`](#x3_imm39_1_size_x) - Intel-IA64-Filler
- [`X3_IMM39_2_INST_WORD_POS_X`](#x3_imm39_2_inst_word_pos_x) - Intel-IA64-Filler
- [`X3_IMM39_2_INST_WORD_X`](#x3_imm39_2_inst_word_x) - Intel-IA64-Filler
- [`X3_IMM39_2_SIGN_VAL_POS_X`](#x3_imm39_2_sign_val_pos_x) - Intel-IA64-Filler
- [`X3_IMM39_2_SIZE_X`](#x3_imm39_2_size_x) - Intel-IA64-Filler
- [`X3_I_INST_WORD_POS_X`](#x3_i_inst_word_pos_x) - Intel-IA64-Filler
- [`X3_I_INST_WORD_X`](#x3_i_inst_word_x) - Intel-IA64-Filler
- [`X3_I_SIGN_VAL_POS_X`](#x3_i_sign_val_pos_x) - Intel-IA64-Filler
- [`X3_I_SIZE_X`](#x3_i_size_x) - Intel-IA64-Filler
- [`X3_OPCODE_INST_WORD_POS_X`](#x3_opcode_inst_word_pos_x) - Intel-IA64-Filler
- [`X3_OPCODE_INST_WORD_X`](#x3_opcode_inst_word_x) - Intel-IA64-Filler
- [`X3_OPCODE_SIGN_VAL_POS_X`](#x3_opcode_sign_val_pos_x) - Intel-IA64-Filler
- [`X3_OPCODE_SIZE_X`](#x3_opcode_size_x) - Intel-IA64-Filler
- [`X3_P_INST_WORD_POS_X`](#x3_p_inst_word_pos_x) - Intel-IA64-Filler
- [`X3_P_INST_WORD_X`](#x3_p_inst_word_x) - Intel-IA64-Filler
- [`X3_P_SIGN_VAL_POS_X`](#x3_p_sign_val_pos_x) - Intel-IA64-Filler
- [`X3_P_SIZE_X`](#x3_p_size_x) - Intel-IA64-Filler
- [`X3_TMPLT_INST_WORD_POS_X`](#x3_tmplt_inst_word_pos_x) - Intel-IA64-Filler
- [`X3_TMPLT_INST_WORD_X`](#x3_tmplt_inst_word_x) - Intel-IA64-Filler
- [`X3_TMPLT_SIGN_VAL_POS_X`](#x3_tmplt_sign_val_pos_x) - Intel-IA64-Filler
- [`X3_TMPLT_SIZE_X`](#x3_tmplt_size_x) - Intel-IA64-Filler

---

## object::pe::ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID

*Constant*: `ClsId`

The required value of `AnonObjectHeaderBigobj::class_id`.



## object::pe::AnonObjectHeader

*Struct*

Non-COFF Object file header

**Fields:**
- `sig1: crate::endian::U16<crate::endian::LittleEndian>` - Must be IMAGE_FILE_MACHINE_UNKNOWN
- `sig2: crate::endian::U16<crate::endian::LittleEndian>` - Must be 0xffff
- `version: crate::endian::U16<crate::endian::LittleEndian>` - >= 1 (implies the ClsId field is present)
- `machine: crate::endian::U16<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `class_id: ClsId` - Used to invoke CoCreateInstance
- `size_of_data: crate::endian::U32<crate::endian::LittleEndian>` - Size of data that follows the header

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AnonObjectHeader`



## object::pe::AnonObjectHeaderBigobj

*Struct*

**Fields:**
- `sig1: crate::endian::U16<crate::endian::LittleEndian>` - Must be IMAGE_FILE_MACHINE_UNKNOWN
- `sig2: crate::endian::U16<crate::endian::LittleEndian>` - Must be 0xffff
- `version: crate::endian::U16<crate::endian::LittleEndian>` - >= 2 (implies the Flags field is present)
- `machine: crate::endian::U16<crate::endian::LittleEndian>` - Actual machine - IMAGE_FILE_MACHINE_xxx
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `class_id: ClsId` - Must be `ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID`.
- `size_of_data: crate::endian::U32<crate::endian::LittleEndian>` - Size of data that follows the header
- `flags: crate::endian::U32<crate::endian::LittleEndian>` - 0x1 -> contains metadata
- `meta_data_size: crate::endian::U32<crate::endian::LittleEndian>` - Size of CLR metadata
- `meta_data_offset: crate::endian::U32<crate::endian::LittleEndian>` - Offset of CLR metadata
- `number_of_sections: crate::endian::U32<crate::endian::LittleEndian>` - extended from WORD
- `pointer_to_symbol_table: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_symbols: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AnonObjectHeaderBigobj`
- **CoffHeader**
  - `fn is_type_bigobj() -> bool`
  - `fn machine(self: &Self) -> u16`
  - `fn number_of_sections(self: &Self) -> u32`
  - `fn pointer_to_symbol_table(self: &Self) -> u32`
  - `fn number_of_symbols(self: &Self) -> u32`
  - `fn characteristics(self: &Self) -> u16`
  - `fn parse<'data, R>(data: R, offset: & mut u64) -> read::Result<&'data Self>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::AnonObjectHeaderV2

*Struct*

**Fields:**
- `sig1: crate::endian::U16<crate::endian::LittleEndian>` - Must be IMAGE_FILE_MACHINE_UNKNOWN
- `sig2: crate::endian::U16<crate::endian::LittleEndian>` - Must be 0xffff
- `version: crate::endian::U16<crate::endian::LittleEndian>` - >= 2 (implies the Flags field is present - otherwise V1)
- `machine: crate::endian::U16<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `class_id: ClsId` - Used to invoke CoCreateInstance
- `size_of_data: crate::endian::U32<crate::endian::LittleEndian>` - Size of data that follows the header
- `flags: crate::endian::U32<crate::endian::LittleEndian>` - 0x1 -> contains metadata
- `meta_data_size: crate::endian::U32<crate::endian::LittleEndian>` - Size of CLR metadata
- `meta_data_offset: crate::endian::U32<crate::endian::LittleEndian>` - Offset of CLR metadata

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AnonObjectHeaderV2`



## object::pe::COMIMAGE_FLAGS_32BITPREFERRED

*Constant*: `u32`



## object::pe::COMIMAGE_FLAGS_32BITREQUIRED

*Constant*: `u32`



## object::pe::COMIMAGE_FLAGS_ILONLY

*Constant*: `u32`



## object::pe::COMIMAGE_FLAGS_IL_LIBRARY

*Constant*: `u32`



## object::pe::COMIMAGE_FLAGS_NATIVE_ENTRYPOINT

*Constant*: `u32`



## object::pe::COMIMAGE_FLAGS_STRONGNAMESIGNED

*Constant*: `u32`



## object::pe::COMIMAGE_FLAGS_TRACKDEBUGDATA

*Constant*: `u32`



## object::pe::COR_DELETED_NAME_LENGTH

*Constant*: `usize`



## object::pe::COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE

*Constant*: `u16`



## object::pe::COR_VERSION_MAJOR

*Constant*: `u16`



## object::pe::COR_VERSION_MAJOR_V2

*Constant*: `u16`



## object::pe::COR_VERSION_MINOR

*Constant*: `u16`



## object::pe::COR_VTABLEGAP_NAME_LENGTH

*Constant*: `usize`



## object::pe::COR_VTABLE_32BIT

*Constant*: `u16`

V-table slots are 32-bits in size.



## object::pe::COR_VTABLE_64BIT

*Constant*: `u16`

V-table slots are 64-bits in size.



## object::pe::COR_VTABLE_CALL_MOST_DERIVED

*Constant*: `u16`

Call most derived method described by



## object::pe::COR_VTABLE_FROM_UNMANAGED

*Constant*: `u16`

If set, transition from unmanaged.



## object::pe::COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN

*Constant*: `u16`

If set, transition from unmanaged with keeping the current appdomain.



## object::pe::EMARCH_ENC_I17_IC_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IC_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IC_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IC_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41A_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41A_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41A_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41A_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41B_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41B_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41B_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41B_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41C_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41C_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41C_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM41C_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM5C_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM5C_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM5C_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM7B_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM7B_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM7B_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM9D_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM9D_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_IMM9D_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_SIGN_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_SIGN_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_SIGN_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::EMARCH_ENC_I17_SIGN_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::FRAME_FPO

*Constant*: `u16`



## object::pe::FRAME_NONFPO

*Constant*: `u16`



## object::pe::FRAME_TRAP

*Constant*: `u16`



## object::pe::FRAME_TSS

*Constant*: `u16`



## object::pe::Guid

*Struct*

**Tuple Struct**: `([u8; 16])`

**Methods:**

- `fn data1(self: Self) -> U32<LE>`
- `fn data2(self: Self) -> U16<LE>`
- `fn data3(self: Self) -> U16<LE>`
- `fn data4(self: Self) -> [u8; 8]`

**Traits:** Eq, Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Guid) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Guid`



## object::pe::IMAGE_ARCHIVE_END

*Constant*: `&[u8]`



## object::pe::IMAGE_ARCHIVE_HYBRIDMAP_MEMBER

*Constant*: `&[u8; 16]`



## object::pe::IMAGE_ARCHIVE_LINKER_MEMBER

*Constant*: `&[u8; 16]`



## object::pe::IMAGE_ARCHIVE_LONGNAMES_MEMBER

*Constant*: `&[u8; 16]`



## object::pe::IMAGE_ARCHIVE_PAD

*Constant*: `&[u8]`



## object::pe::IMAGE_ARCHIVE_START

*Constant*: `&[u8; 8]`



## object::pe::IMAGE_ARCHIVE_START_SIZE

*Constant*: `usize`



## object::pe::IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF

*Constant*: `u16`



## object::pe::IMAGE_COMDAT_SELECT_ANY

*Constant*: `u8`



## object::pe::IMAGE_COMDAT_SELECT_ASSOCIATIVE

*Constant*: `u8`



## object::pe::IMAGE_COMDAT_SELECT_EXACT_MATCH

*Constant*: `u8`



## object::pe::IMAGE_COMDAT_SELECT_LARGEST

*Constant*: `u8`



## object::pe::IMAGE_COMDAT_SELECT_NEWEST

*Constant*: `u8`



## object::pe::IMAGE_COMDAT_SELECT_NODUPLICATES

*Constant*: `u8`



## object::pe::IMAGE_COMDAT_SELECT_SAME_SIZE

*Constant*: `u8`



## object::pe::IMAGE_COR_EATJ_THUNK_SIZE

*Constant*: `usize`

Size of a jump thunk reserved range.



## object::pe::IMAGE_COR_MIH_BASICBLOCK

*Constant*: `u16`



## object::pe::IMAGE_COR_MIH_EHRVA

*Constant*: `u16`



## object::pe::IMAGE_COR_MIH_METHODRVA

*Constant*: `u16`



## object::pe::IMAGE_DEBUG_MISC_EXENAME

*Constant*: `u16`



## object::pe::IMAGE_DEBUG_TYPE_BORLAND

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_CLSID

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_CODEVIEW

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_COFF

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_EXCEPTION

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_FIXUP

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_FPO

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_ILTCG

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_MISC

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_MPX

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_OMAP_FROM_SRC

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_OMAP_TO_SRC

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_POGO

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_REPRO

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_RESERVED10

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_UNKNOWN

*Constant*: `u32`



## object::pe::IMAGE_DEBUG_TYPE_VC_FEATURE

*Constant*: `u32`



## object::pe::IMAGE_DELAYLOAD_RVA_BASED

*Constant*: `u32`

Delay load version 2 flag for `ImageDelayloadDescriptor::attributes`.



## object::pe::IMAGE_DIRECTORY_ENTRY_ARCHITECTURE

*Constant*: `usize`

Architecture Specific Data



## object::pe::IMAGE_DIRECTORY_ENTRY_BASERELOC

*Constant*: `usize`

Base Relocation Table



## object::pe::IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT

*Constant*: `usize`

Bound Import Directory in headers



## object::pe::IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR

*Constant*: `usize`

COM Runtime descriptor



## object::pe::IMAGE_DIRECTORY_ENTRY_DEBUG

*Constant*: `usize`

Debug Directory



## object::pe::IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT

*Constant*: `usize`

Delay Load Import Descriptors



## object::pe::IMAGE_DIRECTORY_ENTRY_EXCEPTION

*Constant*: `usize`

Exception Directory



## object::pe::IMAGE_DIRECTORY_ENTRY_EXPORT

*Constant*: `usize`

Export Directory



## object::pe::IMAGE_DIRECTORY_ENTRY_GLOBALPTR

*Constant*: `usize`

RVA of GP



## object::pe::IMAGE_DIRECTORY_ENTRY_IAT

*Constant*: `usize`

Import Address Table



## object::pe::IMAGE_DIRECTORY_ENTRY_IMPORT

*Constant*: `usize`

Import Directory



## object::pe::IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG

*Constant*: `usize`

Load Configuration Directory



## object::pe::IMAGE_DIRECTORY_ENTRY_RESOURCE

*Constant*: `usize`

Resource Directory



## object::pe::IMAGE_DIRECTORY_ENTRY_SECURITY

*Constant*: `usize`

Security Directory



## object::pe::IMAGE_DIRECTORY_ENTRY_TLS

*Constant*: `usize`

TLS Directory



## object::pe::IMAGE_DLLCHARACTERISTICS_APPCONTAINER

*Constant*: `u16`

Image should execute in an AppContainer



## object::pe::IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE

*Constant*: `u16`

DLL can move.



## object::pe::IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY

*Constant*: `u16`

Code Integrity Image



## object::pe::IMAGE_DLLCHARACTERISTICS_GUARD_CF

*Constant*: `u16`

Image supports Control Flow Guard.



## object::pe::IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA

*Constant*: `u16`

Image can handle a high entropy 64-bit virtual address space.



## object::pe::IMAGE_DLLCHARACTERISTICS_NO_BIND

*Constant*: `u16`

Do not bind this image.



## object::pe::IMAGE_DLLCHARACTERISTICS_NO_ISOLATION

*Constant*: `u16`

Image understands isolation and doesn't want it



## object::pe::IMAGE_DLLCHARACTERISTICS_NO_SEH

*Constant*: `u16`

Image does not use SEH.  No SE handler may reside in this image



## object::pe::IMAGE_DLLCHARACTERISTICS_NX_COMPAT

*Constant*: `u16`

Image is NX compatible



## object::pe::IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE

*Constant*: `u16`



## object::pe::IMAGE_DLLCHARACTERISTICS_WDM_DRIVER

*Constant*: `u16`

Driver uses WDM model



## object::pe::IMAGE_DOS_SIGNATURE

*Constant*: `u16`

MZ



## object::pe::IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER

*Constant*: `u32`



## object::pe::IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER

*Constant*: `u32`



## object::pe::IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE

*Constant*: `u32`



## object::pe::IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE

*Constant*: `u32`



## object::pe::IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH

*Constant*: `u32`



## object::pe::IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE

*Constant*: `u32`



## object::pe::IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID

*Constant*: `u32`



## object::pe::IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID

*Constant*: `u32`



## object::pe::IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID

*Constant*: `u32`



## object::pe::IMAGE_ENCLAVE_IMPORT_MATCH_NONE

*Constant*: `u32`



## object::pe::IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID

*Constant*: `u32`



## object::pe::IMAGE_ENCLAVE_LONG_ID_LENGTH

*Constant*: `usize`



## object::pe::IMAGE_ENCLAVE_POLICY_DEBUGGABLE

*Constant*: `u32`



## object::pe::IMAGE_ENCLAVE_SHORT_ID_LENGTH

*Constant*: `usize`



## object::pe::IMAGE_FILE_32BIT_MACHINE

*Constant*: `u16`

32 bit word machine.



## object::pe::IMAGE_FILE_AGGRESIVE_WS_TRIM

*Constant*: `u16`

Aggressively trim working set



## object::pe::IMAGE_FILE_BYTES_REVERSED_HI

*Constant*: `u16`

Bytes of machine word are reversed.



## object::pe::IMAGE_FILE_BYTES_REVERSED_LO

*Constant*: `u16`

Bytes of machine word are reversed.



## object::pe::IMAGE_FILE_DEBUG_STRIPPED

*Constant*: `u16`

Debugging info stripped from file in .DBG file



## object::pe::IMAGE_FILE_DLL

*Constant*: `u16`

File is a DLL.



## object::pe::IMAGE_FILE_EXECUTABLE_IMAGE

*Constant*: `u16`

File is executable  (i.e. no unresolved external references).



## object::pe::IMAGE_FILE_LARGE_ADDRESS_AWARE

*Constant*: `u16`

App can handle >2gb addresses



## object::pe::IMAGE_FILE_LINE_NUMS_STRIPPED

*Constant*: `u16`

Line numbers stripped from file.



## object::pe::IMAGE_FILE_LOCAL_SYMS_STRIPPED

*Constant*: `u16`

Local symbols stripped from file.



## object::pe::IMAGE_FILE_MACHINE_ALPHA

*Constant*: `u16`

Alpha_AXP



## object::pe::IMAGE_FILE_MACHINE_ALPHA64

*Constant*: `u16`

ALPHA64



## object::pe::IMAGE_FILE_MACHINE_AM33

*Constant*: `u16`



## object::pe::IMAGE_FILE_MACHINE_AMD64

*Constant*: `u16`

AMD64 (K8)



## object::pe::IMAGE_FILE_MACHINE_ARM

*Constant*: `u16`

ARM Little-Endian



## object::pe::IMAGE_FILE_MACHINE_ARM64

*Constant*: `u16`

ARM64 Little-Endian



## object::pe::IMAGE_FILE_MACHINE_ARM64EC

*Constant*: `u16`

ARM64EC ("Emulation Compatible")



## object::pe::IMAGE_FILE_MACHINE_ARM64X

*Constant*: `u16`

ARM64X (Mixed ARM64 and ARM64EC)



## object::pe::IMAGE_FILE_MACHINE_ARMNT

*Constant*: `u16`

ARM Thumb-2 Little-Endian



## object::pe::IMAGE_FILE_MACHINE_AXP64

*Constant*: `u16`



## object::pe::IMAGE_FILE_MACHINE_CEE

*Constant*: `u16`



## object::pe::IMAGE_FILE_MACHINE_CEF

*Constant*: `u16`



## object::pe::IMAGE_FILE_MACHINE_CHPE_X86

*Constant*: `u16`

CHPE x86 ("Compiled Hybrid Portable Executable")



## object::pe::IMAGE_FILE_MACHINE_EBC

*Constant*: `u16`

EFI Byte Code



## object::pe::IMAGE_FILE_MACHINE_I386

*Constant*: `u16`

Intel 386.



## object::pe::IMAGE_FILE_MACHINE_IA64

*Constant*: `u16`

Intel 64



## object::pe::IMAGE_FILE_MACHINE_M32R

*Constant*: `u16`

M32R little-endian



## object::pe::IMAGE_FILE_MACHINE_MIPS16

*Constant*: `u16`

MIPS



## object::pe::IMAGE_FILE_MACHINE_MIPSFPU

*Constant*: `u16`

MIPS



## object::pe::IMAGE_FILE_MACHINE_MIPSFPU16

*Constant*: `u16`

MIPS



## object::pe::IMAGE_FILE_MACHINE_POWERPC

*Constant*: `u16`

IBM PowerPC Little-Endian



## object::pe::IMAGE_FILE_MACHINE_POWERPCBE

*Constant*: `u16`

IBM PowerPC Big-Endian



## object::pe::IMAGE_FILE_MACHINE_POWERPCFP

*Constant*: `u16`



## object::pe::IMAGE_FILE_MACHINE_R10000

*Constant*: `u16`

MIPS little-endian



## object::pe::IMAGE_FILE_MACHINE_R3000

*Constant*: `u16`

MIPS little-endian, 0x160 big-endian



## object::pe::IMAGE_FILE_MACHINE_R4000

*Constant*: `u16`

MIPS little-endian



## object::pe::IMAGE_FILE_MACHINE_RISCV128

*Constant*: `u16`

RISCV128



## object::pe::IMAGE_FILE_MACHINE_RISCV32

*Constant*: `u16`

RISCV32



## object::pe::IMAGE_FILE_MACHINE_RISCV64

*Constant*: `u16`

RISCV64



## object::pe::IMAGE_FILE_MACHINE_SH3

*Constant*: `u16`

SH3 little-endian



## object::pe::IMAGE_FILE_MACHINE_SH3DSP

*Constant*: `u16`



## object::pe::IMAGE_FILE_MACHINE_SH3E

*Constant*: `u16`

SH3E little-endian



## object::pe::IMAGE_FILE_MACHINE_SH4

*Constant*: `u16`

SH4 little-endian



## object::pe::IMAGE_FILE_MACHINE_SH5

*Constant*: `u16`

SH5



## object::pe::IMAGE_FILE_MACHINE_TARGET_HOST

*Constant*: `u16`

Useful for indicating we want to interact with the host and not a WoW guest.



## object::pe::IMAGE_FILE_MACHINE_THUMB

*Constant*: `u16`

ARM Thumb/Thumb-2 Little-Endian



## object::pe::IMAGE_FILE_MACHINE_TRICORE

*Constant*: `u16`

Infineon



## object::pe::IMAGE_FILE_MACHINE_UNKNOWN

*Constant*: `u16`



## object::pe::IMAGE_FILE_MACHINE_WCEMIPSV2

*Constant*: `u16`

MIPS little-endian WCE v2



## object::pe::IMAGE_FILE_NET_RUN_FROM_SWAP

*Constant*: `u16`

If Image is on Net, copy and run from the swap file.



## object::pe::IMAGE_FILE_RELOCS_STRIPPED

*Constant*: `u16`

Relocation info stripped from file.



## object::pe::IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP

*Constant*: `u16`

If Image is on removable media, copy and run from the swap file.



## object::pe::IMAGE_FILE_SYSTEM

*Constant*: `u16`

System File.



## object::pe::IMAGE_FILE_UP_SYSTEM_ONLY

*Constant*: `u16`

File should only be run on a UP machine



## object::pe::IMAGE_GUARD_CFW_INSTRUMENTED

*Constant*: `u32`

Module performs control flow and write integrity checks



## object::pe::IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION

*Constant*: `u32`

Module enables suppression of exports



## object::pe::IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT

*Constant*: `u32`

Module contains suppressed export information.

This also infers that the address taken taken IAT table is also present in the load config.



## object::pe::IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT

*Constant*: `u32`

Module contains valid control flow target metadata



## object::pe::IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK

*Constant*: `u32`

Stride of Guard CF function table encoded in these bits (additional count of bytes per element)



## object::pe::IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT

*Constant*: `u32`

Shift to right-justify Guard CF function table stride



## object::pe::IMAGE_GUARD_CF_INSTRUMENTED

*Constant*: `u32`

Module performs control flow integrity checks using system-supplied support



## object::pe::IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT

*Constant*: `u32`

Module contains longjmp target information



## object::pe::IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION

*Constant*: `u32`

Delayload import table in its own .didat section (with nothing else in it) that can be freely reprotected



## object::pe::IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED

*Constant*: `u16`

The containing GFID entry is export suppressed



## object::pe::IMAGE_GUARD_FLAG_FID_SUPPRESSED

*Constant*: `u16`

The containing GFID entry is suppressed



## object::pe::IMAGE_GUARD_PROTECT_DELAYLOAD_IAT

*Constant*: `u32`

Module supports read only delay load IAT



## object::pe::IMAGE_GUARD_RETPOLINE_PRESENT

*Constant*: `u32`

Module was built with retpoline support



## object::pe::IMAGE_GUARD_RF_ENABLE

*Constant*: `u32`

Module requests that the OS enable return flow protection



## object::pe::IMAGE_GUARD_RF_INSTRUMENTED

*Constant*: `u32`

Module contains return flow instrumentation and metadata



## object::pe::IMAGE_GUARD_RF_STRICT

*Constant*: `u32`

Module requests that the OS enable return flow protection in strict mode



## object::pe::IMAGE_GUARD_SECURITY_COOKIE_UNUSED

*Constant*: `u32`

Module does not make use of the /GS security cookie



## object::pe::IMAGE_HOT_PATCH_ABSOLUTE

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_BASE_OBLIGATORY

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_CALL_TARGET

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_CHUNK_INVERSE

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_CHUNK_OBLIGATORY

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_CHUNK_RESERVED

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_CHUNK_SIZE

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_CHUNK_TARGET_RVA

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_CHUNK_TYPE

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_DYNAMIC_VALUE

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_FUNCTION

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_INDIRECT

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_NONE

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_NO_CALL_TARGET

*Constant*: `u32`



## object::pe::IMAGE_HOT_PATCH_REL32

*Constant*: `u32`



## object::pe::IMAGE_NT_OPTIONAL_HDR32_MAGIC

*Constant*: `u16`



## object::pe::IMAGE_NT_OPTIONAL_HDR64_MAGIC

*Constant*: `u16`



## object::pe::IMAGE_NT_SIGNATURE

*Constant*: `u32`

PE00



## object::pe::IMAGE_NUMBEROF_DIRECTORY_ENTRIES

*Constant*: `usize`



## object::pe::IMAGE_ORDINAL_FLAG32

*Constant*: `u32`



## object::pe::IMAGE_ORDINAL_FLAG64

*Constant*: `u64`



## object::pe::IMAGE_OS2_SIGNATURE

*Constant*: `u16`

NE



## object::pe::IMAGE_OS2_SIGNATURE_LE

*Constant*: `u16`

LE



## object::pe::IMAGE_REL_ALPHA_ABSOLUTE

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_BRADDR

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_GPDISP

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_GPREL32

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_GPRELHI

*Constant*: `u16`

High 16-bit GP relative reference



## object::pe::IMAGE_REL_ALPHA_GPRELLO

*Constant*: `u16`

Low 16-bit GP relative reference



## object::pe::IMAGE_REL_ALPHA_HINT

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_INLINE_REFLONG

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_LITERAL

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_LITUSE

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_MATCH

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_PAIR

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_REFHI

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_REFLO

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_REFLONG

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_REFLONGNB

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_REFQ1

*Constant*: `u16`

Low 16 bits of 48 bit reference



## object::pe::IMAGE_REL_ALPHA_REFQ2

*Constant*: `u16`

Middle 16 bits of 48 bit reference



## object::pe::IMAGE_REL_ALPHA_REFQ3

*Constant*: `u16`

High 16 bits of 48 bit reference



## object::pe::IMAGE_REL_ALPHA_REFQUAD

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_SECREL

*Constant*: `u16`



## object::pe::IMAGE_REL_ALPHA_SECRELHI

*Constant*: `u16`

High 16-bit section relative reference



## object::pe::IMAGE_REL_ALPHA_SECRELLO

*Constant*: `u16`

Low 16-bit section relative reference



## object::pe::IMAGE_REL_ALPHA_SECTION

*Constant*: `u16`



## object::pe::IMAGE_REL_AMD64_ABSOLUTE

*Constant*: `u16`

Reference is absolute, no relocation is necessary



## object::pe::IMAGE_REL_AMD64_ADDR32

*Constant*: `u16`

32-bit address (VA).



## object::pe::IMAGE_REL_AMD64_ADDR32NB

*Constant*: `u16`

32-bit address w/o image base (RVA).



## object::pe::IMAGE_REL_AMD64_ADDR64

*Constant*: `u16`

64-bit address (VA).



## object::pe::IMAGE_REL_AMD64_CFG_BR

*Constant*: `u16`

Indirect branch to a CFG check



## object::pe::IMAGE_REL_AMD64_CFG_BR_REX

*Constant*: `u16`

Indirect branch to a CFG check, with REX.W prefix



## object::pe::IMAGE_REL_AMD64_CFG_CALL

*Constant*: `u16`

Indirect call to a CFG check



## object::pe::IMAGE_REL_AMD64_EHANDLER

*Constant*: `u16`



## object::pe::IMAGE_REL_AMD64_IMPORT_BR

*Constant*: `u16`

Indirect branch to an import



## object::pe::IMAGE_REL_AMD64_IMPORT_CALL

*Constant*: `u16`

Indirect call to an import



## object::pe::IMAGE_REL_AMD64_INDIR_BR

*Constant*: `u16`

Indirect branch to a target in RAX (no CFG)



## object::pe::IMAGE_REL_AMD64_INDIR_BR_REX

*Constant*: `u16`

Indirect branch to a target in RAX, with REX.W prefix (no CFG)



## object::pe::IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST

*Constant*: `u16`

Indirect branch for a switch table using Reg 0 (RAX)



## object::pe::IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST

*Constant*: `u16`

Indirect branch for a switch table using Reg 15 (R15)



## object::pe::IMAGE_REL_AMD64_INDIR_CALL

*Constant*: `u16`

Indirect call to a target in RAX (no CFG)



## object::pe::IMAGE_REL_AMD64_PAIR

*Constant*: `u16`



## object::pe::IMAGE_REL_AMD64_REL32

*Constant*: `u16`

32-bit relative address from byte following reloc



## object::pe::IMAGE_REL_AMD64_REL32_1

*Constant*: `u16`

32-bit relative address from byte distance 1 from reloc



## object::pe::IMAGE_REL_AMD64_REL32_2

*Constant*: `u16`

32-bit relative address from byte distance 2 from reloc



## object::pe::IMAGE_REL_AMD64_REL32_3

*Constant*: `u16`

32-bit relative address from byte distance 3 from reloc



## object::pe::IMAGE_REL_AMD64_REL32_4

*Constant*: `u16`

32-bit relative address from byte distance 4 from reloc



## object::pe::IMAGE_REL_AMD64_REL32_5

*Constant*: `u16`

32-bit relative address from byte distance 5 from reloc



## object::pe::IMAGE_REL_AMD64_SECREL

*Constant*: `u16`

32 bit offset from base of section containing target



## object::pe::IMAGE_REL_AMD64_SECREL7

*Constant*: `u16`

7 bit unsigned offset from base of section containing target



## object::pe::IMAGE_REL_AMD64_SECTION

*Constant*: `u16`

Section index



## object::pe::IMAGE_REL_AMD64_SREL32

*Constant*: `u16`

32 bit signed span-dependent value emitted into object



## object::pe::IMAGE_REL_AMD64_SSPAN32

*Constant*: `u16`

32 bit signed span-dependent value applied at link time



## object::pe::IMAGE_REL_AMD64_TOKEN

*Constant*: `u16`

32 bit metadata token



## object::pe::IMAGE_REL_AM_ABSOLUTE

*Constant*: `u16`



## object::pe::IMAGE_REL_AM_ADDR32

*Constant*: `u16`



## object::pe::IMAGE_REL_AM_ADDR32NB

*Constant*: `u16`



## object::pe::IMAGE_REL_AM_CALL32

*Constant*: `u16`



## object::pe::IMAGE_REL_AM_FUNCINFO

*Constant*: `u16`



## object::pe::IMAGE_REL_AM_REL32_1

*Constant*: `u16`



## object::pe::IMAGE_REL_AM_REL32_2

*Constant*: `u16`



## object::pe::IMAGE_REL_AM_SECREL

*Constant*: `u16`



## object::pe::IMAGE_REL_AM_SECTION

*Constant*: `u16`



## object::pe::IMAGE_REL_AM_TOKEN

*Constant*: `u16`



## object::pe::IMAGE_REL_ARM64_ABSOLUTE

*Constant*: `u16`

No relocation required



## object::pe::IMAGE_REL_ARM64_ADDR32

*Constant*: `u16`

32 bit address. Review! do we need it?



## object::pe::IMAGE_REL_ARM64_ADDR32NB

*Constant*: `u16`

32 bit address w/o image base (RVA: for Data/PData/XData)



## object::pe::IMAGE_REL_ARM64_ADDR64

*Constant*: `u16`

64 bit address



## object::pe::IMAGE_REL_ARM64_BRANCH14

*Constant*: `u16`

TBZ/TBNZ



## object::pe::IMAGE_REL_ARM64_BRANCH19

*Constant*: `u16`

19 bit offset << 2 & sign ext. for conditional B



## object::pe::IMAGE_REL_ARM64_BRANCH26

*Constant*: `u16`

26 bit offset << 2 & sign ext. for B & BL



## object::pe::IMAGE_REL_ARM64_PAGEBASE_REL21

*Constant*: `u16`

ADRP



## object::pe::IMAGE_REL_ARM64_PAGEOFFSET_12A

*Constant*: `u16`

ADD/ADDS (immediate) with zero shift, for page offset



## object::pe::IMAGE_REL_ARM64_PAGEOFFSET_12L

*Constant*: `u16`

LDR (indexed, unsigned immediate), for page offset



## object::pe::IMAGE_REL_ARM64_REL21

*Constant*: `u16`

ADR



## object::pe::IMAGE_REL_ARM64_REL32

*Constant*: `u16`

32-bit relative address from byte following reloc



## object::pe::IMAGE_REL_ARM64_SECREL

*Constant*: `u16`

Offset within section



## object::pe::IMAGE_REL_ARM64_SECREL_HIGH12A

*Constant*: `u16`

ADD/ADDS (immediate) with zero shift, for bit 12:23 of section offset



## object::pe::IMAGE_REL_ARM64_SECREL_LOW12A

*Constant*: `u16`

ADD/ADDS (immediate) with zero shift, for bit 0:11 of section offset



## object::pe::IMAGE_REL_ARM64_SECREL_LOW12L

*Constant*: `u16`

LDR (indexed, unsigned immediate), for bit 0:11 of section offset



## object::pe::IMAGE_REL_ARM64_SECTION

*Constant*: `u16`

Section table index



## object::pe::IMAGE_REL_ARM64_TOKEN

*Constant*: `u16`



## object::pe::IMAGE_REL_ARM_ABSOLUTE

*Constant*: `u16`

No relocation required



## object::pe::IMAGE_REL_ARM_ADDR32

*Constant*: `u16`

32 bit address



## object::pe::IMAGE_REL_ARM_ADDR32NB

*Constant*: `u16`

32 bit address w/o image base



## object::pe::IMAGE_REL_ARM_BLX11

*Constant*: `u16`



## object::pe::IMAGE_REL_ARM_BLX23T

*Constant*: `u16`

Thumb: BLX immediate



## object::pe::IMAGE_REL_ARM_BLX24

*Constant*: `u16`



## object::pe::IMAGE_REL_ARM_BRANCH11

*Constant*: `u16`

Thumb: 2 11 bit offsets



## object::pe::IMAGE_REL_ARM_BRANCH20T

*Constant*: `u16`

Thumb: 32-bit conditional B



## object::pe::IMAGE_REL_ARM_BRANCH24

*Constant*: `u16`

24 bit offset << 2 & sign ext.



## object::pe::IMAGE_REL_ARM_BRANCH24T

*Constant*: `u16`

Thumb: 32-bit B or BL



## object::pe::IMAGE_REL_ARM_GPREL12

*Constant*: `u16`

GP-relative addressing (ARM)



## object::pe::IMAGE_REL_ARM_GPREL7

*Constant*: `u16`

GP-relative addressing (Thumb)



## object::pe::IMAGE_REL_ARM_MOV32

*Constant*: `u16`

ARM: MOVW/MOVT (deprecated)



## object::pe::IMAGE_REL_ARM_MOV32A

*Constant*: `u16`

ARM: MOVW/MOVT



## object::pe::IMAGE_REL_ARM_MOV32T

*Constant*: `u16`

Thumb: MOVW/MOVT



## object::pe::IMAGE_REL_ARM_REL32

*Constant*: `u16`

32-bit relative address from byte following reloc



## object::pe::IMAGE_REL_ARM_SECREL

*Constant*: `u16`

Offset within section



## object::pe::IMAGE_REL_ARM_SECTION

*Constant*: `u16`

Section table index



## object::pe::IMAGE_REL_ARM_TOKEN

*Constant*: `u16`

clr token



## object::pe::IMAGE_REL_BASED_ABSOLUTE

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_ARM_MOV32

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_DIR64

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_HIGH

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_HIGHADJ

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_HIGHLOW

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_IA64_IMM64

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_LOW

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_MACHINE_SPECIFIC_5

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_MACHINE_SPECIFIC_7

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_MACHINE_SPECIFIC_8

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_MACHINE_SPECIFIC_9

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_MIPS_JMPADDR

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_MIPS_JMPADDR16

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_RESERVED

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_RISCV_HIGH20

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_RISCV_LOW12I

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_RISCV_LOW12S

*Constant*: `u16`



## object::pe::IMAGE_REL_BASED_THUMB_MOV32

*Constant*: `u16`



## object::pe::IMAGE_REL_CEE_ABSOLUTE

*Constant*: `u16`

Reference is absolute, no relocation is necessary



## object::pe::IMAGE_REL_CEE_ADDR32

*Constant*: `u16`

32-bit address (VA).



## object::pe::IMAGE_REL_CEE_ADDR32NB

*Constant*: `u16`

32-bit address w/o image base (RVA).



## object::pe::IMAGE_REL_CEE_ADDR64

*Constant*: `u16`

64-bit address (VA).



## object::pe::IMAGE_REL_CEE_SECREL

*Constant*: `u16`

32 bit offset from base of section containing target



## object::pe::IMAGE_REL_CEE_SECTION

*Constant*: `u16`

Section index



## object::pe::IMAGE_REL_CEE_TOKEN

*Constant*: `u16`

32 bit metadata token



## object::pe::IMAGE_REL_CEF_ABSOLUTE

*Constant*: `u16`

Reference is absolute, no relocation is necessary



## object::pe::IMAGE_REL_CEF_ADDR32

*Constant*: `u16`

32-bit address (VA).



## object::pe::IMAGE_REL_CEF_ADDR32NB

*Constant*: `u16`

32-bit address w/o image base (RVA).



## object::pe::IMAGE_REL_CEF_ADDR64

*Constant*: `u16`

64-bit address (VA).



## object::pe::IMAGE_REL_CEF_SECREL

*Constant*: `u16`

32 bit offset from base of section containing target



## object::pe::IMAGE_REL_CEF_SECTION

*Constant*: `u16`

Section index



## object::pe::IMAGE_REL_CEF_TOKEN

*Constant*: `u16`

32 bit metadata token



## object::pe::IMAGE_REL_EBC_ABSOLUTE

*Constant*: `u16`

No relocation required



## object::pe::IMAGE_REL_EBC_ADDR32NB

*Constant*: `u16`

32 bit address w/o image base



## object::pe::IMAGE_REL_EBC_REL32

*Constant*: `u16`

32-bit relative address from byte following reloc



## object::pe::IMAGE_REL_EBC_SECREL

*Constant*: `u16`

Offset within section



## object::pe::IMAGE_REL_EBC_SECTION

*Constant*: `u16`

Section table index



## object::pe::IMAGE_REL_I386_ABSOLUTE

*Constant*: `u16`

Reference is absolute, no relocation is necessary



## object::pe::IMAGE_REL_I386_DIR16

*Constant*: `u16`

Direct 16-bit reference to the symbols virtual address



## object::pe::IMAGE_REL_I386_DIR32

*Constant*: `u16`

Direct 32-bit reference to the symbols virtual address



## object::pe::IMAGE_REL_I386_DIR32NB

*Constant*: `u16`

Direct 32-bit reference to the symbols virtual address, base not included



## object::pe::IMAGE_REL_I386_REL16

*Constant*: `u16`

PC-relative 16-bit reference to the symbols virtual address



## object::pe::IMAGE_REL_I386_REL32

*Constant*: `u16`

PC-relative 32-bit reference to the symbols virtual address



## object::pe::IMAGE_REL_I386_SECREL

*Constant*: `u16`



## object::pe::IMAGE_REL_I386_SECREL7

*Constant*: `u16`

7 bit offset from base of section containing target



## object::pe::IMAGE_REL_I386_SECTION

*Constant*: `u16`



## object::pe::IMAGE_REL_I386_SEG12

*Constant*: `u16`

Direct 16-bit reference to the segment-selector bits of a 32-bit virtual address



## object::pe::IMAGE_REL_I386_TOKEN

*Constant*: `u16`

clr token



## object::pe::IMAGE_REL_IA64_ABSOLUTE

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_ADDEND

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_DIR32

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_DIR32NB

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_DIR64

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_GPREL22

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_GPREL32

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_IMM14

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_IMM22

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_IMM64

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_IMMGPREL64

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_LTOFF22

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_PCREL21B

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_PCREL21F

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_PCREL21M

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_PCREL60B

*Constant*: `u16`

If possible, convert to MBB bundle with NOP.B in slot 1



## object::pe::IMAGE_REL_IA64_PCREL60F

*Constant*: `u16`

If possible, convert to MFB bundle with NOP.F in slot 1



## object::pe::IMAGE_REL_IA64_PCREL60I

*Constant*: `u16`

If possible, convert to MIB bundle with NOP.I in slot 1



## object::pe::IMAGE_REL_IA64_PCREL60M

*Constant*: `u16`

If possible, convert to MMB bundle with NOP.M in slot 1



## object::pe::IMAGE_REL_IA64_PCREL60X

*Constant*: `u16`

This is always a BRL and never converted



## object::pe::IMAGE_REL_IA64_SECREL22

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_SECREL32

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_SECREL64I

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_SECTION

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_SREL14

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_SREL22

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_SREL32

*Constant*: `u16`



## object::pe::IMAGE_REL_IA64_TOKEN

*Constant*: `u16`

clr token



## object::pe::IMAGE_REL_IA64_UREL32

*Constant*: `u16`



## object::pe::IMAGE_REL_M32R_ABSOLUTE

*Constant*: `u16`

No relocation required



## object::pe::IMAGE_REL_M32R_ADDR24

*Constant*: `u16`

24 bit address



## object::pe::IMAGE_REL_M32R_ADDR32

*Constant*: `u16`

32 bit address



## object::pe::IMAGE_REL_M32R_ADDR32NB

*Constant*: `u16`

32 bit address w/o image base



## object::pe::IMAGE_REL_M32R_GPREL16

*Constant*: `u16`

GP relative addressing



## object::pe::IMAGE_REL_M32R_PAIR

*Constant*: `u16`

Link HI and LO



## object::pe::IMAGE_REL_M32R_PCREL16

*Constant*: `u16`

16 bit offset << 2 & sign ext.



## object::pe::IMAGE_REL_M32R_PCREL24

*Constant*: `u16`

24 bit offset << 2 & sign ext.



## object::pe::IMAGE_REL_M32R_PCREL8

*Constant*: `u16`

8 bit offset << 2 & sign ext.



## object::pe::IMAGE_REL_M32R_REFHALF

*Constant*: `u16`

16 MSBs



## object::pe::IMAGE_REL_M32R_REFHI

*Constant*: `u16`

16 MSBs; adj for LSB sign ext.



## object::pe::IMAGE_REL_M32R_REFLO

*Constant*: `u16`

16 LSBs



## object::pe::IMAGE_REL_M32R_SECREL32

*Constant*: `u16`

32 bit section relative reference



## object::pe::IMAGE_REL_M32R_SECTION

*Constant*: `u16`

Section table index



## object::pe::IMAGE_REL_M32R_TOKEN

*Constant*: `u16`

clr token



## object::pe::IMAGE_REL_MIPS_ABSOLUTE

*Constant*: `u16`

Reference is absolute, no relocation is necessary



## object::pe::IMAGE_REL_MIPS_GPREL

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_JMPADDR

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_JMPADDR16

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_LITERAL

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_PAIR

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_REFHALF

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_REFHI

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_REFLO

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_REFWORD

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_REFWORDNB

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_SECREL

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_SECRELHI

*Constant*: `u16`

High 16-bit section relative reference (used for >32k TLS)



## object::pe::IMAGE_REL_MIPS_SECRELLO

*Constant*: `u16`

Low 16-bit section relative reference (used for >32k TLS)



## object::pe::IMAGE_REL_MIPS_SECTION

*Constant*: `u16`



## object::pe::IMAGE_REL_MIPS_TOKEN

*Constant*: `u16`

clr token



## object::pe::IMAGE_REL_PPC_ABSOLUTE

*Constant*: `u16`

NOP



## object::pe::IMAGE_REL_PPC_ADDR14

*Constant*: `u16`

16-bit address, shifted left 2 (load doubleword)



## object::pe::IMAGE_REL_PPC_ADDR16

*Constant*: `u16`

16-bit address



## object::pe::IMAGE_REL_PPC_ADDR24

*Constant*: `u16`

26-bit address, shifted left 2 (branch absolute)



## object::pe::IMAGE_REL_PPC_ADDR32

*Constant*: `u16`

32-bit address



## object::pe::IMAGE_REL_PPC_ADDR32NB

*Constant*: `u16`

32-bit addr w/o image base



## object::pe::IMAGE_REL_PPC_ADDR64

*Constant*: `u16`

64-bit address



## object::pe::IMAGE_REL_PPC_BRNTAKEN

*Constant*: `u16`

fix branch prediction bit to predict branch not taken



## object::pe::IMAGE_REL_PPC_BRTAKEN

*Constant*: `u16`

fix branch prediction bit to predict branch taken



## object::pe::IMAGE_REL_PPC_GPREL

*Constant*: `u16`



## object::pe::IMAGE_REL_PPC_IFGLUE

*Constant*: `u16`

substitute TOC restore instruction iff symbol is glue code



## object::pe::IMAGE_REL_PPC_IMGLUE

*Constant*: `u16`

symbol is glue code; virtual address is TOC restore instruction



## object::pe::IMAGE_REL_PPC_NEG

*Constant*: `u16`

subtract reloc value rather than adding it



## object::pe::IMAGE_REL_PPC_PAIR

*Constant*: `u16`



## object::pe::IMAGE_REL_PPC_REFHI

*Constant*: `u16`



## object::pe::IMAGE_REL_PPC_REFLO

*Constant*: `u16`



## object::pe::IMAGE_REL_PPC_REL14

*Constant*: `u16`

16-bit PC-relative offset, shifted left 2 (br cond relative)



## object::pe::IMAGE_REL_PPC_REL24

*Constant*: `u16`

26-bit PC-relative offset, shifted left 2 (branch relative)



## object::pe::IMAGE_REL_PPC_SECREL

*Constant*: `u16`

va of containing section (as in an image sectionhdr)



## object::pe::IMAGE_REL_PPC_SECREL16

*Constant*: `u16`

va of containing section (limited to 16 bits)



## object::pe::IMAGE_REL_PPC_SECRELHI

*Constant*: `u16`

High 16-bit section relative reference (used for >32k TLS)



## object::pe::IMAGE_REL_PPC_SECRELLO

*Constant*: `u16`

Low 16-bit section relative reference (used for >32k TLS)



## object::pe::IMAGE_REL_PPC_SECTION

*Constant*: `u16`

sectionheader number



## object::pe::IMAGE_REL_PPC_TOCDEFN

*Constant*: `u16`

toc slot defined in file (or, data in toc)



## object::pe::IMAGE_REL_PPC_TOCREL14

*Constant*: `u16`

16-bit offset from TOC base, shifted left 2 (load doubleword)



## object::pe::IMAGE_REL_PPC_TOCREL16

*Constant*: `u16`

16-bit offset from TOC base



## object::pe::IMAGE_REL_PPC_TOKEN

*Constant*: `u16`

clr token



## object::pe::IMAGE_REL_PPC_TYPEMASK

*Constant*: `u16`

mask to isolate above values in IMAGE_RELOCATION.Type



## object::pe::IMAGE_REL_SH3_ABSOLUTE

*Constant*: `u16`

No relocation



## object::pe::IMAGE_REL_SH3_DIRECT16

*Constant*: `u16`

16 bit direct



## object::pe::IMAGE_REL_SH3_DIRECT32

*Constant*: `u16`

32 bit direct



## object::pe::IMAGE_REL_SH3_DIRECT32_NB

*Constant*: `u16`

32 bit direct not based



## object::pe::IMAGE_REL_SH3_DIRECT4

*Constant*: `u16`

4 bit direct (0 ext.)



## object::pe::IMAGE_REL_SH3_DIRECT4_LONG

*Constant*: `u16`

4 bit direct .L (0 ext.)



## object::pe::IMAGE_REL_SH3_DIRECT4_WORD

*Constant*: `u16`

4 bit direct .W (0 ext.)



## object::pe::IMAGE_REL_SH3_DIRECT8

*Constant*: `u16`

8 bit direct, -128..255



## object::pe::IMAGE_REL_SH3_DIRECT8_LONG

*Constant*: `u16`

8 bit direct .L (0 ext.)



## object::pe::IMAGE_REL_SH3_DIRECT8_WORD

*Constant*: `u16`

8 bit direct .W (0 ext.)



## object::pe::IMAGE_REL_SH3_GPREL4_LONG

*Constant*: `u16`

GP-relative addressing



## object::pe::IMAGE_REL_SH3_PCREL12_WORD

*Constant*: `u16`

12 LSB PC relative .W



## object::pe::IMAGE_REL_SH3_PCREL8_LONG

*Constant*: `u16`

8 bit PC relative .L



## object::pe::IMAGE_REL_SH3_PCREL8_WORD

*Constant*: `u16`

8 bit PC relative .W



## object::pe::IMAGE_REL_SH3_SECREL

*Constant*: `u16`

Offset within section



## object::pe::IMAGE_REL_SH3_SECTION

*Constant*: `u16`

Section table index



## object::pe::IMAGE_REL_SH3_SIZEOF_SECTION

*Constant*: `u16`

Size of EXE section



## object::pe::IMAGE_REL_SH3_STARTOF_SECTION

*Constant*: `u16`

Start of EXE section



## object::pe::IMAGE_REL_SH3_TOKEN

*Constant*: `u16`

clr token



## object::pe::IMAGE_REL_SHM_PAIR

*Constant*: `u16`

offset operand for relocation



## object::pe::IMAGE_REL_SHM_PCRELPT

*Constant*: `u16`

Offset from current instruction in longwords
if not NOMODE, insert the inverse of the low bit at bit 32 to select PTA/PTB



## object::pe::IMAGE_REL_SHM_REFHALF

*Constant*: `u16`

High bits of 32-bit address



## object::pe::IMAGE_REL_SHM_REFLO

*Constant*: `u16`

Low bits of 32-bit address



## object::pe::IMAGE_REL_SHM_RELHALF

*Constant*: `u16`

High bits of relative reference



## object::pe::IMAGE_REL_SHM_RELLO

*Constant*: `u16`

Low bits of relative reference



## object::pe::IMAGE_REL_SH_NOMODE

*Constant*: `u16`

relocation ignores section mode



## object::pe::IMAGE_REL_THUMB_BLX23

*Constant*: `u16`

Thumb: BLX immediate (deprecated)



## object::pe::IMAGE_REL_THUMB_BRANCH20

*Constant*: `u16`

Thumb: 32-bit conditional B (deprecated)



## object::pe::IMAGE_REL_THUMB_BRANCH24

*Constant*: `u16`

Thumb: 32-bit B or BL (deprecated)



## object::pe::IMAGE_REL_THUMB_MOV32

*Constant*: `u16`

Thumb: MOVW/MOVT (deprecated)



## object::pe::IMAGE_RESOURCE_DATA_IS_DIRECTORY

*Constant*: `u32`



## object::pe::IMAGE_RESOURCE_NAME_IS_STRING

*Constant*: `u32`



## object::pe::IMAGE_ROM_OPTIONAL_HDR_MAGIC

*Constant*: `u16`



## object::pe::IMAGE_SCN_ALIGN_1024BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_128BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_16BYTES

*Constant*: `u32`

Default alignment if no others are specified.



## object::pe::IMAGE_SCN_ALIGN_1BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_2048BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_256BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_2BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_32BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_4096BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_4BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_512BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_64BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_8192BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_8BYTES

*Constant*: `u32`



## object::pe::IMAGE_SCN_ALIGN_MASK

*Constant*: `u32`



## object::pe::IMAGE_SCN_CNT_CODE

*Constant*: `u32`

Section contains code.



## object::pe::IMAGE_SCN_CNT_INITIALIZED_DATA

*Constant*: `u32`

Section contains initialized data.



## object::pe::IMAGE_SCN_CNT_UNINITIALIZED_DATA

*Constant*: `u32`

Section contains uninitialized data.



## object::pe::IMAGE_SCN_GPREL

*Constant*: `u32`

Section content can be accessed relative to GP



## object::pe::IMAGE_SCN_LNK_COMDAT

*Constant*: `u32`

Section contents comdat.



## object::pe::IMAGE_SCN_LNK_INFO

*Constant*: `u32`

Section contains comments or some other type of information.



## object::pe::IMAGE_SCN_LNK_NRELOC_OVFL

*Constant*: `u32`

Section contains extended relocations.



## object::pe::IMAGE_SCN_LNK_OTHER

*Constant*: `u32`

Reserved.



## object::pe::IMAGE_SCN_LNK_REMOVE

*Constant*: `u32`

Section contents will not become part of image.



## object::pe::IMAGE_SCN_MEM_16BIT

*Constant*: `u32`



## object::pe::IMAGE_SCN_MEM_DISCARDABLE

*Constant*: `u32`

Section can be discarded.



## object::pe::IMAGE_SCN_MEM_EXECUTE

*Constant*: `u32`

Section is executable.



## object::pe::IMAGE_SCN_MEM_FARDATA

*Constant*: `u32`



## object::pe::IMAGE_SCN_MEM_LOCKED

*Constant*: `u32`



## object::pe::IMAGE_SCN_MEM_NOT_CACHED

*Constant*: `u32`

Section is not cacheable.



## object::pe::IMAGE_SCN_MEM_NOT_PAGED

*Constant*: `u32`

Section is not pageable.



## object::pe::IMAGE_SCN_MEM_PRELOAD

*Constant*: `u32`



## object::pe::IMAGE_SCN_MEM_PURGEABLE

*Constant*: `u32`



## object::pe::IMAGE_SCN_MEM_READ

*Constant*: `u32`

Section is readable.



## object::pe::IMAGE_SCN_MEM_SHARED

*Constant*: `u32`

Section is shareable.



## object::pe::IMAGE_SCN_MEM_WRITE

*Constant*: `u32`

Section is writeable.



## object::pe::IMAGE_SCN_NO_DEFER_SPEC_EXC

*Constant*: `u32`

Reset speculative exceptions handling bits in the TLB entries for this section.



## object::pe::IMAGE_SCN_SCALE_INDEX

*Constant*: `u32`

Tls index is scaled



## object::pe::IMAGE_SCN_TYPE_NO_PAD

*Constant*: `u32`

Reserved.



## object::pe::IMAGE_SEPARATE_DEBUG_FLAGS_MASK

*Constant*: `u16`



## object::pe::IMAGE_SEPARATE_DEBUG_MISMATCH

*Constant*: `u16`

when DBG was updated, the old checksum didn't match.



## object::pe::IMAGE_SEPARATE_DEBUG_SIGNATURE

*Constant*: `u16`



## object::pe::IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR

*Constant*: `u16`



## object::pe::IMAGE_SIZEOF_FILE_HEADER

*Constant*: `usize`



## object::pe::IMAGE_SIZEOF_SECTION_HEADER

*Constant*: `usize`



## object::pe::IMAGE_SIZEOF_SHORT_NAME

*Constant*: `usize`



## object::pe::IMAGE_SIZEOF_SYMBOL

*Constant*: `usize`



## object::pe::IMAGE_SIZEOF_SYMBOL_EX

*Constant*: `usize`



## object::pe::IMAGE_SUBSYSTEM_EFI_APPLICATION

*Constant*: `u16`



## object::pe::IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER

*Constant*: `u16`



## object::pe::IMAGE_SUBSYSTEM_EFI_ROM

*Constant*: `u16`



## object::pe::IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER

*Constant*: `u16`



## object::pe::IMAGE_SUBSYSTEM_NATIVE

*Constant*: `u16`

Image doesn't require a subsystem.



## object::pe::IMAGE_SUBSYSTEM_NATIVE_WINDOWS

*Constant*: `u16`

image is a native Win9x driver.



## object::pe::IMAGE_SUBSYSTEM_OS2_CUI

*Constant*: `u16`

image runs in the OS/2 character subsystem.



## object::pe::IMAGE_SUBSYSTEM_POSIX_CUI

*Constant*: `u16`

image runs in the Posix character subsystem.



## object::pe::IMAGE_SUBSYSTEM_UNKNOWN

*Constant*: `u16`

Unknown subsystem.



## object::pe::IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION

*Constant*: `u16`



## object::pe::IMAGE_SUBSYSTEM_WINDOWS_CE_GUI

*Constant*: `u16`

Image runs in the Windows CE subsystem.



## object::pe::IMAGE_SUBSYSTEM_WINDOWS_CUI

*Constant*: `u16`

Image runs in the Windows character subsystem.



## object::pe::IMAGE_SUBSYSTEM_WINDOWS_GUI

*Constant*: `u16`

Image runs in the Windows GUI subsystem.



## object::pe::IMAGE_SUBSYSTEM_XBOX

*Constant*: `u16`



## object::pe::IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG

*Constant*: `u16`



## object::pe::IMAGE_SYM_ABSOLUTE

*Constant*: `i32`

Symbol is an absolute value.



## object::pe::IMAGE_SYM_CLASS_ARGUMENT

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_AUTOMATIC

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_BIT_FIELD

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_BLOCK

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_CLR_TOKEN

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_END_OF_FUNCTION

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_END_OF_STRUCT

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_ENUM_TAG

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_EXTERNAL

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_EXTERNAL_DEF

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_FAR_EXTERNAL

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_FILE

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_FUNCTION

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_LABEL

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_MEMBER_OF_ENUM

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_MEMBER_OF_STRUCT

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_MEMBER_OF_UNION

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_NULL

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_REGISTER

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_REGISTER_PARAM

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_SECTION

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_STATIC

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_STRUCT_TAG

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_TYPE_DEFINITION

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_UNDEFINED_LABEL

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_UNDEFINED_STATIC

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_UNION_TAG

*Constant*: `u8`



## object::pe::IMAGE_SYM_CLASS_WEAK_EXTERNAL

*Constant*: `u8`



## object::pe::IMAGE_SYM_DEBUG

*Constant*: `i32`

Symbol is a special debug item.



## object::pe::IMAGE_SYM_DTYPE_ARRAY

*Constant*: `u16`

array.



## object::pe::IMAGE_SYM_DTYPE_FUNCTION

*Constant*: `u16`

function.



## object::pe::IMAGE_SYM_DTYPE_NULL

*Constant*: `u16`

no derived type.



## object::pe::IMAGE_SYM_DTYPE_POINTER

*Constant*: `u16`

pointer.



## object::pe::IMAGE_SYM_DTYPE_SHIFT

*Constant*: `usize`



## object::pe::IMAGE_SYM_SECTION_MAX

*Constant*: `u16`

Values 0xFF00-0xFFFF are special



## object::pe::IMAGE_SYM_SECTION_MAX_EX

*Constant*: `u32`



## object::pe::IMAGE_SYM_TYPE_BYTE

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_CHAR

*Constant*: `u16`

type character.



## object::pe::IMAGE_SYM_TYPE_DOUBLE

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_DWORD

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_ENUM

*Constant*: `u16`

enumeration.



## object::pe::IMAGE_SYM_TYPE_FLOAT

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_INT

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_LONG

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_MOE

*Constant*: `u16`

member of enumeration.



## object::pe::IMAGE_SYM_TYPE_NULL

*Constant*: `u16`

no type.



## object::pe::IMAGE_SYM_TYPE_PCODE

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_SHORT

*Constant*: `u16`

type short integer.



## object::pe::IMAGE_SYM_TYPE_STRUCT

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_UINT

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_UNION

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_VOID

*Constant*: `u16`



## object::pe::IMAGE_SYM_TYPE_WORD

*Constant*: `u16`



## object::pe::IMAGE_SYM_UNDEFINED

*Constant*: `i32`

Symbol is undefined or is common.



## object::pe::IMAGE_VXD_SIGNATURE

*Constant*: `u16`

LE



## object::pe::IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY

*Constant*: `u32`



## object::pe::IMAGE_WEAK_EXTERN_SEARCH_ALIAS

*Constant*: `u32`



## object::pe::IMAGE_WEAK_EXTERN_SEARCH_LIBRARY

*Constant*: `u32`



## object::pe::IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY

*Constant*: `u32`



## object::pe::IMPORT_OBJECT_CODE

*Constant*: `u16`



## object::pe::IMPORT_OBJECT_CONST

*Constant*: `u16`



## object::pe::IMPORT_OBJECT_DATA

*Constant*: `u16`



## object::pe::IMPORT_OBJECT_HDR_SIG2

*Constant*: `u16`



## object::pe::IMPORT_OBJECT_NAME

*Constant*: `u16`

Import name == public symbol name.



## object::pe::IMPORT_OBJECT_NAME_EXPORTAS

*Constant*: `u16`

Import name == a name is explicitly provided after the DLL name.



## object::pe::IMPORT_OBJECT_NAME_MASK

*Constant*: `u16`



## object::pe::IMPORT_OBJECT_NAME_NO_PREFIX

*Constant*: `u16`

Import name == public symbol name skipping leading ?, @, or optionally _.



## object::pe::IMPORT_OBJECT_NAME_SHIFT

*Constant*: `u16`



## object::pe::IMPORT_OBJECT_NAME_UNDECORATE

*Constant*: `u16`

Import name == public symbol name skipping leading ?, @, or optionally _ and truncating at first @.



## object::pe::IMPORT_OBJECT_ORDINAL

*Constant*: `u16`

Import by ordinal



## object::pe::IMPORT_OBJECT_TYPE_MASK

*Constant*: `u16`



## object::pe::IMPORT_OBJECT_TYPE_SHIFT

*Constant*: `u16`



## object::pe::ImageAlpha64RuntimeFunctionEntry

*Struct*

**Fields:**
- `begin_address: crate::endian::U64<crate::endian::LittleEndian>`
- `end_address: crate::endian::U64<crate::endian::LittleEndian>`
- `exception_handler: crate::endian::U64<crate::endian::LittleEndian>`
- `handler_data: crate::endian::U64<crate::endian::LittleEndian>`
- `prolog_end_address: crate::endian::U64<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageAlpha64RuntimeFunctionEntry`



## object::pe::ImageAlphaRuntimeFunctionEntry

*Struct*

**Fields:**
- `begin_address: crate::endian::U32<crate::endian::LittleEndian>`
- `end_address: crate::endian::U32<crate::endian::LittleEndian>`
- `exception_handler: crate::endian::U32<crate::endian::LittleEndian>`
- `handler_data: crate::endian::U32<crate::endian::LittleEndian>`
- `prolog_end_address: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageAlphaRuntimeFunctionEntry`



## object::pe::ImageArchitectureEntry

*Struct*

**Fields:**
- `fixup_inst_rva: crate::endian::U32<crate::endian::LittleEndian>` - RVA of instruction to fixup
- `new_inst: crate::endian::U32<crate::endian::LittleEndian>` - fixup instruction (see alphaops.h)

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageArchitectureEntry`



## object::pe::ImageArchiveMemberHeader

*Struct*

**Fields:**
- `name: [u8; 16]` - File member name - `/' terminated.
- `date: [u8; 12]` - File member date - decimal.
- `user_id: [u8; 6]` - File member user id - decimal.
- `group_id: [u8; 6]` - File member group id - decimal.
- `mode: [u8; 8]` - File member mode - octal.
- `size: [u8; 10]` - File member size - decimal.
- `end_header: [u8; 2]` - String to end header.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageArchiveMemberHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageArm64RuntimeFunctionEntry

*Struct*

**Fields:**
- `begin_address: crate::endian::U32<crate::endian::LittleEndian>`
- `unwind_data: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageArm64RuntimeFunctionEntry`



## object::pe::ImageArmRuntimeFunctionEntry

*Struct*

**Fields:**
- `begin_address: crate::endian::U32<crate::endian::LittleEndian>`
- `unwind_data: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageArmRuntimeFunctionEntry`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageAuxSymbolCrc

*Struct*

**Fields:**
- `crc: crate::endian::U32Bytes<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageAuxSymbolCrc`



## object::pe::ImageAuxSymbolFunction

*Struct*

Auxiliary symbol format 1: function definitions.

**Fields:**
- `tag_index: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `total_size: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `pointer_to_linenumber: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `pointer_to_next_function: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `unused: [u8; 2]`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageAuxSymbolFunction`



## object::pe::ImageAuxSymbolFunctionBeginEnd

*Struct*

Auxiliary symbol format 2: .bf and .ef symbols.

**Fields:**
- `unused1: [u8; 4]`
- `linenumber: crate::endian::U16Bytes<crate::endian::LittleEndian>` - declaration line number
- `unused2: [u8; 6]`
- `pointer_to_next_function: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `unused3: [u8; 2]`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageAuxSymbolFunctionBeginEnd`



## object::pe::ImageAuxSymbolSection

*Struct*

Auxiliary symbol format 5: sections.

Used for both `ImageSymbol` and `ImageSymbolEx` (with padding).

**Fields:**
- `length: crate::endian::U32Bytes<crate::endian::LittleEndian>` - section length
- `number_of_relocations: crate::endian::U16Bytes<crate::endian::LittleEndian>` - number of relocation entries
- `number_of_linenumbers: crate::endian::U16Bytes<crate::endian::LittleEndian>` - number of line numbers
- `check_sum: crate::endian::U32Bytes<crate::endian::LittleEndian>` - checksum for communal
- `number: crate::endian::U16Bytes<crate::endian::LittleEndian>` - section number to associate with
- `selection: u8` - communal selection type
- `reserved: u8`
- `high_number: crate::endian::U16Bytes<crate::endian::LittleEndian>` - high bits of the section number

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageAuxSymbolSection`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageAuxSymbolTokenDef

*Struct*

**Fields:**
- `aux_type: u8` - IMAGE_AUX_SYMBOL_TYPE
- `reserved1: u8` - Must be 0
- `symbol_table_index: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `reserved2: [u8; 12]` - Must be 0

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageAuxSymbolTokenDef`



## object::pe::ImageAuxSymbolWeak

*Struct*

Auxiliary symbol format 3: weak externals.

Used for both `ImageSymbol` and `ImageSymbolEx` (both with padding).

**Fields:**
- `weak_default_sym_index: crate::endian::U32Bytes<crate::endian::LittleEndian>` - the weak extern default symbol index
- `weak_search_type: crate::endian::U32Bytes<crate::endian::LittleEndian>`

**Methods:**

- `fn default_symbol(self: &Self) -> SymbolIndex` - Get the symbol index of the default definition.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageAuxSymbolWeak`



## object::pe::ImageBaseRelocation

*Struct*

**Fields:**
- `virtual_address: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_block: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageBaseRelocation`



## object::pe::ImageBoundForwarderRef

*Struct*

**Fields:**
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `offset_module_name: crate::endian::U16<crate::endian::LittleEndian>`
- `reserved: crate::endian::U16<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageBoundForwarderRef`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageBoundImportDescriptor

*Struct*

**Fields:**
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `offset_module_name: crate::endian::U16<crate::endian::LittleEndian>`
- `number_of_module_forwarder_refs: crate::endian::U16<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageBoundImportDescriptor`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageCoffSymbolsHeader

*Struct*

**Fields:**
- `number_of_symbols: crate::endian::U32<crate::endian::LittleEndian>`
- `lva_to_first_symbol: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_linenumbers: crate::endian::U32<crate::endian::LittleEndian>`
- `lva_to_first_linenumber: crate::endian::U32<crate::endian::LittleEndian>`
- `rva_to_first_byte_of_code: crate::endian::U32<crate::endian::LittleEndian>`
- `rva_to_last_byte_of_code: crate::endian::U32<crate::endian::LittleEndian>`
- `rva_to_first_byte_of_data: crate::endian::U32<crate::endian::LittleEndian>`
- `rva_to_last_byte_of_data: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageCoffSymbolsHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageCor20Header

*Struct*

**Fields:**
- `cb: crate::endian::U32<crate::endian::LittleEndian>`
- `major_runtime_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_runtime_version: crate::endian::U16<crate::endian::LittleEndian>`
- `meta_data: ImageDataDirectory`
- `flags: crate::endian::U32<crate::endian::LittleEndian>`
- `entry_point_token_or_rva: crate::endian::U32<crate::endian::LittleEndian>`
- `resources: ImageDataDirectory`
- `strong_name_signature: ImageDataDirectory`
- `code_manager_table: ImageDataDirectory`
- `vtable_fixups: ImageDataDirectory`
- `export_address_table_jumps: ImageDataDirectory`
- `managed_native_header: ImageDataDirectory`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageCor20Header`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageDataDirectory

*Struct*

**Fields:**
- `virtual_address: crate::endian::U32<crate::endian::LittleEndian>`
- `size: crate::endian::U32<crate::endian::LittleEndian>`

**Methods:**

- `fn address_range(self: &Self) -> (u32, u32)` - Return the virtual address range of this directory entry.
- `fn file_range(self: &Self, sections: &SectionTable) -> Result<(u32, u32)>` - Return the file offset and size of this directory entry.
- `fn data<'data, R>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<&'data [u8]>` - Get the data referenced by this directory entry.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageDataDirectory`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageDebugDirectory

*Struct*

**Fields:**
- `characteristics: crate::endian::U32<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `major_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_version: crate::endian::U16<crate::endian::LittleEndian>`
- `typ: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_data: crate::endian::U32<crate::endian::LittleEndian>`
- `address_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>`
- `pointer_to_raw_data: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageDebugDirectory`



## object::pe::ImageDebugMisc

*Struct*

**Fields:**
- `data_type: crate::endian::U32<crate::endian::LittleEndian>` - type of misc data, see defines
- `length: crate::endian::U32<crate::endian::LittleEndian>` - total length of record, rounded to four byte multiple.
- `unicode: u8` - TRUE if data is unicode string
- `reserved: [u8; 3]`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageDebugMisc`



## object::pe::ImageDelayloadDescriptor

*Struct*

**Fields:**
- `attributes: crate::endian::U32<crate::endian::LittleEndian>`
- `dll_name_rva: crate::endian::U32<crate::endian::LittleEndian>` - RVA to the name of the target library (NULL-terminate ASCII string)
- `module_handle_rva: crate::endian::U32<crate::endian::LittleEndian>` - RVA to the HMODULE caching location (PHMODULE)
- `import_address_table_rva: crate::endian::U32<crate::endian::LittleEndian>` - RVA to the start of the IAT (PIMAGE_THUNK_DATA)
- `import_name_table_rva: crate::endian::U32<crate::endian::LittleEndian>` - RVA to the start of the name table (PIMAGE_THUNK_DATA::AddressOfData)
- `bound_import_address_table_rva: crate::endian::U32<crate::endian::LittleEndian>` - RVA to an optional bound IAT
- `unload_information_table_rva: crate::endian::U32<crate::endian::LittleEndian>` - RVA to an optional unload info table
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>` - 0 if not bound, otherwise, date/time of the target DLL

**Methods:**

- `fn is_null(self: &Self) -> bool` - Tell whether this delay-load import descriptor is the null descriptor

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageDelayloadDescriptor`



## object::pe::ImageDosHeader

*Struct*

DOS .EXE header

**Fields:**
- `e_magic: crate::endian::U16<crate::endian::LittleEndian>` - Magic number
- `e_cblp: crate::endian::U16<crate::endian::LittleEndian>` - Bytes on last page of file
- `e_cp: crate::endian::U16<crate::endian::LittleEndian>` - Pages in file
- `e_crlc: crate::endian::U16<crate::endian::LittleEndian>` - Relocations
- `e_cparhdr: crate::endian::U16<crate::endian::LittleEndian>` - Size of header in paragraphs
- `e_minalloc: crate::endian::U16<crate::endian::LittleEndian>` - Minimum extra paragraphs needed
- `e_maxalloc: crate::endian::U16<crate::endian::LittleEndian>` - Maximum extra paragraphs needed
- `e_ss: crate::endian::U16<crate::endian::LittleEndian>` - Initial (relative) SS value
- `e_sp: crate::endian::U16<crate::endian::LittleEndian>` - Initial SP value
- `e_csum: crate::endian::U16<crate::endian::LittleEndian>` - Checksum
- `e_ip: crate::endian::U16<crate::endian::LittleEndian>` - Initial IP value
- `e_cs: crate::endian::U16<crate::endian::LittleEndian>` - Initial (relative) CS value
- `e_lfarlc: crate::endian::U16<crate::endian::LittleEndian>` - File address of relocation table
- `e_ovno: crate::endian::U16<crate::endian::LittleEndian>` - Overlay number
- `e_res: [crate::endian::U16<crate::endian::LittleEndian>; 4]` - Reserved words
- `e_oemid: crate::endian::U16<crate::endian::LittleEndian>` - OEM identifier (for e_oeminfo)
- `e_oeminfo: crate::endian::U16<crate::endian::LittleEndian>` - OEM information; e_oemid specific
- `e_res2: [crate::endian::U16<crate::endian::LittleEndian>; 10]` - Reserved words
- `e_lfanew: crate::endian::U32<crate::endian::LittleEndian>` - File address of new exe header

**Methods:**

- `fn parse<'data, R>(data: R) -> read::Result<&'data Self>` - Read the DOS header.
- `fn nt_headers_offset(self: &Self) -> u32` - Return the file offset of the nt_headers.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageDosHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageDynamicRelocation32

*Struct*

**Fields:**
- `symbol: crate::endian::U32<crate::endian::LittleEndian>`
- `base_reloc_size: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageDynamicRelocation32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageDynamicRelocation32V2

*Struct*

**Fields:**
- `header_size: crate::endian::U32<crate::endian::LittleEndian>`
- `fixup_info_size: crate::endian::U32<crate::endian::LittleEndian>`
- `symbol: crate::endian::U32<crate::endian::LittleEndian>`
- `symbol_group: crate::endian::U32<crate::endian::LittleEndian>`
- `flags: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageDynamicRelocation32V2`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageDynamicRelocation64

*Struct*

**Fields:**
- `symbol: crate::endian::U64<crate::endian::LittleEndian>`
- `base_reloc_size: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageDynamicRelocation64`



## object::pe::ImageDynamicRelocation64V2

*Struct*

**Fields:**
- `header_size: crate::endian::U32<crate::endian::LittleEndian>`
- `fixup_info_size: crate::endian::U32<crate::endian::LittleEndian>`
- `symbol: crate::endian::U64<crate::endian::LittleEndian>`
- `symbol_group: crate::endian::U32<crate::endian::LittleEndian>`
- `flags: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageDynamicRelocation64V2`



## object::pe::ImageDynamicRelocationTable

*Struct*

**Fields:**
- `version: crate::endian::U32<crate::endian::LittleEndian>`
- `size: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageDynamicRelocationTable`



## object::pe::ImageEnclaveConfig32

*Struct*

**Fields:**
- `size: crate::endian::U32<crate::endian::LittleEndian>`
- `minimum_required_config_size: crate::endian::U32<crate::endian::LittleEndian>`
- `policy_flags: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_imports: crate::endian::U32<crate::endian::LittleEndian>`
- `import_list: crate::endian::U32<crate::endian::LittleEndian>`
- `import_entry_size: crate::endian::U32<crate::endian::LittleEndian>`
- `family_id: [u8; 16]`
- `image_id: [u8; 16]`
- `image_version: crate::endian::U32<crate::endian::LittleEndian>`
- `security_version: crate::endian::U32<crate::endian::LittleEndian>`
- `enclave_size: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_threads: crate::endian::U32<crate::endian::LittleEndian>`
- `enclave_flags: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageEnclaveConfig32`



## object::pe::ImageEnclaveConfig64

*Struct*

**Fields:**
- `size: crate::endian::U32<crate::endian::LittleEndian>`
- `minimum_required_config_size: crate::endian::U32<crate::endian::LittleEndian>`
- `policy_flags: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_imports: crate::endian::U32<crate::endian::LittleEndian>`
- `import_list: crate::endian::U32<crate::endian::LittleEndian>`
- `import_entry_size: crate::endian::U32<crate::endian::LittleEndian>`
- `family_id: [u8; 16]`
- `image_id: [u8; 16]`
- `image_version: crate::endian::U32<crate::endian::LittleEndian>`
- `security_version: crate::endian::U32<crate::endian::LittleEndian>`
- `enclave_size: crate::endian::U64<crate::endian::LittleEndian>`
- `number_of_threads: crate::endian::U32<crate::endian::LittleEndian>`
- `enclave_flags: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageEnclaveConfig64`



## object::pe::ImageEnclaveImport

*Struct*

**Fields:**
- `match_type: crate::endian::U32<crate::endian::LittleEndian>`
- `minimum_security_version: crate::endian::U32<crate::endian::LittleEndian>`
- `unique_or_author_id: [u8; 32]`
- `family_id: [u8; 16]`
- `image_id: [u8; 16]`
- `import_name: crate::endian::U32<crate::endian::LittleEndian>`
- `reserved: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageEnclaveImport`



## object::pe::ImageEpilogueDynamicRelocationHeader

*Struct*

**Fields:**
- `epilogue_count: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `epilogue_byte_count: u8`
- `branch_descriptor_element_size: u8`
- `branch_descriptor_count: crate::endian::U16Bytes<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageEpilogueDynamicRelocationHeader`



## object::pe::ImageExportDirectory

*Struct*

**Fields:**
- `characteristics: crate::endian::U32<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `major_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_version: crate::endian::U16<crate::endian::LittleEndian>`
- `name: crate::endian::U32<crate::endian::LittleEndian>`
- `base: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_functions: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_names: crate::endian::U32<crate::endian::LittleEndian>`
- `address_of_functions: crate::endian::U32<crate::endian::LittleEndian>` - RVA from base of image
- `address_of_names: crate::endian::U32<crate::endian::LittleEndian>` - RVA from base of image
- `address_of_name_ordinals: crate::endian::U32<crate::endian::LittleEndian>` - RVA from base of image

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageExportDirectory`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageFileHeader

*Struct*

**Fields:**
- `machine: crate::endian::U16<crate::endian::LittleEndian>`
- `number_of_sections: crate::endian::U16<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `pointer_to_symbol_table: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_symbols: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_optional_header: crate::endian::U16<crate::endian::LittleEndian>`
- `characteristics: crate::endian::U16<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageFileHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CoffHeader**
  - `fn is_type_bigobj() -> bool`
  - `fn machine(self: &Self) -> u16`
  - `fn number_of_sections(self: &Self) -> u32`
  - `fn pointer_to_symbol_table(self: &Self) -> u32`
  - `fn number_of_symbols(self: &Self) -> u32`
  - `fn characteristics(self: &Self) -> u16`
  - `fn parse<'data, R>(data: R, offset: & mut u64) -> read::Result<&'data Self>`



## object::pe::ImageFunctionEntry

*Struct*

**Fields:**
- `starting_address: crate::endian::U32<crate::endian::LittleEndian>`
- `ending_address: crate::endian::U32<crate::endian::LittleEndian>`
- `end_of_prologue: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageFunctionEntry`



## object::pe::ImageFunctionEntry64

*Struct*

**Fields:**
- `starting_address: crate::endian::U64<crate::endian::LittleEndian>`
- `ending_address: crate::endian::U64<crate::endian::LittleEndian>`
- `end_of_prologue_or_unwind_info_address: crate::endian::U64<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageFunctionEntry64`



## object::pe::ImageHotPatchBase

*Struct*

**Fields:**
- `sequence_number: crate::endian::U32<crate::endian::LittleEndian>`
- `flags: crate::endian::U32<crate::endian::LittleEndian>`
- `original_time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `original_check_sum: crate::endian::U32<crate::endian::LittleEndian>`
- `code_integrity_info: crate::endian::U32<crate::endian::LittleEndian>`
- `code_integrity_size: crate::endian::U32<crate::endian::LittleEndian>`
- `patch_table: crate::endian::U32<crate::endian::LittleEndian>`
- `buffer_offset: crate::endian::U32<crate::endian::LittleEndian>` - Version 2 and later

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageHotPatchBase`



## object::pe::ImageHotPatchHashes

*Struct*

**Fields:**
- `sha256: [u8; 32]`
- `sha1: [u8; 20]`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageHotPatchHashes`



## object::pe::ImageHotPatchInfo

*Struct*

**Fields:**
- `version: crate::endian::U32<crate::endian::LittleEndian>`
- `size: crate::endian::U32<crate::endian::LittleEndian>`
- `sequence_number: crate::endian::U32<crate::endian::LittleEndian>`
- `base_image_list: crate::endian::U32<crate::endian::LittleEndian>`
- `base_image_count: crate::endian::U32<crate::endian::LittleEndian>`
- `buffer_offset: crate::endian::U32<crate::endian::LittleEndian>` - Version 2 and later
- `extra_patch_size: crate::endian::U32<crate::endian::LittleEndian>` - Version 3 and later

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageHotPatchInfo`



## object::pe::ImageImportByName

*Struct*

**Fields:**
- `hint: crate::endian::U16<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageImportByName`



## object::pe::ImageImportDescriptor

*Struct*

**Fields:**
- `original_first_thunk: crate::endian::U32Bytes<crate::endian::LittleEndian>` - RVA to original unbound IAT (`ImageThunkData32`/`ImageThunkData64`)
- `time_date_stamp: crate::endian::U32Bytes<crate::endian::LittleEndian>` - 0 if not bound,
- `forwarder_chain: crate::endian::U32Bytes<crate::endian::LittleEndian>` - -1 if no forwarders
- `name: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `first_thunk: crate::endian::U32Bytes<crate::endian::LittleEndian>` - RVA to IAT (if bound this IAT has actual addresses)

**Methods:**

- `fn is_null(self: &Self) -> bool` - Tell whether this import descriptor is the null descriptor

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageImportDescriptor`



## object::pe::ImageLinenumber

*Struct*

**Fields:**
- `symbol_table_index_or_virtual_address: crate::endian::U32Bytes<crate::endian::LittleEndian>` - Symbol table index of function name if Linenumber is 0.
- `linenumber: crate::endian::U16Bytes<crate::endian::LittleEndian>` - Line number.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageLinenumber`



## object::pe::ImageLoadConfigCodeIntegrity

*Struct*

**Fields:**
- `flags: crate::endian::U16<crate::endian::LittleEndian>` - Flags to indicate if CI information is available, etc.
- `catalog: crate::endian::U16<crate::endian::LittleEndian>` - 0xFFFF means not available
- `catalog_offset: crate::endian::U32<crate::endian::LittleEndian>`
- `reserved: crate::endian::U32<crate::endian::LittleEndian>` - Additional bitmask to be defined later

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageLoadConfigCodeIntegrity`



## object::pe::ImageLoadConfigDirectory32

*Struct*

**Fields:**
- `size: crate::endian::U32<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `major_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_version: crate::endian::U16<crate::endian::LittleEndian>`
- `global_flags_clear: crate::endian::U32<crate::endian::LittleEndian>`
- `global_flags_set: crate::endian::U32<crate::endian::LittleEndian>`
- `critical_section_default_timeout: crate::endian::U32<crate::endian::LittleEndian>`
- `de_commit_free_block_threshold: crate::endian::U32<crate::endian::LittleEndian>`
- `de_commit_total_free_threshold: crate::endian::U32<crate::endian::LittleEndian>`
- `lock_prefix_table: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `maximum_allocation_size: crate::endian::U32<crate::endian::LittleEndian>`
- `virtual_memory_threshold: crate::endian::U32<crate::endian::LittleEndian>`
- `process_heap_flags: crate::endian::U32<crate::endian::LittleEndian>`
- `process_affinity_mask: crate::endian::U32<crate::endian::LittleEndian>`
- `csd_version: crate::endian::U16<crate::endian::LittleEndian>`
- `dependent_load_flags: crate::endian::U16<crate::endian::LittleEndian>`
- `edit_list: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `security_cookie: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `sehandler_table: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `sehandler_count: crate::endian::U32<crate::endian::LittleEndian>`
- `guard_cf_check_function_pointer: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `guard_cf_dispatch_function_pointer: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `guard_cf_function_table: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `guard_cf_function_count: crate::endian::U32<crate::endian::LittleEndian>`
- `guard_flags: crate::endian::U32<crate::endian::LittleEndian>`
- `code_integrity: ImageLoadConfigCodeIntegrity`
- `guard_address_taken_iat_entry_table: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `guard_address_taken_iat_entry_count: crate::endian::U32<crate::endian::LittleEndian>`
- `guard_long_jump_target_table: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `guard_long_jump_target_count: crate::endian::U32<crate::endian::LittleEndian>`
- `dynamic_value_reloc_table: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `chpe_metadata_pointer: crate::endian::U32<crate::endian::LittleEndian>`
- `guard_rf_failure_routine: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `guard_rf_failure_routine_function_pointer: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `dynamic_value_reloc_table_offset: crate::endian::U32<crate::endian::LittleEndian>`
- `dynamic_value_reloc_table_section: crate::endian::U16<crate::endian::LittleEndian>`
- `reserved2: crate::endian::U16<crate::endian::LittleEndian>`
- `guard_rf_verify_stack_pointer_function_pointer: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `hot_patch_table_offset: crate::endian::U32<crate::endian::LittleEndian>`
- `reserved3: crate::endian::U32<crate::endian::LittleEndian>`
- `enclave_configuration_pointer: crate::endian::U32<crate::endian::LittleEndian>` - VA
- `volatile_metadata_pointer: crate::endian::U32<crate::endian::LittleEndian>` - VA

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageLoadConfigDirectory32`



## object::pe::ImageLoadConfigDirectory64

*Struct*

**Fields:**
- `size: crate::endian::U32<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `major_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_version: crate::endian::U16<crate::endian::LittleEndian>`
- `global_flags_clear: crate::endian::U32<crate::endian::LittleEndian>`
- `global_flags_set: crate::endian::U32<crate::endian::LittleEndian>`
- `critical_section_default_timeout: crate::endian::U32<crate::endian::LittleEndian>`
- `de_commit_free_block_threshold: crate::endian::U64<crate::endian::LittleEndian>`
- `de_commit_total_free_threshold: crate::endian::U64<crate::endian::LittleEndian>`
- `lock_prefix_table: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `maximum_allocation_size: crate::endian::U64<crate::endian::LittleEndian>`
- `virtual_memory_threshold: crate::endian::U64<crate::endian::LittleEndian>`
- `process_affinity_mask: crate::endian::U64<crate::endian::LittleEndian>`
- `process_heap_flags: crate::endian::U32<crate::endian::LittleEndian>`
- `csd_version: crate::endian::U16<crate::endian::LittleEndian>`
- `dependent_load_flags: crate::endian::U16<crate::endian::LittleEndian>`
- `edit_list: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `security_cookie: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `sehandler_table: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `sehandler_count: crate::endian::U64<crate::endian::LittleEndian>`
- `guard_cf_check_function_pointer: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `guard_cf_dispatch_function_pointer: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `guard_cf_function_table: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `guard_cf_function_count: crate::endian::U64<crate::endian::LittleEndian>`
- `guard_flags: crate::endian::U32<crate::endian::LittleEndian>`
- `code_integrity: ImageLoadConfigCodeIntegrity`
- `guard_address_taken_iat_entry_table: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `guard_address_taken_iat_entry_count: crate::endian::U64<crate::endian::LittleEndian>`
- `guard_long_jump_target_table: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `guard_long_jump_target_count: crate::endian::U64<crate::endian::LittleEndian>`
- `dynamic_value_reloc_table: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `chpe_metadata_pointer: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `guard_rf_failure_routine: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `guard_rf_failure_routine_function_pointer: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `dynamic_value_reloc_table_offset: crate::endian::U32<crate::endian::LittleEndian>`
- `dynamic_value_reloc_table_section: crate::endian::U16<crate::endian::LittleEndian>`
- `reserved2: crate::endian::U16<crate::endian::LittleEndian>`
- `guard_rf_verify_stack_pointer_function_pointer: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `hot_patch_table_offset: crate::endian::U32<crate::endian::LittleEndian>`
- `reserved3: crate::endian::U32<crate::endian::LittleEndian>`
- `enclave_configuration_pointer: crate::endian::U64<crate::endian::LittleEndian>` - VA
- `volatile_metadata_pointer: crate::endian::U64<crate::endian::LittleEndian>` - VA

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageLoadConfigDirectory64`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageNtHeaders32

*Struct*

**Fields:**
- `signature: crate::endian::U32<crate::endian::LittleEndian>`
- `file_header: ImageFileHeader`
- `optional_header: ImageOptionalHeader32`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ImageNtHeaders**
  - `fn is_type_64(self: &Self) -> bool`
  - `fn is_valid_optional_magic(self: &Self) -> bool`
  - `fn signature(self: &Self) -> u32`
  - `fn file_header(self: &Self) -> &pe::ImageFileHeader`
  - `fn optional_header(self: &Self) -> &<Self as >::ImageOptionalHeader`
- **Clone**
  - `fn clone(self: &Self) -> ImageNtHeaders32`



## object::pe::ImageNtHeaders64

*Struct*

**Fields:**
- `signature: crate::endian::U32<crate::endian::LittleEndian>`
- `file_header: ImageFileHeader`
- `optional_header: ImageOptionalHeader64`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageNtHeaders64`
- **ImageNtHeaders**
  - `fn is_type_64(self: &Self) -> bool`
  - `fn is_valid_optional_magic(self: &Self) -> bool`
  - `fn signature(self: &Self) -> u32`
  - `fn file_header(self: &Self) -> &pe::ImageFileHeader`
  - `fn optional_header(self: &Self) -> &<Self as >::ImageOptionalHeader`



## object::pe::ImageOptionalHeader32

*Struct*

**Fields:**
- `magic: crate::endian::U16<crate::endian::LittleEndian>`
- `major_linker_version: u8`
- `minor_linker_version: u8`
- `size_of_code: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_initialized_data: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_uninitialized_data: crate::endian::U32<crate::endian::LittleEndian>`
- `address_of_entry_point: crate::endian::U32<crate::endian::LittleEndian>`
- `base_of_code: crate::endian::U32<crate::endian::LittleEndian>`
- `base_of_data: crate::endian::U32<crate::endian::LittleEndian>`
- `image_base: crate::endian::U32<crate::endian::LittleEndian>`
- `section_alignment: crate::endian::U32<crate::endian::LittleEndian>`
- `file_alignment: crate::endian::U32<crate::endian::LittleEndian>`
- `major_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>`
- `major_image_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_image_version: crate::endian::U16<crate::endian::LittleEndian>`
- `major_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>`
- `win32_version_value: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_image: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_headers: crate::endian::U32<crate::endian::LittleEndian>`
- `check_sum: crate::endian::U32<crate::endian::LittleEndian>`
- `subsystem: crate::endian::U16<crate::endian::LittleEndian>`
- `dll_characteristics: crate::endian::U16<crate::endian::LittleEndian>`
- `size_of_stack_reserve: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_stack_commit: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_heap_reserve: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_heap_commit: crate::endian::U32<crate::endian::LittleEndian>`
- `loader_flags: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_rva_and_sizes: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ImageOptionalHeader**
  - `fn magic(self: &Self) -> u16`
  - `fn major_linker_version(self: &Self) -> u8`
  - `fn minor_linker_version(self: &Self) -> u8`
  - `fn size_of_code(self: &Self) -> u32`
  - `fn size_of_initialized_data(self: &Self) -> u32`
  - `fn size_of_uninitialized_data(self: &Self) -> u32`
  - `fn address_of_entry_point(self: &Self) -> u32`
  - `fn base_of_code(self: &Self) -> u32`
  - `fn base_of_data(self: &Self) -> Option<u32>`
  - `fn image_base(self: &Self) -> u64`
  - `fn section_alignment(self: &Self) -> u32`
  - `fn file_alignment(self: &Self) -> u32`
  - `fn major_operating_system_version(self: &Self) -> u16`
  - `fn minor_operating_system_version(self: &Self) -> u16`
  - `fn major_image_version(self: &Self) -> u16`
  - `fn minor_image_version(self: &Self) -> u16`
  - `fn major_subsystem_version(self: &Self) -> u16`
  - `fn minor_subsystem_version(self: &Self) -> u16`
  - `fn win32_version_value(self: &Self) -> u32`
  - `fn size_of_image(self: &Self) -> u32`
  - `fn size_of_headers(self: &Self) -> u32`
  - `fn check_sum(self: &Self) -> u32`
  - `fn subsystem(self: &Self) -> u16`
  - `fn dll_characteristics(self: &Self) -> u16`
  - `fn size_of_stack_reserve(self: &Self) -> u64`
  - `fn size_of_stack_commit(self: &Self) -> u64`
  - `fn size_of_heap_reserve(self: &Self) -> u64`
  - `fn size_of_heap_commit(self: &Self) -> u64`
  - `fn loader_flags(self: &Self) -> u32`
  - `fn number_of_rva_and_sizes(self: &Self) -> u32`
- **Clone**
  - `fn clone(self: &Self) -> ImageOptionalHeader32`



## object::pe::ImageOptionalHeader64

*Struct*

**Fields:**
- `magic: crate::endian::U16<crate::endian::LittleEndian>`
- `major_linker_version: u8`
- `minor_linker_version: u8`
- `size_of_code: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_initialized_data: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_uninitialized_data: crate::endian::U32<crate::endian::LittleEndian>`
- `address_of_entry_point: crate::endian::U32<crate::endian::LittleEndian>`
- `base_of_code: crate::endian::U32<crate::endian::LittleEndian>`
- `image_base: crate::endian::U64<crate::endian::LittleEndian>`
- `section_alignment: crate::endian::U32<crate::endian::LittleEndian>`
- `file_alignment: crate::endian::U32<crate::endian::LittleEndian>`
- `major_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>`
- `major_image_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_image_version: crate::endian::U16<crate::endian::LittleEndian>`
- `major_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>`
- `win32_version_value: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_image: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_headers: crate::endian::U32<crate::endian::LittleEndian>`
- `check_sum: crate::endian::U32<crate::endian::LittleEndian>`
- `subsystem: crate::endian::U16<crate::endian::LittleEndian>`
- `dll_characteristics: crate::endian::U16<crate::endian::LittleEndian>`
- `size_of_stack_reserve: crate::endian::U64<crate::endian::LittleEndian>`
- `size_of_stack_commit: crate::endian::U64<crate::endian::LittleEndian>`
- `size_of_heap_reserve: crate::endian::U64<crate::endian::LittleEndian>`
- `size_of_heap_commit: crate::endian::U64<crate::endian::LittleEndian>`
- `loader_flags: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_rva_and_sizes: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageOptionalHeader64`
- **ImageOptionalHeader**
  - `fn magic(self: &Self) -> u16`
  - `fn major_linker_version(self: &Self) -> u8`
  - `fn minor_linker_version(self: &Self) -> u8`
  - `fn size_of_code(self: &Self) -> u32`
  - `fn size_of_initialized_data(self: &Self) -> u32`
  - `fn size_of_uninitialized_data(self: &Self) -> u32`
  - `fn address_of_entry_point(self: &Self) -> u32`
  - `fn base_of_code(self: &Self) -> u32`
  - `fn base_of_data(self: &Self) -> Option<u32>`
  - `fn image_base(self: &Self) -> u64`
  - `fn section_alignment(self: &Self) -> u32`
  - `fn file_alignment(self: &Self) -> u32`
  - `fn major_operating_system_version(self: &Self) -> u16`
  - `fn minor_operating_system_version(self: &Self) -> u16`
  - `fn major_image_version(self: &Self) -> u16`
  - `fn minor_image_version(self: &Self) -> u16`
  - `fn major_subsystem_version(self: &Self) -> u16`
  - `fn minor_subsystem_version(self: &Self) -> u16`
  - `fn win32_version_value(self: &Self) -> u32`
  - `fn size_of_image(self: &Self) -> u32`
  - `fn size_of_headers(self: &Self) -> u32`
  - `fn check_sum(self: &Self) -> u32`
  - `fn subsystem(self: &Self) -> u16`
  - `fn dll_characteristics(self: &Self) -> u16`
  - `fn size_of_stack_reserve(self: &Self) -> u64`
  - `fn size_of_stack_commit(self: &Self) -> u64`
  - `fn size_of_heap_reserve(self: &Self) -> u64`
  - `fn size_of_heap_commit(self: &Self) -> u64`
  - `fn loader_flags(self: &Self) -> u32`
  - `fn number_of_rva_and_sizes(self: &Self) -> u32`



## object::pe::ImageOs2Header

*Struct*

OS/2 .EXE header

**Fields:**
- `ne_magic: crate::endian::U16<crate::endian::LittleEndian>` - Magic number
- `ne_ver: i8` - Version number
- `ne_rev: i8` - Revision number
- `ne_enttab: crate::endian::U16<crate::endian::LittleEndian>` - Offset of Entry Table
- `ne_cbenttab: crate::endian::U16<crate::endian::LittleEndian>` - Number of bytes in Entry Table
- `ne_crc: crate::endian::I32<crate::endian::LittleEndian>` - Checksum of whole file
- `ne_flags: crate::endian::U16<crate::endian::LittleEndian>` - Flag word
- `ne_autodata: crate::endian::U16<crate::endian::LittleEndian>` - Automatic data segment number
- `ne_heap: crate::endian::U16<crate::endian::LittleEndian>` - Initial heap allocation
- `ne_stack: crate::endian::U16<crate::endian::LittleEndian>` - Initial stack allocation
- `ne_csip: crate::endian::I32<crate::endian::LittleEndian>` - Initial CS:IP setting
- `ne_sssp: crate::endian::I32<crate::endian::LittleEndian>` - Initial SS:SP setting
- `ne_cseg: crate::endian::U16<crate::endian::LittleEndian>` - Count of file segments
- `ne_cmod: crate::endian::U16<crate::endian::LittleEndian>` - Entries in Module Reference Table
- `ne_cbnrestab: crate::endian::U16<crate::endian::LittleEndian>` - Size of non-resident name table
- `ne_segtab: crate::endian::U16<crate::endian::LittleEndian>` - Offset of Segment Table
- `ne_rsrctab: crate::endian::U16<crate::endian::LittleEndian>` - Offset of Resource Table
- `ne_restab: crate::endian::U16<crate::endian::LittleEndian>` - Offset of resident name table
- `ne_modtab: crate::endian::U16<crate::endian::LittleEndian>` - Offset of Module Reference Table
- `ne_imptab: crate::endian::U16<crate::endian::LittleEndian>` - Offset of Imported Names Table
- `ne_nrestab: crate::endian::I32<crate::endian::LittleEndian>` - Offset of Non-resident Names Table
- `ne_cmovent: crate::endian::U16<crate::endian::LittleEndian>` - Count of movable entries
- `ne_align: crate::endian::U16<crate::endian::LittleEndian>` - Segment alignment shift count
- `ne_cres: crate::endian::U16<crate::endian::LittleEndian>` - Count of resource segments
- `ne_exetyp: u8` - Target Operating system
- `ne_flagsothers: u8` - Other .EXE flags
- `ne_pretthunks: crate::endian::U16<crate::endian::LittleEndian>` - offset to return thunks
- `ne_psegrefbytes: crate::endian::U16<crate::endian::LittleEndian>` - offset to segment ref. bytes
- `ne_swaparea: crate::endian::U16<crate::endian::LittleEndian>` - Minimum code swap area size
- `ne_expver: crate::endian::U16<crate::endian::LittleEndian>` - Expected Windows version number

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageOs2Header`



## object::pe::ImagePrologueDynamicRelocationHeader

*Struct*

**Fields:**
- `prologue_byte_count: u8`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImagePrologueDynamicRelocationHeader`



## object::pe::ImageRelocation

*Struct*

**Fields:**
- `virtual_address: crate::endian::U32Bytes<crate::endian::LittleEndian>` - Also `RelocCount` when IMAGE_SCN_LNK_NRELOC_OVFL is set
- `symbol_table_index: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `typ: crate::endian::U16Bytes<crate::endian::LittleEndian>`

**Methods:**

- `fn symbol(self: &Self) -> SymbolIndex` - Get the index of the symbol referenced by this relocation.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageRelocation`



## object::pe::ImageResourceDataEntry

*Struct*

**Fields:**
- `offset_to_data: crate::endian::U32<crate::endian::LittleEndian>` - RVA of the data.
- `size: crate::endian::U32<crate::endian::LittleEndian>`
- `code_page: crate::endian::U32<crate::endian::LittleEndian>`
- `reserved: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageResourceDataEntry`



## object::pe::ImageResourceDirStringU

*Struct*

**Fields:**
- `length: crate::endian::U16<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageResourceDirStringU`



## object::pe::ImageResourceDirectory

*Struct*

**Fields:**
- `characteristics: crate::endian::U32<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `major_version: crate::endian::U16<crate::endian::LittleEndian>`
- `minor_version: crate::endian::U16<crate::endian::LittleEndian>`
- `number_of_named_entries: crate::endian::U16<crate::endian::LittleEndian>`
- `number_of_id_entries: crate::endian::U16<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageResourceDirectory`



## object::pe::ImageResourceDirectoryEntry

*Struct*

**Fields:**
- `name_or_id: crate::endian::U32<crate::endian::LittleEndian>`
- `offset_to_data_or_directory: crate::endian::U32<crate::endian::LittleEndian>`

**Methods:**

- `fn has_name(self: &Self) -> bool` - Returns true if the entry has a name, rather than an ID.
- `fn name_or_id(self: &Self) -> ResourceNameOrId` - Returns the entry name
- `fn is_table(self: &Self) -> bool` - Returns true if the entry is a subtable.
- `fn data_offset(self: &Self) -> u32` - Returns the section offset of the associated table or data.
- `fn data<'data>(self: &Self, section: ResourceDirectory<'data>) -> Result<ResourceDirectoryEntryData<'data>>` - Returns the data associated to this directory entry.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageResourceDirectoryEntry`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageResourceDirectoryString

*Struct*

**Fields:**
- `length: crate::endian::U16<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageResourceDirectoryString`



## object::pe::ImageRomHeaders

*Struct*

**Fields:**
- `file_header: ImageFileHeader`
- `optional_header: ImageRomOptionalHeader`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageRomHeaders`



## object::pe::ImageRomOptionalHeader

*Struct*

**Fields:**
- `magic: crate::endian::U16<crate::endian::LittleEndian>`
- `major_linker_version: u8`
- `minor_linker_version: u8`
- `size_of_code: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_initialized_data: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_uninitialized_data: crate::endian::U32<crate::endian::LittleEndian>`
- `address_of_entry_point: crate::endian::U32<crate::endian::LittleEndian>`
- `base_of_code: crate::endian::U32<crate::endian::LittleEndian>`
- `base_of_data: crate::endian::U32<crate::endian::LittleEndian>`
- `base_of_bss: crate::endian::U32<crate::endian::LittleEndian>`
- `gpr_mask: crate::endian::U32<crate::endian::LittleEndian>`
- `cpr_mask: [crate::endian::U32<crate::endian::LittleEndian>; 4]`
- `gp_value: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageRomOptionalHeader`



## object::pe::ImageRuntimeFunctionEntry

*Struct*

**Fields:**
- `begin_address: crate::endian::U32<crate::endian::LittleEndian>`
- `end_address: crate::endian::U32<crate::endian::LittleEndian>`
- `unwind_info_address_or_data: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageRuntimeFunctionEntry`



## object::pe::ImageSectionHeader

*Struct*

**Fields:**
- `name: [u8; 8]`
- `virtual_size: crate::endian::U32<crate::endian::LittleEndian>`
- `virtual_address: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>`
- `pointer_to_raw_data: crate::endian::U32<crate::endian::LittleEndian>`
- `pointer_to_relocations: crate::endian::U32<crate::endian::LittleEndian>`
- `pointer_to_linenumbers: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_relocations: crate::endian::U16<crate::endian::LittleEndian>`
- `number_of_linenumbers: crate::endian::U16<crate::endian::LittleEndian>`
- `characteristics: crate::endian::U32<crate::endian::LittleEndian>`

**Methods:**

- `fn name_offset(self: &Self) -> Result<Option<u32>>` - Return the string table offset of the section name.
- `fn name<'data, R>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>` - Return the section name.
- `fn raw_name(self: &Self) -> &[u8]` - Return the raw section name.
- `fn coff_file_range(self: &Self) -> Option<(u32, u32)>` - Return the offset and size of the section in a COFF file.
- `fn coff_data<'data, R>(self: &Self, data: R) -> result::Result<&'data [u8], ()>` - Return the section data in a COFF file.
- `fn coff_alignment(self: &Self) -> u64` - Return the section alignment in bytes.
- `fn coff_relocations<'data, R>(self: &Self, data: R) -> read::Result<&'data [pe::ImageRelocation]>` - Read the relocations in a COFF file.
- `fn pe_file_range(self: &Self) -> (u32, u32)` - Return the offset and size of the section in a PE file.
- `fn pe_file_range_at(self: &Self, va: u32) -> Option<(u32, u32)>` - Return the file offset of the given virtual address, and the remaining size up
- `fn pe_address_range(self: &Self) -> (u32, u32)` - Return the virtual address and size of the section.
- `fn pe_data<'data, R>(self: &Self, data: R) -> Result<&'data [u8]>` - Return the section data in a PE file.
- `fn pe_data_at<'data, R>(self: &Self, data: R, va: u32) -> Option<&'data [u8]>` - Return the data starting at the given virtual address, up to the end of the
- `fn contains_rva(self: &Self, va: u32) -> bool` - Tests whether a given RVA is part of this section
- `fn pe_data_containing<'data, R>(self: &Self, data: R, va: u32) -> Option<(&'data [u8], u32)>` - Return the section data if it contains the given virtual address.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> ImageSectionHeader`
- **Clone**
  - `fn clone(self: &Self) -> ImageSectionHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageSeparateDebugHeader

*Struct*

**Fields:**
- `signature: crate::endian::U16<crate::endian::LittleEndian>`
- `flags: crate::endian::U16<crate::endian::LittleEndian>`
- `machine: crate::endian::U16<crate::endian::LittleEndian>`
- `characteristics: crate::endian::U16<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `check_sum: crate::endian::U32<crate::endian::LittleEndian>`
- `image_base: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_image: crate::endian::U32<crate::endian::LittleEndian>`
- `number_of_sections: crate::endian::U32<crate::endian::LittleEndian>`
- `exported_names_size: crate::endian::U32<crate::endian::LittleEndian>`
- `debug_directory_size: crate::endian::U32<crate::endian::LittleEndian>`
- `section_alignment: crate::endian::U32<crate::endian::LittleEndian>`
- `reserved: [crate::endian::U32<crate::endian::LittleEndian>; 2]`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageSeparateDebugHeader`



## object::pe::ImageSymbol

*Struct*

**Fields:**
- `name: [u8; 8]` - If first 4 bytes are 0, then second 4 bytes are offset into string table.
- `value: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `section_number: crate::endian::U16Bytes<crate::endian::LittleEndian>`
- `typ: crate::endian::U16Bytes<crate::endian::LittleEndian>`
- `storage_class: u8`
- `number_of_aux_symbols: u8`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageSymbol`
- **ImageSymbol**
  - `fn raw_name(self: &Self) -> &[u8; 8]`
  - `fn value(self: &Self) -> u32`
  - `fn section_number(self: &Self) -> i32`
  - `fn typ(self: &Self) -> u16`
  - `fn storage_class(self: &Self) -> u8`
  - `fn number_of_aux_symbols(self: &Self) -> u8`



## object::pe::ImageSymbolBytes

*Struct*

**Tuple Struct**: `([u8; 18])`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageSymbolBytes`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageSymbolEx

*Struct*

**Fields:**
- `name: [u8; 8]` - If first 4 bytes are 0, then second 4 bytes are offset into string table.
- `value: crate::endian::U32Bytes<crate::endian::LittleEndian>`
- `section_number: crate::endian::I32Bytes<crate::endian::LittleEndian>`
- `typ: crate::endian::U16Bytes<crate::endian::LittleEndian>`
- `storage_class: u8`
- `number_of_aux_symbols: u8`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageSymbolEx`
- **ImageSymbol**
  - `fn raw_name(self: &Self) -> &[u8; 8]`
  - `fn value(self: &Self) -> u32`
  - `fn section_number(self: &Self) -> i32`
  - `fn typ(self: &Self) -> u16`
  - `fn storage_class(self: &Self) -> u8`
  - `fn number_of_aux_symbols(self: &Self) -> u8`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageSymbolExBytes

*Struct*

**Tuple Struct**: `([u8; 20])`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageSymbolExBytes`



## object::pe::ImageThunkData32

*Struct*

**Tuple Struct**: `(crate::endian::U32<crate::endian::LittleEndian>)`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ImageThunkData**
  - `fn raw(self: Self) -> u64`
  - `fn is_ordinal(self: Self) -> bool`
  - `fn ordinal(self: Self) -> u16`
  - `fn address(self: Self) -> u32`
- **Clone**
  - `fn clone(self: &Self) -> ImageThunkData32`



## object::pe::ImageThunkData64

*Struct*

**Tuple Struct**: `(crate::endian::U64<crate::endian::LittleEndian>)`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageThunkData64`
- **ImageThunkData**
  - `fn raw(self: Self) -> u64`
  - `fn is_ordinal(self: Self) -> bool`
  - `fn ordinal(self: Self) -> u16`
  - `fn address(self: Self) -> u32`



## object::pe::ImageTlsDirectory32

*Struct*

**Fields:**
- `start_address_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>`
- `end_address_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>`
- `address_of_index: crate::endian::U32<crate::endian::LittleEndian>` - PDWORD
- `address_of_call_backs: crate::endian::U32<crate::endian::LittleEndian>` - PIMAGE_TLS_CALLBACK *
- `size_of_zero_fill: crate::endian::U32<crate::endian::LittleEndian>`
- `characteristics: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImageTlsDirectory32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::ImageTlsDirectory64

*Struct*

**Fields:**
- `start_address_of_raw_data: crate::endian::U64<crate::endian::LittleEndian>`
- `end_address_of_raw_data: crate::endian::U64<crate::endian::LittleEndian>`
- `address_of_index: crate::endian::U64<crate::endian::LittleEndian>` - PDWORD
- `address_of_call_backs: crate::endian::U64<crate::endian::LittleEndian>` - PIMAGE_TLS_CALLBACK *;
- `size_of_zero_fill: crate::endian::U32<crate::endian::LittleEndian>`
- `characteristics: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageTlsDirectory64`



## object::pe::ImageVxdHeader

*Struct*

Windows VXD header

**Fields:**
- `e32_magic: crate::endian::U16<crate::endian::LittleEndian>` - Magic number
- `e32_border: u8` - The byte ordering for the VXD
- `e32_worder: u8` - The word ordering for the VXD
- `e32_level: crate::endian::U32<crate::endian::LittleEndian>` - The EXE format level for now = 0
- `e32_cpu: crate::endian::U16<crate::endian::LittleEndian>` - The CPU type
- `e32_os: crate::endian::U16<crate::endian::LittleEndian>` - The OS type
- `e32_ver: crate::endian::U32<crate::endian::LittleEndian>` - Module version
- `e32_mflags: crate::endian::U32<crate::endian::LittleEndian>` - Module flags
- `e32_mpages: crate::endian::U32<crate::endian::LittleEndian>` - Module # pages
- `e32_startobj: crate::endian::U32<crate::endian::LittleEndian>` - Object # for instruction pointer
- `e32_eip: crate::endian::U32<crate::endian::LittleEndian>` - Extended instruction pointer
- `e32_stackobj: crate::endian::U32<crate::endian::LittleEndian>` - Object # for stack pointer
- `e32_esp: crate::endian::U32<crate::endian::LittleEndian>` - Extended stack pointer
- `e32_pagesize: crate::endian::U32<crate::endian::LittleEndian>` - VXD page size
- `e32_lastpagesize: crate::endian::U32<crate::endian::LittleEndian>` - Last page size in VXD
- `e32_fixupsize: crate::endian::U32<crate::endian::LittleEndian>` - Fixup section size
- `e32_fixupsum: crate::endian::U32<crate::endian::LittleEndian>` - Fixup section checksum
- `e32_ldrsize: crate::endian::U32<crate::endian::LittleEndian>` - Loader section size
- `e32_ldrsum: crate::endian::U32<crate::endian::LittleEndian>` - Loader section checksum
- `e32_objtab: crate::endian::U32<crate::endian::LittleEndian>` - Object table offset
- `e32_objcnt: crate::endian::U32<crate::endian::LittleEndian>` - Number of objects in module
- `e32_objmap: crate::endian::U32<crate::endian::LittleEndian>` - Object page map offset
- `e32_itermap: crate::endian::U32<crate::endian::LittleEndian>` - Object iterated data map offset
- `e32_rsrctab: crate::endian::U32<crate::endian::LittleEndian>` - Offset of Resource Table
- `e32_rsrccnt: crate::endian::U32<crate::endian::LittleEndian>` - Number of resource entries
- `e32_restab: crate::endian::U32<crate::endian::LittleEndian>` - Offset of resident name table
- `e32_enttab: crate::endian::U32<crate::endian::LittleEndian>` - Offset of Entry Table
- `e32_dirtab: crate::endian::U32<crate::endian::LittleEndian>` - Offset of Module Directive Table
- `e32_dircnt: crate::endian::U32<crate::endian::LittleEndian>` - Number of module directives
- `e32_fpagetab: crate::endian::U32<crate::endian::LittleEndian>` - Offset of Fixup Page Table
- `e32_frectab: crate::endian::U32<crate::endian::LittleEndian>` - Offset of Fixup Record Table
- `e32_impmod: crate::endian::U32<crate::endian::LittleEndian>` - Offset of Import Module Name Table
- `e32_impmodcnt: crate::endian::U32<crate::endian::LittleEndian>` - Number of entries in Import Module Name Table
- `e32_impproc: crate::endian::U32<crate::endian::LittleEndian>` - Offset of Import Procedure Name Table
- `e32_pagesum: crate::endian::U32<crate::endian::LittleEndian>` - Offset of Per-Page Checksum Table
- `e32_datapage: crate::endian::U32<crate::endian::LittleEndian>` - Offset of Enumerated Data Pages
- `e32_preload: crate::endian::U32<crate::endian::LittleEndian>` - Number of preload pages
- `e32_nrestab: crate::endian::U32<crate::endian::LittleEndian>` - Offset of Non-resident Names Table
- `e32_cbnrestab: crate::endian::U32<crate::endian::LittleEndian>` - Size of Non-resident Name Table
- `e32_nressum: crate::endian::U32<crate::endian::LittleEndian>` - Non-resident Name Table Checksum
- `e32_autodata: crate::endian::U32<crate::endian::LittleEndian>` - Object # for automatic data object
- `e32_debuginfo: crate::endian::U32<crate::endian::LittleEndian>` - Offset of the debugging information
- `e32_debuglen: crate::endian::U32<crate::endian::LittleEndian>` - The length of the debugging info. in bytes
- `e32_instpreload: crate::endian::U32<crate::endian::LittleEndian>` - Number of instance pages in preload section of VXD file
- `e32_instdemand: crate::endian::U32<crate::endian::LittleEndian>` - Number of instance pages in demand load section of VXD file
- `e32_heapsize: crate::endian::U32<crate::endian::LittleEndian>` - Size of heap - for 16-bit apps
- `e32_res3: [u8; 12]` - Reserved words
- `e32_winresoff: crate::endian::U32<crate::endian::LittleEndian>`
- `e32_winreslen: crate::endian::U32<crate::endian::LittleEndian>`
- `e32_devid: crate::endian::U16<crate::endian::LittleEndian>` - Device ID for VxD
- `e32_ddkver: crate::endian::U16<crate::endian::LittleEndian>` - DDK version for VxD

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ImageVxdHeader`



## object::pe::ImportObjectHeader

*Struct*

**Fields:**
- `sig1: crate::endian::U16<crate::endian::LittleEndian>` - Must be IMAGE_FILE_MACHINE_UNKNOWN
- `sig2: crate::endian::U16<crate::endian::LittleEndian>` - Must be IMPORT_OBJECT_HDR_SIG2.
- `version: crate::endian::U16<crate::endian::LittleEndian>`
- `machine: crate::endian::U16<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>` - Time/date stamp
- `size_of_data: crate::endian::U32<crate::endian::LittleEndian>` - particularly useful for incremental links
- `ordinal_or_hint: crate::endian::U16<crate::endian::LittleEndian>` - if grf & IMPORT_OBJECT_ORDINAL
- `name_type: crate::endian::U16<crate::endian::LittleEndian>`

**Methods:**

- `fn parse<'data, R>(data: R, offset: & mut u64) -> Result<&'data Self>` - Read the short import header.
- `fn parse_data<'data, R>(self: &Self, data: R, offset: & mut u64) -> Result<ImportObjectData<'data>>` - Parse the data following the header.
- `fn import_type(self: &Self) -> u16` - The type of import.
- `fn name_type(self: &Self) -> u16` - The type of import name.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ImportObjectHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::MAX_CLASS_NAME

*Constant*: `usize`



## object::pe::MAX_PACKAGE_NAME

*Constant*: `usize`



## object::pe::MaskedRichHeaderEntry

*Struct*

A PE rich header entry.

Rich headers have no official documentation, but have been heavily
reversed-engineered and documented in the wild, e.g.:
* `http://www.ntcore.com/files/richsign.htm`
* `https://www.researchgate.net/figure/Structure-of-the-Rich-Header_fig1_318145388`

This data is "masked", i.e. XORed with a checksum derived from the file data.

**Fields:**
- `masked_comp_id: crate::endian::U32<crate::endian::LittleEndian>`
- `masked_count: crate::endian::U32<crate::endian::LittleEndian>`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> MaskedRichHeaderEntry`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::pe::NATIVE_TYPE_MAX_CB

*Constant*: `u16`



## object::pe::NON_PAGED_DEBUG_SIGNATURE

*Constant*: `u16`



## object::pe::N_BTMASK

*Constant*: `u16`



## object::pe::N_BTSHFT

*Constant*: `usize`



## object::pe::N_TMASK

*Constant*: `u16`



## object::pe::N_TMASK1

*Constant*: `u16`



## object::pe::N_TMASK2

*Constant*: `u16`



## object::pe::N_TSHIFT

*Constant*: `usize`



## object::pe::NonPagedDebugInfo

*Struct*

**Fields:**
- `signature: crate::endian::U16<crate::endian::LittleEndian>`
- `flags: crate::endian::U16<crate::endian::LittleEndian>`
- `size: crate::endian::U32<crate::endian::LittleEndian>`
- `machine: crate::endian::U16<crate::endian::LittleEndian>`
- `characteristics: crate::endian::U16<crate::endian::LittleEndian>`
- `time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>`
- `check_sum: crate::endian::U32<crate::endian::LittleEndian>`
- `size_of_image: crate::endian::U32<crate::endian::LittleEndian>`
- `image_base: crate::endian::U64<crate::endian::LittleEndian>`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> NonPagedDebugInfo`



## object::pe::RT_ACCELERATOR

*Constant*: `u16`

ID for: Accelerator table.



## object::pe::RT_ANICURSOR

*Constant*: `u16`

ID for: Animated cursor.



## object::pe::RT_ANIICON

*Constant*: `u16`

ID for: Animated icon.



## object::pe::RT_BITMAP

*Constant*: `u16`

ID for: Bitmap resource.



## object::pe::RT_CURSOR

*Constant*: `u16`

ID for: Hardware-dependent cursor resource.



## object::pe::RT_DIALOG

*Constant*: `u16`

ID for: Dialog box.



## object::pe::RT_DLGINCLUDE

*Constant*: `u16`

ID for: Allows a resource editing tool to associate a string with an .rc file.



## object::pe::RT_FONT

*Constant*: `u16`

ID for: Font resource.



## object::pe::RT_FONTDIR

*Constant*: `u16`

ID for: Font directory resource.



## object::pe::RT_GROUP_CURSOR

*Constant*: `u16`

ID for: Hardware-independent cursor resource.



## object::pe::RT_GROUP_ICON

*Constant*: `u16`

ID for: Hardware-independent icon resource.



## object::pe::RT_HTML

*Constant*: `u16`

ID for: HTML resource.



## object::pe::RT_ICON

*Constant*: `u16`

ID for: Hardware-dependent icon resource.



## object::pe::RT_MANIFEST

*Constant*: `u16`

ID for: Side-by-Side Assembly Manifest.



## object::pe::RT_MENU

*Constant*: `u16`

ID for: Menu resource.



## object::pe::RT_MESSAGETABLE

*Constant*: `u16`

ID for: Message-table entry.



## object::pe::RT_PLUGPLAY

*Constant*: `u16`

ID for: Plug and Play resource.



## object::pe::RT_RCDATA

*Constant*: `u16`

ID for: Application-defined resource (raw data).



## object::pe::RT_STRING

*Constant*: `u16`

ID for: String-table entry.



## object::pe::RT_VERSION

*Constant*: `u16`

ID for: Version resource.



## object::pe::RT_VXD

*Constant*: `u16`

ID for: VXD.



## object::pe::X3_BTYPE_QP_INST_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_BTYPE_QP_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_BTYPE_QP_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_BTYPE_QP_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_D_WH_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_D_WH_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_D_WH_SIGN_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_D_WH_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_EMPTY_INST_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_EMPTY_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_EMPTY_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_EMPTY_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM20_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM20_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM20_SIGN_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM20_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM39_1_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM39_1_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM39_1_SIGN_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM39_1_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM39_2_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM39_2_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM39_2_SIGN_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_IMM39_2_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_I_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_I_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_I_SIGN_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_I_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_OPCODE_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_OPCODE_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_OPCODE_SIGN_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_OPCODE_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_P_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_P_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_P_SIGN_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_P_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_TMPLT_INST_WORD_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_TMPLT_INST_WORD_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_TMPLT_SIGN_VAL_POS_X

*Constant*: `u16`

Intel-IA64-Filler



## object::pe::X3_TMPLT_SIZE_X

*Constant*: `u16`

Intel-IA64-Filler



