# Crate `sel4_ctors_dtors`

## Contents

- [Modules](#modules)
  - [`_weak`](#weak)
- [Enums](#enums)
  - [`Error`](#error)
- [Functions](#functions)
  - [`_init`](#init)
  - [`_fini`](#fini)
  - [`run_array`](#run-array)
  - [`run_preinit_array`](#run-preinit-array)
  - [`run_init_array`](#run-init-array)
  - [`run_fini_array`](#run-fini-array)
  - [`run_init`](#run-init)
  - [`run_fini`](#run-fini)
  - [`run_ctors`](#run-ctors)
  - [`run_dtors`](#run-dtors)
- [Type Aliases](#type-aliases)
  - [`ArrayEntry`](#arrayentry)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`_weak`](#weak) | mod |  |
| [`Error`](#error) | enum |  |
| [`_init`](#init) | fn |  |
| [`_fini`](#fini) | fn |  |
| [`run_array`](#run-array) | fn |  |
| [`run_preinit_array`](#run-preinit-array) | fn |  |
| [`run_init_array`](#run-init-array) | fn |  |
| [`run_fini_array`](#run-fini-array) | fn |  |
| [`run_init`](#run-init) | fn |  |
| [`run_fini`](#run-fini) | fn |  |
| [`run_ctors`](#run-ctors) | fn |  |
| [`run_dtors`](#run-dtors) | fn |  |
| [`ArrayEntry`](#arrayentry) | type |  |

## Modules

- [`_weak`](_weak/index.md)

## Enums

### `Error`

```rust
enum Error {
    Misaligned {
        section_name: &'static str,
    },
}
```

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

## Functions

### `_init`

```rust
unsafe fn _init()
```

### `_fini`

```rust
unsafe fn _fini()
```

### `run_array`

```rust
unsafe fn run_array(start_addr: usize, end_addr: usize, section_name: &'static str) -> Result<(), Error>
```

### `run_preinit_array`

```rust
fn run_preinit_array() -> Result<(), Error>
```

### `run_init_array`

```rust
fn run_init_array() -> Result<(), Error>
```

### `run_fini_array`

```rust
fn run_fini_array() -> Result<(), Error>
```

### `run_init`

```rust
fn run_init()
```

### `run_fini`

```rust
fn run_fini()
```

### `run_ctors`

```rust
fn run_ctors() -> Result<(), Error>
```

### `run_dtors`

```rust
fn run_dtors() -> Result<(), Error>
```

## Type Aliases

### `ArrayEntry`

```rust
type ArrayEntry = fn();
```

