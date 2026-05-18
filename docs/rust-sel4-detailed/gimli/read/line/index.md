*[gimli](../../index.md) / [read](../index.md) / [line](index.md)*

---

# Module `line`

## Contents

- [Structs](#structs)
  - [`DebugLine`](#debugline)
  - [`LineRows`](#linerows)
  - [`LineInstructions`](#lineinstructions)
  - [`LineRow`](#linerow)
  - [`LineSequence`](#linesequence)
  - [`LineProgramHeader`](#lineprogramheader)
  - [`IncompleteLineProgram`](#incompletelineprogram)
  - [`CompleteLineProgram`](#completelineprogram)
  - [`FileEntry`](#fileentry)
  - [`FileEntryFormat`](#fileentryformat)
- [Enums](#enums)
  - [`LineInstruction`](#lineinstruction)
  - [`ColumnType`](#columntype)
- [Traits](#traits)
  - [`LineProgram`](#lineprogram)
- [Functions](#functions)
  - [`parse_directory_v5`](#parse-directory-v5)
  - [`parse_file_v5`](#parse-file-v5)
  - [`parse_attribute`](#parse-attribute)
- [Type Aliases](#type-aliases)
  - [`OneShotLineRows`](#oneshotlinerows)
  - [`ResumedLineRows`](#resumedlinerows)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugLine`](#debugline) | struct | The `DebugLine` struct contains the source location to instruction mapping found in the `.debug_line` section. |
| [`LineRows`](#linerows) | struct | Executes a `LineProgram` to iterate over the rows in the matrix of line number information. |
| [`LineInstructions`](#lineinstructions) | struct | An iterator yielding parsed instructions. |
| [`LineRow`](#linerow) | struct | A row in the line number program's resulting matrix. |
| [`LineSequence`](#linesequence) | struct | A sequence within a line number program. |
| [`LineProgramHeader`](#lineprogramheader) | struct | A header for a line number program in the `.debug_line` section, as defined in section 6.2.4 of the standard. |
| [`IncompleteLineProgram`](#incompletelineprogram) | struct | A line number program that has not been run to completion. |
| [`CompleteLineProgram`](#completelineprogram) | struct | A line number program that has previously been run to completion. |
| [`FileEntry`](#fileentry) | struct | An entry in the `LineProgramHeader`'s `file_names` set. |
| [`FileEntryFormat`](#fileentryformat) | struct | The format of a component of an include directory or file name entry. |
| [`LineInstruction`](#lineinstruction) | enum | A parsed line number program instruction. |
| [`ColumnType`](#columntype) | enum | The type of column that a row is referring to. |
| [`LineProgram`](#lineprogram) | trait | A `LineProgram` provides access to a `LineProgramHeader` and a way to add files to the files table if necessary. |
| [`parse_directory_v5`](#parse-directory-v5) | fn |  |
| [`parse_file_v5`](#parse-file-v5) | fn |  |
| [`parse_attribute`](#parse-attribute) | fn |  |
| [`OneShotLineRows`](#oneshotlinerows) | type |  |
| [`ResumedLineRows`](#resumedlinerows) | type |  |

## Structs

### `DebugLine<R>`

```rust
struct DebugLine<R> {
    debug_line_section: R,
}
```

The `DebugLine` struct contains the source location to instruction mapping
found in the `.debug_line` section.

#### Implementations

- <span id="debugline-new"></span>`fn new(debug_line_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLine` instance from the data in the `.debug_line`

  section.

  

  It is the caller's responsibility to read the `.debug_line` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLine, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_line_section_somehow = || &buf;

  let debug_line = DebugLine::new(read_debug_line_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLine<R>`

- <span id="debugline-clone"></span>`fn clone(&self) -> DebugLine<R>` — [`DebugLine`](../index.md#debugline)

##### `impl<R: marker::Copy> Copy for DebugLine<R>`

##### `impl<R: fmt::Debug> Debug for DebugLine<R>`

- <span id="debugline-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLine<R>`

- <span id="debugline-default"></span>`fn default() -> DebugLine<R>` — [`DebugLine`](../index.md#debugline)

##### `impl<R> Section for DebugLine<R>`

- <span id="debugline-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugline-section-reader"></span>`fn reader(&self) -> &R`

### `LineRows<R, Program, Offset>`

```rust
struct LineRows<R, Program, Offset>
where
    Program: LineProgram<R, Offset>,
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    program: Program,
    row: LineRow,
    instructions: LineInstructions<R>,
}
```

Executes a `LineProgram` to iterate over the rows in the matrix of line number information.

"The hypothetical machine used by a consumer of the line number information
to expand the byte-coded instruction stream into a matrix of line number
information." -- Section 6.2.1

#### Implementations

- <span id="linerows-new"></span>`fn new(program: IncompleteLineProgram<R, Offset>) -> LineRows<R, IncompleteLineProgram<R, Offset>, Offset>` — [`IncompleteLineProgram`](../index.md#incompletelineprogram), [`LineRows`](../index.md#linerows)

- <span id="linerows-resume"></span>`fn resume<'program>(program: &'program CompleteLineProgram<R, Offset>, sequence: &LineSequence<R>) -> LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>` — [`CompleteLineProgram`](../index.md#completelineprogram), [`LineSequence`](../index.md#linesequence), [`LineRows`](../index.md#linerows)

- <span id="linerows-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md#lineprogramheader)

  Get a reference to the header for this state machine's line number

  program.

- <span id="linerows-next-row"></span>`fn next_row(&mut self) -> Result<Option<(&LineProgramHeader<R, Offset>, &LineRow)>>` — [`Result`](../../index.md#result), [`LineProgramHeader`](../index.md#lineprogramheader), [`LineRow`](../index.md#linerow)

  Parse and execute the next instructions in the line number program until

  another row in the line number matrix is computed.

  

  The freshly computed row is returned as `Ok(Some((header, row)))`.

  If the matrix is complete, and there are no more new rows in the line

  number matrix, then `Ok(None)` is returned. If there was an error parsing

  an instruction, then `Err(e)` is returned.

  

  Unfortunately, the references mean that this cannot be a

  `Iterator`.

#### Trait Implementations

##### `impl<R, Program, Offset> Clone for LineRows<R, Program, Offset>`

- <span id="linerows-clone"></span>`fn clone(&self) -> LineRows<R, Program, Offset>` — [`LineRows`](../index.md#linerows)

##### `impl<R, Program, Offset> Debug for LineRows<R, Program, Offset>`

- <span id="linerows-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LineInstructions<R: Reader>`

```rust
struct LineInstructions<R: Reader> {
    input: R,
}
```

An iterator yielding parsed instructions.

See
[`LineProgramHeader::instructions`](./struct.LineProgramHeader.html#method.instructions)
for more details.

#### Implementations

- <span id="lineinstructions-remove-trailing"></span>`fn remove_trailing(&self, other: &LineInstructions<R>) -> Result<LineInstructions<R>>` — [`LineInstructions`](../index.md#lineinstructions), [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for LineInstructions<R>`

- <span id="lineinstructions-clone"></span>`fn clone(&self) -> LineInstructions<R>` — [`LineInstructions`](../index.md#lineinstructions)

##### `impl<R: fmt::Debug + Reader> Debug for LineInstructions<R>`

- <span id="lineinstructions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LineRow`

```rust
struct LineRow {
    tombstone: bool,
    address: u64,
    op_index: core::num::Wrapping<u64>,
    file: u64,
    line: core::num::Wrapping<u64>,
    column: u64,
    is_stmt: bool,
    basic_block: bool,
    end_sequence: bool,
    prologue_end: bool,
    epilogue_begin: bool,
    isa: u64,
    discriminator: u64,
}
```

A row in the line number program's resulting matrix.

Each row is a copy of the registers of the state machine, as defined in section 6.2.2.

#### Implementations

- <span id="linerow-new"></span>`fn new<R: Reader>(header: &LineProgramHeader<R>) -> Self` — [`LineProgramHeader`](../index.md#lineprogramheader)

  Create a line number row in the initial state for the given program.

- <span id="linerow-address"></span>`fn address(&self) -> u64`

  "The program-counter value corresponding to a machine instruction

  generated by the compiler."

- <span id="linerow-op-index"></span>`fn op_index(&self) -> u64`

  > An unsigned integer representing the index of an operation within a VLIW

  > instruction. The index of the first operation is 0. For non-VLIW

  > architectures, this register will always be 0.

  >

  > The address and op_index registers, taken together, form an operation

  > pointer that can reference any individual operation with the

  > instruction stream.

- <span id="linerow-file-index"></span>`fn file_index(&self) -> u64`

  "An unsigned integer indicating the identity of the source file

  corresponding to a machine instruction."

- <span id="linerow-file"></span>`fn file<'header, R: Reader>(&self, header: &'header LineProgramHeader<R>) -> Option<&'header FileEntry<R>>` — [`LineProgramHeader`](../index.md#lineprogramheader), [`FileEntry`](../index.md#fileentry)

  The source file corresponding to the current machine instruction.

- <span id="linerow-line"></span>`fn line(&self) -> Option<NonZeroU64>`

  "An unsigned integer indicating a source line number. Lines are numbered

  beginning at 1. The compiler may emit the value 0 in cases where an

  instruction cannot be attributed to any source line."

  Line number values of 0 are represented as `None`.

- <span id="linerow-column"></span>`fn column(&self) -> ColumnType` — [`ColumnType`](../index.md#columntype)

  "An unsigned integer indicating a column number within a source

  line. Columns are numbered beginning at 1. The value 0 is reserved to

  indicate that a statement begins at the “left edge” of the line."

- <span id="linerow-is-stmt"></span>`fn is_stmt(&self) -> bool`

  "A boolean indicating that the current instruction is a recommended

  breakpoint location. A recommended breakpoint location is intended to

  “represent” a line, a statement and/or a semantically distinct subpart

  of a statement."

- <span id="linerow-basic-block"></span>`fn basic_block(&self) -> bool`

  "A boolean indicating that the current instruction is the beginning of a

  basic block."

- <span id="linerow-end-sequence"></span>`fn end_sequence(&self) -> bool`

  "A boolean indicating that the current address is that of the first byte

  after the end of a sequence of target machine instructions. end_sequence

  terminates a sequence of lines; therefore other information in the same

  row is not meaningful."

- <span id="linerow-prologue-end"></span>`fn prologue_end(&self) -> bool`

  "A boolean indicating that the current address is one (of possibly many)

  where execution should be suspended for an entry breakpoint of a

  function."

- <span id="linerow-epilogue-begin"></span>`fn epilogue_begin(&self) -> bool`

  "A boolean indicating that the current address is one (of possibly many)

  where execution should be suspended for an exit breakpoint of a

  function."

- <span id="linerow-isa"></span>`fn isa(&self) -> u64`

  Tag for the current instruction set architecture.

  

  > An unsigned integer whose value encodes the applicable instruction set

  > architecture for the current instruction.

  >

  > The encoding of instruction sets should be shared by all users of a

  > given architecture. It is recommended that this encoding be defined by

  > the ABI authoring committee for each architecture.

- <span id="linerow-discriminator"></span>`fn discriminator(&self) -> u64`

  "An unsigned integer identifying the block to which the current

  instruction belongs. Discriminator values are assigned arbitrarily by

  the DWARF producer and serve to distinguish among multiple blocks that

  may all be associated with the same source file, line, and column. Where

  only one block exists for a given source position, the discriminator

  value should be zero."

- <span id="linerow-execute"></span>`fn execute<R, Program>(&mut self, instruction: LineInstruction<R>, program: &mut Program) -> Result<bool>` — [`LineInstruction`](../index.md#lineinstruction), [`Result`](../../index.md#result)

  Execute the given instruction, and return true if a new row in the

  line number matrix needs to be generated.

  

  Unknown opcodes are treated as no-ops.

- <span id="linerow-reset"></span>`fn reset<R: Reader>(&mut self, header: &LineProgramHeader<R>)` — [`LineProgramHeader`](../index.md#lineprogramheader)

  Perform any reset that was required after copying the previous row.

- <span id="linerow-apply-line-advance"></span>`fn apply_line_advance(&mut self, line_increment: i64)`

  Step 1 of section 6.2.5.1

- <span id="linerow-apply-operation-advance"></span>`fn apply_operation_advance<R: Reader>(&mut self, operation_advance: u64, header: &LineProgramHeader<R>) -> Result<()>` — [`LineProgramHeader`](../index.md#lineprogramheader), [`Result`](../../index.md#result)

  Step 2 of section 6.2.5.1

- <span id="linerow-adjust-opcode"></span>`fn adjust_opcode<R: Reader>(&self, opcode: u8, header: &LineProgramHeader<R>) -> u8` — [`LineProgramHeader`](../index.md#lineprogramheader)

- <span id="linerow-exec-special-opcode"></span>`fn exec_special_opcode<R: Reader>(&mut self, opcode: u8, header: &LineProgramHeader<R>) -> Result<()>` — [`LineProgramHeader`](../index.md#lineprogramheader), [`Result`](../../index.md#result)

  Section 6.2.5.1

#### Trait Implementations

##### `impl Clone for LineRow`

- <span id="linerow-clone"></span>`fn clone(&self) -> LineRow` — [`LineRow`](../index.md#linerow)

##### `impl Copy for LineRow`

##### `impl Debug for LineRow`

- <span id="linerow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LineRow`

##### `impl PartialEq for LineRow`

- <span id="linerow-partialeq-eq"></span>`fn eq(&self, other: &LineRow) -> bool` — [`LineRow`](../index.md#linerow)

##### `impl StructuralPartialEq for LineRow`

### `LineSequence<R: Reader>`

```rust
struct LineSequence<R: Reader> {
    pub start: u64,
    pub end: u64,
    instructions: LineInstructions<R>,
}
```

A sequence within a line number program.  A sequence, as defined in section
6.2.5 of the standard, is a linear subset of a line number program within
which addresses are monotonically increasing.

#### Fields

- **`start`**: `u64`

  The first address that is covered by this sequence within the line number
  program.

- **`end`**: `u64`

  The first address that is *not* covered by this sequence within the line
  number program.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for LineSequence<R>`

- <span id="linesequence-clone"></span>`fn clone(&self) -> LineSequence<R>` — [`LineSequence`](../index.md#linesequence)

##### `impl<R: fmt::Debug + Reader> Debug for LineSequence<R>`

- <span id="linesequence-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LineProgramHeader<R, Offset>`

```rust
struct LineProgramHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    encoding: crate::common::Encoding,
    offset: crate::common::DebugLineOffset<Offset>,
    unit_length: Offset,
    header_length: Offset,
    line_encoding: crate::common::LineEncoding,
    opcode_base: u8,
    standard_opcode_lengths: R,
    directory_entry_format: alloc::vec::Vec<FileEntryFormat>,
    include_directories: alloc::vec::Vec<crate::read::AttributeValue<R, Offset>>,
    file_name_entry_format: alloc::vec::Vec<FileEntryFormat>,
    file_names: alloc::vec::Vec<FileEntry<R, Offset>>,
    program_buf: R,
    comp_dir: Option<R>,
    comp_file: Option<FileEntry<R, Offset>>,
}
```

A header for a line number program in the `.debug_line` section, as defined
in section 6.2.4 of the standard.

#### Fields

- **`opcode_base`**: `u8`

  "The number assigned to the first special opcode."

- **`standard_opcode_lengths`**: `R`

  "This array specifies the number of LEB128 operands for each of the
  standard opcodes. The first element of the array corresponds to the
  opcode whose value is 1, and the last element corresponds to the opcode
  whose value is `opcode_base - 1`."

- **`directory_entry_format`**: `alloc::vec::Vec<FileEntryFormat>`

  "A sequence of directory entry format descriptions."

- **`include_directories`**: `alloc::vec::Vec<crate::read::AttributeValue<R, Offset>>`

  > Entries in this sequence describe each path that was searched for
  > included source files in this compilation. (The paths include those
  > directories specified explicitly by the user for the compiler to search
  > and those the compiler searches without explicit direction.) Each path
  > entry is either a full path name or is relative to the current directory
  > of the compilation.
  >
  > The last entry is followed by a single null byte.

- **`file_name_entry_format`**: `alloc::vec::Vec<FileEntryFormat>`

  "A sequence of file entry format descriptions."

- **`file_names`**: `alloc::vec::Vec<FileEntry<R, Offset>>`

  "Entries in this sequence describe source files that contribute to the
  line number information for this compilation unit or is used in other
  contexts."

- **`program_buf`**: `R`

  The encoded line program instructions.

- **`comp_dir`**: `Option<R>`

  The current directory of the compilation.

- **`comp_file`**: `Option<FileEntry<R, Offset>>`

  The primary source file.

#### Implementations

- <span id="lineprogramheader-offset"></span>`fn offset(&self) -> DebugLineOffset<<R as >::Offset>` — [`DebugLineOffset`](../../index.md#debuglineoffset), [`Reader`](../index.md#reader)

  Return the offset of the line number program header in the `.debug_line` section.

- <span id="lineprogramheader-unit-length"></span>`fn unit_length(&self) -> <R as >::Offset` — [`Reader`](../index.md#reader)

  Return the length of the line number program and header, not including

  the length of the encoded length itself.

- <span id="lineprogramheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../../index.md#encoding)

  Return the encoding parameters for this header's line program.

- <span id="lineprogramheader-version"></span>`fn version(&self) -> u16`

  Get the version of this header's line program.

- <span id="lineprogramheader-header-length"></span>`fn header_length(&self) -> <R as >::Offset` — [`Reader`](../index.md#reader)

  Get the length of the encoded line number program header, not including

  the length of the encoded length itself.

- <span id="lineprogramheader-address-size"></span>`fn address_size(&self) -> u8`

  Get the size in bytes of a target machine address.

- <span id="lineprogramheader-format"></span>`fn format(&self) -> Format` — [`Format`](../../index.md#format)

  Whether this line program is encoded in 64- or 32-bit DWARF.

- <span id="lineprogramheader-line-encoding"></span>`fn line_encoding(&self) -> LineEncoding` — [`LineEncoding`](../../index.md#lineencoding)

  Get the line encoding parameters for this header's line program.

- <span id="lineprogramheader-minimum-instruction-length"></span>`fn minimum_instruction_length(&self) -> u8`

  Get the minimum instruction length any instruction in this header's line

  program may have.

- <span id="lineprogramheader-maximum-operations-per-instruction"></span>`fn maximum_operations_per_instruction(&self) -> u8`

  Get the maximum number of operations each instruction in this header's

  line program may have.

- <span id="lineprogramheader-default-is-stmt"></span>`fn default_is_stmt(&self) -> bool`

  Get the default value of the `is_stmt` register for this header's line

  program.

- <span id="lineprogramheader-line-base"></span>`fn line_base(&self) -> i8`

  Get the line base for this header's line program.

- <span id="lineprogramheader-line-range"></span>`fn line_range(&self) -> u8`

  Get the line range for this header's line program.

- <span id="lineprogramheader-opcode-base"></span>`fn opcode_base(&self) -> u8`

  Get opcode base for this header's line program.

- <span id="lineprogramheader-standard-opcode-lengths"></span>`fn standard_opcode_lengths(&self) -> &R`

  An array of `u8` that specifies the number of LEB128 operands for

  each of the standard opcodes.

- <span id="lineprogramheader-directory-entry-format"></span>`fn directory_entry_format(&self) -> &[FileEntryFormat]` — [`FileEntryFormat`](../index.md#fileentryformat)

  Get the format of a directory entry.

- <span id="lineprogramheader-include-directories"></span>`fn include_directories(&self) -> &[AttributeValue<R, Offset>]` — [`AttributeValue`](../index.md#attributevalue)

  Get the set of include directories for this header's line program.

  

  For DWARF version <= 4, the compilation's current directory is not included

  in the return value, but is implicitly considered to be in the set per spec.

- <span id="lineprogramheader-directory"></span>`fn directory(&self, directory: u64) -> Option<AttributeValue<R, Offset>>` — [`AttributeValue`](../index.md#attributevalue)

  The include directory with the given directory index.

  

  A directory index of 0 corresponds to the compilation unit directory.

- <span id="lineprogramheader-file-name-entry-format"></span>`fn file_name_entry_format(&self) -> &[FileEntryFormat]` — [`FileEntryFormat`](../index.md#fileentryformat)

  Get the format of a file name entry.

- <span id="lineprogramheader-file-has-timestamp"></span>`fn file_has_timestamp(&self) -> bool`

  Return true if the file entries may have valid timestamps.

  

  Only returns false if we definitely know that all timestamp fields

  are invalid.

- <span id="lineprogramheader-file-has-size"></span>`fn file_has_size(&self) -> bool`

  Return true if the file entries may have valid sizes.

  

  Only returns false if we definitely know that all size fields

  are invalid.

- <span id="lineprogramheader-file-has-md5"></span>`fn file_has_md5(&self) -> bool`

  Return true if the file name entry format contains an MD5 field.

- <span id="lineprogramheader-file-has-source"></span>`fn file_has_source(&self) -> bool`

  Return true if the file name entry format contains a source field.

- <span id="lineprogramheader-file-names"></span>`fn file_names(&self) -> &[FileEntry<R, Offset>]` — [`FileEntry`](../index.md#fileentry)

  Get the list of source files that appear in this header's line program.

- <span id="lineprogramheader-file"></span>`fn file(&self, file: u64) -> Option<&FileEntry<R, Offset>>` — [`FileEntry`](../index.md#fileentry)

  The source file with the given file index.

  

  A file index of 0 corresponds to the compilation unit file.

  Note that a file index of 0 is invalid for DWARF version <= 4,

  but we support it anyway.

- <span id="lineprogramheader-raw-program-buf"></span>`fn raw_program_buf(&self) -> R`

  Get the raw, un-parsed `EndianSlice` containing this header's line number

  program.

  

  ```rust

  fn foo() {

  use gimli::{LineProgramHeader, EndianSlice, NativeEndian};

  

  fn get_line_number_program_header<'a>() -> LineProgramHeader<EndianSlice<'a, NativeEndian>> {

      // Get a line number program header from some offset in a

      // `.debug_line` section...

    unimplemented!()

  }

  

  let header = get_line_number_program_header();

  let raw_program = header.raw_program_buf();

  println!("The length of the raw program in bytes is {}", raw_program.len());

  }

  ```

- <span id="lineprogramheader-instructions"></span>`fn instructions(&self) -> LineInstructions<R>` — [`LineInstructions`](../index.md#lineinstructions)

  Iterate over the instructions in this header's line number program, parsing

  them as we go.

- <span id="lineprogramheader-parse"></span>`fn parse(input: &mut R, offset: DebugLineOffset<Offset>, address_size: u8, comp_dir: Option<R>, comp_name: Option<R>) -> Result<LineProgramHeader<R, Offset>>` — [`DebugLineOffset`](../../index.md#debuglineoffset), [`Result`](../../index.md#result), [`LineProgramHeader`](../index.md#lineprogramheader)

#### Trait Implementations

##### `impl<R, Offset> Clone for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-clone"></span>`fn clone(&self) -> LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md#lineprogramheader)

##### `impl<R, Offset> Debug for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for LineProgramHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-partialeq-eq"></span>`fn eq(&self, other: &LineProgramHeader<R, Offset>) -> bool` — [`LineProgramHeader`](../index.md#lineprogramheader)

##### `impl<R, Offset> StructuralPartialEq for LineProgramHeader<R, Offset>`

### `IncompleteLineProgram<R, Offset>`

```rust
struct IncompleteLineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    header: LineProgramHeader<R, Offset>,
}
```

A line number program that has not been run to completion.

#### Implementations

- <span id="incompletelineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md#lineprogramheader)

  Retrieve the `LineProgramHeader` for this program.

- <span id="incompletelineprogram-rows"></span>`fn rows(self) -> LineRows<R, IncompleteLineProgram<R, Offset>, Offset>` — [`LineRows`](../index.md#linerows), [`IncompleteLineProgram`](../index.md#incompletelineprogram)

  Construct a new `LineRows` for executing this program to iterate

  over rows in the line information matrix.

- <span id="incompletelineprogram-sequences"></span>`fn sequences(self) -> Result<(CompleteLineProgram<R, Offset>, Vec<LineSequence<R>>)>` — [`Result`](../../index.md#result), [`CompleteLineProgram`](../index.md#completelineprogram), [`LineSequence`](../index.md#linesequence)

  Execute the line number program, completing the `IncompleteLineProgram`

  into a `CompleteLineProgram` and producing an array of sequences within

  the line number program that can later be used with

  `CompleteLineProgram::resume_from`.

  

  ```rust

  fn foo() {

  use gimli::{IncompleteLineProgram, EndianSlice, NativeEndian};

  

  fn get_line_number_program<'a>() -> IncompleteLineProgram<EndianSlice<'a, NativeEndian>> {

      // Get a line number program from some offset in a

      // `.debug_line` section...

    unimplemented!()

  }

  

  let program = get_line_number_program();

  let (program, sequences) = program.sequences().unwrap();

  println!("There are {} sequences in this line number program", sequences.len());

  }

  ```

#### Trait Implementations

##### `impl<R, Offset> Clone for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-clone"></span>`fn clone(&self) -> IncompleteLineProgram<R, Offset>` — [`IncompleteLineProgram`](../index.md#incompletelineprogram)

##### `impl<R, Offset> Debug for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for IncompleteLineProgram<R, Offset>`

##### `impl<R, Offset> LineProgram for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-lineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md#lineprogramheader)

- <span id="incompletelineprogram-lineprogram-add-file"></span>`fn add_file(&mut self, file: FileEntry<R, Offset>)` — [`FileEntry`](../index.md#fileentry)

##### `impl<R, Offset> PartialEq for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-partialeq-eq"></span>`fn eq(&self, other: &IncompleteLineProgram<R, Offset>) -> bool` — [`IncompleteLineProgram`](../index.md#incompletelineprogram)

##### `impl<R, Offset> StructuralPartialEq for IncompleteLineProgram<R, Offset>`

### `CompleteLineProgram<R, Offset>`

```rust
struct CompleteLineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    header: LineProgramHeader<R, Offset>,
}
```

A line number program that has previously been run to completion.

#### Implementations

- <span id="completelineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md#lineprogramheader)

  Retrieve the `LineProgramHeader` for this program.

- <span id="completelineprogram-resume-from"></span>`fn resume_from<'program>(self: &'program Self, sequence: &LineSequence<R>) -> LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>` — [`LineSequence`](../index.md#linesequence), [`LineRows`](../index.md#linerows), [`CompleteLineProgram`](../index.md#completelineprogram)

  Construct a new `LineRows` for executing the subset of the line

  number program identified by 'sequence' and  generating the line information

  matrix.

  

  ```rust

  fn foo() {

  use gimli::{IncompleteLineProgram, EndianSlice, NativeEndian};

  

  fn get_line_number_program<'a>() -> IncompleteLineProgram<EndianSlice<'a, NativeEndian>> {

      // Get a line number program from some offset in a

      // `.debug_line` section...

    unimplemented!()

  }

  

  let program = get_line_number_program();

  let (program, sequences) = program.sequences().unwrap();

  for sequence in &sequences {

      let mut sm = program.resume_from(sequence);

  }

  }

  ```

#### Trait Implementations

##### `impl<R, Offset> Clone for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-clone"></span>`fn clone(&self) -> CompleteLineProgram<R, Offset>` — [`CompleteLineProgram`](../index.md#completelineprogram)

##### `impl<R, Offset> Debug for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for CompleteLineProgram<R, Offset>`

##### `impl<R, Offset> LineProgram for &'program CompleteLineProgram<R, Offset>`

- <span id="program-completelineprogram-lineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md#lineprogramheader)

- <span id="program-completelineprogram-lineprogram-add-file"></span>`fn add_file(&mut self, _: FileEntry<R, Offset>)` — [`FileEntry`](../index.md#fileentry)

##### `impl<R, Offset> PartialEq for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-partialeq-eq"></span>`fn eq(&self, other: &CompleteLineProgram<R, Offset>) -> bool` — [`CompleteLineProgram`](../index.md#completelineprogram)

##### `impl<R, Offset> StructuralPartialEq for CompleteLineProgram<R, Offset>`

### `FileEntry<R, Offset>`

```rust
struct FileEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    path_name: crate::read::AttributeValue<R, Offset>,
    directory_index: u64,
    timestamp: u64,
    size: u64,
    md5: [u8; 16],
    source: Option<crate::read::AttributeValue<R, Offset>>,
}
```

An entry in the `LineProgramHeader`'s `file_names` set.

#### Implementations

- <span id="fileentry-parse"></span>`fn parse(input: &mut R, path_name: R) -> Result<FileEntry<R, Offset>>` — [`Result`](../../index.md#result), [`FileEntry`](../index.md#fileentry)

- <span id="fileentry-path-name"></span>`fn path_name(&self) -> AttributeValue<R, Offset>` — [`AttributeValue`](../index.md#attributevalue)

  > A slice containing the full or relative path name of

  > a source file. If the entry contains a file name or a relative path

  > name, the file is located relative to either the compilation directory

  > (as specified by the DW_AT_comp_dir attribute given in the compilation

  > unit) or one of the directories in the include_directories section.

- <span id="fileentry-directory-index"></span>`fn directory_index(&self) -> u64`

  > An unsigned LEB128 number representing the directory index of the

  > directory in which the file was found.

  >

  > ...

  >

  > The directory index represents an entry in the include_directories

  > section of the line number program header. The index is 0 if the file

  > was found in the current directory of the compilation, 1 if it was found

  > in the first directory in the include_directories section, and so

  > on. The directory index is ignored for file names that represent full

  > path names.

- <span id="fileentry-directory"></span>`fn directory(&self, header: &LineProgramHeader<R>) -> Option<AttributeValue<R, Offset>>` — [`LineProgramHeader`](../index.md#lineprogramheader), [`AttributeValue`](../index.md#attributevalue)

  Get this file's directory.

  

  A directory index of 0 corresponds to the compilation unit directory.

- <span id="fileentry-timestamp"></span>`fn timestamp(&self) -> u64`

  The implementation-defined time of last modification of the file,

  or 0 if not available.

- <span id="fileentry-size"></span>`fn size(&self) -> u64`

  The size of the file in bytes, or 0 if not available.

- <span id="fileentry-md5"></span>`fn md5(&self) -> &[u8; 16]`

  A 16-byte MD5 digest of the file contents.

  

  Only valid if `LineProgramHeader::file_has_md5` returns `true`.

- <span id="fileentry-source"></span>`fn source(&self) -> Option<AttributeValue<R, Offset>>` — [`AttributeValue`](../index.md#attributevalue)

  The source code of this file. (UTF-8 source text string with "\n" line

  endings).

  

  Note: For DWARF v5 files this may return an empty attribute that

  indicates that no source code is available, which this function

  represents as `Some(<zero-length attr>)`.

#### Trait Implementations

##### `impl<R, Offset> Clone for FileEntry<R, Offset>`

- <span id="fileentry-clone"></span>`fn clone(&self) -> FileEntry<R, Offset>` — [`FileEntry`](../index.md#fileentry)

##### `impl<R, Offset> Copy for FileEntry<R, Offset>`

##### `impl<R, Offset> Debug for FileEntry<R, Offset>`

- <span id="fileentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for FileEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for FileEntry<R, Offset>`

- <span id="fileentry-partialeq-eq"></span>`fn eq(&self, other: &FileEntry<R, Offset>) -> bool` — [`FileEntry`](../index.md#fileentry)

##### `impl<R, Offset> StructuralPartialEq for FileEntry<R, Offset>`

### `FileEntryFormat`

```rust
struct FileEntryFormat {
    pub content_type: constants::DwLnct,
    pub form: constants::DwForm,
}
```

The format of a component of an include directory or file name entry.

#### Fields

- **`content_type`**: `constants::DwLnct`

  The type of information that is represented by the component.

- **`form`**: `constants::DwForm`

  The encoding form of the component value.

#### Implementations

- <span id="fileentryformat-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Vec<FileEntryFormat>>` — [`Result`](../../index.md#result), [`FileEntryFormat`](../index.md#fileentryformat)

#### Trait Implementations

##### `impl Clone for FileEntryFormat`

- <span id="fileentryformat-clone"></span>`fn clone(&self) -> FileEntryFormat` — [`FileEntryFormat`](../index.md#fileentryformat)

##### `impl Copy for FileEntryFormat`

##### `impl Debug for FileEntryFormat`

- <span id="fileentryformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileEntryFormat`

##### `impl PartialEq for FileEntryFormat`

- <span id="fileentryformat-partialeq-eq"></span>`fn eq(&self, other: &FileEntryFormat) -> bool` — [`FileEntryFormat`](../index.md#fileentryformat)

##### `impl StructuralPartialEq for FileEntryFormat`

## Enums

### `LineInstruction<R, Offset>`

```rust
enum LineInstruction<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Special(u8),
    Copy,
    AdvancePc(u64),
    AdvanceLine(i64),
    SetFile(u64),
    SetColumn(u64),
    NegateStatement,
    SetBasicBlock,
    ConstAddPc,
    FixedAddPc(u16),
    SetPrologueEnd,
    SetEpilogueBegin,
    SetIsa(u64),
    UnknownStandard0(constants::DwLns),
    UnknownStandard1(constants::DwLns, u64),
    UnknownStandardN(constants::DwLns, R),
    EndSequence,
    SetAddress(u64),
    DefineFile(FileEntry<R, Offset>),
    SetDiscriminator(u64),
    UnknownExtended(constants::DwLne, R),
}
```

A parsed line number program instruction.

#### Variants

- **`Special`**

  > ### 6.2.5.1 Special Opcodes
  >
  > Each ubyte special opcode has the following effect on the state machine:
  >
  >   1. Add a signed integer to the line register.
  >
  >   2. Modify the operation pointer by incrementing the address and
  >      op_index registers as described below.
  >
  >   3. Append a row to the matrix using the current values of the state
  >      machine registers.
  >
  >   4. Set the basic_block register to “false.”
  >
  >   5. Set the prologue_end register to “false.”
  >
  >   6. Set the epilogue_begin register to “false.”
  >
  >   7. Set the discriminator register to 0.
  >
  > All of the special opcodes do those same seven things; they differ from
  > one another only in what values they add to the line, address and
  > op_index registers.

- **`Copy`**

  "[`LineInstruction::Copy`](../../index.md) appends a row to the matrix using the current
  values of the state machine registers. Then it sets the discriminator
  register to 0, and sets the basic_block, prologue_end and epilogue_begin
  registers to “false.”"

- **`AdvancePc`**

  "The DW_LNS_advance_pc opcode takes a single unsigned LEB128 operand as
  the operation advance and modifies the address and op_index registers
  [the same as `LineInstruction::Special`]"

- **`AdvanceLine`**

  "The DW_LNS_advance_line opcode takes a single signed LEB128 operand and
  adds that value to the line register of the state machine."

- **`SetFile`**

  "The DW_LNS_set_file opcode takes a single unsigned LEB128 operand and
  stores it in the file register of the state machine."

- **`SetColumn`**

  "The DW_LNS_set_column opcode takes a single unsigned LEB128 operand and
  stores it in the column register of the state machine."

- **`NegateStatement`**

  "The DW_LNS_negate_stmt opcode takes no operands. It sets the is_stmt
  register of the state machine to the logical negation of its current
  value."

- **`SetBasicBlock`**

  "The DW_LNS_set_basic_block opcode takes no operands. It sets the
  basic_block register of the state machine to “true.”"

- **`ConstAddPc`**

  > The DW_LNS_const_add_pc opcode takes no operands. It advances the
  > address and op_index registers by the increments corresponding to
  > special opcode 255.
  >
  > When the line number program needs to advance the address by a small
  > amount, it can use a single special opcode, which occupies a single
  > byte. When it needs to advance the address by up to twice the range of
  > the last special opcode, it can use DW_LNS_const_add_pc followed by a
  > special opcode, for a total of two bytes. Only if it needs to advance
  > the address by more than twice that range will it need to use both
  > DW_LNS_advance_pc and a special opcode, requiring three or more bytes.

- **`FixedAddPc`**

  > The DW_LNS_fixed_advance_pc opcode takes a single uhalf (unencoded)
  > operand and adds it to the address register of the state machine and
  > sets the op_index register to 0. This is the only standard opcode whose
  > operand is not a variable length number. It also does not multiply the
  > operand by the minimum_instruction_length field of the header.

- **`SetPrologueEnd`**

  "[`LineInstruction::SetPrologueEnd`](../../index.md) sets the prologue_end register to “true”."

- **`SetEpilogueBegin`**

  "[`LineInstruction::SetEpilogueBegin`](../../index.md) sets the epilogue_begin register to
  “true”."

- **`SetIsa`**

  "The DW_LNS_set_isa opcode takes a single unsigned LEB128 operand and
  stores that value in the isa register of the state machine."

- **`UnknownStandard0`**

  An unknown standard opcode with zero operands.

- **`UnknownStandard1`**

  An unknown standard opcode with one operand.

- **`UnknownStandardN`**

  An unknown standard opcode with multiple operands.

- **`EndSequence`**

  > [`LineInstruction::EndSequence`](../../index.md) sets the end_sequence register of the state
  > machine to “true” and appends a row to the matrix using the current
  > values of the state-machine registers. Then it resets the registers to
  > the initial values specified above (see Section 6.2.2). Every line
  > number program sequence must end with a DW_LNE_end_sequence instruction
  > which creates a row whose address is that of the byte after the last
  > target machine instruction of the sequence.

- **`SetAddress`**

  > The DW_LNE_set_address opcode takes a single relocatable address as an
  > operand. The size of the operand is the size of an address on the target
  > machine. It sets the address register to the value given by the
  > relocatable address and sets the op_index register to 0.
  >
  > All of the other line number program opcodes that affect the address
  > register add a delta to it. This instruction stores a relocatable value
  > into it instead.

- **`DefineFile`**

  Defines a new source file in the line number program and appends it to
  the line number program header's list of source files.

- **`SetDiscriminator`**

  "The DW_LNE_set_discriminator opcode takes a single parameter, an
  unsigned LEB128 integer. It sets the discriminator register to the new
  value."

- **`UnknownExtended`**

  An unknown extended opcode and the slice of its unparsed operands.

#### Implementations

- <span id="lineinstruction-parse"></span>`fn parse<'header>(header: &'header LineProgramHeader<R>, input: &mut R) -> Result<LineInstruction<R>>` — [`LineProgramHeader`](../index.md#lineprogramheader), [`Result`](../../index.md#result), [`LineInstruction`](../index.md#lineinstruction)

#### Trait Implementations

##### `impl<R, Offset> Clone for LineInstruction<R, Offset>`

- <span id="lineinstruction-clone"></span>`fn clone(&self) -> LineInstruction<R, Offset>` — [`LineInstruction`](../index.md#lineinstruction)

##### `impl<R, Offset> Copy for LineInstruction<R, Offset>`

##### `impl<R, Offset> Debug for LineInstruction<R, Offset>`

- <span id="lineinstruction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for LineInstruction<R, Offset>`

##### `impl<R, Offset> PartialEq for LineInstruction<R, Offset>`

- <span id="lineinstruction-partialeq-eq"></span>`fn eq(&self, other: &LineInstruction<R, Offset>) -> bool` — [`LineInstruction`](../index.md#lineinstruction)

##### `impl<R, Offset> StructuralPartialEq for LineInstruction<R, Offset>`

### `ColumnType`

```rust
enum ColumnType {
    LeftEdge,
    Column(core::num::NonZeroU64),
}
```

The type of column that a row is referring to.

#### Variants

- **`LeftEdge`**

  The `LeftEdge` means that the statement begins at the start of the new
  line.

- **`Column`**

  A column number, whose range begins at 1.

#### Trait Implementations

##### `impl Clone for ColumnType`

- <span id="columntype-clone"></span>`fn clone(&self) -> ColumnType` — [`ColumnType`](../index.md#columntype)

##### `impl Copy for ColumnType`

##### `impl Debug for ColumnType`

- <span id="columntype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ColumnType`

##### `impl Ord for ColumnType`

- <span id="columntype-ord-cmp"></span>`fn cmp(&self, other: &ColumnType) -> cmp::Ordering` — [`ColumnType`](../index.md#columntype)

##### `impl PartialEq for ColumnType`

- <span id="columntype-partialeq-eq"></span>`fn eq(&self, other: &ColumnType) -> bool` — [`ColumnType`](../index.md#columntype)

##### `impl PartialOrd for ColumnType`

- <span id="columntype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ColumnType) -> option::Option<cmp::Ordering>` — [`ColumnType`](../index.md#columntype)

##### `impl StructuralPartialEq for ColumnType`

## Traits

### `LineProgram<R, Offset>`

```rust
trait LineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset { ... }
```

A `LineProgram` provides access to a `LineProgramHeader` and
a way to add files to the files table if necessary. Gimli consumers should
never need to use or see this trait.

#### Required Methods

- `fn header(&self) -> &LineProgramHeader<R, Offset>`

  Get a reference to the held `LineProgramHeader`.

- `fn add_file(&mut self, file: FileEntry<R, Offset>)`

  Add a file to the file table if necessary.

#### Implementors

- [`IncompleteLineProgram`](../index.md#incompletelineprogram)
- `&'program CompleteLineProgram<R, Offset>`

## Functions

### `parse_directory_v5`

```rust
fn parse_directory_v5<R: Reader>(input: &mut R, encoding: crate::common::Encoding, formats: &[FileEntryFormat]) -> crate::read::Result<crate::read::AttributeValue<R>>
```

### `parse_file_v5`

```rust
fn parse_file_v5<R: Reader>(input: &mut R, encoding: crate::common::Encoding, formats: &[FileEntryFormat]) -> crate::read::Result<FileEntry<R>>
```

### `parse_attribute`

```rust
fn parse_attribute<R: Reader>(input: &mut R, encoding: crate::common::Encoding, form: constants::DwForm) -> crate::read::Result<crate::read::AttributeValue<R>>
```

## Type Aliases

### `OneShotLineRows<R, Offset>`

```rust
type OneShotLineRows<R, Offset> = LineRows<R, IncompleteLineProgram<R, Offset>, Offset>;
```

### `ResumedLineRows<'program, R, Offset>`

```rust
type ResumedLineRows<'program, R, Offset> = LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>;
```

