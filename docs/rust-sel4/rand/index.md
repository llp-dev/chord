# rand

Utilities for random number generation

Rand provides utilities to generate random numbers, to convert them to
useful types and distributions, and some randomness-related algorithms.

# Quick Start

```
// The prelude import enables methods we use below, specifically
// Rng::random, Rng::sample, SliceRandom::shuffle and IndexedRandom::choose.
use rand::prelude::*;

// Get an RNG:
let mut rng = rand::rng();

// Try printing a random unicode code point (probably a bad idea)!
println!("char: '{}'", rng.random::<char>());
// Try printing a random alphanumeric value instead!
println!("alpha: '{}'", rng.sample(rand::distr::Alphanumeric) as char);

// Generate and shuffle a sequence:
let mut nums: Vec<i32> = (1..100).collect();
nums.shuffle(&mut rng);
// And take a random pick (yes, we didn't need to shuffle first!):
let _ = nums.choose(&mut rng);
```

# The Book

For the user guide and further documentation, please read
[The Rust Rand Book](https://rust-random.github.io/book).

## Modules

### [`rand`](rand.md)

*4 modules*

### [`distr`](distr.md)

*1 struct, 2 modules*

### [`distr::bernoulli`](distr/bernoulli.md)

*1 enum, 1 struct*

### [`distr::distribution`](distr/distribution.md)

*1 trait, 2 structs*

### [`distr::float`](distr/float.md)

*2 structs*

### [`distr::other`](distr/other.md)

*2 structs*

### [`distr::slice`](distr/slice.md)

*2 structs*

### [`distr::uniform`](distr/uniform.md)

*1 enum, 1 struct, 4 traits*

### [`distr::uniform::float`](distr/uniform/float.md)

*1 struct*

### [`distr::uniform::int`](distr/uniform/int.md)

*2 structs*

### [`distr::uniform::other`](distr/uniform/other.md)

*2 structs*

### [`rng`](rng.md)

*2 traits*

### [`rngs::small`](rngs/small.md)

*1 struct*

### [`rngs::xoshiro128plusplus`](rngs/xoshiro128plusplus.md)

*1 struct*

### [`rngs::xoshiro256plusplus`](rngs/xoshiro256plusplus.md)

*1 struct*

### [`seq`](seq.md)

*1 module*

### [`seq::index`](seq/index.md)

*1 function*

### [`seq::iterator`](seq/iterator.md)

*1 trait*

### [`seq::slice`](seq/slice.md)

*3 traits*

