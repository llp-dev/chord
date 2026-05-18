**sel4_shared_memory**

# Module: sel4_shared_memory

## Contents

**Structs**

- [`SharedMemory`](#sharedmemory)

**Type Aliases**

- [`SharedMemoryPtr`](#sharedmemoryptr)
- [`SharedMemoryRef`](#sharedmemoryref)

---

## sel4_shared_memory::SharedMemory

*Struct*

**Tuple Struct**: `()`

**Traits:** MemoryType

**Trait Implementations:**

- **UnitaryOps**
  - `fn read(src: *const T) -> T`
  - `fn write(dst: *mut T, src: T)`
- **BulkOps**
  - `fn memmove(dst: *mut T, src: *const T, count: usize)`
  - `fn memcpy_into(dst: *mut T, src: *const T, count: usize)`
  - `fn memcpy_from(dst: *mut T, src: *const T, count: usize)`
  - `fn memset(dst: *mut T, val: u8, count: usize)`



## sel4_shared_memory::SharedMemoryPtr

*Type Alias*: `sel4_abstract_ptr::AbstractPtr<'a, SharedMemory, T, A>`



## sel4_shared_memory::SharedMemoryRef

*Type Alias*: `sel4_abstract_ptr::AbstractRef<'a, SharedMemory, T, A>`



