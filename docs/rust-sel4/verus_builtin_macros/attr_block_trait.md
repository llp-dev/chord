**verus_builtin_macros > attr_block_trait**

# Module: attr_block_trait

## Contents

**Enums**

- [`AnyFnOrLoop`](#anyfnorloop)

**Traits**

- [`AnyAttrBlock`](#anyattrblock)

---

## verus_builtin_macros::attr_block_trait::AnyAttrBlock

*Trait*

**Methods:**

- `attrs_mut`
- `block_mut`



## verus_builtin_macros::attr_block_trait::AnyFnOrLoop

*Enum*

**Variants:**
- `Fn(syn::ItemFn)`
- `TraitMethod(syn::TraitItemFn)`
- `Loop(syn::ExprLoop)`
- `ForLoop(syn::ExprForLoop)`
- `While(syn::ExprWhile)`
- `Closure(syn::ExprClosure)`

**Traits:** Eq

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut proc_macro2::TokenStream)`
- **PartialEq**
  - `fn eq(self: &Self, other: &AnyFnOrLoop) -> bool`
- **Parse**
  - `fn parse(input: syn::parse::ParseStream) -> syn::Result<Self>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



