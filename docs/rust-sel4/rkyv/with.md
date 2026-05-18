**rkyv > with**

# Module: with

## Contents

**Structs**

- [`Acquire`](#acquire) - A type indicating acquire atomic loads.
- [`AsBox`](#asbox) - A wrapper that serializes a field into a box.
- [`AsOwned`](#asowned) - A wrapper that serializes a `Cow` as if it were owned.
- [`AsString`](#asstring) - A wrapper that attempts to convert a type to and from UTF-8.
- [`AsUnixTime`](#asunixtime) - A wrapper that converts a [`SystemTime`](std::time::SystemTime) to a
- [`AsVec`](#asvec) - A wrapper that serializes associative containers as a `Vec` of key-value
- [`AtomicLoad`](#atomicload) - A wrapper that archives an atomic by loading its value with a particular
- [`Identity`](#identity) - A no-op wrapper which uses the default impls for the type.
- [`Inline`](#inline) - A wrapper that serializes a reference inline.
- [`InlineAsBox`](#inlineasbox) - A wrapper that serializes a reference as if it were boxed.
- [`Lock`](#lock) - A wrapper that locks a lock and serializes the value immutably.
- [`Map`](#map) - A wrapper that applies another wrapper to the values contained in a type.
- [`MapKV`](#mapkv) - A wrapper that applies key and value wrappers to the key-value pairs
- [`MapNiche`](#mapniche) - A wrapper that first applies another wrapper `W` to the value inside an
- [`Niche`](#niche) - A wrapper that niches some type combinations.
- [`NicheInto`](#nicheinto) - A wrapper that niches based on a generic [`Niching`].
- [`Relaxed`](#relaxed) - A type indicating relaxed atomic loads.
- [`SeqCst`](#seqcst) - A type indicating sequentially-consistent atomic loads.
- [`Skip`](#skip) - A wrapper that skips serializing a field.
- [`Unsafe`](#unsafe) - A wrapper that allows serialize-unsafe types to be serialized.
- [`Unshare`](#unshare) - A wrapper that clones the contents of `Arc` and `Rc` pointers.
- [`With`](#with) - A transparent wrapper which applies a "with" type.

**Traits**

- [`ArchiveWith`](#archivewith) - A variant of [`Archive`] that works with wrappers.
- [`DeserializeWith`](#deserializewith) - A variant of `Deserialize` for "with" types.
- [`SerializeWith`](#serializewith) - A variant of `Serialize` for "with" types.

---

## rkyv::with::Acquire

*Struct*

A type indicating acquire atomic loads.

**Unit Struct**



## rkyv::with::ArchiveWith

*Trait*

A variant of [`Archive`] that works with wrappers.

Creating a wrapper allows users to customize how fields are archived easily
without changing the unarchived type.

This trait allows wrapper types to transparently change the archive
behaviors for struct and enum fields. When a field is serialized, it may use
the implementations for the wrapper type and the given field instead of the
implementation for the type itself.

Only a single implementation of [`Archive`] may be written
for each type, but multiple implementations of ArchiveWith can be written
for the same type because it is parametric over the wrapper type. This is
used with the `#[rkyv(with = ..)]` macro attribute to provide a more
flexible interface for serialization.

# Example

```
use rkyv::{
    access_unchecked, deserialize,
    rancor::{Error, Fallible, Infallible, ResultExt as _},
    to_bytes,
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Archive, Archived, Deserialize, Place, Resolver, Serialize,
};

struct Incremented;

impl ArchiveWith<i32> for Incremented {
    type Archived = Archived<i32>;
    type Resolver = Resolver<i32>;

    fn resolve_with(field: &i32, _: (), out: Place<Self::Archived>) {
        let incremented = field + 1;
        incremented.resolve((), out);
    }
}

impl<S> SerializeWith<i32, S> for Incremented
where
    S: Fallible + ?Sized,
    i32: Serialize<S>,
{
    fn serialize_with(
        field: &i32,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        let incremented = field + 1;
        incremented.serialize(serializer)
    }
}

impl<D> DeserializeWith<Archived<i32>, i32, D> for Incremented
where
    D: Fallible + ?Sized,
    Archived<i32>: Deserialize<i32, D>,
{
    fn deserialize_with(
        field: &Archived<i32>,
        deserializer: &mut D,
    ) -> Result<i32, D::Error> {
        Ok(field.deserialize(deserializer)? - 1)
    }
}

#[derive(Archive, Deserialize, Serialize)]
struct Example {
    #[rkyv(with = Incremented)]
    a: i32,
    // Another i32 field, but not incremented this time
    b: i32,
}

let value = Example { a: 4, b: 9 };

let buf = to_bytes::<Error>(&value).unwrap();

let archived =
    unsafe { access_unchecked::<Archived<Example>>(buf.as_ref()) };
// The wrapped field has been incremented
assert_eq!(archived.a, 5);
// ... and the unwrapped field has not
assert_eq!(archived.b, 9);

let deserialized = deserialize::<Example, Infallible>(archived).always_ok();
// The wrapped field is back to normal
assert_eq!(deserialized.a, 4);
// ... and the unwrapped field is unchanged
assert_eq!(deserialized.b, 9);
```

**Methods:**

- `Archived`: The archived type of `Self` with `F`.
- `Resolver`: The resolver of a `Self` with `F`.
- `resolve_with`: Resolves the archived type using a reference to the field type `F`.



## rkyv::with::AsBox

*Struct*

A wrapper that serializes a field into a box.

This functions similarly to [`InlineAsBox`], but is for regular fields
instead of references.

# Example

```
use rkyv::{with::AsBox, Archive};

#[derive(Archive)]
struct Example {
    #[rkyv(with = AsBox)]
    a: i32,
    #[rkyv(with = AsBox)]
    b: str,
}
```

**Unit Struct**

**Trait Implementations:**

- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedBox<<F as >::Archived>, deserializer: & mut D) -> Result<F, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &F, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &F, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rkyv::with::AsOwned

*Struct*

A wrapper that serializes a `Cow` as if it were owned.

# Example

```
use std::borrow::Cow;

use rkyv::{with::AsOwned, Archive};

#[derive(Archive)]
struct Example<'a> {
    #[rkyv(with = AsOwned)]
    a: Cow<'a, str>,
}
```

**Unit Struct**

**Trait Implementations:**

- **SerializeWith**
  - `fn serialize_with(field: &Cow<'a, [T]>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **SerializeWith**
  - `fn serialize_with(field: &Cow<'a, CStr>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DeserializeWith**
  - `fn deserialize_with(field: &<T as >::Archived, deserializer: & mut D) -> Result<T, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Cow<'a, [T]>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &Cow<'a, F>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedVec<<T as >::Archived>, deserializer: & mut D) -> Result<Cow<'a, [T]>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Cow<'a, str>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &Cow<'a, str>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedString, deserializer: & mut D) -> Result<Cow<'a, str>, <D as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedCString, deserializer: & mut D) -> Result<Cow<'a, CStr>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Cow<'a, F>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &Cow<'a, CStr>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`



## rkyv::with::AsString

*Struct*

A wrapper that attempts to convert a type to and from UTF-8.

Types like `OsString` and `PathBuf` aren't guaranteed to be encoded as
UTF-8, but they usually are anyway. Using this wrapper will archive them as
if they were regular `String`s.

It also allows `&str` to be archived like an owned `String`. However, note
that `&str` cannot be *deserialized* this way.

# Example

```
use std::{ffi::OsString, path::PathBuf};

use rkyv::{with::AsString, Archive};

#[derive(Archive)]
struct Example<'a> {
    #[rkyv(with = AsString)]
    os_string: OsString,
    #[rkyv(with = AsString)]
    path: PathBuf,
    #[rkyv(with = AsString)]
    reference: &'a str,
}
```

**Unit Struct**

**Trait Implementations:**

- **SerializeWith**
  - `fn serialize_with(field: &PathBuf, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedString, _: & mut D) -> Result<PathBuf, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(field: &&str, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &OsString, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &&str, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &OsString, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ArchiveWith**
  - `fn resolve_with(field: &PathBuf, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedString, _: & mut D) -> Result<OsString, <D as >::Error>`



## rkyv::with::AsUnixTime

*Struct*

A wrapper that converts a [`SystemTime`](std::time::SystemTime) to a
[`Duration`](std::time::Duration) since
[`UNIX_EPOCH`](std::time::UNIX_EPOCH).

If the serialized time occurs before the UNIX epoch, serialization will
panic during `resolve`. The resulting archived time will be an
[`ArchivedDuration`](crate::time::ArchivedDuration) relative to the UNIX
epoch.

# Example

```
use rkyv::{Archive, with::AsUnixTime};
use std::time::SystemTime;

#[derive(Archive)]
struct Example {
    #[rkyv(with = AsUnixTime)]
    time: SystemTime,
}

**Unit Struct**

**Trait Implementations:**

- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedDuration, _: & mut D) -> Result<SystemTime, <D as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **SerializeWith**
  - `fn serialize_with(field: &SystemTime, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &SystemTime, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`



## rkyv::with::AsVec

*Struct*

A wrapper that serializes associative containers as a `Vec` of key-value
pairs.

This provides faster serialization for containers like `HashMap` and
`BTreeMap` by serializing the key-value pairs directly instead of building a
data structure in the buffer.

It also allows `&[T]` to be archived like an owned `Vec`. However, note
that `&[T]` cannot be *deserialized* this way.

# Example

```
use std::collections::HashMap;

use rkyv::{with::AsVec, Archive};

#[derive(Archive)]
struct Example<'a> {
    #[rkyv(with = AsVec)]
    values: HashMap<String, u32>,
    #[rkyv(with = AsVec)]
    slice: &'a [u32],
}
```

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ArchiveWith**
  - `fn resolve_with(field: &BTreeMap<K, V>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &BTreeSet<T>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedVec<u8>, _: & mut D) -> Result<AlignedVec<A>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(field: &HashMap<K, V, H>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedVec<Entry<<K as >::Archived, <V as >::Archived>>, deserializer: & mut D) -> Result<BTreeMap<K, V>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &HashSet<T, H>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedVec<<T as >::Archived>, deserializer: & mut D) -> Result<HashSet<T, H>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &AlignedVec<A>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &BTreeMap<K, V>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **SerializeWith**
  - `fn serialize_with(field: &AlignedVec<A>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &BTreeSet<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedVec<Entry<<K as >::Archived, <V as >::Archived>>, deserializer: & mut D) -> Result<HashMap<K, V, H>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &&[T], resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &HashSet<T, H>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &HashMap<K, V, H>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedVec<<T as >::Archived>, deserializer: & mut D) -> Result<BTreeSet<T>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(field: &&[T], serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`



## rkyv::with::AtomicLoad

*Struct*

A wrapper that archives an atomic by loading its value with a particular
ordering.

When serializing, the specified ordering will be used to load the value from
the source atomic. The underlying archived type is still a non-atomic value.

# Example

```
# #[cfg(target_has_atomic = "32")]
use core::sync::atomic::AtomicU32;

use rkyv::{
    with::{AtomicLoad, Relaxed},
    Archive,
};

# #[cfg(target_has_atomic = "32")]
#[derive(Archive)]
struct Example {
    #[rkyv(with = AtomicLoad<Relaxed>)]
    a: AtomicU32,
}
```

**Generic Parameters:**
- SO

**Trait Implementations:**

- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicI32_be, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicU64_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &rend::u16_le, _: & mut D) -> Result<rend::AtomicU16_le, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicI32_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicI16_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(_: &core::sync::atomic::AtomicI64, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedU32, _: & mut D) -> Result<core::sync::atomic::AtomicU32, <D as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicI64_be, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedU32, _: & mut D) -> Result<rend::AtomicU32_le, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &core::sync::atomic::AtomicUsize, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(_: &core::sync::atomic::AtomicIsize, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &core::sync::atomic::AtomicI64, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicU16_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedU64, _: & mut D) -> Result<core::sync::atomic::AtomicU64, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicI16_le, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &AtomicI8, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedU64, _: & mut D) -> Result<rend::AtomicU64_le, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicU16_be, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedI16, _: & mut D) -> Result<core::sync::atomic::AtomicI16, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &AtomicU8, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedUsize, _: & mut D) -> Result<core::sync::atomic::AtomicUsize, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicI64_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicI32_le, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicI32_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &core::sync::atomic::AtomicU16, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &rend::i16_be, _: & mut D) -> Result<rend::AtomicI16_be, <D as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &u8, _: & mut D) -> Result<AtomicU8, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicU32_be, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedI32, _: & mut D) -> Result<core::sync::atomic::AtomicI32, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicI64_le, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &core::sync::atomic::AtomicIsize, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedI32, _: & mut D) -> Result<rend::AtomicI32_be, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicU32_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicU16_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicU64_be, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedI64, _: & mut D) -> Result<core::sync::atomic::AtomicI64, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &core::sync::atomic::AtomicU16, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &AtomicBool, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedI64, _: & mut D) -> Result<rend::AtomicI64_be, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicU16_le, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &AtomicI8, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &core::sync::atomic::AtomicI16, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicI64_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &core::sync::atomic::AtomicU32, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedIsize, _: & mut D) -> Result<core::sync::atomic::AtomicIsize, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &core::sync::atomic::AtomicU32, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &rend::i16_le, _: & mut D) -> Result<rend::AtomicI16_le, <D as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &i8, _: & mut D) -> Result<AtomicI8, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicU32_le, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &rend::u16_be, _: & mut D) -> Result<rend::AtomicU16_be, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicU64_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(_: &core::sync::atomic::AtomicU64, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicU32_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &rend::AtomicI16_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedI32, _: & mut D) -> Result<rend::AtomicI32_le, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicU64_le, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedU32, _: & mut D) -> Result<rend::AtomicU32_be, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &core::sync::atomic::AtomicI16, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &core::sync::atomic::AtomicUsize, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &AtomicBool, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedI64, _: & mut D) -> Result<rend::AtomicI64_le, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &core::sync::atomic::AtomicU64, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(_: &rend::AtomicI16_be, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &core::sync::atomic::AtomicI32, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(_: &AtomicU8, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedU64, _: & mut D) -> Result<rend::AtomicU64_be, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &core::sync::atomic::AtomicI32, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &crate::primitive::ArchivedU16, _: & mut D) -> Result<core::sync::atomic::AtomicU16, <D as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &bool, _: & mut D) -> Result<AtomicBool, <D as >::Error>`



## rkyv::with::DeserializeWith

*Trait*

A variant of `Deserialize` for "with" types.

See [ArchiveWith] for more details.

**Methods:**

- `deserialize_with`: Deserializes the field type `F` using the given deserializer.



## rkyv::with::Identity

*Struct*

A no-op wrapper which uses the default impls for the type.

This is most useful for wrappers like [`MapKV`] when you only want to apply
a wrapper to either the key or the value.

# Example

```
use std::collections::HashMap;

use rkyv::{
    with::{Identity, Inline, MapKV},
    Archive,
};

#[derive(Archive)]
struct Example<'a> {
    #[rkyv(with = MapKV<Identity, Inline>)]
    a: HashMap<u32, &'a u32>,
}
```

**Unit Struct**

**Trait Implementations:**

- **SerializeWith**
  - `fn serialize_with(field: &F, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &F, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DeserializeWith**
  - `fn deserialize_with(field: &F, deserializer: & mut D) -> Result<T, <D as Fallible>::Error>`



## rkyv::with::Inline

*Struct*

A wrapper that serializes a reference inline.

References serialized with `Inline` cannot be deserialized because the
struct cannot own the deserialized value.

# Example

```
use rkyv::{with::Inline, Archive};

#[derive(Archive)]
struct Example<'a> {
    #[rkyv(with = Inline)]
    a: &'a i32,
}
```

**Unit Struct**

**Trait Implementations:**

- **SerializeWith**
  - `fn serialize_with(field: &&F, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &&F, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rkyv::with::InlineAsBox

*Struct*

A wrapper that serializes a reference as if it were boxed.

Unlike [`Inline`], unsized references can be serialized with `InlineAsBox`.

References serialized with `InlineAsBox` cannot be deserialized because the
struct cannot own the deserialized value.

# Example

```
use rkyv::{with::InlineAsBox, Archive};

#[derive(Archive)]
struct Example<'a> {
    #[rkyv(with = InlineAsBox)]
    a: &'a i32,
    #[rkyv(with = InlineAsBox)]
    b: &'a str,
}
```

**Unit Struct**

**Trait Implementations:**

- **SerializeWith**
  - `fn serialize_with(field: &&F, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ArchiveWith**
  - `fn resolve_with(field: &&F, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`



## rkyv::with::Lock

*Struct*

A wrapper that locks a lock and serializes the value immutably.

This wrapper can panic under very specific circumstances when:

1. `serialize_with` is called and succeeds in locking the value to serialize
   it.
2. Another thread locks the value and panics, poisoning the lock
3. `resolve_with` is called and gets a poisoned value.

Unfortunately, it's not possible to work around this issue internally. Users
must ensure this doesn't happen on their own through manual synchronization
or guaranteeing that panics do not occur while holding locks.

# Example

```
use std::sync::Mutex;

use rkyv::{with::Lock, Archive};

#[derive(Archive)]
struct Example {
    #[rkyv(with = Lock)]
    a: Mutex<i32>,
}
```

**Unit Struct**

**Trait Implementations:**

- **DeserializeWith**
  - `fn deserialize_with(field: &F, deserializer: & mut D) -> Result<RwLock<T>, <D as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ArchiveWith**
  - `fn resolve_with(field: &Mutex<F>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &RwLock<F>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &F, deserializer: & mut D) -> Result<Mutex<T>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &RwLock<F>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &Mutex<F>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`



## rkyv::with::Map

*Struct*

A wrapper that applies another wrapper to the values contained in a type.
This can be applied to a vector to map each element, or an option to map any
contained value.

See [ArchiveWith] for more details.

# Example

```
use rkyv::{
    with::{InlineAsBox, Map},
    Archive,
};

#[derive(Archive)]
struct Example<'a> {
    // This will apply `InlineAsBox` to the `&i32` contained in this option
    #[rkyv(with = Map<InlineAsBox>)]
    option: Option<&'a i32>,
    // This will apply `InlineAsBox` to each `&i32` contained in this vector
    #[rkyv(with = Map<InlineAsBox>)]
    vec: Vec<&'a i32>,
}
```

**Generic Parameters:**
- T

**Trait Implementations:**

- **SerializeWith**
  - `fn serialize_with(field: &[O; N], serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &[<A as >::Archived; N], deserializer: & mut D) -> Result<[O; N], <D as Fallible>::Error>`
- **SerializeWith**
  - `fn serialize_with(field: &Option<O>, s: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOption<<A as ArchiveWith>::Archived>, d: & mut D) -> Result<Option<O>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<O>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &Vec<O>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &Vec<O>, s: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedVec<<A as ArchiveWith>::Archived>, d: & mut D) -> Result<Vec<O>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &[O; N], resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`



## rkyv::with::MapKV

*Struct*

A wrapper that applies key and value wrappers to the key-value pairs
contained in a type. This can be applied to a hash map or B-tree map to map
the key-value pairs.

# Example
```
use std::collections::HashMap;

use rkyv::{
    with::{Inline, InlineAsBox, MapKV},
    Archive,
};

#[derive(Archive)]
struct Example<'a> {
    // This will apply `InlineAsBox` to the `&str` key, and `Inline` to the
    // `&str` value.
    #[rkyv(with = MapKV<InlineAsBox, Inline>)]
    hash_map: HashMap<&'a str, &'a str>,
}
```

**Generic Parameters:**
- K
- V

**Trait Implementations:**

- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedHashMap<<A as ArchiveWith>::Archived, <B as ArchiveWith>::Archived>, deserializer: & mut D) -> Result<HashMap<K, V, S>, <D as Fallible>::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedBTreeMap<<A as ArchiveWith>::Archived, <B as ArchiveWith>::Archived>, deserializer: & mut D) -> Result<BTreeMap<K, V>, <D as Fallible>::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &BTreeMap<K, V>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &HashMap<K, V, H>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as Fallible>::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &HashMap<K, V, H>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(field: &BTreeMap<K, V>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as Fallible>::Error>`



## rkyv::with::MapNiche

*Struct*

A wrapper that first applies another wrapper `W` to the value inside an
`Option` and then niches the result based on the [`Niching`] `N`.

# Example

```
use rkyv::{
    with::{AsBox, MapNiche},
    Archive, Serialize,
};

#[derive(Archive, Serialize)]
struct BasicExample {
    option: Option<HugeType>,
}

#[derive(Archive, Serialize)]
struct NichedExample {
    #[rkyv(with = MapNiche<AsBox>)]
    option: Option<HugeType>,
}

#[derive(Archive, Serialize)]
struct HugeType([u8; 1024]);

# fn main() -> Result<(), rkyv::rancor::Error> {
let basic_value = BasicExample { option: None };
let basic_bytes = rkyv::to_bytes(&basic_value)?;
assert_eq!(basic_bytes.len(), 1 + 1024);

let niched_value = NichedExample { option: None };
let niched_bytes = rkyv::to_bytes(&niched_value)?;
assert_eq!(niched_bytes.len(), 4); // size_of::<ArchivedBox<_>>()
# Ok(()) }
```

[`Niching`]: crate::niche::niching::Niching

**Generic Parameters:**
- W
- N

**Trait Implementations:**

- **ArchiveWith**
  - `fn resolve_with(field: &Option<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &NichedOption<<W as ArchiveWith>::Archived, N>, deserializer: & mut D) -> Result<Option<T>, <D as >::Error>`
- **Default**
  - `fn default() -> Self`
- **SerializeWith**
  - `fn serialize_with(field: &Option<T>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## rkyv::with::Niche

*Struct*

A wrapper that niches some type combinations.

A common type combination is `Option<Box<T>>`. By using a null pointer, the
archived version can save some space on-disk.

# Example

```
use core::mem::size_of;

use rkyv::{with::Niche, Archive, Archived};

#[derive(Archive)]
struct BasicExample {
    value: Option<Box<str>>,
}

#[derive(Archive)]
struct NichedExample {
    #[rkyv(with = Niche)]
    value: Option<Box<str>>,
}

assert!(
    size_of::<Archived<BasicExample>>()
        > size_of::<Archived<NichedExample>>()
);
```

**Unit Struct**

**Trait Implementations:**

- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroU64, _: & mut D) -> Result<Option<NonZeroU64>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroI32>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroU8, _: & mut D) -> Result<Option<NonZeroU8>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroU8>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroU8>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroUsize, _: & mut D) -> Result<Option<NonZeroUsize>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroU64>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroU64>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroI8>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroUsize>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroUsize>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionBox<<T as >::Archived>, deserializer: & mut D) -> Result<Option<Box<T>>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroI64>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroI8, _: & mut D) -> Result<Option<NonZeroI8>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroI16>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroI128>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroU16>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroI32, _: & mut D) -> Result<Option<NonZeroI32>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroU32>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroIsize>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroI128, _: & mut D) -> Result<Option<NonZeroI128>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroU128>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroI16, _: & mut D) -> Result<Option<NonZeroI16>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroI16>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroI64, _: & mut D) -> Result<Option<NonZeroI64>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroI8>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroI128>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroI64>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **SerializeWith**
  - `fn serialize_with(field: &Option<Box<T>>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<Box<T>>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroU32, _: & mut D) -> Result<Option<NonZeroU32>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroU16>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroU32>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroIsize, _: & mut D) -> Result<Option<NonZeroIsize>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &Option<NonZeroU128>, _: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroU128, _: & mut D) -> Result<Option<NonZeroU128>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroIsize>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &ArchivedOptionNonZeroU16, _: & mut D) -> Result<Option<NonZeroU16>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<NonZeroI32>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>)`



## rkyv::with::NicheInto

*Struct*

A wrapper that niches based on a generic [`Niching`].

A common type combination is `Option<Box<T>>`. By niching `None` into the
null pointer, the archived version can save some space on-disk.

# Example

```
use core::mem::size_of;

use rkyv::{
    niche::niching::{NaN, Null},
    with::NicheInto,
    Archive, Archived,
};

#[derive(Archive)]
struct BasicExample {
    maybe_box: Option<Box<str>>,
    maybe_non_nan: Option<f32>,
}

#[derive(Archive)]
struct NichedExample {
    #[rkyv(with = NicheInto<Null>)]
    maybe_box: Option<Box<str>>,
    #[rkyv(with = NicheInto<NaN>)]
    maybe_non_nan: Option<f32>,
}

assert!(
    size_of::<Archived<BasicExample>>()
        > size_of::<Archived<NichedExample>>()
);
```

[`Niching`]: crate::niche::niching::Niching

**Generic Parameters:**
- N

**Tuple Struct**: `()`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **DeserializeWith**
  - `fn deserialize_with(field: &NichedOption<<T as >::Archived, N>, deserializer: & mut D) -> Result<Option<T>, <D as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **SerializeWith**
  - `fn serialize_with(field: &Option<T>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`



## rkyv::with::Relaxed

*Struct*

A type indicating relaxed atomic loads.

**Unit Struct**



## rkyv::with::SeqCst

*Struct*

A type indicating sequentially-consistent atomic loads.

**Unit Struct**



## rkyv::with::SerializeWith

*Trait*

A variant of `Serialize` for "with" types.

See [ArchiveWith] for more details.

**Methods:**

- `serialize_with`: Serializes the field type `F` using the given serializer.



## rkyv::with::Skip

*Struct*

A wrapper that skips serializing a field.

Skipped fields must implement `Default` to be deserialized.

# Example

```
use rkyv::{with::Skip, Archive};

#[derive(Archive)]
struct Example {
    #[rkyv(with = Skip)]
    a: u32,
}
```

**Unit Struct**

**Trait Implementations:**

- **DeserializeWith**
  - `fn deserialize_with(_: &(), _: & mut D) -> Result<F, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(_: &F, _: & mut S) -> Result<(), <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(_: &F, _: <Self as >::Resolver, _: Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rkyv::with::Unsafe

*Struct*

A wrapper that allows serialize-unsafe types to be serialized.

Types like `Cell` and `UnsafeCell` may contain serializable types, but have
unsafe access semantics due to interior mutability. They may be safe to
serialize, but only under conditions that rkyv is unable to guarantee.

This wrapper enables serializing these types, and places the burden of
verifying that their access semantics are used safely on the user.

# Safety

Using this wrapper on types with interior mutability can create races
conditions or allow access to data in an invalid state if access semantics
are not followed properly. During serialization, the data must not be
modified.

# Example

```
use core::cell::{Cell, UnsafeCell};

use rkyv::{with::Unsafe, Archive};

#[derive(Archive)]
struct Example {
    #[rkyv(with = Unsafe)]
    cell: Cell<String>,
    #[rkyv(with = Unsafe)]
    unsafe_cell: UnsafeCell<String>,
}
```

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ArchiveWith**
  - `fn resolve_with(field: &UnsafeCell<F>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **ArchiveWith**
  - `fn resolve_with(field: &Cell<F>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &<F as >::Archived, deserializer: & mut D) -> Result<Cell<F>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(field: &Cell<F>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(field: &<F as >::Archived, deserializer: & mut D) -> Result<UnsafeCell<F>, <D as >::Error>`
- **SerializeWith**
  - `fn serialize_with(field: &UnsafeCell<F>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`



## rkyv::with::Unshare

*Struct*

A wrapper that clones the contents of `Arc` and `Rc` pointers.

**Unit Struct**

**Trait Implementations:**

- **SerializeWith**
  - `fn serialize_with(x: &Rc<T>, s: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **ArchiveWith**
  - `fn resolve_with(x: &crate::alloc::sync::Arc<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **SerializeWith**
  - `fn serialize_with(x: &crate::alloc::sync::Arc<T>, s: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **DeserializeWith**
  - `fn deserialize_with(x: &A, d: & mut D) -> Result<Rc<T>, <D as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ArchiveWith**
  - `fn resolve_with(x: &Rc<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(x: &A, d: & mut D) -> Result<crate::alloc::sync::Arc<T>, <D as >::Error>`



## rkyv::with::With

*Struct*

A transparent wrapper which applies a "with" type.

`With` wraps a reference to a type and applies the specified wrapper type
when serializing and deserializing.

**Generic Parameters:**
- F
- W

**Methods:**

- `fn cast(field: &F) -> &Self` - Casts a `With` reference from a reference to the underlying field.

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<T, <D as Fallible>::Error>`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut S) -> Result<<Self as >::Resolver, <S as Fallible>::Error>`



