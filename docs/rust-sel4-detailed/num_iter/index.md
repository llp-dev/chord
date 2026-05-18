# Crate `num_iter`

External iterators for generic mathematics

## Compatibility

The `num-iter` crate is tested for rustc 1.31 and greater.

## Contents

- [Structs](#structs)
  - [`Range`](#range)
  - [`RangeInclusive`](#rangeinclusive)
  - [`RangeStep`](#rangestep)
  - [`RangeStepInclusive`](#rangestepinclusive)
  - [`RangeFrom`](#rangefrom)
  - [`RangeStepFrom`](#rangestepfrom)
- [Functions](#functions)
  - [`range`](#range)
  - [`unsigned`](#unsigned)
  - [`range_inclusive`](#range-inclusive)
  - [`range_step`](#range-step)
  - [`range_step_inclusive`](#range-step-inclusive)
  - [`range_from`](#range-from)
  - [`range_step_from`](#range-step-from)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Range`](#range) | struct | An iterator over the range [start, stop) |
| [`RangeInclusive`](#rangeinclusive) | struct | An iterator over the range [start, stop] |
| [`RangeStep`](#rangestep) | struct | An iterator over the range [start, stop) by `step`. |
| [`RangeStepInclusive`](#rangestepinclusive) | struct | An iterator over the range [start, stop] by `step`. |
| [`RangeFrom`](#rangefrom) | struct | An iterator over the infinite range starting at `start` |
| [`RangeStepFrom`](#rangestepfrom) | struct | An iterator over the infinite range starting at `start` by `step` |
| [`range`](#range) | fn | Returns an iterator over the given range [start, stop) (that is, starting at start (inclusive), and ending at stop (exclusive)). |
| [`unsigned`](#unsigned) | fn |  |
| [`range_inclusive`](#range-inclusive) | fn | Return an iterator over the range [start, stop] |
| [`range_step`](#range-step) | fn | Return an iterator over the range [start, stop) by `step`. |
| [`range_step_inclusive`](#range-step-inclusive) | fn | Return an iterator over the range [start, stop] by `step`. |
| [`range_from`](#range-from) | fn | Return an iterator over the infinite range starting at `start` and continuing forever. |
| [`range_step_from`](#range-step-from) | fn | Return an iterator over the infinite range starting at `start` and continuing forever by `step`. |

## Structs

### `Range<A>`

```rust
struct Range<A> {
    state: A,
    stop: A,
    one: A,
}
```

An iterator over the range [start, stop)

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for Range<A>`

- <span id="range-clone"></span>`fn clone(&self) -> Range<A>` — [`Range`](#range)

##### `impl<A> DoubleEndedIterator for Range<A>`

- <span id="range-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<A>`

##### `impl IntoIterator for Range<A>`

- <span id="range-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="range-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="range-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A> Iterator for Range<A>`

- <span id="range-iterator-type-item"></span>`type Item = A`

- <span id="range-iterator-next"></span>`fn next(&mut self) -> Option<A>`

- <span id="range-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<A> RangeBounds for Range<A>`

- <span id="range-rangebounds-start-bound"></span>`fn start_bound(&self) -> Bound<&A>`

- <span id="range-rangebounds-end-bound"></span>`fn end_bound(&self) -> Bound<&A>`

### `RangeInclusive<A>`

```rust
struct RangeInclusive<A> {
    range: Range<A>,
    done: bool,
}
```

An iterator over the range [start, stop]

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for RangeInclusive<A>`

- <span id="rangeinclusive-clone"></span>`fn clone(&self) -> RangeInclusive<A>` — [`RangeInclusive`](#rangeinclusive)

##### `impl<A> DoubleEndedIterator for RangeInclusive<A>`

- <span id="rangeinclusive-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<A>`

##### `impl IntoIterator for RangeInclusive<A>`

- <span id="rangeinclusive-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rangeinclusive-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rangeinclusive-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A> Iterator for RangeInclusive<A>`

- <span id="rangeinclusive-iterator-type-item"></span>`type Item = A`

- <span id="rangeinclusive-iterator-next"></span>`fn next(&mut self) -> Option<A>`

- <span id="rangeinclusive-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<A> RangeBounds for RangeInclusive<A>`

- <span id="rangeinclusive-rangebounds-start-bound"></span>`fn start_bound(&self) -> Bound<&A>`

- <span id="rangeinclusive-rangebounds-end-bound"></span>`fn end_bound(&self) -> Bound<&A>`

### `RangeStep<A>`

```rust
struct RangeStep<A> {
    state: A,
    stop: A,
    step: A,
    rev: bool,
}
```

An iterator over the range [start, stop) by `step`. It handles overflow by stopping.

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for RangeStep<A>`

- <span id="rangestep-clone"></span>`fn clone(&self) -> RangeStep<A>` — [`RangeStep`](#rangestep)

##### `impl IntoIterator for RangeStep<A>`

- <span id="rangestep-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rangestep-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rangestep-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A> Iterator for RangeStep<A>`

- <span id="rangestep-iterator-type-item"></span>`type Item = A`

- <span id="rangestep-iterator-next"></span>`fn next(&mut self) -> Option<A>`

### `RangeStepInclusive<A>`

```rust
struct RangeStepInclusive<A> {
    state: A,
    stop: A,
    step: A,
    rev: bool,
    done: bool,
}
```

An iterator over the range [start, stop] by `step`. It handles overflow by stopping.

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for RangeStepInclusive<A>`

- <span id="rangestepinclusive-clone"></span>`fn clone(&self) -> RangeStepInclusive<A>` — [`RangeStepInclusive`](#rangestepinclusive)

##### `impl IntoIterator for RangeStepInclusive<A>`

- <span id="rangestepinclusive-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rangestepinclusive-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rangestepinclusive-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A> Iterator for RangeStepInclusive<A>`

- <span id="rangestepinclusive-iterator-type-item"></span>`type Item = A`

- <span id="rangestepinclusive-iterator-next"></span>`fn next(&mut self) -> Option<A>`

### `RangeFrom<A>`

```rust
struct RangeFrom<A> {
    state: A,
    one: A,
}
```

An iterator over the infinite range starting at `start`

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for RangeFrom<A>`

- <span id="rangefrom-clone"></span>`fn clone(&self) -> RangeFrom<A>` — [`RangeFrom`](#rangefrom)

##### `impl IntoIterator for RangeFrom<A>`

- <span id="rangefrom-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rangefrom-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rangefrom-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A> Iterator for RangeFrom<A>`

- <span id="rangefrom-iterator-type-item"></span>`type Item = A`

- <span id="rangefrom-iterator-next"></span>`fn next(&mut self) -> Option<A>`

- <span id="rangefrom-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<A> RangeBounds for RangeFrom<A>`

- <span id="rangefrom-rangebounds-start-bound"></span>`fn start_bound(&self) -> Bound<&A>`

- <span id="rangefrom-rangebounds-end-bound"></span>`fn end_bound(&self) -> Bound<&A>`

### `RangeStepFrom<A>`

```rust
struct RangeStepFrom<A> {
    state: A,
    step: A,
}
```

An iterator over the infinite range starting at `start` by `step`

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for RangeStepFrom<A>`

- <span id="rangestepfrom-clone"></span>`fn clone(&self) -> RangeStepFrom<A>` — [`RangeStepFrom`](#rangestepfrom)

##### `impl IntoIterator for RangeStepFrom<A>`

- <span id="rangestepfrom-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rangestepfrom-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rangestepfrom-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A> Iterator for RangeStepFrom<A>`

- <span id="rangestepfrom-iterator-type-item"></span>`type Item = A`

- <span id="rangestepfrom-iterator-next"></span>`fn next(&mut self) -> Option<A>`

- <span id="rangestepfrom-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Functions

### `range`

```rust
fn range<A>(start: A, stop: A) -> Range<A>
where
    A: Add<A, Output = A> + PartialOrd + Clone + One
```

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

### `unsigned`

```rust
fn unsigned<T: ToPrimitive>(x: &T) -> Option<u128>
```

### `range_inclusive`

```rust
fn range_inclusive<A>(start: A, stop: A) -> RangeInclusive<A>
where
    A: Add<A, Output = A> + PartialOrd + Clone + One
```

Return an iterator over the range [start, stop]

### `range_step`

```rust
fn range_step<A>(start: A, stop: A, step: A) -> RangeStep<A>
where
    A: CheckedAdd + PartialOrd + Clone + Zero
```

Return an iterator over the range [start, stop) by `step`. It handles overflow by stopping.

### `range_step_inclusive`

```rust
fn range_step_inclusive<A>(start: A, stop: A, step: A) -> RangeStepInclusive<A>
where
    A: CheckedAdd + PartialOrd + Clone + Zero
```

Return an iterator over the range [start, stop] by `step`. It handles overflow by stopping.

### `range_from`

```rust
fn range_from<A>(start: A) -> RangeFrom<A>
where
    A: Add<A, Output = A> + Clone + One
```

Return an iterator over the infinite range starting at `start` and continuing forever.

*Note*: Currently, the `Iterator` implementation is not checked for overflow.
If you use a finite-sized integer type and the integer overflows,
it might panic in debug mode or wrap around in release mode.
**This behavior is not guaranteed and may change at any time.**

### `range_step_from`

```rust
fn range_step_from<A>(start: A, step: A) -> RangeStepFrom<A>
where
    A: Add<A, Output = A> + Clone
```

Return an iterator over the infinite range starting at `start` and continuing forever by `step`.

*Note*: Currently, the `Iterator` implementation is not checked for overflow.
If you use a finite-sized integer type and the integer overflows,
it might panic in debug mode or wrap around in release mode.
**This behavior is not guaranteed and may change at any time.**

