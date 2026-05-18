**pin_project**

# Module: pin_project

## Contents

**Traits**

- [`UnsafeUnpin`](#unsafeunpin) - A trait used for custom implementations of [`Unpin`].

---

## pin_project::UnsafeUnpin

*Trait*

A trait used for custom implementations of [`Unpin`].

This trait is used in conjunction with the `UnsafeUnpin` argument to
the [`#[pin_project]`][macro@pin_project] attribute.

# Safety

The Rust [`Unpin`] trait is safe to implement - by itself,
implementing it cannot lead to [undefined behavior][undefined-behavior].
Undefined behavior can only occur when other unsafe code is used.

It turns out that using pin projections, which requires unsafe code,
imposes additional requirements on an [`Unpin`] impl. Normally, all of this
unsafety is contained within this crate, ensuring that it's impossible for
you to violate any of the guarantees required by pin projection.

However, things change if you want to provide a custom [`Unpin`] impl
for your `#[pin_project]` type. As stated in [the Rust
documentation][pin-projection], you must be sure to only implement [`Unpin`]
when all of your `#[pin]` fields (i.e. structurally pinned fields) are also
[`Unpin`].

To help highlight this unsafety, the `UnsafeUnpin` trait is provided.
Implementing this trait is logically equivalent to implementing [`Unpin`] -
this crate will generate an [`Unpin`] impl for your type that 'forwards' to
your `UnsafeUnpin` impl. However, this trait is `unsafe` - since your type
uses structural pinning (otherwise, you wouldn't be using this crate!),
you must be sure that your `UnsafeUnpin` impls follows all of
the requirements for an [`Unpin`] impl of a structurally-pinned type.

Note that if you specify `#[pin_project(UnsafeUnpin)]`, but do *not*
provide an impl of `UnsafeUnpin`, your type will never implement [`Unpin`].
This is effectively the same thing as adding a [`PhantomPinned`] to your
type.

Since this trait is `unsafe`, impls of it will be detected by the
`unsafe_code` lint, and by tools like [`cargo geiger`][cargo-geiger].

# Examples

An `UnsafeUnpin` impl which, in addition to requiring that structurally
pinned fields be [`Unpin`], imposes an additional requirement:

```
use pin_project::{UnsafeUnpin, pin_project};

#[pin_project(UnsafeUnpin)]
struct Struct<K, V> {
    #[pin]
    field_1: K,
    field_2: V,
}

# #[allow(clippy::undocumented_unsafe_blocks)]
unsafe impl<K, V> UnsafeUnpin for Struct<K, V> where K: Unpin + Clone {}
```

[`PhantomPinned`]: core::marker::PhantomPinned
[cargo-geiger]: https://github.com/rust-secure-code/cargo-geiger
[pin-projection]: core::pin#projections-and-structural-pinning
[undefined-behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html



