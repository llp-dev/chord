**syn > meta**

# Module: meta

## Contents

**Structs**

- [`ParseNestedMeta`](#parsenestedmeta) - Context for parsing a single property in the conventional syntax for

**Functions**

- [`parse_meta_path`](#parse_meta_path)
- [`parse_nested_meta`](#parse_nested_meta)
- [`parser`](#parser) - Make a parser that is usable with `parse_macro_input!` in a

---

## syn::meta::ParseNestedMeta

*Struct*

Context for parsing a single property in the conventional syntax for
structured attributes.

# Examples

Refer to usage examples on the following two entry-points:

- [`Attribute::parse_nested_meta`] if you have an entire `Attribute` to
  parse. Always use this if possible. Generally this is able to produce
  better error messages because `Attribute` holds span information for all
  of the delimiters therein.

- [`syn::meta::parser`] if you are implementing a `proc_macro_attribute`
  macro and parsing the arguments to the attribute macro, i.e. the ones
  written in the same attribute that dispatched the macro invocation. Rustc
  does not pass span information for the surrounding delimiters into the
  attribute macro invocation in this situation, so error messages might be
  less precise.

[`Attribute::parse_nested_meta`]: crate::Attribute::parse_nested_meta
[`syn::meta::parser`]: crate::meta::parser

**Generic Parameters:**
- 'a

**Fields:**
- `path: crate::path::Path`
- `input: crate::parse::ParseStream<'a>`

**Methods:**

- `fn value(self: &Self) -> Result<ParseStream<'a>>` - Used when parsing `key = "value"` syntax.
- `fn parse_nested_meta<impl FnMut(ParseNestedMeta) -> Result<()>>(self: &Self, logic: impl Trait) -> Result<()>` - Used when parsing `list(...)` syntax **if** the content inside the
- `fn error<impl Display>(self: &Self, msg: impl Trait) -> Error` - Report that the attribute's content did not conform to expectations.



## syn::meta::parse_meta_path

*Function*

```rust
fn parse_meta_path(input: crate::parse::ParseStream) -> crate::error::Result<crate::path::Path>
```



## syn::meta::parse_nested_meta

*Function*

```rust
fn parse_nested_meta<impl FnMut(ParseNestedMeta) -> Result<()>>(input: crate::parse::ParseStream, logic: impl Trait) -> crate::error::Result<()>
```



## syn::meta::parser

*Function*

Make a parser that is usable with `parse_macro_input!` in a
`#[proc_macro_attribute]` macro.

*Warning:* When parsing attribute args **other than** the
`proc_macro::TokenStream` input of a `proc_macro_attribute`, you do **not**
need this function. In several cases your callers will get worse error
messages if you use this function, because the surrounding delimiter's span
is concealed from attribute macros by rustc. Use
[`Attribute::parse_nested_meta`] instead.

[`Attribute::parse_nested_meta`]: crate::Attribute::parse_nested_meta

# Example

This example implements an attribute macro whose invocations look like this:

```
# const IGNORE: &str = stringify! {
#[tea(kind = "EarlGrey", hot)]
struct Picard {...}
# };
```

The "parameters" supported by the attribute are:

- `kind = "..."`
- `hot`
- `with(sugar, milk, ...)`, a comma-separated list of ingredients

```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr, Path};

# const IGNORE: &str = stringify! {
#[proc_macro_attribute]
# };
pub fn tea(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut kind: Option<LitStr> = None;
    let mut hot: bool = false;
    let mut with: Vec<Path> = Vec::new();
    let tea_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("kind") {
            kind = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("hot") {
            hot = true;
            Ok(())
        } else if meta.path.is_ident("with") {
            meta.parse_nested_meta(|meta| {
                with.push(meta.path);
                Ok(())
            })
        } else {
            Err(meta.error("unsupported tea property"))
        }
    });

    parse_macro_input!(args with tea_parser);
    eprintln!("kind={kind:?} hot={hot} with={with:?}");

    /* ... */
#   TokenStream::new()
}
```

The `syn::meta` library will take care of dealing with the commas including
trailing commas, and producing sensible error messages on unexpected input.

```console
error: expected `,`
 --> src/main.rs:3:37
  |
3 | #[tea(kind = "EarlGrey", with(sugar = "lol", milk))]
  |                                     ^
```

# Example

Same as above but we factor out most of the logic into a separate function.

```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use syn::meta::ParseNestedMeta;
use syn::parse::{Parser, Result};
use syn::{parse_macro_input, LitStr, Path};

# const IGNORE: &str = stringify! {
#[proc_macro_attribute]
# };
pub fn tea(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut attrs = TeaAttributes::default();
    let tea_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with tea_parser);

    /* ... */
#   TokenStream::new()
}

#[derive(Default)]
struct TeaAttributes {
    kind: Option<LitStr>,
    hot: bool,
    with: Vec<Path>,
}

impl TeaAttributes {
    fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("kind") {
            self.kind = Some(meta.value()?.parse()?);
            Ok(())
        } else /* just like in last example */
#           { unimplemented!() }

    }
}
```

```rust
fn parser<impl FnMut(ParseNestedMeta) -> Result<()>>(logic: impl Trait) -> impl Trait
```



