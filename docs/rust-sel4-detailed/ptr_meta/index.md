# Crate `ptr_meta`

A radioactive stabilization of the [`ptr_meta` RFC][rfc].

This crate provides the [`Pointee`](#pointee) trait, [`from_raw_parts`](#from-raw-parts) and
[`to_raw_parts`](#to-raw-parts) functions, and proc macros for deriving `Pointee` for
structs and implementing `Pointee` for trait objects.

# Usage

Raw pointers can be decomposed into the data address and metadata components
with [`to_raw_parts`](#to-raw-parts) or [`to_raw_parts_mut`](#to-raw-parts-mut).

Alternatively, metadata alone can be extracted with the [`metadata`](#metadata)
function. Although [`metadata`](#metadata) accepts pointers, references can be passed
and will be implicitly coerced.

A pointer can be created from its address and metadata with
[`from_raw_parts`](#from-raw-parts) or [`from_raw_parts_mut`](#from-raw-parts-mut).

## Provided impls

`ptr_meta` provides inherent implementations for many builtin types:

- All [`Sized`](../zerocopy_derive/index.md) types implement [`Pointee`](#pointee) via a blanket implementation.
- `slice`, `str`, and `CStr`
- `OsStr` (requires `std`)
- `dyn Any`, optionally with `+ Send` and/or `+ Sync`
- `dyn Error`, optionally with `+ Send` and/or `+ Sync`

## Structs with trailing DSTs

You can derive [`Pointee`](#pointee) for structs with trailing DSTs:

```rust
use ptr_meta::Pointee;

#[derive(Pointee)]
struct Block<H, T> {
    header: H,
    elements: [T],
}
```

Note that the last field is required to be a DST. Structs with a generic
type as the last field may have conflicting blanket implementations, as the
generic type may be `Sized`. A collection of specific implementations may be
required in these cases, with the generic parameter set (for example) a
slice, `str`, or specific trait object.

## Trait objects

You can generate [`Pointee`](#pointee) implementations for trait objects:

```rust
use ptr_meta::pointee;

// Generates Pointee for dyn Stringy
#[ptr_meta::pointee]
trait Stringy {
    fn as_string(&self) -> String;
}
```

Note that this will not produce implementations for `Trait + Send + Sync`.

## Features

- `derive`: Re-exports the macros from `ptr_meta_derive`. Enabled by
  default.
- `std`: Enables additional impls for `std` types. Enabled by default.

## Example
```rust
// Get the associated metadata for pointers
let str = "hello world";
assert_eq!(ptr_meta::metadata(str), str.len());

let slice = &[1, 2, 3, 4, 5] as &[i32];
assert_eq!(ptr_meta::metadata(slice), slice.len());

// Make your own wide pointers from data pointers and metadata
let bytes = [b'h', b'e', b'l', b'l', b'o'];
let ptr = ptr_meta::from_raw_parts::<str>(bytes.as_ptr().cast(), 5);
println!("{} world!", unsafe { &*ptr }); // prints "hello world!"

// Derive Pointee on your own types
#[derive(ptr_meta::Pointee)]
#[repr(transparent)]
struct CoolStr {
    inner: str,
}

impl CoolStr {
    fn print_cool(&self) {
        println!("😎 {} 😎", &self.inner);
    }
}

let ptr = ptr_meta::from_raw_parts::<CoolStr>(bytes.as_ptr().cast(), 5);
let cool = unsafe { &*ptr };
cool.print_cool(); // prints "😎 hello 😎"

// Implement Pointee for trait objects
#[ptr_meta::pointee]
trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("i32: {self}");
    }
}

let i32_vtable = ptr_meta::metadata(&0i32 as &dyn Printable);
let one_hundred = 100i32;
let printable = ptr_meta::from_raw_parts::<dyn Printable>(
    (&one_hundred as *const i32).cast(),
    i32_vtable,
);
unsafe {
    (*printable).print(); // prints "i32: 100"
}
```

## Contents

- [Modules](#modules)
  - [`impls`](#impls)
  - [`Pointee`](#pointee)
- [Structs](#structs)
  - [`PtrComponents`](#ptrcomponents)
  - [`DynMetadata`](#dynmetadata)
  - [`VTable`](#vtable)
- [Traits](#traits)
  - [`Pointee`](#pointee)
- [Functions](#functions)
  - [`metadata`](#metadata)
  - [`to_raw_parts`](#to-raw-parts)
  - [`to_raw_parts_mut`](#to-raw-parts-mut)
  - [`from_raw_parts`](#from-raw-parts)
  - [`from_raw_parts_mut`](#from-raw-parts-mut)
- [Type Aliases](#type-aliases)
  - [`pointee`](#pointee)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`impls`](#impls) | mod |  |
| [`Pointee`](#pointee) | mod |  |
| [`PtrComponents`](#ptrcomponents) | struct |  |
| [`DynMetadata`](#dynmetadata) | struct | The metadata for a trait object. |
| [`VTable`](#vtable) | struct |  |
| [`Pointee`](#pointee) | trait | A trait which associates pointer metadata with a pointee type. |
| [`metadata`](#metadata) | fn | Returns the metadata of the given pointer. |
| [`to_raw_parts`](#to-raw-parts) | fn | Returns the data address and metadata of the given pointer. |
| [`to_raw_parts_mut`](#to-raw-parts-mut) | fn | Returns the mutable data address and metadata of the given pointer. |
| [`from_raw_parts`](#from-raw-parts) | fn | Returns a raw pointer with the given data address and metadata. |
| [`from_raw_parts_mut`](#from-raw-parts-mut) | fn | Returns a mutable raw pointer with the given data address and metadata. |
| [`pointee`](#pointee) | type |  |

## Modules

- [`impls`](impls/index.md)
- [`Pointee`](Pointee/index.md)

## Structs

### `PtrComponents<T: Pointee + ?Sized>`

```rust
struct PtrComponents<T: Pointee + ?Sized> {
    data_address: *const (),
    metadata: <T as Pointee>::Metadata,
}
```

#### Trait Implementations

##### `impl<T: Pointee + ?Sized> Clone for PtrComponents<T>`

- <span id="ptrcomponents-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: Pointee + ?Sized> Copy for PtrComponents<T>`

##### `impl<T> Pointee for PtrComponents<T>`

- <span id="ptrcomponents-pointee-type-metadata"></span>`type Metadata = ()`

### `DynMetadata<Dyn: ?Sized>`

```rust
struct DynMetadata<Dyn: ?Sized> {
    vtable_ptr: &'static VTable,
    phantom: core::marker::PhantomData<Dyn>,
}
```

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

#### Implementations

- <span id="dynmetadata-size-of"></span>`fn size_of(self) -> usize`

  Returns the size of the type associated with this metadata.

- <span id="dynmetadata-align-of"></span>`fn align_of(self) -> usize`

  Returns the alignment of the type associated with this metadata.

- <span id="dynmetadata-layout"></span>`fn layout(self) -> core::alloc::Layout`

  Returns the layout of the type associated with this metadata.

#### Trait Implementations

##### `impl<Dyn: ?Sized> Clone for DynMetadata<Dyn>`

- <span id="dynmetadata-clone"></span>`fn clone(&self) -> Self`

##### `impl<Dyn: ?Sized> Copy for DynMetadata<Dyn>`

##### `impl<Dyn: ?Sized> Debug for DynMetadata<Dyn>`

- <span id="dynmetadata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Dyn: ?Sized> Eq for DynMetadata<Dyn>`

##### `impl<Dyn: ?Sized> Hash for DynMetadata<Dyn>`

- <span id="dynmetadata-hash"></span>`fn hash<H: Hasher>(&self, hasher: &mut H)`

##### `impl<Dyn: ?Sized> Ord for DynMetadata<Dyn>`

- <span id="dynmetadata-ord-cmp"></span>`fn cmp(&self, other: &Self) -> core::cmp::Ordering`

##### `impl<Dyn: ?Sized> PartialEq for DynMetadata<Dyn>`

- <span id="dynmetadata-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<Dyn: ?Sized> PartialOrd for DynMetadata<Dyn>`

- <span id="dynmetadata-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl Pointee for DynMetadata<Dyn>`

- <span id="dynmetadata-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl<Dyn: ?Sized> Send for DynMetadata<Dyn>`

##### `impl<Dyn: ?Sized> Sync for DynMetadata<Dyn>`

##### `impl<Dyn: ?Sized> Unpin for DynMetadata<Dyn>`

### `VTable`

```rust
struct VTable;
```

#### Trait Implementations

##### `impl Pointee for VTable`

- <span id="vtable-pointee-type-metadata"></span>`type Metadata = ()`

## Traits

### `Pointee`

```rust
trait Pointee { ... }
```

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
* Trait objects like `dyn SomeTrait` have [`DynMetadata`](#dynmetadata) pointer metadata,
  which point to the trait objects' virtual method tables.
* Structs with a trailing DST have the same metadata as the trailing DST.

In the future, Rust may add new kinds of types which have different pointer
metadata.

# Safety

The associated `Metadata` type must be the pointer metadata type for the
implementing type.

#### Associated Types

- `type Metadata: 6`

#### Implementors

- `T`
- `[T]`
- `core::ffi::CStr`
- `dyn Any + Send + Sync`
- `dyn Any + Send`
- `dyn Any + Sync`
- `dyn Any`
- `dyn Error + Send + Sync`
- `dyn Error + Send`
- `dyn Error + Sync`
- `dyn Error`
- `std::ffi::OsStr`
- `str`

## Functions

### `metadata`

```rust
const fn metadata<T: Pointee + ?Sized>(ptr: *const T) -> <T as Pointee>::Metadata
```

Returns the metadata of the given pointer.

`*mut T`, `&T`, and `&mut T` can all be passed directly to this function as
they implicitly coerce to `*const T`.

# Example

```rust
// String slices have pointer metadata equal to their size in bytes
assert_eq!(ptr_meta::metadata("foo"), 3_usize);
```

### `to_raw_parts`

```rust
const fn to_raw_parts<T: Pointee + ?Sized>(ptr: *const T) -> (*const (), <T as Pointee>::Metadata)
```

Returns the data address and metadata of the given pointer.

`*mut T`, `&T`, and `&mut T` can all be passed directly to this function as
they implicitly coerce to `*const T`.

# Example

```rust
let (data_address, metadata) = ptr_meta::to_raw_parts("foo");
assert_ne!(data_address, core::ptr::null());
assert_eq!(metadata, 3);
```

### `to_raw_parts_mut`

```rust
const fn to_raw_parts_mut<T: Pointee + ?Sized>(ptr: *mut T) -> (*mut (), <T as Pointee>::Metadata)
```

Returns the mutable data address and metadata of the given pointer.

See [`to_raw_parts`](#to-raw-parts) for more details.

### `from_raw_parts`

```rust
const fn from_raw_parts<T: Pointee + ?Sized>(data_address: *const (), metadata: <T as Pointee>::Metadata) -> *const T
```

Returns a raw pointer with the given data address and metadata.

This function is safe, but the returned pointer is not necessarily safe to
dereference. For slices, see the documentation of `slice::from_raw_parts`
for safety requirements. For trait objects, the metadata must come from a
a trait object with the same underlying type.


### `from_raw_parts_mut`

```rust
const fn from_raw_parts_mut<T: Pointee + ?Sized>(data_address: *mut (), metadata: <T as Pointee>::Metadata) -> *mut T
```

Returns a mutable raw pointer with the given data address and metadata.

See [`from_raw_parts`](#from-raw-parts) for more details.

## Type Aliases

### `pointee<T>`

```rust
type pointee<T> = smallvec::IntoIter<[T; 16]>;
```

