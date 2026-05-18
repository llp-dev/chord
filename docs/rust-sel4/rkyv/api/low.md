**rkyv > api > low**

# Module: api::low

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a value from the given archived value.
- [`from_bytes_unchecked`](#from_bytes_unchecked) - Deserialize a value from the given bytes.
- [`to_bytes_in_with_alloc`](#to_bytes_in_with_alloc) - Serialize a value using the given allocator and write the bytes to the given

**Type Aliases**

- [`LowDeserializer`](#lowdeserializer) - A general-purpose deserializer suitable for environments where allocations
- [`LowSerializer`](#lowserializer) - A general-purpose serializer suitable for environments where allocations

---

## rkyv::api::low::LowDeserializer

*Type Alias*: `rancor::Strategy<(), E>`

A general-purpose deserializer suitable for environments where allocations
cannot be made.

This is part of the [low-level API](crate::api::low).



## rkyv::api::low::LowSerializer

*Type Alias*: `rancor::Strategy<crate::ser::Serializer<W, A, ()>, E>`

A general-purpose serializer suitable for environments where allocations
cannot be made.

This is part of the [low-level API](crate::api::low).



## rkyv::api::low::deserialize

*Function*

Deserialize a value from the given archived value.

This is part of the [low-level API](crate::api::low).

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    access_unchecked,
    api::low::{deserialize, to_bytes_in_with_alloc},
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
    with::InlineAsBox,
    Archive, Deserialize, Serialize,
};

let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    inner: i32,
}

let value = Example { inner: 42 };

let bytes = to_bytes_in_with_alloc::<_, _, Failure>(
    &value,
    Buffer::from(&mut *output),
    SubAllocator::new(&mut alloc),
)
.unwrap();

let archived = unsafe { access_unchecked::<ArchivedExample>(&*bytes) };
let deserialized = deserialize::<Example, Failure>(archived).unwrap();
assert_eq!(value, deserialized);
```

```rust
fn deserialize<T, E, impl Deserialize<T, LowDeserializer<E>>>(value: &impl Trait) -> Result<T, E>
```



## rkyv::api::low::from_bytes_unchecked

*Function*

Deserialize a value from the given bytes.

This function does not check that the data is valid. Use [`from_bytes`] to
validate the data instead.

This is part of the [low-level API](crate::api::low).

# Safety

The byte slice must represent a valid archived type when accessed at the
default root position. See the [module docs](crate::api) for more
information.

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    access_unchecked,
    api::low::{from_bytes_unchecked, to_bytes_in_with_alloc},
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
    with::InlineAsBox,
    Archive, Deserialize, Serialize,
};

let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    inner: i32,
}

let value = Example { inner: 42 };

let bytes = to_bytes_in_with_alloc::<_, _, Failure>(
    &value,
    Buffer::from(&mut *output),
    SubAllocator::new(&mut alloc),
)
.unwrap();

let deserialized =
    unsafe { from_bytes_unchecked::<Example, Failure>(&*bytes).unwrap() };
assert_eq!(value, deserialized);
```

```rust
fn from_bytes_unchecked<T, E>(bytes: &[u8]) -> Result<T, E>
```



## rkyv::api::low::to_bytes_in_with_alloc

*Function*

Serialize a value using the given allocator and write the bytes to the given
writer.

This is part of the [low-level API](crate::api::low).

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    access_unchecked,
    api::low::to_bytes_in_with_alloc,
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
    with::InlineAsBox,
    Archive, Serialize,
};

let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

#[derive(Archive, Serialize)]
struct Example<'a> {
    #[rkyv(with = InlineAsBox)]
    inner: &'a i32,
}

let forty_two = 42;
let value = Example { inner: &forty_two };

let bytes = to_bytes_in_with_alloc::<_, _, Failure>(
    &value,
    Buffer::from(&mut *output),
    SubAllocator::new(&mut alloc),
)
.unwrap();

let archived = unsafe { access_unchecked::<ArchivedExample<'_>>(&*bytes) };
assert_eq!(*archived.inner, 42);
```

```rust
fn to_bytes_in_with_alloc<W, A, E, impl Serialize<LowSerializer<W, A, E>>>(value: &impl Trait, writer: W, alloc: A) -> Result<W, E>
```



