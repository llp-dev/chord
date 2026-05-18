**simd_adler32**

# Module: simd_adler32

## Contents

**Structs**

- [`Adler32`](#adler32) - An adler32 hash generator type.

**Functions**

- [`adler32`](#adler32) - Compute Adler-32 hash on `Adler32Hash` type.

**Traits**

- [`Adler32Hash`](#adler32hash) - A Adler-32 hash-able type.

---

## simd_adler32::Adler32

*Struct*

An adler32 hash generator type.

**Methods:**

- `fn new() -> Self` - Constructs a new `Adler32`.
- `fn from_checksum(checksum: u32) -> Self` - Constructs a new `Adler32` using existing checksum.
- `fn write(self: & mut Self, data: &[u8])` - Computes hash for supplied data and stores results in internal state.
- `fn finish(self: &Self) -> u32` - Returns the hash value for the values written so far.
- `fn reset(self: & mut Self)` - Resets the internal state.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Adler32`



## simd_adler32::Adler32Hash

*Trait*

A Adler-32 hash-able type.

**Methods:**

- `hash`: Feeds this value into `Adler32`.



## simd_adler32::adler32

*Function*

Compute Adler-32 hash on `Adler32Hash` type.

# Arguments
* `hash` - A Adler-32 hash-able type.

# Examples
```rust
use simd_adler32::adler32;

let hash = adler32(b"Adler-32");
println!("{}", hash); // 800813569
```

```rust
fn adler32<H>(hash: &H) -> u32
```



