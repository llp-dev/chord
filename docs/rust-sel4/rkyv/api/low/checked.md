**rkyv > api > low > checked**

# Module: api::low::checked

## Contents

**Functions**

- [`access`](#access) - Access a byte slice.
- [`access_mut`](#access_mut) - Mutably accesses a byte slice.
- [`access_pos`](#access_pos) - Access a byte slice with a given root position.
- [`access_pos_mut`](#access_pos_mut) - Mutably access a byte slice with a given root position.
- [`from_bytes`](#from_bytes) - Deserialize a value from the given bytes.

**Type Aliases**

- [`LowValidator`](#lowvalidator) - A low-level validator.

---

## rkyv::api::low::checked::LowValidator

*Type Alias*: `rancor::Strategy<crate::validation::Validator<crate::validation::archive::ArchiveValidator<'a>, ()>, E>`

A low-level validator.

This is part of the [low-level API](crate::api::low).



## rkyv::api::low::checked::access

*Function*

Access a byte slice.

This is a safe alternative to [`access_unchecked`] and is part of the
[low-level API](crate::api::low).

[`access_unchecked`]: crate::api::access_unchecked

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    api::{
        low::{access, to_bytes_in_with_alloc},
        root_position,
    },
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

let archived = access::<ArchivedExample<'_>, Failure>(&*bytes).unwrap();
assert_eq!(*archived.inner, 42);
```

```rust
fn access<T, E>(bytes: &[u8]) -> Result<&T, E>
```



## rkyv::api::low::checked::access_mut

*Function*

Mutably accesses a byte slice.

This is a safe alternative to [`access_unchecked_mut`] and is part of the
[low-level API](crate::api::low).

[`access_unchecked_mut`]: crate::api::access_unchecked_mut

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    api::low::{to_bytes_in_with_alloc, access_mut},
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
    with::InlineAsBox,
    Archive, Serialize,
    munge::munge,
};

let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

#[derive(Archive, Serialize)]
struct Example {
    inner: i32,
}

let value = Example { inner: 42 };

let mut bytes = to_bytes_in_with_alloc::<_, _, Failure>(
    &value,
    Buffer::from(&mut *output),
    SubAllocator::new(&mut alloc),
)
.unwrap();

let mut archived = access_mut::<ArchivedExample, Failure>(
    &mut *bytes,
).unwrap();

// Because the access is mutable, we can mutate the archived data
munge!(let ArchivedExample { mut inner, .. } = archived);
assert_eq!(*inner, 42);
*inner = 12345.into();
assert_eq!(*inner, 12345);
```

```rust
fn access_mut<T, E>(bytes: & mut [u8]) -> Result<crate::seal::Seal<T>, E>
```



## rkyv::api::low::checked::access_pos

*Function*

Access a byte slice with a given root position.

This is a safe alternative to [`access_pos_unchecked`] and is part of the
[low-level API](crate::api::low).

[`access_pos_unchecked`]: crate::api::access_pos_unchecked

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    api::{
        low::{access_pos, to_bytes_in_with_alloc},
        root_position,
    },
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

let archived = access_pos::<ArchivedExample<'_>, Failure>(
    &*bytes,
    root_position::<ArchivedExample<'_>>(bytes.len()),
)
.unwrap();
assert_eq!(*archived.inner, 42);
```

```rust
fn access_pos<T, E>(bytes: &[u8], pos: usize) -> Result<&T, E>
```



## rkyv::api::low::checked::access_pos_mut

*Function*

Mutably access a byte slice with a given root position.

This is a safe alternative to [`access_pos_unchecked_mut`] and is part of
the [low-level API](crate::api::low).

[`access_pos_unchecked_mut`]: crate::api::access_pos_unchecked_mut

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    api::{root_position, low::{to_bytes_in_with_alloc, access_pos_mut}},
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
    with::InlineAsBox,
    Archive, Serialize,
    munge::munge,
};

let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

#[derive(Archive, Serialize)]
struct Example {
    inner: i32,
}

let value = Example { inner: 42 };

let mut bytes = to_bytes_in_with_alloc::<_, _, Failure>(
    &value,
    Buffer::from(&mut *output),
    SubAllocator::new(&mut alloc),
)
.unwrap();

let root_pos = root_position::<ArchivedExample>(bytes.len());
let mut archived = access_pos_mut::<ArchivedExample, Failure>(
    &mut *bytes,
    root_pos,
).unwrap();

// Because the access is mutable, we can mutate the archived data
munge!(let ArchivedExample { mut inner, .. } = archived);
assert_eq!(*inner, 42);
*inner = 12345.into();
assert_eq!(*inner, 12345);
```

```rust
fn access_pos_mut<T, E>(bytes: & mut [u8], pos: usize) -> Result<crate::seal::Seal<T>, E>
```



## rkyv::api::low::checked::from_bytes

*Function*

Deserialize a value from the given bytes.

This is a safe alternative to [`from_bytes_unchecked`] and is part of the
[low-level API](crate::api::low).

[`from_bytes_unchecked`]: crate::api::low::from_bytes_unchecked

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    api::low::{from_bytes, to_bytes_in_with_alloc},
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
    Archive, Deserialize, Serialize,
};

let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug)]
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

let deserialized = from_bytes::<Example, Failure>(&*bytes).unwrap();
assert_eq!(value, deserialized);
```

```rust
fn from_bytes<T, E>(bytes: &[u8]) -> Result<T, E>
```



