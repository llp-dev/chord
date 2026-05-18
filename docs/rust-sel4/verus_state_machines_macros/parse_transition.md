**verus_state_machines_macros > parse_transition**

# Module: parse_transition

## Contents

**Structs**

- [`Ctxt`](#ctxt)
- [`IfLetVisitor`](#ifletvisitor)
- [`TLet`](#tlet)
- [`TSpecial`](#tspecial)
- [`TransitionInner`](#transitioninner)

**Enums**

- [`Refute`](#refute)
- [`StmtOrLet`](#stmtorlet)

**Functions**

- [`error_on_if_let`](#error_on_if_let) - Error if the user writes `if let ... = ... { ... }` which is unsupported.
- [`multi_pat_impl`](#multi_pat_impl)
- [`multi_pat_with_leading_vert`](#multi_pat_with_leading_vert)
- [`parse_arg_typed`](#parse_arg_typed)
- [`parse_arm`](#parse_arm) - Parse an arm of a match statement. Based on `impl Parse for syn::Arm`
- [`parse_assert`](#parse_assert) - Parse `assert ...;` or `assert ... by { ... }`
- [`parse_conditional`](#parse_conditional) - Parse conditional `if { stmts } else { stmts }`
- [`parse_else_block`](#parse_else_block)
- [`parse_expr_for_monoid_elt`](#parse_expr_for_monoid_elt)
- [`parse_init`](#parse_init) - Parse `init field = ...;`
- [`parse_let`](#parse_let) - Parse `let x = ...;` or `birds_eye let x = ...;`
- [`parse_match`](#parse_match) - Parse `match ... { ... }`
- [`parse_monoid_elt`](#parse_monoid_elt) - Parse the element to be added, removed, etc. Looks like one of:
- [`parse_monoid_stmt`](#parse_monoid_stmt) - Parse a statement that looks like `add field += ...;`
- [`parse_params`](#parse_params)
- [`parse_require`](#parse_require) - Parse `require ...;`
- [`parse_transition`](#parse_transition) - Translate Rust AST into a transition AST by parsing our transition DSL.
- [`parse_transition_block`](#parse_transition_block) - Parse a block `{ transition stmts }`
- [`parse_transition_stmt`](#parse_transition_stmt) - Parse any kind of transition statement. Note that 'let' statements aren't turned
- [`parse_update`](#parse_update) - Parse `update field = ...;`
- [`stmts_or_lets_to_block`](#stmts_or_lets_to_block)

---

## verus_state_machines_macros::parse_transition::Ctxt

*Struct*

**Fields:**
- `counter: u64`



## verus_state_machines_macros::parse_transition::IfLetVisitor

*Struct*

**Fields:**
- `errors: Vec<verus_syn::Error>`

**Trait Implementations:**

- **Visit**
  - `fn visit_expr_block(self: & mut Self, _node: &'ast ExprBlock)`
  - `fn visit_expr_let(self: & mut Self, node: &'ast ExprLet)`



## verus_state_machines_macros::parse_transition::Refute

*Enum*

**Variants:**
- `Exhaustive`
- `Require`
- `Assert`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Refute) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Refute`



## verus_state_machines_macros::parse_transition::StmtOrLet

*Enum*

**Variants:**
- `Stmt(crate::ast::TransitionStmt)`
- `Let(TLet)`
- `Special(TSpecial)`



## verus_state_machines_macros::parse_transition::TLet

*Struct*

**Tuple Struct**: `(proc_macro2::Span, verus_syn::Pat, Option<Box<verus_syn::Type>>, crate::ast::LetKind, verus_syn::Expr)`



## verus_state_machines_macros::parse_transition::TSpecial

*Struct*

**Tuple Struct**: `(proc_macro2::Span, verus_syn::Ident, crate::ast::SpecialOp, crate::ast::AssertProof, verus_syn::Pat)`



## verus_state_machines_macros::parse_transition::TransitionInner

*Struct*

**Fields:**
- `name: verus_syn::Ident`
- `params: Vec<crate::ast::TransitionParam>`
- `body: crate::ast::TransitionStmt`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> parse::Result<TransitionInner>`



## verus_state_machines_macros::parse_transition::error_on_if_let

*Function*

Error if the user writes `if let ... = ... { ... }` which is unsupported.
Descend into the expression to check for chained if-let as well.

```rust
fn error_on_if_let(cond: &verus_syn::Expr) -> parse::Result<()>
```



## verus_state_machines_macros::parse_transition::multi_pat_impl

*Function*

```rust
fn multi_pat_impl(input: verus_syn::parse::ParseStream, leading_vert: Option<$crate::token::Or>) -> parse::Result<verus_syn::Pat>
```



## verus_state_machines_macros::parse_transition::multi_pat_with_leading_vert

*Function*

```rust
fn multi_pat_with_leading_vert(input: verus_syn::parse::ParseStream) -> parse::Result<verus_syn::Pat>
```



## verus_state_machines_macros::parse_transition::parse_arg_typed

*Function*

```rust
fn parse_arg_typed(input: verus_syn::parse::ParseStream) -> parse::Result<(verus_syn::Ident, verus_syn::Type)>
```



## verus_state_machines_macros::parse_transition::parse_arm

*Function*

Parse an arm of a match statement. Based on `impl Parse for syn::Arm`
(but note that we return our own ast::Arm, not the syn::Arm)

```rust
fn parse_arm(ctxt: & mut Ctxt, input: verus_syn::parse::ParseStream) -> parse::Result<(crate::ast::Arm, crate::ast::TransitionStmt)>
```



## verus_state_machines_macros::parse_transition::parse_assert

*Function*

Parse `assert ...;` or `assert ... by { ... }`

```rust
fn parse_assert(kw: verus_syn::Ident, input: verus_syn::parse::ParseStream) -> parse::Result<crate::ast::TransitionStmt>
```



## verus_state_machines_macros::parse_transition::parse_conditional

*Function*

Parse conditional `if { stmts } else { stmts }`

```rust
fn parse_conditional(ctxt: & mut Ctxt, input: verus_syn::parse::ParseStream) -> parse::Result<crate::ast::TransitionStmt>
```



## verus_state_machines_macros::parse_transition::parse_else_block

*Function*

```rust
fn parse_else_block(ctxt: & mut Ctxt, input: verus_syn::parse::ParseStream) -> parse::Result<crate::ast::TransitionStmt>
```



## verus_state_machines_macros::parse_transition::parse_expr_for_monoid_elt

*Function*

```rust
fn parse_expr_for_monoid_elt(input: verus_syn::parse::ParseStream) -> parse::Result<verus_syn::Expr>
```



## verus_state_machines_macros::parse_transition::parse_init

*Function*

Parse `init field = ...;`

```rust
fn parse_init(kw: verus_syn::Ident, input: verus_syn::parse::ParseStream) -> parse::Result<crate::ast::TransitionStmt>
```



## verus_state_machines_macros::parse_transition::parse_let

*Function*

Parse `let x = ...;` or `birds_eye let x = ...;`

```rust
fn parse_let(ctxt: & mut Ctxt, span: proc_macro2::Span, refute: Refute, lk: crate::ast::LetKind, input: verus_syn::parse::ParseStream) -> parse::Result<Vec<StmtOrLet>>
```



## verus_state_machines_macros::parse_transition::parse_match

*Function*

Parse `match ... { ... }`
This is based on syn's `impl Parse for ExprMatch`,
but we have to modify it so that it parses a TransitionStmt instead of an Expr
in each arm. However, we can re-use some of the code for parsing the patterns

```rust
fn parse_match(ctxt: & mut Ctxt, input: verus_syn::parse::ParseStream) -> parse::Result<crate::ast::TransitionStmt>
```



## verus_state_machines_macros::parse_transition::parse_monoid_elt

*Function*

Parse the element to be added, removed, etc. Looks like one of:

* `{x}` multiset singleton (TODO change to `multiset {...}`?
* `set {x}` set singleton
* `true`
* `[key => value]` map singleton
* `Some(x)` optional value
* `(x)` general value

```rust
fn parse_monoid_elt(input: verus_syn::parse::ParseStream, monoid_stmt_type: crate::ast::MonoidStmtType) -> parse::Result<(crate::ast::MonoidElt, Option<verus_syn::Pat>)>
```



## verus_state_machines_macros::parse_transition::parse_monoid_stmt

*Function*

Parse a statement that looks like `add field += ...;`
Assumes the parsing cursor starts after the initial keyword.
This handles add, remove, have, deposit, withdraw, guard.

```rust
fn parse_monoid_stmt(kw: verus_syn::Ident, input: verus_syn::parse::ParseStream, monoid_stmt_type: crate::ast::MonoidStmtType) -> parse::Result<StmtOrLet>
```



## verus_state_machines_macros::parse_transition::parse_params

*Function*

```rust
fn parse_params(input: verus_syn::parse::ParseStream) -> parse::Result<Vec<crate::ast::TransitionParam>>
```



## verus_state_machines_macros::parse_transition::parse_require

*Function*

Parse `require ...;`

```rust
fn parse_require(kw: verus_syn::Ident, input: verus_syn::parse::ParseStream) -> parse::Result<crate::ast::TransitionStmt>
```



## verus_state_machines_macros::parse_transition::parse_transition

*Function*

Translate Rust AST into a transition AST by parsing our transition DSL.
Every statement should be one of:
  let, if, match (similar to the same statements in Rust)
  init, update, add, remove, have, deposit, withdraw, guard (statements specific to our DSL)

```rust
fn parse_transition(mac: verus_syn::Macro) -> parse::Result<crate::ast::Transition>
```



## verus_state_machines_macros::parse_transition::parse_transition_block

*Function*

Parse a block `{ transition stmts }`

```rust
fn parse_transition_block(ctxt: & mut Ctxt, input: verus_syn::parse::ParseStream) -> parse::Result<crate::ast::TransitionStmt>
```



## verus_state_machines_macros::parse_transition::parse_transition_stmt

*Function*

Parse any kind of transition statement. Note that 'let' statements aren't turned
into full TransitionStmts yet; instead we return the TLet stub.

```rust
fn parse_transition_stmt(ctxt: & mut Ctxt, input: verus_syn::parse::ParseStream) -> parse::Result<Vec<StmtOrLet>>
```



## verus_state_machines_macros::parse_transition::parse_update

*Function*

Parse `update field = ...;`

```rust
fn parse_update(kw: verus_syn::Ident, input: verus_syn::parse::ParseStream) -> parse::Result<crate::ast::TransitionStmt>
```



## verus_state_machines_macros::parse_transition::stmts_or_lets_to_block

*Function*

```rust
fn stmts_or_lets_to_block(span: proc_macro2::Span, tstmts: Vec<StmtOrLet>) -> crate::ast::TransitionStmt
```



