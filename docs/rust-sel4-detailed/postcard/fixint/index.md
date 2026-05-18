*[postcard](../index.md) / [fixint](index.md)*

---

# Module `fixint`

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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`le`](#le) | mod | Use with the `#[serde(with = "postcard::fixint::le")]` field attribute. |
| [`be`](#be) | mod | Disables varint serialization/deserialization for the specified integer field. |
| [`impl_fixint!`](#impl-fixint) | macro |  |

## Modules

- [`le`](le/index.md) — Use with the `#[serde(with = "postcard::fixint::le")]` field attribute.
- [`be`](be/index.md) — Disables varint serialization/deserialization for the specified integer field.

## Macros

### `impl_fixint!`

