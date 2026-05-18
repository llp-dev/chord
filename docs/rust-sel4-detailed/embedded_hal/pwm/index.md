*[embedded_hal](../index.md) / [pwm](index.md)*

---

# Module `pwm`

Pulse Width Modulation (PWM) traits.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ErrorKind`](#errorkind) | enum | Error kind. |
| [`Error`](#error) | trait | Error |
| [`ErrorType`](#errortype) | trait | Error type trait. |
| [`SetDutyCycle`](#setdutycycle) | trait | Single PWM channel / pin. |

## Enums

### `ErrorKind`

```rust
enum ErrorKind {
    Other,
}
```

Error kind.

This represents a common set of operation errors. HAL implementations are
free to define more specific or additional error types. However, by providing
a mapping to these common errors, generic code can still react to them.

#### Variants

- **`Other`**

  A different error occurred. The original error may contain more information.

#### Trait Implementations

##### `impl Clone for ErrorKind`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl Copy for ErrorKind`

##### `impl Debug for ErrorKind`

- <span id="errorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ErrorKind`

- <span id="errorkind-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for ErrorKind`

##### `impl Error for ErrorKind`

- <span id="errorkind-error-kind"></span>`fn kind(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl Hash for ErrorKind`

- <span id="errorkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for ErrorKind`

- <span id="errorkind-ord-cmp"></span>`fn cmp(&self, other: &ErrorKind) -> cmp::Ordering` — [`ErrorKind`](#errorkind)

##### `impl PartialEq for ErrorKind`

- <span id="errorkind-partialeq-eq"></span>`fn eq(&self, other: &ErrorKind) -> bool` — [`ErrorKind`](#errorkind)

##### `impl PartialOrd for ErrorKind`

- <span id="errorkind-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ErrorKind) -> option::Option<cmp::Ordering>` — [`ErrorKind`](#errorkind)

##### `impl StructuralPartialEq for ErrorKind`

## Traits

### `Error`

```rust
trait Error: core::fmt::Debug { ... }
```

Error

#### Required Methods

- `fn kind(&self) -> ErrorKind`

  Convert error to a generic error kind.

#### Implementors

- [`ErrorKind`](#errorkind)
- `core::convert::Infallible`

### `ErrorType`

```rust
trait ErrorType { ... }
```

Error type trait.

This just defines the error type, to be used by the other traits.

#### Associated Types

- `type Error: 1`

#### Implementors

- `&mut T`

### `SetDutyCycle`

```rust
trait SetDutyCycle: ErrorType { ... }
```

Single PWM channel / pin.

#### Required Methods

- `fn max_duty_cycle(&self) -> u16`

  Get the maximum duty cycle value.

- `fn set_duty_cycle(&mut self, duty: u16) -> Result<(), <Self as >::Error>`

  Set the duty cycle to `duty / max_duty`.

#### Provided Methods

- `fn set_duty_cycle_fully_off(&mut self) -> Result<(), <Self as >::Error>`

  Set the duty cycle to 0%, or always inactive.

- `fn set_duty_cycle_fully_on(&mut self) -> Result<(), <Self as >::Error>`

  Set the duty cycle to 100%, or always active.

- `fn set_duty_cycle_fraction(&mut self, num: u16, denom: u16) -> Result<(), <Self as >::Error>`

  Set the duty cycle to `num / denom`.

- `fn set_duty_cycle_percent(&mut self, percent: u8) -> Result<(), <Self as >::Error>`

  Set the duty cycle to `percent / 100`

#### Implementors

- `&mut T`

