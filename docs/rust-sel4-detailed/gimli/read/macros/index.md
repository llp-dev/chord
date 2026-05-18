*[gimli](../../index.md) / [read](../index.md) / [macros](index.md)*

---

# Module `macros`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugMacinfo`](#debugmacinfo) | struct | The raw contents of the `.debug_macinfo` section. |
| [`DebugMacro`](#debugmacro) | struct | The raw contents of the `.debug_macro` section. |
| [`MacroUnitHeader`](#macrounitheader) | struct |  |
| [`MacroIter`](#macroiter) | struct | Iterator over the entries in the `.debug_macro` section. |
| [`MacroString`](#macrostring) | enum | A string in a macro entry. |
| [`MacroEntry`](#macroentry) | enum | an Entry in the `.debug_macro` section. |

## Structs

### `DebugMacinfo<R>`

```rust
struct DebugMacinfo<R> {
    section: R,
}
```

The raw contents of the `.debug_macinfo` section.

#### Implementations

- <span id="debugmacinfo-new"></span>`fn new(macinfo_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugMacinfo` instance from the data in the `.debug_macinfo`

  section.

  

  It is the caller's responsibility to read the `.debug_macinfo` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugMacinfo, LittleEndian};

  

  let buf = [1, 0, 95, 95, 83, 84, 68, 67, 95, 95, 32, 49, 0];

  let read_section_somehow = || &buf;

  let debug_str = DebugMacinfo::new(read_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugMacinfo<R>`

- <span id="debugmacinfo-clone"></span>`fn clone(&self) -> DebugMacinfo<R>` — [`DebugMacinfo`](../index.md#debugmacinfo)

##### `impl<R: marker::Copy> Copy for DebugMacinfo<R>`

##### `impl<R: fmt::Debug> Debug for DebugMacinfo<R>`

- <span id="debugmacinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugMacinfo<R>`

- <span id="debugmacinfo-default"></span>`fn default() -> DebugMacinfo<R>` — [`DebugMacinfo`](../index.md#debugmacinfo)

##### `impl<R> Section for DebugMacinfo<R>`

- <span id="debugmacinfo-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugmacinfo-section-reader"></span>`fn reader(&self) -> &R`

### `DebugMacro<R>`

```rust
struct DebugMacro<R> {
    section: R,
}
```

The raw contents of the `.debug_macro` section.

#### Implementations

- <span id="debugmacro-new"></span>`fn new(macro_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugMacro` instance from the data in the `.debug_macro`

  section.

  

  It is the caller's responsibility to read the `.debug_macro` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugMacro, LittleEndian};

  

  let buf = [1, 0, 95, 95, 83, 84, 68, 67, 95, 95, 32, 49, 0];

  let read_section_somehow = || &buf;

  let debug_str = DebugMacro::new(read_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugMacro<R>`

- <span id="debugmacro-clone"></span>`fn clone(&self) -> DebugMacro<R>` — [`DebugMacro`](../index.md#debugmacro)

##### `impl<R: marker::Copy> Copy for DebugMacro<R>`

##### `impl<R: fmt::Debug> Debug for DebugMacro<R>`

- <span id="debugmacro-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugMacro<R>`

- <span id="debugmacro-default"></span>`fn default() -> DebugMacro<R>` — [`DebugMacro`](../index.md#debugmacro)

##### `impl<R> Section for DebugMacro<R>`

- <span id="debugmacro-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugmacro-section-reader"></span>`fn reader(&self) -> &R`

### `MacroUnitHeader<R: Reader>`

```rust
struct MacroUnitHeader<R: Reader> {
    _version: u16,
    flags: u8,
    _debug_line_offset: crate::DebugLineOffset<<R as >::Offset>,
}
```

#### Fields

- **`_version`**: `u16`

  The version of the macro unit header. At the moment only version 5 is defined.

#### Implementations

- <span id="macrounitheader-const-offset-size-flag"></span>`const OFFSET_SIZE_FLAG: u8`

- <span id="macrounitheader-const-debug-line-offset-flag"></span>`const DEBUG_LINE_OFFSET_FLAG: u8`

- <span id="macrounitheader-const-opcode-operands-table-flag"></span>`const OPCODE_OPERANDS_TABLE_FLAG: u8`

- <span id="macrounitheader-parse"></span>`fn parse(input: &mut R) -> Result<Self>` — [`Result`](../../index.md#result)

- <span id="macrounitheader-format"></span>`fn format(&self) -> Format` — [`Format`](../../index.md#format)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for MacroUnitHeader<R>`

- <span id="macrounitheader-clone"></span>`fn clone(&self) -> MacroUnitHeader<R>` — [`MacroUnitHeader`](#macrounitheader)

##### `impl<R: fmt::Debug + Reader> Debug for MacroUnitHeader<R>`

- <span id="macrounitheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MacroIter<R: Reader>`

```rust
struct MacroIter<R: Reader> {
    input: R,
    format: crate::Format,
    is_macro: bool,
}
```

Iterator over the entries in the `.debug_macro` section.

#### Implementations

- <span id="macroiter-next"></span>`fn next(&mut self) -> Result<Option<MacroEntry<R>>>` — [`Result`](../../index.md#result), [`MacroEntry`](../index.md#macroentry)

  Advance the iterator to the next entry in the `.debug_macro` section.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for MacroIter<R>`

- <span id="macroiter-clone"></span>`fn clone(&self) -> MacroIter<R>` — [`MacroIter`](../index.md#macroiter)

##### `impl<R: fmt::Debug + Reader> Debug for MacroIter<R>`

- <span id="macroiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for MacroIter<R>`

- <span id="macroiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="macroiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="macroiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for MacroIter<R>`

- <span id="macroiter-iterator-type-item"></span>`type Item = Result<MacroEntry<R>, Error>`

- <span id="macroiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `MacroString<R, Offset>`

```rust
enum MacroString<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Direct(R),
    StringPointer(crate::DebugStrOffset<Offset>),
    IndirectStringPointer(crate::DebugStrOffsetsIndex<Offset>),
    Supplementary(crate::DebugStrOffset<Offset>),
}
```

A string in a macro entry.

#### Variants

- **`Direct`**

  The string is directly embedded in the macro entry

- **`StringPointer`**

  The macro refers to a string in the `.debug_str` section using a `DebugStrOffset`.

- **`IndirectStringPointer`**

  The macro contains an index into an array in the `.debug_str_offsets`
  section, which refers to a string in the `.debug_str` section.

- **`Supplementary`**

  The macro refers to a string in the `.debug_str` section in the supplementary object file

#### Implementations

- <span id="macrostring-string"></span>`fn string(&self, unit: UnitRef<'_, R>) -> Result<R>` — [`UnitRef`](../index.md#unitref), [`Result`](../../index.md#result)

  Get the string slice from the macro entry.

#### Trait Implementations

##### `impl<R, Offset> Clone for MacroString<R, Offset>`

- <span id="macrostring-clone"></span>`fn clone(&self) -> MacroString<R, Offset>` — [`MacroString`](../index.md#macrostring)

##### `impl<R, Offset> Debug for MacroString<R, Offset>`

- <span id="macrostring-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for MacroString<R, Offset>`

##### `impl<R, Offset> PartialEq for MacroString<R, Offset>`

- <span id="macrostring-partialeq-eq"></span>`fn eq(&self, other: &MacroString<R, Offset>) -> bool` — [`MacroString`](../index.md#macrostring)

##### `impl<R, Offset> StructuralPartialEq for MacroString<R, Offset>`

### `MacroEntry<R, Offset>`

```rust
enum MacroEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Define {
        line: u64,
        text: MacroString<R>,
    },
    Undef {
        line: u64,
        name: MacroString<R>,
    },
    StartFile {
        line: u64,
        file: u64,
    },
    EndFile,
    Import {
        offset: crate::DebugMacroOffset<Offset>,
    },
    ImportSup {
        offset: crate::DebugMacroOffset<Offset>,
    },
    VendorExt {
        numeric: u64,
        string: R,
    },
}
```

an Entry in the `.debug_macro` section.

#### Variants

- **`Define`**

  A macro definition.

- **`Undef`**

  A macro undefinition.

- **`StartFile`**

  The start of a file.

- **`EndFile`**

  The end of the current included file.

- **`Import`**

  import a macro unit

- **`ImportSup`**

  import a macro unit from the supplementary object file

- **`VendorExt`**

  A vendor-specific extension.

#### Trait Implementations

##### `impl<R, Offset> Clone for MacroEntry<R, Offset>`

- <span id="macroentry-clone"></span>`fn clone(&self) -> MacroEntry<R, Offset>` — [`MacroEntry`](../index.md#macroentry)

##### `impl<R, Offset> Debug for MacroEntry<R, Offset>`

- <span id="macroentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for MacroEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for MacroEntry<R, Offset>`

- <span id="macroentry-partialeq-eq"></span>`fn eq(&self, other: &MacroEntry<R, Offset>) -> bool` — [`MacroEntry`](../index.md#macroentry)

##### `impl<R, Offset> StructuralPartialEq for MacroEntry<R, Offset>`

