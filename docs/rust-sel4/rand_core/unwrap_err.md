**rand_core > unwrap_err**

# Module: unwrap_err

## Contents

**Structs**

- [`UnwrapErr`](#unwraperr) - Wrapper around [`TryRng`] implementation which implements [`Rng`][crate::Rng]

---

## rand_core::unwrap_err::UnwrapErr

*Struct*

Wrapper around [`TryRng`] implementation which implements [`Rng`][crate::Rng]
by panicking on potential errors.

# Examples

```rust
# use rand_core::{UnwrapErr, TryRng, Rng};
fn with_try_rng<R: TryRng>(mut rng: R) {
    // rng does not impl Rng:
    let _ = rng.try_next_u32(); // okay
    // let _ = rng.next_u32(); // error

    // An adapter borrowing rng:
    let _ = UnwrapErr(&mut rng).next_u32();

    // An adapter moving rng:
    let mut rng = UnwrapErr(rng);
    let _ = rng.next_u32();
}

fn call_with_unsized_try_rng<R: TryRng + ?Sized>(rng: &mut R) {
    // R is unsized, thus we must use &mut R:
    let mut rng = UnwrapErr(rng);
    let _ = rng.next_u32();
}
```

**Generic Parameters:**
- R

**Tuple Struct**: `(R)`

**Methods:**

- `fn re<'b>(self: &'b  mut Self) -> UnwrapErr<&'b  mut R>` - Reborrow with a new lifetime

**Traits:** Eq, Copy, TryCryptoRng

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UnwrapErr<R>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> UnwrapErr<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnwrapErr<R>) -> bool`
- **TryRng**
  - `fn try_next_u32(self: & mut Self) -> Result<u32, <Self as >::Error>`
  - `fn try_next_u64(self: & mut Self) -> Result<u64, <Self as >::Error>`
  - `fn try_fill_bytes(self: & mut Self, dst: & mut [u8]) -> Result<(), <Self as >::Error>`



