*[serde_derive](../../index.md) / [internals](../index.md) / [case](index.md)*

---

# Module `case`

Code to convert the Rust-styled field/variant (e.g. `my_field`, `MyType`) to the
case of the source (e.g. `my-field`, `MY_FIELD`).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParseError`](#parseerror) | struct |  |
| [`RenameRule`](#renamerule) | enum | The different possible ways to change case of fields in a struct, or variants in an enum. |

## Structs

### `ParseError<'a>`

```rust
struct ParseError<'a> {
    unknown: &'a str,
}
```

#### Trait Implementations

##### `impl Display for ParseError<'a>`

- <span id="parseerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for ParseError<'a>`

- <span id="parseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `RenameRule`

```rust
enum RenameRule {
    None,
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}
```

The different possible ways to change case of fields in a struct, or variants in an enum.

#### Variants

- **`None`**

  Don't apply a default rename rule.

- **`LowerCase`**

  Rename direct children to "lowercase" style.

- **`UpperCase`**

  Rename direct children to "UPPERCASE" style.

- **`PascalCase`**

  Rename direct children to "PascalCase" style, as typically used for
  enum variants.

- **`CamelCase`**

  Rename direct children to "camelCase" style.

- **`SnakeCase`**

  Rename direct children to "snake_case" style, as commonly used for
  fields.

- **`ScreamingSnakeCase`**

  Rename direct children to "SCREAMING_SNAKE_CASE" style, as commonly
  used for constants.

- **`KebabCase`**

  Rename direct children to "kebab-case" style.

- **`ScreamingKebabCase`**

  Rename direct children to "SCREAMING-KEBAB-CASE" style.

#### Implementations

- <span id="renamerule-from-str"></span>`fn from_str(rename_all_str: &str) -> Result<Self, ParseError<'_>>` — [`ParseError`](#parseerror)

- <span id="renamerule-apply-to-variant"></span>`fn apply_to_variant(self, variant: &str) -> String`

  Apply a renaming rule to an enum variant, returning the version expected in the source.

- <span id="renamerule-apply-to-field"></span>`fn apply_to_field(self, field: &str) -> String`

  Apply a renaming rule to a struct field, returning the version expected in the source.

- <span id="renamerule-or"></span>`fn or(self, rule_b: Self) -> Self`

  Returns the `RenameRule` if it is not `None`, `rule_b` otherwise.

#### Trait Implementations

##### `impl Clone for RenameRule`

- <span id="renamerule-clone"></span>`fn clone(&self) -> RenameRule` — [`RenameRule`](#renamerule)

##### `impl Copy for RenameRule`

##### `impl PartialEq for RenameRule`

- <span id="renamerule-partialeq-eq"></span>`fn eq(&self, other: &RenameRule) -> bool` — [`RenameRule`](#renamerule)

##### `impl StructuralPartialEq for RenameRule`

