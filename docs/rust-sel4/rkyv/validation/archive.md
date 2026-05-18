**rkyv > validation > archive**

# Module: validation::archive

## Contents

**Traits**

- [`ArchiveContext`](#archivecontext) - A context that can validate nonlocal archive memory.
- [`ArchiveContextExt`](#archivecontextext) - Helper methods for [`ArchiveContext`].

---

## rkyv::validation::archive::ArchiveContext

*Trait*

A context that can validate nonlocal archive memory.

# Safety

`check_subtree_ptr` must only return true if `ptr` is located entirely
within the subtree range and is safe to dereference.

**Methods:**

- `check_subtree_ptr`: Checks that the given data address and layout is located completely
- `push_subtree_range`: Pushes a new subtree range onto the validator and starts validating it.
- `pop_subtree_range`: Pops the given range, restoring the original state with the pushed range



## rkyv::validation::archive::ArchiveContextExt

*Trait*

Helper methods for [`ArchiveContext`].

**Methods:**

- `in_subtree_raw`: Checks that the given pointer and layout are within the current subtree
- `in_subtree`: Checks that the value the given pointer points to is within the current



