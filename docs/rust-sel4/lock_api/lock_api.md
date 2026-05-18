**lock_api**

# Module: lock_api

## Contents

**Structs**

- [`GuardNoSend`](#guardnosend) - Marker type which indicates that the Guard type for a lock is not `Send`.
- [`GuardSend`](#guardsend) - Marker type which indicates that the Guard type for a lock is `Send`.

---

## lock_api::GuardNoSend

*Struct*

Marker type which indicates that the Guard type for a lock is not `Send`.

**Tuple Struct**: `()`

**Traits:** Sync



## lock_api::GuardSend

*Struct*

Marker type which indicates that the Guard type for a lock is `Send`.

**Tuple Struct**: `()`



