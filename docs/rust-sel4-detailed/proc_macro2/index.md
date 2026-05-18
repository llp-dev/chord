# Crate `proc_macro2`

[![github]](https://github.com/dtolnay/proc-macro2)&ensp;[![crates-io]](https://crates.io/crates/proc-macro2)&ensp;[![docs-rs]](crate)



<br>

A wrapper around the procedural macro API of the compiler's `proc_macro`
crate. This library serves two purposes:

- **Bring proc-macro-like functionality to other contexts like build.rs and
  main.rs.** Types from `proc_macro` are entirely specific to procedural
  macros and cannot ever exist in code outside of a procedural macro.
  Meanwhile `proc_macro2` types may exist anywhere including non-macro code.
  By developing foundational libraries like [`syn`](../syn/index.md) and [`quote`](../quote/index.md) against
  `proc_macro2` rather than `proc_macro`, the procedural macro ecosystem
  becomes easily applicable to many other use cases and we avoid
  reimplementing non-macro equivalents of those libraries.

- **Make procedural macros unit testable.** As a consequence of being
  specific to procedural macros, nothing that uses `proc_macro` can be
  executed from a unit test. In order for helper libraries or components of
  a macro to be testable in isolation, they must be implemented using
  `proc_macro2`.


# Usage

The skeleton of a typical procedural macro typically looks like this:

```rust
extern crate proc_macro;

const IGNORE: &str = stringify! {
#[proc_macro_derive(MyDerive)]
};
#[cfg(wrap_proc_macro)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output: proc_macro2::TokenStream = {
        /* transform input */
        input
    };

    proc_macro::TokenStream::from(output)
}
```

If parsing with [Syn], you'll use `parse_macro_input!` instead to
propagate parse errors correctly back to the compiler when parsing fails.

# Unstable features

The default feature set of proc-macro2 tracks the most recent stable
compiler API. Functionality in `proc_macro` that is not yet stable is not
exposed by proc-macro2 by default.

To opt into the additional APIs available in the most recent nightly
compiler, the `procmacro2_semver_exempt` config flag must be passed to
rustc. We will polyfill those nightly-only APIs back to Rust 1.68.0. As
these are unstable APIs that track the nightly compiler, minor versions of
proc-macro2 may make breaking changes to them at any time.

```sh
RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build
```

Note that this must not only be done for your crate, but for any crate that
depends on your crate. This infectious nature is intentional, as it serves
as a reminder that you are outside of the normal semver guarantees.

Semver exempt methods are marked as such in the proc-macro2 documentation.

# Thread-Safety

Most types in this crate are `!Sync` because the underlying compiler
types make use of thread-local memory, meaning they cannot be accessed from
a different thread.

## Contents

- [Modules](#modules)
  - [`marker`](#marker)
  - [`parse`](#parse)
  - [`probe`](#probe)
  - [`rcvec`](#rcvec)
  - [`detection`](#detection)
  - [`extra`](#extra)
  - [`imp`](#imp)
  - [`token_stream`](#token-stream)
- [Structs](#structs)
  - [`TokenStream`](#tokenstream)
  - [`LexError`](#lexerror)
  - [`Span`](#span)
  - [`Group`](#group)
  - [`Punct`](#punct)
  - [`Ident`](#ident)
  - [`Literal`](#literal)
- [Enums](#enums)
  - [`TokenTree`](#tokentree)
  - [`Delimiter`](#delimiter)
  - [`Spacing`](#spacing)
- [Macros](#macros)
  - [`suffixed_int_literals!`](#suffixed-int-literals)
  - [`unsuffixed_int_literals!`](#unsuffixed-int-literals)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`marker`](#marker) | mod |  |
| [`parse`](#parse) | mod |  |
| [`probe`](#probe) | mod |  |
| [`rcvec`](#rcvec) | mod |  |
| [`detection`](#detection) | mod |  |
| [`extra`](#extra) | mod | Items which do not have a correspondence to any API in the proc_macro crate, but are necessary to include in proc-macro2. |
| [`imp`](#imp) | mod |  |
| [`token_stream`](#token-stream) | mod | Public implementation details for the `TokenStream` type, such as iterators. |
| [`TokenStream`](#tokenstream) | struct | An abstract stream of tokens, or more concretely a sequence of token trees. |
| [`LexError`](#lexerror) | struct | Error returned from `TokenStream::from_str`. |
| [`Span`](#span) | struct | A region of source code, along with macro expansion information. |
| [`Group`](#group) | struct | A delimited token stream. |
| [`Punct`](#punct) | struct | A `Punct` is a single punctuation character like `+`, `-` or `#`. |
| [`Ident`](#ident) | struct | A word of Rust code, which may be a keyword or legal variable name. |
| [`Literal`](#literal) | struct | A literal string (`"hello"`), byte string (`b"hello"`), character (`'a'`), byte character (`b'a'`), an integer or floating point number with or without a suffix (`1`, `1u8`, `2.3`, `2.3f32`). |
| [`TokenTree`](#tokentree) | enum | A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`). |
| [`Delimiter`](#delimiter) | enum | Describes how a sequence of token trees is delimited. |
| [`Spacing`](#spacing) | enum | Whether a `Punct` is followed immediately by another `Punct` or followed by another token or whitespace. |
| [`suffixed_int_literals!`](#suffixed-int-literals) | macro |  |
| [`unsuffixed_int_literals!`](#unsuffixed-int-literals) | macro |  |

## Modules

- [`marker`](marker/index.md)
- [`parse`](parse/index.md)
- [`probe`](probe/index.md)
- [`rcvec`](rcvec/index.md)
- [`detection`](detection/index.md)
- [`extra`](extra/index.md) — Items which do not have a correspondence to any API in the proc_macro crate,
- [`imp`](imp/index.md)
- [`token_stream`](token_stream/index.md) — Public implementation details for the `TokenStream` type, such as iterators.

## Structs

### `TokenStream`

```rust
struct TokenStream {
    inner: imp::TokenStream,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

An abstract stream of tokens, or more concretely a sequence of token trees.

This type provides interfaces for iterating over token trees and for
collecting token trees into one stream.

Token stream is both the input and output of `#[proc_macro]`,
`#[proc_macro_attribute]` and `#[proc_macro_derive]` definitions.

#### Implementations

- <span id="tokenstream-new"></span>`fn _new(inner: imp::TokenStream) -> Self` — [`TokenStream`](imp/index.md#tokenstream)

- <span id="tokenstream-new-fallback"></span>`fn _new_fallback(inner: fallback::TokenStream) -> Self`

- <span id="tokenstream-new"></span>`fn new() -> Self`

  Returns an empty `TokenStream` containing no token trees.

- <span id="tokenstream-is-empty"></span>`fn is_empty(&self) -> bool`

  Checks if this `TokenStream` is empty.

#### Trait Implementations

##### `impl Clone for TokenStream`

- <span id="tokenstream-clone"></span>`fn clone(&self) -> TokenStream` — [`TokenStream`](#tokenstream)

##### `impl Debug for TokenStream`

- <span id="tokenstream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TokenStream`

- <span id="tokenstream-default"></span>`fn default() -> Self`

##### `impl Display for TokenStream`

- <span id="tokenstream-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, tokens: I)`

##### `impl FromIterator for TokenStream`

- <span id="tokenstream-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = TokenTree>>(tokens: I) -> Self`

##### `impl FromStr for TokenStream`

- <span id="tokenstream-fromstr-type-err"></span>`type Err = LexError`

- <span id="tokenstream-fromstr-from-str"></span>`fn from_str(src: &str) -> Result<TokenStream, LexError>` — [`TokenStream`](#tokenstream), [`LexError`](#lexerror)

##### `impl IntoIterator for TokenStream`

- <span id="tokenstream-intoiterator-type-item"></span>`type Item = TokenTree`

- <span id="tokenstream-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter`

- <span id="tokenstream-intoiterator-into-iter"></span>`fn into_iter(self) -> IntoIter` — [`IntoIter`](token_stream/index.md#intoiter)

##### `impl IntoTokenStream for proc_macro2::TokenStream`

##### `impl Parse for proc_macro2::TokenStream`

##### `impl Sealed for proc_macro2::TokenStream`

##### `impl ToString for TokenStream`

- <span id="tokenstream-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::TokenStream`

- <span id="proc-macro2tokenstream-totokens-raw-string"></span>`fn raw_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](parse/index.md#cursor), [`Reject`](parse/index.md#reject)

- <span id="proc-macro2tokenstream-totokens-byte-string"></span>`fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](parse/index.md#cursor), [`Reject`](parse/index.md#reject)

##### `impl TokenStreamExt for proc_macro2::TokenStream`

- <span id="proc-macro2tokenstream-tokenstreamext-borrow"></span>`fn borrow(&self) -> &T`

### `LexError`

```rust
struct LexError {
    inner: imp::LexError,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

Error returned from `TokenStream::from_str`.

#### Implementations

- <span id="lexerror-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

#### Trait Implementations

##### `impl Debug for LexError`

- <span id="lexerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LexError`

- <span id="lexerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for LexError`

##### `impl ToString for LexError`

- <span id="lexerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Span`

```rust
struct Span {
    inner: imp::Span,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

A region of source code, along with macro expansion information.

#### Implementations

- <span id="span-new"></span>`fn _new(inner: imp::Span) -> Self` — [`Span`](imp/index.md#span)

- <span id="span-new-fallback"></span>`fn _new_fallback(inner: fallback::Span) -> Self`

- <span id="span-call-site"></span>`fn call_site() -> Self`

  The span of the invocation of the current procedural macro.

  

  Identifiers created with this span will be resolved as if they were

  written directly at the macro call location (call-site hygiene) and

  other code at the macro call site will be able to refer to them as well.

- <span id="span-mixed-site"></span>`fn mixed_site() -> Self`

  The span located at the invocation of the procedural macro, but with

  local variables, labels, and `$crate` resolved at the definition site

  of the macro. This is the same hygiene behavior as `macro_rules`.

- <span id="span-resolved-at"></span>`fn resolved_at(&self, other: Span) -> Span` — [`Span`](#span)

  Creates a new span with the same line/column information as `self` but

  that resolves symbols as though it were at `other`.

- <span id="span-located-at"></span>`fn located_at(&self, other: Span) -> Span` — [`Span`](#span)

  Creates a new span with the same name resolution behavior as `self` but

  with the line/column information of `other`.

- <span id="span-unwrap"></span>`fn unwrap(self) -> proc_macro::Span`

  Convert `proc_macro2::Span` to `proc_macro::Span`.

  

  This method is available when building with a nightly compiler, or when

  building with rustc 1.29+ *without* semver exempt features.

  

  # Panics

  

  Panics if called from outside of a procedural macro. Unlike

  `proc_macro2::Span`, the `proc_macro::Span` type can only exist within

  the context of a procedural macro invocation.

- <span id="span-join"></span>`fn join(&self, other: Span) -> Option<Span>` — [`Span`](#span)

  Create a new span encompassing `self` and `other`.

  

  Returns `None` if `self` and `other` are from different files.

  

  Warning: the underlying `proc_macro::Span::join` method is

  nightly-only. When called from within a procedural macro not using a

  nightly compiler, this method will always return `None`.

- <span id="span-source-text"></span>`fn source_text(&self) -> Option<String>`

  Returns the source text behind a span. This preserves the original

  source code, including spaces and comments. It only returns a result if

  the span corresponds to real source code.

  

  Note: The observable result of a macro should only rely on the tokens

  and not on this source text. The result of this function is a best

  effort to be used for diagnostics only.

#### Trait Implementations

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](#span)

##### `impl Copy for Span`

##### `impl Debug for Span`

- <span id="span-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Group`

```rust
struct Group {
    inner: imp::Group,
}
```

A delimited token stream.

A `Group` internally contains a `TokenStream` which is surrounded by
`Delimiter`s.

#### Implementations

- <span id="group-new"></span>`fn _new(inner: imp::Group) -> Self` — [`Group`](imp/index.md#group)

- <span id="group-new-fallback"></span>`fn _new_fallback(inner: fallback::Group) -> Self`

- <span id="group-new"></span>`fn new(delimiter: Delimiter, stream: TokenStream) -> Self` — [`Delimiter`](#delimiter), [`TokenStream`](#tokenstream)

  Creates a new `Group` with the given delimiter and token stream.

  

  This constructor will set the span for this group to

  `Span::call_site()`. To change the span you can use the `set_span`

  method below.

- <span id="group-delimiter"></span>`fn delimiter(&self) -> Delimiter` — [`Delimiter`](#delimiter)

  Returns the punctuation used as the delimiter for this group: a set of

  parentheses, square brackets, or curly braces.

- <span id="group-stream"></span>`fn stream(&self) -> TokenStream` — [`TokenStream`](#tokenstream)

  Returns the `TokenStream` of tokens that are delimited in this `Group`.

  

  Note that the returned token stream does not include the delimiter

  returned above.

- <span id="group-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

  Returns the span for the delimiters of this token stream, spanning the

  entire `Group`.

  

  ```text

  pub fn span(&self) -> Span {

             ^^^^^^^

  ```

- <span id="group-span-open"></span>`fn span_open(&self) -> Span` — [`Span`](#span)

  Returns the span pointing to the opening delimiter of this group.

  

  ```text

  pub fn span_open(&self) -> Span {

                  ^

  ```

- <span id="group-span-close"></span>`fn span_close(&self) -> Span` — [`Span`](#span)

  Returns the span pointing to the closing delimiter of this group.

  

  ```text

  pub fn span_close(&self) -> Span {

                         ^

  ```

- <span id="group-delim-span"></span>`fn delim_span(&self) -> DelimSpan` — [`DelimSpan`](extra/index.md#delimspan)

  Returns an object that holds this group's `span_open()` and

  `span_close()` together (in a more compact representation than holding

  those 2 spans individually).

- <span id="group-set-span"></span>`fn set_span(&mut self, span: Span)` — [`Span`](#span)

  Configures the span for this `Group`'s delimiters, but not its internal

  tokens.

  

  This method will **not** set the span of all the internal tokens spanned

  by this group, but rather it will only set the span of the delimiter

  tokens at the level of the `Group`.

#### Trait Implementations

##### `impl Clone for Group`

- <span id="group-clone"></span>`fn clone(&self) -> Group` — [`Group`](#group)

##### `impl Debug for Group`

- <span id="group-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Group`

- <span id="group-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = Group>>(&mut self, tokens: I)`

##### `impl Parse for proc_macro2::Group`

##### `impl Sealed for proc_macro2::Group`

##### `impl ToString for Group`

- <span id="group-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::Group`

- <span id="proc-macro2group-totokens-lex-error"></span>`fn lex_error(cursor: Cursor<'_>) -> crate::fallback::LexError` — [`Cursor`](parse/index.md#cursor)

##### `impl Token for proc_macro2::Group`

### `Punct`

```rust
struct Punct {
    ch: char,
    spacing: Spacing,
    span: Span,
}
```

A `Punct` is a single punctuation character like `+`, `-` or `#`.

Multicharacter operators like `+=` are represented as two instances of
`Punct` with different forms of `Spacing` returned.

#### Implementations

- <span id="punct-new"></span>`fn new(ch: char, spacing: Spacing) -> Self` — [`Spacing`](#spacing)

  Creates a new `Punct` from the given character and spacing.

  

  The `ch` argument must be a valid punctuation character permitted by the

  language, otherwise the function will panic.

  

  The returned `Punct` will have the default span of `Span::call_site()`

  which can be further configured with the `set_span` method below.

- <span id="punct-as-char"></span>`fn as_char(&self) -> char`

  Returns the value of this punctuation character as `char`.

- <span id="punct-spacing"></span>`fn spacing(&self) -> Spacing` — [`Spacing`](#spacing)

  Returns the spacing of this punctuation character, indicating whether

  it's immediately followed by another `Punct` in the token stream, so

  they can potentially be combined into a multicharacter operator

  (`Joint`), or it's followed by some other token or whitespace (`Alone`)

  so the operator has certainly ended.

- <span id="punct-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

  Returns the span for this punctuation character.

- <span id="punct-set-span"></span>`fn set_span(&mut self, span: Span)` — [`Span`](#span)

  Configure the span for this punctuation character.

#### Trait Implementations

##### `impl Clone for Punct`

- <span id="punct-clone"></span>`fn clone(&self) -> Punct` — [`Punct`](#punct)

##### `impl Debug for Punct`

- <span id="punct-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Punct`

- <span id="punct-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = Punct>>(&mut self, tokens: I)`

##### `impl Parse for proc_macro2::Punct`

##### `impl PunctExt for proc_macro2::Punct`

##### `impl Sealed for proc_macro2::Punct`

##### `impl ToString for Punct`

- <span id="punct-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::Punct`

- <span id="proc-macro2punct-totokens-ident-any"></span>`fn ident_any(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::Ident), Reject>` — [`Cursor`](parse/index.md#cursor), [`Ident`](#ident), [`Reject`](parse/index.md#reject)

##### `impl Token for proc_macro2::Punct`

### `Ident`

```rust
struct Ident {
    inner: imp::Ident,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

A word of Rust code, which may be a keyword or legal variable name.

An identifier consists of at least one Unicode code point, the first of
which has the XID_Start property and the rest of which have the XID_Continue
property.

- The empty string is not an identifier. Use `Option<Ident>`.
- A lifetime is not an identifier. Use `syn::Lifetime` instead.

An identifier constructed with `Ident::new` is permitted to be a Rust
keyword, though parsing one through its `Parse` implementation rejects
Rust keywords. Use `input.call(Ident::parse_any)` when parsing to match the
behaviour of `Ident::new`.

# Examples

A new ident can be created from a string using the `Ident::new` function.
A span must be provided explicitly which governs the name resolution
behavior of the resulting identifier.

```rust
use proc_macro2::{Ident, Span};

fn main() {
    let call_ident = Ident::new("calligraphy", Span::call_site());

    println!("{}", call_ident);
}
```

An ident can be interpolated into a token stream using the `quote!` macro.

```rust
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

```rust
use proc_macro2::{Ident, Span};

let ident = Ident::new("another_identifier", Span::call_site());

// Examine the ident as a string.
let ident_string = ident.to_string();
if ident_string.len() > 60 {
    println!("Very long identifier: {}", ident_string)
}
```

#### Implementations

- <span id="ident-new"></span>`fn _new(inner: imp::Ident) -> Self` — [`Ident`](imp/index.md#ident)

- <span id="ident-new-fallback"></span>`fn _new_fallback(inner: fallback::Ident) -> Self`

- <span id="ident-new"></span>`fn new(string: &str, span: Span) -> Self` — [`Span`](#span)

  Creates a new `Ident` with the given `string` as well as the specified

  `span`.

  

  The `string` argument must be a valid identifier permitted by the

  language, otherwise the function will panic.

  

  Note that `span`, currently in rustc, configures the hygiene information

  for this identifier.

  

  As of this time `Span::call_site()` explicitly opts-in to "call-site"

  hygiene meaning that identifiers created with this span will be resolved

  as if they were written directly at the location of the macro call, and

  other code at the macro call site will be able to refer to them as well.

  

  Later spans like `Span::def_site()` will allow to opt-in to

  "definition-site" hygiene meaning that identifiers created with this

  span will be resolved at the location of the macro definition and other

  code at the macro call site will not be able to refer to them.

  

  Due to the current importance of hygiene this constructor, unlike other

  tokens, requires a `Span` to be specified at construction.

  

  # Panics

  

  Panics if the input string is neither a keyword nor a legal variable

  name. If you are not sure whether the string contains an identifier and

  need to handle an error case, use

  <a href="https://docs.rs/syn/2.0/syn/fn.parse_str.html"><code

    style="padding-right:0;">syn::parse_str</code></a><code

    style="padding-left:0;">::&lt;Ident&gt;</code>

  rather than `Ident::new`.

- <span id="ident-new-raw"></span>`fn new_raw(string: &str, span: Span) -> Self` — [`Span`](#span)

  Same as `Ident::new`, but creates a raw identifier (`r#ident`). The

  `string` argument must be a valid identifier permitted by the language

  (including keywords, e.g. `fn`). Keywords which are usable in path

  segments (e.g. `self`, `super`) are not supported, and will cause a

  panic.

- <span id="ident-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

  Returns the span of this `Ident`.

- <span id="ident-set-span"></span>`fn set_span(&mut self, span: Span)` — [`Span`](#span)

  Configures the span of this `Ident`, possibly changing its hygiene

  context.

#### Trait Implementations

##### `impl Clone for Ident`

- <span id="ident-clone"></span>`fn clone(&self) -> Ident` — [`Ident`](#ident)

##### `impl Debug for Ident`

- <span id="ident-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Ident`

- <span id="ident-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ident`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = Ident>>(&mut self, tokens: I)`

##### `impl Hash for Ident`

- <span id="ident-hash"></span>`fn hash<H: Hasher>(&self, hasher: &mut H)`

##### `impl IdentExt for proc_macro2::Ident`

##### `impl IdentFragment for proc_macro2::Ident`

##### `impl Ord for Ident`

- <span id="ident-ord-cmp"></span>`fn cmp(&self, other: &Ident) -> Ordering` — [`Ident`](#ident)

##### `impl Parse for proc_macro2::Ident`

##### `impl PartialEq for Ident`

- <span id="ident-partialeq-eq"></span>`fn eq(&self, other: &Ident) -> bool` — [`Ident`](#ident)

##### `impl PartialOrd for Ident`

- <span id="ident-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Ident) -> Option<Ordering>` — [`Ident`](#ident)

##### `impl Sealed for proc_macro2::Ident`

##### `impl ToString for Ident`

- <span id="ident-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::Ident`

- <span id="proc-macro2ident-totokens-ident"></span>`fn ident(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::Ident), Reject>` — [`Cursor`](parse/index.md#cursor), [`Ident`](#ident), [`Reject`](parse/index.md#reject)

##### `impl Token for proc_macro2::Ident`

### `Literal`

```rust
struct Literal {
    inner: imp::Literal,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

A literal string (`"hello"`), byte string (`b"hello"`), character (`'a'`),
byte character (`b'a'`), an integer or floating point number with or without
a suffix (`1`, `1u8`, `2.3`, `2.3f32`).

Boolean literals like `true` and `false` do not belong here, they are
`Ident`s.

#### Implementations

- <span id="literal-new"></span>`fn _new(inner: imp::Literal) -> Self` — [`Literal`](imp/index.md#literal)

- <span id="literal-new-fallback"></span>`fn _new_fallback(inner: fallback::Literal) -> Self`

- <span id="literal-u8-suffixed"></span>`fn u8_suffixed(n: u8) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-u16-suffixed"></span>`fn u16_suffixed(n: u16) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-u32-suffixed"></span>`fn u32_suffixed(n: u32) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-u64-suffixed"></span>`fn u64_suffixed(n: u64) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-u128-suffixed"></span>`fn u128_suffixed(n: u128) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-usize-suffixed"></span>`fn usize_suffixed(n: usize) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-i8-suffixed"></span>`fn i8_suffixed(n: i8) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-i16-suffixed"></span>`fn i16_suffixed(n: i16) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-i32-suffixed"></span>`fn i32_suffixed(n: i32) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-i64-suffixed"></span>`fn i64_suffixed(n: i64) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-i128-suffixed"></span>`fn i128_suffixed(n: i128) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-isize-suffixed"></span>`fn isize_suffixed(n: isize) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed integer literal with the specified value.

  

  This function will create an integer like `1u32` where the integer

  value specified is the first part of the token and the integral is

  also suffixed at the end. Literals created from negative numbers may

  not survive roundtrips through `TokenStream` or strings and may be

  broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-u8-unsuffixed"></span>`fn u8_unsuffixed(n: u8) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-u16-unsuffixed"></span>`fn u16_unsuffixed(n: u16) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-u32-unsuffixed"></span>`fn u32_unsuffixed(n: u32) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-u64-unsuffixed"></span>`fn u64_unsuffixed(n: u64) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-u128-unsuffixed"></span>`fn u128_unsuffixed(n: u128) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-usize-unsuffixed"></span>`fn usize_unsuffixed(n: usize) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-i8-unsuffixed"></span>`fn i8_unsuffixed(n: i8) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-i16-unsuffixed"></span>`fn i16_unsuffixed(n: i16) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-i32-unsuffixed"></span>`fn i32_unsuffixed(n: i32) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-i64-unsuffixed"></span>`fn i64_unsuffixed(n: i64) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-i128-unsuffixed"></span>`fn i128_unsuffixed(n: i128) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-isize-unsuffixed"></span>`fn isize_unsuffixed(n: isize) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed integer literal with the specified value.

  

  This function will create an integer like `1` where the integer

  value specified is the first part of the token. No suffix is

  specified on this token, meaning that invocations like

  `Literal::i8_unsuffixed(1)` are equivalent to

  `Literal::u32_unsuffixed(1)`. Literals created from negative numbers

  may not survive roundtrips through `TokenStream` or strings and may

  be broken into two tokens (`-` and positive literal).

  

  Literals created through this method have the `Span::call_site()`

  span by default, which can be configured with the `set_span` method

  below.

- <span id="literal-f64-unsuffixed"></span>`fn f64_unsuffixed(f: f64) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed floating-point literal.

  

  This constructor is similar to those like `Literal::i8_unsuffixed` where

  the float's value is emitted directly into the token but no suffix is

  used, so it may be inferred to be a `f64` later in the compiler.

  Literals created from negative numbers may not survive round-trips

  through `TokenStream` or strings and may be broken into two tokens (`-`

  and positive literal).

  

  # Panics

  

  This function requires that the specified float is finite, for example

  if it is infinity or NaN this function will panic.

- <span id="literal-f64-suffixed"></span>`fn f64_suffixed(f: f64) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed floating-point literal.

  

  This constructor will create a literal like `1.0f64` where the value

  specified is the preceding part of the token and `f64` is the suffix of

  the token. This token will always be inferred to be an `f64` in the

  compiler. Literals created from negative numbers may not survive

  round-trips through `TokenStream` or strings and may be broken into two

  tokens (`-` and positive literal).

  

  # Panics

  

  This function requires that the specified float is finite, for example

  if it is infinity or NaN this function will panic.

- <span id="literal-f32-unsuffixed"></span>`fn f32_unsuffixed(f: f32) -> Literal` — [`Literal`](#literal)

  Creates a new unsuffixed floating-point literal.

  

  This constructor is similar to those like `Literal::i8_unsuffixed` where

  the float's value is emitted directly into the token but no suffix is

  used, so it may be inferred to be a `f64` later in the compiler.

  Literals created from negative numbers may not survive round-trips

  through `TokenStream` or strings and may be broken into two tokens (`-`

  and positive literal).

  

  # Panics

  

  This function requires that the specified float is finite, for example

  if it is infinity or NaN this function will panic.

- <span id="literal-f32-suffixed"></span>`fn f32_suffixed(f: f32) -> Literal` — [`Literal`](#literal)

  Creates a new suffixed floating-point literal.

  

  This constructor will create a literal like `1.0f32` where the value

  specified is the preceding part of the token and `f32` is the suffix of

  the token. This token will always be inferred to be an `f32` in the

  compiler. Literals created from negative numbers may not survive

  round-trips through `TokenStream` or strings and may be broken into two

  tokens (`-` and positive literal).

  

  # Panics

  

  This function requires that the specified float is finite, for example

  if it is infinity or NaN this function will panic.

- <span id="literal-string"></span>`fn string(string: &str) -> Literal` — [`Literal`](#literal)

  String literal.

- <span id="literal-character"></span>`fn character(ch: char) -> Literal` — [`Literal`](#literal)

  Character literal.

- <span id="literal-byte-character"></span>`fn byte_character(byte: u8) -> Literal` — [`Literal`](#literal)

  Byte character literal.

- <span id="literal-byte-string"></span>`fn byte_string(bytes: &[u8]) -> Literal` — [`Literal`](#literal)

  Byte string literal.

- <span id="literal-c-string"></span>`fn c_string(string: &CStr) -> Literal` — [`Literal`](#literal)

  C string literal.

- <span id="literal-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

  Returns the span encompassing this literal.

- <span id="literal-set-span"></span>`fn set_span(&mut self, span: Span)` — [`Span`](#span)

  Configures the span associated for this literal.

- <span id="literal-subspan"></span>`fn subspan<R: RangeBounds<usize>>(&self, range: R) -> Option<Span>` — [`Span`](#span)

  Returns a `Span` that is a subset of `self.span()` containing only

  the source bytes in range `range`. Returns `None` if the would-be

  trimmed span is outside the bounds of `self`.

  

  Warning: the underlying `proc_macro::Literal::subspan` method is

  nightly-only. When called from within a procedural macro not using a

  nightly compiler, this method will always return `None`.

#### Trait Implementations

##### `impl Clone for Literal`

- <span id="literal-clone"></span>`fn clone(&self) -> Literal` — [`Literal`](#literal)

##### `impl Debug for Literal`

- <span id="literal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Literal`

- <span id="literal-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = Literal>>(&mut self, tokens: I)`

##### `impl FromStr for Literal`

- <span id="literal-fromstr-type-err"></span>`type Err = LexError`

- <span id="literal-fromstr-from-str"></span>`fn from_str(repr: &str) -> Result<Self, LexError>` — [`LexError`](#lexerror)

##### `impl Parse for proc_macro2::Literal`

##### `impl Sealed for proc_macro2::Literal`

##### `impl ToString for Literal`

- <span id="literal-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::Literal`

##### `impl Token for proc_macro2::Literal`

## Enums

### `TokenTree`

```rust
enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}
```

A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`).

#### Variants

- **`Group`**

  A token stream surrounded by bracket delimiters.

- **`Ident`**

  An identifier.

- **`Punct`**

  A single punctuation character (`+`, `,`, `$`, etc.).

- **`Literal`**

  A literal character (`'a'`), string (`"hello"`), number (`2.3`), etc.

#### Implementations

- <span id="tokentree-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

  Returns the span of this tree, delegating to the `span` method of

  the contained token or a delimited stream.

- <span id="tokentree-set-span"></span>`fn set_span(&mut self, span: Span)` — [`Span`](#span)

  Configures the span for *only this token*.

  

  Note that if this token is a `Group` then this method will not configure

  the span of each of the internal tokens, this will simply delegate to

  the `set_span` method of each variant.

#### Trait Implementations

##### `impl Clone for TokenTree`

- <span id="tokentree-clone"></span>`fn clone(&self) -> TokenTree` — [`TokenTree`](#tokentree)

##### `impl Debug for TokenTree`

- <span id="tokentree-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TokenTree`

- <span id="tokentree-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, tokens: I)`

##### `impl FromIterator for TokenStream`

- <span id="tokenstream-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = TokenTree>>(tokens: I) -> Self`

##### `impl Parse for proc_macro2::TokenTree`

##### `impl Sealed for proc_macro2::TokenTree`

##### `impl ToString for TokenTree`

- <span id="tokentree-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::TokenTree`

- <span id="proc-macro2tokentree-totokens-string"></span>`fn string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](parse/index.md#cursor), [`Reject`](parse/index.md#reject)

##### `impl Token for proc_macro2::TokenTree`

### `Delimiter`

```rust
enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
    None,
}
```

Describes how a sequence of token trees is delimited.

#### Variants

- **`Parenthesis`**

  `( ... )`

- **`Brace`**

  `{ ... }`

- **`Bracket`**

  `[ ... ]`

- **`None`**

  `∅ ... ∅`
  
  An invisible delimiter, that may, for example, appear around tokens
  coming from a "macro variable" `$var`. It is important to preserve
  operator priorities in cases like `$var * 3` where `$var` is `1 + 2`.
  Invisible delimiters may not survive roundtrip of a token stream through
  a string.
  
  <div class="warning">
  
  Note: rustc currently can ignore the grouping of tokens delimited by `None` in the output
  of a proc_macro. Only `None`-delimited groups created by a macro_rules macro in the input
  of a proc_macro macro are preserved, and only in very specific circumstances.
  Any `None`-delimited groups (re)created by a proc_macro will therefore not preserve
  operator priorities as indicated above. The other `Delimiter` variants should be used
  instead in this context. This is a rustc bug. For details, see
  [rust-lang/rust#67062](https://github.com/rust-lang/rust/issues/67062).
  
  </div>

#### Trait Implementations

##### `impl Clone for Delimiter`

- <span id="delimiter-clone"></span>`fn clone(&self) -> Delimiter` — [`Delimiter`](#delimiter)

##### `impl Copy for Delimiter`

##### `impl Debug for Delimiter`

- <span id="delimiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Delimiter`

##### `impl PartialEq for Delimiter`

- <span id="delimiter-partialeq-eq"></span>`fn eq(&self, other: &Delimiter) -> bool` — [`Delimiter`](#delimiter)

##### `impl StructuralPartialEq for Delimiter`

### `Spacing`

```rust
enum Spacing {
    Alone,
    Joint,
}
```

Whether a `Punct` is followed immediately by another `Punct` or followed by
another token or whitespace.

#### Variants

- **`Alone`**

  E.g. `+` is `Alone` in `+ =`, `+ident` or `+()`.

- **`Joint`**

  E.g. `+` is `Joint` in `+=` or `'` is `Joint` in `'#`.
  
  Additionally, single quote `'` can join with identifiers to form
  lifetimes `'ident`.

#### Trait Implementations

##### `impl Clone for Spacing`

- <span id="spacing-clone"></span>`fn clone(&self) -> Spacing` — [`Spacing`](#spacing)

##### `impl Copy for Spacing`

##### `impl Debug for Spacing`

- <span id="spacing-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Spacing`

##### `impl PartialEq for Spacing`

- <span id="spacing-partialeq-eq"></span>`fn eq(&self, other: &Spacing) -> bool` — [`Spacing`](#spacing)

##### `impl StructuralPartialEq for Spacing`

## Macros

### `suffixed_int_literals!`

### `unsuffixed_int_literals!`

