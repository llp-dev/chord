# cobs

# `cobs`

This is an implementation of the Consistent Overhead Byte Stuffing (COBS) algorithm in Rust.

COBS is an algorithm for transforming a message into an encoding where a specific value (the
"sentinel" value) is not used. This value can then be used to mark frame boundaries in a serial
communication channel.

See the [wikipedia article](https://www.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing) for details.

## Features

`cobs` supports various runtime environments and is also suitable for `no_std` environments.

### Default features

- [`std`](https://doc.rust-lang.org/std/): Enables functionality relying on the standard library
  and also activates the `alloc` feature. Currently only adds [std::error::Error] support for the
  library error types.
- [`alloc`](https://doc.rust-lang.org/alloc/): Enables features which operate on containers
  like [alloc::vec::Vec](https://doc.rust-lang.org/beta/alloc/vec/struct.Vec.html).
  Enabled by the `std` feature.

### Optional features

- [`defmt`](https://docs.rs/defmt/latest/defmt/): Adds `defmt::Format` derives on some data
  structures and error types.
- [`serde`](https://serde.rs/): Adds `serde` derives on some data structures and error types.

## Modules

### [`cobs`](cobs.md)

*2 functions*

### [`dec`](dec.md)

*2 structs, 3 enums, 5 functions*

### [`enc`](enc.md)

*1 enum, 3 functions, 3 structs*

