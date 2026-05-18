**bytemuck > contiguous**

# Module: contiguous

## Contents

**Traits**

- [`Contiguous`](#contiguous) - A trait indicating that:

---

## bytemuck::contiguous::Contiguous

*Trait*

A trait indicating that:

1. A type has an equivalent representation to some known integral type.
2. All instances of this type fall in a fixed range of values.
3. Within that range, there are no gaps.

This is generally useful for fieldless enums (aka "c-style" enums), however
it's important that it only be used for those with an explicit `#[repr]`, as
`#[repr(Rust)]` fieldess enums have an unspecified layout.

Additionally, you shouldn't assume that all implementations are enums. Any
type which meets the requirements above while following the rules under
"Safety" below is valid.

# Example

```
# use bytemuck::Contiguous;
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Foo {
  A = 0,
  B = 1,
  C = 2,
  D = 3,
  E = 4,
}
unsafe impl Contiguous for Foo {
  type Int = u8;
  const MIN_VALUE: u8 = Foo::A as u8;
  const MAX_VALUE: u8 = Foo::E as u8;
}
assert_eq!(Foo::from_integer(3).unwrap(), Foo::D);
assert_eq!(Foo::from_integer(8), None);
assert_eq!(Foo::C.into_integer(), 2);
```
# Safety

This is an unsafe trait, and incorrectly implementing it is undefined
behavior.

Informally, by implementing it, you're asserting that `C` is identical to
the integral type `C::Int`, and that every `C` falls between `C::MIN_VALUE`
and `C::MAX_VALUE` exactly once, without any gaps.

Precisely, the guarantees you must uphold when implementing `Contiguous` for
some type `C` are:

1. The size of `C` and `C::Int` must be the same, and neither may be a ZST.
   (Note: alignment is explicitly allowed to differ)

2. `C::Int` must be a primitive integer, and not a wrapper type. In the
   future, this may be lifted to include cases where the behavior is
   identical for a relevant set of traits (Ord, arithmetic, ...).

3. All `C::Int`s which are in the *inclusive* range between `C::MIN_VALUE`
   and `C::MAX_VALUE` are bitwise identical to unique valid instances of
   `C`.

4. There exist no instances of `C` such that their bitpatterns, when
   interpreted as instances of `C::Int`, fall outside of the `MAX_VALUE` /
   `MIN_VALUE` range -- It is legal for unsafe code to assume that if it
   gets a `C` that implements `Contiguous`, it is in the appropriate range.

5. Finally, you promise not to provide overridden implementations of
   `Contiguous::from_integer` and `Contiguous::into_integer`.

For clarity, the following rules could be derived from the above, but are
listed explicitly:

- `C::MAX_VALUE` must be greater or equal to `C::MIN_VALUE` (therefore, `C`
  must be an inhabited type).

- There exist no two values between `MIN_VALUE` and `MAX_VALUE` such that
  when interpreted as a `C` they are considered identical (by, say, match).

**Methods:**

- `Int`: The primitive integer type with an identical representation to this
- `MAX_VALUE`: The upper *inclusive* bound for valid instances of this type.
- `MIN_VALUE`: The lower *inclusive* bound for valid instances of this type.
- `from_integer`: If `value` is within the range for valid instances of this type,
- `into_integer`: Perform the conversion from `C` into the underlying integral type. This



