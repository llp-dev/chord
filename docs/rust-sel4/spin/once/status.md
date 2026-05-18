**spin > once > status**

# Module: once::status

## Contents

**Structs**

- [`AtomicStatus`](#atomicstatus)

**Enums**

- [`Status`](#status)

---

## spin::once::status::AtomicStatus

*Struct*

**Tuple Struct**: `(crate::atomic::AtomicU8)`

**Methods:**

- `fn new(status: Status) -> Self`
- `fn load(self: &Self, ordering: Ordering) -> Status`
- `fn store(self: &Self, status: Status, ordering: Ordering)`
- `fn compare_exchange(self: &Self, old: Status, new: Status, success: Ordering, failure: Ordering) -> Result<Status, Status>`
- `fn get_mut(self: & mut Self) -> & mut Status`



## spin::once::status::Status

*Enum*

**Variants:**
- `Incomplete`
- `Running`
- `Complete`
- `Panicked`

**Methods:**

- `fn new_unchecked(inner: u8) -> Self`

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Status) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Status`



