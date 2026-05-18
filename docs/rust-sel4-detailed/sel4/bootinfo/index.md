*[sel4](../index.md) / [bootinfo](index.md)*

---

# Module `bootinfo`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BootInfoPtr`](#bootinfoptr) | struct | A wrapped pointer to a [`BootInfo`] block. |
| [`BootInfo`](#bootinfo) | struct | Corresponds to `seL4_BootInfo`. |
| [`UntypedDesc`](#untypeddesc) | struct | Corresponds to `seL4_UntypedDesc`. |
| [`BootInfoExtra`](#bootinfoextra) | struct | An extra bootinfo chunk along with its ID. |
| [`BootInfoExtraIter`](#bootinfoextraiter) | struct | An iterator for accessing the [`BootInfoExtra`] entires associated with a [`BootInfoPtr`]. |
| [`BootInfoExtraId`](#bootinfoextraid) | enum | Corresponds to `seL4_BootInfoID`. |

## Structs

### `BootInfoPtr`

```rust
struct BootInfoPtr {
    ptr: *const BootInfo,
}
```

A wrapped pointer to a [`BootInfo`](#bootinfo) block.

Access [`BootInfo`](#bootinfo) via `Deref`, and [`BootInfoExtraIter`](#bootinfoextraiter) via [`extra`](BootInfoPtr::extra).

#### Implementations

- <span id="bootinfoptr-new"></span>`unsafe fn new(ptr: *const BootInfo) -> Self` — [`BootInfo`](#bootinfo)

- <span id="bootinfoptr-ptr"></span>`fn ptr(&self) -> *const BootInfo` — [`BootInfo`](#bootinfo)

- <span id="bootinfoptr-extra-ptr"></span>`fn extra_ptr(&self) -> *const u8`

- <span id="bootinfoptr-extra-slice"></span>`fn extra_slice(&self) -> &[u8]`

- <span id="bootinfoptr-extra"></span>`fn extra(&self) -> BootInfoExtraIter<'_>` — [`BootInfoExtraIter`](#bootinfoextraiter)

- <span id="bootinfoptr-footprint-size"></span>`fn footprint_size(&self) -> usize`

- <span id="bootinfoptr-const-extra-offset"></span>`const EXTRA_OFFSET: usize`

#### Trait Implementations

##### `impl Debug for BootInfoPtr`

- <span id="bootinfoptr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for BootInfoPtr`

- <span id="bootinfoptr-deref-type-target"></span>`type Target = BootInfo`

- <span id="bootinfoptr-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for BootInfoPtr`

- <span id="bootinfoptr-receiver-type-target"></span>`type Target = T`

### `BootInfo`

```rust
struct BootInfo(sys::seL4_BootInfo);
```

Corresponds to `seL4_BootInfo`.

#### Implementations

- <span id="bootinfo-from-inner"></span>`const fn from_inner(inner: sys::seL4_BootInfo) -> Self`

- <span id="bootinfo-into-inner"></span>`const fn into_inner(self) -> sys::seL4_BootInfo`

- <span id="bootinfo-inner"></span>`const fn inner(&self) -> &sys::seL4_BootInfo`

- <span id="bootinfo-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_BootInfo`

- <span id="bootinfo-extra-len"></span>`fn extra_len(&self) -> usize`

- <span id="bootinfo-ipc-buffer"></span>`fn ipc_buffer(&self) -> *mut IpcBuffer` — [`IpcBuffer`](../ipc_buffer/index.md#ipcbuffer)

- <span id="bootinfo-empty"></span>`fn empty(&self) -> SlotRegion<cap_type::Null>` — [`SlotRegion`](../init_thread/index.md#slotregion), [`Null`](../cptr/cap_type/index.md#null)

- <span id="bootinfo-user-image-frames"></span>`fn user_image_frames(&self) -> SlotRegion<cap_type::Granule>` — [`SlotRegion`](../init_thread/index.md#slotregion), [`Granule`](../cptr/cap_type/index.md#granule)

- <span id="bootinfo-untyped"></span>`fn untyped(&self) -> SlotRegion<cap_type::Untyped>` — [`SlotRegion`](../init_thread/index.md#slotregion), [`Untyped`](../cptr/cap_type/index.md#untyped)

- <span id="bootinfo-untyped-list-inner"></span>`fn untyped_list_inner(&self) -> &[sys::seL4_UntypedDesc]`

- <span id="bootinfo-untyped-list"></span>`fn untyped_list(&self) -> &[UntypedDesc]` — [`UntypedDesc`](#untypeddesc)

- <span id="bootinfo-untyped-list-partition-point"></span>`fn untyped_list_partition_point(&self) -> usize`

- <span id="bootinfo-device-untyped-range"></span>`fn device_untyped_range(&self) -> Range<usize>`

- <span id="bootinfo-kernel-untyped-range"></span>`fn kernel_untyped_range(&self) -> Range<usize>`

#### Trait Implementations

##### `impl Debug for BootInfo`

- <span id="bootinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UntypedDesc`

```rust
struct UntypedDesc(sys::seL4_UntypedDesc);
```

Corresponds to `seL4_UntypedDesc`.

#### Implementations

- <span id="untypeddesc-from-inner"></span>`const fn from_inner(inner: sys::seL4_UntypedDesc) -> Self`

- <span id="untypeddesc-into-inner"></span>`const fn into_inner(self) -> sys::seL4_UntypedDesc`

- <span id="untypeddesc-inner"></span>`const fn inner(&self) -> &sys::seL4_UntypedDesc`

- <span id="untypeddesc-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_UntypedDesc`

- <span id="untypeddesc-paddr"></span>`fn paddr(&self) -> usize`

- <span id="untypeddesc-size-bits"></span>`fn size_bits(&self) -> usize`

- <span id="untypeddesc-is-device"></span>`fn is_device(&self) -> bool`

#### Trait Implementations

##### `impl Clone for UntypedDesc`

- <span id="untypeddesc-clone"></span>`fn clone(&self) -> UntypedDesc` — [`UntypedDesc`](#untypeddesc)

##### `impl Debug for UntypedDesc`

- <span id="untypeddesc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UntypedDesc`

##### `impl PartialEq for UntypedDesc`

- <span id="untypeddesc-partialeq-eq"></span>`fn eq(&self, other: &UntypedDesc) -> bool` — [`UntypedDesc`](#untypeddesc)

##### `impl StructuralPartialEq for UntypedDesc`

### `BootInfoExtra<'a>`

```rust
struct BootInfoExtra<'a> {
    pub id: BootInfoExtraId,
    pub content_with_header: &'a [u8],
}
```

An extra bootinfo chunk along with its ID.

#### Implementations

- <span id="bootinfoextra-content-with-header"></span>`fn content_with_header(&self) -> &[u8]`

- <span id="bootinfoextra-content"></span>`fn content(&self) -> &[u8]`

#### Trait Implementations

##### `impl Clone for BootInfoExtra<'a>`

- <span id="bootinfoextra-clone"></span>`fn clone(&self) -> BootInfoExtra<'a>` — [`BootInfoExtra`](#bootinfoextra)

##### `impl Debug for BootInfoExtra<'a>`

- <span id="bootinfoextra-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BootInfoExtra<'a>`

##### `impl PartialEq for BootInfoExtra<'a>`

- <span id="bootinfoextra-partialeq-eq"></span>`fn eq(&self, other: &BootInfoExtra<'a>) -> bool` — [`BootInfoExtra`](#bootinfoextra)

##### `impl StructuralPartialEq for BootInfoExtra<'a>`

### `BootInfoExtraIter<'a>`

```rust
struct BootInfoExtraIter<'a> {
    bootinfo: &'a BootInfoPtr,
    cursor: usize,
}
```

An iterator for accessing the [`BootInfoExtra`](#bootinfoextra) entires associated with a [`BootInfoPtr`](#bootinfoptr).

#### Implementations

- <span id="bootinfoextraiter-new"></span>`fn new(bootinfo: &'a BootInfoPtr) -> Self` — [`BootInfoPtr`](#bootinfoptr)

#### Trait Implementations

##### `impl IntoIterator for BootInfoExtraIter<'a>`

- <span id="bootinfoextraiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="bootinfoextraiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="bootinfoextraiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for BootInfoExtraIter<'a>`

- <span id="bootinfoextraiter-iterator-type-item"></span>`type Item = BootInfoExtra<'a>`

- <span id="bootinfoextraiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `BootInfoExtraId`

```rust
enum BootInfoExtraId {
    Padding,
    Fdt,
}
```

Corresponds to `seL4_BootInfoID`.

#### Implementations

- <span id="bootinfoextraid-from-sys"></span>`fn from_sys(id: sys::seL4_BootInfoID::Type) -> Option<Self>`

#### Trait Implementations

##### `impl Clone for BootInfoExtraId`

- <span id="bootinfoextraid-clone"></span>`fn clone(&self) -> BootInfoExtraId` — [`BootInfoExtraId`](#bootinfoextraid)

##### `impl Debug for BootInfoExtraId`

- <span id="bootinfoextraid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BootInfoExtraId`

##### `impl PartialEq for BootInfoExtraId`

- <span id="bootinfoextraid-partialeq-eq"></span>`fn eq(&self, other: &BootInfoExtraId) -> bool` — [`BootInfoExtraId`](#bootinfoextraid)

##### `impl StructuralPartialEq for BootInfoExtraId`

