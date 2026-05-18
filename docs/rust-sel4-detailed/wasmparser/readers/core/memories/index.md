*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [memories](index.md)*

---

# Module `memories`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MemorySectionReader`](#memorysectionreader) | type | A reader for the memory section of a WebAssembly module. |

## Type Aliases

### `MemorySectionReader<'a>`

```rust
type MemorySectionReader<'a> = crate::SectionLimited<'a, crate::MemoryType>;
```

A reader for the memory section of a WebAssembly module.

