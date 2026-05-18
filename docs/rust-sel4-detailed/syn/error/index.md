*[syn](../index.md) / [error](index.md)*

---

# Module `error`

## Contents

- [Structs](#structs)
  - [`Error`](#error)
  - [`ErrorMessage`](#errormessage)
  - [`SpanRange`](#spanrange)
  - [`IntoIter`](#intoiter)
  - [`Iter`](#iter)
- [Functions](#functions)
  - [`new_at`](#new-at)
  - [`new2`](#new2)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct | Error returned when a Syn parser cannot parse the input tokens. |
| [`ErrorMessage`](#errormessage) | struct |  |
| [`SpanRange`](#spanrange) | struct |  |
| [`IntoIter`](#intoiter) | struct |  |
| [`Iter`](#iter) | struct |  |
| [`new_at`](#new-at) | fn |  |
| [`new2`](#new2) | fn |  |
| [`Result`](#result) | type | The result of a Syn parser. |

## Structs

### `Error`

```rust
struct Error {
    messages: alloc::vec::Vec<ErrorMessage>,
}
```

Error returned when a Syn parser cannot parse the input tokens.

# Error reporting in proc macros

The correct way to report errors back to the compiler from a procedural
macro is by emitting an appropriately spanned invocation of
`compile_error!` in the generated code. This produces a better diagnostic
message than simply panicking the macro.

When parsing macro input, the `parse_macro_input!` macro handles the
conversion to `compile_error!` automatically.

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, ItemFn};

const IGNORE: &str = stringify! {
#[proc_macro_attribute]
};
pub fn my_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as MyAttrArgs);
    let input = parse_macro_input!(input as ItemFn);

    /* ... */
    TokenStream::new()
}

struct MyAttrArgs {
    _k: [(); { stringify! {
    ...
    }; 0 }]
}

impl Parse for MyAttrArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        stringify! {
        ...
        };
        unimplemented!()
    }
}
```

For errors that arise later than the initial parsing stage, the
`.to_compile_error()` or `.into_compile_error()` methods can be used to
perform an explicit conversion to `compile_error!`.


```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

const IGNORE: &str = stringify! {
#[proc_macro_derive(MyDerive)]
};
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // fn(DeriveInput) -> syn::Result<proc_macro2::TokenStream>
    expand::my_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

mod expand {
    use proc_macro2::TokenStream;
    use syn::{DeriveInput, Result};

    pub fn my_derive(input: DeriveInput) -> Result<TokenStream> {
        unimplemented!()
    }
}
```

#### Implementations

- <span id="error-new"></span>`fn new<T: Display>(span: Span, message: T) -> Self`

  Usually the `ParseStream::error` method will be used instead, which

  automatically uses the correct span from the current position of the

  parse stream.

  

  Use `Error::new` when the error needs to be triggered on some span other

  than where the parse stream is currently positioned.

  

  # Example

  

  ```rust

  use syn::{Error, Ident, LitStr, Result, Token};

  use syn::parse::ParseStream;

  

  // Parses input that looks like `name = "string"` where the key must be

  // the identifier `name` and the value may be any string literal.

  // Returns the string literal.

  fn parse_name(input: ParseStream) -> Result<LitStr> {

      let name_token: Ident = input.parse()?;

      if name_token != "name" {

          // Trigger an error not on the current position of the stream,

          // but on the position of the unexpected identifier.

          return Err(Error::new(name_token.span(), "expected `name`"));

      }

      input.parse::<Token![=]>()?;

      let s: LitStr = input.parse()?;

      Ok(s)

  }

  ```

- <span id="error-new-spanned"></span>`fn new_spanned<T: ToTokens, U: Display>(tokens: T, message: U) -> Self`

  Creates an error with the specified message spanning the given syntax

  tree node.

  

  Unlike the `Error::new` constructor, this constructor takes an argument

  `tokens` which is a syntax tree node. This allows the resulting `Error`

  to attempt to span all tokens inside of `tokens`. While you would

  typically be able to use the `Spanned` trait with the above `Error::new`

  constructor, implementation limitations today mean that

  `Error::new_spanned` may provide a higher-quality error message on

  stable Rust.

  

  When in doubt it's recommended to stick to `Error::new` (or

  `ParseStream::error`)!

- <span id="error-span"></span>`fn span(&self) -> Span`

  The source location of the error.

  

  Spans are not thread-safe so this function returns `Span::call_site()`

  if called from a different thread than the one on which the `Error` was

  originally created.

- <span id="error-to-compile-error"></span>`fn to_compile_error(&self) -> TokenStream`

  Render the error as an invocation of `compile_error!`.

  

  The `parse_macro_input!` macro provides a convenient way to invoke

  this method correctly in a procedural macro.

  

- <span id="error-into-compile-error"></span>`fn into_compile_error(self) -> TokenStream`

  Render the error as an invocation of `compile_error!`.

  

  # Example

  

  ```rust

  extern crate proc_macro;

  

  use proc_macro::TokenStream;

  use syn::{parse_macro_input, DeriveInput, Error};

  

  const _: &str = stringify! {

  #[proc_macro_derive(MyTrait)]

  };

  pub fn derive_my_trait(input: TokenStream) -> TokenStream {

      let input = parse_macro_input!(input as DeriveInput);

      my_trait::expand(input)

          .unwrap_or_else(Error::into_compile_error)

          .into()

  }

  

  mod my_trait {

      use proc_macro2::TokenStream;

      use syn::{DeriveInput, Result};

  

      pub(crate) fn expand(input: DeriveInput) -> Result<TokenStream> {

          /* ... */

          unimplemented!()

      }

  }

  ```

- <span id="error-combine"></span>`fn combine(&mut self, another: Error)` — [`Error`](#error)

  Add another error message to self such that when `to_compile_error()` is

  called, both errors will be emitted together.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

##### `impl Extend for Error`

- <span id="error-extend"></span>`fn extend<T: IntoIterator<Item = Error>>(&mut self, iter: T)`

##### `impl IntoIterator for Error`

- <span id="error-intoiterator-type-item"></span>`type Item = Error`

- <span id="error-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter`

- <span id="error-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

### `ErrorMessage`

```rust
struct ErrorMessage {
    span: crate::thread::ThreadBound<SpanRange>,
    message: alloc::string::String,
}
```

#### Implementations

- <span id="errormessage-to-compile-error"></span>`fn to_compile_error(&self, tokens: &mut TokenStream)`

#### Trait Implementations

##### `impl Clone for ErrorMessage`

- <span id="errormessage-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for ErrorMessage`

- <span id="errormessage-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SpanRange`

```rust
struct SpanRange {
    start: proc_macro2::Span,
    end: proc_macro2::Span,
}
```

#### Trait Implementations

##### `impl Clone for SpanRange`

- <span id="spanrange-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for SpanRange`

### `IntoIter`

```rust
struct IntoIter {
    messages: vec::IntoIter<ErrorMessage>,
}
```

#### Trait Implementations

##### `impl IntoIterator for IntoIter`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoIter`

- <span id="intoiter-iterator-type-item"></span>`type Item = Error`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Iter<'a>`

```rust
struct Iter<'a> {
    messages: slice::Iter<'a, ErrorMessage>,
}
```

#### Trait Implementations

##### `impl IntoIterator for Iter<'a>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Iter<'a>`

- <span id="iter-iterator-type-item"></span>`type Item = Error`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Functions

### `new_at`

```rust
fn new_at<T: Display>(scope: proc_macro2::Span, cursor: crate::buffer::Cursor<'_>, message: T) -> Error
```

### `new2`

```rust
fn new2<T: Display>(start: proc_macro2::Span, end: proc_macro2::Span, message: T) -> Error
```

## Type Aliases

### `Result<T>`

```rust
type Result<T> = core::result::Result<T, Error>;
```

The result of a Syn parser.

