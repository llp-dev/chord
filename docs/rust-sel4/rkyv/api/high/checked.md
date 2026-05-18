**rkyv > api > high > checked**

# Module: api::high::checked

## Contents

**Functions**

- [`access`](#access) - Access a byte slice.
- [`access_mut`](#access_mut) - Mutably access a byte slice.
- [`access_pos`](#access_pos) - Access a byte slice with a given root position.
- [`access_pos_mut`](#access_pos_mut) - Mutably access a byte slice with a given root position.
- [`from_bytes`](#from_bytes) - Deserialize a value from the given bytes.

**Type Aliases**

- [`HighValidator`](#highvalidator) - A high-level validator.

---

## rkyv::api::high::checked::HighValidator

*Type Alias*: `rancor::Strategy<crate::validation::Validator<crate::validation::archive::ArchiveValidator<'a>, crate::validation::shared::SharedValidator>, E>`

A high-level validator.

This is part of the [high-level API](crate::api::high).



## rkyv::api::high::checked::access

*Function*

Access a byte slice.

This is a safe alternative to [`access_unchecked`] and is part of the
[high-level API](crate::api::high).

[`access_unchecked`]: crate::access_unchecked

# Example

```
use rkyv::{
    access, bytecheck::CheckBytes, rancor::Error, to_bytes, Archive,
    Archived, Serialize,
};

#[derive(Archive, Serialize)]
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

assert_eq!(archived.name, "pi");
assert_eq!(archived.value, 31415926);
```

```rust
fn access<T, E>(bytes: &[u8]) -> Result<&T, E>
```



## rkyv::api::high::checked::access_mut

*Function*

Mutably access a byte slice.

This is a safe alternative to [`access_unchecked_mut`] and is part of the
[high-level API](crate::api::high).

[`access_unchecked_mut`]: crate::api::access_unchecked_mut

# Example

```
use rkyv::{
    access_mut,
    bytecheck::CheckBytes,
    rancor::Error, munge::munge,
    to_bytes, Archive, Archived, Serialize,
};

#[derive(Archive, Serialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let mut bytes = to_bytes::<Error>(&value).unwrap();

let mut archived = access_mut::<ArchivedExample, Error>(&mut bytes)
    .unwrap();

// Because the access is mutable, we can mutate the archived data
munge!(let ArchivedExample { mut value, .. } = archived);
assert_eq!(*value, 31415926);
*value = 12345.into();
assert_eq!(*value, 12345);
```

```rust
fn access_mut<T, E>(bytes: & mut [u8]) -> Result<crate::seal::Seal<T>, E>
```



## rkyv::api::high::checked::access_pos

*Function*

Access a byte slice with a given root position.

This is a safe alternative to [`access_pos_unchecked`] and is part of the
[high-level API](crate::api::high).

[`access_pos_unchecked`]: crate::api::access_pos_unchecked

# Example

```
use rkyv::{
    api::{high::access_pos, root_position},
    bytecheck::CheckBytes,
    rancor::Error,
    to_bytes, Archive, Archived, Serialize,
};

#[derive(Archive, Serialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();
let archived = access_pos::<ArchivedExample, Error>(
    &bytes,
    root_position::<ArchivedExample>(bytes.len()),
)
.unwrap();

assert_eq!(archived.name, "pi");
assert_eq!(archived.value, 31415926);
```

```rust
fn access_pos<T, E>(bytes: &[u8], pos: usize) -> Result<&T, E>
```



## rkyv::api::high::checked::access_pos_mut

*Function*

Mutably access a byte slice with a given root position.

This is a safe alternative to [`access_pos_unchecked_mut`] and is part of
the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    api::{high::access_pos_mut, root_position},
    bytecheck::CheckBytes,
    rancor::Error, munge::munge,
    to_bytes, Archive, Archived, Serialize,
};

#[derive(Archive, Serialize)]
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

let mut archived =
    access_pos_mut::<ArchivedExample, Error>(&mut bytes, root_pos).unwrap();

// Because the access is mutable, we can mutate the archived data
munge!(let ArchivedExample { mut value, .. } = archived);
assert_eq!(*value, 31415926);
*value = 12345.into();
assert_eq!(*value, 12345);
```

```rust
fn access_pos_mut<T, E>(bytes: & mut [u8], pos: usize) -> Result<crate::seal::Seal<T>, E>
```



## rkyv::api::high::checked::from_bytes

*Function*

Deserialize a value from the given bytes.

This is a safe alternative to [`from_bytes_unchecked`] and is part of the
[high-level API](crate::api::high).

[`from_bytes_unchecked`]: crate::api::high::from_bytes_unchecked

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
fn from_bytes<T, E>(bytes: &[u8]) -> Result<T, E>
```



