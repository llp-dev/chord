*[gimli](../index.md) / [case_fold](index.md)*

---

# Module `case_fold`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`case_fold_data`](#case-fold-data) | fn |  |
| [`case_fold`](#case-fold) | fn | Perform case folding for the DWARF name index hashing. |
| [`case_folding_djb_hash`](#case-folding-djb-hash) | fn | Calculate a case folding DJB hash for the DWARF name index. |
| [`djb_hash_byte`](#djb-hash-byte) | fn |  |

## Functions

### `case_fold_data`

```rust
fn case_fold_data(c: char) -> char
```

### `case_fold`

```rust
fn case_fold(c: char) -> char
```

Perform case folding for the DWARF name index hashing.

This implements the case folding specified in DWARF 5 Section 6.1.1.4.5.

"The simple case folding algorithm is further described in the CaseFolding.txt file
distributed with the Unicode Character Database. That file defines four classes of
mappings: Common (C), Simple (S), Full (F), and Turkish (T). The hash
computation specified here uses the C + S mappings only, which do not affect the
total length of the string, with the addition that Turkish upper case dotted ’İ’ and
lower case dotless ’ı’ are folded to the Latin lower case ’i’."

### `case_folding_djb_hash`

```rust
fn case_folding_djb_hash(s: &str) -> u32
```

Calculate a case folding DJB hash for the DWARF name index.

This uses the case folding specified in DWARF 5 Section 6.1.1.4.5
with the DJB hash specified in DWARF 5 Section 7.33.

### `djb_hash_byte`

```rust
fn djb_hash_byte(hash: u32, byte: u8) -> u32
```

