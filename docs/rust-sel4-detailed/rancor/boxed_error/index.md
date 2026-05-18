*[rancor](../index.md) / [boxed_error](index.md)*

---

# Module `boxed_error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ErrorWithTrace`](#errorwithtrace) | struct |  |
| [`BoxedError`](#boxederror) | struct | An error type that preserves all detailed error messages. |
| [`ErrorTrace`](#errortrace) | trait |  |

## Structs

### `ErrorWithTrace`

```rust
struct ErrorWithTrace {
    error: BoxedError,
    trace: crate::thin_box::ThinBox<dyn ErrorTrace>,
}
```

#### Trait Implementations

##### `impl Debug for ErrorWithTrace`

- <span id="errorwithtrace-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ErrorWithTrace`

- <span id="errorwithtrace-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ErrorWithTrace`

- <span id="errorwithtrace-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl Pointee for ErrorWithTrace`

- <span id="errorwithtrace-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for ErrorWithTrace`

- <span id="errorwithtrace-tostring-to-string"></span>`fn to_string(&self) -> String`

### `BoxedError`

```rust
struct BoxedError {
    inner: crate::thin_box::ThinBox<dyn error::Error + Send + Sync>,
}
```

An error type that preserves all detailed error messages. It is optimized
to fit in a single pointer.

#### Trait Implementations

##### `impl Debug for BoxedError`

- <span id="boxederror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BoxedError`

- <span id="boxederror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for BoxedError`

- <span id="boxederror-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl Pointee for BoxedError`

- <span id="boxederror-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Source for BoxedError`

- <span id="boxederror-source-new"></span>`fn new<T: error::Error + Send + Sync + 'static>(source: T) -> Self`

##### `impl ToString for BoxedError`

- <span id="boxederror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl Trace for BoxedError`

- <span id="boxederror-trace"></span>`fn trace<R>(self, trace: R) -> Self`

## Traits

### `ErrorTrace`

```rust
trait ErrorTrace: fmt::Debug + fmt::Display + Send + Sync + 'static { ... }
```

#### Implementors

- `T`

