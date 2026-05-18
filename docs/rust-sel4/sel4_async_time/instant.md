**sel4_async_time > instant**

# Module: instant

## Contents

**Structs**

- [`Instant`](#instant)

---

## sel4_async_time::instant::Instant

*Struct*

**Methods:**

- `fn new(since_zero: Duration) -> Self`
- `fn since_zero(self: &Self) -> Duration`
- `fn checked_duration_since(self: &Self, earlier: Instant) -> Option<Duration>`
- `fn saturating_duration_since(self: &Self, earlier: Instant) -> Duration`
- `fn checked_add(self: &Self, duration: Duration) -> Option<Instant>`
- `fn checked_sub(self: &Self, duration: Duration) -> Option<Instant>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Instant) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Sub**
  - `fn sub(self: Self, other: Instant) -> Duration`
- **Add**
  - `fn add(self: Self, other: Duration) -> Instant`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Duration)`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Instant) -> bool`
- **Sub**
  - `fn sub(self: Self, other: Duration) -> Instant`
- **Ord**
  - `fn cmp(self: &Self, other: &Instant) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Instant`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: Duration)`



