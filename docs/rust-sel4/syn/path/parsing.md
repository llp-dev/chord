**syn > path > parsing**

# Module: path::parsing

## Contents

**Functions**

- [`const_argument`](#const_argument)
- [`qpath`](#qpath)

---

## syn::path::parsing::const_argument

*Function*

```rust
fn const_argument(input: crate::parse::ParseStream) -> crate::error::Result<crate::expr::Expr>
```



## syn::path::parsing::qpath

*Function*

```rust
fn qpath(input: crate::parse::ParseStream, expr_style: bool) -> crate::error::Result<(Option<crate::path::QSelf>, crate::path::Path)>
```



