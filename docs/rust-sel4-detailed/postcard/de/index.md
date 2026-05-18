*[postcard](../index.md) / [de](index.md)*

---

# Module `de`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deserializer`](#deserializer) | mod |  |
| [`flavors`](#flavors) | mod | # Deserialization Flavors |
| [`from_bytes`](#from-bytes) | fn | Deserialize a message of type `T` from a byte slice. |
| [`from_bytes_cobs`](#from-bytes-cobs) | fn | Deserialize a message of type `T` from a cobs-encoded byte slice. |
| [`take_from_bytes_cobs`](#take-from-bytes-cobs) | fn | Deserialize a message of type `T` from a cobs-encoded byte slice. |
| [`take_from_bytes`](#take-from-bytes) | fn | Deserialize a message of type `T` from a byte slice. |

## Modules

- [`deserializer`](deserializer/index.md)
- [`flavors`](flavors/index.md) — # Deserialization Flavors

## Functions

### `from_bytes`

```rust
fn from_bytes<'a, T>(s: &'a [u8]) -> crate::error::Result<T>
where
    T: Deserialize<'a>
```

Deserialize a message of type `T` from a byte slice. The unused portion (if any)
of the byte slice is not returned.

### `from_bytes_cobs`

```rust
fn from_bytes_cobs<'a, T>(s: &'a mut [u8]) -> crate::error::Result<T>
where
    T: Deserialize<'a>
```

Deserialize a message of type `T` from a cobs-encoded byte slice.

The unused portion (if any) of the byte slice is not returned.
The used portion of the input slice is modified during deserialization (even if an error is returned).
Therefore, if this is not desired, pass a clone of the original slice.

### `take_from_bytes_cobs`

```rust
fn take_from_bytes_cobs<'a, T>(s: &'a mut [u8]) -> crate::error::Result<(T, &'a mut [u8])>
where
    T: Deserialize<'a>
```

Deserialize a message of type `T` from a cobs-encoded byte slice.

The unused portion (if any) of the byte slice is returned for further usage.
The used portion of the input slice is modified during deserialization (even if an error is returned).
Therefore, if this is not desired, pass a clone of the original slice.

### `take_from_bytes`

```rust
fn take_from_bytes<'a, T>(s: &'a [u8]) -> crate::error::Result<(T, &'a [u8])>
where
    T: Deserialize<'a>
```

Deserialize a message of type `T` from a byte slice. The unused portion (if any)
of the byte slice is returned for further usage

