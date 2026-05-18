**libc > unix > linux_like**

# Module: unix::linux_like

## Contents

**Structs**

- [`Dl_info`](#dl_info)
- [`addrinfo`](#addrinfo)
- [`arphdr`](#arphdr)
- [`arpreq`](#arpreq)
- [`arpreq_old`](#arpreq_old)
- [`epoll_event`](#epoll_event)
- [`fd_set`](#fd_set)
- [`file_clone_range`](#file_clone_range)
- [`if_nameindex`](#if_nameindex)
- [`ifaddrs`](#ifaddrs)
- [`in6_rtmsg`](#in6_rtmsg)
- [`in_addr`](#in_addr)
- [`in_pktinfo`](#in_pktinfo)
- [`ip_mreq`](#ip_mreq)
- [`ip_mreq_source`](#ip_mreq_source)
- [`ip_mreqn`](#ip_mreqn)
- [`lconv`](#lconv)
- [`mmsghdr`](#mmsghdr)
- [`sched_param`](#sched_param)
- [`sigevent`](#sigevent)
- [`sock_filter`](#sock_filter)
- [`sock_fprog`](#sock_fprog)
- [`sockaddr`](#sockaddr)
- [`sockaddr_in`](#sockaddr_in)
- [`sockaddr_in6`](#sockaddr_in6)
- [`sockaddr_ll`](#sockaddr_ll)
- [`sockaddr_storage`](#sockaddr_storage)
- [`sockaddr_un`](#sockaddr_un)
- [`statx`](#statx)
- [`statx_timestamp`](#statx_timestamp)
- [`tm`](#tm)
- [`utsname`](#utsname)

**Enums**

- [`timezone`](#timezone)

**Functions**

- [`CMSG_DATA`](#cmsg_data)
- [`CMSG_FIRSTHDR`](#cmsg_firsthdr)
- [`CMSG_LEN`](#cmsg_len)
- [`CMSG_SPACE`](#cmsg_space)
- [`FD_CLR`](#fd_clr)
- [`FD_ISSET`](#fd_isset)
- [`FD_SET`](#fd_set)
- [`FD_ZERO`](#fd_zero)
- [`IPOPT_CLASS`](#ipopt_class)
- [`IPOPT_COPIED`](#ipopt_copied)
- [`IPOPT_NUMBER`](#ipopt_number)
- [`IPTOS_ECN`](#iptos_ecn)
- [`KERNEL_VERSION`](#kernel_version)
- [`QCMD`](#qcmd)
- [`SIGRTMAX`](#sigrtmax)
- [`SIGRTMIN`](#sigrtmin)
- [`WCOREDUMP`](#wcoredump)
- [`WEXITSTATUS`](#wexitstatus)
- [`WIFCONTINUED`](#wifcontinued)
- [`WIFEXITED`](#wifexited)
- [`WIFSIGNALED`](#wifsignaled)
- [`WIFSTOPPED`](#wifstopped)
- [`WSTOPSIG`](#wstopsig)
- [`WTERMSIG`](#wtermsig)
- [`W_EXITCODE`](#w_exitcode)
- [`W_STOPCODE`](#w_stopcode)
- [`_IO`](#_io) - Build an ioctl number for an argumentless ioctl.
- [`_IOR`](#_ior) - Build an ioctl number for an read-only ioctl.
- [`_IOW`](#_iow) - Build an ioctl number for an write-only ioctl.
- [`_IOWR`](#_iowr) - Build an ioctl number for a read-write ioctl.
- [`acct`](#acct)
- [`bind`](#bind)
- [`brk`](#brk)
- [`clearenv`](#clearenv)
- [`clock_getcpuclockid`](#clock_getcpuclockid)
- [`clock_getres`](#clock_getres)
- [`clock_gettime`](#clock_gettime)
- [`clock_settime`](#clock_settime)
- [`creat64`](#creat64)
- [`dirfd`](#dirfd)
- [`duplocale`](#duplocale)
- [`execvpe`](#execvpe)
- [`fdatasync`](#fdatasync)
- [`fexecve`](#fexecve)
- [`forkpty`](#forkpty)
- [`freeifaddrs`](#freeifaddrs)
- [`freelocale`](#freelocale)
- [`fstat64`](#fstat64)
- [`fstatat64`](#fstatat64)
- [`fstatfs`](#fstatfs)
- [`fstatfs64`](#fstatfs64)
- [`fstatvfs64`](#fstatvfs64)
- [`ftruncate64`](#ftruncate64)
- [`futimens`](#futimens)
- [`getdomainname`](#getdomainname)
- [`getifaddrs`](#getifaddrs)
- [`getitimer`](#getitimer)
- [`getpwnam_r`](#getpwnam_r)
- [`getpwuid_r`](#getpwuid_r)
- [`getresgid`](#getresgid)
- [`getresuid`](#getresuid)
- [`if_freenameindex`](#if_freenameindex)
- [`if_nameindex`](#if_nameindex)
- [`ioctl`](#ioctl)
- [`login_tty`](#login_tty)
- [`lseek64`](#lseek64)
- [`lstat64`](#lstat64)
- [`memalign`](#memalign)
- [`memrchr`](#memrchr)
- [`mincore`](#mincore)
- [`mknodat`](#mknodat)
- [`mkostemp`](#mkostemp)
- [`mkostemps`](#mkostemps)
- [`mmap64`](#mmap64)
- [`newlocale`](#newlocale)
- [`open64`](#open64)
- [`openat64`](#openat64)
- [`openpty`](#openpty)
- [`pipe2`](#pipe2)
- [`posix_fadvise`](#posix_fadvise)
- [`posix_fadvise64`](#posix_fadvise64)
- [`pread64`](#pread64)
- [`preadv64`](#preadv64)
- [`ptsname_r`](#ptsname_r)
- [`pwrite64`](#pwrite64)
- [`pwritev64`](#pwritev64)
- [`readdir64`](#readdir64)
- [`readdir64_r`](#readdir64_r)
- [`readv`](#readv)
- [`recvmsg`](#recvmsg)
- [`sbrk`](#sbrk)
- [`sem_destroy`](#sem_destroy)
- [`sem_init`](#sem_init)
- [`sendmsg`](#sendmsg)
- [`setdomainname`](#setdomainname)
- [`setgroups`](#setgroups)
- [`setitimer`](#setitimer)
- [`setresgid`](#setresgid)
- [`setresuid`](#setresuid)
- [`stat64`](#stat64)
- [`statfs`](#statfs)
- [`statfs64`](#statfs64)
- [`statvfs64`](#statvfs64)
- [`statx`](#statx)
- [`strchrnul`](#strchrnul)
- [`strftime`](#strftime)
- [`strftime_l`](#strftime_l)
- [`strptime`](#strptime)
- [`truncate64`](#truncate64)
- [`uname`](#uname)
- [`uselocale`](#uselocale)
- [`utimensat`](#utimensat)
- [`vfork`](#vfork)
- [`wait4`](#wait4)
- [`waitid`](#waitid)
- [`writev`](#writev)

**Constants**

- [`ADDR_COMPAT_LAYOUT`](#addr_compat_layout)
- [`ADDR_LIMIT_32BIT`](#addr_limit_32bit)
- [`ADDR_LIMIT_3GB`](#addr_limit_3gb)
- [`ADDR_NO_RANDOMIZE`](#addr_no_randomize)
- [`ADFS_SUPER_MAGIC`](#adfs_super_magic)
- [`AFFS_SUPER_MAGIC`](#affs_super_magic)
- [`AFS_SUPER_MAGIC`](#afs_super_magic)
- [`AF_ALG`](#af_alg)
- [`AF_APPLETALK`](#af_appletalk)
- [`AF_ASH`](#af_ash)
- [`AF_ATMPVC`](#af_atmpvc)
- [`AF_ATMSVC`](#af_atmsvc)
- [`AF_AX25`](#af_ax25)
- [`AF_BLUETOOTH`](#af_bluetooth)
- [`AF_BRIDGE`](#af_bridge)
- [`AF_CAIF`](#af_caif)
- [`AF_CAN`](#af_can)
- [`AF_DECnet`](#af_decnet)
- [`AF_ECONET`](#af_econet)
- [`AF_IEEE802154`](#af_ieee802154)
- [`AF_INET`](#af_inet)
- [`AF_INET6`](#af_inet6)
- [`AF_IPX`](#af_ipx)
- [`AF_IRDA`](#af_irda)
- [`AF_ISDN`](#af_isdn)
- [`AF_IUCV`](#af_iucv)
- [`AF_KEY`](#af_key)
- [`AF_LLC`](#af_llc)
- [`AF_LOCAL`](#af_local)
- [`AF_NETBEUI`](#af_netbeui)
- [`AF_NETLINK`](#af_netlink)
- [`AF_NETROM`](#af_netrom)
- [`AF_PACKET`](#af_packet)
- [`AF_PHONET`](#af_phonet)
- [`AF_PPPOX`](#af_pppox)
- [`AF_RDS`](#af_rds)
- [`AF_ROSE`](#af_rose)
- [`AF_ROUTE`](#af_route)
- [`AF_RXRPC`](#af_rxrpc)
- [`AF_SECURITY`](#af_security)
- [`AF_SNA`](#af_sna)
- [`AF_TIPC`](#af_tipc)
- [`AF_UNIX`](#af_unix)
- [`AF_UNSPEC`](#af_unspec)
- [`AF_WANPIPE`](#af_wanpipe)
- [`AF_X25`](#af_x25)
- [`ARPHRD_ADAPT`](#arphrd_adapt)
- [`ARPHRD_APPLETLK`](#arphrd_appletlk)
- [`ARPHRD_ARCNET`](#arphrd_arcnet)
- [`ARPHRD_ASH`](#arphrd_ash)
- [`ARPHRD_ATM`](#arphrd_atm)
- [`ARPHRD_AX25`](#arphrd_ax25)
- [`ARPHRD_BIF`](#arphrd_bif)
- [`ARPHRD_CAN`](#arphrd_can)
- [`ARPHRD_CHAOS`](#arphrd_chaos)
- [`ARPHRD_CISCO`](#arphrd_cisco)
- [`ARPHRD_CSLIP`](#arphrd_cslip)
- [`ARPHRD_CSLIP6`](#arphrd_cslip6)
- [`ARPHRD_DDCMP`](#arphrd_ddcmp)
- [`ARPHRD_DLCI`](#arphrd_dlci)
- [`ARPHRD_ECONET`](#arphrd_econet)
- [`ARPHRD_EETHER`](#arphrd_eether)
- [`ARPHRD_ETHER`](#arphrd_ether)
- [`ARPHRD_EUI64`](#arphrd_eui64)
- [`ARPHRD_FCAL`](#arphrd_fcal)
- [`ARPHRD_FCFABRIC`](#arphrd_fcfabric)
- [`ARPHRD_FCPL`](#arphrd_fcpl)
- [`ARPHRD_FCPP`](#arphrd_fcpp)
- [`ARPHRD_FDDI`](#arphrd_fddi)
- [`ARPHRD_FRAD`](#arphrd_frad)
- [`ARPHRD_HDLC`](#arphrd_hdlc)
- [`ARPHRD_HIPPI`](#arphrd_hippi)
- [`ARPHRD_HWX25`](#arphrd_hwx25)
- [`ARPHRD_IEEE1394`](#arphrd_ieee1394)
- [`ARPHRD_IEEE802`](#arphrd_ieee802)
- [`ARPHRD_IEEE80211`](#arphrd_ieee80211)
- [`ARPHRD_IEEE80211_PRISM`](#arphrd_ieee80211_prism)
- [`ARPHRD_IEEE80211_RADIOTAP`](#arphrd_ieee80211_radiotap)
- [`ARPHRD_IEEE802154`](#arphrd_ieee802154)
- [`ARPHRD_IEEE802_TR`](#arphrd_ieee802_tr)
- [`ARPHRD_INFINIBAND`](#arphrd_infiniband)
- [`ARPHRD_IPDDP`](#arphrd_ipddp)
- [`ARPHRD_IPGRE`](#arphrd_ipgre)
- [`ARPHRD_IRDA`](#arphrd_irda)
- [`ARPHRD_LAPB`](#arphrd_lapb)
- [`ARPHRD_LOCALTLK`](#arphrd_localtlk)
- [`ARPHRD_LOOPBACK`](#arphrd_loopback)
- [`ARPHRD_METRICOM`](#arphrd_metricom)
- [`ARPHRD_NETROM`](#arphrd_netrom)
- [`ARPHRD_NONE`](#arphrd_none)
- [`ARPHRD_PIMREG`](#arphrd_pimreg)
- [`ARPHRD_PPP`](#arphrd_ppp)
- [`ARPHRD_PRONET`](#arphrd_pronet)
- [`ARPHRD_RAWHDLC`](#arphrd_rawhdlc)
- [`ARPHRD_ROSE`](#arphrd_rose)
- [`ARPHRD_RSRVD`](#arphrd_rsrvd)
- [`ARPHRD_SIT`](#arphrd_sit)
- [`ARPHRD_SKIP`](#arphrd_skip)
- [`ARPHRD_SLIP`](#arphrd_slip)
- [`ARPHRD_SLIP6`](#arphrd_slip6)
- [`ARPHRD_TUNNEL`](#arphrd_tunnel)
- [`ARPHRD_TUNNEL6`](#arphrd_tunnel6)
- [`ARPHRD_VOID`](#arphrd_void)
- [`ARPHRD_X25`](#arphrd_x25)
- [`ARPOP_InREPLY`](#arpop_inreply)
- [`ARPOP_InREQUEST`](#arpop_inrequest)
- [`ARPOP_NAK`](#arpop_nak)
- [`ARPOP_RREPLY`](#arpop_rreply)
- [`ARPOP_RREQUEST`](#arpop_rrequest)
- [`ATF_DONTPUB`](#atf_dontpub)
- [`ATF_NETMASK`](#atf_netmask)
- [`AT_EMPTY_PATH`](#at_empty_path)
- [`AT_FDCWD`](#at_fdcwd)
- [`AT_NO_AUTOMOUNT`](#at_no_automount)
- [`AT_RECURSIVE`](#at_recursive)
- [`AT_REMOVEDIR`](#at_removedir)
- [`AT_STATX_DONT_SYNC`](#at_statx_dont_sync)
- [`AT_STATX_FORCE_SYNC`](#at_statx_force_sync)
- [`AT_STATX_SYNC_AS_STAT`](#at_statx_sync_as_stat)
- [`AT_STATX_SYNC_TYPE`](#at_statx_sync_type)
- [`AT_SYMLINK_FOLLOW`](#at_symlink_follow)
- [`AT_SYMLINK_NOFOLLOW`](#at_symlink_nofollow)
- [`AUTOFS_SUPER_MAGIC`](#autofs_super_magic)
- [`BPF_FS_MAGIC`](#bpf_fs_magic)
- [`BRKINT`](#brkint)
- [`BS0`](#bs0)
- [`BTRFS_SUPER_MAGIC`](#btrfs_super_magic)
- [`BUS_ADRALN`](#bus_adraln)
- [`BUS_ADRERR`](#bus_adrerr)
- [`BUS_MCEERR_AO`](#bus_mceerr_ao)
- [`BUS_MCEERR_AR`](#bus_mceerr_ar)
- [`BUS_OBJERR`](#bus_objerr)
- [`CGROUP2_SUPER_MAGIC`](#cgroup2_super_magic)
- [`CGROUP_SUPER_MAGIC`](#cgroup_super_magic)
- [`CLD_CONTINUED`](#cld_continued)
- [`CLD_DUMPED`](#cld_dumped)
- [`CLD_EXITED`](#cld_exited)
- [`CLD_KILLED`](#cld_killed)
- [`CLD_STOPPED`](#cld_stopped)
- [`CLD_TRAPPED`](#cld_trapped)
- [`CLOCK_BOOTTIME`](#clock_boottime)
- [`CLOCK_BOOTTIME_ALARM`](#clock_boottime_alarm)
- [`CLOCK_MONOTONIC`](#clock_monotonic)
- [`CLOCK_MONOTONIC_COARSE`](#clock_monotonic_coarse)
- [`CLOCK_MONOTONIC_RAW`](#clock_monotonic_raw)
- [`CLOCK_PROCESS_CPUTIME_ID`](#clock_process_cputime_id)
- [`CLOCK_REALTIME`](#clock_realtime)
- [`CLOCK_REALTIME_ALARM`](#clock_realtime_alarm)
- [`CLOCK_REALTIME_COARSE`](#clock_realtime_coarse)
- [`CLOCK_TAI`](#clock_tai)
- [`CLOCK_THREAD_CPUTIME_ID`](#clock_thread_cputime_id)
- [`CLONE_CHILD_CLEARTID`](#clone_child_cleartid)
- [`CLONE_CHILD_SETTID`](#clone_child_settid)
- [`CLONE_DETACHED`](#clone_detached)
- [`CLONE_FILES`](#clone_files)
- [`CLONE_FS`](#clone_fs)
- [`CLONE_IO`](#clone_io)
- [`CLONE_NEWCGROUP`](#clone_newcgroup)
- [`CLONE_NEWIPC`](#clone_newipc)
- [`CLONE_NEWNET`](#clone_newnet)
- [`CLONE_NEWNS`](#clone_newns)
- [`CLONE_NEWPID`](#clone_newpid)
- [`CLONE_NEWUSER`](#clone_newuser)
- [`CLONE_NEWUTS`](#clone_newuts)
- [`CLONE_PARENT`](#clone_parent)
- [`CLONE_PARENT_SETTID`](#clone_parent_settid)
- [`CLONE_PTRACE`](#clone_ptrace)
- [`CLONE_SETTLS`](#clone_settls)
- [`CLONE_SIGHAND`](#clone_sighand)
- [`CLONE_SYSVSEM`](#clone_sysvsem)
- [`CLONE_THREAD`](#clone_thread)
- [`CLONE_UNTRACED`](#clone_untraced)
- [`CLONE_VFORK`](#clone_vfork)
- [`CLONE_VM`](#clone_vm)
- [`CODA_SUPER_MAGIC`](#coda_super_magic)
- [`CR0`](#cr0)
- [`CRAMFS_MAGIC`](#cramfs_magic)
- [`CRTSCTS`](#crtscts)
- [`CS5`](#cs5)
- [`DEBUGFS_MAGIC`](#debugfs_magic)
- [`DEVPTS_SUPER_MAGIC`](#devpts_super_magic)
- [`ECHO`](#echo)
- [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs_super_magic)
- [`EFS_SUPER_MAGIC`](#efs_super_magic)
- [`EOF`](#eof)
- [`EPOLLERR`](#epollerr)
- [`EPOLLET`](#epollet)
- [`EPOLLEXCLUSIVE`](#epollexclusive)
- [`EPOLLHUP`](#epollhup)
- [`EPOLLIN`](#epollin)
- [`EPOLLMSG`](#epollmsg)
- [`EPOLLONESHOT`](#epolloneshot)
- [`EPOLLOUT`](#epollout)
- [`EPOLLPRI`](#epollpri)
- [`EPOLLRDBAND`](#epollrdband)
- [`EPOLLRDHUP`](#epollrdhup)
- [`EPOLLRDNORM`](#epollrdnorm)
- [`EPOLLWAKEUP`](#epollwakeup)
- [`EPOLLWRBAND`](#epollwrband)
- [`EPOLLWRNORM`](#epollwrnorm)
- [`EPOLL_CTL_ADD`](#epoll_ctl_add)
- [`EPOLL_CTL_DEL`](#epoll_ctl_del)
- [`EPOLL_CTL_MOD`](#epoll_ctl_mod)
- [`EXIT_FAILURE`](#exit_failure)
- [`EXIT_SUCCESS`](#exit_success)
- [`EXT2_SUPER_MAGIC`](#ext2_super_magic)
- [`EXT3_SUPER_MAGIC`](#ext3_super_magic)
- [`EXT4_SUPER_MAGIC`](#ext4_super_magic)
- [`F2FS_SUPER_MAGIC`](#f2fs_super_magic)
- [`FD_SETSIZE`](#fd_setsize)
- [`FF0`](#ff0)
- [`FICLONE`](#ficlone)
- [`FICLONERANGE`](#ficlonerange)
- [`FS_IOC32_GETFLAGS`](#fs_ioc32_getflags)
- [`FS_IOC32_GETVERSION`](#fs_ioc32_getversion)
- [`FS_IOC32_SETFLAGS`](#fs_ioc32_setflags)
- [`FS_IOC32_SETVERSION`](#fs_ioc32_setversion)
- [`FS_IOC_GETFLAGS`](#fs_ioc_getflags)
- [`FS_IOC_GETVERSION`](#fs_ioc_getversion)
- [`FS_IOC_SETFLAGS`](#fs_ioc_setflags)
- [`FS_IOC_SETVERSION`](#fs_ioc_setversion)
- [`FUSE_SUPER_MAGIC`](#fuse_super_magic)
- [`FUTEXFS_SUPER_MAGIC`](#futexfs_super_magic)
- [`F_ADD_SEALS`](#f_add_seals)
- [`F_CANCELLK`](#f_cancellk)
- [`F_DUPFD`](#f_dupfd)
- [`F_DUPFD_CLOEXEC`](#f_dupfd_cloexec)
- [`F_GETFD`](#f_getfd)
- [`F_GETFL`](#f_getfl)
- [`F_GETLEASE`](#f_getlease)
- [`F_GETPIPE_SZ`](#f_getpipe_sz)
- [`F_GET_SEALS`](#f_get_seals)
- [`F_NOTIFY`](#f_notify)
- [`F_OK`](#f_ok)
- [`F_SEAL_GROW`](#f_seal_grow)
- [`F_SEAL_SEAL`](#f_seal_seal)
- [`F_SEAL_SHRINK`](#f_seal_shrink)
- [`F_SEAL_WRITE`](#f_seal_write)
- [`F_SETFD`](#f_setfd)
- [`F_SETFL`](#f_setfl)
- [`F_SETLEASE`](#f_setlease)
- [`F_SETPIPE_SZ`](#f_setpipe_sz)
- [`HOSTFS_SUPER_MAGIC`](#hostfs_super_magic)
- [`HPFS_SUPER_MAGIC`](#hpfs_super_magic)
- [`HUGETLBFS_MAGIC`](#hugetlbfs_magic)
- [`ICRNL`](#icrnl)
- [`IFF_ALLMULTI`](#iff_allmulti)
- [`IFF_ATTACH_QUEUE`](#iff_attach_queue)
- [`IFF_AUTOMEDIA`](#iff_automedia)
- [`IFF_BROADCAST`](#iff_broadcast)
- [`IFF_DEBUG`](#iff_debug)
- [`IFF_DETACH_QUEUE`](#iff_detach_queue)
- [`IFF_DYNAMIC`](#iff_dynamic)
- [`IFF_LOOPBACK`](#iff_loopback)
- [`IFF_MASTER`](#iff_master)
- [`IFF_MULTICAST`](#iff_multicast)
- [`IFF_MULTI_QUEUE`](#iff_multi_queue)
- [`IFF_NAPI`](#iff_napi)
- [`IFF_NAPI_FRAGS`](#iff_napi_frags)
- [`IFF_NOARP`](#iff_noarp)
- [`IFF_NOFILTER`](#iff_nofilter)
- [`IFF_NOTRAILERS`](#iff_notrailers)
- [`IFF_NO_CARRIER`](#iff_no_carrier)
- [`IFF_NO_PI`](#iff_no_pi)
- [`IFF_ONE_QUEUE`](#iff_one_queue)
- [`IFF_PERSIST`](#iff_persist)
- [`IFF_POINTOPOINT`](#iff_pointopoint)
- [`IFF_PORTSEL`](#iff_portsel)
- [`IFF_PROMISC`](#iff_promisc)
- [`IFF_RUNNING`](#iff_running)
- [`IFF_SLAVE`](#iff_slave)
- [`IFF_TAP`](#iff_tap)
- [`IFF_TUN`](#iff_tun)
- [`IFF_TUN_EXCL`](#iff_tun_excl)
- [`IFF_UP`](#iff_up)
- [`IFF_VNET_HDR`](#iff_vnet_hdr)
- [`IGNBRK`](#ignbrk)
- [`IGNCR`](#igncr)
- [`IGNPAR`](#ignpar)
- [`IMAXBEL`](#imaxbel)
- [`INLCR`](#inlcr)
- [`INPCK`](#inpck)
- [`IPDEFTTL`](#ipdefttl)
- [`IPOPT_CLASS_MASK`](#ipopt_class_mask)
- [`IPOPT_CONTROL`](#ipopt_control)
- [`IPOPT_COPY`](#ipopt_copy)
- [`IPOPT_END`](#ipopt_end)
- [`IPOPT_EOL`](#ipopt_eol)
- [`IPOPT_LSRR`](#ipopt_lsrr)
- [`IPOPT_MEASUREMENT`](#ipopt_measurement)
- [`IPOPT_MINOFF`](#ipopt_minoff)
- [`IPOPT_NOOP`](#ipopt_noop)
- [`IPOPT_NOP`](#ipopt_nop)
- [`IPOPT_NUMBER_MASK`](#ipopt_number_mask)
- [`IPOPT_OFFSET`](#ipopt_offset)
- [`IPOPT_OLEN`](#ipopt_olen)
- [`IPOPT_OPTVAL`](#ipopt_optval)
- [`IPOPT_RA`](#ipopt_ra)
- [`IPOPT_RESERVED1`](#ipopt_reserved1)
- [`IPOPT_RESERVED2`](#ipopt_reserved2)
- [`IPOPT_RR`](#ipopt_rr)
- [`IPOPT_SEC`](#ipopt_sec)
- [`IPOPT_SID`](#ipopt_sid)
- [`IPOPT_SSRR`](#ipopt_ssrr)
- [`IPOPT_TIMESTAMP`](#ipopt_timestamp)
- [`IPOPT_TS`](#ipopt_ts)
- [`IPOPT_TS_PRESPEC`](#ipopt_ts_prespec)
- [`IPOPT_TS_TSANDADDR`](#ipopt_ts_tsandaddr)
- [`IPOPT_TS_TSONLY`](#ipopt_ts_tsonly)
- [`IPPROTO_AH`](#ipproto_ah) - IP6 Auth Header
- [`IPPROTO_BEETPH`](#ipproto_beetph)
- [`IPPROTO_COMP`](#ipproto_comp) - IP Payload Comp. Protocol
- [`IPPROTO_DCCP`](#ipproto_dccp) - DCCP
- [`IPPROTO_DSTOPTS`](#ipproto_dstopts) - IP6 destination option
- [`IPPROTO_EGP`](#ipproto_egp) - exterior gateway protocol
- [`IPPROTO_ENCAP`](#ipproto_encap) - encapsulation header
- [`IPPROTO_ESP`](#ipproto_esp) - IP6 Encap Sec. Payload
- [`IPPROTO_ETHERNET`](#ipproto_ethernet) - Ethernet-within-IPv6 encapsulation.
- [`IPPROTO_FRAGMENT`](#ipproto_fragment) - IP6 fragmentation header
- [`IPPROTO_GRE`](#ipproto_gre) - General Routing Encap.
- [`IPPROTO_HOPOPTS`](#ipproto_hopopts) - Hop-by-hop option header
- [`IPPROTO_IDP`](#ipproto_idp) - xns idp
- [`IPPROTO_IGMP`](#ipproto_igmp) - group mgmt protocol
- [`IPPROTO_IPIP`](#ipproto_ipip) - for compatibility
- [`IPPROTO_MH`](#ipproto_mh)
- [`IPPROTO_MPLS`](#ipproto_mpls)
- [`IPPROTO_MPTCP`](#ipproto_mptcp) - Multipath TCP
- [`IPPROTO_MTP`](#ipproto_mtp)
- [`IPPROTO_NONE`](#ipproto_none) - IP6 no next header
- [`IPPROTO_PIM`](#ipproto_pim) - Protocol indep. multicast
- [`IPPROTO_PUP`](#ipproto_pup) - pup
- [`IPPROTO_RAW`](#ipproto_raw) - raw IP packet
- [`IPPROTO_ROUTING`](#ipproto_routing) - IP6 routing header
- [`IPPROTO_RSVP`](#ipproto_rsvp) - resource reservation
- [`IPPROTO_SCTP`](#ipproto_sctp) - SCTP
- [`IPPROTO_TP`](#ipproto_tp) - tp-4 w/ class negotiation
- [`IPPROTO_UDPLITE`](#ipproto_udplite)
- [`IPTOS_ECN_CE`](#iptos_ecn_ce)
- [`IPTOS_ECN_ECT0`](#iptos_ecn_ect0)
- [`IPTOS_ECN_ECT1`](#iptos_ecn_ect1)
- [`IPTOS_ECN_MASK`](#iptos_ecn_mask)
- [`IPTOS_LOWDELAY`](#iptos_lowdelay)
- [`IPTOS_MINCOST`](#iptos_mincost)
- [`IPTOS_PREC_CRITIC_ECP`](#iptos_prec_critic_ecp)
- [`IPTOS_PREC_FLASH`](#iptos_prec_flash)
- [`IPTOS_PREC_FLASHOVERRIDE`](#iptos_prec_flashoverride)
- [`IPTOS_PREC_IMMEDIATE`](#iptos_prec_immediate)
- [`IPTOS_PREC_INTERNETCONTROL`](#iptos_prec_internetcontrol)
- [`IPTOS_PREC_NETCONTROL`](#iptos_prec_netcontrol)
- [`IPTOS_PREC_PRIORITY`](#iptos_prec_priority)
- [`IPTOS_PREC_ROUTINE`](#iptos_prec_routine)
- [`IPTOS_RELIABILITY`](#iptos_reliability)
- [`IPTOS_THROUGHPUT`](#iptos_throughput)
- [`IPV6_2292DSTOPTS`](#ipv6_2292dstopts)
- [`IPV6_2292HOPLIMIT`](#ipv6_2292hoplimit)
- [`IPV6_2292HOPOPTS`](#ipv6_2292hopopts)
- [`IPV6_2292PKTINFO`](#ipv6_2292pktinfo)
- [`IPV6_2292PKTOPTIONS`](#ipv6_2292pktoptions)
- [`IPV6_2292RTHDR`](#ipv6_2292rthdr)
- [`IPV6_ADDRFORM`](#ipv6_addrform)
- [`IPV6_ADDR_PREFERENCES`](#ipv6_addr_preferences)
- [`IPV6_ADD_MEMBERSHIP`](#ipv6_add_membership)
- [`IPV6_AUTHHDR`](#ipv6_authhdr)
- [`IPV6_AUTOFLOWLABEL`](#ipv6_autoflowlabel)
- [`IPV6_CHECKSUM`](#ipv6_checksum)
- [`IPV6_DONTFRAG`](#ipv6_dontfrag)
- [`IPV6_DROP_MEMBERSHIP`](#ipv6_drop_membership)
- [`IPV6_DSTOPTS`](#ipv6_dstopts)
- [`IPV6_HDRINCL`](#ipv6_hdrincl)
- [`IPV6_HOPLIMIT`](#ipv6_hoplimit)
- [`IPV6_HOPOPTS`](#ipv6_hopopts)
- [`IPV6_IPSEC_POLICY`](#ipv6_ipsec_policy)
- [`IPV6_JOIN_ANYCAST`](#ipv6_join_anycast)
- [`IPV6_LEAVE_ANYCAST`](#ipv6_leave_anycast)
- [`IPV6_MINHOPCOUNT`](#ipv6_minhopcount)
- [`IPV6_MTU`](#ipv6_mtu)
- [`IPV6_MTU_DISCOVER`](#ipv6_mtu_discover)
- [`IPV6_MULTICAST_HOPS`](#ipv6_multicast_hops)
- [`IPV6_MULTICAST_IF`](#ipv6_multicast_if)
- [`IPV6_MULTICAST_LOOP`](#ipv6_multicast_loop)
- [`IPV6_NEXTHOP`](#ipv6_nexthop)
- [`IPV6_ORIGDSTADDR`](#ipv6_origdstaddr)
- [`IPV6_PATHMTU`](#ipv6_pathmtu)
- [`IPV6_PKTINFO`](#ipv6_pktinfo)
- [`IPV6_PMTUDISC_DO`](#ipv6_pmtudisc_do)
- [`IPV6_PMTUDISC_DONT`](#ipv6_pmtudisc_dont)
- [`IPV6_PMTUDISC_INTERFACE`](#ipv6_pmtudisc_interface)
- [`IPV6_PMTUDISC_OMIT`](#ipv6_pmtudisc_omit)
- [`IPV6_PMTUDISC_PROBE`](#ipv6_pmtudisc_probe)
- [`IPV6_PMTUDISC_WANT`](#ipv6_pmtudisc_want)
- [`IPV6_PREFER_SRC_CGA`](#ipv6_prefer_src_cga)
- [`IPV6_PREFER_SRC_COA`](#ipv6_prefer_src_coa)
- [`IPV6_PREFER_SRC_HOME`](#ipv6_prefer_src_home)
- [`IPV6_PREFER_SRC_NONCGA`](#ipv6_prefer_src_noncga)
- [`IPV6_PREFER_SRC_PUBLIC`](#ipv6_prefer_src_public)
- [`IPV6_PREFER_SRC_PUBTMP_DEFAULT`](#ipv6_prefer_src_pubtmp_default)
- [`IPV6_PREFER_SRC_TMP`](#ipv6_prefer_src_tmp)
- [`IPV6_RECVDSTOPTS`](#ipv6_recvdstopts)
- [`IPV6_RECVERR`](#ipv6_recverr)
- [`IPV6_RECVHOPLIMIT`](#ipv6_recvhoplimit)
- [`IPV6_RECVHOPOPTS`](#ipv6_recvhopopts)
- [`IPV6_RECVORIGDSTADDR`](#ipv6_recvorigdstaddr)
- [`IPV6_RECVPATHMTU`](#ipv6_recvpathmtu)
- [`IPV6_RECVPKTINFO`](#ipv6_recvpktinfo)
- [`IPV6_RECVRTHDR`](#ipv6_recvrthdr)
- [`IPV6_RECVTCLASS`](#ipv6_recvtclass)
- [`IPV6_ROUTER_ALERT`](#ipv6_router_alert)
- [`IPV6_RTHDR`](#ipv6_rthdr)
- [`IPV6_RTHDRDSTOPTS`](#ipv6_rthdrdstopts)
- [`IPV6_TCLASS`](#ipv6_tclass)
- [`IPV6_TRANSPARENT`](#ipv6_transparent)
- [`IPV6_UNICAST_HOPS`](#ipv6_unicast_hops)
- [`IPV6_UNICAST_IF`](#ipv6_unicast_if)
- [`IPV6_V6ONLY`](#ipv6_v6only)
- [`IPV6_XFRM_POLICY`](#ipv6_xfrm_policy)
- [`IPVERSION`](#ipversion)
- [`IP_ADD_MEMBERSHIP`](#ip_add_membership)
- [`IP_ADD_SOURCE_MEMBERSHIP`](#ip_add_source_membership)
- [`IP_BIND_ADDRESS_NO_PORT`](#ip_bind_address_no_port)
- [`IP_BLOCK_SOURCE`](#ip_block_source)
- [`IP_CHECKSUM`](#ip_checksum)
- [`IP_DEFAULT_MULTICAST_LOOP`](#ip_default_multicast_loop)
- [`IP_DEFAULT_MULTICAST_TTL`](#ip_default_multicast_ttl)
- [`IP_DROP_MEMBERSHIP`](#ip_drop_membership)
- [`IP_DROP_SOURCE_MEMBERSHIP`](#ip_drop_source_membership)
- [`IP_FREEBIND`](#ip_freebind)
- [`IP_HDRINCL`](#ip_hdrincl)
- [`IP_IPSEC_POLICY`](#ip_ipsec_policy)
- [`IP_MINTTL`](#ip_minttl)
- [`IP_MSFILTER`](#ip_msfilter)
- [`IP_MTU`](#ip_mtu)
- [`IP_MTU_DISCOVER`](#ip_mtu_discover)
- [`IP_MULTICAST_ALL`](#ip_multicast_all)
- [`IP_MULTICAST_IF`](#ip_multicast_if)
- [`IP_MULTICAST_LOOP`](#ip_multicast_loop)
- [`IP_MULTICAST_TTL`](#ip_multicast_ttl)
- [`IP_NODEFRAG`](#ip_nodefrag)
- [`IP_OPTIONS`](#ip_options)
- [`IP_ORIGDSTADDR`](#ip_origdstaddr)
- [`IP_PASSSEC`](#ip_passsec)
- [`IP_PKTINFO`](#ip_pktinfo)
- [`IP_PKTOPTIONS`](#ip_pktoptions)
- [`IP_PMTUDISC_DO`](#ip_pmtudisc_do)
- [`IP_PMTUDISC_DONT`](#ip_pmtudisc_dont)
- [`IP_PMTUDISC_INTERFACE`](#ip_pmtudisc_interface)
- [`IP_PMTUDISC_OMIT`](#ip_pmtudisc_omit)
- [`IP_PMTUDISC_PROBE`](#ip_pmtudisc_probe)
- [`IP_PMTUDISC_WANT`](#ip_pmtudisc_want)
- [`IP_RECVERR`](#ip_recverr)
- [`IP_RECVOPTS`](#ip_recvopts)
- [`IP_RECVORIGDSTADDR`](#ip_recvorigdstaddr)
- [`IP_RECVTOS`](#ip_recvtos)
- [`IP_RECVTTL`](#ip_recvttl)
- [`IP_RETOPTS`](#ip_retopts)
- [`IP_ROUTER_ALERT`](#ip_router_alert)
- [`IP_TOS`](#ip_tos)
- [`IP_TRANSPARENT`](#ip_transparent)
- [`IP_TTL`](#ip_ttl)
- [`IP_UNBLOCK_SOURCE`](#ip_unblock_source)
- [`IP_UNICAST_IF`](#ip_unicast_if)
- [`IP_XFRM_POLICY`](#ip_xfrm_policy)
- [`ISOFS_SUPER_MAGIC`](#isofs_super_magic)
- [`ISTRIP`](#istrip)
- [`IXANY`](#ixany)
- [`JFFS2_SUPER_MAGIC`](#jffs2_super_magic)
- [`LC_ALL`](#lc_all)
- [`LC_COLLATE`](#lc_collate)
- [`LC_COLLATE_MASK`](#lc_collate_mask)
- [`LC_CTYPE`](#lc_ctype)
- [`LC_CTYPE_MASK`](#lc_ctype_mask)
- [`LC_MESSAGES`](#lc_messages)
- [`LC_MESSAGES_MASK`](#lc_messages_mask)
- [`LC_MONETARY`](#lc_monetary)
- [`LC_MONETARY_MASK`](#lc_monetary_mask)
- [`LC_NUMERIC`](#lc_numeric)
- [`LC_NUMERIC_MASK`](#lc_numeric_mask)
- [`LC_TIME`](#lc_time)
- [`LC_TIME_MASK`](#lc_time_mask)
- [`LOCK_EX`](#lock_ex)
- [`LOCK_NB`](#lock_nb)
- [`LOCK_SH`](#lock_sh)
- [`LOCK_UN`](#lock_un)
- [`LOG_AUTHPRIV`](#log_authpriv)
- [`LOG_CRON`](#log_cron)
- [`LOG_FTP`](#log_ftp)
- [`LOG_PERROR`](#log_perror)
- [`MADV_COLD`](#madv_cold)
- [`MADV_DODUMP`](#madv_dodump)
- [`MADV_DOFORK`](#madv_dofork)
- [`MADV_DONTDUMP`](#madv_dontdump)
- [`MADV_DONTFORK`](#madv_dontfork)
- [`MADV_DONTNEED`](#madv_dontneed)
- [`MADV_DONTNEED_LOCKED`](#madv_dontneed_locked)
- [`MADV_FREE`](#madv_free)
- [`MADV_HUGEPAGE`](#madv_hugepage)
- [`MADV_HWPOISON`](#madv_hwpoison)
- [`MADV_KEEPONFORK`](#madv_keeponfork)
- [`MADV_MERGEABLE`](#madv_mergeable)
- [`MADV_NOHUGEPAGE`](#madv_nohugepage)
- [`MADV_NORMAL`](#madv_normal)
- [`MADV_PAGEOUT`](#madv_pageout)
- [`MADV_POPULATE_READ`](#madv_populate_read)
- [`MADV_POPULATE_WRITE`](#madv_populate_write)
- [`MADV_RANDOM`](#madv_random)
- [`MADV_REMOVE`](#madv_remove)
- [`MADV_SEQUENTIAL`](#madv_sequential)
- [`MADV_UNMERGEABLE`](#madv_unmergeable)
- [`MADV_WILLNEED`](#madv_willneed)
- [`MADV_WIPEONFORK`](#madv_wipeonfork)
- [`MAP_FAILED`](#map_failed)
- [`MAP_FILE`](#map_file)
- [`MAP_FIXED`](#map_fixed)
- [`MAP_PRIVATE`](#map_private)
- [`MAP_SHARED`](#map_shared)
- [`MAP_TYPE`](#map_type)
- [`MAXTTL`](#maxttl)
- [`MAX_IPOPTLEN`](#max_ipoptlen)
- [`MCAST_BLOCK_SOURCE`](#mcast_block_source)
- [`MCAST_EXCLUDE`](#mcast_exclude)
- [`MCAST_INCLUDE`](#mcast_include)
- [`MCAST_JOIN_GROUP`](#mcast_join_group)
- [`MCAST_JOIN_SOURCE_GROUP`](#mcast_join_source_group)
- [`MCAST_LEAVE_GROUP`](#mcast_leave_group)
- [`MCAST_LEAVE_SOURCE_GROUP`](#mcast_leave_source_group)
- [`MCAST_MSFILTER`](#mcast_msfilter)
- [`MCAST_UNBLOCK_SOURCE`](#mcast_unblock_source)
- [`MINIX2_SUPER_MAGIC`](#minix2_super_magic)
- [`MINIX2_SUPER_MAGIC2`](#minix2_super_magic2)
- [`MINIX3_SUPER_MAGIC`](#minix3_super_magic)
- [`MINIX_SUPER_MAGIC`](#minix_super_magic)
- [`MINIX_SUPER_MAGIC2`](#minix_super_magic2)
- [`MMAP_PAGE_ZERO`](#mmap_page_zero)
- [`MNT_DETACH`](#mnt_detach)
- [`MNT_EXPIRE`](#mnt_expire)
- [`MNT_FORCE`](#mnt_force)
- [`MSDOS_SUPER_MAGIC`](#msdos_super_magic)
- [`MSG_CMSG_CLOEXEC`](#msg_cmsg_cloexec)
- [`MSG_CONFIRM`](#msg_confirm)
- [`MSG_CTRUNC`](#msg_ctrunc)
- [`MSG_DONTROUTE`](#msg_dontroute)
- [`MSG_DONTWAIT`](#msg_dontwait)
- [`MSG_EOR`](#msg_eor)
- [`MSG_ERRQUEUE`](#msg_errqueue)
- [`MSG_FASTOPEN`](#msg_fastopen)
- [`MSG_FIN`](#msg_fin)
- [`MSG_MORE`](#msg_more)
- [`MSG_NOSIGNAL`](#msg_nosignal)
- [`MSG_OOB`](#msg_oob)
- [`MSG_PEEK`](#msg_peek)
- [`MSG_RST`](#msg_rst)
- [`MSG_SYN`](#msg_syn)
- [`MSG_TRUNC`](#msg_trunc)
- [`MSG_WAITALL`](#msg_waitall)
- [`MSG_WAITFORONE`](#msg_waitforone)
- [`MS_ACTIVE`](#ms_active)
- [`MS_ASYNC`](#ms_async)
- [`MS_BIND`](#ms_bind)
- [`MS_DIRSYNC`](#ms_dirsync)
- [`MS_INVALIDATE`](#ms_invalidate)
- [`MS_I_VERSION`](#ms_i_version)
- [`MS_KERNMOUNT`](#ms_kernmount)
- [`MS_LAZYTIME`](#ms_lazytime)
- [`MS_MANDLOCK`](#ms_mandlock)
- [`MS_MGC_MSK`](#ms_mgc_msk)
- [`MS_MGC_VAL`](#ms_mgc_val)
- [`MS_MOVE`](#ms_move)
- [`MS_NOATIME`](#ms_noatime)
- [`MS_NODEV`](#ms_nodev)
- [`MS_NODIRATIME`](#ms_nodiratime)
- [`MS_NOEXEC`](#ms_noexec)
- [`MS_NOSUID`](#ms_nosuid)
- [`MS_NOSYMFOLLOW`](#ms_nosymfollow)
- [`MS_POSIXACL`](#ms_posixacl)
- [`MS_PRIVATE`](#ms_private)
- [`MS_RDONLY`](#ms_rdonly)
- [`MS_REC`](#ms_rec)
- [`MS_RELATIME`](#ms_relatime)
- [`MS_REMOUNT`](#ms_remount)
- [`MS_SHARED`](#ms_shared)
- [`MS_SILENT`](#ms_silent)
- [`MS_SLAVE`](#ms_slave)
- [`MS_STRICTATIME`](#ms_strictatime)
- [`MS_SYNC`](#ms_sync)
- [`MS_SYNCHRONOUS`](#ms_synchronous)
- [`MS_UNBINDABLE`](#ms_unbindable)
- [`NAME_MAX`](#name_max)
- [`NCP_SUPER_MAGIC`](#ncp_super_magic)
- [`NFS_SUPER_MAGIC`](#nfs_super_magic)
- [`NILFS_SUPER_MAGIC`](#nilfs_super_magic)
- [`NL0`](#nl0)
- [`NL1`](#nl1)
- [`NSFS_MAGIC`](#nsfs_magic)
- [`OCFS2_SUPER_MAGIC`](#ocfs2_super_magic)
- [`OCRNL`](#ocrnl)
- [`OFDEL`](#ofdel)
- [`OFILL`](#ofill)
- [`ONLRET`](#onlret)
- [`ONOCR`](#onocr)
- [`OPENPROM_SUPER_MAGIC`](#openprom_super_magic)
- [`OPOST`](#opost)
- [`OVERLAYFS_SUPER_MAGIC`](#overlayfs_super_magic)
- [`O_RDONLY`](#o_rdonly)
- [`O_RDWR`](#o_rdwr)
- [`O_WRONLY`](#o_wronly)
- [`PARMRK`](#parmrk)
- [`PATH_MAX`](#path_max)
- [`PF_ALG`](#pf_alg)
- [`PF_APPLETALK`](#pf_appletalk)
- [`PF_ASH`](#pf_ash)
- [`PF_ATMPVC`](#pf_atmpvc)
- [`PF_ATMSVC`](#pf_atmsvc)
- [`PF_AX25`](#pf_ax25)
- [`PF_BLUETOOTH`](#pf_bluetooth)
- [`PF_BRIDGE`](#pf_bridge)
- [`PF_CAIF`](#pf_caif)
- [`PF_CAN`](#pf_can)
- [`PF_DECnet`](#pf_decnet)
- [`PF_ECONET`](#pf_econet)
- [`PF_IEEE802154`](#pf_ieee802154)
- [`PF_INET`](#pf_inet)
- [`PF_INET6`](#pf_inet6)
- [`PF_IPX`](#pf_ipx)
- [`PF_IRDA`](#pf_irda)
- [`PF_ISDN`](#pf_isdn)
- [`PF_IUCV`](#pf_iucv)
- [`PF_KEY`](#pf_key)
- [`PF_LLC`](#pf_llc)
- [`PF_LOCAL`](#pf_local)
- [`PF_NETBEUI`](#pf_netbeui)
- [`PF_NETLINK`](#pf_netlink)
- [`PF_NETROM`](#pf_netrom)
- [`PF_PACKET`](#pf_packet)
- [`PF_PHONET`](#pf_phonet)
- [`PF_PPPOX`](#pf_pppox)
- [`PF_RDS`](#pf_rds)
- [`PF_ROSE`](#pf_rose)
- [`PF_ROUTE`](#pf_route)
- [`PF_RXRPC`](#pf_rxrpc)
- [`PF_SECURITY`](#pf_security)
- [`PF_SNA`](#pf_sna)
- [`PF_TIPC`](#pf_tipc)
- [`PF_UNIX`](#pf_unix)
- [`PF_UNSPEC`](#pf_unspec)
- [`PF_WANPIPE`](#pf_wanpipe)
- [`PF_X25`](#pf_x25)
- [`PIPE_BUF`](#pipe_buf)
- [`POLLERR`](#pollerr)
- [`POLLHUP`](#pollhup)
- [`POLLIN`](#pollin)
- [`POLLNVAL`](#pollnval)
- [`POLLOUT`](#pollout)
- [`POLLPRI`](#pollpri)
- [`POLLRDBAND`](#pollrdband)
- [`POLLRDHUP`](#pollrdhup)
- [`POLLRDNORM`](#pollrdnorm)
- [`POSIX_FADV_NORMAL`](#posix_fadv_normal)
- [`POSIX_FADV_RANDOM`](#posix_fadv_random)
- [`POSIX_FADV_SEQUENTIAL`](#posix_fadv_sequential)
- [`POSIX_FADV_WILLNEED`](#posix_fadv_willneed)
- [`PROC_SUPER_MAGIC`](#proc_super_magic)
- [`PROT_EXEC`](#prot_exec)
- [`PROT_GROWSDOWN`](#prot_growsdown)
- [`PROT_GROWSUP`](#prot_growsup)
- [`PROT_NONE`](#prot_none)
- [`PROT_READ`](#prot_read)
- [`PROT_WRITE`](#prot_write)
- [`PTHREAD_CREATE_DETACHED`](#pthread_create_detached)
- [`PTHREAD_CREATE_JOINABLE`](#pthread_create_joinable)
- [`PTRACE_EVENT_CLONE`](#ptrace_event_clone)
- [`PTRACE_EVENT_EXEC`](#ptrace_event_exec)
- [`PTRACE_EVENT_EXIT`](#ptrace_event_exit)
- [`PTRACE_EVENT_FORK`](#ptrace_event_fork)
- [`PTRACE_EVENT_SECCOMP`](#ptrace_event_seccomp)
- [`PTRACE_EVENT_VFORK`](#ptrace_event_vfork)
- [`PTRACE_EVENT_VFORK_DONE`](#ptrace_event_vfork_done)
- [`PTRACE_O_EXITKILL`](#ptrace_o_exitkill)
- [`PTRACE_O_MASK`](#ptrace_o_mask)
- [`PTRACE_O_SUSPEND_SECCOMP`](#ptrace_o_suspend_seccomp)
- [`PTRACE_O_TRACECLONE`](#ptrace_o_traceclone)
- [`PTRACE_O_TRACEEXEC`](#ptrace_o_traceexec)
- [`PTRACE_O_TRACEEXIT`](#ptrace_o_traceexit)
- [`PTRACE_O_TRACEFORK`](#ptrace_o_tracefork)
- [`PTRACE_O_TRACESECCOMP`](#ptrace_o_traceseccomp)
- [`PTRACE_O_TRACESYSGOOD`](#ptrace_o_tracesysgood)
- [`PTRACE_O_TRACEVFORK`](#ptrace_o_tracevfork)
- [`PTRACE_O_TRACEVFORKDONE`](#ptrace_o_tracevforkdone)
- [`P_ALL`](#p_all)
- [`P_PGID`](#p_pgid)
- [`P_PID`](#p_pid)
- [`P_PIDFD`](#p_pidfd)
- [`QIF_ALL`](#qif_all)
- [`QIF_BLIMITS`](#qif_blimits)
- [`QIF_BTIME`](#qif_btime)
- [`QIF_ILIMITS`](#qif_ilimits)
- [`QIF_INODES`](#qif_inodes)
- [`QIF_ITIME`](#qif_itime)
- [`QIF_LIMITS`](#qif_limits)
- [`QIF_SPACE`](#qif_space)
- [`QIF_TIMES`](#qif_times)
- [`QIF_USAGE`](#qif_usage)
- [`QNX4_SUPER_MAGIC`](#qnx4_super_magic)
- [`QNX6_SUPER_MAGIC`](#qnx6_super_magic)
- [`Q_GETFMT`](#q_getfmt)
- [`Q_GETINFO`](#q_getinfo)
- [`Q_GETQUOTA`](#q_getquota)
- [`Q_QUOTAOFF`](#q_quotaoff)
- [`Q_QUOTAON`](#q_quotaon)
- [`Q_SETINFO`](#q_setinfo)
- [`Q_SETQUOTA`](#q_setquota)
- [`Q_SYNC`](#q_sync)
- [`RAND_MAX`](#rand_max)
- [`RDTGROUP_SUPER_MAGIC`](#rdtgroup_super_magic)
- [`READ_IMPLIES_EXEC`](#read_implies_exec)
- [`REISERFS_SUPER_MAGIC`](#reiserfs_super_magic)
- [`RLIM64_INFINITY`](#rlim64_infinity)
- [`RTLD_LAZY`](#rtld_lazy)
- [`RTLD_LOCAL`](#rtld_local)
- [`RUSAGE_SELF`](#rusage_self)
- [`R_OK`](#r_ok)
- [`SCM_CREDENTIALS`](#scm_credentials)
- [`SCM_RIGHTS`](#scm_rights)
- [`SCM_TIMESTAMP`](#scm_timestamp)
- [`SECURITYFS_MAGIC`](#securityfs_magic)
- [`SEEK_CUR`](#seek_cur)
- [`SEEK_END`](#seek_end)
- [`SEEK_SET`](#seek_set)
- [`SELINUX_MAGIC`](#selinux_magic)
- [`SHORT_INODE`](#short_inode)
- [`SHUT_RD`](#shut_rd)
- [`SHUT_RDWR`](#shut_rdwr)
- [`SHUT_WR`](#shut_wr)
- [`SIGABRT`](#sigabrt)
- [`SIGALRM`](#sigalrm)
- [`SIGEV_NONE`](#sigev_none)
- [`SIGEV_SIGNAL`](#sigev_signal)
- [`SIGEV_THREAD`](#sigev_thread)
- [`SIGFPE`](#sigfpe)
- [`SIGHUP`](#sighup)
- [`SIGILL`](#sigill)
- [`SIGINT`](#sigint)
- [`SIGKILL`](#sigkill)
- [`SIGPIPE`](#sigpipe)
- [`SIGQUIT`](#sigquit)
- [`SIGSEGV`](#sigsegv)
- [`SIGTERM`](#sigterm)
- [`SIGTRAP`](#sigtrap)
- [`SI_ASYNCIO`](#si_asyncio)
- [`SI_ASYNCNL`](#si_asyncnl)
- [`SI_KERNEL`](#si_kernel)
- [`SI_LOAD_SHIFT`](#si_load_shift)
- [`SI_MESGQ`](#si_mesgq)
- [`SI_QUEUE`](#si_queue)
- [`SI_SIGIO`](#si_sigio)
- [`SI_TIMER`](#si_timer)
- [`SI_TKILL`](#si_tkill)
- [`SI_USER`](#si_user)
- [`SMACK_MAGIC`](#smack_magic)
- [`SMB_SUPER_MAGIC`](#smb_super_magic)
- [`SOCK_CLOEXEC`](#sock_cloexec)
- [`SOCK_RAW`](#sock_raw)
- [`SOCK_RDM`](#sock_rdm)
- [`SOL_AAL`](#sol_aal)
- [`SOL_ALG`](#sol_alg)
- [`SOL_ATM`](#sol_atm)
- [`SOL_BLUETOOTH`](#sol_bluetooth)
- [`SOL_DCCP`](#sol_dccp)
- [`SOL_DECNET`](#sol_decnet)
- [`SOL_ICMPV6`](#sol_icmpv6)
- [`SOL_IP`](#sol_ip)
- [`SOL_IPV6`](#sol_ipv6)
- [`SOL_IRDA`](#sol_irda)
- [`SOL_LLC`](#sol_llc)
- [`SOL_NETBEUI`](#sol_netbeui)
- [`SOL_NETLINK`](#sol_netlink)
- [`SOL_PACKET`](#sol_packet)
- [`SOL_RAW`](#sol_raw)
- [`SOL_TCP`](#sol_tcp)
- [`SOL_TIPC`](#sol_tipc)
- [`SOL_UDP`](#sol_udp)
- [`SOL_X25`](#sol_x25)
- [`SO_DEBUG`](#so_debug)
- [`SPLICE_F_GIFT`](#splice_f_gift)
- [`SPLICE_F_MORE`](#splice_f_more)
- [`SPLICE_F_MOVE`](#splice_f_move)
- [`SPLICE_F_NONBLOCK`](#splice_f_nonblock)
- [`SS_DISABLE`](#ss_disable)
- [`SS_ONSTACK`](#ss_onstack)
- [`STATX_ALL`](#statx_all)
- [`STATX_ATIME`](#statx_atime)
- [`STATX_ATTR_APPEND`](#statx_attr_append)
- [`STATX_ATTR_AUTOMOUNT`](#statx_attr_automount)
- [`STATX_ATTR_COMPRESSED`](#statx_attr_compressed)
- [`STATX_ATTR_DAX`](#statx_attr_dax)
- [`STATX_ATTR_ENCRYPTED`](#statx_attr_encrypted)
- [`STATX_ATTR_IMMUTABLE`](#statx_attr_immutable)
- [`STATX_ATTR_MOUNT_ROOT`](#statx_attr_mount_root)
- [`STATX_ATTR_NODUMP`](#statx_attr_nodump)
- [`STATX_ATTR_VERITY`](#statx_attr_verity)
- [`STATX_BASIC_STATS`](#statx_basic_stats)
- [`STATX_BLOCKS`](#statx_blocks)
- [`STATX_BTIME`](#statx_btime)
- [`STATX_CTIME`](#statx_ctime)
- [`STATX_DIOALIGN`](#statx_dioalign)
- [`STATX_GID`](#statx_gid)
- [`STATX_INO`](#statx_ino)
- [`STATX_MNT_ID`](#statx_mnt_id)
- [`STATX_MODE`](#statx_mode)
- [`STATX_MTIME`](#statx_mtime)
- [`STATX_NLINK`](#statx_nlink)
- [`STATX_SIZE`](#statx_size)
- [`STATX_TYPE`](#statx_type)
- [`STATX_UID`](#statx_uid)
- [`STATX__RESERVED`](#statx__reserved)
- [`STICKY_TIMEOUTS`](#sticky_timeouts)
- [`SYSFS_MAGIC`](#sysfs_magic)
- [`S_IFBLK`](#s_ifblk)
- [`S_IFCHR`](#s_ifchr)
- [`S_IFDIR`](#s_ifdir)
- [`S_IFIFO`](#s_ififo)
- [`S_IFLNK`](#s_iflnk)
- [`S_IFMT`](#s_ifmt)
- [`S_IFREG`](#s_ifreg)
- [`S_IFSOCK`](#s_ifsock)
- [`S_IRGRP`](#s_irgrp)
- [`S_IROTH`](#s_iroth)
- [`S_IRUSR`](#s_irusr)
- [`S_IRWXG`](#s_irwxg)
- [`S_IRWXO`](#s_irwxo)
- [`S_IRWXU`](#s_irwxu)
- [`S_IWGRP`](#s_iwgrp)
- [`S_IWOTH`](#s_iwoth)
- [`S_IWUSR`](#s_iwusr)
- [`S_IXGRP`](#s_ixgrp)
- [`S_IXOTH`](#s_ixoth)
- [`S_IXUSR`](#s_ixusr)
- [`TAB0`](#tab0)
- [`TCIFLUSH`](#tciflush)
- [`TCIOFF`](#tcioff)
- [`TCIOFLUSH`](#tcioflush)
- [`TCION`](#tcion)
- [`TCOFLUSH`](#tcoflush)
- [`TCOOFF`](#tcooff)
- [`TCOON`](#tcoon)
- [`TCP_CC_INFO`](#tcp_cc_info)
- [`TCP_CM_INQ`](#tcp_cm_inq)
- [`TCP_CONGESTION`](#tcp_congestion)
- [`TCP_COOKIE_TRANSACTIONS`](#tcp_cookie_transactions)
- [`TCP_CORK`](#tcp_cork)
- [`TCP_DEFER_ACCEPT`](#tcp_defer_accept)
- [`TCP_FASTOPEN`](#tcp_fastopen)
- [`TCP_FASTOPEN_CONNECT`](#tcp_fastopen_connect)
- [`TCP_FASTOPEN_KEY`](#tcp_fastopen_key)
- [`TCP_FASTOPEN_NO_COOKIE`](#tcp_fastopen_no_cookie)
- [`TCP_INFO`](#tcp_info)
- [`TCP_INQ`](#tcp_inq)
- [`TCP_KEEPCNT`](#tcp_keepcnt)
- [`TCP_KEEPIDLE`](#tcp_keepidle)
- [`TCP_KEEPINTVL`](#tcp_keepintvl)
- [`TCP_LINGER2`](#tcp_linger2)
- [`TCP_MAXSEG`](#tcp_maxseg)
- [`TCP_MD5SIG`](#tcp_md5sig)
- [`TCP_MD5SIG_EXT`](#tcp_md5sig_ext)
- [`TCP_MD5SIG_MAXKEYLEN`](#tcp_md5sig_maxkeylen)
- [`TCP_NODELAY`](#tcp_nodelay)
- [`TCP_NOTSENT_LOWAT`](#tcp_notsent_lowat)
- [`TCP_QUEUE_SEQ`](#tcp_queue_seq)
- [`TCP_QUICKACK`](#tcp_quickack)
- [`TCP_REPAIR`](#tcp_repair)
- [`TCP_REPAIR_OPTIONS`](#tcp_repair_options)
- [`TCP_REPAIR_QUEUE`](#tcp_repair_queue)
- [`TCP_REPAIR_WINDOW`](#tcp_repair_window)
- [`TCP_SAVED_SYN`](#tcp_saved_syn)
- [`TCP_SAVE_SYN`](#tcp_save_syn)
- [`TCP_SYNCNT`](#tcp_syncnt)
- [`TCP_THIN_DUPACK`](#tcp_thin_dupack)
- [`TCP_THIN_LINEAR_TIMEOUTS`](#tcp_thin_linear_timeouts)
- [`TCP_TIMESTAMP`](#tcp_timestamp)
- [`TCP_ULP`](#tcp_ulp)
- [`TCP_USER_TIMEOUT`](#tcp_user_timeout)
- [`TCP_WINDOW_CLAMP`](#tcp_window_clamp)
- [`TCP_ZEROCOPY_RECEIVE`](#tcp_zerocopy_receive)
- [`TIMER_ABSTIME`](#timer_abstime)
- [`TMPFS_MAGIC`](#tmpfs_magic)
- [`TRACEFS_MAGIC`](#tracefs_magic)
- [`TRAP_BRANCH`](#trap_branch)
- [`TRAP_BRKPT`](#trap_brkpt)
- [`TRAP_HWBKPT`](#trap_hwbkpt)
- [`TRAP_TRACE`](#trap_trace)
- [`TRAP_UNK`](#trap_unk)
- [`TUNATTACHFILTER`](#tunattachfilter)
- [`TUNDETACHFILTER`](#tundetachfilter)
- [`TUNGETDEVNETNS`](#tungetdevnetns)
- [`TUNGETFEATURES`](#tungetfeatures)
- [`TUNGETFILTER`](#tungetfilter)
- [`TUNGETIFF`](#tungetiff)
- [`TUNGETSNDBUF`](#tungetsndbuf)
- [`TUNGETVNETBE`](#tungetvnetbe)
- [`TUNGETVNETHDRSZ`](#tungetvnethdrsz)
- [`TUNGETVNETLE`](#tungetvnetle)
- [`TUNSETCARRIER`](#tunsetcarrier)
- [`TUNSETDEBUG`](#tunsetdebug)
- [`TUNSETFILTEREBPF`](#tunsetfilterebpf)
- [`TUNSETGROUP`](#tunsetgroup)
- [`TUNSETIFF`](#tunsetiff)
- [`TUNSETIFINDEX`](#tunsetifindex)
- [`TUNSETLINK`](#tunsetlink)
- [`TUNSETNOCSUM`](#tunsetnocsum)
- [`TUNSETOFFLOAD`](#tunsetoffload)
- [`TUNSETOWNER`](#tunsetowner)
- [`TUNSETPERSIST`](#tunsetpersist)
- [`TUNSETQUEUE`](#tunsetqueue)
- [`TUNSETSNDBUF`](#tunsetsndbuf)
- [`TUNSETSTEERINGEBPF`](#tunsetsteeringebpf)
- [`TUNSETTXFILTER`](#tunsettxfilter)
- [`TUNSETVNETBE`](#tunsetvnetbe)
- [`TUNSETVNETHDRSZ`](#tunsetvnethdrsz)
- [`TUNSETVNETLE`](#tunsetvnetle)
- [`TUN_FLT_ALLMULTI`](#tun_flt_allmulti)
- [`TUN_F_CSUM`](#tun_f_csum)
- [`TUN_F_TSO4`](#tun_f_tso4)
- [`TUN_F_TSO6`](#tun_f_tso6)
- [`TUN_F_TSO_ECN`](#tun_f_tso_ecn)
- [`TUN_F_UFO`](#tun_f_ufo)
- [`TUN_F_USO4`](#tun_f_uso4)
- [`TUN_F_USO6`](#tun_f_uso6)
- [`TUN_PKT_STRIP`](#tun_pkt_strip)
- [`TUN_READQ_SIZE`](#tun_readq_size)
- [`TUN_TAP_DEV`](#tun_tap_dev)
- [`TUN_TUN_DEV`](#tun_tun_dev)
- [`TUN_TX_TIMESTAMP`](#tun_tx_timestamp)
- [`TUN_TYPE_MASK`](#tun_type_mask)
- [`UDF_SUPER_MAGIC`](#udf_super_magic)
- [`UIO_MAXIOV`](#uio_maxiov)
- [`UMOUNT_NOFOLLOW`](#umount_nofollow)
- [`USBDEVICE_SUPER_MAGIC`](#usbdevice_super_magic)
- [`UTIME_NOW`](#utime_now)
- [`UTIME_OMIT`](#utime_omit)
- [`VERASE`](#verase)
- [`VINTR`](#vintr)
- [`VKILL`](#vkill)
- [`VLNEXT`](#vlnext)
- [`VQUIT`](#vquit)
- [`VT0`](#vt0)
- [`WCONTINUED`](#wcontinued)
- [`WEXITED`](#wexited)
- [`WHOLE_SECONDS`](#whole_seconds)
- [`WNOHANG`](#wnohang)
- [`WNOWAIT`](#wnowait)
- [`WSTOPPED`](#wstopped)
- [`WUNTRACED`](#wuntraced)
- [`W_OK`](#w_ok)
- [`XATTR_CREATE`](#xattr_create)
- [`XATTR_REPLACE`](#xattr_replace)
- [`XENFS_SUPER_MAGIC`](#xenfs_super_magic)
- [`X_OK`](#x_ok)
- [`_IOFBF`](#_iofbf)
- [`_IOLBF`](#_iolbf)
- [`_IONBF`](#_ionbf)
- [`__WALL`](#__wall)
- [`__WCLONE`](#__wclone)
- [`__WNOTHREAD`](#__wnothread)

**Type Aliases**

- [`clockid_t`](#clockid_t)
- [`id_t`](#id_t)
- [`key_t`](#key_t)
- [`sa_family_t`](#sa_family_t)
- [`speed_t`](#speed_t)
- [`tcflag_t`](#tcflag_t)
- [`timer_t`](#timer_t)
- [`useconds_t`](#useconds_t)

---

## libc::unix::linux_like::ADDR_COMPAT_LAYOUT

*Constant*: `c_int`



## libc::unix::linux_like::ADDR_LIMIT_32BIT

*Constant*: `c_int`



## libc::unix::linux_like::ADDR_LIMIT_3GB

*Constant*: `c_int`



## libc::unix::linux_like::ADDR_NO_RANDOMIZE

*Constant*: `c_int`



## libc::unix::linux_like::ADFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::AFFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::AFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::AF_ALG

*Constant*: `c_int`



## libc::unix::linux_like::AF_APPLETALK

*Constant*: `c_int`



## libc::unix::linux_like::AF_ASH

*Constant*: `c_int`



## libc::unix::linux_like::AF_ATMPVC

*Constant*: `c_int`



## libc::unix::linux_like::AF_ATMSVC

*Constant*: `c_int`



## libc::unix::linux_like::AF_AX25

*Constant*: `c_int`



## libc::unix::linux_like::AF_BLUETOOTH

*Constant*: `c_int`



## libc::unix::linux_like::AF_BRIDGE

*Constant*: `c_int`



## libc::unix::linux_like::AF_CAIF

*Constant*: `c_int`



## libc::unix::linux_like::AF_CAN

*Constant*: `c_int`



## libc::unix::linux_like::AF_DECnet

*Constant*: `c_int`



## libc::unix::linux_like::AF_ECONET

*Constant*: `c_int`



## libc::unix::linux_like::AF_IEEE802154

*Constant*: `c_int`



## libc::unix::linux_like::AF_INET

*Constant*: `c_int`



## libc::unix::linux_like::AF_INET6

*Constant*: `c_int`



## libc::unix::linux_like::AF_IPX

*Constant*: `c_int`



## libc::unix::linux_like::AF_IRDA

*Constant*: `c_int`



## libc::unix::linux_like::AF_ISDN

*Constant*: `c_int`



## libc::unix::linux_like::AF_IUCV

*Constant*: `c_int`



## libc::unix::linux_like::AF_KEY

*Constant*: `c_int`



## libc::unix::linux_like::AF_LLC

*Constant*: `c_int`



## libc::unix::linux_like::AF_LOCAL

*Constant*: `c_int`



## libc::unix::linux_like::AF_NETBEUI

*Constant*: `c_int`



## libc::unix::linux_like::AF_NETLINK

*Constant*: `c_int`



## libc::unix::linux_like::AF_NETROM

*Constant*: `c_int`



## libc::unix::linux_like::AF_PACKET

*Constant*: `c_int`



## libc::unix::linux_like::AF_PHONET

*Constant*: `c_int`



## libc::unix::linux_like::AF_PPPOX

*Constant*: `c_int`



## libc::unix::linux_like::AF_RDS

*Constant*: `c_int`



## libc::unix::linux_like::AF_ROSE

*Constant*: `c_int`



## libc::unix::linux_like::AF_ROUTE

*Constant*: `c_int`



## libc::unix::linux_like::AF_RXRPC

*Constant*: `c_int`



## libc::unix::linux_like::AF_SECURITY

*Constant*: `c_int`



## libc::unix::linux_like::AF_SNA

*Constant*: `c_int`



## libc::unix::linux_like::AF_TIPC

*Constant*: `c_int`



## libc::unix::linux_like::AF_UNIX

*Constant*: `c_int`



## libc::unix::linux_like::AF_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::AF_WANPIPE

*Constant*: `c_int`



## libc::unix::linux_like::AF_X25

*Constant*: `c_int`



## libc::unix::linux_like::ARPHRD_ADAPT

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_APPLETLK

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_ARCNET

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_ASH

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_ATM

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_AX25

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_BIF

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_CAN

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_CHAOS

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_CISCO

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_CSLIP

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_CSLIP6

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_DDCMP

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_DLCI

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_ECONET

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_EETHER

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_ETHER

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_EUI64

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_FCAL

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_FCFABRIC

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_FCPL

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_FCPP

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_FDDI

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_FRAD

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_HDLC

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_HIPPI

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_HWX25

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_IEEE1394

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_IEEE802

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_IEEE80211

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_IEEE80211_PRISM

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_IEEE80211_RADIOTAP

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_IEEE802154

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_IEEE802_TR

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_INFINIBAND

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_IPDDP

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_IPGRE

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_IRDA

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_LAPB

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_LOCALTLK

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_LOOPBACK

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_METRICOM

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_NETROM

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_NONE

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_PIMREG

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_PPP

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_PRONET

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_RAWHDLC

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_ROSE

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_RSRVD

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_SIT

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_SKIP

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_SLIP

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_SLIP6

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_TUNNEL

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_TUNNEL6

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_VOID

*Constant*: `u16`



## libc::unix::linux_like::ARPHRD_X25

*Constant*: `u16`



## libc::unix::linux_like::ARPOP_InREPLY

*Constant*: `u16`



## libc::unix::linux_like::ARPOP_InREQUEST

*Constant*: `u16`



## libc::unix::linux_like::ARPOP_NAK

*Constant*: `u16`



## libc::unix::linux_like::ARPOP_RREPLY

*Constant*: `u16`



## libc::unix::linux_like::ARPOP_RREQUEST

*Constant*: `u16`



## libc::unix::linux_like::ATF_DONTPUB

*Constant*: `c_int`



## libc::unix::linux_like::ATF_NETMASK

*Constant*: `c_int`



## libc::unix::linux_like::AT_EMPTY_PATH

*Constant*: `c_int`



## libc::unix::linux_like::AT_FDCWD

*Constant*: `c_int`



## libc::unix::linux_like::AT_NO_AUTOMOUNT

*Constant*: `c_int`



## libc::unix::linux_like::AT_RECURSIVE

*Constant*: `c_int`



## libc::unix::linux_like::AT_REMOVEDIR

*Constant*: `c_int`



## libc::unix::linux_like::AT_STATX_DONT_SYNC

*Constant*: `c_int`



## libc::unix::linux_like::AT_STATX_FORCE_SYNC

*Constant*: `c_int`



## libc::unix::linux_like::AT_STATX_SYNC_AS_STAT

*Constant*: `c_int`



## libc::unix::linux_like::AT_STATX_SYNC_TYPE

*Constant*: `c_int`



## libc::unix::linux_like::AT_SYMLINK_FOLLOW

*Constant*: `c_int`



## libc::unix::linux_like::AT_SYMLINK_NOFOLLOW

*Constant*: `c_int`



## libc::unix::linux_like::AUTOFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::BPF_FS_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::BRKINT

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::BS0

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::BTRFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::BUS_ADRALN

*Constant*: `c_int`



## libc::unix::linux_like::BUS_ADRERR

*Constant*: `c_int`



## libc::unix::linux_like::BUS_MCEERR_AO

*Constant*: `c_int`



## libc::unix::linux_like::BUS_MCEERR_AR

*Constant*: `c_int`



## libc::unix::linux_like::BUS_OBJERR

*Constant*: `c_int`



## libc::unix::linux_like::CGROUP2_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::CGROUP_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::CLD_CONTINUED

*Constant*: `c_int`



## libc::unix::linux_like::CLD_DUMPED

*Constant*: `c_int`



## libc::unix::linux_like::CLD_EXITED

*Constant*: `c_int`



## libc::unix::linux_like::CLD_KILLED

*Constant*: `c_int`



## libc::unix::linux_like::CLD_STOPPED

*Constant*: `c_int`



## libc::unix::linux_like::CLD_TRAPPED

*Constant*: `c_int`



## libc::unix::linux_like::CLOCK_BOOTTIME

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLOCK_BOOTTIME_ALARM

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLOCK_MONOTONIC

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLOCK_MONOTONIC_COARSE

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLOCK_MONOTONIC_RAW

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLOCK_PROCESS_CPUTIME_ID

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLOCK_REALTIME

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLOCK_REALTIME_ALARM

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLOCK_REALTIME_COARSE

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLOCK_TAI

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLOCK_THREAD_CPUTIME_ID

*Constant*: `crate::clockid_t`



## libc::unix::linux_like::CLONE_CHILD_CLEARTID

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_CHILD_SETTID

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_DETACHED

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_FILES

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_FS

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_IO

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_NEWCGROUP

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_NEWIPC

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_NEWNET

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_NEWNS

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_NEWPID

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_NEWUSER

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_NEWUTS

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_PARENT

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_PARENT_SETTID

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_PTRACE

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_SETTLS

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_SIGHAND

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_SYSVSEM

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_THREAD

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_UNTRACED

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_VFORK

*Constant*: `c_int`



## libc::unix::linux_like::CLONE_VM

*Constant*: `c_int`



## libc::unix::linux_like::CMSG_DATA

*Function*

```rust
fn CMSG_DATA(cmsg: *const crate::cmsghdr) -> *mut c_uchar
```



## libc::unix::linux_like::CMSG_FIRSTHDR

*Function*

```rust
fn CMSG_FIRSTHDR(mhdr: *const crate::msghdr) -> *mut crate::cmsghdr
```



## libc::unix::linux_like::CMSG_LEN

*Function*

```rust
fn CMSG_LEN(length: c_uint) -> c_uint
```



## libc::unix::linux_like::CMSG_SPACE

*Function*

```rust
fn CMSG_SPACE(length: c_uint) -> c_uint
```



## libc::unix::linux_like::CODA_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::CR0

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::CRAMFS_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::CRTSCTS

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::CS5

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::DEBUGFS_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::DEVPTS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::Dl_info

*Struct*

**Fields:**
- `dli_fname: *const c_char`
- `dli_fbase: *mut c_void`
- `dli_sname: *const c_char`
- `dli_saddr: *mut c_void`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Dl_info`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::ECHO

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::ECRYPTFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::EFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::EOF

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLERR

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLET

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLEXCLUSIVE

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLHUP

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLIN

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLMSG

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLONESHOT

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLOUT

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLPRI

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLRDBAND

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLRDHUP

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLRDNORM

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLWAKEUP

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLWRBAND

*Constant*: `c_int`



## libc::unix::linux_like::EPOLLWRNORM

*Constant*: `c_int`



## libc::unix::linux_like::EPOLL_CTL_ADD

*Constant*: `c_int`



## libc::unix::linux_like::EPOLL_CTL_DEL

*Constant*: `c_int`



## libc::unix::linux_like::EPOLL_CTL_MOD

*Constant*: `c_int`



## libc::unix::linux_like::EXIT_FAILURE

*Constant*: `c_int`



## libc::unix::linux_like::EXIT_SUCCESS

*Constant*: `c_int`



## libc::unix::linux_like::EXT2_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::EXT3_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::EXT4_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::F2FS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::FD_CLR

*Function*

```rust
fn FD_CLR(fd: c_int, set: *mut fd_set)
```



## libc::unix::linux_like::FD_ISSET

*Function*

```rust
fn FD_ISSET(fd: c_int, set: *const fd_set) -> bool
```



## libc::unix::linux_like::FD_SET

*Function*

```rust
fn FD_SET(fd: c_int, set: *mut fd_set)
```



## libc::unix::linux_like::FD_SETSIZE

*Constant*: `usize`



## libc::unix::linux_like::FD_ZERO

*Function*

```rust
fn FD_ZERO(set: *mut fd_set)
```



## libc::unix::linux_like::FF0

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::FICLONE

*Constant*: `c_ulong`



## libc::unix::linux_like::FICLONERANGE

*Constant*: `c_ulong`



## libc::unix::linux_like::FS_IOC32_GETFLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::FS_IOC32_GETVERSION

*Constant*: `c_ulong`



## libc::unix::linux_like::FS_IOC32_SETFLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::FS_IOC32_SETVERSION

*Constant*: `c_ulong`



## libc::unix::linux_like::FS_IOC_GETFLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::FS_IOC_GETVERSION

*Constant*: `c_ulong`



## libc::unix::linux_like::FS_IOC_SETFLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::FS_IOC_SETVERSION

*Constant*: `c_ulong`



## libc::unix::linux_like::FUSE_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::FUTEXFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::F_ADD_SEALS

*Constant*: `c_int`



## libc::unix::linux_like::F_CANCELLK

*Constant*: `c_int`



## libc::unix::linux_like::F_DUPFD

*Constant*: `c_int`



## libc::unix::linux_like::F_DUPFD_CLOEXEC

*Constant*: `c_int`



## libc::unix::linux_like::F_GETFD

*Constant*: `c_int`



## libc::unix::linux_like::F_GETFL

*Constant*: `c_int`



## libc::unix::linux_like::F_GETLEASE

*Constant*: `c_int`



## libc::unix::linux_like::F_GETPIPE_SZ

*Constant*: `c_int`



## libc::unix::linux_like::F_GET_SEALS

*Constant*: `c_int`



## libc::unix::linux_like::F_NOTIFY

*Constant*: `c_int`



## libc::unix::linux_like::F_OK

*Constant*: `c_int`



## libc::unix::linux_like::F_SEAL_GROW

*Constant*: `c_int`



## libc::unix::linux_like::F_SEAL_SEAL

*Constant*: `c_int`



## libc::unix::linux_like::F_SEAL_SHRINK

*Constant*: `c_int`



## libc::unix::linux_like::F_SEAL_WRITE

*Constant*: `c_int`



## libc::unix::linux_like::F_SETFD

*Constant*: `c_int`



## libc::unix::linux_like::F_SETFL

*Constant*: `c_int`



## libc::unix::linux_like::F_SETLEASE

*Constant*: `c_int`



## libc::unix::linux_like::F_SETPIPE_SZ

*Constant*: `c_int`



## libc::unix::linux_like::HOSTFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::HPFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::HUGETLBFS_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::ICRNL

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::IFF_ALLMULTI

*Constant*: `c_int`



## libc::unix::linux_like::IFF_ATTACH_QUEUE

*Constant*: `c_int`



## libc::unix::linux_like::IFF_AUTOMEDIA

*Constant*: `c_int`



## libc::unix::linux_like::IFF_BROADCAST

*Constant*: `c_int`



## libc::unix::linux_like::IFF_DEBUG

*Constant*: `c_int`



## libc::unix::linux_like::IFF_DETACH_QUEUE

*Constant*: `c_int`



## libc::unix::linux_like::IFF_DYNAMIC

*Constant*: `c_int`



## libc::unix::linux_like::IFF_LOOPBACK

*Constant*: `c_int`



## libc::unix::linux_like::IFF_MASTER

*Constant*: `c_int`



## libc::unix::linux_like::IFF_MULTICAST

*Constant*: `c_int`



## libc::unix::linux_like::IFF_MULTI_QUEUE

*Constant*: `c_int`



## libc::unix::linux_like::IFF_NAPI

*Constant*: `c_int`



## libc::unix::linux_like::IFF_NAPI_FRAGS

*Constant*: `c_int`



## libc::unix::linux_like::IFF_NOARP

*Constant*: `c_int`



## libc::unix::linux_like::IFF_NOFILTER

*Constant*: `c_int`



## libc::unix::linux_like::IFF_NOTRAILERS

*Constant*: `c_int`



## libc::unix::linux_like::IFF_NO_CARRIER

*Constant*: `c_int`



## libc::unix::linux_like::IFF_NO_PI

*Constant*: `c_int`



## libc::unix::linux_like::IFF_ONE_QUEUE

*Constant*: `c_int`



## libc::unix::linux_like::IFF_PERSIST

*Constant*: `c_int`



## libc::unix::linux_like::IFF_POINTOPOINT

*Constant*: `c_int`



## libc::unix::linux_like::IFF_PORTSEL

*Constant*: `c_int`



## libc::unix::linux_like::IFF_PROMISC

*Constant*: `c_int`



## libc::unix::linux_like::IFF_RUNNING

*Constant*: `c_int`



## libc::unix::linux_like::IFF_SLAVE

*Constant*: `c_int`



## libc::unix::linux_like::IFF_TAP

*Constant*: `c_int`



## libc::unix::linux_like::IFF_TUN

*Constant*: `c_int`



## libc::unix::linux_like::IFF_TUN_EXCL

*Constant*: `c_int`



## libc::unix::linux_like::IFF_UP

*Constant*: `c_int`



## libc::unix::linux_like::IFF_VNET_HDR

*Constant*: `c_int`



## libc::unix::linux_like::IGNBRK

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::IGNCR

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::IGNPAR

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::IMAXBEL

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::INLCR

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::INPCK

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::IPDEFTTL

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_CLASS

*Function*

```rust
fn IPOPT_CLASS(o: u8) -> u8
```



## libc::unix::linux_like::IPOPT_CLASS_MASK

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_CONTROL

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_COPIED

*Function*

```rust
fn IPOPT_COPIED(o: u8) -> u8
```



## libc::unix::linux_like::IPOPT_COPY

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_END

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_EOL

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_LSRR

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_MEASUREMENT

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_MINOFF

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_NOOP

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_NOP

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_NUMBER

*Function*

```rust
fn IPOPT_NUMBER(o: u8) -> u8
```



## libc::unix::linux_like::IPOPT_NUMBER_MASK

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_OFFSET

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_OLEN

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_OPTVAL

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_RA

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_RESERVED1

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_RESERVED2

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_RR

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_SEC

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_SID

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_SSRR

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_TIMESTAMP

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_TS

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_TS_PRESPEC

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_TS_TSANDADDR

*Constant*: `u8`



## libc::unix::linux_like::IPOPT_TS_TSONLY

*Constant*: `u8`



## libc::unix::linux_like::IPPROTO_AH

*Constant*: `c_int`

IP6 Auth Header



## libc::unix::linux_like::IPPROTO_BEETPH

*Constant*: `c_int`



## libc::unix::linux_like::IPPROTO_COMP

*Constant*: `c_int`

IP Payload Comp. Protocol



## libc::unix::linux_like::IPPROTO_DCCP

*Constant*: `c_int`

DCCP



## libc::unix::linux_like::IPPROTO_DSTOPTS

*Constant*: `c_int`

IP6 destination option



## libc::unix::linux_like::IPPROTO_EGP

*Constant*: `c_int`

exterior gateway protocol



## libc::unix::linux_like::IPPROTO_ENCAP

*Constant*: `c_int`

encapsulation header



## libc::unix::linux_like::IPPROTO_ESP

*Constant*: `c_int`

IP6 Encap Sec. Payload



## libc::unix::linux_like::IPPROTO_ETHERNET

*Constant*: `c_int`

Ethernet-within-IPv6 encapsulation.



## libc::unix::linux_like::IPPROTO_FRAGMENT

*Constant*: `c_int`

IP6 fragmentation header



## libc::unix::linux_like::IPPROTO_GRE

*Constant*: `c_int`

General Routing Encap.



## libc::unix::linux_like::IPPROTO_HOPOPTS

*Constant*: `c_int`

Hop-by-hop option header



## libc::unix::linux_like::IPPROTO_IDP

*Constant*: `c_int`

xns idp



## libc::unix::linux_like::IPPROTO_IGMP

*Constant*: `c_int`

group mgmt protocol



## libc::unix::linux_like::IPPROTO_IPIP

*Constant*: `c_int`

for compatibility



## libc::unix::linux_like::IPPROTO_MH

*Constant*: `c_int`



## libc::unix::linux_like::IPPROTO_MPLS

*Constant*: `c_int`



## libc::unix::linux_like::IPPROTO_MPTCP

*Constant*: `c_int`

Multipath TCP



## libc::unix::linux_like::IPPROTO_MTP

*Constant*: `c_int`



## libc::unix::linux_like::IPPROTO_NONE

*Constant*: `c_int`

IP6 no next header



## libc::unix::linux_like::IPPROTO_PIM

*Constant*: `c_int`

Protocol indep. multicast



## libc::unix::linux_like::IPPROTO_PUP

*Constant*: `c_int`

pup



## libc::unix::linux_like::IPPROTO_RAW

*Constant*: `c_int`

raw IP packet



## libc::unix::linux_like::IPPROTO_ROUTING

*Constant*: `c_int`

IP6 routing header



## libc::unix::linux_like::IPPROTO_RSVP

*Constant*: `c_int`

resource reservation



## libc::unix::linux_like::IPPROTO_SCTP

*Constant*: `c_int`

SCTP



## libc::unix::linux_like::IPPROTO_TP

*Constant*: `c_int`

tp-4 w/ class negotiation



## libc::unix::linux_like::IPPROTO_UDPLITE

*Constant*: `c_int`



## libc::unix::linux_like::IPTOS_ECN

*Function*

```rust
fn IPTOS_ECN(x: u8) -> u8
```



## libc::unix::linux_like::IPTOS_ECN_CE

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_ECN_ECT0

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_ECN_ECT1

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_ECN_MASK

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_LOWDELAY

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_MINCOST

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_PREC_CRITIC_ECP

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_PREC_FLASH

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_PREC_FLASHOVERRIDE

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_PREC_IMMEDIATE

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_PREC_INTERNETCONTROL

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_PREC_NETCONTROL

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_PREC_PRIORITY

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_PREC_ROUTINE

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_RELIABILITY

*Constant*: `u8`



## libc::unix::linux_like::IPTOS_THROUGHPUT

*Constant*: `u8`



## libc::unix::linux_like::IPV6_2292DSTOPTS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_2292HOPLIMIT

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_2292HOPOPTS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_2292PKTINFO

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_2292PKTOPTIONS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_2292RTHDR

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_ADDRFORM

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_ADDR_PREFERENCES

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_ADD_MEMBERSHIP

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_AUTHHDR

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_AUTOFLOWLABEL

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_CHECKSUM

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_DONTFRAG

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_DROP_MEMBERSHIP

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_DSTOPTS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_HDRINCL

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_HOPLIMIT

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_HOPOPTS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_IPSEC_POLICY

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_JOIN_ANYCAST

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_LEAVE_ANYCAST

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_MINHOPCOUNT

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_MTU

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_MTU_DISCOVER

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_MULTICAST_HOPS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_MULTICAST_IF

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_MULTICAST_LOOP

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_NEXTHOP

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_ORIGDSTADDR

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PATHMTU

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PKTINFO

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PMTUDISC_DO

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PMTUDISC_DONT

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PMTUDISC_INTERFACE

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PMTUDISC_OMIT

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PMTUDISC_PROBE

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PMTUDISC_WANT

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PREFER_SRC_CGA

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PREFER_SRC_COA

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PREFER_SRC_HOME

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PREFER_SRC_NONCGA

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PREFER_SRC_PUBLIC

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PREFER_SRC_PUBTMP_DEFAULT

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_PREFER_SRC_TMP

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RECVDSTOPTS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RECVERR

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RECVHOPLIMIT

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RECVHOPOPTS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RECVORIGDSTADDR

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RECVPATHMTU

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RECVPKTINFO

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RECVRTHDR

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RECVTCLASS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_ROUTER_ALERT

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RTHDR

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_RTHDRDSTOPTS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_TCLASS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_TRANSPARENT

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_UNICAST_HOPS

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_UNICAST_IF

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_V6ONLY

*Constant*: `c_int`



## libc::unix::linux_like::IPV6_XFRM_POLICY

*Constant*: `c_int`



## libc::unix::linux_like::IPVERSION

*Constant*: `u8`



## libc::unix::linux_like::IP_ADD_MEMBERSHIP

*Constant*: `c_int`



## libc::unix::linux_like::IP_ADD_SOURCE_MEMBERSHIP

*Constant*: `c_int`



## libc::unix::linux_like::IP_BIND_ADDRESS_NO_PORT

*Constant*: `c_int`



## libc::unix::linux_like::IP_BLOCK_SOURCE

*Constant*: `c_int`



## libc::unix::linux_like::IP_CHECKSUM

*Constant*: `c_int`



## libc::unix::linux_like::IP_DEFAULT_MULTICAST_LOOP

*Constant*: `c_int`



## libc::unix::linux_like::IP_DEFAULT_MULTICAST_TTL

*Constant*: `c_int`



## libc::unix::linux_like::IP_DROP_MEMBERSHIP

*Constant*: `c_int`



## libc::unix::linux_like::IP_DROP_SOURCE_MEMBERSHIP

*Constant*: `c_int`



## libc::unix::linux_like::IP_FREEBIND

*Constant*: `c_int`



## libc::unix::linux_like::IP_HDRINCL

*Constant*: `c_int`



## libc::unix::linux_like::IP_IPSEC_POLICY

*Constant*: `c_int`



## libc::unix::linux_like::IP_MINTTL

*Constant*: `c_int`



## libc::unix::linux_like::IP_MSFILTER

*Constant*: `c_int`



## libc::unix::linux_like::IP_MTU

*Constant*: `c_int`



## libc::unix::linux_like::IP_MTU_DISCOVER

*Constant*: `c_int`



## libc::unix::linux_like::IP_MULTICAST_ALL

*Constant*: `c_int`



## libc::unix::linux_like::IP_MULTICAST_IF

*Constant*: `c_int`



## libc::unix::linux_like::IP_MULTICAST_LOOP

*Constant*: `c_int`



## libc::unix::linux_like::IP_MULTICAST_TTL

*Constant*: `c_int`



## libc::unix::linux_like::IP_NODEFRAG

*Constant*: `c_int`



## libc::unix::linux_like::IP_OPTIONS

*Constant*: `c_int`



## libc::unix::linux_like::IP_ORIGDSTADDR

*Constant*: `c_int`



## libc::unix::linux_like::IP_PASSSEC

*Constant*: `c_int`



## libc::unix::linux_like::IP_PKTINFO

*Constant*: `c_int`



## libc::unix::linux_like::IP_PKTOPTIONS

*Constant*: `c_int`



## libc::unix::linux_like::IP_PMTUDISC_DO

*Constant*: `c_int`



## libc::unix::linux_like::IP_PMTUDISC_DONT

*Constant*: `c_int`



## libc::unix::linux_like::IP_PMTUDISC_INTERFACE

*Constant*: `c_int`



## libc::unix::linux_like::IP_PMTUDISC_OMIT

*Constant*: `c_int`



## libc::unix::linux_like::IP_PMTUDISC_PROBE

*Constant*: `c_int`



## libc::unix::linux_like::IP_PMTUDISC_WANT

*Constant*: `c_int`



## libc::unix::linux_like::IP_RECVERR

*Constant*: `c_int`



## libc::unix::linux_like::IP_RECVOPTS

*Constant*: `c_int`



## libc::unix::linux_like::IP_RECVORIGDSTADDR

*Constant*: `c_int`



## libc::unix::linux_like::IP_RECVTOS

*Constant*: `c_int`



## libc::unix::linux_like::IP_RECVTTL

*Constant*: `c_int`



## libc::unix::linux_like::IP_RETOPTS

*Constant*: `c_int`



## libc::unix::linux_like::IP_ROUTER_ALERT

*Constant*: `c_int`



## libc::unix::linux_like::IP_TOS

*Constant*: `c_int`



## libc::unix::linux_like::IP_TRANSPARENT

*Constant*: `c_int`



## libc::unix::linux_like::IP_TTL

*Constant*: `c_int`



## libc::unix::linux_like::IP_UNBLOCK_SOURCE

*Constant*: `c_int`



## libc::unix::linux_like::IP_UNICAST_IF

*Constant*: `c_int`



## libc::unix::linux_like::IP_XFRM_POLICY

*Constant*: `c_int`



## libc::unix::linux_like::ISOFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::ISTRIP

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::IXANY

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::JFFS2_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::KERNEL_VERSION

*Function*

```rust
fn KERNEL_VERSION(a: u32, b: u32, c: u32) -> u32
```



## libc::unix::linux_like::LC_ALL

*Constant*: `c_int`



## libc::unix::linux_like::LC_COLLATE

*Constant*: `c_int`



## libc::unix::linux_like::LC_COLLATE_MASK

*Constant*: `c_int`



## libc::unix::linux_like::LC_CTYPE

*Constant*: `c_int`



## libc::unix::linux_like::LC_CTYPE_MASK

*Constant*: `c_int`



## libc::unix::linux_like::LC_MESSAGES

*Constant*: `c_int`



## libc::unix::linux_like::LC_MESSAGES_MASK

*Constant*: `c_int`



## libc::unix::linux_like::LC_MONETARY

*Constant*: `c_int`



## libc::unix::linux_like::LC_MONETARY_MASK

*Constant*: `c_int`



## libc::unix::linux_like::LC_NUMERIC

*Constant*: `c_int`



## libc::unix::linux_like::LC_NUMERIC_MASK

*Constant*: `c_int`



## libc::unix::linux_like::LC_TIME

*Constant*: `c_int`



## libc::unix::linux_like::LC_TIME_MASK

*Constant*: `c_int`



## libc::unix::linux_like::LOCK_EX

*Constant*: `c_int`



## libc::unix::linux_like::LOCK_NB

*Constant*: `c_int`



## libc::unix::linux_like::LOCK_SH

*Constant*: `c_int`



## libc::unix::linux_like::LOCK_UN

*Constant*: `c_int`



## libc::unix::linux_like::LOG_AUTHPRIV

*Constant*: `c_int`



## libc::unix::linux_like::LOG_CRON

*Constant*: `c_int`



## libc::unix::linux_like::LOG_FTP

*Constant*: `c_int`



## libc::unix::linux_like::LOG_PERROR

*Constant*: `c_int`



## libc::unix::linux_like::MADV_COLD

*Constant*: `c_int`



## libc::unix::linux_like::MADV_DODUMP

*Constant*: `c_int`



## libc::unix::linux_like::MADV_DOFORK

*Constant*: `c_int`



## libc::unix::linux_like::MADV_DONTDUMP

*Constant*: `c_int`



## libc::unix::linux_like::MADV_DONTFORK

*Constant*: `c_int`



## libc::unix::linux_like::MADV_DONTNEED

*Constant*: `c_int`



## libc::unix::linux_like::MADV_DONTNEED_LOCKED

*Constant*: `c_int`



## libc::unix::linux_like::MADV_FREE

*Constant*: `c_int`



## libc::unix::linux_like::MADV_HUGEPAGE

*Constant*: `c_int`



## libc::unix::linux_like::MADV_HWPOISON

*Constant*: `c_int`



## libc::unix::linux_like::MADV_KEEPONFORK

*Constant*: `c_int`



## libc::unix::linux_like::MADV_MERGEABLE

*Constant*: `c_int`



## libc::unix::linux_like::MADV_NOHUGEPAGE

*Constant*: `c_int`



## libc::unix::linux_like::MADV_NORMAL

*Constant*: `c_int`



## libc::unix::linux_like::MADV_PAGEOUT

*Constant*: `c_int`



## libc::unix::linux_like::MADV_POPULATE_READ

*Constant*: `c_int`



## libc::unix::linux_like::MADV_POPULATE_WRITE

*Constant*: `c_int`



## libc::unix::linux_like::MADV_RANDOM

*Constant*: `c_int`



## libc::unix::linux_like::MADV_REMOVE

*Constant*: `c_int`



## libc::unix::linux_like::MADV_SEQUENTIAL

*Constant*: `c_int`



## libc::unix::linux_like::MADV_UNMERGEABLE

*Constant*: `c_int`



## libc::unix::linux_like::MADV_WILLNEED

*Constant*: `c_int`



## libc::unix::linux_like::MADV_WIPEONFORK

*Constant*: `c_int`



## libc::unix::linux_like::MAP_FAILED

*Constant*: `*mut c_void`



## libc::unix::linux_like::MAP_FILE

*Constant*: `c_int`



## libc::unix::linux_like::MAP_FIXED

*Constant*: `c_int`



## libc::unix::linux_like::MAP_PRIVATE

*Constant*: `c_int`



## libc::unix::linux_like::MAP_SHARED

*Constant*: `c_int`



## libc::unix::linux_like::MAP_TYPE

*Constant*: `c_int`



## libc::unix::linux_like::MAXTTL

*Constant*: `u8`



## libc::unix::linux_like::MAX_IPOPTLEN

*Constant*: `u8`



## libc::unix::linux_like::MCAST_BLOCK_SOURCE

*Constant*: `c_int`



## libc::unix::linux_like::MCAST_EXCLUDE

*Constant*: `c_int`



## libc::unix::linux_like::MCAST_INCLUDE

*Constant*: `c_int`



## libc::unix::linux_like::MCAST_JOIN_GROUP

*Constant*: `c_int`



## libc::unix::linux_like::MCAST_JOIN_SOURCE_GROUP

*Constant*: `c_int`



## libc::unix::linux_like::MCAST_LEAVE_GROUP

*Constant*: `c_int`



## libc::unix::linux_like::MCAST_LEAVE_SOURCE_GROUP

*Constant*: `c_int`



## libc::unix::linux_like::MCAST_MSFILTER

*Constant*: `c_int`



## libc::unix::linux_like::MCAST_UNBLOCK_SOURCE

*Constant*: `c_int`



## libc::unix::linux_like::MINIX2_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::MINIX2_SUPER_MAGIC2

*Constant*: `c_long`



## libc::unix::linux_like::MINIX3_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::MINIX_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::MINIX_SUPER_MAGIC2

*Constant*: `c_long`



## libc::unix::linux_like::MMAP_PAGE_ZERO

*Constant*: `c_int`



## libc::unix::linux_like::MNT_DETACH

*Constant*: `c_int`



## libc::unix::linux_like::MNT_EXPIRE

*Constant*: `c_int`



## libc::unix::linux_like::MNT_FORCE

*Constant*: `c_int`



## libc::unix::linux_like::MSDOS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::MSG_CMSG_CLOEXEC

*Constant*: `c_int`



## libc::unix::linux_like::MSG_CONFIRM

*Constant*: `c_int`



## libc::unix::linux_like::MSG_CTRUNC

*Constant*: `c_int`



## libc::unix::linux_like::MSG_DONTROUTE

*Constant*: `c_int`



## libc::unix::linux_like::MSG_DONTWAIT

*Constant*: `c_int`



## libc::unix::linux_like::MSG_EOR

*Constant*: `c_int`



## libc::unix::linux_like::MSG_ERRQUEUE

*Constant*: `c_int`



## libc::unix::linux_like::MSG_FASTOPEN

*Constant*: `c_int`



## libc::unix::linux_like::MSG_FIN

*Constant*: `c_int`



## libc::unix::linux_like::MSG_MORE

*Constant*: `c_int`



## libc::unix::linux_like::MSG_NOSIGNAL

*Constant*: `c_int`



## libc::unix::linux_like::MSG_OOB

*Constant*: `c_int`



## libc::unix::linux_like::MSG_PEEK

*Constant*: `c_int`



## libc::unix::linux_like::MSG_RST

*Constant*: `c_int`



## libc::unix::linux_like::MSG_SYN

*Constant*: `c_int`



## libc::unix::linux_like::MSG_TRUNC

*Constant*: `c_int`



## libc::unix::linux_like::MSG_WAITALL

*Constant*: `c_int`



## libc::unix::linux_like::MSG_WAITFORONE

*Constant*: `c_int`



## libc::unix::linux_like::MS_ACTIVE

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_ASYNC

*Constant*: `c_int`



## libc::unix::linux_like::MS_BIND

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_DIRSYNC

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_INVALIDATE

*Constant*: `c_int`



## libc::unix::linux_like::MS_I_VERSION

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_KERNMOUNT

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_LAZYTIME

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_MANDLOCK

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_MGC_MSK

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_MGC_VAL

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_MOVE

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_NOATIME

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_NODEV

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_NODIRATIME

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_NOEXEC

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_NOSUID

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_NOSYMFOLLOW

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_POSIXACL

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_PRIVATE

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_RDONLY

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_REC

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_RELATIME

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_REMOUNT

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_SHARED

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_SILENT

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_SLAVE

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_STRICTATIME

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_SYNC

*Constant*: `c_int`



## libc::unix::linux_like::MS_SYNCHRONOUS

*Constant*: `c_ulong`



## libc::unix::linux_like::MS_UNBINDABLE

*Constant*: `c_ulong`



## libc::unix::linux_like::NAME_MAX

*Constant*: `c_int`



## libc::unix::linux_like::NCP_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::NFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::NILFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::NL0

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::NL1

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::NSFS_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::OCFS2_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::OCRNL

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::OFDEL

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::OFILL

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::ONLRET

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::ONOCR

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::OPENPROM_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::OPOST

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::OVERLAYFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::O_RDONLY

*Constant*: `c_int`



## libc::unix::linux_like::O_RDWR

*Constant*: `c_int`



## libc::unix::linux_like::O_WRONLY

*Constant*: `c_int`



## libc::unix::linux_like::PARMRK

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::PATH_MAX

*Constant*: `c_int`



## libc::unix::linux_like::PF_ALG

*Constant*: `c_int`



## libc::unix::linux_like::PF_APPLETALK

*Constant*: `c_int`



## libc::unix::linux_like::PF_ASH

*Constant*: `c_int`



## libc::unix::linux_like::PF_ATMPVC

*Constant*: `c_int`



## libc::unix::linux_like::PF_ATMSVC

*Constant*: `c_int`



## libc::unix::linux_like::PF_AX25

*Constant*: `c_int`



## libc::unix::linux_like::PF_BLUETOOTH

*Constant*: `c_int`



## libc::unix::linux_like::PF_BRIDGE

*Constant*: `c_int`



## libc::unix::linux_like::PF_CAIF

*Constant*: `c_int`



## libc::unix::linux_like::PF_CAN

*Constant*: `c_int`



## libc::unix::linux_like::PF_DECnet

*Constant*: `c_int`



## libc::unix::linux_like::PF_ECONET

*Constant*: `c_int`



## libc::unix::linux_like::PF_IEEE802154

*Constant*: `c_int`



## libc::unix::linux_like::PF_INET

*Constant*: `c_int`



## libc::unix::linux_like::PF_INET6

*Constant*: `c_int`



## libc::unix::linux_like::PF_IPX

*Constant*: `c_int`



## libc::unix::linux_like::PF_IRDA

*Constant*: `c_int`



## libc::unix::linux_like::PF_ISDN

*Constant*: `c_int`



## libc::unix::linux_like::PF_IUCV

*Constant*: `c_int`



## libc::unix::linux_like::PF_KEY

*Constant*: `c_int`



## libc::unix::linux_like::PF_LLC

*Constant*: `c_int`



## libc::unix::linux_like::PF_LOCAL

*Constant*: `c_int`



## libc::unix::linux_like::PF_NETBEUI

*Constant*: `c_int`



## libc::unix::linux_like::PF_NETLINK

*Constant*: `c_int`



## libc::unix::linux_like::PF_NETROM

*Constant*: `c_int`



## libc::unix::linux_like::PF_PACKET

*Constant*: `c_int`



## libc::unix::linux_like::PF_PHONET

*Constant*: `c_int`



## libc::unix::linux_like::PF_PPPOX

*Constant*: `c_int`



## libc::unix::linux_like::PF_RDS

*Constant*: `c_int`



## libc::unix::linux_like::PF_ROSE

*Constant*: `c_int`



## libc::unix::linux_like::PF_ROUTE

*Constant*: `c_int`



## libc::unix::linux_like::PF_RXRPC

*Constant*: `c_int`



## libc::unix::linux_like::PF_SECURITY

*Constant*: `c_int`



## libc::unix::linux_like::PF_SNA

*Constant*: `c_int`



## libc::unix::linux_like::PF_TIPC

*Constant*: `c_int`



## libc::unix::linux_like::PF_UNIX

*Constant*: `c_int`



## libc::unix::linux_like::PF_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::PF_WANPIPE

*Constant*: `c_int`



## libc::unix::linux_like::PF_X25

*Constant*: `c_int`



## libc::unix::linux_like::PIPE_BUF

*Constant*: `usize`



## libc::unix::linux_like::POLLERR

*Constant*: `c_short`



## libc::unix::linux_like::POLLHUP

*Constant*: `c_short`



## libc::unix::linux_like::POLLIN

*Constant*: `c_short`



## libc::unix::linux_like::POLLNVAL

*Constant*: `c_short`



## libc::unix::linux_like::POLLOUT

*Constant*: `c_short`



## libc::unix::linux_like::POLLPRI

*Constant*: `c_short`



## libc::unix::linux_like::POLLRDBAND

*Constant*: `c_short`



## libc::unix::linux_like::POLLRDHUP

*Constant*: `c_short`



## libc::unix::linux_like::POLLRDNORM

*Constant*: `c_short`



## libc::unix::linux_like::POSIX_FADV_NORMAL

*Constant*: `c_int`



## libc::unix::linux_like::POSIX_FADV_RANDOM

*Constant*: `c_int`



## libc::unix::linux_like::POSIX_FADV_SEQUENTIAL

*Constant*: `c_int`



## libc::unix::linux_like::POSIX_FADV_WILLNEED

*Constant*: `c_int`



## libc::unix::linux_like::PROC_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::PROT_EXEC

*Constant*: `c_int`



## libc::unix::linux_like::PROT_GROWSDOWN

*Constant*: `c_int`



## libc::unix::linux_like::PROT_GROWSUP

*Constant*: `c_int`



## libc::unix::linux_like::PROT_NONE

*Constant*: `c_int`



## libc::unix::linux_like::PROT_READ

*Constant*: `c_int`



## libc::unix::linux_like::PROT_WRITE

*Constant*: `c_int`



## libc::unix::linux_like::PTHREAD_CREATE_DETACHED

*Constant*: `c_int`



## libc::unix::linux_like::PTHREAD_CREATE_JOINABLE

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_EVENT_CLONE

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_EVENT_EXEC

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_EVENT_EXIT

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_EVENT_FORK

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_EVENT_SECCOMP

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_EVENT_VFORK

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_EVENT_VFORK_DONE

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_EXITKILL

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_MASK

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_SUSPEND_SECCOMP

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_TRACECLONE

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_TRACEEXEC

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_TRACEEXIT

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_TRACEFORK

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_TRACESECCOMP

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_TRACESYSGOOD

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_TRACEVFORK

*Constant*: `c_int`



## libc::unix::linux_like::PTRACE_O_TRACEVFORKDONE

*Constant*: `c_int`



## libc::unix::linux_like::P_ALL

*Constant*: `idtype_t`



## libc::unix::linux_like::P_PGID

*Constant*: `idtype_t`



## libc::unix::linux_like::P_PID

*Constant*: `idtype_t`



## libc::unix::linux_like::P_PIDFD

*Constant*: `idtype_t`



## libc::unix::linux_like::QCMD

*Function*

```rust
fn QCMD(cmd: c_int, type_: c_int) -> c_int
```



## libc::unix::linux_like::QIF_ALL

*Constant*: `u32`



## libc::unix::linux_like::QIF_BLIMITS

*Constant*: `u32`



## libc::unix::linux_like::QIF_BTIME

*Constant*: `u32`



## libc::unix::linux_like::QIF_ILIMITS

*Constant*: `u32`



## libc::unix::linux_like::QIF_INODES

*Constant*: `u32`



## libc::unix::linux_like::QIF_ITIME

*Constant*: `u32`



## libc::unix::linux_like::QIF_LIMITS

*Constant*: `u32`



## libc::unix::linux_like::QIF_SPACE

*Constant*: `u32`



## libc::unix::linux_like::QIF_TIMES

*Constant*: `u32`



## libc::unix::linux_like::QIF_USAGE

*Constant*: `u32`



## libc::unix::linux_like::QNX4_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::QNX6_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::Q_GETFMT

*Constant*: `c_int`



## libc::unix::linux_like::Q_GETINFO

*Constant*: `c_int`



## libc::unix::linux_like::Q_GETQUOTA

*Constant*: `c_int`



## libc::unix::linux_like::Q_QUOTAOFF

*Constant*: `c_int`



## libc::unix::linux_like::Q_QUOTAON

*Constant*: `c_int`



## libc::unix::linux_like::Q_SETINFO

*Constant*: `c_int`



## libc::unix::linux_like::Q_SETQUOTA

*Constant*: `c_int`



## libc::unix::linux_like::Q_SYNC

*Constant*: `c_int`



## libc::unix::linux_like::RAND_MAX

*Constant*: `c_int`



## libc::unix::linux_like::RDTGROUP_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::READ_IMPLIES_EXEC

*Constant*: `c_int`



## libc::unix::linux_like::REISERFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::RLIM64_INFINITY

*Constant*: `crate::rlim64_t`



## libc::unix::linux_like::RTLD_LAZY

*Constant*: `c_int`



## libc::unix::linux_like::RTLD_LOCAL

*Constant*: `c_int`



## libc::unix::linux_like::RUSAGE_SELF

*Constant*: `c_int`



## libc::unix::linux_like::R_OK

*Constant*: `c_int`



## libc::unix::linux_like::SCM_CREDENTIALS

*Constant*: `c_int`



## libc::unix::linux_like::SCM_RIGHTS

*Constant*: `c_int`



## libc::unix::linux_like::SCM_TIMESTAMP

*Constant*: `c_int`



## libc::unix::linux_like::SECURITYFS_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::SEEK_CUR

*Constant*: `c_int`



## libc::unix::linux_like::SEEK_END

*Constant*: `c_int`



## libc::unix::linux_like::SEEK_SET

*Constant*: `c_int`



## libc::unix::linux_like::SELINUX_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::SHORT_INODE

*Constant*: `c_int`



## libc::unix::linux_like::SHUT_RD

*Constant*: `c_int`



## libc::unix::linux_like::SHUT_RDWR

*Constant*: `c_int`



## libc::unix::linux_like::SHUT_WR

*Constant*: `c_int`



## libc::unix::linux_like::SIGABRT

*Constant*: `c_int`



## libc::unix::linux_like::SIGALRM

*Constant*: `c_int`



## libc::unix::linux_like::SIGEV_NONE

*Constant*: `c_int`



## libc::unix::linux_like::SIGEV_SIGNAL

*Constant*: `c_int`



## libc::unix::linux_like::SIGEV_THREAD

*Constant*: `c_int`



## libc::unix::linux_like::SIGFPE

*Constant*: `c_int`



## libc::unix::linux_like::SIGHUP

*Constant*: `c_int`



## libc::unix::linux_like::SIGILL

*Constant*: `c_int`



## libc::unix::linux_like::SIGINT

*Constant*: `c_int`



## libc::unix::linux_like::SIGKILL

*Constant*: `c_int`



## libc::unix::linux_like::SIGPIPE

*Constant*: `c_int`



## libc::unix::linux_like::SIGQUIT

*Constant*: `c_int`



## libc::unix::linux_like::SIGRTMAX

*Function*

```rust
fn SIGRTMAX() -> c_int
```



## libc::unix::linux_like::SIGRTMIN

*Function*

```rust
fn SIGRTMIN() -> c_int
```



## libc::unix::linux_like::SIGSEGV

*Constant*: `c_int`



## libc::unix::linux_like::SIGTERM

*Constant*: `c_int`



## libc::unix::linux_like::SIGTRAP

*Constant*: `c_int`



## libc::unix::linux_like::SI_ASYNCIO

*Constant*: `c_int`



## libc::unix::linux_like::SI_ASYNCNL

*Constant*: `c_int`



## libc::unix::linux_like::SI_KERNEL

*Constant*: `c_int`



## libc::unix::linux_like::SI_LOAD_SHIFT

*Constant*: `c_uint`



## libc::unix::linux_like::SI_MESGQ

*Constant*: `c_int`



## libc::unix::linux_like::SI_QUEUE

*Constant*: `c_int`



## libc::unix::linux_like::SI_SIGIO

*Constant*: `c_int`



## libc::unix::linux_like::SI_TIMER

*Constant*: `c_int`



## libc::unix::linux_like::SI_TKILL

*Constant*: `c_int`



## libc::unix::linux_like::SI_USER

*Constant*: `c_int`



## libc::unix::linux_like::SMACK_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::SMB_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::SOCK_CLOEXEC

*Constant*: `c_int`



## libc::unix::linux_like::SOCK_RAW

*Constant*: `c_int`



## libc::unix::linux_like::SOCK_RDM

*Constant*: `c_int`



## libc::unix::linux_like::SOL_AAL

*Constant*: `c_int`



## libc::unix::linux_like::SOL_ALG

*Constant*: `c_int`



## libc::unix::linux_like::SOL_ATM

*Constant*: `c_int`



## libc::unix::linux_like::SOL_BLUETOOTH

*Constant*: `c_int`



## libc::unix::linux_like::SOL_DCCP

*Constant*: `c_int`



## libc::unix::linux_like::SOL_DECNET

*Constant*: `c_int`



## libc::unix::linux_like::SOL_ICMPV6

*Constant*: `c_int`



## libc::unix::linux_like::SOL_IP

*Constant*: `c_int`



## libc::unix::linux_like::SOL_IPV6

*Constant*: `c_int`



## libc::unix::linux_like::SOL_IRDA

*Constant*: `c_int`



## libc::unix::linux_like::SOL_LLC

*Constant*: `c_int`



## libc::unix::linux_like::SOL_NETBEUI

*Constant*: `c_int`



## libc::unix::linux_like::SOL_NETLINK

*Constant*: `c_int`



## libc::unix::linux_like::SOL_PACKET

*Constant*: `c_int`



## libc::unix::linux_like::SOL_RAW

*Constant*: `c_int`



## libc::unix::linux_like::SOL_TCP

*Constant*: `c_int`



## libc::unix::linux_like::SOL_TIPC

*Constant*: `c_int`



## libc::unix::linux_like::SOL_UDP

*Constant*: `c_int`



## libc::unix::linux_like::SOL_X25

*Constant*: `c_int`



## libc::unix::linux_like::SO_DEBUG

*Constant*: `c_int`



## libc::unix::linux_like::SPLICE_F_GIFT

*Constant*: `c_uint`



## libc::unix::linux_like::SPLICE_F_MORE

*Constant*: `c_uint`



## libc::unix::linux_like::SPLICE_F_MOVE

*Constant*: `c_uint`



## libc::unix::linux_like::SPLICE_F_NONBLOCK

*Constant*: `c_uint`



## libc::unix::linux_like::SS_DISABLE

*Constant*: `c_int`



## libc::unix::linux_like::SS_ONSTACK

*Constant*: `c_int`



## libc::unix::linux_like::STATX_ALL

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_ATIME

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_ATTR_APPEND

*Constant*: `c_int`



## libc::unix::linux_like::STATX_ATTR_AUTOMOUNT

*Constant*: `c_int`



## libc::unix::linux_like::STATX_ATTR_COMPRESSED

*Constant*: `c_int`



## libc::unix::linux_like::STATX_ATTR_DAX

*Constant*: `c_int`



## libc::unix::linux_like::STATX_ATTR_ENCRYPTED

*Constant*: `c_int`



## libc::unix::linux_like::STATX_ATTR_IMMUTABLE

*Constant*: `c_int`



## libc::unix::linux_like::STATX_ATTR_MOUNT_ROOT

*Constant*: `c_int`



## libc::unix::linux_like::STATX_ATTR_NODUMP

*Constant*: `c_int`



## libc::unix::linux_like::STATX_ATTR_VERITY

*Constant*: `c_int`



## libc::unix::linux_like::STATX_BASIC_STATS

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_BLOCKS

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_BTIME

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_CTIME

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_DIOALIGN

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_GID

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_INO

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_MNT_ID

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_MODE

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_MTIME

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_NLINK

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_SIZE

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_TYPE

*Constant*: `c_uint`



## libc::unix::linux_like::STATX_UID

*Constant*: `c_uint`



## libc::unix::linux_like::STATX__RESERVED

*Constant*: `c_int`



## libc::unix::linux_like::STICKY_TIMEOUTS

*Constant*: `c_int`



## libc::unix::linux_like::SYSFS_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::S_IFBLK

*Constant*: `mode_t`



## libc::unix::linux_like::S_IFCHR

*Constant*: `mode_t`



## libc::unix::linux_like::S_IFDIR

*Constant*: `mode_t`



## libc::unix::linux_like::S_IFIFO

*Constant*: `mode_t`



## libc::unix::linux_like::S_IFLNK

*Constant*: `mode_t`



## libc::unix::linux_like::S_IFMT

*Constant*: `mode_t`



## libc::unix::linux_like::S_IFREG

*Constant*: `mode_t`



## libc::unix::linux_like::S_IFSOCK

*Constant*: `mode_t`



## libc::unix::linux_like::S_IRGRP

*Constant*: `mode_t`



## libc::unix::linux_like::S_IROTH

*Constant*: `mode_t`



## libc::unix::linux_like::S_IRUSR

*Constant*: `mode_t`



## libc::unix::linux_like::S_IRWXG

*Constant*: `mode_t`



## libc::unix::linux_like::S_IRWXO

*Constant*: `mode_t`



## libc::unix::linux_like::S_IRWXU

*Constant*: `mode_t`



## libc::unix::linux_like::S_IWGRP

*Constant*: `mode_t`



## libc::unix::linux_like::S_IWOTH

*Constant*: `mode_t`



## libc::unix::linux_like::S_IWUSR

*Constant*: `mode_t`



## libc::unix::linux_like::S_IXGRP

*Constant*: `mode_t`



## libc::unix::linux_like::S_IXOTH

*Constant*: `mode_t`



## libc::unix::linux_like::S_IXUSR

*Constant*: `mode_t`



## libc::unix::linux_like::TAB0

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::TCIFLUSH

*Constant*: `c_int`



## libc::unix::linux_like::TCIOFF

*Constant*: `c_int`



## libc::unix::linux_like::TCIOFLUSH

*Constant*: `c_int`



## libc::unix::linux_like::TCION

*Constant*: `c_int`



## libc::unix::linux_like::TCOFLUSH

*Constant*: `c_int`



## libc::unix::linux_like::TCOOFF

*Constant*: `c_int`



## libc::unix::linux_like::TCOON

*Constant*: `c_int`



## libc::unix::linux_like::TCP_CC_INFO

*Constant*: `c_int`



## libc::unix::linux_like::TCP_CM_INQ

*Constant*: `c_int`



## libc::unix::linux_like::TCP_CONGESTION

*Constant*: `c_int`



## libc::unix::linux_like::TCP_COOKIE_TRANSACTIONS

*Constant*: `c_int`



## libc::unix::linux_like::TCP_CORK

*Constant*: `c_int`



## libc::unix::linux_like::TCP_DEFER_ACCEPT

*Constant*: `c_int`



## libc::unix::linux_like::TCP_FASTOPEN

*Constant*: `c_int`



## libc::unix::linux_like::TCP_FASTOPEN_CONNECT

*Constant*: `c_int`



## libc::unix::linux_like::TCP_FASTOPEN_KEY

*Constant*: `c_int`



## libc::unix::linux_like::TCP_FASTOPEN_NO_COOKIE

*Constant*: `c_int`



## libc::unix::linux_like::TCP_INFO

*Constant*: `c_int`



## libc::unix::linux_like::TCP_INQ

*Constant*: `c_int`



## libc::unix::linux_like::TCP_KEEPCNT

*Constant*: `c_int`



## libc::unix::linux_like::TCP_KEEPIDLE

*Constant*: `c_int`



## libc::unix::linux_like::TCP_KEEPINTVL

*Constant*: `c_int`



## libc::unix::linux_like::TCP_LINGER2

*Constant*: `c_int`



## libc::unix::linux_like::TCP_MAXSEG

*Constant*: `c_int`



## libc::unix::linux_like::TCP_MD5SIG

*Constant*: `c_int`



## libc::unix::linux_like::TCP_MD5SIG_EXT

*Constant*: `c_int`



## libc::unix::linux_like::TCP_MD5SIG_MAXKEYLEN

*Constant*: `usize`



## libc::unix::linux_like::TCP_NODELAY

*Constant*: `c_int`



## libc::unix::linux_like::TCP_NOTSENT_LOWAT

*Constant*: `c_int`



## libc::unix::linux_like::TCP_QUEUE_SEQ

*Constant*: `c_int`



## libc::unix::linux_like::TCP_QUICKACK

*Constant*: `c_int`



## libc::unix::linux_like::TCP_REPAIR

*Constant*: `c_int`



## libc::unix::linux_like::TCP_REPAIR_OPTIONS

*Constant*: `c_int`



## libc::unix::linux_like::TCP_REPAIR_QUEUE

*Constant*: `c_int`



## libc::unix::linux_like::TCP_REPAIR_WINDOW

*Constant*: `c_int`



## libc::unix::linux_like::TCP_SAVED_SYN

*Constant*: `c_int`



## libc::unix::linux_like::TCP_SAVE_SYN

*Constant*: `c_int`



## libc::unix::linux_like::TCP_SYNCNT

*Constant*: `c_int`



## libc::unix::linux_like::TCP_THIN_DUPACK

*Constant*: `c_int`



## libc::unix::linux_like::TCP_THIN_LINEAR_TIMEOUTS

*Constant*: `c_int`



## libc::unix::linux_like::TCP_TIMESTAMP

*Constant*: `c_int`



## libc::unix::linux_like::TCP_ULP

*Constant*: `c_int`



## libc::unix::linux_like::TCP_USER_TIMEOUT

*Constant*: `c_int`



## libc::unix::linux_like::TCP_WINDOW_CLAMP

*Constant*: `c_int`



## libc::unix::linux_like::TCP_ZEROCOPY_RECEIVE

*Constant*: `c_int`



## libc::unix::linux_like::TIMER_ABSTIME

*Constant*: `c_int`



## libc::unix::linux_like::TMPFS_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::TRACEFS_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::TRAP_BRANCH

*Constant*: `c_int`



## libc::unix::linux_like::TRAP_BRKPT

*Constant*: `c_int`



## libc::unix::linux_like::TRAP_HWBKPT

*Constant*: `c_int`



## libc::unix::linux_like::TRAP_TRACE

*Constant*: `c_int`



## libc::unix::linux_like::TRAP_UNK

*Constant*: `c_int`



## libc::unix::linux_like::TUNATTACHFILTER

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNDETACHFILTER

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNGETDEVNETNS

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNGETFEATURES

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNGETFILTER

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNGETIFF

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNGETSNDBUF

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNGETVNETBE

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNGETVNETHDRSZ

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNGETVNETLE

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETCARRIER

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETDEBUG

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETFILTEREBPF

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETGROUP

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETIFF

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETIFINDEX

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETLINK

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETNOCSUM

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETOFFLOAD

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETOWNER

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETPERSIST

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETQUEUE

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETSNDBUF

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETSTEERINGEBPF

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETTXFILTER

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETVNETBE

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETVNETHDRSZ

*Constant*: `c_ulong`



## libc::unix::linux_like::TUNSETVNETLE

*Constant*: `c_ulong`



## libc::unix::linux_like::TUN_FLT_ALLMULTI

*Constant*: `c_int`



## libc::unix::linux_like::TUN_F_CSUM

*Constant*: `c_uint`



## libc::unix::linux_like::TUN_F_TSO4

*Constant*: `c_uint`



## libc::unix::linux_like::TUN_F_TSO6

*Constant*: `c_uint`



## libc::unix::linux_like::TUN_F_TSO_ECN

*Constant*: `c_uint`



## libc::unix::linux_like::TUN_F_UFO

*Constant*: `c_uint`



## libc::unix::linux_like::TUN_F_USO4

*Constant*: `c_uint`



## libc::unix::linux_like::TUN_F_USO6

*Constant*: `c_uint`



## libc::unix::linux_like::TUN_PKT_STRIP

*Constant*: `c_int`



## libc::unix::linux_like::TUN_READQ_SIZE

*Constant*: `c_short`



## libc::unix::linux_like::TUN_TAP_DEV

*Constant*: `c_short`



## libc::unix::linux_like::TUN_TUN_DEV

*Constant*: `c_short`



## libc::unix::linux_like::TUN_TX_TIMESTAMP

*Constant*: `c_int`



## libc::unix::linux_like::TUN_TYPE_MASK

*Constant*: `c_short`



## libc::unix::linux_like::UDF_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::UIO_MAXIOV

*Constant*: `c_int`



## libc::unix::linux_like::UMOUNT_NOFOLLOW

*Constant*: `c_int`



## libc::unix::linux_like::USBDEVICE_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::UTIME_NOW

*Constant*: `c_long`



## libc::unix::linux_like::UTIME_OMIT

*Constant*: `c_long`



## libc::unix::linux_like::VERASE

*Constant*: `usize`



## libc::unix::linux_like::VINTR

*Constant*: `usize`



## libc::unix::linux_like::VKILL

*Constant*: `usize`



## libc::unix::linux_like::VLNEXT

*Constant*: `usize`



## libc::unix::linux_like::VQUIT

*Constant*: `usize`



## libc::unix::linux_like::VT0

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::WCONTINUED

*Constant*: `c_int`



## libc::unix::linux_like::WCOREDUMP

*Function*

```rust
fn WCOREDUMP(status: c_int) -> bool
```



## libc::unix::linux_like::WEXITED

*Constant*: `c_int`



## libc::unix::linux_like::WEXITSTATUS

*Function*

```rust
fn WEXITSTATUS(status: c_int) -> c_int
```



## libc::unix::linux_like::WHOLE_SECONDS

*Constant*: `c_int`



## libc::unix::linux_like::WIFCONTINUED

*Function*

```rust
fn WIFCONTINUED(status: c_int) -> bool
```



## libc::unix::linux_like::WIFEXITED

*Function*

```rust
fn WIFEXITED(status: c_int) -> bool
```



## libc::unix::linux_like::WIFSIGNALED

*Function*

```rust
fn WIFSIGNALED(status: c_int) -> bool
```



## libc::unix::linux_like::WIFSTOPPED

*Function*

```rust
fn WIFSTOPPED(status: c_int) -> bool
```



## libc::unix::linux_like::WNOHANG

*Constant*: `c_int`



## libc::unix::linux_like::WNOWAIT

*Constant*: `c_int`



## libc::unix::linux_like::WSTOPPED

*Constant*: `c_int`



## libc::unix::linux_like::WSTOPSIG

*Function*

```rust
fn WSTOPSIG(status: c_int) -> c_int
```



## libc::unix::linux_like::WTERMSIG

*Function*

```rust
fn WTERMSIG(status: c_int) -> c_int
```



## libc::unix::linux_like::WUNTRACED

*Constant*: `c_int`



## libc::unix::linux_like::W_EXITCODE

*Function*

```rust
fn W_EXITCODE(ret: c_int, sig: c_int) -> c_int
```



## libc::unix::linux_like::W_OK

*Constant*: `c_int`



## libc::unix::linux_like::W_STOPCODE

*Function*

```rust
fn W_STOPCODE(sig: c_int) -> c_int
```



## libc::unix::linux_like::XATTR_CREATE

*Constant*: `c_int`



## libc::unix::linux_like::XATTR_REPLACE

*Constant*: `c_int`



## libc::unix::linux_like::XENFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::X_OK

*Constant*: `c_int`



## libc::unix::linux_like::_IO

*Function*

Build an ioctl number for an argumentless ioctl.

```rust
fn _IO(ty: u32, nr: u32) -> c_ulong
```



## libc::unix::linux_like::_IOFBF

*Constant*: `c_int`



## libc::unix::linux_like::_IOLBF

*Constant*: `c_int`



## libc::unix::linux_like::_IONBF

*Constant*: `c_int`



## libc::unix::linux_like::_IOR

*Function*

Build an ioctl number for an read-only ioctl.

```rust
fn _IOR<T>(ty: u32, nr: u32) -> c_ulong
```



## libc::unix::linux_like::_IOW

*Function*

Build an ioctl number for an write-only ioctl.

```rust
fn _IOW<T>(ty: u32, nr: u32) -> c_ulong
```



## libc::unix::linux_like::_IOWR

*Function*

Build an ioctl number for a read-write ioctl.

```rust
fn _IOWR<T>(ty: u32, nr: u32) -> c_ulong
```



## libc::unix::linux_like::__WALL

*Constant*: `c_int`



## libc::unix::linux_like::__WCLONE

*Constant*: `c_int`



## libc::unix::linux_like::__WNOTHREAD

*Constant*: `c_int`



## libc::unix::linux_like::acct

*Function*

```rust
fn acct(filename: *const c_char) -> c_int
```



## libc::unix::linux_like::addrinfo

*Struct*

**Fields:**
- `ai_flags: c_int`
- `ai_family: c_int`
- `ai_socktype: c_int`
- `ai_protocol: c_int`
- `ai_addrlen: socklen_t`
- `ai_addr: *mut crate::sockaddr`
- `ai_canonname: *mut c_char`
- `ai_next: *mut addrinfo`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> addrinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::arphdr

*Struct*

**Fields:**
- `ar_hrd: u16`
- `ar_pro: u16`
- `ar_hln: u8`
- `ar_pln: u8`
- `ar_op: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> arphdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::arpreq

*Struct*

**Fields:**
- `arp_pa: crate::sockaddr`
- `arp_ha: crate::sockaddr`
- `arp_flags: c_int`
- `arp_netmask: crate::sockaddr`
- `arp_dev: [c_char; 16]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> arpreq`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::arpreq_old

*Struct*

**Fields:**
- `arp_pa: crate::sockaddr`
- `arp_ha: crate::sockaddr`
- `arp_flags: c_int`
- `arp_netmask: crate::sockaddr`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> arpreq_old`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::bind

*Function*

```rust
fn bind(socket: c_int, address: *const crate::sockaddr, address_len: crate::socklen_t) -> c_int
```



## libc::unix::linux_like::brk

*Function*

```rust
fn brk(addr: *mut c_void) -> c_int
```



## libc::unix::linux_like::clearenv

*Function*

```rust
fn clearenv() -> c_int
```



## libc::unix::linux_like::clock_getcpuclockid

*Function*

```rust
fn clock_getcpuclockid(pid: crate::pid_t, clk_id: *mut crate::clockid_t) -> c_int
```



## libc::unix::linux_like::clock_getres

*Function*

```rust
fn clock_getres(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int
```



## libc::unix::linux_like::clock_gettime

*Function*

```rust
fn clock_gettime(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int
```



## libc::unix::linux_like::clock_settime

*Function*

```rust
fn clock_settime(clk_id: crate::clockid_t, tp: *const crate::timespec) -> c_int
```



## libc::unix::linux_like::clockid_t

*Type Alias*: `c_int`



## libc::unix::linux_like::creat64

*Function*

```rust
fn creat64(path: *const c_char, mode: mode_t) -> c_int
```



## libc::unix::linux_like::dirfd

*Function*

```rust
fn dirfd(dirp: *mut crate::DIR) -> c_int
```



## libc::unix::linux_like::duplocale

*Function*

```rust
fn duplocale(base: crate::locale_t) -> crate::locale_t
```



## libc::unix::linux_like::epoll_event

*Struct*

**Fields:**
- `events: u32`
- `u64: u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> epoll_event`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::execvpe

*Function*

```rust
fn execvpe(file: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```



## libc::unix::linux_like::fd_set

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fd_set`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::fdatasync

*Function*

```rust
fn fdatasync(fd: c_int) -> c_int
```



## libc::unix::linux_like::fexecve

*Function*

```rust
fn fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```



## libc::unix::linux_like::file_clone_range

*Struct*

**Fields:**
- `src_fd: crate::__s64`
- `src_offset: crate::__u64`
- `src_length: crate::__u64`
- `dest_offset: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> file_clone_range`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::forkpty

*Function*

```rust
fn forkpty(amaster: *mut c_int, name: *mut c_char, termp: *const termios, winp: *const crate::winsize) -> crate::pid_t
```



## libc::unix::linux_like::freeifaddrs

*Function*

```rust
fn freeifaddrs(ifa: *mut crate::ifaddrs)
```



## libc::unix::linux_like::freelocale

*Function*

```rust
fn freelocale(loc: crate::locale_t)
```



## libc::unix::linux_like::fstat64

*Function*

```rust
fn fstat64(fildes: c_int, buf: *mut stat64) -> c_int
```



## libc::unix::linux_like::fstatat64

*Function*

```rust
fn fstatat64(dirfd: c_int, pathname: *const c_char, buf: *mut stat64, flags: c_int) -> c_int
```



## libc::unix::linux_like::fstatfs

*Function*

```rust
fn fstatfs(fd: c_int, buf: *mut statfs) -> c_int
```



## libc::unix::linux_like::fstatfs64

*Function*

```rust
fn fstatfs64(fd: c_int, buf: *mut statfs64) -> c_int
```



## libc::unix::linux_like::fstatvfs64

*Function*

```rust
fn fstatvfs64(fd: c_int, buf: *mut statvfs64) -> c_int
```



## libc::unix::linux_like::ftruncate64

*Function*

```rust
fn ftruncate64(fd: c_int, length: off64_t) -> c_int
```



## libc::unix::linux_like::futimens

*Function*

```rust
fn futimens(fd: c_int, times: *const crate::timespec) -> c_int
```



## libc::unix::linux_like::getdomainname

*Function*

```rust
fn getdomainname(name: *mut c_char, len: size_t) -> c_int
```



## libc::unix::linux_like::getifaddrs

*Function*

```rust
fn getifaddrs(ifap: *mut *mut crate::ifaddrs) -> c_int
```



## libc::unix::linux_like::getitimer

*Function*

```rust
fn getitimer(which: c_int, curr_value: *mut crate::itimerval) -> c_int
```



## libc::unix::linux_like::getpwnam_r

*Function*

```rust
fn getpwnam_r(name: *const c_char, pwd: *mut passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut passwd) -> c_int
```



## libc::unix::linux_like::getpwuid_r

*Function*

```rust
fn getpwuid_r(uid: crate::uid_t, pwd: *mut passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut passwd) -> c_int
```



## libc::unix::linux_like::getresgid

*Function*

```rust
fn getresgid(rgid: *mut crate::gid_t, egid: *mut crate::gid_t, sgid: *mut crate::gid_t) -> c_int
```



## libc::unix::linux_like::getresuid

*Function*

```rust
fn getresuid(ruid: *mut crate::uid_t, euid: *mut crate::uid_t, suid: *mut crate::uid_t) -> c_int
```



## libc::unix::linux_like::id_t

*Type Alias*: `c_uint`



## libc::unix::linux_like::if_freenameindex

*Function*

```rust
fn if_freenameindex(ptr: *mut if_nameindex)
```



## libc::unix::linux_like::if_nameindex

*Function*

```rust
fn if_nameindex() -> *mut if_nameindex
```



## libc::unix::linux_like::if_nameindex

*Struct*

**Fields:**
- `if_index: c_uint`
- `if_name: *mut c_char`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> if_nameindex`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::ifaddrs

*Struct*

**Fields:**
- `ifa_next: *mut ifaddrs`
- `ifa_name: *mut c_char`
- `ifa_flags: c_uint`
- `ifa_addr: *mut crate::sockaddr`
- `ifa_netmask: *mut crate::sockaddr`
- `ifa_ifu: *mut crate::sockaddr`
- `ifa_data: *mut c_void`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ifaddrs`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::in6_rtmsg

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> in6_rtmsg`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::in_addr

*Struct*

**Fields:**
- `s_addr: crate::in_addr_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> in_addr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::in_pktinfo

*Struct*

**Fields:**
- `ipi_ifindex: c_int`
- `ipi_spec_dst: crate::in_addr`
- `ipi_addr: crate::in_addr`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> in_pktinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::ioctl

*Function*

```rust
fn ioctl(fd: c_int, request: c_ulong) -> c_int
```



## libc::unix::linux_like::ip_mreq

*Struct*

**Fields:**
- `imr_multiaddr: in_addr`
- `imr_interface: in_addr`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ip_mreq`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::ip_mreq_source

*Struct*

**Fields:**
- `imr_multiaddr: in_addr`
- `imr_interface: in_addr`
- `imr_sourceaddr: in_addr`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ip_mreq_source`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::ip_mreqn

*Struct*

**Fields:**
- `imr_multiaddr: in_addr`
- `imr_address: in_addr`
- `imr_ifindex: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ip_mreqn`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::key_t

*Type Alias*: `c_int`



## libc::unix::linux_like::lconv

*Struct*

**Fields:**
- `decimal_point: *mut c_char`
- `thousands_sep: *mut c_char`
- `grouping: *mut c_char`
- `int_curr_symbol: *mut c_char`
- `currency_symbol: *mut c_char`
- `mon_decimal_point: *mut c_char`
- `mon_thousands_sep: *mut c_char`
- `mon_grouping: *mut c_char`
- `positive_sign: *mut c_char`
- `negative_sign: *mut c_char`
- `int_frac_digits: c_char`
- `frac_digits: c_char`
- `p_cs_precedes: c_char`
- `p_sep_by_space: c_char`
- `n_cs_precedes: c_char`
- `n_sep_by_space: c_char`
- `p_sign_posn: c_char`
- `n_sign_posn: c_char`
- `int_p_cs_precedes: c_char`
- `int_p_sep_by_space: c_char`
- `int_n_cs_precedes: c_char`
- `int_n_sep_by_space: c_char`
- `int_p_sign_posn: c_char`
- `int_n_sign_posn: c_char`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> lconv`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::login_tty

*Function*

```rust
fn login_tty(fd: c_int) -> c_int
```



## libc::unix::linux_like::lseek64

*Function*

```rust
fn lseek64(fd: c_int, offset: off64_t, whence: c_int) -> off64_t
```



## libc::unix::linux_like::lstat64

*Function*

```rust
fn lstat64(path: *const c_char, buf: *mut stat64) -> c_int
```



## libc::unix::linux_like::memalign

*Function*

```rust
fn memalign(align: size_t, size: size_t) -> *mut c_void
```



## libc::unix::linux_like::memrchr

*Function*

```rust
fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void
```



## libc::unix::linux_like::mincore

*Function*

```rust
fn mincore(addr: *mut c_void, len: size_t, vec: *mut c_uchar) -> c_int
```



## libc::unix::linux_like::mknodat

*Function*

```rust
fn mknodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, dev: dev_t) -> c_int
```



## libc::unix::linux_like::mkostemp

*Function*

```rust
fn mkostemp(template: *mut c_char, flags: c_int) -> c_int
```



## libc::unix::linux_like::mkostemps

*Function*

```rust
fn mkostemps(template: *mut c_char, suffixlen: c_int, flags: c_int) -> c_int
```



## libc::unix::linux_like::mmap64

*Function*

```rust
fn mmap64(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off64_t) -> *mut c_void
```



## libc::unix::linux_like::mmsghdr

*Struct*

**Fields:**
- `msg_hdr: crate::msghdr`
- `msg_len: c_uint`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> mmsghdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::newlocale

*Function*

```rust
fn newlocale(mask: c_int, locale: *const c_char, base: crate::locale_t) -> crate::locale_t
```



## libc::unix::linux_like::open64

*Function*

```rust
fn open64(path: *const c_char, oflag: c_int) -> c_int
```



## libc::unix::linux_like::openat64

*Function*

```rust
fn openat64(fd: c_int, path: *const c_char, oflag: c_int) -> c_int
```



## libc::unix::linux_like::openpty

*Function*

```rust
fn openpty(amaster: *mut c_int, aslave: *mut c_int, name: *mut c_char, termp: *const termios, winp: *const crate::winsize) -> c_int
```



## libc::unix::linux_like::pipe2

*Function*

```rust
fn pipe2(fds: *mut c_int, flags: c_int) -> c_int
```



## libc::unix::linux_like::posix_fadvise

*Function*

```rust
fn posix_fadvise(fd: c_int, offset: off_t, len: off_t, advise: c_int) -> c_int
```



## libc::unix::linux_like::posix_fadvise64

*Function*

```rust
fn posix_fadvise64(fd: c_int, offset: off64_t, len: off64_t, advise: c_int) -> c_int
```



## libc::unix::linux_like::pread64

*Function*

```rust
fn pread64(fd: c_int, buf: *mut c_void, count: size_t, offset: off64_t) -> ssize_t
```



## libc::unix::linux_like::preadv64

*Function*

```rust
fn preadv64(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t) -> ssize_t
```



## libc::unix::linux_like::ptsname_r

*Function*

```rust
fn ptsname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```



## libc::unix::linux_like::pwrite64

*Function*

```rust
fn pwrite64(fd: c_int, buf: *const c_void, count: size_t, offset: off64_t) -> ssize_t
```



## libc::unix::linux_like::pwritev64

*Function*

```rust
fn pwritev64(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t) -> ssize_t
```



## libc::unix::linux_like::readdir64

*Function*

```rust
fn readdir64(dirp: *mut crate::DIR) -> *mut crate::dirent64
```



## libc::unix::linux_like::readdir64_r

*Function*

```rust
fn readdir64_r(dirp: *mut crate::DIR, entry: *mut crate::dirent64, result: *mut *mut crate::dirent64) -> c_int
```



## libc::unix::linux_like::readv

*Function*

```rust
fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t
```



## libc::unix::linux_like::recvmsg

*Function*

```rust
fn recvmsg(fd: c_int, msg: *mut crate::msghdr, flags: c_int) -> ssize_t
```



## libc::unix::linux_like::sa_family_t

*Type Alias*: `u16`



## libc::unix::linux_like::sbrk

*Function*

```rust
fn sbrk(increment: intptr_t) -> *mut c_void
```



## libc::unix::linux_like::sched_param

*Struct*

**Fields:**
- `sched_priority: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sched_param`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::sem_destroy

*Function*

```rust
fn sem_destroy(sem: *mut sem_t) -> c_int
```



## libc::unix::linux_like::sem_init

*Function*

```rust
fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int
```



## libc::unix::linux_like::sendmsg

*Function*

```rust
fn sendmsg(fd: c_int, msg: *const crate::msghdr, flags: c_int) -> ssize_t
```



## libc::unix::linux_like::setdomainname

*Function*

```rust
fn setdomainname(name: *const c_char, len: size_t) -> c_int
```



## libc::unix::linux_like::setgroups

*Function*

```rust
fn setgroups(ngroups: size_t, ptr: *const crate::gid_t) -> c_int
```



## libc::unix::linux_like::setitimer

*Function*

```rust
fn setitimer(which: c_int, new_value: *const crate::itimerval, old_value: *mut crate::itimerval) -> c_int
```



## libc::unix::linux_like::setresgid

*Function*

```rust
fn setresgid(rgid: crate::gid_t, egid: crate::gid_t, sgid: crate::gid_t) -> c_int
```



## libc::unix::linux_like::setresuid

*Function*

```rust
fn setresuid(ruid: crate::uid_t, euid: crate::uid_t, suid: crate::uid_t) -> c_int
```



## libc::unix::linux_like::sigevent

*Struct*

**Fields:**
- `sigev_value: crate::sigval`
- `sigev_signo: c_int`
- `sigev_notify: c_int`
- `sigev_notify_thread_id: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sigevent`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::sock_filter

*Struct*

**Fields:**
- `code: __u16`
- `jt: __u8`
- `jf: __u8`
- `k: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sock_filter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::sock_fprog

*Struct*

**Fields:**
- `len: c_ushort`
- `filter: *mut sock_filter`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sock_fprog`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::sockaddr

*Struct*

**Fields:**
- `sa_family: sa_family_t`
- `sa_data: [c_char; 14]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::sockaddr_in

*Struct*

**Fields:**
- `sin_family: sa_family_t`
- `sin_port: crate::in_port_t`
- `sin_addr: crate::in_addr`
- `sin_zero: [u8; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_in`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::sockaddr_in6

*Struct*

**Fields:**
- `sin6_family: sa_family_t`
- `sin6_port: crate::in_port_t`
- `sin6_flowinfo: u32`
- `sin6_addr: crate::in6_addr`
- `sin6_scope_id: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_in6`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::sockaddr_ll

*Struct*

**Fields:**
- `sll_family: c_ushort`
- `sll_protocol: c_ushort`
- `sll_ifindex: c_int`
- `sll_hatype: c_ushort`
- `sll_pkttype: c_uchar`
- `sll_halen: c_uchar`
- `sll_addr: [c_uchar; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_ll`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::sockaddr_storage

*Struct*

**Fields:**
- `ss_family: sa_family_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_storage`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::sockaddr_un

*Struct*

**Fields:**
- `sun_family: sa_family_t`
- `sun_path: [c_char; 108]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_un`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::speed_t

*Type Alias*: `c_uint`



## libc::unix::linux_like::stat64

*Function*

```rust
fn stat64(path: *const c_char, buf: *mut stat64) -> c_int
```



## libc::unix::linux_like::statfs

*Function*

```rust
fn statfs(path: *const c_char, buf: *mut statfs) -> c_int
```



## libc::unix::linux_like::statfs64

*Function*

```rust
fn statfs64(path: *const c_char, buf: *mut statfs64) -> c_int
```



## libc::unix::linux_like::statvfs64

*Function*

```rust
fn statvfs64(path: *const c_char, buf: *mut statvfs64) -> c_int
```



## libc::unix::linux_like::statx

*Struct*

**Fields:**
- `stx_mask: crate::__u32`
- `stx_blksize: crate::__u32`
- `stx_attributes: crate::__u64`
- `stx_nlink: crate::__u32`
- `stx_uid: crate::__u32`
- `stx_gid: crate::__u32`
- `stx_mode: crate::__u16`
- `stx_ino: crate::__u64`
- `stx_size: crate::__u64`
- `stx_blocks: crate::__u64`
- `stx_attributes_mask: crate::__u64`
- `stx_atime: statx_timestamp`
- `stx_btime: statx_timestamp`
- `stx_ctime: statx_timestamp`
- `stx_mtime: statx_timestamp`
- `stx_rdev_major: crate::__u32`
- `stx_rdev_minor: crate::__u32`
- `stx_dev_major: crate::__u32`
- `stx_dev_minor: crate::__u32`
- `stx_mnt_id: crate::__u64`
- `stx_dio_mem_align: crate::__u32`
- `stx_dio_offset_align: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> statx`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::statx

*Function*

```rust
fn statx(dirfd: c_int, pathname: *const c_char, flags: c_int, mask: c_uint, statxbuf: *mut statx) -> c_int
```



## libc::unix::linux_like::statx_timestamp

*Struct*

**Fields:**
- `tv_sec: crate::__s64`
- `tv_nsec: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> statx_timestamp`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::strchrnul

*Function*

```rust
fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char
```



## libc::unix::linux_like::strftime

*Function*

```rust
fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const crate::tm) -> size_t
```



## libc::unix::linux_like::strftime_l

*Function*

```rust
fn strftime_l(s: *mut c_char, max: size_t, format: *const c_char, tm: *const crate::tm, locale: crate::locale_t) -> size_t
```



## libc::unix::linux_like::strptime

*Function*

```rust
fn strptime(s: *const c_char, format: *const c_char, tm: *mut crate::tm) -> *mut c_char
```



## libc::unix::linux_like::tcflag_t

*Type Alias*: `c_uint`



## libc::unix::linux_like::timer_t

*Type Alias*: `*mut c_void`



## libc::unix::linux_like::timezone

*Enum*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> timezone`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::tm

*Struct*

**Fields:**
- `tm_sec: c_int`
- `tm_min: c_int`
- `tm_hour: c_int`
- `tm_mday: c_int`
- `tm_mon: c_int`
- `tm_year: c_int`
- `tm_wday: c_int`
- `tm_yday: c_int`
- `tm_isdst: c_int`
- `tm_gmtoff: c_long`
- `tm_zone: *const c_char`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tm`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::truncate64

*Function*

```rust
fn truncate64(path: *const c_char, length: off64_t) -> c_int
```



## libc::unix::linux_like::uname

*Function*

```rust
fn uname(buf: *mut crate::utsname) -> c_int
```



## libc::unix::linux_like::useconds_t

*Type Alias*: `u32`



## libc::unix::linux_like::uselocale

*Function*

```rust
fn uselocale(loc: crate::locale_t) -> crate::locale_t
```



## libc::unix::linux_like::utimensat

*Function*

```rust
fn utimensat(dirfd: c_int, path: *const c_char, times: *const crate::timespec, flag: c_int) -> c_int
```



## libc::unix::linux_like::utsname

*Struct*

**Fields:**
- `sysname: [c_char; 65]`
- `nodename: [c_char; 65]`
- `release: [c_char; 65]`
- `version: [c_char; 65]`
- `machine: [c_char; 65]`
- `domainname: [c_char; 65]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> utsname`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::vfork

*Function*

```rust
fn vfork() -> crate::pid_t
```



## libc::unix::linux_like::wait4

*Function*

```rust
fn wait4(pid: crate::pid_t, status: *mut c_int, options: c_int, rusage: *mut crate::rusage) -> crate::pid_t
```



## libc::unix::linux_like::waitid

*Function*

```rust
fn waitid(idtype: idtype_t, id: id_t, infop: *mut crate::siginfo_t, options: c_int) -> c_int
```



## libc::unix::linux_like::writev

*Function*

```rust
fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t
```



