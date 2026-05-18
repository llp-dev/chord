*[num_traits](../index.md) / [bounds](index.md)*

---

# Module `bounds`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Bounded`](#bounded) | trait | Numbers which have upper and lower bounds |
| [`LowerBounded`](#lowerbounded) | trait | Numbers which have lower bounds |
| [`UpperBounded`](#upperbounded) | trait | Numbers which have upper bounds |
| [`bounded_impl!`](#bounded-impl) | macro |  |
| [`for_each_tuple_!`](#for-each-tuple) | macro |  |
| [`for_each_tuple!`](#for-each-tuple) | macro |  |
| [`bounded_tuple!`](#bounded-tuple) | macro |  |

## Traits

### `Bounded`

```rust
trait Bounded { ... }
```

Numbers which have upper and lower bounds

#### Required Methods

- `fn min_value() -> Self`

  Returns the smallest finite number this type can represent

- `fn max_value() -> Self`

  Returns the largest finite number this type can represent

#### Implementors

- `()`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(J, K, L, M, N, O, P, Q, R, S, T)`
- `(K, L, M, N, O, P, Q, R, S, T)`
- `(L, M, N, O, P, Q, R, S, T)`
- `(M, N, O, P, Q, R, S, T)`
- `(N, O, P, Q, R, S, T)`
- `(O, P, Q, R, S, T)`
- `(P, Q, R, S, T)`
- `(Q, R, S, T)`
- `(R, S, T)`
- `(S, T)`
- `(T)`
- `core::num::Wrapping<T>`
- `f32`
- `f64`
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

### `LowerBounded`

```rust
trait LowerBounded { ... }
```

Numbers which have lower bounds

#### Required Methods

- `fn min_value() -> Self`

  Returns the smallest finite number this type can represent

#### Implementors

- `T`

### `UpperBounded`

```rust
trait UpperBounded { ... }
```

Numbers which have upper bounds

#### Required Methods

- `fn max_value() -> Self`

  Returns the largest finite number this type can represent

#### Implementors

- `T`

## Macros

### `bounded_impl!`

### `for_each_tuple_!`

### `for_each_tuple!`

### `bounded_tuple!`

