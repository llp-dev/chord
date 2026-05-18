**object > pod**

# Module: pod

## Contents

**Functions**

- [`bytes_of`](#bytes_of) - Cast a `Pod` type to a byte slice.
- [`bytes_of_mut`](#bytes_of_mut) - Cast a `Pod` type to a mutable byte slice.
- [`bytes_of_slice`](#bytes_of_slice) - Cast a slice of a `Pod` type to a byte slice.
- [`bytes_of_slice_mut`](#bytes_of_slice_mut) - Cast a slice of a `Pod` type to a mutable byte slice.
- [`from_bytes`](#from_bytes) - Cast the head of a byte slice to a `Pod` type.
- [`from_bytes_mut`](#from_bytes_mut) - Cast the head of a mutable byte slice to a `Pod` type.
- [`slice_from_all_bytes`](#slice_from_all_bytes) - Cast all of a byte slice to a slice of a `Pod` type.
- [`slice_from_all_bytes_mut`](#slice_from_all_bytes_mut) - Cast all of a byte slice to a slice of a `Pod` type.
- [`slice_from_bytes`](#slice_from_bytes) - Cast the head of a byte slice to a slice of a `Pod` type.
- [`slice_from_bytes_mut`](#slice_from_bytes_mut) - Cast the head of a mutable byte slice to a slice of a `Pod` type.

**Traits**

- [`Pod`](#pod) - A trait for types that can safely be converted from and to byte slices.

---

## object::pod::Pod

*Trait*

A trait for types that can safely be converted from and to byte slices.

# Safety
A type that is `Pod` must:
- be `#[repr(C)]` or `#[repr(transparent)]`
- have no invalid byte values
- have no padding



## object::pod::bytes_of

*Function*

Cast a `Pod` type to a byte slice.

```rust
fn bytes_of<T>(val: &T) -> &[u8]
```



## object::pod::bytes_of_mut

*Function*

Cast a `Pod` type to a mutable byte slice.

```rust
fn bytes_of_mut<T>(val: & mut T) -> & mut [u8]
```



## object::pod::bytes_of_slice

*Function*

Cast a slice of a `Pod` type to a byte slice.

```rust
fn bytes_of_slice<T>(val: &[T]) -> &[u8]
```



## object::pod::bytes_of_slice_mut

*Function*

Cast a slice of a `Pod` type to a mutable byte slice.

```rust
fn bytes_of_slice_mut<T>(val: & mut [T]) -> & mut [u8]
```



## object::pod::from_bytes

*Function*

Cast the head of a byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

```rust
fn from_bytes<T>(data: &[u8]) -> result::Result<(&T, &[u8]), ()>
```



## object::pod::from_bytes_mut

*Function*

Cast the head of a mutable byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

```rust
fn from_bytes_mut<T>(data: & mut [u8]) -> result::Result<(& mut T, & mut [u8]), ()>
```



## object::pod::slice_from_all_bytes

*Function*

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

```rust
fn slice_from_all_bytes<T>(data: &[u8]) -> result::Result<&[T], ()>
```



## object::pod::slice_from_all_bytes_mut

*Function*

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

```rust
fn slice_from_all_bytes_mut<T>(data: & mut [u8]) -> result::Result<& mut [T], ()>
```



## object::pod::slice_from_bytes

*Function*

Cast the head of a byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

```rust
fn slice_from_bytes<T>(data: &[u8], count: usize) -> result::Result<(&[T], &[u8]), ()>
```



## object::pod::slice_from_bytes_mut

*Function*

Cast the head of a mutable byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

```rust
fn slice_from_bytes_mut<T>(data: & mut [u8], count: usize) -> result::Result<(& mut [T], & mut [u8]), ()>
```



