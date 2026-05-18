**verus_state_machines_macros > transitions**

# Module: transitions

## Contents

**Functions**

- [`check_at_most_one_update`](#check_at_most_one_update) - For each field, checks that this field is updated *at most* once.
- [`check_at_most_one_update_rec`](#check_at_most_one_update_rec)
- [`check_exactly_one_init`](#check_exactly_one_init) - For each field, checks that this field is initialized *exactly* once.
- [`check_exactly_one_init_rec`](#check_exactly_one_init_rec)
- [`check_label_param`](#check_label_param)
- [`check_let_shadowing`](#check_let_shadowing) - Check that the identifiers bound in 'let' statements are all distinct,
- [`check_let_shadowing_rec`](#check_let_shadowing_rec)
- [`check_transition`](#check_transition)
- [`check_transitions`](#check_transitions) - Check simple well-formedness properties of the transitions.
- [`check_updates_refer_to_valid_fields`](#check_updates_refer_to_valid_fields) - Check that every update statement actually refers to a valid field.
- [`check_valid_ops`](#check_valid_ops) - Check that every Update and SpecialOp statement is allowed for the given
- [`check_valid_ops_init`](#check_valid_ops_init) - Version of `check_valid_ops` but for 'init' routines.
- [`fields_contain`](#fields_contain)
- [`get_field`](#get_field)
- [`is_allowed_in_special_op`](#is_allowed_in_special_op) - Big matrix for whether a given sharding type is allowed for a given SpecialOp type
- [`is_allowed_in_update_in_normal_transition`](#is_allowed_in_update_in_normal_transition) - Returns `true` if you're allowed to have an 'update' statement for the given
- [`op_matches_type`](#op_matches_type)
- [`stmt_get_bound_idents`](#stmt_get_bound_idents)

---

## verus_state_machines_macros::transitions::check_at_most_one_update

*Function*

For each field, checks that this field is updated *at most* once.
Only checks 'update' statements, not special ops, and it
only does the check for fields for which 'update' statements are supported.
Ignores sub-updates, there is actually reason to have more than one of those.

```rust
fn check_at_most_one_update(sm: &crate::ast::SM, ts: &crate::ast::TransitionStmt, errors: & mut Vec<verus_syn::Error>)
```



## verus_state_machines_macros::transitions::check_at_most_one_update_rec

*Function*

```rust
fn check_at_most_one_update_rec(field: &crate::ast::Field, ts: &crate::ast::TransitionStmt) -> parse::Result<Option<proc_macro2::Span>>
```



## verus_state_machines_macros::transitions::check_exactly_one_init

*Function*

For each field, checks that this field is initialized *exactly* once.
This check applies for *all* fields.

```rust
fn check_exactly_one_init(sm: &crate::ast::SM, ts: &crate::ast::TransitionStmt, errors: & mut Vec<verus_syn::Error>)
```



## verus_state_machines_macros::transitions::check_exactly_one_init_rec

*Function*

```rust
fn check_exactly_one_init_rec(field: &crate::ast::Field, ts: &crate::ast::TransitionStmt) -> parse::Result<Option<proc_macro2::Span>>
```



## verus_state_machines_macros::transitions::check_label_param

*Function*

```rust
fn check_label_param(sm: &crate::ast::SM, tr: &crate::ast::Transition, errors: & mut Vec<verus_syn::Error>)
```



## verus_state_machines_macros::transitions::check_let_shadowing

*Function*

Check that the identifiers bound in 'let' statements are all distinct,
and that they don't overlap with the parameters of a transition.

```rust
fn check_let_shadowing(trans: &crate::ast::Transition, errors: & mut Vec<verus_syn::Error>)
```



## verus_state_machines_macros::transitions::check_let_shadowing_rec

*Function*

```rust
fn check_let_shadowing_rec(ts: &crate::ast::TransitionStmt, ids: & mut Vec<String>, errors: & mut Vec<verus_syn::Error>)
```



## verus_state_machines_macros::transitions::check_transition

*Function*

```rust
fn check_transition(sm: &crate::ast::SM, tr: & mut crate::ast::Transition) -> parse::Result<()>
```



## verus_state_machines_macros::transitions::check_transitions

*Function*

Check simple well-formedness properties of the transitions.

```rust
fn check_transitions(sm: & mut crate::ast::SM) -> parse::Result<()>
```



## verus_state_machines_macros::transitions::check_updates_refer_to_valid_fields

*Function*

Check that every update statement actually refers to a valid field.
We'll assume that all the fields are valid in the later checks.

```rust
fn check_updates_refer_to_valid_fields(fields: &Vec<crate::ast::Field>, ts: &crate::ast::TransitionStmt, errors: & mut Vec<verus_syn::Error>)
```



## verus_state_machines_macros::transitions::check_valid_ops

*Function*

Check that every Update and SpecialOp statement is allowed for the given
sharding strategy of its field.

This check is meant for 'transition' and 'readonly' transitions, so it also checks
that there are no 'init' statements, which are only meaningful in 'init' transitions.

It also checks that no fields are modified if it's a 'readonly' transition,
and conversely for a 'transition' transition, it checks that there are no
guard operations (these operations are allowed ONLY in 'readonly' transitions).

```rust
fn check_valid_ops(fields: &Vec<crate::ast::Field>, ts: &crate::ast::TransitionStmt, is_readonly: bool, is_property: bool, errors: & mut Vec<verus_syn::Error>)
```



## verus_state_machines_macros::transitions::check_valid_ops_init

*Function*

Version of `check_valid_ops` but for 'init' routines.
The only valid ops in an 'init' routine are the 'init' statements.
Updates and special ops are all disallowed.

```rust
fn check_valid_ops_init(fields: &Vec<crate::ast::Field>, ts: &crate::ast::TransitionStmt, errors: & mut Vec<verus_syn::Error>)
```



## verus_state_machines_macros::transitions::fields_contain

*Function*

```rust
fn fields_contain(fields: &Vec<crate::ast::Field>, ident: &verus_syn::Ident) -> bool
```



## verus_state_machines_macros::transitions::get_field

*Function*

```rust
fn get_field<'a>(fields: &'a Vec<crate::ast::Field>, ident: &verus_syn::Ident) -> &'a crate::ast::Field
```



## verus_state_machines_macros::transitions::is_allowed_in_special_op

*Function*

Big matrix for whether a given sharding type is allowed for a given SpecialOp type

```rust
fn is_allowed_in_special_op(span: proc_macro2::Span, stype: &crate::ast::ShardableType, sop: &crate::ast::SpecialOp) -> parse::Result<()>
```



## verus_state_machines_macros::transitions::is_allowed_in_update_in_normal_transition

*Function*

Returns `true` if you're allowed to have an 'update' statement for the given
sharding strategy. Naturally, this is false for constants, which cannot be updated
at all, and also false for option, multiset, etc. which have to be updated with
special ops.

```rust
fn is_allowed_in_update_in_normal_transition(stype: &crate::ast::ShardableType) -> bool
```



## verus_state_machines_macros::transitions::op_matches_type

*Function*

```rust
fn op_matches_type(stype: &crate::ast::ShardableType, elt: &crate::ast::MonoidElt) -> bool
```



## verus_state_machines_macros::transitions::stmt_get_bound_idents

*Function*

```rust
fn stmt_get_bound_idents(ts: &crate::ast::TransitionStmt) -> Vec<verus_syn::Ident>
```



