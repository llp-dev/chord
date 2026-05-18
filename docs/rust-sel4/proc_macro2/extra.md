**proc_macro2 > extra**

# Module: extra

## Contents

**Structs**

- [`DelimSpan`](#delimspan) - An object that holds a [`Group`]'s `span_open()` and `span_close()` together

---

## proc_macro2::extra::DelimSpan

*Struct*

An object that holds a [`Group`]'s `span_open()` and `span_close()` together
in a more compact representation than holding those 2 spans individually.

[`Group`]: crate::Group

**Methods:**

- `fn join(self: &Self) -> Span` - Returns a span covering the entire delimited group.
- `fn open(self: &Self) -> Span` - Returns a span for the opening punctuation of the group only.
- `fn close(self: &Self) -> Span` - Returns a span for the closing punctuation of the group only.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DelimSpan`



