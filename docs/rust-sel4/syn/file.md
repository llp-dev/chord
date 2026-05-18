**syn > file**

# Module: file

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`File`](#file) - A complete file of Rust source code.

---

## syn::file::File

*Struct*

A complete file of Rust source code.

Typically `File` objects are created with [`parse_file`].

[`parse_file`]: crate::parse_file

# Example

Parse a Rust source file into a `syn::File` and print out a debug
representation of the syntax tree.

```
use std::env;
use std::fs;
use std::process;

fn main() {
# }
#
# fn fake_main() {
    let mut args = env::args();
    let _ = args.next(); // executable name

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: dump-syntax path/to/filename.rs");
            process::exit(1);
        }
    };

    let src = fs::read_to_string(&filename).expect("unable to read file");
    let syntax = syn::parse_file(&src).expect("unable to parse file");

    // Debug impl is available if Syn is built with "extra-traits" feature.
    println!("{:#?}", syntax);
}
```

Running with its own source code as input, this program prints output
that begins with:

```text
File {
    shebang: None,
    attrs: [],
    items: [
        Use(
            ItemUse {
                attrs: [],
                vis: Inherited,
                use_token: Use,
                leading_colon: None,
                tree: Path(
                    UsePath {
                        ident: Ident(
                            std,
                        ),
                        colon2_token: Colon2,
                        tree: Name(
                            UseName {
                                ident: Ident(
                                    env,
                                ),
                            },
                        ),
                    },
                ),
                semi_token: Semi,
            },
        ),
...
```

**Fields:**
- `shebang: Option<alloc::string::String>`
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `items: alloc::vec::Vec<crate::item::Item>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## Module: parsing



## Module: printing



