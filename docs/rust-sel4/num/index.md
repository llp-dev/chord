# num

A collection of numeric types and traits for Rust.

This includes new types for big integers, rationals, and complex numbers,
new traits for generic programming on numeric properties like `Integer`,
and generic range iterators.

## Example

This example uses the BigRational type and [Newton's method][newt] to
approximate a square root to arbitrary precision:

```
# #[cfg(any(feature = "alloc", feature = "std"))]
# mod test {

use num::FromPrimitive;
use num::bigint::BigInt;
use num::rational::{Ratio, BigRational};

# pub
fn approx_sqrt(number: u64, iterations: usize) -> BigRational {
    let start: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(number).unwrap());
    let mut approx = start.clone();

    for _ in 0..iterations {
        approx = (&approx + (&start / &approx)) /
            Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
    }

    approx
}
# }
# #[cfg(not(any(feature = "alloc", feature = "std")))]
# mod test { pub fn approx_sqrt(n: u64, _: usize) -> u64 { n } }
# use crate::test::approx_sqrt;

fn main() {
    println!("{}", approx_sqrt(10, 4)); // prints 4057691201/1283082416
}

```

[newt]: https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method

## Compatibility

The `num` crate is tested for rustc 1.60 and greater.

## Modules

### [`num`](num.md)

*6 modules*

