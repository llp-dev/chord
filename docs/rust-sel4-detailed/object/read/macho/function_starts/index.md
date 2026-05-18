*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [function_starts](index.md)*

---

# Module `function_starts`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FunctionStartsIterator`](#functionstartsiterator) | struct | Iterator over the function starts in a `LC_FUNCTION_STARTS` load command. |

## Structs

### `FunctionStartsIterator<'data>`

```rust
struct FunctionStartsIterator<'data> {
    data: crate::read::Bytes<'data>,
    addr: u64,
}
```

Iterator over the function starts in a `LC_FUNCTION_STARTS` load command.

#### Implementations

- <span id="functionstartsiterator-new"></span>`fn new(data: &'data [u8], addr: u64) -> Self`

#### Trait Implementations

##### `impl Clone for FunctionStartsIterator<'data>`

- <span id="functionstartsiterator-clone"></span>`fn clone(&self) -> FunctionStartsIterator<'data>` — [`FunctionStartsIterator`](../index.md#functionstartsiterator)

##### `impl Copy for FunctionStartsIterator<'data>`

##### `impl Debug for FunctionStartsIterator<'data>`

- <span id="functionstartsiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FunctionStartsIterator<'data>`

- <span id="functionstartsiterator-default"></span>`fn default() -> FunctionStartsIterator<'data>` — [`FunctionStartsIterator`](../index.md#functionstartsiterator)

##### `impl IntoIterator for FunctionStartsIterator<'data>`

- <span id="functionstartsiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="functionstartsiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="functionstartsiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FunctionStartsIterator<'data>`

- <span id="functionstartsiterator-iterator-type-item"></span>`type Item = Result<u64, Error>`

- <span id="functionstartsiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

