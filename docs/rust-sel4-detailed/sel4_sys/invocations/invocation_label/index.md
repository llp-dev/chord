*[sel4_sys](../../index.md) / [invocations](../index.md) / [invocation_label](index.md)*

---

# Module `invocation_label`

## Contents

- [Constants](#constants)
  - [`InvalidInvocation`](#invalidinvocation)
  - [`UntypedRetype`](#untypedretype)
  - [`TCBReadRegisters`](#tcbreadregisters)
  - [`TCBWriteRegisters`](#tcbwriteregisters)
  - [`TCBCopyRegisters`](#tcbcopyregisters)
  - [`TCBConfigure`](#tcbconfigure)
  - [`TCBSetPriority`](#tcbsetpriority)
  - [`TCBSetMCPriority`](#tcbsetmcpriority)
  - [`TCBSetSchedParams`](#tcbsetschedparams)
  - [`TCBSetIPCBuffer`](#tcbsetipcbuffer)
  - [`TCBSetSpace`](#tcbsetspace)
  - [`TCBSuspend`](#tcbsuspend)
  - [`TCBResume`](#tcbresume)
  - [`TCBBindNotification`](#tcbbindnotification)
  - [`TCBUnbindNotification`](#tcbunbindnotification)
  - [`TCBSetTLSBase`](#tcbsettlsbase)
  - [`TCBSetFlags`](#tcbsetflags)
  - [`CNodeRevoke`](#cnoderevoke)
  - [`CNodeDelete`](#cnodedelete)
  - [`CNodeCancelBadgedSends`](#cnodecancelbadgedsends)
  - [`CNodeCopy`](#cnodecopy)
  - [`CNodeMint`](#cnodemint)
  - [`CNodeMove`](#cnodemove)
  - [`CNodeMutate`](#cnodemutate)
  - [`CNodeRotate`](#cnoderotate)
  - [`CNodeSaveCaller`](#cnodesavecaller)
  - [`IRQIssueIRQHandler`](#irqissueirqhandler)
  - [`IRQAckIRQ`](#irqackirq)
  - [`IRQSetIRQHandler`](#irqsetirqhandler)
  - [`IRQClearIRQHandler`](#irqclearirqhandler)
  - [`DomainSetSet`](#domainsetset)
  - [`DomainScheduleConfigure`](#domainscheduleconfigure)
  - [`DomainScheduleSetStart`](#domainschedulesetstart)
  - [`X86PDPTMap`](#x86pdptmap)
  - [`X86PDPTUnmap`](#x86pdptunmap)
  - [`X86PageDirectoryMap`](#x86pagedirectorymap)
  - [`X86PageDirectoryUnmap`](#x86pagedirectoryunmap)
  - [`X86PageTableMap`](#x86pagetablemap)
  - [`X86PageTableUnmap`](#x86pagetableunmap)
  - [`X86IOPageTableMap`](#x86iopagetablemap)
  - [`X86IOPageTableUnmap`](#x86iopagetableunmap)
  - [`X86PageMap`](#x86pagemap)
  - [`X86PageUnmap`](#x86pageunmap)
  - [`X86PageMapIO`](#x86pagemapio)
  - [`X86PageGetAddress`](#x86pagegetaddress)
  - [`X86ASIDControlMakePool`](#x86asidcontrolmakepool)
  - [`X86ASIDPoolAssign`](#x86asidpoolassign)
  - [`X86IOPortControlIssue`](#x86ioportcontrolissue)
  - [`X86IOPortIn8`](#x86ioportin8)
  - [`X86IOPortIn16`](#x86ioportin16)
  - [`X86IOPortIn32`](#x86ioportin32)
  - [`X86IOPortOut8`](#x86ioportout8)
  - [`X86IOPortOut16`](#x86ioportout16)
  - [`X86IOPortOut32`](#x86ioportout32)
  - [`X86IRQIssueIRQHandlerIOAPIC`](#x86irqissueirqhandlerioapic)
  - [`X86IRQIssueIRQHandlerMSI`](#x86irqissueirqhandlermsi)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`InvalidInvocation`](#invalidinvocation) | const |  |
| [`UntypedRetype`](#untypedretype) | const |  |
| [`TCBReadRegisters`](#tcbreadregisters) | const |  |
| [`TCBWriteRegisters`](#tcbwriteregisters) | const |  |
| [`TCBCopyRegisters`](#tcbcopyregisters) | const |  |
| [`TCBConfigure`](#tcbconfigure) | const |  |
| [`TCBSetPriority`](#tcbsetpriority) | const |  |
| [`TCBSetMCPriority`](#tcbsetmcpriority) | const |  |
| [`TCBSetSchedParams`](#tcbsetschedparams) | const |  |
| [`TCBSetIPCBuffer`](#tcbsetipcbuffer) | const |  |
| [`TCBSetSpace`](#tcbsetspace) | const |  |
| [`TCBSuspend`](#tcbsuspend) | const |  |
| [`TCBResume`](#tcbresume) | const |  |
| [`TCBBindNotification`](#tcbbindnotification) | const |  |
| [`TCBUnbindNotification`](#tcbunbindnotification) | const |  |
| [`TCBSetTLSBase`](#tcbsettlsbase) | const |  |
| [`TCBSetFlags`](#tcbsetflags) | const |  |
| [`CNodeRevoke`](#cnoderevoke) | const |  |
| [`CNodeDelete`](#cnodedelete) | const |  |
| [`CNodeCancelBadgedSends`](#cnodecancelbadgedsends) | const |  |
| [`CNodeCopy`](#cnodecopy) | const |  |
| [`CNodeMint`](#cnodemint) | const |  |
| [`CNodeMove`](#cnodemove) | const |  |
| [`CNodeMutate`](#cnodemutate) | const |  |
| [`CNodeRotate`](#cnoderotate) | const |  |
| [`CNodeSaveCaller`](#cnodesavecaller) | const |  |
| [`IRQIssueIRQHandler`](#irqissueirqhandler) | const |  |
| [`IRQAckIRQ`](#irqackirq) | const |  |
| [`IRQSetIRQHandler`](#irqsetirqhandler) | const |  |
| [`IRQClearIRQHandler`](#irqclearirqhandler) | const |  |
| [`DomainSetSet`](#domainsetset) | const |  |
| [`DomainScheduleConfigure`](#domainscheduleconfigure) | const |  |
| [`DomainScheduleSetStart`](#domainschedulesetstart) | const |  |
| [`X86PDPTMap`](#x86pdptmap) | const |  |
| [`X86PDPTUnmap`](#x86pdptunmap) | const |  |
| [`X86PageDirectoryMap`](#x86pagedirectorymap) | const |  |
| [`X86PageDirectoryUnmap`](#x86pagedirectoryunmap) | const |  |
| [`X86PageTableMap`](#x86pagetablemap) | const |  |
| [`X86PageTableUnmap`](#x86pagetableunmap) | const |  |
| [`X86IOPageTableMap`](#x86iopagetablemap) | const |  |
| [`X86IOPageTableUnmap`](#x86iopagetableunmap) | const |  |
| [`X86PageMap`](#x86pagemap) | const |  |
| [`X86PageUnmap`](#x86pageunmap) | const |  |
| [`X86PageMapIO`](#x86pagemapio) | const |  |
| [`X86PageGetAddress`](#x86pagegetaddress) | const |  |
| [`X86ASIDControlMakePool`](#x86asidcontrolmakepool) | const |  |
| [`X86ASIDPoolAssign`](#x86asidpoolassign) | const |  |
| [`X86IOPortControlIssue`](#x86ioportcontrolissue) | const |  |
| [`X86IOPortIn8`](#x86ioportin8) | const |  |
| [`X86IOPortIn16`](#x86ioportin16) | const |  |
| [`X86IOPortIn32`](#x86ioportin32) | const |  |
| [`X86IOPortOut8`](#x86ioportout8) | const |  |
| [`X86IOPortOut16`](#x86ioportout16) | const |  |
| [`X86IOPortOut32`](#x86ioportout32) | const |  |
| [`X86IRQIssueIRQHandlerIOAPIC`](#x86irqissueirqhandlerioapic) | const |  |
| [`X86IRQIssueIRQHandlerMSI`](#x86irqissueirqhandlermsi) | const |  |

## Constants

### `InvalidInvocation`
```rust
const InvalidInvocation: u32 = 0u32;
```

### `UntypedRetype`
```rust
const UntypedRetype: u32 = 1u32;
```

### `TCBReadRegisters`
```rust
const TCBReadRegisters: u32 = 2u32;
```

### `TCBWriteRegisters`
```rust
const TCBWriteRegisters: u32 = 3u32;
```

### `TCBCopyRegisters`
```rust
const TCBCopyRegisters: u32 = 4u32;
```

### `TCBConfigure`
```rust
const TCBConfigure: u32 = 5u32;
```

### `TCBSetPriority`
```rust
const TCBSetPriority: u32 = 6u32;
```

### `TCBSetMCPriority`
```rust
const TCBSetMCPriority: u32 = 7u32;
```

### `TCBSetSchedParams`
```rust
const TCBSetSchedParams: u32 = 8u32;
```

### `TCBSetIPCBuffer`
```rust
const TCBSetIPCBuffer: u32 = 9u32;
```

### `TCBSetSpace`
```rust
const TCBSetSpace: u32 = 10u32;
```

### `TCBSuspend`
```rust
const TCBSuspend: u32 = 11u32;
```

### `TCBResume`
```rust
const TCBResume: u32 = 12u32;
```

### `TCBBindNotification`
```rust
const TCBBindNotification: u32 = 13u32;
```

### `TCBUnbindNotification`
```rust
const TCBUnbindNotification: u32 = 14u32;
```

### `TCBSetTLSBase`
```rust
const TCBSetTLSBase: u32 = 15u32;
```

### `TCBSetFlags`
```rust
const TCBSetFlags: u32 = 16u32;
```

### `CNodeRevoke`
```rust
const CNodeRevoke: u32 = 17u32;
```

### `CNodeDelete`
```rust
const CNodeDelete: u32 = 18u32;
```

### `CNodeCancelBadgedSends`
```rust
const CNodeCancelBadgedSends: u32 = 19u32;
```

### `CNodeCopy`
```rust
const CNodeCopy: u32 = 20u32;
```

### `CNodeMint`
```rust
const CNodeMint: u32 = 21u32;
```

### `CNodeMove`
```rust
const CNodeMove: u32 = 22u32;
```

### `CNodeMutate`
```rust
const CNodeMutate: u32 = 23u32;
```

### `CNodeRotate`
```rust
const CNodeRotate: u32 = 24u32;
```

### `CNodeSaveCaller`
```rust
const CNodeSaveCaller: u32 = 25u32;
```

### `IRQIssueIRQHandler`
```rust
const IRQIssueIRQHandler: u32 = 26u32;
```

### `IRQAckIRQ`
```rust
const IRQAckIRQ: u32 = 27u32;
```

### `IRQSetIRQHandler`
```rust
const IRQSetIRQHandler: u32 = 28u32;
```

### `IRQClearIRQHandler`
```rust
const IRQClearIRQHandler: u32 = 29u32;
```

### `DomainSetSet`
```rust
const DomainSetSet: u32 = 30u32;
```

### `DomainScheduleConfigure`
```rust
const DomainScheduleConfigure: u32 = 31u32;
```

### `DomainScheduleSetStart`
```rust
const DomainScheduleSetStart: u32 = 32u32;
```

### `X86PDPTMap`
```rust
const X86PDPTMap: u32 = 33u32;
```

### `X86PDPTUnmap`
```rust
const X86PDPTUnmap: u32 = 34u32;
```

### `X86PageDirectoryMap`
```rust
const X86PageDirectoryMap: u32 = 35u32;
```

### `X86PageDirectoryUnmap`
```rust
const X86PageDirectoryUnmap: u32 = 36u32;
```

### `X86PageTableMap`
```rust
const X86PageTableMap: u32 = 37u32;
```

### `X86PageTableUnmap`
```rust
const X86PageTableUnmap: u32 = 38u32;
```

### `X86IOPageTableMap`
```rust
const X86IOPageTableMap: u32 = 39u32;
```

### `X86IOPageTableUnmap`
```rust
const X86IOPageTableUnmap: u32 = 40u32;
```

### `X86PageMap`
```rust
const X86PageMap: u32 = 41u32;
```

### `X86PageUnmap`
```rust
const X86PageUnmap: u32 = 42u32;
```

### `X86PageMapIO`
```rust
const X86PageMapIO: u32 = 43u32;
```

### `X86PageGetAddress`
```rust
const X86PageGetAddress: u32 = 44u32;
```

### `X86ASIDControlMakePool`
```rust
const X86ASIDControlMakePool: u32 = 45u32;
```

### `X86ASIDPoolAssign`
```rust
const X86ASIDPoolAssign: u32 = 46u32;
```

### `X86IOPortControlIssue`
```rust
const X86IOPortControlIssue: u32 = 47u32;
```

### `X86IOPortIn8`
```rust
const X86IOPortIn8: u32 = 48u32;
```

### `X86IOPortIn16`
```rust
const X86IOPortIn16: u32 = 49u32;
```

### `X86IOPortIn32`
```rust
const X86IOPortIn32: u32 = 50u32;
```

### `X86IOPortOut8`
```rust
const X86IOPortOut8: u32 = 51u32;
```

### `X86IOPortOut16`
```rust
const X86IOPortOut16: u32 = 52u32;
```

### `X86IOPortOut32`
```rust
const X86IOPortOut32: u32 = 53u32;
```

### `X86IRQIssueIRQHandlerIOAPIC`
```rust
const X86IRQIssueIRQHandlerIOAPIC: u32 = 54u32;
```

### `X86IRQIssueIRQHandlerMSI`
```rust
const X86IRQIssueIRQHandlerMSI: u32 = 55u32;
```

