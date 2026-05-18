**one_shot_mutex**

# Module: one_shot_mutex

## Contents

**Modules**

- [`sync`](#sync) - One-shot lock variants that implement `Sync`.
- [`unsync`](#unsync) - One-shot lock variants that do not implement `Sync`.

---

## Module: sync

One-shot lock variants that implement `Sync`.



## Module: unsync

One-shot lock variants that do not implement `Sync`.

These one-shot locks not implement `Sync`, which permits slightly more efficient
implementations.

For variants that do implement `Sync`, see the [`sync`](crate::sync) module.



