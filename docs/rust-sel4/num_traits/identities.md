**num_traits > identities**

# Module: identities

## Contents

**Functions**

- [`one`](#one) - Returns the multiplicative identity, `1`.
- [`zero`](#zero) - Returns the additive identity, `0`.

**Traits**

- [`ConstOne`](#constone) - Defines an associated constant representing the multiplicative identity
- [`ConstZero`](#constzero) - Defines an associated constant representing the additive identity element
- [`One`](#one) - Defines a multiplicative identity element for `Self`.
- [`Zero`](#zero) - Defines an additive identity element for `Self`.

---

## num_traits::identities::ConstOne

*Trait*

Defines an associated constant representing the multiplicative identity
element for `Self`.

**Methods:**

- `ONE`: The multiplicative identity element of `Self`, `1`.



## num_traits::identities::ConstZero

*Trait*

Defines an associated constant representing the additive identity element
for `Self`.

**Methods:**

- `ZERO`: The additive identity element of `Self`, `0`.



## num_traits::identities::One

*Trait*

Defines a multiplicative identity element for `Self`.

# Laws

```text
a * 1 = a       ∀ a ∈ Self
1 * a = a       ∀ a ∈ Self
```

**Methods:**

- `one`: Returns the multiplicative identity element of `Self`, `1`.
- `set_one`: Sets `self` to the multiplicative identity element of `Self`, `1`.
- `is_one`: Returns `true` if `self` is equal to the multiplicative identity.



## num_traits::identities::Zero

*Trait*

Defines an additive identity element for `Self`.

# Laws

```text
a + 0 = a       ∀ a ∈ Self
0 + a = a       ∀ a ∈ Self
```

**Methods:**

- `zero`: Returns the additive identity element of `Self`, `0`.
- `set_zero`: Sets `self` to the additive identity element of `Self`, `0`.
- `is_zero`: Returns `true` if `self` is equal to the additive identity.



## num_traits::identities::one

*Function*

Returns the multiplicative identity, `1`.

```rust
fn one<T>() -> T
```



## num_traits::identities::zero

*Function*

Returns the additive identity, `0`.

```rust
fn zero<T>() -> T
```



