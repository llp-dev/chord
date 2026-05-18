**bitfield_macros**

# Module: bitfield_macros

## Contents

**Proc Macros**

- [`bitfield_constructor`](#bitfield_constructor) - Implements an exhaustive constructor function for a bitfield. Should only be called by `bitfield!` when using `impl new;`
- [`bitfield_debug`](#bitfield_debug) - Generates a `fmt::Debug` implementation.
- [`bitfield_fields`](#bitfield_fields) - Declares the fields of a struct.

---

## bitfield_macros::bitfield_constructor

*Function-like Macro*

Implements an exhaustive constructor function for a bitfield. Should only be called by `bitfield!` when using `impl new;`

```rust
bitfield_constructor!(...)
```



## bitfield_macros::bitfield_debug

*Function-like Macro*

Generates a `fmt::Debug` implementation.

This macros must be called from a `impl Debug for ...` block. It will generate the `fmt` method.

In most of the case, you will not directly call this macros, but use `bitfield`.

The syntax is `struct TheNameOfTheStruct;` followed by the syntax of `bitfield_fields`.

The write-only fields are ignored.

# Example

```ignore
struct FooBar(u32);
bitfield_bitrange!{struct FooBar(u32)}
impl FooBar{
    bitfield_fields!{
       u32;
       field1, _: 7, 0;
       field2, _: 31, 24;
    }
}

impl std::fmt::Debug for FooBar {
    bitfield_debug!{
       struct FooBar;
       field1, _: 7, 0;
       field2, _: 31, 24;
    }
}

fn main() {
    let foobar = FooBar(0x11223344);
    println!("{:?}", foobar);
}
```

```rust
bitfield_debug!(...)
```



## bitfield_macros::bitfield_fields

*Function-like Macro*

Declares the fields of a struct.

This macro will generate the methods to access the fields of a bitfield. It must be called
from an `impl` block for a type that implements the `BitRange` and/or the `Bit` traits
(which traits are required depending on what type of fields are used).

The syntax of this macro is composed of declarations ended by semicolons. There are two types
of declarations: default type, and fields.

A default type is just a type followed by a semicolon. This will affect all the following field
declarations.

A field declaration is composed of the following:

* Optional attributes (`#[...]`), documentation comments (`///`) are attributes;
* An optional pub keyword to make the methods public
* An optional type followed by a comma
* Optionnaly, the word `mask` followed by an identifier and an type in parentheses, followed by
  a comma
* Optionally, the word `from` and/or (`into` or `try_into`) followed by a type, followed by a comma
* The getter and setter idents, separated by a comma
* A colon
* One to three expressions of type `usize`

The attributes and pub will be applied to the two methods generated.

If the `into` part is used, the getter will convert the field after reading it.
If the `try_into` part is used, the getter will try to convert the field after reading it.

The getter and setter idents can be `_` to not generate one of the two. For example, if the
setter is `_`, the field will be read-only.

The expressions at the end are the bit positions. Their meaning depends on the number of
expressions:

 * One expression: the field is a single bit. The type is ignored and `bool` is used. The trait
   `Bit` is used.
 * Two expressions: `msb, lsb`, the field is composed of the bits from `msb` to `lsb`, included.
 * Three expressions: `msb, lsb, count`, the field is an array. The first element is composed of
   the bits from `msb` to `lsb`. The following elements are consecutive bits range of the same
   size.

# Example

```ignore
# use bitfield_macros::bitfield_fields;
# fn main() {}
# struct FooBar(u64);
# bitfield_bitrange!{struct FooBar(u64)}
# impl From<u32> for FooBar{ fn from(_: u32) -> FooBar {unimplemented!()}}
# impl From<FooBar> for u32{ fn from(_: FooBar) -> u32 {unimplemented!()}}
# impl FooBar {
bitfield_fields!{
    // The default type will be `u64
    u64;
    // filed1 is read-write, public, the methods are inline
    #[inline]
    pub field1, set_field1: 10, 0;
    // `field2` is  read-only, private, and of type bool.
    field2, _ : 0;
    // `field3` will be read as an `u32` and then converted to `FooBar`.
    // The setter is not affected, it still need an `u32` value.
    u32, into FooBar, field3, set_field3: 10, 0;
    // `field4` will be read as an `u32` and then converted to `FooBar`.
    // The setter will take a `FooBar`, and converted back to an `u32`.
    u32, from into FooBar, field4, set_field4: 10, 0;
    // `field5` will be read as an `u32` and then converted to `FooBar`.
    // The setter will take a `FooBar`, and converted back to an `u32`.
    // The struct will have an associated constant `FIELD5_MASK` of type u64
    //with the bits of field5 set
    u32, mask FIELD5_MASK(u64), from into FooBar, field5, set_field5: 10, 0;
}
# }
```

```rust
bitfield_fields!(...)
```



