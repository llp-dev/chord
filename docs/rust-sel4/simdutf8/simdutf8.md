**simdutf8**

# Module: simdutf8

## Contents

**Modules**

- [`basic`](#basic) - The `basic` API flavor provides barebones UTF-8 checking at the highest speed.
- [`compat`](#compat) - The `compat` API flavor provides full compatibility with [`std::str::from_utf8()`] and detailed validation errors.

---

## Module: basic

The `basic` API flavor provides barebones UTF-8 checking at the highest speed.

It is fastest on valid UTF-8, but only checks for errors after processing the whole byte sequence
and does not provide detailed information if the data is not valid UTF-8. [`Utf8Error`] is a zero-sized error struct.

If you need detailed error information use the functions from the [`crate::compat`] module instead.



## Module: compat

The `compat` API flavor provides full compatibility with [`std::str::from_utf8()`] and detailed validation errors.

In particular, [`from_utf8()`]
returns an [`Utf8Error`], which has the [`valid_up_to()`](Utf8Error#method.valid_up_to) and
[`error_len()`](Utf8Error#method.error_len) methods. The first is useful for verification of streamed data. The
second is useful e.g. for replacing invalid byte sequences with a replacement character.

The functions in this module also fail early: errors are checked on-the-fly as the string is processed and once
an invalid UTF-8 sequence is encountered, it returns without processing the rest of the data.
This comes at a slight performance penalty compared to the [`crate::basic`] module if the input is valid UTF-8.



