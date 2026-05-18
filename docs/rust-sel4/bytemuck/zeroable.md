**bytemuck > zeroable**

# Module: zeroable

## Contents

**Traits**

- [`Zeroable`](#zeroable) - Trait for types that can be safely created with

---

## bytemuck::zeroable::Zeroable

*Trait*

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

**Methods:**

- `zeroed`: Calls [`zeroed`](core::mem::zeroed).



