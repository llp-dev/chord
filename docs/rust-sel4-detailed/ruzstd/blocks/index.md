*[ruzstd](../index.md) / [blocks](index.md)*

---

# Module `blocks`

In a Zstandard frame, there's a frame header, followed by one or more *blocks*.

A block contains data, and a header describing how that data is encoded, as well
as other misc metadata.

<https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#blocks>

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`block`](#block) | mod | Block header definitions. |
| [`literals_section`](#literals-section) | mod | Utilities and representations for the first half of a block, the literals section. |
| [`sequence_section`](#sequence-section) | mod | Utilities and representations for the second half of a block, the sequence section. |

## Modules

- [`block`](block/index.md) — Block header definitions.
- [`literals_section`](literals_section/index.md) — Utilities and representations for the first half of a block, the literals section.
- [`sequence_section`](sequence_section/index.md) — Utilities and representations for the second half of a block, the sequence section.

