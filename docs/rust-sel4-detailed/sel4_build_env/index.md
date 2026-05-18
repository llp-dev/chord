# Crate `sel4_build_env`

## Contents

- [Functions](#functions)
  - [`get_asserting_valid_unicode`](#get-asserting-valid-unicode)
  - [`get_with_sel4_prefix_relative_fallback`](#get-with-sel4-prefix-relative-fallback)
  - [`try_get_with_sel4_prefix_relative_fallback`](#try-get-with-sel4-prefix-relative-fallback)
  - [`get_sel4_prefix`](#get-sel4-prefix)
  - [`get_libsel4_include_dirs`](#get-libsel4-include-dirs)
  - [`try_find_in_libsel4_include_dirs`](#try-find-in-libsel4-include-dirs)
  - [`find_in_libsel4_include_dirs`](#find-in-libsel4-include-dirs)
- [Constants](#constants)
  - [`SEL4_PREFIX_ENV`](#sel4-prefix-env)
  - [`SEL4_INCLUDE_DIRS_ENV`](#sel4-include-dirs-env)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`get_asserting_valid_unicode`](#get-asserting-valid-unicode) | fn |  |
| [`get_with_sel4_prefix_relative_fallback`](#get-with-sel4-prefix-relative-fallback) | fn |  |
| [`try_get_with_sel4_prefix_relative_fallback`](#try-get-with-sel4-prefix-relative-fallback) | fn |  |
| [`get_sel4_prefix`](#get-sel4-prefix) | fn |  |
| [`get_libsel4_include_dirs`](#get-libsel4-include-dirs) | fn |  |
| [`try_find_in_libsel4_include_dirs`](#try-find-in-libsel4-include-dirs) | fn |  |
| [`find_in_libsel4_include_dirs`](#find-in-libsel4-include-dirs) | fn |  |
| [`SEL4_PREFIX_ENV`](#sel4-prefix-env) | const |  |
| [`SEL4_INCLUDE_DIRS_ENV`](#sel4-include-dirs-env) | const |  |

## Functions

### `get_asserting_valid_unicode`

```rust
fn get_asserting_valid_unicode(var: &str) -> Option<String>
```

### `get_with_sel4_prefix_relative_fallback`

```rust
fn get_with_sel4_prefix_relative_fallback(var: &str, relative_path_from_fallback: impl AsRef<std::path::Path>) -> std::path::PathBuf
```

### `try_get_with_sel4_prefix_relative_fallback`

```rust
fn try_get_with_sel4_prefix_relative_fallback(var: &str, relative_path_from_fallback: impl AsRef<std::path::Path>) -> Option<std::path::PathBuf>
```

### `get_sel4_prefix`

```rust
fn get_sel4_prefix() -> Option<std::path::PathBuf>
```

### `get_libsel4_include_dirs`

```rust
fn get_libsel4_include_dirs() -> impl Iterator<Item = std::path::PathBuf>
```

### `try_find_in_libsel4_include_dirs`

```rust
fn try_find_in_libsel4_include_dirs(relative_path: impl AsRef<std::path::Path>) -> Option<std::path::PathBuf>
```

### `find_in_libsel4_include_dirs`

```rust
fn find_in_libsel4_include_dirs(relative_path: impl AsRef<std::path::Path>) -> std::path::PathBuf
```

## Constants

### `SEL4_PREFIX_ENV`
```rust
const SEL4_PREFIX_ENV: &str;
```

### `SEL4_INCLUDE_DIRS_ENV`
```rust
const SEL4_INCLUDE_DIRS_ENV: &str;
```

