**ptr_meta**

# Module: ptr_meta

## Contents

**Structs**

- [`DynMetadata`](#dynmetadata) - The metadata for a trait object.

**Functions**

- [`from_raw_parts`](#from_raw_parts) - Returns a raw pointer with the given data address and metadata.
- [`from_raw_parts_mut`](#from_raw_parts_mut) - Returns a mutable raw pointer with the given data address and metadata.
- [`metadata`](#metadata) - Returns the metadata of the given pointer.
- [`to_raw_parts`](#to_raw_parts) - Returns the data address and metadata of the given pointer.
- [`to_raw_parts_mut`](#to_raw_parts_mut) - Returns the mutable data address and metadata of the given pointer.

**Traits**

- [`Pointee`](#pointee) - A trait which associates pointer metadata with a pointee type.

---

## ptr_meta::DynMetadata

*Struct*

The metadata for a trait object.

This struct wraps a pointer to a vtable (virtual method table) which
contains all of the necessary information to manipulate the concrete type
stored inside of the trait object:

* The size and alignment of the concrete type
* A function pointer to the type's `drop_in_place` impl
* Function pointers for each method in the concrete type's trait
  implementation

Providing a type argument that is not a `dyn` trait object is possible, but
does not correspond with a meaningful type.

**Generic Parameters:**
- Dyn

**Methods:**

- `fn size_of(self: Self) -> usize` - Returns the size of the type associated with this metadata.
- `fn align_of(self: Self) -> usize` - Returns the alignment of the type associated with this metadata.
- `fn layout(self: Self) -> core::alloc::Layout` - Returns the layout of the type associated with this metadata.

**Traits:** Eq, Copy, Sync, Send

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<H>(self: &Self, hasher: & mut H)`



## ptr_meta::Pointee

*Trait*

A trait which associates pointer metadata with a pointee type.

# Pointer metadata

Pointers and references can be thought of as having two parts: a data
address and some extra "pointer metadata".

Pointers to [statically-sized types](`Sized`) and `extern` types are
"narrow": their pointer metadata is `()`.

Pointers to [dynamically-sized types][dst] are "wide": they have pointer
metadata with a non-zero size. There are four classes of dynamically-sized
types currently available:

* `str`s have `usize` pointer metadata equal to the length of the string
  slice in bytes.
* Slices like `[i32]` have `usize` pointer metadata equal to the length of
  the slice in items.
* Trait objects like `dyn SomeTrait` have [`DynMetadata`] pointer metadata,
  which point to the trait objects' virtual method tables.
* Structs with a trailing DST have the same metadata as the trailing DST.

In the future, Rust may add new kinds of types which have different pointer
metadata.

[dst]: https://doc.rust-lang.org/reference/dynamically-sized-types.html

# Safety

The associated `Metadata` type must be the pointer metadata type for the
implementing type.

**Methods:**

- `Metadata`: The metadata type for pointers and references to this type.



## ptr_meta::from_raw_parts

*Function*

Returns a raw pointer with the given data address and metadata.

This function is safe, but the returned pointer is not necessarily safe to
dereference. For slices, see the documentation of [`slice::from_raw_parts`]
for safety requirements. For trait objects, the metadata must come from a
a trait object with the same underlying type.

[`slice::from_raw_parts`]: core::slice::from_raw_parts

```rust
fn from_raw_parts<T>(data_address: *const (), metadata: <T as Pointee>::Metadata) -> *const T
```



## ptr_meta::from_raw_parts_mut

*Function*

Returns a mutable raw pointer with the given data address and metadata.

See [`from_raw_parts`] for more details.

```rust
fn from_raw_parts_mut<T>(data_address: *mut (), metadata: <T as Pointee>::Metadata) -> *mut T
```



## ptr_meta::metadata

*Function*

Returns the metadata of the given pointer.

`*mut T`, `&T`, and `&mut T` can all be passed directly to this function as
they implicitly coerce to `*const T`.

# Example

```
// String slices have pointer metadata equal to their size in bytes
assert_eq!(ptr_meta::metadata("foo"), 3_usize);
```

```rust
fn metadata<T>(ptr: *const T) -> <T as Pointee>::Metadata
```



## ptr_meta::to_raw_parts

*Function*

Returns the data address and metadata of the given pointer.

`*mut T`, `&T`, and `&mut T` can all be passed directly to this function as
they implicitly coerce to `*const T`.

# Example

```
let (data_address, metadata) = ptr_meta::to_raw_parts("foo");
assert_ne!(data_address, core::ptr::null());
assert_eq!(metadata, 3);
```

```rust
fn to_raw_parts<T>(ptr: *const T) -> (*const (), <T as Pointee>::Metadata)
```



## ptr_meta::to_raw_parts_mut

*Function*

Returns the mutable data address and metadata of the given pointer.

See [`to_raw_parts`] for more details.

```rust
fn to_raw_parts_mut<T>(ptr: *mut T) -> (*mut (), <T as Pointee>::Metadata)
```



