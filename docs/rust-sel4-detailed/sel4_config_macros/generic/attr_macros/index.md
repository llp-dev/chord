*[sel4_config_macros](../../index.md) / [generic](../index.md) / [attr_macros](index.md)*

---

# Module `attr_macros`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CfgAttrInput`](#cfgattrinput) | struct |  |
| [`Helper`](#helper) | struct |  |
| [`filter_punctuated`](#filter-punctuated) | fn |  |
| [`ensure_empty!`](#ensure-empty) | macro |  |

## Structs

### `CfgAttrInput`

```rust
struct CfgAttrInput {
    condition: condition::Condition,
    body: proc_macro2::TokenStream,
}
```

#### Trait Implementations

##### `impl Parse for CfgAttrInput`

- <span id="cfgattrinput-parse"></span>`fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self>`

### `Helper<'a>`

```rust
struct Helper<'a> {
    impls: &'a super::MacroImpls<'a>,
    first_err: Option<proc_macro2::TokenStream>,
}
```

#### Implementations

- <span id="helper-new"></span>`fn new(impls: &'a MacroImpls<'a>) -> Self` — [`MacroImpls`](../index.md#macroimpls)

- <span id="helper-report-err"></span>`fn report_err(&mut self, err: TokenStream)`

- <span id="helper-first-err-or"></span>`fn first_err_or(self, otherwise: impl ToTokens) -> TokenStream`

- <span id="helper-process-attrs"></span>`fn process_attrs(&mut self, attrs: &mut Vec<syn::Attribute>) -> bool`

- <span id="helper-filter-punctuated"></span>`fn filter_punctuated<T, P>(&mut self, punctuated: syn::punctuated::Punctuated<T, P>, f: impl Fn(&mut T) -> &mut Vec<syn::Attribute>) -> syn::punctuated::Punctuated<T, P>`

## Functions

### `filter_punctuated`

```rust
fn filter_punctuated<T, P>(punctuated: syn::punctuated::Punctuated<T, P>, f: impl FnMut(&mut T) -> bool) -> syn::punctuated::Punctuated<T, P>
```

## Macros

### `ensure_empty!`

