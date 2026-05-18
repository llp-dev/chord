**ruzstd > encoding > match_generator**

# Module: encoding::match_generator

## Contents

**Structs**

- [`MatchGeneratorDriver`](#matchgeneratordriver) - This is the default implementation of the `Matcher` trait. It allocates and reuses the buffers when possible.

---

## ruzstd::encoding::match_generator::MatchGeneratorDriver

*Struct*

This is the default implementation of the `Matcher` trait. It allocates and reuses the buffers when possible.

**Trait Implementations:**

- **Matcher**
  - `fn reset(self: & mut Self, _level: CompressionLevel)`
  - `fn window_size(self: &Self) -> u64`
  - `fn get_next_space(self: & mut Self) -> Vec<u8>`
  - `fn get_last_space(self: & mut Self) -> &[u8]`
  - `fn commit_space(self: & mut Self, space: Vec<u8>)`
  - `fn start_matching<impl for<'a> FnMut(Sequence<'a>)>(self: & mut Self, handle_sequence: impl Trait)`
  - `fn skip_matching(self: & mut Self)`



