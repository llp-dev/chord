**sel4_shared_ring_buffer > roles**

# Module: roles

## Contents

**Enums**

- [`Provide`](#provide)
- [`Read`](#read)
- [`RingBufferRoleValue`](#ringbufferrolevalue)
- [`RingBuffersRoleValue`](#ringbuffersrolevalue)
- [`Use`](#use)
- [`Write`](#write)

**Traits**

- [`RingBufferRole`](#ringbufferrole)
- [`RingBuffersRole`](#ringbuffersrole)

---

## sel4_shared_ring_buffer::roles::Provide

*Enum*

**Trait Implementations:**

- **RingBuffersRole**
  - `fn default_initialization_strategy() -> InitializationStrategy`



## sel4_shared_ring_buffer::roles::Read

*Enum*

**Traits:** RingBufferRole



## sel4_shared_ring_buffer::roles::RingBufferRole

*Trait*

**Methods:**

- `ROLE`



## sel4_shared_ring_buffer::roles::RingBufferRoleValue

*Enum*

**Variants:**
- `Write`
- `Read`

**Methods:**

- `fn is_write(self: Self) -> bool`
- `fn is_read(self: Self) -> bool`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &RingBufferRoleValue) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &RingBufferRoleValue) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> RingBufferRoleValue`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &RingBufferRoleValue) -> bool`



## sel4_shared_ring_buffer::roles::RingBuffersRole

*Trait*

**Methods:**

- `FreeRole`
- `UsedRole`
- `ROLE`
- `default_initialization_strategy`



## sel4_shared_ring_buffer::roles::RingBuffersRoleValue

*Enum*

**Variants:**
- `Provide`
- `Use`

**Methods:**

- `fn is_provide(self: Self) -> bool`
- `fn is_use(self: Self) -> bool`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &RingBuffersRoleValue) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &RingBuffersRoleValue) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> RingBuffersRoleValue`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &RingBuffersRoleValue) -> bool`



## sel4_shared_ring_buffer::roles::Use

*Enum*

**Trait Implementations:**

- **RingBuffersRole**
  - `fn default_initialization_strategy() -> InitializationStrategy`



## sel4_shared_ring_buffer::roles::Write

*Enum*

**Traits:** RingBufferRole



