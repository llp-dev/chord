**munge**

# Module: munge

## Contents

**Macros**

- [`munge`](#munge) - Destructures a type using a pattern.

**Structs**

- [`Borrow`](#borrow) - Destructuring by borrow, e.g. `let (a, b) = c` where `c` is a reference.
- [`Move`](#move) - Destructuring by move, e.g. `let (a, b) = c` where `c` is a value.

**Traits**

- [`Destructure`](#destructure) - A type that can be destructured into its constituent parts.
- [`Restructure`](#restructure) - A type that can be "restructured" as a field of some containing type.

---

## munge::Borrow

*Struct*

Destructuring by borrow, e.g. `let (a, b) = c` where `c` is a reference.

Borrow destructuring leaves the original value intact, only borrowing from
the destructured value. Borrow destructuring may use rest patterns (`..`)
because the original value is not moved and so it is safe to restructure
only some of the fields of the destructured value.

**Unit Struct**



## munge::Destructure

*Trait*

A type that can be destructured into its constituent parts.

See the [crate docs](index.html#examples) for an example of implementing
`Destructure` and `Restructure`.

# Safety

- [`Destructuring`](Destructure::Destructuring) must reflect the type of
  destructuring allowed for the type:
  - [`Borrow`] if the type is restructured by creating disjoint borrows of
    the fields of `Underlying`.
  - [`Move`] if the type may be restructured by moving the fields out of the
    destructured `Underlying`.
- [`underlying`](Destructure::underlying) must return a pointer that is
  non-null, properly aligned, and valid for reads.

**Methods:**

- `Underlying`: The underlying type that is destructured.
- `Destructuring`: The type of destructuring to perform.
- `underlying`: Returns a mutable pointer to the underlying type.



## munge::Move

*Struct*

Destructuring by move, e.g. `let (a, b) = c` where `c` is a value.

Move destructuring forgets the original value and moves each destructured
field during restructuring. Move destructuring may not use rest patterns
(`..`) because every field of the original value must be restructured, else
they will be forgotten.

**Unit Struct**



## munge::Restructure

*Trait*

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

**Methods:**

- `Restructured`: The restructured version of this type.
- `restructure`: Restructures a pointer to this type into the target type.



## munge::munge

*Declarative Macro*

Destructures a type using a pattern.

To prevent unsound union destructurings, this macro emits field accesses
which fail to compile in safe Rust. However, if `munge!` is used inside of
an `unsafe` block, these accesses will compile without emitting an error.
This matches the behavior of regular destructuring, but may be surprising in
some situations.

# Example

```
# use core::mem::MaybeUninit;
# use munge::munge;
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

```rust
macro_rules! munge {
    ($($t:tt)*) => { ... };
}
```



