*[libc](../../../../index.md) / [new](../../../index.md) / [linux_uapi](../../index.md) / [linux](../index.md) / [netlink](index.md)*

---

# Module `netlink`

Header: `uapi/linux/netlink.h`

## Contents

- [Structs](#structs)
  - [`sockaddr_nl`](#sockaddr-nl)
  - [`nlmsghdr`](#nlmsghdr)
  - [`nlmsgerr`](#nlmsgerr)
  - [`nl_pktinfo`](#nl-pktinfo)
  - [`nl_mmap_req`](#nl-mmap-req)
  - [`nl_mmap_hdr`](#nl-mmap-hdr)
  - [`nlattr`](#nlattr)
- [Functions](#functions)
  - [`NLA_ALIGN`](#nla-align)
- [Constants](#constants)
  - [`NETLINK_ROUTE`](#netlink-route)
  - [`NETLINK_UNUSED`](#netlink-unused)
  - [`NETLINK_USERSOCK`](#netlink-usersock)
  - [`NETLINK_FIREWALL`](#netlink-firewall)
  - [`NETLINK_SOCK_DIAG`](#netlink-sock-diag)
  - [`NETLINK_NFLOG`](#netlink-nflog)
  - [`NETLINK_XFRM`](#netlink-xfrm)
  - [`NETLINK_SELINUX`](#netlink-selinux)
  - [`NETLINK_ISCSI`](#netlink-iscsi)
  - [`NETLINK_AUDIT`](#netlink-audit)
  - [`NETLINK_FIB_LOOKUP`](#netlink-fib-lookup)
  - [`NETLINK_CONNECTOR`](#netlink-connector)
  - [`NETLINK_NETFILTER`](#netlink-netfilter)
  - [`NETLINK_IP6_FW`](#netlink-ip6-fw)
  - [`NETLINK_DNRTMSG`](#netlink-dnrtmsg)
  - [`NETLINK_KOBJECT_UEVENT`](#netlink-kobject-uevent)
  - [`NETLINK_GENERIC`](#netlink-generic)
  - [`NETLINK_SCSITRANSPORT`](#netlink-scsitransport)
  - [`NETLINK_ECRYPTFS`](#netlink-ecryptfs)
  - [`NETLINK_RDMA`](#netlink-rdma)
  - [`NETLINK_CRYPTO`](#netlink-crypto)
  - [`NETLINK_INET_DIAG`](#netlink-inet-diag)
  - [`MAX_LINKS`](#max-links)
  - [`NLM_F_REQUEST`](#nlm-f-request)
  - [`NLM_F_MULTI`](#nlm-f-multi)
  - [`NLM_F_ACK`](#nlm-f-ack)
  - [`NLM_F_ECHO`](#nlm-f-echo)
  - [`NLM_F_DUMP_INTR`](#nlm-f-dump-intr)
  - [`NLM_F_DUMP_FILTERED`](#nlm-f-dump-filtered)
  - [`NLM_F_ROOT`](#nlm-f-root)
  - [`NLM_F_MATCH`](#nlm-f-match)
  - [`NLM_F_ATOMIC`](#nlm-f-atomic)
  - [`NLM_F_DUMP`](#nlm-f-dump)
  - [`NLM_F_REPLACE`](#nlm-f-replace)
  - [`NLM_F_EXCL`](#nlm-f-excl)
  - [`NLM_F_CREATE`](#nlm-f-create)
  - [`NLM_F_APPEND`](#nlm-f-append)
  - [`NLM_F_NONREC`](#nlm-f-nonrec)
  - [`NLM_F_CAPPED`](#nlm-f-capped)
  - [`NLM_F_ACK_TLVS`](#nlm-f-ack-tlvs)
  - [`NLMSG_NOOP`](#nlmsg-noop)
  - [`NLMSG_ERROR`](#nlmsg-error)
  - [`NLMSG_DONE`](#nlmsg-done)
  - [`NLMSG_OVERRUN`](#nlmsg-overrun)
  - [`NLMSG_MIN_TYPE`](#nlmsg-min-type)
  - [`NETLINK_ADD_MEMBERSHIP`](#netlink-add-membership)
  - [`NETLINK_DROP_MEMBERSHIP`](#netlink-drop-membership)
  - [`NETLINK_PKTINFO`](#netlink-pktinfo)
  - [`NETLINK_BROADCAST_ERROR`](#netlink-broadcast-error)
  - [`NETLINK_NO_ENOBUFS`](#netlink-no-enobufs)
  - [`NETLINK_RX_RING`](#netlink-rx-ring)
  - [`NETLINK_TX_RING`](#netlink-tx-ring)
  - [`NETLINK_LISTEN_ALL_NSID`](#netlink-listen-all-nsid)
  - [`NETLINK_LIST_MEMBERSHIPS`](#netlink-list-memberships)
  - [`NETLINK_CAP_ACK`](#netlink-cap-ack)
  - [`NETLINK_EXT_ACK`](#netlink-ext-ack)
  - [`NETLINK_GET_STRICT_CHK`](#netlink-get-strict-chk)
  - [`NLA_F_NESTED`](#nla-f-nested)
  - [`NLA_F_NET_BYTEORDER`](#nla-f-net-byteorder)
  - [`NLA_TYPE_MASK`](#nla-type-mask)
  - [`NLA_ALIGNTO`](#nla-alignto)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sockaddr_nl`](#sockaddr-nl) | struct |  |
| [`nlmsghdr`](#nlmsghdr) | struct |  |
| [`nlmsgerr`](#nlmsgerr) | struct |  |
| [`nl_pktinfo`](#nl-pktinfo) | struct |  |
| [`nl_mmap_req`](#nl-mmap-req) | struct |  |
| [`nl_mmap_hdr`](#nl-mmap-hdr) | struct |  |
| [`nlattr`](#nlattr) | struct |  |
| [`NLA_ALIGN`](#nla-align) | fn |  |
| [`NETLINK_ROUTE`](#netlink-route) | const |  |
| [`NETLINK_UNUSED`](#netlink-unused) | const |  |
| [`NETLINK_USERSOCK`](#netlink-usersock) | const |  |
| [`NETLINK_FIREWALL`](#netlink-firewall) | const |  |
| [`NETLINK_SOCK_DIAG`](#netlink-sock-diag) | const |  |
| [`NETLINK_NFLOG`](#netlink-nflog) | const |  |
| [`NETLINK_XFRM`](#netlink-xfrm) | const |  |
| [`NETLINK_SELINUX`](#netlink-selinux) | const |  |
| [`NETLINK_ISCSI`](#netlink-iscsi) | const |  |
| [`NETLINK_AUDIT`](#netlink-audit) | const |  |
| [`NETLINK_FIB_LOOKUP`](#netlink-fib-lookup) | const |  |
| [`NETLINK_CONNECTOR`](#netlink-connector) | const |  |
| [`NETLINK_NETFILTER`](#netlink-netfilter) | const |  |
| [`NETLINK_IP6_FW`](#netlink-ip6-fw) | const |  |
| [`NETLINK_DNRTMSG`](#netlink-dnrtmsg) | const |  |
| [`NETLINK_KOBJECT_UEVENT`](#netlink-kobject-uevent) | const |  |
| [`NETLINK_GENERIC`](#netlink-generic) | const |  |
| [`NETLINK_SCSITRANSPORT`](#netlink-scsitransport) | const |  |
| [`NETLINK_ECRYPTFS`](#netlink-ecryptfs) | const |  |
| [`NETLINK_RDMA`](#netlink-rdma) | const |  |
| [`NETLINK_CRYPTO`](#netlink-crypto) | const |  |
| [`NETLINK_INET_DIAG`](#netlink-inet-diag) | const |  |
| [`MAX_LINKS`](#max-links) | const |  |
| [`NLM_F_REQUEST`](#nlm-f-request) | const |  |
| [`NLM_F_MULTI`](#nlm-f-multi) | const |  |
| [`NLM_F_ACK`](#nlm-f-ack) | const |  |
| [`NLM_F_ECHO`](#nlm-f-echo) | const |  |
| [`NLM_F_DUMP_INTR`](#nlm-f-dump-intr) | const |  |
| [`NLM_F_DUMP_FILTERED`](#nlm-f-dump-filtered) | const |  |
| [`NLM_F_ROOT`](#nlm-f-root) | const |  |
| [`NLM_F_MATCH`](#nlm-f-match) | const |  |
| [`NLM_F_ATOMIC`](#nlm-f-atomic) | const |  |
| [`NLM_F_DUMP`](#nlm-f-dump) | const |  |
| [`NLM_F_REPLACE`](#nlm-f-replace) | const |  |
| [`NLM_F_EXCL`](#nlm-f-excl) | const |  |
| [`NLM_F_CREATE`](#nlm-f-create) | const |  |
| [`NLM_F_APPEND`](#nlm-f-append) | const |  |
| [`NLM_F_NONREC`](#nlm-f-nonrec) | const |  |
| [`NLM_F_CAPPED`](#nlm-f-capped) | const |  |
| [`NLM_F_ACK_TLVS`](#nlm-f-ack-tlvs) | const |  |
| [`NLMSG_NOOP`](#nlmsg-noop) | const |  |
| [`NLMSG_ERROR`](#nlmsg-error) | const |  |
| [`NLMSG_DONE`](#nlmsg-done) | const |  |
| [`NLMSG_OVERRUN`](#nlmsg-overrun) | const |  |
| [`NLMSG_MIN_TYPE`](#nlmsg-min-type) | const |  |
| [`NETLINK_ADD_MEMBERSHIP`](#netlink-add-membership) | const |  |
| [`NETLINK_DROP_MEMBERSHIP`](#netlink-drop-membership) | const |  |
| [`NETLINK_PKTINFO`](#netlink-pktinfo) | const |  |
| [`NETLINK_BROADCAST_ERROR`](#netlink-broadcast-error) | const |  |
| [`NETLINK_NO_ENOBUFS`](#netlink-no-enobufs) | const |  |
| [`NETLINK_RX_RING`](#netlink-rx-ring) | const |  |
| [`NETLINK_TX_RING`](#netlink-tx-ring) | const |  |
| [`NETLINK_LISTEN_ALL_NSID`](#netlink-listen-all-nsid) | const |  |
| [`NETLINK_LIST_MEMBERSHIPS`](#netlink-list-memberships) | const |  |
| [`NETLINK_CAP_ACK`](#netlink-cap-ack) | const |  |
| [`NETLINK_EXT_ACK`](#netlink-ext-ack) | const |  |
| [`NETLINK_GET_STRICT_CHK`](#netlink-get-strict-chk) | const |  |
| [`NLA_F_NESTED`](#nla-f-nested) | const |  |
| [`NLA_F_NET_BYTEORDER`](#nla-f-net-byteorder) | const |  |
| [`NLA_TYPE_MASK`](#nla-type-mask) | const |  |
| [`NLA_ALIGNTO`](#nla-alignto) | const |  |

## Structs

### `sockaddr_nl`

```rust
struct sockaddr_nl {
    pub nl_family: crate::sa_family_t,
    nl_pad: Padding<c_ushort>,
    pub nl_pid: u32,
    pub nl_groups: u32,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_nl`

- <span id="sockaddr-nl-clone"></span>`fn clone(&self) -> sockaddr_nl` — [`sockaddr_nl`](../../../index.md#sockaddr-nl)

##### `impl Copy for sockaddr_nl`

##### `impl Debug for sockaddr_nl`

- <span id="sockaddr-nl-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `nlmsghdr`

```rust
struct nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}
```

#### Trait Implementations

##### `impl Clone for nlmsghdr`

- <span id="nlmsghdr-clone"></span>`fn clone(&self) -> nlmsghdr` — [`nlmsghdr`](../../../index.md#nlmsghdr)

##### `impl Copy for nlmsghdr`

##### `impl Debug for nlmsghdr`

- <span id="nlmsghdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `nlmsgerr`

```rust
struct nlmsgerr {
    pub error: c_int,
    pub msg: nlmsghdr,
}
```

#### Trait Implementations

##### `impl Clone for nlmsgerr`

- <span id="nlmsgerr-clone"></span>`fn clone(&self) -> nlmsgerr` — [`nlmsgerr`](../../../index.md#nlmsgerr)

##### `impl Copy for nlmsgerr`

##### `impl Debug for nlmsgerr`

- <span id="nlmsgerr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `nl_pktinfo`

```rust
struct nl_pktinfo {
    pub group: u32,
}
```

#### Trait Implementations

##### `impl Clone for nl_pktinfo`

- <span id="nl-pktinfo-clone"></span>`fn clone(&self) -> nl_pktinfo` — [`nl_pktinfo`](../../../index.md#nl-pktinfo)

##### `impl Copy for nl_pktinfo`

##### `impl Debug for nl_pktinfo`

- <span id="nl-pktinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `nl_mmap_req`

```rust
struct nl_mmap_req {
    pub nm_block_size: c_uint,
    pub nm_block_nr: c_uint,
    pub nm_frame_size: c_uint,
    pub nm_frame_nr: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for nl_mmap_req`

- <span id="nl-mmap-req-clone"></span>`fn clone(&self) -> nl_mmap_req` — [`nl_mmap_req`](../../../index.md#nl-mmap-req)

##### `impl Copy for nl_mmap_req`

##### `impl Debug for nl_mmap_req`

- <span id="nl-mmap-req-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `nl_mmap_hdr`

```rust
struct nl_mmap_hdr {
    pub nm_status: c_uint,
    pub nm_len: c_uint,
    pub nm_group: u32,
    pub nm_pid: u32,
    pub nm_uid: u32,
    pub nm_gid: u32,
}
```

#### Trait Implementations

##### `impl Clone for nl_mmap_hdr`

- <span id="nl-mmap-hdr-clone"></span>`fn clone(&self) -> nl_mmap_hdr` — [`nl_mmap_hdr`](../../../index.md#nl-mmap-hdr)

##### `impl Copy for nl_mmap_hdr`

##### `impl Debug for nl_mmap_hdr`

- <span id="nl-mmap-hdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `nlattr`

```rust
struct nlattr {
    pub nla_len: u16,
    pub nla_type: u16,
}
```

#### Trait Implementations

##### `impl Clone for nlattr`

- <span id="nlattr-clone"></span>`fn clone(&self) -> nlattr` — [`nlattr`](../../../index.md#nlattr)

##### `impl Copy for nlattr`

##### `impl Debug for nlattr`

- <span id="nlattr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `NLA_ALIGN`

```rust
unsafe fn NLA_ALIGN(len: c_int) -> c_int
```

## Constants

### `NETLINK_ROUTE`
```rust
const NETLINK_ROUTE: c_int = 0i32;
```

### `NETLINK_UNUSED`
```rust
const NETLINK_UNUSED: c_int = 1i32;
```

### `NETLINK_USERSOCK`
```rust
const NETLINK_USERSOCK: c_int = 2i32;
```

### `NETLINK_FIREWALL`
```rust
const NETLINK_FIREWALL: c_int = 3i32;
```

### `NETLINK_SOCK_DIAG`
```rust
const NETLINK_SOCK_DIAG: c_int = 4i32;
```

### `NETLINK_NFLOG`
```rust
const NETLINK_NFLOG: c_int = 5i32;
```

### `NETLINK_XFRM`
```rust
const NETLINK_XFRM: c_int = 6i32;
```

### `NETLINK_SELINUX`
```rust
const NETLINK_SELINUX: c_int = 7i32;
```

### `NETLINK_ISCSI`
```rust
const NETLINK_ISCSI: c_int = 8i32;
```

### `NETLINK_AUDIT`
```rust
const NETLINK_AUDIT: c_int = 9i32;
```

### `NETLINK_FIB_LOOKUP`
```rust
const NETLINK_FIB_LOOKUP: c_int = 10i32;
```

### `NETLINK_CONNECTOR`
```rust
const NETLINK_CONNECTOR: c_int = 11i32;
```

### `NETLINK_NETFILTER`
```rust
const NETLINK_NETFILTER: c_int = 12i32;
```

### `NETLINK_IP6_FW`
```rust
const NETLINK_IP6_FW: c_int = 13i32;
```

### `NETLINK_DNRTMSG`
```rust
const NETLINK_DNRTMSG: c_int = 14i32;
```

### `NETLINK_KOBJECT_UEVENT`
```rust
const NETLINK_KOBJECT_UEVENT: c_int = 15i32;
```

### `NETLINK_GENERIC`
```rust
const NETLINK_GENERIC: c_int = 16i32;
```

### `NETLINK_SCSITRANSPORT`
```rust
const NETLINK_SCSITRANSPORT: c_int = 18i32;
```

### `NETLINK_ECRYPTFS`
```rust
const NETLINK_ECRYPTFS: c_int = 19i32;
```

### `NETLINK_RDMA`
```rust
const NETLINK_RDMA: c_int = 20i32;
```

### `NETLINK_CRYPTO`
```rust
const NETLINK_CRYPTO: c_int = 21i32;
```

### `NETLINK_INET_DIAG`
```rust
const NETLINK_INET_DIAG: c_int = 4i32;
```

### `MAX_LINKS`
```rust
const MAX_LINKS: c_int = 32i32;
```

### `NLM_F_REQUEST`
```rust
const NLM_F_REQUEST: c_int = 1i32;
```

### `NLM_F_MULTI`
```rust
const NLM_F_MULTI: c_int = 2i32;
```

### `NLM_F_ACK`
```rust
const NLM_F_ACK: c_int = 4i32;
```

### `NLM_F_ECHO`
```rust
const NLM_F_ECHO: c_int = 8i32;
```

### `NLM_F_DUMP_INTR`
```rust
const NLM_F_DUMP_INTR: c_int = 16i32;
```

### `NLM_F_DUMP_FILTERED`
```rust
const NLM_F_DUMP_FILTERED: c_int = 32i32;
```

### `NLM_F_ROOT`
```rust
const NLM_F_ROOT: c_int = 256i32;
```

### `NLM_F_MATCH`
```rust
const NLM_F_MATCH: c_int = 512i32;
```

### `NLM_F_ATOMIC`
```rust
const NLM_F_ATOMIC: c_int = 1_024i32;
```

### `NLM_F_DUMP`
```rust
const NLM_F_DUMP: c_int = 768i32;
```

### `NLM_F_REPLACE`
```rust
const NLM_F_REPLACE: c_int = 256i32;
```

### `NLM_F_EXCL`
```rust
const NLM_F_EXCL: c_int = 512i32;
```

### `NLM_F_CREATE`
```rust
const NLM_F_CREATE: c_int = 1_024i32;
```

### `NLM_F_APPEND`
```rust
const NLM_F_APPEND: c_int = 2_048i32;
```

### `NLM_F_NONREC`
```rust
const NLM_F_NONREC: c_int = 256i32;
```

### `NLM_F_CAPPED`
```rust
const NLM_F_CAPPED: c_int = 256i32;
```

### `NLM_F_ACK_TLVS`
```rust
const NLM_F_ACK_TLVS: c_int = 512i32;
```

### `NLMSG_NOOP`
```rust
const NLMSG_NOOP: c_int = 1i32;
```

### `NLMSG_ERROR`
```rust
const NLMSG_ERROR: c_int = 2i32;
```

### `NLMSG_DONE`
```rust
const NLMSG_DONE: c_int = 3i32;
```

### `NLMSG_OVERRUN`
```rust
const NLMSG_OVERRUN: c_int = 4i32;
```

### `NLMSG_MIN_TYPE`
```rust
const NLMSG_MIN_TYPE: c_int = 16i32;
```

### `NETLINK_ADD_MEMBERSHIP`
```rust
const NETLINK_ADD_MEMBERSHIP: c_int = 1i32;
```

### `NETLINK_DROP_MEMBERSHIP`
```rust
const NETLINK_DROP_MEMBERSHIP: c_int = 2i32;
```

### `NETLINK_PKTINFO`
```rust
const NETLINK_PKTINFO: c_int = 3i32;
```

### `NETLINK_BROADCAST_ERROR`
```rust
const NETLINK_BROADCAST_ERROR: c_int = 4i32;
```

### `NETLINK_NO_ENOBUFS`
```rust
const NETLINK_NO_ENOBUFS: c_int = 5i32;
```

### `NETLINK_RX_RING`
```rust
const NETLINK_RX_RING: c_int = 6i32;
```

### `NETLINK_TX_RING`
```rust
const NETLINK_TX_RING: c_int = 7i32;
```

### `NETLINK_LISTEN_ALL_NSID`
```rust
const NETLINK_LISTEN_ALL_NSID: c_int = 8i32;
```

### `NETLINK_LIST_MEMBERSHIPS`
```rust
const NETLINK_LIST_MEMBERSHIPS: c_int = 9i32;
```

### `NETLINK_CAP_ACK`
```rust
const NETLINK_CAP_ACK: c_int = 10i32;
```

### `NETLINK_EXT_ACK`
```rust
const NETLINK_EXT_ACK: c_int = 11i32;
```

### `NETLINK_GET_STRICT_CHK`
```rust
const NETLINK_GET_STRICT_CHK: c_int = 12i32;
```

### `NLA_F_NESTED`
```rust
const NLA_F_NESTED: c_int = 32_768i32;
```

### `NLA_F_NET_BYTEORDER`
```rust
const NLA_F_NET_BYTEORDER: c_int = 16_384i32;
```

### `NLA_TYPE_MASK`
```rust
const NLA_TYPE_MASK: c_int = -49_153i32;
```

### `NLA_ALIGNTO`
```rust
const NLA_ALIGNTO: c_int = 4i32;
```

