**gimli > read > cfi**

# Module: read::cfi

## Contents

**Structs**

- [`Augmentation`](#augmentation) - We support the z-style augmentation [defined by `.eh_frame`][ehframe].
- [`BaseAddresses`](#baseaddresses) - Optional base addresses for the relative `DW_EH_PE_*` encoded pointers.
- [`CallFrameInstructionIter`](#callframeinstructioniter) - A lazy iterator parsing call frame instructions.
- [`CfiEntriesIter`](#cfientriesiter) - An iterator over CIE and FDE entries in a `.debug_frame` or `.eh_frame`
- [`CommonInformationEntry`](#commoninformationentry) - > A Common Information Entry holds information that is shared among many
- [`DebugFrame`](#debugframe) - `DebugFrame` contains the `.debug_frame` section's frame unwinding
- [`EhFrame`](#ehframe) - `EhFrame` contains the frame unwinding information needed during exception
- [`EhFrameHdr`](#ehframehdr) - `EhFrameHdr` contains the information about the `.eh_frame_hdr` section.
- [`EhHdrTable`](#ehhdrtable) - The CFI binary search table that is an optional part of the `.eh_frame_hdr` section.
- [`EhHdrTableIter`](#ehhdrtableiter) - An iterator for `.eh_frame_hdr` section's binary search table.
- [`FrameDescriptionEntry`](#framedescriptionentry) - A `FrameDescriptionEntry` is a set of CFA instructions for an address range.
- [`ParsedEhFrameHdr`](#parsedehframehdr) - `ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section.
- [`PartialFrameDescriptionEntry`](#partialframedescriptionentry) - A partially parsed `FrameDescriptionEntry`.
- [`RegisterRuleIter`](#registerruleiter) - An unordered iterator for register rules.
- [`SectionBaseAddresses`](#sectionbaseaddresses) - Optional base addresses for the relative `DW_EH_PE_*` encoded pointers
- [`UnwindContext`](#unwindcontext) - Common context needed when evaluating the call frame unwinding information.
- [`UnwindExpression`](#unwindexpression) - The location of a DWARF expression within an unwind section.
- [`UnwindTable`](#unwindtable) - The `UnwindTable` iteratively evaluates a `FrameDescriptionEntry`'s
- [`UnwindTableRow`](#unwindtablerow) - A row in the virtual unwind table that describes how to find the values of

**Enums**

- [`CallFrameInstruction`](#callframeinstruction) - A parsed call frame instruction.
- [`CfaRule`](#cfarule) - The canonical frame address (CFA) recovery rules.
- [`CieOrFde`](#cieorfde) - Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE).
- [`Pointer`](#pointer) - A decoded pointer.
- [`RegisterRule`](#registerrule) - An entry in the abstract CFI table that describes how to find the value of a

**Traits**

- [`UnwindContextStorage`](#unwindcontextstorage) - Specification of what storage should be used for [`UnwindContext`].
- [`UnwindOffset`](#unwindoffset) - An offset into an `UnwindSection`.
- [`UnwindSection`](#unwindsection) - A section holding unwind information: either `.debug_frame` or

---

## gimli::read::cfi::Augmentation

*Struct*

We support the z-style augmentation [defined by `.eh_frame`][ehframe].

[ehframe]: https://refspecs.linuxfoundation.org/LSB_3.0.0/LSB-Core-generic/LSB-Core-generic/ehframechpt.html

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Augmentation`
- **PartialEq**
  - `fn eq(self: &Self, other: &Augmentation) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Augmentation`



## gimli::read::cfi::BaseAddresses

*Struct*

Optional base addresses for the relative `DW_EH_PE_*` encoded pointers.

During CIE/FDE parsing, if a relative pointer is encountered for a base
address that is unknown, an Err will be returned.

```
use gimli::BaseAddresses;

# fn foo() {
# let address_of_eh_frame_hdr_section_in_memory = unimplemented!();
# let address_of_eh_frame_section_in_memory = unimplemented!();
# let address_of_text_section_in_memory = unimplemented!();
# let address_of_got_section_in_memory = unimplemented!();
# let address_of_the_start_of_current_func = unimplemented!();
let bases = BaseAddresses::default()
    .set_eh_frame_hdr(address_of_eh_frame_hdr_section_in_memory)
    .set_eh_frame(address_of_eh_frame_section_in_memory)
    .set_text(address_of_text_section_in_memory)
    .set_got(address_of_got_section_in_memory);
# let _ = bases;
# }
```

**Fields:**
- `eh_frame_hdr: SectionBaseAddresses` - The base addresses to use for pointers in the `.eh_frame_hdr` section.
- `eh_frame: SectionBaseAddresses` - The base addresses to use for pointers in the `.eh_frame` section.

**Methods:**

- `fn set_eh_frame_hdr(self: Self, addr: u64) -> Self` - Set the `.eh_frame_hdr` section base address.
- `fn set_eh_frame(self: Self, addr: u64) -> Self` - Set the `.eh_frame` section base address.
- `fn set_text(self: Self, addr: u64) -> Self` - Set the `.text` section base address.
- `fn set_got(self: Self, addr: u64) -> Self` - Set the `.got` section base address.

**Traits:** Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> BaseAddresses`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BaseAddresses) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BaseAddresses`



## gimli::read::cfi::CallFrameInstruction

*Enum*

A parsed call frame instruction.

**Generic Parameters:**
- T

**Variants:**
- `SetLoc{ address: u64 }` - > 1. DW_CFA_set_loc
- `AdvanceLoc{ delta: u32 }` - The `AdvanceLoc` instruction is used for all of `DW_CFA_advance_loc` and
- `DefCfa{ register: crate::common::Register, offset: u64 }` - > 1. DW_CFA_def_cfa
- `DefCfaSf{ register: crate::common::Register, factored_offset: i64 }` - > 2. DW_CFA_def_cfa_sf
- `DefCfaRegister{ register: crate::common::Register }` - > 3. DW_CFA_def_cfa_register
- `DefCfaOffset{ offset: u64 }` - > 4. DW_CFA_def_cfa_offset
- `DefCfaOffsetSf{ factored_offset: i64 }` - > 5. DW_CFA_def_cfa_offset_sf
- `DefCfaExpression{ expression: UnwindExpression<T> }` - > 6. DW_CFA_def_cfa_expression
- `Undefined{ register: crate::common::Register }` - > 1. DW_CFA_undefined
- `SameValue{ register: crate::common::Register }` - > 2. DW_CFA_same_value
- `Offset{ register: crate::common::Register, factored_offset: u64 }` - The `Offset` instruction represents both `DW_CFA_offset` and
- `OffsetExtendedSf{ register: crate::common::Register, factored_offset: i64 }` - > 5. DW_CFA_offset_extended_sf
- `ValOffset{ register: crate::common::Register, factored_offset: u64 }` - > 6. DW_CFA_val_offset
- `ValOffsetSf{ register: crate::common::Register, factored_offset: i64 }` - > 7. DW_CFA_val_offset_sf
- `Register{ dest_register: crate::common::Register, src_register: crate::common::Register }` - > 8. DW_CFA_register
- `Expression{ register: crate::common::Register, expression: UnwindExpression<T> }` - > 9. DW_CFA_expression
- `ValExpression{ register: crate::common::Register, expression: UnwindExpression<T> }` - > 10. DW_CFA_val_expression
- `Restore{ register: crate::common::Register }` - The `Restore` instruction represents both `DW_CFA_restore` and
- `RememberState` - > 1. DW_CFA_remember_state
- `RestoreState` - > 2. DW_CFA_restore_state
- `ArgsSize{ size: u64 }` - > DW_CFA_GNU_args_size
- `NegateRaState` - > DW_CFA_AARCH64_negate_ra_state
- `Nop` - > 1. DW_CFA_nop

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CallFrameInstruction<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CallFrameInstruction<T>) -> bool`



## gimli::read::cfi::CallFrameInstructionIter

*Struct*

A lazy iterator parsing call frame instructions.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

**Generic Parameters:**
- 'a
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<CallFrameInstruction<<R as >::Offset>>>` - Parse the next call frame instruction.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CallFrameInstructionIter<'a, R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::cfi::CfaRule

*Enum*

The canonical frame address (CFA) recovery rules.

**Generic Parameters:**
- T

**Variants:**
- `RegisterAndOffset{ register: crate::common::Register, offset: i64 }` - The CFA is given offset from the given register's value.
- `Expression(UnwindExpression<T>)` - The CFA is obtained by evaluating a DWARF expression program.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &CfaRule<T>) -> bool`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> CfaRule<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::cfi::CfiEntriesIter

*Struct*

An iterator over CIE and FDE entries in a `.debug_frame` or `.eh_frame`
section.

Some pointers may be encoded relative to various base addresses. Use the
[`BaseAddresses`](./struct.BaseAddresses.html) parameter to provide them. By
default, none are provided. If a relative pointer is encountered for a base
address that is unknown, an `Err` will be returned and iteration will abort.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

```
use gimli::{BaseAddresses, EhFrame, EndianSlice, NativeEndian, UnwindSection};

# fn foo() -> gimli::Result<()> {
# let read_eh_frame_somehow = || unimplemented!();
let eh_frame = EhFrame::new(read_eh_frame_somehow(), NativeEndian);

# let address_of_eh_frame_hdr_section_in_memory = unimplemented!();
# let address_of_eh_frame_section_in_memory = unimplemented!();
# let address_of_text_section_in_memory = unimplemented!();
# let address_of_got_section_in_memory = unimplemented!();
# let address_of_the_start_of_current_func = unimplemented!();
// Provide base addresses for relative pointers.
let bases = BaseAddresses::default()
    .set_eh_frame_hdr(address_of_eh_frame_hdr_section_in_memory)
    .set_eh_frame(address_of_eh_frame_section_in_memory)
    .set_text(address_of_text_section_in_memory)
    .set_got(address_of_got_section_in_memory);

let mut entries = eh_frame.entries(&bases);

# let do_stuff_with = |_| unimplemented!();
while let Some(entry) = entries.next()? {
    do_stuff_with(entry)
}
# unreachable!()
# }
```

**Generic Parameters:**
- 'bases
- Section
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<CieOrFde<'bases, Section, R>>>` - Advance the iterator to the next entry.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CfiEntriesIter<'bases, Section, R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::cfi::CieOrFde

*Enum*

Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE).

**Generic Parameters:**
- 'bases
- Section
- R

**Variants:**
- `Cie(CommonInformationEntry<R>)` - This CFI entry is a `CommonInformationEntry`.
- `Fde(PartialFrameDescriptionEntry<'bases, Section, R>)` - This CFI entry is a `FrameDescriptionEntry`, however fully parsing it

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CieOrFde<'bases, Section, R>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CieOrFde<'bases, Section, R>`



## gimli::read::cfi::CommonInformationEntry

*Struct*

> A Common Information Entry holds information that is shared among many
> Frame Description Entries. There is at least one CIE in every non-empty
> `.debug_frame` section.

**Generic Parameters:**
- R
- Offset

**Methods:**

- `fn offset(self: &Self) -> <R as >::Offset` - Get the offset of this entry from the start of its containing section.
- `fn encoding(self: &Self) -> Encoding` - Return the encoding parameters for this CIE.
- `fn address_size(self: &Self) -> u8` - The size of addresses (in bytes) in this CIE.
- `fn instructions<'a, Section>(self: &Self, section: &'a Section, bases: &'a BaseAddresses) -> CallFrameInstructionIter<'a, R>` - Iterate over this CIE's initial instructions.
- `fn entry_len(self: &Self) -> <R as >::Offset` - > A constant that gives the number of bytes of the CIE structure, not
- `fn version(self: &Self) -> u8` - > A version number (see Section 7.23). This number is specific to the
- `fn augmentation(self: &Self) -> Option<&Augmentation>` - Get the augmentation data, if any exists.
- `fn has_lsda(self: &Self) -> bool` - True if this CIE's FDEs have a LSDA.
- `fn lsda_encoding(self: &Self) -> Option<constants::DwEhPe>` - Return the encoding of the LSDA address for this CIE's FDEs.
- `fn personality_with_encoding(self: &Self) -> Option<(constants::DwEhPe, Pointer)>` - Return the encoding and address of the personality routine handler
- `fn personality(self: &Self) -> Option<Pointer>` - Return the address of the personality routine handler
- `fn fde_address_encoding(self: &Self) -> Option<constants::DwEhPe>` - Return the encoding of the addresses for this CIE's FDEs.
- `fn is_signal_trampoline(self: &Self) -> bool` - True if this CIE's FDEs are trampolines for signal handlers.
- `fn code_alignment_factor(self: &Self) -> u64` - > A constant that is factored out of all advance location instructions
- `fn data_alignment_factor(self: &Self) -> i64` - > A constant that is factored out of certain offset instructions (see
- `fn return_address_register(self: &Self) -> Register` - > An unsigned ... constant that indicates which column in the rule

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CommonInformationEntry<R, Offset>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CommonInformationEntry<R, Offset>) -> bool`



## gimli::read::cfi::DebugFrame

*Struct*

`DebugFrame` contains the `.debug_frame` section's frame unwinding
information required to unwind to and recover registers from older frames on
the stack. For example, this is useful for a debugger that wants to print
locals in a backtrace.

Most interesting methods are defined in the
[`UnwindSection`](trait.UnwindSection.html) trait.

### Differences between `.debug_frame` and `.eh_frame`

While the `.debug_frame` section's information has a lot of overlap with the
`.eh_frame` section's information, the `.eh_frame` information tends to only
encode the subset of information needed for exception handling. Often, only
one of `.eh_frame` or `.debug_frame` will be present in an object file.

**Generic Parameters:**
- R

**Methods:**

- `fn set_address_size(self: & mut Self, address_size: u8)` - Set the size of a target address in bytes.
- `fn set_vendor(self: & mut Self, vendor: Vendor)` - Set the vendor extensions to use.
- `fn new(section: &'input [u8], endian: Endian) -> Self` - Construct a new `DebugFrame` instance from the data in the

**Traits:** UnwindSection, Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DebugFrame<R>) -> bool`
- **From**
  - `fn from(section: R) -> Self`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`
- **Clone**
  - `fn clone(self: &Self) -> DebugFrame<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::cfi::EhFrame

*Struct*

`EhFrame` contains the frame unwinding information needed during exception
handling found in the `.eh_frame` section.

Most interesting methods are defined in the
[`UnwindSection`](trait.UnwindSection.html) trait.

See
[`DebugFrame`](./struct.DebugFrame.html#differences-between-debug_frame-and-eh_frame)
for some discussion on the differences between `.debug_frame` and
`.eh_frame`.

**Generic Parameters:**
- R

**Methods:**

- `fn set_address_size(self: & mut Self, address_size: u8)` - Set the size of a target address in bytes.
- `fn set_vendor(self: & mut Self, vendor: Vendor)` - Set the vendor extensions to use.
- `fn new(section: &'input [u8], endian: Endian) -> Self` - Construct a new `EhFrame` instance from the data in the

**Traits:** UnwindSection, Copy, Eq

**Trait Implementations:**

- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`
- **Clone**
  - `fn clone(self: &Self) -> EhFrame<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &EhFrame<R>) -> bool`
- **From**
  - `fn from(section: R) -> Self`



## gimli::read::cfi::EhFrameHdr

*Struct*

`EhFrameHdr` contains the information about the `.eh_frame_hdr` section.

A pointer to the start of the `.eh_frame` data, and optionally, a binary
search table of pointers to the `.eh_frame` records that are found in this section.

**Generic Parameters:**
- R

**Tuple Struct**: `()`

**Methods:**

- `fn parse(self: &Self, bases: &BaseAddresses, address_size: u8) -> Result<ParsedEhFrameHdr<R>>` - Parses this `EhFrameHdr` to a `ParsedEhFrameHdr`.
- `fn new(section: &'input [u8], endian: Endian) -> Self` - Constructs a new `EhFrameHdr` instance from the data in the `.eh_frame_hdr` section.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`
- **Clone**
  - `fn clone(self: &Self) -> EhFrameHdr<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &EhFrameHdr<R>) -> bool`
- **From**
  - `fn from(section: R) -> Self`



## gimli::read::cfi::EhHdrTable

*Struct*

The CFI binary search table that is an optional part of the `.eh_frame_hdr` section.

**Generic Parameters:**
- 'a
- R

**Methods:**

- `fn iter<'bases>(self: &Self, bases: &'bases BaseAddresses) -> EhHdrTableIter<'bases, R>` - Return an iterator that can walk the `.eh_frame_hdr` table.
- `fn lookup(self: &Self, address: u64, bases: &BaseAddresses) -> Result<Pointer>` - *Probably* returns a pointer to the FDE for the given address.
- `fn pointer_to_offset(self: &Self, ptr: Pointer) -> Result<EhFrameOffset<<R as >::Offset>>` - Convert a `Pointer` to a section offset.
- `fn fde_for_address<F>(self: &Self, frame: &EhFrame<R>, bases: &BaseAddresses, address: u64, get_cie: F) -> Result<FrameDescriptionEntry<R>>` - Returns a parsed FDE for the given address, or `NoUnwindInfoForAddress`
- `fn unwind_info_for_address<'ctx, F, S>(self: &Self, frame: &EhFrame<R>, bases: &BaseAddresses, ctx: &'ctx  mut UnwindContext<<R as >::Offset, S>, address: u64, get_cie: F) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` - Returns the frame unwind information for the given address,

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> EhHdrTable<'a, R>`



## gimli::read::cfi::EhHdrTableIter

*Struct*

An iterator for `.eh_frame_hdr` section's binary search table.

Each table entry consists of a tuple containing an  `initial_location` and `address`.
The `initial location` represents the first address that the targeted FDE
is able to decode. The `address` is the address of the FDE in the `.eh_frame` section.
The `address` can be converted with `EhHdrTable::pointer_to_offset` and `EhFrame::fde_from_offset` to an FDE.

**Generic Parameters:**
- 'a
- 'bases
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<(Pointer, Pointer)>>` - Yield the next entry in the `EhHdrTableIter`.
- `fn nth(self: & mut Self, n: usize) -> Result<Option<(Pointer, Pointer)>>` - Yield the nth entry in the `EhHdrTableIter`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::cfi::FrameDescriptionEntry

*Struct*

A `FrameDescriptionEntry` is a set of CFA instructions for an address range.

**Generic Parameters:**
- R
- Offset

**Methods:**

- `fn rows<'a, 'ctx, Section, S>(self: &Self, section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx  mut UnwindContext<<R as >::Offset, S>) -> Result<UnwindTable<'a, 'ctx, R, S>>` - Return the table of unwind information for this FDE.
- `fn unwind_info_for_address<'ctx, Section, S>(self: &Self, section: &Section, bases: &BaseAddresses, ctx: &'ctx  mut UnwindContext<<R as >::Offset, S>, address: u64) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` - Find the frame unwind information for the given address.
- `fn offset(self: &Self) -> <R as >::Offset` - Get the offset of this entry from the start of its containing section.
- `fn cie(self: &Self) -> &CommonInformationEntry<R>` - Get a reference to this FDE's CIE.
- `fn entry_len(self: &Self) -> <R as >::Offset` - > A constant that gives the number of bytes of the header and
- `fn instructions<'a, Section>(self: &Self, section: &'a Section, bases: &'a BaseAddresses) -> CallFrameInstructionIter<'a, R>` - Iterate over this FDE's instructions.
- `fn initial_address(self: &Self) -> u64` - The first address for which this entry has unwind information for.
- `fn end_address(self: &Self) -> u64` - One more than the last address that this entry has unwind information for.
- `fn len(self: &Self) -> u64` - The number of bytes of instructions that this entry has unwind
- `fn contains(self: &Self, address: u64) -> bool` - Return `true` if the given address is within this FDE, `false`
- `fn lsda(self: &Self) -> Option<Pointer>` - The address of this FDE's language-specific data area (LSDA), if it has
- `fn is_signal_trampoline(self: &Self) -> bool` - Return true if this FDE's function is a trampoline for a signal handler.
- `fn personality(self: &Self) -> Option<Pointer>` - Return the address of the FDE's function's personality routine

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FrameDescriptionEntry<R, Offset>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FrameDescriptionEntry<R, Offset>) -> bool`



## gimli::read::cfi::ParsedEhFrameHdr

*Struct*

`ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section.

**Generic Parameters:**
- R

**Methods:**

- `fn eh_frame_ptr(self: &Self) -> Pointer` - Returns the address of the binary's `.eh_frame` section.
- `fn table(self: &Self) -> Option<EhHdrTable<R>>` - Retrieves the CFI binary search table, if there is one.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ParsedEhFrameHdr<R>`



## gimli::read::cfi::PartialFrameDescriptionEntry

*Struct*

A partially parsed `FrameDescriptionEntry`.

Fully parsing this FDE requires first parsing its CIE.

**Generic Parameters:**
- 'bases
- Section
- R

**Methods:**

- `fn parse<F>(self: &Self, get_cie: F) -> Result<FrameDescriptionEntry<R>>` - Fully parse this FDE.
- `fn offset(self: &Self) -> <R as >::Offset` - Get the offset of this entry from the start of its containing section.
- `fn cie_offset(self: &Self) -> <Section as >::Offset` - Get the offset of this FDE's CIE.
- `fn entry_len(self: &Self) -> <R as >::Offset` - > A constant that gives the number of bytes of the header and

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PartialFrameDescriptionEntry<'bases, Section, R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PartialFrameDescriptionEntry<'bases, Section, R>) -> bool`



## gimli::read::cfi::Pointer

*Enum*

A decoded pointer.

**Variants:**
- `Direct(u64)` - This value is the decoded pointer value.
- `Indirect(u64)` - This value is *not* the pointer value, but points to the address of

**Methods:**

- `fn direct(self: Self) -> Result<u64>` - Return the direct pointer value.
- `fn pointer(self: Self) -> u64` - Return the pointer value, discarding indirectness information.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Pointer) -> bool`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Pointer`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::cfi::RegisterRule

*Enum*

An entry in the abstract CFI table that describes how to find the value of a
register.

"The register columns contain rules that describe whether a given register
has been saved and the rule to find the value for the register in the
previous frame."

**Generic Parameters:**
- T

**Variants:**
- `Undefined` - > A register that has this rule has no recoverable value in the previous
- `SameValue` - > This register has not been modified from the previous frame. (By
- `Offset(i64)` - "The previous value of this register is saved at the address CFA+N where
- `ValOffset(i64)` - "The previous value of this register is the value CFA+N where CFA is the
- `Register(crate::common::Register)` - "The previous value of this register is stored in another register
- `Expression(UnwindExpression<T>)` - "The previous value of this register is located at the address produced
- `ValExpression(UnwindExpression<T>)` - "The previous value of this register is the value produced by executing
- `Architectural` - "The rule is defined externally to this specification by the augmenter."
- `Constant(u64)` - This is a pseudo-register with a constant value.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RegisterRule<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RegisterRule<T>) -> bool`



## gimli::read::cfi::RegisterRuleIter

*Struct*

An unordered iterator for register rules.

**Generic Parameters:**
- 'iter
- T

**Tuple Struct**: `()`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RegisterRuleIter<'iter, T>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::cfi::SectionBaseAddresses

*Struct*

Optional base addresses for the relative `DW_EH_PE_*` encoded pointers
in a particular section.

See `BaseAddresses` for methods that are helpful in setting these addresses.

**Fields:**
- `section: Option<u64>` - The address of the section containing the pointer.
- `text: Option<u64>` - The base address for text relative pointers.
- `data: Option<u64>` - The base address for data relative pointers.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SectionBaseAddresses`
- **Default**
  - `fn default() -> SectionBaseAddresses`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SectionBaseAddresses) -> bool`



## gimli::read::cfi::UnwindContext

*Struct*

Common context needed when evaluating the call frame unwinding information.

By default, this structure is small and allocates its internal storage
on the heap using [`Box`] during [`UnwindContext::new`].

This can be overridden by providing a custom [`UnwindContextStorage`] type parameter.
When using a custom storage with in-line arrays, the [`UnwindContext`] type itself
will be big, so in that case it's recommended to place [`UnwindContext`] on the
heap, e.g. using `Box::new(UnwindContext::<R, MyCustomStorage>::new_in())`.

To avoid re-allocating the context multiple times when evaluating multiple
CFI programs, the same [`UnwindContext`] can be reused for multiple unwinds.

```
use gimli::{UnwindContext, UnwindTable};

# fn foo<'a>(some_fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'a, gimli::LittleEndian>>)
#            -> gimli::Result<()> {
# let eh_frame: gimli::EhFrame<_> = unreachable!();
# let bases = unimplemented!();
// An uninitialized context.
let mut ctx = UnwindContext::new();

// Initialize the context by evaluating the CIE's initial instruction program,
// and generate the unwind table.
let mut table = some_fde.rows(&eh_frame, &bases, &mut ctx)?;
while let Some(row) = table.next_row()? {
    // Do stuff with each row...
#   let _ = row;
}
# unreachable!()
# }
```

**Generic Parameters:**
- T
- S

**Methods:**

- `fn new_in() -> Self` - Construct a new call frame unwinding context.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UnwindContext<T, S>`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnwindContext<T, S>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`



## gimli::read::cfi::UnwindContextStorage

*Trait*

Specification of what storage should be used for [`UnwindContext`].


Here's an implementation which uses a fixed-size stack and allocates everything in-line,
which will cause `UnwindContext` to be large:

```rust,no_run
# use gimli::*;
#
# fn foo<'a>(some_fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'a, gimli::LittleEndian>>)
#            -> gimli::Result<()> {
# let eh_frame: gimli::EhFrame<_> = unreachable!();
# let bases = unimplemented!();
#
struct StoreOnStack;

impl<T: ReaderOffset> UnwindContextStorage<T> for StoreOnStack {
    type Rules = [(Register, RegisterRule<T>); 192];
    type Stack = [UnwindTableRow<T, Self>; 4];
}

let mut ctx = UnwindContext::<_, StoreOnStack>::new_in();

// Initialize the context by evaluating the CIE's initial instruction program,
// and generate the unwind table.
let mut table = some_fde.rows(&eh_frame, &bases, &mut ctx)?;
while let Some(row) = table.next_row()? {
    // Do stuff with each row...
#   let _ = row;
}
# unreachable!()
# }
```

**Methods:**

- `Rules`: The storage used for register rules in a unwind table row.
- `Stack`: The storage used for unwind table row stack.



## gimli::read::cfi::UnwindExpression

*Struct*

The location of a DWARF expression within an unwind section.

This is stored as an offset and length within the section instead of as a
`Reader` to avoid lifetime issues when reusing [`UnwindContext`].

# Example
```
# use gimli::{EhFrame, EndianSlice, NativeEndian, Error, FrameDescriptionEntry, UnwindExpression, EvaluationResult};
# fn foo() -> Result<(), Error> {
# let eh_frame: EhFrame<EndianSlice<NativeEndian>> = unreachable!();
# let fde: FrameDescriptionEntry<EndianSlice<NativeEndian>> = unimplemented!();
# let unwind_expression: UnwindExpression<_> = unimplemented!();
let expression = unwind_expression.get(&eh_frame)?;
let mut evaluation = expression.evaluation(fde.cie().encoding());
let mut result = evaluation.evaluate()?;
loop {
  match result {
     EvaluationResult::Complete => break,
     // Provide information to the evaluation.
     _ => { unimplemented!()}
  }
}
let value = evaluation.value_result();
# Ok(())
# }
```

**Generic Parameters:**
- T

**Fields:**
- `offset: T` - The offset of the expression within the section.
- `length: T` - The length of the expression.

**Methods:**

- `fn get<R, S>(self: &Self, section: &S) -> Result<Expression<R>>` - Get the expression from the section.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UnwindExpression<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnwindExpression<T>) -> bool`



## gimli::read::cfi::UnwindOffset

*Trait*

An offset into an `UnwindSection`.

**Methods:**

- `into`: Convert an `UnwindOffset<T>` into a `T`.



## gimli::read::cfi::UnwindSection

*Trait*

A section holding unwind information: either `.debug_frame` or
`.eh_frame`. See [`DebugFrame`](./struct.DebugFrame.html) and
[`EhFrame`](./struct.EhFrame.html) respectively.

**Methods:**

- `Offset`: The offset type associated with this CFI section. Either
- `entries`: Iterate over the `CommonInformationEntry`s and `FrameDescriptionEntry`s
- `cie_from_offset`: Parse the `CommonInformationEntry` at the given offset.
- `partial_fde_from_offset`: Parse the `PartialFrameDescriptionEntry` at the given offset.
- `fde_from_offset`: Parse the `FrameDescriptionEntry` at the given offset.
- `fde_for_address`: Find the `FrameDescriptionEntry` for the given address.
- `unwind_info_for_address`: Find the frame unwind information for the given address.



## gimli::read::cfi::UnwindTable

*Struct*

The `UnwindTable` iteratively evaluates a `FrameDescriptionEntry`'s
`CallFrameInstruction` program, yielding the each row one at a time.

> 6.4.1 Structure of Call Frame Information
>
> DWARF supports virtual unwinding by defining an architecture independent
> basis for recording how procedures save and restore registers during their
> lifetimes. This basis must be augmented on some machines with specific
> information that is defined by an architecture specific ABI authoring
> committee, a hardware vendor, or a compiler producer. The body defining a
> specific augmentation is referred to below as the “augmenter.”
>
> Abstractly, this mechanism describes a very large table that has the
> following structure:
>
> <table>
>   <tr>
>     <th>LOC</th><th>CFA</th><th>R0</th><th>R1</th><td>...</td><th>RN</th>
>   </tr>
>   <tr>
>     <th>L0</th> <td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
>   <tr>
>     <th>L1</th> <td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
>   <tr>
>     <td>...</td><td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
>   <tr>
>     <th>LN</th> <td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
> </table>
>
> The first column indicates an address for every location that contains code
> in a program. (In shared objects, this is an object-relative offset.) The
> remaining columns contain virtual unwinding rules that are associated with
> the indicated location.
>
> The CFA column defines the rule which computes the Canonical Frame Address
> value; it may be either a register and a signed offset that are added
> together, or a DWARF expression that is evaluated.
>
> The remaining columns are labeled by register number. This includes some
> registers that have special designation on some architectures such as the PC
> and the stack pointer register. (The actual mapping of registers for a
> particular architecture is defined by the augmenter.) The register columns
> contain rules that describe whether a given register has been saved and the
> rule to find the value for the register in the previous frame.
>
> ...
>
> This table would be extremely large if actually constructed as
> described. Most of the entries at any point in the table are identical to
> the ones above them. The whole table can be represented quite compactly by
> recording just the differences starting at the beginning address of each
> subroutine in the program.

**Generic Parameters:**
- 'a
- 'ctx
- R
- S

**Methods:**

- `fn new<Section>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx  mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Result<Self>` - Construct a new `UnwindTable` for the given
- `fn next_row(self: & mut Self) -> Result<Option<&UnwindTableRow<<R as >::Offset, S>>>` - Evaluate call frame instructions until the next row of the table is
- `fn into_current_row(self: Self) -> Option<&'ctx UnwindTableRow<<R as >::Offset, S>>` - Returns the current row with the lifetime of the context.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::cfi::UnwindTableRow

*Struct*

A row in the virtual unwind table that describes how to find the values of
the registers in the *previous* frame for a range of PC addresses.

**Generic Parameters:**
- T
- S

**Methods:**

- `fn start_address(self: &Self) -> u64` - Get the starting PC address that this row applies to.
- `fn end_address(self: &Self) -> u64` - Get the end PC address where this row's register rules become
- `fn contains(self: &Self, address: u64) -> bool` - Return `true` if the given `address` is within this row's address range,
- `fn saved_args_size(self: &Self) -> u64` - Returns the amount of args currently on the stack.
- `fn cfa(self: &Self) -> &CfaRule<T>` - Get the canonical frame address (CFA) recovery rule for this row.
- `fn register(self: &Self, register: Register) -> RegisterRule<T>` - Get the register recovery rule for the given register number.
- `fn registers(self: &Self) -> RegisterRuleIter<T>` - Iterate over all defined register `(number, rule)` pairs.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &UnwindTableRow<T, S>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



