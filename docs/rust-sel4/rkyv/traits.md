**rkyv > traits**

# Module: traits

## Contents

**Structs**

- [`CopyOptimization`](#copyoptimization) - An optimization hint about whether `T` is trivially copyable.

**Traits**

- [`Archive`](#archive) - A type that can be used without deserializing.
- [`ArchivePointee`](#archivepointee) - An archived type with associated metadata for its relative pointer.
- [`ArchiveUnsized`](#archiveunsized) - A counterpart of [`Archive`] that's suitable for unsized types.
- [`Deserialize`](#deserialize) - Converts a type back from its archived form.
- [`DeserializeUnsized`](#deserializeunsized) - A counterpart of [`Deserialize`] that's suitable for unsized types.
- [`LayoutRaw`](#layoutraw) - Returns the layout of a type from its metadata.
- [`NoUndef`](#noundef) - A type with no undefined bytes.
- [`Portable`](#portable) - A type with a stable, well-defined layout that is the same on all targets.
- [`Serialize`](#serialize) - Converts a type to its archived form.
- [`SerializeUnsized`](#serializeunsized) - A counterpart of [`Serialize`] that's suitable for unsized types.

---

## rkyv::traits::Archive

*Trait*

A type that can be used without deserializing.

`Archive` is one of three basic traits used to work with zero-copy data and
controls the layout of the data in its archived zero-copy representation.
The [`Serialize`] trait helps transform types into that representation, and
the [`Deserialize`] trait helps transform types back out.

Types that implement `Archive` must have a well-defined archived size.
Unsized types can be supported using the [`ArchiveUnsized`] trait, along
with [`SerializeUnsized`] and [`DeserializeUnsized`].

Archiving is done depth-first, writing any data owned by a type before
writing the data for the type itself. The type must be able to create the
archived type from only its own data and its resolver.

Archived data is always treated as if it is tree-shaped, with the root
owning its direct descendents and so on. Data that is not tree-shaped can be
supported using special serializer and deserializer bounds (see
[`ArchivedRc`](crate::rc::ArchivedRc) for example). In a buffer of
serialized data, objects are laid out in *reverse order*. This means that
the root object is located near the end of the buffer and leaf objects are
located near the beginning.

# Examples

Most of the time, `#[derive(Archive)]` will create an acceptable
implementation. You can use the `#[rkyv(...)]` attribute to control how the
implementation is generated. See the [`Archive`](macro@crate::Archive)
derive macro for more details.
```
use rkyv::{deserialize, rancor::Error, Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(
    // This will generate a PartialEq impl between our unarchived
    // and archived types
    compare(PartialEq),
    // Derives can be passed through to the generated type:
    derive(Debug),
)]
struct Test {
    int: u8,
    string: String,
    option: Option<Vec<i32>>,
}

fn main() {
    let value = Test {
        int: 42,
        string: "hello world".to_string(),
        option: Some(vec![1, 2, 3, 4]),
    };

    // Serializing is as easy as a single function call
    let _bytes = rkyv::to_bytes::<Error>(&value).unwrap();

    // Or you can customize your serialization for better performance or control
    // over resource usage
    use rkyv::{api::high::to_bytes_with_alloc, ser::allocator::Arena};

    let mut arena = Arena::new();
    let bytes =
        to_bytes_with_alloc::<_, Error>(&value, arena.acquire()).unwrap();

    // You can use the safe API for fast zero-copy deserialization
    let archived = rkyv::access::<ArchivedTest, Error>(&bytes[..]).unwrap();
    assert_eq!(archived, &value);

    // Or you can use the unsafe API for maximum performance
    let archived =
        unsafe { rkyv::access_unchecked::<ArchivedTest>(&bytes[..]) };
    assert_eq!(archived, &value);

    // And you can always deserialize back to the original type
    let deserialized = deserialize::<Test, Error>(archived).unwrap();
    assert_eq!(deserialized, value);
}
```
_Note: the safe API requires the `bytecheck` feature._

Many of the core and standard library types already have `Archive`
implementations available, but you may need to implement `Archive` for your
own types in some cases the derive macro cannot handle.

In this example, we add our own wrapper that serializes a `&'static str` as
if it's owned. Normally you can lean on the archived version of `String` to
do most of the work, or use the [`Inline`](crate::with::Inline) to do
exactly this. This example does everything to demonstrate how to implement
`Archive` for your own types.
```
use core::{slice, str};

use rkyv::{
    access_unchecked,
    rancor::{Error, Fallible},
    ser::Writer,
    to_bytes,
    Archive, ArchiveUnsized, Archived, Portable, RelPtr, Serialize,
    SerializeUnsized, munge::munge, Place,
};

struct OwnedStr {
    inner: &'static str,
}

#[derive(Portable)]
#[repr(transparent)]
struct ArchivedOwnedStr {
    // This will be a relative pointer to our string
    ptr: RelPtr<str>,
}

impl ArchivedOwnedStr {
    // This will help us get the bytes of our type as a str again.
    fn as_str(&self) -> &str {
        unsafe {
            // The as_ptr() function of RelPtr will get a pointer the str
            &*self.ptr.as_ptr()
        }
    }
}

struct OwnedStrResolver {
    // This will be the position that the bytes of our string are stored at.
    // We'll use this to resolve the relative pointer of our
    // ArchivedOwnedStr.
    pos: usize,
}

// The Archive implementation defines the archived version of our type and
// determines how to turn the resolver into the archived form. The Serialize
// implementations determine how to make a resolver from the original value.
impl Archive for OwnedStr {
    type Archived = ArchivedOwnedStr;
    // This is the resolver we can create our Archived version from.
    type Resolver = OwnedStrResolver;

    // The resolve function consumes the resolver and produces the archived
    // value at the given position.
    fn resolve(
        &self,
        resolver: Self::Resolver,
        out: Place<Self::Archived>,
    ) {
        munge!(let ArchivedOwnedStr { ptr } = out);
        RelPtr::emplace_unsized(
            resolver.pos,
            self.inner.archived_metadata(),
            ptr,
        );
    }
}

// We restrict our serializer types with Writer because we need its
// capabilities to serialize the inner string. For other types, we might
// need more or less restrictive bounds on the type of S.
impl<S: Fallible + Writer + ?Sized> Serialize<S> for OwnedStr {
    fn serialize(
        &self,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        // This is where we want to write the bytes of our string and return
        // a resolver that knows where those bytes were written.
        // We also need to serialize the metadata for our str.
        Ok(OwnedStrResolver {
            pos: self.inner.serialize_unsized(serializer)?,
        })
    }
}

const STR_VAL: &'static str = "I'm in an OwnedStr!";
let value = OwnedStr { inner: STR_VAL };
// It works!
let buf = to_bytes::<Error>(&value).expect("failed to serialize");
let archived =
    unsafe { access_unchecked::<ArchivedOwnedStr>(buf.as_ref()) };
// Let's make sure our data got written correctly
assert_eq!(archived.as_str(), STR_VAL);
```

**Methods:**

- `COPY_OPTIMIZATION`: An optimization flag that allows the bytes of this type to be copied
- `Archived`: The archived representation of this type.
- `Resolver`: The resolver for this type. It must contain all the additional
- `resolve`: Creates the archived version of this value at the given position and



## rkyv::traits::ArchivePointee

*Trait*

An archived type with associated metadata for its relative pointer.

This is mostly used in the context of smart pointers and unsized types, and
is implemented for all sized types by default.

**Methods:**

- `ArchivedMetadata`: The archived version of the pointer metadata for this type.
- `pointer_metadata`: Converts some archived metadata to the pointer metadata for itself.



## rkyv::traits::ArchiveUnsized

*Trait*

A counterpart of [`Archive`] that's suitable for unsized types.

Unlike `Archive`, types that implement `ArchiveUnsized` must be serialized
separately from their owning object. For example, whereas an `i32` might be
laid out as part of a larger struct, a `Box<i32>` would serialize the `i32`
somewhere in the archive and the `Box` would point to it as part of the
larger struct. Because of this, the equivalent
[`Resolver`](Archive::Resolver) type for `ArchiveUnsized` is always a
`usize` representing the position of the serialized value.

`ArchiveUnsized` is automatically implemented for all types that implement
[`Archive`]. Nothing special needs to be done to use them with types like
`Box`, `Rc`, and `Arc`. It is also already implemented for slices and string
slices, and the `rkyv_dyn` crate can be used to archive trait objects. Other
unsized types must manually implement `ArchiveUnsized`.

# Examples

This example shows how to manually implement `ArchiveUnsized` for an unsized
type. Special care must be taken to ensure that the types are laid out
correctly.

```
use core::ops::{Deref, DerefMut};

use ptr_meta::Pointee;
use rkyv::{
    access_unchecked,
    primitive::ArchivedUsize,
    rancor::{Error, Fallible},
    ser::{Positional, Writer, WriterExt as _},
    to_bytes,
    traits::ArchivePointee,
    Archive, ArchiveUnsized, Archived, ArchivedMetadata, Portable, RelPtr,
    Serialize, SerializeUnsized,
};

// We're going to be dealing mostly with blocks that have a trailing slice
#[derive(Portable)]
#[repr(C)]
pub struct Block<H, T: ?Sized> {
    head: H,
    tail: T,
}

unsafe impl<H, T> Pointee for Block<H, [T]> {
    type Metadata = <[T] as Pointee>::Metadata;
}

// ArchivePointee is automatically derived for sized types because pointers
// to sized types don't need to store any extra information. Because we're
// making an unsized block, we need to define what metadata gets stored with
// our data pointer.
impl<H, T> ArchivePointee for Block<H, [T]> {
    // This is the extra data that needs to get stored for blocks with
    // trailing slices
    type ArchivedMetadata = <[T] as ArchivePointee>::ArchivedMetadata;

    // We need to be able to turn our archived metadata into regular
    // metadata for our type
    fn pointer_metadata(
        metadata: &Self::ArchivedMetadata,
    ) -> <Self as Pointee>::Metadata {
        metadata.to_native() as usize
    }
}

// We're implementing ArchiveUnsized for just Block<H, [T]>. We can still
// implement Archive for blocks with sized tails and they won't conflict.
impl<H: Archive, T: Archive> ArchiveUnsized for Block<H, [T]> {
    // We'll reuse our block type as our archived type.
    type Archived = Block<Archived<H>, [Archived<T>]>;

    // Here's where we make the metadata for our archived type.
    fn archived_metadata(&self) -> ArchivedMetadata<Self> {
        // Because the metadata for our `ArchivedBlock` is the metadata of
        // the trailing slice, we just need to return that archived
        // metadata.
        self.tail.archived_metadata()
    }
}

// The bounds we use on our serializer type indicate that we need basic
// serializer capabilities, and then whatever capabilities our head and tail
// types need to serialize themselves.
impl<H, T, S> SerializeUnsized<S> for Block<H, [T]>
where
    H: Serialize<S>,
    T: Serialize<S>,
    S: Fallible + Writer + ?Sized,
{
    // This is where we construct our unsized type in the serializer
    fn serialize_unsized(
        &self,
        serializer: &mut S,
    ) -> Result<usize, S::Error> {
        // First, we serialize the head and all the tails. This will make
        // sure that when we finally build our block, we don't accidentally
        // mess up the structure with serialized dependencies.
        let head_resolver = self.head.serialize(serializer)?;
        let mut resolvers = Vec::new();
        for tail in self.tail.iter() {
            resolvers.push(tail.serialize(serializer)?);
        }
        // Now we align our serializer for our archived type and resolve it.
        // We can't align for unsized types so we treat the trailing slice
        // like an array of 0 length for now.
        let result = serializer
            .align_for::<Block<Archived<H>, [Archived<T>; 0]>>()?;
        unsafe {
            serializer.resolve_aligned(&self.head, head_resolver)?;
        }
        serializer.align_for::<Archived<T>>()?;
        for (item, resolver) in self.tail.iter().zip(resolvers.drain(..)) {
            unsafe {
                serializer.resolve_aligned(item, resolver)?;
            }
        }
        Ok(result)
    }
}

let value = Box::new(Block {
    head: "Numbers 1-4".to_string(),
    tail: [1, 2, 3, 4],
});

// We have a Box<Block<String, [i32; 4]>> but we want to it to be a
// Box<Block<String, [i32]>>, so we need manually "unsize" the pointer.
let ptr = Box::into_raw(value);
let unsized_ptr = ptr_meta::from_raw_parts_mut::<Block<String, [i32]>>(
    ptr.cast::<()>(),
    4,
);
let unsized_value = unsafe { Box::from_raw(unsized_ptr) };

let bytes = to_bytes::<Error>(&unsized_value).unwrap();

let archived = unsafe {
    access_unchecked::<Archived<Box<Block<String, [i32]>>>>(&bytes)
};
assert_eq!(archived.head, "Numbers 1-4");
assert_eq!(archived.tail.len(), 4);
assert_eq!(archived.tail, [1, 2, 3, 4]);
```

**Methods:**

- `Archived`: The archived counterpart of this type. Unlike `Archive`, it may be
- `archived_metadata`: Creates the archived version of the metadata for this value.



## rkyv::traits::CopyOptimization

*Struct*

An optimization hint about whether `T` is trivially copyable.

**Generic Parameters:**
- T

**Tuple Struct**: `()`

**Methods:**

- `fn enable() -> Self` - Returns a `CopyOptimization` hint with the optimization enabled for `T`.
- `fn enable_if(value: bool) -> Self` - Returns a `CopyOptimization` hint with the optimization enabled for `T`
- `fn disable() -> Self` - Returns a `CopyOptimization` hint with the optimization disabled for
- `fn is_enabled(self: &Self) -> bool` - Returns whether the optimization is enabled for `T`.



## rkyv::traits::Deserialize

*Trait*

Converts a type back from its archived form.

Some types may require specific deserializer capabilities, such as `Rc` and
`Arc`. In these cases, the deserializer type `D` should be bound so that it
implements traits that provide those capabilities (e.g.
[`Pooling`](crate::de::Pooling)).

This can be derived with [`Deserialize`](macro@crate::Deserialize).

**Methods:**

- `deserialize`: Deserializes using the given deserializer



## rkyv::traits::DeserializeUnsized

*Trait*

A counterpart of [`Deserialize`] that's suitable for unsized types.

**Methods:**

- `deserialize_unsized`: Deserializes a reference to the given value.
- `deserialize_metadata`: Deserializes the metadata for the given type.



## rkyv::traits::LayoutRaw

*Trait*

Returns the layout of a type from its metadata.

**Methods:**

- `layout_raw`: Returns the layout of the type.



## rkyv::traits::NoUndef

*Trait*

A type with no undefined bytes.

# Safety

The bytes of types implementing `NoUndef` must always be well-defined. Among
other things, this means that `NoUndef` types may not contain padding or
uninitialized `MaybeUninit`s.



## rkyv::traits::Portable

*Trait*

A type with a stable, well-defined layout that is the same on all targets.

# Safety

The implementing type must have a stable, well-defined layout that is the
same on all targets. Structs and unions must be `#[repr(transparent)]` or
`#[repr(C)]`. Enums must be `#[repr(C)]`, `#[repr(int)]`, or `#[repr(C,
int)]`.

The implementing type must not have interior mutability (i.e. no
`UnsafeCell`s).



## rkyv::traits::Serialize

*Trait*

Converts a type to its archived form.

Objects perform any supportive serialization during
[`serialize`](Serialize::serialize). For types that reference nonlocal
(pointed-to) data, this is when that data must be serialized to the output.
These types will need to bound `S` to implement
[`Writer`](crate::ser::Writer) and any other required traits (e.g.
[`Sharing`](crate::ser::Sharing)). They should then serialize their
dependencies during `serialize`.

See [`Archive`] for examples of implementing `Serialize`.

**Methods:**

- `serialize`: Writes the dependencies for the object and returns a resolver that can



## rkyv::traits::SerializeUnsized

*Trait*

A counterpart of [`Serialize`] that's suitable for unsized types.

See [`ArchiveUnsized`] for examples of implementing `SerializeUnsized`.

**Methods:**

- `serialize_unsized`: Writes the object and returns the position of the archived type.



