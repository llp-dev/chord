*[const_oid](../index.md) / [parser](index.md)*

---

# Module `parser`

OID string parser with `const` support.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parser`](#parser) | struct | Const-friendly OID string parser. |

## Structs

### `Parser`

```rust
struct Parser {
    current_arc: crate::Arc,
    encoder: crate::encoder::Encoder,
}
```

Const-friendly OID string parser.

Parses an OID from the dotted string representation.

#### Fields

- **`current_arc`**: `crate::Arc`

  Current arc in progress

- **`encoder`**: `crate::encoder::Encoder`

  BER/DER encoder

#### Implementations

- <span id="parser-parse"></span>`const fn parse(s: &str) -> Result<Self>` — [`Result`](../error/index.md#result)

  Parse an OID from a dot-delimited string e.g. `1.2.840.113549.1.1.1`

- <span id="parser-finish"></span>`const fn finish(self) -> Result<ObjectIdentifier>` — [`Result`](../error/index.md#result), [`ObjectIdentifier`](../index.md#objectidentifier)

  Finish parsing, returning the result

- <span id="parser-parse-bytes"></span>`const fn parse_bytes(self, bytes: &[u8]) -> Result<Self>` — [`Result`](../error/index.md#result)

  Parse the remaining bytes

#### Trait Implementations

##### `impl Debug for Parser`

- <span id="parser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

