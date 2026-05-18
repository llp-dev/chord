**sel4_capdl_initializer_types > spec > cap**

# Module: spec::cap

## Contents

**Structs**

- [`ArchivedArmIrqHandler`](#archivedarmirqhandler) - An archived [`ArmIrqHandler`]
- [`ArchivedArmSmc`](#archivedarmsmc) - An archived [`ArmSmc`]
- [`ArchivedAsidPool`](#archivedasidpool) - An archived [`AsidPool`]
- [`ArchivedCNode`](#archivedcnode) - An archived [`CNode`]
- [`ArchivedDomainSet`](#archiveddomainset) - An archived [`DomainSet`]
- [`ArchivedEndpoint`](#archivedendpoint) - An archived [`Endpoint`]
- [`ArchivedFrame`](#archivedframe) - An archived [`Frame`]
- [`ArchivedIOPorts`](#archivedioports) - An archived [`IOPorts`]
- [`ArchivedIrqHandler`](#archivedirqhandler) - An archived [`IrqHandler`]
- [`ArchivedIrqIOApicHandler`](#archivedirqioapichandler) - An archived [`IrqIOApicHandler`]
- [`ArchivedIrqMsiHandler`](#archivedirqmsihandler) - An archived [`IrqMsiHandler`]
- [`ArchivedNotification`](#archivednotification) - An archived [`Notification`]
- [`ArchivedPageTable`](#archivedpagetable) - An archived [`PageTable`]
- [`ArchivedReply`](#archivedreply) - An archived [`Reply`]
- [`ArchivedRiscvIrqHandler`](#archivedriscvirqhandler) - An archived [`RiscvIrqHandler`]
- [`ArchivedSchedContext`](#archivedschedcontext) - An archived [`SchedContext`]
- [`ArchivedTcb`](#archivedtcb) - An archived [`Tcb`]
- [`ArchivedUntyped`](#archiveduntyped) - An archived [`Untyped`]
- [`ArchivedVCpu`](#archivedvcpu) - An archived [`VCpu`]
- [`ArmIrqHandler`](#armirqhandler)
- [`ArmIrqHandlerResolver`](#armirqhandlerresolver) - The resolver for an archived [`ArmIrqHandler`]
- [`ArmSmc`](#armsmc)
- [`ArmSmcResolver`](#armsmcresolver) - The resolver for an archived [`ArmSmc`]
- [`AsidPool`](#asidpool)
- [`AsidPoolResolver`](#asidpoolresolver) - The resolver for an archived [`AsidPool`]
- [`CNode`](#cnode)
- [`CNodeResolver`](#cnoderesolver) - The resolver for an archived [`CNode`]
- [`DomainSet`](#domainset)
- [`DomainSetResolver`](#domainsetresolver) - The resolver for an archived [`DomainSet`]
- [`Endpoint`](#endpoint)
- [`EndpointResolver`](#endpointresolver) - The resolver for an archived [`Endpoint`]
- [`Frame`](#frame)
- [`FrameResolver`](#frameresolver) - The resolver for an archived [`Frame`]
- [`IOPorts`](#ioports)
- [`IOPortsResolver`](#ioportsresolver) - The resolver for an archived [`IOPorts`]
- [`IrqHandler`](#irqhandler)
- [`IrqHandlerResolver`](#irqhandlerresolver) - The resolver for an archived [`IrqHandler`]
- [`IrqIOApicHandler`](#irqioapichandler)
- [`IrqIOApicHandlerResolver`](#irqioapichandlerresolver) - The resolver for an archived [`IrqIOApicHandler`]
- [`IrqMsiHandler`](#irqmsihandler)
- [`IrqMsiHandlerResolver`](#irqmsihandlerresolver) - The resolver for an archived [`IrqMsiHandler`]
- [`Notification`](#notification)
- [`NotificationResolver`](#notificationresolver) - The resolver for an archived [`Notification`]
- [`PageTable`](#pagetable)
- [`PageTableResolver`](#pagetableresolver) - The resolver for an archived [`PageTable`]
- [`Reply`](#reply)
- [`ReplyResolver`](#replyresolver) - The resolver for an archived [`Reply`]
- [`RiscvIrqHandler`](#riscvirqhandler)
- [`RiscvIrqHandlerResolver`](#riscvirqhandlerresolver) - The resolver for an archived [`RiscvIrqHandler`]
- [`SchedContext`](#schedcontext)
- [`SchedContextResolver`](#schedcontextresolver) - The resolver for an archived [`SchedContext`]
- [`Tcb`](#tcb)
- [`TcbResolver`](#tcbresolver) - The resolver for an archived [`Tcb`]
- [`Untyped`](#untyped)
- [`UntypedResolver`](#untypedresolver) - The resolver for an archived [`Untyped`]
- [`VCpu`](#vcpu)
- [`VCpuResolver`](#vcpuresolver) - The resolver for an archived [`VCpu`]

---

## sel4_capdl_initializer_types::spec::cap::ArchivedArmIrqHandler

*Struct*

An archived [`ArmIrqHandler`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`ArmIrqHandler::object`]

**Traits:** Portable

**Trait Implementations:**

- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::cap::ArchivedArmSmc

*Struct*

An archived [`ArmSmc`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`ArmSmc::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedAsidPool

*Struct*

An archived [`AsidPool`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`AsidPool::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedCNode

*Struct*

An archived [`CNode`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`CNode::object`]
- `guard: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`CNode::guard`]
- `guard_size: <u8 as ::rkyv::Archive>::Archived` - The archived counterpart of [`CNode::guard_size`]

**Traits:** Portable

**Trait Implementations:**

- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::cap::ArchivedDomainSet

*Struct*

An archived [`DomainSet`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`DomainSet::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedEndpoint

*Struct*

An archived [`Endpoint`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`Endpoint::object`]
- `badge: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`Endpoint::badge`]
- `rights: <Rights as ::rkyv::Archive>::Archived` - The archived counterpart of [`Endpoint::rights`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedFrame

*Struct*

An archived [`Frame`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`Frame::object`]
- `rights: <Rights as ::rkyv::Archive>::Archived` - The archived counterpart of [`Frame::rights`]
- `cached: <bool as ::rkyv::Archive>::Archived` - The archived counterpart of [`Frame::cached`]
- `executable: <bool as ::rkyv::Archive>::Archived` - The archived counterpart of [`Frame::executable`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedIOPorts

*Struct*

An archived [`IOPorts`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`IOPorts::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedIrqHandler

*Struct*

An archived [`IrqHandler`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqHandler::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedIrqIOApicHandler

*Struct*

An archived [`IrqIOApicHandler`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqIOApicHandler::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedIrqMsiHandler

*Struct*

An archived [`IrqMsiHandler`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqMsiHandler::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedNotification

*Struct*

An archived [`Notification`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`Notification::object`]
- `badge: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`Notification::badge`]
- `rights: <Rights as ::rkyv::Archive>::Archived` - The archived counterpart of [`Notification::rights`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedPageTable

*Struct*

An archived [`PageTable`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`PageTable::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedReply

*Struct*

An archived [`Reply`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`Reply::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedRiscvIrqHandler

*Struct*

An archived [`RiscvIrqHandler`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`RiscvIrqHandler::object`]

**Traits:** Portable

**Trait Implementations:**

- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::cap::ArchivedSchedContext

*Struct*

An archived [`SchedContext`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`SchedContext::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArchivedTcb

*Struct*

An archived [`Tcb`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`Tcb::object`]

**Traits:** Portable

**Trait Implementations:**

- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::cap::ArchivedUntyped

*Struct*

An archived [`Untyped`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`Untyped::object`]

**Traits:** Portable

**Trait Implementations:**

- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::cap::ArchivedVCpu

*Struct*

An archived [`VCpu`]

**Fields:**
- `object: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`VCpu::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **IsArchivedCap**
  - `fn try_from_cap(cap: &ArchivedCap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ArmIrqHandler

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArmIrqHandler) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> ArmIrqHandler`



## sel4_capdl_initializer_types::spec::cap::ArmIrqHandlerResolver

*Struct*

The resolver for an archived [`ArmIrqHandler`]



## sel4_capdl_initializer_types::spec::cap::ArmSmc

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArmSmc) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> ArmSmc`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::cap::ArmSmcResolver

*Struct*

The resolver for an archived [`ArmSmc`]



## sel4_capdl_initializer_types::spec::cap::AsidPool

*Struct*

**Fields:**
- `object: ObjectId`

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
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::AsidPoolResolver

*Struct*

The resolver for an archived [`AsidPool`]



## sel4_capdl_initializer_types::spec::cap::CNode

*Struct*

**Fields:**
- `object: ObjectId`
- `guard: Word`
- `guard_size: u8`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CNode) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> CNode`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::cap::CNodeResolver

*Struct*

The resolver for an archived [`CNode`]



## sel4_capdl_initializer_types::spec::cap::DomainSet

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DomainSet) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> DomainSet`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_capdl_initializer_types::spec::cap::DomainSetResolver

*Struct*

The resolver for an archived [`DomainSet`]



## sel4_capdl_initializer_types::spec::cap::Endpoint

*Struct*

**Fields:**
- `object: ObjectId`
- `badge: Word`
- `rights: Rights`

**Traits:** Eq

**Trait Implementations:**

- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> Endpoint`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Endpoint) -> bool`



## sel4_capdl_initializer_types::spec::cap::EndpointResolver

*Struct*

The resolver for an archived [`Endpoint`]



## sel4_capdl_initializer_types::spec::cap::Frame

*Struct*

**Fields:**
- `object: ObjectId`
- `rights: Rights`
- `cached: bool`
- `executable: bool`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Frame) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> Frame`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::cap::FrameResolver

*Struct*

The resolver for an archived [`Frame`]



## sel4_capdl_initializer_types::spec::cap::IOPorts

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IOPorts) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> IOPorts`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::cap::IOPortsResolver

*Struct*

The resolver for an archived [`IOPorts`]



## sel4_capdl_initializer_types::spec::cap::IrqHandler

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IrqHandler) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> IrqHandler`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::cap::IrqHandlerResolver

*Struct*

The resolver for an archived [`IrqHandler`]



## sel4_capdl_initializer_types::spec::cap::IrqIOApicHandler

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> IrqIOApicHandler`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IrqIOApicHandler) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::IrqIOApicHandlerResolver

*Struct*

The resolver for an archived [`IrqIOApicHandler`]



## sel4_capdl_initializer_types::spec::cap::IrqMsiHandler

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IrqMsiHandler) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> IrqMsiHandler`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::cap::IrqMsiHandlerResolver

*Struct*

The resolver for an archived [`IrqMsiHandler`]



## sel4_capdl_initializer_types::spec::cap::Notification

*Struct*

**Fields:**
- `object: ObjectId`
- `badge: Word`
- `rights: Rights`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Notification) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> Notification`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_capdl_initializer_types::spec::cap::NotificationResolver

*Struct*

The resolver for an archived [`Notification`]



## sel4_capdl_initializer_types::spec::cap::PageTable

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PageTable) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> PageTable`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::cap::PageTableResolver

*Struct*

The resolver for an archived [`PageTable`]



## sel4_capdl_initializer_types::spec::cap::Reply

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Reply`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Reply) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`



## sel4_capdl_initializer_types::spec::cap::ReplyResolver

*Struct*

The resolver for an archived [`Reply`]



## sel4_capdl_initializer_types::spec::cap::RiscvIrqHandler

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RiscvIrqHandler) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> RiscvIrqHandler`



## sel4_capdl_initializer_types::spec::cap::RiscvIrqHandlerResolver

*Struct*

The resolver for an archived [`RiscvIrqHandler`]



## sel4_capdl_initializer_types::spec::cap::SchedContext

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SchedContext) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> SchedContext`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::cap::SchedContextResolver

*Struct*

The resolver for an archived [`SchedContext`]



## sel4_capdl_initializer_types::spec::cap::Tcb

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Tcb) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> Tcb`



## sel4_capdl_initializer_types::spec::cap::TcbResolver

*Struct*

The resolver for an archived [`Tcb`]



## sel4_capdl_initializer_types::spec::cap::Untyped

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Untyped) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Untyped`



## sel4_capdl_initializer_types::spec::cap::UntypedResolver

*Struct*

The resolver for an archived [`Untyped`]



## sel4_capdl_initializer_types::spec::cap::VCpu

*Struct*

**Fields:**
- `object: ObjectId`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &VCpu) -> bool`
- **IsCap**
  - `fn into_cap(self: Self) -> Cap`
  - `fn try_from_cap(cap: &Cap) -> Option<&Self>`
- **Clone**
  - `fn clone(self: &Self) -> VCpu`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`



## sel4_capdl_initializer_types::spec::cap::VCpuResolver

*Struct*

The resolver for an archived [`VCpu`]



