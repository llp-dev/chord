**syn > stmt**

# Module: stmt

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`Block`](#block) - A braced block containing Rust statements.
- [`Local`](#local) - A local `let` binding: `let x: u64 = s.parse()?;`.
- [`LocalInit`](#localinit) - The expression assigned in a local `let` binding, including optional
- [`StmtMacro`](#stmtmacro) - A macro invocation in statement position.

**Enums**

- [`Stmt`](#stmt) - A statement, usually ending in a semicolon.

---

## syn::stmt::Block

*Struct*

A braced block containing Rust statements.

**Fields:**
- `brace_token: token::Brace`
- `stmts: alloc::vec::Vec<Stmt>` - Statements in a block

**Methods:**

- `fn parse_within(input: ParseStream) -> Result<Vec<Stmt>>` - Parse the body of a block as zero or more statements, possibly

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::stmt::Local

*Struct*

A local `let` binding: `let x: u64 = s.parse()?;`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `let_token: $crate::token::Let`
- `pat: crate::pat::Pat`
- `init: Option<LocalInit>`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::stmt::LocalInit

*Struct*

The expression assigned in a local `let` binding, including optional
diverging `else` block.

`LocalInit` represents `= s.parse()?` in `let x: u64 = s.parse()?` and
`= r else { return }` in `let Ok(x) = r else { return }`.

**Fields:**
- `eq_token: $crate::token::Eq`
- `expr: alloc::boxed::Box<crate::expr::Expr>`
- `diverge: Option<($crate::token::Else, alloc::boxed::Box<crate::expr::Expr>)>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::stmt::Stmt

*Enum*

A statement, usually ending in a semicolon.

**Variants:**
- `Local(Local)` - A local (let) binding.
- `Item(crate::item::Item)` - An item definition.
- `Expr(crate::expr::Expr, Option<$crate::token::Semi>)` - Expression, with or without trailing semicolon.
- `Macro(StmtMacro)` - A macro invocation in statement position.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::stmt::StmtMacro

*Struct*

A macro invocation in statement position.

Syntactically it's ambiguous which other kind of statement this macro
would expand to. It can be any of local variable (`let`), item, or
expression.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `mac: crate::mac::Macro`
- `semi_token: Option<$crate::token::Semi>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## Module: parsing



## Module: printing



