# Crate `paste`

[![github]](https://github.com/dtolnay/paste)&ensp;[![crates-io]](https://crates.io/crates/paste)&ensp;[![docs-rs]](https://docs.rs/paste)



<br>

The nightly-only `concat_idents!` macro in the Rust standard library is
notoriously underpowered in that its concatenated identifiers can only refer to
existing items, they can never be used to define something new.

This crate provides a flexible way to paste together identifiers in a macro,
including using pasted identifiers to define new items.

This approach works with any Rust compiler 1.31+.

<br>

# Pasting identifiers

Within the `paste!` macro, identifiers inside `[<`...`>]` are pasted
together to form a single identifier.

```rust
use paste::paste;

paste! {
    // Defines a const called `QRST`.
    const [<Q R S T>]: &str = "success!";
}

fn main() {
    assert_eq!(
        paste! { [<Q R S T>].len() },
        8,
    );
}
```

<br><br>

# More elaborate example

The next example shows a macro that generates accessor methods for some
struct fields. It demonstrates how you might find it useful to bundle a
paste invocation inside of a macro\_rules macro.

```rust
use paste::paste;

macro_rules! make_a_struct_and_getters {
    ($name:ident { $($field:ident),* }) => {
        // Define a struct. This expands to:
        //
        //     pub struct S {
        //         a: String,
        //         b: String,
        //         c: String,
        //     }
        pub struct $name {
            $(
                $field: String,
            )*
        }

        // Build an impl block with getters. This expands to:
        //
        //     impl S {
        //         pub fn get_a(&self) -> &str { &self.a }
        //         pub fn get_b(&self) -> &str { &self.b }
        //         pub fn get_c(&self) -> &str { &self.c }
        //     }
        paste! {
            impl $name {
                $(
                    pub fn [<get_ $field>](&self) -> &str {
                        &self.$field
                    }
                )*
            }
        }
    }
}

make_a_struct_and_getters!(S { a, b, c });

fn call_some_getters(s: &S) -> bool {
    s.get_a() == s.get_b() && s.get_c().is_empty()
}

fn main() {}
```

<br><br>

# Case conversion

Use `$var:lower` or `$var:upper` in the segment list to convert an
interpolated segment to lower- or uppercase as part of the paste. For
example, `[<ld_ $reg:lower _expr>]` would paste to `ld_bc_expr` if invoked
with $reg=`Bc`.

Use `$var:snake` to convert CamelCase input to snake\_case.
Use `$var:camel` to convert snake\_case to CamelCase.
These compose, so for example `$var:snake:upper` would give you SCREAMING\_CASE.

The precise Unicode conversions are as defined by `str::to_lowercase` and
`str::to_uppercase`.


<br>

# Pasting documentation strings

Within the `paste!` macro, arguments to a #\[doc ...\] attribute are
implicitly concatenated together to form a coherent documentation string.

```rust
use paste::paste;

macro_rules! method_new {
    ($ret:ident) => {
        paste! {
            #[doc = "Create a new `" $ret "` object."]
            pub fn new() -> $ret { todo!() }
        }
    };
}

pub struct Paste {}

method_new!(Paste);  // expands to #[doc = "Create a new `Paste` object"]
```

## Contents

- [Modules](#modules)
  - [`attr`](#attr)
  - [`error`](#error)
  - [`segment`](#segment)
- [Enums](#enums)
  - [`Lookbehind`](#lookbehind)
- [Functions](#functions)
  - [`expand`](#expand)
  - [`is_single_interpolation_group`](#is-single-interpolation-group)
  - [`is_paste_operation`](#is-paste-operation)
  - [`parse_bracket_as_segments`](#parse-bracket-as-segments)
  - [`pasted_to_tokens`](#pasted-to-tokens)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`attr`](#attr) | mod |  |
| [`error`](#error) | mod |  |
| [`segment`](#segment) | mod |  |
| [`Lookbehind`](#lookbehind) | enum |  |
| [`expand`](#expand) | fn |  |
| [`is_single_interpolation_group`](#is-single-interpolation-group) | fn |  |
| [`is_paste_operation`](#is-paste-operation) | fn |  |
| [`parse_bracket_as_segments`](#parse-bracket-as-segments) | fn |  |
| [`pasted_to_tokens`](#pasted-to-tokens) | fn |  |

## Modules

- [`attr`](attr/index.md)
- [`error`](error/index.md)
- [`segment`](segment/index.md)

## Enums

### `Lookbehind`

```rust
enum Lookbehind {
    JointColon,
    DoubleColon,
    Pound,
    PoundBang,
    Other,
}
```

#### Trait Implementations

##### `impl PartialEq for Lookbehind`

- <span id="lookbehind-partialeq-eq"></span>`fn eq(&self, other: &Lookbehind) -> bool` — [`Lookbehind`](#lookbehind)

##### `impl StructuralPartialEq for Lookbehind`

## Functions

### `expand`

```rust
fn expand(input: proc_macro::TokenStream, contains_paste: &mut bool, flatten_single_interpolation: bool) -> std::result::Result<proc_macro::TokenStream, Error>
```

### `is_single_interpolation_group`

```rust
fn is_single_interpolation_group(input: &proc_macro::TokenStream) -> bool
```

### `is_paste_operation`

```rust
fn is_paste_operation(input: &proc_macro::TokenStream) -> bool
```

### `parse_bracket_as_segments`

```rust
fn parse_bracket_as_segments(input: proc_macro::TokenStream, scope: proc_macro::Span) -> std::result::Result<Vec<crate::segment::Segment>, Error>
```

### `pasted_to_tokens`

```rust
fn pasted_to_tokens(pasted: String, span: proc_macro::Span) -> std::result::Result<proc_macro::TokenStream, Error>
```

