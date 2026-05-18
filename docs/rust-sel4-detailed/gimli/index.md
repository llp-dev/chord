# Crate `gimli`

`gimli` is a library for reading and writing the
[DWARF debugging format](https://dwarfstd.org/).

See the [read](./read/index.html) and [write](./write/index.html) modules
for examples and API documentation.

## Cargo Features

Cargo features that can be enabled with `gimli`:

* `std`: Enabled by default. Use the `std` library. Disabling this feature allows
  using `gimli` in embedded environments that do not have access to `std`.

* `read`: Enabled by default. Enables the `read` module. Use of `std` is optional.
  Requires the `alloc` crate.

* `read-core`: Enabled by default. Enables a subset of the `read` module that does
  not require the `alloc` crate.

* `endian-reader`: Enabled by default. Enables the [`EndianReader`](read/index.md) trait. This is
  a separate feature because it also depends on the `stable_deref_trait` crate.

* `write`: Enabled by default. Enables the `write` module. Use of `std` is optional.
  Requires the `alloc` crate. Enabling both `read` and `write` will also enable
  conversion functionality in the `write` module.


## Contents

- [Modules](#modules)
  - [`common`](#common)
  - [`arch`](#arch)
  - [`constants`](#constants)
  - [`endianity`](#endianity)
  - [`leb128`](#leb128)
  - [`case_fold`](#case-fold)
  - [`read`](#read)
  - [`util`](#util)
  - [`addr`](#addr)
  - [`cfi`](#cfi)
  - [`dwarf`](#dwarf)
  - [`endian_slice`](#endian-slice)
  - [`endian_reader`](#endian-reader)
  - [`reader`](#reader)
  - [`relocate`](#relocate)
  - [`abbrev`](#abbrev)
  - [`aranges`](#aranges)
  - [`index`](#index)
  - [`line`](#line)
  - [`lists`](#lists)
  - [`loclists`](#loclists)
  - [`lookup`](#lookup)
  - [`macros`](#macros)
  - [`names`](#names)
  - [`op`](#op)
  - [`pubnames`](#pubnames)
  - [`pubtypes`](#pubtypes)
  - [`rnglists`](#rnglists)
  - [`str`](#str)
  - [`unit`](#unit)
  - [`value`](#value)
- [Structs](#structs)
  - [`Encoding`](#encoding)
  - [`LineEncoding`](#lineencoding)
  - [`Register`](#register)
  - [`DebugAbbrevOffset`](#debugabbrevoffset)
  - [`DebugAddrOffset`](#debugaddroffset)
  - [`DebugAddrBase`](#debugaddrbase)
  - [`DebugAddrIndex`](#debugaddrindex)
  - [`DebugArangesOffset`](#debugarangesoffset)
  - [`DebugInfoOffset`](#debuginfooffset)
  - [`DebugLineOffset`](#debuglineoffset)
  - [`DebugLineStrOffset`](#debuglinestroffset)
  - [`LocationListsOffset`](#locationlistsoffset)
  - [`DebugLocListsBase`](#debugloclistsbase)
  - [`DebugLocListsIndex`](#debugloclistsindex)
  - [`DebugMacinfoOffset`](#debugmacinfooffset)
  - [`DebugMacroOffset`](#debugmacrooffset)
  - [`DebugNamesOffset`](#debugnamesoffset)
  - [`RawRangeListsOffset`](#rawrangelistsoffset)
  - [`RangeListsOffset`](#rangelistsoffset)
  - [`DebugRngListsBase`](#debugrnglistsbase)
  - [`DebugRngListsIndex`](#debugrnglistsindex)
  - [`DebugStrOffset`](#debugstroffset)
  - [`DebugStrOffsetsBase`](#debugstroffsetsbase)
  - [`DebugStrOffsetsIndex`](#debugstroffsetsindex)
  - [`DebugTypesOffset`](#debugtypesoffset)
  - [`DebugTypeSignature`](#debugtypesignature)
  - [`DebugFrameOffset`](#debugframeoffset)
  - [`EhFrameOffset`](#ehframeoffset)
  - [`UnitSectionOffset`](#unitsectionoffset)
  - [`DwoId`](#dwoid)
  - [`Arm`](#arm)
  - [`AArch64`](#aarch64)
  - [`LoongArch`](#loongarch)
  - [`MIPS`](#mips)
  - [`RiscV`](#riscv)
  - [`X86`](#x86)
  - [`X86_64`](#x86-64)
  - [`PowerPc64`](#powerpc64)
  - [`DwSect`](#dwsect)
  - [`DwSectV2`](#dwsectv2)
  - [`DwUt`](#dwut)
  - [`DwCfa`](#dwcfa)
  - [`DwChildren`](#dwchildren)
  - [`DwTag`](#dwtag)
  - [`DwAt`](#dwat)
  - [`DwForm`](#dwform)
  - [`DwAte`](#dwate)
  - [`DwLle`](#dwlle)
  - [`DwDs`](#dwds)
  - [`DwEnd`](#dwend)
  - [`DwAccess`](#dwaccess)
  - [`DwVis`](#dwvis)
  - [`DwVirtuality`](#dwvirtuality)
  - [`DwLang`](#dwlang)
  - [`DwAddr`](#dwaddr)
  - [`DwId`](#dwid)
  - [`DwCc`](#dwcc)
  - [`DwInl`](#dwinl)
  - [`DwOrd`](#dword)
  - [`DwDsc`](#dwdsc)
  - [`DwIdx`](#dwidx)
  - [`DwDefaulted`](#dwdefaulted)
  - [`DwLns`](#dwlns)
  - [`DwLne`](#dwlne)
  - [`DwLnct`](#dwlnct)
  - [`DwMacinfo`](#dwmacinfo)
  - [`DwMacro`](#dwmacro)
  - [`DwRle`](#dwrle)
  - [`DwOp`](#dwop)
  - [`DwEhPe`](#dwehpe)
  - [`LittleEndian`](#littleendian)
  - [`BigEndian`](#bigendian)
  - [`UnitOffset`](#unitoffset)
  - [`StoreOnHeap`](#storeonheap)
- [Enums](#enums)
  - [`Format`](#format)
  - [`Vendor`](#vendor)
  - [`SectionId`](#sectionid)
  - [`DwarfFileType`](#dwarffiletype)
  - [`RunTimeEndian`](#runtimeendian)
  - [`Error`](#error)
- [Traits](#traits)
  - [`Endianity`](#endianity)
  - [`Section`](#section)
- [Functions](#functions)
  - [`case_fold_data`](#case-fold-data)
  - [`case_fold`](#case-fold)
  - [`case_folding_djb_hash`](#case-folding-djb-hash)
  - [`djb_hash_byte`](#djb-hash-byte)
- [Type Aliases](#type-aliases)
  - [`NativeEndian`](#nativeendian)
  - [`Result`](#result)
- [Constants](#constants)
  - [`DW_SECT_INFO`](#dw-sect-info)
  - [`DW_SECT_ABBREV`](#dw-sect-abbrev)
  - [`DW_SECT_LINE`](#dw-sect-line)
  - [`DW_SECT_LOCLISTS`](#dw-sect-loclists)
  - [`DW_SECT_STR_OFFSETS`](#dw-sect-str-offsets)
  - [`DW_SECT_MACRO`](#dw-sect-macro)
  - [`DW_SECT_RNGLISTS`](#dw-sect-rnglists)
  - [`DW_SECT_V2_INFO`](#dw-sect-v2-info)
  - [`DW_SECT_V2_TYPES`](#dw-sect-v2-types)
  - [`DW_SECT_V2_ABBREV`](#dw-sect-v2-abbrev)
  - [`DW_SECT_V2_LINE`](#dw-sect-v2-line)
  - [`DW_SECT_V2_LOC`](#dw-sect-v2-loc)
  - [`DW_SECT_V2_STR_OFFSETS`](#dw-sect-v2-str-offsets)
  - [`DW_SECT_V2_MACINFO`](#dw-sect-v2-macinfo)
  - [`DW_SECT_V2_MACRO`](#dw-sect-v2-macro)
  - [`DW_UT_compile`](#dw-ut-compile)
  - [`DW_UT_type`](#dw-ut-type)
  - [`DW_UT_partial`](#dw-ut-partial)
  - [`DW_UT_skeleton`](#dw-ut-skeleton)
  - [`DW_UT_split_compile`](#dw-ut-split-compile)
  - [`DW_UT_split_type`](#dw-ut-split-type)
  - [`DW_UT_lo_user`](#dw-ut-lo-user)
  - [`DW_UT_hi_user`](#dw-ut-hi-user)
  - [`DW_CFA_advance_loc`](#dw-cfa-advance-loc)
  - [`DW_CFA_offset`](#dw-cfa-offset)
  - [`DW_CFA_restore`](#dw-cfa-restore)
  - [`DW_CFA_nop`](#dw-cfa-nop)
  - [`DW_CFA_set_loc`](#dw-cfa-set-loc)
  - [`DW_CFA_advance_loc1`](#dw-cfa-advance-loc1)
  - [`DW_CFA_advance_loc2`](#dw-cfa-advance-loc2)
  - [`DW_CFA_advance_loc4`](#dw-cfa-advance-loc4)
  - [`DW_CFA_offset_extended`](#dw-cfa-offset-extended)
  - [`DW_CFA_restore_extended`](#dw-cfa-restore-extended)
  - [`DW_CFA_undefined`](#dw-cfa-undefined)
  - [`DW_CFA_same_value`](#dw-cfa-same-value)
  - [`DW_CFA_register`](#dw-cfa-register)
  - [`DW_CFA_remember_state`](#dw-cfa-remember-state)
  - [`DW_CFA_restore_state`](#dw-cfa-restore-state)
  - [`DW_CFA_def_cfa`](#dw-cfa-def-cfa)
  - [`DW_CFA_def_cfa_register`](#dw-cfa-def-cfa-register)
  - [`DW_CFA_def_cfa_offset`](#dw-cfa-def-cfa-offset)
  - [`DW_CFA_def_cfa_expression`](#dw-cfa-def-cfa-expression)
  - [`DW_CFA_expression`](#dw-cfa-expression)
  - [`DW_CFA_offset_extended_sf`](#dw-cfa-offset-extended-sf)
  - [`DW_CFA_def_cfa_sf`](#dw-cfa-def-cfa-sf)
  - [`DW_CFA_def_cfa_offset_sf`](#dw-cfa-def-cfa-offset-sf)
  - [`DW_CFA_val_offset`](#dw-cfa-val-offset)
  - [`DW_CFA_val_offset_sf`](#dw-cfa-val-offset-sf)
  - [`DW_CFA_val_expression`](#dw-cfa-val-expression)
  - [`DW_CFA_lo_user`](#dw-cfa-lo-user)
  - [`DW_CFA_hi_user`](#dw-cfa-hi-user)
  - [`DW_CFA_MIPS_advance_loc8`](#dw-cfa-mips-advance-loc8)
  - [`DW_CFA_GNU_window_save`](#dw-cfa-gnu-window-save)
  - [`DW_CFA_GNU_args_size`](#dw-cfa-gnu-args-size)
  - [`DW_CFA_GNU_negative_offset_extended`](#dw-cfa-gnu-negative-offset-extended)
  - [`DW_CFA_AARCH64_negate_ra_state`](#dw-cfa-aarch64-negate-ra-state)
  - [`DW_CHILDREN_no`](#dw-children-no)
  - [`DW_CHILDREN_yes`](#dw-children-yes)
  - [`DW_TAG_null`](#dw-tag-null)
  - [`DW_TAG_global_subroutine`](#dw-tag-global-subroutine)
  - [`DW_TAG_global_variable`](#dw-tag-global-variable)
  - [`DW_TAG_local_variable`](#dw-tag-local-variable)
  - [`DW_TAG_subroutine`](#dw-tag-subroutine)
  - [`DW_TAG_array_type`](#dw-tag-array-type)
  - [`DW_TAG_class_type`](#dw-tag-class-type)
  - [`DW_TAG_entry_point`](#dw-tag-entry-point)
  - [`DW_TAG_enumeration_type`](#dw-tag-enumeration-type)
  - [`DW_TAG_formal_parameter`](#dw-tag-formal-parameter)
  - [`DW_TAG_imported_declaration`](#dw-tag-imported-declaration)
  - [`DW_TAG_label`](#dw-tag-label)
  - [`DW_TAG_lexical_block`](#dw-tag-lexical-block)
  - [`DW_TAG_member`](#dw-tag-member)
  - [`DW_TAG_pointer_type`](#dw-tag-pointer-type)
  - [`DW_TAG_reference_type`](#dw-tag-reference-type)
  - [`DW_TAG_compile_unit`](#dw-tag-compile-unit)
  - [`DW_TAG_string_type`](#dw-tag-string-type)
  - [`DW_TAG_structure_type`](#dw-tag-structure-type)
  - [`DW_TAG_subroutine_type`](#dw-tag-subroutine-type)
  - [`DW_TAG_typedef`](#dw-tag-typedef)
  - [`DW_TAG_union_type`](#dw-tag-union-type)
  - [`DW_TAG_unspecified_parameters`](#dw-tag-unspecified-parameters)
  - [`DW_TAG_variant`](#dw-tag-variant)
  - [`DW_TAG_common_block`](#dw-tag-common-block)
  - [`DW_TAG_common_inclusion`](#dw-tag-common-inclusion)
  - [`DW_TAG_inheritance`](#dw-tag-inheritance)
  - [`DW_TAG_inlined_subroutine`](#dw-tag-inlined-subroutine)
  - [`DW_TAG_module`](#dw-tag-module)
  - [`DW_TAG_ptr_to_member_type`](#dw-tag-ptr-to-member-type)
  - [`DW_TAG_set_type`](#dw-tag-set-type)
  - [`DW_TAG_subrange_type`](#dw-tag-subrange-type)
  - [`DW_TAG_with_stmt`](#dw-tag-with-stmt)
  - [`DW_TAG_access_declaration`](#dw-tag-access-declaration)
  - [`DW_TAG_base_type`](#dw-tag-base-type)
  - [`DW_TAG_catch_block`](#dw-tag-catch-block)
  - [`DW_TAG_const_type`](#dw-tag-const-type)
  - [`DW_TAG_constant`](#dw-tag-constant)
  - [`DW_TAG_enumerator`](#dw-tag-enumerator)
  - [`DW_TAG_file_type`](#dw-tag-file-type)
  - [`DW_TAG_friend`](#dw-tag-friend)
  - [`DW_TAG_namelist`](#dw-tag-namelist)
  - [`DW_TAG_namelist_item`](#dw-tag-namelist-item)
  - [`DW_TAG_packed_type`](#dw-tag-packed-type)
  - [`DW_TAG_subprogram`](#dw-tag-subprogram)
  - [`DW_TAG_template_type_parameter`](#dw-tag-template-type-parameter)
  - [`DW_TAG_template_value_parameter`](#dw-tag-template-value-parameter)
  - [`DW_TAG_thrown_type`](#dw-tag-thrown-type)
  - [`DW_TAG_try_block`](#dw-tag-try-block)
  - [`DW_TAG_variant_part`](#dw-tag-variant-part)
  - [`DW_TAG_variable`](#dw-tag-variable)
  - [`DW_TAG_volatile_type`](#dw-tag-volatile-type)
  - [`DW_TAG_dwarf_procedure`](#dw-tag-dwarf-procedure)
  - [`DW_TAG_restrict_type`](#dw-tag-restrict-type)
  - [`DW_TAG_interface_type`](#dw-tag-interface-type)
  - [`DW_TAG_namespace`](#dw-tag-namespace)
  - [`DW_TAG_imported_module`](#dw-tag-imported-module)
  - [`DW_TAG_unspecified_type`](#dw-tag-unspecified-type)
  - [`DW_TAG_partial_unit`](#dw-tag-partial-unit)
  - [`DW_TAG_imported_unit`](#dw-tag-imported-unit)
  - [`DW_TAG_condition`](#dw-tag-condition)
  - [`DW_TAG_shared_type`](#dw-tag-shared-type)
  - [`DW_TAG_type_unit`](#dw-tag-type-unit)
  - [`DW_TAG_rvalue_reference_type`](#dw-tag-rvalue-reference-type)
  - [`DW_TAG_template_alias`](#dw-tag-template-alias)
  - [`DW_TAG_coarray_type`](#dw-tag-coarray-type)
  - [`DW_TAG_generic_subrange`](#dw-tag-generic-subrange)
  - [`DW_TAG_dynamic_type`](#dw-tag-dynamic-type)
  - [`DW_TAG_atomic_type`](#dw-tag-atomic-type)
  - [`DW_TAG_call_site`](#dw-tag-call-site)
  - [`DW_TAG_call_site_parameter`](#dw-tag-call-site-parameter)
  - [`DW_TAG_skeleton_unit`](#dw-tag-skeleton-unit)
  - [`DW_TAG_immutable_type`](#dw-tag-immutable-type)
  - [`DW_TAG_lo_user`](#dw-tag-lo-user)
  - [`DW_TAG_hi_user`](#dw-tag-hi-user)
  - [`DW_TAG_MIPS_loop`](#dw-tag-mips-loop)
  - [`DW_TAG_HP_array_descriptor`](#dw-tag-hp-array-descriptor)
  - [`DW_TAG_HP_Bliss_field`](#dw-tag-hp-bliss-field)
  - [`DW_TAG_HP_Bliss_field_set`](#dw-tag-hp-bliss-field-set)
  - [`DW_TAG_format_label`](#dw-tag-format-label)
  - [`DW_TAG_function_template`](#dw-tag-function-template)
  - [`DW_TAG_class_template`](#dw-tag-class-template)
  - [`DW_TAG_GNU_BINCL`](#dw-tag-gnu-bincl)
  - [`DW_TAG_GNU_EINCL`](#dw-tag-gnu-eincl)
  - [`DW_TAG_GNU_template_template_param`](#dw-tag-gnu-template-template-param)
  - [`DW_TAG_GNU_template_parameter_pack`](#dw-tag-gnu-template-parameter-pack)
  - [`DW_TAG_GNU_formal_parameter_pack`](#dw-tag-gnu-formal-parameter-pack)
  - [`DW_TAG_GNU_call_site`](#dw-tag-gnu-call-site)
  - [`DW_TAG_GNU_call_site_parameter`](#dw-tag-gnu-call-site-parameter)
  - [`DW_TAG_APPLE_property`](#dw-tag-apple-property)
  - [`DW_TAG_SUN_function_template`](#dw-tag-sun-function-template)
  - [`DW_TAG_SUN_class_template`](#dw-tag-sun-class-template)
  - [`DW_TAG_SUN_struct_template`](#dw-tag-sun-struct-template)
  - [`DW_TAG_SUN_union_template`](#dw-tag-sun-union-template)
  - [`DW_TAG_SUN_indirect_inheritance`](#dw-tag-sun-indirect-inheritance)
  - [`DW_TAG_SUN_codeflags`](#dw-tag-sun-codeflags)
  - [`DW_TAG_SUN_memop_info`](#dw-tag-sun-memop-info)
  - [`DW_TAG_SUN_omp_child_func`](#dw-tag-sun-omp-child-func)
  - [`DW_TAG_SUN_rtti_descriptor`](#dw-tag-sun-rtti-descriptor)
  - [`DW_TAG_SUN_dtor_info`](#dw-tag-sun-dtor-info)
  - [`DW_TAG_SUN_dtor`](#dw-tag-sun-dtor)
  - [`DW_TAG_SUN_f90_interface`](#dw-tag-sun-f90-interface)
  - [`DW_TAG_SUN_fortran_vax_structure`](#dw-tag-sun-fortran-vax-structure)
  - [`DW_TAG_ALTIUM_circ_type`](#dw-tag-altium-circ-type)
  - [`DW_TAG_ALTIUM_mwa_circ_type`](#dw-tag-altium-mwa-circ-type)
  - [`DW_TAG_ALTIUM_rev_carry_type`](#dw-tag-altium-rev-carry-type)
  - [`DW_TAG_ALTIUM_rom`](#dw-tag-altium-rom)
  - [`DW_TAG_upc_shared_type`](#dw-tag-upc-shared-type)
  - [`DW_TAG_upc_strict_type`](#dw-tag-upc-strict-type)
  - [`DW_TAG_upc_relaxed_type`](#dw-tag-upc-relaxed-type)
  - [`DW_TAG_PGI_kanji_type`](#dw-tag-pgi-kanji-type)
  - [`DW_TAG_PGI_interface_block`](#dw-tag-pgi-interface-block)
  - [`DW_TAG_BORLAND_property`](#dw-tag-borland-property)
  - [`DW_TAG_BORLAND_Delphi_string`](#dw-tag-borland-delphi-string)
  - [`DW_TAG_BORLAND_Delphi_dynamic_array`](#dw-tag-borland-delphi-dynamic-array)
  - [`DW_TAG_BORLAND_Delphi_set`](#dw-tag-borland-delphi-set)
  - [`DW_TAG_BORLAND_Delphi_variant`](#dw-tag-borland-delphi-variant)
  - [`DW_AT_null`](#dw-at-null)
  - [`DW_AT_fund_type`](#dw-at-fund-type)
  - [`DW_AT_mod_fund_type`](#dw-at-mod-fund-type)
  - [`DW_AT_user_def_type`](#dw-at-user-def-type)
  - [`DW_AT_mod_u_d_type`](#dw-at-mod-u-d-type)
  - [`DW_AT_subscr_data`](#dw-at-subscr-data)
  - [`DW_AT_element_list`](#dw-at-element-list)
  - [`DW_AT_member`](#dw-at-member)
  - [`DW_AT_friends`](#dw-at-friends)
  - [`DW_AT_program`](#dw-at-program)
  - [`DW_AT_private`](#dw-at-private)
  - [`DW_AT_protected`](#dw-at-protected)
  - [`DW_AT_public`](#dw-at-public)
  - [`DW_AT_pure_virtual`](#dw-at-pure-virtual)
  - [`DW_AT_virtual`](#dw-at-virtual)
  - [`DW_AT_specification_v1`](#dw-at-specification-v1)
  - [`DW_AT_sibling`](#dw-at-sibling)
  - [`DW_AT_location`](#dw-at-location)
  - [`DW_AT_name`](#dw-at-name)
  - [`DW_AT_ordering`](#dw-at-ordering)
  - [`DW_AT_byte_size`](#dw-at-byte-size)
  - [`DW_AT_bit_offset`](#dw-at-bit-offset)
  - [`DW_AT_bit_size`](#dw-at-bit-size)
  - [`DW_AT_stmt_list`](#dw-at-stmt-list)
  - [`DW_AT_low_pc`](#dw-at-low-pc)
  - [`DW_AT_high_pc`](#dw-at-high-pc)
  - [`DW_AT_language`](#dw-at-language)
  - [`DW_AT_discr`](#dw-at-discr)
  - [`DW_AT_discr_value`](#dw-at-discr-value)
  - [`DW_AT_visibility`](#dw-at-visibility)
  - [`DW_AT_import`](#dw-at-import)
  - [`DW_AT_string_length`](#dw-at-string-length)
  - [`DW_AT_common_reference`](#dw-at-common-reference)
  - [`DW_AT_comp_dir`](#dw-at-comp-dir)
  - [`DW_AT_const_value`](#dw-at-const-value)
  - [`DW_AT_containing_type`](#dw-at-containing-type)
  - [`DW_AT_default_value`](#dw-at-default-value)
  - [`DW_AT_inline`](#dw-at-inline)
  - [`DW_AT_is_optional`](#dw-at-is-optional)
  - [`DW_AT_lower_bound`](#dw-at-lower-bound)
  - [`DW_AT_producer`](#dw-at-producer)
  - [`DW_AT_prototyped`](#dw-at-prototyped)
  - [`DW_AT_return_addr`](#dw-at-return-addr)
  - [`DW_AT_start_scope`](#dw-at-start-scope)
  - [`DW_AT_bit_stride`](#dw-at-bit-stride)
  - [`DW_AT_upper_bound`](#dw-at-upper-bound)
  - [`DW_AT_abstract_origin`](#dw-at-abstract-origin)
  - [`DW_AT_accessibility`](#dw-at-accessibility)
  - [`DW_AT_address_class`](#dw-at-address-class)
  - [`DW_AT_artificial`](#dw-at-artificial)
  - [`DW_AT_base_types`](#dw-at-base-types)
  - [`DW_AT_calling_convention`](#dw-at-calling-convention)
  - [`DW_AT_count`](#dw-at-count)
  - [`DW_AT_data_member_location`](#dw-at-data-member-location)
  - [`DW_AT_decl_column`](#dw-at-decl-column)
  - [`DW_AT_decl_file`](#dw-at-decl-file)
  - [`DW_AT_decl_line`](#dw-at-decl-line)
  - [`DW_AT_declaration`](#dw-at-declaration)
  - [`DW_AT_discr_list`](#dw-at-discr-list)
  - [`DW_AT_encoding`](#dw-at-encoding)
  - [`DW_AT_external`](#dw-at-external)
  - [`DW_AT_frame_base`](#dw-at-frame-base)
  - [`DW_AT_friend`](#dw-at-friend)
  - [`DW_AT_identifier_case`](#dw-at-identifier-case)
  - [`DW_AT_macro_info`](#dw-at-macro-info)
  - [`DW_AT_namelist_item`](#dw-at-namelist-item)
  - [`DW_AT_priority`](#dw-at-priority)
  - [`DW_AT_segment`](#dw-at-segment)
  - [`DW_AT_specification`](#dw-at-specification)
  - [`DW_AT_static_link`](#dw-at-static-link)
  - [`DW_AT_type`](#dw-at-type)
  - [`DW_AT_use_location`](#dw-at-use-location)
  - [`DW_AT_variable_parameter`](#dw-at-variable-parameter)
  - [`DW_AT_virtuality`](#dw-at-virtuality)
  - [`DW_AT_vtable_elem_location`](#dw-at-vtable-elem-location)
  - [`DW_AT_allocated`](#dw-at-allocated)
  - [`DW_AT_associated`](#dw-at-associated)
  - [`DW_AT_data_location`](#dw-at-data-location)
  - [`DW_AT_byte_stride`](#dw-at-byte-stride)
  - [`DW_AT_entry_pc`](#dw-at-entry-pc)
  - [`DW_AT_use_UTF8`](#dw-at-use-utf8)
  - [`DW_AT_extension`](#dw-at-extension)
  - [`DW_AT_ranges`](#dw-at-ranges)
  - [`DW_AT_trampoline`](#dw-at-trampoline)
  - [`DW_AT_call_column`](#dw-at-call-column)
  - [`DW_AT_call_file`](#dw-at-call-file)
  - [`DW_AT_call_line`](#dw-at-call-line)
  - [`DW_AT_description`](#dw-at-description)
  - [`DW_AT_binary_scale`](#dw-at-binary-scale)
  - [`DW_AT_decimal_scale`](#dw-at-decimal-scale)
  - [`DW_AT_small`](#dw-at-small)
  - [`DW_AT_decimal_sign`](#dw-at-decimal-sign)
  - [`DW_AT_digit_count`](#dw-at-digit-count)
  - [`DW_AT_picture_string`](#dw-at-picture-string)
  - [`DW_AT_mutable`](#dw-at-mutable)
  - [`DW_AT_threads_scaled`](#dw-at-threads-scaled)
  - [`DW_AT_explicit`](#dw-at-explicit)
  - [`DW_AT_object_pointer`](#dw-at-object-pointer)
  - [`DW_AT_endianity`](#dw-at-endianity)
  - [`DW_AT_elemental`](#dw-at-elemental)
  - [`DW_AT_pure`](#dw-at-pure)
  - [`DW_AT_recursive`](#dw-at-recursive)
  - [`DW_AT_signature`](#dw-at-signature)
  - [`DW_AT_main_subprogram`](#dw-at-main-subprogram)
  - [`DW_AT_data_bit_offset`](#dw-at-data-bit-offset)
  - [`DW_AT_const_expr`](#dw-at-const-expr)
  - [`DW_AT_enum_class`](#dw-at-enum-class)
  - [`DW_AT_linkage_name`](#dw-at-linkage-name)
  - [`DW_AT_string_length_bit_size`](#dw-at-string-length-bit-size)
  - [`DW_AT_string_length_byte_size`](#dw-at-string-length-byte-size)
  - [`DW_AT_rank`](#dw-at-rank)
  - [`DW_AT_str_offsets_base`](#dw-at-str-offsets-base)
  - [`DW_AT_addr_base`](#dw-at-addr-base)
  - [`DW_AT_rnglists_base`](#dw-at-rnglists-base)
  - [`DW_AT_dwo_name`](#dw-at-dwo-name)
  - [`DW_AT_reference`](#dw-at-reference)
  - [`DW_AT_rvalue_reference`](#dw-at-rvalue-reference)
  - [`DW_AT_macros`](#dw-at-macros)
  - [`DW_AT_call_all_calls`](#dw-at-call-all-calls)
  - [`DW_AT_call_all_source_calls`](#dw-at-call-all-source-calls)
  - [`DW_AT_call_all_tail_calls`](#dw-at-call-all-tail-calls)
  - [`DW_AT_call_return_pc`](#dw-at-call-return-pc)
  - [`DW_AT_call_value`](#dw-at-call-value)
  - [`DW_AT_call_origin`](#dw-at-call-origin)
  - [`DW_AT_call_parameter`](#dw-at-call-parameter)
  - [`DW_AT_call_pc`](#dw-at-call-pc)
  - [`DW_AT_call_tail_call`](#dw-at-call-tail-call)
  - [`DW_AT_call_target`](#dw-at-call-target)
  - [`DW_AT_call_target_clobbered`](#dw-at-call-target-clobbered)
  - [`DW_AT_call_data_location`](#dw-at-call-data-location)
  - [`DW_AT_call_data_value`](#dw-at-call-data-value)
  - [`DW_AT_noreturn`](#dw-at-noreturn)
  - [`DW_AT_alignment`](#dw-at-alignment)
  - [`DW_AT_export_symbols`](#dw-at-export-symbols)
  - [`DW_AT_deleted`](#dw-at-deleted)
  - [`DW_AT_defaulted`](#dw-at-defaulted)
  - [`DW_AT_loclists_base`](#dw-at-loclists-base)
  - [`DW_AT_lo_user`](#dw-at-lo-user)
  - [`DW_AT_hi_user`](#dw-at-hi-user)
  - [`DW_AT_MIPS_fde`](#dw-at-mips-fde)
  - [`DW_AT_MIPS_loop_begin`](#dw-at-mips-loop-begin)
  - [`DW_AT_MIPS_tail_loop_begin`](#dw-at-mips-tail-loop-begin)
  - [`DW_AT_MIPS_epilog_begin`](#dw-at-mips-epilog-begin)
  - [`DW_AT_MIPS_loop_unroll_factor`](#dw-at-mips-loop-unroll-factor)
  - [`DW_AT_MIPS_software_pipeline_depth`](#dw-at-mips-software-pipeline-depth)
  - [`DW_AT_MIPS_linkage_name`](#dw-at-mips-linkage-name)
  - [`DW_AT_MIPS_stride`](#dw-at-mips-stride)
  - [`DW_AT_MIPS_abstract_name`](#dw-at-mips-abstract-name)
  - [`DW_AT_MIPS_clone_origin`](#dw-at-mips-clone-origin)
  - [`DW_AT_MIPS_has_inlines`](#dw-at-mips-has-inlines)
  - [`DW_AT_MIPS_stride_byte`](#dw-at-mips-stride-byte)
  - [`DW_AT_MIPS_stride_elem`](#dw-at-mips-stride-elem)
  - [`DW_AT_MIPS_ptr_dopetype`](#dw-at-mips-ptr-dopetype)
  - [`DW_AT_MIPS_allocatable_dopetype`](#dw-at-mips-allocatable-dopetype)
  - [`DW_AT_MIPS_assumed_shape_dopetype`](#dw-at-mips-assumed-shape-dopetype)
  - [`DW_AT_MIPS_assumed_size`](#dw-at-mips-assumed-size)
  - [`DW_AT_INTEL_other_endian`](#dw-at-intel-other-endian)
  - [`DW_AT_sf_names`](#dw-at-sf-names)
  - [`DW_AT_src_info`](#dw-at-src-info)
  - [`DW_AT_mac_info`](#dw-at-mac-info)
  - [`DW_AT_src_coords`](#dw-at-src-coords)
  - [`DW_AT_body_begin`](#dw-at-body-begin)
  - [`DW_AT_body_end`](#dw-at-body-end)
  - [`DW_AT_GNU_vector`](#dw-at-gnu-vector)
  - [`DW_AT_GNU_guarded_by`](#dw-at-gnu-guarded-by)
  - [`DW_AT_GNU_pt_guarded_by`](#dw-at-gnu-pt-guarded-by)
  - [`DW_AT_GNU_guarded`](#dw-at-gnu-guarded)
  - [`DW_AT_GNU_pt_guarded`](#dw-at-gnu-pt-guarded)
  - [`DW_AT_GNU_locks_excluded`](#dw-at-gnu-locks-excluded)
  - [`DW_AT_GNU_exclusive_locks_required`](#dw-at-gnu-exclusive-locks-required)
  - [`DW_AT_GNU_shared_locks_required`](#dw-at-gnu-shared-locks-required)
  - [`DW_AT_GNU_odr_signature`](#dw-at-gnu-odr-signature)
  - [`DW_AT_GNU_template_name`](#dw-at-gnu-template-name)
  - [`DW_AT_GNU_call_site_value`](#dw-at-gnu-call-site-value)
  - [`DW_AT_GNU_call_site_data_value`](#dw-at-gnu-call-site-data-value)
  - [`DW_AT_GNU_call_site_target`](#dw-at-gnu-call-site-target)
  - [`DW_AT_GNU_call_site_target_clobbered`](#dw-at-gnu-call-site-target-clobbered)
  - [`DW_AT_GNU_tail_call`](#dw-at-gnu-tail-call)
  - [`DW_AT_GNU_all_tail_call_sites`](#dw-at-gnu-all-tail-call-sites)
  - [`DW_AT_GNU_all_call_sites`](#dw-at-gnu-all-call-sites)
  - [`DW_AT_GNU_all_source_call_sites`](#dw-at-gnu-all-source-call-sites)
  - [`DW_AT_GNU_macros`](#dw-at-gnu-macros)
  - [`DW_AT_GNU_deleted`](#dw-at-gnu-deleted)
  - [`DW_AT_GNU_dwo_name`](#dw-at-gnu-dwo-name)
  - [`DW_AT_GNU_dwo_id`](#dw-at-gnu-dwo-id)
  - [`DW_AT_GNU_ranges_base`](#dw-at-gnu-ranges-base)
  - [`DW_AT_GNU_addr_base`](#dw-at-gnu-addr-base)
  - [`DW_AT_GNU_pubnames`](#dw-at-gnu-pubnames)
  - [`DW_AT_GNU_pubtypes`](#dw-at-gnu-pubtypes)
  - [`DW_AT_GNU_discriminator`](#dw-at-gnu-discriminator)
  - [`DW_AT_GNU_locviews`](#dw-at-gnu-locviews)
  - [`DW_AT_GNU_entry_view`](#dw-at-gnu-entry-view)
  - [`DW_AT_SUN_template`](#dw-at-sun-template)
  - [`DW_AT_SUN_alignment`](#dw-at-sun-alignment)
  - [`DW_AT_SUN_vtable`](#dw-at-sun-vtable)
  - [`DW_AT_SUN_count_guarantee`](#dw-at-sun-count-guarantee)
  - [`DW_AT_SUN_command_line`](#dw-at-sun-command-line)
  - [`DW_AT_SUN_vbase`](#dw-at-sun-vbase)
  - [`DW_AT_SUN_compile_options`](#dw-at-sun-compile-options)
  - [`DW_AT_SUN_language`](#dw-at-sun-language)
  - [`DW_AT_SUN_browser_file`](#dw-at-sun-browser-file)
  - [`DW_AT_SUN_vtable_abi`](#dw-at-sun-vtable-abi)
  - [`DW_AT_SUN_func_offsets`](#dw-at-sun-func-offsets)
  - [`DW_AT_SUN_cf_kind`](#dw-at-sun-cf-kind)
  - [`DW_AT_SUN_vtable_index`](#dw-at-sun-vtable-index)
  - [`DW_AT_SUN_omp_tpriv_addr`](#dw-at-sun-omp-tpriv-addr)
  - [`DW_AT_SUN_omp_child_func`](#dw-at-sun-omp-child-func)
  - [`DW_AT_SUN_func_offset`](#dw-at-sun-func-offset)
  - [`DW_AT_SUN_memop_type_ref`](#dw-at-sun-memop-type-ref)
  - [`DW_AT_SUN_profile_id`](#dw-at-sun-profile-id)
  - [`DW_AT_SUN_memop_signature`](#dw-at-sun-memop-signature)
  - [`DW_AT_SUN_obj_dir`](#dw-at-sun-obj-dir)
  - [`DW_AT_SUN_obj_file`](#dw-at-sun-obj-file)
  - [`DW_AT_SUN_original_name`](#dw-at-sun-original-name)
  - [`DW_AT_SUN_hwcprof_signature`](#dw-at-sun-hwcprof-signature)
  - [`DW_AT_SUN_amd64_parmdump`](#dw-at-sun-amd64-parmdump)
  - [`DW_AT_SUN_part_link_name`](#dw-at-sun-part-link-name)
  - [`DW_AT_SUN_link_name`](#dw-at-sun-link-name)
  - [`DW_AT_SUN_pass_with_const`](#dw-at-sun-pass-with-const)
  - [`DW_AT_SUN_return_with_const`](#dw-at-sun-return-with-const)
  - [`DW_AT_SUN_import_by_name`](#dw-at-sun-import-by-name)
  - [`DW_AT_SUN_f90_pointer`](#dw-at-sun-f90-pointer)
  - [`DW_AT_SUN_pass_by_ref`](#dw-at-sun-pass-by-ref)
  - [`DW_AT_SUN_f90_allocatable`](#dw-at-sun-f90-allocatable)
  - [`DW_AT_SUN_f90_assumed_shape_array`](#dw-at-sun-f90-assumed-shape-array)
  - [`DW_AT_SUN_c_vla`](#dw-at-sun-c-vla)
  - [`DW_AT_SUN_return_value_ptr`](#dw-at-sun-return-value-ptr)
  - [`DW_AT_SUN_dtor_start`](#dw-at-sun-dtor-start)
  - [`DW_AT_SUN_dtor_length`](#dw-at-sun-dtor-length)
  - [`DW_AT_SUN_dtor_state_initial`](#dw-at-sun-dtor-state-initial)
  - [`DW_AT_SUN_dtor_state_final`](#dw-at-sun-dtor-state-final)
  - [`DW_AT_SUN_dtor_state_deltas`](#dw-at-sun-dtor-state-deltas)
  - [`DW_AT_SUN_import_by_lname`](#dw-at-sun-import-by-lname)
  - [`DW_AT_SUN_f90_use_only`](#dw-at-sun-f90-use-only)
  - [`DW_AT_SUN_namelist_spec`](#dw-at-sun-namelist-spec)
  - [`DW_AT_SUN_is_omp_child_func`](#dw-at-sun-is-omp-child-func)
  - [`DW_AT_SUN_fortran_main_alias`](#dw-at-sun-fortran-main-alias)
  - [`DW_AT_SUN_fortran_based`](#dw-at-sun-fortran-based)
  - [`DW_AT_ALTIUM_loclist`](#dw-at-altium-loclist)
  - [`DW_AT_use_GNAT_descriptive_type`](#dw-at-use-gnat-descriptive-type)
  - [`DW_AT_GNAT_descriptive_type`](#dw-at-gnat-descriptive-type)
  - [`DW_AT_GNU_numerator`](#dw-at-gnu-numerator)
  - [`DW_AT_GNU_denominator`](#dw-at-gnu-denominator)
  - [`DW_AT_GNU_bias`](#dw-at-gnu-bias)
  - [`DW_AT_upc_threads_scaled`](#dw-at-upc-threads-scaled)
  - [`DW_AT_PGI_lbase`](#dw-at-pgi-lbase)
  - [`DW_AT_PGI_soffset`](#dw-at-pgi-soffset)
  - [`DW_AT_PGI_lstride`](#dw-at-pgi-lstride)
  - [`DW_AT_BORLAND_property_read`](#dw-at-borland-property-read)
  - [`DW_AT_BORLAND_property_write`](#dw-at-borland-property-write)
  - [`DW_AT_BORLAND_property_implements`](#dw-at-borland-property-implements)
  - [`DW_AT_BORLAND_property_index`](#dw-at-borland-property-index)
  - [`DW_AT_BORLAND_property_default`](#dw-at-borland-property-default)
  - [`DW_AT_BORLAND_Delphi_unit`](#dw-at-borland-delphi-unit)
  - [`DW_AT_BORLAND_Delphi_class`](#dw-at-borland-delphi-class)
  - [`DW_AT_BORLAND_Delphi_record`](#dw-at-borland-delphi-record)
  - [`DW_AT_BORLAND_Delphi_metaclass`](#dw-at-borland-delphi-metaclass)
  - [`DW_AT_BORLAND_Delphi_constructor`](#dw-at-borland-delphi-constructor)
  - [`DW_AT_BORLAND_Delphi_destructor`](#dw-at-borland-delphi-destructor)
  - [`DW_AT_BORLAND_Delphi_anonymous_method`](#dw-at-borland-delphi-anonymous-method)
  - [`DW_AT_BORLAND_Delphi_interface`](#dw-at-borland-delphi-interface)
  - [`DW_AT_BORLAND_Delphi_ABI`](#dw-at-borland-delphi-abi)
  - [`DW_AT_BORLAND_Delphi_return`](#dw-at-borland-delphi-return)
  - [`DW_AT_BORLAND_Delphi_frameptr`](#dw-at-borland-delphi-frameptr)
  - [`DW_AT_BORLAND_closure`](#dw-at-borland-closure)
  - [`DW_AT_LLVM_include_path`](#dw-at-llvm-include-path)
  - [`DW_AT_LLVM_config_macros`](#dw-at-llvm-config-macros)
  - [`DW_AT_LLVM_isysroot`](#dw-at-llvm-isysroot)
  - [`DW_AT_APPLE_optimized`](#dw-at-apple-optimized)
  - [`DW_AT_APPLE_flags`](#dw-at-apple-flags)
  - [`DW_AT_APPLE_isa`](#dw-at-apple-isa)
  - [`DW_AT_APPLE_block`](#dw-at-apple-block)
  - [`DW_AT_APPLE_major_runtime_vers`](#dw-at-apple-major-runtime-vers)
  - [`DW_AT_APPLE_runtime_class`](#dw-at-apple-runtime-class)
  - [`DW_AT_APPLE_omit_frame_ptr`](#dw-at-apple-omit-frame-ptr)
  - [`DW_AT_APPLE_property_name`](#dw-at-apple-property-name)
  - [`DW_AT_APPLE_property_getter`](#dw-at-apple-property-getter)
  - [`DW_AT_APPLE_property_setter`](#dw-at-apple-property-setter)
  - [`DW_AT_APPLE_property_attribute`](#dw-at-apple-property-attribute)
  - [`DW_AT_APPLE_objc_complete_type`](#dw-at-apple-objc-complete-type)
  - [`DW_AT_APPLE_property`](#dw-at-apple-property)
  - [`DW_FORM_null`](#dw-form-null)
  - [`DW_FORM_ref`](#dw-form-ref)
  - [`DW_FORM_addr`](#dw-form-addr)
  - [`DW_FORM_block2`](#dw-form-block2)
  - [`DW_FORM_block4`](#dw-form-block4)
  - [`DW_FORM_data2`](#dw-form-data2)
  - [`DW_FORM_data4`](#dw-form-data4)
  - [`DW_FORM_data8`](#dw-form-data8)
  - [`DW_FORM_string`](#dw-form-string)
  - [`DW_FORM_block`](#dw-form-block)
  - [`DW_FORM_block1`](#dw-form-block1)
  - [`DW_FORM_data1`](#dw-form-data1)
  - [`DW_FORM_flag`](#dw-form-flag)
  - [`DW_FORM_sdata`](#dw-form-sdata)
  - [`DW_FORM_strp`](#dw-form-strp)
  - [`DW_FORM_udata`](#dw-form-udata)
  - [`DW_FORM_ref_addr`](#dw-form-ref-addr)
  - [`DW_FORM_ref1`](#dw-form-ref1)
  - [`DW_FORM_ref2`](#dw-form-ref2)
  - [`DW_FORM_ref4`](#dw-form-ref4)
  - [`DW_FORM_ref8`](#dw-form-ref8)
  - [`DW_FORM_ref_udata`](#dw-form-ref-udata)
  - [`DW_FORM_indirect`](#dw-form-indirect)
  - [`DW_FORM_sec_offset`](#dw-form-sec-offset)
  - [`DW_FORM_exprloc`](#dw-form-exprloc)
  - [`DW_FORM_flag_present`](#dw-form-flag-present)
  - [`DW_FORM_ref_sig8`](#dw-form-ref-sig8)
  - [`DW_FORM_strx`](#dw-form-strx)
  - [`DW_FORM_addrx`](#dw-form-addrx)
  - [`DW_FORM_ref_sup4`](#dw-form-ref-sup4)
  - [`DW_FORM_strp_sup`](#dw-form-strp-sup)
  - [`DW_FORM_data16`](#dw-form-data16)
  - [`DW_FORM_line_strp`](#dw-form-line-strp)
  - [`DW_FORM_implicit_const`](#dw-form-implicit-const)
  - [`DW_FORM_loclistx`](#dw-form-loclistx)
  - [`DW_FORM_rnglistx`](#dw-form-rnglistx)
  - [`DW_FORM_ref_sup8`](#dw-form-ref-sup8)
  - [`DW_FORM_strx1`](#dw-form-strx1)
  - [`DW_FORM_strx2`](#dw-form-strx2)
  - [`DW_FORM_strx3`](#dw-form-strx3)
  - [`DW_FORM_strx4`](#dw-form-strx4)
  - [`DW_FORM_addrx1`](#dw-form-addrx1)
  - [`DW_FORM_addrx2`](#dw-form-addrx2)
  - [`DW_FORM_addrx3`](#dw-form-addrx3)
  - [`DW_FORM_addrx4`](#dw-form-addrx4)
  - [`DW_FORM_GNU_addr_index`](#dw-form-gnu-addr-index)
  - [`DW_FORM_GNU_str_index`](#dw-form-gnu-str-index)
  - [`DW_FORM_GNU_ref_alt`](#dw-form-gnu-ref-alt)
  - [`DW_FORM_GNU_strp_alt`](#dw-form-gnu-strp-alt)
  - [`DW_ATE_address`](#dw-ate-address)
  - [`DW_ATE_boolean`](#dw-ate-boolean)
  - [`DW_ATE_complex_float`](#dw-ate-complex-float)
  - [`DW_ATE_float`](#dw-ate-float)
  - [`DW_ATE_signed`](#dw-ate-signed)
  - [`DW_ATE_signed_char`](#dw-ate-signed-char)
  - [`DW_ATE_unsigned`](#dw-ate-unsigned)
  - [`DW_ATE_unsigned_char`](#dw-ate-unsigned-char)
  - [`DW_ATE_imaginary_float`](#dw-ate-imaginary-float)
  - [`DW_ATE_packed_decimal`](#dw-ate-packed-decimal)
  - [`DW_ATE_numeric_string`](#dw-ate-numeric-string)
  - [`DW_ATE_edited`](#dw-ate-edited)
  - [`DW_ATE_signed_fixed`](#dw-ate-signed-fixed)
  - [`DW_ATE_unsigned_fixed`](#dw-ate-unsigned-fixed)
  - [`DW_ATE_decimal_float`](#dw-ate-decimal-float)
  - [`DW_ATE_UTF`](#dw-ate-utf)
  - [`DW_ATE_UCS`](#dw-ate-ucs)
  - [`DW_ATE_ASCII`](#dw-ate-ascii)
  - [`DW_ATE_lo_user`](#dw-ate-lo-user)
  - [`DW_ATE_hi_user`](#dw-ate-hi-user)
  - [`DW_LLE_end_of_list`](#dw-lle-end-of-list)
  - [`DW_LLE_base_addressx`](#dw-lle-base-addressx)
  - [`DW_LLE_startx_endx`](#dw-lle-startx-endx)
  - [`DW_LLE_startx_length`](#dw-lle-startx-length)
  - [`DW_LLE_offset_pair`](#dw-lle-offset-pair)
  - [`DW_LLE_default_location`](#dw-lle-default-location)
  - [`DW_LLE_base_address`](#dw-lle-base-address)
  - [`DW_LLE_start_end`](#dw-lle-start-end)
  - [`DW_LLE_start_length`](#dw-lle-start-length)
  - [`DW_LLE_GNU_view_pair`](#dw-lle-gnu-view-pair)
  - [`DW_DS_unsigned`](#dw-ds-unsigned)
  - [`DW_DS_leading_overpunch`](#dw-ds-leading-overpunch)
  - [`DW_DS_trailing_overpunch`](#dw-ds-trailing-overpunch)
  - [`DW_DS_leading_separate`](#dw-ds-leading-separate)
  - [`DW_DS_trailing_separate`](#dw-ds-trailing-separate)
  - [`DW_END_default`](#dw-end-default)
  - [`DW_END_big`](#dw-end-big)
  - [`DW_END_little`](#dw-end-little)
  - [`DW_END_lo_user`](#dw-end-lo-user)
  - [`DW_END_hi_user`](#dw-end-hi-user)
  - [`DW_ACCESS_public`](#dw-access-public)
  - [`DW_ACCESS_protected`](#dw-access-protected)
  - [`DW_ACCESS_private`](#dw-access-private)
  - [`DW_VIS_local`](#dw-vis-local)
  - [`DW_VIS_exported`](#dw-vis-exported)
  - [`DW_VIS_qualified`](#dw-vis-qualified)
  - [`DW_VIRTUALITY_none`](#dw-virtuality-none)
  - [`DW_VIRTUALITY_virtual`](#dw-virtuality-virtual)
  - [`DW_VIRTUALITY_pure_virtual`](#dw-virtuality-pure-virtual)
  - [`DW_LANG_C89`](#dw-lang-c89)
  - [`DW_LANG_C`](#dw-lang-c)
  - [`DW_LANG_Ada83`](#dw-lang-ada83)
  - [`DW_LANG_C_plus_plus`](#dw-lang-c-plus-plus)
  - [`DW_LANG_Cobol74`](#dw-lang-cobol74)
  - [`DW_LANG_Cobol85`](#dw-lang-cobol85)
  - [`DW_LANG_Fortran77`](#dw-lang-fortran77)
  - [`DW_LANG_Fortran90`](#dw-lang-fortran90)
  - [`DW_LANG_Pascal83`](#dw-lang-pascal83)
  - [`DW_LANG_Modula2`](#dw-lang-modula2)
  - [`DW_LANG_Java`](#dw-lang-java)
  - [`DW_LANG_C99`](#dw-lang-c99)
  - [`DW_LANG_Ada95`](#dw-lang-ada95)
  - [`DW_LANG_Fortran95`](#dw-lang-fortran95)
  - [`DW_LANG_PLI`](#dw-lang-pli)
  - [`DW_LANG_ObjC`](#dw-lang-objc)
  - [`DW_LANG_ObjC_plus_plus`](#dw-lang-objc-plus-plus)
  - [`DW_LANG_UPC`](#dw-lang-upc)
  - [`DW_LANG_D`](#dw-lang-d)
  - [`DW_LANG_Python`](#dw-lang-python)
  - [`DW_LANG_OpenCL`](#dw-lang-opencl)
  - [`DW_LANG_Go`](#dw-lang-go)
  - [`DW_LANG_Modula3`](#dw-lang-modula3)
  - [`DW_LANG_Haskell`](#dw-lang-haskell)
  - [`DW_LANG_C_plus_plus_03`](#dw-lang-c-plus-plus-03)
  - [`DW_LANG_C_plus_plus_11`](#dw-lang-c-plus-plus-11)
  - [`DW_LANG_OCaml`](#dw-lang-ocaml)
  - [`DW_LANG_Rust`](#dw-lang-rust)
  - [`DW_LANG_C11`](#dw-lang-c11)
  - [`DW_LANG_Swift`](#dw-lang-swift)
  - [`DW_LANG_Julia`](#dw-lang-julia)
  - [`DW_LANG_Dylan`](#dw-lang-dylan)
  - [`DW_LANG_C_plus_plus_14`](#dw-lang-c-plus-plus-14)
  - [`DW_LANG_Fortran03`](#dw-lang-fortran03)
  - [`DW_LANG_Fortran08`](#dw-lang-fortran08)
  - [`DW_LANG_RenderScript`](#dw-lang-renderscript)
  - [`DW_LANG_BLISS`](#dw-lang-bliss)
  - [`DW_LANG_Kotlin`](#dw-lang-kotlin)
  - [`DW_LANG_Zig`](#dw-lang-zig)
  - [`DW_LANG_Crystal`](#dw-lang-crystal)
  - [`DW_LANG_C_plus_plus_17`](#dw-lang-c-plus-plus-17)
  - [`DW_LANG_C_plus_plus_20`](#dw-lang-c-plus-plus-20)
  - [`DW_LANG_C17`](#dw-lang-c17)
  - [`DW_LANG_Fortran18`](#dw-lang-fortran18)
  - [`DW_LANG_Ada2005`](#dw-lang-ada2005)
  - [`DW_LANG_Ada2012`](#dw-lang-ada2012)
  - [`DW_LANG_lo_user`](#dw-lang-lo-user)
  - [`DW_LANG_hi_user`](#dw-lang-hi-user)
  - [`DW_LANG_Mips_Assembler`](#dw-lang-mips-assembler)
  - [`DW_LANG_GOOGLE_RenderScript`](#dw-lang-google-renderscript)
  - [`DW_LANG_SUN_Assembler`](#dw-lang-sun-assembler)
  - [`DW_LANG_ALTIUM_Assembler`](#dw-lang-altium-assembler)
  - [`DW_LANG_BORLAND_Delphi`](#dw-lang-borland-delphi)
  - [`DW_ADDR_none`](#dw-addr-none)
  - [`DW_ID_case_sensitive`](#dw-id-case-sensitive)
  - [`DW_ID_up_case`](#dw-id-up-case)
  - [`DW_ID_down_case`](#dw-id-down-case)
  - [`DW_ID_case_insensitive`](#dw-id-case-insensitive)
  - [`DW_CC_normal`](#dw-cc-normal)
  - [`DW_CC_program`](#dw-cc-program)
  - [`DW_CC_nocall`](#dw-cc-nocall)
  - [`DW_CC_pass_by_reference`](#dw-cc-pass-by-reference)
  - [`DW_CC_pass_by_value`](#dw-cc-pass-by-value)
  - [`DW_CC_lo_user`](#dw-cc-lo-user)
  - [`DW_CC_hi_user`](#dw-cc-hi-user)
  - [`DW_INL_not_inlined`](#dw-inl-not-inlined)
  - [`DW_INL_inlined`](#dw-inl-inlined)
  - [`DW_INL_declared_not_inlined`](#dw-inl-declared-not-inlined)
  - [`DW_INL_declared_inlined`](#dw-inl-declared-inlined)
  - [`DW_ORD_row_major`](#dw-ord-row-major)
  - [`DW_ORD_col_major`](#dw-ord-col-major)
  - [`DW_DSC_label`](#dw-dsc-label)
  - [`DW_DSC_range`](#dw-dsc-range)
  - [`DW_IDX_compile_unit`](#dw-idx-compile-unit)
  - [`DW_IDX_type_unit`](#dw-idx-type-unit)
  - [`DW_IDX_die_offset`](#dw-idx-die-offset)
  - [`DW_IDX_parent`](#dw-idx-parent)
  - [`DW_IDX_type_hash`](#dw-idx-type-hash)
  - [`DW_IDX_lo_user`](#dw-idx-lo-user)
  - [`DW_IDX_hi_user`](#dw-idx-hi-user)
  - [`DW_DEFAULTED_no`](#dw-defaulted-no)
  - [`DW_DEFAULTED_in_class`](#dw-defaulted-in-class)
  - [`DW_DEFAULTED_out_of_class`](#dw-defaulted-out-of-class)
  - [`DW_LNS_copy`](#dw-lns-copy)
  - [`DW_LNS_advance_pc`](#dw-lns-advance-pc)
  - [`DW_LNS_advance_line`](#dw-lns-advance-line)
  - [`DW_LNS_set_file`](#dw-lns-set-file)
  - [`DW_LNS_set_column`](#dw-lns-set-column)
  - [`DW_LNS_negate_stmt`](#dw-lns-negate-stmt)
  - [`DW_LNS_set_basic_block`](#dw-lns-set-basic-block)
  - [`DW_LNS_const_add_pc`](#dw-lns-const-add-pc)
  - [`DW_LNS_fixed_advance_pc`](#dw-lns-fixed-advance-pc)
  - [`DW_LNS_set_prologue_end`](#dw-lns-set-prologue-end)
  - [`DW_LNS_set_epilogue_begin`](#dw-lns-set-epilogue-begin)
  - [`DW_LNS_set_isa`](#dw-lns-set-isa)
  - [`DW_LNE_end_sequence`](#dw-lne-end-sequence)
  - [`DW_LNE_set_address`](#dw-lne-set-address)
  - [`DW_LNE_define_file`](#dw-lne-define-file)
  - [`DW_LNE_set_discriminator`](#dw-lne-set-discriminator)
  - [`DW_LNE_lo_user`](#dw-lne-lo-user)
  - [`DW_LNE_hi_user`](#dw-lne-hi-user)
  - [`DW_LNCT_path`](#dw-lnct-path)
  - [`DW_LNCT_directory_index`](#dw-lnct-directory-index)
  - [`DW_LNCT_timestamp`](#dw-lnct-timestamp)
  - [`DW_LNCT_size`](#dw-lnct-size)
  - [`DW_LNCT_MD5`](#dw-lnct-md5)
  - [`DW_LNCT_lo_user`](#dw-lnct-lo-user)
  - [`DW_LNCT_LLVM_source`](#dw-lnct-llvm-source)
  - [`DW_LNCT_hi_user`](#dw-lnct-hi-user)
  - [`DW_MACINFO_define`](#dw-macinfo-define)
  - [`DW_MACINFO_undef`](#dw-macinfo-undef)
  - [`DW_MACINFO_start_file`](#dw-macinfo-start-file)
  - [`DW_MACINFO_end_file`](#dw-macinfo-end-file)
  - [`DW_MACINFO_vendor_ext`](#dw-macinfo-vendor-ext)
  - [`DW_MACRO_define`](#dw-macro-define)
  - [`DW_MACRO_undef`](#dw-macro-undef)
  - [`DW_MACRO_start_file`](#dw-macro-start-file)
  - [`DW_MACRO_end_file`](#dw-macro-end-file)
  - [`DW_MACRO_define_strp`](#dw-macro-define-strp)
  - [`DW_MACRO_undef_strp`](#dw-macro-undef-strp)
  - [`DW_MACRO_import`](#dw-macro-import)
  - [`DW_MACRO_define_sup`](#dw-macro-define-sup)
  - [`DW_MACRO_undef_sup`](#dw-macro-undef-sup)
  - [`DW_MACRO_import_sup`](#dw-macro-import-sup)
  - [`DW_MACRO_define_strx`](#dw-macro-define-strx)
  - [`DW_MACRO_undef_strx`](#dw-macro-undef-strx)
  - [`DW_MACRO_lo_user`](#dw-macro-lo-user)
  - [`DW_MACRO_hi_user`](#dw-macro-hi-user)
  - [`DW_RLE_end_of_list`](#dw-rle-end-of-list)
  - [`DW_RLE_base_addressx`](#dw-rle-base-addressx)
  - [`DW_RLE_startx_endx`](#dw-rle-startx-endx)
  - [`DW_RLE_startx_length`](#dw-rle-startx-length)
  - [`DW_RLE_offset_pair`](#dw-rle-offset-pair)
  - [`DW_RLE_base_address`](#dw-rle-base-address)
  - [`DW_RLE_start_end`](#dw-rle-start-end)
  - [`DW_RLE_start_length`](#dw-rle-start-length)
  - [`DW_OP_addr`](#dw-op-addr)
  - [`DW_OP_deref`](#dw-op-deref)
  - [`DW_OP_const1u`](#dw-op-const1u)
  - [`DW_OP_const1s`](#dw-op-const1s)
  - [`DW_OP_const2u`](#dw-op-const2u)
  - [`DW_OP_const2s`](#dw-op-const2s)
  - [`DW_OP_const4u`](#dw-op-const4u)
  - [`DW_OP_const4s`](#dw-op-const4s)
  - [`DW_OP_const8u`](#dw-op-const8u)
  - [`DW_OP_const8s`](#dw-op-const8s)
  - [`DW_OP_constu`](#dw-op-constu)
  - [`DW_OP_consts`](#dw-op-consts)
  - [`DW_OP_dup`](#dw-op-dup)
  - [`DW_OP_drop`](#dw-op-drop)
  - [`DW_OP_over`](#dw-op-over)
  - [`DW_OP_pick`](#dw-op-pick)
  - [`DW_OP_swap`](#dw-op-swap)
  - [`DW_OP_rot`](#dw-op-rot)
  - [`DW_OP_xderef`](#dw-op-xderef)
  - [`DW_OP_abs`](#dw-op-abs)
  - [`DW_OP_and`](#dw-op-and)
  - [`DW_OP_div`](#dw-op-div)
  - [`DW_OP_minus`](#dw-op-minus)
  - [`DW_OP_mod`](#dw-op-mod)
  - [`DW_OP_mul`](#dw-op-mul)
  - [`DW_OP_neg`](#dw-op-neg)
  - [`DW_OP_not`](#dw-op-not)
  - [`DW_OP_or`](#dw-op-or)
  - [`DW_OP_plus`](#dw-op-plus)
  - [`DW_OP_plus_uconst`](#dw-op-plus-uconst)
  - [`DW_OP_shl`](#dw-op-shl)
  - [`DW_OP_shr`](#dw-op-shr)
  - [`DW_OP_shra`](#dw-op-shra)
  - [`DW_OP_xor`](#dw-op-xor)
  - [`DW_OP_bra`](#dw-op-bra)
  - [`DW_OP_eq`](#dw-op-eq)
  - [`DW_OP_ge`](#dw-op-ge)
  - [`DW_OP_gt`](#dw-op-gt)
  - [`DW_OP_le`](#dw-op-le)
  - [`DW_OP_lt`](#dw-op-lt)
  - [`DW_OP_ne`](#dw-op-ne)
  - [`DW_OP_skip`](#dw-op-skip)
  - [`DW_OP_lit0`](#dw-op-lit0)
  - [`DW_OP_lit1`](#dw-op-lit1)
  - [`DW_OP_lit2`](#dw-op-lit2)
  - [`DW_OP_lit3`](#dw-op-lit3)
  - [`DW_OP_lit4`](#dw-op-lit4)
  - [`DW_OP_lit5`](#dw-op-lit5)
  - [`DW_OP_lit6`](#dw-op-lit6)
  - [`DW_OP_lit7`](#dw-op-lit7)
  - [`DW_OP_lit8`](#dw-op-lit8)
  - [`DW_OP_lit9`](#dw-op-lit9)
  - [`DW_OP_lit10`](#dw-op-lit10)
  - [`DW_OP_lit11`](#dw-op-lit11)
  - [`DW_OP_lit12`](#dw-op-lit12)
  - [`DW_OP_lit13`](#dw-op-lit13)
  - [`DW_OP_lit14`](#dw-op-lit14)
  - [`DW_OP_lit15`](#dw-op-lit15)
  - [`DW_OP_lit16`](#dw-op-lit16)
  - [`DW_OP_lit17`](#dw-op-lit17)
  - [`DW_OP_lit18`](#dw-op-lit18)
  - [`DW_OP_lit19`](#dw-op-lit19)
  - [`DW_OP_lit20`](#dw-op-lit20)
  - [`DW_OP_lit21`](#dw-op-lit21)
  - [`DW_OP_lit22`](#dw-op-lit22)
  - [`DW_OP_lit23`](#dw-op-lit23)
  - [`DW_OP_lit24`](#dw-op-lit24)
  - [`DW_OP_lit25`](#dw-op-lit25)
  - [`DW_OP_lit26`](#dw-op-lit26)
  - [`DW_OP_lit27`](#dw-op-lit27)
  - [`DW_OP_lit28`](#dw-op-lit28)
  - [`DW_OP_lit29`](#dw-op-lit29)
  - [`DW_OP_lit30`](#dw-op-lit30)
  - [`DW_OP_lit31`](#dw-op-lit31)
  - [`DW_OP_reg0`](#dw-op-reg0)
  - [`DW_OP_reg1`](#dw-op-reg1)
  - [`DW_OP_reg2`](#dw-op-reg2)
  - [`DW_OP_reg3`](#dw-op-reg3)
  - [`DW_OP_reg4`](#dw-op-reg4)
  - [`DW_OP_reg5`](#dw-op-reg5)
  - [`DW_OP_reg6`](#dw-op-reg6)
  - [`DW_OP_reg7`](#dw-op-reg7)
  - [`DW_OP_reg8`](#dw-op-reg8)
  - [`DW_OP_reg9`](#dw-op-reg9)
  - [`DW_OP_reg10`](#dw-op-reg10)
  - [`DW_OP_reg11`](#dw-op-reg11)
  - [`DW_OP_reg12`](#dw-op-reg12)
  - [`DW_OP_reg13`](#dw-op-reg13)
  - [`DW_OP_reg14`](#dw-op-reg14)
  - [`DW_OP_reg15`](#dw-op-reg15)
  - [`DW_OP_reg16`](#dw-op-reg16)
  - [`DW_OP_reg17`](#dw-op-reg17)
  - [`DW_OP_reg18`](#dw-op-reg18)
  - [`DW_OP_reg19`](#dw-op-reg19)
  - [`DW_OP_reg20`](#dw-op-reg20)
  - [`DW_OP_reg21`](#dw-op-reg21)
  - [`DW_OP_reg22`](#dw-op-reg22)
  - [`DW_OP_reg23`](#dw-op-reg23)
  - [`DW_OP_reg24`](#dw-op-reg24)
  - [`DW_OP_reg25`](#dw-op-reg25)
  - [`DW_OP_reg26`](#dw-op-reg26)
  - [`DW_OP_reg27`](#dw-op-reg27)
  - [`DW_OP_reg28`](#dw-op-reg28)
  - [`DW_OP_reg29`](#dw-op-reg29)
  - [`DW_OP_reg30`](#dw-op-reg30)
  - [`DW_OP_reg31`](#dw-op-reg31)
  - [`DW_OP_breg0`](#dw-op-breg0)
  - [`DW_OP_breg1`](#dw-op-breg1)
  - [`DW_OP_breg2`](#dw-op-breg2)
  - [`DW_OP_breg3`](#dw-op-breg3)
  - [`DW_OP_breg4`](#dw-op-breg4)
  - [`DW_OP_breg5`](#dw-op-breg5)
  - [`DW_OP_breg6`](#dw-op-breg6)
  - [`DW_OP_breg7`](#dw-op-breg7)
  - [`DW_OP_breg8`](#dw-op-breg8)
  - [`DW_OP_breg9`](#dw-op-breg9)
  - [`DW_OP_breg10`](#dw-op-breg10)
  - [`DW_OP_breg11`](#dw-op-breg11)
  - [`DW_OP_breg12`](#dw-op-breg12)
  - [`DW_OP_breg13`](#dw-op-breg13)
  - [`DW_OP_breg14`](#dw-op-breg14)
  - [`DW_OP_breg15`](#dw-op-breg15)
  - [`DW_OP_breg16`](#dw-op-breg16)
  - [`DW_OP_breg17`](#dw-op-breg17)
  - [`DW_OP_breg18`](#dw-op-breg18)
  - [`DW_OP_breg19`](#dw-op-breg19)
  - [`DW_OP_breg20`](#dw-op-breg20)
  - [`DW_OP_breg21`](#dw-op-breg21)
  - [`DW_OP_breg22`](#dw-op-breg22)
  - [`DW_OP_breg23`](#dw-op-breg23)
  - [`DW_OP_breg24`](#dw-op-breg24)
  - [`DW_OP_breg25`](#dw-op-breg25)
  - [`DW_OP_breg26`](#dw-op-breg26)
  - [`DW_OP_breg27`](#dw-op-breg27)
  - [`DW_OP_breg28`](#dw-op-breg28)
  - [`DW_OP_breg29`](#dw-op-breg29)
  - [`DW_OP_breg30`](#dw-op-breg30)
  - [`DW_OP_breg31`](#dw-op-breg31)
  - [`DW_OP_regx`](#dw-op-regx)
  - [`DW_OP_fbreg`](#dw-op-fbreg)
  - [`DW_OP_bregx`](#dw-op-bregx)
  - [`DW_OP_piece`](#dw-op-piece)
  - [`DW_OP_deref_size`](#dw-op-deref-size)
  - [`DW_OP_xderef_size`](#dw-op-xderef-size)
  - [`DW_OP_nop`](#dw-op-nop)
  - [`DW_OP_push_object_address`](#dw-op-push-object-address)
  - [`DW_OP_call2`](#dw-op-call2)
  - [`DW_OP_call4`](#dw-op-call4)
  - [`DW_OP_call_ref`](#dw-op-call-ref)
  - [`DW_OP_form_tls_address`](#dw-op-form-tls-address)
  - [`DW_OP_call_frame_cfa`](#dw-op-call-frame-cfa)
  - [`DW_OP_bit_piece`](#dw-op-bit-piece)
  - [`DW_OP_implicit_value`](#dw-op-implicit-value)
  - [`DW_OP_stack_value`](#dw-op-stack-value)
  - [`DW_OP_implicit_pointer`](#dw-op-implicit-pointer)
  - [`DW_OP_addrx`](#dw-op-addrx)
  - [`DW_OP_constx`](#dw-op-constx)
  - [`DW_OP_entry_value`](#dw-op-entry-value)
  - [`DW_OP_const_type`](#dw-op-const-type)
  - [`DW_OP_regval_type`](#dw-op-regval-type)
  - [`DW_OP_deref_type`](#dw-op-deref-type)
  - [`DW_OP_xderef_type`](#dw-op-xderef-type)
  - [`DW_OP_convert`](#dw-op-convert)
  - [`DW_OP_reinterpret`](#dw-op-reinterpret)
  - [`DW_OP_GNU_push_tls_address`](#dw-op-gnu-push-tls-address)
  - [`DW_OP_GNU_uninit`](#dw-op-gnu-uninit)
  - [`DW_OP_GNU_encoded_addr`](#dw-op-gnu-encoded-addr)
  - [`DW_OP_GNU_implicit_pointer`](#dw-op-gnu-implicit-pointer)
  - [`DW_OP_GNU_entry_value`](#dw-op-gnu-entry-value)
  - [`DW_OP_GNU_const_type`](#dw-op-gnu-const-type)
  - [`DW_OP_GNU_regval_type`](#dw-op-gnu-regval-type)
  - [`DW_OP_GNU_deref_type`](#dw-op-gnu-deref-type)
  - [`DW_OP_GNU_convert`](#dw-op-gnu-convert)
  - [`DW_OP_GNU_reinterpret`](#dw-op-gnu-reinterpret)
  - [`DW_OP_GNU_parameter_ref`](#dw-op-gnu-parameter-ref)
  - [`DW_OP_GNU_addr_index`](#dw-op-gnu-addr-index)
  - [`DW_OP_GNU_const_index`](#dw-op-gnu-const-index)
  - [`DW_OP_GNU_variable_value`](#dw-op-gnu-variable-value)
  - [`DW_OP_WASM_location`](#dw-op-wasm-location)
  - [`DW_EH_PE_uleb128`](#dw-eh-pe-uleb128)
  - [`DW_EH_PE_udata2`](#dw-eh-pe-udata2)
  - [`DW_EH_PE_udata4`](#dw-eh-pe-udata4)
  - [`DW_EH_PE_udata8`](#dw-eh-pe-udata8)
  - [`DW_EH_PE_sleb128`](#dw-eh-pe-sleb128)
  - [`DW_EH_PE_sdata2`](#dw-eh-pe-sdata2)
  - [`DW_EH_PE_sdata4`](#dw-eh-pe-sdata4)
  - [`DW_EH_PE_sdata8`](#dw-eh-pe-sdata8)
  - [`DW_EH_PE_pcrel`](#dw-eh-pe-pcrel)
  - [`DW_EH_PE_textrel`](#dw-eh-pe-textrel)
  - [`DW_EH_PE_datarel`](#dw-eh-pe-datarel)
  - [`DW_EH_PE_funcrel`](#dw-eh-pe-funcrel)
  - [`DW_EH_PE_aligned`](#dw-eh-pe-aligned)
  - [`DW_EH_PE_indirect`](#dw-eh-pe-indirect)
  - [`DW_EH_PE_absptr`](#dw-eh-pe-absptr)
  - [`DW_EH_PE_omit`](#dw-eh-pe-omit)
  - [`DW_EH_PE_FORMAT_MASK`](#dw-eh-pe-format-mask)
  - [`DW_EH_PE_APPLICATION_MASK`](#dw-eh-pe-application-mask)
- [Macros](#macros)
  - [`registers!`](#registers)
  - [`dw!`](#dw)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`common`](#common) | mod |  |
| [`arch`](#arch) | mod |  |
| [`constants`](#constants) | mod | Constant definitions. |
| [`endianity`](#endianity) | mod | Types for compile-time and run-time endianity. |
| [`leb128`](#leb128) | mod | Read and write DWARF's "Little Endian Base 128" (LEB128) variable length integer encoding. |
| [`case_fold`](#case-fold) | mod |  |
| [`read`](#read) | mod | Read DWARF debugging information. |
| [`util`](#util) | mod |  |
| [`addr`](#addr) | mod |  |
| [`cfi`](#cfi) | mod |  |
| [`dwarf`](#dwarf) | mod |  |
| [`endian_slice`](#endian-slice) | mod | Working with byte slices that have an associated endianity. |
| [`endian_reader`](#endian-reader) | mod | Defining custom `Reader`s quickly. |
| [`reader`](#reader) | mod |  |
| [`relocate`](#relocate) | mod |  |
| [`abbrev`](#abbrev) | mod | Functions for parsing DWARF debugging abbreviations. |
| [`aranges`](#aranges) | mod |  |
| [`index`](#index) | mod |  |
| [`line`](#line) | mod |  |
| [`lists`](#lists) | mod |  |
| [`loclists`](#loclists) | mod |  |
| [`lookup`](#lookup) | mod |  |
| [`macros`](#macros) | mod |  |
| [`names`](#names) | mod | Functions for parsing DWARF 5 `.debug_names` sections. |
| [`op`](#op) | mod | Functions for parsing and evaluating DWARF expressions. |
| [`pubnames`](#pubnames) | mod |  |
| [`pubtypes`](#pubtypes) | mod |  |
| [`rnglists`](#rnglists) | mod |  |
| [`str`](#str) | mod |  |
| [`unit`](#unit) | mod | Functions for parsing DWARF `.debug_info` and `.debug_types` sections. |
| [`value`](#value) | mod | Definitions for values used in DWARF expressions. |
| [`Encoding`](#encoding) | struct | Encoding parameters that are commonly used for multiple DWARF sections. |
| [`LineEncoding`](#lineencoding) | struct | Encoding parameters for a line number program. |
| [`Register`](#register) | struct | A DWARF register number. |
| [`DebugAbbrevOffset`](#debugabbrevoffset) | struct | An offset into the `.debug_abbrev` section. |
| [`DebugAddrOffset`](#debugaddroffset) | struct | An offset into the `.debug_addr` section. |
| [`DebugAddrBase`](#debugaddrbase) | struct | An offset to a set of entries in the `.debug_addr` section. |
| [`DebugAddrIndex`](#debugaddrindex) | struct | An index into a set of addresses in the `.debug_addr` section. |
| [`DebugArangesOffset`](#debugarangesoffset) | struct | An offset into the `.debug_aranges` section. |
| [`DebugInfoOffset`](#debuginfooffset) | struct | An offset into the `.debug_info` section. |
| [`DebugLineOffset`](#debuglineoffset) | struct | An offset into the `.debug_line` section. |
| [`DebugLineStrOffset`](#debuglinestroffset) | struct | An offset into the `.debug_line_str` section. |
| [`LocationListsOffset`](#locationlistsoffset) | struct | An offset into either the `.debug_loc` section or the `.debug_loclists` section, depending on the version of the unit the offset was contained in. |
| [`DebugLocListsBase`](#debugloclistsbase) | struct | An offset to a set of location list offsets in the `.debug_loclists` section. |
| [`DebugLocListsIndex`](#debugloclistsindex) | struct | An index into a set of location list offsets in the `.debug_loclists` section. |
| [`DebugMacinfoOffset`](#debugmacinfooffset) | struct | An offset into the `.debug_macinfo` section. |
| [`DebugMacroOffset`](#debugmacrooffset) | struct | An offset into the `.debug_macro` section. |
| [`DebugNamesOffset`](#debugnamesoffset) | struct | An offset into the `.debug_names` section. |
| [`RawRangeListsOffset`](#rawrangelistsoffset) | struct | An offset into either the `.debug_ranges` section or the `.debug_rnglists` section, depending on the version of the unit the offset was contained in. |
| [`RangeListsOffset`](#rangelistsoffset) | struct | An offset into either the `.debug_ranges` section or the `.debug_rnglists` section, depending on the version of the unit the offset was contained in. |
| [`DebugRngListsBase`](#debugrnglistsbase) | struct | An offset to a set of range list offsets in the `.debug_rnglists` section. |
| [`DebugRngListsIndex`](#debugrnglistsindex) | struct | An index into a set of range list offsets in the `.debug_rnglists` section. |
| [`DebugStrOffset`](#debugstroffset) | struct | An offset into the `.debug_str` section. |
| [`DebugStrOffsetsBase`](#debugstroffsetsbase) | struct | An offset to a set of entries in the `.debug_str_offsets` section. |
| [`DebugStrOffsetsIndex`](#debugstroffsetsindex) | struct | An index into a set of entries in the `.debug_str_offsets` section. |
| [`DebugTypesOffset`](#debugtypesoffset) | struct | An offset into the `.debug_types` section. |
| [`DebugTypeSignature`](#debugtypesignature) | struct | A type signature as used in the `.debug_types` section. |
| [`DebugFrameOffset`](#debugframeoffset) | struct | An offset into the `.debug_frame` section. |
| [`EhFrameOffset`](#ehframeoffset) | struct | An offset into the `.eh_frame` section. |
| [`UnitSectionOffset`](#unitsectionoffset) | struct | An offset into the `.debug_info` or `.debug_types` sections. |
| [`DwoId`](#dwoid) | struct | An optionally-provided implementation-defined compilation unit ID to enable split DWARF and linking a split compilation unit back together. |
| [`Arm`](#arm) | struct | ARM architecture specific definitions. |
| [`AArch64`](#aarch64) | struct | ARM 64-bit (AArch64) architecture specific definitions. |
| [`LoongArch`](#loongarch) | struct | LoongArch architecture specific definitions. |
| [`MIPS`](#mips) | struct | MIPS architecture specific definitions. |
| [`RiscV`](#riscv) | struct | RISC-V architecture specific definitions. |
| [`X86`](#x86) | struct | Intel i386 architecture specific definitions. |
| [`X86_64`](#x86-64) | struct | AMD64 architecture specific definitions. |
| [`PowerPc64`](#powerpc64) | struct | PowerPC 64bit |
| [`DwSect`](#dwsect) | struct | The section type field in a `.dwp` unit index. |
| [`DwSectV2`](#dwsectv2) | struct | The section type field in a `.dwp` unit index with version 2. |
| [`DwUt`](#dwut) | struct | The unit type field in a unit header. |
| [`DwCfa`](#dwcfa) | struct | The opcode for a call frame instruction. |
| [`DwChildren`](#dwchildren) | struct | The child determination encodings for DIE attributes. |
| [`DwTag`](#dwtag) | struct | The tag encodings for DIE attributes. |
| [`DwAt`](#dwat) | struct | The attribute encodings for DIE attributes. |
| [`DwForm`](#dwform) | struct | The attribute form encodings for DIE attributes. |
| [`DwAte`](#dwate) | struct | The encodings of the constants used in the `DW_AT_encoding` attribute. |
| [`DwLle`](#dwlle) | struct | The encodings of the constants used in location list entries. |
| [`DwDs`](#dwds) | struct | The encodings of the constants used in the `DW_AT_decimal_sign` attribute. |
| [`DwEnd`](#dwend) | struct | The encodings of the constants used in the `DW_AT_endianity` attribute. |
| [`DwAccess`](#dwaccess) | struct | The encodings of the constants used in the `DW_AT_accessibility` attribute. |
| [`DwVis`](#dwvis) | struct | The encodings of the constants used in the `DW_AT_visibility` attribute. |
| [`DwVirtuality`](#dwvirtuality) | struct | The encodings of the constants used in the `DW_AT_virtuality` attribute. |
| [`DwLang`](#dwlang) | struct | The encodings of the constants used in the `DW_AT_language` attribute. |
| [`DwAddr`](#dwaddr) | struct | The encodings of the constants used in the `DW_AT_address_class` attribute. |
| [`DwId`](#dwid) | struct | The encodings of the constants used in the `DW_AT_identifier_case` attribute. |
| [`DwCc`](#dwcc) | struct | The encodings of the constants used in the `DW_AT_calling_convention` attribute. |
| [`DwInl`](#dwinl) | struct | The encodings of the constants used in the `DW_AT_inline` attribute. |
| [`DwOrd`](#dword) | struct | The encodings of the constants used in the `DW_AT_ordering` attribute. |
| [`DwDsc`](#dwdsc) | struct | The encodings of the constants used in the `DW_AT_discr_list` attribute. |
| [`DwIdx`](#dwidx) | struct | Name index attribute encodings. |
| [`DwDefaulted`](#dwdefaulted) | struct | The encodings of the constants used in the `DW_AT_defaulted` attribute. |
| [`DwLns`](#dwlns) | struct | The encodings for the standard opcodes for line number information. |
| [`DwLne`](#dwlne) | struct | The encodings for the extended opcodes for line number information. |
| [`DwLnct`](#dwlnct) | struct | The encodings for the line number header entry formats. |
| [`DwMacinfo`](#dwmacinfo) | struct | Type codes for macro definitions in the `.debug_macinfo` section. |
| [`DwMacro`](#dwmacro) | struct | The encodings for macro information entry types. |
| [`DwRle`](#dwrle) | struct | Range list entry encoding values. |
| [`DwOp`](#dwop) | struct | The encodings for DWARF expression operations. |
| [`DwEhPe`](#dwehpe) | struct | Pointer encoding used by `.eh_frame`. |
| [`LittleEndian`](#littleendian) | struct | Little endian byte order. |
| [`BigEndian`](#bigendian) | struct | Big endian byte order. |
| [`UnitOffset`](#unitoffset) | struct | An offset into the current compilation or type unit. |
| [`StoreOnHeap`](#storeonheap) | struct | Indicates that storage should be allocated on heap. |
| [`Format`](#format) | enum | Whether the format of a compilation unit is 32- or 64-bit. |
| [`Vendor`](#vendor) | enum | Which vendor extensions to support. |
| [`SectionId`](#sectionid) | enum | An identifier for a DWARF section. |
| [`DwarfFileType`](#dwarffiletype) | enum | The "type" of file with DWARF debugging information. |
| [`RunTimeEndian`](#runtimeendian) | enum | Byte order that is selectable at runtime. |
| [`Error`](#error) | enum | An error that occurred when parsing. |
| [`Endianity`](#endianity) | trait | A trait describing the endianity of some buffer. |
| [`Section`](#section) | trait | A convenience trait for loading DWARF sections from object files. |
| [`case_fold_data`](#case-fold-data) | fn |  |
| [`case_fold`](#case-fold) | fn | Perform case folding for the DWARF name index hashing. |
| [`case_folding_djb_hash`](#case-folding-djb-hash) | fn | Calculate a case folding DJB hash for the DWARF name index. |
| [`djb_hash_byte`](#djb-hash-byte) | fn |  |
| [`NativeEndian`](#nativeendian) | type | The native endianity for the target platform. |
| [`Result`](#result) | type | The result of a parse. |
| [`DW_SECT_INFO`](#dw-sect-info) | const |  |
| [`DW_SECT_ABBREV`](#dw-sect-abbrev) | const |  |
| [`DW_SECT_LINE`](#dw-sect-line) | const |  |
| [`DW_SECT_LOCLISTS`](#dw-sect-loclists) | const |  |
| [`DW_SECT_STR_OFFSETS`](#dw-sect-str-offsets) | const |  |
| [`DW_SECT_MACRO`](#dw-sect-macro) | const |  |
| [`DW_SECT_RNGLISTS`](#dw-sect-rnglists) | const |  |
| [`DW_SECT_V2_INFO`](#dw-sect-v2-info) | const |  |
| [`DW_SECT_V2_TYPES`](#dw-sect-v2-types) | const |  |
| [`DW_SECT_V2_ABBREV`](#dw-sect-v2-abbrev) | const |  |
| [`DW_SECT_V2_LINE`](#dw-sect-v2-line) | const |  |
| [`DW_SECT_V2_LOC`](#dw-sect-v2-loc) | const |  |
| [`DW_SECT_V2_STR_OFFSETS`](#dw-sect-v2-str-offsets) | const |  |
| [`DW_SECT_V2_MACINFO`](#dw-sect-v2-macinfo) | const |  |
| [`DW_SECT_V2_MACRO`](#dw-sect-v2-macro) | const |  |
| [`DW_UT_compile`](#dw-ut-compile) | const |  |
| [`DW_UT_type`](#dw-ut-type) | const |  |
| [`DW_UT_partial`](#dw-ut-partial) | const |  |
| [`DW_UT_skeleton`](#dw-ut-skeleton) | const |  |
| [`DW_UT_split_compile`](#dw-ut-split-compile) | const |  |
| [`DW_UT_split_type`](#dw-ut-split-type) | const |  |
| [`DW_UT_lo_user`](#dw-ut-lo-user) | const |  |
| [`DW_UT_hi_user`](#dw-ut-hi-user) | const |  |
| [`DW_CFA_advance_loc`](#dw-cfa-advance-loc) | const |  |
| [`DW_CFA_offset`](#dw-cfa-offset) | const |  |
| [`DW_CFA_restore`](#dw-cfa-restore) | const |  |
| [`DW_CFA_nop`](#dw-cfa-nop) | const |  |
| [`DW_CFA_set_loc`](#dw-cfa-set-loc) | const |  |
| [`DW_CFA_advance_loc1`](#dw-cfa-advance-loc1) | const |  |
| [`DW_CFA_advance_loc2`](#dw-cfa-advance-loc2) | const |  |
| [`DW_CFA_advance_loc4`](#dw-cfa-advance-loc4) | const |  |
| [`DW_CFA_offset_extended`](#dw-cfa-offset-extended) | const |  |
| [`DW_CFA_restore_extended`](#dw-cfa-restore-extended) | const |  |
| [`DW_CFA_undefined`](#dw-cfa-undefined) | const |  |
| [`DW_CFA_same_value`](#dw-cfa-same-value) | const |  |
| [`DW_CFA_register`](#dw-cfa-register) | const |  |
| [`DW_CFA_remember_state`](#dw-cfa-remember-state) | const |  |
| [`DW_CFA_restore_state`](#dw-cfa-restore-state) | const |  |
| [`DW_CFA_def_cfa`](#dw-cfa-def-cfa) | const |  |
| [`DW_CFA_def_cfa_register`](#dw-cfa-def-cfa-register) | const |  |
| [`DW_CFA_def_cfa_offset`](#dw-cfa-def-cfa-offset) | const |  |
| [`DW_CFA_def_cfa_expression`](#dw-cfa-def-cfa-expression) | const |  |
| [`DW_CFA_expression`](#dw-cfa-expression) | const |  |
| [`DW_CFA_offset_extended_sf`](#dw-cfa-offset-extended-sf) | const |  |
| [`DW_CFA_def_cfa_sf`](#dw-cfa-def-cfa-sf) | const |  |
| [`DW_CFA_def_cfa_offset_sf`](#dw-cfa-def-cfa-offset-sf) | const |  |
| [`DW_CFA_val_offset`](#dw-cfa-val-offset) | const |  |
| [`DW_CFA_val_offset_sf`](#dw-cfa-val-offset-sf) | const |  |
| [`DW_CFA_val_expression`](#dw-cfa-val-expression) | const |  |
| [`DW_CFA_lo_user`](#dw-cfa-lo-user) | const |  |
| [`DW_CFA_hi_user`](#dw-cfa-hi-user) | const |  |
| [`DW_CFA_MIPS_advance_loc8`](#dw-cfa-mips-advance-loc8) | const |  |
| [`DW_CFA_GNU_window_save`](#dw-cfa-gnu-window-save) | const |  |
| [`DW_CFA_GNU_args_size`](#dw-cfa-gnu-args-size) | const |  |
| [`DW_CFA_GNU_negative_offset_extended`](#dw-cfa-gnu-negative-offset-extended) | const |  |
| [`DW_CFA_AARCH64_negate_ra_state`](#dw-cfa-aarch64-negate-ra-state) | const |  |
| [`DW_CHILDREN_no`](#dw-children-no) | const |  |
| [`DW_CHILDREN_yes`](#dw-children-yes) | const |  |
| [`DW_TAG_null`](#dw-tag-null) | const |  |
| [`DW_TAG_global_subroutine`](#dw-tag-global-subroutine) | const |  |
| [`DW_TAG_global_variable`](#dw-tag-global-variable) | const |  |
| [`DW_TAG_local_variable`](#dw-tag-local-variable) | const |  |
| [`DW_TAG_subroutine`](#dw-tag-subroutine) | const |  |
| [`DW_TAG_array_type`](#dw-tag-array-type) | const |  |
| [`DW_TAG_class_type`](#dw-tag-class-type) | const |  |
| [`DW_TAG_entry_point`](#dw-tag-entry-point) | const |  |
| [`DW_TAG_enumeration_type`](#dw-tag-enumeration-type) | const |  |
| [`DW_TAG_formal_parameter`](#dw-tag-formal-parameter) | const |  |
| [`DW_TAG_imported_declaration`](#dw-tag-imported-declaration) | const |  |
| [`DW_TAG_label`](#dw-tag-label) | const |  |
| [`DW_TAG_lexical_block`](#dw-tag-lexical-block) | const |  |
| [`DW_TAG_member`](#dw-tag-member) | const |  |
| [`DW_TAG_pointer_type`](#dw-tag-pointer-type) | const |  |
| [`DW_TAG_reference_type`](#dw-tag-reference-type) | const |  |
| [`DW_TAG_compile_unit`](#dw-tag-compile-unit) | const |  |
| [`DW_TAG_string_type`](#dw-tag-string-type) | const |  |
| [`DW_TAG_structure_type`](#dw-tag-structure-type) | const |  |
| [`DW_TAG_subroutine_type`](#dw-tag-subroutine-type) | const |  |
| [`DW_TAG_typedef`](#dw-tag-typedef) | const |  |
| [`DW_TAG_union_type`](#dw-tag-union-type) | const |  |
| [`DW_TAG_unspecified_parameters`](#dw-tag-unspecified-parameters) | const |  |
| [`DW_TAG_variant`](#dw-tag-variant) | const |  |
| [`DW_TAG_common_block`](#dw-tag-common-block) | const |  |
| [`DW_TAG_common_inclusion`](#dw-tag-common-inclusion) | const |  |
| [`DW_TAG_inheritance`](#dw-tag-inheritance) | const |  |
| [`DW_TAG_inlined_subroutine`](#dw-tag-inlined-subroutine) | const |  |
| [`DW_TAG_module`](#dw-tag-module) | const |  |
| [`DW_TAG_ptr_to_member_type`](#dw-tag-ptr-to-member-type) | const |  |
| [`DW_TAG_set_type`](#dw-tag-set-type) | const |  |
| [`DW_TAG_subrange_type`](#dw-tag-subrange-type) | const |  |
| [`DW_TAG_with_stmt`](#dw-tag-with-stmt) | const |  |
| [`DW_TAG_access_declaration`](#dw-tag-access-declaration) | const |  |
| [`DW_TAG_base_type`](#dw-tag-base-type) | const |  |
| [`DW_TAG_catch_block`](#dw-tag-catch-block) | const |  |
| [`DW_TAG_const_type`](#dw-tag-const-type) | const |  |
| [`DW_TAG_constant`](#dw-tag-constant) | const |  |
| [`DW_TAG_enumerator`](#dw-tag-enumerator) | const |  |
| [`DW_TAG_file_type`](#dw-tag-file-type) | const |  |
| [`DW_TAG_friend`](#dw-tag-friend) | const |  |
| [`DW_TAG_namelist`](#dw-tag-namelist) | const |  |
| [`DW_TAG_namelist_item`](#dw-tag-namelist-item) | const |  |
| [`DW_TAG_packed_type`](#dw-tag-packed-type) | const |  |
| [`DW_TAG_subprogram`](#dw-tag-subprogram) | const |  |
| [`DW_TAG_template_type_parameter`](#dw-tag-template-type-parameter) | const |  |
| [`DW_TAG_template_value_parameter`](#dw-tag-template-value-parameter) | const |  |
| [`DW_TAG_thrown_type`](#dw-tag-thrown-type) | const |  |
| [`DW_TAG_try_block`](#dw-tag-try-block) | const |  |
| [`DW_TAG_variant_part`](#dw-tag-variant-part) | const |  |
| [`DW_TAG_variable`](#dw-tag-variable) | const |  |
| [`DW_TAG_volatile_type`](#dw-tag-volatile-type) | const |  |
| [`DW_TAG_dwarf_procedure`](#dw-tag-dwarf-procedure) | const |  |
| [`DW_TAG_restrict_type`](#dw-tag-restrict-type) | const |  |
| [`DW_TAG_interface_type`](#dw-tag-interface-type) | const |  |
| [`DW_TAG_namespace`](#dw-tag-namespace) | const |  |
| [`DW_TAG_imported_module`](#dw-tag-imported-module) | const |  |
| [`DW_TAG_unspecified_type`](#dw-tag-unspecified-type) | const |  |
| [`DW_TAG_partial_unit`](#dw-tag-partial-unit) | const |  |
| [`DW_TAG_imported_unit`](#dw-tag-imported-unit) | const |  |
| [`DW_TAG_condition`](#dw-tag-condition) | const |  |
| [`DW_TAG_shared_type`](#dw-tag-shared-type) | const |  |
| [`DW_TAG_type_unit`](#dw-tag-type-unit) | const |  |
| [`DW_TAG_rvalue_reference_type`](#dw-tag-rvalue-reference-type) | const |  |
| [`DW_TAG_template_alias`](#dw-tag-template-alias) | const |  |
| [`DW_TAG_coarray_type`](#dw-tag-coarray-type) | const |  |
| [`DW_TAG_generic_subrange`](#dw-tag-generic-subrange) | const |  |
| [`DW_TAG_dynamic_type`](#dw-tag-dynamic-type) | const |  |
| [`DW_TAG_atomic_type`](#dw-tag-atomic-type) | const |  |
| [`DW_TAG_call_site`](#dw-tag-call-site) | const |  |
| [`DW_TAG_call_site_parameter`](#dw-tag-call-site-parameter) | const |  |
| [`DW_TAG_skeleton_unit`](#dw-tag-skeleton-unit) | const |  |
| [`DW_TAG_immutable_type`](#dw-tag-immutable-type) | const |  |
| [`DW_TAG_lo_user`](#dw-tag-lo-user) | const |  |
| [`DW_TAG_hi_user`](#dw-tag-hi-user) | const |  |
| [`DW_TAG_MIPS_loop`](#dw-tag-mips-loop) | const |  |
| [`DW_TAG_HP_array_descriptor`](#dw-tag-hp-array-descriptor) | const |  |
| [`DW_TAG_HP_Bliss_field`](#dw-tag-hp-bliss-field) | const |  |
| [`DW_TAG_HP_Bliss_field_set`](#dw-tag-hp-bliss-field-set) | const |  |
| [`DW_TAG_format_label`](#dw-tag-format-label) | const |  |
| [`DW_TAG_function_template`](#dw-tag-function-template) | const |  |
| [`DW_TAG_class_template`](#dw-tag-class-template) | const |  |
| [`DW_TAG_GNU_BINCL`](#dw-tag-gnu-bincl) | const |  |
| [`DW_TAG_GNU_EINCL`](#dw-tag-gnu-eincl) | const |  |
| [`DW_TAG_GNU_template_template_param`](#dw-tag-gnu-template-template-param) | const |  |
| [`DW_TAG_GNU_template_parameter_pack`](#dw-tag-gnu-template-parameter-pack) | const |  |
| [`DW_TAG_GNU_formal_parameter_pack`](#dw-tag-gnu-formal-parameter-pack) | const |  |
| [`DW_TAG_GNU_call_site`](#dw-tag-gnu-call-site) | const |  |
| [`DW_TAG_GNU_call_site_parameter`](#dw-tag-gnu-call-site-parameter) | const |  |
| [`DW_TAG_APPLE_property`](#dw-tag-apple-property) | const |  |
| [`DW_TAG_SUN_function_template`](#dw-tag-sun-function-template) | const |  |
| [`DW_TAG_SUN_class_template`](#dw-tag-sun-class-template) | const |  |
| [`DW_TAG_SUN_struct_template`](#dw-tag-sun-struct-template) | const |  |
| [`DW_TAG_SUN_union_template`](#dw-tag-sun-union-template) | const |  |
| [`DW_TAG_SUN_indirect_inheritance`](#dw-tag-sun-indirect-inheritance) | const |  |
| [`DW_TAG_SUN_codeflags`](#dw-tag-sun-codeflags) | const |  |
| [`DW_TAG_SUN_memop_info`](#dw-tag-sun-memop-info) | const |  |
| [`DW_TAG_SUN_omp_child_func`](#dw-tag-sun-omp-child-func) | const |  |
| [`DW_TAG_SUN_rtti_descriptor`](#dw-tag-sun-rtti-descriptor) | const |  |
| [`DW_TAG_SUN_dtor_info`](#dw-tag-sun-dtor-info) | const |  |
| [`DW_TAG_SUN_dtor`](#dw-tag-sun-dtor) | const |  |
| [`DW_TAG_SUN_f90_interface`](#dw-tag-sun-f90-interface) | const |  |
| [`DW_TAG_SUN_fortran_vax_structure`](#dw-tag-sun-fortran-vax-structure) | const |  |
| [`DW_TAG_ALTIUM_circ_type`](#dw-tag-altium-circ-type) | const |  |
| [`DW_TAG_ALTIUM_mwa_circ_type`](#dw-tag-altium-mwa-circ-type) | const |  |
| [`DW_TAG_ALTIUM_rev_carry_type`](#dw-tag-altium-rev-carry-type) | const |  |
| [`DW_TAG_ALTIUM_rom`](#dw-tag-altium-rom) | const |  |
| [`DW_TAG_upc_shared_type`](#dw-tag-upc-shared-type) | const |  |
| [`DW_TAG_upc_strict_type`](#dw-tag-upc-strict-type) | const |  |
| [`DW_TAG_upc_relaxed_type`](#dw-tag-upc-relaxed-type) | const |  |
| [`DW_TAG_PGI_kanji_type`](#dw-tag-pgi-kanji-type) | const |  |
| [`DW_TAG_PGI_interface_block`](#dw-tag-pgi-interface-block) | const |  |
| [`DW_TAG_BORLAND_property`](#dw-tag-borland-property) | const |  |
| [`DW_TAG_BORLAND_Delphi_string`](#dw-tag-borland-delphi-string) | const |  |
| [`DW_TAG_BORLAND_Delphi_dynamic_array`](#dw-tag-borland-delphi-dynamic-array) | const |  |
| [`DW_TAG_BORLAND_Delphi_set`](#dw-tag-borland-delphi-set) | const |  |
| [`DW_TAG_BORLAND_Delphi_variant`](#dw-tag-borland-delphi-variant) | const |  |
| [`DW_AT_null`](#dw-at-null) | const |  |
| [`DW_AT_fund_type`](#dw-at-fund-type) | const |  |
| [`DW_AT_mod_fund_type`](#dw-at-mod-fund-type) | const |  |
| [`DW_AT_user_def_type`](#dw-at-user-def-type) | const |  |
| [`DW_AT_mod_u_d_type`](#dw-at-mod-u-d-type) | const |  |
| [`DW_AT_subscr_data`](#dw-at-subscr-data) | const |  |
| [`DW_AT_element_list`](#dw-at-element-list) | const |  |
| [`DW_AT_member`](#dw-at-member) | const |  |
| [`DW_AT_friends`](#dw-at-friends) | const |  |
| [`DW_AT_program`](#dw-at-program) | const |  |
| [`DW_AT_private`](#dw-at-private) | const |  |
| [`DW_AT_protected`](#dw-at-protected) | const |  |
| [`DW_AT_public`](#dw-at-public) | const |  |
| [`DW_AT_pure_virtual`](#dw-at-pure-virtual) | const |  |
| [`DW_AT_virtual`](#dw-at-virtual) | const |  |
| [`DW_AT_specification_v1`](#dw-at-specification-v1) | const |  |
| [`DW_AT_sibling`](#dw-at-sibling) | const |  |
| [`DW_AT_location`](#dw-at-location) | const |  |
| [`DW_AT_name`](#dw-at-name) | const |  |
| [`DW_AT_ordering`](#dw-at-ordering) | const |  |
| [`DW_AT_byte_size`](#dw-at-byte-size) | const |  |
| [`DW_AT_bit_offset`](#dw-at-bit-offset) | const |  |
| [`DW_AT_bit_size`](#dw-at-bit-size) | const |  |
| [`DW_AT_stmt_list`](#dw-at-stmt-list) | const |  |
| [`DW_AT_low_pc`](#dw-at-low-pc) | const |  |
| [`DW_AT_high_pc`](#dw-at-high-pc) | const |  |
| [`DW_AT_language`](#dw-at-language) | const |  |
| [`DW_AT_discr`](#dw-at-discr) | const |  |
| [`DW_AT_discr_value`](#dw-at-discr-value) | const |  |
| [`DW_AT_visibility`](#dw-at-visibility) | const |  |
| [`DW_AT_import`](#dw-at-import) | const |  |
| [`DW_AT_string_length`](#dw-at-string-length) | const |  |
| [`DW_AT_common_reference`](#dw-at-common-reference) | const |  |
| [`DW_AT_comp_dir`](#dw-at-comp-dir) | const |  |
| [`DW_AT_const_value`](#dw-at-const-value) | const |  |
| [`DW_AT_containing_type`](#dw-at-containing-type) | const |  |
| [`DW_AT_default_value`](#dw-at-default-value) | const |  |
| [`DW_AT_inline`](#dw-at-inline) | const |  |
| [`DW_AT_is_optional`](#dw-at-is-optional) | const |  |
| [`DW_AT_lower_bound`](#dw-at-lower-bound) | const |  |
| [`DW_AT_producer`](#dw-at-producer) | const |  |
| [`DW_AT_prototyped`](#dw-at-prototyped) | const |  |
| [`DW_AT_return_addr`](#dw-at-return-addr) | const |  |
| [`DW_AT_start_scope`](#dw-at-start-scope) | const |  |
| [`DW_AT_bit_stride`](#dw-at-bit-stride) | const |  |
| [`DW_AT_upper_bound`](#dw-at-upper-bound) | const |  |
| [`DW_AT_abstract_origin`](#dw-at-abstract-origin) | const |  |
| [`DW_AT_accessibility`](#dw-at-accessibility) | const |  |
| [`DW_AT_address_class`](#dw-at-address-class) | const |  |
| [`DW_AT_artificial`](#dw-at-artificial) | const |  |
| [`DW_AT_base_types`](#dw-at-base-types) | const |  |
| [`DW_AT_calling_convention`](#dw-at-calling-convention) | const |  |
| [`DW_AT_count`](#dw-at-count) | const |  |
| [`DW_AT_data_member_location`](#dw-at-data-member-location) | const |  |
| [`DW_AT_decl_column`](#dw-at-decl-column) | const |  |
| [`DW_AT_decl_file`](#dw-at-decl-file) | const |  |
| [`DW_AT_decl_line`](#dw-at-decl-line) | const |  |
| [`DW_AT_declaration`](#dw-at-declaration) | const |  |
| [`DW_AT_discr_list`](#dw-at-discr-list) | const |  |
| [`DW_AT_encoding`](#dw-at-encoding) | const |  |
| [`DW_AT_external`](#dw-at-external) | const |  |
| [`DW_AT_frame_base`](#dw-at-frame-base) | const |  |
| [`DW_AT_friend`](#dw-at-friend) | const |  |
| [`DW_AT_identifier_case`](#dw-at-identifier-case) | const |  |
| [`DW_AT_macro_info`](#dw-at-macro-info) | const |  |
| [`DW_AT_namelist_item`](#dw-at-namelist-item) | const |  |
| [`DW_AT_priority`](#dw-at-priority) | const |  |
| [`DW_AT_segment`](#dw-at-segment) | const |  |
| [`DW_AT_specification`](#dw-at-specification) | const |  |
| [`DW_AT_static_link`](#dw-at-static-link) | const |  |
| [`DW_AT_type`](#dw-at-type) | const |  |
| [`DW_AT_use_location`](#dw-at-use-location) | const |  |
| [`DW_AT_variable_parameter`](#dw-at-variable-parameter) | const |  |
| [`DW_AT_virtuality`](#dw-at-virtuality) | const |  |
| [`DW_AT_vtable_elem_location`](#dw-at-vtable-elem-location) | const |  |
| [`DW_AT_allocated`](#dw-at-allocated) | const |  |
| [`DW_AT_associated`](#dw-at-associated) | const |  |
| [`DW_AT_data_location`](#dw-at-data-location) | const |  |
| [`DW_AT_byte_stride`](#dw-at-byte-stride) | const |  |
| [`DW_AT_entry_pc`](#dw-at-entry-pc) | const |  |
| [`DW_AT_use_UTF8`](#dw-at-use-utf8) | const |  |
| [`DW_AT_extension`](#dw-at-extension) | const |  |
| [`DW_AT_ranges`](#dw-at-ranges) | const |  |
| [`DW_AT_trampoline`](#dw-at-trampoline) | const |  |
| [`DW_AT_call_column`](#dw-at-call-column) | const |  |
| [`DW_AT_call_file`](#dw-at-call-file) | const |  |
| [`DW_AT_call_line`](#dw-at-call-line) | const |  |
| [`DW_AT_description`](#dw-at-description) | const |  |
| [`DW_AT_binary_scale`](#dw-at-binary-scale) | const |  |
| [`DW_AT_decimal_scale`](#dw-at-decimal-scale) | const |  |
| [`DW_AT_small`](#dw-at-small) | const |  |
| [`DW_AT_decimal_sign`](#dw-at-decimal-sign) | const |  |
| [`DW_AT_digit_count`](#dw-at-digit-count) | const |  |
| [`DW_AT_picture_string`](#dw-at-picture-string) | const |  |
| [`DW_AT_mutable`](#dw-at-mutable) | const |  |
| [`DW_AT_threads_scaled`](#dw-at-threads-scaled) | const |  |
| [`DW_AT_explicit`](#dw-at-explicit) | const |  |
| [`DW_AT_object_pointer`](#dw-at-object-pointer) | const |  |
| [`DW_AT_endianity`](#dw-at-endianity) | const |  |
| [`DW_AT_elemental`](#dw-at-elemental) | const |  |
| [`DW_AT_pure`](#dw-at-pure) | const |  |
| [`DW_AT_recursive`](#dw-at-recursive) | const |  |
| [`DW_AT_signature`](#dw-at-signature) | const |  |
| [`DW_AT_main_subprogram`](#dw-at-main-subprogram) | const |  |
| [`DW_AT_data_bit_offset`](#dw-at-data-bit-offset) | const |  |
| [`DW_AT_const_expr`](#dw-at-const-expr) | const |  |
| [`DW_AT_enum_class`](#dw-at-enum-class) | const |  |
| [`DW_AT_linkage_name`](#dw-at-linkage-name) | const |  |
| [`DW_AT_string_length_bit_size`](#dw-at-string-length-bit-size) | const |  |
| [`DW_AT_string_length_byte_size`](#dw-at-string-length-byte-size) | const |  |
| [`DW_AT_rank`](#dw-at-rank) | const |  |
| [`DW_AT_str_offsets_base`](#dw-at-str-offsets-base) | const |  |
| [`DW_AT_addr_base`](#dw-at-addr-base) | const |  |
| [`DW_AT_rnglists_base`](#dw-at-rnglists-base) | const |  |
| [`DW_AT_dwo_name`](#dw-at-dwo-name) | const |  |
| [`DW_AT_reference`](#dw-at-reference) | const |  |
| [`DW_AT_rvalue_reference`](#dw-at-rvalue-reference) | const |  |
| [`DW_AT_macros`](#dw-at-macros) | const |  |
| [`DW_AT_call_all_calls`](#dw-at-call-all-calls) | const |  |
| [`DW_AT_call_all_source_calls`](#dw-at-call-all-source-calls) | const |  |
| [`DW_AT_call_all_tail_calls`](#dw-at-call-all-tail-calls) | const |  |
| [`DW_AT_call_return_pc`](#dw-at-call-return-pc) | const |  |
| [`DW_AT_call_value`](#dw-at-call-value) | const |  |
| [`DW_AT_call_origin`](#dw-at-call-origin) | const |  |
| [`DW_AT_call_parameter`](#dw-at-call-parameter) | const |  |
| [`DW_AT_call_pc`](#dw-at-call-pc) | const |  |
| [`DW_AT_call_tail_call`](#dw-at-call-tail-call) | const |  |
| [`DW_AT_call_target`](#dw-at-call-target) | const |  |
| [`DW_AT_call_target_clobbered`](#dw-at-call-target-clobbered) | const |  |
| [`DW_AT_call_data_location`](#dw-at-call-data-location) | const |  |
| [`DW_AT_call_data_value`](#dw-at-call-data-value) | const |  |
| [`DW_AT_noreturn`](#dw-at-noreturn) | const |  |
| [`DW_AT_alignment`](#dw-at-alignment) | const |  |
| [`DW_AT_export_symbols`](#dw-at-export-symbols) | const |  |
| [`DW_AT_deleted`](#dw-at-deleted) | const |  |
| [`DW_AT_defaulted`](#dw-at-defaulted) | const |  |
| [`DW_AT_loclists_base`](#dw-at-loclists-base) | const |  |
| [`DW_AT_lo_user`](#dw-at-lo-user) | const |  |
| [`DW_AT_hi_user`](#dw-at-hi-user) | const |  |
| [`DW_AT_MIPS_fde`](#dw-at-mips-fde) | const |  |
| [`DW_AT_MIPS_loop_begin`](#dw-at-mips-loop-begin) | const |  |
| [`DW_AT_MIPS_tail_loop_begin`](#dw-at-mips-tail-loop-begin) | const |  |
| [`DW_AT_MIPS_epilog_begin`](#dw-at-mips-epilog-begin) | const |  |
| [`DW_AT_MIPS_loop_unroll_factor`](#dw-at-mips-loop-unroll-factor) | const |  |
| [`DW_AT_MIPS_software_pipeline_depth`](#dw-at-mips-software-pipeline-depth) | const |  |
| [`DW_AT_MIPS_linkage_name`](#dw-at-mips-linkage-name) | const |  |
| [`DW_AT_MIPS_stride`](#dw-at-mips-stride) | const |  |
| [`DW_AT_MIPS_abstract_name`](#dw-at-mips-abstract-name) | const |  |
| [`DW_AT_MIPS_clone_origin`](#dw-at-mips-clone-origin) | const |  |
| [`DW_AT_MIPS_has_inlines`](#dw-at-mips-has-inlines) | const |  |
| [`DW_AT_MIPS_stride_byte`](#dw-at-mips-stride-byte) | const |  |
| [`DW_AT_MIPS_stride_elem`](#dw-at-mips-stride-elem) | const |  |
| [`DW_AT_MIPS_ptr_dopetype`](#dw-at-mips-ptr-dopetype) | const |  |
| [`DW_AT_MIPS_allocatable_dopetype`](#dw-at-mips-allocatable-dopetype) | const |  |
| [`DW_AT_MIPS_assumed_shape_dopetype`](#dw-at-mips-assumed-shape-dopetype) | const |  |
| [`DW_AT_MIPS_assumed_size`](#dw-at-mips-assumed-size) | const |  |
| [`DW_AT_INTEL_other_endian`](#dw-at-intel-other-endian) | const |  |
| [`DW_AT_sf_names`](#dw-at-sf-names) | const |  |
| [`DW_AT_src_info`](#dw-at-src-info) | const |  |
| [`DW_AT_mac_info`](#dw-at-mac-info) | const |  |
| [`DW_AT_src_coords`](#dw-at-src-coords) | const |  |
| [`DW_AT_body_begin`](#dw-at-body-begin) | const |  |
| [`DW_AT_body_end`](#dw-at-body-end) | const |  |
| [`DW_AT_GNU_vector`](#dw-at-gnu-vector) | const |  |
| [`DW_AT_GNU_guarded_by`](#dw-at-gnu-guarded-by) | const |  |
| [`DW_AT_GNU_pt_guarded_by`](#dw-at-gnu-pt-guarded-by) | const |  |
| [`DW_AT_GNU_guarded`](#dw-at-gnu-guarded) | const |  |
| [`DW_AT_GNU_pt_guarded`](#dw-at-gnu-pt-guarded) | const |  |
| [`DW_AT_GNU_locks_excluded`](#dw-at-gnu-locks-excluded) | const |  |
| [`DW_AT_GNU_exclusive_locks_required`](#dw-at-gnu-exclusive-locks-required) | const |  |
| [`DW_AT_GNU_shared_locks_required`](#dw-at-gnu-shared-locks-required) | const |  |
| [`DW_AT_GNU_odr_signature`](#dw-at-gnu-odr-signature) | const |  |
| [`DW_AT_GNU_template_name`](#dw-at-gnu-template-name) | const |  |
| [`DW_AT_GNU_call_site_value`](#dw-at-gnu-call-site-value) | const |  |
| [`DW_AT_GNU_call_site_data_value`](#dw-at-gnu-call-site-data-value) | const |  |
| [`DW_AT_GNU_call_site_target`](#dw-at-gnu-call-site-target) | const |  |
| [`DW_AT_GNU_call_site_target_clobbered`](#dw-at-gnu-call-site-target-clobbered) | const |  |
| [`DW_AT_GNU_tail_call`](#dw-at-gnu-tail-call) | const |  |
| [`DW_AT_GNU_all_tail_call_sites`](#dw-at-gnu-all-tail-call-sites) | const |  |
| [`DW_AT_GNU_all_call_sites`](#dw-at-gnu-all-call-sites) | const |  |
| [`DW_AT_GNU_all_source_call_sites`](#dw-at-gnu-all-source-call-sites) | const |  |
| [`DW_AT_GNU_macros`](#dw-at-gnu-macros) | const |  |
| [`DW_AT_GNU_deleted`](#dw-at-gnu-deleted) | const |  |
| [`DW_AT_GNU_dwo_name`](#dw-at-gnu-dwo-name) | const |  |
| [`DW_AT_GNU_dwo_id`](#dw-at-gnu-dwo-id) | const |  |
| [`DW_AT_GNU_ranges_base`](#dw-at-gnu-ranges-base) | const |  |
| [`DW_AT_GNU_addr_base`](#dw-at-gnu-addr-base) | const |  |
| [`DW_AT_GNU_pubnames`](#dw-at-gnu-pubnames) | const |  |
| [`DW_AT_GNU_pubtypes`](#dw-at-gnu-pubtypes) | const |  |
| [`DW_AT_GNU_discriminator`](#dw-at-gnu-discriminator) | const |  |
| [`DW_AT_GNU_locviews`](#dw-at-gnu-locviews) | const |  |
| [`DW_AT_GNU_entry_view`](#dw-at-gnu-entry-view) | const |  |
| [`DW_AT_SUN_template`](#dw-at-sun-template) | const |  |
| [`DW_AT_SUN_alignment`](#dw-at-sun-alignment) | const |  |
| [`DW_AT_SUN_vtable`](#dw-at-sun-vtable) | const |  |
| [`DW_AT_SUN_count_guarantee`](#dw-at-sun-count-guarantee) | const |  |
| [`DW_AT_SUN_command_line`](#dw-at-sun-command-line) | const |  |
| [`DW_AT_SUN_vbase`](#dw-at-sun-vbase) | const |  |
| [`DW_AT_SUN_compile_options`](#dw-at-sun-compile-options) | const |  |
| [`DW_AT_SUN_language`](#dw-at-sun-language) | const |  |
| [`DW_AT_SUN_browser_file`](#dw-at-sun-browser-file) | const |  |
| [`DW_AT_SUN_vtable_abi`](#dw-at-sun-vtable-abi) | const |  |
| [`DW_AT_SUN_func_offsets`](#dw-at-sun-func-offsets) | const |  |
| [`DW_AT_SUN_cf_kind`](#dw-at-sun-cf-kind) | const |  |
| [`DW_AT_SUN_vtable_index`](#dw-at-sun-vtable-index) | const |  |
| [`DW_AT_SUN_omp_tpriv_addr`](#dw-at-sun-omp-tpriv-addr) | const |  |
| [`DW_AT_SUN_omp_child_func`](#dw-at-sun-omp-child-func) | const |  |
| [`DW_AT_SUN_func_offset`](#dw-at-sun-func-offset) | const |  |
| [`DW_AT_SUN_memop_type_ref`](#dw-at-sun-memop-type-ref) | const |  |
| [`DW_AT_SUN_profile_id`](#dw-at-sun-profile-id) | const |  |
| [`DW_AT_SUN_memop_signature`](#dw-at-sun-memop-signature) | const |  |
| [`DW_AT_SUN_obj_dir`](#dw-at-sun-obj-dir) | const |  |
| [`DW_AT_SUN_obj_file`](#dw-at-sun-obj-file) | const |  |
| [`DW_AT_SUN_original_name`](#dw-at-sun-original-name) | const |  |
| [`DW_AT_SUN_hwcprof_signature`](#dw-at-sun-hwcprof-signature) | const |  |
| [`DW_AT_SUN_amd64_parmdump`](#dw-at-sun-amd64-parmdump) | const |  |
| [`DW_AT_SUN_part_link_name`](#dw-at-sun-part-link-name) | const |  |
| [`DW_AT_SUN_link_name`](#dw-at-sun-link-name) | const |  |
| [`DW_AT_SUN_pass_with_const`](#dw-at-sun-pass-with-const) | const |  |
| [`DW_AT_SUN_return_with_const`](#dw-at-sun-return-with-const) | const |  |
| [`DW_AT_SUN_import_by_name`](#dw-at-sun-import-by-name) | const |  |
| [`DW_AT_SUN_f90_pointer`](#dw-at-sun-f90-pointer) | const |  |
| [`DW_AT_SUN_pass_by_ref`](#dw-at-sun-pass-by-ref) | const |  |
| [`DW_AT_SUN_f90_allocatable`](#dw-at-sun-f90-allocatable) | const |  |
| [`DW_AT_SUN_f90_assumed_shape_array`](#dw-at-sun-f90-assumed-shape-array) | const |  |
| [`DW_AT_SUN_c_vla`](#dw-at-sun-c-vla) | const |  |
| [`DW_AT_SUN_return_value_ptr`](#dw-at-sun-return-value-ptr) | const |  |
| [`DW_AT_SUN_dtor_start`](#dw-at-sun-dtor-start) | const |  |
| [`DW_AT_SUN_dtor_length`](#dw-at-sun-dtor-length) | const |  |
| [`DW_AT_SUN_dtor_state_initial`](#dw-at-sun-dtor-state-initial) | const |  |
| [`DW_AT_SUN_dtor_state_final`](#dw-at-sun-dtor-state-final) | const |  |
| [`DW_AT_SUN_dtor_state_deltas`](#dw-at-sun-dtor-state-deltas) | const |  |
| [`DW_AT_SUN_import_by_lname`](#dw-at-sun-import-by-lname) | const |  |
| [`DW_AT_SUN_f90_use_only`](#dw-at-sun-f90-use-only) | const |  |
| [`DW_AT_SUN_namelist_spec`](#dw-at-sun-namelist-spec) | const |  |
| [`DW_AT_SUN_is_omp_child_func`](#dw-at-sun-is-omp-child-func) | const |  |
| [`DW_AT_SUN_fortran_main_alias`](#dw-at-sun-fortran-main-alias) | const |  |
| [`DW_AT_SUN_fortran_based`](#dw-at-sun-fortran-based) | const |  |
| [`DW_AT_ALTIUM_loclist`](#dw-at-altium-loclist) | const |  |
| [`DW_AT_use_GNAT_descriptive_type`](#dw-at-use-gnat-descriptive-type) | const |  |
| [`DW_AT_GNAT_descriptive_type`](#dw-at-gnat-descriptive-type) | const |  |
| [`DW_AT_GNU_numerator`](#dw-at-gnu-numerator) | const |  |
| [`DW_AT_GNU_denominator`](#dw-at-gnu-denominator) | const |  |
| [`DW_AT_GNU_bias`](#dw-at-gnu-bias) | const |  |
| [`DW_AT_upc_threads_scaled`](#dw-at-upc-threads-scaled) | const |  |
| [`DW_AT_PGI_lbase`](#dw-at-pgi-lbase) | const |  |
| [`DW_AT_PGI_soffset`](#dw-at-pgi-soffset) | const |  |
| [`DW_AT_PGI_lstride`](#dw-at-pgi-lstride) | const |  |
| [`DW_AT_BORLAND_property_read`](#dw-at-borland-property-read) | const |  |
| [`DW_AT_BORLAND_property_write`](#dw-at-borland-property-write) | const |  |
| [`DW_AT_BORLAND_property_implements`](#dw-at-borland-property-implements) | const |  |
| [`DW_AT_BORLAND_property_index`](#dw-at-borland-property-index) | const |  |
| [`DW_AT_BORLAND_property_default`](#dw-at-borland-property-default) | const |  |
| [`DW_AT_BORLAND_Delphi_unit`](#dw-at-borland-delphi-unit) | const |  |
| [`DW_AT_BORLAND_Delphi_class`](#dw-at-borland-delphi-class) | const |  |
| [`DW_AT_BORLAND_Delphi_record`](#dw-at-borland-delphi-record) | const |  |
| [`DW_AT_BORLAND_Delphi_metaclass`](#dw-at-borland-delphi-metaclass) | const |  |
| [`DW_AT_BORLAND_Delphi_constructor`](#dw-at-borland-delphi-constructor) | const |  |
| [`DW_AT_BORLAND_Delphi_destructor`](#dw-at-borland-delphi-destructor) | const |  |
| [`DW_AT_BORLAND_Delphi_anonymous_method`](#dw-at-borland-delphi-anonymous-method) | const |  |
| [`DW_AT_BORLAND_Delphi_interface`](#dw-at-borland-delphi-interface) | const |  |
| [`DW_AT_BORLAND_Delphi_ABI`](#dw-at-borland-delphi-abi) | const |  |
| [`DW_AT_BORLAND_Delphi_return`](#dw-at-borland-delphi-return) | const |  |
| [`DW_AT_BORLAND_Delphi_frameptr`](#dw-at-borland-delphi-frameptr) | const |  |
| [`DW_AT_BORLAND_closure`](#dw-at-borland-closure) | const |  |
| [`DW_AT_LLVM_include_path`](#dw-at-llvm-include-path) | const |  |
| [`DW_AT_LLVM_config_macros`](#dw-at-llvm-config-macros) | const |  |
| [`DW_AT_LLVM_isysroot`](#dw-at-llvm-isysroot) | const |  |
| [`DW_AT_APPLE_optimized`](#dw-at-apple-optimized) | const |  |
| [`DW_AT_APPLE_flags`](#dw-at-apple-flags) | const |  |
| [`DW_AT_APPLE_isa`](#dw-at-apple-isa) | const |  |
| [`DW_AT_APPLE_block`](#dw-at-apple-block) | const |  |
| [`DW_AT_APPLE_major_runtime_vers`](#dw-at-apple-major-runtime-vers) | const |  |
| [`DW_AT_APPLE_runtime_class`](#dw-at-apple-runtime-class) | const |  |
| [`DW_AT_APPLE_omit_frame_ptr`](#dw-at-apple-omit-frame-ptr) | const |  |
| [`DW_AT_APPLE_property_name`](#dw-at-apple-property-name) | const |  |
| [`DW_AT_APPLE_property_getter`](#dw-at-apple-property-getter) | const |  |
| [`DW_AT_APPLE_property_setter`](#dw-at-apple-property-setter) | const |  |
| [`DW_AT_APPLE_property_attribute`](#dw-at-apple-property-attribute) | const |  |
| [`DW_AT_APPLE_objc_complete_type`](#dw-at-apple-objc-complete-type) | const |  |
| [`DW_AT_APPLE_property`](#dw-at-apple-property) | const |  |
| [`DW_FORM_null`](#dw-form-null) | const |  |
| [`DW_FORM_ref`](#dw-form-ref) | const |  |
| [`DW_FORM_addr`](#dw-form-addr) | const |  |
| [`DW_FORM_block2`](#dw-form-block2) | const |  |
| [`DW_FORM_block4`](#dw-form-block4) | const |  |
| [`DW_FORM_data2`](#dw-form-data2) | const |  |
| [`DW_FORM_data4`](#dw-form-data4) | const |  |
| [`DW_FORM_data8`](#dw-form-data8) | const |  |
| [`DW_FORM_string`](#dw-form-string) | const |  |
| [`DW_FORM_block`](#dw-form-block) | const |  |
| [`DW_FORM_block1`](#dw-form-block1) | const |  |
| [`DW_FORM_data1`](#dw-form-data1) | const |  |
| [`DW_FORM_flag`](#dw-form-flag) | const |  |
| [`DW_FORM_sdata`](#dw-form-sdata) | const |  |
| [`DW_FORM_strp`](#dw-form-strp) | const |  |
| [`DW_FORM_udata`](#dw-form-udata) | const |  |
| [`DW_FORM_ref_addr`](#dw-form-ref-addr) | const |  |
| [`DW_FORM_ref1`](#dw-form-ref1) | const |  |
| [`DW_FORM_ref2`](#dw-form-ref2) | const |  |
| [`DW_FORM_ref4`](#dw-form-ref4) | const |  |
| [`DW_FORM_ref8`](#dw-form-ref8) | const |  |
| [`DW_FORM_ref_udata`](#dw-form-ref-udata) | const |  |
| [`DW_FORM_indirect`](#dw-form-indirect) | const |  |
| [`DW_FORM_sec_offset`](#dw-form-sec-offset) | const |  |
| [`DW_FORM_exprloc`](#dw-form-exprloc) | const |  |
| [`DW_FORM_flag_present`](#dw-form-flag-present) | const |  |
| [`DW_FORM_ref_sig8`](#dw-form-ref-sig8) | const |  |
| [`DW_FORM_strx`](#dw-form-strx) | const |  |
| [`DW_FORM_addrx`](#dw-form-addrx) | const |  |
| [`DW_FORM_ref_sup4`](#dw-form-ref-sup4) | const |  |
| [`DW_FORM_strp_sup`](#dw-form-strp-sup) | const |  |
| [`DW_FORM_data16`](#dw-form-data16) | const |  |
| [`DW_FORM_line_strp`](#dw-form-line-strp) | const |  |
| [`DW_FORM_implicit_const`](#dw-form-implicit-const) | const |  |
| [`DW_FORM_loclistx`](#dw-form-loclistx) | const |  |
| [`DW_FORM_rnglistx`](#dw-form-rnglistx) | const |  |
| [`DW_FORM_ref_sup8`](#dw-form-ref-sup8) | const |  |
| [`DW_FORM_strx1`](#dw-form-strx1) | const |  |
| [`DW_FORM_strx2`](#dw-form-strx2) | const |  |
| [`DW_FORM_strx3`](#dw-form-strx3) | const |  |
| [`DW_FORM_strx4`](#dw-form-strx4) | const |  |
| [`DW_FORM_addrx1`](#dw-form-addrx1) | const |  |
| [`DW_FORM_addrx2`](#dw-form-addrx2) | const |  |
| [`DW_FORM_addrx3`](#dw-form-addrx3) | const |  |
| [`DW_FORM_addrx4`](#dw-form-addrx4) | const |  |
| [`DW_FORM_GNU_addr_index`](#dw-form-gnu-addr-index) | const |  |
| [`DW_FORM_GNU_str_index`](#dw-form-gnu-str-index) | const |  |
| [`DW_FORM_GNU_ref_alt`](#dw-form-gnu-ref-alt) | const |  |
| [`DW_FORM_GNU_strp_alt`](#dw-form-gnu-strp-alt) | const |  |
| [`DW_ATE_address`](#dw-ate-address) | const |  |
| [`DW_ATE_boolean`](#dw-ate-boolean) | const |  |
| [`DW_ATE_complex_float`](#dw-ate-complex-float) | const |  |
| [`DW_ATE_float`](#dw-ate-float) | const |  |
| [`DW_ATE_signed`](#dw-ate-signed) | const |  |
| [`DW_ATE_signed_char`](#dw-ate-signed-char) | const |  |
| [`DW_ATE_unsigned`](#dw-ate-unsigned) | const |  |
| [`DW_ATE_unsigned_char`](#dw-ate-unsigned-char) | const |  |
| [`DW_ATE_imaginary_float`](#dw-ate-imaginary-float) | const |  |
| [`DW_ATE_packed_decimal`](#dw-ate-packed-decimal) | const |  |
| [`DW_ATE_numeric_string`](#dw-ate-numeric-string) | const |  |
| [`DW_ATE_edited`](#dw-ate-edited) | const |  |
| [`DW_ATE_signed_fixed`](#dw-ate-signed-fixed) | const |  |
| [`DW_ATE_unsigned_fixed`](#dw-ate-unsigned-fixed) | const |  |
| [`DW_ATE_decimal_float`](#dw-ate-decimal-float) | const |  |
| [`DW_ATE_UTF`](#dw-ate-utf) | const |  |
| [`DW_ATE_UCS`](#dw-ate-ucs) | const |  |
| [`DW_ATE_ASCII`](#dw-ate-ascii) | const |  |
| [`DW_ATE_lo_user`](#dw-ate-lo-user) | const |  |
| [`DW_ATE_hi_user`](#dw-ate-hi-user) | const |  |
| [`DW_LLE_end_of_list`](#dw-lle-end-of-list) | const |  |
| [`DW_LLE_base_addressx`](#dw-lle-base-addressx) | const |  |
| [`DW_LLE_startx_endx`](#dw-lle-startx-endx) | const |  |
| [`DW_LLE_startx_length`](#dw-lle-startx-length) | const |  |
| [`DW_LLE_offset_pair`](#dw-lle-offset-pair) | const |  |
| [`DW_LLE_default_location`](#dw-lle-default-location) | const |  |
| [`DW_LLE_base_address`](#dw-lle-base-address) | const |  |
| [`DW_LLE_start_end`](#dw-lle-start-end) | const |  |
| [`DW_LLE_start_length`](#dw-lle-start-length) | const |  |
| [`DW_LLE_GNU_view_pair`](#dw-lle-gnu-view-pair) | const |  |
| [`DW_DS_unsigned`](#dw-ds-unsigned) | const |  |
| [`DW_DS_leading_overpunch`](#dw-ds-leading-overpunch) | const |  |
| [`DW_DS_trailing_overpunch`](#dw-ds-trailing-overpunch) | const |  |
| [`DW_DS_leading_separate`](#dw-ds-leading-separate) | const |  |
| [`DW_DS_trailing_separate`](#dw-ds-trailing-separate) | const |  |
| [`DW_END_default`](#dw-end-default) | const |  |
| [`DW_END_big`](#dw-end-big) | const |  |
| [`DW_END_little`](#dw-end-little) | const |  |
| [`DW_END_lo_user`](#dw-end-lo-user) | const |  |
| [`DW_END_hi_user`](#dw-end-hi-user) | const |  |
| [`DW_ACCESS_public`](#dw-access-public) | const |  |
| [`DW_ACCESS_protected`](#dw-access-protected) | const |  |
| [`DW_ACCESS_private`](#dw-access-private) | const |  |
| [`DW_VIS_local`](#dw-vis-local) | const |  |
| [`DW_VIS_exported`](#dw-vis-exported) | const |  |
| [`DW_VIS_qualified`](#dw-vis-qualified) | const |  |
| [`DW_VIRTUALITY_none`](#dw-virtuality-none) | const |  |
| [`DW_VIRTUALITY_virtual`](#dw-virtuality-virtual) | const |  |
| [`DW_VIRTUALITY_pure_virtual`](#dw-virtuality-pure-virtual) | const |  |
| [`DW_LANG_C89`](#dw-lang-c89) | const |  |
| [`DW_LANG_C`](#dw-lang-c) | const |  |
| [`DW_LANG_Ada83`](#dw-lang-ada83) | const |  |
| [`DW_LANG_C_plus_plus`](#dw-lang-c-plus-plus) | const |  |
| [`DW_LANG_Cobol74`](#dw-lang-cobol74) | const |  |
| [`DW_LANG_Cobol85`](#dw-lang-cobol85) | const |  |
| [`DW_LANG_Fortran77`](#dw-lang-fortran77) | const |  |
| [`DW_LANG_Fortran90`](#dw-lang-fortran90) | const |  |
| [`DW_LANG_Pascal83`](#dw-lang-pascal83) | const |  |
| [`DW_LANG_Modula2`](#dw-lang-modula2) | const |  |
| [`DW_LANG_Java`](#dw-lang-java) | const |  |
| [`DW_LANG_C99`](#dw-lang-c99) | const |  |
| [`DW_LANG_Ada95`](#dw-lang-ada95) | const |  |
| [`DW_LANG_Fortran95`](#dw-lang-fortran95) | const |  |
| [`DW_LANG_PLI`](#dw-lang-pli) | const |  |
| [`DW_LANG_ObjC`](#dw-lang-objc) | const |  |
| [`DW_LANG_ObjC_plus_plus`](#dw-lang-objc-plus-plus) | const |  |
| [`DW_LANG_UPC`](#dw-lang-upc) | const |  |
| [`DW_LANG_D`](#dw-lang-d) | const |  |
| [`DW_LANG_Python`](#dw-lang-python) | const |  |
| [`DW_LANG_OpenCL`](#dw-lang-opencl) | const |  |
| [`DW_LANG_Go`](#dw-lang-go) | const |  |
| [`DW_LANG_Modula3`](#dw-lang-modula3) | const |  |
| [`DW_LANG_Haskell`](#dw-lang-haskell) | const |  |
| [`DW_LANG_C_plus_plus_03`](#dw-lang-c-plus-plus-03) | const |  |
| [`DW_LANG_C_plus_plus_11`](#dw-lang-c-plus-plus-11) | const |  |
| [`DW_LANG_OCaml`](#dw-lang-ocaml) | const |  |
| [`DW_LANG_Rust`](#dw-lang-rust) | const |  |
| [`DW_LANG_C11`](#dw-lang-c11) | const |  |
| [`DW_LANG_Swift`](#dw-lang-swift) | const |  |
| [`DW_LANG_Julia`](#dw-lang-julia) | const |  |
| [`DW_LANG_Dylan`](#dw-lang-dylan) | const |  |
| [`DW_LANG_C_plus_plus_14`](#dw-lang-c-plus-plus-14) | const |  |
| [`DW_LANG_Fortran03`](#dw-lang-fortran03) | const |  |
| [`DW_LANG_Fortran08`](#dw-lang-fortran08) | const |  |
| [`DW_LANG_RenderScript`](#dw-lang-renderscript) | const |  |
| [`DW_LANG_BLISS`](#dw-lang-bliss) | const |  |
| [`DW_LANG_Kotlin`](#dw-lang-kotlin) | const |  |
| [`DW_LANG_Zig`](#dw-lang-zig) | const |  |
| [`DW_LANG_Crystal`](#dw-lang-crystal) | const |  |
| [`DW_LANG_C_plus_plus_17`](#dw-lang-c-plus-plus-17) | const |  |
| [`DW_LANG_C_plus_plus_20`](#dw-lang-c-plus-plus-20) | const |  |
| [`DW_LANG_C17`](#dw-lang-c17) | const |  |
| [`DW_LANG_Fortran18`](#dw-lang-fortran18) | const |  |
| [`DW_LANG_Ada2005`](#dw-lang-ada2005) | const |  |
| [`DW_LANG_Ada2012`](#dw-lang-ada2012) | const |  |
| [`DW_LANG_lo_user`](#dw-lang-lo-user) | const |  |
| [`DW_LANG_hi_user`](#dw-lang-hi-user) | const |  |
| [`DW_LANG_Mips_Assembler`](#dw-lang-mips-assembler) | const |  |
| [`DW_LANG_GOOGLE_RenderScript`](#dw-lang-google-renderscript) | const |  |
| [`DW_LANG_SUN_Assembler`](#dw-lang-sun-assembler) | const |  |
| [`DW_LANG_ALTIUM_Assembler`](#dw-lang-altium-assembler) | const |  |
| [`DW_LANG_BORLAND_Delphi`](#dw-lang-borland-delphi) | const |  |
| [`DW_ADDR_none`](#dw-addr-none) | const |  |
| [`DW_ID_case_sensitive`](#dw-id-case-sensitive) | const |  |
| [`DW_ID_up_case`](#dw-id-up-case) | const |  |
| [`DW_ID_down_case`](#dw-id-down-case) | const |  |
| [`DW_ID_case_insensitive`](#dw-id-case-insensitive) | const |  |
| [`DW_CC_normal`](#dw-cc-normal) | const |  |
| [`DW_CC_program`](#dw-cc-program) | const |  |
| [`DW_CC_nocall`](#dw-cc-nocall) | const |  |
| [`DW_CC_pass_by_reference`](#dw-cc-pass-by-reference) | const |  |
| [`DW_CC_pass_by_value`](#dw-cc-pass-by-value) | const |  |
| [`DW_CC_lo_user`](#dw-cc-lo-user) | const |  |
| [`DW_CC_hi_user`](#dw-cc-hi-user) | const |  |
| [`DW_INL_not_inlined`](#dw-inl-not-inlined) | const |  |
| [`DW_INL_inlined`](#dw-inl-inlined) | const |  |
| [`DW_INL_declared_not_inlined`](#dw-inl-declared-not-inlined) | const |  |
| [`DW_INL_declared_inlined`](#dw-inl-declared-inlined) | const |  |
| [`DW_ORD_row_major`](#dw-ord-row-major) | const |  |
| [`DW_ORD_col_major`](#dw-ord-col-major) | const |  |
| [`DW_DSC_label`](#dw-dsc-label) | const |  |
| [`DW_DSC_range`](#dw-dsc-range) | const |  |
| [`DW_IDX_compile_unit`](#dw-idx-compile-unit) | const |  |
| [`DW_IDX_type_unit`](#dw-idx-type-unit) | const |  |
| [`DW_IDX_die_offset`](#dw-idx-die-offset) | const |  |
| [`DW_IDX_parent`](#dw-idx-parent) | const |  |
| [`DW_IDX_type_hash`](#dw-idx-type-hash) | const |  |
| [`DW_IDX_lo_user`](#dw-idx-lo-user) | const |  |
| [`DW_IDX_hi_user`](#dw-idx-hi-user) | const |  |
| [`DW_DEFAULTED_no`](#dw-defaulted-no) | const |  |
| [`DW_DEFAULTED_in_class`](#dw-defaulted-in-class) | const |  |
| [`DW_DEFAULTED_out_of_class`](#dw-defaulted-out-of-class) | const |  |
| [`DW_LNS_copy`](#dw-lns-copy) | const |  |
| [`DW_LNS_advance_pc`](#dw-lns-advance-pc) | const |  |
| [`DW_LNS_advance_line`](#dw-lns-advance-line) | const |  |
| [`DW_LNS_set_file`](#dw-lns-set-file) | const |  |
| [`DW_LNS_set_column`](#dw-lns-set-column) | const |  |
| [`DW_LNS_negate_stmt`](#dw-lns-negate-stmt) | const |  |
| [`DW_LNS_set_basic_block`](#dw-lns-set-basic-block) | const |  |
| [`DW_LNS_const_add_pc`](#dw-lns-const-add-pc) | const |  |
| [`DW_LNS_fixed_advance_pc`](#dw-lns-fixed-advance-pc) | const |  |
| [`DW_LNS_set_prologue_end`](#dw-lns-set-prologue-end) | const |  |
| [`DW_LNS_set_epilogue_begin`](#dw-lns-set-epilogue-begin) | const |  |
| [`DW_LNS_set_isa`](#dw-lns-set-isa) | const |  |
| [`DW_LNE_end_sequence`](#dw-lne-end-sequence) | const |  |
| [`DW_LNE_set_address`](#dw-lne-set-address) | const |  |
| [`DW_LNE_define_file`](#dw-lne-define-file) | const |  |
| [`DW_LNE_set_discriminator`](#dw-lne-set-discriminator) | const |  |
| [`DW_LNE_lo_user`](#dw-lne-lo-user) | const |  |
| [`DW_LNE_hi_user`](#dw-lne-hi-user) | const |  |
| [`DW_LNCT_path`](#dw-lnct-path) | const |  |
| [`DW_LNCT_directory_index`](#dw-lnct-directory-index) | const |  |
| [`DW_LNCT_timestamp`](#dw-lnct-timestamp) | const |  |
| [`DW_LNCT_size`](#dw-lnct-size) | const |  |
| [`DW_LNCT_MD5`](#dw-lnct-md5) | const |  |
| [`DW_LNCT_lo_user`](#dw-lnct-lo-user) | const |  |
| [`DW_LNCT_LLVM_source`](#dw-lnct-llvm-source) | const |  |
| [`DW_LNCT_hi_user`](#dw-lnct-hi-user) | const |  |
| [`DW_MACINFO_define`](#dw-macinfo-define) | const |  |
| [`DW_MACINFO_undef`](#dw-macinfo-undef) | const |  |
| [`DW_MACINFO_start_file`](#dw-macinfo-start-file) | const |  |
| [`DW_MACINFO_end_file`](#dw-macinfo-end-file) | const |  |
| [`DW_MACINFO_vendor_ext`](#dw-macinfo-vendor-ext) | const |  |
| [`DW_MACRO_define`](#dw-macro-define) | const |  |
| [`DW_MACRO_undef`](#dw-macro-undef) | const |  |
| [`DW_MACRO_start_file`](#dw-macro-start-file) | const |  |
| [`DW_MACRO_end_file`](#dw-macro-end-file) | const |  |
| [`DW_MACRO_define_strp`](#dw-macro-define-strp) | const |  |
| [`DW_MACRO_undef_strp`](#dw-macro-undef-strp) | const |  |
| [`DW_MACRO_import`](#dw-macro-import) | const |  |
| [`DW_MACRO_define_sup`](#dw-macro-define-sup) | const |  |
| [`DW_MACRO_undef_sup`](#dw-macro-undef-sup) | const |  |
| [`DW_MACRO_import_sup`](#dw-macro-import-sup) | const |  |
| [`DW_MACRO_define_strx`](#dw-macro-define-strx) | const |  |
| [`DW_MACRO_undef_strx`](#dw-macro-undef-strx) | const |  |
| [`DW_MACRO_lo_user`](#dw-macro-lo-user) | const |  |
| [`DW_MACRO_hi_user`](#dw-macro-hi-user) | const |  |
| [`DW_RLE_end_of_list`](#dw-rle-end-of-list) | const |  |
| [`DW_RLE_base_addressx`](#dw-rle-base-addressx) | const |  |
| [`DW_RLE_startx_endx`](#dw-rle-startx-endx) | const |  |
| [`DW_RLE_startx_length`](#dw-rle-startx-length) | const |  |
| [`DW_RLE_offset_pair`](#dw-rle-offset-pair) | const |  |
| [`DW_RLE_base_address`](#dw-rle-base-address) | const |  |
| [`DW_RLE_start_end`](#dw-rle-start-end) | const |  |
| [`DW_RLE_start_length`](#dw-rle-start-length) | const |  |
| [`DW_OP_addr`](#dw-op-addr) | const |  |
| [`DW_OP_deref`](#dw-op-deref) | const |  |
| [`DW_OP_const1u`](#dw-op-const1u) | const |  |
| [`DW_OP_const1s`](#dw-op-const1s) | const |  |
| [`DW_OP_const2u`](#dw-op-const2u) | const |  |
| [`DW_OP_const2s`](#dw-op-const2s) | const |  |
| [`DW_OP_const4u`](#dw-op-const4u) | const |  |
| [`DW_OP_const4s`](#dw-op-const4s) | const |  |
| [`DW_OP_const8u`](#dw-op-const8u) | const |  |
| [`DW_OP_const8s`](#dw-op-const8s) | const |  |
| [`DW_OP_constu`](#dw-op-constu) | const |  |
| [`DW_OP_consts`](#dw-op-consts) | const |  |
| [`DW_OP_dup`](#dw-op-dup) | const |  |
| [`DW_OP_drop`](#dw-op-drop) | const |  |
| [`DW_OP_over`](#dw-op-over) | const |  |
| [`DW_OP_pick`](#dw-op-pick) | const |  |
| [`DW_OP_swap`](#dw-op-swap) | const |  |
| [`DW_OP_rot`](#dw-op-rot) | const |  |
| [`DW_OP_xderef`](#dw-op-xderef) | const |  |
| [`DW_OP_abs`](#dw-op-abs) | const |  |
| [`DW_OP_and`](#dw-op-and) | const |  |
| [`DW_OP_div`](#dw-op-div) | const |  |
| [`DW_OP_minus`](#dw-op-minus) | const |  |
| [`DW_OP_mod`](#dw-op-mod) | const |  |
| [`DW_OP_mul`](#dw-op-mul) | const |  |
| [`DW_OP_neg`](#dw-op-neg) | const |  |
| [`DW_OP_not`](#dw-op-not) | const |  |
| [`DW_OP_or`](#dw-op-or) | const |  |
| [`DW_OP_plus`](#dw-op-plus) | const |  |
| [`DW_OP_plus_uconst`](#dw-op-plus-uconst) | const |  |
| [`DW_OP_shl`](#dw-op-shl) | const |  |
| [`DW_OP_shr`](#dw-op-shr) | const |  |
| [`DW_OP_shra`](#dw-op-shra) | const |  |
| [`DW_OP_xor`](#dw-op-xor) | const |  |
| [`DW_OP_bra`](#dw-op-bra) | const |  |
| [`DW_OP_eq`](#dw-op-eq) | const |  |
| [`DW_OP_ge`](#dw-op-ge) | const |  |
| [`DW_OP_gt`](#dw-op-gt) | const |  |
| [`DW_OP_le`](#dw-op-le) | const |  |
| [`DW_OP_lt`](#dw-op-lt) | const |  |
| [`DW_OP_ne`](#dw-op-ne) | const |  |
| [`DW_OP_skip`](#dw-op-skip) | const |  |
| [`DW_OP_lit0`](#dw-op-lit0) | const |  |
| [`DW_OP_lit1`](#dw-op-lit1) | const |  |
| [`DW_OP_lit2`](#dw-op-lit2) | const |  |
| [`DW_OP_lit3`](#dw-op-lit3) | const |  |
| [`DW_OP_lit4`](#dw-op-lit4) | const |  |
| [`DW_OP_lit5`](#dw-op-lit5) | const |  |
| [`DW_OP_lit6`](#dw-op-lit6) | const |  |
| [`DW_OP_lit7`](#dw-op-lit7) | const |  |
| [`DW_OP_lit8`](#dw-op-lit8) | const |  |
| [`DW_OP_lit9`](#dw-op-lit9) | const |  |
| [`DW_OP_lit10`](#dw-op-lit10) | const |  |
| [`DW_OP_lit11`](#dw-op-lit11) | const |  |
| [`DW_OP_lit12`](#dw-op-lit12) | const |  |
| [`DW_OP_lit13`](#dw-op-lit13) | const |  |
| [`DW_OP_lit14`](#dw-op-lit14) | const |  |
| [`DW_OP_lit15`](#dw-op-lit15) | const |  |
| [`DW_OP_lit16`](#dw-op-lit16) | const |  |
| [`DW_OP_lit17`](#dw-op-lit17) | const |  |
| [`DW_OP_lit18`](#dw-op-lit18) | const |  |
| [`DW_OP_lit19`](#dw-op-lit19) | const |  |
| [`DW_OP_lit20`](#dw-op-lit20) | const |  |
| [`DW_OP_lit21`](#dw-op-lit21) | const |  |
| [`DW_OP_lit22`](#dw-op-lit22) | const |  |
| [`DW_OP_lit23`](#dw-op-lit23) | const |  |
| [`DW_OP_lit24`](#dw-op-lit24) | const |  |
| [`DW_OP_lit25`](#dw-op-lit25) | const |  |
| [`DW_OP_lit26`](#dw-op-lit26) | const |  |
| [`DW_OP_lit27`](#dw-op-lit27) | const |  |
| [`DW_OP_lit28`](#dw-op-lit28) | const |  |
| [`DW_OP_lit29`](#dw-op-lit29) | const |  |
| [`DW_OP_lit30`](#dw-op-lit30) | const |  |
| [`DW_OP_lit31`](#dw-op-lit31) | const |  |
| [`DW_OP_reg0`](#dw-op-reg0) | const |  |
| [`DW_OP_reg1`](#dw-op-reg1) | const |  |
| [`DW_OP_reg2`](#dw-op-reg2) | const |  |
| [`DW_OP_reg3`](#dw-op-reg3) | const |  |
| [`DW_OP_reg4`](#dw-op-reg4) | const |  |
| [`DW_OP_reg5`](#dw-op-reg5) | const |  |
| [`DW_OP_reg6`](#dw-op-reg6) | const |  |
| [`DW_OP_reg7`](#dw-op-reg7) | const |  |
| [`DW_OP_reg8`](#dw-op-reg8) | const |  |
| [`DW_OP_reg9`](#dw-op-reg9) | const |  |
| [`DW_OP_reg10`](#dw-op-reg10) | const |  |
| [`DW_OP_reg11`](#dw-op-reg11) | const |  |
| [`DW_OP_reg12`](#dw-op-reg12) | const |  |
| [`DW_OP_reg13`](#dw-op-reg13) | const |  |
| [`DW_OP_reg14`](#dw-op-reg14) | const |  |
| [`DW_OP_reg15`](#dw-op-reg15) | const |  |
| [`DW_OP_reg16`](#dw-op-reg16) | const |  |
| [`DW_OP_reg17`](#dw-op-reg17) | const |  |
| [`DW_OP_reg18`](#dw-op-reg18) | const |  |
| [`DW_OP_reg19`](#dw-op-reg19) | const |  |
| [`DW_OP_reg20`](#dw-op-reg20) | const |  |
| [`DW_OP_reg21`](#dw-op-reg21) | const |  |
| [`DW_OP_reg22`](#dw-op-reg22) | const |  |
| [`DW_OP_reg23`](#dw-op-reg23) | const |  |
| [`DW_OP_reg24`](#dw-op-reg24) | const |  |
| [`DW_OP_reg25`](#dw-op-reg25) | const |  |
| [`DW_OP_reg26`](#dw-op-reg26) | const |  |
| [`DW_OP_reg27`](#dw-op-reg27) | const |  |
| [`DW_OP_reg28`](#dw-op-reg28) | const |  |
| [`DW_OP_reg29`](#dw-op-reg29) | const |  |
| [`DW_OP_reg30`](#dw-op-reg30) | const |  |
| [`DW_OP_reg31`](#dw-op-reg31) | const |  |
| [`DW_OP_breg0`](#dw-op-breg0) | const |  |
| [`DW_OP_breg1`](#dw-op-breg1) | const |  |
| [`DW_OP_breg2`](#dw-op-breg2) | const |  |
| [`DW_OP_breg3`](#dw-op-breg3) | const |  |
| [`DW_OP_breg4`](#dw-op-breg4) | const |  |
| [`DW_OP_breg5`](#dw-op-breg5) | const |  |
| [`DW_OP_breg6`](#dw-op-breg6) | const |  |
| [`DW_OP_breg7`](#dw-op-breg7) | const |  |
| [`DW_OP_breg8`](#dw-op-breg8) | const |  |
| [`DW_OP_breg9`](#dw-op-breg9) | const |  |
| [`DW_OP_breg10`](#dw-op-breg10) | const |  |
| [`DW_OP_breg11`](#dw-op-breg11) | const |  |
| [`DW_OP_breg12`](#dw-op-breg12) | const |  |
| [`DW_OP_breg13`](#dw-op-breg13) | const |  |
| [`DW_OP_breg14`](#dw-op-breg14) | const |  |
| [`DW_OP_breg15`](#dw-op-breg15) | const |  |
| [`DW_OP_breg16`](#dw-op-breg16) | const |  |
| [`DW_OP_breg17`](#dw-op-breg17) | const |  |
| [`DW_OP_breg18`](#dw-op-breg18) | const |  |
| [`DW_OP_breg19`](#dw-op-breg19) | const |  |
| [`DW_OP_breg20`](#dw-op-breg20) | const |  |
| [`DW_OP_breg21`](#dw-op-breg21) | const |  |
| [`DW_OP_breg22`](#dw-op-breg22) | const |  |
| [`DW_OP_breg23`](#dw-op-breg23) | const |  |
| [`DW_OP_breg24`](#dw-op-breg24) | const |  |
| [`DW_OP_breg25`](#dw-op-breg25) | const |  |
| [`DW_OP_breg26`](#dw-op-breg26) | const |  |
| [`DW_OP_breg27`](#dw-op-breg27) | const |  |
| [`DW_OP_breg28`](#dw-op-breg28) | const |  |
| [`DW_OP_breg29`](#dw-op-breg29) | const |  |
| [`DW_OP_breg30`](#dw-op-breg30) | const |  |
| [`DW_OP_breg31`](#dw-op-breg31) | const |  |
| [`DW_OP_regx`](#dw-op-regx) | const |  |
| [`DW_OP_fbreg`](#dw-op-fbreg) | const |  |
| [`DW_OP_bregx`](#dw-op-bregx) | const |  |
| [`DW_OP_piece`](#dw-op-piece) | const |  |
| [`DW_OP_deref_size`](#dw-op-deref-size) | const |  |
| [`DW_OP_xderef_size`](#dw-op-xderef-size) | const |  |
| [`DW_OP_nop`](#dw-op-nop) | const |  |
| [`DW_OP_push_object_address`](#dw-op-push-object-address) | const |  |
| [`DW_OP_call2`](#dw-op-call2) | const |  |
| [`DW_OP_call4`](#dw-op-call4) | const |  |
| [`DW_OP_call_ref`](#dw-op-call-ref) | const |  |
| [`DW_OP_form_tls_address`](#dw-op-form-tls-address) | const |  |
| [`DW_OP_call_frame_cfa`](#dw-op-call-frame-cfa) | const |  |
| [`DW_OP_bit_piece`](#dw-op-bit-piece) | const |  |
| [`DW_OP_implicit_value`](#dw-op-implicit-value) | const |  |
| [`DW_OP_stack_value`](#dw-op-stack-value) | const |  |
| [`DW_OP_implicit_pointer`](#dw-op-implicit-pointer) | const |  |
| [`DW_OP_addrx`](#dw-op-addrx) | const |  |
| [`DW_OP_constx`](#dw-op-constx) | const |  |
| [`DW_OP_entry_value`](#dw-op-entry-value) | const |  |
| [`DW_OP_const_type`](#dw-op-const-type) | const |  |
| [`DW_OP_regval_type`](#dw-op-regval-type) | const |  |
| [`DW_OP_deref_type`](#dw-op-deref-type) | const |  |
| [`DW_OP_xderef_type`](#dw-op-xderef-type) | const |  |
| [`DW_OP_convert`](#dw-op-convert) | const |  |
| [`DW_OP_reinterpret`](#dw-op-reinterpret) | const |  |
| [`DW_OP_GNU_push_tls_address`](#dw-op-gnu-push-tls-address) | const |  |
| [`DW_OP_GNU_uninit`](#dw-op-gnu-uninit) | const |  |
| [`DW_OP_GNU_encoded_addr`](#dw-op-gnu-encoded-addr) | const |  |
| [`DW_OP_GNU_implicit_pointer`](#dw-op-gnu-implicit-pointer) | const |  |
| [`DW_OP_GNU_entry_value`](#dw-op-gnu-entry-value) | const |  |
| [`DW_OP_GNU_const_type`](#dw-op-gnu-const-type) | const |  |
| [`DW_OP_GNU_regval_type`](#dw-op-gnu-regval-type) | const |  |
| [`DW_OP_GNU_deref_type`](#dw-op-gnu-deref-type) | const |  |
| [`DW_OP_GNU_convert`](#dw-op-gnu-convert) | const |  |
| [`DW_OP_GNU_reinterpret`](#dw-op-gnu-reinterpret) | const |  |
| [`DW_OP_GNU_parameter_ref`](#dw-op-gnu-parameter-ref) | const |  |
| [`DW_OP_GNU_addr_index`](#dw-op-gnu-addr-index) | const |  |
| [`DW_OP_GNU_const_index`](#dw-op-gnu-const-index) | const |  |
| [`DW_OP_GNU_variable_value`](#dw-op-gnu-variable-value) | const |  |
| [`DW_OP_WASM_location`](#dw-op-wasm-location) | const |  |
| [`DW_EH_PE_uleb128`](#dw-eh-pe-uleb128) | const |  |
| [`DW_EH_PE_udata2`](#dw-eh-pe-udata2) | const |  |
| [`DW_EH_PE_udata4`](#dw-eh-pe-udata4) | const |  |
| [`DW_EH_PE_udata8`](#dw-eh-pe-udata8) | const |  |
| [`DW_EH_PE_sleb128`](#dw-eh-pe-sleb128) | const |  |
| [`DW_EH_PE_sdata2`](#dw-eh-pe-sdata2) | const |  |
| [`DW_EH_PE_sdata4`](#dw-eh-pe-sdata4) | const |  |
| [`DW_EH_PE_sdata8`](#dw-eh-pe-sdata8) | const |  |
| [`DW_EH_PE_pcrel`](#dw-eh-pe-pcrel) | const |  |
| [`DW_EH_PE_textrel`](#dw-eh-pe-textrel) | const |  |
| [`DW_EH_PE_datarel`](#dw-eh-pe-datarel) | const |  |
| [`DW_EH_PE_funcrel`](#dw-eh-pe-funcrel) | const |  |
| [`DW_EH_PE_aligned`](#dw-eh-pe-aligned) | const |  |
| [`DW_EH_PE_indirect`](#dw-eh-pe-indirect) | const |  |
| [`DW_EH_PE_absptr`](#dw-eh-pe-absptr) | const |  |
| [`DW_EH_PE_omit`](#dw-eh-pe-omit) | const |  |
| [`DW_EH_PE_FORMAT_MASK`](#dw-eh-pe-format-mask) | const |  |
| [`DW_EH_PE_APPLICATION_MASK`](#dw-eh-pe-application-mask) | const |  |
| [`registers!`](#registers) | macro |  |
| [`dw!`](#dw) | macro |  |

## Modules

- [`common`](common/index.md)
- [`arch`](arch/index.md)
- [`constants`](constants/index.md) — Constant definitions.
- [`endianity`](endianity/index.md) — Types for compile-time and run-time endianity.
- [`leb128`](leb128/index.md) — Read and write DWARF's "Little Endian Base 128" (LEB128) variable length
- [`case_fold`](case_fold/index.md)
- [`read`](read/index.md) — Read DWARF debugging information.
- [`util`](util/index.md)
- [`addr`](addr/index.md)
- [`cfi`](cfi/index.md)
- [`dwarf`](dwarf/index.md)
- [`endian_slice`](endian_slice/index.md) — Working with byte slices that have an associated endianity.
- [`endian_reader`](endian_reader/index.md) — Defining custom `Reader`s quickly.
- [`reader`](reader/index.md)
- [`relocate`](relocate/index.md)
- [`abbrev`](abbrev/index.md) — Functions for parsing DWARF debugging abbreviations.
- [`aranges`](aranges/index.md)
- [`index`](index/index.md)
- [`line`](line/index.md)
- [`lists`](lists/index.md)
- [`loclists`](loclists/index.md)
- [`lookup`](lookup/index.md)
- [`macros`](macros/index.md)
- [`names`](names/index.md) — Functions for parsing DWARF 5 `.debug_names` sections.
- [`op`](op/index.md) — Functions for parsing and evaluating DWARF expressions.
- [`pubnames`](pubnames/index.md)
- [`pubtypes`](pubtypes/index.md)
- [`rnglists`](rnglists/index.md)
- [`str`](str/index.md)
- [`unit`](unit/index.md) — Functions for parsing DWARF `.debug_info` and `.debug_types` sections.
- [`value`](value/index.md) — Definitions for values used in DWARF expressions.

## Structs

### `Encoding`

```rust
struct Encoding {
    pub address_size: u8,
    pub format: Format,
    pub version: u16,
}
```

Encoding parameters that are commonly used for multiple DWARF sections.

This is intended to be small enough to pass by value.

#### Fields

- **`address_size`**: `u8`

  The size of an address.

- **`format`**: `Format`

  Whether the DWARF format is 32- or 64-bit.

- **`version`**: `u16`

  The DWARF version of the header.

#### Trait Implementations

##### `impl Clone for Encoding`

- <span id="encoding-clone"></span>`fn clone(&self) -> Encoding` — [`Encoding`](#encoding)

##### `impl Copy for Encoding`

##### `impl Debug for Encoding`

- <span id="encoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Encoding`

##### `impl Hash for Encoding`

- <span id="encoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Encoding`

- <span id="encoding-partialeq-eq"></span>`fn eq(&self, other: &Encoding) -> bool` — [`Encoding`](#encoding)

##### `impl StructuralPartialEq for Encoding`

### `LineEncoding`

```rust
struct LineEncoding {
    pub minimum_instruction_length: u8,
    pub maximum_operations_per_instruction: u8,
    pub default_is_stmt: bool,
    pub line_base: i8,
    pub line_range: u8,
}
```

Encoding parameters for a line number program.

#### Fields

- **`minimum_instruction_length`**: `u8`

  The size in bytes of the smallest target machine instruction.

- **`maximum_operations_per_instruction`**: `u8`

  The maximum number of individual operations that may be encoded in an
  instruction.

- **`default_is_stmt`**: `bool`

  The initial value of the `is_stmt` register.

- **`line_base`**: `i8`

  The minimum value which a special opcode can add to the line register.

- **`line_range`**: `u8`

  The range of values which a special opcode can add to the line register.

#### Trait Implementations

##### `impl Clone for LineEncoding`

- <span id="lineencoding-clone"></span>`fn clone(&self) -> LineEncoding` — [`LineEncoding`](#lineencoding)

##### `impl Copy for LineEncoding`

##### `impl Debug for LineEncoding`

- <span id="lineencoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LineEncoding`

- <span id="lineencoding-default"></span>`fn default() -> Self`

##### `impl Eq for LineEncoding`

##### `impl Hash for LineEncoding`

- <span id="lineencoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for LineEncoding`

- <span id="lineencoding-partialeq-eq"></span>`fn eq(&self, other: &LineEncoding) -> bool` — [`LineEncoding`](#lineencoding)

##### `impl StructuralPartialEq for LineEncoding`

### `Register`

```rust
struct Register(u16);
```

A DWARF register number.

The meaning of this value is ABI dependent. This is generally encoded as
a ULEB128, but supported architectures need 16 bits at most.

#### Implementations

- <span id="cratecommonregister-from-u64"></span>`fn from_u64(x: u64) -> Result<Register>` — [`Result`](#result), [`Register`](#register)

#### Trait Implementations

##### `impl Clone for Register`

- <span id="register-clone"></span>`fn clone(&self) -> Register` — [`Register`](#register)

##### `impl Copy for Register`

##### `impl Debug for Register`

- <span id="register-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Register`

##### `impl Hash for Register`

- <span id="register-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Register`

- <span id="register-ord-cmp"></span>`fn cmp(&self, other: &Register) -> cmp::Ordering` — [`Register`](#register)

##### `impl PartialEq for Register`

- <span id="register-partialeq-eq"></span>`fn eq(&self, other: &Register) -> bool` — [`Register`](#register)

##### `impl PartialOrd for Register`

- <span id="register-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Register) -> option::Option<cmp::Ordering>` — [`Register`](#register)

##### `impl StructuralPartialEq for Register`

### `DebugAbbrevOffset<T>`

```rust
struct DebugAbbrevOffset<T>(T);
```

An offset into the `.debug_abbrev` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-clone"></span>`fn clone(&self) -> DebugAbbrevOffset<T>` — [`DebugAbbrevOffset`](#debugabbrevoffset)

##### `impl<T: marker::Copy> Copy for DebugAbbrevOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAbbrevOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugAbbrevOffset<T>) -> bool` — [`DebugAbbrevOffset`](#debugabbrevoffset)

##### `impl<T> StructuralPartialEq for DebugAbbrevOffset<T>`

### `DebugAddrOffset<T>`

```rust
struct DebugAddrOffset<T>(T);
```

An offset into the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAddrOffset<T>`

- <span id="debugaddroffset-clone"></span>`fn clone(&self) -> DebugAddrOffset<T>` — [`DebugAddrOffset`](#debugaddroffset)

##### `impl<T: marker::Copy> Copy for DebugAddrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrOffset<T>`

- <span id="debugaddroffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrOffset<T>`

- <span id="debugaddroffset-partialeq-eq"></span>`fn eq(&self, other: &DebugAddrOffset<T>) -> bool` — [`DebugAddrOffset`](#debugaddroffset)

##### `impl<T> StructuralPartialEq for DebugAddrOffset<T>`

### `DebugAddrBase<T>`

```rust
struct DebugAddrBase<T>(T);
```

An offset to a set of entries in the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAddrBase<T>`

- <span id="debugaddrbase-clone"></span>`fn clone(&self) -> DebugAddrBase<T>` — [`DebugAddrBase`](#debugaddrbase)

##### `impl<T: marker::Copy> Copy for DebugAddrBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrBase<T>`

- <span id="debugaddrbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrBase<T>`

- <span id="debugaddrbase-partialeq-eq"></span>`fn eq(&self, other: &DebugAddrBase<T>) -> bool` — [`DebugAddrBase`](#debugaddrbase)

##### `impl<T> StructuralPartialEq for DebugAddrBase<T>`

### `DebugAddrIndex<T>`

```rust
struct DebugAddrIndex<T>(T);
```

An index into a set of addresses in the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAddrIndex<T>`

- <span id="debugaddrindex-clone"></span>`fn clone(&self) -> DebugAddrIndex<T>` — [`DebugAddrIndex`](#debugaddrindex)

##### `impl<T: marker::Copy> Copy for DebugAddrIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrIndex<T>`

- <span id="debugaddrindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrIndex<T>`

- <span id="debugaddrindex-partialeq-eq"></span>`fn eq(&self, other: &DebugAddrIndex<T>) -> bool` — [`DebugAddrIndex`](#debugaddrindex)

##### `impl<T> StructuralPartialEq for DebugAddrIndex<T>`

### `DebugArangesOffset<T>`

```rust
struct DebugArangesOffset<T>(T);
```

An offset into the `.debug_aranges` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugArangesOffset<T>`

- <span id="debugarangesoffset-clone"></span>`fn clone(&self) -> DebugArangesOffset<T>` — [`DebugArangesOffset`](#debugarangesoffset)

##### `impl<T: marker::Copy> Copy for DebugArangesOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugArangesOffset<T>`

- <span id="debugarangesoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugArangesOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugArangesOffset<T>`

- <span id="debugarangesoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugArangesOffset<T>) -> bool` — [`DebugArangesOffset`](#debugarangesoffset)

##### `impl<T> StructuralPartialEq for DebugArangesOffset<T>`

### `DebugInfoOffset<T>`

```rust
struct DebugInfoOffset<T>(T);
```

An offset into the `.debug_info` section.

#### Implementations

- <span id="cratecommondebuginfooffset-to-unit-section-offset"></span>`fn to_unit_section_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitSectionOffset<T>>` — [`UnitHeader`](read/index.md#unitheader), [`UnitSectionOffset`](#unitsectionoffset)

  Convert a `.debug_info` offset to be an offset within the section containing the

  given unit.

  

  Returns `None` if the unit is not within the `.debug_info` section.

- <span id="cratecommondebuginfooffset-to-unit-offset"></span>`fn to_unit_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](read/index.md#unitheader), [`UnitOffset`](#unitoffset)

  Convert an offset to be relative to the start of the given unit,

  instead of relative to the start of the `.debug_info` section.

  

  Returns `None` if the offset is not in bounds for the unit's entries.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugInfoOffset<T>`

- <span id="debuginfooffset-clone"></span>`fn clone(&self) -> DebugInfoOffset<T>` — [`DebugInfoOffset`](#debuginfooffset)

##### `impl<T: marker::Copy> Copy for DebugInfoOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugInfoOffset<T>`

- <span id="debuginfooffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugInfoOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugInfoOffset<T>`

- <span id="debuginfooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for DebugInfoOffset<T>`

- <span id="debuginfooffset-ord-cmp"></span>`fn cmp(&self, other: &DebugInfoOffset<T>) -> cmp::Ordering` — [`DebugInfoOffset`](#debuginfooffset)

##### `impl<T: cmp::PartialEq> PartialEq for DebugInfoOffset<T>`

- <span id="debuginfooffset-partialeq-eq"></span>`fn eq(&self, other: &DebugInfoOffset<T>) -> bool` — [`DebugInfoOffset`](#debuginfooffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for DebugInfoOffset<T>`

- <span id="debuginfooffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DebugInfoOffset<T>) -> option::Option<cmp::Ordering>` — [`DebugInfoOffset`](#debuginfooffset)

##### `impl<T> StructuralPartialEq for DebugInfoOffset<T>`

### `DebugLineOffset<T>`

```rust
struct DebugLineOffset<T>(T);
```

An offset into the `.debug_line` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLineOffset<T>`

- <span id="debuglineoffset-clone"></span>`fn clone(&self) -> DebugLineOffset<T>` — [`DebugLineOffset`](#debuglineoffset)

##### `impl<T: marker::Copy> Copy for DebugLineOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugLineOffset<T>`

- <span id="debuglineoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLineOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLineOffset<T>`

- <span id="debuglineoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugLineOffset<T>) -> bool` — [`DebugLineOffset`](#debuglineoffset)

##### `impl<T> StructuralPartialEq for DebugLineOffset<T>`

### `DebugLineStrOffset<T>`

```rust
struct DebugLineStrOffset<T>(T);
```

An offset into the `.debug_line_str` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-clone"></span>`fn clone(&self) -> DebugLineStrOffset<T>` — [`DebugLineStrOffset`](#debuglinestroffset)

##### `impl<T: marker::Copy> Copy for DebugLineStrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLineStrOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-partialeq-eq"></span>`fn eq(&self, other: &DebugLineStrOffset<T>) -> bool` — [`DebugLineStrOffset`](#debuglinestroffset)

##### `impl<T> StructuralPartialEq for DebugLineStrOffset<T>`

### `LocationListsOffset<T>`

```rust
struct LocationListsOffset<T>(T);
```

An offset into either the `.debug_loc` section or the `.debug_loclists` section,
depending on the version of the unit the offset was contained in.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for LocationListsOffset<T>`

- <span id="locationlistsoffset-clone"></span>`fn clone(&self) -> LocationListsOffset<T>` — [`LocationListsOffset`](#locationlistsoffset)

##### `impl<T: marker::Copy> Copy for LocationListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for LocationListsOffset<T>`

- <span id="locationlistsoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for LocationListsOffset<T>`

##### `impl<T: hash::Hash> Hash for LocationListsOffset<T>`

- <span id="locationlistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for LocationListsOffset<T>`

- <span id="locationlistsoffset-partialeq-eq"></span>`fn eq(&self, other: &LocationListsOffset<T>) -> bool` — [`LocationListsOffset`](#locationlistsoffset)

##### `impl<T> StructuralPartialEq for LocationListsOffset<T>`

### `DebugLocListsBase<T>`

```rust
struct DebugLocListsBase<T>(T);
```

An offset to a set of location list offsets in the `.debug_loclists` section.

#### Implementations

- <span id="cratecommondebugloclistsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugLocListsBase<Offset>` — [`Encoding`](#encoding), [`DwarfFileType`](#dwarffiletype), [`DebugLocListsBase`](#debugloclistsbase)

  Returns a `DebugLocListsBase` with the default value of DW_AT_loclists_base

  for the given `Encoding` and `DwarfFileType`.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLocListsBase<T>`

- <span id="debugloclistsbase-clone"></span>`fn clone(&self) -> DebugLocListsBase<T>` — [`DebugLocListsBase`](#debugloclistsbase)

##### `impl<T: marker::Copy> Copy for DebugLocListsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugLocListsBase<T>`

- <span id="debugloclistsbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLocListsBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLocListsBase<T>`

- <span id="debugloclistsbase-partialeq-eq"></span>`fn eq(&self, other: &DebugLocListsBase<T>) -> bool` — [`DebugLocListsBase`](#debugloclistsbase)

##### `impl<T> StructuralPartialEq for DebugLocListsBase<T>`

### `DebugLocListsIndex<T>`

```rust
struct DebugLocListsIndex<T>(T);
```

An index into a set of location list offsets in the `.debug_loclists` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-clone"></span>`fn clone(&self) -> DebugLocListsIndex<T>` — [`DebugLocListsIndex`](#debugloclistsindex)

##### `impl<T: marker::Copy> Copy for DebugLocListsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLocListsIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-partialeq-eq"></span>`fn eq(&self, other: &DebugLocListsIndex<T>) -> bool` — [`DebugLocListsIndex`](#debugloclistsindex)

##### `impl<T> StructuralPartialEq for DebugLocListsIndex<T>`

### `DebugMacinfoOffset<T>`

```rust
struct DebugMacinfoOffset<T>(T);
```

An offset into the `.debug_macinfo` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-clone"></span>`fn clone(&self) -> DebugMacinfoOffset<T>` — [`DebugMacinfoOffset`](#debugmacinfooffset)

##### `impl<T: marker::Copy> Copy for DebugMacinfoOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugMacinfoOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-partialeq-eq"></span>`fn eq(&self, other: &DebugMacinfoOffset<T>) -> bool` — [`DebugMacinfoOffset`](#debugmacinfooffset)

##### `impl<T> StructuralPartialEq for DebugMacinfoOffset<T>`

### `DebugMacroOffset<T>`

```rust
struct DebugMacroOffset<T>(T);
```

An offset into the `.debug_macro` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugMacroOffset<T>`

- <span id="debugmacrooffset-clone"></span>`fn clone(&self) -> DebugMacroOffset<T>` — [`DebugMacroOffset`](#debugmacrooffset)

##### `impl<T: marker::Copy> Copy for DebugMacroOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugMacroOffset<T>`

- <span id="debugmacrooffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugMacroOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugMacroOffset<T>`

- <span id="debugmacrooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugMacroOffset<T>`

- <span id="debugmacrooffset-partialeq-eq"></span>`fn eq(&self, other: &DebugMacroOffset<T>) -> bool` — [`DebugMacroOffset`](#debugmacrooffset)

##### `impl<T> StructuralPartialEq for DebugMacroOffset<T>`

### `DebugNamesOffset<T>`

```rust
struct DebugNamesOffset<T>(T);
```

An offset into the `.debug_names` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugNamesOffset<T>`

- <span id="debugnamesoffset-clone"></span>`fn clone(&self) -> DebugNamesOffset<T>` — [`DebugNamesOffset`](#debugnamesoffset)

##### `impl<T: marker::Copy> Copy for DebugNamesOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugNamesOffset<T>`

- <span id="debugnamesoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugNamesOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugNamesOffset<T>`

- <span id="debugnamesoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugNamesOffset<T>`

- <span id="debugnamesoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugNamesOffset<T>) -> bool` — [`DebugNamesOffset`](#debugnamesoffset)

##### `impl<T> StructuralPartialEq for DebugNamesOffset<T>`

### `RawRangeListsOffset<T>`

```rust
struct RawRangeListsOffset<T>(T);
```

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

If this is from a DWARF 4 DWO file, then it must additionally be offset by the
value of `DW_AT_GNU_ranges_base`. You can use `Dwarf::ranges_offset_from_raw` to do this.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-clone"></span>`fn clone(&self) -> RawRangeListsOffset<T>` — [`RawRangeListsOffset`](#rawrangelistsoffset)

##### `impl<T: marker::Copy> Copy for RawRangeListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for RawRangeListsOffset<T>`

##### `impl<T: hash::Hash> Hash for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-partialeq-eq"></span>`fn eq(&self, other: &RawRangeListsOffset<T>) -> bool` — [`RawRangeListsOffset`](#rawrangelistsoffset)

##### `impl<T> StructuralPartialEq for RawRangeListsOffset<T>`

### `RangeListsOffset<T>`

```rust
struct RangeListsOffset<T>(T);
```

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RangeListsOffset<T>`

- <span id="rangelistsoffset-clone"></span>`fn clone(&self) -> RangeListsOffset<T>` — [`RangeListsOffset`](#rangelistsoffset)

##### `impl<T: marker::Copy> Copy for RangeListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for RangeListsOffset<T>`

- <span id="rangelistsoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for RangeListsOffset<T>`

##### `impl<T: hash::Hash> Hash for RangeListsOffset<T>`

- <span id="rangelistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for RangeListsOffset<T>`

- <span id="rangelistsoffset-partialeq-eq"></span>`fn eq(&self, other: &RangeListsOffset<T>) -> bool` — [`RangeListsOffset`](#rangelistsoffset)

##### `impl<T> StructuralPartialEq for RangeListsOffset<T>`

### `DebugRngListsBase<T>`

```rust
struct DebugRngListsBase<T>(T);
```

An offset to a set of range list offsets in the `.debug_rnglists` section.

#### Implementations

- <span id="cratecommondebugrnglistsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugRngListsBase<Offset>` — [`Encoding`](#encoding), [`DwarfFileType`](#dwarffiletype), [`DebugRngListsBase`](#debugrnglistsbase)

  Returns a `DebugRngListsBase` with the default value of DW_AT_rnglists_base

  for the given `Encoding` and `DwarfFileType`.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-clone"></span>`fn clone(&self) -> DebugRngListsBase<T>` — [`DebugRngListsBase`](#debugrnglistsbase)

##### `impl<T: marker::Copy> Copy for DebugRngListsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugRngListsBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-partialeq-eq"></span>`fn eq(&self, other: &DebugRngListsBase<T>) -> bool` — [`DebugRngListsBase`](#debugrnglistsbase)

##### `impl<T> StructuralPartialEq for DebugRngListsBase<T>`

### `DebugRngListsIndex<T>`

```rust
struct DebugRngListsIndex<T>(T);
```

An index into a set of range list offsets in the `.debug_rnglists` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-clone"></span>`fn clone(&self) -> DebugRngListsIndex<T>` — [`DebugRngListsIndex`](#debugrnglistsindex)

##### `impl<T: marker::Copy> Copy for DebugRngListsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugRngListsIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-partialeq-eq"></span>`fn eq(&self, other: &DebugRngListsIndex<T>) -> bool` — [`DebugRngListsIndex`](#debugrnglistsindex)

##### `impl<T> StructuralPartialEq for DebugRngListsIndex<T>`

### `DebugStrOffset<T>`

```rust
struct DebugStrOffset<T>(T);
```

An offset into the `.debug_str` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugStrOffset<T>`

- <span id="debugstroffset-clone"></span>`fn clone(&self) -> DebugStrOffset<T>` — [`DebugStrOffset`](#debugstroffset)

##### `impl<T: marker::Copy> Copy for DebugStrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffset<T>`

- <span id="debugstroffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffset<T>`

- <span id="debugstroffset-partialeq-eq"></span>`fn eq(&self, other: &DebugStrOffset<T>) -> bool` — [`DebugStrOffset`](#debugstroffset)

##### `impl<T> StructuralPartialEq for DebugStrOffset<T>`

### `DebugStrOffsetsBase<T>`

```rust
struct DebugStrOffsetsBase<T>(T);
```

An offset to a set of entries in the `.debug_str_offsets` section.

#### Implementations

- <span id="cratecommondebugstroffsetsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugStrOffsetsBase<Offset>` — [`Encoding`](#encoding), [`DwarfFileType`](#dwarffiletype), [`DebugStrOffsetsBase`](#debugstroffsetsbase)

  Returns a `DebugStrOffsetsBase` with the default value of DW_AT_str_offsets_base

  for the given `Encoding` and `DwarfFileType`.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-clone"></span>`fn clone(&self) -> DebugStrOffsetsBase<T>` — [`DebugStrOffsetsBase`](#debugstroffsetsbase)

##### `impl<T: marker::Copy> Copy for DebugStrOffsetsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffsetsBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-partialeq-eq"></span>`fn eq(&self, other: &DebugStrOffsetsBase<T>) -> bool` — [`DebugStrOffsetsBase`](#debugstroffsetsbase)

##### `impl<T> StructuralPartialEq for DebugStrOffsetsBase<T>`

### `DebugStrOffsetsIndex<T>`

```rust
struct DebugStrOffsetsIndex<T>(T);
```

An index into a set of entries in the `.debug_str_offsets` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-clone"></span>`fn clone(&self) -> DebugStrOffsetsIndex<T>` — [`DebugStrOffsetsIndex`](#debugstroffsetsindex)

##### `impl<T: marker::Copy> Copy for DebugStrOffsetsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffsetsIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-partialeq-eq"></span>`fn eq(&self, other: &DebugStrOffsetsIndex<T>) -> bool` — [`DebugStrOffsetsIndex`](#debugstroffsetsindex)

##### `impl<T> StructuralPartialEq for DebugStrOffsetsIndex<T>`

### `DebugTypesOffset<T>`

```rust
struct DebugTypesOffset<T>(T);
```

An offset into the `.debug_types` section.

#### Implementations

- <span id="cratecommondebugtypesoffset-to-unit-section-offset"></span>`fn to_unit_section_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitSectionOffset<T>>` — [`UnitHeader`](read/index.md#unitheader), [`UnitSectionOffset`](#unitsectionoffset)

  Convert a `.debug_types` offset to be an offset within the section containing the

  given unit.

  

  Returns `None` if the unit is not within the `.debug_types` section.

- <span id="cratecommondebugtypesoffset-to-unit-offset"></span>`fn to_unit_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](read/index.md#unitheader), [`UnitOffset`](#unitoffset)

  Convert an offset to be relative to the start of the given unit,

  instead of relative to the start of the `.debug_types` section.

  

  Returns `None` if the offset is not in bounds for the unit's entries.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugTypesOffset<T>`

- <span id="debugtypesoffset-clone"></span>`fn clone(&self) -> DebugTypesOffset<T>` — [`DebugTypesOffset`](#debugtypesoffset)

##### `impl<T: marker::Copy> Copy for DebugTypesOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugTypesOffset<T>`

- <span id="debugtypesoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugTypesOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugTypesOffset<T>`

- <span id="debugtypesoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for DebugTypesOffset<T>`

- <span id="debugtypesoffset-ord-cmp"></span>`fn cmp(&self, other: &DebugTypesOffset<T>) -> cmp::Ordering` — [`DebugTypesOffset`](#debugtypesoffset)

##### `impl<T: cmp::PartialEq> PartialEq for DebugTypesOffset<T>`

- <span id="debugtypesoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugTypesOffset<T>) -> bool` — [`DebugTypesOffset`](#debugtypesoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for DebugTypesOffset<T>`

- <span id="debugtypesoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DebugTypesOffset<T>) -> option::Option<cmp::Ordering>` — [`DebugTypesOffset`](#debugtypesoffset)

##### `impl<T> StructuralPartialEq for DebugTypesOffset<T>`

### `DebugTypeSignature`

```rust
struct DebugTypeSignature(u64);
```

A type signature as used in the `.debug_types` section.

#### Trait Implementations

##### `impl Clone for DebugTypeSignature`

- <span id="debugtypesignature-clone"></span>`fn clone(&self) -> DebugTypeSignature` — [`DebugTypeSignature`](#debugtypesignature)

##### `impl Copy for DebugTypeSignature`

##### `impl Debug for DebugTypeSignature`

- <span id="debugtypesignature-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DebugTypeSignature`

##### `impl Hash for DebugTypeSignature`

- <span id="debugtypesignature-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DebugTypeSignature`

- <span id="debugtypesignature-partialeq-eq"></span>`fn eq(&self, other: &DebugTypeSignature) -> bool` — [`DebugTypeSignature`](#debugtypesignature)

##### `impl StructuralPartialEq for DebugTypeSignature`

### `DebugFrameOffset<T>`

```rust
struct DebugFrameOffset<T>(T);
```

An offset into the `.debug_frame` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugFrameOffset<T>`

- <span id="debugframeoffset-clone"></span>`fn clone(&self) -> DebugFrameOffset<T>` — [`DebugFrameOffset`](#debugframeoffset)

##### `impl<T: marker::Copy> Copy for DebugFrameOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugFrameOffset<T>`

- <span id="debugframeoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugFrameOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugFrameOffset<T>`

- <span id="debugframeoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugFrameOffset<T>`

- <span id="debugframeoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugFrameOffset<T>) -> bool` — [`DebugFrameOffset`](#debugframeoffset)

##### `impl<T> StructuralPartialEq for DebugFrameOffset<T>`

##### `impl<T> UnwindOffset for crate::common::DebugFrameOffset<T>`

- <span id="cratecommondebugframeoffset-unwindoffset-into"></span>`fn into(self) -> T`

### `EhFrameOffset<T>`

```rust
struct EhFrameOffset<T>(T);
```

An offset into the `.eh_frame` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for EhFrameOffset<T>`

- <span id="ehframeoffset-clone"></span>`fn clone(&self) -> EhFrameOffset<T>` — [`EhFrameOffset`](#ehframeoffset)

##### `impl<T: marker::Copy> Copy for EhFrameOffset<T>`

##### `impl<T: fmt::Debug> Debug for EhFrameOffset<T>`

- <span id="ehframeoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for EhFrameOffset<T>`

##### `impl<T: hash::Hash> Hash for EhFrameOffset<T>`

- <span id="ehframeoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for EhFrameOffset<T>`

- <span id="ehframeoffset-partialeq-eq"></span>`fn eq(&self, other: &EhFrameOffset<T>) -> bool` — [`EhFrameOffset`](#ehframeoffset)

##### `impl<T> StructuralPartialEq for EhFrameOffset<T>`

##### `impl<T> UnwindOffset for crate::common::EhFrameOffset<T>`

- <span id="cratecommonehframeoffset-unwindoffset-into"></span>`fn into(self) -> T`

### `UnitSectionOffset<T>`

```rust
struct UnitSectionOffset<T>(T);
```

An offset into the `.debug_info` or `.debug_types` sections.

This type does not store which section the offset applies to. You will need to either
determine that from the context of its use, or store a [`SectionId`](#sectionid) along with it.

#### Implementations

- <span id="cratecommonunitsectionoffset-to-unit-offset"></span>`fn to_unit_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](read/index.md#unitheader), [`UnitOffset`](#unitoffset)

  Convert an offset to be relative to the start of the given unit,

  instead of relative to the start of the section.

  

  Returns `None` if the offset is not in bounds for the unit's entries.

- <span id="cratecommonunitsectionoffset-to-debug-info-offset"></span>`fn to_debug_info_offset<R>(&self, unit: &UnitHeader<R>) -> Option<DebugInfoOffset<T>>` — [`UnitHeader`](read/index.md#unitheader), [`DebugInfoOffset`](#debuginfooffset)

  Convert an offset to be relative to the start of the `.debug_info` section,

  instead of relative to the start of the section for the given unit.

  

  Returns `None` if the unit is not within the `.debug_info` section.

- <span id="cratecommonunitsectionoffset-to-debug-types-offset"></span>`fn to_debug_types_offset<R>(&self, unit: &UnitHeader<R>) -> Option<DebugTypesOffset<T>>` — [`UnitHeader`](read/index.md#unitheader), [`DebugTypesOffset`](#debugtypesoffset)

  Convert an offset to be relative to the start of the `.debug_types` section,

  instead of relative to the start of the section for the given unit.

  

  Returns `None` if the unit is not within the `.debug_types` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for UnitSectionOffset<T>`

- <span id="unitsectionoffset-clone"></span>`fn clone(&self) -> UnitSectionOffset<T>` — [`UnitSectionOffset`](#unitsectionoffset)

##### `impl<T: marker::Copy> Copy for UnitSectionOffset<T>`

##### `impl<T: fmt::Debug> Debug for UnitSectionOffset<T>`

- <span id="unitsectionoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for UnitSectionOffset<T>`

##### `impl<T: hash::Hash> Hash for UnitSectionOffset<T>`

- <span id="unitsectionoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for UnitSectionOffset<T>`

- <span id="unitsectionoffset-ord-cmp"></span>`fn cmp(&self, other: &UnitSectionOffset<T>) -> cmp::Ordering` — [`UnitSectionOffset`](#unitsectionoffset)

##### `impl<T: cmp::PartialEq> PartialEq for UnitSectionOffset<T>`

- <span id="unitsectionoffset-partialeq-eq"></span>`fn eq(&self, other: &UnitSectionOffset<T>) -> bool` — [`UnitSectionOffset`](#unitsectionoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for UnitSectionOffset<T>`

- <span id="unitsectionoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnitSectionOffset<T>) -> option::Option<cmp::Ordering>` — [`UnitSectionOffset`](#unitsectionoffset)

##### `impl<T> StructuralPartialEq for UnitSectionOffset<T>`

### `DwoId`

```rust
struct DwoId(u64);
```

An optionally-provided implementation-defined compilation unit ID to enable
split DWARF and linking a split compilation unit back together.

#### Trait Implementations

##### `impl Clone for DwoId`

- <span id="dwoid-clone"></span>`fn clone(&self) -> DwoId` — [`DwoId`](#dwoid)

##### `impl Copy for DwoId`

##### `impl Debug for DwoId`

- <span id="dwoid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DwoId`

##### `impl Hash for DwoId`

- <span id="dwoid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DwoId`

- <span id="dwoid-partialeq-eq"></span>`fn eq(&self, other: &DwoId) -> bool` — [`DwoId`](#dwoid)

##### `impl StructuralPartialEq for DwoId`

### `Arm`

```rust
struct Arm;
```

ARM architecture specific definitions.

See [DWARF for the ARM Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf32/aadwarf32.rst).

#### Implementations

- <span id="arm-const-r0"></span>`const R0: Register`

- <span id="arm-const-r1"></span>`const R1: Register`

- <span id="arm-const-r2"></span>`const R2: Register`

- <span id="arm-const-r3"></span>`const R3: Register`

- <span id="arm-const-r4"></span>`const R4: Register`

- <span id="arm-const-r5"></span>`const R5: Register`

- <span id="arm-const-r6"></span>`const R6: Register`

- <span id="arm-const-r7"></span>`const R7: Register`

- <span id="arm-const-r8"></span>`const R8: Register`

- <span id="arm-const-r9"></span>`const R9: Register`

- <span id="arm-const-r10"></span>`const R10: Register`

- <span id="arm-const-r11"></span>`const R11: Register`

- <span id="arm-const-r12"></span>`const R12: Register`

- <span id="arm-const-r13"></span>`const R13: Register`

- <span id="arm-const-r14"></span>`const R14: Register`

- <span id="arm-const-r15"></span>`const R15: Register`

- <span id="arm-const-wcgr0"></span>`const WCGR0: Register`

- <span id="arm-const-wcgr1"></span>`const WCGR1: Register`

- <span id="arm-const-wcgr2"></span>`const WCGR2: Register`

- <span id="arm-const-wcgr3"></span>`const WCGR3: Register`

- <span id="arm-const-wcgr4"></span>`const WCGR4: Register`

- <span id="arm-const-wcgr5"></span>`const WCGR5: Register`

- <span id="arm-const-wcgr6"></span>`const WCGR6: Register`

- <span id="arm-const-wcgr7"></span>`const WCGR7: Register`

- <span id="arm-const-wr0"></span>`const WR0: Register`

- <span id="arm-const-wr1"></span>`const WR1: Register`

- <span id="arm-const-wr2"></span>`const WR2: Register`

- <span id="arm-const-wr3"></span>`const WR3: Register`

- <span id="arm-const-wr4"></span>`const WR4: Register`

- <span id="arm-const-wr5"></span>`const WR5: Register`

- <span id="arm-const-wr6"></span>`const WR6: Register`

- <span id="arm-const-wr7"></span>`const WR7: Register`

- <span id="arm-const-wr8"></span>`const WR8: Register`

- <span id="arm-const-wr9"></span>`const WR9: Register`

- <span id="arm-const-wr10"></span>`const WR10: Register`

- <span id="arm-const-wr11"></span>`const WR11: Register`

- <span id="arm-const-wr12"></span>`const WR12: Register`

- <span id="arm-const-wr13"></span>`const WR13: Register`

- <span id="arm-const-wr14"></span>`const WR14: Register`

- <span id="arm-const-wr15"></span>`const WR15: Register`

- <span id="arm-const-spsr"></span>`const SPSR: Register`

- <span id="arm-const-spsr-fiq"></span>`const SPSR_FIQ: Register`

- <span id="arm-const-spsr-irq"></span>`const SPSR_IRQ: Register`

- <span id="arm-const-spsr-abt"></span>`const SPSR_ABT: Register`

- <span id="arm-const-spsr-und"></span>`const SPSR_UND: Register`

- <span id="arm-const-spsr-svc"></span>`const SPSR_SVC: Register`

- <span id="arm-const-ra-auth-code"></span>`const RA_AUTH_CODE: Register`

- <span id="arm-const-r8-usr"></span>`const R8_USR: Register`

- <span id="arm-const-r9-usr"></span>`const R9_USR: Register`

- <span id="arm-const-r10-usr"></span>`const R10_USR: Register`

- <span id="arm-const-r11-usr"></span>`const R11_USR: Register`

- <span id="arm-const-r12-usr"></span>`const R12_USR: Register`

- <span id="arm-const-r13-usr"></span>`const R13_USR: Register`

- <span id="arm-const-r14-usr"></span>`const R14_USR: Register`

- <span id="arm-const-r8-fiq"></span>`const R8_FIQ: Register`

- <span id="arm-const-r9-fiq"></span>`const R9_FIQ: Register`

- <span id="arm-const-r10-fiq"></span>`const R10_FIQ: Register`

- <span id="arm-const-r11-fiq"></span>`const R11_FIQ: Register`

- <span id="arm-const-r12-fiq"></span>`const R12_FIQ: Register`

- <span id="arm-const-r13-fiq"></span>`const R13_FIQ: Register`

- <span id="arm-const-r14-fiq"></span>`const R14_FIQ: Register`

- <span id="arm-const-r13-irq"></span>`const R13_IRQ: Register`

- <span id="arm-const-r14-irq"></span>`const R14_IRQ: Register`

- <span id="arm-const-r13-abt"></span>`const R13_ABT: Register`

- <span id="arm-const-r14-abt"></span>`const R14_ABT: Register`

- <span id="arm-const-r13-und"></span>`const R13_UND: Register`

- <span id="arm-const-r14-und"></span>`const R14_UND: Register`

- <span id="arm-const-r13-svc"></span>`const R13_SVC: Register`

- <span id="arm-const-r14-svc"></span>`const R14_SVC: Register`

- <span id="arm-const-wc0"></span>`const WC0: Register`

- <span id="arm-const-wc1"></span>`const WC1: Register`

- <span id="arm-const-wc2"></span>`const WC2: Register`

- <span id="arm-const-wc3"></span>`const WC3: Register`

- <span id="arm-const-wc4"></span>`const WC4: Register`

- <span id="arm-const-wc5"></span>`const WC5: Register`

- <span id="arm-const-wc6"></span>`const WC6: Register`

- <span id="arm-const-wc7"></span>`const WC7: Register`

- <span id="arm-const-d0"></span>`const D0: Register`

- <span id="arm-const-d1"></span>`const D1: Register`

- <span id="arm-const-d2"></span>`const D2: Register`

- <span id="arm-const-d3"></span>`const D3: Register`

- <span id="arm-const-d4"></span>`const D4: Register`

- <span id="arm-const-d5"></span>`const D5: Register`

- <span id="arm-const-d6"></span>`const D6: Register`

- <span id="arm-const-d7"></span>`const D7: Register`

- <span id="arm-const-d8"></span>`const D8: Register`

- <span id="arm-const-d9"></span>`const D9: Register`

- <span id="arm-const-d10"></span>`const D10: Register`

- <span id="arm-const-d11"></span>`const D11: Register`

- <span id="arm-const-d12"></span>`const D12: Register`

- <span id="arm-const-d13"></span>`const D13: Register`

- <span id="arm-const-d14"></span>`const D14: Register`

- <span id="arm-const-d15"></span>`const D15: Register`

- <span id="arm-const-d16"></span>`const D16: Register`

- <span id="arm-const-d17"></span>`const D17: Register`

- <span id="arm-const-d18"></span>`const D18: Register`

- <span id="arm-const-d19"></span>`const D19: Register`

- <span id="arm-const-d20"></span>`const D20: Register`

- <span id="arm-const-d21"></span>`const D21: Register`

- <span id="arm-const-d22"></span>`const D22: Register`

- <span id="arm-const-d23"></span>`const D23: Register`

- <span id="arm-const-d24"></span>`const D24: Register`

- <span id="arm-const-d25"></span>`const D25: Register`

- <span id="arm-const-d26"></span>`const D26: Register`

- <span id="arm-const-d27"></span>`const D27: Register`

- <span id="arm-const-d28"></span>`const D28: Register`

- <span id="arm-const-d29"></span>`const D29: Register`

- <span id="arm-const-d30"></span>`const D30: Register`

- <span id="arm-const-d31"></span>`const D31: Register`

- <span id="arm-const-tpidruro"></span>`const TPIDRURO: Register`

- <span id="arm-const-tpidrurw"></span>`const TPIDRURW: Register`

- <span id="arm-const-tpidpr"></span>`const TPIDPR: Register`

- <span id="arm-const-htpidpr"></span>`const HTPIDPR: Register`

- <span id="arm-const-sp"></span>`const SP: Register`

- <span id="arm-const-lr"></span>`const LR: Register`

- <span id="arm-const-pc"></span>`const PC: Register`

- <span id="arm-const-acc0"></span>`const ACC0: Register`

- <span id="arm-const-acc1"></span>`const ACC1: Register`

- <span id="arm-const-acc2"></span>`const ACC2: Register`

- <span id="arm-const-acc3"></span>`const ACC3: Register`

- <span id="arm-const-acc4"></span>`const ACC4: Register`

- <span id="arm-const-acc5"></span>`const ACC5: Register`

- <span id="arm-const-acc6"></span>`const ACC6: Register`

- <span id="arm-const-acc7"></span>`const ACC7: Register`

- <span id="arm-const-s0"></span>`const S0: Register`

- <span id="arm-const-s1"></span>`const S1: Register`

- <span id="arm-const-s2"></span>`const S2: Register`

- <span id="arm-const-s3"></span>`const S3: Register`

- <span id="arm-const-s4"></span>`const S4: Register`

- <span id="arm-const-s5"></span>`const S5: Register`

- <span id="arm-const-s6"></span>`const S6: Register`

- <span id="arm-const-s7"></span>`const S7: Register`

- <span id="arm-const-s8"></span>`const S8: Register`

- <span id="arm-const-s9"></span>`const S9: Register`

- <span id="arm-const-s10"></span>`const S10: Register`

- <span id="arm-const-s11"></span>`const S11: Register`

- <span id="arm-const-s12"></span>`const S12: Register`

- <span id="arm-const-s13"></span>`const S13: Register`

- <span id="arm-const-s14"></span>`const S14: Register`

- <span id="arm-const-s15"></span>`const S15: Register`

- <span id="arm-const-s16"></span>`const S16: Register`

- <span id="arm-const-s17"></span>`const S17: Register`

- <span id="arm-const-s18"></span>`const S18: Register`

- <span id="arm-const-s19"></span>`const S19: Register`

- <span id="arm-const-s20"></span>`const S20: Register`

- <span id="arm-const-s21"></span>`const S21: Register`

- <span id="arm-const-s22"></span>`const S22: Register`

- <span id="arm-const-s23"></span>`const S23: Register`

- <span id="arm-const-s24"></span>`const S24: Register`

- <span id="arm-const-s25"></span>`const S25: Register`

- <span id="arm-const-s26"></span>`const S26: Register`

- <span id="arm-const-s27"></span>`const S27: Register`

- <span id="arm-const-s28"></span>`const S28: Register`

- <span id="arm-const-s29"></span>`const S29: Register`

- <span id="arm-const-s30"></span>`const S30: Register`

- <span id="arm-const-s31"></span>`const S31: Register`

#### Trait Implementations

##### `impl Clone for Arm`

- <span id="arm-clone"></span>`fn clone(&self) -> Arm` — [`Arm`](#arm)

##### `impl Copy for Arm`

##### `impl Debug for Arm`

- <span id="arm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AArch64`

```rust
struct AArch64;
```

ARM 64-bit (AArch64) architecture specific definitions.

See [DWARF for the ARM 64-bit Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf64/aadwarf64.rst).

#### Implementations

- <span id="aarch64-const-x0"></span>`const X0: Register`

- <span id="aarch64-const-x1"></span>`const X1: Register`

- <span id="aarch64-const-x2"></span>`const X2: Register`

- <span id="aarch64-const-x3"></span>`const X3: Register`

- <span id="aarch64-const-x4"></span>`const X4: Register`

- <span id="aarch64-const-x5"></span>`const X5: Register`

- <span id="aarch64-const-x6"></span>`const X6: Register`

- <span id="aarch64-const-x7"></span>`const X7: Register`

- <span id="aarch64-const-x8"></span>`const X8: Register`

- <span id="aarch64-const-x9"></span>`const X9: Register`

- <span id="aarch64-const-x10"></span>`const X10: Register`

- <span id="aarch64-const-x11"></span>`const X11: Register`

- <span id="aarch64-const-x12"></span>`const X12: Register`

- <span id="aarch64-const-x13"></span>`const X13: Register`

- <span id="aarch64-const-x14"></span>`const X14: Register`

- <span id="aarch64-const-x15"></span>`const X15: Register`

- <span id="aarch64-const-x16"></span>`const X16: Register`

- <span id="aarch64-const-x17"></span>`const X17: Register`

- <span id="aarch64-const-x18"></span>`const X18: Register`

- <span id="aarch64-const-x19"></span>`const X19: Register`

- <span id="aarch64-const-x20"></span>`const X20: Register`

- <span id="aarch64-const-x21"></span>`const X21: Register`

- <span id="aarch64-const-x22"></span>`const X22: Register`

- <span id="aarch64-const-x23"></span>`const X23: Register`

- <span id="aarch64-const-x24"></span>`const X24: Register`

- <span id="aarch64-const-x25"></span>`const X25: Register`

- <span id="aarch64-const-x26"></span>`const X26: Register`

- <span id="aarch64-const-x27"></span>`const X27: Register`

- <span id="aarch64-const-x28"></span>`const X28: Register`

- <span id="aarch64-const-x29"></span>`const X29: Register`

- <span id="aarch64-const-x30"></span>`const X30: Register`

- <span id="aarch64-const-sp"></span>`const SP: Register`

- <span id="aarch64-const-pc"></span>`const PC: Register`

- <span id="aarch64-const-elr-mode"></span>`const ELR_MODE: Register`

- <span id="aarch64-const-ra-sign-state"></span>`const RA_SIGN_STATE: Register`

- <span id="aarch64-const-tpidrro-el0"></span>`const TPIDRRO_EL0: Register`

- <span id="aarch64-const-tpidr-el0"></span>`const TPIDR_EL0: Register`

- <span id="aarch64-const-tpidr-el1"></span>`const TPIDR_EL1: Register`

- <span id="aarch64-const-tpidr-el2"></span>`const TPIDR_EL2: Register`

- <span id="aarch64-const-tpidr-el3"></span>`const TPIDR_EL3: Register`

- <span id="aarch64-const-vg"></span>`const VG: Register`

- <span id="aarch64-const-ffr"></span>`const FFR: Register`

- <span id="aarch64-const-p0"></span>`const P0: Register`

- <span id="aarch64-const-p1"></span>`const P1: Register`

- <span id="aarch64-const-p2"></span>`const P2: Register`

- <span id="aarch64-const-p3"></span>`const P3: Register`

- <span id="aarch64-const-p4"></span>`const P4: Register`

- <span id="aarch64-const-p5"></span>`const P5: Register`

- <span id="aarch64-const-p6"></span>`const P6: Register`

- <span id="aarch64-const-p7"></span>`const P7: Register`

- <span id="aarch64-const-p8"></span>`const P8: Register`

- <span id="aarch64-const-p9"></span>`const P9: Register`

- <span id="aarch64-const-p10"></span>`const P10: Register`

- <span id="aarch64-const-p11"></span>`const P11: Register`

- <span id="aarch64-const-p12"></span>`const P12: Register`

- <span id="aarch64-const-p13"></span>`const P13: Register`

- <span id="aarch64-const-p14"></span>`const P14: Register`

- <span id="aarch64-const-p15"></span>`const P15: Register`

- <span id="aarch64-const-v0"></span>`const V0: Register`

- <span id="aarch64-const-v1"></span>`const V1: Register`

- <span id="aarch64-const-v2"></span>`const V2: Register`

- <span id="aarch64-const-v3"></span>`const V3: Register`

- <span id="aarch64-const-v4"></span>`const V4: Register`

- <span id="aarch64-const-v5"></span>`const V5: Register`

- <span id="aarch64-const-v6"></span>`const V6: Register`

- <span id="aarch64-const-v7"></span>`const V7: Register`

- <span id="aarch64-const-v8"></span>`const V8: Register`

- <span id="aarch64-const-v9"></span>`const V9: Register`

- <span id="aarch64-const-v10"></span>`const V10: Register`

- <span id="aarch64-const-v11"></span>`const V11: Register`

- <span id="aarch64-const-v12"></span>`const V12: Register`

- <span id="aarch64-const-v13"></span>`const V13: Register`

- <span id="aarch64-const-v14"></span>`const V14: Register`

- <span id="aarch64-const-v15"></span>`const V15: Register`

- <span id="aarch64-const-v16"></span>`const V16: Register`

- <span id="aarch64-const-v17"></span>`const V17: Register`

- <span id="aarch64-const-v18"></span>`const V18: Register`

- <span id="aarch64-const-v19"></span>`const V19: Register`

- <span id="aarch64-const-v20"></span>`const V20: Register`

- <span id="aarch64-const-v21"></span>`const V21: Register`

- <span id="aarch64-const-v22"></span>`const V22: Register`

- <span id="aarch64-const-v23"></span>`const V23: Register`

- <span id="aarch64-const-v24"></span>`const V24: Register`

- <span id="aarch64-const-v25"></span>`const V25: Register`

- <span id="aarch64-const-v26"></span>`const V26: Register`

- <span id="aarch64-const-v27"></span>`const V27: Register`

- <span id="aarch64-const-v28"></span>`const V28: Register`

- <span id="aarch64-const-v29"></span>`const V29: Register`

- <span id="aarch64-const-v30"></span>`const V30: Register`

- <span id="aarch64-const-v31"></span>`const V31: Register`

- <span id="aarch64-const-z0"></span>`const Z0: Register`

- <span id="aarch64-const-z1"></span>`const Z1: Register`

- <span id="aarch64-const-z2"></span>`const Z2: Register`

- <span id="aarch64-const-z3"></span>`const Z3: Register`

- <span id="aarch64-const-z4"></span>`const Z4: Register`

- <span id="aarch64-const-z5"></span>`const Z5: Register`

- <span id="aarch64-const-z6"></span>`const Z6: Register`

- <span id="aarch64-const-z7"></span>`const Z7: Register`

- <span id="aarch64-const-z8"></span>`const Z8: Register`

- <span id="aarch64-const-z9"></span>`const Z9: Register`

- <span id="aarch64-const-z10"></span>`const Z10: Register`

- <span id="aarch64-const-z11"></span>`const Z11: Register`

- <span id="aarch64-const-z12"></span>`const Z12: Register`

- <span id="aarch64-const-z13"></span>`const Z13: Register`

- <span id="aarch64-const-z14"></span>`const Z14: Register`

- <span id="aarch64-const-z15"></span>`const Z15: Register`

- <span id="aarch64-const-z16"></span>`const Z16: Register`

- <span id="aarch64-const-z17"></span>`const Z17: Register`

- <span id="aarch64-const-z18"></span>`const Z18: Register`

- <span id="aarch64-const-z19"></span>`const Z19: Register`

- <span id="aarch64-const-z20"></span>`const Z20: Register`

- <span id="aarch64-const-z21"></span>`const Z21: Register`

- <span id="aarch64-const-z22"></span>`const Z22: Register`

- <span id="aarch64-const-z23"></span>`const Z23: Register`

- <span id="aarch64-const-z24"></span>`const Z24: Register`

- <span id="aarch64-const-z25"></span>`const Z25: Register`

- <span id="aarch64-const-z26"></span>`const Z26: Register`

- <span id="aarch64-const-z27"></span>`const Z27: Register`

- <span id="aarch64-const-z28"></span>`const Z28: Register`

- <span id="aarch64-const-z29"></span>`const Z29: Register`

- <span id="aarch64-const-z30"></span>`const Z30: Register`

- <span id="aarch64-const-z31"></span>`const Z31: Register`

#### Trait Implementations

##### `impl Clone for AArch64`

- <span id="aarch64-clone"></span>`fn clone(&self) -> AArch64` — [`AArch64`](#aarch64)

##### `impl Copy for AArch64`

##### `impl Debug for AArch64`

- <span id="aarch64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LoongArch`

```rust
struct LoongArch;
```

LoongArch architecture specific definitions.

See [LoongArch ELF psABI specification](https://loongson.github.io/LoongArch-Documentation/LoongArch-ELF-ABI-EN.html).

#### Implementations

- <span id="loongarch-const-r0"></span>`const R0: Register`

- <span id="loongarch-const-r1"></span>`const R1: Register`

- <span id="loongarch-const-r2"></span>`const R2: Register`

- <span id="loongarch-const-r3"></span>`const R3: Register`

- <span id="loongarch-const-r4"></span>`const R4: Register`

- <span id="loongarch-const-r5"></span>`const R5: Register`

- <span id="loongarch-const-r6"></span>`const R6: Register`

- <span id="loongarch-const-r7"></span>`const R7: Register`

- <span id="loongarch-const-r8"></span>`const R8: Register`

- <span id="loongarch-const-r9"></span>`const R9: Register`

- <span id="loongarch-const-r10"></span>`const R10: Register`

- <span id="loongarch-const-r11"></span>`const R11: Register`

- <span id="loongarch-const-r12"></span>`const R12: Register`

- <span id="loongarch-const-r13"></span>`const R13: Register`

- <span id="loongarch-const-r14"></span>`const R14: Register`

- <span id="loongarch-const-r15"></span>`const R15: Register`

- <span id="loongarch-const-r16"></span>`const R16: Register`

- <span id="loongarch-const-r17"></span>`const R17: Register`

- <span id="loongarch-const-r18"></span>`const R18: Register`

- <span id="loongarch-const-r19"></span>`const R19: Register`

- <span id="loongarch-const-r20"></span>`const R20: Register`

- <span id="loongarch-const-r21"></span>`const R21: Register`

- <span id="loongarch-const-r22"></span>`const R22: Register`

- <span id="loongarch-const-r23"></span>`const R23: Register`

- <span id="loongarch-const-r24"></span>`const R24: Register`

- <span id="loongarch-const-r25"></span>`const R25: Register`

- <span id="loongarch-const-r26"></span>`const R26: Register`

- <span id="loongarch-const-r27"></span>`const R27: Register`

- <span id="loongarch-const-r28"></span>`const R28: Register`

- <span id="loongarch-const-r29"></span>`const R29: Register`

- <span id="loongarch-const-r30"></span>`const R30: Register`

- <span id="loongarch-const-r31"></span>`const R31: Register`

- <span id="loongarch-const-f0"></span>`const F0: Register`

- <span id="loongarch-const-f1"></span>`const F1: Register`

- <span id="loongarch-const-f2"></span>`const F2: Register`

- <span id="loongarch-const-f3"></span>`const F3: Register`

- <span id="loongarch-const-f4"></span>`const F4: Register`

- <span id="loongarch-const-f5"></span>`const F5: Register`

- <span id="loongarch-const-f6"></span>`const F6: Register`

- <span id="loongarch-const-f7"></span>`const F7: Register`

- <span id="loongarch-const-f8"></span>`const F8: Register`

- <span id="loongarch-const-f9"></span>`const F9: Register`

- <span id="loongarch-const-f10"></span>`const F10: Register`

- <span id="loongarch-const-f11"></span>`const F11: Register`

- <span id="loongarch-const-f12"></span>`const F12: Register`

- <span id="loongarch-const-f13"></span>`const F13: Register`

- <span id="loongarch-const-f14"></span>`const F14: Register`

- <span id="loongarch-const-f15"></span>`const F15: Register`

- <span id="loongarch-const-f16"></span>`const F16: Register`

- <span id="loongarch-const-f17"></span>`const F17: Register`

- <span id="loongarch-const-f18"></span>`const F18: Register`

- <span id="loongarch-const-f19"></span>`const F19: Register`

- <span id="loongarch-const-f20"></span>`const F20: Register`

- <span id="loongarch-const-f21"></span>`const F21: Register`

- <span id="loongarch-const-f22"></span>`const F22: Register`

- <span id="loongarch-const-f23"></span>`const F23: Register`

- <span id="loongarch-const-f24"></span>`const F24: Register`

- <span id="loongarch-const-f25"></span>`const F25: Register`

- <span id="loongarch-const-f26"></span>`const F26: Register`

- <span id="loongarch-const-f27"></span>`const F27: Register`

- <span id="loongarch-const-f28"></span>`const F28: Register`

- <span id="loongarch-const-f29"></span>`const F29: Register`

- <span id="loongarch-const-f30"></span>`const F30: Register`

- <span id="loongarch-const-f31"></span>`const F31: Register`

- <span id="loongarch-const-fcc0"></span>`const FCC0: Register`

- <span id="loongarch-const-fcc1"></span>`const FCC1: Register`

- <span id="loongarch-const-fcc2"></span>`const FCC2: Register`

- <span id="loongarch-const-fcc3"></span>`const FCC3: Register`

- <span id="loongarch-const-fcc4"></span>`const FCC4: Register`

- <span id="loongarch-const-fcc5"></span>`const FCC5: Register`

- <span id="loongarch-const-fcc6"></span>`const FCC6: Register`

- <span id="loongarch-const-fcc7"></span>`const FCC7: Register`

- <span id="loongarch-const-zero"></span>`const ZERO: Register`

- <span id="loongarch-const-ra"></span>`const RA: Register`

- <span id="loongarch-const-tp"></span>`const TP: Register`

- <span id="loongarch-const-sp"></span>`const SP: Register`

- <span id="loongarch-const-a0"></span>`const A0: Register`

- <span id="loongarch-const-a1"></span>`const A1: Register`

- <span id="loongarch-const-a2"></span>`const A2: Register`

- <span id="loongarch-const-a3"></span>`const A3: Register`

- <span id="loongarch-const-a4"></span>`const A4: Register`

- <span id="loongarch-const-a5"></span>`const A5: Register`

- <span id="loongarch-const-a6"></span>`const A6: Register`

- <span id="loongarch-const-a7"></span>`const A7: Register`

- <span id="loongarch-const-t0"></span>`const T0: Register`

- <span id="loongarch-const-t1"></span>`const T1: Register`

- <span id="loongarch-const-t2"></span>`const T2: Register`

- <span id="loongarch-const-t3"></span>`const T3: Register`

- <span id="loongarch-const-t4"></span>`const T4: Register`

- <span id="loongarch-const-t5"></span>`const T5: Register`

- <span id="loongarch-const-t6"></span>`const T6: Register`

- <span id="loongarch-const-t7"></span>`const T7: Register`

- <span id="loongarch-const-t8"></span>`const T8: Register`

- <span id="loongarch-const-fp"></span>`const FP: Register`

- <span id="loongarch-const-s0"></span>`const S0: Register`

- <span id="loongarch-const-s1"></span>`const S1: Register`

- <span id="loongarch-const-s2"></span>`const S2: Register`

- <span id="loongarch-const-s3"></span>`const S3: Register`

- <span id="loongarch-const-s4"></span>`const S4: Register`

- <span id="loongarch-const-s5"></span>`const S5: Register`

- <span id="loongarch-const-s6"></span>`const S6: Register`

- <span id="loongarch-const-s7"></span>`const S7: Register`

- <span id="loongarch-const-s8"></span>`const S8: Register`

- <span id="loongarch-const-fa0"></span>`const FA0: Register`

- <span id="loongarch-const-fa1"></span>`const FA1: Register`

- <span id="loongarch-const-fa2"></span>`const FA2: Register`

- <span id="loongarch-const-fa3"></span>`const FA3: Register`

- <span id="loongarch-const-fa4"></span>`const FA4: Register`

- <span id="loongarch-const-fa5"></span>`const FA5: Register`

- <span id="loongarch-const-fa6"></span>`const FA6: Register`

- <span id="loongarch-const-fa7"></span>`const FA7: Register`

- <span id="loongarch-const-ft0"></span>`const FT0: Register`

- <span id="loongarch-const-ft1"></span>`const FT1: Register`

- <span id="loongarch-const-ft2"></span>`const FT2: Register`

- <span id="loongarch-const-ft3"></span>`const FT3: Register`

- <span id="loongarch-const-ft4"></span>`const FT4: Register`

- <span id="loongarch-const-ft5"></span>`const FT5: Register`

- <span id="loongarch-const-ft6"></span>`const FT6: Register`

- <span id="loongarch-const-ft7"></span>`const FT7: Register`

- <span id="loongarch-const-ft8"></span>`const FT8: Register`

- <span id="loongarch-const-ft9"></span>`const FT9: Register`

- <span id="loongarch-const-ft10"></span>`const FT10: Register`

- <span id="loongarch-const-ft11"></span>`const FT11: Register`

- <span id="loongarch-const-ft12"></span>`const FT12: Register`

- <span id="loongarch-const-ft13"></span>`const FT13: Register`

- <span id="loongarch-const-ft14"></span>`const FT14: Register`

- <span id="loongarch-const-ft15"></span>`const FT15: Register`

- <span id="loongarch-const-fs0"></span>`const FS0: Register`

- <span id="loongarch-const-fs1"></span>`const FS1: Register`

- <span id="loongarch-const-fs2"></span>`const FS2: Register`

- <span id="loongarch-const-fs3"></span>`const FS3: Register`

- <span id="loongarch-const-fs4"></span>`const FS4: Register`

- <span id="loongarch-const-fs5"></span>`const FS5: Register`

- <span id="loongarch-const-fs6"></span>`const FS6: Register`

- <span id="loongarch-const-fs7"></span>`const FS7: Register`

#### Trait Implementations

##### `impl Clone for LoongArch`

- <span id="loongarch-clone"></span>`fn clone(&self) -> LoongArch` — [`LoongArch`](#loongarch)

##### `impl Copy for LoongArch`

##### `impl Debug for LoongArch`

- <span id="loongarch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MIPS`

```rust
struct MIPS;
```

MIPS architecture specific definitions.

See [MIPS Details](https://en.wikibooks.org/wiki/MIPS_Assembly/MIPS_Details).

#### Implementations

- <span id="mips-const-r0"></span>`const R0: Register`

- <span id="mips-const-r1"></span>`const R1: Register`

- <span id="mips-const-r2"></span>`const R2: Register`

- <span id="mips-const-r3"></span>`const R3: Register`

- <span id="mips-const-r4"></span>`const R4: Register`

- <span id="mips-const-r5"></span>`const R5: Register`

- <span id="mips-const-r6"></span>`const R6: Register`

- <span id="mips-const-r7"></span>`const R7: Register`

- <span id="mips-const-r8"></span>`const R8: Register`

- <span id="mips-const-r9"></span>`const R9: Register`

- <span id="mips-const-r10"></span>`const R10: Register`

- <span id="mips-const-r11"></span>`const R11: Register`

- <span id="mips-const-r12"></span>`const R12: Register`

- <span id="mips-const-r13"></span>`const R13: Register`

- <span id="mips-const-r14"></span>`const R14: Register`

- <span id="mips-const-r15"></span>`const R15: Register`

- <span id="mips-const-r16"></span>`const R16: Register`

- <span id="mips-const-r17"></span>`const R17: Register`

- <span id="mips-const-r18"></span>`const R18: Register`

- <span id="mips-const-r19"></span>`const R19: Register`

- <span id="mips-const-r20"></span>`const R20: Register`

- <span id="mips-const-r21"></span>`const R21: Register`

- <span id="mips-const-r22"></span>`const R22: Register`

- <span id="mips-const-r23"></span>`const R23: Register`

- <span id="mips-const-r24"></span>`const R24: Register`

- <span id="mips-const-r25"></span>`const R25: Register`

- <span id="mips-const-r26"></span>`const R26: Register`

- <span id="mips-const-r27"></span>`const R27: Register`

- <span id="mips-const-r28"></span>`const R28: Register`

- <span id="mips-const-r29"></span>`const R29: Register`

- <span id="mips-const-r30"></span>`const R30: Register`

- <span id="mips-const-r31"></span>`const R31: Register`

- <span id="mips-const-f0"></span>`const F0: Register`

- <span id="mips-const-f1"></span>`const F1: Register`

- <span id="mips-const-f2"></span>`const F2: Register`

- <span id="mips-const-f3"></span>`const F3: Register`

- <span id="mips-const-f4"></span>`const F4: Register`

- <span id="mips-const-f5"></span>`const F5: Register`

- <span id="mips-const-f6"></span>`const F6: Register`

- <span id="mips-const-f7"></span>`const F7: Register`

- <span id="mips-const-f8"></span>`const F8: Register`

- <span id="mips-const-f9"></span>`const F9: Register`

- <span id="mips-const-f10"></span>`const F10: Register`

- <span id="mips-const-f11"></span>`const F11: Register`

- <span id="mips-const-f12"></span>`const F12: Register`

- <span id="mips-const-f13"></span>`const F13: Register`

- <span id="mips-const-f14"></span>`const F14: Register`

- <span id="mips-const-f15"></span>`const F15: Register`

- <span id="mips-const-f16"></span>`const F16: Register`

- <span id="mips-const-f17"></span>`const F17: Register`

- <span id="mips-const-f18"></span>`const F18: Register`

- <span id="mips-const-f19"></span>`const F19: Register`

- <span id="mips-const-f20"></span>`const F20: Register`

- <span id="mips-const-f21"></span>`const F21: Register`

- <span id="mips-const-f22"></span>`const F22: Register`

- <span id="mips-const-f23"></span>`const F23: Register`

- <span id="mips-const-f24"></span>`const F24: Register`

- <span id="mips-const-f25"></span>`const F25: Register`

- <span id="mips-const-f26"></span>`const F26: Register`

- <span id="mips-const-f27"></span>`const F27: Register`

- <span id="mips-const-f28"></span>`const F28: Register`

- <span id="mips-const-f29"></span>`const F29: Register`

- <span id="mips-const-f30"></span>`const F30: Register`

- <span id="mips-const-f31"></span>`const F31: Register`

- <span id="mips-const-hi"></span>`const HI: Register`

- <span id="mips-const-lo"></span>`const LO: Register`

- <span id="mips-const-zero"></span>`const ZERO: Register`

- <span id="mips-const-at"></span>`const AT: Register`

- <span id="mips-const-v0"></span>`const V0: Register`

- <span id="mips-const-v1"></span>`const V1: Register`

- <span id="mips-const-a0"></span>`const A0: Register`

- <span id="mips-const-a1"></span>`const A1: Register`

- <span id="mips-const-a2"></span>`const A2: Register`

- <span id="mips-const-a3"></span>`const A3: Register`

- <span id="mips-const-t0"></span>`const T0: Register`

- <span id="mips-const-t1"></span>`const T1: Register`

- <span id="mips-const-t2"></span>`const T2: Register`

- <span id="mips-const-t3"></span>`const T3: Register`

- <span id="mips-const-t4"></span>`const T4: Register`

- <span id="mips-const-t5"></span>`const T5: Register`

- <span id="mips-const-t6"></span>`const T6: Register`

- <span id="mips-const-t7"></span>`const T7: Register`

- <span id="mips-const-s0"></span>`const S0: Register`

- <span id="mips-const-s1"></span>`const S1: Register`

- <span id="mips-const-s2"></span>`const S2: Register`

- <span id="mips-const-s3"></span>`const S3: Register`

- <span id="mips-const-s4"></span>`const S4: Register`

- <span id="mips-const-s5"></span>`const S5: Register`

- <span id="mips-const-s6"></span>`const S6: Register`

- <span id="mips-const-s7"></span>`const S7: Register`

- <span id="mips-const-t8"></span>`const T8: Register`

- <span id="mips-const-t9"></span>`const T9: Register`

- <span id="mips-const-k0"></span>`const K0: Register`

- <span id="mips-const-k1"></span>`const K1: Register`

- <span id="mips-const-gp"></span>`const GP: Register`

- <span id="mips-const-sp"></span>`const SP: Register`

- <span id="mips-const-fp"></span>`const FP: Register`

- <span id="mips-const-ra"></span>`const RA: Register`

- <span id="mips-const-s8"></span>`const S8: Register`

#### Trait Implementations

##### `impl Clone for MIPS`

- <span id="mips-clone"></span>`fn clone(&self) -> MIPS` — [`MIPS`](#mips)

##### `impl Copy for MIPS`

##### `impl Debug for MIPS`

- <span id="mips-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RiscV`

```rust
struct RiscV;
```

RISC-V architecture specific definitions.

See [RISC-V ELF psABI specification](https://github.com/riscv/riscv-elf-psabi-doc).

#### Implementations

- <span id="riscv-const-x0"></span>`const X0: Register`

- <span id="riscv-const-x1"></span>`const X1: Register`

- <span id="riscv-const-x2"></span>`const X2: Register`

- <span id="riscv-const-x3"></span>`const X3: Register`

- <span id="riscv-const-x4"></span>`const X4: Register`

- <span id="riscv-const-x5"></span>`const X5: Register`

- <span id="riscv-const-x6"></span>`const X6: Register`

- <span id="riscv-const-x7"></span>`const X7: Register`

- <span id="riscv-const-x8"></span>`const X8: Register`

- <span id="riscv-const-x9"></span>`const X9: Register`

- <span id="riscv-const-x10"></span>`const X10: Register`

- <span id="riscv-const-x11"></span>`const X11: Register`

- <span id="riscv-const-x12"></span>`const X12: Register`

- <span id="riscv-const-x13"></span>`const X13: Register`

- <span id="riscv-const-x14"></span>`const X14: Register`

- <span id="riscv-const-x15"></span>`const X15: Register`

- <span id="riscv-const-x16"></span>`const X16: Register`

- <span id="riscv-const-x17"></span>`const X17: Register`

- <span id="riscv-const-x18"></span>`const X18: Register`

- <span id="riscv-const-x19"></span>`const X19: Register`

- <span id="riscv-const-x20"></span>`const X20: Register`

- <span id="riscv-const-x21"></span>`const X21: Register`

- <span id="riscv-const-x22"></span>`const X22: Register`

- <span id="riscv-const-x23"></span>`const X23: Register`

- <span id="riscv-const-x24"></span>`const X24: Register`

- <span id="riscv-const-x25"></span>`const X25: Register`

- <span id="riscv-const-x26"></span>`const X26: Register`

- <span id="riscv-const-x27"></span>`const X27: Register`

- <span id="riscv-const-x28"></span>`const X28: Register`

- <span id="riscv-const-x29"></span>`const X29: Register`

- <span id="riscv-const-x30"></span>`const X30: Register`

- <span id="riscv-const-x31"></span>`const X31: Register`

- <span id="riscv-const-f0"></span>`const F0: Register`

- <span id="riscv-const-f1"></span>`const F1: Register`

- <span id="riscv-const-f2"></span>`const F2: Register`

- <span id="riscv-const-f3"></span>`const F3: Register`

- <span id="riscv-const-f4"></span>`const F4: Register`

- <span id="riscv-const-f5"></span>`const F5: Register`

- <span id="riscv-const-f6"></span>`const F6: Register`

- <span id="riscv-const-f7"></span>`const F7: Register`

- <span id="riscv-const-f8"></span>`const F8: Register`

- <span id="riscv-const-f9"></span>`const F9: Register`

- <span id="riscv-const-f10"></span>`const F10: Register`

- <span id="riscv-const-f11"></span>`const F11: Register`

- <span id="riscv-const-f12"></span>`const F12: Register`

- <span id="riscv-const-f13"></span>`const F13: Register`

- <span id="riscv-const-f14"></span>`const F14: Register`

- <span id="riscv-const-f15"></span>`const F15: Register`

- <span id="riscv-const-f16"></span>`const F16: Register`

- <span id="riscv-const-f17"></span>`const F17: Register`

- <span id="riscv-const-f18"></span>`const F18: Register`

- <span id="riscv-const-f19"></span>`const F19: Register`

- <span id="riscv-const-f20"></span>`const F20: Register`

- <span id="riscv-const-f21"></span>`const F21: Register`

- <span id="riscv-const-f22"></span>`const F22: Register`

- <span id="riscv-const-f23"></span>`const F23: Register`

- <span id="riscv-const-f24"></span>`const F24: Register`

- <span id="riscv-const-f25"></span>`const F25: Register`

- <span id="riscv-const-f26"></span>`const F26: Register`

- <span id="riscv-const-f27"></span>`const F27: Register`

- <span id="riscv-const-f28"></span>`const F28: Register`

- <span id="riscv-const-f29"></span>`const F29: Register`

- <span id="riscv-const-f30"></span>`const F30: Register`

- <span id="riscv-const-f31"></span>`const F31: Register`

- <span id="riscv-const-afrc"></span>`const AFRC: Register`

- <span id="riscv-const-v0"></span>`const V0: Register`

- <span id="riscv-const-v1"></span>`const V1: Register`

- <span id="riscv-const-v2"></span>`const V2: Register`

- <span id="riscv-const-v3"></span>`const V3: Register`

- <span id="riscv-const-v4"></span>`const V4: Register`

- <span id="riscv-const-v5"></span>`const V5: Register`

- <span id="riscv-const-v6"></span>`const V6: Register`

- <span id="riscv-const-v7"></span>`const V7: Register`

- <span id="riscv-const-v8"></span>`const V8: Register`

- <span id="riscv-const-v9"></span>`const V9: Register`

- <span id="riscv-const-v10"></span>`const V10: Register`

- <span id="riscv-const-v11"></span>`const V11: Register`

- <span id="riscv-const-v12"></span>`const V12: Register`

- <span id="riscv-const-v13"></span>`const V13: Register`

- <span id="riscv-const-v14"></span>`const V14: Register`

- <span id="riscv-const-v15"></span>`const V15: Register`

- <span id="riscv-const-v16"></span>`const V16: Register`

- <span id="riscv-const-v17"></span>`const V17: Register`

- <span id="riscv-const-v18"></span>`const V18: Register`

- <span id="riscv-const-v19"></span>`const V19: Register`

- <span id="riscv-const-v20"></span>`const V20: Register`

- <span id="riscv-const-v21"></span>`const V21: Register`

- <span id="riscv-const-v22"></span>`const V22: Register`

- <span id="riscv-const-v23"></span>`const V23: Register`

- <span id="riscv-const-v24"></span>`const V24: Register`

- <span id="riscv-const-v25"></span>`const V25: Register`

- <span id="riscv-const-v26"></span>`const V26: Register`

- <span id="riscv-const-v27"></span>`const V27: Register`

- <span id="riscv-const-v28"></span>`const V28: Register`

- <span id="riscv-const-v29"></span>`const V29: Register`

- <span id="riscv-const-v30"></span>`const V30: Register`

- <span id="riscv-const-v31"></span>`const V31: Register`

- <span id="riscv-const-zero"></span>`const ZERO: Register`

- <span id="riscv-const-ra"></span>`const RA: Register`

- <span id="riscv-const-sp"></span>`const SP: Register`

- <span id="riscv-const-gp"></span>`const GP: Register`

- <span id="riscv-const-tp"></span>`const TP: Register`

- <span id="riscv-const-t0"></span>`const T0: Register`

- <span id="riscv-const-t1"></span>`const T1: Register`

- <span id="riscv-const-t2"></span>`const T2: Register`

- <span id="riscv-const-s0"></span>`const S0: Register`

- <span id="riscv-const-fp"></span>`const FP: Register`

- <span id="riscv-const-s1"></span>`const S1: Register`

- <span id="riscv-const-a0"></span>`const A0: Register`

- <span id="riscv-const-a1"></span>`const A1: Register`

- <span id="riscv-const-a2"></span>`const A2: Register`

- <span id="riscv-const-a3"></span>`const A3: Register`

- <span id="riscv-const-a4"></span>`const A4: Register`

- <span id="riscv-const-a5"></span>`const A5: Register`

- <span id="riscv-const-a6"></span>`const A6: Register`

- <span id="riscv-const-a7"></span>`const A7: Register`

- <span id="riscv-const-s2"></span>`const S2: Register`

- <span id="riscv-const-s3"></span>`const S3: Register`

- <span id="riscv-const-s4"></span>`const S4: Register`

- <span id="riscv-const-s5"></span>`const S5: Register`

- <span id="riscv-const-s6"></span>`const S6: Register`

- <span id="riscv-const-s7"></span>`const S7: Register`

- <span id="riscv-const-s8"></span>`const S8: Register`

- <span id="riscv-const-s9"></span>`const S9: Register`

- <span id="riscv-const-s10"></span>`const S10: Register`

- <span id="riscv-const-s11"></span>`const S11: Register`

- <span id="riscv-const-t3"></span>`const T3: Register`

- <span id="riscv-const-t4"></span>`const T4: Register`

- <span id="riscv-const-t5"></span>`const T5: Register`

- <span id="riscv-const-t6"></span>`const T6: Register`

- <span id="riscv-const-ft0"></span>`const FT0: Register`

- <span id="riscv-const-ft1"></span>`const FT1: Register`

- <span id="riscv-const-ft2"></span>`const FT2: Register`

- <span id="riscv-const-ft3"></span>`const FT3: Register`

- <span id="riscv-const-ft4"></span>`const FT4: Register`

- <span id="riscv-const-ft5"></span>`const FT5: Register`

- <span id="riscv-const-ft6"></span>`const FT6: Register`

- <span id="riscv-const-ft7"></span>`const FT7: Register`

- <span id="riscv-const-fs0"></span>`const FS0: Register`

- <span id="riscv-const-fs1"></span>`const FS1: Register`

- <span id="riscv-const-fa0"></span>`const FA0: Register`

- <span id="riscv-const-fa1"></span>`const FA1: Register`

- <span id="riscv-const-fa2"></span>`const FA2: Register`

- <span id="riscv-const-fa3"></span>`const FA3: Register`

- <span id="riscv-const-fa4"></span>`const FA4: Register`

- <span id="riscv-const-fa5"></span>`const FA5: Register`

- <span id="riscv-const-fa6"></span>`const FA6: Register`

- <span id="riscv-const-fa7"></span>`const FA7: Register`

- <span id="riscv-const-fs2"></span>`const FS2: Register`

- <span id="riscv-const-fs3"></span>`const FS3: Register`

- <span id="riscv-const-fs4"></span>`const FS4: Register`

- <span id="riscv-const-fs5"></span>`const FS5: Register`

- <span id="riscv-const-fs6"></span>`const FS6: Register`

- <span id="riscv-const-fs7"></span>`const FS7: Register`

- <span id="riscv-const-fs8"></span>`const FS8: Register`

- <span id="riscv-const-fs9"></span>`const FS9: Register`

- <span id="riscv-const-fs10"></span>`const FS10: Register`

- <span id="riscv-const-fs11"></span>`const FS11: Register`

- <span id="riscv-const-ft8"></span>`const FT8: Register`

- <span id="riscv-const-ft9"></span>`const FT9: Register`

- <span id="riscv-const-ft10"></span>`const FT10: Register`

- <span id="riscv-const-ft11"></span>`const FT11: Register`

#### Trait Implementations

##### `impl Clone for RiscV`

- <span id="riscv-clone"></span>`fn clone(&self) -> RiscV` — [`RiscV`](#riscv)

##### `impl Copy for RiscV`

##### `impl Debug for RiscV`

- <span id="riscv-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `X86`

```rust
struct X86;
```

Intel i386 architecture specific definitions.

See section 2.4.2 of the [i386 psABI](https://gitlab.com/x86-psABIs/i386-ABI).

#### Implementations

- <span id="x86-const-eax"></span>`const EAX: Register`

- <span id="x86-const-ecx"></span>`const ECX: Register`

- <span id="x86-const-edx"></span>`const EDX: Register`

- <span id="x86-const-ebx"></span>`const EBX: Register`

- <span id="x86-const-esp"></span>`const ESP: Register`

- <span id="x86-const-ebp"></span>`const EBP: Register`

- <span id="x86-const-esi"></span>`const ESI: Register`

- <span id="x86-const-edi"></span>`const EDI: Register`

- <span id="x86-const-ra"></span>`const RA: Register`

- <span id="x86-const-st0"></span>`const ST0: Register`

- <span id="x86-const-st1"></span>`const ST1: Register`

- <span id="x86-const-st2"></span>`const ST2: Register`

- <span id="x86-const-st3"></span>`const ST3: Register`

- <span id="x86-const-st4"></span>`const ST4: Register`

- <span id="x86-const-st5"></span>`const ST5: Register`

- <span id="x86-const-st6"></span>`const ST6: Register`

- <span id="x86-const-st7"></span>`const ST7: Register`

- <span id="x86-const-xmm0"></span>`const XMM0: Register`

- <span id="x86-const-xmm1"></span>`const XMM1: Register`

- <span id="x86-const-xmm2"></span>`const XMM2: Register`

- <span id="x86-const-xmm3"></span>`const XMM3: Register`

- <span id="x86-const-xmm4"></span>`const XMM4: Register`

- <span id="x86-const-xmm5"></span>`const XMM5: Register`

- <span id="x86-const-xmm6"></span>`const XMM6: Register`

- <span id="x86-const-xmm7"></span>`const XMM7: Register`

- <span id="x86-const-mm0"></span>`const MM0: Register`

- <span id="x86-const-mm1"></span>`const MM1: Register`

- <span id="x86-const-mm2"></span>`const MM2: Register`

- <span id="x86-const-mm3"></span>`const MM3: Register`

- <span id="x86-const-mm4"></span>`const MM4: Register`

- <span id="x86-const-mm5"></span>`const MM5: Register`

- <span id="x86-const-mm6"></span>`const MM6: Register`

- <span id="x86-const-mm7"></span>`const MM7: Register`

- <span id="x86-const-mxcsr"></span>`const MXCSR: Register`

- <span id="x86-const-es"></span>`const ES: Register`

- <span id="x86-const-cs"></span>`const CS: Register`

- <span id="x86-const-ss"></span>`const SS: Register`

- <span id="x86-const-ds"></span>`const DS: Register`

- <span id="x86-const-fs"></span>`const FS: Register`

- <span id="x86-const-gs"></span>`const GS: Register`

- <span id="x86-const-tr"></span>`const TR: Register`

- <span id="x86-const-ldtr"></span>`const LDTR: Register`

- <span id="x86-const-fs-base"></span>`const FS_BASE: Register`

- <span id="x86-const-gs-base"></span>`const GS_BASE: Register`

#### Trait Implementations

##### `impl Clone for X86`

- <span id="x86-clone"></span>`fn clone(&self) -> X86` — [`X86`](#x86)

##### `impl Copy for X86`

##### `impl Debug for X86`

- <span id="x86-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `X86_64`

```rust
struct X86_64;
```

AMD64 architecture specific definitions.

See section 3.6.2 of the [x86-64 psABI](https://gitlab.com/x86-psABIs/x86-64-ABI).

#### Implementations

- <span id="x86-64-const-rax"></span>`const RAX: Register`

- <span id="x86-64-const-rdx"></span>`const RDX: Register`

- <span id="x86-64-const-rcx"></span>`const RCX: Register`

- <span id="x86-64-const-rbx"></span>`const RBX: Register`

- <span id="x86-64-const-rsi"></span>`const RSI: Register`

- <span id="x86-64-const-rdi"></span>`const RDI: Register`

- <span id="x86-64-const-rbp"></span>`const RBP: Register`

- <span id="x86-64-const-rsp"></span>`const RSP: Register`

- <span id="x86-64-const-r8"></span>`const R8: Register`

- <span id="x86-64-const-r9"></span>`const R9: Register`

- <span id="x86-64-const-r10"></span>`const R10: Register`

- <span id="x86-64-const-r11"></span>`const R11: Register`

- <span id="x86-64-const-r12"></span>`const R12: Register`

- <span id="x86-64-const-r13"></span>`const R13: Register`

- <span id="x86-64-const-r14"></span>`const R14: Register`

- <span id="x86-64-const-r15"></span>`const R15: Register`

- <span id="x86-64-const-ra"></span>`const RA: Register`

- <span id="x86-64-const-xmm0"></span>`const XMM0: Register`

- <span id="x86-64-const-xmm1"></span>`const XMM1: Register`

- <span id="x86-64-const-xmm2"></span>`const XMM2: Register`

- <span id="x86-64-const-xmm3"></span>`const XMM3: Register`

- <span id="x86-64-const-xmm4"></span>`const XMM4: Register`

- <span id="x86-64-const-xmm5"></span>`const XMM5: Register`

- <span id="x86-64-const-xmm6"></span>`const XMM6: Register`

- <span id="x86-64-const-xmm7"></span>`const XMM7: Register`

- <span id="x86-64-const-xmm8"></span>`const XMM8: Register`

- <span id="x86-64-const-xmm9"></span>`const XMM9: Register`

- <span id="x86-64-const-xmm10"></span>`const XMM10: Register`

- <span id="x86-64-const-xmm11"></span>`const XMM11: Register`

- <span id="x86-64-const-xmm12"></span>`const XMM12: Register`

- <span id="x86-64-const-xmm13"></span>`const XMM13: Register`

- <span id="x86-64-const-xmm14"></span>`const XMM14: Register`

- <span id="x86-64-const-xmm15"></span>`const XMM15: Register`

- <span id="x86-64-const-st0"></span>`const ST0: Register`

- <span id="x86-64-const-st1"></span>`const ST1: Register`

- <span id="x86-64-const-st2"></span>`const ST2: Register`

- <span id="x86-64-const-st3"></span>`const ST3: Register`

- <span id="x86-64-const-st4"></span>`const ST4: Register`

- <span id="x86-64-const-st5"></span>`const ST5: Register`

- <span id="x86-64-const-st6"></span>`const ST6: Register`

- <span id="x86-64-const-st7"></span>`const ST7: Register`

- <span id="x86-64-const-mm0"></span>`const MM0: Register`

- <span id="x86-64-const-mm1"></span>`const MM1: Register`

- <span id="x86-64-const-mm2"></span>`const MM2: Register`

- <span id="x86-64-const-mm3"></span>`const MM3: Register`

- <span id="x86-64-const-mm4"></span>`const MM4: Register`

- <span id="x86-64-const-mm5"></span>`const MM5: Register`

- <span id="x86-64-const-mm6"></span>`const MM6: Register`

- <span id="x86-64-const-mm7"></span>`const MM7: Register`

- <span id="x86-64-const-rflags"></span>`const RFLAGS: Register`

- <span id="x86-64-const-es"></span>`const ES: Register`

- <span id="x86-64-const-cs"></span>`const CS: Register`

- <span id="x86-64-const-ss"></span>`const SS: Register`

- <span id="x86-64-const-ds"></span>`const DS: Register`

- <span id="x86-64-const-fs"></span>`const FS: Register`

- <span id="x86-64-const-gs"></span>`const GS: Register`

- <span id="x86-64-const-fs-base"></span>`const FS_BASE: Register`

- <span id="x86-64-const-gs-base"></span>`const GS_BASE: Register`

- <span id="x86-64-const-tr"></span>`const TR: Register`

- <span id="x86-64-const-ldtr"></span>`const LDTR: Register`

- <span id="x86-64-const-mxcsr"></span>`const MXCSR: Register`

- <span id="x86-64-const-fcw"></span>`const FCW: Register`

- <span id="x86-64-const-fsw"></span>`const FSW: Register`

- <span id="x86-64-const-xmm16"></span>`const XMM16: Register`

- <span id="x86-64-const-xmm17"></span>`const XMM17: Register`

- <span id="x86-64-const-xmm18"></span>`const XMM18: Register`

- <span id="x86-64-const-xmm19"></span>`const XMM19: Register`

- <span id="x86-64-const-xmm20"></span>`const XMM20: Register`

- <span id="x86-64-const-xmm21"></span>`const XMM21: Register`

- <span id="x86-64-const-xmm22"></span>`const XMM22: Register`

- <span id="x86-64-const-xmm23"></span>`const XMM23: Register`

- <span id="x86-64-const-xmm24"></span>`const XMM24: Register`

- <span id="x86-64-const-xmm25"></span>`const XMM25: Register`

- <span id="x86-64-const-xmm26"></span>`const XMM26: Register`

- <span id="x86-64-const-xmm27"></span>`const XMM27: Register`

- <span id="x86-64-const-xmm28"></span>`const XMM28: Register`

- <span id="x86-64-const-xmm29"></span>`const XMM29: Register`

- <span id="x86-64-const-xmm30"></span>`const XMM30: Register`

- <span id="x86-64-const-xmm31"></span>`const XMM31: Register`

- <span id="x86-64-const-k0"></span>`const K0: Register`

- <span id="x86-64-const-k1"></span>`const K1: Register`

- <span id="x86-64-const-k2"></span>`const K2: Register`

- <span id="x86-64-const-k3"></span>`const K3: Register`

- <span id="x86-64-const-k4"></span>`const K4: Register`

- <span id="x86-64-const-k5"></span>`const K5: Register`

- <span id="x86-64-const-k6"></span>`const K6: Register`

- <span id="x86-64-const-k7"></span>`const K7: Register`

- <span id="x86-64-const-r16"></span>`const R16: Register`

- <span id="x86-64-const-r17"></span>`const R17: Register`

- <span id="x86-64-const-r18"></span>`const R18: Register`

- <span id="x86-64-const-r19"></span>`const R19: Register`

- <span id="x86-64-const-r20"></span>`const R20: Register`

- <span id="x86-64-const-r21"></span>`const R21: Register`

- <span id="x86-64-const-r22"></span>`const R22: Register`

- <span id="x86-64-const-r23"></span>`const R23: Register`

- <span id="x86-64-const-r24"></span>`const R24: Register`

- <span id="x86-64-const-r25"></span>`const R25: Register`

- <span id="x86-64-const-r26"></span>`const R26: Register`

- <span id="x86-64-const-r27"></span>`const R27: Register`

- <span id="x86-64-const-r28"></span>`const R28: Register`

- <span id="x86-64-const-r29"></span>`const R29: Register`

- <span id="x86-64-const-r30"></span>`const R30: Register`

- <span id="x86-64-const-r31"></span>`const R31: Register`

- <span id="x86-64-const-tmm0"></span>`const TMM0: Register`

- <span id="x86-64-const-tmm1"></span>`const TMM1: Register`

- <span id="x86-64-const-tmm2"></span>`const TMM2: Register`

- <span id="x86-64-const-tmm3"></span>`const TMM3: Register`

- <span id="x86-64-const-tmm4"></span>`const TMM4: Register`

- <span id="x86-64-const-tmm5"></span>`const TMM5: Register`

- <span id="x86-64-const-tmm6"></span>`const TMM6: Register`

- <span id="x86-64-const-tmm7"></span>`const TMM7: Register`

- <span id="x86-64-const-tilecfg"></span>`const TILECFG: Register`

#### Trait Implementations

##### `impl Clone for X86_64`

- <span id="x86-64-clone"></span>`fn clone(&self) -> X86_64` — [`X86_64`](#x86-64)

##### `impl Copy for X86_64`

##### `impl Debug for X86_64`

- <span id="x86-64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PowerPc64`

```rust
struct PowerPc64;
```

PowerPC 64bit

See [64-bit ELF ABI Specification for OpenPOWER Architecture](https://openpowerfoundation.org/specifications/64bitelfabi/).

#### Implementations

- <span id="powerpc64-const-r0"></span>`const R0: Register`

- <span id="powerpc64-const-r1"></span>`const R1: Register`

- <span id="powerpc64-const-r2"></span>`const R2: Register`

- <span id="powerpc64-const-r3"></span>`const R3: Register`

- <span id="powerpc64-const-r4"></span>`const R4: Register`

- <span id="powerpc64-const-r5"></span>`const R5: Register`

- <span id="powerpc64-const-r6"></span>`const R6: Register`

- <span id="powerpc64-const-r7"></span>`const R7: Register`

- <span id="powerpc64-const-r8"></span>`const R8: Register`

- <span id="powerpc64-const-r9"></span>`const R9: Register`

- <span id="powerpc64-const-r10"></span>`const R10: Register`

- <span id="powerpc64-const-r11"></span>`const R11: Register`

- <span id="powerpc64-const-r12"></span>`const R12: Register`

- <span id="powerpc64-const-r13"></span>`const R13: Register`

- <span id="powerpc64-const-r14"></span>`const R14: Register`

- <span id="powerpc64-const-r15"></span>`const R15: Register`

- <span id="powerpc64-const-r16"></span>`const R16: Register`

- <span id="powerpc64-const-r17"></span>`const R17: Register`

- <span id="powerpc64-const-r18"></span>`const R18: Register`

- <span id="powerpc64-const-r19"></span>`const R19: Register`

- <span id="powerpc64-const-r20"></span>`const R20: Register`

- <span id="powerpc64-const-r21"></span>`const R21: Register`

- <span id="powerpc64-const-r22"></span>`const R22: Register`

- <span id="powerpc64-const-r23"></span>`const R23: Register`

- <span id="powerpc64-const-r24"></span>`const R24: Register`

- <span id="powerpc64-const-r25"></span>`const R25: Register`

- <span id="powerpc64-const-r26"></span>`const R26: Register`

- <span id="powerpc64-const-r27"></span>`const R27: Register`

- <span id="powerpc64-const-r28"></span>`const R28: Register`

- <span id="powerpc64-const-r29"></span>`const R29: Register`

- <span id="powerpc64-const-r30"></span>`const R30: Register`

- <span id="powerpc64-const-r31"></span>`const R31: Register`

- <span id="powerpc64-const-f0"></span>`const F0: Register`

- <span id="powerpc64-const-f1"></span>`const F1: Register`

- <span id="powerpc64-const-f2"></span>`const F2: Register`

- <span id="powerpc64-const-f3"></span>`const F3: Register`

- <span id="powerpc64-const-f4"></span>`const F4: Register`

- <span id="powerpc64-const-f5"></span>`const F5: Register`

- <span id="powerpc64-const-f6"></span>`const F6: Register`

- <span id="powerpc64-const-f7"></span>`const F7: Register`

- <span id="powerpc64-const-f8"></span>`const F8: Register`

- <span id="powerpc64-const-f9"></span>`const F9: Register`

- <span id="powerpc64-const-f10"></span>`const F10: Register`

- <span id="powerpc64-const-f11"></span>`const F11: Register`

- <span id="powerpc64-const-f12"></span>`const F12: Register`

- <span id="powerpc64-const-f13"></span>`const F13: Register`

- <span id="powerpc64-const-f14"></span>`const F14: Register`

- <span id="powerpc64-const-f15"></span>`const F15: Register`

- <span id="powerpc64-const-f16"></span>`const F16: Register`

- <span id="powerpc64-const-f17"></span>`const F17: Register`

- <span id="powerpc64-const-f18"></span>`const F18: Register`

- <span id="powerpc64-const-f19"></span>`const F19: Register`

- <span id="powerpc64-const-f20"></span>`const F20: Register`

- <span id="powerpc64-const-f21"></span>`const F21: Register`

- <span id="powerpc64-const-f22"></span>`const F22: Register`

- <span id="powerpc64-const-f23"></span>`const F23: Register`

- <span id="powerpc64-const-f24"></span>`const F24: Register`

- <span id="powerpc64-const-f25"></span>`const F25: Register`

- <span id="powerpc64-const-f26"></span>`const F26: Register`

- <span id="powerpc64-const-f27"></span>`const F27: Register`

- <span id="powerpc64-const-f28"></span>`const F28: Register`

- <span id="powerpc64-const-f29"></span>`const F29: Register`

- <span id="powerpc64-const-f30"></span>`const F30: Register`

- <span id="powerpc64-const-f31"></span>`const F31: Register`

- <span id="powerpc64-const-lr"></span>`const LR: Register`

- <span id="powerpc64-const-ctr"></span>`const CTR: Register`

- <span id="powerpc64-const-cr0"></span>`const CR0: Register`

- <span id="powerpc64-const-cr1"></span>`const CR1: Register`

- <span id="powerpc64-const-cr2"></span>`const CR2: Register`

- <span id="powerpc64-const-cr3"></span>`const CR3: Register`

- <span id="powerpc64-const-cr4"></span>`const CR4: Register`

- <span id="powerpc64-const-cr5"></span>`const CR5: Register`

- <span id="powerpc64-const-cr6"></span>`const CR6: Register`

- <span id="powerpc64-const-cr7"></span>`const CR7: Register`

- <span id="powerpc64-const-xer"></span>`const XER: Register`

- <span id="powerpc64-const-vr0"></span>`const VR0: Register`

- <span id="powerpc64-const-vr1"></span>`const VR1: Register`

- <span id="powerpc64-const-vr2"></span>`const VR2: Register`

- <span id="powerpc64-const-vr3"></span>`const VR3: Register`

- <span id="powerpc64-const-vr4"></span>`const VR4: Register`

- <span id="powerpc64-const-vr5"></span>`const VR5: Register`

- <span id="powerpc64-const-vr6"></span>`const VR6: Register`

- <span id="powerpc64-const-vr7"></span>`const VR7: Register`

- <span id="powerpc64-const-vr8"></span>`const VR8: Register`

- <span id="powerpc64-const-vr9"></span>`const VR9: Register`

- <span id="powerpc64-const-vr10"></span>`const VR10: Register`

- <span id="powerpc64-const-vr11"></span>`const VR11: Register`

- <span id="powerpc64-const-vr12"></span>`const VR12: Register`

- <span id="powerpc64-const-vr13"></span>`const VR13: Register`

- <span id="powerpc64-const-vr14"></span>`const VR14: Register`

- <span id="powerpc64-const-vr15"></span>`const VR15: Register`

- <span id="powerpc64-const-vr16"></span>`const VR16: Register`

- <span id="powerpc64-const-vr17"></span>`const VR17: Register`

- <span id="powerpc64-const-vr18"></span>`const VR18: Register`

- <span id="powerpc64-const-vr19"></span>`const VR19: Register`

- <span id="powerpc64-const-vr20"></span>`const VR20: Register`

- <span id="powerpc64-const-vr21"></span>`const VR21: Register`

- <span id="powerpc64-const-vr22"></span>`const VR22: Register`

- <span id="powerpc64-const-vr23"></span>`const VR23: Register`

- <span id="powerpc64-const-vr24"></span>`const VR24: Register`

- <span id="powerpc64-const-vr25"></span>`const VR25: Register`

- <span id="powerpc64-const-vr26"></span>`const VR26: Register`

- <span id="powerpc64-const-vr27"></span>`const VR27: Register`

- <span id="powerpc64-const-vr28"></span>`const VR28: Register`

- <span id="powerpc64-const-vr29"></span>`const VR29: Register`

- <span id="powerpc64-const-vr30"></span>`const VR30: Register`

- <span id="powerpc64-const-vr31"></span>`const VR31: Register`

- <span id="powerpc64-const-vscr"></span>`const VSCR: Register`

- <span id="powerpc64-const-tfhar"></span>`const TFHAR: Register`

- <span id="powerpc64-const-tfiar"></span>`const TFIAR: Register`

- <span id="powerpc64-const-texasr"></span>`const TEXASR: Register`

#### Trait Implementations

##### `impl Clone for PowerPc64`

- <span id="powerpc64-clone"></span>`fn clone(&self) -> PowerPc64` — [`PowerPc64`](#powerpc64)

##### `impl Copy for PowerPc64`

##### `impl Debug for PowerPc64`

- <span id="powerpc64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DwSect`

```rust
struct DwSect(u32);
```

The section type field in a `.dwp` unit index.

This is used for version 5 and later.

See Section 7.3.5.

#### Implementations

- <span id="dwsect-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwSect`

- <span id="dwsect-clone"></span>`fn clone(&self) -> DwSect` — [`DwSect`](#dwsect)

##### `impl Copy for DwSect`

##### `impl Debug for DwSect`

- <span id="dwsect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwSect`

- <span id="dwsect-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwSect`

##### `impl Hash for DwSect`

- <span id="dwsect-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwSect`

- <span id="dwsect-ord-cmp"></span>`fn cmp(&self, other: &DwSect) -> cmp::Ordering` — [`DwSect`](#dwsect)

##### `impl PartialEq for DwSect`

- <span id="dwsect-partialeq-eq"></span>`fn eq(&self, other: &DwSect) -> bool` — [`DwSect`](#dwsect)

##### `impl PartialOrd for DwSect`

- <span id="dwsect-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwSect) -> option::Option<cmp::Ordering>` — [`DwSect`](#dwsect)

##### `impl StructuralPartialEq for DwSect`

##### `impl ToString for DwSect`

- <span id="dwsect-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwSectV2`

```rust
struct DwSectV2(u32);
```

The section type field in a `.dwp` unit index with version 2.

#### Implementations

- <span id="dwsectv2-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwSectV2`

- <span id="dwsectv2-clone"></span>`fn clone(&self) -> DwSectV2` — [`DwSectV2`](#dwsectv2)

##### `impl Copy for DwSectV2`

##### `impl Debug for DwSectV2`

- <span id="dwsectv2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwSectV2`

- <span id="dwsectv2-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwSectV2`

##### `impl Hash for DwSectV2`

- <span id="dwsectv2-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwSectV2`

- <span id="dwsectv2-ord-cmp"></span>`fn cmp(&self, other: &DwSectV2) -> cmp::Ordering` — [`DwSectV2`](#dwsectv2)

##### `impl PartialEq for DwSectV2`

- <span id="dwsectv2-partialeq-eq"></span>`fn eq(&self, other: &DwSectV2) -> bool` — [`DwSectV2`](#dwsectv2)

##### `impl PartialOrd for DwSectV2`

- <span id="dwsectv2-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwSectV2) -> option::Option<cmp::Ordering>` — [`DwSectV2`](#dwsectv2)

##### `impl StructuralPartialEq for DwSectV2`

##### `impl ToString for DwSectV2`

- <span id="dwsectv2-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwUt`

```rust
struct DwUt(u8);
```

The unit type field in a unit header.

See Section 7.5.1, Table 7.2.

#### Implementations

- <span id="dwut-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwUt`

- <span id="dwut-clone"></span>`fn clone(&self) -> DwUt` — [`DwUt`](#dwut)

##### `impl Copy for DwUt`

##### `impl Debug for DwUt`

- <span id="dwut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwUt`

- <span id="dwut-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwUt`

##### `impl Hash for DwUt`

- <span id="dwut-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwUt`

- <span id="dwut-ord-cmp"></span>`fn cmp(&self, other: &DwUt) -> cmp::Ordering` — [`DwUt`](#dwut)

##### `impl PartialEq for DwUt`

- <span id="dwut-partialeq-eq"></span>`fn eq(&self, other: &DwUt) -> bool` — [`DwUt`](#dwut)

##### `impl PartialOrd for DwUt`

- <span id="dwut-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwUt) -> option::Option<cmp::Ordering>` — [`DwUt`](#dwut)

##### `impl StructuralPartialEq for DwUt`

##### `impl ToString for DwUt`

- <span id="dwut-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwCfa`

```rust
struct DwCfa(u8);
```

The opcode for a call frame instruction.

Section 7.24:
> Call frame instructions are encoded in one or more bytes. The primary
> opcode is encoded in the high order two bits of the first byte (that is,
> opcode = byte >> 6). An operand or extended opcode may be encoded in the
> low order 6 bits. Additional operands are encoded in subsequent bytes.

#### Implementations

- <span id="dwcfa-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwCfa`

- <span id="dwcfa-clone"></span>`fn clone(&self) -> DwCfa` — [`DwCfa`](#dwcfa)

##### `impl Copy for DwCfa`

##### `impl Debug for DwCfa`

- <span id="dwcfa-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwCfa`

- <span id="dwcfa-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwCfa`

##### `impl Hash for DwCfa`

- <span id="dwcfa-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwCfa`

- <span id="dwcfa-ord-cmp"></span>`fn cmp(&self, other: &DwCfa) -> cmp::Ordering` — [`DwCfa`](#dwcfa)

##### `impl PartialEq for DwCfa`

- <span id="dwcfa-partialeq-eq"></span>`fn eq(&self, other: &DwCfa) -> bool` — [`DwCfa`](#dwcfa)

##### `impl PartialOrd for DwCfa`

- <span id="dwcfa-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwCfa) -> option::Option<cmp::Ordering>` — [`DwCfa`](#dwcfa)

##### `impl StructuralPartialEq for DwCfa`

##### `impl ToString for DwCfa`

- <span id="dwcfa-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwChildren`

```rust
struct DwChildren(u8);
```

The child determination encodings for DIE attributes.

See Section 7.5.3, Table 7.4.

#### Implementations

- <span id="dwchildren-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwChildren`

- <span id="dwchildren-clone"></span>`fn clone(&self) -> DwChildren` — [`DwChildren`](#dwchildren)

##### `impl Copy for DwChildren`

##### `impl Debug for DwChildren`

- <span id="dwchildren-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwChildren`

- <span id="dwchildren-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwChildren`

##### `impl Hash for DwChildren`

- <span id="dwchildren-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwChildren`

- <span id="dwchildren-ord-cmp"></span>`fn cmp(&self, other: &DwChildren) -> cmp::Ordering` — [`DwChildren`](#dwchildren)

##### `impl PartialEq for DwChildren`

- <span id="dwchildren-partialeq-eq"></span>`fn eq(&self, other: &DwChildren) -> bool` — [`DwChildren`](#dwchildren)

##### `impl PartialOrd for DwChildren`

- <span id="dwchildren-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwChildren) -> option::Option<cmp::Ordering>` — [`DwChildren`](#dwchildren)

##### `impl StructuralPartialEq for DwChildren`

##### `impl ToString for DwChildren`

- <span id="dwchildren-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwTag`

```rust
struct DwTag(u16);
```

The tag encodings for DIE attributes.

See Section 7.5.3, Table 7.3.

#### Implementations

- <span id="dwtag-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwTag`

- <span id="dwtag-clone"></span>`fn clone(&self) -> DwTag` — [`DwTag`](#dwtag)

##### `impl Copy for DwTag`

##### `impl Debug for DwTag`

- <span id="dwtag-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwTag`

- <span id="dwtag-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwTag`

##### `impl Hash for DwTag`

- <span id="dwtag-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwTag`

- <span id="dwtag-ord-cmp"></span>`fn cmp(&self, other: &DwTag) -> cmp::Ordering` — [`DwTag`](#dwtag)

##### `impl PartialEq for DwTag`

- <span id="dwtag-partialeq-eq"></span>`fn eq(&self, other: &DwTag) -> bool` — [`DwTag`](#dwtag)

##### `impl PartialOrd for DwTag`

- <span id="dwtag-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwTag) -> option::Option<cmp::Ordering>` — [`DwTag`](#dwtag)

##### `impl StructuralPartialEq for DwTag`

##### `impl ToString for DwTag`

- <span id="dwtag-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwAt`

```rust
struct DwAt(u16);
```

The attribute encodings for DIE attributes.

See Section 7.5.4, Table 7.5.

#### Implementations

- <span id="dwat-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAt`

- <span id="dwat-clone"></span>`fn clone(&self) -> DwAt` — [`DwAt`](#dwat)

##### `impl Copy for DwAt`

##### `impl Debug for DwAt`

- <span id="dwat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAt`

- <span id="dwat-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAt`

##### `impl Hash for DwAt`

- <span id="dwat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAt`

- <span id="dwat-ord-cmp"></span>`fn cmp(&self, other: &DwAt) -> cmp::Ordering` — [`DwAt`](#dwat)

##### `impl PartialEq for DwAt`

- <span id="dwat-partialeq-eq"></span>`fn eq(&self, other: &DwAt) -> bool` — [`DwAt`](#dwat)

##### `impl PartialOrd for DwAt`

- <span id="dwat-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAt) -> option::Option<cmp::Ordering>` — [`DwAt`](#dwat)

##### `impl StructuralPartialEq for DwAt`

##### `impl ToString for DwAt`

- <span id="dwat-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwForm`

```rust
struct DwForm(u16);
```

The attribute form encodings for DIE attributes.

See Section 7.5.6, Table 7.6.

#### Implementations

- <span id="dwform-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwForm`

- <span id="dwform-clone"></span>`fn clone(&self) -> DwForm` — [`DwForm`](#dwform)

##### `impl Copy for DwForm`

##### `impl Debug for DwForm`

- <span id="dwform-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwForm`

- <span id="dwform-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwForm`

##### `impl Hash for DwForm`

- <span id="dwform-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwForm`

- <span id="dwform-ord-cmp"></span>`fn cmp(&self, other: &DwForm) -> cmp::Ordering` — [`DwForm`](#dwform)

##### `impl PartialEq for DwForm`

- <span id="dwform-partialeq-eq"></span>`fn eq(&self, other: &DwForm) -> bool` — [`DwForm`](#dwform)

##### `impl PartialOrd for DwForm`

- <span id="dwform-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwForm) -> option::Option<cmp::Ordering>` — [`DwForm`](#dwform)

##### `impl StructuralPartialEq for DwForm`

##### `impl ToString for DwForm`

- <span id="dwform-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwAte`

```rust
struct DwAte(u8);
```

The encodings of the constants used in the `DW_AT_encoding` attribute.

See Section 7.8, Table 7.11.

#### Implementations

- <span id="dwate-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAte`

- <span id="dwate-clone"></span>`fn clone(&self) -> DwAte` — [`DwAte`](#dwate)

##### `impl Copy for DwAte`

##### `impl Debug for DwAte`

- <span id="dwate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAte`

- <span id="dwate-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAte`

##### `impl Hash for DwAte`

- <span id="dwate-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAte`

- <span id="dwate-ord-cmp"></span>`fn cmp(&self, other: &DwAte) -> cmp::Ordering` — [`DwAte`](#dwate)

##### `impl PartialEq for DwAte`

- <span id="dwate-partialeq-eq"></span>`fn eq(&self, other: &DwAte) -> bool` — [`DwAte`](#dwate)

##### `impl PartialOrd for DwAte`

- <span id="dwate-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAte) -> option::Option<cmp::Ordering>` — [`DwAte`](#dwate)

##### `impl StructuralPartialEq for DwAte`

##### `impl ToString for DwAte`

- <span id="dwate-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwLle`

```rust
struct DwLle(u8);
```

The encodings of the constants used in location list entries.

See Section 7.7.3, Table 7.10.

#### Implementations

- <span id="dwlle-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLle`

- <span id="dwlle-clone"></span>`fn clone(&self) -> DwLle` — [`DwLle`](#dwlle)

##### `impl Copy for DwLle`

##### `impl Debug for DwLle`

- <span id="dwlle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLle`

- <span id="dwlle-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLle`

##### `impl Hash for DwLle`

- <span id="dwlle-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLle`

- <span id="dwlle-ord-cmp"></span>`fn cmp(&self, other: &DwLle) -> cmp::Ordering` — [`DwLle`](#dwlle)

##### `impl PartialEq for DwLle`

- <span id="dwlle-partialeq-eq"></span>`fn eq(&self, other: &DwLle) -> bool` — [`DwLle`](#dwlle)

##### `impl PartialOrd for DwLle`

- <span id="dwlle-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLle) -> option::Option<cmp::Ordering>` — [`DwLle`](#dwlle)

##### `impl StructuralPartialEq for DwLle`

##### `impl ToString for DwLle`

- <span id="dwlle-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwDs`

```rust
struct DwDs(u8);
```

The encodings of the constants used in the `DW_AT_decimal_sign` attribute.

See Section 7.8, Table 7.12.

#### Implementations

- <span id="dwds-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDs`

- <span id="dwds-clone"></span>`fn clone(&self) -> DwDs` — [`DwDs`](#dwds)

##### `impl Copy for DwDs`

##### `impl Debug for DwDs`

- <span id="dwds-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwDs`

- <span id="dwds-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDs`

##### `impl Hash for DwDs`

- <span id="dwds-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwDs`

- <span id="dwds-ord-cmp"></span>`fn cmp(&self, other: &DwDs) -> cmp::Ordering` — [`DwDs`](#dwds)

##### `impl PartialEq for DwDs`

- <span id="dwds-partialeq-eq"></span>`fn eq(&self, other: &DwDs) -> bool` — [`DwDs`](#dwds)

##### `impl PartialOrd for DwDs`

- <span id="dwds-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwDs) -> option::Option<cmp::Ordering>` — [`DwDs`](#dwds)

##### `impl StructuralPartialEq for DwDs`

##### `impl ToString for DwDs`

- <span id="dwds-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwEnd`

```rust
struct DwEnd(u8);
```

The encodings of the constants used in the `DW_AT_endianity` attribute.

See Section 7.8, Table 7.13.

#### Implementations

- <span id="dwend-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwEnd`

- <span id="dwend-clone"></span>`fn clone(&self) -> DwEnd` — [`DwEnd`](#dwend)

##### `impl Copy for DwEnd`

##### `impl Debug for DwEnd`

- <span id="dwend-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwEnd`

- <span id="dwend-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwEnd`

##### `impl Hash for DwEnd`

- <span id="dwend-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwEnd`

- <span id="dwend-ord-cmp"></span>`fn cmp(&self, other: &DwEnd) -> cmp::Ordering` — [`DwEnd`](#dwend)

##### `impl PartialEq for DwEnd`

- <span id="dwend-partialeq-eq"></span>`fn eq(&self, other: &DwEnd) -> bool` — [`DwEnd`](#dwend)

##### `impl PartialOrd for DwEnd`

- <span id="dwend-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwEnd) -> option::Option<cmp::Ordering>` — [`DwEnd`](#dwend)

##### `impl StructuralPartialEq for DwEnd`

##### `impl ToString for DwEnd`

- <span id="dwend-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwAccess`

```rust
struct DwAccess(u8);
```

The encodings of the constants used in the `DW_AT_accessibility` attribute.

See Section 7.9, Table 7.14.

#### Implementations

- <span id="dwaccess-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAccess`

- <span id="dwaccess-clone"></span>`fn clone(&self) -> DwAccess` — [`DwAccess`](#dwaccess)

##### `impl Copy for DwAccess`

##### `impl Debug for DwAccess`

- <span id="dwaccess-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAccess`

- <span id="dwaccess-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAccess`

##### `impl Hash for DwAccess`

- <span id="dwaccess-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAccess`

- <span id="dwaccess-ord-cmp"></span>`fn cmp(&self, other: &DwAccess) -> cmp::Ordering` — [`DwAccess`](#dwaccess)

##### `impl PartialEq for DwAccess`

- <span id="dwaccess-partialeq-eq"></span>`fn eq(&self, other: &DwAccess) -> bool` — [`DwAccess`](#dwaccess)

##### `impl PartialOrd for DwAccess`

- <span id="dwaccess-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAccess) -> option::Option<cmp::Ordering>` — [`DwAccess`](#dwaccess)

##### `impl StructuralPartialEq for DwAccess`

##### `impl ToString for DwAccess`

- <span id="dwaccess-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwVis`

```rust
struct DwVis(u8);
```

The encodings of the constants used in the `DW_AT_visibility` attribute.

See Section 7.10, Table 7.15.

#### Implementations

- <span id="dwvis-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwVis`

- <span id="dwvis-clone"></span>`fn clone(&self) -> DwVis` — [`DwVis`](#dwvis)

##### `impl Copy for DwVis`

##### `impl Debug for DwVis`

- <span id="dwvis-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwVis`

- <span id="dwvis-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwVis`

##### `impl Hash for DwVis`

- <span id="dwvis-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwVis`

- <span id="dwvis-ord-cmp"></span>`fn cmp(&self, other: &DwVis) -> cmp::Ordering` — [`DwVis`](#dwvis)

##### `impl PartialEq for DwVis`

- <span id="dwvis-partialeq-eq"></span>`fn eq(&self, other: &DwVis) -> bool` — [`DwVis`](#dwvis)

##### `impl PartialOrd for DwVis`

- <span id="dwvis-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwVis) -> option::Option<cmp::Ordering>` — [`DwVis`](#dwvis)

##### `impl StructuralPartialEq for DwVis`

##### `impl ToString for DwVis`

- <span id="dwvis-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwVirtuality`

```rust
struct DwVirtuality(u8);
```

The encodings of the constants used in the `DW_AT_virtuality` attribute.

See Section 7.11, Table 7.16.

#### Implementations

- <span id="dwvirtuality-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwVirtuality`

- <span id="dwvirtuality-clone"></span>`fn clone(&self) -> DwVirtuality` — [`DwVirtuality`](#dwvirtuality)

##### `impl Copy for DwVirtuality`

##### `impl Debug for DwVirtuality`

- <span id="dwvirtuality-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwVirtuality`

- <span id="dwvirtuality-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwVirtuality`

##### `impl Hash for DwVirtuality`

- <span id="dwvirtuality-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwVirtuality`

- <span id="dwvirtuality-ord-cmp"></span>`fn cmp(&self, other: &DwVirtuality) -> cmp::Ordering` — [`DwVirtuality`](#dwvirtuality)

##### `impl PartialEq for DwVirtuality`

- <span id="dwvirtuality-partialeq-eq"></span>`fn eq(&self, other: &DwVirtuality) -> bool` — [`DwVirtuality`](#dwvirtuality)

##### `impl PartialOrd for DwVirtuality`

- <span id="dwvirtuality-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwVirtuality) -> option::Option<cmp::Ordering>` — [`DwVirtuality`](#dwvirtuality)

##### `impl StructuralPartialEq for DwVirtuality`

##### `impl ToString for DwVirtuality`

- <span id="dwvirtuality-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwLang`

```rust
struct DwLang(u16);
```

The encodings of the constants used in the `DW_AT_language` attribute.

See Section 7.12, Table 7.17.

#### Implementations

- <span id="dwlang-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLang`

- <span id="dwlang-clone"></span>`fn clone(&self) -> DwLang` — [`DwLang`](#dwlang)

##### `impl Copy for DwLang`

##### `impl Debug for DwLang`

- <span id="dwlang-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLang`

- <span id="dwlang-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLang`

##### `impl Hash for DwLang`

- <span id="dwlang-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLang`

- <span id="dwlang-ord-cmp"></span>`fn cmp(&self, other: &DwLang) -> cmp::Ordering` — [`DwLang`](#dwlang)

##### `impl PartialEq for DwLang`

- <span id="dwlang-partialeq-eq"></span>`fn eq(&self, other: &DwLang) -> bool` — [`DwLang`](#dwlang)

##### `impl PartialOrd for DwLang`

- <span id="dwlang-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLang) -> option::Option<cmp::Ordering>` — [`DwLang`](#dwlang)

##### `impl StructuralPartialEq for DwLang`

##### `impl ToString for DwLang`

- <span id="dwlang-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwAddr`

```rust
struct DwAddr(u64);
```

The encodings of the constants used in the `DW_AT_address_class` attribute.

There is only one value that is common to all target architectures.
See Section 7.13.

#### Implementations

- <span id="dwaddr-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAddr`

- <span id="dwaddr-clone"></span>`fn clone(&self) -> DwAddr` — [`DwAddr`](#dwaddr)

##### `impl Copy for DwAddr`

##### `impl Debug for DwAddr`

- <span id="dwaddr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwAddr`

- <span id="dwaddr-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAddr`

##### `impl Hash for DwAddr`

- <span id="dwaddr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwAddr`

- <span id="dwaddr-ord-cmp"></span>`fn cmp(&self, other: &DwAddr) -> cmp::Ordering` — [`DwAddr`](#dwaddr)

##### `impl PartialEq for DwAddr`

- <span id="dwaddr-partialeq-eq"></span>`fn eq(&self, other: &DwAddr) -> bool` — [`DwAddr`](#dwaddr)

##### `impl PartialOrd for DwAddr`

- <span id="dwaddr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwAddr) -> option::Option<cmp::Ordering>` — [`DwAddr`](#dwaddr)

##### `impl StructuralPartialEq for DwAddr`

##### `impl ToString for DwAddr`

- <span id="dwaddr-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwId`

```rust
struct DwId(u8);
```

The encodings of the constants used in the `DW_AT_identifier_case` attribute.

See Section 7.14, Table 7.18.

#### Implementations

- <span id="dwid-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwId`

- <span id="dwid-clone"></span>`fn clone(&self) -> DwId` — [`DwId`](#dwid)

##### `impl Copy for DwId`

##### `impl Debug for DwId`

- <span id="dwid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwId`

- <span id="dwid-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwId`

##### `impl Hash for DwId`

- <span id="dwid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwId`

- <span id="dwid-ord-cmp"></span>`fn cmp(&self, other: &DwId) -> cmp::Ordering` — [`DwId`](#dwid)

##### `impl PartialEq for DwId`

- <span id="dwid-partialeq-eq"></span>`fn eq(&self, other: &DwId) -> bool` — [`DwId`](#dwid)

##### `impl PartialOrd for DwId`

- <span id="dwid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwId) -> option::Option<cmp::Ordering>` — [`DwId`](#dwid)

##### `impl StructuralPartialEq for DwId`

##### `impl ToString for DwId`

- <span id="dwid-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwCc`

```rust
struct DwCc(u8);
```

The encodings of the constants used in the `DW_AT_calling_convention` attribute.

See Section 7.15, Table 7.19.

#### Implementations

- <span id="dwcc-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwCc`

- <span id="dwcc-clone"></span>`fn clone(&self) -> DwCc` — [`DwCc`](#dwcc)

##### `impl Copy for DwCc`

##### `impl Debug for DwCc`

- <span id="dwcc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwCc`

- <span id="dwcc-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwCc`

##### `impl Hash for DwCc`

- <span id="dwcc-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwCc`

- <span id="dwcc-ord-cmp"></span>`fn cmp(&self, other: &DwCc) -> cmp::Ordering` — [`DwCc`](#dwcc)

##### `impl PartialEq for DwCc`

- <span id="dwcc-partialeq-eq"></span>`fn eq(&self, other: &DwCc) -> bool` — [`DwCc`](#dwcc)

##### `impl PartialOrd for DwCc`

- <span id="dwcc-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwCc) -> option::Option<cmp::Ordering>` — [`DwCc`](#dwcc)

##### `impl StructuralPartialEq for DwCc`

##### `impl ToString for DwCc`

- <span id="dwcc-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwInl`

```rust
struct DwInl(u8);
```

The encodings of the constants used in the `DW_AT_inline` attribute.

See Section 7.16, Table 7.20.

#### Implementations

- <span id="dwinl-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwInl`

- <span id="dwinl-clone"></span>`fn clone(&self) -> DwInl` — [`DwInl`](#dwinl)

##### `impl Copy for DwInl`

##### `impl Debug for DwInl`

- <span id="dwinl-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwInl`

- <span id="dwinl-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwInl`

##### `impl Hash for DwInl`

- <span id="dwinl-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwInl`

- <span id="dwinl-ord-cmp"></span>`fn cmp(&self, other: &DwInl) -> cmp::Ordering` — [`DwInl`](#dwinl)

##### `impl PartialEq for DwInl`

- <span id="dwinl-partialeq-eq"></span>`fn eq(&self, other: &DwInl) -> bool` — [`DwInl`](#dwinl)

##### `impl PartialOrd for DwInl`

- <span id="dwinl-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwInl) -> option::Option<cmp::Ordering>` — [`DwInl`](#dwinl)

##### `impl StructuralPartialEq for DwInl`

##### `impl ToString for DwInl`

- <span id="dwinl-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwOrd`

```rust
struct DwOrd(u8);
```

The encodings of the constants used in the `DW_AT_ordering` attribute.

See Section 7.17, Table 7.17.

#### Implementations

- <span id="dword-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwOrd`

- <span id="dword-clone"></span>`fn clone(&self) -> DwOrd` — [`DwOrd`](#dword)

##### `impl Copy for DwOrd`

##### `impl Debug for DwOrd`

- <span id="dword-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwOrd`

- <span id="dword-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwOrd`

##### `impl Hash for DwOrd`

- <span id="dword-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwOrd`

- <span id="dword-ord-cmp"></span>`fn cmp(&self, other: &DwOrd) -> cmp::Ordering` — [`DwOrd`](#dword)

##### `impl PartialEq for DwOrd`

- <span id="dword-partialeq-eq"></span>`fn eq(&self, other: &DwOrd) -> bool` — [`DwOrd`](#dword)

##### `impl PartialOrd for DwOrd`

- <span id="dword-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwOrd) -> option::Option<cmp::Ordering>` — [`DwOrd`](#dword)

##### `impl StructuralPartialEq for DwOrd`

##### `impl ToString for DwOrd`

- <span id="dword-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwDsc`

```rust
struct DwDsc(u8);
```

The encodings of the constants used in the `DW_AT_discr_list` attribute.

See Section 7.18, Table 7.22.

#### Implementations

- <span id="dwdsc-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDsc`

- <span id="dwdsc-clone"></span>`fn clone(&self) -> DwDsc` — [`DwDsc`](#dwdsc)

##### `impl Copy for DwDsc`

##### `impl Debug for DwDsc`

- <span id="dwdsc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwDsc`

- <span id="dwdsc-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDsc`

##### `impl Hash for DwDsc`

- <span id="dwdsc-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwDsc`

- <span id="dwdsc-ord-cmp"></span>`fn cmp(&self, other: &DwDsc) -> cmp::Ordering` — [`DwDsc`](#dwdsc)

##### `impl PartialEq for DwDsc`

- <span id="dwdsc-partialeq-eq"></span>`fn eq(&self, other: &DwDsc) -> bool` — [`DwDsc`](#dwdsc)

##### `impl PartialOrd for DwDsc`

- <span id="dwdsc-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwDsc) -> option::Option<cmp::Ordering>` — [`DwDsc`](#dwdsc)

##### `impl StructuralPartialEq for DwDsc`

##### `impl ToString for DwDsc`

- <span id="dwdsc-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwIdx`

```rust
struct DwIdx(u16);
```

Name index attribute encodings.

See Section 7.19, Table 7.23.

#### Implementations

- <span id="dwidx-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwIdx`

- <span id="dwidx-clone"></span>`fn clone(&self) -> DwIdx` — [`DwIdx`](#dwidx)

##### `impl Copy for DwIdx`

##### `impl Debug for DwIdx`

- <span id="dwidx-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwIdx`

- <span id="dwidx-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwIdx`

##### `impl Hash for DwIdx`

- <span id="dwidx-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwIdx`

- <span id="dwidx-ord-cmp"></span>`fn cmp(&self, other: &DwIdx) -> cmp::Ordering` — [`DwIdx`](#dwidx)

##### `impl PartialEq for DwIdx`

- <span id="dwidx-partialeq-eq"></span>`fn eq(&self, other: &DwIdx) -> bool` — [`DwIdx`](#dwidx)

##### `impl PartialOrd for DwIdx`

- <span id="dwidx-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwIdx) -> option::Option<cmp::Ordering>` — [`DwIdx`](#dwidx)

##### `impl StructuralPartialEq for DwIdx`

##### `impl ToString for DwIdx`

- <span id="dwidx-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwDefaulted`

```rust
struct DwDefaulted(u8);
```

The encodings of the constants used in the `DW_AT_defaulted` attribute.

See Section 7.20, Table 7.24.

#### Implementations

- <span id="dwdefaulted-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDefaulted`

- <span id="dwdefaulted-clone"></span>`fn clone(&self) -> DwDefaulted` — [`DwDefaulted`](#dwdefaulted)

##### `impl Copy for DwDefaulted`

##### `impl Debug for DwDefaulted`

- <span id="dwdefaulted-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwDefaulted`

- <span id="dwdefaulted-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDefaulted`

##### `impl Hash for DwDefaulted`

- <span id="dwdefaulted-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwDefaulted`

- <span id="dwdefaulted-ord-cmp"></span>`fn cmp(&self, other: &DwDefaulted) -> cmp::Ordering` — [`DwDefaulted`](#dwdefaulted)

##### `impl PartialEq for DwDefaulted`

- <span id="dwdefaulted-partialeq-eq"></span>`fn eq(&self, other: &DwDefaulted) -> bool` — [`DwDefaulted`](#dwdefaulted)

##### `impl PartialOrd for DwDefaulted`

- <span id="dwdefaulted-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwDefaulted) -> option::Option<cmp::Ordering>` — [`DwDefaulted`](#dwdefaulted)

##### `impl StructuralPartialEq for DwDefaulted`

##### `impl ToString for DwDefaulted`

- <span id="dwdefaulted-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwLns`

```rust
struct DwLns(u8);
```

The encodings for the standard opcodes for line number information.

See Section 7.22, Table 7.25.

#### Implementations

- <span id="dwlns-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLns`

- <span id="dwlns-clone"></span>`fn clone(&self) -> DwLns` — [`DwLns`](#dwlns)

##### `impl Copy for DwLns`

##### `impl Debug for DwLns`

- <span id="dwlns-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLns`

- <span id="dwlns-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLns`

##### `impl Hash for DwLns`

- <span id="dwlns-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLns`

- <span id="dwlns-ord-cmp"></span>`fn cmp(&self, other: &DwLns) -> cmp::Ordering` — [`DwLns`](#dwlns)

##### `impl PartialEq for DwLns`

- <span id="dwlns-partialeq-eq"></span>`fn eq(&self, other: &DwLns) -> bool` — [`DwLns`](#dwlns)

##### `impl PartialOrd for DwLns`

- <span id="dwlns-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLns) -> option::Option<cmp::Ordering>` — [`DwLns`](#dwlns)

##### `impl StructuralPartialEq for DwLns`

##### `impl ToString for DwLns`

- <span id="dwlns-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwLne`

```rust
struct DwLne(u8);
```

The encodings for the extended opcodes for line number information.

See Section 7.22, Table 7.26.

#### Implementations

- <span id="dwlne-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLne`

- <span id="dwlne-clone"></span>`fn clone(&self) -> DwLne` — [`DwLne`](#dwlne)

##### `impl Copy for DwLne`

##### `impl Debug for DwLne`

- <span id="dwlne-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLne`

- <span id="dwlne-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLne`

##### `impl Hash for DwLne`

- <span id="dwlne-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLne`

- <span id="dwlne-ord-cmp"></span>`fn cmp(&self, other: &DwLne) -> cmp::Ordering` — [`DwLne`](#dwlne)

##### `impl PartialEq for DwLne`

- <span id="dwlne-partialeq-eq"></span>`fn eq(&self, other: &DwLne) -> bool` — [`DwLne`](#dwlne)

##### `impl PartialOrd for DwLne`

- <span id="dwlne-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLne) -> option::Option<cmp::Ordering>` — [`DwLne`](#dwlne)

##### `impl StructuralPartialEq for DwLne`

##### `impl ToString for DwLne`

- <span id="dwlne-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwLnct`

```rust
struct DwLnct(u16);
```

The encodings for the line number header entry formats.

See Section 7.22, Table 7.27.

#### Implementations

- <span id="dwlnct-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLnct`

- <span id="dwlnct-clone"></span>`fn clone(&self) -> DwLnct` — [`DwLnct`](#dwlnct)

##### `impl Copy for DwLnct`

##### `impl Debug for DwLnct`

- <span id="dwlnct-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwLnct`

- <span id="dwlnct-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLnct`

##### `impl Hash for DwLnct`

- <span id="dwlnct-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwLnct`

- <span id="dwlnct-ord-cmp"></span>`fn cmp(&self, other: &DwLnct) -> cmp::Ordering` — [`DwLnct`](#dwlnct)

##### `impl PartialEq for DwLnct`

- <span id="dwlnct-partialeq-eq"></span>`fn eq(&self, other: &DwLnct) -> bool` — [`DwLnct`](#dwlnct)

##### `impl PartialOrd for DwLnct`

- <span id="dwlnct-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwLnct) -> option::Option<cmp::Ordering>` — [`DwLnct`](#dwlnct)

##### `impl StructuralPartialEq for DwLnct`

##### `impl ToString for DwLnct`

- <span id="dwlnct-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwMacinfo`

```rust
struct DwMacinfo(u8);
```

Type codes for macro definitions in the `.debug_macinfo` section.

See Section 7.22, Figure 39 for DWARF 4.

#### Implementations

- <span id="dwmacinfo-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwMacinfo`

- <span id="dwmacinfo-clone"></span>`fn clone(&self) -> DwMacinfo` — [`DwMacinfo`](#dwmacinfo)

##### `impl Copy for DwMacinfo`

##### `impl Debug for DwMacinfo`

- <span id="dwmacinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwMacinfo`

- <span id="dwmacinfo-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwMacinfo`

##### `impl Hash for DwMacinfo`

- <span id="dwmacinfo-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwMacinfo`

- <span id="dwmacinfo-ord-cmp"></span>`fn cmp(&self, other: &DwMacinfo) -> cmp::Ordering` — [`DwMacinfo`](#dwmacinfo)

##### `impl PartialEq for DwMacinfo`

- <span id="dwmacinfo-partialeq-eq"></span>`fn eq(&self, other: &DwMacinfo) -> bool` — [`DwMacinfo`](#dwmacinfo)

##### `impl PartialOrd for DwMacinfo`

- <span id="dwmacinfo-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwMacinfo) -> option::Option<cmp::Ordering>` — [`DwMacinfo`](#dwmacinfo)

##### `impl StructuralPartialEq for DwMacinfo`

##### `impl ToString for DwMacinfo`

- <span id="dwmacinfo-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwMacro`

```rust
struct DwMacro(u8);
```

The encodings for macro information entry types.

See Section 7.23, Table 7.28 for DWARF 5.

#### Implementations

- <span id="dwmacro-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwMacro`

- <span id="dwmacro-clone"></span>`fn clone(&self) -> DwMacro` — [`DwMacro`](#dwmacro)

##### `impl Copy for DwMacro`

##### `impl Debug for DwMacro`

- <span id="dwmacro-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwMacro`

- <span id="dwmacro-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwMacro`

##### `impl Hash for DwMacro`

- <span id="dwmacro-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwMacro`

- <span id="dwmacro-ord-cmp"></span>`fn cmp(&self, other: &DwMacro) -> cmp::Ordering` — [`DwMacro`](#dwmacro)

##### `impl PartialEq for DwMacro`

- <span id="dwmacro-partialeq-eq"></span>`fn eq(&self, other: &DwMacro) -> bool` — [`DwMacro`](#dwmacro)

##### `impl PartialOrd for DwMacro`

- <span id="dwmacro-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwMacro) -> option::Option<cmp::Ordering>` — [`DwMacro`](#dwmacro)

##### `impl StructuralPartialEq for DwMacro`

##### `impl ToString for DwMacro`

- <span id="dwmacro-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwRle`

```rust
struct DwRle(u8);
```

Range list entry encoding values.

See Section 7.25, Table 7.30.

#### Implementations

- <span id="dwrle-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwRle`

- <span id="dwrle-clone"></span>`fn clone(&self) -> DwRle` — [`DwRle`](#dwrle)

##### `impl Copy for DwRle`

##### `impl Debug for DwRle`

- <span id="dwrle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwRle`

- <span id="dwrle-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwRle`

##### `impl Hash for DwRle`

- <span id="dwrle-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwRle`

- <span id="dwrle-ord-cmp"></span>`fn cmp(&self, other: &DwRle) -> cmp::Ordering` — [`DwRle`](#dwrle)

##### `impl PartialEq for DwRle`

- <span id="dwrle-partialeq-eq"></span>`fn eq(&self, other: &DwRle) -> bool` — [`DwRle`](#dwrle)

##### `impl PartialOrd for DwRle`

- <span id="dwrle-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwRle) -> option::Option<cmp::Ordering>` — [`DwRle`](#dwrle)

##### `impl StructuralPartialEq for DwRle`

##### `impl ToString for DwRle`

- <span id="dwrle-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwOp`

```rust
struct DwOp(u8);
```

The encodings for DWARF expression operations.

See Section 7.7.1, Table 7.9.

#### Implementations

- <span id="dwop-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwOp`

- <span id="dwop-clone"></span>`fn clone(&self) -> DwOp` — [`DwOp`](#dwop)

##### `impl Copy for DwOp`

##### `impl Debug for DwOp`

- <span id="dwop-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwOp`

- <span id="dwop-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwOp`

##### `impl Hash for DwOp`

- <span id="dwop-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwOp`

- <span id="dwop-ord-cmp"></span>`fn cmp(&self, other: &DwOp) -> cmp::Ordering` — [`DwOp`](#dwop)

##### `impl PartialEq for DwOp`

- <span id="dwop-partialeq-eq"></span>`fn eq(&self, other: &DwOp) -> bool` — [`DwOp`](#dwop)

##### `impl PartialOrd for DwOp`

- <span id="dwop-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwOp) -> option::Option<cmp::Ordering>` — [`DwOp`](#dwop)

##### `impl StructuralPartialEq for DwOp`

##### `impl ToString for DwOp`

- <span id="dwop-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DwEhPe`

```rust
struct DwEhPe(u8);
```

Pointer encoding used by `.eh_frame`.

The four lower bits describe the
format of the pointer, the upper four bits describe how the encoding should
be applied.

Defined in `<https://refspecs.linuxfoundation.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/dwarfext.html>`

#### Implementations

- <span id="dwehpe-static-string"></span>`fn static_string(&self) -> Option<&'static str>`

#### Trait Implementations

##### `impl BitOr for DwEhPe`

- <span id="dwehpe-bitor-type-output"></span>`type Output = DwEhPe`

- <span id="dwehpe-bitor"></span>`fn bitor(self, rhs: DwEhPe) -> DwEhPe` — [`DwEhPe`](#dwehpe)

##### `impl Clone for DwEhPe`

- <span id="dwehpe-clone"></span>`fn clone(&self) -> DwEhPe` — [`DwEhPe`](#dwehpe)

##### `impl Copy for DwEhPe`

##### `impl Debug for DwEhPe`

- <span id="dwehpe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DwEhPe`

- <span id="dwehpe-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwEhPe`

##### `impl Hash for DwEhPe`

- <span id="dwehpe-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DwEhPe`

- <span id="dwehpe-ord-cmp"></span>`fn cmp(&self, other: &DwEhPe) -> cmp::Ordering` — [`DwEhPe`](#dwehpe)

##### `impl PartialEq for DwEhPe`

- <span id="dwehpe-partialeq-eq"></span>`fn eq(&self, other: &DwEhPe) -> bool` — [`DwEhPe`](#dwehpe)

##### `impl PartialOrd for DwEhPe`

- <span id="dwehpe-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DwEhPe) -> option::Option<cmp::Ordering>` — [`DwEhPe`](#dwehpe)

##### `impl StructuralPartialEq for DwEhPe`

##### `impl ToString for DwEhPe`

- <span id="dwehpe-tostring-to-string"></span>`fn to_string(&self) -> String`

### `LittleEndian`

```rust
struct LittleEndian;
```

Little endian byte order.

#### Trait Implementations

##### `impl Clone for LittleEndian`

- <span id="littleendian-clone"></span>`fn clone(&self) -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- <span id="littleendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LittleEndian`

- <span id="littleendian-default"></span>`fn default() -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Endianity for LittleEndian`

- <span id="littleendian-endianity-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for LittleEndian`

##### `impl Hash for LittleEndian`

- <span id="littleendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for LittleEndian`

- <span id="littleendian-partialeq-eq"></span>`fn eq(&self, other: &LittleEndian) -> bool` — [`LittleEndian`](#littleendian)

##### `impl StructuralPartialEq for LittleEndian`

### `BigEndian`

```rust
struct BigEndian;
```

Big endian byte order.

#### Trait Implementations

##### `impl Clone for BigEndian`

- <span id="bigendian-clone"></span>`fn clone(&self) -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- <span id="bigendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigEndian`

- <span id="bigendian-default"></span>`fn default() -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Endianity for BigEndian`

- <span id="bigendian-endianity-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for BigEndian`

##### `impl Hash for BigEndian`

- <span id="bigendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for BigEndian`

- <span id="bigendian-partialeq-eq"></span>`fn eq(&self, other: &BigEndian) -> bool` — [`BigEndian`](#bigendian)

##### `impl StructuralPartialEq for BigEndian`

### `UnitOffset<T>`

```rust
struct UnitOffset<T>(T);
```

An offset into the current compilation or type unit.

#### Implementations

- <span id="cratereadunitoffset-is-in-bounds"></span>`fn is_in_bounds<R>(&self, unit: &UnitHeader<R>) -> bool` — [`UnitHeader`](read/index.md#unitheader)

  Return true if this offset is within the entries of the given unit.

  

  This only checks that the offset is within the range of the data for unit entries,

  not that there is a valid DIE at this offset.

- <span id="cratereadunitoffset-to-unit-section-offset"></span>`fn to_unit_section_offset<R>(&self, unit: &UnitHeader<R>) -> UnitSectionOffset<T>` — [`UnitHeader`](read/index.md#unitheader), [`UnitSectionOffset`](#unitsectionoffset)

  Convert an offset to be relative to the start of the section containing the unit,

  instead of relative to the start of the given unit.

  

  Does not check that the offset is in bounds or valid. This is because often you

  will know that the offset came from this unit so it must be valid.

  For offsets of uncertain origin, use `UnitOffset::is_in_bounds` first.

- <span id="cratereadunitoffset-to-debug-info-offset"></span>`fn to_debug_info_offset<R>(&self, unit: &UnitHeader<R>) -> Option<DebugInfoOffset<T>>` — [`UnitHeader`](read/index.md#unitheader), [`DebugInfoOffset`](#debuginfooffset)

  Convert an offset to be relative to the start of the `.debug_info` section,

  instead of relative to the start of the given unit.

  

  Returns `None` if the unit is not within the `.debug_info` section.

- <span id="cratereadunitoffset-to-debug-types-offset"></span>`fn to_debug_types_offset<R>(&self, unit: &UnitHeader<R>) -> Option<DebugTypesOffset<T>>` — [`UnitHeader`](read/index.md#unitheader), [`DebugTypesOffset`](#debugtypesoffset)

  Convert an offset to be relative to the start of the `.debug_types` section,

  instead of relative to the start of the given unit.

  

  Returns `None` if the unit is not within the `.debug_types` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for UnitOffset<T>`

- <span id="unitoffset-clone"></span>`fn clone(&self) -> UnitOffset<T>` — [`UnitOffset`](#unitoffset)

##### `impl<T: marker::Copy> Copy for UnitOffset<T>`

##### `impl<T: fmt::Debug> Debug for UnitOffset<T>`

- <span id="unitoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for UnitOffset<T>`

##### `impl<T: hash::Hash> Hash for UnitOffset<T>`

- <span id="unitoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for UnitOffset<T>`

- <span id="unitoffset-ord-cmp"></span>`fn cmp(&self, other: &UnitOffset<T>) -> cmp::Ordering` — [`UnitOffset`](#unitoffset)

##### `impl<T: cmp::PartialEq> PartialEq for UnitOffset<T>`

- <span id="unitoffset-partialeq-eq"></span>`fn eq(&self, other: &UnitOffset<T>) -> bool` — [`UnitOffset`](#unitoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for UnitOffset<T>`

- <span id="unitoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnitOffset<T>) -> option::Option<cmp::Ordering>` — [`UnitOffset`](#unitoffset)

##### `impl<T> StructuralPartialEq for UnitOffset<T>`

### `StoreOnHeap`

```rust
struct StoreOnHeap;
```

Indicates that storage should be allocated on heap.

#### Trait Implementations

##### `impl Clone for StoreOnHeap`

- <span id="storeonheap-clone"></span>`fn clone(&self) -> StoreOnHeap` — [`StoreOnHeap`](#storeonheap)

##### `impl Copy for StoreOnHeap`

##### `impl Debug for StoreOnHeap`

- <span id="storeonheap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StoreOnHeap`

##### `impl<R: Reader> EvaluationStorage for crate::read::StoreOnHeap`

- <span id="cratereadstoreonheap-evaluationstorage-type-stack"></span>`type Stack = Vec<Value>`

- <span id="cratereadstoreonheap-evaluationstorage-type-expressionstack"></span>`type ExpressionStack = Vec<(R, R)>`

- <span id="cratereadstoreonheap-evaluationstorage-type-result"></span>`type Result = Vec<Piece<R>>`

##### `impl PartialEq for StoreOnHeap`

- <span id="storeonheap-partialeq-eq"></span>`fn eq(&self, other: &StoreOnHeap) -> bool` — [`StoreOnHeap`](#storeonheap)

##### `impl StructuralPartialEq for StoreOnHeap`

##### `impl<T: ReaderOffset> UnwindContextStorage for crate::read::StoreOnHeap`

- <span id="cratereadstoreonheap-unwindcontextstorage-type-rules"></span>`type Rules = [(Register, RegisterRule<T>); 192]`

- <span id="cratereadstoreonheap-unwindcontextstorage-type-stack"></span>`type Stack = Box<[UnwindTableRow<T>; 4]>`

## Enums

### `Format`

```rust
enum Format {
    Dwarf64,
    Dwarf32,
}
```

Whether the format of a compilation unit is 32- or 64-bit.

#### Variants

- **`Dwarf64`**

  64-bit DWARF

- **`Dwarf32`**

  32-bit DWARF

#### Implementations

- <span id="format-initial-length-size"></span>`fn initial_length_size(self) -> u8`

  Return the serialized size of an initial length field for the format.

- <span id="format-word-size"></span>`fn word_size(self) -> u8`

  Return the natural word size for the format

#### Trait Implementations

##### `impl Clone for Format`

- <span id="format-clone"></span>`fn clone(&self) -> Format` — [`Format`](#format)

##### `impl Copy for Format`

##### `impl Debug for Format`

- <span id="format-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Format`

##### `impl Hash for Format`

- <span id="format-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Format`

- <span id="format-partialeq-eq"></span>`fn eq(&self, other: &Format) -> bool` — [`Format`](#format)

##### `impl StructuralPartialEq for Format`

### `Vendor`

```rust
enum Vendor {
    Default,
    AArch64,
}
```

Which vendor extensions to support.

#### Variants

- **`Default`**

  A default set of extensions, including some common GNU extensions.

- **`AArch64`**

  AAarch64 extensions.

#### Trait Implementations

##### `impl Clone for Vendor`

- <span id="vendor-clone"></span>`fn clone(&self) -> Vendor` — [`Vendor`](#vendor)

##### `impl Copy for Vendor`

##### `impl Debug for Vendor`

- <span id="vendor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Vendor`

##### `impl PartialEq for Vendor`

- <span id="vendor-partialeq-eq"></span>`fn eq(&self, other: &Vendor) -> bool` — [`Vendor`](#vendor)

##### `impl StructuralPartialEq for Vendor`

### `SectionId`

```rust
enum SectionId {
    DebugAbbrev,
    DebugAddr,
    DebugAranges,
    DebugCuIndex,
    DebugFrame,
    EhFrame,
    EhFrameHdr,
    DebugInfo,
    DebugLine,
    DebugLineStr,
    DebugLoc,
    DebugLocLists,
    DebugMacinfo,
    DebugMacro,
    DebugNames,
    DebugPubNames,
    DebugPubTypes,
    DebugRanges,
    DebugRngLists,
    DebugStr,
    DebugStrOffsets,
    DebugTuIndex,
    DebugTypes,
}
```

An identifier for a DWARF section.

#### Variants

- **`DebugAbbrev`**

  The `.debug_abbrev` section.

- **`DebugAddr`**

  The `.debug_addr` section.

- **`DebugAranges`**

  The `.debug_aranges` section.

- **`DebugCuIndex`**

  The `.debug_cu_index` section.

- **`DebugFrame`**

  The `.debug_frame` section.

- **`EhFrame`**

  The `.eh_frame` section.

- **`EhFrameHdr`**

  The `.eh_frame_hdr` section.

- **`DebugInfo`**

  The `.debug_info` section.

- **`DebugLine`**

  The `.debug_line` section.

- **`DebugLineStr`**

  The `.debug_line_str` section.

- **`DebugLoc`**

  The `.debug_loc` section.

- **`DebugLocLists`**

  The `.debug_loclists` section.

- **`DebugMacinfo`**

  The `.debug_macinfo` section.

- **`DebugMacro`**

  The `.debug_macro` section.

- **`DebugNames`**

  The `.debug_names` section.

- **`DebugPubNames`**

  The `.debug_pubnames` section.

- **`DebugPubTypes`**

  The `.debug_pubtypes` section.

- **`DebugRanges`**

  The `.debug_ranges` section.

- **`DebugRngLists`**

  The `.debug_rnglists` section.

- **`DebugStr`**

  The `.debug_str` section.

- **`DebugStrOffsets`**

  The `.debug_str_offsets` section.

- **`DebugTuIndex`**

  The `.debug_tu_index` section.

- **`DebugTypes`**

  The `.debug_types` section.

#### Implementations

- <span id="sectionid-name"></span>`fn name(self) -> &'static str`

  Returns the ELF section name for this kind.

- <span id="sectionid-dwo-name"></span>`fn dwo_name(self) -> Option<&'static str>`

  Returns the ELF section name for this kind, when found in a .dwo or .dwp file.

- <span id="sectionid-xcoff-name"></span>`fn xcoff_name(self) -> Option<&'static str>`

  Returns the XCOFF section name for this kind.

- <span id="sectionid-is-string"></span>`fn is_string(self) -> bool`

  Returns true if this is a mergeable string section.

  

  This is useful for determining the correct section flags.

#### Trait Implementations

##### `impl Clone for SectionId`

- <span id="sectionid-clone"></span>`fn clone(&self) -> SectionId` — [`SectionId`](#sectionid)

##### `impl Copy for SectionId`

##### `impl Debug for SectionId`

- <span id="sectionid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionId`

##### `impl Hash for SectionId`

- <span id="sectionid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SectionId`

- <span id="sectionid-ord-cmp"></span>`fn cmp(&self, other: &SectionId) -> cmp::Ordering` — [`SectionId`](#sectionid)

##### `impl PartialEq for SectionId`

- <span id="sectionid-partialeq-eq"></span>`fn eq(&self, other: &SectionId) -> bool` — [`SectionId`](#sectionid)

##### `impl PartialOrd for SectionId`

- <span id="sectionid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SectionId) -> option::Option<cmp::Ordering>` — [`SectionId`](#sectionid)

##### `impl StructuralPartialEq for SectionId`

### `DwarfFileType`

```rust
enum DwarfFileType {
    Main,
    Dwo,
}
```

The "type" of file with DWARF debugging information. This determines, among other things,
which files DWARF sections should be loaded from.

#### Variants

- **`Main`**

  A normal executable or object file.

- **`Dwo`**

  A .dwo split DWARF file.

#### Trait Implementations

##### `impl Clone for DwarfFileType`

- <span id="dwarffiletype-clone"></span>`fn clone(&self) -> DwarfFileType` — [`DwarfFileType`](#dwarffiletype)

##### `impl Copy for DwarfFileType`

##### `impl Debug for DwarfFileType`

- <span id="dwarffiletype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DwarfFileType`

- <span id="dwarffiletype-default"></span>`fn default() -> DwarfFileType` — [`DwarfFileType`](#dwarffiletype)

##### `impl Eq for DwarfFileType`

##### `impl PartialEq for DwarfFileType`

- <span id="dwarffiletype-partialeq-eq"></span>`fn eq(&self, other: &DwarfFileType) -> bool` — [`DwarfFileType`](#dwarffiletype)

##### `impl StructuralPartialEq for DwarfFileType`

### `RunTimeEndian`

```rust
enum RunTimeEndian {
    Little,
    Big,
}
```

Byte order that is selectable at runtime.

#### Variants

- **`Little`**

  Little endian byte order.

- **`Big`**

  Big endian byte order.

#### Trait Implementations

##### `impl Clone for RunTimeEndian`

- <span id="runtimeendian-clone"></span>`fn clone(&self) -> RunTimeEndian` — [`RunTimeEndian`](#runtimeendian)

##### `impl Copy for RunTimeEndian`

##### `impl Debug for RunTimeEndian`

- <span id="runtimeendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RunTimeEndian`

- <span id="runtimeendian-default"></span>`fn default() -> RunTimeEndian` — [`RunTimeEndian`](#runtimeendian)

##### `impl Endianity for RunTimeEndian`

- <span id="runtimeendian-endianity-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for RunTimeEndian`

##### `impl Hash for RunTimeEndian`

- <span id="runtimeendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RunTimeEndian`

- <span id="runtimeendian-partialeq-eq"></span>`fn eq(&self, other: &RunTimeEndian) -> bool` — [`RunTimeEndian`](#runtimeendian)

##### `impl StructuralPartialEq for RunTimeEndian`

### `Error`

```rust
enum Error {
    Io,
    PcRelativePointerButSectionBaseIsUndefined,
    TextRelativePointerButTextBaseIsUndefined,
    DataRelativePointerButDataBaseIsUndefined,
    FuncRelativePointerInBadContext,
    CannotParseOmitPointerEncoding,
    BadUnsignedLeb128,
    BadSignedLeb128,
    AbbreviationTagZero,
    AttributeNameZero,
    AttributeFormZero,
    InvalidAbbreviationChildren(constants::DwChildren),
    UnknownForm(constants::DwForm),
    DuplicateAbbreviationCode(u64),
    UnknownReservedLength(u32),
    UnknownVersion(u64),
    InvalidAbbreviationCode(u64),
    UnexpectedEof(ReaderOffsetId),
    UnknownLocListsEntry(constants::DwLle),
    UnknownRangeListsEntry(constants::DwRle),
    UnsupportedAddressSize(u8),
    UnsupportedOffsetSize(u8),
    MinimumInstructionLengthZero,
    MaximumOperationsPerInstructionZero,
    LineRangeZero,
    OpcodeBaseZero,
    BadUtf8,
    NotCieId(u64),
    NotCiePointer(u64),
    BadBranchTarget(u64),
    InvalidPushObjectAddress,
    NotEnoughStackItems,
    TooManyIterations,
    InvalidExpression(constants::DwOp),
    UnsupportedEvaluation,
    InvalidPiece,
    InvalidExpressionTerminator(u64),
    DivisionByZero,
    TypeMismatch,
    IntegralTypeRequired,
    UnsupportedTypeOperation,
    InvalidShiftExpression,
    InvalidDerefSize(u8),
    UnknownCallFrameInstruction(constants::DwCfa),
    InvalidCfiSetLoc(u64),
    AddressOverflow,
    CfiInstructionInInvalidContext,
    PopWithEmptyStack,
    NoUnwindInfoForAddress,
    UnsupportedOffset,
    UnknownPointerEncoding(constants::DwEhPe),
    NoEntryAtGivenOffset(u64),
    OffsetOutOfBounds(u64),
    UnknownAugmentation,
    UnsupportedPointerEncoding(constants::DwEhPe),
    UnsupportedIndirectPointer,
    UnsupportedRegister(u64),
    TooManyRegisterRules,
    StackFull,
    UnknownUnitType(constants::DwUt),
    UnsupportedSegmentSize(u8),
    MissingUnitDie,
    MissingSplitUnit,
    UnsupportedAttributeForm(constants::DwForm),
    MissingFileEntryFormatPath,
    ExpectedStringAttributeValue,
    InvalidImplicitConst,
    UnsupportedIndexSectionCount(u32),
    InvalidIndexSlotCount(u32),
    InvalidIndexRow(u32),
    UnknownIndexSection(constants::DwSect),
    UnknownIndexSectionV2(constants::DwSectV2),
    InvalidMacinfoType(constants::DwMacinfo),
    InvalidMacroType(constants::DwMacro),
    UnsupportedOpcodeOperandsTable,
    InvalidNameAttributeIndex(u64),
}
```

An error that occurred when parsing.

#### Variants

- **`Io`**

  An I/O error occurred while reading.

- **`PcRelativePointerButSectionBaseIsUndefined`**

  Found a PC relative pointer, but the section base is undefined.

- **`TextRelativePointerButTextBaseIsUndefined`**

  Found a `.text` relative pointer, but the `.text` base is undefined.

- **`DataRelativePointerButDataBaseIsUndefined`**

  Found a data relative pointer, but the data base is undefined.

- **`FuncRelativePointerInBadContext`**

  Found a function relative pointer in a context that does not have a
  function base.

- **`CannotParseOmitPointerEncoding`**

  Cannot parse a pointer with a `DW_EH_PE_omit` encoding.

- **`BadUnsignedLeb128`**

  An error parsing an unsigned LEB128 value.

- **`BadSignedLeb128`**

  An error parsing a signed LEB128 value.

- **`AbbreviationTagZero`**

  An abbreviation declared that its tag is zero, but zero is reserved for
  null records.

- **`AttributeNameZero`**

  An attribute specification declared that its name is zero, but zero is
  reserved for null records.

- **`AttributeFormZero`**

  An attribute specification declared that its form is zero, but zero is
  reserved for null records.

- **`InvalidAbbreviationChildren`**

  The abbreviation's has-children byte was not one of
  `DW_CHILDREN_{yes,no}`.

- **`UnknownForm`**

  Found an unknown `DW_FORM_*` type.

- **`DuplicateAbbreviationCode`**

  Found an abbreviation code that has already been used.

- **`UnknownReservedLength`**

  Found an unknown reserved length value.

- **`UnknownVersion`**

  Found an unknown DWARF version.

- **`InvalidAbbreviationCode`**

  Found an entry with an invalid abbreviation code.

- **`UnexpectedEof`**

  Hit the end of input before it was expected.

- **`UnknownLocListsEntry`**

  Found an unknown location-lists format.

- **`UnknownRangeListsEntry`**

  Found an unknown range-lists format.

- **`UnsupportedAddressSize`**

  The specified address size is not supported.

- **`UnsupportedOffsetSize`**

  The specified offset size is not supported.

- **`MinimumInstructionLengthZero`**

  The minimum instruction length must not be zero.

- **`MaximumOperationsPerInstructionZero`**

  The maximum operations per instruction must not be zero.

- **`LineRangeZero`**

  The line range must not be zero.

- **`OpcodeBaseZero`**

  The opcode base must not be zero.

- **`BadUtf8`**

  Found an invalid UTF-8 string.

- **`NotCieId`**

  Expected to find the CIE ID, but found something else.

- **`NotCiePointer`**

  Expected to find a pointer to a CIE, but found the CIE ID instead.

- **`BadBranchTarget`**

  Invalid branch target for a DW_OP_bra or DW_OP_skip.

- **`InvalidPushObjectAddress`**

  DW_OP_push_object_address used but no address passed in.

- **`NotEnoughStackItems`**

  Not enough items on the stack when evaluating an expression.

- **`TooManyIterations`**

  Too many iterations to compute the expression.

- **`InvalidExpression`**

  An unrecognized operation was found while parsing a DWARF
  expression.

- **`UnsupportedEvaluation`**

  An unsupported operation was found while evaluating a DWARF expression.

- **`InvalidPiece`**

  The expression had a piece followed by an expression
  terminator without a piece.

- **`InvalidExpressionTerminator`**

  An expression-terminating operation was followed by something
  other than the end of the expression or a piece operation.

- **`DivisionByZero`**

  Division or modulus by zero when evaluating an expression.

- **`TypeMismatch`**

  An expression operation used mismatching types.

- **`IntegralTypeRequired`**

  An expression operation required an integral type but saw a
  floating point type.

- **`UnsupportedTypeOperation`**

  An expression operation used types that are not supported.

- **`InvalidShiftExpression`**

  The shift value in an expression must be a non-negative integer.

- **`InvalidDerefSize`**

  The size of a deref expression must not be larger than the size of an address.

- **`UnknownCallFrameInstruction`**

  An unknown DW_CFA_* instruction.

- **`InvalidCfiSetLoc`**

  A `DW_CFA_set_loc` instruction moved the address backward.

- **`AddressOverflow`**

  An address calculation overflowed.
  
  This is returned in cases where the address is expected to be
  larger than a previous address, but the calculation overflowed.

- **`CfiInstructionInInvalidContext`**

  Encountered a call frame instruction in a context in which it is not
  valid.

- **`PopWithEmptyStack`**

  When evaluating call frame instructions, found a `DW_CFA_restore_state`
  stack pop instruction, but the stack was empty, and had nothing to pop.

- **`NoUnwindInfoForAddress`**

  Do not have unwind info for the given address.

- **`UnsupportedOffset`**

  An offset value was larger than the maximum supported value.

- **`UnknownPointerEncoding`**

  The given pointer encoding is either unknown or invalid.

- **`NoEntryAtGivenOffset`**

  Did not find an entry at the given offset.

- **`OffsetOutOfBounds`**

  The given offset is out of bounds.

- **`UnknownAugmentation`**

  Found an unknown CFI augmentation.

- **`UnsupportedPointerEncoding`**

  We do not support the given pointer encoding yet.

- **`UnsupportedIndirectPointer`**

  We do not support dereferencing indirect pointers.

- **`UnsupportedRegister`**

  Registers larger than `u16` are not supported.

- **`TooManyRegisterRules`**

  The CFI program defined more register rules than we have storage for.

- **`StackFull`**

  Attempted to push onto the CFI or evaluation stack, but it was already
  at full capacity.

- **`UnknownUnitType`**

  The `DW_UT_*` value for this unit is not supported yet.

- **`UnsupportedSegmentSize`**

  Nonzero segment selector sizes aren't supported yet.

- **`MissingUnitDie`**

  A compilation unit or type unit is missing its top level DIE.

- **`MissingSplitUnit`**

  A split DWARF section does not contain the split compilation unit.

- **`UnsupportedAttributeForm`**

  A DIE attribute used an unsupported form.

- **`MissingFileEntryFormatPath`**

  Missing DW_LNCT_path in file entry format.

- **`ExpectedStringAttributeValue`**

  Expected an attribute value to be a string form.

- **`InvalidImplicitConst`**

  An attribute with an indirect form cannot use `DW_FORM_implicit_const`.

- **`UnsupportedIndexSectionCount`**

  Invalid section count in `.dwp` index.

- **`InvalidIndexSlotCount`**

  Invalid slot count in `.dwp` index.

- **`InvalidIndexRow`**

  Invalid row index in `.dwp` index.

- **`UnknownIndexSection`**

  Unknown section type in `.dwp` index.

- **`UnknownIndexSectionV2`**

  Unknown section type in version 2 `.dwp` index.

- **`InvalidMacinfoType`**

  Invalid macinfo type in `.debug_macinfo`.

- **`InvalidMacroType`**

  Invalid macro type in `.debug_macro`.

- **`UnsupportedOpcodeOperandsTable`**

  The optional `opcode_operands_table` in `.debug_macro` is currently not supported.

- **`InvalidNameAttributeIndex`**

  Invalid index in a `.debug_names` attribute value.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> ::core::result::Result<(), fmt::Error>`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `Endianity`

```rust
trait Endianity: Debug + Default + Clone + Copy + PartialEq + Eq { ... }
```

A trait describing the endianity of some buffer.

#### Required Methods

- `fn is_big_endian(self) -> bool`

  Return true for big endian byte order.

#### Provided Methods

- `fn is_little_endian(self) -> bool`

  Return true for little endian byte order.

- `fn read_u16(self, buf: &[u8]) -> u16`

  Reads an unsigned 16 bit integer from `buf`.

- `fn read_u32(self, buf: &[u8]) -> u32`

  Reads an unsigned 32 bit integer from `buf`.

- `fn read_u64(self, buf: &[u8]) -> u64`

  Reads an unsigned 64 bit integer from `buf`.

- `fn read_u128(self, buf: &[u8]) -> u128`

  Reads an unsigned 128 bit integer from `buf`.

- `fn read_uint(&mut self, buf: &[u8]) -> u64`

  Read an unsigned n-bytes integer u64.

- `fn read_i16(self, buf: &[u8]) -> i16`

  Reads a signed 16 bit integer from `buf`.

- `fn read_i32(self, buf: &[u8]) -> i32`

  Reads a signed 32 bit integer from `buf`.

- `fn read_i64(self, buf: &[u8]) -> i64`

  Reads a signed 64 bit integer from `buf`.

- `fn read_f32(self, buf: &[u8]) -> f32`

  Reads a 32 bit floating point number from `buf`.

- `fn read_f64(self, buf: &[u8]) -> f64`

  Reads a 32 bit floating point number from `buf`.

- `fn write_u16(self, buf: &mut [u8], n: u16)`

  Writes an unsigned 16 bit integer `n` to `buf`.

- `fn write_u32(self, buf: &mut [u8], n: u32)`

  Writes an unsigned 32 bit integer `n` to `buf`.

- `fn write_u64(self, buf: &mut [u8], n: u64)`

  Writes an unsigned 64 bit integer `n` to `buf`.

- `fn write_u128(self, buf: &mut [u8], n: u128)`

  Writes an unsigned 128 bit integer `n` to `buf`.

#### Implementors

- [`BigEndian`](#bigendian)
- [`LittleEndian`](#littleendian)
- [`RunTimeEndian`](#runtimeendian)

### `Section<R>`

```rust
trait Section<R>: From<R> { ... }
```

A convenience trait for loading DWARF sections from object files.  To be
used like:

```rust
use gimli::{DebugInfo, EndianSlice, LittleEndian, Reader, Section};

let buf = [0x00, 0x01, 0x02, 0x03];
let reader = EndianSlice::new(&buf, LittleEndian);
let loader = |name| -> Result<_, ()> { Ok(reader) };

let debug_info: DebugInfo<_> = Section::load(loader).unwrap();
```

#### Required Methods

- `fn id() -> SectionId`

  Returns the section id for this type.

- `fn reader(&self) -> &R`

  Returns the `Reader` for this section.

#### Provided Methods

- `fn section_name() -> &'static str`

  Returns the ELF section name for this type.

- `fn dwo_section_name() -> Option<&'static str>`

  Returns the ELF section name (if any) for this type when used in a dwo

- `fn xcoff_section_name() -> Option<&'static str>`

  Returns the XCOFF section name (if any) for this type when used in a XCOFF

- `fn load<F, E>(f: F) -> core::result::Result<Self, E>`

  Try to load the section using the given loader function.

- `fn dwp_range(&self, offset: u32, size: u32) -> Result<Self>`

  Returns the subrange of the section that is the contribution of

- `fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>`

  Returns the `Reader` for this section.

#### Implementors

- [`DebugAbbrev`](read/index.md#debugabbrev)
- [`DebugAddr`](read/index.md#debugaddr)
- [`DebugAranges`](read/index.md#debugaranges)
- [`DebugCuIndex`](read/index.md#debugcuindex)
- [`DebugFrame`](read/index.md#debugframe)
- [`DebugInfo`](read/index.md#debuginfo)
- [`DebugLineStr`](read/index.md#debuglinestr)
- [`DebugLine`](read/index.md#debugline)
- [`DebugLocLists`](read/index.md#debugloclists)
- [`DebugLoc`](read/index.md#debugloc)
- [`DebugMacinfo`](read/index.md#debugmacinfo)
- [`DebugMacro`](read/index.md#debugmacro)
- [`DebugNames`](read/index.md#debugnames)
- [`DebugPubNames`](read/index.md#debugpubnames)
- [`DebugPubTypes`](read/index.md#debugpubtypes)
- [`DebugRanges`](read/index.md#debugranges)
- [`DebugRngLists`](read/index.md#debugrnglists)
- [`DebugStrOffsets`](read/index.md#debugstroffsets)
- [`DebugStr`](read/index.md#debugstr)
- [`DebugTuIndex`](read/index.md#debugtuindex)
- [`DebugTypes`](read/index.md#debugtypes)
- [`EhFrameHdr`](read/index.md#ehframehdr)
- [`EhFrame`](read/index.md#ehframe)

## Functions

### `case_fold_data`

```rust
fn case_fold_data(c: char) -> char
```

### `case_fold`

```rust
fn case_fold(c: char) -> char
```

Perform case folding for the DWARF name index hashing.

This implements the case folding specified in DWARF 5 Section 6.1.1.4.5.

"The simple case folding algorithm is further described in the CaseFolding.txt file
distributed with the Unicode Character Database. That file defines four classes of
mappings: Common (C), Simple (S), Full (F), and Turkish (T). The hash
computation specified here uses the C + S mappings only, which do not affect the
total length of the string, with the addition that Turkish upper case dotted ’İ’ and
lower case dotless ’ı’ are folded to the Latin lower case ’i’."

### `case_folding_djb_hash`

```rust
fn case_folding_djb_hash(s: &str) -> u32
```

Calculate a case folding DJB hash for the DWARF name index.

This uses the case folding specified in DWARF 5 Section 6.1.1.4.5
with the DJB hash specified in DWARF 5 Section 7.33.

### `djb_hash_byte`

```rust
fn djb_hash_byte(hash: u32, byte: u8) -> u32
```

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

The native endianity for the target platform.

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

The result of a parse.

## Constants

### `DW_SECT_INFO`
```rust
const DW_SECT_INFO: DwSect;
```

### `DW_SECT_ABBREV`
```rust
const DW_SECT_ABBREV: DwSect;
```

### `DW_SECT_LINE`
```rust
const DW_SECT_LINE: DwSect;
```

### `DW_SECT_LOCLISTS`
```rust
const DW_SECT_LOCLISTS: DwSect;
```

### `DW_SECT_STR_OFFSETS`
```rust
const DW_SECT_STR_OFFSETS: DwSect;
```

### `DW_SECT_MACRO`
```rust
const DW_SECT_MACRO: DwSect;
```

### `DW_SECT_RNGLISTS`
```rust
const DW_SECT_RNGLISTS: DwSect;
```

### `DW_SECT_V2_INFO`
```rust
const DW_SECT_V2_INFO: DwSectV2;
```

### `DW_SECT_V2_TYPES`
```rust
const DW_SECT_V2_TYPES: DwSectV2;
```

### `DW_SECT_V2_ABBREV`
```rust
const DW_SECT_V2_ABBREV: DwSectV2;
```

### `DW_SECT_V2_LINE`
```rust
const DW_SECT_V2_LINE: DwSectV2;
```

### `DW_SECT_V2_LOC`
```rust
const DW_SECT_V2_LOC: DwSectV2;
```

### `DW_SECT_V2_STR_OFFSETS`
```rust
const DW_SECT_V2_STR_OFFSETS: DwSectV2;
```

### `DW_SECT_V2_MACINFO`
```rust
const DW_SECT_V2_MACINFO: DwSectV2;
```

### `DW_SECT_V2_MACRO`
```rust
const DW_SECT_V2_MACRO: DwSectV2;
```

### `DW_UT_compile`
```rust
const DW_UT_compile: DwUt;
```

### `DW_UT_type`
```rust
const DW_UT_type: DwUt;
```

### `DW_UT_partial`
```rust
const DW_UT_partial: DwUt;
```

### `DW_UT_skeleton`
```rust
const DW_UT_skeleton: DwUt;
```

### `DW_UT_split_compile`
```rust
const DW_UT_split_compile: DwUt;
```

### `DW_UT_split_type`
```rust
const DW_UT_split_type: DwUt;
```

### `DW_UT_lo_user`
```rust
const DW_UT_lo_user: DwUt;
```

### `DW_UT_hi_user`
```rust
const DW_UT_hi_user: DwUt;
```

### `DW_CFA_advance_loc`
```rust
const DW_CFA_advance_loc: DwCfa;
```

### `DW_CFA_offset`
```rust
const DW_CFA_offset: DwCfa;
```

### `DW_CFA_restore`
```rust
const DW_CFA_restore: DwCfa;
```

### `DW_CFA_nop`
```rust
const DW_CFA_nop: DwCfa;
```

### `DW_CFA_set_loc`
```rust
const DW_CFA_set_loc: DwCfa;
```

### `DW_CFA_advance_loc1`
```rust
const DW_CFA_advance_loc1: DwCfa;
```

### `DW_CFA_advance_loc2`
```rust
const DW_CFA_advance_loc2: DwCfa;
```

### `DW_CFA_advance_loc4`
```rust
const DW_CFA_advance_loc4: DwCfa;
```

### `DW_CFA_offset_extended`
```rust
const DW_CFA_offset_extended: DwCfa;
```

### `DW_CFA_restore_extended`
```rust
const DW_CFA_restore_extended: DwCfa;
```

### `DW_CFA_undefined`
```rust
const DW_CFA_undefined: DwCfa;
```

### `DW_CFA_same_value`
```rust
const DW_CFA_same_value: DwCfa;
```

### `DW_CFA_register`
```rust
const DW_CFA_register: DwCfa;
```

### `DW_CFA_remember_state`
```rust
const DW_CFA_remember_state: DwCfa;
```

### `DW_CFA_restore_state`
```rust
const DW_CFA_restore_state: DwCfa;
```

### `DW_CFA_def_cfa`
```rust
const DW_CFA_def_cfa: DwCfa;
```

### `DW_CFA_def_cfa_register`
```rust
const DW_CFA_def_cfa_register: DwCfa;
```

### `DW_CFA_def_cfa_offset`
```rust
const DW_CFA_def_cfa_offset: DwCfa;
```

### `DW_CFA_def_cfa_expression`
```rust
const DW_CFA_def_cfa_expression: DwCfa;
```

### `DW_CFA_expression`
```rust
const DW_CFA_expression: DwCfa;
```

### `DW_CFA_offset_extended_sf`
```rust
const DW_CFA_offset_extended_sf: DwCfa;
```

### `DW_CFA_def_cfa_sf`
```rust
const DW_CFA_def_cfa_sf: DwCfa;
```

### `DW_CFA_def_cfa_offset_sf`
```rust
const DW_CFA_def_cfa_offset_sf: DwCfa;
```

### `DW_CFA_val_offset`
```rust
const DW_CFA_val_offset: DwCfa;
```

### `DW_CFA_val_offset_sf`
```rust
const DW_CFA_val_offset_sf: DwCfa;
```

### `DW_CFA_val_expression`
```rust
const DW_CFA_val_expression: DwCfa;
```

### `DW_CFA_lo_user`
```rust
const DW_CFA_lo_user: DwCfa;
```

### `DW_CFA_hi_user`
```rust
const DW_CFA_hi_user: DwCfa;
```

### `DW_CFA_MIPS_advance_loc8`
```rust
const DW_CFA_MIPS_advance_loc8: DwCfa;
```

### `DW_CFA_GNU_window_save`
```rust
const DW_CFA_GNU_window_save: DwCfa;
```

### `DW_CFA_GNU_args_size`
```rust
const DW_CFA_GNU_args_size: DwCfa;
```

### `DW_CFA_GNU_negative_offset_extended`
```rust
const DW_CFA_GNU_negative_offset_extended: DwCfa;
```

### `DW_CFA_AARCH64_negate_ra_state`
```rust
const DW_CFA_AARCH64_negate_ra_state: DwCfa;
```

### `DW_CHILDREN_no`
```rust
const DW_CHILDREN_no: DwChildren;
```

### `DW_CHILDREN_yes`
```rust
const DW_CHILDREN_yes: DwChildren;
```

### `DW_TAG_null`
```rust
const DW_TAG_null: DwTag;
```

### `DW_TAG_global_subroutine`
```rust
const DW_TAG_global_subroutine: DwTag;
```

### `DW_TAG_global_variable`
```rust
const DW_TAG_global_variable: DwTag;
```

### `DW_TAG_local_variable`
```rust
const DW_TAG_local_variable: DwTag;
```

### `DW_TAG_subroutine`
```rust
const DW_TAG_subroutine: DwTag;
```

### `DW_TAG_array_type`
```rust
const DW_TAG_array_type: DwTag;
```

### `DW_TAG_class_type`
```rust
const DW_TAG_class_type: DwTag;
```

### `DW_TAG_entry_point`
```rust
const DW_TAG_entry_point: DwTag;
```

### `DW_TAG_enumeration_type`
```rust
const DW_TAG_enumeration_type: DwTag;
```

### `DW_TAG_formal_parameter`
```rust
const DW_TAG_formal_parameter: DwTag;
```

### `DW_TAG_imported_declaration`
```rust
const DW_TAG_imported_declaration: DwTag;
```

### `DW_TAG_label`
```rust
const DW_TAG_label: DwTag;
```

### `DW_TAG_lexical_block`
```rust
const DW_TAG_lexical_block: DwTag;
```

### `DW_TAG_member`
```rust
const DW_TAG_member: DwTag;
```

### `DW_TAG_pointer_type`
```rust
const DW_TAG_pointer_type: DwTag;
```

### `DW_TAG_reference_type`
```rust
const DW_TAG_reference_type: DwTag;
```

### `DW_TAG_compile_unit`
```rust
const DW_TAG_compile_unit: DwTag;
```

### `DW_TAG_string_type`
```rust
const DW_TAG_string_type: DwTag;
```

### `DW_TAG_structure_type`
```rust
const DW_TAG_structure_type: DwTag;
```

### `DW_TAG_subroutine_type`
```rust
const DW_TAG_subroutine_type: DwTag;
```

### `DW_TAG_typedef`
```rust
const DW_TAG_typedef: DwTag;
```

### `DW_TAG_union_type`
```rust
const DW_TAG_union_type: DwTag;
```

### `DW_TAG_unspecified_parameters`
```rust
const DW_TAG_unspecified_parameters: DwTag;
```

### `DW_TAG_variant`
```rust
const DW_TAG_variant: DwTag;
```

### `DW_TAG_common_block`
```rust
const DW_TAG_common_block: DwTag;
```

### `DW_TAG_common_inclusion`
```rust
const DW_TAG_common_inclusion: DwTag;
```

### `DW_TAG_inheritance`
```rust
const DW_TAG_inheritance: DwTag;
```

### `DW_TAG_inlined_subroutine`
```rust
const DW_TAG_inlined_subroutine: DwTag;
```

### `DW_TAG_module`
```rust
const DW_TAG_module: DwTag;
```

### `DW_TAG_ptr_to_member_type`
```rust
const DW_TAG_ptr_to_member_type: DwTag;
```

### `DW_TAG_set_type`
```rust
const DW_TAG_set_type: DwTag;
```

### `DW_TAG_subrange_type`
```rust
const DW_TAG_subrange_type: DwTag;
```

### `DW_TAG_with_stmt`
```rust
const DW_TAG_with_stmt: DwTag;
```

### `DW_TAG_access_declaration`
```rust
const DW_TAG_access_declaration: DwTag;
```

### `DW_TAG_base_type`
```rust
const DW_TAG_base_type: DwTag;
```

### `DW_TAG_catch_block`
```rust
const DW_TAG_catch_block: DwTag;
```

### `DW_TAG_const_type`
```rust
const DW_TAG_const_type: DwTag;
```

### `DW_TAG_constant`
```rust
const DW_TAG_constant: DwTag;
```

### `DW_TAG_enumerator`
```rust
const DW_TAG_enumerator: DwTag;
```

### `DW_TAG_file_type`
```rust
const DW_TAG_file_type: DwTag;
```

### `DW_TAG_friend`
```rust
const DW_TAG_friend: DwTag;
```

### `DW_TAG_namelist`
```rust
const DW_TAG_namelist: DwTag;
```

### `DW_TAG_namelist_item`
```rust
const DW_TAG_namelist_item: DwTag;
```

### `DW_TAG_packed_type`
```rust
const DW_TAG_packed_type: DwTag;
```

### `DW_TAG_subprogram`
```rust
const DW_TAG_subprogram: DwTag;
```

### `DW_TAG_template_type_parameter`
```rust
const DW_TAG_template_type_parameter: DwTag;
```

### `DW_TAG_template_value_parameter`
```rust
const DW_TAG_template_value_parameter: DwTag;
```

### `DW_TAG_thrown_type`
```rust
const DW_TAG_thrown_type: DwTag;
```

### `DW_TAG_try_block`
```rust
const DW_TAG_try_block: DwTag;
```

### `DW_TAG_variant_part`
```rust
const DW_TAG_variant_part: DwTag;
```

### `DW_TAG_variable`
```rust
const DW_TAG_variable: DwTag;
```

### `DW_TAG_volatile_type`
```rust
const DW_TAG_volatile_type: DwTag;
```

### `DW_TAG_dwarf_procedure`
```rust
const DW_TAG_dwarf_procedure: DwTag;
```

### `DW_TAG_restrict_type`
```rust
const DW_TAG_restrict_type: DwTag;
```

### `DW_TAG_interface_type`
```rust
const DW_TAG_interface_type: DwTag;
```

### `DW_TAG_namespace`
```rust
const DW_TAG_namespace: DwTag;
```

### `DW_TAG_imported_module`
```rust
const DW_TAG_imported_module: DwTag;
```

### `DW_TAG_unspecified_type`
```rust
const DW_TAG_unspecified_type: DwTag;
```

### `DW_TAG_partial_unit`
```rust
const DW_TAG_partial_unit: DwTag;
```

### `DW_TAG_imported_unit`
```rust
const DW_TAG_imported_unit: DwTag;
```

### `DW_TAG_condition`
```rust
const DW_TAG_condition: DwTag;
```

### `DW_TAG_shared_type`
```rust
const DW_TAG_shared_type: DwTag;
```

### `DW_TAG_type_unit`
```rust
const DW_TAG_type_unit: DwTag;
```

### `DW_TAG_rvalue_reference_type`
```rust
const DW_TAG_rvalue_reference_type: DwTag;
```

### `DW_TAG_template_alias`
```rust
const DW_TAG_template_alias: DwTag;
```

### `DW_TAG_coarray_type`
```rust
const DW_TAG_coarray_type: DwTag;
```

### `DW_TAG_generic_subrange`
```rust
const DW_TAG_generic_subrange: DwTag;
```

### `DW_TAG_dynamic_type`
```rust
const DW_TAG_dynamic_type: DwTag;
```

### `DW_TAG_atomic_type`
```rust
const DW_TAG_atomic_type: DwTag;
```

### `DW_TAG_call_site`
```rust
const DW_TAG_call_site: DwTag;
```

### `DW_TAG_call_site_parameter`
```rust
const DW_TAG_call_site_parameter: DwTag;
```

### `DW_TAG_skeleton_unit`
```rust
const DW_TAG_skeleton_unit: DwTag;
```

### `DW_TAG_immutable_type`
```rust
const DW_TAG_immutable_type: DwTag;
```

### `DW_TAG_lo_user`
```rust
const DW_TAG_lo_user: DwTag;
```

### `DW_TAG_hi_user`
```rust
const DW_TAG_hi_user: DwTag;
```

### `DW_TAG_MIPS_loop`
```rust
const DW_TAG_MIPS_loop: DwTag;
```

### `DW_TAG_HP_array_descriptor`
```rust
const DW_TAG_HP_array_descriptor: DwTag;
```

### `DW_TAG_HP_Bliss_field`
```rust
const DW_TAG_HP_Bliss_field: DwTag;
```

### `DW_TAG_HP_Bliss_field_set`
```rust
const DW_TAG_HP_Bliss_field_set: DwTag;
```

### `DW_TAG_format_label`
```rust
const DW_TAG_format_label: DwTag;
```

### `DW_TAG_function_template`
```rust
const DW_TAG_function_template: DwTag;
```

### `DW_TAG_class_template`
```rust
const DW_TAG_class_template: DwTag;
```

### `DW_TAG_GNU_BINCL`
```rust
const DW_TAG_GNU_BINCL: DwTag;
```

### `DW_TAG_GNU_EINCL`
```rust
const DW_TAG_GNU_EINCL: DwTag;
```

### `DW_TAG_GNU_template_template_param`
```rust
const DW_TAG_GNU_template_template_param: DwTag;
```

### `DW_TAG_GNU_template_parameter_pack`
```rust
const DW_TAG_GNU_template_parameter_pack: DwTag;
```

### `DW_TAG_GNU_formal_parameter_pack`
```rust
const DW_TAG_GNU_formal_parameter_pack: DwTag;
```

### `DW_TAG_GNU_call_site`
```rust
const DW_TAG_GNU_call_site: DwTag;
```

### `DW_TAG_GNU_call_site_parameter`
```rust
const DW_TAG_GNU_call_site_parameter: DwTag;
```

### `DW_TAG_APPLE_property`
```rust
const DW_TAG_APPLE_property: DwTag;
```

### `DW_TAG_SUN_function_template`
```rust
const DW_TAG_SUN_function_template: DwTag;
```

### `DW_TAG_SUN_class_template`
```rust
const DW_TAG_SUN_class_template: DwTag;
```

### `DW_TAG_SUN_struct_template`
```rust
const DW_TAG_SUN_struct_template: DwTag;
```

### `DW_TAG_SUN_union_template`
```rust
const DW_TAG_SUN_union_template: DwTag;
```

### `DW_TAG_SUN_indirect_inheritance`
```rust
const DW_TAG_SUN_indirect_inheritance: DwTag;
```

### `DW_TAG_SUN_codeflags`
```rust
const DW_TAG_SUN_codeflags: DwTag;
```

### `DW_TAG_SUN_memop_info`
```rust
const DW_TAG_SUN_memop_info: DwTag;
```

### `DW_TAG_SUN_omp_child_func`
```rust
const DW_TAG_SUN_omp_child_func: DwTag;
```

### `DW_TAG_SUN_rtti_descriptor`
```rust
const DW_TAG_SUN_rtti_descriptor: DwTag;
```

### `DW_TAG_SUN_dtor_info`
```rust
const DW_TAG_SUN_dtor_info: DwTag;
```

### `DW_TAG_SUN_dtor`
```rust
const DW_TAG_SUN_dtor: DwTag;
```

### `DW_TAG_SUN_f90_interface`
```rust
const DW_TAG_SUN_f90_interface: DwTag;
```

### `DW_TAG_SUN_fortran_vax_structure`
```rust
const DW_TAG_SUN_fortran_vax_structure: DwTag;
```

### `DW_TAG_ALTIUM_circ_type`
```rust
const DW_TAG_ALTIUM_circ_type: DwTag;
```

### `DW_TAG_ALTIUM_mwa_circ_type`
```rust
const DW_TAG_ALTIUM_mwa_circ_type: DwTag;
```

### `DW_TAG_ALTIUM_rev_carry_type`
```rust
const DW_TAG_ALTIUM_rev_carry_type: DwTag;
```

### `DW_TAG_ALTIUM_rom`
```rust
const DW_TAG_ALTIUM_rom: DwTag;
```

### `DW_TAG_upc_shared_type`
```rust
const DW_TAG_upc_shared_type: DwTag;
```

### `DW_TAG_upc_strict_type`
```rust
const DW_TAG_upc_strict_type: DwTag;
```

### `DW_TAG_upc_relaxed_type`
```rust
const DW_TAG_upc_relaxed_type: DwTag;
```

### `DW_TAG_PGI_kanji_type`
```rust
const DW_TAG_PGI_kanji_type: DwTag;
```

### `DW_TAG_PGI_interface_block`
```rust
const DW_TAG_PGI_interface_block: DwTag;
```

### `DW_TAG_BORLAND_property`
```rust
const DW_TAG_BORLAND_property: DwTag;
```

### `DW_TAG_BORLAND_Delphi_string`
```rust
const DW_TAG_BORLAND_Delphi_string: DwTag;
```

### `DW_TAG_BORLAND_Delphi_dynamic_array`
```rust
const DW_TAG_BORLAND_Delphi_dynamic_array: DwTag;
```

### `DW_TAG_BORLAND_Delphi_set`
```rust
const DW_TAG_BORLAND_Delphi_set: DwTag;
```

### `DW_TAG_BORLAND_Delphi_variant`
```rust
const DW_TAG_BORLAND_Delphi_variant: DwTag;
```

### `DW_AT_null`
```rust
const DW_AT_null: DwAt;
```

### `DW_AT_fund_type`
```rust
const DW_AT_fund_type: DwAt;
```

### `DW_AT_mod_fund_type`
```rust
const DW_AT_mod_fund_type: DwAt;
```

### `DW_AT_user_def_type`
```rust
const DW_AT_user_def_type: DwAt;
```

### `DW_AT_mod_u_d_type`
```rust
const DW_AT_mod_u_d_type: DwAt;
```

### `DW_AT_subscr_data`
```rust
const DW_AT_subscr_data: DwAt;
```

### `DW_AT_element_list`
```rust
const DW_AT_element_list: DwAt;
```

### `DW_AT_member`
```rust
const DW_AT_member: DwAt;
```

### `DW_AT_friends`
```rust
const DW_AT_friends: DwAt;
```

### `DW_AT_program`
```rust
const DW_AT_program: DwAt;
```

### `DW_AT_private`
```rust
const DW_AT_private: DwAt;
```

### `DW_AT_protected`
```rust
const DW_AT_protected: DwAt;
```

### `DW_AT_public`
```rust
const DW_AT_public: DwAt;
```

### `DW_AT_pure_virtual`
```rust
const DW_AT_pure_virtual: DwAt;
```

### `DW_AT_virtual`
```rust
const DW_AT_virtual: DwAt;
```

### `DW_AT_specification_v1`
```rust
const DW_AT_specification_v1: DwAt;
```

### `DW_AT_sibling`
```rust
const DW_AT_sibling: DwAt;
```

### `DW_AT_location`
```rust
const DW_AT_location: DwAt;
```

### `DW_AT_name`
```rust
const DW_AT_name: DwAt;
```

### `DW_AT_ordering`
```rust
const DW_AT_ordering: DwAt;
```

### `DW_AT_byte_size`
```rust
const DW_AT_byte_size: DwAt;
```

### `DW_AT_bit_offset`
```rust
const DW_AT_bit_offset: DwAt;
```

### `DW_AT_bit_size`
```rust
const DW_AT_bit_size: DwAt;
```

### `DW_AT_stmt_list`
```rust
const DW_AT_stmt_list: DwAt;
```

### `DW_AT_low_pc`
```rust
const DW_AT_low_pc: DwAt;
```

### `DW_AT_high_pc`
```rust
const DW_AT_high_pc: DwAt;
```

### `DW_AT_language`
```rust
const DW_AT_language: DwAt;
```

### `DW_AT_discr`
```rust
const DW_AT_discr: DwAt;
```

### `DW_AT_discr_value`
```rust
const DW_AT_discr_value: DwAt;
```

### `DW_AT_visibility`
```rust
const DW_AT_visibility: DwAt;
```

### `DW_AT_import`
```rust
const DW_AT_import: DwAt;
```

### `DW_AT_string_length`
```rust
const DW_AT_string_length: DwAt;
```

### `DW_AT_common_reference`
```rust
const DW_AT_common_reference: DwAt;
```

### `DW_AT_comp_dir`
```rust
const DW_AT_comp_dir: DwAt;
```

### `DW_AT_const_value`
```rust
const DW_AT_const_value: DwAt;
```

### `DW_AT_containing_type`
```rust
const DW_AT_containing_type: DwAt;
```

### `DW_AT_default_value`
```rust
const DW_AT_default_value: DwAt;
```

### `DW_AT_inline`
```rust
const DW_AT_inline: DwAt;
```

### `DW_AT_is_optional`
```rust
const DW_AT_is_optional: DwAt;
```

### `DW_AT_lower_bound`
```rust
const DW_AT_lower_bound: DwAt;
```

### `DW_AT_producer`
```rust
const DW_AT_producer: DwAt;
```

### `DW_AT_prototyped`
```rust
const DW_AT_prototyped: DwAt;
```

### `DW_AT_return_addr`
```rust
const DW_AT_return_addr: DwAt;
```

### `DW_AT_start_scope`
```rust
const DW_AT_start_scope: DwAt;
```

### `DW_AT_bit_stride`
```rust
const DW_AT_bit_stride: DwAt;
```

### `DW_AT_upper_bound`
```rust
const DW_AT_upper_bound: DwAt;
```

### `DW_AT_abstract_origin`
```rust
const DW_AT_abstract_origin: DwAt;
```

### `DW_AT_accessibility`
```rust
const DW_AT_accessibility: DwAt;
```

### `DW_AT_address_class`
```rust
const DW_AT_address_class: DwAt;
```

### `DW_AT_artificial`
```rust
const DW_AT_artificial: DwAt;
```

### `DW_AT_base_types`
```rust
const DW_AT_base_types: DwAt;
```

### `DW_AT_calling_convention`
```rust
const DW_AT_calling_convention: DwAt;
```

### `DW_AT_count`
```rust
const DW_AT_count: DwAt;
```

### `DW_AT_data_member_location`
```rust
const DW_AT_data_member_location: DwAt;
```

### `DW_AT_decl_column`
```rust
const DW_AT_decl_column: DwAt;
```

### `DW_AT_decl_file`
```rust
const DW_AT_decl_file: DwAt;
```

### `DW_AT_decl_line`
```rust
const DW_AT_decl_line: DwAt;
```

### `DW_AT_declaration`
```rust
const DW_AT_declaration: DwAt;
```

### `DW_AT_discr_list`
```rust
const DW_AT_discr_list: DwAt;
```

### `DW_AT_encoding`
```rust
const DW_AT_encoding: DwAt;
```

### `DW_AT_external`
```rust
const DW_AT_external: DwAt;
```

### `DW_AT_frame_base`
```rust
const DW_AT_frame_base: DwAt;
```

### `DW_AT_friend`
```rust
const DW_AT_friend: DwAt;
```

### `DW_AT_identifier_case`
```rust
const DW_AT_identifier_case: DwAt;
```

### `DW_AT_macro_info`
```rust
const DW_AT_macro_info: DwAt;
```

### `DW_AT_namelist_item`
```rust
const DW_AT_namelist_item: DwAt;
```

### `DW_AT_priority`
```rust
const DW_AT_priority: DwAt;
```

### `DW_AT_segment`
```rust
const DW_AT_segment: DwAt;
```

### `DW_AT_specification`
```rust
const DW_AT_specification: DwAt;
```

### `DW_AT_static_link`
```rust
const DW_AT_static_link: DwAt;
```

### `DW_AT_type`
```rust
const DW_AT_type: DwAt;
```

### `DW_AT_use_location`
```rust
const DW_AT_use_location: DwAt;
```

### `DW_AT_variable_parameter`
```rust
const DW_AT_variable_parameter: DwAt;
```

### `DW_AT_virtuality`
```rust
const DW_AT_virtuality: DwAt;
```

### `DW_AT_vtable_elem_location`
```rust
const DW_AT_vtable_elem_location: DwAt;
```

### `DW_AT_allocated`
```rust
const DW_AT_allocated: DwAt;
```

### `DW_AT_associated`
```rust
const DW_AT_associated: DwAt;
```

### `DW_AT_data_location`
```rust
const DW_AT_data_location: DwAt;
```

### `DW_AT_byte_stride`
```rust
const DW_AT_byte_stride: DwAt;
```

### `DW_AT_entry_pc`
```rust
const DW_AT_entry_pc: DwAt;
```

### `DW_AT_use_UTF8`
```rust
const DW_AT_use_UTF8: DwAt;
```

### `DW_AT_extension`
```rust
const DW_AT_extension: DwAt;
```

### `DW_AT_ranges`
```rust
const DW_AT_ranges: DwAt;
```

### `DW_AT_trampoline`
```rust
const DW_AT_trampoline: DwAt;
```

### `DW_AT_call_column`
```rust
const DW_AT_call_column: DwAt;
```

### `DW_AT_call_file`
```rust
const DW_AT_call_file: DwAt;
```

### `DW_AT_call_line`
```rust
const DW_AT_call_line: DwAt;
```

### `DW_AT_description`
```rust
const DW_AT_description: DwAt;
```

### `DW_AT_binary_scale`
```rust
const DW_AT_binary_scale: DwAt;
```

### `DW_AT_decimal_scale`
```rust
const DW_AT_decimal_scale: DwAt;
```

### `DW_AT_small`
```rust
const DW_AT_small: DwAt;
```

### `DW_AT_decimal_sign`
```rust
const DW_AT_decimal_sign: DwAt;
```

### `DW_AT_digit_count`
```rust
const DW_AT_digit_count: DwAt;
```

### `DW_AT_picture_string`
```rust
const DW_AT_picture_string: DwAt;
```

### `DW_AT_mutable`
```rust
const DW_AT_mutable: DwAt;
```

### `DW_AT_threads_scaled`
```rust
const DW_AT_threads_scaled: DwAt;
```

### `DW_AT_explicit`
```rust
const DW_AT_explicit: DwAt;
```

### `DW_AT_object_pointer`
```rust
const DW_AT_object_pointer: DwAt;
```

### `DW_AT_endianity`
```rust
const DW_AT_endianity: DwAt;
```

### `DW_AT_elemental`
```rust
const DW_AT_elemental: DwAt;
```

### `DW_AT_pure`
```rust
const DW_AT_pure: DwAt;
```

### `DW_AT_recursive`
```rust
const DW_AT_recursive: DwAt;
```

### `DW_AT_signature`
```rust
const DW_AT_signature: DwAt;
```

### `DW_AT_main_subprogram`
```rust
const DW_AT_main_subprogram: DwAt;
```

### `DW_AT_data_bit_offset`
```rust
const DW_AT_data_bit_offset: DwAt;
```

### `DW_AT_const_expr`
```rust
const DW_AT_const_expr: DwAt;
```

### `DW_AT_enum_class`
```rust
const DW_AT_enum_class: DwAt;
```

### `DW_AT_linkage_name`
```rust
const DW_AT_linkage_name: DwAt;
```

### `DW_AT_string_length_bit_size`
```rust
const DW_AT_string_length_bit_size: DwAt;
```

### `DW_AT_string_length_byte_size`
```rust
const DW_AT_string_length_byte_size: DwAt;
```

### `DW_AT_rank`
```rust
const DW_AT_rank: DwAt;
```

### `DW_AT_str_offsets_base`
```rust
const DW_AT_str_offsets_base: DwAt;
```

### `DW_AT_addr_base`
```rust
const DW_AT_addr_base: DwAt;
```

### `DW_AT_rnglists_base`
```rust
const DW_AT_rnglists_base: DwAt;
```

### `DW_AT_dwo_name`
```rust
const DW_AT_dwo_name: DwAt;
```

### `DW_AT_reference`
```rust
const DW_AT_reference: DwAt;
```

### `DW_AT_rvalue_reference`
```rust
const DW_AT_rvalue_reference: DwAt;
```

### `DW_AT_macros`
```rust
const DW_AT_macros: DwAt;
```

### `DW_AT_call_all_calls`
```rust
const DW_AT_call_all_calls: DwAt;
```

### `DW_AT_call_all_source_calls`
```rust
const DW_AT_call_all_source_calls: DwAt;
```

### `DW_AT_call_all_tail_calls`
```rust
const DW_AT_call_all_tail_calls: DwAt;
```

### `DW_AT_call_return_pc`
```rust
const DW_AT_call_return_pc: DwAt;
```

### `DW_AT_call_value`
```rust
const DW_AT_call_value: DwAt;
```

### `DW_AT_call_origin`
```rust
const DW_AT_call_origin: DwAt;
```

### `DW_AT_call_parameter`
```rust
const DW_AT_call_parameter: DwAt;
```

### `DW_AT_call_pc`
```rust
const DW_AT_call_pc: DwAt;
```

### `DW_AT_call_tail_call`
```rust
const DW_AT_call_tail_call: DwAt;
```

### `DW_AT_call_target`
```rust
const DW_AT_call_target: DwAt;
```

### `DW_AT_call_target_clobbered`
```rust
const DW_AT_call_target_clobbered: DwAt;
```

### `DW_AT_call_data_location`
```rust
const DW_AT_call_data_location: DwAt;
```

### `DW_AT_call_data_value`
```rust
const DW_AT_call_data_value: DwAt;
```

### `DW_AT_noreturn`
```rust
const DW_AT_noreturn: DwAt;
```

### `DW_AT_alignment`
```rust
const DW_AT_alignment: DwAt;
```

### `DW_AT_export_symbols`
```rust
const DW_AT_export_symbols: DwAt;
```

### `DW_AT_deleted`
```rust
const DW_AT_deleted: DwAt;
```

### `DW_AT_defaulted`
```rust
const DW_AT_defaulted: DwAt;
```

### `DW_AT_loclists_base`
```rust
const DW_AT_loclists_base: DwAt;
```

### `DW_AT_lo_user`
```rust
const DW_AT_lo_user: DwAt;
```

### `DW_AT_hi_user`
```rust
const DW_AT_hi_user: DwAt;
```

### `DW_AT_MIPS_fde`
```rust
const DW_AT_MIPS_fde: DwAt;
```

### `DW_AT_MIPS_loop_begin`
```rust
const DW_AT_MIPS_loop_begin: DwAt;
```

### `DW_AT_MIPS_tail_loop_begin`
```rust
const DW_AT_MIPS_tail_loop_begin: DwAt;
```

### `DW_AT_MIPS_epilog_begin`
```rust
const DW_AT_MIPS_epilog_begin: DwAt;
```

### `DW_AT_MIPS_loop_unroll_factor`
```rust
const DW_AT_MIPS_loop_unroll_factor: DwAt;
```

### `DW_AT_MIPS_software_pipeline_depth`
```rust
const DW_AT_MIPS_software_pipeline_depth: DwAt;
```

### `DW_AT_MIPS_linkage_name`
```rust
const DW_AT_MIPS_linkage_name: DwAt;
```

### `DW_AT_MIPS_stride`
```rust
const DW_AT_MIPS_stride: DwAt;
```

### `DW_AT_MIPS_abstract_name`
```rust
const DW_AT_MIPS_abstract_name: DwAt;
```

### `DW_AT_MIPS_clone_origin`
```rust
const DW_AT_MIPS_clone_origin: DwAt;
```

### `DW_AT_MIPS_has_inlines`
```rust
const DW_AT_MIPS_has_inlines: DwAt;
```

### `DW_AT_MIPS_stride_byte`
```rust
const DW_AT_MIPS_stride_byte: DwAt;
```

### `DW_AT_MIPS_stride_elem`
```rust
const DW_AT_MIPS_stride_elem: DwAt;
```

### `DW_AT_MIPS_ptr_dopetype`
```rust
const DW_AT_MIPS_ptr_dopetype: DwAt;
```

### `DW_AT_MIPS_allocatable_dopetype`
```rust
const DW_AT_MIPS_allocatable_dopetype: DwAt;
```

### `DW_AT_MIPS_assumed_shape_dopetype`
```rust
const DW_AT_MIPS_assumed_shape_dopetype: DwAt;
```

### `DW_AT_MIPS_assumed_size`
```rust
const DW_AT_MIPS_assumed_size: DwAt;
```

### `DW_AT_INTEL_other_endian`
```rust
const DW_AT_INTEL_other_endian: DwAt;
```

### `DW_AT_sf_names`
```rust
const DW_AT_sf_names: DwAt;
```

### `DW_AT_src_info`
```rust
const DW_AT_src_info: DwAt;
```

### `DW_AT_mac_info`
```rust
const DW_AT_mac_info: DwAt;
```

### `DW_AT_src_coords`
```rust
const DW_AT_src_coords: DwAt;
```

### `DW_AT_body_begin`
```rust
const DW_AT_body_begin: DwAt;
```

### `DW_AT_body_end`
```rust
const DW_AT_body_end: DwAt;
```

### `DW_AT_GNU_vector`
```rust
const DW_AT_GNU_vector: DwAt;
```

### `DW_AT_GNU_guarded_by`
```rust
const DW_AT_GNU_guarded_by: DwAt;
```

### `DW_AT_GNU_pt_guarded_by`
```rust
const DW_AT_GNU_pt_guarded_by: DwAt;
```

### `DW_AT_GNU_guarded`
```rust
const DW_AT_GNU_guarded: DwAt;
```

### `DW_AT_GNU_pt_guarded`
```rust
const DW_AT_GNU_pt_guarded: DwAt;
```

### `DW_AT_GNU_locks_excluded`
```rust
const DW_AT_GNU_locks_excluded: DwAt;
```

### `DW_AT_GNU_exclusive_locks_required`
```rust
const DW_AT_GNU_exclusive_locks_required: DwAt;
```

### `DW_AT_GNU_shared_locks_required`
```rust
const DW_AT_GNU_shared_locks_required: DwAt;
```

### `DW_AT_GNU_odr_signature`
```rust
const DW_AT_GNU_odr_signature: DwAt;
```

### `DW_AT_GNU_template_name`
```rust
const DW_AT_GNU_template_name: DwAt;
```

### `DW_AT_GNU_call_site_value`
```rust
const DW_AT_GNU_call_site_value: DwAt;
```

### `DW_AT_GNU_call_site_data_value`
```rust
const DW_AT_GNU_call_site_data_value: DwAt;
```

### `DW_AT_GNU_call_site_target`
```rust
const DW_AT_GNU_call_site_target: DwAt;
```

### `DW_AT_GNU_call_site_target_clobbered`
```rust
const DW_AT_GNU_call_site_target_clobbered: DwAt;
```

### `DW_AT_GNU_tail_call`
```rust
const DW_AT_GNU_tail_call: DwAt;
```

### `DW_AT_GNU_all_tail_call_sites`
```rust
const DW_AT_GNU_all_tail_call_sites: DwAt;
```

### `DW_AT_GNU_all_call_sites`
```rust
const DW_AT_GNU_all_call_sites: DwAt;
```

### `DW_AT_GNU_all_source_call_sites`
```rust
const DW_AT_GNU_all_source_call_sites: DwAt;
```

### `DW_AT_GNU_macros`
```rust
const DW_AT_GNU_macros: DwAt;
```

### `DW_AT_GNU_deleted`
```rust
const DW_AT_GNU_deleted: DwAt;
```

### `DW_AT_GNU_dwo_name`
```rust
const DW_AT_GNU_dwo_name: DwAt;
```

### `DW_AT_GNU_dwo_id`
```rust
const DW_AT_GNU_dwo_id: DwAt;
```

### `DW_AT_GNU_ranges_base`
```rust
const DW_AT_GNU_ranges_base: DwAt;
```

### `DW_AT_GNU_addr_base`
```rust
const DW_AT_GNU_addr_base: DwAt;
```

### `DW_AT_GNU_pubnames`
```rust
const DW_AT_GNU_pubnames: DwAt;
```

### `DW_AT_GNU_pubtypes`
```rust
const DW_AT_GNU_pubtypes: DwAt;
```

### `DW_AT_GNU_discriminator`
```rust
const DW_AT_GNU_discriminator: DwAt;
```

### `DW_AT_GNU_locviews`
```rust
const DW_AT_GNU_locviews: DwAt;
```

### `DW_AT_GNU_entry_view`
```rust
const DW_AT_GNU_entry_view: DwAt;
```

### `DW_AT_SUN_template`
```rust
const DW_AT_SUN_template: DwAt;
```

### `DW_AT_SUN_alignment`
```rust
const DW_AT_SUN_alignment: DwAt;
```

### `DW_AT_SUN_vtable`
```rust
const DW_AT_SUN_vtable: DwAt;
```

### `DW_AT_SUN_count_guarantee`
```rust
const DW_AT_SUN_count_guarantee: DwAt;
```

### `DW_AT_SUN_command_line`
```rust
const DW_AT_SUN_command_line: DwAt;
```

### `DW_AT_SUN_vbase`
```rust
const DW_AT_SUN_vbase: DwAt;
```

### `DW_AT_SUN_compile_options`
```rust
const DW_AT_SUN_compile_options: DwAt;
```

### `DW_AT_SUN_language`
```rust
const DW_AT_SUN_language: DwAt;
```

### `DW_AT_SUN_browser_file`
```rust
const DW_AT_SUN_browser_file: DwAt;
```

### `DW_AT_SUN_vtable_abi`
```rust
const DW_AT_SUN_vtable_abi: DwAt;
```

### `DW_AT_SUN_func_offsets`
```rust
const DW_AT_SUN_func_offsets: DwAt;
```

### `DW_AT_SUN_cf_kind`
```rust
const DW_AT_SUN_cf_kind: DwAt;
```

### `DW_AT_SUN_vtable_index`
```rust
const DW_AT_SUN_vtable_index: DwAt;
```

### `DW_AT_SUN_omp_tpriv_addr`
```rust
const DW_AT_SUN_omp_tpriv_addr: DwAt;
```

### `DW_AT_SUN_omp_child_func`
```rust
const DW_AT_SUN_omp_child_func: DwAt;
```

### `DW_AT_SUN_func_offset`
```rust
const DW_AT_SUN_func_offset: DwAt;
```

### `DW_AT_SUN_memop_type_ref`
```rust
const DW_AT_SUN_memop_type_ref: DwAt;
```

### `DW_AT_SUN_profile_id`
```rust
const DW_AT_SUN_profile_id: DwAt;
```

### `DW_AT_SUN_memop_signature`
```rust
const DW_AT_SUN_memop_signature: DwAt;
```

### `DW_AT_SUN_obj_dir`
```rust
const DW_AT_SUN_obj_dir: DwAt;
```

### `DW_AT_SUN_obj_file`
```rust
const DW_AT_SUN_obj_file: DwAt;
```

### `DW_AT_SUN_original_name`
```rust
const DW_AT_SUN_original_name: DwAt;
```

### `DW_AT_SUN_hwcprof_signature`
```rust
const DW_AT_SUN_hwcprof_signature: DwAt;
```

### `DW_AT_SUN_amd64_parmdump`
```rust
const DW_AT_SUN_amd64_parmdump: DwAt;
```

### `DW_AT_SUN_part_link_name`
```rust
const DW_AT_SUN_part_link_name: DwAt;
```

### `DW_AT_SUN_link_name`
```rust
const DW_AT_SUN_link_name: DwAt;
```

### `DW_AT_SUN_pass_with_const`
```rust
const DW_AT_SUN_pass_with_const: DwAt;
```

### `DW_AT_SUN_return_with_const`
```rust
const DW_AT_SUN_return_with_const: DwAt;
```

### `DW_AT_SUN_import_by_name`
```rust
const DW_AT_SUN_import_by_name: DwAt;
```

### `DW_AT_SUN_f90_pointer`
```rust
const DW_AT_SUN_f90_pointer: DwAt;
```

### `DW_AT_SUN_pass_by_ref`
```rust
const DW_AT_SUN_pass_by_ref: DwAt;
```

### `DW_AT_SUN_f90_allocatable`
```rust
const DW_AT_SUN_f90_allocatable: DwAt;
```

### `DW_AT_SUN_f90_assumed_shape_array`
```rust
const DW_AT_SUN_f90_assumed_shape_array: DwAt;
```

### `DW_AT_SUN_c_vla`
```rust
const DW_AT_SUN_c_vla: DwAt;
```

### `DW_AT_SUN_return_value_ptr`
```rust
const DW_AT_SUN_return_value_ptr: DwAt;
```

### `DW_AT_SUN_dtor_start`
```rust
const DW_AT_SUN_dtor_start: DwAt;
```

### `DW_AT_SUN_dtor_length`
```rust
const DW_AT_SUN_dtor_length: DwAt;
```

### `DW_AT_SUN_dtor_state_initial`
```rust
const DW_AT_SUN_dtor_state_initial: DwAt;
```

### `DW_AT_SUN_dtor_state_final`
```rust
const DW_AT_SUN_dtor_state_final: DwAt;
```

### `DW_AT_SUN_dtor_state_deltas`
```rust
const DW_AT_SUN_dtor_state_deltas: DwAt;
```

### `DW_AT_SUN_import_by_lname`
```rust
const DW_AT_SUN_import_by_lname: DwAt;
```

### `DW_AT_SUN_f90_use_only`
```rust
const DW_AT_SUN_f90_use_only: DwAt;
```

### `DW_AT_SUN_namelist_spec`
```rust
const DW_AT_SUN_namelist_spec: DwAt;
```

### `DW_AT_SUN_is_omp_child_func`
```rust
const DW_AT_SUN_is_omp_child_func: DwAt;
```

### `DW_AT_SUN_fortran_main_alias`
```rust
const DW_AT_SUN_fortran_main_alias: DwAt;
```

### `DW_AT_SUN_fortran_based`
```rust
const DW_AT_SUN_fortran_based: DwAt;
```

### `DW_AT_ALTIUM_loclist`
```rust
const DW_AT_ALTIUM_loclist: DwAt;
```

### `DW_AT_use_GNAT_descriptive_type`
```rust
const DW_AT_use_GNAT_descriptive_type: DwAt;
```

### `DW_AT_GNAT_descriptive_type`
```rust
const DW_AT_GNAT_descriptive_type: DwAt;
```

### `DW_AT_GNU_numerator`
```rust
const DW_AT_GNU_numerator: DwAt;
```

### `DW_AT_GNU_denominator`
```rust
const DW_AT_GNU_denominator: DwAt;
```

### `DW_AT_GNU_bias`
```rust
const DW_AT_GNU_bias: DwAt;
```

### `DW_AT_upc_threads_scaled`
```rust
const DW_AT_upc_threads_scaled: DwAt;
```

### `DW_AT_PGI_lbase`
```rust
const DW_AT_PGI_lbase: DwAt;
```

### `DW_AT_PGI_soffset`
```rust
const DW_AT_PGI_soffset: DwAt;
```

### `DW_AT_PGI_lstride`
```rust
const DW_AT_PGI_lstride: DwAt;
```

### `DW_AT_BORLAND_property_read`
```rust
const DW_AT_BORLAND_property_read: DwAt;
```

### `DW_AT_BORLAND_property_write`
```rust
const DW_AT_BORLAND_property_write: DwAt;
```

### `DW_AT_BORLAND_property_implements`
```rust
const DW_AT_BORLAND_property_implements: DwAt;
```

### `DW_AT_BORLAND_property_index`
```rust
const DW_AT_BORLAND_property_index: DwAt;
```

### `DW_AT_BORLAND_property_default`
```rust
const DW_AT_BORLAND_property_default: DwAt;
```

### `DW_AT_BORLAND_Delphi_unit`
```rust
const DW_AT_BORLAND_Delphi_unit: DwAt;
```

### `DW_AT_BORLAND_Delphi_class`
```rust
const DW_AT_BORLAND_Delphi_class: DwAt;
```

### `DW_AT_BORLAND_Delphi_record`
```rust
const DW_AT_BORLAND_Delphi_record: DwAt;
```

### `DW_AT_BORLAND_Delphi_metaclass`
```rust
const DW_AT_BORLAND_Delphi_metaclass: DwAt;
```

### `DW_AT_BORLAND_Delphi_constructor`
```rust
const DW_AT_BORLAND_Delphi_constructor: DwAt;
```

### `DW_AT_BORLAND_Delphi_destructor`
```rust
const DW_AT_BORLAND_Delphi_destructor: DwAt;
```

### `DW_AT_BORLAND_Delphi_anonymous_method`
```rust
const DW_AT_BORLAND_Delphi_anonymous_method: DwAt;
```

### `DW_AT_BORLAND_Delphi_interface`
```rust
const DW_AT_BORLAND_Delphi_interface: DwAt;
```

### `DW_AT_BORLAND_Delphi_ABI`
```rust
const DW_AT_BORLAND_Delphi_ABI: DwAt;
```

### `DW_AT_BORLAND_Delphi_return`
```rust
const DW_AT_BORLAND_Delphi_return: DwAt;
```

### `DW_AT_BORLAND_Delphi_frameptr`
```rust
const DW_AT_BORLAND_Delphi_frameptr: DwAt;
```

### `DW_AT_BORLAND_closure`
```rust
const DW_AT_BORLAND_closure: DwAt;
```

### `DW_AT_LLVM_include_path`
```rust
const DW_AT_LLVM_include_path: DwAt;
```

### `DW_AT_LLVM_config_macros`
```rust
const DW_AT_LLVM_config_macros: DwAt;
```

### `DW_AT_LLVM_isysroot`
```rust
const DW_AT_LLVM_isysroot: DwAt;
```

### `DW_AT_APPLE_optimized`
```rust
const DW_AT_APPLE_optimized: DwAt;
```

### `DW_AT_APPLE_flags`
```rust
const DW_AT_APPLE_flags: DwAt;
```

### `DW_AT_APPLE_isa`
```rust
const DW_AT_APPLE_isa: DwAt;
```

### `DW_AT_APPLE_block`
```rust
const DW_AT_APPLE_block: DwAt;
```

### `DW_AT_APPLE_major_runtime_vers`
```rust
const DW_AT_APPLE_major_runtime_vers: DwAt;
```

### `DW_AT_APPLE_runtime_class`
```rust
const DW_AT_APPLE_runtime_class: DwAt;
```

### `DW_AT_APPLE_omit_frame_ptr`
```rust
const DW_AT_APPLE_omit_frame_ptr: DwAt;
```

### `DW_AT_APPLE_property_name`
```rust
const DW_AT_APPLE_property_name: DwAt;
```

### `DW_AT_APPLE_property_getter`
```rust
const DW_AT_APPLE_property_getter: DwAt;
```

### `DW_AT_APPLE_property_setter`
```rust
const DW_AT_APPLE_property_setter: DwAt;
```

### `DW_AT_APPLE_property_attribute`
```rust
const DW_AT_APPLE_property_attribute: DwAt;
```

### `DW_AT_APPLE_objc_complete_type`
```rust
const DW_AT_APPLE_objc_complete_type: DwAt;
```

### `DW_AT_APPLE_property`
```rust
const DW_AT_APPLE_property: DwAt;
```

### `DW_FORM_null`
```rust
const DW_FORM_null: DwForm;
```

### `DW_FORM_ref`
```rust
const DW_FORM_ref: DwForm;
```

### `DW_FORM_addr`
```rust
const DW_FORM_addr: DwForm;
```

### `DW_FORM_block2`
```rust
const DW_FORM_block2: DwForm;
```

### `DW_FORM_block4`
```rust
const DW_FORM_block4: DwForm;
```

### `DW_FORM_data2`
```rust
const DW_FORM_data2: DwForm;
```

### `DW_FORM_data4`
```rust
const DW_FORM_data4: DwForm;
```

### `DW_FORM_data8`
```rust
const DW_FORM_data8: DwForm;
```

### `DW_FORM_string`
```rust
const DW_FORM_string: DwForm;
```

### `DW_FORM_block`
```rust
const DW_FORM_block: DwForm;
```

### `DW_FORM_block1`
```rust
const DW_FORM_block1: DwForm;
```

### `DW_FORM_data1`
```rust
const DW_FORM_data1: DwForm;
```

### `DW_FORM_flag`
```rust
const DW_FORM_flag: DwForm;
```

### `DW_FORM_sdata`
```rust
const DW_FORM_sdata: DwForm;
```

### `DW_FORM_strp`
```rust
const DW_FORM_strp: DwForm;
```

### `DW_FORM_udata`
```rust
const DW_FORM_udata: DwForm;
```

### `DW_FORM_ref_addr`
```rust
const DW_FORM_ref_addr: DwForm;
```

### `DW_FORM_ref1`
```rust
const DW_FORM_ref1: DwForm;
```

### `DW_FORM_ref2`
```rust
const DW_FORM_ref2: DwForm;
```

### `DW_FORM_ref4`
```rust
const DW_FORM_ref4: DwForm;
```

### `DW_FORM_ref8`
```rust
const DW_FORM_ref8: DwForm;
```

### `DW_FORM_ref_udata`
```rust
const DW_FORM_ref_udata: DwForm;
```

### `DW_FORM_indirect`
```rust
const DW_FORM_indirect: DwForm;
```

### `DW_FORM_sec_offset`
```rust
const DW_FORM_sec_offset: DwForm;
```

### `DW_FORM_exprloc`
```rust
const DW_FORM_exprloc: DwForm;
```

### `DW_FORM_flag_present`
```rust
const DW_FORM_flag_present: DwForm;
```

### `DW_FORM_ref_sig8`
```rust
const DW_FORM_ref_sig8: DwForm;
```

### `DW_FORM_strx`
```rust
const DW_FORM_strx: DwForm;
```

### `DW_FORM_addrx`
```rust
const DW_FORM_addrx: DwForm;
```

### `DW_FORM_ref_sup4`
```rust
const DW_FORM_ref_sup4: DwForm;
```

### `DW_FORM_strp_sup`
```rust
const DW_FORM_strp_sup: DwForm;
```

### `DW_FORM_data16`
```rust
const DW_FORM_data16: DwForm;
```

### `DW_FORM_line_strp`
```rust
const DW_FORM_line_strp: DwForm;
```

### `DW_FORM_implicit_const`
```rust
const DW_FORM_implicit_const: DwForm;
```

### `DW_FORM_loclistx`
```rust
const DW_FORM_loclistx: DwForm;
```

### `DW_FORM_rnglistx`
```rust
const DW_FORM_rnglistx: DwForm;
```

### `DW_FORM_ref_sup8`
```rust
const DW_FORM_ref_sup8: DwForm;
```

### `DW_FORM_strx1`
```rust
const DW_FORM_strx1: DwForm;
```

### `DW_FORM_strx2`
```rust
const DW_FORM_strx2: DwForm;
```

### `DW_FORM_strx3`
```rust
const DW_FORM_strx3: DwForm;
```

### `DW_FORM_strx4`
```rust
const DW_FORM_strx4: DwForm;
```

### `DW_FORM_addrx1`
```rust
const DW_FORM_addrx1: DwForm;
```

### `DW_FORM_addrx2`
```rust
const DW_FORM_addrx2: DwForm;
```

### `DW_FORM_addrx3`
```rust
const DW_FORM_addrx3: DwForm;
```

### `DW_FORM_addrx4`
```rust
const DW_FORM_addrx4: DwForm;
```

### `DW_FORM_GNU_addr_index`
```rust
const DW_FORM_GNU_addr_index: DwForm;
```

### `DW_FORM_GNU_str_index`
```rust
const DW_FORM_GNU_str_index: DwForm;
```

### `DW_FORM_GNU_ref_alt`
```rust
const DW_FORM_GNU_ref_alt: DwForm;
```

### `DW_FORM_GNU_strp_alt`
```rust
const DW_FORM_GNU_strp_alt: DwForm;
```

### `DW_ATE_address`
```rust
const DW_ATE_address: DwAte;
```

### `DW_ATE_boolean`
```rust
const DW_ATE_boolean: DwAte;
```

### `DW_ATE_complex_float`
```rust
const DW_ATE_complex_float: DwAte;
```

### `DW_ATE_float`
```rust
const DW_ATE_float: DwAte;
```

### `DW_ATE_signed`
```rust
const DW_ATE_signed: DwAte;
```

### `DW_ATE_signed_char`
```rust
const DW_ATE_signed_char: DwAte;
```

### `DW_ATE_unsigned`
```rust
const DW_ATE_unsigned: DwAte;
```

### `DW_ATE_unsigned_char`
```rust
const DW_ATE_unsigned_char: DwAte;
```

### `DW_ATE_imaginary_float`
```rust
const DW_ATE_imaginary_float: DwAte;
```

### `DW_ATE_packed_decimal`
```rust
const DW_ATE_packed_decimal: DwAte;
```

### `DW_ATE_numeric_string`
```rust
const DW_ATE_numeric_string: DwAte;
```

### `DW_ATE_edited`
```rust
const DW_ATE_edited: DwAte;
```

### `DW_ATE_signed_fixed`
```rust
const DW_ATE_signed_fixed: DwAte;
```

### `DW_ATE_unsigned_fixed`
```rust
const DW_ATE_unsigned_fixed: DwAte;
```

### `DW_ATE_decimal_float`
```rust
const DW_ATE_decimal_float: DwAte;
```

### `DW_ATE_UTF`
```rust
const DW_ATE_UTF: DwAte;
```

### `DW_ATE_UCS`
```rust
const DW_ATE_UCS: DwAte;
```

### `DW_ATE_ASCII`
```rust
const DW_ATE_ASCII: DwAte;
```

### `DW_ATE_lo_user`
```rust
const DW_ATE_lo_user: DwAte;
```

### `DW_ATE_hi_user`
```rust
const DW_ATE_hi_user: DwAte;
```

### `DW_LLE_end_of_list`
```rust
const DW_LLE_end_of_list: DwLle;
```

### `DW_LLE_base_addressx`
```rust
const DW_LLE_base_addressx: DwLle;
```

### `DW_LLE_startx_endx`
```rust
const DW_LLE_startx_endx: DwLle;
```

### `DW_LLE_startx_length`
```rust
const DW_LLE_startx_length: DwLle;
```

### `DW_LLE_offset_pair`
```rust
const DW_LLE_offset_pair: DwLle;
```

### `DW_LLE_default_location`
```rust
const DW_LLE_default_location: DwLle;
```

### `DW_LLE_base_address`
```rust
const DW_LLE_base_address: DwLle;
```

### `DW_LLE_start_end`
```rust
const DW_LLE_start_end: DwLle;
```

### `DW_LLE_start_length`
```rust
const DW_LLE_start_length: DwLle;
```

### `DW_LLE_GNU_view_pair`
```rust
const DW_LLE_GNU_view_pair: DwLle;
```

### `DW_DS_unsigned`
```rust
const DW_DS_unsigned: DwDs;
```

### `DW_DS_leading_overpunch`
```rust
const DW_DS_leading_overpunch: DwDs;
```

### `DW_DS_trailing_overpunch`
```rust
const DW_DS_trailing_overpunch: DwDs;
```

### `DW_DS_leading_separate`
```rust
const DW_DS_leading_separate: DwDs;
```

### `DW_DS_trailing_separate`
```rust
const DW_DS_trailing_separate: DwDs;
```

### `DW_END_default`
```rust
const DW_END_default: DwEnd;
```

### `DW_END_big`
```rust
const DW_END_big: DwEnd;
```

### `DW_END_little`
```rust
const DW_END_little: DwEnd;
```

### `DW_END_lo_user`
```rust
const DW_END_lo_user: DwEnd;
```

### `DW_END_hi_user`
```rust
const DW_END_hi_user: DwEnd;
```

### `DW_ACCESS_public`
```rust
const DW_ACCESS_public: DwAccess;
```

### `DW_ACCESS_protected`
```rust
const DW_ACCESS_protected: DwAccess;
```

### `DW_ACCESS_private`
```rust
const DW_ACCESS_private: DwAccess;
```

### `DW_VIS_local`
```rust
const DW_VIS_local: DwVis;
```

### `DW_VIS_exported`
```rust
const DW_VIS_exported: DwVis;
```

### `DW_VIS_qualified`
```rust
const DW_VIS_qualified: DwVis;
```

### `DW_VIRTUALITY_none`
```rust
const DW_VIRTUALITY_none: DwVirtuality;
```

### `DW_VIRTUALITY_virtual`
```rust
const DW_VIRTUALITY_virtual: DwVirtuality;
```

### `DW_VIRTUALITY_pure_virtual`
```rust
const DW_VIRTUALITY_pure_virtual: DwVirtuality;
```

### `DW_LANG_C89`
```rust
const DW_LANG_C89: DwLang;
```

### `DW_LANG_C`
```rust
const DW_LANG_C: DwLang;
```

### `DW_LANG_Ada83`
```rust
const DW_LANG_Ada83: DwLang;
```

### `DW_LANG_C_plus_plus`
```rust
const DW_LANG_C_plus_plus: DwLang;
```

### `DW_LANG_Cobol74`
```rust
const DW_LANG_Cobol74: DwLang;
```

### `DW_LANG_Cobol85`
```rust
const DW_LANG_Cobol85: DwLang;
```

### `DW_LANG_Fortran77`
```rust
const DW_LANG_Fortran77: DwLang;
```

### `DW_LANG_Fortran90`
```rust
const DW_LANG_Fortran90: DwLang;
```

### `DW_LANG_Pascal83`
```rust
const DW_LANG_Pascal83: DwLang;
```

### `DW_LANG_Modula2`
```rust
const DW_LANG_Modula2: DwLang;
```

### `DW_LANG_Java`
```rust
const DW_LANG_Java: DwLang;
```

### `DW_LANG_C99`
```rust
const DW_LANG_C99: DwLang;
```

### `DW_LANG_Ada95`
```rust
const DW_LANG_Ada95: DwLang;
```

### `DW_LANG_Fortran95`
```rust
const DW_LANG_Fortran95: DwLang;
```

### `DW_LANG_PLI`
```rust
const DW_LANG_PLI: DwLang;
```

### `DW_LANG_ObjC`
```rust
const DW_LANG_ObjC: DwLang;
```

### `DW_LANG_ObjC_plus_plus`
```rust
const DW_LANG_ObjC_plus_plus: DwLang;
```

### `DW_LANG_UPC`
```rust
const DW_LANG_UPC: DwLang;
```

### `DW_LANG_D`
```rust
const DW_LANG_D: DwLang;
```

### `DW_LANG_Python`
```rust
const DW_LANG_Python: DwLang;
```

### `DW_LANG_OpenCL`
```rust
const DW_LANG_OpenCL: DwLang;
```

### `DW_LANG_Go`
```rust
const DW_LANG_Go: DwLang;
```

### `DW_LANG_Modula3`
```rust
const DW_LANG_Modula3: DwLang;
```

### `DW_LANG_Haskell`
```rust
const DW_LANG_Haskell: DwLang;
```

### `DW_LANG_C_plus_plus_03`
```rust
const DW_LANG_C_plus_plus_03: DwLang;
```

### `DW_LANG_C_plus_plus_11`
```rust
const DW_LANG_C_plus_plus_11: DwLang;
```

### `DW_LANG_OCaml`
```rust
const DW_LANG_OCaml: DwLang;
```

### `DW_LANG_Rust`
```rust
const DW_LANG_Rust: DwLang;
```

### `DW_LANG_C11`
```rust
const DW_LANG_C11: DwLang;
```

### `DW_LANG_Swift`
```rust
const DW_LANG_Swift: DwLang;
```

### `DW_LANG_Julia`
```rust
const DW_LANG_Julia: DwLang;
```

### `DW_LANG_Dylan`
```rust
const DW_LANG_Dylan: DwLang;
```

### `DW_LANG_C_plus_plus_14`
```rust
const DW_LANG_C_plus_plus_14: DwLang;
```

### `DW_LANG_Fortran03`
```rust
const DW_LANG_Fortran03: DwLang;
```

### `DW_LANG_Fortran08`
```rust
const DW_LANG_Fortran08: DwLang;
```

### `DW_LANG_RenderScript`
```rust
const DW_LANG_RenderScript: DwLang;
```

### `DW_LANG_BLISS`
```rust
const DW_LANG_BLISS: DwLang;
```

### `DW_LANG_Kotlin`
```rust
const DW_LANG_Kotlin: DwLang;
```

### `DW_LANG_Zig`
```rust
const DW_LANG_Zig: DwLang;
```

### `DW_LANG_Crystal`
```rust
const DW_LANG_Crystal: DwLang;
```

### `DW_LANG_C_plus_plus_17`
```rust
const DW_LANG_C_plus_plus_17: DwLang;
```

### `DW_LANG_C_plus_plus_20`
```rust
const DW_LANG_C_plus_plus_20: DwLang;
```

### `DW_LANG_C17`
```rust
const DW_LANG_C17: DwLang;
```

### `DW_LANG_Fortran18`
```rust
const DW_LANG_Fortran18: DwLang;
```

### `DW_LANG_Ada2005`
```rust
const DW_LANG_Ada2005: DwLang;
```

### `DW_LANG_Ada2012`
```rust
const DW_LANG_Ada2012: DwLang;
```

### `DW_LANG_lo_user`
```rust
const DW_LANG_lo_user: DwLang;
```

### `DW_LANG_hi_user`
```rust
const DW_LANG_hi_user: DwLang;
```

### `DW_LANG_Mips_Assembler`
```rust
const DW_LANG_Mips_Assembler: DwLang;
```

### `DW_LANG_GOOGLE_RenderScript`
```rust
const DW_LANG_GOOGLE_RenderScript: DwLang;
```

### `DW_LANG_SUN_Assembler`
```rust
const DW_LANG_SUN_Assembler: DwLang;
```

### `DW_LANG_ALTIUM_Assembler`
```rust
const DW_LANG_ALTIUM_Assembler: DwLang;
```

### `DW_LANG_BORLAND_Delphi`
```rust
const DW_LANG_BORLAND_Delphi: DwLang;
```

### `DW_ADDR_none`
```rust
const DW_ADDR_none: DwAddr;
```

### `DW_ID_case_sensitive`
```rust
const DW_ID_case_sensitive: DwId;
```

### `DW_ID_up_case`
```rust
const DW_ID_up_case: DwId;
```

### `DW_ID_down_case`
```rust
const DW_ID_down_case: DwId;
```

### `DW_ID_case_insensitive`
```rust
const DW_ID_case_insensitive: DwId;
```

### `DW_CC_normal`
```rust
const DW_CC_normal: DwCc;
```

### `DW_CC_program`
```rust
const DW_CC_program: DwCc;
```

### `DW_CC_nocall`
```rust
const DW_CC_nocall: DwCc;
```

### `DW_CC_pass_by_reference`
```rust
const DW_CC_pass_by_reference: DwCc;
```

### `DW_CC_pass_by_value`
```rust
const DW_CC_pass_by_value: DwCc;
```

### `DW_CC_lo_user`
```rust
const DW_CC_lo_user: DwCc;
```

### `DW_CC_hi_user`
```rust
const DW_CC_hi_user: DwCc;
```

### `DW_INL_not_inlined`
```rust
const DW_INL_not_inlined: DwInl;
```

### `DW_INL_inlined`
```rust
const DW_INL_inlined: DwInl;
```

### `DW_INL_declared_not_inlined`
```rust
const DW_INL_declared_not_inlined: DwInl;
```

### `DW_INL_declared_inlined`
```rust
const DW_INL_declared_inlined: DwInl;
```

### `DW_ORD_row_major`
```rust
const DW_ORD_row_major: DwOrd;
```

### `DW_ORD_col_major`
```rust
const DW_ORD_col_major: DwOrd;
```

### `DW_DSC_label`
```rust
const DW_DSC_label: DwDsc;
```

### `DW_DSC_range`
```rust
const DW_DSC_range: DwDsc;
```

### `DW_IDX_compile_unit`
```rust
const DW_IDX_compile_unit: DwIdx;
```

### `DW_IDX_type_unit`
```rust
const DW_IDX_type_unit: DwIdx;
```

### `DW_IDX_die_offset`
```rust
const DW_IDX_die_offset: DwIdx;
```

### `DW_IDX_parent`
```rust
const DW_IDX_parent: DwIdx;
```

### `DW_IDX_type_hash`
```rust
const DW_IDX_type_hash: DwIdx;
```

### `DW_IDX_lo_user`
```rust
const DW_IDX_lo_user: DwIdx;
```

### `DW_IDX_hi_user`
```rust
const DW_IDX_hi_user: DwIdx;
```

### `DW_DEFAULTED_no`
```rust
const DW_DEFAULTED_no: DwDefaulted;
```

### `DW_DEFAULTED_in_class`
```rust
const DW_DEFAULTED_in_class: DwDefaulted;
```

### `DW_DEFAULTED_out_of_class`
```rust
const DW_DEFAULTED_out_of_class: DwDefaulted;
```

### `DW_LNS_copy`
```rust
const DW_LNS_copy: DwLns;
```

### `DW_LNS_advance_pc`
```rust
const DW_LNS_advance_pc: DwLns;
```

### `DW_LNS_advance_line`
```rust
const DW_LNS_advance_line: DwLns;
```

### `DW_LNS_set_file`
```rust
const DW_LNS_set_file: DwLns;
```

### `DW_LNS_set_column`
```rust
const DW_LNS_set_column: DwLns;
```

### `DW_LNS_negate_stmt`
```rust
const DW_LNS_negate_stmt: DwLns;
```

### `DW_LNS_set_basic_block`
```rust
const DW_LNS_set_basic_block: DwLns;
```

### `DW_LNS_const_add_pc`
```rust
const DW_LNS_const_add_pc: DwLns;
```

### `DW_LNS_fixed_advance_pc`
```rust
const DW_LNS_fixed_advance_pc: DwLns;
```

### `DW_LNS_set_prologue_end`
```rust
const DW_LNS_set_prologue_end: DwLns;
```

### `DW_LNS_set_epilogue_begin`
```rust
const DW_LNS_set_epilogue_begin: DwLns;
```

### `DW_LNS_set_isa`
```rust
const DW_LNS_set_isa: DwLns;
```

### `DW_LNE_end_sequence`
```rust
const DW_LNE_end_sequence: DwLne;
```

### `DW_LNE_set_address`
```rust
const DW_LNE_set_address: DwLne;
```

### `DW_LNE_define_file`
```rust
const DW_LNE_define_file: DwLne;
```

### `DW_LNE_set_discriminator`
```rust
const DW_LNE_set_discriminator: DwLne;
```

### `DW_LNE_lo_user`
```rust
const DW_LNE_lo_user: DwLne;
```

### `DW_LNE_hi_user`
```rust
const DW_LNE_hi_user: DwLne;
```

### `DW_LNCT_path`
```rust
const DW_LNCT_path: DwLnct;
```

### `DW_LNCT_directory_index`
```rust
const DW_LNCT_directory_index: DwLnct;
```

### `DW_LNCT_timestamp`
```rust
const DW_LNCT_timestamp: DwLnct;
```

### `DW_LNCT_size`
```rust
const DW_LNCT_size: DwLnct;
```

### `DW_LNCT_MD5`
```rust
const DW_LNCT_MD5: DwLnct;
```

### `DW_LNCT_lo_user`
```rust
const DW_LNCT_lo_user: DwLnct;
```

### `DW_LNCT_LLVM_source`
```rust
const DW_LNCT_LLVM_source: DwLnct;
```

### `DW_LNCT_hi_user`
```rust
const DW_LNCT_hi_user: DwLnct;
```

### `DW_MACINFO_define`
```rust
const DW_MACINFO_define: DwMacinfo;
```

### `DW_MACINFO_undef`
```rust
const DW_MACINFO_undef: DwMacinfo;
```

### `DW_MACINFO_start_file`
```rust
const DW_MACINFO_start_file: DwMacinfo;
```

### `DW_MACINFO_end_file`
```rust
const DW_MACINFO_end_file: DwMacinfo;
```

### `DW_MACINFO_vendor_ext`
```rust
const DW_MACINFO_vendor_ext: DwMacinfo;
```

### `DW_MACRO_define`
```rust
const DW_MACRO_define: DwMacro;
```

### `DW_MACRO_undef`
```rust
const DW_MACRO_undef: DwMacro;
```

### `DW_MACRO_start_file`
```rust
const DW_MACRO_start_file: DwMacro;
```

### `DW_MACRO_end_file`
```rust
const DW_MACRO_end_file: DwMacro;
```

### `DW_MACRO_define_strp`
```rust
const DW_MACRO_define_strp: DwMacro;
```

### `DW_MACRO_undef_strp`
```rust
const DW_MACRO_undef_strp: DwMacro;
```

### `DW_MACRO_import`
```rust
const DW_MACRO_import: DwMacro;
```

### `DW_MACRO_define_sup`
```rust
const DW_MACRO_define_sup: DwMacro;
```

### `DW_MACRO_undef_sup`
```rust
const DW_MACRO_undef_sup: DwMacro;
```

### `DW_MACRO_import_sup`
```rust
const DW_MACRO_import_sup: DwMacro;
```

### `DW_MACRO_define_strx`
```rust
const DW_MACRO_define_strx: DwMacro;
```

### `DW_MACRO_undef_strx`
```rust
const DW_MACRO_undef_strx: DwMacro;
```

### `DW_MACRO_lo_user`
```rust
const DW_MACRO_lo_user: DwMacro;
```

### `DW_MACRO_hi_user`
```rust
const DW_MACRO_hi_user: DwMacro;
```

### `DW_RLE_end_of_list`
```rust
const DW_RLE_end_of_list: DwRle;
```

### `DW_RLE_base_addressx`
```rust
const DW_RLE_base_addressx: DwRle;
```

### `DW_RLE_startx_endx`
```rust
const DW_RLE_startx_endx: DwRle;
```

### `DW_RLE_startx_length`
```rust
const DW_RLE_startx_length: DwRle;
```

### `DW_RLE_offset_pair`
```rust
const DW_RLE_offset_pair: DwRle;
```

### `DW_RLE_base_address`
```rust
const DW_RLE_base_address: DwRle;
```

### `DW_RLE_start_end`
```rust
const DW_RLE_start_end: DwRle;
```

### `DW_RLE_start_length`
```rust
const DW_RLE_start_length: DwRle;
```

### `DW_OP_addr`
```rust
const DW_OP_addr: DwOp;
```

### `DW_OP_deref`
```rust
const DW_OP_deref: DwOp;
```

### `DW_OP_const1u`
```rust
const DW_OP_const1u: DwOp;
```

### `DW_OP_const1s`
```rust
const DW_OP_const1s: DwOp;
```

### `DW_OP_const2u`
```rust
const DW_OP_const2u: DwOp;
```

### `DW_OP_const2s`
```rust
const DW_OP_const2s: DwOp;
```

### `DW_OP_const4u`
```rust
const DW_OP_const4u: DwOp;
```

### `DW_OP_const4s`
```rust
const DW_OP_const4s: DwOp;
```

### `DW_OP_const8u`
```rust
const DW_OP_const8u: DwOp;
```

### `DW_OP_const8s`
```rust
const DW_OP_const8s: DwOp;
```

### `DW_OP_constu`
```rust
const DW_OP_constu: DwOp;
```

### `DW_OP_consts`
```rust
const DW_OP_consts: DwOp;
```

### `DW_OP_dup`
```rust
const DW_OP_dup: DwOp;
```

### `DW_OP_drop`
```rust
const DW_OP_drop: DwOp;
```

### `DW_OP_over`
```rust
const DW_OP_over: DwOp;
```

### `DW_OP_pick`
```rust
const DW_OP_pick: DwOp;
```

### `DW_OP_swap`
```rust
const DW_OP_swap: DwOp;
```

### `DW_OP_rot`
```rust
const DW_OP_rot: DwOp;
```

### `DW_OP_xderef`
```rust
const DW_OP_xderef: DwOp;
```

### `DW_OP_abs`
```rust
const DW_OP_abs: DwOp;
```

### `DW_OP_and`
```rust
const DW_OP_and: DwOp;
```

### `DW_OP_div`
```rust
const DW_OP_div: DwOp;
```

### `DW_OP_minus`
```rust
const DW_OP_minus: DwOp;
```

### `DW_OP_mod`
```rust
const DW_OP_mod: DwOp;
```

### `DW_OP_mul`
```rust
const DW_OP_mul: DwOp;
```

### `DW_OP_neg`
```rust
const DW_OP_neg: DwOp;
```

### `DW_OP_not`
```rust
const DW_OP_not: DwOp;
```

### `DW_OP_or`
```rust
const DW_OP_or: DwOp;
```

### `DW_OP_plus`
```rust
const DW_OP_plus: DwOp;
```

### `DW_OP_plus_uconst`
```rust
const DW_OP_plus_uconst: DwOp;
```

### `DW_OP_shl`
```rust
const DW_OP_shl: DwOp;
```

### `DW_OP_shr`
```rust
const DW_OP_shr: DwOp;
```

### `DW_OP_shra`
```rust
const DW_OP_shra: DwOp;
```

### `DW_OP_xor`
```rust
const DW_OP_xor: DwOp;
```

### `DW_OP_bra`
```rust
const DW_OP_bra: DwOp;
```

### `DW_OP_eq`
```rust
const DW_OP_eq: DwOp;
```

### `DW_OP_ge`
```rust
const DW_OP_ge: DwOp;
```

### `DW_OP_gt`
```rust
const DW_OP_gt: DwOp;
```

### `DW_OP_le`
```rust
const DW_OP_le: DwOp;
```

### `DW_OP_lt`
```rust
const DW_OP_lt: DwOp;
```

### `DW_OP_ne`
```rust
const DW_OP_ne: DwOp;
```

### `DW_OP_skip`
```rust
const DW_OP_skip: DwOp;
```

### `DW_OP_lit0`
```rust
const DW_OP_lit0: DwOp;
```

### `DW_OP_lit1`
```rust
const DW_OP_lit1: DwOp;
```

### `DW_OP_lit2`
```rust
const DW_OP_lit2: DwOp;
```

### `DW_OP_lit3`
```rust
const DW_OP_lit3: DwOp;
```

### `DW_OP_lit4`
```rust
const DW_OP_lit4: DwOp;
```

### `DW_OP_lit5`
```rust
const DW_OP_lit5: DwOp;
```

### `DW_OP_lit6`
```rust
const DW_OP_lit6: DwOp;
```

### `DW_OP_lit7`
```rust
const DW_OP_lit7: DwOp;
```

### `DW_OP_lit8`
```rust
const DW_OP_lit8: DwOp;
```

### `DW_OP_lit9`
```rust
const DW_OP_lit9: DwOp;
```

### `DW_OP_lit10`
```rust
const DW_OP_lit10: DwOp;
```

### `DW_OP_lit11`
```rust
const DW_OP_lit11: DwOp;
```

### `DW_OP_lit12`
```rust
const DW_OP_lit12: DwOp;
```

### `DW_OP_lit13`
```rust
const DW_OP_lit13: DwOp;
```

### `DW_OP_lit14`
```rust
const DW_OP_lit14: DwOp;
```

### `DW_OP_lit15`
```rust
const DW_OP_lit15: DwOp;
```

### `DW_OP_lit16`
```rust
const DW_OP_lit16: DwOp;
```

### `DW_OP_lit17`
```rust
const DW_OP_lit17: DwOp;
```

### `DW_OP_lit18`
```rust
const DW_OP_lit18: DwOp;
```

### `DW_OP_lit19`
```rust
const DW_OP_lit19: DwOp;
```

### `DW_OP_lit20`
```rust
const DW_OP_lit20: DwOp;
```

### `DW_OP_lit21`
```rust
const DW_OP_lit21: DwOp;
```

### `DW_OP_lit22`
```rust
const DW_OP_lit22: DwOp;
```

### `DW_OP_lit23`
```rust
const DW_OP_lit23: DwOp;
```

### `DW_OP_lit24`
```rust
const DW_OP_lit24: DwOp;
```

### `DW_OP_lit25`
```rust
const DW_OP_lit25: DwOp;
```

### `DW_OP_lit26`
```rust
const DW_OP_lit26: DwOp;
```

### `DW_OP_lit27`
```rust
const DW_OP_lit27: DwOp;
```

### `DW_OP_lit28`
```rust
const DW_OP_lit28: DwOp;
```

### `DW_OP_lit29`
```rust
const DW_OP_lit29: DwOp;
```

### `DW_OP_lit30`
```rust
const DW_OP_lit30: DwOp;
```

### `DW_OP_lit31`
```rust
const DW_OP_lit31: DwOp;
```

### `DW_OP_reg0`
```rust
const DW_OP_reg0: DwOp;
```

### `DW_OP_reg1`
```rust
const DW_OP_reg1: DwOp;
```

### `DW_OP_reg2`
```rust
const DW_OP_reg2: DwOp;
```

### `DW_OP_reg3`
```rust
const DW_OP_reg3: DwOp;
```

### `DW_OP_reg4`
```rust
const DW_OP_reg4: DwOp;
```

### `DW_OP_reg5`
```rust
const DW_OP_reg5: DwOp;
```

### `DW_OP_reg6`
```rust
const DW_OP_reg6: DwOp;
```

### `DW_OP_reg7`
```rust
const DW_OP_reg7: DwOp;
```

### `DW_OP_reg8`
```rust
const DW_OP_reg8: DwOp;
```

### `DW_OP_reg9`
```rust
const DW_OP_reg9: DwOp;
```

### `DW_OP_reg10`
```rust
const DW_OP_reg10: DwOp;
```

### `DW_OP_reg11`
```rust
const DW_OP_reg11: DwOp;
```

### `DW_OP_reg12`
```rust
const DW_OP_reg12: DwOp;
```

### `DW_OP_reg13`
```rust
const DW_OP_reg13: DwOp;
```

### `DW_OP_reg14`
```rust
const DW_OP_reg14: DwOp;
```

### `DW_OP_reg15`
```rust
const DW_OP_reg15: DwOp;
```

### `DW_OP_reg16`
```rust
const DW_OP_reg16: DwOp;
```

### `DW_OP_reg17`
```rust
const DW_OP_reg17: DwOp;
```

### `DW_OP_reg18`
```rust
const DW_OP_reg18: DwOp;
```

### `DW_OP_reg19`
```rust
const DW_OP_reg19: DwOp;
```

### `DW_OP_reg20`
```rust
const DW_OP_reg20: DwOp;
```

### `DW_OP_reg21`
```rust
const DW_OP_reg21: DwOp;
```

### `DW_OP_reg22`
```rust
const DW_OP_reg22: DwOp;
```

### `DW_OP_reg23`
```rust
const DW_OP_reg23: DwOp;
```

### `DW_OP_reg24`
```rust
const DW_OP_reg24: DwOp;
```

### `DW_OP_reg25`
```rust
const DW_OP_reg25: DwOp;
```

### `DW_OP_reg26`
```rust
const DW_OP_reg26: DwOp;
```

### `DW_OP_reg27`
```rust
const DW_OP_reg27: DwOp;
```

### `DW_OP_reg28`
```rust
const DW_OP_reg28: DwOp;
```

### `DW_OP_reg29`
```rust
const DW_OP_reg29: DwOp;
```

### `DW_OP_reg30`
```rust
const DW_OP_reg30: DwOp;
```

### `DW_OP_reg31`
```rust
const DW_OP_reg31: DwOp;
```

### `DW_OP_breg0`
```rust
const DW_OP_breg0: DwOp;
```

### `DW_OP_breg1`
```rust
const DW_OP_breg1: DwOp;
```

### `DW_OP_breg2`
```rust
const DW_OP_breg2: DwOp;
```

### `DW_OP_breg3`
```rust
const DW_OP_breg3: DwOp;
```

### `DW_OP_breg4`
```rust
const DW_OP_breg4: DwOp;
```

### `DW_OP_breg5`
```rust
const DW_OP_breg5: DwOp;
```

### `DW_OP_breg6`
```rust
const DW_OP_breg6: DwOp;
```

### `DW_OP_breg7`
```rust
const DW_OP_breg7: DwOp;
```

### `DW_OP_breg8`
```rust
const DW_OP_breg8: DwOp;
```

### `DW_OP_breg9`
```rust
const DW_OP_breg9: DwOp;
```

### `DW_OP_breg10`
```rust
const DW_OP_breg10: DwOp;
```

### `DW_OP_breg11`
```rust
const DW_OP_breg11: DwOp;
```

### `DW_OP_breg12`
```rust
const DW_OP_breg12: DwOp;
```

### `DW_OP_breg13`
```rust
const DW_OP_breg13: DwOp;
```

### `DW_OP_breg14`
```rust
const DW_OP_breg14: DwOp;
```

### `DW_OP_breg15`
```rust
const DW_OP_breg15: DwOp;
```

### `DW_OP_breg16`
```rust
const DW_OP_breg16: DwOp;
```

### `DW_OP_breg17`
```rust
const DW_OP_breg17: DwOp;
```

### `DW_OP_breg18`
```rust
const DW_OP_breg18: DwOp;
```

### `DW_OP_breg19`
```rust
const DW_OP_breg19: DwOp;
```

### `DW_OP_breg20`
```rust
const DW_OP_breg20: DwOp;
```

### `DW_OP_breg21`
```rust
const DW_OP_breg21: DwOp;
```

### `DW_OP_breg22`
```rust
const DW_OP_breg22: DwOp;
```

### `DW_OP_breg23`
```rust
const DW_OP_breg23: DwOp;
```

### `DW_OP_breg24`
```rust
const DW_OP_breg24: DwOp;
```

### `DW_OP_breg25`
```rust
const DW_OP_breg25: DwOp;
```

### `DW_OP_breg26`
```rust
const DW_OP_breg26: DwOp;
```

### `DW_OP_breg27`
```rust
const DW_OP_breg27: DwOp;
```

### `DW_OP_breg28`
```rust
const DW_OP_breg28: DwOp;
```

### `DW_OP_breg29`
```rust
const DW_OP_breg29: DwOp;
```

### `DW_OP_breg30`
```rust
const DW_OP_breg30: DwOp;
```

### `DW_OP_breg31`
```rust
const DW_OP_breg31: DwOp;
```

### `DW_OP_regx`
```rust
const DW_OP_regx: DwOp;
```

### `DW_OP_fbreg`
```rust
const DW_OP_fbreg: DwOp;
```

### `DW_OP_bregx`
```rust
const DW_OP_bregx: DwOp;
```

### `DW_OP_piece`
```rust
const DW_OP_piece: DwOp;
```

### `DW_OP_deref_size`
```rust
const DW_OP_deref_size: DwOp;
```

### `DW_OP_xderef_size`
```rust
const DW_OP_xderef_size: DwOp;
```

### `DW_OP_nop`
```rust
const DW_OP_nop: DwOp;
```

### `DW_OP_push_object_address`
```rust
const DW_OP_push_object_address: DwOp;
```

### `DW_OP_call2`
```rust
const DW_OP_call2: DwOp;
```

### `DW_OP_call4`
```rust
const DW_OP_call4: DwOp;
```

### `DW_OP_call_ref`
```rust
const DW_OP_call_ref: DwOp;
```

### `DW_OP_form_tls_address`
```rust
const DW_OP_form_tls_address: DwOp;
```

### `DW_OP_call_frame_cfa`
```rust
const DW_OP_call_frame_cfa: DwOp;
```

### `DW_OP_bit_piece`
```rust
const DW_OP_bit_piece: DwOp;
```

### `DW_OP_implicit_value`
```rust
const DW_OP_implicit_value: DwOp;
```

### `DW_OP_stack_value`
```rust
const DW_OP_stack_value: DwOp;
```

### `DW_OP_implicit_pointer`
```rust
const DW_OP_implicit_pointer: DwOp;
```

### `DW_OP_addrx`
```rust
const DW_OP_addrx: DwOp;
```

### `DW_OP_constx`
```rust
const DW_OP_constx: DwOp;
```

### `DW_OP_entry_value`
```rust
const DW_OP_entry_value: DwOp;
```

### `DW_OP_const_type`
```rust
const DW_OP_const_type: DwOp;
```

### `DW_OP_regval_type`
```rust
const DW_OP_regval_type: DwOp;
```

### `DW_OP_deref_type`
```rust
const DW_OP_deref_type: DwOp;
```

### `DW_OP_xderef_type`
```rust
const DW_OP_xderef_type: DwOp;
```

### `DW_OP_convert`
```rust
const DW_OP_convert: DwOp;
```

### `DW_OP_reinterpret`
```rust
const DW_OP_reinterpret: DwOp;
```

### `DW_OP_GNU_push_tls_address`
```rust
const DW_OP_GNU_push_tls_address: DwOp;
```

### `DW_OP_GNU_uninit`
```rust
const DW_OP_GNU_uninit: DwOp;
```

### `DW_OP_GNU_encoded_addr`
```rust
const DW_OP_GNU_encoded_addr: DwOp;
```

### `DW_OP_GNU_implicit_pointer`
```rust
const DW_OP_GNU_implicit_pointer: DwOp;
```

### `DW_OP_GNU_entry_value`
```rust
const DW_OP_GNU_entry_value: DwOp;
```

### `DW_OP_GNU_const_type`
```rust
const DW_OP_GNU_const_type: DwOp;
```

### `DW_OP_GNU_regval_type`
```rust
const DW_OP_GNU_regval_type: DwOp;
```

### `DW_OP_GNU_deref_type`
```rust
const DW_OP_GNU_deref_type: DwOp;
```

### `DW_OP_GNU_convert`
```rust
const DW_OP_GNU_convert: DwOp;
```

### `DW_OP_GNU_reinterpret`
```rust
const DW_OP_GNU_reinterpret: DwOp;
```

### `DW_OP_GNU_parameter_ref`
```rust
const DW_OP_GNU_parameter_ref: DwOp;
```

### `DW_OP_GNU_addr_index`
```rust
const DW_OP_GNU_addr_index: DwOp;
```

### `DW_OP_GNU_const_index`
```rust
const DW_OP_GNU_const_index: DwOp;
```

### `DW_OP_GNU_variable_value`
```rust
const DW_OP_GNU_variable_value: DwOp;
```

### `DW_OP_WASM_location`
```rust
const DW_OP_WASM_location: DwOp;
```

### `DW_EH_PE_uleb128`
```rust
const DW_EH_PE_uleb128: DwEhPe;
```

### `DW_EH_PE_udata2`
```rust
const DW_EH_PE_udata2: DwEhPe;
```

### `DW_EH_PE_udata4`
```rust
const DW_EH_PE_udata4: DwEhPe;
```

### `DW_EH_PE_udata8`
```rust
const DW_EH_PE_udata8: DwEhPe;
```

### `DW_EH_PE_sleb128`
```rust
const DW_EH_PE_sleb128: DwEhPe;
```

### `DW_EH_PE_sdata2`
```rust
const DW_EH_PE_sdata2: DwEhPe;
```

### `DW_EH_PE_sdata4`
```rust
const DW_EH_PE_sdata4: DwEhPe;
```

### `DW_EH_PE_sdata8`
```rust
const DW_EH_PE_sdata8: DwEhPe;
```

### `DW_EH_PE_pcrel`
```rust
const DW_EH_PE_pcrel: DwEhPe;
```

### `DW_EH_PE_textrel`
```rust
const DW_EH_PE_textrel: DwEhPe;
```

### `DW_EH_PE_datarel`
```rust
const DW_EH_PE_datarel: DwEhPe;
```

### `DW_EH_PE_funcrel`
```rust
const DW_EH_PE_funcrel: DwEhPe;
```

### `DW_EH_PE_aligned`
```rust
const DW_EH_PE_aligned: DwEhPe;
```

### `DW_EH_PE_indirect`
```rust
const DW_EH_PE_indirect: DwEhPe;
```

### `DW_EH_PE_absptr`
```rust
const DW_EH_PE_absptr: DwEhPe;
```

### `DW_EH_PE_omit`
```rust
const DW_EH_PE_omit: DwEhPe;
```

### `DW_EH_PE_FORMAT_MASK`
```rust
const DW_EH_PE_FORMAT_MASK: u8 = 15u8;
```

### `DW_EH_PE_APPLICATION_MASK`
```rust
const DW_EH_PE_APPLICATION_MASK: u8 = 112u8;
```

## Macros

### `registers!`

### `dw!`

