*[sel4](../index.md) / [cnode_cap_data](index.md)*

---

# Module `cnode_cap_data`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CNodeCapData`](#cnodecapdata) | struct | Corresponds to `seL4_CNode_CapData`. |

## Structs

### `CNodeCapData`

```rust
struct CNodeCapData(sys::seL4_CNode_CapData);
```

Corresponds to `seL4_CNode_CapData`.

#### Implementations

- <span id="cnodecapdata-from-inner"></span>`const fn from_inner(inner: sys::seL4_CNode_CapData) -> Self`

- <span id="cnodecapdata-into-inner"></span>`const fn into_inner(self) -> sys::seL4_CNode_CapData`

- <span id="cnodecapdata-inner"></span>`const fn inner(&self) -> &sys::seL4_CNode_CapData`

- <span id="cnodecapdata-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_CNode_CapData`

- <span id="cnodecapdata-new"></span>`fn new(guard: Word, guard_size: usize) -> Self` — [`Word`](../index.md#word)

- <span id="cnodecapdata-skip"></span>`fn skip(num_bits: usize) -> Self`

- <span id="cnodecapdata-skip-high-bits"></span>`fn skip_high_bits(cnode_size_bits: usize) -> Self`

- <span id="cnodecapdata-into-word"></span>`fn into_word(self) -> Word` — [`Word`](../index.md#word)

#### Trait Implementations

##### `impl Clone for CNodeCapData`

- <span id="cnodecapdata-clone"></span>`fn clone(&self) -> CNodeCapData` — [`CNodeCapData`](#cnodecapdata)

##### `impl Debug for CNodeCapData`

- <span id="cnodecapdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CNodeCapData`

##### `impl PartialEq for CNodeCapData`

- <span id="cnodecapdata-partialeq-eq"></span>`fn eq(&self, other: &CNodeCapData) -> bool` — [`CNodeCapData`](#cnodecapdata)

##### `impl StructuralPartialEq for CNodeCapData`

