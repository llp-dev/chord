**chrono > format**

# Module: format

## Contents

**Modules**

- [`strftime`](#strftime) - `strftime`/`strptime`-inspired date and time formatting syntax.

**Structs**

- [`InternalFixed`](#internalfixed) - An opaque type representing fixed-format item types for internal uses only.
- [`InternalNumeric`](#internalnumeric) - An opaque type representing numeric item types for internal uses only.
- [`OffsetFormat`](#offsetformat) - Type for specifying the format of UTC offsets.
- [`ParseError`](#parseerror) - An error from the `parse` function.

**Enums**

- [`Colons`](#colons) - The separator between hours and minutes in an offset.
- [`Fixed`](#fixed) - Fixed-format item types.
- [`Item`](#item) - A single formatting item. This is used for both formatting and parsing.
- [`Numeric`](#numeric) - Numeric item types.
- [`OffsetPrecision`](#offsetprecision) - The precision of an offset from UTC formatting item.
- [`Pad`](#pad) - Padding characters for numeric items.
- [`ParseErrorKind`](#parseerrorkind) - The category of parse error

**Type Aliases**

- [`ParseResult`](#parseresult) - Same as `Result<T, ParseError>`.

---

## chrono::format::Colons

*Enum*

The separator between hours and minutes in an offset.

**Variants:**
- `None` - No separator
- `Colon` - Colon (`:`) as separator
- `Maybe` - No separator when formatting, colon allowed when parsing.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Colons`
- **PartialEq**
  - `fn eq(self: &Self, other: &Colons) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## chrono::format::Fixed

*Enum*

Fixed-format item types.

They have their own rules of formatting and parsing.
Otherwise noted, they print in the specified cases but parse case-insensitively.

**Variants:**
- `ShortMonthName` - Abbreviated month names.
- `LongMonthName` - Full month names.
- `ShortWeekdayName` - Abbreviated day of the week names.
- `LongWeekdayName` - Full day of the week names.
- `LowerAmPm` - AM/PM.
- `UpperAmPm` - AM/PM.
- `Nanosecond` - An optional dot plus one or more digits for left-aligned nanoseconds.
- `Nanosecond3` - Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 3.
- `Nanosecond6` - Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 6.
- `Nanosecond9` - Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 9.
- `TimezoneName` - Timezone name.
- `TimezoneOffsetColon` - Offset from the local time to UTC (`+09:00` or `-04:00` or `+00:00`).
- `TimezoneOffsetDoubleColon` - Offset from the local time to UTC with seconds (`+09:00:00` or `-04:00:00` or `+00:00:00`).
- `TimezoneOffsetTripleColon` - Offset from the local time to UTC without minutes (`+09` or `-04` or `+00`).
- `TimezoneOffsetColonZ` - Offset from the local time to UTC (`+09:00` or `-04:00` or `Z`).
- `TimezoneOffset` - Same as [`TimezoneOffsetColon`](#variant.TimezoneOffsetColon) but prints no colon.
- `TimezoneOffsetZ` - Same as [`TimezoneOffsetColonZ`](#variant.TimezoneOffsetColonZ) but prints no colon.
- `RFC2822` - RFC 2822 date and time syntax. Commonly used for email and MIME date and time.
- `RFC3339` - RFC 3339 & ISO 8601 date and time syntax.
- `Internal(InternalFixed)` - Internal uses only.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Fixed) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Fixed`



## chrono::format::InternalFixed

*Struct*

An opaque type representing fixed-format item types for internal uses only.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> InternalFixed`
- **PartialEq**
  - `fn eq(self: &Self, other: &InternalFixed) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## chrono::format::InternalNumeric

*Struct*

An opaque type representing numeric item types for internal uses only.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> InternalNumeric`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &InternalNumeric) -> bool`



## chrono::format::Item

*Enum*

A single formatting item. This is used for both formatting and parsing.

**Generic Parameters:**
- 'a

**Variants:**
- `Literal(&'a str)` - A literally printed and parsed text.
- `Space(&'a str)` - Whitespace. Prints literally but reads zero or more whitespace.
- `Numeric(Numeric, Pad)` - Numeric item. Can be optionally padded to the maximal length (if any) when formatting;
- `Fixed(Fixed)` - Fixed-format item.
- `Error` - Issues a formatting error. Used to signal an invalid format string.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Item<'a>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Item<'a>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## chrono::format::Numeric

*Enum*

Numeric item types.
They have associated formatting width (FW) and parsing width (PW).

The **formatting width** is the minimal width to be formatted.
If the number is too short, and the padding is not [`Pad::None`](./enum.Pad.html#variant.None),
then it is left-padded.
If the number is too long or (in some cases) negative, it is printed as is.

The **parsing width** is the maximal width to be scanned.
The parser only tries to consume from one to given number of digits (greedily).
It also trims the preceding whitespace if any.
It cannot parse the negative number, so some date and time cannot be formatted then
parsed with the same formatting items.

**Variants:**
- `Year` - Full Gregorian year (FW=4, PW=∞).
- `YearDiv100` - Gregorian year divided by 100 (century number; FW=PW=2). Implies the non-negative year.
- `YearMod100` - Gregorian year modulo 100 (FW=PW=2). Cannot be negative.
- `IsoYear` - Year in the ISO week date (FW=4, PW=∞).
- `IsoYearDiv100` - Year in the ISO week date, divided by 100 (FW=PW=2). Implies the non-negative year.
- `IsoYearMod100` - Year in the ISO week date, modulo 100 (FW=PW=2). Cannot be negative.
- `Quarter` - Quarter (FW=PW=1).
- `Month` - Month (FW=PW=2).
- `Day` - Day of the month (FW=PW=2).
- `WeekFromSun` - Week number, where the week 1 starts at the first Sunday of January (FW=PW=2).
- `WeekFromMon` - Week number, where the week 1 starts at the first Monday of January (FW=PW=2).
- `IsoWeek` - Week number in the ISO week date (FW=PW=2).
- `NumDaysFromSun` - Day of the week, where Sunday = 0 and Saturday = 6 (FW=PW=1).
- `WeekdayFromMon` - Day of the week, where Monday = 1 and Sunday = 7 (FW=PW=1).
- `Ordinal` - Day of the year (FW=PW=3).
- `Hour` - Hour number in the 24-hour clocks (FW=PW=2).
- `Hour12` - Hour number in the 12-hour clocks (FW=PW=2).
- `Minute` - The number of minutes since the last whole hour (FW=PW=2).
- `Second` - The number of seconds since the last whole minute (FW=PW=2).
- `Nanosecond` - The number of nanoseconds since the last whole second (FW=PW=9).
- `Timestamp` - The number of non-leap seconds since the midnight UTC on January 1, 1970 (FW=1, PW=∞).
- `Internal(InternalNumeric)` - Internal uses only.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Numeric`
- **PartialEq**
  - `fn eq(self: &Self, other: &Numeric) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## chrono::format::OffsetFormat

*Struct*

Type for specifying the format of UTC offsets.

**Fields:**
- `precision: OffsetPrecision` - See `OffsetPrecision`.
- `colons: Colons` - Separator between hours, minutes and seconds.
- `allow_zulu: bool` - Represent `+00:00` as `Z`.
- `padding: Pad` - Pad the hour value to two digits.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> OffsetFormat`
- **PartialEq**
  - `fn eq(self: &Self, other: &OffsetFormat) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## chrono::format::OffsetPrecision

*Enum*

The precision of an offset from UTC formatting item.

**Variants:**
- `Hours` - Format offset from UTC as only hours. Not recommended, it is not uncommon for timezones to
- `Minutes` - Format offset from UTC as hours and minutes.
- `Seconds` - Format offset from UTC as hours, minutes and seconds.
- `OptionalMinutes` - Format offset from UTC as hours, and optionally with minutes.
- `OptionalSeconds` - Format offset from UTC as hours and minutes, and optionally seconds.
- `OptionalMinutesAndSeconds` - Format offset from UTC as hours and optionally minutes and seconds.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> OffsetPrecision`
- **PartialEq**
  - `fn eq(self: &Self, other: &OffsetPrecision) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## chrono::format::Pad

*Enum*

Padding characters for numeric items.

**Variants:**
- `None` - No padding.
- `Zero` - Zero (`0`) padding.
- `Space` - Space padding.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Pad`
- **PartialEq**
  - `fn eq(self: &Self, other: &Pad) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## chrono::format::ParseError

*Struct*

An error from the `parse` function.

**Tuple Struct**: `()`

**Methods:**

- `fn kind(self: &Self) -> ParseErrorKind` - The category of parse error

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> ParseError`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParseError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## chrono::format::ParseErrorKind

*Enum*

The category of parse error

**Variants:**
- `OutOfRange` - Given field is out of permitted range.
- `Impossible` - There is no possible date and time value with given set of fields.
- `NotEnough` - Given set of fields is not enough to make a requested date and time value.
- `Invalid` - The input string has some invalid character sequence for given formatting items.
- `TooShort` - The input string has been prematurely ended.
- `TooLong` - All formatting items have been read but there is a remaining input.
- `BadFormat` - There was an error on the formatting string, or there were non-supported formatting items.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ParseErrorKind`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParseErrorKind) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## chrono::format::ParseResult

*Type Alias*: `Result<T, ParseError>`

Same as `Result<T, ParseError>`.



## Module: strftime

`strftime`/`strptime`-inspired date and time formatting syntax.

## Specifiers

The following specifiers are available both to formatting and parsing.

| Spec. | Example  | Description                                                                |
|-------|----------|----------------------------------------------------------------------------|
|       |          | **DATE SPECIFIERS:**                                                       |
| `%Y`  | `2001`   | The full proleptic Gregorian year, zero-padded to 4 digits. chrono supports years from -262144 to 262143. Note: years before 1 BCE or after 9999 CE, require an initial sign (+/-).|
| `%C`  | `20`     | The proleptic Gregorian year divided by 100, zero-padded to 2 digits. [^1] |
| `%y`  | `01`     | The proleptic Gregorian year modulo 100, zero-padded to 2 digits. [^1]     |
|       |          |                                                                            |
| `%q`  | `1`      | Quarter of year (1-4)                                                      |
| `%m`  | `07`     | Month number (01--12), zero-padded to 2 digits.                            |
| `%b`  | `Jul`    | Abbreviated month name. Always 3 letters.                                  |
| `%B`  | `July`   | Full month name. Also accepts corresponding abbreviation in parsing.       |
| `%h`  | `Jul`    | Same as `%b`.                                                              |
|       |          |                                                                            |
| `%d`  | `08`     | Day number (01--31), zero-padded to 2 digits.                              |
| `%e`  | ` 8`     | Same as `%d` but space-padded. Same as `%_d`.                              |
|       |          |                                                                            |
| `%a`  | `Sun`    | Abbreviated weekday name. Always 3 letters.                                |
| `%A`  | `Sunday` | Full weekday name. Also accepts corresponding abbreviation in parsing.     |
| `%w`  | `0`      | Sunday = 0, Monday = 1, ..., Saturday = 6.                                 |
| `%u`  | `7`      | Monday = 1, Tuesday = 2, ..., Sunday = 7. (ISO 8601)                       |
|       |          |                                                                            |
| `%U`  | `28`     | Week number starting with Sunday (00--53), zero-padded to 2 digits. [^2]   |
| `%W`  | `27`     | Same as `%U`, but week 1 starts with the first Monday in that year instead.|
|       |          |                                                                            |
| `%G`  | `2001`   | Same as `%Y` but uses the year number in ISO 8601 week date. [^3]          |
| `%g`  | `01`     | Same as `%y` but uses the year number in ISO 8601 week date. [^3]          |
| `%V`  | `27`     | Same as `%U` but uses the week number in ISO 8601 week date (01--53). [^3] |
|       |          |                                                                            |
| `%j`  | `189`    | Day of the year (001--366), zero-padded to 3 digits.                       |
|       |          |                                                                            |
| `%D`  | `07/08/01`    | Month-day-year format. Same as `%m/%d/%y`.                            |
| `%x`  | `07/08/01`    | Locale's date representation (e.g., 12/31/99).                        |
| `%F`  | `2001-07-08`  | Year-month-day format (ISO 8601). Same as `%Y-%m-%d`.                 |
| `%v`  | ` 8-Jul-2001` | Day-month-year format. Same as `%e-%b-%Y`.                            |
|       |          |                                                                            |
|       |          | **TIME SPECIFIERS:**                                                       |
| `%H`  | `00`     | Hour number (00--23), zero-padded to 2 digits.                             |
| `%k`  | ` 0`     | Same as `%H` but space-padded. Same as `%_H`.                              |
| `%I`  | `12`     | Hour number in 12-hour clocks (01--12), zero-padded to 2 digits.           |
| `%l`  | `12`     | Same as `%I` but space-padded. Same as `%_I`.                              |
|       |          |                                                                            |
| `%P`  | `am`     | `am` or `pm` in 12-hour clocks.                                            |
| `%p`  | `AM`     | `AM` or `PM` in 12-hour clocks.                                            |
|       |          |                                                                            |
| `%M`  | `34`     | Minute number (00--59), zero-padded to 2 digits.                           |
| `%S`  | `60`     | Second number (00--60), zero-padded to 2 digits. [^4]                      |
| `%f`  | `26490000`    | Number of nanoseconds since last whole second. [^7]                   |
| `%.f` | `.026490`| Decimal fraction of a second. Consumes the leading dot. [^7]               |
| `%.3f`| `.026`        | Decimal fraction of a second with a fixed length of 3.                |
| `%.6f`| `.026490`     | Decimal fraction of a second with a fixed length of 6.                |
| `%.9f`| `.026490000`  | Decimal fraction of a second with a fixed length of 9.                |
| `%3f` | `026`         | Decimal fraction of a second like `%.3f` but without the leading dot. |
| `%6f` | `026490`      | Decimal fraction of a second like `%.6f` but without the leading dot. |
| `%9f` | `026490000`   | Decimal fraction of a second like `%.9f` but without the leading dot. |
|       |               |                                                                       |
| `%R`  | `00:34`       | Hour-minute format. Same as `%H:%M`.                                  |
| `%T`  | `00:34:60`    | Hour-minute-second format. Same as `%H:%M:%S`.                        |
| `%X`  | `00:34:60`    | Locale's time representation (e.g., 23:13:48).                        |
| `%r`  | `12:34:60 AM` | Locale's 12 hour clock time. (e.g., 11:11:04 PM). Falls back to `%X` if the locale does not have a 12 hour clock format. |
|       |          |                                                                            |
|       |          | **TIME ZONE SPECIFIERS:**                                                  |
| `%Z`  | `ACST`   | Local time zone name. Skips all non-whitespace characters during parsing. Identical to `%:z` when formatting. [^8] |
| `%z`  | `+0930`  | Offset from the local time to UTC (with UTC being `+0000`).                |
| `%:z` | `+09:30` | Same as `%z` but with a colon.                                             |
|`%::z`|`+09:30:00`| Offset from the local time to UTC with seconds.                            |
|`%:::z`| `+09`    | Offset from the local time to UTC without minutes.                         |
| `%#z` | `+09`    | *Parsing only:* Same as `%z` but allows minutes to be missing or present.  |
|       |          |                                                                            |
|       |          | **DATE & TIME SPECIFIERS:**                                                |
|`%c`|`Sun Jul  8 00:34:60 2001`|Locale's date and time (e.g., Thu Mar  3 23:05:25 2005).       |
| `%+`  | `2001-07-08T00:34:60.026490+09:30` | ISO 8601 / RFC 3339 date & time format. [^5]     |
|       |               |                                                                       |
| `%s`  | `994518299`   | UNIX timestamp, the number of seconds since 1970-01-01 00:00 UTC. [^6]|
|       |          |                                                                            |
|       |          | **SPECIAL SPECIFIERS:**                                                    |
| `%t`  |          | Literal tab (`\t`).                                                        |
| `%n`  |          | Literal newline (`\n`).                                                    |
| `%%`  |          | Literal percent sign.                                                      |

It is possible to override the default padding behavior of numeric specifiers `%?`.
This is not allowed for other specifiers and will result in the `BAD_FORMAT` error.

Modifier | Description
-------- | -----------
`%-?`    | Suppresses any padding including spaces and zeroes. (e.g. `%j` = `012`, `%-j` = `12`)
`%_?`    | Uses spaces as a padding. (e.g. `%j` = `012`, `%_j` = ` 12`)
`%0?`    | Uses zeroes as a padding. (e.g. `%e` = ` 9`, `%0e` = `09`)

Notes:

[^1]: `%C`, `%y`:
   This is floor division, so 100 BCE (year number -99) will print `-1` and `99` respectively.
   For `%y`, values greater or equal to 70 are interpreted as being in the 20th century,
   values smaller than 70 in the 21st century.

[^2]: `%U`:
   Week 1 starts with the first Sunday in that year.
   It is possible to have week 0 for days before the first Sunday.

[^3]: `%G`, `%g`, `%V`:
   Week 1 is the first week with at least 4 days in that year.
   Week 0 does not exist, so this should be used with `%G` or `%g`.

[^4]: `%S`:
   It accounts for leap seconds, so `60` is possible.

[^5]: `%+`: Same as `%Y-%m-%dT%H:%M:%S%.f%:z`, i.e. 0, 3, 6 or 9 fractional
   digits for seconds and colons in the time zone offset.
   <br>
   <br>
   This format also supports having a `Z` or `UTC` in place of `%:z`. They
   are equivalent to `+00:00`.
   <br>
   <br>
   Note that all `T`, `Z`, and `UTC` are parsed case-insensitively.
   <br>
   <br>
   The typical `strftime` implementations have different (and locale-dependent)
   formats for this specifier. While Chrono's format for `%+` is far more
   stable, it is best to avoid this specifier if you want to control the exact
   output.

[^6]: `%s`:
   This is not padded and can be negative.
   For the purpose of Chrono, it only accounts for non-leap seconds
   so it slightly differs from ISO C `strftime` behavior.

[^7]: `%f`, `%.f`:
   <br>
   `%f` and `%.f` are notably different formatting specifiers.<br>
   `%f` counts the number of nanoseconds since the last whole second, while `%.f` is a fraction of a
   second.<br>
   Example: 7μs is formatted as `7000` with `%f`, and formatted as `.000007` with `%.f`.

[^8]: `%Z`:
   Since `chrono` is not aware of timezones beyond their offsets, this specifier
   **only prints the offset** when used for formatting. The timezone abbreviation
   will NOT be printed. See [this issue](https://github.com/chronotope/chrono/issues/960)
   for more information.
   <br>
   <br>
   Offset will not be populated from the parsed data, nor will it be validated.
   Timezone is completely ignored. Similar to the glibc `strptime` treatment of
   this format code.
   <br>
   <br>
   It is not possible to reliably convert from an abbreviation to an offset,
   for example CDT can mean either Central Daylight Time (North America) or
   China Daylight Time.



