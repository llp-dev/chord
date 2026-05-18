*[generic_array](../index.md) / [sequence](index.md)*

---

# Module `sequence`

Useful traits for manipulating sequences of data stored in `GenericArray`s

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GenericSequence`](#genericsequence) | trait | Defines some sequence with an associated length and iteration capabilities. |
| [`Lengthen`](#lengthen) | trait | Defines any `GenericSequence` which can be lengthened or extended by appending or prepending an element to it. |
| [`Shorten`](#shorten) | trait | Defines a `GenericSequence` which can be shortened by removing the first or last element from it. |
| [`Split`](#split) | trait | Defines a `GenericSequence` that can be split into two parts at a given pivot index. |
| [`Concat`](#concat) | trait | Defines `GenericSequence`s which can be joined together, forming a larger array. |
| [`SequenceItem`](#sequenceitem) | type | Accessor for `GenericSequence` item type, which is really `IntoIterator::Item` |

## Traits

### `GenericSequence<T>`

```rust
trait GenericSequence<T>: Sized + IntoIterator { ... }
```

Defines some sequence with an associated length and iteration capabilities.

This is useful for passing N-length generic arrays as generics.

#### Associated Types

- `type Length: 1`

- `type Sequence: 2`

#### Required Methods

- `fn generate<F>(f: F) -> <Self as >::Sequence`

  Initializes a new sequence instance using the given function.

#### Implementors

- [`GenericArray`](../index.md#genericarray)
- `&'a S`
- `&'a mut S`

### `Lengthen<T>`

```rust
trait Lengthen<T>: Sized + GenericSequence<T> { ... }
```

Defines any `GenericSequence` which can be lengthened or extended by appending
or prepending an element to it.

Any lengthened sequence can be shortened back to the original using `pop_front` or `pop_back`

#### Associated Types

- `type Longer: 1`

#### Required Methods

- `fn append(self, last: T) -> <Self as >::Longer`

  Returns a new array with the given element appended to the end of it.

- `fn prepend(self, first: T) -> <Self as >::Longer`

  Returns a new array with the given element prepended to the front of it.

#### Implementors

- [`GenericArray`](../index.md#genericarray)

### `Shorten<T>`

```rust
trait Shorten<T>: Sized + GenericSequence<T> { ... }
```

Defines a `GenericSequence` which can be shortened by removing the first or last element from it.

Additionally, any shortened sequence can be lengthened by
appending or prepending an element to it.

#### Associated Types

- `type Shorter: 1`

#### Required Methods

- `fn pop_back(self) -> (<Self as >::Shorter, T)`

  Returns a new array without the last element, and the last element.

- `fn pop_front(self) -> (T, <Self as >::Shorter)`

  Returns a new array without the first element, and the first element.

#### Implementors

- [`GenericArray`](../index.md#genericarray)

### `Split<T, K>`

```rust
trait Split<T, K>: GenericSequence<T>
where
    K: ArrayLength<T> { ... }
```

Defines a `GenericSequence` that can be split into two parts at a given pivot index.

#### Associated Types

- `type First: 1`

- `type Second: 1`

#### Required Methods

- `fn split(self) -> (<Self as >::First, <Self as >::Second)`

  Splits an array at the given index, returning the separate parts of the array.

#### Implementors

- [`GenericArray`](../index.md#genericarray)
- `&'a GenericArray<T, N>`
- `&'a mut GenericArray<T, N>`

### `Concat<T, M>`

```rust
trait Concat<T, M>: GenericSequence<T>
where
    M: ArrayLength<T> { ... }
```

Defines `GenericSequence`s which can be joined together, forming a larger array.

#### Associated Types

- `type Rest: 1`

- `type Output: 1`

#### Required Methods

- `fn concat(self, rest: <Self as >::Rest) -> <Self as >::Output`

  Concatenate, or join, two sequences.

#### Implementors

- [`GenericArray`](../index.md#genericarray)

## Type Aliases

### `SequenceItem<T>`

```rust
type SequenceItem<T> = <T as IntoIterator>::Item;
```

Accessor for `GenericSequence` item type, which is really `IntoIterator::Item`

For deeply nested generic mapped sequence types, like shown in `tests/generics.rs`,
this can be useful for keeping things organized.

