# Crate `addr2line`

`addr2line` provides a cross-platform library for retrieving per-address debug information
from files with DWARF debug information. Given an address, it can return the file name,
line number, and function name associated with that address, as well as the inline call
stack leading to that address.

At the lowest level, the library uses a [`Context`](#context) to cache parsed information so that
multiple lookups are efficient. To create a `Context`, you first need to open and parse the
file using an object file parser such as [`object`](https://github.com/gimli-rs/object),
create a [`gimli::Dwarf`](../gimli/read/index.md), and finally call `Context::from_dwarf`.

Location information is obtained with `Context::find_location` or
`Context::find_location_range`. Function information is obtained with
`Context::find_frames`, which returns a frame for each inline function. Each frame
contains both name and location.

The library also provides a `Loader` which internally memory maps the files,
uses the `object` crate to do the parsing, and creates a `Context`.
The `Context` is not exposed, but the `Loader` provides the same functionality
via `Loader::find_location`, `Loader::find_location_range`, and
`Loader::find_frames`. The `Loader` also provides `Loader::find_symbol`
to use the symbol table instead of DWARF debugging information.
The `Loader` will load Mach-O dSYM files and split DWARF files as needed.

The crate has a CLI wrapper around the library which provides some of
the functionality of the `addr2line` command line tool distributed with
[GNU binutils](https://sourceware.org/binutils/docs/binutils/addr2line.html).

## Contents

- [Modules](#modules)
  - [`maybe_small`](#maybe-small)
  - [`frame`](#frame)
  - [`function`](#function)
  - [`line`](#line)
  - [`lookup`](#lookup)
  - [`unit`](#unit)
- [Structs](#structs)
  - [`Frame`](#frame)
  - [`FrameIter`](#frameiter)
  - [`FunctionName`](#functionname)
  - [`Location`](#location)
  - [`SplitDwarfLoad`](#splitdwarfload)
  - [`LocationRangeIter`](#locationrangeiter)
  - [`Context`](#context)
  - [`RangeAttributes`](#rangeattributes)
- [Enums](#enums)
  - [`LookupResult`](#lookupresult)
  - [`DebugFile`](#debugfile)
- [Traits](#traits)
  - [`LookupContinuation`](#lookupcontinuation)
- [Functions](#functions)
  - [`demangle`](#demangle)
  - [`demangle_auto`](#demangle-auto)
- [Type Aliases](#type-aliases)
  - [`Error`](#error)
  - [`LazyResult`](#lazyresult)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`maybe_small`](#maybe-small) | mod |  |
| [`frame`](#frame) | mod |  |
| [`function`](#function) | mod |  |
| [`line`](#line) | mod |  |
| [`lookup`](#lookup) | mod |  |
| [`unit`](#unit) | mod |  |
| [`Frame`](#frame) | struct |  |
| [`FrameIter`](#frameiter) | struct |  |
| [`FunctionName`](#functionname) | struct |  |
| [`Location`](#location) | struct |  |
| [`SplitDwarfLoad`](#splitdwarfload) | struct |  |
| [`LocationRangeIter`](#locationrangeiter) | struct |  |
| [`Context`](#context) | struct | The state necessary to perform address to line translation. |
| [`RangeAttributes`](#rangeattributes) | struct |  |
| [`LookupResult`](#lookupresult) | enum |  |
| [`DebugFile`](#debugfile) | enum |  |
| [`LookupContinuation`](#lookupcontinuation) | trait |  |
| [`demangle`](#demangle) | fn |  |
| [`demangle_auto`](#demangle-auto) | fn |  |
| [`Error`](#error) | type |  |
| [`LazyResult`](#lazyresult) | type |  |

## Modules

- [`maybe_small`](maybe_small/index.md)
- [`frame`](frame/index.md)
- [`function`](function/index.md)
- [`line`](line/index.md)
- [`lookup`](lookup/index.md)
- [`unit`](unit/index.md)

## Structs

### `Frame<'ctx, R: gimli::Reader>`

```rust
struct Frame<'ctx, R: gimli::Reader> {
    pub dw_die_offset: Option<gimli::UnitOffset<<R as >::Offset>>,
    pub function: Option<FunctionName<R>>,
    pub location: Option<Location<'ctx>>,
}
```

A function frame.

#### Fields

- **`dw_die_offset`**: `Option<gimli::UnitOffset<<R as >::Offset>>`

  The DWARF unit offset corresponding to the DIE of the function.

- **`function`**: `Option<FunctionName<R>>`

  The name of the function.

- **`location`**: `Option<Location<'ctx>>`

  The source location corresponding to this frame.

### `FrameIter<'ctx, R>`

```rust
struct FrameIter<'ctx, R>(FrameIterState<'ctx, R>)
where
    R: gimli::Reader;
```

An iterator over function frames.

#### Implementations

- <span id="frameiter-new-empty"></span>`fn new_empty() -> Self`

- <span id="frameiter-new-location"></span>`fn new_location(location: Location<'ctx>) -> Self` — [`Location`](frame/index.md#location)

- <span id="frameiter-new-frames"></span>`fn new_frames(unit: &'ctx ResUnit<R>, sections: &'ctx gimli::Dwarf<R>, function: &'ctx Function<R>, inlined_functions: smallvec::SmallVec<[&'ctx InlinedFunction<R>; 16]>, location: Option<Location<'ctx>>) -> Self` — [`ResUnit`](unit/index.md#resunit), [`Function`](function/index.md#function), [`InlinedFunction`](function/index.md#inlinedfunction), [`Location`](frame/index.md#location)

- <span id="frameiter-next"></span>`fn next(&mut self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>` — [`Frame`](frame/index.md#frame)

  Advances the iterator and returns the next frame.

#### Trait Implementations

##### `impl<R> FallibleIterator for FrameIter<'ctx, R>`

- <span id="frameiter-fallibleiterator-type-item"></span>`type Item = Frame<'ctx, R>`

- <span id="frameiter-fallibleiterator-type-error"></span>`type Error = Error`

- <span id="frameiter-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>` — [`Frame`](frame/index.md#frame)

##### `impl IntoFallibleIterator for FrameIter<'ctx, R>`

- <span id="frameiter-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="frameiter-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="frameiter-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="frameiter-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `FunctionName<R: gimli::Reader>`

```rust
struct FunctionName<R: gimli::Reader> {
    pub name: R,
    pub language: Option<gimli::DwLang>,
}
```

A function name.

#### Fields

- **`name`**: `R`

  The name of the function.

- **`language`**: `Option<gimli::DwLang>`

  The language of the compilation unit containing this function.

#### Implementations

- <span id="functionname-raw-name"></span>`fn raw_name(&self) -> Result<Cow<'_, str>, gimli::Error>`

  The raw name of this function before demangling.

- <span id="functionname-demangle"></span>`fn demangle(&self) -> Result<Cow<'_, str>, gimli::Error>`

  The name of this function after demangling (if applicable).

### `Location<'a>`

```rust
struct Location<'a> {
    pub file: Option<&'a str>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}
```

A source location.

#### Fields

- **`file`**: `Option<&'a str>`

  The file name.

- **`line`**: `Option<u32>`

  The line number.

- **`column`**: `Option<u32>`

  The column number.
  
  A value of `Some(0)` indicates the left edge.

### `SplitDwarfLoad<R>`

```rust
struct SplitDwarfLoad<R> {
    pub dwo_id: gimli::DwoId,
    pub comp_dir: Option<R>,
    pub path: Option<R>,
    pub parent: alloc::sync::Arc<gimli::Dwarf<R>>,
}
```

This struct contains the information needed to find split DWARF data
and to produce a `gimli::Dwarf<R>` for it.

#### Fields

- **`dwo_id`**: `gimli::DwoId`

  The dwo id, for looking up in a DWARF package, or for
  verifying an unpacked dwo found on the file system

- **`comp_dir`**: `Option<R>`

  The compilation directory `path` is relative to.

- **`path`**: `Option<R>`

  A path on the filesystem, relative to `comp_dir` to find this dwo.

- **`parent`**: `alloc::sync::Arc<gimli::Dwarf<R>>`

  Once the split DWARF data is loaded, the loader is expected
  to call [make_dwo(parent)](gimli::read::Dwarf::make_dwo) before
  returning the data.

### `LocationRangeIter<'ctx, R: gimli::Reader>`

```rust
struct LocationRangeIter<'ctx, R: gimli::Reader> {
    unit_iter: alloc::boxed::Box<dyn Iterator<Item = (&'ctx ResUnit<R>, &'ctx gimli::Range)>>,
    iter: Option<crate::line::LineLocationRangeIter<'ctx>>,
    probe_low: u64,
    probe_high: u64,
    sections: &'ctx gimli::Dwarf<R>,
}
```

Iterator over `Location`s in a range of addresses, returned by `Context::find_location_range`.

#### Implementations

- <span id="locationrangeiter-next-loc"></span>`fn next_loc(&mut self) -> Result<Option<(u64, u64, Location<'ctx>)>, gimli::Error>` — [`Location`](frame/index.md#location)

#### Trait Implementations

##### `impl<R> FallibleIterator for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-fallibleiterator-type-item"></span>`type Item = (u64, u64, Location<'ctx>)`

- <span id="locationrangeiter-fallibleiterator-type-error"></span>`type Error = Error`

- <span id="locationrangeiter-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

##### `impl IntoFallibleIterator for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="locationrangeiter-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="locationrangeiter-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="locationrangeiter-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

##### `impl IntoIterator for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="locationrangeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="locationrangeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R> Iterator for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-iterator-type-item"></span>`type Item = (u64, u64, Location<'ctx>)`

- <span id="locationrangeiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl IteratorExt for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-iteratorext-transpose-into-fallible"></span>`fn transpose_into_fallible<T, E>(self) -> Convert<I>`

  Convert an iterator of `Result`s into `FallibleIterator` by transposition

- <span id="locationrangeiter-iteratorext-into-fallible"></span>`fn into_fallible<T>(self) -> IntoFallible<I>`

  Convert an iterator of anything into `FallibleIterator` by wrapping

  into `Result<T, Infallible>` where `Infallible` is an error that can never actually

  happen.

### `Context<R: gimli::Reader>`

```rust
struct Context<R: gimli::Reader> {
    sections: alloc::sync::Arc<gimli::Dwarf<R>>,
    units: crate::unit::ResUnits<R>,
    sup_units: crate::unit::SupUnits<R>,
}
```

The state necessary to perform address to line translation.

Constructing a `Context` is somewhat costly, so users should aim to reuse `Context`s
when performing lookups for many addresses in the same executable.

#### Implementations

- <span id="context-from-sections"></span>`fn from_sections(debug_abbrev: gimli::DebugAbbrev<R>, debug_addr: gimli::DebugAddr<R>, debug_aranges: gimli::DebugAranges<R>, debug_info: gimli::DebugInfo<R>, debug_line: gimli::DebugLine<R>, debug_line_str: gimli::DebugLineStr<R>, debug_ranges: gimli::DebugRanges<R>, debug_rnglists: gimli::DebugRngLists<R>, debug_str: gimli::DebugStr<R>, debug_str_offsets: gimli::DebugStrOffsets<R>, default_section: R) -> Result<Self, gimli::Error>`

  Construct a new `Context` from DWARF sections.

  

  This method does not support using a supplementary object file.

- <span id="context-from-dwarf"></span>`fn from_dwarf(sections: gimli::Dwarf<R>) -> Result<Context<R>, gimli::Error>` — [`Context`](#context)

  Construct a new `Context` from an existing [`gimli::Dwarf`](../gimli/read/index.md) object.

- <span id="context-from-arc-dwarf"></span>`fn from_arc_dwarf(sections: Arc<gimli::Dwarf<R>>) -> Result<Context<R>, gimli::Error>` — [`Context`](#context)

  Construct a new `Context` from an existing [`gimli::Dwarf`](../gimli/read/index.md) object.

### `RangeAttributes<R: gimli::Reader>`

```rust
struct RangeAttributes<R: gimli::Reader> {
    low_pc: Option<u64>,
    high_pc: Option<u64>,
    size: Option<u64>,
    ranges_offset: Option<gimli::RangeListsOffset<<R as gimli::Reader>::Offset>>,
}
```

#### Implementations

- <span id="rangeattributes-for-each-range"></span>`fn for_each_range<F: FnMut(gimli::Range)>(&self, unit: gimli::UnitRef<'_, R>, f: F) -> Result<bool, gimli::Error>`

#### Trait Implementations

##### `impl<R: gimli::Reader> Default for RangeAttributes<R>`

- <span id="rangeattributes-default"></span>`fn default() -> Self`

## Enums

### `LookupResult<L: LookupContinuation>`

```rust
enum LookupResult<L: LookupContinuation> {
    Load {
        load: SplitDwarfLoad<<L as LookupContinuation>::Buf>,
        continuation: L,
    },
    Output(<L as LookupContinuation>::Output),
}
```

Operations that consult debug information may require additional files
to be loaded if split DWARF is being used. This enum returns the result
of the operation in the `Output` variant, or information about the split
DWARF that is required and a continuation to invoke once it is available
in the `Load` variant.

This enum is intended to be used in a loop like so:
```no_run
  use addr2line::*;
  use std::sync::Arc;
  let ctx: Context<gimli::EndianSlice<gimli::RunTimeEndian>> = todo!();
  let do_split_dwarf_load = |load: SplitDwarfLoad<gimli::EndianSlice<gimli::RunTimeEndian>>| -> Option<Arc<gimli::Dwarf<gimli::EndianSlice<gimli::RunTimeEndian>>>> { None };
  const ADDRESS: u64 = 0xdeadbeef;
  let mut r = ctx.find_frames(ADDRESS);
  let result = loop {
    match r {
      LookupResult::Output(result) => break result,
      LookupResult::Load { load, continuation } => {
        let dwo = do_split_dwarf_load(load);
        r = continuation.resume(dwo);
      }
    }
  };
```

#### Variants

- **`Load`**

  The lookup requires split DWARF data to be loaded.

- **`Output`**

  The lookup has completed and produced an output.

#### Implementations

- <span id="lookupresult-skip-all-loads"></span>`fn skip_all_loads(self) -> <L as >::Output` — [`LookupContinuation`](lookup/index.md#lookupcontinuation)

  Callers that do not handle split DWARF can call `skip_all_loads`

  to fast-forward to the end result. This result is produced with

  the data that is available and may be less accurate than the

  the results that would be produced if the caller did properly

  support split DWARF.

- <span id="lookupresult-map"></span>`fn map<T, F: FnOnce(<L as >::Output) -> T>(self, f: F) -> LookupResult<MappedLookup<T, L, F>>` — [`LookupResult`](lookup/index.md#lookupresult), [`MappedLookup`](lookup/index.md#mappedlookup)

- <span id="lookupresult-unwrap"></span>`fn unwrap(self) -> <L as >::Output` — [`LookupContinuation`](lookup/index.md#lookupcontinuation)

### `DebugFile`

```rust
enum DebugFile {
    Primary,
    Supplementary,
    Dwo,
}
```

#### Trait Implementations

##### `impl Clone for DebugFile`

- <span id="debugfile-clone"></span>`fn clone(&self) -> DebugFile` — [`DebugFile`](#debugfile)

##### `impl Copy for DebugFile`

##### `impl Debug for DebugFile`

- <span id="debugfile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DebugFile`

##### `impl PartialEq for DebugFile`

- <span id="debugfile-partialeq-eq"></span>`fn eq(&self, other: &DebugFile) -> bool` — [`DebugFile`](#debugfile)

##### `impl StructuralPartialEq for DebugFile`

## Traits

### `LookupContinuation`

```rust
trait LookupContinuation: Sized { ... }
```

This trait represents a partially complete operation that can be resumed
once a load of needed split DWARF data is completed or abandoned by the
API consumer.

#### Associated Types

- `type Output`

- `type Buf: 1`

#### Required Methods

- `fn resume(self, input: Option<Arc<gimli::Dwarf<<Self as >::Buf>>>) -> LookupResult<Self>`

  Resumes the operation with the provided data.

#### Implementors

- [`LoopingLookup`](lookup/index.md#loopinglookup)
- [`MappedLookup`](lookup/index.md#mappedlookup)
- [`SimpleLookup`](lookup/index.md#simplelookup)

## Functions

### `demangle`

```rust
fn demangle(name: &str, language: gimli::DwLang) -> Option<alloc::string::String>
```

Demangle a symbol name using the demangling scheme for the given language.

Returns `None` if demangling failed or is not required.

### `demangle_auto`

```rust
fn demangle_auto(name: alloc::borrow::Cow<'_, str>, language: Option<gimli::DwLang>) -> alloc::borrow::Cow<'_, str>
```

Apply 'best effort' demangling of a symbol name.

If `language` is given, then only the demangling scheme for that language
is used.

If `language` is `None`, then heuristics are used to determine how to
demangle the name. Currently, these heuristics are very basic.

If demangling fails or is not required, then `name` is returned unchanged.

## Type Aliases

### `Error`

```rust
type Error = gimli::Error;
```

### `LazyResult<T>`

```rust
type LazyResult<T> = core::cell::OnceCell<Result<T, gimli::Error>>;
```

