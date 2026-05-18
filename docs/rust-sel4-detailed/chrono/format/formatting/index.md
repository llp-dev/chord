*[chrono](../../index.md) / [format](../index.md) / [formatting](index.md)*

---

# Module `formatting`

Date and time formatting routines.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SecondsFormat`](#secondsformat) | enum | Specific formatting options for seconds. |
| [`write_rfc3339`](#write-rfc3339) | fn | Writes the date, time and offset to the string. |
| [`write_hundreds`](#write-hundreds) | fn | Equivalent to `{:02}` formatting for n < 100. |

## Enums

### `SecondsFormat`

```rust
enum SecondsFormat {
    Secs,
    Millis,
    Micros,
    Nanos,
    AutoSi,
}
```

Specific formatting options for seconds. This may be extended in the
future, so exhaustive matching in external code is not recommended.

See the `TimeZone::to_rfc3339_opts` function for usage.

#### Variants

- **`Secs`**

  Format whole seconds only, with no decimal point nor subseconds.

- **`Millis`**

  Use fixed 3 subsecond digits. This corresponds to [Fixed::Nanosecond3].

- **`Micros`**

  Use fixed 6 subsecond digits. This corresponds to [Fixed::Nanosecond6].

- **`Nanos`**

  Use fixed 9 subsecond digits. This corresponds to [Fixed::Nanosecond9].

- **`AutoSi`**

  Automatically select one of `Secs`, `Millis`, `Micros`, or `Nanos` to display all available
  non-zero sub-second digits.  This corresponds to [Fixed::Nanosecond].

#### Trait Implementations

##### `impl Clone for SecondsFormat`

- <span id="secondsformat-clone"></span>`fn clone(&self) -> SecondsFormat` — [`SecondsFormat`](#secondsformat)

##### `impl Copy for SecondsFormat`

##### `impl Debug for SecondsFormat`

- <span id="secondsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SecondsFormat`

##### `impl Hash for SecondsFormat`

- <span id="secondsformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SecondsFormat`

- <span id="secondsformat-partialeq-eq"></span>`fn eq(&self, other: &SecondsFormat) -> bool` — [`SecondsFormat`](#secondsformat)

##### `impl StructuralPartialEq for SecondsFormat`

## Functions

### `write_rfc3339`

```rust
fn write_rfc3339(w: &mut impl Write + ?Sized, dt: crate::NaiveDateTime, off: crate::FixedOffset, secform: SecondsFormat, use_z: bool) -> fmt::Result
```

Writes the date, time and offset to the string. same as `%Y-%m-%dT%H:%M:%S%.f%:z`

### `write_hundreds`

```rust
fn write_hundreds(w: &mut impl Write + ?Sized, n: u8) -> fmt::Result
```

Equivalent to `{:02}` formatting for n < 100.

