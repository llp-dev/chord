*[libc](../index.md) / [types](index.md)*

---

# Module `types`

Platform-agnostic support types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Padding`](#padding) | struct | A transparent wrapper over `MaybeUninit<T>` to represent uninitialized padding while providing `Default`. |
| [`CEnumRepr`](#cenumrepr) | type |  |

## Structs

### `Padding<T: Copy>`

```rust
struct Padding<T: Copy>(core::mem::MaybeUninit<T>);
```

A transparent wrapper over `MaybeUninit<T>` to represent uninitialized padding
while providing `Default`.

#### Implementations

- <span id="padding-uninit"></span>`const fn uninit() -> Self`

  Const constructor for uninitialized padding in const contexts.

#### Trait Implementations

##### `impl<T: clone::Clone + Copy> Clone for Padding<T>`

- <span id="padding-clone"></span>`fn clone(&self) -> Padding<T>` — [`Padding`](#padding)

##### `impl<T: marker::Copy + Copy> Copy for Padding<T>`

##### `impl<T: Copy> Debug for Padding<T>`

- <span id="padding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Copy> Default for Padding<T>`

- <span id="padding-default"></span>`fn default() -> Self`

## Type Aliases

### `CEnumRepr`

```rust
type CEnumRepr = c_uint;
```

