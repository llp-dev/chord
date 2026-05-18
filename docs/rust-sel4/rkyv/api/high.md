**rkyv > api > high**

# Module: api::high

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a value from the given archived value.
- [`from_bytes_unchecked`](#from_bytes_unchecked) - Deserialize a value from the given bytes.
- [`to_bytes`](#to_bytes) - Serialize a value to bytes.
- [`to_bytes_in`](#to_bytes_in) - Serialize a value and write the bytes to the given writer.
- [`to_bytes_in_with_alloc`](#to_bytes_in_with_alloc) - Serialize a value using the given allocator and write the bytes to the given
- [`to_bytes_with_alloc`](#to_bytes_with_alloc) - Serialize a value using the given allocator.

**Type Aliases**

- [`HighDeserializer`](#highdeserializer) - A high-level deserializer.
- [`HighSerializer`](#highserializer) - A high-level serializer.

---

## rkyv::api::high::HighDeserializer

*Type Alias*: `rancor::Strategy<crate::de::Pool, E>`

A high-level deserializer.

This is part of the [high-level API](crate::api::high).



## rkyv::api::high::HighSerializer

*Type Alias*: `rancor::Strategy<crate::ser::Serializer<W, A, crate::ser::sharing::Share>, E>`

A high-level serializer.

This is part of the [high-level API](crate::api::high).



## rkyv::api::high::deserialize

*Function*

Deserialize a value from the given archived value.

This is part of the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    access, deserialize, rancor::Error, to_bytes, Archive, Deserialize,
    Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();
let archived = access::<ArchivedExample, Error>(&*bytes).unwrap();
let deserialized = deserialize::<Example, Error>(archived).unwrap();

assert_eq!(deserialized, value);
```

```rust
fn deserialize<T, E, impl Deserialize<T, HighDeserializer<E>>>(value: &impl Trait) -> Result<T, E>
```



## rkyv::api::high::from_bytes_unchecked

*Function*

Deserialize a value from the given bytes.

This function does not check that the data is valid. Use [`from_bytes`] to
validate the data instead.

This is part of the [high-level API](crate::api::high).

# Safety

The byte slice must represent a valid archived type when accessed at the
default root position. See the [module docs](crate::api) for more
information.

# Example

```
use rkyv::{
    from_bytes_unchecked, rancor::Error, to_bytes, Archive, Deserialize,
    Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();
let deserialized =
    unsafe { from_bytes_unchecked::<Example, Error>(&bytes).unwrap() };

assert_eq!(deserialized, value);
```

```rust
fn from_bytes_unchecked<T, E>(bytes: &[u8]) -> Result<T, E>
```



## rkyv::api::high::to_bytes

*Function*

Serialize a value to bytes.

Returns the serialized bytes in an [`AlignedVec`].

This is part of the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    from_bytes, rancor::Error, to_bytes, Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();
let deserialized = from_bytes::<Example, Error>(&bytes).unwrap();

assert_eq!(deserialized, value);
```

```rust
fn to_bytes<E, impl for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, E>>>(value: &impl Trait) -> Result<crate::util::AlignedVec, E>
```



## rkyv::api::high::to_bytes_in

*Function*

Serialize a value and write the bytes to the given writer.

This is part of the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    api::high::to_bytes_in, from_bytes, rancor::Error, util::AlignedVec,
    Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes =
    to_bytes_in::<_, Error>(&value, AlignedVec::<8>::new()).unwrap();
let deserialized = from_bytes::<Example, Error>(&bytes).unwrap();

assert_eq!(deserialized, value);
```

```rust
fn to_bytes_in<W, E, impl for<'a> Serialize<HighSerializer<W, ArenaHandle<'a>, E>>>(value: &impl Trait, writer: W) -> Result<W, E>
```



## rkyv::api::high::to_bytes_in_with_alloc

*Function*

Serialize a value using the given allocator and write the bytes to the given
writer.

This is part of the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    api::high::to_bytes_in_with_alloc,
    from_bytes,
    rancor::Error,
    util::{with_arena, AlignedVec},
    Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

with_arena(|arena| {
    let bytes = to_bytes_in_with_alloc::<_, _, Error>(
        &value,
        AlignedVec::<8>::new(),
        arena.acquire(),
    )
    .expect("failed to serialize vec");

    let deserialized = from_bytes::<Example, Error>(&bytes)
        .expect("failed to deserialize vec");

    assert_eq!(deserialized, value);
});
```

```rust
fn to_bytes_in_with_alloc<W, A, E, impl Serialize<HighSerializer<W, A, E>>>(value: &impl Trait, writer: W, alloc: A) -> Result<W, E>
```



## rkyv::api::high::to_bytes_with_alloc

*Function*

Serialize a value using the given allocator.

This is part of the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    api::high::to_bytes_with_alloc, from_bytes, rancor::Error,
    util::with_arena, Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

with_arena(|arena| {
    let bytes =
        to_bytes_with_alloc::<_, Error>(&value, arena.acquire()).unwrap();
    let deserialized = from_bytes::<Example, Error>(&bytes).unwrap();

    assert_eq!(deserialized, value);
});
```

```rust
fn to_bytes_with_alloc<A, E, impl Serialize<HighSerializer<AlignedVec, A, E>>>(value: &impl Trait, alloc: A) -> Result<crate::util::AlignedVec, E>
```



