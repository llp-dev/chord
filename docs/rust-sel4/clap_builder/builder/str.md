**clap_builder > builder > str**

# Module: builder::str

## Contents

**Structs**

- [`Str`](#str) - A UTF-8-encoded fixed string

---

## clap_builder::builder::str::Str

*Struct*

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `String`), enable the `string`
feature

</div>

**Methods:**

- `fn as_str(self: &Self) -> &str` - Get the raw string of the `Str`

**Traits:** Eq

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &std::path::Path`
- **From**
  - `fn from(id: &Str) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &str`
- **PartialEq**
  - `fn eq(self: &Self, other: &Str) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &std::ffi::OsStr`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(name: Id) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &String) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &std::ffi::OsStr) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &str) -> bool`
- **From**
  - `fn from(name: &&'static str) -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Ord**
  - `fn cmp(self: &Self, other: &Str) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Str`
- **Borrow**
  - `fn borrow(self: &Self) -> &str`
- **From**
  - `fn from(name: &'static str) -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &str`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Str) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Default**
  - `fn default() -> Str`
- **PartialEq**
  - `fn eq(self: &Self, other: &Id) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &&std::ffi::OsStr) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &&str) -> bool`



