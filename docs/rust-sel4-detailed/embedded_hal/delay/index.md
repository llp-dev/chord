*[embedded_hal](../index.md) / [delay](index.md)*

---

# Module `delay`

Delays.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DelayNs`](#delayns) | trait | Delay with up to nanosecond precision. |
| [`NANOS_PER_MICRO`](#nanos-per-micro) | const | Nanoseconds per microsecond |
| [`NANOS_PER_MILLI`](#nanos-per-milli) | const | Nanoseconds per millisecond |

## Traits

### `DelayNs`

```rust
trait DelayNs { ... }
```

Delay with up to nanosecond precision.

#### Required Methods

- `fn delay_ns(&mut self, ns: u32)`

  Pauses execution for at minimum `ns` nanoseconds. Pause can be longer

#### Provided Methods

- `fn delay_us(&mut self, us: u32)`

  Pauses execution for at minimum `us` microseconds. Pause can be longer

- `fn delay_ms(&mut self, ms: u32)`

  Pauses execution for at minimum `ms` milliseconds. Pause can be longer

#### Implementors

- `&mut T`

## Constants

### `NANOS_PER_MICRO`
```rust
const NANOS_PER_MICRO: u32 = 1_000u32;
```

Nanoseconds per microsecond

### `NANOS_PER_MILLI`
```rust
const NANOS_PER_MILLI: u32 = 1_000_000u32;
```

Nanoseconds per millisecond

