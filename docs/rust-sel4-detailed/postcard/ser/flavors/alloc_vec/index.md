*[postcard](../../../index.md) / [ser](../../index.md) / [flavors](../index.md) / [alloc_vec](index.md)*

---

# Module `alloc_vec`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AllocVec`](#allocvec) | struct | The `AllocVec` flavor is a wrapper type around an [`alloc::vec::Vec`]. |

## Structs

### `AllocVec`

```rust
struct AllocVec {
    vec: alloc::vec::Vec<u8>,
}
```

The `AllocVec` flavor is a wrapper type around an [`alloc::vec::Vec`](../../../../addr2line/maybe_small/index.md).

This type is only available when the (non-default) `alloc` feature is active

#### Fields

- **`vec`**: `alloc::vec::Vec<u8>`

  The vec to be used for serialization

#### Implementations

- <span id="allocvec-new"></span>`fn new() -> Self`

  Create a new, currently empty, [`alloc::vec::Vec`](../../../../addr2line/maybe_small/index.md) to be used for storing serialized

  output data.

#### Trait Implementations

##### `impl Default for AllocVec`

- <span id="allocvec-default"></span>`fn default() -> AllocVec` — [`AllocVec`](../index.md#allocvec)

##### `impl Flavor for AllocVec`

- <span id="allocvec-flavor-type-output"></span>`type Output = Vec<u8>`

- <span id="allocvec-flavor-try-extend"></span>`fn try_extend(&mut self, data: &[u8]) -> Result<()>` — [`Result`](../../../error/index.md#result)

- <span id="allocvec-flavor-try-push"></span>`fn try_push(&mut self, data: u8) -> Result<()>` — [`Result`](../../../error/index.md#result)

- <span id="allocvec-flavor-finalize"></span>`fn finalize(self) -> Result<<Self as >::Output>` — [`Result`](../../../error/index.md#result), [`Flavor`](../index.md#flavor)

##### `impl Index for AllocVec`

- <span id="allocvec-index-type-output"></span>`type Output = u8`

- <span id="allocvec-index"></span>`fn index(&self, idx: usize) -> &u8`

##### `impl IndexMut for AllocVec`

- <span id="allocvec-indexmut-index-mut"></span>`fn index_mut(&mut self, idx: usize) -> &mut u8`

