**syn > fixup**

# Module: fixup

## Contents

**Structs**

- [`FixupContext`](#fixupcontext)

**Enums**

- [`Scan`](#scan)

**Functions**

- [`scan_left`](#scan_left)
- [`scan_right`](#scan_right)

---

## syn::fixup::FixupContext

*Struct*

**Fields:**
- `previous_operator: crate::precedence::Precedence`
- `next_operator: crate::precedence::Precedence`
- `stmt: bool`
- `leftmost_subexpression_in_stmt: bool`
- `match_arm: bool`
- `leftmost_subexpression_in_match_arm: bool`
- `condition: bool`
- `rightmost_subexpression_in_condition: bool`
- `leftmost_subexpression_in_optional_operand: bool`
- `next_operator_can_begin_expr: bool`
- `next_operator_can_continue_expr: bool`
- `next_operator_can_begin_generics: bool`

**Methods:**

- `fn new_stmt() -> Self` - Create the initial fixup for printing an expression in statement
- `fn new_match_arm() -> Self` - Create the initial fixup for printing an expression as the right-hand
- `fn new_condition() -> Self` - Create the initial fixup for printing an expression as the "condition"
- `fn leftmost_subexpression_with_operator(self: Self, expr: &Expr, next_operator_can_begin_expr: bool, next_operator_can_begin_generics: bool, precedence: Precedence) -> (Precedence, Self)` - Transform this fixup into the one that should apply when printing the
- `fn leftmost_subexpression_with_dot(self: Self, expr: &Expr) -> (Precedence, Self)` - Transform this fixup into the one that should apply when printing a
- `fn leftmost_subexpression_precedence(self: Self, expr: &Expr) -> Precedence`
- `fn rightmost_subexpression(self: Self, expr: &Expr, precedence: Precedence) -> (Precedence, Self)` - Transform this fixup into the one that should apply when printing the
- `fn rightmost_subexpression_fixup(self: Self, reset_allow_struct: bool, optional_operand: bool, precedence: Precedence) -> Self`
- `fn rightmost_subexpression_precedence(self: Self, expr: &Expr) -> Precedence`
- `fn parenthesize(self: Self, expr: &Expr) -> bool` - Determine whether parentheses are needed around the given expression to
- `fn precedence(self: Self, expr: &Expr) -> Precedence` - Determines the effective precedence of a subexpression. Some expressions

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::fixup::Scan

*Enum*

**Variants:**
- `Fail`
- `Bailout`
- `Consume`

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::fixup::scan_left

*Function*

```rust
fn scan_left(expr: &crate::expr::Expr, fixup: FixupContext) -> bool
```



## syn::fixup::scan_right

*Function*

```rust
fn scan_right(expr: &crate::expr::Expr, fixup: FixupContext, precedence: crate::precedence::Precedence, fail_offset: u8, bailout_offset: u8) -> Scan
```



