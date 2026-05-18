*[pem_rfc7468](../index.md) / [decoder](index.md)*

---

# Module `decoder`

Decoder for PEM encapsulated data.

From RFC 7468 Section 2:

> Textual encoding begins with a line comprising "-----BEGIN ", a
> label, and "-----", and ends with a line comprising "-----END ", a
> label, and "-----".  Between these lines, or "encapsulation
> boundaries", are base64-encoded data according to Section 4 of
> [RFC 4648].


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Decoder`](#decoder) | struct | Buffered PEM decoder. |
| [`Encapsulation`](#encapsulation) | struct | PEM encapsulation parser. |
| [`decode`](#decode) | fn | Decode a PEM document according to RFC 7468's "Strict" grammar. |
| [`decode_vec`](#decode-vec) | fn | Decode a PEM document according to RFC 7468's "Strict" grammar, returning the result as a [`Vec`] upon success. |
| [`decode_label`](#decode-label) | fn | Decode the encapsulation boundaries of a PEM document according to RFC 7468's "Strict" grammar. |
| [`check_for_headers`](#check-for-headers) | fn | Check for PEM headers in the input, as they are disallowed by RFC7468. |

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

- <span id="decoder-new"></span>`fn new(pem: &'i [u8]) -> Result<Self>` — [`Result`](../error/index.md#result)

  Create a new PEM [`Decoder`](#decoder) with the default options.

  

  Uses the default 64-character line wrapping.

- <span id="decoder-new-wrapped"></span>`fn new_wrapped(pem: &'i [u8], line_width: usize) -> Result<Self>` — [`Result`](../error/index.md#result)

  Create a new PEM [`Decoder`](#decoder) which wraps at the given line width.

- <span id="decoder-type-label"></span>`fn type_label(&self) -> &'i str`

  Get the PEM type label for the input document.

- <span id="decoder-decode"></span>`fn decode<'o>(&mut self, buf: &'o mut [u8]) -> Result<&'o [u8]>` — [`Result`](../error/index.md#result)

  Decode data into the provided output buffer.

  

  There must be at least as much remaining Base64 input to be decoded

  in order to completely fill `buf`.

- <span id="decoder-decode-to-end"></span>`fn decode_to_end<'o>(&mut self, buf: &'o mut Vec<u8>) -> Result<&'o [u8]>` — [`Result`](../error/index.md#result)

  Decode all of the remaining data in the input buffer into `buf`.

- <span id="decoder-remaining-len"></span>`fn remaining_len(&self) -> usize`

  Get the decoded length of the remaining PEM data after Base64 decoding.

- <span id="decoder-is-finished"></span>`fn is_finished(&self) -> bool`

  Are we finished decoding the PEM input?

#### Trait Implementations

##### `impl Clone for Decoder<'i>`

- <span id="decoder-clone"></span>`fn clone(&self) -> Decoder<'i>` — [`Decoder`](#decoder)

### `Encapsulation<'a>`

```rust
struct Encapsulation<'a> {
    label: &'a str,
    encapsulated_text: &'a [u8],
}
```

PEM encapsulation parser.

This parser performs an initial pass over the data, locating the
pre-encapsulation (`---BEGIN [...]---`) and post-encapsulation
(`---END [...]`) boundaries while attempting to avoid branching
on the potentially secret Base64-encoded data encapsulated between
the two boundaries.

It only supports a single encapsulated message at present. Future work
could potentially include extending it provide an iterator over a series
of encapsulated messages.

#### Fields

- **`label`**: `&'a str`

  Type label extracted from the pre/post-encapsulation boundaries.
  
  From RFC 7468 Section 2:
  
  > The type of data encoded is labeled depending on the type label in
  > the "-----BEGIN " line (pre-encapsulation boundary).  For example,
  > the line may be "-----BEGIN CERTIFICATE-----" to indicate that the
  > content is a PKIX certificate (see further below).  Generators MUST
  > put the same label on the "-----END " line (post-encapsulation
  > boundary) as the corresponding "-----BEGIN " line.  Labels are
  > formally case-sensitive, uppercase, and comprised of zero or more
  > characters; they do not contain consecutive spaces or hyphen-minuses,
  > nor do they contain spaces or hyphen-minuses at either end.  Parsers
  > MAY disregard the label in the post-encapsulation boundary instead of
  > signaling an error if there is a label mismatch: some extant
  > implementations require the labels to match; others do not.

- **`encapsulated_text`**: `&'a [u8]`

  Encapsulated text portion contained between the boundaries.
  
  This data should be encoded as Base64, however this type performs no
  validation of it so it can be handled in constant-time.

#### Implementations

- <span id="encapsulation-parse"></span>`fn parse(data: &'a [u8]) -> Result<Self>` — [`Result`](../error/index.md#result)

  Parse the type label and encapsulated text from between the

  pre/post-encapsulation boundaries.

- <span id="encapsulation-label"></span>`fn label(self) -> &'a str`

  Get the label parsed from the encapsulation boundaries.

#### Trait Implementations

##### `impl Clone for Encapsulation<'a>`

- <span id="encapsulation-clone"></span>`fn clone(&self) -> Encapsulation<'a>` — [`Encapsulation`](#encapsulation)

##### `impl Copy for Encapsulation<'a>`

##### `impl Debug for Encapsulation<'a>`

- <span id="encapsulation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `decode`

```rust
fn decode<'i, 'o>(pem: &'i [u8], buf: &'o mut [u8]) -> crate::Result<(&'i str, &'o [u8])>
```

Decode a PEM document according to RFC 7468's "Strict" grammar.

On success, writes the decoded document into the provided buffer, returning
the decoded label and the portion of the provided buffer containing the
decoded message.

### `decode_vec`

```rust
fn decode_vec(pem: &[u8]) -> crate::Result<(&str, alloc::vec::Vec<u8>)>
```

Decode a PEM document according to RFC 7468's "Strict" grammar, returning
the result as a [`Vec`](../../addr2line/maybe_small/index.md) upon success.

### `decode_label`

```rust
fn decode_label(pem: &[u8]) -> crate::Result<&str>
```

Decode the encapsulation boundaries of a PEM document according to RFC 7468's "Strict" grammar.

On success, returning the decoded label.

### `check_for_headers`

```rust
fn check_for_headers(pem: &[u8], err: crate::Error) -> crate::Error
```

Check for PEM headers in the input, as they are disallowed by RFC7468.

Returns `Error::HeaderDisallowed` if headers are encountered.

