*[gimli](../../index.md) / [read](../index.md) / [lookup](index.md)*

---

# Module `lookup`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugLookup`](#debuglookup) | struct |  |
| [`LookupEntryIter`](#lookupentryiter) | struct |  |
| [`PubStuffHeader`](#pubstuffheader) | struct |  |
| [`PubStuffParser`](#pubstuffparser) | struct |  |
| [`LookupParser`](#lookupparser) | trait |  |
| [`PubStuffEntry`](#pubstuffentry) | trait |  |

## Structs

### `DebugLookup<R, Parser>`

```rust
struct DebugLookup<R, Parser>
where
    R: Reader,
    Parser: LookupParser<R> {
    input_buffer: R,
    phantom: core::marker::PhantomData<Parser>,
}
```

#### Implementations

- <span id="debuglookup-items"></span>`fn items(&self) -> LookupEntryIter<R, Parser>` — [`LookupEntryIter`](#lookupentryiter)

- <span id="debuglookup-reader"></span>`fn reader(&self) -> &R`

#### Trait Implementations

##### `impl<R, Parser> Clone for DebugLookup<R, Parser>`

- <span id="debuglookup-clone"></span>`fn clone(&self) -> DebugLookup<R, Parser>` — [`DebugLookup`](#debuglookup)

##### `impl<R, Parser> Debug for DebugLookup<R, Parser>`

- <span id="debuglookup-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LookupEntryIter<R, Parser>`

```rust
struct LookupEntryIter<R, Parser>
where
    R: Reader,
    Parser: LookupParser<R> {
    current_set: Option<(R, <Parser as >::Header)>,
    remaining_input: R,
}
```

#### Implementations

- <span id="lookupentryiter-next"></span>`fn next(&mut self) -> Result<Option<<Parser as >::Entry>>` — [`Result`](../../index.md#result), [`LookupParser`](#lookupparser)

  Advance the iterator and return the next entry.

  

  Returns the newly parsed entry as `Ok(Some(Parser::Entry))`. Returns

  `Ok(None)` when iteration is complete and all entries have already been

  parsed and yielded. If an error occurs while parsing the next entry,

  then this error is returned as `Err(e)`, and all subsequent calls return

  `Ok(None)`.

#### Trait Implementations

##### `impl<R, Parser> Clone for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-clone"></span>`fn clone(&self) -> LookupEntryIter<R, Parser>` — [`LookupEntryIter`](#lookupentryiter)

##### `impl<R, Parser> Debug for LookupEntryIter<R, Parser>`

- <span id="lookupentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PubStuffHeader<T>`

```rust
struct PubStuffHeader<T> {
    format: crate::common::Format,
    length: T,
    version: u16,
    unit_offset: crate::common::DebugInfoOffset<T>,
    unit_length: T,
}
```

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for PubStuffHeader<T>`

- <span id="pubstuffheader-clone"></span>`fn clone(&self) -> PubStuffHeader<T>` — [`PubStuffHeader`](#pubstuffheader)

##### `impl<T: fmt::Debug> Debug for PubStuffHeader<T>`

- <span id="pubstuffheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for PubStuffHeader<T>`

##### `impl<T: cmp::PartialEq> PartialEq for PubStuffHeader<T>`

- <span id="pubstuffheader-partialeq-eq"></span>`fn eq(&self, other: &PubStuffHeader<T>) -> bool` — [`PubStuffHeader`](#pubstuffheader)

##### `impl<T> StructuralPartialEq for PubStuffHeader<T>`

### `PubStuffParser<R, Entry>`

```rust
struct PubStuffParser<R, Entry>
where
    R: Reader,
    Entry: PubStuffEntry<R> {
    phantom: core::marker::PhantomData<(R, Entry)>,
}
```

#### Trait Implementations

##### `impl<R, Entry> Clone for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-clone"></span>`fn clone(&self) -> PubStuffParser<R, Entry>` — [`PubStuffParser`](#pubstuffparser)

##### `impl<R, Entry> Debug for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Entry> LookupParser for PubStuffParser<R, Entry>`

- <span id="pubstuffparser-lookupparser-type-header"></span>`type Header = PubStuffHeader<<R as Reader>::Offset>`

- <span id="pubstuffparser-lookupparser-type-entry"></span>`type Entry = Entry`

- <span id="pubstuffparser-lookupparser-parse-header"></span>`fn parse_header(input: &mut R) -> Result<(R, <Self as >::Header)>` — [`Result`](../../index.md#result), [`LookupParser`](#lookupparser)

  Parse an pubthings set header. Returns a tuple of the

  pubthings to be parsed for this set, and the newly created PubThingHeader struct.

- <span id="pubstuffparser-lookupparser-parse-entry"></span>`fn parse_entry(input: &mut R, header: &<Self as >::Header) -> Result<Option<<Self as >::Entry>>` — [`LookupParser`](#lookupparser), [`Result`](../../index.md#result)

  Parse a single pubthing. Return `None` for the null pubthing, `Some` for an actual pubthing.

## Traits

### `LookupParser<R: Reader>`

```rust
trait LookupParser<R: Reader> { ... }
```

#### Associated Types

- `type Header`

- `type Entry`

#### Required Methods

- `fn parse_header(input: &mut R) -> Result<(R, <Self as >::Header)>`

  Parse a header from `input`. Returns a tuple of `input` sliced to contain just the entries

- `fn parse_entry(input: &mut R, header: &<Self as >::Header) -> Result<Option<<Self as >::Entry>>`

  Parse a single entry from `input`. Returns either a parsed representation of the entry

#### Implementors

- [`PubStuffParser`](#pubstuffparser)

### `PubStuffEntry<R: Reader>`

```rust
trait PubStuffEntry<R: Reader> { ... }
```

#### Required Methods

- `fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self`

#### Implementors

- [`PubNamesEntry`](../index.md#pubnamesentry)
- [`PubTypesEntry`](../index.md#pubtypesentry)

