**object > read > macho > dyld_cache**

# Module: read::macho::dyld_cache

## Contents

**Structs**

- [`DyldCache`](#dyldcache) - A parsed representation of the dyld shared cache.
- [`DyldCacheImage`](#dyldcacheimage) - One image (dylib) from inside the dyld shared cache.
- [`DyldCacheImageIterator`](#dyldcacheimageiterator) - An iterator over all the images (dylibs) in the dyld shared cache.
- [`DyldCacheMapping`](#dyldcachemapping) - Information about a mapping.
- [`DyldCacheMappingIterator`](#dyldcachemappingiterator) - An iterator over all the mappings for one subcache in a dyld shared cache.
- [`DyldCacheRelocationIterator`](#dyldcacherelocationiterator) - An iterator over relocations in a mapping
- [`DyldRelocation`](#dyldrelocation) - A cache mapping relocation.
- [`DyldRelocationAuth`](#dyldrelocationauth) - Pointer authentication data.

**Enums**

- [`DyldCacheMappingSlice`](#dyldcachemappingslice) - The array of mappings for a single dyld cache file.
- [`DyldCacheSlideInfo`](#dyldcacheslideinfo) - The slide info for a dyld cache mapping, including variable length arrays.
- [`DyldSubCacheSlice`](#dyldsubcacheslice) - A slice of structs describing each subcache.

---

## object::read::macho::dyld_cache::DyldCache

*Struct*

A parsed representation of the dyld shared cache.

**Generic Parameters:**
- 'data
- E
- R

**Methods:**

- `fn subcache_suffixes(data: R) -> Result<Vec<String>>` - Return the suffixes of the subcache files given the data of the main cache file.
- `fn parse(data: R, subcache_data: &[R]) -> Result<Self>` - Parse the raw dyld shared cache data.
- `fn architecture(self: &Self) -> Architecture` - Get the architecture type of the file.
- `fn endianness(self: &Self) -> Endianness` - Get the endianness of the file.
- `fn data(self: &Self) -> R` - Get the data of the main cache file.
- `fn is_little_endian(self: &Self) -> bool` - Return true if the file is little endian, false if it is big endian.
- `fn images<'cache>(self: &'cache Self) -> DyldCacheImageIterator<'data, 'cache, E, R>` - Iterate over the images in this cache.
- `fn mappings<'cache>(self: &'cache Self) -> impl Trait` - Return all the mappings in this cache.
- `fn data_and_offset_for_address(self: &Self, address: u64) -> Option<(R, u64)>` - Find the address in a mapping and return the cache or subcache data it was found in,

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::macho::dyld_cache::DyldCacheImage

*Struct*

One image (dylib) from inside the dyld shared cache.

**Generic Parameters:**
- 'data
- 'cache
- E
- R

**Methods:**

- `fn info(self: &Self) -> &'data macho::DyldCacheImageInfo<E>` - Return the raw data structure for this image.
- `fn path(self: &Self) -> Result<&'data str>` - The file system path of this image.
- `fn image_data_and_offset(self: &Self) -> Result<(R, u64)>` - The subcache data which contains the Mach-O header for this image,
- `fn parse_object(self: &Self) -> Result<File<'data, R>>` - Parse this image into an Object.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::macho::dyld_cache::DyldCacheImageIterator

*Struct*

An iterator over all the images (dylibs) in the dyld shared cache.

**Generic Parameters:**
- 'data
- 'cache
- E
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<DyldCacheImage<'data, 'cache, E, R>>`



## object::read::macho::dyld_cache::DyldCacheMapping

*Struct*

Information about a mapping.

**Generic Parameters:**
- 'data
- E
- R

**Methods:**

- `fn address(self: &Self) -> u64` - The mapping address
- `fn size(self: &Self) -> u64` - The mapping size
- `fn file_offset(self: &Self) -> u64` - The mapping file offset
- `fn max_prot(self: &Self) -> u32` - The mapping maximum protection
- `fn init_prot(self: &Self) -> u32` - The mapping initial protection
- `fn data(self: &Self) -> Result<&'data [u8]>` - The mapping data
- `fn relocations(self: &Self) -> Result<DyldCacheRelocationIterator<'data, E, R>>` - Relocations for the mapping

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DyldCacheMapping<'data, E, R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::read::macho::dyld_cache::DyldCacheMappingIterator

*Struct*

An iterator over all the mappings for one subcache in a dyld shared cache.

**Generic Parameters:**
- 'data
- E
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::macho::dyld_cache::DyldCacheMappingSlice

*Enum*

The array of mappings for a single dyld cache file.

The mappings gained slide info in dyld-832.7 (macOS 11)
so this is an enum of the two possible slice types.

**Generic Parameters:**
- 'data
- E

**Variants:**
- `V1(&'data [macho::DyldCacheMappingInfo<E>])` - V1, used before dyld-832.7.
- `V2(&'data [macho::DyldCacheMappingAndSlideInfo<E>])` - V2, used since dyld-832.7.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldCacheMappingSlice<'data, E>`



## object::read::macho::dyld_cache::DyldCacheRelocationIterator

*Struct*

An iterator over relocations in a mapping

**Generic Parameters:**
- 'data
- E
- R

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::macho::dyld_cache::DyldCacheSlideInfo

*Enum*

The slide info for a dyld cache mapping, including variable length arrays.

**Generic Parameters:**
- 'data
- E

**Variants:**
- `None`
- `V2{ slide: &'data macho::DyldCacheSlideInfo2<E>, page_starts: &'data [crate::endian::U16<E>], page_extras: &'data [crate::endian::U16<E>] }`
- `V3{ slide: &'data macho::DyldCacheSlideInfo3<E>, page_starts: &'data [crate::endian::U16<E>] }`
- `V5{ slide: &'data macho::DyldCacheSlideInfo5<E>, page_starts: &'data [crate::endian::U16<E>] }`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldCacheSlideInfo<'data, E>`



## object::read::macho::dyld_cache::DyldRelocation

*Struct*

A cache mapping relocation.

**Fields:**
- `offset: u64` - The offset of the relocation within the mapping.
- `value: u64` - The value to be relocated.
- `auth: Option<DyldRelocationAuth>` - The pointer authentication data, if present.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::read::macho::dyld_cache::DyldRelocationAuth

*Struct*

Pointer authentication data.

This is used for signing pointers for the arm64e ABI.

**Fields:**
- `key: macho::PtrauthKey` - The key used to generate the signed value.
- `diversity: u16` - The integer diversity value.
- `addr_div: bool` - Whether the address should be blended with the diversity value.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::read::macho::dyld_cache::DyldSubCacheSlice

*Enum*

A slice of structs describing each subcache.

The struct gained an additional field (the file suffix) in dyld-1042.1 (macOS 13 / iOS 16),
so this is an enum of the two possible slice types.

**Generic Parameters:**
- 'data
- E

**Variants:**
- `V1(&'data [macho::DyldSubCacheEntryV1<E>])` - V1, used between dyld-940 and dyld-1042.1.
- `V2(&'data [macho::DyldSubCacheEntryV2<E>])` - V2, used since dyld-1042.1.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DyldSubCacheSlice<'data, E>`



