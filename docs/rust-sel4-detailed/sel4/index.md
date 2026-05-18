# Crate `sel4`

This crate provides straightforward, pure-Rust bindings to the [seL4
API](https://sel4.systems/Info/Docs/seL4-manual-latest.pdf).

Most items in this crate correspond to types, constants, and functions in `libsel4`. See the
[seL4 manual](https://sel4.systems/Info/Docs/seL4-manual-latest.pdf) for more information about
the seL4 concepts and `libsel4` items references in this crate's documentation.

This crate's implementation is based on the lower-level [`sel4-sys`](::sel4_sys) crate, which is
generated from the libsel4 headers and interface definition files.

### Features

#### `"state"`

Functions in the C libsel4 use the thread-local variable `__sel4_ipc_buffer` to obtain a pointer
to the current thread's IPC buffer:

```C
extern __thread seL4_IPCBuffer *__sel4_ipc_buffer;
```

[libmicrokit](https://github.com/seL4/microkit/tree/main/libmicrokit), which does not support
thread-local storage uses the following snippet to force `__sel4_ipc_buffer` to global rather
than thread-local:

```C
#define __thread
#include <sel4/sel4.h>
```

For the sake of flexibility and applicability, this crate can be configured to use no state at
all. Users can opt out of state and explicitly pass around references to the active IPC buffer
instead of relying on the implementation to obtain such a reference using thread-local or global
state. Such a paradigm is useful in certain uncommon circumstances, but most users will benefit
from the convenience of an implicit IPC buffer. The `"state"` feature, enabled by default, uses
state to allow one to make seL4 API calls without having to explicitly specify an IPC buffer.

For the sake of interoperability with C, the state looks something like: `static mut
__sel4_ipc_buffer: *mut IpcBuffer`. If the `"state-exposed"` feature is enabled, it is exposed
with `#![no_mangle]`. If the `"state-extern"` feature is enabled, it is wrapped in an `extern
"C"` block. Whether it is thread-local is determined by the following pseudocode:

```rust
cfg_if! {
    if #[cfg(all(any(target_thread_local, feature = "tls"), not(feature = "non-thread-local-state")))] {
        // thread-local
    } else if #[cfg(not(feature = "thread-local-state"))] {
        // not thread-local
    } else {
        compile_error!(r#"invalid configuration"#);
    }
}
```

The non-thread-local configuration should only be used in cases where the language runtime does
not support thread-local storage. In those cases without thread-local storage where this crate
will only ever run in a single thread, use the `"single-threaded"` feature to enable a more
efficient implementation. Note that enabling the `"single-threaded"` feature in a case where
this crate runs in more than one thread is unsafe.

At the API level, the `"state"` feature causes [`NoExplicitInvocationContext`](invocation_context/index.md) to be an alias
for [`ImplicitInvocationContext`](state/index.md), which implements [`InvocationContext`](invocation_context/index.md) by accessing the
thread-local pointer to an IPC buffer. When the `"state"` feature is not enabled,
[`NoExplicitInvocationContext`](invocation_context/index.md) is an alias for [`NoInvocationContext`](invocation_context/index.md), which does not
implement [`InvocationContext`](invocation_context/index.md). The thread-local IPC buffer pointer is modified and accessed by
[`set_ipc_buffer`](state/index.md), [`with_ipc_buffer`](state/index.md), and [`with_ipc_buffer_mut`](state/index.md). The lower-level
[`try_with_ipc_buffer_slot`](state/index.md) and [`try_with_ipc_buffer_slot_mut`](state/index.md) are provided as well.

### Building

This crate and its dependencies depend, at build time, on the libsel4 headers. The location of
these headers is provided to this crate at build time by environment variables. If
`SEL4_INCLUDE_DIRS` is set, then its value is interpreted as a colon-separated list of include
paths for the libsel4 headers. Otherwise, if `SEL4_PREFIX` is set, then
`$SEL4_PREFIX/libsel4/include` is used.

## Contents

- [Modules](#modules)
  - [`arch`](#arch)
  - [`bootinfo`](#bootinfo)
  - [`cap_rights`](#cap-rights)
  - [`cnode_cap_data`](#cnode-cap-data)
  - [`const_helpers`](#const-helpers)
  - [`cptr`](#cptr)
  - [`error`](#error)
  - [`fault`](#fault)
  - [`helper_macros`](#helper-macros)
  - [`invocation_context`](#invocation-context)
  - [`invocations`](#invocations)
  - [`ipc_buffer`](#ipc-buffer)
  - [`message_info`](#message-info)
  - [`object`](#object)
  - [`reply_authority`](#reply-authority)
  - [`syscalls`](#syscalls)
  - [`vspace`](#vspace)
  - [`init_thread`](#init-thread)
  - [`printing`](#printing)
  - [`debug`](#debug)
  - [`state`](#state)
  - [`cap`](#cap)
  - [`cap_type`](#cap-type)
  - [`vspace_levels`](#vspace-levels)
- [Structs](#structs)
  - [`sel4_cfg_bool`](#sel4-cfg-bool)
  - [`BootInfo`](#bootinfo)
  - [`BootInfoExtra`](#bootinfoextra)
  - [`BootInfoExtraIter`](#bootinfoextraiter)
  - [`BootInfoPtr`](#bootinfoptr)
  - [`UntypedDesc`](#untypeddesc)
  - [`CapRights`](#caprights)
  - [`CapRightsBuilder`](#caprightsbuilder)
  - [`CNodeCapData`](#cnodecapdata)
  - [`AbsoluteCPtr`](#absolutecptr)
  - [`CPtr`](#cptr)
  - [`CPtrWithDepth`](#cptrwithdepth)
  - [`Cap`](#cap)
  - [`NoInvocationContext`](#noinvocationcontext)
  - [`TcbFlagsBuilder`](#tcbflagsbuilder)
  - [`IpcBuffer`](#ipcbuffer)
  - [`MessageInfo`](#messageinfo)
  - [`MessageInfoBuilder`](#messageinfobuilder)
  - [`CallWithMRs`](#callwithmrs)
  - [`RecvWithMRs`](#recvwithmrs)
  - [`ImplicitReplyAuthority`](#implicitreplyauthority)
  - [`DebugWrite`](#debugwrite)
  - [`ImplicitInvocationContext`](#implicitinvocationcontext)
- [Enums](#enums)
  - [`BootInfoExtraId`](#bootinfoextraid)
  - [`Error`](#error)
  - [`ObjectBlueprint`](#objectblueprint)
  - [`ObjectType`](#objecttype)
- [Traits](#traits)
  - [`CapType`](#captype)
  - [`HasCPtrWithDepth`](#hascptrwithdepth)
  - [`InvocationContext`](#invocationcontext)
  - [`CapTypeForObject`](#captypeforobject)
  - [`CapTypeForObjectOfFixedSize`](#captypeforobjectoffixedsize)
  - [`CapTypeForObjectOfVariableSize`](#captypeforobjectofvariablesize)
  - [`ConveysReplyAuthority`](#conveysreplyauthority)
  - [`FastMessages`](#fastmessages)
  - [`IpcCapType`](#ipccaptype)
  - [`CapTypeForFrameObject`](#captypeforframeobject)
  - [`CapTypeForFrameObjectOfFixedSize`](#captypeforframeobjectoffixedsize)
  - [`CapTypeForTranslationTableObject`](#captypefortranslationtableobject)
- [Functions](#functions)
  - [`sys`](#sys)
  - [`yield`](#yield)
  - [`reply`](#reply)
  - [`debug_put_char`](#debug-put-char)
  - [`debug_snapshot`](#debug-snapshot)
  - [`debug_halt`](#debug-halt)
  - [`ipc_buffer_is_thread_local`](#ipc-buffer-is-thread-local)
  - [`set_ipc_buffer`](#set-ipc-buffer)
  - [`try_with_ipc_buffer_slot`](#try-with-ipc-buffer-slot)
  - [`try_with_ipc_buffer_slot_mut`](#try-with-ipc-buffer-slot-mut)
  - [`with_ipc_buffer`](#with-ipc-buffer)
  - [`with_ipc_buffer_mut`](#with-ipc-buffer-mut)
- [Type Aliases](#type-aliases)
  - [`CPtrBits`](#cptrbits)
  - [`Result`](#result)
  - [`NoExplicitInvocationContext`](#noexplicitinvocationcontext)
  - [`ReplyAuthority`](#replyauthority)
  - [`Badge`](#badge)
  - [`Word`](#word)
- [Constants](#constants)
  - [`NUM_MESSAGE_REGISTERS`](#num-message-registers)
  - [`WORD_SIZE`](#word-size)
- [Macros](#macros)
  - [`debug_print!`](#debug-print)
  - [`debug_println!`](#debug-println)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`arch`](#arch) | mod |  |
| [`bootinfo`](#bootinfo) | mod |  |
| [`cap_rights`](#cap-rights) | mod |  |
| [`cnode_cap_data`](#cnode-cap-data) | mod |  |
| [`const_helpers`](#const-helpers) | mod |  |
| [`cptr`](#cptr) | mod |  |
| [`error`](#error) | mod |  |
| [`fault`](#fault) | mod | Fault types. |
| [`helper_macros`](#helper-macros) | mod |  |
| [`invocation_context`](#invocation-context) | mod |  |
| [`invocations`](#invocations) | mod |  |
| [`ipc_buffer`](#ipc-buffer) | mod |  |
| [`message_info`](#message-info) | mod |  |
| [`object`](#object) | mod |  |
| [`reply_authority`](#reply-authority) | mod |  |
| [`syscalls`](#syscalls) | mod |  |
| [`vspace`](#vspace) | mod |  |
| [`init_thread`](#init-thread) | mod | Items that are applicable within the context of the root task's initial thread's CSpace. |
| [`printing`](#printing) | mod |  |
| [`debug`](#debug) | mod |  |
| [`state`](#state) | mod |  |
| [`cap`](#cap) | mod |  |
| [`cap_type`](#cap-type) | mod |  |
| [`vspace_levels`](#vspace-levels) | mod |  |
| [`sel4_cfg_bool`](#sel4-cfg-bool) | struct |  |
| [`BootInfo`](#bootinfo) | struct |  |
| [`BootInfoExtra`](#bootinfoextra) | struct |  |
| [`BootInfoExtraIter`](#bootinfoextraiter) | struct |  |
| [`BootInfoPtr`](#bootinfoptr) | struct |  |
| [`UntypedDesc`](#untypeddesc) | struct |  |
| [`CapRights`](#caprights) | struct |  |
| [`CapRightsBuilder`](#caprightsbuilder) | struct |  |
| [`CNodeCapData`](#cnodecapdata) | struct |  |
| [`AbsoluteCPtr`](#absolutecptr) | struct |  |
| [`CPtr`](#cptr) | struct |  |
| [`CPtrWithDepth`](#cptrwithdepth) | struct |  |
| [`Cap`](#cap) | struct |  |
| [`NoInvocationContext`](#noinvocationcontext) | struct |  |
| [`TcbFlagsBuilder`](#tcbflagsbuilder) | struct |  |
| [`IpcBuffer`](#ipcbuffer) | struct |  |
| [`MessageInfo`](#messageinfo) | struct |  |
| [`MessageInfoBuilder`](#messageinfobuilder) | struct |  |
| [`CallWithMRs`](#callwithmrs) | struct |  |
| [`RecvWithMRs`](#recvwithmrs) | struct |  |
| [`ImplicitReplyAuthority`](#implicitreplyauthority) | struct |  |
| [`DebugWrite`](#debugwrite) | struct |  |
| [`ImplicitInvocationContext`](#implicitinvocationcontext) | struct |  |
| [`BootInfoExtraId`](#bootinfoextraid) | enum |  |
| [`Error`](#error) | enum |  |
| [`ObjectBlueprint`](#objectblueprint) | enum |  |
| [`ObjectType`](#objecttype) | enum |  |
| [`CapType`](#captype) | trait |  |
| [`HasCPtrWithDepth`](#hascptrwithdepth) | trait |  |
| [`InvocationContext`](#invocationcontext) | trait |  |
| [`CapTypeForObject`](#captypeforobject) | trait |  |
| [`CapTypeForObjectOfFixedSize`](#captypeforobjectoffixedsize) | trait |  |
| [`CapTypeForObjectOfVariableSize`](#captypeforobjectofvariablesize) | trait |  |
| [`ConveysReplyAuthority`](#conveysreplyauthority) | trait |  |
| [`FastMessages`](#fastmessages) | trait |  |
| [`IpcCapType`](#ipccaptype) | trait |  |
| [`CapTypeForFrameObject`](#captypeforframeobject) | trait |  |
| [`CapTypeForFrameObjectOfFixedSize`](#captypeforframeobjectoffixedsize) | trait |  |
| [`CapTypeForTranslationTableObject`](#captypefortranslationtableobject) | trait |  |
| [`sys`](#sys) | fn |  |
| [`yield`](#yield) | fn |  |
| [`reply`](#reply) | fn |  |
| [`debug_put_char`](#debug-put-char) | fn |  |
| [`debug_snapshot`](#debug-snapshot) | fn |  |
| [`debug_halt`](#debug-halt) | fn |  |
| [`ipc_buffer_is_thread_local`](#ipc-buffer-is-thread-local) | fn |  |
| [`set_ipc_buffer`](#set-ipc-buffer) | fn |  |
| [`try_with_ipc_buffer_slot`](#try-with-ipc-buffer-slot) | fn |  |
| [`try_with_ipc_buffer_slot_mut`](#try-with-ipc-buffer-slot-mut) | fn |  |
| [`with_ipc_buffer`](#with-ipc-buffer) | fn |  |
| [`with_ipc_buffer_mut`](#with-ipc-buffer-mut) | fn |  |
| [`CPtrBits`](#cptrbits) | type |  |
| [`Result`](#result) | type |  |
| [`NoExplicitInvocationContext`](#noexplicitinvocationcontext) | type |  |
| [`ReplyAuthority`](#replyauthority) | type |  |
| [`Badge`](#badge) | type |  |
| [`Word`](#word) | type | Corresponds to `seL4_Word`. |
| [`NUM_MESSAGE_REGISTERS`](#num-message-registers) | const |  |
| [`WORD_SIZE`](#word-size) | const | The size of [`Word`] in bits. |
| [`debug_print!`](#debug-print) | macro | Like `std::debug_print!`, except backed by `seL4_DebugPutChar`. |
| [`debug_println!`](#debug-println) | macro | Like `std::debug_println!`, except backed by `seL4_DebugPutChar`. |

## Modules

- [`arch`](arch/index.md)
- [`bootinfo`](bootinfo/index.md)
- [`cap_rights`](cap_rights/index.md)
- [`cnode_cap_data`](cnode_cap_data/index.md)
- [`const_helpers`](const_helpers/index.md)
- [`cptr`](cptr/index.md)
- [`error`](error/index.md)
- [`fault`](fault/index.md) — Fault types.
- [`helper_macros`](helper_macros/index.md)
- [`invocation_context`](invocation_context/index.md)
- [`invocations`](invocations/index.md)
- [`ipc_buffer`](ipc_buffer/index.md)
- [`message_info`](message_info/index.md)
- [`object`](object/index.md)
- [`reply_authority`](reply_authority/index.md)
- [`syscalls`](syscalls/index.md)
- [`vspace`](vspace/index.md)
- [`init_thread`](init_thread/index.md) — Items that are applicable within the context of the root task's initial thread's CSpace.
- [`printing`](printing/index.md)
- [`debug`](debug/index.md)
- [`state`](state/index.md)
- [`cap`](cap/index.md) — Marked aliases of [`Cap`](crate::Cap).
- [`cap_type`](cap_type/index.md) — Markers corresponding to capability types and classes of capability types.
- [`vspace_levels`](vspace_levels/index.md) — Items describing the layout of address translation structures for this kernel configuration.

## Structs

### `sel4_cfg_bool`

```rust
struct sel4_cfg_bool;
```

*Re-exported from `chrono`*

#### Trait Implementations

##### `impl Expected for MonthVisitor`

- <span id="monthvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for MonthVisitor`

- <span id="monthvisitor-visitor-type-value"></span>`type Value = Month`

- <span id="monthvisitor-visitor-expecting"></span>`fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="monthvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, value: &str) -> Result<<Self as >::Value, E>`

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

- <span id="bootinfo-ipc-buffer"></span>`fn ipc_buffer(&self) -> *mut IpcBuffer` — [`IpcBuffer`](ipc_buffer/index.md#ipcbuffer)

- <span id="bootinfo-empty"></span>`fn empty(&self) -> SlotRegion<cap_type::Null>` — [`SlotRegion`](init_thread/index.md#slotregion), [`Null`](cptr/cap_type/index.md#null)

- <span id="bootinfo-user-image-frames"></span>`fn user_image_frames(&self) -> SlotRegion<cap_type::Granule>` — [`SlotRegion`](init_thread/index.md#slotregion), [`Granule`](cptr/cap_type/index.md#granule)

- <span id="bootinfo-untyped"></span>`fn untyped(&self) -> SlotRegion<cap_type::Untyped>` — [`SlotRegion`](init_thread/index.md#slotregion), [`Untyped`](cptr/cap_type/index.md#untyped)

- <span id="bootinfo-untyped-list-inner"></span>`fn untyped_list_inner(&self) -> &[sys::seL4_UntypedDesc]`

- <span id="bootinfo-untyped-list"></span>`fn untyped_list(&self) -> &[UntypedDesc]` — [`UntypedDesc`](bootinfo/index.md#untypeddesc)

- <span id="bootinfo-untyped-list-partition-point"></span>`fn untyped_list_partition_point(&self) -> usize`

- <span id="bootinfo-device-untyped-range"></span>`fn device_untyped_range(&self) -> Range<usize>`

- <span id="bootinfo-kernel-untyped-range"></span>`fn kernel_untyped_range(&self) -> Range<usize>`

#### Trait Implementations

##### `impl Debug for BootInfo`

- <span id="bootinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="bootinfoextra-clone"></span>`fn clone(&self) -> BootInfoExtra<'a>` — [`BootInfoExtra`](bootinfo/index.md#bootinfoextra)

##### `impl Debug for BootInfoExtra<'a>`

- <span id="bootinfoextra-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BootInfoExtra<'a>`

##### `impl PartialEq for BootInfoExtra<'a>`

- <span id="bootinfoextra-partialeq-eq"></span>`fn eq(&self, other: &BootInfoExtra<'a>) -> bool` — [`BootInfoExtra`](bootinfo/index.md#bootinfoextra)

##### `impl StructuralPartialEq for BootInfoExtra<'a>`

### `BootInfoExtraIter<'a>`

```rust
struct BootInfoExtraIter<'a> {
    bootinfo: &'a BootInfoPtr,
    cursor: usize,
}
```

An iterator for accessing the [`BootInfoExtra`](bootinfo/index.md) entires associated with a [`BootInfoPtr`](bootinfo/index.md).

#### Implementations

- <span id="bootinfoextraiter-new"></span>`fn new(bootinfo: &'a BootInfoPtr) -> Self` — [`BootInfoPtr`](bootinfo/index.md#bootinfoptr)

#### Trait Implementations

##### `impl IntoIterator for BootInfoExtraIter<'a>`

- <span id="bootinfoextraiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="bootinfoextraiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="bootinfoextraiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for BootInfoExtraIter<'a>`

- <span id="bootinfoextraiter-iterator-type-item"></span>`type Item = BootInfoExtra<'a>`

- <span id="bootinfoextraiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `BootInfoPtr`

```rust
struct BootInfoPtr {
    ptr: *const BootInfo,
}
```

A wrapped pointer to a [`BootInfo`](bootinfo/index.md) block.

Access [`BootInfo`](bootinfo/index.md) via `Deref`, and [`BootInfoExtraIter`](bootinfo/index.md) via [`extra`](BootInfoPtr::extra).

#### Implementations

- <span id="bootinfoptr-new"></span>`unsafe fn new(ptr: *const BootInfo) -> Self` — [`BootInfo`](bootinfo/index.md#bootinfo)

- <span id="bootinfoptr-ptr"></span>`fn ptr(&self) -> *const BootInfo` — [`BootInfo`](bootinfo/index.md#bootinfo)

- <span id="bootinfoptr-extra-ptr"></span>`fn extra_ptr(&self) -> *const u8`

- <span id="bootinfoptr-extra-slice"></span>`fn extra_slice(&self) -> &[u8]`

- <span id="bootinfoptr-extra"></span>`fn extra(&self) -> BootInfoExtraIter<'_>` — [`BootInfoExtraIter`](bootinfo/index.md#bootinfoextraiter)

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

- <span id="untypeddesc-clone"></span>`fn clone(&self) -> UntypedDesc` — [`UntypedDesc`](bootinfo/index.md#untypeddesc)

##### `impl Debug for UntypedDesc`

- <span id="untypeddesc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UntypedDesc`

##### `impl PartialEq for UntypedDesc`

- <span id="untypeddesc-partialeq-eq"></span>`fn eq(&self, other: &UntypedDesc) -> bool` — [`UntypedDesc`](bootinfo/index.md#untypeddesc)

##### `impl StructuralPartialEq for UntypedDesc`

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

- <span id="caprights-clone"></span>`fn clone(&self) -> CapRights` — [`CapRights`](cap_rights/index.md#caprights)

##### `impl Debug for CapRights`

- <span id="caprights-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CapRights`

##### `impl PartialEq for CapRights`

- <span id="caprights-partialeq-eq"></span>`fn eq(&self, other: &CapRights) -> bool` — [`CapRights`](cap_rights/index.md#caprights)

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

Helper for constructing [`CapRights`](cap_rights/index.md).

#### Implementations

- <span id="caprightsbuilder-none"></span>`fn none() -> Self`

- <span id="caprightsbuilder-all"></span>`fn all() -> Self`

- <span id="caprightsbuilder-build"></span>`fn build(self) -> CapRights` — [`CapRights`](cap_rights/index.md#caprights)

- <span id="caprightsbuilder-grant-reply"></span>`fn grant_reply(self, can: bool) -> Self`

- <span id="caprightsbuilder-grant"></span>`fn grant(self, can: bool) -> Self`

- <span id="caprightsbuilder-read"></span>`fn read(self, can: bool) -> Self`

- <span id="caprightsbuilder-write"></span>`fn write(self, can: bool) -> Self`

#### Trait Implementations

##### `impl Clone for CapRightsBuilder`

- <span id="caprightsbuilder-clone"></span>`fn clone(&self) -> CapRightsBuilder` — [`CapRightsBuilder`](cap_rights/index.md#caprightsbuilder)

##### `impl Copy for CapRightsBuilder`

##### `impl Debug for CapRightsBuilder`

- <span id="caprightsbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CapRightsBuilder`

- <span id="caprightsbuilder-default"></span>`fn default() -> CapRightsBuilder` — [`CapRightsBuilder`](cap_rights/index.md#caprightsbuilder)

##### `impl Eq for CapRightsBuilder`

##### `impl PartialEq for CapRightsBuilder`

- <span id="caprightsbuilder-partialeq-eq"></span>`fn eq(&self, other: &CapRightsBuilder) -> bool` — [`CapRightsBuilder`](cap_rights/index.md#caprightsbuilder)

##### `impl StructuralPartialEq for CapRightsBuilder`

### `CNodeCapData`

```rust
struct CNodeCapData(sys::seL4_CNode_CapData);
```

Corresponds to `seL4_CNode_CapData`.

#### Implementations

- <span id="cnodecapdata-from-inner"></span>`const fn from_inner(inner: sys::seL4_CNode_CapData) -> Self`

- <span id="cnodecapdata-into-inner"></span>`const fn into_inner(self) -> sys::seL4_CNode_CapData`

- <span id="cnodecapdata-inner"></span>`const fn inner(&self) -> &sys::seL4_CNode_CapData`

- <span id="cnodecapdata-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_CNode_CapData`

- <span id="cnodecapdata-new"></span>`fn new(guard: Word, guard_size: usize) -> Self` — [`Word`](#word)

- <span id="cnodecapdata-skip"></span>`fn skip(num_bits: usize) -> Self`

- <span id="cnodecapdata-skip-high-bits"></span>`fn skip_high_bits(cnode_size_bits: usize) -> Self`

- <span id="cnodecapdata-into-word"></span>`fn into_word(self) -> Word` — [`Word`](#word)

#### Trait Implementations

##### `impl Clone for CNodeCapData`

- <span id="cnodecapdata-clone"></span>`fn clone(&self) -> CNodeCapData` — [`CNodeCapData`](cnode_cap_data/index.md#cnodecapdata)

##### `impl Debug for CNodeCapData`

- <span id="cnodecapdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CNodeCapData`

##### `impl PartialEq for CNodeCapData`

- <span id="cnodecapdata-partialeq-eq"></span>`fn eq(&self, other: &CNodeCapData) -> bool` — [`CNodeCapData`](cnode_cap_data/index.md#cnodecapdata)

##### `impl StructuralPartialEq for CNodeCapData`

### `AbsoluteCPtr<C>`

```rust
struct AbsoluteCPtr<C> {
    root: CNode<C>,
    path: CPtrWithDepth,
}
```

A [`CPtrWithDepth`](cptr/index.md) to be resolved in the context of a particular [`CNode`](cptr/cap/index.md).

[`AbsoluteCPtr`](cptr/index.md) addresses capability slots in a more general way than [`Cap`](cptr/index.md). It allows one to
address any capability slot that is directly addressable from any CNode that is directly
addressible in the current thread's CSpace. Furthermore, it allows one to address capability
slots that contain CNodes by limiting the lookup depth to prevent the kernel's lookup procedure
from descending into the CNode contained in that slot.

`seL4_CNode_*` capability invocations are methods of [`AbsoluteCPtr`](cptr/index.md) rather than [`Cap`](cptr/index.md).

In addition to `AbsoluteCPtr::new`, the following methods can be used to construct an
[`AbsoluteCPtr`](cptr/index.md):
- `CNode::absolute_cptr`
- `CNode::absolute_cptr_from_bits_with_depth`
- `CNode::absolute_cptr_for_self`

#### Implementations

- <span id="absolutecptr-new"></span>`const fn new(root: CNode<C>, path: CPtrWithDepth) -> Self` — [`CNode`](cptr/cap/index.md#cnode), [`CPtrWithDepth`](cptr/index.md#cptrwithdepth)

- <span id="absolutecptr-root"></span>`const fn root(&self) -> &CNode<C>` — [`CNode`](cptr/cap/index.md#cnode)

- <span id="absolutecptr-into-root"></span>`fn into_root(self) -> CNode<C>` — [`CNode`](cptr/cap/index.md#cnode)

- <span id="absolutecptr-path"></span>`const fn path(&self) -> &CPtrWithDepth` — [`CPtrWithDepth`](cptr/index.md#cptrwithdepth)

- <span id="absolutecptr-with"></span>`fn with<C1>(self, context: C1) -> AbsoluteCPtr<C1>` — [`AbsoluteCPtr`](cptr/index.md#absolutecptr)

- <span id="absolutecptr-without-context"></span>`fn without_context(self) -> AbsoluteCPtr` — [`AbsoluteCPtr`](cptr/index.md#absolutecptr)

#### Trait Implementations

##### `impl<C: clone::Clone> Clone for AbsoluteCPtr<C>`

- <span id="absolutecptr-clone"></span>`fn clone(&self) -> AbsoluteCPtr<C>` — [`AbsoluteCPtr`](cptr/index.md#absolutecptr)

##### `impl<C: marker::Copy> Copy for AbsoluteCPtr<C>`

##### `impl<C: fmt::Debug> Debug for AbsoluteCPtr<C>`

- <span id="absolutecptr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<C: cmp::Eq> Eq for AbsoluteCPtr<C>`

##### `impl<C: cmp::PartialEq> PartialEq for AbsoluteCPtr<C>`

- <span id="absolutecptr-partialeq-eq"></span>`fn eq(&self, other: &AbsoluteCPtr<C>) -> bool` — [`AbsoluteCPtr`](cptr/index.md#absolutecptr)

##### `impl<C> StructuralPartialEq for AbsoluteCPtr<C>`

### `CPtr`

```rust
struct CPtr {
    bits: CPtrBits,
}
```

A capability pointer.

#### Implementations

- <span id="cptr-bits"></span>`const fn bits(self) -> CPtrBits` — [`CPtrBits`](cptr/index.md#cptrbits)

- <span id="cptr-from-bits"></span>`const fn from_bits(bits: CPtrBits) -> Self` — [`CPtrBits`](cptr/index.md#cptrbits)

- <span id="cptr-cast"></span>`const fn cast<T: CapType>(self) -> Cap<T>` — [`Cap`](cptr/index.md#cap)

#### Trait Implementations

##### `impl Clone for CPtr`

- <span id="cptr-clone"></span>`fn clone(&self) -> CPtr` — [`CPtr`](cptr/index.md#cptr)

##### `impl Copy for CPtr`

##### `impl Debug for CPtr`

- <span id="cptr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CPtr`

##### `impl HasCPtrWithDepth for CPtr`

- <span id="cptr-hascptrwithdepth-cptr-with-depth"></span>`fn cptr_with_depth(self) -> CPtrWithDepth` — [`CPtrWithDepth`](cptr/index.md#cptrwithdepth)

##### `impl Hash for CPtr`

- <span id="cptr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CPtr`

- <span id="cptr-ord-cmp"></span>`fn cmp(&self, other: &CPtr) -> cmp::Ordering` — [`CPtr`](cptr/index.md#cptr)

##### `impl PartialEq for CPtr`

- <span id="cptr-partialeq-eq"></span>`fn eq(&self, other: &CPtr) -> bool` — [`CPtr`](cptr/index.md#cptr)

##### `impl PartialOrd for CPtr`

- <span id="cptr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CPtr) -> option::Option<cmp::Ordering>` — [`CPtr`](cptr/index.md#cptr)

##### `impl StructuralPartialEq for CPtr`

### `CPtrWithDepth`

```rust
struct CPtrWithDepth {
    bits: CPtrBits,
    depth: usize,
}
```

A capability pointer with a number of bits to resolve.

#### Implementations

- <span id="cptrwithdepth-from-bits-with-depth"></span>`const fn from_bits_with_depth(bits: CPtrBits, depth: usize) -> Self` — [`CPtrBits`](cptr/index.md#cptrbits)

- <span id="cptrwithdepth-bits"></span>`const fn bits(&self) -> CPtrBits` — [`CPtrBits`](cptr/index.md#cptrbits)

- <span id="cptrwithdepth-depth"></span>`const fn depth(&self) -> usize`

- <span id="cptrwithdepth-empty"></span>`const fn empty() -> Self`

  The [`CPtrWithDepth`](cptr/index.md) with a depth of 0.

- <span id="cptrwithdepth-depth-for-kernel"></span>`fn depth_for_kernel(&self) -> u8`

#### Trait Implementations

##### `impl Clone for CPtrWithDepth`

- <span id="cptrwithdepth-clone"></span>`fn clone(&self) -> CPtrWithDepth` — [`CPtrWithDepth`](cptr/index.md#cptrwithdepth)

##### `impl Copy for CPtrWithDepth`

##### `impl Debug for CPtrWithDepth`

- <span id="cptrwithdepth-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CPtrWithDepth`

##### `impl HasCPtrWithDepth for CPtrWithDepth`

- <span id="cptrwithdepth-hascptrwithdepth-cptr-with-depth"></span>`fn cptr_with_depth(self) -> CPtrWithDepth` — [`CPtrWithDepth`](cptr/index.md#cptrwithdepth)

##### `impl Hash for CPtrWithDepth`

- <span id="cptrwithdepth-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CPtrWithDepth`

- <span id="cptrwithdepth-ord-cmp"></span>`fn cmp(&self, other: &CPtrWithDepth) -> cmp::Ordering` — [`CPtrWithDepth`](cptr/index.md#cptrwithdepth)

##### `impl PartialEq for CPtrWithDepth`

- <span id="cptrwithdepth-partialeq-eq"></span>`fn eq(&self, other: &CPtrWithDepth) -> bool` — [`CPtrWithDepth`](cptr/index.md#cptrwithdepth)

##### `impl PartialOrd for CPtrWithDepth`

- <span id="cptrwithdepth-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CPtrWithDepth) -> option::Option<cmp::Ordering>` — [`CPtrWithDepth`](cptr/index.md#cptrwithdepth)

##### `impl StructuralPartialEq for CPtrWithDepth`

### `Cap<T: CapType, C>`

```rust
struct Cap<T: CapType, C> {
    cptr: CPtr,
    invocation_context: C,
    _phantom: core::marker::PhantomData<T>,
}
```

A capability pointer to be resolved in the current CSpace.

- The `T` parameter is a [`CapType`](cptr/index.md) marking the type of the pointed-to capability.
- The `C` parameter is a strategy for discovering the current thread's IPC buffer. When the
  `"state"` feature is enabled, [`NoExplicitInvocationContext`](invocation_context/index.md) is an alias for
  [`ImplicitInvocationContext`](crate::ImplicitInvocationContext), which uses the [`IpcBuffer`](ipc_buffer/index.md)
  set by [`set_ipc_buffer`](crate::set_ipc_buffer). Otherwise, it is an alias for
  [`NoInvocationContext`](crate::NoInvocationContext), which does not implement
  [`InvocationContext`](invocation_context/index.md). In such cases, the [`with`](Cap::with) method is used to specify an
  invocation context before the capability is invoked.

The most general way to construct a [`Cap`](cptr/index.md) is with `CPtr::cast`.

Note that `seL4_CNode_*` capability invocations are methods of [`AbsoluteCPtr`](cptr/index.md) rather than
[`Cap`](cptr/index.md).

#### Implementations

- <span id="cratecap-frame-map"></span>`fn frame_map(self, vspace: VSpace, vaddr: usize, rights: CapRights, attrs: VmAttributes) -> Result<()>` — [`VSpace`](cptr/cap/index.md#vspace), [`CapRights`](cap_rights/index.md#caprights), [`VmAttributes`](arch/imp/vm_attributes/index.md#vmattributes), [`Result`](error/index.md#result)

  Corresponds to `seL4_X86_Page_Map`.

- <span id="cratecap-frame-unmap"></span>`fn frame_unmap(self) -> Result<()>` — [`Result`](error/index.md#result)

  Corresponds to `seL4_X86_Page_Unmap`.

- <span id="cratecap-frame-get-address"></span>`fn frame_get_address(self) -> Result<usize>` — [`Result`](error/index.md#result)

  Corresponds to `seL4_X86_Page_GetAddress`.

#### Trait Implementations

##### `impl<T: clone::Clone + CapType, C: clone::Clone> Clone for Cap<T, C>`

- <span id="cap-clone"></span>`fn clone(&self) -> Cap<T, C>` — [`Cap`](cptr/index.md#cap)

##### `impl<T: marker::Copy + CapType, C: marker::Copy> Copy for Cap<T, C>`

##### `impl<T: CapType, C> Debug for Cap<T, C>`

- <span id="cap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + CapType, C: cmp::Eq> Eq for Cap<T, C>`

##### `impl<T: CapType, C> HasCPtrWithDepth for Cap<T, C>`

- <span id="cap-hascptrwithdepth-cptr-with-depth"></span>`fn cptr_with_depth(self) -> CPtrWithDepth` — [`CPtrWithDepth`](cptr/index.md#cptrwithdepth)

##### `impl<T: hash::Hash + CapType, C: hash::Hash> Hash for Cap<T, C>`

- <span id="cap-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord + CapType, C: cmp::Ord> Ord for Cap<T, C>`

- <span id="cap-ord-cmp"></span>`fn cmp(&self, other: &Cap<T, C>) -> cmp::Ordering` — [`Cap`](cptr/index.md#cap)

##### `impl<T: cmp::PartialEq + CapType, C: cmp::PartialEq> PartialEq for Cap<T, C>`

- <span id="cap-partialeq-eq"></span>`fn eq(&self, other: &Cap<T, C>) -> bool` — [`Cap`](cptr/index.md#cap)

##### `impl<T: cmp::PartialOrd + CapType, C: cmp::PartialOrd> PartialOrd for Cap<T, C>`

- <span id="cap-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Cap<T, C>) -> option::Option<cmp::Ordering>` — [`Cap`](cptr/index.md#cap)

##### `impl<T: CapType, C> StructuralPartialEq for Cap<T, C>`

### `NoInvocationContext`

```rust
struct NoInvocationContext;
```

The absence of a strategy for discovering the current thread's IPC buffer.

#### Implementations

- <span id="noinvocationcontext-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl Clone for NoInvocationContext`

- <span id="noinvocationcontext-clone"></span>`fn clone(&self) -> NoInvocationContext` — [`NoInvocationContext`](invocation_context/index.md#noinvocationcontext)

##### `impl Copy for NoInvocationContext`

##### `impl Debug for NoInvocationContext`

- <span id="noinvocationcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NoInvocationContext`

- <span id="noinvocationcontext-default"></span>`fn default() -> NoInvocationContext` — [`NoInvocationContext`](invocation_context/index.md#noinvocationcontext)

##### `impl Eq for NoInvocationContext`

##### `impl Hash for NoInvocationContext`

- <span id="noinvocationcontext-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for NoInvocationContext`

- <span id="noinvocationcontext-partialeq-eq"></span>`fn eq(&self, other: &NoInvocationContext) -> bool` — [`NoInvocationContext`](invocation_context/index.md#noinvocationcontext)

##### `impl StructuralPartialEq for NoInvocationContext`

### `TcbFlagsBuilder`

```rust
struct TcbFlagsBuilder(crate::Word);
```

#### Implementations

- <span id="tcbflagsbuilder-new"></span>`fn new() -> Self`

- <span id="tcbflagsbuilder-build"></span>`fn build(self) -> Word` — [`Word`](#word)

- <span id="tcbflagsbuilder-fpu-disabled"></span>`fn fpu_disabled(self, val: bool) -> Self`

- <span id="tcbflagsbuilder-apply-flag-val"></span>`fn apply_flag_val(self, flag: Word, val: bool) -> Self` — [`Word`](#word)

#### Trait Implementations

##### `impl Clone for TcbFlagsBuilder`

- <span id="tcbflagsbuilder-clone"></span>`fn clone(&self) -> TcbFlagsBuilder` — [`TcbFlagsBuilder`](invocations/index.md#tcbflagsbuilder)

##### `impl Debug for TcbFlagsBuilder`

- <span id="tcbflagsbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TcbFlagsBuilder`

- <span id="tcbflagsbuilder-default"></span>`fn default() -> Self`

##### `impl PartialEq for TcbFlagsBuilder`

- <span id="tcbflagsbuilder-partialeq-eq"></span>`fn eq(&self, other: &TcbFlagsBuilder) -> bool` — [`TcbFlagsBuilder`](invocations/index.md#tcbflagsbuilder)

##### `impl StructuralPartialEq for TcbFlagsBuilder`

### `IpcBuffer`

```rust
struct IpcBuffer(sys::seL4_IPCBuffer);
```

Corresponds to `seL4_IPCBuffer`.

#### Implementations

- <span id="ipcbuffer-from-inner"></span>`const fn from_inner(inner: sys::seL4_IPCBuffer) -> Self`

- <span id="ipcbuffer-into-inner"></span>`const fn into_inner(self) -> sys::seL4_IPCBuffer`

- <span id="ipcbuffer-inner"></span>`const fn inner(&self) -> &sys::seL4_IPCBuffer`

- <span id="ipcbuffer-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_IPCBuffer`

- <span id="ipcbuffer-msg-regs"></span>`fn msg_regs(&self) -> &[Word]` — [`Word`](#word)

- <span id="ipcbuffer-msg-regs-mut"></span>`fn msg_regs_mut(&mut self) -> &mut [Word]` — [`Word`](#word)

- <span id="ipcbuffer-msg-bytes"></span>`fn msg_bytes(&self) -> &[u8]`

- <span id="ipcbuffer-msg-bytes-mut"></span>`fn msg_bytes_mut(&mut self) -> &mut [u8]`

- <span id="ipcbuffer-user-data"></span>`fn user_data(&self) -> Word` — [`Word`](#word)

- <span id="ipcbuffer-set-user-data"></span>`fn set_user_data(&mut self, data: Word)` — [`Word`](#word)

- <span id="ipcbuffer-caps-or-badges"></span>`fn caps_or_badges(&self) -> &[Word]` — [`Word`](#word)

- <span id="ipcbuffer-caps-or-badges-mut"></span>`fn caps_or_badges_mut(&mut self) -> &mut [Word]` — [`Word`](#word)

- <span id="ipcbuffer-recv-slot"></span>`fn recv_slot(&self) -> AbsoluteCPtr` — [`AbsoluteCPtr`](cptr/index.md#absolutecptr)

- <span id="ipcbuffer-set-recv-slot"></span>`fn set_recv_slot(&mut self, slot: &AbsoluteCPtr)` — [`AbsoluteCPtr`](cptr/index.md#absolutecptr)

#### Trait Implementations

##### `impl Default for IpcBuffer`

- <span id="ipcbuffer-default"></span>`fn default() -> IpcBuffer` — [`IpcBuffer`](ipc_buffer/index.md#ipcbuffer)

##### `impl InvocationContext for &mut crate::IpcBuffer`

- <span id="mut-crateipcbuffer-invocationcontext-with-context"></span>`fn with_context<T>(&mut self, f: impl FnOnce(&mut IpcBuffer) -> T) -> T` — [`IpcBuffer`](ipc_buffer/index.md#ipcbuffer)

### `MessageInfo`

```rust
struct MessageInfo(sys::seL4_MessageInfo);
```

Corresponds to `seL4_MessageInfo_t`.

#### Implementations

- <span id="messageinfo-from-inner"></span>`const fn from_inner(inner: sys::seL4_MessageInfo) -> Self`

- <span id="messageinfo-into-inner"></span>`const fn into_inner(self) -> sys::seL4_MessageInfo`

- <span id="messageinfo-inner"></span>`const fn inner(&self) -> &sys::seL4_MessageInfo`

- <span id="messageinfo-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_MessageInfo`

- <span id="messageinfo-new"></span>`fn new(label: Word, caps_unwrapped: usize, extra_caps: usize, length: usize) -> Self` — [`Word`](#word)

- <span id="messageinfo-label"></span>`fn label(&self) -> Word` — [`Word`](#word)

- <span id="messageinfo-label-width"></span>`const fn label_width() -> usize`

- <span id="messageinfo-caps-unwrapped"></span>`fn caps_unwrapped(&self) -> usize`

- <span id="messageinfo-extra-caps"></span>`fn extra_caps(&self) -> usize`

- <span id="messageinfo-length"></span>`fn length(&self) -> usize`

#### Trait Implementations

##### `impl Clone for MessageInfo`

- <span id="messageinfo-clone"></span>`fn clone(&self) -> MessageInfo` — [`MessageInfo`](message_info/index.md#messageinfo)

##### `impl Debug for MessageInfo`

- <span id="messageinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MessageInfo`

##### `impl PartialEq for MessageInfo`

- <span id="messageinfo-partialeq-eq"></span>`fn eq(&self, other: &MessageInfo) -> bool` — [`MessageInfo`](message_info/index.md#messageinfo)

##### `impl StructuralPartialEq for MessageInfo`

### `MessageInfoBuilder`

```rust
struct MessageInfoBuilder {
    label: crate::Word,
    caps_unwrapped: usize,
    extra_caps: usize,
    length: usize,
}
```

Helper for constructing [`MessageInfo`](message_info/index.md).

#### Implementations

- <span id="messageinfobuilder-build"></span>`fn build(self) -> MessageInfo` — [`MessageInfo`](message_info/index.md#messageinfo)

- <span id="messageinfobuilder-label"></span>`fn label(self, label: Word) -> Self` — [`Word`](#word)

- <span id="messageinfobuilder-caps-unwrapped"></span>`fn caps_unwrapped(self, caps_unwrapped: usize) -> Self`

- <span id="messageinfobuilder-extra-caps"></span>`fn extra_caps(self, extra_caps: usize) -> Self`

- <span id="messageinfobuilder-length"></span>`fn length(self, length: usize) -> Self`

#### Trait Implementations

##### `impl Clone for MessageInfoBuilder`

- <span id="messageinfobuilder-clone"></span>`fn clone(&self) -> MessageInfoBuilder` — [`MessageInfoBuilder`](message_info/index.md#messageinfobuilder)

##### `impl Copy for MessageInfoBuilder`

##### `impl Debug for MessageInfoBuilder`

- <span id="messageinfobuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MessageInfoBuilder`

- <span id="messageinfobuilder-default"></span>`fn default() -> MessageInfoBuilder` — [`MessageInfoBuilder`](message_info/index.md#messageinfobuilder)

##### `impl Eq for MessageInfoBuilder`

##### `impl PartialEq for MessageInfoBuilder`

- <span id="messageinfobuilder-partialeq-eq"></span>`fn eq(&self, other: &MessageInfoBuilder) -> bool` — [`MessageInfoBuilder`](message_info/index.md#messageinfobuilder)

##### `impl StructuralPartialEq for MessageInfoBuilder`

### `CallWithMRs`

```rust
struct CallWithMRs {
    pub info: crate::MessageInfo,
    pub msg: [crate::Word; 4],
}
```

The result of `cap::Endpoint::call_with_mrs`.

### `RecvWithMRs`

```rust
struct RecvWithMRs {
    pub info: crate::MessageInfo,
    pub badge: Badge,
    pub msg: [crate::Word; 4],
}
```

The result of `cap::Endpoint::recv_with_mrs`.

### `ImplicitReplyAuthority<C>`

```rust
struct ImplicitReplyAuthority<C> {
    invocation_context: C,
}
```

Under this configuration, no reply authority is required.

#### Implementations

- <span id="implicitreplyauthority-into-sys-reply-authority"></span>`fn into_sys_reply_authority(self) -> sys::ReplyAuthority`

#### Trait Implementations

##### `impl<C: default::Default> Default for ImplicitReplyAuthority<C>`

- <span id="implicitreplyauthority-default"></span>`fn default() -> ImplicitReplyAuthority<C>` — [`ImplicitReplyAuthority`](reply_authority/index.md#implicitreplyauthority)

### `DebugWrite`

```rust
struct DebugWrite;
```

Implements `core::fmt::Write` using [`debug_put_char`](printing/index.md).

#### Trait Implementations

##### `impl Write for DebugWrite`

- <span id="debugwrite-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

### `ImplicitInvocationContext`

```rust
struct ImplicitInvocationContext;
```

The strategy for discovering the current thread's IPC buffer which uses thread-local state.

This thread-local state can be modified using [`with_ipc_buffer`](state/index.md) and [`set_ipc_buffer`](state/index.md).

Requires the `"state"` feature to be enabled.

#### Implementations

- <span id="implicitinvocationcontext-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl Clone for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-clone"></span>`fn clone(&self) -> ImplicitInvocationContext` — [`ImplicitInvocationContext`](state/index.md#implicitinvocationcontext)

##### `impl Copy for ImplicitInvocationContext`

##### `impl Debug for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-default"></span>`fn default() -> ImplicitInvocationContext` — [`ImplicitInvocationContext`](state/index.md#implicitinvocationcontext)

##### `impl Eq for ImplicitInvocationContext`

##### `impl Hash for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl InvocationContext for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-invocationcontext-with-context"></span>`fn with_context<T>(&mut self, f: impl FnOnce(&mut IpcBuffer) -> T) -> T` — [`IpcBuffer`](ipc_buffer/index.md#ipcbuffer)

##### `impl PartialEq for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-partialeq-eq"></span>`fn eq(&self, other: &ImplicitInvocationContext) -> bool` — [`ImplicitInvocationContext`](state/index.md#implicitinvocationcontext)

##### `impl StructuralPartialEq for ImplicitInvocationContext`

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

- <span id="bootinfoextraid-clone"></span>`fn clone(&self) -> BootInfoExtraId` — [`BootInfoExtraId`](bootinfo/index.md#bootinfoextraid)

##### `impl Debug for BootInfoExtraId`

- <span id="bootinfoextraid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BootInfoExtraId`

##### `impl PartialEq for BootInfoExtraId`

- <span id="bootinfoextraid-partialeq-eq"></span>`fn eq(&self, other: &BootInfoExtraId) -> bool` — [`BootInfoExtraId`](bootinfo/index.md#bootinfoextraid)

##### `impl StructuralPartialEq for BootInfoExtraId`

### `Error`

```rust
enum Error {
    InvalidArgument,
    InvalidCapability,
    IllegalOperation,
    RangeError,
    AlignmentError,
    FailedLookup,
    TruncatedMessage,
    DeleteFirst,
    RevokeFirst,
    NotEnoughMemory,
}
```

Corresponds to `seL4_Error`.

#### Implementations

- <span id="error-into-sys"></span>`const fn into_sys(self) -> sys::seL4_Error::Type`

- <span id="error-from-sys"></span>`fn from_sys(err: sys::seL4_Error::Type) -> Option<Self>`

- <span id="error-wrap"></span>`fn wrap(err: sys::seL4_Error::Type) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="error-or"></span>`fn or<T>(err: sys::seL4_Error::Type, value: T) -> Result<T>` — [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](error/index.md#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](error/index.md#error)

##### `impl StructuralPartialEq for Error`

### `ObjectBlueprint`

```rust
enum ObjectBlueprint {
    Untyped {
        size_bits: usize,
    },
    Endpoint,
    Notification,
    CNode {
        size_bits: usize,
    },
    Tcb,
    Arch(crate::ObjectBlueprintArch),
}
```

An object description for [`Untyped::untyped_retype`](crate::cap::Untyped::untyped_retype).

#### Implementations

- <span id="objectblueprint-ty"></span>`const fn ty(self) -> ObjectType` — [`ObjectType`](object/index.md#objecttype)

- <span id="objectblueprint-api-size-bits"></span>`const fn api_size_bits(self) -> Option<usize>`

- <span id="objectblueprint-physical-size-bits"></span>`const fn physical_size_bits(self) -> usize`

- <span id="objectblueprint-asid-pool"></span>`const fn asid_pool() -> Self`

#### Trait Implementations

##### `impl Clone for ObjectBlueprint`

- <span id="objectblueprint-clone"></span>`fn clone(&self) -> ObjectBlueprint` — [`ObjectBlueprint`](object/index.md#objectblueprint)

##### `impl Copy for ObjectBlueprint`

##### `impl Debug for ObjectBlueprint`

- <span id="objectblueprint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectBlueprint`

##### `impl Ord for ObjectBlueprint`

- <span id="objectblueprint-ord-cmp"></span>`fn cmp(&self, other: &ObjectBlueprint) -> cmp::Ordering` — [`ObjectBlueprint`](object/index.md#objectblueprint)

##### `impl PartialEq for ObjectBlueprint`

- <span id="objectblueprint-partialeq-eq"></span>`fn eq(&self, other: &ObjectBlueprint) -> bool` — [`ObjectBlueprint`](object/index.md#objectblueprint)

##### `impl PartialOrd for ObjectBlueprint`

- <span id="objectblueprint-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ObjectBlueprint) -> option::Option<cmp::Ordering>` — [`ObjectBlueprint`](object/index.md#objectblueprint)

##### `impl StructuralPartialEq for ObjectBlueprint`

### `ObjectType`

```rust
enum ObjectType {
    Untyped,
    Endpoint,
    Notification,
    CNode,
    Tcb,
    Arch(crate::ObjectTypeArch),
}
```

Corresponds to `seL4_ObjectType`.

#### Implementations

- <span id="objecttype-into-sys"></span>`const fn into_sys(self) -> c_uint`

#### Trait Implementations

##### `impl Clone for ObjectType`

- <span id="objecttype-clone"></span>`fn clone(&self) -> ObjectType` — [`ObjectType`](object/index.md#objecttype)

##### `impl Debug for ObjectType`

- <span id="objecttype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectType`

##### `impl PartialEq for ObjectType`

- <span id="objecttype-partialeq-eq"></span>`fn eq(&self, other: &ObjectType) -> bool` — [`ObjectType`](object/index.md#objecttype)

##### `impl StructuralPartialEq for ObjectType`

## Traits

### `CapType`

```rust
trait CapType: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash { ... }
```

Trait for marker types corresponding to capability types in the seL4 API.

Implementors are used to mark instantiations of [`Cap`](cptr/index.md).

#### Associated Constants

- `const NAME: &'static str`

#### Implementors

- [`AsidControl`](cptr/cap_type/index.md#asidcontrol)
- [`AsidPool`](cptr/cap_type/index.md#asidpool)
- [`CNode`](cptr/cap_type/index.md#cnode)
- [`DomainSet`](cptr/cap_type/index.md#domainset)
- [`Endpoint`](cptr/cap_type/index.md#endpoint)
- [`HugePage`](cptr/cap_type/index.md#hugepage)
- [`IOPortControl`](cptr/cap_type/index.md#ioportcontrol)
- [`IrqControl`](cptr/cap_type/index.md#irqcontrol)
- [`IrqHandler`](cptr/cap_type/index.md#irqhandler)
- [`LargePage`](cptr/cap_type/index.md#largepage)
- [`Notification`](cptr/cap_type/index.md#notification)
- [`Null`](cptr/cap_type/index.md#null)
- [`PDPT`](cptr/cap_type/index.md#pdpt)
- [`PML4`](cptr/cap_type/index.md#pml4)
- [`PageDirectory`](cptr/cap_type/index.md#pagedirectory)
- [`PageTable`](cptr/cap_type/index.md#pagetable)
- [`Tcb`](cptr/cap_type/index.md#tcb)
- [`UnspecifiedIntermediateTranslationTable`](cptr/cap_type/index.md#unspecifiedintermediatetranslationtable)
- [`UnspecifiedPage`](cptr/cap_type/index.md#unspecifiedpage)
- [`Unspecified`](cptr/cap_type/index.md#unspecified)
- [`Untyped`](cptr/cap_type/index.md#untyped)
- [`_4k`](cptr/cap_type/index.md#4k)

### `HasCPtrWithDepth`

```rust
trait HasCPtrWithDepth { ... }
```

Trait for types whose members which logically contain a [`CPtrWithDepth`](cptr/index.md).

[`CPtr`](cptr/index.md) and [`Cap`](cptr/index.md) each logically contain a [`CPtrWithDepth`](cptr/index.md) with a depth of [`WORD_SIZE`](#word-size).

#### Required Methods

- `fn cptr_with_depth(self) -> CPtrWithDepth`

  Returns the logical [`CPtrWithDepth`](cptr/index.md) entailed by `self`.

#### Implementors

- [`CPtrWithDepth`](cptr/index.md#cptrwithdepth)
- [`CPtr`](cptr/index.md#cptr)
- [`Cap`](cptr/index.md#cap)

### `InvocationContext`

```rust
trait InvocationContext { ... }
```

A strategy for discovering the current thread's IPC buffer.

#### Required Methods

- `fn with_context<T>(&mut self, f: impl FnOnce(&mut IpcBuffer) -> T) -> T`

#### Implementors

- [`ImplicitInvocationContext`](state/index.md#implicitinvocationcontext)
- `&core::cell::RefCell<U>`
- `&mut U`
- `&mut crate::IpcBuffer`

### `CapTypeForObject`

```rust
trait CapTypeForObject: CapType { ... }
```

Trait for [`CapType`](cptr/index.md)s which correspond to kernel objects.

#### Required Methods

- `fn object_type() -> ObjectType`

#### Implementors

- [`CNode`](cptr/cap_type/index.md#cnode)
- [`Endpoint`](cptr/cap_type/index.md#endpoint)
- [`HugePage`](cptr/cap_type/index.md#hugepage)
- [`LargePage`](cptr/cap_type/index.md#largepage)
- [`Notification`](cptr/cap_type/index.md#notification)
- [`PDPT`](cptr/cap_type/index.md#pdpt)
- [`PML4`](cptr/cap_type/index.md#pml4)
- [`PageDirectory`](cptr/cap_type/index.md#pagedirectory)
- [`PageTable`](cptr/cap_type/index.md#pagetable)
- [`Tcb`](cptr/cap_type/index.md#tcb)
- [`Untyped`](cptr/cap_type/index.md#untyped)
- [`_4k`](cptr/cap_type/index.md#4k)

### `CapTypeForObjectOfFixedSize`

```rust
trait CapTypeForObjectOfFixedSize: CapTypeForObject { ... }
```

Trait for [`CapType`](cptr/index.md)s which correspond to kernel objects of fixed size.

#### Required Methods

- `fn object_blueprint() -> ObjectBlueprint`

#### Implementors

- [`Endpoint`](cptr/cap_type/index.md#endpoint)
- [`HugePage`](cptr/cap_type/index.md#hugepage)
- [`LargePage`](cptr/cap_type/index.md#largepage)
- [`Notification`](cptr/cap_type/index.md#notification)
- [`PDPT`](cptr/cap_type/index.md#pdpt)
- [`PML4`](cptr/cap_type/index.md#pml4)
- [`PageDirectory`](cptr/cap_type/index.md#pagedirectory)
- [`PageTable`](cptr/cap_type/index.md#pagetable)
- [`Tcb`](cptr/cap_type/index.md#tcb)
- [`_4k`](cptr/cap_type/index.md#4k)

### `CapTypeForObjectOfVariableSize`

```rust
trait CapTypeForObjectOfVariableSize: CapTypeForObject { ... }
```

Trait for [`CapType`](cptr/index.md)s which correspond to kernel objects of variable size.

#### Required Methods

- `fn object_blueprint(size_bits: usize) -> ObjectBlueprint`

#### Implementors

- [`CNode`](cptr/cap_type/index.md#cnode)
- [`Untyped`](cptr/cap_type/index.md#untyped)

### `ConveysReplyAuthority`

```rust
trait ConveysReplyAuthority { ... }
```

Trait for types from which [`ReplyAuthority`](reply_authority/index.md) can be derived.

#### Associated Types

- `type C`

#### Required Methods

- `fn into_reply_authority(self) -> ReplyAuthority<<Self as >::C>`

#### Implementors

- [`ReplyAuthority`](reply_authority/index.md#replyauthority)
- `()`

### `FastMessages`

```rust
trait FastMessages: fast_messages_sealing::FastMessagesSealed { ... }
```

Trait for types which can hold the contents of a set of inline message registers.

#### Required Methods

- `fn prepare_in(self) -> [Option<crate::Word>; 4]`

- `fn prepare_in_out(self) -> [crate::Word; 4]`

#### Implementors

- `&[crate::Word]`
- `[crate::Word; N]`

### `IpcCapType`

```rust
trait IpcCapType: CapType { ... }
```

Trait for [`CapType`](cptr/index.md)s which are used as targets of IPC syscalls.

#### Implementors

- [`Endpoint`](cptr/cap_type/index.md#endpoint)
- [`Notification`](cptr/cap_type/index.md#notification)
- [`Unspecified`](cptr/cap_type/index.md#unspecified)

### `CapTypeForFrameObject`

```rust
trait CapTypeForFrameObject: CapType { ... }
```

Trait for [`CapType`](cptr/index.md)s which correspond to frame objects.

#### Implementors

- [`HugePage`](cptr/cap_type/index.md#hugepage)
- [`LargePage`](cptr/cap_type/index.md#largepage)
- [`UnspecifiedPage`](cptr/cap_type/index.md#unspecifiedpage)
- [`_4k`](cptr/cap_type/index.md#4k)

### `CapTypeForFrameObjectOfFixedSize`

```rust
trait CapTypeForFrameObjectOfFixedSize: CapTypeForObjectOfFixedSize + CapTypeForFrameObject { ... }
```

Trait for [`CapType`](cptr/index.md)s which correspond to frame objects of fixed size.

#### Associated Constants

- `const FRAME_OBJECT_TYPE: FrameObjectType`

#### Implementors

- [`HugePage`](cptr/cap_type/index.md#hugepage)
- [`LargePage`](cptr/cap_type/index.md#largepage)
- [`_4k`](cptr/cap_type/index.md#4k)

### `CapTypeForTranslationTableObject`

```rust
trait CapTypeForTranslationTableObject: CapTypeForObjectOfFixedSize { ... }
```

Trait for [`CapType`](cptr/index.md)s which correspond to translation table objects.

#### Associated Constants

- `const TRANSLATION_TABLE_OBJECT_TYPE: TranslationTableObjectType`

#### Implementors

- [`PDPT`](cptr/cap_type/index.md#pdpt)
- [`PML4`](cptr/cap_type/index.md#pml4)
- [`PageDirectory`](cptr/cap_type/index.md#pagedirectory)
- [`PageTable`](cptr/cap_type/index.md#pagetable)

## Functions

### `sys`

```rust
fn sys(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

### `yield`

```rust
fn yield()
```

Corresponds to `seL4_Yield`.

### `reply`

```rust
fn reply(ipc_buffer: &mut crate::IpcBuffer, info: crate::MessageInfo)
```

Corresponds to `seL4_Reply`.

### `debug_put_char`

```rust
fn debug_put_char(c: u8)
```

Corresponds to `seL4_DebugPutChar`.

### `debug_snapshot`

```rust
fn debug_snapshot()
```

Corresponds to `seL4_DebugSnapshot`.

### `debug_halt`

```rust
fn debug_halt()
```

Corresponds to `seL4_DebugHalt`.

### `ipc_buffer_is_thread_local`

```rust
const fn ipc_buffer_is_thread_local() -> bool
```

Returns whether this crate's IPC buffer slot is thread-local.

Requires the `"state"` feature to be enabled.

### `set_ipc_buffer`

```rust
fn set_ipc_buffer(ipc_buffer: &'static mut crate::IpcBuffer)
```

Sets the IPC buffer that this crate will use for this thread.

This function does not modify kernel state. It only affects this crate's thread-local state.

Requires the `"state"` feature to be enabled.

### `try_with_ipc_buffer_slot`

```rust
fn try_with_ipc_buffer_slot<F, T>(f: F) -> T
where
    F: FnOnce(Result<&Option<&'static mut crate::IpcBuffer>, token::BorrowError>) -> T
```

Provides low-level access to this thread's IPC buffer.

This function does not modify kernel state. It only affects this crate's thread-local state.

Requires the `"state"` feature to be enabled.

### `try_with_ipc_buffer_slot_mut`

```rust
fn try_with_ipc_buffer_slot_mut<F, T>(f: F) -> T
where
    F: FnOnce(Result<&mut Option<&'static mut crate::IpcBuffer>, token::BorrowMutError>) -> T
```

Provides low-level mutable access to this thread's IPC buffer.

This function does not modify kernel state. It only affects this crate's thread-local state.

Requires the `"state"` feature to be enabled.

### `with_ipc_buffer`

```rust
fn with_ipc_buffer<F, T>(f: F) -> T
where
    F: FnOnce(&crate::IpcBuffer) -> T
```

Provides access to this thread's IPC buffer.

Requires the `"state"` feature to be enabled.

### `with_ipc_buffer_mut`

```rust
fn with_ipc_buffer_mut<F, T>(f: F) -> T
where
    F: FnOnce(&mut crate::IpcBuffer) -> T
```

Provides mutable access to this thread's IPC buffer.

Requires the `"state"` feature to be enabled.

## Type Aliases

### `CPtrBits`

```rust
type CPtrBits = sys::seL4_CPtr;
```

The raw bits of a capability pointer.

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

Alias for `Result<_, Error>`.

### `NoExplicitInvocationContext`

```rust
type NoExplicitInvocationContext = crate::ImplicitInvocationContext;
```

The default strategy for discovering the current thread's IPC buffer.

When the `"state"` feature is enabled, [`NoExplicitInvocationContext`](invocation_context/index.md) is an alias for
[`ImplicitInvocationContext`](crate::ImplicitInvocationContext), which uses the [`IpcBuffer`](ipc_buffer/index.md)
set by [`set_ipc_buffer`](crate::set_ipc_buffer). Otherwise, it is an alias for
[`NoInvocationContext`](crate::NoInvocationContext), which does not implement
[`InvocationContext`](invocation_context/index.md).

### `ReplyAuthority<C>`

```rust
type ReplyAuthority<C> = ImplicitReplyAuthority<C>;
```

Configuration-dependant alias for conveying reply authority to syscalls.

### `Badge`

```rust
type Badge = crate::Word;
```

A capability badge.

### `Word`

```rust
type Word = sys::seL4_Word;
```

Corresponds to `seL4_Word`.

## Constants

### `NUM_MESSAGE_REGISTERS`
```rust
const NUM_MESSAGE_REGISTERS: usize = 120usize;
```

Number of message registers in the IPC buffer.

### `WORD_SIZE`
```rust
const WORD_SIZE: usize = 64usize;
```

The size of [`Word`](#word) in bits.

## Macros

### `debug_print!`

Like `std::debug_print!`, except backed by `seL4_DebugPutChar`.

### `debug_println!`

Like `std::debug_println!`, except backed by `seL4_DebugPutChar`.

