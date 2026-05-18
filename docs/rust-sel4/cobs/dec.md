**cobs > dec**

# Module: dec

## Contents

**Structs**

- [`CobsDecoder`](#cobsdecoder) - The [`CobsDecoder`] type is used to decode a stream of bytes to a
- [`DecodeReport`](#decodereport) - A report of the source and destination bytes used during in-place decoding

**Enums**

- [`DecodeError`](#decodeerror)
- [`DecodeResult`](#decoderesult) - [`DecodeResult`] represents the possible non-error outcomes of
- [`DecoderState`](#decoderstate) - The [`DecoderState`] is used to track the current state of a

**Functions**

- [`decode`](#decode) - Decodes the `source` buffer into the `dest` buffer.
- [`decode_in_place`](#decode_in_place) - Decodes a message in-place.
- [`decode_in_place_report`](#decode_in_place_report) - Decodes a message in-place.
- [`decode_in_place_with_sentinel`](#decode_in_place_with_sentinel) - Decodes a message in-place using an arbitrary sentinel value.
- [`decode_with_sentinel`](#decode_with_sentinel) - Decodes the `source` buffer into the `dest` buffer using an arbitrary sentinel value.

---

## cobs::dec::CobsDecoder

*Struct*

The [`CobsDecoder`] type is used to decode a stream of bytes to a
given mutable output slice. This is often useful when heap data
structures are not available, or when not all message bytes are
received at a single point in time.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(dest: &'a  mut [u8]) -> CobsDecoder<'a>` - Create a new streaming Cobs Decoder. Provide the output buffer
- `fn feed(self: & mut Self, data: u8) -> Result<Option<usize>, DecodeError>` - Push a single byte into the streaming CobsDecoder. Return values mean:
- `fn push(self: & mut Self, data: &[u8]) -> Result<Option<(usize, usize)>, DecodeError>` - Push a slice of bytes into the streaming CobsDecoder. Return values mean:

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cobs::dec::DecodeError

*Enum*

**Variants:**
- `EmptyFrame`
- `InvalidFrame{ decoded_bytes: usize }`
- `TargetBufTooSmall`

**Traits:** Error, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DecodeError) -> bool`



## cobs::dec::DecodeReport

*Struct*

A report of the source and destination bytes used during in-place decoding

**Fields:**
- `src_used: usize`
- `dst_used: usize`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DecodeReport`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cobs::dec::DecodeResult

*Enum*

[`DecodeResult`] represents the possible non-error outcomes of
pushing an encoded data byte into the [`DecoderState`] state machine

**Variants:**
- `NoData` - The given input byte did not prompt an output byte, either because the
- `DataComplete` - We have received a complete and well-encoded COBS message. The
- `DataContinue(u8)` - The following byte should be appended to the current end of the decoded



## cobs::dec::DecoderState

*Enum*

The [`DecoderState`] is used to track the current state of a
streaming decoder. This struct does not contain the output buffer
(or a reference to one), and can be used when streaming the decoded
output to a custom data type.

**Variants:**
- `Idle` - State machine has not received any non-zero bytes
- `Grab(u8)` - 1-254 bytes, can be header or 00
- `GrabChain(u8)` - 255 bytes, will be a header next

**Methods:**

- `fn feed(self: & mut Self, data: u8) -> Result<DecodeResult, DecodeError>` - Push a single encoded byte into the state machine. If the input was

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cobs::dec::decode

*Function*

Decodes the `source` buffer into the `dest` buffer.

This function uses the typical sentinel value of 0.

# Failures

This will return `Err(())` if there was a decoding error. Otherwise,
it will return `Ok(n)` where `n` is the length of the decoded message.

```rust
fn decode(source: &[u8], dest: & mut [u8]) -> Result<usize, DecodeError>
```



## cobs::dec::decode_in_place

*Function*

Decodes a message in-place.

This is the same function as [decode], but replaces the encoded message
with the decoded message instead of writing to another buffer.

The returned `usize` is the number of bytes used for the DECODED value,
NOT the number of source bytes consumed during decoding.

```rust
fn decode_in_place(buff: & mut [u8]) -> Result<usize, DecodeError>
```



## cobs::dec::decode_in_place_report

*Function*

Decodes a message in-place.

This is the same function as [decode_in_place], but provides a report
of both the number of source bytes consumed as well as the size of the
destination used.

```rust
fn decode_in_place_report(buf: & mut [u8]) -> Result<DecodeReport, DecodeError>
```



## cobs::dec::decode_in_place_with_sentinel

*Function*

Decodes a message in-place using an arbitrary sentinel value.

The returned `usize` is the number of bytes used for the DECODED value,
NOT the number of source bytes consumed during decoding.

```rust
fn decode_in_place_with_sentinel(buff: & mut [u8], sentinel: u8) -> Result<usize, DecodeError>
```



## cobs::dec::decode_with_sentinel

*Function*

Decodes the `source` buffer into the `dest` buffer using an arbitrary sentinel value.

This is done by XOR-ing each byte of the source message with the chosen sentinel value,
which transforms the message into the same message encoded with a sentinel value of 0.
Then the regular decoding transformation is performed.

The returned `usize` is the number of bytes used for the DECODED value,
NOT the number of source bytes consumed during decoding.

```rust
fn decode_with_sentinel(source: &[u8], dest: & mut [u8], sentinel: u8) -> Result<usize, DecodeError>
```



