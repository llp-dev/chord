**safe_mmio > physical**

# Module: physical

## Contents

**Structs**

- [`PhysicalInstance`](#physicalinstance) - The physical instance of some device's MMIO space.

---

## safe_mmio::physical::PhysicalInstance

*Struct*

The physical instance of some device's MMIO space.

**Generic Parameters:**
- T

**Methods:**

- `fn new(pa: usize) -> Self` - # Safety
- `fn pa(self: &Self) -> usize` - Returns the physical base address of the device's registers.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`



