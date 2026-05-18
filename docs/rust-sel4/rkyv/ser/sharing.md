**rkyv > ser > sharing**

# Module: ser::sharing

## Contents

**Enums**

- [`SharingState`](#sharingstate) - The result of starting to serialize a shared pointer.

**Traits**

- [`Sharing`](#sharing) - A shared pointer serialization strategy.
- [`SharingExt`](#sharingext) - Helper methods for [`Sharing`].

---

## rkyv::ser::sharing::Sharing

*Trait*

A shared pointer serialization strategy.

This trait is required to serialize `Rc` and `Arc`.

**Methods:**

- `start_sharing`: Starts sharing the value associated with the given address.
- `finish_sharing`: Finishes sharing the value associated with the given address.



## rkyv::ser::sharing::SharingExt

*Trait*

Helper methods for [`Sharing`].

**Methods:**

- `serialize_shared`: Serializes the given shared value and returns its position. If the value



## rkyv::ser::sharing::SharingState

*Enum*

The result of starting to serialize a shared pointer.

**Variants:**
- `Started` - The caller started sharing this value. They should proceed to serialize
- `Pending` - Another caller started sharing this value, but has not finished yet.
- `Finished(usize)` - This value has already been shared. The caller should use the returned



