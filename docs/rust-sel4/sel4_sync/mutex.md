**sel4_sync > mutex**

# Module: mutex

## Contents

**Structs**

- [`NotificationAlreadySetError`](#notificationalreadyseterror)
- [`RawDeferredNotificationMutex`](#rawdeferrednotificationmutex)
- [`RawLazyNotificationMutex`](#rawlazynotificationmutex)
- [`RawNotificationMutex`](#rawnotificationmutex)

---

## sel4_sync::mutex::NotificationAlreadySetError

*Struct*

**Tuple Struct**: `()`



## sel4_sync::mutex::RawDeferredNotificationMutex

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self`
- `fn set_notification(self: &Self, nfn: sel4::cap::Notification) -> Result<(), NotificationAlreadySetError>`

**Trait Implementations:**

- **RawMutex**
  - `fn lock(self: &Self)`
  - `fn try_lock(self: &Self) -> bool`
  - `fn unlock(self: &Self)`
- **Default**
  - `fn default() -> Self`



## sel4_sync::mutex::RawLazyNotificationMutex

*Struct*

**Generic Parameters:**
- F

**Tuple Struct**: `()`

**Methods:**

- `fn new(f: F) -> Self`

**Trait Implementations:**

- **RawMutex**
  - `fn lock(self: &Self)`
  - `fn try_lock(self: &Self) -> bool`
  - `fn unlock(self: &Self)`



## sel4_sync::mutex::RawNotificationMutex

*Struct*

**Generic Parameters:**
- _T

**Methods:**

- `fn new(nfn: sel4::cap::Notification) -> Self`

**Trait Implementations:**

- **RawMutex**
  - `fn lock(self: &Self)`
  - `fn try_lock(self: &Self) -> bool`
  - `fn unlock(self: &Self)`



