**gimli > read > util**

# Module: read::util

## Contents

**Traits**

- [`ArrayLike`](#arraylike) - Marker trait for types that can be used as backing storage when a growable array type is needed.

---

## gimli::read::util::ArrayLike

*Trait*

Marker trait for types that can be used as backing storage when a growable array type is needed.

This trait is sealed and cannot be implemented for types outside this crate.

**Methods:**

- `Item`: Type of the elements being stored.



