**rkyv > ser**

# Module: ser

## Contents

**Modules**

- [`allocator`](#allocator) - Allocators for serializers to use during serialization.
- [`sharing`](#sharing) - Shared pointer serialization.
- [`writer`](#writer) - Writing backends for serializers.

**Structs**

- [`Serializer`](#serializer) - A serializer built from composeable pieces.

---

## rkyv::ser::Serializer

*Struct*

A serializer built from composeable pieces.

**Generic Parameters:**
- W
- A
- S

**Fields:**
- `writer: W` - The writer of the serializer.
- `allocator: A` - The allocator of the serializer.
- `sharing: S` - The pointer sharing of the serializer.

**Methods:**

- `fn new(writer: W, allocator: A, sharing: S) -> Self` - Creates a new serializer from a writer, allocator, and pointer sharing.
- `fn into_raw_parts(self: Self) -> (W, A, S)` - Consumes the serializer and returns the components.
- `fn into_writer(self: Self) -> W` - Consumes the serializer and returns the writer.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Positional**
  - `fn pos(self: &Self) -> usize`
- **Allocator**
  - `fn push_alloc(self: & mut Self, layout: Layout) -> Result<NonNull<[u8]>, E>`
  - `fn pop_alloc(self: & mut Self, ptr: NonNull<u8>, layout: Layout) -> Result<(), E>`
- **Writer**
  - `fn write(self: & mut Self, bytes: &[u8]) -> Result<(), E>`
- **Sharing**
  - `fn start_sharing(self: & mut Self, address: usize) -> sharing::SharingState`
  - `fn finish_sharing(self: & mut Self, address: usize, pos: usize) -> Result<(), E>`
- **Default**
  - `fn default() -> Serializer<W, A, S>`



## Module: allocator

Allocators for serializers to use during serialization.



## Module: sharing

Shared pointer serialization.



## Module: writer

Writing backends for serializers.



