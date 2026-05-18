**sel4_async_unsync > mutex**

# Module: mutex

## Contents

**Structs**

- [`Guard`](#guard)
- [`Mutex`](#mutex)

---

## sel4_async_unsync::mutex::Guard

*Struct*

**Generic Parameters:**
- 'a
- T

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`



## sel4_async_unsync::mutex::Mutex

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(data: T) -> Self`
- `fn lock(self: &Self) -> Guard<T>`

**Traits:** Send, Sync



