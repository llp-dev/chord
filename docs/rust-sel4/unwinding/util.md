**unwinding > util**

# Module: util

## Contents

**Functions**

- [`deref_pointer`](#deref_pointer)
- [`get_unlimited_slice`](#get_unlimited_slice)

**Type Aliases**

- [`StaticSlice`](#staticslice)
- [`c_int`](#c_int)

---

## unwinding::util::StaticSlice

*Type Alias*: `gimli::EndianSlice<'static, gimli::NativeEndian>`



## unwinding::util::c_int

*Type Alias*: `i32`



## unwinding::util::deref_pointer

*Function*

```rust
fn deref_pointer(ptr: gimli::Pointer) -> usize
```



## unwinding::util::get_unlimited_slice

*Function*

```rust
fn get_unlimited_slice<'a>(start: *const u8) -> &'a [u8]
```



