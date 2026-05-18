**bytemuck > transparent**

# Module: transparent

## Contents

**Traits**

- [`TransparentWrapper`](#transparentwrapper) - A trait which indicates that a type is a `#[repr(transparent)]` wrapper

---

## bytemuck::transparent::TransparentWrapper

*Trait*

A trait which indicates that a type is a `#[repr(transparent)]` wrapper
around the `Inner` value.

This allows safely copy transmuting between the `Inner` type and the
`TransparentWrapper` type. Functions like `wrap_{}` convert from the inner
type to the wrapper type and `peel_{}` functions do the inverse conversion
from the wrapper type to the inner type. We deliberately do not call the
wrapper-removing methods "unwrap" because at this point that word is too
strongly tied to the Option/ Result methods.

# Safety

The safety contract of `TransparentWrapper` is relatively simple:

For a given `Wrapper` which implements `TransparentWrapper<Inner>`:

1. `Wrapper` must be a wrapper around `Inner` with an identical data
   representations. This    either means that it must be a
   `#[repr(transparent)]` struct which    contains a either a field of type
   `Inner` (or a field of some other    transparent wrapper for `Inner`) as
   the only non-ZST field.

2. Any fields *other* than the `Inner` field must be trivially constructable
   ZSTs, for example `PhantomData`, `PhantomPinned`, etc. (When deriving
   `TransparentWrapper` on a type with ZST fields, the ZST fields must be
   [`Zeroable`]).

3. The `Wrapper` may not impose additional alignment requirements over
   `Inner`.
    - Note: this is currently guaranteed by `repr(transparent)`, but there
      have been discussions of lifting it, so it's stated here explicitly.

4. All functions on `TransparentWrapper` **may not** be overridden.

## Caveats

If the wrapper imposes additional constraints upon the inner type which are
required for safety, it's responsible for ensuring those still hold -- this
generally requires preventing access to instances of the inner type, as
implementing `TransparentWrapper<U> for T` means anybody can call
`T::cast_ref(any_instance_of_u)`.

For example, it would be invalid to implement TransparentWrapper for `str`
to implement `TransparentWrapper` around `[u8]` because of this.

# Examples

## Basic

```
use bytemuck::TransparentWrapper;
# #[derive(Default)]
# struct SomeStruct(u32);

#[repr(transparent)]
struct MyWrapper(SomeStruct);

unsafe impl TransparentWrapper<SomeStruct> for MyWrapper {}

// interpret a reference to &SomeStruct as a &MyWrapper
let thing = SomeStruct::default();
let inner_ref: &MyWrapper = MyWrapper::wrap_ref(&thing);

// Works with &mut too.
let mut mut_thing = SomeStruct::default();
let inner_mut: &mut MyWrapper = MyWrapper::wrap_mut(&mut mut_thing);

# let _ = (inner_ref, inner_mut); // silence warnings
```

## Use with dynamically sized types

```
use bytemuck::TransparentWrapper;

#[repr(transparent)]
struct Slice<T>([T]);

unsafe impl<T> TransparentWrapper<[T]> for Slice<T> {}

let s = Slice::wrap_ref(&[1u32, 2, 3]);
assert_eq!(&s.0, &[1, 2, 3]);

let mut buf = [1, 2, 3u8];
let sm = Slice::wrap_mut(&mut buf);
```

## Deriving

When deriving, the non-wrapped fields must uphold all the normal
requirements, and must also be `Zeroable`.
```
use bytemuck::TransparentWrapper;
use std::marker::PhantomData;

#[derive(TransparentWrapper)]
#[repr(transparent)]
#[transparent(usize)]
struct Wrapper<T: ?Sized>(usize, PhantomData<T>); // PhantomData<T> implements Zeroable for all T
```

Here, an error will occur, because `MyZst` does not implement `Zeroable`.
```compile_fail
use bytemuck::TransparentWrapper;
struct MyZst;

#[derive(TransparentWrapper)]
#[repr(transparent)]
#[transparent(usize)]
struct Wrapper(usize, MyZst); // MyZst does not implement Zeroable
```

**Methods:**

- `wrap`: Convert the inner type into the wrapper type.
- `wrap_ref`: Convert a reference to the inner type into a reference to the wrapper
- `wrap_mut`: Convert a mutable reference to the inner type into a mutable reference to
- `wrap_slice`: Convert a slice to the inner type into a slice to the wrapper type.
- `wrap_slice_mut`: Convert a mutable slice to the inner type into a mutable slice to the
- `peel`: Convert the wrapper type into the inner type.
- `peel_ref`: Convert a reference to the wrapper type into a reference to the inner
- `peel_mut`: Convert a mutable reference to the wrapper type into a mutable reference
- `peel_slice`: Convert a slice to the wrapped type into a slice to the inner type.
- `peel_slice_mut`: Convert a mutable slice to the wrapped type into a mutable slice to the



