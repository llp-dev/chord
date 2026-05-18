**bytemuck > pod_in_option**

# Module: pod_in_option

## Contents

**Traits**

- [`PodInOption`](#podinoption) - Trait for types which are [Pod](Pod) when wrapped in

---

## bytemuck::pod_in_option::PodInOption

*Trait*

Trait for types which are [Pod](Pod) when wrapped in
[Option](core::option::Option).

## Safety

* `Option<T>` must uphold the same invariants as [Pod](Pod).
* **Reminder:** pointers are **not** pod! **Do not** mix this trait with a
  newtype over [NonNull](core::ptr::NonNull).



