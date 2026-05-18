*[pem_rfc7468](../index.md) / [grammar](index.md)*

---

# Module `grammar`

Helper functions and rules for enforcing the ABNF grammar for
RFC 7468-flavored PEM as described in Section 3.

The grammar described below is intended to follow the "ABNF (Strict)"
subset of the grammar as described in Section 3 Figure 3.

## Contents

- [Functions](#functions)
  - [`is_labelchar`](#is-labelchar)
  - [`is_allowed_in_label`](#is-allowed-in-label)
  - [`is_wsp`](#is-wsp)
  - [`strip_preamble`](#strip-preamble)
  - [`strip_leading_eol`](#strip-leading-eol)
  - [`strip_trailing_eol`](#strip-trailing-eol)
  - [`split_label`](#split-label)
  - [`validate_label`](#validate-label)
- [Constants](#constants)
  - [`CHAR_NUL`](#char-nul)
  - [`CHAR_HT`](#char-ht)
  - [`CHAR_SP`](#char-sp)
  - [`CHAR_CR`](#char-cr)
  - [`CHAR_LF`](#char-lf)
  - [`CHAR_COLON`](#char-colon)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`is_labelchar`](#is-labelchar) | fn | Any printable character except hyphen-minus, as defined in the 'labelchar' production in the RFC 7468 ABNF grammar |
| [`is_allowed_in_label`](#is-allowed-in-label) | fn | Does the provided byte match a character allowed in a label? |
| [`is_wsp`](#is-wsp) | fn | Does the provided byte match the "WSP" ABNF production from Section 3? |
| [`strip_preamble`](#strip-preamble) | fn | Strip the "preamble", i.e. data that appears before the PEM pre-encapsulation boundary. |
| [`strip_leading_eol`](#strip-leading-eol) | fn | Strip a newline (`eol`) from the beginning of the provided byte slice. |
| [`strip_trailing_eol`](#strip-trailing-eol) | fn | Strip a newline (`eol`) from the end of the provided byte slice. |
| [`split_label`](#split-label) | fn | Split a slice beginning with a type label as located in an encapsulation boundary. |
| [`validate_label`](#validate-label) | fn | Validate that the given bytes are allowed as a PEM type label, i.e. the label encoded in the `BEGIN` and `END` encapsulation boundaries. |
| [`CHAR_NUL`](#char-nul) | const | NUL char |
| [`CHAR_HT`](#char-ht) | const | Horizontal tab |
| [`CHAR_SP`](#char-sp) | const | Space |
| [`CHAR_CR`](#char-cr) | const | Carriage return |
| [`CHAR_LF`](#char-lf) | const | Line feed |
| [`CHAR_COLON`](#char-colon) | const | Colon ':' |

## Functions

### `is_labelchar`

```rust
fn is_labelchar(char: u8) -> bool
```

Any printable character except hyphen-minus, as defined in the
'labelchar' production in the RFC 7468 ABNF grammar

### `is_allowed_in_label`

```rust
fn is_allowed_in_label(char: u8) -> bool
```

Does the provided byte match a character allowed in a label?

### `is_wsp`

```rust
fn is_wsp(char: u8) -> bool
```

Does the provided byte match the "WSP" ABNF production from Section 3?

> The common ABNF production WSP is congruent with "blank";
> a new production W is used for "whitespace"

### `strip_preamble`

```rust
fn strip_preamble(bytes: &[u8]) -> crate::Result<&[u8]>
```

Strip the "preamble", i.e. data that appears before the PEM
pre-encapsulation boundary.

Presently no attempt is made to ensure the preamble decodes successfully
under any particular character encoding. The only byte which is disallowed
is the NUL byte. This restriction does not appear in RFC7468, but rather
is inspired by the OpenSSL PEM decoder.

Returns a slice which starts at the beginning of the encapsulated text.

From RFC7468:
> Data before the encapsulation boundaries are permitted, and
> parsers MUST NOT malfunction when processing such data.

### `strip_leading_eol`

```rust
fn strip_leading_eol(bytes: &[u8]) -> Option<&[u8]>
```

Strip a newline (`eol`) from the beginning of the provided byte slice.

The newline is considered mandatory and a decoding error will occur if it
is not present.

From RFC 7468 Section 3:
> lines are divided with CRLF, CR, or LF.

### `strip_trailing_eol`

```rust
fn strip_trailing_eol(bytes: &[u8]) -> Option<&[u8]>
```

Strip a newline (`eol`) from the end of the provided byte slice.

The newline is considered mandatory and a decoding error will occur if it
is not present.

From RFC 7468 Section 3:
> lines are divided with CRLF, CR, or LF.

### `split_label`

```rust
fn split_label(bytes: &[u8]) -> Option<(&str, &[u8])>
```

Split a slice beginning with a type label as located in an encapsulation
boundary. Returns the label as a `&str`, and slice beginning with the
encapsulated text with leading `-----` and newline removed.

This implementation follows the rules put forth in Section 2, which are
stricter than those found in the ABNF grammar:

> Labels are formally case-sensitive, uppercase, and comprised of zero or more
> characters; they do not contain consecutive spaces or hyphen-minuses,
> nor do they contain spaces or hyphen-minuses at either end.

We apply a slightly stricter interpretation:
- Labels MAY be empty
- Non-empty labels MUST start with an upper-case letter: `'A'..='Z'`
- The only allowable characters subsequently are `'A'..='Z'` or WSP.
  (NOTE: this is an overly strict initial implementation and should be relaxed)
- Whitespace MUST NOT contain more than one consecutive WSP character

### `validate_label`

```rust
fn validate_label(label: &[u8]) -> crate::Result<()>
```

Validate that the given bytes are allowed as a PEM type label, i.e. the
label encoded in the `BEGIN` and `END` encapsulation boundaries.

## Constants

### `CHAR_NUL`
```rust
const CHAR_NUL: u8 = 0u8;
```

NUL char

### `CHAR_HT`
```rust
const CHAR_HT: u8 = 9u8;
```

Horizontal tab

### `CHAR_SP`
```rust
const CHAR_SP: u8 = 32u8;
```

Space

### `CHAR_CR`
```rust
const CHAR_CR: u8 = 13u8;
```

Carriage return

### `CHAR_LF`
```rust
const CHAR_LF: u8 = 10u8;
```

Line feed

### `CHAR_COLON`
```rust
const CHAR_COLON: u8 = 58u8;
```

Colon ':'

