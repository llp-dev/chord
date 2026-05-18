**clap_lex**

# Module: clap_lex

## Contents

**Structs**

- [`ArgCursor`](#argcursor) - Position within [`RawArgs`]
- [`ParsedArg`](#parsedarg) - Command-line Argument
- [`RawArgs`](#rawargs) - Command-line arguments
- [`ShortFlags`](#shortflags) - Walk through short flags within a [`ParsedArg`]

---

## clap_lex::ArgCursor

*Struct*

Position within [`RawArgs`]

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ArgCursor`
- **Ord**
  - `fn cmp(self: &Self, other: &ArgCursor) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArgCursor) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArgCursor) -> bool`



## clap_lex::ParsedArg

*Struct*

Command-line Argument

**Generic Parameters:**
- 's

**Methods:**

- `fn is_empty(self: &Self) -> bool` - Argument is length of 0
- `fn is_stdio(self: &Self) -> bool` - Does the argument look like a stdio argument (`-`)
- `fn is_escape(self: &Self) -> bool` - Does the argument look like an argument escape (`--`)
- `fn is_negative_number(self: &Self) -> bool` - Does the argument look like a negative number?
- `fn to_long(self: &Self) -> Option<(Result<&'s str, &'s OsStr>, Option<&'s OsStr>)>` - Treat as a long-flag
- `fn is_long(self: &Self) -> bool` - Can treat as a long-flag
- `fn to_short(self: &Self) -> Option<ShortFlags<'s>>` - Treat as a short-flag
- `fn is_short(self: &Self) -> bool` - Can treat as a short-flag
- `fn to_value_os(self: &Self) -> &'s OsStr` - Treat as a value
- `fn to_value(self: &Self) -> Result<&'s str, &'s OsStr>` - Treat as a value
- `fn display(self: &Self) -> impl Trait` - Safely print an argument that may contain non-UTF8 content

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ParsedArg<'s>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> ParsedArg<'s>`
- **Ord**
  - `fn cmp(self: &Self, other: &ParsedArg<'s>) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ParsedArg<'s>) -> $crate::option::Option<$crate::cmp::Ordering>`



## clap_lex::RawArgs

*Struct*

Command-line arguments

**Methods:**

- `fn from_args() -> Self`
- `fn new<impl Into<OsString>, impl IntoIterator<Item = impl Into<OsString>>>(iter: impl Trait) -> Self`
- `fn cursor(self: &Self) -> ArgCursor` - Create a cursor for walking the arguments
- `fn next<'s>(self: &'s Self, cursor: & mut ArgCursor) -> Option<ParsedArg<'s>>` - Advance the cursor, returning the next [`ParsedArg`]
- `fn next_os<'s>(self: &'s Self, cursor: & mut ArgCursor) -> Option<&'s OsStr>` - Advance the cursor, returning a raw argument value.
- `fn peek<'s>(self: &'s Self, cursor: &ArgCursor) -> Option<ParsedArg<'s>>` - Return the next [`ParsedArg`]
- `fn peek_os<'s>(self: &'s Self, cursor: &ArgCursor) -> Option<&'s OsStr>` - Return a raw argument value.
- `fn remaining<'s>(self: &'s Self, cursor: & mut ArgCursor) -> impl Trait` - Return all remaining raw arguments, advancing the cursor to the end
- `fn seek(self: &Self, cursor: & mut ArgCursor, pos: SeekFrom)` - Adjust the cursor's position
- `fn insert<impl Into<OsString>, impl IntoIterator<Item = impl Into<OsString>>>(self: & mut Self, cursor: &ArgCursor, insert_items: impl Trait)` - Inject arguments before the [`RawArgs::next`]
- `fn is_end(self: &Self, cursor: &ArgCursor) -> bool` - Any remaining args?

**Traits:** Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> RawArgs`
- **PartialEq**
  - `fn eq(self: &Self, other: &RawArgs) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(val: I) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> RawArgs`



## clap_lex::ShortFlags

*Struct*

Walk through short flags within a [`ParsedArg`]

**Generic Parameters:**
- 's

**Methods:**

- `fn advance_by(self: & mut Self, n: usize) -> Result<(), usize>` - Move the iterator forward by `n` short flags
- `fn is_empty(self: &Self) -> bool` - No short flags left
- `fn is_negative_number(self: &Self) -> bool` - Does the short flag look like a number
- `fn next_flag(self: & mut Self) -> Option<Result<char, &'s OsStr>>` - Advance the iterator, returning the next short flag on success
- `fn next_value_os(self: & mut Self) -> Option<&'s OsStr>` - Advance the iterator, returning everything left as a value

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ShortFlags<'s>`



