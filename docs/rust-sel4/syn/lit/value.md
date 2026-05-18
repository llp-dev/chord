**syn > lit > value**

# Module: lit::value

## Contents

**Functions**

- [`backslash_u`](#backslash_u)
- [`backslash_x`](#backslash_x)
- [`byte`](#byte) - Get the byte at offset idx, or a default of `b'\0'` if we're looking
- [`next_chr`](#next_chr)
- [`parse_lit_byte`](#parse_lit_byte)
- [`parse_lit_byte_str`](#parse_lit_byte_str)
- [`parse_lit_byte_str_cooked`](#parse_lit_byte_str_cooked)
- [`parse_lit_byte_str_raw`](#parse_lit_byte_str_raw)
- [`parse_lit_c_str`](#parse_lit_c_str)
- [`parse_lit_c_str_cooked`](#parse_lit_c_str_cooked)
- [`parse_lit_c_str_raw`](#parse_lit_c_str_raw)
- [`parse_lit_char`](#parse_lit_char)
- [`parse_lit_float`](#parse_lit_float)
- [`parse_lit_int`](#parse_lit_int)
- [`parse_lit_str`](#parse_lit_str)
- [`parse_lit_str_cooked`](#parse_lit_str_cooked)
- [`parse_lit_str_raw`](#parse_lit_str_raw)

---

## syn::lit::value::backslash_u

*Function*

```rust
fn backslash_u<S>(s: &S) -> Option<(char, &S)>
```



## syn::lit::value::backslash_x

*Function*

```rust
fn backslash_x<S>(s: &S) -> Option<(u8, &S)>
```



## syn::lit::value::byte

*Function*

Get the byte at offset idx, or a default of `b'\0'` if we're looking
past the end of the input buffer.

```rust
fn byte<S>(s: &S, idx: usize) -> u8
```



## syn::lit::value::next_chr

*Function*

```rust
fn next_chr(s: &str) -> char
```



## syn::lit::value::parse_lit_byte

*Function*

```rust
fn parse_lit_byte(s: &str) -> Option<(u8, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_byte_str

*Function*

```rust
fn parse_lit_byte_str(s: &str) -> Option<(alloc::vec::Vec<u8>, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_byte_str_cooked

*Function*

```rust
fn parse_lit_byte_str_cooked(s: &str) -> Option<(alloc::vec::Vec<u8>, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_byte_str_raw

*Function*

```rust
fn parse_lit_byte_str_raw(s: &str) -> Option<(alloc::vec::Vec<u8>, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_c_str

*Function*

```rust
fn parse_lit_c_str(s: &str) -> Option<(alloc::ffi::CString, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_c_str_cooked

*Function*

```rust
fn parse_lit_c_str_cooked(s: &str) -> Option<(alloc::ffi::CString, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_c_str_raw

*Function*

```rust
fn parse_lit_c_str_raw(s: &str) -> Option<(alloc::ffi::CString, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_char

*Function*

```rust
fn parse_lit_char(s: &str) -> Option<(char, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_float

*Function*

```rust
fn parse_lit_float(input: &str) -> Option<(alloc::boxed::Box<str>, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_int

*Function*

```rust
fn parse_lit_int(s: &str) -> Option<(alloc::boxed::Box<str>, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_str

*Function*

```rust
fn parse_lit_str(s: &str) -> Option<(alloc::boxed::Box<str>, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_str_cooked

*Function*

```rust
fn parse_lit_str_cooked(s: &str) -> Option<(alloc::boxed::Box<str>, alloc::boxed::Box<str>)>
```



## syn::lit::value::parse_lit_str_raw

*Function*

```rust
fn parse_lit_str_raw(s: &str) -> Option<(alloc::boxed::Box<str>, alloc::boxed::Box<str>)>
```



