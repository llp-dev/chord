**num_enum**

# Module: num_enum

## Contents

**Structs**

- [`TryFromPrimitiveError`](#tryfromprimitiveerror)

**Traits**

- [`FromPrimitive`](#fromprimitive)
- [`TryFromPrimitive`](#tryfromprimitive)
- [`UnsafeFromPrimitive`](#unsafefromprimitive)

---

## num_enum::FromPrimitive

*Trait*

**Methods:**

- `Primitive`
- `from_primitive`



## num_enum::TryFromPrimitive

*Trait*

**Methods:**

- `Primitive`
- `Error`
- `NAME`
- `try_from_primitive`



## num_enum::TryFromPrimitiveError

*Struct*

**Generic Parameters:**
- Enum

**Fields:**
- `number: <Enum as >::Primitive`

**Methods:**

- `fn new(number: <Enum as >::Primitive) -> Self`

**Traits:** Copy, Error, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, stream: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TryFromPrimitiveError<Enum>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TryFromPrimitiveError<Enum>`



## num_enum::UnsafeFromPrimitive

*Trait*

**Methods:**

- `Primitive`
- `from_unchecked`: Transmutes into an enum from its primitive.
- `unchecked_transmute_from`: Transmutes into an enum from its primitive.



