*[chrono](../../index.md) / [naive](../index.md) / [internals](index.md)*

---

# Module `internals`

Internal helper types for working with dates.

## Contents

- [Structs](#structs)
  - [`YearFlags`](#yearflags)
  - [`Mdf`](#mdf)
- [Constants](#constants)
  - [`YEAR_STARTS_AFTER_MONDAY`](#year-starts-after-monday)
  - [`YEAR_STARTS_AFTER_THUESDAY`](#year-starts-after-thuesday)
  - [`YEAR_STARTS_AFTER_WEDNESDAY`](#year-starts-after-wednesday)
  - [`YEAR_STARTS_AFTER_THURSDAY`](#year-starts-after-thursday)
  - [`YEAR_STARTS_AFTER_FRIDAY`](#year-starts-after-friday)
  - [`YEAR_STARTS_AFTER_SATURDAY`](#year-starts-after-saturday)
  - [`YEAR_STARTS_AFTER_SUNDAY`](#year-starts-after-sunday)
  - [`COMMON_YEAR`](#common-year)
  - [`LEAP_YEAR`](#leap-year)
  - [`A`](#a)
  - [`AG`](#ag)
  - [`B`](#b)
  - [`BA`](#ba)
  - [`C`](#c)
  - [`CB`](#cb)
  - [`D`](#d)
  - [`DC`](#dc)
  - [`E`](#e)
  - [`ED`](#ed)
  - [`F`](#f)
  - [`FE`](#fe)
  - [`G`](#g)
  - [`GF`](#gf)
  - [`YEAR_TO_FLAGS`](#year-to-flags)
  - [`MAX_OL`](#max-ol)
  - [`MAX_MDL`](#max-mdl)
  - [`XX`](#xx)
  - [`MDL_TO_OL`](#mdl-to-ol)
  - [`OL_TO_MDL`](#ol-to-mdl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`YearFlags`](#yearflags) | struct | Year flags (aka the dominical letter). |
| [`Mdf`](#mdf) | struct | Month, day of month and year flags: `(month << 9) \| (day << 4) \| flags` `M_MMMD_DDDD_LFFF` |
| [`YEAR_STARTS_AFTER_MONDAY`](#year-starts-after-monday) | const |  |
| [`YEAR_STARTS_AFTER_THUESDAY`](#year-starts-after-thuesday) | const |  |
| [`YEAR_STARTS_AFTER_WEDNESDAY`](#year-starts-after-wednesday) | const |  |
| [`YEAR_STARTS_AFTER_THURSDAY`](#year-starts-after-thursday) | const |  |
| [`YEAR_STARTS_AFTER_FRIDAY`](#year-starts-after-friday) | const |  |
| [`YEAR_STARTS_AFTER_SATURDAY`](#year-starts-after-saturday) | const |  |
| [`YEAR_STARTS_AFTER_SUNDAY`](#year-starts-after-sunday) | const |  |
| [`COMMON_YEAR`](#common-year) | const |  |
| [`LEAP_YEAR`](#leap-year) | const |  |
| [`A`](#a) | const |  |
| [`AG`](#ag) | const |  |
| [`B`](#b) | const |  |
| [`BA`](#ba) | const |  |
| [`C`](#c) | const |  |
| [`CB`](#cb) | const |  |
| [`D`](#d) | const |  |
| [`DC`](#dc) | const |  |
| [`E`](#e) | const |  |
| [`ED`](#ed) | const |  |
| [`F`](#f) | const |  |
| [`FE`](#fe) | const |  |
| [`G`](#g) | const |  |
| [`GF`](#gf) | const |  |
| [`YEAR_TO_FLAGS`](#year-to-flags) | const |  |
| [`MAX_OL`](#max-ol) | const |  |
| [`MAX_MDL`](#max-mdl) | const |  |
| [`XX`](#xx) | const |  |
| [`MDL_TO_OL`](#mdl-to-ol) | const |  |
| [`OL_TO_MDL`](#ol-to-mdl) | const |  |

## Structs

### `YearFlags`

```rust
struct YearFlags(u8);
```

Year flags (aka the dominical letter).

`YearFlags` are used as the last four bits of `NaiveDate`, `Mdf` and `IsoWeek`.

There are 14 possible classes of year in the Gregorian calendar:
common and leap years starting with Monday through Sunday.

The `YearFlags` stores this information into 4 bits `LWWW`. `L` is the leap year flag, with `1`
for the common year (this simplifies validating an ordinal in `NaiveDate`). `WWW` is a non-zero
`Weekday` of the last day in the preceding year.

#### Implementations

- <span id="yearflags-from-year-mod-400"></span>`const fn from_year_mod_400(year: i32) -> YearFlags` — [`YearFlags`](#yearflags)

- <span id="yearflags-ndays"></span>`const fn ndays(&self) -> u32`

- <span id="yearflags-isoweek-delta"></span>`const fn isoweek_delta(&self) -> u32`

- <span id="yearflags-nisoweeks"></span>`const fn nisoweeks(&self) -> u32`

#### Trait Implementations

##### `impl Clone for YearFlags`

- <span id="yearflags-clone"></span>`fn clone(&self) -> YearFlags` — [`YearFlags`](#yearflags)

##### `impl Copy for YearFlags`

##### `impl Debug for YearFlags`

- <span id="yearflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for YearFlags`

##### `impl Hash for YearFlags`

- <span id="yearflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for YearFlags`

- <span id="yearflags-partialeq-eq"></span>`fn eq(&self, other: &YearFlags) -> bool` — [`YearFlags`](#yearflags)

##### `impl StructuralPartialEq for YearFlags`

### `Mdf`

```rust
struct Mdf(u32);
```

Month, day of month and year flags: `(month << 9) | (day << 4) | flags`
`M_MMMD_DDDD_LFFF`

The whole bits except for the least 3 bits are referred as `Mdl` (month, day of month, and leap
year flag), which is an index to the `MDL_TO_OL` lookup table.

The conversion between the packed calendar date (`Mdf`) and the ordinal date (`NaiveDate`) is
based on the moderately-sized lookup table (~1.5KB) and the packed representation is chosen for
efficient lookup.

The methods of `Mdf` validate their inputs as late as possible. Dates that can't exist, like
February 30, can still be represented. This allows the validation to be combined with the final
table lookup, which is good for performance.

#### Implementations

- <span id="mdf-new"></span>`const fn new(month: u32, day: u32, YearFlags: YearFlags) -> Option<Mdf>` — [`YearFlags`](#yearflags), [`Mdf`](#mdf)

  Makes a new `Mdf` value from month, day and `YearFlags`.

  

  This method doesn't fully validate the range of the `month` and `day` parameters, only as

  much as what can't be deferred until later. The year `flags` are trusted to be correct.

  

  # Errors

  

  Returns `None` if `month > 12` or `day > 31`.

- <span id="mdf-from-ol"></span>`const fn from_ol(ol: i32, YearFlags: YearFlags) -> Mdf` — [`YearFlags`](#yearflags), [`Mdf`](#mdf)

  Makes a new `Mdf` value from an `i32` with an ordinal and a leap year flag, and year

  `flags`.

  

  The `ol` is trusted to be valid, and the `flags` are trusted to match it.

- <span id="mdf-month"></span>`const fn month(&self) -> u32`

  Returns the month of this `Mdf`.

- <span id="mdf-with-month"></span>`const fn with_month(&self, month: u32) -> Option<Mdf>` — [`Mdf`](#mdf)

  Replaces the month of this `Mdf`, keeping the day and flags.

  

  # Errors

  

  Returns `None` if `month > 12`.

- <span id="mdf-day"></span>`const fn day(&self) -> u32`

  Returns the day of this `Mdf`.

- <span id="mdf-with-day"></span>`const fn with_day(&self, day: u32) -> Option<Mdf>` — [`Mdf`](#mdf)

  Replaces the day of this `Mdf`, keeping the month and flags.

  

  # Errors

  

  Returns `None` if `day > 31`.

- <span id="mdf-with-flags"></span>`const fn with_flags(&self, YearFlags: YearFlags) -> Mdf` — [`YearFlags`](#yearflags), [`Mdf`](#mdf)

  Replaces the flags of this `Mdf`, keeping the month and day.

- <span id="mdf-ordinal"></span>`const fn ordinal(&self) -> Option<u32>`

  Returns the ordinal that corresponds to this `Mdf`.

  

  This does a table lookup to calculate the corresponding ordinal. It will return an error if

  the `Mdl` turns out not to be a valid date.

  

  # Errors

  

  Returns `None` if `month == 0` or `day == 0`, or if a the given day does not exist in the

  given month.

- <span id="mdf-year-flags"></span>`const fn year_flags(&self) -> YearFlags` — [`YearFlags`](#yearflags)

  Returns the year flags of this `Mdf`.

- <span id="mdf-ordinal-and-flags"></span>`const fn ordinal_and_flags(&self) -> Option<i32>`

  Returns the ordinal that corresponds to this `Mdf`, encoded as a value including year flags.

  

  This does a table lookup to calculate the corresponding ordinal. It will return an error if

  the `Mdl` turns out not to be a valid date.

  

  # Errors

  

  Returns `None` if `month == 0` or `day == 0`, or if a the given day does not exist in the

  given month.

#### Trait Implementations

##### `impl Clone for Mdf`

- <span id="mdf-clone"></span>`fn clone(&self) -> Mdf` — [`Mdf`](#mdf)

##### `impl Copy for Mdf`

##### `impl Debug for Mdf`

- <span id="mdf-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Mdf`

- <span id="mdf-partialeq-eq"></span>`fn eq(&self, other: &Mdf) -> bool` — [`Mdf`](#mdf)

##### `impl PartialOrd for Mdf`

- <span id="mdf-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Mdf) -> option::Option<cmp::Ordering>` — [`Mdf`](#mdf)

##### `impl StructuralPartialEq for Mdf`

## Constants

### `YEAR_STARTS_AFTER_MONDAY`
```rust
const YEAR_STARTS_AFTER_MONDAY: u8 = 7u8;
```

### `YEAR_STARTS_AFTER_THUESDAY`
```rust
const YEAR_STARTS_AFTER_THUESDAY: u8 = 1u8;
```

### `YEAR_STARTS_AFTER_WEDNESDAY`
```rust
const YEAR_STARTS_AFTER_WEDNESDAY: u8 = 2u8;
```

### `YEAR_STARTS_AFTER_THURSDAY`
```rust
const YEAR_STARTS_AFTER_THURSDAY: u8 = 3u8;
```

### `YEAR_STARTS_AFTER_FRIDAY`
```rust
const YEAR_STARTS_AFTER_FRIDAY: u8 = 4u8;
```

### `YEAR_STARTS_AFTER_SATURDAY`
```rust
const YEAR_STARTS_AFTER_SATURDAY: u8 = 5u8;
```

### `YEAR_STARTS_AFTER_SUNDAY`
```rust
const YEAR_STARTS_AFTER_SUNDAY: u8 = 6u8;
```

### `COMMON_YEAR`
```rust
const COMMON_YEAR: u8 = 8u8;
```

### `LEAP_YEAR`
```rust
const LEAP_YEAR: u8 = 0u8;
```

### `A`
```rust
const A: YearFlags;
```

### `AG`
```rust
const AG: YearFlags;
```

### `B`
```rust
const B: YearFlags;
```

### `BA`
```rust
const BA: YearFlags;
```

### `C`
```rust
const C: YearFlags;
```

### `CB`
```rust
const CB: YearFlags;
```

### `D`
```rust
const D: YearFlags;
```

### `DC`
```rust
const DC: YearFlags;
```

### `E`
```rust
const E: YearFlags;
```

### `ED`
```rust
const ED: YearFlags;
```

### `F`
```rust
const F: YearFlags;
```

### `FE`
```rust
const FE: YearFlags;
```

### `G`
```rust
const G: YearFlags;
```

### `GF`
```rust
const GF: YearFlags;
```

### `YEAR_TO_FLAGS`
```rust
const YEAR_TO_FLAGS: &[YearFlags; 400];
```

### `MAX_OL`
```rust
const MAX_OL: u32 = 732u32;
```

### `MAX_MDL`
```rust
const MAX_MDL: u32 = 831u32;
```

### `XX`
```rust
const XX: i8 = 0i8;
```

### `MDL_TO_OL`
```rust
const MDL_TO_OL: &[i8; 832];
```

### `OL_TO_MDL`
```rust
const OL_TO_MDL: &[u8; 733];
```

