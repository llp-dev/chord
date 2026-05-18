**safe_mmio > fields**

# Module: fields

## Contents

**Structs**

- [`ReadOnly`](#readonly) - Wrapper for a field which may safely be read but not written. Reading may cause side-effects,
- [`ReadPure`](#readpure) - Wrapper for a field which may safely be read with no side-effects but not written.
- [`ReadPureWrite`](#readpurewrite) - Wrapper for a field which may safely be written (with side-effects) and read with no
- [`ReadWrite`](#readwrite) - Wrapper for a field which may safely be written and read. Reading may cause side-effects,
- [`WriteOnly`](#writeonly) - Wrapper for a field which may safely be written but not read.

---

## safe_mmio::fields::ReadOnly

*Struct*

Wrapper for a field which may safely be read but not written. Reading may cause side-effects,
changing the state of the device in some way.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** KnownLayout, IntoBytes, Immutable, FromBytes, FromZeros, TryFromBytes, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> ReadOnly<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ReadOnly<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ReadOnly<T>`



## safe_mmio::fields::ReadPure

*Struct*

Wrapper for a field which may safely be read with no side-effects but not written.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** KnownLayout, IntoBytes, Immutable, FromBytes, FromZeros, TryFromBytes, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ReadPure<T>) -> bool`
- **Default**
  - `fn default() -> ReadPure<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ReadPure<T>`



## safe_mmio::fields::ReadPureWrite

*Struct*

Wrapper for a field which may safely be written (with side-effects) and read with no
side-effects.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** KnownLayout, IntoBytes, Immutable, FromBytes, FromZeros, TryFromBytes, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ReadPureWrite<T>) -> bool`
- **Default**
  - `fn default() -> ReadPureWrite<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ReadPureWrite<T>`



## safe_mmio::fields::ReadWrite

*Struct*

Wrapper for a field which may safely be written and read. Reading may cause side-effects,
changing the state of the device in some way.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, KnownLayout, IntoBytes, Immutable, FromBytes, FromZeros, TryFromBytes

**Trait Implementations:**

- **Default**
  - `fn default() -> ReadWrite<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ReadWrite<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ReadWrite<T>) -> bool`



## safe_mmio::fields::WriteOnly

*Struct*

Wrapper for a field which may safely be written but not read.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** KnownLayout, IntoBytes, Immutable, FromBytes, FromZeros, TryFromBytes, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &WriteOnly<T>) -> bool`
- **Default**
  - `fn default() -> WriteOnly<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> WriteOnly<T>`



