*[bytemuck](../index.md) / [zeroable_in_option](index.md)*

---

# Module `zeroable_in_option`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ZeroableInOption`](#zeroableinoption) | trait | Trait for types which are [Zeroable](Zeroable) when wrapped in [Option](core::option::Option). |
| [`impl_for_fn!`](#impl-for-fn) | macro |  |

## Traits

### `ZeroableInOption`

```rust
trait ZeroableInOption: Sized { ... }
```

Trait for types which are [Zeroable](Zeroable) when wrapped in
[Option](core::option::Option).

## Safety

* `Option<YourType>` must uphold the same invariants as
  [Zeroable](Zeroable).

#### Implementors

- `&T`
- `&mut T`
- `NonNull<T>`
- `NonZeroI128`
- `NonZeroI16`
- `NonZeroI32`
- `NonZeroI64`
- `NonZeroI8`
- `NonZeroIsize`
- `NonZeroU128`
- `NonZeroU16`
- `NonZeroU32`
- `NonZeroU64`
- `NonZeroU8`
- `NonZeroUsize`
- `fn() -> R`
- `fn(A) -> R`
- `fn(A, B) -> R`
- `fn(A, B, C) -> R`
- `fn(A, B, C, D) -> R`
- `fn(A, B, C, D, E) -> R`
- `fn(A, B, C, D, E, F) -> R`
- `fn(A, B, C, D, E, F, G) -> R`
- `fn(A, B, C, D, E, F, G, H) -> R`
- `fn(A, B, C, D, E, F, G, H, I) -> R`
- `fn(A, B, C, D, E, F, G, H, I, J) -> R`
- `fn(A, B, C, D, E, F, G, H, I, J, K) -> R`
- `fn(A, B, C, D, E, F, G, H, I, J, K, L) -> R`
- `fn(A, B, C, D, E, F, G, H, I, J, K, L, M) -> R`

## Macros

### `impl_for_fn!`

