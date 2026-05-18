**libc > unix > linux_like > linux > arch > generic**

# Module: unix::linux_like::linux::arch::generic

## Contents

**Structs**

- [`termios2`](#termios2)

**Constants**

- [`BLKIOMIN`](#blkiomin)
- [`BLKIOOPT`](#blkioopt)
- [`BLKPBSZGET`](#blkpbszget)
- [`BLKSSZGET`](#blksszget)
- [`BOTHER`](#bother)
- [`FIOASYNC`](#fioasync)
- [`FIOCLEX`](#fioclex)
- [`FIONBIO`](#fionbio)
- [`FIONCLEX`](#fionclex)
- [`FIONREAD`](#fionread)
- [`FIOQSIZE`](#fioqsize)
- [`IBSHIFT`](#ibshift)
- [`IUCLC`](#iuclc)
- [`RLIMIT_AS`](#rlimit_as)
- [`RLIMIT_CORE`](#rlimit_core)
- [`RLIMIT_CPU`](#rlimit_cpu)
- [`RLIMIT_DATA`](#rlimit_data)
- [`RLIMIT_FSIZE`](#rlimit_fsize)
- [`RLIMIT_LOCKS`](#rlimit_locks)
- [`RLIMIT_MEMLOCK`](#rlimit_memlock)
- [`RLIMIT_MSGQUEUE`](#rlimit_msgqueue)
- [`RLIMIT_NICE`](#rlimit_nice)
- [`RLIMIT_NLIMITS`](#rlimit_nlimits)
- [`RLIMIT_NOFILE`](#rlimit_nofile)
- [`RLIMIT_NPROC`](#rlimit_nproc)
- [`RLIMIT_RSS`](#rlimit_rss)
- [`RLIMIT_RTPRIO`](#rlimit_rtprio)
- [`RLIMIT_RTTIME`](#rlimit_rttime)
- [`RLIMIT_SIGPENDING`](#rlimit_sigpending)
- [`RLIMIT_STACK`](#rlimit_stack)
- [`RLIM_INFINITY`](#rlim_infinity)
- [`RLIM_NLIMITS`](#rlim_nlimits)
- [`SCM_DEVMEM_DMABUF`](#scm_devmem_dmabuf)
- [`SCM_DEVMEM_LINEAR`](#scm_devmem_linear)
- [`SCM_TIMESTAMPING`](#scm_timestamping)
- [`SCM_TIMESTAMPING_OPT_STATS`](#scm_timestamping_opt_stats)
- [`SCM_TIMESTAMPING_PKTINFO`](#scm_timestamping_pktinfo)
- [`SCM_TIMESTAMPNS`](#scm_timestampns)
- [`SCM_TXTIME`](#scm_txtime)
- [`SCM_WIFI_STATUS`](#scm_wifi_status)
- [`SOL_SOCKET`](#sol_socket)
- [`SO_ACCEPTCONN`](#so_acceptconn)
- [`SO_ATTACH_BPF`](#so_attach_bpf)
- [`SO_ATTACH_FILTER`](#so_attach_filter)
- [`SO_ATTACH_REUSEPORT_CBPF`](#so_attach_reuseport_cbpf)
- [`SO_ATTACH_REUSEPORT_EBPF`](#so_attach_reuseport_ebpf)
- [`SO_BINDTODEVICE`](#so_bindtodevice)
- [`SO_BINDTOIFINDEX`](#so_bindtoifindex)
- [`SO_BPF_EXTENSIONS`](#so_bpf_extensions)
- [`SO_BROADCAST`](#so_broadcast)
- [`SO_BSDCOMPAT`](#so_bsdcompat)
- [`SO_BUF_LOCK`](#so_buf_lock)
- [`SO_BUSY_POLL`](#so_busy_poll)
- [`SO_BUSY_POLL_BUDGET`](#so_busy_poll_budget)
- [`SO_CNX_ADVICE`](#so_cnx_advice)
- [`SO_COOKIE`](#so_cookie)
- [`SO_DETACH_BPF`](#so_detach_bpf)
- [`SO_DETACH_FILTER`](#so_detach_filter)
- [`SO_DETACH_REUSEPORT_BPF`](#so_detach_reuseport_bpf)
- [`SO_DEVMEM_DMABUF`](#so_devmem_dmabuf)
- [`SO_DEVMEM_DONTNEED`](#so_devmem_dontneed)
- [`SO_DEVMEM_LINEAR`](#so_devmem_linear)
- [`SO_DOMAIN`](#so_domain)
- [`SO_DONTROUTE`](#so_dontroute)
- [`SO_ERROR`](#so_error)
- [`SO_GET_FILTER`](#so_get_filter)
- [`SO_INCOMING_CPU`](#so_incoming_cpu)
- [`SO_INCOMING_NAPI_ID`](#so_incoming_napi_id)
- [`SO_KEEPALIVE`](#so_keepalive)
- [`SO_LINGER`](#so_linger)
- [`SO_LOCK_FILTER`](#so_lock_filter)
- [`SO_MARK`](#so_mark)
- [`SO_MAX_PACING_RATE`](#so_max_pacing_rate)
- [`SO_MEMINFO`](#so_meminfo)
- [`SO_NETNS_COOKIE`](#so_netns_cookie)
- [`SO_NOFCS`](#so_nofcs)
- [`SO_NO_CHECK`](#so_no_check)
- [`SO_OOBINLINE`](#so_oobinline)
- [`SO_PASSCRED`](#so_passcred)
- [`SO_PASSPIDFD`](#so_passpidfd)
- [`SO_PASSSEC`](#so_passsec)
- [`SO_PEEK_OFF`](#so_peek_off)
- [`SO_PEERCRED`](#so_peercred)
- [`SO_PEERGROUPS`](#so_peergroups)
- [`SO_PEERNAME`](#so_peername)
- [`SO_PEERPIDFD`](#so_peerpidfd)
- [`SO_PEERSEC`](#so_peersec)
- [`SO_PREFER_BUSY_POLL`](#so_prefer_busy_poll)
- [`SO_PRIORITY`](#so_priority)
- [`SO_PROTOCOL`](#so_protocol)
- [`SO_RCVBUF`](#so_rcvbuf)
- [`SO_RCVBUFFORCE`](#so_rcvbufforce)
- [`SO_RCVLOWAT`](#so_rcvlowat)
- [`SO_RCVMARK`](#so_rcvmark)
- [`SO_RCVTIMEO`](#so_rcvtimeo)
- [`SO_RCVTIMEO_NEW`](#so_rcvtimeo_new)
- [`SO_RESERVE_MEM`](#so_reserve_mem)
- [`SO_REUSEADDR`](#so_reuseaddr)
- [`SO_REUSEPORT`](#so_reuseport)
- [`SO_RXQ_OVFL`](#so_rxq_ovfl)
- [`SO_SECURITY_AUTHENTICATION`](#so_security_authentication)
- [`SO_SECURITY_ENCRYPTION_NETWORK`](#so_security_encryption_network)
- [`SO_SECURITY_ENCRYPTION_TRANSPORT`](#so_security_encryption_transport)
- [`SO_SELECT_ERR_QUEUE`](#so_select_err_queue)
- [`SO_SNDBUF`](#so_sndbuf)
- [`SO_SNDBUFFORCE`](#so_sndbufforce)
- [`SO_SNDLOWAT`](#so_sndlowat)
- [`SO_SNDTIMEO`](#so_sndtimeo)
- [`SO_SNDTIMEO_NEW`](#so_sndtimeo_new)
- [`SO_TIMESTAMP`](#so_timestamp)
- [`SO_TIMESTAMPING`](#so_timestamping)
- [`SO_TIMESTAMPING_NEW`](#so_timestamping_new)
- [`SO_TIMESTAMPNS`](#so_timestampns)
- [`SO_TIMESTAMPNS_NEW`](#so_timestampns_new)
- [`SO_TIMESTAMP_NEW`](#so_timestamp_new)
- [`SO_TXREHASH`](#so_txrehash)
- [`SO_TXTIME`](#so_txtime)
- [`SO_TYPE`](#so_type)
- [`SO_WIFI_STATUS`](#so_wifi_status)
- [`SO_ZEROCOPY`](#so_zerocopy)
- [`TCFLSH`](#tcflsh)
- [`TCGETA`](#tcgeta)
- [`TCGETS`](#tcgets)
- [`TCGETS2`](#tcgets2)
- [`TCGETX`](#tcgetx)
- [`TCSBRK`](#tcsbrk)
- [`TCSBRKP`](#tcsbrkp)
- [`TCSETA`](#tcseta)
- [`TCSETAF`](#tcsetaf)
- [`TCSETAW`](#tcsetaw)
- [`TCSETS`](#tcsets)
- [`TCSETS2`](#tcsets2)
- [`TCSETSF`](#tcsetsf)
- [`TCSETSF2`](#tcsetsf2)
- [`TCSETSW`](#tcsetsw)
- [`TCSETSW2`](#tcsetsw2)
- [`TCSETX`](#tcsetx)
- [`TCSETXF`](#tcsetxf)
- [`TCSETXW`](#tcsetxw)
- [`TCXONC`](#tcxonc)
- [`TIOCCBRK`](#tioccbrk)
- [`TIOCCONS`](#tioccons)
- [`TIOCEXCL`](#tiocexcl)
- [`TIOCGDEV`](#tiocgdev)
- [`TIOCGETD`](#tiocgetd)
- [`TIOCGEXCL`](#tiocgexcl)
- [`TIOCGICOUNT`](#tiocgicount)
- [`TIOCGLCKTRMIOS`](#tiocglcktrmios)
- [`TIOCGPGRP`](#tiocgpgrp)
- [`TIOCGPKT`](#tiocgpkt)
- [`TIOCGPTLCK`](#tiocgptlck)
- [`TIOCGPTN`](#tiocgptn)
- [`TIOCGPTPEER`](#tiocgptpeer)
- [`TIOCGRS485`](#tiocgrs485)
- [`TIOCGSERIAL`](#tiocgserial)
- [`TIOCGSID`](#tiocgsid)
- [`TIOCGSOFTCAR`](#tiocgsoftcar)
- [`TIOCGWINSZ`](#tiocgwinsz)
- [`TIOCINQ`](#tiocinq)
- [`TIOCLINUX`](#tioclinux)
- [`TIOCMBIC`](#tiocmbic)
- [`TIOCMBIS`](#tiocmbis)
- [`TIOCMGET`](#tiocmget)
- [`TIOCMIWAIT`](#tiocmiwait)
- [`TIOCMSET`](#tiocmset)
- [`TIOCM_CAR`](#tiocm_car)
- [`TIOCM_CD`](#tiocm_cd)
- [`TIOCM_CTS`](#tiocm_cts)
- [`TIOCM_DSR`](#tiocm_dsr)
- [`TIOCM_DTR`](#tiocm_dtr)
- [`TIOCM_LE`](#tiocm_le)
- [`TIOCM_RI`](#tiocm_ri)
- [`TIOCM_RNG`](#tiocm_rng)
- [`TIOCM_RTS`](#tiocm_rts)
- [`TIOCM_SR`](#tiocm_sr)
- [`TIOCM_ST`](#tiocm_st)
- [`TIOCNOTTY`](#tiocnotty)
- [`TIOCNXCL`](#tiocnxcl)
- [`TIOCOUTQ`](#tiocoutq)
- [`TIOCPKT`](#tiocpkt)
- [`TIOCSBRK`](#tiocsbrk)
- [`TIOCSCTTY`](#tiocsctty)
- [`TIOCSERCONFIG`](#tiocserconfig)
- [`TIOCSERGETLSR`](#tiocsergetlsr)
- [`TIOCSERGETMULTI`](#tiocsergetmulti)
- [`TIOCSERGSTRUCT`](#tiocsergstruct)
- [`TIOCSERGWILD`](#tiocsergwild)
- [`TIOCSERSETMULTI`](#tiocsersetmulti)
- [`TIOCSERSWILD`](#tiocserswild)
- [`TIOCSETD`](#tiocsetd)
- [`TIOCSIG`](#tiocsig)
- [`TIOCSLCKTRMIOS`](#tiocslcktrmios)
- [`TIOCSPGRP`](#tiocspgrp)
- [`TIOCSPTLCK`](#tiocsptlck)
- [`TIOCSRS485`](#tiocsrs485)
- [`TIOCSSERIAL`](#tiocsserial)
- [`TIOCSSOFTCAR`](#tiocssoftcar)
- [`TIOCSTI`](#tiocsti)
- [`TIOCSWINSZ`](#tiocswinsz)
- [`TIOCVHANGUP`](#tiocvhangup)
- [`XCASE`](#xcase)

---

## libc::unix::linux_like::linux::arch::generic::BLKIOMIN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::BLKIOOPT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::BLKPBSZGET

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::BLKSSZGET

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::BOTHER

*Constant*: `crate::speed_t`



## libc::unix::linux_like::linux::arch::generic::FIOASYNC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::FIOCLEX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::FIONBIO

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::FIONCLEX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::FIONREAD

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::FIOQSIZE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::IBSHIFT

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::linux::arch::generic::IUCLC

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_AS

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_CORE

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_CPU

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_DATA

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_FSIZE

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_LOCKS

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_MEMLOCK

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_MSGQUEUE

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_NICE

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_NLIMITS

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_NOFILE

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_NPROC

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_RSS

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_RTPRIO

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_RTTIME

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_SIGPENDING

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIMIT_STACK

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::RLIM_INFINITY

*Constant*: `crate::rlim_t`



## libc::unix::linux_like::linux::arch::generic::RLIM_NLIMITS

*Constant*: `crate::__rlimit_resource_t`



## libc::unix::linux_like::linux::arch::generic::SCM_DEVMEM_DMABUF

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SCM_DEVMEM_LINEAR

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SCM_TIMESTAMPING

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SCM_TIMESTAMPING_OPT_STATS

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SCM_TIMESTAMPING_PKTINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SCM_TIMESTAMPNS

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SCM_TXTIME

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SCM_WIFI_STATUS

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SOL_SOCKET

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_ACCEPTCONN

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_ATTACH_BPF

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_ATTACH_FILTER

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_ATTACH_REUSEPORT_CBPF

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_ATTACH_REUSEPORT_EBPF

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_BINDTODEVICE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_BINDTOIFINDEX

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_BPF_EXTENSIONS

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_BROADCAST

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_BSDCOMPAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_BUF_LOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_BUSY_POLL

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_BUSY_POLL_BUDGET

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_CNX_ADVICE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_COOKIE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_DETACH_BPF

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_DETACH_FILTER

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_DETACH_REUSEPORT_BPF

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_DEVMEM_DMABUF

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_DEVMEM_DONTNEED

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_DEVMEM_LINEAR

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_DOMAIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_DONTROUTE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_ERROR

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_GET_FILTER

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_INCOMING_CPU

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_INCOMING_NAPI_ID

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_KEEPALIVE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_LINGER

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_LOCK_FILTER

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_MARK

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_MAX_PACING_RATE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_MEMINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_NETNS_COOKIE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_NOFCS

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_NO_CHECK

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_OOBINLINE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PASSCRED

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PASSPIDFD

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PASSSEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PEEK_OFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PEERCRED

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PEERGROUPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PEERNAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PEERPIDFD

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PEERSEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PREFER_BUSY_POLL

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PRIORITY

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_PROTOCOL

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_RCVBUF

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_RCVBUFFORCE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_RCVLOWAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_RCVMARK

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_RCVTIMEO

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_RCVTIMEO_NEW

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_RESERVE_MEM

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_REUSEADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_REUSEPORT

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_RXQ_OVFL

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_SECURITY_AUTHENTICATION

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_SECURITY_ENCRYPTION_NETWORK

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_SECURITY_ENCRYPTION_TRANSPORT

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_SELECT_ERR_QUEUE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_SNDBUF

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_SNDBUFFORCE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_SNDLOWAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_SNDTIMEO

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_SNDTIMEO_NEW

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_TIMESTAMP

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_TIMESTAMPING

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_TIMESTAMPING_NEW

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_TIMESTAMPNS

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_TIMESTAMPNS_NEW

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_TIMESTAMP_NEW

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_TXREHASH

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_TXTIME

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_TYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_WIFI_STATUS

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::SO_ZEROCOPY

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TCFLSH

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCGETA

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCGETS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCGETS2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCGETX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSBRK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSBRKP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETA

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETAF

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETAW

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETS2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETSF

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETSF2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETSW

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETSW2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETXF

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCSETXW

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TCXONC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCCBRK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCCONS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCEXCL

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGDEV

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGETD

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGEXCL

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGICOUNT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGLCKTRMIOS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGPGRP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGPKT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGPTLCK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGPTN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGPTPEER

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGRS485

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGSERIAL

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGSID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGSOFTCAR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCGWINSZ

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCINQ

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCLINUX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCMBIC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCMBIS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCMGET

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCMIWAIT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCMSET

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCM_CAR

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCM_CD

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCM_CTS

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCM_DSR

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCM_DTR

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCM_LE

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCM_RI

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCM_RNG

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCM_RTS

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCM_SR

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCM_ST

*Constant*: `c_int`



## libc::unix::linux_like::linux::arch::generic::TIOCNOTTY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCNXCL

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCOUTQ

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCPKT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSBRK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSCTTY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSERCONFIG

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSERGETLSR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSERGETMULTI

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSERGSTRUCT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSERGWILD

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSERSETMULTI

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSERSWILD

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSETD

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSIG

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSLCKTRMIOS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSPGRP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSPTLCK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSRS485

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSSERIAL

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSSOFTCAR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSTI

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCSWINSZ

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::TIOCVHANGUP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::arch::generic::XCASE

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::linux::arch::generic::termios2

*Struct*

**Fields:**
- `c_iflag: crate::tcflag_t`
- `c_oflag: crate::tcflag_t`
- `c_cflag: crate::tcflag_t`
- `c_lflag: crate::tcflag_t`
- `c_line: crate::cc_t`
- `c_cc: [crate::cc_t; 19]`
- `c_ispeed: crate::speed_t`
- `c_ospeed: crate::speed_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> termios2`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



