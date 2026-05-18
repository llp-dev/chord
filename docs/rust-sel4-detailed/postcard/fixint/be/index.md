*[postcard](../../index.md) / [fixint](../index.md) / [be](index.md)*

---

# Module `be`

Disables varint serialization/deserialization for the specified integer field.

Use with the `#[serde(with = "postcard::fixint::be")]` field attribute.
The integer will always be serialized in the same way as a fixed
size array, in **Big Endian** order on the wire.

```rust
use serde::Serialize;
#[derive(Serialize)]
pub struct DefinitelyBigEndian {
    #[serde(with = "postcard::fixint::be")]
    x: u16,
}
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`serialize`](#serialize) | fn | Serialize the integer value as a big-endian fixed-size array. |
| [`deserialize`](#deserialize) | fn | Deserialize the integer value from a big-endian fixed-size array. |

## Functions

### `serialize`

```rust
fn serialize<S, T>(val: &T, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer,
    T: Copy,
    super::BE<T>: Serialize
```

Serialize the integer value as a big-endian fixed-size array.

### `deserialize`

```rust
fn deserialize<'de, D, T>(deserializer: D) -> Result<T, <D as >::Error>
where
    D: Deserializer<'de>,
    super::BE<T>: Deserialize<'de>
```

Deserialize the integer value from a big-endian fixed-size array.

