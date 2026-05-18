**verus_builtin_macros > contrib > set_build**

# Module: contrib::set_build

## Contents

**Structs**

- [`Decl`](#decl)
- [`Element`](#element)
- [`SetBuild`](#setbuild)

**Enums**

- [`DeclOrCond`](#declorcond)
- [`Source`](#source)

**Functions**

- [`conds_to_expr`](#conds_to_expr)
- [`decl_to_expr`](#decl_to_expr)
- [`is_exactly_x`](#is_exactly_x)
- [`is_probably_datatype_name`](#is_probably_datatype_name)
- [`parse_set_build`](#parse_set_build)
- [`reverse`](#reverse)
- [`reverse_rec`](#reverse_rec)
- [`rewrite_expr_to_token_stream`](#rewrite_expr_to_token_stream)
- [`set_build`](#set_build)
- [`set_build_inner`](#set_build_inner)

---

## verus_builtin_macros::contrib::set_build::Decl

*Struct*

**Fields:**
- `x: verus_syn::Ident`
- `typ: Box<verus_syn::Type>`
- `source: Source`
- `is_exists: bool`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::contrib::set_build::DeclOrCond

*Enum*

**Variants:**
- `Decl(Decl)`
- `Cond(Box<verus_syn::Expr>)`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self, Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::contrib::set_build::Element

*Struct*

**Fields:**
- `expr: Box<verus_syn::Expr>`
- `typ: Option<Box<verus_syn::Type>>`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self, Error>`



## verus_builtin_macros::contrib::set_build::SetBuild

*Struct*

**Fields:**
- `element: Element`
- `decls: Vec<Decl>`
- `conds: Vec<verus_syn::Expr>`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::contrib::set_build::Source

*Enum*

**Variants:**
- `FiniteType`
- `Set(Box<verus_syn::Expr>)`
- `Range{ start: Box<verus_syn::Expr>, end: Box<verus_syn::Expr>, inclusive_end: bool }`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::contrib::set_build::conds_to_expr

*Function*

```rust
fn conds_to_expr(conds: &Vec<verus_syn::Expr>) -> Box<verus_syn::Expr>
```



## verus_builtin_macros::contrib::set_build::decl_to_expr

*Function*

```rust
fn decl_to_expr(d: &Decl) -> verus_syn::Expr
```



## verus_builtin_macros::contrib::set_build::is_exactly_x

*Function*

```rust
fn is_exactly_x(x: &verus_syn::Ident, e: &verus_syn::Expr) -> bool
```



## verus_builtin_macros::contrib::set_build::is_probably_datatype_name

*Function*

```rust
fn is_probably_datatype_name(e: &verus_syn::Expr) -> bool
```



## verus_builtin_macros::contrib::set_build::parse_set_build

*Function*

```rust
fn parse_set_build(input: proc_macro2::TokenStream) -> Result<SetBuild, verus_syn::Error>
```



## verus_builtin_macros::contrib::set_build::reverse

*Function*

```rust
fn reverse(build: &SetBuild, x: &verus_syn::Ident) -> Result<verus_syn::Expr, verus_syn::Error>
```



## verus_builtin_macros::contrib::set_build::reverse_rec

*Function*

```rust
fn reverse_rec(build: &SetBuild, x: &verus_syn::Ident, e: &verus_syn::Expr) -> Option<Box<dyn Fn>>
```



## verus_builtin_macros::contrib::set_build::rewrite_expr_to_token_stream

*Function*

```rust
fn rewrite_expr_to_token_stream(expr: &verus_syn::Expr) -> proc_macro2::TokenStream
```



## verus_builtin_macros::contrib::set_build::set_build

*Function*

```rust
fn set_build(input: proc_macro::TokenStream, debug: bool) -> proc_macro::TokenStream
```



## verus_builtin_macros::contrib::set_build::set_build_inner

*Function*

```rust
fn set_build_inner(input: proc_macro2::TokenStream, debug: bool) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



