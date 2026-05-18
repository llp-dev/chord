**syn > precedence**

# Module: precedence

## Contents

**Enums**

- [`Precedence`](#precedence)

---

## syn::precedence::Precedence

*Enum*

**Variants:**
- `Jump`
- `Assign`
- `Range`
- `Or`
- `And`
- `Let`
- `Compare`
- `BitOr`
- `BitXor`
- `BitAnd`
- `Shift`
- `Sum`
- `Product`
- `Cast`
- `Prefix`
- `Unambiguous`

**Methods:**

- `fn of_binop(op: &BinOp) -> Self`
- `fn of(e: &Expr) -> Self`

**Traits:** Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



