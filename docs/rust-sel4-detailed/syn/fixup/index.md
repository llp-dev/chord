*[syn](../index.md) / [fixup](index.md)*

---

# Module `fixup`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FixupContext`](#fixupcontext) | struct |  |
| [`Scan`](#scan) | enum |  |
| [`scan_left`](#scan-left) | fn |  |
| [`scan_right`](#scan-right) | fn |  |

## Structs

### `FixupContext`

```rust
struct FixupContext {
    previous_operator: crate::precedence::Precedence,
    next_operator: crate::precedence::Precedence,
    stmt: bool,
    leftmost_subexpression_in_stmt: bool,
    match_arm: bool,
    leftmost_subexpression_in_match_arm: bool,
    condition: bool,
    rightmost_subexpression_in_condition: bool,
    leftmost_subexpression_in_optional_operand: bool,
    next_operator_can_begin_expr: bool,
    next_operator_can_continue_expr: bool,
    next_operator_can_begin_generics: bool,
}
```

#### Implementations

- <span id="fixupcontext-const-none"></span>`const NONE: Self`

- <span id="fixupcontext-new-stmt"></span>`fn new_stmt() -> Self`

  Create the initial fixup for printing an expression in statement

  position.

- <span id="fixupcontext-new-match-arm"></span>`fn new_match_arm() -> Self`

  Create the initial fixup for printing an expression as the right-hand

  side of a match arm.

- <span id="fixupcontext-new-condition"></span>`fn new_condition() -> Self`

  Create the initial fixup for printing an expression as the "condition"

  of an `if` or `while`. There are a few other positions which are

  grammatically equivalent and also use this, such as the iterator

  expression in `for` and the scrutinee in `match`.

- <span id="fixupcontext-leftmost-subexpression-with-operator"></span>`fn leftmost_subexpression_with_operator(self, expr: &Expr, next_operator_can_begin_expr: bool, next_operator_can_begin_generics: bool, precedence: Precedence) -> (Precedence, Self)` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

  Transform this fixup into the one that should apply when printing the

  leftmost subexpression of the current expression.

  

  The leftmost subexpression is any subexpression that has the same first

  token as the current expression, but has a different last token.

  

  For example in `$a + $b` and `$a.method()`, the subexpression `$a` is a

  leftmost subexpression.

  

  Not every expression has a leftmost subexpression. For example neither

  `-$a` nor `[$a]` have one.

- <span id="fixupcontext-leftmost-subexpression-with-dot"></span>`fn leftmost_subexpression_with_dot(self, expr: &Expr) -> (Precedence, Self)` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

  Transform this fixup into the one that should apply when printing a

  leftmost subexpression followed by a `.` or `?` token, which confer

  different statement boundary rules compared to other leftmost

  subexpressions.

- <span id="fixupcontext-leftmost-subexpression-precedence"></span>`fn leftmost_subexpression_precedence(self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-rightmost-subexpression"></span>`fn rightmost_subexpression(self, expr: &Expr, precedence: Precedence) -> (Precedence, Self)` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

  Transform this fixup into the one that should apply when printing the

  rightmost subexpression of the current expression.

  

  The rightmost subexpression is any subexpression that has a different

  first token than the current expression, but has the same last token.

  

  For example in `$a + $b` and `-$b`, the subexpression `$b` is a

  rightmost subexpression.

  

  Not every expression has a rightmost subexpression. For example neither

  `[$b]` nor `$a.f($b)` have one.

- <span id="fixupcontext-rightmost-subexpression-fixup"></span>`fn rightmost_subexpression_fixup(self, reset_allow_struct: bool, optional_operand: bool, precedence: Precedence) -> Self` — [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-rightmost-subexpression-precedence"></span>`fn rightmost_subexpression_precedence(self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-parenthesize"></span>`fn parenthesize(self, expr: &Expr) -> bool` — [`Expr`](../expr/index.md#expr)

  Determine whether parentheses are needed around the given expression to

  head off the early termination of a statement or condition.

- <span id="fixupcontext-precedence"></span>`fn precedence(self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

  Determines the effective precedence of a subexpression. Some expressions

  have higher or lower precedence when adjacent to particular operators.

#### Trait Implementations

##### `impl Clone for FixupContext`

- <span id="fixupcontext-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for FixupContext`

## Enums

### `Scan`

```rust
enum Scan {
    Fail,
    Bailout,
    Consume,
}
```

#### Trait Implementations

##### `impl Clone for Scan`

- <span id="scan-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Scan`

##### `impl PartialEq for Scan`

- <span id="scan-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

## Functions

### `scan_left`

```rust
fn scan_left(expr: &crate::expr::Expr, fixup: FixupContext) -> bool
```

### `scan_right`

```rust
fn scan_right(expr: &crate::expr::Expr, fixup: FixupContext, precedence: crate::precedence::Precedence, fail_offset: u8, bailout_offset: u8) -> Scan
```

