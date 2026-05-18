**verus_state_machines_macros > inherent_safety_conditions**

# Module: inherent_safety_conditions

## Contents

**Enums**

- [`CollectionType`](#collectiontype)

**Functions**

- [`check_inherent_condition_for_special_op`](#check_inherent_condition_for_special_op) - Many of the "special ops" have inherent safety conditions.
- [`check_inherent_conditions`](#check_inherent_conditions)

---

## verus_state_machines_macros::inherent_safety_conditions::CollectionType

*Enum*

**Variants:**
- `Map`
- `PersistentMap`
- `Multiset`
- `Option`
- `PersistentOption`
- `Nat`
- `PersistentNat`
- `Set`
- `PersistentSet`
- `Bool`
- `PersistentBool`

**Methods:**

- `fn name(self: Self) -> &'static str`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CollectionType`



## verus_state_machines_macros::inherent_safety_conditions::check_inherent_condition_for_special_op

*Function*

Many of the "special ops" have inherent safety conditions.
(That is, they contain an 'assert' when expanded out.)
For those that do, the user is responsible for ensuring they hold,
which they can do with a 'proof block' if necessary. For example:

  add m += Some(x) by { /* proof here ... */ }

This function looks at the given statement and returns the error message
that should be used in the case that the condition fails to prove.
(Technically, it returns the identifier of a lemma in
`pervasive::state_machine_internal` that contains the error message.)

However, not all statements have nontrivial inherent safety conditions.
This depends on both the type of statement and the sharding strategy.
In the case that this is such a statement, we either:

 * Return an Error if the user incorrectly adds such a block.
 * Return Ok("") otherwise.

We check if a statement has an inherent safety condition with the following logic:

withdraw, guard: yes, has a safety condition
remove, have: no safety condition; thus no proof needed
add, desposit: yes iff the underlying monoid's composition operator is not total.
  (e.g., composition is total for multiset)

See `docs/command-reference.md` for more explanation, or `simplification.rs`
for the expansions.

```rust
fn check_inherent_condition_for_special_op(span: proc_macro2::Span, op: &crate::ast::SpecialOp, stype: &crate::ast::ShardableType, user_gave_proof_body: bool) -> parse::Result<String>
```



## verus_state_machines_macros::inherent_safety_conditions::check_inherent_conditions

*Function*

```rust
fn check_inherent_conditions(sm: &crate::ast::SM, ts: & mut crate::ast::TransitionStmt, errors: & mut Vec<verus_syn::parse::Error>)
```



