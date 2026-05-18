# rkyv

rkyv is a zero-copy deserialization framework for Rust.

## Overview

rkyv uses Rust's powerful trait system to serialize data without reflection.
Many zero-copy deserialization frameworks use external schemas and heavily
restrict the available data types. By contrast, rkyv allows all serialized
types to be defined in code and can serialize a wide variety of types that
other frameworks cannot.

rkyv scales to highly-capable as well as highly-restricted environments. Not
only does rkyv support "no-std" builds for targets without a standard
library implementation, it also supports "no-alloc" builds for targets where
allocations cannot be made.

rkyv supports limited in-place data mutation, and so can access and update
data without ever deserializing back to native types. When rkyv's in-place
mutation is too limited, rkyv also provides ergonomic and performant
deserialization back into native types.

rkyv prioritizes performance, and is one of the fastest serialization
frameworks available. All of rkyv's features can be individually enabled and
disabled, so you only pay for what you use. Additionally, all of rkyv's
zero-copy types are designed to have little to no overhead. In most cases,
rkyv's types will have exactly the same performance as native types.

See the [rkyv book] for guide-level documentation and usage examples.

[rkyv book]: https://rkyv.org

## Components

rkyv has [a hash map implementation] that is built for zero-copy
deserialization, with the same lookup and iteration performance as the
standard library hash maps. The hash map implementation is based on
[Swiss Tables] and uses a target-independent version of FxHash to ensure
that all targets compute the same hashes.

It also has [a B-tree implementation] that has the same performance
characteristics as the standard library B-tree maps. Its compact
representation and localized data storage is best-suited for very large
amounts of data.

rkyv supports [shared pointers] by default, and is able to serialize and
deserialize them without duplicating the underlying data. Shared pointers
which point to the same data when serialized will still point to the same
data when deserialized. By default, rkyv only supports non-cyclic data
structures.

Alongside its [unchecked API], rkyv also provides optional [validation] so
you can ensure safety and data integrity at the cost of some overhead.
Because checking serialized data can generally be done without allocations,
the cost of checking and zero-copy access can be much lower than that of
traditional deserialization.

rkyv is trait-oriented from top to bottom, and is made to be extended with
custom and specialized types. Serialization, deserialization, and
validation traits all accept generic context types, making it easy to add
new capabilities without degrading ergonomics.

[a hash map implementation]: collections::swiss_table::ArchivedHashMap
[Swiss Tables]: https://abseil.io/about/design/swisstables
[a B-tree implementation]: collections::btree_map::ArchivedBTreeMap
[shared pointers]: rc
[unchecked API]: access_unchecked
[validation]: access

## Features

rkyv has several feature flags which can be used to modify its behavior. By
default, rkyv enables the `std`, `alloc`, and `bytecheck` features.

### Format control

These features control how rkyv formats its serialized data. Enabling and
disabling these features may change rkyv's serialized format, and as such
can cause previously-serialized data to become unreadable. Enabling format
control features that are not the default should be considered a breaking
change to rkyv's serialized format.

Binaries should consider explicitly choosing format control options from the
start, even though doing so is not required. This ensures that developers
stay informed about the specific choices being made, and prevents any
unexpected compatibility issues with libraries they depend on.

Libraries should avoid enabling format control features unless they intend
to only support rkyv when those specific format control features are
enabled. In general, libraries should be able to support all format control
options if they use rkyv's exported types and aliases.

#### Endianness

If an endianness feature is not enabled, rkyv will use little-endian byte
ordering by default.

- `little_endian`: Forces data serialization to use little-endian byte
  ordering. This optimizes serialized data for little-endian architectures.
- `big_endian`: Forces data serialization to use big-endian byte ordering.
  This optimizes serialized data for big-endian architectures.

#### Alignment

If an alignment feature is not enabled, rkyv will use aligned primitives by
default.

- `aligned`: Forces data serialization to use aligned primitives. This adds
  alignment requirements for accessing data and prevents rkyv from working
  with unaligned data.
- `unaligned`: Forces data serialization to use unaligned primitives. This
  removes alignment requirements for accessing data and allows rkyv to work
  with unaligned data more easily.

#### Pointer width

If a pointer width feature is not enabled, rkyv will serialize `isize` and
`usize` as 32-bit integers by default.

- `pointer_width_16`: Serializes `isize` and `usize` as 16-bit integers.
  This is intended to be used only for small data sizes and may not handle
  large amounts of data.
- `pointer_width_32`: Serializes `isize` and `usize` as 32-bit integers.
  This is a good choice for most data, and balances the storage overhead
  with support for large data sizes.
- `pointer_width_64`: Serializes `isize` and `usize` as 64-bit integers.
  This is intended to be used only for extremely large data sizes and may
  cause unnecessary data bloat for smaller amounts of data.

### Functionality

These features enable more built-in functionality and provide more powerful
and ergonomic APIs. Enabling and disabling these features does not change
rkyv's serialized format.

- `alloc`: Enables support for the `alloc` crate. Enabled by default.
- `std`: Enables standard library support. Enabled by default.
- `bytecheck`: Enables data validation through `bytecheck`. Enabled by
  default.

### Crates

rkyv provides integrations for some common crates by default. In the future,
crates should depend on rkyv and provide their own integration. Enabling and
disabling these features does not change rkyv's serialized format.

- [`arrayvec-0_7`](https://docs.rs/arrayvec/0.7)
- [`bytes-1`](https://docs.rs/bytes/1)
- [`hashbrown-0_14`](https://docs.rs/hashbrown/0.14)
- [`hashbrown-0_15`](https://docs.rs/hashbrown/0.15)
- [`indexmap-2`](https://docs.rs/indexmap/2)
- [`smallvec-1`](https://docs.rs/smallvec/1)
- [`smol_str-0_2`](https://docs.rs/smol_str/0.2)
- [`smol_str-0_3`](https://docs.rs/smol_str/0.3)
- [`thin-vec-0_2`](https://docs.rs/thin-vec/0.2)
- [`tinyvec-1`](https://docs.rs/tinyvec/1)
- [`triomphe-0_1`](https://docs.rs/triomphe/0.1)
- [`uuid-1`](https://docs.rs/uuid/1)

## Compatibility

Serialized data can be accessed later as long as:

- The underlying schema has not changed
- The serialized format has not been changed by format control features
- The data was serialized by a semver-compatible version of rkyv

## Modules

### [`rkyv`](rkyv.md)

*25 modules*

### [`alias`](alias.md)

*5 type aliases*

### [`api`](api.md)

*2 modules, 7 functions*

### [`api::checked`](api/checked.md)

*3 functions*

### [`api::high`](api/high.md)

*2 type aliases, 6 functions*

### [`api::high::checked`](api/high/checked.md)

*1 type alias, 5 functions*

### [`api::low`](api/low.md)

*2 type aliases, 3 functions*

### [`api::low::checked`](api/low/checked.md)

*1 type alias, 5 functions*

### [`boxed`](boxed.md)

*2 structs*

### [`collections`](collections.md)

*2 modules*

### [`collections::btree`](collections/btree.md)

*2 modules*

### [`collections::btree::map`](collections/btree/map.md)

*2 structs*

### [`collections::btree::map::iter`](collections/btree/map/iter.md)

*7 structs*

### [`collections::btree::set`](collections/btree/set.md)

*2 structs*

### [`collections::btree::set::iter`](collections/btree/set/iter.md)

*1 struct*

### [`collections::swiss_table`](collections/swiss_table.md)

*5 modules*

### [`collections::swiss_table::index_map`](collections/swiss_table/index_map.md)

*5 structs*

### [`collections::swiss_table::index_set`](collections/swiss_table/index_set.md)

*2 structs*

### [`collections::swiss_table::map`](collections/swiss_table/map.md)

*7 structs*

### [`collections::swiss_table::set`](collections/swiss_table/set.md)

*2 structs*

### [`collections::swiss_table::table`](collections/swiss_table/table.md)

*3 structs*

### [`collections::util`](collections/util.md)

*4 structs*

### [`de`](de.md)

*1 module*

### [`de::pooling`](de/pooling.md)

*1 enum, 1 struct, 1 union, 4 traits*

### [`de::pooling::alloc`](de/pooling/alloc.md)

*1 struct*

### [`de::pooling::core`](de/pooling/core.md)

*1 struct*

### [`ffi`](ffi.md)

*2 structs*

### [`hash`](hash.md)

*1 function, 1 struct*

### [`net`](net.md)

*2 enums, 4 structs*

### [`niche`](niche.md)

*4 modules*

### [`niche::niched_option`](niche/niched_option.md)

*1 struct, 1 type alias*

### [`niche::niching`](niche/niching.md)

*2 traits, 5 structs*

### [`niche::option_box`](niche/option_box.md)

*1 enum, 1 struct, 1 type alias*

### [`niche::option_nonzero`](niche/option_nonzero.md)

*10 structs, 3 type aliases*

### [`ops`](ops.md)

*1 enum, 6 structs*

### [`option`](option.md)

*1 enum, 1 struct*

### [`place`](place.md)

*1 struct*

### [`primitive`](primitive.md)

*27 type aliases*

### [`rc`](rc.md)

*1 trait, 6 structs*

### [`rel_ptr`](rel_ptr.md)

*1 function, 1 trait, 2 structs, 8 type aliases*

### [`result`](result.md)

*1 enum, 1 type alias*

### [`seal`](seal.md)

*1 struct*

### [`ser`](ser.md)

*1 struct, 3 modules*

### [`ser::allocator`](ser/allocator.md)

*1 trait, 2 structs*

### [`ser::allocator::alloc`](ser/allocator/alloc.md)

*2 structs*

### [`ser::allocator::core`](ser/allocator/core.md)

*1 struct*

### [`ser::sharing`](ser/sharing.md)

*1 enum, 2 traits*

### [`ser::sharing::alloc`](ser/sharing/alloc.md)

*1 struct*

### [`ser::sharing::core`](ser/sharing/core.md)

*1 struct*

### [`ser::writer`](ser/writer.md)

*3 traits*

### [`ser::writer::core`](ser/writer/core.md)

*1 struct*

### [`ser::writer::std`](ser/writer/std.md)

*1 struct*

### [`string`](string.md)

*1 module, 2 structs*

### [`string::repr`](string/repr.md)

*1 union, 2 constants*

### [`time`](time.md)

*1 struct*

### [`traits`](traits.md)

*1 struct, 10 traits*

### [`tuple`](tuple.md)

*13 structs*

### [`util`](util.md)

*1 struct*

### [`util::alloc::aligned_vec`](util/alloc/aligned_vec.md)

*1 struct*

### [`util::alloc::arena`](util/alloc/arena.md)

*2 functions*

### [`util::inline_vec`](util/inline_vec.md)

*2 structs*

### [`util::ser_vec`](util/ser_vec.md)

*2 structs*

### [`validation`](validation.md)

*1 struct, 2 modules*

### [`validation::archive`](validation/archive.md)

*2 traits*

### [`validation::archive::validator`](validation/archive/validator.md)

*1 struct*

### [`validation::shared`](validation/shared.md)

*1 enum, 1 trait*

### [`validation::shared::validator`](validation/shared/validator.md)

*1 struct*

### [`vec`](vec.md)

*2 structs*

### [`with`](with.md)

*22 structs, 3 traits*

