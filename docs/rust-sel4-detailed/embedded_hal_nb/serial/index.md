*[embedded_hal_nb](../index.md) / [serial](index.md)*

---

# Module `serial`

Serial interface.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ErrorKind`](#errorkind) | enum | Serial error kind. |
| [`Error`](#error) | trait | Serial error. |
| [`ErrorType`](#errortype) | trait | Serial error type trait. |
| [`Read`](#read) | trait | Read half of a serial interface. |
| [`Write`](#write) | trait | Write half of a serial interface. |

## Enums

### `ErrorKind`

```rust
enum ErrorKind {
    Overrun,
    FrameFormat,
    Parity,
    Noise,
    Other,
}
```

Serial error kind.

This represents a common set of serial operation errors. HAL implementations are
free to define more specific or additional error types. However, by providing
a mapping to these common serial errors, generic code can still react to them.

#### Variants

- **`Overrun`**

  The peripheral receive buffer was overrun.

- **`FrameFormat`**

  Received data does not conform to the peripheral configuration.
  Can be caused by a misconfigured device on either end of the serial line.

- **`Parity`**

  Parity check failed.

- **`Noise`**

  Serial line is too noisy to read valid data.

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

Serial error.

#### Required Methods

- `fn kind(&self) -> ErrorKind`

  Convert error to a generic serial error kind

#### Implementors

- [`ErrorKind`](#errorkind)
- `core::convert::Infallible`

### `ErrorType`

```rust
trait ErrorType { ... }
```

Serial error type trait.

This just defines the error type, to be used by the other traits.

#### Associated Types

- `type Error: 1`

#### Implementors

- `&mut T`

### `Read<Word: Copy>`

```rust
trait Read<Word: Copy>: ErrorType { ... }
```

Read half of a serial interface.

Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
This can be encoded in this trait via the `Word` type parameter.

#### Required Methods

- `fn read(&mut self) -> nb::Result<Word, <Self as >::Error>`

  Reads a single word from the serial interface

#### Implementors

- `&mut T`

### `Write<Word: Copy>`

```rust
trait Write<Word: Copy>: ErrorType { ... }
```

Write half of a serial interface.

#### Required Methods

- `fn write(&mut self, word: Word) -> nb::Result<(), <Self as >::Error>`

  Writes a single word to the serial interface.

- `fn flush(&mut self) -> nb::Result<(), <Self as >::Error>`

  Ensures that none of the previously written words are still buffered.

#### Implementors

- `&mut T`

