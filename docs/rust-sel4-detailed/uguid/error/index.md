*[uguid](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GuidFromStrError`](#guidfromstrerror) | enum | Error type for [`Guid::try_parse`] and [`Guid::from_str`]. |

## Enums

### `GuidFromStrError`

```rust
enum GuidFromStrError {
    Length,
    Separator(u8),
    Hex(u8),
}
```

Error type for `Guid::try_parse` and `Guid::from_str`.



#### Variants

- **`Length`**

  Input has the wrong length, expected 36 bytes.

- **`Separator`**

  Input is missing a separator (`-`) at this byte index.

- **`Hex`**

  Input contains invalid ASCII hex at this byte index.

#### Trait Implementations

##### `impl Clone for GuidFromStrError`

- <span id="guidfromstrerror-clone"></span>`fn clone(&self) -> GuidFromStrError` — [`GuidFromStrError`](#guidfromstrerror)

##### `impl Copy for GuidFromStrError`

##### `impl Debug for GuidFromStrError`

- <span id="guidfromstrerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GuidFromStrError`

- <span id="guidfromstrerror-default"></span>`fn default() -> Self`

##### `impl Display for GuidFromStrError`

- <span id="guidfromstrerror-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for GuidFromStrError`

##### `impl Error for GuidFromStrError`

##### `impl Hash for GuidFromStrError`

- <span id="guidfromstrerror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for GuidFromStrError`

- <span id="guidfromstrerror-ord-cmp"></span>`fn cmp(&self, other: &GuidFromStrError) -> cmp::Ordering` — [`GuidFromStrError`](#guidfromstrerror)

##### `impl PartialEq for GuidFromStrError`

- <span id="guidfromstrerror-partialeq-eq"></span>`fn eq(&self, other: &GuidFromStrError) -> bool` — [`GuidFromStrError`](#guidfromstrerror)

##### `impl PartialOrd for GuidFromStrError`

- <span id="guidfromstrerror-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &GuidFromStrError) -> option::Option<cmp::Ordering>` — [`GuidFromStrError`](#guidfromstrerror)

##### `impl StructuralPartialEq for GuidFromStrError`

