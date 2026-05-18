*[indexmap](../../index.md) / [set](../index.md) / [mutable](index.md)*

---

# Module `mutable`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MutableValues`](#mutablevalues) | trait | Opt-in mutable access to [`IndexSet`] values. |
| [`Sealed`](#sealed) | trait |  |

## Traits

### `MutableValues`

```rust
trait MutableValues: Sealed { ... }
```

Opt-in mutable access to [`IndexSet`](../index.md) values.

These methods expose `&mut T`, mutable references to the value as it is stored
in the set.
You are allowed to modify the values in the set **if the modification
does not change the value's hash and equality**.

If values are modified erroneously, you can no longer look them up.
This is sound (memory safe) but a logical error hazard (just like
implementing `PartialEq`, `Eq`, or `Hash` incorrectly would be).

`use` this trait to enable its methods for `IndexSet`.

This trait is sealed and cannot be implemented for types outside this crate.

#### Associated Types

- `type Value`

#### Required Methods

- `fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut <Self as >::Value)>`

  Return item index and mutable reference to the value

- `fn get_index_mut2(&mut self, index: usize) -> Option<&mut <Self as >::Value>`

  Return mutable reference to the value at an index.

- `fn retain2<F>(&mut self, keep: F)`

  Scan through each value in the set and keep those where the

#### Implementors

- [`IndexSet`](../index.md#indexset)

### `Sealed`

```rust
trait Sealed { ... }
```

#### Implementors

- [`IndexSet`](../index.md#indexset)

