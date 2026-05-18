*[gimli](../../index.md) / [read](../index.md) / [addr](index.md)*

---

# Module `addr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugAddr`](#debugaddr) | struct | The raw contents of the `.debug_addr` section. |
| [`AddrHeaderIter`](#addrheaderiter) | struct | An iterator over the headers of a `.debug_addr` section. |
| [`AddrHeader`](#addrheader) | struct | A header for a set of entries in the `.debug_addr` section. |
| [`AddrEntryIter`](#addrentryiter) | struct | An iterator over the addresses from a `.debug_addr` section. |

## Structs

### `DebugAddr<R>`

```rust
struct DebugAddr<R> {
    section: R,
}
```

The raw contents of the `.debug_addr` section.

#### Implementations

- <span id="debugaddr-get-address"></span>`fn get_address(&self, address_size: u8, base: DebugAddrBase<<R as >::Offset>, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrBase`](../../index.md#debugaddrbase), [`Reader`](../index.md#reader), [`DebugAddrIndex`](../../index.md#debugaddrindex), [`Result`](../../index.md#result)

  Returns the address at the given `base` and `index`.

  

  A set of addresses in the `.debug_addr` section consists of a header

  followed by a series of addresses.

  

  The `base` must be the `DW_AT_addr_base` value from the compilation unit DIE.

  This is an offset that points to the first address following the header.

  

  The `index` is the value of a `DW_FORM_addrx` attribute.

  

  The `address_size` must be the size of the address for the compilation unit.

  This value must also match the header. However, note that we do not parse the

  header to validate this, since locating the header is unreliable, and the GNU

  extensions do not emit it.

- <span id="debugaddr-headers"></span>`fn headers(&self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](../index.md#addrheaderiter)

  Iterate the sets of entries in the `.debug_addr` section.

  

  Each set of entries belongs to a single unit.

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugAddr<R>`

- <span id="debugaddr-clone"></span>`fn clone(&self) -> DebugAddr<R>` — [`DebugAddr`](../index.md#debugaddr)

##### `impl<R: marker::Copy> Copy for DebugAddr<R>`

##### `impl<R: fmt::Debug> Debug for DebugAddr<R>`

- <span id="debugaddr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAddr<R>`

- <span id="debugaddr-default"></span>`fn default() -> DebugAddr<R>` — [`DebugAddr`](../index.md#debugaddr)

##### `impl<R> Section for DebugAddr<R>`

- <span id="debugaddr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugaddr-section-reader"></span>`fn reader(&self) -> &R`

### `AddrHeaderIter<R: Reader>`

```rust
struct AddrHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugAddrOffset<<R as >::Offset>,
}
```

An iterator over the headers of a `.debug_addr` section.

#### Implementations

- <span id="addrheaderiter-next"></span>`fn next(&mut self) -> Result<Option<AddrHeader<R>>>` — [`Result`](../../index.md#result), [`AddrHeader`](../index.md#addrheader)

  Advance the iterator to the next header.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for AddrHeaderIter<R>`

- <span id="addrheaderiter-clone"></span>`fn clone(&self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](../index.md#addrheaderiter)

##### `impl<R: fmt::Debug + Reader> Debug for AddrHeaderIter<R>`

- <span id="addrheaderiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for AddrHeaderIter<R>`

- <span id="addrheaderiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="addrheaderiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="addrheaderiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for AddrHeaderIter<R>`

- <span id="addrheaderiter-iterator-type-item"></span>`type Item = Result<AddrHeader<R>, Error>`

- <span id="addrheaderiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `AddrHeader<R, Offset>`

```rust
struct AddrHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugAddrOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    entries: R,
}
```

A header for a set of entries in the `.debug_addr` section.

These entries all belong to a single unit.

#### Implementations

- <span id="addrheader-parse"></span>`fn parse(input: &mut R, offset: DebugAddrOffset<Offset>) -> Result<Self>` — [`DebugAddrOffset`](../../index.md#debugaddroffset), [`Result`](../../index.md#result)

- <span id="addrheader-offset"></span>`fn offset(&self) -> DebugAddrOffset<Offset>` — [`DebugAddrOffset`](../../index.md#debugaddroffset)

  Return the offset of this header within the `.debug_addr` section.

- <span id="addrheader-length"></span>`fn length(&self) -> Offset`

  Return the length of this set of entries, including the header.

- <span id="addrheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../../index.md#encoding)

  Return the encoding parameters for this set of entries.

- <span id="addrheader-entries"></span>`fn entries(&self) -> AddrEntryIter<R>` — [`AddrEntryIter`](../index.md#addrentryiter)

  Return the address entries in this set.

#### Trait Implementations

##### `impl<R, Offset> Clone for AddrHeader<R, Offset>`

- <span id="addrheader-clone"></span>`fn clone(&self) -> AddrHeader<R, Offset>` — [`AddrHeader`](../index.md#addrheader)

##### `impl<R, Offset> Debug for AddrHeader<R, Offset>`

- <span id="addrheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for AddrHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for AddrHeader<R, Offset>`

- <span id="addrheader-partialeq-eq"></span>`fn eq(&self, other: &AddrHeader<R, Offset>) -> bool` — [`AddrHeader`](../index.md#addrheader)

##### `impl<R, Offset> StructuralPartialEq for AddrHeader<R, Offset>`

### `AddrEntryIter<R: Reader>`

```rust
struct AddrEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

An iterator over the addresses from a `.debug_addr` section.

#### Implementations

- <span id="addrentryiter-next"></span>`fn next(&mut self) -> Result<Option<u64>>` — [`Result`](../../index.md#result)

  Advance the iterator and return the next address.

  

  Returns the newly parsed address as `Ok(Some(addr))`. Returns `Ok(None)`

  when iteration is complete and all addresses have already been parsed and

  yielded. If an error occurs while parsing the next address, then this error

  is returned as `Err(e)`, and all subsequent calls return `Ok(None)`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for AddrEntryIter<R>`

- <span id="addrentryiter-clone"></span>`fn clone(&self) -> AddrEntryIter<R>` — [`AddrEntryIter`](../index.md#addrentryiter)

##### `impl<R: fmt::Debug + Reader> Debug for AddrEntryIter<R>`

- <span id="addrentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for AddrEntryIter<R>`

- <span id="addrentryiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="addrentryiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="addrentryiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for AddrEntryIter<R>`

- <span id="addrentryiter-iterator-type-item"></span>`type Item = Result<u64, Error>`

- <span id="addrentryiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

