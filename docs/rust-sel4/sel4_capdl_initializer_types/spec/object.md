**sel4_capdl_initializer_types > spec > object**

# Module: spec::object

## Contents

**Structs**

- [`ArchivedArmIrq`](#archivedarmirq) - An archived [`ArmIrq`]
- [`ArchivedArmIrqExtraInfo`](#archivedarmirqextrainfo) - An archived [`ArmIrqExtraInfo`]
- [`ArchivedAsidPool`](#archivedasidpool) - An archived [`AsidPool`]
- [`ArchivedCNode`](#archivedcnode) - An archived [`CNode`]
- [`ArchivedFrame`](#archivedframe) - An archived [`Frame`]
- [`ArchivedIOPorts`](#archivedioports) - An archived [`IOPorts`]
- [`ArchivedIrq`](#archivedirq) - An archived [`Irq`]
- [`ArchivedIrqIOApic`](#archivedirqioapic) - An archived [`IrqIOApic`]
- [`ArchivedIrqIOApicExtraInfo`](#archivedirqioapicextrainfo) - An archived [`IrqIOApicExtraInfo`]
- [`ArchivedIrqMsi`](#archivedirqmsi) - An archived [`IrqMsi`]
- [`ArchivedIrqMsiExtraInfo`](#archivedirqmsiextrainfo) - An archived [`IrqMsiExtraInfo`]
- [`ArchivedPageTable`](#archivedpagetable) - An archived [`PageTable`]
- [`ArchivedRiscvIrq`](#archivedriscvirq) - An archived [`RiscvIrq`]
- [`ArchivedRiscvIrqExtraInfo`](#archivedriscvirqextrainfo) - An archived [`RiscvIrqExtraInfo`]
- [`ArchivedSchedContext`](#archivedschedcontext) - An archived [`SchedContext`]
- [`ArchivedSchedContextExtraInfo`](#archivedschedcontextextrainfo) - An archived [`SchedContextExtraInfo`]
- [`ArchivedTcb`](#archivedtcb) - An archived [`Tcb`]
- [`ArchivedTcbExtraInfo`](#archivedtcbextrainfo) - An archived [`TcbExtraInfo`]
- [`ArchivedUntyped`](#archiveduntyped) - An archived [`Untyped`]
- [`ArmIrq`](#armirq)
- [`ArmIrqExtraInfo`](#armirqextrainfo)
- [`ArmIrqExtraInfoResolver`](#armirqextrainforesolver) - The resolver for an archived [`ArmIrqExtraInfo`]
- [`ArmIrqResolver`](#armirqresolver) - The resolver for an archived [`ArmIrq`]
- [`AsidPool`](#asidpool)
- [`AsidPoolResolver`](#asidpoolresolver) - The resolver for an archived [`AsidPool`]
- [`CNode`](#cnode)
- [`CNodeResolver`](#cnoderesolver) - The resolver for an archived [`CNode`]
- [`Frame`](#frame)
- [`FrameResolver`](#frameresolver) - The resolver for an archived [`Frame`]
- [`IOPorts`](#ioports)
- [`IOPortsResolver`](#ioportsresolver) - The resolver for an archived [`IOPorts`]
- [`Irq`](#irq)
- [`IrqIOApic`](#irqioapic)
- [`IrqIOApicExtraInfo`](#irqioapicextrainfo)
- [`IrqIOApicExtraInfoResolver`](#irqioapicextrainforesolver) - The resolver for an archived [`IrqIOApicExtraInfo`]
- [`IrqIOApicResolver`](#irqioapicresolver) - The resolver for an archived [`IrqIOApic`]
- [`IrqMsi`](#irqmsi)
- [`IrqMsiExtraInfo`](#irqmsiextrainfo)
- [`IrqMsiExtraInfoResolver`](#irqmsiextrainforesolver) - The resolver for an archived [`IrqMsiExtraInfo`]
- [`IrqMsiResolver`](#irqmsiresolver) - The resolver for an archived [`IrqMsi`]
- [`IrqResolver`](#irqresolver) - The resolver for an archived [`Irq`]
- [`PageTable`](#pagetable)
- [`PageTableResolver`](#pagetableresolver) - The resolver for an archived [`PageTable`]
- [`RiscvIrq`](#riscvirq)
- [`RiscvIrqExtraInfo`](#riscvirqextrainfo)
- [`RiscvIrqExtraInfoResolver`](#riscvirqextrainforesolver) - The resolver for an archived [`RiscvIrqExtraInfo`]
- [`RiscvIrqResolver`](#riscvirqresolver) - The resolver for an archived [`RiscvIrq`]
- [`SchedContext`](#schedcontext)
- [`SchedContextExtraInfo`](#schedcontextextrainfo)
- [`SchedContextExtraInfoResolver`](#schedcontextextrainforesolver) - The resolver for an archived [`SchedContextExtraInfo`]
- [`SchedContextResolver`](#schedcontextresolver) - The resolver for an archived [`SchedContext`]
- [`Tcb`](#tcb)
- [`TcbExtraInfo`](#tcbextrainfo)
- [`TcbExtraInfoResolver`](#tcbextrainforesolver) - The resolver for an archived [`TcbExtraInfo`]
- [`TcbResolver`](#tcbresolver) - The resolver for an archived [`Tcb`]
- [`Untyped`](#untyped)
- [`UntypedResolver`](#untypedresolver) - The resolver for an archived [`Untyped`]

---

## sel4_capdl_initializer_types::spec::object::ArchivedArmIrq

*Struct*

An archived [`ArmIrq`]

**Fields:**
- `slots: <alloc::vec::Vec<CapTableEntry> as ::rkyv::Archive>::Archived` - The archived counterpart of [`ArmIrq::slots`]
- `extra: <alloc::boxed::Box<ArmIrqExtraInfo> as ::rkyv::Archive>::Archived` - The archived counterpart of [`ArmIrq::extra`]

**Methods:**

- `fn notification(self: &Self) -> Option<&<cap::Notification as Archive>::Archived>`

**Traits:** Portable

**Trait Implementations:**

- **HasArchivedCapTable**
  - `fn slots(self: &Self) -> &[ArchivedCapTableEntry]`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::ArchivedArmIrqExtraInfo

*Struct*

An archived [`ArmIrqExtraInfo`]

**Fields:**
- `trigger: <u8 as ::rkyv::Archive>::Archived` - The archived counterpart of [`ArmIrqExtraInfo::trigger`]
- `target: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`ArmIrqExtraInfo::target`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::ArchivedAsidPool

*Struct*

An archived [`AsidPool`]

**Fields:**
- `high: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`AsidPool::high`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::ArchivedCNode

*Struct*

An archived [`CNode`]

**Fields:**
- `size_bits: <u8 as ::rkyv::Archive>::Archived` - The archived counterpart of [`CNode::size_bits`]
- `slots: <alloc::vec::Vec<CapTableEntry> as ::rkyv::Archive>::Archived` - The archived counterpart of [`CNode::slots`]

**Traits:** Portable

**Trait Implementations:**

- **HasArchivedCapTable**
  - `fn slots(self: &Self) -> &[ArchivedCapTableEntry]`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::ArchivedFrame

*Struct*

An archived [`Frame`]

**Generic Parameters:**
- D

**Fields:**
- `size_bits: <u8 as ::rkyv::Archive>::Archived` - The archived counterpart of [`Frame::size_bits`]
- `paddr: <Option<Word> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Frame::paddr`]
- `init: <D as ::rkyv::Archive>::Archived` - The archived counterpart of [`Frame::init`]

**Traits:** Portable

**Trait Implementations:**

- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::ArchivedIOPorts

*Struct*

An archived [`IOPorts`]

**Fields:**
- `start_port: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IOPorts::start_port`]
- `end_port: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IOPorts::end_port`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::ArchivedIrq

*Struct*

An archived [`Irq`]

**Fields:**
- `slots: <alloc::vec::Vec<CapTableEntry> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Irq::slots`]

**Methods:**

- `fn notification(self: &Self) -> Option<&<cap::Notification as Archive>::Archived>`

**Traits:** Portable

**Trait Implementations:**

- **HasArchivedCapTable**
  - `fn slots(self: &Self) -> &[ArchivedCapTableEntry]`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::ArchivedIrqIOApic

*Struct*

An archived [`IrqIOApic`]

**Fields:**
- `slots: <alloc::vec::Vec<CapTableEntry> as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqIOApic::slots`]
- `extra: <alloc::boxed::Box<IrqIOApicExtraInfo> as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqIOApic::extra`]

**Methods:**

- `fn notification(self: &Self) -> Option<&<cap::Notification as Archive>::Archived>`

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`
- **HasArchivedCapTable**
  - `fn slots(self: &Self) -> &[ArchivedCapTableEntry]`



## sel4_capdl_initializer_types::spec::object::ArchivedIrqIOApicExtraInfo

*Struct*

An archived [`IrqIOApicExtraInfo`]

**Fields:**
- `ioapic: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqIOApicExtraInfo::ioapic`]
- `pin: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqIOApicExtraInfo::pin`]
- `level: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqIOApicExtraInfo::level`]
- `polarity: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqIOApicExtraInfo::polarity`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::ArchivedIrqMsi

*Struct*

An archived [`IrqMsi`]

**Fields:**
- `slots: <alloc::vec::Vec<CapTableEntry> as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqMsi::slots`]
- `extra: <alloc::boxed::Box<IrqMsiExtraInfo> as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqMsi::extra`]

**Methods:**

- `fn notification(self: &Self) -> Option<&<cap::Notification as Archive>::Archived>`

**Traits:** Portable

**Trait Implementations:**

- **HasArchivedCapTable**
  - `fn slots(self: &Self) -> &[ArchivedCapTableEntry]`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::ArchivedIrqMsiExtraInfo

*Struct*

An archived [`IrqMsiExtraInfo`]

**Fields:**
- `handle: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqMsiExtraInfo::handle`]
- `pci_bus: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqMsiExtraInfo::pci_bus`]
- `pci_dev: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqMsiExtraInfo::pci_dev`]
- `pci_func: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqMsiExtraInfo::pci_func`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::ArchivedPageTable

*Struct*

An archived [`PageTable`]

**Fields:**
- `x86_ept: <bool as ::rkyv::Archive>::Archived` - The archived counterpart of [`PageTable::x86_ept`]
- `is_root: <bool as ::rkyv::Archive>::Archived` - The archived counterpart of [`PageTable::is_root`]
- `level: <Option<u8> as ::rkyv::Archive>::Archived` - The archived counterpart of [`PageTable::level`]
- `slots: <alloc::vec::Vec<CapTableEntry> as ::rkyv::Archive>::Archived` - The archived counterpart of [`PageTable::slots`]

**Methods:**

- `fn entries(self: &Self) -> impl Trait`

**Traits:** Portable

**Trait Implementations:**

- **HasArchivedCapTable**
  - `fn slots(self: &Self) -> &[ArchivedCapTableEntry]`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::ArchivedRiscvIrq

*Struct*

An archived [`RiscvIrq`]

**Fields:**
- `slots: <alloc::vec::Vec<CapTableEntry> as ::rkyv::Archive>::Archived` - The archived counterpart of [`RiscvIrq::slots`]
- `extra: <RiscvIrqExtraInfo as ::rkyv::Archive>::Archived` - The archived counterpart of [`RiscvIrq::extra`]

**Methods:**

- `fn notification(self: &Self) -> Option<&<cap::Notification as Archive>::Archived>`

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`
- **HasArchivedCapTable**
  - `fn slots(self: &Self) -> &[ArchivedCapTableEntry]`



## sel4_capdl_initializer_types::spec::object::ArchivedRiscvIrqExtraInfo

*Struct*

An archived [`RiscvIrqExtraInfo`]

**Fields:**
- `trigger: <u8 as ::rkyv::Archive>::Archived` - The archived counterpart of [`RiscvIrqExtraInfo::trigger`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::ArchivedSchedContext

*Struct*

An archived [`SchedContext`]

**Fields:**
- `size_bits: <u8 as ::rkyv::Archive>::Archived` - The archived counterpart of [`SchedContext::size_bits`]
- `extra: <SchedContextExtraInfo as ::rkyv::Archive>::Archived` - The archived counterpart of [`SchedContext::extra`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::ArchivedSchedContextExtraInfo

*Struct*

An archived [`SchedContextExtraInfo`]

**Fields:**
- `period: <u64 as ::rkyv::Archive>::Archived` - The archived counterpart of [`SchedContextExtraInfo::period`]
- `budget: <u64 as ::rkyv::Archive>::Archived` - The archived counterpart of [`SchedContextExtraInfo::budget`]
- `badge: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`SchedContextExtraInfo::badge`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::ArchivedTcb

*Struct*

An archived [`Tcb`]

**Fields:**
- `slots: <alloc::vec::Vec<CapTableEntry> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Tcb::slots`]
- `extra: <alloc::boxed::Box<TcbExtraInfo> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Tcb::extra`]

**Methods:**

- `fn cspace(self: &Self) -> &<cap::CNode as Archive>::Archived`
- `fn vspace(self: &Self) -> &<cap::PageTable as Archive>::Archived`
- `fn ipc_buffer(self: &Self) -> &<cap::Frame as Archive>::Archived`
- `fn mcs_fault_ep(self: &Self) -> Option<&<cap::Endpoint as Archive>::Archived>`
- `fn sc(self: &Self) -> Option<&<cap::SchedContext as Archive>::Archived>`
- `fn temp_fault_ep(self: &Self) -> Option<&<cap::Endpoint as Archive>::Archived>`
- `fn bound_notification(self: &Self) -> Option<&<cap::Notification as Archive>::Archived>`
- `fn vcpu(self: &Self) -> Option<&<cap::VCpu as Archive>::Archived>`
- `fn x86_eptpml4(self: &Self) -> Option<&<cap::PageTable as Archive>::Archived>`

**Traits:** Portable

**Trait Implementations:**

- **HasArchivedCapTable**
  - `fn slots(self: &Self) -> &[ArchivedCapTableEntry]`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::ArchivedTcbExtraInfo

*Struct*

An archived [`TcbExtraInfo`]

**Fields:**
- `ipc_buffer_addr: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::ipc_buffer_addr`]
- `affinity: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::affinity`]
- `prio: <u8 as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::prio`]
- `max_prio: <u8 as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::max_prio`]
- `fpu_disabled: <bool as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::fpu_disabled`]
- `resume: <bool as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::resume`]
- `domain: <Option<u8> as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::domain`]
- `ip: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::ip`]
- `sp: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::sp`]
- `gprs: <alloc::vec::Vec<Word> as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::gprs`]
- `master_fault_ep: <Option<Word> as ::rkyv::Archive>::Archived` - The archived counterpart of [`TcbExtraInfo::master_fault_ep`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::ArchivedUntyped

*Struct*

An archived [`Untyped`]

**Fields:**
- `size_bits: <u8 as ::rkyv::Archive>::Archived` - The archived counterpart of [`Untyped::size_bits`]
- `paddr: <Option<Word> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Untyped::paddr`]

**Traits:** Portable

**Trait Implementations:**

- **IsArchivedObject**
  - `fn try_from_object(obj: &ArchivedObject<D>) -> Option<&Self>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::ArmIrq

*Struct*

**Fields:**
- `slots: alloc::vec::Vec<CapTableEntry>`
- `extra: alloc::boxed::Box<ArmIrqExtraInfo>`

**Methods:**

- `fn notification(self: &Self) -> Option<&cap::Notification>`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArmIrq) -> bool`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`
- **HasCapTable**
  - `fn slots(self: &Self) -> &[CapTableEntry]`
- **Clone**
  - `fn clone(self: &Self) -> ArmIrq`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::ArmIrqExtraInfo

*Struct*

**Fields:**
- `trigger: u8`
- `target: Word`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArmIrqExtraInfo) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ArmIrqExtraInfo`



## sel4_capdl_initializer_types::spec::object::ArmIrqExtraInfoResolver

*Struct*

The resolver for an archived [`ArmIrqExtraInfo`]



## sel4_capdl_initializer_types::spec::object::ArmIrqResolver

*Struct*

The resolver for an archived [`ArmIrq`]



## sel4_capdl_initializer_types::spec::object::AsidPool

*Struct*

**Fields:**
- `high: Word`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AsidPool`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &AsidPool) -> bool`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::AsidPoolResolver

*Struct*

The resolver for an archived [`AsidPool`]



## sel4_capdl_initializer_types::spec::object::CNode

*Struct*

**Fields:**
- `size_bits: u8`
- `slots: alloc::vec::Vec<CapTableEntry>`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CNode) -> bool`
- **HasCapTable**
  - `fn slots(self: &Self) -> &[CapTableEntry]`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> CNode`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::CNodeResolver

*Struct*

The resolver for an archived [`CNode`]



## sel4_capdl_initializer_types::spec::object::Frame

*Struct*

**Generic Parameters:**
- D

**Fields:**
- `size_bits: u8`
- `paddr: Option<Word>`
- `init: D`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Frame<D>) -> bool`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> Frame<D>`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::object::FrameResolver

*Struct*

The resolver for an archived [`Frame`]

**Generic Parameters:**
- D



## sel4_capdl_initializer_types::spec::object::IOPorts

*Struct*

**Fields:**
- `start_port: Word`
- `end_port: Word`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IOPorts) -> bool`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> IOPorts`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::object::IOPortsResolver

*Struct*

The resolver for an archived [`IOPorts`]



## sel4_capdl_initializer_types::spec::object::Irq

*Struct*

**Fields:**
- `slots: alloc::vec::Vec<CapTableEntry>`

**Methods:**

- `fn notification(self: &Self) -> Option<&cap::Notification>`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Irq`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **HasCapTable**
  - `fn slots(self: &Self) -> &[CapTableEntry]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Irq) -> bool`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::IrqIOApic

*Struct*

**Fields:**
- `slots: alloc::vec::Vec<CapTableEntry>`
- `extra: alloc::boxed::Box<IrqIOApicExtraInfo>`

**Methods:**

- `fn notification(self: &Self) -> Option<&cap::Notification>`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &IrqIOApic) -> bool`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> IrqIOApic`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **HasCapTable**
  - `fn slots(self: &Self) -> &[CapTableEntry]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_capdl_initializer_types::spec::object::IrqIOApicExtraInfo

*Struct*

**Fields:**
- `ioapic: Word`
- `pin: Word`
- `level: Word`
- `polarity: Word`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IrqIOApicExtraInfo) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> IrqIOApicExtraInfo`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::IrqIOApicExtraInfoResolver

*Struct*

The resolver for an archived [`IrqIOApicExtraInfo`]



## sel4_capdl_initializer_types::spec::object::IrqIOApicResolver

*Struct*

The resolver for an archived [`IrqIOApic`]



## sel4_capdl_initializer_types::spec::object::IrqMsi

*Struct*

**Fields:**
- `slots: alloc::vec::Vec<CapTableEntry>`
- `extra: alloc::boxed::Box<IrqMsiExtraInfo>`

**Methods:**

- `fn notification(self: &Self) -> Option<&cap::Notification>`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IrqMsi) -> bool`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> IrqMsi`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **HasCapTable**
  - `fn slots(self: &Self) -> &[CapTableEntry]`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::object::IrqMsiExtraInfo

*Struct*

**Fields:**
- `handle: Word`
- `pci_bus: Word`
- `pci_dev: Word`
- `pci_func: Word`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IrqMsiExtraInfo) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> IrqMsiExtraInfo`



## sel4_capdl_initializer_types::spec::object::IrqMsiExtraInfoResolver

*Struct*

The resolver for an archived [`IrqMsiExtraInfo`]



## sel4_capdl_initializer_types::spec::object::IrqMsiResolver

*Struct*

The resolver for an archived [`IrqMsi`]



## sel4_capdl_initializer_types::spec::object::IrqResolver

*Struct*

The resolver for an archived [`Irq`]



## sel4_capdl_initializer_types::spec::object::PageTable

*Struct*

**Fields:**
- `x86_ept: bool`
- `is_root: bool`
- `level: Option<u8>`
- `slots: alloc::vec::Vec<CapTableEntry>`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PageTable) -> bool`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> PageTable`
- **HasCapTable**
  - `fn slots(self: &Self) -> &[CapTableEntry]`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::PageTableResolver

*Struct*

The resolver for an archived [`PageTable`]



## sel4_capdl_initializer_types::spec::object::RiscvIrq

*Struct*

**Fields:**
- `slots: alloc::vec::Vec<CapTableEntry>`
- `extra: RiscvIrqExtraInfo`

**Methods:**

- `fn notification(self: &Self) -> Option<&cap::Notification>`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RiscvIrq`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RiscvIrq) -> bool`
- **HasCapTable**
  - `fn slots(self: &Self) -> &[CapTableEntry]`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::object::RiscvIrqExtraInfo

*Struct*

**Fields:**
- `trigger: u8`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RiscvIrqExtraInfo) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RiscvIrqExtraInfo`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::RiscvIrqExtraInfoResolver

*Struct*

The resolver for an archived [`RiscvIrqExtraInfo`]



## sel4_capdl_initializer_types::spec::object::RiscvIrqResolver

*Struct*

The resolver for an archived [`RiscvIrq`]



## sel4_capdl_initializer_types::spec::object::SchedContext

*Struct*

**Fields:**
- `size_bits: u8`
- `extra: SchedContextExtraInfo`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SchedContext) -> bool`
- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> SchedContext`



## sel4_capdl_initializer_types::spec::object::SchedContextExtraInfo

*Struct*

**Fields:**
- `period: u64`
- `budget: u64`
- `badge: Word`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SchedContextExtraInfo) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SchedContextExtraInfo`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::SchedContextExtraInfoResolver

*Struct*

The resolver for an archived [`SchedContextExtraInfo`]



## sel4_capdl_initializer_types::spec::object::SchedContextResolver

*Struct*

The resolver for an archived [`SchedContext`]



## sel4_capdl_initializer_types::spec::object::Tcb

*Struct*

**Fields:**
- `slots: alloc::vec::Vec<CapTableEntry>`
- `extra: alloc::boxed::Box<TcbExtraInfo>`

**Methods:**

- `fn cspace(self: &Self) -> &cap::CNode`
- `fn vspace(self: &Self) -> &cap::PageTable`
- `fn ipc_buffer(self: &Self) -> &cap::Frame`
- `fn mcs_fault_ep(self: &Self) -> Option<&cap::Endpoint>`
- `fn sc(self: &Self) -> Option<&cap::SchedContext>`
- `fn temp_fault_ep(self: &Self) -> Option<&cap::Endpoint>`
- `fn bound_notification(self: &Self) -> Option<&cap::Notification>`
- `fn vcpu(self: &Self) -> Option<&cap::VCpu>`
- `fn x86_eptpml4(self: &Self) -> Option<&cap::PageTable>`

**Traits:** Eq

**Trait Implementations:**

- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> Tcb`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **HasCapTable**
  - `fn slots(self: &Self) -> &[CapTableEntry]`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Tcb) -> bool`



## sel4_capdl_initializer_types::spec::object::TcbExtraInfo

*Struct*

**Fields:**
- `ipc_buffer_addr: Word`
- `affinity: Word`
- `prio: u8`
- `max_prio: u8`
- `fpu_disabled: bool`
- `resume: bool`
- `domain: Option<u8>`
- `ip: Word`
- `sp: Word`
- `gprs: alloc::vec::Vec<Word>`
- `master_fault_ep: Option<Word>`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TcbExtraInfo) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TcbExtraInfo`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::object::TcbExtraInfoResolver

*Struct*

The resolver for an archived [`TcbExtraInfo`]



## sel4_capdl_initializer_types::spec::object::TcbResolver

*Struct*

The resolver for an archived [`Tcb`]



## sel4_capdl_initializer_types::spec::object::Untyped

*Struct*

**Fields:**
- `size_bits: u8`
- `paddr: Option<Word>`

**Traits:** Eq

**Trait Implementations:**

- **IsObject**
  - `fn into_object(self: Self) -> Object<D>`
  - `fn try_from_object(obj: &Object<D>) -> Option<&Self>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Untyped) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Untyped`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::object::UntypedResolver

*Struct*

The resolver for an archived [`Untyped`]



