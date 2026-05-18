**syn > bigint**

# Module: bigint

## Contents

**Structs**

- [`BigInt`](#bigint)

---

## syn::bigint::BigInt

*Struct*

**Fields:**
- `digits: alloc::vec::Vec<u8>`

**Methods:**

- `fn new() -> Self`
- `fn to_string(self: &Self) -> String`
- `fn reserve_two_digits(self: & mut Self)`

**Trait Implementations:**

- **MulAssign**
  - `fn mul_assign(self: & mut Self, base: u8)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, increment: u8)`



