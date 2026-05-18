*[postcard](../../index.md) / [fixint](../index.md) / [le](index.md)*

---

# Module `le`

Use with the `#[serde(with = "postcard::fixint::le")]` field attribute.

Disables varint serialization/deserialization for the specified integer
field. The integer will always be serialized in the same way as a fixed
size array, in **Little Endian** order on the wire.

```rust
use serde::Serialize;
#[derive(Serialize)]
pub struct DefinitelyLittleEndian {
    #[serde(with = "postcard::fixint::le")]
    x: u16,
}
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`serialize`](#serialize) | fn | Serialize the integer value as a little-endian fixed-size array. |
| [`deserialize`](#deserialize) | fn | Deserialize the integer value from a little-endian fixed-size array. |

## Functions

### `serialize`

```rust
fn serialize<S, T>(val: &T, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer,
    T: Copy,
    super::LE<T>: Serialize
```

Serialize the integer value as a little-endian fixed-size array.

### `deserialize`

```rust
fn deserialize<'de, D, T>(deserializer: D) -> Result<T, <D as >::Error>
where
    D: Deserializer<'de>,
    super::LE<T>: Deserialize<'de>
```

Deserialize the integer value from a little-endian fixed-size array.

