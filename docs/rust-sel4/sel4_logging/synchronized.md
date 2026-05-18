**sel4_logging > synchronized**

# Module: synchronized

## Contents

**Structs**

- [`SynchronizedLogger`](#synchronizedlogger)

---

## sel4_logging::synchronized::SynchronizedLogger

*Struct*

**Generic Parameters:**
- R
- T

**Tuple Struct**: `()`

**Methods:**

- `fn new_with_raw_mutex(raw_mutex: R, inner: T) -> Self`
- `fn into_inner(self: Self) -> Mutex<R, T>`
- `fn inner(self: &Self) -> &Mutex<R, T>`
- `fn inner_mut(self: & mut Self) -> & mut Mutex<R, T>`
- `fn new(inner: T) -> Self`

**Trait Implementations:**

- **Log**
  - `fn enabled(self: &Self, metadata: &Metadata) -> bool`
  - `fn log(self: &Self, record: &Record)`
  - `fn flush(self: &Self)`



