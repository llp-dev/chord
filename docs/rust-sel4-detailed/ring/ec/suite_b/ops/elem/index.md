*[ring](../../../../index.md) / [ec](../../../index.md) / [suite_b](../../index.md) / [ops](../index.md) / [elem](index.md)*

---

# Module `elem`

## Contents

- [Structs](#structs)
  - [`Elem`](#elem)
- [Functions](#functions)
  - [`mul_mont`](#mul-mont)
  - [`binary_op`](#binary-op)
  - [`binary_op_assign`](#binary-op-assign)
  - [`unary_op`](#unary-op)
  - [`unary_op_assign`](#unary-op-assign)
  - [`unary_op_from_binary_op_assign`](#unary-op-from-binary-op-assign)
- [Constants](#constants)
  - [`MAX_LIMBS`](#max-limbs)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Elem`](#elem) | struct | Elements of ℤ/mℤ for some modulus *m*. |
| [`mul_mont`](#mul-mont) | fn |  |
| [`binary_op`](#binary-op) | fn |  |
| [`binary_op_assign`](#binary-op-assign) | fn |  |
| [`unary_op`](#unary-op) | fn |  |
| [`unary_op_assign`](#unary-op-assign) | fn |  |
| [`unary_op_from_binary_op_assign`](#unary-op-from-binary-op-assign) | fn |  |
| [`MAX_LIMBS`](#max-limbs) | const |  |

## Structs

### `Elem<M, E: Encoding>`

```rust
struct Elem<M, E: Encoding> {
    limbs: [u64; 6],
    m: core::marker::PhantomData<M>,
    encoding: core::marker::PhantomData<E>,
}
```

Elements of ℤ/mℤ for some modulus *m*. Elements are always fully reduced
with respect to *m*; i.e. the 0 <= x < m for every value x.

#### Fields

- **`m`**: `core::marker::PhantomData<M>`

  The modulus *m* for the ring ℤ/mℤ for which this element is a value.

- **`encoding`**: `core::marker::PhantomData<E>`

  The number of Montgomery factors that need to be canceled out from
  `value` to get the actual value.

#### Implementations

- <span id="elem-zero"></span>`fn zero() -> Self`

- <span id="elem-from-hex"></span>`const fn from_hex(hex: &str) -> Self`

#### Trait Implementations

##### `impl<M: clone::Clone, E: clone::Clone + Encoding> Clone for Elem<M, E>`

- <span id="elem-clone"></span>`fn clone(&self) -> Elem<M, E>` — [`Elem`](../index.md#elem)

##### `impl<M: marker::Copy, E: marker::Copy + Encoding> Copy for Elem<M, E>`

## Functions

### `mul_mont`

```rust
fn mul_mont<M, EA: Encoding, EB: Encoding>(f: fn(*mut u64, *const u64, *const u64), a: &Elem<M, EA>, b: &Elem<M, EB>) -> Elem<M, <(EA, EB) as ProductEncoding>::Output>
where
    (EA, EB): ProductEncoding
```

### `binary_op`

```rust
fn binary_op<M, EA: Encoding, EB: Encoding, ER: Encoding>(f: fn(*mut u64, *const u64, *const u64), a: &Elem<M, EA>, b: &Elem<M, EB>) -> Elem<M, ER>
```

### `binary_op_assign`

```rust
fn binary_op_assign<M, EA: Encoding, EB: Encoding>(f: fn(*mut u64, *const u64, *const u64), a: &mut Elem<M, EA>, b: &Elem<M, EB>)
```

### `unary_op`

```rust
fn unary_op<M, E: Encoding>(f: fn(*mut u64, *const u64), a: &Elem<M, E>) -> Elem<M, E>
```

### `unary_op_assign`

```rust
fn unary_op_assign<M, E: Encoding>(f: fn(*mut u64, *const u64), a: &mut Elem<M, E>)
```

### `unary_op_from_binary_op_assign`

```rust
fn unary_op_from_binary_op_assign<M, E: Encoding>(f: fn(*mut u64, *const u64, *const u64), a: &mut Elem<M, E>)
```

## Constants

### `MAX_LIMBS`
```rust
const MAX_LIMBS: usize = 6usize;
```

