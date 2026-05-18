*[heapless](../../index.md) / [string](../index.md) / [storage](index.md)*

---

# Module `storage`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StringStorage`](#stringstorage) | trait | Trait defining how data for a String is stored. |
| [`StringStorageSealed`](#stringstoragesealed) | trait |  |

## Traits

### `StringStorage`

```rust
trait StringStorage: StringStorageSealed { ... }
```

Trait defining how data for a String is stored.

There's two implementations available:

- [`OwnedStorage`](../../linear_map/index.md): stores the data in an array whose size is known at compile time.
- [`ViewStorage`](../index.md): stores the data in an unsized slice

This allows [`String`](../index.md) to be generic over either sized or unsized storage. The [`string`](super)
module contains a [`StringInner`](../index.md) struct that's generic on [`StringStorage`](#stringstorage),
and two type aliases for convenience:

- [`String<N>`](crate::string::String) = `StringInner<OwnedStorage<u8, N>>`
- [`StringView<T>`](crate::string::StringView) = `StringInner<ViewStorage<u8>>`

`String` can be unsized into `StrinsgView`, either by unsizing coercions such as `&mut String -> &mut StringView` or
`Box<String> -> Box<StringView>`, or explicitly with [`.as_view()`](crate::string::String::as_view) or [`.as_mut_view()`](crate::string::String::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.





#### Implementors

- [`OwnedVecStorage`](../../vec/storage/index.md#ownedvecstorage)
- [`ViewVecStorage`](../../vec/storage/index.md#viewvecstorage)

### `StringStorageSealed`

```rust
trait StringStorageSealed: VecStorage<u8> { ... }
```

#### Required Methods

- `fn as_string_view<LenT: LenType>(this: &StringInner<LenT, Self>) -> &StringView<LenT>`

- `fn as_string_mut_view<LenT: LenType>(this: &mut StringInner<LenT, Self>) -> &mut StringView<LenT>`

#### Implementors

- [`OwnedVecStorage`](../../vec/storage/index.md#ownedvecstorage)
- [`ViewVecStorage`](../../vec/storage/index.md#viewvecstorage)

