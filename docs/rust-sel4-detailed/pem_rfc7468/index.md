# Crate `pem_rfc7468`

# [RustCrypto]: PEM Encoding ([RFC 7468])

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]

Pure Rust implementation of PEM Encoding ([RFC 7468]) for PKIX, PKCS, and
CMS Structures, a strict subset of the original Privacy-Enhanced Mail encoding
intended  specifically for use with cryptographic keys, certificates, and other
messages.

Provides a `no_std`-friendly, constant-time implementation suitable for use with
cryptographic private keys.

[Documentation][docs-link]

## About

Many cryptography-related document formats, such as certificates (PKIX),
private and public keys/keypairs (PKCS), and other cryptographic messages (CMS)
provide an ASCII encoding which can be traced back to Privacy-Enhanced Mail
(PEM) as defined [RFC 1421], which look like the following:

```text
-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIBftnHPp22SewYmmEoMcX8VwI4IHwaqd+9LFPj/15eqF
-----END PRIVATE KEY-----
```

However, all of these formats actually implement a text-based encoding that is
similar but *not* identical to the legacy PEM encoding as described in
[RFC 1421].

For this reason, [RFC 7468] was created to describe a stricter form of
"PEM encoding" for use in these applications which codifies the previously
de facto rules that most implementations operate by, and makes recommendations
to promote interoperability.

This crate provides a strict interpretation of the [RFC 7468] rules,
implementing MUSTs and SHOULDs while avoiding the MAYs, targeting the
"ABNF (Strict)" subset of the grammar as described in
[RFC 7468 Section 3 Figure 3 (p6)][RFC 7468 p6].

## Implementation notes

- `no_std`-friendly core implementation which requires no heap allocations
  and avoids copies and temporary buffers.
- Optional `alloc`-dependent convenience features and buffered decoder/encoder.
- Uses the [`base64ct`](#base64ct) crate to decode/encode Base64 in constant-time.
- PEM parser avoids branching on potentially secret data as much as possible.

The paper `Util::Lookup: Exploiting key decoding in cryptographic libraries`
demonstrates how the leakage from non-constant-time PEM parsers can be used
to practically extract RSA private keys from SGX enclaves.

## Minimum Supported Rust Version

This crate requires **Rust 1.60** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a minor
version bump.

## License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)










[//]: # (links)






# Usage

 ```ignore
fn main() -> Result<(), Box<dyn std::error::Error>> {
/// Example PEM document
/// NOTE: do not actually put private key literals into your source code!!!
let example_pem = "\
-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIBftnHPp22SewYmmEoMcX8VwI4IHwaqd+9LFPj/15eqF
-----END PRIVATE KEY-----
";

// Decode PEM
let (type_label, data) = pem_rfc7468::decode_vec(example_pem.as_bytes())?;
assert_eq!(type_label, "PRIVATE KEY");
assert_eq!(
    data,
    &[
        48, 46, 2, 1, 0, 48, 5, 6, 3, 43, 101, 112, 4, 34, 4, 32, 23, 237, 156, 115, 233, 219,
        100, 158, 193, 137, 166, 18, 131, 28, 95, 197, 112, 35, 130, 7, 193, 170, 157, 251,
        210, 197, 62, 63, 245, 229, 234, 133
    ]
);

// Encode PEM
use pem_rfc7468::LineEnding;
let encoded_pem = pem_rfc7468::encode_string(type_label, LineEnding::default(), &data)?;
assert_eq!(&encoded_pem, example_pem);
Ok(())
}
```

## Contents

- [Modules](#modules)
  - [`decoder`](#decoder)
  - [`encoder`](#encoder)
  - [`error`](#error)
  - [`grammar`](#grammar)
- [Structs](#structs)
  - [`Decoder`](#decoder)
  - [`Encoder`](#encoder)
- [Enums](#enums)
  - [`Error`](#error)
- [Traits](#traits)
  - [`PemLabel`](#pemlabel)
- [Functions](#functions)
  - [`decode`](#decode)
  - [`decode_label`](#decode-label)
  - [`encapsulated_len`](#encapsulated-len)
  - [`encapsulated_len_wrapped`](#encapsulated-len-wrapped)
  - [`encode`](#encode)
  - [`encoded_len`](#encoded-len)
  - [`decode_vec`](#decode-vec)
  - [`encode_string`](#encode-string)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
  - [`Base64Decoder`](#base64decoder)
  - [`Base64Encoder`](#base64encoder)
- [Constants](#constants)
  - [`PRE_ENCAPSULATION_BOUNDARY`](#pre-encapsulation-boundary)
  - [`POST_ENCAPSULATION_BOUNDARY`](#post-encapsulation-boundary)
  - [`ENCAPSULATION_BOUNDARY_DELIMITER`](#encapsulation-boundary-delimiter)
  - [`BASE64_WRAP_WIDTH`](#base64-wrap-width)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`decoder`](#decoder) | mod | Decoder for PEM encapsulated data. |
| [`encoder`](#encoder) | mod | PEM encoder. |
| [`error`](#error) | mod | Error types |
| [`grammar`](#grammar) | mod | Helper functions and rules for enforcing the ABNF grammar for RFC 7468-flavored PEM as described in Section 3. |
| [`Decoder`](#decoder) | struct |  |
| [`Encoder`](#encoder) | struct |  |
| [`Error`](#error) | enum |  |
| [`PemLabel`](#pemlabel) | trait | Marker trait for types with an associated PEM type label. |
| [`decode`](#decode) | fn |  |
| [`decode_label`](#decode-label) | fn |  |
| [`encapsulated_len`](#encapsulated-len) | fn |  |
| [`encapsulated_len_wrapped`](#encapsulated-len-wrapped) | fn |  |
| [`encode`](#encode) | fn |  |
| [`encoded_len`](#encoded-len) | fn |  |
| [`decode_vec`](#decode-vec) | fn |  |
| [`encode_string`](#encode-string) | fn |  |
| [`Result`](#result) | type |  |
| [`Base64Decoder`](#base64decoder) | type | Buffered Base64 decoder type. |
| [`Base64Encoder`](#base64encoder) | type | Buffered Base64 encoder type. |
| [`PRE_ENCAPSULATION_BOUNDARY`](#pre-encapsulation-boundary) | const | The pre-encapsulation boundary appears before the encapsulated text. |
| [`POST_ENCAPSULATION_BOUNDARY`](#post-encapsulation-boundary) | const | The post-encapsulation boundary appears immediately after the encapsulated text. |
| [`ENCAPSULATION_BOUNDARY_DELIMITER`](#encapsulation-boundary-delimiter) | const | Delimiter of encapsulation boundaries. |
| [`BASE64_WRAP_WIDTH`](#base64-wrap-width) | const | Width at which the Base64 body of RFC7468-compliant PEM is wrapped. |

## Modules

- [`decoder`](decoder/index.md) — Decoder for PEM encapsulated data.
- [`encoder`](encoder/index.md) — PEM encoder.
- [`error`](error/index.md) — Error types
- [`grammar`](grammar/index.md) — Helper functions and rules for enforcing the ABNF grammar for

## Structs

### `Decoder<'i>`

```rust
struct Decoder<'i> {
    type_label: &'i str,
    base64: crate::Base64Decoder<'i>,
}
```

Buffered PEM decoder.

Stateful buffered decoder type which decodes an input PEM document according
to RFC 7468's "Strict" grammar.

#### Fields

- **`type_label`**: `&'i str`

  PEM type label.

- **`base64`**: `crate::Base64Decoder<'i>`

  Buffered Base64 decoder.

#### Implementations

- <span id="decoder-new"></span>`fn new(pem: &'i [u8]) -> Result<Self>` — [`Result`](error/index.md#result)

  Create a new PEM [`Decoder`](decoder/index.md) with the default options.

  

  Uses the default 64-character line wrapping.

- <span id="decoder-new-wrapped"></span>`fn new_wrapped(pem: &'i [u8], line_width: usize) -> Result<Self>` — [`Result`](error/index.md#result)

  Create a new PEM [`Decoder`](decoder/index.md) which wraps at the given line width.

- <span id="decoder-type-label"></span>`fn type_label(&self) -> &'i str`

  Get the PEM type label for the input document.

- <span id="decoder-decode"></span>`fn decode<'o>(&mut self, buf: &'o mut [u8]) -> Result<&'o [u8]>` — [`Result`](error/index.md#result)

  Decode data into the provided output buffer.

  

  There must be at least as much remaining Base64 input to be decoded

  in order to completely fill `buf`.

- <span id="decoder-decode-to-end"></span>`fn decode_to_end<'o>(&mut self, buf: &'o mut Vec<u8>) -> Result<&'o [u8]>` — [`Result`](error/index.md#result)

  Decode all of the remaining data in the input buffer into `buf`.

- <span id="decoder-remaining-len"></span>`fn remaining_len(&self) -> usize`

  Get the decoded length of the remaining PEM data after Base64 decoding.

- <span id="decoder-is-finished"></span>`fn is_finished(&self) -> bool`

  Are we finished decoding the PEM input?

#### Trait Implementations

##### `impl Clone for Decoder<'i>`

- <span id="decoder-clone"></span>`fn clone(&self) -> Decoder<'i>` — [`Decoder`](decoder/index.md#decoder)

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

- <span id="encoder-new"></span>`fn new(type_label: &'l str, line_ending: LineEnding, out: &'o mut [u8]) -> Result<Self>` — [`LineEnding`](#lineending), [`Result`](error/index.md#result)

  Create a new PEM [`Encoder`](encoder/index.md) with the default options which

  writes output into the provided buffer.

  

  Uses the default 64-character line wrapping.

- <span id="encoder-new-wrapped"></span>`fn new_wrapped(type_label: &'l str, line_width: usize, line_ending: LineEnding, out: &'o mut [u8]) -> Result<Self>` — [`LineEnding`](#lineending), [`Result`](error/index.md#result)

  Create a new PEM [`Encoder`](encoder/index.md) which wraps at the given line width.

  

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

- <span id="encoder-encode"></span>`fn encode(&mut self, input: &[u8]) -> Result<()>` — [`Result`](error/index.md#result)

  Encode the provided input data.

  

  This method can be called as many times as needed with any sized input

  to write data encoded data into the output buffer, so long as there is

  sufficient space in the buffer to handle the resulting Base64 encoded

  data.

- <span id="encoder-base64-encoder"></span>`fn base64_encoder(&mut self) -> &mut Base64Encoder<'o>` — [`Base64Encoder`](#base64encoder)

  Borrow the inner [`Base64Encoder`](#base64encoder).

- <span id="encoder-finish"></span>`fn finish(self) -> Result<usize>` — [`Result`](error/index.md#result)

  Finish encoding PEM, writing the post-encapsulation boundary.

  

  On success, returns the total number of bytes written to the output

  buffer.

## Enums

### `Error`

```rust
enum Error {
    Base64(base64ct::Error),
    CharacterEncoding,
    EncapsulatedText,
    HeaderDisallowed,
    Label,
    Length,
    Preamble,
    PreEncapsulationBoundary,
    PostEncapsulationBoundary,
    UnexpectedTypeLabel {
        expected: &'static str,
    },
}
```

PEM errors.

#### Variants

- **`Base64`**

  Base64-related errors.

- **`CharacterEncoding`**

  Character encoding-related errors.

- **`EncapsulatedText`**

  Errors in the encapsulated text (which aren't specifically Base64-related).

- **`HeaderDisallowed`**

  Header detected in the encapsulated text.

- **`Label`**

  Invalid label.

- **`Length`**

  Invalid length.

- **`Preamble`**

  "Preamble" (text before pre-encapsulation boundary) contains invalid data.

- **`PreEncapsulationBoundary`**

  Errors in the pre-encapsulation boundary.

- **`PostEncapsulationBoundary`**

  Errors in the post-encapsulation boundary.

- **`UnexpectedTypeLabel`**

  Unexpected PEM type label.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](error/index.md#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](error/index.md#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `PemLabel`

```rust
trait PemLabel { ... }
```

Marker trait for types with an associated PEM type label.

#### Associated Constants

- `const PEM_LABEL: &'static str`

#### Provided Methods

- `fn validate_pem_label(actual: &str) -> Result<()>`

  Validate that a given label matches the expected label.

## Functions

### `decode`

```rust
fn decode<'i, 'o>(pem: &'i [u8], buf: &'o mut [u8]) -> crate::Result<(&'i str, &'o [u8])>
```

Decode a PEM document according to RFC 7468's "Strict" grammar.

On success, writes the decoded document into the provided buffer, returning
the decoded label and the portion of the provided buffer containing the
decoded message.

### `decode_label`

```rust
fn decode_label(pem: &[u8]) -> crate::Result<&str>
```

Decode the encapsulation boundaries of a PEM document according to RFC 7468's "Strict" grammar.

On success, returning the decoded label.

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

Use [`encoded_len`](encoder/index.md) (when possible) to obtain a precise length.

## Returns
- `Ok(len)` on success
- `Err(Error::Length)` on length overflow

### `encapsulated_len_wrapped`

```rust
fn encapsulated_len_wrapped(label: &str, line_width: usize, line_ending: crate::LineEnding, input_len: usize) -> crate::Result<usize>
```

Compute the length of a PEM encoded document with the Base64 body
line wrapped at the specified `width`.

This is the same as [`encapsulated_len`](encoder/index.md), which defaults to a width of 64.

Note that per [RFC7468 § 2] encoding PEM with any other wrap width besides
64 is technically non-compliant:

> Generators MUST wrap the base64-encoded lines so that each line
> consists of exactly 64 characters except for the final line, which
> will encode the remainder of the data (within the 64-character line
> boundary)


### `encode`

```rust
fn encode<'o>(type_label: &str, line_ending: crate::LineEnding, input: &[u8], buf: &'o mut [u8]) -> crate::Result<&'o str>
```

Encode a PEM document according to RFC 7468's "Strict" grammar.

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

### `decode_vec`

```rust
fn decode_vec(pem: &[u8]) -> crate::Result<(&str, alloc::vec::Vec<u8>)>
```

Decode a PEM document according to RFC 7468's "Strict" grammar, returning
the result as a [`Vec`](../addr2line/maybe_small/index.md) upon success.

### `encode_string`

```rust
fn encode_string(label: &str, line_ending: crate::LineEnding, input: &[u8]) -> crate::Result<alloc::string::String>
```

Encode a PEM document according to RFC 7468's "Strict" grammar, returning
the result as a [`String`](../clap_builder/index.md).

## Type Aliases

### `Result<T>`

```rust
type Result<T> = core::result::Result<T, Error>;
```

Result type with the `pem-rfc7468` crate's [`Error`](error/index.md) type.

### `Base64Decoder<'i>`

```rust
type Base64Decoder<'i> = base64ct::Decoder<'i, base64ct::Base64>;
```

Buffered Base64 decoder type.

### `Base64Encoder<'o>`

```rust
type Base64Encoder<'o> = base64ct::Encoder<'o, base64ct::Base64>;
```

Buffered Base64 encoder type.

## Constants

### `PRE_ENCAPSULATION_BOUNDARY`
```rust
const PRE_ENCAPSULATION_BOUNDARY: &[u8];
```

The pre-encapsulation boundary appears before the encapsulated text.

From RFC 7468 Section 2:
> There are exactly five hyphen-minus (also known as dash) characters ("-")
> on both ends of the encapsulation boundaries, no more, no less.

### `POST_ENCAPSULATION_BOUNDARY`
```rust
const POST_ENCAPSULATION_BOUNDARY: &[u8];
```

The post-encapsulation boundary appears immediately after the encapsulated text.

### `ENCAPSULATION_BOUNDARY_DELIMITER`
```rust
const ENCAPSULATION_BOUNDARY_DELIMITER: &[u8];
```

Delimiter of encapsulation boundaries.

### `BASE64_WRAP_WIDTH`
```rust
const BASE64_WRAP_WIDTH: usize = 64usize;
```

Width at which the Base64 body of RFC7468-compliant PEM is wrapped.

From [RFC7468 § 2]:

> Generators MUST wrap the base64-encoded lines so that each line
> consists of exactly 64 characters except for the final line, which
> will encode the remainder of the data (within the 64-character line
> boundary), and they MUST NOT emit extraneous whitespace.  Parsers MAY
> handle other line sizes.


