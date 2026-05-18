**syn > derive > parsing**

# Module: derive::parsing

## Contents

**Functions**

- [`data_enum`](#data_enum)
- [`data_struct`](#data_struct)
- [`data_union`](#data_union)

---

## syn::derive::parsing::data_enum

*Function*

```rust
fn data_enum(input: crate::parse::ParseStream) -> crate::error::Result<(Option<crate::generics::WhereClause>, token::Brace, crate::punctuated::Punctuated<crate::data::Variant, $crate::token::Comma>)>
```



## syn::derive::parsing::data_struct

*Function*

```rust
fn data_struct(input: crate::parse::ParseStream) -> crate::error::Result<(Option<crate::generics::WhereClause>, crate::data::Fields, Option<$crate::token::Semi>)>
```



## syn::derive::parsing::data_union

*Function*

```rust
fn data_union(input: crate::parse::ParseStream) -> crate::error::Result<(Option<crate::generics::WhereClause>, crate::data::FieldsNamed)>
```



