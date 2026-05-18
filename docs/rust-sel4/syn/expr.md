**syn > expr**

# Module: expr

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`Arm`](#arm) - One arm of a `match` expression: `0..=10 => { return true; }`.
- [`ExprArray`](#exprarray) - A slice literal expression: `[a, b, c, d]`.
- [`ExprAssign`](#exprassign) - An assignment expression: `a = compute()`.
- [`ExprAsync`](#exprasync) - An async block: `async { ... }`.
- [`ExprAwait`](#exprawait) - An await expression: `fut.await`.
- [`ExprBinary`](#exprbinary) - A binary operation: `a + b`, `a += b`.
- [`ExprBlock`](#exprblock) - A blocked scope: `{ ... }`.
- [`ExprBreak`](#exprbreak) - A `break`, with an optional label to break and an optional
- [`ExprCall`](#exprcall) - A function call expression: `invoke(a, b)`.
- [`ExprCast`](#exprcast) - A cast expression: `foo as f64`.
- [`ExprClosure`](#exprclosure) - A closure expression: `|a, b| a + b`.
- [`ExprConst`](#exprconst) - A const block: `const { ... }`.
- [`ExprContinue`](#exprcontinue) - A `continue`, with an optional label.
- [`ExprField`](#exprfield) - Access of a named struct field (`obj.k`) or unnamed tuple struct
- [`ExprForLoop`](#exprforloop) - A for loop: `for pat in expr { ... }`.
- [`ExprGroup`](#exprgroup) - An expression contained within invisible delimiters.
- [`ExprIf`](#exprif) - An `if` expression with an optional `else` block: `if expr { ... }
- [`ExprIndex`](#exprindex) - A square bracketed indexing expression: `vector[2]`.
- [`ExprInfer`](#exprinfer) - The inferred value of a const generic argument, denoted `_`.
- [`ExprLet`](#exprlet) - A `let` guard: `let Some(x) = opt`.
- [`ExprLit`](#exprlit) - A literal in place of an expression: `1`, `"foo"`.
- [`ExprLoop`](#exprloop) - Conditionless loop: `loop { ... }`.
- [`ExprMacro`](#exprmacro) - A macro invocation expression: `format!("{}", q)`.
- [`ExprMatch`](#exprmatch) - A `match` expression: `match n { Some(n) => {}, None => {} }`.
- [`ExprMethodCall`](#exprmethodcall) - A method call expression: `x.foo::<T>(a, b)`.
- [`ExprParen`](#exprparen) - A parenthesized expression: `(a + b)`.
- [`ExprPath`](#exprpath) - A path like `core::mem::replace` possibly containing generic
- [`ExprRange`](#exprrange) - A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.
- [`ExprRawAddr`](#exprrawaddr) - Address-of operation: `&raw const place` or `&raw mut place`.
- [`ExprReference`](#exprreference) - A referencing operation: `&a` or `&mut a`.
- [`ExprRepeat`](#exprrepeat) - An array literal constructed from one repeated element: `[0u8; N]`.
- [`ExprReturn`](#exprreturn) - A `return`, with an optional value to be returned.
- [`ExprStruct`](#exprstruct) - A struct literal expression: `Point { x: 1, y: 1 }`.
- [`ExprTry`](#exprtry) - A try-expression: `expr?`.
- [`ExprTryBlock`](#exprtryblock) - A try block: `try { ... }`.
- [`ExprTuple`](#exprtuple) - A tuple expression: `(a, b, c, d)`.
- [`ExprUnary`](#exprunary) - A unary operation: `!x`, `*x`.
- [`ExprUnsafe`](#exprunsafe) - An unsafe block: `unsafe { ... }`.
- [`ExprWhile`](#exprwhile) - A while loop: `while expr { ... }`.
- [`ExprYield`](#expryield) - A yield expression: `yield expr`.
- [`FieldValue`](#fieldvalue) - A field-value pair in a struct literal.
- [`Index`](#index) - The index of an unnamed tuple struct field.
- [`Label`](#label) - A lifetime labeling a `for`, `while`, or `loop`.

**Enums**

- [`Expr`](#expr) - A Rust expression.
- [`Member`](#member) - A struct or tuple struct field accessed in a struct literal or field
- [`PointerMutability`](#pointermutability) - Mutability of a raw pointer (`*const T`, `*mut T`), in which non-mutable
- [`RangeLimits`](#rangelimits) - Limit types of a range, inclusive or exclusive.

---

## syn::expr::Arm

*Struct*

One arm of a `match` expression: `0..=10 => { return true; }`.

As in:

```
# fn f() -> bool {
#     let n = 0;
match n {
    0..=10 => {
        return true;
    }
    // ...
    # _ => {}
}
#   false
# }
```

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `pat: crate::pat::Pat`
- `guard: Option<($crate::token::If, alloc::boxed::Box<Expr>)>`
- `fat_arrow_token: $crate::token::FatArrow`
- `body: alloc::boxed::Box<Expr>`
- `comma: Option<$crate::token::Comma>`

**Methods:**

- `fn parse_multiple(input: ParseStream) -> Result<Vec<Self>>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Arm>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::Expr

*Enum*

A Rust expression.

*This type is available only if Syn is built with the `"derive"` or `"full"`
feature, but most of the variants are not available unless "full" is enabled.*

# Syntax tree enums

This type is a syntax tree enum. In Syn this and other syntax tree enums
are designed to be traversed using the following rebinding idiom.

```
# use syn::Expr;
#
# fn example(expr: Expr) {
# const IGNORE: &str = stringify! {
let expr: Expr = /* ... */;
# };
match expr {
    Expr::MethodCall(expr) => {
        /* ... */
    }
    Expr::Cast(expr) => {
        /* ... */
    }
    Expr::If(expr) => {
        /* ... */
    }

    /* ... */
    # _ => {}
# }
# }
```

We begin with a variable `expr` of type `Expr` that has no fields
(because it is an enum), and by matching on it and rebinding a variable
with the same name `expr` we effectively imbue our variable with all of
the data fields provided by the variant that it turned out to be. So for
example above if we ended up in the `MethodCall` case then we get to use
`expr.receiver`, `expr.args` etc; if we ended up in the `If` case we get
to use `expr.cond`, `expr.then_branch`, `expr.else_branch`.

This approach avoids repeating the variant names twice on every line.

```
# use syn::{Expr, ExprMethodCall};
#
# fn example(expr: Expr) {
// Repetitive; recommend not doing this.
match expr {
    Expr::MethodCall(ExprMethodCall { method, args, .. }) => {
# }
# _ => {}
# }
# }
```

In general, the name to which a syntax tree enum variant is bound should
be a suitable name for the complete syntax tree enum type.

```
# use syn::{Expr, ExprField};
#
# fn example(discriminant: ExprField) {
// Binding is called `base` which is the name I would use if I were
// assigning `*discriminant.base` without an `if let`.
if let Expr::Tuple(base) = *discriminant.base {
# }
# }
```

A sign that you may not be choosing the right variable names is if you
see names getting repeated in your code, like accessing
`receiver.receiver` or `pat.pat` or `cond.cond`.

**Variants:**
- `Array(ExprArray)` - A slice literal expression: `[a, b, c, d]`.
- `Assign(ExprAssign)` - An assignment expression: `a = compute()`.
- `Async(ExprAsync)` - An async block: `async { ... }`.
- `Await(ExprAwait)` - An await expression: `fut.await`.
- `Binary(ExprBinary)` - A binary operation: `a + b`, `a += b`.
- `Block(ExprBlock)` - A blocked scope: `{ ... }`.
- `Break(ExprBreak)` - A `break`, with an optional label to break and an optional
- `Call(ExprCall)` - A function call expression: `invoke(a, b)`.
- `Cast(ExprCast)` - A cast expression: `foo as f64`.
- `Closure(ExprClosure)` - A closure expression: `|a, b| a + b`.
- `Const(ExprConst)` - A const block: `const { ... }`.
- `Continue(ExprContinue)` - A `continue`, with an optional label.
- `Field(ExprField)` - Access of a named struct field (`obj.k`) or unnamed tuple struct
- `ForLoop(ExprForLoop)` - A for loop: `for pat in expr { ... }`.
- `Group(ExprGroup)` - An expression contained within invisible delimiters.
- `If(ExprIf)` - An `if` expression with an optional `else` block: `if expr { ... }
- `Index(ExprIndex)` - A square bracketed indexing expression: `vector[2]`.
- `Infer(ExprInfer)` - The inferred value of a const generic argument, denoted `_`.
- `Let(ExprLet)` - A `let` guard: `let Some(x) = opt`.
- `Lit(ExprLit)` - A literal in place of an expression: `1`, `"foo"`.
- `Loop(ExprLoop)` - Conditionless loop: `loop { ... }`.
- `Macro(ExprMacro)` - A macro invocation expression: `format!("{}", q)`.
- `Match(ExprMatch)` - A `match` expression: `match n { Some(n) => {}, None => {} }`.
- `MethodCall(ExprMethodCall)` - A method call expression: `x.foo::<T>(a, b)`.
- `Paren(ExprParen)` - A parenthesized expression: `(a + b)`.
- `Path(ExprPath)` - A path like `core::mem::replace` possibly containing generic
- `Range(ExprRange)` - A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.
- `RawAddr(ExprRawAddr)` - Address-of operation: `&raw const place` or `&raw mut place`.
- `Reference(ExprReference)` - A referencing operation: `&a` or `&mut a`.
- `Repeat(ExprRepeat)` - An array literal constructed from one repeated element: `[0u8; N]`.
- `Return(ExprReturn)` - A `return`, with an optional value to be returned.
- `Struct(ExprStruct)` - A struct literal expression: `Point { x: 1, y: 1 }`.
- `Try(ExprTry)` - A try-expression: `expr?`.
- `TryBlock(ExprTryBlock)` - A try block: `try { ... }`.
- `Tuple(ExprTuple)` - A tuple expression: `(a, b, c, d)`.
- `Unary(ExprUnary)` - A unary operation: `!x`, `*x`.
- `Unsafe(ExprUnsafe)` - An unsafe block: `unsafe { ... }`.
- `Verbatim(proc_macro2::TokenStream)` - Tokens in expression position not interpreted by Syn.
- `While(ExprWhile)` - A while loop: `while expr { ... }`.
- `Yield(ExprYield)` - A yield expression: `yield expr`.

**Methods:**

- `fn parse_without_eager_brace(input: ParseStream) -> Result<Expr>` - An alternative to the primary `Expr::parse` parser (from the [`Parse`]
- `fn parse_with_earlier_boundary_rule(input: ParseStream) -> Result<Expr>` - An alternative to the primary `Expr::parse` parser (from the [`Parse`]
- `fn peek(input: ParseStream) -> bool` - Returns whether the next token in the parse stream is one that might
- `fn replace_attrs(self: & mut Self, new: Vec<Attribute>) -> Vec<Attribute>`

**Trait Implementations:**

- **From**
  - `fn from(e: ExprUnary) -> Expr`
- **From**
  - `fn from(e: ExprArray) -> Expr`
- **From**
  - `fn from(e: ExprLit) -> Expr`
- **From**
  - `fn from(e: ExprYield) -> Expr`
- **From**
  - `fn from(e: ExprAwait) -> Expr`
- **From**
  - `fn from(e: ExprMatch) -> Expr`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: ExprBreak) -> Expr`
- **From**
  - `fn from(e: ExprPath) -> Expr`
- **From**
  - `fn from(e: ExprClosure) -> Expr`
- **From**
  - `fn from(e: ExprReference) -> Expr`
- **From**
  - `fn from(e: ExprField) -> Expr`
- **From**
  - `fn from(e: ExprStruct) -> Expr`
- **From**
  - `fn from(e: ExprIf) -> Expr`
- **From**
  - `fn from(e: ExprTuple) -> Expr`
- **From**
  - `fn from(e: ExprLet) -> Expr`
- **From**
  - `fn from(e: ExprWhile) -> Expr`
- **From**
  - `fn from(e: ExprAsync) -> Expr`
- **From**
  - `fn from(e: ExprMacro) -> Expr`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(e: ExprBlock) -> Expr`
- **From**
  - `fn from(e: ExprParen) -> Expr`
- **From**
  - `fn from(e: ExprCast) -> Expr`
- **From**
  - `fn from(e: ExprRawAddr) -> Expr`
- **From**
  - `fn from(e: ExprContinue) -> Expr`
- **From**
  - `fn from(e: ExprReturn) -> Expr`
- **From**
  - `fn from(e: ExprGroup) -> Expr`
- **From**
  - `fn from(e: ExprTryBlock) -> Expr`
- **From**
  - `fn from(e: ExprInfer) -> Expr`
- **From**
  - `fn from(e: ExprUnsafe) -> Expr`
- **From**
  - `fn from(e: ExprAssign) -> Expr`
- **From**
  - `fn from(e: ExprLoop) -> Expr`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`
- **From**
  - `fn from(e: ExprBinary) -> Expr`
- **From**
  - `fn from(e: ExprMethodCall) -> Expr`
- **From**
  - `fn from(e: ExprCall) -> Expr`
- **From**
  - `fn from(e: ExprRange) -> Expr`
- **From**
  - `fn from(e: ExprConst) -> Expr`
- **From**
  - `fn from(e: ExprRepeat) -> Expr`
- **From**
  - `fn from(e: ExprForLoop) -> Expr`
- **From**
  - `fn from(e: ExprTry) -> Expr`
- **From**
  - `fn from(e: ExprIndex) -> Expr`



## syn::expr::ExprArray

*Struct*

A slice literal expression: `[a, b, c, d]`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `bracket_token: token::Bracket`
- `elems: crate::punctuated::Punctuated<Expr, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprAssign

*Struct*

An assignment expression: `a = compute()`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `left: alloc::boxed::Box<Expr>`
- `eq_token: $crate::token::Eq`
- `right: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprAsync

*Struct*

An async block: `async { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `async_token: $crate::token::Async`
- `capture: Option<$crate::token::Move>`
- `block: crate::stmt::Block`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprAwait

*Struct*

An await expression: `fut.await`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `base: alloc::boxed::Box<Expr>`
- `dot_token: $crate::token::Dot`
- `await_token: $crate::token::Await`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprBinary

*Struct*

A binary operation: `a + b`, `a += b`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `left: alloc::boxed::Box<Expr>`
- `op: crate::op::BinOp`
- `right: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprBlock

*Struct*

A blocked scope: `{ ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `label: Option<Label>`
- `block: crate::stmt::Block`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprBreak

*Struct*

A `break`, with an optional label to break and an optional
expression.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `break_token: $crate::token::Break`
- `label: Option<crate::lifetime::Lifetime>`
- `expr: Option<alloc::boxed::Box<Expr>>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprCall

*Struct*

A function call expression: `invoke(a, b)`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `func: alloc::boxed::Box<Expr>`
- `paren_token: token::Paren`
- `args: crate::punctuated::Punctuated<Expr, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprCast

*Struct*

A cast expression: `foo as f64`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `expr: alloc::boxed::Box<Expr>`
- `as_token: $crate::token::As`
- `ty: alloc::boxed::Box<crate::ty::Type>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprClosure

*Struct*

A closure expression: `|a, b| a + b`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `lifetimes: Option<crate::generics::BoundLifetimes>`
- `constness: Option<$crate::token::Const>`
- `movability: Option<$crate::token::Static>`
- `asyncness: Option<$crate::token::Async>`
- `capture: Option<$crate::token::Move>`
- `or1_token: $crate::token::Or`
- `inputs: crate::punctuated::Punctuated<crate::pat::Pat, $crate::token::Comma>`
- `or2_token: $crate::token::Or`
- `output: crate::ty::ReturnType`
- `body: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprConst

*Struct*

A const block: `const { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `const_token: $crate::token::Const`
- `block: crate::stmt::Block`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprContinue

*Struct*

A `continue`, with an optional label.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `continue_token: $crate::token::Continue`
- `label: Option<crate::lifetime::Lifetime>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprField

*Struct*

Access of a named struct field (`obj.k`) or unnamed tuple struct
field (`obj.0`).

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `base: alloc::boxed::Box<Expr>`
- `dot_token: $crate::token::Dot`
- `member: Member`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprForLoop

*Struct*

A for loop: `for pat in expr { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `label: Option<Label>`
- `for_token: $crate::token::For`
- `pat: alloc::boxed::Box<crate::pat::Pat>`
- `in_token: $crate::token::In`
- `expr: alloc::boxed::Box<Expr>`
- `body: crate::stmt::Block`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprGroup

*Struct*

An expression contained within invisible delimiters.

This variant is important for faithfully representing the precedence
of expressions and is related to `None`-delimited spans in a
`TokenStream`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `group_token: token::Group`
- `expr: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprIf

*Struct*

An `if` expression with an optional `else` block: `if expr { ... }
else { ... }`.

The `else` branch expression may only be an `If` or `Block`
expression, not any of the other types of expression.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `if_token: $crate::token::If`
- `cond: alloc::boxed::Box<Expr>`
- `then_branch: crate::stmt::Block`
- `else_branch: Option<($crate::token::Else, alloc::boxed::Box<Expr>)>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprIndex

*Struct*

A square bracketed indexing expression: `vector[2]`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `expr: alloc::boxed::Box<Expr>`
- `bracket_token: token::Bracket`
- `index: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprInfer

*Struct*

The inferred value of a const generic argument, denoted `_`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `underscore_token: $crate::token::Underscore`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprLet

*Struct*

A `let` guard: `let Some(x) = opt`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `let_token: $crate::token::Let`
- `pat: alloc::boxed::Box<crate::pat::Pat>`
- `eq_token: $crate::token::Eq`
- `expr: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprLit

*Struct*

A literal in place of an expression: `1`, `"foo"`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `lit: crate::lit::Lit`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprLoop

*Struct*

Conditionless loop: `loop { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `label: Option<Label>`
- `loop_token: $crate::token::Loop`
- `body: crate::stmt::Block`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprMacro

*Struct*

A macro invocation expression: `format!("{}", q)`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `mac: crate::mac::Macro`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprMatch

*Struct*

A `match` expression: `match n { Some(n) => {}, None => {} }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `match_token: $crate::token::Match`
- `expr: alloc::boxed::Box<Expr>`
- `brace_token: token::Brace`
- `arms: alloc::vec::Vec<Arm>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprMethodCall

*Struct*

A method call expression: `x.foo::<T>(a, b)`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `receiver: alloc::boxed::Box<Expr>`
- `dot_token: $crate::token::Dot`
- `method: crate::ident::Ident`
- `turbofish: Option<crate::path::AngleBracketedGenericArguments>`
- `paren_token: token::Paren`
- `args: crate::punctuated::Punctuated<Expr, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprParen

*Struct*

A parenthesized expression: `(a + b)`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `paren_token: token::Paren`
- `expr: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprPath

*Struct*

A path like `core::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `qself: Option<crate::path::QSelf>`
- `path: crate::path::Path`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprRange

*Struct*

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `start: Option<alloc::boxed::Box<Expr>>`
- `limits: RangeLimits`
- `end: Option<alloc::boxed::Box<Expr>>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprRawAddr

*Struct*

Address-of operation: `&raw const place` or `&raw mut place`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `and_token: $crate::token::And`
- `raw: $crate::token::Raw`
- `mutability: PointerMutability`
- `expr: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprReference

*Struct*

A referencing operation: `&a` or `&mut a`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `and_token: $crate::token::And`
- `mutability: Option<$crate::token::Mut>`
- `expr: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprRepeat

*Struct*

An array literal constructed from one repeated element: `[0u8; N]`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `bracket_token: token::Bracket`
- `expr: alloc::boxed::Box<Expr>`
- `semi_token: $crate::token::Semi`
- `len: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprReturn

*Struct*

A `return`, with an optional value to be returned.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `return_token: $crate::token::Return`
- `expr: Option<alloc::boxed::Box<Expr>>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprStruct

*Struct*

A struct literal expression: `Point { x: 1, y: 1 }`.

The `rest` provides the value of the remaining fields as in `S { a:
1, b: 1, ..rest }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `qself: Option<crate::path::QSelf>`
- `path: crate::path::Path`
- `brace_token: token::Brace`
- `fields: crate::punctuated::Punctuated<FieldValue, $crate::token::Comma>`
- `dot2_token: Option<$crate::token::DotDot>`
- `rest: Option<alloc::boxed::Box<Expr>>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::ExprTry

*Struct*

A try-expression: `expr?`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `expr: alloc::boxed::Box<Expr>`
- `question_token: $crate::token::Question`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprTryBlock

*Struct*

A try block: `try { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `try_token: $crate::token::Try`
- `block: crate::stmt::Block`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprTuple

*Struct*

A tuple expression: `(a, b, c, d)`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `paren_token: token::Paren`
- `elems: crate::punctuated::Punctuated<Expr, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprUnary

*Struct*

A unary operation: `!x`, `*x`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `op: crate::op::UnOp`
- `expr: alloc::boxed::Box<Expr>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::ExprUnsafe

*Struct*

An unsafe block: `unsafe { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `unsafe_token: $crate::token::Unsafe`
- `block: crate::stmt::Block`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprWhile

*Struct*

A while loop: `while expr { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `label: Option<Label>`
- `while_token: $crate::token::While`
- `cond: alloc::boxed::Box<Expr>`
- `body: crate::stmt::Block`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::ExprYield

*Struct*

A yield expression: `yield expr`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `yield_token: $crate::token::Yield`
- `expr: Option<alloc::boxed::Box<Expr>>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::FieldValue

*Struct*

A field-value pair in a struct literal.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `member: Member`
- `colon_token: Option<$crate::token::Colon>` - The colon in `Struct { x: x }`. If written in shorthand like
- `expr: Expr`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::Index

*Struct*

The index of an unnamed tuple struct field.

**Fields:**
- `index: u32`
- `span: proc_macro2::Span`

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **IdentFragment**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
  - `fn span(self: &Self) -> Option<Span>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(index: usize) -> Index`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::expr::Label

*Struct*

A lifetime labeling a `for`, `while`, or `loop`.

**Fields:**
- `name: crate::lifetime::Lifetime`
- `colon_token: $crate::token::Colon`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::expr::Member

*Enum*

A struct or tuple struct field accessed in a struct literal or field
expression.

**Variants:**
- `Named(crate::ident::Ident)` - A named field like `self.x`.
- `Unnamed(Index)` - An unnamed field like `self.0`.

**Methods:**

- `fn is_named(self: &Self) -> bool`

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **From**
  - `fn from(index: Index) -> Member`
- **IdentFragment**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
  - `fn span(self: &Self) -> Option<Span>`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(ident: Ident) -> Member`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(index: usize) -> Member`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## syn::expr::PointerMutability

*Enum*

Mutability of a raw pointer (`*const T`, `*mut T`), in which non-mutable
isn't the implicit default.

**Variants:**
- `Const($crate::token::Const)`
- `Mut($crate::token::Mut)`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::RangeLimits

*Enum*

Limit types of a range, inclusive or exclusive.

**Variants:**
- `HalfOpen($crate::token::DotDot)` - Inclusive at the beginning, exclusive at the end.
- `Closed($crate::token::DotDotEq)` - Inclusive at the beginning and end.

**Methods:**

- `fn parse_obsolete(input: ParseStream) -> Result<Self>`

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



