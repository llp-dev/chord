*[zerocopy](../index.md) / [ref](index.md)*

---

# Module `ref`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`def`](#def) | mod |  |
| [`Ref`](#ref) | struct |  |
| [`cast_for_sized`](#cast-for-sized) | fn | # Safety |

## Modules

- [`def`](def/index.md)

## Structs

### `Ref<B, T: ?Sized>`

```rust
struct Ref<B, T: ?Sized>(B, core::marker::PhantomData<T>);
```

A typed reference derived from a byte slice.

A `Ref<B, T>` is a reference to a `T` which is stored in a byte slice, `B`.
Unlike a native reference (`&T` or `&mut T`), `Ref<B, T>` has the same
mutability as the byte slice it was constructed from (`B`).

# Examples

`Ref` can be used to treat a sequence of bytes as a structured type, and
to read and write the fields of that type as if the byte slice reference
were simply a reference to that type.

```rust
use zerocopy::*;
use zerocopy_derive::*;

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
struct UdpHeader {
    src_port: [u8; 2],
    dst_port: [u8; 2],
    length: [u8; 2],
    checksum: [u8; 2],
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct UdpPacket {
    header: UdpHeader,
    body: [u8],
}

impl UdpPacket {
    pub fn parse<B: ByteSlice>(bytes: B) -> Option<Ref<B, UdpPacket>> {
        Ref::from_bytes(bytes).ok()
    }
}
```

#### Implementations

- <span id="ref-new-unchecked"></span>`unsafe fn new_unchecked(bytes: B) -> Ref<B, T>` — [`Ref`](def/index.md#ref)

  Constructs a new `Ref`.

  

  # Safety

  

  `bytes` dereferences (via `deref`, `deref_mut`, and `into`) to

  a byte slice which is aligned to `T`'s alignment and whose size is a

  valid size for `T`.

  

  

#### Trait Implementations

##### `impl<B: CloneableByteSlice + Clone, T: ?Sized> Clone for Ref<B, T>`

- <span id="ref-clone"></span>`fn clone(&self) -> Ref<B, T>` — [`Ref`](def/index.md#ref)

##### `impl<B: CopyableByteSlice + Copy, T: ?Sized> Copy for Ref<B, T>`

##### `impl<T, B> Debug for Ref<B, T>`

- <span id="ref-debug-fmt"></span>`fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result`

##### `impl<B, T> Deref for Ref<B, T>`

- <span id="ref-deref-type-target"></span>`type Target = T`

- <span id="ref-deref"></span>`fn deref(&self) -> &T`

##### `impl<B, T> DerefMut for Ref<B, T>`

- <span id="ref-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T, B> Display for Ref<B, T>`

- <span id="ref-display-fmt"></span>`fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T, B> Eq for Ref<B, T>`

##### `impl<T, B> Ord for Ref<B, T>`

- <span id="ref-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<T, B> PartialEq for Ref<B, T>`

- <span id="ref-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T, B> PartialOrd for Ref<B, T>`

- <span id="ref-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<T> Receiver for Ref<B, T>`

- <span id="ref-receiver-type-target"></span>`type Target = T`

##### `impl<T> ToString for Ref<B, T>`

- <span id="ref-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `cast_for_sized`

```rust
unsafe fn cast_for_sized<'a, T, A, R, S>(ptr: crate::pointer::Ptr<'a, [u8], (A, crate::pointer::invariant::Aligned, crate::pointer::invariant::Valid)>) -> crate::pointer::Ptr<'a, T, (A, crate::pointer::invariant::Unaligned, crate::pointer::invariant::Valid)>
where
    T: FromBytes + KnownLayout + ?Sized + TransmuteFromPtr<T, A, crate::pointer::invariant::Initialized, crate::pointer::invariant::Valid, crate::pointer::cast::IdCast, S>,
    A: crate::invariant::Aliasing,
    [u8]: MutationCompatible<T, A, crate::pointer::invariant::Initialized, crate::pointer::invariant::Initialized, R>
```

# Safety

`T: Sized` and `ptr`'s referent must have size `size_of::<T>()`.

