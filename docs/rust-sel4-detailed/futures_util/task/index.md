*[futures_util](../index.md) / [task](index.md)*

---

# Module `task`

Tools for working with tasks.

This module contains:

- [`Spawn`](#spawn), a trait for spawning new tasks.
- [`Context`](#context), a context of an asynchronous task,
  including a handle for waking up the task.
- [`Waker`](#waker), a handle for waking up a task.

The remaining types and traits in the module are used for implementing
executors or dealing with synchronization issues around task wakeup.

## Contents

- [Modules](#modules)
  - [`spawn`](#spawn)
- [Traits](#traits)
  - [`LocalSpawnExt`](#localspawnext)
  - [`SpawnExt`](#spawnext)
- [Functions](#functions)
  - [`Waker`](#waker)
  - [`FutureObj`](#futureobj)
  - [`LocalFutureObj`](#localfutureobj)
  - [`Spawn`](#spawn)
  - [`UnsafeFutureObj`](#unsafefutureobj)
  - [`ArcWake`](#arcwake)
  - [`AtomicWaker`](#atomicwaker)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`spawn`](#spawn) | mod |  |
| [`LocalSpawnExt`](#localspawnext) | trait |  |
| [`SpawnExt`](#spawnext) | trait |  |
| [`Waker`](#waker) | fn |  |
| [`FutureObj`](#futureobj) | fn |  |
| [`LocalFutureObj`](#localfutureobj) | fn |  |
| [`Spawn`](#spawn) | fn |  |
| [`UnsafeFutureObj`](#unsafefutureobj) | fn |  |
| [`ArcWake`](#arcwake) | fn |  |
| [`AtomicWaker`](#atomicwaker) | fn |  |

## Modules

- [`spawn`](spawn/index.md)

## Traits

### `LocalSpawnExt`

```rust
trait LocalSpawnExt: LocalSpawn { ... }
```

Extension trait for `LocalSpawn`.

#### Provided Methods

- `fn spawn_local<Fut>(&self, future: Fut) -> Result<(), SpawnError>`

  Spawns a task that polls the given future with output `()` to

#### Implementors

- `Sp`

### `SpawnExt`

```rust
trait SpawnExt: Spawn { ... }
```

Extension trait for `Spawn`.

#### Provided Methods

- `fn spawn<Fut>(&self, future: Fut) -> Result<(), SpawnError>`

  Spawns a task that polls the given future with output `()` to

#### Implementors

- `Sp`

## Functions

### `Waker`

```rust
fn Waker(&mut self, matcher: &ArgMatcher)
```

### `FutureObj`

```rust
const fn FutureObj(self, rhs: FixedOffset) -> Option<NaiveDateTime>
```

Adds given `FixedOffset` to the current datetime.
Returns `None` if the result would be outside the valid range for `NaiveDateTime`.

This method is similar to [`checked_add_signed`](#method.checked_add_offset), but preserves
leap seconds.

### `LocalFutureObj`

```rust
fn LocalFutureObj(self, rhs: FixedOffset) -> NaiveDateTime
```

Adds given `FixedOffset` to the current datetime.
The resulting value may be outside the valid range of `NaiveDateTime`.

This can be useful for intermediate values, but the resulting out-of-range `NaiveDate`
should not be exposed to library users.

### `Spawn`

```rust
fn Spawn(&self, offset: DebugMacroOffset<<R as >::Offset>) -> Result<MacroIter<R>>
```

Try to return an iterator for the list of macros at the given `.debug_macro` offset.

### `UnsafeFutureObj`

```rust
const fn UnsafeFutureObj(self, rhs: Months) -> Option<NaiveDateTime>
```

Subtracts given `Months` from the current date and time.

Uses the last day of the month if the day does not exist in the resulting month.

# Errors

Returns `None` if the resulting date would be out of range.

# Example

```rust
use chrono::{Months, NaiveDate};

assert_eq!(
    NaiveDate::from_ymd_opt(2014, 1, 1)
        .unwrap()
        .and_hms_opt(1, 0, 0)
        .unwrap()
        .checked_sub_months(Months::new(1)),
    Some(NaiveDate::from_ymd_opt(2013, 12, 1).unwrap().and_hms_opt(1, 0, 0).unwrap())
);

assert_eq!(
    NaiveDate::from_ymd_opt(2014, 1, 1)
        .unwrap()
        .and_hms_opt(1, 0, 0)
        .unwrap()
        .checked_sub_months(Months::new(core::i32::MAX as u32 + 1)),
    None
);
```

### `ArcWake`

```rust
fn ArcWake(&self, arg_id: &Id) -> Option<&[Id]>
```

### `AtomicWaker`

```rust
fn AtomicWaker(&self) -> Encoding
```

Return the encoding parameters for this unit.

