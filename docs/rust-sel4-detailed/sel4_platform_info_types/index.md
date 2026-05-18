# Crate `sel4_platform_info_types`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PlatformInfo`](#platforminfo) | struct |  |

## Structs

### `PlatformInfo<'a, T>`

```rust
struct PlatformInfo<'a, T> {
    pub memory: &'a [core::ops::Range<T>],
    pub devices: &'a [core::ops::Range<T>],
}
```

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for PlatformInfo<'a, T>`

- <span id="platforminfo-clone"></span>`fn clone(&self) -> PlatformInfo<'a, T>` — [`PlatformInfo`](#platforminfo)

##### `impl<T: fmt::Debug> Debug for PlatformInfo<'a, T>`

- <span id="platforminfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

