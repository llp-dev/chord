**thiserror_impl > scan_expr**

# Module: scan_expr

## Contents

**Enums**

- [`Action`](#action)
- [`Input`](#input)

**Functions**

- [`scan_expr`](#scan_expr)

**Statics**

- [`ASYNC`](#async)
- [`BLOCK`](#block)
- [`BREAK_LABEL`](#break_label)
- [`BREAK_VALUE`](#break_value)
- [`CLOSURE`](#closure)
- [`CLOSURE_ARGS`](#closure_args)
- [`CLOSURE_RET`](#closure_ret)
- [`CONST`](#const)
- [`CONTINUE`](#continue)
- [`DOT`](#dot)
- [`FOR`](#for)
- [`IF_ELSE`](#if_else)
- [`IF_THEN`](#if_then)
- [`INIT`](#init)
- [`METHOD`](#method)
- [`PATH`](#path)
- [`PATTERN`](#pattern)
- [`POSTFIX`](#postfix)
- [`RANGE`](#range)
- [`RAW`](#raw)
- [`REFERENCE`](#reference)
- [`RETURN`](#return)

---

## thiserror_impl::scan_expr::ASYNC

*Static*

```rust
static ASYNC: [(Input, Action); 3]
```



## thiserror_impl::scan_expr::Action

*Enum*

**Variants:**
- `SetState(&'static [(Input, Action)])`
- `IncDepth`
- `DecDepth`
- `Finish`



## thiserror_impl::scan_expr::BLOCK

*Static*

```rust
static BLOCK: [(Input, Action); 1]
```



## thiserror_impl::scan_expr::BREAK_LABEL

*Static*

```rust
static BREAK_LABEL: [(Input, Action); 2]
```



## thiserror_impl::scan_expr::BREAK_VALUE

*Static*

```rust
static BREAK_VALUE: [(Input, Action); 3]
```



## thiserror_impl::scan_expr::CLOSURE

*Static*

```rust
static CLOSURE: [(Input, Action); 6]
```



## thiserror_impl::scan_expr::CLOSURE_ARGS

*Static*

```rust
static CLOSURE_ARGS: [(Input, Action); 2]
```



## thiserror_impl::scan_expr::CLOSURE_RET

*Static*

```rust
static CLOSURE_RET: [(Input, Action); 2]
```



## thiserror_impl::scan_expr::CONST

*Static*

```rust
static CONST: [(Input, Action); 2]
```



## thiserror_impl::scan_expr::CONTINUE

*Static*

```rust
static CONTINUE: [(Input, Action); 2]
```



## thiserror_impl::scan_expr::DOT

*Static*

```rust
static DOT: [(Input, Action); 3]
```



## thiserror_impl::scan_expr::FOR

*Static*

```rust
static FOR: [(Input, Action); 2]
```



## thiserror_impl::scan_expr::IF_ELSE

*Static*

```rust
static IF_ELSE: [(Input, Action); 2]
```



## thiserror_impl::scan_expr::IF_THEN

*Static*

```rust
static IF_THEN: [(Input, Action); 2]
```



## thiserror_impl::scan_expr::INIT

*Static*

```rust
static INIT: [(Input, Action); 28]
```



## thiserror_impl::scan_expr::Input

*Enum*

**Variants:**
- `Keyword(&'static str)`
- `Punct(&'static str)`
- `ConsumeAny`
- `ConsumeBinOp`
- `ConsumeBrace`
- `ConsumeDelimiter`
- `ConsumeIdent`
- `ConsumeLifetime`
- `ConsumeLiteral`
- `ConsumeNestedBrace`
- `ExpectPath`
- `ExpectTurbofish`
- `ExpectType`
- `CanBeginExpr`
- `Otherwise`
- `Empty`



## thiserror_impl::scan_expr::METHOD

*Static*

```rust
static METHOD: [(Input, Action); 1]
```



## thiserror_impl::scan_expr::PATH

*Static*

```rust
static PATH: [(Input, Action); 4]
```



## thiserror_impl::scan_expr::PATTERN

*Static*

```rust
static PATTERN: [(Input, Action); 15]
```



## thiserror_impl::scan_expr::POSTFIX

*Static*

```rust
static POSTFIX: [(Input, Action); 10]
```



## thiserror_impl::scan_expr::RANGE

*Static*

```rust
static RANGE: [(Input, Action); 6]
```



## thiserror_impl::scan_expr::RAW

*Static*

```rust
static RAW: [(Input, Action); 3]
```



## thiserror_impl::scan_expr::REFERENCE

*Static*

```rust
static REFERENCE: [(Input, Action); 3]
```



## thiserror_impl::scan_expr::RETURN

*Static*

```rust
static RETURN: [(Input, Action); 2]
```



## thiserror_impl::scan_expr::scan_expr

*Function*

```rust
fn scan_expr(input: syn::parse::ParseStream) -> syn::parse::Result<()>
```



