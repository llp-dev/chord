**sel4_async_block_io > constant_block_sizes**

# Module: constant_block_sizes

## Contents

**Structs**

- [`BlockSize1024`](#blocksize1024)
- [`BlockSize2048`](#blocksize2048)
- [`BlockSize4096`](#blocksize4096)
- [`BlockSize512`](#blocksize512)
- [`BlockSize8192`](#blocksize8192)

---

## sel4_async_block_io::constant_block_sizes::BlockSize1024

*Struct*

**Unit Struct**

**Traits:** ConstantBlockSize, Eq, Copy, HasNextBlockSize, HasPrevBlockSize

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **BlockSize**
  - `fn bytes(self: &Self) -> usize`
  - `fn zeroed_block(self: &Self) -> <Self as >::Block`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockSize1024) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockSize1024) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &BlockSize1024) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BlockSize1024`



## sel4_async_block_io::constant_block_sizes::BlockSize2048

*Struct*

**Unit Struct**

**Traits:** HasPrevBlockSize, ConstantBlockSize, Eq, HasNextBlockSize, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **BlockSize**
  - `fn bytes(self: &Self) -> usize`
  - `fn zeroed_block(self: &Self) -> <Self as >::Block`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockSize2048) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockSize2048) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &BlockSize2048) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BlockSize2048`



## sel4_async_block_io::constant_block_sizes::BlockSize4096

*Struct*

**Unit Struct**

**Traits:** Eq, HasNextBlockSize, Copy, HasPrevBlockSize, ConstantBlockSize

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **BlockSize**
  - `fn bytes(self: &Self) -> usize`
  - `fn zeroed_block(self: &Self) -> <Self as >::Block`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockSize4096) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockSize4096) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &BlockSize4096) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BlockSize4096`



## sel4_async_block_io::constant_block_sizes::BlockSize512

*Struct*

**Unit Struct**

**Traits:** Copy, HasNextBlockSize, ConstantBlockSize, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockSize512) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockSize512) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &BlockSize512) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BlockSize512`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **BlockSize**
  - `fn bytes(self: &Self) -> usize`
  - `fn zeroed_block(self: &Self) -> <Self as >::Block`



## sel4_async_block_io::constant_block_sizes::BlockSize8192

*Struct*

**Unit Struct**

**Traits:** Copy, HasPrevBlockSize, ConstantBlockSize, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockSize8192) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockSize8192) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &BlockSize8192) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BlockSize8192`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **BlockSize**
  - `fn bytes(self: &Self) -> usize`
  - `fn zeroed_block(self: &Self) -> <Self as >::Block`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



