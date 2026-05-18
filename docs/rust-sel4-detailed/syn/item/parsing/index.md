*[syn](../../index.md) / [item](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Contents

- [Structs](#structs)
  - [`FlexibleItemType`](#flexibleitemtype)
- [Enums](#enums)
  - [`TypeDefaultness`](#typedefaultness)
  - [`WhereClauseLocation`](#whereclauselocation)
  - [`FnArgOrVariadic`](#fnargorvariadic)
- [Functions](#functions)
  - [`parse_rest_of_item`](#parse-rest-of-item)
  - [`parse_macro2`](#parse-macro2)
  - [`parse_item_use`](#parse-item-use)
  - [`parse_use_tree`](#parse-use-tree)
  - [`peek_signature`](#peek-signature)
  - [`parse_signature`](#parse-signature)
  - [`parse_rest_of_fn`](#parse-rest-of-fn)
  - [`parse_fn_arg_or_variadic`](#parse-fn-arg-or-variadic)
  - [`parse_receiver_begin`](#parse-receiver-begin)
  - [`parse_rest_of_receiver`](#parse-rest-of-receiver)
  - [`parse_fn_args`](#parse-fn-args)
  - [`parse_foreign_item_type`](#parse-foreign-item-type)
  - [`parse_item_type`](#parse-item-type)
  - [`parse_trait_or_trait_alias`](#parse-trait-or-trait-alias)
  - [`parse_rest_of_trait`](#parse-rest-of-trait)
  - [`parse_start_of_trait_alias`](#parse-start-of-trait-alias)
  - [`parse_rest_of_trait_alias`](#parse-rest-of-trait-alias)
  - [`parse_trait_item_type`](#parse-trait-item-type)
  - [`parse_impl`](#parse-impl)
  - [`parse_impl_item_fn`](#parse-impl-item-fn)
  - [`parse_impl_item_type`](#parse-impl-item-type)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlexibleItemType`](#flexibleitemtype) | struct |  |
| [`TypeDefaultness`](#typedefaultness) | enum |  |
| [`WhereClauseLocation`](#whereclauselocation) | enum |  |
| [`FnArgOrVariadic`](#fnargorvariadic) | enum |  |
| [`parse_rest_of_item`](#parse-rest-of-item) | fn |  |
| [`parse_macro2`](#parse-macro2) | fn |  |
| [`parse_item_use`](#parse-item-use) | fn |  |
| [`parse_use_tree`](#parse-use-tree) | fn |  |
| [`peek_signature`](#peek-signature) | fn |  |
| [`parse_signature`](#parse-signature) | fn |  |
| [`parse_rest_of_fn`](#parse-rest-of-fn) | fn |  |
| [`parse_fn_arg_or_variadic`](#parse-fn-arg-or-variadic) | fn |  |
| [`parse_receiver_begin`](#parse-receiver-begin) | fn |  |
| [`parse_rest_of_receiver`](#parse-rest-of-receiver) | fn |  |
| [`parse_fn_args`](#parse-fn-args) | fn |  |
| [`parse_foreign_item_type`](#parse-foreign-item-type) | fn |  |
| [`parse_item_type`](#parse-item-type) | fn |  |
| [`parse_trait_or_trait_alias`](#parse-trait-or-trait-alias) | fn |  |
| [`parse_rest_of_trait`](#parse-rest-of-trait) | fn |  |
| [`parse_start_of_trait_alias`](#parse-start-of-trait-alias) | fn |  |
| [`parse_rest_of_trait_alias`](#parse-rest-of-trait-alias) | fn |  |
| [`parse_trait_item_type`](#parse-trait-item-type) | fn |  |
| [`parse_impl`](#parse-impl) | fn |  |
| [`parse_impl_item_fn`](#parse-impl-item-fn) | fn |  |
| [`parse_impl_item_type`](#parse-impl-item-type) | fn |  |

## Structs

### `FlexibleItemType`

```rust
struct FlexibleItemType {
    vis: crate::restriction::Visibility,
    defaultness: Option<token::Default>,
    type_token: token::Type,
    ident: crate::ident::Ident,
    generics: crate::generics::Generics,
    colon_token: Option<token::Colon>,
    bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    ty: Option<(token::Eq, crate::ty::Type)>,
    semi_token: token::Semi,
}
```

#### Implementations

- <span id="flexibleitemtype-parse"></span>`fn parse(input: ParseStream<'_>, allow_defaultness: TypeDefaultness, where_clause_location: WhereClauseLocation) -> Result<Self>` — [`ParseStream`](../../parse/index.md#parsestream), [`TypeDefaultness`](#typedefaultness), [`WhereClauseLocation`](#whereclauselocation), [`Result`](../../error/index.md#result)

- <span id="flexibleitemtype-parse-optional-bounds"></span>`fn parse_optional_bounds(input: ParseStream<'_>) -> Result<(Option<token::Colon>, Punctuated<TypeParamBound, token::Plus>)>` — [`ParseStream`](../../parse/index.md#parsestream), [`Result`](../../error/index.md#result), [`Colon`](../../token/index.md#colon), [`Punctuated`](../../punctuated/index.md#punctuated), [`TypeParamBound`](../../generics/index.md#typeparambound), [`Plus`](../../token/index.md#plus)

- <span id="flexibleitemtype-parse-optional-definition"></span>`fn parse_optional_definition(input: ParseStream<'_>) -> Result<Option<(token::Eq, Type)>>` — [`ParseStream`](../../parse/index.md#parsestream), [`Result`](../../error/index.md#result), [`Eq`](../../token/index.md#eq), [`Type`](../../ty/index.md#type)

## Enums

### `TypeDefaultness`

```rust
enum TypeDefaultness {
    Optional,
    Disallowed,
}
```

### `WhereClauseLocation`

```rust
enum WhereClauseLocation {
    BeforeEq,
    AfterEq,
    Both,
}
```

### `FnArgOrVariadic`

```rust
enum FnArgOrVariadic {
    FnArg(crate::item::FnArg),
    Variadic(crate::item::Variadic),
}
```

## Functions

### `parse_rest_of_item`

```rust
fn parse_rest_of_item(begin: crate::parse::ParseBuffer<'_>, attrs: alloc::vec::Vec<crate::attr::Attribute>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::Item>
```

### `parse_macro2`

```rust
fn parse_macro2(begin: crate::parse::ParseBuffer<'_>, _vis: crate::restriction::Visibility, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::Item>
```

### `parse_item_use`

```rust
fn parse_item_use(input: crate::parse::ParseStream<'_>, allow_crate_root_in_path: bool) -> crate::error::Result<Option<crate::item::ItemUse>>
```

### `parse_use_tree`

```rust
fn parse_use_tree(input: crate::parse::ParseStream<'_>, allow_crate_root_in_path: bool) -> crate::error::Result<Option<crate::item::UseTree>>
```

### `peek_signature`

```rust
fn peek_signature(input: crate::parse::ParseStream<'_>, allow_safe: bool) -> bool
```

### `parse_signature`

```rust
fn parse_signature(input: crate::parse::ParseStream<'_>, allow_safe: bool) -> crate::error::Result<Option<crate::item::Signature>>
```

### `parse_rest_of_fn`

```rust
fn parse_rest_of_fn(input: crate::parse::ParseStream<'_>, attrs: alloc::vec::Vec<crate::attr::Attribute>, vis: crate::restriction::Visibility, sig: crate::item::Signature) -> crate::error::Result<crate::item::ItemFn>
```

### `parse_fn_arg_or_variadic`

```rust
fn parse_fn_arg_or_variadic(input: crate::parse::ParseStream<'_>, attrs: alloc::vec::Vec<crate::attr::Attribute>, allow_variadic: bool) -> crate::error::Result<FnArgOrVariadic>
```

### `parse_receiver_begin`

```rust
fn parse_receiver_begin(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(Option<(token::And, Option<crate::lifetime::Lifetime>)>, Option<token::Mut>, token::SelfValue)>
```

### `parse_rest_of_receiver`

```rust
fn parse_rest_of_receiver(reference: Option<(token::And, Option<crate::lifetime::Lifetime>)>, mutability: Option<token::Mut>, self_token: token::SelfValue, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::Receiver>
```

### `parse_fn_args`

```rust
fn parse_fn_args(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(crate::punctuated::Punctuated<crate::item::FnArg, token::Comma>, Option<crate::item::Variadic>)>
```

### `parse_foreign_item_type`

```rust
fn parse_foreign_item_type(begin: crate::parse::ParseBuffer<'_>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::ForeignItem>
```

### `parse_item_type`

```rust
fn parse_item_type(begin: crate::parse::ParseBuffer<'_>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::Item>
```

### `parse_trait_or_trait_alias`

```rust
fn parse_trait_or_trait_alias(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::Item>
```

### `parse_rest_of_trait`

```rust
fn parse_rest_of_trait(input: crate::parse::ParseStream<'_>, attrs: alloc::vec::Vec<crate::attr::Attribute>, vis: crate::restriction::Visibility, unsafety: Option<token::Unsafe>, auto_token: Option<token::Auto>, trait_token: token::Trait, ident: crate::ident::Ident, generics: crate::generics::Generics) -> crate::error::Result<crate::item::ItemTrait>
```

### `parse_start_of_trait_alias`

```rust
fn parse_start_of_trait_alias(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(alloc::vec::Vec<crate::attr::Attribute>, crate::restriction::Visibility, token::Trait, crate::ident::Ident, crate::generics::Generics)>
```

### `parse_rest_of_trait_alias`

```rust
fn parse_rest_of_trait_alias(input: crate::parse::ParseStream<'_>, attrs: alloc::vec::Vec<crate::attr::Attribute>, vis: crate::restriction::Visibility, trait_token: token::Trait, ident: crate::ident::Ident, generics: crate::generics::Generics) -> crate::error::Result<crate::item::ItemTraitAlias>
```

### `parse_trait_item_type`

```rust
fn parse_trait_item_type(begin: crate::parse::ParseBuffer<'_>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::TraitItem>
```

### `parse_impl`

```rust
fn parse_impl(input: crate::parse::ParseStream<'_>, allow_verbatim_impl: bool) -> crate::error::Result<Option<crate::item::ItemImpl>>
```

### `parse_impl_item_fn`

```rust
fn parse_impl_item_fn(input: crate::parse::ParseStream<'_>, allow_omitted_body: bool) -> crate::error::Result<Option<crate::item::ImplItemFn>>
```

### `parse_impl_item_type`

```rust
fn parse_impl_item_type(begin: crate::parse::ParseBuffer<'_>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::ImplItem>
```

