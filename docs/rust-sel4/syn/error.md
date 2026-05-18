**syn > error**

# Module: error

## Contents

**Structs**

- [`Error`](#error) - Error returned when a Syn parser cannot parse the input tokens.
- [`ErrorMessage`](#errormessage)
- [`IntoIter`](#intoiter)
- [`Iter`](#iter)
- [`SpanRange`](#spanrange)

**Functions**

- [`new2`](#new2)
- [`new_at`](#new_at)

**Type Aliases**

- [`Result`](#result) - The result of a Syn parser.

---

## syn::error::Error

*Struct*

Error returned when a Syn parser cannot parse the input tokens.

# Error reporting in proc macros

The correct way to report errors back to the compiler from a procedural
macro is by emitting an appropriately spanned invocation of
[`compile_error!`] in the generated code. This produces a better diagnostic
message than simply panicking the macro.

[`compile_error!`]: core::compile_error!

When parsing macro input, the [`parse_macro_input!`] macro handles the
conversion to `compile_error!` automatically.

[`parse_macro_input!`]: crate::parse_macro_input!

```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, ItemFn};

# const IGNORE: &str = stringify! {
#[proc_macro_attribute]
# };
pub fn my_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as MyAttrArgs);
    let input = parse_macro_input!(input as ItemFn);

    /* ... */
    # TokenStream::new()
}

struct MyAttrArgs {
    # _k: [(); { stringify! {
    ...
    # }; 0 }]
}

impl Parse for MyAttrArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        # stringify! {
        ...
        # };
        # unimplemented!()
    }
}
```

For errors that arise later than the initial parsing stage, the
[`.to_compile_error()`] or [`.into_compile_error()`] methods can be used to
perform an explicit conversion to `compile_error!`.

[`.to_compile_error()`]: Error::to_compile_error
[`.into_compile_error()`]: Error::into_compile_error

```
# extern crate proc_macro;
#
# use proc_macro::TokenStream;
# use syn::{parse_macro_input, DeriveInput};
#
# const IGNORE: &str = stringify! {
#[proc_macro_derive(MyDerive)]
# };
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // fn(DeriveInput) -> syn::Result<proc_macro2::TokenStream>
    expand::my_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
#
# mod expand {
#     use proc_macro2::TokenStream;
#     use syn::{DeriveInput, Result};
#
#     pub fn my_derive(input: DeriveInput) -> Result<TokenStream> {
#         unimplemented!()
#     }
# }
```

**Fields:**
- `messages: alloc::vec::Vec<ErrorMessage>`

**Methods:**

- `fn new<T>(span: Span, message: T) -> Self` - Usually the [`ParseStream::error`] method will be used instead, which
- `fn new_spanned<T, U>(tokens: T, message: U) -> Self` - Creates an error with the specified message spanning the given syntax
- `fn span(self: &Self) -> Span` - The source location of the error.
- `fn to_compile_error(self: &Self) -> TokenStream` - Render the error as an invocation of [`compile_error!`].
- `fn into_compile_error(self: Self) -> TokenStream` - Render the error as an invocation of [`compile_error!`].
- `fn combine(self: & mut Self, another: Error)` - Add another error message to self such that when `to_compile_error()` is

**Traits:** Error

**Trait Implementations:**

- **From**
  - `fn from(err: LexError) -> Self`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`



## syn::error::ErrorMessage

*Struct*

**Fields:**
- `span: crate::thread::ThreadBound<SpanRange>`
- `message: alloc::string::String`

**Methods:**

- `fn to_compile_error(self: &Self, tokens: & mut TokenStream)`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`



## syn::error::IntoIter

*Struct*

**Fields:**
- `messages: vec::IntoIter<ErrorMessage>`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syn::error::Iter

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `messages: slice::Iter<'a, ErrorMessage>`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syn::error::Result

*Type Alias*: `core::result::Result<T, Error>`

The result of a Syn parser.



## syn::error::SpanRange

*Struct*

**Fields:**
- `start: proc_macro2::Span`
- `end: proc_macro2::Span`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::error::new2

*Function*

```rust
fn new2<T>(start: proc_macro2::Span, end: proc_macro2::Span, message: T) -> Error
```



## syn::error::new_at

*Function*

```rust
fn new_at<T>(scope: proc_macro2::Span, cursor: crate::buffer::Cursor, message: T) -> Error
```



