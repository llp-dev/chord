**bytemuck > zeroable_in_option**

# Module: zeroable_in_option

## Contents

**Traits**

- [`ZeroableInOption`](#zeroableinoption) - Trait for types which are [Zeroable](Zeroable) when wrapped in

---

## bytemuck::zeroable_in_option::ZeroableInOption

*Trait*

Trait for types which are [Zeroable](Zeroable) when wrapped in
[Option](core::option::Option).

## Safety

* `Option<YourType>` must uphold the same invariants as
  [Zeroable](Zeroable).



