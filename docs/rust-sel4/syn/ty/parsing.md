**syn > ty > parsing**

# Module: ty::parsing

## Contents

**Functions**

- [`ambig_ty`](#ambig_ty)
- [`parse_bare_fn_arg`](#parse_bare_fn_arg)
- [`parse_bare_variadic`](#parse_bare_variadic)

---

## syn::ty::parsing::ambig_ty

*Function*

```rust
fn ambig_ty(input: crate::parse::ParseStream, allow_plus: bool, allow_group_generic: bool) -> crate::error::Result<crate::ty::Type>
```



## syn::ty::parsing::parse_bare_fn_arg

*Function*

```rust
fn parse_bare_fn_arg(input: crate::parse::ParseStream, allow_self: bool) -> crate::error::Result<crate::ty::BareFnArg>
```



## syn::ty::parsing::parse_bare_variadic

*Function*

```rust
fn parse_bare_variadic(input: crate::parse::ParseStream, attrs: alloc::vec::Vec<crate::attr::Attribute>) -> crate::error::Result<crate::ty::BareVariadic>
```



