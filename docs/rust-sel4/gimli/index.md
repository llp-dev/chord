# gimli

`gimli` is a library for reading and writing the
[DWARF debugging format](https://dwarfstd.org/).

See the [read](./read/index.html) and [write](./write/index.html) modules
for examples and API documentation.

## Cargo Features

Cargo features that can be enabled with `gimli`:

* `std`: Enabled by default. Use the `std` library. Disabling this feature
  allows using `gimli` in embedded environments that do not have access to
  `std`. Note that even when `std` is disabled, `gimli` still requires an
  implementation of the `alloc` crate.

* `read`: Enabled by default. Enables the `read` module. Use of `std` is
  optional.

* `write`: Enabled by default. Enables the `write` module. Always uses
  the `std` library.

## Modules

### [`gimli`](gimli.md)

*3 modules*

### [`arch`](arch.md)

*8 structs*

### [`common`](common.md)

*28 structs, 5 enums*

### [`constants`](constants.md)

*32 structs, 882 constants*

### [`endianity`](endianity.md)

*1 enum, 1 trait, 1 type alias, 2 structs*

### [`leb128`](leb128.md)

*1 module*

### [`leb128::read`](leb128/read.md)

*4 functions*

### [`read`](read.md)

*1 enum, 1 trait, 2 structs, 2 type aliases*

### [`read::addr`](read/addr.md)

*4 structs*

### [`read::aranges`](read/aranges.md)

*5 structs*

### [`read::cfi`](read/cfi.md)

*19 structs, 3 traits, 5 enums*

### [`read::endian_slice`](read/endian_slice.md)

*1 struct*

### [`read::index`](read/index.md)

*1 enum, 5 structs*

### [`read::loclists`](read/loclists.md)

*1 enum, 6 structs*

### [`read::op`](read/op.md)

*1 trait, 4 enums, 4 structs*

### [`read::reader`](read/reader.md)

*1 struct, 2 traits*

### [`read::reader::seal_if_no_alloc`](read/reader/seal_if_no_alloc.md)

*1 struct*

### [`read::relocate`](read/relocate.md)

*1 struct, 1 trait*

### [`read::rnglists`](read/rnglists.md)

*1 enum, 6 structs*

### [`read::str`](read/str.md)

*3 structs*

### [`read::util`](read/util.md)

*1 trait*

### [`read::util::sealed`](read/util/sealed.md)

*1 struct, 1 trait*

### [`read::value`](read/value.md)

*2 enums*

