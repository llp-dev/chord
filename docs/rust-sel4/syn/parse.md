**syn > parse**

# Module: parse

## Contents

**Modules**

- [`discouraged`](#discouraged) - Extensions to the parsing API with niche applicability.

**Structs**

- [`Nothing`](#nothing) - An empty syntax tree node that consumes no tokens when parsed.
- [`ParseBuffer`](#parsebuffer) - Cursor position within a buffered token stream.
- [`StepCursor`](#stepcursor) - Cursor state associated with speculative parsing.

**Enums**

- [`Unexpected`](#unexpected)

**Functions**

- [`advance_step_cursor`](#advance_step_cursor)
- [`cell_clone`](#cell_clone)
- [`err_unexpected_token`](#err_unexpected_token)
- [`get_unexpected`](#get_unexpected)
- [`inner_unexpected`](#inner_unexpected)
- [`new_parse_buffer`](#new_parse_buffer)
- [`parse_scoped`](#parse_scoped)
- [`span_of_unexpected_ignoring_nones`](#span_of_unexpected_ignoring_nones)
- [`tokens_to_parse_buffer`](#tokens_to_parse_buffer)

**Traits**

- [`Parse`](#parse) - Parsing interface implemented by all types that can be parsed in a default
- [`Parser`](#parser) - Parser that can parse Rust tokens into a particular syntax tree node.

**Type Aliases**

- [`ParseStream`](#parsestream) - Input to a Syn parser function.

---

## syn::parse::Nothing

*Struct*

An empty syntax tree node that consumes no tokens when parsed.

This is useful for attribute macros that want to ensure they are not
provided any attribute args.

```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::parse::Nothing;

# const IGNORE: &str = stringify! {
#[proc_macro_attribute]
# };
pub fn my_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);

    /* ... */
#   TokenStream::new()
}
```

```text
error: unexpected token
 --> src/main.rs:3:19
  |
3 | #[my_attr(asdf)]
  |           ^^^^
```

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(_input: ParseStream) -> Result<Self>`



## syn::parse::Parse

*Trait*

Parsing interface implemented by all types that can be parsed in a default
way from a token stream.

Refer to the [module documentation] for details about implementing and using
the `Parse` trait.

[module documentation]: self

**Methods:**

- `parse`



## syn::parse::ParseBuffer

*Struct*

Cursor position within a buffered token stream.

This type is more commonly used through the type alias [`ParseStream`] which
is an alias for `&ParseBuffer`.

`ParseStream` is the input type for all parser functions in Syn. They have
the signature `fn(ParseStream) -> Result<T>`.

## Calling a parser function

There is no public way to construct a `ParseBuffer`. Instead, if you are
looking to invoke a parser function that requires `ParseStream` as input,
you will need to go through one of the public parsing entry points.

- The [`parse_macro_input!`] macro if parsing input of a procedural macro;
- One of [the `syn::parse*` functions][syn-parse]; or
- A method of the [`Parser`] trait.

[`parse_macro_input!`]: crate::parse_macro_input!
[syn-parse]: self#the-synparse-functions

**Generic Parameters:**
- 'a

**Fields:**
- `scope: proc_macro2::Span`
- `cell: core::cell::Cell<crate::buffer::Cursor<'static>>`
- `marker: core::marker::PhantomData<crate::buffer::Cursor<'a>>`
- `unexpected: core::cell::Cell<Option<alloc::rc::Rc<core::cell::Cell<Unexpected>>>>`

**Methods:**

- `fn parse<T>(self: &Self) -> Result<T>` - Parses a syntax tree node of type `T`, advancing the position of our
- `fn call<T>(self: &'a Self, function: fn(...)) -> Result<T>` - Calls the given parser function to parse a syntax tree node of type `T`
- `fn peek<T>(self: &Self, token: T) -> bool` - Looks at the next token in the parse stream to determine whether it
- `fn peek2<T>(self: &Self, token: T) -> bool` - Looks at the second-next token in the parse stream.
- `fn peek3<T>(self: &Self, token: T) -> bool` - Looks at the third-next token in the parse stream.
- `fn parse_terminated<T, P>(self: &'a Self, parser: fn(...), separator: P) -> Result<Punctuated<T, <P as >::Token>>` - Parses zero or more occurrences of `T` separated by punctuation of type
- `fn is_empty(self: &Self) -> bool` - Returns whether there are no more tokens remaining to be parsed from
- `fn lookahead1(self: &Self) -> Lookahead1<'a>` - Constructs a helper for peeking at the next token in this stream and
- `fn fork(self: &Self) -> Self` - Forks a parse stream so that parsing tokens out of either the original
- `fn error<T>(self: &Self, message: T) -> Error` - Triggers an error at the current position of the parse stream.
- `fn step<F, R>(self: &Self, function: F) -> Result<R>` - Speculatively parses tokens from this parse stream, advancing the
- `fn span(self: &Self) -> Span` - Returns the `Span` of the next token in the parse stream, or
- `fn cursor(self: &Self) -> Cursor<'a>` - Provides low-level access to the token representation underlying this
- `fn check_unexpected(self: &Self) -> Result<()>`

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Speculative**
  - `fn advance_to(self: &Self, fork: &Self)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AnyDelimiter**
  - `fn parse_any_delimiter(self: &Self) -> Result<(Delimiter, DelimSpan, ParseBuffer)>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## syn::parse::ParseStream

*Type Alias*: `&'a ParseBuffer<'a>`

Input to a Syn parser function.

See the methods of this type under the documentation of [`ParseBuffer`]. For
an overview of parsing in Syn, refer to the [module documentation].

[module documentation]: self



## syn::parse::Parser

*Trait*

Parser that can parse Rust tokens into a particular syntax tree node.

Refer to the [module documentation] for details about parsing in Syn.

[module documentation]: self

**Methods:**

- `Output`
- `parse2`: Parse a proc-macro2 token stream into the chosen syntax tree node.
- `parse`: Parse tokens of source code into the chosen syntax tree node.
- `parse_str`: Parse a string of Rust code into the chosen syntax tree node.



## syn::parse::StepCursor

*Struct*

Cursor state associated with speculative parsing.

This type is the input of the closure provided to [`ParseStream::step`].

[`ParseStream::step`]: ParseBuffer::step

# Example

```
use proc_macro2::TokenTree;
use syn::Result;
use syn::parse::ParseStream;

// This function advances the stream past the next occurrence of `@`. If
// no `@` is present in the stream, the stream position is unchanged and
// an error is returned.
fn skip_past_next_at(input: ParseStream) -> Result<()> {
    input.step(|cursor| {
        let mut rest = *cursor;
        while let Some((tt, next)) = rest.token_tree() {
            match &tt {
                TokenTree::Punct(punct) if punct.as_char() == '@' => {
                    return Ok(((), next));
                }
                _ => rest = next,
            }
        }
        Err(cursor.error("no `@` was found after this point"))
    })
}
#
# fn remainder_after_skipping_past_next_at(
#     input: ParseStream,
# ) -> Result<proc_macro2::TokenStream> {
#     skip_past_next_at(input)?;
#     input.parse()
# }
#
# use syn::parse::Parser;
# let remainder = remainder_after_skipping_past_next_at
#     .parse_str("a @ b c")
#     .unwrap();
# assert_eq!(remainder.to_string(), "b c");
```

**Generic Parameters:**
- 'c
- 'a

**Fields:**
- `scope: proc_macro2::Span`
- `cursor: crate::buffer::Cursor<'c>`
- `marker: core::marker::PhantomData<fn(...)>`

**Methods:**

- `fn error<T>(self: Self, message: T) -> Error` - Triggers an error at the current position of the parse stream.

**Traits:** Copy

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::parse::Unexpected

*Enum*

**Variants:**
- `None`
- `Some(proc_macro2::Span, proc_macro2::Delimiter)`
- `Chain(alloc::rc::Rc<core::cell::Cell<Unexpected>>)`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::parse::advance_step_cursor

*Function*

```rust
fn advance_step_cursor<'c, 'a>(proof: StepCursor<'c, 'a>, to: crate::buffer::Cursor<'c>) -> crate::buffer::Cursor<'a>
```



## syn::parse::cell_clone

*Function*

```rust
fn cell_clone<T>(cell: &core::cell::Cell<T>) -> T
```



## Module: discouraged

Extensions to the parsing API with niche applicability.



## syn::parse::err_unexpected_token

*Function*

```rust
fn err_unexpected_token(span: proc_macro2::Span, delimiter: proc_macro2::Delimiter) -> Error
```



## syn::parse::get_unexpected

*Function*

```rust
fn get_unexpected(buffer: &ParseBuffer) -> alloc::rc::Rc<core::cell::Cell<Unexpected>>
```



## syn::parse::inner_unexpected

*Function*

```rust
fn inner_unexpected(buffer: &ParseBuffer) -> (alloc::rc::Rc<core::cell::Cell<Unexpected>>, Option<(proc_macro2::Span, proc_macro2::Delimiter)>)
```



## syn::parse::new_parse_buffer

*Function*

```rust
fn new_parse_buffer(scope: proc_macro2::Span, cursor: crate::buffer::Cursor, unexpected: alloc::rc::Rc<core::cell::Cell<Unexpected>>) -> ParseBuffer
```



## syn::parse::parse_scoped

*Function*

```rust
fn parse_scoped<F>(f: F, scope: proc_macro2::Span, tokens: proc_macro2::TokenStream) -> Result<<F as >::Output>
```



## syn::parse::span_of_unexpected_ignoring_nones

*Function*

```rust
fn span_of_unexpected_ignoring_nones(cursor: crate::buffer::Cursor) -> Option<(proc_macro2::Span, proc_macro2::Delimiter)>
```



## syn::parse::tokens_to_parse_buffer

*Function*

```rust
fn tokens_to_parse_buffer(tokens: &crate::buffer::TokenBuffer) -> ParseBuffer
```



