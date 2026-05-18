**sel4_shared_ring_buffer**

# Module: sel4_shared_ring_buffer

## Contents

**Modules**

- [`roles`](#roles)

**Structs**

- [`InitialState`](#initialstate)
- [`PeerMisbehaviorError`](#peermisbehaviorerror)
- [`RawRingBuffer`](#rawringbuffer)
- [`RingBuffer`](#ringbuffer)
- [`RingBuffers`](#ringbuffers)

**Enums**

- [`InitializationStrategy`](#initializationstrategy)

**Constants**

- [`RING_BUFFER_SIZE`](#ring_buffer_size)

---

## sel4_shared_ring_buffer::InitialState

*Struct*

**Methods:**

- `fn new(write_index: u32, read_index: u32) -> Self`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> InitialState`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &InitialState) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &InitialState) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &InitialState) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> InitialState`



## sel4_shared_ring_buffer::InitializationStrategy

*Enum*

**Variants:**
- `ReadState`
- `UseState(InitialState)`
- `UseAndWriteState(InitialState)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &InitializationStrategy) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &InitializationStrategy) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &InitializationStrategy) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> InitializationStrategy`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> InitializationStrategy`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## sel4_shared_ring_buffer::PeerMisbehaviorError

*Struct*

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &PeerMisbehaviorError) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &PeerMisbehaviorError) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &PeerMisbehaviorError) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> PeerMisbehaviorError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_shared_ring_buffer::RING_BUFFER_SIZE

*Constant*: `usize`



## sel4_shared_ring_buffer::RawRingBuffer

*Struct*

**Generic Parameters:**
- T

**Fields:**
- `write_index: core::sync::atomic::AtomicU32`
- `read_index: core::sync::atomic::AtomicU32`
- `descriptors: [T; 512]`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_shared_ring_buffer::RingBuffer

*Struct*

**Generic Parameters:**
- 'a
- R
- T

**Methods:**

- `fn dequeue(self: & mut Self) -> Result<Option<T>, PeerMisbehaviorError>`
- `fn force_dequeue(self: & mut Self) -> T`
- `fn enqueue_and_commit(self: & mut Self, desc: T) -> Result<Result<(), T>, PeerMisbehaviorError>`
- `fn enqueue_without_committing(self: & mut Self, desc: T) -> Result<Result<(), T>, PeerMisbehaviorError>`
- `fn enqueue(self: & mut Self, desc: T, commit: bool) -> Result<Result<(), T>, PeerMisbehaviorError>`
- `fn force_enqueue(self: & mut Self, desc: T, commit: bool)`
- `fn commit(self: & mut Self)`
- `fn new(ptr: SharedMemoryRef<'a, RawRingBuffer<T>>, initialization_strategy: InitializationStrategy) -> Self`
- `fn capacity(self: &Self) -> usize`
- `fn num_filled_slots(self: & mut Self) -> Result<usize, PeerMisbehaviorError>`
- `fn num_empty_slots(self: & mut Self) -> Result<usize, PeerMisbehaviorError>`
- `fn is_empty(self: & mut Self) -> Result<bool, PeerMisbehaviorError>`
- `fn is_full(self: & mut Self) -> Result<bool, PeerMisbehaviorError>`



## sel4_shared_ring_buffer::RingBuffers

*Struct*

**Generic Parameters:**
- 'a
- R
- F
- T

**Methods:**

- `fn notify_mut(self: & mut Self) -> U`
- `fn notify(self: &Self) -> U`
- `fn new(free: RingBuffer<'a, <R as >::FreeRole, T>, used: RingBuffer<'a, <R as >::UsedRole, T>, notify: F) -> Self`
- `fn from_ptrs_using_default_initialization_strategy_for_role(free: SharedMemoryRef<'a, RawRingBuffer<T>>, used: SharedMemoryRef<'a, RawRingBuffer<T>>, notify: F) -> Self`
- `fn free(self: &Self) -> &RingBuffer<'a, <R as >::FreeRole, T>`
- `fn used(self: &Self) -> &RingBuffer<'a, <R as >::UsedRole, T>`
- `fn free_mut(self: & mut Self) -> & mut RingBuffer<'a, <R as >::FreeRole, T>`
- `fn used_mut(self: & mut Self) -> & mut RingBuffer<'a, <R as >::UsedRole, T>`



## Module: roles



