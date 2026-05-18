**async_unsync > error**

# Module: error

## Contents

**Structs**

- [`SendError`](#senderror) - An error which can occur when sending a value through a closed channel.

**Enums**

- [`TryRecvError`](#tryrecverror) - An error which can occur when receiving on a closed or empty channel.
- [`TrySendError`](#trysenderror) - An error which can occur when sending a value through a closed or full

---

## async_unsync::error::SendError

*Struct*

An error which can occur when sending a value through a closed channel.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SendError<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SendError<T>`



## async_unsync::error::TryRecvError

*Enum*

An error which can occur when receiving on a closed or empty channel.

**Variants:**
- `Empty` - This **channel** is currently empty, but the **Sender**(s) have not yet
- `Disconnected` - The **channel**’s sending half has become disconnected, and there will

**Methods:**

- `fn is_empty(self: Self) -> bool` - Returns `true` if the error is [`TryRecvError::Empty`].
- `fn is_disconnected(self: Self) -> bool` - Returns `true` if the error is [`TryRecvError::Disconnected`].

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TryRecvError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TryRecvError`



## async_unsync::error::TrySendError

*Enum*

An error which can occur when sending a value through a closed or full
channel.

**Generic Parameters:**
- T

**Variants:**
- `Full(T)` - This **channel** is currently full.
- `Closed(T)` - This **channel**'s receiving half has been explicitly disconnected or

**Methods:**

- `fn is_full(self: &Self) -> bool` - Returns `true` if the error is [`TrySendError::Full`].
- `fn is_closed(self: &Self) -> bool` - Returns `true` if the error is [`TrySendError::Closed`].

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from((err, elem): (TryAcquireError, T)) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &TrySendError<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TrySendError<T>`
- **From**
  - `fn from(err: SendError<T>) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



