**sel4_shared_ring_buffer_block_io_types**

# Module: sel4_shared_ring_buffer_block_io_types

## Contents

**Structs**

- [`BlockIORequest`](#blockiorequest)

**Enums**

- [`BlockIORequestStatus`](#blockiorequeststatus)
- [`BlockIORequestType`](#blockiorequesttype)

---

## sel4_shared_ring_buffer_block_io_types::BlockIORequest

*Struct*

**Methods:**

- `fn new(status: BlockIORequestStatus, ty: BlockIORequestType, start_block_idx: u64, buf: Descriptor) -> Self`
- `fn status(self: &Self) -> Result<BlockIORequestStatus, TryFromPrimitiveError<BlockIORequestStatus>>`
- `fn set_status(self: & mut Self, status: BlockIORequestStatus)`
- `fn ty(self: &Self) -> Result<BlockIORequestType, TryFromPrimitiveError<BlockIORequestType>>`
- `fn set_ty(self: & mut Self, ty: BlockIORequestType)`
- `fn start_block_idx(self: &Self) -> u64`
- `fn set_start_block_idx(self: & mut Self, start_block_idx: u64)`
- `fn buf(self: &Self) -> &Descriptor`
- `fn buf_mut(self: & mut Self) -> & mut Descriptor`

**Traits:** Immutable, IntoBytes, FromBytes, FromZeros, Copy, TryFromBytes, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &BlockIORequest) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BlockIORequest`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockIORequest) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockIORequest) -> $crate::option::Option<$crate::cmp::Ordering>`



## sel4_shared_ring_buffer_block_io_types::BlockIORequestStatus

*Enum*

**Variants:**
- `Pending`
- `Ok`
- `IOError`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &BlockIORequestStatus) -> bool`
- **TryFrom**
  - `fn try_from(number: i32) -> ::core::result::Result<Self, ::num_enum::TryFromPrimitiveError<Self>>`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockIORequestStatus) -> $crate::cmp::Ordering`
- **TryFromPrimitive**
  - `fn try_from_primitive(number: <Self as >::Primitive) -> ::core::result::Result<Self, ::num_enum::TryFromPrimitiveError<Self>>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockIORequestStatus) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BlockIORequestStatus`



## sel4_shared_ring_buffer_block_io_types::BlockIORequestType

*Enum*

**Variants:**
- `Read`
- `Write`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BlockIORequestType`
- **PartialEq**
  - `fn eq(self: &Self, other: &BlockIORequestType) -> bool`
- **TryFrom**
  - `fn try_from(number: u32) -> ::core::result::Result<Self, ::num_enum::TryFromPrimitiveError<Self>>`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockIORequestType) -> $crate::cmp::Ordering`
- **TryFromPrimitive**
  - `fn try_from_primitive(number: <Self as >::Primitive) -> ::core::result::Result<Self, ::num_enum::TryFromPrimitiveError<Self>>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockIORequestType) -> $crate::option::Option<$crate::cmp::Ordering>`



