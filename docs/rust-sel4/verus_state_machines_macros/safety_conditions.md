**verus_state_machines_macros > safety_conditions**

# Module: safety_conditions

## Contents

**Functions**

- [`has_any_assert_simpl`](#has_any_assert_simpl)
- [`has_any_assert_simpl_vec`](#has_any_assert_simpl_vec) - Returns true if there are any 'assert' statements.
- [`safety_condition_body_simpl`](#safety_condition_body_simpl)
- [`safety_condition_body_simpl_vec`](#safety_condition_body_simpl_vec)

---

## verus_state_machines_macros::safety_conditions::has_any_assert_simpl

*Function*

```rust
fn has_any_assert_simpl(sop: &crate::ast::SimplStmt) -> bool
```



## verus_state_machines_macros::safety_conditions::has_any_assert_simpl_vec

*Function*

Returns true if there are any 'assert' statements.

```rust
fn has_any_assert_simpl_vec(sops: &Vec<crate::ast::SimplStmt>) -> bool
```



## verus_state_machines_macros::safety_conditions::safety_condition_body_simpl

*Function*

```rust
fn safety_condition_body_simpl(sop: &crate::ast::SimplStmt, let_skip_brace: bool) -> Option<verus_syn::Expr>
```



## verus_state_machines_macros::safety_conditions::safety_condition_body_simpl_vec

*Function*

```rust
fn safety_condition_body_simpl_vec(sops: &Vec<crate::ast::SimplStmt>) -> Option<verus_syn::Expr>
```



