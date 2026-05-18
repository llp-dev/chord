**postcard > ser**

# Module: ser

## Contents

**Modules**

- [`flavors`](#flavors) - # Serialization Flavors

**Functions**

- [`serialize_with_flavor`](#serialize_with_flavor) - `serialize_with_flavor()` has three generic parameters, `T, F, O`.
- [`serialized_size`](#serialized_size) - Compute the size of the postcard serialization of `T`.
- [`to_allocvec`](#to_allocvec) - Serialize a `T` to an `alloc::vec::Vec<u8>`.
- [`to_allocvec_cobs`](#to_allocvec_cobs) - Serialize and COBS encode a `T` to an `alloc::vec::Vec<u8>`.
- [`to_extend`](#to_extend) - Serialize a `T` to a [`core::iter::Extend`],
- [`to_slice`](#to_slice) - Serialize a `T` to the given slice, with the resulting slice containing
- [`to_slice_cobs`](#to_slice_cobs) - Serialize a `T` to the given slice, with the resulting slice containing

---

## Module: flavors

# Serialization Flavors

"Flavors" in `postcard` are used as modifiers to the serialization or deserialization
process. Flavors typically modify one or both of the following:

1. The output medium of the serialization, e.g. whether the data is serialized to a `[u8]` slice, or a `heapless::Vec`.
2. The format of the serialization, such as encoding the serialized output in a COBS format, performing CRC32 checksumming while serializing, etc.

Flavors are implemented using the [`Flavor`] trait, which acts as a "middleware" for receiving the bytes as serialized by `serde`.
Multiple flavors may be combined to obtain a desired combination of behavior and storage.
When flavors are combined, it is expected that the storage flavor (such as `Slice` or `HVec`) is the innermost flavor.

Custom flavors may be defined by users of the `postcard` crate, however some commonly useful flavors have been provided in
this module. If you think your custom flavor would be useful to others, PRs adding flavors are very welcome!

## Usability

Flavors may not always be convenient to use directly, as they may expose some implementation details of how the
inner workings of the flavor behaves. It is typical to provide a convenience method for using a flavor, to prevent
the user from having to specify generic parameters, setting correct initialization values, or handling the output of
the flavor correctly. See `postcard::to_vec()` for an example of this.

It is recommended to use the [`serialize_with_flavor()`](../fn.serialize_with_flavor.html) method for serialization. See it's documentation for information
regarding usage and generic type parameters.

## When to use (multiple) flavors

Combining flavors are nice for convenience, as they perform potentially multiple steps of
serialization at one time.

This can often be more memory efficient, as intermediate buffers are not typically required.

## When NOT to use (multiple) flavors

The downside of passing serialization through multiple steps is that it is typically slower than
performing each step serially. Said simply, "cobs encoding while serializing" is often slower
than "serialize then cobs encode", due to the ability to handle longer "runs" of data in each
stage. The downside is that if these stages can not be performed in-place on the buffer, you
will need additional buffers for each stage.

## Examples

### Using a single flavor

In the first example, we use the `Slice` flavor, to store the serialized output into a mutable `[u8]` slice.
No other modification is made to the serialization process.

```rust
use postcard::{
    serialize_with_flavor,
    ser_flavors::Slice,
};

let mut buf = [0u8; 32];

let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];
let buffer = &mut [0u8; 32];
let res = serialize_with_flavor::<[u8], Slice, &mut [u8]>(
    data,
    Slice::new(buffer)
).unwrap();

assert_eq!(res, &[0x04, 0x01, 0x00, 0x20, 0x30]);
```

### Using combined flavors

In the second example, we mix `Slice` with `Cobs`, to cobs encode the output while
the data is serialized. Notice how `Slice` (the storage flavor) is the innermost flavor used.

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



## postcard::ser::serialize_with_flavor

*Function*

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

```rust
fn serialize_with_flavor<T, S, O>(value: &T, storage: S) -> crate::error::Result<O>
```



## postcard::ser::serialized_size

*Function*

Compute the size of the postcard serialization of `T`.

```rust
fn serialized_size<T>(value: &T) -> crate::error::Result<usize>
```



## postcard::ser::to_allocvec

*Function*

Serialize a `T` to an `alloc::vec::Vec<u8>`.

## Example

```rust
use postcard::to_allocvec;

let ser: Vec<u8> = to_allocvec(&true).unwrap();
assert_eq!(ser.as_slice(), &[0x01]);

let ser: Vec<u8> = to_allocvec("Hi!").unwrap();
assert_eq!(ser.as_slice(), &[0x03, b'H', b'i', b'!']);
```

```rust
fn to_allocvec<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
```



## postcard::ser::to_allocvec_cobs

*Function*

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

```rust
fn to_allocvec_cobs<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
```



## postcard::ser::to_extend

*Function*

Serialize a `T` to a [`core::iter::Extend`],
## Example

```rust
use postcard::to_extend;
let mut vec = Vec::new();

let ser = to_extend(&true, vec).unwrap();
let vec = to_extend("Hi!", ser).unwrap();
assert_eq!(&vec[0..5], &[0x01, 0x03, b'H', b'i', b'!']);
```

```rust
fn to_extend<T, W>(value: &T, writer: W) -> crate::error::Result<W>
```



## postcard::ser::to_slice

*Function*

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

```rust
fn to_slice<'a, 'b, T>(value: &'b T, buf: &'a  mut [u8]) -> crate::error::Result<&'a  mut [u8]>
```



## postcard::ser::to_slice_cobs

*Function*

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

```rust
fn to_slice_cobs<'a, 'b, T>(value: &'b T, buf: &'a  mut [u8]) -> crate::error::Result<&'a  mut [u8]>
```



