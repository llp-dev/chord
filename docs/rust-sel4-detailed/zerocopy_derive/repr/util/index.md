*[zerocopy_derive](../../index.md) / [repr](../index.md) / [util](index.md)*

---

# Module `util`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Spanned`](#spanned) | struct | A value with an associated span. |
| [`Inhabited`](#inhabited) | trait |  |
| [`With`](#with) | trait |  |

## Structs

### `Spanned<T>`

```rust
struct Spanned<T> {
    t: T,
    span: proc_macro2::Span,
}
```

A value with an associated span.

#### Implementations

- <span id="spanned-new"></span>`fn new(t: T, span: Span) -> Spanned<T>` — [`Spanned`](#spanned)

- <span id="spanned-from"></span>`fn from<U>(s: Spanned<U>) -> Spanned<T>` — [`Spanned`](#spanned)

- <span id="spanned-try-from"></span>`fn try_from<E, U>(u: Spanned<U>) -> Result<Spanned<T>, FromRawReprError<Spanned<E>>>` — [`Spanned`](#spanned), [`FromRawReprError`](../index.md#fromrawreprerror)

  Delegates to `T: TryFrom`, preserving span information in both the

  success and error cases.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Spanned<T>`

- <span id="spanned-clone"></span>`fn clone(&self) -> Spanned<T>` — [`Spanned`](#spanned)

##### `impl<T: marker::Copy> Copy for Spanned<T>`

##### `impl<T> Spanned for Spanned<T>`

- <span id="spanned-spanned-span"></span>`fn span(&self) -> Span`

##### `impl<Prim: With<PrimitiveRepr> + Copy> ToTokens for Spanned<CompoundRepr<Prim>>`

- <span id="spanned-totokens-to-tokens"></span>`fn to_tokens(&self, ts: &mut TokenStream)`

## Traits

### `Inhabited`

```rust
trait Inhabited { ... }
```

#### Implementors

- [`PrimitiveRepr`](../index.md#primitiverepr)
- `core::num::NonZeroU32`

### `With<T>`

```rust
trait With<T> { ... }
```

#### Required Methods

- `fn with<O, F: FnOnce(T) -> O>(self, f: F) -> O`

- `fn try_with_or<E, F: FnOnce() -> Result<T, E>>(f: F, err: E) -> Result<Self, E>`

#### Implementors

- `T`
- `core::convert::Infallible`

