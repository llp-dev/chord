**strsim**

# Module: strsim

## Contents

**Structs**

- [`GrowingHashmapChar`](#growinghashmapchar) - specialized hashmap to store user provided types
- [`GrowingHashmapMapElemChar`](#growinghashmapmapelemchar)
- [`HybridGrowingHashmapChar`](#hybridgrowinghashmapchar)
- [`RowId`](#rowid)
- [`StringWrapper`](#stringwrapper)

**Enums**

- [`StrSimError`](#strsimerror)

**Functions**

- [`bigrams`](#bigrams) - Returns an Iterator of char tuples.
- [`damerau_levenshtein`](#damerau_levenshtein) - Like optimal string alignment, but substrings can be edited an unlimited
- [`damerau_levenshtein_impl`](#damerau_levenshtein_impl)
- [`flat_index`](#flat_index)
- [`generic_damerau_levenshtein`](#generic_damerau_levenshtein) - Like optimal string alignment, but substrings can be edited an unlimited
- [`generic_hamming`](#generic_hamming) - Calculates the number of positions in the two sequences where the elements
- [`generic_jaro`](#generic_jaro) - Calculates the Jaro similarity between two sequences. The returned value
- [`generic_jaro_winkler`](#generic_jaro_winkler) - Like Jaro but gives a boost to sequences that have a common prefix.
- [`generic_levenshtein`](#generic_levenshtein) - Calculates the minimum number of insertions, deletions, and substitutions
- [`hamming`](#hamming) - Calculates the number of positions in the two strings where the characters
- [`jaro`](#jaro) - Calculates the Jaro similarity between two strings. The returned value
- [`jaro_winkler`](#jaro_winkler) - Like Jaro but gives a boost to strings that have a common prefix.
- [`levenshtein`](#levenshtein) - Calculates the minimum number of insertions, deletions, and substitutions
- [`normalized_damerau_levenshtein`](#normalized_damerau_levenshtein) - Calculates a normalized score of the Damerau–Levenshtein algorithm between
- [`normalized_levenshtein`](#normalized_levenshtein) - Calculates a normalized score of the Levenshtein algorithm between 0.0 and
- [`osa_distance`](#osa_distance) - Like Levenshtein but allows for adjacent transpositions. Each substring can
- [`sorensen_dice`](#sorensen_dice) - Calculates a Sørensen-Dice similarity distance using bigrams.

**Type Aliases**

- [`HammingResult`](#hammingresult)

---

## strsim::GrowingHashmapChar

*Struct*

specialized hashmap to store user provided types
this implementation relies on a couple of base assumptions in order to simplify the implementation
- the hashmap does not have an upper limit of included items
- the default value for the `ValueType` can be used as a dummy value to indicate an empty cell
- elements can't be removed
- only allocates memory on first write access.
  This improves performance for hashmaps that are never written to

**Generic Parameters:**
- ValueType

**Fields:**
- `used: i32`
- `fill: i32`
- `mask: i32`
- `map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>`

**Methods:**

- `fn get(self: &Self, key: u32) -> ValueType`
- `fn get_mut(self: & mut Self, key: u32) -> & mut ValueType`
- `fn allocate(self: & mut Self)`
- `fn lookup(self: &Self, key: u32) -> usize` - lookup key inside the hashmap using a similar collision resolution
- `fn grow(self: & mut Self, min_used: i32)`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## strsim::GrowingHashmapMapElemChar

*Struct*

**Generic Parameters:**
- ValueType

**Fields:**
- `key: u32`
- `value: ValueType`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GrowingHashmapMapElemChar<ValueType>`
- **Default**
  - `fn default() -> GrowingHashmapMapElemChar<ValueType>`



## strsim::HammingResult

*Type Alias*: `Result<usize, StrSimError>`



## strsim::HybridGrowingHashmapChar

*Struct*

**Generic Parameters:**
- ValueType

**Fields:**
- `map: GrowingHashmapChar<ValueType>`
- `extended_ascii: [ValueType; 256]`

**Methods:**

- `fn get(self: &Self, key: char) -> ValueType`
- `fn get_mut(self: & mut Self, key: char) -> & mut ValueType`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## strsim::RowId

*Struct*

**Fields:**
- `val: isize`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &RowId) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RowId`



## strsim::StrSimError

*Enum*

**Variants:**
- `DifferentLengthArgs`

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, fmt: & mut Formatter) -> Result<(), fmt::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &StrSimError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## strsim::StringWrapper

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `(&'a str)`



## strsim::bigrams

*Function*

Returns an Iterator of char tuples.

```rust
fn bigrams(s: &str) -> impl Trait
```



## strsim::damerau_levenshtein

*Function*

Like optimal string alignment, but substrings can be edited an unlimited
number of times, and the triangle inequality holds.

```
use strsim::damerau_levenshtein;

assert_eq!(2, damerau_levenshtein("ab", "bca"));
```

```rust
fn damerau_levenshtein(a: &str, b: &str) -> usize
```



## strsim::damerau_levenshtein_impl

*Function*

```rust
fn damerau_levenshtein_impl<Iter1, Iter2>(s1: Iter1, len1: usize, s2: Iter2, len2: usize) -> usize
```



## strsim::flat_index

*Function*

```rust
fn flat_index(i: usize, j: usize, width: usize) -> usize
```



## strsim::generic_damerau_levenshtein

*Function*

Like optimal string alignment, but substrings can be edited an unlimited
number of times, and the triangle inequality holds.

```
use strsim::generic_damerau_levenshtein;

assert_eq!(2, generic_damerau_levenshtein(&[1,2], &[2,3,1]));
```

```rust
fn generic_damerau_levenshtein<Elem>(a_elems: &[Elem], b_elems: &[Elem]) -> usize
```



## strsim::generic_hamming

*Function*

Calculates the number of positions in the two sequences where the elements
differ. Returns an error if the sequences have different lengths.

```rust
fn generic_hamming<Iter1, Iter2, Elem1, Elem2>(a: Iter1, b: Iter2) -> HammingResult
```



## strsim::generic_jaro

*Function*

Calculates the Jaro similarity between two sequences. The returned value
is between 0.0 and 1.0 (higher value means more similar).

```rust
fn generic_jaro<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
```



## strsim::generic_jaro_winkler

*Function*

Like Jaro but gives a boost to sequences that have a common prefix.

```rust
fn generic_jaro_winkler<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
```



## strsim::generic_levenshtein

*Function*

Calculates the minimum number of insertions, deletions, and substitutions
required to change one sequence into the other.

```
use strsim::generic_levenshtein;

assert_eq!(3, generic_levenshtein(&[1,2,3], &[1,2,3,4,5,6]));
```

```rust
fn generic_levenshtein<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> usize
```



## strsim::hamming

*Function*

Calculates the number of positions in the two strings where the characters
differ. Returns an error if the strings have different lengths.

```
use strsim::{hamming, StrSimError::DifferentLengthArgs};

assert_eq!(Ok(3), hamming("hamming", "hammers"));

assert_eq!(Err(DifferentLengthArgs), hamming("hamming", "ham"));
```

```rust
fn hamming(a: &str, b: &str) -> HammingResult
```



## strsim::jaro

*Function*

Calculates the Jaro similarity between two strings. The returned value
is between 0.0 and 1.0 (higher value means more similar).

```
use strsim::jaro;

assert!((0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() <
        0.001);
```

```rust
fn jaro(a: &str, b: &str) -> f64
```



## strsim::jaro_winkler

*Function*

Like Jaro but gives a boost to strings that have a common prefix.

```
use strsim::jaro_winkler;

assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
        0.001);
```

```rust
fn jaro_winkler(a: &str, b: &str) -> f64
```



## strsim::levenshtein

*Function*

Calculates the minimum number of insertions, deletions, and substitutions
required to change one string into the other.

```
use strsim::levenshtein;

assert_eq!(3, levenshtein("kitten", "sitting"));
```

```rust
fn levenshtein(a: &str, b: &str) -> usize
```



## strsim::normalized_damerau_levenshtein

*Function*

Calculates a normalized score of the Damerau–Levenshtein algorithm between
0.0 and 1.0 (inclusive), where 1.0 means the strings are the same.

```
use strsim::normalized_damerau_levenshtein;

assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.27272).abs() < 0.00001);
assert!((normalized_damerau_levenshtein("", "") - 1.0).abs() < 0.00001);
assert!(normalized_damerau_levenshtein("", "flower").abs() < 0.00001);
assert!(normalized_damerau_levenshtein("tree", "").abs() < 0.00001);
assert!((normalized_damerau_levenshtein("sunglasses", "sunglasses") - 1.0).abs() < 0.00001);
```

```rust
fn normalized_damerau_levenshtein(a: &str, b: &str) -> f64
```



## strsim::normalized_levenshtein

*Function*

Calculates a normalized score of the Levenshtein algorithm between 0.0 and
1.0 (inclusive), where 1.0 means the strings are the same.

```
use strsim::normalized_levenshtein;

assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
assert!((normalized_levenshtein("", "") - 1.0).abs() < 0.00001);
assert!(normalized_levenshtein("", "second").abs() < 0.00001);
assert!(normalized_levenshtein("first", "").abs() < 0.00001);
assert!((normalized_levenshtein("string", "string") - 1.0).abs() < 0.00001);
```

```rust
fn normalized_levenshtein(a: &str, b: &str) -> f64
```



## strsim::osa_distance

*Function*

Like Levenshtein but allows for adjacent transpositions. Each substring can
only be edited once.

```
use strsim::osa_distance;

assert_eq!(3, osa_distance("ab", "bca"));
```

```rust
fn osa_distance(a: &str, b: &str) -> usize
```



## strsim::sorensen_dice

*Function*

Calculates a Sørensen-Dice similarity distance using bigrams.
See <https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient>.

```
use strsim::sorensen_dice;

assert_eq!(1.0, sorensen_dice("", ""));
assert_eq!(0.0, sorensen_dice("", "a"));
assert_eq!(0.0, sorensen_dice("french", "quebec"));
assert_eq!(1.0, sorensen_dice("ferris", "ferris"));
assert_eq!(0.8888888888888888, sorensen_dice("feris", "ferris"));
```

```rust
fn sorensen_dice(a: &str, b: &str) -> f64
```



