*[futures](../index.md) / [prelude](index.md)*

---

# Module `prelude`

A "prelude" for crates using the `futures` crate.

This prelude is similar to the standard library's prelude in that you'll
almost always want to import its entire contents, but unlike the
standard library's prelude you'll have to do so manually:

```rust
#[allow(unused_imports)]
use futures::prelude::*;
```

The prelude may grow over time as additional items see ubiquitous use.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`future`](#future) | mod |  |
| [`Sink`](#sink) | struct |  |
| [`Future`](#future) | fn |  |
| [`_`](#) | fn |  |
| [`_`](#) | fn |  |

## Modules

- [`future`](future/index.md)

## Structs

### `Sink<'a>`

```rust
struct Sink<'a> {
    pub file: Option<&'a str>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}
```

*Re-exported from `addr2line`*

A source location.

#### Fields

- **`file`**: `Option<&'a str>`

  The file name.

- **`line`**: `Option<u32>`

  The line number.

- **`column`**: `Option<u32>`

  The column number.
  
  A value of `Some(0)` indicates the left edge.

## Functions

### `Future`

```rust
fn Future(bytes: &[u8]) -> Self
```

### `_`

```rust
fn _(&self) -> &T
```

### `_`

```rust
fn _(&mut self) -> &mut T
```

