**sel4_abstract_ptr > abstract_ref**

# Module: abstract_ref

## Contents

**Structs**

- [`AbstractRef`](#abstractref)

---

## sel4_abstract_ptr::abstract_ref::AbstractRef

*Struct*

**Generic Parameters:**
- 'a
- M
- T
- A

**Methods:**

- `fn read_only(self: Self) -> AbstractRef<'a, M, T, ReadOnly>`
- `fn write_only(self: Self) -> AbstractRef<'a, M, T, WriteOnly>`
- `fn restrict<To>(self: Self) -> AbstractRef<'a, M, T, <A as >::Restricted>`
- `fn borrow(self: &Self) -> AbstractRef<M, T, <A as >::Restricted>`
- `fn borrow_mut(self: & mut Self) -> AbstractRef<M, T, A>`
- `fn as_ptr(self: &Self) -> AbstractPtr<M, T, <A as >::Restricted>`
- `fn as_mut_ptr(self: & mut Self) -> AbstractPtr<M, T, A>`
- `fn into_ptr(self: Self) -> AbstractPtr<'a, M, T, A>`
- `fn new(pointer: NonNull<T>) -> Self`
- `fn new_read_only(pointer: NonNull<T>) -> AbstractRef<'a, M, T, ReadOnly>`
- `fn new_restricted<A>(access: A, pointer: NonNull<T>) -> AbstractRef<'a, M, T, A>`
- `fn from_ref(reference: &'a T) -> AbstractRef<'a, M, T, ReadOnly>`
- `fn from_mut_ref(reference: &'a  mut T) -> Self`

**Traits:** Eq, Sync, Send, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Pointer**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> Ordering`



