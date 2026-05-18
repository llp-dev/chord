**sel4_synthetic_elf**

# Module: sel4_synthetic_elf

## Contents

**Structs**

- [`Builder`](#builder)

---

## sel4_synthetic_elf::Builder

*Struct*

**Generic Parameters:**
- 'a
- 'data
- T
- R

**Methods:**

- `fn empty(base_elf_file: &'a object::read::elf::ElfFile<'data, T, R>) -> Self`
- `fn new(base_elf_file: &'a object::read::elf::ElfFile<'data, T, R>) -> Result<Self, Box<dyn Error>>`
- `fn discard_p_align(self: & mut Self, doit: bool)`
- `fn add_segment(self: & mut Self, segment: Segment<'a>)`
- `fn footprint(self: &Self) -> Option<Range<u64>>`
- `fn next_vaddr(self: &Self) -> u64`
- `fn patch_bytes(self: & mut Self, name: &str, value: Vec<u8>) -> Result<u64, Box<dyn Error>>`
- `fn patch<impl PatchValue>(self: & mut Self, name: &str, value: impl Trait) -> Result<u64, Box<dyn Error>>`
- `fn patch_word(self: & mut Self, name: &str, value: <T as >::Word) -> Result<u64, Box<dyn Error>>`
- `fn patch_word_with_cast<impl ToPrimitive + fmt::Debug + Copy>(self: & mut Self, name: &str, value: impl Trait) -> Result<u64, Box<dyn Error>>`
- `fn build(self: &Self) -> Result<Vec<u8>, Box<dyn Error>>`



