**syn > attr**

# Module: attr

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`Attribute`](#attribute) - An attribute, like `#[repr(transparent)]`.
- [`MetaList`](#metalist) - A structured list within an attribute, like `derive(Copy, Clone)`.
- [`MetaNameValue`](#metanamevalue) - A name-value pair within an attribute, like `feature = "nightly"`.

**Enums**

- [`AttrStyle`](#attrstyle) - Distinguishes between attributes that decorate an item and attributes
- [`Meta`](#meta) - Content of a compile-time structured attribute.

**Traits**

- [`FilterAttrs`](#filterattrs)

---

## syn::attr::AttrStyle

*Enum*

Distinguishes between attributes that decorate an item and attributes
that are contained within an item.

# Outer attributes

- `#[repr(transparent)]`
- `/// # Example`
- `/** Please file an issue */`

# Inner attributes

- `#![feature(proc_macro)]`
- `//! # Example`
- `/*! Please file an issue */`

**Variants:**
- `Outer`
- `Inner($crate::token::Not)`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::attr::Attribute

*Struct*

An attribute, like `#[repr(transparent)]`.

<br>

# Syntax

Rust has six types of attributes.

- Outer attributes like `#[repr(transparent)]`. These appear outside or
  in front of the item they describe.

- Inner attributes like `#![feature(proc_macro)]`. These appear inside
  of the item they describe, usually a module.

- Outer one-line doc comments like `/// Example`.

- Inner one-line doc comments like `//! Please file an issue`.

- Outer documentation blocks `/** Example */`.

- Inner documentation blocks `/*! Please file an issue */`.

The `style` field of type `AttrStyle` distinguishes whether an attribute
is outer or inner.

Every attribute has a `path` that indicates the intended interpretation
of the rest of the attribute's contents. The path and the optional
additional contents are represented together in the `meta` field of the
attribute in three possible varieties:

- Meta::Path &mdash; attributes whose information content conveys just a
  path, for example the `#[test]` attribute.

- Meta::List &mdash; attributes that carry arbitrary tokens after the
  path, surrounded by a delimiter (parenthesis, bracket, or brace). For
  example `#[derive(Copy)]` or `#[precondition(x < 5)]`.

- Meta::NameValue &mdash; attributes with an `=` sign after the path,
  followed by a Rust expression. For example `#[path =
  "sys/windows.rs"]`.

All doc comments are represented in the NameValue style with a path of
"doc", as this is how they are processed by the compiler and by
`macro_rules!` macros.

```text
#[derive(Copy, Clone)]
  ~~~~~~Path
  ^^^^^^^^^^^^^^^^^^^Meta::List

#[path = "sys/windows.rs"]
  ~~~~Path
  ^^^^^^^^^^^^^^^^^^^^^^^Meta::NameValue

#[test]
  ^^^^Meta::Path
```

<br>

# Parsing from tokens to Attribute

This type does not implement the [`Parse`] trait and thus cannot be
parsed directly by [`ParseStream::parse`]. Instead use
[`ParseStream::call`] with one of the two parser functions
[`Attribute::parse_outer`] or [`Attribute::parse_inner`] depending on
which you intend to parse.

[`Parse`]: crate::parse::Parse
[`ParseStream::parse`]: crate::parse::ParseBuffer::parse
[`ParseStream::call`]: crate::parse::ParseBuffer::call

```
use syn::{Attribute, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};

// Parses a unit struct with attributes.
//
//     #[path = "s.tmpl"]
//     struct S;
struct UnitStruct {
    attrs: Vec<Attribute>,
    struct_token: Token![struct],
    name: Ident,
    semi_token: Token![;],
}

impl Parse for UnitStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(UnitStruct {
            attrs: input.call(Attribute::parse_outer)?,
            struct_token: input.parse()?,
            name: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}
```

<p><br></p>

# Parsing from Attribute to structured arguments

The grammar of attributes in Rust is very flexible, which makes the
syntax tree not that useful on its own. In particular, arguments of the
`Meta::List` variety of attribute are held in an arbitrary `tokens:
TokenStream`. Macros are expected to check the `path` of the attribute,
decide whether they recognize it, and then parse the remaining tokens
according to whatever grammar they wish to require for that kind of
attribute. Use [`parse_args()`] to parse those tokens into the expected
data structure.

[`parse_args()`]: Attribute::parse_args

<p><br></p>

# Doc comments

The compiler transforms doc comments, such as `/// comment` and `/*!
comment */`, into attributes before macros are expanded. Each comment is
expanded into an attribute of the form `#[doc = r"comment"]`.

As an example, the following `mod` items are expanded identically:

```
# use syn::{ItemMod, parse_quote};
let doc: ItemMod = parse_quote! {
    /// Single line doc comments
    /// We write so many!
    /**
     * Multi-line comments...
     * May span many lines
     */
    mod example {
        //! Of course, they can be inner too
        /*! And fit in a single line */
    }
};
let attr: ItemMod = parse_quote! {
    #[doc = r" Single line doc comments"]
    #[doc = r" We write so many!"]
    #[doc = r"
     * Multi-line comments...
     * May span many lines
     "]
    mod example {
        #![doc = r" Of course, they can be inner too"]
        #![doc = r" And fit in a single line "]
    }
};
assert_eq!(doc, attr);
```

**Fields:**
- `pound_token: $crate::token::Pound`
- `style: AttrStyle`
- `bracket_token: token::Bracket`
- `meta: Meta`

**Methods:**

- `fn path(self: &Self) -> &Path` - Returns the path that identifies the interpretation of this attribute.
- `fn parse_args<T>(self: &Self) -> Result<T>` - Parse the arguments to the attribute as a syntax tree.
- `fn parse_args_with<F>(self: &Self, parser: F) -> Result<<F as >::Output>` - Parse the arguments to the attribute using the given parser.
- `fn parse_nested_meta<impl FnMut(ParseNestedMeta) -> Result<()>>(self: &Self, logic: impl Trait) -> Result<()>` - Parse the arguments to the attribute, expecting it to follow the
- `fn parse_outer(input: ParseStream) -> Result<Vec<Self>>` - Parses zero or more outer attributes from the stream.
- `fn parse_inner(input: ParseStream) -> Result<Vec<Self>>` - Parses zero or more inner attributes from the stream.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::attr::FilterAttrs

*Trait*

**Methods:**

- `Ret`
- `outer`
- `inner`



## syn::attr::Meta

*Enum*

Content of a compile-time structured attribute.

## Path

A meta path is like the `test` in `#[test]`.

## List

A meta list is like the `derive(Copy)` in `#[derive(Copy)]`.

## NameValue

A name-value meta is like the `path = "..."` in `#[path =
"sys/windows.rs"]`.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Path(crate::path::Path)`
- `List(MetaList)` - A structured list within an attribute, like `derive(Copy, Clone)`.
- `NameValue(MetaNameValue)` - A name-value pair within an attribute, like `feature = "nightly"`.

**Methods:**

- `fn path(self: &Self) -> &Path` - Returns the path that begins this structured meta item.
- `fn require_path_only(self: &Self) -> Result<&Path>` - Error if this is a `Meta::List` or `Meta::NameValue`.
- `fn require_list(self: &Self) -> Result<&MetaList>` - Error if this is a `Meta::Path` or `Meta::NameValue`.
- `fn require_name_value(self: &Self) -> Result<&MetaNameValue>` - Error if this is a `Meta::Path` or `Meta::List`.

**Trait Implementations:**

- **From**
  - `fn from(meta: MetaNameValue) -> Meta`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(meta: MetaList) -> Meta`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **From**
  - `fn from(meta: Path) -> Meta`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::attr::MetaList

*Struct*

A structured list within an attribute, like `derive(Copy, Clone)`.

**Fields:**
- `path: crate::path::Path`
- `delimiter: crate::mac::MacroDelimiter`
- `tokens: proc_macro2::TokenStream`

**Methods:**

- `fn parse_args<T>(self: &Self) -> Result<T>` - See [`Attribute::parse_args`].
- `fn parse_args_with<F>(self: &Self, parser: F) -> Result<<F as >::Output>` - See [`Attribute::parse_args_with`].
- `fn parse_nested_meta<impl FnMut(ParseNestedMeta) -> Result<()>>(self: &Self, logic: impl Trait) -> Result<()>` - See [`Attribute::parse_nested_meta`].

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::attr::MetaNameValue

*Struct*

A name-value pair within an attribute, like `feature = "nightly"`.

**Fields:**
- `path: crate::path::Path`
- `eq_token: $crate::token::Eq`
- `value: crate::expr::Expr`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## Module: parsing



## Module: printing



