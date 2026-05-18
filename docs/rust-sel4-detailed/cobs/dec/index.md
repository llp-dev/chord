*[cobs](../index.md) / [dec](index.md)*

---

# Module `dec`

## Contents

- [Structs](#structs)
  - [`CobsDecoder`](#cobsdecoder)
  - [`DecodeReport`](#decodereport)
- [Enums](#enums)
  - [`DecoderState`](#decoderstate)
  - [`DecodeResult`](#decoderesult)
  - [`DecodeError`](#decodeerror)
- [Functions](#functions)
  - [`add`](#add)
  - [`decode`](#decode)
  - [`decode_in_place_report`](#decode-in-place-report)
  - [`decode_in_place`](#decode-in-place)
  - [`decode_with_sentinel`](#decode-with-sentinel)
  - [`decode_in_place_with_sentinel`](#decode-in-place-with-sentinel)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CobsDecoder`](#cobsdecoder) | struct | The [`CobsDecoder`] type is used to decode a stream of bytes to a given mutable output slice. |
| [`DecodeReport`](#decodereport) | struct | A report of the source and destination bytes used during in-place decoding |
| [`DecoderState`](#decoderstate) | enum | The [`DecoderState`] is used to track the current state of a streaming decoder. |
| [`DecodeResult`](#decoderesult) | enum | [`DecodeResult`] represents the possible non-error outcomes of pushing an encoded data byte into the [`DecoderState`] state machine |
| [`DecodeError`](#decodeerror) | enum |  |
| [`add`](#add) | fn |  |
| [`decode`](#decode) | fn | Decodes the `source` buffer into the `dest` buffer. |
| [`decode_in_place_report`](#decode-in-place-report) | fn | Decodes a message in-place. |
| [`decode_in_place`](#decode-in-place) | fn | Decodes a message in-place. |
| [`decode_with_sentinel`](#decode-with-sentinel) | fn | Decodes the `source` buffer into the `dest` buffer using an arbitrary sentinel value. |
| [`decode_in_place_with_sentinel`](#decode-in-place-with-sentinel) | fn | Decodes a message in-place using an arbitrary sentinel value. |

## Structs

### `CobsDecoder<'a>`

```rust
struct CobsDecoder<'a> {
    dest: &'a mut [u8],
    dest_idx: usize,
    state: DecoderState,
}
```

The [`CobsDecoder`](../index.md) type is used to decode a stream of bytes to a
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

- <span id="cobsdecoder-new"></span>`fn new(dest: &'a mut [u8]) -> CobsDecoder<'a>` — [`CobsDecoder`](../index.md#cobsdecoder)

  Create a new streaming Cobs Decoder. Provide the output buffer

  for the decoded message to be placed in

- <span id="cobsdecoder-feed"></span>`fn feed(&mut self, data: u8) -> Result<Option<usize>, DecodeError>` — [`DecodeError`](../index.md#decodeerror)

  Push a single byte into the streaming CobsDecoder. Return values mean:

  

  * Ok(None) - State machine okay, more data needed

  * Ok(Some(N)) - A message of N bytes was successfully decoded

  * Err([DecodeError]) - Message decoding failed

  

  NOTE: Sentinel value must be included in the input to this function for the

  decoding to complete

- <span id="cobsdecoder-push"></span>`fn push(&mut self, data: &[u8]) -> Result<Option<(usize, usize)>, DecodeError>` — [`DecodeError`](../index.md#decodeerror)

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

- <span id="decodereport-clone"></span>`fn clone(&self) -> DecodeReport` — [`DecodeReport`](../index.md#decodereport)

##### `impl Copy for DecodeReport`

##### `impl Debug for DecodeReport`

- <span id="decodereport-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `DecoderState`

```rust
enum DecoderState {
    Idle,
    Grab(u8),
    GrabChain(u8),
}
```

The [`DecoderState`](../index.md) is used to track the current state of a
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

- <span id="decoderstate-feed"></span>`fn feed(&mut self, data: u8) -> Result<DecodeResult, DecodeError>` — [`DecodeResult`](../index.md#decoderesult), [`DecodeError`](../index.md#decodeerror)

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

[`DecodeResult`](../index.md) represents the possible non-error outcomes of
pushing an encoded data byte into the [`DecoderState`](../index.md) state machine

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

- <span id="decodeerror-partialeq-eq"></span>`fn eq(&self, other: &DecodeError) -> bool` — [`DecodeError`](../index.md#decodeerror)

##### `impl StructuralPartialEq for DecodeError`

##### `impl ToString for DecodeError`

- <span id="decodeerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

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

This is the same function as [`decode_in_place`](../index.md), but provides a report
of both the number of source bytes consumed as well as the size of the
destination used.

### `decode_in_place`

```rust
fn decode_in_place(buff: &mut [u8]) -> Result<usize, DecodeError>
```

Decodes a message in-place.

This is the same function as [`decode`](../index.md), but replaces the encoded message
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

