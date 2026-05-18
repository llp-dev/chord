**sel4_immediate_sync_once_cell**

# Module: sel4_immediate_sync_once_cell

## Contents

**Structs**

- [`ImmediateSyncOnceCell`](#immediatesynconcecell)

---

## sel4_immediate_sync_once_cell::ImmediateSyncOnceCell

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self`
- `fn get(self: &Self) -> Option<&T>`
- `fn set(self: &Self, value: T) -> Result<(), T>`

**Traits:** Sync

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



