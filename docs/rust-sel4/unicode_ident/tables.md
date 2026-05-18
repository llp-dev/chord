**unicode_ident > tables**

# Module: tables

## Contents

**Structs**

- [`Align64`](#align64)
- [`Align8`](#align8)

**Statics**

- [`LEAF`](#leaf)
- [`TRIE_CONTINUE`](#trie_continue)
- [`TRIE_START`](#trie_start)

**Constants**

- [`ASCII_CONTINUE`](#ascii_continue)
- [`ASCII_START`](#ascii_start)
- [`CHUNK`](#chunk)
- [`UNICODE_VERSION`](#unicode_version)

---

## unicode_ident::tables::ASCII_CONTINUE

*Constant*: `u128`



## unicode_ident::tables::ASCII_START

*Constant*: `u128`



## unicode_ident::tables::Align64

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`



## unicode_ident::tables::Align8

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`



## unicode_ident::tables::CHUNK

*Constant*: `usize`



## unicode_ident::tables::LEAF

*Static*

```rust
static LEAF: Align64<[u8; 7808]>
```



## unicode_ident::tables::TRIE_CONTINUE

*Static*

```rust
static TRIE_CONTINUE: Align8<[u8; 1793]>
```



## unicode_ident::tables::TRIE_START

*Static*

```rust
static TRIE_START: Align8<[u8; 411]>
```



## unicode_ident::tables::UNICODE_VERSION

*Constant*: `(u8, u8, u8)`



