*[unwinding](../index.md) / [personality](index.md)*

---

# Module `personality`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EHAction`](#ehaction) | enum |  |
| [`parse_pointer_encoding`](#parse-pointer-encoding) | fn |  |
| [`parse_encoded_pointer`](#parse-encoded-pointer) | fn |  |
| [`find_eh_action`](#find-eh-action) | fn |  |
| [`rust_eh_personality`](#rust-eh-personality) | fn |  |

## Enums

### `EHAction`

```rust
enum EHAction {
    None,
    Cleanup(usize),
    Catch(usize),
    Filter(usize),
    Terminate,
}
```

#### Trait Implementations

##### `impl Debug for EHAction`

- <span id="ehaction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `parse_pointer_encoding`

```rust
fn parse_pointer_encoding(input: &mut gimli::EndianSlice<'static, gimli::NativeEndian>) -> gimli::Result<constants::DwEhPe>
```

### `parse_encoded_pointer`

```rust
fn parse_encoded_pointer(encoding: constants::DwEhPe, unwind_ctx: &UnwindContext<'_>, input: &mut gimli::EndianSlice<'static, gimli::NativeEndian>) -> gimli::Result<gimli::Pointer>
```

### `find_eh_action`

```rust
fn find_eh_action(reader: &mut gimli::EndianSlice<'static, gimli::NativeEndian>, unwind_ctx: &UnwindContext<'_>) -> gimli::Result<EHAction>
```

### `rust_eh_personality`

```rust
unsafe fn rust_eh_personality(version: i32, actions: UnwindAction, _exception_class: u64, exception: *mut UnwindException, unwind_ctx: &mut UnwindContext<'_>) -> UnwindReasonCode
```

