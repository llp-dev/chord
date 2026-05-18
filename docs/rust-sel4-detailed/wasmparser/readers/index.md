*[wasmparser](../index.md) / [readers](index.md)*

---

# Module `readers`

## Contents

- [Modules](#modules)
  - [`core`](#core)
  - [`branch_hinting`](#branch-hinting)
  - [`code`](#code)
  - [`coredumps`](#coredumps)
  - [`custom`](#custom)
  - [`data`](#data)
  - [`dylink0`](#dylink0)
  - [`elements`](#elements)
  - [`exports`](#exports)
  - [`functions`](#functions)
  - [`globals`](#globals)
  - [`imports`](#imports)
  - [`init`](#init)
  - [`linking`](#linking)
  - [`memories`](#memories)
  - [`names`](#names)
  - [`operators`](#operators)
  - [`producers`](#producers)
  - [`reloc`](#reloc)
  - [`tables`](#tables)
  - [`tags`](#tags)
  - [`types`](#types)
- [Structs](#structs)
  - [`SectionLimited`](#sectionlimited)
  - [`SectionLimitedIntoIter`](#sectionlimitedintoiter)
  - [`SectionLimitedIntoIterWithOffsets`](#sectionlimitedintoiterwithoffsets)
  - [`Subsections`](#subsections)
- [Traits](#traits)
  - [`FromReader`](#fromreader)
  - [`Subsection`](#subsection)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`core`](#core) | mod |  |
| [`branch_hinting`](#branch-hinting) | mod |  |
| [`code`](#code) | mod |  |
| [`coredumps`](#coredumps) | mod |  |
| [`custom`](#custom) | mod |  |
| [`data`](#data) | mod |  |
| [`dylink0`](#dylink0) | mod |  |
| [`elements`](#elements) | mod |  |
| [`exports`](#exports) | mod |  |
| [`functions`](#functions) | mod |  |
| [`globals`](#globals) | mod |  |
| [`imports`](#imports) | mod |  |
| [`init`](#init) | mod |  |
| [`linking`](#linking) | mod |  |
| [`memories`](#memories) | mod |  |
| [`names`](#names) | mod |  |
| [`operators`](#operators) | mod |  |
| [`producers`](#producers) | mod |  |
| [`reloc`](#reloc) | mod |  |
| [`tables`](#tables) | mod |  |
| [`tags`](#tags) | mod |  |
| [`types`](#types) | mod |  |
| [`SectionLimited`](#sectionlimited) | struct | A generic structure for reading a section of a WebAssembly binary which has a limited number of items within it. |
| [`SectionLimitedIntoIter`](#sectionlimitedintoiter) | struct | A consuming iterator of a [`SectionLimited`]. |
| [`SectionLimitedIntoIterWithOffsets`](#sectionlimitedintoiterwithoffsets) | struct | An iterator over a limited section iterator. |
| [`Subsections`](#subsections) | struct | Iterator/reader over the contents of a section which is composed of subsections. |
| [`FromReader`](#fromreader) | trait | A trait implemented for items that can be decoded directly from a `BinaryReader`, or that which can be parsed from the WebAssembly binary format. |
| [`Subsection`](#subsection) | trait | A trait implemented for subsections of another outer section. |

## Modules

- [`core`](core/index.md)
- [`branch_hinting`](branch_hinting/index.md)
- [`code`](code/index.md)
- [`coredumps`](coredumps/index.md)
- [`custom`](custom/index.md)
- [`data`](data/index.md)
- [`dylink0`](dylink0/index.md)
- [`elements`](elements/index.md)
- [`exports`](exports/index.md)
- [`functions`](functions/index.md)
- [`globals`](globals/index.md)
- [`imports`](imports/index.md)
- [`init`](init/index.md)
- [`linking`](linking/index.md)
- [`memories`](memories/index.md)
- [`names`](names/index.md)
- [`operators`](operators/index.md)
- [`producers`](producers/index.md)
- [`reloc`](reloc/index.md)
- [`tables`](tables/index.md)
- [`tags`](tags/index.md)
- [`types`](types/index.md)

## Structs

### `SectionLimited<'a, T>`

```rust
struct SectionLimited<'a, T> {
    reader: crate::BinaryReader<'a>,
    count: u32,
    _marker: marker::PhantomData<T>,
}
```

A generic structure for reading a section of a WebAssembly binary which has
a limited number of items within it.

Many WebAssembly sections are a count of items followed by that many items.
This helper structure can be used to parse these sections and provides
an iteration-based API for reading the contents.

Note that this always implements the `Clone` trait to represent the
ability to parse the section multiple times.

#### Implementations

- <span id="sectionlimited-into-iter-err-on-gc-types"></span>`fn into_iter_err_on_gc_types(self) -> impl Iterator<Item = Result<FuncType>> + 'a` — [`Result`](../binary_reader/index.md#result), [`FuncType`](core/index.md#functype)

  Returns an iterator over this type section which will only yield

  function types and any usage of GC types from the GC proposal will

  be translated into an error.

#### Trait Implementations

##### `impl<T> Clone for SectionLimited<'_, T>`

- <span id="sectionlimited-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Debug for SectionLimited<'_, T>`

- <span id="sectionlimited-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoIterator for SectionLimited<'a, T>`

- <span id="sectionlimited-intoiterator-type-item"></span>`type Item = Result<T, BinaryReaderError>`

- <span id="sectionlimited-intoiterator-type-intoiter"></span>`type IntoIter = SectionLimitedIntoIter<'a, T>`

- <span id="sectionlimited-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

### `SectionLimitedIntoIter<'a, T>`

```rust
struct SectionLimitedIntoIter<'a, T> {
    section: SectionLimited<'a, T>,
    remaining: u32,
    end: bool,
}
```

A consuming iterator of a [`SectionLimited`](../index.md).

This is created via the `IntoIterator` `impl` for the [`SectionLimited`](../index.md)
type.

#### Implementations

- <span id="sectionlimitedintoiter-original-position"></span>`fn original_position(&self) -> usize`

  Returns the current byte offset of the section within this iterator.

#### Trait Implementations

##### `impl<T> ExactSizeIterator for SectionLimitedIntoIter<'a, T>`

##### `impl IntoIterator for SectionLimitedIntoIter<'a, T>`

- <span id="sectionlimitedintoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectionlimitedintoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectionlimitedintoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for SectionLimitedIntoIter<'a, T>`

- <span id="sectionlimitedintoiter-iterator-type-item"></span>`type Item = Result<T, BinaryReaderError>`

- <span id="sectionlimitedintoiter-iterator-next"></span>`fn next(&mut self) -> Option<Result<T>>` — [`Result`](../binary_reader/index.md#result)

- <span id="sectionlimitedintoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `SectionLimitedIntoIterWithOffsets<'a, T>`

```rust
struct SectionLimitedIntoIterWithOffsets<'a, T> {
    iter: SectionLimitedIntoIter<'a, T>,
}
```

An iterator over a limited section iterator.

#### Trait Implementations

##### `impl<T> ExactSizeIterator for SectionLimitedIntoIterWithOffsets<'a, T>`

##### `impl IntoIterator for SectionLimitedIntoIterWithOffsets<'a, T>`

- <span id="sectionlimitedintoiterwithoffsets-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectionlimitedintoiterwithoffsets-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectionlimitedintoiterwithoffsets-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for SectionLimitedIntoIterWithOffsets<'a, T>`

- <span id="sectionlimitedintoiterwithoffsets-iterator-type-item"></span>`type Item = Result<(usize, T), BinaryReaderError>`

- <span id="sectionlimitedintoiterwithoffsets-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="sectionlimitedintoiterwithoffsets-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Subsections<'a, T>`

```rust
struct Subsections<'a, T> {
    reader: crate::BinaryReader<'a>,
    _marker: marker::PhantomData<T>,
}
```

Iterator/reader over the contents of a section which is composed of
subsections.

This reader is used for the core `name` section, for example. This type
primarily implements [`Iterator`](../../fallible_iterator/index.md) for advancing through the sections.

#### Implementations

- <span id="subsections-new"></span>`fn new(reader: BinaryReader<'a>) -> Self` — [`BinaryReader`](../binary_reader/index.md#binaryreader)

  Creates a new reader for the specified section contents starting at

  `offset` within the original wasm file.

- <span id="subsections-original-position"></span>`fn original_position(&self) -> usize`

  Returns whether the original byte offset of this section.

- <span id="subsections-range"></span>`fn range(&self) -> Range<usize>`

  Returns the range, as byte offsets, of this section within the original

  wasm binary.

- <span id="subsections-read"></span>`fn read(&mut self) -> Result<T>` — [`Result`](../binary_reader/index.md#result)

#### Trait Implementations

##### `impl<T> Clone for Subsections<'_, T>`

- <span id="subsections-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Debug for Subsections<'_, T>`

- <span id="subsections-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for Subsections<'a, T>`

- <span id="subsections-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="subsections-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="subsections-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Subsections<'a, T>`

- <span id="subsections-iterator-type-item"></span>`type Item = Result<T, BinaryReaderError>`

- <span id="subsections-iterator-next"></span>`fn next(&mut self) -> Option<Result<T>>` — [`Result`](../binary_reader/index.md#result)

## Traits

### `FromReader<'a>`

```rust
trait FromReader<'a>: Sized { ... }
```

A trait implemented for items that can be decoded directly from a
`BinaryReader`, or that which can be parsed from the WebAssembly binary
format.

Note that this is also accessible as a `BinaryReader::read` method.

#### Required Methods

- `fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>`

  Attempts to read `Self` from the provided binary reader, returning an

#### Implementors

- [`AbstractHeapType`](core/index.md#abstractheaptype)
- [`ArrayType`](core/index.md#arraytype)
- [`BranchHintFunction`](core/index.md#branchhintfunction)
- [`BranchHint`](core/index.md#branchhint)
- [`Catch`](core/index.md#catch)
- [`ComdatSymbolKind`](core/index.md#comdatsymbolkind)
- [`ComdatSymbol`](core/index.md#comdatsymbol)
- [`Comdat`](core/index.md#comdat)
- [`CompositeType`](core/index.md#compositetype)
- [`ConstExpr`](core/index.md#constexpr)
- [`ContType`](core/index.md#conttype)
- [`CoreDumpInstance`](core/index.md#coredumpinstance)
- [`CoreDumpStackFrame`](core/index.md#coredumpstackframe)
- [`CoreDumpValue`](core/index.md#coredumpvalue)
- [`Data`](core/index.md#data)
- [`DefinedDataSymbol`](core/index.md#defineddatasymbol)
- [`Element`](core/index.md#element)
- [`Export`](core/index.md#export)
- [`ExternalKind`](core/index.md#externalkind)
- [`FieldType`](core/index.md#fieldtype)
- [`FuncType`](core/index.md#functype)
- [`FunctionBody`](core/index.md#functionbody)
- [`GlobalType`](core/index.md#globaltype)
- [`Global`](core/index.md#global)
- [`Handle`](core/index.md#handle)
- [`HeapType`](core/index.md#heaptype)
- [`Import`](core/index.md#import)
- [`IndirectNaming`](core/index.md#indirectnaming)
- [`InitFunc`](core/index.md#initfunc)
- [`MemoryType`](core/index.md#memorytype)
- [`Naming`](core/index.md#naming)
- [`ProducersFieldValue`](core/index.md#producersfieldvalue)
- [`ProducersField`](core/index.md#producersfield)
- [`RecGroup`](core/index.md#recgroup)
- [`RefType`](core/index.md#reftype)
- [`RelocationEntry`](core/index.md#relocationentry)
- [`RelocationType`](core/index.md#relocationtype)
- [`ResumeTable`](core/index.md#resumetable)
- [`SegmentFlags`](core/index.md#segmentflags)
- [`Segment`](core/index.md#segment)
- [`StorageType`](core/index.md#storagetype)
- [`StructType`](core/index.md#structtype)
- [`SubType`](core/index.md#subtype)
- [`SymbolFlags`](core/index.md#symbolflags)
- [`SymbolInfo`](core/index.md#symbolinfo)
- [`TableType`](core/index.md#tabletype)
- [`Table`](core/index.md#table)
- [`TagType`](core/index.md#tagtype)
- [`TryTable`](core/index.md#trytable)
- [`TypeRef`](core/index.md#typeref)
- [`ValType`](core/index.md#valtype)
- `&'a str`
- `(T, U)`
- `bool`
- `u32`

### `Subsection<'a>`

```rust
trait Subsection<'a>: Sized { ... }
```

A trait implemented for subsections of another outer section.

This is currently only used for subsections within custom sections, such as
the `name` section of core wasm.

This is used in conjunction with [`Subsections`](../index.md).

#### Required Methods

- `fn from_reader(id: u8, reader: BinaryReader<'a>) -> Result<Self>`

  Converts the section identifier provided with the section contents into

#### Implementors

- [`Dylink0Subsection`](core/index.md#dylink0subsection)
- [`Linking`](core/index.md#linking)
- [`Name`](core/index.md#name)

