*[postcard](../index.md) / [ser](index.md)*

---

# Module `ser`

## Contents

- [Modules](#modules)
  - [`flavors`](#flavors)
  - [`serializer`](#serializer)
- [Functions](#functions)
  - [`to_slice_cobs`](#to-slice-cobs)
  - [`to_slice`](#to-slice)
  - [`to_allocvec`](#to-allocvec)
  - [`to_allocvec_cobs`](#to-allocvec-cobs)
  - [`to_extend`](#to-extend)
  - [`serialize_with_flavor`](#serialize-with-flavor)
  - [`serialized_size`](#serialized-size)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`flavors`](#flavors) | mod | # Serialization Flavors |
| [`serializer`](#serializer) | mod |  |
| [`to_slice_cobs`](#to-slice-cobs) | fn | Serialize a `T` to the given slice, with the resulting slice containing data in a serialized then COBS encoded format. |
| [`to_slice`](#to-slice) | fn | Serialize a `T` to the given slice, with the resulting slice containing data in a serialized format. |
| [`to_allocvec`](#to-allocvec) | fn | Serialize a `T` to an `alloc::vec::Vec<u8>`. |
| [`to_allocvec_cobs`](#to-allocvec-cobs) | fn | Serialize and COBS encode a `T` to an `alloc::vec::Vec<u8>`. |
| [`to_extend`](#to-extend) | fn | Serialize a `T` to a [`core::iter::Extend`], ## Example |
| [`serialize_with_flavor`](#serialize-with-flavor) | fn | `serialize_with_flavor()` has three generic parameters, `T, F, O`. |
| [`serialized_size`](#serialized-size) | fn | Compute the size of the postcard serialization of `T`. |

## Modules

- [`flavors`](flavors/index.md) — # Serialization Flavors
- [`serializer`](serializer/index.md)

## Functions

### `to_slice_cobs`

```rust
fn to_slice_cobs<'a, 'b, T>(value: &'b T, buf: &'a mut [u8]) -> crate::error::Result<&'a mut [u8]>
where
    T: Serialize + ?Sized
```

Serialize a `T` to the given slice, with the resulting slice containing
data in a serialized then COBS encoded format. The terminating sentinel
`0x00` byte is included in the output buffer.

When successful, this function returns the slice containing the
serialized and encoded message.

## Example

```rust
use postcard::to_slice_cobs;
let mut buf = [0u8; 32];

let used = to_slice_cobs(&false, &mut buf).unwrap();
assert_eq!(used, &[0x01, 0x01, 0x00]);

let used = to_slice_cobs("1", &mut buf).unwrap();
assert_eq!(used, &[0x03, 0x01, b'1', 0x00]);

let used = to_slice_cobs("Hi!", &mut buf).unwrap();
assert_eq!(used, &[0x05, 0x03, b'H', b'i', b'!', 0x00]);

let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
let used = to_slice_cobs(data, &mut buf).unwrap();
assert_eq!(used, &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);
```

### `to_slice`

```rust
fn to_slice<'a, 'b, T>(value: &'b T, buf: &'a mut [u8]) -> crate::error::Result<&'a mut [u8]>
where
    T: Serialize + ?Sized
```

Serialize a `T` to the given slice, with the resulting slice containing
data in a serialized format.

When successful, this function returns the slice containing the
serialized message

## Example

```rust
use postcard::to_slice;
let mut buf = [0u8; 32];

let used = to_slice(&true, &mut buf).unwrap();
assert_eq!(used, &[0x01]);

let used = to_slice("Hi!", &mut buf).unwrap();
assert_eq!(used, &[0x03, b'H', b'i', b'!']);

// NOTE: postcard handles `&[u8]` and `&[u8; N]` differently.
let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
let used = to_slice(data, &mut buf).unwrap();
assert_eq!(used, &[0x04, 0x01, 0x00, 0x20, 0x30]);

let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];
let used = to_slice(data, &mut buf).unwrap();
assert_eq!(used, &[0x01, 0x00, 0x20, 0x30]);
```

### `to_allocvec`

```rust
fn to_allocvec<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
where
    T: Serialize + ?Sized
```

Serialize a `T` to an `alloc::vec::Vec<u8>`.

## Example

```rust
use postcard::to_allocvec;

let ser: Vec<u8> = to_allocvec(&true).unwrap();
assert_eq!(ser.as_slice(), &[0x01]);

let ser: Vec<u8> = to_allocvec("Hi!").unwrap();
assert_eq!(ser.as_slice(), &[0x03, b'H', b'i', b'!']);
```

### `to_allocvec_cobs`

```rust
fn to_allocvec_cobs<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
where
    T: Serialize + ?Sized
```

Serialize and COBS encode a `T` to an `alloc::vec::Vec<u8>`.

The terminating sentinel `0x00` byte is included in the output.

## Example

```rust
use postcard::to_allocvec_cobs;

let ser: Vec<u8> = to_allocvec_cobs(&true).unwrap();
assert_eq!(ser.as_slice(), &[0x02, 0x01, 0x00]);

let ser: Vec<u8> = to_allocvec_cobs("Hi!").unwrap();
assert_eq!(ser.as_slice(), &[0x05, 0x03, b'H', b'i', b'!', 0x00]);
```

### `to_extend`

```rust
fn to_extend<T, W>(value: &T, writer: W) -> crate::error::Result<W>
where
    T: Serialize + ?Sized,
    W: core::iter::Extend<u8>
```

Serialize a `T` to a `core::iter::Extend`,
## Example

```rust
use postcard::to_extend;
let mut vec = Vec::new();

let ser = to_extend(&true, vec).unwrap();
let vec = to_extend("Hi!", ser).unwrap();
assert_eq!(&vec[0..5], &[0x01, 0x03, b'H', b'i', b'!']);
```

### `serialize_with_flavor`

```rust
fn serialize_with_flavor<T, S, O>(value: &T, storage: S) -> crate::error::Result<O>
where
    T: Serialize + ?Sized,
    S: Flavor<Output = O>
```

`serialize_with_flavor()` has three generic parameters, `T, F, O`.

* `T`: This is the type that is being serialized
* `S`: This is the Storage that is used during serialization
* `O`: This is the resulting storage type that is returned containing the serialized data

For more information about how Flavors work, please see the
[`flavors` module documentation](./flavors/index.html).

```rust
use postcard::{
    serialize_with_flavor,
    ser_flavors::{Cobs, Slice},
};

let mut buf = [0u8; 32];

let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];
let buffer = &mut [0u8; 32];
let res = serialize_with_flavor::<[u8], Cobs<Slice>, &mut [u8]>(
    data,
    Cobs::try_new(Slice::new(buffer)).unwrap(),
).unwrap();

assert_eq!(res, &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);
```

### `serialized_size`

```rust
fn serialized_size<T>(value: &T) -> crate::error::Result<usize>
where
    T: Serialize + ?Sized
```

Compute the size of the postcard serialization of `T`.

