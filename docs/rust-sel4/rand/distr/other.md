**rand > distr > other**

# Module: distr::other

## Contents

**Structs**

- [`Alphabetic`](#alphabetic) - Sample a [`u8`], uniformly distributed over letters:
- [`Alphanumeric`](#alphanumeric) - Sample a `u8`, uniformly distributed over ASCII letters and numbers:

---

## rand::distr::other::Alphabetic

*Struct*

Sample a [`u8`], uniformly distributed over letters:
a-z and A-Z.

# Example

You're able to generate random Alphabetic characters via mapping or via the
[`SampleString::sample_string`] method like so:

```
use rand::RngExt;
use rand::distr::{Alphabetic, SampleString};

// Manual mapping
let mut rng = rand::rng();
let chars: String = (0..7).map(|_| rng.sample(Alphabetic) as char).collect();
println!("Random chars: {}", chars);

// Using [`SampleString::sample_string`]
let string = Alphabetic.sample_string(&mut rand::rng(), 16);
println!("Random string: {}", string);
```

# Passwords

Refer to [`Alphanumeric#Passwords`].

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> u8`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Alphabetic`
- **Default**
  - `fn default() -> Alphabetic`



## rand::distr::other::Alphanumeric

*Struct*

Sample a `u8`, uniformly distributed over ASCII letters and numbers:
a-z, A-Z and 0-9.

# Example

```
use rand::RngExt;
use rand::distr::Alphanumeric;

let mut rng = rand::rng();
let chars: String = (0..7).map(|_| rng.sample(Alphanumeric) as char).collect();
println!("Random chars: {}", chars);
```

The [`SampleString`] trait provides an easier method of generating
a random [`String`], and offers more efficient allocation:
```
use rand::distr::{Alphanumeric, SampleString};
let string = Alphanumeric.sample_string(&mut rand::rng(), 16);
println!("Random string: {}", string);
```

# Passwords

Users sometimes ask whether it is safe to use a string of random characters
as a password. In principle, all RNGs in Rand implementing `CryptoRng` are
suitable as a source of randomness for generating passwords (if they are
properly seeded), but it is more conservative to only use randomness
directly from the operating system via the `getrandom` crate, or the
corresponding bindings of a crypto library.

When generating passwords or keys, it is important to consider the threat
model and in some cases the memorability of the password. This is out of
scope of the Rand project, and therefore we defer to the following
references:

- [Wikipedia article on Password Strength](https://en.wikipedia.org/wiki/Password_strength)
- [Diceware for generating memorable passwords](https://en.wikipedia.org/wiki/Diceware)

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Alphanumeric`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> u8`
- **Default**
  - `fn default() -> Alphanumeric`



