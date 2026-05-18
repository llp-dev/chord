*[futures_util](../index.md) / [unfold_state](index.md)*

---

# Module `unfold_state`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`UnfoldState`](#unfoldstate) | enum | UnfoldState used for stream and sink unfolds |

## Enums

### `UnfoldState<T, R>`

```rust
enum UnfoldState<T, R> {
    Value {
        value: T,
    },
    Future {
        future: R,
    },
    Empty,
}
```

UnfoldState used for stream and sink unfolds

#### Implementations

- <span id="unfoldstate-project-future"></span>`fn project_future(self: Pin<&mut Self>) -> Option<Pin<&mut R>>`

- <span id="unfoldstate-take-value"></span>`fn take_value(self: Pin<&mut Self>) -> Option<T>`

#### Trait Implementations

##### `impl<T: fmt::Debug, R: fmt::Debug> Debug for UnfoldState<T, R>`

- <span id="unfoldstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, R> Unpin for UnfoldState<T, R>`

