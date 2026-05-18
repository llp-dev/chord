*[syn](../index.md) / [expr](index.md)*

---

# Module `expr`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`ExprArray`](#exprarray)
  - [`ExprAssign`](#exprassign)
  - [`ExprAsync`](#exprasync)
  - [`ExprAwait`](#exprawait)
  - [`ExprBinary`](#exprbinary)
  - [`ExprBlock`](#exprblock)
  - [`ExprBreak`](#exprbreak)
  - [`ExprCall`](#exprcall)
  - [`ExprCast`](#exprcast)
  - [`ExprClosure`](#exprclosure)
  - [`ExprConst`](#exprconst)
  - [`ExprContinue`](#exprcontinue)
  - [`ExprField`](#exprfield)
  - [`ExprForLoop`](#exprforloop)
  - [`ExprGroup`](#exprgroup)
  - [`ExprIf`](#exprif)
  - [`ExprIndex`](#exprindex)
  - [`ExprInfer`](#exprinfer)
  - [`ExprLet`](#exprlet)
  - [`ExprLit`](#exprlit)
  - [`ExprLoop`](#exprloop)
  - [`ExprMacro`](#exprmacro)
  - [`ExprMatch`](#exprmatch)
  - [`ExprMethodCall`](#exprmethodcall)
  - [`ExprParen`](#exprparen)
  - [`ExprPath`](#exprpath)
  - [`ExprRange`](#exprrange)
  - [`ExprRawAddr`](#exprrawaddr)
  - [`ExprReference`](#exprreference)
  - [`ExprRepeat`](#exprrepeat)
  - [`ExprReturn`](#exprreturn)
  - [`ExprStruct`](#exprstruct)
  - [`ExprTry`](#exprtry)
  - [`ExprTryBlock`](#exprtryblock)
  - [`ExprTuple`](#exprtuple)
  - [`ExprUnary`](#exprunary)
  - [`ExprUnsafe`](#exprunsafe)
  - [`ExprWhile`](#exprwhile)
  - [`ExprYield`](#expryield)
  - [`Index`](#index)
  - [`FieldValue`](#fieldvalue)
  - [`Label`](#label)
  - [`Arm`](#arm)
- [Enums](#enums)
  - [`Expr`](#expr)
  - [`Member`](#member)
  - [`RangeLimits`](#rangelimits)
  - [`PointerMutability`](#pointermutability)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`ExprArray`](#exprarray) | struct | A slice literal expression: `[a, b, c, d]`. |
| [`ExprAssign`](#exprassign) | struct | An assignment expression: `a = compute()`. |
| [`ExprAsync`](#exprasync) | struct | An async block: `async { ... |
| [`ExprAwait`](#exprawait) | struct | An await expression: `fut.await`. |
| [`ExprBinary`](#exprbinary) | struct | A binary operation: `a + b`, `a += b`. |
| [`ExprBlock`](#exprblock) | struct | A blocked scope: `{ ... |
| [`ExprBreak`](#exprbreak) | struct | A `break`, with an optional label to break and an optional expression. |
| [`ExprCall`](#exprcall) | struct | A function call expression: `invoke(a, b)`. |
| [`ExprCast`](#exprcast) | struct | A cast expression: `foo as f64`. |
| [`ExprClosure`](#exprclosure) | struct | A closure expression: `\|a, b\| a + b`. |
| [`ExprConst`](#exprconst) | struct | A const block: `const { ... |
| [`ExprContinue`](#exprcontinue) | struct | A `continue`, with an optional label. |
| [`ExprField`](#exprfield) | struct | Access of a named struct field (`obj.k`) or unnamed tuple struct field (`obj.0`). |
| [`ExprForLoop`](#exprforloop) | struct | A for loop: `for pat in expr { ... |
| [`ExprGroup`](#exprgroup) | struct | An expression contained within invisible delimiters. |
| [`ExprIf`](#exprif) | struct | An `if` expression with an optional `else` block: `if expr { ... |
| [`ExprIndex`](#exprindex) | struct | A square bracketed indexing expression: `vector[2]`. |
| [`ExprInfer`](#exprinfer) | struct | The inferred value of a const generic argument, denoted `_`. |
| [`ExprLet`](#exprlet) | struct | A `let` guard: `let Some(x) = opt`. |
| [`ExprLit`](#exprlit) | struct | A literal in place of an expression: `1`, `"foo"`. |
| [`ExprLoop`](#exprloop) | struct | Conditionless loop: `loop { ... |
| [`ExprMacro`](#exprmacro) | struct | A macro invocation expression: `format!("{}", q)`. |
| [`ExprMatch`](#exprmatch) | struct | A `match` expression: `match n { Some(n) => {}, None => {} }`. |
| [`ExprMethodCall`](#exprmethodcall) | struct | A method call expression: `x.foo::<T>(a, b)`. |
| [`ExprParen`](#exprparen) | struct | A parenthesized expression: `(a + b)`. |
| [`ExprPath`](#exprpath) | struct | A path like `core::mem::replace` possibly containing generic parameters and a qualified self-type. |
| [`ExprRange`](#exprrange) | struct | A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`. |
| [`ExprRawAddr`](#exprrawaddr) | struct | Address-of operation: `&raw const place` or `&raw mut place`. |
| [`ExprReference`](#exprreference) | struct | A referencing operation: `&a` or `&mut a`. |
| [`ExprRepeat`](#exprrepeat) | struct | An array literal constructed from one repeated element: `[0u8; N]`. |
| [`ExprReturn`](#exprreturn) | struct | A `return`, with an optional value to be returned. |
| [`ExprStruct`](#exprstruct) | struct | A struct literal expression: `Point { x: 1, y: 1 }`. |
| [`ExprTry`](#exprtry) | struct | A try-expression: `expr?`. |
| [`ExprTryBlock`](#exprtryblock) | struct | A try block: `try { ... |
| [`ExprTuple`](#exprtuple) | struct | A tuple expression: `(a, b, c, d)`. |
| [`ExprUnary`](#exprunary) | struct | A unary operation: `!x`, `*x`. |
| [`ExprUnsafe`](#exprunsafe) | struct | An unsafe block: `unsafe { ... |
| [`ExprWhile`](#exprwhile) | struct | A while loop: `while expr { ... |
| [`ExprYield`](#expryield) | struct | A yield expression: `yield expr`. |
| [`Index`](#index) | struct | The index of an unnamed tuple struct field. |
| [`FieldValue`](#fieldvalue) | struct | A field-value pair in a struct literal. |
| [`Label`](#label) | struct | A lifetime labeling a `for`, `while`, or `loop`. |
| [`Arm`](#arm) | struct | One arm of a `match` expression: `0..=10 => { return true; }`. |
| [`Expr`](#expr) | enum | A Rust expression. |
| [`Member`](#member) | enum | A struct or tuple struct field accessed in a struct literal or field expression. |
| [`RangeLimits`](#rangelimits) | enum | Limit types of a range, inclusive or exclusive. |
| [`PointerMutability`](#pointermutability) | enum | Mutability of a raw pointer (`*const T`, `*mut T`), in which non-mutable isn't the implicit default. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `ExprArray`

```rust
struct ExprArray {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

A slice literal expression: `[a, b, c, d]`.

#### Implementations

- <span id="crateexprarray-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprArray`

- <span id="crateexprarray-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprArray`

- <span id="crateexprarray-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprArray`

##### `impl Hash for crate::ExprArray`

- <span id="crateexprarray-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprArray`

- <span id="crateexprexprarray-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprArray`

- <span id="crateexprarray-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprArray`

##### `impl Spanned for ExprArray`

- <span id="exprarray-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprArray`

- <span id="crateexprexprarray-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprAssign`

```rust
struct ExprAssign {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub left: alloc::boxed::Box<Expr>,
    pub eq_token: token::Eq,
    pub right: alloc::boxed::Box<Expr>,
}
```

An assignment expression: `a = compute()`.

#### Implementations

- <span id="crateexprassign-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAssign`

- <span id="crateexprassign-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprAssign`

- <span id="crateexprassign-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAssign`

##### `impl Hash for crate::ExprAssign`

- <span id="crateexprassign-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAssign`

- <span id="crateexprexprassign-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprAssign`

- <span id="crateexprassign-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprAssign`

##### `impl Spanned for ExprAssign`

- <span id="exprassign-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprAssign`

- <span id="crateexprexprassign-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprAsync`

```rust
struct ExprAsync {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub async_token: token::Async,
    pub capture: Option<token::Move>,
    pub block: crate::stmt::Block,
}
```

An async block: `async { ... }`.

#### Implementations

- <span id="crateexprasync-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAsync`

- <span id="crateexprasync-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprAsync`

- <span id="crateexprasync-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAsync`

##### `impl Hash for crate::ExprAsync`

- <span id="crateexprasync-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAsync`

- <span id="crateexprexprasync-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprAsync`

- <span id="crateexprasync-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprAsync`

##### `impl Spanned for ExprAsync`

- <span id="exprasync-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprAsync`

- <span id="crateexprexprasync-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprAwait`

```rust
struct ExprAwait {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub base: alloc::boxed::Box<Expr>,
    pub dot_token: token::Dot,
    pub await_token: token::Await,
}
```

An await expression: `fut.await`.

#### Implementations

- <span id="crateexprawait-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAwait`

- <span id="crateexprawait-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprAwait`

- <span id="crateexprawait-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAwait`

##### `impl Hash for crate::ExprAwait`

- <span id="crateexprawait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAwait`

- <span id="crateexprexprawait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprAwait`

- <span id="crateexprawait-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprAwait`

##### `impl Spanned for ExprAwait`

- <span id="exprawait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprAwait`

- <span id="crateexprexprawait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprBinary`

```rust
struct ExprBinary {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub left: alloc::boxed::Box<Expr>,
    pub op: crate::op::BinOp,
    pub right: alloc::boxed::Box<Expr>,
}
```

A binary operation: `a + b`, `a += b`.

#### Implementations

- <span id="crateexprbinary-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBinary`

- <span id="crateexprbinary-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprBinary`

- <span id="crateexprbinary-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBinary`

##### `impl Hash for crate::ExprBinary`

- <span id="crateexprbinary-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBinary`

- <span id="crateexprexprbinary-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprBinary`

- <span id="crateexprbinary-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprBinary`

##### `impl Spanned for ExprBinary`

- <span id="exprbinary-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprBinary`

- <span id="crateexprexprbinary-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprBlock`

```rust
struct ExprBlock {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub block: crate::stmt::Block,
}
```

A blocked scope: `{ ... }`.

#### Implementations

- <span id="crateexprblock-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBlock`

- <span id="crateexprblock-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprBlock`

- <span id="crateexprblock-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBlock`

##### `impl Hash for crate::ExprBlock`

- <span id="crateexprblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBlock`

- <span id="crateexprexprblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprBlock`

- <span id="crateexprblock-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprBlock`

##### `impl Spanned for ExprBlock`

- <span id="exprblock-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprBlock`

- <span id="crateexprexprblock-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprBreak`

```rust
struct ExprBreak {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub break_token: token::Break,
    pub label: Option<crate::lifetime::Lifetime>,
    pub expr: Option<alloc::boxed::Box<Expr>>,
}
```

A `break`, with an optional label to break and an optional
expression.

#### Implementations

- <span id="crateexprbreak-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBreak`

- <span id="crateexprbreak-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprBreak`

- <span id="crateexprbreak-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBreak`

##### `impl Hash for crate::ExprBreak`

- <span id="crateexprbreak-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBreak`

- <span id="crateexprexprbreak-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprBreak`

- <span id="crateexprbreak-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprBreak`

##### `impl Spanned for ExprBreak`

- <span id="exprbreak-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprBreak`

- <span id="crateexprexprbreak-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprCall`

```rust
struct ExprCall {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub func: alloc::boxed::Box<Expr>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

A function call expression: `invoke(a, b)`.

#### Implementations

- <span id="crateexprcall-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprCall`

- <span id="crateexprcall-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprCall`

- <span id="crateexprcall-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCall`

##### `impl Hash for crate::ExprCall`

- <span id="crateexprcall-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprCall`

- <span id="crateexprexprcall-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprCall`

- <span id="crateexprcall-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprCall`

##### `impl Spanned for ExprCall`

- <span id="exprcall-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprCall`

- <span id="crateexprexprcall-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprCast`

```rust
struct ExprCast {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub expr: alloc::boxed::Box<Expr>,
    pub as_token: token::As,
    pub ty: alloc::boxed::Box<crate::ty::Type>,
}
```

A cast expression: `foo as f64`.

#### Implementations

- <span id="crateexprcast-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprCast`

- <span id="crateexprcast-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprCast`

- <span id="crateexprcast-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCast`

##### `impl Hash for crate::ExprCast`

- <span id="crateexprcast-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprCast`

- <span id="crateexprexprcast-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprCast`

- <span id="crateexprcast-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprCast`

##### `impl Spanned for ExprCast`

- <span id="exprcast-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprCast`

- <span id="crateexprexprcast-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprClosure`

```rust
struct ExprClosure {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub lifetimes: Option<crate::generics::BoundLifetimes>,
    pub constness: Option<token::Const>,
    pub movability: Option<token::Static>,
    pub asyncness: Option<token::Async>,
    pub capture: Option<token::Move>,
    pub or1_token: token::Or,
    pub inputs: crate::punctuated::Punctuated<crate::pat::Pat, token::Comma>,
    pub or2_token: token::Or,
    pub output: crate::ty::ReturnType,
    pub body: alloc::boxed::Box<Expr>,
}
```

A closure expression: `|a, b| a + b`.

#### Implementations

- <span id="crateexprclosure-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprClosure`

- <span id="crateexprclosure-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprClosure`

- <span id="crateexprclosure-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprClosure`

##### `impl Hash for crate::ExprClosure`

- <span id="crateexprclosure-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprClosure`

- <span id="crateexprexprclosure-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprClosure`

- <span id="crateexprclosure-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprClosure`

##### `impl Spanned for ExprClosure`

- <span id="exprclosure-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprClosure`

- <span id="crateexprexprclosure-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprConst`

```rust
struct ExprConst {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub block: crate::stmt::Block,
}
```

A const block: `const { ... }`.

#### Implementations

- <span id="crateexprconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprConst`

- <span id="crateexprconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprConst`

- <span id="crateexprconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprConst`

##### `impl Hash for crate::ExprConst`

- <span id="crateexprconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprConst`

- <span id="crateexprexprconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprConst`

- <span id="crateexprconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprConst`

##### `impl Spanned for ExprConst`

- <span id="exprconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprConst`

- <span id="crateexprexprconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprContinue`

```rust
struct ExprContinue {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub continue_token: token::Continue,
    pub label: Option<crate::lifetime::Lifetime>,
}
```

A `continue`, with an optional label.

#### Implementations

- <span id="crateexprcontinue-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprContinue`

- <span id="crateexprcontinue-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprContinue`

- <span id="crateexprcontinue-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprContinue`

##### `impl Hash for crate::ExprContinue`

- <span id="crateexprcontinue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprContinue`

- <span id="crateexprexprcontinue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprContinue`

- <span id="crateexprcontinue-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprContinue`

##### `impl Spanned for ExprContinue`

- <span id="exprcontinue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprContinue`

- <span id="crateexprexprcontinue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprField`

```rust
struct ExprField {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub base: alloc::boxed::Box<Expr>,
    pub dot_token: token::Dot,
    pub member: Member,
}
```

Access of a named struct field (`obj.k`) or unnamed tuple struct
field (`obj.0`).

#### Implementations

- <span id="crateexprfield-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprField`

- <span id="crateexprfield-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprField`

- <span id="crateexprfield-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprField`

##### `impl Hash for crate::ExprField`

- <span id="crateexprfield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprField`

- <span id="crateexprexprfield-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprField`

- <span id="crateexprfield-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprField`

##### `impl Spanned for ExprField`

- <span id="exprfield-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprField`

- <span id="crateexprexprfield-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprForLoop`

```rust
struct ExprForLoop {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub for_token: token::For,
    pub pat: alloc::boxed::Box<crate::pat::Pat>,
    pub in_token: token::In,
    pub expr: alloc::boxed::Box<Expr>,
    pub body: crate::stmt::Block,
}
```

A for loop: `for pat in expr { ... }`.

#### Implementations

- <span id="crateexprforloop-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprForLoop`

- <span id="crateexprforloop-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprForLoop`

- <span id="crateexprforloop-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprForLoop`

##### `impl Hash for crate::ExprForLoop`

- <span id="crateexprforloop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprForLoop`

- <span id="crateexprexprforloop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprForLoop`

- <span id="crateexprforloop-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprForLoop`

##### `impl Spanned for ExprForLoop`

- <span id="exprforloop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprForLoop`

- <span id="crateexprexprforloop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprGroup`

```rust
struct ExprGroup {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub group_token: token::Group,
    pub expr: alloc::boxed::Box<Expr>,
}
```

An expression contained within invisible delimiters.

This variant is important for faithfully representing the precedence
of expressions and is related to `None`-delimited spans in a
`TokenStream`.

#### Implementations

- <span id="crateexprgroup-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprGroup`

- <span id="crateexprgroup-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprGroup`

- <span id="crateexprgroup-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprGroup`

##### `impl Hash for crate::ExprGroup`

- <span id="crateexprgroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::ExprGroup`

- <span id="crateexprgroup-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprGroup`

##### `impl Spanned for ExprGroup`

- <span id="exprgroup-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprGroup`

- <span id="crateexprexprgroup-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprIf`

```rust
struct ExprIf {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub if_token: token::If,
    pub cond: alloc::boxed::Box<Expr>,
    pub then_branch: crate::stmt::Block,
    pub else_branch: Option<(token::Else, alloc::boxed::Box<Expr>)>,
}
```

An `if` expression with an optional `else` block: `if expr { ... }
else { ... }`.

The `else` branch expression may only be an `If` or `Block`
expression, not any of the other types of expression.

#### Implementations

- <span id="crateexprif-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprIf`

- <span id="crateexprif-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprIf`

- <span id="crateexprif-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIf`

##### `impl Hash for crate::ExprIf`

- <span id="crateexprif-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprIf`

- <span id="crateexprexprif-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprIf`

- <span id="crateexprif-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprIf`

##### `impl Spanned for ExprIf`

- <span id="exprif-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprIf`

- <span id="crateexprexprif-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprIndex`

```rust
struct ExprIndex {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub expr: alloc::boxed::Box<Expr>,
    pub bracket_token: token::Bracket,
    pub index: alloc::boxed::Box<Expr>,
}
```

A square bracketed indexing expression: `vector[2]`.

#### Implementations

- <span id="crateexprindex-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprIndex`

- <span id="crateexprindex-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprIndex`

- <span id="crateexprindex-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIndex`

##### `impl Hash for crate::ExprIndex`

- <span id="crateexprindex-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprIndex`

- <span id="crateexprexprindex-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprIndex`

- <span id="crateexprindex-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprIndex`

##### `impl Spanned for ExprIndex`

- <span id="exprindex-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprIndex`

- <span id="crateexprexprindex-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprInfer`

```rust
struct ExprInfer {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub underscore_token: token::Underscore,
}
```

The inferred value of a const generic argument, denoted `_`.

#### Implementations

- <span id="crateexprinfer-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprInfer`

- <span id="crateexprinfer-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprInfer`

- <span id="crateexprinfer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprInfer`

##### `impl Hash for crate::ExprInfer`

- <span id="crateexprinfer-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprInfer`

- <span id="crateexprexprinfer-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprInfer`

- <span id="crateexprinfer-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprInfer`

##### `impl Spanned for ExprInfer`

- <span id="exprinfer-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprInfer`

- <span id="crateexprexprinfer-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprLet`

```rust
struct ExprLet {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub let_token: token::Let,
    pub pat: alloc::boxed::Box<crate::pat::Pat>,
    pub eq_token: token::Eq,
    pub expr: alloc::boxed::Box<Expr>,
}
```

A `let` guard: `let Some(x) = opt`.

#### Implementations

- <span id="crateexprlet-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLet`

- <span id="crateexprlet-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprLet`

- <span id="crateexprlet-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLet`

##### `impl Hash for crate::ExprLet`

- <span id="crateexprlet-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLet`

- <span id="crateexprexprlet-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprLet`

- <span id="crateexprlet-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLet`

##### `impl Spanned for ExprLet`

- <span id="exprlet-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprLet`

- <span id="crateexprexprlet-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprLit`

```rust
struct ExprLit {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

A literal in place of an expression: `1`, `"foo"`.

#### Implementations

- <span id="crateexprlit-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLit`

- <span id="crateexprlit-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprLit`

- <span id="crateexprlit-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLit`

##### `impl Hash for crate::ExprLit`

- <span id="crateexprlit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLit`

- <span id="crateexprexprlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprLit`

- <span id="crateexprlit-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLit`

##### `impl Spanned for ExprLit`

- <span id="exprlit-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprLit`

- <span id="crateexprexprlit-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprLoop`

```rust
struct ExprLoop {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub loop_token: token::Loop,
    pub body: crate::stmt::Block,
}
```

Conditionless loop: `loop { ... }`.

#### Implementations

- <span id="crateexprloop-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLoop`

- <span id="crateexprloop-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprLoop`

- <span id="crateexprloop-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLoop`

##### `impl Hash for crate::ExprLoop`

- <span id="crateexprloop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLoop`

- <span id="crateexprexprloop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprLoop`

- <span id="crateexprloop-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLoop`

##### `impl Spanned for ExprLoop`

- <span id="exprloop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprLoop`

- <span id="crateexprexprloop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprMacro`

```rust
struct ExprMacro {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

A macro invocation expression: `format!("{}", q)`.

#### Implementations

- <span id="crateexprmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMacro`

- <span id="crateexprmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprMacro`

- <span id="crateexprmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMacro`

##### `impl Hash for crate::ExprMacro`

- <span id="crateexprmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprMacro`

- <span id="crateexprmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMacro`

##### `impl Spanned for ExprMacro`

- <span id="exprmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprMatch`

```rust
struct ExprMatch {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub match_token: token::Match,
    pub expr: alloc::boxed::Box<Expr>,
    pub brace_token: token::Brace,
    pub arms: alloc::vec::Vec<Arm>,
}
```

A `match` expression: `match n { Some(n) => {}, None => {} }`.

#### Implementations

- <span id="crateexprmatch-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMatch`

- <span id="crateexprmatch-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprMatch`

- <span id="crateexprmatch-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMatch`

##### `impl Hash for crate::ExprMatch`

- <span id="crateexprmatch-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMatch`

- <span id="crateexprexprmatch-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprMatch`

- <span id="crateexprmatch-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMatch`

##### `impl Spanned for ExprMatch`

- <span id="exprmatch-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprMatch`

- <span id="crateexprexprmatch-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprMethodCall`

```rust
struct ExprMethodCall {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub receiver: alloc::boxed::Box<Expr>,
    pub dot_token: token::Dot,
    pub method: crate::ident::Ident,
    pub turbofish: Option<crate::path::AngleBracketedGenericArguments>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

A method call expression: `x.foo::<T>(a, b)`.

#### Implementations

- <span id="crateexprmethodcall-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMethodCall`

- <span id="crateexprmethodcall-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprMethodCall`

- <span id="crateexprmethodcall-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMethodCall`

##### `impl Hash for crate::ExprMethodCall`

- <span id="crateexprmethodcall-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMethodCall`

- <span id="crateexprexprmethodcall-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprMethodCall`

- <span id="crateexprmethodcall-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMethodCall`

##### `impl Spanned for ExprMethodCall`

- <span id="exprmethodcall-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprMethodCall`

- <span id="crateexprexprmethodcall-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprParen`

```rust
struct ExprParen {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub expr: alloc::boxed::Box<Expr>,
}
```

A parenthesized expression: `(a + b)`.

#### Implementations

- <span id="crateexprparen-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprParen`

- <span id="crateexprparen-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprParen`

- <span id="crateexprparen-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprParen`

##### `impl Hash for crate::ExprParen`

- <span id="crateexprparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprParen`

- <span id="crateexprexprparen-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprParen`

- <span id="crateexprparen-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprParen`

##### `impl Spanned for ExprParen`

- <span id="exprparen-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprParen`

- <span id="crateexprexprparen-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprPath`

```rust
struct ExprPath {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `core::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

#### Implementations

- <span id="crateexprpath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprPath`

- <span id="crateexprpath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprPath`

- <span id="crateexprpath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprPath`

##### `impl Hash for crate::ExprPath`

- <span id="crateexprpath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprPath`

- <span id="crateexprexprpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprPath`

- <span id="crateexprpath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprPath`

##### `impl Spanned for ExprPath`

- <span id="exprpath-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprPath`

- <span id="crateexprexprpath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprRange`

```rust
struct ExprRange {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub start: Option<alloc::boxed::Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<alloc::boxed::Box<Expr>>,
}
```

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

#### Implementations

- <span id="crateexprrange-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRange`

- <span id="crateexprrange-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprRange`

- <span id="crateexprrange-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRange`

##### `impl Hash for crate::ExprRange`

- <span id="crateexprrange-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRange`

- <span id="crateexprexprrange-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprRange`

- <span id="crateexprrange-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRange`

##### `impl Spanned for ExprRange`

- <span id="exprrange-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprRange`

- <span id="crateexprexprrange-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprRawAddr`

```rust
struct ExprRawAddr {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub raw: token::Raw,
    pub mutability: PointerMutability,
    pub expr: alloc::boxed::Box<Expr>,
}
```

Address-of operation: `&raw const place` or `&raw mut place`.

#### Implementations

- <span id="crateexprrawaddr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRawAddr`

- <span id="crateexprrawaddr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprRawAddr`

- <span id="crateexprrawaddr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRawAddr`

##### `impl Hash for crate::ExprRawAddr`

- <span id="crateexprrawaddr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRawAddr`

- <span id="crateexprexprrawaddr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprRawAddr`

- <span id="crateexprrawaddr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRawAddr`

##### `impl Spanned for ExprRawAddr`

- <span id="exprrawaddr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprRawAddr`

- <span id="crateexprexprrawaddr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprReference`

```rust
struct ExprReference {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub mutability: Option<token::Mut>,
    pub expr: alloc::boxed::Box<Expr>,
}
```

A referencing operation: `&a` or `&mut a`.

#### Implementations

- <span id="crateexprreference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprReference`

- <span id="crateexprreference-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprReference`

- <span id="crateexprreference-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReference`

##### `impl Hash for crate::ExprReference`

- <span id="crateexprreference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprReference`

- <span id="crateexprexprreference-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprReference`

- <span id="crateexprreference-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprReference`

##### `impl Spanned for ExprReference`

- <span id="exprreference-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprReference`

- <span id="crateexprexprreference-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprRepeat`

```rust
struct ExprRepeat {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub expr: alloc::boxed::Box<Expr>,
    pub semi_token: token::Semi,
    pub len: alloc::boxed::Box<Expr>,
}
```

An array literal constructed from one repeated element: `[0u8; N]`.

#### Implementations

- <span id="crateexprrepeat-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRepeat`

- <span id="crateexprrepeat-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprRepeat`

- <span id="crateexprrepeat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRepeat`

##### `impl Hash for crate::ExprRepeat`

- <span id="crateexprrepeat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRepeat`

- <span id="crateexprexprrepeat-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprRepeat`

- <span id="crateexprrepeat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRepeat`

##### `impl Spanned for ExprRepeat`

- <span id="exprrepeat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprRepeat`

- <span id="crateexprexprrepeat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprReturn`

```rust
struct ExprReturn {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub return_token: token::Return,
    pub expr: Option<alloc::boxed::Box<Expr>>,
}
```

A `return`, with an optional value to be returned.

#### Implementations

- <span id="crateexprreturn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprReturn`

- <span id="crateexprreturn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprReturn`

- <span id="crateexprreturn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReturn`

##### `impl Hash for crate::ExprReturn`

- <span id="crateexprreturn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprReturn`

- <span id="crateexprexprreturn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprReturn`

- <span id="crateexprreturn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprReturn`

##### `impl Spanned for ExprReturn`

- <span id="exprreturn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprReturn`

- <span id="crateexprexprreturn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprStruct`

```rust
struct ExprStruct {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub brace_token: token::Brace,
    pub fields: crate::punctuated::Punctuated<FieldValue, token::Comma>,
    pub dot2_token: Option<token::DotDot>,
    pub rest: Option<alloc::boxed::Box<Expr>>,
}
```

A struct literal expression: `Point { x: 1, y: 1 }`.

The `rest` provides the value of the remaining fields as in `S { a:
1, b: 1, ..rest }`.

#### Implementations

- <span id="crateexprstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprStruct`

- <span id="crateexprstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprStruct`

- <span id="crateexprstruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprStruct`

##### `impl Hash for crate::ExprStruct`

- <span id="crateexprstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprStruct`

- <span id="crateexprexprstruct-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprStruct`

- <span id="crateexprstruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprStruct`

##### `impl Spanned for ExprStruct`

- <span id="exprstruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprStruct`

- <span id="crateexprexprstruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprTry`

```rust
struct ExprTry {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub expr: alloc::boxed::Box<Expr>,
    pub question_token: token::Question,
}
```

A try-expression: `expr?`.

#### Implementations

- <span id="crateexprtry-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTry`

- <span id="crateexprtry-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprTry`

- <span id="crateexprtry-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTry`

##### `impl Hash for crate::ExprTry`

- <span id="crateexprtry-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTry`

- <span id="crateexprexprtry-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprTry`

- <span id="crateexprtry-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprTry`

##### `impl Spanned for ExprTry`

- <span id="exprtry-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprTry`

- <span id="crateexprexprtry-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprTryBlock`

```rust
struct ExprTryBlock {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub try_token: token::Try,
    pub block: crate::stmt::Block,
}
```

A try block: `try { ... }`.

#### Implementations

- <span id="crateexprtryblock-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTryBlock`

- <span id="crateexprtryblock-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprTryBlock`

- <span id="crateexprtryblock-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTryBlock`

##### `impl Hash for crate::ExprTryBlock`

- <span id="crateexprtryblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTryBlock`

- <span id="crateexprexprtryblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprTryBlock`

- <span id="crateexprtryblock-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprTryBlock`

##### `impl Spanned for ExprTryBlock`

- <span id="exprtryblock-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprTryBlock`

- <span id="crateexprexprtryblock-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprTuple`

```rust
struct ExprTuple {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

A tuple expression: `(a, b, c, d)`.

#### Implementations

- <span id="crateexprtuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTuple`

- <span id="crateexprtuple-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprTuple`

- <span id="crateexprtuple-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTuple`

##### `impl Hash for crate::ExprTuple`

- <span id="crateexprtuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTuple`

- <span id="crateexprexprtuple-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprTuple`

- <span id="crateexprtuple-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprTuple`

##### `impl Spanned for ExprTuple`

- <span id="exprtuple-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprTuple`

- <span id="crateexprexprtuple-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprUnary`

```rust
struct ExprUnary {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub op: crate::op::UnOp,
    pub expr: alloc::boxed::Box<Expr>,
}
```

A unary operation: `!x`, `*x`.

#### Implementations

- <span id="crateexprunary-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprUnary`

- <span id="crateexprunary-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprUnary`

- <span id="crateexprunary-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnary`

##### `impl Hash for crate::ExprUnary`

- <span id="crateexprunary-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprUnary`

- <span id="crateexprexprunary-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprUnary`

- <span id="crateexprunary-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprUnary`

##### `impl Spanned for ExprUnary`

- <span id="exprunary-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprUnary`

- <span id="crateexprexprunary-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprUnsafe`

```rust
struct ExprUnsafe {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub unsafe_token: token::Unsafe,
    pub block: crate::stmt::Block,
}
```

An unsafe block: `unsafe { ... }`.

#### Implementations

- <span id="crateexprunsafe-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprUnsafe`

- <span id="crateexprunsafe-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprUnsafe`

- <span id="crateexprunsafe-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnsafe`

##### `impl Hash for crate::ExprUnsafe`

- <span id="crateexprunsafe-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprUnsafe`

- <span id="crateexprexprunsafe-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprUnsafe`

- <span id="crateexprunsafe-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprUnsafe`

##### `impl Spanned for ExprUnsafe`

- <span id="exprunsafe-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprUnsafe`

- <span id="crateexprexprunsafe-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprWhile`

```rust
struct ExprWhile {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub while_token: token::While,
    pub cond: alloc::boxed::Box<Expr>,
    pub body: crate::stmt::Block,
}
```

A while loop: `while expr { ... }`.

#### Implementations

- <span id="crateexprwhile-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprWhile`

- <span id="crateexprwhile-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprWhile`

- <span id="crateexprwhile-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprWhile`

##### `impl Hash for crate::ExprWhile`

- <span id="crateexprwhile-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprWhile`

- <span id="crateexprexprwhile-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprWhile`

- <span id="crateexprwhile-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprWhile`

##### `impl Spanned for ExprWhile`

- <span id="exprwhile-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprWhile`

- <span id="crateexprexprwhile-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ExprYield`

```rust
struct ExprYield {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub yield_token: token::Yield,
    pub expr: Option<alloc::boxed::Box<Expr>>,
}
```

A yield expression: `yield expr`.

#### Implementations

- <span id="crateexpryield-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprYield`

- <span id="crateexpryield-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ExprYield`

- <span id="crateexpryield-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprYield`

##### `impl Hash for crate::ExprYield`

- <span id="crateexpryield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::ExprYield`

- <span id="crateexprexpryield-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprYield`

- <span id="crateexpryield-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprYield`

##### `impl Spanned for ExprYield`

- <span id="expryield-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::ExprYield`

- <span id="crateexprexpryield-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Index`

```rust
struct Index {
    pub index: u32,
    pub span: proc_macro2::Span,
}
```

The index of an unnamed tuple struct field.

#### Trait Implementations

##### `impl Clone for crate::Index`

- <span id="crateindex-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Index`

- <span id="crateindex-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Index`

##### `impl Hash for Index`

- <span id="index-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IdentFragment for Index`

- <span id="index-identfragment-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="index-identfragment-span"></span>`fn span(&self) -> Option<Span>`

##### `impl Parse for crate::expr::Index`

- <span id="crateexprindex-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Index`

- <span id="index-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Index`

##### `impl Spanned for Index`

- <span id="index-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::Index`

- <span id="crateexprindex-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldValue`

```rust
struct FieldValue {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub member: Member,
    pub colon_token: Option<token::Colon>,
    pub expr: Expr,
}
```

A field-value pair in a struct literal.

#### Fields

- **`colon_token`**: `Option<token::Colon>`

  The colon in `Struct { x: x }`. If written in shorthand like
  `Struct { x }`, there is no colon.

#### Trait Implementations

##### `impl Clone for crate::FieldValue`

- <span id="cratefieldvalue-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FieldValue`

- <span id="cratefieldvalue-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldValue`

##### `impl Hash for crate::FieldValue`

- <span id="cratefieldvalue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::FieldValue`

- <span id="crateexprfieldvalue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::FieldValue`

- <span id="cratefieldvalue-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FieldValue`

##### `impl Spanned for FieldValue`

- <span id="fieldvalue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::FieldValue`

- <span id="crateexprfieldvalue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Label`

```rust
struct Label {
    pub name: crate::lifetime::Lifetime,
    pub colon_token: token::Colon,
}
```

A lifetime labeling a `for`, `while`, or `loop`.

#### Trait Implementations

##### `impl Clone for crate::Label`

- <span id="cratelabel-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Label`

- <span id="cratelabel-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Label`

##### `impl Hash for crate::Label`

- <span id="cratelabel-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::Label`

- <span id="crateexprlabel-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Label`

- <span id="cratelabel-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Label`

##### `impl Spanned for Label`

- <span id="label-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::Label`

- <span id="crateexprlabel-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Arm`

```rust
struct Arm {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub pat: crate::pat::Pat,
    pub guard: Option<(token::If, alloc::boxed::Box<Expr>)>,
    pub fat_arrow_token: token::FatArrow,
    pub body: alloc::boxed::Box<Expr>,
    pub comma: Option<token::Comma>,
}
```

One arm of a `match` expression: `0..=10 => { return true; }`.

As in:

```rust
fn f() -> bool {
    let n = 0;
match n {
    0..=10 => {
        return true;
    }
    // ...
    _ => {}
}
  false
}
```

#### Implementations

- <span id="crateexprarm-parse-multiple"></span>`fn parse_multiple(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Clone for crate::Arm`

- <span id="cratearm-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Arm`

- <span id="cratearm-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Arm`

##### `impl Hash for crate::Arm`

- <span id="cratearm-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::Arm`

- <span id="crateexprarm-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Arm>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`Arm`](#arm)

##### `impl PartialEq for crate::Arm`

- <span id="cratearm-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Arm`

##### `impl Spanned for Arm`

- <span id="arm-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::Arm`

- <span id="crateexprarm-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `Expr`

```rust
enum Expr {
    Array(ExprArray),
    Assign(ExprAssign),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Const(ExprConst),
    Continue(ExprContinue),
    Field(ExprField),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    If(ExprIf),
    Index(ExprIndex),
    Infer(ExprInfer),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    RawAddr(ExprRawAddr),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    Verbatim(proc_macro2::TokenStream),
    While(ExprWhile),
    Yield(ExprYield),
}
```

A Rust expression.

*This type is available only if Syn is built with the `"derive"` or `"full"`
feature, but most of the variants are not available unless "full" is enabled.*

# Syntax tree enums

This type is a syntax tree enum. In Syn this and other syntax tree enums
are designed to be traversed using the following rebinding idiom.

```rust
use syn::Expr;

fn example(expr: Expr) {
const IGNORE: &str = stringify! {
let expr: Expr = /* ... */;
};
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
    _ => {}
}
}
```

We begin with a variable `expr` of type `Expr` that has no fields
(because it is an enum), and by matching on it and rebinding a variable
with the same name `expr` we effectively imbue our variable with all of
the data fields provided by the variant that it turned out to be. So for
example above if we ended up in the `MethodCall` case then we get to use
`expr.receiver`, `expr.args` etc; if we ended up in the `If` case we get
to use `expr.cond`, `expr.then_branch`, `expr.else_branch`.

This approach avoids repeating the variant names twice on every line.

```rust
use syn::{Expr, ExprMethodCall};

fn example(expr: Expr) {
// Repetitive; recommend not doing this.
match expr {
    Expr::MethodCall(ExprMethodCall { method, args, .. }) => {
}
_ => {}
}
}
```

In general, the name to which a syntax tree enum variant is bound should
be a suitable name for the complete syntax tree enum type.

```rust
use syn::{Expr, ExprField};

fn example(discriminant: ExprField) {
// Binding is called `base` which is the name I would use if I were
// assigning `*discriminant.base` without an `if let`.
if let Expr::Tuple(base) = *discriminant.base {
}
}
```

A sign that you may not be choosing the right variable names is if you
see names getting repeated in your code, like accessing
`receiver.receiver` or `pat.pat` or `cond.cond`.

#### Variants

- **`Array`**

  A slice literal expression: `[a, b, c, d]`.

- **`Assign`**

  An assignment expression: `a = compute()`.

- **`Async`**

  An async block: `async { ... }`.

- **`Await`**

  An await expression: `fut.await`.

- **`Binary`**

  A binary operation: `a + b`, `a += b`.

- **`Block`**

  A blocked scope: `{ ... }`.

- **`Break`**

  A `break`, with an optional label to break and an optional
  expression.

- **`Call`**

  A function call expression: `invoke(a, b)`.

- **`Cast`**

  A cast expression: `foo as f64`.

- **`Closure`**

  A closure expression: `|a, b| a + b`.

- **`Const`**

  A const block: `const { ... }`.

- **`Continue`**

  A `continue`, with an optional label.

- **`Field`**

  Access of a named struct field (`obj.k`) or unnamed tuple struct
  field (`obj.0`).

- **`ForLoop`**

  A for loop: `for pat in expr { ... }`.

- **`Group`**

  An expression contained within invisible delimiters.
  
  This variant is important for faithfully representing the precedence
  of expressions and is related to `None`-delimited spans in a
  `TokenStream`.

- **`If`**

  An `if` expression with an optional `else` block: `if expr { ... }
  else { ... }`.
  
  The `else` branch expression may only be an `If` or `Block`
  expression, not any of the other types of expression.

- **`Index`**

  A square bracketed indexing expression: `vector[2]`.

- **`Infer`**

  The inferred value of a const generic argument, denoted `_`.

- **`Let`**

  A `let` guard: `let Some(x) = opt`.

- **`Lit`**

  A literal in place of an expression: `1`, `"foo"`.

- **`Loop`**

  Conditionless loop: `loop { ... }`.

- **`Macro`**

  A macro invocation expression: `format!("{}", q)`.

- **`Match`**

  A `match` expression: `match n { Some(n) => {}, None => {} }`.

- **`MethodCall`**

  A method call expression: `x.foo::<T>(a, b)`.

- **`Paren`**

  A parenthesized expression: `(a + b)`.

- **`Path`**

  A path like `core::mem::replace` possibly containing generic
  parameters and a qualified self-type.
  
  A plain identifier like `x` is a path of length 1.

- **`Range`**

  A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

- **`RawAddr`**

  Address-of operation: `&raw const place` or `&raw mut place`.

- **`Reference`**

  A referencing operation: `&a` or `&mut a`.

- **`Repeat`**

  An array literal constructed from one repeated element: `[0u8; N]`.

- **`Return`**

  A `return`, with an optional value to be returned.

- **`Struct`**

  A struct literal expression: `Point { x: 1, y: 1 }`.
  
  The `rest` provides the value of the remaining fields as in `S { a:
  1, b: 1, ..rest }`.

- **`Try`**

  A try-expression: `expr?`.

- **`TryBlock`**

  A try block: `try { ... }`.

- **`Tuple`**

  A tuple expression: `(a, b, c, d)`.

- **`Unary`**

  A unary operation: `!x`, `*x`.

- **`Unsafe`**

  An unsafe block: `unsafe { ... }`.

- **`Verbatim`**

  Tokens in expression position not interpreted by Syn.

- **`While`**

  A while loop: `while expr { ... }`.

- **`Yield`**

  A yield expression: `yield expr`.

#### Implementations

- <span id="expr-const-placeholder"></span>`const PLACEHOLDER: Self`

- <span id="expr-parse-without-eager-brace"></span>`fn parse_without_eager_brace(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`Expr`](#expr)

  An alternative to the primary `Expr::parse` parser (from the [`Parse`](../parse/index.md)

  trait) for ambiguous syntactic positions in which a trailing brace

  should not be taken as part of the expression.

  

  Rust grammar has an ambiguity where braces sometimes turn a path

  expression into a struct initialization and sometimes do not. In the

  following code, the expression `S {}` is one expression. Presumably

  there is an empty struct `struct S {}` defined somewhere which it is

  instantiating.

  

  ```rust

  struct S;

  impl core::ops::Deref for S {

      type Target = bool;

      fn deref(&self) -> &Self::Target {

          &true

      }

  }

  let _ = *S {};

  

  // parsed by rustc as: `*(S {})`

  ```

  

  We would want to parse the above using `Expr::parse` after the `=`

  token.

  

  But in the following, `S {}` is *not* a struct init expression.

  

  ```rust

  const S: &bool = &true;

  if *S {} {}

  

  // parsed by rustc as:

  //

  //    if (*S) {

  //        /* empty block */

  //    }

  //    {

  //        /* another empty block */

  //    }

  ```

  

  For that reason we would want to parse if-conditions using

  `Expr::parse_without_eager_brace` after the `if` token. Same for similar

  syntactic positions such as the condition expr after a `while` token or

  the expr at the top of a `match`.

  

  The Rust grammar's choices around which way this ambiguity is resolved

  at various syntactic positions is fairly arbitrary. Really either parse

  behavior could work in most positions, and language designers just

  decide each case based on which is more likely to be what the programmer

  had in mind most of the time.

  

  ```rust

  struct S;

  fn doc() -> S {

  if return S {} {}

  unreachable!()

  }

  

  // parsed by rustc as:

  //

  //    if (return (S {})) {

  //    }

  //

  // but could equally well have been this other arbitrary choice:

  //

  //    if (return S) {

  //    }

  //    {}

  ```

  

  Note the grammar ambiguity on trailing braces is distinct from

  precedence and is not captured by assigning a precedence level to the

  braced struct init expr in relation to other operators. This can be

  illustrated by `return 0..S {}` vs `match 0..S {}`. The former parses as

  `return (0..(S {}))` implying tighter precedence for struct init than

  `..`, while the latter parses as `match (0..S) {}` implying tighter

  precedence for `..` than struct init, a contradiction.

- <span id="expr-parse-with-earlier-boundary-rule"></span>`fn parse_with_earlier_boundary_rule(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`Expr`](#expr)

  An alternative to the primary `Expr::parse` parser (from the [`Parse`](../parse/index.md)

  trait) for syntactic positions in which expression boundaries are placed

  more eagerly than done by the typical expression grammar. This includes

  expressions at the head of a statement or in the right-hand side of a

  `match` arm.

  

  Compare the following cases:

  

  1.

    ```rust

    let result = ();

    let guard = false;

    let cond = true;

    let f = true;

    let g = f;

  

    let _ = match result {

        () if guard => if cond { f } else { g }

        () => false,

    };

    ```

  

  2.

    ```rust

    let cond = true;

    let f = ();

    let g = f;

  

    let _ = || {

        if cond { f } else { g }

        ()

    };

    ```

  

  3.

    ```rust

    let cond = true;

    let f = || ();

    let g = f;

  

    let _ = [if cond { f } else { g } ()];

    ```

  

  The same sequence of tokens `if cond { f } else { g } ()` appears in

  expression position 3 times. The first two syntactic positions use eager

  placement of expression boundaries, and parse as `Expr::If`, with the

  adjacent `()` becoming `Pat::Tuple` or `Expr::Tuple`. In contrast, the

  third case uses standard expression boundaries and parses as

  `Expr::Call`.

  

  As with `parse_without_eager_brace`, this ambiguity in the Rust

  grammar is independent of precedence.

- <span id="expr-peek"></span>`fn peek(input: ParseStream<'_>) -> bool` — [`ParseStream`](../parse/index.md#parsestream)

  Returns whether the next token in the parse stream is one that might

  possibly form the beginning of an expr.

  

  This classification is a load-bearing part of the grammar of some Rust

  expressions, notably `return` and `break`. For example `return < …` will

  never parse `<` as a binary operator regardless of what comes after,

  because `<` is a legal starting token for an expression and so it's

  required to be continued as a return value, such as `return <Struct as

  Trait>::CONST`. Meanwhile `return > …` treats the `>` as a binary

  operator because it cannot be a starting token for any Rust expression.

- <span id="expr-replace-attrs"></span>`fn replace_attrs(&mut self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](../attr/index.md#attribute)

#### Trait Implementations

##### `impl Clone for crate::Expr`

- <span id="crateexpr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Expr`

- <span id="crateexpr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Expr`

##### `impl Hash for crate::Expr`

- <span id="crateexpr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::Expr`

- <span id="crateexprexpr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Expr`

- <span id="crateexpr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Expr`

##### `impl Spanned for Expr`

- <span id="expr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Expr`

- <span id="expr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `Member`

```rust
enum Member {
    Named(crate::ident::Ident),
    Unnamed(Index),
}
```

A struct or tuple struct field accessed in a struct literal or field
expression.

#### Variants

- **`Named`**

  A named field like `self.x`.

- **`Unnamed`**

  An unnamed field like `self.0`.

#### Implementations

- <span id="member-is-named"></span>`fn is_named(&self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::Member`

- <span id="cratemember-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Member`

- <span id="cratemember-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Member`

##### `impl Hash for Member`

- <span id="member-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IdentFragment for Member`

- <span id="member-identfragment-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="member-identfragment-span"></span>`fn span(&self) -> Option<Span>`

##### `impl Parse for crate::expr::Member`

- <span id="crateexprmember-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Member`

- <span id="member-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Member`

##### `impl Spanned for Member`

- <span id="member-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::Member`

- <span id="crateexprmember-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `RangeLimits`

```rust
enum RangeLimits {
    HalfOpen(token::DotDot),
    Closed(token::DotDotEq),
}
```

Limit types of a range, inclusive or exclusive.

#### Variants

- **`HalfOpen`**

  Inclusive at the beginning, exclusive at the end.

- **`Closed`**

  Inclusive at the beginning and end.

#### Implementations

- <span id="crateexprrangelimits-parse-obsolete"></span>`fn parse_obsolete(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Clone for crate::RangeLimits`

- <span id="craterangelimits-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for crate::RangeLimits`

##### `impl Debug for crate::RangeLimits`

- <span id="craterangelimits-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::RangeLimits`

##### `impl Hash for crate::RangeLimits`

- <span id="craterangelimits-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::RangeLimits`

- <span id="crateexprrangelimits-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::RangeLimits`

- <span id="craterangelimits-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for RangeLimits`

##### `impl Spanned for RangeLimits`

- <span id="rangelimits-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::RangeLimits`

- <span id="crateexprrangelimits-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PointerMutability`

```rust
enum PointerMutability {
    Const(token::Const),
    Mut(token::Mut),
}
```

Mutability of a raw pointer (`*const T`, `*mut T`), in which non-mutable
isn't the implicit default.

#### Trait Implementations

##### `impl Clone for crate::PointerMutability`

- <span id="cratepointermutability-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PointerMutability`

- <span id="cratepointermutability-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PointerMutability`

##### `impl Hash for crate::PointerMutability`

- <span id="cratepointermutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::expr::PointerMutability`

- <span id="crateexprpointermutability-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::PointerMutability`

- <span id="cratepointermutability-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PointerMutability`

##### `impl Spanned for PointerMutability`

- <span id="pointermutability-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::expr::PointerMutability`

- <span id="crateexprpointermutability-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

