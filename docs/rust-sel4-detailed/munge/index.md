# Crate `munge`

Munge makes it easy and safe to destructure `MaybeUninit`s, `Cell`s,
`UnsafeCell`s, `ManuallyDrop`s and more.

Just use the `munge!` macro to destructure opaque types the same way you'd
destructure a value. The `munge!` macro may be used to perform either borrow
destructuring (e.g. `let (a, b) = c` where `c` is a reference) or move
destructuring (e.g. `let (a, b) = c` where `c` is a value) depending on the
type.

Munge has no features and is always `#![no_std]`.

## Examples
Initialize `MaybeUninit`s:

```rust
use core::mem::MaybeUninit;
use munge::munge;

pub struct Example {
    a: u32,
    b: (char, f32),
}

let mut mu = MaybeUninit::<Example>::uninit();

munge!(let Example { a, b: (c, mut f) } = &mut mu);
assert_eq!(a.write(10), &10);
assert_eq!(c.write('x'), &'x');
assert_eq!(f.write(3.14), &3.14);
// Note that `mut` bindings can be reassigned like you'd expect:
f = &mut MaybeUninit::uninit();

// SAFETY: `mu` is completely initialized.
let init = unsafe { mu.assume_init() };
assert_eq!(init.a, 10);
assert_eq!(init.b.0, 'x');
assert_eq!(init.b.1, 3.14);
```

Destructure `Cell`s:

```rust
use core::cell::Cell;
use munge::munge;

pub struct Example {
    a: u32,
    b: (char, f32),
}

let value = Example {
    a: 10,
    b: ('x', 3.14),
};
let cell = Cell::<Example>::new(value);

munge!(let Example { a, b: (c, f) } = &cell);
assert_eq!(a.get(), 10);
a.set(42);
assert_eq!(c.get(), 'x');
c.set('!');
assert_eq!(f.get(), 3.14);
f.set(1.41);

let value = cell.into_inner();
assert_eq!(value.a, 42);
assert_eq!(value.b.0, '!');
assert_eq!(value.b.1, 1.41);
```

You can even extend munge to work with your own types by implementing its
`Destructure` and `Restructure` traits:

```rust
use munge::{Destructure, Restructure, Move, munge};

pub struct Invariant<T>(T);

impl<T> Invariant<T> {
    /// # Safety
    ///
    /// `value` must uphold my custom invariant.
    pub unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
    }

    pub fn unwrap(self) -> T {
        self.0
    }
}

// SAFETY:
// - `Invariant<T>` is destructured by move, so its `Destructuring` type is
//   `Move`.
// - `underlying` returns a pointer to its inner type, so it is guaranteed
//   to be non-null, properly aligned, and valid for reads.
unsafe impl<T> Destructure for Invariant<T> {
    type Underlying = T;
    type Destructuring = Move;

    fn underlying(&mut self) -> *mut Self::Underlying {
        &mut self.0 as *mut Self::Underlying
    }
}

// SAFETY: `restructure` returns an `Invariant<U>` that takes ownership of
// the restructured field because `Invariant<T>` is destructured by move.
unsafe impl<T, U> Restructure<U> for Invariant<T> {
    type Restructured = Invariant<U>;

    unsafe fn restructure(&self, ptr: *mut U) -> Self::Restructured {
        // SAFETY: The caller has guaranteed that `ptr` is a pointer to a
        // subfield of some `T`, so it must be properly aligned, valid for
        // reads, and initialized. We may move the fields because the
        // destructuring type for `Invariant<T>` is `Move`.
        let value = unsafe { ptr.read() };
        Invariant(value)
    }
}

// SAFETY: `(1, 2, 3)` upholds my custom invariant.
let value = unsafe { Invariant::new_unchecked((1, 2, 3)) };
munge!(let (one, two, three) = value);
assert_eq!(one.unwrap(), 1);
assert_eq!(two.unwrap(), 2);
assert_eq!(three.unwrap(), 3);
```

## Contents

- [Modules](#modules)
  - [`impls`](#impls)
  - [`internal`](#internal)
- [Structs](#structs)
  - [`Borrow`](#borrow)
  - [`Move`](#move)
- [Traits](#traits)
  - [`Destructure`](#destructure)
  - [`Restructure`](#restructure)
- [Macros](#macros)
  - [`munge!`](#munge)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`impls`](#impls) | mod |  |
| [`internal`](#internal) | mod |  |
| [`Borrow`](#borrow) | struct | Destructuring by borrow, e.g. `let (a, b) = c` where `c` is a reference. |
| [`Move`](#move) | struct | Destructuring by move, e.g. `let (a, b) = c` where `c` is a value. |
| [`Destructure`](#destructure) | trait | A type that can be destructured into its constituent parts. |
| [`Restructure`](#restructure) | trait | A type that can be "restructured" as a field of some containing type. |
| [`munge!`](#munge) | macro | Destructures a type using a pattern. |

## Modules

- [`impls`](impls/index.md)
- [`internal`](internal/index.md)

## Structs

### `Borrow`

```rust
struct Borrow;
```

Destructuring by borrow, e.g. `let (a, b) = c` where `c` is a reference.

Borrow destructuring leaves the original value intact, only borrowing from
the destructured value. Borrow destructuring may use rest patterns (`..`)
because the original value is not moved and so it is safe to restructure
only some of the fields of the destructured value.

#### Trait Implementations

##### `impl Destructuring for Borrow`

##### `impl<T: Destructure> DestructuringFor for Borrow`

- <span id="borrow-destructuringfor-type-destructurer"></span>`type Destructurer = Borrow<T>`

### `Move`

```rust
struct Move;
```

Destructuring by move, e.g. `let (a, b) = c` where `c` is a value.

Move destructuring forgets the original value and moves each destructured
field during restructuring. Move destructuring may not use rest patterns
(`..`) because every field of the original value must be restructured, else
they will be forgotten.

#### Trait Implementations

##### `impl Destructuring for Move`

##### `impl<T: Destructure> DestructuringFor for Move`

- <span id="move-destructuringfor-type-destructurer"></span>`type Destructurer = Move<T>`

## Traits

### `Destructure`

```rust
trait Destructure: Sized { ... }
```

A type that can be destructured into its constituent parts.

See the [crate docs](index.html#examples) for an example of implementing
`Destructure` and `Restructure`.

# Safety

- [`Destructuring`](Destructure::Destructuring) must reflect the type of
  destructuring allowed for the type:
  - [`Borrow`](#borrow) if the type is restructured by creating disjoint borrows of
    the fields of `Underlying`.
  - [`Move`](#move) if the type may be restructured by moving the fields out of the
    destructured `Underlying`.
- [`underlying`](Destructure::underlying) must return a pointer that is
  non-null, properly aligned, and valid for reads.

#### Associated Types

- `type Underlying: 1`

- `type Destructuring: 1`

#### Required Methods

- `fn underlying(&mut self) -> *mut <Self as >::Underlying`

  Returns a mutable pointer to the underlying type.

#### Implementors

- `&core::cell::Cell<T>`
- `&core::cell::UnsafeCell<T>`
- `&core::mem::ManuallyDrop<T>`
- `&core::mem::MaybeUninit<T>`
- `&mut core::cell::Cell<T>`
- `&mut core::cell::UnsafeCell<T>`
- `&mut core::mem::ManuallyDrop<T>`
- `&mut core::mem::MaybeUninit<T>`
- `core::cell::Cell<T>`
- `core::cell::UnsafeCell<T>`
- `core::mem::ManuallyDrop<T>`
- `core::mem::MaybeUninit<T>`

### `Restructure<T: ?Sized>`

```rust
trait Restructure<T: ?Sized>: Destructure { ... }
```

A type that can be "restructured" as a field of some containing type.

See the [crate docs](index.html#examples) for an example of implementing
`Destructure` and `Restructure`.

# Safety

[`restructure`](Restructure::restructure) must return a valid
[`Restructured`](Restructure::Restructured) that upholds the invariants for
its [`Destructuring`](Destructure::Destructuring):
- If the type is destructured [by borrow](Borrow), then the `Restructured`
  value must behave as a disjoint borrow of a field of the underlying type.
- If the type is destructured [by move](Move), then the `Restructured` value
  must move the fields out of the underlying type.

#### Associated Types

- `type Restructured`

#### Required Methods

- `fn restructure(&self, ptr: *mut T) -> <Self as >::Restructured`

  Restructures a pointer to this type into the target type.

#### Implementors

- `&'a core::cell::Cell<T>`
- `&'a core::cell::UnsafeCell<T>`
- `&'a core::mem::ManuallyDrop<T>`
- `&'a core::mem::MaybeUninit<T>`
- `&'a mut core::cell::Cell<T>`
- `&'a mut core::cell::UnsafeCell<T>`
- `&'a mut core::mem::ManuallyDrop<T>`
- `&'a mut core::mem::MaybeUninit<T>`
- `core::cell::Cell<T>`
- `core::cell::UnsafeCell<T>`
- `core::mem::ManuallyDrop<T>`
- `core::mem::MaybeUninit<T>`

## Macros

### `munge!`

Destructures a type using a pattern.

To prevent unsound union destructurings, this macro emits field accesses
which fail to compile in safe Rust. However, if `munge!` is used inside of
an `unsafe` block, these accesses will compile without emitting an error.
This matches the behavior of regular destructuring, but may be surprising in
some situations.

# Example

```rust
use core::mem::MaybeUninit;
use munge::munge;
pub struct Example {
    a: u32,
    b: (char, f32),
}

let mut mu = MaybeUninit::<Example>::uninit();

munge!(let Example { a, b: (c, mut f) } = &mut mu);
assert_eq!(a.write(10), &10);
assert_eq!(c.write('x'), &'x');
assert_eq!(f.write(3.14), &3.14);
// Note that `mut` bindings can be reassigned like you'd expect:
let mut new_f = MaybeUninit::uninit();
f = &mut new_f;

// SAFETY: `mu` is completely initialized.
let init = unsafe { mu.assume_init() };
assert_eq!(init.a, 10);
assert_eq!(init.b.0, 'x');
assert_eq!(init.b.1, 3.14);
```

