**sel4 > init_thread**

# Module: init_thread

## Contents

**Modules**

- [`slot`](#slot) - Initial CSpace slot constants corresponding to `seL4_Cap*`.

**Structs**

- [`Slot`](#slot) - The index of a slot in the initial thread's root CNode.
- [`SlotRegion`](#slotregion) - Corresponds to `seL4_SlotRegion`.

**Functions**

- [`suspend_self`](#suspend_self) - Suspends the initial thread using [`slot::TCB`].

---

## sel4::init_thread::Slot

*Struct*

The index of a slot in the initial thread's root CNode.

**Generic Parameters:**
- T

**Methods:**

- `fn from_index(index: usize) -> Self`
- `fn index(self: &Self) -> usize`
- `fn cptr_bits(self: &Self) -> CPtrBits`
- `fn cptr(self: &Self) -> CPtr`
- `fn cap(self: &Self) -> Cap<T>`
- `fn cast<T1>(self: &Self) -> Slot<T1>`
- `fn upcast(self: &Self) -> Slot`
- `fn downcast<T>(self: &Self) -> Slot<T>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Slot<T>) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Slot<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Slot<T>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Slot<T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## sel4::init_thread::SlotRegion

*Struct*

Corresponds to `seL4_SlotRegion`.

**Generic Parameters:**
- T

**Methods:**

- `fn start(self: &Self) -> usize`
- `fn end(self: &Self) -> usize`
- `fn range(self: &Self) -> Range<usize>`
- `fn len(self: &Self) -> usize`
- `fn index(self: &Self, i: usize) -> Slot<T>`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SlotRegion<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SlotRegion<T>`



## Module: slot

Initial CSpace slot constants corresponding to `seL4_Cap*`.



## sel4::init_thread::suspend_self

*Function*

Suspends the initial thread using [`slot::TCB`].

```rust
fn suspend_self<T>() -> T
```



