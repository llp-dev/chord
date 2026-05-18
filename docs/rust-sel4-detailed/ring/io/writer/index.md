*[ring](../../index.md) / [io](../index.md) / [writer](index.md)*

---

# Module `writer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LengthMeasurement`](#lengthmeasurement) | struct |  |
| [`Writer`](#writer) | struct |  |
| [`Accumulator`](#accumulator) | trait |  |
| [`write_copy`](#write-copy) | fn |  |

## Structs

### `LengthMeasurement`

```rust
struct LengthMeasurement {
    len: usize,
}
```

#### Implementations

- <span id="lengthmeasurement-zero"></span>`fn zero() -> Self`

#### Trait Implementations

##### `impl Accumulator for LengthMeasurement`

- <span id="lengthmeasurement-accumulator-write-byte"></span>`fn write_byte(&mut self, _value: u8)`

- <span id="lengthmeasurement-accumulator-write-bytes"></span>`fn write_bytes(&mut self, value: &[u8])`

### `Writer`

```rust
struct Writer {
    bytes: alloc::vec::Vec<u8>,
    requested_capacity: usize,
}
```

#### Implementations

- <span id="writer-with-capacity"></span>`fn with_capacity(capacity: LengthMeasurement) -> Self` — [`LengthMeasurement`](#lengthmeasurement)

#### Trait Implementations

##### `impl Accumulator for Writer`

- <span id="writer-accumulator-write-byte"></span>`fn write_byte(&mut self, value: u8)`

- <span id="writer-accumulator-write-bytes"></span>`fn write_bytes(&mut self, value: &[u8])`

## Traits

### `Accumulator`

```rust
trait Accumulator { ... }
```

#### Required Methods

- `fn write_byte(&mut self, value: u8)`

- `fn write_bytes(&mut self, value: &[u8])`

#### Implementors

- [`LengthMeasurement`](#lengthmeasurement)
- [`Writer`](#writer)

## Functions

### `write_copy`

```rust
fn write_copy(accumulator: &mut dyn Accumulator, to_copy: untrusted::Input<'_>)
```

