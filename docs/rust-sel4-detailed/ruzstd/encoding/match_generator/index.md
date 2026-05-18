*[ruzstd](../../index.md) / [encoding](../index.md) / [match_generator](index.md)*

---

# Module `match_generator`

Matching algorithm used find repeated parts in the original data

The Zstd format relies on finden repeated sequences of data and compressing these sequences as instructions to the decoder.
A sequence basically tells the decoder "Go back X bytes and copy Y bytes to the end of your decode buffer".

The task here is to efficiently find matches in the already encoded data for the current suffix of the not yet encoded data.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MatchGeneratorDriver`](#matchgeneratordriver) | struct | This is the default implementation of the `Matcher` trait. |
| [`SuffixStore`](#suffixstore) | struct | This stores the index of a suffix of a string by hashing the first few bytes of that suffix This means that collisions just overwrite and that you need to check validity after a get |
| [`WindowEntry`](#windowentry) | struct | We keep a window of a few of these entries All of these are valid targets for a match to be generated for |
| [`MatchGenerator`](#matchgenerator) | struct |  |
| [`MIN_MATCH_LEN`](#min-match-len) | const |  |

## Structs

### `MatchGeneratorDriver`

```rust
struct MatchGeneratorDriver {
    vec_pool: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    suffix_pool: alloc::vec::Vec<SuffixStore>,
    match_generator: MatchGenerator,
    slice_size: usize,
}
```

This is the default implementation of the `Matcher` trait. It allocates and reuses the buffers when possible.

#### Implementations

- <span id="matchgeneratordriver-new"></span>`fn new(slice_size: usize, max_slices_in_window: usize) -> Self`

  slice_size says how big the slices should be that are allocated to work with

  max_slices_in_window says how many slices should at most be used while looking for matches

#### Trait Implementations

##### `impl Matcher for MatchGeneratorDriver`

- <span id="matchgeneratordriver-matcher-reset"></span>`fn reset(&mut self, _level: CompressionLevel)` — [`CompressionLevel`](../index.md#compressionlevel)

- <span id="matchgeneratordriver-matcher-window-size"></span>`fn window_size(&self) -> u64`

- <span id="matchgeneratordriver-matcher-get-next-space"></span>`fn get_next_space(&mut self) -> Vec<u8>`

- <span id="matchgeneratordriver-matcher-get-last-space"></span>`fn get_last_space(&mut self) -> &[u8]`

- <span id="matchgeneratordriver-matcher-commit-space"></span>`fn commit_space(&mut self, space: Vec<u8>)`

- <span id="matchgeneratordriver-matcher-start-matching"></span>`fn start_matching(&mut self, handle_sequence: impl FnMut(Sequence<'a>))` — [`Sequence`](../index.md#sequence)

- <span id="matchgeneratordriver-matcher-skip-matching"></span>`fn skip_matching(&mut self)`

### `SuffixStore`

```rust
struct SuffixStore {
    slots: alloc::vec::Vec<Option<core::num::NonZeroUsize>>,
    len_log: u32,
}
```

This stores the index of a suffix of a string by hashing the first few bytes of that suffix
This means that collisions just overwrite and that you need to check validity after a get

#### Implementations

- <span id="suffixstore-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

- <span id="suffixstore-insert"></span>`fn insert(&mut self, suffix: &[u8], idx: usize)`

- <span id="suffixstore-contains-key"></span>`fn contains_key(&self, suffix: &[u8]) -> bool`

- <span id="suffixstore-get"></span>`fn get(&self, suffix: &[u8]) -> Option<usize>`

- <span id="suffixstore-key"></span>`fn key(&self, suffix: &[u8]) -> usize`

### `WindowEntry`

```rust
struct WindowEntry {
    data: alloc::vec::Vec<u8>,
    suffixes: SuffixStore,
    base_offset: usize,
}
```

We keep a window of a few of these entries
All of these are valid targets for a match to be generated for

#### Fields

- **`suffixes`**: `SuffixStore`

  Stores indexes into data

- **`base_offset`**: `usize`

  Makes offset calculations efficient

### `MatchGenerator`

```rust
struct MatchGenerator {
    max_window_size: usize,
    window: alloc::vec::Vec<WindowEntry>,
    window_size: usize,
    concat_window: alloc::vec::Vec<u8>,
    suffix_idx: usize,
    last_idx_in_sequence: usize,
}
```

#### Fields

- **`window`**: `alloc::vec::Vec<WindowEntry>`

  Data window we are operating on to find matches
  The data we want to find matches for is in the last slice

- **`suffix_idx`**: `usize`

  Index in the last slice that we already processed

- **`last_idx_in_sequence`**: `usize`

  Gets updated when a new sequence is returned to point right behind that sequence

#### Implementations

- <span id="matchgenerator-new"></span>`fn new(max_size: usize) -> Self`

  max_size defines how many bytes will be used at most in the window used for matching

- <span id="matchgenerator-reset"></span>`fn reset(&mut self, reuse_space: impl FnMut(Vec<u8>, SuffixStore))` — [`SuffixStore`](#suffixstore)

- <span id="matchgenerator-next-sequence"></span>`fn next_sequence(&mut self, handle_sequence: impl FnMut(Sequence<'a>)) -> bool` — [`Sequence`](../index.md#sequence)

  Processes bytes in the current window until either a match is found or no more matches can be found

  * If a match is found handle_sequence is called with the Triple variant

  * If no more matches can be found but there are bytes still left handle_sequence is called with the Literals variant

  * If no more matches can be found and no more bytes are left this returns false

- <span id="matchgenerator-common-prefix-len"></span>`fn common_prefix_len(a: &[u8], b: &[u8]) -> usize`

  Find the common prefix length between two byte slices

- <span id="matchgenerator-mismatch-chunks"></span>`fn mismatch_chunks<const N: usize>(xs: &[u8], ys: &[u8]) -> usize`

  Find the common prefix length between two byte slices with a configurable chunk length

  This enables vectorization optimizations

- <span id="matchgenerator-add-suffixes-till"></span>`fn add_suffixes_till(&mut self, idx: usize)`

  Process bytes and add the suffixes to the suffix store up to a specific index

- <span id="matchgenerator-skip-matching"></span>`fn skip_matching(&mut self)`

  Skip matching for the whole current window entry

- <span id="matchgenerator-add-data"></span>`fn add_data(&mut self, data: Vec<u8>, suffixes: SuffixStore, reuse_space: impl FnMut(Vec<u8>, SuffixStore))` — [`SuffixStore`](#suffixstore)

  Add a new window entry. Will panic if the last window entry hasn't been processed properly.

  If any resources are released by pushing the new entry they are returned via the callback

- <span id="matchgenerator-reserve"></span>`fn reserve(&mut self, amount: usize, reuse_space: impl FnMut(Vec<u8>, SuffixStore))` — [`SuffixStore`](#suffixstore)

  Reserve space for a new window entry

  If any resources are released by pushing the new entry they are returned via the callback

## Constants

### `MIN_MATCH_LEN`
```rust
const MIN_MATCH_LEN: usize = 5usize;
```

