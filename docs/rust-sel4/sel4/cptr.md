**sel4 > cptr**

# Module: cptr

## Contents

**Modules**

- [`cap`](#cap) - Marked aliases of [`Cap`](crate::Cap).
- [`cap_type`](#cap_type) - Markers corresponding to capability types and classes of capability types.

**Structs**

- [`AbsoluteCPtr`](#absolutecptr) - A [`CPtrWithDepth`] to be resolved in the context of a particular [`CNode`].
- [`CPtr`](#cptr) - A capability pointer.
- [`CPtrWithDepth`](#cptrwithdepth) - A capability pointer with a number of bits to resolve.
- [`Cap`](#cap) - A capability pointer to be resolved in the current CSpace.

**Traits**

- [`CapType`](#captype) - Trait for marker types corresponding to capability types in the seL4 API.
- [`HasCPtrWithDepth`](#hascptrwithdepth) - Trait for types whose members which logically contain a [`CPtrWithDepth`].

**Type Aliases**

- [`CPtrBits`](#cptrbits) - The raw bits of a capability pointer.

---

## sel4::cptr::AbsoluteCPtr

*Struct*

A [`CPtrWithDepth`] to be resolved in the context of a particular [`CNode`].

[`AbsoluteCPtr`] addresses capability slots in a more general way than [`Cap`]. It allows one to
address any capability slot that is directly addressable from any CNode that is directly
addressible in the current thread's CSpace. Furthermore, it allows one to address capability
slots that contain CNodes by limiting the lookup depth to prevent the kernel's lookup procedure
from descending into the CNode contained in that slot.

`seL4_CNode_*` capability invocations are methods of [`AbsoluteCPtr`] rather than [`Cap`].

In addition to [`AbsoluteCPtr::new`], the following methods can be used to construct an
[`AbsoluteCPtr`]:
- [`CNode::absolute_cptr`]
- [`CNode::absolute_cptr_from_bits_with_depth`]
- [`CNode::absolute_cptr_for_self`]

**Generic Parameters:**
- C

**Methods:**

- `fn revoke(self: Self) -> Result<()>` - Corresponds to `seL4_CNode_Revoke`.
- `fn delete(self: Self) -> Result<()>` - Corresponds to `seL4_CNode_Delete`.
- `fn copy(self: Self, src: &AbsoluteCPtr, rights: CapRights) -> Result<()>` - Corresponds to `seL4_CNode_Copy`.
- `fn mint(self: Self, src: &AbsoluteCPtr, rights: CapRights, badge: Word) -> Result<()>` - Corresponds to `seL4_CNode_Mint`.
- `fn move_(self: Self, src: &AbsoluteCPtr) -> Result<()>` - Corresponds to `seL4_CNode_Move`.
- `fn mutate(self: Self, src: &AbsoluteCPtr, badge: Word) -> Result<()>` - Corresponds to `seL4_CNode_Mutate`.
- `fn save_caller(self: Self) -> Result<()>` - Corresponds to `seL4_CNode_SaveCaller`.
- `fn new(root: CNode<C>, path: CPtrWithDepth) -> Self`
- `fn root(self: &Self) -> &CNode<C>`
- `fn into_root(self: Self) -> CNode<C>`
- `fn path(self: &Self) -> &CPtrWithDepth`
- `fn with<C1>(self: Self, context: C1) -> AbsoluteCPtr<C1>`
- `fn without_context(self: Self) -> AbsoluteCPtr`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AbsoluteCPtr<C>`
- **PartialEq**
  - `fn eq(self: &Self, other: &AbsoluteCPtr<C>) -> bool`



## sel4::cptr::CPtr

*Struct*

A capability pointer.

**Methods:**

- `fn bits(self: Self) -> CPtrBits`
- `fn from_bits(bits: CPtrBits) -> Self`
- `fn cast<T>(self: Self) -> Cap<T>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &CPtr) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> CPtr`
- **PartialEq**
  - `fn eq(self: &Self, other: &CPtr) -> bool`
- **HasCPtrWithDepth**
  - `fn cptr_with_depth(self: Self) -> CPtrWithDepth`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &CPtr) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::cptr::CPtrBits

*Type Alias*: `sys::seL4_CPtr`

The raw bits of a capability pointer.



## sel4::cptr::CPtrWithDepth

*Struct*

A capability pointer with a number of bits to resolve.

**Methods:**

- `fn from_bits_with_depth(bits: CPtrBits, depth: usize) -> Self`
- `fn bits(self: &Self) -> CPtrBits`
- `fn depth(self: &Self) -> usize`
- `fn empty() -> Self` - The [`CPtrWithDepth`] with a depth of 0.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &CPtrWithDepth) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> CPtrWithDepth`
- **HasCPtrWithDepth**
  - `fn cptr_with_depth(self: Self) -> CPtrWithDepth`
- **PartialEq**
  - `fn eq(self: &Self, other: &CPtrWithDepth) -> bool`
- **From**
  - `fn from(cptr: CPtr) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &CPtrWithDepth) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::cptr::Cap

*Struct*

A capability pointer to be resolved in the current CSpace.

- The `T` parameter is a [`CapType`] marking the type of the pointed-to capability.
- The `C` parameter is a strategy for discovering the current thread's IPC buffer. When the
  `"state"` feature is enabled, [`NoExplicitInvocationContext`] is an alias for
  [`ImplicitInvocationContext`](crate::ImplicitInvocationContext), which uses the [`IpcBuffer`]
  set by [`set_ipc_buffer`](crate::set_ipc_buffer). Otherwise, it is an alias for
  [`NoInvocationContext`](crate::NoInvocationContext), which does not implement
  [`InvocationContext`]. In such cases, the [`with`](Cap::with) method is used to specify an
  invocation context before the capability is invoked.

The most general way to construct a [`Cap`] is with [`CPtr::cast`].

Note that `seL4_CNode_*` capability invocations are methods of [`AbsoluteCPtr`] rather than
[`Cap`].

**Generic Parameters:**
- T
- C

**Methods:**

- `fn cptr(self: &Self) -> CPtr`
- `fn bits(self: &Self) -> CPtrBits`
- `fn cast<T1>(self: Self) -> Cap<T1, C>`
- `fn with<C1>(self: Self, context: C1) -> Cap<T, C1>`
- `fn without_context(self: Self) -> Cap<T>`
- `fn into_invocation_context(self: Self) -> C`
- `fn asid_control_make_pool(self: Self, untyped: Untyped, dst: &AbsoluteCPtr) -> Result<()>` - Corresponds to `seL4_X86_ASIDControl_MakePool`.
- `fn absolute_cptr<T>(self: Self, path: T) -> AbsoluteCPtr<C>` - Returns the [`AbsoluteCPtr`] for `path` in the context of `self`.
- `fn absolute_cptr_from_bits_with_depth(self: Self, bits: CPtrBits, depth: usize) -> AbsoluteCPtr<C>` - Returns the [`AbsoluteCPtr`] for
- `fn absolute_cptr_for_self(self: Self) -> AbsoluteCPtr<C>` - Returns the [`AbsoluteCPtr`] for `self` in its own context.
- `fn signal(self: Self)` - Corresponds to `seL4_Signal`.
- `fn wait(self: Self) -> ((), Badge)` - Corresponds to `seL4_Wait`.
- `fn from_cptr(cptr: CPtr) -> Self`
- `fn from_bits(bits: CPtrBits) -> Self`
- `fn pdpt_map(self: Self, vspace: VSpace, vaddr: usize, attr: VmAttributes) -> Result<()>`
- `fn ioport_control_issue(self: Self, first_port: Word, last_port: Word, dst: &AbsoluteCPtr) -> Result<()>` - Corresponds to `seL4_X86_IOPortControl_Issue`.
- `fn domain_set_set(self: Self, domain: u8, thread: Tcb) -> Result<()>` - Corresponds to `seL4_DomainSet_Set`.
- `fn domain_set_schedule_configure(self: Self, index: Word, domain: u8, duration: u64) -> Result<()>` - Corresponds to `seL4_DomainSet_ScheduleConfigure`
- `fn domain_set_schedule_set_start(self: Self, index: Word) -> Result<()>` - Corresponds to `seL4_DomainSet_ScheduleSetStart`
- `fn generic_intermediate_translation_table_map(self: Self, ty: TranslationTableObjectType, vspace: VSpace, vaddr: usize, attr: VmAttributes) -> Result<()>`
- `fn irq_control_get(self: Self, irq: Word, dst: &AbsoluteCPtr) -> Result<()>` - Corresponds to `seL4_IRQControl_Get`.
- `fn frame_map(self: Self, vspace: VSpace, vaddr: usize, rights: CapRights, attrs: VmAttributes) -> Result<()>` - Corresponds to `seL4_X86_Page_Map`.
- `fn frame_unmap(self: Self) -> Result<()>` - Corresponds to `seL4_X86_Page_Unmap`.
- `fn frame_get_address(self: Self) -> Result<usize>` - Corresponds to `seL4_X86_Page_GetAddress`.
- `fn irq_control_get_ioapic(self: Self, ioapic: Word, pin: Word, level: Word, polarity: Word, vector: Word, dst: &AbsoluteCPtr) -> Result<()>` - Corresponds to `seL4_IRQControl_GetIOAPIC`.
- `fn irq_control_get_msi(self: Self, pci_bus: Word, pci_dev: Word, pci_func: Word, handle: Word, vector: Word, dst: &AbsoluteCPtr) -> Result<()>` - Corresponds to `seL4_IRQControl_GetMSI`.
- `fn debug_identify(self: Self) -> u32` - Corresponds to `seL4_DebugCapIdentify`.
- `fn downcast<T>(self: Self) -> Cap<T, C>`
- `fn page_table_map(self: Self, vspace: VSpace, vaddr: usize, attr: VmAttributes) -> Result<()>`
- `fn tcb_read_registers(self: Self, suspend: bool, count: Word) -> Result<UserContext>` - Corresponds to `seL4_TCB_ReadRegisters`.
- `fn tcb_read_all_registers(self: Self, suspend: bool) -> Result<UserContext>`
- `fn tcb_write_registers(self: Self, resume: bool, count: Word, regs: & mut UserContext) -> Result<()>` - Corresponds to `seL4_TCB_WriteRegisters`.
- `fn tcb_write_all_registers(self: Self, resume: bool, regs: & mut UserContext) -> Result<()>`
- `fn tcb_resume(self: Self) -> Result<()>` - Corresponds to `seL4_TCB_Resume`.
- `fn tcb_suspend(self: Self) -> Result<()>` - Corresponds to `seL4_TCB_Suspend`.
- `fn tcb_set_priority(self: Self, authority: Tcb, priority: Word) -> Result<()>` - Corresponds to `seL4_TCB_SetPriority`.
- `fn tcb_set_mc_priority(self: Self, authority: Tcb, mcp: Word) -> Result<()>` - Corresponds to `seL4_TCB_SetMCPriority`.
- `fn tcb_set_flags(self: Self, clear: Word, set: Word) -> Result<Word>`
- `fn tcb_configure(self: Self, fault_ep: CPtr, cspace_root: CNode, cspace_root_data: CNodeCapData, vspace_root: VSpace, ipc_buffer: Word, ipc_buffer_frame: Granule) -> Result<()>` - Corresponds to `seL4_TCB_Configure`.
- `fn tcb_set_space(self: Self, fault_ep: CPtr, cspace_root: CNode, cspace_root_data: CNodeCapData, vspace_root: VSpace) -> Result<()>` - Corresponds to `seL4_TCB_SetSpace`.
- `fn tcb_set_sched_params(self: Self, authority: Tcb, mcp: Word, priority: Word) -> Result<()>` - Corresponds to `seL4_TCB_SetSchedParams`.
- `fn tcb_set_tls_base(self: Self, tls_base: Word) -> Result<()>` - Corresponds to `seL4_TCB_SetTLSBase`.
- `fn tcb_bind_notification(self: Self, notification: Notification) -> Result<()>` - Corresponds to `seL4_TCB_BindNotification`.
- `fn tcb_unbind_notification(self: Self) -> Result<()>` - Corresponds to `seL4_TCB_UnbindNotification`.
- `fn asid_pool_assign(self: Self, vspace: VSpace) -> Result<()>` - Corresponds to `seL4_X86_ASIDPool_Assign`.
- `fn untyped_retype(self: Self, blueprint: &ObjectBlueprint, dst: &AbsoluteCPtr, dst_offset: usize, num_objects: usize) -> Result<()>` - Corresponds to `seL4_Untyped_Retype`.
- `fn debug_name(self: Self, name: &[u8])` - Corresponds to `seL4_DebugNameThread`.
- `fn upcast(self: Self) -> Unspecified<C>`
- `fn irq_handler_ack(self: Self) -> Result<()>` - Corresponds to `seL4_IRQHandler_Ack`.
- `fn irq_handler_set_notification(self: Self, notification: Notification) -> Result<()>` - Corresponds to `seL4_IRQHandler_SetNotification`.
- `fn irq_handler_clear(self: Self) -> Result<()>` - Corresponds to `seL4_IRQHandler_Clear`.
- `fn page_directory_map(self: Self, vspace: VSpace, vaddr: usize, attr: VmAttributes) -> Result<()>`
- `fn send(self: Self, info: MessageInfo)` - Corresponds to `seL4_Send`.
- `fn nb_send(self: Self, info: MessageInfo)` - Corresponds to `seL4_NBSend`.
- `fn recv<impl ConveysReplyAuthority>(self: Self, reply_authority: impl Trait) -> (MessageInfo, Badge)` - Corresponds to `seL4_Recv`.
- `fn nb_recv<impl ConveysReplyAuthority>(self: Self, reply_authority: impl Trait) -> (MessageInfo, Badge)` - Corresponds to `seL4_NBRecv`.
- `fn call(self: Self, info: MessageInfo) -> MessageInfo` - Corresponds to `seL4_Call`.
- `fn reply_recv<impl ConveysReplyAuthority>(self: Self, info: MessageInfo, reply_authority: impl Trait) -> (MessageInfo, Badge)` - Corresponds to `seL4_ReplyRecv`.
- `fn send_with_mrs<T>(self: Self, info: MessageInfo, messages: T)`
- `fn recv_with_mrs<impl ConveysReplyAuthority>(self: Self, reply_authority: impl Trait) -> RecvWithMRs`
- `fn call_with_mrs<T>(self: Self, info: MessageInfo, messages: T) -> CallWithMRs`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Cap<T, C>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Cap<T, C>`
- **HasCPtrWithDepth**
  - `fn cptr_with_depth(self: Self) -> CPtrWithDepth`
- **Ord**
  - `fn cmp(self: &Self, other: &Cap<T, C>) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Cap<T, C>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## sel4::cptr::CapType

*Trait*

Trait for marker types corresponding to capability types in the seL4 API.

Implementors are used to mark instantiations of [`Cap`].

**Methods:**

- `NAME`



## sel4::cptr::HasCPtrWithDepth

*Trait*

Trait for types whose members which logically contain a [`CPtrWithDepth`].

[`CPtr`] and [`Cap`] each logically contain a [`CPtrWithDepth`] with a depth of [`WORD_SIZE`].

**Methods:**

- `cptr_with_depth`: Returns the logical [`CPtrWithDepth`] entailed by `self`.



## Module: cap

Marked aliases of [`Cap`](crate::Cap).

Each type `$t<C = NoExplicitInvocationContext>` in this module is an alias for `Cap<$t, C>`.



## Module: cap_type

Markers corresponding to capability types and classes of capability types.

These types are used for marking [`Cap`](crate::Cap).



