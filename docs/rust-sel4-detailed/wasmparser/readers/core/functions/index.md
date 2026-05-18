*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [functions](index.md)*

---

# Module `functions`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FunctionSectionReader`](#functionsectionreader) | type | A reader for the function section of a WebAssembly module. |

## Type Aliases

### `FunctionSectionReader<'a>`

```rust
type FunctionSectionReader<'a> = crate::SectionLimited<'a, u32>;
```

A reader for the function section of a WebAssembly module.

