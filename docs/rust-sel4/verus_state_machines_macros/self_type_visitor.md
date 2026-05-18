**verus_state_machines_macros > self_type_visitor**

# Module: self_type_visitor

## Contents

**Structs**

- [`SelfVisitor`](#selfvisitor)

**Functions**

- [`replace_self_expr`](#replace_self_expr)
- [`replace_self_op`](#replace_self_op)
- [`replace_self_pat`](#replace_self_pat)
- [`replace_self_shardable_type`](#replace_self_shardable_type)
- [`replace_self_sm`](#replace_self_sm) - If the user ever uses 'Self' in a transition, then change it out for the explicit
- [`replace_self_ts`](#replace_self_ts)
- [`replace_self_type`](#replace_self_type)

---

## verus_state_machines_macros::self_type_visitor::SelfVisitor

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `subst_path: &'a verus_syn::Path`

**Trait Implementations:**

- **VisitMut**
  - `fn visit_path_mut(self: & mut Self, path: & mut Path)`



## verus_state_machines_macros::self_type_visitor::replace_self_expr

*Function*

```rust
fn replace_self_expr(e: & mut verus_syn::Expr, subst_path: &verus_syn::Path)
```



## verus_state_machines_macros::self_type_visitor::replace_self_op

*Function*

```rust
fn replace_self_op(op: & mut crate::ast::SpecialOp, path: &verus_syn::Path)
```



## verus_state_machines_macros::self_type_visitor::replace_self_pat

*Function*

```rust
fn replace_self_pat(p: & mut verus_syn::Pat, subst_path: &verus_syn::Path)
```



## verus_state_machines_macros::self_type_visitor::replace_self_shardable_type

*Function*

```rust
fn replace_self_shardable_type(stype: & mut crate::ast::ShardableType, path: &verus_syn::Path)
```



## verus_state_machines_macros::self_type_visitor::replace_self_sm

*Function*

If the user ever uses 'Self' in a transition, then change it out for the explicit
self type so that it's safe to use these expressions and types in other places
outside the generated `State` impl.

```rust
fn replace_self_sm(sm: & mut crate::ast::SM)
```



## verus_state_machines_macros::self_type_visitor::replace_self_ts

*Function*

```rust
fn replace_self_ts(ts: & mut crate::ast::TransitionStmt, path: &verus_syn::Path)
```



## verus_state_machines_macros::self_type_visitor::replace_self_type

*Function*

```rust
fn replace_self_type(ty: & mut verus_syn::Type, subst_path: &verus_syn::Path)
```



