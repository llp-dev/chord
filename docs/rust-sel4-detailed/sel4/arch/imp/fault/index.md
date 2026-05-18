*[sel4](../../../index.md) / [arch](../../index.md) / [imp](../index.md) / [fault](index.md)*

---

# Module `fault`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NullFault`](#nullfault) | struct | Corresponds to ` seL4_Fault_NullFault `. |
| [`CapFault`](#capfault) | struct | Corresponds to ` seL4_Fault_CapFault `. |
| [`UnknownSyscall`](#unknownsyscall) | struct | Corresponds to ` seL4_Fault_UnknownSyscall `. |
| [`UserException`](#userexception) | struct | Corresponds to ` seL4_Fault_UserException `. |
| [`VmFault`](#vmfault) | struct | Corresponds to ` seL4_Fault_VMFault `. |
| [`Fault`](#fault) | enum |  |

## Structs

### `NullFault`

```rust
struct NullFault(sys::seL4_Fault_NullFault);
```

Corresponds to `
seL4_Fault_NullFault
`.

#### Implementations

- <span id="nullfault-from-inner"></span>`const fn from_inner(inner: sys::seL4_Fault_NullFault) -> Self`

- <span id="nullfault-into-inner"></span>`const fn into_inner(self) -> sys::seL4_Fault_NullFault`

- <span id="nullfault-inner"></span>`const fn inner(&self) -> &sys::seL4_Fault_NullFault`

- <span id="nullfault-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_Fault_NullFault`

#### Trait Implementations

##### `impl Clone for NullFault`

- <span id="nullfault-clone"></span>`fn clone(&self) -> NullFault` — [`NullFault`](../../../fault/index.md#nullfault)

##### `impl Debug for NullFault`

- <span id="nullfault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NullFault`

##### `impl PartialEq for NullFault`

- <span id="nullfault-partialeq-eq"></span>`fn eq(&self, other: &NullFault) -> bool` — [`NullFault`](../../../fault/index.md#nullfault)

##### `impl StructuralPartialEq for NullFault`

### `CapFault`

```rust
struct CapFault(sys::seL4_Fault_CapFault);
```

Corresponds to `
seL4_Fault_CapFault
`.

#### Implementations

- <span id="capfault-from-inner"></span>`const fn from_inner(inner: sys::seL4_Fault_CapFault) -> Self`

- <span id="capfault-into-inner"></span>`const fn into_inner(self) -> sys::seL4_Fault_CapFault`

- <span id="capfault-inner"></span>`const fn inner(&self) -> &sys::seL4_Fault_CapFault`

- <span id="capfault-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_Fault_CapFault`

#### Trait Implementations

##### `impl Clone for CapFault`

- <span id="capfault-clone"></span>`fn clone(&self) -> CapFault` — [`CapFault`](../../../fault/index.md#capfault)

##### `impl Debug for CapFault`

- <span id="capfault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CapFault`

##### `impl PartialEq for CapFault`

- <span id="capfault-partialeq-eq"></span>`fn eq(&self, other: &CapFault) -> bool` — [`CapFault`](../../../fault/index.md#capfault)

##### `impl StructuralPartialEq for CapFault`

### `UnknownSyscall`

```rust
struct UnknownSyscall(sys::seL4_Fault_UnknownSyscall);
```

Corresponds to `
seL4_Fault_UnknownSyscall
`.

#### Implementations

- <span id="unknownsyscall-from-inner"></span>`const fn from_inner(inner: sys::seL4_Fault_UnknownSyscall) -> Self`

- <span id="unknownsyscall-into-inner"></span>`const fn into_inner(self) -> sys::seL4_Fault_UnknownSyscall`

- <span id="unknownsyscall-inner"></span>`const fn inner(&self) -> &sys::seL4_Fault_UnknownSyscall`

- <span id="unknownsyscall-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_Fault_UnknownSyscall`

#### Trait Implementations

##### `impl Clone for UnknownSyscall`

- <span id="unknownsyscall-clone"></span>`fn clone(&self) -> UnknownSyscall` — [`UnknownSyscall`](../../../fault/index.md#unknownsyscall)

##### `impl Debug for UnknownSyscall`

- <span id="unknownsyscall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnknownSyscall`

##### `impl PartialEq for UnknownSyscall`

- <span id="unknownsyscall-partialeq-eq"></span>`fn eq(&self, other: &UnknownSyscall) -> bool` — [`UnknownSyscall`](../../../fault/index.md#unknownsyscall)

##### `impl StructuralPartialEq for UnknownSyscall`

### `UserException`

```rust
struct UserException(sys::seL4_Fault_UserException);
```

Corresponds to `
seL4_Fault_UserException
`.

#### Implementations

- <span id="userexception-from-inner"></span>`const fn from_inner(inner: sys::seL4_Fault_UserException) -> Self`

- <span id="userexception-into-inner"></span>`const fn into_inner(self) -> sys::seL4_Fault_UserException`

- <span id="userexception-inner"></span>`const fn inner(&self) -> &sys::seL4_Fault_UserException`

- <span id="userexception-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_Fault_UserException`

#### Trait Implementations

##### `impl Clone for UserException`

- <span id="userexception-clone"></span>`fn clone(&self) -> UserException` — [`UserException`](../../../fault/index.md#userexception)

##### `impl Debug for UserException`

- <span id="userexception-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UserException`

##### `impl PartialEq for UserException`

- <span id="userexception-partialeq-eq"></span>`fn eq(&self, other: &UserException) -> bool` — [`UserException`](../../../fault/index.md#userexception)

##### `impl StructuralPartialEq for UserException`

### `VmFault`

```rust
struct VmFault(sys::seL4_Fault_VMFault);
```

Corresponds to `
seL4_Fault_VMFault
`.

#### Implementations

- <span id="vmfault-from-inner"></span>`const fn from_inner(inner: sys::seL4_Fault_VMFault) -> Self`

- <span id="vmfault-into-inner"></span>`const fn into_inner(self) -> sys::seL4_Fault_VMFault`

- <span id="vmfault-inner"></span>`const fn inner(&self) -> &sys::seL4_Fault_VMFault`

- <span id="vmfault-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_Fault_VMFault`

#### Trait Implementations

##### `impl Clone for VmFault`

- <span id="vmfault-clone"></span>`fn clone(&self) -> VmFault` — [`VmFault`](../../../fault/index.md#vmfault)

##### `impl Debug for VmFault`

- <span id="vmfault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for VmFault`

##### `impl PartialEq for VmFault`

- <span id="vmfault-partialeq-eq"></span>`fn eq(&self, other: &VmFault) -> bool` — [`VmFault`](../../../fault/index.md#vmfault)

##### `impl StructuralPartialEq for VmFault`

## Enums

### `Fault`

```rust
enum Fault {
    NullFault(NullFault),
    CapFault(CapFault),
    UnknownSyscall(UnknownSyscall),
    UserException(UserException),
    VmFault(VmFault),
}
```

#### Implementations

- <span id="fault-from-sys"></span>`fn from_sys(raw: sys::seL4_Fault) -> Self`

#### Trait Implementations

##### `impl Clone for Fault`

- <span id="fault-clone"></span>`fn clone(&self) -> Fault` — [`Fault`](../../../fault/index.md#fault)

##### `impl Debug for Fault`

- <span id="fault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Fault`

##### `impl PartialEq for Fault`

- <span id="fault-partialeq-eq"></span>`fn eq(&self, other: &Fault) -> bool` — [`Fault`](../../../fault/index.md#fault)

##### `impl StructuralPartialEq for Fault`

