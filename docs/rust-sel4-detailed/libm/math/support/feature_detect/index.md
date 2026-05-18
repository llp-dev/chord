*[libm](../../../index.md) / [math](../../index.md) / [support](../index.md) / [feature_detect](index.md)*

---

# Module `feature_detect`

Helpers for runtime target feature detection that are shared across architectures.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Flags`](#flags) | struct | Helper for working with bit flags, based on `bitflags`. |
| [`get_or_init_flags_cache`](#get-or-init-flags-cache) | fn | Load flags from an atomic value. |
| [`unique_masks!`](#unique-masks) | macro | Given a list of identifiers, assign each one a unique sequential single-bit mask. |
| [`select_once!`](#select-once) | macro | Call `init` once to choose an implementation, then use it for the rest of the program. |

## Structs

### `Flags`

```rust
struct Flags(u32);
```

Helper for working with bit flags, based on `bitflags`.

#### Implementations

- <span id="flags-empty"></span>`const fn empty() -> Self`

  No bits set.

- <span id="flags-from-bits"></span>`const fn from_bits(val: u32) -> Self`

  Create with bits already set.

- <span id="flags-bits"></span>`fn bits(&self) -> u32`

  Get the integer representation.

- <span id="flags-insert"></span>`fn insert(&mut self, mask: u32)`

  Set any bits in `mask`.

- <span id="flags-contains"></span>`fn contains(&self, mask: u32) -> bool`

  Check whether the mask is set.

- <span id="flags-test-nth"></span>`fn test_nth(&self, bit: u32) -> bool`

  Check whether the nth bit is set.

#### Trait Implementations

##### `impl Clone for Flags`

- <span id="flags-clone"></span>`fn clone(&self) -> Flags` — [`Flags`](#flags)

##### `impl Copy for Flags`

##### `impl Debug for Flags`

- <span id="flags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Flags`

- <span id="flags-partialeq-eq"></span>`fn eq(&self, other: &Flags) -> bool` — [`Flags`](#flags)

##### `impl StructuralPartialEq for Flags`

## Functions

### `get_or_init_flags_cache`

```rust
fn get_or_init_flags_cache(cache: &core::sync::atomic::AtomicU32, init: impl FnOnce() -> Flags) -> Flags
```

Load flags from an atomic value. If the flags have not yet been initialized, call `init`
to do so.

Note that `init` may run more than once.

## Macros

### `unique_masks!`

Given a list of identifiers, assign each one a unique sequential single-bit mask.

### `select_once!`

Call `init` once to choose an implementation, then use it for the rest of the program.

- `sig` is the function type.
- `init` is an expression called at startup that chooses an implementation and returns a
  function pointer.
- `call` is an expression to call a function returned by `init`, encapsulating any safety
  preconditions.

The type `Func` is available in `init` and `call`.

This is effectively our version of an ifunc without linker support. Note that `init` may be
called more than once until one completes.

