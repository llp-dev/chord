**syn > thread**

# Module: thread

## Contents

**Structs**

- [`ThreadBound`](#threadbound) - ThreadBound is a Sync-maker and Send-maker that allows accessing a value

---

## syn::thread::ThreadBound

*Struct*

ThreadBound is a Sync-maker and Send-maker that allows accessing a value
of type T only from the original thread on which the ThreadBound was
constructed.

**Generic Parameters:**
- T

**Fields:**
- `value: T`
- `thread_id: std::thread::ThreadId`

**Methods:**

- `fn new(value: T) -> Self`
- `fn get(self: &Self) -> Option<&T>`

**Traits:** Copy, Sync, Send

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



