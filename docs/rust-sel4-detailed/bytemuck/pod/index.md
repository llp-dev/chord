*[bytemuck](../index.md) / [pod](index.md)*

---

# Module `pod`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pod`](#pod) | trait | Marker trait for "plain old data". |

## Traits

### `Pod`

```rust
trait Pod: Zeroable + Copy + 'static { ... }
```

Marker trait for "plain old data".

The point of this trait is that once something is marked "plain old data"
you can really go to town with the bit fiddling and bit casting. Therefore,
it's a relatively strong claim to make about a type. Do not add this to your
type casually.

**Reminder:** The results of casting around bytes between data types are
_endian dependant_. Little-endian machines are the most common, but
big-endian machines do exist (and big-endian is also used for "network
order" bytes).

## Safety

* The type must be inhabited (eg: no
  [Infallible](core::convert::Infallible)).
* The type must allow any bit pattern (eg: no `bool` or `char`, which have
  illegal bit patterns).
* The type must not contain any uninit (or padding) bytes, either in the
  middle or on the end (eg: no `#[repr(C)] struct Foo(u8, u16)`, which has
  padding in the middle, and also no `#[repr(C)] struct Foo(u16, u8)`, which
  has padding on the end).
* The type needs to have all fields also be `Pod`.
* The type needs to be `repr(C)` or `repr(transparent)`. In the case of
  `repr(C)`, the `packed` and `align` repr modifiers can be used as long as
  all other rules end up being followed.
* It is disallowed for types to contain pointer types, `Cell`, `UnsafeCell`,
  atomics, and any other forms of interior mutability.
* More precisely: A shared reference to the type must allow reads, and
  *only* reads. RustBelt's separation logic is based on the notion that a
  type is allowed to define a sharing predicate, its own invariant that must
  hold for shared references, and this predicate is the reasoning that allow
  it to deal with atomic and cells etc. We require the sharing predicate to
  be trivial and permit only read-only access.

#### Implementors

- `()`
- `Option<T>`
- `PhantomData<T>`
- `PhantomPinned`
- `Wrapping<T>`
- `[T; 0]`
- `[T; 1024]`
- `[T; 10]`
- `[T; 11]`
- `[T; 128]`
- `[T; 12]`
- `[T; 13]`
- `[T; 14]`
- `[T; 15]`
- `[T; 16]`
- `[T; 17]`
- `[T; 18]`
- `[T; 19]`
- `[T; 1]`
- `[T; 2048]`
- `[T; 20]`
- `[T; 21]`
- `[T; 22]`
- `[T; 23]`
- `[T; 24]`
- `[T; 256]`
- `[T; 25]`
- `[T; 26]`
- `[T; 27]`
- `[T; 28]`
- `[T; 29]`
- `[T; 2]`
- `[T; 30]`
- `[T; 31]`
- `[T; 32]`
- `[T; 3]`
- `[T; 4096]`
- `[T; 48]`
- `[T; 4]`
- `[T; 512]`
- `[T; 5]`
- `[T; 64]`
- `[T; 6]`
- `[T; 7]`
- `[T; 8]`
- `[T; 96]`
- `[T; 9]`
- `core::mem::ManuallyDrop<T>`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`
- `x86_64::__m128`
- `x86_64::__m128d`
- `x86_64::__m128i`
- `x86_64::__m256`
- `x86_64::__m256d`
- `x86_64::__m256i`

