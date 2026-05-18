**smoltcp > wire > icmp**

# Module: wire::icmp

## Contents

**Enums**

- [`Repr`](#repr)

---

## smoltcp::wire::icmp::Repr

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `Ipv4(icmpv4::Repr<'a>)`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Repr<'a>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Repr<'a>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(s: icmpv4::Repr<'a>) -> Self`



