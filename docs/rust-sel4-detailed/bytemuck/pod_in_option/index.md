*[bytemuck](../index.md) / [pod_in_option](index.md)*

---

# Module `pod_in_option`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PodInOption`](#podinoption) | trait | Trait for types which are [Pod](Pod) when wrapped in [Option](core::option::Option). |

## Traits

### `PodInOption`

```rust
trait PodInOption: ZeroableInOption + Copy + 'static { ... }
```

Trait for types which are [Pod](Pod) when wrapped in
[Option](core::option::Option).

## Safety

* `Option<T>` must uphold the same invariants as [Pod](Pod).
* **Reminder:** pointers are **not** pod! **Do not** mix this trait with a
  newtype over [NonNull](core::ptr::NonNull).

#### Implementors

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

