**syn > parse > discouraged**

# Module: parse::discouraged

## Contents

**Traits**

- [`AnyDelimiter`](#anydelimiter) - Extensions to the `ParseStream` API to support manipulating invisible
- [`Speculative`](#speculative) - Extensions to the `ParseStream` API to support speculative parsing.

---

## syn::parse::discouraged::AnyDelimiter

*Trait*

Extensions to the `ParseStream` API to support manipulating invisible
delimiters the same as if they were visible.

**Methods:**

- `parse_any_delimiter`: Returns the delimiter, the span of the delimiter token, and the nested



## syn::parse::discouraged::Speculative

*Trait*

Extensions to the `ParseStream` API to support speculative parsing.

**Methods:**

- `advance_to`: Advance this parse stream to the position of a forked parse stream.



