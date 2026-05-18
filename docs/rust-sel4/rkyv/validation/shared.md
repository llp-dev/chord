**rkyv > validation > shared**

# Module: validation::shared

## Contents

**Enums**

- [`ValidationState`](#validationstate) - The result of starting to validate a shared pointer.

**Traits**

- [`SharedContext`](#sharedcontext) - A context that can validate shared archive memory.

---

## rkyv::validation::shared::SharedContext

*Trait*

A context that can validate shared archive memory.

Shared pointers require this kind of context to validate.

**Methods:**

- `start_shared`: Starts validating the value associated with the given address.
- `finish_shared`: Finishes validating the value associated with the given address.



## rkyv::validation::shared::ValidationState

*Enum*

The result of starting to validate a shared pointer.

**Variants:**
- `Started` - The caller started validating this value. They should proceed to check
- `Pending` - Another caller started validating this value, but has not finished yet.
- `Finished` - This value has already been validated.



