**rkyv > de > pooling**

# Module: de::pooling

## Contents

**Structs**

- [`ErasedPtr`](#erasedptr) - A type-erased pointer.

**Unions**

- [`Metadata`](#metadata) - Type-erased pointer metadata.

**Enums**

- [`PoolingState`](#poolingstate) - The result of starting to deserialize a shared pointer.

**Traits**

- [`FromMetadata`](#frommetadata) - A type which can be extracted from `Metadata`.
- [`Pooling`](#pooling) - A shared pointer deserialization strategy.
- [`PoolingExt`](#poolingext) - Helper methods for [`Pooling`].
- [`SharedPointer`](#sharedpointer) - A deserializable shared pointer type.

---

## rkyv::de::pooling::ErasedPtr

*Struct*

A type-erased pointer.

**Methods:**

- `fn new<T>(ptr: NonNull<T>) -> Self` - Returns an erased pointer corresponding to the given pointer.
- `fn data_address(self: &Self) -> *mut ()` - Returns the data address corresponding to this erased pointer.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ErasedPtr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rkyv::de::pooling::FromMetadata

*Trait*

A type which can be extracted from `Metadata`.

**Methods:**

- `from_metadata`: Extracts this type from [`Metadata`].



## rkyv::de::pooling::Metadata

*Union*

Type-erased pointer metadata.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Metadata`
- **From**
  - `fn from(value: ()) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(value: usize) -> Self`
- **From**
  - `fn from(value: DynMetadata<T>) -> Self`



## rkyv::de::pooling::Pooling

*Trait*

A shared pointer deserialization strategy.

This trait is required to deserialize `Rc` and `Arc`.

**Methods:**

- `start_pooling`: Starts pooling the value associated with the given address.
- `finish_pooling`: Finishes pooling the value associated with the given address.



## rkyv::de::pooling::PoolingExt

*Trait*

Helper methods for [`Pooling`].

**Methods:**

- `deserialize_shared`: Checks whether the given reference has been deserialized and either uses



## rkyv::de::pooling::PoolingState

*Enum*

The result of starting to deserialize a shared pointer.

**Variants:**
- `Started` - The caller started pooling this value. They should proceed to
- `Pending` - Another caller started pooling this value, but has not finished yet.
- `Finished(ErasedPtr)` - This value has already been pooled. The caller should use the returned



## rkyv::de::pooling::SharedPointer

*Trait*

A deserializable shared pointer type.

# Safety

`alloc` and `from_value` must return pointers which are non-null, writeable,
and properly aligned for `T`.

**Methods:**

- `alloc`: Allocates space for a value with the given metadata.
- `from_value`: Creates a new `Self` from a pointer to a valid `T`.
- `drop`: Drops a pointer created by `from_value`.



