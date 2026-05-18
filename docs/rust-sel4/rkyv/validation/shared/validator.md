**rkyv > validation > shared > validator**

# Module: validation::shared::validator

## Contents

**Structs**

- [`SharedValidator`](#sharedvalidator) - A validator that can verify shared pointers.

---

## rkyv::validation::shared::validator::SharedValidator

*Struct*

A validator that can verify shared pointers.

**Methods:**

- `fn new() -> Self` - Creates a new shared pointer validator.
- `fn with_capacity(capacity: usize) -> Self` - Creates a new shared pointer validator with specific capacity.

**Trait Implementations:**

- **Default**
  - `fn default() -> SharedValidator`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **SharedContext**
  - `fn start_shared(self: & mut Self, address: usize, type_id: TypeId) -> Result<ValidationState, E>`
  - `fn finish_shared(self: & mut Self, address: usize, type_id: TypeId) -> Result<(), E>`



