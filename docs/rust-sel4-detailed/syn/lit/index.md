*[syn](../index.md) / [lit](index.md)*

---

# Module `lit`

## Contents

- [Modules](#modules)
  - [`debug_impls`](#debug-impls)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
  - [`value`](#value)
- [Structs](#structs)
  - [`LitStr`](#litstr)
  - [`LitByteStr`](#litbytestr)
  - [`LitCStr`](#litcstr)
  - [`LitByte`](#litbyte)
  - [`LitChar`](#litchar)
  - [`LitRepr`](#litrepr)
  - [`LitInt`](#litint)
  - [`LitIntRepr`](#litintrepr)
  - [`LitFloat`](#litfloat)
  - [`LitFloatRepr`](#litfloatrepr)
  - [`LitBool`](#litbool)
- [Enums](#enums)
  - [`Lit`](#lit)
- [Macros](#macros)
  - [`lit_extra_traits!`](#lit-extra-traits)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`debug_impls`](#debug-impls) | mod |  |
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`value`](#value) | mod |  |
| [`LitStr`](#litstr) | struct | A UTF-8 string literal: `"foo"`. |
| [`LitByteStr`](#litbytestr) | struct | A byte string literal: `b"foo"`. |
| [`LitCStr`](#litcstr) | struct | A nul-terminated C-string literal: `c"foo"`. |
| [`LitByte`](#litbyte) | struct | A byte literal: `b'f'`. |
| [`LitChar`](#litchar) | struct | A character literal: `'a'`. |
| [`LitRepr`](#litrepr) | struct |  |
| [`LitInt`](#litint) | struct | An integer literal: `1` or `1u16`. |
| [`LitIntRepr`](#litintrepr) | struct |  |
| [`LitFloat`](#litfloat) | struct | A floating point literal: `1f64` or `1.0e10f64`. |
| [`LitFloatRepr`](#litfloatrepr) | struct |  |
| [`LitBool`](#litbool) | struct | A boolean literal: `true` or `false`. |
| [`Lit`](#lit) | enum | A Rust literal such as a string or integer or boolean. |
| [`lit_extra_traits!`](#lit-extra-traits) | macro |  |

## Modules

- [`debug_impls`](debug_impls/index.md)
- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)
- [`value`](value/index.md)

## Structs

### `LitStr`

```rust
struct LitStr {
    repr: alloc::boxed::Box<LitRepr>,
}
```

A UTF-8 string literal: `"foo"`.

#### Implementations

- <span id="cratelitlitstr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitStr`

- <span id="litstr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitStr`

- <span id="cratelitlitstr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitStr`

##### `impl Hash for LitStr`

- <span id="litstr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitStr`

- <span id="cratelitlitstr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitStr`

- <span id="litstr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitStr`

##### `impl Spanned for LitStr`

- <span id="litstr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitStr`

- <span id="cratelitlitstr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitStr`

### `LitByteStr`

```rust
struct LitByteStr {
    repr: alloc::boxed::Box<LitRepr>,
}
```

A byte string literal: `b"foo"`.

#### Implementations

- <span id="cratelitlitbytestr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitByteStr`

- <span id="litbytestr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByteStr`

##### `impl Hash for LitByteStr`

- <span id="litbytestr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitByteStr`

- <span id="litbytestr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitByteStr`

##### `impl Spanned for LitByteStr`

- <span id="litbytestr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByteStr`

### `LitCStr`

```rust
struct LitCStr {
    repr: alloc::boxed::Box<LitRepr>,
}
```

A nul-terminated C-string literal: `c"foo"`.

#### Implementations

- <span id="cratelitlitcstr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitCStr`

- <span id="litcstr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitCStr`

- <span id="cratelitlitcstr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitCStr`

##### `impl Hash for LitCStr`

- <span id="litcstr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitCStr`

- <span id="cratelitlitcstr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitCStr`

- <span id="litcstr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitCStr`

##### `impl Spanned for LitCStr`

- <span id="litcstr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitCStr`

- <span id="cratelitlitcstr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitCStr`

### `LitByte`

```rust
struct LitByte {
    repr: alloc::boxed::Box<LitRepr>,
}
```

A byte literal: `b'f'`.

#### Implementations

- <span id="cratelitlitbyte-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitByte`

- <span id="litbyte-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitByte`

- <span id="cratelitlitbyte-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByte`

##### `impl Hash for LitByte`

- <span id="litbyte-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitByte`

- <span id="cratelitlitbyte-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitByte`

- <span id="litbyte-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitByte`

##### `impl Spanned for LitByte`

- <span id="litbyte-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitByte`

- <span id="cratelitlitbyte-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByte`

### `LitChar`

```rust
struct LitChar {
    repr: alloc::boxed::Box<LitRepr>,
}
```

A character literal: `'a'`.

#### Implementations

- <span id="cratelitlitchar-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitChar`

- <span id="litchar-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitChar`

- <span id="cratelitlitchar-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitChar`

##### `impl Hash for LitChar`

- <span id="litchar-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitChar`

- <span id="cratelitlitchar-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitChar`

- <span id="litchar-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitChar`

##### `impl Spanned for LitChar`

- <span id="litchar-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitChar`

- <span id="cratelitlitchar-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitChar`

### `LitRepr`

```rust
struct LitRepr {
    token: proc_macro2::Literal,
    suffix: alloc::boxed::Box<str>,
}
```

#### Trait Implementations

##### `impl Clone for LitRepr`

- <span id="litrepr-clone"></span>`fn clone(&self) -> Self`

### `LitInt`

```rust
struct LitInt {
    repr: alloc::boxed::Box<LitIntRepr>,
}
```

An integer literal: `1` or `1u16`.

#### Implementations

- <span id="cratelitlitint-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitInt`

- <span id="litint-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitInt`

- <span id="cratelitlitint-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitInt`

- <span id="litint-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitInt`

##### `impl Hash for LitInt`

- <span id="litint-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitInt`

- <span id="cratelitlitint-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitInt`

- <span id="litint-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitInt`

##### `impl Spanned for LitInt`

- <span id="litint-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToString for LitInt`

- <span id="litint-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lit::LitInt`

- <span id="cratelitlitint-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitInt`

### `LitIntRepr`

```rust
struct LitIntRepr {
    token: proc_macro2::Literal,
    digits: alloc::boxed::Box<str>,
    suffix: alloc::boxed::Box<str>,
}
```

#### Trait Implementations

##### `impl Clone for LitIntRepr`

- <span id="litintrepr-clone"></span>`fn clone(&self) -> Self`

### `LitFloat`

```rust
struct LitFloat {
    repr: alloc::boxed::Box<LitFloatRepr>,
}
```

A floating point literal: `1f64` or `1.0e10f64`.

Must be finite. May not be infinite or NaN.

#### Implementations

- <span id="cratelitlitfloat-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitFloat`

- <span id="litfloat-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitFloat`

- <span id="cratelitlitfloat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitFloat`

- <span id="litfloat-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitFloat`

##### `impl Hash for LitFloat`

- <span id="litfloat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitFloat`

- <span id="cratelitlitfloat-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitFloat`

- <span id="litfloat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitFloat`

##### `impl Spanned for LitFloat`

- <span id="litfloat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToString for LitFloat`

- <span id="litfloat-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lit::LitFloat`

- <span id="cratelitlitfloat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitFloat`

### `LitFloatRepr`

```rust
struct LitFloatRepr {
    token: proc_macro2::Literal,
    digits: alloc::boxed::Box<str>,
    suffix: alloc::boxed::Box<str>,
}
```

#### Trait Implementations

##### `impl Clone for LitFloatRepr`

- <span id="litfloatrepr-clone"></span>`fn clone(&self) -> Self`

### `LitBool`

```rust
struct LitBool {
    pub value: bool,
    pub span: proc_macro2::Span,
}
```

A boolean literal: `true` or `false`.

#### Implementations

- <span id="cratelitlitbool-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::LitBool`

- <span id="cratelitbool-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::lit::LitBool`

- <span id="cratelitlitbool-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitBool`

##### `impl Hash for crate::LitBool`

- <span id="cratelitbool-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::LitBool`

- <span id="cratelitlitbool-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::LitBool`

- <span id="cratelitbool-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitBool`

##### `impl Spanned for LitBool`

- <span id="litbool-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::lit::LitBool`

- <span id="cratelitlitbool-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitBool`

## Enums

### `Lit`

```rust
enum Lit {
    Str(LitStr),
    ByteStr(LitByteStr),
    CStr(LitCStr),
    Byte(LitByte),
    Char(LitChar),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    Verbatim(proc_macro2::Literal),
}
```

A Rust literal such as a string or integer or boolean.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Str`**

  A UTF-8 string literal: `"foo"`.

- **`ByteStr`**

  A byte string literal: `b"foo"`.

- **`CStr`**

  A nul-terminated C-string literal: `c"foo"`.

- **`Byte`**

  A byte literal: `b'f'`.

- **`Char`**

  A character literal: `'a'`.

- **`Int`**

  An integer literal: `1` or `1u16`.

- **`Float`**

  A floating point literal: `1f64` or `1.0e10f64`.
  
  Must be finite. May not be infinite or NaN.

- **`Bool`**

  A boolean literal: `true` or `false`.

- **`Verbatim`**

  A raw token literal not interpreted by Syn.

#### Implementations

- <span id="cratelitlit-new"></span>`fn new(token: Literal) -> Self`

  Interpret a Syn literal from a proc-macro2 literal.

- <span id="cratelitlit-from-str"></span>`fn from_str(token: Literal, repr: &str) -> Self`

- <span id="cratelitlit-suffix"></span>`fn suffix(&self) -> &str`

- <span id="cratelitlit-span"></span>`fn span(&self) -> Span`

- <span id="cratelitlit-set-span"></span>`fn set_span(&mut self, span: Span)`

#### Trait Implementations

##### `impl Clone for crate::Lit`

- <span id="cratelit-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Lit`

- <span id="cratelit-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Lit`

##### `impl Hash for crate::Lit`

- <span id="cratelit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::lit::Lit`

- <span id="cratelitlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Lit`

- <span id="cratelit-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Lit`

##### `impl Spanned for Lit`

- <span id="lit-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Lit`

- <span id="lit-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl Token for crate::lit::Lit`

## Macros

### `lit_extra_traits!`

