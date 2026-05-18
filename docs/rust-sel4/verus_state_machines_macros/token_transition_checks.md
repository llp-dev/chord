**verus_state_machines_macros > token_transition_checks**

# Module: token_transition_checks

## Contents

**Functions**

- [`check_ordering_remove_have_add`](#check_ordering_remove_have_add) - We require the token updates to go in remove -> have -> add order.
- [`check_ordering_remove_have_add_rec`](#check_ordering_remove_have_add_rec)
- [`check_unsupported_updates`](#check_unsupported_updates) - Check if any SpecialOp is inside a conditional, which is currently unsupported.
- [`check_unsupported_updates_helper`](#check_unsupported_updates_helper)

---

## verus_state_machines_macros::token_transition_checks::check_ordering_remove_have_add

*Function*

We require the token updates to go in remove -> have -> add order.
This isn't strictly necessary; however, doing otherwise might result in a transition
relation which is weaker than would be enforced by the exchange method.

The reason is that 'remove' corresponds to input tokens that are consumed
'have' corresponds to readonly input tokens, and 'add' corresponds to output tokens.
However, this translation does not take into account the order of statements,
while the order of statements IS meaningful to the operational definitions of
have/remove/add for the purposes of constructing a relation.
And in particular, the exchange method corresponds to the remove -> have -> add order.

For example, suppose the user entered the commands in a different order:

have(x);
remove(y);

Operationally, this means that the starting state contains `x`, and then the state `y`
is removed. This means `x` and `y` might overlap. However, the exchange method will
take both 'x' and 'y' as tokens (the first read-only, the second not). But here, the
tokens are necessarily disjoint!

Note that we don't apply a similar restriction for withdraw/deposit/guard. For one thing,
'guard' can only be in a readonly transiton and 'withdraw/deposit' in a normal transition.
So those can't interact at all. For another, there could conceivably be reason
to put 'withdraw' and 'deposit' in either order.

```rust
fn check_ordering_remove_have_add(sm: &crate::ast::SM, ts: &crate::ast::TransitionStmt) -> parse::Result<()>
```



## verus_state_machines_macros::token_transition_checks::check_ordering_remove_have_add_rec

*Function*

```rust
fn check_ordering_remove_have_add_rec(ts: &crate::ast::TransitionStmt, field_name: &String, seen_have: bool, seen_add: bool) -> parse::Result<(bool, bool)>
```



## verus_state_machines_macros::token_transition_checks::check_unsupported_updates

*Function*

Check if any SpecialOp is inside a conditional, which is currently unsupported.
e.g., this is disallowed:

if cond {
   add_element(...);
}

Such a thing would mean we need "conditional arguments" in the exchange methods
(presumably via Option types) a feature that we don't implement.

Also checks to make sure there are no sub-updates.

```rust
fn check_unsupported_updates(ts: &crate::ast::TransitionStmt) -> parse::Result<()>
```



## verus_state_machines_macros::token_transition_checks::check_unsupported_updates_helper

*Function*

```rust
fn check_unsupported_updates_helper(ts: &crate::ast::TransitionStmt) -> parse::Result<()>
```



