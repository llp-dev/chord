*[simdutf8](../../index.md) / [implementation](../index.md) / [algorithm](index.md)*

---

# Module `algorithm`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`algorithm_simd!`](#algorithm-simd) | macro | Macros requires newtypes in scope: `SimdU8Value` - implementation of SIMD primitives `SimdInput` - which  holds 64 bytes of SIMD input `TempSimdChunk` - correctly aligned `TempSimdChunk`, either `TempSimdChunkA16` or `TempSimdChunkA32` |
| [`simd_input_128_bit!`](#simd-input-128-bit) | macro |  |
| [`simd_input_256_bit!`](#simd-input-256-bit) | macro |  |

## Macros

### `algorithm_simd!`

Macros requires newtypes in scope:
`SimdU8Value` - implementation of SIMD primitives
`SimdInput` - which  holds 64 bytes of SIMD input
`TempSimdChunk` - correctly aligned `TempSimdChunk`, either `TempSimdChunkA16` or `TempSimdChunkA32`

### `simd_input_128_bit!`

### `simd_input_256_bit!`

