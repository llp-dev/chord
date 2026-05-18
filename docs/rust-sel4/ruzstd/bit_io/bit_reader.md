**ruzstd > bit_io > bit_reader**

# Module: bit_io::bit_reader

## Contents

**Enums**

- [`GetBitsError`](#getbitserror)

---

## ruzstd::bit_io::bit_reader::GetBitsError

*Enum*

**Variants:**
- `TooManyBits{ num_requested_bits: usize, limit: u8 }`
- `NotEnoughRemainingBits{ requested: usize, remaining: usize }`



