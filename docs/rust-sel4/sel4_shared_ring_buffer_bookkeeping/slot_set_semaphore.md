**sel4_shared_ring_buffer_bookkeeping > slot_set_semaphore**

# Module: slot_set_semaphore

## Contents

**Structs**

- [`DummySlotSemaphore`](#dummyslotsemaphore)
- [`SlotReservationExhaustedError`](#slotreservationexhaustederror)
- [`SlotSemaphoreClosedError`](#slotsemaphoreclosederror)
- [`SlotSetReservation`](#slotsetreservation)
- [`SlotSetSemaphore`](#slotsetsemaphore)
- [`SlotSetSemaphoreHandle`](#slotsetsemaphorehandle)

**Enums**

- [`Error`](#error)

**Traits**

- [`AsyncSlotSemaphore`](#asyncslotsemaphore)
- [`SlotSemaphore`](#slotsemaphore)

---

## sel4_shared_ring_buffer_bookkeeping::slot_set_semaphore::AsyncSlotSemaphore

*Trait*

**Methods:**

- `take`



## sel4_shared_ring_buffer_bookkeeping::slot_set_semaphore::DummySlotSemaphore

*Struct*



## sel4_shared_ring_buffer_bookkeeping::slot_set_semaphore::Error

*Enum*

**Variants:**
- `SlotReservationExhausted`
- `SlotCountTrackingError`

**Trait Implementations:**

- **From**
  - `fn from(_err: SlotCountTrackerError) -> Self`
- **From**
  - `fn from(_err: SlotReservationExhaustedError) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_shared_ring_buffer_bookkeeping::slot_set_semaphore::SlotReservationExhaustedError

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_shared_ring_buffer_bookkeeping::slot_set_semaphore::SlotSemaphore

*Trait*

**Methods:**

- `new`
- `try_take`
- `give`
- `close`



## sel4_shared_ring_buffer_bookkeeping::slot_set_semaphore::SlotSemaphoreClosedError

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_shared_ring_buffer_bookkeeping::slot_set_semaphore::SlotSetReservation

*Struct*

**Generic Parameters:**
- 'a
- T
- const N

**Methods:**

- `fn count(self: &Self) -> usize`
- `fn split(self: & mut Self, split_off: usize) -> Result<Self, SlotReservationExhaustedError>`
- `fn merge(self: & mut Self, other: Self)`

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`



## sel4_shared_ring_buffer_bookkeeping::slot_set_semaphore::SlotSetSemaphore

*Struct*

**Generic Parameters:**
- T
- const N

**Methods:**

- `fn new(slot_pool_capacities: [usize; N]) -> Self`
- `fn close(self: &Self)`
- `fn handle(self: &Self) -> &SlotSetSemaphoreHandle<T, N>`
- `fn consume(self: & mut Self, reservation: & mut SlotSetReservation<T, N>, n: usize) -> Result<(), Error>`
- `fn report_current_num_free_slots(self: & mut Self, slot_pool_index: usize, current_num_free_slots: usize) -> Result<(), SlotCountTrackerError>`



## sel4_shared_ring_buffer_bookkeeping::slot_set_semaphore::SlotSetSemaphoreHandle

*Struct*

**Generic Parameters:**
- T
- const N

**Methods:**

- `fn reserve(self: &Self, n: usize) -> Result<SlotSetReservation<T, N>, SlotSemaphoreClosedError>`
- `fn try_reserve(self: &Self, n: usize) -> Result<Option<SlotSetReservation<T, N>>, SlotSemaphoreClosedError>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SlotSetSemaphoreHandle<T, N>`



