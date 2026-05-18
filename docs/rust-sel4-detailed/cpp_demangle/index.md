# Crate `cpp_demangle`

This crate can parse a C++ “mangled” linker symbol name into a Rust value
describing what the name refers to: a variable, a function, a virtual table,
etc. The description type implements functions such as `demangle()`,
producing human-readable text describing the mangled name. Debuggers and
profilers can use this crate to provide more meaningful output.

C++ requires the compiler to choose names for linker symbols consistently
across compilation units, so that two compilation units that have seen the
same declarations can pair up definitions in one unit with references in
another.  Almost all platforms other than Microsoft Windows follow the
[Itanium C++ ABI][itanium]'s rules for this.

For example, suppose a C++ compilation unit has the definition:

```c++
namespace space {
  int foo(int x, int y) { return x+y; }
}
```

The Itanium C++ ABI specifies that the linker symbol for that function must
be named `_ZN5space3fooEii`. This crate can parse that name into a Rust
value representing its structure. That Rust value can be `demangle()`d to the
string `space::foo(int, int)`, which is more meaningful to the C++
developer.

## Contents

- [Modules](#modules)
  - [`logging`](#logging)
  - [`ast`](#ast)
  - [`error`](#error)
  - [`index_str`](#index-str)
  - [`subs`](#subs)
- [Structs](#structs)
  - [`ParseOptions`](#parseoptions)
  - [`DemangleOptions`](#demangleoptions)
  - [`Symbol`](#symbol)
- [Enums](#enums)
  - [`DemangleNodeType`](#demanglenodetype)
- [Traits](#traits)
  - [`DemangleWrite`](#demanglewrite)
- [Type Aliases](#type-aliases)
  - [`OwnedSymbol`](#ownedsymbol)
  - [`BorrowedSymbol`](#borrowedsymbol)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`logging`](#logging) | mod |  |
| [`ast`](#ast) | mod | Abstract syntax tree types for mangled symbols. |
| [`error`](#error) | mod | Custom `Error` and `Result` types for the `cpp_demangle` crate. |
| [`index_str`](#index-str) | mod | Provides the `IndexStr` type to keep track of a substring's index into its original string is. |
| [`subs`](#subs) | mod | Types dealing with the substitutions table. |
| [`ParseOptions`](#parseoptions) | struct | Options to control the parsing process. |
| [`DemangleOptions`](#demangleoptions) | struct | Options to control the demangling process. |
| [`Symbol`](#symbol) | struct | A mangled symbol that has been parsed into an AST. |
| [`DemangleNodeType`](#demanglenodetype) | enum | The type of a demangled AST node. |
| [`DemangleWrite`](#demanglewrite) | trait | Sink for demangled text that reports syntactic structure. |
| [`OwnedSymbol`](#ownedsymbol) | type | A `Symbol` which owns the underlying storage for the mangled name. |
| [`BorrowedSymbol`](#borrowedsymbol) | type | A `Symbol` which borrows the underlying storage for the mangled name. |

## Modules

- [`logging`](logging/index.md)
- [`ast`](ast/index.md) — Abstract syntax tree types for mangled symbols.
- [`error`](error/index.md) — Custom `Error` and `Result` types for the `cpp_demangle` crate.
- [`index_str`](index_str/index.md) — Provides the `IndexStr` type to keep track of a substring's index into its
- [`subs`](subs/index.md) — Types dealing with the substitutions table.

## Structs

### `ParseOptions`

```rust
struct ParseOptions {
    recursion_limit: Option<core::num::NonZeroU32>,
}
```

Options to control the parsing process.

#### Implementations

- <span id="parseoptions-recursion-limit"></span>`fn recursion_limit(self, limit: u32) -> Self`

  Set the limit on recursion depth during the parsing phase. A low

  limit will cause valid symbols to be rejected, but a high limit may

  allow pathological symbols to overflow the stack during parsing.

  The default value is 96, which will not overflow the stack even in

  a debug build.

#### Trait Implementations

##### `impl Clone for ParseOptions`

- <span id="parseoptions-clone"></span>`fn clone(&self) -> ParseOptions` — [`ParseOptions`](#parseoptions)

##### `impl Copy for ParseOptions`

##### `impl Debug for ParseOptions`

- <span id="parseoptions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ParseOptions`

- <span id="parseoptions-default"></span>`fn default() -> ParseOptions` — [`ParseOptions`](#parseoptions)

### `DemangleOptions`

```rust
struct DemangleOptions {
    no_params: bool,
    no_return_type: bool,
    hide_expression_literal_types: bool,
    recursion_limit: Option<core::num::NonZeroU32>,
}
```

Options to control the demangling process.

#### Implementations

- <span id="demangleoptions-new"></span>`fn new() -> Self`

  Construct a new `DemangleOptions` with the default values.

- <span id="demangleoptions-no-params"></span>`fn no_params(self) -> Self`

  Do not display function arguments.

- <span id="demangleoptions-no-return-type"></span>`fn no_return_type(self) -> Self`

  Do not display the function return type.

- <span id="demangleoptions-hide-expression-literal-types"></span>`fn hide_expression_literal_types(self) -> Self`

  Hide type annotations in template value parameters.

  These are not needed to distinguish template instances

  so this can make it easier to match user-provided

  template instance names.

- <span id="demangleoptions-recursion-limit"></span>`fn recursion_limit(self, limit: u32) -> Self`

  Set the limit on recursion depth during the demangling phase. A low

  limit will cause valid symbols to be rejected, but a high limit may

  allow pathological symbols to overflow the stack during demangling.

  The default value is 128.

#### Trait Implementations

##### `impl Clone for DemangleOptions`

- <span id="demangleoptions-clone"></span>`fn clone(&self) -> DemangleOptions` — [`DemangleOptions`](#demangleoptions)

##### `impl Copy for DemangleOptions`

##### `impl Debug for DemangleOptions`

- <span id="demangleoptions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DemangleOptions`

- <span id="demangleoptions-default"></span>`fn default() -> DemangleOptions` — [`DemangleOptions`](#demangleoptions)

### `Symbol<T>`

```rust
struct Symbol<T> {
    raw: T,
    substitutions: subs::SubstitutionTable,
    parsed: ast::MangledName,
}
```

A mangled symbol that has been parsed into an AST.

This is generic over some storage type `T` which can be either owned or
borrowed. See the `OwnedSymbol` and `BorrowedSymbol` type aliases.

#### Implementations

- <span id="symbol-new"></span>`fn new(raw: T) -> Result<Symbol<T>>` — [`Result`](error/index.md#result), [`Symbol`](#symbol)

  Given some raw storage, parse the mangled symbol from it with the default

  options.

  

  ```rust

  use cpp_demangle::Symbol;

  

  // First, something easy :)

  

  let mangled = b"_ZN5space3fooEibc";

  

  let sym = Symbol::new(&mangled[..])

      .expect("Could not parse mangled symbol!");

  

  let demangled = sym.demangle().unwrap();

  assert_eq!(demangled, "space::foo(int, bool, char)");

  

  // Now let's try something a little more complicated!

  

  let mangled =

      b"__Z28JS_GetPropertyDescriptorByIdP9JSContextN2JS6HandleIP8JSObjectEENS2_I4jsidEENS1_13MutableHandleINS1_18PropertyDescriptorEEE";

  

  let sym = Symbol::new(&mangled[..])

      .expect("Could not parse mangled symbol!");

  

  let demangled = sym.demangle().unwrap();

  assert_eq!(

      demangled,

      "JS_GetPropertyDescriptorById(JSContext*, JS::Handle<JSObject*>, JS::Handle<jsid>, JS::MutableHandle<JS::PropertyDescriptor>)"

  );

  ```

- <span id="symbol-new-with-options"></span>`fn new_with_options(raw: T, options: &ParseOptions) -> Result<Symbol<T>>` — [`ParseOptions`](#parseoptions), [`Result`](error/index.md#result), [`Symbol`](#symbol)

  Given some raw storage, parse the mangled symbol from it.

  

  ```rust

  use cpp_demangle::{ParseOptions, Symbol};

  

  // First, something easy :)

  

  let mangled = b"_ZN5space3fooEibc";

  

  let parse_options = ParseOptions::default()

      .recursion_limit(1024);

  

  let sym = Symbol::new_with_options(&mangled[..], &parse_options)

      .expect("Could not parse mangled symbol!");

  

  let demangled = sym.demangle().unwrap();

  assert_eq!(demangled, "space::foo(int, bool, char)");

  

  // Now let's try something a little more complicated!

  

  let mangled =

      b"__Z28JS_GetPropertyDescriptorByIdP9JSContextN2JS6HandleIP8JSObjectEENS2_I4jsidEENS1_13MutableHandleINS1_18PropertyDescriptorEEE";

  

  let sym = Symbol::new(&mangled[..])

      .expect("Could not parse mangled symbol!");

  

  let demangled = sym.demangle().unwrap();

  assert_eq!(

      demangled,

      "JS_GetPropertyDescriptorById(JSContext*, JS::Handle<JSObject*>, JS::Handle<jsid>, JS::MutableHandle<JS::PropertyDescriptor>)"

  );

  ```

- <span id="symbol-demangle"></span>`fn demangle(&self) -> ::core::result::Result<String, fmt::Error>`

  Demangle the symbol and return it as a String, with the default options.

  

  ```rust

  use cpp_demangle::{DemangleOptions, Symbol};

  

  let mangled = b"_ZN5space3fooEibc";

  

  let sym = Symbol::new(&mangled[..])

      .expect("Could not parse mangled symbol!");

  

  let demangled = sym.demangle().unwrap();

  assert_eq!(demangled, "space::foo(int, bool, char)");

  ```

- <span id="symbol-demangle-with-options"></span>`fn demangle_with_options(&self, options: &DemangleOptions) -> ::core::result::Result<String, fmt::Error>` — [`DemangleOptions`](#demangleoptions)

  Demangle the symbol and return it as a String.

  

  ```rust

  use cpp_demangle::{DemangleOptions, Symbol};

  

  let mangled = b"_ZN5space3fooEibc";

  

  let sym = Symbol::new(&mangled[..])

      .expect("Could not parse mangled symbol!");

  

  let options = DemangleOptions::default();

  let demangled = sym.demangle_with_options(&options).unwrap();

  assert_eq!(demangled, "space::foo(int, bool, char)");

  ```

- <span id="symbol-structured-demangle"></span>`fn structured_demangle<W: DemangleWrite>(&self, out: &mut W, options: &DemangleOptions) -> fmt::Result` — [`DemangleOptions`](#demangleoptions)

  Demangle the symbol to a DemangleWrite, which lets the consumer be informed about

  syntactic structure.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Symbol<T>`

- <span id="symbol-clone"></span>`fn clone(&self) -> Symbol<T>` — [`Symbol`](#symbol)

##### `impl<T: fmt::Debug> Debug for Symbol<T>`

- <span id="symbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::PartialEq> PartialEq for Symbol<T>`

- <span id="symbol-partialeq-eq"></span>`fn eq(&self, other: &Symbol<T>) -> bool` — [`Symbol`](#symbol)

##### `impl<T> StructuralPartialEq for Symbol<T>`

## Enums

### `DemangleNodeType`

```rust
enum DemangleNodeType {
    Prefix,
    TemplatePrefix,
    TemplateArgs,
    UnqualifiedName,
    TemplateParam,
    Decltype,
    DataMemberPrefix,
    NestedName,
    VirtualTable,
    __NonExhaustive,
}
```

The type of a demangled AST node.
This is only partial, not all nodes are represented.

#### Variants

- **`Prefix`**

  Entering a <prefix> production

- **`TemplatePrefix`**

  Entering a <template-prefix> production

- **`TemplateArgs`**

  Entering a <template-args> production

- **`UnqualifiedName`**

  Entering a <unqualified-name> production

- **`TemplateParam`**

  Entering a <template-param> production

- **`Decltype`**

  Entering a <decltype> production

- **`DataMemberPrefix`**

  Entering a <data-member-prefix> production

- **`NestedName`**

  Entering a <nested-name> production

- **`VirtualTable`**

  Entering a <special-name> production that is a vtable.

- **`__NonExhaustive`**

  Additional values may be added in the future. Use a
  _ pattern for compatibility.

#### Trait Implementations

##### `impl Clone for DemangleNodeType`

- <span id="demanglenodetype-clone"></span>`fn clone(&self) -> DemangleNodeType` — [`DemangleNodeType`](#demanglenodetype)

##### `impl Copy for DemangleNodeType`

##### `impl Debug for DemangleNodeType`

- <span id="demanglenodetype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DemangleNodeType`

##### `impl Hash for DemangleNodeType`

- <span id="demanglenodetype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DemangleNodeType`

- <span id="demanglenodetype-ord-cmp"></span>`fn cmp(&self, other: &DemangleNodeType) -> cmp::Ordering` — [`DemangleNodeType`](#demanglenodetype)

##### `impl PartialEq for DemangleNodeType`

- <span id="demanglenodetype-partialeq-eq"></span>`fn eq(&self, other: &DemangleNodeType) -> bool` — [`DemangleNodeType`](#demanglenodetype)

##### `impl PartialOrd for DemangleNodeType`

- <span id="demanglenodetype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DemangleNodeType) -> option::Option<cmp::Ordering>` — [`DemangleNodeType`](#demanglenodetype)

##### `impl StructuralPartialEq for DemangleNodeType`

## Traits

### `DemangleWrite`

```rust
trait DemangleWrite { ... }
```

Sink for demangled text that reports syntactic structure.

#### Required Methods

- `fn write_string(&mut self, s: &str) -> fmt::Result`

  Same as `fmt::Write::write_str`.

#### Provided Methods

- `fn push_demangle_node(&mut self, _: DemangleNodeType)`

  Called when we are entering the scope of some AST node.

- `fn pop_demangle_node(&mut self)`

  Called when we are exiting the scope of some AST node for

#### Implementors

- `W`

## Type Aliases

### `OwnedSymbol`

```rust
type OwnedSymbol = Symbol<alloc::vec::Vec<u8>>;
```

A `Symbol` which owns the underlying storage for the mangled name.

### `BorrowedSymbol<'a>`

```rust
type BorrowedSymbol<'a> = Symbol<&'a [u8]>;
```

A `Symbol` which borrows the underlying storage for the mangled name.

