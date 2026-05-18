**syn > op**

# Module: op

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Enums**

- [`BinOp`](#binop) - A binary operator: `+`, `+=`, `&`.
- [`UnOp`](#unop) - A unary operator: `*`, `!`, `-`.

---

## syn::op::BinOp

*Enum*

A binary operator: `+`, `+=`, `&`.

**Variants:**
- `Add($crate::token::Plus)` - The `+` operator (addition)
- `Sub($crate::token::Minus)` - The `-` operator (subtraction)
- `Mul($crate::token::Star)` - The `*` operator (multiplication)
- `Div($crate::token::Slash)` - The `/` operator (division)
- `Rem($crate::token::Percent)` - The `%` operator (modulus)
- `And($crate::token::AndAnd)` - The `&&` operator (logical and)
- `Or($crate::token::OrOr)` - The `||` operator (logical or)
- `BitXor($crate::token::Caret)` - The `^` operator (bitwise xor)
- `BitAnd($crate::token::And)` - The `&` operator (bitwise and)
- `BitOr($crate::token::Or)` - The `|` operator (bitwise or)
- `Shl($crate::token::Shl)` - The `<<` operator (shift left)
- `Shr($crate::token::Shr)` - The `>>` operator (shift right)
- `Eq($crate::token::EqEq)` - The `==` operator (equality)
- `Lt($crate::token::Lt)` - The `<` operator (less than)
- `Le($crate::token::Le)` - The `<=` operator (less than or equal to)
- `Ne($crate::token::Ne)` - The `!=` operator (not equal to)
- `Ge($crate::token::Ge)` - The `>=` operator (greater than or equal to)
- `Gt($crate::token::Gt)` - The `>` operator (greater than)
- `AddAssign($crate::token::PlusEq)` - The `+=` operator
- `SubAssign($crate::token::MinusEq)` - The `-=` operator
- `MulAssign($crate::token::StarEq)` - The `*=` operator
- `DivAssign($crate::token::SlashEq)` - The `/=` operator
- `RemAssign($crate::token::PercentEq)` - The `%=` operator
- `BitXorAssign($crate::token::CaretEq)` - The `^=` operator
- `BitAndAssign($crate::token::AndEq)` - The `&=` operator
- `BitOrAssign($crate::token::OrEq)` - The `|=` operator
- `ShlAssign($crate::token::ShlEq)` - The `<<=` operator
- `ShrAssign($crate::token::ShrEq)` - The `>>=` operator

**Traits:** Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::op::UnOp

*Enum*

A unary operator: `*`, `!`, `-`.

**Variants:**
- `Deref($crate::token::Star)` - The `*` operator for dereferencing
- `Not($crate::token::Not)` - The `!` operator for logical inversion
- `Neg($crate::token::Minus)` - The `-` operator for negation

**Traits:** Copy

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## Module: parsing



## Module: printing



