*[sel4_config_macros](../../index.md) / [generic](../index.md) / [condition](index.md)*

---

# Module `condition`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EvalError`](#evalerror) | struct |  |
| [`ConfigurationForEval`](#configurationforeval) | struct |  |
| [`Condition`](#condition) | enum |  |
| [`err`](#err) | fn |  |

## Structs

### `EvalError`

```rust
struct EvalError {
    span: proc_macro2::Span,
    message: String,
}
```

#### Implementations

- <span id="evalerror-new"></span>`fn new(span: Span, message: String) -> Self`

- <span id="evalerror-render"></span>`fn render(&self) -> TokenStream`

### `ConfigurationForEval<'a>`

```rust
struct ConfigurationForEval<'a>(&'a sel4_config_types::Configuration);
```

#### Implementations

- <span id="configurationforeval-lookup-key"></span>`fn lookup_key(&self, k: &syn::Ident) -> Result<&Value, EvalError>` — [`EvalError`](#evalerror)

- <span id="configurationforeval-eval"></span>`fn eval(&self, cond: &Condition) -> Result<bool, EvalError>` — [`Condition`](#condition), [`EvalError`](#evalerror)

## Enums

### `Condition`

```rust
enum Condition {
    Key(syn::Ident),
    KeyValue(syn::Ident, syn::LitStr),
    Not(Box<Condition>),
    All(Vec<Condition>),
    Any(Vec<Condition>),
}
```

#### Implementations

- <span id="condition-eval"></span>`fn eval(&self, config: &Configuration) -> Result<bool, EvalError>` — [`EvalError`](#evalerror)

#### Trait Implementations

##### `impl Parse for Condition`

- <span id="condition-parse"></span>`fn parse(input: ParseStream<'_>) -> ParseResult<Self>`

## Functions

### `err`

```rust
fn err<T, U: ToString>(node: impl Spanned, message: U) -> Result<T, EvalError>
```

