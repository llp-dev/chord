**cobs > enc**

# Module: enc

## Contents

**Structs**

- [`CobsEncoder`](#cobsencoder) - The [`CobsEncoder`] type is used to encode a stream of bytes to a
- [`DestBufTooSmallError`](#destbuftoosmallerror)
- [`EncoderState`](#encoderstate) - The [`EncoderState`] is used to track the current state of a

**Enums**

- [`PushResult`](#pushresult) - [`PushResult`] is used to represent the changes to an (encoded)

**Functions**

- [`encode`](#encode) - Encodes the `source` buffer into the `dest` buffer.
- [`encode_with_sentinel`](#encode_with_sentinel) - Encodes the `source` buffer into the `dest` buffer using an
- [`try_encode`](#try_encode) - Attempts to encode the `source` buffer into the `dest` buffer.

---

## cobs::enc::CobsEncoder

*Struct*

The [`CobsEncoder`] type is used to encode a stream of bytes to a
given mutable output slice. This is often useful when heap data
structures are not available, or when not all message bytes are
received at a single point in time.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(out_buf: &'a  mut [u8]) -> CobsEncoder<'a>` - Create a new streaming Cobs Encoder
- `fn push(self: & mut Self, data: &[u8]) -> Result<(), DestBufTooSmallError>` - Push a slice of data to be encoded
- `fn finalize(self: Self) -> usize` - Complete encoding of the output message. Does NOT terminate

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cobs::enc::DestBufTooSmallError

*Struct*

**Unit Struct**

**Traits:** Error, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DestBufTooSmallError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cobs::enc::EncoderState

*Struct*

The [`EncoderState`] is used to track the current state of a
streaming encoder. This struct does not contain the output buffer
(or a reference to one), and can be used when streaming the encoded
output to a custom data type

**IMPORTANT NOTE**: When implementing a custom streaming encoder,
the [`EncoderState`] state machine assumes that the output buffer
**ALREADY** contains a single placeholder byte, and no other bytes.
This placeholder byte will be later modified with the first distance
to the next header/zero byte.

**Methods:**

- `fn push(self: & mut Self, data: u8) -> PushResult` - Push a single unencoded byte into the encoder state machine
- `fn finalize(self: Self) -> (usize, u8)` - Finalize the encoding process for a single message.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> EncoderState`
- **Default**
  - `fn default() -> Self` - Create a default initial state representation for a COBS encoder



## cobs::enc::PushResult

*Enum*

[`PushResult`] is used to represent the changes to an (encoded)
output data buffer when an unencoded byte is pushed into [`EncoderState`].

**Variants:**
- `AddSingle(u8)` - The returned byte should be placed at the current end of the data buffer
- `ModifyFromStartAndSkip((usize, u8))` - The byte at the given index should be replaced with the given byte.
- `ModifyFromStartAndPushAndSkip((usize, u8, u8))` - The byte at the given index should be replaced with the given byte.



## cobs::enc::encode

*Function*

Encodes the `source` buffer into the `dest` buffer.

This function assumes the typical sentinel value of 0, but does not terminate the encoded
message with the sentinel value. This should be done by the caller to ensure proper framing.

# Returns

The number of bytes written to in the `dest` buffer.

# Panics

This function will panic if the `dest` buffer is not large enough for the
encoded message. You can calculate the size the `dest` buffer needs to be with
the [crate::max_encoding_length] function.

```rust
fn encode(source: &[u8], dest: & mut [u8]) -> usize
```



## cobs::enc::encode_with_sentinel

*Function*

Encodes the `source` buffer into the `dest` buffer using an
arbitrary sentinel value.

This is done by first encoding the message with the typical sentinel value
of 0, then XOR-ing each byte of the encoded message with the chosen sentinel
value. This will ensure that the sentinel value doesn't show up in the encoded
message. See the paper "Consistent Overhead Byte Stuffing" for details.

This function does not terminate the encoded message with the sentinel value. This should be
done by the caller to ensure proper framing.

# Returns

The number of bytes written to in the `dest` buffer.

```rust
fn encode_with_sentinel(source: &[u8], dest: & mut [u8], sentinel: u8) -> usize
```



## cobs::enc::try_encode

*Function*

Attempts to encode the `source` buffer into the `dest` buffer.

This function assumes the typical sentinel value of 0, but does not terminate the encoded
message with the sentinel value. This should be done by the caller to ensure proper framing.

# Returns

The number of bytes written to in the `dest` buffer.

If the destination buffer does not have enough room, an error will be returned.

```rust
fn try_encode(source: &[u8], dest: & mut [u8]) -> Result<usize, DestBufTooSmallError>
```



