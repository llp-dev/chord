**postcard**

# Module: postcard

## Contents

**Modules**

- [`accumulator`](#accumulator) - An accumulator used to collect chunked COBS data and deserialize it.
- [`experimental`](#experimental) - # Experimental Postcard Features
- [`fixint`](#fixint) - # Fixed Size Integers

---

## Module: accumulator

An accumulator used to collect chunked COBS data and deserialize it.



## Module: experimental

# Experimental Postcard Features

Items inside this module require various feature flags, and are not
subject to SemVer stability. Items may be removed or deprecated at
any point.

## Derive

The `experimental-derive` feature enables one experimental feature:

* Max size calculation

### Max Size Calculation

This features enables calculation of the Max serialized size of a message as
an associated `usize` constant called `POSTCARD_MAX_SIZE`. It also provides a
`#[derive(MaxSize)]` macro that can be used for calculating user types.

This is useful for determining the maximum buffer size needed when receiving
or sending a message that has been serialized.

NOTE: This only covers the size of "plain" flavored messages, e.g. not with COBS
or any other Flavors applied. The overhead for these flavors must be calculated
separately.

Please report any missing types, or any incorrectly calculated values.

### Message Schema Generation

This now lives in the `postcard-schema` crate.



## Module: fixint

# Fixed Size Integers

In some cases, the use of variably length encoded data may not be
preferable. These modules, for use with `#[serde(with = ...)]`
"opt out" of variable length encoding.

Support explicitly not provided for `usize` or `isize`, as
these types would not be portable between systems of different
pointer widths.

Although all data in Postcard is typically encoded in little-endian
order, these modules provide a choice to the user to encode the data
in either little or big endian form, which may be useful for zero-copy
applications.



