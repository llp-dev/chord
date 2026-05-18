**cpp_demangle > error**

# Module: error

## Contents

**Enums**

- [`Error`](#error) - Errors that can occur while demangling a symbol.

**Type Aliases**

- [`Result`](#result) - A demangling result of `T` or a `cpp_demangle::error::Error`.

---

## cpp_demangle::error::Error

*Enum*

Errors that can occur while demangling a symbol.

**Variants:**
- `UnexpectedEnd` - The mangled symbol ends abruptly.
- `UnexpectedText` - The mangled symbol is not well-formed.
- `BadBackReference` - Found a back reference that is out-of-bounds of the substitution
- `BadTemplateArgReference` - Found a reference to a template arg that is either out-of-bounds, or in
- `ForwardTemplateArgReference` - Found a reference to a template arg from within the arg itself (or from
- `BadFunctionArgReference` - Found a reference to a function arg that is either out-of-bounds, or in
- `BadLeafNameReference` - Found a reference to a leaf name in a context where there is no current
- `Overflow` - An overflow or underflow would occur when parsing an integer in a
- `TooMuchRecursion` - Encountered too much recursion when demangling symbol.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Error`



## cpp_demangle::error::Result

*Type Alias*: `::core::result::Result<T, Error>`

A demangling result of `T` or a `cpp_demangle::error::Error`.



