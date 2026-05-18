*[num_integer](../index.md) / [roots](index.md)*

---

# Module `roots`

## Contents

- [Traits](#traits)
  - [`Roots`](#roots)
- [Functions](#functions)
  - [`sqrt`](#sqrt)
  - [`cbrt`](#cbrt)
  - [`nth_root`](#nth-root)
  - [`fixpoint`](#fixpoint)
  - [`bits`](#bits)
  - [`log2`](#log2)
- [Macros](#macros)
  - [`signed_roots!`](#signed-roots)
  - [`unsigned_roots!`](#unsigned-roots)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Roots`](#roots) | trait | Provides methods to compute an integer's square root, cube root, and arbitrary `n`th root. |
| [`sqrt`](#sqrt) | fn | Returns the truncated principal square root of an integer -- see [Roots::sqrt](trait.Roots.html#method.sqrt). |
| [`cbrt`](#cbrt) | fn | Returns the truncated principal cube root of an integer -- see [Roots::cbrt](trait.Roots.html#method.cbrt). |
| [`nth_root`](#nth-root) | fn | Returns the truncated principal `n`th root of an integer -- see [Roots::nth_root](trait.Roots.html#tymethod.nth_root). |
| [`fixpoint`](#fixpoint) | fn |  |
| [`bits`](#bits) | fn |  |
| [`log2`](#log2) | fn |  |
| [`signed_roots!`](#signed-roots) | macro |  |
| [`unsigned_roots!`](#unsigned-roots) | macro |  |

## Traits

### `Roots`

```rust
trait Roots: Integer { ... }
```

Provides methods to compute an integer's square root, cube root,
and arbitrary `n`th root.

#### Required Methods

- `fn nth_root(&self, n: u32) -> Self`

  Returns the truncated principal `n`th root of an integer

#### Provided Methods

- `fn sqrt(&self) -> Self`

  Returns the truncated principal square root of an integer -- `⌊√x⌋`

- `fn cbrt(&self) -> Self`

  Returns the truncated principal cube root of an integer --

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Functions

### `sqrt`

```rust
fn sqrt<T: Roots>(x: T) -> T
```

Returns the truncated principal square root of an integer --
see [Roots::sqrt](#roots-sqrt).

### `cbrt`

```rust
fn cbrt<T: Roots>(x: T) -> T
```

Returns the truncated principal cube root of an integer --
see [Roots::cbrt](#roots-cbrt).

### `nth_root`

```rust
fn nth_root<T: Roots>(x: T, n: u32) -> T
```

Returns the truncated principal `n`th root of an integer --
see [Roots::nth_root](#roots-nth-root).

### `fixpoint`

```rust
fn fixpoint<T, F>(x: T, f: F) -> T
where
    T: Integer + Copy,
    F: Fn(T) -> T
```

### `bits`

```rust
fn bits<T>() -> u32
```

### `log2`

```rust
fn log2<T: PrimInt>(x: T) -> u32
```

## Macros

### `signed_roots!`

### `unsigned_roots!`

