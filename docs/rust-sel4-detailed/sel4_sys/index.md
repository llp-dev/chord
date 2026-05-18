# Crate `sel4_sys`

## Contents

- [Modules](#modules)
  - [`bf`](#bf)
  - [`c`](#c)
  - [`fault`](#fault)
  - [`invocations`](#invocations)
  - [`ipc_buffer`](#ipc-buffer)
  - [`syscalls`](#syscalls)
  - [`seL4_Fault_tag`](#sel4-fault-tag)
  - [`seL4_VMFault_Msg`](#sel4-vmfault-msg)
  - [`seL4_UnknownSyscall_Msg`](#sel4-unknownsyscall-msg)
  - [`seL4_UserException_Msg`](#sel4-userexception-msg)
  - [`seL4_X86_VMAttributes`](#sel4-x86-vmattributes)
  - [`seL4_X86_EPT_VMAttributes`](#sel4-x86-ept-vmattributes)
  - [`api_object`](#api-object)
  - [`_mode_object`](#mode-object)
  - [`_object`](#object)
  - [`seL4_Error`](#sel4-error)
  - [`priorityConstants`](#priorityconstants)
  - [`seL4_MsgLimits`](#sel4-msglimits)
  - [`seL4_LookupFailureType`](#sel4-lookupfailuretype)
  - [`seL4_TCBFlag`](#sel4-tcbflag)
  - [`seL4_CapFault_Msg`](#sel4-capfault-msg)
  - [`seL4_RootCNodeCapSlots`](#sel4-rootcnodecapslots)
  - [`seL4_BootInfoID`](#sel4-bootinfoid)
  - [`invocation_label`](#invocation-label)
  - [`calls`](#calls)
  - [`helpers`](#helpers)
  - [`syscall_id`](#syscall-id)
- [Structs](#structs)
  - [`seL4_Fault_NullFault`](#sel4-fault-nullfault)
  - [`seL4_Fault_NullFault_Unpacked`](#sel4-fault-nullfault-unpacked)
  - [`seL4_Fault_CapFault`](#sel4-fault-capfault)
  - [`seL4_Fault_CapFault_Unpacked`](#sel4-fault-capfault-unpacked)
  - [`seL4_Fault_UnknownSyscall`](#sel4-fault-unknownsyscall)
  - [`seL4_Fault_UnknownSyscall_Unpacked`](#sel4-fault-unknownsyscall-unpacked)
  - [`seL4_Fault_UserException`](#sel4-fault-userexception)
  - [`seL4_Fault_UserException_Unpacked`](#sel4-fault-userexception-unpacked)
  - [`seL4_Fault_VMFault`](#sel4-fault-vmfault)
  - [`seL4_Fault_VMFault_Unpacked`](#sel4-fault-vmfault-unpacked)
  - [`seL4_Fault`](#sel4-fault)
  - [`seL4_MessageInfo`](#sel4-messageinfo)
  - [`seL4_MessageInfo_Unpacked`](#sel4-messageinfo-unpacked)
  - [`seL4_CapRights`](#sel4-caprights)
  - [`seL4_CapRights_Unpacked`](#sel4-caprights-unpacked)
  - [`seL4_CNode_CapData`](#sel4-cnode-capdata)
  - [`seL4_CNode_CapData_Unpacked`](#sel4-cnode-capdata-unpacked)
  - [`seL4_UserContext_`](#sel4-usercontext)
  - [`seL4_VCPUContext_`](#sel4-vcpucontext)
  - [`seL4_IPCBuffer_`](#sel4-ipcbuffer)
  - [`seL4_X86_VCPU_ReadMSR`](#sel4-x86-vcpu-readmsr)
  - [`seL4_X86_VCPU_WriteMSR`](#sel4-x86-vcpu-writemsr)
  - [`seL4_X86_PageDirectory_GetStatusBits`](#sel4-x86-pagedirectory-getstatusbits)
  - [`seL4_X86_VCPU_ReadVMCS`](#sel4-x86-vcpu-readvmcs)
  - [`seL4_X86_VCPU_WriteVMCS`](#sel4-x86-vcpu-writevmcs)
  - [`seL4_TCB_GetBreakpoint`](#sel4-tcb-getbreakpoint)
  - [`seL4_TCB_ConfigureSingleStepping`](#sel4-tcb-configuresinglestepping)
  - [`seL4_SchedContext_Consumed`](#sel4-schedcontext-consumed)
  - [`seL4_SchedContext_YieldTo`](#sel4-schedcontext-yieldto)
  - [`seL4_SlotRegion`](#sel4-slotregion)
  - [`seL4_UntypedDesc`](#sel4-untypeddesc)
  - [`seL4_BootInfo`](#sel4-bootinfo)
  - [`seL4_BootInfoHeader`](#sel4-bootinfoheader)
  - [`seL4_TCB_SetFlags_ret`](#sel4-tcb-setflags-ret)
  - [`seL4_X86_Page_GetAddress_ret`](#sel4-x86-page-getaddress-ret)
  - [`seL4_X86_IOPort_In8_ret`](#sel4-x86-ioport-in8-ret)
  - [`seL4_X86_IOPort_In16_ret`](#sel4-x86-ioport-in16-ret)
  - [`seL4_X86_IOPort_In32_ret`](#sel4-x86-ioport-in32-ret)
- [Enums](#enums)
  - [`seL4_Fault_Splayed`](#sel4-fault-splayed)
- [Type Aliases](#type-aliases)
  - [`ReplyAuthority`](#replyauthority)
  - [`WaitMessageInfo`](#waitmessageinfo)
  - [`SeL4Bitfield`](#sel4bitfield)
  - [`seL4_MessageInfo_t`](#sel4-messageinfo-t)
  - [`seL4_CapRights_t`](#sel4-caprights-t)
  - [`seL4_CNode_CapData_t`](#sel4-cnode-capdata-t)
  - [`seL4_Int8`](#sel4-int8)
  - [`seL4_Uint8`](#sel4-uint8)
  - [`seL4_Int16`](#sel4-int16)
  - [`seL4_Uint16`](#sel4-uint16)
  - [`seL4_Int32`](#sel4-int32)
  - [`seL4_Uint32`](#sel4-uint32)
  - [`seL4_Int64`](#sel4-int64)
  - [`seL4_Uint64`](#sel4-uint64)
  - [`seL4_Bool`](#sel4-bool)
  - [`seL4_Word`](#sel4-word)
  - [`seL4_CPtr`](#sel4-cptr)
  - [`seL4_X64_PML4`](#sel4-x64-pml4)
  - [`seL4_UserContext`](#sel4-usercontext)
  - [`seL4_X86_ASIDControl`](#sel4-x86-asidcontrol)
  - [`seL4_X86_ASIDPool`](#sel4-x86-asidpool)
  - [`seL4_X86_IOSpace`](#sel4-x86-iospace)
  - [`seL4_X86_IOPort`](#sel4-x86-ioport)
  - [`seL4_X86_IOPortControl`](#sel4-x86-ioportcontrol)
  - [`seL4_X86_Page`](#sel4-x86-page)
  - [`seL4_X86_PDPT`](#sel4-x86-pdpt)
  - [`seL4_X86_PageDirectory`](#sel4-x86-pagedirectory)
  - [`seL4_X86_PageTable`](#sel4-x86-pagetable)
  - [`seL4_X86_IOPageTable`](#sel4-x86-iopagetable)
  - [`seL4_X86_EPTPML4`](#sel4-x86-eptpml4)
  - [`seL4_X86_EPTPDPT`](#sel4-x86-eptpdpt)
  - [`seL4_X86_EPTPD`](#sel4-x86-eptpd)
  - [`seL4_X86_EPTPT`](#sel4-x86-eptpt)
  - [`seL4_X86_VCPU`](#sel4-x86-vcpu)
  - [`seL4_VCPUContext`](#sel4-vcpucontext)
  - [`seL4_IPCBuffer`](#sel4-ipcbuffer)
  - [`seL4_NodeId`](#sel4-nodeid)
  - [`seL4_PAddr`](#sel4-paddr)
  - [`seL4_Domain`](#sel4-domain)
  - [`seL4_CNode`](#sel4-cnode)
  - [`seL4_IRQHandler`](#sel4-irqhandler)
  - [`seL4_IRQControl`](#sel4-irqcontrol)
  - [`seL4_TCB`](#sel4-tcb)
  - [`seL4_Untyped`](#sel4-untyped)
  - [`seL4_DomainSet`](#sel4-domainset)
  - [`seL4_SchedContext`](#sel4-schedcontext)
  - [`seL4_SchedControl`](#sel4-schedcontrol)
  - [`seL4_Time`](#sel4-time)
  - [`seL4_SlotPos`](#sel4-slotpos)
- [Constants](#constants)
  - [`seL4_True`](#sel4-true)
  - [`seL4_False`](#sel4-false)
  - [`TLS_GDT_ENTRY`](#tls-gdt-entry)
  - [`TLS_GDT_SELECTOR`](#tls-gdt-selector)
  - [`IPCBUF_GDT_ENTRY`](#ipcbuf-gdt-entry)
  - [`IPCBUF_GDT_SELECTOR`](#ipcbuf-gdt-selector)
  - [`seL4_DataFault`](#sel4-datafault)
  - [`seL4_InstructionFault`](#sel4-instructionfault)
  - [`seL4_WordBits`](#sel4-wordbits)
  - [`seL4_WordSizeBits`](#sel4-wordsizebits)
  - [`seL4_PageBits`](#sel4-pagebits)
  - [`seL4_SlotBits`](#sel4-slotbits)
  - [`seL4_TCBBits`](#sel4-tcbbits)
  - [`seL4_EndpointBits`](#sel4-endpointbits)
  - [`seL4_NotificationBits`](#sel4-notificationbits)
  - [`seL4_PageTableBits`](#sel4-pagetablebits)
  - [`seL4_PageTableEntryBits`](#sel4-pagetableentrybits)
  - [`seL4_PageTableIndexBits`](#sel4-pagetableindexbits)
  - [`seL4_PageDirBits`](#sel4-pagedirbits)
  - [`seL4_PageDirEntryBits`](#sel4-pagedirentrybits)
  - [`seL4_PageDirIndexBits`](#sel4-pagedirindexbits)
  - [`seL4_PDPTBits`](#sel4-pdptbits)
  - [`seL4_PDPTEntryBits`](#sel4-pdptentrybits)
  - [`seL4_PDPTIndexBits`](#sel4-pdptindexbits)
  - [`seL4_PML4Bits`](#sel4-pml4bits)
  - [`seL4_PML4EntryBits`](#sel4-pml4entrybits)
  - [`seL4_PML4IndexBits`](#sel4-pml4indexbits)
  - [`seL4_VSpaceBits`](#sel4-vspacebits)
  - [`seL4_IOPageTableBits`](#sel4-iopagetablebits)
  - [`seL4_LargePageBits`](#sel4-largepagebits)
  - [`seL4_HugePageBits`](#sel4-hugepagebits)
  - [`seL4_NumASIDPoolsBits`](#sel4-numasidpoolsbits)
  - [`seL4_ASIDPoolBits`](#sel4-asidpoolbits)
  - [`seL4_ASIDPoolIndexBits`](#sel4-asidpoolindexbits)
  - [`seL4_MinUntypedBits`](#sel4-minuntypedbits)
  - [`seL4_MaxUntypedBits`](#sel4-maxuntypedbits)
  - [`seL4_FastMessageRegisters`](#sel4-fastmessageregisters)
  - [`seL4_IPCBufferSizeBits`](#sel4-ipcbuffersizebits)
  - [`seL4_UserTop`](#sel4-usertop)
  - [`seL4_X86_VCPUObject`](#sel4-x86-vcpuobject)
  - [`seL4_X86_EPTPML4Object`](#sel4-x86-eptpml4object)
  - [`seL4_X86_EPTPDPTObject`](#sel4-x86-eptpdptobject)
  - [`seL4_X86_EPTPDObject`](#sel4-x86-eptpdobject)
  - [`seL4_X86_EPTPTObject`](#sel4-x86-eptptobject)
  - [`seL4_CapRightsBits`](#sel4-caprightsbits)
  - [`seL4_GuardSizeBits`](#sel4-guardsizebits)
  - [`seL4_GuardBits`](#sel4-guardbits)
  - [`seL4_BadgeBits`](#sel4-badgebits)
  - [`seL4_UntypedRetypeMaxObjects`](#sel4-untypedretypemaxobjects)
  - [`seL4_NilData`](#sel4-nildata)
  - [`IRQ_OFFSET`](#irq-offset)
  - [`VECTOR_MIN`](#vector-min)
  - [`VECTOR_MAX`](#vector-max)
  - [`MSI_MIN`](#msi-min)
  - [`MSI_MAX`](#msi-max)
  - [`seL4_VCPUBits`](#sel4-vcpubits)
  - [`seL4_X86_VCPUBits`](#sel4-x86-vcpubits)
  - [`seL4_X86_EPTPML4EntryBits`](#sel4-x86-eptpml4entrybits)
  - [`seL4_X86_EPTPML4IndexBits`](#sel4-x86-eptpml4indexbits)
  - [`seL4_X86_EPTPML4Bits`](#sel4-x86-eptpml4bits)
  - [`seL4_X86_EPTPDPTEntryBits`](#sel4-x86-eptpdptentrybits)
  - [`seL4_X86_EPTPDPTIndexBits`](#sel4-x86-eptpdptindexbits)
  - [`seL4_X86_EPTPDPTBits`](#sel4-x86-eptpdptbits)
  - [`seL4_X86_EPTPDEntryBits`](#sel4-x86-eptpdentrybits)
  - [`seL4_X86_EPTPDIndexBits`](#sel4-x86-eptpdindexbits)
  - [`seL4_X86_EPTPDBits`](#sel4-x86-eptpdbits)
  - [`seL4_X86_EPTPTEntryBits`](#sel4-x86-eptptentrybits)
  - [`seL4_X86_EPTPTIndexBits`](#sel4-x86-eptptindexbits)
  - [`seL4_X86_EPTPTBits`](#sel4-x86-eptptbits)
  - [`seL4_BootInfoFrameBits`](#sel4-bootinfoframebits)
  - [`SEL4_MAPPING_LOOKUP_LEVEL`](#sel4-mapping-lookup-level)
  - [`SEL4_MAPPING_LOOKUP_NO_PT`](#sel4-mapping-lookup-no-pt)
  - [`SEL4_MAPPING_LOOKUP_NO_PD`](#sel4-mapping-lookup-no-pd)
  - [`SEL4_MAPPING_LOOKUP_NO_PDPT`](#sel4-mapping-lookup-no-pdpt)
  - [`SEL4_MAPPING_LOOKUP_NO_EPTPDPT`](#sel4-mapping-lookup-no-eptpdpt)
  - [`SEL4_MAPPING_LOOKUP_NO_EPTPD`](#sel4-mapping-lookup-no-eptpd)
  - [`SEL4_MAPPING_LOOKUP_NO_EPTPT`](#sel4-mapping-lookup-no-eptpt)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`bf`](#bf) | mod |  |
| [`c`](#c) | mod |  |
| [`fault`](#fault) | mod |  |
| [`invocations`](#invocations) | mod |  |
| [`ipc_buffer`](#ipc-buffer) | mod |  |
| [`syscalls`](#syscalls) | mod |  |
| [`seL4_Fault_tag`](#sel4-fault-tag) | mod |  |
| [`seL4_VMFault_Msg`](#sel4-vmfault-msg) | mod |  |
| [`seL4_UnknownSyscall_Msg`](#sel4-unknownsyscall-msg) | mod |  |
| [`seL4_UserException_Msg`](#sel4-userexception-msg) | mod |  |
| [`seL4_X86_VMAttributes`](#sel4-x86-vmattributes) | mod |  |
| [`seL4_X86_EPT_VMAttributes`](#sel4-x86-ept-vmattributes) | mod |  |
| [`api_object`](#api-object) | mod |  |
| [`_mode_object`](#mode-object) | mod |  |
| [`_object`](#object) | mod |  |
| [`seL4_Error`](#sel4-error) | mod |  |
| [`priorityConstants`](#priorityconstants) | mod |  |
| [`seL4_MsgLimits`](#sel4-msglimits) | mod |  |
| [`seL4_LookupFailureType`](#sel4-lookupfailuretype) | mod |  |
| [`seL4_TCBFlag`](#sel4-tcbflag) | mod |  |
| [`seL4_CapFault_Msg`](#sel4-capfault-msg) | mod |  |
| [`seL4_RootCNodeCapSlots`](#sel4-rootcnodecapslots) | mod |  |
| [`seL4_BootInfoID`](#sel4-bootinfoid) | mod |  |
| [`invocation_label`](#invocation-label) | mod |  |
| [`calls`](#calls) | mod |  |
| [`helpers`](#helpers) | mod |  |
| [`syscall_id`](#syscall-id) | mod |  |
| [`seL4_Fault_NullFault`](#sel4-fault-nullfault) | struct |  |
| [`seL4_Fault_NullFault_Unpacked`](#sel4-fault-nullfault-unpacked) | struct |  |
| [`seL4_Fault_CapFault`](#sel4-fault-capfault) | struct |  |
| [`seL4_Fault_CapFault_Unpacked`](#sel4-fault-capfault-unpacked) | struct |  |
| [`seL4_Fault_UnknownSyscall`](#sel4-fault-unknownsyscall) | struct |  |
| [`seL4_Fault_UnknownSyscall_Unpacked`](#sel4-fault-unknownsyscall-unpacked) | struct |  |
| [`seL4_Fault_UserException`](#sel4-fault-userexception) | struct |  |
| [`seL4_Fault_UserException_Unpacked`](#sel4-fault-userexception-unpacked) | struct |  |
| [`seL4_Fault_VMFault`](#sel4-fault-vmfault) | struct |  |
| [`seL4_Fault_VMFault_Unpacked`](#sel4-fault-vmfault-unpacked) | struct |  |
| [`seL4_Fault`](#sel4-fault) | struct |  |
| [`seL4_MessageInfo`](#sel4-messageinfo) | struct |  |
| [`seL4_MessageInfo_Unpacked`](#sel4-messageinfo-unpacked) | struct |  |
| [`seL4_CapRights`](#sel4-caprights) | struct |  |
| [`seL4_CapRights_Unpacked`](#sel4-caprights-unpacked) | struct |  |
| [`seL4_CNode_CapData`](#sel4-cnode-capdata) | struct |  |
| [`seL4_CNode_CapData_Unpacked`](#sel4-cnode-capdata-unpacked) | struct |  |
| [`seL4_UserContext_`](#sel4-usercontext) | struct |  |
| [`seL4_VCPUContext_`](#sel4-vcpucontext) | struct |  |
| [`seL4_IPCBuffer_`](#sel4-ipcbuffer) | struct |  |
| [`seL4_X86_VCPU_ReadMSR`](#sel4-x86-vcpu-readmsr) | struct |  |
| [`seL4_X86_VCPU_WriteMSR`](#sel4-x86-vcpu-writemsr) | struct |  |
| [`seL4_X86_PageDirectory_GetStatusBits`](#sel4-x86-pagedirectory-getstatusbits) | struct |  |
| [`seL4_X86_VCPU_ReadVMCS`](#sel4-x86-vcpu-readvmcs) | struct |  |
| [`seL4_X86_VCPU_WriteVMCS`](#sel4-x86-vcpu-writevmcs) | struct |  |
| [`seL4_TCB_GetBreakpoint`](#sel4-tcb-getbreakpoint) | struct |  |
| [`seL4_TCB_ConfigureSingleStepping`](#sel4-tcb-configuresinglestepping) | struct |  |
| [`seL4_SchedContext_Consumed`](#sel4-schedcontext-consumed) | struct |  |
| [`seL4_SchedContext_YieldTo`](#sel4-schedcontext-yieldto) | struct |  |
| [`seL4_SlotRegion`](#sel4-slotregion) | struct |  |
| [`seL4_UntypedDesc`](#sel4-untypeddesc) | struct |  |
| [`seL4_BootInfo`](#sel4-bootinfo) | struct |  |
| [`seL4_BootInfoHeader`](#sel4-bootinfoheader) | struct |  |
| [`seL4_TCB_SetFlags_ret`](#sel4-tcb-setflags-ret) | struct |  |
| [`seL4_X86_Page_GetAddress_ret`](#sel4-x86-page-getaddress-ret) | struct |  |
| [`seL4_X86_IOPort_In8_ret`](#sel4-x86-ioport-in8-ret) | struct |  |
| [`seL4_X86_IOPort_In16_ret`](#sel4-x86-ioport-in16-ret) | struct |  |
| [`seL4_X86_IOPort_In32_ret`](#sel4-x86-ioport-in32-ret) | struct |  |
| [`seL4_Fault_Splayed`](#sel4-fault-splayed) | enum |  |
| [`ReplyAuthority`](#replyauthority) | type |  |
| [`WaitMessageInfo`](#waitmessageinfo) | type |  |
| [`SeL4Bitfield`](#sel4bitfield) | type |  |
| [`seL4_MessageInfo_t`](#sel4-messageinfo-t) | type |  |
| [`seL4_CapRights_t`](#sel4-caprights-t) | type |  |
| [`seL4_CNode_CapData_t`](#sel4-cnode-capdata-t) | type |  |
| [`seL4_Int8`](#sel4-int8) | type |  |
| [`seL4_Uint8`](#sel4-uint8) | type |  |
| [`seL4_Int16`](#sel4-int16) | type |  |
| [`seL4_Uint16`](#sel4-uint16) | type |  |
| [`seL4_Int32`](#sel4-int32) | type |  |
| [`seL4_Uint32`](#sel4-uint32) | type |  |
| [`seL4_Int64`](#sel4-int64) | type |  |
| [`seL4_Uint64`](#sel4-uint64) | type |  |
| [`seL4_Bool`](#sel4-bool) | type |  |
| [`seL4_Word`](#sel4-word) | type |  |
| [`seL4_CPtr`](#sel4-cptr) | type |  |
| [`seL4_X64_PML4`](#sel4-x64-pml4) | type |  |
| [`seL4_UserContext`](#sel4-usercontext) | type |  |
| [`seL4_X86_ASIDControl`](#sel4-x86-asidcontrol) | type |  |
| [`seL4_X86_ASIDPool`](#sel4-x86-asidpool) | type |  |
| [`seL4_X86_IOSpace`](#sel4-x86-iospace) | type |  |
| [`seL4_X86_IOPort`](#sel4-x86-ioport) | type |  |
| [`seL4_X86_IOPortControl`](#sel4-x86-ioportcontrol) | type |  |
| [`seL4_X86_Page`](#sel4-x86-page) | type |  |
| [`seL4_X86_PDPT`](#sel4-x86-pdpt) | type |  |
| [`seL4_X86_PageDirectory`](#sel4-x86-pagedirectory) | type |  |
| [`seL4_X86_PageTable`](#sel4-x86-pagetable) | type |  |
| [`seL4_X86_IOPageTable`](#sel4-x86-iopagetable) | type |  |
| [`seL4_X86_EPTPML4`](#sel4-x86-eptpml4) | type |  |
| [`seL4_X86_EPTPDPT`](#sel4-x86-eptpdpt) | type |  |
| [`seL4_X86_EPTPD`](#sel4-x86-eptpd) | type |  |
| [`seL4_X86_EPTPT`](#sel4-x86-eptpt) | type |  |
| [`seL4_X86_VCPU`](#sel4-x86-vcpu) | type |  |
| [`seL4_VCPUContext`](#sel4-vcpucontext) | type |  |
| [`seL4_IPCBuffer`](#sel4-ipcbuffer) | type |  |
| [`seL4_NodeId`](#sel4-nodeid) | type |  |
| [`seL4_PAddr`](#sel4-paddr) | type |  |
| [`seL4_Domain`](#sel4-domain) | type |  |
| [`seL4_CNode`](#sel4-cnode) | type |  |
| [`seL4_IRQHandler`](#sel4-irqhandler) | type |  |
| [`seL4_IRQControl`](#sel4-irqcontrol) | type |  |
| [`seL4_TCB`](#sel4-tcb) | type |  |
| [`seL4_Untyped`](#sel4-untyped) | type |  |
| [`seL4_DomainSet`](#sel4-domainset) | type |  |
| [`seL4_SchedContext`](#sel4-schedcontext) | type |  |
| [`seL4_SchedControl`](#sel4-schedcontrol) | type |  |
| [`seL4_Time`](#sel4-time) | type |  |
| [`seL4_SlotPos`](#sel4-slotpos) | type |  |
| [`seL4_True`](#sel4-true) | const |  |
| [`seL4_False`](#sel4-false) | const |  |
| [`TLS_GDT_ENTRY`](#tls-gdt-entry) | const |  |
| [`TLS_GDT_SELECTOR`](#tls-gdt-selector) | const |  |
| [`IPCBUF_GDT_ENTRY`](#ipcbuf-gdt-entry) | const |  |
| [`IPCBUF_GDT_SELECTOR`](#ipcbuf-gdt-selector) | const |  |
| [`seL4_DataFault`](#sel4-datafault) | const |  |
| [`seL4_InstructionFault`](#sel4-instructionfault) | const |  |
| [`seL4_WordBits`](#sel4-wordbits) | const |  |
| [`seL4_WordSizeBits`](#sel4-wordsizebits) | const |  |
| [`seL4_PageBits`](#sel4-pagebits) | const |  |
| [`seL4_SlotBits`](#sel4-slotbits) | const |  |
| [`seL4_TCBBits`](#sel4-tcbbits) | const |  |
| [`seL4_EndpointBits`](#sel4-endpointbits) | const |  |
| [`seL4_NotificationBits`](#sel4-notificationbits) | const |  |
| [`seL4_PageTableBits`](#sel4-pagetablebits) | const |  |
| [`seL4_PageTableEntryBits`](#sel4-pagetableentrybits) | const |  |
| [`seL4_PageTableIndexBits`](#sel4-pagetableindexbits) | const |  |
| [`seL4_PageDirBits`](#sel4-pagedirbits) | const |  |
| [`seL4_PageDirEntryBits`](#sel4-pagedirentrybits) | const |  |
| [`seL4_PageDirIndexBits`](#sel4-pagedirindexbits) | const |  |
| [`seL4_PDPTBits`](#sel4-pdptbits) | const |  |
| [`seL4_PDPTEntryBits`](#sel4-pdptentrybits) | const |  |
| [`seL4_PDPTIndexBits`](#sel4-pdptindexbits) | const |  |
| [`seL4_PML4Bits`](#sel4-pml4bits) | const |  |
| [`seL4_PML4EntryBits`](#sel4-pml4entrybits) | const |  |
| [`seL4_PML4IndexBits`](#sel4-pml4indexbits) | const |  |
| [`seL4_VSpaceBits`](#sel4-vspacebits) | const |  |
| [`seL4_IOPageTableBits`](#sel4-iopagetablebits) | const |  |
| [`seL4_LargePageBits`](#sel4-largepagebits) | const |  |
| [`seL4_HugePageBits`](#sel4-hugepagebits) | const |  |
| [`seL4_NumASIDPoolsBits`](#sel4-numasidpoolsbits) | const |  |
| [`seL4_ASIDPoolBits`](#sel4-asidpoolbits) | const |  |
| [`seL4_ASIDPoolIndexBits`](#sel4-asidpoolindexbits) | const |  |
| [`seL4_MinUntypedBits`](#sel4-minuntypedbits) | const |  |
| [`seL4_MaxUntypedBits`](#sel4-maxuntypedbits) | const |  |
| [`seL4_FastMessageRegisters`](#sel4-fastmessageregisters) | const |  |
| [`seL4_IPCBufferSizeBits`](#sel4-ipcbuffersizebits) | const |  |
| [`seL4_UserTop`](#sel4-usertop) | const |  |
| [`seL4_X86_VCPUObject`](#sel4-x86-vcpuobject) | const |  |
| [`seL4_X86_EPTPML4Object`](#sel4-x86-eptpml4object) | const |  |
| [`seL4_X86_EPTPDPTObject`](#sel4-x86-eptpdptobject) | const |  |
| [`seL4_X86_EPTPDObject`](#sel4-x86-eptpdobject) | const |  |
| [`seL4_X86_EPTPTObject`](#sel4-x86-eptptobject) | const |  |
| [`seL4_CapRightsBits`](#sel4-caprightsbits) | const |  |
| [`seL4_GuardSizeBits`](#sel4-guardsizebits) | const |  |
| [`seL4_GuardBits`](#sel4-guardbits) | const |  |
| [`seL4_BadgeBits`](#sel4-badgebits) | const |  |
| [`seL4_UntypedRetypeMaxObjects`](#sel4-untypedretypemaxobjects) | const |  |
| [`seL4_NilData`](#sel4-nildata) | const |  |
| [`IRQ_OFFSET`](#irq-offset) | const |  |
| [`VECTOR_MIN`](#vector-min) | const |  |
| [`VECTOR_MAX`](#vector-max) | const |  |
| [`MSI_MIN`](#msi-min) | const |  |
| [`MSI_MAX`](#msi-max) | const |  |
| [`seL4_VCPUBits`](#sel4-vcpubits) | const |  |
| [`seL4_X86_VCPUBits`](#sel4-x86-vcpubits) | const |  |
| [`seL4_X86_EPTPML4EntryBits`](#sel4-x86-eptpml4entrybits) | const |  |
| [`seL4_X86_EPTPML4IndexBits`](#sel4-x86-eptpml4indexbits) | const |  |
| [`seL4_X86_EPTPML4Bits`](#sel4-x86-eptpml4bits) | const |  |
| [`seL4_X86_EPTPDPTEntryBits`](#sel4-x86-eptpdptentrybits) | const |  |
| [`seL4_X86_EPTPDPTIndexBits`](#sel4-x86-eptpdptindexbits) | const |  |
| [`seL4_X86_EPTPDPTBits`](#sel4-x86-eptpdptbits) | const |  |
| [`seL4_X86_EPTPDEntryBits`](#sel4-x86-eptpdentrybits) | const |  |
| [`seL4_X86_EPTPDIndexBits`](#sel4-x86-eptpdindexbits) | const |  |
| [`seL4_X86_EPTPDBits`](#sel4-x86-eptpdbits) | const |  |
| [`seL4_X86_EPTPTEntryBits`](#sel4-x86-eptptentrybits) | const |  |
| [`seL4_X86_EPTPTIndexBits`](#sel4-x86-eptptindexbits) | const |  |
| [`seL4_X86_EPTPTBits`](#sel4-x86-eptptbits) | const |  |
| [`seL4_BootInfoFrameBits`](#sel4-bootinfoframebits) | const |  |
| [`SEL4_MAPPING_LOOKUP_LEVEL`](#sel4-mapping-lookup-level) | const |  |
| [`SEL4_MAPPING_LOOKUP_NO_PT`](#sel4-mapping-lookup-no-pt) | const |  |
| [`SEL4_MAPPING_LOOKUP_NO_PD`](#sel4-mapping-lookup-no-pd) | const |  |
| [`SEL4_MAPPING_LOOKUP_NO_PDPT`](#sel4-mapping-lookup-no-pdpt) | const |  |
| [`SEL4_MAPPING_LOOKUP_NO_EPTPDPT`](#sel4-mapping-lookup-no-eptpdpt) | const |  |
| [`SEL4_MAPPING_LOOKUP_NO_EPTPD`](#sel4-mapping-lookup-no-eptpd) | const |  |
| [`SEL4_MAPPING_LOOKUP_NO_EPTPT`](#sel4-mapping-lookup-no-eptpt) | const |  |

## Modules

- [`bf`](bf/index.md)
- [`c`](c/index.md)
- [`fault`](fault/index.md)
- [`invocations`](invocations/index.md)
- [`ipc_buffer`](ipc_buffer/index.md)
- [`syscalls`](syscalls/index.md)
- [`seL4_Fault_tag`](seL4_Fault_tag/index.md)
- [`seL4_VMFault_Msg`](seL4_VMFault_Msg/index.md)
- [`seL4_UnknownSyscall_Msg`](seL4_UnknownSyscall_Msg/index.md)
- [`seL4_UserException_Msg`](seL4_UserException_Msg/index.md)
- [`seL4_X86_VMAttributes`](seL4_X86_VMAttributes/index.md)
- [`seL4_X86_EPT_VMAttributes`](seL4_X86_EPT_VMAttributes/index.md)
- [`api_object`](api_object/index.md)
- [`_mode_object`](_mode_object/index.md)
- [`_object`](_object/index.md)
- [`seL4_Error`](seL4_Error/index.md)
- [`priorityConstants`](priorityConstants/index.md)
- [`seL4_MsgLimits`](seL4_MsgLimits/index.md)
- [`seL4_LookupFailureType`](seL4_LookupFailureType/index.md)
- [`seL4_TCBFlag`](seL4_TCBFlag/index.md)
- [`seL4_CapFault_Msg`](seL4_CapFault_Msg/index.md)
- [`seL4_RootCNodeCapSlots`](seL4_RootCNodeCapSlots/index.md)
- [`seL4_BootInfoID`](seL4_BootInfoID/index.md)
- [`invocation_label`](invocation_label/index.md)
- [`calls`](calls/index.md)
- [`helpers`](helpers/index.md)
- [`syscall_id`](syscall_id/index.md)

## Structs

### `seL4_Fault_NullFault`

```rust
struct seL4_Fault_NullFault(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-nullfault-new"></span>`fn new() -> Self`

- <span id="sel4-fault-nullfault-unpack"></span>`fn unpack(&self) -> seL4_Fault_NullFault_Unpacked` — [`seL4_Fault_NullFault_Unpacked`](#sel4-fault-nullfault-unpacked)

- <span id="sel4-fault-nullfault-get-sel4-faulttype"></span>`fn get_seL4_FaultType(&self) -> u64`

- <span id="sel4-fault-nullfault-set-sel4-faulttype"></span>`fn set_seL4_FaultType(&mut self, seL4_FaultType: u64)`

- <span id="sel4-fault-nullfault-width-of-sel4-faulttype"></span>`const fn width_of_seL4_FaultType() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_Fault_NullFault`

- <span id="sel4-fault-nullfault-clone"></span>`fn clone(&self) -> seL4_Fault_NullFault` — [`seL4_Fault_NullFault`](#sel4-fault-nullfault)

##### `impl Debug for seL4_Fault_NullFault`

- <span id="sel4-fault-nullfault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_NullFault`

##### `impl PartialEq for seL4_Fault_NullFault`

- <span id="sel4-fault-nullfault-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_NullFault) -> bool` — [`seL4_Fault_NullFault`](#sel4-fault-nullfault)

##### `impl StructuralPartialEq for seL4_Fault_NullFault`

### `seL4_Fault_NullFault_Unpacked`

```rust
struct seL4_Fault_NullFault_Unpacked {
}
```

#### Implementations

- <span id="sel4-fault-nullfault-unpacked-pack"></span>`fn pack(self) -> seL4_Fault_NullFault` — [`seL4_Fault_NullFault`](#sel4-fault-nullfault)

#### Trait Implementations

##### `impl Clone for seL4_Fault_NullFault_Unpacked`

- <span id="sel4-fault-nullfault-unpacked-clone"></span>`fn clone(&self) -> seL4_Fault_NullFault_Unpacked` — [`seL4_Fault_NullFault_Unpacked`](#sel4-fault-nullfault-unpacked)

##### `impl Debug for seL4_Fault_NullFault_Unpacked`

- <span id="sel4-fault-nullfault-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_NullFault_Unpacked`

##### `impl PartialEq for seL4_Fault_NullFault_Unpacked`

- <span id="sel4-fault-nullfault-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_NullFault_Unpacked) -> bool` — [`seL4_Fault_NullFault_Unpacked`](#sel4-fault-nullfault-unpacked)

##### `impl StructuralPartialEq for seL4_Fault_NullFault_Unpacked`

### `seL4_Fault_CapFault`

```rust
struct seL4_Fault_CapFault(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-capfault-new"></span>`fn new(IP: u64, Addr: u64, InRecvPhase: u64, LookupFailureType: u64, MR4: u64, MR5: u64, MR6: u64) -> Self`

- <span id="sel4-fault-capfault-unpack"></span>`fn unpack(&self) -> seL4_Fault_CapFault_Unpacked` — [`seL4_Fault_CapFault_Unpacked`](#sel4-fault-capfault-unpacked)

- <span id="sel4-fault-capfault-get-ip"></span>`fn get_IP(&self) -> u64`

- <span id="sel4-fault-capfault-set-ip"></span>`fn set_IP(&mut self, IP: u64)`

- <span id="sel4-fault-capfault-width-of-ip"></span>`const fn width_of_IP() -> usize`

- <span id="sel4-fault-capfault-get-addr"></span>`fn get_Addr(&self) -> u64`

- <span id="sel4-fault-capfault-set-addr"></span>`fn set_Addr(&mut self, Addr: u64)`

- <span id="sel4-fault-capfault-width-of-addr"></span>`const fn width_of_Addr() -> usize`

- <span id="sel4-fault-capfault-get-inrecvphase"></span>`fn get_InRecvPhase(&self) -> u64`

- <span id="sel4-fault-capfault-set-inrecvphase"></span>`fn set_InRecvPhase(&mut self, InRecvPhase: u64)`

- <span id="sel4-fault-capfault-width-of-inrecvphase"></span>`const fn width_of_InRecvPhase() -> usize`

- <span id="sel4-fault-capfault-get-lookupfailuretype"></span>`fn get_LookupFailureType(&self) -> u64`

- <span id="sel4-fault-capfault-set-lookupfailuretype"></span>`fn set_LookupFailureType(&mut self, LookupFailureType: u64)`

- <span id="sel4-fault-capfault-width-of-lookupfailuretype"></span>`const fn width_of_LookupFailureType() -> usize`

- <span id="sel4-fault-capfault-get-mr4"></span>`fn get_MR4(&self) -> u64`

- <span id="sel4-fault-capfault-set-mr4"></span>`fn set_MR4(&mut self, MR4: u64)`

- <span id="sel4-fault-capfault-width-of-mr4"></span>`const fn width_of_MR4() -> usize`

- <span id="sel4-fault-capfault-get-mr5"></span>`fn get_MR5(&self) -> u64`

- <span id="sel4-fault-capfault-set-mr5"></span>`fn set_MR5(&mut self, MR5: u64)`

- <span id="sel4-fault-capfault-width-of-mr5"></span>`const fn width_of_MR5() -> usize`

- <span id="sel4-fault-capfault-get-mr6"></span>`fn get_MR6(&self) -> u64`

- <span id="sel4-fault-capfault-set-mr6"></span>`fn set_MR6(&mut self, MR6: u64)`

- <span id="sel4-fault-capfault-width-of-mr6"></span>`const fn width_of_MR6() -> usize`

- <span id="sel4-fault-capfault-get-sel4-faulttype"></span>`fn get_seL4_FaultType(&self) -> u64`

- <span id="sel4-fault-capfault-set-sel4-faulttype"></span>`fn set_seL4_FaultType(&mut self, seL4_FaultType: u64)`

- <span id="sel4-fault-capfault-width-of-sel4-faulttype"></span>`const fn width_of_seL4_FaultType() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_Fault_CapFault`

- <span id="sel4-fault-capfault-clone"></span>`fn clone(&self) -> seL4_Fault_CapFault` — [`seL4_Fault_CapFault`](#sel4-fault-capfault)

##### `impl Debug for seL4_Fault_CapFault`

- <span id="sel4-fault-capfault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_CapFault`

##### `impl PartialEq for seL4_Fault_CapFault`

- <span id="sel4-fault-capfault-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_CapFault) -> bool` — [`seL4_Fault_CapFault`](#sel4-fault-capfault)

##### `impl StructuralPartialEq for seL4_Fault_CapFault`

### `seL4_Fault_CapFault_Unpacked`

```rust
struct seL4_Fault_CapFault_Unpacked {
    pub IP: u64,
    pub Addr: u64,
    pub InRecvPhase: u64,
    pub LookupFailureType: u64,
    pub MR4: u64,
    pub MR5: u64,
    pub MR6: u64,
}
```

#### Implementations

- <span id="sel4-fault-capfault-unpacked-pack"></span>`fn pack(self) -> seL4_Fault_CapFault` — [`seL4_Fault_CapFault`](#sel4-fault-capfault)

#### Trait Implementations

##### `impl Clone for seL4_Fault_CapFault_Unpacked`

- <span id="sel4-fault-capfault-unpacked-clone"></span>`fn clone(&self) -> seL4_Fault_CapFault_Unpacked` — [`seL4_Fault_CapFault_Unpacked`](#sel4-fault-capfault-unpacked)

##### `impl Debug for seL4_Fault_CapFault_Unpacked`

- <span id="sel4-fault-capfault-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_CapFault_Unpacked`

##### `impl PartialEq for seL4_Fault_CapFault_Unpacked`

- <span id="sel4-fault-capfault-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_CapFault_Unpacked) -> bool` — [`seL4_Fault_CapFault_Unpacked`](#sel4-fault-capfault-unpacked)

##### `impl StructuralPartialEq for seL4_Fault_CapFault_Unpacked`

### `seL4_Fault_UnknownSyscall`

```rust
struct seL4_Fault_UnknownSyscall(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-unknownsyscall-new"></span>`fn new(RAX: u64, RBX: u64, RCX: u64, RDX: u64, RSI: u64, RDI: u64, RBP: u64, R8: u64, R9: u64, R10: u64, R11: u64, R12: u64, R13: u64, R14: u64, R15: u64, FaultIP: u64, RSP: u64, FLAGS: u64, Syscall: u64) -> Self`

- <span id="sel4-fault-unknownsyscall-unpack"></span>`fn unpack(&self) -> seL4_Fault_UnknownSyscall_Unpacked` — [`seL4_Fault_UnknownSyscall_Unpacked`](#sel4-fault-unknownsyscall-unpacked)

- <span id="sel4-fault-unknownsyscall-get-rax"></span>`fn get_RAX(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rax"></span>`fn set_RAX(&mut self, RAX: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rax"></span>`const fn width_of_RAX() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rbx"></span>`fn get_RBX(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rbx"></span>`fn set_RBX(&mut self, RBX: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rbx"></span>`const fn width_of_RBX() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rcx"></span>`fn get_RCX(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rcx"></span>`fn set_RCX(&mut self, RCX: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rcx"></span>`const fn width_of_RCX() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rdx"></span>`fn get_RDX(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rdx"></span>`fn set_RDX(&mut self, RDX: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rdx"></span>`const fn width_of_RDX() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rsi"></span>`fn get_RSI(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rsi"></span>`fn set_RSI(&mut self, RSI: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rsi"></span>`const fn width_of_RSI() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rdi"></span>`fn get_RDI(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rdi"></span>`fn set_RDI(&mut self, RDI: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rdi"></span>`const fn width_of_RDI() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rbp"></span>`fn get_RBP(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rbp"></span>`fn set_RBP(&mut self, RBP: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rbp"></span>`const fn width_of_RBP() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r8"></span>`fn get_R8(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r8"></span>`fn set_R8(&mut self, R8: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r8"></span>`const fn width_of_R8() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r9"></span>`fn get_R9(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r9"></span>`fn set_R9(&mut self, R9: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r9"></span>`const fn width_of_R9() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r10"></span>`fn get_R10(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r10"></span>`fn set_R10(&mut self, R10: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r10"></span>`const fn width_of_R10() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r11"></span>`fn get_R11(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r11"></span>`fn set_R11(&mut self, R11: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r11"></span>`const fn width_of_R11() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r12"></span>`fn get_R12(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r12"></span>`fn set_R12(&mut self, R12: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r12"></span>`const fn width_of_R12() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r13"></span>`fn get_R13(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r13"></span>`fn set_R13(&mut self, R13: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r13"></span>`const fn width_of_R13() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r14"></span>`fn get_R14(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r14"></span>`fn set_R14(&mut self, R14: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r14"></span>`const fn width_of_R14() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r15"></span>`fn get_R15(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r15"></span>`fn set_R15(&mut self, R15: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r15"></span>`const fn width_of_R15() -> usize`

- <span id="sel4-fault-unknownsyscall-get-faultip"></span>`fn get_FaultIP(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-faultip"></span>`fn set_FaultIP(&mut self, FaultIP: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-faultip"></span>`const fn width_of_FaultIP() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rsp"></span>`fn get_RSP(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rsp"></span>`fn set_RSP(&mut self, RSP: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rsp"></span>`const fn width_of_RSP() -> usize`

- <span id="sel4-fault-unknownsyscall-get-flags"></span>`fn get_FLAGS(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-flags"></span>`fn set_FLAGS(&mut self, FLAGS: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-flags"></span>`const fn width_of_FLAGS() -> usize`

- <span id="sel4-fault-unknownsyscall-get-syscall"></span>`fn get_Syscall(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-syscall"></span>`fn set_Syscall(&mut self, Syscall: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-syscall"></span>`const fn width_of_Syscall() -> usize`

- <span id="sel4-fault-unknownsyscall-get-sel4-faulttype"></span>`fn get_seL4_FaultType(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-sel4-faulttype"></span>`fn set_seL4_FaultType(&mut self, seL4_FaultType: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-sel4-faulttype"></span>`const fn width_of_seL4_FaultType() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_Fault_UnknownSyscall`

- <span id="sel4-fault-unknownsyscall-clone"></span>`fn clone(&self) -> seL4_Fault_UnknownSyscall` — [`seL4_Fault_UnknownSyscall`](#sel4-fault-unknownsyscall)

##### `impl Debug for seL4_Fault_UnknownSyscall`

- <span id="sel4-fault-unknownsyscall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_UnknownSyscall`

##### `impl PartialEq for seL4_Fault_UnknownSyscall`

- <span id="sel4-fault-unknownsyscall-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_UnknownSyscall) -> bool` — [`seL4_Fault_UnknownSyscall`](#sel4-fault-unknownsyscall)

##### `impl StructuralPartialEq for seL4_Fault_UnknownSyscall`

### `seL4_Fault_UnknownSyscall_Unpacked`

```rust
struct seL4_Fault_UnknownSyscall_Unpacked {
    pub RAX: u64,
    pub RBX: u64,
    pub RCX: u64,
    pub RDX: u64,
    pub RSI: u64,
    pub RDI: u64,
    pub RBP: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub FaultIP: u64,
    pub RSP: u64,
    pub FLAGS: u64,
    pub Syscall: u64,
}
```

#### Implementations

- <span id="sel4-fault-unknownsyscall-unpacked-pack"></span>`fn pack(self) -> seL4_Fault_UnknownSyscall` — [`seL4_Fault_UnknownSyscall`](#sel4-fault-unknownsyscall)

#### Trait Implementations

##### `impl Clone for seL4_Fault_UnknownSyscall_Unpacked`

- <span id="sel4-fault-unknownsyscall-unpacked-clone"></span>`fn clone(&self) -> seL4_Fault_UnknownSyscall_Unpacked` — [`seL4_Fault_UnknownSyscall_Unpacked`](#sel4-fault-unknownsyscall-unpacked)

##### `impl Debug for seL4_Fault_UnknownSyscall_Unpacked`

- <span id="sel4-fault-unknownsyscall-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_UnknownSyscall_Unpacked`

##### `impl PartialEq for seL4_Fault_UnknownSyscall_Unpacked`

- <span id="sel4-fault-unknownsyscall-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_UnknownSyscall_Unpacked) -> bool` — [`seL4_Fault_UnknownSyscall_Unpacked`](#sel4-fault-unknownsyscall-unpacked)

##### `impl StructuralPartialEq for seL4_Fault_UnknownSyscall_Unpacked`

### `seL4_Fault_UserException`

```rust
struct seL4_Fault_UserException(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-userexception-new"></span>`fn new(FaultIP: u64, Stack: u64, FLAGS: u64, Number: u64, Code: u64) -> Self`

- <span id="sel4-fault-userexception-unpack"></span>`fn unpack(&self) -> seL4_Fault_UserException_Unpacked` — [`seL4_Fault_UserException_Unpacked`](#sel4-fault-userexception-unpacked)

- <span id="sel4-fault-userexception-get-faultip"></span>`fn get_FaultIP(&self) -> u64`

- <span id="sel4-fault-userexception-set-faultip"></span>`fn set_FaultIP(&mut self, FaultIP: u64)`

- <span id="sel4-fault-userexception-width-of-faultip"></span>`const fn width_of_FaultIP() -> usize`

- <span id="sel4-fault-userexception-get-stack"></span>`fn get_Stack(&self) -> u64`

- <span id="sel4-fault-userexception-set-stack"></span>`fn set_Stack(&mut self, Stack: u64)`

- <span id="sel4-fault-userexception-width-of-stack"></span>`const fn width_of_Stack() -> usize`

- <span id="sel4-fault-userexception-get-flags"></span>`fn get_FLAGS(&self) -> u64`

- <span id="sel4-fault-userexception-set-flags"></span>`fn set_FLAGS(&mut self, FLAGS: u64)`

- <span id="sel4-fault-userexception-width-of-flags"></span>`const fn width_of_FLAGS() -> usize`

- <span id="sel4-fault-userexception-get-number"></span>`fn get_Number(&self) -> u64`

- <span id="sel4-fault-userexception-set-number"></span>`fn set_Number(&mut self, Number: u64)`

- <span id="sel4-fault-userexception-width-of-number"></span>`const fn width_of_Number() -> usize`

- <span id="sel4-fault-userexception-get-code"></span>`fn get_Code(&self) -> u64`

- <span id="sel4-fault-userexception-set-code"></span>`fn set_Code(&mut self, Code: u64)`

- <span id="sel4-fault-userexception-width-of-code"></span>`const fn width_of_Code() -> usize`

- <span id="sel4-fault-userexception-get-sel4-faulttype"></span>`fn get_seL4_FaultType(&self) -> u64`

- <span id="sel4-fault-userexception-set-sel4-faulttype"></span>`fn set_seL4_FaultType(&mut self, seL4_FaultType: u64)`

- <span id="sel4-fault-userexception-width-of-sel4-faulttype"></span>`const fn width_of_seL4_FaultType() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_Fault_UserException`

- <span id="sel4-fault-userexception-clone"></span>`fn clone(&self) -> seL4_Fault_UserException` — [`seL4_Fault_UserException`](#sel4-fault-userexception)

##### `impl Debug for seL4_Fault_UserException`

- <span id="sel4-fault-userexception-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_UserException`

##### `impl PartialEq for seL4_Fault_UserException`

- <span id="sel4-fault-userexception-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_UserException) -> bool` — [`seL4_Fault_UserException`](#sel4-fault-userexception)

##### `impl StructuralPartialEq for seL4_Fault_UserException`

### `seL4_Fault_UserException_Unpacked`

```rust
struct seL4_Fault_UserException_Unpacked {
    pub FaultIP: u64,
    pub Stack: u64,
    pub FLAGS: u64,
    pub Number: u64,
    pub Code: u64,
}
```

#### Implementations

- <span id="sel4-fault-userexception-unpacked-pack"></span>`fn pack(self) -> seL4_Fault_UserException` — [`seL4_Fault_UserException`](#sel4-fault-userexception)

#### Trait Implementations

##### `impl Clone for seL4_Fault_UserException_Unpacked`

- <span id="sel4-fault-userexception-unpacked-clone"></span>`fn clone(&self) -> seL4_Fault_UserException_Unpacked` — [`seL4_Fault_UserException_Unpacked`](#sel4-fault-userexception-unpacked)

##### `impl Debug for seL4_Fault_UserException_Unpacked`

- <span id="sel4-fault-userexception-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_UserException_Unpacked`

##### `impl PartialEq for seL4_Fault_UserException_Unpacked`

- <span id="sel4-fault-userexception-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_UserException_Unpacked) -> bool` — [`seL4_Fault_UserException_Unpacked`](#sel4-fault-userexception-unpacked)

##### `impl StructuralPartialEq for seL4_Fault_UserException_Unpacked`

### `seL4_Fault_VMFault`

```rust
struct seL4_Fault_VMFault(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-vmfault-new"></span>`fn new(IP: u64, Addr: u64, PrefetchFault: u64, FSR: u64) -> Self`

- <span id="sel4-fault-vmfault-unpack"></span>`fn unpack(&self) -> seL4_Fault_VMFault_Unpacked` — [`seL4_Fault_VMFault_Unpacked`](#sel4-fault-vmfault-unpacked)

- <span id="sel4-fault-vmfault-get-ip"></span>`fn get_IP(&self) -> u64`

- <span id="sel4-fault-vmfault-set-ip"></span>`fn set_IP(&mut self, IP: u64)`

- <span id="sel4-fault-vmfault-width-of-ip"></span>`const fn width_of_IP() -> usize`

- <span id="sel4-fault-vmfault-get-addr"></span>`fn get_Addr(&self) -> u64`

- <span id="sel4-fault-vmfault-set-addr"></span>`fn set_Addr(&mut self, Addr: u64)`

- <span id="sel4-fault-vmfault-width-of-addr"></span>`const fn width_of_Addr() -> usize`

- <span id="sel4-fault-vmfault-get-prefetchfault"></span>`fn get_PrefetchFault(&self) -> u64`

- <span id="sel4-fault-vmfault-set-prefetchfault"></span>`fn set_PrefetchFault(&mut self, PrefetchFault: u64)`

- <span id="sel4-fault-vmfault-width-of-prefetchfault"></span>`const fn width_of_PrefetchFault() -> usize`

- <span id="sel4-fault-vmfault-get-fsr"></span>`fn get_FSR(&self) -> u64`

- <span id="sel4-fault-vmfault-set-fsr"></span>`fn set_FSR(&mut self, FSR: u64)`

- <span id="sel4-fault-vmfault-width-of-fsr"></span>`const fn width_of_FSR() -> usize`

- <span id="sel4-fault-vmfault-get-sel4-faulttype"></span>`fn get_seL4_FaultType(&self) -> u64`

- <span id="sel4-fault-vmfault-set-sel4-faulttype"></span>`fn set_seL4_FaultType(&mut self, seL4_FaultType: u64)`

- <span id="sel4-fault-vmfault-width-of-sel4-faulttype"></span>`const fn width_of_seL4_FaultType() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_Fault_VMFault`

- <span id="sel4-fault-vmfault-clone"></span>`fn clone(&self) -> seL4_Fault_VMFault` — [`seL4_Fault_VMFault`](#sel4-fault-vmfault)

##### `impl Debug for seL4_Fault_VMFault`

- <span id="sel4-fault-vmfault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_VMFault`

##### `impl PartialEq for seL4_Fault_VMFault`

- <span id="sel4-fault-vmfault-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_VMFault) -> bool` — [`seL4_Fault_VMFault`](#sel4-fault-vmfault)

##### `impl StructuralPartialEq for seL4_Fault_VMFault`

### `seL4_Fault_VMFault_Unpacked`

```rust
struct seL4_Fault_VMFault_Unpacked {
    pub IP: u64,
    pub Addr: u64,
    pub PrefetchFault: u64,
    pub FSR: u64,
}
```

#### Implementations

- <span id="sel4-fault-vmfault-unpacked-pack"></span>`fn pack(self) -> seL4_Fault_VMFault` — [`seL4_Fault_VMFault`](#sel4-fault-vmfault)

#### Trait Implementations

##### `impl Clone for seL4_Fault_VMFault_Unpacked`

- <span id="sel4-fault-vmfault-unpacked-clone"></span>`fn clone(&self) -> seL4_Fault_VMFault_Unpacked` — [`seL4_Fault_VMFault_Unpacked`](#sel4-fault-vmfault-unpacked)

##### `impl Debug for seL4_Fault_VMFault_Unpacked`

- <span id="sel4-fault-vmfault-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_VMFault_Unpacked`

##### `impl PartialEq for seL4_Fault_VMFault_Unpacked`

- <span id="sel4-fault-vmfault-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_VMFault_Unpacked) -> bool` — [`seL4_Fault_VMFault_Unpacked`](#sel4-fault-vmfault-unpacked)

##### `impl StructuralPartialEq for seL4_Fault_VMFault_Unpacked`

### `seL4_Fault`

```rust
struct seL4_Fault(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-splay"></span>`fn splay(self) -> seL4_Fault_Splayed` — [`seL4_Fault_Splayed`](#sel4-fault-splayed)

- <span id="sel4-fault-get-tag"></span>`fn get_tag(&self) -> u64`

#### Trait Implementations

##### `impl Clone for seL4_Fault`

- <span id="sel4-fault-clone"></span>`fn clone(&self) -> seL4_Fault` — [`seL4_Fault`](#sel4-fault)

##### `impl Debug for seL4_Fault`

- <span id="sel4-fault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault`

##### `impl PartialEq for seL4_Fault`

- <span id="sel4-fault-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault) -> bool` — [`seL4_Fault`](#sel4-fault)

##### `impl StructuralPartialEq for seL4_Fault`

### `seL4_MessageInfo`

```rust
struct seL4_MessageInfo(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-messageinfo-new"></span>`fn new(label: u64, capsUnwrapped: u64, extraCaps: u64, length: u64) -> Self`

- <span id="sel4-messageinfo-unpack"></span>`fn unpack(&self) -> seL4_MessageInfo_Unpacked` — [`seL4_MessageInfo_Unpacked`](#sel4-messageinfo-unpacked)

- <span id="sel4-messageinfo-get-label"></span>`fn get_label(&self) -> u64`

- <span id="sel4-messageinfo-set-label"></span>`fn set_label(&mut self, label: u64)`

- <span id="sel4-messageinfo-width-of-label"></span>`const fn width_of_label() -> usize`

- <span id="sel4-messageinfo-get-capsunwrapped"></span>`fn get_capsUnwrapped(&self) -> u64`

- <span id="sel4-messageinfo-set-capsunwrapped"></span>`fn set_capsUnwrapped(&mut self, capsUnwrapped: u64)`

- <span id="sel4-messageinfo-width-of-capsunwrapped"></span>`const fn width_of_capsUnwrapped() -> usize`

- <span id="sel4-messageinfo-get-extracaps"></span>`fn get_extraCaps(&self) -> u64`

- <span id="sel4-messageinfo-set-extracaps"></span>`fn set_extraCaps(&mut self, extraCaps: u64)`

- <span id="sel4-messageinfo-width-of-extracaps"></span>`const fn width_of_extraCaps() -> usize`

- <span id="sel4-messageinfo-get-length"></span>`fn get_length(&self) -> u64`

- <span id="sel4-messageinfo-set-length"></span>`fn set_length(&mut self, length: u64)`

- <span id="sel4-messageinfo-width-of-length"></span>`const fn width_of_length() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_MessageInfo`

- <span id="sel4-messageinfo-clone"></span>`fn clone(&self) -> seL4_MessageInfo` — [`seL4_MessageInfo`](#sel4-messageinfo)

##### `impl Debug for seL4_MessageInfo`

- <span id="sel4-messageinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_MessageInfo`

##### `impl PartialEq for seL4_MessageInfo`

- <span id="sel4-messageinfo-partialeq-eq"></span>`fn eq(&self, other: &seL4_MessageInfo) -> bool` — [`seL4_MessageInfo`](#sel4-messageinfo)

##### `impl StructuralPartialEq for seL4_MessageInfo`

### `seL4_MessageInfo_Unpacked`

```rust
struct seL4_MessageInfo_Unpacked {
    pub label: u64,
    pub capsUnwrapped: u64,
    pub extraCaps: u64,
    pub length: u64,
}
```

#### Implementations

- <span id="sel4-messageinfo-unpacked-pack"></span>`fn pack(self) -> seL4_MessageInfo` — [`seL4_MessageInfo`](#sel4-messageinfo)

#### Trait Implementations

##### `impl Clone for seL4_MessageInfo_Unpacked`

- <span id="sel4-messageinfo-unpacked-clone"></span>`fn clone(&self) -> seL4_MessageInfo_Unpacked` — [`seL4_MessageInfo_Unpacked`](#sel4-messageinfo-unpacked)

##### `impl Debug for seL4_MessageInfo_Unpacked`

- <span id="sel4-messageinfo-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_MessageInfo_Unpacked`

##### `impl PartialEq for seL4_MessageInfo_Unpacked`

- <span id="sel4-messageinfo-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_MessageInfo_Unpacked) -> bool` — [`seL4_MessageInfo_Unpacked`](#sel4-messageinfo-unpacked)

##### `impl StructuralPartialEq for seL4_MessageInfo_Unpacked`

### `seL4_CapRights`

```rust
struct seL4_CapRights(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-caprights-new"></span>`fn new(capAllowGrantReply: u64, capAllowGrant: u64, capAllowRead: u64, capAllowWrite: u64) -> Self`

- <span id="sel4-caprights-unpack"></span>`fn unpack(&self) -> seL4_CapRights_Unpacked` — [`seL4_CapRights_Unpacked`](#sel4-caprights-unpacked)

- <span id="sel4-caprights-get-capallowgrantreply"></span>`fn get_capAllowGrantReply(&self) -> u64`

- <span id="sel4-caprights-set-capallowgrantreply"></span>`fn set_capAllowGrantReply(&mut self, capAllowGrantReply: u64)`

- <span id="sel4-caprights-width-of-capallowgrantreply"></span>`const fn width_of_capAllowGrantReply() -> usize`

- <span id="sel4-caprights-get-capallowgrant"></span>`fn get_capAllowGrant(&self) -> u64`

- <span id="sel4-caprights-set-capallowgrant"></span>`fn set_capAllowGrant(&mut self, capAllowGrant: u64)`

- <span id="sel4-caprights-width-of-capallowgrant"></span>`const fn width_of_capAllowGrant() -> usize`

- <span id="sel4-caprights-get-capallowread"></span>`fn get_capAllowRead(&self) -> u64`

- <span id="sel4-caprights-set-capallowread"></span>`fn set_capAllowRead(&mut self, capAllowRead: u64)`

- <span id="sel4-caprights-width-of-capallowread"></span>`const fn width_of_capAllowRead() -> usize`

- <span id="sel4-caprights-get-capallowwrite"></span>`fn get_capAllowWrite(&self) -> u64`

- <span id="sel4-caprights-set-capallowwrite"></span>`fn set_capAllowWrite(&mut self, capAllowWrite: u64)`

- <span id="sel4-caprights-width-of-capallowwrite"></span>`const fn width_of_capAllowWrite() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_CapRights`

- <span id="sel4-caprights-clone"></span>`fn clone(&self) -> seL4_CapRights` — [`seL4_CapRights`](#sel4-caprights)

##### `impl Debug for seL4_CapRights`

- <span id="sel4-caprights-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_CapRights`

##### `impl PartialEq for seL4_CapRights`

- <span id="sel4-caprights-partialeq-eq"></span>`fn eq(&self, other: &seL4_CapRights) -> bool` — [`seL4_CapRights`](#sel4-caprights)

##### `impl StructuralPartialEq for seL4_CapRights`

### `seL4_CapRights_Unpacked`

```rust
struct seL4_CapRights_Unpacked {
    pub capAllowGrantReply: u64,
    pub capAllowGrant: u64,
    pub capAllowRead: u64,
    pub capAllowWrite: u64,
}
```

#### Implementations

- <span id="sel4-caprights-unpacked-pack"></span>`fn pack(self) -> seL4_CapRights` — [`seL4_CapRights`](#sel4-caprights)

#### Trait Implementations

##### `impl Clone for seL4_CapRights_Unpacked`

- <span id="sel4-caprights-unpacked-clone"></span>`fn clone(&self) -> seL4_CapRights_Unpacked` — [`seL4_CapRights_Unpacked`](#sel4-caprights-unpacked)

##### `impl Debug for seL4_CapRights_Unpacked`

- <span id="sel4-caprights-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_CapRights_Unpacked`

##### `impl PartialEq for seL4_CapRights_Unpacked`

- <span id="sel4-caprights-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_CapRights_Unpacked) -> bool` — [`seL4_CapRights_Unpacked`](#sel4-caprights-unpacked)

##### `impl StructuralPartialEq for seL4_CapRights_Unpacked`

### `seL4_CNode_CapData`

```rust
struct seL4_CNode_CapData(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-cnode-capdata-new"></span>`fn new(guard: u64, guardSize: u64) -> Self`

- <span id="sel4-cnode-capdata-unpack"></span>`fn unpack(&self) -> seL4_CNode_CapData_Unpacked` — [`seL4_CNode_CapData_Unpacked`](#sel4-cnode-capdata-unpacked)

- <span id="sel4-cnode-capdata-get-guard"></span>`fn get_guard(&self) -> u64`

- <span id="sel4-cnode-capdata-set-guard"></span>`fn set_guard(&mut self, guard: u64)`

- <span id="sel4-cnode-capdata-width-of-guard"></span>`const fn width_of_guard() -> usize`

- <span id="sel4-cnode-capdata-get-guardsize"></span>`fn get_guardSize(&self) -> u64`

- <span id="sel4-cnode-capdata-set-guardsize"></span>`fn set_guardSize(&mut self, guardSize: u64)`

- <span id="sel4-cnode-capdata-width-of-guardsize"></span>`const fn width_of_guardSize() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_CNode_CapData`

- <span id="sel4-cnode-capdata-clone"></span>`fn clone(&self) -> seL4_CNode_CapData` — [`seL4_CNode_CapData`](#sel4-cnode-capdata)

##### `impl Debug for seL4_CNode_CapData`

- <span id="sel4-cnode-capdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_CNode_CapData`

##### `impl PartialEq for seL4_CNode_CapData`

- <span id="sel4-cnode-capdata-partialeq-eq"></span>`fn eq(&self, other: &seL4_CNode_CapData) -> bool` — [`seL4_CNode_CapData`](#sel4-cnode-capdata)

##### `impl StructuralPartialEq for seL4_CNode_CapData`

### `seL4_CNode_CapData_Unpacked`

```rust
struct seL4_CNode_CapData_Unpacked {
    pub guard: u64,
    pub guardSize: u64,
}
```

#### Implementations

- <span id="sel4-cnode-capdata-unpacked-pack"></span>`fn pack(self) -> seL4_CNode_CapData` — [`seL4_CNode_CapData`](#sel4-cnode-capdata)

#### Trait Implementations

##### `impl Clone for seL4_CNode_CapData_Unpacked`

- <span id="sel4-cnode-capdata-unpacked-clone"></span>`fn clone(&self) -> seL4_CNode_CapData_Unpacked` — [`seL4_CNode_CapData_Unpacked`](#sel4-cnode-capdata-unpacked)

##### `impl Debug for seL4_CNode_CapData_Unpacked`

- <span id="sel4-cnode-capdata-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_CNode_CapData_Unpacked`

##### `impl PartialEq for seL4_CNode_CapData_Unpacked`

- <span id="sel4-cnode-capdata-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_CNode_CapData_Unpacked) -> bool` — [`seL4_CNode_CapData_Unpacked`](#sel4-cnode-capdata-unpacked)

##### `impl StructuralPartialEq for seL4_CNode_CapData_Unpacked`

### `seL4_UserContext_`

```rust
struct seL4_UserContext_ {
    pub rip: seL4_Word,
    pub rsp: seL4_Word,
    pub rflags: seL4_Word,
    pub rax: seL4_Word,
    pub rbx: seL4_Word,
    pub rcx: seL4_Word,
    pub rdx: seL4_Word,
    pub rsi: seL4_Word,
    pub rdi: seL4_Word,
    pub rbp: seL4_Word,
    pub r8: seL4_Word,
    pub r9: seL4_Word,
    pub r10: seL4_Word,
    pub r11: seL4_Word,
    pub r12: seL4_Word,
    pub r13: seL4_Word,
    pub r14: seL4_Word,
    pub r15: seL4_Word,
    pub fs_base: seL4_Word,
    pub gs_base: seL4_Word,
}
```

#### Trait Implementations

##### `impl Clone for seL4_UserContext_`

- <span id="sel4-usercontext-clone"></span>`fn clone(&self) -> seL4_UserContext_` — [`seL4_UserContext_`](#sel4-usercontext)

##### `impl Copy for seL4_UserContext_`

##### `impl Debug for seL4_UserContext_`

- <span id="sel4-usercontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_UserContext_`

- <span id="sel4-usercontext-default"></span>`fn default() -> seL4_UserContext_` — [`seL4_UserContext_`](#sel4-usercontext)

##### `impl Eq for seL4_UserContext_`

##### `impl PartialEq for seL4_UserContext_`

- <span id="sel4-usercontext-partialeq-eq"></span>`fn eq(&self, other: &seL4_UserContext_) -> bool` — [`seL4_UserContext_`](#sel4-usercontext)

##### `impl StructuralPartialEq for seL4_UserContext_`

### `seL4_VCPUContext_`

```rust
struct seL4_VCPUContext_ {
    pub eax: seL4_Word,
    pub ebx: seL4_Word,
    pub ecx: seL4_Word,
    pub edx: seL4_Word,
    pub esi: seL4_Word,
    pub edi: seL4_Word,
    pub ebp: seL4_Word,
}
```

#### Trait Implementations

##### `impl Clone for seL4_VCPUContext_`

- <span id="sel4-vcpucontext-clone"></span>`fn clone(&self) -> seL4_VCPUContext_` — [`seL4_VCPUContext_`](#sel4-vcpucontext)

##### `impl Copy for seL4_VCPUContext_`

##### `impl Debug for seL4_VCPUContext_`

- <span id="sel4-vcpucontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_VCPUContext_`

- <span id="sel4-vcpucontext-default"></span>`fn default() -> seL4_VCPUContext_` — [`seL4_VCPUContext_`](#sel4-vcpucontext)

##### `impl Eq for seL4_VCPUContext_`

##### `impl PartialEq for seL4_VCPUContext_`

- <span id="sel4-vcpucontext-partialeq-eq"></span>`fn eq(&self, other: &seL4_VCPUContext_) -> bool` — [`seL4_VCPUContext_`](#sel4-vcpucontext)

##### `impl StructuralPartialEq for seL4_VCPUContext_`

### `seL4_IPCBuffer_`

```rust
struct seL4_IPCBuffer_ {
    pub tag: seL4_MessageInfo_t,
    pub msg: [seL4_Word; 120],
    pub userData: seL4_Word,
    pub caps_or_badges: [seL4_Word; 3],
    pub receiveCNode: seL4_CPtr,
    pub receiveIndex: seL4_CPtr,
    pub receiveDepth: seL4_Word,
}
```

#### Implementations

- <span id="sel4-ipcbuffer-sel4-untyped-retype"></span>`fn seL4_Untyped_Retype(&mut self, service: seL4_Untyped, type: seL4_Word, size_bits: seL4_Word, root: seL4_CNode, node_index: seL4_Word, node_depth: seL4_Word, node_offset: seL4_Word, num_objects: seL4_Word) -> seL4_Error::Type` — [`seL4_Untyped`](#sel4-untyped), [`seL4_Word`](#sel4-word), [`seL4_CNode`](#sel4-cnode), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-readregisters"></span>`fn seL4_TCB_ReadRegisters(&mut self, service: seL4_TCB, suspend_source: seL4_Bool, arch_flags: seL4_Uint8, count: seL4_Word, regs: &mut seL4_UserContext) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_Bool`](#sel4-bool), [`seL4_Uint8`](#sel4-uint8), [`seL4_Word`](#sel4-word), [`seL4_UserContext`](#sel4-usercontext), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-writeregisters"></span>`fn seL4_TCB_WriteRegisters(&mut self, service: seL4_TCB, resume_target: seL4_Bool, arch_flags: seL4_Uint8, count: seL4_Word, regs: &seL4_UserContext) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_Bool`](#sel4-bool), [`seL4_Uint8`](#sel4-uint8), [`seL4_Word`](#sel4-word), [`seL4_UserContext`](#sel4-usercontext), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-copyregisters"></span>`fn seL4_TCB_CopyRegisters(&mut self, service: seL4_TCB, source: seL4_TCB, suspend_source: seL4_Bool, resume_target: seL4_Bool, transfer_frame: seL4_Bool, transfer_integer: seL4_Bool, arch_flags: seL4_Uint8) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_Bool`](#sel4-bool), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-configure"></span>`fn seL4_TCB_Configure(&mut self, service: seL4_TCB, fault_ep: seL4_Word, cspace_root: seL4_CNode, cspace_root_data: seL4_Word, vspace_root: seL4_CPtr, vspace_root_data: seL4_Word, buffer: seL4_Word, bufferFrame: seL4_CPtr) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_Word`](#sel4-word), [`seL4_CNode`](#sel4-cnode), [`seL4_CPtr`](#sel4-cptr), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setpriority"></span>`fn seL4_TCB_SetPriority(&mut self, service: seL4_TCB, authority: seL4_TCB, priority: seL4_Word) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setmcpriority"></span>`fn seL4_TCB_SetMCPriority(&mut self, service: seL4_TCB, authority: seL4_TCB, mcp: seL4_Word) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setschedparams"></span>`fn seL4_TCB_SetSchedParams(&mut self, service: seL4_TCB, authority: seL4_TCB, mcp: seL4_Word, priority: seL4_Word) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setipcbuffer"></span>`fn seL4_TCB_SetIPCBuffer(&mut self, service: seL4_TCB, buffer: seL4_Word, bufferFrame: seL4_CPtr) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_Word`](#sel4-word), [`seL4_CPtr`](#sel4-cptr), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setspace"></span>`fn seL4_TCB_SetSpace(&mut self, service: seL4_TCB, fault_ep: seL4_Word, cspace_root: seL4_CNode, cspace_root_data: seL4_Word, vspace_root: seL4_CPtr, vspace_root_data: seL4_Word) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_Word`](#sel4-word), [`seL4_CNode`](#sel4-cnode), [`seL4_CPtr`](#sel4-cptr), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-suspend"></span>`fn seL4_TCB_Suspend(&mut self, service: seL4_TCB) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-resume"></span>`fn seL4_TCB_Resume(&mut self, service: seL4_TCB) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-bindnotification"></span>`fn seL4_TCB_BindNotification(&mut self, service: seL4_TCB, notification: seL4_CPtr) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_CPtr`](#sel4-cptr), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-unbindnotification"></span>`fn seL4_TCB_UnbindNotification(&mut self, service: seL4_TCB) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-settlsbase"></span>`fn seL4_TCB_SetTLSBase(&mut self, service: seL4_TCB, tls_base: seL4_Word) -> seL4_Error::Type` — [`seL4_TCB`](#sel4-tcb), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setflags"></span>`fn seL4_TCB_SetFlags(&mut self, service: seL4_TCB, clear: seL4_Word, set: seL4_Word) -> seL4_TCB_SetFlags_ret` — [`seL4_TCB`](#sel4-tcb), [`seL4_Word`](#sel4-word), [`seL4_TCB_SetFlags_ret`](#sel4-tcb-setflags-ret)

- <span id="sel4-ipcbuffer-sel4-cnode-revoke"></span>`fn seL4_CNode_Revoke(&mut self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-delete"></span>`fn seL4_CNode_Delete(&mut self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-cancelbadgedsends"></span>`fn seL4_CNode_CancelBadgedSends(&mut self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-copy"></span>`fn seL4_CNode_Copy(&mut self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8, rights: seL4_CapRights_t) -> seL4_Error::Type` — [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`seL4_CapRights_t`](#sel4-caprights-t), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-mint"></span>`fn seL4_CNode_Mint(&mut self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8, rights: seL4_CapRights_t, badge: seL4_Word) -> seL4_Error::Type` — [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`seL4_CapRights_t`](#sel4-caprights-t), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-move"></span>`fn seL4_CNode_Move(&mut self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-mutate"></span>`fn seL4_CNode_Mutate(&mut self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8, badge: seL4_Word) -> seL4_Error::Type` — [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-rotate"></span>`fn seL4_CNode_Rotate(&mut self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, dest_badge: seL4_Word, pivot_root: seL4_CNode, pivot_index: seL4_Word, pivot_depth: seL4_Uint8, pivot_badge: seL4_Word, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-savecaller"></span>`fn seL4_CNode_SaveCaller(&mut self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqcontrol-get"></span>`fn seL4_IRQControl_Get(&mut self, service: seL4_IRQControl, irq: seL4_Word, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_IRQControl`](#sel4-irqcontrol), [`seL4_Word`](#sel4-word), [`seL4_CNode`](#sel4-cnode), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqhandler-ack"></span>`fn seL4_IRQHandler_Ack(&mut self, service: seL4_IRQHandler) -> seL4_Error::Type` — [`seL4_IRQHandler`](#sel4-irqhandler), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqhandler-setnotification"></span>`fn seL4_IRQHandler_SetNotification(&mut self, service: seL4_IRQHandler, notification: seL4_CPtr) -> seL4_Error::Type` — [`seL4_IRQHandler`](#sel4-irqhandler), [`seL4_CPtr`](#sel4-cptr), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqhandler-clear"></span>`fn seL4_IRQHandler_Clear(&mut self, service: seL4_IRQHandler) -> seL4_Error::Type` — [`seL4_IRQHandler`](#sel4-irqhandler), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-domainset-set"></span>`fn seL4_DomainSet_Set(&mut self, service: seL4_DomainSet, domain: seL4_Uint8, thread: seL4_TCB) -> seL4_Error::Type` — [`seL4_DomainSet`](#sel4-domainset), [`seL4_Uint8`](#sel4-uint8), [`seL4_TCB`](#sel4-tcb), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-domainset-scheduleconfigure"></span>`fn seL4_DomainSet_ScheduleConfigure(&mut self, service: seL4_DomainSet, index: seL4_Word, domain: seL4_Uint8, duration: seL4_Time) -> seL4_Error::Type` — [`seL4_DomainSet`](#sel4-domainset), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`seL4_Time`](#sel4-time), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-domainset-schedulesetstart"></span>`fn seL4_DomainSet_ScheduleSetStart(&mut self, service: seL4_DomainSet, index: seL4_Word) -> seL4_Error::Type` — [`seL4_DomainSet`](#sel4-domainset), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pdpt-map"></span>`fn seL4_X86_PDPT_Map(&mut self, service: seL4_X86_PDPT, pml4: seL4_X64_PML4, vaddr: seL4_Word, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type` — [`seL4_X86_PDPT`](#sel4-x86-pdpt), [`seL4_X64_PML4`](#sel4-x64-pml4), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_X86_VMAttributes/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pdpt-unmap"></span>`fn seL4_X86_PDPT_Unmap(&mut self, service: seL4_X86_PDPT) -> seL4_Error::Type` — [`seL4_X86_PDPT`](#sel4-x86-pdpt), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pagedirectory-map"></span>`fn seL4_X86_PageDirectory_Map(&mut self, service: seL4_X86_PageDirectory, vspace: seL4_CPtr, vaddr: seL4_Word, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type` — [`seL4_X86_PageDirectory`](#sel4-x86-pagedirectory), [`seL4_CPtr`](#sel4-cptr), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_X86_VMAttributes/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pagedirectory-unmap"></span>`fn seL4_X86_PageDirectory_Unmap(&mut self, service: seL4_X86_PageDirectory) -> seL4_Error::Type` — [`seL4_X86_PageDirectory`](#sel4-x86-pagedirectory), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pagetable-map"></span>`fn seL4_X86_PageTable_Map(&mut self, service: seL4_X86_PageTable, vspace: seL4_CPtr, vaddr: seL4_Word, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type` — [`seL4_X86_PageTable`](#sel4-x86-pagetable), [`seL4_CPtr`](#sel4-cptr), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_X86_VMAttributes/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pagetable-unmap"></span>`fn seL4_X86_PageTable_Unmap(&mut self, service: seL4_X86_PageTable) -> seL4_Error::Type` — [`seL4_X86_PageTable`](#sel4-x86-pagetable), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-iopagetable-map"></span>`fn seL4_X86_IOPageTable_Map(&mut self, service: seL4_X86_IOPageTable, iospace: seL4_X86_IOSpace, ioaddr: seL4_Word) -> seL4_Error::Type` — [`seL4_X86_IOPageTable`](#sel4-x86-iopagetable), [`seL4_X86_IOSpace`](#sel4-x86-iospace), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-iopagetable-unmap"></span>`fn seL4_X86_IOPageTable_Unmap(&mut self, service: seL4_X86_IOPageTable) -> seL4_Error::Type` — [`seL4_X86_IOPageTable`](#sel4-x86-iopagetable), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-page-map"></span>`fn seL4_X86_Page_Map(&mut self, service: seL4_X86_Page, vspace: seL4_CPtr, vaddr: seL4_Word, rights: seL4_CapRights_t, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type` — [`seL4_X86_Page`](#sel4-x86-page), [`seL4_CPtr`](#sel4-cptr), [`seL4_Word`](#sel4-word), [`seL4_CapRights_t`](#sel4-caprights-t), [`Type`](c/seL4_X86_VMAttributes/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-page-unmap"></span>`fn seL4_X86_Page_Unmap(&mut self, service: seL4_X86_Page) -> seL4_Error::Type` — [`seL4_X86_Page`](#sel4-x86-page), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-page-mapio"></span>`fn seL4_X86_Page_MapIO(&mut self, service: seL4_X86_Page, iospace: seL4_X86_IOSpace, rights: seL4_CapRights_t, ioaddr: seL4_Word) -> seL4_Error::Type` — [`seL4_X86_Page`](#sel4-x86-page), [`seL4_X86_IOSpace`](#sel4-x86-iospace), [`seL4_CapRights_t`](#sel4-caprights-t), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-page-getaddress"></span>`fn seL4_X86_Page_GetAddress(&mut self, service: seL4_X86_Page) -> seL4_X86_Page_GetAddress_ret` — [`seL4_X86_Page`](#sel4-x86-page), [`seL4_X86_Page_GetAddress_ret`](#sel4-x86-page-getaddress-ret)

- <span id="sel4-ipcbuffer-sel4-x86-asidcontrol-makepool"></span>`fn seL4_X86_ASIDControl_MakePool(&mut self, service: seL4_X86_ASIDControl, untyped: seL4_Untyped, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_X86_ASIDControl`](#sel4-x86-asidcontrol), [`seL4_Untyped`](#sel4-untyped), [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-asidpool-assign"></span>`fn seL4_X86_ASIDPool_Assign(&mut self, service: seL4_X86_ASIDPool, vspace: seL4_CPtr) -> seL4_Error::Type` — [`seL4_X86_ASIDPool`](#sel4-x86-asidpool), [`seL4_CPtr`](#sel4-cptr), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-ioportcontrol-issue"></span>`fn seL4_X86_IOPortControl_Issue(&mut self, service: seL4_X86_IOPortControl, first_port: seL4_Word, last_port: seL4_Word, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_X86_IOPortControl`](#sel4-x86-ioportcontrol), [`seL4_Word`](#sel4-word), [`seL4_CNode`](#sel4-cnode), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-in8"></span>`fn seL4_X86_IOPort_In8(&mut self, service: seL4_X86_IOPort, port: seL4_Uint16) -> seL4_X86_IOPort_In8_ret` — [`seL4_X86_IOPort`](#sel4-x86-ioport), [`seL4_Uint16`](#sel4-uint16), [`seL4_X86_IOPort_In8_ret`](#sel4-x86-ioport-in8-ret)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-in16"></span>`fn seL4_X86_IOPort_In16(&mut self, service: seL4_X86_IOPort, port: seL4_Uint16) -> seL4_X86_IOPort_In16_ret` — [`seL4_X86_IOPort`](#sel4-x86-ioport), [`seL4_Uint16`](#sel4-uint16), [`seL4_X86_IOPort_In16_ret`](#sel4-x86-ioport-in16-ret)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-in32"></span>`fn seL4_X86_IOPort_In32(&mut self, service: seL4_X86_IOPort, port: seL4_Uint16) -> seL4_X86_IOPort_In32_ret` — [`seL4_X86_IOPort`](#sel4-x86-ioport), [`seL4_Uint16`](#sel4-uint16), [`seL4_X86_IOPort_In32_ret`](#sel4-x86-ioport-in32-ret)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-out8"></span>`fn seL4_X86_IOPort_Out8(&mut self, service: seL4_X86_IOPort, port: seL4_Word, data: seL4_Word) -> seL4_Error::Type` — [`seL4_X86_IOPort`](#sel4-x86-ioport), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-out16"></span>`fn seL4_X86_IOPort_Out16(&mut self, service: seL4_X86_IOPort, port: seL4_Word, data: seL4_Word) -> seL4_Error::Type` — [`seL4_X86_IOPort`](#sel4-x86-ioport), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-out32"></span>`fn seL4_X86_IOPort_Out32(&mut self, service: seL4_X86_IOPort, port: seL4_Word, data: seL4_Word) -> seL4_Error::Type` — [`seL4_X86_IOPort`](#sel4-x86-ioport), [`seL4_Word`](#sel4-word), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqcontrol-getioapic"></span>`fn seL4_IRQControl_GetIOAPIC(&mut self, service: seL4_IRQControl, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8, ioapic: seL4_Word, pin: seL4_Word, level: seL4_Word, polarity: seL4_Word, vector: seL4_Word) -> seL4_Error::Type` — [`seL4_IRQControl`](#sel4-irqcontrol), [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqcontrol-getmsi"></span>`fn seL4_IRQControl_GetMSI(&mut self, service: seL4_IRQControl, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8, pci_bus: seL4_Word, pci_dev: seL4_Word, pci_func: seL4_Word, handle: seL4_Word, vector: seL4_Word) -> seL4_Error::Type` — [`seL4_IRQControl`](#sel4-irqcontrol), [`seL4_CNode`](#sel4-cnode), [`seL4_Word`](#sel4-word), [`seL4_Uint8`](#sel4-uint8), [`Type`](c/seL4_Error/index.md#type)

#### Trait Implementations

##### `impl Default for seL4_IPCBuffer_`

- <span id="sel4-ipcbuffer-default"></span>`fn default() -> Self`

### `seL4_X86_VCPU_ReadMSR`

```rust
struct seL4_X86_VCPU_ReadMSR {
    pub error: ::core::ffi::c_int,
    pub value: seL4_Word,
}
```

#### Trait Implementations

##### `impl Clone for seL4_X86_VCPU_ReadMSR`

- <span id="sel4-x86-vcpu-readmsr-clone"></span>`fn clone(&self) -> seL4_X86_VCPU_ReadMSR` — [`seL4_X86_VCPU_ReadMSR`](#sel4-x86-vcpu-readmsr)

##### `impl Copy for seL4_X86_VCPU_ReadMSR`

##### `impl Debug for seL4_X86_VCPU_ReadMSR`

- <span id="sel4-x86-vcpu-readmsr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_X86_VCPU_ReadMSR`

- <span id="sel4-x86-vcpu-readmsr-default"></span>`fn default() -> seL4_X86_VCPU_ReadMSR` — [`seL4_X86_VCPU_ReadMSR`](#sel4-x86-vcpu-readmsr)

##### `impl Eq for seL4_X86_VCPU_ReadMSR`

##### `impl PartialEq for seL4_X86_VCPU_ReadMSR`

- <span id="sel4-x86-vcpu-readmsr-partialeq-eq"></span>`fn eq(&self, other: &seL4_X86_VCPU_ReadMSR) -> bool` — [`seL4_X86_VCPU_ReadMSR`](#sel4-x86-vcpu-readmsr)

##### `impl StructuralPartialEq for seL4_X86_VCPU_ReadMSR`

### `seL4_X86_VCPU_WriteMSR`

```rust
struct seL4_X86_VCPU_WriteMSR {
    pub error: ::core::ffi::c_int,
    pub written: seL4_Word,
}
```

#### Trait Implementations

##### `impl Clone for seL4_X86_VCPU_WriteMSR`

- <span id="sel4-x86-vcpu-writemsr-clone"></span>`fn clone(&self) -> seL4_X86_VCPU_WriteMSR` — [`seL4_X86_VCPU_WriteMSR`](#sel4-x86-vcpu-writemsr)

##### `impl Copy for seL4_X86_VCPU_WriteMSR`

##### `impl Debug for seL4_X86_VCPU_WriteMSR`

- <span id="sel4-x86-vcpu-writemsr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_X86_VCPU_WriteMSR`

- <span id="sel4-x86-vcpu-writemsr-default"></span>`fn default() -> seL4_X86_VCPU_WriteMSR` — [`seL4_X86_VCPU_WriteMSR`](#sel4-x86-vcpu-writemsr)

##### `impl Eq for seL4_X86_VCPU_WriteMSR`

##### `impl PartialEq for seL4_X86_VCPU_WriteMSR`

- <span id="sel4-x86-vcpu-writemsr-partialeq-eq"></span>`fn eq(&self, other: &seL4_X86_VCPU_WriteMSR) -> bool` — [`seL4_X86_VCPU_WriteMSR`](#sel4-x86-vcpu-writemsr)

##### `impl StructuralPartialEq for seL4_X86_VCPU_WriteMSR`

### `seL4_X86_PageDirectory_GetStatusBits`

```rust
struct seL4_X86_PageDirectory_GetStatusBits {
    pub error: ::core::ffi::c_int,
    pub accessed: seL4_Word,
    pub dirty: seL4_Word,
}
```

#### Trait Implementations

##### `impl Clone for seL4_X86_PageDirectory_GetStatusBits`

- <span id="sel4-x86-pagedirectory-getstatusbits-clone"></span>`fn clone(&self) -> seL4_X86_PageDirectory_GetStatusBits` — [`seL4_X86_PageDirectory_GetStatusBits`](#sel4-x86-pagedirectory-getstatusbits)

##### `impl Copy for seL4_X86_PageDirectory_GetStatusBits`

##### `impl Debug for seL4_X86_PageDirectory_GetStatusBits`

- <span id="sel4-x86-pagedirectory-getstatusbits-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_X86_PageDirectory_GetStatusBits`

- <span id="sel4-x86-pagedirectory-getstatusbits-default"></span>`fn default() -> seL4_X86_PageDirectory_GetStatusBits` — [`seL4_X86_PageDirectory_GetStatusBits`](#sel4-x86-pagedirectory-getstatusbits)

##### `impl Eq for seL4_X86_PageDirectory_GetStatusBits`

##### `impl PartialEq for seL4_X86_PageDirectory_GetStatusBits`

- <span id="sel4-x86-pagedirectory-getstatusbits-partialeq-eq"></span>`fn eq(&self, other: &seL4_X86_PageDirectory_GetStatusBits) -> bool` — [`seL4_X86_PageDirectory_GetStatusBits`](#sel4-x86-pagedirectory-getstatusbits)

##### `impl StructuralPartialEq for seL4_X86_PageDirectory_GetStatusBits`

### `seL4_X86_VCPU_ReadVMCS`

```rust
struct seL4_X86_VCPU_ReadVMCS {
    pub error: ::core::ffi::c_int,
    pub value: seL4_Word,
}
```

#### Trait Implementations

##### `impl Clone for seL4_X86_VCPU_ReadVMCS`

- <span id="sel4-x86-vcpu-readvmcs-clone"></span>`fn clone(&self) -> seL4_X86_VCPU_ReadVMCS` — [`seL4_X86_VCPU_ReadVMCS`](#sel4-x86-vcpu-readvmcs)

##### `impl Copy for seL4_X86_VCPU_ReadVMCS`

##### `impl Debug for seL4_X86_VCPU_ReadVMCS`

- <span id="sel4-x86-vcpu-readvmcs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_X86_VCPU_ReadVMCS`

- <span id="sel4-x86-vcpu-readvmcs-default"></span>`fn default() -> seL4_X86_VCPU_ReadVMCS` — [`seL4_X86_VCPU_ReadVMCS`](#sel4-x86-vcpu-readvmcs)

##### `impl Eq for seL4_X86_VCPU_ReadVMCS`

##### `impl PartialEq for seL4_X86_VCPU_ReadVMCS`

- <span id="sel4-x86-vcpu-readvmcs-partialeq-eq"></span>`fn eq(&self, other: &seL4_X86_VCPU_ReadVMCS) -> bool` — [`seL4_X86_VCPU_ReadVMCS`](#sel4-x86-vcpu-readvmcs)

##### `impl StructuralPartialEq for seL4_X86_VCPU_ReadVMCS`

### `seL4_X86_VCPU_WriteVMCS`

```rust
struct seL4_X86_VCPU_WriteVMCS {
    pub error: ::core::ffi::c_int,
    pub written: seL4_Word,
}
```

#### Trait Implementations

##### `impl Clone for seL4_X86_VCPU_WriteVMCS`

- <span id="sel4-x86-vcpu-writevmcs-clone"></span>`fn clone(&self) -> seL4_X86_VCPU_WriteVMCS` — [`seL4_X86_VCPU_WriteVMCS`](#sel4-x86-vcpu-writevmcs)

##### `impl Copy for seL4_X86_VCPU_WriteVMCS`

##### `impl Debug for seL4_X86_VCPU_WriteVMCS`

- <span id="sel4-x86-vcpu-writevmcs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_X86_VCPU_WriteVMCS`

- <span id="sel4-x86-vcpu-writevmcs-default"></span>`fn default() -> seL4_X86_VCPU_WriteVMCS` — [`seL4_X86_VCPU_WriteVMCS`](#sel4-x86-vcpu-writevmcs)

##### `impl Eq for seL4_X86_VCPU_WriteVMCS`

##### `impl PartialEq for seL4_X86_VCPU_WriteVMCS`

- <span id="sel4-x86-vcpu-writevmcs-partialeq-eq"></span>`fn eq(&self, other: &seL4_X86_VCPU_WriteVMCS) -> bool` — [`seL4_X86_VCPU_WriteVMCS`](#sel4-x86-vcpu-writevmcs)

##### `impl StructuralPartialEq for seL4_X86_VCPU_WriteVMCS`

### `seL4_TCB_GetBreakpoint`

```rust
struct seL4_TCB_GetBreakpoint {
    pub error: ::core::ffi::c_int,
    pub vaddr: seL4_Word,
    pub type_: seL4_Word,
    pub size: seL4_Word,
    pub rw: seL4_Word,
    pub is_enabled: seL4_Bool,
}
```

#### Trait Implementations

##### `impl Clone for seL4_TCB_GetBreakpoint`

- <span id="sel4-tcb-getbreakpoint-clone"></span>`fn clone(&self) -> seL4_TCB_GetBreakpoint` — [`seL4_TCB_GetBreakpoint`](#sel4-tcb-getbreakpoint)

##### `impl Copy for seL4_TCB_GetBreakpoint`

##### `impl Debug for seL4_TCB_GetBreakpoint`

- <span id="sel4-tcb-getbreakpoint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_TCB_GetBreakpoint`

- <span id="sel4-tcb-getbreakpoint-default"></span>`fn default() -> seL4_TCB_GetBreakpoint` — [`seL4_TCB_GetBreakpoint`](#sel4-tcb-getbreakpoint)

##### `impl Eq for seL4_TCB_GetBreakpoint`

##### `impl PartialEq for seL4_TCB_GetBreakpoint`

- <span id="sel4-tcb-getbreakpoint-partialeq-eq"></span>`fn eq(&self, other: &seL4_TCB_GetBreakpoint) -> bool` — [`seL4_TCB_GetBreakpoint`](#sel4-tcb-getbreakpoint)

##### `impl StructuralPartialEq for seL4_TCB_GetBreakpoint`

### `seL4_TCB_ConfigureSingleStepping`

```rust
struct seL4_TCB_ConfigureSingleStepping {
    pub error: ::core::ffi::c_int,
    pub bp_was_consumed: seL4_Bool,
}
```

#### Trait Implementations

##### `impl Clone for seL4_TCB_ConfigureSingleStepping`

- <span id="sel4-tcb-configuresinglestepping-clone"></span>`fn clone(&self) -> seL4_TCB_ConfigureSingleStepping` — [`seL4_TCB_ConfigureSingleStepping`](#sel4-tcb-configuresinglestepping)

##### `impl Copy for seL4_TCB_ConfigureSingleStepping`

##### `impl Debug for seL4_TCB_ConfigureSingleStepping`

- <span id="sel4-tcb-configuresinglestepping-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_TCB_ConfigureSingleStepping`

- <span id="sel4-tcb-configuresinglestepping-default"></span>`fn default() -> seL4_TCB_ConfigureSingleStepping` — [`seL4_TCB_ConfigureSingleStepping`](#sel4-tcb-configuresinglestepping)

##### `impl Eq for seL4_TCB_ConfigureSingleStepping`

##### `impl PartialEq for seL4_TCB_ConfigureSingleStepping`

- <span id="sel4-tcb-configuresinglestepping-partialeq-eq"></span>`fn eq(&self, other: &seL4_TCB_ConfigureSingleStepping) -> bool` — [`seL4_TCB_ConfigureSingleStepping`](#sel4-tcb-configuresinglestepping)

##### `impl StructuralPartialEq for seL4_TCB_ConfigureSingleStepping`

### `seL4_SchedContext_Consumed`

```rust
struct seL4_SchedContext_Consumed {
    pub error: ::core::ffi::c_int,
    pub consumed: seL4_Time,
}
```

#### Trait Implementations

##### `impl Clone for seL4_SchedContext_Consumed`

- <span id="sel4-schedcontext-consumed-clone"></span>`fn clone(&self) -> seL4_SchedContext_Consumed` — [`seL4_SchedContext_Consumed`](#sel4-schedcontext-consumed)

##### `impl Copy for seL4_SchedContext_Consumed`

##### `impl Debug for seL4_SchedContext_Consumed`

- <span id="sel4-schedcontext-consumed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_SchedContext_Consumed`

- <span id="sel4-schedcontext-consumed-default"></span>`fn default() -> seL4_SchedContext_Consumed` — [`seL4_SchedContext_Consumed`](#sel4-schedcontext-consumed)

##### `impl Eq for seL4_SchedContext_Consumed`

##### `impl PartialEq for seL4_SchedContext_Consumed`

- <span id="sel4-schedcontext-consumed-partialeq-eq"></span>`fn eq(&self, other: &seL4_SchedContext_Consumed) -> bool` — [`seL4_SchedContext_Consumed`](#sel4-schedcontext-consumed)

##### `impl StructuralPartialEq for seL4_SchedContext_Consumed`

### `seL4_SchedContext_YieldTo`

```rust
struct seL4_SchedContext_YieldTo {
    pub error: ::core::ffi::c_int,
    pub consumed: seL4_Time,
}
```

#### Trait Implementations

##### `impl Clone for seL4_SchedContext_YieldTo`

- <span id="sel4-schedcontext-yieldto-clone"></span>`fn clone(&self) -> seL4_SchedContext_YieldTo` — [`seL4_SchedContext_YieldTo`](#sel4-schedcontext-yieldto)

##### `impl Copy for seL4_SchedContext_YieldTo`

##### `impl Debug for seL4_SchedContext_YieldTo`

- <span id="sel4-schedcontext-yieldto-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_SchedContext_YieldTo`

- <span id="sel4-schedcontext-yieldto-default"></span>`fn default() -> seL4_SchedContext_YieldTo` — [`seL4_SchedContext_YieldTo`](#sel4-schedcontext-yieldto)

##### `impl Eq for seL4_SchedContext_YieldTo`

##### `impl PartialEq for seL4_SchedContext_YieldTo`

- <span id="sel4-schedcontext-yieldto-partialeq-eq"></span>`fn eq(&self, other: &seL4_SchedContext_YieldTo) -> bool` — [`seL4_SchedContext_YieldTo`](#sel4-schedcontext-yieldto)

##### `impl StructuralPartialEq for seL4_SchedContext_YieldTo`

### `seL4_SlotRegion`

```rust
struct seL4_SlotRegion {
    pub start: seL4_SlotPos,
    pub end: seL4_SlotPos,
}
```

#### Trait Implementations

##### `impl Clone for seL4_SlotRegion`

- <span id="sel4-slotregion-clone"></span>`fn clone(&self) -> seL4_SlotRegion` — [`seL4_SlotRegion`](#sel4-slotregion)

##### `impl Copy for seL4_SlotRegion`

##### `impl Debug for seL4_SlotRegion`

- <span id="sel4-slotregion-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_SlotRegion`

- <span id="sel4-slotregion-default"></span>`fn default() -> seL4_SlotRegion` — [`seL4_SlotRegion`](#sel4-slotregion)

##### `impl Eq for seL4_SlotRegion`

##### `impl PartialEq for seL4_SlotRegion`

- <span id="sel4-slotregion-partialeq-eq"></span>`fn eq(&self, other: &seL4_SlotRegion) -> bool` — [`seL4_SlotRegion`](#sel4-slotregion)

##### `impl StructuralPartialEq for seL4_SlotRegion`

### `seL4_UntypedDesc`

```rust
struct seL4_UntypedDesc {
    pub paddr: seL4_Word,
    pub sizeBits: seL4_Uint8,
    pub isDevice: seL4_Uint8,
    pub padding: [seL4_Uint8; 6],
}
```

#### Trait Implementations

##### `impl Clone for seL4_UntypedDesc`

- <span id="sel4-untypeddesc-clone"></span>`fn clone(&self) -> seL4_UntypedDesc` — [`seL4_UntypedDesc`](#sel4-untypeddesc)

##### `impl Copy for seL4_UntypedDesc`

##### `impl Debug for seL4_UntypedDesc`

- <span id="sel4-untypeddesc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_UntypedDesc`

- <span id="sel4-untypeddesc-default"></span>`fn default() -> seL4_UntypedDesc` — [`seL4_UntypedDesc`](#sel4-untypeddesc)

##### `impl Eq for seL4_UntypedDesc`

##### `impl PartialEq for seL4_UntypedDesc`

- <span id="sel4-untypeddesc-partialeq-eq"></span>`fn eq(&self, other: &seL4_UntypedDesc) -> bool` — [`seL4_UntypedDesc`](#sel4-untypeddesc)

##### `impl StructuralPartialEq for seL4_UntypedDesc`

### `seL4_BootInfo`

```rust
struct seL4_BootInfo {
    pub extraLen: seL4_Word,
    pub nodeID: seL4_NodeId,
    pub numNodes: seL4_Word,
    pub numIOPTLevels: seL4_Word,
    pub ipcBuffer: *mut seL4_IPCBuffer,
    pub empty: seL4_SlotRegion,
    pub sharedFrames: seL4_SlotRegion,
    pub userImageFrames: seL4_SlotRegion,
    pub userImagePaging: seL4_SlotRegion,
    pub ioSpaceCaps: seL4_SlotRegion,
    pub extraBIPages: seL4_SlotRegion,
    pub initThreadCNodeSizeBits: seL4_Word,
    pub initThreadDomain: seL4_Domain,
    pub untyped: seL4_SlotRegion,
    pub untypedList: [seL4_UntypedDesc; 230],
}
```

#### Trait Implementations

##### `impl Clone for seL4_BootInfo`

- <span id="sel4-bootinfo-clone"></span>`fn clone(&self) -> seL4_BootInfo` — [`seL4_BootInfo`](#sel4-bootinfo)

##### `impl Copy for seL4_BootInfo`

##### `impl Debug for seL4_BootInfo`

- <span id="sel4-bootinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_BootInfo`

- <span id="sel4-bootinfo-default"></span>`fn default() -> Self`

##### `impl Eq for seL4_BootInfo`

##### `impl PartialEq for seL4_BootInfo`

- <span id="sel4-bootinfo-partialeq-eq"></span>`fn eq(&self, other: &seL4_BootInfo) -> bool` — [`seL4_BootInfo`](#sel4-bootinfo)

##### `impl StructuralPartialEq for seL4_BootInfo`

### `seL4_BootInfoHeader`

```rust
struct seL4_BootInfoHeader {
    pub id: seL4_Word,
    pub len: seL4_Word,
}
```

#### Trait Implementations

##### `impl Clone for seL4_BootInfoHeader`

- <span id="sel4-bootinfoheader-clone"></span>`fn clone(&self) -> seL4_BootInfoHeader` — [`seL4_BootInfoHeader`](#sel4-bootinfoheader)

##### `impl Copy for seL4_BootInfoHeader`

##### `impl Debug for seL4_BootInfoHeader`

- <span id="sel4-bootinfoheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_BootInfoHeader`

- <span id="sel4-bootinfoheader-default"></span>`fn default() -> seL4_BootInfoHeader` — [`seL4_BootInfoHeader`](#sel4-bootinfoheader)

##### `impl Eq for seL4_BootInfoHeader`

##### `impl PartialEq for seL4_BootInfoHeader`

- <span id="sel4-bootinfoheader-partialeq-eq"></span>`fn eq(&self, other: &seL4_BootInfoHeader) -> bool` — [`seL4_BootInfoHeader`](#sel4-bootinfoheader)

##### `impl StructuralPartialEq for seL4_BootInfoHeader`

### `seL4_TCB_SetFlags_ret`

```rust
struct seL4_TCB_SetFlags_ret {
    pub error: seL4_Error::Type,
    pub flags: seL4_Word,
}
```

#### Trait Implementations

##### `impl Default for seL4_TCB_SetFlags_ret`

- <span id="sel4-tcb-setflags-ret-default"></span>`fn default() -> seL4_TCB_SetFlags_ret` — [`seL4_TCB_SetFlags_ret`](#sel4-tcb-setflags-ret)

### `seL4_X86_Page_GetAddress_ret`

```rust
struct seL4_X86_Page_GetAddress_ret {
    pub error: seL4_Error::Type,
    pub paddr: seL4_Word,
}
```

#### Trait Implementations

##### `impl Default for seL4_X86_Page_GetAddress_ret`

- <span id="sel4-x86-page-getaddress-ret-default"></span>`fn default() -> seL4_X86_Page_GetAddress_ret` — [`seL4_X86_Page_GetAddress_ret`](#sel4-x86-page-getaddress-ret)

### `seL4_X86_IOPort_In8_ret`

```rust
struct seL4_X86_IOPort_In8_ret {
    pub error: seL4_Error::Type,
    pub result: seL4_Uint8,
}
```

#### Trait Implementations

##### `impl Default for seL4_X86_IOPort_In8_ret`

- <span id="sel4-x86-ioport-in8-ret-default"></span>`fn default() -> seL4_X86_IOPort_In8_ret` — [`seL4_X86_IOPort_In8_ret`](#sel4-x86-ioport-in8-ret)

### `seL4_X86_IOPort_In16_ret`

```rust
struct seL4_X86_IOPort_In16_ret {
    pub error: seL4_Error::Type,
    pub result: seL4_Uint16,
}
```

#### Trait Implementations

##### `impl Default for seL4_X86_IOPort_In16_ret`

- <span id="sel4-x86-ioport-in16-ret-default"></span>`fn default() -> seL4_X86_IOPort_In16_ret` — [`seL4_X86_IOPort_In16_ret`](#sel4-x86-ioport-in16-ret)

### `seL4_X86_IOPort_In32_ret`

```rust
struct seL4_X86_IOPort_In32_ret {
    pub error: seL4_Error::Type,
    pub result: seL4_Uint32,
}
```

#### Trait Implementations

##### `impl Default for seL4_X86_IOPort_In32_ret`

- <span id="sel4-x86-ioport-in32-ret-default"></span>`fn default() -> seL4_X86_IOPort_In32_ret` — [`seL4_X86_IOPort_In32_ret`](#sel4-x86-ioport-in32-ret)

## Enums

### `seL4_Fault_Splayed`

```rust
enum seL4_Fault_Splayed {
    NullFault(seL4_Fault_NullFault),
    CapFault(seL4_Fault_CapFault),
    UnknownSyscall(seL4_Fault_UnknownSyscall),
    UserException(seL4_Fault_UserException),
    VMFault(seL4_Fault_VMFault),
}
```

#### Implementations

- <span id="sel4-fault-splayed-unsplay"></span>`fn unsplay(self) -> seL4_Fault` — [`seL4_Fault`](#sel4-fault)

#### Trait Implementations

##### `impl Clone for seL4_Fault_Splayed`

- <span id="sel4-fault-splayed-clone"></span>`fn clone(&self) -> seL4_Fault_Splayed` — [`seL4_Fault_Splayed`](#sel4-fault-splayed)

##### `impl Debug for seL4_Fault_Splayed`

- <span id="sel4-fault-splayed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_Splayed`

##### `impl PartialEq for seL4_Fault_Splayed`

- <span id="sel4-fault-splayed-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_Splayed) -> bool` — [`seL4_Fault_Splayed`](#sel4-fault-splayed)

##### `impl StructuralPartialEq for seL4_Fault_Splayed`

## Type Aliases

### `ReplyAuthority`

```rust
type ReplyAuthority = ();
```

### `WaitMessageInfo`

```rust
type WaitMessageInfo = ();
```

### `SeL4Bitfield<T, const N: usize>`

```rust
type SeL4Bitfield<T, const N: usize> = sel4_bitfield_ops::Bitfield<[T; N], T>;
```

### `seL4_MessageInfo_t`

```rust
type seL4_MessageInfo_t = seL4_MessageInfo;
```

### `seL4_CapRights_t`

```rust
type seL4_CapRights_t = seL4_CapRights;
```

### `seL4_CNode_CapData_t`

```rust
type seL4_CNode_CapData_t = seL4_CNode_CapData;
```

### `seL4_Int8`

```rust
type seL4_Int8 = ::core::ffi::c_schar;
```

### `seL4_Uint8`

```rust
type seL4_Uint8 = ::core::ffi::c_uchar;
```

### `seL4_Int16`

```rust
type seL4_Int16 = ::core::ffi::c_short;
```

### `seL4_Uint16`

```rust
type seL4_Uint16 = ::core::ffi::c_ushort;
```

### `seL4_Int32`

```rust
type seL4_Int32 = ::core::ffi::c_int;
```

### `seL4_Uint32`

```rust
type seL4_Uint32 = ::core::ffi::c_uint;
```

### `seL4_Int64`

```rust
type seL4_Int64 = ::core::ffi::c_long;
```

### `seL4_Uint64`

```rust
type seL4_Uint64 = ::core::ffi::c_ulong;
```

### `seL4_Bool`

```rust
type seL4_Bool = seL4_Int8;
```

### `seL4_Word`

```rust
type seL4_Word = seL4_Uint64;
```

### `seL4_CPtr`

```rust
type seL4_CPtr = seL4_Word;
```

### `seL4_X64_PML4`

```rust
type seL4_X64_PML4 = seL4_CPtr;
```

### `seL4_UserContext`

```rust
type seL4_UserContext = seL4_UserContext_;
```

### `seL4_X86_ASIDControl`

```rust
type seL4_X86_ASIDControl = seL4_CPtr;
```

### `seL4_X86_ASIDPool`

```rust
type seL4_X86_ASIDPool = seL4_CPtr;
```

### `seL4_X86_IOSpace`

```rust
type seL4_X86_IOSpace = seL4_CPtr;
```

### `seL4_X86_IOPort`

```rust
type seL4_X86_IOPort = seL4_CPtr;
```

### `seL4_X86_IOPortControl`

```rust
type seL4_X86_IOPortControl = seL4_CPtr;
```

### `seL4_X86_Page`

```rust
type seL4_X86_Page = seL4_CPtr;
```

### `seL4_X86_PDPT`

```rust
type seL4_X86_PDPT = seL4_CPtr;
```

### `seL4_X86_PageDirectory`

```rust
type seL4_X86_PageDirectory = seL4_CPtr;
```

### `seL4_X86_PageTable`

```rust
type seL4_X86_PageTable = seL4_CPtr;
```

### `seL4_X86_IOPageTable`

```rust
type seL4_X86_IOPageTable = seL4_CPtr;
```

### `seL4_X86_EPTPML4`

```rust
type seL4_X86_EPTPML4 = seL4_CPtr;
```

### `seL4_X86_EPTPDPT`

```rust
type seL4_X86_EPTPDPT = seL4_CPtr;
```

### `seL4_X86_EPTPD`

```rust
type seL4_X86_EPTPD = seL4_CPtr;
```

### `seL4_X86_EPTPT`

```rust
type seL4_X86_EPTPT = seL4_CPtr;
```

### `seL4_X86_VCPU`

```rust
type seL4_X86_VCPU = seL4_CPtr;
```

### `seL4_VCPUContext`

```rust
type seL4_VCPUContext = seL4_VCPUContext_;
```

### `seL4_IPCBuffer`

```rust
type seL4_IPCBuffer = seL4_IPCBuffer_;
```

### `seL4_NodeId`

```rust
type seL4_NodeId = seL4_Word;
```

### `seL4_PAddr`

```rust
type seL4_PAddr = seL4_Word;
```

### `seL4_Domain`

```rust
type seL4_Domain = seL4_Word;
```

### `seL4_CNode`

```rust
type seL4_CNode = seL4_CPtr;
```

### `seL4_IRQHandler`

```rust
type seL4_IRQHandler = seL4_CPtr;
```

### `seL4_IRQControl`

```rust
type seL4_IRQControl = seL4_CPtr;
```

### `seL4_TCB`

```rust
type seL4_TCB = seL4_CPtr;
```

### `seL4_Untyped`

```rust
type seL4_Untyped = seL4_CPtr;
```

### `seL4_DomainSet`

```rust
type seL4_DomainSet = seL4_CPtr;
```

### `seL4_SchedContext`

```rust
type seL4_SchedContext = seL4_CPtr;
```

### `seL4_SchedControl`

```rust
type seL4_SchedControl = seL4_CPtr;
```

### `seL4_Time`

```rust
type seL4_Time = seL4_Uint64;
```

### `seL4_SlotPos`

```rust
type seL4_SlotPos = seL4_Word;
```

## Constants

### `seL4_True`
```rust
const seL4_True: u32 = 1u32;
```

### `seL4_False`
```rust
const seL4_False: u32 = 0u32;
```

### `TLS_GDT_ENTRY`
```rust
const TLS_GDT_ENTRY: u32 = 7u32;
```

### `TLS_GDT_SELECTOR`
```rust
const TLS_GDT_SELECTOR: u32 = 59u32;
```

### `IPCBUF_GDT_ENTRY`
```rust
const IPCBUF_GDT_ENTRY: u32 = 8u32;
```

### `IPCBUF_GDT_SELECTOR`
```rust
const IPCBUF_GDT_SELECTOR: u32 = 67u32;
```

### `seL4_DataFault`
```rust
const seL4_DataFault: u32 = 0u32;
```

### `seL4_InstructionFault`
```rust
const seL4_InstructionFault: u32 = 1u32;
```

### `seL4_WordBits`
```rust
const seL4_WordBits: u32 = 64u32;
```

### `seL4_WordSizeBits`
```rust
const seL4_WordSizeBits: u32 = 3u32;
```

### `seL4_PageBits`
```rust
const seL4_PageBits: u32 = 12u32;
```

### `seL4_SlotBits`
```rust
const seL4_SlotBits: u32 = 5u32;
```

### `seL4_TCBBits`
```rust
const seL4_TCBBits: u32 = 11u32;
```

### `seL4_EndpointBits`
```rust
const seL4_EndpointBits: u32 = 4u32;
```

### `seL4_NotificationBits`
```rust
const seL4_NotificationBits: u32 = 5u32;
```

### `seL4_PageTableBits`
```rust
const seL4_PageTableBits: u32 = 12u32;
```

### `seL4_PageTableEntryBits`
```rust
const seL4_PageTableEntryBits: u32 = 3u32;
```

### `seL4_PageTableIndexBits`
```rust
const seL4_PageTableIndexBits: u32 = 9u32;
```

### `seL4_PageDirBits`
```rust
const seL4_PageDirBits: u32 = 12u32;
```

### `seL4_PageDirEntryBits`
```rust
const seL4_PageDirEntryBits: u32 = 3u32;
```

### `seL4_PageDirIndexBits`
```rust
const seL4_PageDirIndexBits: u32 = 9u32;
```

### `seL4_PDPTBits`
```rust
const seL4_PDPTBits: u32 = 12u32;
```

### `seL4_PDPTEntryBits`
```rust
const seL4_PDPTEntryBits: u32 = 3u32;
```

### `seL4_PDPTIndexBits`
```rust
const seL4_PDPTIndexBits: u32 = 9u32;
```

### `seL4_PML4Bits`
```rust
const seL4_PML4Bits: u32 = 12u32;
```

### `seL4_PML4EntryBits`
```rust
const seL4_PML4EntryBits: u32 = 3u32;
```

### `seL4_PML4IndexBits`
```rust
const seL4_PML4IndexBits: u32 = 9u32;
```

### `seL4_VSpaceBits`
```rust
const seL4_VSpaceBits: u32 = 12u32;
```

### `seL4_IOPageTableBits`
```rust
const seL4_IOPageTableBits: u32 = 12u32;
```

### `seL4_LargePageBits`
```rust
const seL4_LargePageBits: u32 = 21u32;
```

### `seL4_HugePageBits`
```rust
const seL4_HugePageBits: u32 = 30u32;
```

### `seL4_NumASIDPoolsBits`
```rust
const seL4_NumASIDPoolsBits: u32 = 3u32;
```

### `seL4_ASIDPoolBits`
```rust
const seL4_ASIDPoolBits: u32 = 12u32;
```

### `seL4_ASIDPoolIndexBits`
```rust
const seL4_ASIDPoolIndexBits: u32 = 9u32;
```

### `seL4_MinUntypedBits`
```rust
const seL4_MinUntypedBits: u32 = 4u32;
```

### `seL4_MaxUntypedBits`
```rust
const seL4_MaxUntypedBits: u32 = 47u32;
```

### `seL4_FastMessageRegisters`
```rust
const seL4_FastMessageRegisters: u32 = 4u32;
```

### `seL4_IPCBufferSizeBits`
```rust
const seL4_IPCBufferSizeBits: u32 = 10u32;
```

### `seL4_UserTop`
```rust
const seL4_UserTop: u64 = 140_737_488_351_232u64;
```

### `seL4_X86_VCPUObject`
```rust
const seL4_X86_VCPUObject: u32 = 16_777_214u32;
```

### `seL4_X86_EPTPML4Object`
```rust
const seL4_X86_EPTPML4Object: u32 = 16_777_213u32;
```

### `seL4_X86_EPTPDPTObject`
```rust
const seL4_X86_EPTPDPTObject: u32 = 16_777_212u32;
```

### `seL4_X86_EPTPDObject`
```rust
const seL4_X86_EPTPDObject: u32 = 16_777_211u32;
```

### `seL4_X86_EPTPTObject`
```rust
const seL4_X86_EPTPTObject: u32 = 16_777_210u32;
```

### `seL4_CapRightsBits`
```rust
const seL4_CapRightsBits: u32 = 4u32;
```

### `seL4_GuardSizeBits`
```rust
const seL4_GuardSizeBits: u32 = 6u32;
```

### `seL4_GuardBits`
```rust
const seL4_GuardBits: u32 = 58u32;
```

### `seL4_BadgeBits`
```rust
const seL4_BadgeBits: u32 = 64u32;
```

### `seL4_UntypedRetypeMaxObjects`
```rust
const seL4_UntypedRetypeMaxObjects: u32 = 256u32;
```

### `seL4_NilData`
```rust
const seL4_NilData: u32 = 0u32;
```

### `IRQ_OFFSET`
```rust
const IRQ_OFFSET: u32 = 48u32;
```

### `VECTOR_MIN`
```rust
const VECTOR_MIN: u32 = 0u32;
```

### `VECTOR_MAX`
```rust
const VECTOR_MAX: u32 = 109u32;
```

### `MSI_MIN`
```rust
const MSI_MIN: u32 = 0u32;
```

### `MSI_MAX`
```rust
const MSI_MAX: u32 = 109u32;
```

### `seL4_VCPUBits`
```rust
const seL4_VCPUBits: u32 = 14u32;
```

### `seL4_X86_VCPUBits`
```rust
const seL4_X86_VCPUBits: u32 = 14u32;
```

### `seL4_X86_EPTPML4EntryBits`
```rust
const seL4_X86_EPTPML4EntryBits: u32 = 3u32;
```

### `seL4_X86_EPTPML4IndexBits`
```rust
const seL4_X86_EPTPML4IndexBits: u32 = 9u32;
```

### `seL4_X86_EPTPML4Bits`
```rust
const seL4_X86_EPTPML4Bits: u32 = 12u32;
```

### `seL4_X86_EPTPDPTEntryBits`
```rust
const seL4_X86_EPTPDPTEntryBits: u32 = 3u32;
```

### `seL4_X86_EPTPDPTIndexBits`
```rust
const seL4_X86_EPTPDPTIndexBits: u32 = 9u32;
```

### `seL4_X86_EPTPDPTBits`
```rust
const seL4_X86_EPTPDPTBits: u32 = 12u32;
```

### `seL4_X86_EPTPDEntryBits`
```rust
const seL4_X86_EPTPDEntryBits: u32 = 3u32;
```

### `seL4_X86_EPTPDIndexBits`
```rust
const seL4_X86_EPTPDIndexBits: u32 = 9u32;
```

### `seL4_X86_EPTPDBits`
```rust
const seL4_X86_EPTPDBits: u32 = 12u32;
```

### `seL4_X86_EPTPTEntryBits`
```rust
const seL4_X86_EPTPTEntryBits: u32 = 3u32;
```

### `seL4_X86_EPTPTIndexBits`
```rust
const seL4_X86_EPTPTIndexBits: u32 = 9u32;
```

### `seL4_X86_EPTPTBits`
```rust
const seL4_X86_EPTPTBits: u32 = 12u32;
```

### `seL4_BootInfoFrameBits`
```rust
const seL4_BootInfoFrameBits: u32 = 12u32;
```

### `SEL4_MAPPING_LOOKUP_LEVEL`
```rust
const SEL4_MAPPING_LOOKUP_LEVEL: u32 = 2u32;
```

### `SEL4_MAPPING_LOOKUP_NO_PT`
```rust
const SEL4_MAPPING_LOOKUP_NO_PT: u32 = 21u32;
```

### `SEL4_MAPPING_LOOKUP_NO_PD`
```rust
const SEL4_MAPPING_LOOKUP_NO_PD: u32 = 30u32;
```

### `SEL4_MAPPING_LOOKUP_NO_PDPT`
```rust
const SEL4_MAPPING_LOOKUP_NO_PDPT: u32 = 39u32;
```

### `SEL4_MAPPING_LOOKUP_NO_EPTPDPT`
```rust
const SEL4_MAPPING_LOOKUP_NO_EPTPDPT: u32 = 39u32;
```

### `SEL4_MAPPING_LOOKUP_NO_EPTPD`
```rust
const SEL4_MAPPING_LOOKUP_NO_EPTPD: u32 = 30u32;
```

### `SEL4_MAPPING_LOOKUP_NO_EPTPT`
```rust
const SEL4_MAPPING_LOOKUP_NO_EPTPT: u32 = 21u32;
```

