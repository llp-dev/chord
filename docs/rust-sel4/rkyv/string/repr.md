**rkyv > string > repr**

# Module: string::repr

## Contents

**Unions**

- [`ArchivedStringRepr`](#archivedstringrepr) - An archived string representation that can inline short strings.

**Constants**

- [`INLINE_CAPACITY`](#inline_capacity) - The maximum number of bytes that can be inlined.
- [`OUT_OF_LINE_CAPACITY`](#out_of_line_capacity) - The maximum number of bytes that can be out-of-line.

---

## rkyv::string::repr::ArchivedStringRepr

*Union*

An archived string representation that can inline short strings.

**Methods:**

- `fn is_inline(self: &Self) -> bool` - Returns whether the representation is inline.
- `fn out_of_line_offset(self: &Self) -> isize` - Returns the offset of the representation.
- `fn as_ptr(self: &Self) -> *const u8` - Returns a pointer to the bytes of the string.
- `fn as_mut_ptr(this: Seal<Self>) -> *mut u8` - Returns a mutable pointer to the bytes of the string.
- `fn len(self: &Self) -> usize` - Returns the length of the string.
- `fn is_empty(self: &Self) -> bool` - Returns whether the string is empty.
- `fn as_str_ptr(self: &Self) -> *const str` - Returns a pointer to the string as a `str`.
- `fn as_bytes(self: &Self) -> &[u8]` - Returns a slice of the bytes of the string.
- `fn as_bytes_seal(this: Seal<Self>) -> Seal<[u8]>` - Returns a mutable slice of the bytes of the string.
- `fn as_str(self: &Self) -> &str` - Returns a reference to the string as a `str`.
- `fn as_str_seal(this: Seal<Self>) -> Seal<str>` - Returns a mutable reference to the string as a `str`.
- `fn emplace_inline(value: &str, out: *mut Self)` - Emplaces a new inline representation for the given `str`.
- `fn try_emplace_out_of_line<E>(value: &str, target: usize, out: Place<Self>) -> Result<(), E>` - Emplaces a new out-of-line representation for the given `str`.
- `fn emplace_out_of_line(value: &str, target: usize, out: Place<Self>)` - Emplaces a new out-of-line representation for the given `str`.

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, _: & mut C) -> Result<(), <C as >::Error>`



## rkyv::string::repr::INLINE_CAPACITY

*Constant*: `usize`

The maximum number of bytes that can be inlined.



## rkyv::string::repr::OUT_OF_LINE_CAPACITY

*Constant*: `usize`

The maximum number of bytes that can be out-of-line.



