**postcard > fixint**

# Module: fixint

## Contents

**Modules**

- [`be`](#be) - Disables varint serialization/deserialization for the specified integer field.
- [`le`](#le) - Use with the `#[serde(with = "postcard::fixint::le")]` field attribute.

---

## Module: be

Disables varint serialization/deserialization for the specified integer field.

Use with the `#[serde(with = "postcard::fixint::be")]` field attribute.
The integer will always be serialized in the same way as a fixed
size array, in **Big Endian** order on the wire.

```rust
# use serde::Serialize;
#[derive(Serialize)]
pub struct DefinitelyBigEndian {
    #[serde(with = "postcard::fixint::be")]
    x: u16,
}
```



## Module: le

Use with the `#[serde(with = "postcard::fixint::le")]` field attribute.

Disables varint serialization/deserialization for the specified integer
field. The integer will always be serialized in the same way as a fixed
size array, in **Little Endian** order on the wire.

```rust
# use serde::Serialize;
#[derive(Serialize)]
pub struct DefinitelyLittleEndian {
    #[serde(with = "postcard::fixint::le")]
    x: u16,
}
```



