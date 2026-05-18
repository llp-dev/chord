**clap_lex > ext**

# Module: ext

## Contents

**Structs**

- [`Split`](#split)

**Traits**

- [`OsStrExt`](#osstrext) - String-like methods for [`OsStr`]

---

## clap_lex::ext::OsStrExt

*Trait*

String-like methods for [`OsStr`]

**Methods:**

- `try_str`: Converts to a string slice.
- `contains`: Returns `true` if the given pattern matches a sub-slice of
- `find`: Returns the byte index of the first character of this string slice that
- `strip_prefix`: Returns a string slice with the prefix removed.
- `starts_with`: Returns `true` if the given pattern matches a prefix of this
- `split`: An iterator over substrings of this string slice, separated by
- `split_once`: Splits the string on the first occurrence of the specified delimiter and



## clap_lex::ext::Split

*Struct*

**Generic Parameters:**
- 's
- 'n



