**sel4_shared_ring_buffer_bookkeeping > slot_count_tracker**

# Module: slot_count_tracker

## Contents

**Structs**

- [`SlotCountTracker`](#slotcounttracker)

**Enums**

- [`SlotCountTrackerError`](#slotcounttrackererror)

---

## sel4_shared_ring_buffer_bookkeeping::slot_count_tracker::SlotCountTracker

*Struct*

**Methods:**

- `fn new(initial_num_free: usize) -> Self`
- `fn report_occupied(self: & mut Self, count: usize) -> Result<(), SlotCountTrackerError>`
- `fn redeem_newly_free(self: & mut Self, current_num_free: usize) -> Result<usize, SlotCountTrackerError>`



## sel4_shared_ring_buffer_bookkeeping::slot_count_tracker::SlotCountTrackerError

*Enum*

**Variants:**
- `CurrentNumFreeLessThanStoredNumFree`
- `ReportOccupiedCountGreaterThanStoredNumFree`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



