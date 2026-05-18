**syn > item > parsing**

# Module: item::parsing

## Contents

**Structs**

- [`FlexibleItemType`](#flexibleitemtype)

**Enums**

- [`FnArgOrVariadic`](#fnargorvariadic)
- [`TypeDefaultness`](#typedefaultness)
- [`WhereClauseLocation`](#whereclauselocation)

**Functions**

- [`parse_fn_arg_or_variadic`](#parse_fn_arg_or_variadic)
- [`parse_fn_args`](#parse_fn_args)
- [`parse_foreign_item_type`](#parse_foreign_item_type)
- [`parse_impl`](#parse_impl)
- [`parse_impl_item_fn`](#parse_impl_item_fn)
- [`parse_impl_item_type`](#parse_impl_item_type)
- [`parse_item_type`](#parse_item_type)
- [`parse_item_use`](#parse_item_use)
- [`parse_macro2`](#parse_macro2)
- [`parse_receiver_begin`](#parse_receiver_begin)
- [`parse_rest_of_fn`](#parse_rest_of_fn)
- [`parse_rest_of_item`](#parse_rest_of_item)
- [`parse_rest_of_receiver`](#parse_rest_of_receiver)
- [`parse_rest_of_trait`](#parse_rest_of_trait)
- [`parse_rest_of_trait_alias`](#parse_rest_of_trait_alias)
- [`parse_signature`](#parse_signature)
- [`parse_start_of_trait_alias`](#parse_start_of_trait_alias)
- [`parse_trait_item_type`](#parse_trait_item_type)
- [`parse_trait_or_trait_alias`](#parse_trait_or_trait_alias)
- [`parse_use_tree`](#parse_use_tree)
- [`peek_signature`](#peek_signature)

---

## syn::item::parsing::FlexibleItemType

*Struct*

**Fields:**
- `vis: crate::restriction::Visibility`
- `defaultness: Option<$crate::token::Default>`
- `type_token: $crate::token::Type`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `colon_token: Option<$crate::token::Colon>`
- `bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>`
- `ty: Option<($crate::token::Eq, crate::ty::Type)>`
- `semi_token: $crate::token::Semi`

**Methods:**

- `fn parse(input: ParseStream, allow_defaultness: TypeDefaultness, where_clause_location: WhereClauseLocation) -> Result<Self>`
- `fn parse_optional_bounds(input: ParseStream) -> Result<(Option<$crate::token::Colon>, Punctuated<TypeParamBound, $crate::token::Plus>)>`
- `fn parse_optional_definition(input: ParseStream) -> Result<Option<($crate::token::Eq, Type)>>`



## syn::item::parsing::FnArgOrVariadic

*Enum*

**Variants:**
- `FnArg(crate::item::FnArg)`
- `Variadic(crate::item::Variadic)`



## syn::item::parsing::TypeDefaultness

*Enum*

**Variants:**
- `Optional`
- `Disallowed`



## syn::item::parsing::WhereClauseLocation

*Enum*

**Variants:**
- `BeforeEq`
- `AfterEq`
- `Both`



## syn::item::parsing::parse_fn_arg_or_variadic

*Function*

```rust
fn parse_fn_arg_or_variadic(input: crate::parse::ParseStream, attrs: alloc::vec::Vec<crate::attr::Attribute>, allow_variadic: bool) -> crate::error::Result<FnArgOrVariadic>
```



## syn::item::parsing::parse_fn_args

*Function*

```rust
fn parse_fn_args(input: crate::parse::ParseStream) -> crate::error::Result<(crate::punctuated::Punctuated<crate::item::FnArg, $crate::token::Comma>, Option<crate::item::Variadic>)>
```



## syn::item::parsing::parse_foreign_item_type

*Function*

```rust
fn parse_foreign_item_type(begin: crate::parse::ParseBuffer, input: crate::parse::ParseStream) -> crate::error::Result<crate::item::ForeignItem>
```



## syn::item::parsing::parse_impl

*Function*

```rust
fn parse_impl(input: crate::parse::ParseStream, allow_verbatim_impl: bool) -> crate::error::Result<Option<crate::item::ItemImpl>>
```



## syn::item::parsing::parse_impl_item_fn

*Function*

```rust
fn parse_impl_item_fn(input: crate::parse::ParseStream, allow_omitted_body: bool) -> crate::error::Result<Option<crate::item::ImplItemFn>>
```



## syn::item::parsing::parse_impl_item_type

*Function*

```rust
fn parse_impl_item_type(begin: crate::parse::ParseBuffer, input: crate::parse::ParseStream) -> crate::error::Result<crate::item::ImplItem>
```



## syn::item::parsing::parse_item_type

*Function*

```rust
fn parse_item_type(begin: crate::parse::ParseBuffer, input: crate::parse::ParseStream) -> crate::error::Result<crate::item::Item>
```



## syn::item::parsing::parse_item_use

*Function*

```rust
fn parse_item_use(input: crate::parse::ParseStream, allow_crate_root_in_path: bool) -> crate::error::Result<Option<crate::item::ItemUse>>
```



## syn::item::parsing::parse_macro2

*Function*

```rust
fn parse_macro2(begin: crate::parse::ParseBuffer, _vis: crate::restriction::Visibility, input: crate::parse::ParseStream) -> crate::error::Result<crate::item::Item>
```



## syn::item::parsing::parse_receiver_begin

*Function*

```rust
fn parse_receiver_begin(input: crate::parse::ParseStream) -> crate::error::Result<(Option<($crate::token::And, Option<crate::lifetime::Lifetime>)>, Option<$crate::token::Mut>, $crate::token::SelfValue)>
```



## syn::item::parsing::parse_rest_of_fn

*Function*

```rust
fn parse_rest_of_fn(input: crate::parse::ParseStream, attrs: alloc::vec::Vec<crate::attr::Attribute>, vis: crate::restriction::Visibility, sig: crate::item::Signature) -> crate::error::Result<crate::item::ItemFn>
```



## syn::item::parsing::parse_rest_of_item

*Function*

```rust
fn parse_rest_of_item(begin: crate::parse::ParseBuffer, attrs: alloc::vec::Vec<crate::attr::Attribute>, input: crate::parse::ParseStream) -> crate::error::Result<crate::item::Item>
```



## syn::item::parsing::parse_rest_of_receiver

*Function*

```rust
fn parse_rest_of_receiver(reference: Option<($crate::token::And, Option<crate::lifetime::Lifetime>)>, mutability: Option<$crate::token::Mut>, self_token: $crate::token::SelfValue, input: crate::parse::ParseStream) -> crate::error::Result<crate::item::Receiver>
```



## syn::item::parsing::parse_rest_of_trait

*Function*

```rust
fn parse_rest_of_trait(input: crate::parse::ParseStream, attrs: alloc::vec::Vec<crate::attr::Attribute>, vis: crate::restriction::Visibility, unsafety: Option<$crate::token::Unsafe>, auto_token: Option<$crate::token::Auto>, trait_token: $crate::token::Trait, ident: crate::ident::Ident, generics: crate::generics::Generics) -> crate::error::Result<crate::item::ItemTrait>
```



## syn::item::parsing::parse_rest_of_trait_alias

*Function*

```rust
fn parse_rest_of_trait_alias(input: crate::parse::ParseStream, attrs: alloc::vec::Vec<crate::attr::Attribute>, vis: crate::restriction::Visibility, trait_token: $crate::token::Trait, ident: crate::ident::Ident, generics: crate::generics::Generics) -> crate::error::Result<crate::item::ItemTraitAlias>
```



## syn::item::parsing::parse_signature

*Function*

```rust
fn parse_signature(input: crate::parse::ParseStream, allow_safe: bool) -> crate::error::Result<Option<crate::item::Signature>>
```



## syn::item::parsing::parse_start_of_trait_alias

*Function*

```rust
fn parse_start_of_trait_alias(input: crate::parse::ParseStream) -> crate::error::Result<(alloc::vec::Vec<crate::attr::Attribute>, crate::restriction::Visibility, $crate::token::Trait, crate::ident::Ident, crate::generics::Generics)>
```



## syn::item::parsing::parse_trait_item_type

*Function*

```rust
fn parse_trait_item_type(begin: crate::parse::ParseBuffer, input: crate::parse::ParseStream) -> crate::error::Result<crate::item::TraitItem>
```



## syn::item::parsing::parse_trait_or_trait_alias

*Function*

```rust
fn parse_trait_or_trait_alias(input: crate::parse::ParseStream) -> crate::error::Result<crate::item::Item>
```



## syn::item::parsing::parse_use_tree

*Function*

```rust
fn parse_use_tree(input: crate::parse::ParseStream, allow_crate_root_in_path: bool) -> crate::error::Result<Option<crate::item::UseTree>>
```



## syn::item::parsing::peek_signature

*Function*

```rust
fn peek_signature(input: crate::parse::ParseStream, allow_safe: bool) -> bool
```



