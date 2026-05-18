*[generic_array](../index.md) / [functional](index.md)*

---

# Module `functional`

Functional programming with generic sequences

Please see `tests/generics.rs` for examples of how to best use these in your generic functions.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MappedGenericSequence`](#mappedgenericsequence) | trait | Defines the relationship between one generic sequence and another, for operations such as `map` and `zip`. |
| [`FunctionalSequence`](#functionalsequence) | trait | Defines functional programming methods for generic sequences |
| [`MappedSequence`](#mappedsequence) | type | Accessor type for a mapped generic sequence |

## Traits

### `MappedGenericSequence<T, U>`

```rust
trait MappedGenericSequence<T, U>: GenericSequence<T>
where
    <Self as >::Length: ArrayLength<U> { ... }
```

Defines the relationship between one generic sequence and another,
for operations such as `map` and `zip`.

#### Associated Types

- `type Mapped: 1`

#### Implementors

- [`GenericArray`](../index.md#genericarray)
- `&'a S`
- `&'a mut S`

### `FunctionalSequence<T>`

```rust
trait FunctionalSequence<T>: GenericSequence<T> { ... }
```

Defines functional programming methods for generic sequences

#### Provided Methods

- `fn map<U, F>(self, f: F) -> MappedSequence<Self, T, U>`

  Maps a `GenericSequence` to another `GenericSequence`.

- `fn zip<B, Rhs, U, F>(self, rhs: Rhs, f: F) -> MappedSequence<Self, T, U>`

  Combines two `GenericSequence` instances and iterates through both of them,

- `fn fold<U, F>(self, init: U, f: F) -> U`

  Folds (or reduces) a sequence of data into a single value.

#### Implementors

- [`GenericArray`](../index.md#genericarray)
- `&'a S`
- `&'a mut S`

## Type Aliases

### `MappedSequence<S, T, U>`

```rust
type MappedSequence<S, T, U> = <<S as MappedGenericSequence>::Mapped as GenericSequence>::Sequence;
```

Accessor type for a mapped generic sequence

