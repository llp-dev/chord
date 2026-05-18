**rustls > msgs > codec**

# Module: msgs::codec

## Contents

**Structs**

- [`Reader`](#reader) - Wrapper over a slice of bytes that allows reading chunks from
- [`u24`](#u24)

**Traits**

- [`Codec`](#codec) - Trait for implementing encoding and decoding functionality

---

## rustls::msgs::codec::Codec

*Trait*

Trait for implementing encoding and decoding functionality
on something.

**Methods:**

- `encode`: Function for encoding itself by appending itself to
- `read`: Function for decoding itself from the provided reader
- `get_encoding`: Convenience function for encoding the implementation
- `read_bytes`: Function for wrapping a call to the read function in



## rustls::msgs::codec::Reader

*Struct*

Wrapper over a slice of bytes that allows reading chunks from
with the current position state held using a cursor.

A new reader for a sub section of the buffer can be created
using the `sub` function or a section of a certain length can
be obtained using the `take` function

**Generic Parameters:**
- 'a

**Methods:**

- `fn init(bytes: &'a [u8]) -> Self` - Creates a new Reader of the provided `bytes` slice with
- `fn sub(self: & mut Self, length: usize) -> Result<Self, InvalidMessage>` - Attempts to create a new Reader on a sub section of this
- `fn rest(self: & mut Self) -> &'a [u8]` - Borrows a slice of all the remaining bytes
- `fn take(self: & mut Self, length: usize) -> Option<&'a [u8]>` - Attempts to borrow a slice of bytes from the current
- `fn any_left(self: &Self) -> bool` - Used to check whether the reader has any content left
- `fn expect_empty(self: &Self, name: &'static str) -> Result<(), InvalidMessage>`
- `fn used(self: &Self) -> usize` - Returns the cursor position which is also the number
- `fn left(self: &Self) -> usize` - Returns the number of bytes that are still able to be



## rustls::msgs::codec::u24

*Struct*

**Tuple Struct**: `(u32)`



