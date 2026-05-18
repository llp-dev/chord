*[ring](../../index.md) / [io](../index.md) / [der_writer](index.md)*

---

# Module `der_writer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`write_positive_integer`](#write-positive-integer) | fn |  |
| [`write_all`](#write-all) | fn |  |
| [`write_tlv`](#write-tlv) | fn |  |

## Functions

### `write_positive_integer`

```rust
fn write_positive_integer(output: &mut dyn Accumulator, value: &Positive<'_>)
```

### `write_all`

```rust
fn write_all(tag: Tag, write_value: &dyn Fn(&mut dyn Accumulator)) -> alloc::boxed::Box<[u8]>
```

### `write_tlv`

```rust
fn write_tlv<F>(output: &mut dyn Accumulator, tag: Tag, write_value: F)
where
    F: Fn(&mut dyn Accumulator)
```

