**sel4_sys > c**

# Module: c

## Contents

**Modules**

- [`_mode_object`](#_mode_object)
- [`_object`](#_object)
- [`api_object`](#api_object)
- [`priorityConstants`](#priorityconstants)
- [`seL4_BootInfoID`](#sel4_bootinfoid)
- [`seL4_CapFault_Msg`](#sel4_capfault_msg)
- [`seL4_Error`](#sel4_error)
- [`seL4_LookupFailureType`](#sel4_lookupfailuretype)
- [`seL4_MsgLimits`](#sel4_msglimits)
- [`seL4_RootCNodeCapSlots`](#sel4_rootcnodecapslots)
- [`seL4_TCBFlag`](#sel4_tcbflag)
- [`seL4_UnknownSyscall_Msg`](#sel4_unknownsyscall_msg)
- [`seL4_UserException_Msg`](#sel4_userexception_msg)
- [`seL4_VMFault_Msg`](#sel4_vmfault_msg)
- [`seL4_X86_EPT_VMAttributes`](#sel4_x86_ept_vmattributes)
- [`seL4_X86_VMAttributes`](#sel4_x86_vmattributes)

**Structs**

- [`seL4_BootInfo`](#sel4_bootinfo)
- [`seL4_BootInfoHeader`](#sel4_bootinfoheader)
- [`seL4_IPCBuffer_`](#sel4_ipcbuffer_)
- [`seL4_SchedContext_Consumed`](#sel4_schedcontext_consumed)
- [`seL4_SchedContext_YieldTo`](#sel4_schedcontext_yieldto)
- [`seL4_SlotRegion`](#sel4_slotregion)
- [`seL4_TCB_ConfigureSingleStepping`](#sel4_tcb_configuresinglestepping)
- [`seL4_TCB_GetBreakpoint`](#sel4_tcb_getbreakpoint)
- [`seL4_UntypedDesc`](#sel4_untypeddesc)
- [`seL4_UserContext_`](#sel4_usercontext_)
- [`seL4_VCPUContext_`](#sel4_vcpucontext_)
- [`seL4_X86_PageDirectory_GetStatusBits`](#sel4_x86_pagedirectory_getstatusbits)
- [`seL4_X86_VCPU_ReadMSR`](#sel4_x86_vcpu_readmsr)
- [`seL4_X86_VCPU_ReadVMCS`](#sel4_x86_vcpu_readvmcs)
- [`seL4_X86_VCPU_WriteMSR`](#sel4_x86_vcpu_writemsr)
- [`seL4_X86_VCPU_WriteVMCS`](#sel4_x86_vcpu_writevmcs)

**Constants**

- [`IPCBUF_GDT_ENTRY`](#ipcbuf_gdt_entry)
- [`IPCBUF_GDT_SELECTOR`](#ipcbuf_gdt_selector)
- [`IRQ_OFFSET`](#irq_offset)
- [`MSI_MAX`](#msi_max)
- [`MSI_MIN`](#msi_min)
- [`SEL4_MAPPING_LOOKUP_LEVEL`](#sel4_mapping_lookup_level)
- [`SEL4_MAPPING_LOOKUP_NO_EPTPD`](#sel4_mapping_lookup_no_eptpd)
- [`SEL4_MAPPING_LOOKUP_NO_EPTPDPT`](#sel4_mapping_lookup_no_eptpdpt)
- [`SEL4_MAPPING_LOOKUP_NO_EPTPT`](#sel4_mapping_lookup_no_eptpt)
- [`SEL4_MAPPING_LOOKUP_NO_PD`](#sel4_mapping_lookup_no_pd)
- [`SEL4_MAPPING_LOOKUP_NO_PDPT`](#sel4_mapping_lookup_no_pdpt)
- [`SEL4_MAPPING_LOOKUP_NO_PT`](#sel4_mapping_lookup_no_pt)
- [`TLS_GDT_ENTRY`](#tls_gdt_entry)
- [`TLS_GDT_SELECTOR`](#tls_gdt_selector)
- [`VECTOR_MAX`](#vector_max)
- [`VECTOR_MIN`](#vector_min)
- [`seL4_ASIDPoolBits`](#sel4_asidpoolbits)
- [`seL4_ASIDPoolIndexBits`](#sel4_asidpoolindexbits)
- [`seL4_BadgeBits`](#sel4_badgebits)
- [`seL4_BootInfoFrameBits`](#sel4_bootinfoframebits)
- [`seL4_CapRightsBits`](#sel4_caprightsbits)
- [`seL4_DataFault`](#sel4_datafault)
- [`seL4_EndpointBits`](#sel4_endpointbits)
- [`seL4_False`](#sel4_false)
- [`seL4_FastMessageRegisters`](#sel4_fastmessageregisters)
- [`seL4_GuardBits`](#sel4_guardbits)
- [`seL4_GuardSizeBits`](#sel4_guardsizebits)
- [`seL4_HugePageBits`](#sel4_hugepagebits)
- [`seL4_IOPageTableBits`](#sel4_iopagetablebits)
- [`seL4_IPCBufferSizeBits`](#sel4_ipcbuffersizebits)
- [`seL4_InstructionFault`](#sel4_instructionfault)
- [`seL4_LargePageBits`](#sel4_largepagebits)
- [`seL4_MaxUntypedBits`](#sel4_maxuntypedbits)
- [`seL4_MinUntypedBits`](#sel4_minuntypedbits)
- [`seL4_NilData`](#sel4_nildata)
- [`seL4_NotificationBits`](#sel4_notificationbits)
- [`seL4_NumASIDPoolsBits`](#sel4_numasidpoolsbits)
- [`seL4_PDPTBits`](#sel4_pdptbits)
- [`seL4_PDPTEntryBits`](#sel4_pdptentrybits)
- [`seL4_PDPTIndexBits`](#sel4_pdptindexbits)
- [`seL4_PML4Bits`](#sel4_pml4bits)
- [`seL4_PML4EntryBits`](#sel4_pml4entrybits)
- [`seL4_PML4IndexBits`](#sel4_pml4indexbits)
- [`seL4_PageBits`](#sel4_pagebits)
- [`seL4_PageDirBits`](#sel4_pagedirbits)
- [`seL4_PageDirEntryBits`](#sel4_pagedirentrybits)
- [`seL4_PageDirIndexBits`](#sel4_pagedirindexbits)
- [`seL4_PageTableBits`](#sel4_pagetablebits)
- [`seL4_PageTableEntryBits`](#sel4_pagetableentrybits)
- [`seL4_PageTableIndexBits`](#sel4_pagetableindexbits)
- [`seL4_SlotBits`](#sel4_slotbits)
- [`seL4_TCBBits`](#sel4_tcbbits)
- [`seL4_True`](#sel4_true)
- [`seL4_UntypedRetypeMaxObjects`](#sel4_untypedretypemaxobjects)
- [`seL4_UserTop`](#sel4_usertop)
- [`seL4_VCPUBits`](#sel4_vcpubits)
- [`seL4_VSpaceBits`](#sel4_vspacebits)
- [`seL4_WordBits`](#sel4_wordbits)
- [`seL4_WordSizeBits`](#sel4_wordsizebits)
- [`seL4_X86_EPTPDBits`](#sel4_x86_eptpdbits)
- [`seL4_X86_EPTPDEntryBits`](#sel4_x86_eptpdentrybits)
- [`seL4_X86_EPTPDIndexBits`](#sel4_x86_eptpdindexbits)
- [`seL4_X86_EPTPDObject`](#sel4_x86_eptpdobject)
- [`seL4_X86_EPTPDPTBits`](#sel4_x86_eptpdptbits)
- [`seL4_X86_EPTPDPTEntryBits`](#sel4_x86_eptpdptentrybits)
- [`seL4_X86_EPTPDPTIndexBits`](#sel4_x86_eptpdptindexbits)
- [`seL4_X86_EPTPDPTObject`](#sel4_x86_eptpdptobject)
- [`seL4_X86_EPTPML4Bits`](#sel4_x86_eptpml4bits)
- [`seL4_X86_EPTPML4EntryBits`](#sel4_x86_eptpml4entrybits)
- [`seL4_X86_EPTPML4IndexBits`](#sel4_x86_eptpml4indexbits)
- [`seL4_X86_EPTPML4Object`](#sel4_x86_eptpml4object)
- [`seL4_X86_EPTPTBits`](#sel4_x86_eptptbits)
- [`seL4_X86_EPTPTEntryBits`](#sel4_x86_eptptentrybits)
- [`seL4_X86_EPTPTIndexBits`](#sel4_x86_eptptindexbits)
- [`seL4_X86_EPTPTObject`](#sel4_x86_eptptobject)
- [`seL4_X86_VCPUBits`](#sel4_x86_vcpubits)
- [`seL4_X86_VCPUObject`](#sel4_x86_vcpuobject)

**Type Aliases**

- [`seL4_Bool`](#sel4_bool)
- [`seL4_CNode`](#sel4_cnode)
- [`seL4_CPtr`](#sel4_cptr)
- [`seL4_Domain`](#sel4_domain)
- [`seL4_DomainSet`](#sel4_domainset)
- [`seL4_IPCBuffer`](#sel4_ipcbuffer)
- [`seL4_IRQControl`](#sel4_irqcontrol)
- [`seL4_IRQHandler`](#sel4_irqhandler)
- [`seL4_Int16`](#sel4_int16)
- [`seL4_Int32`](#sel4_int32)
- [`seL4_Int64`](#sel4_int64)
- [`seL4_Int8`](#sel4_int8)
- [`seL4_NodeId`](#sel4_nodeid)
- [`seL4_PAddr`](#sel4_paddr)
- [`seL4_SchedContext`](#sel4_schedcontext)
- [`seL4_SchedControl`](#sel4_schedcontrol)
- [`seL4_SlotPos`](#sel4_slotpos)
- [`seL4_TCB`](#sel4_tcb)
- [`seL4_Time`](#sel4_time)
- [`seL4_Uint16`](#sel4_uint16)
- [`seL4_Uint32`](#sel4_uint32)
- [`seL4_Uint64`](#sel4_uint64)
- [`seL4_Uint8`](#sel4_uint8)
- [`seL4_Untyped`](#sel4_untyped)
- [`seL4_UserContext`](#sel4_usercontext)
- [`seL4_VCPUContext`](#sel4_vcpucontext)
- [`seL4_Word`](#sel4_word)
- [`seL4_X64_PML4`](#sel4_x64_pml4)
- [`seL4_X86_ASIDControl`](#sel4_x86_asidcontrol)
- [`seL4_X86_ASIDPool`](#sel4_x86_asidpool)
- [`seL4_X86_EPTPD`](#sel4_x86_eptpd)
- [`seL4_X86_EPTPDPT`](#sel4_x86_eptpdpt)
- [`seL4_X86_EPTPML4`](#sel4_x86_eptpml4)
- [`seL4_X86_EPTPT`](#sel4_x86_eptpt)
- [`seL4_X86_IOPageTable`](#sel4_x86_iopagetable)
- [`seL4_X86_IOPort`](#sel4_x86_ioport)
- [`seL4_X86_IOPortControl`](#sel4_x86_ioportcontrol)
- [`seL4_X86_IOSpace`](#sel4_x86_iospace)
- [`seL4_X86_PDPT`](#sel4_x86_pdpt)
- [`seL4_X86_Page`](#sel4_x86_page)
- [`seL4_X86_PageDirectory`](#sel4_x86_pagedirectory)
- [`seL4_X86_PageTable`](#sel4_x86_pagetable)
- [`seL4_X86_VCPU`](#sel4_x86_vcpu)

---

## sel4_sys::c::IPCBUF_GDT_ENTRY

*Constant*: `u32`



## sel4_sys::c::IPCBUF_GDT_SELECTOR

*Constant*: `u32`



## sel4_sys::c::IRQ_OFFSET

*Constant*: `u32`



## sel4_sys::c::MSI_MAX

*Constant*: `u32`



## sel4_sys::c::MSI_MIN

*Constant*: `u32`



## sel4_sys::c::SEL4_MAPPING_LOOKUP_LEVEL

*Constant*: `u32`



## sel4_sys::c::SEL4_MAPPING_LOOKUP_NO_EPTPD

*Constant*: `u32`



## sel4_sys::c::SEL4_MAPPING_LOOKUP_NO_EPTPDPT

*Constant*: `u32`



## sel4_sys::c::SEL4_MAPPING_LOOKUP_NO_EPTPT

*Constant*: `u32`



## sel4_sys::c::SEL4_MAPPING_LOOKUP_NO_PD

*Constant*: `u32`



## sel4_sys::c::SEL4_MAPPING_LOOKUP_NO_PDPT

*Constant*: `u32`



## sel4_sys::c::SEL4_MAPPING_LOOKUP_NO_PT

*Constant*: `u32`



## sel4_sys::c::TLS_GDT_ENTRY

*Constant*: `u32`



## sel4_sys::c::TLS_GDT_SELECTOR

*Constant*: `u32`



## sel4_sys::c::VECTOR_MAX

*Constant*: `u32`



## sel4_sys::c::VECTOR_MIN

*Constant*: `u32`



## Module: _mode_object



## Module: _object



## Module: api_object



## Module: priorityConstants



## sel4_sys::c::seL4_ASIDPoolBits

*Constant*: `u32`



## sel4_sys::c::seL4_ASIDPoolIndexBits

*Constant*: `u32`



## sel4_sys::c::seL4_BadgeBits

*Constant*: `u32`



## sel4_sys::c::seL4_Bool

*Type Alias*: `seL4_Int8`



## sel4_sys::c::seL4_BootInfo

*Struct*

**Fields:**
- `extraLen: seL4_Word`
- `nodeID: seL4_NodeId`
- `numNodes: seL4_Word`
- `numIOPTLevels: seL4_Word`
- `ipcBuffer: *mut seL4_IPCBuffer`
- `empty: seL4_SlotRegion`
- `sharedFrames: seL4_SlotRegion`
- `userImageFrames: seL4_SlotRegion`
- `userImagePaging: seL4_SlotRegion`
- `ioSpaceCaps: seL4_SlotRegion`
- `extraBIPages: seL4_SlotRegion`
- `initThreadCNodeSizeBits: seL4_Word`
- `initThreadDomain: seL4_Domain`
- `untyped: seL4_SlotRegion`
- `untypedList: [seL4_UntypedDesc; 230]`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> seL4_BootInfo`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_BootInfo) -> bool`



## sel4_sys::c::seL4_BootInfoFrameBits

*Constant*: `u32`



## sel4_sys::c::seL4_BootInfoHeader

*Struct*

**Fields:**
- `id: seL4_Word`
- `len: seL4_Word`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> seL4_BootInfoHeader`
- **Clone**
  - `fn clone(self: &Self) -> seL4_BootInfoHeader`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_BootInfoHeader) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: seL4_BootInfoID



## sel4_sys::c::seL4_CNode

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_CPtr

*Type Alias*: `seL4_Word`



## Module: seL4_CapFault_Msg



## sel4_sys::c::seL4_CapRightsBits

*Constant*: `u32`



## sel4_sys::c::seL4_DataFault

*Constant*: `u32`



## sel4_sys::c::seL4_Domain

*Type Alias*: `seL4_Word`



## sel4_sys::c::seL4_DomainSet

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_EndpointBits

*Constant*: `u32`



## Module: seL4_Error



## sel4_sys::c::seL4_False

*Constant*: `u32`



## sel4_sys::c::seL4_FastMessageRegisters

*Constant*: `u32`



## sel4_sys::c::seL4_GuardBits

*Constant*: `u32`



## sel4_sys::c::seL4_GuardSizeBits

*Constant*: `u32`



## sel4_sys::c::seL4_HugePageBits

*Constant*: `u32`



## sel4_sys::c::seL4_IOPageTableBits

*Constant*: `u32`



## sel4_sys::c::seL4_IPCBuffer

*Type Alias*: `seL4_IPCBuffer_`



## sel4_sys::c::seL4_IPCBufferSizeBits

*Constant*: `u32`



## sel4_sys::c::seL4_IPCBuffer_

*Struct*

**Fields:**
- `tag: seL4_MessageInfo_t`
- `msg: [seL4_Word; 120]`
- `userData: seL4_Word`
- `caps_or_badges: [seL4_Word; 3]`
- `receiveCNode: seL4_CPtr`
- `receiveIndex: seL4_CPtr`
- `receiveDepth: seL4_Word`

**Methods:**

- `fn seL4_Untyped_Retype(self: & mut Self, service: seL4_Untyped, type: seL4_Word, size_bits: seL4_Word, root: seL4_CNode, node_index: seL4_Word, node_depth: seL4_Word, node_offset: seL4_Word, num_objects: seL4_Word) -> seL4_Error::Type`
- `fn seL4_TCB_ReadRegisters(self: & mut Self, service: seL4_TCB, suspend_source: seL4_Bool, arch_flags: seL4_Uint8, count: seL4_Word, regs: & mut seL4_UserContext) -> seL4_Error::Type`
- `fn seL4_TCB_WriteRegisters(self: & mut Self, service: seL4_TCB, resume_target: seL4_Bool, arch_flags: seL4_Uint8, count: seL4_Word, regs: &seL4_UserContext) -> seL4_Error::Type`
- `fn seL4_TCB_CopyRegisters(self: & mut Self, service: seL4_TCB, source: seL4_TCB, suspend_source: seL4_Bool, resume_target: seL4_Bool, transfer_frame: seL4_Bool, transfer_integer: seL4_Bool, arch_flags: seL4_Uint8) -> seL4_Error::Type`
- `fn seL4_TCB_Configure(self: & mut Self, service: seL4_TCB, fault_ep: seL4_Word, cspace_root: seL4_CNode, cspace_root_data: seL4_Word, vspace_root: seL4_CPtr, vspace_root_data: seL4_Word, buffer: seL4_Word, bufferFrame: seL4_CPtr) -> seL4_Error::Type`
- `fn seL4_TCB_SetPriority(self: & mut Self, service: seL4_TCB, authority: seL4_TCB, priority: seL4_Word) -> seL4_Error::Type`
- `fn seL4_TCB_SetMCPriority(self: & mut Self, service: seL4_TCB, authority: seL4_TCB, mcp: seL4_Word) -> seL4_Error::Type`
- `fn seL4_TCB_SetSchedParams(self: & mut Self, service: seL4_TCB, authority: seL4_TCB, mcp: seL4_Word, priority: seL4_Word) -> seL4_Error::Type`
- `fn seL4_TCB_SetIPCBuffer(self: & mut Self, service: seL4_TCB, buffer: seL4_Word, bufferFrame: seL4_CPtr) -> seL4_Error::Type`
- `fn seL4_TCB_SetSpace(self: & mut Self, service: seL4_TCB, fault_ep: seL4_Word, cspace_root: seL4_CNode, cspace_root_data: seL4_Word, vspace_root: seL4_CPtr, vspace_root_data: seL4_Word) -> seL4_Error::Type`
- `fn seL4_TCB_Suspend(self: & mut Self, service: seL4_TCB) -> seL4_Error::Type`
- `fn seL4_TCB_Resume(self: & mut Self, service: seL4_TCB) -> seL4_Error::Type`
- `fn seL4_TCB_BindNotification(self: & mut Self, service: seL4_TCB, notification: seL4_CPtr) -> seL4_Error::Type`
- `fn seL4_TCB_UnbindNotification(self: & mut Self, service: seL4_TCB) -> seL4_Error::Type`
- `fn seL4_TCB_SetTLSBase(self: & mut Self, service: seL4_TCB, tls_base: seL4_Word) -> seL4_Error::Type`
- `fn seL4_TCB_SetFlags(self: & mut Self, service: seL4_TCB, clear: seL4_Word, set: seL4_Word) -> seL4_TCB_SetFlags_ret`
- `fn seL4_CNode_Revoke(self: & mut Self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type`
- `fn seL4_CNode_Delete(self: & mut Self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type`
- `fn seL4_CNode_CancelBadgedSends(self: & mut Self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type`
- `fn seL4_CNode_Copy(self: & mut Self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8, rights: seL4_CapRights_t) -> seL4_Error::Type`
- `fn seL4_CNode_Mint(self: & mut Self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8, rights: seL4_CapRights_t, badge: seL4_Word) -> seL4_Error::Type`
- `fn seL4_CNode_Move(self: & mut Self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8) -> seL4_Error::Type`
- `fn seL4_CNode_Mutate(self: & mut Self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8, badge: seL4_Word) -> seL4_Error::Type`
- `fn seL4_CNode_Rotate(self: & mut Self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, dest_badge: seL4_Word, pivot_root: seL4_CNode, pivot_index: seL4_Word, pivot_depth: seL4_Uint8, pivot_badge: seL4_Word, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8) -> seL4_Error::Type`
- `fn seL4_CNode_SaveCaller(self: & mut Self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type`
- `fn seL4_IRQControl_Get(self: & mut Self, service: seL4_IRQControl, irq: seL4_Word, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type`
- `fn seL4_IRQHandler_Ack(self: & mut Self, service: seL4_IRQHandler) -> seL4_Error::Type`
- `fn seL4_IRQHandler_SetNotification(self: & mut Self, service: seL4_IRQHandler, notification: seL4_CPtr) -> seL4_Error::Type`
- `fn seL4_IRQHandler_Clear(self: & mut Self, service: seL4_IRQHandler) -> seL4_Error::Type`
- `fn seL4_DomainSet_Set(self: & mut Self, service: seL4_DomainSet, domain: seL4_Uint8, thread: seL4_TCB) -> seL4_Error::Type`
- `fn seL4_DomainSet_ScheduleConfigure(self: & mut Self, service: seL4_DomainSet, index: seL4_Word, domain: seL4_Uint8, duration: seL4_Time) -> seL4_Error::Type`
- `fn seL4_DomainSet_ScheduleSetStart(self: & mut Self, service: seL4_DomainSet, index: seL4_Word) -> seL4_Error::Type`
- `fn seL4_X86_PDPT_Map(self: & mut Self, service: seL4_X86_PDPT, pml4: seL4_X64_PML4, vaddr: seL4_Word, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type`
- `fn seL4_X86_PDPT_Unmap(self: & mut Self, service: seL4_X86_PDPT) -> seL4_Error::Type`
- `fn seL4_X86_PageDirectory_Map(self: & mut Self, service: seL4_X86_PageDirectory, vspace: seL4_CPtr, vaddr: seL4_Word, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type`
- `fn seL4_X86_PageDirectory_Unmap(self: & mut Self, service: seL4_X86_PageDirectory) -> seL4_Error::Type`
- `fn seL4_X86_PageTable_Map(self: & mut Self, service: seL4_X86_PageTable, vspace: seL4_CPtr, vaddr: seL4_Word, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type`
- `fn seL4_X86_PageTable_Unmap(self: & mut Self, service: seL4_X86_PageTable) -> seL4_Error::Type`
- `fn seL4_X86_IOPageTable_Map(self: & mut Self, service: seL4_X86_IOPageTable, iospace: seL4_X86_IOSpace, ioaddr: seL4_Word) -> seL4_Error::Type`
- `fn seL4_X86_IOPageTable_Unmap(self: & mut Self, service: seL4_X86_IOPageTable) -> seL4_Error::Type`
- `fn seL4_X86_Page_Map(self: & mut Self, service: seL4_X86_Page, vspace: seL4_CPtr, vaddr: seL4_Word, rights: seL4_CapRights_t, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type`
- `fn seL4_X86_Page_Unmap(self: & mut Self, service: seL4_X86_Page) -> seL4_Error::Type`
- `fn seL4_X86_Page_MapIO(self: & mut Self, service: seL4_X86_Page, iospace: seL4_X86_IOSpace, rights: seL4_CapRights_t, ioaddr: seL4_Word) -> seL4_Error::Type`
- `fn seL4_X86_Page_GetAddress(self: & mut Self, service: seL4_X86_Page) -> seL4_X86_Page_GetAddress_ret`
- `fn seL4_X86_ASIDControl_MakePool(self: & mut Self, service: seL4_X86_ASIDControl, untyped: seL4_Untyped, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type`
- `fn seL4_X86_ASIDPool_Assign(self: & mut Self, service: seL4_X86_ASIDPool, vspace: seL4_CPtr) -> seL4_Error::Type`
- `fn seL4_X86_IOPortControl_Issue(self: & mut Self, service: seL4_X86_IOPortControl, first_port: seL4_Word, last_port: seL4_Word, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type`
- `fn seL4_X86_IOPort_In8(self: & mut Self, service: seL4_X86_IOPort, port: seL4_Uint16) -> seL4_X86_IOPort_In8_ret`
- `fn seL4_X86_IOPort_In16(self: & mut Self, service: seL4_X86_IOPort, port: seL4_Uint16) -> seL4_X86_IOPort_In16_ret`
- `fn seL4_X86_IOPort_In32(self: & mut Self, service: seL4_X86_IOPort, port: seL4_Uint16) -> seL4_X86_IOPort_In32_ret`
- `fn seL4_X86_IOPort_Out8(self: & mut Self, service: seL4_X86_IOPort, port: seL4_Word, data: seL4_Word) -> seL4_Error::Type`
- `fn seL4_X86_IOPort_Out16(self: & mut Self, service: seL4_X86_IOPort, port: seL4_Word, data: seL4_Word) -> seL4_Error::Type`
- `fn seL4_X86_IOPort_Out32(self: & mut Self, service: seL4_X86_IOPort, port: seL4_Word, data: seL4_Word) -> seL4_Error::Type`
- `fn seL4_IRQControl_GetIOAPIC(self: & mut Self, service: seL4_IRQControl, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8, ioapic: seL4_Word, pin: seL4_Word, level: seL4_Word, polarity: seL4_Word, vector: seL4_Word) -> seL4_Error::Type`
- `fn seL4_IRQControl_GetMSI(self: & mut Self, service: seL4_IRQControl, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8, pci_bus: seL4_Word, pci_dev: seL4_Word, pci_func: seL4_Word, handle: seL4_Word, vector: seL4_Word) -> seL4_Error::Type`
- `fn seL4_Send(self: & mut Self, dest: seL4_CPtr, msg_info: seL4_MessageInfo)`
- `fn seL4_SendWithMRs(self: & mut Self, dest: seL4_CPtr, msg_info: seL4_MessageInfo, msg0: Option<seL4_Word>, msg1: Option<seL4_Word>, msg2: Option<seL4_Word>, msg3: Option<seL4_Word>)`
- `fn seL4_NBSend(self: & mut Self, dest: seL4_CPtr, msg_info: seL4_MessageInfo)`
- `fn seL4_NBSendWithMRs(self: & mut Self, dest: seL4_CPtr, msg_info: seL4_MessageInfo, msg0: Option<seL4_Word>, msg1: Option<seL4_Word>, msg2: Option<seL4_Word>, msg3: Option<seL4_Word>)`
- `fn seL4_Reply(self: & mut Self, msg_info: seL4_MessageInfo)`
- `fn seL4_ReplyWithMRs(self: & mut Self, msg_info: seL4_MessageInfo, msg0: Option<seL4_Word>, msg1: Option<seL4_Word>, msg2: Option<seL4_Word>, msg3: Option<seL4_Word>)`
- `fn seL4_Signal(self: & mut Self, dest: seL4_CPtr)`
- `fn seL4_Recv(self: & mut Self, src: seL4_CPtr, reply_authority: ReplyAuthority) -> (seL4_MessageInfo, seL4_Word)`
- `fn seL4_RecvWithMRs(self: & mut Self, src: seL4_CPtr, msg0: Option<& mut seL4_Word>, msg1: Option<& mut seL4_Word>, msg2: Option<& mut seL4_Word>, msg3: Option<& mut seL4_Word>, reply_authority: ReplyAuthority) -> (seL4_MessageInfo, seL4_Word)`
- `fn seL4_NBRecv(self: & mut Self, src: seL4_CPtr, reply_authority: ReplyAuthority) -> (seL4_MessageInfo, seL4_Word)`
- `fn seL4_Call(self: & mut Self, dest: seL4_CPtr, msg_info: seL4_MessageInfo) -> seL4_MessageInfo`
- `fn seL4_CallWithMRs(self: & mut Self, dest: seL4_CPtr, msg_info: seL4_MessageInfo, msg0: Option<& mut seL4_Word>, msg1: Option<& mut seL4_Word>, msg2: Option<& mut seL4_Word>, msg3: Option<& mut seL4_Word>) -> seL4_MessageInfo`
- `fn seL4_ReplyRecv(self: & mut Self, src: seL4_CPtr, msg_info: seL4_MessageInfo, reply_authority: ReplyAuthority) -> (seL4_MessageInfo, seL4_Word)`
- `fn seL4_Wait(self: & mut Self, src: seL4_CPtr) -> (WaitMessageInfo, seL4_Word)`
- `fn seL4_Poll(self: & mut Self, src: seL4_CPtr) -> (seL4_MessageInfo, seL4_Word)`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## sel4_sys::c::seL4_IRQControl

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_IRQHandler

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_InstructionFault

*Constant*: `u32`



## sel4_sys::c::seL4_Int16

*Type Alias*: `::core::ffi::c_short`



## sel4_sys::c::seL4_Int32

*Type Alias*: `::core::ffi::c_int`



## sel4_sys::c::seL4_Int64

*Type Alias*: `::core::ffi::c_long`



## sel4_sys::c::seL4_Int8

*Type Alias*: `::core::ffi::c_schar`



## sel4_sys::c::seL4_LargePageBits

*Constant*: `u32`



## Module: seL4_LookupFailureType



## sel4_sys::c::seL4_MaxUntypedBits

*Constant*: `u32`



## sel4_sys::c::seL4_MinUntypedBits

*Constant*: `u32`



## Module: seL4_MsgLimits



## sel4_sys::c::seL4_NilData

*Constant*: `u32`



## sel4_sys::c::seL4_NodeId

*Type Alias*: `seL4_Word`



## sel4_sys::c::seL4_NotificationBits

*Constant*: `u32`



## sel4_sys::c::seL4_NumASIDPoolsBits

*Constant*: `u32`



## sel4_sys::c::seL4_PAddr

*Type Alias*: `seL4_Word`



## sel4_sys::c::seL4_PDPTBits

*Constant*: `u32`



## sel4_sys::c::seL4_PDPTEntryBits

*Constant*: `u32`



## sel4_sys::c::seL4_PDPTIndexBits

*Constant*: `u32`



## sel4_sys::c::seL4_PML4Bits

*Constant*: `u32`



## sel4_sys::c::seL4_PML4EntryBits

*Constant*: `u32`



## sel4_sys::c::seL4_PML4IndexBits

*Constant*: `u32`



## sel4_sys::c::seL4_PageBits

*Constant*: `u32`



## sel4_sys::c::seL4_PageDirBits

*Constant*: `u32`



## sel4_sys::c::seL4_PageDirEntryBits

*Constant*: `u32`



## sel4_sys::c::seL4_PageDirIndexBits

*Constant*: `u32`



## sel4_sys::c::seL4_PageTableBits

*Constant*: `u32`



## sel4_sys::c::seL4_PageTableEntryBits

*Constant*: `u32`



## sel4_sys::c::seL4_PageTableIndexBits

*Constant*: `u32`



## Module: seL4_RootCNodeCapSlots



## sel4_sys::c::seL4_SchedContext

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_SchedContext_Consumed

*Struct*

**Fields:**
- `error: ::core::ffi::c_int`
- `consumed: seL4_Time`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> seL4_SchedContext_Consumed`
- **Clone**
  - `fn clone(self: &Self) -> seL4_SchedContext_Consumed`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_SchedContext_Consumed) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_sys::c::seL4_SchedContext_YieldTo

*Struct*

**Fields:**
- `error: ::core::ffi::c_int`
- `consumed: seL4_Time`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_SchedContext_YieldTo`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_SchedContext_YieldTo) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> seL4_SchedContext_YieldTo`



## sel4_sys::c::seL4_SchedControl

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_SlotBits

*Constant*: `u32`



## sel4_sys::c::seL4_SlotPos

*Type Alias*: `seL4_Word`



## sel4_sys::c::seL4_SlotRegion

*Struct*

**Fields:**
- `start: seL4_SlotPos`
- `end: seL4_SlotPos`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_SlotRegion`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_SlotRegion) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> seL4_SlotRegion`



## sel4_sys::c::seL4_TCB

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_TCBBits

*Constant*: `u32`



## Module: seL4_TCBFlag



## sel4_sys::c::seL4_TCB_ConfigureSingleStepping

*Struct*

**Fields:**
- `error: ::core::ffi::c_int`
- `bp_was_consumed: seL4_Bool`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_TCB_ConfigureSingleStepping`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_TCB_ConfigureSingleStepping) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> seL4_TCB_ConfigureSingleStepping`



## sel4_sys::c::seL4_TCB_GetBreakpoint

*Struct*

**Fields:**
- `error: ::core::ffi::c_int`
- `vaddr: seL4_Word`
- `type_: seL4_Word`
- `size: seL4_Word`
- `rw: seL4_Word`
- `is_enabled: seL4_Bool`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> seL4_TCB_GetBreakpoint`
- **Clone**
  - `fn clone(self: &Self) -> seL4_TCB_GetBreakpoint`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_TCB_GetBreakpoint) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_sys::c::seL4_Time

*Type Alias*: `seL4_Uint64`



## sel4_sys::c::seL4_True

*Constant*: `u32`



## sel4_sys::c::seL4_Uint16

*Type Alias*: `::core::ffi::c_ushort`



## sel4_sys::c::seL4_Uint32

*Type Alias*: `::core::ffi::c_uint`



## sel4_sys::c::seL4_Uint64

*Type Alias*: `::core::ffi::c_ulong`



## sel4_sys::c::seL4_Uint8

*Type Alias*: `::core::ffi::c_uchar`



## Module: seL4_UnknownSyscall_Msg



## sel4_sys::c::seL4_Untyped

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_UntypedDesc

*Struct*

**Fields:**
- `paddr: seL4_Word`
- `sizeBits: seL4_Uint8`
- `isDevice: seL4_Uint8`
- `padding: [seL4_Uint8; 6]`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_UntypedDesc`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_UntypedDesc) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> seL4_UntypedDesc`



## sel4_sys::c::seL4_UntypedRetypeMaxObjects

*Constant*: `u32`



## sel4_sys::c::seL4_UserContext

*Type Alias*: `seL4_UserContext_`



## sel4_sys::c::seL4_UserContext_

*Struct*

**Fields:**
- `rip: seL4_Word`
- `rsp: seL4_Word`
- `rflags: seL4_Word`
- `rax: seL4_Word`
- `rbx: seL4_Word`
- `rcx: seL4_Word`
- `rdx: seL4_Word`
- `rsi: seL4_Word`
- `rdi: seL4_Word`
- `rbp: seL4_Word`
- `r8: seL4_Word`
- `r9: seL4_Word`
- `r10: seL4_Word`
- `r11: seL4_Word`
- `r12: seL4_Word`
- `r13: seL4_Word`
- `r14: seL4_Word`
- `r15: seL4_Word`
- `fs_base: seL4_Word`
- `gs_base: seL4_Word`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> seL4_UserContext_`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_UserContext_) -> bool`
- **Default**
  - `fn default() -> seL4_UserContext_`



## Module: seL4_UserException_Msg



## sel4_sys::c::seL4_UserTop

*Constant*: `u64`



## sel4_sys::c::seL4_VCPUBits

*Constant*: `u32`



## sel4_sys::c::seL4_VCPUContext

*Type Alias*: `seL4_VCPUContext_`



## sel4_sys::c::seL4_VCPUContext_

*Struct*

**Fields:**
- `eax: seL4_Word`
- `ebx: seL4_Word`
- `ecx: seL4_Word`
- `edx: seL4_Word`
- `esi: seL4_Word`
- `edi: seL4_Word`
- `ebp: seL4_Word`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> seL4_VCPUContext_`
- **Clone**
  - `fn clone(self: &Self) -> seL4_VCPUContext_`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_VCPUContext_) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: seL4_VMFault_Msg



## sel4_sys::c::seL4_VSpaceBits

*Constant*: `u32`



## sel4_sys::c::seL4_Word

*Type Alias*: `seL4_Uint64`



## sel4_sys::c::seL4_WordBits

*Constant*: `u32`



## sel4_sys::c::seL4_WordSizeBits

*Constant*: `u32`



## sel4_sys::c::seL4_X64_PML4

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_ASIDControl

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_ASIDPool

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_EPTPD

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_EPTPDBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPDEntryBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPDIndexBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPDObject

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPDPT

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_EPTPDPTBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPDPTEntryBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPDPTIndexBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPDPTObject

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPML4

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_EPTPML4Bits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPML4EntryBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPML4IndexBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPML4Object

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPT

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_EPTPTBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPTEntryBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPTIndexBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_EPTPTObject

*Constant*: `u32`



## Module: seL4_X86_EPT_VMAttributes



## sel4_sys::c::seL4_X86_IOPageTable

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_IOPort

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_IOPortControl

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_IOSpace

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_PDPT

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_Page

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_PageDirectory

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_PageDirectory_GetStatusBits

*Struct*

**Fields:**
- `error: ::core::ffi::c_int`
- `accessed: seL4_Word`
- `dirty: seL4_Word`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> seL4_X86_PageDirectory_GetStatusBits`
- **Clone**
  - `fn clone(self: &Self) -> seL4_X86_PageDirectory_GetStatusBits`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_X86_PageDirectory_GetStatusBits) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_sys::c::seL4_X86_PageTable

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_VCPU

*Type Alias*: `seL4_CPtr`



## sel4_sys::c::seL4_X86_VCPUBits

*Constant*: `u32`



## sel4_sys::c::seL4_X86_VCPUObject

*Constant*: `u32`



## sel4_sys::c::seL4_X86_VCPU_ReadMSR

*Struct*

**Fields:**
- `error: ::core::ffi::c_int`
- `value: seL4_Word`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> seL4_X86_VCPU_ReadMSR`
- **Clone**
  - `fn clone(self: &Self) -> seL4_X86_VCPU_ReadMSR`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_X86_VCPU_ReadMSR) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_sys::c::seL4_X86_VCPU_ReadVMCS

*Struct*

**Fields:**
- `error: ::core::ffi::c_int`
- `value: seL4_Word`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> seL4_X86_VCPU_ReadVMCS`
- **Clone**
  - `fn clone(self: &Self) -> seL4_X86_VCPU_ReadVMCS`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_X86_VCPU_ReadVMCS) -> bool`



## sel4_sys::c::seL4_X86_VCPU_WriteMSR

*Struct*

**Fields:**
- `error: ::core::ffi::c_int`
- `written: seL4_Word`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_X86_VCPU_WriteMSR`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_X86_VCPU_WriteMSR) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> seL4_X86_VCPU_WriteMSR`



## sel4_sys::c::seL4_X86_VCPU_WriteVMCS

*Struct*

**Fields:**
- `error: ::core::ffi::c_int`
- `written: seL4_Word`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_X86_VCPU_WriteVMCS`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_X86_VCPU_WriteVMCS) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> seL4_X86_VCPU_WriteVMCS`



## Module: seL4_X86_VMAttributes



