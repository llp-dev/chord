**sel4_driver_interfaces**

# Module: sel4_driver_interfaces

## Contents

**Modules**

- [`block`](#block)
- [`net`](#net)
- [`rtc`](#rtc)
- [`serial`](#serial)
- [`timer`](#timer)

**Structs**

- [`WrappedMutex`](#wrappedmutex)
- [`WrappedRefCell`](#wrappedrefcell)

**Enums**

- [`WrappedRefCellError`](#wrappedrefcellerror)

**Traits**

- [`HandleInterrupt`](#handleinterrupt)

---

## sel4_driver_interfaces::HandleInterrupt

*Trait*

**Methods:**

- `handle_interrupt`



## sel4_driver_interfaces::WrappedMutex

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &WrappedMutex<T>) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &WrappedMutex<T>) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &WrappedMutex<T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> WrappedMutex<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> WrappedMutex<T>`



## sel4_driver_interfaces::WrappedRefCell

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> WrappedRefCell<T>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &WrappedRefCell<T>) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &WrappedRefCell<T>) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &WrappedRefCell<T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> WrappedRefCell<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_driver_interfaces::WrappedRefCellError

*Enum*

**Generic Parameters:**
- E

**Variants:**
- `Contention`
- `Other(E)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &WrappedRefCellError<E>) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &WrappedRefCellError<E>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> WrappedRefCellError<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Error**
  - `fn kind(self: &Self) -> ErrorKind`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(err: E) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &WrappedRefCellError<E>) -> bool`



## Module: block



## Module: net



## Module: rtc



## Module: serial



## Module: timer



