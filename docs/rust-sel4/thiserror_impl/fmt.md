**thiserror_impl > fmt**

# Module: fmt

## Contents

**Structs**

- [`FmtArguments`](#fmtarguments)

**Functions**

- [`between`](#between)
- [`explicit_named_args`](#explicit_named_args)
- [`fallback_explicit_named_args`](#fallback_explicit_named_args)
- [`is_syn_full`](#is_syn_full)
- [`take_ident`](#take_ident)
- [`take_int`](#take_int)
- [`try_explicit_named_args`](#try_explicit_named_args)

---

## thiserror_impl::fmt::FmtArguments

*Struct*

**Fields:**
- `named: std::collections::BTreeSet<crate::unraw::IdentUnraw>`
- `first_unnamed: Option<proc_macro2::TokenStream>`



## thiserror_impl::fmt::between

*Function*

```rust
fn between<'a>(begin: syn::parse::ParseStream<'a>, end: syn::parse::ParseStream<'a>) -> proc_macro2::TokenStream
```



## thiserror_impl::fmt::explicit_named_args

*Function*

```rust
fn explicit_named_args(input: syn::parse::ParseStream) -> syn::parse::Result<FmtArguments>
```



## thiserror_impl::fmt::fallback_explicit_named_args

*Function*

```rust
fn fallback_explicit_named_args(input: syn::parse::ParseStream) -> syn::parse::Result<FmtArguments>
```



## thiserror_impl::fmt::is_syn_full

*Function*

```rust
fn is_syn_full() -> bool
```



## thiserror_impl::fmt::take_ident

*Function*

```rust
fn take_ident<'a>(read: & mut &'a str) -> &'a str
```



## thiserror_impl::fmt::take_int

*Function*

```rust
fn take_int<'a>(read: & mut &'a str) -> &'a str
```



## thiserror_impl::fmt::try_explicit_named_args

*Function*

```rust
fn try_explicit_named_args(input: syn::parse::ParseStream) -> syn::parse::Result<FmtArguments>
```



