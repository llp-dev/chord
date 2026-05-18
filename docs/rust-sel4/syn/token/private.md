**syn > token > private**

# Module: token::private

## Contents

**Structs**

- [`WithSpan`](#withspan) - Support writing `token.span` rather than `token.spans[0]` on tokens that

**Traits**

- [`Sealed`](#sealed)

---

## syn::token::private::Sealed

*Trait*



## syn::token::private::WithSpan

*Struct*

Support writing `token.span` rather than `token.spans[0]` on tokens that
hold a single span.

**Fields:**
- `span: proc_macro2::Span`



