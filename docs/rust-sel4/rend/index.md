# rend

# rend

rend provides cross-platform, endian-aware primitives for Rust.

rend does not provide cross-platform alternatives for types that are
inherently cross-platform, such as `bool` and `u8`. It also does not provide
cross-platform alternatives for types that have an architecture-dependent
size, such as `isize` and `usize`. rend does not support custom types.

rend is intended to be used to build portable types that can be shared
between different architectures.

## Features

- `bytecheck`: Enables support for validating types using `bytecheck`.

## Crates

- `zerocopy-0_8`

## Example:
```rust
use core::mem::transmute;
use rend::*;

let little_int = i32_le::from_native(0x12345678);
// Internal representation is little-endian
assert_eq!(
    [0x78, 0x56, 0x34, 0x12],
    unsafe { transmute::<_, [u8; 4]>(little_int) }
);

// Can also be made with `.into()`
let little_int: i32_le = 0x12345678.into();
// Still formats correctly
assert_eq!("305419896", format!("{}", little_int));
assert_eq!("0x12345678", format!("0x{:x}", little_int));

let big_int = i32_be::from_native(0x12345678);
// Internal representation is big-endian
assert_eq!(
    [0x12, 0x34, 0x56, 0x78],
    unsafe { transmute::<_, [u8; 4]>(big_int) }
);

// Can also be made with `.into()`
let big_int: i32_be = 0x12345678.into();
// Still formats correctly
assert_eq!("305419896", format!("{}", big_int));
assert_eq!("0x12345678", format!("0x{:x}", big_int));
```

## Modules

### [`rend`](rend.md)

*1 module, 50 structs*

### [`unaligned`](unaligned.md)

*38 structs*

