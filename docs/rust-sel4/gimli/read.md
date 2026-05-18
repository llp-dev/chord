**gimli > read**

# Module: read

## Contents

**Structs**

- [`StoreOnHeap`](#storeonheap) - Indicates that storage should be allocated on heap.
- [`UnitOffset`](#unitoffset) - An offset into the current compilation or type unit.

**Enums**

- [`Error`](#error) - An error that occurred when parsing.

**Traits**

- [`Section`](#section) - A convenience trait for loading DWARF sections from object files.  To be

**Type Aliases**

- [`EndianBuf`](#endianbuf) - `EndianBuf` has been renamed to `EndianSlice`. For ease of upgrading across
- [`Result`](#result) - The result of a parse.

---

## gimli::read::EndianBuf

*Type Alias*: `EndianSlice<'input, Endian>`

`EndianBuf` has been renamed to `EndianSlice`. For ease of upgrading across
`gimli` versions, we export this type alias.



## gimli::read::Error

*Enum*

An error that occurred when parsing.

**Variants:**
- `Io` - An I/O error occurred while reading.
- `PcRelativePointerButSectionBaseIsUndefined` - Found a PC relative pointer, but the section base is undefined.
- `TextRelativePointerButTextBaseIsUndefined` - Found a `.text` relative pointer, but the `.text` base is undefined.
- `DataRelativePointerButDataBaseIsUndefined` - Found a data relative pointer, but the data base is undefined.
- `FuncRelativePointerInBadContext` - Found a function relative pointer in a context that does not have a
- `CannotParseOmitPointerEncoding` - Cannot parse a pointer with a `DW_EH_PE_omit` encoding.
- `BadUnsignedLeb128` - An error parsing an unsigned LEB128 value.
- `BadSignedLeb128` - An error parsing a signed LEB128 value.
- `AbbreviationTagZero` - An abbreviation declared that its tag is zero, but zero is reserved for
- `AttributeFormZero` - An attribute specification declared that its form is zero, but zero is
- `BadHasChildren` - The abbreviation's has-children byte was not one of
- `BadLength` - The specified length is impossible.
- `UnknownForm(constants::DwForm)` - Found an unknown `DW_FORM_*` type.
- `ExpectedZero` - Expected a zero, found something else.
- `DuplicateAbbreviationCode` - Found an abbreviation code that has already been used.
- `DuplicateArange` - Found a duplicate arange.
- `UnknownReservedLength` - Found an unknown reserved length value.
- `UnknownVersion(u64)` - Found an unknown DWARF version.
- `UnknownAbbreviation(u64)` - Found a record with an unknown abbreviation code.
- `UnexpectedEof(ReaderOffsetId)` - Hit the end of input before it was expected.
- `UnexpectedNull` - Read a null entry before it was expected.
- `UnknownStandardOpcode(constants::DwLns)` - Found an unknown standard opcode.
- `UnknownExtendedOpcode(constants::DwLne)` - Found an unknown extended opcode.
- `UnknownLocListsEntry(constants::DwLle)` - Found an unknown location-lists format.
- `UnknownRangeListsEntry(constants::DwRle)` - Found an unknown range-lists format.
- `UnsupportedAddressSize(u8)` - The specified address size is not supported.
- `UnsupportedOffsetSize(u8)` - The specified offset size is not supported.
- `UnsupportedFieldSize(u8)` - The specified field size is not supported.
- `MinimumInstructionLengthZero` - The minimum instruction length must not be zero.
- `MaximumOperationsPerInstructionZero` - The maximum operations per instruction must not be zero.
- `LineRangeZero` - The line range must not be zero.
- `OpcodeBaseZero` - The opcode base must not be zero.
- `BadUtf8` - Found an invalid UTF-8 string.
- `NotCieId` - Expected to find the CIE ID, but found something else.
- `NotCiePointer` - Expected to find a pointer to a CIE, but found the CIE ID instead.
- `NotFdePointer` - Expected to find a pointer to an FDE, but found a CIE instead.
- `BadBranchTarget(u64)` - Invalid branch target for a DW_OP_bra or DW_OP_skip.
- `InvalidPushObjectAddress` - DW_OP_push_object_address used but no address passed in.
- `NotEnoughStackItems` - Not enough items on the stack when evaluating an expression.
- `TooManyIterations` - Too many iterations to compute the expression.
- `InvalidExpression(constants::DwOp)` - An unrecognized operation was found while parsing a DWARF
- `UnsupportedEvaluation` - An unsupported operation was found while evaluating a DWARF expression.
- `InvalidPiece` - The expression had a piece followed by an expression
- `InvalidExpressionTerminator(u64)` - An expression-terminating operation was followed by something
- `DivisionByZero` - Division or modulus by zero when evaluating an expression.
- `TypeMismatch` - An expression operation used mismatching types.
- `IntegralTypeRequired` - An expression operation required an integral type but saw a
- `UnsupportedTypeOperation` - An expression operation used types that are not supported.
- `InvalidShiftExpression` - The shift value in an expression must be a non-negative integer.
- `InvalidDerefSize(u8)` - The size of a deref expression must not be larger than the size of an address.
- `UnknownCallFrameInstruction(constants::DwCfa)` - An unknown DW_CFA_* instruction.
- `InvalidAddressRange` - The end of an address range was before the beginning.
- `AddressOverflow` - An address calculation overflowed.
- `CfiInstructionInInvalidContext` - Encountered a call frame instruction in a context in which it is not
- `PopWithEmptyStack` - When evaluating call frame instructions, found a `DW_CFA_restore_state`
- `NoUnwindInfoForAddress` - Do not have unwind info for the given address.
- `UnsupportedOffset` - An offset value was larger than the maximum supported value.
- `UnknownPointerEncoding(constants::DwEhPe)` - The given pointer encoding is either unknown or invalid.
- `NoEntryAtGivenOffset` - Did not find an entry at the given offset.
- `OffsetOutOfBounds` - The given offset is out of bounds.
- `UnknownAugmentation` - Found an unknown CFI augmentation.
- `UnsupportedPointerEncoding` - We do not support the given pointer encoding yet.
- `UnsupportedRegister(u64)` - Registers larger than `u16` are not supported.
- `TooManyRegisterRules` - The CFI program defined more register rules than we have storage for.
- `StackFull` - Attempted to push onto the CFI or evaluation stack, but it was already
- `VariableLengthSearchTable` - The `.eh_frame_hdr` binary search table claims to be variable-length encoded,
- `UnsupportedUnitType` - The `DW_UT_*` value for this unit is not supported yet.
- `UnsupportedAddressIndex` - Ranges using AddressIndex are not supported yet.
- `UnsupportedSegmentSize` - Nonzero segment selector sizes aren't supported yet.
- `MissingUnitDie` - A compilation unit or type unit is missing its top level DIE.
- `UnsupportedAttributeForm` - A DIE attribute used an unsupported form.
- `MissingFileEntryFormatPath` - Missing DW_LNCT_path in file entry format.
- `ExpectedStringAttributeValue` - Expected an attribute value to be a string form.
- `InvalidImplicitConst` - `DW_FORM_implicit_const` used in an invalid context.
- `InvalidIndexSectionCount` - Invalid section count in `.dwp` index.
- `InvalidIndexSlotCount` - Invalid slot count in `.dwp` index.
- `InvalidIndexRow` - Invalid hash row in `.dwp` index.
- `UnknownIndexSection(constants::DwSect)` - Unknown section type in `.dwp` index.
- `UnknownIndexSectionV2(constants::DwSectV2)` - Unknown section type in version 2 `.dwp` index.
- `InvalidMacinfoType(constants::DwMacinfo)` - Invalid macinfo type in `.debug_macinfo`.
- `InvalidMacroType(constants::DwMacro)` - Invalid macro type in `.debug_macro`.
- `UnsupportedOpcodeOperandsTable` - The optional `opcode_operands_table` in `.debug_macro` is currently not supported.

**Methods:**

- `fn description(self: &Self) -> &str` - A short description of the error.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> ::core::result::Result<(), fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Error`



## gimli::read::Result

*Type Alias*: `result::Result<T, Error>`

The result of a parse.



## gimli::read::Section

*Trait*

A convenience trait for loading DWARF sections from object files.  To be
used like:

```
use gimli::{DebugInfo, EndianSlice, LittleEndian, Reader, Section};

let buf = [0x00, 0x01, 0x02, 0x03];
let reader = EndianSlice::new(&buf, LittleEndian);
let loader = |name| -> Result<_, ()> { Ok(reader) };

let debug_info: DebugInfo<_> = Section::load(loader).unwrap();
```

**Methods:**

- `id`: Returns the section id for this type.
- `section_name`: Returns the ELF section name for this type.
- `dwo_section_name`: Returns the ELF section name (if any) for this type when used in a dwo
- `xcoff_section_name`: Returns the XCOFF section name (if any) for this type when used in a XCOFF
- `load`: Try to load the section using the given loader function.
- `reader`: Returns the `Reader` for this section.
- `dwp_range`: Returns the subrange of the section that is the contribution of
- `lookup_offset_id`: Returns the `Reader` for this section.



## gimli::read::StoreOnHeap

*Struct*

Indicates that storage should be allocated on heap.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> StoreOnHeap`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &StoreOnHeap) -> bool`



## gimli::read::UnitOffset

*Struct*

An offset into the current compilation or type unit.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnitOffset<T>) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &UnitOffset<T>) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> UnitOffset<T>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &UnitOffset<T>) -> $crate::option::Option<$crate::cmp::Ordering>`



