**syn > generics**

# Module: generics

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Macros**

- [`generics_wrapper_impls`](#generics_wrapper_impls)

**Structs**

- [`BoundLifetimes`](#boundlifetimes) - A set of bound lifetimes: `for<'a, 'b, 'c>`.
- [`ConstParam`](#constparam) - A const generic parameter: `const LENGTH: usize`.
- [`ConstParams`](#constparams)
- [`ConstParamsMut`](#constparamsmut)
- [`Generics`](#generics) - Lifetimes and type parameters attached to a declaration of a function,
- [`ImplGenerics`](#implgenerics) - Returned by `Generics::split_for_impl`.
- [`LifetimeParam`](#lifetimeparam) - A lifetime definition: `'a: 'b + 'c + 'd`.
- [`Lifetimes`](#lifetimes)
- [`LifetimesMut`](#lifetimesmut)
- [`PreciseCapture`](#precisecapture) - Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait +
- [`PredicateLifetime`](#predicatelifetime) - A lifetime predicate in a `where` clause: `'a: 'b + 'c`.
- [`PredicateType`](#predicatetype) - A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.
- [`TraitBound`](#traitbound) - A trait used as a bound on a type parameter.
- [`Turbofish`](#turbofish) - Returned by `TypeGenerics::as_turbofish`.
- [`TypeGenerics`](#typegenerics) - Returned by `Generics::split_for_impl`.
- [`TypeParam`](#typeparam) - A generic type parameter: `T: Into<String>`.
- [`TypeParams`](#typeparams)
- [`TypeParamsMut`](#typeparamsmut)
- [`WhereClause`](#whereclause) - A `where` clause in a definition: `where T: Deserialize<'de>, D:

**Enums**

- [`CapturedParam`](#capturedparam) - Single parameter in a precise capturing bound.
- [`GenericParam`](#genericparam) - A generic type parameter, lifetime, or const generic: `T: Into<String>`,
- [`TraitBoundModifier`](#traitboundmodifier) - A modifier on a trait bound, currently only used for the `?` in
- [`TypeParamBound`](#typeparambound) - A trait or lifetime used as a bound on a type parameter.
- [`WherePredicate`](#wherepredicate) - A single predicate in a `where` clause: `T: Deserialize<'de>`.

---

## syn::generics::BoundLifetimes

*Struct*

A set of bound lifetimes: `for<'a, 'b, 'c>`.

**Fields:**
- `for_token: $crate::token::For`
- `lt_token: $crate::token::Lt`
- `lifetimes: crate::punctuated::Punctuated<GenericParam, $crate::token::Comma>`
- `gt_token: $crate::token::Gt`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::generics::CapturedParam

*Enum*

Single parameter in a precise capturing bound.

**Variants:**
- `Lifetime(crate::lifetime::Lifetime)` - A lifetime parameter in precise capturing bound: `fn f<'a>() -> impl
- `Ident(crate::ident::Ident)` - A type parameter or const generic parameter in precise capturing

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::generics::ConstParam

*Struct*

A const generic parameter: `const LENGTH: usize`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `const_token: $crate::token::Const`
- `ident: crate::ident::Ident`
- `colon_token: $crate::token::Colon`
- `ty: crate::ty::Type`
- `eq_token: Option<$crate::token::Eq>`
- `default: Option<crate::expr::Expr>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::generics::ConstParams

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `(crate::punctuated::Iter<'a, GenericParam>)`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syn::generics::ConstParamsMut

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `(crate::punctuated::IterMut<'a, GenericParam>)`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syn::generics::GenericParam

*Enum*

A generic type parameter, lifetime, or const generic: `T: Into<String>`,
`'a: 'b`, `const LEN: usize`.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Lifetime(LifetimeParam)` - A lifetime parameter: `'a: 'b + 'c + 'd`.
- `Type(TypeParam)` - A generic type parameter: `T: Into<String>`.
- `Const(ConstParam)` - A const generic parameter: `const LENGTH: usize`.

**Trait Implementations:**

- **From**
  - `fn from(e: LifetimeParam) -> GenericParam`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`
- **From**
  - `fn from(e: ConstParam) -> GenericParam`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: TypeParam) -> GenericParam`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::generics::Generics

*Struct*

Lifetimes and type parameters attached to a declaration of a function,
enum, trait, etc.

This struct represents two distinct optional syntactic elements,
[generic parameters] and [where clause]. In some locations of the
grammar, there may be other tokens in between these two things.

[generic parameters]: https://doc.rust-lang.org/stable/reference/items/generics.html#generic-parameters
[where clause]: https://doc.rust-lang.org/stable/reference/items/generics.html#where-clauses

**Fields:**
- `lt_token: Option<$crate::token::Lt>`
- `params: crate::punctuated::Punctuated<GenericParam, $crate::token::Comma>`
- `gt_token: Option<$crate::token::Gt>`
- `where_clause: Option<WhereClause>`

**Methods:**

- `fn lifetimes(self: &Self) -> Lifetimes` - Iterator over the lifetime parameters in `self.params`.
- `fn lifetimes_mut(self: & mut Self) -> LifetimesMut` - Iterator over the lifetime parameters in `self.params`.
- `fn type_params(self: &Self) -> TypeParams` - Iterator over the type parameters in `self.params`.
- `fn type_params_mut(self: & mut Self) -> TypeParamsMut` - Iterator over the type parameters in `self.params`.
- `fn const_params(self: &Self) -> ConstParams` - Iterator over the constant parameters in `self.params`.
- `fn const_params_mut(self: & mut Self) -> ConstParamsMut` - Iterator over the constant parameters in `self.params`.
- `fn make_where_clause(self: & mut Self) -> & mut WhereClause` - Initializes an empty `where`-clause if there is not one present already.
- `fn split_for_impl(self: &Self) -> (ImplGenerics, TypeGenerics, Option<&WhereClause>)` - Split a type's generics into the pieces required for impl'ing a trait

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::generics::ImplGenerics

*Struct*

Returned by `Generics::split_for_impl`.

**Generic Parameters:**
- 'a

**Tuple Struct**: `(&'a Generics)`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::generics::LifetimeParam

*Struct*

A lifetime definition: `'a: 'b + 'c + 'd`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `lifetime: crate::lifetime::Lifetime`
- `colon_token: Option<$crate::token::Colon>`
- `bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, $crate::token::Plus>`

**Methods:**

- `fn new(lifetime: Lifetime) -> Self`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::generics::Lifetimes

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `(crate::punctuated::Iter<'a, GenericParam>)`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syn::generics::LifetimesMut

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `(crate::punctuated::IterMut<'a, GenericParam>)`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syn::generics::PreciseCapture

*Struct*

Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait +
use<'a, T>`.

**Fields:**
- `use_token: $crate::token::Use`
- `lt_token: $crate::token::Lt`
- `params: crate::punctuated::Punctuated<CapturedParam, $crate::token::Comma>`
- `gt_token: $crate::token::Gt`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::generics::PredicateLifetime

*Struct*

A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

**Fields:**
- `lifetime: crate::lifetime::Lifetime`
- `colon_token: $crate::token::Colon`
- `bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, $crate::token::Plus>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::generics::PredicateType

*Struct*

A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

**Fields:**
- `lifetimes: Option<BoundLifetimes>` - Any lifetimes from a `for` binding
- `bounded_ty: crate::ty::Type` - The type being bounded
- `colon_token: $crate::token::Colon`
- `bounds: crate::punctuated::Punctuated<TypeParamBound, $crate::token::Plus>` - Trait and lifetime bounds (`Clone+Send+'static`)

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::generics::TraitBound

*Struct*

A trait used as a bound on a type parameter.

**Fields:**
- `paren_token: Option<token::Paren>`
- `modifier: TraitBoundModifier`
- `lifetimes: Option<BoundLifetimes>` - The `for<'a>` in `for<'a> Foo<&'a T>`
- `path: crate::path::Path` - The `Foo<&'a T>` in `for<'a> Foo<&'a T>`

**Methods:**

- `fn do_parse(input: ParseStream, allow_const: bool) -> Result<Option<Self>>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::generics::TraitBoundModifier

*Enum*

A modifier on a trait bound, currently only used for the `?` in
`?Sized`.

**Variants:**
- `None`
- `Maybe($crate::token::Question)`

**Traits:** Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::generics::Turbofish

*Struct*

Returned by `TypeGenerics::as_turbofish`.

**Generic Parameters:**
- 'a

**Tuple Struct**: `(&'a Generics)`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::generics::TypeGenerics

*Struct*

Returned by `Generics::split_for_impl`.

**Generic Parameters:**
- 'a

**Tuple Struct**: `(&'a Generics)`

**Methods:**

- `fn as_turbofish(self: &Self) -> Turbofish<'a>` - Turn a type's generics like `<X, Y>` into a turbofish like `::<X, Y>`.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::generics::TypeParam

*Struct*

A generic type parameter: `T: Into<String>`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `ident: crate::ident::Ident`
- `colon_token: Option<$crate::token::Colon>`
- `bounds: crate::punctuated::Punctuated<TypeParamBound, $crate::token::Plus>`
- `eq_token: Option<$crate::token::Eq>`
- `default: Option<crate::ty::Type>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(ident: Ident) -> Self`



## syn::generics::TypeParamBound

*Enum*

A trait or lifetime used as a bound on a type parameter.

**Variants:**
- `Trait(TraitBound)`
- `Lifetime(crate::lifetime::Lifetime)`
- `PreciseCapture(PreciseCapture)`
- `Verbatim(proc_macro2::TokenStream)`

**Methods:**

- `fn parse_single(input: ParseStream, allow_precise_capture: bool, allow_const: bool) -> Result<Self>`
- `fn parse_multiple(input: ParseStream, allow_plus: bool, allow_precise_capture: bool, allow_const: bool) -> Result<Punctuated<Self, $crate::token::Plus>>`

**Trait Implementations:**

- **From**
  - `fn from(e: PreciseCapture) -> TypeParamBound`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: Lifetime) -> TypeParamBound`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(e: TraitBound) -> TypeParamBound`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`



## syn::generics::TypeParams

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `(crate::punctuated::Iter<'a, GenericParam>)`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syn::generics::TypeParamsMut

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `(crate::punctuated::IterMut<'a, GenericParam>)`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syn::generics::WhereClause

*Struct*

A `where` clause in a definition: `where T: Deserialize<'de>, D:
'static`.

**Fields:**
- `where_token: $crate::token::Where`
- `predicates: crate::punctuated::Punctuated<WherePredicate, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::generics::WherePredicate

*Enum*

A single predicate in a `where` clause: `T: Deserialize<'de>`.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Lifetime(PredicateLifetime)` - A lifetime predicate in a `where` clause: `'a: 'b + 'c`.
- `Type(PredicateType)` - A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`
- **From**
  - `fn from(e: PredicateType) -> WherePredicate`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: PredicateLifetime) -> WherePredicate`



## syn::generics::generics_wrapper_impls

*Declarative Macro*

```rust
macro_rules! generics_wrapper_impls {
    ($ty:ident) => { ... };
}
```



## Module: parsing



## Module: printing



