**untrusted**

# Module: untrusted

## Contents

**Modules**

- [`input`](#input)
- [`no_panic`](#no_panic)
- [`reader`](#reader)

**Functions**

- [`read_all_optional`](#read_all_optional) - Calls `read` with the given input as a `Reader`, ensuring that `read`

---

## Module: input



## Module: no_panic



## untrusted::read_all_optional

*Function*

Calls `read` with the given input as a `Reader`, ensuring that `read`
consumed the entire input. When `input` is `None`, `read` will be
called with `None`.

```rust
fn read_all_optional<'a, F, R, E>(input: Option<Input<'a>>, incomplete_read: E, read: F) -> Result<R, E>
```



## Module: reader



