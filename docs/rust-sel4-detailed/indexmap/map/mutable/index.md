*[indexmap](../../index.md) / [map](../index.md) / [mutable](index.md)*

---

# Module `mutable`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MutableKeys`](#mutablekeys) | trait | Opt-in mutable access to [`IndexMap`] keys. |
| [`MutableEntryKey`](#mutableentrykey) | trait | Opt-in mutable access to [`Entry`] keys. |
| [`Sealed`](#sealed) | trait |  |

## Traits

### `MutableKeys`

```rust
trait MutableKeys: Sealed { ... }
```

Opt-in mutable access to [`IndexMap`](../index.md) keys.

These methods expose `&mut K`, mutable references to the key as it is stored
in the map.
You are allowed to modify the keys in the map **if the modification
does not change the key's hash and equality**.

If keys are modified erroneously, you can no longer look them up.
This is sound (memory safe) but a logical error hazard (just like
implementing `PartialEq`, `Eq`, or `Hash` incorrectly would be).

`use` this trait to enable its methods for `IndexMap`.

This trait is sealed and cannot be implemented for types outside this crate.

#### Associated Types

- `type Key`

- `type Value`

#### Required Methods

- `fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut <Self as >::Key, &mut <Self as >::Value)>`

  Return item index, mutable reference to key and value

- `fn get_index_mut2(&mut self, index: usize) -> Option<(&mut <Self as >::Key, &mut <Self as >::Value)>`

  Return mutable reference to key and value at an index.

- `fn iter_mut2(&mut self) -> IterMut2<'_, <Self as >::Key, <Self as >::Value>`

  Return an iterator over the key-value pairs of the map, in their order

- `fn retain2<F>(&mut self, keep: F)`

  Scan through each key-value pair in the map and keep those where the

#### Implementors

- [`IndexMap`](../index.md#indexmap)

### `MutableEntryKey`

```rust
trait MutableEntryKey: Sealed { ... }
```

Opt-in mutable access to [`Entry`](../entry/index.md) keys.

These methods expose `&mut K`, mutable references to the key as it is stored
in the map.
You are allowed to modify the keys in the map **if the modification
does not change the key's hash and equality**.

If keys are modified erroneously, you can no longer look them up.
This is sound (memory safe) but a logical error hazard (just like
implementing `PartialEq`, `Eq`, or `Hash` incorrectly would be).

`use` this trait to enable its methods for `Entry`.

This trait is sealed and cannot be implemented for types outside this crate.

#### Associated Types

- `type Key`

#### Required Methods

- `fn key_mut(&mut self) -> &mut <Self as >::Key`

  Gets a mutable reference to the entry's key, either within the map if occupied,

#### Implementors

- [`Entry`](../entry/index.md#entry)
- [`IndexedEntry`](../entry/index.md#indexedentry)
- [`OccupiedEntry`](../../inner/entry/index.md#occupiedentry)
- [`VacantEntry`](../../inner/entry/index.md#vacantentry)

### `Sealed`

```rust
trait Sealed { ... }
```

#### Implementors

- [`Entry`](../entry/index.md#entry)
- [`IndexMap`](../index.md#indexmap)
- [`IndexedEntry`](../entry/index.md#indexedentry)
- [`OccupiedEntry`](../../inner/entry/index.md#occupiedentry)
- [`VacantEntry`](../../inner/entry/index.md#vacantentry)

