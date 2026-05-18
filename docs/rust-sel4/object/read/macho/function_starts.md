**object > read > macho > function_starts**

# Module: read::macho::function_starts

## Contents

**Structs**

- [`FunctionStartsIterator`](#functionstartsiterator) - Iterator over the function starts in a `LC_FUNCTION_STARTS` load command.

---

## object::read::macho::function_starts::FunctionStartsIterator

*Struct*

Iterator over the function starts in a `LC_FUNCTION_STARTS` load command.

**Generic Parameters:**
- 'data

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<u64>>` - Returns the next function start address.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> FunctionStartsIterator<'data>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> FunctionStartsIterator<'data>`



