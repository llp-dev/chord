# Crate `zeroize`

Securely zero memory with a simple trait ([`Zeroize`](#zeroize)) built on stable Rust
primitives which guarantee the operation will not be "optimized away".

## About

[Zeroing memory securely is hard] - compilers optimize for performance, and
in doing so they love to "optimize away" unnecessary zeroing calls. There are
many documented "tricks" to attempt to avoid these optimizations and ensure
that a zeroing routine is performed reliably.

This crate isn't about tricks: it uses `core::ptr::write_volatile`
and `core::sync::atomic` memory fences to provide easy-to-use, portable
zeroing behavior which works on all of Rust's core number types and slices
thereof, implemented in pure Rust with no usage of FFI or assembly.

- No insecure fallbacks!
- No dependencies!
- No FFI or inline assembly! **WASM friendly** (and tested)!
- `#![no_std]` i.e. **embedded-friendly**!
- No functionality besides securely zeroing memory!
- (Optional) Custom derive support for zeroing complex structures

## Minimum Supported Rust Version

Requires Rust **1.72** or newer.

In the future, we reserve the right to change MSRV (i.e. MSRV is out-of-scope
for this crate's SemVer guarantees), however when we do it will be accompanied
by a minor version bump.

## Usage

```rust
use zeroize::Zeroize;

// Protip: don't embed secrets in your source code.
// This is just an example.
let mut secret = b"Air shield password: 1,2,3,4,5".to_vec();
// [ ... ] open the air shield here

// Now that we're done using the secret, zero it out.
secret.zeroize();
```

The [`Zeroize`](#zeroize) trait is impl'd on all of Rust's core scalar types including
integers, floats, `bool`, and `char`.

Additionally, it's implemented on slices and `IterMut`s of the above types.

When the `alloc` feature is enabled (which it is by default), it's also
impl'd for `Vec<T>` for the above types as well as `String`, where it provides
`Vec::clear` / `String::clear`-like behavior (truncating to zero-length)
but ensures the backing memory is securely zeroed with some caveats.

With the `std` feature enabled (which it is **not** by default), [`Zeroize`](#zeroize)
is also implemented for `CString`. After calling `zeroize()` on a `CString`,
its internal buffer will contain exactly one nul byte. The backing
memory is zeroed by converting it to a `Vec<u8>` and back into a `CString`.
(NOTE: see "Stack/Heap Zeroing Notes" for important `Vec`/`String`/`CString` details)

The [`DefaultIsZeroes`](#defaultiszeroes) marker trait can be impl'd on types which also
impl [`Default`](../cpp_demangle/index.md), which implements [`Zeroize`](#zeroize) by overwriting a value with
the default value.

## Custom Derive Support

This crate has custom derive support for the `Zeroize` trait,
gated under the `zeroize` crate's `zeroize_derive` Cargo feature,
which automatically calls `zeroize()` on all members of a struct
or tuple struct.

Attributes supported for `Zeroize`:

On the item level:
- `#[zeroize(drop)]`: *deprecated* use `ZeroizeOnDrop` instead
- `#[zeroize(bound = "T: MyTrait")]`: this replaces any trait bounds
  inferred by zeroize

On the field level:
- `#[zeroize(skip)]`: skips this field or variant when calling `zeroize()`

Attributes supported for `ZeroizeOnDrop`:

On the field level:
- `#[zeroize(skip)]`: skips this field or variant when calling `zeroize()`

Example which derives `Drop`:

```rust
#[cfg(feature = "zeroize_derive")]
{
use zeroize::{Zeroize, ZeroizeOnDrop};

// This struct will be zeroized on drop
#[derive(Zeroize, ZeroizeOnDrop)]
struct MyStruct([u8; 32]);
}
```

Example which does not derive `Drop` (useful for e.g. `Copy` types)

```rust
#[cfg(feature = "zeroize_derive")]
{
use zeroize::Zeroize;

// This struct will *NOT* be zeroized on drop
#[derive(Copy, Clone, Zeroize)]
struct MyStruct([u8; 32]);
}
```

Example which only derives `Drop`:

```rust
#[cfg(feature = "zeroize_derive")]
{
use zeroize::ZeroizeOnDrop;

// This struct will be zeroized on drop
#[derive(ZeroizeOnDrop)]
struct MyStruct([u8; 32]);
}
```

## `Zeroizing<Z>`: wrapper for zeroizing arbitrary values on drop

`Zeroizing<Z: Zeroize>` is a generic wrapper type that impls `Deref`
and `DerefMut`, allowing access to an inner value of type `Z`, and also
impls a `Drop` handler which calls `zeroize()` on its contents:

```rust
use zeroize::Zeroizing;

fn use_secret() {
    let mut secret = Zeroizing::new([0u8; 5]);

    // Set the air shield password
    // Protip (again): don't embed secrets in your source code.
    secret.copy_from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(secret.as_ref(), &[1, 2, 3, 4, 5]);

    // The contents of `secret` will be automatically zeroized on drop
}

use_secret()
```

## What guarantees does this crate provide?

This crate guarantees the following:

1. The zeroing operation can't be "optimized away" by the compiler.
2. All subsequent reads to memory will see "zeroized" values.

LLVM's volatile semantics ensure #1 is true.

Additionally, thanks to work by the [Unsafe Code Guidelines Working Group],
we can now fairly confidently say #2 is true as well. Previously there were
worries that the approach used by this crate (mixing volatile and
non-volatile accesses) was undefined behavior due to language contained
in the documentation for `write_volatile`, however after some discussion
[these remarks have been removed] and the specific usage pattern in this
crate is considered to be well-defined.

Additionally this crate leverages `core::sync::atomic::compiler_fence`
with the strictest ordering
(`Ordering::SeqCst`) as a
precaution to help ensure reads are not reordered before memory has been
zeroed.

All of that said, there is still potential for microarchitectural attacks
(ala Spectre/Meltdown) to leak "zeroized" secrets through covert channels.
This crate makes no guarantees that zeroized values cannot be leaked
through such channels, as they represent flaws in the underlying hardware.

## Stack/Heap Zeroing Notes

This crate can be used to zero values from either the stack or the heap.

However, be aware several operations in Rust can unintentionally leave
copies of data in memory. This includes but is not limited to:

- Moves and [`Copy`](../gimli/index.md)
- Heap reallocation when using [`Vec`](../addr2line/maybe_small/index.md) and [`String`](../clap_builder/index.md)
- Borrowers of a reference making copies of the data

[`Pin`]`core::pin::Pin` can be leveraged in conjunction with this crate
to ensure data kept on the stack isn't moved.

The `Zeroize` impls for `Vec`, `String` and `CString` zeroize the entire
capacity of their backing buffer, but cannot guarantee copies of the data
were not previously made by buffer reallocation. It's therefore important
when attempting to zeroize such buffers to initialize them to the correct
capacity, and take care to prevent subsequent reallocation.

The `secrecy` crate provides higher-level abstractions for eliminating
usage patterns which can cause reallocations:

<https://crates.io/crates/secrecy>

## What about: clearing registers, mlock, mprotect, etc?

This crate is focused on providing simple, unobtrusive support for reliably
zeroing memory using the best approach possible on stable Rust.

Clearing registers is a difficult problem that can't easily be solved by
something like a crate, and requires either inline ASM or rustc support.
See <https://github.com/rust-lang/rust/issues/17046> for background on
this particular problem.

Other memory protection mechanisms are interesting and useful, but often
overkill (e.g. defending against RAM scraping or attackers with swap access).
In as much as there may be merit to these approaches, there are also many
other crates that already implement more sophisticated memory protections.
Such protections are explicitly out-of-scope for this crate.

Zeroing memory is [good cryptographic hygiene] and this crate seeks to promote
it in the most unobtrusive manner possible. This includes omitting complex
`unsafe` memory protection systems and just trying to make the best memory
zeroing crate available.






## Contents

- [Modules](#modules)
  - [`x86`](#x86)
- [Structs](#structs)
  - [`Zeroizing`](#zeroizing)
- [Traits](#traits)
  - [`Zeroize`](#zeroize)
  - [`ZeroizeOnDrop`](#zeroizeondrop)
  - [`DefaultIsZeroes`](#defaultiszeroes)
  - [`TryZeroize`](#tryzeroize)
- [Functions](#functions)
  - [`atomic_fence`](#atomic-fence)
  - [`volatile_write`](#volatile-write)
  - [`volatile_set`](#volatile-set)
  - [`zeroize_flat_type`](#zeroize-flat-type)
- [Macros](#macros)
  - [`impl_zeroize_with_default!`](#impl-zeroize-with-default)
  - [`impl_zeroize_for_non_zero!`](#impl-zeroize-for-non-zero)
  - [`impl_zeroize_tuple!`](#impl-zeroize-tuple)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`x86`](#x86) | mod | [`Zeroize`] impls for x86 SIMD registers |
| [`Zeroizing`](#zeroizing) | struct | `Zeroizing` is a a wrapper for any `Z: Zeroize` type which implements a `Drop` handler which zeroizes dropped values. |
| [`Zeroize`](#zeroize) | trait | Trait for securely erasing values from memory. |
| [`ZeroizeOnDrop`](#zeroizeondrop) | trait | Marker trait signifying that this type will [`Zeroize::zeroize`] itself on [`Drop`]. |
| [`DefaultIsZeroes`](#defaultiszeroes) | trait | Marker trait for types whose [`Default`] is the desired zeroization result |
| [`TryZeroize`](#tryzeroize) | trait | Fallible trait for representing cases where zeroization may or may not be possible. |
| [`atomic_fence`](#atomic-fence) | fn | Use fences to prevent accesses from being reordered before this point, which should hopefully help ensure that all accessors see zeroes after this point. |
| [`volatile_write`](#volatile-write) | fn | Perform a volatile write to the destination |
| [`volatile_set`](#volatile-set) | fn | Perform a volatile `memset` operation which fills a slice with a value |
| [`zeroize_flat_type`](#zeroize-flat-type) | fn | Zeroizes a flat type/struct. |
| [`impl_zeroize_with_default!`](#impl-zeroize-with-default) | macro |  |
| [`impl_zeroize_for_non_zero!`](#impl-zeroize-for-non-zero) | macro |  |
| [`impl_zeroize_tuple!`](#impl-zeroize-tuple) | macro |  |

## Modules

- [`x86`](x86/index.md) — [`Zeroize`] impls for x86 SIMD registers

## Structs

### `Zeroizing<Z: Zeroize>`

```rust
struct Zeroizing<Z: Zeroize>(Z);
```

`Zeroizing` is a a wrapper for any `Z: Zeroize` type which implements a
`Drop` handler which zeroizes dropped values.

#### Implementations

- <span id="zeroizing-new"></span>`fn new(value: Z) -> Self`

  Move value inside a `Zeroizing` wrapper which ensures it will be

  zeroized when it's dropped.

#### Trait Implementations

##### `impl<T, Z> AsMut for Zeroizing<Z>`

- <span id="zeroizing-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut T`

##### `impl<T, Z> AsRef for Zeroizing<Z>`

- <span id="zeroizing-asref-as-ref"></span>`fn as_ref(&self) -> &T`

##### `impl<Z: Zeroize + Clone> Clone for Zeroizing<Z>`

- <span id="zeroizing-clone"></span>`fn clone(&self) -> Self`

- <span id="zeroizing-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<Z: fmt::Debug + Zeroize> Debug for Zeroizing<Z>`

- <span id="zeroizing-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Z: default::Default + Zeroize> Default for Zeroizing<Z>`

- <span id="zeroizing-default"></span>`fn default() -> Zeroizing<Z>` — [`Zeroizing`](#zeroizing)

##### `impl<Z> Deref for Zeroizing<Z>`

- <span id="zeroizing-deref-type-target"></span>`type Target = Z`

- <span id="zeroizing-deref"></span>`fn deref(&self) -> &Z`

##### `impl<Z> DerefMut for Zeroizing<Z>`

- <span id="zeroizing-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut Z`

##### `impl<Z> Drop for Zeroizing<Z>`

- <span id="zeroizing-drop"></span>`fn drop(&mut self)`

##### `impl<Z: cmp::Eq + Zeroize> Eq for Zeroizing<Z>`

##### `impl<Z: cmp::PartialEq + Zeroize> PartialEq for Zeroizing<Z>`

- <span id="zeroizing-partialeq-eq"></span>`fn eq(&self, other: &Zeroizing<Z>) -> bool` — [`Zeroizing`](#zeroizing)

##### `impl Receiver for Zeroizing<Z>`

- <span id="zeroizing-receiver-type-target"></span>`type Target = T`

##### `impl<Z: Zeroize> StructuralPartialEq for Zeroizing<Z>`

##### `impl<Z> Zeroize for Zeroizing<Z>`

- <span id="zeroizing-zeroize"></span>`fn zeroize(&mut self)`

##### `impl<Z> ZeroizeOnDrop for Zeroizing<Z>`

## Traits

### `Zeroize`

```rust
trait Zeroize { ... }
```

Trait for securely erasing values from memory.

#### Required Methods

- `fn zeroize(&mut self)`

  Zero out this object from memory using Rust intrinsics which ensure the

#### Implementors

- [`Zeroizing`](#zeroizing)
- `(A)`
- `(A, B)`
- `(A, B, C)`
- `(A, B, C, D)`
- `(A, B, C, D, E)`
- `(A, B, C, D, E, F)`
- `(A, B, C, D, E, F, G)`
- `(A, B, C, D, E, F, G, H)`
- `(A, B, C, D, E, F, G, H, I)`
- `(A, B, C, D, E, F, G, H, I, J)`
- `Option<Z>`
- `Z`
- `[Z; N]`
- `[Z]`
- `[core::mem::MaybeUninit<Z>]`
- `__m128`
- `__m128d`
- `__m128i`
- `__m256`
- `__m256d`
- `__m256i`
- `alloc::boxed::Box<[Z]>`
- `alloc::boxed::Box<str>`
- `alloc::string::String`
- `alloc::vec::Vec<Z>`
- `core::marker::PhantomData<Z>`
- `core::mem::MaybeUninit<Z>`
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
- `core::slice::IterMut<'_, Z>`
- `num::Wrapping<Z>`
- `str`

### `ZeroizeOnDrop`

```rust
trait ZeroizeOnDrop { ... }
```

Marker trait signifying that this type will `Zeroize::zeroize` itself on [`Drop`](../gimli/index.md).

#### Implementors

- [`Zeroizing`](#zeroizing)
- `()`
- `(A)`
- `(A, B)`
- `(A, B, C)`
- `(A, B, C, D)`
- `(A, B, C, D, E)`
- `(A, B, C, D, E, F)`
- `(A, B, C, D, E, F, G)`
- `(A, B, C, D, E, F, G, H)`
- `(A, B, C, D, E, F, G, H, I)`
- `(A, B, C, D, E, F, G, H, I, J)`
- `Option<Z>`
- `[Z; N]`
- `alloc::boxed::Box<[Z]>`
- `alloc::vec::Vec<Z>`
- `core::marker::PhantomData<Z>`
- `core::marker::PhantomPinned`

### `DefaultIsZeroes`

```rust
trait DefaultIsZeroes: Copy + Default + Sized { ... }
```

Marker trait for types whose [`Default`](../cpp_demangle/index.md) is the desired zeroization result

#### Implementors

- `()`
- `bool`
- `char`
- `core::marker::PhantomPinned`
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

### `TryZeroize`

```rust
trait TryZeroize { ... }
```

Fallible trait for representing cases where zeroization may or may not be
possible.

This is primarily useful for scenarios like reference counted data, where
zeroization is only possible when the last reference is dropped.

#### Required Methods

- `fn try_zeroize(&mut self) -> bool`

  Try to zero out this object from memory using Rust intrinsics which

## Functions

### `atomic_fence`

```rust
fn atomic_fence()
```

Use fences to prevent accesses from being reordered before this
point, which should hopefully help ensure that all accessors
see zeroes after this point.

### `volatile_write`

```rust
fn volatile_write<T: Copy + Sized>(dst: &mut T, src: T)
```

Perform a volatile write to the destination

### `volatile_set`

```rust
unsafe fn volatile_set<T: Copy + Sized>(dst: *mut T, src: T, count: usize)
```

Perform a volatile `memset` operation which fills a slice with a value

Safety:
The memory pointed to by `dst` must be a single allocated object that is valid for `count`
contiguous elements of `T`.
`count` must not be larger than an `isize`.
`dst` being offset by `size_of::<T> * count` bytes must not wrap around the address space.
Also `dst` must be properly aligned.

### `zeroize_flat_type`

```rust
unsafe fn zeroize_flat_type<F: Sized>(data: *mut F)
```

Zeroizes a flat type/struct. Only zeroizes the values that it owns, and it does not work on
dynamically sized values or trait objects. It would be inefficient to use this function on a
type that already implements `ZeroizeOnDrop`.

# Safety
- The type must not contain references to outside data or dynamically sized data, such as
  `Vec<T>` or `String`.
- Values stored in the type must not have `Drop` impls.
- This function can invalidate the type if it is used after this function is called on it.
  It is advisable to call this function only in `impl Drop`.
- The bit pattern of all zeroes must be valid for the data being zeroized. This may not be
  true for enums and pointers.

# Incompatible data types
Some data types that cannot be safely zeroized using `zeroize_flat_type` include,
but are not limited to:
- References: `&T` and `&mut T`
- Non-nullable types: `NonNull<T>`, `NonZeroU32`, etc.
- Enums with explicit non-zero tags.
- Smart pointers and collections: `Arc<T>`, `Box<T>`, `Vec<T>`, `HashMap<K, V>`, `String`, etc.

# Examples
Safe usage for a struct containing strictly flat data:
```rust
use zeroize::{ZeroizeOnDrop, zeroize_flat_type};

struct DataToZeroize {
    flat_data_1: [u8; 32],
    flat_data_2: SomeMoreFlatData,
}

struct SomeMoreFlatData(u64);

impl Drop for DataToZeroize {
    fn drop(&mut self) {
        unsafe { zeroize_flat_type(self as *mut Self) }
    }
}
impl ZeroizeOnDrop for DataToZeroize {}

let mut data = DataToZeroize {
    flat_data_1: [3u8; 32],
    flat_data_2: SomeMoreFlatData(123u64)
};

// data gets zeroized when dropped
```

## Macros

### `impl_zeroize_with_default!`

### `impl_zeroize_for_non_zero!`

### `impl_zeroize_tuple!`

