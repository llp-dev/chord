*[chrono](../index.md) / [weekday](index.md)*

---

# Module `weekday`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`weekday_serde`](#weekday-serde) | mod |  |
| [`ParseWeekdayError`](#parseweekdayerror) | struct | An error resulting from reading `Weekday` value with `FromStr`. |
| [`Weekday`](#weekday) | enum | The day of week. |

## Modules

- [`weekday_serde`](weekday_serde/index.md)

## Structs

### `ParseWeekdayError`

```rust
struct ParseWeekdayError {
    _dummy: (),
}
```

An error resulting from reading `Weekday` value with `FromStr`.

#### Trait Implementations

##### `impl Clone for ParseWeekdayError`

- <span id="parseweekdayerror-clone"></span>`fn clone(&self) -> ParseWeekdayError` — [`ParseWeekdayError`](#parseweekdayerror)

##### `impl Debug for ParseWeekdayError`

- <span id="parseweekdayerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseWeekdayError`

- <span id="parseweekdayerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseWeekdayError`

##### `impl PartialEq for ParseWeekdayError`

- <span id="parseweekdayerror-partialeq-eq"></span>`fn eq(&self, other: &ParseWeekdayError) -> bool` — [`ParseWeekdayError`](#parseweekdayerror)

##### `impl StructuralPartialEq for ParseWeekdayError`

##### `impl ToString for ParseWeekdayError`

- <span id="parseweekdayerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `Weekday`

```rust
enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}
```

The day of week.

The order of the days of week depends on the context.
(This is why this type does *not* implement `PartialOrd` or `Ord` traits.)
One should prefer `*_from_monday` or `*_from_sunday` methods to get the correct result.

# Example
```rust
use chrono::Weekday;

let monday = "Monday".parse::<Weekday>().unwrap();
assert_eq!(monday, Weekday::Mon);

let sunday = Weekday::try_from(6).unwrap();
assert_eq!(sunday, Weekday::Sun);

assert_eq!(sunday.num_days_from_monday(), 6); // starts counting with Monday = 0
assert_eq!(sunday.number_from_monday(), 7); // starts counting with Monday = 1
assert_eq!(sunday.num_days_from_sunday(), 0); // starts counting with Sunday = 0
assert_eq!(sunday.number_from_sunday(), 1); // starts counting with Sunday = 1

assert_eq!(sunday.succ(), monday);
assert_eq!(sunday.pred(), Weekday::Sat);
```

#### Variants

- **`Mon`**

  Monday.

- **`Tue`**

  Tuesday.

- **`Wed`**

  Wednesday.

- **`Thu`**

  Thursday.

- **`Fri`**

  Friday.

- **`Sat`**

  Saturday.

- **`Sun`**

  Sunday.

#### Implementations

- <span id="weekday-succ"></span>`const fn succ(&self) -> Weekday` — [`Weekday`](#weekday)

  The next day in the week.

  

  `w`:        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  ----------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.succ()`: | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun` | `Mon`

- <span id="weekday-pred"></span>`const fn pred(&self) -> Weekday` — [`Weekday`](#weekday)

  The previous day in the week.

  

  `w`:        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  ----------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.pred()`: | `Sun` | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat`

- <span id="weekday-number-from-monday"></span>`const fn number_from_monday(&self) -> u32`

  Returns a day-of-week number starting from Monday = 1. (ISO 8601 weekday number)

  

  `w`:                      | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  ------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.number_from_monday()`: | 1     | 2     | 3     | 4     | 5     | 6     | 7

- <span id="weekday-number-from-sunday"></span>`const fn number_from_sunday(&self) -> u32`

  Returns a day-of-week number starting from Sunday = 1.

  

  `w`:                      | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  ------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.number_from_sunday()`: | 2     | 3     | 4     | 5     | 6     | 7     | 1

- <span id="weekday-num-days-from-monday"></span>`const fn num_days_from_monday(&self) -> u32`

  Returns a day-of-week number starting from Monday = 0.

  

  `w`:                        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  --------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.num_days_from_monday()`: | 0     | 1     | 2     | 3     | 4     | 5     | 6

  

  # Example

  

  ```rust

  #[cfg(feature = "clock")] {

  use chrono::{Local, Datelike};

  // MTWRFSU is occasionally used as a single-letter abbreviation of the weekdays.

  // Use `num_days_from_monday` to index into the array.

  const MTWRFSU: [char; 7] = ['M', 'T', 'W', 'R', 'F', 'S', 'U'];

  

  let today = Local::now().weekday();

  println!("{}", MTWRFSU[today.num_days_from_monday() as usize]);

  }

  ```

- <span id="weekday-num-days-from-sunday"></span>`const fn num_days_from_sunday(&self) -> u32`

  Returns a day-of-week number starting from Sunday = 0.

  

  `w`:                        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  --------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.num_days_from_sunday()`: | 1     | 2     | 3     | 4     | 5     | 6     | 0

- <span id="weekday-days-since"></span>`const fn days_since(&self, other: Weekday) -> u32` — [`Weekday`](#weekday)

  The number of days since the given day.

  

  # Examples

  

  ```rust

  use chrono::Weekday::*;

  assert_eq!(Mon.days_since(Mon), 0);

  assert_eq!(Sun.days_since(Tue), 5);

  assert_eq!(Wed.days_since(Sun), 3);

  ```

#### Trait Implementations

##### `impl Clone for Weekday`

- <span id="weekday-clone"></span>`fn clone(&self) -> Weekday` — [`Weekday`](#weekday)

##### `impl Copy for Weekday`

##### `impl Debug for Weekday`

- <span id="weekday-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for super::Weekday`

- <span id="superweekday-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Weekday`

##### `impl Display for Weekday`

- <span id="weekday-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Weekday`

##### `impl FromIterator for WeekdaySet`

- <span id="weekdayset-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Weekday>>(iter: T) -> Self`

##### `impl FromPrimitive for Weekday`

- <span id="weekday-fromprimitive-from-i64"></span>`fn from_i64(n: i64) -> Option<Weekday>` — [`Weekday`](#weekday)

- <span id="weekday-fromprimitive-from-u64"></span>`fn from_u64(n: u64) -> Option<Weekday>` — [`Weekday`](#weekday)

##### `impl FromStr for crate::Weekday`

- <span id="crateweekday-fromstr-type-err"></span>`type Err = ParseWeekdayError`

- <span id="crateweekday-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for Weekday`

- <span id="weekday-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Weekday`

- <span id="weekday-partialeq-eq"></span>`fn eq(&self, other: &Weekday) -> bool` — [`Weekday`](#weekday)

##### `impl Serialize for super::Weekday`

- <span id="superweekday-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Weekday`

##### `impl ToString for Weekday`

- <span id="weekday-tostring-to-string"></span>`fn to_string(&self) -> String`

