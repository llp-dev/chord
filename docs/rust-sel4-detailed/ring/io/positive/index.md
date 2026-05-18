*[ring](../../index.md) / [io](../index.md) / [positive](index.md)*

---

# Module `positive`

Serialization and deserialization.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Positive`](#positive) | struct | A serialized positive integer. |

## Structs

### `Positive<'a>`

```rust
struct Positive<'a>(untrusted::Input<'a>);
```

A serialized positive integer.

#### Implementations

- <span id="positive-from-be-bytes"></span>`fn from_be_bytes(input: untrusted::Input<'a>) -> Result<Self, error::Unspecified>` — [`Unspecified`](../../error/index.md#unspecified)

- <span id="positive-big-endian-without-leading-zero"></span>`fn big_endian_without_leading_zero(&self) -> &'a [u8]`

  Returns the value, ordered from significant byte to least significant

  byte, without any leading zeros. The result is guaranteed to be

  non-empty.

- <span id="positive-big-endian-without-leading-zero-as-input"></span>`fn big_endian_without_leading_zero_as_input(&self) -> untrusted::Input<'a>`

#### Trait Implementations

##### `impl Clone for Positive<'a>`

- <span id="positive-clone"></span>`fn clone(&self) -> Positive<'a>` — [`Positive`](#positive)

##### `impl Copy for Positive<'a>`

