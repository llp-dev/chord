# bytecheck

# bytecheck

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

[`CheckBytes`] is at the heart of bytecheck, and does the heavy lifting of
verifying that some bytes represent a valid type. Implementing it can be
done manually or automatically with the [derive macro](macro@CheckBytes).

## Layout stability

The layouts of types may change between compiler versions, or even different
compilations. To guarantee stable type layout between compilations, structs,
enums, and unions can be annotated with `#[repr(C)]`, and enums specifically
can be annotated with `#[repr(int)]` or `#[repr(C, int)]` as well. See
[the reference's page on type layout][reference] for more details.

[reference]: https://doc.rust-lang.org/reference/type-layout.html

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

## Modules

### [`bytecheck`](bytecheck.md)

*2 functions, 2 traits, 5 structs*

