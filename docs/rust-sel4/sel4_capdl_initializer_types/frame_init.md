**sel4_capdl_initializer_types > frame_init**

# Module: frame_init

## Contents

**Structs**

- [`ArchivedBytesContent`](#archivedbytescontent) - An archived [`BytesContent`]
- [`ArchivedDeflatedBytesContent`](#archiveddeflatedbytescontent) - An archived [`DeflatedBytesContent`]
- [`ArchivedEmbeddedFrameIndex`](#archivedembeddedframeindex) - An archived [`EmbeddedFrameIndex`]
- [`ArchivedFill`](#archivedfill) - An archived [`Fill`]
- [`ArchivedFillEntry`](#archivedfillentry) - An archived [`FillEntry`]
- [`ArchivedFillEntryContentBootInfo`](#archivedfillentrycontentbootinfo) - An archived [`FillEntryContentBootInfo`]
- [`ArchivedFillEntryContentFileOffset`](#archivedfillentrycontentfileoffset) - An archived [`FillEntryContentFileOffset`]
- [`BytesContent`](#bytescontent)
- [`BytesContentResolver`](#bytescontentresolver) - The resolver for an archived [`BytesContent`]
- [`DeflatedBytesContent`](#deflatedbytescontent)
- [`DeflatedBytesContentResolver`](#deflatedbytescontentresolver) - The resolver for an archived [`DeflatedBytesContent`]
- [`EmbeddedFrameIndex`](#embeddedframeindex)
- [`EmbeddedFrameIndexResolver`](#embeddedframeindexresolver) - The resolver for an archived [`EmbeddedFrameIndex`]
- [`Fill`](#fill)
- [`FillEntry`](#fillentry)
- [`FillEntryContentBootInfo`](#fillentrycontentbootinfo)
- [`FillEntryContentBootInfoResolver`](#fillentrycontentbootinforesolver) - The resolver for an archived [`FillEntryContentBootInfo`]
- [`FillEntryContentFileOffset`](#fillentrycontentfileoffset)
- [`FillEntryContentFileOffsetResolver`](#fillentrycontentfileoffsetresolver) - The resolver for an archived [`FillEntryContentFileOffset`]
- [`FillEntryResolver`](#fillentryresolver) - The resolver for an archived [`FillEntry`]
- [`FillResolver`](#fillresolver) - The resolver for an archived [`Fill`]

**Enums**

- [`ArchivedContent`](#archivedcontent) - An archived [`Content`]
- [`ArchivedFillEntryContent`](#archivedfillentrycontent) - An archived [`FillEntryContent`]
- [`ArchivedFillEntryContentBootInfoId`](#archivedfillentrycontentbootinfoid) - An archived [`FillEntryContentBootInfoId`]
- [`ArchivedFrameInit`](#archivedframeinit) - An archived [`FrameInit`]
- [`Content`](#content)
- [`ContentResolver`](#contentresolver) - The resolver for an archived [`Content`]
- [`FillEntryContent`](#fillentrycontent)
- [`FillEntryContentBootInfoId`](#fillentrycontentbootinfoid)
- [`FillEntryContentBootInfoIdResolver`](#fillentrycontentbootinfoidresolver) - The resolver for an archived [`FillEntryContentBootInfoId`]
- [`FillEntryContentResolver`](#fillentrycontentresolver) - The resolver for an archived [`FillEntryContent`]
- [`FrameInit`](#frameinit)
- [`FrameInitResolver`](#frameinitresolver) - The resolver for an archived [`FrameInit`]

---

## sel4_capdl_initializer_types::frame_init::ArchivedBytesContent

*Struct*

An archived [`BytesContent`]

**Fields:**
- `bytes: <alloc::vec::Vec<u8> as ::rkyv::Archive>::Archived` - The archived counterpart of [`BytesContent::bytes`]

**Methods:**

- `fn copy_out(self: &Self, dst: & mut [u8])`

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::ArchivedContent

*Enum*

An archived [`Content`]

**Variants:**
- `Bytes(<BytesContent as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Content::Bytes`]
- `DeflatedBytes(<DeflatedBytesContent as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Content::DeflatedBytes`]

**Methods:**

- `fn copy_out(self: &Self, dst: & mut [u8])`

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::ArchivedDeflatedBytesContent

*Struct*

An archived [`DeflatedBytesContent`]

**Fields:**
- `deflated_bytes: <alloc::vec::Vec<u8> as ::rkyv::Archive>::Archived` - The archived counterpart of [`DeflatedBytesContent::deflated_bytes`]

**Methods:**

- `fn copy_out(self: &Self, dst: & mut [u8])`

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::ArchivedEmbeddedFrameIndex

*Struct*

An archived [`EmbeddedFrameIndex`]

**Fields:**
- `index: <u64 as ::rkyv::Archive>::Archived` - The archived counterpart of [`EmbeddedFrameIndex::index`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::ArchivedFill

*Struct*

An archived [`Fill`]

**Generic Parameters:**
- D

**Fields:**
- `entries: <alloc::vec::Vec<FillEntry<D>> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Fill::entries`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::ArchivedFillEntry

*Struct*

An archived [`FillEntry`]

**Generic Parameters:**
- D

**Fields:**
- `range: <core::ops::Range<u64> as ::rkyv::Archive>::Archived` - The archived counterpart of [`FillEntry::range`]
- `content: <FillEntryContent<D> as ::rkyv::Archive>::Archived` - The archived counterpart of [`FillEntry::content`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::ArchivedFillEntryContent

*Enum*

An archived [`FillEntryContent`]

**Generic Parameters:**
- D

**Variants:**
- `Data(<D as ::rkyv::Archive>::Archived)` - The archived counterpart of [`FillEntryContent::Data`]
- `BootInfo(<FillEntryContentBootInfo as ::rkyv::Archive>::Archived)` - The archived counterpart of [`FillEntryContent::BootInfo`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::ArchivedFillEntryContentBootInfo

*Struct*

An archived [`FillEntryContentBootInfo`]

**Fields:**
- `id: <FillEntryContentBootInfoId as ::rkyv::Archive>::Archived` - The archived counterpart of [`FillEntryContentBootInfo::id`]
- `offset: <u64 as ::rkyv::Archive>::Archived` - The archived counterpart of [`FillEntryContentBootInfo::offset`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::ArchivedFillEntryContentBootInfoId

*Enum*

An archived [`FillEntryContentBootInfoId`]

**Variants:**
- `Fdt` - The archived counterpart of [`FillEntryContentBootInfoId::Fdt`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::ArchivedFillEntryContentFileOffset

*Struct*

An archived [`FillEntryContentFileOffset`]

**Fields:**
- `file: <alloc::string::String as ::rkyv::Archive>::Archived` - The archived counterpart of [`FillEntryContentFileOffset::file`]
- `file_offset: <u64 as ::rkyv::Archive>::Archived` - The archived counterpart of [`FillEntryContentFileOffset::file_offset`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::ArchivedFrameInit

*Enum*

An archived [`FrameInit`]

**Variants:**
- `Fill(<Fill<Content> as ::rkyv::Archive>::Archived)` - The archived counterpart of [`FrameInit::Fill`]
- `Embedded(<EmbeddedFrameIndex as ::rkyv::Archive>::Archived)` - The archived counterpart of [`FrameInit::Embedded`]

**Methods:**

- `fn is_embedded(self: &Self) -> bool`

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::BytesContent

*Struct*

**Fields:**
- `bytes: alloc::vec::Vec<u8>`

**Methods:**

- `fn pack(raw_content: &[u8]) -> Self`
- `fn copy_out(self: &Self, dst: & mut [u8])`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BytesContent) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BytesContent`



## sel4_capdl_initializer_types::frame_init::BytesContentResolver

*Struct*

The resolver for an archived [`BytesContent`]



## sel4_capdl_initializer_types::frame_init::Content

*Enum*

**Variants:**
- `Bytes(BytesContent)`
- `DeflatedBytes(DeflatedBytesContent)`

**Methods:**

- `fn copy_out(self: &Self, dst: & mut [u8])`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Content`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Content) -> bool`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as ::rkyv::Archive>::Resolver, out: ::rkyv::Place<<Self as ::rkyv::Archive>::Archived>)`



## sel4_capdl_initializer_types::frame_init::ContentResolver

*Enum*

The resolver for an archived [`Content`]

**Variants:**
- `Bytes(<BytesContent as ::rkyv::Archive>::Resolver)` - The resolver for [`Content::Bytes`]
- `DeflatedBytes(<DeflatedBytesContent as ::rkyv::Archive>::Resolver)` - The resolver for [`Content::DeflatedBytes`]



## sel4_capdl_initializer_types::frame_init::DeflatedBytesContent

*Struct*

**Fields:**
- `deflated_bytes: alloc::vec::Vec<u8>`

**Methods:**

- `fn copy_out(self: &Self, dst: & mut [u8])`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DeflatedBytesContent`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DeflatedBytesContent) -> bool`



## sel4_capdl_initializer_types::frame_init::DeflatedBytesContentResolver

*Struct*

The resolver for an archived [`DeflatedBytesContent`]



## sel4_capdl_initializer_types::frame_init::EmbeddedFrameIndex

*Struct*

**Fields:**
- `index: u64`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &EmbeddedFrameIndex) -> bool`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> EmbeddedFrameIndex`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_capdl_initializer_types::frame_init::EmbeddedFrameIndexResolver

*Struct*

The resolver for an archived [`EmbeddedFrameIndex`]



## sel4_capdl_initializer_types::frame_init::Fill

*Struct*

**Generic Parameters:**
- D

**Fields:**
- `entries: alloc::vec::Vec<FillEntry<D>>`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Fill<D>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Fill<D>`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::FillEntry

*Struct*

**Generic Parameters:**
- D

**Fields:**
- `range: core::ops::Range<u64>`
- `content: FillEntryContent<D>`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FillEntry<D>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> FillEntry<D>`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::frame_init::FillEntryContent

*Enum*

**Generic Parameters:**
- D

**Variants:**
- `Data(D)`
- `BootInfo(FillEntryContentBootInfo)`

**Methods:**

- `fn as_data(self: &Self) -> Option<&D>`
- `fn as_bootinfo(self: &Self) -> Option<&FillEntryContentBootInfo>`
- `fn is_data(self: &Self) -> bool`
- `fn is_bootinfo(self: &Self) -> bool`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as ::rkyv::Archive>::Resolver, out: ::rkyv::Place<<Self as ::rkyv::Archive>::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FillEntryContent<D>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> FillEntryContent<D>`



## sel4_capdl_initializer_types::frame_init::FillEntryContentBootInfo

*Struct*

**Fields:**
- `id: FillEntryContentBootInfoId`
- `offset: u64`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &FillEntryContentBootInfo) -> bool`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FillEntryContentBootInfo`



## sel4_capdl_initializer_types::frame_init::FillEntryContentBootInfoId

*Enum*

**Variants:**
- `Fdt`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &FillEntryContentBootInfoId) -> bool`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as ::rkyv::Archive>::Resolver, out: ::rkyv::Place<<Self as ::rkyv::Archive>::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FillEntryContentBootInfoId`



## sel4_capdl_initializer_types::frame_init::FillEntryContentBootInfoIdResolver

*Enum*

The resolver for an archived [`FillEntryContentBootInfoId`]

**Variants:**
- `Fdt` - The resolver for [`FillEntryContentBootInfoId::Fdt`]



## sel4_capdl_initializer_types::frame_init::FillEntryContentBootInfoResolver

*Struct*

The resolver for an archived [`FillEntryContentBootInfo`]



## sel4_capdl_initializer_types::frame_init::FillEntryContentFileOffset

*Struct*

**Fields:**
- `file: alloc::string::String`
- `file_offset: u64`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FillEntryContentFileOffset`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FillEntryContentFileOffset) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &FillEntryContentFileOffset) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &FillEntryContentFileOffset) -> $crate::option::Option<$crate::cmp::Ordering>`



## sel4_capdl_initializer_types::frame_init::FillEntryContentFileOffsetResolver

*Struct*

The resolver for an archived [`FillEntryContentFileOffset`]



## sel4_capdl_initializer_types::frame_init::FillEntryContentResolver

*Enum*

The resolver for an archived [`FillEntryContent`]

**Generic Parameters:**
- D

**Variants:**
- `Data(<D as ::rkyv::Archive>::Resolver)` - The resolver for [`FillEntryContent::Data`]
- `BootInfo(<FillEntryContentBootInfo as ::rkyv::Archive>::Resolver)` - The resolver for [`FillEntryContent::BootInfo`]



## sel4_capdl_initializer_types::frame_init::FillEntryResolver

*Struct*

The resolver for an archived [`FillEntry`]

**Generic Parameters:**
- D



## sel4_capdl_initializer_types::frame_init::FillResolver

*Struct*

The resolver for an archived [`Fill`]

**Generic Parameters:**
- D



## sel4_capdl_initializer_types::frame_init::FrameInit

*Enum*

**Variants:**
- `Fill(Fill<Content>)`
- `Embedded(EmbeddedFrameIndex)`

**Methods:**

- `fn is_embedded(self: &Self) -> bool`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as ::rkyv::Archive>::Resolver, out: ::rkyv::Place<<Self as ::rkyv::Archive>::Archived>)`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &FrameInit) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FrameInit`



## sel4_capdl_initializer_types::frame_init::FrameInitResolver

*Enum*

The resolver for an archived [`FrameInit`]

**Variants:**
- `Fill(<Fill<Content> as ::rkyv::Archive>::Resolver)` - The resolver for [`FrameInit::Fill`]
- `Embedded(<EmbeddedFrameIndex as ::rkyv::Archive>::Resolver)` - The resolver for [`FrameInit::Embedded`]



