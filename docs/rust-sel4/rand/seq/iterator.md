**rand > seq > iterator**

# Module: seq::iterator

## Contents

**Traits**

- [`IteratorRandom`](#iteratorrandom) - Extension trait on iterators, providing random sampling methods.

---

## rand::seq::iterator::IteratorRandom

*Trait*

Extension trait on iterators, providing random sampling methods.

This trait is implemented on all iterators `I` where `I: Iterator + Sized`
and provides methods for
choosing one or more elements. You must `use` this trait:

```
use rand::seq::IteratorRandom;

let faces = "😀😎😐😕😠😢";
println!("I am {}!", faces.chars().choose(&mut rand::rng()).unwrap());
```
Example output (non-deterministic):
```none
I am 😀!
```

**Methods:**

- `choose`: Uniformly sample one element
- `choose_stable`: Uniformly sample one element (stable)
- `sample_fill`: Uniformly sample `amount` distinct elements into a buffer
- `choose_multiple_fill`: Deprecated: use [`Self::sample_fill`] instead



