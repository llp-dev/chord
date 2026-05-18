*[munge](../index.md) / [internal](index.md)*

---

# Module `internal`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Borrow`](#borrow) | struct |  |
| [`Move`](#move) | struct |  |
| [`Destructuring`](#destructuring) | trait |  |
| [`DestructuringFor`](#destructuringfor) | trait |  |
| [`Destructurer`](#destructurer) | trait |  |
| [`Test`](#test) | trait |  |

## Structs

### `Borrow<T>`

```rust
struct Borrow<T>(T);
```

#### Trait Implementations

##### `impl<T: Destructure> Destructurer for Borrow<T>`

- <span id="borrow-destructurer-type-inner"></span>`type Inner = T`

- <span id="borrow-destructurer-new"></span>`fn new(inner: T) -> Self`

- <span id="borrow-destructurer-inner"></span>`fn inner(&self) -> &<Self as >::Inner` — [`Destructurer`](#destructurer)

- <span id="borrow-destructurer-inner-mut"></span>`fn inner_mut(&mut self) -> &mut <Self as >::Inner` — [`Destructurer`](#destructurer)

##### `impl<T: 'a + Destructure> Test for Borrow<T>`

- <span id="borrow-test-type-test"></span>`type Test = &'a <T as Destructure>::Underlying`

### `Move<T>`

```rust
struct Move<T>(core::mem::ManuallyDrop<T>);
```

#### Trait Implementations

##### `impl<T: Destructure> Destructurer for Move<T>`

- <span id="move-destructurer-type-inner"></span>`type Inner = T`

- <span id="move-destructurer-new"></span>`fn new(inner: T) -> Self`

- <span id="move-destructurer-inner"></span>`fn inner(&self) -> &<Self as >::Inner` — [`Destructurer`](#destructurer)

- <span id="move-destructurer-inner-mut"></span>`fn inner_mut(&mut self) -> &mut <Self as >::Inner` — [`Destructurer`](#destructurer)

##### `impl<T: 'a + Destructure> Test for Move<T>`

- <span id="move-test-type-test"></span>`type Test = <T as Destructure>::Underlying`

## Traits

### `Destructuring`

```rust
trait Destructuring { ... }
```

#### Implementors

- [`Borrow`](../index.md#borrow)
- [`Move`](../index.md#move)

### `DestructuringFor<T>`

```rust
trait DestructuringFor<T>: Destructuring { ... }
```

#### Associated Types

- `type Destructurer: 1`

#### Implementors

- [`Borrow`](../index.md#borrow)
- [`Move`](../index.md#move)

### `Destructurer`

```rust
trait Destructurer { ... }
```

#### Associated Types

- `type Inner: 1`

#### Required Methods

- `fn new(inner: <Self as >::Inner) -> Self`

- `fn inner(&self) -> &<Self as >::Inner`

- `fn inner_mut(&mut self) -> &mut <Self as >::Inner`

#### Implementors

- [`Borrow`](#borrow)
- [`Move`](#move)

### `Test<'a>`

```rust
trait Test<'a> { ... }
```

#### Associated Types

- `type Test`

#### Implementors

- [`Borrow`](#borrow)
- [`Move`](#move)

