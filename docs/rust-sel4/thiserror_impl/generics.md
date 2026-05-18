**thiserror_impl > generics**

# Module: generics

## Contents

**Structs**

- [`InferredBounds`](#inferredbounds)
- [`ParamsInScope`](#paramsinscope)

**Functions**

- [`crawl`](#crawl)

---

## thiserror_impl::generics::InferredBounds

*Struct*

**Fields:**
- `bounds: std::collections::BTreeMap<String, (std::collections::BTreeSet<String>, syn::punctuated::Punctuated<proc_macro2::TokenStream, $crate::token::Plus>)>`
- `order: Vec<proc_macro2::TokenStream>`

**Methods:**

- `fn new() -> Self`
- `fn insert<impl ToTokens, impl ToTokens>(self: & mut Self, ty: impl Trait, bound: impl Trait)`
- `fn augment_where_clause(self: &Self, generics: &Generics) -> WhereClause`



## thiserror_impl::generics::ParamsInScope

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `names: std::collections::BTreeSet<&'a syn::Ident>`

**Methods:**

- `fn new(generics: &'a Generics) -> Self`
- `fn intersects(self: &Self, ty: &Type) -> bool`



## thiserror_impl::generics::crawl

*Function*

```rust
fn crawl(in_scope: &ParamsInScope, ty: &syn::Type, found: & mut bool)
```



