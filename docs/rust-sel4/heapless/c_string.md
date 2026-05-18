**heapless > c_string**

# Module: c_string

## Contents

**Structs**

- [`CString`](#cstring) - A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).

**Enums**

- [`ExtendError`](#extenderror) - An error to extend [`CString`] with bytes.

---

## heapless::c_string::CString

*Struct*

A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).

It stores up to `N - 1` non-nul characters with a trailing nul terminator.

**Generic Parameters:**
- const N
- LenT

**Methods:**

- `fn new() -> Self` - Creates a new C-compatible string with a terminating nul byte.
- `fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> Result<Self, CapacityError>` - Unsafely creates a [`CString`] from a byte slice.
- `fn from_bytes_with_nul(bytes: &[u8]) -> Result<Self, ExtendError>` - Instantiates a [`CString`] copying from the giving byte slice, assuming it is
- `fn from_raw(ptr: *const c_char) -> Result<Self, ExtendError>` - Builds a [`CString`] copying from a raw C string pointer.
- `fn as_c_str(self: &Self) -> &CStr` - Converts the [`CString`] to a [`CStr`] slice.
- `fn extend_from_bytes(self: & mut Self, bytes: &[u8]) -> Result<(), ExtendError>` - Extends the [`CString`] with the given bytes.
- `fn as_bytes_with_nul(self: &Self) -> &[u8]` - Returns the underlying byte slice including the trailing nul terminator.
- `fn as_bytes(self: &Self) -> &[u8]` - Returns the underlying byte slice excluding the trailing nul terminator.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, rhs: &CString<M, LenT2>) -> bool`
- **Borrow**
  - `fn borrow(self: &Self) -> &CStr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, rhs: &CString<M, LenT2>) -> Option<Ordering>`
- **Ord**
  - `fn cmp(self: &Self, rhs: &Self) -> Ordering`
- **Clone**
  - `fn clone(self: &Self) -> CString<N, LenT>`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Default**
  - `fn default() -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **AsRef**
  - `fn as_ref(self: &Self) -> &CStr`



## heapless::c_string::ExtendError

*Enum*

An error to extend [`CString`] with bytes.

**Variants:**
- `Capacity(crate::CapacityError)` - The capacity of the [`CString`] is too small.
- `InteriorNul{ position: usize }` - An invalid interior nul byte found in a given byte slice.

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(error: CapacityError) -> Self`



