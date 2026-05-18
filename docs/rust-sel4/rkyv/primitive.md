**rkyv > primitive**

# Module: primitive

## Contents

**Type Aliases**

- [`ArchivedChar`](#archivedchar) - The archived version of `char`.
- [`ArchivedF32`](#archivedf32) - The archived version of `f32`.
- [`ArchivedF64`](#archivedf64) - The archived version of `f64`.
- [`ArchivedI128`](#archivedi128) - The archived version of `i128`.
- [`ArchivedI16`](#archivedi16) - The archived version of `i16`.
- [`ArchivedI32`](#archivedi32) - The archived version of `i32`.
- [`ArchivedI64`](#archivedi64) - The archived version of `i64`.
- [`ArchivedIsize`](#archivedisize) - The archived version of `isize` chosen based on the currently-enabled
- [`ArchivedNonZeroI128`](#archivednonzeroi128) - The archived version of `NonZeroI128`.
- [`ArchivedNonZeroI16`](#archivednonzeroi16) - The archived version of `NonZeroI16`.
- [`ArchivedNonZeroI32`](#archivednonzeroi32) - The archived version of `NonZeroI32`.
- [`ArchivedNonZeroI64`](#archivednonzeroi64) - The archived version of `NonZeroI64`.
- [`ArchivedNonZeroIsize`](#archivednonzeroisize) - The archived version of `NonZeroIsize` chosen based on the currently-enabled
- [`ArchivedNonZeroU128`](#archivednonzerou128) - The archived version of `NonZeroU128`.
- [`ArchivedNonZeroU16`](#archivednonzerou16) - The archived version of `NonZeroU16`.
- [`ArchivedNonZeroU32`](#archivednonzerou32) - The archived version of `NonZeroU32`.
- [`ArchivedNonZeroU64`](#archivednonzerou64) - The archived version of `NonZeroU64`.
- [`ArchivedNonZeroUsize`](#archivednonzerousize) - The archived version of `NonZeroUsize` chosen based on the currently-enabled
- [`ArchivedU128`](#archivedu128) - The archived version of `u128`.
- [`ArchivedU16`](#archivedu16) - The archived version of `u16`.
- [`ArchivedU32`](#archivedu32) - The archived version of `u32`.
- [`ArchivedU64`](#archivedu64) - The archived version of `u64`.
- [`ArchivedUsize`](#archivedusize) - The archived version of `isize` chosen based on the currently-enabled
- [`FixedIsize`](#fixedisize) - The native type that `isize` is converted to for archiving.
- [`FixedNonZeroIsize`](#fixednonzeroisize) - The native type that `NonZeroIsize` is converted to for archiving.
- [`FixedNonZeroUsize`](#fixednonzerousize) - The native type that `NonZeroUsize` is converted to for archiving.
- [`FixedUsize`](#fixedusize) - The native type that `usize` is converted to for archiving.

---

## rkyv::primitive::ArchivedChar

*Type Alias*: `crate::rend::char_le`

The archived version of `char`.



## rkyv::primitive::ArchivedF32

*Type Alias*: `crate::rend::f32_le`

The archived version of `f32`.



## rkyv::primitive::ArchivedF64

*Type Alias*: `crate::rend::f64_le`

The archived version of `f64`.



## rkyv::primitive::ArchivedI128

*Type Alias*: `crate::rend::i128_le`

The archived version of `i128`.



## rkyv::primitive::ArchivedI16

*Type Alias*: `crate::rend::i16_le`

The archived version of `i16`.



## rkyv::primitive::ArchivedI32

*Type Alias*: `crate::rend::i32_le`

The archived version of `i32`.



## rkyv::primitive::ArchivedI64

*Type Alias*: `crate::rend::i64_le`

The archived version of `i64`.



## rkyv::primitive::ArchivedIsize

*Type Alias*: `ArchivedI32`

The archived version of `isize` chosen based on the currently-enabled
`pointer_width_*` feature.



## rkyv::primitive::ArchivedNonZeroI128

*Type Alias*: `crate::rend::NonZeroI128_le`

The archived version of `NonZeroI128`.



## rkyv::primitive::ArchivedNonZeroI16

*Type Alias*: `crate::rend::NonZeroI16_le`

The archived version of `NonZeroI16`.



## rkyv::primitive::ArchivedNonZeroI32

*Type Alias*: `crate::rend::NonZeroI32_le`

The archived version of `NonZeroI32`.



## rkyv::primitive::ArchivedNonZeroI64

*Type Alias*: `crate::rend::NonZeroI64_le`

The archived version of `NonZeroI64`.



## rkyv::primitive::ArchivedNonZeroIsize

*Type Alias*: `ArchivedNonZeroI32`

The archived version of `NonZeroIsize` chosen based on the currently-enabled
`pointer_width_*` feature.



## rkyv::primitive::ArchivedNonZeroU128

*Type Alias*: `crate::rend::NonZeroU128_le`

The archived version of `NonZeroU128`.



## rkyv::primitive::ArchivedNonZeroU16

*Type Alias*: `crate::rend::NonZeroU16_le`

The archived version of `NonZeroU16`.



## rkyv::primitive::ArchivedNonZeroU32

*Type Alias*: `crate::rend::NonZeroU32_le`

The archived version of `NonZeroU32`.



## rkyv::primitive::ArchivedNonZeroU64

*Type Alias*: `crate::rend::NonZeroU64_le`

The archived version of `NonZeroU64`.



## rkyv::primitive::ArchivedNonZeroUsize

*Type Alias*: `ArchivedNonZeroU32`

The archived version of `NonZeroUsize` chosen based on the currently-enabled
`pointer_width_*` feature.



## rkyv::primitive::ArchivedU128

*Type Alias*: `crate::rend::u128_le`

The archived version of `u128`.



## rkyv::primitive::ArchivedU16

*Type Alias*: `crate::rend::u16_le`

The archived version of `u16`.



## rkyv::primitive::ArchivedU32

*Type Alias*: `crate::rend::u32_le`

The archived version of `u32`.



## rkyv::primitive::ArchivedU64

*Type Alias*: `crate::rend::u64_le`

The archived version of `u64`.



## rkyv::primitive::ArchivedUsize

*Type Alias*: `ArchivedU32`

The archived version of `isize` chosen based on the currently-enabled
`pointer_width_*` feature.



## rkyv::primitive::FixedIsize

*Type Alias*: `i32`

The native type that `isize` is converted to for archiving.

This will be `i16`, `i32`, or `i64` when the `pointer_width_16`,
`pointer_width_32`, or `pointer_width_64` features are enabled,
respectively. With no pointer width features enabled, it defaults to `i32`.



## rkyv::primitive::FixedNonZeroIsize

*Type Alias*: `::core::num::NonZeroI32`

The native type that `NonZeroIsize` is converted to for archiving.

This will be `NonZeroI16`, `NonZeroI32`, or `NonZeroI64` when the
`pointer_width_16`, `pointer_width_32`, or `pointer_width_64` features are
enabled, respectively. With no pointer width features enabled, it defaults
to `NonZeroI32`.



## rkyv::primitive::FixedNonZeroUsize

*Type Alias*: `::core::num::NonZeroU32`

The native type that `NonZeroUsize` is converted to for archiving.

This will be `NonZeroU16`, `NonZeroU32`, or `NonZeroU64` when the
`pointer_width_16`, `pointer_width_32`, or `pointer_width_64` features are
enabled, respectively. With no pointer width features enabled, it defaults
to `NonZeroU32`.



## rkyv::primitive::FixedUsize

*Type Alias*: `u32`

The native type that `usize` is converted to for archiving.

This will be `u16`, `u32`, or `u64` when the `pointer_width_16`,
`pointer_width_32`, or `pointer_width_64` features are enabled,
respectively. With no pointer width features enabled, it defaults to `u32`.



