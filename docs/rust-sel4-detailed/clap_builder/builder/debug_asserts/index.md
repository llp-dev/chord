*[clap_builder](../../index.md) / [builder](../index.md) / [debug_asserts](index.md)*

---

# Module `debug_asserts`

## Contents

- [Enums](#enums)
  - [`Flag`](#flag)
- [Functions](#functions)
  - [`assert_app`](#assert-app)
  - [`duplicate_tip`](#duplicate-tip)
  - [`detect_duplicate_flags`](#detect-duplicate-flags)
  - [`find_duplicates`](#find-duplicates)
  - [`assert_app_flags`](#assert-app-flags)
  - [`_verify_positionals`](#verify-positionals)
  - [`assert_arg`](#assert-arg)
  - [`assert_arg_flags`](#assert-arg-flags)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Flag`](#flag) | enum |  |
| [`assert_app`](#assert-app) | fn |  |
| [`duplicate_tip`](#duplicate-tip) | fn |  |
| [`detect_duplicate_flags`](#detect-duplicate-flags) | fn |  |
| [`find_duplicates`](#find-duplicates) | fn | Find duplicates in a sorted array. |
| [`assert_app_flags`](#assert-app-flags) | fn |  |
| [`_verify_positionals`](#verify-positionals) | fn |  |
| [`assert_arg`](#assert-arg) | fn |  |
| [`assert_arg_flags`](#assert-arg-flags) | fn |  |

## Enums

### `Flag<'a>`

```rust
enum Flag<'a> {
    Command(String, &'a str),
    Arg(String, &'a str),
}
```

#### Trait Implementations

##### `impl Eq for Flag<'a>`

##### `impl Ord for Flag<'_>`

- <span id="flag-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl PartialEq for Flag<'_>`

- <span id="flag-partialeq-eq"></span>`fn eq(&self, other: &Flag<'_>) -> bool` — [`Flag`](#flag)

##### `impl PartialOrd for Flag<'_>`

- <span id="flag-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Flag<'_>) -> Option<Ordering>` — [`Flag`](#flag)

## Functions

### `assert_app`

```rust
fn assert_app(cmd: &crate::Command)
```

### `duplicate_tip`

```rust
fn duplicate_tip(cmd: &crate::Command, first: &crate::Arg, second: &crate::Arg) -> &'static str
```

### `detect_duplicate_flags`

```rust
fn detect_duplicate_flags(flags: &[Flag<'_>], short_or_long: &str)
```

### `find_duplicates`

```rust
fn find_duplicates<T: PartialEq>(slice: &[T]) -> impl Iterator<Item = (&T, &T)>
```

Find duplicates in a sorted array.

The algorithm is simple: the array is sorted, duplicates
must be placed next to each other, we can check only adjacent elements.

### `assert_app_flags`

```rust
fn assert_app_flags(cmd: &crate::Command)
```

### `_verify_positionals`

```rust
fn _verify_positionals(cmd: &crate::Command) -> bool
```

### `assert_arg`

```rust
fn assert_arg(arg: &crate::Arg)
```

### `assert_arg_flags`

```rust
fn assert_arg_flags(arg: &crate::Arg)
```

