**verus_state_machines_macros > to_relation**

# Module: to_relation

## Contents

**Structs**

- [`UseGetter`](#usegetter)

**Functions**

- [`add_used_ids_from_expr`](#add_used_ids_from_expr)
- [`annotate_extractions`](#annotate_extractions) - Mark each scope-creating node with the assign-vars that need to be extracted
- [`annotate_extractions_stmt`](#annotate_extractions_stmt)
- [`annotate_extractions_vec`](#annotate_extractions_vec)
- [`conjunct_opt`](#conjunct_opt)
- [`emit_match`](#emit_match)
- [`extr_vec`](#extr_vec)
- [`get_all_assigns`](#get_all_assigns)
- [`get_extraction_decl_stmt`](#get_extraction_decl_stmt) - Extract assignments that were done within scopes
- [`get_extup`](#get_extup)
- [`prepend_conjunct`](#prepend_conjunct)
- [`prepend_let_stmt`](#prepend_let_stmt)
- [`remove_post_conditions_vec`](#remove_post_conditions_vec)
- [`set_union`](#set_union)
- [`simpl_conjunct_stmt`](#simpl_conjunct_stmt)
- [`simpl_conjunct_vec`](#simpl_conjunct_vec) - Collect all conjuncts
- [`to_is_enabled_condition_weak`](#to_is_enabled_condition_weak)
- [`to_relation`](#to_relation) - Converts a transition description into a relation between `pre` and `post`.

---

## verus_state_machines_macros::to_relation::UseGetter

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `used_ids: &'a  mut indexmap::IndexSet<String>`

**Methods:**

- `fn visit_stream(self: & mut Self, stream: TokenStream)`

**Trait Implementations:**

- **Visit**
  - `fn visit_ident(self: & mut Self, node: &'ast Ident)`
  - `fn visit_expr(self: & mut Self, node: &'ast Expr)`



## verus_state_machines_macros::to_relation::add_used_ids_from_expr

*Function*

```rust
fn add_used_ids_from_expr(used_ids: & mut indexmap::IndexSet<String>, e: &verus_syn::Expr)
```



## verus_state_machines_macros::to_relation::annotate_extractions

*Function*

Mark each scope-creating node with the assign-vars that need to be extracted
Also, remove any 'assign' that isn't used.

```rust
fn annotate_extractions(sops: &Vec<crate::ast::SimplStmt>) -> Vec<crate::ast::SimplStmt>
```



## verus_state_machines_macros::to_relation::annotate_extractions_stmt

*Function*

```rust
fn annotate_extractions_stmt(sop: & mut crate::ast::SimplStmt, used_ids: & mut indexmap::IndexSet<String>)
```



## verus_state_machines_macros::to_relation::annotate_extractions_vec

*Function*

```rust
fn annotate_extractions_vec(sops: & mut Vec<crate::ast::SimplStmt>, used_ids: & mut indexmap::IndexSet<String>)
```



## verus_state_machines_macros::to_relation::conjunct_opt

*Function*

```rust
fn conjunct_opt(a: Option<proc_macro2::TokenStream>, b: Option<proc_macro2::TokenStream>) -> Option<proc_macro2::TokenStream>
```



## verus_state_machines_macros::to_relation::emit_match

*Function*

```rust
fn emit_match<T>(span: proc_macro2::Span, match_e: &verus_syn::Expr, arms: &Vec<crate::ast::Arm>, exprs: &Vec<T>) -> verus_syn::Expr
```



## verus_state_machines_macros::to_relation::extr_vec

*Function*

```rust
fn extr_vec(sops: &Vec<crate::ast::SimplStmt>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_relation::get_all_assigns

*Function*

```rust
fn get_all_assigns(sop: &crate::ast::SimplStmt, assigned: & mut indexmap::IndexSet<String>)
```



## verus_state_machines_macros::to_relation::get_extraction_decl_stmt

*Function*

Extract assignments that were done within scopes

```rust
fn get_extraction_decl_stmt(sop: &crate::ast::SimplStmt) -> Option<proc_macro2::TokenStream>
```



## verus_state_machines_macros::to_relation::get_extup

*Function*

```rust
fn get_extup(a: &Vec<proc_macro2::Ident>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_relation::prepend_conjunct

*Function*

```rust
fn prepend_conjunct(e: &verus_syn::Expr, p: Option<proc_macro2::TokenStream>, msg: &str) -> Option<proc_macro2::TokenStream>
```



## verus_state_machines_macros::to_relation::prepend_let_stmt

*Function*

```rust
fn prepend_let_stmt(span: proc_macro2::Span, l: Option<proc_macro2::TokenStream>, p: Option<proc_macro2::TokenStream>) -> Option<proc_macro2::TokenStream>
```



## verus_state_machines_macros::to_relation::remove_post_conditions_vec

*Function*

```rust
fn remove_post_conditions_vec(sops: &Vec<crate::ast::SimplStmt>) -> Vec<crate::ast::SimplStmt>
```



## verus_state_machines_macros::to_relation::set_union

*Function*

```rust
fn set_union(s: & mut indexmap::IndexSet<String>, t: indexmap::IndexSet<String>)
```



## verus_state_machines_macros::to_relation::simpl_conjunct_stmt

*Function*

```rust
fn simpl_conjunct_stmt(sop: &crate::ast::SimplStmt, p: Option<proc_macro2::TokenStream>, let_skip_brace: bool, assign_skip_brace: bool) -> Option<proc_macro2::TokenStream>
```



## verus_state_machines_macros::to_relation::simpl_conjunct_vec

*Function*

Collect all conjuncts

```rust
fn simpl_conjunct_vec(sops: &Vec<crate::ast::SimplStmt>, p: Option<proc_macro2::TokenStream>) -> Option<proc_macro2::TokenStream>
```



## verus_state_machines_macros::to_relation::to_is_enabled_condition_weak

*Function*

```rust
fn to_is_enabled_condition_weak(sops: &Vec<crate::ast::SimplStmt>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_relation::to_relation

*Function*

Converts a transition description into a relation between `pre` and `post`.
Overall, this process has two steps:

1. Process all 'update' statements and special ops, turning them into
   require, assert, and postcondition operations. (See `simplification.rs`.)
2. Walk the tree and straightforwardly convert it to a relation.

This function performs step (2) (and it assumes that step (1) has already been applied.
See `simplification.rs`.)

There are actually two different relations we can form, the "weak" relation and
the "strong" one.

These differ only in how they handle "assert" statements. (Thus if there are no assert
statements, then the two versions are the same.) In short, the 'strong' version treats
an 'assert' like it does any other enabling condition, while the 'weak' version puts
the 'asserts' on the _left_ side of the implication.

For example, consider a transition like,

  require(A);
  assert(B);
  require(C);

Then the weak relation would become

  A && (B ==> C)

While the strong relation would become simply,

  A && B && C

Note that we require the user to prove that any asserts follow from the invariant.
(In this case, that means showing that (Inv && A ==> B).
Thus, subject to the invariant, the weak & strong versions will actually be equivalent.

```rust
fn to_relation(sops: &Vec<crate::ast::SimplStmt>, weak: bool) -> proc_macro2::TokenStream
```



