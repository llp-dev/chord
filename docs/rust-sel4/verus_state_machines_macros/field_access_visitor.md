**verus_state_machines_macros > field_access_visitor**

# Module: field_access_visitor

## Contents

**Structs**

- [`FieldAccessVisitor`](#fieldaccessvisitor)

**Functions**

- [`find_all_accesses`](#find_all_accesses) - Returns two sets, the first consisting of all fields accessed by `pre.foo`
- [`get_field_by_ident`](#get_field_by_ident)
- [`visit_field_accesses`](#visit_field_accesses) - Given a (Rust AST) Expr `e`, visits the subexpressions of the form
- [`visit_field_accesses_all_exprs`](#visit_field_accesses_all_exprs) - Applies the visitor `visit_field_accesses` to every Expr in the TransitionStmt.
- [`visit_special_op`](#visit_special_op)

---

## verus_state_machines_macros::field_access_visitor::FieldAccessVisitor

*Struct*

**Generic Parameters:**
- 'a
- F

**Fields:**
- `errors: &'a  mut Vec<verus_syn::parse::Error>`
- `user_fn: F`
- `ident_to_field: &'a std::collections::HashMap<String, crate::ast::Field>`

**Trait Implementations:**

- **VisitMut**
  - `fn visit_expr_mut(self: & mut Self, node: & mut Expr)`



## verus_state_machines_macros::field_access_visitor::find_all_accesses

*Function*

Returns two sets, the first consisting of all fields accessed by `pre.foo`
in some expression OTHER than a `birds_eye` let-statement,
and the second consiting of those accesses from a `birds_eye let-statement.

(Note: Even though `ts` is `&mut`, the argument isn't actually modified.
The only reason it is marked `&mut` is because we need to call `visit_field_accesses`,
and it doesn't seem worthwhile to implement two different versions for
`&mut` vs `&` parameters. But if we really needed to pass a `&TransitionStmt` here,
it could be done.)

```rust
fn find_all_accesses(ts: & mut crate::ast::TransitionStmt, errors: & mut Vec<verus_syn::parse::Error>, ident_to_field: &std::collections::HashMap<String, crate::ast::Field>) -> (std::collections::HashSet<String>, std::collections::HashSet<String>)
```



## verus_state_machines_macros::field_access_visitor::get_field_by_ident

*Function*

```rust
fn get_field_by_ident<'a>(ident_to_field: &'a std::collections::HashMap<String, crate::ast::Field>, span: proc_macro2::Span, ident: &verus_syn::Ident) -> parse::Result<&'a crate::ast::Field>
```



## verus_state_machines_macros::field_access_visitor::visit_field_accesses

*Function*

Given a (Rust AST) Expr `e`, visits the subexpressions of the form
`pre.foo` where `foo` is a state machine field, and calls the given
function `f` on each one.
Note `f` takes a `&mut Expr` so it is allowed to modify the subexpression,
and it also takes a `&mut Vec<Error>` so it can produce errors.

The visitor itself may also produce errors. Specifically, it will create
an error if it finds a use of `pre` for any reason that is NOT an access
of a state machine field. For example, `pre.associated_method()` is
not allowed, nor is using `pre` without a "dot" access.

```rust
fn visit_field_accesses<impl FnMut(&mut Vec<Error>, &Field, &mut Expr) -> ()>(e: & mut verus_syn::Expr, f: impl Trait, errors: & mut Vec<verus_syn::parse::Error>, ident_to_field: &std::collections::HashMap<String, crate::ast::Field>)
```



## verus_state_machines_macros::field_access_visitor::visit_field_accesses_all_exprs

*Function*

Applies the visitor `visit_field_accesses` to every Expr in the TransitionStmt.
Here, the visitor function `f` takes a fourth argument: a bool that indicates
if the given expression is from the initializer of a `birds_eye` let-statement
(i.e., the bool is false for expressions in any non-birds-eye `let` statement,
or in any other non-`let` statement).

Corner case: we skip over the 'key' fields in GuardKV, DepositKV, and WithdrawKV,
(i.e., for the StorageMap case). The field is actually irrelevant for the codegen
of an exchange method, because a token guarded, deposited, or withdrawn is just
the value exactly.
(This ONLY applies to the StorageMap, not the ordinary Map; i.e., for
RemoveKV, AddKV, and HaveKV, we check the 'key' expression like you'd expect.)

```rust
fn visit_field_accesses_all_exprs<impl FnMut(&mut Vec<Error>, &Field, &mut Expr, bool) -> ()>(ts: & mut crate::ast::TransitionStmt, f: & mut impl Trait, errors: & mut Vec<verus_syn::parse::Error>, ident_to_field: &std::collections::HashMap<String, crate::ast::Field>)
```



## verus_state_machines_macros::field_access_visitor::visit_special_op

*Function*

```rust
fn visit_special_op<impl FnMut(&mut Vec<Error>, &Field, &mut Expr, bool) -> ()>(op: & mut crate::ast::SpecialOp, f: & mut impl Trait, errors: & mut Vec<verus_syn::parse::Error>, ident_to_field: &std::collections::HashMap<String, crate::ast::Field>)
```



