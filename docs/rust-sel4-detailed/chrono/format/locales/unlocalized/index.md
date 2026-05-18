*[chrono](../../../index.md) / [format](../../index.md) / [locales](../index.md) / [unlocalized](index.md)*

---

# Module `unlocalized`

## Contents

- [Structs](#structs)
  - [`Locale`](#locale)
- [Functions](#functions)
  - [`default_locale`](#default-locale)
  - [`short_months`](#short-months)
  - [`long_months`](#long-months)
  - [`short_weekdays`](#short-weekdays)
  - [`long_weekdays`](#long-weekdays)
  - [`am_pm`](#am-pm)
  - [`decimal_point`](#decimal-point)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Locale`](#locale) | struct |  |
| [`default_locale`](#default-locale) | fn |  |
| [`short_months`](#short-months) | fn |  |
| [`long_months`](#long-months) | fn |  |
| [`short_weekdays`](#short-weekdays) | fn |  |
| [`long_weekdays`](#long-weekdays) | fn |  |
| [`am_pm`](#am-pm) | fn |  |
| [`decimal_point`](#decimal-point) | fn |  |

## Structs

### `Locale`

```rust
struct Locale;
```

#### Trait Implementations

##### `impl Clone for Locale`

- <span id="locale-clone"></span>`fn clone(&self) -> Locale` — [`Locale`](#locale)

##### `impl Copy for Locale`

##### `impl Debug for Locale`

- <span id="locale-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `default_locale`

```rust
const fn default_locale() -> Locale
```

### `short_months`

```rust
const fn short_months(_locale: Locale) -> &'static [&'static str]
```

### `long_months`

```rust
const fn long_months(_locale: Locale) -> &'static [&'static str]
```

### `short_weekdays`

```rust
const fn short_weekdays(_locale: Locale) -> &'static [&'static str]
```

### `long_weekdays`

```rust
const fn long_weekdays(_locale: Locale) -> &'static [&'static str]
```

### `am_pm`

```rust
const fn am_pm(_locale: Locale) -> &'static [&'static str]
```

### `decimal_point`

```rust
const fn decimal_point(_locale: Locale) -> &'static str
```

