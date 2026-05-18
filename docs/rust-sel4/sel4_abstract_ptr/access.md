**sel4_abstract_ptr > access**

# Module: access

## Contents

**Structs**

- [`NoAccess`](#noaccess) - Zero-sized marker type that grants no access.
- [`ReadOnly`](#readonly) - Zero-sized marker type for allowing only read access.
- [`ReadWrite`](#readwrite) - Zero-sized marker type for allowing both read and write access.
- [`WriteOnly`](#writeonly) - Zero-sized marker type for allowing only write access.

**Traits**

- [`Access`](#access) - Sealed trait that is implemented for the types in this module.
- [`Copyable`](#copyable) - Implemented for access types that permit copying of `AbstractRef`.
- [`Readable`](#readable) - Helper trait that is implemented by [`ReadWrite`] and [`ReadOnly`].
- [`RestrictAccess`](#restrictaccess) - A trait for restricting one [`Access`] type to another [`Access`] type.
- [`Writable`](#writable) - Helper trait that is implemented by [`ReadWrite`] and [`WriteOnly`].

---

## sel4_abstract_ptr::access::Access

*Trait*

Sealed trait that is implemented for the types in this module.



## sel4_abstract_ptr::access::Copyable

*Trait*

Implemented for access types that permit copying of `AbstractRef`.



## sel4_abstract_ptr::access::NoAccess

*Struct*

Zero-sized marker type that grants no access.

**Unit Struct**

**Traits:** RestrictAccess, Access, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> NoAccess`
- **Default**
  - `fn default() -> NoAccess`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_abstract_ptr::access::ReadOnly

*Struct*

Zero-sized marker type for allowing only read access.

**Unit Struct**

**Traits:** RestrictAccess, RestrictAccess, RestrictAccess, RestrictAccess, Copy, Access

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ReadOnly`
- **Default**
  - `fn default() -> ReadOnly`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_abstract_ptr::access::ReadWrite

*Struct*

Zero-sized marker type for allowing both read and write access.

**Unit Struct**

**Traits:** RestrictAccess, Copy, Access

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ReadWrite`
- **Default**
  - `fn default() -> ReadWrite`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_abstract_ptr::access::Readable

*Trait*

Helper trait that is implemented by [`ReadWrite`] and [`ReadOnly`].



## sel4_abstract_ptr::access::RestrictAccess

*Trait*

A trait for restricting one [`Access`] type to another [`Access`] type.

Restricting `Self` to `To` results in [`Self::Restricted`].

Restriction is a symmetric operation which is denoted by ∩, as it is the intersection of permissions.
The following table holds:

| `Self`        | `To`          | `Self` ∩ `To` |
| ------------- | ------------- | ------------- |
| `T`           | `T`           | `T`           |
| [`ReadWrite`] | `T`           | `T`           |
| [`NoAccess`]  | `T`           | [`NoAccess`]  |
| [`ReadOnly`]  | [`WriteOnly`] | [`NoAccess`]  |

**Methods:**

- `Restricted`: The resulting [`Access`] type of `Self` restricted to `To`.



## sel4_abstract_ptr::access::Writable

*Trait*

Helper trait that is implemented by [`ReadWrite`] and [`WriteOnly`].



## sel4_abstract_ptr::access::WriteOnly

*Struct*

Zero-sized marker type for allowing only write access.

**Unit Struct**

**Traits:** RestrictAccess, Copy, Access, RestrictAccess, RestrictAccess, RestrictAccess

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> WriteOnly`
- **Default**
  - `fn default() -> WriteOnly`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



