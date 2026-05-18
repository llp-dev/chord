*[libm](../../../../index.md) / [math](../../../index.md) / [arch](../../index.md) / [x86](../index.md) / [detect](index.md)*

---

# Module `detect`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cpu_flags`](#cpu-flags) | mod | CPU features that get cached (doesn't correlate to anything on the CPU). |
| [`get_cpu_features`](#get-cpu-features) | fn | Get CPU features, loading from a cache if available. |
| [`load_x86_features`](#load-x86-features) | fn | Read from cpuid and translate to a `Flags` instance, using `cpu_flags`. |

## Modules

- [`cpu_flags`](cpu_flags/index.md) — CPU features that get cached (doesn't correlate to anything on the CPU).

## Functions

### `get_cpu_features`

```rust
fn get_cpu_features() -> crate::support::feature_detect::Flags
```

Get CPU features, loading from a cache if available.

### `load_x86_features`

```rust
fn load_x86_features() -> crate::support::feature_detect::Flags
```

Read from cpuid and translate to a `Flags` instance, using `cpu_flags`.

Implementation is taken from [std-detect][std-detect].


