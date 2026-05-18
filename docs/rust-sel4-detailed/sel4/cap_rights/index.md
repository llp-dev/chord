*[sel4](../index.md) / [cap_rights](index.md)*

---

# Module `cap_rights`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CapRights`](#caprights) | struct | Corresponds to `seL4_CapRights_t`. |
| [`CapRightsBuilder`](#caprightsbuilder) | struct | Helper for constructing [`CapRights`]. |

## Structs

### `CapRights`

```rust
struct CapRights(sys::seL4_CapRights);
```

Corresponds to `seL4_CapRights_t`.

#### Implementations

- <span id="caprights-from-inner"></span>`const fn from_inner(inner: sys::seL4_CapRights) -> Self`

- <span id="caprights-into-inner"></span>`const fn into_inner(self) -> sys::seL4_CapRights`

- <span id="caprights-inner"></span>`const fn inner(&self) -> &sys::seL4_CapRights`

- <span id="caprights-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_CapRights`

- <span id="caprights-new"></span>`fn new(grant_reply: bool, grant: bool, read: bool, write: bool) -> Self`

- <span id="caprights-none"></span>`fn none() -> Self`

- <span id="caprights-all"></span>`fn all() -> Self`

- <span id="caprights-read-write"></span>`fn read_write() -> Self`

- <span id="caprights-read-only"></span>`fn read_only() -> Self`

- <span id="caprights-write-only"></span>`fn write_only() -> Self`

#### Trait Implementations

##### `impl Clone for CapRights`

- <span id="caprights-clone"></span>`fn clone(&self) -> CapRights` — [`CapRights`](#caprights)

##### `impl Debug for CapRights`

- <span id="caprights-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CapRights`

##### `impl PartialEq for CapRights`

- <span id="caprights-partialeq-eq"></span>`fn eq(&self, other: &CapRights) -> bool` — [`CapRights`](#caprights)

##### `impl StructuralPartialEq for CapRights`

### `CapRightsBuilder`

```rust
struct CapRightsBuilder {
    grant_reply: bool,
    grant: bool,
    read: bool,
    write: bool,
}
```

Helper for constructing [`CapRights`](#caprights).

#### Implementations

- <span id="caprightsbuilder-none"></span>`fn none() -> Self`

- <span id="caprightsbuilder-all"></span>`fn all() -> Self`

- <span id="caprightsbuilder-build"></span>`fn build(self) -> CapRights` — [`CapRights`](#caprights)

- <span id="caprightsbuilder-grant-reply"></span>`fn grant_reply(self, can: bool) -> Self`

- <span id="caprightsbuilder-grant"></span>`fn grant(self, can: bool) -> Self`

- <span id="caprightsbuilder-read"></span>`fn read(self, can: bool) -> Self`

- <span id="caprightsbuilder-write"></span>`fn write(self, can: bool) -> Self`

#### Trait Implementations

##### `impl Clone for CapRightsBuilder`

- <span id="caprightsbuilder-clone"></span>`fn clone(&self) -> CapRightsBuilder` — [`CapRightsBuilder`](#caprightsbuilder)

##### `impl Copy for CapRightsBuilder`

##### `impl Debug for CapRightsBuilder`

- <span id="caprightsbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CapRightsBuilder`

- <span id="caprightsbuilder-default"></span>`fn default() -> CapRightsBuilder` — [`CapRightsBuilder`](#caprightsbuilder)

##### `impl Eq for CapRightsBuilder`

##### `impl PartialEq for CapRightsBuilder`

- <span id="caprightsbuilder-partialeq-eq"></span>`fn eq(&self, other: &CapRightsBuilder) -> bool` — [`CapRightsBuilder`](#caprightsbuilder)

##### `impl StructuralPartialEq for CapRightsBuilder`

