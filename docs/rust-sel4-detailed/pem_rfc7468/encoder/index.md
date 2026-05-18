*[pem_rfc7468](../index.md) / [encoder](index.md)*

---

# Module `encoder`

PEM encoder.

## Contents

- [Structs](#structs)
  - [`Encoder`](#encoder)
- [Functions](#functions)
  - [`encapsulated_len`](#encapsulated-len)
  - [`encapsulated_len_wrapped`](#encapsulated-len-wrapped)
  - [`encoded_len`](#encoded-len)
  - [`encode`](#encode)
  - [`encode_string`](#encode-string)
  - [`encapsulated_len_inner`](#encapsulated-len-inner)
  - [`base64_len_wrapped`](#base64-len-wrapped)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Encoder`](#encoder) | struct | Buffered PEM encoder. |
| [`encapsulated_len`](#encapsulated-len) | fn | Compute the length of a PEM encoded document which encapsulates a Base64-encoded body including line endings every 64 characters. |
| [`encapsulated_len_wrapped`](#encapsulated-len-wrapped) | fn | Compute the length of a PEM encoded document with the Base64 body line wrapped at the specified `width`. |
| [`encoded_len`](#encoded-len) | fn | Get the length of a PEM encoded document with the given bytes and label. |
| [`encode`](#encode) | fn | Encode a PEM document according to RFC 7468's "Strict" grammar. |
| [`encode_string`](#encode-string) | fn | Encode a PEM document according to RFC 7468's "Strict" grammar, returning the result as a [`String`]. |
| [`encapsulated_len_inner`](#encapsulated-len-inner) | fn | Compute the encapsulated length of Base64 data of the given length. |
| [`base64_len_wrapped`](#base64-len-wrapped) | fn | Compute Base64 length line-wrapped at the specified width with the given line ending. |

## Structs

### `Encoder<'l, 'o>`

```rust
struct Encoder<'l, 'o> {
    type_label: &'l str,
    line_ending: crate::LineEnding,
    base64: crate::Base64Encoder<'o>,
}
```

Buffered PEM encoder.

Stateful buffered encoder type which encodes an input PEM document according
to RFC 7468's "Strict" grammar.

#### Fields

- **`type_label`**: `&'l str`

  PEM type label.

- **`line_ending`**: `crate::LineEnding`

  Line ending used to wrap Base64.

- **`base64`**: `crate::Base64Encoder<'o>`

  Buffered Base64 encoder.

#### Implementations

- <span id="encoder-new"></span>`fn new(type_label: &'l str, line_ending: LineEnding, out: &'o mut [u8]) -> Result<Self>` — [`LineEnding`](../index.md#lineending), [`Result`](../error/index.md#result)

  Create a new PEM [`Encoder`](#encoder) with the default options which

  writes output into the provided buffer.

  

  Uses the default 64-character line wrapping.

- <span id="encoder-new-wrapped"></span>`fn new_wrapped(type_label: &'l str, line_width: usize, line_ending: LineEnding, out: &'o mut [u8]) -> Result<Self>` — [`LineEnding`](../index.md#lineending), [`Result`](../error/index.md#result)

  Create a new PEM [`Encoder`](#encoder) which wraps at the given line width.

  

  Note that per [RFC7468 § 2] encoding PEM with any other wrap width besides

  64 is technically non-compliant:

  

  > Generators MUST wrap the base64-encoded lines so that each line

  > consists of exactly 64 characters except for the final line, which

  > will encode the remainder of the data (within the 64-character line

  > boundary)

  

  This method is provided with the intended purpose of implementing the

  OpenSSH private key format, which uses a non-standard wrap width of 70.

- <span id="encoder-type-label"></span>`fn type_label(&self) -> &'l str`

  Get the PEM type label used for this document.

- <span id="encoder-encode"></span>`fn encode(&mut self, input: &[u8]) -> Result<()>` — [`Result`](../error/index.md#result)

  Encode the provided input data.

  

  This method can be called as many times as needed with any sized input

  to write data encoded data into the output buffer, so long as there is

  sufficient space in the buffer to handle the resulting Base64 encoded

  data.

- <span id="encoder-base64-encoder"></span>`fn base64_encoder(&mut self) -> &mut Base64Encoder<'o>` — [`Base64Encoder`](../index.md#base64encoder)

  Borrow the inner [`Base64Encoder`](../index.md).

- <span id="encoder-finish"></span>`fn finish(self) -> Result<usize>` — [`Result`](../error/index.md#result)

  Finish encoding PEM, writing the post-encapsulation boundary.

  

  On success, returns the total number of bytes written to the output

  buffer.

## Functions

### `encapsulated_len`

```rust
fn encapsulated_len(label: &str, line_ending: crate::LineEnding, input_len: usize) -> crate::Result<usize>
```

Compute the length of a PEM encoded document which encapsulates a
Base64-encoded body including line endings every 64 characters.

The `input_len` parameter specifies the length of the raw input
bytes prior to Base64 encoding.

Note that the current implementation of this function computes an upper
bound of the length and the actual encoded document may be slightly shorter
(typically 1-byte). Downstream consumers of this function should check the
actual encoded length and potentially truncate buffers allocated using this
function to estimate the encapsulated size.

Use [`encoded_len`](#encoded-len) (when possible) to obtain a precise length.

## Returns
- `Ok(len)` on success
- `Err(Error::Length)` on length overflow

### `encapsulated_len_wrapped`

```rust
fn encapsulated_len_wrapped(label: &str, line_width: usize, line_ending: crate::LineEnding, input_len: usize) -> crate::Result<usize>
```

Compute the length of a PEM encoded document with the Base64 body
line wrapped at the specified `width`.

This is the same as [`encapsulated_len`](#encapsulated-len), which defaults to a width of 64.

Note that per [RFC7468 § 2] encoding PEM with any other wrap width besides
64 is technically non-compliant:

> Generators MUST wrap the base64-encoded lines so that each line
> consists of exactly 64 characters except for the final line, which
> will encode the remainder of the data (within the 64-character line
> boundary)


### `encoded_len`

```rust
fn encoded_len(label: &str, line_ending: crate::LineEnding, input: &[u8]) -> crate::Result<usize>
```

Get the length of a PEM encoded document with the given bytes and label.

This function computes a precise length of the PEM encoding of the given
`input` data.

## Returns
- `Ok(len)` on success
- `Err(Error::Length)` on length overflow

### `encode`

```rust
fn encode<'o>(type_label: &str, line_ending: crate::LineEnding, input: &[u8], buf: &'o mut [u8]) -> crate::Result<&'o str>
```

Encode a PEM document according to RFC 7468's "Strict" grammar.

### `encode_string`

```rust
fn encode_string(label: &str, line_ending: crate::LineEnding, input: &[u8]) -> crate::Result<alloc::string::String>
```

Encode a PEM document according to RFC 7468's "Strict" grammar, returning
the result as a [`String`](../../clap_builder/index.md).

### `encapsulated_len_inner`

```rust
fn encapsulated_len_inner(label: &str, line_ending: crate::LineEnding, base64_len: usize) -> crate::Result<usize>
```

Compute the encapsulated length of Base64 data of the given length.

### `base64_len_wrapped`

```rust
fn base64_len_wrapped(base64_len: usize, line_width: usize, line_ending: crate::LineEnding) -> crate::Result<usize>
```

Compute Base64 length line-wrapped at the specified width with the given
line ending.

