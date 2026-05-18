*[sel4](../../../index.md) / [arch](../../index.md) / [imp](../index.md) / [vm_attributes](index.md)*

---

# Module `vm_attributes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`VmAttributes`](#vmattributes) | struct | Corresponds to `seL4_X86_VMAttributes`. |

## Structs

### `VmAttributes`

```rust
struct VmAttributes(sys::seL4_X86_VMAttributes::Type);
```

Corresponds to `seL4_X86_VMAttributes`.

#### Implementations

- <span id="vmattributes-const-none"></span>`const NONE: Self`

- <span id="vmattributes-const-default"></span>`const DEFAULT: Self`

- <span id="vmattributes-const-cache-disabled"></span>`const CACHE_DISABLED: Self`

- <span id="vmattributes-from-inner"></span>`const fn from_inner(inner: sys::seL4_X86_VMAttributes::Type) -> Self`

- <span id="vmattributes-into-inner"></span>`const fn into_inner(self) -> sys::seL4_X86_VMAttributes::Type`

- <span id="vmattributes-inner"></span>`const fn inner(&self) -> &sys::seL4_X86_VMAttributes::Type`

- <span id="vmattributes-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_X86_VMAttributes::Type`

- <span id="vmattributes-has"></span>`const fn has(self, rhs: Self) -> bool`

#### Trait Implementations

##### `impl BitAnd for VmAttributes`

- <span id="vmattributes-bitand-type-output"></span>`type Output = VmAttributes`

- <span id="vmattributes-bitand"></span>`fn bitand(self, rhs: Self) -> Self`

##### `impl BitAndAssign for VmAttributes`

- <span id="vmattributes-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: Self)`

##### `impl BitOr for VmAttributes`

- <span id="vmattributes-bitor-type-output"></span>`type Output = VmAttributes`

- <span id="vmattributes-bitor"></span>`fn bitor(self, rhs: Self) -> Self`

##### `impl BitOrAssign for VmAttributes`

- <span id="vmattributes-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: Self)`

##### `impl Clone for VmAttributes`

- <span id="vmattributes-clone"></span>`fn clone(&self) -> VmAttributes` — [`VmAttributes`](#vmattributes)

##### `impl Copy for VmAttributes`

##### `impl Debug for VmAttributes`

- <span id="vmattributes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for VmAttributes`

- <span id="vmattributes-default"></span>`fn default() -> Self`

##### `impl Eq for VmAttributes`

##### `impl Not for VmAttributes`

- <span id="vmattributes-not-type-output"></span>`type Output = VmAttributes`

- <span id="vmattributes-not"></span>`fn not(self) -> Self`

##### `impl PartialEq for VmAttributes`

- <span id="vmattributes-partialeq-eq"></span>`fn eq(&self, other: &VmAttributes) -> bool` — [`VmAttributes`](#vmattributes)

##### `impl StructuralPartialEq for VmAttributes`

