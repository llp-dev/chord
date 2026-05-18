**sel4_abstract_ptr > abstract_ptr**

# Module: abstract_ptr

## Contents

**Structs**

- [`AbstractPtr`](#abstractptr)

---

## sel4_abstract_ptr::abstract_ptr::AbstractPtr

*Struct*

**Generic Parameters:**
- 'a
- M
- T
- A

**Methods:**

- `fn fill(self: Self, value: u8)`
- `fn read(self: Self) -> T`
- `fn write(self: Self, value: T)`
- `fn update<F>(self: Self, f: F)`
- `fn as_raw_ptr(self: Self) -> NonNull<T>`
- `fn map<F, U>(self: Self, f: F) -> AbstractPtr<'a, M, U, A>`
- `fn len(self: Self) -> usize`
- `fn is_empty(self: Self) -> bool`
- `fn index<I>(self: Self, index: I) -> AbstractPtr<'a, M, <I as SliceIndex>::Output, A>`
- `fn iter(self: Self) -> impl Trait`
- `fn copy_into_slice(self: Self, dst: & mut [T])`
- `fn copy_from_slice(self: Self, src: &[T])`
- `fn copy_within<impl RangeBounds<usize>>(self: Self, src: impl Trait, dest: usize)`
- `fn split_at(self: Self, mid: usize) -> (AbstractPtr<'a, M, [T], A>, AbstractPtr<'a, M, [T], A>)`
- `fn as_chunks<const N>(self: Self) -> (AbstractPtr<'a, M, [[T; N]], A>, AbstractPtr<'a, M, [T], A>)`
- `fn as_chunks_unchecked<const N>(self: Self) -> AbstractPtr<'a, M, [[T; N]], A>`
- `fn new(pointer: NonNull<T>) -> AbstractPtr<'a, M, T, ReadWrite>`
- `fn new_read_only(pointer: NonNull<T>) -> AbstractPtr<'a, M, T, ReadOnly>`
- `fn new_restricted<A>(access: A, pointer: NonNull<T>) -> AbstractPtr<'a, M, T, A>`
- `fn atomic_load(self: Self, order: Ordering) -> <M as >::Value`
- `fn atomic_store(self: Self, val: <M as >::Value, order: Ordering)`
- `fn atomic_swap(self: Self, val: <M as >::Value, order: Ordering) -> <M as >::Value`
- `fn atomic_compare_exchange(self: Self, current: <M as >::Value, new: <M as >::Value, success: Ordering, failure: Ordering) -> Result<<M as >::Value, <M as >::Value>`
- `fn atomic_compare_exchange_weak(self: Self, current: <M as >::Value, new: <M as >::Value, success: Ordering, failure: Ordering) -> Result<<M as >::Value, <M as >::Value>`
- `fn atomic_fetch_add(self: Self, val: <M as >::Value, order: Ordering) -> <M as >::Value`
- `fn atomic_fetch_sub(self: Self, val: <M as >::Value, order: Ordering) -> <M as >::Value`
- `fn atomic_fetch_and(self: Self, val: <M as >::Value, order: Ordering) -> <M as >::Value`
- `fn atomic_fetch_nand(self: Self, val: <M as >::Value, order: Ordering) -> <M as >::Value`
- `fn atomic_fetch_or(self: Self, val: <M as >::Value, order: Ordering) -> <M as >::Value`
- `fn atomic_fetch_xor(self: Self, val: <M as >::Value, order: Ordering) -> <M as >::Value`
- `fn atomic_fetch_update<F>(self: Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<<M as >::Value, <M as >::Value>`
- `fn atomic_fetch_max(self: Self, val: <M as >::Value, order: Ordering) -> <M as >::Value`
- `fn atomic_fetch_min(self: Self, val: <M as >::Value, order: Ordering) -> <M as >::Value`
- `fn read_only(self: Self) -> AbstractPtr<'a, M, T, ReadOnly>`
- `fn write_only(self: Self) -> AbstractPtr<'a, M, T, WriteOnly>`
- `fn restrict<To>(self: Self) -> AbstractPtr<'a, M, T, <A as >::Restricted>`
- `fn as_slice(self: Self) -> AbstractPtr<'a, M, [T], A>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Pointer**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



