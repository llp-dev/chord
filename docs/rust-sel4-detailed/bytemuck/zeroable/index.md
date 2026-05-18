*[bytemuck](../index.md) / [zeroable](index.md)*

---

# Module `zeroable`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Zeroable`](#zeroable) | trait | Trait for types that can be safely created with [`zeroed`](core::mem::zeroed). |

## Traits

### `Zeroable`

```rust
trait Zeroable: Sized { ... }
```

Trait for types that can be safely created with
[`zeroed`](core::mem::zeroed).

An all-zeroes value may or may not be the same value as the
[Default](core::default::Default) value of the type.

## Safety

* Your type must be inhabited (eg: no
  [Infallible](core::convert::Infallible)).
* Your type must be allowed to be an "all zeroes" bit pattern (eg: no
  [`NonNull<T>`](core::ptr::NonNull)).

## Features

Some `impl`s are feature gated due to the MSRV policy:

* `MaybeUninit<T>` was not available in 1.34.0, but is available under the
  `zeroable_maybe_uninit` feature flag.
* `Atomic*` types require Rust 1.60.0 or later to work on certain platforms,
  but is available under the `zeroable_atomics` feature flag.
* `[T; N]` for arbitrary `N` requires the `min_const_generics` feature flag.

#### Provided Methods

- `fn zeroed() -> Self`

  Calls [`zeroed`](core::mem::zeroed).

#### Implementors

- `()`
- `(A)`
- `(A, B)`
- `(A, B, C)`
- `(A, B, C, D)`
- `(A, B, C, D, E)`
- `(A, B, C, D, E, F)`
- `(A, B, C, D, E, F, G)`
- `(A, B, C, D, E, F, G, H)`
- `*const T`
- `*const [T]`
- `*const str`
- `*mut T`
- `*mut [T]`
- `*mut str`
- `Option<T>`
- `PhantomData<T>`
- `PhantomPinned`
- `Wrapping<T>`
- `[T; 0]`
- `[T; 1024]`
- `[T; 10]`
- `[T; 11]`
- `[T; 128]`
- `[T; 12]`
- `[T; 13]`
- `[T; 14]`
- `[T; 15]`
- `[T; 16]`
- `[T; 17]`
- `[T; 18]`
- `[T; 19]`
- `[T; 1]`
- `[T; 2048]`
- `[T; 20]`
- `[T; 21]`
- `[T; 22]`
- `[T; 23]`
- `[T; 24]`
- `[T; 256]`
- `[T; 25]`
- `[T; 26]`
- `[T; 27]`
- `[T; 28]`
- `[T; 29]`
- `[T; 2]`
- `[T; 30]`
- `[T; 31]`
- `[T; 32]`
- `[T; 3]`
- `[T; 4096]`
- `[T; 48]`
- `[T; 4]`
- `[T; 512]`
- `[T; 5]`
- `[T; 64]`
- `[T; 6]`
- `[T; 7]`
- `[T; 8]`
- `[T; 96]`
- `[T; 9]`
- `bool`
- `char`
- `core::cell::Cell<T>`
- `core::cell::UnsafeCell<T>`
- `core::cmp::Reverse<T>`
- `core::mem::ManuallyDrop<T>`
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
- `x86_64::__m128`
- `x86_64::__m128d`
- `x86_64::__m128i`
- `x86_64::__m256`
- `x86_64::__m256d`
- `x86_64::__m256i`

