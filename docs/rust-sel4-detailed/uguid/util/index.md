*[uguid](../index.md) / [util](index.md)*

---

# Module `util`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`byte_to_ascii_hex_lower`](#byte-to-ascii-hex-lower) | fn |  |
| [`parse_byte_from_ascii_char`](#parse-byte-from-ascii-char) | fn | Parse a hexadecimal ASCII character as a `u8`. |
| [`parse_byte_from_ascii_char_pair`](#parse-byte-from-ascii-char-pair) | fn | Parse a pair of hexadecimal ASCII characters as a `u8`. |
| [`parse_byte_from_ascii_str_at`](#parse-byte-from-ascii-str-at) | fn | Parse a pair of hexadecimal ASCII characters at position `start` as a `u8`. |

## Functions

### `byte_to_ascii_hex_lower`

```rust
const fn byte_to_ascii_hex_lower(byte: u8) -> (u8, u8)
```

### `parse_byte_from_ascii_char`

```rust
const fn parse_byte_from_ascii_char(c: u8) -> Option<u8>
```

Parse a hexadecimal ASCII character as a `u8`.

### `parse_byte_from_ascii_char_pair`

```rust
const fn parse_byte_from_ascii_char_pair(a: u8, b: u8) -> Option<u8>
```

Parse a pair of hexadecimal ASCII characters as a `u8`. For example,
`(b'1', b'a')` is parsed as `0x1a`.

### `parse_byte_from_ascii_str_at`

```rust
const fn parse_byte_from_ascii_str_at(s: &[u8], start: u8) -> Result<u8, crate::GuidFromStrError>
```

Parse a pair of hexadecimal ASCII characters at position `start` as
a `u8`.

