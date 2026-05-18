*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [segment](index.md)*

---

# Module `segment`

TODO: Support the segment for XCOFF when auxiliary file header and loader section is ready.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`XcoffSegmentIterator`](#xcoffsegmentiterator) | struct | An iterator for the segments in an [`XcoffFile`]. |
| [`XcoffSegment`](#xcoffsegment) | struct | A loadable section in an [`XcoffFile`]. |
| [`XcoffSegmentIterator32`](#xcoffsegmentiterator32) | type | An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSegmentIterator64`](#xcoffsegmentiterator64) | type | An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSegment32`](#xcoffsegment32) | type | A segment in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSegment64`](#xcoffsegment64) | type | A segment in an [`XcoffFile64`](super::XcoffFile64). |

## Structs

### `XcoffSegmentIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegmentIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

An iterator for the segments in an [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-iterator-type-item"></span>`type Item = XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSegment<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegment<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

A loadable section in an [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff, R> ObjectSegment for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="xcoffsegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="xcoffsegment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="xcoffsegment-objectsegment-data-range"></span>`fn data_range(&self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsegment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md#segmentflags)

##### `impl<Xcoff, R> Sealed for XcoffSegment<'data, 'file, Xcoff, R>`

## Type Aliases

### `XcoffSegmentIterator32<'data, 'file, R>`

```rust
type XcoffSegmentIterator32<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegmentIterator64<'data, 'file, R>`

```rust
type XcoffSegmentIterator64<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegment32<'data, 'file, R>`

```rust
type XcoffSegment32<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader32, R>;
```

A segment in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegment64<'data, 'file, R>`

```rust
type XcoffSegment64<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader64, R>;
```

A segment in an [`XcoffFile64`](super::XcoffFile64).

