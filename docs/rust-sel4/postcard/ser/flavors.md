**postcard > ser > flavors**

# Module: ser::flavors

## Contents

**Structs**

- [`Cobs`](#cobs) - The `Cobs` flavor implements [Consistent Overhead Byte Stuffing] on
- [`ExtendFlavor`](#extendflavor) - Wrapper over a [`core::iter::Extend<u8>`] that implements the flavor trait
- [`Size`](#size) - The `Size` flavor is a measurement flavor, which accumulates the number of bytes needed to
- [`Slice`](#slice) - The `Slice` flavor is a storage flavor, storing the serialized (or otherwise modified) bytes into a plain

**Traits**

- [`Flavor`](#flavor) - The serialization Flavor trait

---

## postcard::ser::flavors::Cobs

*Struct*

The `Cobs` flavor implements [Consistent Overhead Byte Stuffing] on
the serialized data. The output of this flavor includes the termination/sentinel
byte of `0x00`.

This protocol is useful when sending data over a serial interface without framing such as a UART

[Consistent Overhead Byte Stuffing]: https://en.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing

**Generic Parameters:**
- B

**Methods:**

- `fn try_new(bee: B) -> Result<Self>` - Create a new Cobs modifier Flavor. If there is insufficient space

**Trait Implementations:**

- **Flavor**
  - `fn try_push(self: & mut Self, data: u8) -> Result<()>`
  - `fn finalize(self: Self) -> Result<<Self as >::Output>`



## postcard::ser::flavors::ExtendFlavor

*Struct*

Wrapper over a [`core::iter::Extend<u8>`] that implements the flavor trait

**Generic Parameters:**
- T

**Methods:**

- `fn new(iter: T) -> Self` - Create a new [`Self`] flavor from a given [`core::iter::Extend<u8>`]

**Trait Implementations:**

- **Flavor**
  - `fn try_push(self: & mut Self, data: u8) -> Result<()>`
  - `fn try_extend(self: & mut Self, b: &[u8]) -> Result<()>`
  - `fn finalize(self: Self) -> Result<<Self as >::Output>`



## postcard::ser::flavors::Flavor

*Trait*

The serialization Flavor trait

This is used as the primary way to encode serialized data into some kind of buffer,
or modify that data in a middleware style pattern.

See the module level docs for an example of how flavors are used.

**Methods:**

- `Output`: The `Output` type is what this storage "resolves" to when the serialization is complete,
- `try_extend`: Override this method when you want to customize processing
- `try_push`: Push a single byte to be modified and/or stored.
- `finalize`: Finalize the serialization process.



## postcard::ser::flavors::Size

*Struct*

The `Size` flavor is a measurement flavor, which accumulates the number of bytes needed to
serialize the data.

```
use postcard::{serialize_with_flavor, ser_flavors};

let value = false;
let size = serialize_with_flavor(&value, ser_flavors::Size::default()).unwrap();

assert_eq!(size, 1);
```

**Trait Implementations:**

- **Flavor**
  - `fn try_push(self: & mut Self, _b: u8) -> Result<()>`
  - `fn try_extend(self: & mut Self, b: &[u8]) -> Result<()>`
  - `fn finalize(self: Self) -> Result<<Self as >::Output>`
- **Default**
  - `fn default() -> Size`



## postcard::ser::flavors::Slice

*Struct*

The `Slice` flavor is a storage flavor, storing the serialized (or otherwise modified) bytes into a plain
`[u8]` slice. The `Slice` flavor resolves into a sub-slice of the original slice buffer.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(buf: &'a  mut [u8]) -> Self` - Create a new `Slice` flavor from a given backing buffer

**Trait Implementations:**

- **Flavor**
  - `fn try_push(self: & mut Self, b: u8) -> Result<()>`
  - `fn try_extend(self: & mut Self, b: &[u8]) -> Result<()>`
  - `fn finalize(self: Self) -> Result<<Self as >::Output>`
- **IndexMut**
  - `fn index_mut(self: & mut Self, idx: usize) -> & mut u8`
- **Index**
  - `fn index(self: &Self, idx: usize) -> &u8`



