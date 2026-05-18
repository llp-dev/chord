**postcard > fixint > be**

# Module: fixint::be

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize the integer value from a big-endian fixed-size array.
- [`serialize`](#serialize) - Serialize the integer value as a big-endian fixed-size array.

---

## postcard::fixint::be::deserialize

*Function*

Deserialize the integer value from a big-endian fixed-size array.

```rust
fn deserialize<'de, D, T>(deserializer: D) -> Result<T, <D as >::Error>
```



## postcard::fixint::be::serialize

*Function*

Serialize the integer value as a big-endian fixed-size array.

```rust
fn serialize<S, T>(val: &T, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



