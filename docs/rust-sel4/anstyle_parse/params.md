**anstyle_parse > params**

# Module: params

## Contents

**Structs**

- [`Params`](#params)
- [`ParamsIter`](#paramsiter) - Immutable subparameter iterator.

---

## anstyle_parse::params::Params

*Struct*

**Methods:**

- `fn len(self: &Self) -> usize` - Returns the number of parameters.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if there are no parameters present.
- `fn iter(self: &Self) -> ParamsIter` - Returns an iterator over all parameters and subparameters.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Params) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Params`
- **Default**
  - `fn default() -> Params`
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`



## anstyle_parse::params::ParamsIter

*Struct*

Immutable subparameter iterator.

**Generic Parameters:**
- 'a

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



