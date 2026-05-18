**sel4_shared_ring_buffer_block_io > errors**

# Module: errors

## Contents

**Structs**

- [`IOError`](#ioerror)

**Enums**

- [`Error`](#error)
- [`ErrorOrUserError`](#errororusererror)
- [`PeerMisbehaviorError`](#peermisbehaviorerror)
- [`UserError`](#usererror)

---

## sel4_shared_ring_buffer_block_io::errors::Error

*Enum*

**Variants:**
- `IOError(IOError)`
- `BounceBufferAllocationError`
- `PeerMisbehaviorError(PeerMisbehaviorError)`

**Trait Implementations:**

- **From**
  - `fn from(err: SharedRingBuffersPeerMisbehaviorError) -> Self`
- **From**
  - `fn from(err: PeerMisbehaviorError) -> Self`
- **From**
  - `fn from(err: IOError) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_shared_ring_buffer_block_io::errors::ErrorOrUserError

*Enum*

**Variants:**
- `Error(Error)`
- `UserError(UserError)`

**Methods:**

- `fn unwrap_error(self: Self) -> Error`

**Trait Implementations:**

- **From**
  - `fn from(err: IOError) -> Self`
- **From**
  - `fn from(err: UserError) -> Self`
- **From**
  - `fn from(err: Error) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> ErrorOrUserError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(err: SlotTrackerError) -> Self`
- **From**
  - `fn from(err: SharedRingBuffersPeerMisbehaviorError) -> Self`
- **From**
  - `fn from(err: PeerMisbehaviorError) -> Self`



## sel4_shared_ring_buffer_block_io::errors::IOError

*Struct*

**Unit Struct**

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> IOError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_shared_ring_buffer_block_io::errors::PeerMisbehaviorError

*Enum*

**Variants:**
- `InvalidDescriptor`
- `DescriptorMismatch`
- `OutOfBoundsCookie`
- `StateMismatch`
- `SharedRingBuffersPeerMisbehaviorError(sel4_shared_ring_buffer::PeerMisbehaviorError)`

**Trait Implementations:**

- **From**
  - `fn from(err: SharedRingBuffersPeerMisbehaviorError) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> PeerMisbehaviorError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_shared_ring_buffer_block_io::errors::UserError

*Enum*

**Variants:**
- `InvalidRequestIndex`
- `RequestStateMismatch`
- `TooManyOutstandingRequests`

**Trait Implementations:**

- **From**
  - `fn from(err: SlotTrackerError) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> UserError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



