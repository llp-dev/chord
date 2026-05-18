*[libc](../index.md) / [new](index.md)*

---

# Module `new`

This module contains the future directory structure. If possible, new definitions should
get added here.

Eventually everything should be moved over, and we will move this directory to the top
level in `src`.

# Basic structure

Each child module here represents a library or group of libraries that we are binding. Each of
these has several submodules, representing either a directory or a header file in that library.

`#include`s turn into `pub use ...*;` statements. Then at the root level (here), we choose
which top-level headers we want to reexport the definitions for.

All modules are only crate-public since we don't reexport this structure.

## Contents

- [Modules](#modules)
  - [`common`](#common)
  - [`linux_uapi`](#linux-uapi)
  - [`glibc`](#glibc)
  - [`bcm`](#bcm)
  - [`error`](#error)
  - [`j1939`](#j1939)
  - [`raw`](#raw)
- [Structs](#structs)
  - [`bcm_timeval`](#bcm-timeval)
  - [`bcm_msg_head`](#bcm-msg-head)
  - [`j1939_filter`](#j1939-filter)
  - [`can_frame`](#can-frame)
  - [`canfd_frame`](#canfd-frame)
  - [`canxl_frame`](#canxl-frame)
  - [`sockaddr_can`](#sockaddr-can)
  - [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp)
  - [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939)
  - [`can_filter`](#can-filter)
  - [`sockaddr_nl`](#sockaddr-nl)
  - [`nlmsghdr`](#nlmsghdr)
  - [`nlmsgerr`](#nlmsgerr)
  - [`nl_pktinfo`](#nl-pktinfo)
  - [`nl_mmap_req`](#nl-mmap-req)
  - [`nl_mmap_hdr`](#nl-mmap-hdr)
  - [`nlattr`](#nlattr)
  - [`pidfd_info`](#pidfd-info)
  - [`rtentry`](#rtentry)
- [Functions](#functions)
  - [`NLA_ALIGN`](#nla-align)
- [Type Aliases](#type-aliases)
  - [`pgn_t`](#pgn-t)
  - [`priority_t`](#priority-t)
  - [`name_t`](#name-t)
  - [`canid_t`](#canid-t)
  - [`can_err_mask_t`](#can-err-mask-t)
  - [`membarrier_cmd`](#membarrier-cmd)
- [Constants](#constants)
  - [`TX_SETUP`](#tx-setup)
  - [`TX_DELETE`](#tx-delete)
  - [`TX_READ`](#tx-read)
  - [`TX_SEND`](#tx-send)
  - [`RX_SETUP`](#rx-setup)
  - [`RX_DELETE`](#rx-delete)
  - [`RX_READ`](#rx-read)
  - [`TX_STATUS`](#tx-status)
  - [`TX_EXPIRED`](#tx-expired)
  - [`RX_STATUS`](#rx-status)
  - [`RX_TIMEOUT`](#rx-timeout)
  - [`RX_CHANGED`](#rx-changed)
  - [`SETTIMER`](#settimer)
  - [`STARTTIMER`](#starttimer)
  - [`TX_COUNTEVT`](#tx-countevt)
  - [`TX_ANNOUNCE`](#tx-announce)
  - [`TX_CP_CAN_ID`](#tx-cp-can-id)
  - [`RX_FILTER_ID`](#rx-filter-id)
  - [`RX_CHECK_DLC`](#rx-check-dlc)
  - [`RX_NO_AUTOTIMER`](#rx-no-autotimer)
  - [`RX_ANNOUNCE_RESUME`](#rx-announce-resume)
  - [`TX_RESET_MULTI_IDX`](#tx-reset-multi-idx)
  - [`RX_RTR_FRAME`](#rx-rtr-frame)
  - [`CAN_FD_FRAME`](#can-fd-frame)
  - [`CAN_ERR_DLC`](#can-err-dlc)
  - [`CAN_ERR_TX_TIMEOUT`](#can-err-tx-timeout)
  - [`CAN_ERR_LOSTARB`](#can-err-lostarb)
  - [`CAN_ERR_CRTL`](#can-err-crtl)
  - [`CAN_ERR_PROT`](#can-err-prot)
  - [`CAN_ERR_TRX`](#can-err-trx)
  - [`CAN_ERR_ACK`](#can-err-ack)
  - [`CAN_ERR_BUSOFF`](#can-err-busoff)
  - [`CAN_ERR_BUSERROR`](#can-err-buserror)
  - [`CAN_ERR_RESTARTED`](#can-err-restarted)
  - [`CAN_ERR_CNT`](#can-err-cnt)
  - [`CAN_ERR_LOSTARB_UNSPEC`](#can-err-lostarb-unspec)
  - [`CAN_ERR_CRTL_UNSPEC`](#can-err-crtl-unspec)
  - [`CAN_ERR_CRTL_RX_OVERFLOW`](#can-err-crtl-rx-overflow)
  - [`CAN_ERR_CRTL_TX_OVERFLOW`](#can-err-crtl-tx-overflow)
  - [`CAN_ERR_CRTL_RX_WARNING`](#can-err-crtl-rx-warning)
  - [`CAN_ERR_CRTL_TX_WARNING`](#can-err-crtl-tx-warning)
  - [`CAN_ERR_CRTL_RX_PASSIVE`](#can-err-crtl-rx-passive)
  - [`CAN_ERR_CRTL_TX_PASSIVE`](#can-err-crtl-tx-passive)
  - [`CAN_ERR_CRTL_ACTIVE`](#can-err-crtl-active)
  - [`CAN_ERR_PROT_UNSPEC`](#can-err-prot-unspec)
  - [`CAN_ERR_PROT_BIT`](#can-err-prot-bit)
  - [`CAN_ERR_PROT_FORM`](#can-err-prot-form)
  - [`CAN_ERR_PROT_STUFF`](#can-err-prot-stuff)
  - [`CAN_ERR_PROT_BIT0`](#can-err-prot-bit0)
  - [`CAN_ERR_PROT_BIT1`](#can-err-prot-bit1)
  - [`CAN_ERR_PROT_OVERLOAD`](#can-err-prot-overload)
  - [`CAN_ERR_PROT_ACTIVE`](#can-err-prot-active)
  - [`CAN_ERR_PROT_TX`](#can-err-prot-tx)
  - [`CAN_ERR_PROT_LOC_UNSPEC`](#can-err-prot-loc-unspec)
  - [`CAN_ERR_PROT_LOC_SOF`](#can-err-prot-loc-sof)
  - [`CAN_ERR_PROT_LOC_ID28_21`](#can-err-prot-loc-id28-21)
  - [`CAN_ERR_PROT_LOC_ID20_18`](#can-err-prot-loc-id20-18)
  - [`CAN_ERR_PROT_LOC_SRTR`](#can-err-prot-loc-srtr)
  - [`CAN_ERR_PROT_LOC_IDE`](#can-err-prot-loc-ide)
  - [`CAN_ERR_PROT_LOC_ID17_13`](#can-err-prot-loc-id17-13)
  - [`CAN_ERR_PROT_LOC_ID12_05`](#can-err-prot-loc-id12-05)
  - [`CAN_ERR_PROT_LOC_ID04_00`](#can-err-prot-loc-id04-00)
  - [`CAN_ERR_PROT_LOC_RTR`](#can-err-prot-loc-rtr)
  - [`CAN_ERR_PROT_LOC_RES1`](#can-err-prot-loc-res1)
  - [`CAN_ERR_PROT_LOC_RES0`](#can-err-prot-loc-res0)
  - [`CAN_ERR_PROT_LOC_DLC`](#can-err-prot-loc-dlc)
  - [`CAN_ERR_PROT_LOC_DATA`](#can-err-prot-loc-data)
  - [`CAN_ERR_PROT_LOC_CRC_SEQ`](#can-err-prot-loc-crc-seq)
  - [`CAN_ERR_PROT_LOC_CRC_DEL`](#can-err-prot-loc-crc-del)
  - [`CAN_ERR_PROT_LOC_ACK`](#can-err-prot-loc-ack)
  - [`CAN_ERR_PROT_LOC_ACK_DEL`](#can-err-prot-loc-ack-del)
  - [`CAN_ERR_PROT_LOC_EOF`](#can-err-prot-loc-eof)
  - [`CAN_ERR_PROT_LOC_INTERM`](#can-err-prot-loc-interm)
  - [`CAN_ERR_TRX_UNSPEC`](#can-err-trx-unspec)
  - [`CAN_ERR_TRX_CANH_NO_WIRE`](#can-err-trx-canh-no-wire)
  - [`CAN_ERR_TRX_CANH_SHORT_TO_BAT`](#can-err-trx-canh-short-to-bat)
  - [`CAN_ERR_TRX_CANH_SHORT_TO_VCC`](#can-err-trx-canh-short-to-vcc)
  - [`CAN_ERR_TRX_CANH_SHORT_TO_GND`](#can-err-trx-canh-short-to-gnd)
  - [`CAN_ERR_TRX_CANL_NO_WIRE`](#can-err-trx-canl-no-wire)
  - [`CAN_ERR_TRX_CANL_SHORT_TO_BAT`](#can-err-trx-canl-short-to-bat)
  - [`CAN_ERR_TRX_CANL_SHORT_TO_VCC`](#can-err-trx-canl-short-to-vcc)
  - [`CAN_ERR_TRX_CANL_SHORT_TO_GND`](#can-err-trx-canl-short-to-gnd)
  - [`CAN_ERR_TRX_CANL_SHORT_TO_CANH`](#can-err-trx-canl-short-to-canh)
  - [`CAN_ERROR_WARNING_THRESHOLD`](#can-error-warning-threshold)
  - [`CAN_ERROR_PASSIVE_THRESHOLD`](#can-error-passive-threshold)
  - [`CAN_BUS_OFF_THRESHOLD`](#can-bus-off-threshold)
  - [`J1939_MAX_UNICAST_ADDR`](#j1939-max-unicast-addr)
  - [`J1939_IDLE_ADDR`](#j1939-idle-addr)
  - [`J1939_NO_ADDR`](#j1939-no-addr)
  - [`J1939_NO_NAME`](#j1939-no-name)
  - [`J1939_PGN_REQUEST`](#j1939-pgn-request)
  - [`J1939_PGN_ADDRESS_CLAIMED`](#j1939-pgn-address-claimed)
  - [`J1939_PGN_ADDRESS_COMMANDED`](#j1939-pgn-address-commanded)
  - [`J1939_PGN_PDU1_MAX`](#j1939-pgn-pdu1-max)
  - [`J1939_PGN_MAX`](#j1939-pgn-max)
  - [`J1939_NO_PGN`](#j1939-no-pgn)
  - [`SOL_CAN_J1939`](#sol-can-j1939)
  - [`SO_J1939_FILTER`](#so-j1939-filter)
  - [`SO_J1939_PROMISC`](#so-j1939-promisc)
  - [`SO_J1939_SEND_PRIO`](#so-j1939-send-prio)
  - [`SO_J1939_ERRQUEUE`](#so-j1939-errqueue)
  - [`SCM_J1939_DEST_ADDR`](#scm-j1939-dest-addr)
  - [`SCM_J1939_DEST_NAME`](#scm-j1939-dest-name)
  - [`SCM_J1939_PRIO`](#scm-j1939-prio)
  - [`SCM_J1939_ERRQUEUE`](#scm-j1939-errqueue)
  - [`J1939_NLA_PAD`](#j1939-nla-pad)
  - [`J1939_NLA_BYTES_ACKED`](#j1939-nla-bytes-acked)
  - [`J1939_NLA_TOTAL_SIZE`](#j1939-nla-total-size)
  - [`J1939_NLA_PGN`](#j1939-nla-pgn)
  - [`J1939_NLA_SRC_NAME`](#j1939-nla-src-name)
  - [`J1939_NLA_DEST_NAME`](#j1939-nla-dest-name)
  - [`J1939_NLA_SRC_ADDR`](#j1939-nla-src-addr)
  - [`J1939_NLA_DEST_ADDR`](#j1939-nla-dest-addr)
  - [`J1939_EE_INFO_NONE`](#j1939-ee-info-none)
  - [`J1939_EE_INFO_TX_ABORT`](#j1939-ee-info-tx-abort)
  - [`J1939_EE_INFO_RX_RTS`](#j1939-ee-info-rx-rts)
  - [`J1939_EE_INFO_RX_DPO`](#j1939-ee-info-rx-dpo)
  - [`J1939_EE_INFO_RX_ABORT`](#j1939-ee-info-rx-abort)
  - [`J1939_FILTER_MAX`](#j1939-filter-max)
  - [`SOL_CAN_RAW`](#sol-can-raw)
  - [`CAN_RAW_FILTER_MAX`](#can-raw-filter-max)
  - [`CAN_RAW_FILTER`](#can-raw-filter)
  - [`CAN_RAW_ERR_FILTER`](#can-raw-err-filter)
  - [`CAN_RAW_LOOPBACK`](#can-raw-loopback)
  - [`CAN_RAW_RECV_OWN_MSGS`](#can-raw-recv-own-msgs)
  - [`CAN_RAW_FD_FRAMES`](#can-raw-fd-frames)
  - [`CAN_RAW_JOIN_FILTERS`](#can-raw-join-filters)
  - [`CAN_RAW_XL_FRAMES`](#can-raw-xl-frames)
  - [`CAN_EFF_FLAG`](#can-eff-flag)
  - [`CAN_RTR_FLAG`](#can-rtr-flag)
  - [`CAN_ERR_FLAG`](#can-err-flag)
  - [`CAN_SFF_MASK`](#can-sff-mask)
  - [`CAN_EFF_MASK`](#can-eff-mask)
  - [`CAN_ERR_MASK`](#can-err-mask)
  - [`CANXL_PRIO_MASK`](#canxl-prio-mask)
  - [`CAN_SFF_ID_BITS`](#can-sff-id-bits)
  - [`CAN_EFF_ID_BITS`](#can-eff-id-bits)
  - [`CANXL_PRIO_BITS`](#canxl-prio-bits)
  - [`CAN_MAX_DLC`](#can-max-dlc)
  - [`CAN_MAX_DLEN`](#can-max-dlen)
  - [`CANFD_MAX_DLC`](#canfd-max-dlc)
  - [`CANFD_MAX_DLEN`](#canfd-max-dlen)
  - [`CANXL_MIN_DLC`](#canxl-min-dlc)
  - [`CANXL_MAX_DLC`](#canxl-max-dlc)
  - [`CANXL_MAX_DLC_MASK`](#canxl-max-dlc-mask)
  - [`CANXL_MIN_DLEN`](#canxl-min-dlen)
  - [`CANXL_MAX_DLEN`](#canxl-max-dlen)
  - [`CANFD_BRS`](#canfd-brs)
  - [`CANFD_ESI`](#canfd-esi)
  - [`CANFD_FDF`](#canfd-fdf)
  - [`CANXL_XLF`](#canxl-xlf)
  - [`CANXL_SEC`](#canxl-sec)
  - [`CAN_MTU`](#can-mtu)
  - [`CANFD_MTU`](#canfd-mtu)
  - [`CANXL_MTU`](#canxl-mtu)
  - [`CANXL_HDR_SIZE`](#canxl-hdr-size)
  - [`CANXL_MIN_MTU`](#canxl-min-mtu)
  - [`CANXL_MAX_MTU`](#canxl-max-mtu)
  - [`CAN_RAW`](#can-raw)
  - [`CAN_BCM`](#can-bcm)
  - [`CAN_TP16`](#can-tp16)
  - [`CAN_TP20`](#can-tp20)
  - [`CAN_MCNET`](#can-mcnet)
  - [`CAN_ISOTP`](#can-isotp)
  - [`CAN_J1939`](#can-j1939)
  - [`CAN_NPROTO`](#can-nproto)
  - [`SOL_CAN_BASE`](#sol-can-base)
  - [`CAN_INV_FILTER`](#can-inv-filter)
  - [`KEY_SPEC_THREAD_KEYRING`](#key-spec-thread-keyring)
  - [`KEY_SPEC_PROCESS_KEYRING`](#key-spec-process-keyring)
  - [`KEY_SPEC_SESSION_KEYRING`](#key-spec-session-keyring)
  - [`KEY_SPEC_USER_KEYRING`](#key-spec-user-keyring)
  - [`KEY_SPEC_USER_SESSION_KEYRING`](#key-spec-user-session-keyring)
  - [`KEY_SPEC_GROUP_KEYRING`](#key-spec-group-keyring)
  - [`KEY_SPEC_REQKEY_AUTH_KEY`](#key-spec-reqkey-auth-key)
  - [`KEY_SPEC_REQUESTOR_KEYRING`](#key-spec-requestor-keyring)
  - [`KEY_REQKEY_DEFL_NO_CHANGE`](#key-reqkey-defl-no-change)
  - [`KEY_REQKEY_DEFL_DEFAULT`](#key-reqkey-defl-default)
  - [`KEY_REQKEY_DEFL_THREAD_KEYRING`](#key-reqkey-defl-thread-keyring)
  - [`KEY_REQKEY_DEFL_PROCESS_KEYRING`](#key-reqkey-defl-process-keyring)
  - [`KEY_REQKEY_DEFL_SESSION_KEYRING`](#key-reqkey-defl-session-keyring)
  - [`KEY_REQKEY_DEFL_USER_KEYRING`](#key-reqkey-defl-user-keyring)
  - [`KEY_REQKEY_DEFL_USER_SESSION_KEYRING`](#key-reqkey-defl-user-session-keyring)
  - [`KEY_REQKEY_DEFL_GROUP_KEYRING`](#key-reqkey-defl-group-keyring)
  - [`KEY_REQKEY_DEFL_REQUESTOR_KEYRING`](#key-reqkey-defl-requestor-keyring)
  - [`KEYCTL_GET_KEYRING_ID`](#keyctl-get-keyring-id)
  - [`KEYCTL_JOIN_SESSION_KEYRING`](#keyctl-join-session-keyring)
  - [`KEYCTL_UPDATE`](#keyctl-update)
  - [`KEYCTL_REVOKE`](#keyctl-revoke)
  - [`KEYCTL_CHOWN`](#keyctl-chown)
  - [`KEYCTL_SETPERM`](#keyctl-setperm)
  - [`KEYCTL_DESCRIBE`](#keyctl-describe)
  - [`KEYCTL_CLEAR`](#keyctl-clear)
  - [`KEYCTL_LINK`](#keyctl-link)
  - [`KEYCTL_UNLINK`](#keyctl-unlink)
  - [`KEYCTL_SEARCH`](#keyctl-search)
  - [`KEYCTL_READ`](#keyctl-read)
  - [`KEYCTL_INSTANTIATE`](#keyctl-instantiate)
  - [`KEYCTL_NEGATE`](#keyctl-negate)
  - [`KEYCTL_SET_REQKEY_KEYRING`](#keyctl-set-reqkey-keyring)
  - [`KEYCTL_SET_TIMEOUT`](#keyctl-set-timeout)
  - [`KEYCTL_ASSUME_AUTHORITY`](#keyctl-assume-authority)
  - [`KEYCTL_GET_SECURITY`](#keyctl-get-security)
  - [`KEYCTL_SESSION_TO_PARENT`](#keyctl-session-to-parent)
  - [`KEYCTL_REJECT`](#keyctl-reject)
  - [`KEYCTL_INSTANTIATE_IOV`](#keyctl-instantiate-iov)
  - [`KEYCTL_INVALIDATE`](#keyctl-invalidate)
  - [`KEYCTL_GET_PERSISTENT`](#keyctl-get-persistent)
  - [`KEYCTL_DH_COMPUTE`](#keyctl-dh-compute)
  - [`KEYCTL_PKEY_QUERY`](#keyctl-pkey-query)
  - [`KEYCTL_PKEY_ENCRYPT`](#keyctl-pkey-encrypt)
  - [`KEYCTL_PKEY_DECRYPT`](#keyctl-pkey-decrypt)
  - [`KEYCTL_PKEY_SIGN`](#keyctl-pkey-sign)
  - [`KEYCTL_PKEY_VERIFY`](#keyctl-pkey-verify)
  - [`KEYCTL_RESTRICT_KEYRING`](#keyctl-restrict-keyring)
  - [`KEYCTL_MOVE`](#keyctl-move)
  - [`KEYCTL_CAPABILITIES`](#keyctl-capabilities)
  - [`KEYCTL_SUPPORTS_ENCRYPT`](#keyctl-supports-encrypt)
  - [`KEYCTL_SUPPORTS_DECRYPT`](#keyctl-supports-decrypt)
  - [`KEYCTL_SUPPORTS_SIGN`](#keyctl-supports-sign)
  - [`KEYCTL_SUPPORTS_VERIFY`](#keyctl-supports-verify)
  - [`KEYCTL_CAPS0_CAPABILITIES`](#keyctl-caps0-capabilities)
  - [`KEYCTL_CAPS0_PERSISTENT_KEYRINGS`](#keyctl-caps0-persistent-keyrings)
  - [`KEYCTL_CAPS0_DIFFIE_HELLMAN`](#keyctl-caps0-diffie-hellman)
  - [`KEYCTL_CAPS0_PUBLIC_KEY`](#keyctl-caps0-public-key)
  - [`KEYCTL_CAPS0_BIG_KEY`](#keyctl-caps0-big-key)
  - [`KEYCTL_CAPS0_INVALIDATE`](#keyctl-caps0-invalidate)
  - [`KEYCTL_CAPS0_RESTRICT_KEYRING`](#keyctl-caps0-restrict-keyring)
  - [`KEYCTL_CAPS0_MOVE`](#keyctl-caps0-move)
  - [`KEYCTL_CAPS1_NS_KEYRING_NAME`](#keyctl-caps1-ns-keyring-name)
  - [`KEYCTL_CAPS1_NS_KEY_TAG`](#keyctl-caps1-ns-key-tag)
  - [`MEMBARRIER_CMD_QUERY`](#membarrier-cmd-query)
  - [`MEMBARRIER_CMD_GLOBAL`](#membarrier-cmd-global)
  - [`MEMBARRIER_CMD_GLOBAL_EXPEDITED`](#membarrier-cmd-global-expedited)
  - [`MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`](#membarrier-cmd-register-global-expedited)
  - [`MEMBARRIER_CMD_PRIVATE_EXPEDITED`](#membarrier-cmd-private-expedited)
  - [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`](#membarrier-cmd-register-private-expedited)
  - [`MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`](#membarrier-cmd-private-expedited-sync-core)
  - [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`](#membarrier-cmd-register-private-expedited-sync-core)
  - [`MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ`](#membarrier-cmd-private-expedited-rseq)
  - [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ`](#membarrier-cmd-register-private-expedited-rseq)
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
  - [`PIDFD_NONBLOCK`](#pidfd-nonblock)
  - [`PIDFD_THREAD`](#pidfd-thread)
  - [`PIDFD_SIGNAL_THREAD`](#pidfd-signal-thread)
  - [`PIDFD_SIGNAL_THREAD_GROUP`](#pidfd-signal-thread-group)
  - [`PIDFD_SIGNAL_PROCESS_GROUP`](#pidfd-signal-process-group)
  - [`PIDFD_INFO_PID`](#pidfd-info-pid)
  - [`PIDFD_INFO_CREDS`](#pidfd-info-creds)
  - [`PIDFD_INFO_CGROUPID`](#pidfd-info-cgroupid)
  - [`PIDFD_INFO_EXIT`](#pidfd-info-exit)
  - [`PIDFD_INFO_SIZE_VER0`](#pidfd-info-size-ver0)
  - [`PIDFS_IOCTL_MAGIC`](#pidfs-ioctl-magic)
  - [`PIDFD_GET_CGROUP_NAMESPACE`](#pidfd-get-cgroup-namespace)
  - [`PIDFD_GET_IPC_NAMESPACE`](#pidfd-get-ipc-namespace)
  - [`PIDFD_GET_MNT_NAMESPACE`](#pidfd-get-mnt-namespace)
  - [`PIDFD_GET_NET_NAMESPACE`](#pidfd-get-net-namespace)
  - [`PIDFD_GET_PID_NAMESPACE`](#pidfd-get-pid-namespace)
  - [`PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE`](#pidfd-get-pid-for-children-namespace)
  - [`PIDFD_GET_TIME_NAMESPACE`](#pidfd-get-time-namespace)
  - [`PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE`](#pidfd-get-time-for-children-namespace)
  - [`PIDFD_GET_USER_NAMESPACE`](#pidfd-get-user-namespace)
  - [`PIDFD_GET_UTS_NAMESPACE`](#pidfd-get-uts-namespace)
  - [`PIDFD_GET_INFO`](#pidfd-get-info)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`common`](#common) | mod | Interfaces that are common across multiple platforms |
| [`linux_uapi`](#linux-uapi) | mod | This directory maps to `include/uapi` in the Linux source tree. |
| [`glibc`](#glibc) | mod | GNU libc. |
| [`bcm`](#bcm) | mod | Header: `linux/can/bcm.h` |
| [`error`](#error) | mod | Header: `linux/can/error.h` |
| [`j1939`](#j1939) | mod | `linux/can/j1939.h` |
| [`raw`](#raw) | mod | Header: `linux/can/raw.h` |
| [`bcm_timeval`](#bcm-timeval) | struct |  |
| [`bcm_msg_head`](#bcm-msg-head) | struct |  |
| [`j1939_filter`](#j1939-filter) | struct |  |
| [`can_frame`](#can-frame) | struct |  |
| [`canfd_frame`](#canfd-frame) | struct |  |
| [`canxl_frame`](#canxl-frame) | struct |  |
| [`sockaddr_can`](#sockaddr-can) | struct |  |
| [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp) | struct |  |
| [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939) | struct |  |
| [`can_filter`](#can-filter) | struct |  |
| [`sockaddr_nl`](#sockaddr-nl) | struct |  |
| [`nlmsghdr`](#nlmsghdr) | struct |  |
| [`nlmsgerr`](#nlmsgerr) | struct |  |
| [`nl_pktinfo`](#nl-pktinfo) | struct |  |
| [`nl_mmap_req`](#nl-mmap-req) | struct |  |
| [`nl_mmap_hdr`](#nl-mmap-hdr) | struct |  |
| [`nlattr`](#nlattr) | struct |  |
| [`pidfd_info`](#pidfd-info) | struct |  |
| [`rtentry`](#rtentry) | struct |  |
| [`NLA_ALIGN`](#nla-align) | fn |  |
| [`pgn_t`](#pgn-t) | type |  |
| [`priority_t`](#priority-t) | type |  |
| [`name_t`](#name-t) | type |  |
| [`canid_t`](#canid-t) | type |  |
| [`can_err_mask_t`](#can-err-mask-t) | type |  |
| [`membarrier_cmd`](#membarrier-cmd) | type |  |
| [`TX_SETUP`](#tx-setup) | const |  |
| [`TX_DELETE`](#tx-delete) | const |  |
| [`TX_READ`](#tx-read) | const |  |
| [`TX_SEND`](#tx-send) | const |  |
| [`RX_SETUP`](#rx-setup) | const |  |
| [`RX_DELETE`](#rx-delete) | const |  |
| [`RX_READ`](#rx-read) | const |  |
| [`TX_STATUS`](#tx-status) | const |  |
| [`TX_EXPIRED`](#tx-expired) | const |  |
| [`RX_STATUS`](#rx-status) | const |  |
| [`RX_TIMEOUT`](#rx-timeout) | const |  |
| [`RX_CHANGED`](#rx-changed) | const |  |
| [`SETTIMER`](#settimer) | const |  |
| [`STARTTIMER`](#starttimer) | const |  |
| [`TX_COUNTEVT`](#tx-countevt) | const |  |
| [`TX_ANNOUNCE`](#tx-announce) | const |  |
| [`TX_CP_CAN_ID`](#tx-cp-can-id) | const |  |
| [`RX_FILTER_ID`](#rx-filter-id) | const |  |
| [`RX_CHECK_DLC`](#rx-check-dlc) | const |  |
| [`RX_NO_AUTOTIMER`](#rx-no-autotimer) | const |  |
| [`RX_ANNOUNCE_RESUME`](#rx-announce-resume) | const |  |
| [`TX_RESET_MULTI_IDX`](#tx-reset-multi-idx) | const |  |
| [`RX_RTR_FRAME`](#rx-rtr-frame) | const |  |
| [`CAN_FD_FRAME`](#can-fd-frame) | const |  |
| [`CAN_ERR_DLC`](#can-err-dlc) | const |  |
| [`CAN_ERR_TX_TIMEOUT`](#can-err-tx-timeout) | const |  |
| [`CAN_ERR_LOSTARB`](#can-err-lostarb) | const |  |
| [`CAN_ERR_CRTL`](#can-err-crtl) | const |  |
| [`CAN_ERR_PROT`](#can-err-prot) | const |  |
| [`CAN_ERR_TRX`](#can-err-trx) | const |  |
| [`CAN_ERR_ACK`](#can-err-ack) | const |  |
| [`CAN_ERR_BUSOFF`](#can-err-busoff) | const |  |
| [`CAN_ERR_BUSERROR`](#can-err-buserror) | const |  |
| [`CAN_ERR_RESTARTED`](#can-err-restarted) | const |  |
| [`CAN_ERR_CNT`](#can-err-cnt) | const |  |
| [`CAN_ERR_LOSTARB_UNSPEC`](#can-err-lostarb-unspec) | const |  |
| [`CAN_ERR_CRTL_UNSPEC`](#can-err-crtl-unspec) | const |  |
| [`CAN_ERR_CRTL_RX_OVERFLOW`](#can-err-crtl-rx-overflow) | const |  |
| [`CAN_ERR_CRTL_TX_OVERFLOW`](#can-err-crtl-tx-overflow) | const |  |
| [`CAN_ERR_CRTL_RX_WARNING`](#can-err-crtl-rx-warning) | const |  |
| [`CAN_ERR_CRTL_TX_WARNING`](#can-err-crtl-tx-warning) | const |  |
| [`CAN_ERR_CRTL_RX_PASSIVE`](#can-err-crtl-rx-passive) | const |  |
| [`CAN_ERR_CRTL_TX_PASSIVE`](#can-err-crtl-tx-passive) | const |  |
| [`CAN_ERR_CRTL_ACTIVE`](#can-err-crtl-active) | const |  |
| [`CAN_ERR_PROT_UNSPEC`](#can-err-prot-unspec) | const |  |
| [`CAN_ERR_PROT_BIT`](#can-err-prot-bit) | const |  |
| [`CAN_ERR_PROT_FORM`](#can-err-prot-form) | const |  |
| [`CAN_ERR_PROT_STUFF`](#can-err-prot-stuff) | const |  |
| [`CAN_ERR_PROT_BIT0`](#can-err-prot-bit0) | const |  |
| [`CAN_ERR_PROT_BIT1`](#can-err-prot-bit1) | const |  |
| [`CAN_ERR_PROT_OVERLOAD`](#can-err-prot-overload) | const |  |
| [`CAN_ERR_PROT_ACTIVE`](#can-err-prot-active) | const |  |
| [`CAN_ERR_PROT_TX`](#can-err-prot-tx) | const |  |
| [`CAN_ERR_PROT_LOC_UNSPEC`](#can-err-prot-loc-unspec) | const |  |
| [`CAN_ERR_PROT_LOC_SOF`](#can-err-prot-loc-sof) | const |  |
| [`CAN_ERR_PROT_LOC_ID28_21`](#can-err-prot-loc-id28-21) | const |  |
| [`CAN_ERR_PROT_LOC_ID20_18`](#can-err-prot-loc-id20-18) | const |  |
| [`CAN_ERR_PROT_LOC_SRTR`](#can-err-prot-loc-srtr) | const |  |
| [`CAN_ERR_PROT_LOC_IDE`](#can-err-prot-loc-ide) | const |  |
| [`CAN_ERR_PROT_LOC_ID17_13`](#can-err-prot-loc-id17-13) | const |  |
| [`CAN_ERR_PROT_LOC_ID12_05`](#can-err-prot-loc-id12-05) | const |  |
| [`CAN_ERR_PROT_LOC_ID04_00`](#can-err-prot-loc-id04-00) | const |  |
| [`CAN_ERR_PROT_LOC_RTR`](#can-err-prot-loc-rtr) | const |  |
| [`CAN_ERR_PROT_LOC_RES1`](#can-err-prot-loc-res1) | const |  |
| [`CAN_ERR_PROT_LOC_RES0`](#can-err-prot-loc-res0) | const |  |
| [`CAN_ERR_PROT_LOC_DLC`](#can-err-prot-loc-dlc) | const |  |
| [`CAN_ERR_PROT_LOC_DATA`](#can-err-prot-loc-data) | const |  |
| [`CAN_ERR_PROT_LOC_CRC_SEQ`](#can-err-prot-loc-crc-seq) | const |  |
| [`CAN_ERR_PROT_LOC_CRC_DEL`](#can-err-prot-loc-crc-del) | const |  |
| [`CAN_ERR_PROT_LOC_ACK`](#can-err-prot-loc-ack) | const |  |
| [`CAN_ERR_PROT_LOC_ACK_DEL`](#can-err-prot-loc-ack-del) | const |  |
| [`CAN_ERR_PROT_LOC_EOF`](#can-err-prot-loc-eof) | const |  |
| [`CAN_ERR_PROT_LOC_INTERM`](#can-err-prot-loc-interm) | const |  |
| [`CAN_ERR_TRX_UNSPEC`](#can-err-trx-unspec) | const |  |
| [`CAN_ERR_TRX_CANH_NO_WIRE`](#can-err-trx-canh-no-wire) | const |  |
| [`CAN_ERR_TRX_CANH_SHORT_TO_BAT`](#can-err-trx-canh-short-to-bat) | const |  |
| [`CAN_ERR_TRX_CANH_SHORT_TO_VCC`](#can-err-trx-canh-short-to-vcc) | const |  |
| [`CAN_ERR_TRX_CANH_SHORT_TO_GND`](#can-err-trx-canh-short-to-gnd) | const |  |
| [`CAN_ERR_TRX_CANL_NO_WIRE`](#can-err-trx-canl-no-wire) | const |  |
| [`CAN_ERR_TRX_CANL_SHORT_TO_BAT`](#can-err-trx-canl-short-to-bat) | const |  |
| [`CAN_ERR_TRX_CANL_SHORT_TO_VCC`](#can-err-trx-canl-short-to-vcc) | const |  |
| [`CAN_ERR_TRX_CANL_SHORT_TO_GND`](#can-err-trx-canl-short-to-gnd) | const |  |
| [`CAN_ERR_TRX_CANL_SHORT_TO_CANH`](#can-err-trx-canl-short-to-canh) | const |  |
| [`CAN_ERROR_WARNING_THRESHOLD`](#can-error-warning-threshold) | const |  |
| [`CAN_ERROR_PASSIVE_THRESHOLD`](#can-error-passive-threshold) | const |  |
| [`CAN_BUS_OFF_THRESHOLD`](#can-bus-off-threshold) | const |  |
| [`J1939_MAX_UNICAST_ADDR`](#j1939-max-unicast-addr) | const |  |
| [`J1939_IDLE_ADDR`](#j1939-idle-addr) | const |  |
| [`J1939_NO_ADDR`](#j1939-no-addr) | const |  |
| [`J1939_NO_NAME`](#j1939-no-name) | const |  |
| [`J1939_PGN_REQUEST`](#j1939-pgn-request) | const |  |
| [`J1939_PGN_ADDRESS_CLAIMED`](#j1939-pgn-address-claimed) | const |  |
| [`J1939_PGN_ADDRESS_COMMANDED`](#j1939-pgn-address-commanded) | const |  |
| [`J1939_PGN_PDU1_MAX`](#j1939-pgn-pdu1-max) | const |  |
| [`J1939_PGN_MAX`](#j1939-pgn-max) | const |  |
| [`J1939_NO_PGN`](#j1939-no-pgn) | const |  |
| [`SOL_CAN_J1939`](#sol-can-j1939) | const |  |
| [`SO_J1939_FILTER`](#so-j1939-filter) | const |  |
| [`SO_J1939_PROMISC`](#so-j1939-promisc) | const |  |
| [`SO_J1939_SEND_PRIO`](#so-j1939-send-prio) | const |  |
| [`SO_J1939_ERRQUEUE`](#so-j1939-errqueue) | const |  |
| [`SCM_J1939_DEST_ADDR`](#scm-j1939-dest-addr) | const |  |
| [`SCM_J1939_DEST_NAME`](#scm-j1939-dest-name) | const |  |
| [`SCM_J1939_PRIO`](#scm-j1939-prio) | const |  |
| [`SCM_J1939_ERRQUEUE`](#scm-j1939-errqueue) | const |  |
| [`J1939_NLA_PAD`](#j1939-nla-pad) | const |  |
| [`J1939_NLA_BYTES_ACKED`](#j1939-nla-bytes-acked) | const |  |
| [`J1939_NLA_TOTAL_SIZE`](#j1939-nla-total-size) | const |  |
| [`J1939_NLA_PGN`](#j1939-nla-pgn) | const |  |
| [`J1939_NLA_SRC_NAME`](#j1939-nla-src-name) | const |  |
| [`J1939_NLA_DEST_NAME`](#j1939-nla-dest-name) | const |  |
| [`J1939_NLA_SRC_ADDR`](#j1939-nla-src-addr) | const |  |
| [`J1939_NLA_DEST_ADDR`](#j1939-nla-dest-addr) | const |  |
| [`J1939_EE_INFO_NONE`](#j1939-ee-info-none) | const |  |
| [`J1939_EE_INFO_TX_ABORT`](#j1939-ee-info-tx-abort) | const |  |
| [`J1939_EE_INFO_RX_RTS`](#j1939-ee-info-rx-rts) | const |  |
| [`J1939_EE_INFO_RX_DPO`](#j1939-ee-info-rx-dpo) | const |  |
| [`J1939_EE_INFO_RX_ABORT`](#j1939-ee-info-rx-abort) | const |  |
| [`J1939_FILTER_MAX`](#j1939-filter-max) | const |  |
| [`SOL_CAN_RAW`](#sol-can-raw) | const |  |
| [`CAN_RAW_FILTER_MAX`](#can-raw-filter-max) | const |  |
| [`CAN_RAW_FILTER`](#can-raw-filter) | const |  |
| [`CAN_RAW_ERR_FILTER`](#can-raw-err-filter) | const |  |
| [`CAN_RAW_LOOPBACK`](#can-raw-loopback) | const |  |
| [`CAN_RAW_RECV_OWN_MSGS`](#can-raw-recv-own-msgs) | const |  |
| [`CAN_RAW_FD_FRAMES`](#can-raw-fd-frames) | const |  |
| [`CAN_RAW_JOIN_FILTERS`](#can-raw-join-filters) | const |  |
| [`CAN_RAW_XL_FRAMES`](#can-raw-xl-frames) | const |  |
| [`CAN_EFF_FLAG`](#can-eff-flag) | const |  |
| [`CAN_RTR_FLAG`](#can-rtr-flag) | const |  |
| [`CAN_ERR_FLAG`](#can-err-flag) | const |  |
| [`CAN_SFF_MASK`](#can-sff-mask) | const |  |
| [`CAN_EFF_MASK`](#can-eff-mask) | const |  |
| [`CAN_ERR_MASK`](#can-err-mask) | const |  |
| [`CANXL_PRIO_MASK`](#canxl-prio-mask) | const |  |
| [`CAN_SFF_ID_BITS`](#can-sff-id-bits) | const |  |
| [`CAN_EFF_ID_BITS`](#can-eff-id-bits) | const |  |
| [`CANXL_PRIO_BITS`](#canxl-prio-bits) | const |  |
| [`CAN_MAX_DLC`](#can-max-dlc) | const |  |
| [`CAN_MAX_DLEN`](#can-max-dlen) | const |  |
| [`CANFD_MAX_DLC`](#canfd-max-dlc) | const |  |
| [`CANFD_MAX_DLEN`](#canfd-max-dlen) | const |  |
| [`CANXL_MIN_DLC`](#canxl-min-dlc) | const |  |
| [`CANXL_MAX_DLC`](#canxl-max-dlc) | const |  |
| [`CANXL_MAX_DLC_MASK`](#canxl-max-dlc-mask) | const |  |
| [`CANXL_MIN_DLEN`](#canxl-min-dlen) | const |  |
| [`CANXL_MAX_DLEN`](#canxl-max-dlen) | const |  |
| [`CANFD_BRS`](#canfd-brs) | const |  |
| [`CANFD_ESI`](#canfd-esi) | const |  |
| [`CANFD_FDF`](#canfd-fdf) | const |  |
| [`CANXL_XLF`](#canxl-xlf) | const |  |
| [`CANXL_SEC`](#canxl-sec) | const |  |
| [`CAN_MTU`](#can-mtu) | const |  |
| [`CANFD_MTU`](#canfd-mtu) | const |  |
| [`CANXL_MTU`](#canxl-mtu) | const |  |
| [`CANXL_HDR_SIZE`](#canxl-hdr-size) | const |  |
| [`CANXL_MIN_MTU`](#canxl-min-mtu) | const |  |
| [`CANXL_MAX_MTU`](#canxl-max-mtu) | const |  |
| [`CAN_RAW`](#can-raw) | const |  |
| [`CAN_BCM`](#can-bcm) | const |  |
| [`CAN_TP16`](#can-tp16) | const |  |
| [`CAN_TP20`](#can-tp20) | const |  |
| [`CAN_MCNET`](#can-mcnet) | const |  |
| [`CAN_ISOTP`](#can-isotp) | const |  |
| [`CAN_J1939`](#can-j1939) | const |  |
| [`CAN_NPROTO`](#can-nproto) | const |  |
| [`SOL_CAN_BASE`](#sol-can-base) | const |  |
| [`CAN_INV_FILTER`](#can-inv-filter) | const |  |
| [`KEY_SPEC_THREAD_KEYRING`](#key-spec-thread-keyring) | const |  |
| [`KEY_SPEC_PROCESS_KEYRING`](#key-spec-process-keyring) | const |  |
| [`KEY_SPEC_SESSION_KEYRING`](#key-spec-session-keyring) | const |  |
| [`KEY_SPEC_USER_KEYRING`](#key-spec-user-keyring) | const |  |
| [`KEY_SPEC_USER_SESSION_KEYRING`](#key-spec-user-session-keyring) | const |  |
| [`KEY_SPEC_GROUP_KEYRING`](#key-spec-group-keyring) | const |  |
| [`KEY_SPEC_REQKEY_AUTH_KEY`](#key-spec-reqkey-auth-key) | const |  |
| [`KEY_SPEC_REQUESTOR_KEYRING`](#key-spec-requestor-keyring) | const |  |
| [`KEY_REQKEY_DEFL_NO_CHANGE`](#key-reqkey-defl-no-change) | const |  |
| [`KEY_REQKEY_DEFL_DEFAULT`](#key-reqkey-defl-default) | const |  |
| [`KEY_REQKEY_DEFL_THREAD_KEYRING`](#key-reqkey-defl-thread-keyring) | const |  |
| [`KEY_REQKEY_DEFL_PROCESS_KEYRING`](#key-reqkey-defl-process-keyring) | const |  |
| [`KEY_REQKEY_DEFL_SESSION_KEYRING`](#key-reqkey-defl-session-keyring) | const |  |
| [`KEY_REQKEY_DEFL_USER_KEYRING`](#key-reqkey-defl-user-keyring) | const |  |
| [`KEY_REQKEY_DEFL_USER_SESSION_KEYRING`](#key-reqkey-defl-user-session-keyring) | const |  |
| [`KEY_REQKEY_DEFL_GROUP_KEYRING`](#key-reqkey-defl-group-keyring) | const |  |
| [`KEY_REQKEY_DEFL_REQUESTOR_KEYRING`](#key-reqkey-defl-requestor-keyring) | const |  |
| [`KEYCTL_GET_KEYRING_ID`](#keyctl-get-keyring-id) | const |  |
| [`KEYCTL_JOIN_SESSION_KEYRING`](#keyctl-join-session-keyring) | const |  |
| [`KEYCTL_UPDATE`](#keyctl-update) | const |  |
| [`KEYCTL_REVOKE`](#keyctl-revoke) | const |  |
| [`KEYCTL_CHOWN`](#keyctl-chown) | const |  |
| [`KEYCTL_SETPERM`](#keyctl-setperm) | const |  |
| [`KEYCTL_DESCRIBE`](#keyctl-describe) | const |  |
| [`KEYCTL_CLEAR`](#keyctl-clear) | const |  |
| [`KEYCTL_LINK`](#keyctl-link) | const |  |
| [`KEYCTL_UNLINK`](#keyctl-unlink) | const |  |
| [`KEYCTL_SEARCH`](#keyctl-search) | const |  |
| [`KEYCTL_READ`](#keyctl-read) | const |  |
| [`KEYCTL_INSTANTIATE`](#keyctl-instantiate) | const |  |
| [`KEYCTL_NEGATE`](#keyctl-negate) | const |  |
| [`KEYCTL_SET_REQKEY_KEYRING`](#keyctl-set-reqkey-keyring) | const |  |
| [`KEYCTL_SET_TIMEOUT`](#keyctl-set-timeout) | const |  |
| [`KEYCTL_ASSUME_AUTHORITY`](#keyctl-assume-authority) | const |  |
| [`KEYCTL_GET_SECURITY`](#keyctl-get-security) | const |  |
| [`KEYCTL_SESSION_TO_PARENT`](#keyctl-session-to-parent) | const |  |
| [`KEYCTL_REJECT`](#keyctl-reject) | const |  |
| [`KEYCTL_INSTANTIATE_IOV`](#keyctl-instantiate-iov) | const |  |
| [`KEYCTL_INVALIDATE`](#keyctl-invalidate) | const |  |
| [`KEYCTL_GET_PERSISTENT`](#keyctl-get-persistent) | const |  |
| [`KEYCTL_DH_COMPUTE`](#keyctl-dh-compute) | const |  |
| [`KEYCTL_PKEY_QUERY`](#keyctl-pkey-query) | const |  |
| [`KEYCTL_PKEY_ENCRYPT`](#keyctl-pkey-encrypt) | const |  |
| [`KEYCTL_PKEY_DECRYPT`](#keyctl-pkey-decrypt) | const |  |
| [`KEYCTL_PKEY_SIGN`](#keyctl-pkey-sign) | const |  |
| [`KEYCTL_PKEY_VERIFY`](#keyctl-pkey-verify) | const |  |
| [`KEYCTL_RESTRICT_KEYRING`](#keyctl-restrict-keyring) | const |  |
| [`KEYCTL_MOVE`](#keyctl-move) | const |  |
| [`KEYCTL_CAPABILITIES`](#keyctl-capabilities) | const |  |
| [`KEYCTL_SUPPORTS_ENCRYPT`](#keyctl-supports-encrypt) | const |  |
| [`KEYCTL_SUPPORTS_DECRYPT`](#keyctl-supports-decrypt) | const |  |
| [`KEYCTL_SUPPORTS_SIGN`](#keyctl-supports-sign) | const |  |
| [`KEYCTL_SUPPORTS_VERIFY`](#keyctl-supports-verify) | const |  |
| [`KEYCTL_CAPS0_CAPABILITIES`](#keyctl-caps0-capabilities) | const |  |
| [`KEYCTL_CAPS0_PERSISTENT_KEYRINGS`](#keyctl-caps0-persistent-keyrings) | const |  |
| [`KEYCTL_CAPS0_DIFFIE_HELLMAN`](#keyctl-caps0-diffie-hellman) | const |  |
| [`KEYCTL_CAPS0_PUBLIC_KEY`](#keyctl-caps0-public-key) | const |  |
| [`KEYCTL_CAPS0_BIG_KEY`](#keyctl-caps0-big-key) | const |  |
| [`KEYCTL_CAPS0_INVALIDATE`](#keyctl-caps0-invalidate) | const |  |
| [`KEYCTL_CAPS0_RESTRICT_KEYRING`](#keyctl-caps0-restrict-keyring) | const |  |
| [`KEYCTL_CAPS0_MOVE`](#keyctl-caps0-move) | const |  |
| [`KEYCTL_CAPS1_NS_KEYRING_NAME`](#keyctl-caps1-ns-keyring-name) | const |  |
| [`KEYCTL_CAPS1_NS_KEY_TAG`](#keyctl-caps1-ns-key-tag) | const |  |
| [`MEMBARRIER_CMD_QUERY`](#membarrier-cmd-query) | const |  |
| [`MEMBARRIER_CMD_GLOBAL`](#membarrier-cmd-global) | const |  |
| [`MEMBARRIER_CMD_GLOBAL_EXPEDITED`](#membarrier-cmd-global-expedited) | const |  |
| [`MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`](#membarrier-cmd-register-global-expedited) | const |  |
| [`MEMBARRIER_CMD_PRIVATE_EXPEDITED`](#membarrier-cmd-private-expedited) | const |  |
| [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`](#membarrier-cmd-register-private-expedited) | const |  |
| [`MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`](#membarrier-cmd-private-expedited-sync-core) | const |  |
| [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`](#membarrier-cmd-register-private-expedited-sync-core) | const |  |
| [`MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ`](#membarrier-cmd-private-expedited-rseq) | const |  |
| [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ`](#membarrier-cmd-register-private-expedited-rseq) | const |  |
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
| [`PIDFD_NONBLOCK`](#pidfd-nonblock) | const |  |
| [`PIDFD_THREAD`](#pidfd-thread) | const |  |
| [`PIDFD_SIGNAL_THREAD`](#pidfd-signal-thread) | const |  |
| [`PIDFD_SIGNAL_THREAD_GROUP`](#pidfd-signal-thread-group) | const |  |
| [`PIDFD_SIGNAL_PROCESS_GROUP`](#pidfd-signal-process-group) | const |  |
| [`PIDFD_INFO_PID`](#pidfd-info-pid) | const |  |
| [`PIDFD_INFO_CREDS`](#pidfd-info-creds) | const |  |
| [`PIDFD_INFO_CGROUPID`](#pidfd-info-cgroupid) | const |  |
| [`PIDFD_INFO_EXIT`](#pidfd-info-exit) | const |  |
| [`PIDFD_INFO_SIZE_VER0`](#pidfd-info-size-ver0) | const |  |
| [`PIDFS_IOCTL_MAGIC`](#pidfs-ioctl-magic) | const |  |
| [`PIDFD_GET_CGROUP_NAMESPACE`](#pidfd-get-cgroup-namespace) | const |  |
| [`PIDFD_GET_IPC_NAMESPACE`](#pidfd-get-ipc-namespace) | const |  |
| [`PIDFD_GET_MNT_NAMESPACE`](#pidfd-get-mnt-namespace) | const |  |
| [`PIDFD_GET_NET_NAMESPACE`](#pidfd-get-net-namespace) | const |  |
| [`PIDFD_GET_PID_NAMESPACE`](#pidfd-get-pid-namespace) | const |  |
| [`PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE`](#pidfd-get-pid-for-children-namespace) | const |  |
| [`PIDFD_GET_TIME_NAMESPACE`](#pidfd-get-time-namespace) | const |  |
| [`PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE`](#pidfd-get-time-for-children-namespace) | const |  |
| [`PIDFD_GET_USER_NAMESPACE`](#pidfd-get-user-namespace) | const |  |
| [`PIDFD_GET_UTS_NAMESPACE`](#pidfd-get-uts-namespace) | const |  |
| [`PIDFD_GET_INFO`](#pidfd-get-info) | const |  |

## Modules

- [`common`](common/index.md) — Interfaces that are common across multiple platforms
- [`linux_uapi`](linux_uapi/index.md) — This directory maps to `include/uapi` in the Linux source tree.
- [`glibc`](glibc/index.md) — GNU libc.
- [`bcm`](bcm/index.md) — Header: `linux/can/bcm.h`
- [`error`](error/index.md) — Header: `linux/can/error.h`
- [`j1939`](j1939/index.md) — `linux/can/j1939.h`
- [`raw`](raw/index.md) — Header: `linux/can/raw.h`

## Structs

### `bcm_timeval`

```rust
struct bcm_timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}
```

#### Trait Implementations

##### `impl Clone for bcm_timeval`

- <span id="bcm-timeval-clone"></span>`fn clone(&self) -> bcm_timeval` — [`bcm_timeval`](#bcm-timeval)

##### `impl Copy for bcm_timeval`

##### `impl Debug for bcm_timeval`

- <span id="bcm-timeval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `bcm_msg_head`

```rust
struct bcm_msg_head {
    pub opcode: u32,
    pub flags: u32,
    pub count: u32,
    pub ival1: bcm_timeval,
    pub ival2: bcm_timeval,
    pub can_id: canid_t,
    pub nframes: u32,
    pub frames: [can_frame; 0],
}
```

#### Trait Implementations

##### `impl Clone for bcm_msg_head`

- <span id="bcm-msg-head-clone"></span>`fn clone(&self) -> bcm_msg_head` — [`bcm_msg_head`](#bcm-msg-head)

##### `impl Copy for bcm_msg_head`

##### `impl Debug for bcm_msg_head`

- <span id="bcm-msg-head-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `j1939_filter`

```rust
struct j1939_filter {
    pub name: name_t,
    pub name_mask: name_t,
    pub pgn: pgn_t,
    pub pgn_mask: pgn_t,
    pub addr: u8,
    pub addr_mask: u8,
}
```

#### Trait Implementations

##### `impl Clone for j1939_filter`

- <span id="j1939-filter-clone"></span>`fn clone(&self) -> j1939_filter` — [`j1939_filter`](#j1939-filter)

##### `impl Copy for j1939_filter`

##### `impl Debug for j1939_filter`

- <span id="j1939-filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `can_frame`

```rust
struct can_frame {
    pub can_id: canid_t,
    pub can_dlc: u8,
    __pad: Padding<u8>,
    __res0: u8,
    pub len8_dlc: u8,
    pub data: [u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for can_frame`

- <span id="can-frame-clone"></span>`fn clone(&self) -> can_frame` — [`can_frame`](#can-frame)

##### `impl Copy for can_frame`

##### `impl Debug for can_frame`

- <span id="can-frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `canfd_frame`

```rust
struct canfd_frame {
    pub can_id: canid_t,
    pub len: u8,
    pub flags: u8,
    __res0: u8,
    __res1: u8,
    pub data: [u8; 64],
}
```

#### Trait Implementations

##### `impl Clone for canfd_frame`

- <span id="canfd-frame-clone"></span>`fn clone(&self) -> canfd_frame` — [`canfd_frame`](#canfd-frame)

##### `impl Copy for canfd_frame`

##### `impl Debug for canfd_frame`

- <span id="canfd-frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `canxl_frame`

```rust
struct canxl_frame {
    pub prio: canid_t,
    pub flags: u8,
    pub sdt: u8,
    pub len: u16,
    pub af: u32,
    pub data: [u8; 2048],
}
```

#### Trait Implementations

##### `impl Clone for canxl_frame`

- <span id="canxl-frame-clone"></span>`fn clone(&self) -> canxl_frame` — [`canxl_frame`](#canxl-frame)

##### `impl Copy for canxl_frame`

##### `impl Debug for canxl_frame`

- <span id="canxl-frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_can`

```rust
struct sockaddr_can {
    pub can_family: crate::sa_family_t,
    pub can_ifindex: c_int,
    pub can_addr: __c_anonymous_sockaddr_can_can_addr,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_can`

- <span id="sockaddr-can-clone"></span>`fn clone(&self) -> sockaddr_can` — [`sockaddr_can`](#sockaddr-can)

##### `impl Copy for sockaddr_can`

##### `impl Debug for sockaddr_can`

- <span id="sockaddr-can-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_sockaddr_can_tp`

```rust
struct __c_anonymous_sockaddr_can_tp {
    pub rx_id: canid_t,
    pub tx_id: canid_t,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-clone"></span>`fn clone(&self) -> __c_anonymous_sockaddr_can_tp` — [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp)

##### `impl Copy for __c_anonymous_sockaddr_can_tp`

##### `impl Debug for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_sockaddr_can_j1939`

```rust
struct __c_anonymous_sockaddr_can_j1939 {
    pub name: u64,
    pub pgn: u32,
    pub addr: u8,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-clone"></span>`fn clone(&self) -> __c_anonymous_sockaddr_can_j1939` — [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939)

##### `impl Copy for __c_anonymous_sockaddr_can_j1939`

##### `impl Debug for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `can_filter`

```rust
struct can_filter {
    pub can_id: canid_t,
    pub can_mask: canid_t,
}
```

#### Trait Implementations

##### `impl Clone for can_filter`

- <span id="can-filter-clone"></span>`fn clone(&self) -> can_filter` — [`can_filter`](#can-filter)

##### `impl Copy for can_filter`

##### `impl Debug for can_filter`

- <span id="can-filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="sockaddr-nl-clone"></span>`fn clone(&self) -> sockaddr_nl` — [`sockaddr_nl`](#sockaddr-nl)

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

- <span id="nlmsghdr-clone"></span>`fn clone(&self) -> nlmsghdr` — [`nlmsghdr`](#nlmsghdr)

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

- <span id="nlmsgerr-clone"></span>`fn clone(&self) -> nlmsgerr` — [`nlmsgerr`](#nlmsgerr)

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

- <span id="nl-pktinfo-clone"></span>`fn clone(&self) -> nl_pktinfo` — [`nl_pktinfo`](#nl-pktinfo)

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

- <span id="nl-mmap-req-clone"></span>`fn clone(&self) -> nl_mmap_req` — [`nl_mmap_req`](#nl-mmap-req)

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

- <span id="nl-mmap-hdr-clone"></span>`fn clone(&self) -> nl_mmap_hdr` — [`nl_mmap_hdr`](#nl-mmap-hdr)

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

- <span id="nlattr-clone"></span>`fn clone(&self) -> nlattr` — [`nlattr`](#nlattr)

##### `impl Copy for nlattr`

##### `impl Debug for nlattr`

- <span id="nlattr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pidfd_info`

```rust
struct pidfd_info {
    pub mask: crate::__u64,
    pub cgroupid: crate::__u64,
    pub pid: crate::__u32,
    pub tgid: crate::__u32,
    pub ppid: crate::__u32,
    pub ruid: crate::__u32,
    pub rgid: crate::__u32,
    pub euid: crate::__u32,
    pub egid: crate::__u32,
    pub suid: crate::__u32,
    pub sgid: crate::__u32,
    pub fsuid: crate::__u32,
    pub fsgid: crate::__u32,
    pub exit_code: crate::__s32,
}
```

#### Trait Implementations

##### `impl Clone for pidfd_info`

- <span id="pidfd-info-clone"></span>`fn clone(&self) -> pidfd_info` — [`pidfd_info`](#pidfd-info)

##### `impl Copy for pidfd_info`

##### `impl Debug for pidfd_info`

- <span id="pidfd-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rtentry`

```rust
struct rtentry {
    pub rt_pad1: c_ulong,
    pub rt_dst: crate::sockaddr,
    pub rt_gateway: crate::sockaddr,
    pub rt_genmask: crate::sockaddr,
    pub rt_flags: c_ushort,
    pub rt_pad2: c_short,
    pub rt_pad3: c_ulong,
    pub rt_tos: c_uchar,
    pub rt_class: c_uchar,
    pub rt_pad4: [c_short; 3],
    pub rt_metric: c_short,
    pub rt_dev: *mut c_char,
    pub rt_mtu: c_ulong,
    pub rt_window: c_ulong,
    pub rt_irtt: c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for rtentry`

- <span id="rtentry-clone"></span>`fn clone(&self) -> rtentry` — [`rtentry`](#rtentry)

##### `impl Copy for rtentry`

##### `impl Debug for rtentry`

- <span id="rtentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `NLA_ALIGN`

```rust
unsafe fn NLA_ALIGN(len: c_int) -> c_int
```

## Type Aliases

### `pgn_t`

```rust
type pgn_t = u32;
```

### `priority_t`

```rust
type priority_t = u8;
```

### `name_t`

```rust
type name_t = u64;
```

### `canid_t`

```rust
type canid_t = u32;
```

### `can_err_mask_t`

```rust
type can_err_mask_t = u32;
```

### `membarrier_cmd`

```rust
type membarrier_cmd = c_int;
```

## Constants

### `TX_SETUP`
```rust
const TX_SETUP: u32 = 1u32;
```

### `TX_DELETE`
```rust
const TX_DELETE: u32 = 2u32;
```

### `TX_READ`
```rust
const TX_READ: u32 = 3u32;
```

### `TX_SEND`
```rust
const TX_SEND: u32 = 4u32;
```

### `RX_SETUP`
```rust
const RX_SETUP: u32 = 5u32;
```

### `RX_DELETE`
```rust
const RX_DELETE: u32 = 6u32;
```

### `RX_READ`
```rust
const RX_READ: u32 = 7u32;
```

### `TX_STATUS`
```rust
const TX_STATUS: u32 = 8u32;
```

### `TX_EXPIRED`
```rust
const TX_EXPIRED: u32 = 9u32;
```

### `RX_STATUS`
```rust
const RX_STATUS: u32 = 10u32;
```

### `RX_TIMEOUT`
```rust
const RX_TIMEOUT: u32 = 11u32;
```

### `RX_CHANGED`
```rust
const RX_CHANGED: u32 = 12u32;
```

### `SETTIMER`
```rust
const SETTIMER: u32 = 1u32;
```

### `STARTTIMER`
```rust
const STARTTIMER: u32 = 2u32;
```

### `TX_COUNTEVT`
```rust
const TX_COUNTEVT: u32 = 4u32;
```

### `TX_ANNOUNCE`
```rust
const TX_ANNOUNCE: u32 = 8u32;
```

### `TX_CP_CAN_ID`
```rust
const TX_CP_CAN_ID: u32 = 16u32;
```

### `RX_FILTER_ID`
```rust
const RX_FILTER_ID: u32 = 32u32;
```

### `RX_CHECK_DLC`
```rust
const RX_CHECK_DLC: u32 = 64u32;
```

### `RX_NO_AUTOTIMER`
```rust
const RX_NO_AUTOTIMER: u32 = 128u32;
```

### `RX_ANNOUNCE_RESUME`
```rust
const RX_ANNOUNCE_RESUME: u32 = 256u32;
```

### `TX_RESET_MULTI_IDX`
```rust
const TX_RESET_MULTI_IDX: u32 = 512u32;
```

### `RX_RTR_FRAME`
```rust
const RX_RTR_FRAME: u32 = 1_024u32;
```

### `CAN_FD_FRAME`
```rust
const CAN_FD_FRAME: u32 = 2_048u32;
```

### `CAN_ERR_DLC`
```rust
const CAN_ERR_DLC: c_int = 8i32;
```

### `CAN_ERR_TX_TIMEOUT`
```rust
const CAN_ERR_TX_TIMEOUT: c_uint = 1u32;
```

### `CAN_ERR_LOSTARB`
```rust
const CAN_ERR_LOSTARB: c_uint = 2u32;
```

### `CAN_ERR_CRTL`
```rust
const CAN_ERR_CRTL: c_uint = 4u32;
```

### `CAN_ERR_PROT`
```rust
const CAN_ERR_PROT: c_uint = 8u32;
```

### `CAN_ERR_TRX`
```rust
const CAN_ERR_TRX: c_uint = 16u32;
```

### `CAN_ERR_ACK`
```rust
const CAN_ERR_ACK: c_uint = 32u32;
```

### `CAN_ERR_BUSOFF`
```rust
const CAN_ERR_BUSOFF: c_uint = 64u32;
```

### `CAN_ERR_BUSERROR`
```rust
const CAN_ERR_BUSERROR: c_uint = 128u32;
```

### `CAN_ERR_RESTARTED`
```rust
const CAN_ERR_RESTARTED: c_uint = 256u32;
```

### `CAN_ERR_CNT`
```rust
const CAN_ERR_CNT: c_uint = 512u32;
```

### `CAN_ERR_LOSTARB_UNSPEC`
```rust
const CAN_ERR_LOSTARB_UNSPEC: c_int = 0i32;
```

### `CAN_ERR_CRTL_UNSPEC`
```rust
const CAN_ERR_CRTL_UNSPEC: c_int = 0i32;
```

### `CAN_ERR_CRTL_RX_OVERFLOW`
```rust
const CAN_ERR_CRTL_RX_OVERFLOW: c_int = 1i32;
```

### `CAN_ERR_CRTL_TX_OVERFLOW`
```rust
const CAN_ERR_CRTL_TX_OVERFLOW: c_int = 2i32;
```

### `CAN_ERR_CRTL_RX_WARNING`
```rust
const CAN_ERR_CRTL_RX_WARNING: c_int = 4i32;
```

### `CAN_ERR_CRTL_TX_WARNING`
```rust
const CAN_ERR_CRTL_TX_WARNING: c_int = 8i32;
```

### `CAN_ERR_CRTL_RX_PASSIVE`
```rust
const CAN_ERR_CRTL_RX_PASSIVE: c_int = 16i32;
```

### `CAN_ERR_CRTL_TX_PASSIVE`
```rust
const CAN_ERR_CRTL_TX_PASSIVE: c_int = 32i32;
```

### `CAN_ERR_CRTL_ACTIVE`
```rust
const CAN_ERR_CRTL_ACTIVE: c_int = 64i32;
```

### `CAN_ERR_PROT_UNSPEC`
```rust
const CAN_ERR_PROT_UNSPEC: c_int = 0i32;
```

### `CAN_ERR_PROT_BIT`
```rust
const CAN_ERR_PROT_BIT: c_int = 1i32;
```

### `CAN_ERR_PROT_FORM`
```rust
const CAN_ERR_PROT_FORM: c_int = 2i32;
```

### `CAN_ERR_PROT_STUFF`
```rust
const CAN_ERR_PROT_STUFF: c_int = 4i32;
```

### `CAN_ERR_PROT_BIT0`
```rust
const CAN_ERR_PROT_BIT0: c_int = 8i32;
```

### `CAN_ERR_PROT_BIT1`
```rust
const CAN_ERR_PROT_BIT1: c_int = 16i32;
```

### `CAN_ERR_PROT_OVERLOAD`
```rust
const CAN_ERR_PROT_OVERLOAD: c_int = 32i32;
```

### `CAN_ERR_PROT_ACTIVE`
```rust
const CAN_ERR_PROT_ACTIVE: c_int = 64i32;
```

### `CAN_ERR_PROT_TX`
```rust
const CAN_ERR_PROT_TX: c_int = 128i32;
```

### `CAN_ERR_PROT_LOC_UNSPEC`
```rust
const CAN_ERR_PROT_LOC_UNSPEC: c_int = 0i32;
```

### `CAN_ERR_PROT_LOC_SOF`
```rust
const CAN_ERR_PROT_LOC_SOF: c_int = 3i32;
```

### `CAN_ERR_PROT_LOC_ID28_21`
```rust
const CAN_ERR_PROT_LOC_ID28_21: c_int = 2i32;
```

### `CAN_ERR_PROT_LOC_ID20_18`
```rust
const CAN_ERR_PROT_LOC_ID20_18: c_int = 6i32;
```

### `CAN_ERR_PROT_LOC_SRTR`
```rust
const CAN_ERR_PROT_LOC_SRTR: c_int = 4i32;
```

### `CAN_ERR_PROT_LOC_IDE`
```rust
const CAN_ERR_PROT_LOC_IDE: c_int = 5i32;
```

### `CAN_ERR_PROT_LOC_ID17_13`
```rust
const CAN_ERR_PROT_LOC_ID17_13: c_int = 7i32;
```

### `CAN_ERR_PROT_LOC_ID12_05`
```rust
const CAN_ERR_PROT_LOC_ID12_05: c_int = 15i32;
```

### `CAN_ERR_PROT_LOC_ID04_00`
```rust
const CAN_ERR_PROT_LOC_ID04_00: c_int = 14i32;
```

### `CAN_ERR_PROT_LOC_RTR`
```rust
const CAN_ERR_PROT_LOC_RTR: c_int = 12i32;
```

### `CAN_ERR_PROT_LOC_RES1`
```rust
const CAN_ERR_PROT_LOC_RES1: c_int = 13i32;
```

### `CAN_ERR_PROT_LOC_RES0`
```rust
const CAN_ERR_PROT_LOC_RES0: c_int = 9i32;
```

### `CAN_ERR_PROT_LOC_DLC`
```rust
const CAN_ERR_PROT_LOC_DLC: c_int = 11i32;
```

### `CAN_ERR_PROT_LOC_DATA`
```rust
const CAN_ERR_PROT_LOC_DATA: c_int = 10i32;
```

### `CAN_ERR_PROT_LOC_CRC_SEQ`
```rust
const CAN_ERR_PROT_LOC_CRC_SEQ: c_int = 8i32;
```

### `CAN_ERR_PROT_LOC_CRC_DEL`
```rust
const CAN_ERR_PROT_LOC_CRC_DEL: c_int = 24i32;
```

### `CAN_ERR_PROT_LOC_ACK`
```rust
const CAN_ERR_PROT_LOC_ACK: c_int = 25i32;
```

### `CAN_ERR_PROT_LOC_ACK_DEL`
```rust
const CAN_ERR_PROT_LOC_ACK_DEL: c_int = 27i32;
```

### `CAN_ERR_PROT_LOC_EOF`
```rust
const CAN_ERR_PROT_LOC_EOF: c_int = 26i32;
```

### `CAN_ERR_PROT_LOC_INTERM`
```rust
const CAN_ERR_PROT_LOC_INTERM: c_int = 18i32;
```

### `CAN_ERR_TRX_UNSPEC`
```rust
const CAN_ERR_TRX_UNSPEC: c_int = 0i32;
```

### `CAN_ERR_TRX_CANH_NO_WIRE`
```rust
const CAN_ERR_TRX_CANH_NO_WIRE: c_int = 4i32;
```

### `CAN_ERR_TRX_CANH_SHORT_TO_BAT`
```rust
const CAN_ERR_TRX_CANH_SHORT_TO_BAT: c_int = 5i32;
```

### `CAN_ERR_TRX_CANH_SHORT_TO_VCC`
```rust
const CAN_ERR_TRX_CANH_SHORT_TO_VCC: c_int = 6i32;
```

### `CAN_ERR_TRX_CANH_SHORT_TO_GND`
```rust
const CAN_ERR_TRX_CANH_SHORT_TO_GND: c_int = 7i32;
```

### `CAN_ERR_TRX_CANL_NO_WIRE`
```rust
const CAN_ERR_TRX_CANL_NO_WIRE: c_int = 64i32;
```

### `CAN_ERR_TRX_CANL_SHORT_TO_BAT`
```rust
const CAN_ERR_TRX_CANL_SHORT_TO_BAT: c_int = 80i32;
```

### `CAN_ERR_TRX_CANL_SHORT_TO_VCC`
```rust
const CAN_ERR_TRX_CANL_SHORT_TO_VCC: c_int = 96i32;
```

### `CAN_ERR_TRX_CANL_SHORT_TO_GND`
```rust
const CAN_ERR_TRX_CANL_SHORT_TO_GND: c_int = 112i32;
```

### `CAN_ERR_TRX_CANL_SHORT_TO_CANH`
```rust
const CAN_ERR_TRX_CANL_SHORT_TO_CANH: c_int = 128i32;
```

### `CAN_ERROR_WARNING_THRESHOLD`
```rust
const CAN_ERROR_WARNING_THRESHOLD: c_int = 96i32;
```

### `CAN_ERROR_PASSIVE_THRESHOLD`
```rust
const CAN_ERROR_PASSIVE_THRESHOLD: c_int = 128i32;
```

### `CAN_BUS_OFF_THRESHOLD`
```rust
const CAN_BUS_OFF_THRESHOLD: c_int = 256i32;
```

### `J1939_MAX_UNICAST_ADDR`
```rust
const J1939_MAX_UNICAST_ADDR: c_uchar = 253u8;
```

### `J1939_IDLE_ADDR`
```rust
const J1939_IDLE_ADDR: c_uchar = 254u8;
```

### `J1939_NO_ADDR`
```rust
const J1939_NO_ADDR: c_uchar = 255u8;
```

### `J1939_NO_NAME`
```rust
const J1939_NO_NAME: c_ulong = 0u64;
```

### `J1939_PGN_REQUEST`
```rust
const J1939_PGN_REQUEST: c_uint = 59_904u32;
```

### `J1939_PGN_ADDRESS_CLAIMED`
```rust
const J1939_PGN_ADDRESS_CLAIMED: c_uint = 60_928u32;
```

### `J1939_PGN_ADDRESS_COMMANDED`
```rust
const J1939_PGN_ADDRESS_COMMANDED: c_uint = 65_240u32;
```

### `J1939_PGN_PDU1_MAX`
```rust
const J1939_PGN_PDU1_MAX: c_uint = 261_888u32;
```

### `J1939_PGN_MAX`
```rust
const J1939_PGN_MAX: c_uint = 262_143u32;
```

### `J1939_NO_PGN`
```rust
const J1939_NO_PGN: c_uint = 262_144u32;
```

### `SOL_CAN_J1939`
```rust
const SOL_CAN_J1939: c_int = 107i32;
```

### `SO_J1939_FILTER`
```rust
const SO_J1939_FILTER: c_int = 1i32;
```

### `SO_J1939_PROMISC`
```rust
const SO_J1939_PROMISC: c_int = 2i32;
```

### `SO_J1939_SEND_PRIO`
```rust
const SO_J1939_SEND_PRIO: c_int = 3i32;
```

### `SO_J1939_ERRQUEUE`
```rust
const SO_J1939_ERRQUEUE: c_int = 4i32;
```

### `SCM_J1939_DEST_ADDR`
```rust
const SCM_J1939_DEST_ADDR: c_int = 1i32;
```

### `SCM_J1939_DEST_NAME`
```rust
const SCM_J1939_DEST_NAME: c_int = 2i32;
```

### `SCM_J1939_PRIO`
```rust
const SCM_J1939_PRIO: c_int = 3i32;
```

### `SCM_J1939_ERRQUEUE`
```rust
const SCM_J1939_ERRQUEUE: c_int = 4i32;
```

### `J1939_NLA_PAD`
```rust
const J1939_NLA_PAD: c_int = 0i32;
```

### `J1939_NLA_BYTES_ACKED`
```rust
const J1939_NLA_BYTES_ACKED: c_int = 1i32;
```

### `J1939_NLA_TOTAL_SIZE`
```rust
const J1939_NLA_TOTAL_SIZE: c_int = 2i32;
```

### `J1939_NLA_PGN`
```rust
const J1939_NLA_PGN: c_int = 3i32;
```

### `J1939_NLA_SRC_NAME`
```rust
const J1939_NLA_SRC_NAME: c_int = 4i32;
```

### `J1939_NLA_DEST_NAME`
```rust
const J1939_NLA_DEST_NAME: c_int = 5i32;
```

### `J1939_NLA_SRC_ADDR`
```rust
const J1939_NLA_SRC_ADDR: c_int = 6i32;
```

### `J1939_NLA_DEST_ADDR`
```rust
const J1939_NLA_DEST_ADDR: c_int = 7i32;
```

### `J1939_EE_INFO_NONE`
```rust
const J1939_EE_INFO_NONE: c_int = 0i32;
```

### `J1939_EE_INFO_TX_ABORT`
```rust
const J1939_EE_INFO_TX_ABORT: c_int = 1i32;
```

### `J1939_EE_INFO_RX_RTS`
```rust
const J1939_EE_INFO_RX_RTS: c_int = 2i32;
```

### `J1939_EE_INFO_RX_DPO`
```rust
const J1939_EE_INFO_RX_DPO: c_int = 3i32;
```

### `J1939_EE_INFO_RX_ABORT`
```rust
const J1939_EE_INFO_RX_ABORT: c_int = 4i32;
```

### `J1939_FILTER_MAX`
```rust
const J1939_FILTER_MAX: c_int = 512i32;
```

### `SOL_CAN_RAW`
```rust
const SOL_CAN_RAW: c_int = 101i32;
```

### `CAN_RAW_FILTER_MAX`
```rust
const CAN_RAW_FILTER_MAX: c_int = 512i32;
```

### `CAN_RAW_FILTER`
```rust
const CAN_RAW_FILTER: c_int = 1i32;
```

### `CAN_RAW_ERR_FILTER`
```rust
const CAN_RAW_ERR_FILTER: c_int = 2i32;
```

### `CAN_RAW_LOOPBACK`
```rust
const CAN_RAW_LOOPBACK: c_int = 3i32;
```

### `CAN_RAW_RECV_OWN_MSGS`
```rust
const CAN_RAW_RECV_OWN_MSGS: c_int = 4i32;
```

### `CAN_RAW_FD_FRAMES`
```rust
const CAN_RAW_FD_FRAMES: c_int = 5i32;
```

### `CAN_RAW_JOIN_FILTERS`
```rust
const CAN_RAW_JOIN_FILTERS: c_int = 6i32;
```

### `CAN_RAW_XL_FRAMES`
```rust
const CAN_RAW_XL_FRAMES: c_int = 7i32;
```

### `CAN_EFF_FLAG`
```rust
const CAN_EFF_FLAG: canid_t = 2_147_483_648u32;
```

### `CAN_RTR_FLAG`
```rust
const CAN_RTR_FLAG: canid_t = 1_073_741_824u32;
```

### `CAN_ERR_FLAG`
```rust
const CAN_ERR_FLAG: canid_t = 536_870_912u32;
```

### `CAN_SFF_MASK`
```rust
const CAN_SFF_MASK: canid_t = 2_047u32;
```

### `CAN_EFF_MASK`
```rust
const CAN_EFF_MASK: canid_t = 536_870_911u32;
```

### `CAN_ERR_MASK`
```rust
const CAN_ERR_MASK: canid_t = 536_870_911u32;
```

### `CANXL_PRIO_MASK`
```rust
const CANXL_PRIO_MASK: crate::canid_t = 2_047u32;
```

### `CAN_SFF_ID_BITS`
```rust
const CAN_SFF_ID_BITS: c_int = 11i32;
```

### `CAN_EFF_ID_BITS`
```rust
const CAN_EFF_ID_BITS: c_int = 29i32;
```

### `CANXL_PRIO_BITS`
```rust
const CANXL_PRIO_BITS: c_int = 11i32;
```

### `CAN_MAX_DLC`
```rust
const CAN_MAX_DLC: c_int = 8i32;
```

### `CAN_MAX_DLEN`
```rust
const CAN_MAX_DLEN: usize = 8usize;
```

### `CANFD_MAX_DLC`
```rust
const CANFD_MAX_DLC: c_int = 15i32;
```

### `CANFD_MAX_DLEN`
```rust
const CANFD_MAX_DLEN: usize = 64usize;
```

### `CANXL_MIN_DLC`
```rust
const CANXL_MIN_DLC: c_int = 0i32;
```

### `CANXL_MAX_DLC`
```rust
const CANXL_MAX_DLC: c_int = 2_047i32;
```

### `CANXL_MAX_DLC_MASK`
```rust
const CANXL_MAX_DLC_MASK: c_int = 2_047i32;
```

### `CANXL_MIN_DLEN`
```rust
const CANXL_MIN_DLEN: usize = 1usize;
```

### `CANXL_MAX_DLEN`
```rust
const CANXL_MAX_DLEN: usize = 2_048usize;
```

### `CANFD_BRS`
```rust
const CANFD_BRS: c_int = 1i32;
```

### `CANFD_ESI`
```rust
const CANFD_ESI: c_int = 2i32;
```

### `CANFD_FDF`
```rust
const CANFD_FDF: c_int = 4i32;
```

### `CANXL_XLF`
```rust
const CANXL_XLF: c_int = 128i32;
```

### `CANXL_SEC`
```rust
const CANXL_SEC: c_int = 1i32;
```

### `CAN_MTU`
```rust
const CAN_MTU: usize = 16usize;
```

### `CANFD_MTU`
```rust
const CANFD_MTU: usize = 72usize;
```

### `CANXL_MTU`
```rust
const CANXL_MTU: usize = 2_060usize;
```

### `CANXL_HDR_SIZE`
```rust
const CANXL_HDR_SIZE: usize = 12usize;
```

### `CANXL_MIN_MTU`
```rust
const CANXL_MIN_MTU: usize = 76usize;
```

### `CANXL_MAX_MTU`
```rust
const CANXL_MAX_MTU: usize = 2_060usize;
```

### `CAN_RAW`
```rust
const CAN_RAW: c_int = 1i32;
```

### `CAN_BCM`
```rust
const CAN_BCM: c_int = 2i32;
```

### `CAN_TP16`
```rust
const CAN_TP16: c_int = 3i32;
```

### `CAN_TP20`
```rust
const CAN_TP20: c_int = 4i32;
```

### `CAN_MCNET`
```rust
const CAN_MCNET: c_int = 5i32;
```

### `CAN_ISOTP`
```rust
const CAN_ISOTP: c_int = 6i32;
```

### `CAN_J1939`
```rust
const CAN_J1939: c_int = 7i32;
```

### `CAN_NPROTO`
```rust
const CAN_NPROTO: c_int = 8i32;
```

### `SOL_CAN_BASE`
```rust
const SOL_CAN_BASE: c_int = 100i32;
```

### `CAN_INV_FILTER`
```rust
const CAN_INV_FILTER: canid_t = 536_870_912u32;
```

### `KEY_SPEC_THREAD_KEYRING`
```rust
const KEY_SPEC_THREAD_KEYRING: i32 = -1i32;
```

### `KEY_SPEC_PROCESS_KEYRING`
```rust
const KEY_SPEC_PROCESS_KEYRING: i32 = -2i32;
```

### `KEY_SPEC_SESSION_KEYRING`
```rust
const KEY_SPEC_SESSION_KEYRING: i32 = -3i32;
```

### `KEY_SPEC_USER_KEYRING`
```rust
const KEY_SPEC_USER_KEYRING: i32 = -4i32;
```

### `KEY_SPEC_USER_SESSION_KEYRING`
```rust
const KEY_SPEC_USER_SESSION_KEYRING: i32 = -5i32;
```

### `KEY_SPEC_GROUP_KEYRING`
```rust
const KEY_SPEC_GROUP_KEYRING: i32 = -6i32;
```

### `KEY_SPEC_REQKEY_AUTH_KEY`
```rust
const KEY_SPEC_REQKEY_AUTH_KEY: i32 = -7i32;
```

### `KEY_SPEC_REQUESTOR_KEYRING`
```rust
const KEY_SPEC_REQUESTOR_KEYRING: i32 = -8i32;
```

### `KEY_REQKEY_DEFL_NO_CHANGE`
```rust
const KEY_REQKEY_DEFL_NO_CHANGE: i32 = -1i32;
```

### `KEY_REQKEY_DEFL_DEFAULT`
```rust
const KEY_REQKEY_DEFL_DEFAULT: i32 = 0i32;
```

### `KEY_REQKEY_DEFL_THREAD_KEYRING`
```rust
const KEY_REQKEY_DEFL_THREAD_KEYRING: i32 = 1i32;
```

### `KEY_REQKEY_DEFL_PROCESS_KEYRING`
```rust
const KEY_REQKEY_DEFL_PROCESS_KEYRING: i32 = 2i32;
```

### `KEY_REQKEY_DEFL_SESSION_KEYRING`
```rust
const KEY_REQKEY_DEFL_SESSION_KEYRING: i32 = 3i32;
```

### `KEY_REQKEY_DEFL_USER_KEYRING`
```rust
const KEY_REQKEY_DEFL_USER_KEYRING: i32 = 4i32;
```

### `KEY_REQKEY_DEFL_USER_SESSION_KEYRING`
```rust
const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: i32 = 5i32;
```

### `KEY_REQKEY_DEFL_GROUP_KEYRING`
```rust
const KEY_REQKEY_DEFL_GROUP_KEYRING: i32 = 6i32;
```

### `KEY_REQKEY_DEFL_REQUESTOR_KEYRING`
```rust
const KEY_REQKEY_DEFL_REQUESTOR_KEYRING: i32 = 7i32;
```

### `KEYCTL_GET_KEYRING_ID`
```rust
const KEYCTL_GET_KEYRING_ID: u32 = 0u32;
```

### `KEYCTL_JOIN_SESSION_KEYRING`
```rust
const KEYCTL_JOIN_SESSION_KEYRING: u32 = 1u32;
```

### `KEYCTL_UPDATE`
```rust
const KEYCTL_UPDATE: u32 = 2u32;
```

### `KEYCTL_REVOKE`
```rust
const KEYCTL_REVOKE: u32 = 3u32;
```

### `KEYCTL_CHOWN`
```rust
const KEYCTL_CHOWN: u32 = 4u32;
```

### `KEYCTL_SETPERM`
```rust
const KEYCTL_SETPERM: u32 = 5u32;
```

### `KEYCTL_DESCRIBE`
```rust
const KEYCTL_DESCRIBE: u32 = 6u32;
```

### `KEYCTL_CLEAR`
```rust
const KEYCTL_CLEAR: u32 = 7u32;
```

### `KEYCTL_LINK`
```rust
const KEYCTL_LINK: u32 = 8u32;
```

### `KEYCTL_UNLINK`
```rust
const KEYCTL_UNLINK: u32 = 9u32;
```

### `KEYCTL_SEARCH`
```rust
const KEYCTL_SEARCH: u32 = 10u32;
```

### `KEYCTL_READ`
```rust
const KEYCTL_READ: u32 = 11u32;
```

### `KEYCTL_INSTANTIATE`
```rust
const KEYCTL_INSTANTIATE: u32 = 12u32;
```

### `KEYCTL_NEGATE`
```rust
const KEYCTL_NEGATE: u32 = 13u32;
```

### `KEYCTL_SET_REQKEY_KEYRING`
```rust
const KEYCTL_SET_REQKEY_KEYRING: u32 = 14u32;
```

### `KEYCTL_SET_TIMEOUT`
```rust
const KEYCTL_SET_TIMEOUT: u32 = 15u32;
```

### `KEYCTL_ASSUME_AUTHORITY`
```rust
const KEYCTL_ASSUME_AUTHORITY: u32 = 16u32;
```

### `KEYCTL_GET_SECURITY`
```rust
const KEYCTL_GET_SECURITY: u32 = 17u32;
```

### `KEYCTL_SESSION_TO_PARENT`
```rust
const KEYCTL_SESSION_TO_PARENT: u32 = 18u32;
```

### `KEYCTL_REJECT`
```rust
const KEYCTL_REJECT: u32 = 19u32;
```

### `KEYCTL_INSTANTIATE_IOV`
```rust
const KEYCTL_INSTANTIATE_IOV: u32 = 20u32;
```

### `KEYCTL_INVALIDATE`
```rust
const KEYCTL_INVALIDATE: u32 = 21u32;
```

### `KEYCTL_GET_PERSISTENT`
```rust
const KEYCTL_GET_PERSISTENT: u32 = 22u32;
```

### `KEYCTL_DH_COMPUTE`
```rust
const KEYCTL_DH_COMPUTE: u32 = 23u32;
```

### `KEYCTL_PKEY_QUERY`
```rust
const KEYCTL_PKEY_QUERY: u32 = 24u32;
```

### `KEYCTL_PKEY_ENCRYPT`
```rust
const KEYCTL_PKEY_ENCRYPT: u32 = 25u32;
```

### `KEYCTL_PKEY_DECRYPT`
```rust
const KEYCTL_PKEY_DECRYPT: u32 = 26u32;
```

### `KEYCTL_PKEY_SIGN`
```rust
const KEYCTL_PKEY_SIGN: u32 = 27u32;
```

### `KEYCTL_PKEY_VERIFY`
```rust
const KEYCTL_PKEY_VERIFY: u32 = 28u32;
```

### `KEYCTL_RESTRICT_KEYRING`
```rust
const KEYCTL_RESTRICT_KEYRING: u32 = 29u32;
```

### `KEYCTL_MOVE`
```rust
const KEYCTL_MOVE: u32 = 30u32;
```

### `KEYCTL_CAPABILITIES`
```rust
const KEYCTL_CAPABILITIES: u32 = 31u32;
```

### `KEYCTL_SUPPORTS_ENCRYPT`
```rust
const KEYCTL_SUPPORTS_ENCRYPT: u32 = 1u32;
```

### `KEYCTL_SUPPORTS_DECRYPT`
```rust
const KEYCTL_SUPPORTS_DECRYPT: u32 = 2u32;
```

### `KEYCTL_SUPPORTS_SIGN`
```rust
const KEYCTL_SUPPORTS_SIGN: u32 = 4u32;
```

### `KEYCTL_SUPPORTS_VERIFY`
```rust
const KEYCTL_SUPPORTS_VERIFY: u32 = 8u32;
```

### `KEYCTL_CAPS0_CAPABILITIES`
```rust
const KEYCTL_CAPS0_CAPABILITIES: u32 = 1u32;
```

### `KEYCTL_CAPS0_PERSISTENT_KEYRINGS`
```rust
const KEYCTL_CAPS0_PERSISTENT_KEYRINGS: u32 = 2u32;
```

### `KEYCTL_CAPS0_DIFFIE_HELLMAN`
```rust
const KEYCTL_CAPS0_DIFFIE_HELLMAN: u32 = 4u32;
```

### `KEYCTL_CAPS0_PUBLIC_KEY`
```rust
const KEYCTL_CAPS0_PUBLIC_KEY: u32 = 8u32;
```

### `KEYCTL_CAPS0_BIG_KEY`
```rust
const KEYCTL_CAPS0_BIG_KEY: u32 = 16u32;
```

### `KEYCTL_CAPS0_INVALIDATE`
```rust
const KEYCTL_CAPS0_INVALIDATE: u32 = 32u32;
```

### `KEYCTL_CAPS0_RESTRICT_KEYRING`
```rust
const KEYCTL_CAPS0_RESTRICT_KEYRING: u32 = 64u32;
```

### `KEYCTL_CAPS0_MOVE`
```rust
const KEYCTL_CAPS0_MOVE: u32 = 128u32;
```

### `KEYCTL_CAPS1_NS_KEYRING_NAME`
```rust
const KEYCTL_CAPS1_NS_KEYRING_NAME: u32 = 1u32;
```

### `KEYCTL_CAPS1_NS_KEY_TAG`
```rust
const KEYCTL_CAPS1_NS_KEY_TAG: u32 = 2u32;
```

### `MEMBARRIER_CMD_QUERY`
```rust
const MEMBARRIER_CMD_QUERY: membarrier_cmd = 0i32;
```

### `MEMBARRIER_CMD_GLOBAL`
```rust
const MEMBARRIER_CMD_GLOBAL: membarrier_cmd = 1i32;
```

### `MEMBARRIER_CMD_GLOBAL_EXPEDITED`
```rust
const MEMBARRIER_CMD_GLOBAL_EXPEDITED: membarrier_cmd = 2i32;
```

### `MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`
```rust
const MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED: membarrier_cmd = 4i32;
```

### `MEMBARRIER_CMD_PRIVATE_EXPEDITED`
```rust
const MEMBARRIER_CMD_PRIVATE_EXPEDITED: membarrier_cmd = 8i32;
```

### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`
```rust
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED: membarrier_cmd = 16i32;
```

### `MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`
```rust
const MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE: membarrier_cmd = 32i32;
```

### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`
```rust
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE: membarrier_cmd = 64i32;
```

### `MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ`
```rust
const MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ: membarrier_cmd = 128i32;
```

### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ`
```rust
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ: membarrier_cmd = 256i32;
```

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

### `PIDFD_NONBLOCK`
```rust
const PIDFD_NONBLOCK: c_uint = 2_048u32;
```

### `PIDFD_THREAD`
```rust
const PIDFD_THREAD: c_uint = 128u32;
```

### `PIDFD_SIGNAL_THREAD`
```rust
const PIDFD_SIGNAL_THREAD: c_uint = 1u32;
```

### `PIDFD_SIGNAL_THREAD_GROUP`
```rust
const PIDFD_SIGNAL_THREAD_GROUP: c_uint = 2u32;
```

### `PIDFD_SIGNAL_PROCESS_GROUP`
```rust
const PIDFD_SIGNAL_PROCESS_GROUP: c_uint = 4u32;
```

### `PIDFD_INFO_PID`
```rust
const PIDFD_INFO_PID: c_uint = 1u32;
```

### `PIDFD_INFO_CREDS`
```rust
const PIDFD_INFO_CREDS: c_uint = 2u32;
```

### `PIDFD_INFO_CGROUPID`
```rust
const PIDFD_INFO_CGROUPID: c_uint = 4u32;
```

### `PIDFD_INFO_EXIT`
```rust
const PIDFD_INFO_EXIT: c_uint = 8u32;
```

### `PIDFD_INFO_SIZE_VER0`
```rust
const PIDFD_INFO_SIZE_VER0: c_uint = 64u32;
```

### `PIDFS_IOCTL_MAGIC`
```rust
const PIDFS_IOCTL_MAGIC: c_uint = 255u32;
```

### `PIDFD_GET_CGROUP_NAMESPACE`
```rust
const PIDFD_GET_CGROUP_NAMESPACE: c_ulong = 65_281u64;
```

### `PIDFD_GET_IPC_NAMESPACE`
```rust
const PIDFD_GET_IPC_NAMESPACE: c_ulong = 65_282u64;
```

### `PIDFD_GET_MNT_NAMESPACE`
```rust
const PIDFD_GET_MNT_NAMESPACE: c_ulong = 65_283u64;
```

### `PIDFD_GET_NET_NAMESPACE`
```rust
const PIDFD_GET_NET_NAMESPACE: c_ulong = 65_284u64;
```

### `PIDFD_GET_PID_NAMESPACE`
```rust
const PIDFD_GET_PID_NAMESPACE: c_ulong = 65_285u64;
```

### `PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE`
```rust
const PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE: c_ulong = 65_286u64;
```

### `PIDFD_GET_TIME_NAMESPACE`
```rust
const PIDFD_GET_TIME_NAMESPACE: c_ulong = 65_287u64;
```

### `PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE`
```rust
const PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE: c_ulong = 65_288u64;
```

### `PIDFD_GET_USER_NAMESPACE`
```rust
const PIDFD_GET_USER_NAMESPACE: c_ulong = 65_289u64;
```

### `PIDFD_GET_UTS_NAMESPACE`
```rust
const PIDFD_GET_UTS_NAMESPACE: c_ulong = 65_290u64;
```

### `PIDFD_GET_INFO`
```rust
const PIDFD_GET_INFO: c_ulong = 3_225_485_067u64;
```

