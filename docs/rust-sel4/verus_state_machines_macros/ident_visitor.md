**verus_state_machines_macros > ident_visitor**

# Module: ident_visitor

## Contents

**Structs**

- [`IdentVisitor`](#identvisitor)
- [`PatIdentVisitor`](#patidentvisitor)
- [`SuperVisitor`](#supervisitor)

**Functions**

- [`error_on_super_path`](#error_on_super_path) - Error if the type contains a `super::...` path.
- [`pattern_get_bound_idents`](#pattern_get_bound_idents) - Get all identifiers bound by the pattern
- [`validate_ident`](#validate_ident) - Validate a single identifier.
- [`validate_idents_expr`](#validate_idents_expr)
- [`validate_idents_op`](#validate_idents_op)
- [`validate_idents_pat`](#validate_idents_pat)
- [`validate_idents_transition`](#validate_idents_transition) - Error if any identifiers conflict with reserved IDs used by macro expanion.
- [`validate_idents_transition_stmt`](#validate_idents_transition_stmt)

---

## verus_state_machines_macros::ident_visitor::IdentVisitor

*Struct*

**Fields:**
- `errors: Vec<verus_syn::Error>`
- `kind: crate::ast::TransitionKind`
- `bound_names: std::collections::HashSet<String>`
- `field_names: std::collections::HashSet<String>`

**Methods:**

- `fn new(kind: TransitionKind, bound_names: HashSet<String>, field_names: HashSet<String>) -> IdentVisitor`

**Trait Implementations:**

- **Visit**
  - `fn visit_ident(self: & mut Self, node: &'ast Ident)`
  - `fn visit_expr_macro(self: & mut Self, node: &'ast ExprMacro)`
  - `fn visit_expr(self: & mut Self, node: &'ast Expr)`



## verus_state_machines_macros::ident_visitor::PatIdentVisitor

*Struct*

**Fields:**
- `idents: Vec<verus_syn::Ident>`

**Methods:**

- `fn new() -> PatIdentVisitor`

**Trait Implementations:**

- **Visit**
  - `fn visit_pat_ident(self: & mut Self, node: &'ast PatIdent)`



## verus_state_machines_macros::ident_visitor::SuperVisitor

*Struct*

**Fields:**
- `errors: Vec<verus_syn::Error>`

**Trait Implementations:**

- **Visit**
  - `fn visit_macro(self: & mut Self, node: &'ast Macro)`
  - `fn visit_path(self: & mut Self, node: &'ast Path)`



## verus_state_machines_macros::ident_visitor::error_on_super_path

*Function*

Error if the type contains a `super::...` path.

```rust
fn error_on_super_path(ty: &verus_syn::Type) -> parse::Result<()>
```



## verus_state_machines_macros::ident_visitor::pattern_get_bound_idents

*Function*

Get all identifiers bound by the pattern

```rust
fn pattern_get_bound_idents(pat: &verus_syn::Pat) -> Vec<verus_syn::Ident>
```



## verus_state_machines_macros::ident_visitor::validate_ident

*Function*

Validate a single identifier.

```rust
fn validate_ident(ident: &verus_syn::Ident) -> Result<(), verus_syn::Error>
```



## verus_state_machines_macros::ident_visitor::validate_idents_expr

*Function*

```rust
fn validate_idents_expr(e: &verus_syn::Expr, kind: crate::ast::TransitionKind, bound_names: &std::collections::HashSet<String>, field_names: &std::collections::HashSet<String>) -> parse::Result<()>
```



## verus_state_machines_macros::ident_visitor::validate_idents_op

*Function*

```rust
fn validate_idents_op(op: &crate::ast::SpecialOp, kind: crate::ast::TransitionKind, bound_names: &std::collections::HashSet<String>, field_names: &std::collections::HashSet<String>) -> parse::Result<()>
```



## verus_state_machines_macros::ident_visitor::validate_idents_pat

*Function*

```rust
fn validate_idents_pat(pat: &verus_syn::Pat, kind: crate::ast::TransitionKind) -> parse::Result<()>
```



## verus_state_machines_macros::ident_visitor::validate_idents_transition

*Function*

Error if any identifiers conflict with reserved IDs used by macro expanion.

This validation should be applied to:
 * the entire body of any transition definition
 * any field name

Since macros might introduce arbitrary identifiers or otherwise interfere with
our checks or transformations, we also disallow macros entirely.
(See the more detailed explanation in `field_access_visitor.rs`.)

Also errors if the user incorrectly uses `self` or `pre` (for the sake of a nicer
error message).

We also check if the user uses a field name as a variable so we can warn them
that they need to use `pre.{field name}`. This isn't essential, but it's pretty
helpful since otherwise the error message from type-checking is really awkward.

```rust
fn validate_idents_transition(trans: &crate::ast::Transition, field_names: std::collections::HashSet<String>) -> parse::Result<()>
```



## verus_state_machines_macros::ident_visitor::validate_idents_transition_stmt

*Function*

```rust
fn validate_idents_transition_stmt(ts: &crate::ast::TransitionStmt, kind: crate::ast::TransitionKind, bound_names: &std::collections::HashSet<String>, field_names: &std::collections::HashSet<String>) -> parse::Result<()>
```



