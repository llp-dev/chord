*[ruzstd](../../index.md) / [decoding](../index.md) / [sequence_execution](index.md)*

---

# Module `sequence_execution`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`execute_sequences`](#execute-sequences) | fn | Take the provided decoder and execute the sequences stored within |
| [`do_offset_history`](#do-offset-history) | fn | Update the most recently used offsets to reflect the provided offset value, and return the "actual" offset needed because offsets are not stored in a raw way, some transformations are needed before you get a functional number. |

## Functions

### `execute_sequences`

```rust
fn execute_sequences(scratch: &mut super::scratch::DecoderScratch) -> Result<(), crate::decoding::errors::ExecuteSequencesError>
```

Take the provided decoder and execute the sequences stored within

### `do_offset_history`

```rust
fn do_offset_history(offset_value: u32, lit_len: u32, scratch: &mut [u32; 3]) -> u32
```

Update the most recently used offsets to reflect the provided offset value, and return the
"actual" offset needed because offsets are not stored in a raw way, some transformations are needed
before you get a functional number.

