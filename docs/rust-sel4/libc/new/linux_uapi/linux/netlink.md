**libc > new > linux_uapi > linux > netlink**

# Module: new::linux_uapi::linux::netlink

## Contents

**Structs**

- [`nl_mmap_hdr`](#nl_mmap_hdr)
- [`nl_mmap_req`](#nl_mmap_req)
- [`nl_pktinfo`](#nl_pktinfo)
- [`nlattr`](#nlattr)
- [`nlmsgerr`](#nlmsgerr)
- [`nlmsghdr`](#nlmsghdr)
- [`sockaddr_nl`](#sockaddr_nl)

**Functions**

- [`NLA_ALIGN`](#nla_align)

**Constants**

- [`MAX_LINKS`](#max_links)
- [`NETLINK_ADD_MEMBERSHIP`](#netlink_add_membership)
- [`NETLINK_AUDIT`](#netlink_audit)
- [`NETLINK_BROADCAST_ERROR`](#netlink_broadcast_error)
- [`NETLINK_CAP_ACK`](#netlink_cap_ack)
- [`NETLINK_CONNECTOR`](#netlink_connector)
- [`NETLINK_CRYPTO`](#netlink_crypto)
- [`NETLINK_DNRTMSG`](#netlink_dnrtmsg)
- [`NETLINK_DROP_MEMBERSHIP`](#netlink_drop_membership)
- [`NETLINK_ECRYPTFS`](#netlink_ecryptfs)
- [`NETLINK_EXT_ACK`](#netlink_ext_ack)
- [`NETLINK_FIB_LOOKUP`](#netlink_fib_lookup)
- [`NETLINK_FIREWALL`](#netlink_firewall)
- [`NETLINK_GENERIC`](#netlink_generic)
- [`NETLINK_GET_STRICT_CHK`](#netlink_get_strict_chk)
- [`NETLINK_INET_DIAG`](#netlink_inet_diag)
- [`NETLINK_IP6_FW`](#netlink_ip6_fw)
- [`NETLINK_ISCSI`](#netlink_iscsi)
- [`NETLINK_KOBJECT_UEVENT`](#netlink_kobject_uevent)
- [`NETLINK_LISTEN_ALL_NSID`](#netlink_listen_all_nsid)
- [`NETLINK_LIST_MEMBERSHIPS`](#netlink_list_memberships)
- [`NETLINK_NETFILTER`](#netlink_netfilter)
- [`NETLINK_NFLOG`](#netlink_nflog)
- [`NETLINK_NO_ENOBUFS`](#netlink_no_enobufs)
- [`NETLINK_PKTINFO`](#netlink_pktinfo)
- [`NETLINK_RDMA`](#netlink_rdma)
- [`NETLINK_ROUTE`](#netlink_route)
- [`NETLINK_RX_RING`](#netlink_rx_ring)
- [`NETLINK_SCSITRANSPORT`](#netlink_scsitransport)
- [`NETLINK_SELINUX`](#netlink_selinux)
- [`NETLINK_SOCK_DIAG`](#netlink_sock_diag)
- [`NETLINK_TX_RING`](#netlink_tx_ring)
- [`NETLINK_UNUSED`](#netlink_unused)
- [`NETLINK_USERSOCK`](#netlink_usersock)
- [`NETLINK_XFRM`](#netlink_xfrm)
- [`NLA_ALIGNTO`](#nla_alignto)
- [`NLA_F_NESTED`](#nla_f_nested)
- [`NLA_F_NET_BYTEORDER`](#nla_f_net_byteorder)
- [`NLA_TYPE_MASK`](#nla_type_mask)
- [`NLMSG_DONE`](#nlmsg_done)
- [`NLMSG_ERROR`](#nlmsg_error)
- [`NLMSG_MIN_TYPE`](#nlmsg_min_type)
- [`NLMSG_NOOP`](#nlmsg_noop)
- [`NLMSG_OVERRUN`](#nlmsg_overrun)
- [`NLM_F_ACK`](#nlm_f_ack)
- [`NLM_F_ACK_TLVS`](#nlm_f_ack_tlvs)
- [`NLM_F_APPEND`](#nlm_f_append)
- [`NLM_F_ATOMIC`](#nlm_f_atomic)
- [`NLM_F_CAPPED`](#nlm_f_capped)
- [`NLM_F_CREATE`](#nlm_f_create)
- [`NLM_F_DUMP`](#nlm_f_dump)
- [`NLM_F_DUMP_FILTERED`](#nlm_f_dump_filtered)
- [`NLM_F_DUMP_INTR`](#nlm_f_dump_intr)
- [`NLM_F_ECHO`](#nlm_f_echo)
- [`NLM_F_EXCL`](#nlm_f_excl)
- [`NLM_F_MATCH`](#nlm_f_match)
- [`NLM_F_MULTI`](#nlm_f_multi)
- [`NLM_F_NONREC`](#nlm_f_nonrec)
- [`NLM_F_REPLACE`](#nlm_f_replace)
- [`NLM_F_REQUEST`](#nlm_f_request)
- [`NLM_F_ROOT`](#nlm_f_root)

---

## libc::new::linux_uapi::linux::netlink::MAX_LINKS

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_ADD_MEMBERSHIP

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_AUDIT

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_BROADCAST_ERROR

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_CAP_ACK

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_CONNECTOR

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_CRYPTO

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_DNRTMSG

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_DROP_MEMBERSHIP

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_ECRYPTFS

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_EXT_ACK

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_FIB_LOOKUP

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_FIREWALL

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_GENERIC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_GET_STRICT_CHK

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_INET_DIAG

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_IP6_FW

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_ISCSI

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_KOBJECT_UEVENT

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_LISTEN_ALL_NSID

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_LIST_MEMBERSHIPS

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_NETFILTER

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_NFLOG

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_NO_ENOBUFS

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_PKTINFO

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_RDMA

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_ROUTE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_RX_RING

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_SCSITRANSPORT

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_SELINUX

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_SOCK_DIAG

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_TX_RING

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_UNUSED

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_USERSOCK

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NETLINK_XFRM

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLA_ALIGN

*Function*

```rust
fn NLA_ALIGN(len: c_int) -> c_int
```



## libc::new::linux_uapi::linux::netlink::NLA_ALIGNTO

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLA_F_NESTED

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLA_F_NET_BYTEORDER

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLA_TYPE_MASK

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLMSG_DONE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLMSG_ERROR

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLMSG_MIN_TYPE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLMSG_NOOP

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLMSG_OVERRUN

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_ACK

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_ACK_TLVS

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_APPEND

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_ATOMIC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_CAPPED

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_CREATE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_DUMP

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_DUMP_FILTERED

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_DUMP_INTR

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_ECHO

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_EXCL

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_MATCH

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_MULTI

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_NONREC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_REPLACE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_REQUEST

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::NLM_F_ROOT

*Constant*: `c_int`



## libc::new::linux_uapi::linux::netlink::nl_mmap_hdr

*Struct*

**Fields:**
- `nm_status: c_uint`
- `nm_len: c_uint`
- `nm_group: u32`
- `nm_pid: u32`
- `nm_uid: u32`
- `nm_gid: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> nl_mmap_hdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::netlink::nl_mmap_req

*Struct*

**Fields:**
- `nm_block_size: c_uint`
- `nm_block_nr: c_uint`
- `nm_frame_size: c_uint`
- `nm_frame_nr: c_uint`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> nl_mmap_req`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::netlink::nl_pktinfo

*Struct*

**Fields:**
- `group: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> nl_pktinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::netlink::nlattr

*Struct*

**Fields:**
- `nla_len: u16`
- `nla_type: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> nlattr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::netlink::nlmsgerr

*Struct*

**Fields:**
- `error: c_int`
- `msg: nlmsghdr`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> nlmsgerr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::netlink::nlmsghdr

*Struct*

**Fields:**
- `nlmsg_len: u32`
- `nlmsg_type: u16`
- `nlmsg_flags: u16`
- `nlmsg_seq: u32`
- `nlmsg_pid: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> nlmsghdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::netlink::sockaddr_nl

*Struct*

**Fields:**
- `nl_family: crate::sa_family_t`
- `nl_pid: u32`
- `nl_groups: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_nl`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



