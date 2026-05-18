**proc_macro2**

# Module: proc_macro2

## Contents

**Modules**

- [`extra`](#extra) - Items which do not have a correspondence to any API in the proc_macro crate,
- [`token_stream`](#token_stream) - Public implementation details for the `TokenStream` type, such as iterators.

**Structs**

- [`Group`](#group) - A delimited token stream.
- [`Ident`](#ident) - A word of Rust code, which may be a keyword or legal variable name.
- [`LexError`](#lexerror) - Error returned from `TokenStream::from_str`.
- [`Literal`](#literal) - A literal string (`"hello"`), byte string (`b"hello"`), character (`'a'`),
- [`Punct`](#punct) - A `Punct` is a single punctuation character like `+`, `-` or `#`.
- [`Span`](#span) - A region of source code, along with macro expansion information.
- [`TokenStream`](#tokenstream) - An abstract stream of tokens, or more concretely a sequence of token trees.

**Enums**

- [`Delimiter`](#delimiter) - Describes how a sequence of token trees is delimited.
- [`Spacing`](#spacing) - Whether a `Punct` is followed immediately by another `Punct` or followed by
- [`TokenTree`](#tokentree) - A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`).

---

## proc_macro2::Delimiter

*Enum*

Describes how a sequence of token trees is delimited.

**Variants:**
- `Parenthesis` - `( ... )`
- `Brace` - `{ ... }`
- `Bracket` - `[ ... ]`
- `None` - `∅ ... ∅`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Delimiter) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Delimiter`



## proc_macro2::Group

*Struct*

A delimited token stream.

A `Group` internally contains a `TokenStream` which is surrounded by
`Delimiter`s.

**Methods:**

- `fn new(delimiter: Delimiter, stream: TokenStream) -> Self` - Creates a new `Group` with the given delimiter and token stream.
- `fn delimiter(self: &Self) -> Delimiter` - Returns the punctuation used as the delimiter for this group: a set of
- `fn stream(self: &Self) -> TokenStream` - Returns the `TokenStream` of tokens that are delimited in this `Group`.
- `fn span(self: &Self) -> Span` - Returns the span for the delimiters of this token stream, spanning the
- `fn span_open(self: &Self) -> Span` - Returns the span pointing to the opening delimiter of this group.
- `fn span_close(self: &Self) -> Span` - Returns the span pointing to the closing delimiter of this group.
- `fn delim_span(self: &Self) -> DelimSpan` - Returns an object that holds this group's `span_open()` and
- `fn set_span(self: & mut Self, span: Span)` - Configures the span for this `Group`'s delimiters, but not its internal

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Group`



## proc_macro2::Ident

*Struct*

A word of Rust code, which may be a keyword or legal variable name.

An identifier consists of at least one Unicode code point, the first of
which has the XID_Start property and the rest of which have the XID_Continue
property.

- The empty string is not an identifier. Use `Option<Ident>`.
- A lifetime is not an identifier. Use `syn::Lifetime` instead.

An identifier constructed with `Ident::new` is permitted to be a Rust
keyword, though parsing one through its [`Parse`] implementation rejects
Rust keywords. Use `input.call(Ident::parse_any)` when parsing to match the
behaviour of `Ident::new`.

[`Parse`]: https://docs.rs/syn/2.0/syn/parse/trait.Parse.html

# Examples

A new ident can be created from a string using the `Ident::new` function.
A span must be provided explicitly which governs the name resolution
behavior of the resulting identifier.

```
use proc_macro2::{Ident, Span};

fn main() {
    let call_ident = Ident::new("calligraphy", Span::call_site());

    println!("{}", call_ident);
}
```

An ident can be interpolated into a token stream using the `quote!` macro.

```
use proc_macro2::{Ident, Span};
use quote::quote;

fn main() {
    let ident = Ident::new("demo", Span::call_site());

    // Create a variable binding whose name is this ident.
    let expanded = quote! { let #ident = 10; };

    // Create a variable binding with a slightly different name.
    let temp_ident = Ident::new(&format!("new_{}", ident), Span::call_site());
    let expanded = quote! { let #temp_ident = 10; };
}
```

A string representation of the ident is available through the `to_string()`
method.

```
# use proc_macro2::{Ident, Span};
#
# let ident = Ident::new("another_identifier", Span::call_site());
#
// Examine the ident as a string.
let ident_string = ident.to_string();
if ident_string.len() > 60 {
    println!("Very long identifier: {}", ident_string)
}
```

**Methods:**

- `fn new(string: &str, span: Span) -> Self` - Creates a new `Ident` with the given `string` as well as the specified
- `fn new_raw(string: &str, span: Span) -> Self` - Same as `Ident::new`, but creates a raw identifier (`r#ident`). The
- `fn span(self: &Self) -> Span` - Returns the span of this `Ident`.
- `fn set_span(self: & mut Self, span: Span)` - Configures the span of this `Ident`, possibly changing its hygiene

**Traits:** Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Ident) -> Option<Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, hasher: & mut H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Ident) -> Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &T) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &Ident) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Ident`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## proc_macro2::LexError

*Struct*

Error returned from `TokenStream::from_str`.

**Methods:**

- `fn span(self: &Self) -> Span`

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## proc_macro2::Literal

*Struct*

A literal string (`"hello"`), byte string (`b"hello"`), character (`'a'`),
byte character (`b'a'`), an integer or floating point number with or without
a suffix (`1`, `1u8`, `2.3`, `2.3f32`).

Boolean literals like `true` and `false` do not belong here, they are
`Ident`s.

**Methods:**

- `fn u8_suffixed(n: u8) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn u16_suffixed(n: u16) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn u32_suffixed(n: u32) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn u64_suffixed(n: u64) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn u128_suffixed(n: u128) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn usize_suffixed(n: usize) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn i8_suffixed(n: i8) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn i16_suffixed(n: i16) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn i32_suffixed(n: i32) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn i64_suffixed(n: i64) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn i128_suffixed(n: i128) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn isize_suffixed(n: isize) -> Literal` - Creates a new suffixed integer literal with the specified value.
- `fn u8_unsuffixed(n: u8) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn u16_unsuffixed(n: u16) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn u32_unsuffixed(n: u32) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn u64_unsuffixed(n: u64) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn u128_unsuffixed(n: u128) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn usize_unsuffixed(n: usize) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn i8_unsuffixed(n: i8) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn i16_unsuffixed(n: i16) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn i32_unsuffixed(n: i32) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn i64_unsuffixed(n: i64) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn i128_unsuffixed(n: i128) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn isize_unsuffixed(n: isize) -> Literal` - Creates a new unsuffixed integer literal with the specified value.
- `fn f64_unsuffixed(f: f64) -> Literal` - Creates a new unsuffixed floating-point literal.
- `fn f64_suffixed(f: f64) -> Literal` - Creates a new suffixed floating-point literal.
- `fn f32_unsuffixed(f: f32) -> Literal` - Creates a new unsuffixed floating-point literal.
- `fn f32_suffixed(f: f32) -> Literal` - Creates a new suffixed floating-point literal.
- `fn string(string: &str) -> Literal` - String literal.
- `fn character(ch: char) -> Literal` - Character literal.
- `fn byte_character(byte: u8) -> Literal` - Byte character literal.
- `fn byte_string(bytes: &[u8]) -> Literal` - Byte string literal.
- `fn c_string(string: &CStr) -> Literal` - C string literal.
- `fn span(self: &Self) -> Span` - Returns the span encompassing this literal.
- `fn set_span(self: & mut Self, span: Span)` - Configures the span associated for this literal.
- `fn subspan<R>(self: &Self, range: R) -> Option<Span>` - Returns a `Span` that is a subset of `self.span()` containing only

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Literal`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FromStr**
  - `fn from_str(repr: &str) -> Result<Self, LexError>`



## proc_macro2::Punct

*Struct*

A `Punct` is a single punctuation character like `+`, `-` or `#`.

Multicharacter operators like `+=` are represented as two instances of
`Punct` with different forms of `Spacing` returned.

**Methods:**

- `fn new(ch: char, spacing: Spacing) -> Self` - Creates a new `Punct` from the given character and spacing.
- `fn as_char(self: &Self) -> char` - Returns the value of this punctuation character as `char`.
- `fn spacing(self: &Self) -> Spacing` - Returns the spacing of this punctuation character, indicating whether
- `fn span(self: &Self) -> Span` - Returns the span for this punctuation character.
- `fn set_span(self: & mut Self, span: Span)` - Configure the span for this punctuation character.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Punct`



## proc_macro2::Spacing

*Enum*

Whether a `Punct` is followed immediately by another `Punct` or followed by
another token or whitespace.

**Variants:**
- `Alone` - E.g. `+` is `Alone` in `+ =`, `+ident` or `+()`.
- `Joint` - E.g. `+` is `Joint` in `+=` or `'` is `Joint` in `'#`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Spacing) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Spacing`



## proc_macro2::Span

*Struct*

A region of source code, along with macro expansion information.

**Methods:**

- `fn call_site() -> Self` - The span of the invocation of the current procedural macro.
- `fn mixed_site() -> Self` - The span located at the invocation of the procedural macro, but with
- `fn resolved_at(self: &Self, other: Span) -> Span` - Creates a new span with the same line/column information as `self` but
- `fn located_at(self: &Self, other: Span) -> Span` - Creates a new span with the same name resolution behavior as `self` but
- `fn unwrap(self: Self) -> proc_macro::Span` - Convert `proc_macro2::Span` to `proc_macro::Span`.
- `fn join(self: &Self, other: Span) -> Option<Span>` - Create a new span encompassing `self` and `other`.
- `fn source_text(self: &Self) -> Option<String>` - Returns the source text behind a span. This preserves the original

**Traits:** Copy

**Trait Implementations:**

- **From**
  - `fn from(proc_span: proc_macro::Span) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Span`



## proc_macro2::TokenStream

*Struct*

An abstract stream of tokens, or more concretely a sequence of token trees.

This type provides interfaces for iterating over token trees and for
collecting token trees into one stream.

Token stream is both the input and output of `#[proc_macro]`,
`#[proc_macro_attribute]` and `#[proc_macro_derive]` definitions.

**Methods:**

- `fn new() -> Self` - Returns an empty `TokenStream` containing no token trees.
- `fn is_empty(self: &Self) -> bool` - Checks if this `TokenStream` is empty.

**Trait Implementations:**

- **From**
  - `fn from(inner: proc_macro::TokenStream) -> Self`
- **FromIterator**
  - `fn from_iter<I>(streams: I) -> Self`
- **FromStr**
  - `fn from_str(src: &str) -> Result<TokenStream, LexError>`
- **Extend**
  - `fn extend<I>(self: & mut Self, tokens: I)`
- **Extend**
  - `fn extend<I>(self: & mut Self, tokens: I)`
- **Extend**
  - `fn extend<I>(self: & mut Self, streams: I)`
- **FromIterator**
  - `fn from_iter<I>(tokens: I) -> Self`
- **Extend**
  - `fn extend<I>(self: & mut Self, tokens: I)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> TokenStream`
- **Extend**
  - `fn extend<I>(self: & mut Self, tokens: I)`
- **IntoIterator**
  - `fn into_iter(self: Self) -> IntoIter`
- **Extend**
  - `fn extend<I>(self: & mut Self, tokens: I)`
- **From**
  - `fn from(token: TokenTree) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## proc_macro2::TokenTree

*Enum*

A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`).

**Variants:**
- `Group(Group)` - A token stream surrounded by bracket delimiters.
- `Ident(Ident)` - An identifier.
- `Punct(Punct)` - A single punctuation character (`+`, `,`, `$`, etc.).
- `Literal(Literal)` - A literal character (`'a'`), string (`"hello"`), number (`2.3`), etc.

**Methods:**

- `fn span(self: &Self) -> Span` - Returns the span of this tree, delegating to the `span` method of
- `fn set_span(self: & mut Self, span: Span)` - Configures the span for *only this token*.

**Trait Implementations:**

- **From**
  - `fn from(g: Ident) -> Self`
- **From**
  - `fn from(g: Group) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> TokenTree`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(g: Literal) -> Self`
- **From**
  - `fn from(g: Punct) -> Self`



## Module: extra

Items which do not have a correspondence to any API in the proc_macro crate,
but are necessary to include in proc-macro2.



## Module: token_stream

Public implementation details for the `TokenStream` type, such as iterators.



