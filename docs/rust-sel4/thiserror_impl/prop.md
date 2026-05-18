**thiserror_impl > prop**

# Module: prop

## Contents

**Functions**

- [`backtrace_field`](#backtrace_field)
- [`distinct_backtrace_field`](#distinct_backtrace_field)
- [`from_field`](#from_field)
- [`source_field`](#source_field)
- [`type_is_backtrace`](#type_is_backtrace)

---

## thiserror_impl::prop::backtrace_field

*Function*

```rust
fn backtrace_field<'a, 'b>(fields: &'a [crate::ast::Field<'b>]) -> Option<&'a crate::ast::Field<'b>>
```



## thiserror_impl::prop::distinct_backtrace_field

*Function*

```rust
fn distinct_backtrace_field<'a, 'b>(backtrace_field: &'a crate::ast::Field<'b>, from_field: Option<&crate::ast::Field>) -> Option<&'a crate::ast::Field<'b>>
```



## thiserror_impl::prop::from_field

*Function*

```rust
fn from_field<'a, 'b>(fields: &'a [crate::ast::Field<'b>]) -> Option<&'a crate::ast::Field<'b>>
```



## thiserror_impl::prop::source_field

*Function*

```rust
fn source_field<'a, 'b>(fields: &'a [crate::ast::Field<'b>]) -> Option<&'a crate::ast::Field<'b>>
```



## thiserror_impl::prop::type_is_backtrace

*Function*

```rust
fn type_is_backtrace(ty: &syn::Type) -> bool
```



