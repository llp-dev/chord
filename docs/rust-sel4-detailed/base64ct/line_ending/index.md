*[base64ct](../index.md) / [line_ending](index.md)*

---

# Module `line_ending`

Line endings.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LineEnding`](#lineending) | enum | Line endings: variants of newline characters that can be used with Base64. |
| [`CHAR_CR`](#char-cr) | const | Carriage return |
| [`CHAR_LF`](#char-lf) | const | Line feed |

## Enums

### `LineEnding`

```rust
enum LineEnding {
    CR,
    LF,
    CRLF,
}
```

Line endings: variants of newline characters that can be used with Base64.

Use `LineEnding::default` to get an appropriate line ending for the
current operating system.

#### Variants

- **`CR`**

  Carriage return: `\r` (Pre-OS X Macintosh)

- **`LF`**

  Line feed: `\n` (Unix OSes)

- **`CRLF`**

  Carriage return + line feed: `\r\n` (Windows)

#### Implementations

- <span id="lineending-as-bytes"></span>`fn as_bytes(self) -> &'static [u8]`

  Get the byte serialization of this [`LineEnding`](#lineending).

- <span id="lineending-len"></span>`fn len(self) -> usize`

  Get the encoded length of this [`LineEnding`](#lineending).

#### Trait Implementations

##### `impl Clone for LineEnding`

- <span id="lineending-clone"></span>`fn clone(&self) -> LineEnding` — [`LineEnding`](#lineending)

##### `impl Copy for LineEnding`

##### `impl Debug for LineEnding`

- <span id="lineending-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LineEnding`

- <span id="lineending-default"></span>`fn default() -> LineEnding` — [`LineEnding`](#lineending)

##### `impl Eq for LineEnding`

##### `impl Ord for LineEnding`

- <span id="lineending-ord-cmp"></span>`fn cmp(&self, other: &LineEnding) -> cmp::Ordering` — [`LineEnding`](#lineending)

##### `impl PartialEq for LineEnding`

- <span id="lineending-partialeq-eq"></span>`fn eq(&self, other: &LineEnding) -> bool` — [`LineEnding`](#lineending)

##### `impl PartialOrd for LineEnding`

- <span id="lineending-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &LineEnding) -> option::Option<cmp::Ordering>` — [`LineEnding`](#lineending)

##### `impl StructuralPartialEq for LineEnding`

## Constants

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

