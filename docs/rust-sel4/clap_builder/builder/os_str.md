**clap_builder > builder > os_str**

# Module: builder::os_str

## Contents

**Structs**

- [`OsStr`](#osstr) - A UTF-8-encoded fixed string

---

## clap_builder::builder::os_str::OsStr

*Struct*

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `OsString`), enable the `string`
feature

</div>

**Methods:**

- `fn as_os_str(self: &Self) -> &std::ffi::OsStr` - Get the raw string as an `std::ffi::OsStr`
- `fn to_os_string(self: &Self) -> std::ffi::OsString` - Get the raw string as an `OsString`

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(name: &'static str) -> Self`
- **Borrow**
  - `fn borrow(self: &Self) -> &std::ffi::OsStr`
- **From**
  - `fn from(id: Str) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &OsStr) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> OsStr`
- **AsRef**
  - `fn as_ref(self: &Self) -> &std::ffi::OsStr`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &String) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &&str) -> bool`
- **From**
  - `fn from(name: &&'static std::ffi::OsStr) -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &std::path::Path`
- **From**
  - `fn from(id: &OsStr) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &OsStr) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Default**
  - `fn default() -> OsStr`
- **Deref**
  - `fn deref(self: &Self) -> &std::ffi::OsStr`
- **From**
  - `fn from(name: &'static std::ffi::OsStr) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &OsStr) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &std::ffi::OsString) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &&std::ffi::OsStr) -> bool`
- **From**
  - `fn from(name: &&'static str) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &str) -> bool`
- **From**
  - `fn from(id: &Str) -> Self`



