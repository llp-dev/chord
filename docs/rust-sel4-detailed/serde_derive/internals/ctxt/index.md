*[serde_derive](../../index.md) / [internals](../index.md) / [ctxt](index.md)*

---

# Module `ctxt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ctxt`](#ctxt) | struct | A type to collect errors together and format them. |

## Structs

### `Ctxt`

```rust
struct Ctxt {
    errors: std::cell::RefCell<Option<Vec<syn::Error>>>,
}
```

A type to collect errors together and format them.

Dropping this object will cause a panic. It must be consumed using `check`.

References can be shared since this type uses run-time exclusive mut checking.

#### Implementations

- <span id="ctxt-new"></span>`fn new() -> Self`

  Create a new context object.

  

  This object contains no errors, but will still trigger a panic if it is not `check`ed.

- <span id="ctxt-error-spanned-by"></span>`fn error_spanned_by<A: ToTokens, T: Display>(&self, obj: A, msg: T)`

  Add an error to the context object with a tokenenizable object.

  

  The object is used for spanning in error messages.

- <span id="ctxt-syn-error"></span>`fn syn_error(&self, err: syn::Error)`

  Add one of Syn's parse errors.

- <span id="ctxt-check"></span>`fn check(self) -> syn::Result<()>`

  Consume this object, producing a formatted error string if there are errors.

#### Trait Implementations

##### `impl Default for Ctxt`

- <span id="ctxt-default"></span>`fn default() -> Ctxt` — [`Ctxt`](#ctxt)

##### `impl Drop for Ctxt`

- <span id="ctxt-drop"></span>`fn drop(&mut self)`

