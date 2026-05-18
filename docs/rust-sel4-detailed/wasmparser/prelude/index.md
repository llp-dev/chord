*[wasmparser](../index.md) / [prelude](index.md)*

---

# Module `prelude`

A small "prelude" to use throughout this crate.

This crate is tagged with `#![no_std]` meaning that we get libcore's prelude
by default. This crate also uses `alloc`, however, and common types there
like `String`. This custom prelude helps bring those types into scope to
avoid having to import each of them manually.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ToOwned`](#toowned) | mod |  |
| [`ToString`](#tostring) | struct |  |
| [`Box`](#box) | fn |  |

## Modules

- [`ToOwned`](ToOwned/index.md)

## Structs

### `ToString<'a>`

```rust
struct ToString<'a> {
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

### `Box`

```rust
fn Box(bytes: &[u8]) -> Self
```

