*[gimli](../../index.md) / [read](../index.md) / [cfi](index.md)*

---

# Module `cfi`

## Contents

- [Structs](#structs)
  - [`DebugFrame`](#debugframe)
  - [`EhFrameHdr`](#ehframehdr)
  - [`ParsedEhFrameHdr`](#parsedehframehdr)
  - [`EhHdrTableIter`](#ehhdrtableiter)
  - [`EhHdrTable`](#ehhdrtable)
  - [`EhFrame`](#ehframe)
  - [`BaseAddresses`](#baseaddresses)
  - [`SectionBaseAddresses`](#sectionbaseaddresses)
  - [`CfiEntriesIter`](#cfientriesiter)
  - [`CfiEntryPrefix`](#cfientryprefix)
  - [`Augmentation`](#augmentation)
  - [`AugmentationData`](#augmentationdata)
  - [`CommonInformationEntry`](#commoninformationentry)
  - [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)
  - [`FrameDescriptionEntry`](#framedescriptionentry)
  - [`UnwindContext`](#unwindcontext)
  - [`UnwindTable`](#unwindtable)
  - [`RegisterRuleMap`](#registerrulemap)
  - [`RegisterRuleIter`](#registerruleiter)
  - [`UnwindTableRow`](#unwindtablerow)
  - [`CallFrameInstructionIter`](#callframeinstructioniter)
  - [`UnwindExpression`](#unwindexpression)
  - [`PointerEncodingParameters`](#pointerencodingparameters)
- [Enums](#enums)
  - [`CieOrFde`](#cieorfde)
  - [`CfaRule`](#cfarule)
  - [`RegisterRule`](#registerrule)
  - [`CallFrameInstruction`](#callframeinstruction)
  - [`Pointer`](#pointer)
- [Traits](#traits)
  - [`UnwindOffset`](#unwindoffset)
  - [`UnwindSection`](#unwindsection)
  - [`UnwindContextStorage`](#unwindcontextstorage)
- [Functions](#functions)
  - [`parse_cfi_entry`](#parse-cfi-entry)
  - [`parse_cfi_entry_prefix`](#parse-cfi-entry-prefix)
  - [`parse_encoded_pointer`](#parse-encoded-pointer)
  - [`parse_encoded_value`](#parse-encoded-value)
- [Constants](#constants)
  - [`MAX_RULES`](#max-rules)
  - [`MAX_UNWIND_STACK_DEPTH`](#max-unwind-stack-depth)
  - [`CFI_INSTRUCTION_HIGH_BITS_MASK`](#cfi-instruction-high-bits-mask)
  - [`CFI_INSTRUCTION_LOW_BITS_MASK`](#cfi-instruction-low-bits-mask)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugFrame`](#debugframe) | struct | `DebugFrame` contains the `.debug_frame` section's frame unwinding information required to unwind to and recover registers from older frames on the stack. |
| [`EhFrameHdr`](#ehframehdr) | struct | `EhFrameHdr` contains the information about the `.eh_frame_hdr` section. |
| [`ParsedEhFrameHdr`](#parsedehframehdr) | struct | `ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section. |
| [`EhHdrTableIter`](#ehhdrtableiter) | struct | An iterator for `.eh_frame_hdr` section's binary search table. |
| [`EhHdrTable`](#ehhdrtable) | struct | The CFI binary search table that is an optional part of the `.eh_frame_hdr` section. |
| [`EhFrame`](#ehframe) | struct | `EhFrame` contains the frame unwinding information needed during exception handling found in the `.eh_frame` section. |
| [`BaseAddresses`](#baseaddresses) | struct | Optional base addresses for the relative `DW_EH_PE_*` encoded pointers. |
| [`SectionBaseAddresses`](#sectionbaseaddresses) | struct | Optional base addresses for the relative `DW_EH_PE_*` encoded pointers in a particular section. |
| [`CfiEntriesIter`](#cfientriesiter) | struct | An iterator over CIE and FDE entries in a `.debug_frame` or `.eh_frame` section. |
| [`CfiEntryPrefix`](#cfientryprefix) | struct | The common prefix of a CIE or FDE. |
| [`Augmentation`](#augmentation) | struct | We support the z-style augmentation [defined by `.eh_frame`][ehframe]. |
| [`AugmentationData`](#augmentationdata) | struct | Parsed augmentation data for a `FrameDescriptEntry`. |
| [`CommonInformationEntry`](#commoninformationentry) | struct | > A Common Information Entry holds information that is shared among many > Frame Description Entries. |
| [`PartialFrameDescriptionEntry`](#partialframedescriptionentry) | struct | A partially parsed `FrameDescriptionEntry`. |
| [`FrameDescriptionEntry`](#framedescriptionentry) | struct | A `FrameDescriptionEntry` is a set of CFA instructions for an address range. |
| [`UnwindContext`](#unwindcontext) | struct | Common context needed when evaluating the call frame unwinding information. |
| [`UnwindTable`](#unwindtable) | struct | The `UnwindTable` iteratively evaluates a `FrameDescriptionEntry`'s `CallFrameInstruction` program, yielding the each row one at a time. |
| [`RegisterRuleMap`](#registerrulemap) | struct |  |
| [`RegisterRuleIter`](#registerruleiter) | struct | An unordered iterator for register rules. |
| [`UnwindTableRow`](#unwindtablerow) | struct | A row in the virtual unwind table that describes how to find the values of the registers in the *previous* frame for a range of PC addresses. |
| [`CallFrameInstructionIter`](#callframeinstructioniter) | struct | A lazy iterator parsing call frame instructions. |
| [`UnwindExpression`](#unwindexpression) | struct | The location of a DWARF expression within an unwind section. |
| [`PointerEncodingParameters`](#pointerencodingparameters) | struct |  |
| [`CieOrFde`](#cieorfde) | enum | Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE). |
| [`CfaRule`](#cfarule) | enum | The canonical frame address (CFA) recovery rules. |
| [`RegisterRule`](#registerrule) | enum | An entry in the abstract CFI table that describes how to find the value of a register. |
| [`CallFrameInstruction`](#callframeinstruction) | enum | A parsed call frame instruction. |
| [`Pointer`](#pointer) | enum | A decoded pointer. |
| [`UnwindOffset`](#unwindoffset) | trait | An offset into an `UnwindSection`. |
| [`UnwindSection`](#unwindsection) | trait | A section holding unwind information: either `.debug_frame` or `.eh_frame`. |
| [`UnwindContextStorage`](#unwindcontextstorage) | trait | Specification of what storage should be used for [`UnwindContext`]. |
| [`parse_cfi_entry`](#parse-cfi-entry) | fn |  |
| [`parse_cfi_entry_prefix`](#parse-cfi-entry-prefix) | fn |  |
| [`parse_encoded_pointer`](#parse-encoded-pointer) | fn |  |
| [`parse_encoded_value`](#parse-encoded-value) | fn |  |
| [`MAX_RULES`](#max-rules) | const |  |
| [`MAX_UNWIND_STACK_DEPTH`](#max-unwind-stack-depth) | const |  |
| [`CFI_INSTRUCTION_HIGH_BITS_MASK`](#cfi-instruction-high-bits-mask) | const |  |
| [`CFI_INSTRUCTION_LOW_BITS_MASK`](#cfi-instruction-low-bits-mask) | const |  |

## Structs

### `DebugFrame<R: Reader>`

```rust
struct DebugFrame<R: Reader> {
    section: R,
    address_size: u8,
    vendor: crate::common::Vendor,
}
```

`DebugFrame` contains the `.debug_frame` section's frame unwinding
information required to unwind to and recover registers from older frames on
the stack. For example, this is useful for a debugger that wants to print
locals in a backtrace.

Most interesting methods are defined in the
[`UnwindSection`](#unwindsection) trait.

### Differences between `.debug_frame` and `.eh_frame`

While the `.debug_frame` section's information has a lot of overlap with the
`.eh_frame` section's information, the `.eh_frame` information tends to only
encode the subset of information needed for exception handling. Often, only
one of `.eh_frame` or `.debug_frame` will be present in an object file.

#### Implementations

- <span id="debugframe-set-address-size"></span>`fn set_address_size(&mut self, address_size: u8)`

  Set the size of a target address in bytes.

  

  This defaults to the native word size.

  This is only used if the CIE version is less than 4.

- <span id="debugframe-set-vendor"></span>`fn set_vendor(&mut self, vendor: Vendor)` — [`Vendor`](../../index.md#vendor)

  Set the vendor extensions to use.

  

  This defaults to `Vendor::Default`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugFrame<R>`

- <span id="debugframe-clone"></span>`fn clone(&self) -> DebugFrame<R>` — [`DebugFrame`](../index.md#debugframe)

##### `impl<R: marker::Copy + Reader> Copy for DebugFrame<R>`

##### `impl<R: fmt::Debug + Reader> Debug for DebugFrame<R>`

- <span id="debugframe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for DebugFrame<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for DebugFrame<R>`

- <span id="debugframe-partialeq-eq"></span>`fn eq(&self, other: &DebugFrame<R>) -> bool` — [`DebugFrame`](../index.md#debugframe)

##### `impl<R: Reader> Section for DebugFrame<R>`

- <span id="debugframe-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugframe-section-reader"></span>`fn reader(&self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for DebugFrame<R>`

##### `impl<R: Reader> UnwindSection for DebugFrame<R>`

- <span id="debugframe-unwindsection-type-offset"></span>`type Offset = DebugFrameOffset<<R as Reader>::Offset>`

### `EhFrameHdr<R: Reader>`

```rust
struct EhFrameHdr<R: Reader>(R);
```

`EhFrameHdr` contains the information about the `.eh_frame_hdr` section.

A pointer to the start of the `.eh_frame` data, and optionally, a binary
search table of pointers to the `.eh_frame` records that are found in this section.

#### Implementations

- <span id="ehframehdr-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Constructs a new `EhFrameHdr` instance from the data in the `.eh_frame_hdr` section.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for EhFrameHdr<R>`

- <span id="ehframehdr-clone"></span>`fn clone(&self) -> EhFrameHdr<R>` — [`EhFrameHdr`](../index.md#ehframehdr)

##### `impl<R: marker::Copy + Reader> Copy for EhFrameHdr<R>`

##### `impl<R: fmt::Debug + Reader> Debug for EhFrameHdr<R>`

- <span id="ehframehdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for EhFrameHdr<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for EhFrameHdr<R>`

- <span id="ehframehdr-partialeq-eq"></span>`fn eq(&self, other: &EhFrameHdr<R>) -> bool` — [`EhFrameHdr`](../index.md#ehframehdr)

##### `impl<R: Reader> Section for EhFrameHdr<R>`

- <span id="ehframehdr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="ehframehdr-section-reader"></span>`fn reader(&self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for EhFrameHdr<R>`

### `ParsedEhFrameHdr<R: Reader>`

```rust
struct ParsedEhFrameHdr<R: Reader> {
    address_size: u8,
    section: R,
    eh_frame_ptr: Pointer,
    fde_count: u64,
    table_enc: crate::constants::DwEhPe,
    table: R,
}
```

`ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section.

#### Implementations

- <span id="parsedehframehdr-eh-frame-ptr"></span>`fn eh_frame_ptr(&self) -> Pointer` — [`Pointer`](../index.md#pointer)

  Returns the address of the binary's `.eh_frame` section.

- <span id="parsedehframehdr-table"></span>`fn table(&self) -> Option<EhHdrTable<'_, R>>` — [`EhHdrTable`](../index.md#ehhdrtable)

  Retrieves the CFI binary search table, if there is one.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-clone"></span>`fn clone(&self) -> ParsedEhFrameHdr<R>` — [`ParsedEhFrameHdr`](../index.md#parsedehframehdr)

##### `impl<R: fmt::Debug + Reader> Debug for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EhHdrTableIter<'a, 'bases, R: Reader>`

```rust
struct EhHdrTableIter<'a, 'bases, R: Reader> {
    hdr: &'a ParsedEhFrameHdr<R>,
    table: R,
    bases: &'bases BaseAddresses,
    remain: u64,
}
```

An iterator for `.eh_frame_hdr` section's binary search table.

Each table entry consists of a tuple containing an  `initial_location` and `address`.
The `initial location` represents the first address that the targeted FDE
is able to decode. The `address` is the address of the FDE in the `.eh_frame` section.
The `address` can be converted with `EhHdrTable::pointer_to_offset` and `EhFrame::fde_from_offset` to an FDE.

#### Implementations

- <span id="ehhdrtableiter-next"></span>`fn next(&mut self) -> Result<Option<(Pointer, Pointer)>>` — [`Result`](../../index.md#result), [`Pointer`](../index.md#pointer)

  Yield the next entry in the `EhHdrTableIter`.

- <span id="ehhdrtableiter-nth"></span>`fn nth(&mut self, n: usize) -> Result<Option<(Pointer, Pointer)>>` — [`Result`](../../index.md#result), [`Pointer`](../index.md#pointer)

  Yield the nth entry in the `EhHdrTableIter`

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="ehhdrtableiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="ehhdrtableiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-iterator-type-item"></span>`type Item = Result<(Pointer, Pointer), Error>`

- <span id="ehhdrtableiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="ehhdrtableiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="ehhdrtableiter-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

### `EhHdrTable<'a, R: Reader>`

```rust
struct EhHdrTable<'a, R: Reader> {
    hdr: &'a ParsedEhFrameHdr<R>,
}
```

The CFI binary search table that is an optional part of the `.eh_frame_hdr` section.

#### Implementations

- <span id="ehhdrtable-iter"></span>`fn iter<'bases>(&self, bases: &'bases BaseAddresses) -> EhHdrTableIter<'_, 'bases, R>` — [`BaseAddresses`](../index.md#baseaddresses), [`EhHdrTableIter`](../index.md#ehhdrtableiter)

  Return an iterator that can walk the `.eh_frame_hdr` table.

  

  Each table entry consists of a tuple containing an `initial_location` and `address`.

  The `initial location` represents the first address that the targeted FDE

  is able to decode. The `address` is the address of the FDE in the `.eh_frame` section.

  The `address` can be converted with `EhHdrTable::pointer_to_offset` and `EhFrame::fde_from_offset` to an FDE.

- <span id="ehhdrtable-lookup"></span>`fn lookup(&self, address: u64, bases: &BaseAddresses) -> Result<Pointer>` — [`BaseAddresses`](../index.md#baseaddresses), [`Result`](../../index.md#result), [`Pointer`](../index.md#pointer)

  *Probably* returns a pointer to the FDE for the given address.

  

  This performs a binary search, so if there is no FDE for the given address,

  this function **will** return a pointer to any other FDE that's close by.

  

  To be sure, you **must** call `contains` on the FDE.

- <span id="ehhdrtable-pointer-to-offset"></span>`fn pointer_to_offset(&self, ptr: Pointer) -> Result<EhFrameOffset<<R as >::Offset>>` — [`Pointer`](../index.md#pointer), [`Result`](../../index.md#result), [`EhFrameOffset`](../../index.md#ehframeoffset), [`Reader`](../index.md#reader)

  Convert a `Pointer` to a section offset.

  

  This does not support indirect pointers.

- <span id="ehhdrtable-fde-for-address"></span>`fn fde_for_address<F>(&self, frame: &EhFrame<R>, bases: &BaseAddresses, address: u64, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`EhFrame`](../index.md#ehframe), [`BaseAddresses`](../index.md#baseaddresses), [`Result`](../../index.md#result), [`FrameDescriptionEntry`](../index.md#framedescriptionentry)

  Returns a parsed FDE for the given address, or `NoUnwindInfoForAddress`

  if there are none.

  

  You must provide a function to get its associated CIE. See

  `PartialFrameDescriptionEntry::parse` for more information.

  

  # Example

  

  ```rust

  use gimli::{BaseAddresses, EhFrame, ParsedEhFrameHdr, EndianSlice, NativeEndian, Error, UnwindSection};

  fn foo() -> Result<(), Error> {

  let eh_frame: EhFrame<EndianSlice<NativeEndian>> = unreachable!();

  let eh_frame_hdr: ParsedEhFrameHdr<EndianSlice<NativeEndian>> = unimplemented!();

  let addr = 0;

  let bases = unimplemented!();

  let table = eh_frame_hdr.table().unwrap();

  let fde = table.fde_for_address(&eh_frame, &bases, addr, EhFrame::cie_from_offset)?;

  Ok(())

  }

  ```

- <span id="ehhdrtable-unwind-info-for-address"></span>`fn unwind_info_for_address<'ctx, F, S>(&self, frame: &EhFrame<R>, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64, get_cie: F) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`EhFrame`](../index.md#ehframe), [`BaseAddresses`](../index.md#baseaddresses), [`UnwindContext`](../index.md#unwindcontext), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`UnwindTableRow`](../index.md#unwindtablerow)

  Returns the frame unwind information for the given address,

  or `NoUnwindInfoForAddress` if there are none.

  

  You must provide a function to get the associated CIE. See

  `PartialFrameDescriptionEntry::parse` for more information.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for EhHdrTable<'a, R>`

- <span id="ehhdrtable-clone"></span>`fn clone(&self) -> EhHdrTable<'a, R>` — [`EhHdrTable`](../index.md#ehhdrtable)

##### `impl<R: fmt::Debug + Reader> Debug for EhHdrTable<'a, R>`

- <span id="ehhdrtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EhFrame<R: Reader>`

```rust
struct EhFrame<R: Reader> {
    section: R,
    address_size: u8,
    vendor: crate::common::Vendor,
}
```

`EhFrame` contains the frame unwinding information needed during exception
handling found in the `.eh_frame` section.

Most interesting methods are defined in the
[`UnwindSection`](#unwindsection) trait.

See
[`DebugFrame`](./struct.DebugFrame.html#differences-between-debug_frame-and-eh_frame)
for some discussion on the differences between `.debug_frame` and
`.eh_frame`.

#### Implementations

- <span id="ehframe-set-address-size"></span>`fn set_address_size(&mut self, address_size: u8)`

  Set the size of a target address in bytes.

  

  This defaults to the native word size.

- <span id="ehframe-set-vendor"></span>`fn set_vendor(&mut self, vendor: Vendor)` — [`Vendor`](../../index.md#vendor)

  Set the vendor extensions to use.

  

  This defaults to `Vendor::Default`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for EhFrame<R>`

- <span id="ehframe-clone"></span>`fn clone(&self) -> EhFrame<R>` — [`EhFrame`](../index.md#ehframe)

##### `impl<R: marker::Copy + Reader> Copy for EhFrame<R>`

##### `impl<R: fmt::Debug + Reader> Debug for EhFrame<R>`

- <span id="ehframe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for EhFrame<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for EhFrame<R>`

- <span id="ehframe-partialeq-eq"></span>`fn eq(&self, other: &EhFrame<R>) -> bool` — [`EhFrame`](../index.md#ehframe)

##### `impl<R: Reader> Section for EhFrame<R>`

- <span id="ehframe-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="ehframe-section-reader"></span>`fn reader(&self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for EhFrame<R>`

##### `impl<R: Reader> UnwindSection for EhFrame<R>`

- <span id="ehframe-unwindsection-type-offset"></span>`type Offset = EhFrameOffset<<R as Reader>::Offset>`

### `BaseAddresses`

```rust
struct BaseAddresses {
    pub eh_frame_hdr: SectionBaseAddresses,
    pub eh_frame: SectionBaseAddresses,
}
```

Optional base addresses for the relative `DW_EH_PE_*` encoded pointers.

During CIE/FDE parsing, if a relative pointer is encountered for a base
address that is unknown, an Err will be returned.

```rust
use gimli::BaseAddresses;

fn foo() {
let address_of_eh_frame_hdr_section_in_memory = unimplemented!();
let address_of_eh_frame_section_in_memory = unimplemented!();
let address_of_text_section_in_memory = unimplemented!();
let address_of_got_section_in_memory = unimplemented!();
let address_of_the_start_of_current_func = unimplemented!();
let bases = BaseAddresses::default()
    .set_eh_frame_hdr(address_of_eh_frame_hdr_section_in_memory)
    .set_eh_frame(address_of_eh_frame_section_in_memory)
    .set_text(address_of_text_section_in_memory)
    .set_got(address_of_got_section_in_memory);
let _ = bases;
}
```

#### Fields

- **`eh_frame_hdr`**: `SectionBaseAddresses`

  The base addresses to use for pointers in the `.eh_frame_hdr` section.

- **`eh_frame`**: `SectionBaseAddresses`

  The base addresses to use for pointers in the `.eh_frame` section.

#### Implementations

- <span id="baseaddresses-set-eh-frame-hdr"></span>`fn set_eh_frame_hdr(self, addr: u64) -> Self`

  Set the `.eh_frame_hdr` section base address.

- <span id="baseaddresses-set-eh-frame"></span>`fn set_eh_frame(self, addr: u64) -> Self`

  Set the `.eh_frame` section base address.

- <span id="baseaddresses-set-text"></span>`fn set_text(self, addr: u64) -> Self`

  Set the `.text` section base address.

- <span id="baseaddresses-set-got"></span>`fn set_got(self, addr: u64) -> Self`

  Set the `.got` section base address.

#### Trait Implementations

##### `impl Clone for BaseAddresses`

- <span id="baseaddresses-clone"></span>`fn clone(&self) -> BaseAddresses` — [`BaseAddresses`](../index.md#baseaddresses)

##### `impl Debug for BaseAddresses`

- <span id="baseaddresses-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BaseAddresses`

- <span id="baseaddresses-default"></span>`fn default() -> BaseAddresses` — [`BaseAddresses`](../index.md#baseaddresses)

##### `impl Eq for BaseAddresses`

##### `impl PartialEq for BaseAddresses`

- <span id="baseaddresses-partialeq-eq"></span>`fn eq(&self, other: &BaseAddresses) -> bool` — [`BaseAddresses`](../index.md#baseaddresses)

##### `impl StructuralPartialEq for BaseAddresses`

### `SectionBaseAddresses`

```rust
struct SectionBaseAddresses {
    pub section: Option<u64>,
    pub text: Option<u64>,
    pub data: Option<u64>,
}
```

Optional base addresses for the relative `DW_EH_PE_*` encoded pointers
in a particular section.

See `BaseAddresses` for methods that are helpful in setting these addresses.

#### Fields

- **`section`**: `Option<u64>`

  The address of the section containing the pointer.

- **`text`**: `Option<u64>`

  The base address for text relative pointers.
  This is generally the address of the `.text` section.

- **`data`**: `Option<u64>`

  The base address for data relative pointers.
  
  For pointers in the `.eh_frame_hdr` section, this is the address
  of the `.eh_frame_hdr` section
  
  For pointers in the `.eh_frame` section, this is generally the
  global pointer, such as the address of the `.got` section.

#### Trait Implementations

##### `impl Clone for SectionBaseAddresses`

- <span id="sectionbaseaddresses-clone"></span>`fn clone(&self) -> SectionBaseAddresses` — [`SectionBaseAddresses`](../index.md#sectionbaseaddresses)

##### `impl Debug for SectionBaseAddresses`

- <span id="sectionbaseaddresses-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionBaseAddresses`

- <span id="sectionbaseaddresses-default"></span>`fn default() -> SectionBaseAddresses` — [`SectionBaseAddresses`](../index.md#sectionbaseaddresses)

##### `impl Eq for SectionBaseAddresses`

##### `impl PartialEq for SectionBaseAddresses`

- <span id="sectionbaseaddresses-partialeq-eq"></span>`fn eq(&self, other: &SectionBaseAddresses) -> bool` — [`SectionBaseAddresses`](../index.md#sectionbaseaddresses)

##### `impl StructuralPartialEq for SectionBaseAddresses`

### `CfiEntriesIter<'bases, Section, R>`

```rust
struct CfiEntriesIter<'bases, Section, R>
where
    R: Reader,
    Section: UnwindSection<R> {
    section: Section,
    bases: &'bases BaseAddresses,
    input: R,
}
```

An iterator over CIE and FDE entries in a `.debug_frame` or `.eh_frame`
section.

Some pointers may be encoded relative to various base addresses. Use the
[`BaseAddresses`](./struct.BaseAddresses.html) parameter to provide them. By
default, none are provided. If a relative pointer is encountered for a base
address that is unknown, an `Err` will be returned and iteration will abort.

```rust
use gimli::{BaseAddresses, EhFrame, EndianSlice, NativeEndian, UnwindSection};

fn foo() -> gimli::Result<()> {
let read_eh_frame_somehow = || unimplemented!();
let eh_frame = EhFrame::new(read_eh_frame_somehow(), NativeEndian);

let address_of_eh_frame_hdr_section_in_memory = unimplemented!();
let address_of_eh_frame_section_in_memory = unimplemented!();
let address_of_text_section_in_memory = unimplemented!();
let address_of_got_section_in_memory = unimplemented!();
let address_of_the_start_of_current_func = unimplemented!();
// Provide base addresses for relative pointers.
let bases = BaseAddresses::default()
    .set_eh_frame_hdr(address_of_eh_frame_hdr_section_in_memory)
    .set_eh_frame(address_of_eh_frame_section_in_memory)
    .set_text(address_of_text_section_in_memory)
    .set_got(address_of_got_section_in_memory);

let mut entries = eh_frame.entries(&bases);

let do_stuff_with = |_| unimplemented!();
while let Some(entry) = entries.next()? {
    do_stuff_with(entry)
}
unreachable!()
}
```

#### Implementations

- <span id="cfientriesiter-next"></span>`fn next(&mut self) -> Result<Option<CieOrFde<'bases, Section, R>>>` — [`Result`](../../index.md#result), [`CieOrFde`](../index.md#cieorfde)

  Advance the iterator to the next entry.

#### Trait Implementations

##### `impl<Section, R> Clone for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-clone"></span>`fn clone(&self) -> CfiEntriesIter<'bases, Section, R>` — [`CfiEntriesIter`](../index.md#cfientriesiter)

##### `impl<Section, R> Debug for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="cfientriesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="cfientriesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Section, R> Iterator for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-iterator-type-item"></span>`type Item = Result<CieOrFde<'bases, Section, R>, Error>`

- <span id="cfientriesiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `CfiEntryPrefix<R>`

```rust
struct CfiEntryPrefix<R>
where
    R: Reader {
    offset: <R as >::Offset,
    length: <R as >::Offset,
    format: crate::common::Format,
    cie_offset_base: <R as >::Offset,
    cie_id_or_offset: u64,
    rest: R,
}
```

The common prefix of a CIE or FDE.

#### Trait Implementations

##### `impl<R> Clone for CfiEntryPrefix<R>`

- <span id="cfientryprefix-clone"></span>`fn clone(&self) -> CfiEntryPrefix<R>` — [`CfiEntryPrefix`](#cfientryprefix)

##### `impl<R> Debug for CfiEntryPrefix<R>`

- <span id="cfientryprefix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Augmentation`

```rust
struct Augmentation {
    lsda: Option<constants::DwEhPe>,
    personality: Option<(constants::DwEhPe, Pointer)>,
    fde_address_encoding: Option<constants::DwEhPe>,
    is_signal_trampoline: bool,
}
```

We support the z-style augmentation [defined by `.eh_frame`][ehframe].


#### Fields

- **`lsda`**: `Option<constants::DwEhPe>`

  > A 'L' may be present at any position after the first character of the
  > string. This character may only be present if 'z' is the first character
  > of the string. If present, it indicates the presence of one argument in
  > the Augmentation Data of the CIE, and a corresponding argument in the
  > Augmentation Data of the FDE. The argument in the Augmentation Data of
  > the CIE is 1-byte and represents the pointer encoding used for the
  > argument in the Augmentation Data of the FDE, which is the address of a
  > language-specific data area (LSDA). The size of the LSDA pointer is
  > specified by the pointer encoding used.

- **`personality`**: `Option<(constants::DwEhPe, Pointer)>`

  > A 'P' may be present at any position after the first character of the
  > string. This character may only be present if 'z' is the first character
  > of the string. If present, it indicates the presence of two arguments in
  > the Augmentation Data of the CIE. The first argument is 1-byte and
  > represents the pointer encoding used for the second argument, which is
  > the address of a personality routine handler. The size of the
  > personality routine pointer is specified by the pointer encoding used.

- **`fde_address_encoding`**: `Option<constants::DwEhPe>`

  > A 'R' may be present at any position after the first character of the
  > string. This character may only be present if 'z' is the first character
  > of the string. If present, The Augmentation Data shall include a 1 byte
  > argument that represents the pointer encoding for the address pointers
  > used in the FDE.

- **`is_signal_trampoline`**: `bool`

  True if this CIE's FDEs are trampolines for signal handlers.

#### Implementations

- <span id="augmentation-parse"></span>`fn parse<Section, R>(augmentation_str: &mut R, bases: &BaseAddresses, address_size: u8, section: &Section, input: &mut R) -> Result<Augmentation>` — [`BaseAddresses`](../index.md#baseaddresses), [`Result`](../../index.md#result), [`Augmentation`](../index.md#augmentation)

#### Trait Implementations

##### `impl Clone for Augmentation`

- <span id="augmentation-clone"></span>`fn clone(&self) -> Augmentation` — [`Augmentation`](../index.md#augmentation)

##### `impl Copy for Augmentation`

##### `impl Debug for Augmentation`

- <span id="augmentation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Augmentation`

- <span id="augmentation-default"></span>`fn default() -> Augmentation` — [`Augmentation`](../index.md#augmentation)

##### `impl Eq for Augmentation`

##### `impl PartialEq for Augmentation`

- <span id="augmentation-partialeq-eq"></span>`fn eq(&self, other: &Augmentation) -> bool` — [`Augmentation`](../index.md#augmentation)

##### `impl StructuralPartialEq for Augmentation`

### `AugmentationData`

```rust
struct AugmentationData {
    lsda: Option<Pointer>,
}
```

Parsed augmentation data for a `FrameDescriptEntry`.

#### Implementations

- <span id="augmentationdata-parse"></span>`fn parse<R: Reader>(augmentation: &Augmentation, encoding_parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> Result<AugmentationData>` — [`Augmentation`](../index.md#augmentation), [`PointerEncodingParameters`](#pointerencodingparameters), [`Result`](../../index.md#result), [`AugmentationData`](#augmentationdata)

#### Trait Implementations

##### `impl Clone for AugmentationData`

- <span id="augmentationdata-clone"></span>`fn clone(&self) -> AugmentationData` — [`AugmentationData`](#augmentationdata)

##### `impl Debug for AugmentationData`

- <span id="augmentationdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AugmentationData`

- <span id="augmentationdata-default"></span>`fn default() -> AugmentationData` — [`AugmentationData`](#augmentationdata)

##### `impl Eq for AugmentationData`

##### `impl PartialEq for AugmentationData`

- <span id="augmentationdata-partialeq-eq"></span>`fn eq(&self, other: &AugmentationData) -> bool` — [`AugmentationData`](#augmentationdata)

##### `impl StructuralPartialEq for AugmentationData`

### `CommonInformationEntry<R, Offset>`

```rust
struct CommonInformationEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: Offset,
    length: Offset,
    format: crate::common::Format,
    version: u8,
    augmentation: Option<Augmentation>,
    address_size: u8,
    code_alignment_factor: u64,
    data_alignment_factor: i64,
    return_address_register: crate::common::Register,
    initial_instructions: R,
}
```

> A Common Information Entry holds information that is shared among many
> Frame Description Entries. There is at least one CIE in every non-empty
> `.debug_frame` section.

#### Fields

- **`offset`**: `Offset`

  The offset of this entry from the start of its containing section.

- **`length`**: `Offset`

  > A constant that gives the number of bytes of the CIE structure, not
  > including the length field itself (see Section 7.2.2). The size of the
  > length field plus the value of length must be an integral multiple of
  > the address size.

- **`version`**: `u8`

  > A version number (see Section 7.23). This number is specific to the
  > call frame information and is independent of the DWARF version number.

- **`augmentation`**: `Option<Augmentation>`

  The parsed augmentation, if any.

- **`address_size`**: `u8`

  > The size of a target address in this CIE and any FDEs that use it, in
  > bytes. If a compilation unit exists for this frame, its address size
  > must match the address size here.

- **`code_alignment_factor`**: `u64`

  "A constant that is factored out of all advance location instructions
  (see Section 6.4.2.1)."

- **`data_alignment_factor`**: `i64`

  > A constant that is factored out of certain offset instructions (see
  > below). The resulting value is (operand * data_alignment_factor).

- **`return_address_register`**: `crate::common::Register`

  > An unsigned LEB128 constant that indicates which column in the rule
  > table represents the return address of the function. Note that this
  > column might not correspond to an actual machine register.

- **`initial_instructions`**: `R`

  > A sequence of rules that are interpreted to create the initial setting
  > of each column in the table.
  
  > The default rule for all columns before interpretation of the initial
  > instructions is the undefined rule. However, an ABI authoring body or a
  > compilation system authoring body may specify an alternate default
  > value for any or all columns.
  
  This is followed by `DW_CFA_nop` padding until the end of `length` bytes
  in the input.

#### Implementations

- <span id="commoninformationentry-parse"></span>`fn parse<Section: UnwindSection<R>>(section: &Section, bases: &BaseAddresses, input: &mut R, offset: <R as >::Offset) -> Result<CommonInformationEntry<R>>` — [`BaseAddresses`](../index.md#baseaddresses), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`CommonInformationEntry`](../index.md#commoninformationentry)

- <span id="commoninformationentry-from-prefix"></span>`fn from_prefix<Section: UnwindSection<R>>(section: &Section, bases: &BaseAddresses, prefix: CfiEntryPrefix<R>) -> Result<CommonInformationEntry<R>>` — [`BaseAddresses`](../index.md#baseaddresses), [`CfiEntryPrefix`](#cfientryprefix), [`Result`](../../index.md#result), [`CommonInformationEntry`](../index.md#commoninformationentry)

#### Trait Implementations

##### `impl<R, Offset> Clone for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-clone"></span>`fn clone(&self) -> CommonInformationEntry<R, Offset>` — [`CommonInformationEntry`](../index.md#commoninformationentry)

##### `impl<R, Offset> Debug for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for CommonInformationEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-partialeq-eq"></span>`fn eq(&self, other: &CommonInformationEntry<R, Offset>) -> bool` — [`CommonInformationEntry`](../index.md#commoninformationentry)

##### `impl<R, Offset> StructuralPartialEq for CommonInformationEntry<R, Offset>`

### `PartialFrameDescriptionEntry<'bases, Section, R>`

```rust
struct PartialFrameDescriptionEntry<'bases, Section, R>
where
    R: Reader,
    Section: UnwindSection<R> {
    offset: <R as >::Offset,
    length: <R as >::Offset,
    format: crate::common::Format,
    cie_offset: <Section as >::Offset,
    rest: R,
    section: Section,
    bases: &'bases BaseAddresses,
}
```

A partially parsed `FrameDescriptionEntry`.

Fully parsing this FDE requires first parsing its CIE.

#### Implementations

- <span id="partialframedescriptionentry-parse-partial"></span>`fn parse_partial(section: &Section, bases: &'bases BaseAddresses, input: &mut R, offset: <R as >::Offset) -> Result<PartialFrameDescriptionEntry<'bases, Section, R>>` — [`BaseAddresses`](../index.md#baseaddresses), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`PartialFrameDescriptionEntry`](../index.md#partialframedescriptionentry)

- <span id="partialframedescriptionentry-from-prefix"></span>`fn from_prefix(section: &Section, bases: &'bases BaseAddresses, prefix: CfiEntryPrefix<R>) -> Result<PartialFrameDescriptionEntry<'bases, Section, R>>` — [`BaseAddresses`](../index.md#baseaddresses), [`CfiEntryPrefix`](#cfientryprefix), [`Result`](../../index.md#result), [`PartialFrameDescriptionEntry`](../index.md#partialframedescriptionentry)

- <span id="partialframedescriptionentry-parse"></span>`fn parse<F>(&self, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`Result`](../../index.md#result), [`FrameDescriptionEntry`](../index.md#framedescriptionentry)

  Fully parse this FDE.

  

  You must provide a function get its associated CIE (either by parsing it

  on demand, or looking it up in some table mapping offsets to CIEs that

  you've already parsed, etc.)

- <span id="partialframedescriptionentry-offset"></span>`fn offset(&self) -> <R as >::Offset` — [`Reader`](../index.md#reader)

  Get the offset of this entry from the start of its containing section.

- <span id="partialframedescriptionentry-cie-offset"></span>`fn cie_offset(&self) -> <Section as >::Offset` — [`UnwindSection`](../index.md#unwindsection)

  Get the offset of this FDE's CIE.

- <span id="partialframedescriptionentry-entry-len"></span>`fn entry_len(&self) -> <R as >::Offset` — [`Reader`](../index.md#reader)

  > A constant that gives the number of bytes of the header and

  > instruction stream for this function, not including the length field

  > itself (see Section 7.2.2). The size of the length field plus the value

  > of length must be an integral multiple of the address size.

#### Trait Implementations

##### `impl<Section, R> Clone for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-clone"></span>`fn clone(&self) -> PartialFrameDescriptionEntry<'bases, Section, R>` — [`PartialFrameDescriptionEntry`](../index.md#partialframedescriptionentry)

##### `impl<Section, R> Debug for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section, R> Eq for PartialFrameDescriptionEntry<'bases, Section, R>`

##### `impl<Section, R> PartialEq for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-partialeq-eq"></span>`fn eq(&self, other: &PartialFrameDescriptionEntry<'bases, Section, R>) -> bool` — [`PartialFrameDescriptionEntry`](../index.md#partialframedescriptionentry)

##### `impl<Section, R> StructuralPartialEq for PartialFrameDescriptionEntry<'bases, Section, R>`

### `FrameDescriptionEntry<R, Offset>`

```rust
struct FrameDescriptionEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: Offset,
    length: Offset,
    format: crate::common::Format,
    cie: CommonInformationEntry<R, Offset>,
    initial_address: u64,
    address_range: u64,
    augmentation: Option<AugmentationData>,
    instructions: R,
}
```

A `FrameDescriptionEntry` is a set of CFA instructions for an address range.

#### Fields

- **`offset`**: `Offset`

  The start of this entry within its containing section.

- **`length`**: `Offset`

  > A constant that gives the number of bytes of the header and
  > instruction stream for this function, not including the length field
  > itself (see Section 7.2.2). The size of the length field plus the value
  > of length must be an integral multiple of the address size.

- **`cie`**: `CommonInformationEntry<R, Offset>`

  "A constant offset into the .debug_frame section that denotes the CIE
  that is associated with this FDE."
  
  This is the CIE at that offset.

- **`initial_address`**: `u64`

  > The address of the first location associated with this table entry. If
  > the segment_size field of this FDE's CIE is non-zero, the initial
  > location is preceded by a segment selector of the given length.

- **`address_range`**: `u64`

  "The number of bytes of program instructions described by this entry."

- **`augmentation`**: `Option<AugmentationData>`

  The parsed augmentation data, if we have any.

- **`instructions`**: `R`

  "A sequence of table defining instructions that are described below."
  
  This is followed by `DW_CFA_nop` padding until `length` bytes of the
  input are consumed.

#### Implementations

- <span id="framedescriptionentry-parse-rest"></span>`fn parse_rest<Section, F>(offset: <R as >::Offset, length: <R as >::Offset, format: Format, cie_pointer: <Section as >::Offset, rest: R, section: &Section, bases: &BaseAddresses, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`Reader`](../index.md#reader), [`Format`](../../index.md#format), [`BaseAddresses`](../index.md#baseaddresses), [`Result`](../../index.md#result), [`FrameDescriptionEntry`](../index.md#framedescriptionentry)

- <span id="framedescriptionentry-parse-addresses"></span>`fn parse_addresses(input: &mut R, cie: &CommonInformationEntry<R>, parameters: &PointerEncodingParameters<'_, R>) -> Result<(u64, u64)>` — [`CommonInformationEntry`](../index.md#commoninformationentry), [`PointerEncodingParameters`](#pointerencodingparameters), [`Result`](../../index.md#result)

- <span id="framedescriptionentry-rows"></span>`fn rows<'a, 'ctx, Section, S>(&self, section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>) -> Result<UnwindTable<'a, 'ctx, R, S>>` — [`BaseAddresses`](../index.md#baseaddresses), [`UnwindContext`](../index.md#unwindcontext), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`UnwindTable`](../index.md#unwindtable)

  Return the table of unwind information for this FDE.

- <span id="framedescriptionentry-unwind-info-for-address"></span>`fn unwind_info_for_address<'ctx, Section, S>(&self, section: &Section, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`BaseAddresses`](../index.md#baseaddresses), [`UnwindContext`](../index.md#unwindcontext), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`UnwindTableRow`](../index.md#unwindtablerow)

  Find the frame unwind information for the given address.

  

  If found, the unwind information is returned along with the reset

  context in the form `Ok((unwind_info, context))`. If not found,

  `Err(gimli::Error::NoUnwindInfoForAddress)` is returned. If parsing or

  CFI evaluation fails, the error is returned.

#### Trait Implementations

##### `impl<R, Offset> Clone for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-clone"></span>`fn clone(&self) -> FrameDescriptionEntry<R, Offset>` — [`FrameDescriptionEntry`](../index.md#framedescriptionentry)

##### `impl<R, Offset> Debug for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for FrameDescriptionEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-partialeq-eq"></span>`fn eq(&self, other: &FrameDescriptionEntry<R, Offset>) -> bool` — [`FrameDescriptionEntry`](../index.md#framedescriptionentry)

##### `impl<R, Offset> StructuralPartialEq for FrameDescriptionEntry<R, Offset>`

### `UnwindContext<T, S>`

```rust
struct UnwindContext<T, S>
where
    T: ReaderOffset,
    S: UnwindContextStorage<T> {
    stack: super::util::ArrayVec<<S as >::Stack>,
    initial_rule: Option<Option<(crate::common::Register, RegisterRule<T>)>>,
    is_initialized: bool,
}
```

Common context needed when evaluating the call frame unwinding information.

By default, this structure is small and allocates its internal storage
on the heap using [`Box`](../../../allocator_api2/stable/boxed/index.md) during `UnwindContext::new`.

This can be overridden by providing a custom [`UnwindContextStorage`](../index.md) type parameter.
When using a custom storage with in-line arrays, the [`UnwindContext`](../index.md) type itself
will be big, so in that case it's recommended to place [`UnwindContext`](../index.md) on the
heap, e.g. using `Box::new(UnwindContext::<R, MyCustomStorage>::new_in())`.

To avoid re-allocating the context multiple times when evaluating multiple
CFI programs, the same [`UnwindContext`](../index.md) can be reused for multiple unwinds.

```rust
use gimli::{UnwindContext, UnwindTable};

fn foo<'a>(some_fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'a, gimli::LittleEndian>>)
           -> gimli::Result<()> {
let eh_frame: gimli::EhFrame<_> = unreachable!();
let bases = unimplemented!();
// An uninitialized context.
let mut ctx = UnwindContext::new();

// Initialize the context by evaluating the CIE's initial instruction program,
// and generate the unwind table.
let mut table = some_fde.rows(&eh_frame, &bases, &mut ctx)?;
while let Some(row) = table.next_row()? {
    // Do stuff with each row...
  let _ = row;
}
unreachable!()
}
```

#### Implementations

- <span id="unwindcontext-new"></span>`fn new() -> Self`

  Construct a new call frame unwinding context.

#### Trait Implementations

##### `impl<T, S> Clone for UnwindContext<T, S>`

- <span id="unwindcontext-clone"></span>`fn clone(&self) -> UnwindContext<T, S>` — [`UnwindContext`](../index.md#unwindcontext)

##### `impl<T, S> Debug for UnwindContext<T, S>`

- <span id="unwindcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for UnwindContext<T, S>`

- <span id="unwindcontext-default"></span>`fn default() -> Self`

##### `impl<T, S> Eq for UnwindContext<T, S>`

##### `impl<T, S> PartialEq for UnwindContext<T, S>`

- <span id="unwindcontext-partialeq-eq"></span>`fn eq(&self, other: &UnwindContext<T, S>) -> bool` — [`UnwindContext`](../index.md#unwindcontext)

##### `impl<T, S> StructuralPartialEq for UnwindContext<T, S>`

### `UnwindTable<'a, 'ctx, R, S>`

```rust
struct UnwindTable<'a, 'ctx, R, S>
where
    R: Reader,
    S: UnwindContextStorage<<R as >::Offset> {
    code_alignment_factor: core::num::Wrapping<u64>,
    data_alignment_factor: core::num::Wrapping<i64>,
    address_size: u8,
    next_start_address: u64,
    last_end_address: u64,
    returned_last_row: bool,
    current_row_valid: bool,
    instructions: CallFrameInstructionIter<'a, R>,
    ctx: &'ctx mut UnwindContext<<R as >::Offset, S>,
}
```

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

#### Implementations

- <span id="unwindtable-new"></span>`fn new<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Result<Self>` — [`BaseAddresses`](../index.md#baseaddresses), [`UnwindContext`](../index.md#unwindcontext), [`Reader`](../index.md#reader), [`FrameDescriptionEntry`](../index.md#framedescriptionentry), [`Result`](../../index.md#result)

  Construct a new `UnwindTable` for the given

  `FrameDescriptionEntry`'s CFI unwinding program.

- <span id="unwindtable-new-for-fde"></span>`fn new_for_fde<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Self` — [`BaseAddresses`](../index.md#baseaddresses), [`UnwindContext`](../index.md#unwindcontext), [`Reader`](../index.md#reader), [`FrameDescriptionEntry`](../index.md#framedescriptionentry)

- <span id="unwindtable-new-for-cie"></span>`fn new_for_cie<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, cie: &CommonInformationEntry<R>) -> Self` — [`BaseAddresses`](../index.md#baseaddresses), [`UnwindContext`](../index.md#unwindcontext), [`Reader`](../index.md#reader), [`CommonInformationEntry`](../index.md#commoninformationentry)

- <span id="unwindtable-next-row"></span>`fn next_row(&mut self) -> Result<Option<&UnwindTableRow<<R as >::Offset, S>>>` — [`Result`](../../index.md#result), [`UnwindTableRow`](../index.md#unwindtablerow), [`Reader`](../index.md#reader)

  Evaluate call frame instructions until the next row of the table is

  completed, and return it.

  

  Unfortunately, this cannot be used with `Iterator` because of

  the restricted lifetime of the yielded item.

- <span id="unwindtable-into-current-row"></span>`fn into_current_row(self) -> Option<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`UnwindTableRow`](../index.md#unwindtablerow), [`Reader`](../index.md#reader)

  Returns the current row with the lifetime of the context.

- <span id="unwindtable-evaluate"></span>`fn evaluate(&mut self, instruction: CallFrameInstruction<<R as >::Offset>) -> Result<bool>` — [`CallFrameInstruction`](../index.md#callframeinstruction), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

  Evaluate one call frame instruction. Return `Ok(true)` if the row is

  complete, `Ok(false)` otherwise.

#### Trait Implementations

##### `impl<R, S> Debug for UnwindTable<'a, 'ctx, R, S>`

- <span id="unwindtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RegisterRuleMap<T, S>`

```rust
struct RegisterRuleMap<T, S>
where
    T: ReaderOffset,
    S: UnwindContextStorage<T> {
    rules: super::util::ArrayVec<<S as >::Rules>,
}
```

#### Implementations

- <span id="registerrulemap-is-default"></span>`fn is_default(&self) -> bool`

- <span id="registerrulemap-get"></span>`fn get(&self, register: Register) -> Option<RegisterRule<T>>` — [`Register`](../../index.md#register), [`RegisterRule`](../index.md#registerrule)

- <span id="registerrulemap-clear"></span>`fn clear(&mut self, register: Register) -> Result<()>` — [`Register`](../../index.md#register), [`Result`](../../index.md#result)

- <span id="registerrulemap-set"></span>`fn set(&mut self, register: Register, rule: RegisterRule<T>) -> Result<()>` — [`Register`](../../index.md#register), [`RegisterRule`](../index.md#registerrule), [`Result`](../../index.md#result)

- <span id="registerrulemap-iter"></span>`fn iter(&self) -> RegisterRuleIter<'_, T>` — [`RegisterRuleIter`](../index.md#registerruleiter)

#### Trait Implementations

##### `impl<T, S> Clone for RegisterRuleMap<T, S>`

- <span id="registerrulemap-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S> Debug for RegisterRuleMap<T, S>`

- <span id="registerrulemap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for RegisterRuleMap<T, S>`

- <span id="registerrulemap-default"></span>`fn default() -> Self`

##### `impl<T, S> Eq for RegisterRuleMap<T, S>`

##### `impl<R, S> FromIterator for RegisterRuleMap<R, S>`

- <span id="registerrulemap-fromiterator-from-iter"></span>`fn from_iter<T>(iter: T) -> Self`

##### `impl<T, S> PartialEq for RegisterRuleMap<T, S>`

- <span id="registerrulemap-partialeq-eq"></span>`fn eq(&self, rhs: &Self) -> bool`

### `RegisterRuleIter<'iter, T>`

```rust
struct RegisterRuleIter<'iter, T>(::core::slice::Iter<'iter, (crate::common::Register, RegisterRule<T>)>)
where
    T: ReaderOffset;
```

An unordered iterator for register rules.

#### Trait Implementations

##### `impl<T> Clone for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-clone"></span>`fn clone(&self) -> RegisterRuleIter<'iter, T>` — [`RegisterRuleIter`](../index.md#registerruleiter)

##### `impl<T> Debug for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="registerruleiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="registerruleiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: ReaderOffset> Iterator for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-iterator-type-item"></span>`type Item = &'iter (Register, RegisterRule<T>)`

- <span id="registerruleiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `UnwindTableRow<T, S>`

```rust
struct UnwindTableRow<T, S>
where
    T: ReaderOffset,
    S: UnwindContextStorage<T> {
    start_address: u64,
    end_address: u64,
    saved_args_size: u64,
    cfa: CfaRule<T>,
    registers: RegisterRuleMap<T, S>,
}
```

A row in the virtual unwind table that describes how to find the values of
the registers in the *previous* frame for a range of PC addresses.

#### Implementations

- <span id="unwindtablerow-is-default"></span>`fn is_default(&self) -> bool`

- <span id="unwindtablerow-start-address"></span>`fn start_address(&self) -> u64`

  Get the starting PC address that this row applies to.

- <span id="unwindtablerow-end-address"></span>`fn end_address(&self) -> u64`

  Get the end PC address where this row's register rules become

  unapplicable.

  

  In other words, this row describes how to recover the last frame's

  registers for all PCs where `row.start_address() <= PC <

  row.end_address()`. This row does NOT describe how to recover registers

  when `PC == row.end_address()`.

- <span id="unwindtablerow-contains"></span>`fn contains(&self, address: u64) -> bool`

  Return `true` if the given `address` is within this row's address range,

  `false` otherwise.

- <span id="unwindtablerow-saved-args-size"></span>`fn saved_args_size(&self) -> u64`

  Returns the amount of args currently on the stack.

  

  When unwinding, if the personality function requested a change in IP,

  the SP needs to be adjusted by saved_args_size.

- <span id="unwindtablerow-cfa"></span>`fn cfa(&self) -> &CfaRule<T>` — [`CfaRule`](../index.md#cfarule)

  Get the canonical frame address (CFA) recovery rule for this row.

- <span id="unwindtablerow-register"></span>`fn register(&self, register: Register) -> Option<RegisterRule<T>>` — [`Register`](../../index.md#register), [`RegisterRule`](../index.md#registerrule)

  Get the register recovery rule for the given register number.

  

  Returns `None` if the register has the default rule, the value of which depends

  on the ABI or compilation system.

  

  The register number mapping is architecture dependent. For example, in

  the x86-64 ABI the register number mapping is defined in Figure 3.36:

  

  > Figure 3.36: DWARF Register Number Mapping

  >

  > <table>

  >   <tr><th>Register Name</th>                    <th>Number</th>  <th>Abbreviation</th></tr>

  >   <tr><td>General Purpose Register RAX</td>     <td>0</td>       <td>%rax</td></tr>

  >   <tr><td>General Purpose Register RDX</td>     <td>1</td>       <td>%rdx</td></tr>

  >   <tr><td>General Purpose Register RCX</td>     <td>2</td>       <td>%rcx</td></tr>

  >   <tr><td>General Purpose Register RBX</td>     <td>3</td>       <td>%rbx</td></tr>

  >   <tr><td>General Purpose Register RSI</td>     <td>4</td>       <td>%rsi</td></tr>

  >   <tr><td>General Purpose Register RDI</td>     <td>5</td>       <td>%rdi</td></tr>

  >   <tr><td>General Purpose Register RBP</td>     <td>6</td>       <td>%rbp</td></tr>

  >   <tr><td>Stack Pointer Register RSP</td>       <td>7</td>       <td>%rsp</td></tr>

  >   <tr><td>Extended Integer Registers 8-15</td>  <td>8-15</td>    <td>%r8-%r15</td></tr>

  >   <tr><td>Return Address RA</td>                <td>16</td>      <td></td></tr>

  >   <tr><td>Vector Registers 0–7</td>             <td>17-24</td>   <td>%xmm0–%xmm7</td></tr>

  >   <tr><td>Extended Vector Registers 8–15</td>   <td>25-32</td>   <td>%xmm8–%xmm15</td></tr>

  >   <tr><td>Floating Point Registers 0–7</td>     <td>33-40</td>   <td>%st0–%st7</td></tr>

  >   <tr><td>MMX Registers 0–7</td>                <td>41-48</td>   <td>%mm0–%mm7</td></tr>

  >   <tr><td>Flag Register</td>                    <td>49</td>      <td>%rFLAGS</td></tr>

  >   <tr><td>Segment Register ES</td>              <td>50</td>      <td>%es</td></tr>

  >   <tr><td>Segment Register CS</td>              <td>51</td>      <td>%cs</td></tr>

  >   <tr><td>Segment Register SS</td>              <td>52</td>      <td>%ss</td></tr>

  >   <tr><td>Segment Register DS</td>              <td>53</td>      <td>%ds</td></tr>

  >   <tr><td>Segment Register FS</td>              <td>54</td>      <td>%fs</td></tr>

  >   <tr><td>Segment Register GS</td>              <td>55</td>      <td>%gs</td></tr>

  >   <tr><td>Reserved</td>                         <td>56-57</td>   <td></td></tr>

  >   <tr><td>FS Base address</td>                  <td>58</td>      <td>%fs.base</td></tr>

  >   <tr><td>GS Base address</td>                  <td>59</td>      <td>%gs.base</td></tr>

  >   <tr><td>Reserved</td>                         <td>60-61</td>   <td></td></tr>

  >   <tr><td>Task Register</td>                    <td>62</td>      <td>%tr</td></tr>

  >   <tr><td>LDT Register</td>                     <td>63</td>      <td>%ldtr</td></tr>

  >   <tr><td>128-bit Media Control and Status</td> <td>64</td>      <td>%mxcsr</td></tr>

  >   <tr><td>x87 Control Word</td>                 <td>65</td>      <td>%fcw</td></tr>

  >   <tr><td>x87 Status Word</td>                  <td>66</td>      <td>%fsw</td></tr>

  >   <tr><td>Upper Vector Registers 16–31</td>     <td>67-82</td>   <td>%xmm16–%xmm31</td></tr>

  >   <tr><td>Reserved</td>                         <td>83-117</td>  <td></td></tr>

  >   <tr><td>Vector Mask Registers 0–7</td>        <td>118-125</td> <td>%k0–%k7</td></tr>

  >   <tr><td>Reserved</td>                         <td>126-129</td> <td></td></tr>

  > </table>

- <span id="unwindtablerow-registers"></span>`fn registers(&self) -> RegisterRuleIter<'_, T>` — [`RegisterRuleIter`](../index.md#registerruleiter)

  Iterate over all non-default register `(number, rule)` pairs.

  

  The rules are not iterated in any guaranteed order. Any register that

  does not make an appearance in the iterator implicitly has the default

  rule, the value of which depends on the ABI or compilation system.

  

  ```rust

  use gimli::{EndianSlice, LittleEndian, UnwindTableRow};

  fn foo<'input>(unwind_table_row: UnwindTableRow<usize>) {

  for &(register, ref rule) in unwind_table_row.registers() {

      // ...

      drop(register); drop(rule);

  }

  }

  ```

#### Trait Implementations

##### `impl<T, S> Clone for UnwindTableRow<T, S>`

- <span id="unwindtablerow-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S> Debug for UnwindTableRow<T, S>`

- <span id="unwindtablerow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for UnwindTableRow<T, S>`

- <span id="unwindtablerow-default"></span>`fn default() -> Self`

##### `impl<T, S> Eq for UnwindTableRow<T, S>`

##### `impl<T, S> PartialEq for UnwindTableRow<T, S>`

- <span id="unwindtablerow-partialeq-eq"></span>`fn eq(&self, other: &UnwindTableRow<T, S>) -> bool` — [`UnwindTableRow`](../index.md#unwindtablerow)

##### `impl<T, S> StructuralPartialEq for UnwindTableRow<T, S>`

### `CallFrameInstructionIter<'a, R: Reader>`

```rust
struct CallFrameInstructionIter<'a, R: Reader> {
    input: R,
    address_encoding: Option<constants::DwEhPe>,
    parameters: PointerEncodingParameters<'a, R>,
    vendor: crate::common::Vendor,
}
```

A lazy iterator parsing call frame instructions.

#### Implementations

- <span id="callframeinstructioniter-next"></span>`fn next(&mut self) -> Result<Option<CallFrameInstruction<<R as >::Offset>>>` — [`Result`](../../index.md#result), [`CallFrameInstruction`](../index.md#callframeinstruction), [`Reader`](../index.md#reader)

  Parse the next call frame instruction.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-clone"></span>`fn clone(&self) -> CallFrameInstructionIter<'a, R>` — [`CallFrameInstructionIter`](../index.md#callframeinstructioniter)

##### `impl<R: fmt::Debug + Reader> Debug for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="callframeinstructioniter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="callframeinstructioniter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-iterator-type-item"></span>`type Item = Result<CallFrameInstruction<<R as Reader>::Offset>, Error>`

- <span id="callframeinstructioniter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `UnwindExpression<T: ReaderOffset>`

```rust
struct UnwindExpression<T: ReaderOffset> {
    pub offset: T,
    pub length: T,
}
```

The location of a DWARF expression within an unwind section.

This is stored as an offset and length within the section instead of as a
`Reader` to avoid lifetime issues when reusing [`UnwindContext`](../index.md).

# Example
```rust
use gimli::{EhFrame, EndianSlice, NativeEndian, Error, FrameDescriptionEntry, UnwindExpression, EvaluationResult};
fn foo() -> Result<(), Error> {
let eh_frame: EhFrame<EndianSlice<NativeEndian>> = unreachable!();
let fde: FrameDescriptionEntry<EndianSlice<NativeEndian>> = unimplemented!();
let unwind_expression: UnwindExpression<_> = unimplemented!();
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
Ok(())
}
```

#### Fields

- **`offset`**: `T`

  The offset of the expression within the section.

- **`length`**: `T`

  The length of the expression.

#### Implementations

- <span id="unwindexpression-get"></span>`fn get<R, S>(&self, section: &S) -> Result<Expression<R>>` — [`Result`](../../index.md#result), [`Expression`](../index.md#expression)

  Get the expression from the section.

  

  The offset and length were previously validated when the

  `UnwindExpression` was created, so this should not fail.

#### Trait Implementations

##### `impl<T: clone::Clone + ReaderOffset> Clone for UnwindExpression<T>`

- <span id="unwindexpression-clone"></span>`fn clone(&self) -> UnwindExpression<T>` — [`UnwindExpression`](../index.md#unwindexpression)

##### `impl<T: marker::Copy + ReaderOffset> Copy for UnwindExpression<T>`

##### `impl<T: fmt::Debug + ReaderOffset> Debug for UnwindExpression<T>`

- <span id="unwindexpression-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for UnwindExpression<T>`

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for UnwindExpression<T>`

- <span id="unwindexpression-partialeq-eq"></span>`fn eq(&self, other: &UnwindExpression<T>) -> bool` — [`UnwindExpression`](../index.md#unwindexpression)

##### `impl<T: ReaderOffset> StructuralPartialEq for UnwindExpression<T>`

### `PointerEncodingParameters<'a, R: Reader>`

```rust
struct PointerEncodingParameters<'a, R: Reader> {
    bases: &'a SectionBaseAddresses,
    func_base: Option<u64>,
    address_size: u8,
    section: &'a R,
}
```

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-clone"></span>`fn clone(&self) -> PointerEncodingParameters<'a, R>` — [`PointerEncodingParameters`](#pointerencodingparameters)

##### `impl<R: fmt::Debug + Reader> Debug for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `CieOrFde<'bases, Section, R>`

```rust
enum CieOrFde<'bases, Section, R>
where
    R: Reader,
    Section: UnwindSection<R> {
    Cie(CommonInformationEntry<R>),
    Fde(PartialFrameDescriptionEntry<'bases, Section, R>),
}
```

Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE).

#### Variants

- **`Cie`**

  This CFI entry is a `CommonInformationEntry`.

- **`Fde`**

  This CFI entry is a `FrameDescriptionEntry`, however fully parsing it
  requires parsing its CIE first, so it is left in a partially parsed
  state.

#### Trait Implementations

##### `impl<Section, R> Clone for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-clone"></span>`fn clone(&self) -> CieOrFde<'bases, Section, R>` — [`CieOrFde`](../index.md#cieorfde)

##### `impl<Section, R> Debug for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section, R> Eq for CieOrFde<'bases, Section, R>`

##### `impl<Section, R> PartialEq for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-partialeq-eq"></span>`fn eq(&self, other: &CieOrFde<'bases, Section, R>) -> bool` — [`CieOrFde`](../index.md#cieorfde)

##### `impl<Section, R> StructuralPartialEq for CieOrFde<'bases, Section, R>`

### `CfaRule<T: ReaderOffset>`

```rust
enum CfaRule<T: ReaderOffset> {
    RegisterAndOffset {
        register: crate::common::Register,
        offset: i64,
    },
    Expression(UnwindExpression<T>),
}
```

The canonical frame address (CFA) recovery rules.

#### Variants

- **`RegisterAndOffset`**

  The CFA is given offset from the given register's value.

- **`Expression`**

  The CFA is obtained by evaluating a DWARF expression program.

#### Implementations

- <span id="cfarule-is-default"></span>`fn is_default(&self) -> bool`

#### Trait Implementations

##### `impl<T: clone::Clone + ReaderOffset> Clone for CfaRule<T>`

- <span id="cfarule-clone"></span>`fn clone(&self) -> CfaRule<T>` — [`CfaRule`](../index.md#cfarule)

##### `impl<T: fmt::Debug + ReaderOffset> Debug for CfaRule<T>`

- <span id="cfarule-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ReaderOffset> Default for CfaRule<T>`

- <span id="cfarule-default"></span>`fn default() -> Self`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for CfaRule<T>`

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for CfaRule<T>`

- <span id="cfarule-partialeq-eq"></span>`fn eq(&self, other: &CfaRule<T>) -> bool` — [`CfaRule`](../index.md#cfarule)

##### `impl<T: ReaderOffset> StructuralPartialEq for CfaRule<T>`

### `RegisterRule<T: ReaderOffset>`

```rust
enum RegisterRule<T: ReaderOffset> {
    Undefined,
    SameValue,
    Offset(i64),
    ValOffset(i64),
    Register(crate::common::Register),
    Expression(UnwindExpression<T>),
    ValExpression(UnwindExpression<T>),
    Architectural,
    Constant(u64),
}
```

An entry in the abstract CFI table that describes how to find the value of a
register.

"The register columns contain rules that describe whether a given register
has been saved and the rule to find the value for the register in the
previous frame."

#### Variants

- **`Undefined`**

  > A register that has this rule has no recoverable value in the previous
  > frame. (By convention, it is not preserved by a callee.)

- **`SameValue`**

  > This register has not been modified from the previous frame. (By
  > convention, it is preserved by the callee, but the callee has not
  > modified it.)

- **`Offset`**

  "The previous value of this register is saved at the address CFA+N where
  CFA is the current CFA value and N is a signed offset."

- **`ValOffset`**

  "The previous value of this register is the value CFA+N where CFA is the
  current CFA value and N is a signed offset."

- **`Register`**

  "The previous value of this register is stored in another register
  numbered R."

- **`Expression`**

  "The previous value of this register is located at the address produced
  by executing the DWARF expression."

- **`ValExpression`**

  "The previous value of this register is the value produced by executing
  the DWARF expression."

- **`Architectural`**

  "The rule is defined externally to this specification by the augmenter."

- **`Constant`**

  This is a pseudo-register with a constant value.

#### Implementations

- <span id="registerrule-is-defined"></span>`fn is_defined(&self) -> bool`

#### Trait Implementations

##### `impl<T: clone::Clone + ReaderOffset> Clone for RegisterRule<T>`

- <span id="registerrule-clone"></span>`fn clone(&self) -> RegisterRule<T>` — [`RegisterRule`](../index.md#registerrule)

##### `impl<T: fmt::Debug + ReaderOffset> Debug for RegisterRule<T>`

- <span id="registerrule-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for RegisterRule<T>`

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for RegisterRule<T>`

- <span id="registerrule-partialeq-eq"></span>`fn eq(&self, other: &RegisterRule<T>) -> bool` — [`RegisterRule`](../index.md#registerrule)

##### `impl<T: ReaderOffset> StructuralPartialEq for RegisterRule<T>`

### `CallFrameInstruction<T: ReaderOffset>`

```rust
enum CallFrameInstruction<T: ReaderOffset> {
    SetLoc {
        address: u64,
    },
    AdvanceLoc {
        delta: u32,
    },
    DefCfa {
        register: crate::common::Register,
        offset: u64,
    },
    DefCfaSf {
        register: crate::common::Register,
        factored_offset: i64,
    },
    DefCfaRegister {
        register: crate::common::Register,
    },
    DefCfaOffset {
        offset: u64,
    },
    DefCfaOffsetSf {
        factored_offset: i64,
    },
    DefCfaExpression {
        expression: UnwindExpression<T>,
    },
    Undefined {
        register: crate::common::Register,
    },
    SameValue {
        register: crate::common::Register,
    },
    Offset {
        register: crate::common::Register,
        factored_offset: u64,
    },
    OffsetExtendedSf {
        register: crate::common::Register,
        factored_offset: i64,
    },
    ValOffset {
        register: crate::common::Register,
        factored_offset: u64,
    },
    ValOffsetSf {
        register: crate::common::Register,
        factored_offset: i64,
    },
    Register {
        dest_register: crate::common::Register,
        src_register: crate::common::Register,
    },
    Expression {
        register: crate::common::Register,
        expression: UnwindExpression<T>,
    },
    ValExpression {
        register: crate::common::Register,
        expression: UnwindExpression<T>,
    },
    Restore {
        register: crate::common::Register,
    },
    RememberState,
    RestoreState,
    ArgsSize {
        size: u64,
    },
    NegateRaState,
    Nop,
}
```

A parsed call frame instruction.

#### Variants

- **`SetLoc`**

  > 1. DW_CFA_set_loc
  >
  > The DW_CFA_set_loc instruction takes a single operand that represents
  > a target address. The required action is to create a new table row
  > using the specified address as the location. All other values in the
  > new row are initially identical to the current row. The new location
  > value is always greater than the current one. If the segment_size
  > field of this FDE's CIE is non- zero, the initial location is preceded
  > by a segment selector of the given length.

- **`AdvanceLoc`**

  The `AdvanceLoc` instruction is used for all of `DW_CFA_advance_loc` and
  `DW_CFA_advance_loc{1,2,4}`.
  
  > 2. DW_CFA_advance_loc
  >
  > The DW_CFA_advance instruction takes a single operand (encoded with
  > the opcode) that represents a constant delta. The required action is
  > to create a new table row with a location value that is computed by
  > taking the current entry’s location value and adding the value of
  > delta * code_alignment_factor. All other values in the new row are
  > initially identical to the current row.

- **`DefCfa`**

  > 1. DW_CFA_def_cfa
  >
  > The DW_CFA_def_cfa instruction takes two unsigned LEB128 operands
  > representing a register number and a (non-factored) offset. The
  > required action is to define the current CFA rule to use the provided
  > register and offset.

- **`DefCfaSf`**

  > 2. DW_CFA_def_cfa_sf
  >
  > The DW_CFA_def_cfa_sf instruction takes two operands: an unsigned
  > LEB128 value representing a register number and a signed LEB128
  > factored offset. This instruction is identical to DW_CFA_def_cfa
  > except that the second operand is signed and factored. The resulting
  > offset is factored_offset * data_alignment_factor.

- **`DefCfaRegister`**

  > 3. DW_CFA_def_cfa_register
  >
  > The DW_CFA_def_cfa_register instruction takes a single unsigned LEB128
  > operand representing a register number. The required action is to
  > define the current CFA rule to use the provided register (but to keep
  > the old offset). This operation is valid only if the current CFA rule
  > is defined to use a register and offset.

- **`DefCfaOffset`**

  > 4. DW_CFA_def_cfa_offset
  >
  > The DW_CFA_def_cfa_offset instruction takes a single unsigned LEB128
  > operand representing a (non-factored) offset. The required action is
  > to define the current CFA rule to use the provided offset (but to keep
  > the old register). This operation is valid only if the current CFA
  > rule is defined to use a register and offset.

- **`DefCfaOffsetSf`**

  > 5. DW_CFA_def_cfa_offset_sf
  >
  > The DW_CFA_def_cfa_offset_sf instruction takes a signed LEB128 operand
  > representing a factored offset. This instruction is identical to
  > DW_CFA_def_cfa_offset except that the operand is signed and
  > factored. The resulting offset is factored_offset *
  > data_alignment_factor. This operation is valid only if the current CFA
  > rule is defined to use a register and offset.

- **`DefCfaExpression`**

  > 6. DW_CFA_def_cfa_expression
  >
  > The DW_CFA_def_cfa_expression instruction takes a single operand
  > encoded as a DW_FORM_exprloc value representing a DWARF
  > expression. The required action is to establish that expression as the
  > means by which the current CFA is computed.

- **`Undefined`**

  > 1. DW_CFA_undefined
  >
  > The DW_CFA_undefined instruction takes a single unsigned LEB128
  > operand that represents a register number. The required action is to
  > set the rule for the specified register to “undefined.”

- **`SameValue`**

  > 2. DW_CFA_same_value
  >
  > The DW_CFA_same_value instruction takes a single unsigned LEB128
  > operand that represents a register number. The required action is to
  > set the rule for the specified register to “same value.”

- **`Offset`**

  The `Offset` instruction represents both `DW_CFA_offset` and
  `DW_CFA_offset_extended`.
  
  > 3. DW_CFA_offset
  >
  > The DW_CFA_offset instruction takes two operands: a register number
  > (encoded with the opcode) and an unsigned LEB128 constant representing
  > a factored offset. The required action is to change the rule for the
  > register indicated by the register number to be an offset(N) rule
  > where the value of N is factored offset * data_alignment_factor.

- **`OffsetExtendedSf`**

  > 5. DW_CFA_offset_extended_sf
  >
  > The DW_CFA_offset_extended_sf instruction takes two operands: an
  > unsigned LEB128 value representing a register number and a signed
  > LEB128 factored offset. This instruction is identical to
  > DW_CFA_offset_extended except that the second operand is signed and
  > factored. The resulting offset is factored_offset *
  > data_alignment_factor.

- **`ValOffset`**

  > 6. DW_CFA_val_offset
  >
  > The DW_CFA_val_offset instruction takes two unsigned LEB128 operands
  > representing a register number and a factored offset. The required
  > action is to change the rule for the register indicated by the
  > register number to be a val_offset(N) rule where the value of N is
  > factored_offset * data_alignment_factor.

- **`ValOffsetSf`**

  > 7. DW_CFA_val_offset_sf
  >
  > The DW_CFA_val_offset_sf instruction takes two operands: an unsigned
  > LEB128 value representing a register number and a signed LEB128
  > factored offset. This instruction is identical to DW_CFA_val_offset
  > except that the second operand is signed and factored. The resulting
  > offset is factored_offset * data_alignment_factor.

- **`Register`**

  > 8. DW_CFA_register
  >
  > The DW_CFA_register instruction takes two unsigned LEB128 operands
  > representing register numbers. The required action is to set the rule
  > for the first register to be register(R) where R is the second
  > register.

- **`Expression`**

  > 9. DW_CFA_expression
  >
  > The DW_CFA_expression instruction takes two operands: an unsigned
  > LEB128 value representing a register number, and a DW_FORM_block value
  > representing a DWARF expression. The required action is to change the
  > rule for the register indicated by the register number to be an
  > expression(E) rule where E is the DWARF expression. That is, the DWARF
  > expression computes the address. The value of the CFA is pushed on the
  > DWARF evaluation stack prior to execution of the DWARF expression.

- **`ValExpression`**

  > 10. DW_CFA_val_expression
  >
  > The DW_CFA_val_expression instruction takes two operands: an unsigned
  > LEB128 value representing a register number, and a DW_FORM_block value
  > representing a DWARF expression. The required action is to change the
  > rule for the register indicated by the register number to be a
  > val_expression(E) rule where E is the DWARF expression. That is, the
  > DWARF expression computes the value of the given register. The value
  > of the CFA is pushed on the DWARF evaluation stack prior to execution
  > of the DWARF expression.

- **`Restore`**

  The `Restore` instruction represents both `DW_CFA_restore` and
  `DW_CFA_restore_extended`.
  
  > 11. DW_CFA_restore
  >
  > The DW_CFA_restore instruction takes a single operand (encoded with
  > the opcode) that represents a register number. The required action is
  > to change the rule for the indicated register to the rule assigned it
  > by the initial_instructions in the CIE.

- **`RememberState`**

  > 1. DW_CFA_remember_state
  >
  > The DW_CFA_remember_state instruction takes no operands. The required
  > action is to push the set of rules for every register onto an implicit
  > stack.

- **`RestoreState`**

  > 2. DW_CFA_restore_state
  >
  > The DW_CFA_restore_state instruction takes no operands. The required
  > action is to pop the set of rules off the implicit stack and place
  > them in the current row.

- **`ArgsSize`**

  > DW_CFA_GNU_args_size
  >
  > GNU Extension
  >
  > The DW_CFA_GNU_args_size instruction takes an unsigned LEB128 operand
  > representing an argument size. This instruction specifies the total of
  > the size of the arguments which have been pushed onto the stack.

- **`NegateRaState`**

  > DW_CFA_AARCH64_negate_ra_state
  >
  > AArch64 Extension
  >
  > The DW_CFA_AARCH64_negate_ra_state operation negates bit 0 of the
  > RA_SIGN_STATE pseudo-register. It does not take any operands. The
  > DW_CFA_AARCH64_negate_ra_state must not be mixed with other DWARF Register
  > Rule Instructions on the RA_SIGN_STATE pseudo-register in one Common
  > Information Entry (CIE) and Frame Descriptor Entry (FDE) program sequence.

- **`Nop`**

  > 1. DW_CFA_nop
  >
  > The DW_CFA_nop instruction has no operands and no required actions. It
  > is used as padding to make a CIE or FDE an appropriate size.

#### Implementations

- <span id="callframeinstruction-parse"></span>`fn parse<R: Reader<Offset = T>>(input: &mut R, address_encoding: Option<DwEhPe>, parameters: &PointerEncodingParameters<'_, R>, vendor: Vendor) -> Result<CallFrameInstruction<T>>` — [`DwEhPe`](../../index.md#dwehpe), [`PointerEncodingParameters`](#pointerencodingparameters), [`Vendor`](../../index.md#vendor), [`Result`](../../index.md#result), [`CallFrameInstruction`](../index.md#callframeinstruction)

#### Trait Implementations

##### `impl<T: clone::Clone + ReaderOffset> Clone for CallFrameInstruction<T>`

- <span id="callframeinstruction-clone"></span>`fn clone(&self) -> CallFrameInstruction<T>` — [`CallFrameInstruction`](../index.md#callframeinstruction)

##### `impl<T: fmt::Debug + ReaderOffset> Debug for CallFrameInstruction<T>`

- <span id="callframeinstruction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for CallFrameInstruction<T>`

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for CallFrameInstruction<T>`

- <span id="callframeinstruction-partialeq-eq"></span>`fn eq(&self, other: &CallFrameInstruction<T>) -> bool` — [`CallFrameInstruction`](../index.md#callframeinstruction)

##### `impl<T: ReaderOffset> StructuralPartialEq for CallFrameInstruction<T>`

### `Pointer`

```rust
enum Pointer {
    Direct(u64),
    Indirect(u64),
}
```

A decoded pointer.

#### Variants

- **`Direct`**

  This value is the decoded pointer value.

- **`Indirect`**

  This value is *not* the pointer value, but points to the address of
  where the real pointer value lives. In other words, deref this pointer
  to get the real pointer value.
  
  Chase this pointer at your own risk: do you trust the DWARF data it came
  from?

#### Implementations

- <span id="pointer-new"></span>`fn new(encoding: constants::DwEhPe, address: u64) -> Pointer` — [`DwEhPe`](../../index.md#dwehpe), [`Pointer`](../index.md#pointer)

- <span id="pointer-direct"></span>`fn direct(self) -> Result<u64>` — [`Result`](../../index.md#result)

  Return the direct pointer value.

- <span id="pointer-pointer"></span>`fn pointer(self) -> u64`

  Return the pointer value, discarding indirectness information.

#### Trait Implementations

##### `impl Clone for Pointer`

- <span id="pointer-clone"></span>`fn clone(&self) -> Pointer` — [`Pointer`](../index.md#pointer)

##### `impl Copy for Pointer`

##### `impl Debug for Pointer`

- <span id="pointer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Pointer`

- <span id="pointer-default"></span>`fn default() -> Self`

##### `impl Eq for Pointer`

##### `impl PartialEq for Pointer`

- <span id="pointer-partialeq-eq"></span>`fn eq(&self, other: &Pointer) -> bool` — [`Pointer`](../index.md#pointer)

##### `impl StructuralPartialEq for Pointer`

## Traits

### `UnwindOffset<T>`

```rust
trait UnwindOffset<T>: Copy + Debug + Eq + From<T>
where
    T: ReaderOffset { ... }
```

An offset into an `UnwindSection`.

#### Required Methods

- `fn into(self) -> T`

  Convert an `UnwindOffset<T>` into a `T`.

#### Implementors

- [`DebugFrameOffset`](../../index.md#debugframeoffset)
- [`EhFrameOffset`](../../index.md#ehframeoffset)

### `UnwindSection<R: Reader>`

```rust
trait UnwindSection<R: Reader>: Clone + Debug + _UnwindSectionPrivate<R> { ... }
```

A section holding unwind information: either `.debug_frame` or
`.eh_frame`. See [`DebugFrame`](./struct.DebugFrame.html) and
[`EhFrame`](./struct.EhFrame.html) respectively.

#### Associated Types

- `type Offset: 1`

#### Provided Methods

- `fn entries<'bases>(&self, bases: &'bases BaseAddresses) -> CfiEntriesIter<'bases, Self, R>`

  Iterate over the `CommonInformationEntry`s and `FrameDescriptionEntry`s

- `fn cie_from_offset(&self, bases: &BaseAddresses, offset: <Self as >::Offset) -> Result<CommonInformationEntry<R>>`

  Parse the `CommonInformationEntry` at the given offset.

- `fn partial_fde_from_offset<'bases>(&self, bases: &'bases BaseAddresses, offset: <Self as >::Offset) -> Result<PartialFrameDescriptionEntry<'bases, Self, R>>`

  Parse the `PartialFrameDescriptionEntry` at the given offset.

- `fn fde_from_offset<F>(&self, bases: &BaseAddresses, offset: <Self as >::Offset, get_cie: F) -> Result<FrameDescriptionEntry<R>>`

  Parse the `FrameDescriptionEntry` at the given offset.

- `fn fde_for_address<F>(&self, bases: &BaseAddresses, address: u64, get_cie: F) -> Result<FrameDescriptionEntry<R>>`

  Find the `FrameDescriptionEntry` for the given address.

- `fn unwind_info_for_address<'ctx, F, S>(&self, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64, get_cie: F) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>`

  Find the frame unwind information for the given address.

#### Implementors

- [`DebugFrame`](../index.md#debugframe)
- [`EhFrame`](../index.md#ehframe)

### `UnwindContextStorage<T: ReaderOffset>`

```rust
trait UnwindContextStorage<T: ReaderOffset>: Sized { ... }
```

Specification of what storage should be used for [`UnwindContext`](../index.md).

Normally you would only need to use [`StoreOnHeap`](../../index.md), which places the stack
on the heap using [`Box`](../../../allocator_api2/stable/boxed/index.md). This is the default storage type parameter for [`UnwindContext`](../index.md).

You may want to supply your own storage type for one of the following reasons:

  1. In rare cases you may run into failed unwinds due to the fixed stack size
     used by [`StoreOnHeap`](../../index.md), so you may want to try a larger `Box`. If denial
     of service is not a concern, then you could also try a `Vec`-based stack which
     can grow as needed.
  2. You may want to avoid heap allocations entirely. You can use a fixed-size
     stack with in-line arrays, which will place the entire storage in-line into
     [`UnwindContext`](../index.md).

Here's an implementation which uses a fixed-size stack and allocates everything in-line,
which will cause `UnwindContext` to be large:

```rust,no_run
use gimli::*;

fn foo<'a>(some_fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'a, gimli::LittleEndian>>)
           -> gimli::Result<()> {
let eh_frame: gimli::EhFrame<_> = unreachable!();
let bases = unimplemented!();

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
  let _ = row;
}
unreachable!()
}
```

#### Associated Types

- `type Rules: 1`

- `type Stack: 1`

#### Implementors

- [`StoreOnHeap`](../../index.md#storeonheap)

## Functions

### `parse_cfi_entry`

```rust
fn parse_cfi_entry<'bases, Section, R>(section: &Section, bases: &'bases BaseAddresses, input: &mut R) -> crate::read::Result<Option<CieOrFde<'bases, Section, R>>>
where
    R: Reader,
    Section: UnwindSection<R>
```

### `parse_cfi_entry_prefix`

```rust
fn parse_cfi_entry_prefix<Section, R>(section: &Section, input: &mut R) -> crate::read::Result<Option<CfiEntryPrefix<R>>>
where
    R: Reader,
    Section: UnwindSection<R>
```

### `parse_encoded_pointer`

```rust
fn parse_encoded_pointer<R: Reader>(encoding: constants::DwEhPe, parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> crate::read::Result<Pointer>
```

### `parse_encoded_value`

```rust
fn parse_encoded_value<R: Reader>(encoding: constants::DwEhPe, parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> crate::read::Result<u64>
```

## Constants

### `MAX_RULES`
```rust
const MAX_RULES: usize = 192usize;
```

### `MAX_UNWIND_STACK_DEPTH`
```rust
const MAX_UNWIND_STACK_DEPTH: usize = 4usize;
```

### `CFI_INSTRUCTION_HIGH_BITS_MASK`
```rust
const CFI_INSTRUCTION_HIGH_BITS_MASK: u8 = 192u8;
```

### `CFI_INSTRUCTION_LOW_BITS_MASK`
```rust
const CFI_INSTRUCTION_LOW_BITS_MASK: u8 = 63u8;
```

