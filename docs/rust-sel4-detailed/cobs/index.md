# Crate `cobs`

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

## Contents

- [Modules](#modules)
  - [`dec`](#dec)
  - [`enc`](#enc)
- [Structs](#structs)
  - [`CobsDecoder`](#cobsdecoder)
  - [`DecodeReport`](#decodereport)
  - [`CobsEncoder`](#cobsencoder)
  - [`DestBufTooSmallError`](#destbuftoosmallerror)
  - [`EncoderState`](#encoderstate)
- [Enums](#enums)
  - [`DecoderState`](#decoderstate)
  - [`DecodeResult`](#decoderesult)
  - [`DecodeError`](#decodeerror)
  - [`PushResult`](#pushresult)
- [Functions](#functions)
  - [`max_encoding_overhead`](#max-encoding-overhead)
  - [`max_encoding_length`](#max-encoding-length)
  - [`add`](#add)
  - [`decode`](#decode)
  - [`decode_in_place_report`](#decode-in-place-report)
  - [`decode_in_place`](#decode-in-place)
  - [`decode_with_sentinel`](#decode-with-sentinel)
  - [`decode_in_place_with_sentinel`](#decode-in-place-with-sentinel)
  - [`encode`](#encode)
  - [`try_encode`](#try-encode)
  - [`encode_with_sentinel`](#encode-with-sentinel)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`dec`](#dec) | mod |  |
| [`enc`](#enc) | mod |  |
| [`CobsDecoder`](#cobsdecoder) | struct | The [`CobsDecoder`] type is used to decode a stream of bytes to a given mutable output slice. |
| [`DecodeReport`](#decodereport) | struct | A report of the source and destination bytes used during in-place decoding |
| [`CobsEncoder`](#cobsencoder) | struct | The [`CobsEncoder`] type is used to encode a stream of bytes to a given mutable output slice. |
| [`DestBufTooSmallError`](#destbuftoosmallerror) | struct |  |
| [`EncoderState`](#encoderstate) | struct | The [`EncoderState`] is used to track the current state of a streaming encoder. |
| [`DecoderState`](#decoderstate) | enum | The [`DecoderState`] is used to track the current state of a streaming decoder. |
| [`DecodeResult`](#decoderesult) | enum | [`DecodeResult`] represents the possible non-error outcomes of pushing an encoded data byte into the [`DecoderState`] state machine |
| [`DecodeError`](#decodeerror) | enum |  |
| [`PushResult`](#pushresult) | enum | [`PushResult`] is used to represent the changes to an (encoded) output data buffer when an unencoded byte is pushed into [`EncoderState`]. |
| [`max_encoding_overhead`](#max-encoding-overhead) | fn | Calculates the maximum overhead when encoding a message with the given length. |
| [`max_encoding_length`](#max-encoding-length) | fn | Calculates the maximum possible size of an encoded message given the length of the source message. |
| [`add`](#add) | fn |  |
| [`decode`](#decode) | fn | Decodes the `source` buffer into the `dest` buffer. |
| [`decode_in_place_report`](#decode-in-place-report) | fn | Decodes a message in-place. |
| [`decode_in_place`](#decode-in-place) | fn | Decodes a message in-place. |
| [`decode_with_sentinel`](#decode-with-sentinel) | fn | Decodes the `source` buffer into the `dest` buffer using an arbitrary sentinel value. |
| [`decode_in_place_with_sentinel`](#decode-in-place-with-sentinel) | fn | Decodes a message in-place using an arbitrary sentinel value. |
| [`encode`](#encode) | fn | Encodes the `source` buffer into the `dest` buffer. |
| [`try_encode`](#try-encode) | fn | Attempts to encode the `source` buffer into the `dest` buffer. |
| [`encode_with_sentinel`](#encode-with-sentinel) | fn | Encodes the `source` buffer into the `dest` buffer using an arbitrary sentinel value. |

## Modules

- [`dec`](dec/index.md)
- [`enc`](enc/index.md)

## Structs

### `CobsDecoder<'a>`

```rust
struct CobsDecoder<'a> {
    dest: &'a mut [u8],
    dest_idx: usize,
    state: DecoderState,
}
```

The [`CobsDecoder`](#cobsdecoder) type is used to decode a stream of bytes to a
given mutable output slice. This is often useful when heap data
structures are not available, or when not all message bytes are
received at a single point in time.

#### Fields

- **`dest`**: `&'a mut [u8]`

  Destination slice for decoded message

- **`dest_idx`**: `usize`

  Index of next byte to write in `dest`

- **`state`**: `DecoderState`

  Decoder state as an enum

#### Implementations

- <span id="cobsdecoder-new"></span>`fn new(dest: &'a mut [u8]) -> CobsDecoder<'a>` — [`CobsDecoder`](#cobsdecoder)

  Create a new streaming Cobs Decoder. Provide the output buffer

  for the decoded message to be placed in

- <span id="cobsdecoder-feed"></span>`fn feed(&mut self, data: u8) -> Result<Option<usize>, DecodeError>` — [`DecodeError`](#decodeerror)

  Push a single byte into the streaming CobsDecoder. Return values mean:

  

  * Ok(None) - State machine okay, more data needed

  * Ok(Some(N)) - A message of N bytes was successfully decoded

  * Err([DecodeError]) - Message decoding failed

  

  NOTE: Sentinel value must be included in the input to this function for the

  decoding to complete

- <span id="cobsdecoder-push"></span>`fn push(&mut self, data: &[u8]) -> Result<Option<(usize, usize)>, DecodeError>` — [`DecodeError`](#decodeerror)

  Push a slice of bytes into the streaming CobsDecoder. Return values mean:

  

  * Ok(None) - State machine okay, more data needed

  * Ok(Some((N, M))) - A message of N bytes was successfully decoded,

      using M bytes from `data` (and earlier data)

  * Err([DecodeError]) - Message decoding failed

  

  NOTE: Sentinel value must be included in the input to this function for the

  decoding to complete

#### Trait Implementations

##### `impl Debug for CobsDecoder<'a>`

- <span id="cobsdecoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DecodeReport`

```rust
struct DecodeReport {
    pub src_used: usize,
    pub dst_used: usize,
}
```

A report of the source and destination bytes used during in-place decoding

#### Trait Implementations

##### `impl Clone for DecodeReport`

- <span id="decodereport-clone"></span>`fn clone(&self) -> DecodeReport` — [`DecodeReport`](#decodereport)

##### `impl Copy for DecodeReport`

##### `impl Debug for DecodeReport`

- <span id="decodereport-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `CobsEncoder<'a>`

```rust
struct CobsEncoder<'a> {
    dest: &'a mut [u8],
    dest_idx: usize,
    state: EncoderState,
    might_be_done: bool,
}
```

The [`CobsEncoder`](#cobsencoder) type is used to encode a stream of bytes to a
given mutable output slice. This is often useful when heap data
structures are not available, or when not all message bytes are
received at a single point in time.

#### Implementations

- <span id="cobsencoder-new"></span>`fn new(out_buf: &'a mut [u8]) -> CobsEncoder<'a>` — [`CobsEncoder`](#cobsencoder)

  Create a new streaming Cobs Encoder

- <span id="cobsencoder-push"></span>`fn push(&mut self, data: &[u8]) -> Result<(), DestBufTooSmallError>` — [`DestBufTooSmallError`](#destbuftoosmallerror)

  Push a slice of data to be encoded

- <span id="cobsencoder-finalize"></span>`fn finalize(self) -> usize`

  Complete encoding of the output message. Does NOT terminate

  the message with the sentinel value

#### Trait Implementations

##### `impl Debug for CobsEncoder<'a>`

- <span id="cobsencoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DestBufTooSmallError`

```rust
struct DestBufTooSmallError;
```

#### Trait Implementations

##### `impl Debug for DestBufTooSmallError`

- <span id="destbuftoosmallerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DestBufTooSmallError`

- <span id="destbuftoosmallerror-display-fmt"></span>`fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for DestBufTooSmallError`

##### `impl Error for DestBufTooSmallError`

##### `impl PartialEq for DestBufTooSmallError`

- <span id="destbuftoosmallerror-partialeq-eq"></span>`fn eq(&self, other: &DestBufTooSmallError) -> bool` — [`DestBufTooSmallError`](#destbuftoosmallerror)

##### `impl StructuralPartialEq for DestBufTooSmallError`

##### `impl ToString for DestBufTooSmallError`

- <span id="destbuftoosmallerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `EncoderState`

```rust
struct EncoderState {
    code_idx: usize,
    num_bt_sent: u8,
    offset_idx: u8,
}
```

The [`EncoderState`](#encoderstate) is used to track the current state of a
streaming encoder. This struct does not contain the output buffer
(or a reference to one), and can be used when streaming the encoded
output to a custom data type

**IMPORTANT NOTE**: When implementing a custom streaming encoder,
the [`EncoderState`](#encoderstate) state machine assumes that the output buffer
**ALREADY** contains a single placeholder byte, and no other bytes.
This placeholder byte will be later modified with the first distance
to the next header/zero byte.

#### Implementations

- <span id="encoderstate-push"></span>`fn push(&mut self, data: u8) -> PushResult` — [`PushResult`](#pushresult)

  Push a single unencoded byte into the encoder state machine

- <span id="encoderstate-finalize"></span>`fn finalize(self) -> (usize, u8)`

  Finalize the encoding process for a single message.

  The byte at the given index should be replaced with the given value,

  and the sentinel value (typically 0u8) must be inserted at the current

  end of the output buffer, serving as a framing byte.

#### Trait Implementations

##### `impl Clone for EncoderState`

- <span id="encoderstate-clone"></span>`fn clone(&self) -> EncoderState` — [`EncoderState`](#encoderstate)

##### `impl Debug for EncoderState`

- <span id="encoderstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EncoderState`

- <span id="encoderstate-default"></span>`fn default() -> Self`

  Create a default initial state representation for a COBS encoder

## Enums

### `DecoderState`

```rust
enum DecoderState {
    Idle,
    Grab(u8),
    GrabChain(u8),
}
```

The [`DecoderState`](#decoderstate) is used to track the current state of a
streaming decoder. This struct does not contain the output buffer
(or a reference to one), and can be used when streaming the decoded
output to a custom data type.

#### Variants

- **`Idle`**

  State machine has not received any non-zero bytes

- **`Grab`**

  1-254 bytes, can be header or 00

- **`GrabChain`**

  255 bytes, will be a header next

#### Implementations

- <span id="decoderstate-feed"></span>`fn feed(&mut self, data: u8) -> Result<DecodeResult, DecodeError>` — [`DecodeResult`](#decoderesult), [`DecodeError`](#decodeerror)

  Push a single encoded byte into the state machine. If the input was

  unexpected, such as an early end of a framed message segment, an Error will

  be returned, and the current associated output buffer contents should be discarded.

  

  If a complete message is indicated, the decoding state machine will automatically

  reset itself to the Idle state, and may be used to begin decoding another message.

  

  NOTE: Sentinel value must be included in the input to this function for the

  decoding to complete

#### Trait Implementations

##### `impl Debug for DecoderState`

- <span id="decoderstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DecodeResult`

```rust
enum DecodeResult {
    NoData,
    DataComplete,
    DataContinue(u8),
}
```

[`DecodeResult`](#decoderesult) represents the possible non-error outcomes of
pushing an encoded data byte into the [`DecoderState`](#decoderstate) state machine

#### Variants

- **`NoData`**

  The given input byte did not prompt an output byte, either because the
  state machine is still idle, or we have just processed a header byte.
  More data is needed to complete the message.

- **`DataComplete`**

  We have received a complete and well-encoded COBS message. The
  contents of the associated output buffer may now be used

- **`DataContinue`**

  The following byte should be appended to the current end of the decoded
  output buffer.
  More data is needed to complete the message.

### `DecodeError`

```rust
enum DecodeError {
    EmptyFrame,
    InvalidFrame {
        decoded_bytes: usize,
    },
    TargetBufTooSmall,
}
```

#### Trait Implementations

##### `impl Debug for DecodeError`

- <span id="decodeerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecodeError`

- <span id="decodeerror-display-fmt"></span>`fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for DecodeError`

##### `impl Error for DecodeError`

##### `impl PartialEq for DecodeError`

- <span id="decodeerror-partialeq-eq"></span>`fn eq(&self, other: &DecodeError) -> bool` — [`DecodeError`](#decodeerror)

##### `impl StructuralPartialEq for DecodeError`

##### `impl ToString for DecodeError`

- <span id="decodeerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `PushResult`

```rust
enum PushResult {
    AddSingle(u8),
    ModifyFromStartAndSkip((usize, u8)),
    ModifyFromStartAndPushAndSkip((usize, u8, u8)),
}
```

[`PushResult`](#pushresult) is used to represent the changes to an (encoded)
output data buffer when an unencoded byte is pushed into [`EncoderState`](#encoderstate).

#### Variants

- **`AddSingle`**

  The returned byte should be placed at the current end of the data buffer

- **`ModifyFromStartAndSkip`**

  The byte at the given index should be replaced with the given byte.
  Additionally, a placeholder byte should be inserted at the current
  end of the output buffer to be later modified

- **`ModifyFromStartAndPushAndSkip`**

  The byte at the given index should be replaced with the given byte.
  Then, the last u8 in this tuple should be inserted at the end of the
  current output buffer. Finally, a placeholder byte should be inserted at
  the current end of the output buffer to be later modified if the encoding process is
  not done yet.

## Functions

### `max_encoding_overhead`

```rust
const fn max_encoding_overhead(source_len: usize) -> usize
```

Calculates the maximum overhead when encoding a message with the given length.
The overhead is a maximum of [n/254] bytes (one in 254 bytes) rounded up.

### `max_encoding_length`

```rust
const fn max_encoding_length(source_len: usize) -> usize
```

Calculates the maximum possible size of an encoded message given the length
of the source message. This may be useful for calculating how large the
`dest` buffer needs to be in the encoding functions.

### `add`

```rust
fn add(to: &mut [u8], idx: usize, data: u8) -> Result<(), DecodeError>
```

### `decode`

```rust
fn decode(source: &[u8], dest: &mut [u8]) -> Result<usize, DecodeError>
```

Decodes the `source` buffer into the `dest` buffer.

This function uses the typical sentinel value of 0.

# Failures

This will return `Err(())` if there was a decoding error. Otherwise,
it will return `Ok(n)` where `n` is the length of the decoded message.

### `decode_in_place_report`

```rust
fn decode_in_place_report(buf: &mut [u8]) -> Result<DecodeReport, DecodeError>
```

Decodes a message in-place.

This is the same function as [decode_in_place](#decode-in-place), but provides a report
of both the number of source bytes consumed as well as the size of the
destination used.

### `decode_in_place`

```rust
fn decode_in_place(buff: &mut [u8]) -> Result<usize, DecodeError>
```

Decodes a message in-place.

This is the same function as [decode](#decode), but replaces the encoded message
with the decoded message instead of writing to another buffer.

The returned `usize` is the number of bytes used for the DECODED value,
NOT the number of source bytes consumed during decoding.

### `decode_with_sentinel`

```rust
fn decode_with_sentinel(source: &[u8], dest: &mut [u8], sentinel: u8) -> Result<usize, DecodeError>
```

Decodes the `source` buffer into the `dest` buffer using an arbitrary sentinel value.

This is done by XOR-ing each byte of the source message with the chosen sentinel value,
which transforms the message into the same message encoded with a sentinel value of 0.
Then the regular decoding transformation is performed.

The returned `usize` is the number of bytes used for the DECODED value,
NOT the number of source bytes consumed during decoding.

### `decode_in_place_with_sentinel`

```rust
fn decode_in_place_with_sentinel(buff: &mut [u8], sentinel: u8) -> Result<usize, DecodeError>
```

Decodes a message in-place using an arbitrary sentinel value.

The returned `usize` is the number of bytes used for the DECODED value,
NOT the number of source bytes consumed during decoding.

### `encode`

```rust
fn encode(source: &[u8], dest: &mut [u8]) -> usize
```

Encodes the `source` buffer into the `dest` buffer.

This function assumes the typical sentinel value of 0, but does not terminate the encoded
message with the sentinel value. This should be done by the caller to ensure proper framing.

# Returns

The number of bytes written to in the `dest` buffer.

# Panics

This function will panic if the `dest` buffer is not large enough for the
encoded message. You can calculate the size the `dest` buffer needs to be with
the [crate::max_encoding_length] function.

### `try_encode`

```rust
fn try_encode(source: &[u8], dest: &mut [u8]) -> Result<usize, DestBufTooSmallError>
```

Attempts to encode the `source` buffer into the `dest` buffer.

This function assumes the typical sentinel value of 0, but does not terminate the encoded
message with the sentinel value. This should be done by the caller to ensure proper framing.

# Returns

The number of bytes written to in the `dest` buffer.

If the destination buffer does not have enough room, an error will be returned.

### `encode_with_sentinel`

```rust
fn encode_with_sentinel(source: &[u8], dest: &mut [u8], sentinel: u8) -> usize
```

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

