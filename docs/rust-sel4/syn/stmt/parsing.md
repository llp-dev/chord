**syn > stmt > parsing**

# Module: stmt::parsing

## Contents

**Structs**

- [`AllowNoSemi`](#allownosemi)

**Functions**

- [`parse_stmt`](#parse_stmt)
- [`stmt_expr`](#stmt_expr)
- [`stmt_local`](#stmt_local)
- [`stmt_mac`](#stmt_mac)

---

## syn::stmt::parsing::AllowNoSemi

*Struct*

**Tuple Struct**: `(bool)`



## syn::stmt::parsing::parse_stmt

*Function*

```rust
fn parse_stmt(input: crate::parse::ParseStream, allow_nosemi: AllowNoSemi) -> crate::error::Result<crate::stmt::Stmt>
```



## syn::stmt::parsing::stmt_expr

*Function*

```rust
fn stmt_expr(input: crate::parse::ParseStream, allow_nosemi: AllowNoSemi, attrs: alloc::vec::Vec<crate::attr::Attribute>) -> crate::error::Result<crate::stmt::Stmt>
```



## syn::stmt::parsing::stmt_local

*Function*

```rust
fn stmt_local(input: crate::parse::ParseStream, attrs: alloc::vec::Vec<crate::attr::Attribute>) -> crate::error::Result<crate::stmt::Local>
```



## syn::stmt::parsing::stmt_mac

*Function*

```rust
fn stmt_mac(input: crate::parse::ParseStream, attrs: alloc::vec::Vec<crate::attr::Attribute>, path: crate::path::Path) -> crate::error::Result<crate::stmt::StmtMacro>
```



