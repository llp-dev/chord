# ttf_parser

A high-level, safe, zero-allocation font parser for:
* [TrueType](https://docs.microsoft.com/en-us/typography/truetype/),
* [OpenType](https://docs.microsoft.com/en-us/typography/opentype/spec/), and
* [AAT](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6AATIntro.html)
  fonts.

Font parsing starts with a [`Face`].

## Features

- A high-level API for most common properties, hiding all parsing and data resolving logic.
- A low-level, but safe API to access TrueType tables data.
- Highly configurable. You can disable most of the features, reducing binary size.
  You can also parse TrueType tables separately, without loading the whole font/face.
- Zero heap allocations.
- Zero unsafe.
- Zero dependencies.
- `no_std`/WASM compatible.
- Fast.
- Stateless. All parsing methods are immutable.
- Simple and maintainable code (no magic numbers).

## Safety

- The library must not panic. Any panic considered as a critical bug and should be reported.
- The library forbids unsafe code.
- No heap allocations, so crash due to OOM is not possible.
- All recursive methods have a depth limit.
- Technically, should use less than 64KiB of stack in worst case scenario.
- Most of arithmetic operations are checked.
- Most of numeric casts are checked.

## Modules

### [`ttf_parser`](ttf_parser.md)

*1 function, 1 trait, 18 structs, 3 enums, 3 modules*

### [`language`](language.md)

*1 enum, 1 static*

### [`parser`](parser.md)

*13 structs, 2 functions, 5 traits*

### [`tables`](tables.md)

*21 modules*

### [`tables::cbdt`](tables/cbdt.md)

*1 struct*

### [`tables::cblc`](tables/cblc.md)

*2 enums, 2 functions, 7 structs*

### [`tables::cff`](tables/cff.md)

*1 enum, 1 trait, 2 structs, 4 functions, 7 modules*

### [`tables::cff::argstack`](tables/cff/argstack.md)

*1 struct*

### [`tables::cff::cff1`](tables/cff/cff1.md)

*10 functions, 2 enums, 4 constants, 5 modules, 7 structs*

### [`tables::cff::cff1::charset_id`](tables/cff/cff1/charset_id.md)

*3 constants*

### [`tables::cff::cff1::encoding_id`](tables/cff/cff1/encoding_id.md)

*2 constants*

### [`tables::cff::cff1::operator`](tables/cff/cff1/operator.md)

*29 constants*

### [`tables::cff::cff1::private_dict_operator`](tables/cff/cff1/private_dict_operator.md)

*3 constants*

### [`tables::cff::cff1::top_dict_operator`](tables/cff/cff1/top_dict_operator.md)

*8 constants*

### [`tables::cff::charset`](tables/cff/charset.md)

*1 enum, 1 function, 2 structs*

### [`tables::cff::charstring`](tables/cff/charstring.md)

*1 struct*

### [`tables::cff::dict`](tables/cff/dict.md)

*2 structs, 3 constants, 5 functions*

### [`tables::cff::encoding`](tables/cff/encoding.md)

*1 constant, 1 enum, 1 function, 3 structs*

### [`tables::cff::index`](tables/cff/index.md)

*1 enum, 1 trait, 3 structs, 4 functions*

### [`tables::cmap`](tables/cmap.md)

*1 enum, 5 structs, 8 modules*

### [`tables::cmap::format0`](tables/cmap/format0.md)

*1 struct*

### [`tables::cmap::format10`](tables/cmap/format10.md)

*1 struct*

### [`tables::cmap::format12`](tables/cmap/format12.md)

*2 structs*

### [`tables::cmap::format13`](tables/cmap/format13.md)

*1 struct*

### [`tables::cmap::format14`](tables/cmap/format14.md)

*1 enum, 4 structs*

### [`tables::cmap::format2`](tables/cmap/format2.md)

*2 structs*

### [`tables::cmap::format4`](tables/cmap/format4.md)

*1 struct*

### [`tables::cmap::format6`](tables/cmap/format6.md)

*1 struct*

### [`tables::colr`](tables/colr.md)

*1 trait, 1 type alias, 14 structs, 4 enums*

### [`tables::cpal`](tables/cpal.md)

*2 structs*

### [`tables::glyf`](tables/glyf.md)

*1 constant, 12 structs, 3 functions*

### [`tables::head`](tables/head.md)

*1 enum, 1 struct*

### [`tables::hhea`](tables/hhea.md)

*1 struct*

### [`tables::hmtx`](tables/hmtx.md)

*2 structs*

### [`tables::kern`](tables/kern.md)

*1 enum, 1 function, 10 structs*

### [`tables::loca`](tables/loca.md)

*1 enum*

### [`tables::maxp`](tables/maxp.md)

*1 struct*

### [`tables::name`](tables/name.md)

*1 enum, 1 function, 1 module, 5 structs*

### [`tables::name::name_id`](tables/name/name_id.md)

*25 constants*

### [`tables::os2`](tables/os2.md)

*1 function, 16 constants, 4 enums, 4 structs*

### [`tables::post`](tables/post.md)

*2 structs, 4 constants*

### [`tables::sbix`](tables/sbix.md)

*1 function, 4 structs*

### [`tables::stat`](tables/stat.md)

*1 enum, 9 structs*

### [`tables::svg`](tables/svg.md)

*5 structs*

### [`tables::vhea`](tables/vhea.md)

*1 struct*

### [`tables::vorg`](tables/vorg.md)

*2 structs*

