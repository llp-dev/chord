*[quote](../index.md) / [to_tokens](index.md)*

---

# Module `to_tokens`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ToTokens`](#totokens) | trait | Types that can be interpolated inside a `quote!` invocation. |

## Traits

### `ToTokens`

```rust
trait ToTokens { ... }
```

Types that can be interpolated inside a `quote!` invocation.

#### Required Methods

- `fn to_tokens(&self, tokens: &mut TokenStream)`

  Write `self` to the given `TokenStream`.

#### Provided Methods

- `fn to_token_stream(&self) -> TokenStream`

  Convert `self` directly into a `TokenStream` object.

- `fn into_token_stream(self) -> TokenStream`

  Convert `self` directly into a `TokenStream` object.

#### Implementors

- `&T`
- `&mut T`
- `Option<T>`
- `alloc::borrow::Cow<'a, T>`
- `alloc::boxed::Box<T>`
- `alloc::ffi::CString`
- `alloc::rc::Rc<T>`
- `alloc::string::String`
- `bool`
- `char`
- `core::ffi::CStr`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `proc_macro2::Group`
- `proc_macro2::Ident`
- `proc_macro2::Literal`
- `proc_macro2::Punct`
- `proc_macro2::TokenStream`
- `proc_macro2::TokenTree`
- `std::sync::Arc<T>`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

