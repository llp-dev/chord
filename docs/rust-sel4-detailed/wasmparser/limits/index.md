*[wasmparser](../index.md) / [limits](index.md)*

---

# Module `limits`

## Contents

- [Functions](#functions)
  - [`max_wasm_memory32_pages`](#max-wasm-memory32-pages)
  - [`max_wasm_memory64_pages`](#max-wasm-memory64-pages)
- [Constants](#constants)
  - [`MAX_WASM_TYPES`](#max-wasm-types)
  - [`MAX_WASM_SUPERTYPES`](#max-wasm-supertypes)
  - [`MAX_WASM_FUNCTIONS`](#max-wasm-functions)
  - [`MAX_WASM_IMPORTS`](#max-wasm-imports)
  - [`MAX_WASM_EXPORTS`](#max-wasm-exports)
  - [`MAX_WASM_GLOBALS`](#max-wasm-globals)
  - [`MAX_WASM_ELEMENT_SEGMENTS`](#max-wasm-element-segments)
  - [`MAX_WASM_DATA_SEGMENTS`](#max-wasm-data-segments)
  - [`MAX_WASM_STRING_SIZE`](#max-wasm-string-size)
  - [`MAX_WASM_FUNCTION_SIZE`](#max-wasm-function-size)
  - [`MAX_WASM_FUNCTION_LOCALS`](#max-wasm-function-locals)
  - [`MAX_WASM_FUNCTION_PARAMS`](#max-wasm-function-params)
  - [`MAX_WASM_FUNCTION_RETURNS`](#max-wasm-function-returns)
  - [`_MAX_WASM_TABLE_SIZE`](#max-wasm-table-size)
  - [`MAX_WASM_TABLE_ENTRIES`](#max-wasm-table-entries)
  - [`MAX_WASM_TABLES`](#max-wasm-tables)
  - [`MAX_WASM_MEMORIES`](#max-wasm-memories)
  - [`MAX_WASM_TAGS`](#max-wasm-tags)
  - [`MAX_WASM_BR_TABLE_SIZE`](#max-wasm-br-table-size)
  - [`MAX_WASM_STRUCT_FIELDS`](#max-wasm-struct-fields)
  - [`MAX_WASM_CATCHES`](#max-wasm-catches)
  - [`MAX_WASM_SUBTYPING_DEPTH`](#max-wasm-subtyping-depth)
  - [`MAX_WASM_HANDLERS`](#max-wasm-handlers)
  - [`MAX_WASM_TYPE_SIZE`](#max-wasm-type-size)
  - [`MAX_WASM_SELECT_RESULT_SIZE`](#max-wasm-select-result-size)
  - [`DEFAULT_WASM_PAGE_SIZE`](#default-wasm-page-size)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`max_wasm_memory32_pages`](#max-wasm-memory32-pages) | fn |  |
| [`max_wasm_memory64_pages`](#max-wasm-memory64-pages) | fn |  |
| [`MAX_WASM_TYPES`](#max-wasm-types) | const |  |
| [`MAX_WASM_SUPERTYPES`](#max-wasm-supertypes) | const |  |
| [`MAX_WASM_FUNCTIONS`](#max-wasm-functions) | const |  |
| [`MAX_WASM_IMPORTS`](#max-wasm-imports) | const |  |
| [`MAX_WASM_EXPORTS`](#max-wasm-exports) | const |  |
| [`MAX_WASM_GLOBALS`](#max-wasm-globals) | const |  |
| [`MAX_WASM_ELEMENT_SEGMENTS`](#max-wasm-element-segments) | const |  |
| [`MAX_WASM_DATA_SEGMENTS`](#max-wasm-data-segments) | const |  |
| [`MAX_WASM_STRING_SIZE`](#max-wasm-string-size) | const |  |
| [`MAX_WASM_FUNCTION_SIZE`](#max-wasm-function-size) | const |  |
| [`MAX_WASM_FUNCTION_LOCALS`](#max-wasm-function-locals) | const |  |
| [`MAX_WASM_FUNCTION_PARAMS`](#max-wasm-function-params) | const |  |
| [`MAX_WASM_FUNCTION_RETURNS`](#max-wasm-function-returns) | const |  |
| [`_MAX_WASM_TABLE_SIZE`](#max-wasm-table-size) | const |  |
| [`MAX_WASM_TABLE_ENTRIES`](#max-wasm-table-entries) | const |  |
| [`MAX_WASM_TABLES`](#max-wasm-tables) | const |  |
| [`MAX_WASM_MEMORIES`](#max-wasm-memories) | const |  |
| [`MAX_WASM_TAGS`](#max-wasm-tags) | const |  |
| [`MAX_WASM_BR_TABLE_SIZE`](#max-wasm-br-table-size) | const |  |
| [`MAX_WASM_STRUCT_FIELDS`](#max-wasm-struct-fields) | const |  |
| [`MAX_WASM_CATCHES`](#max-wasm-catches) | const |  |
| [`MAX_WASM_SUBTYPING_DEPTH`](#max-wasm-subtyping-depth) | const |  |
| [`MAX_WASM_HANDLERS`](#max-wasm-handlers) | const |  |
| [`MAX_WASM_TYPE_SIZE`](#max-wasm-type-size) | const |  |
| [`MAX_WASM_SELECT_RESULT_SIZE`](#max-wasm-select-result-size) | const |  |
| [`DEFAULT_WASM_PAGE_SIZE`](#default-wasm-page-size) | const |  |

## Functions

### `max_wasm_memory32_pages`

```rust
fn max_wasm_memory32_pages(page_size: u64) -> u64
```

### `max_wasm_memory64_pages`

```rust
fn max_wasm_memory64_pages(page_size: u64) -> u64
```

## Constants

### `MAX_WASM_TYPES`
```rust
const MAX_WASM_TYPES: usize = 1_000_000usize;
```

### `MAX_WASM_SUPERTYPES`
```rust
const MAX_WASM_SUPERTYPES: usize = 1usize;
```

### `MAX_WASM_FUNCTIONS`
```rust
const MAX_WASM_FUNCTIONS: usize = 1_000_000usize;
```

### `MAX_WASM_IMPORTS`
```rust
const MAX_WASM_IMPORTS: usize = 1_000_000usize;
```

### `MAX_WASM_EXPORTS`
```rust
const MAX_WASM_EXPORTS: usize = 1_000_000usize;
```

### `MAX_WASM_GLOBALS`
```rust
const MAX_WASM_GLOBALS: usize = 1_000_000usize;
```

### `MAX_WASM_ELEMENT_SEGMENTS`
```rust
const MAX_WASM_ELEMENT_SEGMENTS: usize = 100_000usize;
```

### `MAX_WASM_DATA_SEGMENTS`
```rust
const MAX_WASM_DATA_SEGMENTS: usize = 100_000usize;
```

### `MAX_WASM_STRING_SIZE`
```rust
const MAX_WASM_STRING_SIZE: usize = 100_000usize;
```

### `MAX_WASM_FUNCTION_SIZE`
```rust
const MAX_WASM_FUNCTION_SIZE: usize = 7_654_321usize;
```

### `MAX_WASM_FUNCTION_LOCALS`
```rust
const MAX_WASM_FUNCTION_LOCALS: u32 = 50_000u32;
```

### `MAX_WASM_FUNCTION_PARAMS`
```rust
const MAX_WASM_FUNCTION_PARAMS: usize = 1_000usize;
```

### `MAX_WASM_FUNCTION_RETURNS`
```rust
const MAX_WASM_FUNCTION_RETURNS: usize = 1_000usize;
```

### `_MAX_WASM_TABLE_SIZE`
```rust
const _MAX_WASM_TABLE_SIZE: usize = 10_000_000usize;
```

### `MAX_WASM_TABLE_ENTRIES`
```rust
const MAX_WASM_TABLE_ENTRIES: usize = 10_000_000usize;
```

### `MAX_WASM_TABLES`
```rust
const MAX_WASM_TABLES: usize = 100usize;
```

### `MAX_WASM_MEMORIES`
```rust
const MAX_WASM_MEMORIES: usize = 100usize;
```

### `MAX_WASM_TAGS`
```rust
const MAX_WASM_TAGS: usize = 1_000_000usize;
```

### `MAX_WASM_BR_TABLE_SIZE`
```rust
const MAX_WASM_BR_TABLE_SIZE: usize = 7_654_321usize;
```

### `MAX_WASM_STRUCT_FIELDS`
```rust
const MAX_WASM_STRUCT_FIELDS: usize = 10_000usize;
```

### `MAX_WASM_CATCHES`
```rust
const MAX_WASM_CATCHES: usize = 10_000usize;
```

### `MAX_WASM_SUBTYPING_DEPTH`
```rust
const MAX_WASM_SUBTYPING_DEPTH: usize = 63usize;
```

### `MAX_WASM_HANDLERS`
```rust
const MAX_WASM_HANDLERS: usize = 10_000usize;
```

### `MAX_WASM_TYPE_SIZE`
```rust
const MAX_WASM_TYPE_SIZE: u32 = 1_000_000u32;
```

### `MAX_WASM_SELECT_RESULT_SIZE`
```rust
const MAX_WASM_SELECT_RESULT_SIZE: usize = 10usize;
```

### `DEFAULT_WASM_PAGE_SIZE`
```rust
const DEFAULT_WASM_PAGE_SIZE: u64 = 65_536u64;
```

