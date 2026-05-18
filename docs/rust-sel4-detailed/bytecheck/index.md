# Crate `bytecheck`

bytecheck is a memory validation framework for Rust.

For some types, creating an invalid value immediately results in undefined
behavior. This can cause some issues when trying to validate potentially
invalid bytes, as just casting the bytes to your type can technically cause
errors. This makes it difficult to write validation routines, because until
you're certain that the bytes represent valid values you cannot cast them.

bytecheck provides a framework for performing these byte-level validations
and implements checks for basic types along with a derive macro to implement
validation for custom structs and enums.

## Design

[`CheckBytes`](#checkbytes) is at the heart of bytecheck, and does the heavy lifting of
verifying that some bytes represent a valid type. Implementing it can be
done manually or automatically with the [derive macro](macro@CheckBytes).

## Layout stability

The layouts of types may change between compiler versions, or even different
compilations. To guarantee stable type layout between compilations, structs,
enums, and unions can be annotated with `#[repr(C)]`, and enums specifically
can be annotated with `#[repr(int)]` or `#[repr(C, int)]` as well. See
[the reference's page on type layout][reference] for more details.

## Features

- `derive`: Re-exports the macros from `bytecheck_derive`. Enabled by
  default.
- `simdutf8`: Uses the `simdutf8` crate to validate UTF-8 strings. Enabled
  by default.

### Crates

Bytecheck provides integrations for some common crates by default. In the
future, crates should depend on bytecheck and provide their own integration.

- [`uuid-1`](https://docs.rs/uuid/1)

## Example
```rust
use bytecheck::{CheckBytes, check_bytes, rancor::Failure};

#[derive(CheckBytes, Debug)]
#[repr(C)]
struct Test {
    a: u32,
    b: char,
    c: bool,
}

#[repr(C, align(4))]
struct Aligned<const N: usize>([u8; N]);

macro_rules! bytes {
    ($($byte:literal,)*) => {
        (&Aligned([$($byte,)*]).0 as &[u8]).as_ptr()
    };
    ($($byte:literal),*) => {
        bytes!($($byte,)*)
    };
}

// In this example, the architecture is assumed to be little-endian
#[cfg(target_endian = "little")]
unsafe {
    // These are valid bytes for a `Test`
    check_bytes::<Test, Failure>(
        bytes![
            0u8, 0u8, 0u8, 0u8,
            0x78u8, 0u8, 0u8, 0u8,
            1u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap();

    // Changing the bytes for the u32 is OK, any bytes are a valid u32
    check_bytes::<Test, Failure>(
        bytes![
            42u8, 16u8, 20u8, 3u8,
            0x78u8, 0u8, 0u8, 0u8,
            1u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap();

    // Characters outside the valid ranges are invalid
    check_bytes::<Test, Failure>(
        bytes![
            0u8, 0u8, 0u8, 0u8,
            0x00u8, 0xd8u8, 0u8, 0u8,
            1u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap_err();
    check_bytes::<Test, Failure>(
        bytes![
            0u8, 0u8, 0u8, 0u8,
            0x00u8, 0x00u8, 0x11u8, 0u8,
            1u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap_err();

    // 0 is a valid boolean value (false) but 2 is not
    check_bytes::<Test, Failure>(
        bytes![
            0u8, 0u8, 0u8, 0u8,
            0x78u8, 0u8, 0u8, 0u8,
            0u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap();
    check_bytes::<Test, Failure>(
        bytes![
            0u8, 0u8, 0u8, 0u8,
            0x78u8, 0u8, 0u8, 0u8,
            2u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap_err();
}
```

## Contents

- [Modules](#modules)
  - [`CheckBytes`](#checkbytes)
- [Structs](#structs)
  - [`BoolCheckError`](#boolcheckerror)
  - [`TupleIndexContext`](#tupleindexcontext)
  - [`ArrayCheckContext`](#arraycheckcontext)
  - [`SliceCheckContext`](#slicecheckcontext)
  - [`StructCheckContext`](#structcheckcontext)
  - [`TupleStructCheckContext`](#tuplestructcheckcontext)
  - [`InvalidEnumDiscriminantError`](#invalidenumdiscriminanterror)
  - [`NamedEnumVariantCheckContext`](#namedenumvariantcheckcontext)
  - [`UnnamedEnumVariantCheckContext`](#unnamedenumvariantcheckcontext)
  - [`NonZeroCheckError`](#nonzerocheckerror)
- [Traits](#traits)
  - [`CheckBytes`](#checkbytes)
  - [`Verify`](#verify)
- [Functions](#functions)
  - [`rancor`](#rancor)
  - [`check_bytes`](#check-bytes)
  - [`check_bytes_with_context`](#check-bytes-with-context)
- [Macros](#macros)
  - [`impl_primitive!`](#impl-primitive)
  - [`impl_primitives!`](#impl-primitives)
  - [`impl_tuple!`](#impl-tuple)
  - [`impl_nonzero!`](#impl-nonzero)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CheckBytes`](#checkbytes) | mod |  |
| [`BoolCheckError`](#boolcheckerror) | struct |  |
| [`TupleIndexContext`](#tupleindexcontext) | struct |  |
| [`ArrayCheckContext`](#arraycheckcontext) | struct |  |
| [`SliceCheckContext`](#slicecheckcontext) | struct |  |
| [`StructCheckContext`](#structcheckcontext) | struct | Context for errors resulting from invalid structs. |
| [`TupleStructCheckContext`](#tuplestructcheckcontext) | struct | Context for errors resulting from invalid tuple structs. |
| [`InvalidEnumDiscriminantError`](#invalidenumdiscriminanterror) | struct | An error resulting from an invalid enum tag. |
| [`NamedEnumVariantCheckContext`](#namedenumvariantcheckcontext) | struct | Context for errors resulting from checking enum variants with named fields. |
| [`UnnamedEnumVariantCheckContext`](#unnamedenumvariantcheckcontext) | struct | Context for errors resulting from checking enum variants with unnamed fields. |
| [`NonZeroCheckError`](#nonzerocheckerror) | struct |  |
| [`CheckBytes`](#checkbytes) | trait | A type that can check whether a pointer points to a valid value. |
| [`Verify`](#verify) | trait | A type that can check whether its invariants are upheld. |
| [`rancor`](#rancor) | fn |  |
| [`check_bytes`](#check-bytes) | fn | Checks whether the given pointer points to a valid value. |
| [`check_bytes_with_context`](#check-bytes-with-context) | fn | Checks whether the given pointer points to a valid value within the given context. |
| [`impl_primitive!`](#impl-primitive) | macro |  |
| [`impl_primitives!`](#impl-primitives) | macro |  |
| [`impl_tuple!`](#impl-tuple) | macro |  |
| [`impl_nonzero!`](#impl-nonzero) | macro |  |

## Modules

- [`CheckBytes`](CheckBytes/index.md)

## Structs

### `BoolCheckError`

```rust
struct BoolCheckError {
    byte: u8,
}
```

#### Trait Implementations

##### `impl Debug for BoolCheckError`

- <span id="boolcheckerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BoolCheckError`

- <span id="boolcheckerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for BoolCheckError`

##### `impl Pointee for BoolCheckError`

- <span id="boolcheckerror-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for BoolCheckError`

- <span id="boolcheckerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `TupleIndexContext`

```rust
struct TupleIndexContext {
    index: usize,
}
```

#### Trait Implementations

##### `impl Debug for TupleIndexContext`

- <span id="tupleindexcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TupleIndexContext`

- <span id="tupleindexcontext-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointee for TupleIndexContext`

- <span id="tupleindexcontext-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for TupleIndexContext`

- <span id="tupleindexcontext-tostring-to-string"></span>`fn to_string(&self) -> String`

### `ArrayCheckContext`

```rust
struct ArrayCheckContext {
    index: usize,
}
```

#### Trait Implementations

##### `impl Debug for ArrayCheckContext`

- <span id="arraycheckcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ArrayCheckContext`

- <span id="arraycheckcontext-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointee for ArrayCheckContext`

- <span id="arraycheckcontext-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for ArrayCheckContext`

- <span id="arraycheckcontext-tostring-to-string"></span>`fn to_string(&self) -> String`

### `SliceCheckContext`

```rust
struct SliceCheckContext {
    index: usize,
}
```

#### Trait Implementations

##### `impl Debug for SliceCheckContext`

- <span id="slicecheckcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SliceCheckContext`

- <span id="slicecheckcontext-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointee for SliceCheckContext`

- <span id="slicecheckcontext-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for SliceCheckContext`

- <span id="slicecheckcontext-tostring-to-string"></span>`fn to_string(&self) -> String`

### `StructCheckContext`

```rust
struct StructCheckContext {
    pub struct_name: &'static str,
    pub field_name: &'static str,
}
```

Context for errors resulting from invalid structs.

This context is used by the derive macro to trace which field of a struct
failed validation.

#### Fields

- **`struct_name`**: `&'static str`

  The name of the struct with an invalid field.

- **`field_name`**: `&'static str`

  The name of the struct field that was invalid.

#### Trait Implementations

##### `impl Debug for StructCheckContext`

- <span id="structcheckcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StructCheckContext`

- <span id="structcheckcontext-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointee for StructCheckContext`

- <span id="structcheckcontext-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for StructCheckContext`

- <span id="structcheckcontext-tostring-to-string"></span>`fn to_string(&self) -> String`

### `TupleStructCheckContext`

```rust
struct TupleStructCheckContext {
    pub tuple_struct_name: &'static str,
    pub field_index: usize,
}
```

Context for errors resulting from invalid tuple structs.

This context is used by the derive macro to trace which field of a tuple
struct failed validation.

#### Fields

- **`tuple_struct_name`**: `&'static str`

  The name of the tuple struct with an invalid field.

- **`field_index`**: `usize`

  The index of the tuple struct field that was invalid.

#### Trait Implementations

##### `impl Debug for TupleStructCheckContext`

- <span id="tuplestructcheckcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TupleStructCheckContext`

- <span id="tuplestructcheckcontext-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointee for TupleStructCheckContext`

- <span id="tuplestructcheckcontext-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for TupleStructCheckContext`

- <span id="tuplestructcheckcontext-tostring-to-string"></span>`fn to_string(&self) -> String`

### `InvalidEnumDiscriminantError<T>`

```rust
struct InvalidEnumDiscriminantError<T> {
    pub enum_name: &'static str,
    pub invalid_discriminant: T,
}
```

An error resulting from an invalid enum tag.

This context is used by the derive macro to trace what the invalid
discriminant for an enum is.

#### Fields

- **`enum_name`**: `&'static str`

  The name of the enum with an invalid discriminant.

- **`invalid_discriminant`**: `T`

  The invalid value of the enum discriminant.

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for InvalidEnumDiscriminantError<T>`

- <span id="invalidenumdiscriminanterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Display> Display for InvalidEnumDiscriminantError<T>`

- <span id="invalidenumdiscriminanterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Error for InvalidEnumDiscriminantError<T>`

##### `impl<T> Pointee for InvalidEnumDiscriminantError<T>`

- <span id="invalidenumdiscriminanterror-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl<T> ToString for InvalidEnumDiscriminantError<T>`

- <span id="invalidenumdiscriminanterror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `NamedEnumVariantCheckContext`

```rust
struct NamedEnumVariantCheckContext {
    pub enum_name: &'static str,
    pub variant_name: &'static str,
    pub field_name: &'static str,
}
```

Context for errors resulting from checking enum variants with named fields.

This context is used by the derive macro to trace which field of an enum
variant failed validation.

#### Fields

- **`enum_name`**: `&'static str`

  The name of the enum with an invalid variant.

- **`variant_name`**: `&'static str`

  The name of the variant that was invalid.

- **`field_name`**: `&'static str`

  The name of the field that was invalid.

#### Trait Implementations

##### `impl Debug for NamedEnumVariantCheckContext`

- <span id="namedenumvariantcheckcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for NamedEnumVariantCheckContext`

- <span id="namedenumvariantcheckcontext-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointee for NamedEnumVariantCheckContext`

- <span id="namedenumvariantcheckcontext-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NamedEnumVariantCheckContext`

- <span id="namedenumvariantcheckcontext-tostring-to-string"></span>`fn to_string(&self) -> String`

### `UnnamedEnumVariantCheckContext`

```rust
struct UnnamedEnumVariantCheckContext {
    pub enum_name: &'static str,
    pub variant_name: &'static str,
    pub field_index: usize,
}
```

Context for errors resulting from checking enum variants with unnamed
fields.

This context is used by the derive macro to trace which field of an enum
variant failed validation.

#### Fields

- **`enum_name`**: `&'static str`

  The name of the enum with an invalid variant.

- **`variant_name`**: `&'static str`

  The name of the variant that was invalid.

- **`field_index`**: `usize`

  The name of the field that was invalid.

#### Trait Implementations

##### `impl Debug for UnnamedEnumVariantCheckContext`

- <span id="unnamedenumvariantcheckcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for UnnamedEnumVariantCheckContext`

- <span id="unnamedenumvariantcheckcontext-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointee for UnnamedEnumVariantCheckContext`

- <span id="unnamedenumvariantcheckcontext-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for UnnamedEnumVariantCheckContext`

- <span id="unnamedenumvariantcheckcontext-tostring-to-string"></span>`fn to_string(&self) -> String`

### `NonZeroCheckError`

```rust
struct NonZeroCheckError;
```

#### Trait Implementations

##### `impl Debug for NonZeroCheckError`

- <span id="nonzerocheckerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for NonZeroCheckError`

- <span id="nonzerocheckerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for NonZeroCheckError`

##### `impl Pointee for NonZeroCheckError`

- <span id="nonzerocheckerror-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroCheckError`

- <span id="nonzerocheckerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `CheckBytes<C: Fallible + ?Sized>`

```rust
trait CheckBytes<C: Fallible + ?Sized> { ... }
```

A type that can check whether a pointer points to a valid value.

`CheckBytes` can be derived with [`CheckBytes`](macro@CheckBytes) or
implemented manually for custom behavior.

# Safety

`check_bytes` must only return `Ok` if `value` points to a valid instance of
`Self`. Because `value` must always be properly aligned for `Self` and point
to enough bytes to represent the type, this implies that `value` may be
dereferenced safely.

# Example

```rust
use core::{error::Error, fmt};

use bytecheck::CheckBytes;
use rancor::{fail, Fallible, Source};

#[repr(C, align(4))]
pub struct NonMaxU32(u32);

unsafe impl<C: Fallible + ?Sized> CheckBytes<C> for NonMaxU32
where
    C::Error: Source,
{
    unsafe fn check_bytes(
        value: *const Self,
        context: &mut C,
    ) -> Result<(), C::Error> {
        #[derive(Debug)]
        struct NonMaxCheckError;

        impl fmt::Display for NonMaxCheckError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "non-max u32 was set to u32::MAX")
            }
        }

        impl Error for NonMaxCheckError {}

        let value = unsafe { value.read() };
        if value.0 == u32::MAX {
            fail!(NonMaxCheckError);
        }

        Ok(())
    }
}
```

See [`Verify`](#verify) for an example which uses less unsafe.

#### Required Methods

- `fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

  Checks whether the given pointer points to a valid value within the

#### Implementors

- `()`
- `(T0)`
- `(T0, T1)`
- `(T0, T1, T2)`
- `(T0, T1, T2, T3)`
- `(T0, T1, T2, T3, T4)`
- `(T0, T1, T2, T3, T4, T5)`
- `(T0, T1, T2, T3, T4, T5, T6)`
- `(T0, T1, T2, T3, T4, T5, T6, T7)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)`
- `[T; N]`
- `[T]`
- `bool`
- `char`
- `core::cell::Cell<T>`
- `core::cell::UnsafeCell<T>`
- `core::ffi::CStr`
- `core::marker::PhantomData<T>`
- `core::marker::PhantomPinned`
- `core::mem::ManuallyDrop<T>`
- `core::num::NonZeroI128`
- `core::num::NonZeroI16`
- `core::num::NonZeroI32`
- `core::num::NonZeroI64`
- `core::num::NonZeroI8`
- `core::num::NonZeroU128`
- `core::num::NonZeroU16`
- `core::num::NonZeroU32`
- `core::num::NonZeroU64`
- `core::num::NonZeroU8`
- `core::sync::atomic::AtomicBool`
- `core::sync::atomic::AtomicI16`
- `core::sync::atomic::AtomicI32`
- `core::sync::atomic::AtomicI64`
- `core::sync::atomic::AtomicI8`
- `core::sync::atomic::AtomicU16`
- `core::sync::atomic::AtomicU32`
- `core::sync::atomic::AtomicU64`
- `core::sync::atomic::AtomicU8`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `ops::Range<T>`
- `ops::RangeFrom<T>`
- `ops::RangeFull`
- `ops::RangeTo<T>`
- `ops::RangeToInclusive<T>`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`

### `Verify<C: Fallible + ?Sized>`

```rust
trait Verify<C: Fallible + ?Sized> { ... }
```

A type that can check whether its invariants are upheld.

When using [the derive](macro@CheckBytes), adding `#[bytecheck(verify)]`
allows implementing `Verify` for the derived type. `Verify::verify` will
be called after the type is checked and all fields are known to be valid.

# Safety

- `verify` must only return `Ok` if all of the invariants of this type are
  upheld by `self`.
- `verify` may not assume that its type invariants are upheld by the given
  `self` (the invariants of each field are guaranteed to be upheld).

# Example

```rust
use core::{error::Error, fmt};

use bytecheck::{CheckBytes, Verify};
use rancor::{fail, Fallible, Source};

#[derive(CheckBytes)]
#[bytecheck(verify)]
#[repr(C, align(4))]
pub struct NonMaxU32(u32);

unsafe impl<C: Fallible + ?Sized> Verify<C> for NonMaxU32
where
    C::Error: Source,
{
    fn verify(&self, context: &mut C) -> Result<(), C::Error> {
        #[derive(Debug)]
        struct NonMaxCheckError;

        impl fmt::Display for NonMaxCheckError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "non-max u32 was set to u32::MAX")
            }
        }

        impl Error for NonMaxCheckError {}

        if self.0 == u32::MAX {
            fail!(NonMaxCheckError);
        }

        Ok(())
    }
}
```

#### Required Methods

- `fn verify(&self, context: &mut C) -> Result<(), <C as >::Error>`

  Checks whether the invariants of this type are upheld by `self`.

## Functions

### `rancor`

```rust
fn rancor(bytes: &[u8]) -> Self
```

### `check_bytes`

```rust
unsafe fn check_bytes<T, E>(value: *const T) -> Result<(), E>
where
    T: CheckBytes<rancor::Strategy<(), E>> + ?Sized
```

Checks whether the given pointer points to a valid value.

# Safety

The passed pointer must be aligned and point to enough initialized bytes to
represent the type.

# Example

```rust
use bytecheck::check_bytes;
use rancor::Failure;

unsafe {
    // 0 and 1 are valid values for bools
    check_bytes::<bool, Failure>((&0u8 as *const u8).cast()).unwrap();
    check_bytes::<bool, Failure>((&1u8 as *const u8).cast()).unwrap();

    // 2 is not a valid value
    check_bytes::<bool, Failure>((&2u8 as *const u8).cast()).unwrap_err();
}
```

### `check_bytes_with_context`

```rust
unsafe fn check_bytes_with_context<T, C, E>(value: *const T, context: &mut C) -> Result<(), E>
where
    T: CheckBytes<rancor::Strategy<C, E>> + ?Sized
```

Checks whether the given pointer points to a valid value within the given
context.

# Safety

The passed pointer must be aligned and point to enough initialized bytes to
represent the type.

# Example

```rust
use core::{error::Error, fmt};

use bytecheck::{check_bytes_with_context, CheckBytes, Verify};
use rancor::{fail, Failure, Fallible, Source, Strategy};

trait Context {
    fn is_allowed(&self, value: u8) -> bool;
}

impl<T: Context + ?Sized, E> Context for Strategy<T, E> {
    fn is_allowed(&self, value: u8) -> bool {
        T::is_allowed(self, value)
    }
}

struct Allowed(u8);

impl Context for Allowed {
    fn is_allowed(&self, value: u8) -> bool {
        value == self.0
    }
}

#[derive(CheckBytes)]
#[bytecheck(verify)]
#[repr(C)]
pub struct ContextualByte(u8);

unsafe impl<C: Context + Fallible + ?Sized> Verify<C> for ContextualByte
where
    C::Error: Source,
{
    fn verify(&self, context: &mut C) -> Result<(), C::Error> {
        #[derive(Debug)]
        struct InvalidByte(u8);

        impl fmt::Display for InvalidByte {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "invalid contextual byte: {}", self.0)
            }
        }

        impl Error for InvalidByte {}

        if !context.is_allowed(self.0) {
            fail!(InvalidByte(self.0));
        }

        Ok(())
    }
}

let value = 45u8;
unsafe {
    // Checking passes when the context allows byte 45
    check_bytes_with_context::<ContextualByte, _, Failure>(
        (&value as *const u8).cast(),
        &mut Allowed(45),
    )
    .unwrap();

    // Checking fails when the conteext does not allow byte 45
    check_bytes_with_context::<ContextualByte, _, Failure>(
        (&value as *const u8).cast(),
        &mut Allowed(0),
    )
    .unwrap_err();
}
```

## Macros

### `impl_primitive!`

### `impl_primitives!`

### `impl_tuple!`

### `impl_nonzero!`

