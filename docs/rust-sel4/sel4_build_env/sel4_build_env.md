**sel4_build_env**

# Module: sel4_build_env

## Contents

**Functions**

- [`find_in_libsel4_include_dirs`](#find_in_libsel4_include_dirs)
- [`get_libsel4_include_dirs`](#get_libsel4_include_dirs)
- [`get_sel4_prefix`](#get_sel4_prefix)
- [`get_with_sel4_prefix_relative_fallback`](#get_with_sel4_prefix_relative_fallback)
- [`try_find_in_libsel4_include_dirs`](#try_find_in_libsel4_include_dirs)
- [`try_get_with_sel4_prefix_relative_fallback`](#try_get_with_sel4_prefix_relative_fallback)

**Constants**

- [`SEL4_INCLUDE_DIRS_ENV`](#sel4_include_dirs_env)
- [`SEL4_PREFIX_ENV`](#sel4_prefix_env)

---

## sel4_build_env::SEL4_INCLUDE_DIRS_ENV

*Constant*: `&str`



## sel4_build_env::SEL4_PREFIX_ENV

*Constant*: `&str`



## sel4_build_env::find_in_libsel4_include_dirs

*Function*

```rust
fn find_in_libsel4_include_dirs<impl AsRef<Path>>(relative_path: impl Trait) -> std::path::PathBuf
```



## sel4_build_env::get_libsel4_include_dirs

*Function*

```rust
fn get_libsel4_include_dirs() -> impl Trait
```



## sel4_build_env::get_sel4_prefix

*Function*

```rust
fn get_sel4_prefix() -> Option<std::path::PathBuf>
```



## sel4_build_env::get_with_sel4_prefix_relative_fallback

*Function*

```rust
fn get_with_sel4_prefix_relative_fallback<impl AsRef<Path>>(var: &str, relative_path_from_fallback: impl Trait) -> std::path::PathBuf
```



## sel4_build_env::try_find_in_libsel4_include_dirs

*Function*

```rust
fn try_find_in_libsel4_include_dirs<impl AsRef<Path>>(relative_path: impl Trait) -> Option<std::path::PathBuf>
```



## sel4_build_env::try_get_with_sel4_prefix_relative_fallback

*Function*

```rust
fn try_get_with_sel4_prefix_relative_fallback<impl AsRef<Path>>(var: &str, relative_path_from_fallback: impl Trait) -> Option<std::path::PathBuf>
```



