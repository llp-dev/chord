**sel4_async_block_io > operation**

# Module: operation

## Contents

**Enums**

- [`Operation`](#operation)
- [`OperationType`](#operationtype)

---

## sel4_async_block_io::operation::Operation

*Enum*

**Generic Parameters:**
- 'a
- A

**Variants:**
- `Read{ buf: &'a  mut [u8], witness: <A as >::ReadWitness }`
- `Write{ buf: &'a [u8], witness: <A as >::WriteWitness }`

**Methods:**

- `fn write(buf: &'a [u8]) -> Self`
- `fn read(buf: &'a  mut [u8]) -> Self`
- `fn as_write(self: &'a Self) -> &'a [u8]`
- `fn as_read(self: &'a  mut Self) -> &'a  mut [u8]`
- `fn with_read_access<A1>(self: &'a  mut Self) -> Operation<'a, A1>`
- `fn with_write_access<A1>(self: &'a  mut Self) -> Operation<'a, A1>`
- `fn len(self: &Self) -> usize`
- `fn is_empty(self: &Self) -> bool`
- `fn ty(self: &Self) -> OperationType`
- `fn index(self: &'a  mut Self, index: Range<usize>) -> Self`
- `fn split_at(self: &'a  mut Self, mid: usize) -> (Self, Self)`
- `fn chunks(self: &'a  mut Self, chunk_size: usize) -> impl Trait`



## sel4_async_block_io::operation::OperationType

*Enum*

**Variants:**
- `Read`
- `Write`

**Methods:**

- `fn is_read(self: Self) -> bool`
- `fn is_write(self: Self) -> bool`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &OperationType) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &OperationType) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &OperationType) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> OperationType`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



