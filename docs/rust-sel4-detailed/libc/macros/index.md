*[libc](../index.md) / [macros](index.md)*

---

# Module `macros`

## Contents

- [Macros](#macros)
  - [`cfg_if!`](#cfg-if)
  - [`prelude!`](#prelude)
  - [`s!`](#s)
  - [`s_paren!`](#s-paren)
  - [`s_no_extra_traits!`](#s-no-extra-traits)
  - [`extern_ty!`](#extern-ty)
  - [`e!`](#e)
  - [`c_enum!`](#c-enum)
  - [`f!`](#f)
  - [`safe_f!`](#safe-f)
  - [`deprecated_mach!`](#deprecated-mach)
  - [`offset_of!`](#offset-of)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cfg_if!`](#cfg-if) | macro | A macro for defining #[cfg] if-else statements. |
| [`prelude!`](#prelude) | macro | Create an internal crate prelude with `core` reexports and common types. |
| [`s!`](#s) | macro | Implement `Clone`, `Copy`, and `Debug` for one or more structs, as well as `PartialEq`, `Eq`, and `Hash` if the `extra_traits` feature is enabled. |
| [`s_paren!`](#s-paren) | macro | Implement `Clone`, `Copy`, and `Debug` for a tuple struct, as well as `PartialEq`, `Eq`, and `Hash` if the `extra_traits` feature is enabled. |
| [`s_no_extra_traits!`](#s-no-extra-traits) | macro | Implement `Clone`, `Copy`, and `Debug` for one or more structs/unions, but exclude `PartialEq`, `Eq`, and `Hash`. |
| [`extern_ty!`](#extern-ty) | macro | Create an uninhabited type that can't be constructed. |
| [`e!`](#e) | macro | Implement `Clone` and `Copy` for an enum, as well as `Debug`, `Eq`, `Hash`, and `PartialEq` if the `extra_traits` feature is enabled. |
| [`c_enum!`](#c-enum) | macro | Represent a C enum as Rust constants and a type. |
| [`f!`](#f) | macro | Define a `unsafe` function. |
| [`safe_f!`](#safe-f) | macro | Define a safe function. |
| [`deprecated_mach!`](#deprecated-mach) | macro |  |
| [`offset_of!`](#offset-of) | macro | Polyfill for std's `offset_of`. |

## Macros

### `cfg_if!`

A macro for defining #[`cfg`](../../rustversion/index.md) if-else statements.

This is similar to the `if/elif` C preprocessor macro by allowing definition
of a cascade of `#[cfg]` cases, emitting the implementation which matches
first.

This allows you to conveniently provide a long list #[`cfg`](../../rustversion/index.md)'d blocks of code
without having to rewrite each clause multiple times.

### `prelude!`

Create an internal crate prelude with `core` reexports and common types.

### `s!`

Implement `Clone`, `Copy`, and `Debug` for one or more structs, as well as `PartialEq`, `Eq`,
and `Hash` if the `extra_traits` feature is enabled.

Also mark the type with `repr(C)`.

Use [`s_no_extra_traits`](#s-no-extra-traits) for structs where the `extra_traits` feature does not
make sense, and for unions.

### `s_paren!`

Implement `Clone`, `Copy`, and `Debug` for a tuple struct, as well as `PartialEq`, `Eq`,
and `Hash` if the `extra_traits` feature is enabled.

Unlike `s!`, this does *not* mark the type with `repr(C)`. Users should provide their own
`repr` attribute via `$attr` as necessary.

### `s_no_extra_traits!`

Implement `Clone`, `Copy`, and `Debug` for one or more structs/unions, but exclude `PartialEq`,
`Eq`, and `Hash`.

Also mark the type with `repr(C)`.

Most structs will prefer to use [`s`](#s).

### `extern_ty!`

Create an uninhabited type that can't be constructed. It implements `Debug`, `Clone`,
and `Copy`, but these aren't meaningful for extern types so they should eventually
be removed.

Really what we want here is something that also can't be named without indirection (in
ADTs or function signatures), but this doesn't exist.

### `e!`

Implement `Clone` and `Copy` for an enum, as well as `Debug`, `Eq`, `Hash`, and
`PartialEq` if the `extra_traits` feature is enabled.

### `c_enum!`

Represent a C enum as Rust constants and a type.

C enums can't soundly be mapped to Rust enums since C enums are allowed to have duplicates or
unlisted values, but this is UB in Rust. This enum doesn't implement any traits, its main
purpose is to calculate the correct enum values.

Use the magic name `#anon` if the C enum doesn't create a type.

See <https://github.com/rust-lang/libc/issues/4419> for more.

### `f!`

Define a `unsafe` function.

### `safe_f!`

Define a safe function.

### `deprecated_mach!`

### `offset_of!`

Polyfill for std's `offset_of`.

