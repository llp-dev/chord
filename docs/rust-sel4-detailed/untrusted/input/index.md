*[untrusted](../index.md) / [input](index.md)*

---

# Module `input`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Input`](#input) | struct | A wrapper around `&'a [u8]` that helps in writing panic-free code. |

## Structs

### `Input<'a>`

```rust
struct Input<'a> {
    value: no_panic::Slice<'a>,
}
```

A wrapper around `&'a [u8]` that helps in writing panic-free code.

No methods of `Input` will ever panic.

Intentionally avoids implementing `PartialEq` and `Eq` to avoid implicit
non-constant-time comparisons.

#### Implementations

- <span id="input-from"></span>`const fn from(bytes: &'a [u8]) -> Self`

  Construct a new `Input` for the given input `bytes`.

- <span id="input-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the input is empty and false otherwise.

- <span id="input-len"></span>`fn len(&self) -> usize`

  Returns the length of the `Input`.

- <span id="input-read-all"></span>`fn read_all<F, R, E>(&self, incomplete_read: E, read: F) -> Result<R, E>`

  Calls `read` with the given input as a `Reader`, ensuring that `read`

  consumed the entire input. If `read` does not consume the entire input,

  `incomplete_read` is returned.

- <span id="input-as-slice-less-safe"></span>`fn as_slice_less_safe(&self) -> &'a [u8]`

  Access the input as a slice so it can be processed by functions that

  are not written using the Input/Reader framework.

- <span id="input-into-value"></span>`fn into_value(self) -> no_panic::Slice<'a>` — [`Slice`](../no_panic/index.md#slice)

#### Trait Implementations

##### `impl Clone for Input<'a>`

- <span id="input-clone"></span>`fn clone(&self) -> Input<'a>` — [`Input`](#input)

##### `impl Copy for Input<'a>`

##### `impl Debug for Input<'_>`

- <span id="input-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

