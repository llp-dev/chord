**equivalent**

# Module: equivalent

## Contents

**Traits**

- [`Comparable`](#comparable) - Key ordering trait.
- [`Equivalent`](#equivalent) - Key equivalence trait.

---

## equivalent::Comparable

*Trait*

Key ordering trait.

This trait allows ordered map lookup to be customized. It has one blanket
implementation that uses the regular solution with `Borrow` and `Ord`, just
like `BTreeMap` does, so that you can pass `&str` to lookup into a map with
`String` keys and so on.

**Methods:**

- `compare`: Compare self to `key` and return their ordering.



## equivalent::Equivalent

*Trait*

Key equivalence trait.

This trait allows hash table lookup to be customized. It has one blanket
implementation that uses the regular solution with `Borrow` and `Eq`, just
like `HashMap` does, so that you can pass `&str` to lookup into a map with
`String` keys and so on.

# Contract

The implementor **must** hash like `K`, if it is hashable.

**Methods:**

- `equivalent`: Compare self to `key` and return `true` if they are equal.



