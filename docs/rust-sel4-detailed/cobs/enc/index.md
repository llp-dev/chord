*[cobs](../index.md) / [enc](index.md)*

---

# Module `enc`

## Contents

- [Structs](#structs)
  - [`CobsEncoder`](#cobsencoder)
  - [`DestBufTooSmallError`](#destbuftoosmallerror)
  - [`EncoderState`](#encoderstate)
- [Enums](#enums)
  - [`PushResult`](#pushresult)
- [Functions](#functions)
  - [`encode`](#encode)
  - [`try_encode`](#try-encode)
  - [`encode_with_sentinel`](#encode-with-sentinel)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CobsEncoder`](#cobsencoder) | struct | The [`CobsEncoder`] type is used to encode a stream of bytes to a given mutable output slice. |
| [`DestBufTooSmallError`](#destbuftoosmallerror) | struct |  |
| [`EncoderState`](#encoderstate) | struct | The [`EncoderState`] is used to track the current state of a streaming encoder. |
| [`PushResult`](#pushresult) | enum | [`PushResult`] is used to represent the changes to an (encoded) output data buffer when an unencoded byte is pushed into [`EncoderState`]. |
| [`encode`](#encode) | fn | Encodes the `source` buffer into the `dest` buffer. |
| [`try_encode`](#try-encode) | fn | Attempts to encode the `source` buffer into the `dest` buffer. |
| [`encode_with_sentinel`](#encode-with-sentinel) | fn | Encodes the `source` buffer into the `dest` buffer using an arbitrary sentinel value. |

## Structs

### `CobsEncoder<'a>`

```rust
struct CobsEncoder<'a> {
    dest: &'a mut [u8],
    dest_idx: usize,
    state: EncoderState,
    might_be_done: bool,
}
```

The [`CobsEncoder`](../index.md) type is used to encode a stream of bytes to a
given mutable output slice. This is often useful when heap data
structures are not available, or when not all message bytes are
received at a single point in time.

#### Implementations

- <span id="cobsencoder-new"></span>`fn new(out_buf: &'a mut [u8]) -> CobsEncoder<'a>` — [`CobsEncoder`](../index.md#cobsencoder)

  Create a new streaming Cobs Encoder

- <span id="cobsencoder-push"></span>`fn push(&mut self, data: &[u8]) -> Result<(), DestBufTooSmallError>` — [`DestBufTooSmallError`](../index.md#destbuftoosmallerror)

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

- <span id="destbuftoosmallerror-partialeq-eq"></span>`fn eq(&self, other: &DestBufTooSmallError) -> bool` — [`DestBufTooSmallError`](../index.md#destbuftoosmallerror)

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

The [`EncoderState`](../index.md) is used to track the current state of a
streaming encoder. This struct does not contain the output buffer
(or a reference to one), and can be used when streaming the encoded
output to a custom data type

**IMPORTANT NOTE**: When implementing a custom streaming encoder,
the [`EncoderState`](../index.md) state machine assumes that the output buffer
**ALREADY** contains a single placeholder byte, and no other bytes.
This placeholder byte will be later modified with the first distance
to the next header/zero byte.

#### Implementations

- <span id="encoderstate-push"></span>`fn push(&mut self, data: u8) -> PushResult` — [`PushResult`](../index.md#pushresult)

  Push a single unencoded byte into the encoder state machine

- <span id="encoderstate-finalize"></span>`fn finalize(self) -> (usize, u8)`

  Finalize the encoding process for a single message.

  The byte at the given index should be replaced with the given value,

  and the sentinel value (typically 0u8) must be inserted at the current

  end of the output buffer, serving as a framing byte.

#### Trait Implementations

##### `impl Clone for EncoderState`

- <span id="encoderstate-clone"></span>`fn clone(&self) -> EncoderState` — [`EncoderState`](../index.md#encoderstate)

##### `impl Debug for EncoderState`

- <span id="encoderstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EncoderState`

- <span id="encoderstate-default"></span>`fn default() -> Self`

  Create a default initial state representation for a COBS encoder

## Enums

### `PushResult`

```rust
enum PushResult {
    AddSingle(u8),
    ModifyFromStartAndSkip((usize, u8)),
    ModifyFromStartAndPushAndSkip((usize, u8, u8)),
}
```

[`PushResult`](../index.md) is used to represent the changes to an (encoded)
output data buffer when an unencoded byte is pushed into [`EncoderState`](../index.md).

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

