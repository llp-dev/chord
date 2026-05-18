*[libc](../../../../../index.md) / [unix](../../../../index.md) / [linux_like](../../../index.md) / [linux](../../index.md) / [arch](../index.md) / [generic](index.md)*

---

# Module `generic`

## Contents

- [Structs](#structs)
  - [`termios2`](#termios2)
- [Constants](#constants)
  - [`SOL_SOCKET`](#sol-socket)
  - [`SO_REUSEADDR`](#so-reuseaddr)
  - [`SO_TYPE`](#so-type)
  - [`SO_ERROR`](#so-error)
  - [`SO_DONTROUTE`](#so-dontroute)
  - [`SO_BROADCAST`](#so-broadcast)
  - [`SO_SNDBUF`](#so-sndbuf)
  - [`SO_RCVBUF`](#so-rcvbuf)
  - [`SO_KEEPALIVE`](#so-keepalive)
  - [`SO_OOBINLINE`](#so-oobinline)
  - [`SO_NO_CHECK`](#so-no-check)
  - [`SO_PRIORITY`](#so-priority)
  - [`SO_LINGER`](#so-linger)
  - [`SO_BSDCOMPAT`](#so-bsdcompat)
  - [`SO_REUSEPORT`](#so-reuseport)
  - [`SO_PASSCRED`](#so-passcred)
  - [`SO_PEERCRED`](#so-peercred)
  - [`SO_RCVLOWAT`](#so-rcvlowat)
  - [`SO_SNDLOWAT`](#so-sndlowat)
  - [`SO_SECURITY_AUTHENTICATION`](#so-security-authentication)
  - [`SO_SECURITY_ENCRYPTION_TRANSPORT`](#so-security-encryption-transport)
  - [`SO_SECURITY_ENCRYPTION_NETWORK`](#so-security-encryption-network)
  - [`SO_BINDTODEVICE`](#so-bindtodevice)
  - [`SO_ATTACH_FILTER`](#so-attach-filter)
  - [`SO_DETACH_FILTER`](#so-detach-filter)
  - [`SO_GET_FILTER`](#so-get-filter)
  - [`SO_PEERNAME`](#so-peername)
  - [`SO_TIMESTAMP_OLD`](#so-timestamp-old)
  - [`SO_TIMESTAMPNS_OLD`](#so-timestampns-old)
  - [`SO_TIMESTAMPING_OLD`](#so-timestamping-old)
  - [`SO_RCVTIMEO_OLD`](#so-rcvtimeo-old)
  - [`SO_SNDTIMEO_OLD`](#so-sndtimeo-old)
  - [`SO_TIMESTAMP`](#so-timestamp)
  - [`SO_TIMESTAMPNS`](#so-timestampns)
  - [`SO_TIMESTAMPING`](#so-timestamping)
  - [`SO_RCVTIMEO`](#so-rcvtimeo)
  - [`SO_SNDTIMEO`](#so-sndtimeo)
  - [`SO_ACCEPTCONN`](#so-acceptconn)
  - [`SO_PEERSEC`](#so-peersec)
  - [`SO_SNDBUFFORCE`](#so-sndbufforce)
  - [`SO_RCVBUFFORCE`](#so-rcvbufforce)
  - [`SO_PASSSEC`](#so-passsec)
  - [`SO_MARK`](#so-mark)
  - [`SO_PROTOCOL`](#so-protocol)
  - [`SO_DOMAIN`](#so-domain)
  - [`SO_RXQ_OVFL`](#so-rxq-ovfl)
  - [`SO_WIFI_STATUS`](#so-wifi-status)
  - [`SCM_WIFI_STATUS`](#scm-wifi-status)
  - [`SO_PEEK_OFF`](#so-peek-off)
  - [`SO_NOFCS`](#so-nofcs)
  - [`SO_LOCK_FILTER`](#so-lock-filter)
  - [`SO_SELECT_ERR_QUEUE`](#so-select-err-queue)
  - [`SO_BUSY_POLL`](#so-busy-poll)
  - [`SO_MAX_PACING_RATE`](#so-max-pacing-rate)
  - [`SO_BPF_EXTENSIONS`](#so-bpf-extensions)
  - [`SO_INCOMING_CPU`](#so-incoming-cpu)
  - [`SO_ATTACH_BPF`](#so-attach-bpf)
  - [`SO_DETACH_BPF`](#so-detach-bpf)
  - [`SO_ATTACH_REUSEPORT_CBPF`](#so-attach-reuseport-cbpf)
  - [`SO_ATTACH_REUSEPORT_EBPF`](#so-attach-reuseport-ebpf)
  - [`SO_CNX_ADVICE`](#so-cnx-advice)
  - [`SCM_TIMESTAMPING_OPT_STATS`](#scm-timestamping-opt-stats)
  - [`SO_MEMINFO`](#so-meminfo)
  - [`SO_INCOMING_NAPI_ID`](#so-incoming-napi-id)
  - [`SO_COOKIE`](#so-cookie)
  - [`SCM_TIMESTAMPING_PKTINFO`](#scm-timestamping-pktinfo)
  - [`SO_PEERGROUPS`](#so-peergroups)
  - [`SO_ZEROCOPY`](#so-zerocopy)
  - [`SO_TXTIME`](#so-txtime)
  - [`SCM_TXTIME`](#scm-txtime)
  - [`SO_BINDTOIFINDEX`](#so-bindtoifindex)
  - [`SO_TIMESTAMP_NEW`](#so-timestamp-new)
  - [`SO_TIMESTAMPNS_NEW`](#so-timestampns-new)
  - [`SO_TIMESTAMPING_NEW`](#so-timestamping-new)
  - [`SO_RCVTIMEO_NEW`](#so-rcvtimeo-new)
  - [`SO_SNDTIMEO_NEW`](#so-sndtimeo-new)
  - [`SO_DETACH_REUSEPORT_BPF`](#so-detach-reuseport-bpf)
  - [`SO_PREFER_BUSY_POLL`](#so-prefer-busy-poll)
  - [`SO_BUSY_POLL_BUDGET`](#so-busy-poll-budget)
  - [`SO_NETNS_COOKIE`](#so-netns-cookie)
  - [`SO_BUF_LOCK`](#so-buf-lock)
  - [`SO_RESERVE_MEM`](#so-reserve-mem)
  - [`SO_TXREHASH`](#so-txrehash)
  - [`SO_RCVMARK`](#so-rcvmark)
  - [`SO_PASSPIDFD`](#so-passpidfd)
  - [`SO_PEERPIDFD`](#so-peerpidfd)
  - [`SO_DEVMEM_LINEAR`](#so-devmem-linear)
  - [`SO_DEVMEM_DMABUF`](#so-devmem-dmabuf)
  - [`SO_DEVMEM_DONTNEED`](#so-devmem-dontneed)
  - [`SCM_TIMESTAMPNS`](#scm-timestampns)
  - [`SCM_TIMESTAMPING`](#scm-timestamping)
  - [`SCM_DEVMEM_LINEAR`](#scm-devmem-linear)
  - [`SCM_DEVMEM_DMABUF`](#scm-devmem-dmabuf)
  - [`TCGETS`](#tcgets)
  - [`TCSETS`](#tcsets)
  - [`TCSETSW`](#tcsetsw)
  - [`TCSETSF`](#tcsetsf)
  - [`TCGETA`](#tcgeta)
  - [`TCSETA`](#tcseta)
  - [`TCSETAW`](#tcsetaw)
  - [`TCSETAF`](#tcsetaf)
  - [`TCSBRK`](#tcsbrk)
  - [`TCXONC`](#tcxonc)
  - [`TCFLSH`](#tcflsh)
  - [`TIOCEXCL`](#tiocexcl)
  - [`TIOCNXCL`](#tiocnxcl)
  - [`TIOCSCTTY`](#tiocsctty)
  - [`TIOCGPGRP`](#tiocgpgrp)
  - [`TIOCSPGRP`](#tiocspgrp)
  - [`TIOCOUTQ`](#tiocoutq)
  - [`TIOCSTI`](#tiocsti)
  - [`TIOCGWINSZ`](#tiocgwinsz)
  - [`TIOCSWINSZ`](#tiocswinsz)
  - [`TIOCMGET`](#tiocmget)
  - [`TIOCMBIS`](#tiocmbis)
  - [`TIOCMBIC`](#tiocmbic)
  - [`TIOCMSET`](#tiocmset)
  - [`TIOCGSOFTCAR`](#tiocgsoftcar)
  - [`TIOCSSOFTCAR`](#tiocssoftcar)
  - [`FIONREAD`](#fionread)
  - [`TIOCINQ`](#tiocinq)
  - [`TIOCLINUX`](#tioclinux)
  - [`TIOCCONS`](#tioccons)
  - [`TIOCGSERIAL`](#tiocgserial)
  - [`TIOCSSERIAL`](#tiocsserial)
  - [`TIOCPKT`](#tiocpkt)
  - [`FIONBIO`](#fionbio)
  - [`TIOCNOTTY`](#tiocnotty)
  - [`TIOCSETD`](#tiocsetd)
  - [`TIOCGETD`](#tiocgetd)
  - [`TCSBRKP`](#tcsbrkp)
  - [`TIOCSBRK`](#tiocsbrk)
  - [`TIOCCBRK`](#tioccbrk)
  - [`TIOCGSID`](#tiocgsid)
  - [`TCGETS2`](#tcgets2)
  - [`TCSETS2`](#tcsets2)
  - [`TCSETSW2`](#tcsetsw2)
  - [`TCSETSF2`](#tcsetsf2)
  - [`TIOCGRS485`](#tiocgrs485)
  - [`TIOCSRS485`](#tiocsrs485)
  - [`TIOCGPTN`](#tiocgptn)
  - [`TIOCSPTLCK`](#tiocsptlck)
  - [`TIOCGDEV`](#tiocgdev)
  - [`TCGETX`](#tcgetx)
  - [`TCSETX`](#tcsetx)
  - [`TCSETXF`](#tcsetxf)
  - [`TCSETXW`](#tcsetxw)
  - [`TIOCSIG`](#tiocsig)
  - [`TIOCVHANGUP`](#tiocvhangup)
  - [`TIOCGPKT`](#tiocgpkt)
  - [`TIOCGPTLCK`](#tiocgptlck)
  - [`TIOCGEXCL`](#tiocgexcl)
  - [`TIOCGPTPEER`](#tiocgptpeer)
  - [`FIONCLEX`](#fionclex)
  - [`FIOCLEX`](#fioclex)
  - [`FIOASYNC`](#fioasync)
  - [`TIOCSERCONFIG`](#tiocserconfig)
  - [`TIOCSERGWILD`](#tiocsergwild)
  - [`TIOCSERSWILD`](#tiocserswild)
  - [`TIOCGLCKTRMIOS`](#tiocglcktrmios)
  - [`TIOCSLCKTRMIOS`](#tiocslcktrmios)
  - [`TIOCSERGSTRUCT`](#tiocsergstruct)
  - [`TIOCSERGETLSR`](#tiocsergetlsr)
  - [`TIOCSERGETMULTI`](#tiocsergetmulti)
  - [`TIOCSERSETMULTI`](#tiocsersetmulti)
  - [`TIOCMIWAIT`](#tiocmiwait)
  - [`TIOCGICOUNT`](#tiocgicount)
  - [`BLKIOMIN`](#blkiomin)
  - [`BLKIOOPT`](#blkioopt)
  - [`BLKSSZGET`](#blksszget)
  - [`BLKPBSZGET`](#blkpbszget)
  - [`FIOQSIZE`](#fioqsize)
  - [`TIOCM_LE`](#tiocm-le)
  - [`TIOCM_DTR`](#tiocm-dtr)
  - [`TIOCM_RTS`](#tiocm-rts)
  - [`TIOCM_ST`](#tiocm-st)
  - [`TIOCM_SR`](#tiocm-sr)
  - [`TIOCM_CTS`](#tiocm-cts)
  - [`TIOCM_CAR`](#tiocm-car)
  - [`TIOCM_CD`](#tiocm-cd)
  - [`TIOCM_RNG`](#tiocm-rng)
  - [`TIOCM_RI`](#tiocm-ri)
  - [`TIOCM_DSR`](#tiocm-dsr)
  - [`BOTHER`](#bother)
  - [`IBSHIFT`](#ibshift)
  - [`IUCLC`](#iuclc)
  - [`XCASE`](#xcase)
  - [`RLIMIT_CPU`](#rlimit-cpu)
  - [`RLIMIT_FSIZE`](#rlimit-fsize)
  - [`RLIMIT_DATA`](#rlimit-data)
  - [`RLIMIT_STACK`](#rlimit-stack)
  - [`RLIMIT_CORE`](#rlimit-core)
  - [`RLIMIT_RSS`](#rlimit-rss)
  - [`RLIMIT_NPROC`](#rlimit-nproc)
  - [`RLIMIT_NOFILE`](#rlimit-nofile)
  - [`RLIMIT_MEMLOCK`](#rlimit-memlock)
  - [`RLIMIT_AS`](#rlimit-as)
  - [`RLIMIT_LOCKS`](#rlimit-locks)
  - [`RLIMIT_SIGPENDING`](#rlimit-sigpending)
  - [`RLIMIT_MSGQUEUE`](#rlimit-msgqueue)
  - [`RLIMIT_NICE`](#rlimit-nice)
  - [`RLIMIT_RTPRIO`](#rlimit-rtprio)
  - [`RLIMIT_RTTIME`](#rlimit-rttime)
  - [`RLIMIT_NLIMITS`](#rlimit-nlimits)
  - [`RLIM_NLIMITS`](#rlim-nlimits)
  - [`RLIM_INFINITY`](#rlim-infinity)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`termios2`](#termios2) | struct |  |
| [`SOL_SOCKET`](#sol-socket) | const |  |
| [`SO_REUSEADDR`](#so-reuseaddr) | const |  |
| [`SO_TYPE`](#so-type) | const |  |
| [`SO_ERROR`](#so-error) | const |  |
| [`SO_DONTROUTE`](#so-dontroute) | const |  |
| [`SO_BROADCAST`](#so-broadcast) | const |  |
| [`SO_SNDBUF`](#so-sndbuf) | const |  |
| [`SO_RCVBUF`](#so-rcvbuf) | const |  |
| [`SO_KEEPALIVE`](#so-keepalive) | const |  |
| [`SO_OOBINLINE`](#so-oobinline) | const |  |
| [`SO_NO_CHECK`](#so-no-check) | const |  |
| [`SO_PRIORITY`](#so-priority) | const |  |
| [`SO_LINGER`](#so-linger) | const |  |
| [`SO_BSDCOMPAT`](#so-bsdcompat) | const |  |
| [`SO_REUSEPORT`](#so-reuseport) | const |  |
| [`SO_PASSCRED`](#so-passcred) | const |  |
| [`SO_PEERCRED`](#so-peercred) | const |  |
| [`SO_RCVLOWAT`](#so-rcvlowat) | const |  |
| [`SO_SNDLOWAT`](#so-sndlowat) | const |  |
| [`SO_SECURITY_AUTHENTICATION`](#so-security-authentication) | const |  |
| [`SO_SECURITY_ENCRYPTION_TRANSPORT`](#so-security-encryption-transport) | const |  |
| [`SO_SECURITY_ENCRYPTION_NETWORK`](#so-security-encryption-network) | const |  |
| [`SO_BINDTODEVICE`](#so-bindtodevice) | const |  |
| [`SO_ATTACH_FILTER`](#so-attach-filter) | const |  |
| [`SO_DETACH_FILTER`](#so-detach-filter) | const |  |
| [`SO_GET_FILTER`](#so-get-filter) | const |  |
| [`SO_PEERNAME`](#so-peername) | const |  |
| [`SO_TIMESTAMP_OLD`](#so-timestamp-old) | const |  |
| [`SO_TIMESTAMPNS_OLD`](#so-timestampns-old) | const |  |
| [`SO_TIMESTAMPING_OLD`](#so-timestamping-old) | const |  |
| [`SO_RCVTIMEO_OLD`](#so-rcvtimeo-old) | const |  |
| [`SO_SNDTIMEO_OLD`](#so-sndtimeo-old) | const |  |
| [`SO_TIMESTAMP`](#so-timestamp) | const |  |
| [`SO_TIMESTAMPNS`](#so-timestampns) | const |  |
| [`SO_TIMESTAMPING`](#so-timestamping) | const |  |
| [`SO_RCVTIMEO`](#so-rcvtimeo) | const |  |
| [`SO_SNDTIMEO`](#so-sndtimeo) | const |  |
| [`SO_ACCEPTCONN`](#so-acceptconn) | const |  |
| [`SO_PEERSEC`](#so-peersec) | const |  |
| [`SO_SNDBUFFORCE`](#so-sndbufforce) | const |  |
| [`SO_RCVBUFFORCE`](#so-rcvbufforce) | const |  |
| [`SO_PASSSEC`](#so-passsec) | const |  |
| [`SO_MARK`](#so-mark) | const |  |
| [`SO_PROTOCOL`](#so-protocol) | const |  |
| [`SO_DOMAIN`](#so-domain) | const |  |
| [`SO_RXQ_OVFL`](#so-rxq-ovfl) | const |  |
| [`SO_WIFI_STATUS`](#so-wifi-status) | const |  |
| [`SCM_WIFI_STATUS`](#scm-wifi-status) | const |  |
| [`SO_PEEK_OFF`](#so-peek-off) | const |  |
| [`SO_NOFCS`](#so-nofcs) | const |  |
| [`SO_LOCK_FILTER`](#so-lock-filter) | const |  |
| [`SO_SELECT_ERR_QUEUE`](#so-select-err-queue) | const |  |
| [`SO_BUSY_POLL`](#so-busy-poll) | const |  |
| [`SO_MAX_PACING_RATE`](#so-max-pacing-rate) | const |  |
| [`SO_BPF_EXTENSIONS`](#so-bpf-extensions) | const |  |
| [`SO_INCOMING_CPU`](#so-incoming-cpu) | const |  |
| [`SO_ATTACH_BPF`](#so-attach-bpf) | const |  |
| [`SO_DETACH_BPF`](#so-detach-bpf) | const |  |
| [`SO_ATTACH_REUSEPORT_CBPF`](#so-attach-reuseport-cbpf) | const |  |
| [`SO_ATTACH_REUSEPORT_EBPF`](#so-attach-reuseport-ebpf) | const |  |
| [`SO_CNX_ADVICE`](#so-cnx-advice) | const |  |
| [`SCM_TIMESTAMPING_OPT_STATS`](#scm-timestamping-opt-stats) | const |  |
| [`SO_MEMINFO`](#so-meminfo) | const |  |
| [`SO_INCOMING_NAPI_ID`](#so-incoming-napi-id) | const |  |
| [`SO_COOKIE`](#so-cookie) | const |  |
| [`SCM_TIMESTAMPING_PKTINFO`](#scm-timestamping-pktinfo) | const |  |
| [`SO_PEERGROUPS`](#so-peergroups) | const |  |
| [`SO_ZEROCOPY`](#so-zerocopy) | const |  |
| [`SO_TXTIME`](#so-txtime) | const |  |
| [`SCM_TXTIME`](#scm-txtime) | const |  |
| [`SO_BINDTOIFINDEX`](#so-bindtoifindex) | const |  |
| [`SO_TIMESTAMP_NEW`](#so-timestamp-new) | const |  |
| [`SO_TIMESTAMPNS_NEW`](#so-timestampns-new) | const |  |
| [`SO_TIMESTAMPING_NEW`](#so-timestamping-new) | const |  |
| [`SO_RCVTIMEO_NEW`](#so-rcvtimeo-new) | const |  |
| [`SO_SNDTIMEO_NEW`](#so-sndtimeo-new) | const |  |
| [`SO_DETACH_REUSEPORT_BPF`](#so-detach-reuseport-bpf) | const |  |
| [`SO_PREFER_BUSY_POLL`](#so-prefer-busy-poll) | const |  |
| [`SO_BUSY_POLL_BUDGET`](#so-busy-poll-budget) | const |  |
| [`SO_NETNS_COOKIE`](#so-netns-cookie) | const |  |
| [`SO_BUF_LOCK`](#so-buf-lock) | const |  |
| [`SO_RESERVE_MEM`](#so-reserve-mem) | const |  |
| [`SO_TXREHASH`](#so-txrehash) | const |  |
| [`SO_RCVMARK`](#so-rcvmark) | const |  |
| [`SO_PASSPIDFD`](#so-passpidfd) | const |  |
| [`SO_PEERPIDFD`](#so-peerpidfd) | const |  |
| [`SO_DEVMEM_LINEAR`](#so-devmem-linear) | const |  |
| [`SO_DEVMEM_DMABUF`](#so-devmem-dmabuf) | const |  |
| [`SO_DEVMEM_DONTNEED`](#so-devmem-dontneed) | const |  |
| [`SCM_TIMESTAMPNS`](#scm-timestampns) | const |  |
| [`SCM_TIMESTAMPING`](#scm-timestamping) | const |  |
| [`SCM_DEVMEM_LINEAR`](#scm-devmem-linear) | const |  |
| [`SCM_DEVMEM_DMABUF`](#scm-devmem-dmabuf) | const |  |
| [`TCGETS`](#tcgets) | const |  |
| [`TCSETS`](#tcsets) | const |  |
| [`TCSETSW`](#tcsetsw) | const |  |
| [`TCSETSF`](#tcsetsf) | const |  |
| [`TCGETA`](#tcgeta) | const |  |
| [`TCSETA`](#tcseta) | const |  |
| [`TCSETAW`](#tcsetaw) | const |  |
| [`TCSETAF`](#tcsetaf) | const |  |
| [`TCSBRK`](#tcsbrk) | const |  |
| [`TCXONC`](#tcxonc) | const |  |
| [`TCFLSH`](#tcflsh) | const |  |
| [`TIOCEXCL`](#tiocexcl) | const |  |
| [`TIOCNXCL`](#tiocnxcl) | const |  |
| [`TIOCSCTTY`](#tiocsctty) | const |  |
| [`TIOCGPGRP`](#tiocgpgrp) | const |  |
| [`TIOCSPGRP`](#tiocspgrp) | const |  |
| [`TIOCOUTQ`](#tiocoutq) | const |  |
| [`TIOCSTI`](#tiocsti) | const |  |
| [`TIOCGWINSZ`](#tiocgwinsz) | const |  |
| [`TIOCSWINSZ`](#tiocswinsz) | const |  |
| [`TIOCMGET`](#tiocmget) | const |  |
| [`TIOCMBIS`](#tiocmbis) | const |  |
| [`TIOCMBIC`](#tiocmbic) | const |  |
| [`TIOCMSET`](#tiocmset) | const |  |
| [`TIOCGSOFTCAR`](#tiocgsoftcar) | const |  |
| [`TIOCSSOFTCAR`](#tiocssoftcar) | const |  |
| [`FIONREAD`](#fionread) | const |  |
| [`TIOCINQ`](#tiocinq) | const |  |
| [`TIOCLINUX`](#tioclinux) | const |  |
| [`TIOCCONS`](#tioccons) | const |  |
| [`TIOCGSERIAL`](#tiocgserial) | const |  |
| [`TIOCSSERIAL`](#tiocsserial) | const |  |
| [`TIOCPKT`](#tiocpkt) | const |  |
| [`FIONBIO`](#fionbio) | const |  |
| [`TIOCNOTTY`](#tiocnotty) | const |  |
| [`TIOCSETD`](#tiocsetd) | const |  |
| [`TIOCGETD`](#tiocgetd) | const |  |
| [`TCSBRKP`](#tcsbrkp) | const |  |
| [`TIOCSBRK`](#tiocsbrk) | const |  |
| [`TIOCCBRK`](#tioccbrk) | const |  |
| [`TIOCGSID`](#tiocgsid) | const |  |
| [`TCGETS2`](#tcgets2) | const |  |
| [`TCSETS2`](#tcsets2) | const |  |
| [`TCSETSW2`](#tcsetsw2) | const |  |
| [`TCSETSF2`](#tcsetsf2) | const |  |
| [`TIOCGRS485`](#tiocgrs485) | const |  |
| [`TIOCSRS485`](#tiocsrs485) | const |  |
| [`TIOCGPTN`](#tiocgptn) | const |  |
| [`TIOCSPTLCK`](#tiocsptlck) | const |  |
| [`TIOCGDEV`](#tiocgdev) | const |  |
| [`TCGETX`](#tcgetx) | const |  |
| [`TCSETX`](#tcsetx) | const |  |
| [`TCSETXF`](#tcsetxf) | const |  |
| [`TCSETXW`](#tcsetxw) | const |  |
| [`TIOCSIG`](#tiocsig) | const |  |
| [`TIOCVHANGUP`](#tiocvhangup) | const |  |
| [`TIOCGPKT`](#tiocgpkt) | const |  |
| [`TIOCGPTLCK`](#tiocgptlck) | const |  |
| [`TIOCGEXCL`](#tiocgexcl) | const |  |
| [`TIOCGPTPEER`](#tiocgptpeer) | const |  |
| [`FIONCLEX`](#fionclex) | const |  |
| [`FIOCLEX`](#fioclex) | const |  |
| [`FIOASYNC`](#fioasync) | const |  |
| [`TIOCSERCONFIG`](#tiocserconfig) | const |  |
| [`TIOCSERGWILD`](#tiocsergwild) | const |  |
| [`TIOCSERSWILD`](#tiocserswild) | const |  |
| [`TIOCGLCKTRMIOS`](#tiocglcktrmios) | const |  |
| [`TIOCSLCKTRMIOS`](#tiocslcktrmios) | const |  |
| [`TIOCSERGSTRUCT`](#tiocsergstruct) | const |  |
| [`TIOCSERGETLSR`](#tiocsergetlsr) | const |  |
| [`TIOCSERGETMULTI`](#tiocsergetmulti) | const |  |
| [`TIOCSERSETMULTI`](#tiocsersetmulti) | const |  |
| [`TIOCMIWAIT`](#tiocmiwait) | const |  |
| [`TIOCGICOUNT`](#tiocgicount) | const |  |
| [`BLKIOMIN`](#blkiomin) | const |  |
| [`BLKIOOPT`](#blkioopt) | const |  |
| [`BLKSSZGET`](#blksszget) | const |  |
| [`BLKPBSZGET`](#blkpbszget) | const |  |
| [`FIOQSIZE`](#fioqsize) | const |  |
| [`TIOCM_LE`](#tiocm-le) | const |  |
| [`TIOCM_DTR`](#tiocm-dtr) | const |  |
| [`TIOCM_RTS`](#tiocm-rts) | const |  |
| [`TIOCM_ST`](#tiocm-st) | const |  |
| [`TIOCM_SR`](#tiocm-sr) | const |  |
| [`TIOCM_CTS`](#tiocm-cts) | const |  |
| [`TIOCM_CAR`](#tiocm-car) | const |  |
| [`TIOCM_CD`](#tiocm-cd) | const |  |
| [`TIOCM_RNG`](#tiocm-rng) | const |  |
| [`TIOCM_RI`](#tiocm-ri) | const |  |
| [`TIOCM_DSR`](#tiocm-dsr) | const |  |
| [`BOTHER`](#bother) | const |  |
| [`IBSHIFT`](#ibshift) | const |  |
| [`IUCLC`](#iuclc) | const |  |
| [`XCASE`](#xcase) | const |  |
| [`RLIMIT_CPU`](#rlimit-cpu) | const |  |
| [`RLIMIT_FSIZE`](#rlimit-fsize) | const |  |
| [`RLIMIT_DATA`](#rlimit-data) | const |  |
| [`RLIMIT_STACK`](#rlimit-stack) | const |  |
| [`RLIMIT_CORE`](#rlimit-core) | const |  |
| [`RLIMIT_RSS`](#rlimit-rss) | const |  |
| [`RLIMIT_NPROC`](#rlimit-nproc) | const |  |
| [`RLIMIT_NOFILE`](#rlimit-nofile) | const |  |
| [`RLIMIT_MEMLOCK`](#rlimit-memlock) | const |  |
| [`RLIMIT_AS`](#rlimit-as) | const |  |
| [`RLIMIT_LOCKS`](#rlimit-locks) | const |  |
| [`RLIMIT_SIGPENDING`](#rlimit-sigpending) | const |  |
| [`RLIMIT_MSGQUEUE`](#rlimit-msgqueue) | const |  |
| [`RLIMIT_NICE`](#rlimit-nice) | const |  |
| [`RLIMIT_RTPRIO`](#rlimit-rtprio) | const |  |
| [`RLIMIT_RTTIME`](#rlimit-rttime) | const |  |
| [`RLIMIT_NLIMITS`](#rlimit-nlimits) | const |  |
| [`RLIM_NLIMITS`](#rlim-nlimits) | const |  |
| [`RLIM_INFINITY`](#rlim-infinity) | const |  |

## Structs

### `termios2`

```rust
struct termios2 {
    pub c_iflag: crate::tcflag_t,
    pub c_oflag: crate::tcflag_t,
    pub c_cflag: crate::tcflag_t,
    pub c_lflag: crate::tcflag_t,
    pub c_line: crate::cc_t,
    pub c_cc: [crate::cc_t; 19],
    pub c_ispeed: crate::speed_t,
    pub c_ospeed: crate::speed_t,
}
```

#### Trait Implementations

##### `impl Clone for termios2`

- <span id="termios2-clone"></span>`fn clone(&self) -> termios2` — [`termios2`](../index.md#termios2)

##### `impl Copy for termios2`

##### `impl Debug for termios2`

- <span id="termios2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Constants

### `SOL_SOCKET`
```rust
const SOL_SOCKET: c_int = 1i32;
```

### `SO_REUSEADDR`
```rust
const SO_REUSEADDR: c_int = 2i32;
```

### `SO_TYPE`
```rust
const SO_TYPE: c_int = 3i32;
```

### `SO_ERROR`
```rust
const SO_ERROR: c_int = 4i32;
```

### `SO_DONTROUTE`
```rust
const SO_DONTROUTE: c_int = 5i32;
```

### `SO_BROADCAST`
```rust
const SO_BROADCAST: c_int = 6i32;
```

### `SO_SNDBUF`
```rust
const SO_SNDBUF: c_int = 7i32;
```

### `SO_RCVBUF`
```rust
const SO_RCVBUF: c_int = 8i32;
```

### `SO_KEEPALIVE`
```rust
const SO_KEEPALIVE: c_int = 9i32;
```

### `SO_OOBINLINE`
```rust
const SO_OOBINLINE: c_int = 10i32;
```

### `SO_NO_CHECK`
```rust
const SO_NO_CHECK: c_int = 11i32;
```

### `SO_PRIORITY`
```rust
const SO_PRIORITY: c_int = 12i32;
```

### `SO_LINGER`
```rust
const SO_LINGER: c_int = 13i32;
```

### `SO_BSDCOMPAT`
```rust
const SO_BSDCOMPAT: c_int = 14i32;
```

### `SO_REUSEPORT`
```rust
const SO_REUSEPORT: c_int = 15i32;
```

### `SO_PASSCRED`
```rust
const SO_PASSCRED: c_int = 16i32;
```

### `SO_PEERCRED`
```rust
const SO_PEERCRED: c_int = 17i32;
```

### `SO_RCVLOWAT`
```rust
const SO_RCVLOWAT: c_int = 18i32;
```

### `SO_SNDLOWAT`
```rust
const SO_SNDLOWAT: c_int = 19i32;
```

### `SO_SECURITY_AUTHENTICATION`
```rust
const SO_SECURITY_AUTHENTICATION: c_int = 22i32;
```

### `SO_SECURITY_ENCRYPTION_TRANSPORT`
```rust
const SO_SECURITY_ENCRYPTION_TRANSPORT: c_int = 23i32;
```

### `SO_SECURITY_ENCRYPTION_NETWORK`
```rust
const SO_SECURITY_ENCRYPTION_NETWORK: c_int = 24i32;
```

### `SO_BINDTODEVICE`
```rust
const SO_BINDTODEVICE: c_int = 25i32;
```

### `SO_ATTACH_FILTER`
```rust
const SO_ATTACH_FILTER: c_int = 26i32;
```

### `SO_DETACH_FILTER`
```rust
const SO_DETACH_FILTER: c_int = 27i32;
```

### `SO_GET_FILTER`
```rust
const SO_GET_FILTER: c_int = 26i32;
```

### `SO_PEERNAME`
```rust
const SO_PEERNAME: c_int = 28i32;
```

### `SO_TIMESTAMP_OLD`
```rust
const SO_TIMESTAMP_OLD: c_int = 29i32;
```

### `SO_TIMESTAMPNS_OLD`
```rust
const SO_TIMESTAMPNS_OLD: c_int = 35i32;
```

### `SO_TIMESTAMPING_OLD`
```rust
const SO_TIMESTAMPING_OLD: c_int = 37i32;
```

### `SO_RCVTIMEO_OLD`
```rust
const SO_RCVTIMEO_OLD: c_int = 20i32;
```

### `SO_SNDTIMEO_OLD`
```rust
const SO_SNDTIMEO_OLD: c_int = 21i32;
```

### `SO_TIMESTAMP`
```rust
const SO_TIMESTAMP: c_int = 29i32;
```

### `SO_TIMESTAMPNS`
```rust
const SO_TIMESTAMPNS: c_int = 35i32;
```

### `SO_TIMESTAMPING`
```rust
const SO_TIMESTAMPING: c_int = 37i32;
```

### `SO_RCVTIMEO`
```rust
const SO_RCVTIMEO: c_int = 20i32;
```

### `SO_SNDTIMEO`
```rust
const SO_SNDTIMEO: c_int = 21i32;
```

### `SO_ACCEPTCONN`
```rust
const SO_ACCEPTCONN: c_int = 30i32;
```

### `SO_PEERSEC`
```rust
const SO_PEERSEC: c_int = 31i32;
```

### `SO_SNDBUFFORCE`
```rust
const SO_SNDBUFFORCE: c_int = 32i32;
```

### `SO_RCVBUFFORCE`
```rust
const SO_RCVBUFFORCE: c_int = 33i32;
```

### `SO_PASSSEC`
```rust
const SO_PASSSEC: c_int = 34i32;
```

### `SO_MARK`
```rust
const SO_MARK: c_int = 36i32;
```

### `SO_PROTOCOL`
```rust
const SO_PROTOCOL: c_int = 38i32;
```

### `SO_DOMAIN`
```rust
const SO_DOMAIN: c_int = 39i32;
```

### `SO_RXQ_OVFL`
```rust
const SO_RXQ_OVFL: c_int = 40i32;
```

### `SO_WIFI_STATUS`
```rust
const SO_WIFI_STATUS: c_int = 41i32;
```

### `SCM_WIFI_STATUS`
```rust
const SCM_WIFI_STATUS: c_int = 41i32;
```

### `SO_PEEK_OFF`
```rust
const SO_PEEK_OFF: c_int = 42i32;
```

### `SO_NOFCS`
```rust
const SO_NOFCS: c_int = 43i32;
```

### `SO_LOCK_FILTER`
```rust
const SO_LOCK_FILTER: c_int = 44i32;
```

### `SO_SELECT_ERR_QUEUE`
```rust
const SO_SELECT_ERR_QUEUE: c_int = 45i32;
```

### `SO_BUSY_POLL`
```rust
const SO_BUSY_POLL: c_int = 46i32;
```

### `SO_MAX_PACING_RATE`
```rust
const SO_MAX_PACING_RATE: c_int = 47i32;
```

### `SO_BPF_EXTENSIONS`
```rust
const SO_BPF_EXTENSIONS: c_int = 48i32;
```

### `SO_INCOMING_CPU`
```rust
const SO_INCOMING_CPU: c_int = 49i32;
```

### `SO_ATTACH_BPF`
```rust
const SO_ATTACH_BPF: c_int = 50i32;
```

### `SO_DETACH_BPF`
```rust
const SO_DETACH_BPF: c_int = 27i32;
```

### `SO_ATTACH_REUSEPORT_CBPF`
```rust
const SO_ATTACH_REUSEPORT_CBPF: c_int = 51i32;
```

### `SO_ATTACH_REUSEPORT_EBPF`
```rust
const SO_ATTACH_REUSEPORT_EBPF: c_int = 52i32;
```

### `SO_CNX_ADVICE`
```rust
const SO_CNX_ADVICE: c_int = 53i32;
```

### `SCM_TIMESTAMPING_OPT_STATS`
```rust
const SCM_TIMESTAMPING_OPT_STATS: c_int = 54i32;
```

### `SO_MEMINFO`
```rust
const SO_MEMINFO: c_int = 55i32;
```

### `SO_INCOMING_NAPI_ID`
```rust
const SO_INCOMING_NAPI_ID: c_int = 56i32;
```

### `SO_COOKIE`
```rust
const SO_COOKIE: c_int = 57i32;
```

### `SCM_TIMESTAMPING_PKTINFO`
```rust
const SCM_TIMESTAMPING_PKTINFO: c_int = 58i32;
```

### `SO_PEERGROUPS`
```rust
const SO_PEERGROUPS: c_int = 59i32;
```

### `SO_ZEROCOPY`
```rust
const SO_ZEROCOPY: c_int = 60i32;
```

### `SO_TXTIME`
```rust
const SO_TXTIME: c_int = 61i32;
```

### `SCM_TXTIME`
```rust
const SCM_TXTIME: c_int = 61i32;
```

### `SO_BINDTOIFINDEX`
```rust
const SO_BINDTOIFINDEX: c_int = 62i32;
```

### `SO_TIMESTAMP_NEW`
```rust
const SO_TIMESTAMP_NEW: c_int = 63i32;
```

### `SO_TIMESTAMPNS_NEW`
```rust
const SO_TIMESTAMPNS_NEW: c_int = 64i32;
```

### `SO_TIMESTAMPING_NEW`
```rust
const SO_TIMESTAMPING_NEW: c_int = 65i32;
```

### `SO_RCVTIMEO_NEW`
```rust
const SO_RCVTIMEO_NEW: c_int = 66i32;
```

### `SO_SNDTIMEO_NEW`
```rust
const SO_SNDTIMEO_NEW: c_int = 67i32;
```

### `SO_DETACH_REUSEPORT_BPF`
```rust
const SO_DETACH_REUSEPORT_BPF: c_int = 68i32;
```

### `SO_PREFER_BUSY_POLL`
```rust
const SO_PREFER_BUSY_POLL: c_int = 69i32;
```

### `SO_BUSY_POLL_BUDGET`
```rust
const SO_BUSY_POLL_BUDGET: c_int = 70i32;
```

### `SO_NETNS_COOKIE`
```rust
const SO_NETNS_COOKIE: c_int = 71i32;
```

### `SO_BUF_LOCK`
```rust
const SO_BUF_LOCK: c_int = 72i32;
```

### `SO_RESERVE_MEM`
```rust
const SO_RESERVE_MEM: c_int = 73i32;
```

### `SO_TXREHASH`
```rust
const SO_TXREHASH: c_int = 74i32;
```

### `SO_RCVMARK`
```rust
const SO_RCVMARK: c_int = 75i32;
```

### `SO_PASSPIDFD`
```rust
const SO_PASSPIDFD: c_int = 76i32;
```

### `SO_PEERPIDFD`
```rust
const SO_PEERPIDFD: c_int = 77i32;
```

### `SO_DEVMEM_LINEAR`
```rust
const SO_DEVMEM_LINEAR: c_int = 78i32;
```

### `SO_DEVMEM_DMABUF`
```rust
const SO_DEVMEM_DMABUF: c_int = 79i32;
```

### `SO_DEVMEM_DONTNEED`
```rust
const SO_DEVMEM_DONTNEED: c_int = 80i32;
```

### `SCM_TIMESTAMPNS`
```rust
const SCM_TIMESTAMPNS: c_int = 35i32;
```

### `SCM_TIMESTAMPING`
```rust
const SCM_TIMESTAMPING: c_int = 37i32;
```

### `SCM_DEVMEM_LINEAR`
```rust
const SCM_DEVMEM_LINEAR: c_int = 78i32;
```

### `SCM_DEVMEM_DMABUF`
```rust
const SCM_DEVMEM_DMABUF: c_int = 79i32;
```

### `TCGETS`
```rust
const TCGETS: c_ulong = 21_505u64;
```

### `TCSETS`
```rust
const TCSETS: c_ulong = 21_506u64;
```

### `TCSETSW`
```rust
const TCSETSW: c_ulong = 21_507u64;
```

### `TCSETSF`
```rust
const TCSETSF: c_ulong = 21_508u64;
```

### `TCGETA`
```rust
const TCGETA: c_ulong = 21_509u64;
```

### `TCSETA`
```rust
const TCSETA: c_ulong = 21_510u64;
```

### `TCSETAW`
```rust
const TCSETAW: c_ulong = 21_511u64;
```

### `TCSETAF`
```rust
const TCSETAF: c_ulong = 21_512u64;
```

### `TCSBRK`
```rust
const TCSBRK: c_ulong = 21_513u64;
```

### `TCXONC`
```rust
const TCXONC: c_ulong = 21_514u64;
```

### `TCFLSH`
```rust
const TCFLSH: c_ulong = 21_515u64;
```

### `TIOCEXCL`
```rust
const TIOCEXCL: c_ulong = 21_516u64;
```

### `TIOCNXCL`
```rust
const TIOCNXCL: c_ulong = 21_517u64;
```

### `TIOCSCTTY`
```rust
const TIOCSCTTY: c_ulong = 21_518u64;
```

### `TIOCGPGRP`
```rust
const TIOCGPGRP: c_ulong = 21_519u64;
```

### `TIOCSPGRP`
```rust
const TIOCSPGRP: c_ulong = 21_520u64;
```

### `TIOCOUTQ`
```rust
const TIOCOUTQ: c_ulong = 21_521u64;
```

### `TIOCSTI`
```rust
const TIOCSTI: c_ulong = 21_522u64;
```

### `TIOCGWINSZ`
```rust
const TIOCGWINSZ: c_ulong = 21_523u64;
```

### `TIOCSWINSZ`
```rust
const TIOCSWINSZ: c_ulong = 21_524u64;
```

### `TIOCMGET`
```rust
const TIOCMGET: c_ulong = 21_525u64;
```

### `TIOCMBIS`
```rust
const TIOCMBIS: c_ulong = 21_526u64;
```

### `TIOCMBIC`
```rust
const TIOCMBIC: c_ulong = 21_527u64;
```

### `TIOCMSET`
```rust
const TIOCMSET: c_ulong = 21_528u64;
```

### `TIOCGSOFTCAR`
```rust
const TIOCGSOFTCAR: c_ulong = 21_529u64;
```

### `TIOCSSOFTCAR`
```rust
const TIOCSSOFTCAR: c_ulong = 21_530u64;
```

### `FIONREAD`
```rust
const FIONREAD: c_ulong = 21_531u64;
```

### `TIOCINQ`
```rust
const TIOCINQ: c_ulong = 21_531u64;
```

### `TIOCLINUX`
```rust
const TIOCLINUX: c_ulong = 21_532u64;
```

### `TIOCCONS`
```rust
const TIOCCONS: c_ulong = 21_533u64;
```

### `TIOCGSERIAL`
```rust
const TIOCGSERIAL: c_ulong = 21_534u64;
```

### `TIOCSSERIAL`
```rust
const TIOCSSERIAL: c_ulong = 21_535u64;
```

### `TIOCPKT`
```rust
const TIOCPKT: c_ulong = 21_536u64;
```

### `FIONBIO`
```rust
const FIONBIO: c_ulong = 21_537u64;
```

### `TIOCNOTTY`
```rust
const TIOCNOTTY: c_ulong = 21_538u64;
```

### `TIOCSETD`
```rust
const TIOCSETD: c_ulong = 21_539u64;
```

### `TIOCGETD`
```rust
const TIOCGETD: c_ulong = 21_540u64;
```

### `TCSBRKP`
```rust
const TCSBRKP: c_ulong = 21_541u64;
```

### `TIOCSBRK`
```rust
const TIOCSBRK: c_ulong = 21_543u64;
```

### `TIOCCBRK`
```rust
const TIOCCBRK: c_ulong = 21_544u64;
```

### `TIOCGSID`
```rust
const TIOCGSID: c_ulong = 21_545u64;
```

### `TCGETS2`
```rust
const TCGETS2: c_ulong = 2_150_388_778u64;
```

### `TCSETS2`
```rust
const TCSETS2: c_ulong = 1_076_646_955u64;
```

### `TCSETSW2`
```rust
const TCSETSW2: c_ulong = 1_076_646_956u64;
```

### `TCSETSF2`
```rust
const TCSETSF2: c_ulong = 1_076_646_957u64;
```

### `TIOCGRS485`
```rust
const TIOCGRS485: c_ulong = 21_550u64;
```

### `TIOCSRS485`
```rust
const TIOCSRS485: c_ulong = 21_551u64;
```

### `TIOCGPTN`
```rust
const TIOCGPTN: c_ulong = 2_147_767_344u64;
```

### `TIOCSPTLCK`
```rust
const TIOCSPTLCK: c_ulong = 1_074_025_521u64;
```

### `TIOCGDEV`
```rust
const TIOCGDEV: c_ulong = 2_147_767_346u64;
```

### `TCGETX`
```rust
const TCGETX: c_ulong = 21_554u64;
```

### `TCSETX`
```rust
const TCSETX: c_ulong = 21_555u64;
```

### `TCSETXF`
```rust
const TCSETXF: c_ulong = 21_556u64;
```

### `TCSETXW`
```rust
const TCSETXW: c_ulong = 21_557u64;
```

### `TIOCSIG`
```rust
const TIOCSIG: c_ulong = 1_074_025_526u64;
```

### `TIOCVHANGUP`
```rust
const TIOCVHANGUP: c_ulong = 21_559u64;
```

### `TIOCGPKT`
```rust
const TIOCGPKT: c_ulong = 2_147_767_352u64;
```

### `TIOCGPTLCK`
```rust
const TIOCGPTLCK: c_ulong = 2_147_767_353u64;
```

### `TIOCGEXCL`
```rust
const TIOCGEXCL: c_ulong = 2_147_767_360u64;
```

### `TIOCGPTPEER`
```rust
const TIOCGPTPEER: c_ulong = 21_569u64;
```

### `FIONCLEX`
```rust
const FIONCLEX: c_ulong = 21_584u64;
```

### `FIOCLEX`
```rust
const FIOCLEX: c_ulong = 21_585u64;
```

### `FIOASYNC`
```rust
const FIOASYNC: c_ulong = 21_586u64;
```

### `TIOCSERCONFIG`
```rust
const TIOCSERCONFIG: c_ulong = 21_587u64;
```

### `TIOCSERGWILD`
```rust
const TIOCSERGWILD: c_ulong = 21_588u64;
```

### `TIOCSERSWILD`
```rust
const TIOCSERSWILD: c_ulong = 21_589u64;
```

### `TIOCGLCKTRMIOS`
```rust
const TIOCGLCKTRMIOS: c_ulong = 21_590u64;
```

### `TIOCSLCKTRMIOS`
```rust
const TIOCSLCKTRMIOS: c_ulong = 21_591u64;
```

### `TIOCSERGSTRUCT`
```rust
const TIOCSERGSTRUCT: c_ulong = 21_592u64;
```

### `TIOCSERGETLSR`
```rust
const TIOCSERGETLSR: c_ulong = 21_593u64;
```

### `TIOCSERGETMULTI`
```rust
const TIOCSERGETMULTI: c_ulong = 21_594u64;
```

### `TIOCSERSETMULTI`
```rust
const TIOCSERSETMULTI: c_ulong = 21_595u64;
```

### `TIOCMIWAIT`
```rust
const TIOCMIWAIT: c_ulong = 21_596u64;
```

### `TIOCGICOUNT`
```rust
const TIOCGICOUNT: c_ulong = 21_597u64;
```

### `BLKIOMIN`
```rust
const BLKIOMIN: c_ulong = 4_728u64;
```

### `BLKIOOPT`
```rust
const BLKIOOPT: c_ulong = 4_729u64;
```

### `BLKSSZGET`
```rust
const BLKSSZGET: c_ulong = 4_712u64;
```

### `BLKPBSZGET`
```rust
const BLKPBSZGET: c_ulong = 4_731u64;
```

### `FIOQSIZE`
```rust
const FIOQSIZE: c_ulong = 21_600u64;
```

### `TIOCM_LE`
```rust
const TIOCM_LE: c_int = 1i32;
```

### `TIOCM_DTR`
```rust
const TIOCM_DTR: c_int = 2i32;
```

### `TIOCM_RTS`
```rust
const TIOCM_RTS: c_int = 4i32;
```

### `TIOCM_ST`
```rust
const TIOCM_ST: c_int = 8i32;
```

### `TIOCM_SR`
```rust
const TIOCM_SR: c_int = 16i32;
```

### `TIOCM_CTS`
```rust
const TIOCM_CTS: c_int = 32i32;
```

### `TIOCM_CAR`
```rust
const TIOCM_CAR: c_int = 64i32;
```

### `TIOCM_CD`
```rust
const TIOCM_CD: c_int = 64i32;
```

### `TIOCM_RNG`
```rust
const TIOCM_RNG: c_int = 128i32;
```

### `TIOCM_RI`
```rust
const TIOCM_RI: c_int = 128i32;
```

### `TIOCM_DSR`
```rust
const TIOCM_DSR: c_int = 256i32;
```

### `BOTHER`
```rust
const BOTHER: crate::speed_t = 4_096u32;
```

### `IBSHIFT`
```rust
const IBSHIFT: crate::tcflag_t = 16u32;
```

### `IUCLC`
```rust
const IUCLC: crate::tcflag_t = 512u32;
```

### `XCASE`
```rust
const XCASE: crate::tcflag_t = 4u32;
```

### `RLIMIT_CPU`
```rust
const RLIMIT_CPU: crate::__rlimit_resource_t = 0u32;
```

### `RLIMIT_FSIZE`
```rust
const RLIMIT_FSIZE: crate::__rlimit_resource_t = 1u32;
```

### `RLIMIT_DATA`
```rust
const RLIMIT_DATA: crate::__rlimit_resource_t = 2u32;
```

### `RLIMIT_STACK`
```rust
const RLIMIT_STACK: crate::__rlimit_resource_t = 3u32;
```

### `RLIMIT_CORE`
```rust
const RLIMIT_CORE: crate::__rlimit_resource_t = 4u32;
```

### `RLIMIT_RSS`
```rust
const RLIMIT_RSS: crate::__rlimit_resource_t = 5u32;
```

### `RLIMIT_NPROC`
```rust
const RLIMIT_NPROC: crate::__rlimit_resource_t = 6u32;
```

### `RLIMIT_NOFILE`
```rust
const RLIMIT_NOFILE: crate::__rlimit_resource_t = 7u32;
```

### `RLIMIT_MEMLOCK`
```rust
const RLIMIT_MEMLOCK: crate::__rlimit_resource_t = 8u32;
```

### `RLIMIT_AS`
```rust
const RLIMIT_AS: crate::__rlimit_resource_t = 9u32;
```

### `RLIMIT_LOCKS`
```rust
const RLIMIT_LOCKS: crate::__rlimit_resource_t = 10u32;
```

### `RLIMIT_SIGPENDING`
```rust
const RLIMIT_SIGPENDING: crate::__rlimit_resource_t = 11u32;
```

### `RLIMIT_MSGQUEUE`
```rust
const RLIMIT_MSGQUEUE: crate::__rlimit_resource_t = 12u32;
```

### `RLIMIT_NICE`
```rust
const RLIMIT_NICE: crate::__rlimit_resource_t = 13u32;
```

### `RLIMIT_RTPRIO`
```rust
const RLIMIT_RTPRIO: crate::__rlimit_resource_t = 14u32;
```

### `RLIMIT_RTTIME`
```rust
const RLIMIT_RTTIME: crate::__rlimit_resource_t = 15u32;
```

### `RLIMIT_NLIMITS`
```rust
const RLIMIT_NLIMITS: crate::__rlimit_resource_t = 16u32;
```

### `RLIM_NLIMITS`
```rust
const RLIM_NLIMITS: crate::__rlimit_resource_t = 16u32;
```

### `RLIM_INFINITY`
```rust
const RLIM_INFINITY: crate::rlim_t = 18_446_744_073_709_551_615u64;
```

