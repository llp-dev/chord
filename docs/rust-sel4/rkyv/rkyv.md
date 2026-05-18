**rkyv**

# Module: rkyv

## Contents

**Modules**

- [`api`](#api) - APIs for producing and using archived data.
- [`boxed`](#boxed) - An archived version of `Box`.
- [`collections`](#collections) - Archived versions of standard library containers.
- [`de`](#de) - Deserialization traits, deserializers, and adapters.
- [`ffi`](#ffi) - Archived versions of FFI types.
- [`hash`](#hash) - Hashing support for archived hash maps and sets.
- [`net`](#net) - Archived versions of network types.
- [`niche`](#niche) - Manually niched type replacements.
- [`ops`](#ops) - Archived versions of `ops` types.
- [`option`](#option) - An archived version of `Option`.
- [`place`](#place) - An initialized, writeable location in memory.
- [`primitive`](#primitive) - Definitions of archived primitives and type aliases based on enabled
- [`rc`](#rc) - Archived versions of shared pointers.
- [`rel_ptr`](#rel_ptr) - Relative pointer implementations and options.
- [`result`](#result) - An archived version of `Result`.
- [`seal`](#seal) - Mutable references to values which may not be moved or de-initialized.
- [`ser`](#ser) - Serialization traits and adapters.
- [`string`](#string) - Archived versions of string types.
- [`time`](#time) - Archived versions of `time` types.
- [`traits`](#traits) - The core traits provided by rkyv.
- [`tuple`](#tuple) - Archived versions of tuple types.
- [`util`](#util) - Utilities for common operations.
- [`validation`](#validation) - Validation implementations and helper types.
- [`vec`](#vec) - An archived version of `Vec`.
- [`with`](#with) - Wrapper type support and commonly used wrappers.

---

## Module: api

APIs for producing and using archived data.

# Accessing byte slices

The safety requirements for accessing a byte slice will often state that a
byte slice must "represent a valid archived type". The specific validity
requirements may vary widely depending on the types being accessed, and so
in general the only way to guarantee that this call is safe is to have
previously validated the byte slice.

Using techniques such as cryptographic signing can provide a more performant
way to verify data integrity from trusted sources.

It is generally safe to assume that unchanged and properly-aligned
serialized bytes are always safe to access without validation. By contrast,
bytes from a potentially-malicious source should always be validated prior
to access.



## Module: boxed

An archived version of `Box`.



## Module: collections

Archived versions of standard library containers.



## Module: de

Deserialization traits, deserializers, and adapters.



## Module: ffi

Archived versions of FFI types.



## Module: hash

Hashing support for archived hash maps and sets.



## Module: net

Archived versions of network types.



## Module: niche

Manually niched type replacements.



## Module: ops

Archived versions of `ops` types.



## Module: option

An archived version of `Option`.



## Module: place

An initialized, writeable location in memory.



## Module: primitive

Definitions of archived primitives and type aliases based on enabled
features.



## Module: rc

Archived versions of shared pointers.



## Module: rel_ptr

Relative pointer implementations and options.



## Module: result

An archived version of `Result`.



## Module: seal

Mutable references to values which may not be moved or de-initialized.



## Module: ser

Serialization traits and adapters.



## Module: string

Archived versions of string types.



## Module: time

Archived versions of `time` types.



## Module: traits

The core traits provided by rkyv.



## Module: tuple

Archived versions of tuple types.



## Module: util

Utilities for common operations.



## Module: validation

Validation implementations and helper types.



## Module: vec

An archived version of `Vec`.



## Module: with

Wrapper type support and commonly used wrappers.

Wrappers can be applied with the `#[rkyv(with = ..)]` attribute in the
[`Archive`](macro@crate::Archive) macro.



