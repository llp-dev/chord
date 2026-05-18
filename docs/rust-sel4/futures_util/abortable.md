**futures_util > abortable**

# Module: abortable

## Contents

**Structs**

- [`AbortHandle`](#aborthandle) - A handle to an `Abortable` task.
- [`AbortRegistration`](#abortregistration) - A registration handle for an `Abortable` task.
- [`Abortable`](#abortable) - A future/stream which can be remotely short-circuited using an `AbortHandle`.
- [`Aborted`](#aborted) - Indicator that the `Abortable` task was aborted.

---

## futures_util::abortable::AbortHandle

*Struct*

A handle to an `Abortable` task.

**Methods:**

- `fn abort(self: &Self)` - Abort the `Abortable` stream/future associated with this handle.
- `fn is_aborted(self: &Self) -> bool` - Checks whether [`AbortHandle::abort`] was *called* on any associated
- `fn new_pair() -> (Self, AbortRegistration)` - Creates an (`AbortHandle`, `AbortRegistration`) pair which can be used

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AbortHandle`



## futures_util::abortable::AbortRegistration

*Struct*

A registration handle for an `Abortable` task.
Values of this type can be acquired from `AbortHandle::new` and are used
in calls to `Abortable::new`.

**Methods:**

- `fn handle(self: &Self) -> AbortHandle` - Create an [`AbortHandle`] from the given [`AbortRegistration`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::abortable::Abortable

*Struct*

A future/stream which can be remotely short-circuited using an `AbortHandle`.

**Generic Parameters:**
- T

**Methods:**

- `fn new(task: T, reg: AbortRegistration) -> Self` - Creates a new `Abortable` future/stream using an existing `AbortRegistration`.
- `fn is_aborted(self: &Self) -> bool` - Checks whether the task has been aborted. Note that all this

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Abortable<T>`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::abortable::Aborted

*Struct*

Indicator that the `Abortable` task was aborted.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Aborted`
- **PartialEq**
  - `fn eq(self: &Self, other: &Aborted) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



