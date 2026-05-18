**cpp_demangle**

# Module: cpp_demangle

## Contents

**Modules**

- [`ast`](#ast) - Abstract syntax tree types for mangled symbols.
- [`error`](#error) - Custom `Error` and `Result` types for the `cpp_demangle` crate.

**Structs**

- [`DemangleOptions`](#demangleoptions) - Options to control the demangling process.
- [`ParseOptions`](#parseoptions) - Options to control the parsing process.
- [`Symbol`](#symbol) - A mangled symbol that has been parsed into an AST.

**Enums**

- [`DemangleNodeType`](#demanglenodetype) - The type of a demangled AST node.

**Traits**

- [`DemangleWrite`](#demanglewrite) - Sink for demangled text that reports syntactic structure.

**Type Aliases**

- [`BorrowedSymbol`](#borrowedsymbol) - A `Symbol` which borrows the underlying storage for the mangled name.
- [`OwnedSymbol`](#ownedsymbol) - A `Symbol` which owns the underlying storage for the mangled name.

---

## cpp_demangle::BorrowedSymbol

*Type Alias*: `Symbol<&'a [u8]>`

A `Symbol` which borrows the underlying storage for the mangled name.



## cpp_demangle::DemangleNodeType

*Enum*

The type of a demangled AST node.
This is only partial, not all nodes are represented.

**Variants:**
- `Prefix` - Entering a <prefix> production
- `TemplatePrefix` - Entering a <template-prefix> production
- `TemplateArgs` - Entering a <template-args> production
- `UnqualifiedName` - Entering a <unqualified-name> production
- `TemplateParam` - Entering a <template-param> production
- `Decltype` - Entering a <decltype> production
- `DataMemberPrefix` - Entering a <data-member-prefix> production
- `NestedName` - Entering a <nested-name> production
- `VirtualTable` - Entering a <special-name> production that is a vtable.
- `__NonExhaustive` - Additional values may be added in the future. Use a

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DemangleNodeType) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &DemangleNodeType) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DemangleNodeType) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> DemangleNodeType`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::DemangleOptions

*Struct*

Options to control the demangling process.

**Methods:**

- `fn new() -> Self` - Construct a new `DemangleOptions` with the default values.
- `fn no_params(self: Self) -> Self` - Do not display function arguments.
- `fn no_return_type(self: Self) -> Self` - Do not display the function return type.
- `fn hide_expression_literal_types(self: Self) -> Self` - Hide type annotations in template value parameters.
- `fn recursion_limit(self: Self, limit: u32) -> Self` - Set the limit on recursion depth during the demangling phase. A low

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DemangleOptions`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DemangleOptions`



## cpp_demangle::DemangleWrite

*Trait*

Sink for demangled text that reports syntactic structure.

**Methods:**

- `push_demangle_node`: Called when we are entering the scope of some AST node.
- `write_string`: Same as `fmt::Write::write_str`.
- `pop_demangle_node`: Called when we are exiting the scope of some AST node for



## cpp_demangle::OwnedSymbol

*Type Alias*: `Symbol<alloc::vec::Vec<u8>>`

A `Symbol` which owns the underlying storage for the mangled name.



## cpp_demangle::ParseOptions

*Struct*

Options to control the parsing process.

**Methods:**

- `fn recursion_limit(self: Self, limit: u32) -> Self` - Set the limit on recursion depth during the parsing phase. A low

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> ParseOptions`
- **Clone**
  - `fn clone(self: &Self) -> ParseOptions`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::Symbol

*Struct*

A mangled symbol that has been parsed into an AST.

This is generic over some storage type `T` which can be either owned or
borrowed. See the `OwnedSymbol` and `BorrowedSymbol` type aliases.

**Generic Parameters:**
- T

**Methods:**

- `fn new(raw: T) -> Result<Symbol<T>>` - Given some raw storage, parse the mangled symbol from it with the default
- `fn new_with_options(raw: T, options: &ParseOptions) -> Result<Symbol<T>>` - Given some raw storage, parse the mangled symbol from it.
- `fn demangle(self: &Self) -> ::core::result::Result<String, fmt::Error>` - Demangle the symbol and return it as a String, with the default options.
- `fn demangle_with_options(self: &Self, options: &DemangleOptions) -> ::core::result::Result<String, fmt::Error>` - Demangle the symbol and return it as a String.
- `fn structured_demangle<W>(self: &Self, out: & mut W, options: &DemangleOptions) -> fmt::Result` - Demangle the symbol to a DemangleWrite, which lets the consumer be informed about
- `fn with_tail(input: &'a T) -> Result<(BorrowedSymbol<'a>, &'a [u8])>` - Parse a mangled symbol from input and return it and the trailing tail of
- `fn with_tail_and_options(input: &'a T, options: &ParseOptions) -> Result<(BorrowedSymbol<'a>, &'a [u8])>` - Parse a mangled symbol from input and return it and the trailing tail of

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Symbol<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Symbol<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: ast

Abstract syntax tree types for mangled symbols.



## Module: error

Custom `Error` and `Result` types for the `cpp_demangle` crate.



