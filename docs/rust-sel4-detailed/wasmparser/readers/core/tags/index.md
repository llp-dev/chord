*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [tags](index.md)*

---

# Module `tags`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TagSectionReader`](#tagsectionreader) | type | A reader for the tags section of a WebAssembly module. |

## Type Aliases

### `TagSectionReader<'a>`

```rust
type TagSectionReader<'a> = crate::SectionLimited<'a, crate::TagType>;
```

A reader for the tags section of a WebAssembly module.

