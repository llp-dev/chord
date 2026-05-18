*[heapless](../index.md) / [c_string](index.md)*

---

# Module `c_string`

A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CString`](#cstring) | struct | A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html). |
| [`ExtendError`](#extenderror) | enum | An error to extend [`CString`] with bytes. |

## Structs

### `CString<const N: usize, LenT: LenType>`

```rust
struct CString<const N: usize, LenT: LenType> {
    inner: crate::vec::Vec<u8, N, LenT>,
}
```

A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).

It stores up to `N - 1` non-nul characters with a trailing nul terminator.

#### Implementations

- <span id="cstring-new"></span>`fn new() -> Self`

  Creates a new C-compatible string with a terminating nul byte.

  

  ```rust

  use heapless::CString;

  

  // A fixed-size `CString` that can store up to 10 characters

  // including the nul terminator.

  let empty = CString::<10>::new();

  

  assert_eq!(empty.as_c_str(), c"");

  assert_eq!(empty.to_str(), Ok(""));

  ```

- <span id="cstring-from-bytes-with-nul-unchecked"></span>`unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> Result<Self, CapacityError>` — [`CapacityError`](../index.md#capacityerror)

  Unsafely creates a [`CString`](#cstring) from a byte slice.

  

  This function will copy the provided `bytes` to a [`CString`](#cstring) without

  performing any sanity checks.

  

  The function will fail if `bytes.len() > N`.

  

  # Safety

  

  The provided slice **must** be nul-terminated and not contain any interior

  nul bytes.

  

  # Examples

  

  ```rust

  use heapless::CString;

  let mut c_string = unsafe { CString::<7>::from_bytes_with_nul_unchecked(b"string\0").unwrap() };

  

  assert_eq!(c_string.to_str(), Ok("string"));

  ```

- <span id="cstring-from-bytes-with-nul"></span>`fn from_bytes_with_nul(bytes: &[u8]) -> Result<Self, ExtendError>` — [`ExtendError`](#extenderror)

  Instantiates a [`CString`](#cstring) copying from the giving byte slice, assuming it is

  nul-terminated.

  

  Fails if the given byte slice has any interior nul byte, if the slice does not

  end with a nul byte, or if the byte slice can't fit in `N`.

- <span id="cstring-from-raw"></span>`unsafe fn from_raw(ptr: *const c_char) -> Result<Self, ExtendError>` — [`ExtendError`](#extenderror)

  Builds a [`CString`](#cstring) copying from a raw C string pointer.

  

  # Safety

  

  - The memory pointed to by `ptr` must contain a valid nul terminator at the

    end of the string.

  - `ptr` must be valid for reads of bytes up to and including the nul terminator.

    This means in particular:

      - The entire memory range of this `CStr` must be contained within a single allocated object!

      - `ptr` must be non-nul even for a zero-length `CStr`.

  

  # Example

  

  ```rust

  use core::ffi::{c_char, CStr};

  use heapless::CString;

  

  const HELLO_PTR: *const c_char = {

      const BYTES: &[u8] = b"Hello, world!\0";

      BYTES.as_ptr().cast()

  };

  

  let copied = unsafe { CString::<14>::from_raw(HELLO_PTR) }.unwrap();

  

  assert_eq!(copied.to_str(), Ok("Hello, world!"));

  ```

- <span id="cstring-as-c-str"></span>`fn as_c_str(&self) -> &CStr`

  Converts the [`CString`](#cstring) to a [`CStr`](../../serde_core/lib/index.md) slice.

- <span id="cstring-capacity-with-bytes"></span>`fn capacity_with_bytes(&self, bytes: &[u8]) -> Option<usize>`

  Calculates the length of `self.inner` would have if it appended `bytes`.

- <span id="cstring-extend-from-bytes"></span>`fn extend_from_bytes(&mut self, bytes: &[u8]) -> Result<(), ExtendError>` — [`ExtendError`](#extenderror)

  Extends the [`CString`](#cstring) with the given bytes.

  

  This function fails if the [`CString`](#cstring) would not have enough capacity to append the bytes or

  if the bytes contain an interior nul byte.

  

  # Example

  

  ```rust

  use heapless::CString;

  

  let mut c_string = CString::<10>::new();

  

  c_string.extend_from_bytes(b"hey").unwrap();

  c_string.extend_from_bytes(b" there\0").unwrap();

  

  assert_eq!(c_string.to_str(), Ok("hey there"));

  ```

- <span id="cstring-pop-terminator"></span>`unsafe fn pop_terminator(&mut self)`

  Removes the nul byte terminator from the inner buffer.

  

  # Safety

  

  Callers must ensure to add the nul terminator back after this function is called.

- <span id="cstring-extend-from-bytes-unchecked"></span>`unsafe fn extend_from_bytes_unchecked(&mut self, additional: &[u8]) -> Result<(), CapacityError>` — [`CapacityError`](../index.md#capacityerror)

  Removes the existing nul terminator and then extends `self` with the given bytes.

  

  # Safety

  

  If `additional` is not nul-terminated, the [`CString`](#cstring) is left non nul-terminated, which is

  an invalid state. Caller must ensure that either `additional` has a terminating nul byte

  or ensure to append a trailing nul terminator.

- <span id="cstring-as-bytes-with-nul"></span>`fn as_bytes_with_nul(&self) -> &[u8]`

  Returns the underlying byte slice including the trailing nul terminator.

  

  # Example

  

  ```rust

  use heapless::CString;

  

  let mut c_string = CString::<5>::new();

  c_string.extend_from_bytes(b"abc").unwrap();

  

  assert_eq!(c_string.as_bytes_with_nul(), b"abc\0");

  ```

- <span id="cstring-as-bytes"></span>`fn as_bytes(&self) -> &[u8]`

  Returns the underlying byte slice excluding the trailing nul terminator.

  

  # Example

  

  ```rust

  use heapless::CString;

  

  let mut c_string = CString::<5>::new();

  c_string.extend_from_bytes(b"abc").unwrap();

  

  assert_eq!(c_string.as_bytes(), b"abc");

  ```

#### Trait Implementations

##### `impl<LenT: LenType> AsRef for CString<N, LenT>`

- <span id="cstring-asref-as-ref"></span>`fn as_ref(&self) -> &CStr`

##### `impl<LenT: clone::Clone + LenType> Clone for CString<N, LenT>`

- <span id="cstring-clone"></span>`fn clone(&self) -> CString<N, LenT>` — [`CString`](#cstring)

##### `impl<LenT: LenType> Debug for CString<N, LenT>`

- <span id="cstring-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<LenT: LenType> Default for CString<N, LenT>`

- <span id="cstring-default"></span>`fn default() -> Self`

##### `impl<LenT: LenType> Deref for CString<N, LenT>`

- <span id="cstring-deref-type-target"></span>`type Target = CStr`

- <span id="cstring-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<LenT: LenType> Eq for CString<N, LenT>`

##### `impl<LenT: hash::Hash + LenType> Hash for CString<N, LenT>`

- <span id="cstring-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<LenT: LenType> Ord for CString<N, LenT>`

- <span id="cstring-ord-cmp"></span>`fn cmp(&self, rhs: &Self) -> Ordering`

##### `impl<LenT1: LenType, LenT2: LenType> PartialEq for CString<N, LenT1>`

- <span id="cstring-partialeq-eq"></span>`fn eq(&self, rhs: &CString<M, LenT2>) -> bool` — [`CString`](#cstring)

##### `impl<LenT1: LenType, LenT2: LenType> PartialOrd for CString<N, LenT1>`

- <span id="cstring-partialord-partial-cmp"></span>`fn partial_cmp(&self, rhs: &CString<M, LenT2>) -> Option<Ordering>` — [`CString`](#cstring)

##### `impl Receiver for CString<N, LenT>`

- <span id="cstring-receiver-type-target"></span>`type Target = T`

## Enums

### `ExtendError`

```rust
enum ExtendError {
    Capacity(crate::CapacityError),
    InteriorNul {
        position: usize,
    },
}
```

An error to extend [`CString`](#cstring) with bytes.

#### Variants

- **`Capacity`**

  The capacity of the [`CString`](#cstring) is too small.

- **`InteriorNul`**

  An invalid interior nul byte found in a given byte slice.

#### Trait Implementations

##### `impl Debug for ExtendError`

- <span id="extenderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ExtendError`

- <span id="extenderror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ExtendError`

##### `impl ToString for ExtendError`

- <span id="extenderror-tostring-to-string"></span>`fn to_string(&self) -> String`

