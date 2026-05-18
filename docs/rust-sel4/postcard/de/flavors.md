**postcard > de > flavors**

# Module: de::flavors

## Contents

**Structs**

- [`Slice`](#slice) - A simple [`Flavor`] representing the deserialization from a borrowed slice

**Traits**

- [`Flavor`](#flavor) - The deserialization Flavor trait

---

## postcard::de::flavors::Flavor

*Trait*

The deserialization Flavor trait

This is used as the primary way to decode serialized data from some kind of buffer,
or modify that data in a middleware style pattern.

See the module level docs for an example of how flavors are used.

**Methods:**

- `Remainder`: The remaining data of this flavor after deserializing has completed.
- `Source`: The source of data retrieved for deserialization.
- `pop`: Obtain the next byte for deserialization
- `size_hint`: Returns the number of bytes remaining in the message, if known.
- `try_take_n`: Attempt to take the next `ct` bytes from the serialized message.
- `try_take_n_temp`: Attempt to take the next `ct` bytes from the serialized message.
- `finalize`: Complete the deserialization process.



## postcard::de::flavors::Slice

*Struct*

A simple [`Flavor`] representing the deserialization from a borrowed slice

**Generic Parameters:**
- 'de

**Methods:**

- `fn new(sli: &'de [u8]) -> Self` - Create a new [Slice] from the given buffer

**Trait Implementations:**

- **Flavor**
  - `fn pop(self: & mut Self) -> Result<u8>`
  - `fn size_hint(self: &Self) -> Option<usize>`
  - `fn try_take_n(self: & mut Self, ct: usize) -> Result<&'de [u8]>`
  - `fn finalize(self: Self) -> Result<&'de [u8]>` - Return the remaining (unused) bytes in the Deserializer



