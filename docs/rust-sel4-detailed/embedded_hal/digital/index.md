*[embedded_hal](../index.md) / [digital](index.md)*

---

# Module `digital`

Digital I/O.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ErrorKind`](#errorkind) | enum | Error kind. |
| [`PinState`](#pinstate) | enum | Digital output pin state. |
| [`Error`](#error) | trait | Error. |
| [`ErrorType`](#errortype) | trait | Error type trait. |
| [`OutputPin`](#outputpin) | trait | Single digital push-pull output pin. |
| [`StatefulOutputPin`](#statefuloutputpin) | trait | Push-pull output pin that can read its output state. |
| [`InputPin`](#inputpin) | trait | Single digital input pin. |

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

### `PinState`

```rust
enum PinState {
    Low,
    High,
}
```

Digital output pin state.

Conversion from `bool` and logical negation are also implemented
for this type.
```rust
use embedded_hal::digital::PinState;
let state = PinState::from(false);
assert_eq!(state, PinState::Low);
assert_eq!(!state, PinState::High);
```

#### Variants

- **`Low`**

  Low pin state.

- **`High`**

  High pin state.

#### Trait Implementations

##### `impl Clone for PinState`

- <span id="pinstate-clone"></span>`fn clone(&self) -> PinState` — [`PinState`](#pinstate)

##### `impl Copy for PinState`

##### `impl Debug for PinState`

- <span id="pinstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PinState`

##### `impl Not for PinState`

- <span id="pinstate-not-type-output"></span>`type Output = PinState`

- <span id="pinstate-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl PartialEq for PinState`

- <span id="pinstate-partialeq-eq"></span>`fn eq(&self, other: &PinState) -> bool` — [`PinState`](#pinstate)

##### `impl StructuralPartialEq for PinState`

## Traits

### `Error`

```rust
trait Error: core::fmt::Debug { ... }
```

Error.

#### Required Methods

- `fn kind(&self) -> ErrorKind`

  Convert error to a generic error kind

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

- `&T`
- `&mut T`

### `OutputPin`

```rust
trait OutputPin: ErrorType { ... }
```

Single digital push-pull output pin.

#### Required Methods

- `fn set_low(&mut self) -> Result<(), <Self as >::Error>`

  Drives the pin low.

- `fn set_high(&mut self) -> Result<(), <Self as >::Error>`

  Drives the pin high.

#### Provided Methods

- `fn set_state(&mut self, state: PinState) -> Result<(), <Self as >::Error>`

  Drives the pin high or low depending on the provided value.

#### Implementors

- `&mut T`

### `StatefulOutputPin`

```rust
trait StatefulOutputPin: OutputPin { ... }
```

Push-pull output pin that can read its output state.

#### Required Methods

- `fn is_set_high(&mut self) -> Result<bool, <Self as >::Error>`

  Is the pin in drive high mode?

- `fn is_set_low(&mut self) -> Result<bool, <Self as >::Error>`

  Is the pin in drive low mode?

#### Provided Methods

- `fn toggle(&mut self) -> Result<(), <Self as >::Error>`

  Toggle pin output.

#### Implementors

- `&mut T`

### `InputPin`

```rust
trait InputPin: ErrorType { ... }
```

Single digital input pin.

#### Required Methods

- `fn is_high(&mut self) -> Result<bool, <Self as >::Error>`

  Is the input pin high?

- `fn is_low(&mut self) -> Result<bool, <Self as >::Error>`

  Is the input pin low?

#### Implementors

- `&mut T`

