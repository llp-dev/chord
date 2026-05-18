*[postcard](../index.md) / [max_size](index.md)*

---

# Module `max_size`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MaxSize`](#maxsize) | trait | This trait is used to enforce the maximum size required to store the serialization of a given type. |
| [`max`](#max) | fn |  |

## Traits

### `MaxSize`

```rust
trait MaxSize { ... }
```

This trait is used to enforce the maximum size required to
store the serialization of a given type.

#### Associated Constants

- `const POSTCARD_MAX_SIZE: usize`

#### Implementors

- `&T`
- `&mut T`
- `()`
- `(A)`
- `(A, B)`
- `(A, B, C)`
- `(A, B, C, D)`
- `(A, B, C, D, E)`
- `(A, B, C, D, E, F)`
- `Option<T>`
- `Result<T, E>`
- `[T; N]`
- `alloc::boxed::Box<T>`
- `alloc::rc::Rc<T>`
- `alloc::sync::Arc<T>`
- `bool`
- `char`
- `core::marker::PhantomData<T>`
- `core::num::NonZeroI128`
- `core::num::NonZeroI16`
- `core::num::NonZeroI32`
- `core::num::NonZeroI64`
- `core::num::NonZeroI8`
- `core::num::NonZeroIsize`
- `core::num::NonZeroU128`
- `core::num::NonZeroU16`
- `core::num::NonZeroU32`
- `core::num::NonZeroU64`
- `core::num::NonZeroU8`
- `core::num::NonZeroUsize`
- `core::num::Wrapping<T>`
- `core::ops::Range<T>`
- `core::ops::RangeFrom<T>`
- `core::ops::RangeInclusive<T>`
- `core::ops::RangeTo<T>`
- `core::time::Duration`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Functions

### `max`

```rust
const fn max(lhs: usize, rhs: usize) -> usize
```

