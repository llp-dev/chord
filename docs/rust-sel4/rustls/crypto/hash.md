**rustls > crypto > hash**

# Module: crypto::hash

## Contents

**Structs**

- [`Output`](#output) - A hash output, stored as a value.

**Traits**

- [`Context`](#context) - How to incrementally compute a hash.
- [`Hash`](#hash) - Describes a single cryptographic hash function.

---

## rustls::crypto::hash::Context

*Trait*

How to incrementally compute a hash.

**Methods:**

- `fork_finish`: Finish the computation, returning the resulting output.
- `fork`: Fork the computation, producing another context that has the
- `finish`: Terminate and finish the computation, returning the resulting output.
- `update`: Add `data` to computation.



## rustls::crypto::hash::Hash

*Trait*

Describes a single cryptographic hash function.

This interface can do both one-shot and incremental hashing, using
[`Hash::hash()`] and [`Hash::start()`] respectively.

**Methods:**

- `start`: Start an incremental hash computation.
- `hash`: Return the output of this hash function with input `data`.
- `output_len`: The length in bytes of this hash function's output.
- `algorithm`: Which hash function this is, eg, `HashAlgorithm::SHA256`.
- `fips`: Return `true` if this is backed by a FIPS-approved implementation.



## rustls::crypto::hash::Output

*Struct*

A hash output, stored as a value.

**Methods:**

- `fn new(bytes: &[u8]) -> Self` - Build a `hash::Output` from a slice of no more than `Output::MAX_LEN` bytes.

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



