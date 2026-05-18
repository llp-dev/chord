# ptr_meta

A radioactive stabilization of the [`ptr_meta` RFC][rfc].

This crate provides the [`Pointee`] trait, [`from_raw_parts`] and
[`to_raw_parts`] functions, and proc macros for deriving `Pointee` for
structs and implementing `Pointee` for trait objects.

[rfc]: https://rust-lang.github.io/rfcs/2580-ptr-meta.html

# Usage

Raw pointers can be decomposed into the data address and metadata components
with [`to_raw_parts`] or [`to_raw_parts_mut`].

Alternatively, metadata alone can be extracted with the [`metadata`]
function. Although [`metadata`] accepts pointers, references can be passed
and will be implicitly coerced.

A pointer can be created from its address and metadata with
[`from_raw_parts`] or [`from_raw_parts_mut`].

## Provided impls

`ptr_meta` provides inherent implementations for many builtin types:

- All [`Sized`] types implement [`Pointee`] via a blanket implementation.
- `slice`, `str`, and `CStr`
- `OsStr` (requires `std`)
- `dyn Any`, optionally with `+ Send` and/or `+ Sync`
- `dyn Error`, optionally with `+ Send` and/or `+ Sync`

## Structs with trailing DSTs

You can derive [`Pointee`] for structs with trailing DSTs:

```
use ptr_meta::Pointee;

#[derive(Pointee)]
struct Block<H, T> {
    header: H,
    elements: [T],
}
```

Note that the last field is required to be a DST. Structs with a generic
type as the last field may have conflicting blanket implementations, as the
generic type may be `Sized`. A collection of specific implementations may be
required in these cases, with the generic parameter set (for example) a
slice, `str`, or specific trait object.

## Trait objects

You can generate [`Pointee`] implementations for trait objects:

```
use ptr_meta::pointee;

// Generates Pointee for dyn Stringy
#[ptr_meta::pointee]
trait Stringy {
    fn as_string(&self) -> String;
}
```

Note that this will not produce implementations for `Trait + Send + Sync`.

## Features

- `derive`: Re-exports the macros from `ptr_meta_derive`. Enabled by
  default.
- `std`: Enables additional impls for `std` types. Enabled by default.

## Example
```rust
// Get the associated metadata for pointers
let str = "hello world";
assert_eq!(ptr_meta::metadata(str), str.len());

let slice = &[1, 2, 3, 4, 5] as &[i32];
assert_eq!(ptr_meta::metadata(slice), slice.len());

// Make your own wide pointers from data pointers and metadata
let bytes = [b'h', b'e', b'l', b'l', b'o'];
let ptr = ptr_meta::from_raw_parts::<str>(bytes.as_ptr().cast(), 5);
println!("{} world!", unsafe { &*ptr }); // prints "hello world!"

// Derive Pointee on your own types
#[derive(ptr_meta::Pointee)]
#[repr(transparent)]
struct CoolStr {
    inner: str,
}

impl CoolStr {
    fn print_cool(&self) {
        println!("😎 {} 😎", &self.inner);
    }
}

let ptr = ptr_meta::from_raw_parts::<CoolStr>(bytes.as_ptr().cast(), 5);
let cool = unsafe { &*ptr };
cool.print_cool(); // prints "😎 hello 😎"

// Implement Pointee for trait objects
#[ptr_meta::pointee]
trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("i32: {self}");
    }
}

let i32_vtable = ptr_meta::metadata(&0i32 as &dyn Printable);
let one_hundred = 100i32;
let printable = ptr_meta::from_raw_parts::<dyn Printable>(
    (&one_hundred as *const i32).cast(),
    i32_vtable,
);
unsafe {
    (*printable).print(); // prints "i32: 100"
}
```

## Modules

### [`ptr_meta`](ptr_meta.md)

*1 struct, 1 trait, 5 functions*

