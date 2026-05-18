**syn > lit > parsing**

# Module: lit::parsing

## Contents

**Macros**

- [`impl_token`](#impl_token)

**Functions**

- [`parse_negative_lit`](#parse_negative_lit)
- [`peek_impl`](#peek_impl)

---

## syn::lit::parsing::impl_token

*Declarative Macro*

```rust
macro_rules! impl_token {
    ($display:literal $name:ty) => { ... };
}
```



## syn::lit::parsing::parse_negative_lit

*Function*

```rust
fn parse_negative_lit(neg: proc_macro2::Punct, cursor: crate::buffer::Cursor) -> Option<(crate::lit::Lit, crate::buffer::Cursor)>
```



## syn::lit::parsing::peek_impl

*Function*

```rust
fn peek_impl(cursor: crate::buffer::Cursor, peek: fn(...)) -> bool
```



