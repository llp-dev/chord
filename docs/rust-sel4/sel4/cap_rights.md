**sel4 > cap_rights**

# Module: cap_rights

## Contents

**Structs**

- [`CapRights`](#caprights) - Corresponds to `seL4_CapRights_t`.
- [`CapRightsBuilder`](#caprightsbuilder) - Helper for constructing [`CapRights`].

---

## sel4::cap_rights::CapRights

*Struct*

Corresponds to `seL4_CapRights_t`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: sys::seL4_CapRights) -> Self`
- `fn into_inner(self: Self) -> sys::seL4_CapRights`
- `fn inner(self: &Self) -> &sys::seL4_CapRights`
- `fn inner_mut(self: & mut Self) -> & mut sys::seL4_CapRights`
- `fn new(grant_reply: bool, grant: bool, read: bool, write: bool) -> Self`
- `fn none() -> Self`
- `fn all() -> Self`
- `fn read_write() -> Self`
- `fn read_only() -> Self`
- `fn write_only() -> Self`

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(builder: CapRightsBuilder) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> CapRights`
- **PartialEq**
  - `fn eq(self: &Self, other: &CapRights) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::cap_rights::CapRightsBuilder

*Struct*

Helper for constructing [`CapRights`].

**Methods:**

- `fn none() -> Self`
- `fn all() -> Self`
- `fn build(self: Self) -> CapRights`
- `fn grant_reply(self: Self, can: bool) -> Self`
- `fn grant(self: Self, can: bool) -> Self`
- `fn read(self: Self, can: bool) -> Self`
- `fn write(self: Self, can: bool) -> Self`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CapRightsBuilder`
- **PartialEq**
  - `fn eq(self: &Self, other: &CapRightsBuilder) -> bool`
- **Default**
  - `fn default() -> CapRightsBuilder`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



