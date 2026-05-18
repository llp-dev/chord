**object > xcoff**

# Module: xcoff

## Contents

**Structs**

- [`AuxHeader32`](#auxheader32) - The auxiliary header immediately following file header. If the value of the
- [`AuxHeader64`](#auxheader64) - The auxiliary header immediately following file header. If the value of the
- [`BlockAux32`](#blockaux32) - Block auxiliary entry for the C_BLOCK and C_FCN Symbols.
- [`BlockAux64`](#blockaux64) - Block auxiliary entry for the C_BLOCK and C_FCN Symbols.
- [`CsectAux32`](#csectaux32) - Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols.
- [`CsectAux64`](#csectaux64) - Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols.
- [`DwarfAux32`](#dwarfaux32) - Section auxiliary entry Format for C_DWARF symbols.
- [`DwarfAux64`](#dwarfaux64) - Section auxiliary entry Format for C_DWARF symbols.
- [`ExpAux`](#expaux) - Exception auxiliary entry. (XCOFF64 only)
- [`FileAux32`](#fileaux32) - File Auxiliary Entry for C_FILE Symbols.
- [`FileAux64`](#fileaux64) - File Auxiliary Entry for C_FILE Symbols.
- [`FileHeader32`](#fileheader32) - The header at the start of every 32-bit XCOFF file.
- [`FileHeader64`](#fileheader64) - The header at the start of every 64-bit XCOFF file.
- [`FunAux32`](#funaux32) - Function auxiliary entry.
- [`FunAux64`](#funaux64) - Function auxiliary entry.
- [`Rel32`](#rel32) - Relocation table entry
- [`Rel64`](#rel64) - Relocation table entry
- [`SectionHeader32`](#sectionheader32) - Section header.
- [`SectionHeader64`](#sectionheader64) - Section header.
- [`StatAux`](#stataux) - Section auxiliary entry for the C_STAT Symbol. (XCOFF32 Only)
- [`Symbol32`](#symbol32) - Symbol table entry.
- [`Symbol64`](#symbol64) - Symbol table entry.
- [`SymbolBytes`](#symbolbytes)

**Constants**

- [`AOUTHSZ_SHORT`](#aouthsz_short) - Some AIX programs generate auxiliary headers for 32-bit object files that
- [`AUX_CSECT`](#aux_csect) - Identifies a csect auxiliary entry.
- [`AUX_EXCEPT`](#aux_except) - Identifies an exception auxiliary entry.
- [`AUX_FCN`](#aux_fcn) - Identifies a function auxiliary entry.
- [`AUX_FILE`](#aux_file) - Identifies a file auxiliary entry.
- [`AUX_SECT`](#aux_sect) - Identifies a SECT auxiliary entry.
- [`AUX_SYM`](#aux_sym) - Identifies a symbol auxiliary entry.
- [`C_ALIAS`](#c_alias) - Duplicate tag.
- [`C_ARG`](#c_arg) - Function argument.
- [`C_AUTO`](#c_auto) - Automatic variable.
- [`C_BCOMM`](#c_bcomm) - Beginning of common block.
- [`C_BINCL`](#c_bincl) - Beginning of include file.
- [`C_BLOCK`](#c_block) - Beginning or end of inner block.
- [`C_BSTAT`](#c_bstat) - Beginning of static block.
- [`C_DECL`](#c_decl) - Declaration of object (type).
- [`C_DWARF`](#c_dwarf) - DWARF section symbol.
- [`C_ECOML`](#c_ecoml) - Local member of common block.
- [`C_ECOMM`](#c_ecomm) - End of common block.
- [`C_EFCN`](#c_efcn) - Physical end of function.
- [`C_EINCL`](#c_eincl) - Ending of include file.
- [`C_ENTAG`](#c_entag) - Enumeration tag.
- [`C_ENTRY`](#c_entry) - Alternate entry.
- [`C_EOS`](#c_eos) - End of structure.
- [`C_ESTAT`](#c_estat) - End of static block.
- [`C_EXT`](#c_ext) - External symbol.
- [`C_EXTDEF`](#c_extdef) - External definition.
- [`C_FCN`](#c_fcn) - Beginning or end of function.
- [`C_FIELD`](#c_field) - Bit field.
- [`C_FILE`](#c_file) - Source file name and compiler information.
- [`C_FUN`](#c_fun) - Function or procedure.
- [`C_GSYM`](#c_gsym) - Global variable.
- [`C_GTLS`](#c_gtls) - Global thread-local variable.
- [`C_HIDDEN`](#c_hidden) - Special storage class for external.
- [`C_HIDEXT`](#c_hidext) - Un-named external symbol.
- [`C_INFO`](#c_info) - Comment string in .info section.
- [`C_LABEL`](#c_label) - Label.
- [`C_LSYM`](#c_lsym) - Automatic variable allocated on stack.
- [`C_MOE`](#c_moe) - Member of enumeration.
- [`C_MOS`](#c_mos) - Member of structure.
- [`C_MOU`](#c_mou) - Member of union.
- [`C_NULL`](#c_null) - Symbol table entry marked for deletion.
- [`C_PSYM`](#c_psym) - Argument to subroutine allocated on stack.
- [`C_REG`](#c_reg) - Register variable.
- [`C_REGPARM`](#c_regparm) - Register parameter.
- [`C_RPSYM`](#c_rpsym) - Argument to function or procedure stored in register.
- [`C_RSYM`](#c_rsym) - Register variable.
- [`C_STAT`](#c_stat) - Static.
- [`C_STRTAG`](#c_strtag) - Structure tag.
- [`C_STSYM`](#c_stsym) - Statically allocated symbol.
- [`C_STTLS`](#c_sttls) - Static thread-local variable.
- [`C_TCSYM`](#c_tcsym) - Reserved.
- [`C_TPDEF`](#c_tpdef) - Type definition.
- [`C_ULABEL`](#c_ulabel) - Undefined label.
- [`C_UNTAG`](#c_untag) - Union tag.
- [`C_USTATIC`](#c_ustatic) - Undefined static.
- [`C_WEAKEXT`](#c_weakext) - Weak external symbol.
- [`F_DSA`](#f_dsa) - Indicates that the file uses Very Large Program Support.
- [`F_DYNLOAD`](#f_dynload) - Indicates the file is dynamically loadable and executable. External references
- [`F_EXEC`](#f_exec) - Indicates that the file is executable. No unresolved external references exist.
- [`F_FDPR_OPTI`](#f_fdpr_opti) - Indicates that the file was reordered with the fdpr command.
- [`F_FDPR_PROF`](#f_fdpr_prof) - Indicates that the file was profiled with the fdpr command.
- [`F_LNNO`](#f_lnno) - Indicates that line numbers have been stripped from the file by a utility program.
- [`F_LOADONLY`](#f_loadonly) - If the object file is a member of an archive, it can be loaded by the system
- [`F_RELFLG`](#f_relflg) - Indicates that the relocation information for binding has been removed from
- [`F_SHROBJ`](#f_shrobj) - Indicates the file is a shared object (shared library). The file is separately
- [`F_VARPG`](#f_varpg) - Indicates that one of the members of the auxiliary header specifying the
- [`MAGIC_32`](#magic_32) - the 32-bit mach magic number
- [`MAGIC_64`](#magic_64) - the 64-bit mach magic number
- [`N_ABS`](#n_abs) - An absolute symbol. The symbol has a value but is not relocatable.
- [`N_DEBUG`](#n_debug) - A special symbolic debugging symbol.
- [`N_UNDEF`](#n_undef) - An undefined external symbol.
- [`R_BA`](#r_ba) - Branch absolute relocation. References a non-modifiable instruction.
- [`R_BR`](#r_br) - Branch relative to self relocation. References a non-modifiable instruction.
- [`R_GL`](#r_gl) - Global linkage-external TOC address relocation.
- [`R_NEG`](#r_neg) - Negative relocation.
- [`R_POS`](#r_pos) - Positive relocation.
- [`R_RBA`](#r_rba) - Branch absolute relocation. References a modifiable instruction.
- [`R_RBR`](#r_rbr) - Branch relative to self relocation. References a modifiable instruction.
- [`R_REF`](#r_ref) - A non-relocating relocation.
- [`R_REL`](#r_rel) - Relative to self relocation.
- [`R_RL`](#r_rl) - Positive indirect load relocation.
- [`R_RLA`](#r_rla) - Positive load address relocation. Modifiable instruction.
- [`R_TCL`](#r_tcl) - Local object TOC address relocation.
- [`R_TLS`](#r_tls) - General-dynamic reference to TLS symbol.
- [`R_TLSM`](#r_tlsm) - Module reference to TLS.
- [`R_TLSML`](#r_tlsml) - Module reference to the local TLS storage.
- [`R_TLS_IE`](#r_tls_ie) - Initial-exec reference to TLS symbol.
- [`R_TLS_LD`](#r_tls_ld) - Local-dynamic reference to TLS symbol.
- [`R_TLS_LE`](#r_tls_le) - Local-exec reference to TLS symbol.
- [`R_TOC`](#r_toc) - Relative to the TOC relocation.
- [`R_TOCL`](#r_tocl) - Relative to TOC lower.
- [`R_TOCU`](#r_tocu) - Relative to TOC upper.
- [`R_TRL`](#r_trl) - TOC relative indirect load relocation.
- [`R_TRLA`](#r_trla) - Relative to the TOC or to the thread-local storage base relocation.
- [`SIZEOF_SYMBOL`](#sizeof_symbol)
- [`SSUBTYP_DWABREV`](#ssubtyp_dwabrev)
- [`SSUBTYP_DWARNGE`](#ssubtyp_dwarnge)
- [`SSUBTYP_DWFRAME`](#ssubtyp_dwframe)
- [`SSUBTYP_DWINFO`](#ssubtyp_dwinfo)
- [`SSUBTYP_DWLINE`](#ssubtyp_dwline)
- [`SSUBTYP_DWLOC`](#ssubtyp_dwloc)
- [`SSUBTYP_DWMAC`](#ssubtyp_dwmac)
- [`SSUBTYP_DWPBNMS`](#ssubtyp_dwpbnms)
- [`SSUBTYP_DWPBTYP`](#ssubtyp_dwpbtyp)
- [`SSUBTYP_DWRNGES`](#ssubtyp_dwrnges)
- [`SSUBTYP_DWSTR`](#ssubtyp_dwstr)
- [`STYP_BSS`](#styp_bss) - Specifies an uninitialized data section. A section header of this type
- [`STYP_DATA`](#styp_data) - Specifies an initialized data section. A section of this type contains the
- [`STYP_DEBUG`](#styp_debug) - Specifies a debug section. A section of this type contains stabstring
- [`STYP_DWARF`](#styp_dwarf) - Specifies a DWARF debugging section, which provide source file and symbol
- [`STYP_EXCEPT`](#styp_except) - Specifies an exception section. A section of this type provides information
- [`STYP_INFO`](#styp_info) - Specifies a comment section. A section of this type provides comments or data
- [`STYP_LOADER`](#styp_loader) - Specifies a loader section. A section of this type contains object file
- [`STYP_OVRFLO`](#styp_ovrflo) - Specifies a relocation or line-number field overflow section. A section
- [`STYP_PAD`](#styp_pad) - Specifies a pad section. A section of this type is used to provide alignment
- [`STYP_REG`](#styp_reg) - "regular" section
- [`STYP_TBSS`](#styp_tbss) - Specifies an uninitialized thread-local data section.
- [`STYP_TDATA`](#styp_tdata) - Specifies an initialized thread-local data section.
- [`STYP_TEXT`](#styp_text) - Specifies an executable text (code) section. A section of this type contains
- [`STYP_TYPCHK`](#styp_typchk) - Specifies a type-check section. A section of this type contains
- [`SYM_V_EXPORTED`](#sym_v_exported)
- [`SYM_V_HIDDEN`](#sym_v_hidden)
- [`SYM_V_INTERNAL`](#sym_v_internal)
- [`SYM_V_MASK`](#sym_v_mask) - Values for visibility as they would appear when encoded in the high 4 bits
- [`SYM_V_PROTECTED`](#sym_v_protected)
- [`XFT_CD`](#xft_cd) - Specifies compiler-defined information.
- [`XFT_CT`](#xft_ct) - Specifies the compiler time stamp.
- [`XFT_CV`](#xft_cv) - Specifies the compiler version number.
- [`XFT_FN`](#xft_fn) - Specifies the source-file name.
- [`XMC_BS`](#xmc_bs) - BSS class (uninitialized static internal)
- [`XMC_DB`](#xmc_db) - Debug Dictionary Table
- [`XMC_DS`](#xmc_ds) - Descriptor csect
- [`XMC_GL`](#xmc_gl) - Global Linkage (Interfile Interface Code)
- [`XMC_PR`](#xmc_pr) - Program Code
- [`XMC_RO`](#xmc_ro) - Read Only Constant
- [`XMC_RW`](#xmc_rw) - Read Write Data
- [`XMC_SV`](#xmc_sv) - Supervisor Call (32-bit process only)
- [`XMC_SV3264`](#xmc_sv3264) - Supervisor Call for both 32- and 64-bit processes
- [`XMC_SV64`](#xmc_sv64) - Supervisor Call for 64-bit process
- [`XMC_TB`](#xmc_tb) - Traceback Table csect
- [`XMC_TC`](#xmc_tc) - General TOC item
- [`XMC_TC0`](#xmc_tc0) - TOC Anchor for TOC Addressability
- [`XMC_TD`](#xmc_td) - Scalar data item in the TOC
- [`XMC_TE`](#xmc_te) - Symbol mapped at the end of TOC
- [`XMC_TI`](#xmc_ti) - Traceback Index csect
- [`XMC_TL`](#xmc_tl) - Initialized thread-local variable
- [`XMC_UA`](#xmc_ua) - Unclassified - Treated as Read Write
- [`XMC_UC`](#xmc_uc) - Un-named Fortran Common
- [`XMC_UL`](#xmc_ul) - Uninitialized thread-local variable
- [`XMC_XO`](#xmc_xo) - Extended Operation (Pseudo Machine Instruction)
- [`XTY_CM`](#xty_cm) - Common csect definition. For uninitialized storage.
- [`XTY_ER`](#xty_er) - External reference.
- [`XTY_LD`](#xty_ld) - Defines an entry point to an initialized csect.
- [`XTY_SD`](#xty_sd) - Csect definition for initialized storage.

---

## object::xcoff::AOUTHSZ_SHORT

*Constant*: `u16`

Some AIX programs generate auxiliary headers for 32-bit object files that
end after the data_start field.



## object::xcoff::AUX_CSECT

*Constant*: `u8`

Identifies a csect auxiliary entry.



## object::xcoff::AUX_EXCEPT

*Constant*: `u8`

Identifies an exception auxiliary entry.



## object::xcoff::AUX_FCN

*Constant*: `u8`

Identifies a function auxiliary entry.



## object::xcoff::AUX_FILE

*Constant*: `u8`

Identifies a file auxiliary entry.



## object::xcoff::AUX_SECT

*Constant*: `u8`

Identifies a SECT auxiliary entry.



## object::xcoff::AUX_SYM

*Constant*: `u8`

Identifies a symbol auxiliary entry.



## object::xcoff::AuxHeader32

*Struct*

The auxiliary header immediately following file header. If the value of the
f_opthdr field in the file header is 0, the auxiliary header does not exist.

**Fields:**
- `o_mflag: crate::endian::U16<crate::endian::BigEndian>` - Flags.
- `o_vstamp: crate::endian::U16<crate::endian::BigEndian>` - Version.
- `o_tsize: crate::endian::U32<crate::endian::BigEndian>` - Text size in bytes.
- `o_dsize: crate::endian::U32<crate::endian::BigEndian>` - Initialized data size in bytes.
- `o_bsize: crate::endian::U32<crate::endian::BigEndian>` - Uninitialized data size in bytes.
- `o_entry: crate::endian::U32<crate::endian::BigEndian>` - Entry point descriptor (virtual address).
- `o_text_start: crate::endian::U32<crate::endian::BigEndian>` - Base address of text (virtual address).
- `o_data_start: crate::endian::U32<crate::endian::BigEndian>` - Base address of data (virtual address).
- `o_toc: crate::endian::U32<crate::endian::BigEndian>` - Address of TOC anchor.
- `o_snentry: crate::endian::U16<crate::endian::BigEndian>` - Section number for entry point.
- `o_sntext: crate::endian::U16<crate::endian::BigEndian>` - Section number for .text.
- `o_sndata: crate::endian::U16<crate::endian::BigEndian>` - Section number for .data.
- `o_sntoc: crate::endian::U16<crate::endian::BigEndian>` - Section number for TOC.
- `o_snloader: crate::endian::U16<crate::endian::BigEndian>` - Section number for loader data.
- `o_snbss: crate::endian::U16<crate::endian::BigEndian>` - Section number for .bss.
- `o_algntext: crate::endian::U16<crate::endian::BigEndian>` - Maximum alignment for .text.
- `o_algndata: crate::endian::U16<crate::endian::BigEndian>` - Maximum alignment for .data.
- `o_modtype: crate::endian::U16<crate::endian::BigEndian>` - Module type field.
- `o_cpuflag: u8` - Bit flags - cpu types of objects.
- `o_cputype: u8` - Reserved for CPU type.
- `o_maxstack: crate::endian::U32<crate::endian::BigEndian>` - Maximum stack size allowed (bytes).
- `o_maxdata: crate::endian::U32<crate::endian::BigEndian>` - Maximum data size allowed (bytes).
- `o_debugger: crate::endian::U32<crate::endian::BigEndian>` - Reserved for debuggers.
- `o_textpsize: u8` - Requested text page size.
- `o_datapsize: u8` - Requested data page size.
- `o_stackpsize: u8` - Requested stack page size.
- `o_flags: u8` - Flags and thread-local storage alignment.
- `o_sntdata: crate::endian::U16<crate::endian::BigEndian>` - Section number for .tdata.
- `o_sntbss: crate::endian::U16<crate::endian::BigEndian>` - Section number for .tbss.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AuxHeader32`
- **AuxHeader**
  - `fn o_mflag(self: &Self) -> u16`
  - `fn o_vstamp(self: &Self) -> u16`
  - `fn o_tsize(self: &Self) -> <Self as >::Word`
  - `fn o_dsize(self: &Self) -> <Self as >::Word`
  - `fn o_bsize(self: &Self) -> <Self as >::Word`
  - `fn o_entry(self: &Self) -> <Self as >::Word`
  - `fn o_text_start(self: &Self) -> <Self as >::Word`
  - `fn o_data_start(self: &Self) -> <Self as >::Word`
  - `fn o_toc(self: &Self) -> <Self as >::Word`
  - `fn o_snentry(self: &Self) -> u16`
  - `fn o_sntext(self: &Self) -> u16`
  - `fn o_sndata(self: &Self) -> u16`
  - `fn o_sntoc(self: &Self) -> u16`
  - `fn o_snloader(self: &Self) -> u16`
  - `fn o_snbss(self: &Self) -> u16`
  - `fn o_algntext(self: &Self) -> u16`
  - `fn o_algndata(self: &Self) -> u16`
  - `fn o_modtype(self: &Self) -> u16`
  - `fn o_cpuflag(self: &Self) -> u8`
  - `fn o_cputype(self: &Self) -> u8`
  - `fn o_maxstack(self: &Self) -> <Self as >::Word`
  - `fn o_maxdata(self: &Self) -> <Self as >::Word`
  - `fn o_debugger(self: &Self) -> u32`
  - `fn o_textpsize(self: &Self) -> u8`
  - `fn o_datapsize(self: &Self) -> u8`
  - `fn o_stackpsize(self: &Self) -> u8`
  - `fn o_flags(self: &Self) -> u8`
  - `fn o_sntdata(self: &Self) -> u16`
  - `fn o_sntbss(self: &Self) -> u16`
  - `fn o_x64flags(self: &Self) -> Option<u16>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::xcoff::AuxHeader64

*Struct*

The auxiliary header immediately following file header. If the value of the
f_opthdr field in the file header is 0, the auxiliary header does not exist.

**Fields:**
- `o_mflag: crate::endian::U16<crate::endian::BigEndian>` - Flags.
- `o_vstamp: crate::endian::U16<crate::endian::BigEndian>` - Version.
- `o_debugger: crate::endian::U32<crate::endian::BigEndian>` - Reserved for debuggers.
- `o_text_start: crate::endian::U64<crate::endian::BigEndian>` - Base address of text (virtual address).
- `o_data_start: crate::endian::U64<crate::endian::BigEndian>` - Base address of data (virtual address).
- `o_toc: crate::endian::U64<crate::endian::BigEndian>` - Address of TOC anchor.
- `o_snentry: crate::endian::U16<crate::endian::BigEndian>` - Section number for entry point.
- `o_sntext: crate::endian::U16<crate::endian::BigEndian>` - Section number for .text.
- `o_sndata: crate::endian::U16<crate::endian::BigEndian>` - Section number for .data.
- `o_sntoc: crate::endian::U16<crate::endian::BigEndian>` - Section number for TOC.
- `o_snloader: crate::endian::U16<crate::endian::BigEndian>` - Section number for loader data.
- `o_snbss: crate::endian::U16<crate::endian::BigEndian>` - Section number for .bss.
- `o_algntext: crate::endian::U16<crate::endian::BigEndian>` - Maximum alignment for .text.
- `o_algndata: crate::endian::U16<crate::endian::BigEndian>` - Maximum alignment for .data.
- `o_modtype: crate::endian::U16<crate::endian::BigEndian>` - Module type field.
- `o_cpuflag: u8` - Bit flags - cpu types of objects.
- `o_cputype: u8` - Reserved for CPU type.
- `o_textpsize: u8` - Requested text page size.
- `o_datapsize: u8` - Requested data page size.
- `o_stackpsize: u8` - Requested stack page size.
- `o_flags: u8` - Flags and thread-local storage alignment.
- `o_tsize: crate::endian::U64<crate::endian::BigEndian>` - Text size in bytes.
- `o_dsize: crate::endian::U64<crate::endian::BigEndian>` - Initialized data size in bytes.
- `o_bsize: crate::endian::U64<crate::endian::BigEndian>` - Uninitialized data size in bytes.
- `o_entry: crate::endian::U64<crate::endian::BigEndian>` - Entry point descriptor (virtual address).
- `o_maxstack: crate::endian::U64<crate::endian::BigEndian>` - Maximum stack size allowed (bytes).
- `o_maxdata: crate::endian::U64<crate::endian::BigEndian>` - Maximum data size allowed (bytes).
- `o_sntdata: crate::endian::U16<crate::endian::BigEndian>` - Section number for .tdata.
- `o_sntbss: crate::endian::U16<crate::endian::BigEndian>` - Section number for .tbss.
- `o_x64flags: crate::endian::U16<crate::endian::BigEndian>` - XCOFF64 flags.
- `o_resv3a: crate::endian::U16<crate::endian::BigEndian>` - Reserved.
- `o_resv3: [crate::endian::U32<crate::endian::BigEndian>; 2]` - Reserved.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AuxHeader64`
- **AuxHeader**
  - `fn o_mflag(self: &Self) -> u16`
  - `fn o_vstamp(self: &Self) -> u16`
  - `fn o_tsize(self: &Self) -> <Self as >::Word`
  - `fn o_dsize(self: &Self) -> <Self as >::Word`
  - `fn o_bsize(self: &Self) -> <Self as >::Word`
  - `fn o_entry(self: &Self) -> <Self as >::Word`
  - `fn o_text_start(self: &Self) -> <Self as >::Word`
  - `fn o_data_start(self: &Self) -> <Self as >::Word`
  - `fn o_toc(self: &Self) -> <Self as >::Word`
  - `fn o_snentry(self: &Self) -> u16`
  - `fn o_sntext(self: &Self) -> u16`
  - `fn o_sndata(self: &Self) -> u16`
  - `fn o_sntoc(self: &Self) -> u16`
  - `fn o_snloader(self: &Self) -> u16`
  - `fn o_snbss(self: &Self) -> u16`
  - `fn o_algntext(self: &Self) -> u16`
  - `fn o_algndata(self: &Self) -> u16`
  - `fn o_modtype(self: &Self) -> u16`
  - `fn o_cpuflag(self: &Self) -> u8`
  - `fn o_cputype(self: &Self) -> u8`
  - `fn o_maxstack(self: &Self) -> <Self as >::Word`
  - `fn o_maxdata(self: &Self) -> <Self as >::Word`
  - `fn o_debugger(self: &Self) -> u32`
  - `fn o_textpsize(self: &Self) -> u8`
  - `fn o_datapsize(self: &Self) -> u8`
  - `fn o_stackpsize(self: &Self) -> u8`
  - `fn o_flags(self: &Self) -> u8`
  - `fn o_sntdata(self: &Self) -> u16`
  - `fn o_sntbss(self: &Self) -> u16`
  - `fn o_x64flags(self: &Self) -> Option<u16>`



## object::xcoff::BlockAux32

*Struct*

Block auxiliary entry for the C_BLOCK and C_FCN Symbols.

**Fields:**
- `pad: [u8; 2]` - Reserved.
- `x_lnnohi: crate::endian::U16<crate::endian::BigEndian>` - High-order 2 bytes of the source line number.
- `x_lnnolo: crate::endian::U16<crate::endian::BigEndian>` - Low-order 2 bytes of the source line number.
- `pad2: [u8; 12]` - Reserved.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BlockAux32`



## object::xcoff::BlockAux64

*Struct*

Block auxiliary entry for the C_BLOCK and C_FCN Symbols.

**Fields:**
- `x_lnno: crate::endian::U32<crate::endian::BigEndian>` - Source line number.
- `pad: [u8; 13]` - Reserved.
- `x_auxtype: u8` - Contains _AUX_SYM; Type of auxiliary entry.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BlockAux64`



## object::xcoff::C_ALIAS

*Constant*: `u8`

Duplicate tag.



## object::xcoff::C_ARG

*Constant*: `u8`

Function argument.



## object::xcoff::C_AUTO

*Constant*: `u8`

Automatic variable.



## object::xcoff::C_BCOMM

*Constant*: `u8`

Beginning of common block.



## object::xcoff::C_BINCL

*Constant*: `u8`

Beginning of include file.



## object::xcoff::C_BLOCK

*Constant*: `u8`

Beginning or end of inner block.



## object::xcoff::C_BSTAT

*Constant*: `u8`

Beginning of static block.



## object::xcoff::C_DECL

*Constant*: `u8`

Declaration of object (type).



## object::xcoff::C_DWARF

*Constant*: `u8`

DWARF section symbol.



## object::xcoff::C_ECOML

*Constant*: `u8`

Local member of common block.



## object::xcoff::C_ECOMM

*Constant*: `u8`

End of common block.



## object::xcoff::C_EFCN

*Constant*: `u8`

Physical end of function.



## object::xcoff::C_EINCL

*Constant*: `u8`

Ending of include file.



## object::xcoff::C_ENTAG

*Constant*: `u8`

Enumeration tag.



## object::xcoff::C_ENTRY

*Constant*: `u8`

Alternate entry.



## object::xcoff::C_EOS

*Constant*: `u8`

End of structure.



## object::xcoff::C_ESTAT

*Constant*: `u8`

End of static block.



## object::xcoff::C_EXT

*Constant*: `u8`

External symbol.



## object::xcoff::C_EXTDEF

*Constant*: `u8`

External definition.



## object::xcoff::C_FCN

*Constant*: `u8`

Beginning or end of function.



## object::xcoff::C_FIELD

*Constant*: `u8`

Bit field.



## object::xcoff::C_FILE

*Constant*: `u8`

Source file name and compiler information.



## object::xcoff::C_FUN

*Constant*: `u8`

Function or procedure.



## object::xcoff::C_GSYM

*Constant*: `u8`

Global variable.



## object::xcoff::C_GTLS

*Constant*: `u8`

Global thread-local variable.



## object::xcoff::C_HIDDEN

*Constant*: `u8`

Special storage class for external.



## object::xcoff::C_HIDEXT

*Constant*: `u8`

Un-named external symbol.



## object::xcoff::C_INFO

*Constant*: `u8`

Comment string in .info section.



## object::xcoff::C_LABEL

*Constant*: `u8`

Label.



## object::xcoff::C_LSYM

*Constant*: `u8`

Automatic variable allocated on stack.



## object::xcoff::C_MOE

*Constant*: `u8`

Member of enumeration.



## object::xcoff::C_MOS

*Constant*: `u8`

Member of structure.



## object::xcoff::C_MOU

*Constant*: `u8`

Member of union.



## object::xcoff::C_NULL

*Constant*: `u8`

Symbol table entry marked for deletion.



## object::xcoff::C_PSYM

*Constant*: `u8`

Argument to subroutine allocated on stack.



## object::xcoff::C_REG

*Constant*: `u8`

Register variable.



## object::xcoff::C_REGPARM

*Constant*: `u8`

Register parameter.



## object::xcoff::C_RPSYM

*Constant*: `u8`

Argument to function or procedure stored in register.



## object::xcoff::C_RSYM

*Constant*: `u8`

Register variable.



## object::xcoff::C_STAT

*Constant*: `u8`

Static.



## object::xcoff::C_STRTAG

*Constant*: `u8`

Structure tag.



## object::xcoff::C_STSYM

*Constant*: `u8`

Statically allocated symbol.



## object::xcoff::C_STTLS

*Constant*: `u8`

Static thread-local variable.



## object::xcoff::C_TCSYM

*Constant*: `u8`

Reserved.



## object::xcoff::C_TPDEF

*Constant*: `u8`

Type definition.



## object::xcoff::C_ULABEL

*Constant*: `u8`

Undefined label.



## object::xcoff::C_UNTAG

*Constant*: `u8`

Union tag.



## object::xcoff::C_USTATIC

*Constant*: `u8`

Undefined static.



## object::xcoff::C_WEAKEXT

*Constant*: `u8`

Weak external symbol.



## object::xcoff::CsectAux32

*Struct*

Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols.

**Fields:**
- `x_scnlen: crate::endian::U32<crate::endian::BigEndian>` - Section length.
- `x_parmhash: crate::endian::U32<crate::endian::BigEndian>` - Offset of parameter type-check hash in .typchk section.
- `x_snhash: crate::endian::U16<crate::endian::BigEndian>` - .typchk section number.
- `x_smtyp: u8` - Symbol alignment and type.
- `x_smclas: u8` - Storage mapping class.
- `x_stab: crate::endian::U32<crate::endian::BigEndian>` - Reserved.
- `x_snstab: crate::endian::U16<crate::endian::BigEndian>` - x_snstab.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CsectAux32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CsectAux**
  - `fn x_scnlen(self: &Self) -> u64`
  - `fn x_parmhash(self: &Self) -> u32`
  - `fn x_snhash(self: &Self) -> u16`
  - `fn x_smtyp(self: &Self) -> u8`
  - `fn x_smclas(self: &Self) -> u8`
  - `fn x_stab(self: &Self) -> Option<u32>`
  - `fn x_snstab(self: &Self) -> Option<u16>`
  - `fn x_auxtype(self: &Self) -> Option<u8>`



## object::xcoff::CsectAux64

*Struct*

Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols.

**Fields:**
- `x_scnlen_lo: crate::endian::U32<crate::endian::BigEndian>` - Low 4 bytes of section length.
- `x_parmhash: crate::endian::U32<crate::endian::BigEndian>` - Offset of parameter type-check hash in .typchk section.
- `x_snhash: crate::endian::U16<crate::endian::BigEndian>` - .typchk section number.
- `x_smtyp: u8` - Symbol alignment and type.
- `x_smclas: u8` - Storage mapping class.
- `x_scnlen_hi: crate::endian::U32<crate::endian::BigEndian>` - High 4 bytes of section length.
- `pad: u8` - Reserved.
- `x_auxtype: u8` - Contains _AUX_CSECT; indicates type of auxiliary entry.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CsectAux64`
- **CsectAux**
  - `fn x_scnlen(self: &Self) -> u64`
  - `fn x_parmhash(self: &Self) -> u32`
  - `fn x_snhash(self: &Self) -> u16`
  - `fn x_smtyp(self: &Self) -> u8`
  - `fn x_smclas(self: &Self) -> u8`
  - `fn x_stab(self: &Self) -> Option<u32>`
  - `fn x_snstab(self: &Self) -> Option<u16>`
  - `fn x_auxtype(self: &Self) -> Option<u8>`



## object::xcoff::DwarfAux32

*Struct*

Section auxiliary entry Format for C_DWARF symbols.

**Fields:**
- `x_scnlen: crate::endian::U32<crate::endian::BigEndian>` - Length of portion of section represented by symbol.
- `pad: [u8; 4]` - Reserved.
- `x_nreloc: crate::endian::U32<crate::endian::BigEndian>` - Number of relocation entries in section.
- `pad2: [u8; 6]` - Reserved.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DwarfAux32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::xcoff::DwarfAux64

*Struct*

Section auxiliary entry Format for C_DWARF symbols.

**Fields:**
- `x_scnlen: crate::endian::U64<crate::endian::BigEndian>` - Length of portion of section represented by symbol.
- `x_nreloc: crate::endian::U64<crate::endian::BigEndian>` - Number of relocation entries in section.
- `pad: u8` - Reserved.
- `x_auxtype: u8` - Contains _AUX_SECT; Type of Auxiliary entry.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DwarfAux64`



## object::xcoff::ExpAux

*Struct*

Exception auxiliary entry. (XCOFF64 only)

**Fields:**
- `x_exptr: crate::endian::U64<crate::endian::BigEndian>` - File offset to exception table entry.
- `x_fsize: crate::endian::U32<crate::endian::BigEndian>` - Size of function in bytes.
- `x_endndx: crate::endian::U32<crate::endian::BigEndian>` - Symbol table index of next entry beyond this function.
- `pad: u8` - Pad
- `x_auxtype: u8` - Contains _AUX_EXCEPT; Type of auxiliary entry

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ExpAux`



## object::xcoff::F_DSA

*Constant*: `u16`

Indicates that the file uses Very Large Program Support.



## object::xcoff::F_DYNLOAD

*Constant*: `u16`

Indicates the file is dynamically loadable and executable. External references
are resolved by way of imports, and the file might contain exports and loader
relocation.



## object::xcoff::F_EXEC

*Constant*: `u16`

Indicates that the file is executable. No unresolved external references exist.



## object::xcoff::F_FDPR_OPTI

*Constant*: `u16`

Indicates that the file was reordered with the fdpr command.



## object::xcoff::F_FDPR_PROF

*Constant*: `u16`

Indicates that the file was profiled with the fdpr command.



## object::xcoff::F_LNNO

*Constant*: `u16`

Indicates that line numbers have been stripped from the file by a utility program.



## object::xcoff::F_LOADONLY

*Constant*: `u16`

If the object file is a member of an archive, it can be loaded by the system
loader, but the member is ignored by the binder. If the object file is not in
an archive, this flag has no effect.



## object::xcoff::F_RELFLG

*Constant*: `u16`

Indicates that the relocation information for binding has been removed from
the file.



## object::xcoff::F_SHROBJ

*Constant*: `u16`

Indicates the file is a shared object (shared library). The file is separately
loadable. That is, it is not normally bound with other objects, and its loader
exports symbols are used as automatic import symbols for other object files.



## object::xcoff::F_VARPG

*Constant*: `u16`

Indicates that one of the members of the auxiliary header specifying the
medium page sizes is non-zero.



## object::xcoff::FileAux32

*Struct*

File Auxiliary Entry for C_FILE Symbols.

**Fields:**
- `x_fname: [u8; 8]` - The source file name or compiler-related string.
- `x_fpad: [u8; 6]` - Pad size for file name.
- `x_ftype: u8` - The source-file string type.
- `x_freserve: [u8; 3]` - Reserved.

**Traits:** Pod, Copy

**Trait Implementations:**

- **FileAux**
  - `fn x_fname(self: &Self) -> &[u8; 8]`
  - `fn x_ftype(self: &Self) -> u8`
  - `fn x_auxtype(self: &Self) -> Option<u8>`
- **Clone**
  - `fn clone(self: &Self) -> FileAux32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::xcoff::FileAux64

*Struct*

File Auxiliary Entry for C_FILE Symbols.

**Fields:**
- `x_fname: [u8; 8]` - The source file name or compiler-related string.
- `x_fpad: [u8; 6]` - Pad size for file name.
- `x_ftype: u8` - The source-file string type.
- `x_freserve: [u8; 2]` - Reserved.
- `x_auxtype: u8` - Specifies the type of auxiliary entry. Contains _AUX_FILE for this auxiliary entry.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FileAux**
  - `fn x_fname(self: &Self) -> &[u8; 8]`
  - `fn x_ftype(self: &Self) -> u8`
  - `fn x_auxtype(self: &Self) -> Option<u8>`
- **Clone**
  - `fn clone(self: &Self) -> FileAux64`



## object::xcoff::FileHeader32

*Struct*

The header at the start of every 32-bit XCOFF file.

**Fields:**
- `f_magic: crate::endian::U16<crate::endian::BigEndian>` - Magic number. Must be 0x01DF.
- `f_nscns: crate::endian::U16<crate::endian::BigEndian>` - Number of sections.
- `f_timdat: crate::endian::U32<crate::endian::BigEndian>` - Time and date of file creation.
- `f_symptr: crate::endian::U32<crate::endian::BigEndian>` - Byte offset to symbol table start.
- `f_nsyms: crate::endian::U32<crate::endian::BigEndian>` - Number of entries in symbol table.
- `f_opthdr: crate::endian::U16<crate::endian::BigEndian>` - Number of bytes in optional header
- `f_flags: crate::endian::U16<crate::endian::BigEndian>` - Extra flags.

**Traits:** Copy, Pod

**Trait Implementations:**

- **FileHeader**
  - `fn is_type_64(self: &Self) -> bool`
  - `fn f_magic(self: &Self) -> u16`
  - `fn f_nscns(self: &Self) -> u16`
  - `fn f_timdat(self: &Self) -> u32`
  - `fn f_symptr(self: &Self) -> <Self as >::Word`
  - `fn f_nsyms(self: &Self) -> u32`
  - `fn f_opthdr(self: &Self) -> u16`
  - `fn f_flags(self: &Self) -> u16`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FileHeader32`



## object::xcoff::FileHeader64

*Struct*

The header at the start of every 64-bit XCOFF file.

**Fields:**
- `f_magic: crate::endian::U16<crate::endian::BigEndian>` - Magic number. Must be 0x01F7.
- `f_nscns: crate::endian::U16<crate::endian::BigEndian>` - Number of sections.
- `f_timdat: crate::endian::U32<crate::endian::BigEndian>` - Time and date of file creation
- `f_symptr: crate::endian::U64<crate::endian::BigEndian>` - Byte offset to symbol table start.
- `f_opthdr: crate::endian::U16<crate::endian::BigEndian>` - Number of bytes in optional header
- `f_flags: crate::endian::U16<crate::endian::BigEndian>` - Extra flags.
- `f_nsyms: crate::endian::U32<crate::endian::BigEndian>` - Number of entries in symbol table.

**Traits:** Copy, Pod

**Trait Implementations:**

- **FileHeader**
  - `fn is_type_64(self: &Self) -> bool`
  - `fn f_magic(self: &Self) -> u16`
  - `fn f_nscns(self: &Self) -> u16`
  - `fn f_timdat(self: &Self) -> u32`
  - `fn f_symptr(self: &Self) -> <Self as >::Word`
  - `fn f_nsyms(self: &Self) -> u32`
  - `fn f_opthdr(self: &Self) -> u16`
  - `fn f_flags(self: &Self) -> u16`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FileHeader64`



## object::xcoff::FunAux32

*Struct*

Function auxiliary entry.

**Fields:**
- `x_exptr: crate::endian::U32<crate::endian::BigEndian>` - File offset to exception table entry.
- `x_fsize: crate::endian::U32<crate::endian::BigEndian>` - Size of function in bytes.
- `x_lnnoptr: crate::endian::U32<crate::endian::BigEndian>` - File pointer to line number
- `x_endndx: crate::endian::U32<crate::endian::BigEndian>` - Symbol table index of next entry beyond this function.
- `pad: crate::endian::U16<crate::endian::BigEndian>` - Pad

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FunAux32`



## object::xcoff::FunAux64

*Struct*

Function auxiliary entry.

**Fields:**
- `x_lnnoptr: crate::endian::U64<crate::endian::BigEndian>` - File pointer to line number
- `x_fsize: crate::endian::U32<crate::endian::BigEndian>` - Size of function in bytes.
- `x_endndx: crate::endian::U32<crate::endian::BigEndian>` - Symbol table index of next entry beyond this function.
- `pad: u8` - Pad
- `x_auxtype: u8` - Contains _AUX_FCN; Type of auxiliary entry.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FunAux64`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::xcoff::MAGIC_32

*Constant*: `u16`

the 32-bit mach magic number



## object::xcoff::MAGIC_64

*Constant*: `u16`

the 64-bit mach magic number



## object::xcoff::N_ABS

*Constant*: `i16`

An absolute symbol. The symbol has a value but is not relocatable.



## object::xcoff::N_DEBUG

*Constant*: `i16`

A special symbolic debugging symbol.



## object::xcoff::N_UNDEF

*Constant*: `i16`

An undefined external symbol.



## object::xcoff::R_BA

*Constant*: `u8`

Branch absolute relocation. References a non-modifiable instruction.



## object::xcoff::R_BR

*Constant*: `u8`

Branch relative to self relocation. References a non-modifiable instruction.



## object::xcoff::R_GL

*Constant*: `u8`

Global linkage-external TOC address relocation.



## object::xcoff::R_NEG

*Constant*: `u8`

Negative relocation.



## object::xcoff::R_POS

*Constant*: `u8`

Positive relocation.



## object::xcoff::R_RBA

*Constant*: `u8`

Branch absolute relocation. References a modifiable instruction.



## object::xcoff::R_RBR

*Constant*: `u8`

Branch relative to self relocation. References a modifiable instruction.



## object::xcoff::R_REF

*Constant*: `u8`

A non-relocating relocation.



## object::xcoff::R_REL

*Constant*: `u8`

Relative to self relocation.



## object::xcoff::R_RL

*Constant*: `u8`

Positive indirect load relocation.



## object::xcoff::R_RLA

*Constant*: `u8`

Positive load address relocation. Modifiable instruction.



## object::xcoff::R_TCL

*Constant*: `u8`

Local object TOC address relocation.



## object::xcoff::R_TLS

*Constant*: `u8`

General-dynamic reference to TLS symbol.



## object::xcoff::R_TLSM

*Constant*: `u8`

Module reference to TLS.



## object::xcoff::R_TLSML

*Constant*: `u8`

Module reference to the local TLS storage.



## object::xcoff::R_TLS_IE

*Constant*: `u8`

Initial-exec reference to TLS symbol.



## object::xcoff::R_TLS_LD

*Constant*: `u8`

Local-dynamic reference to TLS symbol.



## object::xcoff::R_TLS_LE

*Constant*: `u8`

Local-exec reference to TLS symbol.



## object::xcoff::R_TOC

*Constant*: `u8`

Relative to the TOC relocation.



## object::xcoff::R_TOCL

*Constant*: `u8`

Relative to TOC lower.



## object::xcoff::R_TOCU

*Constant*: `u8`

Relative to TOC upper.



## object::xcoff::R_TRL

*Constant*: `u8`

TOC relative indirect load relocation.



## object::xcoff::R_TRLA

*Constant*: `u8`

Relative to the TOC or to the thread-local storage base relocation.



## object::xcoff::Rel32

*Struct*

Relocation table entry

**Fields:**
- `r_vaddr: crate::endian::U32<crate::endian::BigEndian>` - Virtual address (position) in section to be relocated.
- `r_symndx: crate::endian::U32<crate::endian::BigEndian>` - Symbol table index of item that is referenced.
- `r_rsize: u8` - Relocation size and information.
- `r_rtype: u8` - Relocation type.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Rel32`
- **Rel**
  - `fn r_vaddr(self: &Self) -> <Self as >::Word`
  - `fn r_symndx(self: &Self) -> u32`
  - `fn r_rsize(self: &Self) -> u8`
  - `fn r_rtype(self: &Self) -> u8`



## object::xcoff::Rel64

*Struct*

Relocation table entry

**Fields:**
- `r_vaddr: crate::endian::U64<crate::endian::BigEndian>` - Virtual address (position) in section to be relocated.
- `r_symndx: crate::endian::U32<crate::endian::BigEndian>` - Symbol table index of item that is referenced.
- `r_rsize: u8` - Relocation size and information.
- `r_rtype: u8` - Relocation type.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Rel64`
- **Rel**
  - `fn r_vaddr(self: &Self) -> <Self as >::Word`
  - `fn r_symndx(self: &Self) -> u32`
  - `fn r_rsize(self: &Self) -> u8`
  - `fn r_rtype(self: &Self) -> u8`



## object::xcoff::SIZEOF_SYMBOL

*Constant*: `usize`



## object::xcoff::SSUBTYP_DWABREV

*Constant*: `u32`



## object::xcoff::SSUBTYP_DWARNGE

*Constant*: `u32`



## object::xcoff::SSUBTYP_DWFRAME

*Constant*: `u32`



## object::xcoff::SSUBTYP_DWINFO

*Constant*: `u32`



## object::xcoff::SSUBTYP_DWLINE

*Constant*: `u32`



## object::xcoff::SSUBTYP_DWLOC

*Constant*: `u32`



## object::xcoff::SSUBTYP_DWMAC

*Constant*: `u32`



## object::xcoff::SSUBTYP_DWPBNMS

*Constant*: `u32`



## object::xcoff::SSUBTYP_DWPBTYP

*Constant*: `u32`



## object::xcoff::SSUBTYP_DWRNGES

*Constant*: `u32`



## object::xcoff::SSUBTYP_DWSTR

*Constant*: `u32`



## object::xcoff::STYP_BSS

*Constant*: `u16`

Specifies an uninitialized data section. A section header of this type
defines the uninitialized data of a program.



## object::xcoff::STYP_DATA

*Constant*: `u16`

Specifies an initialized data section. A section of this type contains the
initialized data and the TOC of a program.



## object::xcoff::STYP_DEBUG

*Constant*: `u16`

Specifies a debug section. A section of this type contains stabstring
information used by the symbolic debugger.



## object::xcoff::STYP_DWARF

*Constant*: `u16`

Specifies a DWARF debugging section, which provide source file and symbol
information for the symbolic debugger.



## object::xcoff::STYP_EXCEPT

*Constant*: `u16`

Specifies an exception section. A section of this type provides information
to identify the reason that a trap or exception occurred within an executable
object program.



## object::xcoff::STYP_INFO

*Constant*: `u16`

Specifies a comment section. A section of this type provides comments or data
to special processing utility programs.



## object::xcoff::STYP_LOADER

*Constant*: `u16`

Specifies a loader section. A section of this type contains object file
information for the system loader to load an XCOFF executable. The information
includes imported symbols, exported symbols, relocation data, type-check
information, and shared object names.



## object::xcoff::STYP_OVRFLO

*Constant*: `u16`

Specifies a relocation or line-number field overflow section. A section
header of this type contains the count of relocation entries and line
number entries for some other section. This section header is required
when either of the counts exceeds 65,534.



## object::xcoff::STYP_PAD

*Constant*: `u16`

Specifies a pad section. A section of this type is used to provide alignment
padding between sections within an XCOFF executable object file. This section
header type is obsolete since padding is allowed in an XCOFF file without a
corresponding pad section header.



## object::xcoff::STYP_REG

*Constant*: `u16`

"regular" section



## object::xcoff::STYP_TBSS

*Constant*: `u16`

Specifies an uninitialized thread-local data section.



## object::xcoff::STYP_TDATA

*Constant*: `u16`

Specifies an initialized thread-local data section.



## object::xcoff::STYP_TEXT

*Constant*: `u16`

Specifies an executable text (code) section. A section of this type contains
the executable instructions of a program.



## object::xcoff::STYP_TYPCHK

*Constant*: `u16`

Specifies a type-check section. A section of this type contains
parameter/argument type-check strings used by the binder.



## object::xcoff::SYM_V_EXPORTED

*Constant*: `u16`



## object::xcoff::SYM_V_HIDDEN

*Constant*: `u16`



## object::xcoff::SYM_V_INTERNAL

*Constant*: `u16`



## object::xcoff::SYM_V_MASK

*Constant*: `u16`

Values for visibility as they would appear when encoded in the high 4 bits
of the 16-bit unsigned n_type field of symbol table entries. Valid for
32-bit XCOFF only when the o_vstamp in the auxiliary header is greater than 1.



## object::xcoff::SYM_V_PROTECTED

*Constant*: `u16`



## object::xcoff::SectionHeader32

*Struct*

Section header.

**Fields:**
- `s_name: [u8; 8]` - Section name.
- `s_paddr: crate::endian::U32<crate::endian::BigEndian>` - Physical address.
- `s_vaddr: crate::endian::U32<crate::endian::BigEndian>` - Virtual address (same as physical address).
- `s_size: crate::endian::U32<crate::endian::BigEndian>` - Section size.
- `s_scnptr: crate::endian::U32<crate::endian::BigEndian>` - Offset in file to raw data for section.
- `s_relptr: crate::endian::U32<crate::endian::BigEndian>` - Offset in file to relocation entries for section.
- `s_lnnoptr: crate::endian::U32<crate::endian::BigEndian>` - Offset in file to line number entries for section.
- `s_nreloc: crate::endian::U16<crate::endian::BigEndian>` - Number of relocation entries.
- `s_nlnno: crate::endian::U16<crate::endian::BigEndian>` - Number of line number entries.
- `s_flags: crate::endian::U32<crate::endian::BigEndian>` - Flags to define the section type.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SectionHeader32`
- **SectionHeader**
  - `fn s_name(self: &Self) -> &[u8; 8]`
  - `fn s_paddr(self: &Self) -> <Self as >::Word`
  - `fn s_vaddr(self: &Self) -> <Self as >::Word`
  - `fn s_size(self: &Self) -> <Self as >::Word`
  - `fn s_scnptr(self: &Self) -> <Self as >::Word`
  - `fn s_relptr(self: &Self) -> <Self as >::Word`
  - `fn s_lnnoptr(self: &Self) -> <Self as >::Word`
  - `fn s_nreloc(self: &Self) -> <Self as >::HalfWord`
  - `fn s_nlnno(self: &Self) -> <Self as >::HalfWord`
  - `fn s_flags(self: &Self) -> u32`
  - `fn relocations<'data, R>(self: &Self, data: R) -> read::Result<&'data [<Self as >::Rel]>` - Read the relocations in a XCOFF32 file.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::xcoff::SectionHeader64

*Struct*

Section header.

**Fields:**
- `s_name: [u8; 8]` - Section name.
- `s_paddr: crate::endian::U64<crate::endian::BigEndian>` - Physical address.
- `s_vaddr: crate::endian::U64<crate::endian::BigEndian>` - Virtual address (same as physical address).
- `s_size: crate::endian::U64<crate::endian::BigEndian>` - Section size.
- `s_scnptr: crate::endian::U64<crate::endian::BigEndian>` - Offset in file to raw data for section.
- `s_relptr: crate::endian::U64<crate::endian::BigEndian>` - Offset in file to relocation entries for section.
- `s_lnnoptr: crate::endian::U64<crate::endian::BigEndian>` - Offset in file to line number entries for section.
- `s_nreloc: crate::endian::U32<crate::endian::BigEndian>` - Number of relocation entries.
- `s_nlnno: crate::endian::U32<crate::endian::BigEndian>` - Number of line number entries.
- `s_flags: crate::endian::U32<crate::endian::BigEndian>` - Flags to define the section type.
- `s_reserve: crate::endian::U32<crate::endian::BigEndian>` - Reserved.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **SectionHeader**
  - `fn s_name(self: &Self) -> &[u8; 8]`
  - `fn s_paddr(self: &Self) -> <Self as >::Word`
  - `fn s_vaddr(self: &Self) -> <Self as >::Word`
  - `fn s_size(self: &Self) -> <Self as >::Word`
  - `fn s_scnptr(self: &Self) -> <Self as >::Word`
  - `fn s_relptr(self: &Self) -> <Self as >::Word`
  - `fn s_lnnoptr(self: &Self) -> <Self as >::Word`
  - `fn s_nreloc(self: &Self) -> <Self as >::HalfWord`
  - `fn s_nlnno(self: &Self) -> <Self as >::HalfWord`
  - `fn s_flags(self: &Self) -> u32`
  - `fn relocations<'data, R>(self: &Self, data: R) -> read::Result<&'data [<Self as >::Rel]>` - Read the relocations in a XCOFF64 file.
- **Clone**
  - `fn clone(self: &Self) -> SectionHeader64`



## object::xcoff::StatAux

*Struct*

Section auxiliary entry for the C_STAT Symbol. (XCOFF32 Only)

**Fields:**
- `x_scnlen: crate::endian::U32<crate::endian::BigEndian>` - Section length.
- `x_nreloc: crate::endian::U16<crate::endian::BigEndian>` - Number of relocation entries.
- `x_nlinno: crate::endian::U16<crate::endian::BigEndian>` - Number of line numbers.
- `pad: [u8; 10]` - Reserved.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> StatAux`



## object::xcoff::Symbol32

*Struct*

Symbol table entry.

**Fields:**
- `n_name: [u8; 8]` - Symbol name.
- `n_value: crate::endian::U32<crate::endian::BigEndian>` - Symbol value; storage class-dependent.
- `n_scnum: crate::endian::I16<crate::endian::BigEndian>` - Section number of symbol.
- `n_type: crate::endian::U16<crate::endian::BigEndian>` - Basic and derived type specification.
- `n_sclass: u8` - Storage class of symbol.
- `n_numaux: u8` - Number of auxiliary entries.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Symbol32`
- **Symbol**
  - `fn n_value(self: &Self) -> <Self as >::Word`
  - `fn n_scnum(self: &Self) -> i16`
  - `fn n_type(self: &Self) -> u16`
  - `fn n_sclass(self: &Self) -> u8`
  - `fn n_numaux(self: &Self) -> u8`
  - `fn name_offset(self: &Self) -> Option<u32>`
  - `fn name<'data, R>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>` - Parse the symbol name for XCOFF32.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::xcoff::Symbol64

*Struct*

Symbol table entry.

**Fields:**
- `n_value: crate::endian::U64<crate::endian::BigEndian>` - Symbol value; storage class-dependent.
- `n_offset: crate::endian::U32<crate::endian::BigEndian>` - Offset of the name in string table or .debug section.
- `n_scnum: crate::endian::I16<crate::endian::BigEndian>` - Section number of symbol.
- `n_type: crate::endian::U16<crate::endian::BigEndian>` - Basic and derived type specification.
- `n_sclass: u8` - Storage class of symbol.
- `n_numaux: u8` - Number of auxiliary entries.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Symbol64`
- **Symbol**
  - `fn n_value(self: &Self) -> <Self as >::Word`
  - `fn n_scnum(self: &Self) -> i16`
  - `fn n_type(self: &Self) -> u16`
  - `fn n_sclass(self: &Self) -> u8`
  - `fn n_numaux(self: &Self) -> u8`
  - `fn name_offset(self: &Self) -> Option<u32>`
  - `fn name<'data, R>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>` - Parse the symbol name for XCOFF64.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::xcoff::SymbolBytes

*Struct*

**Tuple Struct**: `([u8; 18])`

**Traits:** Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SymbolBytes`



## object::xcoff::XFT_CD

*Constant*: `u8`

Specifies compiler-defined information.



## object::xcoff::XFT_CT

*Constant*: `u8`

Specifies the compiler time stamp.



## object::xcoff::XFT_CV

*Constant*: `u8`

Specifies the compiler version number.



## object::xcoff::XFT_FN

*Constant*: `u8`

Specifies the source-file name.



## object::xcoff::XMC_BS

*Constant*: `u8`

BSS class (uninitialized static internal)



## object::xcoff::XMC_DB

*Constant*: `u8`

Debug Dictionary Table



## object::xcoff::XMC_DS

*Constant*: `u8`

Descriptor csect



## object::xcoff::XMC_GL

*Constant*: `u8`

Global Linkage (Interfile Interface Code)



## object::xcoff::XMC_PR

*Constant*: `u8`

Program Code



## object::xcoff::XMC_RO

*Constant*: `u8`

Read Only Constant



## object::xcoff::XMC_RW

*Constant*: `u8`

Read Write Data



## object::xcoff::XMC_SV

*Constant*: `u8`

Supervisor Call (32-bit process only)



## object::xcoff::XMC_SV3264

*Constant*: `u8`

Supervisor Call for both 32- and 64-bit processes



## object::xcoff::XMC_SV64

*Constant*: `u8`

Supervisor Call for 64-bit process



## object::xcoff::XMC_TB

*Constant*: `u8`

Traceback Table csect



## object::xcoff::XMC_TC

*Constant*: `u8`

General TOC item



## object::xcoff::XMC_TC0

*Constant*: `u8`

TOC Anchor for TOC Addressability



## object::xcoff::XMC_TD

*Constant*: `u8`

Scalar data item in the TOC



## object::xcoff::XMC_TE

*Constant*: `u8`

Symbol mapped at the end of TOC



## object::xcoff::XMC_TI

*Constant*: `u8`

Traceback Index csect



## object::xcoff::XMC_TL

*Constant*: `u8`

Initialized thread-local variable



## object::xcoff::XMC_UA

*Constant*: `u8`

Unclassified - Treated as Read Write



## object::xcoff::XMC_UC

*Constant*: `u8`

Un-named Fortran Common



## object::xcoff::XMC_UL

*Constant*: `u8`

Uninitialized thread-local variable



## object::xcoff::XMC_XO

*Constant*: `u8`

Extended Operation (Pseudo Machine Instruction)



## object::xcoff::XTY_CM

*Constant*: `u8`

Common csect definition. For uninitialized storage.



## object::xcoff::XTY_ER

*Constant*: `u8`

External reference.



## object::xcoff::XTY_LD

*Constant*: `u8`

Defines an entry point to an initialized csect.



## object::xcoff::XTY_SD

*Constant*: `u8`

Csect definition for initialized storage.



