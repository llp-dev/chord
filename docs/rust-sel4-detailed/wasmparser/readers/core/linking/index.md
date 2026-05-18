*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [linking](index.md)*

---

# Module `linking`

## Contents

- [Structs](#structs)
  - [`SymbolFlags`](#symbolflags)
  - [`SegmentFlags`](#segmentflags)
  - [`LinkingSectionReader`](#linkingsectionreader)
  - [`Segment`](#segment)
  - [`InitFunc`](#initfunc)
  - [`Comdat`](#comdat)
  - [`ComdatSymbol`](#comdatsymbol)
  - [`DefinedDataSymbol`](#defineddatasymbol)
- [Enums](#enums)
  - [`ComdatSymbolKind`](#comdatsymbolkind)
  - [`SymbolInfo`](#symbolinfo)
  - [`Linking`](#linking)
- [Type Aliases](#type-aliases)
  - [`SegmentMap`](#segmentmap)
  - [`InitFuncMap`](#initfuncmap)
  - [`ComdatMap`](#comdatmap)
  - [`SymbolInfoMap`](#symbolinfomap)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SymbolFlags`](#symbolflags) | struct | Flags for WebAssembly symbols. |
| [`SegmentFlags`](#segmentflags) | struct | Flags for WebAssembly segments. |
| [`LinkingSectionReader`](#linkingsectionreader) | struct | A reader for the `linking` custom section of a WebAssembly module. |
| [`Segment`](#segment) | struct | Represents extra metadata about the data segments. |
| [`InitFunc`](#initfunc) | struct | Represents an init function in the linking custom section. |
| [`Comdat`](#comdat) | struct | Represents [COMDAT](https://llvm.org/docs/LangRef.html#comdats) data in the linking custom section. |
| [`ComdatSymbol`](#comdatsymbol) | struct | Represents a symbol that is part of a comdat. |
| [`DefinedDataSymbol`](#defineddatasymbol) | struct | Represents the metadata about a data symbol defined in the wasm file. |
| [`ComdatSymbolKind`](#comdatsymbolkind) | enum | Represents a symbol kind. |
| [`SymbolInfo`](#symbolinfo) | enum | Represents extra information about symbols in the linking custom section. |
| [`Linking`](#linking) | enum | Represents a subsection read from the linking custom section. |
| [`SegmentMap`](#segmentmap) | type | Represents a reader for segments from the linking custom section. |
| [`InitFuncMap`](#initfuncmap) | type | Represents a reader for init functions from the linking custom section. |
| [`ComdatMap`](#comdatmap) | type | Represents a reader for COMDAT data from the linking custom section. |
| [`SymbolInfoMap`](#symbolinfomap) | type | Represents a reader for symbol info from the linking custom section. |

## Structs

### `SymbolFlags`

```rust
struct SymbolFlags(<SymbolFlags as __private::PublicFlags>::Internal);
```

Flags for WebAssembly symbols.

These flags correspond to those described in
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md>
with the `WASM_SYM_*` prefix.

#### Implementations

- <span id="symbolflags-const-binding-weak"></span>`const BINDING_WEAK: Self`

- <span id="symbolflags-const-binding-local"></span>`const BINDING_LOCAL: Self`

- <span id="symbolflags-const-visibility-hidden"></span>`const VISIBILITY_HIDDEN: Self`

- <span id="symbolflags-const-undefined"></span>`const UNDEFINED: Self`

- <span id="symbolflags-const-exported"></span>`const EXPORTED: Self`

- <span id="symbolflags-const-explicit-name"></span>`const EXPLICIT_NAME: Self`

- <span id="symbolflags-const-no-strip"></span>`const NO_STRIP: Self`

- <span id="symbolflags-const-tls"></span>`const TLS: Self`

- <span id="symbolflags-const-absolute"></span>`const ABSOLUTE: Self`

#### Trait Implementations

##### `impl Binary for SymbolFlags`

- <span id="symbolflags-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for SymbolFlags`

- <span id="symbolflags-bitand-type-output"></span>`type Output = SymbolFlags`

- <span id="symbolflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for SymbolFlags`

- <span id="symbolflags-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for SymbolFlags`

- <span id="symbolflags-bitor-type-output"></span>`type Output = SymbolFlags`

- <span id="symbolflags-bitor"></span>`fn bitor(self, other: SymbolFlags) -> Self` — [`SymbolFlags`](../index.md#symbolflags)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for SymbolFlags`

- <span id="symbolflags-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for SymbolFlags`

- <span id="symbolflags-bitxor-type-output"></span>`type Output = SymbolFlags`

- <span id="symbolflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for SymbolFlags`

- <span id="symbolflags-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl Clone for SymbolFlags`

- <span id="symbolflags-clone"></span>`fn clone(&self) -> SymbolFlags` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl Copy for SymbolFlags`

##### `impl Debug for SymbolFlags`

- <span id="symbolflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SymbolFlags`

- <span id="symbolflags-default"></span>`fn default() -> SymbolFlags` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl Eq for SymbolFlags`

##### `impl Extend for SymbolFlags`

- <span id="symbolflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for SymbolFlags`

- <span id="symbolflags-flags-const-flags"></span>`const FLAGS: &'static [Flag<SymbolFlags>]`

- <span id="symbolflags-flags-type-bits"></span>`type Bits = u32`

- <span id="symbolflags-flags-bits"></span>`fn bits(&self) -> u32`

- <span id="symbolflags-flags-from-bits-retain"></span>`fn from_bits_retain(bits: u32) -> SymbolFlags` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl FromIterator for SymbolFlags`

- <span id="symbolflags-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl FromReader for SymbolFlags`

- <span id="symbolflags-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for SymbolFlags`

- <span id="symbolflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for SymbolFlags`

- <span id="symbolflags-intoiterator-type-item"></span>`type Item = SymbolFlags`

- <span id="symbolflags-intoiterator-type-intoiter"></span>`type IntoIter = Iter<SymbolFlags>`

- <span id="symbolflags-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for SymbolFlags`

- <span id="symbolflags-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for SymbolFlags`

- <span id="symbolflags-not-type-output"></span>`type Output = SymbolFlags`

- <span id="symbolflags-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for SymbolFlags`

- <span id="symbolflags-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Ord for SymbolFlags`

- <span id="symbolflags-ord-cmp"></span>`fn cmp(&self, other: &SymbolFlags) -> cmp::Ordering` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl PartialEq for SymbolFlags`

- <span id="symbolflags-partialeq-eq"></span>`fn eq(&self, other: &SymbolFlags) -> bool` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl PartialOrd for SymbolFlags`

- <span id="symbolflags-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SymbolFlags) -> option::Option<cmp::Ordering>` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl PublicFlags for SymbolFlags`

- <span id="symbolflags-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="symbolflags-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for SymbolFlags`

##### `impl Sub for SymbolFlags`

- <span id="symbolflags-sub-type-output"></span>`type Output = SymbolFlags`

- <span id="symbolflags-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for SymbolFlags`

- <span id="symbolflags-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl UpperHex for SymbolFlags`

- <span id="symbolflags-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `SegmentFlags`

```rust
struct SegmentFlags(<SegmentFlags as __private::PublicFlags>::Internal);
```

Flags for WebAssembly segments.

These flags are defined by implementation at the time of writing:
<https://github.com/llvm/llvm-project/blob/llvmorg-17.0.6/llvm/include/llvm/BinaryFormat/Wasm.h#L391-L394>

#### Implementations

- <span id="segmentflags-const-strings"></span>`const STRINGS: Self`

- <span id="segmentflags-const-tls"></span>`const TLS: Self`

#### Trait Implementations

##### `impl Binary for SegmentFlags`

- <span id="segmentflags-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for SegmentFlags`

- <span id="segmentflags-bitand-type-output"></span>`type Output = SegmentFlags`

- <span id="segmentflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for SegmentFlags`

- <span id="segmentflags-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for SegmentFlags`

- <span id="segmentflags-bitor-type-output"></span>`type Output = SegmentFlags`

- <span id="segmentflags-bitor"></span>`fn bitor(self, other: SegmentFlags) -> Self` — [`SegmentFlags`](../index.md#segmentflags)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for SegmentFlags`

- <span id="segmentflags-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for SegmentFlags`

- <span id="segmentflags-bitxor-type-output"></span>`type Output = SegmentFlags`

- <span id="segmentflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for SegmentFlags`

- <span id="segmentflags-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl Clone for SegmentFlags`

- <span id="segmentflags-clone"></span>`fn clone(&self) -> SegmentFlags` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl Copy for SegmentFlags`

##### `impl Debug for SegmentFlags`

- <span id="segmentflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SegmentFlags`

- <span id="segmentflags-default"></span>`fn default() -> SegmentFlags` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl Eq for SegmentFlags`

##### `impl Extend for SegmentFlags`

- <span id="segmentflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for SegmentFlags`

- <span id="segmentflags-flags-const-flags"></span>`const FLAGS: &'static [Flag<SegmentFlags>]`

- <span id="segmentflags-flags-type-bits"></span>`type Bits = u32`

- <span id="segmentflags-flags-bits"></span>`fn bits(&self) -> u32`

- <span id="segmentflags-flags-from-bits-retain"></span>`fn from_bits_retain(bits: u32) -> SegmentFlags` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl FromIterator for SegmentFlags`

- <span id="segmentflags-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl FromReader for SegmentFlags`

- <span id="segmentflags-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for SegmentFlags`

- <span id="segmentflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for SegmentFlags`

- <span id="segmentflags-intoiterator-type-item"></span>`type Item = SegmentFlags`

- <span id="segmentflags-intoiterator-type-intoiter"></span>`type IntoIter = Iter<SegmentFlags>`

- <span id="segmentflags-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for SegmentFlags`

- <span id="segmentflags-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for SegmentFlags`

- <span id="segmentflags-not-type-output"></span>`type Output = SegmentFlags`

- <span id="segmentflags-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for SegmentFlags`

- <span id="segmentflags-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for SegmentFlags`

- <span id="segmentflags-partialeq-eq"></span>`fn eq(&self, other: &SegmentFlags) -> bool` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl PublicFlags for SegmentFlags`

- <span id="segmentflags-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="segmentflags-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for SegmentFlags`

##### `impl Sub for SegmentFlags`

- <span id="segmentflags-sub-type-output"></span>`type Output = SegmentFlags`

- <span id="segmentflags-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for SegmentFlags`

- <span id="segmentflags-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl UpperHex for SegmentFlags`

- <span id="segmentflags-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `LinkingSectionReader<'a>`

```rust
struct LinkingSectionReader<'a> {
    version: u32,
    subsections: crate::Subsections<'a, Linking<'a>>,
    range: core::ops::Range<usize>,
}
```

A reader for the `linking` custom section of a WebAssembly module.

This format is currently defined upstream at
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md>.

#### Fields

- **`version`**: `u32`

  The version of linking metadata contained in this section.

- **`subsections`**: `crate::Subsections<'a, Linking<'a>>`

  The subsections in this section.

- **`range`**: `core::ops::Range<usize>`

  The range of the entire section, including the version.

#### Implementations

- <span id="linkingsectionreader-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

  Creates a new reader for the linking section contents starting at

  `offset` within the original wasm file.

- <span id="linkingsectionreader-version"></span>`fn version(&self) -> u32`

  Returns the version of linking metadata contained in this section.

- <span id="linkingsectionreader-original-position"></span>`fn original_position(&self) -> usize`

  Returns the original byte offset of this section.

- <span id="linkingsectionreader-range"></span>`fn range(&self) -> Range<usize>`

  Returns the range, as byte offsets, of this section within the original

  wasm binary.

- <span id="linkingsectionreader-subsections"></span>`fn subsections(&self) -> Subsections<'a, Linking<'a>>` — [`Subsections`](../../../index.md#subsections), [`Linking`](../index.md#linking)

  Returns the iterator for advancing through the subsections.

  

  You can also use `IntoIterator::into_iter` directly on this type.

#### Trait Implementations

##### `impl Clone for LinkingSectionReader<'a>`

- <span id="linkingsectionreader-clone"></span>`fn clone(&self) -> LinkingSectionReader<'a>` — [`LinkingSectionReader`](../index.md#linkingsectionreader)

##### `impl Debug for LinkingSectionReader<'a>`

- <span id="linkingsectionreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for LinkingSectionReader<'a>`

- <span id="linkingsectionreader-intoiterator-type-item"></span>`type Item = Result<Linking<'a>, BinaryReaderError>`

- <span id="linkingsectionreader-intoiterator-type-intoiter"></span>`type IntoIter = Subsections<'a, Linking<'a>>`

- <span id="linkingsectionreader-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

### `Segment<'a>`

```rust
struct Segment<'a> {
    pub name: &'a str,
    pub alignment: u32,
    pub flags: SegmentFlags,
}
```

Represents extra metadata about the data segments.

#### Fields

- **`name`**: `&'a str`

  The name for the segment.

- **`alignment`**: `u32`

  The required alignment of the segment, encoded as a power of 2.

- **`flags`**: `SegmentFlags`

  The flags for the segment.

#### Trait Implementations

##### `impl Clone for Segment<'a>`

- <span id="segment-clone"></span>`fn clone(&self) -> Segment<'a>` — [`Segment`](../index.md#segment)

##### `impl Copy for Segment<'a>`

##### `impl Debug for Segment<'a>`

- <span id="segment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Segment<'a>`

- <span id="segment-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

### `InitFunc`

```rust
struct InitFunc {
    pub priority: u32,
    pub symbol_index: u32,
}
```

Represents an init function in the linking custom section.

#### Fields

- **`priority`**: `u32`

  The priority of the init function.

- **`symbol_index`**: `u32`

  The symbol index of init function (*not* the function index).

#### Trait Implementations

##### `impl Clone for InitFunc`

- <span id="initfunc-clone"></span>`fn clone(&self) -> InitFunc` — [`InitFunc`](../index.md#initfunc)

##### `impl Copy for InitFunc`

##### `impl Debug for InitFunc`

- <span id="initfunc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for InitFunc`

- <span id="initfunc-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

### `Comdat<'a>`

```rust
struct Comdat<'a> {
    pub name: &'a str,
    pub flags: u32,
    pub symbols: crate::SectionLimited<'a, ComdatSymbol>,
}
```

Represents [COMDAT](https://llvm.org/docs/LangRef.html#comdats) data in the linking custom section.

#### Fields

- **`name`**: `&'a str`

  The name of this comdat.

- **`flags`**: `u32`

  The flags.

- **`symbols`**: `crate::SectionLimited<'a, ComdatSymbol>`

  The member symbols of this comdat.

#### Trait Implementations

##### `impl Clone for Comdat<'a>`

- <span id="comdat-clone"></span>`fn clone(&self) -> Comdat<'a>` — [`Comdat`](../index.md#comdat)

##### `impl Debug for Comdat<'a>`

- <span id="comdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Comdat<'a>`

- <span id="comdat-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

### `ComdatSymbol`

```rust
struct ComdatSymbol {
    pub kind: ComdatSymbolKind,
    pub index: u32,
}
```

Represents a symbol that is part of a comdat.

#### Fields

- **`kind`**: `ComdatSymbolKind`

  The kind of the symbol.

- **`index`**: `u32`

  The index of the symbol. Must not be an import.

#### Trait Implementations

##### `impl Clone for ComdatSymbol`

- <span id="comdatsymbol-clone"></span>`fn clone(&self) -> ComdatSymbol` — [`ComdatSymbol`](../index.md#comdatsymbol)

##### `impl Copy for ComdatSymbol`

##### `impl Debug for ComdatSymbol`

- <span id="comdatsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for ComdatSymbol`

- <span id="comdatsymbol-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

### `DefinedDataSymbol`

```rust
struct DefinedDataSymbol {
    pub index: u32,
    pub offset: u32,
    pub size: u32,
}
```

Represents the metadata about a data symbol defined in the wasm file.

#### Fields

- **`index`**: `u32`

  The index of the data segment.

- **`offset`**: `u32`

  The offset within the segment. Must be <= the segment's size.

- **`size`**: `u32`

  The size of the data, which can be zero. `offset + size` must be <= the segment's size.

#### Trait Implementations

##### `impl Clone for DefinedDataSymbol`

- <span id="defineddatasymbol-clone"></span>`fn clone(&self) -> DefinedDataSymbol` — [`DefinedDataSymbol`](../index.md#defineddatasymbol)

##### `impl Copy for DefinedDataSymbol`

##### `impl Debug for DefinedDataSymbol`

- <span id="defineddatasymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for DefinedDataSymbol`

- <span id="defineddatasymbol-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Enums

### `ComdatSymbolKind`

```rust
enum ComdatSymbolKind {
    Data,
    Func,
    Global,
    Event,
    Table,
    Section,
}
```

Represents a symbol kind.

#### Variants

- **`Data`**

  The symbol is a data segment.

- **`Func`**

  The symbol is a function.

- **`Global`**

  The symbol is a global.

- **`Event`**

  The symbol is an event.

- **`Table`**

  The symbol is a table.

- **`Section`**

  The symbol is a section.

#### Trait Implementations

##### `impl Clone for ComdatSymbolKind`

- <span id="comdatsymbolkind-clone"></span>`fn clone(&self) -> ComdatSymbolKind` — [`ComdatSymbolKind`](../index.md#comdatsymbolkind)

##### `impl Copy for ComdatSymbolKind`

##### `impl Debug for ComdatSymbolKind`

- <span id="comdatsymbolkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ComdatSymbolKind`

##### `impl FromReader for ComdatSymbolKind`

- <span id="comdatsymbolkind-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for ComdatSymbolKind`

- <span id="comdatsymbolkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ComdatSymbolKind`

- <span id="comdatsymbolkind-partialeq-eq"></span>`fn eq(&self, other: &ComdatSymbolKind) -> bool` — [`ComdatSymbolKind`](../index.md#comdatsymbolkind)

##### `impl StructuralPartialEq for ComdatSymbolKind`

### `SymbolInfo<'a>`

```rust
enum SymbolInfo<'a> {
    Func {
        flags: SymbolFlags,
        index: u32,
        name: Option<&'a str>,
    },
    Data {
        flags: SymbolFlags,
        name: &'a str,
        symbol: Option<DefinedDataSymbol>,
    },
    Global {
        flags: SymbolFlags,
        index: u32,
        name: Option<&'a str>,
    },
    Section {
        flags: SymbolFlags,
        section: u32,
    },
    Event {
        flags: SymbolFlags,
        index: u32,
        name: Option<&'a str>,
    },
    Table {
        flags: SymbolFlags,
        index: u32,
        name: Option<&'a str>,
    },
}
```

Represents extra information about symbols in the linking custom section.

The symbol flags correspond to those described in
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md>
with the `WASM_SYM_*` prefix.

#### Variants

- **`Func`**

  The symbol is a function.

- **`Data`**

  The symbol is a data symbol.

- **`Global`**

  The symbol is a global.

- **`Section`**

  The symbol is a section.

- **`Event`**

  The symbol is an event.

- **`Table`**

  The symbol is a table.

#### Trait Implementations

##### `impl Clone for SymbolInfo<'a>`

- <span id="symbolinfo-clone"></span>`fn clone(&self) -> SymbolInfo<'a>` — [`SymbolInfo`](../index.md#symbolinfo)

##### `impl Copy for SymbolInfo<'a>`

##### `impl Debug for SymbolInfo<'a>`

- <span id="symbolinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for SymbolInfo<'a>`

- <span id="symbolinfo-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

### `Linking<'a>`

```rust
enum Linking<'a> {
    SegmentInfo(SegmentMap<'a>),
    InitFuncs(InitFuncMap<'a>),
    ComdatInfo(ComdatMap<'a>),
    SymbolTable(SymbolInfoMap<'a>),
    Unknown {
        ty: u8,
        data: &'a [u8],
        range: core::ops::Range<usize>,
    },
}
```

Represents a subsection read from the linking custom section.

#### Variants

- **`SegmentInfo`**

  Extra metadata about the data segments.

- **`InitFuncs`**

  A list of constructor functions to be called at startup.

- **`ComdatInfo`**

  The [COMDAT](https://llvm.org/docs/LangRef.html#comdats) groups of associated linking objects.

- **`SymbolTable`**

  Extra information about the symbols present in the module.

- **`Unknown`**

  An unknown [linking subsection](https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#linking-metadata-section).

#### Trait Implementations

##### `impl Clone for Linking<'a>`

- <span id="linking-clone"></span>`fn clone(&self) -> Linking<'a>` — [`Linking`](../index.md#linking)

##### `impl Debug for Linking<'a>`

- <span id="linking-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Subsection for Linking<'a>`

- <span id="linking-subsection-from-reader"></span>`fn from_reader(id: u8, reader: BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Type Aliases

### `SegmentMap<'a>`

```rust
type SegmentMap<'a> = crate::SectionLimited<'a, Segment<'a>>;
```

Represents a reader for segments from the linking custom section.

### `InitFuncMap<'a>`

```rust
type InitFuncMap<'a> = crate::SectionLimited<'a, InitFunc>;
```

Represents a reader for init functions from the linking custom section.

### `ComdatMap<'a>`

```rust
type ComdatMap<'a> = crate::SectionLimited<'a, Comdat<'a>>;
```

Represents a reader for COMDAT data from the linking custom section.

### `SymbolInfoMap<'a>`

```rust
type SymbolInfoMap<'a> = crate::SectionLimited<'a, SymbolInfo<'a>>;
```

Represents a reader for symbol info from the linking custom section.

