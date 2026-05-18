**verus_state_machines_macros > check_birds_eye**

# Module: check_birds_eye

## Contents

**Functions**

- [`affects_precondition`](#affects_precondition) - True if it's the case that an expression in the op
- [`check_birds_eye`](#check_birds_eye)
- [`check_birds_eye_rec`](#check_birds_eye_rec)
- [`opt_or`](#opt_or)

---

## verus_state_machines_macros::check_birds_eye::affects_precondition

*Function*

True if it's the case that an expression in the op
might appear in the _precondition_ of an exchange method.
Should return 'true' for remove, have, and deposit ops.

```rust
fn affects_precondition(op: &crate::ast::SpecialOp) -> bool
```



## verus_state_machines_macros::check_birds_eye::check_birds_eye

*Function*

```rust
fn check_birds_eye(trans: &crate::ast::Transition, concurrent: bool, errors: & mut Vec<verus_syn::parse::Error>)
```



## verus_state_machines_macros::check_birds_eye::check_birds_eye_rec

*Function*

```rust
fn check_birds_eye_rec(ts: &crate::ast::TransitionStmt, is_init: bool, concurrent: bool, scoped_in_birds_eye: Option<&'static str>, past_assert: & mut Option<&'static str>, errors: & mut Vec<verus_syn::parse::Error>)
```



## verus_state_machines_macros::check_birds_eye::opt_or

*Function*

```rust
fn opt_or<'a>(o: Option<&'a str>, b: bool, s: &'a str) -> Option<&'a str>
```



