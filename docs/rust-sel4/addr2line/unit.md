**addr2line > unit**

# Module: unit

## Contents

**Structs**

- [`LocationRangeIter`](#locationrangeiter) - Iterator over `Location`s in a range of addresses, returned by `Context::find_location_range`.

---

## addr2line::unit::LocationRangeIter

*Struct*

Iterator over `Location`s in a range of addresses, returned by `Context::find_location_range`.

**Generic Parameters:**
- 'ctx
- R

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`



