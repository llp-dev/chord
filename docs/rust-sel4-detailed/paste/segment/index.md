*[paste](../index.md) / [segment](index.md)*

---

# Module `segment`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LitStr`](#litstr) | struct |  |
| [`Colon`](#colon) | struct |  |
| [`Segment`](#segment) | enum |  |
| [`parse`](#parse) | fn |  |
| [`paste`](#paste) | fn |  |

## Structs

### `LitStr`

```rust
struct LitStr {
    pub value: String,
    pub span: proc_macro::Span,
}
```

### `Colon`

```rust
struct Colon {
    pub span: proc_macro::Span,
}
```

## Enums

### `Segment`

```rust
enum Segment {
    String(LitStr),
    Apostrophe(proc_macro::Span),
    Env(LitStr),
    Modifier(Colon, proc_macro::Ident),
}
```

## Functions

### `parse`

```rust
fn parse(tokens: &mut std::iter::Peekable<token_stream::IntoIter>) -> std::result::Result<Vec<Segment>, Error>
```

### `paste`

```rust
fn paste(segments: &[Segment]) -> std::result::Result<String, Error>
```

