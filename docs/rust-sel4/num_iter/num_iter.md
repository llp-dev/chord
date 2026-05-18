**num_iter**

# Module: num_iter

## Contents

**Structs**

- [`Range`](#range) - An iterator over the range [start, stop)
- [`RangeFrom`](#rangefrom) - An iterator over the infinite range starting at `start`
- [`RangeInclusive`](#rangeinclusive) - An iterator over the range [start, stop]
- [`RangeStep`](#rangestep) - An iterator over the range [start, stop) by `step`. It handles overflow by stopping.
- [`RangeStepFrom`](#rangestepfrom) - An iterator over the infinite range starting at `start` by `step`
- [`RangeStepInclusive`](#rangestepinclusive) - An iterator over the range [start, stop] by `step`. It handles overflow by stopping.

**Functions**

- [`range`](#range) - Returns an iterator over the given range [start, stop) (that is, starting
- [`range_from`](#range_from) - Return an iterator over the infinite range starting at `start` and continuing forever.
- [`range_inclusive`](#range_inclusive) - Return an iterator over the range [start, stop]
- [`range_step`](#range_step) - Return an iterator over the range [start, stop) by `step`. It handles overflow by stopping.
- [`range_step_from`](#range_step_from) - Return an iterator over the infinite range starting at `start` and continuing forever by `step`.
- [`range_step_inclusive`](#range_step_inclusive) - Return an iterator over the range [start, stop] by `step`. It handles overflow by stopping.

---

## num_iter::Range

*Struct*

An iterator over the range [start, stop)

**Generic Parameters:**
- A

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Range<A>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<A>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **RangeBounds**
  - `fn start_bound(self: &Self) -> Bound<&A>`
  - `fn end_bound(self: &Self) -> Bound<&A>`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<A>`



## num_iter::RangeFrom

*Struct*

An iterator over the infinite range starting at `start`

**Generic Parameters:**
- A

**Trait Implementations:**

- **RangeBounds**
  - `fn start_bound(self: &Self) -> Bound<&A>`
  - `fn end_bound(self: &Self) -> Bound<&A>`
- **Clone**
  - `fn clone(self: &Self) -> RangeFrom<A>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<A>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## num_iter::RangeInclusive

*Struct*

An iterator over the range [start, stop]

**Generic Parameters:**
- A

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<A>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<A>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **RangeBounds**
  - `fn start_bound(self: &Self) -> Bound<&A>`
  - `fn end_bound(self: &Self) -> Bound<&A>`
- **Clone**
  - `fn clone(self: &Self) -> RangeInclusive<A>`



## num_iter::RangeStep

*Struct*

An iterator over the range [start, stop) by `step`. It handles overflow by stopping.

**Generic Parameters:**
- A

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RangeStep<A>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<A>`



## num_iter::RangeStepFrom

*Struct*

An iterator over the infinite range starting at `start` by `step`

**Generic Parameters:**
- A

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RangeStepFrom<A>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<A>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## num_iter::RangeStepInclusive

*Struct*

An iterator over the range [start, stop] by `step`. It handles overflow by stopping.

**Generic Parameters:**
- A

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<A>`
- **Clone**
  - `fn clone(self: &Self) -> RangeStepInclusive<A>`



## num_iter::range

*Function*

Returns an iterator over the given range [start, stop) (that is, starting
at start (inclusive), and ending at stop (exclusive)).

# Example

```rust
let array = [0, 1, 2, 3, 4];

for i in num_iter::range(0, 5) {
    println!("{}", i);
    assert_eq!(i,  array[i]);
}
```

```rust
fn range<A>(start: A, stop: A) -> Range<A>
```



## num_iter::range_from

*Function*

Return an iterator over the infinite range starting at `start` and continuing forever.

*Note*: Currently, the `Iterator` implementation is not checked for overflow.
If you use a finite-sized integer type and the integer overflows,
it might panic in debug mode or wrap around in release mode.
**This behavior is not guaranteed and may change at any time.**

```rust
fn range_from<A>(start: A) -> RangeFrom<A>
```



## num_iter::range_inclusive

*Function*

Return an iterator over the range [start, stop]

```rust
fn range_inclusive<A>(start: A, stop: A) -> RangeInclusive<A>
```



## num_iter::range_step

*Function*

Return an iterator over the range [start, stop) by `step`. It handles overflow by stopping.

```rust
fn range_step<A>(start: A, stop: A, step: A) -> RangeStep<A>
```



## num_iter::range_step_from

*Function*

Return an iterator over the infinite range starting at `start` and continuing forever by `step`.

*Note*: Currently, the `Iterator` implementation is not checked for overflow.
If you use a finite-sized integer type and the integer overflows,
it might panic in debug mode or wrap around in release mode.
**This behavior is not guaranteed and may change at any time.**

```rust
fn range_step_from<A>(start: A, step: A) -> RangeStepFrom<A>
```



## num_iter::range_step_inclusive

*Function*

Return an iterator over the range [start, stop] by `step`. It handles overflow by stopping.

```rust
fn range_step_inclusive<A>(start: A, stop: A, step: A) -> RangeStepInclusive<A>
```



