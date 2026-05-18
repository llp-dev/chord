**bytecheck**

# Module: bytecheck

## Contents

**Structs**

- [`InvalidEnumDiscriminantError`](#invalidenumdiscriminanterror) - An error resulting from an invalid enum tag.
- [`NamedEnumVariantCheckContext`](#namedenumvariantcheckcontext) - Context for errors resulting from checking enum variants with named fields.
- [`StructCheckContext`](#structcheckcontext) - Context for errors resulting from invalid structs.
- [`TupleStructCheckContext`](#tuplestructcheckcontext) - Context for errors resulting from invalid tuple structs.
- [`UnnamedEnumVariantCheckContext`](#unnamedenumvariantcheckcontext) - Context for errors resulting from checking enum variants with unnamed

**Functions**

- [`check_bytes`](#check_bytes) - Checks whether the given pointer points to a valid value.
- [`check_bytes_with_context`](#check_bytes_with_context) - Checks whether the given pointer points to a valid value within the given

**Traits**

- [`CheckBytes`](#checkbytes) - A type that can check whether a pointer points to a valid value.
- [`Verify`](#verify) - A type that can check whether its invariants are upheld.

---

## bytecheck::CheckBytes

*Trait*

A type that can check whether a pointer points to a valid value.

`CheckBytes` can be derived with [`CheckBytes`](macro@CheckBytes) or
implemented manually for custom behavior.

# Safety

`check_bytes` must only return `Ok` if `value` points to a valid instance of
`Self`. Because `value` must always be properly aligned for `Self` and point
to enough bytes to represent the type, this implies that `value` may be
dereferenced safely.

# Example

```
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

See [`Verify`] for an example which uses less unsafe.

**Methods:**

- `check_bytes`: Checks whether the given pointer points to a valid value within the



## bytecheck::InvalidEnumDiscriminantError

*Struct*

An error resulting from an invalid enum tag.

This context is used by the derive macro to trace what the invalid
discriminant for an enum is.

**Generic Parameters:**
- T

**Fields:**
- `enum_name: &'static str` - The name of the enum with an invalid discriminant.
- `invalid_discriminant: T` - The invalid value of the enum discriminant.

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## bytecheck::NamedEnumVariantCheckContext

*Struct*

Context for errors resulting from checking enum variants with named fields.

This context is used by the derive macro to trace which field of an enum
variant failed validation.

**Fields:**
- `enum_name: &'static str` - The name of the enum with an invalid variant.
- `variant_name: &'static str` - The name of the variant that was invalid.
- `field_name: &'static str` - The name of the field that was invalid.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## bytecheck::StructCheckContext

*Struct*

Context for errors resulting from invalid structs.

This context is used by the derive macro to trace which field of a struct
failed validation.

**Fields:**
- `struct_name: &'static str` - The name of the struct with an invalid field.
- `field_name: &'static str` - The name of the struct field that was invalid.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## bytecheck::TupleStructCheckContext

*Struct*

Context for errors resulting from invalid tuple structs.

This context is used by the derive macro to trace which field of a tuple
struct failed validation.

**Fields:**
- `tuple_struct_name: &'static str` - The name of the tuple struct with an invalid field.
- `field_index: usize` - The index of the tuple struct field that was invalid.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## bytecheck::UnnamedEnumVariantCheckContext

*Struct*

Context for errors resulting from checking enum variants with unnamed
fields.

This context is used by the derive macro to trace which field of an enum
variant failed validation.

**Fields:**
- `enum_name: &'static str` - The name of the enum with an invalid variant.
- `variant_name: &'static str` - The name of the variant that was invalid.
- `field_index: usize` - The name of the field that was invalid.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## bytecheck::Verify

*Trait*

A type that can check whether its invariants are upheld.

When using [the derive](macro@CheckBytes), adding `#[bytecheck(verify)]`
allows implementing `Verify` for the derived type. [`Verify::verify`] will
be called after the type is checked and all fields are known to be valid.

# Safety

- `verify` must only return `Ok` if all of the invariants of this type are
  upheld by `self`.
- `verify` may not assume that its type invariants are upheld by the given
  `self` (the invariants of each field are guaranteed to be upheld).

# Example

```
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

**Methods:**

- `verify`: Checks whether the invariants of this type are upheld by `self`.



## bytecheck::check_bytes

*Function*

Checks whether the given pointer points to a valid value.

# Safety

The passed pointer must be aligned and point to enough initialized bytes to
represent the type.

# Example

```
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

```rust
fn check_bytes<T, E>(value: *const T) -> Result<(), E>
```



## bytecheck::check_bytes_with_context

*Function*

Checks whether the given pointer points to a valid value within the given
context.

# Safety

The passed pointer must be aligned and point to enough initialized bytes to
represent the type.

# Example

```
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

```rust
fn check_bytes_with_context<T, C, E>(value: *const T, context: & mut C) -> Result<(), E>
```



