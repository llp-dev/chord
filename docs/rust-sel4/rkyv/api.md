**rkyv > api**

# Module: api

## Contents

**Modules**

- [`high`](#high) - APIs for environments where allocations can be made.
- [`low`](#low) - APIs for environments where allocations cannot be made.

**Functions**

- [`access_pos_unchecked`](#access_pos_unchecked) - Access a byte slice with a given root position.
- [`access_pos_unchecked_mut`](#access_pos_unchecked_mut) - Mutably access a byte slice with a given root position.
- [`access_unchecked`](#access_unchecked) - Access a byte slice.
- [`access_unchecked_mut`](#access_unchecked_mut) - Mutably access a byte slice.
- [`deserialize_using`](#deserialize_using) - Deserialize a value using the given deserializer.
- [`root_position`](#root_position) - Return the position of the root within a buffer of `length` bytes.
- [`serialize_using`](#serialize_using) - Serialize a value using the given serializer.

---

## rkyv::api::access_pos_unchecked

*Function*

Access a byte slice with a given root position.

Most of the time, the root position should be calculated using the root type
and size of the buffer. Prefer [`access_unchecked`] whenever possible.

While the root of the archived data is located at the given position, the
reachable data may be located throughout the byte slice.

This function does not check that the bytes are valid to access. Use
[`access_pos`](high::access_pos) to safely access the buffer using
validation.

# Safety

The byte slice must represent a valid archived type when accessed with the
given root position. See the [module docs](crate::api) for more information.

# Example

```
use rkyv::{
    api::{access_pos_unchecked, root_position},
    rancor::Error,
    to_bytes, Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();

let archived = unsafe {
    access_pos_unchecked::<ArchivedExample>(
        &*bytes,
        root_position::<ArchivedExample>(bytes.len()),
    )
};
assert_eq!(archived.name, "pi");
assert_eq!(archived.value, 31415926);
```

```rust
fn access_pos_unchecked<T>(bytes: &[u8], pos: usize) -> &T
```



## rkyv::api::access_pos_unchecked_mut

*Function*

Mutably access a byte slice with a given root position.

Most of the time, the root position should be calculated using the root type
and size of the buffer. Prefer [`access_unchecked_mut`] whenever possible.

While the root of the archived data is located at the given position, the
reachable data may be located throughout the byte slice.

This function does not check that the bytes are valid to access. Use
[`access_pos_mut`](high::access_pos_mut) to safely access the buffer using
validation.

The returned `Seal` restricts the mutating operations that may be safely
performed on the returned reference. See [`Seal`] for more information.

# Safety

The byte slice must represent a valid archived type when accessed with the
given root position. See the [module docs](crate::api) for more information.

# Example

```
use rkyv::{
    to_bytes, api::{root_position, access_pos_unchecked_mut}, util::Align,
    Archive, Serialize, Deserialize, munge::munge, rancor::Error,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let mut bytes = to_bytes::<Error>(&value).unwrap();
let root_pos = root_position::<ArchivedExample>(bytes.len());

let mut archived = unsafe {
    access_pos_unchecked_mut::<ArchivedExample>(&mut *bytes, root_pos)
};
assert_eq!(archived.name, "pi");
assert_eq!(archived.value, 31415926);

// Because the access is mutable, we can mutate the archived data
munge!(let ArchivedExample { mut value, .. } = archived);
assert_eq!(*value, 31415926);
*value = 12345.into();
assert_eq!(*value, 12345);
```

```rust
fn access_pos_unchecked_mut<T>(bytes: & mut [u8], pos: usize) -> crate::seal::Seal<T>
```



## rkyv::api::access_unchecked

*Function*

Access a byte slice.

This function does not check that the bytes are valid to access. Use
[`access`](high::access) to safely access the buffer using validation.

# Safety

The byte slice must represent a valid archived type when accessed at the
default root position. See the [module docs](crate::api) for more
information.

# Example

```
use rkyv::{
    access_unchecked, rancor::Error, to_bytes, Archive, Deserialize,
    Serialize,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();

let archived = unsafe { access_unchecked::<ArchivedExample>(&*bytes) };
assert_eq!(archived.name, "pi");
assert_eq!(archived.value, 31415926);
```

```rust
fn access_unchecked<T>(bytes: &[u8]) -> &T
```



## rkyv::api::access_unchecked_mut

*Function*

Mutably access a byte slice.

This function does not check that the bytes are valid to access. Use
[`access_mut`](high::access_mut) to safely access the buffer using
validation.

# Safety

The byte slice must represent a valid archived type when accessed at the
default root position. See the [module docs](crate::api) for more
information.

# Example

```
use rkyv::{
    to_bytes, access_unchecked_mut, util::Align, Archive,
    munge::munge, Serialize, Deserialize, rancor::Error,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let mut bytes = to_bytes::<Error>(&value).unwrap();

let mut archived = unsafe {
    access_unchecked_mut::<ArchivedExample>(&mut *bytes)
};
assert_eq!(archived.name, "pi");
assert_eq!(archived.value, 31415926);

// Because the access is mutable, we can mutate the archived data
munge!(let ArchivedExample { mut value, .. } = archived);
assert_eq!(*value, 31415926);
*value = 12345.into();
assert_eq!(*value, 12345);
```

```rust
fn access_unchecked_mut<T>(bytes: & mut [u8]) -> crate::seal::Seal<T>
```



## rkyv::api::deserialize_using

*Function*

Deserialize a value using the given deserializer.

Most of the time, [`deserialize`](high::deserialize) is a more ergonomic way
to deserialize an archived value.

# Example

```
use rkyv::{
    access, api::deserialize_using, de::Pool, rancor::Error, to_bytes,
    Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();
let archived = access::<ArchivedExample, Error>(&bytes).unwrap();
let deserialized =
    deserialize_using::<Example, _, Error>(archived, &mut Pool::new())
        .unwrap();
```

```rust
fn deserialize_using<T, D, E, impl Deserialize<T, Strategy<D, E>>>(value: &impl Trait, deserializer: & mut D) -> Result<T, E>
```



## Module: high

APIs for environments where allocations can be made.

These APIs have default writers, automatically manage allocators, and
support shared pointers.



## Module: low

APIs for environments where allocations cannot be made.

These APIs require user-provided writers and allocators, and do not support
shared pointers.



## rkyv::api::root_position

*Function*

Return the position of the root within a buffer of `length` bytes.

Most accessing functions have a variant which automatically calculates this
value for you. For example, prefer to call [`access_unchecked`] over
[`access_pos_unchecked`].

The root position of a buffer is calculated by subtracing the size of the
root object from the end of the buffer. If the buffer size is too small to
accomodate a root of the given type, then this function will return zero.

# Example

```
use rkyv::{api::root_position, Archive};

#[derive(Archive)]
pub struct MyData {
    inner: u32,
}

assert_eq!(size_of::<ArchivedMyData>(), 4);

// This is too small, and so returns 0
assert_eq!(root_position::<ArchivedMyData>(3), 0);
assert_eq!(root_position::<ArchivedMyData>(4), 0);
assert_eq!(root_position::<ArchivedMyData>(5), 1);
```

```rust
fn root_position<T>(size: usize) -> usize
```



## rkyv::api::serialize_using

*Function*

Serialize a value using the given serializer.

Returns the position of the serialized value.

Most of the time, [`to_bytes`](high::to_bytes) is a more ergonomic way to
serialize a value to bytes.

# Example

```
use rkyv::{
    access,
    api::serialize_using,
    rancor::Error,
    ser::{sharing::Share, Serializer},
    util::{with_arena, AlignedVec},
    Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let bytes = with_arena(|arena| {
    let mut serializer = Serializer::new(
        AlignedVec::<4>::new(),
        arena.acquire(),
        Share::new(),
    );

    let value = Example {
        name: "pi".to_string(),
        value: 31415926,
    };

    serialize_using::<_, Error>(&value, &mut serializer).unwrap();
    serializer.into_writer()
});

let archived = access::<ArchivedExample, Error>(&*bytes).unwrap();
assert_eq!(archived.value, 31415926);
```

```rust
fn serialize_using<S, E, impl SerializeUnsized<Strategy<S, E>>>(value: &impl Trait, serializer: & mut S) -> Result<usize, E>
```



