**rkyv > alias**

# Module: alias

## Contents

**Type Aliases**

- [`Archived`](#archived) - Alias for the archived version of some [`Archive`] type.
- [`ArchivedMetadata`](#archivedmetadata) - Alias for the archived metadata for some [`ArchiveUnsized`] type.
- [`RawRelPtr`](#rawrelptr) - The default raw relative pointer.
- [`RelPtr`](#relptr) - The default relative pointer.
- [`Resolver`](#resolver) - Alias for the resolver for some [`Archive`] type.

---

## rkyv::alias::Archived

*Type Alias*: `<T as Archive>::Archived`

Alias for the archived version of some [`Archive`] type.

This can be useful for reducing the lengths of type definitions.



## rkyv::alias::ArchivedMetadata

*Type Alias*: `<<T as ArchiveUnsized>::Archived as ArchivePointee>::ArchivedMetadata`

Alias for the archived metadata for some [`ArchiveUnsized`] type.

This can be useful for reducing the lengths of type definitions.



## rkyv::alias::RawRelPtr

*Type Alias*: `rel_ptr::RawRelPtr<crate::primitive::ArchivedIsize>`

The default raw relative pointer.

This will use an archived [`FixedIsize`](crate::primitive::FixedIsize) to
hold the offset.



## rkyv::alias::RelPtr

*Type Alias*: `rel_ptr::RelPtr<T, crate::primitive::ArchivedIsize>`

The default relative pointer.

This will use an archived [`FixedIsize`](crate::primitive::FixedIsize) to
hold the offset.



## rkyv::alias::Resolver

*Type Alias*: `<T as Archive>::Resolver`

Alias for the resolver for some [`Archive`] type.

This can be useful for reducing the lengths of type definitions.



