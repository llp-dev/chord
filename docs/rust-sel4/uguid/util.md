**uguid > util**

# Module: util

## Contents

**Functions**

- [`byte_to_ascii_hex_lower`](#byte_to_ascii_hex_lower)
- [`parse_byte_from_ascii_char`](#parse_byte_from_ascii_char) - Parse a hexadecimal ASCII character as a `u8`.
- [`parse_byte_from_ascii_char_pair`](#parse_byte_from_ascii_char_pair) - Parse a pair of hexadecimal ASCII characters as a `u8`. For example,
- [`parse_byte_from_ascii_str_at`](#parse_byte_from_ascii_str_at) - Parse a pair of hexadecimal ASCII characters at position `start` as

---

## uguid::util::byte_to_ascii_hex_lower

*Function*

```rust
fn byte_to_ascii_hex_lower(byte: u8) -> (u8, u8)
```



## uguid::util::parse_byte_from_ascii_char

*Function*

Parse a hexadecimal ASCII character as a `u8`.

```rust
fn parse_byte_from_ascii_char(c: u8) -> Option<u8>
```



## uguid::util::parse_byte_from_ascii_char_pair

*Function*

Parse a pair of hexadecimal ASCII characters as a `u8`. For example,
`(b'1', b'a')` is parsed as `0x1a`.

```rust
fn parse_byte_from_ascii_char_pair(a: u8, b: u8) -> Option<u8>
```



## uguid::util::parse_byte_from_ascii_str_at

*Function*

Parse a pair of hexadecimal ASCII characters at position `start` as
a `u8`.

```rust
fn parse_byte_from_ascii_str_at(s: &[u8], start: u8) -> Result<u8, crate::GuidFromStrError>
```



