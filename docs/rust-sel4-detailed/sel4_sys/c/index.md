*[sel4_sys](../index.md) / [c](index.md)*

---

# Module `c`

## Contents

- [Modules](#modules)
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
- [Structs](#structs)
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
- [Type Aliases](#type-aliases)
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
  - [`seL4_ObjectType`](#sel4-objecttype)
  - [`seL4_seL4ArchObjectType`](#sel4-sel4archobjecttype)
  - [`seL4_ArchObjectType`](#sel4-archobjecttype)
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
| [`seL4_ObjectType`](#sel4-objecttype) | type |  |
| [`seL4_seL4ArchObjectType`](#sel4-sel4archobjecttype) | type |  |
| [`seL4_ArchObjectType`](#sel4-archobjecttype) | type |  |
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

## Structs

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

- <span id="sel4-usercontext-clone"></span>`fn clone(&self) -> seL4_UserContext_` — [`seL4_UserContext_`](../index.md#sel4-usercontext)

##### `impl Copy for seL4_UserContext_`

##### `impl Debug for seL4_UserContext_`

- <span id="sel4-usercontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_UserContext_`

- <span id="sel4-usercontext-default"></span>`fn default() -> seL4_UserContext_` — [`seL4_UserContext_`](../index.md#sel4-usercontext)

##### `impl Eq for seL4_UserContext_`

##### `impl PartialEq for seL4_UserContext_`

- <span id="sel4-usercontext-partialeq-eq"></span>`fn eq(&self, other: &seL4_UserContext_) -> bool` — [`seL4_UserContext_`](../index.md#sel4-usercontext)

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

- <span id="sel4-vcpucontext-clone"></span>`fn clone(&self) -> seL4_VCPUContext_` — [`seL4_VCPUContext_`](../index.md#sel4-vcpucontext)

##### `impl Copy for seL4_VCPUContext_`

##### `impl Debug for seL4_VCPUContext_`

- <span id="sel4-vcpucontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_VCPUContext_`

- <span id="sel4-vcpucontext-default"></span>`fn default() -> seL4_VCPUContext_` — [`seL4_VCPUContext_`](../index.md#sel4-vcpucontext)

##### `impl Eq for seL4_VCPUContext_`

##### `impl PartialEq for seL4_VCPUContext_`

- <span id="sel4-vcpucontext-partialeq-eq"></span>`fn eq(&self, other: &seL4_VCPUContext_) -> bool` — [`seL4_VCPUContext_`](../index.md#sel4-vcpucontext)

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

- <span id="sel4-ipcbuffer-sel4-untyped-retype"></span>`fn seL4_Untyped_Retype(&mut self, service: seL4_Untyped, type: seL4_Word, size_bits: seL4_Word, root: seL4_CNode, node_index: seL4_Word, node_depth: seL4_Word, node_offset: seL4_Word, num_objects: seL4_Word) -> seL4_Error::Type` — [`seL4_Untyped`](../index.md#sel4-untyped), [`seL4_Word`](../index.md#sel4-word), [`seL4_CNode`](../index.md#sel4-cnode), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-readregisters"></span>`fn seL4_TCB_ReadRegisters(&mut self, service: seL4_TCB, suspend_source: seL4_Bool, arch_flags: seL4_Uint8, count: seL4_Word, regs: &mut seL4_UserContext) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Bool`](../index.md#sel4-bool), [`seL4_Uint8`](../index.md#sel4-uint8), [`seL4_Word`](../index.md#sel4-word), [`seL4_UserContext`](../index.md#sel4-usercontext), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-writeregisters"></span>`fn seL4_TCB_WriteRegisters(&mut self, service: seL4_TCB, resume_target: seL4_Bool, arch_flags: seL4_Uint8, count: seL4_Word, regs: &seL4_UserContext) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Bool`](../index.md#sel4-bool), [`seL4_Uint8`](../index.md#sel4-uint8), [`seL4_Word`](../index.md#sel4-word), [`seL4_UserContext`](../index.md#sel4-usercontext), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-copyregisters"></span>`fn seL4_TCB_CopyRegisters(&mut self, service: seL4_TCB, source: seL4_TCB, suspend_source: seL4_Bool, resume_target: seL4_Bool, transfer_frame: seL4_Bool, transfer_integer: seL4_Bool, arch_flags: seL4_Uint8) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Bool`](../index.md#sel4-bool), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-configure"></span>`fn seL4_TCB_Configure(&mut self, service: seL4_TCB, fault_ep: seL4_Word, cspace_root: seL4_CNode, cspace_root_data: seL4_Word, vspace_root: seL4_CPtr, vspace_root_data: seL4_Word, buffer: seL4_Word, bufferFrame: seL4_CPtr) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Word`](../index.md#sel4-word), [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_CPtr`](../index.md#sel4-cptr), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setpriority"></span>`fn seL4_TCB_SetPriority(&mut self, service: seL4_TCB, authority: seL4_TCB, priority: seL4_Word) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setmcpriority"></span>`fn seL4_TCB_SetMCPriority(&mut self, service: seL4_TCB, authority: seL4_TCB, mcp: seL4_Word) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setschedparams"></span>`fn seL4_TCB_SetSchedParams(&mut self, service: seL4_TCB, authority: seL4_TCB, mcp: seL4_Word, priority: seL4_Word) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setipcbuffer"></span>`fn seL4_TCB_SetIPCBuffer(&mut self, service: seL4_TCB, buffer: seL4_Word, bufferFrame: seL4_CPtr) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Word`](../index.md#sel4-word), [`seL4_CPtr`](../index.md#sel4-cptr), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setspace"></span>`fn seL4_TCB_SetSpace(&mut self, service: seL4_TCB, fault_ep: seL4_Word, cspace_root: seL4_CNode, cspace_root_data: seL4_Word, vspace_root: seL4_CPtr, vspace_root_data: seL4_Word) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Word`](../index.md#sel4-word), [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_CPtr`](../index.md#sel4-cptr), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-suspend"></span>`fn seL4_TCB_Suspend(&mut self, service: seL4_TCB) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-resume"></span>`fn seL4_TCB_Resume(&mut self, service: seL4_TCB) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-bindnotification"></span>`fn seL4_TCB_BindNotification(&mut self, service: seL4_TCB, notification: seL4_CPtr) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_CPtr`](../index.md#sel4-cptr), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-unbindnotification"></span>`fn seL4_TCB_UnbindNotification(&mut self, service: seL4_TCB) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-settlsbase"></span>`fn seL4_TCB_SetTLSBase(&mut self, service: seL4_TCB, tls_base: seL4_Word) -> seL4_Error::Type` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-tcb-setflags"></span>`fn seL4_TCB_SetFlags(&mut self, service: seL4_TCB, clear: seL4_Word, set: seL4_Word) -> seL4_TCB_SetFlags_ret` — [`seL4_TCB`](../index.md#sel4-tcb), [`seL4_Word`](../index.md#sel4-word), [`seL4_TCB_SetFlags_ret`](../index.md#sel4-tcb-setflags-ret)

- <span id="sel4-ipcbuffer-sel4-cnode-revoke"></span>`fn seL4_CNode_Revoke(&mut self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-delete"></span>`fn seL4_CNode_Delete(&mut self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-cancelbadgedsends"></span>`fn seL4_CNode_CancelBadgedSends(&mut self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-copy"></span>`fn seL4_CNode_Copy(&mut self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8, rights: seL4_CapRights_t) -> seL4_Error::Type` — [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`seL4_CapRights_t`](../index.md#sel4-caprights-t), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-mint"></span>`fn seL4_CNode_Mint(&mut self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8, rights: seL4_CapRights_t, badge: seL4_Word) -> seL4_Error::Type` — [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`seL4_CapRights_t`](../index.md#sel4-caprights-t), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-move"></span>`fn seL4_CNode_Move(&mut self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-mutate"></span>`fn seL4_CNode_Mutate(&mut self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8, badge: seL4_Word) -> seL4_Error::Type` — [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-rotate"></span>`fn seL4_CNode_Rotate(&mut self, service: seL4_CNode, dest_index: seL4_Word, dest_depth: seL4_Uint8, dest_badge: seL4_Word, pivot_root: seL4_CNode, pivot_index: seL4_Word, pivot_depth: seL4_Uint8, pivot_badge: seL4_Word, src_root: seL4_CNode, src_index: seL4_Word, src_depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-cnode-savecaller"></span>`fn seL4_CNode_SaveCaller(&mut self, service: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqcontrol-get"></span>`fn seL4_IRQControl_Get(&mut self, service: seL4_IRQControl, irq: seL4_Word, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_IRQControl`](../index.md#sel4-irqcontrol), [`seL4_Word`](../index.md#sel4-word), [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqhandler-ack"></span>`fn seL4_IRQHandler_Ack(&mut self, service: seL4_IRQHandler) -> seL4_Error::Type` — [`seL4_IRQHandler`](../index.md#sel4-irqhandler), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqhandler-setnotification"></span>`fn seL4_IRQHandler_SetNotification(&mut self, service: seL4_IRQHandler, notification: seL4_CPtr) -> seL4_Error::Type` — [`seL4_IRQHandler`](../index.md#sel4-irqhandler), [`seL4_CPtr`](../index.md#sel4-cptr), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqhandler-clear"></span>`fn seL4_IRQHandler_Clear(&mut self, service: seL4_IRQHandler) -> seL4_Error::Type` — [`seL4_IRQHandler`](../index.md#sel4-irqhandler), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-domainset-set"></span>`fn seL4_DomainSet_Set(&mut self, service: seL4_DomainSet, domain: seL4_Uint8, thread: seL4_TCB) -> seL4_Error::Type` — [`seL4_DomainSet`](../index.md#sel4-domainset), [`seL4_Uint8`](../index.md#sel4-uint8), [`seL4_TCB`](../index.md#sel4-tcb), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-domainset-scheduleconfigure"></span>`fn seL4_DomainSet_ScheduleConfigure(&mut self, service: seL4_DomainSet, index: seL4_Word, domain: seL4_Uint8, duration: seL4_Time) -> seL4_Error::Type` — [`seL4_DomainSet`](../index.md#sel4-domainset), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`seL4_Time`](../index.md#sel4-time), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-domainset-schedulesetstart"></span>`fn seL4_DomainSet_ScheduleSetStart(&mut self, service: seL4_DomainSet, index: seL4_Word) -> seL4_Error::Type` — [`seL4_DomainSet`](../index.md#sel4-domainset), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pdpt-map"></span>`fn seL4_X86_PDPT_Map(&mut self, service: seL4_X86_PDPT, pml4: seL4_X64_PML4, vaddr: seL4_Word, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type` — [`seL4_X86_PDPT`](../index.md#sel4-x86-pdpt), [`seL4_X64_PML4`](../index.md#sel4-x64-pml4), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_X86_VMAttributes/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pdpt-unmap"></span>`fn seL4_X86_PDPT_Unmap(&mut self, service: seL4_X86_PDPT) -> seL4_Error::Type` — [`seL4_X86_PDPT`](../index.md#sel4-x86-pdpt), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pagedirectory-map"></span>`fn seL4_X86_PageDirectory_Map(&mut self, service: seL4_X86_PageDirectory, vspace: seL4_CPtr, vaddr: seL4_Word, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type` — [`seL4_X86_PageDirectory`](../index.md#sel4-x86-pagedirectory), [`seL4_CPtr`](../index.md#sel4-cptr), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_X86_VMAttributes/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pagedirectory-unmap"></span>`fn seL4_X86_PageDirectory_Unmap(&mut self, service: seL4_X86_PageDirectory) -> seL4_Error::Type` — [`seL4_X86_PageDirectory`](../index.md#sel4-x86-pagedirectory), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pagetable-map"></span>`fn seL4_X86_PageTable_Map(&mut self, service: seL4_X86_PageTable, vspace: seL4_CPtr, vaddr: seL4_Word, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type` — [`seL4_X86_PageTable`](../index.md#sel4-x86-pagetable), [`seL4_CPtr`](../index.md#sel4-cptr), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_X86_VMAttributes/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-pagetable-unmap"></span>`fn seL4_X86_PageTable_Unmap(&mut self, service: seL4_X86_PageTable) -> seL4_Error::Type` — [`seL4_X86_PageTable`](../index.md#sel4-x86-pagetable), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-iopagetable-map"></span>`fn seL4_X86_IOPageTable_Map(&mut self, service: seL4_X86_IOPageTable, iospace: seL4_X86_IOSpace, ioaddr: seL4_Word) -> seL4_Error::Type` — [`seL4_X86_IOPageTable`](../index.md#sel4-x86-iopagetable), [`seL4_X86_IOSpace`](../index.md#sel4-x86-iospace), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-iopagetable-unmap"></span>`fn seL4_X86_IOPageTable_Unmap(&mut self, service: seL4_X86_IOPageTable) -> seL4_Error::Type` — [`seL4_X86_IOPageTable`](../index.md#sel4-x86-iopagetable), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-page-map"></span>`fn seL4_X86_Page_Map(&mut self, service: seL4_X86_Page, vspace: seL4_CPtr, vaddr: seL4_Word, rights: seL4_CapRights_t, attr: seL4_X86_VMAttributes::Type) -> seL4_Error::Type` — [`seL4_X86_Page`](../index.md#sel4-x86-page), [`seL4_CPtr`](../index.md#sel4-cptr), [`seL4_Word`](../index.md#sel4-word), [`seL4_CapRights_t`](../index.md#sel4-caprights-t), [`Type`](seL4_X86_VMAttributes/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-page-unmap"></span>`fn seL4_X86_Page_Unmap(&mut self, service: seL4_X86_Page) -> seL4_Error::Type` — [`seL4_X86_Page`](../index.md#sel4-x86-page), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-page-mapio"></span>`fn seL4_X86_Page_MapIO(&mut self, service: seL4_X86_Page, iospace: seL4_X86_IOSpace, rights: seL4_CapRights_t, ioaddr: seL4_Word) -> seL4_Error::Type` — [`seL4_X86_Page`](../index.md#sel4-x86-page), [`seL4_X86_IOSpace`](../index.md#sel4-x86-iospace), [`seL4_CapRights_t`](../index.md#sel4-caprights-t), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-page-getaddress"></span>`fn seL4_X86_Page_GetAddress(&mut self, service: seL4_X86_Page) -> seL4_X86_Page_GetAddress_ret` — [`seL4_X86_Page`](../index.md#sel4-x86-page), [`seL4_X86_Page_GetAddress_ret`](../index.md#sel4-x86-page-getaddress-ret)

- <span id="sel4-ipcbuffer-sel4-x86-asidcontrol-makepool"></span>`fn seL4_X86_ASIDControl_MakePool(&mut self, service: seL4_X86_ASIDControl, untyped: seL4_Untyped, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_X86_ASIDControl`](../index.md#sel4-x86-asidcontrol), [`seL4_Untyped`](../index.md#sel4-untyped), [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-asidpool-assign"></span>`fn seL4_X86_ASIDPool_Assign(&mut self, service: seL4_X86_ASIDPool, vspace: seL4_CPtr) -> seL4_Error::Type` — [`seL4_X86_ASIDPool`](../index.md#sel4-x86-asidpool), [`seL4_CPtr`](../index.md#sel4-cptr), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-ioportcontrol-issue"></span>`fn seL4_X86_IOPortControl_Issue(&mut self, service: seL4_X86_IOPortControl, first_port: seL4_Word, last_port: seL4_Word, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8) -> seL4_Error::Type` — [`seL4_X86_IOPortControl`](../index.md#sel4-x86-ioportcontrol), [`seL4_Word`](../index.md#sel4-word), [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-in8"></span>`fn seL4_X86_IOPort_In8(&mut self, service: seL4_X86_IOPort, port: seL4_Uint16) -> seL4_X86_IOPort_In8_ret` — [`seL4_X86_IOPort`](../index.md#sel4-x86-ioport), [`seL4_Uint16`](../index.md#sel4-uint16), [`seL4_X86_IOPort_In8_ret`](../index.md#sel4-x86-ioport-in8-ret)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-in16"></span>`fn seL4_X86_IOPort_In16(&mut self, service: seL4_X86_IOPort, port: seL4_Uint16) -> seL4_X86_IOPort_In16_ret` — [`seL4_X86_IOPort`](../index.md#sel4-x86-ioport), [`seL4_Uint16`](../index.md#sel4-uint16), [`seL4_X86_IOPort_In16_ret`](../index.md#sel4-x86-ioport-in16-ret)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-in32"></span>`fn seL4_X86_IOPort_In32(&mut self, service: seL4_X86_IOPort, port: seL4_Uint16) -> seL4_X86_IOPort_In32_ret` — [`seL4_X86_IOPort`](../index.md#sel4-x86-ioport), [`seL4_Uint16`](../index.md#sel4-uint16), [`seL4_X86_IOPort_In32_ret`](../index.md#sel4-x86-ioport-in32-ret)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-out8"></span>`fn seL4_X86_IOPort_Out8(&mut self, service: seL4_X86_IOPort, port: seL4_Word, data: seL4_Word) -> seL4_Error::Type` — [`seL4_X86_IOPort`](../index.md#sel4-x86-ioport), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-out16"></span>`fn seL4_X86_IOPort_Out16(&mut self, service: seL4_X86_IOPort, port: seL4_Word, data: seL4_Word) -> seL4_Error::Type` — [`seL4_X86_IOPort`](../index.md#sel4-x86-ioport), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-x86-ioport-out32"></span>`fn seL4_X86_IOPort_Out32(&mut self, service: seL4_X86_IOPort, port: seL4_Word, data: seL4_Word) -> seL4_Error::Type` — [`seL4_X86_IOPort`](../index.md#sel4-x86-ioport), [`seL4_Word`](../index.md#sel4-word), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqcontrol-getioapic"></span>`fn seL4_IRQControl_GetIOAPIC(&mut self, service: seL4_IRQControl, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8, ioapic: seL4_Word, pin: seL4_Word, level: seL4_Word, polarity: seL4_Word, vector: seL4_Word) -> seL4_Error::Type` — [`seL4_IRQControl`](../index.md#sel4-irqcontrol), [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

- <span id="sel4-ipcbuffer-sel4-irqcontrol-getmsi"></span>`fn seL4_IRQControl_GetMSI(&mut self, service: seL4_IRQControl, root: seL4_CNode, index: seL4_Word, depth: seL4_Uint8, pci_bus: seL4_Word, pci_dev: seL4_Word, pci_func: seL4_Word, handle: seL4_Word, vector: seL4_Word) -> seL4_Error::Type` — [`seL4_IRQControl`](../index.md#sel4-irqcontrol), [`seL4_CNode`](../index.md#sel4-cnode), [`seL4_Word`](../index.md#sel4-word), [`seL4_Uint8`](../index.md#sel4-uint8), [`Type`](seL4_Error/index.md#type)

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

- <span id="sel4-x86-vcpu-readmsr-clone"></span>`fn clone(&self) -> seL4_X86_VCPU_ReadMSR` — [`seL4_X86_VCPU_ReadMSR`](../index.md#sel4-x86-vcpu-readmsr)

##### `impl Copy for seL4_X86_VCPU_ReadMSR`

##### `impl Debug for seL4_X86_VCPU_ReadMSR`

- <span id="sel4-x86-vcpu-readmsr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_X86_VCPU_ReadMSR`

- <span id="sel4-x86-vcpu-readmsr-default"></span>`fn default() -> seL4_X86_VCPU_ReadMSR` — [`seL4_X86_VCPU_ReadMSR`](../index.md#sel4-x86-vcpu-readmsr)

##### `impl Eq for seL4_X86_VCPU_ReadMSR`

##### `impl PartialEq for seL4_X86_VCPU_ReadMSR`

- <span id="sel4-x86-vcpu-readmsr-partialeq-eq"></span>`fn eq(&self, other: &seL4_X86_VCPU_ReadMSR) -> bool` — [`seL4_X86_VCPU_ReadMSR`](../index.md#sel4-x86-vcpu-readmsr)

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

- <span id="sel4-x86-vcpu-writemsr-clone"></span>`fn clone(&self) -> seL4_X86_VCPU_WriteMSR` — [`seL4_X86_VCPU_WriteMSR`](../index.md#sel4-x86-vcpu-writemsr)

##### `impl Copy for seL4_X86_VCPU_WriteMSR`

##### `impl Debug for seL4_X86_VCPU_WriteMSR`

- <span id="sel4-x86-vcpu-writemsr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_X86_VCPU_WriteMSR`

- <span id="sel4-x86-vcpu-writemsr-default"></span>`fn default() -> seL4_X86_VCPU_WriteMSR` — [`seL4_X86_VCPU_WriteMSR`](../index.md#sel4-x86-vcpu-writemsr)

##### `impl Eq for seL4_X86_VCPU_WriteMSR`

##### `impl PartialEq for seL4_X86_VCPU_WriteMSR`

- <span id="sel4-x86-vcpu-writemsr-partialeq-eq"></span>`fn eq(&self, other: &seL4_X86_VCPU_WriteMSR) -> bool` — [`seL4_X86_VCPU_WriteMSR`](../index.md#sel4-x86-vcpu-writemsr)

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

- <span id="sel4-x86-pagedirectory-getstatusbits-clone"></span>`fn clone(&self) -> seL4_X86_PageDirectory_GetStatusBits` — [`seL4_X86_PageDirectory_GetStatusBits`](../index.md#sel4-x86-pagedirectory-getstatusbits)

##### `impl Copy for seL4_X86_PageDirectory_GetStatusBits`

##### `impl Debug for seL4_X86_PageDirectory_GetStatusBits`

- <span id="sel4-x86-pagedirectory-getstatusbits-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_X86_PageDirectory_GetStatusBits`

- <span id="sel4-x86-pagedirectory-getstatusbits-default"></span>`fn default() -> seL4_X86_PageDirectory_GetStatusBits` — [`seL4_X86_PageDirectory_GetStatusBits`](../index.md#sel4-x86-pagedirectory-getstatusbits)

##### `impl Eq for seL4_X86_PageDirectory_GetStatusBits`

##### `impl PartialEq for seL4_X86_PageDirectory_GetStatusBits`

- <span id="sel4-x86-pagedirectory-getstatusbits-partialeq-eq"></span>`fn eq(&self, other: &seL4_X86_PageDirectory_GetStatusBits) -> bool` — [`seL4_X86_PageDirectory_GetStatusBits`](../index.md#sel4-x86-pagedirectory-getstatusbits)

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

- <span id="sel4-x86-vcpu-readvmcs-clone"></span>`fn clone(&self) -> seL4_X86_VCPU_ReadVMCS` — [`seL4_X86_VCPU_ReadVMCS`](../index.md#sel4-x86-vcpu-readvmcs)

##### `impl Copy for seL4_X86_VCPU_ReadVMCS`

##### `impl Debug for seL4_X86_VCPU_ReadVMCS`

- <span id="sel4-x86-vcpu-readvmcs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_X86_VCPU_ReadVMCS`

- <span id="sel4-x86-vcpu-readvmcs-default"></span>`fn default() -> seL4_X86_VCPU_ReadVMCS` — [`seL4_X86_VCPU_ReadVMCS`](../index.md#sel4-x86-vcpu-readvmcs)

##### `impl Eq for seL4_X86_VCPU_ReadVMCS`

##### `impl PartialEq for seL4_X86_VCPU_ReadVMCS`

- <span id="sel4-x86-vcpu-readvmcs-partialeq-eq"></span>`fn eq(&self, other: &seL4_X86_VCPU_ReadVMCS) -> bool` — [`seL4_X86_VCPU_ReadVMCS`](../index.md#sel4-x86-vcpu-readvmcs)

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

- <span id="sel4-x86-vcpu-writevmcs-clone"></span>`fn clone(&self) -> seL4_X86_VCPU_WriteVMCS` — [`seL4_X86_VCPU_WriteVMCS`](../index.md#sel4-x86-vcpu-writevmcs)

##### `impl Copy for seL4_X86_VCPU_WriteVMCS`

##### `impl Debug for seL4_X86_VCPU_WriteVMCS`

- <span id="sel4-x86-vcpu-writevmcs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_X86_VCPU_WriteVMCS`

- <span id="sel4-x86-vcpu-writevmcs-default"></span>`fn default() -> seL4_X86_VCPU_WriteVMCS` — [`seL4_X86_VCPU_WriteVMCS`](../index.md#sel4-x86-vcpu-writevmcs)

##### `impl Eq for seL4_X86_VCPU_WriteVMCS`

##### `impl PartialEq for seL4_X86_VCPU_WriteVMCS`

- <span id="sel4-x86-vcpu-writevmcs-partialeq-eq"></span>`fn eq(&self, other: &seL4_X86_VCPU_WriteVMCS) -> bool` — [`seL4_X86_VCPU_WriteVMCS`](../index.md#sel4-x86-vcpu-writevmcs)

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

- <span id="sel4-tcb-getbreakpoint-clone"></span>`fn clone(&self) -> seL4_TCB_GetBreakpoint` — [`seL4_TCB_GetBreakpoint`](../index.md#sel4-tcb-getbreakpoint)

##### `impl Copy for seL4_TCB_GetBreakpoint`

##### `impl Debug for seL4_TCB_GetBreakpoint`

- <span id="sel4-tcb-getbreakpoint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_TCB_GetBreakpoint`

- <span id="sel4-tcb-getbreakpoint-default"></span>`fn default() -> seL4_TCB_GetBreakpoint` — [`seL4_TCB_GetBreakpoint`](../index.md#sel4-tcb-getbreakpoint)

##### `impl Eq for seL4_TCB_GetBreakpoint`

##### `impl PartialEq for seL4_TCB_GetBreakpoint`

- <span id="sel4-tcb-getbreakpoint-partialeq-eq"></span>`fn eq(&self, other: &seL4_TCB_GetBreakpoint) -> bool` — [`seL4_TCB_GetBreakpoint`](../index.md#sel4-tcb-getbreakpoint)

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

- <span id="sel4-tcb-configuresinglestepping-clone"></span>`fn clone(&self) -> seL4_TCB_ConfigureSingleStepping` — [`seL4_TCB_ConfigureSingleStepping`](../index.md#sel4-tcb-configuresinglestepping)

##### `impl Copy for seL4_TCB_ConfigureSingleStepping`

##### `impl Debug for seL4_TCB_ConfigureSingleStepping`

- <span id="sel4-tcb-configuresinglestepping-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_TCB_ConfigureSingleStepping`

- <span id="sel4-tcb-configuresinglestepping-default"></span>`fn default() -> seL4_TCB_ConfigureSingleStepping` — [`seL4_TCB_ConfigureSingleStepping`](../index.md#sel4-tcb-configuresinglestepping)

##### `impl Eq for seL4_TCB_ConfigureSingleStepping`

##### `impl PartialEq for seL4_TCB_ConfigureSingleStepping`

- <span id="sel4-tcb-configuresinglestepping-partialeq-eq"></span>`fn eq(&self, other: &seL4_TCB_ConfigureSingleStepping) -> bool` — [`seL4_TCB_ConfigureSingleStepping`](../index.md#sel4-tcb-configuresinglestepping)

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

- <span id="sel4-schedcontext-consumed-clone"></span>`fn clone(&self) -> seL4_SchedContext_Consumed` — [`seL4_SchedContext_Consumed`](../index.md#sel4-schedcontext-consumed)

##### `impl Copy for seL4_SchedContext_Consumed`

##### `impl Debug for seL4_SchedContext_Consumed`

- <span id="sel4-schedcontext-consumed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_SchedContext_Consumed`

- <span id="sel4-schedcontext-consumed-default"></span>`fn default() -> seL4_SchedContext_Consumed` — [`seL4_SchedContext_Consumed`](../index.md#sel4-schedcontext-consumed)

##### `impl Eq for seL4_SchedContext_Consumed`

##### `impl PartialEq for seL4_SchedContext_Consumed`

- <span id="sel4-schedcontext-consumed-partialeq-eq"></span>`fn eq(&self, other: &seL4_SchedContext_Consumed) -> bool` — [`seL4_SchedContext_Consumed`](../index.md#sel4-schedcontext-consumed)

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

- <span id="sel4-schedcontext-yieldto-clone"></span>`fn clone(&self) -> seL4_SchedContext_YieldTo` — [`seL4_SchedContext_YieldTo`](../index.md#sel4-schedcontext-yieldto)

##### `impl Copy for seL4_SchedContext_YieldTo`

##### `impl Debug for seL4_SchedContext_YieldTo`

- <span id="sel4-schedcontext-yieldto-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_SchedContext_YieldTo`

- <span id="sel4-schedcontext-yieldto-default"></span>`fn default() -> seL4_SchedContext_YieldTo` — [`seL4_SchedContext_YieldTo`](../index.md#sel4-schedcontext-yieldto)

##### `impl Eq for seL4_SchedContext_YieldTo`

##### `impl PartialEq for seL4_SchedContext_YieldTo`

- <span id="sel4-schedcontext-yieldto-partialeq-eq"></span>`fn eq(&self, other: &seL4_SchedContext_YieldTo) -> bool` — [`seL4_SchedContext_YieldTo`](../index.md#sel4-schedcontext-yieldto)

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

- <span id="sel4-slotregion-clone"></span>`fn clone(&self) -> seL4_SlotRegion` — [`seL4_SlotRegion`](../index.md#sel4-slotregion)

##### `impl Copy for seL4_SlotRegion`

##### `impl Debug for seL4_SlotRegion`

- <span id="sel4-slotregion-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_SlotRegion`

- <span id="sel4-slotregion-default"></span>`fn default() -> seL4_SlotRegion` — [`seL4_SlotRegion`](../index.md#sel4-slotregion)

##### `impl Eq for seL4_SlotRegion`

##### `impl PartialEq for seL4_SlotRegion`

- <span id="sel4-slotregion-partialeq-eq"></span>`fn eq(&self, other: &seL4_SlotRegion) -> bool` — [`seL4_SlotRegion`](../index.md#sel4-slotregion)

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

- <span id="sel4-untypeddesc-clone"></span>`fn clone(&self) -> seL4_UntypedDesc` — [`seL4_UntypedDesc`](../index.md#sel4-untypeddesc)

##### `impl Copy for seL4_UntypedDesc`

##### `impl Debug for seL4_UntypedDesc`

- <span id="sel4-untypeddesc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_UntypedDesc`

- <span id="sel4-untypeddesc-default"></span>`fn default() -> seL4_UntypedDesc` — [`seL4_UntypedDesc`](../index.md#sel4-untypeddesc)

##### `impl Eq for seL4_UntypedDesc`

##### `impl PartialEq for seL4_UntypedDesc`

- <span id="sel4-untypeddesc-partialeq-eq"></span>`fn eq(&self, other: &seL4_UntypedDesc) -> bool` — [`seL4_UntypedDesc`](../index.md#sel4-untypeddesc)

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

- <span id="sel4-bootinfo-clone"></span>`fn clone(&self) -> seL4_BootInfo` — [`seL4_BootInfo`](../index.md#sel4-bootinfo)

##### `impl Copy for seL4_BootInfo`

##### `impl Debug for seL4_BootInfo`

- <span id="sel4-bootinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_BootInfo`

- <span id="sel4-bootinfo-default"></span>`fn default() -> Self`

##### `impl Eq for seL4_BootInfo`

##### `impl PartialEq for seL4_BootInfo`

- <span id="sel4-bootinfo-partialeq-eq"></span>`fn eq(&self, other: &seL4_BootInfo) -> bool` — [`seL4_BootInfo`](../index.md#sel4-bootinfo)

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

- <span id="sel4-bootinfoheader-clone"></span>`fn clone(&self) -> seL4_BootInfoHeader` — [`seL4_BootInfoHeader`](../index.md#sel4-bootinfoheader)

##### `impl Copy for seL4_BootInfoHeader`

##### `impl Debug for seL4_BootInfoHeader`

- <span id="sel4-bootinfoheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for seL4_BootInfoHeader`

- <span id="sel4-bootinfoheader-default"></span>`fn default() -> seL4_BootInfoHeader` — [`seL4_BootInfoHeader`](../index.md#sel4-bootinfoheader)

##### `impl Eq for seL4_BootInfoHeader`

##### `impl PartialEq for seL4_BootInfoHeader`

- <span id="sel4-bootinfoheader-partialeq-eq"></span>`fn eq(&self, other: &seL4_BootInfoHeader) -> bool` — [`seL4_BootInfoHeader`](../index.md#sel4-bootinfoheader)

##### `impl StructuralPartialEq for seL4_BootInfoHeader`

## Type Aliases

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

### `seL4_ObjectType`

```rust
type seL4_ObjectType = ::core::ffi::c_uint;
```

### `seL4_seL4ArchObjectType`

```rust
type seL4_seL4ArchObjectType = ::core::ffi::c_uint;
```

### `seL4_ArchObjectType`

```rust
type seL4_ArchObjectType = ::core::ffi::c_uint;
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

