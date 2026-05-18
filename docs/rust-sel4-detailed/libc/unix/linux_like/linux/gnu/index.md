*[libc](../../../../index.md) / [unix](../../../index.md) / [linux_like](../../index.md) / [linux](../index.md) / [gnu](index.md)*

---

# Module `gnu`

## Contents

- [Modules](#modules)
  - [`b64`](#b64)
  - [`x86_64`](#x86-64)
- [Structs](#structs)
  - [`aiocb`](#aiocb)
  - [`__exit_status`](#exit-status)
  - [`__timeval`](#timeval)
  - [`glob64_t`](#glob64-t)
  - [`msghdr`](#msghdr)
  - [`cmsghdr`](#cmsghdr)
  - [`termios`](#termios)
  - [`mallinfo`](#mallinfo)
  - [`mallinfo2`](#mallinfo2)
  - [`ntptimeval`](#ntptimeval)
  - [`regex_t`](#regex-t)
  - [`Elf64_Chdr`](#elf64-chdr)
  - [`Elf32_Chdr`](#elf32-chdr)
  - [`seminfo`](#seminfo)
  - [`ptrace_peeksiginfo_args`](#ptrace-peeksiginfo-args)
  - [`__c_anonymous_ptrace_syscall_info_entry`](#c-anonymous-ptrace-syscall-info-entry)
  - [`__c_anonymous_ptrace_syscall_info_exit`](#c-anonymous-ptrace-syscall-info-exit)
  - [`__c_anonymous_ptrace_syscall_info_seccomp`](#c-anonymous-ptrace-syscall-info-seccomp)
  - [`ptrace_syscall_info`](#ptrace-syscall-info)
  - [`ptrace_sud_config`](#ptrace-sud-config)
  - [`iocb`](#iocb)
  - [`tcp_info`](#tcp-info)
  - [`fanotify_event_info_pidfd`](#fanotify-event-info-pidfd)
  - [`fanotify_event_info_error`](#fanotify-event-info-error)
  - [`sem_t`](#sem-t)
  - [`mbstate_t`](#mbstate-t)
  - [`fpos64_t`](#fpos64-t)
  - [`fpos_t`](#fpos-t)
  - [`timespec`](#timespec)
  - [`utmpx`](#utmpx)
  - [`sifields_sigchld`](#sifields-sigchld)
  - [`siginfo_f`](#siginfo-f)
  - [`sigset_t`](#sigset-t)
  - [`sysinfo`](#sysinfo)
  - [`msqid_ds`](#msqid-ds)
  - [`semid_ds`](#semid-ds)
  - [`timex`](#timex)
- [Functions](#functions)
  - [`fgetspent_r`](#fgetspent-r)
  - [`sgetspent_r`](#sgetspent-r)
  - [`getspent_r`](#getspent-r)
  - [`qsort_r`](#qsort-r)
  - [`sendmmsg`](#sendmmsg)
  - [`recvmmsg`](#recvmmsg)
  - [`getrlimit64`](#getrlimit64)
  - [`setrlimit64`](#setrlimit64)
  - [`getrlimit`](#getrlimit)
  - [`setrlimit`](#setrlimit)
  - [`prlimit`](#prlimit)
  - [`prlimit64`](#prlimit64)
  - [`utmpname`](#utmpname)
  - [`utmpxname`](#utmpxname)
  - [`getutxent`](#getutxent)
  - [`getutxid`](#getutxid)
  - [`getutxline`](#getutxline)
  - [`pututxline`](#pututxline)
  - [`setutxent`](#setutxent)
  - [`endutxent`](#endutxent)
  - [`getpt`](#getpt)
  - [`mallopt`](#mallopt)
  - [`gettimeofday`](#gettimeofday)
  - [`getentropy`](#getentropy)
  - [`getrandom`](#getrandom)
  - [`getauxval`](#getauxval)
  - [`adjtimex`](#adjtimex)
  - [`ntp_adjtime`](#ntp-adjtime)
  - [`ntp_gettime`](#ntp-gettime)
  - [`clock_adjtime`](#clock-adjtime)
  - [`fanotify_mark`](#fanotify-mark)
  - [`preadv2`](#preadv2)
  - [`pwritev2`](#pwritev2)
  - [`preadv64v2`](#preadv64v2)
  - [`pwritev64v2`](#pwritev64v2)
  - [`renameat2`](#renameat2)
  - [`explicit_bzero`](#explicit-bzero)
  - [`reallocarray`](#reallocarray)
  - [`ctermid`](#ctermid)
  - [`backtrace`](#backtrace)
  - [`backtrace_symbols`](#backtrace-symbols)
  - [`backtrace_symbols_fd`](#backtrace-symbols-fd)
  - [`glob64`](#glob64)
  - [`globfree64`](#globfree64)
  - [`ptrace`](#ptrace)
  - [`pthread_attr_getaffinity_np`](#pthread-attr-getaffinity-np)
  - [`pthread_attr_setaffinity_np`](#pthread-attr-setaffinity-np)
  - [`getpriority`](#getpriority)
  - [`setpriority`](#setpriority)
  - [`pthread_rwlockattr_getkind_np`](#pthread-rwlockattr-getkind-np)
  - [`pthread_rwlockattr_setkind_np`](#pthread-rwlockattr-setkind-np)
  - [`pthread_sigqueue`](#pthread-sigqueue)
  - [`pthread_tryjoin_np`](#pthread-tryjoin-np)
  - [`pthread_timedjoin_np`](#pthread-timedjoin-np)
  - [`mallinfo`](#mallinfo)
  - [`mallinfo2`](#mallinfo2)
  - [`malloc_stats`](#malloc-stats)
  - [`malloc_info`](#malloc-info)
  - [`malloc_usable_size`](#malloc-usable-size)
  - [`getpwent_r`](#getpwent-r)
  - [`getgrent_r`](#getgrent-r)
  - [`fgetpwent_r`](#fgetpwent-r)
  - [`fgetgrent_r`](#fgetgrent-r)
  - [`putpwent`](#putpwent)
  - [`putgrent`](#putgrent)
  - [`sethostid`](#sethostid)
  - [`memfd_create`](#memfd-create)
  - [`mlock2`](#mlock2)
  - [`euidaccess`](#euidaccess)
  - [`eaccess`](#eaccess)
  - [`asctime_r`](#asctime-r)
  - [`ctime_r`](#ctime-r)
  - [`dirname`](#dirname)
  - [`posix_basename`](#posix-basename)
  - [`gnu_basename`](#gnu-basename)
  - [`dlmopen`](#dlmopen)
  - [`dlinfo`](#dlinfo)
  - [`dladdr1`](#dladdr1)
  - [`dlvsym`](#dlvsym)
  - [`malloc_trim`](#malloc-trim)
  - [`gnu_get_libc_release`](#gnu-get-libc-release)
  - [`gnu_get_libc_version`](#gnu-get-libc-version)
  - [`posix_spawn_file_actions_addchdir_np`](#posix-spawn-file-actions-addchdir-np)
  - [`posix_spawn_file_actions_addfchdir_np`](#posix-spawn-file-actions-addfchdir-np)
  - [`posix_spawn_file_actions_addclosefrom_np`](#posix-spawn-file-actions-addclosefrom-np)
  - [`posix_spawn_file_actions_addtcsetpgrp_np`](#posix-spawn-file-actions-addtcsetpgrp-np)
  - [`getmntent_r`](#getmntent-r)
  - [`execveat`](#execveat)
  - [`close_range`](#close-range)
  - [`mq_notify`](#mq-notify)
  - [`epoll_pwait2`](#epoll-pwait2)
  - [`mempcpy`](#mempcpy)
  - [`tgkill`](#tgkill)
- [Type Aliases](#type-aliases)
  - [`pthread_t`](#pthread-t)
  - [`__priority_which_t`](#priority-which-t)
  - [`__rlimit_resource_t`](#rlimit-resource-t)
  - [`Lmid_t`](#lmid-t)
  - [`regoff_t`](#regoff-t)
  - [`__kernel_rwf_t`](#kernel-rwf-t)
  - [`Ioctl`](#ioctl)
  - [`ino_t`](#ino-t)
  - [`off_t`](#off-t)
  - [`blkcnt_t`](#blkcnt-t)
  - [`shmatt_t`](#shmatt-t)
  - [`msgqnum_t`](#msgqnum-t)
  - [`msglen_t`](#msglen-t)
  - [`fsblkcnt_t`](#fsblkcnt-t)
  - [`fsfilcnt_t`](#fsfilcnt-t)
  - [`rlim_t`](#rlim-t)
  - [`__syscall_ulong_t`](#syscall-ulong-t)
  - [`__fsword_t`](#fsword-t)
  - [`clock_t`](#clock-t)
  - [`time_t`](#time-t)
- [Constants](#constants)
  - [`HUGETLB_FLAG_ENCODE_SHIFT`](#hugetlb-flag-encode-shift)
  - [`HUGETLB_FLAG_ENCODE_MASK`](#hugetlb-flag-encode-mask)
  - [`HUGETLB_FLAG_ENCODE_64KB`](#hugetlb-flag-encode-64kb)
  - [`HUGETLB_FLAG_ENCODE_512KB`](#hugetlb-flag-encode-512kb)
  - [`HUGETLB_FLAG_ENCODE_1MB`](#hugetlb-flag-encode-1mb)
  - [`HUGETLB_FLAG_ENCODE_2MB`](#hugetlb-flag-encode-2mb)
  - [`HUGETLB_FLAG_ENCODE_8MB`](#hugetlb-flag-encode-8mb)
  - [`HUGETLB_FLAG_ENCODE_16MB`](#hugetlb-flag-encode-16mb)
  - [`HUGETLB_FLAG_ENCODE_32MB`](#hugetlb-flag-encode-32mb)
  - [`HUGETLB_FLAG_ENCODE_256MB`](#hugetlb-flag-encode-256mb)
  - [`HUGETLB_FLAG_ENCODE_512MB`](#hugetlb-flag-encode-512mb)
  - [`HUGETLB_FLAG_ENCODE_1GB`](#hugetlb-flag-encode-1gb)
  - [`HUGETLB_FLAG_ENCODE_2GB`](#hugetlb-flag-encode-2gb)
  - [`HUGETLB_FLAG_ENCODE_16GB`](#hugetlb-flag-encode-16gb)
  - [`MAP_HUGE_SHIFT`](#map-huge-shift)
  - [`MAP_HUGE_MASK`](#map-huge-mask)
  - [`MAP_HUGE_64KB`](#map-huge-64kb)
  - [`MAP_HUGE_512KB`](#map-huge-512kb)
  - [`MAP_HUGE_1MB`](#map-huge-1mb)
  - [`MAP_HUGE_2MB`](#map-huge-2mb)
  - [`MAP_HUGE_8MB`](#map-huge-8mb)
  - [`MAP_HUGE_16MB`](#map-huge-16mb)
  - [`MAP_HUGE_32MB`](#map-huge-32mb)
  - [`MAP_HUGE_256MB`](#map-huge-256mb)
  - [`MAP_HUGE_512MB`](#map-huge-512mb)
  - [`MAP_HUGE_1GB`](#map-huge-1gb)
  - [`MAP_HUGE_2GB`](#map-huge-2gb)
  - [`MAP_HUGE_16GB`](#map-huge-16gb)
  - [`PRIO_PROCESS`](#prio-process)
  - [`PRIO_PGRP`](#prio-pgrp)
  - [`PRIO_USER`](#prio-user)
  - [`MS_RMT_MASK`](#ms-rmt-mask)
  - [`__UT_LINESIZE`](#ut-linesize)
  - [`__UT_NAMESIZE`](#ut-namesize)
  - [`__UT_HOSTSIZE`](#ut-hostsize)
  - [`EMPTY`](#empty)
  - [`RUN_LVL`](#run-lvl)
  - [`BOOT_TIME`](#boot-time)
  - [`NEW_TIME`](#new-time)
  - [`OLD_TIME`](#old-time)
  - [`INIT_PROCESS`](#init-process)
  - [`LOGIN_PROCESS`](#login-process)
  - [`USER_PROCESS`](#user-process)
  - [`DEAD_PROCESS`](#dead-process)
  - [`ACCOUNTING`](#accounting)
  - [`LM_ID_BASE`](#lm-id-base)
  - [`LM_ID_NEWLM`](#lm-id-newlm)
  - [`RTLD_DI_LMID`](#rtld-di-lmid)
  - [`RTLD_DI_LINKMAP`](#rtld-di-linkmap)
  - [`RTLD_DI_CONFIGADDR`](#rtld-di-configaddr)
  - [`RTLD_DI_SERINFO`](#rtld-di-serinfo)
  - [`RTLD_DI_SERINFOSIZE`](#rtld-di-serinfosize)
  - [`RTLD_DI_ORIGIN`](#rtld-di-origin)
  - [`RTLD_DI_PROFILENAME`](#rtld-di-profilename)
  - [`RTLD_DI_PROFILEOUT`](#rtld-di-profileout)
  - [`RTLD_DI_TLS_MODID`](#rtld-di-tls-modid)
  - [`RTLD_DI_TLS_DATA`](#rtld-di-tls-data)
  - [`SOCK_NONBLOCK`](#sock-nonblock)
  - [`SOL_RXRPC`](#sol-rxrpc)
  - [`SOL_PPPOL2TP`](#sol-pppol2tp)
  - [`SOL_PNPIPE`](#sol-pnpipe)
  - [`SOL_RDS`](#sol-rds)
  - [`SOL_IUCV`](#sol-iucv)
  - [`SOL_CAIF`](#sol-caif)
  - [`SOL_NFC`](#sol-nfc)
  - [`MSG_TRYHARD`](#msg-tryhard)
  - [`LC_PAPER`](#lc-paper)
  - [`LC_NAME`](#lc-name)
  - [`LC_ADDRESS`](#lc-address)
  - [`LC_TELEPHONE`](#lc-telephone)
  - [`LC_MEASUREMENT`](#lc-measurement)
  - [`LC_IDENTIFICATION`](#lc-identification)
  - [`LC_PAPER_MASK`](#lc-paper-mask)
  - [`LC_NAME_MASK`](#lc-name-mask)
  - [`LC_ADDRESS_MASK`](#lc-address-mask)
  - [`LC_TELEPHONE_MASK`](#lc-telephone-mask)
  - [`LC_MEASUREMENT_MASK`](#lc-measurement-mask)
  - [`LC_IDENTIFICATION_MASK`](#lc-identification-mask)
  - [`LC_ALL_MASK`](#lc-all-mask)
  - [`ENOTSUP`](#enotsup)
  - [`SOCK_SEQPACKET`](#sock-seqpacket)
  - [`SOCK_DCCP`](#sock-dccp)
  - [`SOCK_PACKET`](#sock-packet)
  - [`AF_IB`](#af-ib)
  - [`AF_MPLS`](#af-mpls)
  - [`AF_NFC`](#af-nfc)
  - [`AF_VSOCK`](#af-vsock)
  - [`AF_XDP`](#af-xdp)
  - [`PF_IB`](#pf-ib)
  - [`PF_MPLS`](#pf-mpls)
  - [`PF_NFC`](#pf-nfc)
  - [`PF_VSOCK`](#pf-vsock)
  - [`PF_XDP`](#pf-xdp)
  - [`SIGEV_THREAD_ID`](#sigev-thread-id)
  - [`BUFSIZ`](#bufsiz)
  - [`TMP_MAX`](#tmp-max)
  - [`FOPEN_MAX`](#fopen-max)
  - [`FILENAME_MAX`](#filename-max)
  - [`_CS_GNU_LIBC_VERSION`](#cs-gnu-libc-version)
  - [`_CS_GNU_LIBPTHREAD_VERSION`](#cs-gnu-libpthread-version)
  - [`_CS_V6_ENV`](#cs-v6-env)
  - [`_CS_V7_ENV`](#cs-v7-env)
  - [`_SC_EQUIV_CLASS_MAX`](#sc-equiv-class-max)
  - [`_SC_CHARCLASS_NAME_MAX`](#sc-charclass-name-max)
  - [`_SC_PII`](#sc-pii)
  - [`_SC_PII_XTI`](#sc-pii-xti)
  - [`_SC_PII_SOCKET`](#sc-pii-socket)
  - [`_SC_PII_INTERNET`](#sc-pii-internet)
  - [`_SC_PII_OSI`](#sc-pii-osi)
  - [`_SC_POLL`](#sc-poll)
  - [`_SC_SELECT`](#sc-select)
  - [`_SC_PII_INTERNET_STREAM`](#sc-pii-internet-stream)
  - [`_SC_PII_INTERNET_DGRAM`](#sc-pii-internet-dgram)
  - [`_SC_PII_OSI_COTS`](#sc-pii-osi-cots)
  - [`_SC_PII_OSI_CLTS`](#sc-pii-osi-clts)
  - [`_SC_PII_OSI_M`](#sc-pii-osi-m)
  - [`_SC_T_IOV_MAX`](#sc-t-iov-max)
  - [`_SC_2_C_VERSION`](#sc-2-c-version)
  - [`_SC_CHAR_BIT`](#sc-char-bit)
  - [`_SC_CHAR_MAX`](#sc-char-max)
  - [`_SC_CHAR_MIN`](#sc-char-min)
  - [`_SC_INT_MAX`](#sc-int-max)
  - [`_SC_INT_MIN`](#sc-int-min)
  - [`_SC_LONG_BIT`](#sc-long-bit)
  - [`_SC_WORD_BIT`](#sc-word-bit)
  - [`_SC_MB_LEN_MAX`](#sc-mb-len-max)
  - [`_SC_SSIZE_MAX`](#sc-ssize-max)
  - [`_SC_SCHAR_MAX`](#sc-schar-max)
  - [`_SC_SCHAR_MIN`](#sc-schar-min)
  - [`_SC_SHRT_MAX`](#sc-shrt-max)
  - [`_SC_SHRT_MIN`](#sc-shrt-min)
  - [`_SC_UCHAR_MAX`](#sc-uchar-max)
  - [`_SC_UINT_MAX`](#sc-uint-max)
  - [`_SC_ULONG_MAX`](#sc-ulong-max)
  - [`_SC_USHRT_MAX`](#sc-ushrt-max)
  - [`_SC_NL_ARGMAX`](#sc-nl-argmax)
  - [`_SC_NL_LANGMAX`](#sc-nl-langmax)
  - [`_SC_NL_MSGMAX`](#sc-nl-msgmax)
  - [`_SC_NL_NMAX`](#sc-nl-nmax)
  - [`_SC_NL_SETMAX`](#sc-nl-setmax)
  - [`_SC_NL_TEXTMAX`](#sc-nl-textmax)
  - [`_SC_BASE`](#sc-base)
  - [`_SC_C_LANG_SUPPORT`](#sc-c-lang-support)
  - [`_SC_C_LANG_SUPPORT_R`](#sc-c-lang-support-r)
  - [`_SC_DEVICE_IO`](#sc-device-io)
  - [`_SC_DEVICE_SPECIFIC`](#sc-device-specific)
  - [`_SC_DEVICE_SPECIFIC_R`](#sc-device-specific-r)
  - [`_SC_FD_MGMT`](#sc-fd-mgmt)
  - [`_SC_FIFO`](#sc-fifo)
  - [`_SC_PIPE`](#sc-pipe)
  - [`_SC_FILE_ATTRIBUTES`](#sc-file-attributes)
  - [`_SC_FILE_LOCKING`](#sc-file-locking)
  - [`_SC_FILE_SYSTEM`](#sc-file-system)
  - [`_SC_MULTI_PROCESS`](#sc-multi-process)
  - [`_SC_SINGLE_PROCESS`](#sc-single-process)
  - [`_SC_NETWORKING`](#sc-networking)
  - [`_SC_REGEX_VERSION`](#sc-regex-version)
  - [`_SC_SIGNALS`](#sc-signals)
  - [`_SC_SYSTEM_DATABASE`](#sc-system-database)
  - [`_SC_SYSTEM_DATABASE_R`](#sc-system-database-r)
  - [`_SC_USER_GROUPS`](#sc-user-groups)
  - [`_SC_USER_GROUPS_R`](#sc-user-groups-r)
  - [`_SC_LEVEL1_ICACHE_SIZE`](#sc-level1-icache-size)
  - [`_SC_LEVEL1_ICACHE_ASSOC`](#sc-level1-icache-assoc)
  - [`_SC_LEVEL1_ICACHE_LINESIZE`](#sc-level1-icache-linesize)
  - [`_SC_LEVEL1_DCACHE_SIZE`](#sc-level1-dcache-size)
  - [`_SC_LEVEL1_DCACHE_ASSOC`](#sc-level1-dcache-assoc)
  - [`_SC_LEVEL1_DCACHE_LINESIZE`](#sc-level1-dcache-linesize)
  - [`_SC_LEVEL2_CACHE_SIZE`](#sc-level2-cache-size)
  - [`_SC_LEVEL2_CACHE_ASSOC`](#sc-level2-cache-assoc)
  - [`_SC_LEVEL2_CACHE_LINESIZE`](#sc-level2-cache-linesize)
  - [`_SC_LEVEL3_CACHE_SIZE`](#sc-level3-cache-size)
  - [`_SC_LEVEL3_CACHE_ASSOC`](#sc-level3-cache-assoc)
  - [`_SC_LEVEL3_CACHE_LINESIZE`](#sc-level3-cache-linesize)
  - [`_SC_LEVEL4_CACHE_SIZE`](#sc-level4-cache-size)
  - [`_SC_LEVEL4_CACHE_ASSOC`](#sc-level4-cache-assoc)
  - [`_SC_LEVEL4_CACHE_LINESIZE`](#sc-level4-cache-linesize)
  - [`O_ACCMODE`](#o-accmode)
  - [`ST_RELATIME`](#st-relatime)
  - [`NI_MAXHOST`](#ni-maxhost)
  - [`BINDERFS_SUPER_MAGIC`](#binderfs-super-magic)
  - [`XFS_SUPER_MAGIC`](#xfs-super-magic)
  - [`CPU_SETSIZE`](#cpu-setsize)
  - [`PTRACE_TRACEME`](#ptrace-traceme)
  - [`PTRACE_PEEKTEXT`](#ptrace-peektext)
  - [`PTRACE_PEEKDATA`](#ptrace-peekdata)
  - [`PTRACE_PEEKUSER`](#ptrace-peekuser)
  - [`PTRACE_POKETEXT`](#ptrace-poketext)
  - [`PTRACE_POKEDATA`](#ptrace-pokedata)
  - [`PTRACE_POKEUSER`](#ptrace-pokeuser)
  - [`PTRACE_CONT`](#ptrace-cont)
  - [`PTRACE_KILL`](#ptrace-kill)
  - [`PTRACE_SINGLESTEP`](#ptrace-singlestep)
  - [`PTRACE_ATTACH`](#ptrace-attach)
  - [`PTRACE_SYSCALL`](#ptrace-syscall)
  - [`PTRACE_SETOPTIONS`](#ptrace-setoptions)
  - [`PTRACE_GETEVENTMSG`](#ptrace-geteventmsg)
  - [`PTRACE_GETSIGINFO`](#ptrace-getsiginfo)
  - [`PTRACE_SETSIGINFO`](#ptrace-setsiginfo)
  - [`PTRACE_GETREGSET`](#ptrace-getregset)
  - [`PTRACE_SETREGSET`](#ptrace-setregset)
  - [`PTRACE_SEIZE`](#ptrace-seize)
  - [`PTRACE_INTERRUPT`](#ptrace-interrupt)
  - [`PTRACE_LISTEN`](#ptrace-listen)
  - [`PTRACE_PEEKSIGINFO`](#ptrace-peeksiginfo)
  - [`PTRACE_GETSIGMASK`](#ptrace-getsigmask)
  - [`PTRACE_SETSIGMASK`](#ptrace-setsigmask)
  - [`PTRACE_GET_SYSCALL_INFO`](#ptrace-get-syscall-info)
  - [`PTRACE_SET_SYSCALL_INFO`](#ptrace-set-syscall-info)
  - [`PTRACE_SYSCALL_INFO_NONE`](#ptrace-syscall-info-none)
  - [`PTRACE_SYSCALL_INFO_ENTRY`](#ptrace-syscall-info-entry)
  - [`PTRACE_SYSCALL_INFO_EXIT`](#ptrace-syscall-info-exit)
  - [`PTRACE_SYSCALL_INFO_SECCOMP`](#ptrace-syscall-info-seccomp)
  - [`PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG`](#ptrace-set-syscall-user-dispatch-config)
  - [`PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG`](#ptrace-get-syscall-user-dispatch-config)
  - [`TCA_PAD`](#tca-pad)
  - [`TCA_DUMP_INVISIBLE`](#tca-dump-invisible)
  - [`TCA_CHAIN`](#tca-chain)
  - [`TCA_HW_OFFLOAD`](#tca-hw-offload)
  - [`RTM_DELNETCONF`](#rtm-delnetconf)
  - [`RTM_NEWSTATS`](#rtm-newstats)
  - [`RTM_GETSTATS`](#rtm-getstats)
  - [`RTM_NEWCACHEREPORT`](#rtm-newcachereport)
  - [`RTM_F_LOOKUP_TABLE`](#rtm-f-lookup-table)
  - [`RTM_F_FIB_MATCH`](#rtm-f-fib-match)
  - [`RTA_VIA`](#rta-via)
  - [`RTA_NEWDST`](#rta-newdst)
  - [`RTA_PREF`](#rta-pref)
  - [`RTA_ENCAP_TYPE`](#rta-encap-type)
  - [`RTA_ENCAP`](#rta-encap)
  - [`RTA_EXPIRES`](#rta-expires)
  - [`RTA_PAD`](#rta-pad)
  - [`RTA_UID`](#rta-uid)
  - [`RTA_TTL_PROPAGATE`](#rta-ttl-propagate)
  - [`NTF_EXT_LEARNED`](#ntf-ext-learned)
  - [`NTF_OFFLOADED`](#ntf-offloaded)
  - [`NDA_MASTER`](#nda-master)
  - [`NDA_LINK_NETNSID`](#nda-link-netnsid)
  - [`NDA_SRC_VNI`](#nda-src-vni)
  - [`UNAME26`](#uname26)
  - [`FDPIC_FUNCPTRS`](#fdpic-funcptrs)
  - [`GENL_UNS_ADMIN_PERM`](#genl-uns-admin-perm)
  - [`GENL_ID_VFS_DQUOT`](#genl-id-vfs-dquot)
  - [`GENL_ID_PMCRAID`](#genl-id-pmcraid)
  - [`ELFOSABI_ARM_AEABI`](#elfosabi-arm-aeabi)
  - [`CLONE_NEWTIME`](#clone-newtime)
  - [`CLONE_CLEAR_SIGHAND`](#clone-clear-sighand)
  - [`CLONE_INTO_CGROUP`](#clone-into-cgroup)
  - [`M_MXFAST`](#m-mxfast)
  - [`M_NLBLKS`](#m-nlblks)
  - [`M_GRAIN`](#m-grain)
  - [`M_KEEP`](#m-keep)
  - [`M_TRIM_THRESHOLD`](#m-trim-threshold)
  - [`M_TOP_PAD`](#m-top-pad)
  - [`M_MMAP_THRESHOLD`](#m-mmap-threshold)
  - [`M_MMAP_MAX`](#m-mmap-max)
  - [`M_CHECK_ACTION`](#m-check-action)
  - [`M_PERTURB`](#m-perturb)
  - [`M_ARENA_TEST`](#m-arena-test)
  - [`M_ARENA_MAX`](#m-arena-max)
  - [`SOMAXCONN`](#somaxconn)
  - [`MOVE_MOUNT_F_SYMLINKS`](#move-mount-f-symlinks)
  - [`MOVE_MOUNT_F_AUTOMOUNTS`](#move-mount-f-automounts)
  - [`MOVE_MOUNT_F_EMPTY_PATH`](#move-mount-f-empty-path)
  - [`MOVE_MOUNT_T_SYMLINKS`](#move-mount-t-symlinks)
  - [`MOVE_MOUNT_T_AUTOMOUNTS`](#move-mount-t-automounts)
  - [`MOVE_MOUNT_T_EMPTY_PATH`](#move-mount-t-empty-path)
  - [`MOVE_MOUNT_SET_GROUP`](#move-mount-set-group)
  - [`MOVE_MOUNT_BENEATH`](#move-mount-beneath)
  - [`ADJ_OFFSET`](#adj-offset)
  - [`ADJ_FREQUENCY`](#adj-frequency)
  - [`ADJ_MAXERROR`](#adj-maxerror)
  - [`ADJ_ESTERROR`](#adj-esterror)
  - [`ADJ_STATUS`](#adj-status)
  - [`ADJ_TIMECONST`](#adj-timeconst)
  - [`ADJ_TAI`](#adj-tai)
  - [`ADJ_SETOFFSET`](#adj-setoffset)
  - [`ADJ_MICRO`](#adj-micro)
  - [`ADJ_NANO`](#adj-nano)
  - [`ADJ_TICK`](#adj-tick)
  - [`ADJ_OFFSET_SINGLESHOT`](#adj-offset-singleshot)
  - [`ADJ_OFFSET_SS_READ`](#adj-offset-ss-read)
  - [`MOD_OFFSET`](#mod-offset)
  - [`MOD_FREQUENCY`](#mod-frequency)
  - [`MOD_MAXERROR`](#mod-maxerror)
  - [`MOD_ESTERROR`](#mod-esterror)
  - [`MOD_STATUS`](#mod-status)
  - [`MOD_TIMECONST`](#mod-timeconst)
  - [`MOD_CLKB`](#mod-clkb)
  - [`MOD_CLKA`](#mod-clka)
  - [`MOD_TAI`](#mod-tai)
  - [`MOD_MICRO`](#mod-micro)
  - [`MOD_NANO`](#mod-nano)
  - [`STA_PLL`](#sta-pll)
  - [`STA_PPSFREQ`](#sta-ppsfreq)
  - [`STA_PPSTIME`](#sta-ppstime)
  - [`STA_FLL`](#sta-fll)
  - [`STA_INS`](#sta-ins)
  - [`STA_DEL`](#sta-del)
  - [`STA_UNSYNC`](#sta-unsync)
  - [`STA_FREQHOLD`](#sta-freqhold)
  - [`STA_PPSSIGNAL`](#sta-ppssignal)
  - [`STA_PPSJITTER`](#sta-ppsjitter)
  - [`STA_PPSWANDER`](#sta-ppswander)
  - [`STA_PPSERROR`](#sta-ppserror)
  - [`STA_CLOCKERR`](#sta-clockerr)
  - [`STA_NANO`](#sta-nano)
  - [`STA_MODE`](#sta-mode)
  - [`STA_CLK`](#sta-clk)
  - [`STA_RONLY`](#sta-ronly)
  - [`NTP_API`](#ntp-api)
  - [`TIME_OK`](#time-ok)
  - [`TIME_INS`](#time-ins)
  - [`TIME_DEL`](#time-del)
  - [`TIME_OOP`](#time-oop)
  - [`TIME_WAIT`](#time-wait)
  - [`TIME_ERROR`](#time-error)
  - [`TIME_BAD`](#time-bad)
  - [`MAXTC`](#maxtc)
  - [`GLOB_PERIOD`](#glob-period)
  - [`GLOB_ALTDIRFUNC`](#glob-altdirfunc)
  - [`GLOB_BRACE`](#glob-brace)
  - [`GLOB_NOMAGIC`](#glob-nomagic)
  - [`GLOB_TILDE`](#glob-tilde)
  - [`GLOB_ONLYDIR`](#glob-onlydir)
  - [`GLOB_TILDE_CHECK`](#glob-tilde-check)
  - [`MADV_COLLAPSE`](#madv-collapse)
  - [`PTHREAD_STACK_MIN`](#pthread-stack-min)
  - [`PTHREAD_MUTEX_ADAPTIVE_NP`](#pthread-mutex-adaptive-np)
  - [`REG_STARTEND`](#reg-startend)
  - [`REG_EEND`](#reg-eend)
  - [`REG_ESIZE`](#reg-esize)
  - [`REG_ERPAREN`](#reg-erparen)
  - [`__SIZEOF_PTHREAD_RWLOCKATTR_T`](#sizeof-pthread-rwlockattr-t)
  - [`O_LARGEFILE`](#o-largefile)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`b64`](#b64) | mod | 64-bit specific definitions for linux-like values |
| [`x86_64`](#x86-64) | mod | x86_64-specific definitions for 64-bit linux-like values |
| [`aiocb`](#aiocb) | struct |  |
| [`__exit_status`](#exit-status) | struct |  |
| [`__timeval`](#timeval) | struct |  |
| [`glob64_t`](#glob64-t) | struct |  |
| [`msghdr`](#msghdr) | struct |  |
| [`cmsghdr`](#cmsghdr) | struct |  |
| [`termios`](#termios) | struct |  |
| [`mallinfo`](#mallinfo) | struct |  |
| [`mallinfo2`](#mallinfo2) | struct |  |
| [`ntptimeval`](#ntptimeval) | struct |  |
| [`regex_t`](#regex-t) | struct |  |
| [`Elf64_Chdr`](#elf64-chdr) | struct |  |
| [`Elf32_Chdr`](#elf32-chdr) | struct |  |
| [`seminfo`](#seminfo) | struct |  |
| [`ptrace_peeksiginfo_args`](#ptrace-peeksiginfo-args) | struct |  |
| [`__c_anonymous_ptrace_syscall_info_entry`](#c-anonymous-ptrace-syscall-info-entry) | struct |  |
| [`__c_anonymous_ptrace_syscall_info_exit`](#c-anonymous-ptrace-syscall-info-exit) | struct |  |
| [`__c_anonymous_ptrace_syscall_info_seccomp`](#c-anonymous-ptrace-syscall-info-seccomp) | struct |  |
| [`ptrace_syscall_info`](#ptrace-syscall-info) | struct |  |
| [`ptrace_sud_config`](#ptrace-sud-config) | struct |  |
| [`iocb`](#iocb) | struct |  |
| [`tcp_info`](#tcp-info) | struct |  |
| [`fanotify_event_info_pidfd`](#fanotify-event-info-pidfd) | struct |  |
| [`fanotify_event_info_error`](#fanotify-event-info-error) | struct |  |
| [`sem_t`](#sem-t) | struct |  |
| [`mbstate_t`](#mbstate-t) | struct |  |
| [`fpos64_t`](#fpos64-t) | struct |  |
| [`fpos_t`](#fpos-t) | struct |  |
| [`timespec`](#timespec) | struct |  |
| [`utmpx`](#utmpx) | struct |  |
| [`sifields_sigchld`](#sifields-sigchld) | struct |  |
| [`siginfo_f`](#siginfo-f) | struct |  |
| [`sigset_t`](#sigset-t) | struct |  |
| [`sysinfo`](#sysinfo) | struct |  |
| [`msqid_ds`](#msqid-ds) | struct |  |
| [`semid_ds`](#semid-ds) | struct |  |
| [`timex`](#timex) | struct |  |
| [`fgetspent_r`](#fgetspent-r) | fn |  |
| [`sgetspent_r`](#sgetspent-r) | fn |  |
| [`getspent_r`](#getspent-r) | fn |  |
| [`qsort_r`](#qsort-r) | fn |  |
| [`sendmmsg`](#sendmmsg) | fn |  |
| [`recvmmsg`](#recvmmsg) | fn |  |
| [`getrlimit64`](#getrlimit64) | fn |  |
| [`setrlimit64`](#setrlimit64) | fn |  |
| [`getrlimit`](#getrlimit) | fn |  |
| [`setrlimit`](#setrlimit) | fn |  |
| [`prlimit`](#prlimit) | fn |  |
| [`prlimit64`](#prlimit64) | fn |  |
| [`utmpname`](#utmpname) | fn |  |
| [`utmpxname`](#utmpxname) | fn |  |
| [`getutxent`](#getutxent) | fn |  |
| [`getutxid`](#getutxid) | fn |  |
| [`getutxline`](#getutxline) | fn |  |
| [`pututxline`](#pututxline) | fn |  |
| [`setutxent`](#setutxent) | fn |  |
| [`endutxent`](#endutxent) | fn |  |
| [`getpt`](#getpt) | fn |  |
| [`mallopt`](#mallopt) | fn |  |
| [`gettimeofday`](#gettimeofday) | fn |  |
| [`getentropy`](#getentropy) | fn |  |
| [`getrandom`](#getrandom) | fn |  |
| [`getauxval`](#getauxval) | fn |  |
| [`adjtimex`](#adjtimex) | fn |  |
| [`ntp_adjtime`](#ntp-adjtime) | fn |  |
| [`ntp_gettime`](#ntp-gettime) | fn |  |
| [`clock_adjtime`](#clock-adjtime) | fn |  |
| [`fanotify_mark`](#fanotify-mark) | fn |  |
| [`preadv2`](#preadv2) | fn |  |
| [`pwritev2`](#pwritev2) | fn |  |
| [`preadv64v2`](#preadv64v2) | fn |  |
| [`pwritev64v2`](#pwritev64v2) | fn |  |
| [`renameat2`](#renameat2) | fn |  |
| [`explicit_bzero`](#explicit-bzero) | fn |  |
| [`reallocarray`](#reallocarray) | fn |  |
| [`ctermid`](#ctermid) | fn |  |
| [`backtrace`](#backtrace) | fn |  |
| [`backtrace_symbols`](#backtrace-symbols) | fn |  |
| [`backtrace_symbols_fd`](#backtrace-symbols-fd) | fn |  |
| [`glob64`](#glob64) | fn |  |
| [`globfree64`](#globfree64) | fn |  |
| [`ptrace`](#ptrace) | fn |  |
| [`pthread_attr_getaffinity_np`](#pthread-attr-getaffinity-np) | fn |  |
| [`pthread_attr_setaffinity_np`](#pthread-attr-setaffinity-np) | fn |  |
| [`getpriority`](#getpriority) | fn |  |
| [`setpriority`](#setpriority) | fn |  |
| [`pthread_rwlockattr_getkind_np`](#pthread-rwlockattr-getkind-np) | fn |  |
| [`pthread_rwlockattr_setkind_np`](#pthread-rwlockattr-setkind-np) | fn |  |
| [`pthread_sigqueue`](#pthread-sigqueue) | fn |  |
| [`pthread_tryjoin_np`](#pthread-tryjoin-np) | fn |  |
| [`pthread_timedjoin_np`](#pthread-timedjoin-np) | fn |  |
| [`mallinfo`](#mallinfo) | fn |  |
| [`mallinfo2`](#mallinfo2) | fn |  |
| [`malloc_stats`](#malloc-stats) | fn |  |
| [`malloc_info`](#malloc-info) | fn |  |
| [`malloc_usable_size`](#malloc-usable-size) | fn |  |
| [`getpwent_r`](#getpwent-r) | fn |  |
| [`getgrent_r`](#getgrent-r) | fn |  |
| [`fgetpwent_r`](#fgetpwent-r) | fn |  |
| [`fgetgrent_r`](#fgetgrent-r) | fn |  |
| [`putpwent`](#putpwent) | fn |  |
| [`putgrent`](#putgrent) | fn |  |
| [`sethostid`](#sethostid) | fn |  |
| [`memfd_create`](#memfd-create) | fn |  |
| [`mlock2`](#mlock2) | fn |  |
| [`euidaccess`](#euidaccess) | fn |  |
| [`eaccess`](#eaccess) | fn |  |
| [`asctime_r`](#asctime-r) | fn |  |
| [`ctime_r`](#ctime-r) | fn |  |
| [`dirname`](#dirname) | fn |  |
| [`posix_basename`](#posix-basename) | fn | POSIX version of `basename(3)`, defined in `libgen.h`. |
| [`gnu_basename`](#gnu-basename) | fn | GNU version of `basename(3)`, defined in `string.h`. |
| [`dlmopen`](#dlmopen) | fn |  |
| [`dlinfo`](#dlinfo) | fn |  |
| [`dladdr1`](#dladdr1) | fn |  |
| [`dlvsym`](#dlvsym) | fn |  |
| [`malloc_trim`](#malloc-trim) | fn |  |
| [`gnu_get_libc_release`](#gnu-get-libc-release) | fn |  |
| [`gnu_get_libc_version`](#gnu-get-libc-version) | fn |  |
| [`posix_spawn_file_actions_addchdir_np`](#posix-spawn-file-actions-addchdir-np) | fn |  |
| [`posix_spawn_file_actions_addfchdir_np`](#posix-spawn-file-actions-addfchdir-np) | fn |  |
| [`posix_spawn_file_actions_addclosefrom_np`](#posix-spawn-file-actions-addclosefrom-np) | fn |  |
| [`posix_spawn_file_actions_addtcsetpgrp_np`](#posix-spawn-file-actions-addtcsetpgrp-np) | fn |  |
| [`getmntent_r`](#getmntent-r) | fn |  |
| [`execveat`](#execveat) | fn |  |
| [`close_range`](#close-range) | fn |  |
| [`mq_notify`](#mq-notify) | fn |  |
| [`epoll_pwait2`](#epoll-pwait2) | fn |  |
| [`mempcpy`](#mempcpy) | fn |  |
| [`tgkill`](#tgkill) | fn |  |
| [`pthread_t`](#pthread-t) | type |  |
| [`__priority_which_t`](#priority-which-t) | type |  |
| [`__rlimit_resource_t`](#rlimit-resource-t) | type |  |
| [`Lmid_t`](#lmid-t) | type |  |
| [`regoff_t`](#regoff-t) | type |  |
| [`__kernel_rwf_t`](#kernel-rwf-t) | type |  |
| [`Ioctl`](#ioctl) | type |  |
| [`ino_t`](#ino-t) | type |  |
| [`off_t`](#off-t) | type |  |
| [`blkcnt_t`](#blkcnt-t) | type |  |
| [`shmatt_t`](#shmatt-t) | type |  |
| [`msgqnum_t`](#msgqnum-t) | type |  |
| [`msglen_t`](#msglen-t) | type |  |
| [`fsblkcnt_t`](#fsblkcnt-t) | type |  |
| [`fsfilcnt_t`](#fsfilcnt-t) | type |  |
| [`rlim_t`](#rlim-t) | type |  |
| [`__syscall_ulong_t`](#syscall-ulong-t) | type |  |
| [`__fsword_t`](#fsword-t) | type |  |
| [`clock_t`](#clock-t) | type |  |
| [`time_t`](#time-t) | type |  |
| [`HUGETLB_FLAG_ENCODE_SHIFT`](#hugetlb-flag-encode-shift) | const |  |
| [`HUGETLB_FLAG_ENCODE_MASK`](#hugetlb-flag-encode-mask) | const |  |
| [`HUGETLB_FLAG_ENCODE_64KB`](#hugetlb-flag-encode-64kb) | const |  |
| [`HUGETLB_FLAG_ENCODE_512KB`](#hugetlb-flag-encode-512kb) | const |  |
| [`HUGETLB_FLAG_ENCODE_1MB`](#hugetlb-flag-encode-1mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_2MB`](#hugetlb-flag-encode-2mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_8MB`](#hugetlb-flag-encode-8mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_16MB`](#hugetlb-flag-encode-16mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_32MB`](#hugetlb-flag-encode-32mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_256MB`](#hugetlb-flag-encode-256mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_512MB`](#hugetlb-flag-encode-512mb) | const |  |
| [`HUGETLB_FLAG_ENCODE_1GB`](#hugetlb-flag-encode-1gb) | const |  |
| [`HUGETLB_FLAG_ENCODE_2GB`](#hugetlb-flag-encode-2gb) | const |  |
| [`HUGETLB_FLAG_ENCODE_16GB`](#hugetlb-flag-encode-16gb) | const |  |
| [`MAP_HUGE_SHIFT`](#map-huge-shift) | const |  |
| [`MAP_HUGE_MASK`](#map-huge-mask) | const |  |
| [`MAP_HUGE_64KB`](#map-huge-64kb) | const |  |
| [`MAP_HUGE_512KB`](#map-huge-512kb) | const |  |
| [`MAP_HUGE_1MB`](#map-huge-1mb) | const |  |
| [`MAP_HUGE_2MB`](#map-huge-2mb) | const |  |
| [`MAP_HUGE_8MB`](#map-huge-8mb) | const |  |
| [`MAP_HUGE_16MB`](#map-huge-16mb) | const |  |
| [`MAP_HUGE_32MB`](#map-huge-32mb) | const |  |
| [`MAP_HUGE_256MB`](#map-huge-256mb) | const |  |
| [`MAP_HUGE_512MB`](#map-huge-512mb) | const |  |
| [`MAP_HUGE_1GB`](#map-huge-1gb) | const |  |
| [`MAP_HUGE_2GB`](#map-huge-2gb) | const |  |
| [`MAP_HUGE_16GB`](#map-huge-16gb) | const |  |
| [`PRIO_PROCESS`](#prio-process) | const |  |
| [`PRIO_PGRP`](#prio-pgrp) | const |  |
| [`PRIO_USER`](#prio-user) | const |  |
| [`MS_RMT_MASK`](#ms-rmt-mask) | const |  |
| [`__UT_LINESIZE`](#ut-linesize) | const |  |
| [`__UT_NAMESIZE`](#ut-namesize) | const |  |
| [`__UT_HOSTSIZE`](#ut-hostsize) | const |  |
| [`EMPTY`](#empty) | const |  |
| [`RUN_LVL`](#run-lvl) | const |  |
| [`BOOT_TIME`](#boot-time) | const |  |
| [`NEW_TIME`](#new-time) | const |  |
| [`OLD_TIME`](#old-time) | const |  |
| [`INIT_PROCESS`](#init-process) | const |  |
| [`LOGIN_PROCESS`](#login-process) | const |  |
| [`USER_PROCESS`](#user-process) | const |  |
| [`DEAD_PROCESS`](#dead-process) | const |  |
| [`ACCOUNTING`](#accounting) | const |  |
| [`LM_ID_BASE`](#lm-id-base) | const |  |
| [`LM_ID_NEWLM`](#lm-id-newlm) | const |  |
| [`RTLD_DI_LMID`](#rtld-di-lmid) | const |  |
| [`RTLD_DI_LINKMAP`](#rtld-di-linkmap) | const |  |
| [`RTLD_DI_CONFIGADDR`](#rtld-di-configaddr) | const |  |
| [`RTLD_DI_SERINFO`](#rtld-di-serinfo) | const |  |
| [`RTLD_DI_SERINFOSIZE`](#rtld-di-serinfosize) | const |  |
| [`RTLD_DI_ORIGIN`](#rtld-di-origin) | const |  |
| [`RTLD_DI_PROFILENAME`](#rtld-di-profilename) | const |  |
| [`RTLD_DI_PROFILEOUT`](#rtld-di-profileout) | const |  |
| [`RTLD_DI_TLS_MODID`](#rtld-di-tls-modid) | const |  |
| [`RTLD_DI_TLS_DATA`](#rtld-di-tls-data) | const |  |
| [`SOCK_NONBLOCK`](#sock-nonblock) | const |  |
| [`SOL_RXRPC`](#sol-rxrpc) | const |  |
| [`SOL_PPPOL2TP`](#sol-pppol2tp) | const |  |
| [`SOL_PNPIPE`](#sol-pnpipe) | const |  |
| [`SOL_RDS`](#sol-rds) | const |  |
| [`SOL_IUCV`](#sol-iucv) | const |  |
| [`SOL_CAIF`](#sol-caif) | const |  |
| [`SOL_NFC`](#sol-nfc) | const |  |
| [`MSG_TRYHARD`](#msg-tryhard) | const |  |
| [`LC_PAPER`](#lc-paper) | const |  |
| [`LC_NAME`](#lc-name) | const |  |
| [`LC_ADDRESS`](#lc-address) | const |  |
| [`LC_TELEPHONE`](#lc-telephone) | const |  |
| [`LC_MEASUREMENT`](#lc-measurement) | const |  |
| [`LC_IDENTIFICATION`](#lc-identification) | const |  |
| [`LC_PAPER_MASK`](#lc-paper-mask) | const |  |
| [`LC_NAME_MASK`](#lc-name-mask) | const |  |
| [`LC_ADDRESS_MASK`](#lc-address-mask) | const |  |
| [`LC_TELEPHONE_MASK`](#lc-telephone-mask) | const |  |
| [`LC_MEASUREMENT_MASK`](#lc-measurement-mask) | const |  |
| [`LC_IDENTIFICATION_MASK`](#lc-identification-mask) | const |  |
| [`LC_ALL_MASK`](#lc-all-mask) | const |  |
| [`ENOTSUP`](#enotsup) | const |  |
| [`SOCK_SEQPACKET`](#sock-seqpacket) | const |  |
| [`SOCK_DCCP`](#sock-dccp) | const |  |
| [`SOCK_PACKET`](#sock-packet) | const |  |
| [`AF_IB`](#af-ib) | const |  |
| [`AF_MPLS`](#af-mpls) | const |  |
| [`AF_NFC`](#af-nfc) | const |  |
| [`AF_VSOCK`](#af-vsock) | const |  |
| [`AF_XDP`](#af-xdp) | const |  |
| [`PF_IB`](#pf-ib) | const |  |
| [`PF_MPLS`](#pf-mpls) | const |  |
| [`PF_NFC`](#pf-nfc) | const |  |
| [`PF_VSOCK`](#pf-vsock) | const |  |
| [`PF_XDP`](#pf-xdp) | const |  |
| [`SIGEV_THREAD_ID`](#sigev-thread-id) | const |  |
| [`BUFSIZ`](#bufsiz) | const |  |
| [`TMP_MAX`](#tmp-max) | const |  |
| [`FOPEN_MAX`](#fopen-max) | const |  |
| [`FILENAME_MAX`](#filename-max) | const |  |
| [`_CS_GNU_LIBC_VERSION`](#cs-gnu-libc-version) | const |  |
| [`_CS_GNU_LIBPTHREAD_VERSION`](#cs-gnu-libpthread-version) | const |  |
| [`_CS_V6_ENV`](#cs-v6-env) | const |  |
| [`_CS_V7_ENV`](#cs-v7-env) | const |  |
| [`_SC_EQUIV_CLASS_MAX`](#sc-equiv-class-max) | const |  |
| [`_SC_CHARCLASS_NAME_MAX`](#sc-charclass-name-max) | const |  |
| [`_SC_PII`](#sc-pii) | const |  |
| [`_SC_PII_XTI`](#sc-pii-xti) | const |  |
| [`_SC_PII_SOCKET`](#sc-pii-socket) | const |  |
| [`_SC_PII_INTERNET`](#sc-pii-internet) | const |  |
| [`_SC_PII_OSI`](#sc-pii-osi) | const |  |
| [`_SC_POLL`](#sc-poll) | const |  |
| [`_SC_SELECT`](#sc-select) | const |  |
| [`_SC_PII_INTERNET_STREAM`](#sc-pii-internet-stream) | const |  |
| [`_SC_PII_INTERNET_DGRAM`](#sc-pii-internet-dgram) | const |  |
| [`_SC_PII_OSI_COTS`](#sc-pii-osi-cots) | const |  |
| [`_SC_PII_OSI_CLTS`](#sc-pii-osi-clts) | const |  |
| [`_SC_PII_OSI_M`](#sc-pii-osi-m) | const |  |
| [`_SC_T_IOV_MAX`](#sc-t-iov-max) | const |  |
| [`_SC_2_C_VERSION`](#sc-2-c-version) | const |  |
| [`_SC_CHAR_BIT`](#sc-char-bit) | const |  |
| [`_SC_CHAR_MAX`](#sc-char-max) | const |  |
| [`_SC_CHAR_MIN`](#sc-char-min) | const |  |
| [`_SC_INT_MAX`](#sc-int-max) | const |  |
| [`_SC_INT_MIN`](#sc-int-min) | const |  |
| [`_SC_LONG_BIT`](#sc-long-bit) | const |  |
| [`_SC_WORD_BIT`](#sc-word-bit) | const |  |
| [`_SC_MB_LEN_MAX`](#sc-mb-len-max) | const |  |
| [`_SC_SSIZE_MAX`](#sc-ssize-max) | const |  |
| [`_SC_SCHAR_MAX`](#sc-schar-max) | const |  |
| [`_SC_SCHAR_MIN`](#sc-schar-min) | const |  |
| [`_SC_SHRT_MAX`](#sc-shrt-max) | const |  |
| [`_SC_SHRT_MIN`](#sc-shrt-min) | const |  |
| [`_SC_UCHAR_MAX`](#sc-uchar-max) | const |  |
| [`_SC_UINT_MAX`](#sc-uint-max) | const |  |
| [`_SC_ULONG_MAX`](#sc-ulong-max) | const |  |
| [`_SC_USHRT_MAX`](#sc-ushrt-max) | const |  |
| [`_SC_NL_ARGMAX`](#sc-nl-argmax) | const |  |
| [`_SC_NL_LANGMAX`](#sc-nl-langmax) | const |  |
| [`_SC_NL_MSGMAX`](#sc-nl-msgmax) | const |  |
| [`_SC_NL_NMAX`](#sc-nl-nmax) | const |  |
| [`_SC_NL_SETMAX`](#sc-nl-setmax) | const |  |
| [`_SC_NL_TEXTMAX`](#sc-nl-textmax) | const |  |
| [`_SC_BASE`](#sc-base) | const |  |
| [`_SC_C_LANG_SUPPORT`](#sc-c-lang-support) | const |  |
| [`_SC_C_LANG_SUPPORT_R`](#sc-c-lang-support-r) | const |  |
| [`_SC_DEVICE_IO`](#sc-device-io) | const |  |
| [`_SC_DEVICE_SPECIFIC`](#sc-device-specific) | const |  |
| [`_SC_DEVICE_SPECIFIC_R`](#sc-device-specific-r) | const |  |
| [`_SC_FD_MGMT`](#sc-fd-mgmt) | const |  |
| [`_SC_FIFO`](#sc-fifo) | const |  |
| [`_SC_PIPE`](#sc-pipe) | const |  |
| [`_SC_FILE_ATTRIBUTES`](#sc-file-attributes) | const |  |
| [`_SC_FILE_LOCKING`](#sc-file-locking) | const |  |
| [`_SC_FILE_SYSTEM`](#sc-file-system) | const |  |
| [`_SC_MULTI_PROCESS`](#sc-multi-process) | const |  |
| [`_SC_SINGLE_PROCESS`](#sc-single-process) | const |  |
| [`_SC_NETWORKING`](#sc-networking) | const |  |
| [`_SC_REGEX_VERSION`](#sc-regex-version) | const |  |
| [`_SC_SIGNALS`](#sc-signals) | const |  |
| [`_SC_SYSTEM_DATABASE`](#sc-system-database) | const |  |
| [`_SC_SYSTEM_DATABASE_R`](#sc-system-database-r) | const |  |
| [`_SC_USER_GROUPS`](#sc-user-groups) | const |  |
| [`_SC_USER_GROUPS_R`](#sc-user-groups-r) | const |  |
| [`_SC_LEVEL1_ICACHE_SIZE`](#sc-level1-icache-size) | const |  |
| [`_SC_LEVEL1_ICACHE_ASSOC`](#sc-level1-icache-assoc) | const |  |
| [`_SC_LEVEL1_ICACHE_LINESIZE`](#sc-level1-icache-linesize) | const |  |
| [`_SC_LEVEL1_DCACHE_SIZE`](#sc-level1-dcache-size) | const |  |
| [`_SC_LEVEL1_DCACHE_ASSOC`](#sc-level1-dcache-assoc) | const |  |
| [`_SC_LEVEL1_DCACHE_LINESIZE`](#sc-level1-dcache-linesize) | const |  |
| [`_SC_LEVEL2_CACHE_SIZE`](#sc-level2-cache-size) | const |  |
| [`_SC_LEVEL2_CACHE_ASSOC`](#sc-level2-cache-assoc) | const |  |
| [`_SC_LEVEL2_CACHE_LINESIZE`](#sc-level2-cache-linesize) | const |  |
| [`_SC_LEVEL3_CACHE_SIZE`](#sc-level3-cache-size) | const |  |
| [`_SC_LEVEL3_CACHE_ASSOC`](#sc-level3-cache-assoc) | const |  |
| [`_SC_LEVEL3_CACHE_LINESIZE`](#sc-level3-cache-linesize) | const |  |
| [`_SC_LEVEL4_CACHE_SIZE`](#sc-level4-cache-size) | const |  |
| [`_SC_LEVEL4_CACHE_ASSOC`](#sc-level4-cache-assoc) | const |  |
| [`_SC_LEVEL4_CACHE_LINESIZE`](#sc-level4-cache-linesize) | const |  |
| [`O_ACCMODE`](#o-accmode) | const |  |
| [`ST_RELATIME`](#st-relatime) | const |  |
| [`NI_MAXHOST`](#ni-maxhost) | const |  |
| [`BINDERFS_SUPER_MAGIC`](#binderfs-super-magic) | const |  |
| [`XFS_SUPER_MAGIC`](#xfs-super-magic) | const |  |
| [`CPU_SETSIZE`](#cpu-setsize) | const |  |
| [`PTRACE_TRACEME`](#ptrace-traceme) | const |  |
| [`PTRACE_PEEKTEXT`](#ptrace-peektext) | const |  |
| [`PTRACE_PEEKDATA`](#ptrace-peekdata) | const |  |
| [`PTRACE_PEEKUSER`](#ptrace-peekuser) | const |  |
| [`PTRACE_POKETEXT`](#ptrace-poketext) | const |  |
| [`PTRACE_POKEDATA`](#ptrace-pokedata) | const |  |
| [`PTRACE_POKEUSER`](#ptrace-pokeuser) | const |  |
| [`PTRACE_CONT`](#ptrace-cont) | const |  |
| [`PTRACE_KILL`](#ptrace-kill) | const |  |
| [`PTRACE_SINGLESTEP`](#ptrace-singlestep) | const |  |
| [`PTRACE_ATTACH`](#ptrace-attach) | const |  |
| [`PTRACE_SYSCALL`](#ptrace-syscall) | const |  |
| [`PTRACE_SETOPTIONS`](#ptrace-setoptions) | const |  |
| [`PTRACE_GETEVENTMSG`](#ptrace-geteventmsg) | const |  |
| [`PTRACE_GETSIGINFO`](#ptrace-getsiginfo) | const |  |
| [`PTRACE_SETSIGINFO`](#ptrace-setsiginfo) | const |  |
| [`PTRACE_GETREGSET`](#ptrace-getregset) | const |  |
| [`PTRACE_SETREGSET`](#ptrace-setregset) | const |  |
| [`PTRACE_SEIZE`](#ptrace-seize) | const |  |
| [`PTRACE_INTERRUPT`](#ptrace-interrupt) | const |  |
| [`PTRACE_LISTEN`](#ptrace-listen) | const |  |
| [`PTRACE_PEEKSIGINFO`](#ptrace-peeksiginfo) | const |  |
| [`PTRACE_GETSIGMASK`](#ptrace-getsigmask) | const |  |
| [`PTRACE_SETSIGMASK`](#ptrace-setsigmask) | const |  |
| [`PTRACE_GET_SYSCALL_INFO`](#ptrace-get-syscall-info) | const |  |
| [`PTRACE_SET_SYSCALL_INFO`](#ptrace-set-syscall-info) | const |  |
| [`PTRACE_SYSCALL_INFO_NONE`](#ptrace-syscall-info-none) | const |  |
| [`PTRACE_SYSCALL_INFO_ENTRY`](#ptrace-syscall-info-entry) | const |  |
| [`PTRACE_SYSCALL_INFO_EXIT`](#ptrace-syscall-info-exit) | const |  |
| [`PTRACE_SYSCALL_INFO_SECCOMP`](#ptrace-syscall-info-seccomp) | const |  |
| [`PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG`](#ptrace-set-syscall-user-dispatch-config) | const |  |
| [`PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG`](#ptrace-get-syscall-user-dispatch-config) | const |  |
| [`TCA_PAD`](#tca-pad) | const |  |
| [`TCA_DUMP_INVISIBLE`](#tca-dump-invisible) | const |  |
| [`TCA_CHAIN`](#tca-chain) | const |  |
| [`TCA_HW_OFFLOAD`](#tca-hw-offload) | const |  |
| [`RTM_DELNETCONF`](#rtm-delnetconf) | const |  |
| [`RTM_NEWSTATS`](#rtm-newstats) | const |  |
| [`RTM_GETSTATS`](#rtm-getstats) | const |  |
| [`RTM_NEWCACHEREPORT`](#rtm-newcachereport) | const |  |
| [`RTM_F_LOOKUP_TABLE`](#rtm-f-lookup-table) | const |  |
| [`RTM_F_FIB_MATCH`](#rtm-f-fib-match) | const |  |
| [`RTA_VIA`](#rta-via) | const |  |
| [`RTA_NEWDST`](#rta-newdst) | const |  |
| [`RTA_PREF`](#rta-pref) | const |  |
| [`RTA_ENCAP_TYPE`](#rta-encap-type) | const |  |
| [`RTA_ENCAP`](#rta-encap) | const |  |
| [`RTA_EXPIRES`](#rta-expires) | const |  |
| [`RTA_PAD`](#rta-pad) | const |  |
| [`RTA_UID`](#rta-uid) | const |  |
| [`RTA_TTL_PROPAGATE`](#rta-ttl-propagate) | const |  |
| [`NTF_EXT_LEARNED`](#ntf-ext-learned) | const |  |
| [`NTF_OFFLOADED`](#ntf-offloaded) | const |  |
| [`NDA_MASTER`](#nda-master) | const |  |
| [`NDA_LINK_NETNSID`](#nda-link-netnsid) | const |  |
| [`NDA_SRC_VNI`](#nda-src-vni) | const |  |
| [`UNAME26`](#uname26) | const |  |
| [`FDPIC_FUNCPTRS`](#fdpic-funcptrs) | const |  |
| [`GENL_UNS_ADMIN_PERM`](#genl-uns-admin-perm) | const |  |
| [`GENL_ID_VFS_DQUOT`](#genl-id-vfs-dquot) | const |  |
| [`GENL_ID_PMCRAID`](#genl-id-pmcraid) | const |  |
| [`ELFOSABI_ARM_AEABI`](#elfosabi-arm-aeabi) | const |  |
| [`CLONE_NEWTIME`](#clone-newtime) | const |  |
| [`CLONE_CLEAR_SIGHAND`](#clone-clear-sighand) | const |  |
| [`CLONE_INTO_CGROUP`](#clone-into-cgroup) | const |  |
| [`M_MXFAST`](#m-mxfast) | const |  |
| [`M_NLBLKS`](#m-nlblks) | const |  |
| [`M_GRAIN`](#m-grain) | const |  |
| [`M_KEEP`](#m-keep) | const |  |
| [`M_TRIM_THRESHOLD`](#m-trim-threshold) | const |  |
| [`M_TOP_PAD`](#m-top-pad) | const |  |
| [`M_MMAP_THRESHOLD`](#m-mmap-threshold) | const |  |
| [`M_MMAP_MAX`](#m-mmap-max) | const |  |
| [`M_CHECK_ACTION`](#m-check-action) | const |  |
| [`M_PERTURB`](#m-perturb) | const |  |
| [`M_ARENA_TEST`](#m-arena-test) | const |  |
| [`M_ARENA_MAX`](#m-arena-max) | const |  |
| [`SOMAXCONN`](#somaxconn) | const |  |
| [`MOVE_MOUNT_F_SYMLINKS`](#move-mount-f-symlinks) | const |  |
| [`MOVE_MOUNT_F_AUTOMOUNTS`](#move-mount-f-automounts) | const |  |
| [`MOVE_MOUNT_F_EMPTY_PATH`](#move-mount-f-empty-path) | const |  |
| [`MOVE_MOUNT_T_SYMLINKS`](#move-mount-t-symlinks) | const |  |
| [`MOVE_MOUNT_T_AUTOMOUNTS`](#move-mount-t-automounts) | const |  |
| [`MOVE_MOUNT_T_EMPTY_PATH`](#move-mount-t-empty-path) | const |  |
| [`MOVE_MOUNT_SET_GROUP`](#move-mount-set-group) | const |  |
| [`MOVE_MOUNT_BENEATH`](#move-mount-beneath) | const |  |
| [`ADJ_OFFSET`](#adj-offset) | const |  |
| [`ADJ_FREQUENCY`](#adj-frequency) | const |  |
| [`ADJ_MAXERROR`](#adj-maxerror) | const |  |
| [`ADJ_ESTERROR`](#adj-esterror) | const |  |
| [`ADJ_STATUS`](#adj-status) | const |  |
| [`ADJ_TIMECONST`](#adj-timeconst) | const |  |
| [`ADJ_TAI`](#adj-tai) | const |  |
| [`ADJ_SETOFFSET`](#adj-setoffset) | const |  |
| [`ADJ_MICRO`](#adj-micro) | const |  |
| [`ADJ_NANO`](#adj-nano) | const |  |
| [`ADJ_TICK`](#adj-tick) | const |  |
| [`ADJ_OFFSET_SINGLESHOT`](#adj-offset-singleshot) | const |  |
| [`ADJ_OFFSET_SS_READ`](#adj-offset-ss-read) | const |  |
| [`MOD_OFFSET`](#mod-offset) | const |  |
| [`MOD_FREQUENCY`](#mod-frequency) | const |  |
| [`MOD_MAXERROR`](#mod-maxerror) | const |  |
| [`MOD_ESTERROR`](#mod-esterror) | const |  |
| [`MOD_STATUS`](#mod-status) | const |  |
| [`MOD_TIMECONST`](#mod-timeconst) | const |  |
| [`MOD_CLKB`](#mod-clkb) | const |  |
| [`MOD_CLKA`](#mod-clka) | const |  |
| [`MOD_TAI`](#mod-tai) | const |  |
| [`MOD_MICRO`](#mod-micro) | const |  |
| [`MOD_NANO`](#mod-nano) | const |  |
| [`STA_PLL`](#sta-pll) | const |  |
| [`STA_PPSFREQ`](#sta-ppsfreq) | const |  |
| [`STA_PPSTIME`](#sta-ppstime) | const |  |
| [`STA_FLL`](#sta-fll) | const |  |
| [`STA_INS`](#sta-ins) | const |  |
| [`STA_DEL`](#sta-del) | const |  |
| [`STA_UNSYNC`](#sta-unsync) | const |  |
| [`STA_FREQHOLD`](#sta-freqhold) | const |  |
| [`STA_PPSSIGNAL`](#sta-ppssignal) | const |  |
| [`STA_PPSJITTER`](#sta-ppsjitter) | const |  |
| [`STA_PPSWANDER`](#sta-ppswander) | const |  |
| [`STA_PPSERROR`](#sta-ppserror) | const |  |
| [`STA_CLOCKERR`](#sta-clockerr) | const |  |
| [`STA_NANO`](#sta-nano) | const |  |
| [`STA_MODE`](#sta-mode) | const |  |
| [`STA_CLK`](#sta-clk) | const |  |
| [`STA_RONLY`](#sta-ronly) | const |  |
| [`NTP_API`](#ntp-api) | const |  |
| [`TIME_OK`](#time-ok) | const |  |
| [`TIME_INS`](#time-ins) | const |  |
| [`TIME_DEL`](#time-del) | const |  |
| [`TIME_OOP`](#time-oop) | const |  |
| [`TIME_WAIT`](#time-wait) | const |  |
| [`TIME_ERROR`](#time-error) | const |  |
| [`TIME_BAD`](#time-bad) | const |  |
| [`MAXTC`](#maxtc) | const |  |
| [`GLOB_PERIOD`](#glob-period) | const |  |
| [`GLOB_ALTDIRFUNC`](#glob-altdirfunc) | const |  |
| [`GLOB_BRACE`](#glob-brace) | const |  |
| [`GLOB_NOMAGIC`](#glob-nomagic) | const |  |
| [`GLOB_TILDE`](#glob-tilde) | const |  |
| [`GLOB_ONLYDIR`](#glob-onlydir) | const |  |
| [`GLOB_TILDE_CHECK`](#glob-tilde-check) | const |  |
| [`MADV_COLLAPSE`](#madv-collapse) | const |  |
| [`PTHREAD_STACK_MIN`](#pthread-stack-min) | const |  |
| [`PTHREAD_MUTEX_ADAPTIVE_NP`](#pthread-mutex-adaptive-np) | const |  |
| [`REG_STARTEND`](#reg-startend) | const |  |
| [`REG_EEND`](#reg-eend) | const |  |
| [`REG_ESIZE`](#reg-esize) | const |  |
| [`REG_ERPAREN`](#reg-erparen) | const |  |
| [`__SIZEOF_PTHREAD_RWLOCKATTR_T`](#sizeof-pthread-rwlockattr-t) | const |  |
| [`O_LARGEFILE`](#o-largefile) | const |  |

## Modules

- [`b64`](b64/index.md) — 64-bit specific definitions for linux-like values
- [`x86_64`](x86_64/index.md) — x86_64-specific definitions for 64-bit linux-like values

## Structs

### `aiocb`

```rust
struct aiocb {
    pub aio_fildes: c_int,
    pub aio_lio_opcode: c_int,
    pub aio_reqprio: c_int,
    pub aio_buf: *mut c_void,
    pub aio_nbytes: size_t,
    pub aio_sigevent: crate::sigevent,
    __next_prio: *mut aiocb,
    __abs_prio: c_int,
    __policy: c_int,
    __error_code: c_int,
    __return_value: ssize_t,
    pub aio_offset: off_t,
    __glibc_reserved: Padding<[c_char; 32]>,
}
```

#### Trait Implementations

##### `impl Clone for aiocb`

- <span id="aiocb-clone"></span>`fn clone(&self) -> aiocb` — [`aiocb`](../index.md#aiocb)

##### `impl Copy for aiocb`

##### `impl Debug for aiocb`

- <span id="aiocb-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__exit_status`

```rust
struct __exit_status {
    pub e_termination: c_short,
    pub e_exit: c_short,
}
```

#### Trait Implementations

##### `impl Clone for __exit_status`

- <span id="exit-status-clone"></span>`fn clone(&self) -> __exit_status` — [`__exit_status`](../index.md#exit-status)

##### `impl Copy for __exit_status`

##### `impl Debug for __exit_status`

- <span id="exit-status-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__timeval`

```rust
struct __timeval {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
```

#### Trait Implementations

##### `impl Clone for __timeval`

- <span id="timeval-clone"></span>`fn clone(&self) -> __timeval` — [`__timeval`](../index.md#timeval)

##### `impl Copy for __timeval`

##### `impl Debug for __timeval`

- <span id="timeval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `glob64_t`

```rust
struct glob64_t {
    pub gl_pathc: size_t,
    pub gl_pathv: *mut *mut c_char,
    pub gl_offs: size_t,
    pub gl_flags: c_int,
    __unused1: Padding<*mut c_void>,
    __unused2: Padding<*mut c_void>,
    __unused3: Padding<*mut c_void>,
    __unused4: Padding<*mut c_void>,
    __unused5: Padding<*mut c_void>,
}
```

#### Trait Implementations

##### `impl Clone for glob64_t`

- <span id="glob64-t-clone"></span>`fn clone(&self) -> glob64_t` — [`glob64_t`](../index.md#glob64-t)

##### `impl Copy for glob64_t`

##### `impl Debug for glob64_t`

- <span id="glob64-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `msghdr`

```rust
struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: crate::socklen_t,
    pub msg_iov: *mut crate::iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut c_void,
    pub msg_controllen: size_t,
    pub msg_flags: c_int,
}
```

#### Trait Implementations

##### `impl Clone for msghdr`

- <span id="msghdr-clone"></span>`fn clone(&self) -> msghdr` — [`msghdr`](../index.md#msghdr)

##### `impl Copy for msghdr`

##### `impl Debug for msghdr`

- <span id="msghdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `cmsghdr`

```rust
struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}
```

#### Trait Implementations

##### `impl Clone for cmsghdr`

- <span id="cmsghdr-clone"></span>`fn clone(&self) -> cmsghdr` — [`cmsghdr`](../index.md#cmsghdr)

##### `impl Copy for cmsghdr`

##### `impl Debug for cmsghdr`

- <span id="cmsghdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `termios`

```rust
struct termios {
    pub c_iflag: crate::tcflag_t,
    pub c_oflag: crate::tcflag_t,
    pub c_cflag: crate::tcflag_t,
    pub c_lflag: crate::tcflag_t,
    pub c_line: crate::cc_t,
    pub c_cc: [crate::cc_t; 32],
    pub c_ispeed: crate::speed_t,
    pub c_ospeed: crate::speed_t,
}
```

#### Trait Implementations

##### `impl Clone for termios`

- <span id="termios-clone"></span>`fn clone(&self) -> termios` — [`termios`](../index.md#termios)

##### `impl Copy for termios`

##### `impl Debug for termios`

- <span id="termios-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mallinfo`

```rust
struct mallinfo {
    pub arena: c_int,
    pub ordblks: c_int,
    pub smblks: c_int,
    pub hblks: c_int,
    pub hblkhd: c_int,
    pub usmblks: c_int,
    pub fsmblks: c_int,
    pub uordblks: c_int,
    pub fordblks: c_int,
    pub keepcost: c_int,
}
```

#### Trait Implementations

##### `impl Clone for mallinfo`

- <span id="mallinfo-clone"></span>`fn clone(&self) -> mallinfo` — [`mallinfo`](../index.md#mallinfo)

##### `impl Copy for mallinfo`

##### `impl Debug for mallinfo`

- <span id="mallinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mallinfo2`

```rust
struct mallinfo2 {
    pub arena: size_t,
    pub ordblks: size_t,
    pub smblks: size_t,
    pub hblks: size_t,
    pub hblkhd: size_t,
    pub usmblks: size_t,
    pub fsmblks: size_t,
    pub uordblks: size_t,
    pub fordblks: size_t,
    pub keepcost: size_t,
}
```

#### Trait Implementations

##### `impl Clone for mallinfo2`

- <span id="mallinfo2-clone"></span>`fn clone(&self) -> mallinfo2` — [`mallinfo2`](../index.md#mallinfo2)

##### `impl Copy for mallinfo2`

##### `impl Debug for mallinfo2`

- <span id="mallinfo2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ntptimeval`

```rust
struct ntptimeval {
    pub time: crate::timeval,
    pub maxerror: c_long,
    pub esterror: c_long,
    pub tai: c_long,
    pub __glibc_reserved1: c_long,
    pub __glibc_reserved2: c_long,
    pub __glibc_reserved3: c_long,
    pub __glibc_reserved4: c_long,
}
```

#### Trait Implementations

##### `impl Clone for ntptimeval`

- <span id="ntptimeval-clone"></span>`fn clone(&self) -> ntptimeval` — [`ntptimeval`](../index.md#ntptimeval)

##### `impl Copy for ntptimeval`

##### `impl Debug for ntptimeval`

- <span id="ntptimeval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `regex_t`

```rust
struct regex_t {
    __buffer: *mut c_void,
    __allocated: size_t,
    __used: size_t,
    __syntax: c_ulong,
    __fastmap: *mut c_char,
    __translate: *mut c_char,
    __re_nsub: size_t,
    __bitfield: u8,
}
```

#### Trait Implementations

##### `impl Clone for regex_t`

- <span id="regex-t-clone"></span>`fn clone(&self) -> regex_t` — [`regex_t`](../index.md#regex-t)

##### `impl Copy for regex_t`

##### `impl Debug for regex_t`

- <span id="regex-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Elf64_Chdr`

```rust
struct Elf64_Chdr {
    pub ch_type: crate::Elf64_Word,
    pub ch_reserved: crate::Elf64_Word,
    pub ch_size: crate::Elf64_Xword,
    pub ch_addralign: crate::Elf64_Xword,
}
```

#### Trait Implementations

##### `impl Clone for Elf64_Chdr`

- <span id="elf64-chdr-clone"></span>`fn clone(&self) -> Elf64_Chdr` — [`Elf64_Chdr`](../index.md#elf64-chdr)

##### `impl Copy for Elf64_Chdr`

##### `impl Debug for Elf64_Chdr`

- <span id="elf64-chdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Elf32_Chdr`

```rust
struct Elf32_Chdr {
    pub ch_type: crate::Elf32_Word,
    pub ch_size: crate::Elf32_Word,
    pub ch_addralign: crate::Elf32_Word,
}
```

#### Trait Implementations

##### `impl Clone for Elf32_Chdr`

- <span id="elf32-chdr-clone"></span>`fn clone(&self) -> Elf32_Chdr` — [`Elf32_Chdr`](../index.md#elf32-chdr)

##### `impl Copy for Elf32_Chdr`

##### `impl Debug for Elf32_Chdr`

- <span id="elf32-chdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `seminfo`

```rust
struct seminfo {
    pub semmap: c_int,
    pub semmni: c_int,
    pub semmns: c_int,
    pub semmnu: c_int,
    pub semmsl: c_int,
    pub semopm: c_int,
    pub semume: c_int,
    pub semusz: c_int,
    pub semvmx: c_int,
    pub semaem: c_int,
}
```

#### Trait Implementations

##### `impl Clone for seminfo`

- <span id="seminfo-clone"></span>`fn clone(&self) -> seminfo` — [`seminfo`](../index.md#seminfo)

##### `impl Copy for seminfo`

##### `impl Debug for seminfo`

- <span id="seminfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptrace_peeksiginfo_args`

```rust
struct ptrace_peeksiginfo_args {
    pub off: crate::__u64,
    pub flags: crate::__u32,
    pub nr: crate::__s32,
}
```

#### Trait Implementations

##### `impl Clone for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-clone"></span>`fn clone(&self) -> ptrace_peeksiginfo_args` — [`ptrace_peeksiginfo_args`](../index.md#ptrace-peeksiginfo-args)

##### `impl Copy for ptrace_peeksiginfo_args`

##### `impl Debug for ptrace_peeksiginfo_args`

- <span id="ptrace-peeksiginfo-args-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_ptrace_syscall_info_entry`

```rust
struct __c_anonymous_ptrace_syscall_info_entry {
    pub nr: crate::__u64,
    pub args: [crate::__u64; 6],
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-clone"></span>`fn clone(&self) -> __c_anonymous_ptrace_syscall_info_entry` — [`__c_anonymous_ptrace_syscall_info_entry`](../index.md#c-anonymous-ptrace-syscall-info-entry)

##### `impl Copy for __c_anonymous_ptrace_syscall_info_entry`

##### `impl Debug for __c_anonymous_ptrace_syscall_info_entry`

- <span id="c-anonymous-ptrace-syscall-info-entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_ptrace_syscall_info_exit`

```rust
struct __c_anonymous_ptrace_syscall_info_exit {
    pub sval: crate::__s64,
    pub is_error: crate::__u8,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-clone"></span>`fn clone(&self) -> __c_anonymous_ptrace_syscall_info_exit` — [`__c_anonymous_ptrace_syscall_info_exit`](../index.md#c-anonymous-ptrace-syscall-info-exit)

##### `impl Copy for __c_anonymous_ptrace_syscall_info_exit`

##### `impl Debug for __c_anonymous_ptrace_syscall_info_exit`

- <span id="c-anonymous-ptrace-syscall-info-exit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_ptrace_syscall_info_seccomp`

```rust
struct __c_anonymous_ptrace_syscall_info_seccomp {
    pub nr: crate::__u64,
    pub args: [crate::__u64; 6],
    pub ret_data: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-clone"></span>`fn clone(&self) -> __c_anonymous_ptrace_syscall_info_seccomp` — [`__c_anonymous_ptrace_syscall_info_seccomp`](../index.md#c-anonymous-ptrace-syscall-info-seccomp)

##### `impl Copy for __c_anonymous_ptrace_syscall_info_seccomp`

##### `impl Debug for __c_anonymous_ptrace_syscall_info_seccomp`

- <span id="c-anonymous-ptrace-syscall-info-seccomp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptrace_syscall_info`

```rust
struct ptrace_syscall_info {
    pub op: crate::__u8,
    pub pad: [crate::__u8; 3],
    pub arch: crate::__u32,
    pub instruction_pointer: crate::__u64,
    pub stack_pointer: crate::__u64,
    pub u: __c_anonymous_ptrace_syscall_info_data,
}
```

#### Trait Implementations

##### `impl Clone for ptrace_syscall_info`

- <span id="ptrace-syscall-info-clone"></span>`fn clone(&self) -> ptrace_syscall_info` — [`ptrace_syscall_info`](../index.md#ptrace-syscall-info)

##### `impl Copy for ptrace_syscall_info`

##### `impl Debug for ptrace_syscall_info`

- <span id="ptrace-syscall-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptrace_sud_config`

```rust
struct ptrace_sud_config {
    pub mode: crate::__u64,
    pub selector: crate::__u64,
    pub offset: crate::__u64,
    pub len: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for ptrace_sud_config`

- <span id="ptrace-sud-config-clone"></span>`fn clone(&self) -> ptrace_sud_config` — [`ptrace_sud_config`](../index.md#ptrace-sud-config)

##### `impl Copy for ptrace_sud_config`

##### `impl Debug for ptrace_sud_config`

- <span id="ptrace-sud-config-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iocb`

```rust
struct iocb {
    pub aio_data: crate::__u64,
    pub aio_key: crate::__u32,
    pub aio_rw_flags: crate::__kernel_rwf_t,
    pub aio_lio_opcode: crate::__u16,
    pub aio_reqprio: crate::__s16,
    pub aio_fildes: crate::__u32,
    pub aio_buf: crate::__u64,
    pub aio_nbytes: crate::__u64,
    pub aio_offset: crate::__s64,
    aio_reserved2: Padding<crate::__u64>,
    pub aio_flags: crate::__u32,
    pub aio_resfd: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for iocb`

- <span id="iocb-clone"></span>`fn clone(&self) -> iocb` — [`iocb`](../index.md#iocb)

##### `impl Copy for iocb`

##### `impl Debug for iocb`

- <span id="iocb-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tcp_info`

```rust
struct tcp_info {
    pub tcpi_state: u8,
    pub tcpi_ca_state: u8,
    pub tcpi_retransmits: u8,
    pub tcpi_probes: u8,
    pub tcpi_backoff: u8,
    pub tcpi_options: u8,
    pub tcpi_snd_rcv_wscale: u8,
    pub tcpi_rto: u32,
    pub tcpi_ato: u32,
    pub tcpi_snd_mss: u32,
    pub tcpi_rcv_mss: u32,
    pub tcpi_unacked: u32,
    pub tcpi_sacked: u32,
    pub tcpi_lost: u32,
    pub tcpi_retrans: u32,
    pub tcpi_fackets: u32,
    pub tcpi_last_data_sent: u32,
    pub tcpi_last_ack_sent: u32,
    pub tcpi_last_data_recv: u32,
    pub tcpi_last_ack_recv: u32,
    pub tcpi_pmtu: u32,
    pub tcpi_rcv_ssthresh: u32,
    pub tcpi_rtt: u32,
    pub tcpi_rttvar: u32,
    pub tcpi_snd_ssthresh: u32,
    pub tcpi_snd_cwnd: u32,
    pub tcpi_advmss: u32,
    pub tcpi_reordering: u32,
    pub tcpi_rcv_rtt: u32,
    pub tcpi_rcv_space: u32,
    pub tcpi_total_retrans: u32,
}
```

#### Fields

- **`tcpi_snd_rcv_wscale`**: `u8`

  This contains the bitfields `tcpi_snd_wscale` and `tcpi_rcv_wscale`.
  Each is 4 bits.

#### Trait Implementations

##### `impl Clone for tcp_info`

- <span id="tcp-info-clone"></span>`fn clone(&self) -> tcp_info` — [`tcp_info`](../index.md#tcp-info)

##### `impl Copy for tcp_info`

##### `impl Debug for tcp_info`

- <span id="tcp-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fanotify_event_info_pidfd`

```rust
struct fanotify_event_info_pidfd {
    pub hdr: crate::fanotify_event_info_header,
    pub pidfd: crate::__s32,
}
```

#### Trait Implementations

##### `impl Clone for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-clone"></span>`fn clone(&self) -> fanotify_event_info_pidfd` — [`fanotify_event_info_pidfd`](../index.md#fanotify-event-info-pidfd)

##### `impl Copy for fanotify_event_info_pidfd`

##### `impl Debug for fanotify_event_info_pidfd`

- <span id="fanotify-event-info-pidfd-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fanotify_event_info_error`

```rust
struct fanotify_event_info_error {
    pub hdr: crate::fanotify_event_info_header,
    pub error: crate::__s32,
    pub error_count: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for fanotify_event_info_error`

- <span id="fanotify-event-info-error-clone"></span>`fn clone(&self) -> fanotify_event_info_error` — [`fanotify_event_info_error`](../index.md#fanotify-event-info-error)

##### `impl Copy for fanotify_event_info_error`

##### `impl Debug for fanotify_event_info_error`

- <span id="fanotify-event-info-error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sem_t`

```rust
struct sem_t {
    __size: [c_char; 32],
}
```

#### Trait Implementations

##### `impl Clone for sem_t`

- <span id="sem-t-clone"></span>`fn clone(&self) -> sem_t` — [`sem_t`](../index.md#sem-t)

##### `impl Copy for sem_t`

##### `impl Debug for sem_t`

- <span id="sem-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mbstate_t`

```rust
struct mbstate_t {
    __count: c_int,
    __wchb: [c_char; 4],
}
```

#### Trait Implementations

##### `impl Clone for mbstate_t`

- <span id="mbstate-t-clone"></span>`fn clone(&self) -> mbstate_t` — [`mbstate_t`](../index.md#mbstate-t)

##### `impl Copy for mbstate_t`

##### `impl Debug for mbstate_t`

- <span id="mbstate-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fpos64_t`

```rust
struct fpos64_t {
    __pos: crate::off64_t,
    __state: crate::mbstate_t,
}
```

#### Trait Implementations

##### `impl Clone for fpos64_t`

- <span id="fpos64-t-clone"></span>`fn clone(&self) -> fpos64_t` — [`fpos64_t`](../index.md#fpos64-t)

##### `impl Copy for fpos64_t`

##### `impl Debug for fpos64_t`

- <span id="fpos64-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fpos_t`

```rust
struct fpos_t {
    __pos: off_t,
    __state: crate::mbstate_t,
}
```

#### Trait Implementations

##### `impl Clone for fpos_t`

- <span id="fpos-t-clone"></span>`fn clone(&self) -> fpos_t` — [`fpos_t`](../index.md#fpos-t)

##### `impl Copy for fpos_t`

##### `impl Debug for fpos_t`

- <span id="fpos-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timespec`

```rust
struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}
```

#### Trait Implementations

##### `impl Clone for timespec`

- <span id="timespec-clone"></span>`fn clone(&self) -> timespec` — [`timespec`](../index.md#timespec)

##### `impl Copy for timespec`

##### `impl Debug for timespec`

- <span id="timespec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for timespec`

- <span id="timespec-default"></span>`fn default() -> timespec` — [`timespec`](../index.md#timespec)

### `utmpx`

```rust
struct utmpx {
    pub ut_type: c_short,
    pub ut_pid: crate::pid_t,
    pub ut_line: [c_char; 32],
    pub ut_id: [c_char; 4],
    pub ut_user: [c_char; 32],
    pub ut_host: [c_char; 256],
    pub ut_exit: __exit_status,
    pub ut_session: i32,
    pub ut_tv: __timeval,
    pub ut_addr_v6: [i32; 4],
    __glibc_reserved: Padding<[c_char; 20]>,
}
```

#### Trait Implementations

##### `impl Clone for utmpx`

- <span id="utmpx-clone"></span>`fn clone(&self) -> utmpx` — [`utmpx`](../index.md#utmpx)

##### `impl Copy for utmpx`

##### `impl Debug for utmpx`

- <span id="utmpx-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sifields_sigchld`

```rust
struct sifields_sigchld {
    si_pid: crate::pid_t,
    si_uid: crate::uid_t,
    si_status: c_int,
    si_utime: c_long,
    si_stime: c_long,
}
```

#### Trait Implementations

##### `impl Clone for sifields_sigchld`

- <span id="sifields-sigchld-clone"></span>`fn clone(&self) -> sifields_sigchld` — [`sifields_sigchld`](#sifields-sigchld)

##### `impl Copy for sifields_sigchld`

##### `impl Debug for sifields_sigchld`

- <span id="sifields-sigchld-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `siginfo_f`

```rust
struct siginfo_f {
    _siginfo_base: [c_int; 3],
    sifields: sifields,
}
```

#### Trait Implementations

##### `impl Clone for siginfo_f`

- <span id="siginfo-f-clone"></span>`fn clone(&self) -> siginfo_f` — [`siginfo_f`](#siginfo-f)

##### `impl Copy for siginfo_f`

##### `impl Debug for siginfo_f`

- <span id="siginfo-f-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sigset_t`

```rust
struct sigset_t {
    __val: [u64; 16],
}
```

#### Trait Implementations

##### `impl Clone for sigset_t`

- <span id="sigset-t-clone"></span>`fn clone(&self) -> sigset_t` — [`sigset_t`](#sigset-t)

##### `impl Copy for sigset_t`

##### `impl Debug for sigset_t`

- <span id="sigset-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sysinfo`

```rust
struct sysinfo {
    pub uptime: i64,
    pub loads: [u64; 3],
    pub totalram: u64,
    pub freeram: u64,
    pub sharedram: u64,
    pub bufferram: u64,
    pub totalswap: u64,
    pub freeswap: u64,
    pub procs: c_ushort,
    pub pad: c_ushort,
    pub totalhigh: u64,
    pub freehigh: u64,
    pub mem_unit: c_uint,
    pub _f: [c_char; 0],
}
```

#### Trait Implementations

##### `impl Clone for sysinfo`

- <span id="sysinfo-clone"></span>`fn clone(&self) -> sysinfo` — [`sysinfo`](#sysinfo)

##### `impl Copy for sysinfo`

##### `impl Debug for sysinfo`

- <span id="sysinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `msqid_ds`

```rust
struct msqid_ds {
    pub msg_perm: crate::ipc_perm,
    pub msg_stime: crate::time_t,
    pub msg_rtime: crate::time_t,
    pub msg_ctime: crate::time_t,
    pub __msg_cbytes: u64,
    pub msg_qnum: crate::msgqnum_t,
    pub msg_qbytes: crate::msglen_t,
    pub msg_lspid: crate::pid_t,
    pub msg_lrpid: crate::pid_t,
    __glibc_reserved4: Padding<u64>,
    __glibc_reserved5: Padding<u64>,
}
```

#### Trait Implementations

##### `impl Clone for msqid_ds`

- <span id="msqid-ds-clone"></span>`fn clone(&self) -> msqid_ds` — [`msqid_ds`](#msqid-ds)

##### `impl Copy for msqid_ds`

##### `impl Debug for msqid_ds`

- <span id="msqid-ds-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `semid_ds`

```rust
struct semid_ds {
    pub sem_perm: ipc_perm,
    pub sem_otime: crate::time_t,
    __reserved: Padding<crate::__syscall_ulong_t>,
    pub sem_ctime: crate::time_t,
    __reserved2: Padding<crate::__syscall_ulong_t>,
    pub sem_nsems: crate::__syscall_ulong_t,
    __glibc_reserved3: Padding<crate::__syscall_ulong_t>,
    __glibc_reserved4: Padding<crate::__syscall_ulong_t>,
}
```

#### Trait Implementations

##### `impl Clone for semid_ds`

- <span id="semid-ds-clone"></span>`fn clone(&self) -> semid_ds` — [`semid_ds`](#semid-ds)

##### `impl Copy for semid_ds`

##### `impl Debug for semid_ds`

- <span id="semid-ds-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timex`

```rust
struct timex {
    pub modes: c_uint,
    pub offset: c_long,
    pub freq: c_long,
    pub maxerror: c_long,
    pub esterror: c_long,
    pub status: c_int,
    pub constant: c_long,
    pub precision: c_long,
    pub tolerance: c_long,
    pub time: crate::timeval,
    pub tick: c_long,
    pub ppsfreq: c_long,
    pub jitter: c_long,
    pub shift: c_int,
    pub stabil: c_long,
    pub jitcnt: c_long,
    pub calcnt: c_long,
    pub errcnt: c_long,
    pub stbcnt: c_long,
    pub tai: c_int,
    pub __unused1: i32,
    pub __unused2: i32,
    pub __unused3: i32,
    pub __unused4: i32,
    pub __unused5: i32,
    pub __unused6: i32,
    pub __unused7: i32,
    pub __unused8: i32,
    pub __unused9: i32,
    pub __unused10: i32,
    pub __unused11: i32,
}
```

#### Trait Implementations

##### `impl Clone for timex`

- <span id="timex-clone"></span>`fn clone(&self) -> timex` — [`timex`](#timex)

##### `impl Copy for timex`

##### `impl Debug for timex`

- <span id="timex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `fgetspent_r`

```rust
unsafe fn fgetspent_r(fp: *mut crate::FILE, spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```

### `sgetspent_r`

```rust
unsafe fn sgetspent_r(s: *const c_char, spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```

### `getspent_r`

```rust
unsafe fn getspent_r(spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```

### `qsort_r`

```rust
unsafe fn qsort_r(base: *mut c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void, *mut c_void) -> c_int>, arg: *mut c_void)
```

### `sendmmsg`

```rust
unsafe fn sendmmsg(sockfd: c_int, msgvec: *mut crate::mmsghdr, vlen: c_uint, flags: c_int) -> c_int
```

### `recvmmsg`

```rust
unsafe fn recvmmsg(sockfd: c_int, msgvec: *mut crate::mmsghdr, vlen: c_uint, flags: c_int, timeout: *mut crate::timespec) -> c_int
```

### `getrlimit64`

```rust
unsafe fn getrlimit64(resource: crate::__rlimit_resource_t, rlim: *mut crate::rlimit64) -> c_int
```

### `setrlimit64`

```rust
unsafe fn setrlimit64(resource: crate::__rlimit_resource_t, rlim: *const crate::rlimit64) -> c_int
```

### `getrlimit`

```rust
unsafe fn getrlimit(resource: crate::__rlimit_resource_t, rlim: *mut crate::rlimit) -> c_int
```

### `setrlimit`

```rust
unsafe fn setrlimit(resource: crate::__rlimit_resource_t, rlim: *const crate::rlimit) -> c_int
```

### `prlimit`

```rust
unsafe fn prlimit(pid: crate::pid_t, resource: crate::__rlimit_resource_t, new_limit: *const crate::rlimit, old_limit: *mut crate::rlimit) -> c_int
```

### `prlimit64`

```rust
unsafe fn prlimit64(pid: crate::pid_t, resource: crate::__rlimit_resource_t, new_limit: *const crate::rlimit64, old_limit: *mut crate::rlimit64) -> c_int
```

### `utmpname`

```rust
unsafe fn utmpname(file: *const c_char) -> c_int
```

### `utmpxname`

```rust
unsafe fn utmpxname(file: *const c_char) -> c_int
```

### `getutxent`

```rust
unsafe fn getutxent() -> *mut utmpx
```

### `getutxid`

```rust
unsafe fn getutxid(ut: *const utmpx) -> *mut utmpx
```

### `getutxline`

```rust
unsafe fn getutxline(ut: *const utmpx) -> *mut utmpx
```

### `pututxline`

```rust
unsafe fn pututxline(ut: *const utmpx) -> *mut utmpx
```

### `setutxent`

```rust
unsafe fn setutxent()
```

### `endutxent`

```rust
unsafe fn endutxent()
```

### `getpt`

```rust
unsafe fn getpt() -> c_int
```

### `mallopt`

```rust
unsafe fn mallopt(param: c_int, value: c_int) -> c_int
```

### `gettimeofday`

```rust
unsafe fn gettimeofday(tp: *mut crate::timeval, tz: *mut crate::timezone) -> c_int
```

### `getentropy`

```rust
unsafe fn getentropy(buf: *mut c_void, buflen: size_t) -> c_int
```

### `getrandom`

```rust
unsafe fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> ssize_t
```

### `getauxval`

```rust
unsafe fn getauxval(type_: c_ulong) -> c_ulong
```

### `adjtimex`

```rust
unsafe fn adjtimex(buf: *mut timex) -> c_int
```

### `ntp_adjtime`

```rust
unsafe fn ntp_adjtime(buf: *mut timex) -> c_int
```

### `ntp_gettime`

```rust
unsafe fn ntp_gettime(buf: *mut ntptimeval) -> c_int
```

### `clock_adjtime`

```rust
unsafe fn clock_adjtime(clk_id: crate::clockid_t, buf: *mut crate::timex) -> c_int
```

### `fanotify_mark`

```rust
unsafe fn fanotify_mark(fd: c_int, flags: c_uint, mask: u64, dirfd: c_int, path: *const c_char) -> c_int
```

### `preadv2`

```rust
unsafe fn preadv2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> ssize_t
```

### `pwritev2`

```rust
unsafe fn pwritev2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> ssize_t
```

### `preadv64v2`

```rust
unsafe fn preadv64v2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t, flags: c_int) -> ssize_t
```

### `pwritev64v2`

```rust
unsafe fn pwritev64v2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t, flags: c_int) -> ssize_t
```

### `renameat2`

```rust
unsafe fn renameat2(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_uint) -> c_int
```

### `explicit_bzero`

```rust
unsafe fn explicit_bzero(s: *mut c_void, len: size_t)
```

### `reallocarray`

```rust
unsafe fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void
```

### `ctermid`

```rust
unsafe fn ctermid(s: *mut c_char) -> *mut c_char
```

### `backtrace`

```rust
unsafe fn backtrace(buf: *mut *mut c_void, sz: c_int) -> c_int
```

### `backtrace_symbols`

```rust
unsafe fn backtrace_symbols(buffer: *const *mut c_void, len: c_int) -> *mut *mut c_char
```

### `backtrace_symbols_fd`

```rust
unsafe fn backtrace_symbols_fd(buffer: *const *mut c_void, len: c_int, fd: c_int)
```

### `glob64`

```rust
unsafe fn glob64(pattern: *const c_char, flags: c_int, errfunc: Option<fn(*const c_char, c_int) -> c_int>, pglob: *mut glob64_t) -> c_int
```

### `globfree64`

```rust
unsafe fn globfree64(pglob: *mut glob64_t)
```

### `ptrace`

```rust
unsafe fn ptrace(request: c_uint) -> c_long
```

### `pthread_attr_getaffinity_np`

```rust
unsafe fn pthread_attr_getaffinity_np(attr: *const crate::pthread_attr_t, cpusetsize: size_t, cpuset: *mut crate::cpu_set_t) -> c_int
```

### `pthread_attr_setaffinity_np`

```rust
unsafe fn pthread_attr_setaffinity_np(attr: *mut crate::pthread_attr_t, cpusetsize: size_t, cpuset: *const crate::cpu_set_t) -> c_int
```

### `getpriority`

```rust
unsafe fn getpriority(which: crate::__priority_which_t, who: crate::id_t) -> c_int
```

### `setpriority`

```rust
unsafe fn setpriority(which: crate::__priority_which_t, who: crate::id_t, prio: c_int) -> c_int
```

### `pthread_rwlockattr_getkind_np`

```rust
unsafe fn pthread_rwlockattr_getkind_np(attr: *const crate::pthread_rwlockattr_t, val: *mut c_int) -> c_int
```

### `pthread_rwlockattr_setkind_np`

```rust
unsafe fn pthread_rwlockattr_setkind_np(attr: *mut crate::pthread_rwlockattr_t, val: c_int) -> c_int
```

### `pthread_sigqueue`

```rust
unsafe fn pthread_sigqueue(thread: crate::pthread_t, sig: c_int, value: crate::sigval) -> c_int
```

### `pthread_tryjoin_np`

```rust
unsafe fn pthread_tryjoin_np(thread: crate::pthread_t, retval: *mut *mut c_void) -> c_int
```

### `pthread_timedjoin_np`

```rust
unsafe fn pthread_timedjoin_np(thread: crate::pthread_t, retval: *mut *mut c_void, abstime: *const crate::timespec) -> c_int
```

### `mallinfo`

```rust
unsafe fn mallinfo() -> crate::mallinfo
```

### `mallinfo2`

```rust
unsafe fn mallinfo2() -> crate::mallinfo2
```

### `malloc_stats`

```rust
unsafe fn malloc_stats()
```

### `malloc_info`

```rust
unsafe fn malloc_info(options: c_int, stream: *mut crate::FILE) -> c_int
```

### `malloc_usable_size`

```rust
unsafe fn malloc_usable_size(ptr: *mut c_void) -> size_t
```

### `getpwent_r`

```rust
unsafe fn getpwent_r(pwd: *mut crate::passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::passwd) -> c_int
```

### `getgrent_r`

```rust
unsafe fn getgrent_r(grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```

### `fgetpwent_r`

```rust
unsafe fn fgetpwent_r(stream: *mut crate::FILE, pwd: *mut crate::passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::passwd) -> c_int
```

### `fgetgrent_r`

```rust
unsafe fn fgetgrent_r(stream: *mut crate::FILE, grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```

### `putpwent`

```rust
unsafe fn putpwent(p: *const crate::passwd, stream: *mut crate::FILE) -> c_int
```

### `putgrent`

```rust
unsafe fn putgrent(grp: *const crate::group, stream: *mut crate::FILE) -> c_int
```

### `sethostid`

```rust
unsafe fn sethostid(hostid: c_long) -> c_int
```

### `memfd_create`

```rust
unsafe fn memfd_create(name: *const c_char, flags: c_uint) -> c_int
```

### `mlock2`

```rust
unsafe fn mlock2(addr: *const c_void, len: size_t, flags: c_uint) -> c_int
```

### `euidaccess`

```rust
unsafe fn euidaccess(pathname: *const c_char, mode: c_int) -> c_int
```

### `eaccess`

```rust
unsafe fn eaccess(pathname: *const c_char, mode: c_int) -> c_int
```

### `asctime_r`

```rust
unsafe fn asctime_r(tm: *const crate::tm, buf: *mut c_char) -> *mut c_char
```

### `ctime_r`

```rust
unsafe fn ctime_r(timep: *const time_t, buf: *mut c_char) -> *mut c_char
```

### `dirname`

```rust
unsafe fn dirname(path: *mut c_char) -> *mut c_char
```

### `posix_basename`

```rust
unsafe fn posix_basename(path: *mut c_char) -> *mut c_char
```

POSIX version of `basename(3)`, defined in `libgen.h`.

### `gnu_basename`

```rust
unsafe fn gnu_basename(path: *const c_char) -> *mut c_char
```

GNU version of `basename(3)`, defined in `string.h`.

### `dlmopen`

```rust
unsafe fn dlmopen(lmid: Lmid_t, filename: *const c_char, flag: c_int) -> *mut c_void
```

### `dlinfo`

```rust
unsafe fn dlinfo(handle: *mut c_void, request: c_int, info: *mut c_void) -> c_int
```

### `dladdr1`

```rust
unsafe fn dladdr1(addr: *const c_void, info: *mut crate::Dl_info, extra_info: *mut *mut c_void, flags: c_int) -> c_int
```

### `dlvsym`

```rust
unsafe fn dlvsym(handle: *mut c_void, symbol: *const c_char, version: *const c_char) -> *mut c_void
```

### `malloc_trim`

```rust
unsafe fn malloc_trim(__pad: size_t) -> c_int
```

### `gnu_get_libc_release`

```rust
unsafe fn gnu_get_libc_release() -> *const c_char
```

### `gnu_get_libc_version`

```rust
unsafe fn gnu_get_libc_version() -> *const c_char
```

### `posix_spawn_file_actions_addchdir_np`

```rust
unsafe fn posix_spawn_file_actions_addchdir_np(actions: *mut crate::posix_spawn_file_actions_t, path: *const c_char) -> c_int
```

### `posix_spawn_file_actions_addfchdir_np`

```rust
unsafe fn posix_spawn_file_actions_addfchdir_np(actions: *mut crate::posix_spawn_file_actions_t, fd: c_int) -> c_int
```

### `posix_spawn_file_actions_addclosefrom_np`

```rust
unsafe fn posix_spawn_file_actions_addclosefrom_np(actions: *mut crate::posix_spawn_file_actions_t, from: c_int) -> c_int
```

### `posix_spawn_file_actions_addtcsetpgrp_np`

```rust
unsafe fn posix_spawn_file_actions_addtcsetpgrp_np(actions: *mut crate::posix_spawn_file_actions_t, tcfd: c_int) -> c_int
```

### `getmntent_r`

```rust
unsafe fn getmntent_r(stream: *mut crate::FILE, mntbuf: *mut crate::mntent, buf: *mut c_char, buflen: c_int) -> *mut crate::mntent
```

### `execveat`

```rust
unsafe fn execveat(dirfd: c_int, pathname: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char, flags: c_int) -> c_int
```

### `close_range`

```rust
unsafe fn close_range(first: c_uint, last: c_uint, flags: c_int) -> c_int
```

### `mq_notify`

```rust
unsafe fn mq_notify(mqdes: crate::mqd_t, sevp: *const crate::sigevent) -> c_int
```

### `epoll_pwait2`

```rust
unsafe fn epoll_pwait2(epfd: c_int, events: *mut crate::epoll_event, maxevents: c_int, timeout: *const crate::timespec, sigmask: *const crate::sigset_t) -> c_int
```

### `mempcpy`

```rust
unsafe fn mempcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

### `tgkill`

```rust
unsafe fn tgkill(tgid: crate::pid_t, tid: crate::pid_t, sig: c_int) -> c_int
```

## Type Aliases

### `pthread_t`

```rust
type pthread_t = c_ulong;
```

### `__priority_which_t`

```rust
type __priority_which_t = c_uint;
```

### `__rlimit_resource_t`

```rust
type __rlimit_resource_t = c_uint;
```

### `Lmid_t`

```rust
type Lmid_t = c_long;
```

### `regoff_t`

```rust
type regoff_t = c_int;
```

### `__kernel_rwf_t`

```rust
type __kernel_rwf_t = c_int;
```

### `Ioctl`

```rust
type Ioctl = c_ulong;
```

### `ino_t`

```rust
type ino_t = u64;
```

### `off_t`

```rust
type off_t = i64;
```

### `blkcnt_t`

```rust
type blkcnt_t = i64;
```

### `shmatt_t`

```rust
type shmatt_t = u64;
```

### `msgqnum_t`

```rust
type msgqnum_t = u64;
```

### `msglen_t`

```rust
type msglen_t = u64;
```

### `fsblkcnt_t`

```rust
type fsblkcnt_t = u64;
```

### `fsfilcnt_t`

```rust
type fsfilcnt_t = u64;
```

### `rlim_t`

```rust
type rlim_t = u64;
```

### `__syscall_ulong_t`

```rust
type __syscall_ulong_t = c_ulong;
```

### `__fsword_t`

```rust
type __fsword_t = i64;
```

### `clock_t`

```rust
type clock_t = i64;
```

### `time_t`

```rust
type time_t = i64;
```

## Constants

### `HUGETLB_FLAG_ENCODE_SHIFT`
```rust
const HUGETLB_FLAG_ENCODE_SHIFT: c_int = 26i32;
```

### `HUGETLB_FLAG_ENCODE_MASK`
```rust
const HUGETLB_FLAG_ENCODE_MASK: c_int = 63i32;
```

### `HUGETLB_FLAG_ENCODE_64KB`
```rust
const HUGETLB_FLAG_ENCODE_64KB: c_int = 1_073_741_824i32;
```

### `HUGETLB_FLAG_ENCODE_512KB`
```rust
const HUGETLB_FLAG_ENCODE_512KB: c_int = 1_275_068_416i32;
```

### `HUGETLB_FLAG_ENCODE_1MB`
```rust
const HUGETLB_FLAG_ENCODE_1MB: c_int = 1_342_177_280i32;
```

### `HUGETLB_FLAG_ENCODE_2MB`
```rust
const HUGETLB_FLAG_ENCODE_2MB: c_int = 1_409_286_144i32;
```

### `HUGETLB_FLAG_ENCODE_8MB`
```rust
const HUGETLB_FLAG_ENCODE_8MB: c_int = 1_543_503_872i32;
```

### `HUGETLB_FLAG_ENCODE_16MB`
```rust
const HUGETLB_FLAG_ENCODE_16MB: c_int = 1_610_612_736i32;
```

### `HUGETLB_FLAG_ENCODE_32MB`
```rust
const HUGETLB_FLAG_ENCODE_32MB: c_int = 1_677_721_600i32;
```

### `HUGETLB_FLAG_ENCODE_256MB`
```rust
const HUGETLB_FLAG_ENCODE_256MB: c_int = 1_879_048_192i32;
```

### `HUGETLB_FLAG_ENCODE_512MB`
```rust
const HUGETLB_FLAG_ENCODE_512MB: c_int = 1_946_157_056i32;
```

### `HUGETLB_FLAG_ENCODE_1GB`
```rust
const HUGETLB_FLAG_ENCODE_1GB: c_int = 2_013_265_920i32;
```

### `HUGETLB_FLAG_ENCODE_2GB`
```rust
const HUGETLB_FLAG_ENCODE_2GB: c_int = 2_080_374_784i32;
```

### `HUGETLB_FLAG_ENCODE_16GB`
```rust
const HUGETLB_FLAG_ENCODE_16GB: c_int = -2_013_265_920i32;
```

### `MAP_HUGE_SHIFT`
```rust
const MAP_HUGE_SHIFT: c_int = 26i32;
```

### `MAP_HUGE_MASK`
```rust
const MAP_HUGE_MASK: c_int = 63i32;
```

### `MAP_HUGE_64KB`
```rust
const MAP_HUGE_64KB: c_int = 1_073_741_824i32;
```

### `MAP_HUGE_512KB`
```rust
const MAP_HUGE_512KB: c_int = 1_275_068_416i32;
```

### `MAP_HUGE_1MB`
```rust
const MAP_HUGE_1MB: c_int = 1_342_177_280i32;
```

### `MAP_HUGE_2MB`
```rust
const MAP_HUGE_2MB: c_int = 1_409_286_144i32;
```

### `MAP_HUGE_8MB`
```rust
const MAP_HUGE_8MB: c_int = 1_543_503_872i32;
```

### `MAP_HUGE_16MB`
```rust
const MAP_HUGE_16MB: c_int = 1_610_612_736i32;
```

### `MAP_HUGE_32MB`
```rust
const MAP_HUGE_32MB: c_int = 1_677_721_600i32;
```

### `MAP_HUGE_256MB`
```rust
const MAP_HUGE_256MB: c_int = 1_879_048_192i32;
```

### `MAP_HUGE_512MB`
```rust
const MAP_HUGE_512MB: c_int = 1_946_157_056i32;
```

### `MAP_HUGE_1GB`
```rust
const MAP_HUGE_1GB: c_int = 2_013_265_920i32;
```

### `MAP_HUGE_2GB`
```rust
const MAP_HUGE_2GB: c_int = 2_080_374_784i32;
```

### `MAP_HUGE_16GB`
```rust
const MAP_HUGE_16GB: c_int = -2_013_265_920i32;
```

### `PRIO_PROCESS`
```rust
const PRIO_PROCESS: crate::__priority_which_t = 0u32;
```

### `PRIO_PGRP`
```rust
const PRIO_PGRP: crate::__priority_which_t = 1u32;
```

### `PRIO_USER`
```rust
const PRIO_USER: crate::__priority_which_t = 2u32;
```

### `MS_RMT_MASK`
```rust
const MS_RMT_MASK: c_ulong = 41_943_121u64;
```

### `__UT_LINESIZE`
```rust
const __UT_LINESIZE: usize = 32usize;
```

### `__UT_NAMESIZE`
```rust
const __UT_NAMESIZE: usize = 32usize;
```

### `__UT_HOSTSIZE`
```rust
const __UT_HOSTSIZE: usize = 256usize;
```

### `EMPTY`
```rust
const EMPTY: c_short = 0i16;
```

### `RUN_LVL`
```rust
const RUN_LVL: c_short = 1i16;
```

### `BOOT_TIME`
```rust
const BOOT_TIME: c_short = 2i16;
```

### `NEW_TIME`
```rust
const NEW_TIME: c_short = 3i16;
```

### `OLD_TIME`
```rust
const OLD_TIME: c_short = 4i16;
```

### `INIT_PROCESS`
```rust
const INIT_PROCESS: c_short = 5i16;
```

### `LOGIN_PROCESS`
```rust
const LOGIN_PROCESS: c_short = 6i16;
```

### `USER_PROCESS`
```rust
const USER_PROCESS: c_short = 7i16;
```

### `DEAD_PROCESS`
```rust
const DEAD_PROCESS: c_short = 8i16;
```

### `ACCOUNTING`
```rust
const ACCOUNTING: c_short = 9i16;
```

### `LM_ID_BASE`
```rust
const LM_ID_BASE: c_long = 0i64;
```

### `LM_ID_NEWLM`
```rust
const LM_ID_NEWLM: c_long = -1i64;
```

### `RTLD_DI_LMID`
```rust
const RTLD_DI_LMID: c_int = 1i32;
```

### `RTLD_DI_LINKMAP`
```rust
const RTLD_DI_LINKMAP: c_int = 2i32;
```

### `RTLD_DI_CONFIGADDR`
```rust
const RTLD_DI_CONFIGADDR: c_int = 3i32;
```

### `RTLD_DI_SERINFO`
```rust
const RTLD_DI_SERINFO: c_int = 4i32;
```

### `RTLD_DI_SERINFOSIZE`
```rust
const RTLD_DI_SERINFOSIZE: c_int = 5i32;
```

### `RTLD_DI_ORIGIN`
```rust
const RTLD_DI_ORIGIN: c_int = 6i32;
```

### `RTLD_DI_PROFILENAME`
```rust
const RTLD_DI_PROFILENAME: c_int = 7i32;
```

### `RTLD_DI_PROFILEOUT`
```rust
const RTLD_DI_PROFILEOUT: c_int = 8i32;
```

### `RTLD_DI_TLS_MODID`
```rust
const RTLD_DI_TLS_MODID: c_int = 9i32;
```

### `RTLD_DI_TLS_DATA`
```rust
const RTLD_DI_TLS_DATA: c_int = 10i32;
```

### `SOCK_NONBLOCK`
```rust
const SOCK_NONBLOCK: c_int = 2_048i32;
```

### `SOL_RXRPC`
```rust
const SOL_RXRPC: c_int = 272i32;
```

### `SOL_PPPOL2TP`
```rust
const SOL_PPPOL2TP: c_int = 273i32;
```

### `SOL_PNPIPE`
```rust
const SOL_PNPIPE: c_int = 275i32;
```

### `SOL_RDS`
```rust
const SOL_RDS: c_int = 276i32;
```

### `SOL_IUCV`
```rust
const SOL_IUCV: c_int = 277i32;
```

### `SOL_CAIF`
```rust
const SOL_CAIF: c_int = 278i32;
```

### `SOL_NFC`
```rust
const SOL_NFC: c_int = 280i32;
```

### `MSG_TRYHARD`
```rust
const MSG_TRYHARD: c_int = 4i32;
```

### `LC_PAPER`
```rust
const LC_PAPER: c_int = 7i32;
```

### `LC_NAME`
```rust
const LC_NAME: c_int = 8i32;
```

### `LC_ADDRESS`
```rust
const LC_ADDRESS: c_int = 9i32;
```

### `LC_TELEPHONE`
```rust
const LC_TELEPHONE: c_int = 10i32;
```

### `LC_MEASUREMENT`
```rust
const LC_MEASUREMENT: c_int = 11i32;
```

### `LC_IDENTIFICATION`
```rust
const LC_IDENTIFICATION: c_int = 12i32;
```

### `LC_PAPER_MASK`
```rust
const LC_PAPER_MASK: c_int = 128i32;
```

### `LC_NAME_MASK`
```rust
const LC_NAME_MASK: c_int = 256i32;
```

### `LC_ADDRESS_MASK`
```rust
const LC_ADDRESS_MASK: c_int = 512i32;
```

### `LC_TELEPHONE_MASK`
```rust
const LC_TELEPHONE_MASK: c_int = 1_024i32;
```

### `LC_MEASUREMENT_MASK`
```rust
const LC_MEASUREMENT_MASK: c_int = 2_048i32;
```

### `LC_IDENTIFICATION_MASK`
```rust
const LC_IDENTIFICATION_MASK: c_int = 4_096i32;
```

### `LC_ALL_MASK`
```rust
const LC_ALL_MASK: c_int = 8_127i32;
```

### `ENOTSUP`
```rust
const ENOTSUP: c_int = 95i32;
```

### `SOCK_SEQPACKET`
```rust
const SOCK_SEQPACKET: c_int = 5i32;
```

### `SOCK_DCCP`
```rust
const SOCK_DCCP: c_int = 6i32;
```

### `SOCK_PACKET`
```rust
const SOCK_PACKET: c_int = 10i32;
```

### `AF_IB`
```rust
const AF_IB: c_int = 27i32;
```

### `AF_MPLS`
```rust
const AF_MPLS: c_int = 28i32;
```

### `AF_NFC`
```rust
const AF_NFC: c_int = 39i32;
```

### `AF_VSOCK`
```rust
const AF_VSOCK: c_int = 40i32;
```

### `AF_XDP`
```rust
const AF_XDP: c_int = 44i32;
```

### `PF_IB`
```rust
const PF_IB: c_int = 27i32;
```

### `PF_MPLS`
```rust
const PF_MPLS: c_int = 28i32;
```

### `PF_NFC`
```rust
const PF_NFC: c_int = 39i32;
```

### `PF_VSOCK`
```rust
const PF_VSOCK: c_int = 40i32;
```

### `PF_XDP`
```rust
const PF_XDP: c_int = 44i32;
```

### `SIGEV_THREAD_ID`
```rust
const SIGEV_THREAD_ID: c_int = 4i32;
```

### `BUFSIZ`
```rust
const BUFSIZ: c_uint = 8_192u32;
```

### `TMP_MAX`
```rust
const TMP_MAX: c_uint = 238_328u32;
```

### `FOPEN_MAX`
```rust
const FOPEN_MAX: c_uint = 16u32;
```

### `FILENAME_MAX`
```rust
const FILENAME_MAX: c_uint = 4_096u32;
```

### `_CS_GNU_LIBC_VERSION`
```rust
const _CS_GNU_LIBC_VERSION: c_int = 2i32;
```

### `_CS_GNU_LIBPTHREAD_VERSION`
```rust
const _CS_GNU_LIBPTHREAD_VERSION: c_int = 3i32;
```

### `_CS_V6_ENV`
```rust
const _CS_V6_ENV: c_int = 1_148i32;
```

### `_CS_V7_ENV`
```rust
const _CS_V7_ENV: c_int = 1_149i32;
```

### `_SC_EQUIV_CLASS_MAX`
```rust
const _SC_EQUIV_CLASS_MAX: c_int = 41i32;
```

### `_SC_CHARCLASS_NAME_MAX`
```rust
const _SC_CHARCLASS_NAME_MAX: c_int = 45i32;
```

### `_SC_PII`
```rust
const _SC_PII: c_int = 53i32;
```

### `_SC_PII_XTI`
```rust
const _SC_PII_XTI: c_int = 54i32;
```

### `_SC_PII_SOCKET`
```rust
const _SC_PII_SOCKET: c_int = 55i32;
```

### `_SC_PII_INTERNET`
```rust
const _SC_PII_INTERNET: c_int = 56i32;
```

### `_SC_PII_OSI`
```rust
const _SC_PII_OSI: c_int = 57i32;
```

### `_SC_POLL`
```rust
const _SC_POLL: c_int = 58i32;
```

### `_SC_SELECT`
```rust
const _SC_SELECT: c_int = 59i32;
```

### `_SC_PII_INTERNET_STREAM`
```rust
const _SC_PII_INTERNET_STREAM: c_int = 61i32;
```

### `_SC_PII_INTERNET_DGRAM`
```rust
const _SC_PII_INTERNET_DGRAM: c_int = 62i32;
```

### `_SC_PII_OSI_COTS`
```rust
const _SC_PII_OSI_COTS: c_int = 63i32;
```

### `_SC_PII_OSI_CLTS`
```rust
const _SC_PII_OSI_CLTS: c_int = 64i32;
```

### `_SC_PII_OSI_M`
```rust
const _SC_PII_OSI_M: c_int = 65i32;
```

### `_SC_T_IOV_MAX`
```rust
const _SC_T_IOV_MAX: c_int = 66i32;
```

### `_SC_2_C_VERSION`
```rust
const _SC_2_C_VERSION: c_int = 96i32;
```

### `_SC_CHAR_BIT`
```rust
const _SC_CHAR_BIT: c_int = 101i32;
```

### `_SC_CHAR_MAX`
```rust
const _SC_CHAR_MAX: c_int = 102i32;
```

### `_SC_CHAR_MIN`
```rust
const _SC_CHAR_MIN: c_int = 103i32;
```

### `_SC_INT_MAX`
```rust
const _SC_INT_MAX: c_int = 104i32;
```

### `_SC_INT_MIN`
```rust
const _SC_INT_MIN: c_int = 105i32;
```

### `_SC_LONG_BIT`
```rust
const _SC_LONG_BIT: c_int = 106i32;
```

### `_SC_WORD_BIT`
```rust
const _SC_WORD_BIT: c_int = 107i32;
```

### `_SC_MB_LEN_MAX`
```rust
const _SC_MB_LEN_MAX: c_int = 108i32;
```

### `_SC_SSIZE_MAX`
```rust
const _SC_SSIZE_MAX: c_int = 110i32;
```

### `_SC_SCHAR_MAX`
```rust
const _SC_SCHAR_MAX: c_int = 111i32;
```

### `_SC_SCHAR_MIN`
```rust
const _SC_SCHAR_MIN: c_int = 112i32;
```

### `_SC_SHRT_MAX`
```rust
const _SC_SHRT_MAX: c_int = 113i32;
```

### `_SC_SHRT_MIN`
```rust
const _SC_SHRT_MIN: c_int = 114i32;
```

### `_SC_UCHAR_MAX`
```rust
const _SC_UCHAR_MAX: c_int = 115i32;
```

### `_SC_UINT_MAX`
```rust
const _SC_UINT_MAX: c_int = 116i32;
```

### `_SC_ULONG_MAX`
```rust
const _SC_ULONG_MAX: c_int = 117i32;
```

### `_SC_USHRT_MAX`
```rust
const _SC_USHRT_MAX: c_int = 118i32;
```

### `_SC_NL_ARGMAX`
```rust
const _SC_NL_ARGMAX: c_int = 119i32;
```

### `_SC_NL_LANGMAX`
```rust
const _SC_NL_LANGMAX: c_int = 120i32;
```

### `_SC_NL_MSGMAX`
```rust
const _SC_NL_MSGMAX: c_int = 121i32;
```

### `_SC_NL_NMAX`
```rust
const _SC_NL_NMAX: c_int = 122i32;
```

### `_SC_NL_SETMAX`
```rust
const _SC_NL_SETMAX: c_int = 123i32;
```

### `_SC_NL_TEXTMAX`
```rust
const _SC_NL_TEXTMAX: c_int = 124i32;
```

### `_SC_BASE`
```rust
const _SC_BASE: c_int = 134i32;
```

### `_SC_C_LANG_SUPPORT`
```rust
const _SC_C_LANG_SUPPORT: c_int = 135i32;
```

### `_SC_C_LANG_SUPPORT_R`
```rust
const _SC_C_LANG_SUPPORT_R: c_int = 136i32;
```

### `_SC_DEVICE_IO`
```rust
const _SC_DEVICE_IO: c_int = 140i32;
```

### `_SC_DEVICE_SPECIFIC`
```rust
const _SC_DEVICE_SPECIFIC: c_int = 141i32;
```

### `_SC_DEVICE_SPECIFIC_R`
```rust
const _SC_DEVICE_SPECIFIC_R: c_int = 142i32;
```

### `_SC_FD_MGMT`
```rust
const _SC_FD_MGMT: c_int = 143i32;
```

### `_SC_FIFO`
```rust
const _SC_FIFO: c_int = 144i32;
```

### `_SC_PIPE`
```rust
const _SC_PIPE: c_int = 145i32;
```

### `_SC_FILE_ATTRIBUTES`
```rust
const _SC_FILE_ATTRIBUTES: c_int = 146i32;
```

### `_SC_FILE_LOCKING`
```rust
const _SC_FILE_LOCKING: c_int = 147i32;
```

### `_SC_FILE_SYSTEM`
```rust
const _SC_FILE_SYSTEM: c_int = 148i32;
```

### `_SC_MULTI_PROCESS`
```rust
const _SC_MULTI_PROCESS: c_int = 150i32;
```

### `_SC_SINGLE_PROCESS`
```rust
const _SC_SINGLE_PROCESS: c_int = 151i32;
```

### `_SC_NETWORKING`
```rust
const _SC_NETWORKING: c_int = 152i32;
```

### `_SC_REGEX_VERSION`
```rust
const _SC_REGEX_VERSION: c_int = 156i32;
```

### `_SC_SIGNALS`
```rust
const _SC_SIGNALS: c_int = 158i32;
```

### `_SC_SYSTEM_DATABASE`
```rust
const _SC_SYSTEM_DATABASE: c_int = 162i32;
```

### `_SC_SYSTEM_DATABASE_R`
```rust
const _SC_SYSTEM_DATABASE_R: c_int = 163i32;
```

### `_SC_USER_GROUPS`
```rust
const _SC_USER_GROUPS: c_int = 166i32;
```

### `_SC_USER_GROUPS_R`
```rust
const _SC_USER_GROUPS_R: c_int = 167i32;
```

### `_SC_LEVEL1_ICACHE_SIZE`
```rust
const _SC_LEVEL1_ICACHE_SIZE: c_int = 185i32;
```

### `_SC_LEVEL1_ICACHE_ASSOC`
```rust
const _SC_LEVEL1_ICACHE_ASSOC: c_int = 186i32;
```

### `_SC_LEVEL1_ICACHE_LINESIZE`
```rust
const _SC_LEVEL1_ICACHE_LINESIZE: c_int = 187i32;
```

### `_SC_LEVEL1_DCACHE_SIZE`
```rust
const _SC_LEVEL1_DCACHE_SIZE: c_int = 188i32;
```

### `_SC_LEVEL1_DCACHE_ASSOC`
```rust
const _SC_LEVEL1_DCACHE_ASSOC: c_int = 189i32;
```

### `_SC_LEVEL1_DCACHE_LINESIZE`
```rust
const _SC_LEVEL1_DCACHE_LINESIZE: c_int = 190i32;
```

### `_SC_LEVEL2_CACHE_SIZE`
```rust
const _SC_LEVEL2_CACHE_SIZE: c_int = 191i32;
```

### `_SC_LEVEL2_CACHE_ASSOC`
```rust
const _SC_LEVEL2_CACHE_ASSOC: c_int = 192i32;
```

### `_SC_LEVEL2_CACHE_LINESIZE`
```rust
const _SC_LEVEL2_CACHE_LINESIZE: c_int = 193i32;
```

### `_SC_LEVEL3_CACHE_SIZE`
```rust
const _SC_LEVEL3_CACHE_SIZE: c_int = 194i32;
```

### `_SC_LEVEL3_CACHE_ASSOC`
```rust
const _SC_LEVEL3_CACHE_ASSOC: c_int = 195i32;
```

### `_SC_LEVEL3_CACHE_LINESIZE`
```rust
const _SC_LEVEL3_CACHE_LINESIZE: c_int = 196i32;
```

### `_SC_LEVEL4_CACHE_SIZE`
```rust
const _SC_LEVEL4_CACHE_SIZE: c_int = 197i32;
```

### `_SC_LEVEL4_CACHE_ASSOC`
```rust
const _SC_LEVEL4_CACHE_ASSOC: c_int = 198i32;
```

### `_SC_LEVEL4_CACHE_LINESIZE`
```rust
const _SC_LEVEL4_CACHE_LINESIZE: c_int = 199i32;
```

### `O_ACCMODE`
```rust
const O_ACCMODE: c_int = 3i32;
```

### `ST_RELATIME`
```rust
const ST_RELATIME: c_ulong = 4_096u64;
```

### `NI_MAXHOST`
```rust
const NI_MAXHOST: crate::socklen_t = 1_025u32;
```

### `BINDERFS_SUPER_MAGIC`
```rust
const BINDERFS_SUPER_MAGIC: c_long = 1_819_242_352i64;
```

### `XFS_SUPER_MAGIC`
```rust
const XFS_SUPER_MAGIC: c_long = 1_481_003_842i64;
```

### `CPU_SETSIZE`
```rust
const CPU_SETSIZE: c_int = 1_024i32;
```

### `PTRACE_TRACEME`
```rust
const PTRACE_TRACEME: c_uint = 0u32;
```

### `PTRACE_PEEKTEXT`
```rust
const PTRACE_PEEKTEXT: c_uint = 1u32;
```

### `PTRACE_PEEKDATA`
```rust
const PTRACE_PEEKDATA: c_uint = 2u32;
```

### `PTRACE_PEEKUSER`
```rust
const PTRACE_PEEKUSER: c_uint = 3u32;
```

### `PTRACE_POKETEXT`
```rust
const PTRACE_POKETEXT: c_uint = 4u32;
```

### `PTRACE_POKEDATA`
```rust
const PTRACE_POKEDATA: c_uint = 5u32;
```

### `PTRACE_POKEUSER`
```rust
const PTRACE_POKEUSER: c_uint = 6u32;
```

### `PTRACE_CONT`
```rust
const PTRACE_CONT: c_uint = 7u32;
```

### `PTRACE_KILL`
```rust
const PTRACE_KILL: c_uint = 8u32;
```

### `PTRACE_SINGLESTEP`
```rust
const PTRACE_SINGLESTEP: c_uint = 9u32;
```

### `PTRACE_ATTACH`
```rust
const PTRACE_ATTACH: c_uint = 16u32;
```

### `PTRACE_SYSCALL`
```rust
const PTRACE_SYSCALL: c_uint = 24u32;
```

### `PTRACE_SETOPTIONS`
```rust
const PTRACE_SETOPTIONS: c_uint = 16_896u32;
```

### `PTRACE_GETEVENTMSG`
```rust
const PTRACE_GETEVENTMSG: c_uint = 16_897u32;
```

### `PTRACE_GETSIGINFO`
```rust
const PTRACE_GETSIGINFO: c_uint = 16_898u32;
```

### `PTRACE_SETSIGINFO`
```rust
const PTRACE_SETSIGINFO: c_uint = 16_899u32;
```

### `PTRACE_GETREGSET`
```rust
const PTRACE_GETREGSET: c_uint = 16_900u32;
```

### `PTRACE_SETREGSET`
```rust
const PTRACE_SETREGSET: c_uint = 16_901u32;
```

### `PTRACE_SEIZE`
```rust
const PTRACE_SEIZE: c_uint = 16_902u32;
```

### `PTRACE_INTERRUPT`
```rust
const PTRACE_INTERRUPT: c_uint = 16_903u32;
```

### `PTRACE_LISTEN`
```rust
const PTRACE_LISTEN: c_uint = 16_904u32;
```

### `PTRACE_PEEKSIGINFO`
```rust
const PTRACE_PEEKSIGINFO: c_uint = 16_905u32;
```

### `PTRACE_GETSIGMASK`
```rust
const PTRACE_GETSIGMASK: c_uint = 16_906u32;
```

### `PTRACE_SETSIGMASK`
```rust
const PTRACE_SETSIGMASK: c_uint = 16_907u32;
```

### `PTRACE_GET_SYSCALL_INFO`
```rust
const PTRACE_GET_SYSCALL_INFO: c_uint = 16_910u32;
```

### `PTRACE_SET_SYSCALL_INFO`
```rust
const PTRACE_SET_SYSCALL_INFO: c_uint = 16_914u32;
```

### `PTRACE_SYSCALL_INFO_NONE`
```rust
const PTRACE_SYSCALL_INFO_NONE: crate::__u8 = 0u8;
```

### `PTRACE_SYSCALL_INFO_ENTRY`
```rust
const PTRACE_SYSCALL_INFO_ENTRY: crate::__u8 = 1u8;
```

### `PTRACE_SYSCALL_INFO_EXIT`
```rust
const PTRACE_SYSCALL_INFO_EXIT: crate::__u8 = 2u8;
```

### `PTRACE_SYSCALL_INFO_SECCOMP`
```rust
const PTRACE_SYSCALL_INFO_SECCOMP: crate::__u8 = 3u8;
```

### `PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG`
```rust
const PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG: crate::__u8 = 16u8;
```

### `PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG`
```rust
const PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG: crate::__u8 = 17u8;
```

### `TCA_PAD`
```rust
const TCA_PAD: c_ushort = 9u16;
```

### `TCA_DUMP_INVISIBLE`
```rust
const TCA_DUMP_INVISIBLE: c_ushort = 10u16;
```

### `TCA_CHAIN`
```rust
const TCA_CHAIN: c_ushort = 11u16;
```

### `TCA_HW_OFFLOAD`
```rust
const TCA_HW_OFFLOAD: c_ushort = 12u16;
```

### `RTM_DELNETCONF`
```rust
const RTM_DELNETCONF: u16 = 81u16;
```

### `RTM_NEWSTATS`
```rust
const RTM_NEWSTATS: u16 = 92u16;
```

### `RTM_GETSTATS`
```rust
const RTM_GETSTATS: u16 = 94u16;
```

### `RTM_NEWCACHEREPORT`
```rust
const RTM_NEWCACHEREPORT: u16 = 96u16;
```

### `RTM_F_LOOKUP_TABLE`
```rust
const RTM_F_LOOKUP_TABLE: c_uint = 4_096u32;
```

### `RTM_F_FIB_MATCH`
```rust
const RTM_F_FIB_MATCH: c_uint = 8_192u32;
```

### `RTA_VIA`
```rust
const RTA_VIA: c_ushort = 18u16;
```

### `RTA_NEWDST`
```rust
const RTA_NEWDST: c_ushort = 19u16;
```

### `RTA_PREF`
```rust
const RTA_PREF: c_ushort = 20u16;
```

### `RTA_ENCAP_TYPE`
```rust
const RTA_ENCAP_TYPE: c_ushort = 21u16;
```

### `RTA_ENCAP`
```rust
const RTA_ENCAP: c_ushort = 22u16;
```

### `RTA_EXPIRES`
```rust
const RTA_EXPIRES: c_ushort = 23u16;
```

### `RTA_PAD`
```rust
const RTA_PAD: c_ushort = 24u16;
```

### `RTA_UID`
```rust
const RTA_UID: c_ushort = 25u16;
```

### `RTA_TTL_PROPAGATE`
```rust
const RTA_TTL_PROPAGATE: c_ushort = 26u16;
```

### `NTF_EXT_LEARNED`
```rust
const NTF_EXT_LEARNED: u8 = 16u8;
```

### `NTF_OFFLOADED`
```rust
const NTF_OFFLOADED: u8 = 32u8;
```

### `NDA_MASTER`
```rust
const NDA_MASTER: c_ushort = 9u16;
```

### `NDA_LINK_NETNSID`
```rust
const NDA_LINK_NETNSID: c_ushort = 10u16;
```

### `NDA_SRC_VNI`
```rust
const NDA_SRC_VNI: c_ushort = 11u16;
```

### `UNAME26`
```rust
const UNAME26: c_int = 131_072i32;
```

### `FDPIC_FUNCPTRS`
```rust
const FDPIC_FUNCPTRS: c_int = 524_288i32;
```

### `GENL_UNS_ADMIN_PERM`
```rust
const GENL_UNS_ADMIN_PERM: c_int = 16i32;
```

### `GENL_ID_VFS_DQUOT`
```rust
const GENL_ID_VFS_DQUOT: c_int = 17i32;
```

### `GENL_ID_PMCRAID`
```rust
const GENL_ID_PMCRAID: c_int = 18i32;
```

### `ELFOSABI_ARM_AEABI`
```rust
const ELFOSABI_ARM_AEABI: u8 = 64u8;
```

### `CLONE_NEWTIME`
```rust
const CLONE_NEWTIME: c_int = 128i32;
```

### `CLONE_CLEAR_SIGHAND`
```rust
const CLONE_CLEAR_SIGHAND: c_int = 0i32;
```

### `CLONE_INTO_CGROUP`
```rust
const CLONE_INTO_CGROUP: c_int = 0i32;
```

### `M_MXFAST`
```rust
const M_MXFAST: c_int = 1i32;
```

### `M_NLBLKS`
```rust
const M_NLBLKS: c_int = 2i32;
```

### `M_GRAIN`
```rust
const M_GRAIN: c_int = 3i32;
```

### `M_KEEP`
```rust
const M_KEEP: c_int = 4i32;
```

### `M_TRIM_THRESHOLD`
```rust
const M_TRIM_THRESHOLD: c_int = -1i32;
```

### `M_TOP_PAD`
```rust
const M_TOP_PAD: c_int = -2i32;
```

### `M_MMAP_THRESHOLD`
```rust
const M_MMAP_THRESHOLD: c_int = -3i32;
```

### `M_MMAP_MAX`
```rust
const M_MMAP_MAX: c_int = -4i32;
```

### `M_CHECK_ACTION`
```rust
const M_CHECK_ACTION: c_int = -5i32;
```

### `M_PERTURB`
```rust
const M_PERTURB: c_int = -6i32;
```

### `M_ARENA_TEST`
```rust
const M_ARENA_TEST: c_int = -7i32;
```

### `M_ARENA_MAX`
```rust
const M_ARENA_MAX: c_int = -8i32;
```

### `SOMAXCONN`
```rust
const SOMAXCONN: c_int = 4_096i32;
```

### `MOVE_MOUNT_F_SYMLINKS`
```rust
const MOVE_MOUNT_F_SYMLINKS: c_uint = 1u32;
```

### `MOVE_MOUNT_F_AUTOMOUNTS`
```rust
const MOVE_MOUNT_F_AUTOMOUNTS: c_uint = 2u32;
```

### `MOVE_MOUNT_F_EMPTY_PATH`
```rust
const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 4u32;
```

### `MOVE_MOUNT_T_SYMLINKS`
```rust
const MOVE_MOUNT_T_SYMLINKS: c_uint = 16u32;
```

### `MOVE_MOUNT_T_AUTOMOUNTS`
```rust
const MOVE_MOUNT_T_AUTOMOUNTS: c_uint = 32u32;
```

### `MOVE_MOUNT_T_EMPTY_PATH`
```rust
const MOVE_MOUNT_T_EMPTY_PATH: c_uint = 64u32;
```

### `MOVE_MOUNT_SET_GROUP`
```rust
const MOVE_MOUNT_SET_GROUP: c_uint = 256u32;
```

### `MOVE_MOUNT_BENEATH`
```rust
const MOVE_MOUNT_BENEATH: c_uint = 512u32;
```

### `ADJ_OFFSET`
```rust
const ADJ_OFFSET: c_uint = 1u32;
```

### `ADJ_FREQUENCY`
```rust
const ADJ_FREQUENCY: c_uint = 2u32;
```

### `ADJ_MAXERROR`
```rust
const ADJ_MAXERROR: c_uint = 4u32;
```

### `ADJ_ESTERROR`
```rust
const ADJ_ESTERROR: c_uint = 8u32;
```

### `ADJ_STATUS`
```rust
const ADJ_STATUS: c_uint = 16u32;
```

### `ADJ_TIMECONST`
```rust
const ADJ_TIMECONST: c_uint = 32u32;
```

### `ADJ_TAI`
```rust
const ADJ_TAI: c_uint = 128u32;
```

### `ADJ_SETOFFSET`
```rust
const ADJ_SETOFFSET: c_uint = 256u32;
```

### `ADJ_MICRO`
```rust
const ADJ_MICRO: c_uint = 4_096u32;
```

### `ADJ_NANO`
```rust
const ADJ_NANO: c_uint = 8_192u32;
```

### `ADJ_TICK`
```rust
const ADJ_TICK: c_uint = 16_384u32;
```

### `ADJ_OFFSET_SINGLESHOT`
```rust
const ADJ_OFFSET_SINGLESHOT: c_uint = 32_769u32;
```

### `ADJ_OFFSET_SS_READ`
```rust
const ADJ_OFFSET_SS_READ: c_uint = 40_961u32;
```

### `MOD_OFFSET`
```rust
const MOD_OFFSET: c_uint = 1u32;
```

### `MOD_FREQUENCY`
```rust
const MOD_FREQUENCY: c_uint = 2u32;
```

### `MOD_MAXERROR`
```rust
const MOD_MAXERROR: c_uint = 4u32;
```

### `MOD_ESTERROR`
```rust
const MOD_ESTERROR: c_uint = 8u32;
```

### `MOD_STATUS`
```rust
const MOD_STATUS: c_uint = 16u32;
```

### `MOD_TIMECONST`
```rust
const MOD_TIMECONST: c_uint = 32u32;
```

### `MOD_CLKB`
```rust
const MOD_CLKB: c_uint = 16_384u32;
```

### `MOD_CLKA`
```rust
const MOD_CLKA: c_uint = 32_769u32;
```

### `MOD_TAI`
```rust
const MOD_TAI: c_uint = 128u32;
```

### `MOD_MICRO`
```rust
const MOD_MICRO: c_uint = 4_096u32;
```

### `MOD_NANO`
```rust
const MOD_NANO: c_uint = 8_192u32;
```

### `STA_PLL`
```rust
const STA_PLL: c_int = 1i32;
```

### `STA_PPSFREQ`
```rust
const STA_PPSFREQ: c_int = 2i32;
```

### `STA_PPSTIME`
```rust
const STA_PPSTIME: c_int = 4i32;
```

### `STA_FLL`
```rust
const STA_FLL: c_int = 8i32;
```

### `STA_INS`
```rust
const STA_INS: c_int = 16i32;
```

### `STA_DEL`
```rust
const STA_DEL: c_int = 32i32;
```

### `STA_UNSYNC`
```rust
const STA_UNSYNC: c_int = 64i32;
```

### `STA_FREQHOLD`
```rust
const STA_FREQHOLD: c_int = 128i32;
```

### `STA_PPSSIGNAL`
```rust
const STA_PPSSIGNAL: c_int = 256i32;
```

### `STA_PPSJITTER`
```rust
const STA_PPSJITTER: c_int = 512i32;
```

### `STA_PPSWANDER`
```rust
const STA_PPSWANDER: c_int = 1_024i32;
```

### `STA_PPSERROR`
```rust
const STA_PPSERROR: c_int = 2_048i32;
```

### `STA_CLOCKERR`
```rust
const STA_CLOCKERR: c_int = 4_096i32;
```

### `STA_NANO`
```rust
const STA_NANO: c_int = 8_192i32;
```

### `STA_MODE`
```rust
const STA_MODE: c_int = 16_384i32;
```

### `STA_CLK`
```rust
const STA_CLK: c_int = 32_768i32;
```

### `STA_RONLY`
```rust
const STA_RONLY: c_int = 65_280i32;
```

### `NTP_API`
```rust
const NTP_API: c_int = 4i32;
```

### `TIME_OK`
```rust
const TIME_OK: c_int = 0i32;
```

### `TIME_INS`
```rust
const TIME_INS: c_int = 1i32;
```

### `TIME_DEL`
```rust
const TIME_DEL: c_int = 2i32;
```

### `TIME_OOP`
```rust
const TIME_OOP: c_int = 3i32;
```

### `TIME_WAIT`
```rust
const TIME_WAIT: c_int = 4i32;
```

### `TIME_ERROR`
```rust
const TIME_ERROR: c_int = 5i32;
```

### `TIME_BAD`
```rust
const TIME_BAD: c_int = 5i32;
```

### `MAXTC`
```rust
const MAXTC: c_long = 6i64;
```

### `GLOB_PERIOD`
```rust
const GLOB_PERIOD: c_int = 128i32;
```

### `GLOB_ALTDIRFUNC`
```rust
const GLOB_ALTDIRFUNC: c_int = 512i32;
```

### `GLOB_BRACE`
```rust
const GLOB_BRACE: c_int = 1_024i32;
```

### `GLOB_NOMAGIC`
```rust
const GLOB_NOMAGIC: c_int = 2_048i32;
```

### `GLOB_TILDE`
```rust
const GLOB_TILDE: c_int = 4_096i32;
```

### `GLOB_ONLYDIR`
```rust
const GLOB_ONLYDIR: c_int = 8_192i32;
```

### `GLOB_TILDE_CHECK`
```rust
const GLOB_TILDE_CHECK: c_int = 16_384i32;
```

### `MADV_COLLAPSE`
```rust
const MADV_COLLAPSE: c_int = 25i32;
```

### `PTHREAD_STACK_MIN`
```rust
const PTHREAD_STACK_MIN: size_t = 16_384usize;
```

### `PTHREAD_MUTEX_ADAPTIVE_NP`
```rust
const PTHREAD_MUTEX_ADAPTIVE_NP: c_int = 3i32;
```

### `REG_STARTEND`
```rust
const REG_STARTEND: c_int = 4i32;
```

### `REG_EEND`
```rust
const REG_EEND: c_int = 14i32;
```

### `REG_ESIZE`
```rust
const REG_ESIZE: c_int = 15i32;
```

### `REG_ERPAREN`
```rust
const REG_ERPAREN: c_int = 16i32;
```

### `__SIZEOF_PTHREAD_RWLOCKATTR_T`
```rust
const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8usize;
```

### `O_LARGEFILE`
```rust
const O_LARGEFILE: c_int = 0i32;
```

