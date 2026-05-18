**chrono > format > formatting**

# Module: format::formatting

## Contents

**Enums**

- [`SecondsFormat`](#secondsformat) - Specific formatting options for seconds. This may be extended in the

---

## chrono::format::formatting::SecondsFormat

*Enum*

Specific formatting options for seconds. This may be extended in the
future, so exhaustive matching in external code is not recommended.

See the `TimeZone::to_rfc3339_opts` function for usage.

**Variants:**
- `Secs` - Format whole seconds only, with no decimal point nor subseconds.
- `Millis` - Use fixed 3 subsecond digits. This corresponds to [Fixed::Nanosecond3].
- `Micros` - Use fixed 6 subsecond digits. This corresponds to [Fixed::Nanosecond6].
- `Nanos` - Use fixed 9 subsecond digits. This corresponds to [Fixed::Nanosecond9].
- `AutoSi` - Automatically select one of `Secs`, `Millis`, `Micros`, or `Nanos` to display all available

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &SecondsFormat) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SecondsFormat`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



