**syn**

# Module: syn

## Contents

**Modules**

- [`attr`](#attr)
- [`bigint`](#bigint)
- [`buffer`](#buffer) - A stably addressed token buffer supporting efficient traversal based on a
- [`classify`](#classify)
- [`custom_keyword`](#custom_keyword)
- [`custom_punctuation`](#custom_punctuation)
- [`data`](#data)
- [`derive`](#derive)
- [`drops`](#drops)
- [`error`](#error)
- [`expr`](#expr)
- [`ext`](#ext) - Extension traits to provide parsing methods on foreign types.
- [`file`](#file)
- [`fixup`](#fixup)
- [`gen`](#gen)
- [`generics`](#generics)
- [`group`](#group)
- [`ident`](#ident)
- [`item`](#item)
- [`lifetime`](#lifetime)
- [`lit`](#lit)
- [`lookahead`](#lookahead)
- [`mac`](#mac)
- [`macros`](#macros)
- [`meta`](#meta) - Facility for interpreting structured content inside of an `Attribute`.
- [`op`](#op)
- [`parse`](#parse) - Parsing interface for parsing a token stream into a syntax tree node.
- [`parse_macro_input`](#parse_macro_input)
- [`parse_quote`](#parse_quote)
- [`pat`](#pat)
- [`path`](#path)
- [`precedence`](#precedence)
- [`print`](#print)
- [`punctuated`](#punctuated) - A punctuated sequence of syntax tree nodes separated by punctuation.
- [`restriction`](#restriction)
- [`sealed`](#sealed)
- [`span`](#span)
- [`spanned`](#spanned) - A trait that can provide the `Span` of the complete contents of a syntax
- [`stmt`](#stmt)
- [`thread`](#thread)
- [`token`](#token) - Tokens representing Rust punctuation, keywords, and delimiters.
- [`ty`](#ty)
- [`verbatim`](#verbatim)
- [`whitespace`](#whitespace)

**Macros**

- [`Token`](#token) - A type-macro that expands to the name of the Rust type representation of a
- [`braced`](#braced) - Parse a set of curly braces and expose their content to subsequent parsers.
- [`bracketed`](#bracketed) - Parse a set of square brackets and expose their content to subsequent
- [`custom_keyword`](#custom_keyword) - Define a type that supports parsing and printing a given identifier as if it
- [`custom_punctuation`](#custom_punctuation) - Define a type that supports parsing and printing a multi-character symbol
- [`parenthesized`](#parenthesized) - Parse a set of parentheses and expose their content to subsequent parsers.
- [`parse_macro_input`](#parse_macro_input) - Parse the input TokenStream of a macro, triggering a compile error if the
- [`parse_quote`](#parse_quote) - Quasi-quotation macro that accepts input like the [`quote!`] macro but uses
- [`parse_quote_spanned`](#parse_quote_spanned) - This macro is [`parse_quote!`] + [`quote_spanned!`][quote::quote_spanned].

**Functions**

- [`parse`](#parse) - Parse tokens of source code into the chosen syntax tree node.
- [`parse2`](#parse2) - Parse a proc-macro2 token stream into the chosen syntax tree node.
- [`parse_file`](#parse_file) - Parse the content of a file of Rust code.
- [`parse_str`](#parse_str) - Parse a string of Rust code into the chosen syntax tree node.

---

## syn::Token

*Declarative Macro*

A type-macro that expands to the name of the Rust type representation of a
given token.

As a type, `Token!` is commonly used in the type of struct fields, the type
of a `let` statement, or in turbofish for a `parse` function.

```
use syn::{Ident, Token};
use syn::parse::{Parse, ParseStream, Result};

// `struct Foo;`
pub struct UnitStruct {
    struct_token: Token![struct],
    ident: Ident,
    semi_token: Token![;],
}

impl Parse for UnitStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_token: Token![struct] = input.parse()?;
        let ident: Ident = input.parse()?;
        let semi_token = input.parse::<Token![;]>()?;
        Ok(UnitStruct { struct_token, ident, semi_token })
    }
}
```

As an expression, `Token!` is used for peeking tokens or instantiating
tokens from a span.

```
# use syn::{Ident, Token};
# use syn::parse::{Parse, ParseStream, Result};
#
# struct UnitStruct {
#     struct_token: Token![struct],
#     ident: Ident,
#     semi_token: Token![;],
# }
#
# impl Parse for UnitStruct {
#     fn parse(input: ParseStream) -> Result<Self> {
#         unimplemented!()
#     }
# }
#
fn make_unit_struct(name: Ident) -> UnitStruct {
    let span = name.span();
    UnitStruct {
        struct_token: Token![struct](span),
        ident: name,
        semi_token: Token![;](span),
    }
}

# fn parse(input: ParseStream) -> Result<()> {
if input.peek(Token![struct]) {
    let unit_struct: UnitStruct = input.parse()?;
    /* ... */
}
# Ok(())
# }
```

See the [token module] documentation for details and examples.

[token module]: crate::token

```rust
macro_rules! Token {
    [abstract] => { ... };
    [as] => { ... };
    [async] => { ... };
    [auto] => { ... };
    [await] => { ... };
    [become] => { ... };
    [box] => { ... };
    [break] => { ... };
    [const] => { ... };
    [continue] => { ... };
    [crate] => { ... };
    [default] => { ... };
    [do] => { ... };
    [dyn] => { ... };
    [else] => { ... };
    [enum] => { ... };
    [extern] => { ... };
    [final] => { ... };
    [fn] => { ... };
    [for] => { ... };
    [if] => { ... };
    [impl] => { ... };
    [in] => { ... };
    [let] => { ... };
    [loop] => { ... };
    [macro] => { ... };
    [match] => { ... };
    [mod] => { ... };
    [move] => { ... };
    [mut] => { ... };
    [override] => { ... };
    [priv] => { ... };
    [pub] => { ... };
    [raw] => { ... };
    [ref] => { ... };
    [return] => { ... };
    [Self] => { ... };
    [self] => { ... };
    [static] => { ... };
    [struct] => { ... };
    [super] => { ... };
    [trait] => { ... };
    [try] => { ... };
    [type] => { ... };
    [typeof] => { ... };
    [union] => { ... };
    [unsafe] => { ... };
    [unsized] => { ... };
    [use] => { ... };
    [virtual] => { ... };
    [where] => { ... };
    [while] => { ... };
    [yield] => { ... };
    [&] => { ... };
    [&&] => { ... };
    [&=] => { ... };
    [@] => { ... };
    [^] => { ... };
    [^=] => { ... };
    [:] => { ... };
    [,] => { ... };
    [$] => { ... };
    [.] => { ... };
    [..] => { ... };
    [...] => { ... };
    [..=] => { ... };
    [=] => { ... };
    [==] => { ... };
    [=>] => { ... };
    [>=] => { ... };
    [>] => { ... };
    [<-] => { ... };
    [<=] => { ... };
    [<] => { ... };
    [-] => { ... };
    [-=] => { ... };
    [!=] => { ... };
    [!] => { ... };
    [|] => { ... };
    [|=] => { ... };
    [||] => { ... };
    [::] => { ... };
    [%] => { ... };
    [%=] => { ... };
    [+] => { ... };
    [+=] => { ... };
    [#] => { ... };
    [?] => { ... };
    [->] => { ... };
    [;] => { ... };
    [<<] => { ... };
    [<<=] => { ... };
    [>>] => { ... };
    [>>=] => { ... };
    [/] => { ... };
    [/=] => { ... };
    [*] => { ... };
    [*=] => { ... };
    [~] => { ... };
    [_] => { ... };
}
```



## Module: attr



## Module: bigint



## syn::braced

*Declarative Macro*

Parse a set of curly braces and expose their content to subsequent parsers.

# Example

```
# use quote::quote;
#
use syn::{braced, token, Ident, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

// Parse a simplified struct syntax like:
//
//     struct S {
//         a: A,
//         b: B,
//     }
struct Struct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

struct Field {
    name: Ident,
    colon_token: Token![:],
    ty: Type,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Struct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse, Token![,])?,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Field {
            name: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}
#
# fn main() {
#     let input = quote! {
#         struct S {
#             a: A,
#             b: B,
#         }
#     };
#     syn::parse2::<Struct>(input).unwrap();
# }
```

```rust
macro_rules! braced {
    ($content:ident in $cursor:expr) => { ... };
}
```



## syn::bracketed

*Declarative Macro*

Parse a set of square brackets and expose their content to subsequent
parsers.

# Example

```
# use quote::quote;
#
use proc_macro2::TokenStream;
use syn::{bracketed, token, Result, Token};
use syn::parse::{Parse, ParseStream};

// Parse an outer attribute like:
//
//     #[repr(C, packed)]
struct OuterAttribute {
    pound_token: Token![#],
    bracket_token: token::Bracket,
    content: TokenStream,
}

impl Parse for OuterAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(OuterAttribute {
            pound_token: input.parse()?,
            bracket_token: bracketed!(content in input),
            content: content.parse()?,
        })
    }
}
#
# fn main() {
#     let input = quote! {
#         #[repr(C, packed)]
#     };
#     syn::parse2::<OuterAttribute>(input).unwrap();
# }
```

```rust
macro_rules! bracketed {
    ($content:ident in $cursor:expr) => { ... };
}
```



## Module: buffer

A stably addressed token buffer supporting efficient traversal based on a
cheaply copyable cursor.



## Module: classify



## syn::custom_keyword

*Declarative Macro*

Define a type that supports parsing and printing a given identifier as if it
were a keyword.

# Usage

As a convention, it is recommended that this macro be invoked within a
module called `kw` or `keyword` and that the resulting parser be invoked
with a `kw::` or `keyword::` prefix.

```
mod kw {
    syn::custom_keyword!(whatever);
}
```

The generated syntax tree node supports the following operations just like
any built-in keyword token.

- [Peeking] — `input.peek(kw::whatever)`

- [Parsing] — `input.parse::<kw::whatever>()?`

- [Printing] — `quote!( ... #whatever_token ... )`

- Construction from a [`Span`] — `let whatever_token = kw::whatever(sp)`

- Field access to its span — `let sp = whatever_token.span`

[Peeking]: crate::parse::ParseBuffer::peek
[Parsing]: crate::parse::ParseBuffer::parse
[Printing]: quote::ToTokens
[`Span`]: proc_macro2::Span

# Example

This example parses input that looks like `bool = true` or `str = "value"`.
The key must be either the identifier `bool` or the identifier `str`. If
`bool`, the value may be either `true` or `false`. If `str`, the value may
be any string literal.

The symbols `bool` and `str` are not reserved keywords in Rust so these are
not considered keywords in the `syn::token` module. Like any other
identifier that is not a keyword, these can be declared as custom keywords
by crates that need to use them as such.

```
use syn::{LitBool, LitStr, Result, Token};
use syn::parse::{Parse, ParseStream};

mod kw {
    syn::custom_keyword!(bool);
    syn::custom_keyword!(str);
}

enum Argument {
    Bool {
        bool_token: kw::bool,
        eq_token: Token![=],
        value: LitBool,
    },
    Str {
        str_token: kw::str,
        eq_token: Token![=],
        value: LitStr,
    },
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::bool) {
            Ok(Argument::Bool {
                bool_token: input.parse::<kw::bool>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::str) {
            Ok(Argument::Str {
                str_token: input.parse::<kw::str>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
```

```rust
macro_rules! custom_keyword {
    ($ident:ident) => { ... };
}
```



## Module: custom_keyword



## Module: custom_punctuation



## syn::custom_punctuation

*Declarative Macro*

Define a type that supports parsing and printing a multi-character symbol
as if it were a punctuation token.

# Usage

```
syn::custom_punctuation!(LeftRightArrow, <=>);
```

The generated syntax tree node supports the following operations just like
any built-in punctuation token.

- [Peeking] — `input.peek(LeftRightArrow)`

- [Parsing] — `input.parse::<LeftRightArrow>()?`

- [Printing] — `quote!( ... #lrarrow ... )`

- Construction from a [`Span`] — `let lrarrow = LeftRightArrow(sp)`

- Construction from multiple [`Span`] — `let lrarrow = LeftRightArrow([sp, sp, sp])`

- Field access to its spans — `let spans = lrarrow.spans`

[Peeking]: crate::parse::ParseBuffer::peek
[Parsing]: crate::parse::ParseBuffer::parse
[Printing]: quote::ToTokens
[`Span`]: proc_macro2::Span

# Example

```
use core::iter;
use proc_macro2::{TokenStream, TokenTree};
use syn::parse::{Parse, ParseStream, Peek, Result};
use syn::punctuated::Punctuated;
use syn::Expr;

syn::custom_punctuation!(PathSeparator, </>);

// expr </> expr </> expr ...
struct PathSegments {
    segments: Punctuated<Expr, PathSeparator>,
}

impl Parse for PathSegments {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut segments = Punctuated::new();

        let first = parse_until(input, PathSeparator)?;
        segments.push_value(syn::parse2(first)?);

        while input.peek(PathSeparator) {
            segments.push_punct(input.parse()?);

            let next = parse_until(input, PathSeparator)?;
            segments.push_value(syn::parse2(next)?);
        }

        Ok(PathSegments { segments })
    }
}

fn parse_until<E: Peek>(input: ParseStream, end: E) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();
    while !input.is_empty() && !input.peek(end) {
        let next: TokenTree = input.parse()?;
        tokens.extend(iter::once(next));
    }
    Ok(tokens)
}

fn main() {
    let input = r#" a::b </> c::d::e "#;
    let _: PathSegments = syn::parse_str(input).unwrap();
}
```

```rust
macro_rules! custom_punctuation {
    ($ident:ident, $($tt:tt)+) => { ... };
}
```



## Module: data



## Module: derive



## Module: drops



## Module: error



## Module: expr



## Module: ext

Extension traits to provide parsing methods on foreign types.



## Module: file



## Module: fixup



## Module: gen



## Module: generics



## Module: group



## Module: ident



## Module: item



## Module: lifetime



## Module: lit



## Module: lookahead



## Module: mac



## Module: macros



## Module: meta

Facility for interpreting structured content inside of an `Attribute`.



## Module: op



## syn::parenthesized

*Declarative Macro*

Parse a set of parentheses and expose their content to subsequent parsers.

# Example

```
# use quote::quote;
#
use syn::{parenthesized, token, Ident, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

// Parse a simplified tuple struct syntax like:
//
//     struct S(A, B);
struct TupleStruct {
    struct_token: Token![struct],
    ident: Ident,
    paren_token: token::Paren,
    fields: Punctuated<Type, Token![,]>,
    semi_token: Token![;],
}

impl Parse for TupleStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(TupleStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            paren_token: parenthesized!(content in input),
            fields: content.parse_terminated(Type::parse, Token![,])?,
            semi_token: input.parse()?,
        })
    }
}
#
# fn main() {
#     let input = quote! {
#         struct S(A, B);
#     };
#     syn::parse2::<TupleStruct>(input).unwrap();
# }
```

```rust
macro_rules! parenthesized {
    ($content:ident in $cursor:expr) => { ... };
}
```



## Module: parse

Parsing interface for parsing a token stream into a syntax tree node.

Parsing in Syn is built on parser functions that take in a [`ParseStream`]
and produce a [`Result<T>`] where `T` is some syntax tree node. Underlying
these parser functions is a lower level mechanism built around the
[`Cursor`] type. `Cursor` is a cheaply copyable cursor over a range of
tokens in a token stream.

[`Result<T>`]: Result
[`Cursor`]: crate::buffer::Cursor

# Example

Here is a snippet of parsing code to get a feel for the style of the
library. We define data structures for a subset of Rust syntax including
enums (not shown) and structs, then provide implementations of the [`Parse`]
trait to parse these syntax tree data structures from a token stream.

Once `Parse` impls have been defined, they can be called conveniently from a
procedural macro through [`parse_macro_input!`] as shown at the bottom of
the snippet. If the caller provides syntactically invalid input to the
procedural macro, they will receive a helpful compiler error message
pointing out the exact token that triggered the failure to parse.

[`parse_macro_input!`]: crate::parse_macro_input!

```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use syn::{braced, parse_macro_input, token, Field, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

enum Item {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

struct ItemStruct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}
#
# enum ItemEnum {}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![struct]) {
            input.parse().map(Item::Struct)
        } else if lookahead.peek(Token![enum]) {
            input.parse().map(Item::Enum)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ItemStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse_named, Token![,])?,
        })
    }
}
#
# impl Parse for ItemEnum {
#     fn parse(input: ParseStream) -> Result<Self> {
#         unimplemented!()
#     }
# }

# const IGNORE: &str = stringify! {
#[proc_macro]
# };
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Item);

    /* ... */
#   TokenStream::new()
}
```

# The `syn::parse*` functions

The [`syn::parse`], [`syn::parse2`], and [`syn::parse_str`] functions serve
as an entry point for parsing syntax tree nodes that can be parsed in an
obvious default way. These functions can return any syntax tree node that
implements the [`Parse`] trait, which includes most types in Syn.

[`syn::parse`]: crate::parse()
[`syn::parse2`]: crate::parse2()
[`syn::parse_str`]: crate::parse_str()

```
use syn::Type;

# fn run_parser() -> syn::Result<()> {
let t: Type = syn::parse_str("alloc::collections::HashMap<String, Value>")?;
#     Ok(())
# }
#
# run_parser().unwrap();
```

The [`parse_quote!`] macro also uses this approach.

[`parse_quote!`]: crate::parse_quote!

# The `Parser` trait

Some types can be parsed in several ways depending on context. For example
an [`Attribute`] can be either "outer" like `#[...]` or "inner" like
`#![...]` and parsing the wrong one would be a bug. Similarly [`Punctuated`]
may or may not allow trailing punctuation, and parsing it the wrong way
would either reject valid input or accept invalid input.

[`Attribute`]: crate::Attribute
[`Punctuated`]: crate::punctuated

The `Parse` trait is not implemented in these cases because there is no good
behavior to consider the default.

```compile_fail
# extern crate proc_macro;
#
# use syn::punctuated::Punctuated;
# use syn::{PathSegment, Result, Token};
#
# fn f(tokens: proc_macro::TokenStream) -> Result<()> {
#
// Can't parse `Punctuated` without knowing whether trailing punctuation
// should be allowed in this context.
let path: Punctuated<PathSegment, Token![::]> = syn::parse(tokens)?;
#
#     Ok(())
# }
```

In these cases the types provide a choice of parser functions rather than a
single `Parse` implementation, and those parser functions can be invoked
through the [`Parser`] trait.


```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, PathSegment, Result, Token};

fn call_some_parser_methods(input: TokenStream) -> Result<()> {
    // Parse a nonempty sequence of path segments separated by `::` punctuation
    // with no trailing punctuation.
    let tokens = input.clone();
    let parser = Punctuated::<PathSegment, Token![::]>::parse_separated_nonempty;
    let _path = parser.parse(tokens)?;

    // Parse a possibly empty sequence of expressions terminated by commas with
    // an optional trailing punctuation.
    let tokens = input.clone();
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let _args = parser.parse(tokens)?;

    // Parse zero or more outer attributes but not inner attributes.
    let tokens = input.clone();
    let parser = Attribute::parse_outer;
    let _attrs = parser.parse(tokens)?;

    Ok(())
}
```



## syn::parse

*Function*

Parse tokens of source code into the chosen syntax tree node.

This is preferred over parsing a string because tokens are able to preserve
information about where in the user's code they were originally written (the
"span" of the token), possibly allowing the compiler to produce better error
messages.

This function parses a `proc_macro::TokenStream` which is the type used for
interop with the compiler in a procedural macro. To parse a
`proc_macro2::TokenStream`, use [`syn::parse2`] instead.

[`syn::parse2`]: parse2

This function enforces that the input is fully parsed. If there are any
unparsed tokens at the end of the stream, an error is returned.

```rust
fn parse<T>(tokens: proc_macro::TokenStream) -> Result<T>
```



## syn::parse2

*Function*

Parse a proc-macro2 token stream into the chosen syntax tree node.

This function parses a `proc_macro2::TokenStream` which is commonly useful
when the input comes from a node of the Syn syntax tree, for example the
body tokens of a [`Macro`] node. When in a procedural macro parsing the
`proc_macro::TokenStream` provided by the compiler, use [`syn::parse`]
instead.

[`syn::parse`]: parse()

This function enforces that the input is fully parsed. If there are any
unparsed tokens at the end of the stream, an error is returned.

```rust
fn parse2<T>(tokens: proc_macro2::TokenStream) -> Result<T>
```



## syn::parse_file

*Function*

Parse the content of a file of Rust code.

This is different from `syn::parse_str::<File>(content)` in two ways:

- It discards a leading byte order mark `\u{FEFF}` if the file has one.
- It preserves the shebang line of the file, such as `#!/usr/bin/env rustx`.

If present, either of these would be an error using `from_str`.

# Examples

```no_run
use std::error::Error;
use std::fs;
use std::io::Read;

fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("path/to/code.rs")?;
    let ast = syn::parse_file(&content)?;
    if let Some(shebang) = ast.shebang {
        println!("{}", shebang);
    }
    println!("{} items", ast.items.len());

    Ok(())
}
#
# run().unwrap();
```

```rust
fn parse_file(content: &str) -> Result<File>
```



## syn::parse_macro_input

*Declarative Macro*

Parse the input TokenStream of a macro, triggering a compile error if the
tokens fail to parse.

Refer to the [`parse` module] documentation for more details about parsing
in Syn.

[`parse` module]: mod@crate::parse

<br>

# Intended usage

This macro must be called from a function that returns
`proc_macro::TokenStream`. Usually this will be your proc macro entry point,
the function that has the #\[proc_macro\] / #\[proc_macro_derive\] /
#\[proc_macro_attribute\] attribute.

```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use syn::{parse_macro_input, Result};
use syn::parse::{Parse, ParseStream};

struct MyMacroInput {
    /* ... */
}

impl Parse for MyMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        /* ... */
#       Ok(MyMacroInput {})
    }
}

# const IGNORE: &str = stringify! {
#[proc_macro]
# };
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MyMacroInput);

    /* ... */
#   TokenStream::new()
}
```

<br>

# Usage with Parser

This macro can also be used with the [`Parser` trait] for types that have
multiple ways that they can be parsed.

[`Parser` trait]: crate::parse::Parser

```
# extern crate proc_macro;
#
# use proc_macro::TokenStream;
# use syn::{parse_macro_input, Result};
# use syn::parse::ParseStream;
#
# struct MyMacroInput {}
#
impl MyMacroInput {
    fn parse_alternate(input: ParseStream) -> Result<Self> {
        /* ... */
#       Ok(MyMacroInput {})
    }
}

# const IGNORE: &str = stringify! {
#[proc_macro]
# };
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens with MyMacroInput::parse_alternate);

    /* ... */
#   TokenStream::new()
}
```

<br>

# Expansion

`parse_macro_input!($variable as $Type)` expands to something like:

```no_run
# extern crate proc_macro;
#
# macro_rules! doc_test {
#     ($variable:ident as $Type:ty) => {
match syn::parse::<$Type>($variable) {
    Ok(syntax_tree) => syntax_tree,
    Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
}
#     };
# }
#
# fn test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
#     let _ = doc_test!(input as syn::Ident);
#     proc_macro::TokenStream::new()
# }
```

```rust
macro_rules! parse_macro_input {
    ($tokenstream:ident as $ty:ty) => { ... };
    ($tokenstream:ident with $parser:path) => { ... };
    ($tokenstream:ident) => { ... };
}
```



## Module: parse_macro_input



## syn::parse_quote

*Declarative Macro*

Quasi-quotation macro that accepts input like the [`quote!`] macro but uses
type inference to figure out a return type for those tokens.

[`quote!`]: https://docs.rs/quote/1.0/quote/index.html

The return type can be any syntax tree node that implements the [`Parse`]
trait.

[`Parse`]: crate::parse::Parse

```
use quote::quote;
use syn::{parse_quote, Stmt};

fn main() {
    let name = quote!(v);
    let ty = quote!(u8);

    let stmt: Stmt = parse_quote! {
        let #name: #ty = Default::default();
    };

    println!("{:#?}", stmt);
}
```

*This macro is available only if Syn is built with both the `"parsing"` and
`"printing"` features.*

# Example

The following helper function adds a bound `T: HeapSize` to every type
parameter `T` in the input generics.

```
use syn::{parse_quote, Generics, GenericParam};

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(HeapSize));
        }
    }
    generics
}
```

# Special cases

This macro can parse the following additional types as a special case even
though they do not implement the `Parse` trait.

- [`Attribute`] — parses one attribute, allowing either outer like `#[...]`
  or inner like `#![...]`
- [`Vec<Attribute>`] — parses multiple attributes, including mixed kinds in
  any order
- [`Punctuated<T, P>`] — parses zero or more `T` separated by punctuation
  `P` with optional trailing punctuation
- [`Vec<Arm>`] — parses arms separated by optional commas according to the
  same grammar as the inside of a `match` expression
- [`Vec<Stmt>`] — parses the same as `Block::parse_within`
- [`Pat`], [`Box<Pat>`] — parses the same as
  `Pat::parse_multi_with_leading_vert`
- [`Field`] — parses a named or unnamed struct field

[`Vec<Attribute>`]: Attribute
[`Vec<Arm>`]: Arm
[`Vec<Stmt>`]: Block::parse_within
[`Pat`]: Pat::parse_multi_with_leading_vert
[`Box<Pat>`]: Pat::parse_multi_with_leading_vert

# Panics

Panics if the tokens fail to parse as the expected syntax tree type. The
caller is responsible for ensuring that the input tokens are syntactically
valid.

```rust
macro_rules! parse_quote {
    ($($tt:tt)*) => { ... };
}
```



## Module: parse_quote



## syn::parse_quote_spanned

*Declarative Macro*

This macro is [`parse_quote!`] + [`quote_spanned!`][quote::quote_spanned].

Please refer to each of their documentation.

# Example

```
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote_spanned, ReturnType, Signature};

// Changes `fn()` to `fn() -> Pin<Box<dyn Future<Output = ()>>>`,
// and `fn() -> T` to `fn() -> Pin<Box<dyn Future<Output = T>>>`,
// without introducing any call_site() spans.
fn make_ret_pinned_future(sig: &mut Signature) {
    let ret = match &sig.output {
        ReturnType::Default => quote_spanned!(sig.paren_token.span=> ()),
        ReturnType::Type(_, ret) => quote!(#ret),
    };
    sig.output = parse_quote_spanned! {ret.span()=>
        -> ::core::pin::Pin<::alloc::boxed::Box<dyn ::core::future::Future<Output = #ret>>>
    };
}
```

```rust
macro_rules! parse_quote_spanned {
    ($span:expr=> $($tt:tt)*) => { ... };
}
```



## syn::parse_str

*Function*

Parse a string of Rust code into the chosen syntax tree node.

This function enforces that the input is fully parsed. If there are any
unparsed tokens at the end of the stream, an error is returned.

# Hygiene

Every span in the resulting syntax tree will be set to resolve at the macro
call site.

# Examples

```
use syn::{Expr, Result};

fn run() -> Result<()> {
    let code = "assert_eq!(u8::max_value(), 255)";
    let expr = syn::parse_str::<Expr>(code)?;
    println!("{:#?}", expr);
    Ok(())
}
#
# run().unwrap();
```

```rust
fn parse_str<T>(s: &str) -> Result<T>
```



## Module: pat



## Module: path



## Module: precedence



## Module: print



## Module: punctuated

A punctuated sequence of syntax tree nodes separated by punctuation.

Lots of things in Rust are punctuated sequences.

- The fields of a struct are `Punctuated<Field, Token![,]>`.
- The segments of a path are `Punctuated<PathSegment, Token![::]>`.
- The bounds on a generic parameter are `Punctuated<TypeParamBound,
  Token![+]>`.
- The arguments to a function call are `Punctuated<Expr, Token![,]>`.

This module provides a common representation for these punctuated sequences
in the form of the [`Punctuated<T, P>`] type. We store a vector of pairs of
syntax tree node + punctuation, where every node in the sequence is followed
by punctuation except for possibly the final one.

[`Punctuated<T, P>`]: Punctuated

```text
a_function_call(arg1, arg2, arg3);
                ~~~~^ ~~~~^ ~~~~
```



## Module: restriction



## Module: sealed



## Module: span



## Module: spanned

A trait that can provide the `Span` of the complete contents of a syntax
tree node.

<br>

# Example

Suppose in a procedural macro we have a [`Type`] that we want to assert
implements the [`Sync`] trait. Maybe this is the type of one of the fields
of a struct for which we are deriving a trait implementation, and we need to
be able to pass a reference to one of those fields across threads.

[`Type`]: crate::Type
[`Sync`]: core::marker::Sync

If the field type does *not* implement `Sync` as required, we want the
compiler to report an error pointing out exactly which type it was.

The following macro code takes a variable `ty` of type `Type` and produces a
static assertion that `Sync` is implemented for that type.

```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote_spanned;
use syn::Type;
use syn::spanned::Spanned;

# const IGNORE_TOKENS: &str = stringify! {
#[proc_macro_derive(MyMacro)]
# };
pub fn my_macro(input: TokenStream) -> TokenStream {
    # let ty = get_a_type();
    /* ... */

    let assert_sync = quote_spanned! {ty.span()=>
        struct _AssertSync where #ty: Sync;
    };

    /* ... */
    # input
}
#
# fn get_a_type() -> Type {
#     unimplemented!()
# }
```

By inserting this `assert_sync` fragment into the output code generated by
our macro, the user's code will fail to compile if `ty` does not implement
`Sync`. The errors they would see look like the following.

```text
error[E0277]: the trait bound `*const i32: core::marker::Sync` is not satisfied
  --> src/main.rs:10:21
   |
10 |     bad_field: *const i32,
   |                ^^^^^^^^^^ `*const i32` cannot be shared between threads safely
```

In this technique, using the `Type`'s span for the error message makes the
error appear in the correct place underlining the right type.

<br>

# Limitations

The underlying [`proc_macro::Span::join`] method is nightly-only. When
called from within a procedural macro in a nightly compiler, `Spanned` will
use `join` to produce the intended span. When not using a nightly compiler,
only the span of the *first token* of the syntax tree node is returned.

In the common case of wanting to use the joined span as the span of a
`syn::Error`, consider instead using [`syn::Error::new_spanned`] which is
able to span the error correctly under the complete syntax tree node without
needing the unstable `join`.

[`syn::Error::new_spanned`]: crate::Error::new_spanned



## Module: stmt



## Module: thread



## Module: token

Tokens representing Rust punctuation, keywords, and delimiters.

The type names in this module can be difficult to keep straight, so we
prefer to use the [`Token!`] macro instead. This is a type-macro that
expands to the token type of the given token.

[`Token!`]: crate::Token

# Example

The [`ItemStatic`] syntax tree node is defined like this.

[`ItemStatic`]: crate::ItemStatic

```
# use syn::{Attribute, Expr, Ident, Token, Type, Visibility};
#
pub struct ItemStatic {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub static_token: Token![static],
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    pub colon_token: Token![:],
    pub ty: Box<Type>,
    pub eq_token: Token![=],
    pub expr: Box<Expr>,
    pub semi_token: Token![;],
}
```

# Parsing

Keywords and punctuation can be parsed through the [`ParseStream::parse`]
method. Delimiter tokens are parsed using the [`parenthesized!`],
[`bracketed!`] and [`braced!`] macros.

[`ParseStream::parse`]: crate::parse::ParseBuffer::parse()
[`parenthesized!`]: crate::parenthesized!
[`bracketed!`]: crate::bracketed!
[`braced!`]: crate::braced!

```
use syn::{Attribute, Result};
use syn::parse::{Parse, ParseStream};
#
# enum ItemStatic {}

// Parse the ItemStatic struct shown above.
impl Parse for ItemStatic {
    fn parse(input: ParseStream) -> Result<Self> {
        # use syn::ItemStatic;
        # fn parse(input: ParseStream) -> Result<ItemStatic> {
        Ok(ItemStatic {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            static_token: input.parse()?,
            mutability: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            eq_token: input.parse()?,
            expr: input.parse()?,
            semi_token: input.parse()?,
        })
        # }
        # unimplemented!()
    }
}
```

# Other operations

Every keyword and punctuation token supports the following operations.

- [Peeking] — `input.peek(Token![...])`

- [Parsing] — `input.parse::<Token![...]>()?`

- [Printing] — `quote!( ... #the_token ... )`

- Construction from a [`Span`] — `let the_token = Token![...](sp)`

- Field access to its span — `let sp = the_token.span`

[Peeking]: crate::parse::ParseBuffer::peek()
[Parsing]: crate::parse::ParseBuffer::parse()
[Printing]: https://docs.rs/quote/1.0/quote/trait.ToTokens.html
[`Span`]: https://docs.rs/proc-macro2/1.0/proc_macro2/struct.Span.html



## Module: ty



## Module: verbatim



## Module: whitespace



