**syn > spanned**

# Module: spanned

## Contents

**Modules**

- [`private`](#private)

**Traits**

- [`Spanned`](#spanned) - A trait that can provide the `Span` of the complete contents of a syntax

---

## syn::spanned::Spanned

*Trait*

A trait that can provide the `Span` of the complete contents of a syntax
tree node.

This trait is automatically implemented for all types that implement
[`ToTokens`] from the `quote` crate, as well as for `Span` itself.

[`ToTokens`]: quote::ToTokens

See the [module documentation] for an example.

[module documentation]: self

**Methods:**

- `span`: Returns a `Span` covering the complete contents of this syntax tree



## Module: private



