*[heapless](../../index.md) / [string](../index.md) / [drain](index.md)*

---

# Module `drain`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Drain`](#drain) | struct | A draining iterator for `String`. |

## Structs

### `Drain<'a, LenT: LenType>`

```rust
struct Drain<'a, LenT: LenType> {
    string: *mut super::StringView<LenT>,
    start: LenT,
    end: LenT,
    iter: core::str::Chars<'a>,
}
```

A draining iterator for `String`.

This struct is created by the [`drain`](#drain) method on [`crate::String`](../index.md). See its
documentation for more.


#### Fields

- **`string`**: `*mut super::StringView<LenT>`

  Will be used as &'a mut String in the destructor

- **`start`**: `LenT`

  Stast of part to remove

- **`end`**: `LenT`

  End of part to remove

- **`iter`**: `core::str::Chars<'a>`

  Current remaining range to remove

#### Implementations

- <span id="drain-as-str"></span>`fn as_str(&self) -> &str`

  Returns the remaining (sub)string of this iterator as a slice.

  

  # Examples

  

  ```rust

  use heapless::String;

  

  let mut s = String::<8>::try_from("abc").unwrap();

  let mut drain = s.drain(..);

  assert_eq!(drain.as_str(), "abc");

  let _ = drain.next().unwrap();

  assert_eq!(drain.as_str(), "bc");

  ```

#### Trait Implementations

##### `impl<LenT: LenType> AsRef for Drain<'_, LenT>`

- <span id="drain-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<LenT: LenType> Debug for Drain<'_, LenT>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<LenT: LenType> DoubleEndedIterator for Drain<'_, LenT>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<char>`

##### `impl<LenT: LenType> Drop for Drain<'_, LenT>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<LenT: LenType> FusedIterator for Drain<'_, LenT>`

##### `impl IntoIterator for Drain<'a, LenT>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<LenT: LenType> Iterator for Drain<'_, LenT>`

- <span id="drain-iterator-type-item"></span>`type Item = char`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-iterator-last"></span>`fn last(self) -> Option<char>`

##### `impl<LenT: LenType> Send for Drain<'_, LenT>`

##### `impl<LenT: LenType> Sync for Drain<'_, LenT>`

