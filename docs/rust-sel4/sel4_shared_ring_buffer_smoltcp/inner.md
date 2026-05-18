**sel4_shared_ring_buffer_smoltcp > inner**

# Module: inner

## Contents

**Enums**

- [`Error`](#error)
- [`PeerMisbehaviorError`](#peermisbehaviorerror)

---

## sel4_shared_ring_buffer_smoltcp::inner::Error

*Enum*

**Variants:**
- `BounceBufferAllocationError`
- `PeerMisbehaviorError(PeerMisbehaviorError)`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(err: SharedRingBuffersPeerMisbehaviorError) -> Self`
- **From**
  - `fn from(err: PeerMisbehaviorError) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Error`



## sel4_shared_ring_buffer_smoltcp::inner::PeerMisbehaviorError

*Enum*

**Variants:**
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



