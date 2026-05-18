**gimli > constants**

# Module: constants

## Contents

**Structs**

- [`DwAccess`](#dwaccess) - The encodings of the constants used in the `DW_AT_accessibility` attribute.
- [`DwAddr`](#dwaddr) - The encodings of the constants used in the `DW_AT_address_class` attribute.
- [`DwAt`](#dwat) - The attribute encodings for DIE attributes.
- [`DwAte`](#dwate) - The encodings of the constants used in the `DW_AT_encoding` attribute.
- [`DwCc`](#dwcc) - The encodings of the constants used in the `DW_AT_calling_convention` attribute.
- [`DwCfa`](#dwcfa) - The opcode for a call frame instruction.
- [`DwChildren`](#dwchildren) - The child determination encodings for DIE attributes.
- [`DwDefaulted`](#dwdefaulted) - The encodings of the constants used in the `DW_AT_defaulted` attribute.
- [`DwDs`](#dwds) - The encodings of the constants used in the `DW_AT_decimal_sign` attribute.
- [`DwDsc`](#dwdsc) - The encodings of the constants used in the `DW_AT_discr_list` attribute.
- [`DwEhPe`](#dwehpe) - Pointer encoding used by `.eh_frame`.
- [`DwEnd`](#dwend) - The encodings of the constants used in the `DW_AT_endianity` attribute.
- [`DwForm`](#dwform) - The attribute form encodings for DIE attributes.
- [`DwId`](#dwid) - The encodings of the constants used in the `DW_AT_identifier_case` attribute.
- [`DwIdx`](#dwidx) - Name index attribute encodings.
- [`DwInl`](#dwinl) - The encodings of the constants used in the `DW_AT_inline` attribute.
- [`DwLang`](#dwlang) - The encodings of the constants used in the `DW_AT_language` attribute.
- [`DwLle`](#dwlle) - The encodings of the constants used in location list entries.
- [`DwLnct`](#dwlnct) - The encodings for the line number header entry formats.
- [`DwLne`](#dwlne) - The encodings for the extended opcodes for line number information.
- [`DwLns`](#dwlns) - The encodings for the standard opcodes for line number information.
- [`DwMacinfo`](#dwmacinfo) - Type codes for macro definitions in the `.debug_macinfo` section.
- [`DwMacro`](#dwmacro) - The encodings for macro information entry types.
- [`DwOp`](#dwop) - The encodings for DWARF expression operations.
- [`DwOrd`](#dword) - The encodings of the constants used in the `DW_AT_ordering` attribute.
- [`DwRle`](#dwrle) - Range list entry encoding values.
- [`DwSect`](#dwsect) - The section type field in a `.dwp` unit index.
- [`DwSectV2`](#dwsectv2) - The section type field in a `.dwp` unit index with version 2.
- [`DwTag`](#dwtag) - The tag encodings for DIE attributes.
- [`DwUt`](#dwut) - The unit type field in a unit header.
- [`DwVirtuality`](#dwvirtuality) - The encodings of the constants used in the `DW_AT_virtuality` attribute.
- [`DwVis`](#dwvis) - The encodings of the constants used in the `DW_AT_visibility` attribute.

**Constants**

- [`DW_ACCESS_private`](#dw_access_private)
- [`DW_ACCESS_protected`](#dw_access_protected)
- [`DW_ACCESS_public`](#dw_access_public)
- [`DW_ADDR_none`](#dw_addr_none)
- [`DW_ATE_ASCII`](#dw_ate_ascii)
- [`DW_ATE_UCS`](#dw_ate_ucs)
- [`DW_ATE_UTF`](#dw_ate_utf)
- [`DW_ATE_address`](#dw_ate_address)
- [`DW_ATE_boolean`](#dw_ate_boolean)
- [`DW_ATE_complex_float`](#dw_ate_complex_float)
- [`DW_ATE_decimal_float`](#dw_ate_decimal_float)
- [`DW_ATE_edited`](#dw_ate_edited)
- [`DW_ATE_float`](#dw_ate_float)
- [`DW_ATE_hi_user`](#dw_ate_hi_user)
- [`DW_ATE_imaginary_float`](#dw_ate_imaginary_float)
- [`DW_ATE_lo_user`](#dw_ate_lo_user)
- [`DW_ATE_numeric_string`](#dw_ate_numeric_string)
- [`DW_ATE_packed_decimal`](#dw_ate_packed_decimal)
- [`DW_ATE_signed`](#dw_ate_signed)
- [`DW_ATE_signed_char`](#dw_ate_signed_char)
- [`DW_ATE_signed_fixed`](#dw_ate_signed_fixed)
- [`DW_ATE_unsigned`](#dw_ate_unsigned)
- [`DW_ATE_unsigned_char`](#dw_ate_unsigned_char)
- [`DW_ATE_unsigned_fixed`](#dw_ate_unsigned_fixed)
- [`DW_AT_ALTIUM_loclist`](#dw_at_altium_loclist)
- [`DW_AT_APPLE_block`](#dw_at_apple_block)
- [`DW_AT_APPLE_flags`](#dw_at_apple_flags)
- [`DW_AT_APPLE_isa`](#dw_at_apple_isa)
- [`DW_AT_APPLE_major_runtime_vers`](#dw_at_apple_major_runtime_vers)
- [`DW_AT_APPLE_objc_complete_type`](#dw_at_apple_objc_complete_type)
- [`DW_AT_APPLE_omit_frame_ptr`](#dw_at_apple_omit_frame_ptr)
- [`DW_AT_APPLE_optimized`](#dw_at_apple_optimized)
- [`DW_AT_APPLE_property`](#dw_at_apple_property)
- [`DW_AT_APPLE_property_attribute`](#dw_at_apple_property_attribute)
- [`DW_AT_APPLE_property_getter`](#dw_at_apple_property_getter)
- [`DW_AT_APPLE_property_name`](#dw_at_apple_property_name)
- [`DW_AT_APPLE_property_setter`](#dw_at_apple_property_setter)
- [`DW_AT_APPLE_runtime_class`](#dw_at_apple_runtime_class)
- [`DW_AT_BORLAND_Delphi_ABI`](#dw_at_borland_delphi_abi)
- [`DW_AT_BORLAND_Delphi_anonymous_method`](#dw_at_borland_delphi_anonymous_method)
- [`DW_AT_BORLAND_Delphi_class`](#dw_at_borland_delphi_class)
- [`DW_AT_BORLAND_Delphi_constructor`](#dw_at_borland_delphi_constructor)
- [`DW_AT_BORLAND_Delphi_destructor`](#dw_at_borland_delphi_destructor)
- [`DW_AT_BORLAND_Delphi_frameptr`](#dw_at_borland_delphi_frameptr)
- [`DW_AT_BORLAND_Delphi_interface`](#dw_at_borland_delphi_interface)
- [`DW_AT_BORLAND_Delphi_metaclass`](#dw_at_borland_delphi_metaclass)
- [`DW_AT_BORLAND_Delphi_record`](#dw_at_borland_delphi_record)
- [`DW_AT_BORLAND_Delphi_return`](#dw_at_borland_delphi_return)
- [`DW_AT_BORLAND_Delphi_unit`](#dw_at_borland_delphi_unit)
- [`DW_AT_BORLAND_closure`](#dw_at_borland_closure)
- [`DW_AT_BORLAND_property_default`](#dw_at_borland_property_default)
- [`DW_AT_BORLAND_property_implements`](#dw_at_borland_property_implements)
- [`DW_AT_BORLAND_property_index`](#dw_at_borland_property_index)
- [`DW_AT_BORLAND_property_read`](#dw_at_borland_property_read)
- [`DW_AT_BORLAND_property_write`](#dw_at_borland_property_write)
- [`DW_AT_GNAT_descriptive_type`](#dw_at_gnat_descriptive_type)
- [`DW_AT_GNU_addr_base`](#dw_at_gnu_addr_base)
- [`DW_AT_GNU_all_call_sites`](#dw_at_gnu_all_call_sites)
- [`DW_AT_GNU_all_source_call_sites`](#dw_at_gnu_all_source_call_sites)
- [`DW_AT_GNU_all_tail_call_sites`](#dw_at_gnu_all_tail_call_sites)
- [`DW_AT_GNU_bias`](#dw_at_gnu_bias)
- [`DW_AT_GNU_call_site_data_value`](#dw_at_gnu_call_site_data_value)
- [`DW_AT_GNU_call_site_target`](#dw_at_gnu_call_site_target)
- [`DW_AT_GNU_call_site_target_clobbered`](#dw_at_gnu_call_site_target_clobbered)
- [`DW_AT_GNU_call_site_value`](#dw_at_gnu_call_site_value)
- [`DW_AT_GNU_deleted`](#dw_at_gnu_deleted)
- [`DW_AT_GNU_denominator`](#dw_at_gnu_denominator)
- [`DW_AT_GNU_discriminator`](#dw_at_gnu_discriminator)
- [`DW_AT_GNU_dwo_id`](#dw_at_gnu_dwo_id)
- [`DW_AT_GNU_dwo_name`](#dw_at_gnu_dwo_name)
- [`DW_AT_GNU_entry_view`](#dw_at_gnu_entry_view)
- [`DW_AT_GNU_exclusive_locks_required`](#dw_at_gnu_exclusive_locks_required)
- [`DW_AT_GNU_guarded`](#dw_at_gnu_guarded)
- [`DW_AT_GNU_guarded_by`](#dw_at_gnu_guarded_by)
- [`DW_AT_GNU_locks_excluded`](#dw_at_gnu_locks_excluded)
- [`DW_AT_GNU_locviews`](#dw_at_gnu_locviews)
- [`DW_AT_GNU_macros`](#dw_at_gnu_macros)
- [`DW_AT_GNU_numerator`](#dw_at_gnu_numerator)
- [`DW_AT_GNU_odr_signature`](#dw_at_gnu_odr_signature)
- [`DW_AT_GNU_pt_guarded`](#dw_at_gnu_pt_guarded)
- [`DW_AT_GNU_pt_guarded_by`](#dw_at_gnu_pt_guarded_by)
- [`DW_AT_GNU_pubnames`](#dw_at_gnu_pubnames)
- [`DW_AT_GNU_pubtypes`](#dw_at_gnu_pubtypes)
- [`DW_AT_GNU_ranges_base`](#dw_at_gnu_ranges_base)
- [`DW_AT_GNU_shared_locks_required`](#dw_at_gnu_shared_locks_required)
- [`DW_AT_GNU_tail_call`](#dw_at_gnu_tail_call)
- [`DW_AT_GNU_template_name`](#dw_at_gnu_template_name)
- [`DW_AT_GNU_vector`](#dw_at_gnu_vector)
- [`DW_AT_INTEL_other_endian`](#dw_at_intel_other_endian)
- [`DW_AT_LLVM_config_macros`](#dw_at_llvm_config_macros)
- [`DW_AT_LLVM_include_path`](#dw_at_llvm_include_path)
- [`DW_AT_LLVM_isysroot`](#dw_at_llvm_isysroot)
- [`DW_AT_MIPS_abstract_name`](#dw_at_mips_abstract_name)
- [`DW_AT_MIPS_allocatable_dopetype`](#dw_at_mips_allocatable_dopetype)
- [`DW_AT_MIPS_assumed_shape_dopetype`](#dw_at_mips_assumed_shape_dopetype)
- [`DW_AT_MIPS_assumed_size`](#dw_at_mips_assumed_size)
- [`DW_AT_MIPS_clone_origin`](#dw_at_mips_clone_origin)
- [`DW_AT_MIPS_epilog_begin`](#dw_at_mips_epilog_begin)
- [`DW_AT_MIPS_fde`](#dw_at_mips_fde)
- [`DW_AT_MIPS_has_inlines`](#dw_at_mips_has_inlines)
- [`DW_AT_MIPS_linkage_name`](#dw_at_mips_linkage_name)
- [`DW_AT_MIPS_loop_begin`](#dw_at_mips_loop_begin)
- [`DW_AT_MIPS_loop_unroll_factor`](#dw_at_mips_loop_unroll_factor)
- [`DW_AT_MIPS_ptr_dopetype`](#dw_at_mips_ptr_dopetype)
- [`DW_AT_MIPS_software_pipeline_depth`](#dw_at_mips_software_pipeline_depth)
- [`DW_AT_MIPS_stride`](#dw_at_mips_stride)
- [`DW_AT_MIPS_stride_byte`](#dw_at_mips_stride_byte)
- [`DW_AT_MIPS_stride_elem`](#dw_at_mips_stride_elem)
- [`DW_AT_MIPS_tail_loop_begin`](#dw_at_mips_tail_loop_begin)
- [`DW_AT_PGI_lbase`](#dw_at_pgi_lbase)
- [`DW_AT_PGI_lstride`](#dw_at_pgi_lstride)
- [`DW_AT_PGI_soffset`](#dw_at_pgi_soffset)
- [`DW_AT_SUN_alignment`](#dw_at_sun_alignment)
- [`DW_AT_SUN_amd64_parmdump`](#dw_at_sun_amd64_parmdump)
- [`DW_AT_SUN_browser_file`](#dw_at_sun_browser_file)
- [`DW_AT_SUN_c_vla`](#dw_at_sun_c_vla)
- [`DW_AT_SUN_cf_kind`](#dw_at_sun_cf_kind)
- [`DW_AT_SUN_command_line`](#dw_at_sun_command_line)
- [`DW_AT_SUN_compile_options`](#dw_at_sun_compile_options)
- [`DW_AT_SUN_count_guarantee`](#dw_at_sun_count_guarantee)
- [`DW_AT_SUN_dtor_length`](#dw_at_sun_dtor_length)
- [`DW_AT_SUN_dtor_start`](#dw_at_sun_dtor_start)
- [`DW_AT_SUN_dtor_state_deltas`](#dw_at_sun_dtor_state_deltas)
- [`DW_AT_SUN_dtor_state_final`](#dw_at_sun_dtor_state_final)
- [`DW_AT_SUN_dtor_state_initial`](#dw_at_sun_dtor_state_initial)
- [`DW_AT_SUN_f90_allocatable`](#dw_at_sun_f90_allocatable)
- [`DW_AT_SUN_f90_assumed_shape_array`](#dw_at_sun_f90_assumed_shape_array)
- [`DW_AT_SUN_f90_pointer`](#dw_at_sun_f90_pointer)
- [`DW_AT_SUN_f90_use_only`](#dw_at_sun_f90_use_only)
- [`DW_AT_SUN_fortran_based`](#dw_at_sun_fortran_based)
- [`DW_AT_SUN_fortran_main_alias`](#dw_at_sun_fortran_main_alias)
- [`DW_AT_SUN_func_offset`](#dw_at_sun_func_offset)
- [`DW_AT_SUN_func_offsets`](#dw_at_sun_func_offsets)
- [`DW_AT_SUN_hwcprof_signature`](#dw_at_sun_hwcprof_signature)
- [`DW_AT_SUN_import_by_lname`](#dw_at_sun_import_by_lname)
- [`DW_AT_SUN_import_by_name`](#dw_at_sun_import_by_name)
- [`DW_AT_SUN_is_omp_child_func`](#dw_at_sun_is_omp_child_func)
- [`DW_AT_SUN_language`](#dw_at_sun_language)
- [`DW_AT_SUN_link_name`](#dw_at_sun_link_name)
- [`DW_AT_SUN_memop_signature`](#dw_at_sun_memop_signature)
- [`DW_AT_SUN_memop_type_ref`](#dw_at_sun_memop_type_ref)
- [`DW_AT_SUN_namelist_spec`](#dw_at_sun_namelist_spec)
- [`DW_AT_SUN_obj_dir`](#dw_at_sun_obj_dir)
- [`DW_AT_SUN_obj_file`](#dw_at_sun_obj_file)
- [`DW_AT_SUN_omp_child_func`](#dw_at_sun_omp_child_func)
- [`DW_AT_SUN_omp_tpriv_addr`](#dw_at_sun_omp_tpriv_addr)
- [`DW_AT_SUN_original_name`](#dw_at_sun_original_name)
- [`DW_AT_SUN_part_link_name`](#dw_at_sun_part_link_name)
- [`DW_AT_SUN_pass_by_ref`](#dw_at_sun_pass_by_ref)
- [`DW_AT_SUN_pass_with_const`](#dw_at_sun_pass_with_const)
- [`DW_AT_SUN_profile_id`](#dw_at_sun_profile_id)
- [`DW_AT_SUN_return_value_ptr`](#dw_at_sun_return_value_ptr)
- [`DW_AT_SUN_return_with_const`](#dw_at_sun_return_with_const)
- [`DW_AT_SUN_template`](#dw_at_sun_template)
- [`DW_AT_SUN_vbase`](#dw_at_sun_vbase)
- [`DW_AT_SUN_vtable`](#dw_at_sun_vtable)
- [`DW_AT_SUN_vtable_abi`](#dw_at_sun_vtable_abi)
- [`DW_AT_SUN_vtable_index`](#dw_at_sun_vtable_index)
- [`DW_AT_abstract_origin`](#dw_at_abstract_origin)
- [`DW_AT_accessibility`](#dw_at_accessibility)
- [`DW_AT_addr_base`](#dw_at_addr_base)
- [`DW_AT_address_class`](#dw_at_address_class)
- [`DW_AT_alignment`](#dw_at_alignment)
- [`DW_AT_allocated`](#dw_at_allocated)
- [`DW_AT_artificial`](#dw_at_artificial)
- [`DW_AT_associated`](#dw_at_associated)
- [`DW_AT_base_types`](#dw_at_base_types)
- [`DW_AT_binary_scale`](#dw_at_binary_scale)
- [`DW_AT_bit_offset`](#dw_at_bit_offset)
- [`DW_AT_bit_size`](#dw_at_bit_size)
- [`DW_AT_bit_stride`](#dw_at_bit_stride)
- [`DW_AT_body_begin`](#dw_at_body_begin)
- [`DW_AT_body_end`](#dw_at_body_end)
- [`DW_AT_byte_size`](#dw_at_byte_size)
- [`DW_AT_byte_stride`](#dw_at_byte_stride)
- [`DW_AT_call_all_calls`](#dw_at_call_all_calls)
- [`DW_AT_call_all_source_calls`](#dw_at_call_all_source_calls)
- [`DW_AT_call_all_tail_calls`](#dw_at_call_all_tail_calls)
- [`DW_AT_call_column`](#dw_at_call_column)
- [`DW_AT_call_data_location`](#dw_at_call_data_location)
- [`DW_AT_call_data_value`](#dw_at_call_data_value)
- [`DW_AT_call_file`](#dw_at_call_file)
- [`DW_AT_call_line`](#dw_at_call_line)
- [`DW_AT_call_origin`](#dw_at_call_origin)
- [`DW_AT_call_parameter`](#dw_at_call_parameter)
- [`DW_AT_call_pc`](#dw_at_call_pc)
- [`DW_AT_call_return_pc`](#dw_at_call_return_pc)
- [`DW_AT_call_tail_call`](#dw_at_call_tail_call)
- [`DW_AT_call_target`](#dw_at_call_target)
- [`DW_AT_call_target_clobbered`](#dw_at_call_target_clobbered)
- [`DW_AT_call_value`](#dw_at_call_value)
- [`DW_AT_calling_convention`](#dw_at_calling_convention)
- [`DW_AT_common_reference`](#dw_at_common_reference)
- [`DW_AT_comp_dir`](#dw_at_comp_dir)
- [`DW_AT_const_expr`](#dw_at_const_expr)
- [`DW_AT_const_value`](#dw_at_const_value)
- [`DW_AT_containing_type`](#dw_at_containing_type)
- [`DW_AT_count`](#dw_at_count)
- [`DW_AT_data_bit_offset`](#dw_at_data_bit_offset)
- [`DW_AT_data_location`](#dw_at_data_location)
- [`DW_AT_data_member_location`](#dw_at_data_member_location)
- [`DW_AT_decimal_scale`](#dw_at_decimal_scale)
- [`DW_AT_decimal_sign`](#dw_at_decimal_sign)
- [`DW_AT_decl_column`](#dw_at_decl_column)
- [`DW_AT_decl_file`](#dw_at_decl_file)
- [`DW_AT_decl_line`](#dw_at_decl_line)
- [`DW_AT_declaration`](#dw_at_declaration)
- [`DW_AT_default_value`](#dw_at_default_value)
- [`DW_AT_defaulted`](#dw_at_defaulted)
- [`DW_AT_deleted`](#dw_at_deleted)
- [`DW_AT_description`](#dw_at_description)
- [`DW_AT_digit_count`](#dw_at_digit_count)
- [`DW_AT_discr`](#dw_at_discr)
- [`DW_AT_discr_list`](#dw_at_discr_list)
- [`DW_AT_discr_value`](#dw_at_discr_value)
- [`DW_AT_dwo_name`](#dw_at_dwo_name)
- [`DW_AT_element_list`](#dw_at_element_list)
- [`DW_AT_elemental`](#dw_at_elemental)
- [`DW_AT_encoding`](#dw_at_encoding)
- [`DW_AT_endianity`](#dw_at_endianity)
- [`DW_AT_entry_pc`](#dw_at_entry_pc)
- [`DW_AT_enum_class`](#dw_at_enum_class)
- [`DW_AT_explicit`](#dw_at_explicit)
- [`DW_AT_export_symbols`](#dw_at_export_symbols)
- [`DW_AT_extension`](#dw_at_extension)
- [`DW_AT_external`](#dw_at_external)
- [`DW_AT_frame_base`](#dw_at_frame_base)
- [`DW_AT_friend`](#dw_at_friend)
- [`DW_AT_friends`](#dw_at_friends)
- [`DW_AT_fund_type`](#dw_at_fund_type)
- [`DW_AT_hi_user`](#dw_at_hi_user)
- [`DW_AT_high_pc`](#dw_at_high_pc)
- [`DW_AT_identifier_case`](#dw_at_identifier_case)
- [`DW_AT_import`](#dw_at_import)
- [`DW_AT_inline`](#dw_at_inline)
- [`DW_AT_is_optional`](#dw_at_is_optional)
- [`DW_AT_language`](#dw_at_language)
- [`DW_AT_linkage_name`](#dw_at_linkage_name)
- [`DW_AT_lo_user`](#dw_at_lo_user)
- [`DW_AT_location`](#dw_at_location)
- [`DW_AT_loclists_base`](#dw_at_loclists_base)
- [`DW_AT_low_pc`](#dw_at_low_pc)
- [`DW_AT_lower_bound`](#dw_at_lower_bound)
- [`DW_AT_mac_info`](#dw_at_mac_info)
- [`DW_AT_macro_info`](#dw_at_macro_info)
- [`DW_AT_macros`](#dw_at_macros)
- [`DW_AT_main_subprogram`](#dw_at_main_subprogram)
- [`DW_AT_member`](#dw_at_member)
- [`DW_AT_mod_fund_type`](#dw_at_mod_fund_type)
- [`DW_AT_mod_u_d_type`](#dw_at_mod_u_d_type)
- [`DW_AT_mutable`](#dw_at_mutable)
- [`DW_AT_name`](#dw_at_name)
- [`DW_AT_namelist_item`](#dw_at_namelist_item)
- [`DW_AT_noreturn`](#dw_at_noreturn)
- [`DW_AT_null`](#dw_at_null)
- [`DW_AT_object_pointer`](#dw_at_object_pointer)
- [`DW_AT_ordering`](#dw_at_ordering)
- [`DW_AT_picture_string`](#dw_at_picture_string)
- [`DW_AT_priority`](#dw_at_priority)
- [`DW_AT_private`](#dw_at_private)
- [`DW_AT_producer`](#dw_at_producer)
- [`DW_AT_program`](#dw_at_program)
- [`DW_AT_protected`](#dw_at_protected)
- [`DW_AT_prototyped`](#dw_at_prototyped)
- [`DW_AT_public`](#dw_at_public)
- [`DW_AT_pure`](#dw_at_pure)
- [`DW_AT_pure_virtual`](#dw_at_pure_virtual)
- [`DW_AT_ranges`](#dw_at_ranges)
- [`DW_AT_rank`](#dw_at_rank)
- [`DW_AT_recursive`](#dw_at_recursive)
- [`DW_AT_reference`](#dw_at_reference)
- [`DW_AT_return_addr`](#dw_at_return_addr)
- [`DW_AT_rnglists_base`](#dw_at_rnglists_base)
- [`DW_AT_rvalue_reference`](#dw_at_rvalue_reference)
- [`DW_AT_segment`](#dw_at_segment)
- [`DW_AT_sf_names`](#dw_at_sf_names)
- [`DW_AT_sibling`](#dw_at_sibling)
- [`DW_AT_signature`](#dw_at_signature)
- [`DW_AT_small`](#dw_at_small)
- [`DW_AT_specification`](#dw_at_specification)
- [`DW_AT_specification_v1`](#dw_at_specification_v1)
- [`DW_AT_src_coords`](#dw_at_src_coords)
- [`DW_AT_src_info`](#dw_at_src_info)
- [`DW_AT_start_scope`](#dw_at_start_scope)
- [`DW_AT_static_link`](#dw_at_static_link)
- [`DW_AT_stmt_list`](#dw_at_stmt_list)
- [`DW_AT_str_offsets_base`](#dw_at_str_offsets_base)
- [`DW_AT_string_length`](#dw_at_string_length)
- [`DW_AT_string_length_bit_size`](#dw_at_string_length_bit_size)
- [`DW_AT_string_length_byte_size`](#dw_at_string_length_byte_size)
- [`DW_AT_subscr_data`](#dw_at_subscr_data)
- [`DW_AT_threads_scaled`](#dw_at_threads_scaled)
- [`DW_AT_trampoline`](#dw_at_trampoline)
- [`DW_AT_type`](#dw_at_type)
- [`DW_AT_upc_threads_scaled`](#dw_at_upc_threads_scaled)
- [`DW_AT_upper_bound`](#dw_at_upper_bound)
- [`DW_AT_use_GNAT_descriptive_type`](#dw_at_use_gnat_descriptive_type)
- [`DW_AT_use_UTF8`](#dw_at_use_utf8)
- [`DW_AT_use_location`](#dw_at_use_location)
- [`DW_AT_user_def_type`](#dw_at_user_def_type)
- [`DW_AT_variable_parameter`](#dw_at_variable_parameter)
- [`DW_AT_virtual`](#dw_at_virtual)
- [`DW_AT_virtuality`](#dw_at_virtuality)
- [`DW_AT_visibility`](#dw_at_visibility)
- [`DW_AT_vtable_elem_location`](#dw_at_vtable_elem_location)
- [`DW_CC_hi_user`](#dw_cc_hi_user)
- [`DW_CC_lo_user`](#dw_cc_lo_user)
- [`DW_CC_nocall`](#dw_cc_nocall)
- [`DW_CC_normal`](#dw_cc_normal)
- [`DW_CC_pass_by_reference`](#dw_cc_pass_by_reference)
- [`DW_CC_pass_by_value`](#dw_cc_pass_by_value)
- [`DW_CC_program`](#dw_cc_program)
- [`DW_CFA_AARCH64_negate_ra_state`](#dw_cfa_aarch64_negate_ra_state)
- [`DW_CFA_GNU_args_size`](#dw_cfa_gnu_args_size)
- [`DW_CFA_GNU_negative_offset_extended`](#dw_cfa_gnu_negative_offset_extended)
- [`DW_CFA_GNU_window_save`](#dw_cfa_gnu_window_save)
- [`DW_CFA_MIPS_advance_loc8`](#dw_cfa_mips_advance_loc8)
- [`DW_CFA_advance_loc`](#dw_cfa_advance_loc)
- [`DW_CFA_advance_loc1`](#dw_cfa_advance_loc1)
- [`DW_CFA_advance_loc2`](#dw_cfa_advance_loc2)
- [`DW_CFA_advance_loc4`](#dw_cfa_advance_loc4)
- [`DW_CFA_def_cfa`](#dw_cfa_def_cfa)
- [`DW_CFA_def_cfa_expression`](#dw_cfa_def_cfa_expression)
- [`DW_CFA_def_cfa_offset`](#dw_cfa_def_cfa_offset)
- [`DW_CFA_def_cfa_offset_sf`](#dw_cfa_def_cfa_offset_sf)
- [`DW_CFA_def_cfa_register`](#dw_cfa_def_cfa_register)
- [`DW_CFA_def_cfa_sf`](#dw_cfa_def_cfa_sf)
- [`DW_CFA_expression`](#dw_cfa_expression)
- [`DW_CFA_hi_user`](#dw_cfa_hi_user)
- [`DW_CFA_lo_user`](#dw_cfa_lo_user)
- [`DW_CFA_nop`](#dw_cfa_nop)
- [`DW_CFA_offset`](#dw_cfa_offset)
- [`DW_CFA_offset_extended`](#dw_cfa_offset_extended)
- [`DW_CFA_offset_extended_sf`](#dw_cfa_offset_extended_sf)
- [`DW_CFA_register`](#dw_cfa_register)
- [`DW_CFA_remember_state`](#dw_cfa_remember_state)
- [`DW_CFA_restore`](#dw_cfa_restore)
- [`DW_CFA_restore_extended`](#dw_cfa_restore_extended)
- [`DW_CFA_restore_state`](#dw_cfa_restore_state)
- [`DW_CFA_same_value`](#dw_cfa_same_value)
- [`DW_CFA_set_loc`](#dw_cfa_set_loc)
- [`DW_CFA_undefined`](#dw_cfa_undefined)
- [`DW_CFA_val_expression`](#dw_cfa_val_expression)
- [`DW_CFA_val_offset`](#dw_cfa_val_offset)
- [`DW_CFA_val_offset_sf`](#dw_cfa_val_offset_sf)
- [`DW_CHILDREN_no`](#dw_children_no)
- [`DW_CHILDREN_yes`](#dw_children_yes)
- [`DW_DEFAULTED_in_class`](#dw_defaulted_in_class)
- [`DW_DEFAULTED_no`](#dw_defaulted_no)
- [`DW_DEFAULTED_out_of_class`](#dw_defaulted_out_of_class)
- [`DW_DSC_label`](#dw_dsc_label)
- [`DW_DSC_range`](#dw_dsc_range)
- [`DW_DS_leading_overpunch`](#dw_ds_leading_overpunch)
- [`DW_DS_leading_separate`](#dw_ds_leading_separate)
- [`DW_DS_trailing_overpunch`](#dw_ds_trailing_overpunch)
- [`DW_DS_trailing_separate`](#dw_ds_trailing_separate)
- [`DW_DS_unsigned`](#dw_ds_unsigned)
- [`DW_EH_PE_absptr`](#dw_eh_pe_absptr)
- [`DW_EH_PE_aligned`](#dw_eh_pe_aligned)
- [`DW_EH_PE_datarel`](#dw_eh_pe_datarel)
- [`DW_EH_PE_funcrel`](#dw_eh_pe_funcrel)
- [`DW_EH_PE_indirect`](#dw_eh_pe_indirect)
- [`DW_EH_PE_omit`](#dw_eh_pe_omit)
- [`DW_EH_PE_pcrel`](#dw_eh_pe_pcrel)
- [`DW_EH_PE_sdata2`](#dw_eh_pe_sdata2)
- [`DW_EH_PE_sdata4`](#dw_eh_pe_sdata4)
- [`DW_EH_PE_sdata8`](#dw_eh_pe_sdata8)
- [`DW_EH_PE_sleb128`](#dw_eh_pe_sleb128)
- [`DW_EH_PE_textrel`](#dw_eh_pe_textrel)
- [`DW_EH_PE_udata2`](#dw_eh_pe_udata2)
- [`DW_EH_PE_udata4`](#dw_eh_pe_udata4)
- [`DW_EH_PE_udata8`](#dw_eh_pe_udata8)
- [`DW_EH_PE_uleb128`](#dw_eh_pe_uleb128)
- [`DW_END_big`](#dw_end_big)
- [`DW_END_default`](#dw_end_default)
- [`DW_END_hi_user`](#dw_end_hi_user)
- [`DW_END_little`](#dw_end_little)
- [`DW_END_lo_user`](#dw_end_lo_user)
- [`DW_FORM_GNU_addr_index`](#dw_form_gnu_addr_index)
- [`DW_FORM_GNU_ref_alt`](#dw_form_gnu_ref_alt)
- [`DW_FORM_GNU_str_index`](#dw_form_gnu_str_index)
- [`DW_FORM_GNU_strp_alt`](#dw_form_gnu_strp_alt)
- [`DW_FORM_addr`](#dw_form_addr)
- [`DW_FORM_addrx`](#dw_form_addrx)
- [`DW_FORM_addrx1`](#dw_form_addrx1)
- [`DW_FORM_addrx2`](#dw_form_addrx2)
- [`DW_FORM_addrx3`](#dw_form_addrx3)
- [`DW_FORM_addrx4`](#dw_form_addrx4)
- [`DW_FORM_block`](#dw_form_block)
- [`DW_FORM_block1`](#dw_form_block1)
- [`DW_FORM_block2`](#dw_form_block2)
- [`DW_FORM_block4`](#dw_form_block4)
- [`DW_FORM_data1`](#dw_form_data1)
- [`DW_FORM_data16`](#dw_form_data16)
- [`DW_FORM_data2`](#dw_form_data2)
- [`DW_FORM_data4`](#dw_form_data4)
- [`DW_FORM_data8`](#dw_form_data8)
- [`DW_FORM_exprloc`](#dw_form_exprloc)
- [`DW_FORM_flag`](#dw_form_flag)
- [`DW_FORM_flag_present`](#dw_form_flag_present)
- [`DW_FORM_implicit_const`](#dw_form_implicit_const)
- [`DW_FORM_indirect`](#dw_form_indirect)
- [`DW_FORM_line_strp`](#dw_form_line_strp)
- [`DW_FORM_loclistx`](#dw_form_loclistx)
- [`DW_FORM_null`](#dw_form_null)
- [`DW_FORM_ref`](#dw_form_ref)
- [`DW_FORM_ref1`](#dw_form_ref1)
- [`DW_FORM_ref2`](#dw_form_ref2)
- [`DW_FORM_ref4`](#dw_form_ref4)
- [`DW_FORM_ref8`](#dw_form_ref8)
- [`DW_FORM_ref_addr`](#dw_form_ref_addr)
- [`DW_FORM_ref_sig8`](#dw_form_ref_sig8)
- [`DW_FORM_ref_sup4`](#dw_form_ref_sup4)
- [`DW_FORM_ref_sup8`](#dw_form_ref_sup8)
- [`DW_FORM_ref_udata`](#dw_form_ref_udata)
- [`DW_FORM_rnglistx`](#dw_form_rnglistx)
- [`DW_FORM_sdata`](#dw_form_sdata)
- [`DW_FORM_sec_offset`](#dw_form_sec_offset)
- [`DW_FORM_string`](#dw_form_string)
- [`DW_FORM_strp`](#dw_form_strp)
- [`DW_FORM_strp_sup`](#dw_form_strp_sup)
- [`DW_FORM_strx`](#dw_form_strx)
- [`DW_FORM_strx1`](#dw_form_strx1)
- [`DW_FORM_strx2`](#dw_form_strx2)
- [`DW_FORM_strx3`](#dw_form_strx3)
- [`DW_FORM_strx4`](#dw_form_strx4)
- [`DW_FORM_udata`](#dw_form_udata)
- [`DW_IDX_compile_unit`](#dw_idx_compile_unit)
- [`DW_IDX_die_offset`](#dw_idx_die_offset)
- [`DW_IDX_hi_user`](#dw_idx_hi_user)
- [`DW_IDX_lo_user`](#dw_idx_lo_user)
- [`DW_IDX_parent`](#dw_idx_parent)
- [`DW_IDX_type_hash`](#dw_idx_type_hash)
- [`DW_IDX_type_unit`](#dw_idx_type_unit)
- [`DW_ID_case_insensitive`](#dw_id_case_insensitive)
- [`DW_ID_case_sensitive`](#dw_id_case_sensitive)
- [`DW_ID_down_case`](#dw_id_down_case)
- [`DW_ID_up_case`](#dw_id_up_case)
- [`DW_INL_declared_inlined`](#dw_inl_declared_inlined)
- [`DW_INL_declared_not_inlined`](#dw_inl_declared_not_inlined)
- [`DW_INL_inlined`](#dw_inl_inlined)
- [`DW_INL_not_inlined`](#dw_inl_not_inlined)
- [`DW_LANG_ALTIUM_Assembler`](#dw_lang_altium_assembler)
- [`DW_LANG_Ada2005`](#dw_lang_ada2005)
- [`DW_LANG_Ada2012`](#dw_lang_ada2012)
- [`DW_LANG_Ada83`](#dw_lang_ada83)
- [`DW_LANG_Ada95`](#dw_lang_ada95)
- [`DW_LANG_BLISS`](#dw_lang_bliss)
- [`DW_LANG_BORLAND_Delphi`](#dw_lang_borland_delphi)
- [`DW_LANG_C`](#dw_lang_c)
- [`DW_LANG_C11`](#dw_lang_c11)
- [`DW_LANG_C17`](#dw_lang_c17)
- [`DW_LANG_C89`](#dw_lang_c89)
- [`DW_LANG_C99`](#dw_lang_c99)
- [`DW_LANG_C_plus_plus`](#dw_lang_c_plus_plus)
- [`DW_LANG_C_plus_plus_03`](#dw_lang_c_plus_plus_03)
- [`DW_LANG_C_plus_plus_11`](#dw_lang_c_plus_plus_11)
- [`DW_LANG_C_plus_plus_14`](#dw_lang_c_plus_plus_14)
- [`DW_LANG_C_plus_plus_17`](#dw_lang_c_plus_plus_17)
- [`DW_LANG_C_plus_plus_20`](#dw_lang_c_plus_plus_20)
- [`DW_LANG_Cobol74`](#dw_lang_cobol74)
- [`DW_LANG_Cobol85`](#dw_lang_cobol85)
- [`DW_LANG_Crystal`](#dw_lang_crystal)
- [`DW_LANG_D`](#dw_lang_d)
- [`DW_LANG_Dylan`](#dw_lang_dylan)
- [`DW_LANG_Fortran03`](#dw_lang_fortran03)
- [`DW_LANG_Fortran08`](#dw_lang_fortran08)
- [`DW_LANG_Fortran18`](#dw_lang_fortran18)
- [`DW_LANG_Fortran77`](#dw_lang_fortran77)
- [`DW_LANG_Fortran90`](#dw_lang_fortran90)
- [`DW_LANG_Fortran95`](#dw_lang_fortran95)
- [`DW_LANG_GOOGLE_RenderScript`](#dw_lang_google_renderscript)
- [`DW_LANG_Go`](#dw_lang_go)
- [`DW_LANG_Haskell`](#dw_lang_haskell)
- [`DW_LANG_Java`](#dw_lang_java)
- [`DW_LANG_Julia`](#dw_lang_julia)
- [`DW_LANG_Kotlin`](#dw_lang_kotlin)
- [`DW_LANG_Mips_Assembler`](#dw_lang_mips_assembler)
- [`DW_LANG_Modula2`](#dw_lang_modula2)
- [`DW_LANG_Modula3`](#dw_lang_modula3)
- [`DW_LANG_OCaml`](#dw_lang_ocaml)
- [`DW_LANG_ObjC`](#dw_lang_objc)
- [`DW_LANG_ObjC_plus_plus`](#dw_lang_objc_plus_plus)
- [`DW_LANG_OpenCL`](#dw_lang_opencl)
- [`DW_LANG_PLI`](#dw_lang_pli)
- [`DW_LANG_Pascal83`](#dw_lang_pascal83)
- [`DW_LANG_Python`](#dw_lang_python)
- [`DW_LANG_RenderScript`](#dw_lang_renderscript)
- [`DW_LANG_Rust`](#dw_lang_rust)
- [`DW_LANG_SUN_Assembler`](#dw_lang_sun_assembler)
- [`DW_LANG_Swift`](#dw_lang_swift)
- [`DW_LANG_UPC`](#dw_lang_upc)
- [`DW_LANG_Zig`](#dw_lang_zig)
- [`DW_LANG_hi_user`](#dw_lang_hi_user)
- [`DW_LANG_lo_user`](#dw_lang_lo_user)
- [`DW_LLE_GNU_view_pair`](#dw_lle_gnu_view_pair)
- [`DW_LLE_base_address`](#dw_lle_base_address)
- [`DW_LLE_base_addressx`](#dw_lle_base_addressx)
- [`DW_LLE_default_location`](#dw_lle_default_location)
- [`DW_LLE_end_of_list`](#dw_lle_end_of_list)
- [`DW_LLE_offset_pair`](#dw_lle_offset_pair)
- [`DW_LLE_start_end`](#dw_lle_start_end)
- [`DW_LLE_start_length`](#dw_lle_start_length)
- [`DW_LLE_startx_endx`](#dw_lle_startx_endx)
- [`DW_LLE_startx_length`](#dw_lle_startx_length)
- [`DW_LNCT_LLVM_source`](#dw_lnct_llvm_source)
- [`DW_LNCT_MD5`](#dw_lnct_md5)
- [`DW_LNCT_directory_index`](#dw_lnct_directory_index)
- [`DW_LNCT_hi_user`](#dw_lnct_hi_user)
- [`DW_LNCT_lo_user`](#dw_lnct_lo_user)
- [`DW_LNCT_path`](#dw_lnct_path)
- [`DW_LNCT_size`](#dw_lnct_size)
- [`DW_LNCT_timestamp`](#dw_lnct_timestamp)
- [`DW_LNE_define_file`](#dw_lne_define_file)
- [`DW_LNE_end_sequence`](#dw_lne_end_sequence)
- [`DW_LNE_hi_user`](#dw_lne_hi_user)
- [`DW_LNE_lo_user`](#dw_lne_lo_user)
- [`DW_LNE_set_address`](#dw_lne_set_address)
- [`DW_LNE_set_discriminator`](#dw_lne_set_discriminator)
- [`DW_LNS_advance_line`](#dw_lns_advance_line)
- [`DW_LNS_advance_pc`](#dw_lns_advance_pc)
- [`DW_LNS_const_add_pc`](#dw_lns_const_add_pc)
- [`DW_LNS_copy`](#dw_lns_copy)
- [`DW_LNS_fixed_advance_pc`](#dw_lns_fixed_advance_pc)
- [`DW_LNS_negate_stmt`](#dw_lns_negate_stmt)
- [`DW_LNS_set_basic_block`](#dw_lns_set_basic_block)
- [`DW_LNS_set_column`](#dw_lns_set_column)
- [`DW_LNS_set_epilogue_begin`](#dw_lns_set_epilogue_begin)
- [`DW_LNS_set_file`](#dw_lns_set_file)
- [`DW_LNS_set_isa`](#dw_lns_set_isa)
- [`DW_LNS_set_prologue_end`](#dw_lns_set_prologue_end)
- [`DW_MACINFO_define`](#dw_macinfo_define)
- [`DW_MACINFO_end_file`](#dw_macinfo_end_file)
- [`DW_MACINFO_start_file`](#dw_macinfo_start_file)
- [`DW_MACINFO_undef`](#dw_macinfo_undef)
- [`DW_MACINFO_vendor_ext`](#dw_macinfo_vendor_ext)
- [`DW_MACRO_define`](#dw_macro_define)
- [`DW_MACRO_define_strp`](#dw_macro_define_strp)
- [`DW_MACRO_define_strx`](#dw_macro_define_strx)
- [`DW_MACRO_define_sup`](#dw_macro_define_sup)
- [`DW_MACRO_end_file`](#dw_macro_end_file)
- [`DW_MACRO_hi_user`](#dw_macro_hi_user)
- [`DW_MACRO_import`](#dw_macro_import)
- [`DW_MACRO_import_sup`](#dw_macro_import_sup)
- [`DW_MACRO_lo_user`](#dw_macro_lo_user)
- [`DW_MACRO_start_file`](#dw_macro_start_file)
- [`DW_MACRO_undef`](#dw_macro_undef)
- [`DW_MACRO_undef_strp`](#dw_macro_undef_strp)
- [`DW_MACRO_undef_strx`](#dw_macro_undef_strx)
- [`DW_MACRO_undef_sup`](#dw_macro_undef_sup)
- [`DW_OP_GNU_addr_index`](#dw_op_gnu_addr_index)
- [`DW_OP_GNU_const_index`](#dw_op_gnu_const_index)
- [`DW_OP_GNU_const_type`](#dw_op_gnu_const_type)
- [`DW_OP_GNU_convert`](#dw_op_gnu_convert)
- [`DW_OP_GNU_deref_type`](#dw_op_gnu_deref_type)
- [`DW_OP_GNU_entry_value`](#dw_op_gnu_entry_value)
- [`DW_OP_GNU_implicit_pointer`](#dw_op_gnu_implicit_pointer)
- [`DW_OP_GNU_parameter_ref`](#dw_op_gnu_parameter_ref)
- [`DW_OP_GNU_push_tls_address`](#dw_op_gnu_push_tls_address)
- [`DW_OP_GNU_regval_type`](#dw_op_gnu_regval_type)
- [`DW_OP_GNU_reinterpret`](#dw_op_gnu_reinterpret)
- [`DW_OP_WASM_location`](#dw_op_wasm_location)
- [`DW_OP_abs`](#dw_op_abs)
- [`DW_OP_addr`](#dw_op_addr)
- [`DW_OP_addrx`](#dw_op_addrx)
- [`DW_OP_and`](#dw_op_and)
- [`DW_OP_bit_piece`](#dw_op_bit_piece)
- [`DW_OP_bra`](#dw_op_bra)
- [`DW_OP_breg0`](#dw_op_breg0)
- [`DW_OP_breg1`](#dw_op_breg1)
- [`DW_OP_breg10`](#dw_op_breg10)
- [`DW_OP_breg11`](#dw_op_breg11)
- [`DW_OP_breg12`](#dw_op_breg12)
- [`DW_OP_breg13`](#dw_op_breg13)
- [`DW_OP_breg14`](#dw_op_breg14)
- [`DW_OP_breg15`](#dw_op_breg15)
- [`DW_OP_breg16`](#dw_op_breg16)
- [`DW_OP_breg17`](#dw_op_breg17)
- [`DW_OP_breg18`](#dw_op_breg18)
- [`DW_OP_breg19`](#dw_op_breg19)
- [`DW_OP_breg2`](#dw_op_breg2)
- [`DW_OP_breg20`](#dw_op_breg20)
- [`DW_OP_breg21`](#dw_op_breg21)
- [`DW_OP_breg22`](#dw_op_breg22)
- [`DW_OP_breg23`](#dw_op_breg23)
- [`DW_OP_breg24`](#dw_op_breg24)
- [`DW_OP_breg25`](#dw_op_breg25)
- [`DW_OP_breg26`](#dw_op_breg26)
- [`DW_OP_breg27`](#dw_op_breg27)
- [`DW_OP_breg28`](#dw_op_breg28)
- [`DW_OP_breg29`](#dw_op_breg29)
- [`DW_OP_breg3`](#dw_op_breg3)
- [`DW_OP_breg30`](#dw_op_breg30)
- [`DW_OP_breg31`](#dw_op_breg31)
- [`DW_OP_breg4`](#dw_op_breg4)
- [`DW_OP_breg5`](#dw_op_breg5)
- [`DW_OP_breg6`](#dw_op_breg6)
- [`DW_OP_breg7`](#dw_op_breg7)
- [`DW_OP_breg8`](#dw_op_breg8)
- [`DW_OP_breg9`](#dw_op_breg9)
- [`DW_OP_bregx`](#dw_op_bregx)
- [`DW_OP_call2`](#dw_op_call2)
- [`DW_OP_call4`](#dw_op_call4)
- [`DW_OP_call_frame_cfa`](#dw_op_call_frame_cfa)
- [`DW_OP_call_ref`](#dw_op_call_ref)
- [`DW_OP_const1s`](#dw_op_const1s)
- [`DW_OP_const1u`](#dw_op_const1u)
- [`DW_OP_const2s`](#dw_op_const2s)
- [`DW_OP_const2u`](#dw_op_const2u)
- [`DW_OP_const4s`](#dw_op_const4s)
- [`DW_OP_const4u`](#dw_op_const4u)
- [`DW_OP_const8s`](#dw_op_const8s)
- [`DW_OP_const8u`](#dw_op_const8u)
- [`DW_OP_const_type`](#dw_op_const_type)
- [`DW_OP_consts`](#dw_op_consts)
- [`DW_OP_constu`](#dw_op_constu)
- [`DW_OP_constx`](#dw_op_constx)
- [`DW_OP_convert`](#dw_op_convert)
- [`DW_OP_deref`](#dw_op_deref)
- [`DW_OP_deref_size`](#dw_op_deref_size)
- [`DW_OP_deref_type`](#dw_op_deref_type)
- [`DW_OP_div`](#dw_op_div)
- [`DW_OP_drop`](#dw_op_drop)
- [`DW_OP_dup`](#dw_op_dup)
- [`DW_OP_entry_value`](#dw_op_entry_value)
- [`DW_OP_eq`](#dw_op_eq)
- [`DW_OP_fbreg`](#dw_op_fbreg)
- [`DW_OP_form_tls_address`](#dw_op_form_tls_address)
- [`DW_OP_ge`](#dw_op_ge)
- [`DW_OP_gt`](#dw_op_gt)
- [`DW_OP_implicit_pointer`](#dw_op_implicit_pointer)
- [`DW_OP_implicit_value`](#dw_op_implicit_value)
- [`DW_OP_le`](#dw_op_le)
- [`DW_OP_lit0`](#dw_op_lit0)
- [`DW_OP_lit1`](#dw_op_lit1)
- [`DW_OP_lit10`](#dw_op_lit10)
- [`DW_OP_lit11`](#dw_op_lit11)
- [`DW_OP_lit12`](#dw_op_lit12)
- [`DW_OP_lit13`](#dw_op_lit13)
- [`DW_OP_lit14`](#dw_op_lit14)
- [`DW_OP_lit15`](#dw_op_lit15)
- [`DW_OP_lit16`](#dw_op_lit16)
- [`DW_OP_lit17`](#dw_op_lit17)
- [`DW_OP_lit18`](#dw_op_lit18)
- [`DW_OP_lit19`](#dw_op_lit19)
- [`DW_OP_lit2`](#dw_op_lit2)
- [`DW_OP_lit20`](#dw_op_lit20)
- [`DW_OP_lit21`](#dw_op_lit21)
- [`DW_OP_lit22`](#dw_op_lit22)
- [`DW_OP_lit23`](#dw_op_lit23)
- [`DW_OP_lit24`](#dw_op_lit24)
- [`DW_OP_lit25`](#dw_op_lit25)
- [`DW_OP_lit26`](#dw_op_lit26)
- [`DW_OP_lit27`](#dw_op_lit27)
- [`DW_OP_lit28`](#dw_op_lit28)
- [`DW_OP_lit29`](#dw_op_lit29)
- [`DW_OP_lit3`](#dw_op_lit3)
- [`DW_OP_lit30`](#dw_op_lit30)
- [`DW_OP_lit31`](#dw_op_lit31)
- [`DW_OP_lit4`](#dw_op_lit4)
- [`DW_OP_lit5`](#dw_op_lit5)
- [`DW_OP_lit6`](#dw_op_lit6)
- [`DW_OP_lit7`](#dw_op_lit7)
- [`DW_OP_lit8`](#dw_op_lit8)
- [`DW_OP_lit9`](#dw_op_lit9)
- [`DW_OP_lt`](#dw_op_lt)
- [`DW_OP_minus`](#dw_op_minus)
- [`DW_OP_mod`](#dw_op_mod)
- [`DW_OP_mul`](#dw_op_mul)
- [`DW_OP_ne`](#dw_op_ne)
- [`DW_OP_neg`](#dw_op_neg)
- [`DW_OP_nop`](#dw_op_nop)
- [`DW_OP_not`](#dw_op_not)
- [`DW_OP_or`](#dw_op_or)
- [`DW_OP_over`](#dw_op_over)
- [`DW_OP_pick`](#dw_op_pick)
- [`DW_OP_piece`](#dw_op_piece)
- [`DW_OP_plus`](#dw_op_plus)
- [`DW_OP_plus_uconst`](#dw_op_plus_uconst)
- [`DW_OP_push_object_address`](#dw_op_push_object_address)
- [`DW_OP_reg0`](#dw_op_reg0)
- [`DW_OP_reg1`](#dw_op_reg1)
- [`DW_OP_reg10`](#dw_op_reg10)
- [`DW_OP_reg11`](#dw_op_reg11)
- [`DW_OP_reg12`](#dw_op_reg12)
- [`DW_OP_reg13`](#dw_op_reg13)
- [`DW_OP_reg14`](#dw_op_reg14)
- [`DW_OP_reg15`](#dw_op_reg15)
- [`DW_OP_reg16`](#dw_op_reg16)
- [`DW_OP_reg17`](#dw_op_reg17)
- [`DW_OP_reg18`](#dw_op_reg18)
- [`DW_OP_reg19`](#dw_op_reg19)
- [`DW_OP_reg2`](#dw_op_reg2)
- [`DW_OP_reg20`](#dw_op_reg20)
- [`DW_OP_reg21`](#dw_op_reg21)
- [`DW_OP_reg22`](#dw_op_reg22)
- [`DW_OP_reg23`](#dw_op_reg23)
- [`DW_OP_reg24`](#dw_op_reg24)
- [`DW_OP_reg25`](#dw_op_reg25)
- [`DW_OP_reg26`](#dw_op_reg26)
- [`DW_OP_reg27`](#dw_op_reg27)
- [`DW_OP_reg28`](#dw_op_reg28)
- [`DW_OP_reg29`](#dw_op_reg29)
- [`DW_OP_reg3`](#dw_op_reg3)
- [`DW_OP_reg30`](#dw_op_reg30)
- [`DW_OP_reg31`](#dw_op_reg31)
- [`DW_OP_reg4`](#dw_op_reg4)
- [`DW_OP_reg5`](#dw_op_reg5)
- [`DW_OP_reg6`](#dw_op_reg6)
- [`DW_OP_reg7`](#dw_op_reg7)
- [`DW_OP_reg8`](#dw_op_reg8)
- [`DW_OP_reg9`](#dw_op_reg9)
- [`DW_OP_regval_type`](#dw_op_regval_type)
- [`DW_OP_regx`](#dw_op_regx)
- [`DW_OP_reinterpret`](#dw_op_reinterpret)
- [`DW_OP_rot`](#dw_op_rot)
- [`DW_OP_shl`](#dw_op_shl)
- [`DW_OP_shr`](#dw_op_shr)
- [`DW_OP_shra`](#dw_op_shra)
- [`DW_OP_skip`](#dw_op_skip)
- [`DW_OP_stack_value`](#dw_op_stack_value)
- [`DW_OP_swap`](#dw_op_swap)
- [`DW_OP_xderef`](#dw_op_xderef)
- [`DW_OP_xderef_size`](#dw_op_xderef_size)
- [`DW_OP_xderef_type`](#dw_op_xderef_type)
- [`DW_OP_xor`](#dw_op_xor)
- [`DW_ORD_col_major`](#dw_ord_col_major)
- [`DW_ORD_row_major`](#dw_ord_row_major)
- [`DW_RLE_base_address`](#dw_rle_base_address)
- [`DW_RLE_base_addressx`](#dw_rle_base_addressx)
- [`DW_RLE_end_of_list`](#dw_rle_end_of_list)
- [`DW_RLE_offset_pair`](#dw_rle_offset_pair)
- [`DW_RLE_start_end`](#dw_rle_start_end)
- [`DW_RLE_start_length`](#dw_rle_start_length)
- [`DW_RLE_startx_endx`](#dw_rle_startx_endx)
- [`DW_RLE_startx_length`](#dw_rle_startx_length)
- [`DW_SECT_ABBREV`](#dw_sect_abbrev)
- [`DW_SECT_INFO`](#dw_sect_info)
- [`DW_SECT_LINE`](#dw_sect_line)
- [`DW_SECT_LOCLISTS`](#dw_sect_loclists)
- [`DW_SECT_MACRO`](#dw_sect_macro)
- [`DW_SECT_RNGLISTS`](#dw_sect_rnglists)
- [`DW_SECT_STR_OFFSETS`](#dw_sect_str_offsets)
- [`DW_SECT_V2_ABBREV`](#dw_sect_v2_abbrev)
- [`DW_SECT_V2_INFO`](#dw_sect_v2_info)
- [`DW_SECT_V2_LINE`](#dw_sect_v2_line)
- [`DW_SECT_V2_LOC`](#dw_sect_v2_loc)
- [`DW_SECT_V2_MACINFO`](#dw_sect_v2_macinfo)
- [`DW_SECT_V2_MACRO`](#dw_sect_v2_macro)
- [`DW_SECT_V2_STR_OFFSETS`](#dw_sect_v2_str_offsets)
- [`DW_SECT_V2_TYPES`](#dw_sect_v2_types)
- [`DW_TAG_ALTIUM_circ_type`](#dw_tag_altium_circ_type)
- [`DW_TAG_ALTIUM_mwa_circ_type`](#dw_tag_altium_mwa_circ_type)
- [`DW_TAG_ALTIUM_rev_carry_type`](#dw_tag_altium_rev_carry_type)
- [`DW_TAG_ALTIUM_rom`](#dw_tag_altium_rom)
- [`DW_TAG_APPLE_property`](#dw_tag_apple_property)
- [`DW_TAG_BORLAND_Delphi_dynamic_array`](#dw_tag_borland_delphi_dynamic_array)
- [`DW_TAG_BORLAND_Delphi_set`](#dw_tag_borland_delphi_set)
- [`DW_TAG_BORLAND_Delphi_string`](#dw_tag_borland_delphi_string)
- [`DW_TAG_BORLAND_Delphi_variant`](#dw_tag_borland_delphi_variant)
- [`DW_TAG_BORLAND_property`](#dw_tag_borland_property)
- [`DW_TAG_GNU_BINCL`](#dw_tag_gnu_bincl)
- [`DW_TAG_GNU_EINCL`](#dw_tag_gnu_eincl)
- [`DW_TAG_GNU_call_site`](#dw_tag_gnu_call_site)
- [`DW_TAG_GNU_call_site_parameter`](#dw_tag_gnu_call_site_parameter)
- [`DW_TAG_GNU_formal_parameter_pack`](#dw_tag_gnu_formal_parameter_pack)
- [`DW_TAG_GNU_template_parameter_pack`](#dw_tag_gnu_template_parameter_pack)
- [`DW_TAG_GNU_template_template_param`](#dw_tag_gnu_template_template_param)
- [`DW_TAG_HP_Bliss_field`](#dw_tag_hp_bliss_field)
- [`DW_TAG_HP_Bliss_field_set`](#dw_tag_hp_bliss_field_set)
- [`DW_TAG_HP_array_descriptor`](#dw_tag_hp_array_descriptor)
- [`DW_TAG_MIPS_loop`](#dw_tag_mips_loop)
- [`DW_TAG_PGI_interface_block`](#dw_tag_pgi_interface_block)
- [`DW_TAG_PGI_kanji_type`](#dw_tag_pgi_kanji_type)
- [`DW_TAG_SUN_class_template`](#dw_tag_sun_class_template)
- [`DW_TAG_SUN_codeflags`](#dw_tag_sun_codeflags)
- [`DW_TAG_SUN_dtor`](#dw_tag_sun_dtor)
- [`DW_TAG_SUN_dtor_info`](#dw_tag_sun_dtor_info)
- [`DW_TAG_SUN_f90_interface`](#dw_tag_sun_f90_interface)
- [`DW_TAG_SUN_fortran_vax_structure`](#dw_tag_sun_fortran_vax_structure)
- [`DW_TAG_SUN_function_template`](#dw_tag_sun_function_template)
- [`DW_TAG_SUN_indirect_inheritance`](#dw_tag_sun_indirect_inheritance)
- [`DW_TAG_SUN_memop_info`](#dw_tag_sun_memop_info)
- [`DW_TAG_SUN_omp_child_func`](#dw_tag_sun_omp_child_func)
- [`DW_TAG_SUN_rtti_descriptor`](#dw_tag_sun_rtti_descriptor)
- [`DW_TAG_SUN_struct_template`](#dw_tag_sun_struct_template)
- [`DW_TAG_SUN_union_template`](#dw_tag_sun_union_template)
- [`DW_TAG_access_declaration`](#dw_tag_access_declaration)
- [`DW_TAG_array_type`](#dw_tag_array_type)
- [`DW_TAG_atomic_type`](#dw_tag_atomic_type)
- [`DW_TAG_base_type`](#dw_tag_base_type)
- [`DW_TAG_call_site`](#dw_tag_call_site)
- [`DW_TAG_call_site_parameter`](#dw_tag_call_site_parameter)
- [`DW_TAG_catch_block`](#dw_tag_catch_block)
- [`DW_TAG_class_template`](#dw_tag_class_template)
- [`DW_TAG_class_type`](#dw_tag_class_type)
- [`DW_TAG_coarray_type`](#dw_tag_coarray_type)
- [`DW_TAG_common_block`](#dw_tag_common_block)
- [`DW_TAG_common_inclusion`](#dw_tag_common_inclusion)
- [`DW_TAG_compile_unit`](#dw_tag_compile_unit)
- [`DW_TAG_condition`](#dw_tag_condition)
- [`DW_TAG_const_type`](#dw_tag_const_type)
- [`DW_TAG_constant`](#dw_tag_constant)
- [`DW_TAG_dwarf_procedure`](#dw_tag_dwarf_procedure)
- [`DW_TAG_dynamic_type`](#dw_tag_dynamic_type)
- [`DW_TAG_entry_point`](#dw_tag_entry_point)
- [`DW_TAG_enumeration_type`](#dw_tag_enumeration_type)
- [`DW_TAG_enumerator`](#dw_tag_enumerator)
- [`DW_TAG_file_type`](#dw_tag_file_type)
- [`DW_TAG_formal_parameter`](#dw_tag_formal_parameter)
- [`DW_TAG_format_label`](#dw_tag_format_label)
- [`DW_TAG_friend`](#dw_tag_friend)
- [`DW_TAG_function_template`](#dw_tag_function_template)
- [`DW_TAG_generic_subrange`](#dw_tag_generic_subrange)
- [`DW_TAG_global_subroutine`](#dw_tag_global_subroutine)
- [`DW_TAG_global_variable`](#dw_tag_global_variable)
- [`DW_TAG_hi_user`](#dw_tag_hi_user)
- [`DW_TAG_immutable_type`](#dw_tag_immutable_type)
- [`DW_TAG_imported_declaration`](#dw_tag_imported_declaration)
- [`DW_TAG_imported_module`](#dw_tag_imported_module)
- [`DW_TAG_imported_unit`](#dw_tag_imported_unit)
- [`DW_TAG_inheritance`](#dw_tag_inheritance)
- [`DW_TAG_inlined_subroutine`](#dw_tag_inlined_subroutine)
- [`DW_TAG_interface_type`](#dw_tag_interface_type)
- [`DW_TAG_label`](#dw_tag_label)
- [`DW_TAG_lexical_block`](#dw_tag_lexical_block)
- [`DW_TAG_lo_user`](#dw_tag_lo_user)
- [`DW_TAG_local_variable`](#dw_tag_local_variable)
- [`DW_TAG_member`](#dw_tag_member)
- [`DW_TAG_module`](#dw_tag_module)
- [`DW_TAG_namelist`](#dw_tag_namelist)
- [`DW_TAG_namelist_item`](#dw_tag_namelist_item)
- [`DW_TAG_namespace`](#dw_tag_namespace)
- [`DW_TAG_null`](#dw_tag_null)
- [`DW_TAG_packed_type`](#dw_tag_packed_type)
- [`DW_TAG_partial_unit`](#dw_tag_partial_unit)
- [`DW_TAG_pointer_type`](#dw_tag_pointer_type)
- [`DW_TAG_ptr_to_member_type`](#dw_tag_ptr_to_member_type)
- [`DW_TAG_reference_type`](#dw_tag_reference_type)
- [`DW_TAG_restrict_type`](#dw_tag_restrict_type)
- [`DW_TAG_rvalue_reference_type`](#dw_tag_rvalue_reference_type)
- [`DW_TAG_set_type`](#dw_tag_set_type)
- [`DW_TAG_shared_type`](#dw_tag_shared_type)
- [`DW_TAG_skeleton_unit`](#dw_tag_skeleton_unit)
- [`DW_TAG_string_type`](#dw_tag_string_type)
- [`DW_TAG_structure_type`](#dw_tag_structure_type)
- [`DW_TAG_subprogram`](#dw_tag_subprogram)
- [`DW_TAG_subrange_type`](#dw_tag_subrange_type)
- [`DW_TAG_subroutine`](#dw_tag_subroutine)
- [`DW_TAG_subroutine_type`](#dw_tag_subroutine_type)
- [`DW_TAG_template_alias`](#dw_tag_template_alias)
- [`DW_TAG_template_type_parameter`](#dw_tag_template_type_parameter)
- [`DW_TAG_template_value_parameter`](#dw_tag_template_value_parameter)
- [`DW_TAG_thrown_type`](#dw_tag_thrown_type)
- [`DW_TAG_try_block`](#dw_tag_try_block)
- [`DW_TAG_type_unit`](#dw_tag_type_unit)
- [`DW_TAG_typedef`](#dw_tag_typedef)
- [`DW_TAG_union_type`](#dw_tag_union_type)
- [`DW_TAG_unspecified_parameters`](#dw_tag_unspecified_parameters)
- [`DW_TAG_unspecified_type`](#dw_tag_unspecified_type)
- [`DW_TAG_upc_relaxed_type`](#dw_tag_upc_relaxed_type)
- [`DW_TAG_upc_shared_type`](#dw_tag_upc_shared_type)
- [`DW_TAG_upc_strict_type`](#dw_tag_upc_strict_type)
- [`DW_TAG_variable`](#dw_tag_variable)
- [`DW_TAG_variant`](#dw_tag_variant)
- [`DW_TAG_variant_part`](#dw_tag_variant_part)
- [`DW_TAG_volatile_type`](#dw_tag_volatile_type)
- [`DW_TAG_with_stmt`](#dw_tag_with_stmt)
- [`DW_UT_compile`](#dw_ut_compile)
- [`DW_UT_hi_user`](#dw_ut_hi_user)
- [`DW_UT_lo_user`](#dw_ut_lo_user)
- [`DW_UT_partial`](#dw_ut_partial)
- [`DW_UT_skeleton`](#dw_ut_skeleton)
- [`DW_UT_split_compile`](#dw_ut_split_compile)
- [`DW_UT_split_type`](#dw_ut_split_type)
- [`DW_UT_type`](#dw_ut_type)
- [`DW_VIRTUALITY_none`](#dw_virtuality_none)
- [`DW_VIRTUALITY_pure_virtual`](#dw_virtuality_pure_virtual)
- [`DW_VIRTUALITY_virtual`](#dw_virtuality_virtual)
- [`DW_VIS_exported`](#dw_vis_exported)
- [`DW_VIS_local`](#dw_vis_local)
- [`DW_VIS_qualified`](#dw_vis_qualified)

---

## gimli::constants::DW_ACCESS_private

*Constant*: `DwAccess`



## gimli::constants::DW_ACCESS_protected

*Constant*: `DwAccess`



## gimli::constants::DW_ACCESS_public

*Constant*: `DwAccess`



## gimli::constants::DW_ADDR_none

*Constant*: `DwAddr`



## gimli::constants::DW_ATE_ASCII

*Constant*: `DwAte`



## gimli::constants::DW_ATE_UCS

*Constant*: `DwAte`



## gimli::constants::DW_ATE_UTF

*Constant*: `DwAte`



## gimli::constants::DW_ATE_address

*Constant*: `DwAte`



## gimli::constants::DW_ATE_boolean

*Constant*: `DwAte`



## gimli::constants::DW_ATE_complex_float

*Constant*: `DwAte`



## gimli::constants::DW_ATE_decimal_float

*Constant*: `DwAte`



## gimli::constants::DW_ATE_edited

*Constant*: `DwAte`



## gimli::constants::DW_ATE_float

*Constant*: `DwAte`



## gimli::constants::DW_ATE_hi_user

*Constant*: `DwAte`



## gimli::constants::DW_ATE_imaginary_float

*Constant*: `DwAte`



## gimli::constants::DW_ATE_lo_user

*Constant*: `DwAte`



## gimli::constants::DW_ATE_numeric_string

*Constant*: `DwAte`



## gimli::constants::DW_ATE_packed_decimal

*Constant*: `DwAte`



## gimli::constants::DW_ATE_signed

*Constant*: `DwAte`



## gimli::constants::DW_ATE_signed_char

*Constant*: `DwAte`



## gimli::constants::DW_ATE_signed_fixed

*Constant*: `DwAte`



## gimli::constants::DW_ATE_unsigned

*Constant*: `DwAte`



## gimli::constants::DW_ATE_unsigned_char

*Constant*: `DwAte`



## gimli::constants::DW_ATE_unsigned_fixed

*Constant*: `DwAte`



## gimli::constants::DW_AT_ALTIUM_loclist

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_block

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_flags

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_isa

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_major_runtime_vers

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_objc_complete_type

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_omit_frame_ptr

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_optimized

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_property

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_property_attribute

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_property_getter

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_property_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_property_setter

*Constant*: `DwAt`



## gimli::constants::DW_AT_APPLE_runtime_class

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_ABI

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_anonymous_method

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_class

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_constructor

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_destructor

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_frameptr

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_interface

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_metaclass

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_record

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_return

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_Delphi_unit

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_closure

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_property_default

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_property_implements

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_property_index

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_property_read

*Constant*: `DwAt`



## gimli::constants::DW_AT_BORLAND_property_write

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNAT_descriptive_type

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_addr_base

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_all_call_sites

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_all_source_call_sites

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_all_tail_call_sites

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_bias

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_call_site_data_value

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_call_site_target

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_call_site_target_clobbered

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_call_site_value

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_deleted

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_denominator

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_discriminator

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_dwo_id

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_dwo_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_entry_view

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_exclusive_locks_required

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_guarded

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_guarded_by

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_locks_excluded

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_locviews

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_macros

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_numerator

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_odr_signature

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_pt_guarded

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_pt_guarded_by

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_pubnames

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_pubtypes

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_ranges_base

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_shared_locks_required

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_tail_call

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_template_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_GNU_vector

*Constant*: `DwAt`



## gimli::constants::DW_AT_INTEL_other_endian

*Constant*: `DwAt`



## gimli::constants::DW_AT_LLVM_config_macros

*Constant*: `DwAt`



## gimli::constants::DW_AT_LLVM_include_path

*Constant*: `DwAt`



## gimli::constants::DW_AT_LLVM_isysroot

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_abstract_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_allocatable_dopetype

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_assumed_shape_dopetype

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_assumed_size

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_clone_origin

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_epilog_begin

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_fde

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_has_inlines

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_linkage_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_loop_begin

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_loop_unroll_factor

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_ptr_dopetype

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_software_pipeline_depth

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_stride

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_stride_byte

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_stride_elem

*Constant*: `DwAt`



## gimli::constants::DW_AT_MIPS_tail_loop_begin

*Constant*: `DwAt`



## gimli::constants::DW_AT_PGI_lbase

*Constant*: `DwAt`



## gimli::constants::DW_AT_PGI_lstride

*Constant*: `DwAt`



## gimli::constants::DW_AT_PGI_soffset

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_alignment

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_amd64_parmdump

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_browser_file

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_c_vla

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_cf_kind

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_command_line

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_compile_options

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_count_guarantee

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_dtor_length

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_dtor_start

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_dtor_state_deltas

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_dtor_state_final

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_dtor_state_initial

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_f90_allocatable

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_f90_assumed_shape_array

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_f90_pointer

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_f90_use_only

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_fortran_based

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_fortran_main_alias

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_func_offset

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_func_offsets

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_hwcprof_signature

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_import_by_lname

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_import_by_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_is_omp_child_func

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_language

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_link_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_memop_signature

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_memop_type_ref

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_namelist_spec

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_obj_dir

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_obj_file

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_omp_child_func

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_omp_tpriv_addr

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_original_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_part_link_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_pass_by_ref

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_pass_with_const

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_profile_id

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_return_value_ptr

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_return_with_const

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_template

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_vbase

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_vtable

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_vtable_abi

*Constant*: `DwAt`



## gimli::constants::DW_AT_SUN_vtable_index

*Constant*: `DwAt`



## gimli::constants::DW_AT_abstract_origin

*Constant*: `DwAt`



## gimli::constants::DW_AT_accessibility

*Constant*: `DwAt`



## gimli::constants::DW_AT_addr_base

*Constant*: `DwAt`



## gimli::constants::DW_AT_address_class

*Constant*: `DwAt`



## gimli::constants::DW_AT_alignment

*Constant*: `DwAt`



## gimli::constants::DW_AT_allocated

*Constant*: `DwAt`



## gimli::constants::DW_AT_artificial

*Constant*: `DwAt`



## gimli::constants::DW_AT_associated

*Constant*: `DwAt`



## gimli::constants::DW_AT_base_types

*Constant*: `DwAt`



## gimli::constants::DW_AT_binary_scale

*Constant*: `DwAt`



## gimli::constants::DW_AT_bit_offset

*Constant*: `DwAt`



## gimli::constants::DW_AT_bit_size

*Constant*: `DwAt`



## gimli::constants::DW_AT_bit_stride

*Constant*: `DwAt`



## gimli::constants::DW_AT_body_begin

*Constant*: `DwAt`



## gimli::constants::DW_AT_body_end

*Constant*: `DwAt`



## gimli::constants::DW_AT_byte_size

*Constant*: `DwAt`



## gimli::constants::DW_AT_byte_stride

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_all_calls

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_all_source_calls

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_all_tail_calls

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_column

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_data_location

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_data_value

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_file

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_line

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_origin

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_parameter

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_pc

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_return_pc

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_tail_call

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_target

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_target_clobbered

*Constant*: `DwAt`



## gimli::constants::DW_AT_call_value

*Constant*: `DwAt`



## gimli::constants::DW_AT_calling_convention

*Constant*: `DwAt`



## gimli::constants::DW_AT_common_reference

*Constant*: `DwAt`



## gimli::constants::DW_AT_comp_dir

*Constant*: `DwAt`



## gimli::constants::DW_AT_const_expr

*Constant*: `DwAt`



## gimli::constants::DW_AT_const_value

*Constant*: `DwAt`



## gimli::constants::DW_AT_containing_type

*Constant*: `DwAt`



## gimli::constants::DW_AT_count

*Constant*: `DwAt`



## gimli::constants::DW_AT_data_bit_offset

*Constant*: `DwAt`



## gimli::constants::DW_AT_data_location

*Constant*: `DwAt`



## gimli::constants::DW_AT_data_member_location

*Constant*: `DwAt`



## gimli::constants::DW_AT_decimal_scale

*Constant*: `DwAt`



## gimli::constants::DW_AT_decimal_sign

*Constant*: `DwAt`



## gimli::constants::DW_AT_decl_column

*Constant*: `DwAt`



## gimli::constants::DW_AT_decl_file

*Constant*: `DwAt`



## gimli::constants::DW_AT_decl_line

*Constant*: `DwAt`



## gimli::constants::DW_AT_declaration

*Constant*: `DwAt`



## gimli::constants::DW_AT_default_value

*Constant*: `DwAt`



## gimli::constants::DW_AT_defaulted

*Constant*: `DwAt`



## gimli::constants::DW_AT_deleted

*Constant*: `DwAt`



## gimli::constants::DW_AT_description

*Constant*: `DwAt`



## gimli::constants::DW_AT_digit_count

*Constant*: `DwAt`



## gimli::constants::DW_AT_discr

*Constant*: `DwAt`



## gimli::constants::DW_AT_discr_list

*Constant*: `DwAt`



## gimli::constants::DW_AT_discr_value

*Constant*: `DwAt`



## gimli::constants::DW_AT_dwo_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_element_list

*Constant*: `DwAt`



## gimli::constants::DW_AT_elemental

*Constant*: `DwAt`



## gimli::constants::DW_AT_encoding

*Constant*: `DwAt`



## gimli::constants::DW_AT_endianity

*Constant*: `DwAt`



## gimli::constants::DW_AT_entry_pc

*Constant*: `DwAt`



## gimli::constants::DW_AT_enum_class

*Constant*: `DwAt`



## gimli::constants::DW_AT_explicit

*Constant*: `DwAt`



## gimli::constants::DW_AT_export_symbols

*Constant*: `DwAt`



## gimli::constants::DW_AT_extension

*Constant*: `DwAt`



## gimli::constants::DW_AT_external

*Constant*: `DwAt`



## gimli::constants::DW_AT_frame_base

*Constant*: `DwAt`



## gimli::constants::DW_AT_friend

*Constant*: `DwAt`



## gimli::constants::DW_AT_friends

*Constant*: `DwAt`



## gimli::constants::DW_AT_fund_type

*Constant*: `DwAt`



## gimli::constants::DW_AT_hi_user

*Constant*: `DwAt`



## gimli::constants::DW_AT_high_pc

*Constant*: `DwAt`



## gimli::constants::DW_AT_identifier_case

*Constant*: `DwAt`



## gimli::constants::DW_AT_import

*Constant*: `DwAt`



## gimli::constants::DW_AT_inline

*Constant*: `DwAt`



## gimli::constants::DW_AT_is_optional

*Constant*: `DwAt`



## gimli::constants::DW_AT_language

*Constant*: `DwAt`



## gimli::constants::DW_AT_linkage_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_lo_user

*Constant*: `DwAt`



## gimli::constants::DW_AT_location

*Constant*: `DwAt`



## gimli::constants::DW_AT_loclists_base

*Constant*: `DwAt`



## gimli::constants::DW_AT_low_pc

*Constant*: `DwAt`



## gimli::constants::DW_AT_lower_bound

*Constant*: `DwAt`



## gimli::constants::DW_AT_mac_info

*Constant*: `DwAt`



## gimli::constants::DW_AT_macro_info

*Constant*: `DwAt`



## gimli::constants::DW_AT_macros

*Constant*: `DwAt`



## gimli::constants::DW_AT_main_subprogram

*Constant*: `DwAt`



## gimli::constants::DW_AT_member

*Constant*: `DwAt`



## gimli::constants::DW_AT_mod_fund_type

*Constant*: `DwAt`



## gimli::constants::DW_AT_mod_u_d_type

*Constant*: `DwAt`



## gimli::constants::DW_AT_mutable

*Constant*: `DwAt`



## gimli::constants::DW_AT_name

*Constant*: `DwAt`



## gimli::constants::DW_AT_namelist_item

*Constant*: `DwAt`



## gimli::constants::DW_AT_noreturn

*Constant*: `DwAt`



## gimli::constants::DW_AT_null

*Constant*: `DwAt`



## gimli::constants::DW_AT_object_pointer

*Constant*: `DwAt`



## gimli::constants::DW_AT_ordering

*Constant*: `DwAt`



## gimli::constants::DW_AT_picture_string

*Constant*: `DwAt`



## gimli::constants::DW_AT_priority

*Constant*: `DwAt`



## gimli::constants::DW_AT_private

*Constant*: `DwAt`



## gimli::constants::DW_AT_producer

*Constant*: `DwAt`



## gimli::constants::DW_AT_program

*Constant*: `DwAt`



## gimli::constants::DW_AT_protected

*Constant*: `DwAt`



## gimli::constants::DW_AT_prototyped

*Constant*: `DwAt`



## gimli::constants::DW_AT_public

*Constant*: `DwAt`



## gimli::constants::DW_AT_pure

*Constant*: `DwAt`



## gimli::constants::DW_AT_pure_virtual

*Constant*: `DwAt`



## gimli::constants::DW_AT_ranges

*Constant*: `DwAt`



## gimli::constants::DW_AT_rank

*Constant*: `DwAt`



## gimli::constants::DW_AT_recursive

*Constant*: `DwAt`



## gimli::constants::DW_AT_reference

*Constant*: `DwAt`



## gimli::constants::DW_AT_return_addr

*Constant*: `DwAt`



## gimli::constants::DW_AT_rnglists_base

*Constant*: `DwAt`



## gimli::constants::DW_AT_rvalue_reference

*Constant*: `DwAt`



## gimli::constants::DW_AT_segment

*Constant*: `DwAt`



## gimli::constants::DW_AT_sf_names

*Constant*: `DwAt`



## gimli::constants::DW_AT_sibling

*Constant*: `DwAt`



## gimli::constants::DW_AT_signature

*Constant*: `DwAt`



## gimli::constants::DW_AT_small

*Constant*: `DwAt`



## gimli::constants::DW_AT_specification

*Constant*: `DwAt`



## gimli::constants::DW_AT_specification_v1

*Constant*: `DwAt`



## gimli::constants::DW_AT_src_coords

*Constant*: `DwAt`



## gimli::constants::DW_AT_src_info

*Constant*: `DwAt`



## gimli::constants::DW_AT_start_scope

*Constant*: `DwAt`



## gimli::constants::DW_AT_static_link

*Constant*: `DwAt`



## gimli::constants::DW_AT_stmt_list

*Constant*: `DwAt`



## gimli::constants::DW_AT_str_offsets_base

*Constant*: `DwAt`



## gimli::constants::DW_AT_string_length

*Constant*: `DwAt`



## gimli::constants::DW_AT_string_length_bit_size

*Constant*: `DwAt`



## gimli::constants::DW_AT_string_length_byte_size

*Constant*: `DwAt`



## gimli::constants::DW_AT_subscr_data

*Constant*: `DwAt`



## gimli::constants::DW_AT_threads_scaled

*Constant*: `DwAt`



## gimli::constants::DW_AT_trampoline

*Constant*: `DwAt`



## gimli::constants::DW_AT_type

*Constant*: `DwAt`



## gimli::constants::DW_AT_upc_threads_scaled

*Constant*: `DwAt`



## gimli::constants::DW_AT_upper_bound

*Constant*: `DwAt`



## gimli::constants::DW_AT_use_GNAT_descriptive_type

*Constant*: `DwAt`



## gimli::constants::DW_AT_use_UTF8

*Constant*: `DwAt`



## gimli::constants::DW_AT_use_location

*Constant*: `DwAt`



## gimli::constants::DW_AT_user_def_type

*Constant*: `DwAt`



## gimli::constants::DW_AT_variable_parameter

*Constant*: `DwAt`



## gimli::constants::DW_AT_virtual

*Constant*: `DwAt`



## gimli::constants::DW_AT_virtuality

*Constant*: `DwAt`



## gimli::constants::DW_AT_visibility

*Constant*: `DwAt`



## gimli::constants::DW_AT_vtable_elem_location

*Constant*: `DwAt`



## gimli::constants::DW_CC_hi_user

*Constant*: `DwCc`



## gimli::constants::DW_CC_lo_user

*Constant*: `DwCc`



## gimli::constants::DW_CC_nocall

*Constant*: `DwCc`



## gimli::constants::DW_CC_normal

*Constant*: `DwCc`



## gimli::constants::DW_CC_pass_by_reference

*Constant*: `DwCc`



## gimli::constants::DW_CC_pass_by_value

*Constant*: `DwCc`



## gimli::constants::DW_CC_program

*Constant*: `DwCc`



## gimli::constants::DW_CFA_AARCH64_negate_ra_state

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_GNU_args_size

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_GNU_negative_offset_extended

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_GNU_window_save

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_MIPS_advance_loc8

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_advance_loc

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_advance_loc1

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_advance_loc2

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_advance_loc4

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_def_cfa

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_def_cfa_expression

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_def_cfa_offset

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_def_cfa_offset_sf

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_def_cfa_register

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_def_cfa_sf

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_expression

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_hi_user

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_lo_user

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_nop

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_offset

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_offset_extended

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_offset_extended_sf

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_register

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_remember_state

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_restore

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_restore_extended

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_restore_state

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_same_value

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_set_loc

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_undefined

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_val_expression

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_val_offset

*Constant*: `DwCfa`



## gimli::constants::DW_CFA_val_offset_sf

*Constant*: `DwCfa`



## gimli::constants::DW_CHILDREN_no

*Constant*: `DwChildren`



## gimli::constants::DW_CHILDREN_yes

*Constant*: `DwChildren`



## gimli::constants::DW_DEFAULTED_in_class

*Constant*: `DwDefaulted`



## gimli::constants::DW_DEFAULTED_no

*Constant*: `DwDefaulted`



## gimli::constants::DW_DEFAULTED_out_of_class

*Constant*: `DwDefaulted`



## gimli::constants::DW_DSC_label

*Constant*: `DwDsc`



## gimli::constants::DW_DSC_range

*Constant*: `DwDsc`



## gimli::constants::DW_DS_leading_overpunch

*Constant*: `DwDs`



## gimli::constants::DW_DS_leading_separate

*Constant*: `DwDs`



## gimli::constants::DW_DS_trailing_overpunch

*Constant*: `DwDs`



## gimli::constants::DW_DS_trailing_separate

*Constant*: `DwDs`



## gimli::constants::DW_DS_unsigned

*Constant*: `DwDs`



## gimli::constants::DW_EH_PE_absptr

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_aligned

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_datarel

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_funcrel

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_indirect

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_omit

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_pcrel

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_sdata2

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_sdata4

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_sdata8

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_sleb128

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_textrel

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_udata2

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_udata4

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_udata8

*Constant*: `DwEhPe`



## gimli::constants::DW_EH_PE_uleb128

*Constant*: `DwEhPe`



## gimli::constants::DW_END_big

*Constant*: `DwEnd`



## gimli::constants::DW_END_default

*Constant*: `DwEnd`



## gimli::constants::DW_END_hi_user

*Constant*: `DwEnd`



## gimli::constants::DW_END_little

*Constant*: `DwEnd`



## gimli::constants::DW_END_lo_user

*Constant*: `DwEnd`



## gimli::constants::DW_FORM_GNU_addr_index

*Constant*: `DwForm`



## gimli::constants::DW_FORM_GNU_ref_alt

*Constant*: `DwForm`



## gimli::constants::DW_FORM_GNU_str_index

*Constant*: `DwForm`



## gimli::constants::DW_FORM_GNU_strp_alt

*Constant*: `DwForm`



## gimli::constants::DW_FORM_addr

*Constant*: `DwForm`



## gimli::constants::DW_FORM_addrx

*Constant*: `DwForm`



## gimli::constants::DW_FORM_addrx1

*Constant*: `DwForm`



## gimli::constants::DW_FORM_addrx2

*Constant*: `DwForm`



## gimli::constants::DW_FORM_addrx3

*Constant*: `DwForm`



## gimli::constants::DW_FORM_addrx4

*Constant*: `DwForm`



## gimli::constants::DW_FORM_block

*Constant*: `DwForm`



## gimli::constants::DW_FORM_block1

*Constant*: `DwForm`



## gimli::constants::DW_FORM_block2

*Constant*: `DwForm`



## gimli::constants::DW_FORM_block4

*Constant*: `DwForm`



## gimli::constants::DW_FORM_data1

*Constant*: `DwForm`



## gimli::constants::DW_FORM_data16

*Constant*: `DwForm`



## gimli::constants::DW_FORM_data2

*Constant*: `DwForm`



## gimli::constants::DW_FORM_data4

*Constant*: `DwForm`



## gimli::constants::DW_FORM_data8

*Constant*: `DwForm`



## gimli::constants::DW_FORM_exprloc

*Constant*: `DwForm`



## gimli::constants::DW_FORM_flag

*Constant*: `DwForm`



## gimli::constants::DW_FORM_flag_present

*Constant*: `DwForm`



## gimli::constants::DW_FORM_implicit_const

*Constant*: `DwForm`



## gimli::constants::DW_FORM_indirect

*Constant*: `DwForm`



## gimli::constants::DW_FORM_line_strp

*Constant*: `DwForm`



## gimli::constants::DW_FORM_loclistx

*Constant*: `DwForm`



## gimli::constants::DW_FORM_null

*Constant*: `DwForm`



## gimli::constants::DW_FORM_ref

*Constant*: `DwForm`



## gimli::constants::DW_FORM_ref1

*Constant*: `DwForm`



## gimli::constants::DW_FORM_ref2

*Constant*: `DwForm`



## gimli::constants::DW_FORM_ref4

*Constant*: `DwForm`



## gimli::constants::DW_FORM_ref8

*Constant*: `DwForm`



## gimli::constants::DW_FORM_ref_addr

*Constant*: `DwForm`



## gimli::constants::DW_FORM_ref_sig8

*Constant*: `DwForm`



## gimli::constants::DW_FORM_ref_sup4

*Constant*: `DwForm`



## gimli::constants::DW_FORM_ref_sup8

*Constant*: `DwForm`



## gimli::constants::DW_FORM_ref_udata

*Constant*: `DwForm`



## gimli::constants::DW_FORM_rnglistx

*Constant*: `DwForm`



## gimli::constants::DW_FORM_sdata

*Constant*: `DwForm`



## gimli::constants::DW_FORM_sec_offset

*Constant*: `DwForm`



## gimli::constants::DW_FORM_string

*Constant*: `DwForm`



## gimli::constants::DW_FORM_strp

*Constant*: `DwForm`



## gimli::constants::DW_FORM_strp_sup

*Constant*: `DwForm`



## gimli::constants::DW_FORM_strx

*Constant*: `DwForm`



## gimli::constants::DW_FORM_strx1

*Constant*: `DwForm`



## gimli::constants::DW_FORM_strx2

*Constant*: `DwForm`



## gimli::constants::DW_FORM_strx3

*Constant*: `DwForm`



## gimli::constants::DW_FORM_strx4

*Constant*: `DwForm`



## gimli::constants::DW_FORM_udata

*Constant*: `DwForm`



## gimli::constants::DW_IDX_compile_unit

*Constant*: `DwIdx`



## gimli::constants::DW_IDX_die_offset

*Constant*: `DwIdx`



## gimli::constants::DW_IDX_hi_user

*Constant*: `DwIdx`



## gimli::constants::DW_IDX_lo_user

*Constant*: `DwIdx`



## gimli::constants::DW_IDX_parent

*Constant*: `DwIdx`



## gimli::constants::DW_IDX_type_hash

*Constant*: `DwIdx`



## gimli::constants::DW_IDX_type_unit

*Constant*: `DwIdx`



## gimli::constants::DW_ID_case_insensitive

*Constant*: `DwId`



## gimli::constants::DW_ID_case_sensitive

*Constant*: `DwId`



## gimli::constants::DW_ID_down_case

*Constant*: `DwId`



## gimli::constants::DW_ID_up_case

*Constant*: `DwId`



## gimli::constants::DW_INL_declared_inlined

*Constant*: `DwInl`



## gimli::constants::DW_INL_declared_not_inlined

*Constant*: `DwInl`



## gimli::constants::DW_INL_inlined

*Constant*: `DwInl`



## gimli::constants::DW_INL_not_inlined

*Constant*: `DwInl`



## gimli::constants::DW_LANG_ALTIUM_Assembler

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Ada2005

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Ada2012

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Ada83

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Ada95

*Constant*: `DwLang`



## gimli::constants::DW_LANG_BLISS

*Constant*: `DwLang`



## gimli::constants::DW_LANG_BORLAND_Delphi

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C11

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C17

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C89

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C99

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C_plus_plus

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C_plus_plus_03

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C_plus_plus_11

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C_plus_plus_14

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C_plus_plus_17

*Constant*: `DwLang`



## gimli::constants::DW_LANG_C_plus_plus_20

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Cobol74

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Cobol85

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Crystal

*Constant*: `DwLang`



## gimli::constants::DW_LANG_D

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Dylan

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Fortran03

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Fortran08

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Fortran18

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Fortran77

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Fortran90

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Fortran95

*Constant*: `DwLang`



## gimli::constants::DW_LANG_GOOGLE_RenderScript

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Go

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Haskell

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Java

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Julia

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Kotlin

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Mips_Assembler

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Modula2

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Modula3

*Constant*: `DwLang`



## gimli::constants::DW_LANG_OCaml

*Constant*: `DwLang`



## gimli::constants::DW_LANG_ObjC

*Constant*: `DwLang`



## gimli::constants::DW_LANG_ObjC_plus_plus

*Constant*: `DwLang`



## gimli::constants::DW_LANG_OpenCL

*Constant*: `DwLang`



## gimli::constants::DW_LANG_PLI

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Pascal83

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Python

*Constant*: `DwLang`



## gimli::constants::DW_LANG_RenderScript

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Rust

*Constant*: `DwLang`



## gimli::constants::DW_LANG_SUN_Assembler

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Swift

*Constant*: `DwLang`



## gimli::constants::DW_LANG_UPC

*Constant*: `DwLang`



## gimli::constants::DW_LANG_Zig

*Constant*: `DwLang`



## gimli::constants::DW_LANG_hi_user

*Constant*: `DwLang`



## gimli::constants::DW_LANG_lo_user

*Constant*: `DwLang`



## gimli::constants::DW_LLE_GNU_view_pair

*Constant*: `DwLle`



## gimli::constants::DW_LLE_base_address

*Constant*: `DwLle`



## gimli::constants::DW_LLE_base_addressx

*Constant*: `DwLle`



## gimli::constants::DW_LLE_default_location

*Constant*: `DwLle`



## gimli::constants::DW_LLE_end_of_list

*Constant*: `DwLle`



## gimli::constants::DW_LLE_offset_pair

*Constant*: `DwLle`



## gimli::constants::DW_LLE_start_end

*Constant*: `DwLle`



## gimli::constants::DW_LLE_start_length

*Constant*: `DwLle`



## gimli::constants::DW_LLE_startx_endx

*Constant*: `DwLle`



## gimli::constants::DW_LLE_startx_length

*Constant*: `DwLle`



## gimli::constants::DW_LNCT_LLVM_source

*Constant*: `DwLnct`



## gimli::constants::DW_LNCT_MD5

*Constant*: `DwLnct`



## gimli::constants::DW_LNCT_directory_index

*Constant*: `DwLnct`



## gimli::constants::DW_LNCT_hi_user

*Constant*: `DwLnct`



## gimli::constants::DW_LNCT_lo_user

*Constant*: `DwLnct`



## gimli::constants::DW_LNCT_path

*Constant*: `DwLnct`



## gimli::constants::DW_LNCT_size

*Constant*: `DwLnct`



## gimli::constants::DW_LNCT_timestamp

*Constant*: `DwLnct`



## gimli::constants::DW_LNE_define_file

*Constant*: `DwLne`



## gimli::constants::DW_LNE_end_sequence

*Constant*: `DwLne`



## gimli::constants::DW_LNE_hi_user

*Constant*: `DwLne`



## gimli::constants::DW_LNE_lo_user

*Constant*: `DwLne`



## gimli::constants::DW_LNE_set_address

*Constant*: `DwLne`



## gimli::constants::DW_LNE_set_discriminator

*Constant*: `DwLne`



## gimli::constants::DW_LNS_advance_line

*Constant*: `DwLns`



## gimli::constants::DW_LNS_advance_pc

*Constant*: `DwLns`



## gimli::constants::DW_LNS_const_add_pc

*Constant*: `DwLns`



## gimli::constants::DW_LNS_copy

*Constant*: `DwLns`



## gimli::constants::DW_LNS_fixed_advance_pc

*Constant*: `DwLns`



## gimli::constants::DW_LNS_negate_stmt

*Constant*: `DwLns`



## gimli::constants::DW_LNS_set_basic_block

*Constant*: `DwLns`



## gimli::constants::DW_LNS_set_column

*Constant*: `DwLns`



## gimli::constants::DW_LNS_set_epilogue_begin

*Constant*: `DwLns`



## gimli::constants::DW_LNS_set_file

*Constant*: `DwLns`



## gimli::constants::DW_LNS_set_isa

*Constant*: `DwLns`



## gimli::constants::DW_LNS_set_prologue_end

*Constant*: `DwLns`



## gimli::constants::DW_MACINFO_define

*Constant*: `DwMacinfo`



## gimli::constants::DW_MACINFO_end_file

*Constant*: `DwMacinfo`



## gimli::constants::DW_MACINFO_start_file

*Constant*: `DwMacinfo`



## gimli::constants::DW_MACINFO_undef

*Constant*: `DwMacinfo`



## gimli::constants::DW_MACINFO_vendor_ext

*Constant*: `DwMacinfo`



## gimli::constants::DW_MACRO_define

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_define_strp

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_define_strx

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_define_sup

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_end_file

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_hi_user

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_import

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_import_sup

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_lo_user

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_start_file

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_undef

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_undef_strp

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_undef_strx

*Constant*: `DwMacro`



## gimli::constants::DW_MACRO_undef_sup

*Constant*: `DwMacro`



## gimli::constants::DW_OP_GNU_addr_index

*Constant*: `DwOp`



## gimli::constants::DW_OP_GNU_const_index

*Constant*: `DwOp`



## gimli::constants::DW_OP_GNU_const_type

*Constant*: `DwOp`



## gimli::constants::DW_OP_GNU_convert

*Constant*: `DwOp`



## gimli::constants::DW_OP_GNU_deref_type

*Constant*: `DwOp`



## gimli::constants::DW_OP_GNU_entry_value

*Constant*: `DwOp`



## gimli::constants::DW_OP_GNU_implicit_pointer

*Constant*: `DwOp`



## gimli::constants::DW_OP_GNU_parameter_ref

*Constant*: `DwOp`



## gimli::constants::DW_OP_GNU_push_tls_address

*Constant*: `DwOp`



## gimli::constants::DW_OP_GNU_regval_type

*Constant*: `DwOp`



## gimli::constants::DW_OP_GNU_reinterpret

*Constant*: `DwOp`



## gimli::constants::DW_OP_WASM_location

*Constant*: `DwOp`



## gimli::constants::DW_OP_abs

*Constant*: `DwOp`



## gimli::constants::DW_OP_addr

*Constant*: `DwOp`



## gimli::constants::DW_OP_addrx

*Constant*: `DwOp`



## gimli::constants::DW_OP_and

*Constant*: `DwOp`



## gimli::constants::DW_OP_bit_piece

*Constant*: `DwOp`



## gimli::constants::DW_OP_bra

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg0

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg1

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg10

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg11

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg12

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg13

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg14

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg15

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg16

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg17

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg18

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg19

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg2

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg20

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg21

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg22

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg23

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg24

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg25

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg26

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg27

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg28

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg29

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg3

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg30

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg31

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg4

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg5

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg6

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg7

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg8

*Constant*: `DwOp`



## gimli::constants::DW_OP_breg9

*Constant*: `DwOp`



## gimli::constants::DW_OP_bregx

*Constant*: `DwOp`



## gimli::constants::DW_OP_call2

*Constant*: `DwOp`



## gimli::constants::DW_OP_call4

*Constant*: `DwOp`



## gimli::constants::DW_OP_call_frame_cfa

*Constant*: `DwOp`



## gimli::constants::DW_OP_call_ref

*Constant*: `DwOp`



## gimli::constants::DW_OP_const1s

*Constant*: `DwOp`



## gimli::constants::DW_OP_const1u

*Constant*: `DwOp`



## gimli::constants::DW_OP_const2s

*Constant*: `DwOp`



## gimli::constants::DW_OP_const2u

*Constant*: `DwOp`



## gimli::constants::DW_OP_const4s

*Constant*: `DwOp`



## gimli::constants::DW_OP_const4u

*Constant*: `DwOp`



## gimli::constants::DW_OP_const8s

*Constant*: `DwOp`



## gimli::constants::DW_OP_const8u

*Constant*: `DwOp`



## gimli::constants::DW_OP_const_type

*Constant*: `DwOp`



## gimli::constants::DW_OP_consts

*Constant*: `DwOp`



## gimli::constants::DW_OP_constu

*Constant*: `DwOp`



## gimli::constants::DW_OP_constx

*Constant*: `DwOp`



## gimli::constants::DW_OP_convert

*Constant*: `DwOp`



## gimli::constants::DW_OP_deref

*Constant*: `DwOp`



## gimli::constants::DW_OP_deref_size

*Constant*: `DwOp`



## gimli::constants::DW_OP_deref_type

*Constant*: `DwOp`



## gimli::constants::DW_OP_div

*Constant*: `DwOp`



## gimli::constants::DW_OP_drop

*Constant*: `DwOp`



## gimli::constants::DW_OP_dup

*Constant*: `DwOp`



## gimli::constants::DW_OP_entry_value

*Constant*: `DwOp`



## gimli::constants::DW_OP_eq

*Constant*: `DwOp`



## gimli::constants::DW_OP_fbreg

*Constant*: `DwOp`



## gimli::constants::DW_OP_form_tls_address

*Constant*: `DwOp`



## gimli::constants::DW_OP_ge

*Constant*: `DwOp`



## gimli::constants::DW_OP_gt

*Constant*: `DwOp`



## gimli::constants::DW_OP_implicit_pointer

*Constant*: `DwOp`



## gimli::constants::DW_OP_implicit_value

*Constant*: `DwOp`



## gimli::constants::DW_OP_le

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit0

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit1

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit10

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit11

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit12

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit13

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit14

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit15

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit16

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit17

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit18

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit19

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit2

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit20

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit21

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit22

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit23

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit24

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit25

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit26

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit27

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit28

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit29

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit3

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit30

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit31

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit4

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit5

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit6

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit7

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit8

*Constant*: `DwOp`



## gimli::constants::DW_OP_lit9

*Constant*: `DwOp`



## gimli::constants::DW_OP_lt

*Constant*: `DwOp`



## gimli::constants::DW_OP_minus

*Constant*: `DwOp`



## gimli::constants::DW_OP_mod

*Constant*: `DwOp`



## gimli::constants::DW_OP_mul

*Constant*: `DwOp`



## gimli::constants::DW_OP_ne

*Constant*: `DwOp`



## gimli::constants::DW_OP_neg

*Constant*: `DwOp`



## gimli::constants::DW_OP_nop

*Constant*: `DwOp`



## gimli::constants::DW_OP_not

*Constant*: `DwOp`



## gimli::constants::DW_OP_or

*Constant*: `DwOp`



## gimli::constants::DW_OP_over

*Constant*: `DwOp`



## gimli::constants::DW_OP_pick

*Constant*: `DwOp`



## gimli::constants::DW_OP_piece

*Constant*: `DwOp`



## gimli::constants::DW_OP_plus

*Constant*: `DwOp`



## gimli::constants::DW_OP_plus_uconst

*Constant*: `DwOp`



## gimli::constants::DW_OP_push_object_address

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg0

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg1

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg10

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg11

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg12

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg13

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg14

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg15

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg16

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg17

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg18

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg19

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg2

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg20

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg21

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg22

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg23

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg24

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg25

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg26

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg27

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg28

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg29

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg3

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg30

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg31

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg4

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg5

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg6

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg7

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg8

*Constant*: `DwOp`



## gimli::constants::DW_OP_reg9

*Constant*: `DwOp`



## gimli::constants::DW_OP_regval_type

*Constant*: `DwOp`



## gimli::constants::DW_OP_regx

*Constant*: `DwOp`



## gimli::constants::DW_OP_reinterpret

*Constant*: `DwOp`



## gimli::constants::DW_OP_rot

*Constant*: `DwOp`



## gimli::constants::DW_OP_shl

*Constant*: `DwOp`



## gimli::constants::DW_OP_shr

*Constant*: `DwOp`



## gimli::constants::DW_OP_shra

*Constant*: `DwOp`



## gimli::constants::DW_OP_skip

*Constant*: `DwOp`



## gimli::constants::DW_OP_stack_value

*Constant*: `DwOp`



## gimli::constants::DW_OP_swap

*Constant*: `DwOp`



## gimli::constants::DW_OP_xderef

*Constant*: `DwOp`



## gimli::constants::DW_OP_xderef_size

*Constant*: `DwOp`



## gimli::constants::DW_OP_xderef_type

*Constant*: `DwOp`



## gimli::constants::DW_OP_xor

*Constant*: `DwOp`



## gimli::constants::DW_ORD_col_major

*Constant*: `DwOrd`



## gimli::constants::DW_ORD_row_major

*Constant*: `DwOrd`



## gimli::constants::DW_RLE_base_address

*Constant*: `DwRle`



## gimli::constants::DW_RLE_base_addressx

*Constant*: `DwRle`



## gimli::constants::DW_RLE_end_of_list

*Constant*: `DwRle`



## gimli::constants::DW_RLE_offset_pair

*Constant*: `DwRle`



## gimli::constants::DW_RLE_start_end

*Constant*: `DwRle`



## gimli::constants::DW_RLE_start_length

*Constant*: `DwRle`



## gimli::constants::DW_RLE_startx_endx

*Constant*: `DwRle`



## gimli::constants::DW_RLE_startx_length

*Constant*: `DwRle`



## gimli::constants::DW_SECT_ABBREV

*Constant*: `DwSect`



## gimli::constants::DW_SECT_INFO

*Constant*: `DwSect`



## gimli::constants::DW_SECT_LINE

*Constant*: `DwSect`



## gimli::constants::DW_SECT_LOCLISTS

*Constant*: `DwSect`



## gimli::constants::DW_SECT_MACRO

*Constant*: `DwSect`



## gimli::constants::DW_SECT_RNGLISTS

*Constant*: `DwSect`



## gimli::constants::DW_SECT_STR_OFFSETS

*Constant*: `DwSect`



## gimli::constants::DW_SECT_V2_ABBREV

*Constant*: `DwSectV2`



## gimli::constants::DW_SECT_V2_INFO

*Constant*: `DwSectV2`



## gimli::constants::DW_SECT_V2_LINE

*Constant*: `DwSectV2`



## gimli::constants::DW_SECT_V2_LOC

*Constant*: `DwSectV2`



## gimli::constants::DW_SECT_V2_MACINFO

*Constant*: `DwSectV2`



## gimli::constants::DW_SECT_V2_MACRO

*Constant*: `DwSectV2`



## gimli::constants::DW_SECT_V2_STR_OFFSETS

*Constant*: `DwSectV2`



## gimli::constants::DW_SECT_V2_TYPES

*Constant*: `DwSectV2`



## gimli::constants::DW_TAG_ALTIUM_circ_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_ALTIUM_mwa_circ_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_ALTIUM_rev_carry_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_ALTIUM_rom

*Constant*: `DwTag`



## gimli::constants::DW_TAG_APPLE_property

*Constant*: `DwTag`



## gimli::constants::DW_TAG_BORLAND_Delphi_dynamic_array

*Constant*: `DwTag`



## gimli::constants::DW_TAG_BORLAND_Delphi_set

*Constant*: `DwTag`



## gimli::constants::DW_TAG_BORLAND_Delphi_string

*Constant*: `DwTag`



## gimli::constants::DW_TAG_BORLAND_Delphi_variant

*Constant*: `DwTag`



## gimli::constants::DW_TAG_BORLAND_property

*Constant*: `DwTag`



## gimli::constants::DW_TAG_GNU_BINCL

*Constant*: `DwTag`



## gimli::constants::DW_TAG_GNU_EINCL

*Constant*: `DwTag`



## gimli::constants::DW_TAG_GNU_call_site

*Constant*: `DwTag`



## gimli::constants::DW_TAG_GNU_call_site_parameter

*Constant*: `DwTag`



## gimli::constants::DW_TAG_GNU_formal_parameter_pack

*Constant*: `DwTag`



## gimli::constants::DW_TAG_GNU_template_parameter_pack

*Constant*: `DwTag`



## gimli::constants::DW_TAG_GNU_template_template_param

*Constant*: `DwTag`



## gimli::constants::DW_TAG_HP_Bliss_field

*Constant*: `DwTag`



## gimli::constants::DW_TAG_HP_Bliss_field_set

*Constant*: `DwTag`



## gimli::constants::DW_TAG_HP_array_descriptor

*Constant*: `DwTag`



## gimli::constants::DW_TAG_MIPS_loop

*Constant*: `DwTag`



## gimli::constants::DW_TAG_PGI_interface_block

*Constant*: `DwTag`



## gimli::constants::DW_TAG_PGI_kanji_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_class_template

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_codeflags

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_dtor

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_dtor_info

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_f90_interface

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_fortran_vax_structure

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_function_template

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_indirect_inheritance

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_memop_info

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_omp_child_func

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_rtti_descriptor

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_struct_template

*Constant*: `DwTag`



## gimli::constants::DW_TAG_SUN_union_template

*Constant*: `DwTag`



## gimli::constants::DW_TAG_access_declaration

*Constant*: `DwTag`



## gimli::constants::DW_TAG_array_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_atomic_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_base_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_call_site

*Constant*: `DwTag`



## gimli::constants::DW_TAG_call_site_parameter

*Constant*: `DwTag`



## gimli::constants::DW_TAG_catch_block

*Constant*: `DwTag`



## gimli::constants::DW_TAG_class_template

*Constant*: `DwTag`



## gimli::constants::DW_TAG_class_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_coarray_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_common_block

*Constant*: `DwTag`



## gimli::constants::DW_TAG_common_inclusion

*Constant*: `DwTag`



## gimli::constants::DW_TAG_compile_unit

*Constant*: `DwTag`



## gimli::constants::DW_TAG_condition

*Constant*: `DwTag`



## gimli::constants::DW_TAG_const_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_constant

*Constant*: `DwTag`



## gimli::constants::DW_TAG_dwarf_procedure

*Constant*: `DwTag`



## gimli::constants::DW_TAG_dynamic_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_entry_point

*Constant*: `DwTag`



## gimli::constants::DW_TAG_enumeration_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_enumerator

*Constant*: `DwTag`



## gimli::constants::DW_TAG_file_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_formal_parameter

*Constant*: `DwTag`



## gimli::constants::DW_TAG_format_label

*Constant*: `DwTag`



## gimli::constants::DW_TAG_friend

*Constant*: `DwTag`



## gimli::constants::DW_TAG_function_template

*Constant*: `DwTag`



## gimli::constants::DW_TAG_generic_subrange

*Constant*: `DwTag`



## gimli::constants::DW_TAG_global_subroutine

*Constant*: `DwTag`



## gimli::constants::DW_TAG_global_variable

*Constant*: `DwTag`



## gimli::constants::DW_TAG_hi_user

*Constant*: `DwTag`



## gimli::constants::DW_TAG_immutable_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_imported_declaration

*Constant*: `DwTag`



## gimli::constants::DW_TAG_imported_module

*Constant*: `DwTag`



## gimli::constants::DW_TAG_imported_unit

*Constant*: `DwTag`



## gimli::constants::DW_TAG_inheritance

*Constant*: `DwTag`



## gimli::constants::DW_TAG_inlined_subroutine

*Constant*: `DwTag`



## gimli::constants::DW_TAG_interface_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_label

*Constant*: `DwTag`



## gimli::constants::DW_TAG_lexical_block

*Constant*: `DwTag`



## gimli::constants::DW_TAG_lo_user

*Constant*: `DwTag`



## gimli::constants::DW_TAG_local_variable

*Constant*: `DwTag`



## gimli::constants::DW_TAG_member

*Constant*: `DwTag`



## gimli::constants::DW_TAG_module

*Constant*: `DwTag`



## gimli::constants::DW_TAG_namelist

*Constant*: `DwTag`



## gimli::constants::DW_TAG_namelist_item

*Constant*: `DwTag`



## gimli::constants::DW_TAG_namespace

*Constant*: `DwTag`



## gimli::constants::DW_TAG_null

*Constant*: `DwTag`



## gimli::constants::DW_TAG_packed_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_partial_unit

*Constant*: `DwTag`



## gimli::constants::DW_TAG_pointer_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_ptr_to_member_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_reference_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_restrict_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_rvalue_reference_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_set_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_shared_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_skeleton_unit

*Constant*: `DwTag`



## gimli::constants::DW_TAG_string_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_structure_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_subprogram

*Constant*: `DwTag`



## gimli::constants::DW_TAG_subrange_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_subroutine

*Constant*: `DwTag`



## gimli::constants::DW_TAG_subroutine_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_template_alias

*Constant*: `DwTag`



## gimli::constants::DW_TAG_template_type_parameter

*Constant*: `DwTag`



## gimli::constants::DW_TAG_template_value_parameter

*Constant*: `DwTag`



## gimli::constants::DW_TAG_thrown_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_try_block

*Constant*: `DwTag`



## gimli::constants::DW_TAG_type_unit

*Constant*: `DwTag`



## gimli::constants::DW_TAG_typedef

*Constant*: `DwTag`



## gimli::constants::DW_TAG_union_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_unspecified_parameters

*Constant*: `DwTag`



## gimli::constants::DW_TAG_unspecified_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_upc_relaxed_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_upc_shared_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_upc_strict_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_variable

*Constant*: `DwTag`



## gimli::constants::DW_TAG_variant

*Constant*: `DwTag`



## gimli::constants::DW_TAG_variant_part

*Constant*: `DwTag`



## gimli::constants::DW_TAG_volatile_type

*Constant*: `DwTag`



## gimli::constants::DW_TAG_with_stmt

*Constant*: `DwTag`



## gimli::constants::DW_UT_compile

*Constant*: `DwUt`



## gimli::constants::DW_UT_hi_user

*Constant*: `DwUt`



## gimli::constants::DW_UT_lo_user

*Constant*: `DwUt`



## gimli::constants::DW_UT_partial

*Constant*: `DwUt`



## gimli::constants::DW_UT_skeleton

*Constant*: `DwUt`



## gimli::constants::DW_UT_split_compile

*Constant*: `DwUt`



## gimli::constants::DW_UT_split_type

*Constant*: `DwUt`



## gimli::constants::DW_UT_type

*Constant*: `DwUt`



## gimli::constants::DW_VIRTUALITY_none

*Constant*: `DwVirtuality`



## gimli::constants::DW_VIRTUALITY_pure_virtual

*Constant*: `DwVirtuality`



## gimli::constants::DW_VIRTUALITY_virtual

*Constant*: `DwVirtuality`



## gimli::constants::DW_VIS_exported

*Constant*: `DwVis`



## gimli::constants::DW_VIS_local

*Constant*: `DwVis`



## gimli::constants::DW_VIS_qualified

*Constant*: `DwVis`



## gimli::constants::DwAccess

*Struct*

The encodings of the constants used in the `DW_AT_accessibility` attribute.

See Section 7.9, Table 7.14.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DwAccess) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwAccess`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwAccess) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwAccess) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::constants::DwAddr

*Struct*

The encodings of the constants used in the `DW_AT_address_class` attribute.

There is only one value that is common to all target architectures.
See Section 7.13.

**Tuple Struct**: `(u64)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwAddr) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwAddr) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwAddr`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwAddr) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwAt

*Struct*

The attribute encodings for DIE attributes.

See Section 7.5.4, Table 7.5.

**Tuple Struct**: `(u16)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwAt) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwAt) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwAt) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwAt`



## gimli::constants::DwAte

*Struct*

The encodings of the constants used in the `DW_AT_encoding` attribute.

See Section 7.8, Table 7.11.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwAte`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwAte) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwAte) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwAte) -> bool`



## gimli::constants::DwCc

*Struct*

The encodings of the constants used in the `DW_AT_calling_convention` attribute.

See Section 7.15, Table 7.19.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwCc) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwCc) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwCc`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwCc) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwCfa

*Struct*

The opcode for a call frame instruction.

Section 7.24:
> Call frame instructions are encoded in one or more bytes. The primary
> opcode is encoded in the high order two bits of the first byte (that is,
> opcode = byte >> 6). An operand or extended opcode may be encoded in the
> low order 6 bits. Additional operands are encoded in subsequent bytes.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwCfa) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwCfa) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwCfa`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwCfa) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwChildren

*Struct*

The child determination encodings for DIE attributes.

See Section 7.5.3, Table 7.4.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DwChildren`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwChildren) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwChildren) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwChildren) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`



## gimli::constants::DwDefaulted

*Struct*

The encodings of the constants used in the `DW_AT_defaulted` attribute.

See Section 7.20, Table 7.24.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwDefaulted) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwDefaulted) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwDefaulted) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwDefaulted`



## gimli::constants::DwDs

*Struct*

The encodings of the constants used in the `DW_AT_decimal_sign` attribute.

See Section 7.8, Table 7.12.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwDs`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwDs) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwDs) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwDs) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::constants::DwDsc

*Struct*

The encodings of the constants used in the `DW_AT_discr_list` attribute.

See Section 7.18, Table 7.22.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwDsc) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwDsc`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwDsc) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwDsc) -> $crate::cmp::Ordering`



## gimli::constants::DwEhPe

*Struct*

Pointer encoding used by `.eh_frame`.

The four lower bits describe the
format of the pointer, the upper four bits describe how the encoding should
be applied.

Defined in `<https://refspecs.linuxfoundation.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/dwarfext.html>`

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`
- `fn format(self: Self) -> DwEhPe` - Get the pointer encoding's format.
- `fn application(self: Self) -> DwEhPe` - Get the pointer encoding's application.
- `fn is_absent(self: Self) -> bool` - Is this encoding the absent pointer encoding?
- `fn is_indirect(self: Self) -> bool` - Is this coding indirect? If so, its encoded value is the address of the
- `fn is_valid_encoding(self: Self) -> bool` - Is this a known, valid pointer encoding?

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **BitOr**
  - `fn bitor(self: Self, rhs: DwEhPe) -> DwEhPe`
- **Clone**
  - `fn clone(self: &Self) -> DwEhPe`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwEhPe) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwEhPe) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwEhPe) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::constants::DwEnd

*Struct*

The encodings of the constants used in the `DW_AT_endianity` attribute.

See Section 7.8, Table 7.13.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwEnd`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwEnd) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwEnd) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwEnd) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::constants::DwForm

*Struct*

The attribute form encodings for DIE attributes.

See Section 7.5.6, Table 7.6.

**Tuple Struct**: `(u16)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwForm) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwForm) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwForm) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwForm`



## gimli::constants::DwId

*Struct*

The encodings of the constants used in the `DW_AT_identifier_case` attribute.

See Section 7.14, Table 7.18.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwId`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwId) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwId) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwId) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::constants::DwIdx

*Struct*

Name index attribute encodings.

See Section 7.19, Table 7.23.

**Tuple Struct**: `(u16)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwIdx) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwIdx`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwIdx) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwIdx) -> $crate::cmp::Ordering`



## gimli::constants::DwInl

*Struct*

The encodings of the constants used in the `DW_AT_inline` attribute.

See Section 7.16, Table 7.20.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwInl) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwInl) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwInl) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwInl`



## gimli::constants::DwLang

*Struct*

The encodings of the constants used in the `DW_AT_language` attribute.

See Section 7.12, Table 7.17.

**Tuple Struct**: `(u16)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`
- `fn default_lower_bound(self: Self) -> Option<usize>` - Get the default DW_AT_lower_bound for this language.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwLang) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwLang) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwLang`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwLang) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwLle

*Struct*

The encodings of the constants used in location list entries.

See Section 7.7.3, Table 7.10.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwLle) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwLle) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwLle`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwLle) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwLnct

*Struct*

The encodings for the line number header entry formats.

See Section 7.22, Table 7.27.

**Tuple Struct**: `(u16)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwLnct`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwLnct) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwLnct) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwLnct) -> bool`



## gimli::constants::DwLne

*Struct*

The encodings for the extended opcodes for line number information.

See Section 7.22, Table 7.26.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwLne) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwLne) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwLne`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwLne) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwLns

*Struct*

The encodings for the standard opcodes for line number information.

See Section 7.22, Table 7.25.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwLns) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwLns) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwLns`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwLns) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwMacinfo

*Struct*

Type codes for macro definitions in the `.debug_macinfo` section.

See Section 7.22, Figure 39 for DWARF 4.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwMacinfo) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwMacinfo`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwMacinfo) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwMacinfo) -> $crate::cmp::Ordering`



## gimli::constants::DwMacro

*Struct*

The encodings for macro information entry types.

See Section 7.23, Table 7.28 for DWARF 5.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwMacro) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwMacro) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwMacro`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwMacro) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwOp

*Struct*

The encodings for DWARF expression operations.

See Section 7.7.1, Table 7.9.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwOp) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwOp) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwOp) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwOp`



## gimli::constants::DwOrd

*Struct*

The encodings of the constants used in the `DW_AT_ordering` attribute.

See Section 7.17, Table 7.17.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DwOrd) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwOrd`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwOrd) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwOrd) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::constants::DwRle

*Struct*

Range list entry encoding values.

See Section 7.25, Table 7.30.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwRle) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwRle) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwRle`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwRle) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwSect

*Struct*

The section type field in a `.dwp` unit index.

This is used for version 5 and later.

See Section 7.3.5.

**Tuple Struct**: `(u32)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwSect) -> $crate::cmp::Ordering`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwSect) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> DwSect`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwSect) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwSectV2

*Struct*

The section type field in a `.dwp` unit index with version 2.

**Tuple Struct**: `(u32)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwSectV2) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwSectV2`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwSectV2) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwSectV2) -> $crate::cmp::Ordering`



## gimli::constants::DwTag

*Struct*

The tag encodings for DIE attributes.

See Section 7.5.3, Table 7.3.

**Tuple Struct**: `(u16)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DwTag`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwTag) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwTag) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwTag) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`



## gimli::constants::DwUt

*Struct*

The unit type field in a unit header.

See Section 7.5.1, Table 7.2.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwUt) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwUt) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwUt`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwUt) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwVirtuality

*Struct*

The encodings of the constants used in the `DW_AT_virtuality` attribute.

See Section 7.11, Table 7.16.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &DwVirtuality) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwVirtuality) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwVirtuality`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwVirtuality) -> $crate::option::Option<$crate::cmp::Ordering>`



## gimli::constants::DwVis

*Struct*

The encodings of the constants used in the `DW_AT_visibility` attribute.

See Section 7.10, Table 7.15.

**Tuple Struct**: `(u8)`

**Methods:**

- `fn static_string(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DwVis) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &DwVis) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwVis) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> DwVis`



