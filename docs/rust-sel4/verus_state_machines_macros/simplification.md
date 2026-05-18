**verus_state_machines_macros > simplification**

# Module: simplification

## Contents

**Functions**

- [`add_postconditions`](#add_postconditions)
- [`add_postconditions_vec`](#add_postconditions_vec)
- [`contains_ident`](#contains_ident)
- [`expr_add`](#expr_add) - Add the elements, assuming they are addable. Build the expression assuming
- [`expr_can_add`](#expr_can_add)
- [`expr_ge`](#expr_ge)
- [`expr_matches`](#expr_matches)
- [`expr_remove`](#expr_remove)
- [`field_name_from_tmp`](#field_name_from_tmp)
- [`get_cur`](#get_cur)
- [`get_cur_ident`](#get_cur_ident)
- [`get_initializer_expr`](#get_initializer_expr)
- [`get_opt_type`](#get_opt_type)
- [`postcondition_stmt`](#postcondition_stmt)
- [`simplify_ops`](#simplify_ops) - Simplify out `update` statements, including `add_element` etc.
- [`simplify_ops_rec`](#simplify_ops_rec)
- [`simplify_ops_with_pre`](#simplify_ops_with_pre)
- [`simplify_special_op`](#simplify_special_op)
- [`update_sub_expr`](#update_sub_expr)

**Constants**

- [`UPDATE_TMP_PREFIX`](#update_tmp_prefix)

---

## verus_state_machines_macros::simplification::UPDATE_TMP_PREFIX

*Constant*: `&str`



## verus_state_machines_macros::simplification::add_postconditions

*Function*

```rust
fn add_postconditions(sm: &crate::ast::SM, sops: Vec<crate::ast::SimplStmt>) -> Vec<crate::ast::SimplStmt>
```



## verus_state_machines_macros::simplification::add_postconditions_vec

*Function*

```rust
fn add_postconditions_vec(sops: & mut Vec<crate::ast::SimplStmt>, found: & mut Vec<verus_syn::Ident>)
```



## verus_state_machines_macros::simplification::contains_ident

*Function*

```rust
fn contains_ident(v: &Vec<verus_syn::Ident>, id: &verus_syn::Ident) -> bool
```



## verus_state_machines_macros::simplification::expr_add

*Function*

Add the elements, assuming they are addable. Build the expression assuming
to "prefer" the content from the right-hand side, even though it shouldn't really matter.
That's often easier, anyway.

```rust
fn expr_add(stype: &crate::ast::ShardableType, cur: &verus_syn::Expr, elt: &crate::ast::MonoidElt) -> verus_syn::Expr
```



## verus_state_machines_macros::simplification::expr_can_add

*Function*

```rust
fn expr_can_add(stype: &crate::ast::ShardableType, cur: &verus_syn::Expr, elt: &crate::ast::MonoidElt) -> Option<verus_syn::Expr>
```



## verus_state_machines_macros::simplification::expr_ge

*Function*

```rust
fn expr_ge(stype: &crate::ast::ShardableType, cur: &verus_syn::Expr, elt: &crate::ast::MonoidElt, pat_opt: &Option<Box<verus_syn::Pat>>) -> verus_syn::Expr
```



## verus_state_machines_macros::simplification::expr_matches

*Function*

```rust
fn expr_matches(e: &verus_syn::Expr, pat: &verus_syn::Pat) -> verus_syn::Expr
```



## verus_state_machines_macros::simplification::expr_remove

*Function*

```rust
fn expr_remove(stype: &crate::ast::ShardableType, cur: &verus_syn::Expr, elt: &crate::ast::MonoidElt) -> verus_syn::Expr
```



## verus_state_machines_macros::simplification::field_name_from_tmp

*Function*

```rust
fn field_name_from_tmp(tmp_name: &verus_syn::Ident) -> Option<verus_syn::Ident>
```



## verus_state_machines_macros::simplification::get_cur

*Function*

```rust
fn get_cur(field_name: &verus_syn::Ident) -> verus_syn::Expr
```



## verus_state_machines_macros::simplification::get_cur_ident

*Function*

```rust
fn get_cur_ident(field_name: &verus_syn::Ident) -> verus_syn::Ident
```



## verus_state_machines_macros::simplification::get_initializer_expr

*Function*

```rust
fn get_initializer_expr(f: &verus_syn::Ident, op: &crate::ast::SpecialOp) -> verus_syn::Expr
```



## verus_state_machines_macros::simplification::get_opt_type

*Function*

```rust
fn get_opt_type(stype: &crate::ast::ShardableType) -> verus_syn::Type
```



## verus_state_machines_macros::simplification::postcondition_stmt

*Function*

```rust
fn postcondition_stmt(span: proc_macro2::Span, f: verus_syn::Ident, pcrf: crate::ast::PostConditionReasonField) -> crate::ast::SimplStmt
```



## verus_state_machines_macros::simplification::simplify_ops

*Function*

Simplify out `update` statements, including `add_element` etc.

Note: for 'readonly' stuff, there's less to do because we don't need to handle
updates. However, we still need to handle 'guard' and 'have' statements, which will
be translated into 'asserts'.

```rust
fn simplify_ops(sm: &crate::ast::SM, ts: &crate::ast::TransitionStmt, kind: crate::ast::TransitionKind) -> Vec<crate::ast::SimplStmt>
```



## verus_state_machines_macros::simplification::simplify_ops_rec

*Function*

```rust
fn simplify_ops_rec(ts: &crate::ast::TransitionStmt, sm: &crate::ast::SM) -> Vec<crate::ast::SimplStmt>
```



## verus_state_machines_macros::simplification::simplify_ops_with_pre

*Function*

```rust
fn simplify_ops_with_pre(ts: &crate::ast::TransitionStmt, sm: &crate::ast::SM, kind: crate::ast::TransitionKind) -> Vec<crate::ast::SimplStmt>
```



## verus_state_machines_macros::simplification::simplify_special_op

*Function*

```rust
fn simplify_special_op(span: proc_macro2::Span, field: &crate::ast::Field, op: &crate::ast::SpecialOp, pat_opt: &Option<Box<verus_syn::Pat>>, proof: &crate::ast::AssertProof) -> (Vec<crate::ast::SimplStmt>, Vec<crate::ast::SimplStmt>)
```



## verus_state_machines_macros::simplification::update_sub_expr

*Function*

```rust
fn update_sub_expr(root: &verus_syn::Expr, subs: &Vec<crate::ast::SubIdx>, i: usize, val: &verus_syn::Expr) -> verus_syn::Expr
```



