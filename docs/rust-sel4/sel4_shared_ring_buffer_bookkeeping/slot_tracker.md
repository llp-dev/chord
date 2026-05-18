**sel4_shared_ring_buffer_bookkeeping > slot_tracker**

# Module: slot_tracker

## Contents

**Structs**

- [`SlotTracker`](#slottracker)

**Enums**

- [`SlotState`](#slotstate)
- [`SlotStateValueRef`](#slotstatevalueref)
- [`SlotStateValueRefMut`](#slotstatevaluerefmut)
- [`SlotTrackerError`](#slottrackererror)

**Traits**

- [`SlotStateTypes`](#slotstatetypes)

---

## sel4_shared_ring_buffer_bookkeeping::slot_tracker::SlotState

*Enum*

**Variants:**
- `Free`
- `Occupied`

**Methods:**

- `fn is_free(self: &Self) -> bool`
- `fn is_occupied(self: &Self) -> bool`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &SlotState) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &SlotState) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SlotState) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> SlotState`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## sel4_shared_ring_buffer_bookkeeping::slot_tracker::SlotStateTypes

*Trait*

**Methods:**

- `Common`
- `Free`
- `Occupied`



## sel4_shared_ring_buffer_bookkeeping::slot_tracker::SlotStateValueRef

*Enum*

**Generic Parameters:**
- 'a
- T

**Variants:**
- `Free(&'a <T as >::Free)`
- `Occupied(&'a <T as >::Occupied)`

**Methods:**

- `fn as_free(self: Self) -> core::result::Result<&'a <T as >::Free, SlotTrackerError>`
- `fn as_occupied(self: Self) -> core::result::Result<&'a <T as >::Occupied, SlotTrackerError>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &SlotStateValueRef<'a, T>) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &SlotStateValueRef<'a, T>) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SlotStateValueRef<'a, T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> SlotStateValueRef<'a, T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_shared_ring_buffer_bookkeeping::slot_tracker::SlotStateValueRefMut

*Enum*

**Generic Parameters:**
- 'a
- T

**Variants:**
- `Free(&'a  mut <T as >::Free)`
- `Occupied(&'a  mut <T as >::Occupied)`

**Methods:**

- `fn as_free(self: Self) -> core::result::Result<&'a  mut <T as >::Free, SlotTrackerError>`
- `fn as_occupied(self: Self) -> core::result::Result<&'a  mut <T as >::Occupied, SlotTrackerError>`

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &SlotStateValueRefMut<'a, T>) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &SlotStateValueRefMut<'a, T>) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SlotStateValueRefMut<'a, T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_shared_ring_buffer_bookkeeping::slot_tracker::SlotTracker

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new<impl Iterator<Item = (T::Common, T::Free)>>(iter: impl Trait) -> Self`
- `fn new_with_capacity(common: <T as >::Common, free: <T as >::Free, capacity: usize) -> Self`
- `fn new_occupied<impl Iterator<Item = (T::Common, T::Occupied)>>(iter: impl Trait) -> Self`
- `fn new_occupied_with_capacity(common: <T as >::Common, occupied: <T as >::Occupied, capacity: usize) -> Self`
- `fn capacity(self: &Self) -> usize`
- `fn num_free(self: &Self) -> usize`
- `fn num_occupied(self: &Self) -> usize`
- `fn peek_next_free_index(self: &Self) -> Option<usize>`
- `fn peek_next_free_value(self: &Self) -> Option<&<T as >::Free>`
- `fn get_common_value(self: &Self, index: usize) -> core::result::Result<&<T as >::Common, SlotTrackerError>`
- `fn get_common_value_mut(self: & mut Self, index: usize) -> core::result::Result<& mut <T as >::Common, SlotTrackerError>`
- `fn get_state(self: &Self, index: usize) -> core::result::Result<SlotState, SlotTrackerError>`
- `fn get_state_value(self: &Self, index: usize) -> core::result::Result<SlotStateValueRef<T>, SlotTrackerError>`
- `fn get_state_value_mut(self: & mut Self, index: usize) -> core::result::Result<SlotStateValueRefMut<T>, SlotTrackerError>`
- `fn occupy(self: & mut Self, value: <T as >::Occupied) -> Option<(usize, <T as >::Free)>`
- `fn free(self: & mut Self, index: usize, value: <T as >::Free) -> core::result::Result<<T as >::Occupied, SlotTrackerError>`



## sel4_shared_ring_buffer_bookkeeping::slot_tracker::SlotTrackerError

*Enum*

**Variants:**
- `OutOfBounds`
- `StateMismatch`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SlotTrackerError) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> SlotTrackerError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &SlotTrackerError) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &SlotTrackerError) -> $crate::cmp::Ordering`



