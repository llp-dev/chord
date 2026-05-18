**verus_state_machines_macros > simplify_asserts**

# Module: simplify_asserts

## Contents

**Functions**

- [`simplify_asserts`](#simplify_asserts) - Returns an equivalent SimplStmt sequence that has no 'assert' statements in it.
- [`simplify_asserts_stmt`](#simplify_asserts_stmt)
- [`simplify_asserts_vec`](#simplify_asserts_vec)

---

## verus_state_machines_macros::simplify_asserts::simplify_asserts

*Function*

Returns an equivalent SimplStmt sequence that has no 'assert' statements in it.

Essentially, we replace
    assert a;
    require b;
with:
    require a ==> b;

```rust
fn simplify_asserts(sops: &Vec<crate::ast::SimplStmt>) -> Vec<crate::ast::SimplStmt>
```



## verus_state_machines_macros::simplify_asserts::simplify_asserts_stmt

*Function*

```rust
fn simplify_asserts_stmt(sop: &crate::ast::SimplStmt, assert_ident: &verus_syn::Ident) -> crate::ast::SimplStmt
```



## verus_state_machines_macros::simplify_asserts::simplify_asserts_vec

*Function*

```rust
fn simplify_asserts_vec(sops: &Vec<crate::ast::SimplStmt>, assert_ident: &verus_syn::Ident) -> Vec<crate::ast::SimplStmt>
```



