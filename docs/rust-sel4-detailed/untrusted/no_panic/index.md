*[untrusted](../index.md) / [no_panic](index.md)*

---

# Module `no_panic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Slice`](#slice) | struct | A wrapper around a slice that exposes no functions that can panic. |

## Structs

### `Slice<'a>`

```rust
struct Slice<'a> {
    bytes: &'a [u8],
}
```

A wrapper around a slice that exposes no functions that can panic.

Intentionally avoids implementing `Debug`, `Eq`, and `PartialEq` to avoid
creating a side channel that would leak information about the value.

#### Implementations

- <span id="slice-new"></span>`const fn new(bytes: &'a [u8]) -> Self`

- <span id="slice-get"></span>`fn get(&self, i: usize) -> Option<&u8>`

- <span id="slice-subslice"></span>`fn subslice(&self, r: core::ops::Range<usize>) -> Option<Self>`

- <span id="slice-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="slice-len"></span>`fn len(&self) -> usize`

- <span id="slice-as-slice-less-safe"></span>`fn as_slice_less_safe(&self) -> &'a [u8]`

#### Trait Implementations

##### `impl Clone for Slice<'a>`

- <span id="slice-clone"></span>`fn clone(&self) -> Slice<'a>` — [`Slice`](#slice)

##### `impl Copy for Slice<'a>`

