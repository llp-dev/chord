*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [exports_trie](index.md)*

---

# Module `exports_trie`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ExportsTrieIterator`](#exportstrieiterator) | struct | Iterator over the exports trie. |
| [`ExportSymbol`](#exportsymbol) | struct | Exported symbol information. |
| [`Frame`](#frame) | struct |  |
| [`NodeIterator`](#nodeiterator) | struct |  |
| [`ExportData`](#exportdata) | enum | Terminal data for an exports trie node. |

## Structs

### `ExportsTrieIterator<'data>`

```rust
struct ExportsTrieIterator<'data> {
    node_iter: NodeIterator<'data>,
}
```

Iterator over the exports trie.

#### Implementations

- <span id="exportstrieiterator-new"></span>`fn new(data: &'data [u8]) -> Self`

- <span id="exportstrieiterator-next"></span>`fn next(&mut self) -> Result<Option<ExportSymbol<'data>>>` — [`Result`](../../../index.md#result), [`ExportSymbol`](../index.md#exportsymbol)

  Returns the next exported symbol, if any.

#### Trait Implementations

##### `impl Debug for ExportsTrieIterator<'data>`

- <span id="exportstrieiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ExportsTrieIterator<'data>`

- <span id="exportstrieiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="exportstrieiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="exportstrieiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ExportsTrieIterator<'data>`

- <span id="exportstrieiterator-iterator-type-item"></span>`type Item = Result<ExportSymbol<'data>, Error>`

- <span id="exportstrieiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ExportSymbol<'data>`

```rust
struct ExportSymbol<'data> {
    name: alloc::boxed::Box<[u8]>,
    flags: u8,
    data: ExportData<'data>,
}
```

Exported symbol information.

#### Implementations

- <span id="exportsymbol-name"></span>`fn name(&self) -> &[u8]`

  The name of the exported symbol.

- <span id="exportsymbol-flags"></span>`fn flags(&self) -> u8`

  The flags for the exported symbol.

- <span id="exportsymbol-data"></span>`fn data(&self) -> &ExportData<'data>` — [`ExportData`](../index.md#exportdata)

  The terminal data for the exported symbol.

#### Trait Implementations

##### `impl Debug for ExportSymbol<'data>`

- <span id="exportsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Frame<'data>`

```rust
struct Frame<'data> {
    data: crate::read::Bytes<'data>,
    children_remaining: u8,
    name_buf_len: usize,
}
```

#### Trait Implementations

##### `impl Debug for Frame<'data>`

- <span id="frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `NodeIterator<'data>`

```rust
struct NodeIterator<'data> {
    data: &'data [u8],
    offset: usize,
    stack: alloc::vec::Vec<Frame<'data>>,
    name_buf: alloc::vec::Vec<u8>,
}
```

#### Implementations

- <span id="nodeiterator-new"></span>`fn new(data: &'data [u8]) -> Self`

- <span id="nodeiterator-push-node"></span>`fn push_node(&mut self) -> Result<Option<ExportSymbol<'data>>>` — [`Result`](../../../index.md#result), [`ExportSymbol`](../index.md#exportsymbol)

- <span id="nodeiterator-next"></span>`fn next(&mut self) -> Result<Option<Option<ExportSymbol<'data>>>>` — [`Result`](../../../index.md#result), [`ExportSymbol`](../index.md#exportsymbol)

#### Trait Implementations

##### `impl Debug for NodeIterator<'data>`

- <span id="nodeiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NodeIterator<'data>`

- <span id="nodeiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nodeiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nodeiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NodeIterator<'data>`

- <span id="nodeiterator-iterator-type-item"></span>`type Item = Result<Option<ExportSymbol<'data>>, Error>`

- <span id="nodeiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `ExportData<'data>`

```rust
enum ExportData<'data> {
    Regular {
        address: u64,
    },
    Reexport {
        dylib_ordinal: u64,
        import_name: &'data [u8],
    },
    StubAndResolver {
        stub_address: u64,
        resolver_address: u64,
    },
}
```

Terminal data for an exports trie node.

#### Variants

- **`Regular`**

  A regular export.

- **`Reexport`**

  A re-exported symbol.

- **`StubAndResolver`**

  A stub-and-resolver symbol.

#### Implementations

- <span id="exportdata-parse"></span>`fn parse(data: Bytes<'data>) -> Result<(u8, Self)>` — [`Bytes`](../../index.md#bytes), [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Debug for ExportData<'data>`

- <span id="exportdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

