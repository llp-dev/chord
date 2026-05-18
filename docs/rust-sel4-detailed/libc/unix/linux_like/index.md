*[libc](../../index.md) / [unix](../index.md) / [linux_like](index.md)*

---

# Module `linux_like`

## Contents

- [Modules](#modules)
  - [`linux`](#linux)
  - [`linux_l4re_shared`](#linux-l4re-shared)
  - [`gnu`](#gnu)
  - [`arch`](#arch)
- [Structs](#structs)
  - [`in_addr`](#in-addr)
  - [`ip_mreq`](#ip-mreq)
  - [`ip_mreqn`](#ip-mreqn)
  - [`ip_mreq_source`](#ip-mreq-source)
  - [`sockaddr`](#sockaddr)
  - [`sockaddr_in`](#sockaddr-in)
  - [`sockaddr_in6`](#sockaddr-in6)
  - [`addrinfo`](#addrinfo)
  - [`sockaddr_ll`](#sockaddr-ll)
  - [`fd_set`](#fd-set)
  - [`tm`](#tm)
  - [`sched_param`](#sched-param)
  - [`Dl_info`](#dl-info)
  - [`lconv`](#lconv)
  - [`in_pktinfo`](#in-pktinfo)
  - [`ifaddrs`](#ifaddrs)
  - [`in6_rtmsg`](#in6-rtmsg)
  - [`arpreq`](#arpreq)
  - [`arpreq_old`](#arpreq-old)
  - [`arphdr`](#arphdr)
  - [`mmsghdr`](#mmsghdr)
  - [`sockaddr_un`](#sockaddr-un)
  - [`sockaddr_storage`](#sockaddr-storage)
  - [`utsname`](#utsname)
  - [`if_nameindex`](#if-nameindex)
  - [`file_clone_range`](#file-clone-range)
  - [`sock_filter`](#sock-filter)
  - [`sock_fprog`](#sock-fprog)
  - [`statx`](#statx)
  - [`statx_timestamp`](#statx-timestamp)
  - [`epoll_event`](#epoll-event)
  - [`sigevent`](#sigevent)
  - [`dqblk`](#dqblk)
  - [`signalfd_siginfo`](#signalfd-siginfo)
  - [`fanout_args`](#fanout-args)
  - [`sockaddr_pkt`](#sockaddr-pkt)
  - [`tpacket_auxdata`](#tpacket-auxdata)
  - [`tpacket_hdr`](#tpacket-hdr)
  - [`tpacket_hdr_variant1`](#tpacket-hdr-variant1)
  - [`tpacket2_hdr`](#tpacket2-hdr)
  - [`tpacket_req`](#tpacket-req)
  - [`tpacket_req3`](#tpacket-req3)
  - [`tpacket_rollover_stats`](#tpacket-rollover-stats)
  - [`tpacket_stats`](#tpacket-stats)
  - [`tpacket_stats_v3`](#tpacket-stats-v3)
  - [`tpacket3_hdr`](#tpacket3-hdr)
  - [`tpacket_bd_ts`](#tpacket-bd-ts)
  - [`tpacket_hdr_v1`](#tpacket-hdr-v1)
  - [`msginfo`](#msginfo)
  - [`input_event`](#input-event)
  - [`input_id`](#input-id)
  - [`input_absinfo`](#input-absinfo)
  - [`input_keymap_entry`](#input-keymap-entry)
  - [`input_mask`](#input-mask)
  - [`ff_replay`](#ff-replay)
  - [`ff_trigger`](#ff-trigger)
  - [`ff_envelope`](#ff-envelope)
  - [`ff_constant_effect`](#ff-constant-effect)
  - [`ff_ramp_effect`](#ff-ramp-effect)
  - [`ff_condition_effect`](#ff-condition-effect)
  - [`ff_periodic_effect`](#ff-periodic-effect)
  - [`ff_rumble_effect`](#ff-rumble-effect)
  - [`ff_effect`](#ff-effect)
  - [`uinput_ff_upload`](#uinput-ff-upload)
  - [`uinput_ff_erase`](#uinput-ff-erase)
  - [`uinput_abs_setup`](#uinput-abs-setup)
  - [`__c_anonymous__kernel_fsid_t`](#c-anonymous-kernel-fsid-t)
  - [`posix_spawn_file_actions_t`](#posix-spawn-file-actions-t)
  - [`posix_spawnattr_t`](#posix-spawnattr-t)
  - [`genlmsghdr`](#genlmsghdr)
  - [`inotify_event`](#inotify-event)
  - [`fanotify_response`](#fanotify-response)
  - [`fanotify_event_info_header`](#fanotify-event-info-header)
  - [`fanotify_event_info_fid`](#fanotify-event-info-fid)
  - [`sockaddr_vm`](#sockaddr-vm)
  - [`sock_extended_err`](#sock-extended-err)
  - [`seccomp_data`](#seccomp-data)
  - [`seccomp_notif_sizes`](#seccomp-notif-sizes)
  - [`seccomp_notif`](#seccomp-notif)
  - [`seccomp_notif_resp`](#seccomp-notif-resp)
  - [`seccomp_notif_addfd`](#seccomp-notif-addfd)
  - [`in6_ifreq`](#in6-ifreq)
  - [`open_how`](#open-how)
  - [`ptp_clock_time`](#ptp-clock-time)
  - [`ptp_extts_request`](#ptp-extts-request)
  - [`ptp_sys_offset_extended`](#ptp-sys-offset-extended)
  - [`ptp_sys_offset_precise`](#ptp-sys-offset-precise)
  - [`ptp_extts_event`](#ptp-extts-event)
  - [`sctp_initmsg`](#sctp-initmsg)
  - [`sctp_sndrcvinfo`](#sctp-sndrcvinfo)
  - [`sctp_sndinfo`](#sctp-sndinfo)
  - [`sctp_rcvinfo`](#sctp-rcvinfo)
  - [`sctp_nxtinfo`](#sctp-nxtinfo)
  - [`sctp_prinfo`](#sctp-prinfo)
  - [`sctp_authinfo`](#sctp-authinfo)
  - [`tls_crypto_info`](#tls-crypto-info)
  - [`tls12_crypto_info_aes_gcm_128`](#tls12-crypto-info-aes-gcm-128)
  - [`tls12_crypto_info_aes_gcm_256`](#tls12-crypto-info-aes-gcm-256)
  - [`tls12_crypto_info_aes_ccm_128`](#tls12-crypto-info-aes-ccm-128)
  - [`tls12_crypto_info_chacha20_poly1305`](#tls12-crypto-info-chacha20-poly1305)
  - [`tls12_crypto_info_sm4_gcm`](#tls12-crypto-info-sm4-gcm)
  - [`tls12_crypto_info_sm4_ccm`](#tls12-crypto-info-sm4-ccm)
  - [`tls12_crypto_info_aria_gcm_128`](#tls12-crypto-info-aria-gcm-128)
  - [`tls12_crypto_info_aria_gcm_256`](#tls12-crypto-info-aria-gcm-256)
  - [`iw_param`](#iw-param)
  - [`iw_point`](#iw-point)
  - [`iw_freq`](#iw-freq)
  - [`iw_quality`](#iw-quality)
  - [`iw_discarded`](#iw-discarded)
  - [`iw_missed`](#iw-missed)
  - [`iw_scan_req`](#iw-scan-req)
  - [`iw_encode_ext`](#iw-encode-ext)
  - [`iw_pmksa`](#iw-pmksa)
  - [`iw_pmkid_cand`](#iw-pmkid-cand)
  - [`iw_statistics`](#iw-statistics)
  - [`iw_range`](#iw-range)
  - [`iw_priv_args`](#iw-priv-args)
  - [`epoll_params`](#epoll-params)
  - [`pthread_mutexattr_t`](#pthread-mutexattr-t)
  - [`pthread_rwlockattr_t`](#pthread-rwlockattr-t)
  - [`pthread_condattr_t`](#pthread-condattr-t)
  - [`pthread_barrierattr_t`](#pthread-barrierattr-t)
  - [`fanotify_event_metadata`](#fanotify-event-metadata)
  - [`ptp_sys_offset`](#ptp-sys-offset)
  - [`ptp_pin_desc`](#ptp-pin-desc)
  - [`ptp_clock_caps`](#ptp-clock-caps)
  - [`sockaddr_xdp`](#sockaddr-xdp)
  - [`xdp_ring_offset`](#xdp-ring-offset)
  - [`xdp_mmap_offsets`](#xdp-mmap-offsets)
  - [`xdp_ring_offset_v1`](#xdp-ring-offset-v1)
  - [`xdp_mmap_offsets_v1`](#xdp-mmap-offsets-v1)
  - [`xdp_umem_reg`](#xdp-umem-reg)
  - [`xdp_umem_reg_v1`](#xdp-umem-reg-v1)
  - [`xdp_statistics`](#xdp-statistics)
  - [`xdp_statistics_v1`](#xdp-statistics-v1)
  - [`xdp_options`](#xdp-options)
  - [`xdp_desc`](#xdp-desc)
  - [`xsk_tx_metadata_completion`](#xsk-tx-metadata-completion)
  - [`xsk_tx_metadata_request`](#xsk-tx-metadata-request)
  - [`mount_attr`](#mount-attr)
  - [`mnt_ns_info`](#mnt-ns-info)
  - [`dmabuf_cmsg`](#dmabuf-cmsg)
  - [`dmabuf_token`](#dmabuf-token)
  - [`sockaddr_alg`](#sockaddr-alg)
  - [`pthread_cond_t`](#pthread-cond-t)
  - [`pthread_mutex_t`](#pthread-mutex-t)
  - [`pthread_rwlock_t`](#pthread-rwlock-t)
  - [`pthread_barrier_t`](#pthread-barrier-t)
  - [`uinput_setup`](#uinput-setup)
  - [`uinput_user_dev`](#uinput-user-dev)
  - [`mq_attr`](#mq-attr)
  - [`hwtstamp_config`](#hwtstamp-config)
  - [`sched_attr`](#sched-attr)
  - [`file_handle`](#file-handle)
  - [`iw_thrspy`](#iw-thrspy)
  - [`iw_mlme`](#iw-mlme)
  - [`iw_michaelmicfailure`](#iw-michaelmicfailure)
  - [`af_alg_iv`](#af-alg-iv)
  - [`tpacket_block_desc`](#tpacket-block-desc)
  - [`sock_txtime`](#sock-txtime)
  - [`iw_event`](#iw-event)
  - [`iwreq`](#iwreq)
  - [`ptp_perout_request`](#ptp-perout-request)
  - [`xsk_tx_metadata`](#xsk-tx-metadata)
  - [`glob_t`](#glob-t)
  - [`passwd`](#passwd)
  - [`spwd`](#spwd)
  - [`itimerspec`](#itimerspec)
  - [`fsid_t`](#fsid-t)
  - [`packet_mreq`](#packet-mreq)
  - [`cpu_set_t`](#cpu-set-t)
  - [`sembuf`](#sembuf)
  - [`dl_phdr_info`](#dl-phdr-info)
  - [`Elf32_Ehdr`](#elf32-ehdr)
  - [`Elf64_Ehdr`](#elf64-ehdr)
  - [`Elf32_Sym`](#elf32-sym)
  - [`Elf64_Sym`](#elf64-sym)
  - [`Elf32_Phdr`](#elf32-phdr)
  - [`Elf64_Phdr`](#elf64-phdr)
  - [`Elf32_Shdr`](#elf32-shdr)
  - [`Elf64_Shdr`](#elf64-shdr)
  - [`__c_anonymous_elf32_rel`](#c-anonymous-elf32-rel)
  - [`__c_anonymous_elf64_rel`](#c-anonymous-elf64-rel)
  - [`ucred`](#ucred)
  - [`mntent`](#mntent)
  - [`in6_pktinfo`](#in6-pktinfo)
  - [`arpd_request`](#arpd-request)
  - [`regmatch_t`](#regmatch-t)
  - [`option`](#option)
  - [`rlimit64`](#rlimit64)
  - [`__c_anonymous_ifru_map`](#c-anonymous-ifru-map)
  - [`dirent`](#dirent)
  - [`dirent64`](#dirent64)
  - [`__c_anonymous_elf32_rela`](#c-anonymous-elf32-rela)
  - [`__c_anonymous_elf64_rela`](#c-anonymous-elf64-rela)
  - [`ifreq`](#ifreq)
  - [`ifconf`](#ifconf)
- [Enums](#enums)
  - [`timezone`](#timezone)
  - [`tpacket_versions`](#tpacket-versions)
- [Functions](#functions)
  - [`ioctl`](#ioctl)
  - [`sem_destroy`](#sem-destroy)
  - [`sem_init`](#sem-init)
  - [`fdatasync`](#fdatasync)
  - [`mincore`](#mincore)
  - [`clock_getres`](#clock-getres)
  - [`clock_gettime`](#clock-gettime)
  - [`clock_settime`](#clock-settime)
  - [`clock_getcpuclockid`](#clock-getcpuclockid)
  - [`getitimer`](#getitimer)
  - [`setitimer`](#setitimer)
  - [`dirfd`](#dirfd)
  - [`memalign`](#memalign)
  - [`setgroups`](#setgroups)
  - [`pipe2`](#pipe2)
  - [`statfs`](#statfs)
  - [`fstatfs`](#fstatfs)
  - [`memrchr`](#memrchr)
  - [`posix_fadvise`](#posix-fadvise)
  - [`futimens`](#futimens)
  - [`utimensat`](#utimensat)
  - [`duplocale`](#duplocale)
  - [`freelocale`](#freelocale)
  - [`newlocale`](#newlocale)
  - [`uselocale`](#uselocale)
  - [`mknodat`](#mknodat)
  - [`ptsname_r`](#ptsname-r)
  - [`clearenv`](#clearenv)
  - [`waitid`](#waitid)
  - [`getresuid`](#getresuid)
  - [`getresgid`](#getresgid)
  - [`acct`](#acct)
  - [`brk`](#brk)
  - [`sbrk`](#sbrk)
  - [`vfork`](#vfork)
  - [`setresgid`](#setresgid)
  - [`setresuid`](#setresuid)
  - [`wait4`](#wait4)
  - [`login_tty`](#login-tty)
  - [`execvpe`](#execvpe)
  - [`fexecve`](#fexecve)
  - [`getifaddrs`](#getifaddrs)
  - [`freeifaddrs`](#freeifaddrs)
  - [`bind`](#bind)
  - [`writev`](#writev)
  - [`readv`](#readv)
  - [`sendmsg`](#sendmsg)
  - [`recvmsg`](#recvmsg)
  - [`uname`](#uname)
  - [`strchrnul`](#strchrnul)
  - [`strftime`](#strftime)
  - [`strftime_l`](#strftime-l)
  - [`strptime`](#strptime)
  - [`mkostemp`](#mkostemp)
  - [`mkostemps`](#mkostemps)
  - [`getdomainname`](#getdomainname)
  - [`setdomainname`](#setdomainname)
  - [`if_nameindex`](#if-nameindex)
  - [`if_freenameindex`](#if-freenameindex)
  - [`getpwnam_r`](#getpwnam-r)
  - [`getpwuid_r`](#getpwuid-r)
  - [`fstatfs64`](#fstatfs64)
  - [`statvfs64`](#statvfs64)
  - [`fstatvfs64`](#fstatvfs64)
  - [`statfs64`](#statfs64)
  - [`creat64`](#creat64)
  - [`fstat64`](#fstat64)
  - [`fstatat64`](#fstatat64)
  - [`ftruncate64`](#ftruncate64)
  - [`lseek64`](#lseek64)
  - [`lstat64`](#lstat64)
  - [`mmap64`](#mmap64)
  - [`open64`](#open64)
  - [`openat64`](#openat64)
  - [`posix_fadvise64`](#posix-fadvise64)
  - [`pread64`](#pread64)
  - [`pwrite64`](#pwrite64)
  - [`readdir64`](#readdir64)
  - [`readdir64_r`](#readdir64-r)
  - [`stat64`](#stat64)
  - [`truncate64`](#truncate64)
  - [`preadv64`](#preadv64)
  - [`pwritev64`](#pwritev64)
  - [`forkpty`](#forkpty)
  - [`openpty`](#openpty)
  - [`statx`](#statx)
  - [`_IOC`](#ioc)
  - [`_IO`](#io)
  - [`_IOR`](#ior)
  - [`_IOW`](#iow)
  - [`_IOWR`](#iowr)
  - [`CMSG_ALIGN`](#cmsg-align)
  - [`CMSG_FIRSTHDR`](#cmsg-firsthdr)
  - [`CMSG_DATA`](#cmsg-data)
  - [`CMSG_SPACE`](#cmsg-space)
  - [`CMSG_LEN`](#cmsg-len)
  - [`FD_CLR`](#fd-clr)
  - [`FD_ISSET`](#fd-isset)
  - [`FD_SET`](#fd-set)
  - [`FD_ZERO`](#fd-zero)
  - [`SIGRTMAX`](#sigrtmax)
  - [`SIGRTMIN`](#sigrtmin)
  - [`WIFSTOPPED`](#wifstopped)
  - [`WSTOPSIG`](#wstopsig)
  - [`WIFCONTINUED`](#wifcontinued)
  - [`WIFSIGNALED`](#wifsignaled)
  - [`WTERMSIG`](#wtermsig)
  - [`WIFEXITED`](#wifexited)
  - [`WEXITSTATUS`](#wexitstatus)
  - [`WCOREDUMP`](#wcoredump)
  - [`W_EXITCODE`](#w-exitcode)
  - [`W_STOPCODE`](#w-stopcode)
  - [`QCMD`](#qcmd)
  - [`IPOPT_COPIED`](#ipopt-copied)
  - [`IPOPT_CLASS`](#ipopt-class)
  - [`IPOPT_NUMBER`](#ipopt-number)
  - [`IPTOS_ECN`](#iptos-ecn)
  - [`KERNEL_VERSION`](#kernel-version)
  - [`getspnam_r`](#getspnam-r)
  - [`mq_open`](#mq-open)
  - [`mq_close`](#mq-close)
  - [`mq_unlink`](#mq-unlink)
  - [`mq_receive`](#mq-receive)
  - [`mq_timedreceive`](#mq-timedreceive)
  - [`mq_send`](#mq-send)
  - [`mq_timedsend`](#mq-timedsend)
  - [`mq_getattr`](#mq-getattr)
  - [`mq_setattr`](#mq-setattr)
  - [`mrand48`](#mrand48)
  - [`seed48`](#seed48)
  - [`lcong48`](#lcong48)
  - [`lutimes`](#lutimes)
  - [`shm_open`](#shm-open)
  - [`shm_unlink`](#shm-unlink)
  - [`ftok`](#ftok)
  - [`semget`](#semget)
  - [`semop`](#semop)
  - [`semctl`](#semctl)
  - [`msgctl`](#msgctl)
  - [`msgget`](#msgget)
  - [`msgrcv`](#msgrcv)
  - [`msgsnd`](#msgsnd)
  - [`fallocate`](#fallocate)
  - [`posix_fallocate`](#posix-fallocate)
  - [`readahead`](#readahead)
  - [`getxattr`](#getxattr)
  - [`lgetxattr`](#lgetxattr)
  - [`fgetxattr`](#fgetxattr)
  - [`setxattr`](#setxattr)
  - [`lsetxattr`](#lsetxattr)
  - [`fsetxattr`](#fsetxattr)
  - [`listxattr`](#listxattr)
  - [`llistxattr`](#llistxattr)
  - [`flistxattr`](#flistxattr)
  - [`removexattr`](#removexattr)
  - [`lremovexattr`](#lremovexattr)
  - [`fremovexattr`](#fremovexattr)
  - [`signalfd`](#signalfd)
  - [`timerfd_create`](#timerfd-create)
  - [`timerfd_gettime`](#timerfd-gettime)
  - [`timerfd_settime`](#timerfd-settime)
  - [`quotactl`](#quotactl)
  - [`epoll_pwait`](#epoll-pwait)
  - [`dup3`](#dup3)
  - [`sigtimedwait`](#sigtimedwait)
  - [`sigwaitinfo`](#sigwaitinfo)
  - [`accept4`](#accept4)
  - [`reboot`](#reboot)
  - [`setfsgid`](#setfsgid)
  - [`setfsuid`](#setfsuid)
  - [`mkfifoat`](#mkfifoat)
  - [`sync_file_range`](#sync-file-range)
  - [`posix_madvise`](#posix-madvise)
  - [`remap_file_pages`](#remap-file-pages)
  - [`mkstemps`](#mkstemps)
  - [`vhangup`](#vhangup)
  - [`sync`](#sync)
  - [`syncfs`](#syncfs)
  - [`syscall`](#syscall)
  - [`sched_setaffinity`](#sched-setaffinity)
  - [`epoll_create`](#epoll-create)
  - [`epoll_create1`](#epoll-create1)
  - [`epoll_wait`](#epoll-wait)
  - [`epoll_ctl`](#epoll-ctl)
  - [`unshare`](#unshare)
  - [`umount`](#umount)
  - [`tee`](#tee)
  - [`splice`](#splice)
  - [`eventfd`](#eventfd)
  - [`eventfd_read`](#eventfd-read)
  - [`eventfd_write`](#eventfd-write)
  - [`sched_rr_get_interval`](#sched-rr-get-interval)
  - [`sched_setparam`](#sched-setparam)
  - [`setns`](#setns)
  - [`swapoff`](#swapoff)
  - [`vmsplice`](#vmsplice)
  - [`personality`](#personality)
  - [`sched_getparam`](#sched-getparam)
  - [`clone`](#clone)
  - [`sched_getscheduler`](#sched-getscheduler)
  - [`clock_nanosleep`](#clock-nanosleep)
  - [`umount2`](#umount2)
  - [`swapon`](#swapon)
  - [`sched_setscheduler`](#sched-setscheduler)
  - [`sendfile`](#sendfile)
  - [`sigaltstack`](#sigaltstack)
  - [`getdtablesize`](#getdtablesize)
  - [`getgrouplist`](#getgrouplist)
  - [`posix_spawn`](#posix-spawn)
  - [`posix_spawnp`](#posix-spawnp)
  - [`posix_spawnattr_init`](#posix-spawnattr-init)
  - [`posix_spawnattr_destroy`](#posix-spawnattr-destroy)
  - [`posix_spawnattr_getsigdefault`](#posix-spawnattr-getsigdefault)
  - [`posix_spawnattr_setsigdefault`](#posix-spawnattr-setsigdefault)
  - [`posix_spawnattr_getsigmask`](#posix-spawnattr-getsigmask)
  - [`posix_spawnattr_setsigmask`](#posix-spawnattr-setsigmask)
  - [`posix_spawnattr_getflags`](#posix-spawnattr-getflags)
  - [`posix_spawnattr_setflags`](#posix-spawnattr-setflags)
  - [`posix_spawnattr_getpgroup`](#posix-spawnattr-getpgroup)
  - [`posix_spawnattr_setpgroup`](#posix-spawnattr-setpgroup)
  - [`posix_spawnattr_getschedpolicy`](#posix-spawnattr-getschedpolicy)
  - [`posix_spawnattr_setschedpolicy`](#posix-spawnattr-setschedpolicy)
  - [`posix_spawnattr_getschedparam`](#posix-spawnattr-getschedparam)
  - [`posix_spawnattr_setschedparam`](#posix-spawnattr-setschedparam)
  - [`posix_spawn_file_actions_init`](#posix-spawn-file-actions-init)
  - [`posix_spawn_file_actions_destroy`](#posix-spawn-file-actions-destroy)
  - [`posix_spawn_file_actions_addopen`](#posix-spawn-file-actions-addopen)
  - [`posix_spawn_file_actions_addclose`](#posix-spawn-file-actions-addclose)
  - [`posix_spawn_file_actions_adddup2`](#posix-spawn-file-actions-adddup2)
  - [`fread_unlocked`](#fread-unlocked)
  - [`inotify_rm_watch`](#inotify-rm-watch)
  - [`inotify_init`](#inotify-init)
  - [`inotify_init1`](#inotify-init1)
  - [`inotify_add_watch`](#inotify-add-watch)
  - [`fanotify_init`](#fanotify-init)
  - [`gethostid`](#gethostid)
  - [`klogctl`](#klogctl)
  - [`name_to_handle_at`](#name-to-handle-at)
  - [`open_by_handle_at`](#open-by-handle-at)
  - [`fallocate64`](#fallocate64)
  - [`fgetpos64`](#fgetpos64)
  - [`fopen64`](#fopen64)
  - [`posix_fallocate64`](#posix-fallocate64)
  - [`sendfile64`](#sendfile64)
  - [`tmpfile64`](#tmpfile64)
  - [`issecure_mask`](#issecure-mask)
  - [`FUTEX_OP`](#futex-op)
  - [`SCTP_PR_INDEX`](#sctp-pr-index)
  - [`SCTP_PR_POLICY`](#sctp-pr-policy)
  - [`SCTP_PR_SET_POLICY`](#sctp-pr-set-policy)
  - [`SO_EE_OFFENDER`](#so-ee-offender)
  - [`TPACKET_ALIGN`](#tpacket-align)
  - [`BPF_CLASS`](#bpf-class)
  - [`BPF_SIZE`](#bpf-size)
  - [`BPF_MODE`](#bpf-mode)
  - [`BPF_OP`](#bpf-op)
  - [`BPF_SRC`](#bpf-src)
  - [`BPF_RVAL`](#bpf-rval)
  - [`BPF_MISCOP`](#bpf-miscop)
  - [`BPF_STMT`](#bpf-stmt)
  - [`BPF_JUMP`](#bpf-jump)
  - [`SUN_LEN`](#sun-len)
  - [`SCTP_PR_TTL_ENABLED`](#sctp-pr-ttl-enabled)
  - [`SCTP_PR_RTX_ENABLED`](#sctp-pr-rtx-enabled)
  - [`SCTP_PR_PRIO_ENABLED`](#sctp-pr-prio-enabled)
  - [`iopl`](#iopl)
  - [`ioperm`](#ioperm)
  - [`aio_read`](#aio-read)
  - [`aio_write`](#aio-write)
  - [`aio_fsync`](#aio-fsync)
  - [`aio_error`](#aio-error)
  - [`aio_return`](#aio-return)
  - [`aio_suspend`](#aio-suspend)
  - [`aio_cancel`](#aio-cancel)
  - [`lio_listio`](#lio-listio)
  - [`pwritev`](#pwritev)
  - [`preadv`](#preadv)
  - [`getnameinfo`](#getnameinfo)
  - [`getloadavg`](#getloadavg)
  - [`process_vm_readv`](#process-vm-readv)
  - [`process_vm_writev`](#process-vm-writev)
  - [`futimes`](#futimes)
  - [`strerror_r`](#strerror-r)
  - [`abs`](#abs)
  - [`labs`](#labs)
  - [`rand`](#rand)
  - [`srand`](#srand)
  - [`drand48`](#drand48)
  - [`erand48`](#erand48)
  - [`lrand48`](#lrand48)
  - [`nrand48`](#nrand48)
  - [`jrand48`](#jrand48)
  - [`srand48`](#srand48)
  - [`setpwent`](#setpwent)
  - [`endpwent`](#endpwent)
  - [`getpwent`](#getpwent)
  - [`setgrent`](#setgrent)
  - [`endgrent`](#endgrent)
  - [`getgrent`](#getgrent)
  - [`setspent`](#setspent)
  - [`endspent`](#endspent)
  - [`getspent`](#getspent)
  - [`getspnam`](#getspnam)
  - [`shmget`](#shmget)
  - [`shmat`](#shmat)
  - [`shmdt`](#shmdt)
  - [`shmctl`](#shmctl)
  - [`mprotect`](#mprotect)
  - [`__errno_location`](#errno-location)
  - [`mremap`](#mremap)
  - [`glob`](#glob)
  - [`globfree`](#globfree)
  - [`seekdir`](#seekdir)
  - [`telldir`](#telldir)
  - [`madvise`](#madvise)
  - [`msync`](#msync)
  - [`recvfrom`](#recvfrom)
  - [`nl_langinfo`](#nl-langinfo)
  - [`nl_langinfo_l`](#nl-langinfo-l)
  - [`sched_getaffinity`](#sched-getaffinity)
  - [`sched_get_priority_max`](#sched-get-priority-max)
  - [`settimeofday`](#settimeofday)
  - [`sem_timedwait`](#sem-timedwait)
  - [`sem_getvalue`](#sem-getvalue)
  - [`mount`](#mount)
  - [`prctl`](#prctl)
  - [`ppoll`](#ppoll)
  - [`sethostname`](#sethostname)
  - [`sched_get_priority_min`](#sched-get-priority-min)
  - [`sysinfo`](#sysinfo)
  - [`sigsuspend`](#sigsuspend)
  - [`getgrgid_r`](#getgrgid-r)
  - [`sem_close`](#sem-close)
  - [`getgrnam_r`](#getgrnam-r)
  - [`initgroups`](#initgroups)
  - [`sem_open`](#sem-open)
  - [`getgrnam`](#getgrnam)
  - [`sem_unlink`](#sem-unlink)
  - [`daemon`](#daemon)
  - [`sigwait`](#sigwait)
  - [`getgrgid`](#getgrgid)
  - [`popen`](#popen)
  - [`faccessat`](#faccessat)
  - [`dl_iterate_phdr`](#dl-iterate-phdr)
  - [`setmntent`](#setmntent)
  - [`getmntent`](#getmntent)
  - [`addmntent`](#addmntent)
  - [`endmntent`](#endmntent)
  - [`hasmntopt`](#hasmntopt)
  - [`regcomp`](#regcomp)
  - [`regexec`](#regexec)
  - [`regerror`](#regerror)
  - [`regfree`](#regfree)
  - [`iconv_open`](#iconv-open)
  - [`iconv`](#iconv)
  - [`iconv_close`](#iconv-close)
  - [`gettid`](#gettid)
  - [`timer_create`](#timer-create)
  - [`timer_delete`](#timer-delete)
  - [`timer_getoverrun`](#timer-getoverrun)
  - [`timer_gettime`](#timer-gettime)
  - [`timer_settime`](#timer-settime)
  - [`memmem`](#memmem)
  - [`sched_getcpu`](#sched-getcpu)
  - [`getopt_long`](#getopt-long)
  - [`copy_file_range`](#copy-file-range)
  - [`freopen64`](#freopen64)
  - [`fseeko64`](#fseeko64)
  - [`fsetpos64`](#fsetpos64)
  - [`ftello64`](#ftello64)
  - [`CMSG_NXTHDR`](#cmsg-nxthdr)
  - [`CPU_ALLOC_SIZE`](#cpu-alloc-size)
  - [`CPU_ZERO`](#cpu-zero)
  - [`CPU_SET`](#cpu-set)
  - [`CPU_CLR`](#cpu-clr)
  - [`CPU_ISSET`](#cpu-isset)
  - [`CPU_COUNT_S`](#cpu-count-s)
  - [`CPU_COUNT`](#cpu-count)
  - [`CPU_EQUAL`](#cpu-equal)
  - [`IPTOS_TOS`](#iptos-tos)
  - [`IPTOS_PREC`](#iptos-prec)
  - [`RT_TOS`](#rt-tos)
  - [`RT_ADDRCLASS`](#rt-addrclass)
  - [`RT_LOCALADDR`](#rt-localaddr)
  - [`ELF32_R_SYM`](#elf32-r-sym)
  - [`ELF32_R_TYPE`](#elf32-r-type)
  - [`ELF32_R_INFO`](#elf32-r-info)
  - [`ELF64_R_SYM`](#elf64-r-sym)
  - [`ELF64_R_TYPE`](#elf64-r-type)
  - [`ELF64_R_INFO`](#elf64-r-info)
  - [`makedev`](#makedev)
  - [`major`](#major)
  - [`minor`](#minor)
- [Type Aliases](#type-aliases)
  - [`sa_family_t`](#sa-family-t)
  - [`speed_t`](#speed-t)
  - [`tcflag_t`](#tcflag-t)
  - [`clockid_t`](#clockid-t)
  - [`timer_t`](#timer-t)
  - [`useconds_t`](#useconds-t)
  - [`key_t`](#key-t)
  - [`id_t`](#id-t)
  - [`dev_t`](#dev-t)
  - [`socklen_t`](#socklen-t)
  - [`mode_t`](#mode-t)
  - [`ino64_t`](#ino64-t)
  - [`off64_t`](#off64-t)
  - [`blkcnt64_t`](#blkcnt64-t)
  - [`rlim64_t`](#rlim64-t)
  - [`mqd_t`](#mqd-t)
  - [`nfds_t`](#nfds-t)
  - [`nl_item`](#nl-item)
  - [`idtype_t`](#idtype-t)
  - [`loff_t`](#loff-t)
  - [`pthread_key_t`](#pthread-key-t)
  - [`pthread_once_t`](#pthread-once-t)
  - [`pthread_spinlock_t`](#pthread-spinlock-t)
  - [`__kernel_fsid_t`](#kernel-fsid-t)
  - [`__kernel_clockid_t`](#kernel-clockid-t)
  - [`__u8`](#u8)
  - [`__u16`](#u16)
  - [`__s16`](#s16)
  - [`__u32`](#u32)
  - [`__s32`](#s32)
  - [`sctp_assoc_t`](#sctp-assoc-t)
  - [`eventfd_t`](#eventfd-t)
  - [`pid_type`](#pid-type)
  - [`proc_cn_mcast_op`](#proc-cn-mcast-op)
  - [`proc_cn_event`](#proc-cn-event)
  - [`Elf32_Half`](#elf32-half)
  - [`Elf32_Word`](#elf32-word)
  - [`Elf32_Off`](#elf32-off)
  - [`Elf32_Addr`](#elf32-addr)
  - [`Elf32_Xword`](#elf32-xword)
  - [`Elf32_Sword`](#elf32-sword)
  - [`Elf64_Half`](#elf64-half)
  - [`Elf64_Word`](#elf64-word)
  - [`Elf64_Off`](#elf64-off)
  - [`Elf64_Addr`](#elf64-addr)
  - [`Elf64_Xword`](#elf64-xword)
  - [`Elf64_Sxword`](#elf64-sxword)
  - [`Elf64_Sword`](#elf64-sword)
  - [`Elf32_Section`](#elf32-section)
  - [`Elf64_Section`](#elf64-section)
  - [`Elf32_Relr`](#elf32-relr)
  - [`Elf64_Relr`](#elf64-relr)
  - [`Elf32_Rel`](#elf32-rel)
  - [`Elf64_Rel`](#elf64-rel)
  - [`Elf32_Rela`](#elf32-rela)
  - [`Elf64_Rela`](#elf64-rela)
  - [`iconv_t`](#iconv-t)
- [Constants](#constants)
  - [`ULONG_SIZE`](#ulong-size)
  - [`EXIT_FAILURE`](#exit-failure)
  - [`EXIT_SUCCESS`](#exit-success)
  - [`RAND_MAX`](#rand-max)
  - [`EOF`](#eof)
  - [`SEEK_SET`](#seek-set)
  - [`SEEK_CUR`](#seek-cur)
  - [`SEEK_END`](#seek-end)
  - [`_IOFBF`](#iofbf)
  - [`_IONBF`](#ionbf)
  - [`_IOLBF`](#iolbf)
  - [`F_DUPFD`](#f-dupfd)
  - [`F_GETFD`](#f-getfd)
  - [`F_SETFD`](#f-setfd)
  - [`F_GETFL`](#f-getfl)
  - [`F_SETFL`](#f-setfl)
  - [`F_SETLEASE`](#f-setlease)
  - [`F_GETLEASE`](#f-getlease)
  - [`F_NOTIFY`](#f-notify)
  - [`F_CANCELLK`](#f-cancellk)
  - [`F_DUPFD_CLOEXEC`](#f-dupfd-cloexec)
  - [`F_SETPIPE_SZ`](#f-setpipe-sz)
  - [`F_GETPIPE_SZ`](#f-getpipe-sz)
  - [`F_ADD_SEALS`](#f-add-seals)
  - [`F_GET_SEALS`](#f-get-seals)
  - [`F_SEAL_SEAL`](#f-seal-seal)
  - [`F_SEAL_SHRINK`](#f-seal-shrink)
  - [`F_SEAL_GROW`](#f-seal-grow)
  - [`F_SEAL_WRITE`](#f-seal-write)
  - [`SIGTRAP`](#sigtrap)
  - [`PTHREAD_CREATE_JOINABLE`](#pthread-create-joinable)
  - [`PTHREAD_CREATE_DETACHED`](#pthread-create-detached)
  - [`CLOCK_REALTIME`](#clock-realtime)
  - [`CLOCK_MONOTONIC`](#clock-monotonic)
  - [`CLOCK_PROCESS_CPUTIME_ID`](#clock-process-cputime-id)
  - [`CLOCK_THREAD_CPUTIME_ID`](#clock-thread-cputime-id)
  - [`CLOCK_MONOTONIC_RAW`](#clock-monotonic-raw)
  - [`CLOCK_REALTIME_COARSE`](#clock-realtime-coarse)
  - [`CLOCK_MONOTONIC_COARSE`](#clock-monotonic-coarse)
  - [`CLOCK_BOOTTIME`](#clock-boottime)
  - [`CLOCK_REALTIME_ALARM`](#clock-realtime-alarm)
  - [`CLOCK_BOOTTIME_ALARM`](#clock-boottime-alarm)
  - [`CLOCK_TAI`](#clock-tai)
  - [`TIMER_ABSTIME`](#timer-abstime)
  - [`RUSAGE_SELF`](#rusage-self)
  - [`O_RDONLY`](#o-rdonly)
  - [`O_WRONLY`](#o-wronly)
  - [`O_RDWR`](#o-rdwr)
  - [`SOCK_CLOEXEC`](#sock-cloexec)
  - [`S_IFIFO`](#s-ififo)
  - [`S_IFCHR`](#s-ifchr)
  - [`S_IFBLK`](#s-ifblk)
  - [`S_IFDIR`](#s-ifdir)
  - [`S_IFREG`](#s-ifreg)
  - [`S_IFLNK`](#s-iflnk)
  - [`S_IFSOCK`](#s-ifsock)
  - [`S_IFMT`](#s-ifmt)
  - [`S_IRWXU`](#s-irwxu)
  - [`S_IXUSR`](#s-ixusr)
  - [`S_IWUSR`](#s-iwusr)
  - [`S_IRUSR`](#s-irusr)
  - [`S_IRWXG`](#s-irwxg)
  - [`S_IXGRP`](#s-ixgrp)
  - [`S_IWGRP`](#s-iwgrp)
  - [`S_IRGRP`](#s-irgrp)
  - [`S_IRWXO`](#s-irwxo)
  - [`S_IXOTH`](#s-ixoth)
  - [`S_IWOTH`](#s-iwoth)
  - [`S_IROTH`](#s-iroth)
  - [`F_OK`](#f-ok)
  - [`R_OK`](#r-ok)
  - [`W_OK`](#w-ok)
  - [`X_OK`](#x-ok)
  - [`SIGHUP`](#sighup)
  - [`SIGINT`](#sigint)
  - [`SIGQUIT`](#sigquit)
  - [`SIGILL`](#sigill)
  - [`SIGABRT`](#sigabrt)
  - [`SIGFPE`](#sigfpe)
  - [`SIGKILL`](#sigkill)
  - [`SIGSEGV`](#sigsegv)
  - [`SIGPIPE`](#sigpipe)
  - [`SIGALRM`](#sigalrm)
  - [`SIGTERM`](#sigterm)
  - [`PROT_NONE`](#prot-none)
  - [`PROT_READ`](#prot-read)
  - [`PROT_WRITE`](#prot-write)
  - [`PROT_EXEC`](#prot-exec)
  - [`XATTR_CREATE`](#xattr-create)
  - [`XATTR_REPLACE`](#xattr-replace)
  - [`RLIM64_INFINITY`](#rlim64-infinity)
  - [`LC_CTYPE`](#lc-ctype)
  - [`LC_NUMERIC`](#lc-numeric)
  - [`LC_TIME`](#lc-time)
  - [`LC_COLLATE`](#lc-collate)
  - [`LC_MONETARY`](#lc-monetary)
  - [`LC_MESSAGES`](#lc-messages)
  - [`LC_ALL`](#lc-all)
  - [`LC_CTYPE_MASK`](#lc-ctype-mask)
  - [`LC_NUMERIC_MASK`](#lc-numeric-mask)
  - [`LC_TIME_MASK`](#lc-time-mask)
  - [`LC_COLLATE_MASK`](#lc-collate-mask)
  - [`LC_MONETARY_MASK`](#lc-monetary-mask)
  - [`LC_MESSAGES_MASK`](#lc-messages-mask)
  - [`MAP_FILE`](#map-file)
  - [`MAP_SHARED`](#map-shared)
  - [`MAP_PRIVATE`](#map-private)
  - [`MAP_FIXED`](#map-fixed)
  - [`MAP_FAILED`](#map-failed)
  - [`MS_ASYNC`](#ms-async)
  - [`MS_INVALIDATE`](#ms-invalidate)
  - [`MS_SYNC`](#ms-sync)
  - [`MS_RDONLY`](#ms-rdonly)
  - [`MS_NOSUID`](#ms-nosuid)
  - [`MS_NODEV`](#ms-nodev)
  - [`MS_NOEXEC`](#ms-noexec)
  - [`MS_SYNCHRONOUS`](#ms-synchronous)
  - [`MS_REMOUNT`](#ms-remount)
  - [`MS_MANDLOCK`](#ms-mandlock)
  - [`MS_DIRSYNC`](#ms-dirsync)
  - [`MS_NOSYMFOLLOW`](#ms-nosymfollow)
  - [`MS_NOATIME`](#ms-noatime)
  - [`MS_NODIRATIME`](#ms-nodiratime)
  - [`MS_BIND`](#ms-bind)
  - [`MS_MOVE`](#ms-move)
  - [`MS_REC`](#ms-rec)
  - [`MS_SILENT`](#ms-silent)
  - [`MS_POSIXACL`](#ms-posixacl)
  - [`MS_UNBINDABLE`](#ms-unbindable)
  - [`MS_PRIVATE`](#ms-private)
  - [`MS_SLAVE`](#ms-slave)
  - [`MS_SHARED`](#ms-shared)
  - [`MS_RELATIME`](#ms-relatime)
  - [`MS_KERNMOUNT`](#ms-kernmount)
  - [`MS_I_VERSION`](#ms-i-version)
  - [`MS_STRICTATIME`](#ms-strictatime)
  - [`MS_LAZYTIME`](#ms-lazytime)
  - [`MS_ACTIVE`](#ms-active)
  - [`MS_MGC_VAL`](#ms-mgc-val)
  - [`MS_MGC_MSK`](#ms-mgc-msk)
  - [`SCM_RIGHTS`](#scm-rights)
  - [`SCM_CREDENTIALS`](#scm-credentials)
  - [`PROT_GROWSDOWN`](#prot-growsdown)
  - [`PROT_GROWSUP`](#prot-growsup)
  - [`MAP_TYPE`](#map-type)
  - [`MADV_NORMAL`](#madv-normal)
  - [`MADV_RANDOM`](#madv-random)
  - [`MADV_SEQUENTIAL`](#madv-sequential)
  - [`MADV_WILLNEED`](#madv-willneed)
  - [`MADV_DONTNEED`](#madv-dontneed)
  - [`MADV_FREE`](#madv-free)
  - [`MADV_REMOVE`](#madv-remove)
  - [`MADV_DONTFORK`](#madv-dontfork)
  - [`MADV_DOFORK`](#madv-dofork)
  - [`MADV_MERGEABLE`](#madv-mergeable)
  - [`MADV_UNMERGEABLE`](#madv-unmergeable)
  - [`MADV_HUGEPAGE`](#madv-hugepage)
  - [`MADV_NOHUGEPAGE`](#madv-nohugepage)
  - [`MADV_DONTDUMP`](#madv-dontdump)
  - [`MADV_DODUMP`](#madv-dodump)
  - [`MADV_WIPEONFORK`](#madv-wipeonfork)
  - [`MADV_KEEPONFORK`](#madv-keeponfork)
  - [`MADV_COLD`](#madv-cold)
  - [`MADV_PAGEOUT`](#madv-pageout)
  - [`MADV_HWPOISON`](#madv-hwpoison)
  - [`MADV_POPULATE_READ`](#madv-populate-read)
  - [`MADV_POPULATE_WRITE`](#madv-populate-write)
  - [`MADV_DONTNEED_LOCKED`](#madv-dontneed-locked)
  - [`IFF_UP`](#iff-up)
  - [`IFF_BROADCAST`](#iff-broadcast)
  - [`IFF_DEBUG`](#iff-debug)
  - [`IFF_LOOPBACK`](#iff-loopback)
  - [`IFF_POINTOPOINT`](#iff-pointopoint)
  - [`IFF_NOTRAILERS`](#iff-notrailers)
  - [`IFF_RUNNING`](#iff-running)
  - [`IFF_NOARP`](#iff-noarp)
  - [`IFF_PROMISC`](#iff-promisc)
  - [`IFF_ALLMULTI`](#iff-allmulti)
  - [`IFF_MASTER`](#iff-master)
  - [`IFF_SLAVE`](#iff-slave)
  - [`IFF_MULTICAST`](#iff-multicast)
  - [`IFF_PORTSEL`](#iff-portsel)
  - [`IFF_AUTOMEDIA`](#iff-automedia)
  - [`IFF_DYNAMIC`](#iff-dynamic)
  - [`SOL_IP`](#sol-ip)
  - [`SOL_TCP`](#sol-tcp)
  - [`SOL_UDP`](#sol-udp)
  - [`SOL_IPV6`](#sol-ipv6)
  - [`SOL_ICMPV6`](#sol-icmpv6)
  - [`SOL_RAW`](#sol-raw)
  - [`SOL_DECNET`](#sol-decnet)
  - [`SOL_X25`](#sol-x25)
  - [`SOL_PACKET`](#sol-packet)
  - [`SOL_ATM`](#sol-atm)
  - [`SOL_AAL`](#sol-aal)
  - [`SOL_IRDA`](#sol-irda)
  - [`SOL_NETBEUI`](#sol-netbeui)
  - [`SOL_LLC`](#sol-llc)
  - [`SOL_DCCP`](#sol-dccp)
  - [`SOL_NETLINK`](#sol-netlink)
  - [`SOL_TIPC`](#sol-tipc)
  - [`SOL_BLUETOOTH`](#sol-bluetooth)
  - [`SOL_ALG`](#sol-alg)
  - [`AF_UNSPEC`](#af-unspec)
  - [`AF_UNIX`](#af-unix)
  - [`AF_LOCAL`](#af-local)
  - [`AF_INET`](#af-inet)
  - [`AF_AX25`](#af-ax25)
  - [`AF_IPX`](#af-ipx)
  - [`AF_APPLETALK`](#af-appletalk)
  - [`AF_NETROM`](#af-netrom)
  - [`AF_BRIDGE`](#af-bridge)
  - [`AF_ATMPVC`](#af-atmpvc)
  - [`AF_X25`](#af-x25)
  - [`AF_INET6`](#af-inet6)
  - [`AF_ROSE`](#af-rose)
  - [`AF_DECnet`](#af-decnet)
  - [`AF_NETBEUI`](#af-netbeui)
  - [`AF_SECURITY`](#af-security)
  - [`AF_KEY`](#af-key)
  - [`AF_NETLINK`](#af-netlink)
  - [`AF_ROUTE`](#af-route)
  - [`AF_PACKET`](#af-packet)
  - [`AF_ASH`](#af-ash)
  - [`AF_ECONET`](#af-econet)
  - [`AF_ATMSVC`](#af-atmsvc)
  - [`AF_RDS`](#af-rds)
  - [`AF_SNA`](#af-sna)
  - [`AF_IRDA`](#af-irda)
  - [`AF_PPPOX`](#af-pppox)
  - [`AF_WANPIPE`](#af-wanpipe)
  - [`AF_LLC`](#af-llc)
  - [`AF_CAN`](#af-can)
  - [`AF_TIPC`](#af-tipc)
  - [`AF_BLUETOOTH`](#af-bluetooth)
  - [`AF_IUCV`](#af-iucv)
  - [`AF_RXRPC`](#af-rxrpc)
  - [`AF_ISDN`](#af-isdn)
  - [`AF_PHONET`](#af-phonet)
  - [`AF_IEEE802154`](#af-ieee802154)
  - [`AF_CAIF`](#af-caif)
  - [`AF_ALG`](#af-alg)
  - [`PF_UNSPEC`](#pf-unspec)
  - [`PF_UNIX`](#pf-unix)
  - [`PF_LOCAL`](#pf-local)
  - [`PF_INET`](#pf-inet)
  - [`PF_AX25`](#pf-ax25)
  - [`PF_IPX`](#pf-ipx)
  - [`PF_APPLETALK`](#pf-appletalk)
  - [`PF_NETROM`](#pf-netrom)
  - [`PF_BRIDGE`](#pf-bridge)
  - [`PF_ATMPVC`](#pf-atmpvc)
  - [`PF_X25`](#pf-x25)
  - [`PF_INET6`](#pf-inet6)
  - [`PF_ROSE`](#pf-rose)
  - [`PF_DECnet`](#pf-decnet)
  - [`PF_NETBEUI`](#pf-netbeui)
  - [`PF_SECURITY`](#pf-security)
  - [`PF_KEY`](#pf-key)
  - [`PF_NETLINK`](#pf-netlink)
  - [`PF_ROUTE`](#pf-route)
  - [`PF_PACKET`](#pf-packet)
  - [`PF_ASH`](#pf-ash)
  - [`PF_ECONET`](#pf-econet)
  - [`PF_ATMSVC`](#pf-atmsvc)
  - [`PF_RDS`](#pf-rds)
  - [`PF_SNA`](#pf-sna)
  - [`PF_IRDA`](#pf-irda)
  - [`PF_PPPOX`](#pf-pppox)
  - [`PF_WANPIPE`](#pf-wanpipe)
  - [`PF_LLC`](#pf-llc)
  - [`PF_CAN`](#pf-can)
  - [`PF_TIPC`](#pf-tipc)
  - [`PF_BLUETOOTH`](#pf-bluetooth)
  - [`PF_IUCV`](#pf-iucv)
  - [`PF_RXRPC`](#pf-rxrpc)
  - [`PF_ISDN`](#pf-isdn)
  - [`PF_PHONET`](#pf-phonet)
  - [`PF_IEEE802154`](#pf-ieee802154)
  - [`PF_CAIF`](#pf-caif)
  - [`PF_ALG`](#pf-alg)
  - [`MSG_OOB`](#msg-oob)
  - [`MSG_PEEK`](#msg-peek)
  - [`MSG_DONTROUTE`](#msg-dontroute)
  - [`MSG_CTRUNC`](#msg-ctrunc)
  - [`MSG_TRUNC`](#msg-trunc)
  - [`MSG_DONTWAIT`](#msg-dontwait)
  - [`MSG_EOR`](#msg-eor)
  - [`MSG_WAITALL`](#msg-waitall)
  - [`MSG_FIN`](#msg-fin)
  - [`MSG_SYN`](#msg-syn)
  - [`MSG_CONFIRM`](#msg-confirm)
  - [`MSG_RST`](#msg-rst)
  - [`MSG_ERRQUEUE`](#msg-errqueue)
  - [`MSG_NOSIGNAL`](#msg-nosignal)
  - [`MSG_MORE`](#msg-more)
  - [`MSG_WAITFORONE`](#msg-waitforone)
  - [`MSG_FASTOPEN`](#msg-fastopen)
  - [`MSG_CMSG_CLOEXEC`](#msg-cmsg-cloexec)
  - [`SCM_TIMESTAMP`](#scm-timestamp)
  - [`SOCK_RAW`](#sock-raw)
  - [`SOCK_RDM`](#sock-rdm)
  - [`IP_TOS`](#ip-tos)
  - [`IP_TTL`](#ip-ttl)
  - [`IP_HDRINCL`](#ip-hdrincl)
  - [`IP_OPTIONS`](#ip-options)
  - [`IP_ROUTER_ALERT`](#ip-router-alert)
  - [`IP_RECVOPTS`](#ip-recvopts)
  - [`IP_RETOPTS`](#ip-retopts)
  - [`IP_PKTINFO`](#ip-pktinfo)
  - [`IP_PKTOPTIONS`](#ip-pktoptions)
  - [`IP_MTU_DISCOVER`](#ip-mtu-discover)
  - [`IP_RECVERR`](#ip-recverr)
  - [`IP_RECVTTL`](#ip-recvttl)
  - [`IP_RECVTOS`](#ip-recvtos)
  - [`IP_MTU`](#ip-mtu)
  - [`IP_FREEBIND`](#ip-freebind)
  - [`IP_IPSEC_POLICY`](#ip-ipsec-policy)
  - [`IP_XFRM_POLICY`](#ip-xfrm-policy)
  - [`IP_PASSSEC`](#ip-passsec)
  - [`IP_TRANSPARENT`](#ip-transparent)
  - [`IP_ORIGDSTADDR`](#ip-origdstaddr)
  - [`IP_RECVORIGDSTADDR`](#ip-recvorigdstaddr)
  - [`IP_MINTTL`](#ip-minttl)
  - [`IP_NODEFRAG`](#ip-nodefrag)
  - [`IP_CHECKSUM`](#ip-checksum)
  - [`IP_BIND_ADDRESS_NO_PORT`](#ip-bind-address-no-port)
  - [`IP_MULTICAST_IF`](#ip-multicast-if)
  - [`IP_MULTICAST_TTL`](#ip-multicast-ttl)
  - [`IP_MULTICAST_LOOP`](#ip-multicast-loop)
  - [`IP_ADD_MEMBERSHIP`](#ip-add-membership)
  - [`IP_DROP_MEMBERSHIP`](#ip-drop-membership)
  - [`IP_UNBLOCK_SOURCE`](#ip-unblock-source)
  - [`IP_BLOCK_SOURCE`](#ip-block-source)
  - [`IP_ADD_SOURCE_MEMBERSHIP`](#ip-add-source-membership)
  - [`IP_DROP_SOURCE_MEMBERSHIP`](#ip-drop-source-membership)
  - [`IP_MSFILTER`](#ip-msfilter)
  - [`IP_MULTICAST_ALL`](#ip-multicast-all)
  - [`IP_UNICAST_IF`](#ip-unicast-if)
  - [`IP_DEFAULT_MULTICAST_TTL`](#ip-default-multicast-ttl)
  - [`IP_DEFAULT_MULTICAST_LOOP`](#ip-default-multicast-loop)
  - [`IP_PMTUDISC_DONT`](#ip-pmtudisc-dont)
  - [`IP_PMTUDISC_WANT`](#ip-pmtudisc-want)
  - [`IP_PMTUDISC_DO`](#ip-pmtudisc-do)
  - [`IP_PMTUDISC_PROBE`](#ip-pmtudisc-probe)
  - [`IP_PMTUDISC_INTERFACE`](#ip-pmtudisc-interface)
  - [`IP_PMTUDISC_OMIT`](#ip-pmtudisc-omit)
  - [`IPPROTO_HOPOPTS`](#ipproto-hopopts)
  - [`IPPROTO_IGMP`](#ipproto-igmp)
  - [`IPPROTO_IPIP`](#ipproto-ipip)
  - [`IPPROTO_EGP`](#ipproto-egp)
  - [`IPPROTO_PUP`](#ipproto-pup)
  - [`IPPROTO_IDP`](#ipproto-idp)
  - [`IPPROTO_TP`](#ipproto-tp)
  - [`IPPROTO_DCCP`](#ipproto-dccp)
  - [`IPPROTO_ROUTING`](#ipproto-routing)
  - [`IPPROTO_FRAGMENT`](#ipproto-fragment)
  - [`IPPROTO_RSVP`](#ipproto-rsvp)
  - [`IPPROTO_GRE`](#ipproto-gre)
  - [`IPPROTO_ESP`](#ipproto-esp)
  - [`IPPROTO_AH`](#ipproto-ah)
  - [`IPPROTO_NONE`](#ipproto-none)
  - [`IPPROTO_DSTOPTS`](#ipproto-dstopts)
  - [`IPPROTO_MTP`](#ipproto-mtp)
  - [`IPPROTO_ENCAP`](#ipproto-encap)
  - [`IPPROTO_PIM`](#ipproto-pim)
  - [`IPPROTO_COMP`](#ipproto-comp)
  - [`IPPROTO_SCTP`](#ipproto-sctp)
  - [`IPPROTO_MH`](#ipproto-mh)
  - [`IPPROTO_UDPLITE`](#ipproto-udplite)
  - [`IPPROTO_RAW`](#ipproto-raw)
  - [`IPPROTO_BEETPH`](#ipproto-beetph)
  - [`IPPROTO_MPLS`](#ipproto-mpls)
  - [`IPPROTO_MPTCP`](#ipproto-mptcp)
  - [`IPPROTO_ETHERNET`](#ipproto-ethernet)
  - [`MCAST_EXCLUDE`](#mcast-exclude)
  - [`MCAST_INCLUDE`](#mcast-include)
  - [`MCAST_JOIN_GROUP`](#mcast-join-group)
  - [`MCAST_BLOCK_SOURCE`](#mcast-block-source)
  - [`MCAST_UNBLOCK_SOURCE`](#mcast-unblock-source)
  - [`MCAST_LEAVE_GROUP`](#mcast-leave-group)
  - [`MCAST_JOIN_SOURCE_GROUP`](#mcast-join-source-group)
  - [`MCAST_LEAVE_SOURCE_GROUP`](#mcast-leave-source-group)
  - [`MCAST_MSFILTER`](#mcast-msfilter)
  - [`IPV6_ADDRFORM`](#ipv6-addrform)
  - [`IPV6_2292PKTINFO`](#ipv6-2292pktinfo)
  - [`IPV6_2292HOPOPTS`](#ipv6-2292hopopts)
  - [`IPV6_2292DSTOPTS`](#ipv6-2292dstopts)
  - [`IPV6_2292RTHDR`](#ipv6-2292rthdr)
  - [`IPV6_2292PKTOPTIONS`](#ipv6-2292pktoptions)
  - [`IPV6_CHECKSUM`](#ipv6-checksum)
  - [`IPV6_2292HOPLIMIT`](#ipv6-2292hoplimit)
  - [`IPV6_NEXTHOP`](#ipv6-nexthop)
  - [`IPV6_AUTHHDR`](#ipv6-authhdr)
  - [`IPV6_UNICAST_HOPS`](#ipv6-unicast-hops)
  - [`IPV6_MULTICAST_IF`](#ipv6-multicast-if)
  - [`IPV6_MULTICAST_HOPS`](#ipv6-multicast-hops)
  - [`IPV6_MULTICAST_LOOP`](#ipv6-multicast-loop)
  - [`IPV6_ADD_MEMBERSHIP`](#ipv6-add-membership)
  - [`IPV6_DROP_MEMBERSHIP`](#ipv6-drop-membership)
  - [`IPV6_ROUTER_ALERT`](#ipv6-router-alert)
  - [`IPV6_MTU_DISCOVER`](#ipv6-mtu-discover)
  - [`IPV6_MTU`](#ipv6-mtu)
  - [`IPV6_RECVERR`](#ipv6-recverr)
  - [`IPV6_V6ONLY`](#ipv6-v6only)
  - [`IPV6_JOIN_ANYCAST`](#ipv6-join-anycast)
  - [`IPV6_LEAVE_ANYCAST`](#ipv6-leave-anycast)
  - [`IPV6_IPSEC_POLICY`](#ipv6-ipsec-policy)
  - [`IPV6_XFRM_POLICY`](#ipv6-xfrm-policy)
  - [`IPV6_HDRINCL`](#ipv6-hdrincl)
  - [`IPV6_RECVPKTINFO`](#ipv6-recvpktinfo)
  - [`IPV6_PKTINFO`](#ipv6-pktinfo)
  - [`IPV6_RECVHOPLIMIT`](#ipv6-recvhoplimit)
  - [`IPV6_HOPLIMIT`](#ipv6-hoplimit)
  - [`IPV6_RECVHOPOPTS`](#ipv6-recvhopopts)
  - [`IPV6_HOPOPTS`](#ipv6-hopopts)
  - [`IPV6_RTHDRDSTOPTS`](#ipv6-rthdrdstopts)
  - [`IPV6_RECVRTHDR`](#ipv6-recvrthdr)
  - [`IPV6_RTHDR`](#ipv6-rthdr)
  - [`IPV6_RECVDSTOPTS`](#ipv6-recvdstopts)
  - [`IPV6_DSTOPTS`](#ipv6-dstopts)
  - [`IPV6_RECVPATHMTU`](#ipv6-recvpathmtu)
  - [`IPV6_PATHMTU`](#ipv6-pathmtu)
  - [`IPV6_DONTFRAG`](#ipv6-dontfrag)
  - [`IPV6_RECVTCLASS`](#ipv6-recvtclass)
  - [`IPV6_TCLASS`](#ipv6-tclass)
  - [`IPV6_AUTOFLOWLABEL`](#ipv6-autoflowlabel)
  - [`IPV6_ADDR_PREFERENCES`](#ipv6-addr-preferences)
  - [`IPV6_MINHOPCOUNT`](#ipv6-minhopcount)
  - [`IPV6_ORIGDSTADDR`](#ipv6-origdstaddr)
  - [`IPV6_RECVORIGDSTADDR`](#ipv6-recvorigdstaddr)
  - [`IPV6_TRANSPARENT`](#ipv6-transparent)
  - [`IPV6_UNICAST_IF`](#ipv6-unicast-if)
  - [`IPV6_PREFER_SRC_TMP`](#ipv6-prefer-src-tmp)
  - [`IPV6_PREFER_SRC_PUBLIC`](#ipv6-prefer-src-public)
  - [`IPV6_PREFER_SRC_PUBTMP_DEFAULT`](#ipv6-prefer-src-pubtmp-default)
  - [`IPV6_PREFER_SRC_COA`](#ipv6-prefer-src-coa)
  - [`IPV6_PREFER_SRC_HOME`](#ipv6-prefer-src-home)
  - [`IPV6_PREFER_SRC_CGA`](#ipv6-prefer-src-cga)
  - [`IPV6_PREFER_SRC_NONCGA`](#ipv6-prefer-src-noncga)
  - [`IPV6_PMTUDISC_DONT`](#ipv6-pmtudisc-dont)
  - [`IPV6_PMTUDISC_WANT`](#ipv6-pmtudisc-want)
  - [`IPV6_PMTUDISC_DO`](#ipv6-pmtudisc-do)
  - [`IPV6_PMTUDISC_PROBE`](#ipv6-pmtudisc-probe)
  - [`IPV6_PMTUDISC_INTERFACE`](#ipv6-pmtudisc-interface)
  - [`IPV6_PMTUDISC_OMIT`](#ipv6-pmtudisc-omit)
  - [`TCP_NODELAY`](#tcp-nodelay)
  - [`TCP_MAXSEG`](#tcp-maxseg)
  - [`TCP_CORK`](#tcp-cork)
  - [`TCP_KEEPIDLE`](#tcp-keepidle)
  - [`TCP_KEEPINTVL`](#tcp-keepintvl)
  - [`TCP_KEEPCNT`](#tcp-keepcnt)
  - [`TCP_SYNCNT`](#tcp-syncnt)
  - [`TCP_LINGER2`](#tcp-linger2)
  - [`TCP_DEFER_ACCEPT`](#tcp-defer-accept)
  - [`TCP_WINDOW_CLAMP`](#tcp-window-clamp)
  - [`TCP_INFO`](#tcp-info)
  - [`TCP_QUICKACK`](#tcp-quickack)
  - [`TCP_CONGESTION`](#tcp-congestion)
  - [`TCP_MD5SIG`](#tcp-md5sig)
  - [`TCP_COOKIE_TRANSACTIONS`](#tcp-cookie-transactions)
  - [`TCP_THIN_LINEAR_TIMEOUTS`](#tcp-thin-linear-timeouts)
  - [`TCP_THIN_DUPACK`](#tcp-thin-dupack)
  - [`TCP_USER_TIMEOUT`](#tcp-user-timeout)
  - [`TCP_REPAIR`](#tcp-repair)
  - [`TCP_REPAIR_QUEUE`](#tcp-repair-queue)
  - [`TCP_QUEUE_SEQ`](#tcp-queue-seq)
  - [`TCP_REPAIR_OPTIONS`](#tcp-repair-options)
  - [`TCP_FASTOPEN`](#tcp-fastopen)
  - [`TCP_TIMESTAMP`](#tcp-timestamp)
  - [`TCP_NOTSENT_LOWAT`](#tcp-notsent-lowat)
  - [`TCP_CC_INFO`](#tcp-cc-info)
  - [`TCP_SAVE_SYN`](#tcp-save-syn)
  - [`TCP_SAVED_SYN`](#tcp-saved-syn)
  - [`TCP_REPAIR_WINDOW`](#tcp-repair-window)
  - [`TCP_FASTOPEN_CONNECT`](#tcp-fastopen-connect)
  - [`TCP_ULP`](#tcp-ulp)
  - [`TCP_MD5SIG_EXT`](#tcp-md5sig-ext)
  - [`TCP_FASTOPEN_KEY`](#tcp-fastopen-key)
  - [`TCP_FASTOPEN_NO_COOKIE`](#tcp-fastopen-no-cookie)
  - [`TCP_ZEROCOPY_RECEIVE`](#tcp-zerocopy-receive)
  - [`TCP_INQ`](#tcp-inq)
  - [`TCP_CM_INQ`](#tcp-cm-inq)
  - [`TCP_MD5SIG_MAXKEYLEN`](#tcp-md5sig-maxkeylen)
  - [`SO_DEBUG`](#so-debug)
  - [`SHUT_RD`](#shut-rd)
  - [`SHUT_WR`](#shut-wr)
  - [`SHUT_RDWR`](#shut-rdwr)
  - [`LOCK_SH`](#lock-sh)
  - [`LOCK_EX`](#lock-ex)
  - [`LOCK_NB`](#lock-nb)
  - [`LOCK_UN`](#lock-un)
  - [`SS_ONSTACK`](#ss-onstack)
  - [`SS_DISABLE`](#ss-disable)
  - [`NAME_MAX`](#name-max)
  - [`PATH_MAX`](#path-max)
  - [`UIO_MAXIOV`](#uio-maxiov)
  - [`FD_SETSIZE`](#fd-setsize)
  - [`EPOLLIN`](#epollin)
  - [`EPOLLPRI`](#epollpri)
  - [`EPOLLOUT`](#epollout)
  - [`EPOLLERR`](#epollerr)
  - [`EPOLLHUP`](#epollhup)
  - [`EPOLLRDNORM`](#epollrdnorm)
  - [`EPOLLRDBAND`](#epollrdband)
  - [`EPOLLWRNORM`](#epollwrnorm)
  - [`EPOLLWRBAND`](#epollwrband)
  - [`EPOLLMSG`](#epollmsg)
  - [`EPOLLRDHUP`](#epollrdhup)
  - [`EPOLLEXCLUSIVE`](#epollexclusive)
  - [`EPOLLWAKEUP`](#epollwakeup)
  - [`EPOLLONESHOT`](#epolloneshot)
  - [`EPOLLET`](#epollet)
  - [`EPOLL_CTL_ADD`](#epoll-ctl-add)
  - [`EPOLL_CTL_MOD`](#epoll-ctl-mod)
  - [`EPOLL_CTL_DEL`](#epoll-ctl-del)
  - [`MNT_FORCE`](#mnt-force)
  - [`MNT_DETACH`](#mnt-detach)
  - [`MNT_EXPIRE`](#mnt-expire)
  - [`UMOUNT_NOFOLLOW`](#umount-nofollow)
  - [`Q_GETFMT`](#q-getfmt)
  - [`Q_GETINFO`](#q-getinfo)
  - [`Q_SETINFO`](#q-setinfo)
  - [`QIF_BLIMITS`](#qif-blimits)
  - [`QIF_SPACE`](#qif-space)
  - [`QIF_ILIMITS`](#qif-ilimits)
  - [`QIF_INODES`](#qif-inodes)
  - [`QIF_BTIME`](#qif-btime)
  - [`QIF_ITIME`](#qif-itime)
  - [`QIF_LIMITS`](#qif-limits)
  - [`QIF_USAGE`](#qif-usage)
  - [`QIF_TIMES`](#qif-times)
  - [`QIF_ALL`](#qif-all)
  - [`Q_SYNC`](#q-sync)
  - [`Q_QUOTAON`](#q-quotaon)
  - [`Q_QUOTAOFF`](#q-quotaoff)
  - [`Q_GETQUOTA`](#q-getquota)
  - [`Q_SETQUOTA`](#q-setquota)
  - [`TCIOFF`](#tcioff)
  - [`TCION`](#tcion)
  - [`TCOOFF`](#tcooff)
  - [`TCOON`](#tcoon)
  - [`TCIFLUSH`](#tciflush)
  - [`TCOFLUSH`](#tcoflush)
  - [`TCIOFLUSH`](#tcioflush)
  - [`NL0`](#nl0)
  - [`NL1`](#nl1)
  - [`TAB0`](#tab0)
  - [`CR0`](#cr0)
  - [`FF0`](#ff0)
  - [`BS0`](#bs0)
  - [`VT0`](#vt0)
  - [`VERASE`](#verase)
  - [`VKILL`](#vkill)
  - [`VINTR`](#vintr)
  - [`VQUIT`](#vquit)
  - [`VLNEXT`](#vlnext)
  - [`IGNBRK`](#ignbrk)
  - [`BRKINT`](#brkint)
  - [`IGNPAR`](#ignpar)
  - [`PARMRK`](#parmrk)
  - [`INPCK`](#inpck)
  - [`ISTRIP`](#istrip)
  - [`INLCR`](#inlcr)
  - [`IGNCR`](#igncr)
  - [`ICRNL`](#icrnl)
  - [`IXANY`](#ixany)
  - [`IMAXBEL`](#imaxbel)
  - [`OPOST`](#opost)
  - [`CS5`](#cs5)
  - [`CRTSCTS`](#crtscts)
  - [`ECHO`](#echo)
  - [`OCRNL`](#ocrnl)
  - [`ONOCR`](#onocr)
  - [`ONLRET`](#onlret)
  - [`OFILL`](#ofill)
  - [`OFDEL`](#ofdel)
  - [`CLONE_VM`](#clone-vm)
  - [`CLONE_FS`](#clone-fs)
  - [`CLONE_FILES`](#clone-files)
  - [`CLONE_SIGHAND`](#clone-sighand)
  - [`CLONE_PTRACE`](#clone-ptrace)
  - [`CLONE_VFORK`](#clone-vfork)
  - [`CLONE_PARENT`](#clone-parent)
  - [`CLONE_THREAD`](#clone-thread)
  - [`CLONE_NEWNS`](#clone-newns)
  - [`CLONE_SYSVSEM`](#clone-sysvsem)
  - [`CLONE_SETTLS`](#clone-settls)
  - [`CLONE_PARENT_SETTID`](#clone-parent-settid)
  - [`CLONE_CHILD_CLEARTID`](#clone-child-cleartid)
  - [`CLONE_DETACHED`](#clone-detached)
  - [`CLONE_UNTRACED`](#clone-untraced)
  - [`CLONE_CHILD_SETTID`](#clone-child-settid)
  - [`CLONE_NEWCGROUP`](#clone-newcgroup)
  - [`CLONE_NEWUTS`](#clone-newuts)
  - [`CLONE_NEWIPC`](#clone-newipc)
  - [`CLONE_NEWUSER`](#clone-newuser)
  - [`CLONE_NEWPID`](#clone-newpid)
  - [`CLONE_NEWNET`](#clone-newnet)
  - [`CLONE_IO`](#clone-io)
  - [`WNOHANG`](#wnohang)
  - [`WUNTRACED`](#wuntraced)
  - [`WSTOPPED`](#wstopped)
  - [`WEXITED`](#wexited)
  - [`WCONTINUED`](#wcontinued)
  - [`WNOWAIT`](#wnowait)
  - [`ADDR_NO_RANDOMIZE`](#addr-no-randomize)
  - [`MMAP_PAGE_ZERO`](#mmap-page-zero)
  - [`ADDR_COMPAT_LAYOUT`](#addr-compat-layout)
  - [`READ_IMPLIES_EXEC`](#read-implies-exec)
  - [`ADDR_LIMIT_32BIT`](#addr-limit-32bit)
  - [`SHORT_INODE`](#short-inode)
  - [`WHOLE_SECONDS`](#whole-seconds)
  - [`STICKY_TIMEOUTS`](#sticky-timeouts)
  - [`ADDR_LIMIT_3GB`](#addr-limit-3gb)
  - [`PTRACE_O_TRACESYSGOOD`](#ptrace-o-tracesysgood)
  - [`PTRACE_O_TRACEFORK`](#ptrace-o-tracefork)
  - [`PTRACE_O_TRACEVFORK`](#ptrace-o-tracevfork)
  - [`PTRACE_O_TRACECLONE`](#ptrace-o-traceclone)
  - [`PTRACE_O_TRACEEXEC`](#ptrace-o-traceexec)
  - [`PTRACE_O_TRACEVFORKDONE`](#ptrace-o-tracevforkdone)
  - [`PTRACE_O_TRACEEXIT`](#ptrace-o-traceexit)
  - [`PTRACE_O_TRACESECCOMP`](#ptrace-o-traceseccomp)
  - [`PTRACE_O_SUSPEND_SECCOMP`](#ptrace-o-suspend-seccomp)
  - [`PTRACE_O_EXITKILL`](#ptrace-o-exitkill)
  - [`PTRACE_O_MASK`](#ptrace-o-mask)
  - [`PTRACE_EVENT_FORK`](#ptrace-event-fork)
  - [`PTRACE_EVENT_VFORK`](#ptrace-event-vfork)
  - [`PTRACE_EVENT_CLONE`](#ptrace-event-clone)
  - [`PTRACE_EVENT_EXEC`](#ptrace-event-exec)
  - [`PTRACE_EVENT_VFORK_DONE`](#ptrace-event-vfork-done)
  - [`PTRACE_EVENT_EXIT`](#ptrace-event-exit)
  - [`PTRACE_EVENT_SECCOMP`](#ptrace-event-seccomp)
  - [`__WNOTHREAD`](#wnothread)
  - [`__WALL`](#wall)
  - [`__WCLONE`](#wclone)
  - [`SPLICE_F_MOVE`](#splice-f-move)
  - [`SPLICE_F_NONBLOCK`](#splice-f-nonblock)
  - [`SPLICE_F_MORE`](#splice-f-more)
  - [`SPLICE_F_GIFT`](#splice-f-gift)
  - [`RTLD_LOCAL`](#rtld-local)
  - [`RTLD_LAZY`](#rtld-lazy)
  - [`POSIX_FADV_NORMAL`](#posix-fadv-normal)
  - [`POSIX_FADV_RANDOM`](#posix-fadv-random)
  - [`POSIX_FADV_SEQUENTIAL`](#posix-fadv-sequential)
  - [`POSIX_FADV_WILLNEED`](#posix-fadv-willneed)
  - [`AT_FDCWD`](#at-fdcwd)
  - [`AT_SYMLINK_NOFOLLOW`](#at-symlink-nofollow)
  - [`AT_REMOVEDIR`](#at-removedir)
  - [`AT_SYMLINK_FOLLOW`](#at-symlink-follow)
  - [`AT_NO_AUTOMOUNT`](#at-no-automount)
  - [`AT_EMPTY_PATH`](#at-empty-path)
  - [`AT_RECURSIVE`](#at-recursive)
  - [`LOG_CRON`](#log-cron)
  - [`LOG_AUTHPRIV`](#log-authpriv)
  - [`LOG_FTP`](#log-ftp)
  - [`LOG_PERROR`](#log-perror)
  - [`PIPE_BUF`](#pipe-buf)
  - [`SI_LOAD_SHIFT`](#si-load-shift)
  - [`SI_USER`](#si-user)
  - [`SI_KERNEL`](#si-kernel)
  - [`SI_QUEUE`](#si-queue)
  - [`SI_TIMER`](#si-timer)
  - [`SI_MESGQ`](#si-mesgq)
  - [`SI_ASYNCIO`](#si-asyncio)
  - [`SI_SIGIO`](#si-sigio)
  - [`SI_TKILL`](#si-tkill)
  - [`SI_ASYNCNL`](#si-asyncnl)
  - [`BUS_ADRALN`](#bus-adraln)
  - [`BUS_ADRERR`](#bus-adrerr)
  - [`BUS_OBJERR`](#bus-objerr)
  - [`BUS_MCEERR_AR`](#bus-mceerr-ar)
  - [`BUS_MCEERR_AO`](#bus-mceerr-ao)
  - [`TRAP_BRKPT`](#trap-brkpt)
  - [`TRAP_TRACE`](#trap-trace)
  - [`TRAP_BRANCH`](#trap-branch)
  - [`TRAP_HWBKPT`](#trap-hwbkpt)
  - [`TRAP_UNK`](#trap-unk)
  - [`CLD_EXITED`](#cld-exited)
  - [`CLD_KILLED`](#cld-killed)
  - [`CLD_DUMPED`](#cld-dumped)
  - [`CLD_TRAPPED`](#cld-trapped)
  - [`CLD_STOPPED`](#cld-stopped)
  - [`CLD_CONTINUED`](#cld-continued)
  - [`SIGEV_SIGNAL`](#sigev-signal)
  - [`SIGEV_NONE`](#sigev-none)
  - [`SIGEV_THREAD`](#sigev-thread)
  - [`P_ALL`](#p-all)
  - [`P_PID`](#p-pid)
  - [`P_PGID`](#p-pgid)
  - [`P_PIDFD`](#p-pidfd)
  - [`UTIME_OMIT`](#utime-omit)
  - [`UTIME_NOW`](#utime-now)
  - [`POLLIN`](#pollin)
  - [`POLLPRI`](#pollpri)
  - [`POLLOUT`](#pollout)
  - [`POLLERR`](#pollerr)
  - [`POLLHUP`](#pollhup)
  - [`POLLNVAL`](#pollnval)
  - [`POLLRDNORM`](#pollrdnorm)
  - [`POLLRDBAND`](#pollrdband)
  - [`POLLRDHUP`](#pollrdhup)
  - [`IPTOS_LOWDELAY`](#iptos-lowdelay)
  - [`IPTOS_THROUGHPUT`](#iptos-throughput)
  - [`IPTOS_RELIABILITY`](#iptos-reliability)
  - [`IPTOS_MINCOST`](#iptos-mincost)
  - [`IPTOS_PREC_NETCONTROL`](#iptos-prec-netcontrol)
  - [`IPTOS_PREC_INTERNETCONTROL`](#iptos-prec-internetcontrol)
  - [`IPTOS_PREC_CRITIC_ECP`](#iptos-prec-critic-ecp)
  - [`IPTOS_PREC_FLASHOVERRIDE`](#iptos-prec-flashoverride)
  - [`IPTOS_PREC_FLASH`](#iptos-prec-flash)
  - [`IPTOS_PREC_IMMEDIATE`](#iptos-prec-immediate)
  - [`IPTOS_PREC_PRIORITY`](#iptos-prec-priority)
  - [`IPTOS_PREC_ROUTINE`](#iptos-prec-routine)
  - [`IPTOS_ECN_MASK`](#iptos-ecn-mask)
  - [`IPTOS_ECN_ECT1`](#iptos-ecn-ect1)
  - [`IPTOS_ECN_ECT0`](#iptos-ecn-ect0)
  - [`IPTOS_ECN_CE`](#iptos-ecn-ce)
  - [`IPOPT_COPY`](#ipopt-copy)
  - [`IPOPT_CLASS_MASK`](#ipopt-class-mask)
  - [`IPOPT_NUMBER_MASK`](#ipopt-number-mask)
  - [`IPOPT_CONTROL`](#ipopt-control)
  - [`IPOPT_RESERVED1`](#ipopt-reserved1)
  - [`IPOPT_MEASUREMENT`](#ipopt-measurement)
  - [`IPOPT_RESERVED2`](#ipopt-reserved2)
  - [`IPOPT_END`](#ipopt-end)
  - [`IPOPT_NOOP`](#ipopt-noop)
  - [`IPOPT_SEC`](#ipopt-sec)
  - [`IPOPT_LSRR`](#ipopt-lsrr)
  - [`IPOPT_TIMESTAMP`](#ipopt-timestamp)
  - [`IPOPT_RR`](#ipopt-rr)
  - [`IPOPT_SID`](#ipopt-sid)
  - [`IPOPT_SSRR`](#ipopt-ssrr)
  - [`IPOPT_RA`](#ipopt-ra)
  - [`IPVERSION`](#ipversion)
  - [`MAXTTL`](#maxttl)
  - [`IPDEFTTL`](#ipdefttl)
  - [`IPOPT_OPTVAL`](#ipopt-optval)
  - [`IPOPT_OLEN`](#ipopt-olen)
  - [`IPOPT_OFFSET`](#ipopt-offset)
  - [`IPOPT_MINOFF`](#ipopt-minoff)
  - [`MAX_IPOPTLEN`](#max-ipoptlen)
  - [`IPOPT_NOP`](#ipopt-nop)
  - [`IPOPT_EOL`](#ipopt-eol)
  - [`IPOPT_TS`](#ipopt-ts)
  - [`IPOPT_TS_TSONLY`](#ipopt-ts-tsonly)
  - [`IPOPT_TS_TSANDADDR`](#ipopt-ts-tsandaddr)
  - [`IPOPT_TS_PRESPEC`](#ipopt-ts-prespec)
  - [`ARPOP_RREQUEST`](#arpop-rrequest)
  - [`ARPOP_RREPLY`](#arpop-rreply)
  - [`ARPOP_InREQUEST`](#arpop-inrequest)
  - [`ARPOP_InREPLY`](#arpop-inreply)
  - [`ARPOP_NAK`](#arpop-nak)
  - [`ATF_NETMASK`](#atf-netmask)
  - [`ATF_DONTPUB`](#atf-dontpub)
  - [`ARPHRD_NETROM`](#arphrd-netrom)
  - [`ARPHRD_ETHER`](#arphrd-ether)
  - [`ARPHRD_EETHER`](#arphrd-eether)
  - [`ARPHRD_AX25`](#arphrd-ax25)
  - [`ARPHRD_PRONET`](#arphrd-pronet)
  - [`ARPHRD_CHAOS`](#arphrd-chaos)
  - [`ARPHRD_IEEE802`](#arphrd-ieee802)
  - [`ARPHRD_ARCNET`](#arphrd-arcnet)
  - [`ARPHRD_APPLETLK`](#arphrd-appletlk)
  - [`ARPHRD_DLCI`](#arphrd-dlci)
  - [`ARPHRD_ATM`](#arphrd-atm)
  - [`ARPHRD_METRICOM`](#arphrd-metricom)
  - [`ARPHRD_IEEE1394`](#arphrd-ieee1394)
  - [`ARPHRD_EUI64`](#arphrd-eui64)
  - [`ARPHRD_INFINIBAND`](#arphrd-infiniband)
  - [`ARPHRD_SLIP`](#arphrd-slip)
  - [`ARPHRD_CSLIP`](#arphrd-cslip)
  - [`ARPHRD_SLIP6`](#arphrd-slip6)
  - [`ARPHRD_CSLIP6`](#arphrd-cslip6)
  - [`ARPHRD_RSRVD`](#arphrd-rsrvd)
  - [`ARPHRD_ADAPT`](#arphrd-adapt)
  - [`ARPHRD_ROSE`](#arphrd-rose)
  - [`ARPHRD_X25`](#arphrd-x25)
  - [`ARPHRD_HWX25`](#arphrd-hwx25)
  - [`ARPHRD_CAN`](#arphrd-can)
  - [`ARPHRD_PPP`](#arphrd-ppp)
  - [`ARPHRD_CISCO`](#arphrd-cisco)
  - [`ARPHRD_HDLC`](#arphrd-hdlc)
  - [`ARPHRD_LAPB`](#arphrd-lapb)
  - [`ARPHRD_DDCMP`](#arphrd-ddcmp)
  - [`ARPHRD_RAWHDLC`](#arphrd-rawhdlc)
  - [`ARPHRD_TUNNEL`](#arphrd-tunnel)
  - [`ARPHRD_TUNNEL6`](#arphrd-tunnel6)
  - [`ARPHRD_FRAD`](#arphrd-frad)
  - [`ARPHRD_SKIP`](#arphrd-skip)
  - [`ARPHRD_LOOPBACK`](#arphrd-loopback)
  - [`ARPHRD_LOCALTLK`](#arphrd-localtlk)
  - [`ARPHRD_FDDI`](#arphrd-fddi)
  - [`ARPHRD_BIF`](#arphrd-bif)
  - [`ARPHRD_SIT`](#arphrd-sit)
  - [`ARPHRD_IPDDP`](#arphrd-ipddp)
  - [`ARPHRD_IPGRE`](#arphrd-ipgre)
  - [`ARPHRD_PIMREG`](#arphrd-pimreg)
  - [`ARPHRD_HIPPI`](#arphrd-hippi)
  - [`ARPHRD_ASH`](#arphrd-ash)
  - [`ARPHRD_ECONET`](#arphrd-econet)
  - [`ARPHRD_IRDA`](#arphrd-irda)
  - [`ARPHRD_FCPP`](#arphrd-fcpp)
  - [`ARPHRD_FCAL`](#arphrd-fcal)
  - [`ARPHRD_FCPL`](#arphrd-fcpl)
  - [`ARPHRD_FCFABRIC`](#arphrd-fcfabric)
  - [`ARPHRD_IEEE802_TR`](#arphrd-ieee802-tr)
  - [`ARPHRD_IEEE80211`](#arphrd-ieee80211)
  - [`ARPHRD_IEEE80211_PRISM`](#arphrd-ieee80211-prism)
  - [`ARPHRD_IEEE80211_RADIOTAP`](#arphrd-ieee80211-radiotap)
  - [`ARPHRD_IEEE802154`](#arphrd-ieee802154)
  - [`ARPHRD_VOID`](#arphrd-void)
  - [`ARPHRD_NONE`](#arphrd-none)
  - [`IFF_TUN`](#iff-tun)
  - [`IFF_TAP`](#iff-tap)
  - [`IFF_NAPI`](#iff-napi)
  - [`IFF_NAPI_FRAGS`](#iff-napi-frags)
  - [`IFF_NO_CARRIER`](#iff-no-carrier)
  - [`IFF_NO_PI`](#iff-no-pi)
  - [`TUN_READQ_SIZE`](#tun-readq-size)
  - [`TUN_TUN_DEV`](#tun-tun-dev)
  - [`TUN_TAP_DEV`](#tun-tap-dev)
  - [`TUN_TYPE_MASK`](#tun-type-mask)
  - [`IFF_ONE_QUEUE`](#iff-one-queue)
  - [`IFF_VNET_HDR`](#iff-vnet-hdr)
  - [`IFF_TUN_EXCL`](#iff-tun-excl)
  - [`IFF_MULTI_QUEUE`](#iff-multi-queue)
  - [`IFF_ATTACH_QUEUE`](#iff-attach-queue)
  - [`IFF_DETACH_QUEUE`](#iff-detach-queue)
  - [`IFF_PERSIST`](#iff-persist)
  - [`IFF_NOFILTER`](#iff-nofilter)
  - [`TUN_TX_TIMESTAMP`](#tun-tx-timestamp)
  - [`TUN_F_CSUM`](#tun-f-csum)
  - [`TUN_F_TSO4`](#tun-f-tso4)
  - [`TUN_F_TSO6`](#tun-f-tso6)
  - [`TUN_F_TSO_ECN`](#tun-f-tso-ecn)
  - [`TUN_F_UFO`](#tun-f-ufo)
  - [`TUN_F_USO4`](#tun-f-uso4)
  - [`TUN_F_USO6`](#tun-f-uso6)
  - [`TUN_PKT_STRIP`](#tun-pkt-strip)
  - [`TUN_FLT_ALLMULTI`](#tun-flt-allmulti)
  - [`T_TYPE`](#t-type)
  - [`TUNSETNOCSUM`](#tunsetnocsum)
  - [`TUNSETDEBUG`](#tunsetdebug)
  - [`TUNSETIFF`](#tunsetiff)
  - [`TUNSETPERSIST`](#tunsetpersist)
  - [`TUNSETOWNER`](#tunsetowner)
  - [`TUNSETLINK`](#tunsetlink)
  - [`TUNSETGROUP`](#tunsetgroup)
  - [`TUNGETFEATURES`](#tungetfeatures)
  - [`TUNSETOFFLOAD`](#tunsetoffload)
  - [`TUNSETTXFILTER`](#tunsettxfilter)
  - [`TUNGETIFF`](#tungetiff)
  - [`TUNGETSNDBUF`](#tungetsndbuf)
  - [`TUNSETSNDBUF`](#tunsetsndbuf)
  - [`TUNATTACHFILTER`](#tunattachfilter)
  - [`TUNDETACHFILTER`](#tundetachfilter)
  - [`TUNGETVNETHDRSZ`](#tungetvnethdrsz)
  - [`TUNSETVNETHDRSZ`](#tunsetvnethdrsz)
  - [`TUNSETQUEUE`](#tunsetqueue)
  - [`TUNSETIFINDEX`](#tunsetifindex)
  - [`TUNGETFILTER`](#tungetfilter)
  - [`TUNSETVNETLE`](#tunsetvnetle)
  - [`TUNGETVNETLE`](#tungetvnetle)
  - [`TUNSETVNETBE`](#tunsetvnetbe)
  - [`TUNGETVNETBE`](#tungetvnetbe)
  - [`TUNSETSTEERINGEBPF`](#tunsetsteeringebpf)
  - [`TUNSETFILTEREBPF`](#tunsetfilterebpf)
  - [`TUNSETCARRIER`](#tunsetcarrier)
  - [`TUNGETDEVNETNS`](#tungetdevnetns)
  - [`FS_IOC_GETFLAGS`](#fs-ioc-getflags)
  - [`FS_IOC_SETFLAGS`](#fs-ioc-setflags)
  - [`FS_IOC_GETVERSION`](#fs-ioc-getversion)
  - [`FS_IOC_SETVERSION`](#fs-ioc-setversion)
  - [`FS_IOC32_GETFLAGS`](#fs-ioc32-getflags)
  - [`FS_IOC32_SETFLAGS`](#fs-ioc32-setflags)
  - [`FS_IOC32_GETVERSION`](#fs-ioc32-getversion)
  - [`FS_IOC32_SETVERSION`](#fs-ioc32-setversion)
  - [`FICLONE`](#ficlone)
  - [`FICLONERANGE`](#ficlonerange)
  - [`ADFS_SUPER_MAGIC`](#adfs-super-magic)
  - [`AFFS_SUPER_MAGIC`](#affs-super-magic)
  - [`AFS_SUPER_MAGIC`](#afs-super-magic)
  - [`AUTOFS_SUPER_MAGIC`](#autofs-super-magic)
  - [`BPF_FS_MAGIC`](#bpf-fs-magic)
  - [`BTRFS_SUPER_MAGIC`](#btrfs-super-magic)
  - [`CGROUP2_SUPER_MAGIC`](#cgroup2-super-magic)
  - [`CGROUP_SUPER_MAGIC`](#cgroup-super-magic)
  - [`CODA_SUPER_MAGIC`](#coda-super-magic)
  - [`CRAMFS_MAGIC`](#cramfs-magic)
  - [`DEBUGFS_MAGIC`](#debugfs-magic)
  - [`DEVPTS_SUPER_MAGIC`](#devpts-super-magic)
  - [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs-super-magic)
  - [`EFS_SUPER_MAGIC`](#efs-super-magic)
  - [`EXT2_SUPER_MAGIC`](#ext2-super-magic)
  - [`EXT3_SUPER_MAGIC`](#ext3-super-magic)
  - [`EXT4_SUPER_MAGIC`](#ext4-super-magic)
  - [`F2FS_SUPER_MAGIC`](#f2fs-super-magic)
  - [`FUSE_SUPER_MAGIC`](#fuse-super-magic)
  - [`FUTEXFS_SUPER_MAGIC`](#futexfs-super-magic)
  - [`HOSTFS_SUPER_MAGIC`](#hostfs-super-magic)
  - [`HPFS_SUPER_MAGIC`](#hpfs-super-magic)
  - [`HUGETLBFS_MAGIC`](#hugetlbfs-magic)
  - [`ISOFS_SUPER_MAGIC`](#isofs-super-magic)
  - [`JFFS2_SUPER_MAGIC`](#jffs2-super-magic)
  - [`MINIX2_SUPER_MAGIC2`](#minix2-super-magic2)
  - [`MINIX2_SUPER_MAGIC`](#minix2-super-magic)
  - [`MINIX3_SUPER_MAGIC`](#minix3-super-magic)
  - [`MINIX_SUPER_MAGIC2`](#minix-super-magic2)
  - [`MINIX_SUPER_MAGIC`](#minix-super-magic)
  - [`MSDOS_SUPER_MAGIC`](#msdos-super-magic)
  - [`NCP_SUPER_MAGIC`](#ncp-super-magic)
  - [`NFS_SUPER_MAGIC`](#nfs-super-magic)
  - [`NILFS_SUPER_MAGIC`](#nilfs-super-magic)
  - [`OCFS2_SUPER_MAGIC`](#ocfs2-super-magic)
  - [`OPENPROM_SUPER_MAGIC`](#openprom-super-magic)
  - [`OVERLAYFS_SUPER_MAGIC`](#overlayfs-super-magic)
  - [`PROC_SUPER_MAGIC`](#proc-super-magic)
  - [`QNX4_SUPER_MAGIC`](#qnx4-super-magic)
  - [`QNX6_SUPER_MAGIC`](#qnx6-super-magic)
  - [`RDTGROUP_SUPER_MAGIC`](#rdtgroup-super-magic)
  - [`REISERFS_SUPER_MAGIC`](#reiserfs-super-magic)
  - [`SECURITYFS_MAGIC`](#securityfs-magic)
  - [`SELINUX_MAGIC`](#selinux-magic)
  - [`SMACK_MAGIC`](#smack-magic)
  - [`SMB_SUPER_MAGIC`](#smb-super-magic)
  - [`SYSFS_MAGIC`](#sysfs-magic)
  - [`TMPFS_MAGIC`](#tmpfs-magic)
  - [`TRACEFS_MAGIC`](#tracefs-magic)
  - [`UDF_SUPER_MAGIC`](#udf-super-magic)
  - [`USBDEVICE_SUPER_MAGIC`](#usbdevice-super-magic)
  - [`XENFS_SUPER_MAGIC`](#xenfs-super-magic)
  - [`NSFS_MAGIC`](#nsfs-magic)
  - [`AT_STATX_SYNC_TYPE`](#at-statx-sync-type)
  - [`AT_STATX_SYNC_AS_STAT`](#at-statx-sync-as-stat)
  - [`AT_STATX_FORCE_SYNC`](#at-statx-force-sync)
  - [`AT_STATX_DONT_SYNC`](#at-statx-dont-sync)
  - [`STATX_TYPE`](#statx-type)
  - [`STATX_MODE`](#statx-mode)
  - [`STATX_NLINK`](#statx-nlink)
  - [`STATX_UID`](#statx-uid)
  - [`STATX_GID`](#statx-gid)
  - [`STATX_ATIME`](#statx-atime)
  - [`STATX_MTIME`](#statx-mtime)
  - [`STATX_CTIME`](#statx-ctime)
  - [`STATX_INO`](#statx-ino)
  - [`STATX_SIZE`](#statx-size)
  - [`STATX_BLOCKS`](#statx-blocks)
  - [`STATX_BASIC_STATS`](#statx-basic-stats)
  - [`STATX_BTIME`](#statx-btime)
  - [`STATX_ALL`](#statx-all)
  - [`STATX_MNT_ID`](#statx-mnt-id)
  - [`STATX_DIOALIGN`](#statx-dioalign)
  - [`STATX__RESERVED`](#statx-reserved)
  - [`STATX_ATTR_COMPRESSED`](#statx-attr-compressed)
  - [`STATX_ATTR_IMMUTABLE`](#statx-attr-immutable)
  - [`STATX_ATTR_APPEND`](#statx-attr-append)
  - [`STATX_ATTR_NODUMP`](#statx-attr-nodump)
  - [`STATX_ATTR_ENCRYPTED`](#statx-attr-encrypted)
  - [`STATX_ATTR_AUTOMOUNT`](#statx-attr-automount)
  - [`STATX_ATTR_MOUNT_ROOT`](#statx-attr-mount-root)
  - [`STATX_ATTR_VERITY`](#statx-attr-verity)
  - [`STATX_ATTR_DAX`](#statx-attr-dax)
  - [`_IOC_NRBITS`](#ioc-nrbits)
  - [`_IOC_TYPEBITS`](#ioc-typebits)
  - [`_IOC_SIZEBITS`](#ioc-sizebits)
  - [`_IOC_DIRBITS`](#ioc-dirbits)
  - [`_IOC_NONE`](#ioc-none)
  - [`_IOC_WRITE`](#ioc-write)
  - [`_IOC_READ`](#ioc-read)
  - [`_IOC_NRMASK`](#ioc-nrmask)
  - [`_IOC_TYPEMASK`](#ioc-typemask)
  - [`_IOC_SIZEMASK`](#ioc-sizemask)
  - [`_IOC_DIRMASK`](#ioc-dirmask)
  - [`_IOC_NRSHIFT`](#ioc-nrshift)
  - [`_IOC_TYPESHIFT`](#ioc-typeshift)
  - [`_IOC_SIZESHIFT`](#ioc-sizeshift)
  - [`_IOC_DIRSHIFT`](#ioc-dirshift)
  - [`PIDTYPE_PID`](#pidtype-pid)
  - [`PIDTYPE_TGID`](#pidtype-tgid)
  - [`PIDTYPE_PGID`](#pidtype-pgid)
  - [`PIDTYPE_SID`](#pidtype-sid)
  - [`PIDTYPE_MAX`](#pidtype-max)
  - [`POSIX_SPAWN_USEVFORK`](#posix-spawn-usevfork)
  - [`POSIX_SPAWN_SETSID`](#posix-spawn-setsid)
  - [`F_SEAL_FUTURE_WRITE`](#f-seal-future-write)
  - [`F_SEAL_EXEC`](#f-seal-exec)
  - [`IFF_LOWER_UP`](#iff-lower-up)
  - [`IFF_DORMANT`](#iff-dormant)
  - [`IFF_ECHO`](#iff-echo)
  - [`AT_EXECVE_CHECK`](#at-execve-check)
  - [`MAX_HANDLE_SZ`](#max-handle-sz)
  - [`AT_HANDLE_FID`](#at-handle-fid)
  - [`AT_HANDLE_MNT_ID_UNIQUE`](#at-handle-mnt-id-unique)
  - [`AT_HANDLE_CONNECTABLE`](#at-handle-connectable)
  - [`IFA_UNSPEC`](#ifa-unspec)
  - [`IFA_ADDRESS`](#ifa-address)
  - [`IFA_LOCAL`](#ifa-local)
  - [`IFA_LABEL`](#ifa-label)
  - [`IFA_BROADCAST`](#ifa-broadcast)
  - [`IFA_ANYCAST`](#ifa-anycast)
  - [`IFA_CACHEINFO`](#ifa-cacheinfo)
  - [`IFA_MULTICAST`](#ifa-multicast)
  - [`IFA_FLAGS`](#ifa-flags)
  - [`IFA_F_SECONDARY`](#ifa-f-secondary)
  - [`IFA_F_TEMPORARY`](#ifa-f-temporary)
  - [`IFA_F_NODAD`](#ifa-f-nodad)
  - [`IFA_F_OPTIMISTIC`](#ifa-f-optimistic)
  - [`IFA_F_DADFAILED`](#ifa-f-dadfailed)
  - [`IFA_F_HOMEADDRESS`](#ifa-f-homeaddress)
  - [`IFA_F_DEPRECATED`](#ifa-f-deprecated)
  - [`IFA_F_TENTATIVE`](#ifa-f-tentative)
  - [`IFA_F_PERMANENT`](#ifa-f-permanent)
  - [`IFA_F_MANAGETEMPADDR`](#ifa-f-managetempaddr)
  - [`IFA_F_NOPREFIXROUTE`](#ifa-f-noprefixroute)
  - [`IFA_F_MCAUTOJOIN`](#ifa-f-mcautojoin)
  - [`IFA_F_STABLE_PRIVACY`](#ifa-f-stable-privacy)
  - [`RWF_HIPRI`](#rwf-hipri)
  - [`RWF_DSYNC`](#rwf-dsync)
  - [`RWF_SYNC`](#rwf-sync)
  - [`RWF_NOWAIT`](#rwf-nowait)
  - [`RWF_APPEND`](#rwf-append)
  - [`RWF_NOAPPEND`](#rwf-noappend)
  - [`RWF_ATOMIC`](#rwf-atomic)
  - [`RWF_DONTCACHE`](#rwf-dontcache)
  - [`IFLA_UNSPEC`](#ifla-unspec)
  - [`IFLA_ADDRESS`](#ifla-address)
  - [`IFLA_BROADCAST`](#ifla-broadcast)
  - [`IFLA_IFNAME`](#ifla-ifname)
  - [`IFLA_MTU`](#ifla-mtu)
  - [`IFLA_LINK`](#ifla-link)
  - [`IFLA_QDISC`](#ifla-qdisc)
  - [`IFLA_STATS`](#ifla-stats)
  - [`IFLA_COST`](#ifla-cost)
  - [`IFLA_PRIORITY`](#ifla-priority)
  - [`IFLA_MASTER`](#ifla-master)
  - [`IFLA_WIRELESS`](#ifla-wireless)
  - [`IFLA_PROTINFO`](#ifla-protinfo)
  - [`IFLA_TXQLEN`](#ifla-txqlen)
  - [`IFLA_MAP`](#ifla-map)
  - [`IFLA_WEIGHT`](#ifla-weight)
  - [`IFLA_OPERSTATE`](#ifla-operstate)
  - [`IFLA_LINKMODE`](#ifla-linkmode)
  - [`IFLA_LINKINFO`](#ifla-linkinfo)
  - [`IFLA_NET_NS_PID`](#ifla-net-ns-pid)
  - [`IFLA_IFALIAS`](#ifla-ifalias)
  - [`IFLA_NUM_VF`](#ifla-num-vf)
  - [`IFLA_VFINFO_LIST`](#ifla-vfinfo-list)
  - [`IFLA_STATS64`](#ifla-stats64)
  - [`IFLA_VF_PORTS`](#ifla-vf-ports)
  - [`IFLA_PORT_SELF`](#ifla-port-self)
  - [`IFLA_AF_SPEC`](#ifla-af-spec)
  - [`IFLA_GROUP`](#ifla-group)
  - [`IFLA_NET_NS_FD`](#ifla-net-ns-fd)
  - [`IFLA_EXT_MASK`](#ifla-ext-mask)
  - [`IFLA_PROMISCUITY`](#ifla-promiscuity)
  - [`IFLA_NUM_TX_QUEUES`](#ifla-num-tx-queues)
  - [`IFLA_NUM_RX_QUEUES`](#ifla-num-rx-queues)
  - [`IFLA_CARRIER`](#ifla-carrier)
  - [`IFLA_PHYS_PORT_ID`](#ifla-phys-port-id)
  - [`IFLA_CARRIER_CHANGES`](#ifla-carrier-changes)
  - [`IFLA_PHYS_SWITCH_ID`](#ifla-phys-switch-id)
  - [`IFLA_LINK_NETNSID`](#ifla-link-netnsid)
  - [`IFLA_PHYS_PORT_NAME`](#ifla-phys-port-name)
  - [`IFLA_PROTO_DOWN`](#ifla-proto-down)
  - [`IFLA_GSO_MAX_SEGS`](#ifla-gso-max-segs)
  - [`IFLA_GSO_MAX_SIZE`](#ifla-gso-max-size)
  - [`IFLA_PAD`](#ifla-pad)
  - [`IFLA_XDP`](#ifla-xdp)
  - [`IFLA_EVENT`](#ifla-event)
  - [`IFLA_NEW_NETNSID`](#ifla-new-netnsid)
  - [`IFLA_IF_NETNSID`](#ifla-if-netnsid)
  - [`IFLA_TARGET_NETNSID`](#ifla-target-netnsid)
  - [`IFLA_CARRIER_UP_COUNT`](#ifla-carrier-up-count)
  - [`IFLA_CARRIER_DOWN_COUNT`](#ifla-carrier-down-count)
  - [`IFLA_NEW_IFINDEX`](#ifla-new-ifindex)
  - [`IFLA_MIN_MTU`](#ifla-min-mtu)
  - [`IFLA_MAX_MTU`](#ifla-max-mtu)
  - [`IFLA_PROP_LIST`](#ifla-prop-list)
  - [`IFLA_ALT_IFNAME`](#ifla-alt-ifname)
  - [`IFLA_PERM_ADDRESS`](#ifla-perm-address)
  - [`IFLA_PROTO_DOWN_REASON`](#ifla-proto-down-reason)
  - [`IFLA_PARENT_DEV_NAME`](#ifla-parent-dev-name)
  - [`IFLA_PARENT_DEV_BUS_NAME`](#ifla-parent-dev-bus-name)
  - [`IFLA_GRO_MAX_SIZE`](#ifla-gro-max-size)
  - [`IFLA_TSO_MAX_SIZE`](#ifla-tso-max-size)
  - [`IFLA_TSO_MAX_SEGS`](#ifla-tso-max-segs)
  - [`IFLA_ALLMULTI`](#ifla-allmulti)
  - [`IFLA_INFO_UNSPEC`](#ifla-info-unspec)
  - [`IFLA_INFO_KIND`](#ifla-info-kind)
  - [`IFLA_INFO_DATA`](#ifla-info-data)
  - [`IFLA_INFO_XSTATS`](#ifla-info-xstats)
  - [`IFLA_INFO_SLAVE_KIND`](#ifla-info-slave-kind)
  - [`IFLA_INFO_SLAVE_DATA`](#ifla-info-slave-data)
  - [`SEEK_DATA`](#seek-data)
  - [`SEEK_HOLE`](#seek-hole)
  - [`MPOL_DEFAULT`](#mpol-default)
  - [`MPOL_PREFERRED`](#mpol-preferred)
  - [`MPOL_BIND`](#mpol-bind)
  - [`MPOL_INTERLEAVE`](#mpol-interleave)
  - [`MPOL_LOCAL`](#mpol-local)
  - [`MPOL_F_NUMA_BALANCING`](#mpol-f-numa-balancing)
  - [`MPOL_F_RELATIVE_NODES`](#mpol-f-relative-nodes)
  - [`MPOL_F_STATIC_NODES`](#mpol-f-static-nodes)
  - [`PTHREAD_MUTEX_INITIALIZER`](#pthread-mutex-initializer)
  - [`PTHREAD_COND_INITIALIZER`](#pthread-cond-initializer)
  - [`PTHREAD_RWLOCK_INITIALIZER`](#pthread-rwlock-initializer)
  - [`RENAME_NOREPLACE`](#rename-noreplace)
  - [`RENAME_EXCHANGE`](#rename-exchange)
  - [`RENAME_WHITEOUT`](#rename-whiteout)
  - [`MSG_STAT`](#msg-stat)
  - [`MSG_INFO`](#msg-info)
  - [`MSG_NOTIFICATION`](#msg-notification)
  - [`MSG_NOERROR`](#msg-noerror)
  - [`MSG_EXCEPT`](#msg-except)
  - [`MSG_ZEROCOPY`](#msg-zerocopy)
  - [`SEM_UNDO`](#sem-undo)
  - [`GETPID`](#getpid)
  - [`GETVAL`](#getval)
  - [`GETALL`](#getall)
  - [`GETNCNT`](#getncnt)
  - [`GETZCNT`](#getzcnt)
  - [`SETVAL`](#setval)
  - [`SETALL`](#setall)
  - [`SEM_STAT`](#sem-stat)
  - [`SEM_INFO`](#sem-info)
  - [`SEM_STAT_ANY`](#sem-stat-any)
  - [`QFMT_VFS_OLD`](#qfmt-vfs-old)
  - [`QFMT_VFS_V0`](#qfmt-vfs-v0)
  - [`QFMT_VFS_V1`](#qfmt-vfs-v1)
  - [`EFD_SEMAPHORE`](#efd-semaphore)
  - [`RB_AUTOBOOT`](#rb-autoboot)
  - [`RB_HALT_SYSTEM`](#rb-halt-system)
  - [`RB_ENABLE_CAD`](#rb-enable-cad)
  - [`RB_DISABLE_CAD`](#rb-disable-cad)
  - [`RB_POWER_OFF`](#rb-power-off)
  - [`RB_SW_SUSPEND`](#rb-sw-suspend)
  - [`RB_KEXEC`](#rb-kexec)
  - [`SYNC_FILE_RANGE_WAIT_BEFORE`](#sync-file-range-wait-before)
  - [`SYNC_FILE_RANGE_WRITE`](#sync-file-range-write)
  - [`SYNC_FILE_RANGE_WAIT_AFTER`](#sync-file-range-wait-after)
  - [`MREMAP_MAYMOVE`](#mremap-maymove)
  - [`MREMAP_FIXED`](#mremap-fixed)
  - [`MREMAP_DONTUNMAP`](#mremap-dontunmap)
  - [`NSIO`](#nsio)
  - [`NS_GET_USERNS`](#ns-get-userns)
  - [`NS_GET_PARENT`](#ns-get-parent)
  - [`NS_GET_NSTYPE`](#ns-get-nstype)
  - [`NS_GET_OWNER_UID`](#ns-get-owner-uid)
  - [`NS_GET_MNTNS_ID`](#ns-get-mntns-id)
  - [`NS_GET_PID_FROM_PIDNS`](#ns-get-pid-from-pidns)
  - [`NS_GET_TGID_FROM_PIDNS`](#ns-get-tgid-from-pidns)
  - [`NS_GET_PID_IN_PIDNS`](#ns-get-pid-in-pidns)
  - [`NS_GET_TGID_IN_PIDNS`](#ns-get-tgid-in-pidns)
  - [`MNT_NS_INFO_SIZE_VER0`](#mnt-ns-info-size-ver0)
  - [`NS_MNT_GET_INFO`](#ns-mnt-get-info)
  - [`NS_MNT_GET_NEXT`](#ns-mnt-get-next)
  - [`NS_MNT_GET_PREV`](#ns-mnt-get-prev)
  - [`PR_SET_MDWE`](#pr-set-mdwe)
  - [`PR_GET_MDWE`](#pr-get-mdwe)
  - [`PR_MDWE_REFUSE_EXEC_GAIN`](#pr-mdwe-refuse-exec-gain)
  - [`PR_MDWE_NO_INHERIT`](#pr-mdwe-no-inherit)
  - [`GRND_NONBLOCK`](#grnd-nonblock)
  - [`GRND_RANDOM`](#grnd-random)
  - [`GRND_INSECURE`](#grnd-insecure)
  - [`SECCOMP_MODE_DISABLED`](#seccomp-mode-disabled)
  - [`SECCOMP_MODE_STRICT`](#seccomp-mode-strict)
  - [`SECCOMP_MODE_FILTER`](#seccomp-mode-filter)
  - [`SECCOMP_SET_MODE_STRICT`](#seccomp-set-mode-strict)
  - [`SECCOMP_SET_MODE_FILTER`](#seccomp-set-mode-filter)
  - [`SECCOMP_GET_ACTION_AVAIL`](#seccomp-get-action-avail)
  - [`SECCOMP_GET_NOTIF_SIZES`](#seccomp-get-notif-sizes)
  - [`SECCOMP_FILTER_FLAG_TSYNC`](#seccomp-filter-flag-tsync)
  - [`SECCOMP_FILTER_FLAG_LOG`](#seccomp-filter-flag-log)
  - [`SECCOMP_FILTER_FLAG_SPEC_ALLOW`](#seccomp-filter-flag-spec-allow)
  - [`SECCOMP_FILTER_FLAG_NEW_LISTENER`](#seccomp-filter-flag-new-listener)
  - [`SECCOMP_FILTER_FLAG_TSYNC_ESRCH`](#seccomp-filter-flag-tsync-esrch)
  - [`SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV`](#seccomp-filter-flag-wait-killable-recv)
  - [`SECCOMP_RET_KILL_PROCESS`](#seccomp-ret-kill-process)
  - [`SECCOMP_RET_KILL_THREAD`](#seccomp-ret-kill-thread)
  - [`SECCOMP_RET_KILL`](#seccomp-ret-kill)
  - [`SECCOMP_RET_TRAP`](#seccomp-ret-trap)
  - [`SECCOMP_RET_ERRNO`](#seccomp-ret-errno)
  - [`SECCOMP_RET_USER_NOTIF`](#seccomp-ret-user-notif)
  - [`SECCOMP_RET_TRACE`](#seccomp-ret-trace)
  - [`SECCOMP_RET_LOG`](#seccomp-ret-log)
  - [`SECCOMP_RET_ALLOW`](#seccomp-ret-allow)
  - [`SECCOMP_RET_ACTION_FULL`](#seccomp-ret-action-full)
  - [`SECCOMP_RET_ACTION`](#seccomp-ret-action)
  - [`SECCOMP_RET_DATA`](#seccomp-ret-data)
  - [`SECCOMP_USER_NOTIF_FLAG_CONTINUE`](#seccomp-user-notif-flag-continue)
  - [`SECCOMP_ADDFD_FLAG_SETFD`](#seccomp-addfd-flag-setfd)
  - [`SECCOMP_ADDFD_FLAG_SEND`](#seccomp-addfd-flag-send)
  - [`TFD_CLOEXEC`](#tfd-cloexec)
  - [`TFD_NONBLOCK`](#tfd-nonblock)
  - [`TFD_TIMER_ABSTIME`](#tfd-timer-abstime)
  - [`TFD_TIMER_CANCEL_ON_SET`](#tfd-timer-cancel-on-set)
  - [`FALLOC_FL_KEEP_SIZE`](#falloc-fl-keep-size)
  - [`FALLOC_FL_PUNCH_HOLE`](#falloc-fl-punch-hole)
  - [`FALLOC_FL_COLLAPSE_RANGE`](#falloc-fl-collapse-range)
  - [`FALLOC_FL_ZERO_RANGE`](#falloc-fl-zero-range)
  - [`FALLOC_FL_INSERT_RANGE`](#falloc-fl-insert-range)
  - [`FALLOC_FL_UNSHARE_RANGE`](#falloc-fl-unshare-range)
  - [`ENOATTR`](#enoattr)
  - [`SO_ORIGINAL_DST`](#so-original-dst)
  - [`IP_RECVFRAGSIZE`](#ip-recvfragsize)
  - [`IPV6_FLOWINFO`](#ipv6-flowinfo)
  - [`IPV6_FLOWLABEL_MGR`](#ipv6-flowlabel-mgr)
  - [`IPV6_FLOWINFO_SEND`](#ipv6-flowinfo-send)
  - [`IPV6_RECVFRAGSIZE`](#ipv6-recvfragsize)
  - [`IPV6_FREEBIND`](#ipv6-freebind)
  - [`IPV6_FLOWINFO_FLOWLABEL`](#ipv6-flowinfo-flowlabel)
  - [`IPV6_FLOWINFO_PRIORITY`](#ipv6-flowinfo-priority)
  - [`SK_MEMINFO_RMEM_ALLOC`](#sk-meminfo-rmem-alloc)
  - [`SK_MEMINFO_RCVBUF`](#sk-meminfo-rcvbuf)
  - [`SK_MEMINFO_WMEM_ALLOC`](#sk-meminfo-wmem-alloc)
  - [`SK_MEMINFO_SNDBUF`](#sk-meminfo-sndbuf)
  - [`SK_MEMINFO_FWD_ALLOC`](#sk-meminfo-fwd-alloc)
  - [`SK_MEMINFO_WMEM_QUEUED`](#sk-meminfo-wmem-queued)
  - [`SK_MEMINFO_OPTMEM`](#sk-meminfo-optmem)
  - [`SK_MEMINFO_BACKLOG`](#sk-meminfo-backlog)
  - [`SK_MEMINFO_DROPS`](#sk-meminfo-drops)
  - [`CLOSE_RANGE_UNSHARE`](#close-range-unshare)
  - [`CLOSE_RANGE_CLOEXEC`](#close-range-cloexec)
  - [`SKF_AD_OFF`](#skf-ad-off)
  - [`SKF_AD_PROTOCOL`](#skf-ad-protocol)
  - [`SKF_AD_PKTTYPE`](#skf-ad-pkttype)
  - [`SKF_AD_IFINDEX`](#skf-ad-ifindex)
  - [`SKF_AD_NLATTR`](#skf-ad-nlattr)
  - [`SKF_AD_NLATTR_NEST`](#skf-ad-nlattr-nest)
  - [`SKF_AD_MARK`](#skf-ad-mark)
  - [`SKF_AD_QUEUE`](#skf-ad-queue)
  - [`SKF_AD_HATYPE`](#skf-ad-hatype)
  - [`SKF_AD_RXHASH`](#skf-ad-rxhash)
  - [`SKF_AD_CPU`](#skf-ad-cpu)
  - [`SKF_AD_ALU_XOR_X`](#skf-ad-alu-xor-x)
  - [`SKF_AD_VLAN_TAG`](#skf-ad-vlan-tag)
  - [`SKF_AD_VLAN_TAG_PRESENT`](#skf-ad-vlan-tag-present)
  - [`SKF_AD_PAY_OFFSET`](#skf-ad-pay-offset)
  - [`SKF_AD_RANDOM`](#skf-ad-random)
  - [`SKF_AD_VLAN_TPID`](#skf-ad-vlan-tpid)
  - [`SKF_AD_MAX`](#skf-ad-max)
  - [`SKF_NET_OFF`](#skf-net-off)
  - [`SKF_LL_OFF`](#skf-ll-off)
  - [`BPF_NET_OFF`](#bpf-net-off)
  - [`BPF_LL_OFF`](#bpf-ll-off)
  - [`BPF_MEMWORDS`](#bpf-memwords)
  - [`BPF_MAXINSNS`](#bpf-maxinsns)
  - [`BPF_LD`](#bpf-ld)
  - [`BPF_LDX`](#bpf-ldx)
  - [`BPF_ST`](#bpf-st)
  - [`BPF_STX`](#bpf-stx)
  - [`BPF_ALU`](#bpf-alu)
  - [`BPF_JMP`](#bpf-jmp)
  - [`BPF_RET`](#bpf-ret)
  - [`BPF_MISC`](#bpf-misc)
  - [`BPF_W`](#bpf-w)
  - [`BPF_H`](#bpf-h)
  - [`BPF_B`](#bpf-b)
  - [`BPF_IMM`](#bpf-imm)
  - [`BPF_ABS`](#bpf-abs)
  - [`BPF_IND`](#bpf-ind)
  - [`BPF_MEM`](#bpf-mem)
  - [`BPF_LEN`](#bpf-len)
  - [`BPF_MSH`](#bpf-msh)
  - [`BPF_ADD`](#bpf-add)
  - [`BPF_SUB`](#bpf-sub)
  - [`BPF_MUL`](#bpf-mul)
  - [`BPF_DIV`](#bpf-div)
  - [`BPF_OR`](#bpf-or)
  - [`BPF_AND`](#bpf-and)
  - [`BPF_LSH`](#bpf-lsh)
  - [`BPF_RSH`](#bpf-rsh)
  - [`BPF_NEG`](#bpf-neg)
  - [`BPF_MOD`](#bpf-mod)
  - [`BPF_XOR`](#bpf-xor)
  - [`BPF_JA`](#bpf-ja)
  - [`BPF_JEQ`](#bpf-jeq)
  - [`BPF_JGT`](#bpf-jgt)
  - [`BPF_JGE`](#bpf-jge)
  - [`BPF_JSET`](#bpf-jset)
  - [`BPF_K`](#bpf-k)
  - [`BPF_X`](#bpf-x)
  - [`BPF_A`](#bpf-a)
  - [`BPF_TAX`](#bpf-tax)
  - [`BPF_TXA`](#bpf-txa)
  - [`RESOLVE_NO_XDEV`](#resolve-no-xdev)
  - [`RESOLVE_NO_MAGICLINKS`](#resolve-no-magiclinks)
  - [`RESOLVE_NO_SYMLINKS`](#resolve-no-symlinks)
  - [`RESOLVE_BENEATH`](#resolve-beneath)
  - [`RESOLVE_IN_ROOT`](#resolve-in-root)
  - [`RESOLVE_CACHED`](#resolve-cached)
  - [`ETH_ALEN`](#eth-alen)
  - [`ETH_HLEN`](#eth-hlen)
  - [`ETH_ZLEN`](#eth-zlen)
  - [`ETH_DATA_LEN`](#eth-data-len)
  - [`ETH_FRAME_LEN`](#eth-frame-len)
  - [`ETH_FCS_LEN`](#eth-fcs-len)
  - [`ETH_P_LOOP`](#eth-p-loop)
  - [`ETH_P_PUP`](#eth-p-pup)
  - [`ETH_P_PUPAT`](#eth-p-pupat)
  - [`ETH_P_IP`](#eth-p-ip)
  - [`ETH_P_X25`](#eth-p-x25)
  - [`ETH_P_ARP`](#eth-p-arp)
  - [`ETH_P_BPQ`](#eth-p-bpq)
  - [`ETH_P_IEEEPUP`](#eth-p-ieeepup)
  - [`ETH_P_IEEEPUPAT`](#eth-p-ieeepupat)
  - [`ETH_P_BATMAN`](#eth-p-batman)
  - [`ETH_P_DEC`](#eth-p-dec)
  - [`ETH_P_DNA_DL`](#eth-p-dna-dl)
  - [`ETH_P_DNA_RC`](#eth-p-dna-rc)
  - [`ETH_P_DNA_RT`](#eth-p-dna-rt)
  - [`ETH_P_LAT`](#eth-p-lat)
  - [`ETH_P_DIAG`](#eth-p-diag)
  - [`ETH_P_CUST`](#eth-p-cust)
  - [`ETH_P_SCA`](#eth-p-sca)
  - [`ETH_P_TEB`](#eth-p-teb)
  - [`ETH_P_RARP`](#eth-p-rarp)
  - [`ETH_P_ATALK`](#eth-p-atalk)
  - [`ETH_P_AARP`](#eth-p-aarp)
  - [`ETH_P_8021Q`](#eth-p-8021q)
  - [`ETH_P_IPX`](#eth-p-ipx)
  - [`ETH_P_IPV6`](#eth-p-ipv6)
  - [`ETH_P_PAUSE`](#eth-p-pause)
  - [`ETH_P_SLOW`](#eth-p-slow)
  - [`ETH_P_WCCP`](#eth-p-wccp)
  - [`ETH_P_MPLS_UC`](#eth-p-mpls-uc)
  - [`ETH_P_MPLS_MC`](#eth-p-mpls-mc)
  - [`ETH_P_ATMMPOA`](#eth-p-atmmpoa)
  - [`ETH_P_PPP_DISC`](#eth-p-ppp-disc)
  - [`ETH_P_PPP_SES`](#eth-p-ppp-ses)
  - [`ETH_P_LINK_CTL`](#eth-p-link-ctl)
  - [`ETH_P_ATMFATE`](#eth-p-atmfate)
  - [`ETH_P_PAE`](#eth-p-pae)
  - [`ETH_P_AOE`](#eth-p-aoe)
  - [`ETH_P_8021AD`](#eth-p-8021ad)
  - [`ETH_P_802_EX1`](#eth-p-802-ex1)
  - [`ETH_P_TIPC`](#eth-p-tipc)
  - [`ETH_P_MACSEC`](#eth-p-macsec)
  - [`ETH_P_8021AH`](#eth-p-8021ah)
  - [`ETH_P_MVRP`](#eth-p-mvrp)
  - [`ETH_P_1588`](#eth-p-1588)
  - [`ETH_P_PRP`](#eth-p-prp)
  - [`ETH_P_FCOE`](#eth-p-fcoe)
  - [`ETH_P_TDLS`](#eth-p-tdls)
  - [`ETH_P_FIP`](#eth-p-fip)
  - [`ETH_P_80221`](#eth-p-80221)
  - [`ETH_P_LOOPBACK`](#eth-p-loopback)
  - [`ETH_P_QINQ1`](#eth-p-qinq1)
  - [`ETH_P_QINQ2`](#eth-p-qinq2)
  - [`ETH_P_QINQ3`](#eth-p-qinq3)
  - [`ETH_P_EDSA`](#eth-p-edsa)
  - [`ETH_P_AF_IUCV`](#eth-p-af-iucv)
  - [`ETH_P_802_3_MIN`](#eth-p-802-3-min)
  - [`ETH_P_802_3`](#eth-p-802-3)
  - [`ETH_P_AX25`](#eth-p-ax25)
  - [`ETH_P_ALL`](#eth-p-all)
  - [`ETH_P_802_2`](#eth-p-802-2)
  - [`ETH_P_SNAP`](#eth-p-snap)
  - [`ETH_P_DDCMP`](#eth-p-ddcmp)
  - [`ETH_P_WAN_PPP`](#eth-p-wan-ppp)
  - [`ETH_P_PPP_MP`](#eth-p-ppp-mp)
  - [`ETH_P_LOCALTALK`](#eth-p-localtalk)
  - [`ETH_P_CANFD`](#eth-p-canfd)
  - [`ETH_P_PPPTALK`](#eth-p-ppptalk)
  - [`ETH_P_TR_802_2`](#eth-p-tr-802-2)
  - [`ETH_P_MOBITEX`](#eth-p-mobitex)
  - [`ETH_P_CONTROL`](#eth-p-control)
  - [`ETH_P_IRDA`](#eth-p-irda)
  - [`ETH_P_ECONET`](#eth-p-econet)
  - [`ETH_P_HDLC`](#eth-p-hdlc)
  - [`ETH_P_ARCNET`](#eth-p-arcnet)
  - [`ETH_P_DSA`](#eth-p-dsa)
  - [`ETH_P_TRAILER`](#eth-p-trailer)
  - [`ETH_P_PHONET`](#eth-p-phonet)
  - [`ETH_P_IEEE802154`](#eth-p-ieee802154)
  - [`ETH_P_CAIF`](#eth-p-caif)
  - [`POSIX_SPAWN_RESETIDS`](#posix-spawn-resetids)
  - [`POSIX_SPAWN_SETPGROUP`](#posix-spawn-setpgroup)
  - [`POSIX_SPAWN_SETSIGDEF`](#posix-spawn-setsigdef)
  - [`POSIX_SPAWN_SETSIGMASK`](#posix-spawn-setsigmask)
  - [`POSIX_SPAWN_SETSCHEDPARAM`](#posix-spawn-setschedparam)
  - [`POSIX_SPAWN_SETSCHEDULER`](#posix-spawn-setscheduler)
  - [`NFNLGRP_NONE`](#nfnlgrp-none)
  - [`NFNLGRP_CONNTRACK_NEW`](#nfnlgrp-conntrack-new)
  - [`NFNLGRP_CONNTRACK_UPDATE`](#nfnlgrp-conntrack-update)
  - [`NFNLGRP_CONNTRACK_DESTROY`](#nfnlgrp-conntrack-destroy)
  - [`NFNLGRP_CONNTRACK_EXP_NEW`](#nfnlgrp-conntrack-exp-new)
  - [`NFNLGRP_CONNTRACK_EXP_UPDATE`](#nfnlgrp-conntrack-exp-update)
  - [`NFNLGRP_CONNTRACK_EXP_DESTROY`](#nfnlgrp-conntrack-exp-destroy)
  - [`NFNLGRP_NFTABLES`](#nfnlgrp-nftables)
  - [`NFNLGRP_ACCT_QUOTA`](#nfnlgrp-acct-quota)
  - [`NFNLGRP_NFTRACE`](#nfnlgrp-nftrace)
  - [`NFNETLINK_V0`](#nfnetlink-v0)
  - [`NFNL_SUBSYS_NONE`](#nfnl-subsys-none)
  - [`NFNL_SUBSYS_CTNETLINK`](#nfnl-subsys-ctnetlink)
  - [`NFNL_SUBSYS_CTNETLINK_EXP`](#nfnl-subsys-ctnetlink-exp)
  - [`NFNL_SUBSYS_QUEUE`](#nfnl-subsys-queue)
  - [`NFNL_SUBSYS_ULOG`](#nfnl-subsys-ulog)
  - [`NFNL_SUBSYS_OSF`](#nfnl-subsys-osf)
  - [`NFNL_SUBSYS_IPSET`](#nfnl-subsys-ipset)
  - [`NFNL_SUBSYS_ACCT`](#nfnl-subsys-acct)
  - [`NFNL_SUBSYS_CTNETLINK_TIMEOUT`](#nfnl-subsys-ctnetlink-timeout)
  - [`NFNL_SUBSYS_CTHELPER`](#nfnl-subsys-cthelper)
  - [`NFNL_SUBSYS_NFTABLES`](#nfnl-subsys-nftables)
  - [`NFNL_SUBSYS_NFT_COMPAT`](#nfnl-subsys-nft-compat)
  - [`NFNL_SUBSYS_HOOK`](#nfnl-subsys-hook)
  - [`NFNL_SUBSYS_COUNT`](#nfnl-subsys-count)
  - [`NFNL_MSG_BATCH_BEGIN`](#nfnl-msg-batch-begin)
  - [`NFNL_MSG_BATCH_END`](#nfnl-msg-batch-end)
  - [`NFNL_BATCH_UNSPEC`](#nfnl-batch-unspec)
  - [`NFNL_BATCH_GENID`](#nfnl-batch-genid)
  - [`NFULNL_MSG_PACKET`](#nfulnl-msg-packet)
  - [`NFULNL_MSG_CONFIG`](#nfulnl-msg-config)
  - [`NFULA_VLAN_UNSPEC`](#nfula-vlan-unspec)
  - [`NFULA_VLAN_PROTO`](#nfula-vlan-proto)
  - [`NFULA_VLAN_TCI`](#nfula-vlan-tci)
  - [`NFULA_UNSPEC`](#nfula-unspec)
  - [`NFULA_PACKET_HDR`](#nfula-packet-hdr)
  - [`NFULA_MARK`](#nfula-mark)
  - [`NFULA_TIMESTAMP`](#nfula-timestamp)
  - [`NFULA_IFINDEX_INDEV`](#nfula-ifindex-indev)
  - [`NFULA_IFINDEX_OUTDEV`](#nfula-ifindex-outdev)
  - [`NFULA_IFINDEX_PHYSINDEV`](#nfula-ifindex-physindev)
  - [`NFULA_IFINDEX_PHYSOUTDEV`](#nfula-ifindex-physoutdev)
  - [`NFULA_HWADDR`](#nfula-hwaddr)
  - [`NFULA_PAYLOAD`](#nfula-payload)
  - [`NFULA_PREFIX`](#nfula-prefix)
  - [`NFULA_UID`](#nfula-uid)
  - [`NFULA_SEQ`](#nfula-seq)
  - [`NFULA_SEQ_GLOBAL`](#nfula-seq-global)
  - [`NFULA_GID`](#nfula-gid)
  - [`NFULA_HWTYPE`](#nfula-hwtype)
  - [`NFULA_HWHEADER`](#nfula-hwheader)
  - [`NFULA_HWLEN`](#nfula-hwlen)
  - [`NFULA_CT`](#nfula-ct)
  - [`NFULA_CT_INFO`](#nfula-ct-info)
  - [`NFULA_VLAN`](#nfula-vlan)
  - [`NFULA_L2HDR`](#nfula-l2hdr)
  - [`NFULNL_CFG_CMD_NONE`](#nfulnl-cfg-cmd-none)
  - [`NFULNL_CFG_CMD_BIND`](#nfulnl-cfg-cmd-bind)
  - [`NFULNL_CFG_CMD_UNBIND`](#nfulnl-cfg-cmd-unbind)
  - [`NFULNL_CFG_CMD_PF_BIND`](#nfulnl-cfg-cmd-pf-bind)
  - [`NFULNL_CFG_CMD_PF_UNBIND`](#nfulnl-cfg-cmd-pf-unbind)
  - [`NFULA_CFG_UNSPEC`](#nfula-cfg-unspec)
  - [`NFULA_CFG_CMD`](#nfula-cfg-cmd)
  - [`NFULA_CFG_MODE`](#nfula-cfg-mode)
  - [`NFULA_CFG_NLBUFSIZ`](#nfula-cfg-nlbufsiz)
  - [`NFULA_CFG_TIMEOUT`](#nfula-cfg-timeout)
  - [`NFULA_CFG_QTHRESH`](#nfula-cfg-qthresh)
  - [`NFULA_CFG_FLAGS`](#nfula-cfg-flags)
  - [`NFULNL_COPY_NONE`](#nfulnl-copy-none)
  - [`NFULNL_COPY_META`](#nfulnl-copy-meta)
  - [`NFULNL_COPY_PACKET`](#nfulnl-copy-packet)
  - [`NFULNL_CFG_F_SEQ`](#nfulnl-cfg-f-seq)
  - [`NFULNL_CFG_F_SEQ_GLOBAL`](#nfulnl-cfg-f-seq-global)
  - [`NFULNL_CFG_F_CONNTRACK`](#nfulnl-cfg-f-conntrack)
  - [`NFQNL_MSG_PACKET`](#nfqnl-msg-packet)
  - [`NFQNL_MSG_VERDICT`](#nfqnl-msg-verdict)
  - [`NFQNL_MSG_CONFIG`](#nfqnl-msg-config)
  - [`NFQNL_MSG_VERDICT_BATCH`](#nfqnl-msg-verdict-batch)
  - [`NFQA_UNSPEC`](#nfqa-unspec)
  - [`NFQA_PACKET_HDR`](#nfqa-packet-hdr)
  - [`NFQA_VERDICT_HDR`](#nfqa-verdict-hdr)
  - [`NFQA_MARK`](#nfqa-mark)
  - [`NFQA_TIMESTAMP`](#nfqa-timestamp)
  - [`NFQA_IFINDEX_INDEV`](#nfqa-ifindex-indev)
  - [`NFQA_IFINDEX_OUTDEV`](#nfqa-ifindex-outdev)
  - [`NFQA_IFINDEX_PHYSINDEV`](#nfqa-ifindex-physindev)
  - [`NFQA_IFINDEX_PHYSOUTDEV`](#nfqa-ifindex-physoutdev)
  - [`NFQA_HWADDR`](#nfqa-hwaddr)
  - [`NFQA_PAYLOAD`](#nfqa-payload)
  - [`NFQA_CT`](#nfqa-ct)
  - [`NFQA_CT_INFO`](#nfqa-ct-info)
  - [`NFQA_CAP_LEN`](#nfqa-cap-len)
  - [`NFQA_SKB_INFO`](#nfqa-skb-info)
  - [`NFQA_EXP`](#nfqa-exp)
  - [`NFQA_UID`](#nfqa-uid)
  - [`NFQA_GID`](#nfqa-gid)
  - [`NFQA_SECCTX`](#nfqa-secctx)
  - [`NFQA_VLAN`](#nfqa-vlan)
  - [`NFQA_L2HDR`](#nfqa-l2hdr)
  - [`NFQA_PRIORITY`](#nfqa-priority)
  - [`NFQA_VLAN_UNSPEC`](#nfqa-vlan-unspec)
  - [`NFQA_VLAN_PROTO`](#nfqa-vlan-proto)
  - [`NFQA_VLAN_TCI`](#nfqa-vlan-tci)
  - [`NFQNL_CFG_CMD_NONE`](#nfqnl-cfg-cmd-none)
  - [`NFQNL_CFG_CMD_BIND`](#nfqnl-cfg-cmd-bind)
  - [`NFQNL_CFG_CMD_UNBIND`](#nfqnl-cfg-cmd-unbind)
  - [`NFQNL_CFG_CMD_PF_BIND`](#nfqnl-cfg-cmd-pf-bind)
  - [`NFQNL_CFG_CMD_PF_UNBIND`](#nfqnl-cfg-cmd-pf-unbind)
  - [`NFQNL_COPY_NONE`](#nfqnl-copy-none)
  - [`NFQNL_COPY_META`](#nfqnl-copy-meta)
  - [`NFQNL_COPY_PACKET`](#nfqnl-copy-packet)
  - [`NFQA_CFG_UNSPEC`](#nfqa-cfg-unspec)
  - [`NFQA_CFG_CMD`](#nfqa-cfg-cmd)
  - [`NFQA_CFG_PARAMS`](#nfqa-cfg-params)
  - [`NFQA_CFG_QUEUE_MAXLEN`](#nfqa-cfg-queue-maxlen)
  - [`NFQA_CFG_MASK`](#nfqa-cfg-mask)
  - [`NFQA_CFG_FLAGS`](#nfqa-cfg-flags)
  - [`NFQA_CFG_F_FAIL_OPEN`](#nfqa-cfg-f-fail-open)
  - [`NFQA_CFG_F_CONNTRACK`](#nfqa-cfg-f-conntrack)
  - [`NFQA_CFG_F_GSO`](#nfqa-cfg-f-gso)
  - [`NFQA_CFG_F_UID_GID`](#nfqa-cfg-f-uid-gid)
  - [`NFQA_CFG_F_SECCTX`](#nfqa-cfg-f-secctx)
  - [`NFQA_CFG_F_MAX`](#nfqa-cfg-f-max)
  - [`NFQA_SKB_CSUMNOTREADY`](#nfqa-skb-csumnotready)
  - [`NFQA_SKB_GSO`](#nfqa-skb-gso)
  - [`NFQA_SKB_CSUM_NOTVERIFIED`](#nfqa-skb-csum-notverified)
  - [`GENL_NAMSIZ`](#genl-namsiz)
  - [`GENL_MIN_ID`](#genl-min-id)
  - [`GENL_MAX_ID`](#genl-max-id)
  - [`GENL_ADMIN_PERM`](#genl-admin-perm)
  - [`GENL_CMD_CAP_DO`](#genl-cmd-cap-do)
  - [`GENL_CMD_CAP_DUMP`](#genl-cmd-cap-dump)
  - [`GENL_CMD_CAP_HASPOL`](#genl-cmd-cap-haspol)
  - [`GENL_ID_CTRL`](#genl-id-ctrl)
  - [`CTRL_CMD_UNSPEC`](#ctrl-cmd-unspec)
  - [`CTRL_CMD_NEWFAMILY`](#ctrl-cmd-newfamily)
  - [`CTRL_CMD_DELFAMILY`](#ctrl-cmd-delfamily)
  - [`CTRL_CMD_GETFAMILY`](#ctrl-cmd-getfamily)
  - [`CTRL_CMD_NEWOPS`](#ctrl-cmd-newops)
  - [`CTRL_CMD_DELOPS`](#ctrl-cmd-delops)
  - [`CTRL_CMD_GETOPS`](#ctrl-cmd-getops)
  - [`CTRL_CMD_NEWMCAST_GRP`](#ctrl-cmd-newmcast-grp)
  - [`CTRL_CMD_DELMCAST_GRP`](#ctrl-cmd-delmcast-grp)
  - [`CTRL_CMD_GETMCAST_GRP`](#ctrl-cmd-getmcast-grp)
  - [`CTRL_ATTR_UNSPEC`](#ctrl-attr-unspec)
  - [`CTRL_ATTR_FAMILY_ID`](#ctrl-attr-family-id)
  - [`CTRL_ATTR_FAMILY_NAME`](#ctrl-attr-family-name)
  - [`CTRL_ATTR_VERSION`](#ctrl-attr-version)
  - [`CTRL_ATTR_HDRSIZE`](#ctrl-attr-hdrsize)
  - [`CTRL_ATTR_MAXATTR`](#ctrl-attr-maxattr)
  - [`CTRL_ATTR_OPS`](#ctrl-attr-ops)
  - [`CTRL_ATTR_MCAST_GROUPS`](#ctrl-attr-mcast-groups)
  - [`CTRL_ATTR_OP_UNSPEC`](#ctrl-attr-op-unspec)
  - [`CTRL_ATTR_OP_ID`](#ctrl-attr-op-id)
  - [`CTRL_ATTR_OP_FLAGS`](#ctrl-attr-op-flags)
  - [`CTRL_ATTR_MCAST_GRP_UNSPEC`](#ctrl-attr-mcast-grp-unspec)
  - [`CTRL_ATTR_MCAST_GRP_NAME`](#ctrl-attr-mcast-grp-name)
  - [`CTRL_ATTR_MCAST_GRP_ID`](#ctrl-attr-mcast-grp-id)
  - [`PACKET_FANOUT`](#packet-fanout)
  - [`PACKET_TX_HAS_OFF`](#packet-tx-has-off)
  - [`PACKET_QDISC_BYPASS`](#packet-qdisc-bypass)
  - [`PACKET_ROLLOVER_STATS`](#packet-rollover-stats)
  - [`PACKET_FANOUT_DATA`](#packet-fanout-data)
  - [`PACKET_IGNORE_OUTGOING`](#packet-ignore-outgoing)
  - [`PACKET_VNET_HDR_SZ`](#packet-vnet-hdr-sz)
  - [`PACKET_FANOUT_HASH`](#packet-fanout-hash)
  - [`PACKET_FANOUT_LB`](#packet-fanout-lb)
  - [`PACKET_FANOUT_CPU`](#packet-fanout-cpu)
  - [`PACKET_FANOUT_ROLLOVER`](#packet-fanout-rollover)
  - [`PACKET_FANOUT_RND`](#packet-fanout-rnd)
  - [`PACKET_FANOUT_QM`](#packet-fanout-qm)
  - [`PACKET_FANOUT_CBPF`](#packet-fanout-cbpf)
  - [`PACKET_FANOUT_EBPF`](#packet-fanout-ebpf)
  - [`PACKET_FANOUT_FLAG_ROLLOVER`](#packet-fanout-flag-rollover)
  - [`PACKET_FANOUT_FLAG_UNIQUEID`](#packet-fanout-flag-uniqueid)
  - [`PACKET_FANOUT_FLAG_IGNORE_OUTGOING`](#packet-fanout-flag-ignore-outgoing)
  - [`PACKET_FANOUT_FLAG_DEFRAG`](#packet-fanout-flag-defrag)
  - [`TP_STATUS_KERNEL`](#tp-status-kernel)
  - [`TP_STATUS_USER`](#tp-status-user)
  - [`TP_STATUS_COPY`](#tp-status-copy)
  - [`TP_STATUS_LOSING`](#tp-status-losing)
  - [`TP_STATUS_CSUMNOTREADY`](#tp-status-csumnotready)
  - [`TP_STATUS_VLAN_VALID`](#tp-status-vlan-valid)
  - [`TP_STATUS_BLK_TMO`](#tp-status-blk-tmo)
  - [`TP_STATUS_VLAN_TPID_VALID`](#tp-status-vlan-tpid-valid)
  - [`TP_STATUS_CSUM_VALID`](#tp-status-csum-valid)
  - [`TP_STATUS_AVAILABLE`](#tp-status-available)
  - [`TP_STATUS_SEND_REQUEST`](#tp-status-send-request)
  - [`TP_STATUS_SENDING`](#tp-status-sending)
  - [`TP_STATUS_WRONG_FORMAT`](#tp-status-wrong-format)
  - [`TP_STATUS_TS_SOFTWARE`](#tp-status-ts-software)
  - [`TP_STATUS_TS_SYS_HARDWARE`](#tp-status-ts-sys-hardware)
  - [`TP_STATUS_TS_RAW_HARDWARE`](#tp-status-ts-raw-hardware)
  - [`TP_FT_REQ_FILL_RXHASH`](#tp-ft-req-fill-rxhash)
  - [`TPACKET_ALIGNMENT`](#tpacket-alignment)
  - [`TPACKET_HDRLEN`](#tpacket-hdrlen)
  - [`TPACKET2_HDRLEN`](#tpacket2-hdrlen)
  - [`TPACKET3_HDRLEN`](#tpacket3-hdrlen)
  - [`NF_DROP`](#nf-drop)
  - [`NF_ACCEPT`](#nf-accept)
  - [`NF_STOLEN`](#nf-stolen)
  - [`NF_QUEUE`](#nf-queue)
  - [`NF_REPEAT`](#nf-repeat)
  - [`NF_STOP`](#nf-stop)
  - [`NF_MAX_VERDICT`](#nf-max-verdict)
  - [`NF_VERDICT_MASK`](#nf-verdict-mask)
  - [`NF_VERDICT_FLAG_QUEUE_BYPASS`](#nf-verdict-flag-queue-bypass)
  - [`NF_VERDICT_QMASK`](#nf-verdict-qmask)
  - [`NF_VERDICT_QBITS`](#nf-verdict-qbits)
  - [`NF_VERDICT_BITS`](#nf-verdict-bits)
  - [`NF_INET_PRE_ROUTING`](#nf-inet-pre-routing)
  - [`NF_INET_LOCAL_IN`](#nf-inet-local-in)
  - [`NF_INET_FORWARD`](#nf-inet-forward)
  - [`NF_INET_LOCAL_OUT`](#nf-inet-local-out)
  - [`NF_INET_POST_ROUTING`](#nf-inet-post-routing)
  - [`NF_INET_NUMHOOKS`](#nf-inet-numhooks)
  - [`NF_INET_INGRESS`](#nf-inet-ingress)
  - [`NF_NETDEV_INGRESS`](#nf-netdev-ingress)
  - [`NF_NETDEV_EGRESS`](#nf-netdev-egress)
  - [`NF_NETDEV_NUMHOOKS`](#nf-netdev-numhooks)
  - [`NFPROTO_UNSPEC`](#nfproto-unspec)
  - [`NFPROTO_INET`](#nfproto-inet)
  - [`NFPROTO_IPV4`](#nfproto-ipv4)
  - [`NFPROTO_ARP`](#nfproto-arp)
  - [`NFPROTO_NETDEV`](#nfproto-netdev)
  - [`NFPROTO_BRIDGE`](#nfproto-bridge)
  - [`NFPROTO_IPV6`](#nfproto-ipv6)
  - [`NFPROTO_DECNET`](#nfproto-decnet)
  - [`NFPROTO_NUMPROTO`](#nfproto-numproto)
  - [`NF_ARP`](#nf-arp)
  - [`NF_ARP_IN`](#nf-arp-in)
  - [`NF_ARP_OUT`](#nf-arp-out)
  - [`NF_ARP_FORWARD`](#nf-arp-forward)
  - [`NF_ARP_NUMHOOKS`](#nf-arp-numhooks)
  - [`NF_BR_PRE_ROUTING`](#nf-br-pre-routing)
  - [`NF_BR_LOCAL_IN`](#nf-br-local-in)
  - [`NF_BR_FORWARD`](#nf-br-forward)
  - [`NF_BR_LOCAL_OUT`](#nf-br-local-out)
  - [`NF_BR_POST_ROUTING`](#nf-br-post-routing)
  - [`NF_BR_BROUTING`](#nf-br-brouting)
  - [`NF_BR_NUMHOOKS`](#nf-br-numhooks)
  - [`NF_BR_PRI_FIRST`](#nf-br-pri-first)
  - [`NF_BR_PRI_NAT_DST_BRIDGED`](#nf-br-pri-nat-dst-bridged)
  - [`NF_BR_PRI_FILTER_BRIDGED`](#nf-br-pri-filter-bridged)
  - [`NF_BR_PRI_BRNF`](#nf-br-pri-brnf)
  - [`NF_BR_PRI_NAT_DST_OTHER`](#nf-br-pri-nat-dst-other)
  - [`NF_BR_PRI_FILTER_OTHER`](#nf-br-pri-filter-other)
  - [`NF_BR_PRI_NAT_SRC`](#nf-br-pri-nat-src)
  - [`NF_BR_PRI_LAST`](#nf-br-pri-last)
  - [`NF_IP_PRE_ROUTING`](#nf-ip-pre-routing)
  - [`NF_IP_LOCAL_IN`](#nf-ip-local-in)
  - [`NF_IP_FORWARD`](#nf-ip-forward)
  - [`NF_IP_LOCAL_OUT`](#nf-ip-local-out)
  - [`NF_IP_POST_ROUTING`](#nf-ip-post-routing)
  - [`NF_IP_NUMHOOKS`](#nf-ip-numhooks)
  - [`NF_IP_PRI_FIRST`](#nf-ip-pri-first)
  - [`NF_IP_PRI_RAW_BEFORE_DEFRAG`](#nf-ip-pri-raw-before-defrag)
  - [`NF_IP_PRI_CONNTRACK_DEFRAG`](#nf-ip-pri-conntrack-defrag)
  - [`NF_IP_PRI_RAW`](#nf-ip-pri-raw)
  - [`NF_IP_PRI_SELINUX_FIRST`](#nf-ip-pri-selinux-first)
  - [`NF_IP_PRI_CONNTRACK`](#nf-ip-pri-conntrack)
  - [`NF_IP_PRI_MANGLE`](#nf-ip-pri-mangle)
  - [`NF_IP_PRI_NAT_DST`](#nf-ip-pri-nat-dst)
  - [`NF_IP_PRI_FILTER`](#nf-ip-pri-filter)
  - [`NF_IP_PRI_SECURITY`](#nf-ip-pri-security)
  - [`NF_IP_PRI_NAT_SRC`](#nf-ip-pri-nat-src)
  - [`NF_IP_PRI_SELINUX_LAST`](#nf-ip-pri-selinux-last)
  - [`NF_IP_PRI_CONNTRACK_HELPER`](#nf-ip-pri-conntrack-helper)
  - [`NF_IP_PRI_CONNTRACK_CONFIRM`](#nf-ip-pri-conntrack-confirm)
  - [`NF_IP_PRI_LAST`](#nf-ip-pri-last)
  - [`NF_IP6_PRE_ROUTING`](#nf-ip6-pre-routing)
  - [`NF_IP6_LOCAL_IN`](#nf-ip6-local-in)
  - [`NF_IP6_FORWARD`](#nf-ip6-forward)
  - [`NF_IP6_LOCAL_OUT`](#nf-ip6-local-out)
  - [`NF_IP6_POST_ROUTING`](#nf-ip6-post-routing)
  - [`NF_IP6_NUMHOOKS`](#nf-ip6-numhooks)
  - [`NF_IP6_PRI_FIRST`](#nf-ip6-pri-first)
  - [`NF_IP6_PRI_RAW_BEFORE_DEFRAG`](#nf-ip6-pri-raw-before-defrag)
  - [`NF_IP6_PRI_CONNTRACK_DEFRAG`](#nf-ip6-pri-conntrack-defrag)
  - [`NF_IP6_PRI_RAW`](#nf-ip6-pri-raw)
  - [`NF_IP6_PRI_SELINUX_FIRST`](#nf-ip6-pri-selinux-first)
  - [`NF_IP6_PRI_CONNTRACK`](#nf-ip6-pri-conntrack)
  - [`NF_IP6_PRI_MANGLE`](#nf-ip6-pri-mangle)
  - [`NF_IP6_PRI_NAT_DST`](#nf-ip6-pri-nat-dst)
  - [`NF_IP6_PRI_FILTER`](#nf-ip6-pri-filter)
  - [`NF_IP6_PRI_SECURITY`](#nf-ip6-pri-security)
  - [`NF_IP6_PRI_NAT_SRC`](#nf-ip6-pri-nat-src)
  - [`NF_IP6_PRI_SELINUX_LAST`](#nf-ip6-pri-selinux-last)
  - [`NF_IP6_PRI_CONNTRACK_HELPER`](#nf-ip6-pri-conntrack-helper)
  - [`NF_IP6_PRI_LAST`](#nf-ip6-pri-last)
  - [`IP6T_SO_ORIGINAL_DST`](#ip6t-so-original-dst)
  - [`SIOCSHWTSTAMP`](#siocshwtstamp)
  - [`SIOCGHWTSTAMP`](#siocghwtstamp)
  - [`WIRELESS_EXT`](#wireless-ext)
  - [`SIOCSIWCOMMIT`](#siocsiwcommit)
  - [`SIOCGIWNAME`](#siocgiwname)
  - [`SIOCSIWNWID`](#siocsiwnwid)
  - [`SIOCGIWNWID`](#siocgiwnwid)
  - [`SIOCSIWFREQ`](#siocsiwfreq)
  - [`SIOCGIWFREQ`](#siocgiwfreq)
  - [`SIOCSIWMODE`](#siocsiwmode)
  - [`SIOCGIWMODE`](#siocgiwmode)
  - [`SIOCSIWSENS`](#siocsiwsens)
  - [`SIOCGIWSENS`](#siocgiwsens)
  - [`SIOCSIWRANGE`](#siocsiwrange)
  - [`SIOCGIWRANGE`](#siocgiwrange)
  - [`SIOCSIWPRIV`](#siocsiwpriv)
  - [`SIOCGIWPRIV`](#siocgiwpriv)
  - [`SIOCSIWSTATS`](#siocsiwstats)
  - [`SIOCGIWSTATS`](#siocgiwstats)
  - [`SIOCSIWSPY`](#siocsiwspy)
  - [`SIOCGIWSPY`](#siocgiwspy)
  - [`SIOCSIWTHRSPY`](#siocsiwthrspy)
  - [`SIOCGIWTHRSPY`](#siocgiwthrspy)
  - [`SIOCSIWAP`](#siocsiwap)
  - [`SIOCGIWAP`](#siocgiwap)
  - [`SIOCGIWAPLIST`](#siocgiwaplist)
  - [`SIOCSIWSCAN`](#siocsiwscan)
  - [`SIOCGIWSCAN`](#siocgiwscan)
  - [`SIOCSIWESSID`](#siocsiwessid)
  - [`SIOCGIWESSID`](#siocgiwessid)
  - [`SIOCSIWNICKN`](#siocsiwnickn)
  - [`SIOCGIWNICKN`](#siocgiwnickn)
  - [`SIOCSIWRATE`](#siocsiwrate)
  - [`SIOCGIWRATE`](#siocgiwrate)
  - [`SIOCSIWRTS`](#siocsiwrts)
  - [`SIOCGIWRTS`](#siocgiwrts)
  - [`SIOCSIWFRAG`](#siocsiwfrag)
  - [`SIOCGIWFRAG`](#siocgiwfrag)
  - [`SIOCSIWTXPOW`](#siocsiwtxpow)
  - [`SIOCGIWTXPOW`](#siocgiwtxpow)
  - [`SIOCSIWRETRY`](#siocsiwretry)
  - [`SIOCGIWRETRY`](#siocgiwretry)
  - [`SIOCSIWENCODE`](#siocsiwencode)
  - [`SIOCGIWENCODE`](#siocgiwencode)
  - [`SIOCSIWPOWER`](#siocsiwpower)
  - [`SIOCGIWPOWER`](#siocgiwpower)
  - [`SIOCSIWGENIE`](#siocsiwgenie)
  - [`SIOCGIWGENIE`](#siocgiwgenie)
  - [`SIOCSIWMLME`](#siocsiwmlme)
  - [`SIOCSIWAUTH`](#siocsiwauth)
  - [`SIOCGIWAUTH`](#siocgiwauth)
  - [`SIOCSIWENCODEEXT`](#siocsiwencodeext)
  - [`SIOCGIWENCODEEXT`](#siocgiwencodeext)
  - [`SIOCSIWPMKSA`](#siocsiwpmksa)
  - [`SIOCIWFIRSTPRIV`](#siociwfirstpriv)
  - [`SIOCIWLASTPRIV`](#siociwlastpriv)
  - [`SIOCIWFIRST`](#siociwfirst)
  - [`SIOCIWLAST`](#siociwlast)
  - [`IWEVTXDROP`](#iwevtxdrop)
  - [`IWEVQUAL`](#iwevqual)
  - [`IWEVCUSTOM`](#iwevcustom)
  - [`IWEVREGISTERED`](#iwevregistered)
  - [`IWEVEXPIRED`](#iwevexpired)
  - [`IWEVGENIE`](#iwevgenie)
  - [`IWEVMICHAELMICFAILURE`](#iwevmichaelmicfailure)
  - [`IWEVASSOCREQIE`](#iwevassocreqie)
  - [`IWEVASSOCRESPIE`](#iwevassocrespie)
  - [`IWEVPMKIDCAND`](#iwevpmkidcand)
  - [`IWEVFIRST`](#iwevfirst)
  - [`IW_PRIV_TYPE_MASK`](#iw-priv-type-mask)
  - [`IW_PRIV_TYPE_NONE`](#iw-priv-type-none)
  - [`IW_PRIV_TYPE_BYTE`](#iw-priv-type-byte)
  - [`IW_PRIV_TYPE_CHAR`](#iw-priv-type-char)
  - [`IW_PRIV_TYPE_INT`](#iw-priv-type-int)
  - [`IW_PRIV_TYPE_FLOAT`](#iw-priv-type-float)
  - [`IW_PRIV_TYPE_ADDR`](#iw-priv-type-addr)
  - [`IW_PRIV_SIZE_FIXED`](#iw-priv-size-fixed)
  - [`IW_PRIV_SIZE_MASK`](#iw-priv-size-mask)
  - [`IW_MAX_FREQUENCIES`](#iw-max-frequencies)
  - [`IW_MAX_BITRATES`](#iw-max-bitrates)
  - [`IW_MAX_TXPOWER`](#iw-max-txpower)
  - [`IW_MAX_SPY`](#iw-max-spy)
  - [`IW_MAX_AP`](#iw-max-ap)
  - [`IW_ESSID_MAX_SIZE`](#iw-essid-max-size)
  - [`IW_MODE_AUTO`](#iw-mode-auto)
  - [`IW_MODE_ADHOC`](#iw-mode-adhoc)
  - [`IW_MODE_INFRA`](#iw-mode-infra)
  - [`IW_MODE_MASTER`](#iw-mode-master)
  - [`IW_MODE_REPEAT`](#iw-mode-repeat)
  - [`IW_MODE_SECOND`](#iw-mode-second)
  - [`IW_MODE_MONITOR`](#iw-mode-monitor)
  - [`IW_MODE_MESH`](#iw-mode-mesh)
  - [`IW_QUAL_QUAL_UPDATED`](#iw-qual-qual-updated)
  - [`IW_QUAL_LEVEL_UPDATED`](#iw-qual-level-updated)
  - [`IW_QUAL_NOISE_UPDATED`](#iw-qual-noise-updated)
  - [`IW_QUAL_ALL_UPDATED`](#iw-qual-all-updated)
  - [`IW_QUAL_DBM`](#iw-qual-dbm)
  - [`IW_QUAL_QUAL_INVALID`](#iw-qual-qual-invalid)
  - [`IW_QUAL_LEVEL_INVALID`](#iw-qual-level-invalid)
  - [`IW_QUAL_NOISE_INVALID`](#iw-qual-noise-invalid)
  - [`IW_QUAL_RCPI`](#iw-qual-rcpi)
  - [`IW_QUAL_ALL_INVALID`](#iw-qual-all-invalid)
  - [`IW_FREQ_AUTO`](#iw-freq-auto)
  - [`IW_FREQ_FIXED`](#iw-freq-fixed)
  - [`IW_MAX_ENCODING_SIZES`](#iw-max-encoding-sizes)
  - [`IW_ENCODING_TOKEN_MAX`](#iw-encoding-token-max)
  - [`IW_ENCODE_INDEX`](#iw-encode-index)
  - [`IW_ENCODE_FLAGS`](#iw-encode-flags)
  - [`IW_ENCODE_MODE`](#iw-encode-mode)
  - [`IW_ENCODE_DISABLED`](#iw-encode-disabled)
  - [`IW_ENCODE_ENABLED`](#iw-encode-enabled)
  - [`IW_ENCODE_RESTRICTED`](#iw-encode-restricted)
  - [`IW_ENCODE_OPEN`](#iw-encode-open)
  - [`IW_ENCODE_NOKEY`](#iw-encode-nokey)
  - [`IW_ENCODE_TEMP`](#iw-encode-temp)
  - [`IW_POWER_ON`](#iw-power-on)
  - [`IW_POWER_TYPE`](#iw-power-type)
  - [`IW_POWER_PERIOD`](#iw-power-period)
  - [`IW_POWER_TIMEOUT`](#iw-power-timeout)
  - [`IW_POWER_MODE`](#iw-power-mode)
  - [`IW_POWER_UNICAST_R`](#iw-power-unicast-r)
  - [`IW_POWER_MULTICAST_R`](#iw-power-multicast-r)
  - [`IW_POWER_ALL_R`](#iw-power-all-r)
  - [`IW_POWER_FORCE_S`](#iw-power-force-s)
  - [`IW_POWER_REPEATER`](#iw-power-repeater)
  - [`IW_POWER_MODIFIER`](#iw-power-modifier)
  - [`IW_POWER_MIN`](#iw-power-min)
  - [`IW_POWER_MAX`](#iw-power-max)
  - [`IW_POWER_RELATIVE`](#iw-power-relative)
  - [`IW_TXPOW_TYPE`](#iw-txpow-type)
  - [`IW_TXPOW_DBM`](#iw-txpow-dbm)
  - [`IW_TXPOW_MWATT`](#iw-txpow-mwatt)
  - [`IW_TXPOW_RELATIVE`](#iw-txpow-relative)
  - [`IW_TXPOW_RANGE`](#iw-txpow-range)
  - [`IW_RETRY_ON`](#iw-retry-on)
  - [`IW_RETRY_TYPE`](#iw-retry-type)
  - [`IW_RETRY_LIMIT`](#iw-retry-limit)
  - [`IW_RETRY_LIFETIME`](#iw-retry-lifetime)
  - [`IW_RETRY_MODIFIER`](#iw-retry-modifier)
  - [`IW_RETRY_MIN`](#iw-retry-min)
  - [`IW_RETRY_MAX`](#iw-retry-max)
  - [`IW_RETRY_RELATIVE`](#iw-retry-relative)
  - [`IW_RETRY_SHORT`](#iw-retry-short)
  - [`IW_RETRY_LONG`](#iw-retry-long)
  - [`IW_SCAN_DEFAULT`](#iw-scan-default)
  - [`IW_SCAN_ALL_ESSID`](#iw-scan-all-essid)
  - [`IW_SCAN_THIS_ESSID`](#iw-scan-this-essid)
  - [`IW_SCAN_ALL_FREQ`](#iw-scan-all-freq)
  - [`IW_SCAN_THIS_FREQ`](#iw-scan-this-freq)
  - [`IW_SCAN_ALL_MODE`](#iw-scan-all-mode)
  - [`IW_SCAN_THIS_MODE`](#iw-scan-this-mode)
  - [`IW_SCAN_ALL_RATE`](#iw-scan-all-rate)
  - [`IW_SCAN_THIS_RATE`](#iw-scan-this-rate)
  - [`IW_SCAN_TYPE_ACTIVE`](#iw-scan-type-active)
  - [`IW_SCAN_TYPE_PASSIVE`](#iw-scan-type-passive)
  - [`IW_SCAN_MAX_DATA`](#iw-scan-max-data)
  - [`IW_SCAN_CAPA_NONE`](#iw-scan-capa-none)
  - [`IW_SCAN_CAPA_ESSID`](#iw-scan-capa-essid)
  - [`IW_SCAN_CAPA_BSSID`](#iw-scan-capa-bssid)
  - [`IW_SCAN_CAPA_CHANNEL`](#iw-scan-capa-channel)
  - [`IW_SCAN_CAPA_MODE`](#iw-scan-capa-mode)
  - [`IW_SCAN_CAPA_RATE`](#iw-scan-capa-rate)
  - [`IW_SCAN_CAPA_TYPE`](#iw-scan-capa-type)
  - [`IW_SCAN_CAPA_TIME`](#iw-scan-capa-time)
  - [`IW_CUSTOM_MAX`](#iw-custom-max)
  - [`IW_GENERIC_IE_MAX`](#iw-generic-ie-max)
  - [`IW_MLME_DEAUTH`](#iw-mlme-deauth)
  - [`IW_MLME_DISASSOC`](#iw-mlme-disassoc)
  - [`IW_MLME_AUTH`](#iw-mlme-auth)
  - [`IW_MLME_ASSOC`](#iw-mlme-assoc)
  - [`IW_AUTH_INDEX`](#iw-auth-index)
  - [`IW_AUTH_FLAGS`](#iw-auth-flags)
  - [`IW_AUTH_WPA_VERSION`](#iw-auth-wpa-version)
  - [`IW_AUTH_CIPHER_PAIRWISE`](#iw-auth-cipher-pairwise)
  - [`IW_AUTH_CIPHER_GROUP`](#iw-auth-cipher-group)
  - [`IW_AUTH_KEY_MGMT`](#iw-auth-key-mgmt)
  - [`IW_AUTH_TKIP_COUNTERMEASURES`](#iw-auth-tkip-countermeasures)
  - [`IW_AUTH_DROP_UNENCRYPTED`](#iw-auth-drop-unencrypted)
  - [`IW_AUTH_80211_AUTH_ALG`](#iw-auth-80211-auth-alg)
  - [`IW_AUTH_WPA_ENABLED`](#iw-auth-wpa-enabled)
  - [`IW_AUTH_RX_UNENCRYPTED_EAPOL`](#iw-auth-rx-unencrypted-eapol)
  - [`IW_AUTH_ROAMING_CONTROL`](#iw-auth-roaming-control)
  - [`IW_AUTH_PRIVACY_INVOKED`](#iw-auth-privacy-invoked)
  - [`IW_AUTH_CIPHER_GROUP_MGMT`](#iw-auth-cipher-group-mgmt)
  - [`IW_AUTH_MFP`](#iw-auth-mfp)
  - [`IW_AUTH_WPA_VERSION_DISABLED`](#iw-auth-wpa-version-disabled)
  - [`IW_AUTH_WPA_VERSION_WPA`](#iw-auth-wpa-version-wpa)
  - [`IW_AUTH_WPA_VERSION_WPA2`](#iw-auth-wpa-version-wpa2)
  - [`IW_AUTH_CIPHER_NONE`](#iw-auth-cipher-none)
  - [`IW_AUTH_CIPHER_WEP40`](#iw-auth-cipher-wep40)
  - [`IW_AUTH_CIPHER_TKIP`](#iw-auth-cipher-tkip)
  - [`IW_AUTH_CIPHER_CCMP`](#iw-auth-cipher-ccmp)
  - [`IW_AUTH_CIPHER_WEP104`](#iw-auth-cipher-wep104)
  - [`IW_AUTH_CIPHER_AES_CMAC`](#iw-auth-cipher-aes-cmac)
  - [`IW_AUTH_KEY_MGMT_802_1X`](#iw-auth-key-mgmt-802-1x)
  - [`IW_AUTH_KEY_MGMT_PSK`](#iw-auth-key-mgmt-psk)
  - [`IW_AUTH_ALG_OPEN_SYSTEM`](#iw-auth-alg-open-system)
  - [`IW_AUTH_ALG_SHARED_KEY`](#iw-auth-alg-shared-key)
  - [`IW_AUTH_ALG_LEAP`](#iw-auth-alg-leap)
  - [`IW_AUTH_ROAMING_ENABLE`](#iw-auth-roaming-enable)
  - [`IW_AUTH_ROAMING_DISABLE`](#iw-auth-roaming-disable)
  - [`IW_AUTH_MFP_DISABLED`](#iw-auth-mfp-disabled)
  - [`IW_AUTH_MFP_OPTIONAL`](#iw-auth-mfp-optional)
  - [`IW_AUTH_MFP_REQUIRED`](#iw-auth-mfp-required)
  - [`IW_ENCODE_SEQ_MAX_SIZE`](#iw-encode-seq-max-size)
  - [`IW_ENCODE_ALG_NONE`](#iw-encode-alg-none)
  - [`IW_ENCODE_ALG_WEP`](#iw-encode-alg-wep)
  - [`IW_ENCODE_ALG_TKIP`](#iw-encode-alg-tkip)
  - [`IW_ENCODE_ALG_CCMP`](#iw-encode-alg-ccmp)
  - [`IW_ENCODE_ALG_PMK`](#iw-encode-alg-pmk)
  - [`IW_ENCODE_ALG_AES_CMAC`](#iw-encode-alg-aes-cmac)
  - [`IW_ENCODE_EXT_TX_SEQ_VALID`](#iw-encode-ext-tx-seq-valid)
  - [`IW_ENCODE_EXT_RX_SEQ_VALID`](#iw-encode-ext-rx-seq-valid)
  - [`IW_ENCODE_EXT_GROUP_KEY`](#iw-encode-ext-group-key)
  - [`IW_ENCODE_EXT_SET_TX_KEY`](#iw-encode-ext-set-tx-key)
  - [`IW_MICFAILURE_KEY_ID`](#iw-micfailure-key-id)
  - [`IW_MICFAILURE_GROUP`](#iw-micfailure-group)
  - [`IW_MICFAILURE_PAIRWISE`](#iw-micfailure-pairwise)
  - [`IW_MICFAILURE_STAKEY`](#iw-micfailure-stakey)
  - [`IW_MICFAILURE_COUNT`](#iw-micfailure-count)
  - [`IW_ENC_CAPA_WPA`](#iw-enc-capa-wpa)
  - [`IW_ENC_CAPA_WPA2`](#iw-enc-capa-wpa2)
  - [`IW_ENC_CAPA_CIPHER_TKIP`](#iw-enc-capa-cipher-tkip)
  - [`IW_ENC_CAPA_CIPHER_CCMP`](#iw-enc-capa-cipher-ccmp)
  - [`IW_ENC_CAPA_4WAY_HANDSHAKE`](#iw-enc-capa-4way-handshake)
  - [`IW_EVENT_CAPA_K_0`](#iw-event-capa-k-0)
  - [`IW_EVENT_CAPA_K_1`](#iw-event-capa-k-1)
  - [`IW_PMKSA_ADD`](#iw-pmksa-add)
  - [`IW_PMKSA_REMOVE`](#iw-pmksa-remove)
  - [`IW_PMKSA_FLUSH`](#iw-pmksa-flush)
  - [`IW_PMKID_LEN`](#iw-pmkid-len)
  - [`IW_PMKID_CAND_PREAUTH`](#iw-pmkid-cand-preauth)
  - [`IW_EV_LCP_PK_LEN`](#iw-ev-lcp-pk-len)
  - [`IW_EV_CHAR_PK_LEN`](#iw-ev-char-pk-len)
  - [`IW_EV_UINT_PK_LEN`](#iw-ev-uint-pk-len)
  - [`IW_EV_FREQ_PK_LEN`](#iw-ev-freq-pk-len)
  - [`IW_EV_PARAM_PK_LEN`](#iw-ev-param-pk-len)
  - [`IW_EV_ADDR_PK_LEN`](#iw-ev-addr-pk-len)
  - [`IW_EV_QUAL_PK_LEN`](#iw-ev-qual-pk-len)
  - [`IW_EV_POINT_PK_LEN`](#iw-ev-point-pk-len)
  - [`NUD_NONE`](#nud-none)
  - [`NUD_INCOMPLETE`](#nud-incomplete)
  - [`NUD_REACHABLE`](#nud-reachable)
  - [`NUD_STALE`](#nud-stale)
  - [`NUD_DELAY`](#nud-delay)
  - [`NUD_PROBE`](#nud-probe)
  - [`NUD_FAILED`](#nud-failed)
  - [`NUD_NOARP`](#nud-noarp)
  - [`NUD_PERMANENT`](#nud-permanent)
  - [`NTF_USE`](#ntf-use)
  - [`NTF_SELF`](#ntf-self)
  - [`NTF_MASTER`](#ntf-master)
  - [`NTF_PROXY`](#ntf-proxy)
  - [`NTF_ROUTER`](#ntf-router)
  - [`NDA_UNSPEC`](#nda-unspec)
  - [`NDA_DST`](#nda-dst)
  - [`NDA_LLADDR`](#nda-lladdr)
  - [`NDA_CACHEINFO`](#nda-cacheinfo)
  - [`NDA_PROBES`](#nda-probes)
  - [`NDA_VLAN`](#nda-vlan)
  - [`NDA_PORT`](#nda-port)
  - [`NDA_VNI`](#nda-vni)
  - [`NDA_IFINDEX`](#nda-ifindex)
  - [`NLM_F_BULK`](#nlm-f-bulk)
  - [`TCA_UNSPEC`](#tca-unspec)
  - [`TCA_KIND`](#tca-kind)
  - [`TCA_OPTIONS`](#tca-options)
  - [`TCA_STATS`](#tca-stats)
  - [`TCA_XSTATS`](#tca-xstats)
  - [`TCA_RATE`](#tca-rate)
  - [`TCA_FCNT`](#tca-fcnt)
  - [`TCA_STATS2`](#tca-stats2)
  - [`TCA_STAB`](#tca-stab)
  - [`RTM_NEWLINK`](#rtm-newlink)
  - [`RTM_DELLINK`](#rtm-dellink)
  - [`RTM_GETLINK`](#rtm-getlink)
  - [`RTM_SETLINK`](#rtm-setlink)
  - [`RTM_NEWADDR`](#rtm-newaddr)
  - [`RTM_DELADDR`](#rtm-deladdr)
  - [`RTM_GETADDR`](#rtm-getaddr)
  - [`RTM_NEWROUTE`](#rtm-newroute)
  - [`RTM_DELROUTE`](#rtm-delroute)
  - [`RTM_GETROUTE`](#rtm-getroute)
  - [`RTM_NEWNEIGH`](#rtm-newneigh)
  - [`RTM_DELNEIGH`](#rtm-delneigh)
  - [`RTM_GETNEIGH`](#rtm-getneigh)
  - [`RTM_NEWRULE`](#rtm-newrule)
  - [`RTM_DELRULE`](#rtm-delrule)
  - [`RTM_GETRULE`](#rtm-getrule)
  - [`RTM_NEWQDISC`](#rtm-newqdisc)
  - [`RTM_DELQDISC`](#rtm-delqdisc)
  - [`RTM_GETQDISC`](#rtm-getqdisc)
  - [`RTM_NEWTCLASS`](#rtm-newtclass)
  - [`RTM_DELTCLASS`](#rtm-deltclass)
  - [`RTM_GETTCLASS`](#rtm-gettclass)
  - [`RTM_NEWTFILTER`](#rtm-newtfilter)
  - [`RTM_DELTFILTER`](#rtm-deltfilter)
  - [`RTM_GETTFILTER`](#rtm-gettfilter)
  - [`RTM_NEWACTION`](#rtm-newaction)
  - [`RTM_DELACTION`](#rtm-delaction)
  - [`RTM_GETACTION`](#rtm-getaction)
  - [`RTM_NEWPREFIX`](#rtm-newprefix)
  - [`RTM_GETMULTICAST`](#rtm-getmulticast)
  - [`RTM_GETANYCAST`](#rtm-getanycast)
  - [`RTM_NEWNEIGHTBL`](#rtm-newneightbl)
  - [`RTM_GETNEIGHTBL`](#rtm-getneightbl)
  - [`RTM_SETNEIGHTBL`](#rtm-setneightbl)
  - [`RTM_NEWNDUSEROPT`](#rtm-newnduseropt)
  - [`RTM_NEWADDRLABEL`](#rtm-newaddrlabel)
  - [`RTM_DELADDRLABEL`](#rtm-deladdrlabel)
  - [`RTM_GETADDRLABEL`](#rtm-getaddrlabel)
  - [`RTM_GETDCB`](#rtm-getdcb)
  - [`RTM_SETDCB`](#rtm-setdcb)
  - [`RTM_NEWNETCONF`](#rtm-newnetconf)
  - [`RTM_GETNETCONF`](#rtm-getnetconf)
  - [`RTM_NEWMDB`](#rtm-newmdb)
  - [`RTM_DELMDB`](#rtm-delmdb)
  - [`RTM_GETMDB`](#rtm-getmdb)
  - [`RTM_NEWNSID`](#rtm-newnsid)
  - [`RTM_DELNSID`](#rtm-delnsid)
  - [`RTM_GETNSID`](#rtm-getnsid)
  - [`RTM_F_NOTIFY`](#rtm-f-notify)
  - [`RTM_F_CLONED`](#rtm-f-cloned)
  - [`RTM_F_EQUALIZE`](#rtm-f-equalize)
  - [`RTM_F_PREFIX`](#rtm-f-prefix)
  - [`RTA_UNSPEC`](#rta-unspec)
  - [`RTA_DST`](#rta-dst)
  - [`RTA_SRC`](#rta-src)
  - [`RTA_IIF`](#rta-iif)
  - [`RTA_OIF`](#rta-oif)
  - [`RTA_GATEWAY`](#rta-gateway)
  - [`RTA_PRIORITY`](#rta-priority)
  - [`RTA_PREFSRC`](#rta-prefsrc)
  - [`RTA_METRICS`](#rta-metrics)
  - [`RTA_MULTIPATH`](#rta-multipath)
  - [`RTA_PROTOINFO`](#rta-protoinfo)
  - [`RTA_FLOW`](#rta-flow)
  - [`RTA_CACHEINFO`](#rta-cacheinfo)
  - [`RTA_SESSION`](#rta-session)
  - [`RTA_MP_ALGO`](#rta-mp-algo)
  - [`RTA_TABLE`](#rta-table)
  - [`RTA_MARK`](#rta-mark)
  - [`RTA_MFC_STATS`](#rta-mfc-stats)
  - [`RTN_UNSPEC`](#rtn-unspec)
  - [`RTN_UNICAST`](#rtn-unicast)
  - [`RTN_LOCAL`](#rtn-local)
  - [`RTN_BROADCAST`](#rtn-broadcast)
  - [`RTN_ANYCAST`](#rtn-anycast)
  - [`RTN_MULTICAST`](#rtn-multicast)
  - [`RTN_BLACKHOLE`](#rtn-blackhole)
  - [`RTN_UNREACHABLE`](#rtn-unreachable)
  - [`RTN_PROHIBIT`](#rtn-prohibit)
  - [`RTN_THROW`](#rtn-throw)
  - [`RTN_NAT`](#rtn-nat)
  - [`RTN_XRESOLVE`](#rtn-xresolve)
  - [`RTPROT_UNSPEC`](#rtprot-unspec)
  - [`RTPROT_REDIRECT`](#rtprot-redirect)
  - [`RTPROT_KERNEL`](#rtprot-kernel)
  - [`RTPROT_BOOT`](#rtprot-boot)
  - [`RTPROT_STATIC`](#rtprot-static)
  - [`RT_SCOPE_UNIVERSE`](#rt-scope-universe)
  - [`RT_SCOPE_SITE`](#rt-scope-site)
  - [`RT_SCOPE_LINK`](#rt-scope-link)
  - [`RT_SCOPE_HOST`](#rt-scope-host)
  - [`RT_SCOPE_NOWHERE`](#rt-scope-nowhere)
  - [`RT_TABLE_UNSPEC`](#rt-table-unspec)
  - [`RT_TABLE_COMPAT`](#rt-table-compat)
  - [`RT_TABLE_DEFAULT`](#rt-table-default)
  - [`RT_TABLE_MAIN`](#rt-table-main)
  - [`RT_TABLE_LOCAL`](#rt-table-local)
  - [`RTMSG_OVERRUN`](#rtmsg-overrun)
  - [`RTMSG_NEWDEVICE`](#rtmsg-newdevice)
  - [`RTMSG_DELDEVICE`](#rtmsg-deldevice)
  - [`RTMSG_NEWROUTE`](#rtmsg-newroute)
  - [`RTMSG_DELROUTE`](#rtmsg-delroute)
  - [`RTMSG_NEWRULE`](#rtmsg-newrule)
  - [`RTMSG_DELRULE`](#rtmsg-delrule)
  - [`RTMSG_CONTROL`](#rtmsg-control)
  - [`RTMSG_AR_FAILED`](#rtmsg-ar-failed)
  - [`RTEXT_FILTER_VF`](#rtext-filter-vf)
  - [`RTEXT_FILTER_BRVLAN`](#rtext-filter-brvlan)
  - [`RTEXT_FILTER_BRVLAN_COMPRESSED`](#rtext-filter-brvlan-compressed)
  - [`RTEXT_FILTER_SKIP_STATS`](#rtext-filter-skip-stats)
  - [`RTEXT_FILTER_MRP`](#rtext-filter-mrp)
  - [`RTEXT_FILTER_CFM_CONFIG`](#rtext-filter-cfm-config)
  - [`RTEXT_FILTER_CFM_STATUS`](#rtext-filter-cfm-status)
  - [`RTMGRP_LINK`](#rtmgrp-link)
  - [`RTMGRP_NOTIFY`](#rtmgrp-notify)
  - [`RTMGRP_NEIGH`](#rtmgrp-neigh)
  - [`RTMGRP_TC`](#rtmgrp-tc)
  - [`RTMGRP_IPV4_IFADDR`](#rtmgrp-ipv4-ifaddr)
  - [`RTMGRP_IPV4_MROUTE`](#rtmgrp-ipv4-mroute)
  - [`RTMGRP_IPV4_ROUTE`](#rtmgrp-ipv4-route)
  - [`RTMGRP_IPV4_RULE`](#rtmgrp-ipv4-rule)
  - [`RTMGRP_IPV6_IFADDR`](#rtmgrp-ipv6-ifaddr)
  - [`RTMGRP_IPV6_MROUTE`](#rtmgrp-ipv6-mroute)
  - [`RTMGRP_IPV6_ROUTE`](#rtmgrp-ipv6-route)
  - [`RTMGRP_IPV6_IFINFO`](#rtmgrp-ipv6-ifinfo)
  - [`RTMGRP_DECnet_IFADDR`](#rtmgrp-decnet-ifaddr)
  - [`RTMGRP_DECnet_ROUTE`](#rtmgrp-decnet-route)
  - [`RTMGRP_IPV6_PREFIX`](#rtmgrp-ipv6-prefix)
  - [`RTNLGRP_NONE`](#rtnlgrp-none)
  - [`RTNLGRP_LINK`](#rtnlgrp-link)
  - [`RTNLGRP_NOTIFY`](#rtnlgrp-notify)
  - [`RTNLGRP_NEIGH`](#rtnlgrp-neigh)
  - [`RTNLGRP_TC`](#rtnlgrp-tc)
  - [`RTNLGRP_IPV4_IFADDR`](#rtnlgrp-ipv4-ifaddr)
  - [`RTNLGRP_IPV4_MROUTE`](#rtnlgrp-ipv4-mroute)
  - [`RTNLGRP_IPV4_ROUTE`](#rtnlgrp-ipv4-route)
  - [`RTNLGRP_IPV4_RULE`](#rtnlgrp-ipv4-rule)
  - [`RTNLGRP_IPV6_IFADDR`](#rtnlgrp-ipv6-ifaddr)
  - [`RTNLGRP_IPV6_MROUTE`](#rtnlgrp-ipv6-mroute)
  - [`RTNLGRP_IPV6_ROUTE`](#rtnlgrp-ipv6-route)
  - [`RTNLGRP_IPV6_IFINFO`](#rtnlgrp-ipv6-ifinfo)
  - [`RTNLGRP_DECnet_IFADDR`](#rtnlgrp-decnet-ifaddr)
  - [`RTNLGRP_NOP2`](#rtnlgrp-nop2)
  - [`RTNLGRP_DECnet_ROUTE`](#rtnlgrp-decnet-route)
  - [`RTNLGRP_DECnet_RULE`](#rtnlgrp-decnet-rule)
  - [`RTNLGRP_NOP4`](#rtnlgrp-nop4)
  - [`RTNLGRP_IPV6_PREFIX`](#rtnlgrp-ipv6-prefix)
  - [`RTNLGRP_IPV6_RULE`](#rtnlgrp-ipv6-rule)
  - [`RTNLGRP_ND_USEROPT`](#rtnlgrp-nd-useropt)
  - [`RTNLGRP_PHONET_IFADDR`](#rtnlgrp-phonet-ifaddr)
  - [`RTNLGRP_PHONET_ROUTE`](#rtnlgrp-phonet-route)
  - [`RTNLGRP_DCB`](#rtnlgrp-dcb)
  - [`RTNLGRP_IPV4_NETCONF`](#rtnlgrp-ipv4-netconf)
  - [`RTNLGRP_IPV6_NETCONF`](#rtnlgrp-ipv6-netconf)
  - [`RTNLGRP_MDB`](#rtnlgrp-mdb)
  - [`RTNLGRP_MPLS_ROUTE`](#rtnlgrp-mpls-route)
  - [`RTNLGRP_NSID`](#rtnlgrp-nsid)
  - [`RTNLGRP_MPLS_NETCONF`](#rtnlgrp-mpls-netconf)
  - [`RTNLGRP_IPV4_MROUTE_R`](#rtnlgrp-ipv4-mroute-r)
  - [`RTNLGRP_IPV6_MROUTE_R`](#rtnlgrp-ipv6-mroute-r)
  - [`RTNLGRP_NEXTHOP`](#rtnlgrp-nexthop)
  - [`RTNLGRP_BRVLAN`](#rtnlgrp-brvlan)
  - [`RTNLGRP_MCTP_IFADDR`](#rtnlgrp-mctp-ifaddr)
  - [`RTNLGRP_TUNNEL`](#rtnlgrp-tunnel)
  - [`RTNLGRP_STATS`](#rtnlgrp-stats)
  - [`PROC_CN_MCAST_LISTEN`](#proc-cn-mcast-listen)
  - [`PROC_CN_MCAST_IGNORE`](#proc-cn-mcast-ignore)
  - [`PROC_EVENT_NONE`](#proc-event-none)
  - [`PROC_EVENT_FORK`](#proc-event-fork)
  - [`PROC_EVENT_EXEC`](#proc-event-exec)
  - [`PROC_EVENT_UID`](#proc-event-uid)
  - [`PROC_EVENT_GID`](#proc-event-gid)
  - [`PROC_EVENT_SID`](#proc-event-sid)
  - [`PROC_EVENT_PTRACE`](#proc-event-ptrace)
  - [`PROC_EVENT_COMM`](#proc-event-comm)
  - [`PROC_EVENT_NONZERO_EXIT`](#proc-event-nonzero-exit)
  - [`PROC_EVENT_COREDUMP`](#proc-event-coredump)
  - [`PROC_EVENT_EXIT`](#proc-event-exit)
  - [`CN_IDX_PROC`](#cn-idx-proc)
  - [`CN_VAL_PROC`](#cn-val-proc)
  - [`CN_IDX_CIFS`](#cn-idx-cifs)
  - [`CN_VAL_CIFS`](#cn-val-cifs)
  - [`CN_W1_IDX`](#cn-w1-idx)
  - [`CN_W1_VAL`](#cn-w1-val)
  - [`CN_IDX_V86D`](#cn-idx-v86d)
  - [`CN_VAL_V86D_UVESAFB`](#cn-val-v86d-uvesafb)
  - [`CN_IDX_BB`](#cn-idx-bb)
  - [`CN_DST_IDX`](#cn-dst-idx)
  - [`CN_DST_VAL`](#cn-dst-val)
  - [`CN_IDX_DM`](#cn-idx-dm)
  - [`CN_VAL_DM_USERSPACE_LOG`](#cn-val-dm-userspace-log)
  - [`CN_IDX_DRBD`](#cn-idx-drbd)
  - [`CN_VAL_DRBD`](#cn-val-drbd)
  - [`CN_KVP_IDX`](#cn-kvp-idx)
  - [`CN_KVP_VAL`](#cn-kvp-val)
  - [`CN_VSS_IDX`](#cn-vss-idx)
  - [`CN_VSS_VAL`](#cn-vss-val)
  - [`MODULE_INIT_IGNORE_MODVERSIONS`](#module-init-ignore-modversions)
  - [`MODULE_INIT_IGNORE_VERMAGIC`](#module-init-ignore-vermagic)
  - [`SOF_TIMESTAMPING_TX_HARDWARE`](#sof-timestamping-tx-hardware)
  - [`SOF_TIMESTAMPING_TX_SOFTWARE`](#sof-timestamping-tx-software)
  - [`SOF_TIMESTAMPING_RX_HARDWARE`](#sof-timestamping-rx-hardware)
  - [`SOF_TIMESTAMPING_RX_SOFTWARE`](#sof-timestamping-rx-software)
  - [`SOF_TIMESTAMPING_SOFTWARE`](#sof-timestamping-software)
  - [`SOF_TIMESTAMPING_SYS_HARDWARE`](#sof-timestamping-sys-hardware)
  - [`SOF_TIMESTAMPING_RAW_HARDWARE`](#sof-timestamping-raw-hardware)
  - [`SOF_TIMESTAMPING_OPT_ID`](#sof-timestamping-opt-id)
  - [`SOF_TIMESTAMPING_TX_SCHED`](#sof-timestamping-tx-sched)
  - [`SOF_TIMESTAMPING_TX_ACK`](#sof-timestamping-tx-ack)
  - [`SOF_TIMESTAMPING_OPT_CMSG`](#sof-timestamping-opt-cmsg)
  - [`SOF_TIMESTAMPING_OPT_TSONLY`](#sof-timestamping-opt-tsonly)
  - [`SOF_TIMESTAMPING_OPT_STATS`](#sof-timestamping-opt-stats)
  - [`SOF_TIMESTAMPING_OPT_PKTINFO`](#sof-timestamping-opt-pktinfo)
  - [`SOF_TIMESTAMPING_OPT_TX_SWHW`](#sof-timestamping-opt-tx-swhw)
  - [`SOF_TIMESTAMPING_BIND_PHC`](#sof-timestamping-bind-phc)
  - [`SOF_TIMESTAMPING_OPT_ID_TCP`](#sof-timestamping-opt-id-tcp)
  - [`SOF_TIMESTAMPING_OPT_RX_FILTER`](#sof-timestamping-opt-rx-filter)
  - [`SOF_TXTIME_DEADLINE_MODE`](#sof-txtime-deadline-mode)
  - [`SOF_TXTIME_REPORT_ERRORS`](#sof-txtime-report-errors)
  - [`HWTSTAMP_TX_OFF`](#hwtstamp-tx-off)
  - [`HWTSTAMP_TX_ON`](#hwtstamp-tx-on)
  - [`HWTSTAMP_TX_ONESTEP_SYNC`](#hwtstamp-tx-onestep-sync)
  - [`HWTSTAMP_TX_ONESTEP_P2P`](#hwtstamp-tx-onestep-p2p)
  - [`HWTSTAMP_FILTER_NONE`](#hwtstamp-filter-none)
  - [`HWTSTAMP_FILTER_ALL`](#hwtstamp-filter-all)
  - [`HWTSTAMP_FILTER_SOME`](#hwtstamp-filter-some)
  - [`HWTSTAMP_FILTER_PTP_V1_L4_EVENT`](#hwtstamp-filter-ptp-v1-l4-event)
  - [`HWTSTAMP_FILTER_PTP_V1_L4_SYNC`](#hwtstamp-filter-ptp-v1-l4-sync)
  - [`HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ`](#hwtstamp-filter-ptp-v1-l4-delay-req)
  - [`HWTSTAMP_FILTER_PTP_V2_L4_EVENT`](#hwtstamp-filter-ptp-v2-l4-event)
  - [`HWTSTAMP_FILTER_PTP_V2_L4_SYNC`](#hwtstamp-filter-ptp-v2-l4-sync)
  - [`HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ`](#hwtstamp-filter-ptp-v2-l4-delay-req)
  - [`HWTSTAMP_FILTER_PTP_V2_L2_EVENT`](#hwtstamp-filter-ptp-v2-l2-event)
  - [`HWTSTAMP_FILTER_PTP_V2_L2_SYNC`](#hwtstamp-filter-ptp-v2-l2-sync)
  - [`HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ`](#hwtstamp-filter-ptp-v2-l2-delay-req)
  - [`HWTSTAMP_FILTER_PTP_V2_EVENT`](#hwtstamp-filter-ptp-v2-event)
  - [`HWTSTAMP_FILTER_PTP_V2_SYNC`](#hwtstamp-filter-ptp-v2-sync)
  - [`HWTSTAMP_FILTER_PTP_V2_DELAY_REQ`](#hwtstamp-filter-ptp-v2-delay-req)
  - [`HWTSTAMP_FILTER_NTP_ALL`](#hwtstamp-filter-ntp-all)
  - [`PTP_MAX_SAMPLES`](#ptp-max-samples)
  - [`PTP_CLK_MAGIC`](#ptp-clk-magic)
  - [`PTP_CLOCK_GETCAPS`](#ptp-clock-getcaps)
  - [`PTP_EXTTS_REQUEST`](#ptp-extts-request)
  - [`PTP_PEROUT_REQUEST`](#ptp-perout-request)
  - [`PTP_ENABLE_PPS`](#ptp-enable-pps)
  - [`PTP_SYS_OFFSET`](#ptp-sys-offset)
  - [`PTP_PIN_GETFUNC`](#ptp-pin-getfunc)
  - [`PTP_PIN_SETFUNC`](#ptp-pin-setfunc)
  - [`PTP_SYS_OFFSET_PRECISE`](#ptp-sys-offset-precise)
  - [`PTP_SYS_OFFSET_EXTENDED`](#ptp-sys-offset-extended)
  - [`PTP_CLOCK_GETCAPS2`](#ptp-clock-getcaps2)
  - [`PTP_EXTTS_REQUEST2`](#ptp-extts-request2)
  - [`PTP_PEROUT_REQUEST2`](#ptp-perout-request2)
  - [`PTP_ENABLE_PPS2`](#ptp-enable-pps2)
  - [`PTP_SYS_OFFSET2`](#ptp-sys-offset2)
  - [`PTP_PIN_GETFUNC2`](#ptp-pin-getfunc2)
  - [`PTP_PIN_SETFUNC2`](#ptp-pin-setfunc2)
  - [`PTP_SYS_OFFSET_PRECISE2`](#ptp-sys-offset-precise2)
  - [`PTP_SYS_OFFSET_EXTENDED2`](#ptp-sys-offset-extended2)
  - [`PTP_PF_NONE`](#ptp-pf-none)
  - [`PTP_PF_EXTTS`](#ptp-pf-extts)
  - [`PTP_PF_PEROUT`](#ptp-pf-perout)
  - [`PTP_PF_PHYSYNC`](#ptp-pf-physync)
  - [`TLS_TX`](#tls-tx)
  - [`TLS_RX`](#tls-rx)
  - [`TLS_TX_ZEROCOPY_RO`](#tls-tx-zerocopy-ro)
  - [`TLS_RX_EXPECT_NO_PAD`](#tls-rx-expect-no-pad)
  - [`TLS_1_2_VERSION_MAJOR`](#tls-1-2-version-major)
  - [`TLS_1_2_VERSION_MINOR`](#tls-1-2-version-minor)
  - [`TLS_1_2_VERSION`](#tls-1-2-version)
  - [`TLS_1_3_VERSION_MAJOR`](#tls-1-3-version-major)
  - [`TLS_1_3_VERSION_MINOR`](#tls-1-3-version-minor)
  - [`TLS_1_3_VERSION`](#tls-1-3-version)
  - [`TLS_CIPHER_AES_GCM_128`](#tls-cipher-aes-gcm-128)
  - [`TLS_CIPHER_AES_GCM_128_IV_SIZE`](#tls-cipher-aes-gcm-128-iv-size)
  - [`TLS_CIPHER_AES_GCM_128_KEY_SIZE`](#tls-cipher-aes-gcm-128-key-size)
  - [`TLS_CIPHER_AES_GCM_128_SALT_SIZE`](#tls-cipher-aes-gcm-128-salt-size)
  - [`TLS_CIPHER_AES_GCM_128_TAG_SIZE`](#tls-cipher-aes-gcm-128-tag-size)
  - [`TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE`](#tls-cipher-aes-gcm-128-rec-seq-size)
  - [`TLS_CIPHER_AES_GCM_256`](#tls-cipher-aes-gcm-256)
  - [`TLS_CIPHER_AES_GCM_256_IV_SIZE`](#tls-cipher-aes-gcm-256-iv-size)
  - [`TLS_CIPHER_AES_GCM_256_KEY_SIZE`](#tls-cipher-aes-gcm-256-key-size)
  - [`TLS_CIPHER_AES_GCM_256_SALT_SIZE`](#tls-cipher-aes-gcm-256-salt-size)
  - [`TLS_CIPHER_AES_GCM_256_TAG_SIZE`](#tls-cipher-aes-gcm-256-tag-size)
  - [`TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE`](#tls-cipher-aes-gcm-256-rec-seq-size)
  - [`TLS_CIPHER_AES_CCM_128`](#tls-cipher-aes-ccm-128)
  - [`TLS_CIPHER_AES_CCM_128_IV_SIZE`](#tls-cipher-aes-ccm-128-iv-size)
  - [`TLS_CIPHER_AES_CCM_128_KEY_SIZE`](#tls-cipher-aes-ccm-128-key-size)
  - [`TLS_CIPHER_AES_CCM_128_SALT_SIZE`](#tls-cipher-aes-ccm-128-salt-size)
  - [`TLS_CIPHER_AES_CCM_128_TAG_SIZE`](#tls-cipher-aes-ccm-128-tag-size)
  - [`TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE`](#tls-cipher-aes-ccm-128-rec-seq-size)
  - [`TLS_CIPHER_CHACHA20_POLY1305`](#tls-cipher-chacha20-poly1305)
  - [`TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE`](#tls-cipher-chacha20-poly1305-iv-size)
  - [`TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE`](#tls-cipher-chacha20-poly1305-key-size)
  - [`TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE`](#tls-cipher-chacha20-poly1305-salt-size)
  - [`TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE`](#tls-cipher-chacha20-poly1305-tag-size)
  - [`TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE`](#tls-cipher-chacha20-poly1305-rec-seq-size)
  - [`TLS_CIPHER_SM4_GCM`](#tls-cipher-sm4-gcm)
  - [`TLS_CIPHER_SM4_GCM_IV_SIZE`](#tls-cipher-sm4-gcm-iv-size)
  - [`TLS_CIPHER_SM4_GCM_KEY_SIZE`](#tls-cipher-sm4-gcm-key-size)
  - [`TLS_CIPHER_SM4_GCM_SALT_SIZE`](#tls-cipher-sm4-gcm-salt-size)
  - [`TLS_CIPHER_SM4_GCM_TAG_SIZE`](#tls-cipher-sm4-gcm-tag-size)
  - [`TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE`](#tls-cipher-sm4-gcm-rec-seq-size)
  - [`TLS_CIPHER_SM4_CCM`](#tls-cipher-sm4-ccm)
  - [`TLS_CIPHER_SM4_CCM_IV_SIZE`](#tls-cipher-sm4-ccm-iv-size)
  - [`TLS_CIPHER_SM4_CCM_KEY_SIZE`](#tls-cipher-sm4-ccm-key-size)
  - [`TLS_CIPHER_SM4_CCM_SALT_SIZE`](#tls-cipher-sm4-ccm-salt-size)
  - [`TLS_CIPHER_SM4_CCM_TAG_SIZE`](#tls-cipher-sm4-ccm-tag-size)
  - [`TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE`](#tls-cipher-sm4-ccm-rec-seq-size)
  - [`TLS_CIPHER_ARIA_GCM_128`](#tls-cipher-aria-gcm-128)
  - [`TLS_CIPHER_ARIA_GCM_128_IV_SIZE`](#tls-cipher-aria-gcm-128-iv-size)
  - [`TLS_CIPHER_ARIA_GCM_128_KEY_SIZE`](#tls-cipher-aria-gcm-128-key-size)
  - [`TLS_CIPHER_ARIA_GCM_128_SALT_SIZE`](#tls-cipher-aria-gcm-128-salt-size)
  - [`TLS_CIPHER_ARIA_GCM_128_TAG_SIZE`](#tls-cipher-aria-gcm-128-tag-size)
  - [`TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE`](#tls-cipher-aria-gcm-128-rec-seq-size)
  - [`TLS_CIPHER_ARIA_GCM_256`](#tls-cipher-aria-gcm-256)
  - [`TLS_CIPHER_ARIA_GCM_256_IV_SIZE`](#tls-cipher-aria-gcm-256-iv-size)
  - [`TLS_CIPHER_ARIA_GCM_256_KEY_SIZE`](#tls-cipher-aria-gcm-256-key-size)
  - [`TLS_CIPHER_ARIA_GCM_256_SALT_SIZE`](#tls-cipher-aria-gcm-256-salt-size)
  - [`TLS_CIPHER_ARIA_GCM_256_TAG_SIZE`](#tls-cipher-aria-gcm-256-tag-size)
  - [`TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE`](#tls-cipher-aria-gcm-256-rec-seq-size)
  - [`TLS_SET_RECORD_TYPE`](#tls-set-record-type)
  - [`TLS_GET_RECORD_TYPE`](#tls-get-record-type)
  - [`SOL_TLS`](#sol-tls)
  - [`TLS_INFO_UNSPEC`](#tls-info-unspec)
  - [`TLS_INFO_VERSION`](#tls-info-version)
  - [`TLS_INFO_CIPHER`](#tls-info-cipher)
  - [`TLS_INFO_TXCONF`](#tls-info-txconf)
  - [`TLS_INFO_RXCONF`](#tls-info-rxconf)
  - [`TLS_INFO_ZC_RO_TX`](#tls-info-zc-ro-tx)
  - [`TLS_INFO_RX_NO_PAD`](#tls-info-rx-no-pad)
  - [`TLS_CONF_BASE`](#tls-conf-base)
  - [`TLS_CONF_SW`](#tls-conf-sw)
  - [`TLS_CONF_HW`](#tls-conf-hw)
  - [`TLS_CONF_HW_RECORD`](#tls-conf-hw-record)
  - [`ALG_SET_KEY`](#alg-set-key)
  - [`ALG_SET_IV`](#alg-set-iv)
  - [`ALG_SET_OP`](#alg-set-op)
  - [`ALG_SET_AEAD_ASSOCLEN`](#alg-set-aead-assoclen)
  - [`ALG_SET_AEAD_AUTHSIZE`](#alg-set-aead-authsize)
  - [`ALG_SET_DRBG_ENTROPY`](#alg-set-drbg-entropy)
  - [`ALG_SET_KEY_BY_KEY_SERIAL`](#alg-set-key-by-key-serial)
  - [`ALG_OP_DECRYPT`](#alg-op-decrypt)
  - [`ALG_OP_ENCRYPT`](#alg-op-encrypt)
  - [`IF_OPER_UNKNOWN`](#if-oper-unknown)
  - [`IF_OPER_NOTPRESENT`](#if-oper-notpresent)
  - [`IF_OPER_DOWN`](#if-oper-down)
  - [`IF_OPER_LOWERLAYERDOWN`](#if-oper-lowerlayerdown)
  - [`IF_OPER_TESTING`](#if-oper-testing)
  - [`IF_OPER_DORMANT`](#if-oper-dormant)
  - [`IF_OPER_UP`](#if-oper-up)
  - [`IF_LINK_MODE_DEFAULT`](#if-link-mode-default)
  - [`IF_LINK_MODE_DORMANT`](#if-link-mode-dormant)
  - [`IF_LINK_MODE_TESTING`](#if-link-mode-testing)
  - [`MAP_SHARED_VALIDATE`](#map-shared-validate)
  - [`MAP_DROPPABLE`](#map-droppable)
  - [`VMADDR_CID_ANY`](#vmaddr-cid-any)
  - [`VMADDR_CID_HYPERVISOR`](#vmaddr-cid-hypervisor)
  - [`VMADDR_CID_RESERVED`](#vmaddr-cid-reserved)
  - [`VMADDR_CID_LOCAL`](#vmaddr-cid-local)
  - [`VMADDR_CID_HOST`](#vmaddr-cid-host)
  - [`VMADDR_PORT_ANY`](#vmaddr-port-any)
  - [`IN_ACCESS`](#in-access)
  - [`IN_MODIFY`](#in-modify)
  - [`IN_ATTRIB`](#in-attrib)
  - [`IN_CLOSE_WRITE`](#in-close-write)
  - [`IN_CLOSE_NOWRITE`](#in-close-nowrite)
  - [`IN_CLOSE`](#in-close)
  - [`IN_OPEN`](#in-open)
  - [`IN_MOVED_FROM`](#in-moved-from)
  - [`IN_MOVED_TO`](#in-moved-to)
  - [`IN_MOVE`](#in-move)
  - [`IN_CREATE`](#in-create)
  - [`IN_DELETE`](#in-delete)
  - [`IN_DELETE_SELF`](#in-delete-self)
  - [`IN_MOVE_SELF`](#in-move-self)
  - [`IN_UNMOUNT`](#in-unmount)
  - [`IN_Q_OVERFLOW`](#in-q-overflow)
  - [`IN_IGNORED`](#in-ignored)
  - [`IN_ONLYDIR`](#in-onlydir)
  - [`IN_DONT_FOLLOW`](#in-dont-follow)
  - [`IN_EXCL_UNLINK`](#in-excl-unlink)
  - [`SECURE_NOROOT`](#secure-noroot)
  - [`SECURE_NOROOT_LOCKED`](#secure-noroot-locked)
  - [`SECBIT_NOROOT`](#secbit-noroot)
  - [`SECBIT_NOROOT_LOCKED`](#secbit-noroot-locked)
  - [`SECURE_NO_SETUID_FIXUP`](#secure-no-setuid-fixup)
  - [`SECURE_NO_SETUID_FIXUP_LOCKED`](#secure-no-setuid-fixup-locked)
  - [`SECBIT_NO_SETUID_FIXUP`](#secbit-no-setuid-fixup)
  - [`SECBIT_NO_SETUID_FIXUP_LOCKED`](#secbit-no-setuid-fixup-locked)
  - [`SECURE_KEEP_CAPS`](#secure-keep-caps)
  - [`SECURE_KEEP_CAPS_LOCKED`](#secure-keep-caps-locked)
  - [`SECBIT_KEEP_CAPS`](#secbit-keep-caps)
  - [`SECBIT_KEEP_CAPS_LOCKED`](#secbit-keep-caps-locked)
  - [`SECURE_NO_CAP_AMBIENT_RAISE`](#secure-no-cap-ambient-raise)
  - [`SECURE_NO_CAP_AMBIENT_RAISE_LOCKED`](#secure-no-cap-ambient-raise-locked)
  - [`SECBIT_NO_CAP_AMBIENT_RAISE`](#secbit-no-cap-ambient-raise)
  - [`SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED`](#secbit-no-cap-ambient-raise-locked)
  - [`SECURE_EXEC_RESTRICT_FILE`](#secure-exec-restrict-file)
  - [`SECURE_EXEC_RESTRICT_FILE_LOCKED`](#secure-exec-restrict-file-locked)
  - [`SECBIT_EXEC_RESTRICT_FILE`](#secbit-exec-restrict-file)
  - [`SECBIT_EXEC_RESTRICT_FILE_LOCKED`](#secbit-exec-restrict-file-locked)
  - [`SECURE_EXEC_DENY_INTERACTIVE`](#secure-exec-deny-interactive)
  - [`SECURE_EXEC_DENY_INTERACTIVE_LOCKED`](#secure-exec-deny-interactive-locked)
  - [`SECBIT_EXEC_DENY_INTERACTIVE`](#secbit-exec-deny-interactive)
  - [`SECBIT_EXEC_DENY_INTERACTIVE_LOCKED`](#secbit-exec-deny-interactive-locked)
  - [`SECUREBITS_DEFAULT`](#securebits-default)
  - [`SECURE_ALL_BITS`](#secure-all-bits)
  - [`SECURE_ALL_LOCKS`](#secure-all-locks)
  - [`SECURE_ALL_UNPRIVILEGED`](#secure-all-unprivileged)
  - [`IN_MASK_CREATE`](#in-mask-create)
  - [`IN_MASK_ADD`](#in-mask-add)
  - [`IN_ISDIR`](#in-isdir)
  - [`IN_ONESHOT`](#in-oneshot)
  - [`IN_ALL_EVENTS`](#in-all-events)
  - [`IN_CLOEXEC`](#in-cloexec)
  - [`IN_NONBLOCK`](#in-nonblock)
  - [`OPEN_TREE_CLONE`](#open-tree-clone)
  - [`OPEN_TREE_CLOEXEC`](#open-tree-cloexec)
  - [`NFT_TABLE_MAXNAMELEN`](#nft-table-maxnamelen)
  - [`NFT_CHAIN_MAXNAMELEN`](#nft-chain-maxnamelen)
  - [`NFT_SET_MAXNAMELEN`](#nft-set-maxnamelen)
  - [`NFT_OBJ_MAXNAMELEN`](#nft-obj-maxnamelen)
  - [`NFT_USERDATA_MAXLEN`](#nft-userdata-maxlen)
  - [`NFT_REG_VERDICT`](#nft-reg-verdict)
  - [`NFT_REG_1`](#nft-reg-1)
  - [`NFT_REG_2`](#nft-reg-2)
  - [`NFT_REG_3`](#nft-reg-3)
  - [`NFT_REG_4`](#nft-reg-4)
  - [`__NFT_REG_MAX`](#nft-reg-max)
  - [`NFT_REG32_00`](#nft-reg32-00)
  - [`NFT_REG32_01`](#nft-reg32-01)
  - [`NFT_REG32_02`](#nft-reg32-02)
  - [`NFT_REG32_03`](#nft-reg32-03)
  - [`NFT_REG32_04`](#nft-reg32-04)
  - [`NFT_REG32_05`](#nft-reg32-05)
  - [`NFT_REG32_06`](#nft-reg32-06)
  - [`NFT_REG32_07`](#nft-reg32-07)
  - [`NFT_REG32_08`](#nft-reg32-08)
  - [`NFT_REG32_09`](#nft-reg32-09)
  - [`NFT_REG32_10`](#nft-reg32-10)
  - [`NFT_REG32_11`](#nft-reg32-11)
  - [`NFT_REG32_12`](#nft-reg32-12)
  - [`NFT_REG32_13`](#nft-reg32-13)
  - [`NFT_REG32_14`](#nft-reg32-14)
  - [`NFT_REG32_15`](#nft-reg32-15)
  - [`NFT_REG_SIZE`](#nft-reg-size)
  - [`NFT_REG32_SIZE`](#nft-reg32-size)
  - [`NFT_CONTINUE`](#nft-continue)
  - [`NFT_BREAK`](#nft-break)
  - [`NFT_JUMP`](#nft-jump)
  - [`NFT_GOTO`](#nft-goto)
  - [`NFT_RETURN`](#nft-return)
  - [`NFT_MSG_NEWTABLE`](#nft-msg-newtable)
  - [`NFT_MSG_GETTABLE`](#nft-msg-gettable)
  - [`NFT_MSG_DELTABLE`](#nft-msg-deltable)
  - [`NFT_MSG_NEWCHAIN`](#nft-msg-newchain)
  - [`NFT_MSG_GETCHAIN`](#nft-msg-getchain)
  - [`NFT_MSG_DELCHAIN`](#nft-msg-delchain)
  - [`NFT_MSG_NEWRULE`](#nft-msg-newrule)
  - [`NFT_MSG_GETRULE`](#nft-msg-getrule)
  - [`NFT_MSG_DELRULE`](#nft-msg-delrule)
  - [`NFT_MSG_NEWSET`](#nft-msg-newset)
  - [`NFT_MSG_GETSET`](#nft-msg-getset)
  - [`NFT_MSG_DELSET`](#nft-msg-delset)
  - [`NFT_MSG_NEWSETELEM`](#nft-msg-newsetelem)
  - [`NFT_MSG_GETSETELEM`](#nft-msg-getsetelem)
  - [`NFT_MSG_DELSETELEM`](#nft-msg-delsetelem)
  - [`NFT_MSG_NEWGEN`](#nft-msg-newgen)
  - [`NFT_MSG_GETGEN`](#nft-msg-getgen)
  - [`NFT_MSG_TRACE`](#nft-msg-trace)
  - [`NFT_MSG_NEWOBJ`](#nft-msg-newobj)
  - [`NFT_MSG_GETOBJ`](#nft-msg-getobj)
  - [`NFT_MSG_DELOBJ`](#nft-msg-delobj)
  - [`NFT_MSG_GETOBJ_RESET`](#nft-msg-getobj-reset)
  - [`NFT_MSG_MAX`](#nft-msg-max)
  - [`NFT_SET_ANONYMOUS`](#nft-set-anonymous)
  - [`NFT_SET_CONSTANT`](#nft-set-constant)
  - [`NFT_SET_INTERVAL`](#nft-set-interval)
  - [`NFT_SET_MAP`](#nft-set-map)
  - [`NFT_SET_TIMEOUT`](#nft-set-timeout)
  - [`NFT_SET_EVAL`](#nft-set-eval)
  - [`NFT_SET_POL_PERFORMANCE`](#nft-set-pol-performance)
  - [`NFT_SET_POL_MEMORY`](#nft-set-pol-memory)
  - [`NFT_SET_ELEM_INTERVAL_END`](#nft-set-elem-interval-end)
  - [`NFT_DATA_VALUE`](#nft-data-value)
  - [`NFT_DATA_VERDICT`](#nft-data-verdict)
  - [`NFT_DATA_RESERVED_MASK`](#nft-data-reserved-mask)
  - [`NFT_DATA_VALUE_MAXLEN`](#nft-data-value-maxlen)
  - [`NFT_BYTEORDER_NTOH`](#nft-byteorder-ntoh)
  - [`NFT_BYTEORDER_HTON`](#nft-byteorder-hton)
  - [`NFT_CMP_EQ`](#nft-cmp-eq)
  - [`NFT_CMP_NEQ`](#nft-cmp-neq)
  - [`NFT_CMP_LT`](#nft-cmp-lt)
  - [`NFT_CMP_LTE`](#nft-cmp-lte)
  - [`NFT_CMP_GT`](#nft-cmp-gt)
  - [`NFT_CMP_GTE`](#nft-cmp-gte)
  - [`NFT_RANGE_EQ`](#nft-range-eq)
  - [`NFT_RANGE_NEQ`](#nft-range-neq)
  - [`NFT_LOOKUP_F_INV`](#nft-lookup-f-inv)
  - [`NFT_DYNSET_OP_ADD`](#nft-dynset-op-add)
  - [`NFT_DYNSET_OP_UPDATE`](#nft-dynset-op-update)
  - [`NFT_DYNSET_F_INV`](#nft-dynset-f-inv)
  - [`NFT_PAYLOAD_LL_HEADER`](#nft-payload-ll-header)
  - [`NFT_PAYLOAD_NETWORK_HEADER`](#nft-payload-network-header)
  - [`NFT_PAYLOAD_TRANSPORT_HEADER`](#nft-payload-transport-header)
  - [`NFT_PAYLOAD_CSUM_NONE`](#nft-payload-csum-none)
  - [`NFT_PAYLOAD_CSUM_INET`](#nft-payload-csum-inet)
  - [`NFT_META_LEN`](#nft-meta-len)
  - [`NFT_META_PROTOCOL`](#nft-meta-protocol)
  - [`NFT_META_PRIORITY`](#nft-meta-priority)
  - [`NFT_META_MARK`](#nft-meta-mark)
  - [`NFT_META_IIF`](#nft-meta-iif)
  - [`NFT_META_OIF`](#nft-meta-oif)
  - [`NFT_META_IIFNAME`](#nft-meta-iifname)
  - [`NFT_META_OIFNAME`](#nft-meta-oifname)
  - [`NFT_META_IIFTYPE`](#nft-meta-iiftype)
  - [`NFT_META_OIFTYPE`](#nft-meta-oiftype)
  - [`NFT_META_SKUID`](#nft-meta-skuid)
  - [`NFT_META_SKGID`](#nft-meta-skgid)
  - [`NFT_META_NFTRACE`](#nft-meta-nftrace)
  - [`NFT_META_RTCLASSID`](#nft-meta-rtclassid)
  - [`NFT_META_SECMARK`](#nft-meta-secmark)
  - [`NFT_META_NFPROTO`](#nft-meta-nfproto)
  - [`NFT_META_L4PROTO`](#nft-meta-l4proto)
  - [`NFT_META_BRI_IIFNAME`](#nft-meta-bri-iifname)
  - [`NFT_META_BRI_OIFNAME`](#nft-meta-bri-oifname)
  - [`NFT_META_PKTTYPE`](#nft-meta-pkttype)
  - [`NFT_META_CPU`](#nft-meta-cpu)
  - [`NFT_META_IIFGROUP`](#nft-meta-iifgroup)
  - [`NFT_META_OIFGROUP`](#nft-meta-oifgroup)
  - [`NFT_META_CGROUP`](#nft-meta-cgroup)
  - [`NFT_META_PRANDOM`](#nft-meta-prandom)
  - [`NFT_CT_STATE`](#nft-ct-state)
  - [`NFT_CT_DIRECTION`](#nft-ct-direction)
  - [`NFT_CT_STATUS`](#nft-ct-status)
  - [`NFT_CT_MARK`](#nft-ct-mark)
  - [`NFT_CT_SECMARK`](#nft-ct-secmark)
  - [`NFT_CT_EXPIRATION`](#nft-ct-expiration)
  - [`NFT_CT_HELPER`](#nft-ct-helper)
  - [`NFT_CT_L3PROTOCOL`](#nft-ct-l3protocol)
  - [`NFT_CT_SRC`](#nft-ct-src)
  - [`NFT_CT_DST`](#nft-ct-dst)
  - [`NFT_CT_PROTOCOL`](#nft-ct-protocol)
  - [`NFT_CT_PROTO_SRC`](#nft-ct-proto-src)
  - [`NFT_CT_PROTO_DST`](#nft-ct-proto-dst)
  - [`NFT_CT_LABELS`](#nft-ct-labels)
  - [`NFT_CT_PKTS`](#nft-ct-pkts)
  - [`NFT_CT_BYTES`](#nft-ct-bytes)
  - [`NFT_CT_AVGPKT`](#nft-ct-avgpkt)
  - [`NFT_CT_ZONE`](#nft-ct-zone)
  - [`NFT_CT_EVENTMASK`](#nft-ct-eventmask)
  - [`NFT_CT_SRC_IP`](#nft-ct-src-ip)
  - [`NFT_CT_DST_IP`](#nft-ct-dst-ip)
  - [`NFT_CT_SRC_IP6`](#nft-ct-src-ip6)
  - [`NFT_CT_DST_IP6`](#nft-ct-dst-ip6)
  - [`NFT_LIMIT_PKTS`](#nft-limit-pkts)
  - [`NFT_LIMIT_PKT_BYTES`](#nft-limit-pkt-bytes)
  - [`NFT_LIMIT_F_INV`](#nft-limit-f-inv)
  - [`NFT_QUEUE_FLAG_BYPASS`](#nft-queue-flag-bypass)
  - [`NFT_QUEUE_FLAG_CPU_FANOUT`](#nft-queue-flag-cpu-fanout)
  - [`NFT_QUEUE_FLAG_MASK`](#nft-queue-flag-mask)
  - [`NFT_QUOTA_F_INV`](#nft-quota-f-inv)
  - [`NFT_REJECT_ICMP_UNREACH`](#nft-reject-icmp-unreach)
  - [`NFT_REJECT_TCP_RST`](#nft-reject-tcp-rst)
  - [`NFT_REJECT_ICMPX_UNREACH`](#nft-reject-icmpx-unreach)
  - [`NFT_REJECT_ICMPX_NO_ROUTE`](#nft-reject-icmpx-no-route)
  - [`NFT_REJECT_ICMPX_PORT_UNREACH`](#nft-reject-icmpx-port-unreach)
  - [`NFT_REJECT_ICMPX_HOST_UNREACH`](#nft-reject-icmpx-host-unreach)
  - [`NFT_REJECT_ICMPX_ADMIN_PROHIBITED`](#nft-reject-icmpx-admin-prohibited)
  - [`NFT_NAT_SNAT`](#nft-nat-snat)
  - [`NFT_NAT_DNAT`](#nft-nat-dnat)
  - [`NFT_TRACETYPE_UNSPEC`](#nft-tracetype-unspec)
  - [`NFT_TRACETYPE_POLICY`](#nft-tracetype-policy)
  - [`NFT_TRACETYPE_RETURN`](#nft-tracetype-return)
  - [`NFT_TRACETYPE_RULE`](#nft-tracetype-rule)
  - [`NFT_NG_INCREMENTAL`](#nft-ng-incremental)
  - [`NFT_NG_RANDOM`](#nft-ng-random)
  - [`FF_MAX`](#ff-max)
  - [`FF_CNT`](#ff-cnt)
  - [`INPUT_PROP_POINTER`](#input-prop-pointer)
  - [`INPUT_PROP_DIRECT`](#input-prop-direct)
  - [`INPUT_PROP_BUTTONPAD`](#input-prop-buttonpad)
  - [`INPUT_PROP_SEMI_MT`](#input-prop-semi-mt)
  - [`INPUT_PROP_TOPBUTTONPAD`](#input-prop-topbuttonpad)
  - [`INPUT_PROP_POINTING_STICK`](#input-prop-pointing-stick)
  - [`INPUT_PROP_ACCELEROMETER`](#input-prop-accelerometer)
  - [`INPUT_PROP_MAX`](#input-prop-max)
  - [`INPUT_PROP_CNT`](#input-prop-cnt)
  - [`EV_MAX`](#ev-max)
  - [`EV_CNT`](#ev-cnt)
  - [`SYN_MAX`](#syn-max)
  - [`SYN_CNT`](#syn-cnt)
  - [`KEY_MAX`](#key-max)
  - [`KEY_CNT`](#key-cnt)
  - [`REL_MAX`](#rel-max)
  - [`REL_CNT`](#rel-cnt)
  - [`ABS_MAX`](#abs-max)
  - [`ABS_CNT`](#abs-cnt)
  - [`SW_MAX`](#sw-max)
  - [`SW_CNT`](#sw-cnt)
  - [`MSC_MAX`](#msc-max)
  - [`MSC_CNT`](#msc-cnt)
  - [`LED_MAX`](#led-max)
  - [`LED_CNT`](#led-cnt)
  - [`REP_MAX`](#rep-max)
  - [`REP_CNT`](#rep-cnt)
  - [`SND_MAX`](#snd-max)
  - [`SND_CNT`](#snd-cnt)
  - [`UINPUT_VERSION`](#uinput-version)
  - [`UINPUT_MAX_NAME_SIZE`](#uinput-max-name-size)
  - [`FAN_ACCESS`](#fan-access)
  - [`FAN_MODIFY`](#fan-modify)
  - [`FAN_ATTRIB`](#fan-attrib)
  - [`FAN_CLOSE_WRITE`](#fan-close-write)
  - [`FAN_CLOSE_NOWRITE`](#fan-close-nowrite)
  - [`FAN_OPEN`](#fan-open)
  - [`FAN_MOVED_FROM`](#fan-moved-from)
  - [`FAN_MOVED_TO`](#fan-moved-to)
  - [`FAN_CREATE`](#fan-create)
  - [`FAN_DELETE`](#fan-delete)
  - [`FAN_DELETE_SELF`](#fan-delete-self)
  - [`FAN_MOVE_SELF`](#fan-move-self)
  - [`FAN_OPEN_EXEC`](#fan-open-exec)
  - [`FAN_Q_OVERFLOW`](#fan-q-overflow)
  - [`FAN_FS_ERROR`](#fan-fs-error)
  - [`FAN_OPEN_PERM`](#fan-open-perm)
  - [`FAN_ACCESS_PERM`](#fan-access-perm)
  - [`FAN_OPEN_EXEC_PERM`](#fan-open-exec-perm)
  - [`FAN_EVENT_ON_CHILD`](#fan-event-on-child)
  - [`FAN_RENAME`](#fan-rename)
  - [`FAN_ONDIR`](#fan-ondir)
  - [`FAN_CLOSE`](#fan-close)
  - [`FAN_MOVE`](#fan-move)
  - [`FAN_CLOEXEC`](#fan-cloexec)
  - [`FAN_NONBLOCK`](#fan-nonblock)
  - [`FAN_CLASS_NOTIF`](#fan-class-notif)
  - [`FAN_CLASS_CONTENT`](#fan-class-content)
  - [`FAN_CLASS_PRE_CONTENT`](#fan-class-pre-content)
  - [`FAN_UNLIMITED_QUEUE`](#fan-unlimited-queue)
  - [`FAN_UNLIMITED_MARKS`](#fan-unlimited-marks)
  - [`FAN_ENABLE_AUDIT`](#fan-enable-audit)
  - [`FAN_REPORT_PIDFD`](#fan-report-pidfd)
  - [`FAN_REPORT_TID`](#fan-report-tid)
  - [`FAN_REPORT_FID`](#fan-report-fid)
  - [`FAN_REPORT_DIR_FID`](#fan-report-dir-fid)
  - [`FAN_REPORT_NAME`](#fan-report-name)
  - [`FAN_REPORT_TARGET_FID`](#fan-report-target-fid)
  - [`FAN_REPORT_DFID_NAME`](#fan-report-dfid-name)
  - [`FAN_REPORT_DFID_NAME_TARGET`](#fan-report-dfid-name-target)
  - [`FAN_MARK_ADD`](#fan-mark-add)
  - [`FAN_MARK_REMOVE`](#fan-mark-remove)
  - [`FAN_MARK_DONT_FOLLOW`](#fan-mark-dont-follow)
  - [`FAN_MARK_ONLYDIR`](#fan-mark-onlydir)
  - [`FAN_MARK_IGNORED_MASK`](#fan-mark-ignored-mask)
  - [`FAN_MARK_IGNORED_SURV_MODIFY`](#fan-mark-ignored-surv-modify)
  - [`FAN_MARK_FLUSH`](#fan-mark-flush)
  - [`FAN_MARK_EVICTABLE`](#fan-mark-evictable)
  - [`FAN_MARK_IGNORE`](#fan-mark-ignore)
  - [`FAN_MARK_INODE`](#fan-mark-inode)
  - [`FAN_MARK_MOUNT`](#fan-mark-mount)
  - [`FAN_MARK_FILESYSTEM`](#fan-mark-filesystem)
  - [`FAN_MARK_IGNORE_SURV`](#fan-mark-ignore-surv)
  - [`FANOTIFY_METADATA_VERSION`](#fanotify-metadata-version)
  - [`FAN_EVENT_INFO_TYPE_FID`](#fan-event-info-type-fid)
  - [`FAN_EVENT_INFO_TYPE_DFID_NAME`](#fan-event-info-type-dfid-name)
  - [`FAN_EVENT_INFO_TYPE_DFID`](#fan-event-info-type-dfid)
  - [`FAN_EVENT_INFO_TYPE_PIDFD`](#fan-event-info-type-pidfd)
  - [`FAN_EVENT_INFO_TYPE_ERROR`](#fan-event-info-type-error)
  - [`FAN_EVENT_INFO_TYPE_OLD_DFID_NAME`](#fan-event-info-type-old-dfid-name)
  - [`FAN_EVENT_INFO_TYPE_NEW_DFID_NAME`](#fan-event-info-type-new-dfid-name)
  - [`FAN_RESPONSE_INFO_NONE`](#fan-response-info-none)
  - [`FAN_RESPONSE_INFO_AUDIT_RULE`](#fan-response-info-audit-rule)
  - [`FAN_ALLOW`](#fan-allow)
  - [`FAN_DENY`](#fan-deny)
  - [`FAN_AUDIT`](#fan-audit)
  - [`FAN_INFO`](#fan-info)
  - [`FAN_NOFD`](#fan-nofd)
  - [`FAN_NOPIDFD`](#fan-nopidfd)
  - [`FAN_EPIDFD`](#fan-epidfd)
  - [`FUTEX_WAIT`](#futex-wait)
  - [`FUTEX_WAKE`](#futex-wake)
  - [`FUTEX_FD`](#futex-fd)
  - [`FUTEX_REQUEUE`](#futex-requeue)
  - [`FUTEX_CMP_REQUEUE`](#futex-cmp-requeue)
  - [`FUTEX_WAKE_OP`](#futex-wake-op)
  - [`FUTEX_LOCK_PI`](#futex-lock-pi)
  - [`FUTEX_UNLOCK_PI`](#futex-unlock-pi)
  - [`FUTEX_TRYLOCK_PI`](#futex-trylock-pi)
  - [`FUTEX_WAIT_BITSET`](#futex-wait-bitset)
  - [`FUTEX_WAKE_BITSET`](#futex-wake-bitset)
  - [`FUTEX_WAIT_REQUEUE_PI`](#futex-wait-requeue-pi)
  - [`FUTEX_CMP_REQUEUE_PI`](#futex-cmp-requeue-pi)
  - [`FUTEX_LOCK_PI2`](#futex-lock-pi2)
  - [`FUTEX_PRIVATE_FLAG`](#futex-private-flag)
  - [`FUTEX_CLOCK_REALTIME`](#futex-clock-realtime)
  - [`FUTEX_CMD_MASK`](#futex-cmd-mask)
  - [`FUTEX_WAITERS`](#futex-waiters)
  - [`FUTEX_OWNER_DIED`](#futex-owner-died)
  - [`FUTEX_TID_MASK`](#futex-tid-mask)
  - [`FUTEX_BITSET_MATCH_ANY`](#futex-bitset-match-any)
  - [`FUTEX_OP_SET`](#futex-op-set)
  - [`FUTEX_OP_ADD`](#futex-op-add)
  - [`FUTEX_OP_OR`](#futex-op-or)
  - [`FUTEX_OP_ANDN`](#futex-op-andn)
  - [`FUTEX_OP_XOR`](#futex-op-xor)
  - [`FUTEX_OP_OPARG_SHIFT`](#futex-op-oparg-shift)
  - [`FUTEX_OP_CMP_EQ`](#futex-op-cmp-eq)
  - [`FUTEX_OP_CMP_NE`](#futex-op-cmp-ne)
  - [`FUTEX_OP_CMP_LT`](#futex-op-cmp-lt)
  - [`FUTEX_OP_CMP_LE`](#futex-op-cmp-le)
  - [`FUTEX_OP_CMP_GT`](#futex-op-cmp-gt)
  - [`FUTEX_OP_CMP_GE`](#futex-op-cmp-ge)
  - [`KEXEC_ON_CRASH`](#kexec-on-crash)
  - [`KEXEC_PRESERVE_CONTEXT`](#kexec-preserve-context)
  - [`KEXEC_ARCH_MASK`](#kexec-arch-mask)
  - [`KEXEC_FILE_UNLOAD`](#kexec-file-unload)
  - [`KEXEC_FILE_ON_CRASH`](#kexec-file-on-crash)
  - [`KEXEC_FILE_NO_INITRAMFS`](#kexec-file-no-initramfs)
  - [`LINUX_REBOOT_MAGIC1`](#linux-reboot-magic1)
  - [`LINUX_REBOOT_MAGIC2`](#linux-reboot-magic2)
  - [`LINUX_REBOOT_MAGIC2A`](#linux-reboot-magic2a)
  - [`LINUX_REBOOT_MAGIC2B`](#linux-reboot-magic2b)
  - [`LINUX_REBOOT_MAGIC2C`](#linux-reboot-magic2c)
  - [`LINUX_REBOOT_CMD_RESTART`](#linux-reboot-cmd-restart)
  - [`LINUX_REBOOT_CMD_HALT`](#linux-reboot-cmd-halt)
  - [`LINUX_REBOOT_CMD_CAD_ON`](#linux-reboot-cmd-cad-on)
  - [`LINUX_REBOOT_CMD_CAD_OFF`](#linux-reboot-cmd-cad-off)
  - [`LINUX_REBOOT_CMD_POWER_OFF`](#linux-reboot-cmd-power-off)
  - [`LINUX_REBOOT_CMD_RESTART2`](#linux-reboot-cmd-restart2)
  - [`LINUX_REBOOT_CMD_SW_SUSPEND`](#linux-reboot-cmd-sw-suspend)
  - [`LINUX_REBOOT_CMD_KEXEC`](#linux-reboot-cmd-kexec)
  - [`SO_EE_ORIGIN_NONE`](#so-ee-origin-none)
  - [`SO_EE_ORIGIN_LOCAL`](#so-ee-origin-local)
  - [`SO_EE_ORIGIN_ICMP`](#so-ee-origin-icmp)
  - [`SO_EE_ORIGIN_ICMP6`](#so-ee-origin-icmp6)
  - [`SO_EE_ORIGIN_TXSTATUS`](#so-ee-origin-txstatus)
  - [`SO_EE_ORIGIN_TIMESTAMPING`](#so-ee-origin-timestamping)
  - [`SCTP_FUTURE_ASSOC`](#sctp-future-assoc)
  - [`SCTP_CURRENT_ASSOC`](#sctp-current-assoc)
  - [`SCTP_ALL_ASSOC`](#sctp-all-assoc)
  - [`SCTP_RTOINFO`](#sctp-rtoinfo)
  - [`SCTP_ASSOCINFO`](#sctp-associnfo)
  - [`SCTP_INITMSG`](#sctp-initmsg)
  - [`SCTP_NODELAY`](#sctp-nodelay)
  - [`SCTP_AUTOCLOSE`](#sctp-autoclose)
  - [`SCTP_SET_PEER_PRIMARY_ADDR`](#sctp-set-peer-primary-addr)
  - [`SCTP_PRIMARY_ADDR`](#sctp-primary-addr)
  - [`SCTP_ADAPTATION_LAYER`](#sctp-adaptation-layer)
  - [`SCTP_DISABLE_FRAGMENTS`](#sctp-disable-fragments)
  - [`SCTP_PEER_ADDR_PARAMS`](#sctp-peer-addr-params)
  - [`SCTP_DEFAULT_SEND_PARAM`](#sctp-default-send-param)
  - [`SCTP_EVENTS`](#sctp-events)
  - [`SCTP_I_WANT_MAPPED_V4_ADDR`](#sctp-i-want-mapped-v4-addr)
  - [`SCTP_MAXSEG`](#sctp-maxseg)
  - [`SCTP_STATUS`](#sctp-status)
  - [`SCTP_GET_PEER_ADDR_INFO`](#sctp-get-peer-addr-info)
  - [`SCTP_DELAYED_ACK_TIME`](#sctp-delayed-ack-time)
  - [`SCTP_DELAYED_ACK`](#sctp-delayed-ack)
  - [`SCTP_DELAYED_SACK`](#sctp-delayed-sack)
  - [`SCTP_CONTEXT`](#sctp-context)
  - [`SCTP_FRAGMENT_INTERLEAVE`](#sctp-fragment-interleave)
  - [`SCTP_PARTIAL_DELIVERY_POINT`](#sctp-partial-delivery-point)
  - [`SCTP_MAX_BURST`](#sctp-max-burst)
  - [`SCTP_AUTH_CHUNK`](#sctp-auth-chunk)
  - [`SCTP_HMAC_IDENT`](#sctp-hmac-ident)
  - [`SCTP_AUTH_KEY`](#sctp-auth-key)
  - [`SCTP_AUTH_ACTIVE_KEY`](#sctp-auth-active-key)
  - [`SCTP_AUTH_DELETE_KEY`](#sctp-auth-delete-key)
  - [`SCTP_PEER_AUTH_CHUNKS`](#sctp-peer-auth-chunks)
  - [`SCTP_LOCAL_AUTH_CHUNKS`](#sctp-local-auth-chunks)
  - [`SCTP_GET_ASSOC_NUMBER`](#sctp-get-assoc-number)
  - [`SCTP_GET_ASSOC_ID_LIST`](#sctp-get-assoc-id-list)
  - [`SCTP_AUTO_ASCONF`](#sctp-auto-asconf)
  - [`SCTP_PEER_ADDR_THLDS`](#sctp-peer-addr-thlds)
  - [`SCTP_RECVRCVINFO`](#sctp-recvrcvinfo)
  - [`SCTP_RECVNXTINFO`](#sctp-recvnxtinfo)
  - [`SCTP_DEFAULT_SNDINFO`](#sctp-default-sndinfo)
  - [`SCTP_AUTH_DEACTIVATE_KEY`](#sctp-auth-deactivate-key)
  - [`SCTP_REUSE_PORT`](#sctp-reuse-port)
  - [`SCTP_PEER_ADDR_THLDS_V2`](#sctp-peer-addr-thlds-v2)
  - [`SCTP_PR_SCTP_NONE`](#sctp-pr-sctp-none)
  - [`SCTP_PR_SCTP_TTL`](#sctp-pr-sctp-ttl)
  - [`SCTP_PR_SCTP_RTX`](#sctp-pr-sctp-rtx)
  - [`SCTP_PR_SCTP_PRIO`](#sctp-pr-sctp-prio)
  - [`SCTP_PR_SCTP_MAX`](#sctp-pr-sctp-max)
  - [`SCTP_PR_SCTP_MASK`](#sctp-pr-sctp-mask)
  - [`SCTP_ENABLE_RESET_STREAM_REQ`](#sctp-enable-reset-stream-req)
  - [`SCTP_ENABLE_RESET_ASSOC_REQ`](#sctp-enable-reset-assoc-req)
  - [`SCTP_ENABLE_CHANGE_ASSOC_REQ`](#sctp-enable-change-assoc-req)
  - [`SCTP_ENABLE_STRRESET_MASK`](#sctp-enable-strreset-mask)
  - [`SCTP_STREAM_RESET_INCOMING`](#sctp-stream-reset-incoming)
  - [`SCTP_STREAM_RESET_OUTGOING`](#sctp-stream-reset-outgoing)
  - [`SCTP_INIT`](#sctp-init)
  - [`SCTP_SNDRCV`](#sctp-sndrcv)
  - [`SCTP_SNDINFO`](#sctp-sndinfo)
  - [`SCTP_RCVINFO`](#sctp-rcvinfo)
  - [`SCTP_NXTINFO`](#sctp-nxtinfo)
  - [`SCTP_PRINFO`](#sctp-prinfo)
  - [`SCTP_AUTHINFO`](#sctp-authinfo)
  - [`SCTP_DSTADDRV4`](#sctp-dstaddrv4)
  - [`SCTP_DSTADDRV6`](#sctp-dstaddrv6)
  - [`SCTP_UNORDERED`](#sctp-unordered)
  - [`SCTP_ADDR_OVER`](#sctp-addr-over)
  - [`SCTP_ABORT`](#sctp-abort)
  - [`SCTP_SACK_IMMEDIATELY`](#sctp-sack-immediately)
  - [`SCTP_SENDALL`](#sctp-sendall)
  - [`SCTP_PR_SCTP_ALL`](#sctp-pr-sctp-all)
  - [`SCTP_NOTIFICATION`](#sctp-notification)
  - [`SCTP_EOF`](#sctp-eof)
  - [`DCCP_SOCKOPT_PACKET_SIZE`](#dccp-sockopt-packet-size)
  - [`DCCP_SOCKOPT_SERVICE`](#dccp-sockopt-service)
  - [`DCCP_SOCKOPT_CHANGE_L`](#dccp-sockopt-change-l)
  - [`DCCP_SOCKOPT_CHANGE_R`](#dccp-sockopt-change-r)
  - [`DCCP_SOCKOPT_GET_CUR_MPS`](#dccp-sockopt-get-cur-mps)
  - [`DCCP_SOCKOPT_SERVER_TIMEWAIT`](#dccp-sockopt-server-timewait)
  - [`DCCP_SOCKOPT_SEND_CSCOV`](#dccp-sockopt-send-cscov)
  - [`DCCP_SOCKOPT_RECV_CSCOV`](#dccp-sockopt-recv-cscov)
  - [`DCCP_SOCKOPT_AVAILABLE_CCIDS`](#dccp-sockopt-available-ccids)
  - [`DCCP_SOCKOPT_CCID`](#dccp-sockopt-ccid)
  - [`DCCP_SOCKOPT_TX_CCID`](#dccp-sockopt-tx-ccid)
  - [`DCCP_SOCKOPT_RX_CCID`](#dccp-sockopt-rx-ccid)
  - [`DCCP_SOCKOPT_QPOLICY_ID`](#dccp-sockopt-qpolicy-id)
  - [`DCCP_SOCKOPT_QPOLICY_TXQLEN`](#dccp-sockopt-qpolicy-txqlen)
  - [`DCCP_SOCKOPT_CCID_RX_INFO`](#dccp-sockopt-ccid-rx-info)
  - [`DCCP_SOCKOPT_CCID_TX_INFO`](#dccp-sockopt-ccid-tx-info)
  - [`DCCP_SERVICE_LIST_MAX_LEN`](#dccp-service-list-max-len)
  - [`CTL_KERN`](#ctl-kern)
  - [`CTL_VM`](#ctl-vm)
  - [`CTL_NET`](#ctl-net)
  - [`CTL_FS`](#ctl-fs)
  - [`CTL_DEBUG`](#ctl-debug)
  - [`CTL_DEV`](#ctl-dev)
  - [`CTL_BUS`](#ctl-bus)
  - [`CTL_ABI`](#ctl-abi)
  - [`CTL_CPU`](#ctl-cpu)
  - [`CTL_BUS_ISA`](#ctl-bus-isa)
  - [`INOTIFY_MAX_USER_INSTANCES`](#inotify-max-user-instances)
  - [`INOTIFY_MAX_USER_WATCHES`](#inotify-max-user-watches)
  - [`INOTIFY_MAX_QUEUED_EVENTS`](#inotify-max-queued-events)
  - [`KERN_OSTYPE`](#kern-ostype)
  - [`KERN_OSRELEASE`](#kern-osrelease)
  - [`KERN_OSREV`](#kern-osrev)
  - [`KERN_VERSION`](#kern-version)
  - [`KERN_SECUREMASK`](#kern-securemask)
  - [`KERN_PROF`](#kern-prof)
  - [`KERN_NODENAME`](#kern-nodename)
  - [`KERN_DOMAINNAME`](#kern-domainname)
  - [`KERN_PANIC`](#kern-panic)
  - [`KERN_REALROOTDEV`](#kern-realrootdev)
  - [`KERN_SPARC_REBOOT`](#kern-sparc-reboot)
  - [`KERN_CTLALTDEL`](#kern-ctlaltdel)
  - [`KERN_PRINTK`](#kern-printk)
  - [`KERN_NAMETRANS`](#kern-nametrans)
  - [`KERN_PPC_HTABRECLAIM`](#kern-ppc-htabreclaim)
  - [`KERN_PPC_ZEROPAGED`](#kern-ppc-zeropaged)
  - [`KERN_PPC_POWERSAVE_NAP`](#kern-ppc-powersave-nap)
  - [`KERN_MODPROBE`](#kern-modprobe)
  - [`KERN_SG_BIG_BUFF`](#kern-sg-big-buff)
  - [`KERN_ACCT`](#kern-acct)
  - [`KERN_PPC_L2CR`](#kern-ppc-l2cr)
  - [`KERN_RTSIGNR`](#kern-rtsignr)
  - [`KERN_RTSIGMAX`](#kern-rtsigmax)
  - [`KERN_SHMMAX`](#kern-shmmax)
  - [`KERN_MSGMAX`](#kern-msgmax)
  - [`KERN_MSGMNB`](#kern-msgmnb)
  - [`KERN_MSGPOOL`](#kern-msgpool)
  - [`KERN_SYSRQ`](#kern-sysrq)
  - [`KERN_MAX_THREADS`](#kern-max-threads)
  - [`KERN_RANDOM`](#kern-random)
  - [`KERN_SHMALL`](#kern-shmall)
  - [`KERN_MSGMNI`](#kern-msgmni)
  - [`KERN_SEM`](#kern-sem)
  - [`KERN_SPARC_STOP_A`](#kern-sparc-stop-a)
  - [`KERN_SHMMNI`](#kern-shmmni)
  - [`KERN_OVERFLOWUID`](#kern-overflowuid)
  - [`KERN_OVERFLOWGID`](#kern-overflowgid)
  - [`KERN_SHMPATH`](#kern-shmpath)
  - [`KERN_HOTPLUG`](#kern-hotplug)
  - [`KERN_IEEE_EMULATION_WARNINGS`](#kern-ieee-emulation-warnings)
  - [`KERN_S390_USER_DEBUG_LOGGING`](#kern-s390-user-debug-logging)
  - [`KERN_CORE_USES_PID`](#kern-core-uses-pid)
  - [`KERN_TAINTED`](#kern-tainted)
  - [`KERN_CADPID`](#kern-cadpid)
  - [`KERN_PIDMAX`](#kern-pidmax)
  - [`KERN_CORE_PATTERN`](#kern-core-pattern)
  - [`KERN_PANIC_ON_OOPS`](#kern-panic-on-oops)
  - [`KERN_HPPA_PWRSW`](#kern-hppa-pwrsw)
  - [`KERN_HPPA_UNALIGNED`](#kern-hppa-unaligned)
  - [`KERN_PRINTK_RATELIMIT`](#kern-printk-ratelimit)
  - [`KERN_PRINTK_RATELIMIT_BURST`](#kern-printk-ratelimit-burst)
  - [`KERN_PTY`](#kern-pty)
  - [`KERN_NGROUPS_MAX`](#kern-ngroups-max)
  - [`KERN_SPARC_SCONS_PWROFF`](#kern-sparc-scons-pwroff)
  - [`KERN_HZ_TIMER`](#kern-hz-timer)
  - [`KERN_UNKNOWN_NMI_PANIC`](#kern-unknown-nmi-panic)
  - [`KERN_BOOTLOADER_TYPE`](#kern-bootloader-type)
  - [`KERN_RANDOMIZE`](#kern-randomize)
  - [`KERN_SETUID_DUMPABLE`](#kern-setuid-dumpable)
  - [`KERN_SPIN_RETRY`](#kern-spin-retry)
  - [`KERN_ACPI_VIDEO_FLAGS`](#kern-acpi-video-flags)
  - [`KERN_IA64_UNALIGNED`](#kern-ia64-unaligned)
  - [`KERN_COMPAT_LOG`](#kern-compat-log)
  - [`KERN_MAX_LOCK_DEPTH`](#kern-max-lock-depth)
  - [`KERN_NMI_WATCHDOG`](#kern-nmi-watchdog)
  - [`KERN_PANIC_ON_NMI`](#kern-panic-on-nmi)
  - [`VM_OVERCOMMIT_MEMORY`](#vm-overcommit-memory)
  - [`VM_PAGE_CLUSTER`](#vm-page-cluster)
  - [`VM_DIRTY_BACKGROUND`](#vm-dirty-background)
  - [`VM_DIRTY_RATIO`](#vm-dirty-ratio)
  - [`VM_DIRTY_WB_CS`](#vm-dirty-wb-cs)
  - [`VM_DIRTY_EXPIRE_CS`](#vm-dirty-expire-cs)
  - [`VM_NR_PDFLUSH_THREADS`](#vm-nr-pdflush-threads)
  - [`VM_OVERCOMMIT_RATIO`](#vm-overcommit-ratio)
  - [`VM_PAGEBUF`](#vm-pagebuf)
  - [`VM_HUGETLB_PAGES`](#vm-hugetlb-pages)
  - [`VM_SWAPPINESS`](#vm-swappiness)
  - [`VM_LOWMEM_RESERVE_RATIO`](#vm-lowmem-reserve-ratio)
  - [`VM_MIN_FREE_KBYTES`](#vm-min-free-kbytes)
  - [`VM_MAX_MAP_COUNT`](#vm-max-map-count)
  - [`VM_LAPTOP_MODE`](#vm-laptop-mode)
  - [`VM_BLOCK_DUMP`](#vm-block-dump)
  - [`VM_HUGETLB_GROUP`](#vm-hugetlb-group)
  - [`VM_VFS_CACHE_PRESSURE`](#vm-vfs-cache-pressure)
  - [`VM_LEGACY_VA_LAYOUT`](#vm-legacy-va-layout)
  - [`VM_SWAP_TOKEN_TIMEOUT`](#vm-swap-token-timeout)
  - [`VM_DROP_PAGECACHE`](#vm-drop-pagecache)
  - [`VM_PERCPU_PAGELIST_FRACTION`](#vm-percpu-pagelist-fraction)
  - [`VM_ZONE_RECLAIM_MODE`](#vm-zone-reclaim-mode)
  - [`VM_MIN_UNMAPPED`](#vm-min-unmapped)
  - [`VM_PANIC_ON_OOM`](#vm-panic-on-oom)
  - [`VM_VDSO_ENABLED`](#vm-vdso-enabled)
  - [`VM_MIN_SLAB`](#vm-min-slab)
  - [`NET_CORE`](#net-core)
  - [`NET_ETHER`](#net-ether)
  - [`NET_802`](#net-802)
  - [`NET_UNIX`](#net-unix)
  - [`NET_IPV4`](#net-ipv4)
  - [`NET_IPX`](#net-ipx)
  - [`NET_ATALK`](#net-atalk)
  - [`NET_NETROM`](#net-netrom)
  - [`NET_AX25`](#net-ax25)
  - [`NET_BRIDGE`](#net-bridge)
  - [`NET_ROSE`](#net-rose)
  - [`NET_IPV6`](#net-ipv6)
  - [`NET_X25`](#net-x25)
  - [`NET_TR`](#net-tr)
  - [`NET_DECNET`](#net-decnet)
  - [`NET_ECONET`](#net-econet)
  - [`NET_SCTP`](#net-sctp)
  - [`NET_LLC`](#net-llc)
  - [`NET_NETFILTER`](#net-netfilter)
  - [`NET_DCCP`](#net-dccp)
  - [`NET_IRDA`](#net-irda)
  - [`PF_VCPU`](#pf-vcpu)
  - [`PF_IDLE`](#pf-idle)
  - [`PF_EXITING`](#pf-exiting)
  - [`PF_POSTCOREDUMP`](#pf-postcoredump)
  - [`PF_IO_WORKER`](#pf-io-worker)
  - [`PF_WQ_WORKER`](#pf-wq-worker)
  - [`PF_FORKNOEXEC`](#pf-forknoexec)
  - [`PF_MCE_PROCESS`](#pf-mce-process)
  - [`PF_SUPERPRIV`](#pf-superpriv)
  - [`PF_DUMPCORE`](#pf-dumpcore)
  - [`PF_SIGNALED`](#pf-signaled)
  - [`PF_MEMALLOC`](#pf-memalloc)
  - [`PF_NPROC_EXCEEDED`](#pf-nproc-exceeded)
  - [`PF_USED_MATH`](#pf-used-math)
  - [`PF_USER_WORKER`](#pf-user-worker)
  - [`PF_NOFREEZE`](#pf-nofreeze)
  - [`PF_KSWAPD`](#pf-kswapd)
  - [`PF_MEMALLOC_NOFS`](#pf-memalloc-nofs)
  - [`PF_MEMALLOC_NOIO`](#pf-memalloc-noio)
  - [`PF_LOCAL_THROTTLE`](#pf-local-throttle)
  - [`PF_KTHREAD`](#pf-kthread)
  - [`PF_RANDOMIZE`](#pf-randomize)
  - [`PF_NO_SETAFFINITY`](#pf-no-setaffinity)
  - [`PF_MCE_EARLY`](#pf-mce-early)
  - [`PF_MEMALLOC_PIN`](#pf-memalloc-pin)
  - [`PF_BLOCK_TS`](#pf-block-ts)
  - [`PF_SUSPEND_TASK`](#pf-suspend-task)
  - [`PF_SUSPEND_TASK_UINT`](#pf-suspend-task-uint)
  - [`CLONE_PIDFD`](#clone-pidfd)
  - [`SCHED_FLAG_RESET_ON_FORK`](#sched-flag-reset-on-fork)
  - [`SCHED_FLAG_RECLAIM`](#sched-flag-reclaim)
  - [`SCHED_FLAG_DL_OVERRUN`](#sched-flag-dl-overrun)
  - [`SCHED_FLAG_KEEP_POLICY`](#sched-flag-keep-policy)
  - [`SCHED_FLAG_KEEP_PARAMS`](#sched-flag-keep-params)
  - [`SCHED_FLAG_UTIL_CLAMP_MIN`](#sched-flag-util-clamp-min)
  - [`SCHED_FLAG_UTIL_CLAMP_MAX`](#sched-flag-util-clamp-max)
  - [`XDP_SHARED_UMEM`](#xdp-shared-umem)
  - [`XDP_COPY`](#xdp-copy)
  - [`XDP_ZEROCOPY`](#xdp-zerocopy)
  - [`XDP_USE_NEED_WAKEUP`](#xdp-use-need-wakeup)
  - [`XDP_USE_SG`](#xdp-use-sg)
  - [`XDP_UMEM_UNALIGNED_CHUNK_FLAG`](#xdp-umem-unaligned-chunk-flag)
  - [`XDP_RING_NEED_WAKEUP`](#xdp-ring-need-wakeup)
  - [`XDP_MMAP_OFFSETS`](#xdp-mmap-offsets)
  - [`XDP_RX_RING`](#xdp-rx-ring)
  - [`XDP_TX_RING`](#xdp-tx-ring)
  - [`XDP_UMEM_REG`](#xdp-umem-reg)
  - [`XDP_UMEM_FILL_RING`](#xdp-umem-fill-ring)
  - [`XDP_UMEM_COMPLETION_RING`](#xdp-umem-completion-ring)
  - [`XDP_STATISTICS`](#xdp-statistics)
  - [`XDP_OPTIONS`](#xdp-options)
  - [`XDP_OPTIONS_ZEROCOPY`](#xdp-options-zerocopy)
  - [`XDP_PGOFF_RX_RING`](#xdp-pgoff-rx-ring)
  - [`XDP_PGOFF_TX_RING`](#xdp-pgoff-tx-ring)
  - [`XDP_UMEM_PGOFF_FILL_RING`](#xdp-umem-pgoff-fill-ring)
  - [`XDP_UMEM_PGOFF_COMPLETION_RING`](#xdp-umem-pgoff-completion-ring)
  - [`XSK_UNALIGNED_BUF_OFFSET_SHIFT`](#xsk-unaligned-buf-offset-shift)
  - [`XSK_UNALIGNED_BUF_ADDR_MASK`](#xsk-unaligned-buf-addr-mask)
  - [`XDP_PKT_CONTD`](#xdp-pkt-contd)
  - [`XDP_UMEM_TX_SW_CSUM`](#xdp-umem-tx-sw-csum)
  - [`XDP_UMEM_TX_METADATA_LEN`](#xdp-umem-tx-metadata-len)
  - [`XDP_TXMD_FLAGS_TIMESTAMP`](#xdp-txmd-flags-timestamp)
  - [`XDP_TXMD_FLAGS_CHECKSUM`](#xdp-txmd-flags-checksum)
  - [`XDP_TX_METADATA`](#xdp-tx-metadata)
  - [`SOL_XDP`](#sol-xdp)
  - [`MOUNT_ATTR_RDONLY`](#mount-attr-rdonly)
  - [`MOUNT_ATTR_NOSUID`](#mount-attr-nosuid)
  - [`MOUNT_ATTR_NODEV`](#mount-attr-nodev)
  - [`MOUNT_ATTR_NOEXEC`](#mount-attr-noexec)
  - [`MOUNT_ATTR__ATIME`](#mount-attr-atime)
  - [`MOUNT_ATTR_RELATIME`](#mount-attr-relatime)
  - [`MOUNT_ATTR_NOATIME`](#mount-attr-noatime)
  - [`MOUNT_ATTR_STRICTATIME`](#mount-attr-strictatime)
  - [`MOUNT_ATTR_NODIRATIME`](#mount-attr-nodiratime)
  - [`MOUNT_ATTR_IDMAP`](#mount-attr-idmap)
  - [`MOUNT_ATTR_NOSYMFOLLOW`](#mount-attr-nosymfollow)
  - [`MOUNT_ATTR_SIZE_VER0`](#mount-attr-size-ver0)
  - [`SCHED_FLAG_KEEP_ALL`](#sched-flag-keep-all)
  - [`SCHED_FLAG_UTIL_CLAMP`](#sched-flag-util-clamp)
  - [`SCHED_FLAG_ALL`](#sched-flag-all)
  - [`EPIOCSPARAMS`](#epiocsparams)
  - [`EPIOCGPARAMS`](#epiocgparams)
  - [`SI_DETHREAD`](#si-dethread)
  - [`TRAP_PERF`](#trap-perf)
  - [`DATE_BASE`](#date-base)
  - [`ABDAY_1`](#abday-1)
  - [`ABDAY_2`](#abday-2)
  - [`ABDAY_3`](#abday-3)
  - [`ABDAY_4`](#abday-4)
  - [`ABDAY_5`](#abday-5)
  - [`ABDAY_6`](#abday-6)
  - [`ABDAY_7`](#abday-7)
  - [`DAY_1`](#day-1)
  - [`DAY_2`](#day-2)
  - [`DAY_3`](#day-3)
  - [`DAY_4`](#day-4)
  - [`DAY_5`](#day-5)
  - [`DAY_6`](#day-6)
  - [`DAY_7`](#day-7)
  - [`ABMON_1`](#abmon-1)
  - [`ABMON_2`](#abmon-2)
  - [`ABMON_3`](#abmon-3)
  - [`ABMON_4`](#abmon-4)
  - [`ABMON_5`](#abmon-5)
  - [`ABMON_6`](#abmon-6)
  - [`ABMON_7`](#abmon-7)
  - [`ABMON_8`](#abmon-8)
  - [`ABMON_9`](#abmon-9)
  - [`ABMON_10`](#abmon-10)
  - [`ABMON_11`](#abmon-11)
  - [`ABMON_12`](#abmon-12)
  - [`MON_1`](#mon-1)
  - [`MON_2`](#mon-2)
  - [`MON_3`](#mon-3)
  - [`MON_4`](#mon-4)
  - [`MON_5`](#mon-5)
  - [`MON_6`](#mon-6)
  - [`MON_7`](#mon-7)
  - [`MON_8`](#mon-8)
  - [`MON_9`](#mon-9)
  - [`MON_10`](#mon-10)
  - [`MON_11`](#mon-11)
  - [`MON_12`](#mon-12)
  - [`AM_STR`](#am-str)
  - [`PM_STR`](#pm-str)
  - [`D_T_FMT`](#d-t-fmt)
  - [`D_FMT`](#d-fmt)
  - [`T_FMT`](#t-fmt)
  - [`T_FMT_AMPM`](#t-fmt-ampm)
  - [`ERA`](#era)
  - [`ERA_D_FMT`](#era-d-fmt)
  - [`ALT_DIGITS`](#alt-digits)
  - [`ERA_D_T_FMT`](#era-d-t-fmt)
  - [`ERA_T_FMT`](#era-t-fmt)
  - [`CODESET`](#codeset)
  - [`CRNCYSTR`](#crncystr)
  - [`RADIXCHAR`](#radixchar)
  - [`THOUSEP`](#thousep)
  - [`YESEXPR`](#yesexpr)
  - [`NOEXPR`](#noexpr)
  - [`YESSTR`](#yesstr)
  - [`NOSTR`](#nostr)
  - [`RUSAGE_CHILDREN`](#rusage-children)
  - [`L_tmpnam`](#l-tmpnam)
  - [`_PC_LINK_MAX`](#pc-link-max)
  - [`_PC_MAX_CANON`](#pc-max-canon)
  - [`_PC_MAX_INPUT`](#pc-max-input)
  - [`_PC_NAME_MAX`](#pc-name-max)
  - [`_PC_PATH_MAX`](#pc-path-max)
  - [`_PC_PIPE_BUF`](#pc-pipe-buf)
  - [`_PC_CHOWN_RESTRICTED`](#pc-chown-restricted)
  - [`_PC_NO_TRUNC`](#pc-no-trunc)
  - [`_PC_VDISABLE`](#pc-vdisable)
  - [`_PC_SYNC_IO`](#pc-sync-io)
  - [`_PC_ASYNC_IO`](#pc-async-io)
  - [`_PC_PRIO_IO`](#pc-prio-io)
  - [`_PC_SOCK_MAXBUF`](#pc-sock-maxbuf)
  - [`_PC_FILESIZEBITS`](#pc-filesizebits)
  - [`_PC_REC_INCR_XFER_SIZE`](#pc-rec-incr-xfer-size)
  - [`_PC_REC_MAX_XFER_SIZE`](#pc-rec-max-xfer-size)
  - [`_PC_REC_MIN_XFER_SIZE`](#pc-rec-min-xfer-size)
  - [`_PC_REC_XFER_ALIGN`](#pc-rec-xfer-align)
  - [`_PC_ALLOC_SIZE_MIN`](#pc-alloc-size-min)
  - [`_PC_SYMLINK_MAX`](#pc-symlink-max)
  - [`_PC_2_SYMLINKS`](#pc-2-symlinks)
  - [`_SC_ARG_MAX`](#sc-arg-max)
  - [`_SC_CHILD_MAX`](#sc-child-max)
  - [`_SC_CLK_TCK`](#sc-clk-tck)
  - [`_SC_NGROUPS_MAX`](#sc-ngroups-max)
  - [`_SC_OPEN_MAX`](#sc-open-max)
  - [`_SC_STREAM_MAX`](#sc-stream-max)
  - [`_SC_TZNAME_MAX`](#sc-tzname-max)
  - [`_SC_JOB_CONTROL`](#sc-job-control)
  - [`_SC_SAVED_IDS`](#sc-saved-ids)
  - [`_SC_REALTIME_SIGNALS`](#sc-realtime-signals)
  - [`_SC_PRIORITY_SCHEDULING`](#sc-priority-scheduling)
  - [`_SC_TIMERS`](#sc-timers)
  - [`_SC_ASYNCHRONOUS_IO`](#sc-asynchronous-io)
  - [`_SC_PRIORITIZED_IO`](#sc-prioritized-io)
  - [`_SC_SYNCHRONIZED_IO`](#sc-synchronized-io)
  - [`_SC_FSYNC`](#sc-fsync)
  - [`_SC_MAPPED_FILES`](#sc-mapped-files)
  - [`_SC_MEMLOCK`](#sc-memlock)
  - [`_SC_MEMLOCK_RANGE`](#sc-memlock-range)
  - [`_SC_MEMORY_PROTECTION`](#sc-memory-protection)
  - [`_SC_MESSAGE_PASSING`](#sc-message-passing)
  - [`_SC_SEMAPHORES`](#sc-semaphores)
  - [`_SC_SHARED_MEMORY_OBJECTS`](#sc-shared-memory-objects)
  - [`_SC_AIO_LISTIO_MAX`](#sc-aio-listio-max)
  - [`_SC_AIO_MAX`](#sc-aio-max)
  - [`_SC_AIO_PRIO_DELTA_MAX`](#sc-aio-prio-delta-max)
  - [`_SC_DELAYTIMER_MAX`](#sc-delaytimer-max)
  - [`_SC_MQ_OPEN_MAX`](#sc-mq-open-max)
  - [`_SC_MQ_PRIO_MAX`](#sc-mq-prio-max)
  - [`_SC_VERSION`](#sc-version)
  - [`_SC_PAGESIZE`](#sc-pagesize)
  - [`_SC_PAGE_SIZE`](#sc-page-size)
  - [`_SC_RTSIG_MAX`](#sc-rtsig-max)
  - [`_SC_SEM_NSEMS_MAX`](#sc-sem-nsems-max)
  - [`_SC_SEM_VALUE_MAX`](#sc-sem-value-max)
  - [`_SC_SIGQUEUE_MAX`](#sc-sigqueue-max)
  - [`_SC_TIMER_MAX`](#sc-timer-max)
  - [`_SC_BC_BASE_MAX`](#sc-bc-base-max)
  - [`_SC_BC_DIM_MAX`](#sc-bc-dim-max)
  - [`_SC_BC_SCALE_MAX`](#sc-bc-scale-max)
  - [`_SC_BC_STRING_MAX`](#sc-bc-string-max)
  - [`_SC_COLL_WEIGHTS_MAX`](#sc-coll-weights-max)
  - [`_SC_EXPR_NEST_MAX`](#sc-expr-nest-max)
  - [`_SC_LINE_MAX`](#sc-line-max)
  - [`_SC_RE_DUP_MAX`](#sc-re-dup-max)
  - [`_SC_2_VERSION`](#sc-2-version)
  - [`_SC_2_C_BIND`](#sc-2-c-bind)
  - [`_SC_2_C_DEV`](#sc-2-c-dev)
  - [`_SC_2_FORT_DEV`](#sc-2-fort-dev)
  - [`_SC_2_FORT_RUN`](#sc-2-fort-run)
  - [`_SC_2_SW_DEV`](#sc-2-sw-dev)
  - [`_SC_2_LOCALEDEF`](#sc-2-localedef)
  - [`_SC_UIO_MAXIOV`](#sc-uio-maxiov)
  - [`_SC_IOV_MAX`](#sc-iov-max)
  - [`_SC_THREADS`](#sc-threads)
  - [`_SC_THREAD_SAFE_FUNCTIONS`](#sc-thread-safe-functions)
  - [`_SC_GETGR_R_SIZE_MAX`](#sc-getgr-r-size-max)
  - [`_SC_GETPW_R_SIZE_MAX`](#sc-getpw-r-size-max)
  - [`_SC_LOGIN_NAME_MAX`](#sc-login-name-max)
  - [`_SC_TTY_NAME_MAX`](#sc-tty-name-max)
  - [`_SC_THREAD_DESTRUCTOR_ITERATIONS`](#sc-thread-destructor-iterations)
  - [`_SC_THREAD_KEYS_MAX`](#sc-thread-keys-max)
  - [`_SC_THREAD_STACK_MIN`](#sc-thread-stack-min)
  - [`_SC_THREAD_THREADS_MAX`](#sc-thread-threads-max)
  - [`_SC_THREAD_ATTR_STACKADDR`](#sc-thread-attr-stackaddr)
  - [`_SC_THREAD_ATTR_STACKSIZE`](#sc-thread-attr-stacksize)
  - [`_SC_THREAD_PRIORITY_SCHEDULING`](#sc-thread-priority-scheduling)
  - [`_SC_THREAD_PRIO_INHERIT`](#sc-thread-prio-inherit)
  - [`_SC_THREAD_PRIO_PROTECT`](#sc-thread-prio-protect)
  - [`_SC_THREAD_PROCESS_SHARED`](#sc-thread-process-shared)
  - [`_SC_NPROCESSORS_CONF`](#sc-nprocessors-conf)
  - [`_SC_NPROCESSORS_ONLN`](#sc-nprocessors-onln)
  - [`_SC_PHYS_PAGES`](#sc-phys-pages)
  - [`_SC_AVPHYS_PAGES`](#sc-avphys-pages)
  - [`_SC_ATEXIT_MAX`](#sc-atexit-max)
  - [`_SC_PASS_MAX`](#sc-pass-max)
  - [`_SC_XOPEN_VERSION`](#sc-xopen-version)
  - [`_SC_XOPEN_XCU_VERSION`](#sc-xopen-xcu-version)
  - [`_SC_XOPEN_UNIX`](#sc-xopen-unix)
  - [`_SC_XOPEN_CRYPT`](#sc-xopen-crypt)
  - [`_SC_XOPEN_ENH_I18N`](#sc-xopen-enh-i18n)
  - [`_SC_XOPEN_SHM`](#sc-xopen-shm)
  - [`_SC_2_CHAR_TERM`](#sc-2-char-term)
  - [`_SC_2_UPE`](#sc-2-upe)
  - [`_SC_XOPEN_XPG2`](#sc-xopen-xpg2)
  - [`_SC_XOPEN_XPG3`](#sc-xopen-xpg3)
  - [`_SC_XOPEN_XPG4`](#sc-xopen-xpg4)
  - [`_SC_NZERO`](#sc-nzero)
  - [`_SC_XBS5_ILP32_OFF32`](#sc-xbs5-ilp32-off32)
  - [`_SC_XBS5_ILP32_OFFBIG`](#sc-xbs5-ilp32-offbig)
  - [`_SC_XBS5_LP64_OFF64`](#sc-xbs5-lp64-off64)
  - [`_SC_XBS5_LPBIG_OFFBIG`](#sc-xbs5-lpbig-offbig)
  - [`_SC_XOPEN_LEGACY`](#sc-xopen-legacy)
  - [`_SC_XOPEN_REALTIME`](#sc-xopen-realtime)
  - [`_SC_XOPEN_REALTIME_THREADS`](#sc-xopen-realtime-threads)
  - [`_SC_ADVISORY_INFO`](#sc-advisory-info)
  - [`_SC_BARRIERS`](#sc-barriers)
  - [`_SC_CLOCK_SELECTION`](#sc-clock-selection)
  - [`_SC_CPUTIME`](#sc-cputime)
  - [`_SC_THREAD_CPUTIME`](#sc-thread-cputime)
  - [`_SC_MONOTONIC_CLOCK`](#sc-monotonic-clock)
  - [`_SC_READER_WRITER_LOCKS`](#sc-reader-writer-locks)
  - [`_SC_SPIN_LOCKS`](#sc-spin-locks)
  - [`_SC_REGEXP`](#sc-regexp)
  - [`_SC_SHELL`](#sc-shell)
  - [`_SC_SPAWN`](#sc-spawn)
  - [`_SC_SPORADIC_SERVER`](#sc-sporadic-server)
  - [`_SC_THREAD_SPORADIC_SERVER`](#sc-thread-sporadic-server)
  - [`_SC_TIMEOUTS`](#sc-timeouts)
  - [`_SC_TYPED_MEMORY_OBJECTS`](#sc-typed-memory-objects)
  - [`_SC_2_PBS`](#sc-2-pbs)
  - [`_SC_2_PBS_ACCOUNTING`](#sc-2-pbs-accounting)
  - [`_SC_2_PBS_LOCATE`](#sc-2-pbs-locate)
  - [`_SC_2_PBS_MESSAGE`](#sc-2-pbs-message)
  - [`_SC_2_PBS_TRACK`](#sc-2-pbs-track)
  - [`_SC_SYMLOOP_MAX`](#sc-symloop-max)
  - [`_SC_STREAMS`](#sc-streams)
  - [`_SC_2_PBS_CHECKPOINT`](#sc-2-pbs-checkpoint)
  - [`_SC_V6_ILP32_OFF32`](#sc-v6-ilp32-off32)
  - [`_SC_V6_ILP32_OFFBIG`](#sc-v6-ilp32-offbig)
  - [`_SC_V6_LP64_OFF64`](#sc-v6-lp64-off64)
  - [`_SC_V6_LPBIG_OFFBIG`](#sc-v6-lpbig-offbig)
  - [`_SC_HOST_NAME_MAX`](#sc-host-name-max)
  - [`_SC_TRACE`](#sc-trace)
  - [`_SC_TRACE_EVENT_FILTER`](#sc-trace-event-filter)
  - [`_SC_TRACE_INHERIT`](#sc-trace-inherit)
  - [`_SC_TRACE_LOG`](#sc-trace-log)
  - [`_SC_IPV6`](#sc-ipv6)
  - [`_SC_RAW_SOCKETS`](#sc-raw-sockets)
  - [`_SC_V7_ILP32_OFF32`](#sc-v7-ilp32-off32)
  - [`_SC_V7_ILP32_OFFBIG`](#sc-v7-ilp32-offbig)
  - [`_SC_V7_LP64_OFF64`](#sc-v7-lp64-off64)
  - [`_SC_V7_LPBIG_OFFBIG`](#sc-v7-lpbig-offbig)
  - [`_SC_SS_REPL_MAX`](#sc-ss-repl-max)
  - [`_SC_TRACE_EVENT_NAME_MAX`](#sc-trace-event-name-max)
  - [`_SC_TRACE_NAME_MAX`](#sc-trace-name-max)
  - [`_SC_TRACE_SYS_MAX`](#sc-trace-sys-max)
  - [`_SC_TRACE_USER_EVENT_MAX`](#sc-trace-user-event-max)
  - [`_SC_XOPEN_STREAMS`](#sc-xopen-streams)
  - [`_SC_THREAD_ROBUST_PRIO_INHERIT`](#sc-thread-robust-prio-inherit)
  - [`_SC_THREAD_ROBUST_PRIO_PROTECT`](#sc-thread-robust-prio-protect)
  - [`_CS_PATH`](#cs-path)
  - [`_CS_POSIX_V6_WIDTH_RESTRICTED_ENVS`](#cs-posix-v6-width-restricted-envs)
  - [`_CS_POSIX_V5_WIDTH_RESTRICTED_ENVS`](#cs-posix-v5-width-restricted-envs)
  - [`_CS_POSIX_V7_WIDTH_RESTRICTED_ENVS`](#cs-posix-v7-width-restricted-envs)
  - [`_CS_POSIX_V6_ILP32_OFF32_CFLAGS`](#cs-posix-v6-ilp32-off32-cflags)
  - [`_CS_POSIX_V6_ILP32_OFF32_LDFLAGS`](#cs-posix-v6-ilp32-off32-ldflags)
  - [`_CS_POSIX_V6_ILP32_OFF32_LIBS`](#cs-posix-v6-ilp32-off32-libs)
  - [`_CS_POSIX_V6_ILP32_OFF32_LINTFLAGS`](#cs-posix-v6-ilp32-off32-lintflags)
  - [`_CS_POSIX_V6_ILP32_OFFBIG_CFLAGS`](#cs-posix-v6-ilp32-offbig-cflags)
  - [`_CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS`](#cs-posix-v6-ilp32-offbig-ldflags)
  - [`_CS_POSIX_V6_ILP32_OFFBIG_LIBS`](#cs-posix-v6-ilp32-offbig-libs)
  - [`_CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS`](#cs-posix-v6-ilp32-offbig-lintflags)
  - [`_CS_POSIX_V6_LP64_OFF64_CFLAGS`](#cs-posix-v6-lp64-off64-cflags)
  - [`_CS_POSIX_V6_LP64_OFF64_LDFLAGS`](#cs-posix-v6-lp64-off64-ldflags)
  - [`_CS_POSIX_V6_LP64_OFF64_LIBS`](#cs-posix-v6-lp64-off64-libs)
  - [`_CS_POSIX_V6_LP64_OFF64_LINTFLAGS`](#cs-posix-v6-lp64-off64-lintflags)
  - [`_CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS`](#cs-posix-v6-lpbig-offbig-cflags)
  - [`_CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS`](#cs-posix-v6-lpbig-offbig-ldflags)
  - [`_CS_POSIX_V6_LPBIG_OFFBIG_LIBS`](#cs-posix-v6-lpbig-offbig-libs)
  - [`_CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS`](#cs-posix-v6-lpbig-offbig-lintflags)
  - [`_CS_POSIX_V7_ILP32_OFF32_CFLAGS`](#cs-posix-v7-ilp32-off32-cflags)
  - [`_CS_POSIX_V7_ILP32_OFF32_LDFLAGS`](#cs-posix-v7-ilp32-off32-ldflags)
  - [`_CS_POSIX_V7_ILP32_OFF32_LIBS`](#cs-posix-v7-ilp32-off32-libs)
  - [`_CS_POSIX_V7_ILP32_OFF32_LINTFLAGS`](#cs-posix-v7-ilp32-off32-lintflags)
  - [`_CS_POSIX_V7_ILP32_OFFBIG_CFLAGS`](#cs-posix-v7-ilp32-offbig-cflags)
  - [`_CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS`](#cs-posix-v7-ilp32-offbig-ldflags)
  - [`_CS_POSIX_V7_ILP32_OFFBIG_LIBS`](#cs-posix-v7-ilp32-offbig-libs)
  - [`_CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS`](#cs-posix-v7-ilp32-offbig-lintflags)
  - [`_CS_POSIX_V7_LP64_OFF64_CFLAGS`](#cs-posix-v7-lp64-off64-cflags)
  - [`_CS_POSIX_V7_LP64_OFF64_LDFLAGS`](#cs-posix-v7-lp64-off64-ldflags)
  - [`_CS_POSIX_V7_LP64_OFF64_LIBS`](#cs-posix-v7-lp64-off64-libs)
  - [`_CS_POSIX_V7_LP64_OFF64_LINTFLAGS`](#cs-posix-v7-lp64-off64-lintflags)
  - [`_CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS`](#cs-posix-v7-lpbig-offbig-cflags)
  - [`_CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS`](#cs-posix-v7-lpbig-offbig-ldflags)
  - [`_CS_POSIX_V7_LPBIG_OFFBIG_LIBS`](#cs-posix-v7-lpbig-offbig-libs)
  - [`_CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS`](#cs-posix-v7-lpbig-offbig-lintflags)
  - [`RLIM_SAVED_MAX`](#rlim-saved-max)
  - [`RLIM_SAVED_CUR`](#rlim-saved-cur)
  - [`EI_NIDENT`](#ei-nident)
  - [`EI_MAG0`](#ei-mag0)
  - [`ELFMAG0`](#elfmag0)
  - [`EI_MAG1`](#ei-mag1)
  - [`ELFMAG1`](#elfmag1)
  - [`EI_MAG2`](#ei-mag2)
  - [`ELFMAG2`](#elfmag2)
  - [`EI_MAG3`](#ei-mag3)
  - [`ELFMAG3`](#elfmag3)
  - [`SELFMAG`](#selfmag)
  - [`EI_CLASS`](#ei-class)
  - [`ELFCLASSNONE`](#elfclassnone)
  - [`ELFCLASS32`](#elfclass32)
  - [`ELFCLASS64`](#elfclass64)
  - [`ELFCLASSNUM`](#elfclassnum)
  - [`EI_DATA`](#ei-data)
  - [`ELFDATANONE`](#elfdatanone)
  - [`ELFDATA2LSB`](#elfdata2lsb)
  - [`ELFDATA2MSB`](#elfdata2msb)
  - [`ELFDATANUM`](#elfdatanum)
  - [`EI_VERSION`](#ei-version)
  - [`EI_OSABI`](#ei-osabi)
  - [`ELFOSABI_NONE`](#elfosabi-none)
  - [`ELFOSABI_SYSV`](#elfosabi-sysv)
  - [`ELFOSABI_HPUX`](#elfosabi-hpux)
  - [`ELFOSABI_NETBSD`](#elfosabi-netbsd)
  - [`ELFOSABI_GNU`](#elfosabi-gnu)
  - [`ELFOSABI_LINUX`](#elfosabi-linux)
  - [`ELFOSABI_SOLARIS`](#elfosabi-solaris)
  - [`ELFOSABI_AIX`](#elfosabi-aix)
  - [`ELFOSABI_IRIX`](#elfosabi-irix)
  - [`ELFOSABI_FREEBSD`](#elfosabi-freebsd)
  - [`ELFOSABI_TRU64`](#elfosabi-tru64)
  - [`ELFOSABI_MODESTO`](#elfosabi-modesto)
  - [`ELFOSABI_OPENBSD`](#elfosabi-openbsd)
  - [`ELFOSABI_ARM`](#elfosabi-arm)
  - [`ELFOSABI_STANDALONE`](#elfosabi-standalone)
  - [`EI_ABIVERSION`](#ei-abiversion)
  - [`EI_PAD`](#ei-pad)
  - [`ET_NONE`](#et-none)
  - [`ET_REL`](#et-rel)
  - [`ET_EXEC`](#et-exec)
  - [`ET_DYN`](#et-dyn)
  - [`ET_CORE`](#et-core)
  - [`ET_NUM`](#et-num)
  - [`ET_LOOS`](#et-loos)
  - [`ET_HIOS`](#et-hios)
  - [`ET_LOPROC`](#et-loproc)
  - [`ET_HIPROC`](#et-hiproc)
  - [`EM_NONE`](#em-none)
  - [`EM_M32`](#em-m32)
  - [`EM_SPARC`](#em-sparc)
  - [`EM_386`](#em-386)
  - [`EM_68K`](#em-68k)
  - [`EM_88K`](#em-88k)
  - [`EM_860`](#em-860)
  - [`EM_MIPS`](#em-mips)
  - [`EM_S370`](#em-s370)
  - [`EM_MIPS_RS3_LE`](#em-mips-rs3-le)
  - [`EM_PARISC`](#em-parisc)
  - [`EM_VPP500`](#em-vpp500)
  - [`EM_SPARC32PLUS`](#em-sparc32plus)
  - [`EM_960`](#em-960)
  - [`EM_PPC`](#em-ppc)
  - [`EM_PPC64`](#em-ppc64)
  - [`EM_S390`](#em-s390)
  - [`EM_V800`](#em-v800)
  - [`EM_FR20`](#em-fr20)
  - [`EM_RH32`](#em-rh32)
  - [`EM_RCE`](#em-rce)
  - [`EM_ARM`](#em-arm)
  - [`EM_FAKE_ALPHA`](#em-fake-alpha)
  - [`EM_SH`](#em-sh)
  - [`EM_SPARCV9`](#em-sparcv9)
  - [`EM_TRICORE`](#em-tricore)
  - [`EM_ARC`](#em-arc)
  - [`EM_H8_300`](#em-h8-300)
  - [`EM_H8_300H`](#em-h8-300h)
  - [`EM_H8S`](#em-h8s)
  - [`EM_H8_500`](#em-h8-500)
  - [`EM_IA_64`](#em-ia-64)
  - [`EM_MIPS_X`](#em-mips-x)
  - [`EM_COLDFIRE`](#em-coldfire)
  - [`EM_68HC12`](#em-68hc12)
  - [`EM_MMA`](#em-mma)
  - [`EM_PCP`](#em-pcp)
  - [`EM_NCPU`](#em-ncpu)
  - [`EM_NDR1`](#em-ndr1)
  - [`EM_STARCORE`](#em-starcore)
  - [`EM_ME16`](#em-me16)
  - [`EM_ST100`](#em-st100)
  - [`EM_TINYJ`](#em-tinyj)
  - [`EM_X86_64`](#em-x86-64)
  - [`EM_PDSP`](#em-pdsp)
  - [`EM_FX66`](#em-fx66)
  - [`EM_ST9PLUS`](#em-st9plus)
  - [`EM_ST7`](#em-st7)
  - [`EM_68HC16`](#em-68hc16)
  - [`EM_68HC11`](#em-68hc11)
  - [`EM_68HC08`](#em-68hc08)
  - [`EM_68HC05`](#em-68hc05)
  - [`EM_SVX`](#em-svx)
  - [`EM_ST19`](#em-st19)
  - [`EM_VAX`](#em-vax)
  - [`EM_CRIS`](#em-cris)
  - [`EM_JAVELIN`](#em-javelin)
  - [`EM_FIREPATH`](#em-firepath)
  - [`EM_ZSP`](#em-zsp)
  - [`EM_MMIX`](#em-mmix)
  - [`EM_HUANY`](#em-huany)
  - [`EM_PRISM`](#em-prism)
  - [`EM_AVR`](#em-avr)
  - [`EM_FR30`](#em-fr30)
  - [`EM_D10V`](#em-d10v)
  - [`EM_D30V`](#em-d30v)
  - [`EM_V850`](#em-v850)
  - [`EM_M32R`](#em-m32r)
  - [`EM_MN10300`](#em-mn10300)
  - [`EM_MN10200`](#em-mn10200)
  - [`EM_PJ`](#em-pj)
  - [`EM_OPENRISC`](#em-openrisc)
  - [`EM_ARC_A5`](#em-arc-a5)
  - [`EM_XTENSA`](#em-xtensa)
  - [`EM_AARCH64`](#em-aarch64)
  - [`EM_TILEPRO`](#em-tilepro)
  - [`EM_TILEGX`](#em-tilegx)
  - [`EM_RISCV`](#em-riscv)
  - [`EM_ALPHA`](#em-alpha)
  - [`EV_NONE`](#ev-none)
  - [`EV_CURRENT`](#ev-current)
  - [`EV_NUM`](#ev-num)
  - [`PT_NULL`](#pt-null)
  - [`PT_LOAD`](#pt-load)
  - [`PT_DYNAMIC`](#pt-dynamic)
  - [`PT_INTERP`](#pt-interp)
  - [`PT_NOTE`](#pt-note)
  - [`PT_SHLIB`](#pt-shlib)
  - [`PT_PHDR`](#pt-phdr)
  - [`PT_TLS`](#pt-tls)
  - [`PT_NUM`](#pt-num)
  - [`PT_LOOS`](#pt-loos)
  - [`PT_GNU_EH_FRAME`](#pt-gnu-eh-frame)
  - [`PT_GNU_STACK`](#pt-gnu-stack)
  - [`PT_GNU_RELRO`](#pt-gnu-relro)
  - [`PT_LOSUNW`](#pt-losunw)
  - [`PT_SUNWBSS`](#pt-sunwbss)
  - [`PT_SUNWSTACK`](#pt-sunwstack)
  - [`PT_HISUNW`](#pt-hisunw)
  - [`PT_HIOS`](#pt-hios)
  - [`PT_LOPROC`](#pt-loproc)
  - [`PT_HIPROC`](#pt-hiproc)
  - [`PF_X`](#pf-x)
  - [`PF_W`](#pf-w)
  - [`PF_R`](#pf-r)
  - [`PF_MASKOS`](#pf-maskos)
  - [`PF_MASKPROC`](#pf-maskproc)
  - [`AT_NULL`](#at-null)
  - [`AT_IGNORE`](#at-ignore)
  - [`AT_EXECFD`](#at-execfd)
  - [`AT_PHDR`](#at-phdr)
  - [`AT_PHENT`](#at-phent)
  - [`AT_PHNUM`](#at-phnum)
  - [`AT_PAGESZ`](#at-pagesz)
  - [`AT_BASE`](#at-base)
  - [`AT_FLAGS`](#at-flags)
  - [`AT_ENTRY`](#at-entry)
  - [`AT_NOTELF`](#at-notelf)
  - [`AT_UID`](#at-uid)
  - [`AT_EUID`](#at-euid)
  - [`AT_GID`](#at-gid)
  - [`AT_EGID`](#at-egid)
  - [`AT_PLATFORM`](#at-platform)
  - [`AT_HWCAP`](#at-hwcap)
  - [`AT_CLKTCK`](#at-clktck)
  - [`AT_SECURE`](#at-secure)
  - [`AT_BASE_PLATFORM`](#at-base-platform)
  - [`AT_RANDOM`](#at-random)
  - [`AT_HWCAP2`](#at-hwcap2)
  - [`AT_HWCAP3`](#at-hwcap3)
  - [`AT_HWCAP4`](#at-hwcap4)
  - [`AT_EXECFN`](#at-execfn)
  - [`AT_SYSINFO_EHDR`](#at-sysinfo-ehdr)
  - [`AT_MINSIGSTKSZ`](#at-minsigstksz)
  - [`GLOB_ERR`](#glob-err)
  - [`GLOB_MARK`](#glob-mark)
  - [`GLOB_NOSORT`](#glob-nosort)
  - [`GLOB_DOOFFS`](#glob-dooffs)
  - [`GLOB_NOCHECK`](#glob-nocheck)
  - [`GLOB_APPEND`](#glob-append)
  - [`GLOB_NOESCAPE`](#glob-noescape)
  - [`GLOB_NOSPACE`](#glob-nospace)
  - [`GLOB_ABORTED`](#glob-aborted)
  - [`GLOB_NOMATCH`](#glob-nomatch)
  - [`POSIX_MADV_NORMAL`](#posix-madv-normal)
  - [`POSIX_MADV_RANDOM`](#posix-madv-random)
  - [`POSIX_MADV_SEQUENTIAL`](#posix-madv-sequential)
  - [`POSIX_MADV_WILLNEED`](#posix-madv-willneed)
  - [`POSIX_MADV_DONTNEED`](#posix-madv-dontneed)
  - [`S_IEXEC`](#s-iexec)
  - [`S_IWRITE`](#s-iwrite)
  - [`S_IREAD`](#s-iread)
  - [`F_LOCK`](#f-lock)
  - [`F_TEST`](#f-test)
  - [`F_TLOCK`](#f-tlock)
  - [`F_ULOCK`](#f-ulock)
  - [`ST_RDONLY`](#st-rdonly)
  - [`ST_NOSUID`](#st-nosuid)
  - [`ST_NODEV`](#st-nodev)
  - [`ST_NOEXEC`](#st-noexec)
  - [`ST_SYNCHRONOUS`](#st-synchronous)
  - [`ST_MANDLOCK`](#st-mandlock)
  - [`ST_WRITE`](#st-write)
  - [`ST_APPEND`](#st-append)
  - [`ST_IMMUTABLE`](#st-immutable)
  - [`ST_NOATIME`](#st-noatime)
  - [`ST_NODIRATIME`](#st-nodiratime)
  - [`RTLD_NEXT`](#rtld-next)
  - [`RTLD_DEFAULT`](#rtld-default)
  - [`RTLD_NODELETE`](#rtld-nodelete)
  - [`RTLD_NOW`](#rtld-now)
  - [`AT_EACCESS`](#at-eaccess)
  - [`PTHREAD_BARRIER_SERIAL_THREAD`](#pthread-barrier-serial-thread)
  - [`PTHREAD_ONCE_INIT`](#pthread-once-init)
  - [`PTHREAD_MUTEX_NORMAL`](#pthread-mutex-normal)
  - [`PTHREAD_MUTEX_RECURSIVE`](#pthread-mutex-recursive)
  - [`PTHREAD_MUTEX_ERRORCHECK`](#pthread-mutex-errorcheck)
  - [`PTHREAD_MUTEX_DEFAULT`](#pthread-mutex-default)
  - [`PTHREAD_MUTEX_STALLED`](#pthread-mutex-stalled)
  - [`PTHREAD_MUTEX_ROBUST`](#pthread-mutex-robust)
  - [`PTHREAD_PRIO_NONE`](#pthread-prio-none)
  - [`PTHREAD_PRIO_INHERIT`](#pthread-prio-inherit)
  - [`PTHREAD_PRIO_PROTECT`](#pthread-prio-protect)
  - [`PTHREAD_PROCESS_PRIVATE`](#pthread-process-private)
  - [`PTHREAD_PROCESS_SHARED`](#pthread-process-shared)
  - [`PTHREAD_INHERIT_SCHED`](#pthread-inherit-sched)
  - [`PTHREAD_EXPLICIT_SCHED`](#pthread-explicit-sched)
  - [`__SIZEOF_PTHREAD_COND_T`](#sizeof-pthread-cond-t)
  - [`IPPROTO_MAX`](#ipproto-max)
  - [`IPC_PRIVATE`](#ipc-private)
  - [`IPC_CREAT`](#ipc-creat)
  - [`IPC_EXCL`](#ipc-excl)
  - [`IPC_NOWAIT`](#ipc-nowait)
  - [`IPC_RMID`](#ipc-rmid)
  - [`IPC_SET`](#ipc-set)
  - [`IPC_STAT`](#ipc-stat)
  - [`IPC_INFO`](#ipc-info)
  - [`SHM_R`](#shm-r)
  - [`SHM_W`](#shm-w)
  - [`SHM_RDONLY`](#shm-rdonly)
  - [`SHM_RND`](#shm-rnd)
  - [`SHM_REMAP`](#shm-remap)
  - [`SHM_LOCK`](#shm-lock)
  - [`SHM_UNLOCK`](#shm-unlock)
  - [`SHM_HUGETLB`](#shm-hugetlb)
  - [`SHM_NORESERVE`](#shm-noreserve)
  - [`LOG_NFACILITIES`](#log-nfacilities)
  - [`SEM_FAILED`](#sem-failed)
  - [`AI_PASSIVE`](#ai-passive)
  - [`AI_CANONNAME`](#ai-canonname)
  - [`AI_NUMERICHOST`](#ai-numerichost)
  - [`AI_V4MAPPED`](#ai-v4mapped)
  - [`AI_ALL`](#ai-all)
  - [`AI_ADDRCONFIG`](#ai-addrconfig)
  - [`AI_NUMERICSERV`](#ai-numericserv)
  - [`EAI_BADFLAGS`](#eai-badflags)
  - [`EAI_NONAME`](#eai-noname)
  - [`EAI_AGAIN`](#eai-again)
  - [`EAI_FAIL`](#eai-fail)
  - [`EAI_NODATA`](#eai-nodata)
  - [`EAI_FAMILY`](#eai-family)
  - [`EAI_SOCKTYPE`](#eai-socktype)
  - [`EAI_SERVICE`](#eai-service)
  - [`EAI_MEMORY`](#eai-memory)
  - [`EAI_SYSTEM`](#eai-system)
  - [`EAI_OVERFLOW`](#eai-overflow)
  - [`NI_NUMERICHOST`](#ni-numerichost)
  - [`NI_NUMERICSERV`](#ni-numericserv)
  - [`NI_NOFQDN`](#ni-nofqdn)
  - [`NI_NAMEREQD`](#ni-namereqd)
  - [`NI_DGRAM`](#ni-dgram)
  - [`NI_IDN`](#ni-idn)
  - [`AIO_CANCELED`](#aio-canceled)
  - [`AIO_NOTCANCELED`](#aio-notcanceled)
  - [`AIO_ALLDONE`](#aio-alldone)
  - [`LIO_READ`](#lio-read)
  - [`LIO_WRITE`](#lio-write)
  - [`LIO_NOP`](#lio-nop)
  - [`LIO_WAIT`](#lio-wait)
  - [`LIO_NOWAIT`](#lio-nowait)
  - [`RUSAGE_THREAD`](#rusage-thread)
  - [`MSG_COPY`](#msg-copy)
  - [`SHM_EXEC`](#shm-exec)
  - [`IPV6_MULTICAST_ALL`](#ipv6-multicast-all)
  - [`IPV6_ROUTER_ALERT_ISOLATE`](#ipv6-router-alert-isolate)
  - [`PACKET_MR_UNICAST`](#packet-mr-unicast)
  - [`PTRACE_EVENT_STOP`](#ptrace-event-stop)
  - [`UDP_SEGMENT`](#udp-segment)
  - [`UDP_GRO`](#udp-gro)
  - [`PR_SET_PDEATHSIG`](#pr-set-pdeathsig)
  - [`PR_GET_PDEATHSIG`](#pr-get-pdeathsig)
  - [`PR_GET_DUMPABLE`](#pr-get-dumpable)
  - [`PR_SET_DUMPABLE`](#pr-set-dumpable)
  - [`PR_GET_UNALIGN`](#pr-get-unalign)
  - [`PR_SET_UNALIGN`](#pr-set-unalign)
  - [`PR_UNALIGN_NOPRINT`](#pr-unalign-noprint)
  - [`PR_UNALIGN_SIGBUS`](#pr-unalign-sigbus)
  - [`PR_GET_KEEPCAPS`](#pr-get-keepcaps)
  - [`PR_SET_KEEPCAPS`](#pr-set-keepcaps)
  - [`PR_GET_FPEMU`](#pr-get-fpemu)
  - [`PR_SET_FPEMU`](#pr-set-fpemu)
  - [`PR_FPEMU_NOPRINT`](#pr-fpemu-noprint)
  - [`PR_FPEMU_SIGFPE`](#pr-fpemu-sigfpe)
  - [`PR_GET_FPEXC`](#pr-get-fpexc)
  - [`PR_SET_FPEXC`](#pr-set-fpexc)
  - [`PR_FP_EXC_SW_ENABLE`](#pr-fp-exc-sw-enable)
  - [`PR_FP_EXC_DIV`](#pr-fp-exc-div)
  - [`PR_FP_EXC_OVF`](#pr-fp-exc-ovf)
  - [`PR_FP_EXC_UND`](#pr-fp-exc-und)
  - [`PR_FP_EXC_RES`](#pr-fp-exc-res)
  - [`PR_FP_EXC_INV`](#pr-fp-exc-inv)
  - [`PR_FP_EXC_DISABLED`](#pr-fp-exc-disabled)
  - [`PR_FP_EXC_NONRECOV`](#pr-fp-exc-nonrecov)
  - [`PR_FP_EXC_ASYNC`](#pr-fp-exc-async)
  - [`PR_FP_EXC_PRECISE`](#pr-fp-exc-precise)
  - [`PR_GET_TIMING`](#pr-get-timing)
  - [`PR_SET_TIMING`](#pr-set-timing)
  - [`PR_TIMING_STATISTICAL`](#pr-timing-statistical)
  - [`PR_TIMING_TIMESTAMP`](#pr-timing-timestamp)
  - [`PR_SET_NAME`](#pr-set-name)
  - [`PR_GET_NAME`](#pr-get-name)
  - [`PR_GET_ENDIAN`](#pr-get-endian)
  - [`PR_SET_ENDIAN`](#pr-set-endian)
  - [`PR_ENDIAN_BIG`](#pr-endian-big)
  - [`PR_ENDIAN_LITTLE`](#pr-endian-little)
  - [`PR_ENDIAN_PPC_LITTLE`](#pr-endian-ppc-little)
  - [`PR_GET_SECCOMP`](#pr-get-seccomp)
  - [`PR_SET_SECCOMP`](#pr-set-seccomp)
  - [`PR_CAPBSET_READ`](#pr-capbset-read)
  - [`PR_CAPBSET_DROP`](#pr-capbset-drop)
  - [`PR_GET_TSC`](#pr-get-tsc)
  - [`PR_SET_TSC`](#pr-set-tsc)
  - [`PR_TSC_ENABLE`](#pr-tsc-enable)
  - [`PR_TSC_SIGSEGV`](#pr-tsc-sigsegv)
  - [`PR_GET_SECUREBITS`](#pr-get-securebits)
  - [`PR_SET_SECUREBITS`](#pr-set-securebits)
  - [`PR_SET_TIMERSLACK`](#pr-set-timerslack)
  - [`PR_GET_TIMERSLACK`](#pr-get-timerslack)
  - [`PR_TASK_PERF_EVENTS_DISABLE`](#pr-task-perf-events-disable)
  - [`PR_TASK_PERF_EVENTS_ENABLE`](#pr-task-perf-events-enable)
  - [`PR_MCE_KILL`](#pr-mce-kill)
  - [`PR_MCE_KILL_CLEAR`](#pr-mce-kill-clear)
  - [`PR_MCE_KILL_SET`](#pr-mce-kill-set)
  - [`PR_MCE_KILL_LATE`](#pr-mce-kill-late)
  - [`PR_MCE_KILL_EARLY`](#pr-mce-kill-early)
  - [`PR_MCE_KILL_DEFAULT`](#pr-mce-kill-default)
  - [`PR_MCE_KILL_GET`](#pr-mce-kill-get)
  - [`PR_SET_MM`](#pr-set-mm)
  - [`PR_SET_MM_START_CODE`](#pr-set-mm-start-code)
  - [`PR_SET_MM_END_CODE`](#pr-set-mm-end-code)
  - [`PR_SET_MM_START_DATA`](#pr-set-mm-start-data)
  - [`PR_SET_MM_END_DATA`](#pr-set-mm-end-data)
  - [`PR_SET_MM_START_STACK`](#pr-set-mm-start-stack)
  - [`PR_SET_MM_START_BRK`](#pr-set-mm-start-brk)
  - [`PR_SET_MM_BRK`](#pr-set-mm-brk)
  - [`PR_SET_MM_ARG_START`](#pr-set-mm-arg-start)
  - [`PR_SET_MM_ARG_END`](#pr-set-mm-arg-end)
  - [`PR_SET_MM_ENV_START`](#pr-set-mm-env-start)
  - [`PR_SET_MM_ENV_END`](#pr-set-mm-env-end)
  - [`PR_SET_MM_AUXV`](#pr-set-mm-auxv)
  - [`PR_SET_MM_EXE_FILE`](#pr-set-mm-exe-file)
  - [`PR_SET_MM_MAP`](#pr-set-mm-map)
  - [`PR_SET_MM_MAP_SIZE`](#pr-set-mm-map-size)
  - [`PR_SET_PTRACER`](#pr-set-ptracer)
  - [`PR_SET_PTRACER_ANY`](#pr-set-ptracer-any)
  - [`PR_SET_CHILD_SUBREAPER`](#pr-set-child-subreaper)
  - [`PR_GET_CHILD_SUBREAPER`](#pr-get-child-subreaper)
  - [`PR_SET_NO_NEW_PRIVS`](#pr-set-no-new-privs)
  - [`PR_GET_NO_NEW_PRIVS`](#pr-get-no-new-privs)
  - [`PR_GET_TID_ADDRESS`](#pr-get-tid-address)
  - [`PR_SET_THP_DISABLE`](#pr-set-thp-disable)
  - [`PR_GET_THP_DISABLE`](#pr-get-thp-disable)
  - [`PR_MPX_ENABLE_MANAGEMENT`](#pr-mpx-enable-management)
  - [`PR_MPX_DISABLE_MANAGEMENT`](#pr-mpx-disable-management)
  - [`PR_SET_FP_MODE`](#pr-set-fp-mode)
  - [`PR_GET_FP_MODE`](#pr-get-fp-mode)
  - [`PR_FP_MODE_FR`](#pr-fp-mode-fr)
  - [`PR_FP_MODE_FRE`](#pr-fp-mode-fre)
  - [`PR_CAP_AMBIENT`](#pr-cap-ambient)
  - [`PR_CAP_AMBIENT_IS_SET`](#pr-cap-ambient-is-set)
  - [`PR_CAP_AMBIENT_RAISE`](#pr-cap-ambient-raise)
  - [`PR_CAP_AMBIENT_LOWER`](#pr-cap-ambient-lower)
  - [`PR_CAP_AMBIENT_CLEAR_ALL`](#pr-cap-ambient-clear-all)
  - [`PR_SET_VMA`](#pr-set-vma)
  - [`PR_SET_VMA_ANON_NAME`](#pr-set-vma-anon-name)
  - [`PR_SCHED_CORE`](#pr-sched-core)
  - [`PR_SCHED_CORE_GET`](#pr-sched-core-get)
  - [`PR_SCHED_CORE_CREATE`](#pr-sched-core-create)
  - [`PR_SCHED_CORE_SHARE_TO`](#pr-sched-core-share-to)
  - [`PR_SCHED_CORE_SHARE_FROM`](#pr-sched-core-share-from)
  - [`PR_SCHED_CORE_MAX`](#pr-sched-core-max)
  - [`PR_SCHED_CORE_SCOPE_THREAD`](#pr-sched-core-scope-thread)
  - [`PR_SCHED_CORE_SCOPE_THREAD_GROUP`](#pr-sched-core-scope-thread-group)
  - [`PR_SCHED_CORE_SCOPE_PROCESS_GROUP`](#pr-sched-core-scope-process-group)
  - [`ITIMER_REAL`](#itimer-real)
  - [`ITIMER_VIRTUAL`](#itimer-virtual)
  - [`ITIMER_PROF`](#itimer-prof)
  - [`_POSIX_VDISABLE`](#posix-vdisable)
  - [`IPV6_RTHDR_LOOSE`](#ipv6-rthdr-loose)
  - [`IPV6_RTHDR_STRICT`](#ipv6-rthdr-strict)
  - [`IUTF8`](#iutf8)
  - [`CMSPAR`](#cmspar)
  - [`MFD_CLOEXEC`](#mfd-cloexec)
  - [`MFD_ALLOW_SEALING`](#mfd-allow-sealing)
  - [`MFD_HUGETLB`](#mfd-hugetlb)
  - [`MFD_NOEXEC_SEAL`](#mfd-noexec-seal)
  - [`MFD_EXEC`](#mfd-exec)
  - [`MFD_HUGE_64KB`](#mfd-huge-64kb)
  - [`MFD_HUGE_512KB`](#mfd-huge-512kb)
  - [`MFD_HUGE_1MB`](#mfd-huge-1mb)
  - [`MFD_HUGE_2MB`](#mfd-huge-2mb)
  - [`MFD_HUGE_8MB`](#mfd-huge-8mb)
  - [`MFD_HUGE_16MB`](#mfd-huge-16mb)
  - [`MFD_HUGE_32MB`](#mfd-huge-32mb)
  - [`MFD_HUGE_256MB`](#mfd-huge-256mb)
  - [`MFD_HUGE_512MB`](#mfd-huge-512mb)
  - [`MFD_HUGE_1GB`](#mfd-huge-1gb)
  - [`MFD_HUGE_2GB`](#mfd-huge-2gb)
  - [`MFD_HUGE_16GB`](#mfd-huge-16gb)
  - [`MFD_HUGE_MASK`](#mfd-huge-mask)
  - [`MFD_HUGE_SHIFT`](#mfd-huge-shift)
  - [`PACKET_HOST`](#packet-host)
  - [`PACKET_BROADCAST`](#packet-broadcast)
  - [`PACKET_MULTICAST`](#packet-multicast)
  - [`PACKET_OTHERHOST`](#packet-otherhost)
  - [`PACKET_OUTGOING`](#packet-outgoing)
  - [`PACKET_LOOPBACK`](#packet-loopback)
  - [`PACKET_USER`](#packet-user)
  - [`PACKET_KERNEL`](#packet-kernel)
  - [`PACKET_ADD_MEMBERSHIP`](#packet-add-membership)
  - [`PACKET_DROP_MEMBERSHIP`](#packet-drop-membership)
  - [`PACKET_RECV_OUTPUT`](#packet-recv-output)
  - [`PACKET_RX_RING`](#packet-rx-ring)
  - [`PACKET_STATISTICS`](#packet-statistics)
  - [`PACKET_COPY_THRESH`](#packet-copy-thresh)
  - [`PACKET_AUXDATA`](#packet-auxdata)
  - [`PACKET_ORIGDEV`](#packet-origdev)
  - [`PACKET_VERSION`](#packet-version)
  - [`PACKET_HDRLEN`](#packet-hdrlen)
  - [`PACKET_RESERVE`](#packet-reserve)
  - [`PACKET_TX_RING`](#packet-tx-ring)
  - [`PACKET_LOSS`](#packet-loss)
  - [`PACKET_VNET_HDR`](#packet-vnet-hdr)
  - [`PACKET_TX_TIMESTAMP`](#packet-tx-timestamp)
  - [`PACKET_TIMESTAMP`](#packet-timestamp)
  - [`PACKET_MR_MULTICAST`](#packet-mr-multicast)
  - [`PACKET_MR_PROMISC`](#packet-mr-promisc)
  - [`PACKET_MR_ALLMULTI`](#packet-mr-allmulti)
  - [`SIOCADDRT`](#siocaddrt)
  - [`SIOCDELRT`](#siocdelrt)
  - [`SIOCGIFNAME`](#siocgifname)
  - [`SIOCSIFLINK`](#siocsiflink)
  - [`SIOCGIFCONF`](#siocgifconf)
  - [`SIOCGIFFLAGS`](#siocgifflags)
  - [`SIOCSIFFLAGS`](#siocsifflags)
  - [`SIOCGIFADDR`](#siocgifaddr)
  - [`SIOCSIFADDR`](#siocsifaddr)
  - [`SIOCGIFDSTADDR`](#siocgifdstaddr)
  - [`SIOCSIFDSTADDR`](#siocsifdstaddr)
  - [`SIOCGIFBRDADDR`](#siocgifbrdaddr)
  - [`SIOCSIFBRDADDR`](#siocsifbrdaddr)
  - [`SIOCGIFNETMASK`](#siocgifnetmask)
  - [`SIOCSIFNETMASK`](#siocsifnetmask)
  - [`SIOCGIFMETRIC`](#siocgifmetric)
  - [`SIOCSIFMETRIC`](#siocsifmetric)
  - [`SIOCGIFMEM`](#siocgifmem)
  - [`SIOCSIFMEM`](#siocsifmem)
  - [`SIOCGIFMTU`](#siocgifmtu)
  - [`SIOCSIFMTU`](#siocsifmtu)
  - [`SIOCSIFNAME`](#siocsifname)
  - [`SIOCSIFHWADDR`](#siocsifhwaddr)
  - [`SIOCGIFENCAP`](#siocgifencap)
  - [`SIOCSIFENCAP`](#siocsifencap)
  - [`SIOCGIFHWADDR`](#siocgifhwaddr)
  - [`SIOCGIFSLAVE`](#siocgifslave)
  - [`SIOCSIFSLAVE`](#siocsifslave)
  - [`SIOCADDMULTI`](#siocaddmulti)
  - [`SIOCDELMULTI`](#siocdelmulti)
  - [`SIOCGIFINDEX`](#siocgifindex)
  - [`SIOGIFINDEX`](#siogifindex)
  - [`SIOCSIFPFLAGS`](#siocsifpflags)
  - [`SIOCGIFPFLAGS`](#siocgifpflags)
  - [`SIOCDIFADDR`](#siocdifaddr)
  - [`SIOCSIFHWBROADCAST`](#siocsifhwbroadcast)
  - [`SIOCGIFCOUNT`](#siocgifcount)
  - [`SIOCGIFBR`](#siocgifbr)
  - [`SIOCSIFBR`](#siocsifbr)
  - [`SIOCGIFTXQLEN`](#siocgiftxqlen)
  - [`SIOCSIFTXQLEN`](#siocsiftxqlen)
  - [`SIOCETHTOOL`](#siocethtool)
  - [`SIOCGMIIPHY`](#siocgmiiphy)
  - [`SIOCGMIIREG`](#siocgmiireg)
  - [`SIOCSMIIREG`](#siocsmiireg)
  - [`SIOCWANDEV`](#siocwandev)
  - [`SIOCOUTQNSD`](#siocoutqnsd)
  - [`SIOCGSKNS`](#siocgskns)
  - [`SIOCDARP`](#siocdarp)
  - [`SIOCGARP`](#siocgarp)
  - [`SIOCSARP`](#siocsarp)
  - [`SIOCDRARP`](#siocdrarp)
  - [`SIOCGRARP`](#siocgrarp)
  - [`SIOCSRARP`](#siocsrarp)
  - [`SIOCGIFMAP`](#siocgifmap)
  - [`SIOCSIFMAP`](#siocsifmap)
  - [`IPTOS_TOS_MASK`](#iptos-tos-mask)
  - [`IPTOS_PREC_MASK`](#iptos-prec-mask)
  - [`IPTOS_ECN_NOT_ECT`](#iptos-ecn-not-ect)
  - [`RTF_UP`](#rtf-up)
  - [`RTF_GATEWAY`](#rtf-gateway)
  - [`RTF_HOST`](#rtf-host)
  - [`RTF_REINSTATE`](#rtf-reinstate)
  - [`RTF_DYNAMIC`](#rtf-dynamic)
  - [`RTF_MODIFIED`](#rtf-modified)
  - [`RTF_MTU`](#rtf-mtu)
  - [`RTF_MSS`](#rtf-mss)
  - [`RTF_WINDOW`](#rtf-window)
  - [`RTF_IRTT`](#rtf-irtt)
  - [`RTF_REJECT`](#rtf-reject)
  - [`RTF_STATIC`](#rtf-static)
  - [`RTF_XRESOLVE`](#rtf-xresolve)
  - [`RTF_NOFORWARD`](#rtf-noforward)
  - [`RTF_THROW`](#rtf-throw)
  - [`RTF_NOPMTUDISC`](#rtf-nopmtudisc)
  - [`RTF_DEFAULT`](#rtf-default)
  - [`RTF_ALLONLINK`](#rtf-allonlink)
  - [`RTF_ADDRCONF`](#rtf-addrconf)
  - [`RTF_LINKRT`](#rtf-linkrt)
  - [`RTF_NONEXTHOP`](#rtf-nonexthop)
  - [`RTF_CACHE`](#rtf-cache)
  - [`RTF_FLOW`](#rtf-flow)
  - [`RTF_POLICY`](#rtf-policy)
  - [`RTCF_VALVE`](#rtcf-valve)
  - [`RTCF_MASQ`](#rtcf-masq)
  - [`RTCF_NAT`](#rtcf-nat)
  - [`RTCF_DOREDIRECT`](#rtcf-doredirect)
  - [`RTCF_LOG`](#rtcf-log)
  - [`RTCF_DIRECTSRC`](#rtcf-directsrc)
  - [`RTF_LOCAL`](#rtf-local)
  - [`RTF_INTERFACE`](#rtf-interface)
  - [`RTF_MULTICAST`](#rtf-multicast)
  - [`RTF_BROADCAST`](#rtf-broadcast)
  - [`RTF_NAT`](#rtf-nat)
  - [`RTF_ADDRCLASSMASK`](#rtf-addrclassmask)
  - [`RT_CLASS_UNSPEC`](#rt-class-unspec)
  - [`RT_CLASS_DEFAULT`](#rt-class-default)
  - [`RT_CLASS_MAIN`](#rt-class-main)
  - [`RT_CLASS_LOCAL`](#rt-class-local)
  - [`RT_CLASS_MAX`](#rt-class-max)
  - [`MAX_ADDR_LEN`](#max-addr-len)
  - [`ARPD_UPDATE`](#arpd-update)
  - [`ARPD_LOOKUP`](#arpd-lookup)
  - [`ARPD_FLUSH`](#arpd-flush)
  - [`ATF_MAGIC`](#atf-magic)
  - [`UDP_CORK`](#udp-cork)
  - [`UDP_ENCAP`](#udp-encap)
  - [`UDP_NO_CHECK6_TX`](#udp-no-check6-tx)
  - [`UDP_NO_CHECK6_RX`](#udp-no-check6-rx)
  - [`MAP_FIXED_NOREPLACE`](#map-fixed-noreplace)
  - [`MLOCK_ONFAULT`](#mlock-onfault)
  - [`REG_EXTENDED`](#reg-extended)
  - [`REG_ICASE`](#reg-icase)
  - [`REG_NEWLINE`](#reg-newline)
  - [`REG_NOSUB`](#reg-nosub)
  - [`REG_NOTBOL`](#reg-notbol)
  - [`REG_NOTEOL`](#reg-noteol)
  - [`REG_ENOSYS`](#reg-enosys)
  - [`REG_NOMATCH`](#reg-nomatch)
  - [`REG_BADPAT`](#reg-badpat)
  - [`REG_ECOLLATE`](#reg-ecollate)
  - [`REG_ECTYPE`](#reg-ectype)
  - [`REG_EESCAPE`](#reg-eescape)
  - [`REG_ESUBREG`](#reg-esubreg)
  - [`REG_EBRACK`](#reg-ebrack)
  - [`REG_EPAREN`](#reg-eparen)
  - [`REG_EBRACE`](#reg-ebrace)
  - [`REG_BADBR`](#reg-badbr)
  - [`REG_ERANGE`](#reg-erange)
  - [`REG_ESPACE`](#reg-espace)
  - [`REG_BADRPT`](#reg-badrpt)
  - [`EPERM`](#eperm)
  - [`ENOENT`](#enoent)
  - [`ESRCH`](#esrch)
  - [`EINTR`](#eintr)
  - [`EIO`](#eio)
  - [`ENXIO`](#enxio)
  - [`E2BIG`](#e2big)
  - [`ENOEXEC`](#enoexec)
  - [`EBADF`](#ebadf)
  - [`ECHILD`](#echild)
  - [`EAGAIN`](#eagain)
  - [`ENOMEM`](#enomem)
  - [`EACCES`](#eacces)
  - [`EFAULT`](#efault)
  - [`ENOTBLK`](#enotblk)
  - [`EBUSY`](#ebusy)
  - [`EEXIST`](#eexist)
  - [`EXDEV`](#exdev)
  - [`ENODEV`](#enodev)
  - [`ENOTDIR`](#enotdir)
  - [`EISDIR`](#eisdir)
  - [`EINVAL`](#einval)
  - [`ENFILE`](#enfile)
  - [`EMFILE`](#emfile)
  - [`ENOTTY`](#enotty)
  - [`ETXTBSY`](#etxtbsy)
  - [`EFBIG`](#efbig)
  - [`ENOSPC`](#enospc)
  - [`ESPIPE`](#espipe)
  - [`EROFS`](#erofs)
  - [`EMLINK`](#emlink)
  - [`EPIPE`](#epipe)
  - [`EDOM`](#edom)
  - [`ERANGE`](#erange)
  - [`EWOULDBLOCK`](#ewouldblock)
  - [`CSIGNAL`](#csignal)
  - [`SCHED_NORMAL`](#sched-normal)
  - [`SCHED_OTHER`](#sched-other)
  - [`SCHED_FIFO`](#sched-fifo)
  - [`SCHED_RR`](#sched-rr)
  - [`SCHED_BATCH`](#sched-batch)
  - [`SCHED_IDLE`](#sched-idle)
  - [`SCHED_DEADLINE`](#sched-deadline)
  - [`SCHED_RESET_ON_FORK`](#sched-reset-on-fork)
  - [`NT_PRSTATUS`](#nt-prstatus)
  - [`NT_PRFPREG`](#nt-prfpreg)
  - [`NT_FPREGSET`](#nt-fpregset)
  - [`NT_PRPSINFO`](#nt-prpsinfo)
  - [`NT_PRXREG`](#nt-prxreg)
  - [`NT_TASKSTRUCT`](#nt-taskstruct)
  - [`NT_PLATFORM`](#nt-platform)
  - [`NT_AUXV`](#nt-auxv)
  - [`NT_GWINDOWS`](#nt-gwindows)
  - [`NT_ASRS`](#nt-asrs)
  - [`NT_PSTATUS`](#nt-pstatus)
  - [`NT_PSINFO`](#nt-psinfo)
  - [`NT_PRCRED`](#nt-prcred)
  - [`NT_UTSNAME`](#nt-utsname)
  - [`NT_LWPSTATUS`](#nt-lwpstatus)
  - [`NT_LWPSINFO`](#nt-lwpsinfo)
  - [`NT_PRFPXREG`](#nt-prfpxreg)
  - [`MS_NOUSER`](#ms-nouser)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`linux`](#linux) | mod | Linux-specific definitions for linux-like values |
| [`linux_l4re_shared`](#linux-l4re-shared) | mod |  |
| [`gnu`](#gnu) | mod |  |
| [`arch`](#arch) | mod |  |
| [`in_addr`](#in-addr) | struct |  |
| [`ip_mreq`](#ip-mreq) | struct |  |
| [`ip_mreqn`](#ip-mreqn) | struct |  |
| [`ip_mreq_source`](#ip-mreq-source) | struct |  |
| [`sockaddr`](#sockaddr) | struct |  |
| [`sockaddr_in`](#sockaddr-in) | struct |  |
| [`sockaddr_in6`](#sockaddr-in6) | struct |  |
| [`addrinfo`](#addrinfo) | struct |  |
| [`sockaddr_ll`](#sockaddr-ll) | struct |  |
| [`fd_set`](#fd-set) | struct |  |
| [`tm`](#tm) | struct |  |
| [`sched_param`](#sched-param) | struct |  |
| [`Dl_info`](#dl-info) | struct |  |
| [`lconv`](#lconv) | struct |  |
| [`in_pktinfo`](#in-pktinfo) | struct |  |
| [`ifaddrs`](#ifaddrs) | struct |  |
| [`in6_rtmsg`](#in6-rtmsg) | struct |  |
| [`arpreq`](#arpreq) | struct |  |
| [`arpreq_old`](#arpreq-old) | struct |  |
| [`arphdr`](#arphdr) | struct |  |
| [`mmsghdr`](#mmsghdr) | struct |  |
| [`sockaddr_un`](#sockaddr-un) | struct |  |
| [`sockaddr_storage`](#sockaddr-storage) | struct |  |
| [`utsname`](#utsname) | struct |  |
| [`if_nameindex`](#if-nameindex) | struct |  |
| [`file_clone_range`](#file-clone-range) | struct |  |
| [`sock_filter`](#sock-filter) | struct |  |
| [`sock_fprog`](#sock-fprog) | struct |  |
| [`statx`](#statx) | struct |  |
| [`statx_timestamp`](#statx-timestamp) | struct |  |
| [`epoll_event`](#epoll-event) | struct |  |
| [`sigevent`](#sigevent) | struct |  |
| [`dqblk`](#dqblk) | struct |  |
| [`signalfd_siginfo`](#signalfd-siginfo) | struct |  |
| [`fanout_args`](#fanout-args) | struct |  |
| [`sockaddr_pkt`](#sockaddr-pkt) | struct |  |
| [`tpacket_auxdata`](#tpacket-auxdata) | struct |  |
| [`tpacket_hdr`](#tpacket-hdr) | struct |  |
| [`tpacket_hdr_variant1`](#tpacket-hdr-variant1) | struct |  |
| [`tpacket2_hdr`](#tpacket2-hdr) | struct |  |
| [`tpacket_req`](#tpacket-req) | struct |  |
| [`tpacket_req3`](#tpacket-req3) | struct |  |
| [`tpacket_rollover_stats`](#tpacket-rollover-stats) | struct |  |
| [`tpacket_stats`](#tpacket-stats) | struct |  |
| [`tpacket_stats_v3`](#tpacket-stats-v3) | struct |  |
| [`tpacket3_hdr`](#tpacket3-hdr) | struct |  |
| [`tpacket_bd_ts`](#tpacket-bd-ts) | struct |  |
| [`tpacket_hdr_v1`](#tpacket-hdr-v1) | struct |  |
| [`msginfo`](#msginfo) | struct |  |
| [`input_event`](#input-event) | struct |  |
| [`input_id`](#input-id) | struct |  |
| [`input_absinfo`](#input-absinfo) | struct |  |
| [`input_keymap_entry`](#input-keymap-entry) | struct |  |
| [`input_mask`](#input-mask) | struct |  |
| [`ff_replay`](#ff-replay) | struct |  |
| [`ff_trigger`](#ff-trigger) | struct |  |
| [`ff_envelope`](#ff-envelope) | struct |  |
| [`ff_constant_effect`](#ff-constant-effect) | struct |  |
| [`ff_ramp_effect`](#ff-ramp-effect) | struct |  |
| [`ff_condition_effect`](#ff-condition-effect) | struct |  |
| [`ff_periodic_effect`](#ff-periodic-effect) | struct |  |
| [`ff_rumble_effect`](#ff-rumble-effect) | struct |  |
| [`ff_effect`](#ff-effect) | struct |  |
| [`uinput_ff_upload`](#uinput-ff-upload) | struct |  |
| [`uinput_ff_erase`](#uinput-ff-erase) | struct |  |
| [`uinput_abs_setup`](#uinput-abs-setup) | struct |  |
| [`__c_anonymous__kernel_fsid_t`](#c-anonymous-kernel-fsid-t) | struct |  |
| [`posix_spawn_file_actions_t`](#posix-spawn-file-actions-t) | struct |  |
| [`posix_spawnattr_t`](#posix-spawnattr-t) | struct |  |
| [`genlmsghdr`](#genlmsghdr) | struct |  |
| [`inotify_event`](#inotify-event) | struct |  |
| [`fanotify_response`](#fanotify-response) | struct |  |
| [`fanotify_event_info_header`](#fanotify-event-info-header) | struct |  |
| [`fanotify_event_info_fid`](#fanotify-event-info-fid) | struct |  |
| [`sockaddr_vm`](#sockaddr-vm) | struct |  |
| [`sock_extended_err`](#sock-extended-err) | struct |  |
| [`seccomp_data`](#seccomp-data) | struct |  |
| [`seccomp_notif_sizes`](#seccomp-notif-sizes) | struct |  |
| [`seccomp_notif`](#seccomp-notif) | struct |  |
| [`seccomp_notif_resp`](#seccomp-notif-resp) | struct |  |
| [`seccomp_notif_addfd`](#seccomp-notif-addfd) | struct |  |
| [`in6_ifreq`](#in6-ifreq) | struct |  |
| [`open_how`](#open-how) | struct |  |
| [`ptp_clock_time`](#ptp-clock-time) | struct |  |
| [`ptp_extts_request`](#ptp-extts-request) | struct |  |
| [`ptp_sys_offset_extended`](#ptp-sys-offset-extended) | struct |  |
| [`ptp_sys_offset_precise`](#ptp-sys-offset-precise) | struct |  |
| [`ptp_extts_event`](#ptp-extts-event) | struct |  |
| [`sctp_initmsg`](#sctp-initmsg) | struct |  |
| [`sctp_sndrcvinfo`](#sctp-sndrcvinfo) | struct |  |
| [`sctp_sndinfo`](#sctp-sndinfo) | struct |  |
| [`sctp_rcvinfo`](#sctp-rcvinfo) | struct |  |
| [`sctp_nxtinfo`](#sctp-nxtinfo) | struct |  |
| [`sctp_prinfo`](#sctp-prinfo) | struct |  |
| [`sctp_authinfo`](#sctp-authinfo) | struct |  |
| [`tls_crypto_info`](#tls-crypto-info) | struct |  |
| [`tls12_crypto_info_aes_gcm_128`](#tls12-crypto-info-aes-gcm-128) | struct |  |
| [`tls12_crypto_info_aes_gcm_256`](#tls12-crypto-info-aes-gcm-256) | struct |  |
| [`tls12_crypto_info_aes_ccm_128`](#tls12-crypto-info-aes-ccm-128) | struct |  |
| [`tls12_crypto_info_chacha20_poly1305`](#tls12-crypto-info-chacha20-poly1305) | struct |  |
| [`tls12_crypto_info_sm4_gcm`](#tls12-crypto-info-sm4-gcm) | struct |  |
| [`tls12_crypto_info_sm4_ccm`](#tls12-crypto-info-sm4-ccm) | struct |  |
| [`tls12_crypto_info_aria_gcm_128`](#tls12-crypto-info-aria-gcm-128) | struct |  |
| [`tls12_crypto_info_aria_gcm_256`](#tls12-crypto-info-aria-gcm-256) | struct |  |
| [`iw_param`](#iw-param) | struct |  |
| [`iw_point`](#iw-point) | struct |  |
| [`iw_freq`](#iw-freq) | struct |  |
| [`iw_quality`](#iw-quality) | struct |  |
| [`iw_discarded`](#iw-discarded) | struct |  |
| [`iw_missed`](#iw-missed) | struct |  |
| [`iw_scan_req`](#iw-scan-req) | struct |  |
| [`iw_encode_ext`](#iw-encode-ext) | struct |  |
| [`iw_pmksa`](#iw-pmksa) | struct |  |
| [`iw_pmkid_cand`](#iw-pmkid-cand) | struct |  |
| [`iw_statistics`](#iw-statistics) | struct |  |
| [`iw_range`](#iw-range) | struct |  |
| [`iw_priv_args`](#iw-priv-args) | struct |  |
| [`epoll_params`](#epoll-params) | struct |  |
| [`pthread_mutexattr_t`](#pthread-mutexattr-t) | struct |  |
| [`pthread_rwlockattr_t`](#pthread-rwlockattr-t) | struct |  |
| [`pthread_condattr_t`](#pthread-condattr-t) | struct |  |
| [`pthread_barrierattr_t`](#pthread-barrierattr-t) | struct |  |
| [`fanotify_event_metadata`](#fanotify-event-metadata) | struct |  |
| [`ptp_sys_offset`](#ptp-sys-offset) | struct |  |
| [`ptp_pin_desc`](#ptp-pin-desc) | struct |  |
| [`ptp_clock_caps`](#ptp-clock-caps) | struct |  |
| [`sockaddr_xdp`](#sockaddr-xdp) | struct |  |
| [`xdp_ring_offset`](#xdp-ring-offset) | struct |  |
| [`xdp_mmap_offsets`](#xdp-mmap-offsets) | struct |  |
| [`xdp_ring_offset_v1`](#xdp-ring-offset-v1) | struct |  |
| [`xdp_mmap_offsets_v1`](#xdp-mmap-offsets-v1) | struct |  |
| [`xdp_umem_reg`](#xdp-umem-reg) | struct |  |
| [`xdp_umem_reg_v1`](#xdp-umem-reg-v1) | struct |  |
| [`xdp_statistics`](#xdp-statistics) | struct |  |
| [`xdp_statistics_v1`](#xdp-statistics-v1) | struct |  |
| [`xdp_options`](#xdp-options) | struct |  |
| [`xdp_desc`](#xdp-desc) | struct |  |
| [`xsk_tx_metadata_completion`](#xsk-tx-metadata-completion) | struct |  |
| [`xsk_tx_metadata_request`](#xsk-tx-metadata-request) | struct |  |
| [`mount_attr`](#mount-attr) | struct |  |
| [`mnt_ns_info`](#mnt-ns-info) | struct |  |
| [`dmabuf_cmsg`](#dmabuf-cmsg) | struct |  |
| [`dmabuf_token`](#dmabuf-token) | struct |  |
| [`sockaddr_alg`](#sockaddr-alg) | struct |  |
| [`pthread_cond_t`](#pthread-cond-t) | struct |  |
| [`pthread_mutex_t`](#pthread-mutex-t) | struct |  |
| [`pthread_rwlock_t`](#pthread-rwlock-t) | struct |  |
| [`pthread_barrier_t`](#pthread-barrier-t) | struct |  |
| [`uinput_setup`](#uinput-setup) | struct |  |
| [`uinput_user_dev`](#uinput-user-dev) | struct |  |
| [`mq_attr`](#mq-attr) | struct |  |
| [`hwtstamp_config`](#hwtstamp-config) | struct |  |
| [`sched_attr`](#sched-attr) | struct |  |
| [`file_handle`](#file-handle) | struct |  |
| [`iw_thrspy`](#iw-thrspy) | struct |  |
| [`iw_mlme`](#iw-mlme) | struct |  |
| [`iw_michaelmicfailure`](#iw-michaelmicfailure) | struct |  |
| [`af_alg_iv`](#af-alg-iv) | struct | WARNING: The `PartialEq`, `Eq` and `Hash` implementations of this type are unsound and will be removed in the future. |
| [`tpacket_block_desc`](#tpacket-block-desc) | struct |  |
| [`sock_txtime`](#sock-txtime) | struct |  |
| [`iw_event`](#iw-event) | struct |  |
| [`iwreq`](#iwreq) | struct |  |
| [`ptp_perout_request`](#ptp-perout-request) | struct |  |
| [`xsk_tx_metadata`](#xsk-tx-metadata) | struct |  |
| [`glob_t`](#glob-t) | struct |  |
| [`passwd`](#passwd) | struct |  |
| [`spwd`](#spwd) | struct |  |
| [`itimerspec`](#itimerspec) | struct |  |
| [`fsid_t`](#fsid-t) | struct |  |
| [`packet_mreq`](#packet-mreq) | struct |  |
| [`cpu_set_t`](#cpu-set-t) | struct |  |
| [`sembuf`](#sembuf) | struct |  |
| [`dl_phdr_info`](#dl-phdr-info) | struct |  |
| [`Elf32_Ehdr`](#elf32-ehdr) | struct |  |
| [`Elf64_Ehdr`](#elf64-ehdr) | struct |  |
| [`Elf32_Sym`](#elf32-sym) | struct |  |
| [`Elf64_Sym`](#elf64-sym) | struct |  |
| [`Elf32_Phdr`](#elf32-phdr) | struct |  |
| [`Elf64_Phdr`](#elf64-phdr) | struct |  |
| [`Elf32_Shdr`](#elf32-shdr) | struct |  |
| [`Elf64_Shdr`](#elf64-shdr) | struct |  |
| [`__c_anonymous_elf32_rel`](#c-anonymous-elf32-rel) | struct |  |
| [`__c_anonymous_elf64_rel`](#c-anonymous-elf64-rel) | struct |  |
| [`ucred`](#ucred) | struct |  |
| [`mntent`](#mntent) | struct |  |
| [`in6_pktinfo`](#in6-pktinfo) | struct |  |
| [`arpd_request`](#arpd-request) | struct |  |
| [`regmatch_t`](#regmatch-t) | struct |  |
| [`option`](#option) | struct |  |
| [`rlimit64`](#rlimit64) | struct |  |
| [`__c_anonymous_ifru_map`](#c-anonymous-ifru-map) | struct |  |
| [`dirent`](#dirent) | struct |  |
| [`dirent64`](#dirent64) | struct |  |
| [`__c_anonymous_elf32_rela`](#c-anonymous-elf32-rela) | struct |  |
| [`__c_anonymous_elf64_rela`](#c-anonymous-elf64-rela) | struct |  |
| [`ifreq`](#ifreq) | struct |  |
| [`ifconf`](#ifconf) | struct | Structure used in SIOCGIFCONF request.  Used to retrieve interface configuration for machine (useful for programs which must know all networks accessible). |
| [`timezone`](#timezone) | enum |  |
| [`tpacket_versions`](#tpacket-versions) | enum |  |
| [`ioctl`](#ioctl) | fn |  |
| [`sem_destroy`](#sem-destroy) | fn |  |
| [`sem_init`](#sem-init) | fn |  |
| [`fdatasync`](#fdatasync) | fn |  |
| [`mincore`](#mincore) | fn |  |
| [`clock_getres`](#clock-getres) | fn |  |
| [`clock_gettime`](#clock-gettime) | fn |  |
| [`clock_settime`](#clock-settime) | fn |  |
| [`clock_getcpuclockid`](#clock-getcpuclockid) | fn |  |
| [`getitimer`](#getitimer) | fn |  |
| [`setitimer`](#setitimer) | fn |  |
| [`dirfd`](#dirfd) | fn |  |
| [`memalign`](#memalign) | fn |  |
| [`setgroups`](#setgroups) | fn |  |
| [`pipe2`](#pipe2) | fn |  |
| [`statfs`](#statfs) | fn |  |
| [`fstatfs`](#fstatfs) | fn |  |
| [`memrchr`](#memrchr) | fn |  |
| [`posix_fadvise`](#posix-fadvise) | fn |  |
| [`futimens`](#futimens) | fn |  |
| [`utimensat`](#utimensat) | fn |  |
| [`duplocale`](#duplocale) | fn |  |
| [`freelocale`](#freelocale) | fn |  |
| [`newlocale`](#newlocale) | fn |  |
| [`uselocale`](#uselocale) | fn |  |
| [`mknodat`](#mknodat) | fn |  |
| [`ptsname_r`](#ptsname-r) | fn |  |
| [`clearenv`](#clearenv) | fn |  |
| [`waitid`](#waitid) | fn |  |
| [`getresuid`](#getresuid) | fn |  |
| [`getresgid`](#getresgid) | fn |  |
| [`acct`](#acct) | fn |  |
| [`brk`](#brk) | fn |  |
| [`sbrk`](#sbrk) | fn |  |
| [`vfork`](#vfork) | fn |  |
| [`setresgid`](#setresgid) | fn |  |
| [`setresuid`](#setresuid) | fn |  |
| [`wait4`](#wait4) | fn |  |
| [`login_tty`](#login-tty) | fn |  |
| [`execvpe`](#execvpe) | fn |  |
| [`fexecve`](#fexecve) | fn |  |
| [`getifaddrs`](#getifaddrs) | fn |  |
| [`freeifaddrs`](#freeifaddrs) | fn |  |
| [`bind`](#bind) | fn |  |
| [`writev`](#writev) | fn |  |
| [`readv`](#readv) | fn |  |
| [`sendmsg`](#sendmsg) | fn |  |
| [`recvmsg`](#recvmsg) | fn |  |
| [`uname`](#uname) | fn |  |
| [`strchrnul`](#strchrnul) | fn |  |
| [`strftime`](#strftime) | fn |  |
| [`strftime_l`](#strftime-l) | fn |  |
| [`strptime`](#strptime) | fn |  |
| [`mkostemp`](#mkostemp) | fn |  |
| [`mkostemps`](#mkostemps) | fn |  |
| [`getdomainname`](#getdomainname) | fn |  |
| [`setdomainname`](#setdomainname) | fn |  |
| [`if_nameindex`](#if-nameindex) | fn |  |
| [`if_freenameindex`](#if-freenameindex) | fn |  |
| [`getpwnam_r`](#getpwnam-r) | fn |  |
| [`getpwuid_r`](#getpwuid-r) | fn |  |
| [`fstatfs64`](#fstatfs64) | fn |  |
| [`statvfs64`](#statvfs64) | fn |  |
| [`fstatvfs64`](#fstatvfs64) | fn |  |
| [`statfs64`](#statfs64) | fn |  |
| [`creat64`](#creat64) | fn |  |
| [`fstat64`](#fstat64) | fn |  |
| [`fstatat64`](#fstatat64) | fn |  |
| [`ftruncate64`](#ftruncate64) | fn |  |
| [`lseek64`](#lseek64) | fn |  |
| [`lstat64`](#lstat64) | fn |  |
| [`mmap64`](#mmap64) | fn |  |
| [`open64`](#open64) | fn |  |
| [`openat64`](#openat64) | fn |  |
| [`posix_fadvise64`](#posix-fadvise64) | fn |  |
| [`pread64`](#pread64) | fn |  |
| [`pwrite64`](#pwrite64) | fn |  |
| [`readdir64`](#readdir64) | fn |  |
| [`readdir64_r`](#readdir64-r) | fn |  |
| [`stat64`](#stat64) | fn |  |
| [`truncate64`](#truncate64) | fn |  |
| [`preadv64`](#preadv64) | fn |  |
| [`pwritev64`](#pwritev64) | fn |  |
| [`forkpty`](#forkpty) | fn |  |
| [`openpty`](#openpty) | fn |  |
| [`statx`](#statx) | fn |  |
| [`_IOC`](#ioc) | fn | Build an ioctl number, analogous to the C macro of the same name. |
| [`_IO`](#io) | fn | Build an ioctl number for an argumentless ioctl. |
| [`_IOR`](#ior) | fn | Build an ioctl number for an read-only ioctl. |
| [`_IOW`](#iow) | fn | Build an ioctl number for an write-only ioctl. |
| [`_IOWR`](#iowr) | fn | Build an ioctl number for a read-write ioctl. |
| [`CMSG_ALIGN`](#cmsg-align) | fn |  |
| [`CMSG_FIRSTHDR`](#cmsg-firsthdr) | fn |  |
| [`CMSG_DATA`](#cmsg-data) | fn |  |
| [`CMSG_SPACE`](#cmsg-space) | fn |  |
| [`CMSG_LEN`](#cmsg-len) | fn |  |
| [`FD_CLR`](#fd-clr) | fn |  |
| [`FD_ISSET`](#fd-isset) | fn |  |
| [`FD_SET`](#fd-set) | fn |  |
| [`FD_ZERO`](#fd-zero) | fn |  |
| [`SIGRTMAX`](#sigrtmax) | fn |  |
| [`SIGRTMIN`](#sigrtmin) | fn |  |
| [`WIFSTOPPED`](#wifstopped) | fn |  |
| [`WSTOPSIG`](#wstopsig) | fn |  |
| [`WIFCONTINUED`](#wifcontinued) | fn |  |
| [`WIFSIGNALED`](#wifsignaled) | fn |  |
| [`WTERMSIG`](#wtermsig) | fn |  |
| [`WIFEXITED`](#wifexited) | fn |  |
| [`WEXITSTATUS`](#wexitstatus) | fn |  |
| [`WCOREDUMP`](#wcoredump) | fn |  |
| [`W_EXITCODE`](#w-exitcode) | fn |  |
| [`W_STOPCODE`](#w-stopcode) | fn |  |
| [`QCMD`](#qcmd) | fn |  |
| [`IPOPT_COPIED`](#ipopt-copied) | fn |  |
| [`IPOPT_CLASS`](#ipopt-class) | fn |  |
| [`IPOPT_NUMBER`](#ipopt-number) | fn |  |
| [`IPTOS_ECN`](#iptos-ecn) | fn |  |
| [`KERNEL_VERSION`](#kernel-version) | fn |  |
| [`getspnam_r`](#getspnam-r) | fn |  |
| [`mq_open`](#mq-open) | fn |  |
| [`mq_close`](#mq-close) | fn |  |
| [`mq_unlink`](#mq-unlink) | fn |  |
| [`mq_receive`](#mq-receive) | fn |  |
| [`mq_timedreceive`](#mq-timedreceive) | fn |  |
| [`mq_send`](#mq-send) | fn |  |
| [`mq_timedsend`](#mq-timedsend) | fn |  |
| [`mq_getattr`](#mq-getattr) | fn |  |
| [`mq_setattr`](#mq-setattr) | fn |  |
| [`mrand48`](#mrand48) | fn |  |
| [`seed48`](#seed48) | fn |  |
| [`lcong48`](#lcong48) | fn |  |
| [`lutimes`](#lutimes) | fn |  |
| [`shm_open`](#shm-open) | fn |  |
| [`shm_unlink`](#shm-unlink) | fn |  |
| [`ftok`](#ftok) | fn |  |
| [`semget`](#semget) | fn |  |
| [`semop`](#semop) | fn |  |
| [`semctl`](#semctl) | fn |  |
| [`msgctl`](#msgctl) | fn |  |
| [`msgget`](#msgget) | fn |  |
| [`msgrcv`](#msgrcv) | fn |  |
| [`msgsnd`](#msgsnd) | fn |  |
| [`fallocate`](#fallocate) | fn |  |
| [`posix_fallocate`](#posix-fallocate) | fn |  |
| [`readahead`](#readahead) | fn |  |
| [`getxattr`](#getxattr) | fn |  |
| [`lgetxattr`](#lgetxattr) | fn |  |
| [`fgetxattr`](#fgetxattr) | fn |  |
| [`setxattr`](#setxattr) | fn |  |
| [`lsetxattr`](#lsetxattr) | fn |  |
| [`fsetxattr`](#fsetxattr) | fn |  |
| [`listxattr`](#listxattr) | fn |  |
| [`llistxattr`](#llistxattr) | fn |  |
| [`flistxattr`](#flistxattr) | fn |  |
| [`removexattr`](#removexattr) | fn |  |
| [`lremovexattr`](#lremovexattr) | fn |  |
| [`fremovexattr`](#fremovexattr) | fn |  |
| [`signalfd`](#signalfd) | fn |  |
| [`timerfd_create`](#timerfd-create) | fn |  |
| [`timerfd_gettime`](#timerfd-gettime) | fn |  |
| [`timerfd_settime`](#timerfd-settime) | fn |  |
| [`quotactl`](#quotactl) | fn |  |
| [`epoll_pwait`](#epoll-pwait) | fn |  |
| [`dup3`](#dup3) | fn |  |
| [`sigtimedwait`](#sigtimedwait) | fn |  |
| [`sigwaitinfo`](#sigwaitinfo) | fn |  |
| [`accept4`](#accept4) | fn |  |
| [`reboot`](#reboot) | fn |  |
| [`setfsgid`](#setfsgid) | fn |  |
| [`setfsuid`](#setfsuid) | fn |  |
| [`mkfifoat`](#mkfifoat) | fn |  |
| [`sync_file_range`](#sync-file-range) | fn |  |
| [`posix_madvise`](#posix-madvise) | fn |  |
| [`remap_file_pages`](#remap-file-pages) | fn |  |
| [`mkstemps`](#mkstemps) | fn |  |
| [`vhangup`](#vhangup) | fn |  |
| [`sync`](#sync) | fn |  |
| [`syncfs`](#syncfs) | fn |  |
| [`syscall`](#syscall) | fn |  |
| [`sched_setaffinity`](#sched-setaffinity) | fn |  |
| [`epoll_create`](#epoll-create) | fn |  |
| [`epoll_create1`](#epoll-create1) | fn |  |
| [`epoll_wait`](#epoll-wait) | fn |  |
| [`epoll_ctl`](#epoll-ctl) | fn |  |
| [`unshare`](#unshare) | fn |  |
| [`umount`](#umount) | fn |  |
| [`tee`](#tee) | fn |  |
| [`splice`](#splice) | fn |  |
| [`eventfd`](#eventfd) | fn |  |
| [`eventfd_read`](#eventfd-read) | fn |  |
| [`eventfd_write`](#eventfd-write) | fn |  |
| [`sched_rr_get_interval`](#sched-rr-get-interval) | fn |  |
| [`sched_setparam`](#sched-setparam) | fn |  |
| [`setns`](#setns) | fn |  |
| [`swapoff`](#swapoff) | fn |  |
| [`vmsplice`](#vmsplice) | fn |  |
| [`personality`](#personality) | fn |  |
| [`sched_getparam`](#sched-getparam) | fn |  |
| [`clone`](#clone) | fn |  |
| [`sched_getscheduler`](#sched-getscheduler) | fn |  |
| [`clock_nanosleep`](#clock-nanosleep) | fn |  |
| [`umount2`](#umount2) | fn |  |
| [`swapon`](#swapon) | fn |  |
| [`sched_setscheduler`](#sched-setscheduler) | fn |  |
| [`sendfile`](#sendfile) | fn |  |
| [`sigaltstack`](#sigaltstack) | fn |  |
| [`getdtablesize`](#getdtablesize) | fn |  |
| [`getgrouplist`](#getgrouplist) | fn |  |
| [`posix_spawn`](#posix-spawn) | fn |  |
| [`posix_spawnp`](#posix-spawnp) | fn |  |
| [`posix_spawnattr_init`](#posix-spawnattr-init) | fn |  |
| [`posix_spawnattr_destroy`](#posix-spawnattr-destroy) | fn |  |
| [`posix_spawnattr_getsigdefault`](#posix-spawnattr-getsigdefault) | fn |  |
| [`posix_spawnattr_setsigdefault`](#posix-spawnattr-setsigdefault) | fn |  |
| [`posix_spawnattr_getsigmask`](#posix-spawnattr-getsigmask) | fn |  |
| [`posix_spawnattr_setsigmask`](#posix-spawnattr-setsigmask) | fn |  |
| [`posix_spawnattr_getflags`](#posix-spawnattr-getflags) | fn |  |
| [`posix_spawnattr_setflags`](#posix-spawnattr-setflags) | fn |  |
| [`posix_spawnattr_getpgroup`](#posix-spawnattr-getpgroup) | fn |  |
| [`posix_spawnattr_setpgroup`](#posix-spawnattr-setpgroup) | fn |  |
| [`posix_spawnattr_getschedpolicy`](#posix-spawnattr-getschedpolicy) | fn |  |
| [`posix_spawnattr_setschedpolicy`](#posix-spawnattr-setschedpolicy) | fn |  |
| [`posix_spawnattr_getschedparam`](#posix-spawnattr-getschedparam) | fn |  |
| [`posix_spawnattr_setschedparam`](#posix-spawnattr-setschedparam) | fn |  |
| [`posix_spawn_file_actions_init`](#posix-spawn-file-actions-init) | fn |  |
| [`posix_spawn_file_actions_destroy`](#posix-spawn-file-actions-destroy) | fn |  |
| [`posix_spawn_file_actions_addopen`](#posix-spawn-file-actions-addopen) | fn |  |
| [`posix_spawn_file_actions_addclose`](#posix-spawn-file-actions-addclose) | fn |  |
| [`posix_spawn_file_actions_adddup2`](#posix-spawn-file-actions-adddup2) | fn |  |
| [`fread_unlocked`](#fread-unlocked) | fn |  |
| [`inotify_rm_watch`](#inotify-rm-watch) | fn |  |
| [`inotify_init`](#inotify-init) | fn |  |
| [`inotify_init1`](#inotify-init1) | fn |  |
| [`inotify_add_watch`](#inotify-add-watch) | fn |  |
| [`fanotify_init`](#fanotify-init) | fn |  |
| [`gethostid`](#gethostid) | fn |  |
| [`klogctl`](#klogctl) | fn |  |
| [`name_to_handle_at`](#name-to-handle-at) | fn |  |
| [`open_by_handle_at`](#open-by-handle-at) | fn |  |
| [`fallocate64`](#fallocate64) | fn |  |
| [`fgetpos64`](#fgetpos64) | fn |  |
| [`fopen64`](#fopen64) | fn |  |
| [`posix_fallocate64`](#posix-fallocate64) | fn |  |
| [`sendfile64`](#sendfile64) | fn |  |
| [`tmpfile64`](#tmpfile64) | fn |  |
| [`issecure_mask`](#issecure-mask) | fn |  |
| [`FUTEX_OP`](#futex-op) | fn |  |
| [`SCTP_PR_INDEX`](#sctp-pr-index) | fn |  |
| [`SCTP_PR_POLICY`](#sctp-pr-policy) | fn |  |
| [`SCTP_PR_SET_POLICY`](#sctp-pr-set-policy) | fn |  |
| [`SO_EE_OFFENDER`](#so-ee-offender) | fn |  |
| [`TPACKET_ALIGN`](#tpacket-align) | fn |  |
| [`BPF_CLASS`](#bpf-class) | fn |  |
| [`BPF_SIZE`](#bpf-size) | fn |  |
| [`BPF_MODE`](#bpf-mode) | fn |  |
| [`BPF_OP`](#bpf-op) | fn |  |
| [`BPF_SRC`](#bpf-src) | fn |  |
| [`BPF_RVAL`](#bpf-rval) | fn |  |
| [`BPF_MISCOP`](#bpf-miscop) | fn |  |
| [`BPF_STMT`](#bpf-stmt) | fn |  |
| [`BPF_JUMP`](#bpf-jump) | fn |  |
| [`SUN_LEN`](#sun-len) | fn |  |
| [`SCTP_PR_TTL_ENABLED`](#sctp-pr-ttl-enabled) | fn |  |
| [`SCTP_PR_RTX_ENABLED`](#sctp-pr-rtx-enabled) | fn |  |
| [`SCTP_PR_PRIO_ENABLED`](#sctp-pr-prio-enabled) | fn |  |
| [`iopl`](#iopl) | fn |  |
| [`ioperm`](#ioperm) | fn |  |
| [`aio_read`](#aio-read) | fn |  |
| [`aio_write`](#aio-write) | fn |  |
| [`aio_fsync`](#aio-fsync) | fn |  |
| [`aio_error`](#aio-error) | fn |  |
| [`aio_return`](#aio-return) | fn |  |
| [`aio_suspend`](#aio-suspend) | fn |  |
| [`aio_cancel`](#aio-cancel) | fn |  |
| [`lio_listio`](#lio-listio) | fn |  |
| [`pwritev`](#pwritev) | fn |  |
| [`preadv`](#preadv) | fn |  |
| [`getnameinfo`](#getnameinfo) | fn |  |
| [`getloadavg`](#getloadavg) | fn |  |
| [`process_vm_readv`](#process-vm-readv) | fn |  |
| [`process_vm_writev`](#process-vm-writev) | fn |  |
| [`futimes`](#futimes) | fn |  |
| [`strerror_r`](#strerror-r) | fn |  |
| [`abs`](#abs) | fn |  |
| [`labs`](#labs) | fn |  |
| [`rand`](#rand) | fn |  |
| [`srand`](#srand) | fn |  |
| [`drand48`](#drand48) | fn |  |
| [`erand48`](#erand48) | fn |  |
| [`lrand48`](#lrand48) | fn |  |
| [`nrand48`](#nrand48) | fn |  |
| [`jrand48`](#jrand48) | fn |  |
| [`srand48`](#srand48) | fn |  |
| [`setpwent`](#setpwent) | fn |  |
| [`endpwent`](#endpwent) | fn |  |
| [`getpwent`](#getpwent) | fn |  |
| [`setgrent`](#setgrent) | fn |  |
| [`endgrent`](#endgrent) | fn |  |
| [`getgrent`](#getgrent) | fn |  |
| [`setspent`](#setspent) | fn |  |
| [`endspent`](#endspent) | fn |  |
| [`getspent`](#getspent) | fn |  |
| [`getspnam`](#getspnam) | fn |  |
| [`shmget`](#shmget) | fn |  |
| [`shmat`](#shmat) | fn |  |
| [`shmdt`](#shmdt) | fn |  |
| [`shmctl`](#shmctl) | fn |  |
| [`mprotect`](#mprotect) | fn |  |
| [`__errno_location`](#errno-location) | fn |  |
| [`mremap`](#mremap) | fn |  |
| [`glob`](#glob) | fn |  |
| [`globfree`](#globfree) | fn |  |
| [`seekdir`](#seekdir) | fn |  |
| [`telldir`](#telldir) | fn |  |
| [`madvise`](#madvise) | fn |  |
| [`msync`](#msync) | fn |  |
| [`recvfrom`](#recvfrom) | fn |  |
| [`nl_langinfo`](#nl-langinfo) | fn |  |
| [`nl_langinfo_l`](#nl-langinfo-l) | fn |  |
| [`sched_getaffinity`](#sched-getaffinity) | fn |  |
| [`sched_get_priority_max`](#sched-get-priority-max) | fn |  |
| [`settimeofday`](#settimeofday) | fn |  |
| [`sem_timedwait`](#sem-timedwait) | fn |  |
| [`sem_getvalue`](#sem-getvalue) | fn |  |
| [`mount`](#mount) | fn |  |
| [`prctl`](#prctl) | fn |  |
| [`ppoll`](#ppoll) | fn |  |
| [`sethostname`](#sethostname) | fn |  |
| [`sched_get_priority_min`](#sched-get-priority-min) | fn |  |
| [`sysinfo`](#sysinfo) | fn |  |
| [`sigsuspend`](#sigsuspend) | fn |  |
| [`getgrgid_r`](#getgrgid-r) | fn |  |
| [`sem_close`](#sem-close) | fn |  |
| [`getgrnam_r`](#getgrnam-r) | fn |  |
| [`initgroups`](#initgroups) | fn |  |
| [`sem_open`](#sem-open) | fn |  |
| [`getgrnam`](#getgrnam) | fn |  |
| [`sem_unlink`](#sem-unlink) | fn |  |
| [`daemon`](#daemon) | fn |  |
| [`sigwait`](#sigwait) | fn |  |
| [`getgrgid`](#getgrgid) | fn |  |
| [`popen`](#popen) | fn |  |
| [`faccessat`](#faccessat) | fn |  |
| [`dl_iterate_phdr`](#dl-iterate-phdr) | fn |  |
| [`setmntent`](#setmntent) | fn |  |
| [`getmntent`](#getmntent) | fn |  |
| [`addmntent`](#addmntent) | fn |  |
| [`endmntent`](#endmntent) | fn |  |
| [`hasmntopt`](#hasmntopt) | fn |  |
| [`regcomp`](#regcomp) | fn |  |
| [`regexec`](#regexec) | fn |  |
| [`regerror`](#regerror) | fn |  |
| [`regfree`](#regfree) | fn |  |
| [`iconv_open`](#iconv-open) | fn |  |
| [`iconv`](#iconv) | fn |  |
| [`iconv_close`](#iconv-close) | fn |  |
| [`gettid`](#gettid) | fn |  |
| [`timer_create`](#timer-create) | fn |  |
| [`timer_delete`](#timer-delete) | fn |  |
| [`timer_getoverrun`](#timer-getoverrun) | fn |  |
| [`timer_gettime`](#timer-gettime) | fn |  |
| [`timer_settime`](#timer-settime) | fn |  |
| [`memmem`](#memmem) | fn |  |
| [`sched_getcpu`](#sched-getcpu) | fn |  |
| [`getopt_long`](#getopt-long) | fn |  |
| [`copy_file_range`](#copy-file-range) | fn |  |
| [`freopen64`](#freopen64) | fn |  |
| [`fseeko64`](#fseeko64) | fn |  |
| [`fsetpos64`](#fsetpos64) | fn |  |
| [`ftello64`](#ftello64) | fn |  |
| [`CMSG_NXTHDR`](#cmsg-nxthdr) | fn |  |
| [`CPU_ALLOC_SIZE`](#cpu-alloc-size) | fn |  |
| [`CPU_ZERO`](#cpu-zero) | fn |  |
| [`CPU_SET`](#cpu-set) | fn |  |
| [`CPU_CLR`](#cpu-clr) | fn |  |
| [`CPU_ISSET`](#cpu-isset) | fn |  |
| [`CPU_COUNT_S`](#cpu-count-s) | fn |  |
| [`CPU_COUNT`](#cpu-count) | fn |  |
| [`CPU_EQUAL`](#cpu-equal) | fn |  |
| [`IPTOS_TOS`](#iptos-tos) | fn |  |
| [`IPTOS_PREC`](#iptos-prec) | fn |  |
| [`RT_TOS`](#rt-tos) | fn |  |
| [`RT_ADDRCLASS`](#rt-addrclass) | fn |  |
| [`RT_LOCALADDR`](#rt-localaddr) | fn |  |
| [`ELF32_R_SYM`](#elf32-r-sym) | fn |  |
| [`ELF32_R_TYPE`](#elf32-r-type) | fn |  |
| [`ELF32_R_INFO`](#elf32-r-info) | fn |  |
| [`ELF64_R_SYM`](#elf64-r-sym) | fn |  |
| [`ELF64_R_TYPE`](#elf64-r-type) | fn |  |
| [`ELF64_R_INFO`](#elf64-r-info) | fn |  |
| [`makedev`](#makedev) | fn |  |
| [`major`](#major) | fn |  |
| [`minor`](#minor) | fn |  |
| [`sa_family_t`](#sa-family-t) | type |  |
| [`speed_t`](#speed-t) | type |  |
| [`tcflag_t`](#tcflag-t) | type |  |
| [`clockid_t`](#clockid-t) | type |  |
| [`timer_t`](#timer-t) | type |  |
| [`useconds_t`](#useconds-t) | type |  |
| [`key_t`](#key-t) | type |  |
| [`id_t`](#id-t) | type |  |
| [`dev_t`](#dev-t) | type |  |
| [`socklen_t`](#socklen-t) | type |  |
| [`mode_t`](#mode-t) | type |  |
| [`ino64_t`](#ino64-t) | type |  |
| [`off64_t`](#off64-t) | type |  |
| [`blkcnt64_t`](#blkcnt64-t) | type |  |
| [`rlim64_t`](#rlim64-t) | type |  |
| [`mqd_t`](#mqd-t) | type |  |
| [`nfds_t`](#nfds-t) | type |  |
| [`nl_item`](#nl-item) | type |  |
| [`idtype_t`](#idtype-t) | type |  |
| [`loff_t`](#loff-t) | type |  |
| [`pthread_key_t`](#pthread-key-t) | type |  |
| [`pthread_once_t`](#pthread-once-t) | type |  |
| [`pthread_spinlock_t`](#pthread-spinlock-t) | type |  |
| [`__kernel_fsid_t`](#kernel-fsid-t) | type |  |
| [`__kernel_clockid_t`](#kernel-clockid-t) | type |  |
| [`__u8`](#u8) | type |  |
| [`__u16`](#u16) | type |  |
| [`__s16`](#s16) | type |  |
| [`__u32`](#u32) | type |  |
| [`__s32`](#s32) | type |  |
| [`sctp_assoc_t`](#sctp-assoc-t) | type |  |
| [`eventfd_t`](#eventfd-t) | type |  |
| [`pid_type`](#pid-type) | type |  |
| [`proc_cn_mcast_op`](#proc-cn-mcast-op) | type |  |
| [`proc_cn_event`](#proc-cn-event) | type |  |
| [`Elf32_Half`](#elf32-half) | type |  |
| [`Elf32_Word`](#elf32-word) | type |  |
| [`Elf32_Off`](#elf32-off) | type |  |
| [`Elf32_Addr`](#elf32-addr) | type |  |
| [`Elf32_Xword`](#elf32-xword) | type |  |
| [`Elf32_Sword`](#elf32-sword) | type |  |
| [`Elf64_Half`](#elf64-half) | type |  |
| [`Elf64_Word`](#elf64-word) | type |  |
| [`Elf64_Off`](#elf64-off) | type |  |
| [`Elf64_Addr`](#elf64-addr) | type |  |
| [`Elf64_Xword`](#elf64-xword) | type |  |
| [`Elf64_Sxword`](#elf64-sxword) | type |  |
| [`Elf64_Sword`](#elf64-sword) | type |  |
| [`Elf32_Section`](#elf32-section) | type |  |
| [`Elf64_Section`](#elf64-section) | type |  |
| [`Elf32_Relr`](#elf32-relr) | type |  |
| [`Elf64_Relr`](#elf64-relr) | type |  |
| [`Elf32_Rel`](#elf32-rel) | type |  |
| [`Elf64_Rel`](#elf64-rel) | type |  |
| [`Elf32_Rela`](#elf32-rela) | type |  |
| [`Elf64_Rela`](#elf64-rela) | type |  |
| [`iconv_t`](#iconv-t) | type |  |
| [`ULONG_SIZE`](#ulong-size) | const |  |
| [`EXIT_FAILURE`](#exit-failure) | const |  |
| [`EXIT_SUCCESS`](#exit-success) | const |  |
| [`RAND_MAX`](#rand-max) | const |  |
| [`EOF`](#eof) | const |  |
| [`SEEK_SET`](#seek-set) | const |  |
| [`SEEK_CUR`](#seek-cur) | const |  |
| [`SEEK_END`](#seek-end) | const |  |
| [`_IOFBF`](#iofbf) | const |  |
| [`_IONBF`](#ionbf) | const |  |
| [`_IOLBF`](#iolbf) | const |  |
| [`F_DUPFD`](#f-dupfd) | const |  |
| [`F_GETFD`](#f-getfd) | const |  |
| [`F_SETFD`](#f-setfd) | const |  |
| [`F_GETFL`](#f-getfl) | const |  |
| [`F_SETFL`](#f-setfl) | const |  |
| [`F_SETLEASE`](#f-setlease) | const |  |
| [`F_GETLEASE`](#f-getlease) | const |  |
| [`F_NOTIFY`](#f-notify) | const |  |
| [`F_CANCELLK`](#f-cancellk) | const |  |
| [`F_DUPFD_CLOEXEC`](#f-dupfd-cloexec) | const |  |
| [`F_SETPIPE_SZ`](#f-setpipe-sz) | const |  |
| [`F_GETPIPE_SZ`](#f-getpipe-sz) | const |  |
| [`F_ADD_SEALS`](#f-add-seals) | const |  |
| [`F_GET_SEALS`](#f-get-seals) | const |  |
| [`F_SEAL_SEAL`](#f-seal-seal) | const |  |
| [`F_SEAL_SHRINK`](#f-seal-shrink) | const |  |
| [`F_SEAL_GROW`](#f-seal-grow) | const |  |
| [`F_SEAL_WRITE`](#f-seal-write) | const |  |
| [`SIGTRAP`](#sigtrap) | const |  |
| [`PTHREAD_CREATE_JOINABLE`](#pthread-create-joinable) | const |  |
| [`PTHREAD_CREATE_DETACHED`](#pthread-create-detached) | const |  |
| [`CLOCK_REALTIME`](#clock-realtime) | const |  |
| [`CLOCK_MONOTONIC`](#clock-monotonic) | const |  |
| [`CLOCK_PROCESS_CPUTIME_ID`](#clock-process-cputime-id) | const |  |
| [`CLOCK_THREAD_CPUTIME_ID`](#clock-thread-cputime-id) | const |  |
| [`CLOCK_MONOTONIC_RAW`](#clock-monotonic-raw) | const |  |
| [`CLOCK_REALTIME_COARSE`](#clock-realtime-coarse) | const |  |
| [`CLOCK_MONOTONIC_COARSE`](#clock-monotonic-coarse) | const |  |
| [`CLOCK_BOOTTIME`](#clock-boottime) | const |  |
| [`CLOCK_REALTIME_ALARM`](#clock-realtime-alarm) | const |  |
| [`CLOCK_BOOTTIME_ALARM`](#clock-boottime-alarm) | const |  |
| [`CLOCK_TAI`](#clock-tai) | const |  |
| [`TIMER_ABSTIME`](#timer-abstime) | const |  |
| [`RUSAGE_SELF`](#rusage-self) | const |  |
| [`O_RDONLY`](#o-rdonly) | const |  |
| [`O_WRONLY`](#o-wronly) | const |  |
| [`O_RDWR`](#o-rdwr) | const |  |
| [`SOCK_CLOEXEC`](#sock-cloexec) | const |  |
| [`S_IFIFO`](#s-ififo) | const |  |
| [`S_IFCHR`](#s-ifchr) | const |  |
| [`S_IFBLK`](#s-ifblk) | const |  |
| [`S_IFDIR`](#s-ifdir) | const |  |
| [`S_IFREG`](#s-ifreg) | const |  |
| [`S_IFLNK`](#s-iflnk) | const |  |
| [`S_IFSOCK`](#s-ifsock) | const |  |
| [`S_IFMT`](#s-ifmt) | const |  |
| [`S_IRWXU`](#s-irwxu) | const |  |
| [`S_IXUSR`](#s-ixusr) | const |  |
| [`S_IWUSR`](#s-iwusr) | const |  |
| [`S_IRUSR`](#s-irusr) | const |  |
| [`S_IRWXG`](#s-irwxg) | const |  |
| [`S_IXGRP`](#s-ixgrp) | const |  |
| [`S_IWGRP`](#s-iwgrp) | const |  |
| [`S_IRGRP`](#s-irgrp) | const |  |
| [`S_IRWXO`](#s-irwxo) | const |  |
| [`S_IXOTH`](#s-ixoth) | const |  |
| [`S_IWOTH`](#s-iwoth) | const |  |
| [`S_IROTH`](#s-iroth) | const |  |
| [`F_OK`](#f-ok) | const |  |
| [`R_OK`](#r-ok) | const |  |
| [`W_OK`](#w-ok) | const |  |
| [`X_OK`](#x-ok) | const |  |
| [`SIGHUP`](#sighup) | const |  |
| [`SIGINT`](#sigint) | const |  |
| [`SIGQUIT`](#sigquit) | const |  |
| [`SIGILL`](#sigill) | const |  |
| [`SIGABRT`](#sigabrt) | const |  |
| [`SIGFPE`](#sigfpe) | const |  |
| [`SIGKILL`](#sigkill) | const |  |
| [`SIGSEGV`](#sigsegv) | const |  |
| [`SIGPIPE`](#sigpipe) | const |  |
| [`SIGALRM`](#sigalrm) | const |  |
| [`SIGTERM`](#sigterm) | const |  |
| [`PROT_NONE`](#prot-none) | const |  |
| [`PROT_READ`](#prot-read) | const |  |
| [`PROT_WRITE`](#prot-write) | const |  |
| [`PROT_EXEC`](#prot-exec) | const |  |
| [`XATTR_CREATE`](#xattr-create) | const |  |
| [`XATTR_REPLACE`](#xattr-replace) | const |  |
| [`RLIM64_INFINITY`](#rlim64-infinity) | const |  |
| [`LC_CTYPE`](#lc-ctype) | const |  |
| [`LC_NUMERIC`](#lc-numeric) | const |  |
| [`LC_TIME`](#lc-time) | const |  |
| [`LC_COLLATE`](#lc-collate) | const |  |
| [`LC_MONETARY`](#lc-monetary) | const |  |
| [`LC_MESSAGES`](#lc-messages) | const |  |
| [`LC_ALL`](#lc-all) | const |  |
| [`LC_CTYPE_MASK`](#lc-ctype-mask) | const |  |
| [`LC_NUMERIC_MASK`](#lc-numeric-mask) | const |  |
| [`LC_TIME_MASK`](#lc-time-mask) | const |  |
| [`LC_COLLATE_MASK`](#lc-collate-mask) | const |  |
| [`LC_MONETARY_MASK`](#lc-monetary-mask) | const |  |
| [`LC_MESSAGES_MASK`](#lc-messages-mask) | const |  |
| [`MAP_FILE`](#map-file) | const |  |
| [`MAP_SHARED`](#map-shared) | const |  |
| [`MAP_PRIVATE`](#map-private) | const |  |
| [`MAP_FIXED`](#map-fixed) | const |  |
| [`MAP_FAILED`](#map-failed) | const |  |
| [`MS_ASYNC`](#ms-async) | const |  |
| [`MS_INVALIDATE`](#ms-invalidate) | const |  |
| [`MS_SYNC`](#ms-sync) | const |  |
| [`MS_RDONLY`](#ms-rdonly) | const |  |
| [`MS_NOSUID`](#ms-nosuid) | const |  |
| [`MS_NODEV`](#ms-nodev) | const |  |
| [`MS_NOEXEC`](#ms-noexec) | const |  |
| [`MS_SYNCHRONOUS`](#ms-synchronous) | const |  |
| [`MS_REMOUNT`](#ms-remount) | const |  |
| [`MS_MANDLOCK`](#ms-mandlock) | const |  |
| [`MS_DIRSYNC`](#ms-dirsync) | const |  |
| [`MS_NOSYMFOLLOW`](#ms-nosymfollow) | const |  |
| [`MS_NOATIME`](#ms-noatime) | const |  |
| [`MS_NODIRATIME`](#ms-nodiratime) | const |  |
| [`MS_BIND`](#ms-bind) | const |  |
| [`MS_MOVE`](#ms-move) | const |  |
| [`MS_REC`](#ms-rec) | const |  |
| [`MS_SILENT`](#ms-silent) | const |  |
| [`MS_POSIXACL`](#ms-posixacl) | const |  |
| [`MS_UNBINDABLE`](#ms-unbindable) | const |  |
| [`MS_PRIVATE`](#ms-private) | const |  |
| [`MS_SLAVE`](#ms-slave) | const |  |
| [`MS_SHARED`](#ms-shared) | const |  |
| [`MS_RELATIME`](#ms-relatime) | const |  |
| [`MS_KERNMOUNT`](#ms-kernmount) | const |  |
| [`MS_I_VERSION`](#ms-i-version) | const |  |
| [`MS_STRICTATIME`](#ms-strictatime) | const |  |
| [`MS_LAZYTIME`](#ms-lazytime) | const |  |
| [`MS_ACTIVE`](#ms-active) | const |  |
| [`MS_MGC_VAL`](#ms-mgc-val) | const |  |
| [`MS_MGC_MSK`](#ms-mgc-msk) | const |  |
| [`SCM_RIGHTS`](#scm-rights) | const |  |
| [`SCM_CREDENTIALS`](#scm-credentials) | const |  |
| [`PROT_GROWSDOWN`](#prot-growsdown) | const |  |
| [`PROT_GROWSUP`](#prot-growsup) | const |  |
| [`MAP_TYPE`](#map-type) | const |  |
| [`MADV_NORMAL`](#madv-normal) | const |  |
| [`MADV_RANDOM`](#madv-random) | const |  |
| [`MADV_SEQUENTIAL`](#madv-sequential) | const |  |
| [`MADV_WILLNEED`](#madv-willneed) | const |  |
| [`MADV_DONTNEED`](#madv-dontneed) | const |  |
| [`MADV_FREE`](#madv-free) | const |  |
| [`MADV_REMOVE`](#madv-remove) | const |  |
| [`MADV_DONTFORK`](#madv-dontfork) | const |  |
| [`MADV_DOFORK`](#madv-dofork) | const |  |
| [`MADV_MERGEABLE`](#madv-mergeable) | const |  |
| [`MADV_UNMERGEABLE`](#madv-unmergeable) | const |  |
| [`MADV_HUGEPAGE`](#madv-hugepage) | const |  |
| [`MADV_NOHUGEPAGE`](#madv-nohugepage) | const |  |
| [`MADV_DONTDUMP`](#madv-dontdump) | const |  |
| [`MADV_DODUMP`](#madv-dodump) | const |  |
| [`MADV_WIPEONFORK`](#madv-wipeonfork) | const |  |
| [`MADV_KEEPONFORK`](#madv-keeponfork) | const |  |
| [`MADV_COLD`](#madv-cold) | const |  |
| [`MADV_PAGEOUT`](#madv-pageout) | const |  |
| [`MADV_HWPOISON`](#madv-hwpoison) | const |  |
| [`MADV_POPULATE_READ`](#madv-populate-read) | const |  |
| [`MADV_POPULATE_WRITE`](#madv-populate-write) | const |  |
| [`MADV_DONTNEED_LOCKED`](#madv-dontneed-locked) | const |  |
| [`IFF_UP`](#iff-up) | const |  |
| [`IFF_BROADCAST`](#iff-broadcast) | const |  |
| [`IFF_DEBUG`](#iff-debug) | const |  |
| [`IFF_LOOPBACK`](#iff-loopback) | const |  |
| [`IFF_POINTOPOINT`](#iff-pointopoint) | const |  |
| [`IFF_NOTRAILERS`](#iff-notrailers) | const |  |
| [`IFF_RUNNING`](#iff-running) | const |  |
| [`IFF_NOARP`](#iff-noarp) | const |  |
| [`IFF_PROMISC`](#iff-promisc) | const |  |
| [`IFF_ALLMULTI`](#iff-allmulti) | const |  |
| [`IFF_MASTER`](#iff-master) | const |  |
| [`IFF_SLAVE`](#iff-slave) | const |  |
| [`IFF_MULTICAST`](#iff-multicast) | const |  |
| [`IFF_PORTSEL`](#iff-portsel) | const |  |
| [`IFF_AUTOMEDIA`](#iff-automedia) | const |  |
| [`IFF_DYNAMIC`](#iff-dynamic) | const |  |
| [`SOL_IP`](#sol-ip) | const |  |
| [`SOL_TCP`](#sol-tcp) | const |  |
| [`SOL_UDP`](#sol-udp) | const |  |
| [`SOL_IPV6`](#sol-ipv6) | const |  |
| [`SOL_ICMPV6`](#sol-icmpv6) | const |  |
| [`SOL_RAW`](#sol-raw) | const |  |
| [`SOL_DECNET`](#sol-decnet) | const |  |
| [`SOL_X25`](#sol-x25) | const |  |
| [`SOL_PACKET`](#sol-packet) | const |  |
| [`SOL_ATM`](#sol-atm) | const |  |
| [`SOL_AAL`](#sol-aal) | const |  |
| [`SOL_IRDA`](#sol-irda) | const |  |
| [`SOL_NETBEUI`](#sol-netbeui) | const |  |
| [`SOL_LLC`](#sol-llc) | const |  |
| [`SOL_DCCP`](#sol-dccp) | const |  |
| [`SOL_NETLINK`](#sol-netlink) | const |  |
| [`SOL_TIPC`](#sol-tipc) | const |  |
| [`SOL_BLUETOOTH`](#sol-bluetooth) | const |  |
| [`SOL_ALG`](#sol-alg) | const |  |
| [`AF_UNSPEC`](#af-unspec) | const |  |
| [`AF_UNIX`](#af-unix) | const |  |
| [`AF_LOCAL`](#af-local) | const |  |
| [`AF_INET`](#af-inet) | const |  |
| [`AF_AX25`](#af-ax25) | const |  |
| [`AF_IPX`](#af-ipx) | const |  |
| [`AF_APPLETALK`](#af-appletalk) | const |  |
| [`AF_NETROM`](#af-netrom) | const |  |
| [`AF_BRIDGE`](#af-bridge) | const |  |
| [`AF_ATMPVC`](#af-atmpvc) | const |  |
| [`AF_X25`](#af-x25) | const |  |
| [`AF_INET6`](#af-inet6) | const |  |
| [`AF_ROSE`](#af-rose) | const |  |
| [`AF_DECnet`](#af-decnet) | const |  |
| [`AF_NETBEUI`](#af-netbeui) | const |  |
| [`AF_SECURITY`](#af-security) | const |  |
| [`AF_KEY`](#af-key) | const |  |
| [`AF_NETLINK`](#af-netlink) | const |  |
| [`AF_ROUTE`](#af-route) | const |  |
| [`AF_PACKET`](#af-packet) | const |  |
| [`AF_ASH`](#af-ash) | const |  |
| [`AF_ECONET`](#af-econet) | const |  |
| [`AF_ATMSVC`](#af-atmsvc) | const |  |
| [`AF_RDS`](#af-rds) | const |  |
| [`AF_SNA`](#af-sna) | const |  |
| [`AF_IRDA`](#af-irda) | const |  |
| [`AF_PPPOX`](#af-pppox) | const |  |
| [`AF_WANPIPE`](#af-wanpipe) | const |  |
| [`AF_LLC`](#af-llc) | const |  |
| [`AF_CAN`](#af-can) | const |  |
| [`AF_TIPC`](#af-tipc) | const |  |
| [`AF_BLUETOOTH`](#af-bluetooth) | const |  |
| [`AF_IUCV`](#af-iucv) | const |  |
| [`AF_RXRPC`](#af-rxrpc) | const |  |
| [`AF_ISDN`](#af-isdn) | const |  |
| [`AF_PHONET`](#af-phonet) | const |  |
| [`AF_IEEE802154`](#af-ieee802154) | const |  |
| [`AF_CAIF`](#af-caif) | const |  |
| [`AF_ALG`](#af-alg) | const |  |
| [`PF_UNSPEC`](#pf-unspec) | const |  |
| [`PF_UNIX`](#pf-unix) | const |  |
| [`PF_LOCAL`](#pf-local) | const |  |
| [`PF_INET`](#pf-inet) | const |  |
| [`PF_AX25`](#pf-ax25) | const |  |
| [`PF_IPX`](#pf-ipx) | const |  |
| [`PF_APPLETALK`](#pf-appletalk) | const |  |
| [`PF_NETROM`](#pf-netrom) | const |  |
| [`PF_BRIDGE`](#pf-bridge) | const |  |
| [`PF_ATMPVC`](#pf-atmpvc) | const |  |
| [`PF_X25`](#pf-x25) | const |  |
| [`PF_INET6`](#pf-inet6) | const |  |
| [`PF_ROSE`](#pf-rose) | const |  |
| [`PF_DECnet`](#pf-decnet) | const |  |
| [`PF_NETBEUI`](#pf-netbeui) | const |  |
| [`PF_SECURITY`](#pf-security) | const |  |
| [`PF_KEY`](#pf-key) | const |  |
| [`PF_NETLINK`](#pf-netlink) | const |  |
| [`PF_ROUTE`](#pf-route) | const |  |
| [`PF_PACKET`](#pf-packet) | const |  |
| [`PF_ASH`](#pf-ash) | const |  |
| [`PF_ECONET`](#pf-econet) | const |  |
| [`PF_ATMSVC`](#pf-atmsvc) | const |  |
| [`PF_RDS`](#pf-rds) | const |  |
| [`PF_SNA`](#pf-sna) | const |  |
| [`PF_IRDA`](#pf-irda) | const |  |
| [`PF_PPPOX`](#pf-pppox) | const |  |
| [`PF_WANPIPE`](#pf-wanpipe) | const |  |
| [`PF_LLC`](#pf-llc) | const |  |
| [`PF_CAN`](#pf-can) | const |  |
| [`PF_TIPC`](#pf-tipc) | const |  |
| [`PF_BLUETOOTH`](#pf-bluetooth) | const |  |
| [`PF_IUCV`](#pf-iucv) | const |  |
| [`PF_RXRPC`](#pf-rxrpc) | const |  |
| [`PF_ISDN`](#pf-isdn) | const |  |
| [`PF_PHONET`](#pf-phonet) | const |  |
| [`PF_IEEE802154`](#pf-ieee802154) | const |  |
| [`PF_CAIF`](#pf-caif) | const |  |
| [`PF_ALG`](#pf-alg) | const |  |
| [`MSG_OOB`](#msg-oob) | const |  |
| [`MSG_PEEK`](#msg-peek) | const |  |
| [`MSG_DONTROUTE`](#msg-dontroute) | const |  |
| [`MSG_CTRUNC`](#msg-ctrunc) | const |  |
| [`MSG_TRUNC`](#msg-trunc) | const |  |
| [`MSG_DONTWAIT`](#msg-dontwait) | const |  |
| [`MSG_EOR`](#msg-eor) | const |  |
| [`MSG_WAITALL`](#msg-waitall) | const |  |
| [`MSG_FIN`](#msg-fin) | const |  |
| [`MSG_SYN`](#msg-syn) | const |  |
| [`MSG_CONFIRM`](#msg-confirm) | const |  |
| [`MSG_RST`](#msg-rst) | const |  |
| [`MSG_ERRQUEUE`](#msg-errqueue) | const |  |
| [`MSG_NOSIGNAL`](#msg-nosignal) | const |  |
| [`MSG_MORE`](#msg-more) | const |  |
| [`MSG_WAITFORONE`](#msg-waitforone) | const |  |
| [`MSG_FASTOPEN`](#msg-fastopen) | const |  |
| [`MSG_CMSG_CLOEXEC`](#msg-cmsg-cloexec) | const |  |
| [`SCM_TIMESTAMP`](#scm-timestamp) | const |  |
| [`SOCK_RAW`](#sock-raw) | const |  |
| [`SOCK_RDM`](#sock-rdm) | const |  |
| [`IP_TOS`](#ip-tos) | const |  |
| [`IP_TTL`](#ip-ttl) | const |  |
| [`IP_HDRINCL`](#ip-hdrincl) | const |  |
| [`IP_OPTIONS`](#ip-options) | const |  |
| [`IP_ROUTER_ALERT`](#ip-router-alert) | const |  |
| [`IP_RECVOPTS`](#ip-recvopts) | const |  |
| [`IP_RETOPTS`](#ip-retopts) | const |  |
| [`IP_PKTINFO`](#ip-pktinfo) | const |  |
| [`IP_PKTOPTIONS`](#ip-pktoptions) | const |  |
| [`IP_MTU_DISCOVER`](#ip-mtu-discover) | const |  |
| [`IP_RECVERR`](#ip-recverr) | const |  |
| [`IP_RECVTTL`](#ip-recvttl) | const |  |
| [`IP_RECVTOS`](#ip-recvtos) | const |  |
| [`IP_MTU`](#ip-mtu) | const |  |
| [`IP_FREEBIND`](#ip-freebind) | const |  |
| [`IP_IPSEC_POLICY`](#ip-ipsec-policy) | const |  |
| [`IP_XFRM_POLICY`](#ip-xfrm-policy) | const |  |
| [`IP_PASSSEC`](#ip-passsec) | const |  |
| [`IP_TRANSPARENT`](#ip-transparent) | const |  |
| [`IP_ORIGDSTADDR`](#ip-origdstaddr) | const |  |
| [`IP_RECVORIGDSTADDR`](#ip-recvorigdstaddr) | const |  |
| [`IP_MINTTL`](#ip-minttl) | const |  |
| [`IP_NODEFRAG`](#ip-nodefrag) | const |  |
| [`IP_CHECKSUM`](#ip-checksum) | const |  |
| [`IP_BIND_ADDRESS_NO_PORT`](#ip-bind-address-no-port) | const |  |
| [`IP_MULTICAST_IF`](#ip-multicast-if) | const |  |
| [`IP_MULTICAST_TTL`](#ip-multicast-ttl) | const |  |
| [`IP_MULTICAST_LOOP`](#ip-multicast-loop) | const |  |
| [`IP_ADD_MEMBERSHIP`](#ip-add-membership) | const |  |
| [`IP_DROP_MEMBERSHIP`](#ip-drop-membership) | const |  |
| [`IP_UNBLOCK_SOURCE`](#ip-unblock-source) | const |  |
| [`IP_BLOCK_SOURCE`](#ip-block-source) | const |  |
| [`IP_ADD_SOURCE_MEMBERSHIP`](#ip-add-source-membership) | const |  |
| [`IP_DROP_SOURCE_MEMBERSHIP`](#ip-drop-source-membership) | const |  |
| [`IP_MSFILTER`](#ip-msfilter) | const |  |
| [`IP_MULTICAST_ALL`](#ip-multicast-all) | const |  |
| [`IP_UNICAST_IF`](#ip-unicast-if) | const |  |
| [`IP_DEFAULT_MULTICAST_TTL`](#ip-default-multicast-ttl) | const |  |
| [`IP_DEFAULT_MULTICAST_LOOP`](#ip-default-multicast-loop) | const |  |
| [`IP_PMTUDISC_DONT`](#ip-pmtudisc-dont) | const |  |
| [`IP_PMTUDISC_WANT`](#ip-pmtudisc-want) | const |  |
| [`IP_PMTUDISC_DO`](#ip-pmtudisc-do) | const |  |
| [`IP_PMTUDISC_PROBE`](#ip-pmtudisc-probe) | const |  |
| [`IP_PMTUDISC_INTERFACE`](#ip-pmtudisc-interface) | const |  |
| [`IP_PMTUDISC_OMIT`](#ip-pmtudisc-omit) | const |  |
| [`IPPROTO_HOPOPTS`](#ipproto-hopopts) | const | Hop-by-hop option header |
| [`IPPROTO_IGMP`](#ipproto-igmp) | const | group mgmt protocol |
| [`IPPROTO_IPIP`](#ipproto-ipip) | const | for compatibility |
| [`IPPROTO_EGP`](#ipproto-egp) | const | exterior gateway protocol |
| [`IPPROTO_PUP`](#ipproto-pup) | const | pup |
| [`IPPROTO_IDP`](#ipproto-idp) | const | xns idp |
| [`IPPROTO_TP`](#ipproto-tp) | const | tp-4 w/ class negotiation |
| [`IPPROTO_DCCP`](#ipproto-dccp) | const | DCCP |
| [`IPPROTO_ROUTING`](#ipproto-routing) | const | IP6 routing header |
| [`IPPROTO_FRAGMENT`](#ipproto-fragment) | const | IP6 fragmentation header |
| [`IPPROTO_RSVP`](#ipproto-rsvp) | const | resource reservation |
| [`IPPROTO_GRE`](#ipproto-gre) | const | General Routing Encap. |
| [`IPPROTO_ESP`](#ipproto-esp) | const | IP6 Encap Sec. |
| [`IPPROTO_AH`](#ipproto-ah) | const | IP6 Auth Header |
| [`IPPROTO_NONE`](#ipproto-none) | const | IP6 no next header |
| [`IPPROTO_DSTOPTS`](#ipproto-dstopts) | const | IP6 destination option |
| [`IPPROTO_MTP`](#ipproto-mtp) | const |  |
| [`IPPROTO_ENCAP`](#ipproto-encap) | const | encapsulation header |
| [`IPPROTO_PIM`](#ipproto-pim) | const | Protocol indep. |
| [`IPPROTO_COMP`](#ipproto-comp) | const | IP Payload Comp. |
| [`IPPROTO_SCTP`](#ipproto-sctp) | const | SCTP |
| [`IPPROTO_MH`](#ipproto-mh) | const |  |
| [`IPPROTO_UDPLITE`](#ipproto-udplite) | const |  |
| [`IPPROTO_RAW`](#ipproto-raw) | const | raw IP packet |
| [`IPPROTO_BEETPH`](#ipproto-beetph) | const |  |
| [`IPPROTO_MPLS`](#ipproto-mpls) | const |  |
| [`IPPROTO_MPTCP`](#ipproto-mptcp) | const | Multipath TCP |
| [`IPPROTO_ETHERNET`](#ipproto-ethernet) | const | Ethernet-within-IPv6 encapsulation. |
| [`MCAST_EXCLUDE`](#mcast-exclude) | const |  |
| [`MCAST_INCLUDE`](#mcast-include) | const |  |
| [`MCAST_JOIN_GROUP`](#mcast-join-group) | const |  |
| [`MCAST_BLOCK_SOURCE`](#mcast-block-source) | const |  |
| [`MCAST_UNBLOCK_SOURCE`](#mcast-unblock-source) | const |  |
| [`MCAST_LEAVE_GROUP`](#mcast-leave-group) | const |  |
| [`MCAST_JOIN_SOURCE_GROUP`](#mcast-join-source-group) | const |  |
| [`MCAST_LEAVE_SOURCE_GROUP`](#mcast-leave-source-group) | const |  |
| [`MCAST_MSFILTER`](#mcast-msfilter) | const |  |
| [`IPV6_ADDRFORM`](#ipv6-addrform) | const |  |
| [`IPV6_2292PKTINFO`](#ipv6-2292pktinfo) | const |  |
| [`IPV6_2292HOPOPTS`](#ipv6-2292hopopts) | const |  |
| [`IPV6_2292DSTOPTS`](#ipv6-2292dstopts) | const |  |
| [`IPV6_2292RTHDR`](#ipv6-2292rthdr) | const |  |
| [`IPV6_2292PKTOPTIONS`](#ipv6-2292pktoptions) | const |  |
| [`IPV6_CHECKSUM`](#ipv6-checksum) | const |  |
| [`IPV6_2292HOPLIMIT`](#ipv6-2292hoplimit) | const |  |
| [`IPV6_NEXTHOP`](#ipv6-nexthop) | const |  |
| [`IPV6_AUTHHDR`](#ipv6-authhdr) | const |  |
| [`IPV6_UNICAST_HOPS`](#ipv6-unicast-hops) | const |  |
| [`IPV6_MULTICAST_IF`](#ipv6-multicast-if) | const |  |
| [`IPV6_MULTICAST_HOPS`](#ipv6-multicast-hops) | const |  |
| [`IPV6_MULTICAST_LOOP`](#ipv6-multicast-loop) | const |  |
| [`IPV6_ADD_MEMBERSHIP`](#ipv6-add-membership) | const |  |
| [`IPV6_DROP_MEMBERSHIP`](#ipv6-drop-membership) | const |  |
| [`IPV6_ROUTER_ALERT`](#ipv6-router-alert) | const |  |
| [`IPV6_MTU_DISCOVER`](#ipv6-mtu-discover) | const |  |
| [`IPV6_MTU`](#ipv6-mtu) | const |  |
| [`IPV6_RECVERR`](#ipv6-recverr) | const |  |
| [`IPV6_V6ONLY`](#ipv6-v6only) | const |  |
| [`IPV6_JOIN_ANYCAST`](#ipv6-join-anycast) | const |  |
| [`IPV6_LEAVE_ANYCAST`](#ipv6-leave-anycast) | const |  |
| [`IPV6_IPSEC_POLICY`](#ipv6-ipsec-policy) | const |  |
| [`IPV6_XFRM_POLICY`](#ipv6-xfrm-policy) | const |  |
| [`IPV6_HDRINCL`](#ipv6-hdrincl) | const |  |
| [`IPV6_RECVPKTINFO`](#ipv6-recvpktinfo) | const |  |
| [`IPV6_PKTINFO`](#ipv6-pktinfo) | const |  |
| [`IPV6_RECVHOPLIMIT`](#ipv6-recvhoplimit) | const |  |
| [`IPV6_HOPLIMIT`](#ipv6-hoplimit) | const |  |
| [`IPV6_RECVHOPOPTS`](#ipv6-recvhopopts) | const |  |
| [`IPV6_HOPOPTS`](#ipv6-hopopts) | const |  |
| [`IPV6_RTHDRDSTOPTS`](#ipv6-rthdrdstopts) | const |  |
| [`IPV6_RECVRTHDR`](#ipv6-recvrthdr) | const |  |
| [`IPV6_RTHDR`](#ipv6-rthdr) | const |  |
| [`IPV6_RECVDSTOPTS`](#ipv6-recvdstopts) | const |  |
| [`IPV6_DSTOPTS`](#ipv6-dstopts) | const |  |
| [`IPV6_RECVPATHMTU`](#ipv6-recvpathmtu) | const |  |
| [`IPV6_PATHMTU`](#ipv6-pathmtu) | const |  |
| [`IPV6_DONTFRAG`](#ipv6-dontfrag) | const |  |
| [`IPV6_RECVTCLASS`](#ipv6-recvtclass) | const |  |
| [`IPV6_TCLASS`](#ipv6-tclass) | const |  |
| [`IPV6_AUTOFLOWLABEL`](#ipv6-autoflowlabel) | const |  |
| [`IPV6_ADDR_PREFERENCES`](#ipv6-addr-preferences) | const |  |
| [`IPV6_MINHOPCOUNT`](#ipv6-minhopcount) | const |  |
| [`IPV6_ORIGDSTADDR`](#ipv6-origdstaddr) | const |  |
| [`IPV6_RECVORIGDSTADDR`](#ipv6-recvorigdstaddr) | const |  |
| [`IPV6_TRANSPARENT`](#ipv6-transparent) | const |  |
| [`IPV6_UNICAST_IF`](#ipv6-unicast-if) | const |  |
| [`IPV6_PREFER_SRC_TMP`](#ipv6-prefer-src-tmp) | const |  |
| [`IPV6_PREFER_SRC_PUBLIC`](#ipv6-prefer-src-public) | const |  |
| [`IPV6_PREFER_SRC_PUBTMP_DEFAULT`](#ipv6-prefer-src-pubtmp-default) | const |  |
| [`IPV6_PREFER_SRC_COA`](#ipv6-prefer-src-coa) | const |  |
| [`IPV6_PREFER_SRC_HOME`](#ipv6-prefer-src-home) | const |  |
| [`IPV6_PREFER_SRC_CGA`](#ipv6-prefer-src-cga) | const |  |
| [`IPV6_PREFER_SRC_NONCGA`](#ipv6-prefer-src-noncga) | const |  |
| [`IPV6_PMTUDISC_DONT`](#ipv6-pmtudisc-dont) | const |  |
| [`IPV6_PMTUDISC_WANT`](#ipv6-pmtudisc-want) | const |  |
| [`IPV6_PMTUDISC_DO`](#ipv6-pmtudisc-do) | const |  |
| [`IPV6_PMTUDISC_PROBE`](#ipv6-pmtudisc-probe) | const |  |
| [`IPV6_PMTUDISC_INTERFACE`](#ipv6-pmtudisc-interface) | const |  |
| [`IPV6_PMTUDISC_OMIT`](#ipv6-pmtudisc-omit) | const |  |
| [`TCP_NODELAY`](#tcp-nodelay) | const |  |
| [`TCP_MAXSEG`](#tcp-maxseg) | const |  |
| [`TCP_CORK`](#tcp-cork) | const |  |
| [`TCP_KEEPIDLE`](#tcp-keepidle) | const |  |
| [`TCP_KEEPINTVL`](#tcp-keepintvl) | const |  |
| [`TCP_KEEPCNT`](#tcp-keepcnt) | const |  |
| [`TCP_SYNCNT`](#tcp-syncnt) | const |  |
| [`TCP_LINGER2`](#tcp-linger2) | const |  |
| [`TCP_DEFER_ACCEPT`](#tcp-defer-accept) | const |  |
| [`TCP_WINDOW_CLAMP`](#tcp-window-clamp) | const |  |
| [`TCP_INFO`](#tcp-info) | const |  |
| [`TCP_QUICKACK`](#tcp-quickack) | const |  |
| [`TCP_CONGESTION`](#tcp-congestion) | const |  |
| [`TCP_MD5SIG`](#tcp-md5sig) | const |  |
| [`TCP_COOKIE_TRANSACTIONS`](#tcp-cookie-transactions) | const |  |
| [`TCP_THIN_LINEAR_TIMEOUTS`](#tcp-thin-linear-timeouts) | const |  |
| [`TCP_THIN_DUPACK`](#tcp-thin-dupack) | const |  |
| [`TCP_USER_TIMEOUT`](#tcp-user-timeout) | const |  |
| [`TCP_REPAIR`](#tcp-repair) | const |  |
| [`TCP_REPAIR_QUEUE`](#tcp-repair-queue) | const |  |
| [`TCP_QUEUE_SEQ`](#tcp-queue-seq) | const |  |
| [`TCP_REPAIR_OPTIONS`](#tcp-repair-options) | const |  |
| [`TCP_FASTOPEN`](#tcp-fastopen) | const |  |
| [`TCP_TIMESTAMP`](#tcp-timestamp) | const |  |
| [`TCP_NOTSENT_LOWAT`](#tcp-notsent-lowat) | const |  |
| [`TCP_CC_INFO`](#tcp-cc-info) | const |  |
| [`TCP_SAVE_SYN`](#tcp-save-syn) | const |  |
| [`TCP_SAVED_SYN`](#tcp-saved-syn) | const |  |
| [`TCP_REPAIR_WINDOW`](#tcp-repair-window) | const |  |
| [`TCP_FASTOPEN_CONNECT`](#tcp-fastopen-connect) | const |  |
| [`TCP_ULP`](#tcp-ulp) | const |  |
| [`TCP_MD5SIG_EXT`](#tcp-md5sig-ext) | const |  |
| [`TCP_FASTOPEN_KEY`](#tcp-fastopen-key) | const |  |
| [`TCP_FASTOPEN_NO_COOKIE`](#tcp-fastopen-no-cookie) | const |  |
| [`TCP_ZEROCOPY_RECEIVE`](#tcp-zerocopy-receive) | const |  |
| [`TCP_INQ`](#tcp-inq) | const |  |
| [`TCP_CM_INQ`](#tcp-cm-inq) | const |  |
| [`TCP_MD5SIG_MAXKEYLEN`](#tcp-md5sig-maxkeylen) | const |  |
| [`SO_DEBUG`](#so-debug) | const |  |
| [`SHUT_RD`](#shut-rd) | const |  |
| [`SHUT_WR`](#shut-wr) | const |  |
| [`SHUT_RDWR`](#shut-rdwr) | const |  |
| [`LOCK_SH`](#lock-sh) | const |  |
| [`LOCK_EX`](#lock-ex) | const |  |
| [`LOCK_NB`](#lock-nb) | const |  |
| [`LOCK_UN`](#lock-un) | const |  |
| [`SS_ONSTACK`](#ss-onstack) | const |  |
| [`SS_DISABLE`](#ss-disable) | const |  |
| [`NAME_MAX`](#name-max) | const |  |
| [`PATH_MAX`](#path-max) | const |  |
| [`UIO_MAXIOV`](#uio-maxiov) | const |  |
| [`FD_SETSIZE`](#fd-setsize) | const |  |
| [`EPOLLIN`](#epollin) | const |  |
| [`EPOLLPRI`](#epollpri) | const |  |
| [`EPOLLOUT`](#epollout) | const |  |
| [`EPOLLERR`](#epollerr) | const |  |
| [`EPOLLHUP`](#epollhup) | const |  |
| [`EPOLLRDNORM`](#epollrdnorm) | const |  |
| [`EPOLLRDBAND`](#epollrdband) | const |  |
| [`EPOLLWRNORM`](#epollwrnorm) | const |  |
| [`EPOLLWRBAND`](#epollwrband) | const |  |
| [`EPOLLMSG`](#epollmsg) | const |  |
| [`EPOLLRDHUP`](#epollrdhup) | const |  |
| [`EPOLLEXCLUSIVE`](#epollexclusive) | const |  |
| [`EPOLLWAKEUP`](#epollwakeup) | const |  |
| [`EPOLLONESHOT`](#epolloneshot) | const |  |
| [`EPOLLET`](#epollet) | const |  |
| [`EPOLL_CTL_ADD`](#epoll-ctl-add) | const |  |
| [`EPOLL_CTL_MOD`](#epoll-ctl-mod) | const |  |
| [`EPOLL_CTL_DEL`](#epoll-ctl-del) | const |  |
| [`MNT_FORCE`](#mnt-force) | const |  |
| [`MNT_DETACH`](#mnt-detach) | const |  |
| [`MNT_EXPIRE`](#mnt-expire) | const |  |
| [`UMOUNT_NOFOLLOW`](#umount-nofollow) | const |  |
| [`Q_GETFMT`](#q-getfmt) | const |  |
| [`Q_GETINFO`](#q-getinfo) | const |  |
| [`Q_SETINFO`](#q-setinfo) | const |  |
| [`QIF_BLIMITS`](#qif-blimits) | const |  |
| [`QIF_SPACE`](#qif-space) | const |  |
| [`QIF_ILIMITS`](#qif-ilimits) | const |  |
| [`QIF_INODES`](#qif-inodes) | const |  |
| [`QIF_BTIME`](#qif-btime) | const |  |
| [`QIF_ITIME`](#qif-itime) | const |  |
| [`QIF_LIMITS`](#qif-limits) | const |  |
| [`QIF_USAGE`](#qif-usage) | const |  |
| [`QIF_TIMES`](#qif-times) | const |  |
| [`QIF_ALL`](#qif-all) | const |  |
| [`Q_SYNC`](#q-sync) | const |  |
| [`Q_QUOTAON`](#q-quotaon) | const |  |
| [`Q_QUOTAOFF`](#q-quotaoff) | const |  |
| [`Q_GETQUOTA`](#q-getquota) | const |  |
| [`Q_SETQUOTA`](#q-setquota) | const |  |
| [`TCIOFF`](#tcioff) | const |  |
| [`TCION`](#tcion) | const |  |
| [`TCOOFF`](#tcooff) | const |  |
| [`TCOON`](#tcoon) | const |  |
| [`TCIFLUSH`](#tciflush) | const |  |
| [`TCOFLUSH`](#tcoflush) | const |  |
| [`TCIOFLUSH`](#tcioflush) | const |  |
| [`NL0`](#nl0) | const |  |
| [`NL1`](#nl1) | const |  |
| [`TAB0`](#tab0) | const |  |
| [`CR0`](#cr0) | const |  |
| [`FF0`](#ff0) | const |  |
| [`BS0`](#bs0) | const |  |
| [`VT0`](#vt0) | const |  |
| [`VERASE`](#verase) | const |  |
| [`VKILL`](#vkill) | const |  |
| [`VINTR`](#vintr) | const |  |
| [`VQUIT`](#vquit) | const |  |
| [`VLNEXT`](#vlnext) | const |  |
| [`IGNBRK`](#ignbrk) | const |  |
| [`BRKINT`](#brkint) | const |  |
| [`IGNPAR`](#ignpar) | const |  |
| [`PARMRK`](#parmrk) | const |  |
| [`INPCK`](#inpck) | const |  |
| [`ISTRIP`](#istrip) | const |  |
| [`INLCR`](#inlcr) | const |  |
| [`IGNCR`](#igncr) | const |  |
| [`ICRNL`](#icrnl) | const |  |
| [`IXANY`](#ixany) | const |  |
| [`IMAXBEL`](#imaxbel) | const |  |
| [`OPOST`](#opost) | const |  |
| [`CS5`](#cs5) | const |  |
| [`CRTSCTS`](#crtscts) | const |  |
| [`ECHO`](#echo) | const |  |
| [`OCRNL`](#ocrnl) | const |  |
| [`ONOCR`](#onocr) | const |  |
| [`ONLRET`](#onlret) | const |  |
| [`OFILL`](#ofill) | const |  |
| [`OFDEL`](#ofdel) | const |  |
| [`CLONE_VM`](#clone-vm) | const |  |
| [`CLONE_FS`](#clone-fs) | const |  |
| [`CLONE_FILES`](#clone-files) | const |  |
| [`CLONE_SIGHAND`](#clone-sighand) | const |  |
| [`CLONE_PTRACE`](#clone-ptrace) | const |  |
| [`CLONE_VFORK`](#clone-vfork) | const |  |
| [`CLONE_PARENT`](#clone-parent) | const |  |
| [`CLONE_THREAD`](#clone-thread) | const |  |
| [`CLONE_NEWNS`](#clone-newns) | const |  |
| [`CLONE_SYSVSEM`](#clone-sysvsem) | const |  |
| [`CLONE_SETTLS`](#clone-settls) | const |  |
| [`CLONE_PARENT_SETTID`](#clone-parent-settid) | const |  |
| [`CLONE_CHILD_CLEARTID`](#clone-child-cleartid) | const |  |
| [`CLONE_DETACHED`](#clone-detached) | const |  |
| [`CLONE_UNTRACED`](#clone-untraced) | const |  |
| [`CLONE_CHILD_SETTID`](#clone-child-settid) | const |  |
| [`CLONE_NEWCGROUP`](#clone-newcgroup) | const |  |
| [`CLONE_NEWUTS`](#clone-newuts) | const |  |
| [`CLONE_NEWIPC`](#clone-newipc) | const |  |
| [`CLONE_NEWUSER`](#clone-newuser) | const |  |
| [`CLONE_NEWPID`](#clone-newpid) | const |  |
| [`CLONE_NEWNET`](#clone-newnet) | const |  |
| [`CLONE_IO`](#clone-io) | const |  |
| [`WNOHANG`](#wnohang) | const |  |
| [`WUNTRACED`](#wuntraced) | const |  |
| [`WSTOPPED`](#wstopped) | const |  |
| [`WEXITED`](#wexited) | const |  |
| [`WCONTINUED`](#wcontinued) | const |  |
| [`WNOWAIT`](#wnowait) | const |  |
| [`ADDR_NO_RANDOMIZE`](#addr-no-randomize) | const |  |
| [`MMAP_PAGE_ZERO`](#mmap-page-zero) | const |  |
| [`ADDR_COMPAT_LAYOUT`](#addr-compat-layout) | const |  |
| [`READ_IMPLIES_EXEC`](#read-implies-exec) | const |  |
| [`ADDR_LIMIT_32BIT`](#addr-limit-32bit) | const |  |
| [`SHORT_INODE`](#short-inode) | const |  |
| [`WHOLE_SECONDS`](#whole-seconds) | const |  |
| [`STICKY_TIMEOUTS`](#sticky-timeouts) | const |  |
| [`ADDR_LIMIT_3GB`](#addr-limit-3gb) | const |  |
| [`PTRACE_O_TRACESYSGOOD`](#ptrace-o-tracesysgood) | const |  |
| [`PTRACE_O_TRACEFORK`](#ptrace-o-tracefork) | const |  |
| [`PTRACE_O_TRACEVFORK`](#ptrace-o-tracevfork) | const |  |
| [`PTRACE_O_TRACECLONE`](#ptrace-o-traceclone) | const |  |
| [`PTRACE_O_TRACEEXEC`](#ptrace-o-traceexec) | const |  |
| [`PTRACE_O_TRACEVFORKDONE`](#ptrace-o-tracevforkdone) | const |  |
| [`PTRACE_O_TRACEEXIT`](#ptrace-o-traceexit) | const |  |
| [`PTRACE_O_TRACESECCOMP`](#ptrace-o-traceseccomp) | const |  |
| [`PTRACE_O_SUSPEND_SECCOMP`](#ptrace-o-suspend-seccomp) | const |  |
| [`PTRACE_O_EXITKILL`](#ptrace-o-exitkill) | const |  |
| [`PTRACE_O_MASK`](#ptrace-o-mask) | const |  |
| [`PTRACE_EVENT_FORK`](#ptrace-event-fork) | const |  |
| [`PTRACE_EVENT_VFORK`](#ptrace-event-vfork) | const |  |
| [`PTRACE_EVENT_CLONE`](#ptrace-event-clone) | const |  |
| [`PTRACE_EVENT_EXEC`](#ptrace-event-exec) | const |  |
| [`PTRACE_EVENT_VFORK_DONE`](#ptrace-event-vfork-done) | const |  |
| [`PTRACE_EVENT_EXIT`](#ptrace-event-exit) | const |  |
| [`PTRACE_EVENT_SECCOMP`](#ptrace-event-seccomp) | const |  |
| [`__WNOTHREAD`](#wnothread) | const |  |
| [`__WALL`](#wall) | const |  |
| [`__WCLONE`](#wclone) | const |  |
| [`SPLICE_F_MOVE`](#splice-f-move) | const |  |
| [`SPLICE_F_NONBLOCK`](#splice-f-nonblock) | const |  |
| [`SPLICE_F_MORE`](#splice-f-more) | const |  |
| [`SPLICE_F_GIFT`](#splice-f-gift) | const |  |
| [`RTLD_LOCAL`](#rtld-local) | const |  |
| [`RTLD_LAZY`](#rtld-lazy) | const |  |
| [`POSIX_FADV_NORMAL`](#posix-fadv-normal) | const |  |
| [`POSIX_FADV_RANDOM`](#posix-fadv-random) | const |  |
| [`POSIX_FADV_SEQUENTIAL`](#posix-fadv-sequential) | const |  |
| [`POSIX_FADV_WILLNEED`](#posix-fadv-willneed) | const |  |
| [`AT_FDCWD`](#at-fdcwd) | const |  |
| [`AT_SYMLINK_NOFOLLOW`](#at-symlink-nofollow) | const |  |
| [`AT_REMOVEDIR`](#at-removedir) | const |  |
| [`AT_SYMLINK_FOLLOW`](#at-symlink-follow) | const |  |
| [`AT_NO_AUTOMOUNT`](#at-no-automount) | const |  |
| [`AT_EMPTY_PATH`](#at-empty-path) | const |  |
| [`AT_RECURSIVE`](#at-recursive) | const |  |
| [`LOG_CRON`](#log-cron) | const |  |
| [`LOG_AUTHPRIV`](#log-authpriv) | const |  |
| [`LOG_FTP`](#log-ftp) | const |  |
| [`LOG_PERROR`](#log-perror) | const |  |
| [`PIPE_BUF`](#pipe-buf) | const |  |
| [`SI_LOAD_SHIFT`](#si-load-shift) | const |  |
| [`SI_USER`](#si-user) | const |  |
| [`SI_KERNEL`](#si-kernel) | const |  |
| [`SI_QUEUE`](#si-queue) | const |  |
| [`SI_TIMER`](#si-timer) | const |  |
| [`SI_MESGQ`](#si-mesgq) | const |  |
| [`SI_ASYNCIO`](#si-asyncio) | const |  |
| [`SI_SIGIO`](#si-sigio) | const |  |
| [`SI_TKILL`](#si-tkill) | const |  |
| [`SI_ASYNCNL`](#si-asyncnl) | const |  |
| [`BUS_ADRALN`](#bus-adraln) | const |  |
| [`BUS_ADRERR`](#bus-adrerr) | const |  |
| [`BUS_OBJERR`](#bus-objerr) | const |  |
| [`BUS_MCEERR_AR`](#bus-mceerr-ar) | const |  |
| [`BUS_MCEERR_AO`](#bus-mceerr-ao) | const |  |
| [`TRAP_BRKPT`](#trap-brkpt) | const |  |
| [`TRAP_TRACE`](#trap-trace) | const |  |
| [`TRAP_BRANCH`](#trap-branch) | const |  |
| [`TRAP_HWBKPT`](#trap-hwbkpt) | const |  |
| [`TRAP_UNK`](#trap-unk) | const |  |
| [`CLD_EXITED`](#cld-exited) | const |  |
| [`CLD_KILLED`](#cld-killed) | const |  |
| [`CLD_DUMPED`](#cld-dumped) | const |  |
| [`CLD_TRAPPED`](#cld-trapped) | const |  |
| [`CLD_STOPPED`](#cld-stopped) | const |  |
| [`CLD_CONTINUED`](#cld-continued) | const |  |
| [`SIGEV_SIGNAL`](#sigev-signal) | const |  |
| [`SIGEV_NONE`](#sigev-none) | const |  |
| [`SIGEV_THREAD`](#sigev-thread) | const |  |
| [`P_ALL`](#p-all) | const |  |
| [`P_PID`](#p-pid) | const |  |
| [`P_PGID`](#p-pgid) | const |  |
| [`P_PIDFD`](#p-pidfd) | const |  |
| [`UTIME_OMIT`](#utime-omit) | const |  |
| [`UTIME_NOW`](#utime-now) | const |  |
| [`POLLIN`](#pollin) | const |  |
| [`POLLPRI`](#pollpri) | const |  |
| [`POLLOUT`](#pollout) | const |  |
| [`POLLERR`](#pollerr) | const |  |
| [`POLLHUP`](#pollhup) | const |  |
| [`POLLNVAL`](#pollnval) | const |  |
| [`POLLRDNORM`](#pollrdnorm) | const |  |
| [`POLLRDBAND`](#pollrdband) | const |  |
| [`POLLRDHUP`](#pollrdhup) | const |  |
| [`IPTOS_LOWDELAY`](#iptos-lowdelay) | const |  |
| [`IPTOS_THROUGHPUT`](#iptos-throughput) | const |  |
| [`IPTOS_RELIABILITY`](#iptos-reliability) | const |  |
| [`IPTOS_MINCOST`](#iptos-mincost) | const |  |
| [`IPTOS_PREC_NETCONTROL`](#iptos-prec-netcontrol) | const |  |
| [`IPTOS_PREC_INTERNETCONTROL`](#iptos-prec-internetcontrol) | const |  |
| [`IPTOS_PREC_CRITIC_ECP`](#iptos-prec-critic-ecp) | const |  |
| [`IPTOS_PREC_FLASHOVERRIDE`](#iptos-prec-flashoverride) | const |  |
| [`IPTOS_PREC_FLASH`](#iptos-prec-flash) | const |  |
| [`IPTOS_PREC_IMMEDIATE`](#iptos-prec-immediate) | const |  |
| [`IPTOS_PREC_PRIORITY`](#iptos-prec-priority) | const |  |
| [`IPTOS_PREC_ROUTINE`](#iptos-prec-routine) | const |  |
| [`IPTOS_ECN_MASK`](#iptos-ecn-mask) | const |  |
| [`IPTOS_ECN_ECT1`](#iptos-ecn-ect1) | const |  |
| [`IPTOS_ECN_ECT0`](#iptos-ecn-ect0) | const |  |
| [`IPTOS_ECN_CE`](#iptos-ecn-ce) | const |  |
| [`IPOPT_COPY`](#ipopt-copy) | const |  |
| [`IPOPT_CLASS_MASK`](#ipopt-class-mask) | const |  |
| [`IPOPT_NUMBER_MASK`](#ipopt-number-mask) | const |  |
| [`IPOPT_CONTROL`](#ipopt-control) | const |  |
| [`IPOPT_RESERVED1`](#ipopt-reserved1) | const |  |
| [`IPOPT_MEASUREMENT`](#ipopt-measurement) | const |  |
| [`IPOPT_RESERVED2`](#ipopt-reserved2) | const |  |
| [`IPOPT_END`](#ipopt-end) | const |  |
| [`IPOPT_NOOP`](#ipopt-noop) | const |  |
| [`IPOPT_SEC`](#ipopt-sec) | const |  |
| [`IPOPT_LSRR`](#ipopt-lsrr) | const |  |
| [`IPOPT_TIMESTAMP`](#ipopt-timestamp) | const |  |
| [`IPOPT_RR`](#ipopt-rr) | const |  |
| [`IPOPT_SID`](#ipopt-sid) | const |  |
| [`IPOPT_SSRR`](#ipopt-ssrr) | const |  |
| [`IPOPT_RA`](#ipopt-ra) | const |  |
| [`IPVERSION`](#ipversion) | const |  |
| [`MAXTTL`](#maxttl) | const |  |
| [`IPDEFTTL`](#ipdefttl) | const |  |
| [`IPOPT_OPTVAL`](#ipopt-optval) | const |  |
| [`IPOPT_OLEN`](#ipopt-olen) | const |  |
| [`IPOPT_OFFSET`](#ipopt-offset) | const |  |
| [`IPOPT_MINOFF`](#ipopt-minoff) | const |  |
| [`MAX_IPOPTLEN`](#max-ipoptlen) | const |  |
| [`IPOPT_NOP`](#ipopt-nop) | const |  |
| [`IPOPT_EOL`](#ipopt-eol) | const |  |
| [`IPOPT_TS`](#ipopt-ts) | const |  |
| [`IPOPT_TS_TSONLY`](#ipopt-ts-tsonly) | const |  |
| [`IPOPT_TS_TSANDADDR`](#ipopt-ts-tsandaddr) | const |  |
| [`IPOPT_TS_PRESPEC`](#ipopt-ts-prespec) | const |  |
| [`ARPOP_RREQUEST`](#arpop-rrequest) | const |  |
| [`ARPOP_RREPLY`](#arpop-rreply) | const |  |
| [`ARPOP_InREQUEST`](#arpop-inrequest) | const |  |
| [`ARPOP_InREPLY`](#arpop-inreply) | const |  |
| [`ARPOP_NAK`](#arpop-nak) | const |  |
| [`ATF_NETMASK`](#atf-netmask) | const |  |
| [`ATF_DONTPUB`](#atf-dontpub) | const |  |
| [`ARPHRD_NETROM`](#arphrd-netrom) | const |  |
| [`ARPHRD_ETHER`](#arphrd-ether) | const |  |
| [`ARPHRD_EETHER`](#arphrd-eether) | const |  |
| [`ARPHRD_AX25`](#arphrd-ax25) | const |  |
| [`ARPHRD_PRONET`](#arphrd-pronet) | const |  |
| [`ARPHRD_CHAOS`](#arphrd-chaos) | const |  |
| [`ARPHRD_IEEE802`](#arphrd-ieee802) | const |  |
| [`ARPHRD_ARCNET`](#arphrd-arcnet) | const |  |
| [`ARPHRD_APPLETLK`](#arphrd-appletlk) | const |  |
| [`ARPHRD_DLCI`](#arphrd-dlci) | const |  |
| [`ARPHRD_ATM`](#arphrd-atm) | const |  |
| [`ARPHRD_METRICOM`](#arphrd-metricom) | const |  |
| [`ARPHRD_IEEE1394`](#arphrd-ieee1394) | const |  |
| [`ARPHRD_EUI64`](#arphrd-eui64) | const |  |
| [`ARPHRD_INFINIBAND`](#arphrd-infiniband) | const |  |
| [`ARPHRD_SLIP`](#arphrd-slip) | const |  |
| [`ARPHRD_CSLIP`](#arphrd-cslip) | const |  |
| [`ARPHRD_SLIP6`](#arphrd-slip6) | const |  |
| [`ARPHRD_CSLIP6`](#arphrd-cslip6) | const |  |
| [`ARPHRD_RSRVD`](#arphrd-rsrvd) | const |  |
| [`ARPHRD_ADAPT`](#arphrd-adapt) | const |  |
| [`ARPHRD_ROSE`](#arphrd-rose) | const |  |
| [`ARPHRD_X25`](#arphrd-x25) | const |  |
| [`ARPHRD_HWX25`](#arphrd-hwx25) | const |  |
| [`ARPHRD_CAN`](#arphrd-can) | const |  |
| [`ARPHRD_PPP`](#arphrd-ppp) | const |  |
| [`ARPHRD_CISCO`](#arphrd-cisco) | const |  |
| [`ARPHRD_HDLC`](#arphrd-hdlc) | const |  |
| [`ARPHRD_LAPB`](#arphrd-lapb) | const |  |
| [`ARPHRD_DDCMP`](#arphrd-ddcmp) | const |  |
| [`ARPHRD_RAWHDLC`](#arphrd-rawhdlc) | const |  |
| [`ARPHRD_TUNNEL`](#arphrd-tunnel) | const |  |
| [`ARPHRD_TUNNEL6`](#arphrd-tunnel6) | const |  |
| [`ARPHRD_FRAD`](#arphrd-frad) | const |  |
| [`ARPHRD_SKIP`](#arphrd-skip) | const |  |
| [`ARPHRD_LOOPBACK`](#arphrd-loopback) | const |  |
| [`ARPHRD_LOCALTLK`](#arphrd-localtlk) | const |  |
| [`ARPHRD_FDDI`](#arphrd-fddi) | const |  |
| [`ARPHRD_BIF`](#arphrd-bif) | const |  |
| [`ARPHRD_SIT`](#arphrd-sit) | const |  |
| [`ARPHRD_IPDDP`](#arphrd-ipddp) | const |  |
| [`ARPHRD_IPGRE`](#arphrd-ipgre) | const |  |
| [`ARPHRD_PIMREG`](#arphrd-pimreg) | const |  |
| [`ARPHRD_HIPPI`](#arphrd-hippi) | const |  |
| [`ARPHRD_ASH`](#arphrd-ash) | const |  |
| [`ARPHRD_ECONET`](#arphrd-econet) | const |  |
| [`ARPHRD_IRDA`](#arphrd-irda) | const |  |
| [`ARPHRD_FCPP`](#arphrd-fcpp) | const |  |
| [`ARPHRD_FCAL`](#arphrd-fcal) | const |  |
| [`ARPHRD_FCPL`](#arphrd-fcpl) | const |  |
| [`ARPHRD_FCFABRIC`](#arphrd-fcfabric) | const |  |
| [`ARPHRD_IEEE802_TR`](#arphrd-ieee802-tr) | const |  |
| [`ARPHRD_IEEE80211`](#arphrd-ieee80211) | const |  |
| [`ARPHRD_IEEE80211_PRISM`](#arphrd-ieee80211-prism) | const |  |
| [`ARPHRD_IEEE80211_RADIOTAP`](#arphrd-ieee80211-radiotap) | const |  |
| [`ARPHRD_IEEE802154`](#arphrd-ieee802154) | const |  |
| [`ARPHRD_VOID`](#arphrd-void) | const |  |
| [`ARPHRD_NONE`](#arphrd-none) | const |  |
| [`IFF_TUN`](#iff-tun) | const |  |
| [`IFF_TAP`](#iff-tap) | const |  |
| [`IFF_NAPI`](#iff-napi) | const |  |
| [`IFF_NAPI_FRAGS`](#iff-napi-frags) | const |  |
| [`IFF_NO_CARRIER`](#iff-no-carrier) | const |  |
| [`IFF_NO_PI`](#iff-no-pi) | const |  |
| [`TUN_READQ_SIZE`](#tun-readq-size) | const |  |
| [`TUN_TUN_DEV`](#tun-tun-dev) | const |  |
| [`TUN_TAP_DEV`](#tun-tap-dev) | const |  |
| [`TUN_TYPE_MASK`](#tun-type-mask) | const |  |
| [`IFF_ONE_QUEUE`](#iff-one-queue) | const |  |
| [`IFF_VNET_HDR`](#iff-vnet-hdr) | const |  |
| [`IFF_TUN_EXCL`](#iff-tun-excl) | const |  |
| [`IFF_MULTI_QUEUE`](#iff-multi-queue) | const |  |
| [`IFF_ATTACH_QUEUE`](#iff-attach-queue) | const |  |
| [`IFF_DETACH_QUEUE`](#iff-detach-queue) | const |  |
| [`IFF_PERSIST`](#iff-persist) | const |  |
| [`IFF_NOFILTER`](#iff-nofilter) | const |  |
| [`TUN_TX_TIMESTAMP`](#tun-tx-timestamp) | const |  |
| [`TUN_F_CSUM`](#tun-f-csum) | const |  |
| [`TUN_F_TSO4`](#tun-f-tso4) | const |  |
| [`TUN_F_TSO6`](#tun-f-tso6) | const |  |
| [`TUN_F_TSO_ECN`](#tun-f-tso-ecn) | const |  |
| [`TUN_F_UFO`](#tun-f-ufo) | const |  |
| [`TUN_F_USO4`](#tun-f-uso4) | const |  |
| [`TUN_F_USO6`](#tun-f-uso6) | const |  |
| [`TUN_PKT_STRIP`](#tun-pkt-strip) | const |  |
| [`TUN_FLT_ALLMULTI`](#tun-flt-allmulti) | const |  |
| [`T_TYPE`](#t-type) | const |  |
| [`TUNSETNOCSUM`](#tunsetnocsum) | const |  |
| [`TUNSETDEBUG`](#tunsetdebug) | const |  |
| [`TUNSETIFF`](#tunsetiff) | const |  |
| [`TUNSETPERSIST`](#tunsetpersist) | const |  |
| [`TUNSETOWNER`](#tunsetowner) | const |  |
| [`TUNSETLINK`](#tunsetlink) | const |  |
| [`TUNSETGROUP`](#tunsetgroup) | const |  |
| [`TUNGETFEATURES`](#tungetfeatures) | const |  |
| [`TUNSETOFFLOAD`](#tunsetoffload) | const |  |
| [`TUNSETTXFILTER`](#tunsettxfilter) | const |  |
| [`TUNGETIFF`](#tungetiff) | const |  |
| [`TUNGETSNDBUF`](#tungetsndbuf) | const |  |
| [`TUNSETSNDBUF`](#tunsetsndbuf) | const |  |
| [`TUNATTACHFILTER`](#tunattachfilter) | const |  |
| [`TUNDETACHFILTER`](#tundetachfilter) | const |  |
| [`TUNGETVNETHDRSZ`](#tungetvnethdrsz) | const |  |
| [`TUNSETVNETHDRSZ`](#tunsetvnethdrsz) | const |  |
| [`TUNSETQUEUE`](#tunsetqueue) | const |  |
| [`TUNSETIFINDEX`](#tunsetifindex) | const |  |
| [`TUNGETFILTER`](#tungetfilter) | const |  |
| [`TUNSETVNETLE`](#tunsetvnetle) | const |  |
| [`TUNGETVNETLE`](#tungetvnetle) | const |  |
| [`TUNSETVNETBE`](#tunsetvnetbe) | const |  |
| [`TUNGETVNETBE`](#tungetvnetbe) | const |  |
| [`TUNSETSTEERINGEBPF`](#tunsetsteeringebpf) | const |  |
| [`TUNSETFILTEREBPF`](#tunsetfilterebpf) | const |  |
| [`TUNSETCARRIER`](#tunsetcarrier) | const |  |
| [`TUNGETDEVNETNS`](#tungetdevnetns) | const |  |
| [`FS_IOC_GETFLAGS`](#fs-ioc-getflags) | const |  |
| [`FS_IOC_SETFLAGS`](#fs-ioc-setflags) | const |  |
| [`FS_IOC_GETVERSION`](#fs-ioc-getversion) | const |  |
| [`FS_IOC_SETVERSION`](#fs-ioc-setversion) | const |  |
| [`FS_IOC32_GETFLAGS`](#fs-ioc32-getflags) | const |  |
| [`FS_IOC32_SETFLAGS`](#fs-ioc32-setflags) | const |  |
| [`FS_IOC32_GETVERSION`](#fs-ioc32-getversion) | const |  |
| [`FS_IOC32_SETVERSION`](#fs-ioc32-setversion) | const |  |
| [`FICLONE`](#ficlone) | const |  |
| [`FICLONERANGE`](#ficlonerange) | const |  |
| [`ADFS_SUPER_MAGIC`](#adfs-super-magic) | const |  |
| [`AFFS_SUPER_MAGIC`](#affs-super-magic) | const |  |
| [`AFS_SUPER_MAGIC`](#afs-super-magic) | const |  |
| [`AUTOFS_SUPER_MAGIC`](#autofs-super-magic) | const |  |
| [`BPF_FS_MAGIC`](#bpf-fs-magic) | const |  |
| [`BTRFS_SUPER_MAGIC`](#btrfs-super-magic) | const |  |
| [`CGROUP2_SUPER_MAGIC`](#cgroup2-super-magic) | const |  |
| [`CGROUP_SUPER_MAGIC`](#cgroup-super-magic) | const |  |
| [`CODA_SUPER_MAGIC`](#coda-super-magic) | const |  |
| [`CRAMFS_MAGIC`](#cramfs-magic) | const |  |
| [`DEBUGFS_MAGIC`](#debugfs-magic) | const |  |
| [`DEVPTS_SUPER_MAGIC`](#devpts-super-magic) | const |  |
| [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs-super-magic) | const |  |
| [`EFS_SUPER_MAGIC`](#efs-super-magic) | const |  |
| [`EXT2_SUPER_MAGIC`](#ext2-super-magic) | const |  |
| [`EXT3_SUPER_MAGIC`](#ext3-super-magic) | const |  |
| [`EXT4_SUPER_MAGIC`](#ext4-super-magic) | const |  |
| [`F2FS_SUPER_MAGIC`](#f2fs-super-magic) | const |  |
| [`FUSE_SUPER_MAGIC`](#fuse-super-magic) | const |  |
| [`FUTEXFS_SUPER_MAGIC`](#futexfs-super-magic) | const |  |
| [`HOSTFS_SUPER_MAGIC`](#hostfs-super-magic) | const |  |
| [`HPFS_SUPER_MAGIC`](#hpfs-super-magic) | const |  |
| [`HUGETLBFS_MAGIC`](#hugetlbfs-magic) | const |  |
| [`ISOFS_SUPER_MAGIC`](#isofs-super-magic) | const |  |
| [`JFFS2_SUPER_MAGIC`](#jffs2-super-magic) | const |  |
| [`MINIX2_SUPER_MAGIC2`](#minix2-super-magic2) | const |  |
| [`MINIX2_SUPER_MAGIC`](#minix2-super-magic) | const |  |
| [`MINIX3_SUPER_MAGIC`](#minix3-super-magic) | const |  |
| [`MINIX_SUPER_MAGIC2`](#minix-super-magic2) | const |  |
| [`MINIX_SUPER_MAGIC`](#minix-super-magic) | const |  |
| [`MSDOS_SUPER_MAGIC`](#msdos-super-magic) | const |  |
| [`NCP_SUPER_MAGIC`](#ncp-super-magic) | const |  |
| [`NFS_SUPER_MAGIC`](#nfs-super-magic) | const |  |
| [`NILFS_SUPER_MAGIC`](#nilfs-super-magic) | const |  |
| [`OCFS2_SUPER_MAGIC`](#ocfs2-super-magic) | const |  |
| [`OPENPROM_SUPER_MAGIC`](#openprom-super-magic) | const |  |
| [`OVERLAYFS_SUPER_MAGIC`](#overlayfs-super-magic) | const |  |
| [`PROC_SUPER_MAGIC`](#proc-super-magic) | const |  |
| [`QNX4_SUPER_MAGIC`](#qnx4-super-magic) | const |  |
| [`QNX6_SUPER_MAGIC`](#qnx6-super-magic) | const |  |
| [`RDTGROUP_SUPER_MAGIC`](#rdtgroup-super-magic) | const |  |
| [`REISERFS_SUPER_MAGIC`](#reiserfs-super-magic) | const |  |
| [`SECURITYFS_MAGIC`](#securityfs-magic) | const |  |
| [`SELINUX_MAGIC`](#selinux-magic) | const |  |
| [`SMACK_MAGIC`](#smack-magic) | const |  |
| [`SMB_SUPER_MAGIC`](#smb-super-magic) | const |  |
| [`SYSFS_MAGIC`](#sysfs-magic) | const |  |
| [`TMPFS_MAGIC`](#tmpfs-magic) | const |  |
| [`TRACEFS_MAGIC`](#tracefs-magic) | const |  |
| [`UDF_SUPER_MAGIC`](#udf-super-magic) | const |  |
| [`USBDEVICE_SUPER_MAGIC`](#usbdevice-super-magic) | const |  |
| [`XENFS_SUPER_MAGIC`](#xenfs-super-magic) | const |  |
| [`NSFS_MAGIC`](#nsfs-magic) | const |  |
| [`AT_STATX_SYNC_TYPE`](#at-statx-sync-type) | const |  |
| [`AT_STATX_SYNC_AS_STAT`](#at-statx-sync-as-stat) | const |  |
| [`AT_STATX_FORCE_SYNC`](#at-statx-force-sync) | const |  |
| [`AT_STATX_DONT_SYNC`](#at-statx-dont-sync) | const |  |
| [`STATX_TYPE`](#statx-type) | const |  |
| [`STATX_MODE`](#statx-mode) | const |  |
| [`STATX_NLINK`](#statx-nlink) | const |  |
| [`STATX_UID`](#statx-uid) | const |  |
| [`STATX_GID`](#statx-gid) | const |  |
| [`STATX_ATIME`](#statx-atime) | const |  |
| [`STATX_MTIME`](#statx-mtime) | const |  |
| [`STATX_CTIME`](#statx-ctime) | const |  |
| [`STATX_INO`](#statx-ino) | const |  |
| [`STATX_SIZE`](#statx-size) | const |  |
| [`STATX_BLOCKS`](#statx-blocks) | const |  |
| [`STATX_BASIC_STATS`](#statx-basic-stats) | const |  |
| [`STATX_BTIME`](#statx-btime) | const |  |
| [`STATX_ALL`](#statx-all) | const |  |
| [`STATX_MNT_ID`](#statx-mnt-id) | const |  |
| [`STATX_DIOALIGN`](#statx-dioalign) | const |  |
| [`STATX__RESERVED`](#statx-reserved) | const |  |
| [`STATX_ATTR_COMPRESSED`](#statx-attr-compressed) | const |  |
| [`STATX_ATTR_IMMUTABLE`](#statx-attr-immutable) | const |  |
| [`STATX_ATTR_APPEND`](#statx-attr-append) | const |  |
| [`STATX_ATTR_NODUMP`](#statx-attr-nodump) | const |  |
| [`STATX_ATTR_ENCRYPTED`](#statx-attr-encrypted) | const |  |
| [`STATX_ATTR_AUTOMOUNT`](#statx-attr-automount) | const |  |
| [`STATX_ATTR_MOUNT_ROOT`](#statx-attr-mount-root) | const |  |
| [`STATX_ATTR_VERITY`](#statx-attr-verity) | const |  |
| [`STATX_ATTR_DAX`](#statx-attr-dax) | const |  |
| [`_IOC_NRBITS`](#ioc-nrbits) | const |  |
| [`_IOC_TYPEBITS`](#ioc-typebits) | const |  |
| [`_IOC_SIZEBITS`](#ioc-sizebits) | const |  |
| [`_IOC_DIRBITS`](#ioc-dirbits) | const |  |
| [`_IOC_NONE`](#ioc-none) | const |  |
| [`_IOC_WRITE`](#ioc-write) | const |  |
| [`_IOC_READ`](#ioc-read) | const |  |
| [`_IOC_NRMASK`](#ioc-nrmask) | const |  |
| [`_IOC_TYPEMASK`](#ioc-typemask) | const |  |
| [`_IOC_SIZEMASK`](#ioc-sizemask) | const |  |
| [`_IOC_DIRMASK`](#ioc-dirmask) | const |  |
| [`_IOC_NRSHIFT`](#ioc-nrshift) | const |  |
| [`_IOC_TYPESHIFT`](#ioc-typeshift) | const |  |
| [`_IOC_SIZESHIFT`](#ioc-sizeshift) | const |  |
| [`_IOC_DIRSHIFT`](#ioc-dirshift) | const |  |
| [`PIDTYPE_PID`](#pidtype-pid) | const |  |
| [`PIDTYPE_TGID`](#pidtype-tgid) | const |  |
| [`PIDTYPE_PGID`](#pidtype-pgid) | const |  |
| [`PIDTYPE_SID`](#pidtype-sid) | const |  |
| [`PIDTYPE_MAX`](#pidtype-max) | const |  |
| [`POSIX_SPAWN_USEVFORK`](#posix-spawn-usevfork) | const |  |
| [`POSIX_SPAWN_SETSID`](#posix-spawn-setsid) | const |  |
| [`F_SEAL_FUTURE_WRITE`](#f-seal-future-write) | const |  |
| [`F_SEAL_EXEC`](#f-seal-exec) | const |  |
| [`IFF_LOWER_UP`](#iff-lower-up) | const |  |
| [`IFF_DORMANT`](#iff-dormant) | const |  |
| [`IFF_ECHO`](#iff-echo) | const |  |
| [`AT_EXECVE_CHECK`](#at-execve-check) | const |  |
| [`MAX_HANDLE_SZ`](#max-handle-sz) | const |  |
| [`AT_HANDLE_FID`](#at-handle-fid) | const |  |
| [`AT_HANDLE_MNT_ID_UNIQUE`](#at-handle-mnt-id-unique) | const |  |
| [`AT_HANDLE_CONNECTABLE`](#at-handle-connectable) | const |  |
| [`IFA_UNSPEC`](#ifa-unspec) | const |  |
| [`IFA_ADDRESS`](#ifa-address) | const |  |
| [`IFA_LOCAL`](#ifa-local) | const |  |
| [`IFA_LABEL`](#ifa-label) | const |  |
| [`IFA_BROADCAST`](#ifa-broadcast) | const |  |
| [`IFA_ANYCAST`](#ifa-anycast) | const |  |
| [`IFA_CACHEINFO`](#ifa-cacheinfo) | const |  |
| [`IFA_MULTICAST`](#ifa-multicast) | const |  |
| [`IFA_FLAGS`](#ifa-flags) | const |  |
| [`IFA_F_SECONDARY`](#ifa-f-secondary) | const |  |
| [`IFA_F_TEMPORARY`](#ifa-f-temporary) | const |  |
| [`IFA_F_NODAD`](#ifa-f-nodad) | const |  |
| [`IFA_F_OPTIMISTIC`](#ifa-f-optimistic) | const |  |
| [`IFA_F_DADFAILED`](#ifa-f-dadfailed) | const |  |
| [`IFA_F_HOMEADDRESS`](#ifa-f-homeaddress) | const |  |
| [`IFA_F_DEPRECATED`](#ifa-f-deprecated) | const |  |
| [`IFA_F_TENTATIVE`](#ifa-f-tentative) | const |  |
| [`IFA_F_PERMANENT`](#ifa-f-permanent) | const |  |
| [`IFA_F_MANAGETEMPADDR`](#ifa-f-managetempaddr) | const |  |
| [`IFA_F_NOPREFIXROUTE`](#ifa-f-noprefixroute) | const |  |
| [`IFA_F_MCAUTOJOIN`](#ifa-f-mcautojoin) | const |  |
| [`IFA_F_STABLE_PRIVACY`](#ifa-f-stable-privacy) | const |  |
| [`RWF_HIPRI`](#rwf-hipri) | const |  |
| [`RWF_DSYNC`](#rwf-dsync) | const |  |
| [`RWF_SYNC`](#rwf-sync) | const |  |
| [`RWF_NOWAIT`](#rwf-nowait) | const |  |
| [`RWF_APPEND`](#rwf-append) | const |  |
| [`RWF_NOAPPEND`](#rwf-noappend) | const |  |
| [`RWF_ATOMIC`](#rwf-atomic) | const |  |
| [`RWF_DONTCACHE`](#rwf-dontcache) | const |  |
| [`IFLA_UNSPEC`](#ifla-unspec) | const |  |
| [`IFLA_ADDRESS`](#ifla-address) | const |  |
| [`IFLA_BROADCAST`](#ifla-broadcast) | const |  |
| [`IFLA_IFNAME`](#ifla-ifname) | const |  |
| [`IFLA_MTU`](#ifla-mtu) | const |  |
| [`IFLA_LINK`](#ifla-link) | const |  |
| [`IFLA_QDISC`](#ifla-qdisc) | const |  |
| [`IFLA_STATS`](#ifla-stats) | const |  |
| [`IFLA_COST`](#ifla-cost) | const |  |
| [`IFLA_PRIORITY`](#ifla-priority) | const |  |
| [`IFLA_MASTER`](#ifla-master) | const |  |
| [`IFLA_WIRELESS`](#ifla-wireless) | const |  |
| [`IFLA_PROTINFO`](#ifla-protinfo) | const |  |
| [`IFLA_TXQLEN`](#ifla-txqlen) | const |  |
| [`IFLA_MAP`](#ifla-map) | const |  |
| [`IFLA_WEIGHT`](#ifla-weight) | const |  |
| [`IFLA_OPERSTATE`](#ifla-operstate) | const |  |
| [`IFLA_LINKMODE`](#ifla-linkmode) | const |  |
| [`IFLA_LINKINFO`](#ifla-linkinfo) | const |  |
| [`IFLA_NET_NS_PID`](#ifla-net-ns-pid) | const |  |
| [`IFLA_IFALIAS`](#ifla-ifalias) | const |  |
| [`IFLA_NUM_VF`](#ifla-num-vf) | const |  |
| [`IFLA_VFINFO_LIST`](#ifla-vfinfo-list) | const |  |
| [`IFLA_STATS64`](#ifla-stats64) | const |  |
| [`IFLA_VF_PORTS`](#ifla-vf-ports) | const |  |
| [`IFLA_PORT_SELF`](#ifla-port-self) | const |  |
| [`IFLA_AF_SPEC`](#ifla-af-spec) | const |  |
| [`IFLA_GROUP`](#ifla-group) | const |  |
| [`IFLA_NET_NS_FD`](#ifla-net-ns-fd) | const |  |
| [`IFLA_EXT_MASK`](#ifla-ext-mask) | const |  |
| [`IFLA_PROMISCUITY`](#ifla-promiscuity) | const |  |
| [`IFLA_NUM_TX_QUEUES`](#ifla-num-tx-queues) | const |  |
| [`IFLA_NUM_RX_QUEUES`](#ifla-num-rx-queues) | const |  |
| [`IFLA_CARRIER`](#ifla-carrier) | const |  |
| [`IFLA_PHYS_PORT_ID`](#ifla-phys-port-id) | const |  |
| [`IFLA_CARRIER_CHANGES`](#ifla-carrier-changes) | const |  |
| [`IFLA_PHYS_SWITCH_ID`](#ifla-phys-switch-id) | const |  |
| [`IFLA_LINK_NETNSID`](#ifla-link-netnsid) | const |  |
| [`IFLA_PHYS_PORT_NAME`](#ifla-phys-port-name) | const |  |
| [`IFLA_PROTO_DOWN`](#ifla-proto-down) | const |  |
| [`IFLA_GSO_MAX_SEGS`](#ifla-gso-max-segs) | const |  |
| [`IFLA_GSO_MAX_SIZE`](#ifla-gso-max-size) | const |  |
| [`IFLA_PAD`](#ifla-pad) | const |  |
| [`IFLA_XDP`](#ifla-xdp) | const |  |
| [`IFLA_EVENT`](#ifla-event) | const |  |
| [`IFLA_NEW_NETNSID`](#ifla-new-netnsid) | const |  |
| [`IFLA_IF_NETNSID`](#ifla-if-netnsid) | const |  |
| [`IFLA_TARGET_NETNSID`](#ifla-target-netnsid) | const |  |
| [`IFLA_CARRIER_UP_COUNT`](#ifla-carrier-up-count) | const |  |
| [`IFLA_CARRIER_DOWN_COUNT`](#ifla-carrier-down-count) | const |  |
| [`IFLA_NEW_IFINDEX`](#ifla-new-ifindex) | const |  |
| [`IFLA_MIN_MTU`](#ifla-min-mtu) | const |  |
| [`IFLA_MAX_MTU`](#ifla-max-mtu) | const |  |
| [`IFLA_PROP_LIST`](#ifla-prop-list) | const |  |
| [`IFLA_ALT_IFNAME`](#ifla-alt-ifname) | const |  |
| [`IFLA_PERM_ADDRESS`](#ifla-perm-address) | const |  |
| [`IFLA_PROTO_DOWN_REASON`](#ifla-proto-down-reason) | const |  |
| [`IFLA_PARENT_DEV_NAME`](#ifla-parent-dev-name) | const |  |
| [`IFLA_PARENT_DEV_BUS_NAME`](#ifla-parent-dev-bus-name) | const |  |
| [`IFLA_GRO_MAX_SIZE`](#ifla-gro-max-size) | const |  |
| [`IFLA_TSO_MAX_SIZE`](#ifla-tso-max-size) | const |  |
| [`IFLA_TSO_MAX_SEGS`](#ifla-tso-max-segs) | const |  |
| [`IFLA_ALLMULTI`](#ifla-allmulti) | const |  |
| [`IFLA_INFO_UNSPEC`](#ifla-info-unspec) | const |  |
| [`IFLA_INFO_KIND`](#ifla-info-kind) | const |  |
| [`IFLA_INFO_DATA`](#ifla-info-data) | const |  |
| [`IFLA_INFO_XSTATS`](#ifla-info-xstats) | const |  |
| [`IFLA_INFO_SLAVE_KIND`](#ifla-info-slave-kind) | const |  |
| [`IFLA_INFO_SLAVE_DATA`](#ifla-info-slave-data) | const |  |
| [`SEEK_DATA`](#seek-data) | const |  |
| [`SEEK_HOLE`](#seek-hole) | const |  |
| [`MPOL_DEFAULT`](#mpol-default) | const |  |
| [`MPOL_PREFERRED`](#mpol-preferred) | const |  |
| [`MPOL_BIND`](#mpol-bind) | const |  |
| [`MPOL_INTERLEAVE`](#mpol-interleave) | const |  |
| [`MPOL_LOCAL`](#mpol-local) | const |  |
| [`MPOL_F_NUMA_BALANCING`](#mpol-f-numa-balancing) | const |  |
| [`MPOL_F_RELATIVE_NODES`](#mpol-f-relative-nodes) | const |  |
| [`MPOL_F_STATIC_NODES`](#mpol-f-static-nodes) | const |  |
| [`PTHREAD_MUTEX_INITIALIZER`](#pthread-mutex-initializer) | const |  |
| [`PTHREAD_COND_INITIALIZER`](#pthread-cond-initializer) | const |  |
| [`PTHREAD_RWLOCK_INITIALIZER`](#pthread-rwlock-initializer) | const |  |
| [`RENAME_NOREPLACE`](#rename-noreplace) | const |  |
| [`RENAME_EXCHANGE`](#rename-exchange) | const |  |
| [`RENAME_WHITEOUT`](#rename-whiteout) | const |  |
| [`MSG_STAT`](#msg-stat) | const |  |
| [`MSG_INFO`](#msg-info) | const |  |
| [`MSG_NOTIFICATION`](#msg-notification) | const |  |
| [`MSG_NOERROR`](#msg-noerror) | const |  |
| [`MSG_EXCEPT`](#msg-except) | const |  |
| [`MSG_ZEROCOPY`](#msg-zerocopy) | const |  |
| [`SEM_UNDO`](#sem-undo) | const |  |
| [`GETPID`](#getpid) | const |  |
| [`GETVAL`](#getval) | const |  |
| [`GETALL`](#getall) | const |  |
| [`GETNCNT`](#getncnt) | const |  |
| [`GETZCNT`](#getzcnt) | const |  |
| [`SETVAL`](#setval) | const |  |
| [`SETALL`](#setall) | const |  |
| [`SEM_STAT`](#sem-stat) | const |  |
| [`SEM_INFO`](#sem-info) | const |  |
| [`SEM_STAT_ANY`](#sem-stat-any) | const |  |
| [`QFMT_VFS_OLD`](#qfmt-vfs-old) | const |  |
| [`QFMT_VFS_V0`](#qfmt-vfs-v0) | const |  |
| [`QFMT_VFS_V1`](#qfmt-vfs-v1) | const |  |
| [`EFD_SEMAPHORE`](#efd-semaphore) | const |  |
| [`RB_AUTOBOOT`](#rb-autoboot) | const |  |
| [`RB_HALT_SYSTEM`](#rb-halt-system) | const |  |
| [`RB_ENABLE_CAD`](#rb-enable-cad) | const |  |
| [`RB_DISABLE_CAD`](#rb-disable-cad) | const |  |
| [`RB_POWER_OFF`](#rb-power-off) | const |  |
| [`RB_SW_SUSPEND`](#rb-sw-suspend) | const |  |
| [`RB_KEXEC`](#rb-kexec) | const |  |
| [`SYNC_FILE_RANGE_WAIT_BEFORE`](#sync-file-range-wait-before) | const |  |
| [`SYNC_FILE_RANGE_WRITE`](#sync-file-range-write) | const |  |
| [`SYNC_FILE_RANGE_WAIT_AFTER`](#sync-file-range-wait-after) | const |  |
| [`MREMAP_MAYMOVE`](#mremap-maymove) | const |  |
| [`MREMAP_FIXED`](#mremap-fixed) | const |  |
| [`MREMAP_DONTUNMAP`](#mremap-dontunmap) | const |  |
| [`NSIO`](#nsio) | const |  |
| [`NS_GET_USERNS`](#ns-get-userns) | const |  |
| [`NS_GET_PARENT`](#ns-get-parent) | const |  |
| [`NS_GET_NSTYPE`](#ns-get-nstype) | const |  |
| [`NS_GET_OWNER_UID`](#ns-get-owner-uid) | const |  |
| [`NS_GET_MNTNS_ID`](#ns-get-mntns-id) | const |  |
| [`NS_GET_PID_FROM_PIDNS`](#ns-get-pid-from-pidns) | const |  |
| [`NS_GET_TGID_FROM_PIDNS`](#ns-get-tgid-from-pidns) | const |  |
| [`NS_GET_PID_IN_PIDNS`](#ns-get-pid-in-pidns) | const |  |
| [`NS_GET_TGID_IN_PIDNS`](#ns-get-tgid-in-pidns) | const |  |
| [`MNT_NS_INFO_SIZE_VER0`](#mnt-ns-info-size-ver0) | const |  |
| [`NS_MNT_GET_INFO`](#ns-mnt-get-info) | const |  |
| [`NS_MNT_GET_NEXT`](#ns-mnt-get-next) | const |  |
| [`NS_MNT_GET_PREV`](#ns-mnt-get-prev) | const |  |
| [`PR_SET_MDWE`](#pr-set-mdwe) | const |  |
| [`PR_GET_MDWE`](#pr-get-mdwe) | const |  |
| [`PR_MDWE_REFUSE_EXEC_GAIN`](#pr-mdwe-refuse-exec-gain) | const |  |
| [`PR_MDWE_NO_INHERIT`](#pr-mdwe-no-inherit) | const |  |
| [`GRND_NONBLOCK`](#grnd-nonblock) | const |  |
| [`GRND_RANDOM`](#grnd-random) | const |  |
| [`GRND_INSECURE`](#grnd-insecure) | const |  |
| [`SECCOMP_MODE_DISABLED`](#seccomp-mode-disabled) | const |  |
| [`SECCOMP_MODE_STRICT`](#seccomp-mode-strict) | const |  |
| [`SECCOMP_MODE_FILTER`](#seccomp-mode-filter) | const |  |
| [`SECCOMP_SET_MODE_STRICT`](#seccomp-set-mode-strict) | const |  |
| [`SECCOMP_SET_MODE_FILTER`](#seccomp-set-mode-filter) | const |  |
| [`SECCOMP_GET_ACTION_AVAIL`](#seccomp-get-action-avail) | const |  |
| [`SECCOMP_GET_NOTIF_SIZES`](#seccomp-get-notif-sizes) | const |  |
| [`SECCOMP_FILTER_FLAG_TSYNC`](#seccomp-filter-flag-tsync) | const |  |
| [`SECCOMP_FILTER_FLAG_LOG`](#seccomp-filter-flag-log) | const |  |
| [`SECCOMP_FILTER_FLAG_SPEC_ALLOW`](#seccomp-filter-flag-spec-allow) | const |  |
| [`SECCOMP_FILTER_FLAG_NEW_LISTENER`](#seccomp-filter-flag-new-listener) | const |  |
| [`SECCOMP_FILTER_FLAG_TSYNC_ESRCH`](#seccomp-filter-flag-tsync-esrch) | const |  |
| [`SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV`](#seccomp-filter-flag-wait-killable-recv) | const |  |
| [`SECCOMP_RET_KILL_PROCESS`](#seccomp-ret-kill-process) | const |  |
| [`SECCOMP_RET_KILL_THREAD`](#seccomp-ret-kill-thread) | const |  |
| [`SECCOMP_RET_KILL`](#seccomp-ret-kill) | const |  |
| [`SECCOMP_RET_TRAP`](#seccomp-ret-trap) | const |  |
| [`SECCOMP_RET_ERRNO`](#seccomp-ret-errno) | const |  |
| [`SECCOMP_RET_USER_NOTIF`](#seccomp-ret-user-notif) | const |  |
| [`SECCOMP_RET_TRACE`](#seccomp-ret-trace) | const |  |
| [`SECCOMP_RET_LOG`](#seccomp-ret-log) | const |  |
| [`SECCOMP_RET_ALLOW`](#seccomp-ret-allow) | const |  |
| [`SECCOMP_RET_ACTION_FULL`](#seccomp-ret-action-full) | const |  |
| [`SECCOMP_RET_ACTION`](#seccomp-ret-action) | const |  |
| [`SECCOMP_RET_DATA`](#seccomp-ret-data) | const |  |
| [`SECCOMP_USER_NOTIF_FLAG_CONTINUE`](#seccomp-user-notif-flag-continue) | const |  |
| [`SECCOMP_ADDFD_FLAG_SETFD`](#seccomp-addfd-flag-setfd) | const |  |
| [`SECCOMP_ADDFD_FLAG_SEND`](#seccomp-addfd-flag-send) | const |  |
| [`TFD_CLOEXEC`](#tfd-cloexec) | const |  |
| [`TFD_NONBLOCK`](#tfd-nonblock) | const |  |
| [`TFD_TIMER_ABSTIME`](#tfd-timer-abstime) | const |  |
| [`TFD_TIMER_CANCEL_ON_SET`](#tfd-timer-cancel-on-set) | const |  |
| [`FALLOC_FL_KEEP_SIZE`](#falloc-fl-keep-size) | const |  |
| [`FALLOC_FL_PUNCH_HOLE`](#falloc-fl-punch-hole) | const |  |
| [`FALLOC_FL_COLLAPSE_RANGE`](#falloc-fl-collapse-range) | const |  |
| [`FALLOC_FL_ZERO_RANGE`](#falloc-fl-zero-range) | const |  |
| [`FALLOC_FL_INSERT_RANGE`](#falloc-fl-insert-range) | const |  |
| [`FALLOC_FL_UNSHARE_RANGE`](#falloc-fl-unshare-range) | const |  |
| [`ENOATTR`](#enoattr) | const |  |
| [`SO_ORIGINAL_DST`](#so-original-dst) | const |  |
| [`IP_RECVFRAGSIZE`](#ip-recvfragsize) | const |  |
| [`IPV6_FLOWINFO`](#ipv6-flowinfo) | const |  |
| [`IPV6_FLOWLABEL_MGR`](#ipv6-flowlabel-mgr) | const |  |
| [`IPV6_FLOWINFO_SEND`](#ipv6-flowinfo-send) | const |  |
| [`IPV6_RECVFRAGSIZE`](#ipv6-recvfragsize) | const |  |
| [`IPV6_FREEBIND`](#ipv6-freebind) | const |  |
| [`IPV6_FLOWINFO_FLOWLABEL`](#ipv6-flowinfo-flowlabel) | const |  |
| [`IPV6_FLOWINFO_PRIORITY`](#ipv6-flowinfo-priority) | const |  |
| [`SK_MEMINFO_RMEM_ALLOC`](#sk-meminfo-rmem-alloc) | const |  |
| [`SK_MEMINFO_RCVBUF`](#sk-meminfo-rcvbuf) | const |  |
| [`SK_MEMINFO_WMEM_ALLOC`](#sk-meminfo-wmem-alloc) | const |  |
| [`SK_MEMINFO_SNDBUF`](#sk-meminfo-sndbuf) | const |  |
| [`SK_MEMINFO_FWD_ALLOC`](#sk-meminfo-fwd-alloc) | const |  |
| [`SK_MEMINFO_WMEM_QUEUED`](#sk-meminfo-wmem-queued) | const |  |
| [`SK_MEMINFO_OPTMEM`](#sk-meminfo-optmem) | const |  |
| [`SK_MEMINFO_BACKLOG`](#sk-meminfo-backlog) | const |  |
| [`SK_MEMINFO_DROPS`](#sk-meminfo-drops) | const |  |
| [`CLOSE_RANGE_UNSHARE`](#close-range-unshare) | const |  |
| [`CLOSE_RANGE_CLOEXEC`](#close-range-cloexec) | const |  |
| [`SKF_AD_OFF`](#skf-ad-off) | const |  |
| [`SKF_AD_PROTOCOL`](#skf-ad-protocol) | const |  |
| [`SKF_AD_PKTTYPE`](#skf-ad-pkttype) | const |  |
| [`SKF_AD_IFINDEX`](#skf-ad-ifindex) | const |  |
| [`SKF_AD_NLATTR`](#skf-ad-nlattr) | const |  |
| [`SKF_AD_NLATTR_NEST`](#skf-ad-nlattr-nest) | const |  |
| [`SKF_AD_MARK`](#skf-ad-mark) | const |  |
| [`SKF_AD_QUEUE`](#skf-ad-queue) | const |  |
| [`SKF_AD_HATYPE`](#skf-ad-hatype) | const |  |
| [`SKF_AD_RXHASH`](#skf-ad-rxhash) | const |  |
| [`SKF_AD_CPU`](#skf-ad-cpu) | const |  |
| [`SKF_AD_ALU_XOR_X`](#skf-ad-alu-xor-x) | const |  |
| [`SKF_AD_VLAN_TAG`](#skf-ad-vlan-tag) | const |  |
| [`SKF_AD_VLAN_TAG_PRESENT`](#skf-ad-vlan-tag-present) | const |  |
| [`SKF_AD_PAY_OFFSET`](#skf-ad-pay-offset) | const |  |
| [`SKF_AD_RANDOM`](#skf-ad-random) | const |  |
| [`SKF_AD_VLAN_TPID`](#skf-ad-vlan-tpid) | const |  |
| [`SKF_AD_MAX`](#skf-ad-max) | const |  |
| [`SKF_NET_OFF`](#skf-net-off) | const |  |
| [`SKF_LL_OFF`](#skf-ll-off) | const |  |
| [`BPF_NET_OFF`](#bpf-net-off) | const |  |
| [`BPF_LL_OFF`](#bpf-ll-off) | const |  |
| [`BPF_MEMWORDS`](#bpf-memwords) | const |  |
| [`BPF_MAXINSNS`](#bpf-maxinsns) | const |  |
| [`BPF_LD`](#bpf-ld) | const |  |
| [`BPF_LDX`](#bpf-ldx) | const |  |
| [`BPF_ST`](#bpf-st) | const |  |
| [`BPF_STX`](#bpf-stx) | const |  |
| [`BPF_ALU`](#bpf-alu) | const |  |
| [`BPF_JMP`](#bpf-jmp) | const |  |
| [`BPF_RET`](#bpf-ret) | const |  |
| [`BPF_MISC`](#bpf-misc) | const |  |
| [`BPF_W`](#bpf-w) | const |  |
| [`BPF_H`](#bpf-h) | const |  |
| [`BPF_B`](#bpf-b) | const |  |
| [`BPF_IMM`](#bpf-imm) | const |  |
| [`BPF_ABS`](#bpf-abs) | const |  |
| [`BPF_IND`](#bpf-ind) | const |  |
| [`BPF_MEM`](#bpf-mem) | const |  |
| [`BPF_LEN`](#bpf-len) | const |  |
| [`BPF_MSH`](#bpf-msh) | const |  |
| [`BPF_ADD`](#bpf-add) | const |  |
| [`BPF_SUB`](#bpf-sub) | const |  |
| [`BPF_MUL`](#bpf-mul) | const |  |
| [`BPF_DIV`](#bpf-div) | const |  |
| [`BPF_OR`](#bpf-or) | const |  |
| [`BPF_AND`](#bpf-and) | const |  |
| [`BPF_LSH`](#bpf-lsh) | const |  |
| [`BPF_RSH`](#bpf-rsh) | const |  |
| [`BPF_NEG`](#bpf-neg) | const |  |
| [`BPF_MOD`](#bpf-mod) | const |  |
| [`BPF_XOR`](#bpf-xor) | const |  |
| [`BPF_JA`](#bpf-ja) | const |  |
| [`BPF_JEQ`](#bpf-jeq) | const |  |
| [`BPF_JGT`](#bpf-jgt) | const |  |
| [`BPF_JGE`](#bpf-jge) | const |  |
| [`BPF_JSET`](#bpf-jset) | const |  |
| [`BPF_K`](#bpf-k) | const |  |
| [`BPF_X`](#bpf-x) | const |  |
| [`BPF_A`](#bpf-a) | const |  |
| [`BPF_TAX`](#bpf-tax) | const |  |
| [`BPF_TXA`](#bpf-txa) | const |  |
| [`RESOLVE_NO_XDEV`](#resolve-no-xdev) | const |  |
| [`RESOLVE_NO_MAGICLINKS`](#resolve-no-magiclinks) | const |  |
| [`RESOLVE_NO_SYMLINKS`](#resolve-no-symlinks) | const |  |
| [`RESOLVE_BENEATH`](#resolve-beneath) | const |  |
| [`RESOLVE_IN_ROOT`](#resolve-in-root) | const |  |
| [`RESOLVE_CACHED`](#resolve-cached) | const |  |
| [`ETH_ALEN`](#eth-alen) | const |  |
| [`ETH_HLEN`](#eth-hlen) | const |  |
| [`ETH_ZLEN`](#eth-zlen) | const |  |
| [`ETH_DATA_LEN`](#eth-data-len) | const |  |
| [`ETH_FRAME_LEN`](#eth-frame-len) | const |  |
| [`ETH_FCS_LEN`](#eth-fcs-len) | const |  |
| [`ETH_P_LOOP`](#eth-p-loop) | const |  |
| [`ETH_P_PUP`](#eth-p-pup) | const |  |
| [`ETH_P_PUPAT`](#eth-p-pupat) | const |  |
| [`ETH_P_IP`](#eth-p-ip) | const |  |
| [`ETH_P_X25`](#eth-p-x25) | const |  |
| [`ETH_P_ARP`](#eth-p-arp) | const |  |
| [`ETH_P_BPQ`](#eth-p-bpq) | const |  |
| [`ETH_P_IEEEPUP`](#eth-p-ieeepup) | const |  |
| [`ETH_P_IEEEPUPAT`](#eth-p-ieeepupat) | const |  |
| [`ETH_P_BATMAN`](#eth-p-batman) | const |  |
| [`ETH_P_DEC`](#eth-p-dec) | const |  |
| [`ETH_P_DNA_DL`](#eth-p-dna-dl) | const |  |
| [`ETH_P_DNA_RC`](#eth-p-dna-rc) | const |  |
| [`ETH_P_DNA_RT`](#eth-p-dna-rt) | const |  |
| [`ETH_P_LAT`](#eth-p-lat) | const |  |
| [`ETH_P_DIAG`](#eth-p-diag) | const |  |
| [`ETH_P_CUST`](#eth-p-cust) | const |  |
| [`ETH_P_SCA`](#eth-p-sca) | const |  |
| [`ETH_P_TEB`](#eth-p-teb) | const |  |
| [`ETH_P_RARP`](#eth-p-rarp) | const |  |
| [`ETH_P_ATALK`](#eth-p-atalk) | const |  |
| [`ETH_P_AARP`](#eth-p-aarp) | const |  |
| [`ETH_P_8021Q`](#eth-p-8021q) | const |  |
| [`ETH_P_IPX`](#eth-p-ipx) | const |  |
| [`ETH_P_IPV6`](#eth-p-ipv6) | const |  |
| [`ETH_P_PAUSE`](#eth-p-pause) | const |  |
| [`ETH_P_SLOW`](#eth-p-slow) | const |  |
| [`ETH_P_WCCP`](#eth-p-wccp) | const |  |
| [`ETH_P_MPLS_UC`](#eth-p-mpls-uc) | const |  |
| [`ETH_P_MPLS_MC`](#eth-p-mpls-mc) | const |  |
| [`ETH_P_ATMMPOA`](#eth-p-atmmpoa) | const |  |
| [`ETH_P_PPP_DISC`](#eth-p-ppp-disc) | const |  |
| [`ETH_P_PPP_SES`](#eth-p-ppp-ses) | const |  |
| [`ETH_P_LINK_CTL`](#eth-p-link-ctl) | const |  |
| [`ETH_P_ATMFATE`](#eth-p-atmfate) | const |  |
| [`ETH_P_PAE`](#eth-p-pae) | const |  |
| [`ETH_P_AOE`](#eth-p-aoe) | const |  |
| [`ETH_P_8021AD`](#eth-p-8021ad) | const |  |
| [`ETH_P_802_EX1`](#eth-p-802-ex1) | const |  |
| [`ETH_P_TIPC`](#eth-p-tipc) | const |  |
| [`ETH_P_MACSEC`](#eth-p-macsec) | const |  |
| [`ETH_P_8021AH`](#eth-p-8021ah) | const |  |
| [`ETH_P_MVRP`](#eth-p-mvrp) | const |  |
| [`ETH_P_1588`](#eth-p-1588) | const |  |
| [`ETH_P_PRP`](#eth-p-prp) | const |  |
| [`ETH_P_FCOE`](#eth-p-fcoe) | const |  |
| [`ETH_P_TDLS`](#eth-p-tdls) | const |  |
| [`ETH_P_FIP`](#eth-p-fip) | const |  |
| [`ETH_P_80221`](#eth-p-80221) | const |  |
| [`ETH_P_LOOPBACK`](#eth-p-loopback) | const |  |
| [`ETH_P_QINQ1`](#eth-p-qinq1) | const |  |
| [`ETH_P_QINQ2`](#eth-p-qinq2) | const |  |
| [`ETH_P_QINQ3`](#eth-p-qinq3) | const |  |
| [`ETH_P_EDSA`](#eth-p-edsa) | const |  |
| [`ETH_P_AF_IUCV`](#eth-p-af-iucv) | const |  |
| [`ETH_P_802_3_MIN`](#eth-p-802-3-min) | const |  |
| [`ETH_P_802_3`](#eth-p-802-3) | const |  |
| [`ETH_P_AX25`](#eth-p-ax25) | const |  |
| [`ETH_P_ALL`](#eth-p-all) | const |  |
| [`ETH_P_802_2`](#eth-p-802-2) | const |  |
| [`ETH_P_SNAP`](#eth-p-snap) | const |  |
| [`ETH_P_DDCMP`](#eth-p-ddcmp) | const |  |
| [`ETH_P_WAN_PPP`](#eth-p-wan-ppp) | const |  |
| [`ETH_P_PPP_MP`](#eth-p-ppp-mp) | const |  |
| [`ETH_P_LOCALTALK`](#eth-p-localtalk) | const |  |
| [`ETH_P_CANFD`](#eth-p-canfd) | const |  |
| [`ETH_P_PPPTALK`](#eth-p-ppptalk) | const |  |
| [`ETH_P_TR_802_2`](#eth-p-tr-802-2) | const |  |
| [`ETH_P_MOBITEX`](#eth-p-mobitex) | const |  |
| [`ETH_P_CONTROL`](#eth-p-control) | const |  |
| [`ETH_P_IRDA`](#eth-p-irda) | const |  |
| [`ETH_P_ECONET`](#eth-p-econet) | const |  |
| [`ETH_P_HDLC`](#eth-p-hdlc) | const |  |
| [`ETH_P_ARCNET`](#eth-p-arcnet) | const |  |
| [`ETH_P_DSA`](#eth-p-dsa) | const |  |
| [`ETH_P_TRAILER`](#eth-p-trailer) | const |  |
| [`ETH_P_PHONET`](#eth-p-phonet) | const |  |
| [`ETH_P_IEEE802154`](#eth-p-ieee802154) | const |  |
| [`ETH_P_CAIF`](#eth-p-caif) | const |  |
| [`POSIX_SPAWN_RESETIDS`](#posix-spawn-resetids) | const |  |
| [`POSIX_SPAWN_SETPGROUP`](#posix-spawn-setpgroup) | const |  |
| [`POSIX_SPAWN_SETSIGDEF`](#posix-spawn-setsigdef) | const |  |
| [`POSIX_SPAWN_SETSIGMASK`](#posix-spawn-setsigmask) | const |  |
| [`POSIX_SPAWN_SETSCHEDPARAM`](#posix-spawn-setschedparam) | const |  |
| [`POSIX_SPAWN_SETSCHEDULER`](#posix-spawn-setscheduler) | const |  |
| [`NFNLGRP_NONE`](#nfnlgrp-none) | const |  |
| [`NFNLGRP_CONNTRACK_NEW`](#nfnlgrp-conntrack-new) | const |  |
| [`NFNLGRP_CONNTRACK_UPDATE`](#nfnlgrp-conntrack-update) | const |  |
| [`NFNLGRP_CONNTRACK_DESTROY`](#nfnlgrp-conntrack-destroy) | const |  |
| [`NFNLGRP_CONNTRACK_EXP_NEW`](#nfnlgrp-conntrack-exp-new) | const |  |
| [`NFNLGRP_CONNTRACK_EXP_UPDATE`](#nfnlgrp-conntrack-exp-update) | const |  |
| [`NFNLGRP_CONNTRACK_EXP_DESTROY`](#nfnlgrp-conntrack-exp-destroy) | const |  |
| [`NFNLGRP_NFTABLES`](#nfnlgrp-nftables) | const |  |
| [`NFNLGRP_ACCT_QUOTA`](#nfnlgrp-acct-quota) | const |  |
| [`NFNLGRP_NFTRACE`](#nfnlgrp-nftrace) | const |  |
| [`NFNETLINK_V0`](#nfnetlink-v0) | const |  |
| [`NFNL_SUBSYS_NONE`](#nfnl-subsys-none) | const |  |
| [`NFNL_SUBSYS_CTNETLINK`](#nfnl-subsys-ctnetlink) | const |  |
| [`NFNL_SUBSYS_CTNETLINK_EXP`](#nfnl-subsys-ctnetlink-exp) | const |  |
| [`NFNL_SUBSYS_QUEUE`](#nfnl-subsys-queue) | const |  |
| [`NFNL_SUBSYS_ULOG`](#nfnl-subsys-ulog) | const |  |
| [`NFNL_SUBSYS_OSF`](#nfnl-subsys-osf) | const |  |
| [`NFNL_SUBSYS_IPSET`](#nfnl-subsys-ipset) | const |  |
| [`NFNL_SUBSYS_ACCT`](#nfnl-subsys-acct) | const |  |
| [`NFNL_SUBSYS_CTNETLINK_TIMEOUT`](#nfnl-subsys-ctnetlink-timeout) | const |  |
| [`NFNL_SUBSYS_CTHELPER`](#nfnl-subsys-cthelper) | const |  |
| [`NFNL_SUBSYS_NFTABLES`](#nfnl-subsys-nftables) | const |  |
| [`NFNL_SUBSYS_NFT_COMPAT`](#nfnl-subsys-nft-compat) | const |  |
| [`NFNL_SUBSYS_HOOK`](#nfnl-subsys-hook) | const |  |
| [`NFNL_SUBSYS_COUNT`](#nfnl-subsys-count) | const |  |
| [`NFNL_MSG_BATCH_BEGIN`](#nfnl-msg-batch-begin) | const |  |
| [`NFNL_MSG_BATCH_END`](#nfnl-msg-batch-end) | const |  |
| [`NFNL_BATCH_UNSPEC`](#nfnl-batch-unspec) | const |  |
| [`NFNL_BATCH_GENID`](#nfnl-batch-genid) | const |  |
| [`NFULNL_MSG_PACKET`](#nfulnl-msg-packet) | const |  |
| [`NFULNL_MSG_CONFIG`](#nfulnl-msg-config) | const |  |
| [`NFULA_VLAN_UNSPEC`](#nfula-vlan-unspec) | const |  |
| [`NFULA_VLAN_PROTO`](#nfula-vlan-proto) | const |  |
| [`NFULA_VLAN_TCI`](#nfula-vlan-tci) | const |  |
| [`NFULA_UNSPEC`](#nfula-unspec) | const |  |
| [`NFULA_PACKET_HDR`](#nfula-packet-hdr) | const |  |
| [`NFULA_MARK`](#nfula-mark) | const |  |
| [`NFULA_TIMESTAMP`](#nfula-timestamp) | const |  |
| [`NFULA_IFINDEX_INDEV`](#nfula-ifindex-indev) | const |  |
| [`NFULA_IFINDEX_OUTDEV`](#nfula-ifindex-outdev) | const |  |
| [`NFULA_IFINDEX_PHYSINDEV`](#nfula-ifindex-physindev) | const |  |
| [`NFULA_IFINDEX_PHYSOUTDEV`](#nfula-ifindex-physoutdev) | const |  |
| [`NFULA_HWADDR`](#nfula-hwaddr) | const |  |
| [`NFULA_PAYLOAD`](#nfula-payload) | const |  |
| [`NFULA_PREFIX`](#nfula-prefix) | const |  |
| [`NFULA_UID`](#nfula-uid) | const |  |
| [`NFULA_SEQ`](#nfula-seq) | const |  |
| [`NFULA_SEQ_GLOBAL`](#nfula-seq-global) | const |  |
| [`NFULA_GID`](#nfula-gid) | const |  |
| [`NFULA_HWTYPE`](#nfula-hwtype) | const |  |
| [`NFULA_HWHEADER`](#nfula-hwheader) | const |  |
| [`NFULA_HWLEN`](#nfula-hwlen) | const |  |
| [`NFULA_CT`](#nfula-ct) | const |  |
| [`NFULA_CT_INFO`](#nfula-ct-info) | const |  |
| [`NFULA_VLAN`](#nfula-vlan) | const |  |
| [`NFULA_L2HDR`](#nfula-l2hdr) | const |  |
| [`NFULNL_CFG_CMD_NONE`](#nfulnl-cfg-cmd-none) | const |  |
| [`NFULNL_CFG_CMD_BIND`](#nfulnl-cfg-cmd-bind) | const |  |
| [`NFULNL_CFG_CMD_UNBIND`](#nfulnl-cfg-cmd-unbind) | const |  |
| [`NFULNL_CFG_CMD_PF_BIND`](#nfulnl-cfg-cmd-pf-bind) | const |  |
| [`NFULNL_CFG_CMD_PF_UNBIND`](#nfulnl-cfg-cmd-pf-unbind) | const |  |
| [`NFULA_CFG_UNSPEC`](#nfula-cfg-unspec) | const |  |
| [`NFULA_CFG_CMD`](#nfula-cfg-cmd) | const |  |
| [`NFULA_CFG_MODE`](#nfula-cfg-mode) | const |  |
| [`NFULA_CFG_NLBUFSIZ`](#nfula-cfg-nlbufsiz) | const |  |
| [`NFULA_CFG_TIMEOUT`](#nfula-cfg-timeout) | const |  |
| [`NFULA_CFG_QTHRESH`](#nfula-cfg-qthresh) | const |  |
| [`NFULA_CFG_FLAGS`](#nfula-cfg-flags) | const |  |
| [`NFULNL_COPY_NONE`](#nfulnl-copy-none) | const |  |
| [`NFULNL_COPY_META`](#nfulnl-copy-meta) | const |  |
| [`NFULNL_COPY_PACKET`](#nfulnl-copy-packet) | const |  |
| [`NFULNL_CFG_F_SEQ`](#nfulnl-cfg-f-seq) | const |  |
| [`NFULNL_CFG_F_SEQ_GLOBAL`](#nfulnl-cfg-f-seq-global) | const |  |
| [`NFULNL_CFG_F_CONNTRACK`](#nfulnl-cfg-f-conntrack) | const |  |
| [`NFQNL_MSG_PACKET`](#nfqnl-msg-packet) | const |  |
| [`NFQNL_MSG_VERDICT`](#nfqnl-msg-verdict) | const |  |
| [`NFQNL_MSG_CONFIG`](#nfqnl-msg-config) | const |  |
| [`NFQNL_MSG_VERDICT_BATCH`](#nfqnl-msg-verdict-batch) | const |  |
| [`NFQA_UNSPEC`](#nfqa-unspec) | const |  |
| [`NFQA_PACKET_HDR`](#nfqa-packet-hdr) | const |  |
| [`NFQA_VERDICT_HDR`](#nfqa-verdict-hdr) | const |  |
| [`NFQA_MARK`](#nfqa-mark) | const |  |
| [`NFQA_TIMESTAMP`](#nfqa-timestamp) | const |  |
| [`NFQA_IFINDEX_INDEV`](#nfqa-ifindex-indev) | const |  |
| [`NFQA_IFINDEX_OUTDEV`](#nfqa-ifindex-outdev) | const |  |
| [`NFQA_IFINDEX_PHYSINDEV`](#nfqa-ifindex-physindev) | const |  |
| [`NFQA_IFINDEX_PHYSOUTDEV`](#nfqa-ifindex-physoutdev) | const |  |
| [`NFQA_HWADDR`](#nfqa-hwaddr) | const |  |
| [`NFQA_PAYLOAD`](#nfqa-payload) | const |  |
| [`NFQA_CT`](#nfqa-ct) | const |  |
| [`NFQA_CT_INFO`](#nfqa-ct-info) | const |  |
| [`NFQA_CAP_LEN`](#nfqa-cap-len) | const |  |
| [`NFQA_SKB_INFO`](#nfqa-skb-info) | const |  |
| [`NFQA_EXP`](#nfqa-exp) | const |  |
| [`NFQA_UID`](#nfqa-uid) | const |  |
| [`NFQA_GID`](#nfqa-gid) | const |  |
| [`NFQA_SECCTX`](#nfqa-secctx) | const |  |
| [`NFQA_VLAN`](#nfqa-vlan) | const |  |
| [`NFQA_L2HDR`](#nfqa-l2hdr) | const |  |
| [`NFQA_PRIORITY`](#nfqa-priority) | const |  |
| [`NFQA_VLAN_UNSPEC`](#nfqa-vlan-unspec) | const |  |
| [`NFQA_VLAN_PROTO`](#nfqa-vlan-proto) | const |  |
| [`NFQA_VLAN_TCI`](#nfqa-vlan-tci) | const |  |
| [`NFQNL_CFG_CMD_NONE`](#nfqnl-cfg-cmd-none) | const |  |
| [`NFQNL_CFG_CMD_BIND`](#nfqnl-cfg-cmd-bind) | const |  |
| [`NFQNL_CFG_CMD_UNBIND`](#nfqnl-cfg-cmd-unbind) | const |  |
| [`NFQNL_CFG_CMD_PF_BIND`](#nfqnl-cfg-cmd-pf-bind) | const |  |
| [`NFQNL_CFG_CMD_PF_UNBIND`](#nfqnl-cfg-cmd-pf-unbind) | const |  |
| [`NFQNL_COPY_NONE`](#nfqnl-copy-none) | const |  |
| [`NFQNL_COPY_META`](#nfqnl-copy-meta) | const |  |
| [`NFQNL_COPY_PACKET`](#nfqnl-copy-packet) | const |  |
| [`NFQA_CFG_UNSPEC`](#nfqa-cfg-unspec) | const |  |
| [`NFQA_CFG_CMD`](#nfqa-cfg-cmd) | const |  |
| [`NFQA_CFG_PARAMS`](#nfqa-cfg-params) | const |  |
| [`NFQA_CFG_QUEUE_MAXLEN`](#nfqa-cfg-queue-maxlen) | const |  |
| [`NFQA_CFG_MASK`](#nfqa-cfg-mask) | const |  |
| [`NFQA_CFG_FLAGS`](#nfqa-cfg-flags) | const |  |
| [`NFQA_CFG_F_FAIL_OPEN`](#nfqa-cfg-f-fail-open) | const |  |
| [`NFQA_CFG_F_CONNTRACK`](#nfqa-cfg-f-conntrack) | const |  |
| [`NFQA_CFG_F_GSO`](#nfqa-cfg-f-gso) | const |  |
| [`NFQA_CFG_F_UID_GID`](#nfqa-cfg-f-uid-gid) | const |  |
| [`NFQA_CFG_F_SECCTX`](#nfqa-cfg-f-secctx) | const |  |
| [`NFQA_CFG_F_MAX`](#nfqa-cfg-f-max) | const |  |
| [`NFQA_SKB_CSUMNOTREADY`](#nfqa-skb-csumnotready) | const |  |
| [`NFQA_SKB_GSO`](#nfqa-skb-gso) | const |  |
| [`NFQA_SKB_CSUM_NOTVERIFIED`](#nfqa-skb-csum-notverified) | const |  |
| [`GENL_NAMSIZ`](#genl-namsiz) | const |  |
| [`GENL_MIN_ID`](#genl-min-id) | const |  |
| [`GENL_MAX_ID`](#genl-max-id) | const |  |
| [`GENL_ADMIN_PERM`](#genl-admin-perm) | const |  |
| [`GENL_CMD_CAP_DO`](#genl-cmd-cap-do) | const |  |
| [`GENL_CMD_CAP_DUMP`](#genl-cmd-cap-dump) | const |  |
| [`GENL_CMD_CAP_HASPOL`](#genl-cmd-cap-haspol) | const |  |
| [`GENL_ID_CTRL`](#genl-id-ctrl) | const |  |
| [`CTRL_CMD_UNSPEC`](#ctrl-cmd-unspec) | const |  |
| [`CTRL_CMD_NEWFAMILY`](#ctrl-cmd-newfamily) | const |  |
| [`CTRL_CMD_DELFAMILY`](#ctrl-cmd-delfamily) | const |  |
| [`CTRL_CMD_GETFAMILY`](#ctrl-cmd-getfamily) | const |  |
| [`CTRL_CMD_NEWOPS`](#ctrl-cmd-newops) | const |  |
| [`CTRL_CMD_DELOPS`](#ctrl-cmd-delops) | const |  |
| [`CTRL_CMD_GETOPS`](#ctrl-cmd-getops) | const |  |
| [`CTRL_CMD_NEWMCAST_GRP`](#ctrl-cmd-newmcast-grp) | const |  |
| [`CTRL_CMD_DELMCAST_GRP`](#ctrl-cmd-delmcast-grp) | const |  |
| [`CTRL_CMD_GETMCAST_GRP`](#ctrl-cmd-getmcast-grp) | const |  |
| [`CTRL_ATTR_UNSPEC`](#ctrl-attr-unspec) | const |  |
| [`CTRL_ATTR_FAMILY_ID`](#ctrl-attr-family-id) | const |  |
| [`CTRL_ATTR_FAMILY_NAME`](#ctrl-attr-family-name) | const |  |
| [`CTRL_ATTR_VERSION`](#ctrl-attr-version) | const |  |
| [`CTRL_ATTR_HDRSIZE`](#ctrl-attr-hdrsize) | const |  |
| [`CTRL_ATTR_MAXATTR`](#ctrl-attr-maxattr) | const |  |
| [`CTRL_ATTR_OPS`](#ctrl-attr-ops) | const |  |
| [`CTRL_ATTR_MCAST_GROUPS`](#ctrl-attr-mcast-groups) | const |  |
| [`CTRL_ATTR_OP_UNSPEC`](#ctrl-attr-op-unspec) | const |  |
| [`CTRL_ATTR_OP_ID`](#ctrl-attr-op-id) | const |  |
| [`CTRL_ATTR_OP_FLAGS`](#ctrl-attr-op-flags) | const |  |
| [`CTRL_ATTR_MCAST_GRP_UNSPEC`](#ctrl-attr-mcast-grp-unspec) | const |  |
| [`CTRL_ATTR_MCAST_GRP_NAME`](#ctrl-attr-mcast-grp-name) | const |  |
| [`CTRL_ATTR_MCAST_GRP_ID`](#ctrl-attr-mcast-grp-id) | const |  |
| [`PACKET_FANOUT`](#packet-fanout) | const |  |
| [`PACKET_TX_HAS_OFF`](#packet-tx-has-off) | const |  |
| [`PACKET_QDISC_BYPASS`](#packet-qdisc-bypass) | const |  |
| [`PACKET_ROLLOVER_STATS`](#packet-rollover-stats) | const |  |
| [`PACKET_FANOUT_DATA`](#packet-fanout-data) | const |  |
| [`PACKET_IGNORE_OUTGOING`](#packet-ignore-outgoing) | const |  |
| [`PACKET_VNET_HDR_SZ`](#packet-vnet-hdr-sz) | const |  |
| [`PACKET_FANOUT_HASH`](#packet-fanout-hash) | const |  |
| [`PACKET_FANOUT_LB`](#packet-fanout-lb) | const |  |
| [`PACKET_FANOUT_CPU`](#packet-fanout-cpu) | const |  |
| [`PACKET_FANOUT_ROLLOVER`](#packet-fanout-rollover) | const |  |
| [`PACKET_FANOUT_RND`](#packet-fanout-rnd) | const |  |
| [`PACKET_FANOUT_QM`](#packet-fanout-qm) | const |  |
| [`PACKET_FANOUT_CBPF`](#packet-fanout-cbpf) | const |  |
| [`PACKET_FANOUT_EBPF`](#packet-fanout-ebpf) | const |  |
| [`PACKET_FANOUT_FLAG_ROLLOVER`](#packet-fanout-flag-rollover) | const |  |
| [`PACKET_FANOUT_FLAG_UNIQUEID`](#packet-fanout-flag-uniqueid) | const |  |
| [`PACKET_FANOUT_FLAG_IGNORE_OUTGOING`](#packet-fanout-flag-ignore-outgoing) | const |  |
| [`PACKET_FANOUT_FLAG_DEFRAG`](#packet-fanout-flag-defrag) | const |  |
| [`TP_STATUS_KERNEL`](#tp-status-kernel) | const |  |
| [`TP_STATUS_USER`](#tp-status-user) | const |  |
| [`TP_STATUS_COPY`](#tp-status-copy) | const |  |
| [`TP_STATUS_LOSING`](#tp-status-losing) | const |  |
| [`TP_STATUS_CSUMNOTREADY`](#tp-status-csumnotready) | const |  |
| [`TP_STATUS_VLAN_VALID`](#tp-status-vlan-valid) | const |  |
| [`TP_STATUS_BLK_TMO`](#tp-status-blk-tmo) | const |  |
| [`TP_STATUS_VLAN_TPID_VALID`](#tp-status-vlan-tpid-valid) | const |  |
| [`TP_STATUS_CSUM_VALID`](#tp-status-csum-valid) | const |  |
| [`TP_STATUS_AVAILABLE`](#tp-status-available) | const |  |
| [`TP_STATUS_SEND_REQUEST`](#tp-status-send-request) | const |  |
| [`TP_STATUS_SENDING`](#tp-status-sending) | const |  |
| [`TP_STATUS_WRONG_FORMAT`](#tp-status-wrong-format) | const |  |
| [`TP_STATUS_TS_SOFTWARE`](#tp-status-ts-software) | const |  |
| [`TP_STATUS_TS_SYS_HARDWARE`](#tp-status-ts-sys-hardware) | const |  |
| [`TP_STATUS_TS_RAW_HARDWARE`](#tp-status-ts-raw-hardware) | const |  |
| [`TP_FT_REQ_FILL_RXHASH`](#tp-ft-req-fill-rxhash) | const |  |
| [`TPACKET_ALIGNMENT`](#tpacket-alignment) | const |  |
| [`TPACKET_HDRLEN`](#tpacket-hdrlen) | const |  |
| [`TPACKET2_HDRLEN`](#tpacket2-hdrlen) | const |  |
| [`TPACKET3_HDRLEN`](#tpacket3-hdrlen) | const |  |
| [`NF_DROP`](#nf-drop) | const |  |
| [`NF_ACCEPT`](#nf-accept) | const |  |
| [`NF_STOLEN`](#nf-stolen) | const |  |
| [`NF_QUEUE`](#nf-queue) | const |  |
| [`NF_REPEAT`](#nf-repeat) | const |  |
| [`NF_STOP`](#nf-stop) | const |  |
| [`NF_MAX_VERDICT`](#nf-max-verdict) | const |  |
| [`NF_VERDICT_MASK`](#nf-verdict-mask) | const |  |
| [`NF_VERDICT_FLAG_QUEUE_BYPASS`](#nf-verdict-flag-queue-bypass) | const |  |
| [`NF_VERDICT_QMASK`](#nf-verdict-qmask) | const |  |
| [`NF_VERDICT_QBITS`](#nf-verdict-qbits) | const |  |
| [`NF_VERDICT_BITS`](#nf-verdict-bits) | const |  |
| [`NF_INET_PRE_ROUTING`](#nf-inet-pre-routing) | const |  |
| [`NF_INET_LOCAL_IN`](#nf-inet-local-in) | const |  |
| [`NF_INET_FORWARD`](#nf-inet-forward) | const |  |
| [`NF_INET_LOCAL_OUT`](#nf-inet-local-out) | const |  |
| [`NF_INET_POST_ROUTING`](#nf-inet-post-routing) | const |  |
| [`NF_INET_NUMHOOKS`](#nf-inet-numhooks) | const |  |
| [`NF_INET_INGRESS`](#nf-inet-ingress) | const |  |
| [`NF_NETDEV_INGRESS`](#nf-netdev-ingress) | const |  |
| [`NF_NETDEV_EGRESS`](#nf-netdev-egress) | const |  |
| [`NF_NETDEV_NUMHOOKS`](#nf-netdev-numhooks) | const |  |
| [`NFPROTO_UNSPEC`](#nfproto-unspec) | const |  |
| [`NFPROTO_INET`](#nfproto-inet) | const |  |
| [`NFPROTO_IPV4`](#nfproto-ipv4) | const |  |
| [`NFPROTO_ARP`](#nfproto-arp) | const |  |
| [`NFPROTO_NETDEV`](#nfproto-netdev) | const |  |
| [`NFPROTO_BRIDGE`](#nfproto-bridge) | const |  |
| [`NFPROTO_IPV6`](#nfproto-ipv6) | const |  |
| [`NFPROTO_DECNET`](#nfproto-decnet) | const |  |
| [`NFPROTO_NUMPROTO`](#nfproto-numproto) | const |  |
| [`NF_ARP`](#nf-arp) | const |  |
| [`NF_ARP_IN`](#nf-arp-in) | const |  |
| [`NF_ARP_OUT`](#nf-arp-out) | const |  |
| [`NF_ARP_FORWARD`](#nf-arp-forward) | const |  |
| [`NF_ARP_NUMHOOKS`](#nf-arp-numhooks) | const |  |
| [`NF_BR_PRE_ROUTING`](#nf-br-pre-routing) | const |  |
| [`NF_BR_LOCAL_IN`](#nf-br-local-in) | const |  |
| [`NF_BR_FORWARD`](#nf-br-forward) | const |  |
| [`NF_BR_LOCAL_OUT`](#nf-br-local-out) | const |  |
| [`NF_BR_POST_ROUTING`](#nf-br-post-routing) | const |  |
| [`NF_BR_BROUTING`](#nf-br-brouting) | const |  |
| [`NF_BR_NUMHOOKS`](#nf-br-numhooks) | const |  |
| [`NF_BR_PRI_FIRST`](#nf-br-pri-first) | const |  |
| [`NF_BR_PRI_NAT_DST_BRIDGED`](#nf-br-pri-nat-dst-bridged) | const |  |
| [`NF_BR_PRI_FILTER_BRIDGED`](#nf-br-pri-filter-bridged) | const |  |
| [`NF_BR_PRI_BRNF`](#nf-br-pri-brnf) | const |  |
| [`NF_BR_PRI_NAT_DST_OTHER`](#nf-br-pri-nat-dst-other) | const |  |
| [`NF_BR_PRI_FILTER_OTHER`](#nf-br-pri-filter-other) | const |  |
| [`NF_BR_PRI_NAT_SRC`](#nf-br-pri-nat-src) | const |  |
| [`NF_BR_PRI_LAST`](#nf-br-pri-last) | const |  |
| [`NF_IP_PRE_ROUTING`](#nf-ip-pre-routing) | const |  |
| [`NF_IP_LOCAL_IN`](#nf-ip-local-in) | const |  |
| [`NF_IP_FORWARD`](#nf-ip-forward) | const |  |
| [`NF_IP_LOCAL_OUT`](#nf-ip-local-out) | const |  |
| [`NF_IP_POST_ROUTING`](#nf-ip-post-routing) | const |  |
| [`NF_IP_NUMHOOKS`](#nf-ip-numhooks) | const |  |
| [`NF_IP_PRI_FIRST`](#nf-ip-pri-first) | const |  |
| [`NF_IP_PRI_RAW_BEFORE_DEFRAG`](#nf-ip-pri-raw-before-defrag) | const |  |
| [`NF_IP_PRI_CONNTRACK_DEFRAG`](#nf-ip-pri-conntrack-defrag) | const |  |
| [`NF_IP_PRI_RAW`](#nf-ip-pri-raw) | const |  |
| [`NF_IP_PRI_SELINUX_FIRST`](#nf-ip-pri-selinux-first) | const |  |
| [`NF_IP_PRI_CONNTRACK`](#nf-ip-pri-conntrack) | const |  |
| [`NF_IP_PRI_MANGLE`](#nf-ip-pri-mangle) | const |  |
| [`NF_IP_PRI_NAT_DST`](#nf-ip-pri-nat-dst) | const |  |
| [`NF_IP_PRI_FILTER`](#nf-ip-pri-filter) | const |  |
| [`NF_IP_PRI_SECURITY`](#nf-ip-pri-security) | const |  |
| [`NF_IP_PRI_NAT_SRC`](#nf-ip-pri-nat-src) | const |  |
| [`NF_IP_PRI_SELINUX_LAST`](#nf-ip-pri-selinux-last) | const |  |
| [`NF_IP_PRI_CONNTRACK_HELPER`](#nf-ip-pri-conntrack-helper) | const |  |
| [`NF_IP_PRI_CONNTRACK_CONFIRM`](#nf-ip-pri-conntrack-confirm) | const |  |
| [`NF_IP_PRI_LAST`](#nf-ip-pri-last) | const |  |
| [`NF_IP6_PRE_ROUTING`](#nf-ip6-pre-routing) | const |  |
| [`NF_IP6_LOCAL_IN`](#nf-ip6-local-in) | const |  |
| [`NF_IP6_FORWARD`](#nf-ip6-forward) | const |  |
| [`NF_IP6_LOCAL_OUT`](#nf-ip6-local-out) | const |  |
| [`NF_IP6_POST_ROUTING`](#nf-ip6-post-routing) | const |  |
| [`NF_IP6_NUMHOOKS`](#nf-ip6-numhooks) | const |  |
| [`NF_IP6_PRI_FIRST`](#nf-ip6-pri-first) | const |  |
| [`NF_IP6_PRI_RAW_BEFORE_DEFRAG`](#nf-ip6-pri-raw-before-defrag) | const |  |
| [`NF_IP6_PRI_CONNTRACK_DEFRAG`](#nf-ip6-pri-conntrack-defrag) | const |  |
| [`NF_IP6_PRI_RAW`](#nf-ip6-pri-raw) | const |  |
| [`NF_IP6_PRI_SELINUX_FIRST`](#nf-ip6-pri-selinux-first) | const |  |
| [`NF_IP6_PRI_CONNTRACK`](#nf-ip6-pri-conntrack) | const |  |
| [`NF_IP6_PRI_MANGLE`](#nf-ip6-pri-mangle) | const |  |
| [`NF_IP6_PRI_NAT_DST`](#nf-ip6-pri-nat-dst) | const |  |
| [`NF_IP6_PRI_FILTER`](#nf-ip6-pri-filter) | const |  |
| [`NF_IP6_PRI_SECURITY`](#nf-ip6-pri-security) | const |  |
| [`NF_IP6_PRI_NAT_SRC`](#nf-ip6-pri-nat-src) | const |  |
| [`NF_IP6_PRI_SELINUX_LAST`](#nf-ip6-pri-selinux-last) | const |  |
| [`NF_IP6_PRI_CONNTRACK_HELPER`](#nf-ip6-pri-conntrack-helper) | const |  |
| [`NF_IP6_PRI_LAST`](#nf-ip6-pri-last) | const |  |
| [`IP6T_SO_ORIGINAL_DST`](#ip6t-so-original-dst) | const |  |
| [`SIOCSHWTSTAMP`](#siocshwtstamp) | const |  |
| [`SIOCGHWTSTAMP`](#siocghwtstamp) | const |  |
| [`WIRELESS_EXT`](#wireless-ext) | const |  |
| [`SIOCSIWCOMMIT`](#siocsiwcommit) | const |  |
| [`SIOCGIWNAME`](#siocgiwname) | const |  |
| [`SIOCSIWNWID`](#siocsiwnwid) | const |  |
| [`SIOCGIWNWID`](#siocgiwnwid) | const |  |
| [`SIOCSIWFREQ`](#siocsiwfreq) | const |  |
| [`SIOCGIWFREQ`](#siocgiwfreq) | const |  |
| [`SIOCSIWMODE`](#siocsiwmode) | const |  |
| [`SIOCGIWMODE`](#siocgiwmode) | const |  |
| [`SIOCSIWSENS`](#siocsiwsens) | const |  |
| [`SIOCGIWSENS`](#siocgiwsens) | const |  |
| [`SIOCSIWRANGE`](#siocsiwrange) | const |  |
| [`SIOCGIWRANGE`](#siocgiwrange) | const |  |
| [`SIOCSIWPRIV`](#siocsiwpriv) | const |  |
| [`SIOCGIWPRIV`](#siocgiwpriv) | const |  |
| [`SIOCSIWSTATS`](#siocsiwstats) | const |  |
| [`SIOCGIWSTATS`](#siocgiwstats) | const |  |
| [`SIOCSIWSPY`](#siocsiwspy) | const |  |
| [`SIOCGIWSPY`](#siocgiwspy) | const |  |
| [`SIOCSIWTHRSPY`](#siocsiwthrspy) | const |  |
| [`SIOCGIWTHRSPY`](#siocgiwthrspy) | const |  |
| [`SIOCSIWAP`](#siocsiwap) | const |  |
| [`SIOCGIWAP`](#siocgiwap) | const |  |
| [`SIOCGIWAPLIST`](#siocgiwaplist) | const |  |
| [`SIOCSIWSCAN`](#siocsiwscan) | const |  |
| [`SIOCGIWSCAN`](#siocgiwscan) | const |  |
| [`SIOCSIWESSID`](#siocsiwessid) | const |  |
| [`SIOCGIWESSID`](#siocgiwessid) | const |  |
| [`SIOCSIWNICKN`](#siocsiwnickn) | const |  |
| [`SIOCGIWNICKN`](#siocgiwnickn) | const |  |
| [`SIOCSIWRATE`](#siocsiwrate) | const |  |
| [`SIOCGIWRATE`](#siocgiwrate) | const |  |
| [`SIOCSIWRTS`](#siocsiwrts) | const |  |
| [`SIOCGIWRTS`](#siocgiwrts) | const |  |
| [`SIOCSIWFRAG`](#siocsiwfrag) | const |  |
| [`SIOCGIWFRAG`](#siocgiwfrag) | const |  |
| [`SIOCSIWTXPOW`](#siocsiwtxpow) | const |  |
| [`SIOCGIWTXPOW`](#siocgiwtxpow) | const |  |
| [`SIOCSIWRETRY`](#siocsiwretry) | const |  |
| [`SIOCGIWRETRY`](#siocgiwretry) | const |  |
| [`SIOCSIWENCODE`](#siocsiwencode) | const |  |
| [`SIOCGIWENCODE`](#siocgiwencode) | const |  |
| [`SIOCSIWPOWER`](#siocsiwpower) | const |  |
| [`SIOCGIWPOWER`](#siocgiwpower) | const |  |
| [`SIOCSIWGENIE`](#siocsiwgenie) | const |  |
| [`SIOCGIWGENIE`](#siocgiwgenie) | const |  |
| [`SIOCSIWMLME`](#siocsiwmlme) | const |  |
| [`SIOCSIWAUTH`](#siocsiwauth) | const |  |
| [`SIOCGIWAUTH`](#siocgiwauth) | const |  |
| [`SIOCSIWENCODEEXT`](#siocsiwencodeext) | const |  |
| [`SIOCGIWENCODEEXT`](#siocgiwencodeext) | const |  |
| [`SIOCSIWPMKSA`](#siocsiwpmksa) | const |  |
| [`SIOCIWFIRSTPRIV`](#siociwfirstpriv) | const |  |
| [`SIOCIWLASTPRIV`](#siociwlastpriv) | const |  |
| [`SIOCIWFIRST`](#siociwfirst) | const |  |
| [`SIOCIWLAST`](#siociwlast) | const |  |
| [`IWEVTXDROP`](#iwevtxdrop) | const |  |
| [`IWEVQUAL`](#iwevqual) | const |  |
| [`IWEVCUSTOM`](#iwevcustom) | const |  |
| [`IWEVREGISTERED`](#iwevregistered) | const |  |
| [`IWEVEXPIRED`](#iwevexpired) | const |  |
| [`IWEVGENIE`](#iwevgenie) | const |  |
| [`IWEVMICHAELMICFAILURE`](#iwevmichaelmicfailure) | const |  |
| [`IWEVASSOCREQIE`](#iwevassocreqie) | const |  |
| [`IWEVASSOCRESPIE`](#iwevassocrespie) | const |  |
| [`IWEVPMKIDCAND`](#iwevpmkidcand) | const |  |
| [`IWEVFIRST`](#iwevfirst) | const |  |
| [`IW_PRIV_TYPE_MASK`](#iw-priv-type-mask) | const |  |
| [`IW_PRIV_TYPE_NONE`](#iw-priv-type-none) | const |  |
| [`IW_PRIV_TYPE_BYTE`](#iw-priv-type-byte) | const |  |
| [`IW_PRIV_TYPE_CHAR`](#iw-priv-type-char) | const |  |
| [`IW_PRIV_TYPE_INT`](#iw-priv-type-int) | const |  |
| [`IW_PRIV_TYPE_FLOAT`](#iw-priv-type-float) | const |  |
| [`IW_PRIV_TYPE_ADDR`](#iw-priv-type-addr) | const |  |
| [`IW_PRIV_SIZE_FIXED`](#iw-priv-size-fixed) | const |  |
| [`IW_PRIV_SIZE_MASK`](#iw-priv-size-mask) | const |  |
| [`IW_MAX_FREQUENCIES`](#iw-max-frequencies) | const |  |
| [`IW_MAX_BITRATES`](#iw-max-bitrates) | const |  |
| [`IW_MAX_TXPOWER`](#iw-max-txpower) | const |  |
| [`IW_MAX_SPY`](#iw-max-spy) | const |  |
| [`IW_MAX_AP`](#iw-max-ap) | const |  |
| [`IW_ESSID_MAX_SIZE`](#iw-essid-max-size) | const |  |
| [`IW_MODE_AUTO`](#iw-mode-auto) | const |  |
| [`IW_MODE_ADHOC`](#iw-mode-adhoc) | const |  |
| [`IW_MODE_INFRA`](#iw-mode-infra) | const |  |
| [`IW_MODE_MASTER`](#iw-mode-master) | const |  |
| [`IW_MODE_REPEAT`](#iw-mode-repeat) | const |  |
| [`IW_MODE_SECOND`](#iw-mode-second) | const |  |
| [`IW_MODE_MONITOR`](#iw-mode-monitor) | const |  |
| [`IW_MODE_MESH`](#iw-mode-mesh) | const |  |
| [`IW_QUAL_QUAL_UPDATED`](#iw-qual-qual-updated) | const |  |
| [`IW_QUAL_LEVEL_UPDATED`](#iw-qual-level-updated) | const |  |
| [`IW_QUAL_NOISE_UPDATED`](#iw-qual-noise-updated) | const |  |
| [`IW_QUAL_ALL_UPDATED`](#iw-qual-all-updated) | const |  |
| [`IW_QUAL_DBM`](#iw-qual-dbm) | const |  |
| [`IW_QUAL_QUAL_INVALID`](#iw-qual-qual-invalid) | const |  |
| [`IW_QUAL_LEVEL_INVALID`](#iw-qual-level-invalid) | const |  |
| [`IW_QUAL_NOISE_INVALID`](#iw-qual-noise-invalid) | const |  |
| [`IW_QUAL_RCPI`](#iw-qual-rcpi) | const |  |
| [`IW_QUAL_ALL_INVALID`](#iw-qual-all-invalid) | const |  |
| [`IW_FREQ_AUTO`](#iw-freq-auto) | const |  |
| [`IW_FREQ_FIXED`](#iw-freq-fixed) | const |  |
| [`IW_MAX_ENCODING_SIZES`](#iw-max-encoding-sizes) | const |  |
| [`IW_ENCODING_TOKEN_MAX`](#iw-encoding-token-max) | const |  |
| [`IW_ENCODE_INDEX`](#iw-encode-index) | const |  |
| [`IW_ENCODE_FLAGS`](#iw-encode-flags) | const |  |
| [`IW_ENCODE_MODE`](#iw-encode-mode) | const |  |
| [`IW_ENCODE_DISABLED`](#iw-encode-disabled) | const |  |
| [`IW_ENCODE_ENABLED`](#iw-encode-enabled) | const |  |
| [`IW_ENCODE_RESTRICTED`](#iw-encode-restricted) | const |  |
| [`IW_ENCODE_OPEN`](#iw-encode-open) | const |  |
| [`IW_ENCODE_NOKEY`](#iw-encode-nokey) | const |  |
| [`IW_ENCODE_TEMP`](#iw-encode-temp) | const |  |
| [`IW_POWER_ON`](#iw-power-on) | const |  |
| [`IW_POWER_TYPE`](#iw-power-type) | const |  |
| [`IW_POWER_PERIOD`](#iw-power-period) | const |  |
| [`IW_POWER_TIMEOUT`](#iw-power-timeout) | const |  |
| [`IW_POWER_MODE`](#iw-power-mode) | const |  |
| [`IW_POWER_UNICAST_R`](#iw-power-unicast-r) | const |  |
| [`IW_POWER_MULTICAST_R`](#iw-power-multicast-r) | const |  |
| [`IW_POWER_ALL_R`](#iw-power-all-r) | const |  |
| [`IW_POWER_FORCE_S`](#iw-power-force-s) | const |  |
| [`IW_POWER_REPEATER`](#iw-power-repeater) | const |  |
| [`IW_POWER_MODIFIER`](#iw-power-modifier) | const |  |
| [`IW_POWER_MIN`](#iw-power-min) | const |  |
| [`IW_POWER_MAX`](#iw-power-max) | const |  |
| [`IW_POWER_RELATIVE`](#iw-power-relative) | const |  |
| [`IW_TXPOW_TYPE`](#iw-txpow-type) | const |  |
| [`IW_TXPOW_DBM`](#iw-txpow-dbm) | const |  |
| [`IW_TXPOW_MWATT`](#iw-txpow-mwatt) | const |  |
| [`IW_TXPOW_RELATIVE`](#iw-txpow-relative) | const |  |
| [`IW_TXPOW_RANGE`](#iw-txpow-range) | const |  |
| [`IW_RETRY_ON`](#iw-retry-on) | const |  |
| [`IW_RETRY_TYPE`](#iw-retry-type) | const |  |
| [`IW_RETRY_LIMIT`](#iw-retry-limit) | const |  |
| [`IW_RETRY_LIFETIME`](#iw-retry-lifetime) | const |  |
| [`IW_RETRY_MODIFIER`](#iw-retry-modifier) | const |  |
| [`IW_RETRY_MIN`](#iw-retry-min) | const |  |
| [`IW_RETRY_MAX`](#iw-retry-max) | const |  |
| [`IW_RETRY_RELATIVE`](#iw-retry-relative) | const |  |
| [`IW_RETRY_SHORT`](#iw-retry-short) | const |  |
| [`IW_RETRY_LONG`](#iw-retry-long) | const |  |
| [`IW_SCAN_DEFAULT`](#iw-scan-default) | const |  |
| [`IW_SCAN_ALL_ESSID`](#iw-scan-all-essid) | const |  |
| [`IW_SCAN_THIS_ESSID`](#iw-scan-this-essid) | const |  |
| [`IW_SCAN_ALL_FREQ`](#iw-scan-all-freq) | const |  |
| [`IW_SCAN_THIS_FREQ`](#iw-scan-this-freq) | const |  |
| [`IW_SCAN_ALL_MODE`](#iw-scan-all-mode) | const |  |
| [`IW_SCAN_THIS_MODE`](#iw-scan-this-mode) | const |  |
| [`IW_SCAN_ALL_RATE`](#iw-scan-all-rate) | const |  |
| [`IW_SCAN_THIS_RATE`](#iw-scan-this-rate) | const |  |
| [`IW_SCAN_TYPE_ACTIVE`](#iw-scan-type-active) | const |  |
| [`IW_SCAN_TYPE_PASSIVE`](#iw-scan-type-passive) | const |  |
| [`IW_SCAN_MAX_DATA`](#iw-scan-max-data) | const |  |
| [`IW_SCAN_CAPA_NONE`](#iw-scan-capa-none) | const |  |
| [`IW_SCAN_CAPA_ESSID`](#iw-scan-capa-essid) | const |  |
| [`IW_SCAN_CAPA_BSSID`](#iw-scan-capa-bssid) | const |  |
| [`IW_SCAN_CAPA_CHANNEL`](#iw-scan-capa-channel) | const |  |
| [`IW_SCAN_CAPA_MODE`](#iw-scan-capa-mode) | const |  |
| [`IW_SCAN_CAPA_RATE`](#iw-scan-capa-rate) | const |  |
| [`IW_SCAN_CAPA_TYPE`](#iw-scan-capa-type) | const |  |
| [`IW_SCAN_CAPA_TIME`](#iw-scan-capa-time) | const |  |
| [`IW_CUSTOM_MAX`](#iw-custom-max) | const |  |
| [`IW_GENERIC_IE_MAX`](#iw-generic-ie-max) | const |  |
| [`IW_MLME_DEAUTH`](#iw-mlme-deauth) | const |  |
| [`IW_MLME_DISASSOC`](#iw-mlme-disassoc) | const |  |
| [`IW_MLME_AUTH`](#iw-mlme-auth) | const |  |
| [`IW_MLME_ASSOC`](#iw-mlme-assoc) | const |  |
| [`IW_AUTH_INDEX`](#iw-auth-index) | const |  |
| [`IW_AUTH_FLAGS`](#iw-auth-flags) | const |  |
| [`IW_AUTH_WPA_VERSION`](#iw-auth-wpa-version) | const |  |
| [`IW_AUTH_CIPHER_PAIRWISE`](#iw-auth-cipher-pairwise) | const |  |
| [`IW_AUTH_CIPHER_GROUP`](#iw-auth-cipher-group) | const |  |
| [`IW_AUTH_KEY_MGMT`](#iw-auth-key-mgmt) | const |  |
| [`IW_AUTH_TKIP_COUNTERMEASURES`](#iw-auth-tkip-countermeasures) | const |  |
| [`IW_AUTH_DROP_UNENCRYPTED`](#iw-auth-drop-unencrypted) | const |  |
| [`IW_AUTH_80211_AUTH_ALG`](#iw-auth-80211-auth-alg) | const |  |
| [`IW_AUTH_WPA_ENABLED`](#iw-auth-wpa-enabled) | const |  |
| [`IW_AUTH_RX_UNENCRYPTED_EAPOL`](#iw-auth-rx-unencrypted-eapol) | const |  |
| [`IW_AUTH_ROAMING_CONTROL`](#iw-auth-roaming-control) | const |  |
| [`IW_AUTH_PRIVACY_INVOKED`](#iw-auth-privacy-invoked) | const |  |
| [`IW_AUTH_CIPHER_GROUP_MGMT`](#iw-auth-cipher-group-mgmt) | const |  |
| [`IW_AUTH_MFP`](#iw-auth-mfp) | const |  |
| [`IW_AUTH_WPA_VERSION_DISABLED`](#iw-auth-wpa-version-disabled) | const |  |
| [`IW_AUTH_WPA_VERSION_WPA`](#iw-auth-wpa-version-wpa) | const |  |
| [`IW_AUTH_WPA_VERSION_WPA2`](#iw-auth-wpa-version-wpa2) | const |  |
| [`IW_AUTH_CIPHER_NONE`](#iw-auth-cipher-none) | const |  |
| [`IW_AUTH_CIPHER_WEP40`](#iw-auth-cipher-wep40) | const |  |
| [`IW_AUTH_CIPHER_TKIP`](#iw-auth-cipher-tkip) | const |  |
| [`IW_AUTH_CIPHER_CCMP`](#iw-auth-cipher-ccmp) | const |  |
| [`IW_AUTH_CIPHER_WEP104`](#iw-auth-cipher-wep104) | const |  |
| [`IW_AUTH_CIPHER_AES_CMAC`](#iw-auth-cipher-aes-cmac) | const |  |
| [`IW_AUTH_KEY_MGMT_802_1X`](#iw-auth-key-mgmt-802-1x) | const |  |
| [`IW_AUTH_KEY_MGMT_PSK`](#iw-auth-key-mgmt-psk) | const |  |
| [`IW_AUTH_ALG_OPEN_SYSTEM`](#iw-auth-alg-open-system) | const |  |
| [`IW_AUTH_ALG_SHARED_KEY`](#iw-auth-alg-shared-key) | const |  |
| [`IW_AUTH_ALG_LEAP`](#iw-auth-alg-leap) | const |  |
| [`IW_AUTH_ROAMING_ENABLE`](#iw-auth-roaming-enable) | const |  |
| [`IW_AUTH_ROAMING_DISABLE`](#iw-auth-roaming-disable) | const |  |
| [`IW_AUTH_MFP_DISABLED`](#iw-auth-mfp-disabled) | const |  |
| [`IW_AUTH_MFP_OPTIONAL`](#iw-auth-mfp-optional) | const |  |
| [`IW_AUTH_MFP_REQUIRED`](#iw-auth-mfp-required) | const |  |
| [`IW_ENCODE_SEQ_MAX_SIZE`](#iw-encode-seq-max-size) | const |  |
| [`IW_ENCODE_ALG_NONE`](#iw-encode-alg-none) | const |  |
| [`IW_ENCODE_ALG_WEP`](#iw-encode-alg-wep) | const |  |
| [`IW_ENCODE_ALG_TKIP`](#iw-encode-alg-tkip) | const |  |
| [`IW_ENCODE_ALG_CCMP`](#iw-encode-alg-ccmp) | const |  |
| [`IW_ENCODE_ALG_PMK`](#iw-encode-alg-pmk) | const |  |
| [`IW_ENCODE_ALG_AES_CMAC`](#iw-encode-alg-aes-cmac) | const |  |
| [`IW_ENCODE_EXT_TX_SEQ_VALID`](#iw-encode-ext-tx-seq-valid) | const |  |
| [`IW_ENCODE_EXT_RX_SEQ_VALID`](#iw-encode-ext-rx-seq-valid) | const |  |
| [`IW_ENCODE_EXT_GROUP_KEY`](#iw-encode-ext-group-key) | const |  |
| [`IW_ENCODE_EXT_SET_TX_KEY`](#iw-encode-ext-set-tx-key) | const |  |
| [`IW_MICFAILURE_KEY_ID`](#iw-micfailure-key-id) | const |  |
| [`IW_MICFAILURE_GROUP`](#iw-micfailure-group) | const |  |
| [`IW_MICFAILURE_PAIRWISE`](#iw-micfailure-pairwise) | const |  |
| [`IW_MICFAILURE_STAKEY`](#iw-micfailure-stakey) | const |  |
| [`IW_MICFAILURE_COUNT`](#iw-micfailure-count) | const |  |
| [`IW_ENC_CAPA_WPA`](#iw-enc-capa-wpa) | const |  |
| [`IW_ENC_CAPA_WPA2`](#iw-enc-capa-wpa2) | const |  |
| [`IW_ENC_CAPA_CIPHER_TKIP`](#iw-enc-capa-cipher-tkip) | const |  |
| [`IW_ENC_CAPA_CIPHER_CCMP`](#iw-enc-capa-cipher-ccmp) | const |  |
| [`IW_ENC_CAPA_4WAY_HANDSHAKE`](#iw-enc-capa-4way-handshake) | const |  |
| [`IW_EVENT_CAPA_K_0`](#iw-event-capa-k-0) | const |  |
| [`IW_EVENT_CAPA_K_1`](#iw-event-capa-k-1) | const |  |
| [`IW_PMKSA_ADD`](#iw-pmksa-add) | const |  |
| [`IW_PMKSA_REMOVE`](#iw-pmksa-remove) | const |  |
| [`IW_PMKSA_FLUSH`](#iw-pmksa-flush) | const |  |
| [`IW_PMKID_LEN`](#iw-pmkid-len) | const |  |
| [`IW_PMKID_CAND_PREAUTH`](#iw-pmkid-cand-preauth) | const |  |
| [`IW_EV_LCP_PK_LEN`](#iw-ev-lcp-pk-len) | const |  |
| [`IW_EV_CHAR_PK_LEN`](#iw-ev-char-pk-len) | const |  |
| [`IW_EV_UINT_PK_LEN`](#iw-ev-uint-pk-len) | const |  |
| [`IW_EV_FREQ_PK_LEN`](#iw-ev-freq-pk-len) | const |  |
| [`IW_EV_PARAM_PK_LEN`](#iw-ev-param-pk-len) | const |  |
| [`IW_EV_ADDR_PK_LEN`](#iw-ev-addr-pk-len) | const |  |
| [`IW_EV_QUAL_PK_LEN`](#iw-ev-qual-pk-len) | const |  |
| [`IW_EV_POINT_PK_LEN`](#iw-ev-point-pk-len) | const |  |
| [`NUD_NONE`](#nud-none) | const |  |
| [`NUD_INCOMPLETE`](#nud-incomplete) | const |  |
| [`NUD_REACHABLE`](#nud-reachable) | const |  |
| [`NUD_STALE`](#nud-stale) | const |  |
| [`NUD_DELAY`](#nud-delay) | const |  |
| [`NUD_PROBE`](#nud-probe) | const |  |
| [`NUD_FAILED`](#nud-failed) | const |  |
| [`NUD_NOARP`](#nud-noarp) | const |  |
| [`NUD_PERMANENT`](#nud-permanent) | const |  |
| [`NTF_USE`](#ntf-use) | const |  |
| [`NTF_SELF`](#ntf-self) | const |  |
| [`NTF_MASTER`](#ntf-master) | const |  |
| [`NTF_PROXY`](#ntf-proxy) | const |  |
| [`NTF_ROUTER`](#ntf-router) | const |  |
| [`NDA_UNSPEC`](#nda-unspec) | const |  |
| [`NDA_DST`](#nda-dst) | const |  |
| [`NDA_LLADDR`](#nda-lladdr) | const |  |
| [`NDA_CACHEINFO`](#nda-cacheinfo) | const |  |
| [`NDA_PROBES`](#nda-probes) | const |  |
| [`NDA_VLAN`](#nda-vlan) | const |  |
| [`NDA_PORT`](#nda-port) | const |  |
| [`NDA_VNI`](#nda-vni) | const |  |
| [`NDA_IFINDEX`](#nda-ifindex) | const |  |
| [`NLM_F_BULK`](#nlm-f-bulk) | const |  |
| [`TCA_UNSPEC`](#tca-unspec) | const |  |
| [`TCA_KIND`](#tca-kind) | const |  |
| [`TCA_OPTIONS`](#tca-options) | const |  |
| [`TCA_STATS`](#tca-stats) | const |  |
| [`TCA_XSTATS`](#tca-xstats) | const |  |
| [`TCA_RATE`](#tca-rate) | const |  |
| [`TCA_FCNT`](#tca-fcnt) | const |  |
| [`TCA_STATS2`](#tca-stats2) | const |  |
| [`TCA_STAB`](#tca-stab) | const |  |
| [`RTM_NEWLINK`](#rtm-newlink) | const |  |
| [`RTM_DELLINK`](#rtm-dellink) | const |  |
| [`RTM_GETLINK`](#rtm-getlink) | const |  |
| [`RTM_SETLINK`](#rtm-setlink) | const |  |
| [`RTM_NEWADDR`](#rtm-newaddr) | const |  |
| [`RTM_DELADDR`](#rtm-deladdr) | const |  |
| [`RTM_GETADDR`](#rtm-getaddr) | const |  |
| [`RTM_NEWROUTE`](#rtm-newroute) | const |  |
| [`RTM_DELROUTE`](#rtm-delroute) | const |  |
| [`RTM_GETROUTE`](#rtm-getroute) | const |  |
| [`RTM_NEWNEIGH`](#rtm-newneigh) | const |  |
| [`RTM_DELNEIGH`](#rtm-delneigh) | const |  |
| [`RTM_GETNEIGH`](#rtm-getneigh) | const |  |
| [`RTM_NEWRULE`](#rtm-newrule) | const |  |
| [`RTM_DELRULE`](#rtm-delrule) | const |  |
| [`RTM_GETRULE`](#rtm-getrule) | const |  |
| [`RTM_NEWQDISC`](#rtm-newqdisc) | const |  |
| [`RTM_DELQDISC`](#rtm-delqdisc) | const |  |
| [`RTM_GETQDISC`](#rtm-getqdisc) | const |  |
| [`RTM_NEWTCLASS`](#rtm-newtclass) | const |  |
| [`RTM_DELTCLASS`](#rtm-deltclass) | const |  |
| [`RTM_GETTCLASS`](#rtm-gettclass) | const |  |
| [`RTM_NEWTFILTER`](#rtm-newtfilter) | const |  |
| [`RTM_DELTFILTER`](#rtm-deltfilter) | const |  |
| [`RTM_GETTFILTER`](#rtm-gettfilter) | const |  |
| [`RTM_NEWACTION`](#rtm-newaction) | const |  |
| [`RTM_DELACTION`](#rtm-delaction) | const |  |
| [`RTM_GETACTION`](#rtm-getaction) | const |  |
| [`RTM_NEWPREFIX`](#rtm-newprefix) | const |  |
| [`RTM_GETMULTICAST`](#rtm-getmulticast) | const |  |
| [`RTM_GETANYCAST`](#rtm-getanycast) | const |  |
| [`RTM_NEWNEIGHTBL`](#rtm-newneightbl) | const |  |
| [`RTM_GETNEIGHTBL`](#rtm-getneightbl) | const |  |
| [`RTM_SETNEIGHTBL`](#rtm-setneightbl) | const |  |
| [`RTM_NEWNDUSEROPT`](#rtm-newnduseropt) | const |  |
| [`RTM_NEWADDRLABEL`](#rtm-newaddrlabel) | const |  |
| [`RTM_DELADDRLABEL`](#rtm-deladdrlabel) | const |  |
| [`RTM_GETADDRLABEL`](#rtm-getaddrlabel) | const |  |
| [`RTM_GETDCB`](#rtm-getdcb) | const |  |
| [`RTM_SETDCB`](#rtm-setdcb) | const |  |
| [`RTM_NEWNETCONF`](#rtm-newnetconf) | const |  |
| [`RTM_GETNETCONF`](#rtm-getnetconf) | const |  |
| [`RTM_NEWMDB`](#rtm-newmdb) | const |  |
| [`RTM_DELMDB`](#rtm-delmdb) | const |  |
| [`RTM_GETMDB`](#rtm-getmdb) | const |  |
| [`RTM_NEWNSID`](#rtm-newnsid) | const |  |
| [`RTM_DELNSID`](#rtm-delnsid) | const |  |
| [`RTM_GETNSID`](#rtm-getnsid) | const |  |
| [`RTM_F_NOTIFY`](#rtm-f-notify) | const |  |
| [`RTM_F_CLONED`](#rtm-f-cloned) | const |  |
| [`RTM_F_EQUALIZE`](#rtm-f-equalize) | const |  |
| [`RTM_F_PREFIX`](#rtm-f-prefix) | const |  |
| [`RTA_UNSPEC`](#rta-unspec) | const |  |
| [`RTA_DST`](#rta-dst) | const |  |
| [`RTA_SRC`](#rta-src) | const |  |
| [`RTA_IIF`](#rta-iif) | const |  |
| [`RTA_OIF`](#rta-oif) | const |  |
| [`RTA_GATEWAY`](#rta-gateway) | const |  |
| [`RTA_PRIORITY`](#rta-priority) | const |  |
| [`RTA_PREFSRC`](#rta-prefsrc) | const |  |
| [`RTA_METRICS`](#rta-metrics) | const |  |
| [`RTA_MULTIPATH`](#rta-multipath) | const |  |
| [`RTA_PROTOINFO`](#rta-protoinfo) | const |  |
| [`RTA_FLOW`](#rta-flow) | const |  |
| [`RTA_CACHEINFO`](#rta-cacheinfo) | const |  |
| [`RTA_SESSION`](#rta-session) | const |  |
| [`RTA_MP_ALGO`](#rta-mp-algo) | const |  |
| [`RTA_TABLE`](#rta-table) | const |  |
| [`RTA_MARK`](#rta-mark) | const |  |
| [`RTA_MFC_STATS`](#rta-mfc-stats) | const |  |
| [`RTN_UNSPEC`](#rtn-unspec) | const |  |
| [`RTN_UNICAST`](#rtn-unicast) | const |  |
| [`RTN_LOCAL`](#rtn-local) | const |  |
| [`RTN_BROADCAST`](#rtn-broadcast) | const |  |
| [`RTN_ANYCAST`](#rtn-anycast) | const |  |
| [`RTN_MULTICAST`](#rtn-multicast) | const |  |
| [`RTN_BLACKHOLE`](#rtn-blackhole) | const |  |
| [`RTN_UNREACHABLE`](#rtn-unreachable) | const |  |
| [`RTN_PROHIBIT`](#rtn-prohibit) | const |  |
| [`RTN_THROW`](#rtn-throw) | const |  |
| [`RTN_NAT`](#rtn-nat) | const |  |
| [`RTN_XRESOLVE`](#rtn-xresolve) | const |  |
| [`RTPROT_UNSPEC`](#rtprot-unspec) | const |  |
| [`RTPROT_REDIRECT`](#rtprot-redirect) | const |  |
| [`RTPROT_KERNEL`](#rtprot-kernel) | const |  |
| [`RTPROT_BOOT`](#rtprot-boot) | const |  |
| [`RTPROT_STATIC`](#rtprot-static) | const |  |
| [`RT_SCOPE_UNIVERSE`](#rt-scope-universe) | const |  |
| [`RT_SCOPE_SITE`](#rt-scope-site) | const |  |
| [`RT_SCOPE_LINK`](#rt-scope-link) | const |  |
| [`RT_SCOPE_HOST`](#rt-scope-host) | const |  |
| [`RT_SCOPE_NOWHERE`](#rt-scope-nowhere) | const |  |
| [`RT_TABLE_UNSPEC`](#rt-table-unspec) | const |  |
| [`RT_TABLE_COMPAT`](#rt-table-compat) | const |  |
| [`RT_TABLE_DEFAULT`](#rt-table-default) | const |  |
| [`RT_TABLE_MAIN`](#rt-table-main) | const |  |
| [`RT_TABLE_LOCAL`](#rt-table-local) | const |  |
| [`RTMSG_OVERRUN`](#rtmsg-overrun) | const |  |
| [`RTMSG_NEWDEVICE`](#rtmsg-newdevice) | const |  |
| [`RTMSG_DELDEVICE`](#rtmsg-deldevice) | const |  |
| [`RTMSG_NEWROUTE`](#rtmsg-newroute) | const |  |
| [`RTMSG_DELROUTE`](#rtmsg-delroute) | const |  |
| [`RTMSG_NEWRULE`](#rtmsg-newrule) | const |  |
| [`RTMSG_DELRULE`](#rtmsg-delrule) | const |  |
| [`RTMSG_CONTROL`](#rtmsg-control) | const |  |
| [`RTMSG_AR_FAILED`](#rtmsg-ar-failed) | const |  |
| [`RTEXT_FILTER_VF`](#rtext-filter-vf) | const |  |
| [`RTEXT_FILTER_BRVLAN`](#rtext-filter-brvlan) | const |  |
| [`RTEXT_FILTER_BRVLAN_COMPRESSED`](#rtext-filter-brvlan-compressed) | const |  |
| [`RTEXT_FILTER_SKIP_STATS`](#rtext-filter-skip-stats) | const |  |
| [`RTEXT_FILTER_MRP`](#rtext-filter-mrp) | const |  |
| [`RTEXT_FILTER_CFM_CONFIG`](#rtext-filter-cfm-config) | const |  |
| [`RTEXT_FILTER_CFM_STATUS`](#rtext-filter-cfm-status) | const |  |
| [`RTMGRP_LINK`](#rtmgrp-link) | const |  |
| [`RTMGRP_NOTIFY`](#rtmgrp-notify) | const |  |
| [`RTMGRP_NEIGH`](#rtmgrp-neigh) | const |  |
| [`RTMGRP_TC`](#rtmgrp-tc) | const |  |
| [`RTMGRP_IPV4_IFADDR`](#rtmgrp-ipv4-ifaddr) | const |  |
| [`RTMGRP_IPV4_MROUTE`](#rtmgrp-ipv4-mroute) | const |  |
| [`RTMGRP_IPV4_ROUTE`](#rtmgrp-ipv4-route) | const |  |
| [`RTMGRP_IPV4_RULE`](#rtmgrp-ipv4-rule) | const |  |
| [`RTMGRP_IPV6_IFADDR`](#rtmgrp-ipv6-ifaddr) | const |  |
| [`RTMGRP_IPV6_MROUTE`](#rtmgrp-ipv6-mroute) | const |  |
| [`RTMGRP_IPV6_ROUTE`](#rtmgrp-ipv6-route) | const |  |
| [`RTMGRP_IPV6_IFINFO`](#rtmgrp-ipv6-ifinfo) | const |  |
| [`RTMGRP_DECnet_IFADDR`](#rtmgrp-decnet-ifaddr) | const |  |
| [`RTMGRP_DECnet_ROUTE`](#rtmgrp-decnet-route) | const |  |
| [`RTMGRP_IPV6_PREFIX`](#rtmgrp-ipv6-prefix) | const |  |
| [`RTNLGRP_NONE`](#rtnlgrp-none) | const |  |
| [`RTNLGRP_LINK`](#rtnlgrp-link) | const |  |
| [`RTNLGRP_NOTIFY`](#rtnlgrp-notify) | const |  |
| [`RTNLGRP_NEIGH`](#rtnlgrp-neigh) | const |  |
| [`RTNLGRP_TC`](#rtnlgrp-tc) | const |  |
| [`RTNLGRP_IPV4_IFADDR`](#rtnlgrp-ipv4-ifaddr) | const |  |
| [`RTNLGRP_IPV4_MROUTE`](#rtnlgrp-ipv4-mroute) | const |  |
| [`RTNLGRP_IPV4_ROUTE`](#rtnlgrp-ipv4-route) | const |  |
| [`RTNLGRP_IPV4_RULE`](#rtnlgrp-ipv4-rule) | const |  |
| [`RTNLGRP_IPV6_IFADDR`](#rtnlgrp-ipv6-ifaddr) | const |  |
| [`RTNLGRP_IPV6_MROUTE`](#rtnlgrp-ipv6-mroute) | const |  |
| [`RTNLGRP_IPV6_ROUTE`](#rtnlgrp-ipv6-route) | const |  |
| [`RTNLGRP_IPV6_IFINFO`](#rtnlgrp-ipv6-ifinfo) | const |  |
| [`RTNLGRP_DECnet_IFADDR`](#rtnlgrp-decnet-ifaddr) | const |  |
| [`RTNLGRP_NOP2`](#rtnlgrp-nop2) | const |  |
| [`RTNLGRP_DECnet_ROUTE`](#rtnlgrp-decnet-route) | const |  |
| [`RTNLGRP_DECnet_RULE`](#rtnlgrp-decnet-rule) | const |  |
| [`RTNLGRP_NOP4`](#rtnlgrp-nop4) | const |  |
| [`RTNLGRP_IPV6_PREFIX`](#rtnlgrp-ipv6-prefix) | const |  |
| [`RTNLGRP_IPV6_RULE`](#rtnlgrp-ipv6-rule) | const |  |
| [`RTNLGRP_ND_USEROPT`](#rtnlgrp-nd-useropt) | const |  |
| [`RTNLGRP_PHONET_IFADDR`](#rtnlgrp-phonet-ifaddr) | const |  |
| [`RTNLGRP_PHONET_ROUTE`](#rtnlgrp-phonet-route) | const |  |
| [`RTNLGRP_DCB`](#rtnlgrp-dcb) | const |  |
| [`RTNLGRP_IPV4_NETCONF`](#rtnlgrp-ipv4-netconf) | const |  |
| [`RTNLGRP_IPV6_NETCONF`](#rtnlgrp-ipv6-netconf) | const |  |
| [`RTNLGRP_MDB`](#rtnlgrp-mdb) | const |  |
| [`RTNLGRP_MPLS_ROUTE`](#rtnlgrp-mpls-route) | const |  |
| [`RTNLGRP_NSID`](#rtnlgrp-nsid) | const |  |
| [`RTNLGRP_MPLS_NETCONF`](#rtnlgrp-mpls-netconf) | const |  |
| [`RTNLGRP_IPV4_MROUTE_R`](#rtnlgrp-ipv4-mroute-r) | const |  |
| [`RTNLGRP_IPV6_MROUTE_R`](#rtnlgrp-ipv6-mroute-r) | const |  |
| [`RTNLGRP_NEXTHOP`](#rtnlgrp-nexthop) | const |  |
| [`RTNLGRP_BRVLAN`](#rtnlgrp-brvlan) | const |  |
| [`RTNLGRP_MCTP_IFADDR`](#rtnlgrp-mctp-ifaddr) | const |  |
| [`RTNLGRP_TUNNEL`](#rtnlgrp-tunnel) | const |  |
| [`RTNLGRP_STATS`](#rtnlgrp-stats) | const |  |
| [`PROC_CN_MCAST_LISTEN`](#proc-cn-mcast-listen) | const |  |
| [`PROC_CN_MCAST_IGNORE`](#proc-cn-mcast-ignore) | const |  |
| [`PROC_EVENT_NONE`](#proc-event-none) | const |  |
| [`PROC_EVENT_FORK`](#proc-event-fork) | const |  |
| [`PROC_EVENT_EXEC`](#proc-event-exec) | const |  |
| [`PROC_EVENT_UID`](#proc-event-uid) | const |  |
| [`PROC_EVENT_GID`](#proc-event-gid) | const |  |
| [`PROC_EVENT_SID`](#proc-event-sid) | const |  |
| [`PROC_EVENT_PTRACE`](#proc-event-ptrace) | const |  |
| [`PROC_EVENT_COMM`](#proc-event-comm) | const |  |
| [`PROC_EVENT_NONZERO_EXIT`](#proc-event-nonzero-exit) | const |  |
| [`PROC_EVENT_COREDUMP`](#proc-event-coredump) | const |  |
| [`PROC_EVENT_EXIT`](#proc-event-exit) | const |  |
| [`CN_IDX_PROC`](#cn-idx-proc) | const |  |
| [`CN_VAL_PROC`](#cn-val-proc) | const |  |
| [`CN_IDX_CIFS`](#cn-idx-cifs) | const |  |
| [`CN_VAL_CIFS`](#cn-val-cifs) | const |  |
| [`CN_W1_IDX`](#cn-w1-idx) | const |  |
| [`CN_W1_VAL`](#cn-w1-val) | const |  |
| [`CN_IDX_V86D`](#cn-idx-v86d) | const |  |
| [`CN_VAL_V86D_UVESAFB`](#cn-val-v86d-uvesafb) | const |  |
| [`CN_IDX_BB`](#cn-idx-bb) | const |  |
| [`CN_DST_IDX`](#cn-dst-idx) | const |  |
| [`CN_DST_VAL`](#cn-dst-val) | const |  |
| [`CN_IDX_DM`](#cn-idx-dm) | const |  |
| [`CN_VAL_DM_USERSPACE_LOG`](#cn-val-dm-userspace-log) | const |  |
| [`CN_IDX_DRBD`](#cn-idx-drbd) | const |  |
| [`CN_VAL_DRBD`](#cn-val-drbd) | const |  |
| [`CN_KVP_IDX`](#cn-kvp-idx) | const |  |
| [`CN_KVP_VAL`](#cn-kvp-val) | const |  |
| [`CN_VSS_IDX`](#cn-vss-idx) | const |  |
| [`CN_VSS_VAL`](#cn-vss-val) | const |  |
| [`MODULE_INIT_IGNORE_MODVERSIONS`](#module-init-ignore-modversions) | const |  |
| [`MODULE_INIT_IGNORE_VERMAGIC`](#module-init-ignore-vermagic) | const |  |
| [`SOF_TIMESTAMPING_TX_HARDWARE`](#sof-timestamping-tx-hardware) | const |  |
| [`SOF_TIMESTAMPING_TX_SOFTWARE`](#sof-timestamping-tx-software) | const |  |
| [`SOF_TIMESTAMPING_RX_HARDWARE`](#sof-timestamping-rx-hardware) | const |  |
| [`SOF_TIMESTAMPING_RX_SOFTWARE`](#sof-timestamping-rx-software) | const |  |
| [`SOF_TIMESTAMPING_SOFTWARE`](#sof-timestamping-software) | const |  |
| [`SOF_TIMESTAMPING_SYS_HARDWARE`](#sof-timestamping-sys-hardware) | const |  |
| [`SOF_TIMESTAMPING_RAW_HARDWARE`](#sof-timestamping-raw-hardware) | const |  |
| [`SOF_TIMESTAMPING_OPT_ID`](#sof-timestamping-opt-id) | const |  |
| [`SOF_TIMESTAMPING_TX_SCHED`](#sof-timestamping-tx-sched) | const |  |
| [`SOF_TIMESTAMPING_TX_ACK`](#sof-timestamping-tx-ack) | const |  |
| [`SOF_TIMESTAMPING_OPT_CMSG`](#sof-timestamping-opt-cmsg) | const |  |
| [`SOF_TIMESTAMPING_OPT_TSONLY`](#sof-timestamping-opt-tsonly) | const |  |
| [`SOF_TIMESTAMPING_OPT_STATS`](#sof-timestamping-opt-stats) | const |  |
| [`SOF_TIMESTAMPING_OPT_PKTINFO`](#sof-timestamping-opt-pktinfo) | const |  |
| [`SOF_TIMESTAMPING_OPT_TX_SWHW`](#sof-timestamping-opt-tx-swhw) | const |  |
| [`SOF_TIMESTAMPING_BIND_PHC`](#sof-timestamping-bind-phc) | const |  |
| [`SOF_TIMESTAMPING_OPT_ID_TCP`](#sof-timestamping-opt-id-tcp) | const |  |
| [`SOF_TIMESTAMPING_OPT_RX_FILTER`](#sof-timestamping-opt-rx-filter) | const |  |
| [`SOF_TXTIME_DEADLINE_MODE`](#sof-txtime-deadline-mode) | const |  |
| [`SOF_TXTIME_REPORT_ERRORS`](#sof-txtime-report-errors) | const |  |
| [`HWTSTAMP_TX_OFF`](#hwtstamp-tx-off) | const |  |
| [`HWTSTAMP_TX_ON`](#hwtstamp-tx-on) | const |  |
| [`HWTSTAMP_TX_ONESTEP_SYNC`](#hwtstamp-tx-onestep-sync) | const |  |
| [`HWTSTAMP_TX_ONESTEP_P2P`](#hwtstamp-tx-onestep-p2p) | const |  |
| [`HWTSTAMP_FILTER_NONE`](#hwtstamp-filter-none) | const |  |
| [`HWTSTAMP_FILTER_ALL`](#hwtstamp-filter-all) | const |  |
| [`HWTSTAMP_FILTER_SOME`](#hwtstamp-filter-some) | const |  |
| [`HWTSTAMP_FILTER_PTP_V1_L4_EVENT`](#hwtstamp-filter-ptp-v1-l4-event) | const |  |
| [`HWTSTAMP_FILTER_PTP_V1_L4_SYNC`](#hwtstamp-filter-ptp-v1-l4-sync) | const |  |
| [`HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ`](#hwtstamp-filter-ptp-v1-l4-delay-req) | const |  |
| [`HWTSTAMP_FILTER_PTP_V2_L4_EVENT`](#hwtstamp-filter-ptp-v2-l4-event) | const |  |
| [`HWTSTAMP_FILTER_PTP_V2_L4_SYNC`](#hwtstamp-filter-ptp-v2-l4-sync) | const |  |
| [`HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ`](#hwtstamp-filter-ptp-v2-l4-delay-req) | const |  |
| [`HWTSTAMP_FILTER_PTP_V2_L2_EVENT`](#hwtstamp-filter-ptp-v2-l2-event) | const |  |
| [`HWTSTAMP_FILTER_PTP_V2_L2_SYNC`](#hwtstamp-filter-ptp-v2-l2-sync) | const |  |
| [`HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ`](#hwtstamp-filter-ptp-v2-l2-delay-req) | const |  |
| [`HWTSTAMP_FILTER_PTP_V2_EVENT`](#hwtstamp-filter-ptp-v2-event) | const |  |
| [`HWTSTAMP_FILTER_PTP_V2_SYNC`](#hwtstamp-filter-ptp-v2-sync) | const |  |
| [`HWTSTAMP_FILTER_PTP_V2_DELAY_REQ`](#hwtstamp-filter-ptp-v2-delay-req) | const |  |
| [`HWTSTAMP_FILTER_NTP_ALL`](#hwtstamp-filter-ntp-all) | const |  |
| [`PTP_MAX_SAMPLES`](#ptp-max-samples) | const |  |
| [`PTP_CLK_MAGIC`](#ptp-clk-magic) | const |  |
| [`PTP_CLOCK_GETCAPS`](#ptp-clock-getcaps) | const |  |
| [`PTP_EXTTS_REQUEST`](#ptp-extts-request) | const |  |
| [`PTP_PEROUT_REQUEST`](#ptp-perout-request) | const |  |
| [`PTP_ENABLE_PPS`](#ptp-enable-pps) | const |  |
| [`PTP_SYS_OFFSET`](#ptp-sys-offset) | const |  |
| [`PTP_PIN_GETFUNC`](#ptp-pin-getfunc) | const |  |
| [`PTP_PIN_SETFUNC`](#ptp-pin-setfunc) | const |  |
| [`PTP_SYS_OFFSET_PRECISE`](#ptp-sys-offset-precise) | const |  |
| [`PTP_SYS_OFFSET_EXTENDED`](#ptp-sys-offset-extended) | const |  |
| [`PTP_CLOCK_GETCAPS2`](#ptp-clock-getcaps2) | const |  |
| [`PTP_EXTTS_REQUEST2`](#ptp-extts-request2) | const |  |
| [`PTP_PEROUT_REQUEST2`](#ptp-perout-request2) | const |  |
| [`PTP_ENABLE_PPS2`](#ptp-enable-pps2) | const |  |
| [`PTP_SYS_OFFSET2`](#ptp-sys-offset2) | const |  |
| [`PTP_PIN_GETFUNC2`](#ptp-pin-getfunc2) | const |  |
| [`PTP_PIN_SETFUNC2`](#ptp-pin-setfunc2) | const |  |
| [`PTP_SYS_OFFSET_PRECISE2`](#ptp-sys-offset-precise2) | const |  |
| [`PTP_SYS_OFFSET_EXTENDED2`](#ptp-sys-offset-extended2) | const |  |
| [`PTP_PF_NONE`](#ptp-pf-none) | const |  |
| [`PTP_PF_EXTTS`](#ptp-pf-extts) | const |  |
| [`PTP_PF_PEROUT`](#ptp-pf-perout) | const |  |
| [`PTP_PF_PHYSYNC`](#ptp-pf-physync) | const |  |
| [`TLS_TX`](#tls-tx) | const |  |
| [`TLS_RX`](#tls-rx) | const |  |
| [`TLS_TX_ZEROCOPY_RO`](#tls-tx-zerocopy-ro) | const |  |
| [`TLS_RX_EXPECT_NO_PAD`](#tls-rx-expect-no-pad) | const |  |
| [`TLS_1_2_VERSION_MAJOR`](#tls-1-2-version-major) | const |  |
| [`TLS_1_2_VERSION_MINOR`](#tls-1-2-version-minor) | const |  |
| [`TLS_1_2_VERSION`](#tls-1-2-version) | const |  |
| [`TLS_1_3_VERSION_MAJOR`](#tls-1-3-version-major) | const |  |
| [`TLS_1_3_VERSION_MINOR`](#tls-1-3-version-minor) | const |  |
| [`TLS_1_3_VERSION`](#tls-1-3-version) | const |  |
| [`TLS_CIPHER_AES_GCM_128`](#tls-cipher-aes-gcm-128) | const |  |
| [`TLS_CIPHER_AES_GCM_128_IV_SIZE`](#tls-cipher-aes-gcm-128-iv-size) | const |  |
| [`TLS_CIPHER_AES_GCM_128_KEY_SIZE`](#tls-cipher-aes-gcm-128-key-size) | const |  |
| [`TLS_CIPHER_AES_GCM_128_SALT_SIZE`](#tls-cipher-aes-gcm-128-salt-size) | const |  |
| [`TLS_CIPHER_AES_GCM_128_TAG_SIZE`](#tls-cipher-aes-gcm-128-tag-size) | const |  |
| [`TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE`](#tls-cipher-aes-gcm-128-rec-seq-size) | const |  |
| [`TLS_CIPHER_AES_GCM_256`](#tls-cipher-aes-gcm-256) | const |  |
| [`TLS_CIPHER_AES_GCM_256_IV_SIZE`](#tls-cipher-aes-gcm-256-iv-size) | const |  |
| [`TLS_CIPHER_AES_GCM_256_KEY_SIZE`](#tls-cipher-aes-gcm-256-key-size) | const |  |
| [`TLS_CIPHER_AES_GCM_256_SALT_SIZE`](#tls-cipher-aes-gcm-256-salt-size) | const |  |
| [`TLS_CIPHER_AES_GCM_256_TAG_SIZE`](#tls-cipher-aes-gcm-256-tag-size) | const |  |
| [`TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE`](#tls-cipher-aes-gcm-256-rec-seq-size) | const |  |
| [`TLS_CIPHER_AES_CCM_128`](#tls-cipher-aes-ccm-128) | const |  |
| [`TLS_CIPHER_AES_CCM_128_IV_SIZE`](#tls-cipher-aes-ccm-128-iv-size) | const |  |
| [`TLS_CIPHER_AES_CCM_128_KEY_SIZE`](#tls-cipher-aes-ccm-128-key-size) | const |  |
| [`TLS_CIPHER_AES_CCM_128_SALT_SIZE`](#tls-cipher-aes-ccm-128-salt-size) | const |  |
| [`TLS_CIPHER_AES_CCM_128_TAG_SIZE`](#tls-cipher-aes-ccm-128-tag-size) | const |  |
| [`TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE`](#tls-cipher-aes-ccm-128-rec-seq-size) | const |  |
| [`TLS_CIPHER_CHACHA20_POLY1305`](#tls-cipher-chacha20-poly1305) | const |  |
| [`TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE`](#tls-cipher-chacha20-poly1305-iv-size) | const |  |
| [`TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE`](#tls-cipher-chacha20-poly1305-key-size) | const |  |
| [`TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE`](#tls-cipher-chacha20-poly1305-salt-size) | const |  |
| [`TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE`](#tls-cipher-chacha20-poly1305-tag-size) | const |  |
| [`TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE`](#tls-cipher-chacha20-poly1305-rec-seq-size) | const |  |
| [`TLS_CIPHER_SM4_GCM`](#tls-cipher-sm4-gcm) | const |  |
| [`TLS_CIPHER_SM4_GCM_IV_SIZE`](#tls-cipher-sm4-gcm-iv-size) | const |  |
| [`TLS_CIPHER_SM4_GCM_KEY_SIZE`](#tls-cipher-sm4-gcm-key-size) | const |  |
| [`TLS_CIPHER_SM4_GCM_SALT_SIZE`](#tls-cipher-sm4-gcm-salt-size) | const |  |
| [`TLS_CIPHER_SM4_GCM_TAG_SIZE`](#tls-cipher-sm4-gcm-tag-size) | const |  |
| [`TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE`](#tls-cipher-sm4-gcm-rec-seq-size) | const |  |
| [`TLS_CIPHER_SM4_CCM`](#tls-cipher-sm4-ccm) | const |  |
| [`TLS_CIPHER_SM4_CCM_IV_SIZE`](#tls-cipher-sm4-ccm-iv-size) | const |  |
| [`TLS_CIPHER_SM4_CCM_KEY_SIZE`](#tls-cipher-sm4-ccm-key-size) | const |  |
| [`TLS_CIPHER_SM4_CCM_SALT_SIZE`](#tls-cipher-sm4-ccm-salt-size) | const |  |
| [`TLS_CIPHER_SM4_CCM_TAG_SIZE`](#tls-cipher-sm4-ccm-tag-size) | const |  |
| [`TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE`](#tls-cipher-sm4-ccm-rec-seq-size) | const |  |
| [`TLS_CIPHER_ARIA_GCM_128`](#tls-cipher-aria-gcm-128) | const |  |
| [`TLS_CIPHER_ARIA_GCM_128_IV_SIZE`](#tls-cipher-aria-gcm-128-iv-size) | const |  |
| [`TLS_CIPHER_ARIA_GCM_128_KEY_SIZE`](#tls-cipher-aria-gcm-128-key-size) | const |  |
| [`TLS_CIPHER_ARIA_GCM_128_SALT_SIZE`](#tls-cipher-aria-gcm-128-salt-size) | const |  |
| [`TLS_CIPHER_ARIA_GCM_128_TAG_SIZE`](#tls-cipher-aria-gcm-128-tag-size) | const |  |
| [`TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE`](#tls-cipher-aria-gcm-128-rec-seq-size) | const |  |
| [`TLS_CIPHER_ARIA_GCM_256`](#tls-cipher-aria-gcm-256) | const |  |
| [`TLS_CIPHER_ARIA_GCM_256_IV_SIZE`](#tls-cipher-aria-gcm-256-iv-size) | const |  |
| [`TLS_CIPHER_ARIA_GCM_256_KEY_SIZE`](#tls-cipher-aria-gcm-256-key-size) | const |  |
| [`TLS_CIPHER_ARIA_GCM_256_SALT_SIZE`](#tls-cipher-aria-gcm-256-salt-size) | const |  |
| [`TLS_CIPHER_ARIA_GCM_256_TAG_SIZE`](#tls-cipher-aria-gcm-256-tag-size) | const |  |
| [`TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE`](#tls-cipher-aria-gcm-256-rec-seq-size) | const |  |
| [`TLS_SET_RECORD_TYPE`](#tls-set-record-type) | const |  |
| [`TLS_GET_RECORD_TYPE`](#tls-get-record-type) | const |  |
| [`SOL_TLS`](#sol-tls) | const |  |
| [`TLS_INFO_UNSPEC`](#tls-info-unspec) | const |  |
| [`TLS_INFO_VERSION`](#tls-info-version) | const |  |
| [`TLS_INFO_CIPHER`](#tls-info-cipher) | const |  |
| [`TLS_INFO_TXCONF`](#tls-info-txconf) | const |  |
| [`TLS_INFO_RXCONF`](#tls-info-rxconf) | const |  |
| [`TLS_INFO_ZC_RO_TX`](#tls-info-zc-ro-tx) | const |  |
| [`TLS_INFO_RX_NO_PAD`](#tls-info-rx-no-pad) | const |  |
| [`TLS_CONF_BASE`](#tls-conf-base) | const |  |
| [`TLS_CONF_SW`](#tls-conf-sw) | const |  |
| [`TLS_CONF_HW`](#tls-conf-hw) | const |  |
| [`TLS_CONF_HW_RECORD`](#tls-conf-hw-record) | const |  |
| [`ALG_SET_KEY`](#alg-set-key) | const |  |
| [`ALG_SET_IV`](#alg-set-iv) | const |  |
| [`ALG_SET_OP`](#alg-set-op) | const |  |
| [`ALG_SET_AEAD_ASSOCLEN`](#alg-set-aead-assoclen) | const |  |
| [`ALG_SET_AEAD_AUTHSIZE`](#alg-set-aead-authsize) | const |  |
| [`ALG_SET_DRBG_ENTROPY`](#alg-set-drbg-entropy) | const |  |
| [`ALG_SET_KEY_BY_KEY_SERIAL`](#alg-set-key-by-key-serial) | const |  |
| [`ALG_OP_DECRYPT`](#alg-op-decrypt) | const |  |
| [`ALG_OP_ENCRYPT`](#alg-op-encrypt) | const |  |
| [`IF_OPER_UNKNOWN`](#if-oper-unknown) | const |  |
| [`IF_OPER_NOTPRESENT`](#if-oper-notpresent) | const |  |
| [`IF_OPER_DOWN`](#if-oper-down) | const |  |
| [`IF_OPER_LOWERLAYERDOWN`](#if-oper-lowerlayerdown) | const |  |
| [`IF_OPER_TESTING`](#if-oper-testing) | const |  |
| [`IF_OPER_DORMANT`](#if-oper-dormant) | const |  |
| [`IF_OPER_UP`](#if-oper-up) | const |  |
| [`IF_LINK_MODE_DEFAULT`](#if-link-mode-default) | const |  |
| [`IF_LINK_MODE_DORMANT`](#if-link-mode-dormant) | const |  |
| [`IF_LINK_MODE_TESTING`](#if-link-mode-testing) | const |  |
| [`MAP_SHARED_VALIDATE`](#map-shared-validate) | const |  |
| [`MAP_DROPPABLE`](#map-droppable) | const |  |
| [`VMADDR_CID_ANY`](#vmaddr-cid-any) | const |  |
| [`VMADDR_CID_HYPERVISOR`](#vmaddr-cid-hypervisor) | const |  |
| [`VMADDR_CID_RESERVED`](#vmaddr-cid-reserved) | const |  |
| [`VMADDR_CID_LOCAL`](#vmaddr-cid-local) | const |  |
| [`VMADDR_CID_HOST`](#vmaddr-cid-host) | const |  |
| [`VMADDR_PORT_ANY`](#vmaddr-port-any) | const |  |
| [`IN_ACCESS`](#in-access) | const |  |
| [`IN_MODIFY`](#in-modify) | const |  |
| [`IN_ATTRIB`](#in-attrib) | const |  |
| [`IN_CLOSE_WRITE`](#in-close-write) | const |  |
| [`IN_CLOSE_NOWRITE`](#in-close-nowrite) | const |  |
| [`IN_CLOSE`](#in-close) | const |  |
| [`IN_OPEN`](#in-open) | const |  |
| [`IN_MOVED_FROM`](#in-moved-from) | const |  |
| [`IN_MOVED_TO`](#in-moved-to) | const |  |
| [`IN_MOVE`](#in-move) | const |  |
| [`IN_CREATE`](#in-create) | const |  |
| [`IN_DELETE`](#in-delete) | const |  |
| [`IN_DELETE_SELF`](#in-delete-self) | const |  |
| [`IN_MOVE_SELF`](#in-move-self) | const |  |
| [`IN_UNMOUNT`](#in-unmount) | const |  |
| [`IN_Q_OVERFLOW`](#in-q-overflow) | const |  |
| [`IN_IGNORED`](#in-ignored) | const |  |
| [`IN_ONLYDIR`](#in-onlydir) | const |  |
| [`IN_DONT_FOLLOW`](#in-dont-follow) | const |  |
| [`IN_EXCL_UNLINK`](#in-excl-unlink) | const |  |
| [`SECURE_NOROOT`](#secure-noroot) | const |  |
| [`SECURE_NOROOT_LOCKED`](#secure-noroot-locked) | const |  |
| [`SECBIT_NOROOT`](#secbit-noroot) | const |  |
| [`SECBIT_NOROOT_LOCKED`](#secbit-noroot-locked) | const |  |
| [`SECURE_NO_SETUID_FIXUP`](#secure-no-setuid-fixup) | const |  |
| [`SECURE_NO_SETUID_FIXUP_LOCKED`](#secure-no-setuid-fixup-locked) | const |  |
| [`SECBIT_NO_SETUID_FIXUP`](#secbit-no-setuid-fixup) | const |  |
| [`SECBIT_NO_SETUID_FIXUP_LOCKED`](#secbit-no-setuid-fixup-locked) | const |  |
| [`SECURE_KEEP_CAPS`](#secure-keep-caps) | const |  |
| [`SECURE_KEEP_CAPS_LOCKED`](#secure-keep-caps-locked) | const |  |
| [`SECBIT_KEEP_CAPS`](#secbit-keep-caps) | const |  |
| [`SECBIT_KEEP_CAPS_LOCKED`](#secbit-keep-caps-locked) | const |  |
| [`SECURE_NO_CAP_AMBIENT_RAISE`](#secure-no-cap-ambient-raise) | const |  |
| [`SECURE_NO_CAP_AMBIENT_RAISE_LOCKED`](#secure-no-cap-ambient-raise-locked) | const |  |
| [`SECBIT_NO_CAP_AMBIENT_RAISE`](#secbit-no-cap-ambient-raise) | const |  |
| [`SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED`](#secbit-no-cap-ambient-raise-locked) | const |  |
| [`SECURE_EXEC_RESTRICT_FILE`](#secure-exec-restrict-file) | const |  |
| [`SECURE_EXEC_RESTRICT_FILE_LOCKED`](#secure-exec-restrict-file-locked) | const |  |
| [`SECBIT_EXEC_RESTRICT_FILE`](#secbit-exec-restrict-file) | const |  |
| [`SECBIT_EXEC_RESTRICT_FILE_LOCKED`](#secbit-exec-restrict-file-locked) | const |  |
| [`SECURE_EXEC_DENY_INTERACTIVE`](#secure-exec-deny-interactive) | const |  |
| [`SECURE_EXEC_DENY_INTERACTIVE_LOCKED`](#secure-exec-deny-interactive-locked) | const |  |
| [`SECBIT_EXEC_DENY_INTERACTIVE`](#secbit-exec-deny-interactive) | const |  |
| [`SECBIT_EXEC_DENY_INTERACTIVE_LOCKED`](#secbit-exec-deny-interactive-locked) | const |  |
| [`SECUREBITS_DEFAULT`](#securebits-default) | const |  |
| [`SECURE_ALL_BITS`](#secure-all-bits) | const |  |
| [`SECURE_ALL_LOCKS`](#secure-all-locks) | const |  |
| [`SECURE_ALL_UNPRIVILEGED`](#secure-all-unprivileged) | const |  |
| [`IN_MASK_CREATE`](#in-mask-create) | const |  |
| [`IN_MASK_ADD`](#in-mask-add) | const |  |
| [`IN_ISDIR`](#in-isdir) | const |  |
| [`IN_ONESHOT`](#in-oneshot) | const |  |
| [`IN_ALL_EVENTS`](#in-all-events) | const |  |
| [`IN_CLOEXEC`](#in-cloexec) | const |  |
| [`IN_NONBLOCK`](#in-nonblock) | const |  |
| [`OPEN_TREE_CLONE`](#open-tree-clone) | const |  |
| [`OPEN_TREE_CLOEXEC`](#open-tree-cloexec) | const |  |
| [`NFT_TABLE_MAXNAMELEN`](#nft-table-maxnamelen) | const |  |
| [`NFT_CHAIN_MAXNAMELEN`](#nft-chain-maxnamelen) | const |  |
| [`NFT_SET_MAXNAMELEN`](#nft-set-maxnamelen) | const |  |
| [`NFT_OBJ_MAXNAMELEN`](#nft-obj-maxnamelen) | const |  |
| [`NFT_USERDATA_MAXLEN`](#nft-userdata-maxlen) | const |  |
| [`NFT_REG_VERDICT`](#nft-reg-verdict) | const |  |
| [`NFT_REG_1`](#nft-reg-1) | const |  |
| [`NFT_REG_2`](#nft-reg-2) | const |  |
| [`NFT_REG_3`](#nft-reg-3) | const |  |
| [`NFT_REG_4`](#nft-reg-4) | const |  |
| [`__NFT_REG_MAX`](#nft-reg-max) | const |  |
| [`NFT_REG32_00`](#nft-reg32-00) | const |  |
| [`NFT_REG32_01`](#nft-reg32-01) | const |  |
| [`NFT_REG32_02`](#nft-reg32-02) | const |  |
| [`NFT_REG32_03`](#nft-reg32-03) | const |  |
| [`NFT_REG32_04`](#nft-reg32-04) | const |  |
| [`NFT_REG32_05`](#nft-reg32-05) | const |  |
| [`NFT_REG32_06`](#nft-reg32-06) | const |  |
| [`NFT_REG32_07`](#nft-reg32-07) | const |  |
| [`NFT_REG32_08`](#nft-reg32-08) | const |  |
| [`NFT_REG32_09`](#nft-reg32-09) | const |  |
| [`NFT_REG32_10`](#nft-reg32-10) | const |  |
| [`NFT_REG32_11`](#nft-reg32-11) | const |  |
| [`NFT_REG32_12`](#nft-reg32-12) | const |  |
| [`NFT_REG32_13`](#nft-reg32-13) | const |  |
| [`NFT_REG32_14`](#nft-reg32-14) | const |  |
| [`NFT_REG32_15`](#nft-reg32-15) | const |  |
| [`NFT_REG_SIZE`](#nft-reg-size) | const |  |
| [`NFT_REG32_SIZE`](#nft-reg32-size) | const |  |
| [`NFT_CONTINUE`](#nft-continue) | const |  |
| [`NFT_BREAK`](#nft-break) | const |  |
| [`NFT_JUMP`](#nft-jump) | const |  |
| [`NFT_GOTO`](#nft-goto) | const |  |
| [`NFT_RETURN`](#nft-return) | const |  |
| [`NFT_MSG_NEWTABLE`](#nft-msg-newtable) | const |  |
| [`NFT_MSG_GETTABLE`](#nft-msg-gettable) | const |  |
| [`NFT_MSG_DELTABLE`](#nft-msg-deltable) | const |  |
| [`NFT_MSG_NEWCHAIN`](#nft-msg-newchain) | const |  |
| [`NFT_MSG_GETCHAIN`](#nft-msg-getchain) | const |  |
| [`NFT_MSG_DELCHAIN`](#nft-msg-delchain) | const |  |
| [`NFT_MSG_NEWRULE`](#nft-msg-newrule) | const |  |
| [`NFT_MSG_GETRULE`](#nft-msg-getrule) | const |  |
| [`NFT_MSG_DELRULE`](#nft-msg-delrule) | const |  |
| [`NFT_MSG_NEWSET`](#nft-msg-newset) | const |  |
| [`NFT_MSG_GETSET`](#nft-msg-getset) | const |  |
| [`NFT_MSG_DELSET`](#nft-msg-delset) | const |  |
| [`NFT_MSG_NEWSETELEM`](#nft-msg-newsetelem) | const |  |
| [`NFT_MSG_GETSETELEM`](#nft-msg-getsetelem) | const |  |
| [`NFT_MSG_DELSETELEM`](#nft-msg-delsetelem) | const |  |
| [`NFT_MSG_NEWGEN`](#nft-msg-newgen) | const |  |
| [`NFT_MSG_GETGEN`](#nft-msg-getgen) | const |  |
| [`NFT_MSG_TRACE`](#nft-msg-trace) | const |  |
| [`NFT_MSG_NEWOBJ`](#nft-msg-newobj) | const |  |
| [`NFT_MSG_GETOBJ`](#nft-msg-getobj) | const |  |
| [`NFT_MSG_DELOBJ`](#nft-msg-delobj) | const |  |
| [`NFT_MSG_GETOBJ_RESET`](#nft-msg-getobj-reset) | const |  |
| [`NFT_MSG_MAX`](#nft-msg-max) | const |  |
| [`NFT_SET_ANONYMOUS`](#nft-set-anonymous) | const |  |
| [`NFT_SET_CONSTANT`](#nft-set-constant) | const |  |
| [`NFT_SET_INTERVAL`](#nft-set-interval) | const |  |
| [`NFT_SET_MAP`](#nft-set-map) | const |  |
| [`NFT_SET_TIMEOUT`](#nft-set-timeout) | const |  |
| [`NFT_SET_EVAL`](#nft-set-eval) | const |  |
| [`NFT_SET_POL_PERFORMANCE`](#nft-set-pol-performance) | const |  |
| [`NFT_SET_POL_MEMORY`](#nft-set-pol-memory) | const |  |
| [`NFT_SET_ELEM_INTERVAL_END`](#nft-set-elem-interval-end) | const |  |
| [`NFT_DATA_VALUE`](#nft-data-value) | const |  |
| [`NFT_DATA_VERDICT`](#nft-data-verdict) | const |  |
| [`NFT_DATA_RESERVED_MASK`](#nft-data-reserved-mask) | const |  |
| [`NFT_DATA_VALUE_MAXLEN`](#nft-data-value-maxlen) | const |  |
| [`NFT_BYTEORDER_NTOH`](#nft-byteorder-ntoh) | const |  |
| [`NFT_BYTEORDER_HTON`](#nft-byteorder-hton) | const |  |
| [`NFT_CMP_EQ`](#nft-cmp-eq) | const |  |
| [`NFT_CMP_NEQ`](#nft-cmp-neq) | const |  |
| [`NFT_CMP_LT`](#nft-cmp-lt) | const |  |
| [`NFT_CMP_LTE`](#nft-cmp-lte) | const |  |
| [`NFT_CMP_GT`](#nft-cmp-gt) | const |  |
| [`NFT_CMP_GTE`](#nft-cmp-gte) | const |  |
| [`NFT_RANGE_EQ`](#nft-range-eq) | const |  |
| [`NFT_RANGE_NEQ`](#nft-range-neq) | const |  |
| [`NFT_LOOKUP_F_INV`](#nft-lookup-f-inv) | const |  |
| [`NFT_DYNSET_OP_ADD`](#nft-dynset-op-add) | const |  |
| [`NFT_DYNSET_OP_UPDATE`](#nft-dynset-op-update) | const |  |
| [`NFT_DYNSET_F_INV`](#nft-dynset-f-inv) | const |  |
| [`NFT_PAYLOAD_LL_HEADER`](#nft-payload-ll-header) | const |  |
| [`NFT_PAYLOAD_NETWORK_HEADER`](#nft-payload-network-header) | const |  |
| [`NFT_PAYLOAD_TRANSPORT_HEADER`](#nft-payload-transport-header) | const |  |
| [`NFT_PAYLOAD_CSUM_NONE`](#nft-payload-csum-none) | const |  |
| [`NFT_PAYLOAD_CSUM_INET`](#nft-payload-csum-inet) | const |  |
| [`NFT_META_LEN`](#nft-meta-len) | const |  |
| [`NFT_META_PROTOCOL`](#nft-meta-protocol) | const |  |
| [`NFT_META_PRIORITY`](#nft-meta-priority) | const |  |
| [`NFT_META_MARK`](#nft-meta-mark) | const |  |
| [`NFT_META_IIF`](#nft-meta-iif) | const |  |
| [`NFT_META_OIF`](#nft-meta-oif) | const |  |
| [`NFT_META_IIFNAME`](#nft-meta-iifname) | const |  |
| [`NFT_META_OIFNAME`](#nft-meta-oifname) | const |  |
| [`NFT_META_IIFTYPE`](#nft-meta-iiftype) | const |  |
| [`NFT_META_OIFTYPE`](#nft-meta-oiftype) | const |  |
| [`NFT_META_SKUID`](#nft-meta-skuid) | const |  |
| [`NFT_META_SKGID`](#nft-meta-skgid) | const |  |
| [`NFT_META_NFTRACE`](#nft-meta-nftrace) | const |  |
| [`NFT_META_RTCLASSID`](#nft-meta-rtclassid) | const |  |
| [`NFT_META_SECMARK`](#nft-meta-secmark) | const |  |
| [`NFT_META_NFPROTO`](#nft-meta-nfproto) | const |  |
| [`NFT_META_L4PROTO`](#nft-meta-l4proto) | const |  |
| [`NFT_META_BRI_IIFNAME`](#nft-meta-bri-iifname) | const |  |
| [`NFT_META_BRI_OIFNAME`](#nft-meta-bri-oifname) | const |  |
| [`NFT_META_PKTTYPE`](#nft-meta-pkttype) | const |  |
| [`NFT_META_CPU`](#nft-meta-cpu) | const |  |
| [`NFT_META_IIFGROUP`](#nft-meta-iifgroup) | const |  |
| [`NFT_META_OIFGROUP`](#nft-meta-oifgroup) | const |  |
| [`NFT_META_CGROUP`](#nft-meta-cgroup) | const |  |
| [`NFT_META_PRANDOM`](#nft-meta-prandom) | const |  |
| [`NFT_CT_STATE`](#nft-ct-state) | const |  |
| [`NFT_CT_DIRECTION`](#nft-ct-direction) | const |  |
| [`NFT_CT_STATUS`](#nft-ct-status) | const |  |
| [`NFT_CT_MARK`](#nft-ct-mark) | const |  |
| [`NFT_CT_SECMARK`](#nft-ct-secmark) | const |  |
| [`NFT_CT_EXPIRATION`](#nft-ct-expiration) | const |  |
| [`NFT_CT_HELPER`](#nft-ct-helper) | const |  |
| [`NFT_CT_L3PROTOCOL`](#nft-ct-l3protocol) | const |  |
| [`NFT_CT_SRC`](#nft-ct-src) | const |  |
| [`NFT_CT_DST`](#nft-ct-dst) | const |  |
| [`NFT_CT_PROTOCOL`](#nft-ct-protocol) | const |  |
| [`NFT_CT_PROTO_SRC`](#nft-ct-proto-src) | const |  |
| [`NFT_CT_PROTO_DST`](#nft-ct-proto-dst) | const |  |
| [`NFT_CT_LABELS`](#nft-ct-labels) | const |  |
| [`NFT_CT_PKTS`](#nft-ct-pkts) | const |  |
| [`NFT_CT_BYTES`](#nft-ct-bytes) | const |  |
| [`NFT_CT_AVGPKT`](#nft-ct-avgpkt) | const |  |
| [`NFT_CT_ZONE`](#nft-ct-zone) | const |  |
| [`NFT_CT_EVENTMASK`](#nft-ct-eventmask) | const |  |
| [`NFT_CT_SRC_IP`](#nft-ct-src-ip) | const |  |
| [`NFT_CT_DST_IP`](#nft-ct-dst-ip) | const |  |
| [`NFT_CT_SRC_IP6`](#nft-ct-src-ip6) | const |  |
| [`NFT_CT_DST_IP6`](#nft-ct-dst-ip6) | const |  |
| [`NFT_LIMIT_PKTS`](#nft-limit-pkts) | const |  |
| [`NFT_LIMIT_PKT_BYTES`](#nft-limit-pkt-bytes) | const |  |
| [`NFT_LIMIT_F_INV`](#nft-limit-f-inv) | const |  |
| [`NFT_QUEUE_FLAG_BYPASS`](#nft-queue-flag-bypass) | const |  |
| [`NFT_QUEUE_FLAG_CPU_FANOUT`](#nft-queue-flag-cpu-fanout) | const |  |
| [`NFT_QUEUE_FLAG_MASK`](#nft-queue-flag-mask) | const |  |
| [`NFT_QUOTA_F_INV`](#nft-quota-f-inv) | const |  |
| [`NFT_REJECT_ICMP_UNREACH`](#nft-reject-icmp-unreach) | const |  |
| [`NFT_REJECT_TCP_RST`](#nft-reject-tcp-rst) | const |  |
| [`NFT_REJECT_ICMPX_UNREACH`](#nft-reject-icmpx-unreach) | const |  |
| [`NFT_REJECT_ICMPX_NO_ROUTE`](#nft-reject-icmpx-no-route) | const |  |
| [`NFT_REJECT_ICMPX_PORT_UNREACH`](#nft-reject-icmpx-port-unreach) | const |  |
| [`NFT_REJECT_ICMPX_HOST_UNREACH`](#nft-reject-icmpx-host-unreach) | const |  |
| [`NFT_REJECT_ICMPX_ADMIN_PROHIBITED`](#nft-reject-icmpx-admin-prohibited) | const |  |
| [`NFT_NAT_SNAT`](#nft-nat-snat) | const |  |
| [`NFT_NAT_DNAT`](#nft-nat-dnat) | const |  |
| [`NFT_TRACETYPE_UNSPEC`](#nft-tracetype-unspec) | const |  |
| [`NFT_TRACETYPE_POLICY`](#nft-tracetype-policy) | const |  |
| [`NFT_TRACETYPE_RETURN`](#nft-tracetype-return) | const |  |
| [`NFT_TRACETYPE_RULE`](#nft-tracetype-rule) | const |  |
| [`NFT_NG_INCREMENTAL`](#nft-ng-incremental) | const |  |
| [`NFT_NG_RANDOM`](#nft-ng-random) | const |  |
| [`FF_MAX`](#ff-max) | const |  |
| [`FF_CNT`](#ff-cnt) | const |  |
| [`INPUT_PROP_POINTER`](#input-prop-pointer) | const |  |
| [`INPUT_PROP_DIRECT`](#input-prop-direct) | const |  |
| [`INPUT_PROP_BUTTONPAD`](#input-prop-buttonpad) | const |  |
| [`INPUT_PROP_SEMI_MT`](#input-prop-semi-mt) | const |  |
| [`INPUT_PROP_TOPBUTTONPAD`](#input-prop-topbuttonpad) | const |  |
| [`INPUT_PROP_POINTING_STICK`](#input-prop-pointing-stick) | const |  |
| [`INPUT_PROP_ACCELEROMETER`](#input-prop-accelerometer) | const |  |
| [`INPUT_PROP_MAX`](#input-prop-max) | const |  |
| [`INPUT_PROP_CNT`](#input-prop-cnt) | const |  |
| [`EV_MAX`](#ev-max) | const |  |
| [`EV_CNT`](#ev-cnt) | const |  |
| [`SYN_MAX`](#syn-max) | const |  |
| [`SYN_CNT`](#syn-cnt) | const |  |
| [`KEY_MAX`](#key-max) | const |  |
| [`KEY_CNT`](#key-cnt) | const |  |
| [`REL_MAX`](#rel-max) | const |  |
| [`REL_CNT`](#rel-cnt) | const |  |
| [`ABS_MAX`](#abs-max) | const |  |
| [`ABS_CNT`](#abs-cnt) | const |  |
| [`SW_MAX`](#sw-max) | const |  |
| [`SW_CNT`](#sw-cnt) | const |  |
| [`MSC_MAX`](#msc-max) | const |  |
| [`MSC_CNT`](#msc-cnt) | const |  |
| [`LED_MAX`](#led-max) | const |  |
| [`LED_CNT`](#led-cnt) | const |  |
| [`REP_MAX`](#rep-max) | const |  |
| [`REP_CNT`](#rep-cnt) | const |  |
| [`SND_MAX`](#snd-max) | const |  |
| [`SND_CNT`](#snd-cnt) | const |  |
| [`UINPUT_VERSION`](#uinput-version) | const |  |
| [`UINPUT_MAX_NAME_SIZE`](#uinput-max-name-size) | const |  |
| [`FAN_ACCESS`](#fan-access) | const |  |
| [`FAN_MODIFY`](#fan-modify) | const |  |
| [`FAN_ATTRIB`](#fan-attrib) | const |  |
| [`FAN_CLOSE_WRITE`](#fan-close-write) | const |  |
| [`FAN_CLOSE_NOWRITE`](#fan-close-nowrite) | const |  |
| [`FAN_OPEN`](#fan-open) | const |  |
| [`FAN_MOVED_FROM`](#fan-moved-from) | const |  |
| [`FAN_MOVED_TO`](#fan-moved-to) | const |  |
| [`FAN_CREATE`](#fan-create) | const |  |
| [`FAN_DELETE`](#fan-delete) | const |  |
| [`FAN_DELETE_SELF`](#fan-delete-self) | const |  |
| [`FAN_MOVE_SELF`](#fan-move-self) | const |  |
| [`FAN_OPEN_EXEC`](#fan-open-exec) | const |  |
| [`FAN_Q_OVERFLOW`](#fan-q-overflow) | const |  |
| [`FAN_FS_ERROR`](#fan-fs-error) | const |  |
| [`FAN_OPEN_PERM`](#fan-open-perm) | const |  |
| [`FAN_ACCESS_PERM`](#fan-access-perm) | const |  |
| [`FAN_OPEN_EXEC_PERM`](#fan-open-exec-perm) | const |  |
| [`FAN_EVENT_ON_CHILD`](#fan-event-on-child) | const |  |
| [`FAN_RENAME`](#fan-rename) | const |  |
| [`FAN_ONDIR`](#fan-ondir) | const |  |
| [`FAN_CLOSE`](#fan-close) | const |  |
| [`FAN_MOVE`](#fan-move) | const |  |
| [`FAN_CLOEXEC`](#fan-cloexec) | const |  |
| [`FAN_NONBLOCK`](#fan-nonblock) | const |  |
| [`FAN_CLASS_NOTIF`](#fan-class-notif) | const |  |
| [`FAN_CLASS_CONTENT`](#fan-class-content) | const |  |
| [`FAN_CLASS_PRE_CONTENT`](#fan-class-pre-content) | const |  |
| [`FAN_UNLIMITED_QUEUE`](#fan-unlimited-queue) | const |  |
| [`FAN_UNLIMITED_MARKS`](#fan-unlimited-marks) | const |  |
| [`FAN_ENABLE_AUDIT`](#fan-enable-audit) | const |  |
| [`FAN_REPORT_PIDFD`](#fan-report-pidfd) | const |  |
| [`FAN_REPORT_TID`](#fan-report-tid) | const |  |
| [`FAN_REPORT_FID`](#fan-report-fid) | const |  |
| [`FAN_REPORT_DIR_FID`](#fan-report-dir-fid) | const |  |
| [`FAN_REPORT_NAME`](#fan-report-name) | const |  |
| [`FAN_REPORT_TARGET_FID`](#fan-report-target-fid) | const |  |
| [`FAN_REPORT_DFID_NAME`](#fan-report-dfid-name) | const |  |
| [`FAN_REPORT_DFID_NAME_TARGET`](#fan-report-dfid-name-target) | const |  |
| [`FAN_MARK_ADD`](#fan-mark-add) | const |  |
| [`FAN_MARK_REMOVE`](#fan-mark-remove) | const |  |
| [`FAN_MARK_DONT_FOLLOW`](#fan-mark-dont-follow) | const |  |
| [`FAN_MARK_ONLYDIR`](#fan-mark-onlydir) | const |  |
| [`FAN_MARK_IGNORED_MASK`](#fan-mark-ignored-mask) | const |  |
| [`FAN_MARK_IGNORED_SURV_MODIFY`](#fan-mark-ignored-surv-modify) | const |  |
| [`FAN_MARK_FLUSH`](#fan-mark-flush) | const |  |
| [`FAN_MARK_EVICTABLE`](#fan-mark-evictable) | const |  |
| [`FAN_MARK_IGNORE`](#fan-mark-ignore) | const |  |
| [`FAN_MARK_INODE`](#fan-mark-inode) | const |  |
| [`FAN_MARK_MOUNT`](#fan-mark-mount) | const |  |
| [`FAN_MARK_FILESYSTEM`](#fan-mark-filesystem) | const |  |
| [`FAN_MARK_IGNORE_SURV`](#fan-mark-ignore-surv) | const |  |
| [`FANOTIFY_METADATA_VERSION`](#fanotify-metadata-version) | const |  |
| [`FAN_EVENT_INFO_TYPE_FID`](#fan-event-info-type-fid) | const |  |
| [`FAN_EVENT_INFO_TYPE_DFID_NAME`](#fan-event-info-type-dfid-name) | const |  |
| [`FAN_EVENT_INFO_TYPE_DFID`](#fan-event-info-type-dfid) | const |  |
| [`FAN_EVENT_INFO_TYPE_PIDFD`](#fan-event-info-type-pidfd) | const |  |
| [`FAN_EVENT_INFO_TYPE_ERROR`](#fan-event-info-type-error) | const |  |
| [`FAN_EVENT_INFO_TYPE_OLD_DFID_NAME`](#fan-event-info-type-old-dfid-name) | const |  |
| [`FAN_EVENT_INFO_TYPE_NEW_DFID_NAME`](#fan-event-info-type-new-dfid-name) | const |  |
| [`FAN_RESPONSE_INFO_NONE`](#fan-response-info-none) | const |  |
| [`FAN_RESPONSE_INFO_AUDIT_RULE`](#fan-response-info-audit-rule) | const |  |
| [`FAN_ALLOW`](#fan-allow) | const |  |
| [`FAN_DENY`](#fan-deny) | const |  |
| [`FAN_AUDIT`](#fan-audit) | const |  |
| [`FAN_INFO`](#fan-info) | const |  |
| [`FAN_NOFD`](#fan-nofd) | const |  |
| [`FAN_NOPIDFD`](#fan-nopidfd) | const |  |
| [`FAN_EPIDFD`](#fan-epidfd) | const |  |
| [`FUTEX_WAIT`](#futex-wait) | const |  |
| [`FUTEX_WAKE`](#futex-wake) | const |  |
| [`FUTEX_FD`](#futex-fd) | const |  |
| [`FUTEX_REQUEUE`](#futex-requeue) | const |  |
| [`FUTEX_CMP_REQUEUE`](#futex-cmp-requeue) | const |  |
| [`FUTEX_WAKE_OP`](#futex-wake-op) | const |  |
| [`FUTEX_LOCK_PI`](#futex-lock-pi) | const |  |
| [`FUTEX_UNLOCK_PI`](#futex-unlock-pi) | const |  |
| [`FUTEX_TRYLOCK_PI`](#futex-trylock-pi) | const |  |
| [`FUTEX_WAIT_BITSET`](#futex-wait-bitset) | const |  |
| [`FUTEX_WAKE_BITSET`](#futex-wake-bitset) | const |  |
| [`FUTEX_WAIT_REQUEUE_PI`](#futex-wait-requeue-pi) | const |  |
| [`FUTEX_CMP_REQUEUE_PI`](#futex-cmp-requeue-pi) | const |  |
| [`FUTEX_LOCK_PI2`](#futex-lock-pi2) | const |  |
| [`FUTEX_PRIVATE_FLAG`](#futex-private-flag) | const |  |
| [`FUTEX_CLOCK_REALTIME`](#futex-clock-realtime) | const |  |
| [`FUTEX_CMD_MASK`](#futex-cmd-mask) | const |  |
| [`FUTEX_WAITERS`](#futex-waiters) | const |  |
| [`FUTEX_OWNER_DIED`](#futex-owner-died) | const |  |
| [`FUTEX_TID_MASK`](#futex-tid-mask) | const |  |
| [`FUTEX_BITSET_MATCH_ANY`](#futex-bitset-match-any) | const |  |
| [`FUTEX_OP_SET`](#futex-op-set) | const |  |
| [`FUTEX_OP_ADD`](#futex-op-add) | const |  |
| [`FUTEX_OP_OR`](#futex-op-or) | const |  |
| [`FUTEX_OP_ANDN`](#futex-op-andn) | const |  |
| [`FUTEX_OP_XOR`](#futex-op-xor) | const |  |
| [`FUTEX_OP_OPARG_SHIFT`](#futex-op-oparg-shift) | const |  |
| [`FUTEX_OP_CMP_EQ`](#futex-op-cmp-eq) | const |  |
| [`FUTEX_OP_CMP_NE`](#futex-op-cmp-ne) | const |  |
| [`FUTEX_OP_CMP_LT`](#futex-op-cmp-lt) | const |  |
| [`FUTEX_OP_CMP_LE`](#futex-op-cmp-le) | const |  |
| [`FUTEX_OP_CMP_GT`](#futex-op-cmp-gt) | const |  |
| [`FUTEX_OP_CMP_GE`](#futex-op-cmp-ge) | const |  |
| [`KEXEC_ON_CRASH`](#kexec-on-crash) | const |  |
| [`KEXEC_PRESERVE_CONTEXT`](#kexec-preserve-context) | const |  |
| [`KEXEC_ARCH_MASK`](#kexec-arch-mask) | const |  |
| [`KEXEC_FILE_UNLOAD`](#kexec-file-unload) | const |  |
| [`KEXEC_FILE_ON_CRASH`](#kexec-file-on-crash) | const |  |
| [`KEXEC_FILE_NO_INITRAMFS`](#kexec-file-no-initramfs) | const |  |
| [`LINUX_REBOOT_MAGIC1`](#linux-reboot-magic1) | const |  |
| [`LINUX_REBOOT_MAGIC2`](#linux-reboot-magic2) | const |  |
| [`LINUX_REBOOT_MAGIC2A`](#linux-reboot-magic2a) | const |  |
| [`LINUX_REBOOT_MAGIC2B`](#linux-reboot-magic2b) | const |  |
| [`LINUX_REBOOT_MAGIC2C`](#linux-reboot-magic2c) | const |  |
| [`LINUX_REBOOT_CMD_RESTART`](#linux-reboot-cmd-restart) | const |  |
| [`LINUX_REBOOT_CMD_HALT`](#linux-reboot-cmd-halt) | const |  |
| [`LINUX_REBOOT_CMD_CAD_ON`](#linux-reboot-cmd-cad-on) | const |  |
| [`LINUX_REBOOT_CMD_CAD_OFF`](#linux-reboot-cmd-cad-off) | const |  |
| [`LINUX_REBOOT_CMD_POWER_OFF`](#linux-reboot-cmd-power-off) | const |  |
| [`LINUX_REBOOT_CMD_RESTART2`](#linux-reboot-cmd-restart2) | const |  |
| [`LINUX_REBOOT_CMD_SW_SUSPEND`](#linux-reboot-cmd-sw-suspend) | const |  |
| [`LINUX_REBOOT_CMD_KEXEC`](#linux-reboot-cmd-kexec) | const |  |
| [`SO_EE_ORIGIN_NONE`](#so-ee-origin-none) | const |  |
| [`SO_EE_ORIGIN_LOCAL`](#so-ee-origin-local) | const |  |
| [`SO_EE_ORIGIN_ICMP`](#so-ee-origin-icmp) | const |  |
| [`SO_EE_ORIGIN_ICMP6`](#so-ee-origin-icmp6) | const |  |
| [`SO_EE_ORIGIN_TXSTATUS`](#so-ee-origin-txstatus) | const |  |
| [`SO_EE_ORIGIN_TIMESTAMPING`](#so-ee-origin-timestamping) | const |  |
| [`SCTP_FUTURE_ASSOC`](#sctp-future-assoc) | const |  |
| [`SCTP_CURRENT_ASSOC`](#sctp-current-assoc) | const |  |
| [`SCTP_ALL_ASSOC`](#sctp-all-assoc) | const |  |
| [`SCTP_RTOINFO`](#sctp-rtoinfo) | const |  |
| [`SCTP_ASSOCINFO`](#sctp-associnfo) | const |  |
| [`SCTP_INITMSG`](#sctp-initmsg) | const |  |
| [`SCTP_NODELAY`](#sctp-nodelay) | const |  |
| [`SCTP_AUTOCLOSE`](#sctp-autoclose) | const |  |
| [`SCTP_SET_PEER_PRIMARY_ADDR`](#sctp-set-peer-primary-addr) | const |  |
| [`SCTP_PRIMARY_ADDR`](#sctp-primary-addr) | const |  |
| [`SCTP_ADAPTATION_LAYER`](#sctp-adaptation-layer) | const |  |
| [`SCTP_DISABLE_FRAGMENTS`](#sctp-disable-fragments) | const |  |
| [`SCTP_PEER_ADDR_PARAMS`](#sctp-peer-addr-params) | const |  |
| [`SCTP_DEFAULT_SEND_PARAM`](#sctp-default-send-param) | const |  |
| [`SCTP_EVENTS`](#sctp-events) | const |  |
| [`SCTP_I_WANT_MAPPED_V4_ADDR`](#sctp-i-want-mapped-v4-addr) | const |  |
| [`SCTP_MAXSEG`](#sctp-maxseg) | const |  |
| [`SCTP_STATUS`](#sctp-status) | const |  |
| [`SCTP_GET_PEER_ADDR_INFO`](#sctp-get-peer-addr-info) | const |  |
| [`SCTP_DELAYED_ACK_TIME`](#sctp-delayed-ack-time) | const |  |
| [`SCTP_DELAYED_ACK`](#sctp-delayed-ack) | const |  |
| [`SCTP_DELAYED_SACK`](#sctp-delayed-sack) | const |  |
| [`SCTP_CONTEXT`](#sctp-context) | const |  |
| [`SCTP_FRAGMENT_INTERLEAVE`](#sctp-fragment-interleave) | const |  |
| [`SCTP_PARTIAL_DELIVERY_POINT`](#sctp-partial-delivery-point) | const |  |
| [`SCTP_MAX_BURST`](#sctp-max-burst) | const |  |
| [`SCTP_AUTH_CHUNK`](#sctp-auth-chunk) | const |  |
| [`SCTP_HMAC_IDENT`](#sctp-hmac-ident) | const |  |
| [`SCTP_AUTH_KEY`](#sctp-auth-key) | const |  |
| [`SCTP_AUTH_ACTIVE_KEY`](#sctp-auth-active-key) | const |  |
| [`SCTP_AUTH_DELETE_KEY`](#sctp-auth-delete-key) | const |  |
| [`SCTP_PEER_AUTH_CHUNKS`](#sctp-peer-auth-chunks) | const |  |
| [`SCTP_LOCAL_AUTH_CHUNKS`](#sctp-local-auth-chunks) | const |  |
| [`SCTP_GET_ASSOC_NUMBER`](#sctp-get-assoc-number) | const |  |
| [`SCTP_GET_ASSOC_ID_LIST`](#sctp-get-assoc-id-list) | const |  |
| [`SCTP_AUTO_ASCONF`](#sctp-auto-asconf) | const |  |
| [`SCTP_PEER_ADDR_THLDS`](#sctp-peer-addr-thlds) | const |  |
| [`SCTP_RECVRCVINFO`](#sctp-recvrcvinfo) | const |  |
| [`SCTP_RECVNXTINFO`](#sctp-recvnxtinfo) | const |  |
| [`SCTP_DEFAULT_SNDINFO`](#sctp-default-sndinfo) | const |  |
| [`SCTP_AUTH_DEACTIVATE_KEY`](#sctp-auth-deactivate-key) | const |  |
| [`SCTP_REUSE_PORT`](#sctp-reuse-port) | const |  |
| [`SCTP_PEER_ADDR_THLDS_V2`](#sctp-peer-addr-thlds-v2) | const |  |
| [`SCTP_PR_SCTP_NONE`](#sctp-pr-sctp-none) | const |  |
| [`SCTP_PR_SCTP_TTL`](#sctp-pr-sctp-ttl) | const |  |
| [`SCTP_PR_SCTP_RTX`](#sctp-pr-sctp-rtx) | const |  |
| [`SCTP_PR_SCTP_PRIO`](#sctp-pr-sctp-prio) | const |  |
| [`SCTP_PR_SCTP_MAX`](#sctp-pr-sctp-max) | const |  |
| [`SCTP_PR_SCTP_MASK`](#sctp-pr-sctp-mask) | const |  |
| [`SCTP_ENABLE_RESET_STREAM_REQ`](#sctp-enable-reset-stream-req) | const |  |
| [`SCTP_ENABLE_RESET_ASSOC_REQ`](#sctp-enable-reset-assoc-req) | const |  |
| [`SCTP_ENABLE_CHANGE_ASSOC_REQ`](#sctp-enable-change-assoc-req) | const |  |
| [`SCTP_ENABLE_STRRESET_MASK`](#sctp-enable-strreset-mask) | const |  |
| [`SCTP_STREAM_RESET_INCOMING`](#sctp-stream-reset-incoming) | const |  |
| [`SCTP_STREAM_RESET_OUTGOING`](#sctp-stream-reset-outgoing) | const |  |
| [`SCTP_INIT`](#sctp-init) | const |  |
| [`SCTP_SNDRCV`](#sctp-sndrcv) | const |  |
| [`SCTP_SNDINFO`](#sctp-sndinfo) | const |  |
| [`SCTP_RCVINFO`](#sctp-rcvinfo) | const |  |
| [`SCTP_NXTINFO`](#sctp-nxtinfo) | const |  |
| [`SCTP_PRINFO`](#sctp-prinfo) | const |  |
| [`SCTP_AUTHINFO`](#sctp-authinfo) | const |  |
| [`SCTP_DSTADDRV4`](#sctp-dstaddrv4) | const |  |
| [`SCTP_DSTADDRV6`](#sctp-dstaddrv6) | const |  |
| [`SCTP_UNORDERED`](#sctp-unordered) | const |  |
| [`SCTP_ADDR_OVER`](#sctp-addr-over) | const |  |
| [`SCTP_ABORT`](#sctp-abort) | const |  |
| [`SCTP_SACK_IMMEDIATELY`](#sctp-sack-immediately) | const |  |
| [`SCTP_SENDALL`](#sctp-sendall) | const |  |
| [`SCTP_PR_SCTP_ALL`](#sctp-pr-sctp-all) | const |  |
| [`SCTP_NOTIFICATION`](#sctp-notification) | const |  |
| [`SCTP_EOF`](#sctp-eof) | const |  |
| [`DCCP_SOCKOPT_PACKET_SIZE`](#dccp-sockopt-packet-size) | const |  |
| [`DCCP_SOCKOPT_SERVICE`](#dccp-sockopt-service) | const |  |
| [`DCCP_SOCKOPT_CHANGE_L`](#dccp-sockopt-change-l) | const |  |
| [`DCCP_SOCKOPT_CHANGE_R`](#dccp-sockopt-change-r) | const |  |
| [`DCCP_SOCKOPT_GET_CUR_MPS`](#dccp-sockopt-get-cur-mps) | const |  |
| [`DCCP_SOCKOPT_SERVER_TIMEWAIT`](#dccp-sockopt-server-timewait) | const |  |
| [`DCCP_SOCKOPT_SEND_CSCOV`](#dccp-sockopt-send-cscov) | const |  |
| [`DCCP_SOCKOPT_RECV_CSCOV`](#dccp-sockopt-recv-cscov) | const |  |
| [`DCCP_SOCKOPT_AVAILABLE_CCIDS`](#dccp-sockopt-available-ccids) | const |  |
| [`DCCP_SOCKOPT_CCID`](#dccp-sockopt-ccid) | const |  |
| [`DCCP_SOCKOPT_TX_CCID`](#dccp-sockopt-tx-ccid) | const |  |
| [`DCCP_SOCKOPT_RX_CCID`](#dccp-sockopt-rx-ccid) | const |  |
| [`DCCP_SOCKOPT_QPOLICY_ID`](#dccp-sockopt-qpolicy-id) | const |  |
| [`DCCP_SOCKOPT_QPOLICY_TXQLEN`](#dccp-sockopt-qpolicy-txqlen) | const |  |
| [`DCCP_SOCKOPT_CCID_RX_INFO`](#dccp-sockopt-ccid-rx-info) | const |  |
| [`DCCP_SOCKOPT_CCID_TX_INFO`](#dccp-sockopt-ccid-tx-info) | const |  |
| [`DCCP_SERVICE_LIST_MAX_LEN`](#dccp-service-list-max-len) | const | maximum number of services provided on the same listening port |
| [`CTL_KERN`](#ctl-kern) | const |  |
| [`CTL_VM`](#ctl-vm) | const |  |
| [`CTL_NET`](#ctl-net) | const |  |
| [`CTL_FS`](#ctl-fs) | const |  |
| [`CTL_DEBUG`](#ctl-debug) | const |  |
| [`CTL_DEV`](#ctl-dev) | const |  |
| [`CTL_BUS`](#ctl-bus) | const |  |
| [`CTL_ABI`](#ctl-abi) | const |  |
| [`CTL_CPU`](#ctl-cpu) | const |  |
| [`CTL_BUS_ISA`](#ctl-bus-isa) | const |  |
| [`INOTIFY_MAX_USER_INSTANCES`](#inotify-max-user-instances) | const |  |
| [`INOTIFY_MAX_USER_WATCHES`](#inotify-max-user-watches) | const |  |
| [`INOTIFY_MAX_QUEUED_EVENTS`](#inotify-max-queued-events) | const |  |
| [`KERN_OSTYPE`](#kern-ostype) | const |  |
| [`KERN_OSRELEASE`](#kern-osrelease) | const |  |
| [`KERN_OSREV`](#kern-osrev) | const |  |
| [`KERN_VERSION`](#kern-version) | const |  |
| [`KERN_SECUREMASK`](#kern-securemask) | const |  |
| [`KERN_PROF`](#kern-prof) | const |  |
| [`KERN_NODENAME`](#kern-nodename) | const |  |
| [`KERN_DOMAINNAME`](#kern-domainname) | const |  |
| [`KERN_PANIC`](#kern-panic) | const |  |
| [`KERN_REALROOTDEV`](#kern-realrootdev) | const |  |
| [`KERN_SPARC_REBOOT`](#kern-sparc-reboot) | const |  |
| [`KERN_CTLALTDEL`](#kern-ctlaltdel) | const |  |
| [`KERN_PRINTK`](#kern-printk) | const |  |
| [`KERN_NAMETRANS`](#kern-nametrans) | const |  |
| [`KERN_PPC_HTABRECLAIM`](#kern-ppc-htabreclaim) | const |  |
| [`KERN_PPC_ZEROPAGED`](#kern-ppc-zeropaged) | const |  |
| [`KERN_PPC_POWERSAVE_NAP`](#kern-ppc-powersave-nap) | const |  |
| [`KERN_MODPROBE`](#kern-modprobe) | const |  |
| [`KERN_SG_BIG_BUFF`](#kern-sg-big-buff) | const |  |
| [`KERN_ACCT`](#kern-acct) | const |  |
| [`KERN_PPC_L2CR`](#kern-ppc-l2cr) | const |  |
| [`KERN_RTSIGNR`](#kern-rtsignr) | const |  |
| [`KERN_RTSIGMAX`](#kern-rtsigmax) | const |  |
| [`KERN_SHMMAX`](#kern-shmmax) | const |  |
| [`KERN_MSGMAX`](#kern-msgmax) | const |  |
| [`KERN_MSGMNB`](#kern-msgmnb) | const |  |
| [`KERN_MSGPOOL`](#kern-msgpool) | const |  |
| [`KERN_SYSRQ`](#kern-sysrq) | const |  |
| [`KERN_MAX_THREADS`](#kern-max-threads) | const |  |
| [`KERN_RANDOM`](#kern-random) | const |  |
| [`KERN_SHMALL`](#kern-shmall) | const |  |
| [`KERN_MSGMNI`](#kern-msgmni) | const |  |
| [`KERN_SEM`](#kern-sem) | const |  |
| [`KERN_SPARC_STOP_A`](#kern-sparc-stop-a) | const |  |
| [`KERN_SHMMNI`](#kern-shmmni) | const |  |
| [`KERN_OVERFLOWUID`](#kern-overflowuid) | const |  |
| [`KERN_OVERFLOWGID`](#kern-overflowgid) | const |  |
| [`KERN_SHMPATH`](#kern-shmpath) | const |  |
| [`KERN_HOTPLUG`](#kern-hotplug) | const |  |
| [`KERN_IEEE_EMULATION_WARNINGS`](#kern-ieee-emulation-warnings) | const |  |
| [`KERN_S390_USER_DEBUG_LOGGING`](#kern-s390-user-debug-logging) | const |  |
| [`KERN_CORE_USES_PID`](#kern-core-uses-pid) | const |  |
| [`KERN_TAINTED`](#kern-tainted) | const |  |
| [`KERN_CADPID`](#kern-cadpid) | const |  |
| [`KERN_PIDMAX`](#kern-pidmax) | const |  |
| [`KERN_CORE_PATTERN`](#kern-core-pattern) | const |  |
| [`KERN_PANIC_ON_OOPS`](#kern-panic-on-oops) | const |  |
| [`KERN_HPPA_PWRSW`](#kern-hppa-pwrsw) | const |  |
| [`KERN_HPPA_UNALIGNED`](#kern-hppa-unaligned) | const |  |
| [`KERN_PRINTK_RATELIMIT`](#kern-printk-ratelimit) | const |  |
| [`KERN_PRINTK_RATELIMIT_BURST`](#kern-printk-ratelimit-burst) | const |  |
| [`KERN_PTY`](#kern-pty) | const |  |
| [`KERN_NGROUPS_MAX`](#kern-ngroups-max) | const |  |
| [`KERN_SPARC_SCONS_PWROFF`](#kern-sparc-scons-pwroff) | const |  |
| [`KERN_HZ_TIMER`](#kern-hz-timer) | const |  |
| [`KERN_UNKNOWN_NMI_PANIC`](#kern-unknown-nmi-panic) | const |  |
| [`KERN_BOOTLOADER_TYPE`](#kern-bootloader-type) | const |  |
| [`KERN_RANDOMIZE`](#kern-randomize) | const |  |
| [`KERN_SETUID_DUMPABLE`](#kern-setuid-dumpable) | const |  |
| [`KERN_SPIN_RETRY`](#kern-spin-retry) | const |  |
| [`KERN_ACPI_VIDEO_FLAGS`](#kern-acpi-video-flags) | const |  |
| [`KERN_IA64_UNALIGNED`](#kern-ia64-unaligned) | const |  |
| [`KERN_COMPAT_LOG`](#kern-compat-log) | const |  |
| [`KERN_MAX_LOCK_DEPTH`](#kern-max-lock-depth) | const |  |
| [`KERN_NMI_WATCHDOG`](#kern-nmi-watchdog) | const |  |
| [`KERN_PANIC_ON_NMI`](#kern-panic-on-nmi) | const |  |
| [`VM_OVERCOMMIT_MEMORY`](#vm-overcommit-memory) | const |  |
| [`VM_PAGE_CLUSTER`](#vm-page-cluster) | const |  |
| [`VM_DIRTY_BACKGROUND`](#vm-dirty-background) | const |  |
| [`VM_DIRTY_RATIO`](#vm-dirty-ratio) | const |  |
| [`VM_DIRTY_WB_CS`](#vm-dirty-wb-cs) | const |  |
| [`VM_DIRTY_EXPIRE_CS`](#vm-dirty-expire-cs) | const |  |
| [`VM_NR_PDFLUSH_THREADS`](#vm-nr-pdflush-threads) | const |  |
| [`VM_OVERCOMMIT_RATIO`](#vm-overcommit-ratio) | const |  |
| [`VM_PAGEBUF`](#vm-pagebuf) | const |  |
| [`VM_HUGETLB_PAGES`](#vm-hugetlb-pages) | const |  |
| [`VM_SWAPPINESS`](#vm-swappiness) | const |  |
| [`VM_LOWMEM_RESERVE_RATIO`](#vm-lowmem-reserve-ratio) | const |  |
| [`VM_MIN_FREE_KBYTES`](#vm-min-free-kbytes) | const |  |
| [`VM_MAX_MAP_COUNT`](#vm-max-map-count) | const |  |
| [`VM_LAPTOP_MODE`](#vm-laptop-mode) | const |  |
| [`VM_BLOCK_DUMP`](#vm-block-dump) | const |  |
| [`VM_HUGETLB_GROUP`](#vm-hugetlb-group) | const |  |
| [`VM_VFS_CACHE_PRESSURE`](#vm-vfs-cache-pressure) | const |  |
| [`VM_LEGACY_VA_LAYOUT`](#vm-legacy-va-layout) | const |  |
| [`VM_SWAP_TOKEN_TIMEOUT`](#vm-swap-token-timeout) | const |  |
| [`VM_DROP_PAGECACHE`](#vm-drop-pagecache) | const |  |
| [`VM_PERCPU_PAGELIST_FRACTION`](#vm-percpu-pagelist-fraction) | const |  |
| [`VM_ZONE_RECLAIM_MODE`](#vm-zone-reclaim-mode) | const |  |
| [`VM_MIN_UNMAPPED`](#vm-min-unmapped) | const |  |
| [`VM_PANIC_ON_OOM`](#vm-panic-on-oom) | const |  |
| [`VM_VDSO_ENABLED`](#vm-vdso-enabled) | const |  |
| [`VM_MIN_SLAB`](#vm-min-slab) | const |  |
| [`NET_CORE`](#net-core) | const |  |
| [`NET_ETHER`](#net-ether) | const |  |
| [`NET_802`](#net-802) | const |  |
| [`NET_UNIX`](#net-unix) | const |  |
| [`NET_IPV4`](#net-ipv4) | const |  |
| [`NET_IPX`](#net-ipx) | const |  |
| [`NET_ATALK`](#net-atalk) | const |  |
| [`NET_NETROM`](#net-netrom) | const |  |
| [`NET_AX25`](#net-ax25) | const |  |
| [`NET_BRIDGE`](#net-bridge) | const |  |
| [`NET_ROSE`](#net-rose) | const |  |
| [`NET_IPV6`](#net-ipv6) | const |  |
| [`NET_X25`](#net-x25) | const |  |
| [`NET_TR`](#net-tr) | const |  |
| [`NET_DECNET`](#net-decnet) | const |  |
| [`NET_ECONET`](#net-econet) | const |  |
| [`NET_SCTP`](#net-sctp) | const |  |
| [`NET_LLC`](#net-llc) | const |  |
| [`NET_NETFILTER`](#net-netfilter) | const |  |
| [`NET_DCCP`](#net-dccp) | const |  |
| [`NET_IRDA`](#net-irda) | const |  |
| [`PF_VCPU`](#pf-vcpu) | const | I'm a virtual CPU. |
| [`PF_IDLE`](#pf-idle) | const | I am an IDLE thread. |
| [`PF_EXITING`](#pf-exiting) | const | Getting shut down. |
| [`PF_POSTCOREDUMP`](#pf-postcoredump) | const | Coredumps should ignore this task. |
| [`PF_IO_WORKER`](#pf-io-worker) | const | Task is an IO worker. |
| [`PF_WQ_WORKER`](#pf-wq-worker) | const | I'm a workqueue worker. |
| [`PF_FORKNOEXEC`](#pf-forknoexec) | const | Forked but didn't exec. |
| [`PF_MCE_PROCESS`](#pf-mce-process) | const | Process policy on mce errors. |
| [`PF_SUPERPRIV`](#pf-superpriv) | const | Used super-user privileges. |
| [`PF_DUMPCORE`](#pf-dumpcore) | const | Dumped core. |
| [`PF_SIGNALED`](#pf-signaled) | const | Killed by a signal. |
| [`PF_MEMALLOC`](#pf-memalloc) | const | Allocating memory to free memory. |
| [`PF_NPROC_EXCEEDED`](#pf-nproc-exceeded) | const | `set_user()` noticed that `RLIMIT_NPROC` was exceeded. |
| [`PF_USED_MATH`](#pf-used-math) | const | If unset the fpu must be initialized before use. |
| [`PF_USER_WORKER`](#pf-user-worker) | const | Kernel thread cloned from userspace thread. |
| [`PF_NOFREEZE`](#pf-nofreeze) | const | This thread should not be frozen. |
| [`PF_KSWAPD`](#pf-kswapd) | const | I am `kswapd`. |
| [`PF_MEMALLOC_NOFS`](#pf-memalloc-nofs) | const | All allocations inherit `GFP_NOFS`. |
| [`PF_MEMALLOC_NOIO`](#pf-memalloc-noio) | const | All allocations inherit `GFP_NOIO`. |
| [`PF_LOCAL_THROTTLE`](#pf-local-throttle) | const | Throttle writes only against the bdi I write to, I am cleaning dirty pages from some other bdi. |
| [`PF_KTHREAD`](#pf-kthread) | const | I am a kernel thread. |
| [`PF_RANDOMIZE`](#pf-randomize) | const | Randomize virtual address space. |
| [`PF_NO_SETAFFINITY`](#pf-no-setaffinity) | const | Userland is not allowed to meddle with `cpus_mask`. |
| [`PF_MCE_EARLY`](#pf-mce-early) | const | Early kill for mce process policy. |
| [`PF_MEMALLOC_PIN`](#pf-memalloc-pin) | const | Allocations constrained to zones which allow long term pinning. |
| [`PF_BLOCK_TS`](#pf-block-ts) | const | Plug has ts that needs updating. |
| [`PF_SUSPEND_TASK`](#pf-suspend-task) | const | This thread called `freeze_processes()` and should not be frozen. |
| [`PF_SUSPEND_TASK_UINT`](#pf-suspend-task-uint) | const |  |
| [`CLONE_PIDFD`](#clone-pidfd) | const |  |
| [`SCHED_FLAG_RESET_ON_FORK`](#sched-flag-reset-on-fork) | const |  |
| [`SCHED_FLAG_RECLAIM`](#sched-flag-reclaim) | const |  |
| [`SCHED_FLAG_DL_OVERRUN`](#sched-flag-dl-overrun) | const |  |
| [`SCHED_FLAG_KEEP_POLICY`](#sched-flag-keep-policy) | const |  |
| [`SCHED_FLAG_KEEP_PARAMS`](#sched-flag-keep-params) | const |  |
| [`SCHED_FLAG_UTIL_CLAMP_MIN`](#sched-flag-util-clamp-min) | const |  |
| [`SCHED_FLAG_UTIL_CLAMP_MAX`](#sched-flag-util-clamp-max) | const |  |
| [`XDP_SHARED_UMEM`](#xdp-shared-umem) | const |  |
| [`XDP_COPY`](#xdp-copy) | const |  |
| [`XDP_ZEROCOPY`](#xdp-zerocopy) | const |  |
| [`XDP_USE_NEED_WAKEUP`](#xdp-use-need-wakeup) | const |  |
| [`XDP_USE_SG`](#xdp-use-sg) | const |  |
| [`XDP_UMEM_UNALIGNED_CHUNK_FLAG`](#xdp-umem-unaligned-chunk-flag) | const |  |
| [`XDP_RING_NEED_WAKEUP`](#xdp-ring-need-wakeup) | const |  |
| [`XDP_MMAP_OFFSETS`](#xdp-mmap-offsets) | const |  |
| [`XDP_RX_RING`](#xdp-rx-ring) | const |  |
| [`XDP_TX_RING`](#xdp-tx-ring) | const |  |
| [`XDP_UMEM_REG`](#xdp-umem-reg) | const |  |
| [`XDP_UMEM_FILL_RING`](#xdp-umem-fill-ring) | const |  |
| [`XDP_UMEM_COMPLETION_RING`](#xdp-umem-completion-ring) | const |  |
| [`XDP_STATISTICS`](#xdp-statistics) | const |  |
| [`XDP_OPTIONS`](#xdp-options) | const |  |
| [`XDP_OPTIONS_ZEROCOPY`](#xdp-options-zerocopy) | const |  |
| [`XDP_PGOFF_RX_RING`](#xdp-pgoff-rx-ring) | const |  |
| [`XDP_PGOFF_TX_RING`](#xdp-pgoff-tx-ring) | const |  |
| [`XDP_UMEM_PGOFF_FILL_RING`](#xdp-umem-pgoff-fill-ring) | const |  |
| [`XDP_UMEM_PGOFF_COMPLETION_RING`](#xdp-umem-pgoff-completion-ring) | const |  |
| [`XSK_UNALIGNED_BUF_OFFSET_SHIFT`](#xsk-unaligned-buf-offset-shift) | const |  |
| [`XSK_UNALIGNED_BUF_ADDR_MASK`](#xsk-unaligned-buf-addr-mask) | const |  |
| [`XDP_PKT_CONTD`](#xdp-pkt-contd) | const |  |
| [`XDP_UMEM_TX_SW_CSUM`](#xdp-umem-tx-sw-csum) | const |  |
| [`XDP_UMEM_TX_METADATA_LEN`](#xdp-umem-tx-metadata-len) | const |  |
| [`XDP_TXMD_FLAGS_TIMESTAMP`](#xdp-txmd-flags-timestamp) | const |  |
| [`XDP_TXMD_FLAGS_CHECKSUM`](#xdp-txmd-flags-checksum) | const |  |
| [`XDP_TX_METADATA`](#xdp-tx-metadata) | const |  |
| [`SOL_XDP`](#sol-xdp) | const |  |
| [`MOUNT_ATTR_RDONLY`](#mount-attr-rdonly) | const |  |
| [`MOUNT_ATTR_NOSUID`](#mount-attr-nosuid) | const |  |
| [`MOUNT_ATTR_NODEV`](#mount-attr-nodev) | const |  |
| [`MOUNT_ATTR_NOEXEC`](#mount-attr-noexec) | const |  |
| [`MOUNT_ATTR__ATIME`](#mount-attr-atime) | const |  |
| [`MOUNT_ATTR_RELATIME`](#mount-attr-relatime) | const |  |
| [`MOUNT_ATTR_NOATIME`](#mount-attr-noatime) | const |  |
| [`MOUNT_ATTR_STRICTATIME`](#mount-attr-strictatime) | const |  |
| [`MOUNT_ATTR_NODIRATIME`](#mount-attr-nodiratime) | const |  |
| [`MOUNT_ATTR_IDMAP`](#mount-attr-idmap) | const |  |
| [`MOUNT_ATTR_NOSYMFOLLOW`](#mount-attr-nosymfollow) | const |  |
| [`MOUNT_ATTR_SIZE_VER0`](#mount-attr-size-ver0) | const |  |
| [`SCHED_FLAG_KEEP_ALL`](#sched-flag-keep-all) | const |  |
| [`SCHED_FLAG_UTIL_CLAMP`](#sched-flag-util-clamp) | const |  |
| [`SCHED_FLAG_ALL`](#sched-flag-all) | const |  |
| [`EPIOCSPARAMS`](#epiocsparams) | const |  |
| [`EPIOCGPARAMS`](#epiocgparams) | const |  |
| [`SI_DETHREAD`](#si-dethread) | const |  |
| [`TRAP_PERF`](#trap-perf) | const |  |
| [`DATE_BASE`](#date-base) | const |  |
| [`ABDAY_1`](#abday-1) | const |  |
| [`ABDAY_2`](#abday-2) | const |  |
| [`ABDAY_3`](#abday-3) | const |  |
| [`ABDAY_4`](#abday-4) | const |  |
| [`ABDAY_5`](#abday-5) | const |  |
| [`ABDAY_6`](#abday-6) | const |  |
| [`ABDAY_7`](#abday-7) | const |  |
| [`DAY_1`](#day-1) | const |  |
| [`DAY_2`](#day-2) | const |  |
| [`DAY_3`](#day-3) | const |  |
| [`DAY_4`](#day-4) | const |  |
| [`DAY_5`](#day-5) | const |  |
| [`DAY_6`](#day-6) | const |  |
| [`DAY_7`](#day-7) | const |  |
| [`ABMON_1`](#abmon-1) | const |  |
| [`ABMON_2`](#abmon-2) | const |  |
| [`ABMON_3`](#abmon-3) | const |  |
| [`ABMON_4`](#abmon-4) | const |  |
| [`ABMON_5`](#abmon-5) | const |  |
| [`ABMON_6`](#abmon-6) | const |  |
| [`ABMON_7`](#abmon-7) | const |  |
| [`ABMON_8`](#abmon-8) | const |  |
| [`ABMON_9`](#abmon-9) | const |  |
| [`ABMON_10`](#abmon-10) | const |  |
| [`ABMON_11`](#abmon-11) | const |  |
| [`ABMON_12`](#abmon-12) | const |  |
| [`MON_1`](#mon-1) | const |  |
| [`MON_2`](#mon-2) | const |  |
| [`MON_3`](#mon-3) | const |  |
| [`MON_4`](#mon-4) | const |  |
| [`MON_5`](#mon-5) | const |  |
| [`MON_6`](#mon-6) | const |  |
| [`MON_7`](#mon-7) | const |  |
| [`MON_8`](#mon-8) | const |  |
| [`MON_9`](#mon-9) | const |  |
| [`MON_10`](#mon-10) | const |  |
| [`MON_11`](#mon-11) | const |  |
| [`MON_12`](#mon-12) | const |  |
| [`AM_STR`](#am-str) | const |  |
| [`PM_STR`](#pm-str) | const |  |
| [`D_T_FMT`](#d-t-fmt) | const |  |
| [`D_FMT`](#d-fmt) | const |  |
| [`T_FMT`](#t-fmt) | const |  |
| [`T_FMT_AMPM`](#t-fmt-ampm) | const |  |
| [`ERA`](#era) | const |  |
| [`ERA_D_FMT`](#era-d-fmt) | const |  |
| [`ALT_DIGITS`](#alt-digits) | const |  |
| [`ERA_D_T_FMT`](#era-d-t-fmt) | const |  |
| [`ERA_T_FMT`](#era-t-fmt) | const |  |
| [`CODESET`](#codeset) | const |  |
| [`CRNCYSTR`](#crncystr) | const |  |
| [`RADIXCHAR`](#radixchar) | const |  |
| [`THOUSEP`](#thousep) | const |  |
| [`YESEXPR`](#yesexpr) | const |  |
| [`NOEXPR`](#noexpr) | const |  |
| [`YESSTR`](#yesstr) | const |  |
| [`NOSTR`](#nostr) | const |  |
| [`RUSAGE_CHILDREN`](#rusage-children) | const |  |
| [`L_tmpnam`](#l-tmpnam) | const |  |
| [`_PC_LINK_MAX`](#pc-link-max) | const |  |
| [`_PC_MAX_CANON`](#pc-max-canon) | const |  |
| [`_PC_MAX_INPUT`](#pc-max-input) | const |  |
| [`_PC_NAME_MAX`](#pc-name-max) | const |  |
| [`_PC_PATH_MAX`](#pc-path-max) | const |  |
| [`_PC_PIPE_BUF`](#pc-pipe-buf) | const |  |
| [`_PC_CHOWN_RESTRICTED`](#pc-chown-restricted) | const |  |
| [`_PC_NO_TRUNC`](#pc-no-trunc) | const |  |
| [`_PC_VDISABLE`](#pc-vdisable) | const |  |
| [`_PC_SYNC_IO`](#pc-sync-io) | const |  |
| [`_PC_ASYNC_IO`](#pc-async-io) | const |  |
| [`_PC_PRIO_IO`](#pc-prio-io) | const |  |
| [`_PC_SOCK_MAXBUF`](#pc-sock-maxbuf) | const |  |
| [`_PC_FILESIZEBITS`](#pc-filesizebits) | const |  |
| [`_PC_REC_INCR_XFER_SIZE`](#pc-rec-incr-xfer-size) | const |  |
| [`_PC_REC_MAX_XFER_SIZE`](#pc-rec-max-xfer-size) | const |  |
| [`_PC_REC_MIN_XFER_SIZE`](#pc-rec-min-xfer-size) | const |  |
| [`_PC_REC_XFER_ALIGN`](#pc-rec-xfer-align) | const |  |
| [`_PC_ALLOC_SIZE_MIN`](#pc-alloc-size-min) | const |  |
| [`_PC_SYMLINK_MAX`](#pc-symlink-max) | const |  |
| [`_PC_2_SYMLINKS`](#pc-2-symlinks) | const |  |
| [`_SC_ARG_MAX`](#sc-arg-max) | const |  |
| [`_SC_CHILD_MAX`](#sc-child-max) | const |  |
| [`_SC_CLK_TCK`](#sc-clk-tck) | const |  |
| [`_SC_NGROUPS_MAX`](#sc-ngroups-max) | const |  |
| [`_SC_OPEN_MAX`](#sc-open-max) | const |  |
| [`_SC_STREAM_MAX`](#sc-stream-max) | const |  |
| [`_SC_TZNAME_MAX`](#sc-tzname-max) | const |  |
| [`_SC_JOB_CONTROL`](#sc-job-control) | const |  |
| [`_SC_SAVED_IDS`](#sc-saved-ids) | const |  |
| [`_SC_REALTIME_SIGNALS`](#sc-realtime-signals) | const |  |
| [`_SC_PRIORITY_SCHEDULING`](#sc-priority-scheduling) | const |  |
| [`_SC_TIMERS`](#sc-timers) | const |  |
| [`_SC_ASYNCHRONOUS_IO`](#sc-asynchronous-io) | const |  |
| [`_SC_PRIORITIZED_IO`](#sc-prioritized-io) | const |  |
| [`_SC_SYNCHRONIZED_IO`](#sc-synchronized-io) | const |  |
| [`_SC_FSYNC`](#sc-fsync) | const |  |
| [`_SC_MAPPED_FILES`](#sc-mapped-files) | const |  |
| [`_SC_MEMLOCK`](#sc-memlock) | const |  |
| [`_SC_MEMLOCK_RANGE`](#sc-memlock-range) | const |  |
| [`_SC_MEMORY_PROTECTION`](#sc-memory-protection) | const |  |
| [`_SC_MESSAGE_PASSING`](#sc-message-passing) | const |  |
| [`_SC_SEMAPHORES`](#sc-semaphores) | const |  |
| [`_SC_SHARED_MEMORY_OBJECTS`](#sc-shared-memory-objects) | const |  |
| [`_SC_AIO_LISTIO_MAX`](#sc-aio-listio-max) | const |  |
| [`_SC_AIO_MAX`](#sc-aio-max) | const |  |
| [`_SC_AIO_PRIO_DELTA_MAX`](#sc-aio-prio-delta-max) | const |  |
| [`_SC_DELAYTIMER_MAX`](#sc-delaytimer-max) | const |  |
| [`_SC_MQ_OPEN_MAX`](#sc-mq-open-max) | const |  |
| [`_SC_MQ_PRIO_MAX`](#sc-mq-prio-max) | const |  |
| [`_SC_VERSION`](#sc-version) | const |  |
| [`_SC_PAGESIZE`](#sc-pagesize) | const |  |
| [`_SC_PAGE_SIZE`](#sc-page-size) | const |  |
| [`_SC_RTSIG_MAX`](#sc-rtsig-max) | const |  |
| [`_SC_SEM_NSEMS_MAX`](#sc-sem-nsems-max) | const |  |
| [`_SC_SEM_VALUE_MAX`](#sc-sem-value-max) | const |  |
| [`_SC_SIGQUEUE_MAX`](#sc-sigqueue-max) | const |  |
| [`_SC_TIMER_MAX`](#sc-timer-max) | const |  |
| [`_SC_BC_BASE_MAX`](#sc-bc-base-max) | const |  |
| [`_SC_BC_DIM_MAX`](#sc-bc-dim-max) | const |  |
| [`_SC_BC_SCALE_MAX`](#sc-bc-scale-max) | const |  |
| [`_SC_BC_STRING_MAX`](#sc-bc-string-max) | const |  |
| [`_SC_COLL_WEIGHTS_MAX`](#sc-coll-weights-max) | const |  |
| [`_SC_EXPR_NEST_MAX`](#sc-expr-nest-max) | const |  |
| [`_SC_LINE_MAX`](#sc-line-max) | const |  |
| [`_SC_RE_DUP_MAX`](#sc-re-dup-max) | const |  |
| [`_SC_2_VERSION`](#sc-2-version) | const |  |
| [`_SC_2_C_BIND`](#sc-2-c-bind) | const |  |
| [`_SC_2_C_DEV`](#sc-2-c-dev) | const |  |
| [`_SC_2_FORT_DEV`](#sc-2-fort-dev) | const |  |
| [`_SC_2_FORT_RUN`](#sc-2-fort-run) | const |  |
| [`_SC_2_SW_DEV`](#sc-2-sw-dev) | const |  |
| [`_SC_2_LOCALEDEF`](#sc-2-localedef) | const |  |
| [`_SC_UIO_MAXIOV`](#sc-uio-maxiov) | const |  |
| [`_SC_IOV_MAX`](#sc-iov-max) | const |  |
| [`_SC_THREADS`](#sc-threads) | const |  |
| [`_SC_THREAD_SAFE_FUNCTIONS`](#sc-thread-safe-functions) | const |  |
| [`_SC_GETGR_R_SIZE_MAX`](#sc-getgr-r-size-max) | const |  |
| [`_SC_GETPW_R_SIZE_MAX`](#sc-getpw-r-size-max) | const |  |
| [`_SC_LOGIN_NAME_MAX`](#sc-login-name-max) | const |  |
| [`_SC_TTY_NAME_MAX`](#sc-tty-name-max) | const |  |
| [`_SC_THREAD_DESTRUCTOR_ITERATIONS`](#sc-thread-destructor-iterations) | const |  |
| [`_SC_THREAD_KEYS_MAX`](#sc-thread-keys-max) | const |  |
| [`_SC_THREAD_STACK_MIN`](#sc-thread-stack-min) | const |  |
| [`_SC_THREAD_THREADS_MAX`](#sc-thread-threads-max) | const |  |
| [`_SC_THREAD_ATTR_STACKADDR`](#sc-thread-attr-stackaddr) | const |  |
| [`_SC_THREAD_ATTR_STACKSIZE`](#sc-thread-attr-stacksize) | const |  |
| [`_SC_THREAD_PRIORITY_SCHEDULING`](#sc-thread-priority-scheduling) | const |  |
| [`_SC_THREAD_PRIO_INHERIT`](#sc-thread-prio-inherit) | const |  |
| [`_SC_THREAD_PRIO_PROTECT`](#sc-thread-prio-protect) | const |  |
| [`_SC_THREAD_PROCESS_SHARED`](#sc-thread-process-shared) | const |  |
| [`_SC_NPROCESSORS_CONF`](#sc-nprocessors-conf) | const |  |
| [`_SC_NPROCESSORS_ONLN`](#sc-nprocessors-onln) | const |  |
| [`_SC_PHYS_PAGES`](#sc-phys-pages) | const |  |
| [`_SC_AVPHYS_PAGES`](#sc-avphys-pages) | const |  |
| [`_SC_ATEXIT_MAX`](#sc-atexit-max) | const |  |
| [`_SC_PASS_MAX`](#sc-pass-max) | const |  |
| [`_SC_XOPEN_VERSION`](#sc-xopen-version) | const |  |
| [`_SC_XOPEN_XCU_VERSION`](#sc-xopen-xcu-version) | const |  |
| [`_SC_XOPEN_UNIX`](#sc-xopen-unix) | const |  |
| [`_SC_XOPEN_CRYPT`](#sc-xopen-crypt) | const |  |
| [`_SC_XOPEN_ENH_I18N`](#sc-xopen-enh-i18n) | const |  |
| [`_SC_XOPEN_SHM`](#sc-xopen-shm) | const |  |
| [`_SC_2_CHAR_TERM`](#sc-2-char-term) | const |  |
| [`_SC_2_UPE`](#sc-2-upe) | const |  |
| [`_SC_XOPEN_XPG2`](#sc-xopen-xpg2) | const |  |
| [`_SC_XOPEN_XPG3`](#sc-xopen-xpg3) | const |  |
| [`_SC_XOPEN_XPG4`](#sc-xopen-xpg4) | const |  |
| [`_SC_NZERO`](#sc-nzero) | const |  |
| [`_SC_XBS5_ILP32_OFF32`](#sc-xbs5-ilp32-off32) | const |  |
| [`_SC_XBS5_ILP32_OFFBIG`](#sc-xbs5-ilp32-offbig) | const |  |
| [`_SC_XBS5_LP64_OFF64`](#sc-xbs5-lp64-off64) | const |  |
| [`_SC_XBS5_LPBIG_OFFBIG`](#sc-xbs5-lpbig-offbig) | const |  |
| [`_SC_XOPEN_LEGACY`](#sc-xopen-legacy) | const |  |
| [`_SC_XOPEN_REALTIME`](#sc-xopen-realtime) | const |  |
| [`_SC_XOPEN_REALTIME_THREADS`](#sc-xopen-realtime-threads) | const |  |
| [`_SC_ADVISORY_INFO`](#sc-advisory-info) | const |  |
| [`_SC_BARRIERS`](#sc-barriers) | const |  |
| [`_SC_CLOCK_SELECTION`](#sc-clock-selection) | const |  |
| [`_SC_CPUTIME`](#sc-cputime) | const |  |
| [`_SC_THREAD_CPUTIME`](#sc-thread-cputime) | const |  |
| [`_SC_MONOTONIC_CLOCK`](#sc-monotonic-clock) | const |  |
| [`_SC_READER_WRITER_LOCKS`](#sc-reader-writer-locks) | const |  |
| [`_SC_SPIN_LOCKS`](#sc-spin-locks) | const |  |
| [`_SC_REGEXP`](#sc-regexp) | const |  |
| [`_SC_SHELL`](#sc-shell) | const |  |
| [`_SC_SPAWN`](#sc-spawn) | const |  |
| [`_SC_SPORADIC_SERVER`](#sc-sporadic-server) | const |  |
| [`_SC_THREAD_SPORADIC_SERVER`](#sc-thread-sporadic-server) | const |  |
| [`_SC_TIMEOUTS`](#sc-timeouts) | const |  |
| [`_SC_TYPED_MEMORY_OBJECTS`](#sc-typed-memory-objects) | const |  |
| [`_SC_2_PBS`](#sc-2-pbs) | const |  |
| [`_SC_2_PBS_ACCOUNTING`](#sc-2-pbs-accounting) | const |  |
| [`_SC_2_PBS_LOCATE`](#sc-2-pbs-locate) | const |  |
| [`_SC_2_PBS_MESSAGE`](#sc-2-pbs-message) | const |  |
| [`_SC_2_PBS_TRACK`](#sc-2-pbs-track) | const |  |
| [`_SC_SYMLOOP_MAX`](#sc-symloop-max) | const |  |
| [`_SC_STREAMS`](#sc-streams) | const |  |
| [`_SC_2_PBS_CHECKPOINT`](#sc-2-pbs-checkpoint) | const |  |
| [`_SC_V6_ILP32_OFF32`](#sc-v6-ilp32-off32) | const |  |
| [`_SC_V6_ILP32_OFFBIG`](#sc-v6-ilp32-offbig) | const |  |
| [`_SC_V6_LP64_OFF64`](#sc-v6-lp64-off64) | const |  |
| [`_SC_V6_LPBIG_OFFBIG`](#sc-v6-lpbig-offbig) | const |  |
| [`_SC_HOST_NAME_MAX`](#sc-host-name-max) | const |  |
| [`_SC_TRACE`](#sc-trace) | const |  |
| [`_SC_TRACE_EVENT_FILTER`](#sc-trace-event-filter) | const |  |
| [`_SC_TRACE_INHERIT`](#sc-trace-inherit) | const |  |
| [`_SC_TRACE_LOG`](#sc-trace-log) | const |  |
| [`_SC_IPV6`](#sc-ipv6) | const |  |
| [`_SC_RAW_SOCKETS`](#sc-raw-sockets) | const |  |
| [`_SC_V7_ILP32_OFF32`](#sc-v7-ilp32-off32) | const |  |
| [`_SC_V7_ILP32_OFFBIG`](#sc-v7-ilp32-offbig) | const |  |
| [`_SC_V7_LP64_OFF64`](#sc-v7-lp64-off64) | const |  |
| [`_SC_V7_LPBIG_OFFBIG`](#sc-v7-lpbig-offbig) | const |  |
| [`_SC_SS_REPL_MAX`](#sc-ss-repl-max) | const |  |
| [`_SC_TRACE_EVENT_NAME_MAX`](#sc-trace-event-name-max) | const |  |
| [`_SC_TRACE_NAME_MAX`](#sc-trace-name-max) | const |  |
| [`_SC_TRACE_SYS_MAX`](#sc-trace-sys-max) | const |  |
| [`_SC_TRACE_USER_EVENT_MAX`](#sc-trace-user-event-max) | const |  |
| [`_SC_XOPEN_STREAMS`](#sc-xopen-streams) | const |  |
| [`_SC_THREAD_ROBUST_PRIO_INHERIT`](#sc-thread-robust-prio-inherit) | const |  |
| [`_SC_THREAD_ROBUST_PRIO_PROTECT`](#sc-thread-robust-prio-protect) | const |  |
| [`_CS_PATH`](#cs-path) | const |  |
| [`_CS_POSIX_V6_WIDTH_RESTRICTED_ENVS`](#cs-posix-v6-width-restricted-envs) | const |  |
| [`_CS_POSIX_V5_WIDTH_RESTRICTED_ENVS`](#cs-posix-v5-width-restricted-envs) | const |  |
| [`_CS_POSIX_V7_WIDTH_RESTRICTED_ENVS`](#cs-posix-v7-width-restricted-envs) | const |  |
| [`_CS_POSIX_V6_ILP32_OFF32_CFLAGS`](#cs-posix-v6-ilp32-off32-cflags) | const |  |
| [`_CS_POSIX_V6_ILP32_OFF32_LDFLAGS`](#cs-posix-v6-ilp32-off32-ldflags) | const |  |
| [`_CS_POSIX_V6_ILP32_OFF32_LIBS`](#cs-posix-v6-ilp32-off32-libs) | const |  |
| [`_CS_POSIX_V6_ILP32_OFF32_LINTFLAGS`](#cs-posix-v6-ilp32-off32-lintflags) | const |  |
| [`_CS_POSIX_V6_ILP32_OFFBIG_CFLAGS`](#cs-posix-v6-ilp32-offbig-cflags) | const |  |
| [`_CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS`](#cs-posix-v6-ilp32-offbig-ldflags) | const |  |
| [`_CS_POSIX_V6_ILP32_OFFBIG_LIBS`](#cs-posix-v6-ilp32-offbig-libs) | const |  |
| [`_CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS`](#cs-posix-v6-ilp32-offbig-lintflags) | const |  |
| [`_CS_POSIX_V6_LP64_OFF64_CFLAGS`](#cs-posix-v6-lp64-off64-cflags) | const |  |
| [`_CS_POSIX_V6_LP64_OFF64_LDFLAGS`](#cs-posix-v6-lp64-off64-ldflags) | const |  |
| [`_CS_POSIX_V6_LP64_OFF64_LIBS`](#cs-posix-v6-lp64-off64-libs) | const |  |
| [`_CS_POSIX_V6_LP64_OFF64_LINTFLAGS`](#cs-posix-v6-lp64-off64-lintflags) | const |  |
| [`_CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS`](#cs-posix-v6-lpbig-offbig-cflags) | const |  |
| [`_CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS`](#cs-posix-v6-lpbig-offbig-ldflags) | const |  |
| [`_CS_POSIX_V6_LPBIG_OFFBIG_LIBS`](#cs-posix-v6-lpbig-offbig-libs) | const |  |
| [`_CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS`](#cs-posix-v6-lpbig-offbig-lintflags) | const |  |
| [`_CS_POSIX_V7_ILP32_OFF32_CFLAGS`](#cs-posix-v7-ilp32-off32-cflags) | const |  |
| [`_CS_POSIX_V7_ILP32_OFF32_LDFLAGS`](#cs-posix-v7-ilp32-off32-ldflags) | const |  |
| [`_CS_POSIX_V7_ILP32_OFF32_LIBS`](#cs-posix-v7-ilp32-off32-libs) | const |  |
| [`_CS_POSIX_V7_ILP32_OFF32_LINTFLAGS`](#cs-posix-v7-ilp32-off32-lintflags) | const |  |
| [`_CS_POSIX_V7_ILP32_OFFBIG_CFLAGS`](#cs-posix-v7-ilp32-offbig-cflags) | const |  |
| [`_CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS`](#cs-posix-v7-ilp32-offbig-ldflags) | const |  |
| [`_CS_POSIX_V7_ILP32_OFFBIG_LIBS`](#cs-posix-v7-ilp32-offbig-libs) | const |  |
| [`_CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS`](#cs-posix-v7-ilp32-offbig-lintflags) | const |  |
| [`_CS_POSIX_V7_LP64_OFF64_CFLAGS`](#cs-posix-v7-lp64-off64-cflags) | const |  |
| [`_CS_POSIX_V7_LP64_OFF64_LDFLAGS`](#cs-posix-v7-lp64-off64-ldflags) | const |  |
| [`_CS_POSIX_V7_LP64_OFF64_LIBS`](#cs-posix-v7-lp64-off64-libs) | const |  |
| [`_CS_POSIX_V7_LP64_OFF64_LINTFLAGS`](#cs-posix-v7-lp64-off64-lintflags) | const |  |
| [`_CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS`](#cs-posix-v7-lpbig-offbig-cflags) | const |  |
| [`_CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS`](#cs-posix-v7-lpbig-offbig-ldflags) | const |  |
| [`_CS_POSIX_V7_LPBIG_OFFBIG_LIBS`](#cs-posix-v7-lpbig-offbig-libs) | const |  |
| [`_CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS`](#cs-posix-v7-lpbig-offbig-lintflags) | const |  |
| [`RLIM_SAVED_MAX`](#rlim-saved-max) | const |  |
| [`RLIM_SAVED_CUR`](#rlim-saved-cur) | const |  |
| [`EI_NIDENT`](#ei-nident) | const |  |
| [`EI_MAG0`](#ei-mag0) | const |  |
| [`ELFMAG0`](#elfmag0) | const |  |
| [`EI_MAG1`](#ei-mag1) | const |  |
| [`ELFMAG1`](#elfmag1) | const |  |
| [`EI_MAG2`](#ei-mag2) | const |  |
| [`ELFMAG2`](#elfmag2) | const |  |
| [`EI_MAG3`](#ei-mag3) | const |  |
| [`ELFMAG3`](#elfmag3) | const |  |
| [`SELFMAG`](#selfmag) | const |  |
| [`EI_CLASS`](#ei-class) | const |  |
| [`ELFCLASSNONE`](#elfclassnone) | const |  |
| [`ELFCLASS32`](#elfclass32) | const |  |
| [`ELFCLASS64`](#elfclass64) | const |  |
| [`ELFCLASSNUM`](#elfclassnum) | const |  |
| [`EI_DATA`](#ei-data) | const |  |
| [`ELFDATANONE`](#elfdatanone) | const |  |
| [`ELFDATA2LSB`](#elfdata2lsb) | const |  |
| [`ELFDATA2MSB`](#elfdata2msb) | const |  |
| [`ELFDATANUM`](#elfdatanum) | const |  |
| [`EI_VERSION`](#ei-version) | const |  |
| [`EI_OSABI`](#ei-osabi) | const |  |
| [`ELFOSABI_NONE`](#elfosabi-none) | const |  |
| [`ELFOSABI_SYSV`](#elfosabi-sysv) | const |  |
| [`ELFOSABI_HPUX`](#elfosabi-hpux) | const |  |
| [`ELFOSABI_NETBSD`](#elfosabi-netbsd) | const |  |
| [`ELFOSABI_GNU`](#elfosabi-gnu) | const |  |
| [`ELFOSABI_LINUX`](#elfosabi-linux) | const |  |
| [`ELFOSABI_SOLARIS`](#elfosabi-solaris) | const |  |
| [`ELFOSABI_AIX`](#elfosabi-aix) | const |  |
| [`ELFOSABI_IRIX`](#elfosabi-irix) | const |  |
| [`ELFOSABI_FREEBSD`](#elfosabi-freebsd) | const |  |
| [`ELFOSABI_TRU64`](#elfosabi-tru64) | const |  |
| [`ELFOSABI_MODESTO`](#elfosabi-modesto) | const |  |
| [`ELFOSABI_OPENBSD`](#elfosabi-openbsd) | const |  |
| [`ELFOSABI_ARM`](#elfosabi-arm) | const |  |
| [`ELFOSABI_STANDALONE`](#elfosabi-standalone) | const |  |
| [`EI_ABIVERSION`](#ei-abiversion) | const |  |
| [`EI_PAD`](#ei-pad) | const |  |
| [`ET_NONE`](#et-none) | const |  |
| [`ET_REL`](#et-rel) | const |  |
| [`ET_EXEC`](#et-exec) | const |  |
| [`ET_DYN`](#et-dyn) | const |  |
| [`ET_CORE`](#et-core) | const |  |
| [`ET_NUM`](#et-num) | const |  |
| [`ET_LOOS`](#et-loos) | const |  |
| [`ET_HIOS`](#et-hios) | const |  |
| [`ET_LOPROC`](#et-loproc) | const |  |
| [`ET_HIPROC`](#et-hiproc) | const |  |
| [`EM_NONE`](#em-none) | const |  |
| [`EM_M32`](#em-m32) | const |  |
| [`EM_SPARC`](#em-sparc) | const |  |
| [`EM_386`](#em-386) | const |  |
| [`EM_68K`](#em-68k) | const |  |
| [`EM_88K`](#em-88k) | const |  |
| [`EM_860`](#em-860) | const |  |
| [`EM_MIPS`](#em-mips) | const |  |
| [`EM_S370`](#em-s370) | const |  |
| [`EM_MIPS_RS3_LE`](#em-mips-rs3-le) | const |  |
| [`EM_PARISC`](#em-parisc) | const |  |
| [`EM_VPP500`](#em-vpp500) | const |  |
| [`EM_SPARC32PLUS`](#em-sparc32plus) | const |  |
| [`EM_960`](#em-960) | const |  |
| [`EM_PPC`](#em-ppc) | const |  |
| [`EM_PPC64`](#em-ppc64) | const |  |
| [`EM_S390`](#em-s390) | const |  |
| [`EM_V800`](#em-v800) | const |  |
| [`EM_FR20`](#em-fr20) | const |  |
| [`EM_RH32`](#em-rh32) | const |  |
| [`EM_RCE`](#em-rce) | const |  |
| [`EM_ARM`](#em-arm) | const |  |
| [`EM_FAKE_ALPHA`](#em-fake-alpha) | const |  |
| [`EM_SH`](#em-sh) | const |  |
| [`EM_SPARCV9`](#em-sparcv9) | const |  |
| [`EM_TRICORE`](#em-tricore) | const |  |
| [`EM_ARC`](#em-arc) | const |  |
| [`EM_H8_300`](#em-h8-300) | const |  |
| [`EM_H8_300H`](#em-h8-300h) | const |  |
| [`EM_H8S`](#em-h8s) | const |  |
| [`EM_H8_500`](#em-h8-500) | const |  |
| [`EM_IA_64`](#em-ia-64) | const |  |
| [`EM_MIPS_X`](#em-mips-x) | const |  |
| [`EM_COLDFIRE`](#em-coldfire) | const |  |
| [`EM_68HC12`](#em-68hc12) | const |  |
| [`EM_MMA`](#em-mma) | const |  |
| [`EM_PCP`](#em-pcp) | const |  |
| [`EM_NCPU`](#em-ncpu) | const |  |
| [`EM_NDR1`](#em-ndr1) | const |  |
| [`EM_STARCORE`](#em-starcore) | const |  |
| [`EM_ME16`](#em-me16) | const |  |
| [`EM_ST100`](#em-st100) | const |  |
| [`EM_TINYJ`](#em-tinyj) | const |  |
| [`EM_X86_64`](#em-x86-64) | const |  |
| [`EM_PDSP`](#em-pdsp) | const |  |
| [`EM_FX66`](#em-fx66) | const |  |
| [`EM_ST9PLUS`](#em-st9plus) | const |  |
| [`EM_ST7`](#em-st7) | const |  |
| [`EM_68HC16`](#em-68hc16) | const |  |
| [`EM_68HC11`](#em-68hc11) | const |  |
| [`EM_68HC08`](#em-68hc08) | const |  |
| [`EM_68HC05`](#em-68hc05) | const |  |
| [`EM_SVX`](#em-svx) | const |  |
| [`EM_ST19`](#em-st19) | const |  |
| [`EM_VAX`](#em-vax) | const |  |
| [`EM_CRIS`](#em-cris) | const |  |
| [`EM_JAVELIN`](#em-javelin) | const |  |
| [`EM_FIREPATH`](#em-firepath) | const |  |
| [`EM_ZSP`](#em-zsp) | const |  |
| [`EM_MMIX`](#em-mmix) | const |  |
| [`EM_HUANY`](#em-huany) | const |  |
| [`EM_PRISM`](#em-prism) | const |  |
| [`EM_AVR`](#em-avr) | const |  |
| [`EM_FR30`](#em-fr30) | const |  |
| [`EM_D10V`](#em-d10v) | const |  |
| [`EM_D30V`](#em-d30v) | const |  |
| [`EM_V850`](#em-v850) | const |  |
| [`EM_M32R`](#em-m32r) | const |  |
| [`EM_MN10300`](#em-mn10300) | const |  |
| [`EM_MN10200`](#em-mn10200) | const |  |
| [`EM_PJ`](#em-pj) | const |  |
| [`EM_OPENRISC`](#em-openrisc) | const |  |
| [`EM_ARC_A5`](#em-arc-a5) | const |  |
| [`EM_XTENSA`](#em-xtensa) | const |  |
| [`EM_AARCH64`](#em-aarch64) | const |  |
| [`EM_TILEPRO`](#em-tilepro) | const |  |
| [`EM_TILEGX`](#em-tilegx) | const |  |
| [`EM_RISCV`](#em-riscv) | const |  |
| [`EM_ALPHA`](#em-alpha) | const |  |
| [`EV_NONE`](#ev-none) | const |  |
| [`EV_CURRENT`](#ev-current) | const |  |
| [`EV_NUM`](#ev-num) | const |  |
| [`PT_NULL`](#pt-null) | const |  |
| [`PT_LOAD`](#pt-load) | const |  |
| [`PT_DYNAMIC`](#pt-dynamic) | const |  |
| [`PT_INTERP`](#pt-interp) | const |  |
| [`PT_NOTE`](#pt-note) | const |  |
| [`PT_SHLIB`](#pt-shlib) | const |  |
| [`PT_PHDR`](#pt-phdr) | const |  |
| [`PT_TLS`](#pt-tls) | const |  |
| [`PT_NUM`](#pt-num) | const |  |
| [`PT_LOOS`](#pt-loos) | const |  |
| [`PT_GNU_EH_FRAME`](#pt-gnu-eh-frame) | const |  |
| [`PT_GNU_STACK`](#pt-gnu-stack) | const |  |
| [`PT_GNU_RELRO`](#pt-gnu-relro) | const |  |
| [`PT_LOSUNW`](#pt-losunw) | const |  |
| [`PT_SUNWBSS`](#pt-sunwbss) | const |  |
| [`PT_SUNWSTACK`](#pt-sunwstack) | const |  |
| [`PT_HISUNW`](#pt-hisunw) | const |  |
| [`PT_HIOS`](#pt-hios) | const |  |
| [`PT_LOPROC`](#pt-loproc) | const |  |
| [`PT_HIPROC`](#pt-hiproc) | const |  |
| [`PF_X`](#pf-x) | const |  |
| [`PF_W`](#pf-w) | const |  |
| [`PF_R`](#pf-r) | const |  |
| [`PF_MASKOS`](#pf-maskos) | const |  |
| [`PF_MASKPROC`](#pf-maskproc) | const |  |
| [`AT_NULL`](#at-null) | const |  |
| [`AT_IGNORE`](#at-ignore) | const |  |
| [`AT_EXECFD`](#at-execfd) | const |  |
| [`AT_PHDR`](#at-phdr) | const |  |
| [`AT_PHENT`](#at-phent) | const |  |
| [`AT_PHNUM`](#at-phnum) | const |  |
| [`AT_PAGESZ`](#at-pagesz) | const |  |
| [`AT_BASE`](#at-base) | const |  |
| [`AT_FLAGS`](#at-flags) | const |  |
| [`AT_ENTRY`](#at-entry) | const |  |
| [`AT_NOTELF`](#at-notelf) | const |  |
| [`AT_UID`](#at-uid) | const |  |
| [`AT_EUID`](#at-euid) | const |  |
| [`AT_GID`](#at-gid) | const |  |
| [`AT_EGID`](#at-egid) | const |  |
| [`AT_PLATFORM`](#at-platform) | const |  |
| [`AT_HWCAP`](#at-hwcap) | const |  |
| [`AT_CLKTCK`](#at-clktck) | const |  |
| [`AT_SECURE`](#at-secure) | const |  |
| [`AT_BASE_PLATFORM`](#at-base-platform) | const |  |
| [`AT_RANDOM`](#at-random) | const |  |
| [`AT_HWCAP2`](#at-hwcap2) | const |  |
| [`AT_HWCAP3`](#at-hwcap3) | const |  |
| [`AT_HWCAP4`](#at-hwcap4) | const |  |
| [`AT_EXECFN`](#at-execfn) | const |  |
| [`AT_SYSINFO_EHDR`](#at-sysinfo-ehdr) | const |  |
| [`AT_MINSIGSTKSZ`](#at-minsigstksz) | const |  |
| [`GLOB_ERR`](#glob-err) | const |  |
| [`GLOB_MARK`](#glob-mark) | const |  |
| [`GLOB_NOSORT`](#glob-nosort) | const |  |
| [`GLOB_DOOFFS`](#glob-dooffs) | const |  |
| [`GLOB_NOCHECK`](#glob-nocheck) | const |  |
| [`GLOB_APPEND`](#glob-append) | const |  |
| [`GLOB_NOESCAPE`](#glob-noescape) | const |  |
| [`GLOB_NOSPACE`](#glob-nospace) | const |  |
| [`GLOB_ABORTED`](#glob-aborted) | const |  |
| [`GLOB_NOMATCH`](#glob-nomatch) | const |  |
| [`POSIX_MADV_NORMAL`](#posix-madv-normal) | const |  |
| [`POSIX_MADV_RANDOM`](#posix-madv-random) | const |  |
| [`POSIX_MADV_SEQUENTIAL`](#posix-madv-sequential) | const |  |
| [`POSIX_MADV_WILLNEED`](#posix-madv-willneed) | const |  |
| [`POSIX_MADV_DONTNEED`](#posix-madv-dontneed) | const |  |
| [`S_IEXEC`](#s-iexec) | const |  |
| [`S_IWRITE`](#s-iwrite) | const |  |
| [`S_IREAD`](#s-iread) | const |  |
| [`F_LOCK`](#f-lock) | const |  |
| [`F_TEST`](#f-test) | const |  |
| [`F_TLOCK`](#f-tlock) | const |  |
| [`F_ULOCK`](#f-ulock) | const |  |
| [`ST_RDONLY`](#st-rdonly) | const |  |
| [`ST_NOSUID`](#st-nosuid) | const |  |
| [`ST_NODEV`](#st-nodev) | const |  |
| [`ST_NOEXEC`](#st-noexec) | const |  |
| [`ST_SYNCHRONOUS`](#st-synchronous) | const |  |
| [`ST_MANDLOCK`](#st-mandlock) | const |  |
| [`ST_WRITE`](#st-write) | const |  |
| [`ST_APPEND`](#st-append) | const |  |
| [`ST_IMMUTABLE`](#st-immutable) | const |  |
| [`ST_NOATIME`](#st-noatime) | const |  |
| [`ST_NODIRATIME`](#st-nodiratime) | const |  |
| [`RTLD_NEXT`](#rtld-next) | const |  |
| [`RTLD_DEFAULT`](#rtld-default) | const |  |
| [`RTLD_NODELETE`](#rtld-nodelete) | const |  |
| [`RTLD_NOW`](#rtld-now) | const |  |
| [`AT_EACCESS`](#at-eaccess) | const |  |
| [`PTHREAD_BARRIER_SERIAL_THREAD`](#pthread-barrier-serial-thread) | const |  |
| [`PTHREAD_ONCE_INIT`](#pthread-once-init) | const |  |
| [`PTHREAD_MUTEX_NORMAL`](#pthread-mutex-normal) | const |  |
| [`PTHREAD_MUTEX_RECURSIVE`](#pthread-mutex-recursive) | const |  |
| [`PTHREAD_MUTEX_ERRORCHECK`](#pthread-mutex-errorcheck) | const |  |
| [`PTHREAD_MUTEX_DEFAULT`](#pthread-mutex-default) | const |  |
| [`PTHREAD_MUTEX_STALLED`](#pthread-mutex-stalled) | const |  |
| [`PTHREAD_MUTEX_ROBUST`](#pthread-mutex-robust) | const |  |
| [`PTHREAD_PRIO_NONE`](#pthread-prio-none) | const |  |
| [`PTHREAD_PRIO_INHERIT`](#pthread-prio-inherit) | const |  |
| [`PTHREAD_PRIO_PROTECT`](#pthread-prio-protect) | const |  |
| [`PTHREAD_PROCESS_PRIVATE`](#pthread-process-private) | const |  |
| [`PTHREAD_PROCESS_SHARED`](#pthread-process-shared) | const |  |
| [`PTHREAD_INHERIT_SCHED`](#pthread-inherit-sched) | const |  |
| [`PTHREAD_EXPLICIT_SCHED`](#pthread-explicit-sched) | const |  |
| [`__SIZEOF_PTHREAD_COND_T`](#sizeof-pthread-cond-t) | const |  |
| [`IPPROTO_MAX`](#ipproto-max) | const |  |
| [`IPC_PRIVATE`](#ipc-private) | const |  |
| [`IPC_CREAT`](#ipc-creat) | const |  |
| [`IPC_EXCL`](#ipc-excl) | const |  |
| [`IPC_NOWAIT`](#ipc-nowait) | const |  |
| [`IPC_RMID`](#ipc-rmid) | const |  |
| [`IPC_SET`](#ipc-set) | const |  |
| [`IPC_STAT`](#ipc-stat) | const |  |
| [`IPC_INFO`](#ipc-info) | const |  |
| [`SHM_R`](#shm-r) | const |  |
| [`SHM_W`](#shm-w) | const |  |
| [`SHM_RDONLY`](#shm-rdonly) | const |  |
| [`SHM_RND`](#shm-rnd) | const |  |
| [`SHM_REMAP`](#shm-remap) | const |  |
| [`SHM_LOCK`](#shm-lock) | const |  |
| [`SHM_UNLOCK`](#shm-unlock) | const |  |
| [`SHM_HUGETLB`](#shm-hugetlb) | const |  |
| [`SHM_NORESERVE`](#shm-noreserve) | const |  |
| [`LOG_NFACILITIES`](#log-nfacilities) | const |  |
| [`SEM_FAILED`](#sem-failed) | const |  |
| [`AI_PASSIVE`](#ai-passive) | const |  |
| [`AI_CANONNAME`](#ai-canonname) | const |  |
| [`AI_NUMERICHOST`](#ai-numerichost) | const |  |
| [`AI_V4MAPPED`](#ai-v4mapped) | const |  |
| [`AI_ALL`](#ai-all) | const |  |
| [`AI_ADDRCONFIG`](#ai-addrconfig) | const |  |
| [`AI_NUMERICSERV`](#ai-numericserv) | const |  |
| [`EAI_BADFLAGS`](#eai-badflags) | const |  |
| [`EAI_NONAME`](#eai-noname) | const |  |
| [`EAI_AGAIN`](#eai-again) | const |  |
| [`EAI_FAIL`](#eai-fail) | const |  |
| [`EAI_NODATA`](#eai-nodata) | const |  |
| [`EAI_FAMILY`](#eai-family) | const |  |
| [`EAI_SOCKTYPE`](#eai-socktype) | const |  |
| [`EAI_SERVICE`](#eai-service) | const |  |
| [`EAI_MEMORY`](#eai-memory) | const |  |
| [`EAI_SYSTEM`](#eai-system) | const |  |
| [`EAI_OVERFLOW`](#eai-overflow) | const |  |
| [`NI_NUMERICHOST`](#ni-numerichost) | const |  |
| [`NI_NUMERICSERV`](#ni-numericserv) | const |  |
| [`NI_NOFQDN`](#ni-nofqdn) | const |  |
| [`NI_NAMEREQD`](#ni-namereqd) | const |  |
| [`NI_DGRAM`](#ni-dgram) | const |  |
| [`NI_IDN`](#ni-idn) | const |  |
| [`AIO_CANCELED`](#aio-canceled) | const |  |
| [`AIO_NOTCANCELED`](#aio-notcanceled) | const |  |
| [`AIO_ALLDONE`](#aio-alldone) | const |  |
| [`LIO_READ`](#lio-read) | const |  |
| [`LIO_WRITE`](#lio-write) | const |  |
| [`LIO_NOP`](#lio-nop) | const |  |
| [`LIO_WAIT`](#lio-wait) | const |  |
| [`LIO_NOWAIT`](#lio-nowait) | const |  |
| [`RUSAGE_THREAD`](#rusage-thread) | const |  |
| [`MSG_COPY`](#msg-copy) | const |  |
| [`SHM_EXEC`](#shm-exec) | const |  |
| [`IPV6_MULTICAST_ALL`](#ipv6-multicast-all) | const |  |
| [`IPV6_ROUTER_ALERT_ISOLATE`](#ipv6-router-alert-isolate) | const |  |
| [`PACKET_MR_UNICAST`](#packet-mr-unicast) | const |  |
| [`PTRACE_EVENT_STOP`](#ptrace-event-stop) | const |  |
| [`UDP_SEGMENT`](#udp-segment) | const |  |
| [`UDP_GRO`](#udp-gro) | const |  |
| [`PR_SET_PDEATHSIG`](#pr-set-pdeathsig) | const |  |
| [`PR_GET_PDEATHSIG`](#pr-get-pdeathsig) | const |  |
| [`PR_GET_DUMPABLE`](#pr-get-dumpable) | const |  |
| [`PR_SET_DUMPABLE`](#pr-set-dumpable) | const |  |
| [`PR_GET_UNALIGN`](#pr-get-unalign) | const |  |
| [`PR_SET_UNALIGN`](#pr-set-unalign) | const |  |
| [`PR_UNALIGN_NOPRINT`](#pr-unalign-noprint) | const |  |
| [`PR_UNALIGN_SIGBUS`](#pr-unalign-sigbus) | const |  |
| [`PR_GET_KEEPCAPS`](#pr-get-keepcaps) | const |  |
| [`PR_SET_KEEPCAPS`](#pr-set-keepcaps) | const |  |
| [`PR_GET_FPEMU`](#pr-get-fpemu) | const |  |
| [`PR_SET_FPEMU`](#pr-set-fpemu) | const |  |
| [`PR_FPEMU_NOPRINT`](#pr-fpemu-noprint) | const |  |
| [`PR_FPEMU_SIGFPE`](#pr-fpemu-sigfpe) | const |  |
| [`PR_GET_FPEXC`](#pr-get-fpexc) | const |  |
| [`PR_SET_FPEXC`](#pr-set-fpexc) | const |  |
| [`PR_FP_EXC_SW_ENABLE`](#pr-fp-exc-sw-enable) | const |  |
| [`PR_FP_EXC_DIV`](#pr-fp-exc-div) | const |  |
| [`PR_FP_EXC_OVF`](#pr-fp-exc-ovf) | const |  |
| [`PR_FP_EXC_UND`](#pr-fp-exc-und) | const |  |
| [`PR_FP_EXC_RES`](#pr-fp-exc-res) | const |  |
| [`PR_FP_EXC_INV`](#pr-fp-exc-inv) | const |  |
| [`PR_FP_EXC_DISABLED`](#pr-fp-exc-disabled) | const |  |
| [`PR_FP_EXC_NONRECOV`](#pr-fp-exc-nonrecov) | const |  |
| [`PR_FP_EXC_ASYNC`](#pr-fp-exc-async) | const |  |
| [`PR_FP_EXC_PRECISE`](#pr-fp-exc-precise) | const |  |
| [`PR_GET_TIMING`](#pr-get-timing) | const |  |
| [`PR_SET_TIMING`](#pr-set-timing) | const |  |
| [`PR_TIMING_STATISTICAL`](#pr-timing-statistical) | const |  |
| [`PR_TIMING_TIMESTAMP`](#pr-timing-timestamp) | const |  |
| [`PR_SET_NAME`](#pr-set-name) | const |  |
| [`PR_GET_NAME`](#pr-get-name) | const |  |
| [`PR_GET_ENDIAN`](#pr-get-endian) | const |  |
| [`PR_SET_ENDIAN`](#pr-set-endian) | const |  |
| [`PR_ENDIAN_BIG`](#pr-endian-big) | const |  |
| [`PR_ENDIAN_LITTLE`](#pr-endian-little) | const |  |
| [`PR_ENDIAN_PPC_LITTLE`](#pr-endian-ppc-little) | const |  |
| [`PR_GET_SECCOMP`](#pr-get-seccomp) | const |  |
| [`PR_SET_SECCOMP`](#pr-set-seccomp) | const |  |
| [`PR_CAPBSET_READ`](#pr-capbset-read) | const |  |
| [`PR_CAPBSET_DROP`](#pr-capbset-drop) | const |  |
| [`PR_GET_TSC`](#pr-get-tsc) | const |  |
| [`PR_SET_TSC`](#pr-set-tsc) | const |  |
| [`PR_TSC_ENABLE`](#pr-tsc-enable) | const |  |
| [`PR_TSC_SIGSEGV`](#pr-tsc-sigsegv) | const |  |
| [`PR_GET_SECUREBITS`](#pr-get-securebits) | const |  |
| [`PR_SET_SECUREBITS`](#pr-set-securebits) | const |  |
| [`PR_SET_TIMERSLACK`](#pr-set-timerslack) | const |  |
| [`PR_GET_TIMERSLACK`](#pr-get-timerslack) | const |  |
| [`PR_TASK_PERF_EVENTS_DISABLE`](#pr-task-perf-events-disable) | const |  |
| [`PR_TASK_PERF_EVENTS_ENABLE`](#pr-task-perf-events-enable) | const |  |
| [`PR_MCE_KILL`](#pr-mce-kill) | const |  |
| [`PR_MCE_KILL_CLEAR`](#pr-mce-kill-clear) | const |  |
| [`PR_MCE_KILL_SET`](#pr-mce-kill-set) | const |  |
| [`PR_MCE_KILL_LATE`](#pr-mce-kill-late) | const |  |
| [`PR_MCE_KILL_EARLY`](#pr-mce-kill-early) | const |  |
| [`PR_MCE_KILL_DEFAULT`](#pr-mce-kill-default) | const |  |
| [`PR_MCE_KILL_GET`](#pr-mce-kill-get) | const |  |
| [`PR_SET_MM`](#pr-set-mm) | const |  |
| [`PR_SET_MM_START_CODE`](#pr-set-mm-start-code) | const |  |
| [`PR_SET_MM_END_CODE`](#pr-set-mm-end-code) | const |  |
| [`PR_SET_MM_START_DATA`](#pr-set-mm-start-data) | const |  |
| [`PR_SET_MM_END_DATA`](#pr-set-mm-end-data) | const |  |
| [`PR_SET_MM_START_STACK`](#pr-set-mm-start-stack) | const |  |
| [`PR_SET_MM_START_BRK`](#pr-set-mm-start-brk) | const |  |
| [`PR_SET_MM_BRK`](#pr-set-mm-brk) | const |  |
| [`PR_SET_MM_ARG_START`](#pr-set-mm-arg-start) | const |  |
| [`PR_SET_MM_ARG_END`](#pr-set-mm-arg-end) | const |  |
| [`PR_SET_MM_ENV_START`](#pr-set-mm-env-start) | const |  |
| [`PR_SET_MM_ENV_END`](#pr-set-mm-env-end) | const |  |
| [`PR_SET_MM_AUXV`](#pr-set-mm-auxv) | const |  |
| [`PR_SET_MM_EXE_FILE`](#pr-set-mm-exe-file) | const |  |
| [`PR_SET_MM_MAP`](#pr-set-mm-map) | const |  |
| [`PR_SET_MM_MAP_SIZE`](#pr-set-mm-map-size) | const |  |
| [`PR_SET_PTRACER`](#pr-set-ptracer) | const |  |
| [`PR_SET_PTRACER_ANY`](#pr-set-ptracer-any) | const |  |
| [`PR_SET_CHILD_SUBREAPER`](#pr-set-child-subreaper) | const |  |
| [`PR_GET_CHILD_SUBREAPER`](#pr-get-child-subreaper) | const |  |
| [`PR_SET_NO_NEW_PRIVS`](#pr-set-no-new-privs) | const |  |
| [`PR_GET_NO_NEW_PRIVS`](#pr-get-no-new-privs) | const |  |
| [`PR_GET_TID_ADDRESS`](#pr-get-tid-address) | const |  |
| [`PR_SET_THP_DISABLE`](#pr-set-thp-disable) | const |  |
| [`PR_GET_THP_DISABLE`](#pr-get-thp-disable) | const |  |
| [`PR_MPX_ENABLE_MANAGEMENT`](#pr-mpx-enable-management) | const |  |
| [`PR_MPX_DISABLE_MANAGEMENT`](#pr-mpx-disable-management) | const |  |
| [`PR_SET_FP_MODE`](#pr-set-fp-mode) | const |  |
| [`PR_GET_FP_MODE`](#pr-get-fp-mode) | const |  |
| [`PR_FP_MODE_FR`](#pr-fp-mode-fr) | const |  |
| [`PR_FP_MODE_FRE`](#pr-fp-mode-fre) | const |  |
| [`PR_CAP_AMBIENT`](#pr-cap-ambient) | const |  |
| [`PR_CAP_AMBIENT_IS_SET`](#pr-cap-ambient-is-set) | const |  |
| [`PR_CAP_AMBIENT_RAISE`](#pr-cap-ambient-raise) | const |  |
| [`PR_CAP_AMBIENT_LOWER`](#pr-cap-ambient-lower) | const |  |
| [`PR_CAP_AMBIENT_CLEAR_ALL`](#pr-cap-ambient-clear-all) | const |  |
| [`PR_SET_VMA`](#pr-set-vma) | const |  |
| [`PR_SET_VMA_ANON_NAME`](#pr-set-vma-anon-name) | const |  |
| [`PR_SCHED_CORE`](#pr-sched-core) | const |  |
| [`PR_SCHED_CORE_GET`](#pr-sched-core-get) | const |  |
| [`PR_SCHED_CORE_CREATE`](#pr-sched-core-create) | const |  |
| [`PR_SCHED_CORE_SHARE_TO`](#pr-sched-core-share-to) | const |  |
| [`PR_SCHED_CORE_SHARE_FROM`](#pr-sched-core-share-from) | const |  |
| [`PR_SCHED_CORE_MAX`](#pr-sched-core-max) | const |  |
| [`PR_SCHED_CORE_SCOPE_THREAD`](#pr-sched-core-scope-thread) | const |  |
| [`PR_SCHED_CORE_SCOPE_THREAD_GROUP`](#pr-sched-core-scope-thread-group) | const |  |
| [`PR_SCHED_CORE_SCOPE_PROCESS_GROUP`](#pr-sched-core-scope-process-group) | const |  |
| [`ITIMER_REAL`](#itimer-real) | const |  |
| [`ITIMER_VIRTUAL`](#itimer-virtual) | const |  |
| [`ITIMER_PROF`](#itimer-prof) | const |  |
| [`_POSIX_VDISABLE`](#posix-vdisable) | const |  |
| [`IPV6_RTHDR_LOOSE`](#ipv6-rthdr-loose) | const |  |
| [`IPV6_RTHDR_STRICT`](#ipv6-rthdr-strict) | const |  |
| [`IUTF8`](#iutf8) | const |  |
| [`CMSPAR`](#cmspar) | const |  |
| [`MFD_CLOEXEC`](#mfd-cloexec) | const |  |
| [`MFD_ALLOW_SEALING`](#mfd-allow-sealing) | const |  |
| [`MFD_HUGETLB`](#mfd-hugetlb) | const |  |
| [`MFD_NOEXEC_SEAL`](#mfd-noexec-seal) | const |  |
| [`MFD_EXEC`](#mfd-exec) | const |  |
| [`MFD_HUGE_64KB`](#mfd-huge-64kb) | const |  |
| [`MFD_HUGE_512KB`](#mfd-huge-512kb) | const |  |
| [`MFD_HUGE_1MB`](#mfd-huge-1mb) | const |  |
| [`MFD_HUGE_2MB`](#mfd-huge-2mb) | const |  |
| [`MFD_HUGE_8MB`](#mfd-huge-8mb) | const |  |
| [`MFD_HUGE_16MB`](#mfd-huge-16mb) | const |  |
| [`MFD_HUGE_32MB`](#mfd-huge-32mb) | const |  |
| [`MFD_HUGE_256MB`](#mfd-huge-256mb) | const |  |
| [`MFD_HUGE_512MB`](#mfd-huge-512mb) | const |  |
| [`MFD_HUGE_1GB`](#mfd-huge-1gb) | const |  |
| [`MFD_HUGE_2GB`](#mfd-huge-2gb) | const |  |
| [`MFD_HUGE_16GB`](#mfd-huge-16gb) | const |  |
| [`MFD_HUGE_MASK`](#mfd-huge-mask) | const |  |
| [`MFD_HUGE_SHIFT`](#mfd-huge-shift) | const |  |
| [`PACKET_HOST`](#packet-host) | const |  |
| [`PACKET_BROADCAST`](#packet-broadcast) | const |  |
| [`PACKET_MULTICAST`](#packet-multicast) | const |  |
| [`PACKET_OTHERHOST`](#packet-otherhost) | const |  |
| [`PACKET_OUTGOING`](#packet-outgoing) | const |  |
| [`PACKET_LOOPBACK`](#packet-loopback) | const |  |
| [`PACKET_USER`](#packet-user) | const |  |
| [`PACKET_KERNEL`](#packet-kernel) | const |  |
| [`PACKET_ADD_MEMBERSHIP`](#packet-add-membership) | const |  |
| [`PACKET_DROP_MEMBERSHIP`](#packet-drop-membership) | const |  |
| [`PACKET_RECV_OUTPUT`](#packet-recv-output) | const |  |
| [`PACKET_RX_RING`](#packet-rx-ring) | const |  |
| [`PACKET_STATISTICS`](#packet-statistics) | const |  |
| [`PACKET_COPY_THRESH`](#packet-copy-thresh) | const |  |
| [`PACKET_AUXDATA`](#packet-auxdata) | const |  |
| [`PACKET_ORIGDEV`](#packet-origdev) | const |  |
| [`PACKET_VERSION`](#packet-version) | const |  |
| [`PACKET_HDRLEN`](#packet-hdrlen) | const |  |
| [`PACKET_RESERVE`](#packet-reserve) | const |  |
| [`PACKET_TX_RING`](#packet-tx-ring) | const |  |
| [`PACKET_LOSS`](#packet-loss) | const |  |
| [`PACKET_VNET_HDR`](#packet-vnet-hdr) | const |  |
| [`PACKET_TX_TIMESTAMP`](#packet-tx-timestamp) | const |  |
| [`PACKET_TIMESTAMP`](#packet-timestamp) | const |  |
| [`PACKET_MR_MULTICAST`](#packet-mr-multicast) | const |  |
| [`PACKET_MR_PROMISC`](#packet-mr-promisc) | const |  |
| [`PACKET_MR_ALLMULTI`](#packet-mr-allmulti) | const |  |
| [`SIOCADDRT`](#siocaddrt) | const |  |
| [`SIOCDELRT`](#siocdelrt) | const |  |
| [`SIOCGIFNAME`](#siocgifname) | const |  |
| [`SIOCSIFLINK`](#siocsiflink) | const |  |
| [`SIOCGIFCONF`](#siocgifconf) | const |  |
| [`SIOCGIFFLAGS`](#siocgifflags) | const |  |
| [`SIOCSIFFLAGS`](#siocsifflags) | const |  |
| [`SIOCGIFADDR`](#siocgifaddr) | const |  |
| [`SIOCSIFADDR`](#siocsifaddr) | const |  |
| [`SIOCGIFDSTADDR`](#siocgifdstaddr) | const |  |
| [`SIOCSIFDSTADDR`](#siocsifdstaddr) | const |  |
| [`SIOCGIFBRDADDR`](#siocgifbrdaddr) | const |  |
| [`SIOCSIFBRDADDR`](#siocsifbrdaddr) | const |  |
| [`SIOCGIFNETMASK`](#siocgifnetmask) | const |  |
| [`SIOCSIFNETMASK`](#siocsifnetmask) | const |  |
| [`SIOCGIFMETRIC`](#siocgifmetric) | const |  |
| [`SIOCSIFMETRIC`](#siocsifmetric) | const |  |
| [`SIOCGIFMEM`](#siocgifmem) | const |  |
| [`SIOCSIFMEM`](#siocsifmem) | const |  |
| [`SIOCGIFMTU`](#siocgifmtu) | const |  |
| [`SIOCSIFMTU`](#siocsifmtu) | const |  |
| [`SIOCSIFNAME`](#siocsifname) | const |  |
| [`SIOCSIFHWADDR`](#siocsifhwaddr) | const |  |
| [`SIOCGIFENCAP`](#siocgifencap) | const |  |
| [`SIOCSIFENCAP`](#siocsifencap) | const |  |
| [`SIOCGIFHWADDR`](#siocgifhwaddr) | const |  |
| [`SIOCGIFSLAVE`](#siocgifslave) | const |  |
| [`SIOCSIFSLAVE`](#siocsifslave) | const |  |
| [`SIOCADDMULTI`](#siocaddmulti) | const |  |
| [`SIOCDELMULTI`](#siocdelmulti) | const |  |
| [`SIOCGIFINDEX`](#siocgifindex) | const |  |
| [`SIOGIFINDEX`](#siogifindex) | const |  |
| [`SIOCSIFPFLAGS`](#siocsifpflags) | const |  |
| [`SIOCGIFPFLAGS`](#siocgifpflags) | const |  |
| [`SIOCDIFADDR`](#siocdifaddr) | const |  |
| [`SIOCSIFHWBROADCAST`](#siocsifhwbroadcast) | const |  |
| [`SIOCGIFCOUNT`](#siocgifcount) | const |  |
| [`SIOCGIFBR`](#siocgifbr) | const |  |
| [`SIOCSIFBR`](#siocsifbr) | const |  |
| [`SIOCGIFTXQLEN`](#siocgiftxqlen) | const |  |
| [`SIOCSIFTXQLEN`](#siocsiftxqlen) | const |  |
| [`SIOCETHTOOL`](#siocethtool) | const |  |
| [`SIOCGMIIPHY`](#siocgmiiphy) | const |  |
| [`SIOCGMIIREG`](#siocgmiireg) | const |  |
| [`SIOCSMIIREG`](#siocsmiireg) | const |  |
| [`SIOCWANDEV`](#siocwandev) | const |  |
| [`SIOCOUTQNSD`](#siocoutqnsd) | const |  |
| [`SIOCGSKNS`](#siocgskns) | const |  |
| [`SIOCDARP`](#siocdarp) | const |  |
| [`SIOCGARP`](#siocgarp) | const |  |
| [`SIOCSARP`](#siocsarp) | const |  |
| [`SIOCDRARP`](#siocdrarp) | const |  |
| [`SIOCGRARP`](#siocgrarp) | const |  |
| [`SIOCSRARP`](#siocsrarp) | const |  |
| [`SIOCGIFMAP`](#siocgifmap) | const |  |
| [`SIOCSIFMAP`](#siocsifmap) | const |  |
| [`IPTOS_TOS_MASK`](#iptos-tos-mask) | const |  |
| [`IPTOS_PREC_MASK`](#iptos-prec-mask) | const |  |
| [`IPTOS_ECN_NOT_ECT`](#iptos-ecn-not-ect) | const |  |
| [`RTF_UP`](#rtf-up) | const |  |
| [`RTF_GATEWAY`](#rtf-gateway) | const |  |
| [`RTF_HOST`](#rtf-host) | const |  |
| [`RTF_REINSTATE`](#rtf-reinstate) | const |  |
| [`RTF_DYNAMIC`](#rtf-dynamic) | const |  |
| [`RTF_MODIFIED`](#rtf-modified) | const |  |
| [`RTF_MTU`](#rtf-mtu) | const |  |
| [`RTF_MSS`](#rtf-mss) | const |  |
| [`RTF_WINDOW`](#rtf-window) | const |  |
| [`RTF_IRTT`](#rtf-irtt) | const |  |
| [`RTF_REJECT`](#rtf-reject) | const |  |
| [`RTF_STATIC`](#rtf-static) | const |  |
| [`RTF_XRESOLVE`](#rtf-xresolve) | const |  |
| [`RTF_NOFORWARD`](#rtf-noforward) | const |  |
| [`RTF_THROW`](#rtf-throw) | const |  |
| [`RTF_NOPMTUDISC`](#rtf-nopmtudisc) | const |  |
| [`RTF_DEFAULT`](#rtf-default) | const |  |
| [`RTF_ALLONLINK`](#rtf-allonlink) | const |  |
| [`RTF_ADDRCONF`](#rtf-addrconf) | const |  |
| [`RTF_LINKRT`](#rtf-linkrt) | const |  |
| [`RTF_NONEXTHOP`](#rtf-nonexthop) | const |  |
| [`RTF_CACHE`](#rtf-cache) | const |  |
| [`RTF_FLOW`](#rtf-flow) | const |  |
| [`RTF_POLICY`](#rtf-policy) | const |  |
| [`RTCF_VALVE`](#rtcf-valve) | const |  |
| [`RTCF_MASQ`](#rtcf-masq) | const |  |
| [`RTCF_NAT`](#rtcf-nat) | const |  |
| [`RTCF_DOREDIRECT`](#rtcf-doredirect) | const |  |
| [`RTCF_LOG`](#rtcf-log) | const |  |
| [`RTCF_DIRECTSRC`](#rtcf-directsrc) | const |  |
| [`RTF_LOCAL`](#rtf-local) | const |  |
| [`RTF_INTERFACE`](#rtf-interface) | const |  |
| [`RTF_MULTICAST`](#rtf-multicast) | const |  |
| [`RTF_BROADCAST`](#rtf-broadcast) | const |  |
| [`RTF_NAT`](#rtf-nat) | const |  |
| [`RTF_ADDRCLASSMASK`](#rtf-addrclassmask) | const |  |
| [`RT_CLASS_UNSPEC`](#rt-class-unspec) | const |  |
| [`RT_CLASS_DEFAULT`](#rt-class-default) | const |  |
| [`RT_CLASS_MAIN`](#rt-class-main) | const |  |
| [`RT_CLASS_LOCAL`](#rt-class-local) | const |  |
| [`RT_CLASS_MAX`](#rt-class-max) | const |  |
| [`MAX_ADDR_LEN`](#max-addr-len) | const |  |
| [`ARPD_UPDATE`](#arpd-update) | const |  |
| [`ARPD_LOOKUP`](#arpd-lookup) | const |  |
| [`ARPD_FLUSH`](#arpd-flush) | const |  |
| [`ATF_MAGIC`](#atf-magic) | const |  |
| [`UDP_CORK`](#udp-cork) | const |  |
| [`UDP_ENCAP`](#udp-encap) | const |  |
| [`UDP_NO_CHECK6_TX`](#udp-no-check6-tx) | const |  |
| [`UDP_NO_CHECK6_RX`](#udp-no-check6-rx) | const |  |
| [`MAP_FIXED_NOREPLACE`](#map-fixed-noreplace) | const |  |
| [`MLOCK_ONFAULT`](#mlock-onfault) | const |  |
| [`REG_EXTENDED`](#reg-extended) | const |  |
| [`REG_ICASE`](#reg-icase) | const |  |
| [`REG_NEWLINE`](#reg-newline) | const |  |
| [`REG_NOSUB`](#reg-nosub) | const |  |
| [`REG_NOTBOL`](#reg-notbol) | const |  |
| [`REG_NOTEOL`](#reg-noteol) | const |  |
| [`REG_ENOSYS`](#reg-enosys) | const |  |
| [`REG_NOMATCH`](#reg-nomatch) | const |  |
| [`REG_BADPAT`](#reg-badpat) | const |  |
| [`REG_ECOLLATE`](#reg-ecollate) | const |  |
| [`REG_ECTYPE`](#reg-ectype) | const |  |
| [`REG_EESCAPE`](#reg-eescape) | const |  |
| [`REG_ESUBREG`](#reg-esubreg) | const |  |
| [`REG_EBRACK`](#reg-ebrack) | const |  |
| [`REG_EPAREN`](#reg-eparen) | const |  |
| [`REG_EBRACE`](#reg-ebrace) | const |  |
| [`REG_BADBR`](#reg-badbr) | const |  |
| [`REG_ERANGE`](#reg-erange) | const |  |
| [`REG_ESPACE`](#reg-espace) | const |  |
| [`REG_BADRPT`](#reg-badrpt) | const |  |
| [`EPERM`](#eperm) | const |  |
| [`ENOENT`](#enoent) | const |  |
| [`ESRCH`](#esrch) | const |  |
| [`EINTR`](#eintr) | const |  |
| [`EIO`](#eio) | const |  |
| [`ENXIO`](#enxio) | const |  |
| [`E2BIG`](#e2big) | const |  |
| [`ENOEXEC`](#enoexec) | const |  |
| [`EBADF`](#ebadf) | const |  |
| [`ECHILD`](#echild) | const |  |
| [`EAGAIN`](#eagain) | const |  |
| [`ENOMEM`](#enomem) | const |  |
| [`EACCES`](#eacces) | const |  |
| [`EFAULT`](#efault) | const |  |
| [`ENOTBLK`](#enotblk) | const |  |
| [`EBUSY`](#ebusy) | const |  |
| [`EEXIST`](#eexist) | const |  |
| [`EXDEV`](#exdev) | const |  |
| [`ENODEV`](#enodev) | const |  |
| [`ENOTDIR`](#enotdir) | const |  |
| [`EISDIR`](#eisdir) | const |  |
| [`EINVAL`](#einval) | const |  |
| [`ENFILE`](#enfile) | const |  |
| [`EMFILE`](#emfile) | const |  |
| [`ENOTTY`](#enotty) | const |  |
| [`ETXTBSY`](#etxtbsy) | const |  |
| [`EFBIG`](#efbig) | const |  |
| [`ENOSPC`](#enospc) | const |  |
| [`ESPIPE`](#espipe) | const |  |
| [`EROFS`](#erofs) | const |  |
| [`EMLINK`](#emlink) | const |  |
| [`EPIPE`](#epipe) | const |  |
| [`EDOM`](#edom) | const |  |
| [`ERANGE`](#erange) | const |  |
| [`EWOULDBLOCK`](#ewouldblock) | const |  |
| [`CSIGNAL`](#csignal) | const |  |
| [`SCHED_NORMAL`](#sched-normal) | const |  |
| [`SCHED_OTHER`](#sched-other) | const |  |
| [`SCHED_FIFO`](#sched-fifo) | const |  |
| [`SCHED_RR`](#sched-rr) | const |  |
| [`SCHED_BATCH`](#sched-batch) | const |  |
| [`SCHED_IDLE`](#sched-idle) | const |  |
| [`SCHED_DEADLINE`](#sched-deadline) | const |  |
| [`SCHED_RESET_ON_FORK`](#sched-reset-on-fork) | const |  |
| [`NT_PRSTATUS`](#nt-prstatus) | const |  |
| [`NT_PRFPREG`](#nt-prfpreg) | const |  |
| [`NT_FPREGSET`](#nt-fpregset) | const |  |
| [`NT_PRPSINFO`](#nt-prpsinfo) | const |  |
| [`NT_PRXREG`](#nt-prxreg) | const |  |
| [`NT_TASKSTRUCT`](#nt-taskstruct) | const |  |
| [`NT_PLATFORM`](#nt-platform) | const |  |
| [`NT_AUXV`](#nt-auxv) | const |  |
| [`NT_GWINDOWS`](#nt-gwindows) | const |  |
| [`NT_ASRS`](#nt-asrs) | const |  |
| [`NT_PSTATUS`](#nt-pstatus) | const |  |
| [`NT_PSINFO`](#nt-psinfo) | const |  |
| [`NT_PRCRED`](#nt-prcred) | const |  |
| [`NT_UTSNAME`](#nt-utsname) | const |  |
| [`NT_LWPSTATUS`](#nt-lwpstatus) | const |  |
| [`NT_LWPSINFO`](#nt-lwpsinfo) | const |  |
| [`NT_PRFPXREG`](#nt-prfpxreg) | const |  |
| [`MS_NOUSER`](#ms-nouser) | const |  |

## Modules

- [`linux`](linux/index.md) — Linux-specific definitions for linux-like values
- [`linux_l4re_shared`](linux_l4re_shared/index.md)
- [`gnu`](gnu/index.md)
- [`arch`](arch/index.md)

## Structs

### `in_addr`

```rust
struct in_addr {
    pub s_addr: crate::in_addr_t,
}
```

#### Trait Implementations

##### `impl Clone for in_addr`

- <span id="in-addr-clone"></span>`fn clone(&self) -> in_addr` — [`in_addr`](../index.md#in-addr)

##### `impl Copy for in_addr`

##### `impl Debug for in_addr`

- <span id="in-addr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ip_mreq`

```rust
struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
```

#### Trait Implementations

##### `impl Clone for ip_mreq`

- <span id="ip-mreq-clone"></span>`fn clone(&self) -> ip_mreq` — [`ip_mreq`](../index.md#ip-mreq)

##### `impl Copy for ip_mreq`

##### `impl Debug for ip_mreq`

- <span id="ip-mreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ip_mreqn`

```rust
struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: c_int,
}
```

#### Trait Implementations

##### `impl Clone for ip_mreqn`

- <span id="ip-mreqn-clone"></span>`fn clone(&self) -> ip_mreqn` — [`ip_mreqn`](../index.md#ip-mreqn)

##### `impl Copy for ip_mreqn`

##### `impl Debug for ip_mreqn`

- <span id="ip-mreqn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ip_mreq_source`

```rust
struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
```

#### Trait Implementations

##### `impl Clone for ip_mreq_source`

- <span id="ip-mreq-source-clone"></span>`fn clone(&self) -> ip_mreq_source` — [`ip_mreq_source`](../index.md#ip-mreq-source)

##### `impl Copy for ip_mreq_source`

##### `impl Debug for ip_mreq_source`

- <span id="ip-mreq-source-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr`

```rust
struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr`

- <span id="sockaddr-clone"></span>`fn clone(&self) -> sockaddr` — [`sockaddr`](../index.md#sockaddr)

##### `impl Copy for sockaddr`

##### `impl Debug for sockaddr`

- <span id="sockaddr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_in`

```rust
struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: crate::in_port_t,
    pub sin_addr: crate::in_addr,
    pub sin_zero: [u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_in`

- <span id="sockaddr-in-clone"></span>`fn clone(&self) -> sockaddr_in` — [`sockaddr_in`](../index.md#sockaddr-in)

##### `impl Copy for sockaddr_in`

##### `impl Debug for sockaddr_in`

- <span id="sockaddr-in-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_in6`

```rust
struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: crate::in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: crate::in6_addr,
    pub sin6_scope_id: u32,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_in6`

- <span id="sockaddr-in6-clone"></span>`fn clone(&self) -> sockaddr_in6` — [`sockaddr_in6`](../index.md#sockaddr-in6)

##### `impl Copy for sockaddr_in6`

##### `impl Debug for sockaddr_in6`

- <span id="sockaddr-in6-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `addrinfo`

```rust
struct addrinfo {
    pub ai_flags: c_int,
    pub ai_family: c_int,
    pub ai_socktype: c_int,
    pub ai_protocol: c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut crate::sockaddr,
    pub ai_canonname: *mut c_char,
    pub ai_next: *mut addrinfo,
}
```

#### Trait Implementations

##### `impl Clone for addrinfo`

- <span id="addrinfo-clone"></span>`fn clone(&self) -> addrinfo` — [`addrinfo`](../index.md#addrinfo)

##### `impl Copy for addrinfo`

##### `impl Debug for addrinfo`

- <span id="addrinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_ll`

```rust
struct sockaddr_ll {
    pub sll_family: c_ushort,
    pub sll_protocol: c_ushort,
    pub sll_ifindex: c_int,
    pub sll_hatype: c_ushort,
    pub sll_pkttype: c_uchar,
    pub sll_halen: c_uchar,
    pub sll_addr: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_ll`

- <span id="sockaddr-ll-clone"></span>`fn clone(&self) -> sockaddr_ll` — [`sockaddr_ll`](../index.md#sockaddr-ll)

##### `impl Copy for sockaddr_ll`

##### `impl Debug for sockaddr_ll`

- <span id="sockaddr-ll-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fd_set`

```rust
struct fd_set {
    fds_bits: [c_ulong; 16],
}
```

#### Trait Implementations

##### `impl Clone for fd_set`

- <span id="fd-set-clone"></span>`fn clone(&self) -> fd_set` — [`fd_set`](../index.md#fd-set)

##### `impl Copy for fd_set`

##### `impl Debug for fd_set`

- <span id="fd-set-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tm`

```rust
struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}
```

#### Trait Implementations

##### `impl Clone for tm`

- <span id="tm-clone"></span>`fn clone(&self) -> tm` — [`tm`](../index.md#tm)

##### `impl Copy for tm`

##### `impl Debug for tm`

- <span id="tm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sched_param`

```rust
struct sched_param {
    pub sched_priority: c_int,
}
```

#### Trait Implementations

##### `impl Clone for sched_param`

- <span id="sched-param-clone"></span>`fn clone(&self) -> sched_param` — [`sched_param`](../index.md#sched-param)

##### `impl Copy for sched_param`

##### `impl Debug for sched_param`

- <span id="sched-param-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Dl_info`

```rust
struct Dl_info {
    pub dli_fname: *const c_char,
    pub dli_fbase: *mut c_void,
    pub dli_sname: *const c_char,
    pub dli_saddr: *mut c_void,
}
```

#### Trait Implementations

##### `impl Clone for Dl_info`

- <span id="dl-info-clone"></span>`fn clone(&self) -> Dl_info` — [`Dl_info`](../index.md#dl-info)

##### `impl Copy for Dl_info`

##### `impl Debug for Dl_info`

- <span id="dl-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `lconv`

```rust
struct lconv {
    pub decimal_point: *mut c_char,
    pub thousands_sep: *mut c_char,
    pub grouping: *mut c_char,
    pub int_curr_symbol: *mut c_char,
    pub currency_symbol: *mut c_char,
    pub mon_decimal_point: *mut c_char,
    pub mon_thousands_sep: *mut c_char,
    pub mon_grouping: *mut c_char,
    pub positive_sign: *mut c_char,
    pub negative_sign: *mut c_char,
    pub int_frac_digits: c_char,
    pub frac_digits: c_char,
    pub p_cs_precedes: c_char,
    pub p_sep_by_space: c_char,
    pub n_cs_precedes: c_char,
    pub n_sep_by_space: c_char,
    pub p_sign_posn: c_char,
    pub n_sign_posn: c_char,
    pub int_p_cs_precedes: c_char,
    pub int_p_sep_by_space: c_char,
    pub int_n_cs_precedes: c_char,
    pub int_n_sep_by_space: c_char,
    pub int_p_sign_posn: c_char,
    pub int_n_sign_posn: c_char,
}
```

#### Trait Implementations

##### `impl Clone for lconv`

- <span id="lconv-clone"></span>`fn clone(&self) -> lconv` — [`lconv`](../index.md#lconv)

##### `impl Copy for lconv`

##### `impl Debug for lconv`

- <span id="lconv-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in_pktinfo`

```rust
struct in_pktinfo {
    pub ipi_ifindex: c_int,
    pub ipi_spec_dst: crate::in_addr,
    pub ipi_addr: crate::in_addr,
}
```

#### Trait Implementations

##### `impl Clone for in_pktinfo`

- <span id="in-pktinfo-clone"></span>`fn clone(&self) -> in_pktinfo` — [`in_pktinfo`](../index.md#in-pktinfo)

##### `impl Copy for in_pktinfo`

##### `impl Debug for in_pktinfo`

- <span id="in-pktinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ifaddrs`

```rust
struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut c_char,
    pub ifa_flags: c_uint,
    pub ifa_addr: *mut crate::sockaddr,
    pub ifa_netmask: *mut crate::sockaddr,
    pub ifa_ifu: *mut crate::sockaddr,
    pub ifa_data: *mut c_void,
}
```

#### Trait Implementations

##### `impl Clone for ifaddrs`

- <span id="ifaddrs-clone"></span>`fn clone(&self) -> ifaddrs` — [`ifaddrs`](../index.md#ifaddrs)

##### `impl Copy for ifaddrs`

##### `impl Debug for ifaddrs`

- <span id="ifaddrs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in6_rtmsg`

```rust
struct in6_rtmsg {
    rtmsg_dst: crate::in6_addr,
    rtmsg_src: crate::in6_addr,
    rtmsg_gateway: crate::in6_addr,
    rtmsg_type: u32,
    rtmsg_dst_len: u16,
    rtmsg_src_len: u16,
    rtmsg_metric: u32,
    rtmsg_info: c_ulong,
    rtmsg_flags: u32,
    rtmsg_ifindex: c_int,
}
```

#### Trait Implementations

##### `impl Clone for in6_rtmsg`

- <span id="in6-rtmsg-clone"></span>`fn clone(&self) -> in6_rtmsg` — [`in6_rtmsg`](../index.md#in6-rtmsg)

##### `impl Copy for in6_rtmsg`

##### `impl Debug for in6_rtmsg`

- <span id="in6-rtmsg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `arpreq`

```rust
struct arpreq {
    pub arp_pa: crate::sockaddr,
    pub arp_ha: crate::sockaddr,
    pub arp_flags: c_int,
    pub arp_netmask: crate::sockaddr,
    pub arp_dev: [c_char; 16],
}
```

#### Trait Implementations

##### `impl Clone for arpreq`

- <span id="arpreq-clone"></span>`fn clone(&self) -> arpreq` — [`arpreq`](../index.md#arpreq)

##### `impl Copy for arpreq`

##### `impl Debug for arpreq`

- <span id="arpreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `arpreq_old`

```rust
struct arpreq_old {
    pub arp_pa: crate::sockaddr,
    pub arp_ha: crate::sockaddr,
    pub arp_flags: c_int,
    pub arp_netmask: crate::sockaddr,
}
```

#### Trait Implementations

##### `impl Clone for arpreq_old`

- <span id="arpreq-old-clone"></span>`fn clone(&self) -> arpreq_old` — [`arpreq_old`](../index.md#arpreq-old)

##### `impl Copy for arpreq_old`

##### `impl Debug for arpreq_old`

- <span id="arpreq-old-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `arphdr`

```rust
struct arphdr {
    pub ar_hrd: u16,
    pub ar_pro: u16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: u16,
}
```

#### Trait Implementations

##### `impl Clone for arphdr`

- <span id="arphdr-clone"></span>`fn clone(&self) -> arphdr` — [`arphdr`](../index.md#arphdr)

##### `impl Copy for arphdr`

##### `impl Debug for arphdr`

- <span id="arphdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mmsghdr`

```rust
struct mmsghdr {
    pub msg_hdr: crate::msghdr,
    pub msg_len: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for mmsghdr`

- <span id="mmsghdr-clone"></span>`fn clone(&self) -> mmsghdr` — [`mmsghdr`](../index.md#mmsghdr)

##### `impl Copy for mmsghdr`

##### `impl Debug for mmsghdr`

- <span id="mmsghdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_un`

```rust
struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [c_char; 108],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_un`

- <span id="sockaddr-un-clone"></span>`fn clone(&self) -> sockaddr_un` — [`sockaddr_un`](../index.md#sockaddr-un)

##### `impl Copy for sockaddr_un`

##### `impl Debug for sockaddr_un`

- <span id="sockaddr-un-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_storage`

```rust
struct sockaddr_storage {
    pub ss_family: sa_family_t,
    __ss_pad2: Padding<[u8; 118]>,
    __ss_align: size_t,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_storage`

- <span id="sockaddr-storage-clone"></span>`fn clone(&self) -> sockaddr_storage` — [`sockaddr_storage`](../index.md#sockaddr-storage)

##### `impl Copy for sockaddr_storage`

##### `impl Debug for sockaddr_storage`

- <span id="sockaddr-storage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `utsname`

```rust
struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}
```

#### Trait Implementations

##### `impl Clone for utsname`

- <span id="utsname-clone"></span>`fn clone(&self) -> utsname` — [`utsname`](../index.md#utsname)

##### `impl Copy for utsname`

##### `impl Debug for utsname`

- <span id="utsname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `if_nameindex`

```rust
struct if_nameindex {
    pub if_index: c_uint,
    pub if_name: *mut c_char,
}
```

#### Trait Implementations

##### `impl Clone for if_nameindex`

- <span id="if-nameindex-clone"></span>`fn clone(&self) -> if_nameindex` — [`if_nameindex`](../index.md#if-nameindex)

##### `impl Copy for if_nameindex`

##### `impl Debug for if_nameindex`

- <span id="if-nameindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `file_clone_range`

```rust
struct file_clone_range {
    pub src_fd: crate::__s64,
    pub src_offset: crate::__u64,
    pub src_length: crate::__u64,
    pub dest_offset: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for file_clone_range`

- <span id="file-clone-range-clone"></span>`fn clone(&self) -> file_clone_range` — [`file_clone_range`](../index.md#file-clone-range)

##### `impl Copy for file_clone_range`

##### `impl Debug for file_clone_range`

- <span id="file-clone-range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sock_filter`

```rust
struct sock_filter {
    pub code: __u16,
    pub jt: __u8,
    pub jf: __u8,
    pub k: __u32,
}
```

#### Trait Implementations

##### `impl Clone for sock_filter`

- <span id="sock-filter-clone"></span>`fn clone(&self) -> sock_filter` — [`sock_filter`](../index.md#sock-filter)

##### `impl Copy for sock_filter`

##### `impl Debug for sock_filter`

- <span id="sock-filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sock_fprog`

```rust
struct sock_fprog {
    pub len: c_ushort,
    pub filter: *mut sock_filter,
}
```

#### Trait Implementations

##### `impl Clone for sock_fprog`

- <span id="sock-fprog-clone"></span>`fn clone(&self) -> sock_fprog` — [`sock_fprog`](../index.md#sock-fprog)

##### `impl Copy for sock_fprog`

##### `impl Debug for sock_fprog`

- <span id="sock-fprog-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statx`

```rust
struct statx {
    pub stx_mask: crate::__u32,
    pub stx_blksize: crate::__u32,
    pub stx_attributes: crate::__u64,
    pub stx_nlink: crate::__u32,
    pub stx_uid: crate::__u32,
    pub stx_gid: crate::__u32,
    pub stx_mode: crate::__u16,
    __statx_pad1: Padding<[crate::__u16; 1]>,
    pub stx_ino: crate::__u64,
    pub stx_size: crate::__u64,
    pub stx_blocks: crate::__u64,
    pub stx_attributes_mask: crate::__u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: crate::__u32,
    pub stx_rdev_minor: crate::__u32,
    pub stx_dev_major: crate::__u32,
    pub stx_dev_minor: crate::__u32,
    pub stx_mnt_id: crate::__u64,
    pub stx_dio_mem_align: crate::__u32,
    pub stx_dio_offset_align: crate::__u32,
    __statx_pad3: Padding<[crate::__u64; 12]>,
}
```

#### Trait Implementations

##### `impl Clone for statx`

- <span id="statx-clone"></span>`fn clone(&self) -> statx` — [`statx`](../index.md#statx)

##### `impl Copy for statx`

##### `impl Debug for statx`

- <span id="statx-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statx_timestamp`

```rust
struct statx_timestamp {
    pub tv_sec: crate::__s64,
    pub tv_nsec: crate::__u32,
    __statx_timestamp_pad1: Padding<[crate::__s32; 1]>,
}
```

#### Trait Implementations

##### `impl Clone for statx_timestamp`

- <span id="statx-timestamp-clone"></span>`fn clone(&self) -> statx_timestamp` — [`statx_timestamp`](../index.md#statx-timestamp)

##### `impl Copy for statx_timestamp`

##### `impl Debug for statx_timestamp`

- <span id="statx-timestamp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `epoll_event`

```rust
struct epoll_event {
    pub events: u32,
    pub u64: u64,
}
```

#### Trait Implementations

##### `impl Clone for epoll_event`

- <span id="epoll-event-clone"></span>`fn clone(&self) -> epoll_event` — [`epoll_event`](../index.md#epoll-event)

##### `impl Copy for epoll_event`

##### `impl Debug for epoll_event`

- <span id="epoll-event-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sigevent`

```rust
struct sigevent {
    pub sigev_value: crate::sigval,
    pub sigev_signo: c_int,
    pub sigev_notify: c_int,
    pub sigev_notify_thread_id: c_int,
    __unused1: Padding<[c_int; 11]>,
}
```

#### Trait Implementations

##### `impl Clone for sigevent`

- <span id="sigevent-clone"></span>`fn clone(&self) -> sigevent` — [`sigevent`](../index.md#sigevent)

##### `impl Copy for sigevent`

##### `impl Debug for sigevent`

- <span id="sigevent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `dqblk`

```rust
struct dqblk {
    pub dqb_bhardlimit: u64,
    pub dqb_bsoftlimit: u64,
    pub dqb_curspace: u64,
    pub dqb_ihardlimit: u64,
    pub dqb_isoftlimit: u64,
    pub dqb_curinodes: u64,
    pub dqb_btime: u64,
    pub dqb_itime: u64,
    pub dqb_valid: u32,
}
```

#### Trait Implementations

##### `impl Clone for dqblk`

- <span id="dqblk-clone"></span>`fn clone(&self) -> dqblk` — [`dqblk`](#dqblk)

##### `impl Copy for dqblk`

##### `impl Debug for dqblk`

- <span id="dqblk-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `signalfd_siginfo`

```rust
struct signalfd_siginfo {
    pub ssi_signo: u32,
    pub ssi_errno: i32,
    pub ssi_code: i32,
    pub ssi_pid: u32,
    pub ssi_uid: u32,
    pub ssi_fd: i32,
    pub ssi_tid: u32,
    pub ssi_band: u32,
    pub ssi_overrun: u32,
    pub ssi_trapno: u32,
    pub ssi_status: i32,
    pub ssi_int: i32,
    pub ssi_ptr: u64,
    pub ssi_utime: u64,
    pub ssi_stime: u64,
    pub ssi_addr: u64,
    pub ssi_addr_lsb: u16,
    _pad2: Padding<u16>,
    pub ssi_syscall: i32,
    pub ssi_call_addr: u64,
    pub ssi_arch: u32,
    _pad: Padding<[u8; 28]>,
}
```

#### Trait Implementations

##### `impl Clone for signalfd_siginfo`

- <span id="signalfd-siginfo-clone"></span>`fn clone(&self) -> signalfd_siginfo` — [`signalfd_siginfo`](#signalfd-siginfo)

##### `impl Copy for signalfd_siginfo`

##### `impl Debug for signalfd_siginfo`

- <span id="signalfd-siginfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fanout_args`

```rust
struct fanout_args {
    pub id: __u16,
    pub type_flags: __u16,
    pub max_num_members: __u32,
}
```

#### Trait Implementations

##### `impl Clone for fanout_args`

- <span id="fanout-args-clone"></span>`fn clone(&self) -> fanout_args` — [`fanout_args`](#fanout-args)

##### `impl Copy for fanout_args`

##### `impl Debug for fanout_args`

- <span id="fanout-args-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_pkt`

```rust
struct sockaddr_pkt {
    pub spkt_family: c_ushort,
    pub spkt_device: [c_uchar; 14],
    pub spkt_protocol: c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_pkt`

- <span id="sockaddr-pkt-clone"></span>`fn clone(&self) -> sockaddr_pkt` — [`sockaddr_pkt`](#sockaddr-pkt)

##### `impl Copy for sockaddr_pkt`

##### `impl Debug for sockaddr_pkt`

- <span id="sockaddr-pkt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_auxdata`

```rust
struct tpacket_auxdata {
    pub tp_status: __u32,
    pub tp_len: __u32,
    pub tp_snaplen: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    pub tp_vlan_tci: __u16,
    pub tp_vlan_tpid: __u16,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_auxdata`

- <span id="tpacket-auxdata-clone"></span>`fn clone(&self) -> tpacket_auxdata` — [`tpacket_auxdata`](#tpacket-auxdata)

##### `impl Copy for tpacket_auxdata`

##### `impl Debug for tpacket_auxdata`

- <span id="tpacket-auxdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_hdr`

```rust
struct tpacket_hdr {
    pub tp_status: c_ulong,
    pub tp_len: c_uint,
    pub tp_snaplen: c_uint,
    pub tp_mac: c_ushort,
    pub tp_net: c_ushort,
    pub tp_sec: c_uint,
    pub tp_usec: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_hdr`

- <span id="tpacket-hdr-clone"></span>`fn clone(&self) -> tpacket_hdr` — [`tpacket_hdr`](#tpacket-hdr)

##### `impl Copy for tpacket_hdr`

##### `impl Debug for tpacket_hdr`

- <span id="tpacket-hdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_hdr_variant1`

```rust
struct tpacket_hdr_variant1 {
    pub tp_rxhash: __u32,
    pub tp_vlan_tci: __u32,
    pub tp_vlan_tpid: __u16,
    pub tp_padding: __u16,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_hdr_variant1`

- <span id="tpacket-hdr-variant1-clone"></span>`fn clone(&self) -> tpacket_hdr_variant1` — [`tpacket_hdr_variant1`](#tpacket-hdr-variant1)

##### `impl Copy for tpacket_hdr_variant1`

##### `impl Debug for tpacket_hdr_variant1`

- <span id="tpacket-hdr-variant1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket2_hdr`

```rust
struct tpacket2_hdr {
    pub tp_status: __u32,
    pub tp_len: __u32,
    pub tp_snaplen: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    pub tp_sec: __u32,
    pub tp_nsec: __u32,
    pub tp_vlan_tci: __u16,
    pub tp_vlan_tpid: __u16,
    pub tp_padding: [__u8; 4],
}
```

#### Trait Implementations

##### `impl Clone for tpacket2_hdr`

- <span id="tpacket2-hdr-clone"></span>`fn clone(&self) -> tpacket2_hdr` — [`tpacket2_hdr`](#tpacket2-hdr)

##### `impl Copy for tpacket2_hdr`

##### `impl Debug for tpacket2_hdr`

- <span id="tpacket2-hdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_req`

```rust
struct tpacket_req {
    pub tp_block_size: c_uint,
    pub tp_block_nr: c_uint,
    pub tp_frame_size: c_uint,
    pub tp_frame_nr: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_req`

- <span id="tpacket-req-clone"></span>`fn clone(&self) -> tpacket_req` — [`tpacket_req`](#tpacket-req)

##### `impl Copy for tpacket_req`

##### `impl Debug for tpacket_req`

- <span id="tpacket-req-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_req3`

```rust
struct tpacket_req3 {
    pub tp_block_size: c_uint,
    pub tp_block_nr: c_uint,
    pub tp_frame_size: c_uint,
    pub tp_frame_nr: c_uint,
    pub tp_retire_blk_tov: c_uint,
    pub tp_sizeof_priv: c_uint,
    pub tp_feature_req_word: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_req3`

- <span id="tpacket-req3-clone"></span>`fn clone(&self) -> tpacket_req3` — [`tpacket_req3`](#tpacket-req3)

##### `impl Copy for tpacket_req3`

##### `impl Debug for tpacket_req3`

- <span id="tpacket-req3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_rollover_stats`

```rust
struct tpacket_rollover_stats {
    pub tp_all: crate::__u64,
    pub tp_huge: crate::__u64,
    pub tp_failed: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_rollover_stats`

- <span id="tpacket-rollover-stats-clone"></span>`fn clone(&self) -> tpacket_rollover_stats` — [`tpacket_rollover_stats`](#tpacket-rollover-stats)

##### `impl Copy for tpacket_rollover_stats`

##### `impl Debug for tpacket_rollover_stats`

- <span id="tpacket-rollover-stats-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_stats`

```rust
struct tpacket_stats {
    pub tp_packets: c_uint,
    pub tp_drops: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_stats`

- <span id="tpacket-stats-clone"></span>`fn clone(&self) -> tpacket_stats` — [`tpacket_stats`](#tpacket-stats)

##### `impl Copy for tpacket_stats`

##### `impl Debug for tpacket_stats`

- <span id="tpacket-stats-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_stats_v3`

```rust
struct tpacket_stats_v3 {
    pub tp_packets: c_uint,
    pub tp_drops: c_uint,
    pub tp_freeze_q_cnt: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_stats_v3`

- <span id="tpacket-stats-v3-clone"></span>`fn clone(&self) -> tpacket_stats_v3` — [`tpacket_stats_v3`](#tpacket-stats-v3)

##### `impl Copy for tpacket_stats_v3`

##### `impl Debug for tpacket_stats_v3`

- <span id="tpacket-stats-v3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket3_hdr`

```rust
struct tpacket3_hdr {
    pub tp_next_offset: __u32,
    pub tp_sec: __u32,
    pub tp_nsec: __u32,
    pub tp_snaplen: __u32,
    pub tp_len: __u32,
    pub tp_status: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    pub hv1: crate::tpacket_hdr_variant1,
    pub tp_padding: [__u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for tpacket3_hdr`

- <span id="tpacket3-hdr-clone"></span>`fn clone(&self) -> tpacket3_hdr` — [`tpacket3_hdr`](#tpacket3-hdr)

##### `impl Copy for tpacket3_hdr`

##### `impl Debug for tpacket3_hdr`

- <span id="tpacket3-hdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_bd_ts`

```rust
struct tpacket_bd_ts {
    pub ts_sec: c_uint,
    pub ts_usec: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_bd_ts`

- <span id="tpacket-bd-ts-clone"></span>`fn clone(&self) -> tpacket_bd_ts` — [`tpacket_bd_ts`](#tpacket-bd-ts)

##### `impl Copy for tpacket_bd_ts`

##### `impl Debug for tpacket_bd_ts`

- <span id="tpacket-bd-ts-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_hdr_v1`

```rust
struct tpacket_hdr_v1 {
    pub block_status: __u32,
    pub num_pkts: __u32,
    pub offset_to_first_pkt: __u32,
    pub blk_len: __u32,
    pub seq_num: crate::__u64,
    pub ts_first_pkt: crate::tpacket_bd_ts,
    pub ts_last_pkt: crate::tpacket_bd_ts,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_hdr_v1`

- <span id="tpacket-hdr-v1-clone"></span>`fn clone(&self) -> tpacket_hdr_v1` — [`tpacket_hdr_v1`](#tpacket-hdr-v1)

##### `impl Copy for tpacket_hdr_v1`

##### `impl Debug for tpacket_hdr_v1`

- <span id="tpacket-hdr-v1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `msginfo`

```rust
struct msginfo {
    pub msgpool: c_int,
    pub msgmap: c_int,
    pub msgmax: c_int,
    pub msgmnb: c_int,
    pub msgmni: c_int,
    pub msgssz: c_int,
    pub msgtql: c_int,
    pub msgseg: c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for msginfo`

- <span id="msginfo-clone"></span>`fn clone(&self) -> msginfo` — [`msginfo`](#msginfo)

##### `impl Copy for msginfo`

##### `impl Debug for msginfo`

- <span id="msginfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `input_event`

```rust
struct input_event {
    pub time: crate::timeval,
    pub type_: __u16,
    pub code: __u16,
    pub value: __s32,
}
```

#### Trait Implementations

##### `impl Clone for input_event`

- <span id="input-event-clone"></span>`fn clone(&self) -> input_event` — [`input_event`](#input-event)

##### `impl Copy for input_event`

##### `impl Debug for input_event`

- <span id="input-event-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `input_id`

```rust
struct input_id {
    pub bustype: __u16,
    pub vendor: __u16,
    pub product: __u16,
    pub version: __u16,
}
```

#### Trait Implementations

##### `impl Clone for input_id`

- <span id="input-id-clone"></span>`fn clone(&self) -> input_id` — [`input_id`](#input-id)

##### `impl Copy for input_id`

##### `impl Debug for input_id`

- <span id="input-id-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `input_absinfo`

```rust
struct input_absinfo {
    pub value: __s32,
    pub minimum: __s32,
    pub maximum: __s32,
    pub fuzz: __s32,
    pub flat: __s32,
    pub resolution: __s32,
}
```

#### Trait Implementations

##### `impl Clone for input_absinfo`

- <span id="input-absinfo-clone"></span>`fn clone(&self) -> input_absinfo` — [`input_absinfo`](#input-absinfo)

##### `impl Copy for input_absinfo`

##### `impl Debug for input_absinfo`

- <span id="input-absinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `input_keymap_entry`

```rust
struct input_keymap_entry {
    pub flags: __u8,
    pub len: __u8,
    pub index: __u16,
    pub keycode: __u32,
    pub scancode: [__u8; 32],
}
```

#### Trait Implementations

##### `impl Clone for input_keymap_entry`

- <span id="input-keymap-entry-clone"></span>`fn clone(&self) -> input_keymap_entry` — [`input_keymap_entry`](#input-keymap-entry)

##### `impl Copy for input_keymap_entry`

##### `impl Debug for input_keymap_entry`

- <span id="input-keymap-entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `input_mask`

```rust
struct input_mask {
    pub type_: __u32,
    pub codes_size: __u32,
    pub codes_ptr: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for input_mask`

- <span id="input-mask-clone"></span>`fn clone(&self) -> input_mask` — [`input_mask`](#input-mask)

##### `impl Copy for input_mask`

##### `impl Debug for input_mask`

- <span id="input-mask-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ff_replay`

```rust
struct ff_replay {
    pub length: __u16,
    pub delay: __u16,
}
```

#### Trait Implementations

##### `impl Clone for ff_replay`

- <span id="ff-replay-clone"></span>`fn clone(&self) -> ff_replay` — [`ff_replay`](#ff-replay)

##### `impl Copy for ff_replay`

##### `impl Debug for ff_replay`

- <span id="ff-replay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ff_trigger`

```rust
struct ff_trigger {
    pub button: __u16,
    pub interval: __u16,
}
```

#### Trait Implementations

##### `impl Clone for ff_trigger`

- <span id="ff-trigger-clone"></span>`fn clone(&self) -> ff_trigger` — [`ff_trigger`](#ff-trigger)

##### `impl Copy for ff_trigger`

##### `impl Debug for ff_trigger`

- <span id="ff-trigger-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ff_envelope`

```rust
struct ff_envelope {
    pub attack_length: __u16,
    pub attack_level: __u16,
    pub fade_length: __u16,
    pub fade_level: __u16,
}
```

#### Trait Implementations

##### `impl Clone for ff_envelope`

- <span id="ff-envelope-clone"></span>`fn clone(&self) -> ff_envelope` — [`ff_envelope`](#ff-envelope)

##### `impl Copy for ff_envelope`

##### `impl Debug for ff_envelope`

- <span id="ff-envelope-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ff_constant_effect`

```rust
struct ff_constant_effect {
    pub level: __s16,
    pub envelope: ff_envelope,
}
```

#### Trait Implementations

##### `impl Clone for ff_constant_effect`

- <span id="ff-constant-effect-clone"></span>`fn clone(&self) -> ff_constant_effect` — [`ff_constant_effect`](#ff-constant-effect)

##### `impl Copy for ff_constant_effect`

##### `impl Debug for ff_constant_effect`

- <span id="ff-constant-effect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ff_ramp_effect`

```rust
struct ff_ramp_effect {
    pub start_level: __s16,
    pub end_level: __s16,
    pub envelope: ff_envelope,
}
```

#### Trait Implementations

##### `impl Clone for ff_ramp_effect`

- <span id="ff-ramp-effect-clone"></span>`fn clone(&self) -> ff_ramp_effect` — [`ff_ramp_effect`](#ff-ramp-effect)

##### `impl Copy for ff_ramp_effect`

##### `impl Debug for ff_ramp_effect`

- <span id="ff-ramp-effect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ff_condition_effect`

```rust
struct ff_condition_effect {
    pub right_saturation: __u16,
    pub left_saturation: __u16,
    pub right_coeff: __s16,
    pub left_coeff: __s16,
    pub deadband: __u16,
    pub center: __s16,
}
```

#### Trait Implementations

##### `impl Clone for ff_condition_effect`

- <span id="ff-condition-effect-clone"></span>`fn clone(&self) -> ff_condition_effect` — [`ff_condition_effect`](#ff-condition-effect)

##### `impl Copy for ff_condition_effect`

##### `impl Debug for ff_condition_effect`

- <span id="ff-condition-effect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ff_periodic_effect`

```rust
struct ff_periodic_effect {
    pub waveform: __u16,
    pub period: __u16,
    pub magnitude: __s16,
    pub offset: __s16,
    pub phase: __u16,
    pub envelope: ff_envelope,
    pub custom_len: __u32,
    pub custom_data: *mut __s16,
}
```

#### Trait Implementations

##### `impl Clone for ff_periodic_effect`

- <span id="ff-periodic-effect-clone"></span>`fn clone(&self) -> ff_periodic_effect` — [`ff_periodic_effect`](#ff-periodic-effect)

##### `impl Copy for ff_periodic_effect`

##### `impl Debug for ff_periodic_effect`

- <span id="ff-periodic-effect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ff_rumble_effect`

```rust
struct ff_rumble_effect {
    pub strong_magnitude: __u16,
    pub weak_magnitude: __u16,
}
```

#### Trait Implementations

##### `impl Clone for ff_rumble_effect`

- <span id="ff-rumble-effect-clone"></span>`fn clone(&self) -> ff_rumble_effect` — [`ff_rumble_effect`](#ff-rumble-effect)

##### `impl Copy for ff_rumble_effect`

##### `impl Debug for ff_rumble_effect`

- <span id="ff-rumble-effect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ff_effect`

```rust
struct ff_effect {
    pub type_: __u16,
    pub id: __s16,
    pub direction: __u16,
    pub trigger: ff_trigger,
    pub replay: ff_replay,
    pub u: [u64; 4],
}
```

#### Trait Implementations

##### `impl Clone for ff_effect`

- <span id="ff-effect-clone"></span>`fn clone(&self) -> ff_effect` — [`ff_effect`](#ff-effect)

##### `impl Copy for ff_effect`

##### `impl Debug for ff_effect`

- <span id="ff-effect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uinput_ff_upload`

```rust
struct uinput_ff_upload {
    pub request_id: __u32,
    pub retval: __s32,
    pub effect: ff_effect,
    pub old: ff_effect,
}
```

#### Trait Implementations

##### `impl Clone for uinput_ff_upload`

- <span id="uinput-ff-upload-clone"></span>`fn clone(&self) -> uinput_ff_upload` — [`uinput_ff_upload`](#uinput-ff-upload)

##### `impl Copy for uinput_ff_upload`

##### `impl Debug for uinput_ff_upload`

- <span id="uinput-ff-upload-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uinput_ff_erase`

```rust
struct uinput_ff_erase {
    pub request_id: __u32,
    pub retval: __s32,
    pub effect_id: __u32,
}
```

#### Trait Implementations

##### `impl Clone for uinput_ff_erase`

- <span id="uinput-ff-erase-clone"></span>`fn clone(&self) -> uinput_ff_erase` — [`uinput_ff_erase`](#uinput-ff-erase)

##### `impl Copy for uinput_ff_erase`

##### `impl Debug for uinput_ff_erase`

- <span id="uinput-ff-erase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uinput_abs_setup`

```rust
struct uinput_abs_setup {
    pub code: __u16,
    pub absinfo: input_absinfo,
}
```

#### Trait Implementations

##### `impl Clone for uinput_abs_setup`

- <span id="uinput-abs-setup-clone"></span>`fn clone(&self) -> uinput_abs_setup` — [`uinput_abs_setup`](#uinput-abs-setup)

##### `impl Copy for uinput_abs_setup`

##### `impl Debug for uinput_abs_setup`

- <span id="uinput-abs-setup-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous__kernel_fsid_t`

```rust
struct __c_anonymous__kernel_fsid_t {
    pub val: [c_int; 2],
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous__kernel_fsid_t`

- <span id="c-anonymous-kernel-fsid-t-clone"></span>`fn clone(&self) -> __c_anonymous__kernel_fsid_t` — [`__c_anonymous__kernel_fsid_t`](#c-anonymous-kernel-fsid-t)

##### `impl Copy for __c_anonymous__kernel_fsid_t`

##### `impl Debug for __c_anonymous__kernel_fsid_t`

- <span id="c-anonymous-kernel-fsid-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `posix_spawn_file_actions_t`

```rust
struct posix_spawn_file_actions_t {
    __allocated: c_int,
    __used: c_int,
    __actions: *mut c_int,
    __pad: Padding<[c_int; 16]>,
}
```

#### Trait Implementations

##### `impl Clone for posix_spawn_file_actions_t`

- <span id="posix-spawn-file-actions-t-clone"></span>`fn clone(&self) -> posix_spawn_file_actions_t` — [`posix_spawn_file_actions_t`](#posix-spawn-file-actions-t)

##### `impl Copy for posix_spawn_file_actions_t`

##### `impl Debug for posix_spawn_file_actions_t`

- <span id="posix-spawn-file-actions-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `posix_spawnattr_t`

```rust
struct posix_spawnattr_t {
    __flags: c_short,
    __pgrp: crate::pid_t,
    __sd: crate::sigset_t,
    __ss: crate::sigset_t,
    __sp: crate::sched_param,
    __policy: c_int,
    __pad: Padding<[c_int; 16]>,
}
```

#### Trait Implementations

##### `impl Clone for posix_spawnattr_t`

- <span id="posix-spawnattr-t-clone"></span>`fn clone(&self) -> posix_spawnattr_t` — [`posix_spawnattr_t`](#posix-spawnattr-t)

##### `impl Copy for posix_spawnattr_t`

##### `impl Debug for posix_spawnattr_t`

- <span id="posix-spawnattr-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `genlmsghdr`

```rust
struct genlmsghdr {
    pub cmd: u8,
    pub version: u8,
    pub reserved: u16,
}
```

#### Trait Implementations

##### `impl Clone for genlmsghdr`

- <span id="genlmsghdr-clone"></span>`fn clone(&self) -> genlmsghdr` — [`genlmsghdr`](#genlmsghdr)

##### `impl Copy for genlmsghdr`

##### `impl Debug for genlmsghdr`

- <span id="genlmsghdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `inotify_event`

```rust
struct inotify_event {
    pub wd: c_int,
    pub mask: u32,
    pub cookie: u32,
    pub len: u32,
}
```

#### Trait Implementations

##### `impl Clone for inotify_event`

- <span id="inotify-event-clone"></span>`fn clone(&self) -> inotify_event` — [`inotify_event`](#inotify-event)

##### `impl Copy for inotify_event`

##### `impl Debug for inotify_event`

- <span id="inotify-event-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fanotify_response`

```rust
struct fanotify_response {
    pub fd: c_int,
    pub response: __u32,
}
```

#### Trait Implementations

##### `impl Clone for fanotify_response`

- <span id="fanotify-response-clone"></span>`fn clone(&self) -> fanotify_response` — [`fanotify_response`](#fanotify-response)

##### `impl Copy for fanotify_response`

##### `impl Debug for fanotify_response`

- <span id="fanotify-response-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fanotify_event_info_header`

```rust
struct fanotify_event_info_header {
    pub info_type: __u8,
    pub pad: __u8,
    pub len: __u16,
}
```

#### Trait Implementations

##### `impl Clone for fanotify_event_info_header`

- <span id="fanotify-event-info-header-clone"></span>`fn clone(&self) -> fanotify_event_info_header` — [`fanotify_event_info_header`](#fanotify-event-info-header)

##### `impl Copy for fanotify_event_info_header`

##### `impl Debug for fanotify_event_info_header`

- <span id="fanotify-event-info-header-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fanotify_event_info_fid`

```rust
struct fanotify_event_info_fid {
    pub hdr: fanotify_event_info_header,
    pub fsid: __kernel_fsid_t,
    pub handle: [c_uchar; 0],
}
```

#### Trait Implementations

##### `impl Clone for fanotify_event_info_fid`

- <span id="fanotify-event-info-fid-clone"></span>`fn clone(&self) -> fanotify_event_info_fid` — [`fanotify_event_info_fid`](#fanotify-event-info-fid)

##### `impl Copy for fanotify_event_info_fid`

##### `impl Debug for fanotify_event_info_fid`

- <span id="fanotify-event-info-fid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_vm`

```rust
struct sockaddr_vm {
    pub svm_family: crate::sa_family_t,
    pub svm_reserved1: c_ushort,
    pub svm_port: c_uint,
    pub svm_cid: c_uint,
    pub svm_zero: [u8; 4],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_vm`

- <span id="sockaddr-vm-clone"></span>`fn clone(&self) -> sockaddr_vm` — [`sockaddr_vm`](#sockaddr-vm)

##### `impl Copy for sockaddr_vm`

##### `impl Debug for sockaddr_vm`

- <span id="sockaddr-vm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sock_extended_err`

```rust
struct sock_extended_err {
    pub ee_errno: u32,
    pub ee_origin: u8,
    pub ee_type: u8,
    pub ee_code: u8,
    pub ee_pad: u8,
    pub ee_info: u32,
    pub ee_data: u32,
}
```

#### Trait Implementations

##### `impl Clone for sock_extended_err`

- <span id="sock-extended-err-clone"></span>`fn clone(&self) -> sock_extended_err` — [`sock_extended_err`](#sock-extended-err)

##### `impl Copy for sock_extended_err`

##### `impl Debug for sock_extended_err`

- <span id="sock-extended-err-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `seccomp_data`

```rust
struct seccomp_data {
    pub nr: c_int,
    pub arch: __u32,
    pub instruction_pointer: crate::__u64,
    pub args: [crate::__u64; 6],
}
```

#### Trait Implementations

##### `impl Clone for seccomp_data`

- <span id="seccomp-data-clone"></span>`fn clone(&self) -> seccomp_data` — [`seccomp_data`](#seccomp-data)

##### `impl Copy for seccomp_data`

##### `impl Debug for seccomp_data`

- <span id="seccomp-data-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `seccomp_notif_sizes`

```rust
struct seccomp_notif_sizes {
    pub seccomp_notif: __u16,
    pub seccomp_notif_resp: __u16,
    pub seccomp_data: __u16,
}
```

#### Trait Implementations

##### `impl Clone for seccomp_notif_sizes`

- <span id="seccomp-notif-sizes-clone"></span>`fn clone(&self) -> seccomp_notif_sizes` — [`seccomp_notif_sizes`](#seccomp-notif-sizes)

##### `impl Copy for seccomp_notif_sizes`

##### `impl Debug for seccomp_notif_sizes`

- <span id="seccomp-notif-sizes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `seccomp_notif`

```rust
struct seccomp_notif {
    pub id: crate::__u64,
    pub pid: __u32,
    pub flags: __u32,
    pub data: seccomp_data,
}
```

#### Trait Implementations

##### `impl Clone for seccomp_notif`

- <span id="seccomp-notif-clone"></span>`fn clone(&self) -> seccomp_notif` — [`seccomp_notif`](#seccomp-notif)

##### `impl Copy for seccomp_notif`

##### `impl Debug for seccomp_notif`

- <span id="seccomp-notif-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `seccomp_notif_resp`

```rust
struct seccomp_notif_resp {
    pub id: crate::__u64,
    pub val: crate::__s64,
    pub error: __s32,
    pub flags: __u32,
}
```

#### Trait Implementations

##### `impl Clone for seccomp_notif_resp`

- <span id="seccomp-notif-resp-clone"></span>`fn clone(&self) -> seccomp_notif_resp` — [`seccomp_notif_resp`](#seccomp-notif-resp)

##### `impl Copy for seccomp_notif_resp`

##### `impl Debug for seccomp_notif_resp`

- <span id="seccomp-notif-resp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `seccomp_notif_addfd`

```rust
struct seccomp_notif_addfd {
    pub id: crate::__u64,
    pub flags: __u32,
    pub srcfd: __u32,
    pub newfd: __u32,
    pub newfd_flags: __u32,
}
```

#### Trait Implementations

##### `impl Clone for seccomp_notif_addfd`

- <span id="seccomp-notif-addfd-clone"></span>`fn clone(&self) -> seccomp_notif_addfd` — [`seccomp_notif_addfd`](#seccomp-notif-addfd)

##### `impl Copy for seccomp_notif_addfd`

##### `impl Debug for seccomp_notif_addfd`

- <span id="seccomp-notif-addfd-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in6_ifreq`

```rust
struct in6_ifreq {
    pub ifr6_addr: crate::in6_addr,
    pub ifr6_prefixlen: u32,
    pub ifr6_ifindex: c_int,
}
```

#### Trait Implementations

##### `impl Clone for in6_ifreq`

- <span id="in6-ifreq-clone"></span>`fn clone(&self) -> in6_ifreq` — [`in6_ifreq`](#in6-ifreq)

##### `impl Copy for in6_ifreq`

##### `impl Debug for in6_ifreq`

- <span id="in6-ifreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `open_how`

```rust
struct open_how {
    pub flags: crate::__u64,
    pub mode: crate::__u64,
    pub resolve: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for open_how`

- <span id="open-how-clone"></span>`fn clone(&self) -> open_how` — [`open_how`](#open-how)

##### `impl Copy for open_how`

##### `impl Debug for open_how`

- <span id="open-how-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptp_clock_time`

```rust
struct ptp_clock_time {
    pub sec: crate::__s64,
    pub nsec: __u32,
    pub reserved: __u32,
}
```

#### Trait Implementations

##### `impl Clone for ptp_clock_time`

- <span id="ptp-clock-time-clone"></span>`fn clone(&self) -> ptp_clock_time` — [`ptp_clock_time`](#ptp-clock-time)

##### `impl Copy for ptp_clock_time`

##### `impl Debug for ptp_clock_time`

- <span id="ptp-clock-time-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptp_extts_request`

```rust
struct ptp_extts_request {
    pub index: c_uint,
    pub flags: c_uint,
    pub rsv: [c_uint; 2],
}
```

#### Trait Implementations

##### `impl Clone for ptp_extts_request`

- <span id="ptp-extts-request-clone"></span>`fn clone(&self) -> ptp_extts_request` — [`ptp_extts_request`](#ptp-extts-request)

##### `impl Copy for ptp_extts_request`

##### `impl Debug for ptp_extts_request`

- <span id="ptp-extts-request-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptp_sys_offset_extended`

```rust
struct ptp_sys_offset_extended {
    pub n_samples: c_uint,
    pub clockid: __kernel_clockid_t,
    pub rsv: [c_uint; 2],
    pub ts: [[ptp_clock_time; 3]; 25],
}
```

#### Trait Implementations

##### `impl Clone for ptp_sys_offset_extended`

- <span id="ptp-sys-offset-extended-clone"></span>`fn clone(&self) -> ptp_sys_offset_extended` — [`ptp_sys_offset_extended`](#ptp-sys-offset-extended)

##### `impl Copy for ptp_sys_offset_extended`

##### `impl Debug for ptp_sys_offset_extended`

- <span id="ptp-sys-offset-extended-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptp_sys_offset_precise`

```rust
struct ptp_sys_offset_precise {
    pub device: ptp_clock_time,
    pub sys_realtime: ptp_clock_time,
    pub sys_monoraw: ptp_clock_time,
    pub rsv: [c_uint; 4],
}
```

#### Trait Implementations

##### `impl Clone for ptp_sys_offset_precise`

- <span id="ptp-sys-offset-precise-clone"></span>`fn clone(&self) -> ptp_sys_offset_precise` — [`ptp_sys_offset_precise`](#ptp-sys-offset-precise)

##### `impl Copy for ptp_sys_offset_precise`

##### `impl Debug for ptp_sys_offset_precise`

- <span id="ptp-sys-offset-precise-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptp_extts_event`

```rust
struct ptp_extts_event {
    pub t: ptp_clock_time,
    index: c_uint,
    flags: c_uint,
    rsv: [c_uint; 2],
}
```

#### Trait Implementations

##### `impl Clone for ptp_extts_event`

- <span id="ptp-extts-event-clone"></span>`fn clone(&self) -> ptp_extts_event` — [`ptp_extts_event`](#ptp-extts-event)

##### `impl Copy for ptp_extts_event`

##### `impl Debug for ptp_extts_event`

- <span id="ptp-extts-event-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sctp_initmsg`

```rust
struct sctp_initmsg {
    pub sinit_num_ostreams: __u16,
    pub sinit_max_instreams: __u16,
    pub sinit_max_attempts: __u16,
    pub sinit_max_init_timeo: __u16,
}
```

#### Trait Implementations

##### `impl Clone for sctp_initmsg`

- <span id="sctp-initmsg-clone"></span>`fn clone(&self) -> sctp_initmsg` — [`sctp_initmsg`](#sctp-initmsg)

##### `impl Copy for sctp_initmsg`

##### `impl Debug for sctp_initmsg`

- <span id="sctp-initmsg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sctp_sndrcvinfo`

```rust
struct sctp_sndrcvinfo {
    pub sinfo_stream: __u16,
    pub sinfo_ssn: __u16,
    pub sinfo_flags: __u16,
    pub sinfo_ppid: __u32,
    pub sinfo_context: __u32,
    pub sinfo_timetolive: __u32,
    pub sinfo_tsn: __u32,
    pub sinfo_cumtsn: __u32,
    pub sinfo_assoc_id: crate::sctp_assoc_t,
}
```

#### Trait Implementations

##### `impl Clone for sctp_sndrcvinfo`

- <span id="sctp-sndrcvinfo-clone"></span>`fn clone(&self) -> sctp_sndrcvinfo` — [`sctp_sndrcvinfo`](#sctp-sndrcvinfo)

##### `impl Copy for sctp_sndrcvinfo`

##### `impl Debug for sctp_sndrcvinfo`

- <span id="sctp-sndrcvinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sctp_sndinfo`

```rust
struct sctp_sndinfo {
    pub snd_sid: __u16,
    pub snd_flags: __u16,
    pub snd_ppid: __u32,
    pub snd_context: __u32,
    pub snd_assoc_id: crate::sctp_assoc_t,
}
```

#### Trait Implementations

##### `impl Clone for sctp_sndinfo`

- <span id="sctp-sndinfo-clone"></span>`fn clone(&self) -> sctp_sndinfo` — [`sctp_sndinfo`](#sctp-sndinfo)

##### `impl Copy for sctp_sndinfo`

##### `impl Debug for sctp_sndinfo`

- <span id="sctp-sndinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sctp_rcvinfo`

```rust
struct sctp_rcvinfo {
    pub rcv_sid: __u16,
    pub rcv_ssn: __u16,
    pub rcv_flags: __u16,
    pub rcv_ppid: __u32,
    pub rcv_tsn: __u32,
    pub rcv_cumtsn: __u32,
    pub rcv_context: __u32,
    pub rcv_assoc_id: crate::sctp_assoc_t,
}
```

#### Trait Implementations

##### `impl Clone for sctp_rcvinfo`

- <span id="sctp-rcvinfo-clone"></span>`fn clone(&self) -> sctp_rcvinfo` — [`sctp_rcvinfo`](#sctp-rcvinfo)

##### `impl Copy for sctp_rcvinfo`

##### `impl Debug for sctp_rcvinfo`

- <span id="sctp-rcvinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sctp_nxtinfo`

```rust
struct sctp_nxtinfo {
    pub nxt_sid: __u16,
    pub nxt_flags: __u16,
    pub nxt_ppid: __u32,
    pub nxt_length: __u32,
    pub nxt_assoc_id: crate::sctp_assoc_t,
}
```

#### Trait Implementations

##### `impl Clone for sctp_nxtinfo`

- <span id="sctp-nxtinfo-clone"></span>`fn clone(&self) -> sctp_nxtinfo` — [`sctp_nxtinfo`](#sctp-nxtinfo)

##### `impl Copy for sctp_nxtinfo`

##### `impl Debug for sctp_nxtinfo`

- <span id="sctp-nxtinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sctp_prinfo`

```rust
struct sctp_prinfo {
    pub pr_policy: __u16,
    pub pr_value: __u32,
}
```

#### Trait Implementations

##### `impl Clone for sctp_prinfo`

- <span id="sctp-prinfo-clone"></span>`fn clone(&self) -> sctp_prinfo` — [`sctp_prinfo`](#sctp-prinfo)

##### `impl Copy for sctp_prinfo`

##### `impl Debug for sctp_prinfo`

- <span id="sctp-prinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sctp_authinfo`

```rust
struct sctp_authinfo {
    pub auth_keynumber: __u16,
}
```

#### Trait Implementations

##### `impl Clone for sctp_authinfo`

- <span id="sctp-authinfo-clone"></span>`fn clone(&self) -> sctp_authinfo` — [`sctp_authinfo`](#sctp-authinfo)

##### `impl Copy for sctp_authinfo`

##### `impl Debug for sctp_authinfo`

- <span id="sctp-authinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tls_crypto_info`

```rust
struct tls_crypto_info {
    pub version: __u16,
    pub cipher_type: __u16,
}
```

#### Trait Implementations

##### `impl Clone for tls_crypto_info`

- <span id="tls-crypto-info-clone"></span>`fn clone(&self) -> tls_crypto_info` — [`tls_crypto_info`](#tls-crypto-info)

##### `impl Copy for tls_crypto_info`

##### `impl Debug for tls_crypto_info`

- <span id="tls-crypto-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tls12_crypto_info_aes_gcm_128`

```rust
struct tls12_crypto_info_aes_gcm_128 {
    pub info: tls_crypto_info,
    pub iv: [c_uchar; 8],
    pub key: [c_uchar; 16],
    pub salt: [c_uchar; 4],
    pub rec_seq: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_aes_gcm_128`

- <span id="tls12-crypto-info-aes-gcm-128-clone"></span>`fn clone(&self) -> tls12_crypto_info_aes_gcm_128` — [`tls12_crypto_info_aes_gcm_128`](#tls12-crypto-info-aes-gcm-128)

##### `impl Copy for tls12_crypto_info_aes_gcm_128`

##### `impl Debug for tls12_crypto_info_aes_gcm_128`

- <span id="tls12-crypto-info-aes-gcm-128-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tls12_crypto_info_aes_gcm_256`

```rust
struct tls12_crypto_info_aes_gcm_256 {
    pub info: tls_crypto_info,
    pub iv: [c_uchar; 8],
    pub key: [c_uchar; 32],
    pub salt: [c_uchar; 4],
    pub rec_seq: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_aes_gcm_256`

- <span id="tls12-crypto-info-aes-gcm-256-clone"></span>`fn clone(&self) -> tls12_crypto_info_aes_gcm_256` — [`tls12_crypto_info_aes_gcm_256`](#tls12-crypto-info-aes-gcm-256)

##### `impl Copy for tls12_crypto_info_aes_gcm_256`

##### `impl Debug for tls12_crypto_info_aes_gcm_256`

- <span id="tls12-crypto-info-aes-gcm-256-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tls12_crypto_info_aes_ccm_128`

```rust
struct tls12_crypto_info_aes_ccm_128 {
    pub info: tls_crypto_info,
    pub iv: [c_uchar; 8],
    pub key: [c_uchar; 16],
    pub salt: [c_uchar; 4],
    pub rec_seq: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_aes_ccm_128`

- <span id="tls12-crypto-info-aes-ccm-128-clone"></span>`fn clone(&self) -> tls12_crypto_info_aes_ccm_128` — [`tls12_crypto_info_aes_ccm_128`](#tls12-crypto-info-aes-ccm-128)

##### `impl Copy for tls12_crypto_info_aes_ccm_128`

##### `impl Debug for tls12_crypto_info_aes_ccm_128`

- <span id="tls12-crypto-info-aes-ccm-128-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tls12_crypto_info_chacha20_poly1305`

```rust
struct tls12_crypto_info_chacha20_poly1305 {
    pub info: tls_crypto_info,
    pub iv: [c_uchar; 12],
    pub key: [c_uchar; 32],
    pub salt: [c_uchar; 0],
    pub rec_seq: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_chacha20_poly1305`

- <span id="tls12-crypto-info-chacha20-poly1305-clone"></span>`fn clone(&self) -> tls12_crypto_info_chacha20_poly1305` — [`tls12_crypto_info_chacha20_poly1305`](#tls12-crypto-info-chacha20-poly1305)

##### `impl Copy for tls12_crypto_info_chacha20_poly1305`

##### `impl Debug for tls12_crypto_info_chacha20_poly1305`

- <span id="tls12-crypto-info-chacha20-poly1305-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tls12_crypto_info_sm4_gcm`

```rust
struct tls12_crypto_info_sm4_gcm {
    pub info: tls_crypto_info,
    pub iv: [c_uchar; 8],
    pub key: [c_uchar; 16],
    pub salt: [c_uchar; 4],
    pub rec_seq: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_sm4_gcm`

- <span id="tls12-crypto-info-sm4-gcm-clone"></span>`fn clone(&self) -> tls12_crypto_info_sm4_gcm` — [`tls12_crypto_info_sm4_gcm`](#tls12-crypto-info-sm4-gcm)

##### `impl Copy for tls12_crypto_info_sm4_gcm`

##### `impl Debug for tls12_crypto_info_sm4_gcm`

- <span id="tls12-crypto-info-sm4-gcm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tls12_crypto_info_sm4_ccm`

```rust
struct tls12_crypto_info_sm4_ccm {
    pub info: tls_crypto_info,
    pub iv: [c_uchar; 8],
    pub key: [c_uchar; 16],
    pub salt: [c_uchar; 4],
    pub rec_seq: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_sm4_ccm`

- <span id="tls12-crypto-info-sm4-ccm-clone"></span>`fn clone(&self) -> tls12_crypto_info_sm4_ccm` — [`tls12_crypto_info_sm4_ccm`](#tls12-crypto-info-sm4-ccm)

##### `impl Copy for tls12_crypto_info_sm4_ccm`

##### `impl Debug for tls12_crypto_info_sm4_ccm`

- <span id="tls12-crypto-info-sm4-ccm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tls12_crypto_info_aria_gcm_128`

```rust
struct tls12_crypto_info_aria_gcm_128 {
    pub info: tls_crypto_info,
    pub iv: [c_uchar; 8],
    pub key: [c_uchar; 16],
    pub salt: [c_uchar; 4],
    pub rec_seq: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_aria_gcm_128`

- <span id="tls12-crypto-info-aria-gcm-128-clone"></span>`fn clone(&self) -> tls12_crypto_info_aria_gcm_128` — [`tls12_crypto_info_aria_gcm_128`](#tls12-crypto-info-aria-gcm-128)

##### `impl Copy for tls12_crypto_info_aria_gcm_128`

##### `impl Debug for tls12_crypto_info_aria_gcm_128`

- <span id="tls12-crypto-info-aria-gcm-128-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tls12_crypto_info_aria_gcm_256`

```rust
struct tls12_crypto_info_aria_gcm_256 {
    pub info: tls_crypto_info,
    pub iv: [c_uchar; 8],
    pub key: [c_uchar; 32],
    pub salt: [c_uchar; 4],
    pub rec_seq: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_aria_gcm_256`

- <span id="tls12-crypto-info-aria-gcm-256-clone"></span>`fn clone(&self) -> tls12_crypto_info_aria_gcm_256` — [`tls12_crypto_info_aria_gcm_256`](#tls12-crypto-info-aria-gcm-256)

##### `impl Copy for tls12_crypto_info_aria_gcm_256`

##### `impl Debug for tls12_crypto_info_aria_gcm_256`

- <span id="tls12-crypto-info-aria-gcm-256-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_param`

```rust
struct iw_param {
    pub value: __s32,
    pub fixed: __u8,
    pub disabled: __u8,
    pub flags: __u16,
}
```

#### Trait Implementations

##### `impl Clone for iw_param`

- <span id="iw-param-clone"></span>`fn clone(&self) -> iw_param` — [`iw_param`](#iw-param)

##### `impl Copy for iw_param`

##### `impl Debug for iw_param`

- <span id="iw-param-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_point`

```rust
struct iw_point {
    pub pointer: *mut c_void,
    pub length: __u16,
    pub flags: __u16,
}
```

#### Trait Implementations

##### `impl Clone for iw_point`

- <span id="iw-point-clone"></span>`fn clone(&self) -> iw_point` — [`iw_point`](#iw-point)

##### `impl Copy for iw_point`

##### `impl Debug for iw_point`

- <span id="iw-point-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_freq`

```rust
struct iw_freq {
    pub m: __s32,
    pub e: __s16,
    pub i: __u8,
    pub flags: __u8,
}
```

#### Trait Implementations

##### `impl Clone for iw_freq`

- <span id="iw-freq-clone"></span>`fn clone(&self) -> iw_freq` — [`iw_freq`](#iw-freq)

##### `impl Copy for iw_freq`

##### `impl Debug for iw_freq`

- <span id="iw-freq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_quality`

```rust
struct iw_quality {
    pub qual: __u8,
    pub level: __u8,
    pub noise: __u8,
    pub updated: __u8,
}
```

#### Trait Implementations

##### `impl Clone for iw_quality`

- <span id="iw-quality-clone"></span>`fn clone(&self) -> iw_quality` — [`iw_quality`](#iw-quality)

##### `impl Copy for iw_quality`

##### `impl Debug for iw_quality`

- <span id="iw-quality-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_discarded`

```rust
struct iw_discarded {
    pub nwid: __u32,
    pub code: __u32,
    pub fragment: __u32,
    pub retries: __u32,
    pubmisc: __u32,
}
```

#### Trait Implementations

##### `impl Clone for iw_discarded`

- <span id="iw-discarded-clone"></span>`fn clone(&self) -> iw_discarded` — [`iw_discarded`](#iw-discarded)

##### `impl Copy for iw_discarded`

##### `impl Debug for iw_discarded`

- <span id="iw-discarded-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_missed`

```rust
struct iw_missed {
    pub beacon: __u32,
}
```

#### Trait Implementations

##### `impl Clone for iw_missed`

- <span id="iw-missed-clone"></span>`fn clone(&self) -> iw_missed` — [`iw_missed`](#iw-missed)

##### `impl Copy for iw_missed`

##### `impl Debug for iw_missed`

- <span id="iw-missed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_scan_req`

```rust
struct iw_scan_req {
    pub scan_type: __u8,
    pub essid_len: __u8,
    pub num_channels: __u8,
    pub flags: __u8,
    pub bssid: crate::sockaddr,
    pub essid: [__u8; 32],
    pub min_channel_time: __u32,
    pub max_channel_time: __u32,
    pub channel_list: [iw_freq; 32],
}
```

#### Trait Implementations

##### `impl Clone for iw_scan_req`

- <span id="iw-scan-req-clone"></span>`fn clone(&self) -> iw_scan_req` — [`iw_scan_req`](#iw-scan-req)

##### `impl Copy for iw_scan_req`

##### `impl Debug for iw_scan_req`

- <span id="iw-scan-req-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_encode_ext`

```rust
struct iw_encode_ext {
    pub ext_flags: __u32,
    pub tx_seq: [__u8; 8],
    pub rx_seq: [__u8; 8],
    pub addr: crate::sockaddr,
    pub alg: __u16,
    pub key_len: __u16,
    pub key: [__u8; 0],
}
```

#### Trait Implementations

##### `impl Clone for iw_encode_ext`

- <span id="iw-encode-ext-clone"></span>`fn clone(&self) -> iw_encode_ext` — [`iw_encode_ext`](#iw-encode-ext)

##### `impl Copy for iw_encode_ext`

##### `impl Debug for iw_encode_ext`

- <span id="iw-encode-ext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_pmksa`

```rust
struct iw_pmksa {
    pub cmd: __u32,
    pub bssid: crate::sockaddr,
    pub pmkid: [__u8; 16],
}
```

#### Trait Implementations

##### `impl Clone for iw_pmksa`

- <span id="iw-pmksa-clone"></span>`fn clone(&self) -> iw_pmksa` — [`iw_pmksa`](#iw-pmksa)

##### `impl Copy for iw_pmksa`

##### `impl Debug for iw_pmksa`

- <span id="iw-pmksa-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_pmkid_cand`

```rust
struct iw_pmkid_cand {
    pub flags: __u32,
    pub index: __u32,
    pub bssid: crate::sockaddr,
}
```

#### Trait Implementations

##### `impl Clone for iw_pmkid_cand`

- <span id="iw-pmkid-cand-clone"></span>`fn clone(&self) -> iw_pmkid_cand` — [`iw_pmkid_cand`](#iw-pmkid-cand)

##### `impl Copy for iw_pmkid_cand`

##### `impl Debug for iw_pmkid_cand`

- <span id="iw-pmkid-cand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_statistics`

```rust
struct iw_statistics {
    pub status: __u16,
    pub qual: iw_quality,
    pub discard: iw_discarded,
    pub miss: iw_missed,
}
```

#### Trait Implementations

##### `impl Clone for iw_statistics`

- <span id="iw-statistics-clone"></span>`fn clone(&self) -> iw_statistics` — [`iw_statistics`](#iw-statistics)

##### `impl Copy for iw_statistics`

##### `impl Debug for iw_statistics`

- <span id="iw-statistics-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_range`

```rust
struct iw_range {
    pub throughput: __u32,
    pub min_nwid: __u32,
    pub max_nwid: __u32,
    pub old_num_channels: __u16,
    pub old_num_frequency: __u8,
    pub scan_capa: __u8,
    pub event_capa: [__u32; 6],
    pub sensitivity: __s32,
    pub max_qual: iw_quality,
    pub avg_qual: iw_quality,
    pub num_bitrates: __u8,
    pub bitrate: [__s32; 32],
    pub min_rts: __s32,
    pub max_rts: __s32,
    pub min_frag: __s32,
    pub max_frag: __s32,
    pub min_pmp: __s32,
    pub max_pmp: __s32,
    pub min_pmt: __s32,
    pub max_pmt: __s32,
    pub pmp_flags: __u16,
    pub pmt_flags: __u16,
    pub pm_capa: __u16,
    pub encoding_size: [__u16; 8],
    pub num_encoding_sizes: __u8,
    pub max_encoding_tokens: __u8,
    pub encoding_login_index: __u8,
    pub txpower_capa: __u16,
    pub num_txpower: __u8,
    pub txpower: [__s32; 8],
    pub we_version_compiled: __u8,
    pub we_version_source: __u8,
    pub retry_capa: __u16,
    pub retry_flags: __u16,
    pub r_time_flags: __u16,
    pub min_retry: __s32,
    pub max_retry: __s32,
    pub min_r_time: __s32,
    pub max_r_time: __s32,
    pub num_channels: __u16,
    pub num_frequency: __u8,
    pub freq: [iw_freq; 32],
    pub enc_capa: __u32,
}
```

#### Trait Implementations

##### `impl Clone for iw_range`

- <span id="iw-range-clone"></span>`fn clone(&self) -> iw_range` — [`iw_range`](#iw-range)

##### `impl Copy for iw_range`

##### `impl Debug for iw_range`

- <span id="iw-range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_priv_args`

```rust
struct iw_priv_args {
    pub cmd: __u32,
    pub set_args: __u16,
    pub get_args: __u16,
    pub name: [c_char; 16],
}
```

#### Trait Implementations

##### `impl Clone for iw_priv_args`

- <span id="iw-priv-args-clone"></span>`fn clone(&self) -> iw_priv_args` — [`iw_priv_args`](#iw-priv-args)

##### `impl Copy for iw_priv_args`

##### `impl Debug for iw_priv_args`

- <span id="iw-priv-args-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `epoll_params`

```rust
struct epoll_params {
    pub busy_poll_usecs: u32,
    pub busy_poll_budget: u16,
    pub prefer_busy_poll: u8,
    pub __pad: u8,
}
```

#### Trait Implementations

##### `impl Clone for epoll_params`

- <span id="epoll-params-clone"></span>`fn clone(&self) -> epoll_params` — [`epoll_params`](#epoll-params)

##### `impl Copy for epoll_params`

##### `impl Debug for epoll_params`

- <span id="epoll-params-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_mutexattr_t`

```rust
struct pthread_mutexattr_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_mutexattr_t`

- <span id="pthread-mutexattr-t-clone"></span>`fn clone(&self) -> pthread_mutexattr_t` — [`pthread_mutexattr_t`](#pthread-mutexattr-t)

##### `impl Copy for pthread_mutexattr_t`

##### `impl Debug for pthread_mutexattr_t`

- <span id="pthread-mutexattr-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_rwlockattr_t`

```rust
struct pthread_rwlockattr_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_rwlockattr_t`

- <span id="pthread-rwlockattr-t-clone"></span>`fn clone(&self) -> pthread_rwlockattr_t` — [`pthread_rwlockattr_t`](#pthread-rwlockattr-t)

##### `impl Copy for pthread_rwlockattr_t`

##### `impl Debug for pthread_rwlockattr_t`

- <span id="pthread-rwlockattr-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_condattr_t`

```rust
struct pthread_condattr_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_condattr_t`

- <span id="pthread-condattr-t-clone"></span>`fn clone(&self) -> pthread_condattr_t` — [`pthread_condattr_t`](#pthread-condattr-t)

##### `impl Copy for pthread_condattr_t`

##### `impl Debug for pthread_condattr_t`

- <span id="pthread-condattr-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_barrierattr_t`

```rust
struct pthread_barrierattr_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_barrierattr_t`

- <span id="pthread-barrierattr-t-clone"></span>`fn clone(&self) -> pthread_barrierattr_t` — [`pthread_barrierattr_t`](#pthread-barrierattr-t)

##### `impl Copy for pthread_barrierattr_t`

##### `impl Debug for pthread_barrierattr_t`

- <span id="pthread-barrierattr-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fanotify_event_metadata`

```rust
struct fanotify_event_metadata {
    pub event_len: __u32,
    pub vers: __u8,
    pub reserved: __u8,
    pub metadata_len: __u16,
    pub mask: __u64,
    pub fd: c_int,
    pub pid: c_int,
}
```

#### Trait Implementations

##### `impl Clone for fanotify_event_metadata`

- <span id="fanotify-event-metadata-clone"></span>`fn clone(&self) -> fanotify_event_metadata` — [`fanotify_event_metadata`](#fanotify-event-metadata)

##### `impl Copy for fanotify_event_metadata`

##### `impl Debug for fanotify_event_metadata`

- <span id="fanotify-event-metadata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptp_sys_offset`

```rust
struct ptp_sys_offset {
    pub n_samples: c_uint,
    pub rsv: [c_uint; 3],
    pub ts: [ptp_clock_time; 51],
}
```

#### Trait Implementations

##### `impl Clone for ptp_sys_offset`

- <span id="ptp-sys-offset-clone"></span>`fn clone(&self) -> ptp_sys_offset` — [`ptp_sys_offset`](#ptp-sys-offset)

##### `impl Copy for ptp_sys_offset`

##### `impl Debug for ptp_sys_offset`

- <span id="ptp-sys-offset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptp_pin_desc`

```rust
struct ptp_pin_desc {
    pub name: [c_char; 64],
    pub index: c_uint,
    pub func: c_uint,
    pub chan: c_uint,
    pub rsv: [c_uint; 5],
}
```

#### Trait Implementations

##### `impl Clone for ptp_pin_desc`

- <span id="ptp-pin-desc-clone"></span>`fn clone(&self) -> ptp_pin_desc` — [`ptp_pin_desc`](#ptp-pin-desc)

##### `impl Copy for ptp_pin_desc`

##### `impl Debug for ptp_pin_desc`

- <span id="ptp-pin-desc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptp_clock_caps`

```rust
struct ptp_clock_caps {
    pub max_adj: c_int,
    pub n_alarm: c_int,
    pub n_ext_ts: c_int,
    pub n_per_out: c_int,
    pub pps: c_int,
    pub n_pins: c_int,
    pub cross_timestamping: c_int,
    pub adjust_phase: c_int,
    pub max_phase_adj: c_int,
    pub rsv: [c_int; 11],
}
```

#### Trait Implementations

##### `impl Clone for ptp_clock_caps`

- <span id="ptp-clock-caps-clone"></span>`fn clone(&self) -> ptp_clock_caps` — [`ptp_clock_caps`](#ptp-clock-caps)

##### `impl Copy for ptp_clock_caps`

##### `impl Debug for ptp_clock_caps`

- <span id="ptp-clock-caps-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_xdp`

```rust
struct sockaddr_xdp {
    pub sxdp_family: crate::__u16,
    pub sxdp_flags: crate::__u16,
    pub sxdp_ifindex: crate::__u32,
    pub sxdp_queue_id: crate::__u32,
    pub sxdp_shared_umem_fd: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_xdp`

- <span id="sockaddr-xdp-clone"></span>`fn clone(&self) -> sockaddr_xdp` — [`sockaddr_xdp`](#sockaddr-xdp)

##### `impl Copy for sockaddr_xdp`

##### `impl Debug for sockaddr_xdp`

- <span id="sockaddr-xdp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xdp_ring_offset`

```rust
struct xdp_ring_offset {
    pub producer: crate::__u64,
    pub consumer: crate::__u64,
    pub desc: crate::__u64,
    pub flags: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for xdp_ring_offset`

- <span id="xdp-ring-offset-clone"></span>`fn clone(&self) -> xdp_ring_offset` — [`xdp_ring_offset`](#xdp-ring-offset)

##### `impl Copy for xdp_ring_offset`

##### `impl Debug for xdp_ring_offset`

- <span id="xdp-ring-offset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xdp_mmap_offsets`

```rust
struct xdp_mmap_offsets {
    pub rx: xdp_ring_offset,
    pub tx: xdp_ring_offset,
    pub fr: xdp_ring_offset,
    pub cr: xdp_ring_offset,
}
```

#### Trait Implementations

##### `impl Clone for xdp_mmap_offsets`

- <span id="xdp-mmap-offsets-clone"></span>`fn clone(&self) -> xdp_mmap_offsets` — [`xdp_mmap_offsets`](#xdp-mmap-offsets)

##### `impl Copy for xdp_mmap_offsets`

##### `impl Debug for xdp_mmap_offsets`

- <span id="xdp-mmap-offsets-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xdp_ring_offset_v1`

```rust
struct xdp_ring_offset_v1 {
    pub producer: crate::__u64,
    pub consumer: crate::__u64,
    pub desc: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for xdp_ring_offset_v1`

- <span id="xdp-ring-offset-v1-clone"></span>`fn clone(&self) -> xdp_ring_offset_v1` — [`xdp_ring_offset_v1`](#xdp-ring-offset-v1)

##### `impl Copy for xdp_ring_offset_v1`

##### `impl Debug for xdp_ring_offset_v1`

- <span id="xdp-ring-offset-v1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xdp_mmap_offsets_v1`

```rust
struct xdp_mmap_offsets_v1 {
    pub rx: xdp_ring_offset_v1,
    pub tx: xdp_ring_offset_v1,
    pub fr: xdp_ring_offset_v1,
    pub cr: xdp_ring_offset_v1,
}
```

#### Trait Implementations

##### `impl Clone for xdp_mmap_offsets_v1`

- <span id="xdp-mmap-offsets-v1-clone"></span>`fn clone(&self) -> xdp_mmap_offsets_v1` — [`xdp_mmap_offsets_v1`](#xdp-mmap-offsets-v1)

##### `impl Copy for xdp_mmap_offsets_v1`

##### `impl Debug for xdp_mmap_offsets_v1`

- <span id="xdp-mmap-offsets-v1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xdp_umem_reg`

```rust
struct xdp_umem_reg {
    pub addr: crate::__u64,
    pub len: crate::__u64,
    pub chunk_size: crate::__u32,
    pub headroom: crate::__u32,
    pub flags: crate::__u32,
    pub tx_metadata_len: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for xdp_umem_reg`

- <span id="xdp-umem-reg-clone"></span>`fn clone(&self) -> xdp_umem_reg` — [`xdp_umem_reg`](#xdp-umem-reg)

##### `impl Copy for xdp_umem_reg`

##### `impl Debug for xdp_umem_reg`

- <span id="xdp-umem-reg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xdp_umem_reg_v1`

```rust
struct xdp_umem_reg_v1 {
    pub addr: crate::__u64,
    pub len: crate::__u64,
    pub chunk_size: crate::__u32,
    pub headroom: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for xdp_umem_reg_v1`

- <span id="xdp-umem-reg-v1-clone"></span>`fn clone(&self) -> xdp_umem_reg_v1` — [`xdp_umem_reg_v1`](#xdp-umem-reg-v1)

##### `impl Copy for xdp_umem_reg_v1`

##### `impl Debug for xdp_umem_reg_v1`

- <span id="xdp-umem-reg-v1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xdp_statistics`

```rust
struct xdp_statistics {
    pub rx_dropped: crate::__u64,
    pub rx_invalid_descs: crate::__u64,
    pub tx_invalid_descs: crate::__u64,
    pub rx_ring_full: crate::__u64,
    pub rx_fill_ring_empty_descs: crate::__u64,
    pub tx_ring_empty_descs: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for xdp_statistics`

- <span id="xdp-statistics-clone"></span>`fn clone(&self) -> xdp_statistics` — [`xdp_statistics`](#xdp-statistics)

##### `impl Copy for xdp_statistics`

##### `impl Debug for xdp_statistics`

- <span id="xdp-statistics-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xdp_statistics_v1`

```rust
struct xdp_statistics_v1 {
    pub rx_dropped: crate::__u64,
    pub rx_invalid_descs: crate::__u64,
    pub tx_invalid_descs: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for xdp_statistics_v1`

- <span id="xdp-statistics-v1-clone"></span>`fn clone(&self) -> xdp_statistics_v1` — [`xdp_statistics_v1`](#xdp-statistics-v1)

##### `impl Copy for xdp_statistics_v1`

##### `impl Debug for xdp_statistics_v1`

- <span id="xdp-statistics-v1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xdp_options`

```rust
struct xdp_options {
    pub flags: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for xdp_options`

- <span id="xdp-options-clone"></span>`fn clone(&self) -> xdp_options` — [`xdp_options`](#xdp-options)

##### `impl Copy for xdp_options`

##### `impl Debug for xdp_options`

- <span id="xdp-options-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xdp_desc`

```rust
struct xdp_desc {
    pub addr: crate::__u64,
    pub len: crate::__u32,
    pub options: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for xdp_desc`

- <span id="xdp-desc-clone"></span>`fn clone(&self) -> xdp_desc` — [`xdp_desc`](#xdp-desc)

##### `impl Copy for xdp_desc`

##### `impl Debug for xdp_desc`

- <span id="xdp-desc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xsk_tx_metadata_completion`

```rust
struct xsk_tx_metadata_completion {
    pub tx_timestamp: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for xsk_tx_metadata_completion`

- <span id="xsk-tx-metadata-completion-clone"></span>`fn clone(&self) -> xsk_tx_metadata_completion` — [`xsk_tx_metadata_completion`](#xsk-tx-metadata-completion)

##### `impl Copy for xsk_tx_metadata_completion`

##### `impl Debug for xsk_tx_metadata_completion`

- <span id="xsk-tx-metadata-completion-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xsk_tx_metadata_request`

```rust
struct xsk_tx_metadata_request {
    pub csum_start: __u16,
    pub csum_offset: __u16,
}
```

#### Trait Implementations

##### `impl Clone for xsk_tx_metadata_request`

- <span id="xsk-tx-metadata-request-clone"></span>`fn clone(&self) -> xsk_tx_metadata_request` — [`xsk_tx_metadata_request`](#xsk-tx-metadata-request)

##### `impl Copy for xsk_tx_metadata_request`

##### `impl Debug for xsk_tx_metadata_request`

- <span id="xsk-tx-metadata-request-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mount_attr`

```rust
struct mount_attr {
    pub attr_set: crate::__u64,
    pub attr_clr: crate::__u64,
    pub propagation: crate::__u64,
    pub userns_fd: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for mount_attr`

- <span id="mount-attr-clone"></span>`fn clone(&self) -> mount_attr` — [`mount_attr`](#mount-attr)

##### `impl Copy for mount_attr`

##### `impl Debug for mount_attr`

- <span id="mount-attr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mnt_ns_info`

```rust
struct mnt_ns_info {
    pub size: crate::__u32,
    pub nr_mounts: crate::__u32,
    pub mnt_ns_id: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for mnt_ns_info`

- <span id="mnt-ns-info-clone"></span>`fn clone(&self) -> mnt_ns_info` — [`mnt_ns_info`](#mnt-ns-info)

##### `impl Copy for mnt_ns_info`

##### `impl Debug for mnt_ns_info`

- <span id="mnt-ns-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `dmabuf_cmsg`

```rust
struct dmabuf_cmsg {
    pub frag_offset: crate::__u64,
    pub frag_size: crate::__u32,
    pub frag_token: crate::__u32,
    pub dmabuf_id: crate::__u32,
    pub flags: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for dmabuf_cmsg`

- <span id="dmabuf-cmsg-clone"></span>`fn clone(&self) -> dmabuf_cmsg` — [`dmabuf_cmsg`](#dmabuf-cmsg)

##### `impl Copy for dmabuf_cmsg`

##### `impl Debug for dmabuf_cmsg`

- <span id="dmabuf-cmsg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `dmabuf_token`

```rust
struct dmabuf_token {
    pub token_start: crate::__u32,
    pub token_count: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for dmabuf_token`

- <span id="dmabuf-token-clone"></span>`fn clone(&self) -> dmabuf_token` — [`dmabuf_token`](#dmabuf-token)

##### `impl Copy for dmabuf_token`

##### `impl Debug for dmabuf_token`

- <span id="dmabuf-token-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_alg`

```rust
struct sockaddr_alg {
    pub salg_family: crate::sa_family_t,
    pub salg_type: [c_uchar; 14],
    pub salg_feat: u32,
    pub salg_mask: u32,
    pub salg_name: [c_uchar; 64],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_alg`

- <span id="sockaddr-alg-clone"></span>`fn clone(&self) -> sockaddr_alg` — [`sockaddr_alg`](#sockaddr-alg)

##### `impl Copy for sockaddr_alg`

##### `impl Debug for sockaddr_alg`

- <span id="sockaddr-alg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_cond_t`

```rust
struct pthread_cond_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_cond_t`

- <span id="pthread-cond-t-clone"></span>`fn clone(&self) -> pthread_cond_t` — [`pthread_cond_t`](#pthread-cond-t)

##### `impl Copy for pthread_cond_t`

##### `impl Debug for pthread_cond_t`

- <span id="pthread-cond-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_mutex_t`

```rust
struct pthread_mutex_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_mutex_t`

- <span id="pthread-mutex-t-clone"></span>`fn clone(&self) -> pthread_mutex_t` — [`pthread_mutex_t`](#pthread-mutex-t)

##### `impl Copy for pthread_mutex_t`

##### `impl Debug for pthread_mutex_t`

- <span id="pthread-mutex-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_rwlock_t`

```rust
struct pthread_rwlock_t {
    size: [u8; 56],
}
```

#### Trait Implementations

##### `impl Clone for pthread_rwlock_t`

- <span id="pthread-rwlock-t-clone"></span>`fn clone(&self) -> pthread_rwlock_t` — [`pthread_rwlock_t`](#pthread-rwlock-t)

##### `impl Copy for pthread_rwlock_t`

##### `impl Debug for pthread_rwlock_t`

- <span id="pthread-rwlock-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_barrier_t`

```rust
struct pthread_barrier_t {
    size: [u8; 32],
}
```

#### Trait Implementations

##### `impl Clone for pthread_barrier_t`

- <span id="pthread-barrier-t-clone"></span>`fn clone(&self) -> pthread_barrier_t` — [`pthread_barrier_t`](#pthread-barrier-t)

##### `impl Copy for pthread_barrier_t`

##### `impl Debug for pthread_barrier_t`

- <span id="pthread-barrier-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uinput_setup`

```rust
struct uinput_setup {
    pub id: input_id,
    pub name: [c_char; 80],
    pub ff_effects_max: __u32,
}
```

#### Trait Implementations

##### `impl Clone for uinput_setup`

- <span id="uinput-setup-clone"></span>`fn clone(&self) -> uinput_setup` — [`uinput_setup`](#uinput-setup)

##### `impl Copy for uinput_setup`

##### `impl Debug for uinput_setup`

- <span id="uinput-setup-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `uinput_user_dev`

```rust
struct uinput_user_dev {
    pub name: [c_char; 80],
    pub id: input_id,
    pub ff_effects_max: __u32,
    pub absmax: [__s32; 64],
    pub absmin: [__s32; 64],
    pub absfuzz: [__s32; 64],
    pub absflat: [__s32; 64],
}
```

#### Trait Implementations

##### `impl Clone for uinput_user_dev`

- <span id="uinput-user-dev-clone"></span>`fn clone(&self) -> uinput_user_dev` — [`uinput_user_dev`](#uinput-user-dev)

##### `impl Copy for uinput_user_dev`

##### `impl Debug for uinput_user_dev`

- <span id="uinput-user-dev-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mq_attr`

```rust
struct mq_attr {
    pub mq_flags: c_long,
    pub mq_maxmsg: c_long,
    pub mq_msgsize: c_long,
    pub mq_curmsgs: c_long,
    pad: Padding<[c_long; 4]>,
}
```

#### Trait Implementations

##### `impl Clone for mq_attr`

- <span id="mq-attr-clone"></span>`fn clone(&self) -> mq_attr` — [`mq_attr`](#mq-attr)

##### `impl Copy for mq_attr`

##### `impl Debug for mq_attr`

- <span id="mq-attr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `hwtstamp_config`

```rust
struct hwtstamp_config {
    pub flags: c_int,
    pub tx_type: c_int,
    pub rx_filter: c_int,
}
```

#### Trait Implementations

##### `impl Clone for hwtstamp_config`

- <span id="hwtstamp-config-clone"></span>`fn clone(&self) -> hwtstamp_config` — [`hwtstamp_config`](#hwtstamp-config)

##### `impl Copy for hwtstamp_config`

##### `impl Debug for hwtstamp_config`

- <span id="hwtstamp-config-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sched_attr`

```rust
struct sched_attr {
    pub size: __u32,
    pub sched_policy: __u32,
    pub sched_flags: crate::__u64,
    pub sched_nice: __s32,
    pub sched_priority: __u32,
    pub sched_runtime: crate::__u64,
    pub sched_deadline: crate::__u64,
    pub sched_period: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for sched_attr`

- <span id="sched-attr-clone"></span>`fn clone(&self) -> sched_attr` — [`sched_attr`](#sched-attr)

##### `impl Copy for sched_attr`

##### `impl Debug for sched_attr`

- <span id="sched-attr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `file_handle`

```rust
struct file_handle {
    pub handle_bytes: c_uint,
    pub handle_type: c_int,
    pub f_handle: [c_uchar; 0],
}
```

#### Trait Implementations

##### `impl Clone for file_handle`

- <span id="file-handle-clone"></span>`fn clone(&self) -> file_handle` — [`file_handle`](#file-handle)

##### `impl Copy for file_handle`

##### `impl Debug for file_handle`

- <span id="file-handle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_thrspy`

```rust
struct iw_thrspy {
    pub addr: crate::sockaddr,
    pub qual: iw_quality,
    pub low: iw_quality,
    pub high: iw_quality,
}
```

#### Trait Implementations

##### `impl Clone for iw_thrspy`

- <span id="iw-thrspy-clone"></span>`fn clone(&self) -> iw_thrspy` — [`iw_thrspy`](#iw-thrspy)

##### `impl Copy for iw_thrspy`

##### `impl Debug for iw_thrspy`

- <span id="iw-thrspy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_mlme`

```rust
struct iw_mlme {
    pub cmd: __u16,
    pub reason_code: __u16,
    pub addr: crate::sockaddr,
}
```

#### Trait Implementations

##### `impl Clone for iw_mlme`

- <span id="iw-mlme-clone"></span>`fn clone(&self) -> iw_mlme` — [`iw_mlme`](#iw-mlme)

##### `impl Copy for iw_mlme`

##### `impl Debug for iw_mlme`

- <span id="iw-mlme-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_michaelmicfailure`

```rust
struct iw_michaelmicfailure {
    pub flags: __u32,
    pub src_addr: crate::sockaddr,
    pub tsc: [__u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for iw_michaelmicfailure`

- <span id="iw-michaelmicfailure-clone"></span>`fn clone(&self) -> iw_michaelmicfailure` — [`iw_michaelmicfailure`](#iw-michaelmicfailure)

##### `impl Copy for iw_michaelmicfailure`

##### `impl Debug for iw_michaelmicfailure`

- <span id="iw-michaelmicfailure-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `af_alg_iv`

```rust
struct af_alg_iv {
    pub ivlen: u32,
    pub iv: [c_uchar; 0],
}
```

WARNING: The `PartialEq`, `Eq` and `Hash` implementations of this
type are unsound and will be removed in the future.

#### Trait Implementations

##### `impl Clone for af_alg_iv`

- <span id="af-alg-iv-clone"></span>`fn clone(&self) -> af_alg_iv` — [`af_alg_iv`](#af-alg-iv)

##### `impl Copy for af_alg_iv`

##### `impl Debug for af_alg_iv`

- <span id="af-alg-iv-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_block_desc`

```rust
struct tpacket_block_desc {
    pub version: __u32,
    pub offset_to_priv: __u32,
    pub hdr: crate::tpacket_bd_header_u,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_block_desc`

- <span id="tpacket-block-desc-clone"></span>`fn clone(&self) -> tpacket_block_desc` — [`tpacket_block_desc`](#tpacket-block-desc)

##### `impl Copy for tpacket_block_desc`

##### `impl Debug for tpacket_block_desc`

- <span id="tpacket-block-desc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sock_txtime`

```rust
struct sock_txtime {
    pub clockid: crate::clockid_t,
    pub flags: __u32,
}
```

#### Trait Implementations

##### `impl Clone for sock_txtime`

- <span id="sock-txtime-clone"></span>`fn clone(&self) -> sock_txtime` — [`sock_txtime`](#sock-txtime)

##### `impl Copy for sock_txtime`

##### `impl Debug for sock_txtime`

- <span id="sock-txtime-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iw_event`

```rust
struct iw_event {
    pub len: __u16,
    pub cmd: __u16,
    pub u: iwreq_data,
}
```

#### Trait Implementations

##### `impl Clone for iw_event`

- <span id="iw-event-clone"></span>`fn clone(&self) -> iw_event` — [`iw_event`](#iw-event)

##### `impl Copy for iw_event`

##### `impl Debug for iw_event`

- <span id="iw-event-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iwreq`

```rust
struct iwreq {
    pub ifr_ifrn: __c_anonymous_iwreq,
    pub u: iwreq_data,
}
```

#### Trait Implementations

##### `impl Clone for iwreq`

- <span id="iwreq-clone"></span>`fn clone(&self) -> iwreq` — [`iwreq`](#iwreq)

##### `impl Copy for iwreq`

##### `impl Debug for iwreq`

- <span id="iwreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptp_perout_request`

```rust
struct ptp_perout_request {
    pub anonymous_1: __c_anonymous_ptp_perout_request_1,
    pub period: ptp_clock_time,
    pub index: c_uint,
    pub flags: c_uint,
    pub anonymous_2: __c_anonymous_ptp_perout_request_2,
}
```

#### Trait Implementations

##### `impl Clone for ptp_perout_request`

- <span id="ptp-perout-request-clone"></span>`fn clone(&self) -> ptp_perout_request` — [`ptp_perout_request`](#ptp-perout-request)

##### `impl Copy for ptp_perout_request`

##### `impl Debug for ptp_perout_request`

- <span id="ptp-perout-request-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `xsk_tx_metadata`

```rust
struct xsk_tx_metadata {
    pub flags: crate::__u64,
    pub xsk_tx_metadata_union: __c_anonymous_xsk_tx_metadata_union,
}
```

#### Trait Implementations

##### `impl Clone for xsk_tx_metadata`

- <span id="xsk-tx-metadata-clone"></span>`fn clone(&self) -> xsk_tx_metadata` — [`xsk_tx_metadata`](#xsk-tx-metadata)

##### `impl Copy for xsk_tx_metadata`

##### `impl Debug for xsk_tx_metadata`

- <span id="xsk-tx-metadata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `glob_t`

```rust
struct glob_t {
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

##### `impl Clone for glob_t`

- <span id="glob-t-clone"></span>`fn clone(&self) -> glob_t` — [`glob_t`](#glob-t)

##### `impl Copy for glob_t`

##### `impl Debug for glob_t`

- <span id="glob-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `passwd`

```rust
struct passwd {
    pub pw_name: *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid: crate::uid_t,
    pub pw_gid: crate::gid_t,
    pub pw_gecos: *mut c_char,
    pub pw_dir: *mut c_char,
    pub pw_shell: *mut c_char,
}
```

#### Trait Implementations

##### `impl Clone for passwd`

- <span id="passwd-clone"></span>`fn clone(&self) -> passwd` — [`passwd`](#passwd)

##### `impl Copy for passwd`

##### `impl Debug for passwd`

- <span id="passwd-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `spwd`

```rust
struct spwd {
    pub sp_namp: *mut c_char,
    pub sp_pwdp: *mut c_char,
    pub sp_lstchg: c_long,
    pub sp_min: c_long,
    pub sp_max: c_long,
    pub sp_warn: c_long,
    pub sp_inact: c_long,
    pub sp_expire: c_long,
    pub sp_flag: c_ulong,
}
```

#### Trait Implementations

##### `impl Clone for spwd`

- <span id="spwd-clone"></span>`fn clone(&self) -> spwd` — [`spwd`](#spwd)

##### `impl Copy for spwd`

##### `impl Debug for spwd`

- <span id="spwd-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `itimerspec`

```rust
struct itimerspec {
    pub it_interval: crate::timespec,
    pub it_value: crate::timespec,
}
```

#### Trait Implementations

##### `impl Clone for itimerspec`

- <span id="itimerspec-clone"></span>`fn clone(&self) -> itimerspec` — [`itimerspec`](#itimerspec)

##### `impl Copy for itimerspec`

##### `impl Debug for itimerspec`

- <span id="itimerspec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fsid_t`

```rust
struct fsid_t {
    __val: [c_int; 2],
}
```

#### Trait Implementations

##### `impl Clone for fsid_t`

- <span id="fsid-t-clone"></span>`fn clone(&self) -> fsid_t` — [`fsid_t`](#fsid-t)

##### `impl Copy for fsid_t`

##### `impl Debug for fsid_t`

- <span id="fsid-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `packet_mreq`

```rust
struct packet_mreq {
    pub mr_ifindex: c_int,
    pub mr_type: c_ushort,
    pub mr_alen: c_ushort,
    pub mr_address: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for packet_mreq`

- <span id="packet-mreq-clone"></span>`fn clone(&self) -> packet_mreq` — [`packet_mreq`](#packet-mreq)

##### `impl Copy for packet_mreq`

##### `impl Debug for packet_mreq`

- <span id="packet-mreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `cpu_set_t`

```rust
struct cpu_set_t {
    bits: [u64; 16],
}
```

#### Trait Implementations

##### `impl Clone for cpu_set_t`

- <span id="cpu-set-t-clone"></span>`fn clone(&self) -> cpu_set_t` — [`cpu_set_t`](#cpu-set-t)

##### `impl Copy for cpu_set_t`

##### `impl Debug for cpu_set_t`

- <span id="cpu-set-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sembuf`

```rust
struct sembuf {
    pub sem_num: c_ushort,
    pub sem_op: c_short,
    pub sem_flg: c_short,
}
```

#### Trait Implementations

##### `impl Clone for sembuf`

- <span id="sembuf-clone"></span>`fn clone(&self) -> sembuf` — [`sembuf`](#sembuf)

##### `impl Copy for sembuf`

##### `impl Debug for sembuf`

- <span id="sembuf-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `dl_phdr_info`

```rust
struct dl_phdr_info {
    pub dlpi_addr: Elf64_Addr,
    pub dlpi_name: *const c_char,
    pub dlpi_phdr: *const Elf64_Phdr,
    pub dlpi_phnum: Elf64_Half,
    pub dlpi_adds: c_ulonglong,
    pub dlpi_subs: c_ulonglong,
    pub dlpi_tls_modid: size_t,
    pub dlpi_tls_data: *mut c_void,
}
```

#### Trait Implementations

##### `impl Clone for dl_phdr_info`

- <span id="dl-phdr-info-clone"></span>`fn clone(&self) -> dl_phdr_info` — [`dl_phdr_info`](#dl-phdr-info)

##### `impl Copy for dl_phdr_info`

##### `impl Debug for dl_phdr_info`

- <span id="dl-phdr-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Elf32_Ehdr`

```rust
struct Elf32_Ehdr {
    pub e_ident: [c_uchar; 16],
    pub e_type: Elf32_Half,
    pub e_machine: Elf32_Half,
    pub e_version: Elf32_Word,
    pub e_entry: Elf32_Addr,
    pub e_phoff: Elf32_Off,
    pub e_shoff: Elf32_Off,
    pub e_flags: Elf32_Word,
    pub e_ehsize: Elf32_Half,
    pub e_phentsize: Elf32_Half,
    pub e_phnum: Elf32_Half,
    pub e_shentsize: Elf32_Half,
    pub e_shnum: Elf32_Half,
    pub e_shstrndx: Elf32_Half,
}
```

#### Trait Implementations

##### `impl Clone for Elf32_Ehdr`

- <span id="elf32-ehdr-clone"></span>`fn clone(&self) -> Elf32_Ehdr` — [`Elf32_Ehdr`](#elf32-ehdr)

##### `impl Copy for Elf32_Ehdr`

##### `impl Debug for Elf32_Ehdr`

- <span id="elf32-ehdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Elf64_Ehdr`

```rust
struct Elf64_Ehdr {
    pub e_ident: [c_uchar; 16],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}
```

#### Trait Implementations

##### `impl Clone for Elf64_Ehdr`

- <span id="elf64-ehdr-clone"></span>`fn clone(&self) -> Elf64_Ehdr` — [`Elf64_Ehdr`](#elf64-ehdr)

##### `impl Copy for Elf64_Ehdr`

##### `impl Debug for Elf64_Ehdr`

- <span id="elf64-ehdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Elf32_Sym`

```rust
struct Elf32_Sym {
    pub st_name: Elf32_Word,
    pub st_value: Elf32_Addr,
    pub st_size: Elf32_Word,
    pub st_info: c_uchar,
    pub st_other: c_uchar,
    pub st_shndx: Elf32_Section,
}
```

#### Trait Implementations

##### `impl Clone for Elf32_Sym`

- <span id="elf32-sym-clone"></span>`fn clone(&self) -> Elf32_Sym` — [`Elf32_Sym`](#elf32-sym)

##### `impl Copy for Elf32_Sym`

##### `impl Debug for Elf32_Sym`

- <span id="elf32-sym-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Elf64_Sym`

```rust
struct Elf64_Sym {
    pub st_name: Elf64_Word,
    pub st_info: c_uchar,
    pub st_other: c_uchar,
    pub st_shndx: Elf64_Section,
    pub st_value: Elf64_Addr,
    pub st_size: Elf64_Xword,
}
```

#### Trait Implementations

##### `impl Clone for Elf64_Sym`

- <span id="elf64-sym-clone"></span>`fn clone(&self) -> Elf64_Sym` — [`Elf64_Sym`](#elf64-sym)

##### `impl Copy for Elf64_Sym`

##### `impl Debug for Elf64_Sym`

- <span id="elf64-sym-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Elf32_Phdr`

```rust
struct Elf32_Phdr {
    pub p_type: Elf32_Word,
    pub p_offset: Elf32_Off,
    pub p_vaddr: Elf32_Addr,
    pub p_paddr: Elf32_Addr,
    pub p_filesz: Elf32_Word,
    pub p_memsz: Elf32_Word,
    pub p_flags: Elf32_Word,
    pub p_align: Elf32_Word,
}
```

#### Trait Implementations

##### `impl Clone for Elf32_Phdr`

- <span id="elf32-phdr-clone"></span>`fn clone(&self) -> Elf32_Phdr` — [`Elf32_Phdr`](#elf32-phdr)

##### `impl Copy for Elf32_Phdr`

##### `impl Debug for Elf32_Phdr`

- <span id="elf32-phdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Elf64_Phdr`

```rust
struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}
```

#### Trait Implementations

##### `impl Clone for Elf64_Phdr`

- <span id="elf64-phdr-clone"></span>`fn clone(&self) -> Elf64_Phdr` — [`Elf64_Phdr`](#elf64-phdr)

##### `impl Copy for Elf64_Phdr`

##### `impl Debug for Elf64_Phdr`

- <span id="elf64-phdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Elf32_Shdr`

```rust
struct Elf32_Shdr {
    pub sh_name: Elf32_Word,
    pub sh_type: Elf32_Word,
    pub sh_flags: Elf32_Word,
    pub sh_addr: Elf32_Addr,
    pub sh_offset: Elf32_Off,
    pub sh_size: Elf32_Word,
    pub sh_link: Elf32_Word,
    pub sh_info: Elf32_Word,
    pub sh_addralign: Elf32_Word,
    pub sh_entsize: Elf32_Word,
}
```

#### Trait Implementations

##### `impl Clone for Elf32_Shdr`

- <span id="elf32-shdr-clone"></span>`fn clone(&self) -> Elf32_Shdr` — [`Elf32_Shdr`](#elf32-shdr)

##### `impl Copy for Elf32_Shdr`

##### `impl Debug for Elf32_Shdr`

- <span id="elf32-shdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Elf64_Shdr`

```rust
struct Elf64_Shdr {
    pub sh_name: Elf64_Word,
    pub sh_type: Elf64_Word,
    pub sh_flags: Elf64_Xword,
    pub sh_addr: Elf64_Addr,
    pub sh_offset: Elf64_Off,
    pub sh_size: Elf64_Xword,
    pub sh_link: Elf64_Word,
    pub sh_info: Elf64_Word,
    pub sh_addralign: Elf64_Xword,
    pub sh_entsize: Elf64_Xword,
}
```

#### Trait Implementations

##### `impl Clone for Elf64_Shdr`

- <span id="elf64-shdr-clone"></span>`fn clone(&self) -> Elf64_Shdr` — [`Elf64_Shdr`](#elf64-shdr)

##### `impl Copy for Elf64_Shdr`

##### `impl Debug for Elf64_Shdr`

- <span id="elf64-shdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_elf32_rel`

```rust
struct __c_anonymous_elf32_rel {
    pub r_offset: Elf32_Addr,
    pub r_info: Elf32_Word,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_elf32_rel`

- <span id="c-anonymous-elf32-rel-clone"></span>`fn clone(&self) -> __c_anonymous_elf32_rel` — [`__c_anonymous_elf32_rel`](#c-anonymous-elf32-rel)

##### `impl Copy for __c_anonymous_elf32_rel`

##### `impl Debug for __c_anonymous_elf32_rel`

- <span id="c-anonymous-elf32-rel-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_elf64_rel`

```rust
struct __c_anonymous_elf64_rel {
    pub r_offset: Elf64_Addr,
    pub r_info: Elf64_Xword,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_elf64_rel`

- <span id="c-anonymous-elf64-rel-clone"></span>`fn clone(&self) -> __c_anonymous_elf64_rel` — [`__c_anonymous_elf64_rel`](#c-anonymous-elf64-rel)

##### `impl Copy for __c_anonymous_elf64_rel`

##### `impl Debug for __c_anonymous_elf64_rel`

- <span id="c-anonymous-elf64-rel-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ucred`

```rust
struct ucred {
    pub pid: crate::pid_t,
    pub uid: crate::uid_t,
    pub gid: crate::gid_t,
}
```

#### Trait Implementations

##### `impl Clone for ucred`

- <span id="ucred-clone"></span>`fn clone(&self) -> ucred` — [`ucred`](#ucred)

##### `impl Copy for ucred`

##### `impl Debug for ucred`

- <span id="ucred-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mntent`

```rust
struct mntent {
    pub mnt_fsname: *mut c_char,
    pub mnt_dir: *mut c_char,
    pub mnt_type: *mut c_char,
    pub mnt_opts: *mut c_char,
    pub mnt_freq: c_int,
    pub mnt_passno: c_int,
}
```

#### Trait Implementations

##### `impl Clone for mntent`

- <span id="mntent-clone"></span>`fn clone(&self) -> mntent` — [`mntent`](#mntent)

##### `impl Copy for mntent`

##### `impl Debug for mntent`

- <span id="mntent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in6_pktinfo`

```rust
struct in6_pktinfo {
    pub ipi6_addr: crate::in6_addr,
    pub ipi6_ifindex: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for in6_pktinfo`

- <span id="in6-pktinfo-clone"></span>`fn clone(&self) -> in6_pktinfo` — [`in6_pktinfo`](#in6-pktinfo)

##### `impl Copy for in6_pktinfo`

##### `impl Debug for in6_pktinfo`

- <span id="in6-pktinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `arpd_request`

```rust
struct arpd_request {
    pub req: c_ushort,
    pub ip: u32,
    pub dev: c_ulong,
    pub stamp: c_ulong,
    pub updated: c_ulong,
    pub ha: [c_uchar; 7],
}
```

#### Trait Implementations

##### `impl Clone for arpd_request`

- <span id="arpd-request-clone"></span>`fn clone(&self) -> arpd_request` — [`arpd_request`](#arpd-request)

##### `impl Copy for arpd_request`

##### `impl Debug for arpd_request`

- <span id="arpd-request-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `regmatch_t`

```rust
struct regmatch_t {
    pub rm_so: crate::regoff_t,
    pub rm_eo: crate::regoff_t,
}
```

#### Trait Implementations

##### `impl Clone for regmatch_t`

- <span id="regmatch-t-clone"></span>`fn clone(&self) -> regmatch_t` — [`regmatch_t`](#regmatch-t)

##### `impl Copy for regmatch_t`

##### `impl Debug for regmatch_t`

- <span id="regmatch-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `option`

```rust
struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}
```

#### Trait Implementations

##### `impl Clone for option`

- <span id="option-clone"></span>`fn clone(&self) -> option` — [`option`](#option)

##### `impl Copy for option`

##### `impl Debug for option`

- <span id="option-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rlimit64`

```rust
struct rlimit64 {
    pub rlim_cur: crate::rlim64_t,
    pub rlim_max: crate::rlim64_t,
}
```

#### Trait Implementations

##### `impl Clone for rlimit64`

- <span id="rlimit64-clone"></span>`fn clone(&self) -> rlimit64` — [`rlimit64`](#rlimit64)

##### `impl Copy for rlimit64`

##### `impl Debug for rlimit64`

- <span id="rlimit64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_ifru_map`

```rust
struct __c_anonymous_ifru_map {
    pub mem_start: c_ulong,
    pub mem_end: c_ulong,
    pub base_addr: c_ushort,
    pub irq: c_uchar,
    pub dma: c_uchar,
    pub port: c_uchar,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_ifru_map`

- <span id="c-anonymous-ifru-map-clone"></span>`fn clone(&self) -> __c_anonymous_ifru_map` — [`__c_anonymous_ifru_map`](#c-anonymous-ifru-map)

##### `impl Copy for __c_anonymous_ifru_map`

##### `impl Debug for __c_anonymous_ifru_map`

- <span id="c-anonymous-ifru-map-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `dirent`

```rust
struct dirent {
    pub d_ino: crate::ino_t,
    pub d_off: crate::off_t,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
}
```

#### Trait Implementations

##### `impl Clone for dirent`

- <span id="dirent-clone"></span>`fn clone(&self) -> dirent` — [`dirent`](#dirent)

##### `impl Copy for dirent`

##### `impl Debug for dirent`

- <span id="dirent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `dirent64`

```rust
struct dirent64 {
    pub d_ino: crate::ino64_t,
    pub d_off: crate::off64_t,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
}
```

#### Trait Implementations

##### `impl Clone for dirent64`

- <span id="dirent64-clone"></span>`fn clone(&self) -> dirent64` — [`dirent64`](#dirent64)

##### `impl Copy for dirent64`

##### `impl Debug for dirent64`

- <span id="dirent64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_elf32_rela`

```rust
struct __c_anonymous_elf32_rela {
    pub r_offset: crate::Elf32_Addr,
    pub r_info: crate::Elf32_Word,
    pub r_addend: crate::Elf32_Sword,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_elf32_rela`

- <span id="c-anonymous-elf32-rela-clone"></span>`fn clone(&self) -> __c_anonymous_elf32_rela` — [`__c_anonymous_elf32_rela`](#c-anonymous-elf32-rela)

##### `impl Copy for __c_anonymous_elf32_rela`

##### `impl Debug for __c_anonymous_elf32_rela`

- <span id="c-anonymous-elf32-rela-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_elf64_rela`

```rust
struct __c_anonymous_elf64_rela {
    pub r_offset: crate::Elf64_Addr,
    pub r_info: crate::Elf64_Xword,
    pub r_addend: crate::Elf64_Sxword,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_elf64_rela`

- <span id="c-anonymous-elf64-rela-clone"></span>`fn clone(&self) -> __c_anonymous_elf64_rela` — [`__c_anonymous_elf64_rela`](#c-anonymous-elf64-rela)

##### `impl Copy for __c_anonymous_elf64_rela`

##### `impl Debug for __c_anonymous_elf64_rela`

- <span id="c-anonymous-elf64-rela-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ifreq`

```rust
struct ifreq {
    pub ifr_name: [c_char; 16],
    pub ifr_ifru: __c_anonymous_ifr_ifru,
}
```

#### Fields

- **`ifr_name`**: `[c_char; 16]`

  interface name, e.g. "en0"

#### Trait Implementations

##### `impl Clone for ifreq`

- <span id="ifreq-clone"></span>`fn clone(&self) -> ifreq` — [`ifreq`](#ifreq)

##### `impl Copy for ifreq`

##### `impl Debug for ifreq`

- <span id="ifreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ifconf`

```rust
struct ifconf {
    pub ifc_len: c_int,
    pub ifc_ifcu: __c_anonymous_ifc_ifcu,
}
```

Structure used in SIOCGIFCONF request.  Used to retrieve interface configuration for
machine (useful for programs which must know all networks accessible).

#### Fields

- **`ifc_len`**: `c_int`

  Size of buffer

#### Trait Implementations

##### `impl Clone for ifconf`

- <span id="ifconf-clone"></span>`fn clone(&self) -> ifconf` — [`ifconf`](#ifconf)

##### `impl Copy for ifconf`

##### `impl Debug for ifconf`

- <span id="ifconf-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `timezone`

```rust
enum timezone {
}
```

#### Trait Implementations

##### `impl Clone for timezone`

- <span id="timezone-clone"></span>`fn clone(&self) -> timezone` — [`timezone`](../index.md#timezone)

##### `impl Copy for timezone`

##### `impl Debug for timezone`

- <span id="timezone-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tpacket_versions`

```rust
enum tpacket_versions {
    TPACKET_V1,
    TPACKET_V2,
    TPACKET_V3,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_versions`

- <span id="tpacket-versions-clone"></span>`fn clone(&self) -> tpacket_versions` — [`tpacket_versions`](#tpacket-versions)

##### `impl Copy for tpacket_versions`

##### `impl Debug for tpacket_versions`

- <span id="tpacket-versions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `ioctl`

```rust
unsafe fn ioctl(fd: c_int, request: c_ulong) -> c_int
```

### `sem_destroy`

```rust
unsafe fn sem_destroy(sem: *mut sem_t) -> c_int
```

### `sem_init`

```rust
unsafe fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int
```

### `fdatasync`

```rust
unsafe fn fdatasync(fd: c_int) -> c_int
```

### `mincore`

```rust
unsafe fn mincore(addr: *mut c_void, len: size_t, vec: *mut c_uchar) -> c_int
```

### `clock_getres`

```rust
unsafe fn clock_getres(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int
```

### `clock_gettime`

```rust
unsafe fn clock_gettime(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int
```

### `clock_settime`

```rust
unsafe fn clock_settime(clk_id: crate::clockid_t, tp: *const crate::timespec) -> c_int
```

### `clock_getcpuclockid`

```rust
unsafe fn clock_getcpuclockid(pid: crate::pid_t, clk_id: *mut crate::clockid_t) -> c_int
```

### `getitimer`

```rust
unsafe fn getitimer(which: c_int, curr_value: *mut crate::itimerval) -> c_int
```

### `setitimer`

```rust
unsafe fn setitimer(which: c_int, new_value: *const crate::itimerval, old_value: *mut crate::itimerval) -> c_int
```

### `dirfd`

```rust
unsafe fn dirfd(dirp: *mut crate::DIR) -> c_int
```

### `memalign`

```rust
unsafe fn memalign(align: size_t, size: size_t) -> *mut c_void
```

### `setgroups`

```rust
unsafe fn setgroups(ngroups: size_t, ptr: *const crate::gid_t) -> c_int
```

### `pipe2`

```rust
unsafe fn pipe2(fds: *mut c_int, flags: c_int) -> c_int
```

### `statfs`

```rust
unsafe fn statfs(path: *const c_char, buf: *mut statfs) -> c_int
```

### `fstatfs`

```rust
unsafe fn fstatfs(fd: c_int, buf: *mut statfs) -> c_int
```

### `memrchr`

```rust
unsafe fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

### `posix_fadvise`

```rust
unsafe fn posix_fadvise(fd: c_int, offset: off_t, len: off_t, advise: c_int) -> c_int
```

### `futimens`

```rust
unsafe fn futimens(fd: c_int, times: *const crate::timespec) -> c_int
```

### `utimensat`

```rust
unsafe fn utimensat(dirfd: c_int, path: *const c_char, times: *const crate::timespec, flag: c_int) -> c_int
```

### `duplocale`

```rust
unsafe fn duplocale(base: crate::locale_t) -> crate::locale_t
```

### `freelocale`

```rust
unsafe fn freelocale(loc: crate::locale_t)
```

### `newlocale`

```rust
unsafe fn newlocale(mask: c_int, locale: *const c_char, base: crate::locale_t) -> crate::locale_t
```

### `uselocale`

```rust
unsafe fn uselocale(loc: crate::locale_t) -> crate::locale_t
```

### `mknodat`

```rust
unsafe fn mknodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, dev: dev_t) -> c_int
```

### `ptsname_r`

```rust
unsafe fn ptsname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```

### `clearenv`

```rust
unsafe fn clearenv() -> c_int
```

### `waitid`

```rust
unsafe fn waitid(idtype: idtype_t, id: id_t, infop: *mut crate::siginfo_t, options: c_int) -> c_int
```

### `getresuid`

```rust
unsafe fn getresuid(ruid: *mut crate::uid_t, euid: *mut crate::uid_t, suid: *mut crate::uid_t) -> c_int
```

### `getresgid`

```rust
unsafe fn getresgid(rgid: *mut crate::gid_t, egid: *mut crate::gid_t, sgid: *mut crate::gid_t) -> c_int
```

### `acct`

```rust
unsafe fn acct(filename: *const c_char) -> c_int
```

### `brk`

```rust
unsafe fn brk(addr: *mut c_void) -> c_int
```

### `sbrk`

```rust
unsafe fn sbrk(increment: intptr_t) -> *mut c_void
```

### `vfork`

```rust
unsafe fn vfork() -> crate::pid_t
```

### `setresgid`

```rust
unsafe fn setresgid(rgid: crate::gid_t, egid: crate::gid_t, sgid: crate::gid_t) -> c_int
```

### `setresuid`

```rust
unsafe fn setresuid(ruid: crate::uid_t, euid: crate::uid_t, suid: crate::uid_t) -> c_int
```

### `wait4`

```rust
unsafe fn wait4(pid: crate::pid_t, status: *mut c_int, options: c_int, rusage: *mut crate::rusage) -> crate::pid_t
```

### `login_tty`

```rust
unsafe fn login_tty(fd: c_int) -> c_int
```

### `execvpe`

```rust
unsafe fn execvpe(file: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

### `fexecve`

```rust
unsafe fn fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

### `getifaddrs`

```rust
unsafe fn getifaddrs(ifap: *mut *mut crate::ifaddrs) -> c_int
```

### `freeifaddrs`

```rust
unsafe fn freeifaddrs(ifa: *mut crate::ifaddrs)
```

### `bind`

```rust
unsafe fn bind(socket: c_int, address: *const crate::sockaddr, address_len: crate::socklen_t) -> c_int
```

### `writev`

```rust
unsafe fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t
```

### `readv`

```rust
unsafe fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t
```

### `sendmsg`

```rust
unsafe fn sendmsg(fd: c_int, msg: *const crate::msghdr, flags: c_int) -> ssize_t
```

### `recvmsg`

```rust
unsafe fn recvmsg(fd: c_int, msg: *mut crate::msghdr, flags: c_int) -> ssize_t
```

### `uname`

```rust
unsafe fn uname(buf: *mut crate::utsname) -> c_int
```

### `strchrnul`

```rust
unsafe fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char
```

### `strftime`

```rust
unsafe fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const crate::tm) -> size_t
```

### `strftime_l`

```rust
unsafe fn strftime_l(s: *mut c_char, max: size_t, format: *const c_char, tm: *const crate::tm, locale: crate::locale_t) -> size_t
```

### `strptime`

```rust
unsafe fn strptime(s: *const c_char, format: *const c_char, tm: *mut crate::tm) -> *mut c_char
```

### `mkostemp`

```rust
unsafe fn mkostemp(template: *mut c_char, flags: c_int) -> c_int
```

### `mkostemps`

```rust
unsafe fn mkostemps(template: *mut c_char, suffixlen: c_int, flags: c_int) -> c_int
```

### `getdomainname`

```rust
unsafe fn getdomainname(name: *mut c_char, len: size_t) -> c_int
```

### `setdomainname`

```rust
unsafe fn setdomainname(name: *const c_char, len: size_t) -> c_int
```

### `if_nameindex`

```rust
unsafe fn if_nameindex() -> *mut if_nameindex
```

### `if_freenameindex`

```rust
unsafe fn if_freenameindex(ptr: *mut if_nameindex)
```

### `getpwnam_r`

```rust
unsafe fn getpwnam_r(name: *const c_char, pwd: *mut passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut passwd) -> c_int
```

### `getpwuid_r`

```rust
unsafe fn getpwuid_r(uid: crate::uid_t, pwd: *mut passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut passwd) -> c_int
```

### `fstatfs64`

```rust
unsafe fn fstatfs64(fd: c_int, buf: *mut statfs64) -> c_int
```

### `statvfs64`

```rust
unsafe fn statvfs64(path: *const c_char, buf: *mut statvfs64) -> c_int
```

### `fstatvfs64`

```rust
unsafe fn fstatvfs64(fd: c_int, buf: *mut statvfs64) -> c_int
```

### `statfs64`

```rust
unsafe fn statfs64(path: *const c_char, buf: *mut statfs64) -> c_int
```

### `creat64`

```rust
unsafe fn creat64(path: *const c_char, mode: mode_t) -> c_int
```

### `fstat64`

```rust
unsafe fn fstat64(fildes: c_int, buf: *mut stat64) -> c_int
```

### `fstatat64`

```rust
unsafe fn fstatat64(dirfd: c_int, pathname: *const c_char, buf: *mut stat64, flags: c_int) -> c_int
```

### `ftruncate64`

```rust
unsafe fn ftruncate64(fd: c_int, length: off64_t) -> c_int
```

### `lseek64`

```rust
unsafe fn lseek64(fd: c_int, offset: off64_t, whence: c_int) -> off64_t
```

### `lstat64`

```rust
unsafe fn lstat64(path: *const c_char, buf: *mut stat64) -> c_int
```

### `mmap64`

```rust
unsafe fn mmap64(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off64_t) -> *mut c_void
```

### `open64`

```rust
unsafe fn open64(path: *const c_char, oflag: c_int) -> c_int
```

### `openat64`

```rust
unsafe fn openat64(fd: c_int, path: *const c_char, oflag: c_int) -> c_int
```

### `posix_fadvise64`

```rust
unsafe fn posix_fadvise64(fd: c_int, offset: off64_t, len: off64_t, advise: c_int) -> c_int
```

### `pread64`

```rust
unsafe fn pread64(fd: c_int, buf: *mut c_void, count: size_t, offset: off64_t) -> ssize_t
```

### `pwrite64`

```rust
unsafe fn pwrite64(fd: c_int, buf: *const c_void, count: size_t, offset: off64_t) -> ssize_t
```

### `readdir64`

```rust
unsafe fn readdir64(dirp: *mut crate::DIR) -> *mut crate::dirent64
```

### `readdir64_r`

```rust
unsafe fn readdir64_r(dirp: *mut crate::DIR, entry: *mut crate::dirent64, result: *mut *mut crate::dirent64) -> c_int
```

### `stat64`

```rust
unsafe fn stat64(path: *const c_char, buf: *mut stat64) -> c_int
```

### `truncate64`

```rust
unsafe fn truncate64(path: *const c_char, length: off64_t) -> c_int
```

### `preadv64`

```rust
unsafe fn preadv64(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t) -> ssize_t
```

### `pwritev64`

```rust
unsafe fn pwritev64(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t) -> ssize_t
```

### `forkpty`

```rust
unsafe fn forkpty(amaster: *mut c_int, name: *mut c_char, termp: *const termios, winp: *const crate::winsize) -> crate::pid_t
```

### `openpty`

```rust
unsafe fn openpty(amaster: *mut c_int, aslave: *mut c_int, name: *mut c_char, termp: *const termios, winp: *const crate::winsize) -> c_int
```

### `statx`

```rust
unsafe fn statx(dirfd: c_int, pathname: *const c_char, flags: c_int, mask: c_uint, statxbuf: *mut statx) -> c_int
```

### `_IOC`

```rust
const fn _IOC(dir: u32, ty: u32, nr: u32, size: usize) -> c_ulong
```

Build an ioctl number, analogous to the C macro of the same name.

### `_IO`

```rust
const fn _IO(ty: u32, nr: u32) -> c_ulong
```

Build an ioctl number for an argumentless ioctl.

### `_IOR`

```rust
const fn _IOR<T>(ty: u32, nr: u32) -> c_ulong
```

Build an ioctl number for an read-only ioctl.

### `_IOW`

```rust
const fn _IOW<T>(ty: u32, nr: u32) -> c_ulong
```

Build an ioctl number for an write-only ioctl.

### `_IOWR`

```rust
const fn _IOWR<T>(ty: u32, nr: u32) -> c_ulong
```

Build an ioctl number for a read-write ioctl.

### `CMSG_ALIGN`

```rust
const fn CMSG_ALIGN(len: usize) -> usize
```

### `CMSG_FIRSTHDR`

```rust
unsafe fn CMSG_FIRSTHDR(mhdr: *const crate::msghdr) -> *mut crate::cmsghdr
```

### `CMSG_DATA`

```rust
unsafe fn CMSG_DATA(cmsg: *const crate::cmsghdr) -> *mut c_uchar
```

### `CMSG_SPACE`

```rust
const unsafe fn CMSG_SPACE(length: c_uint) -> c_uint
```

### `CMSG_LEN`

```rust
const unsafe fn CMSG_LEN(length: c_uint) -> c_uint
```

### `FD_CLR`

```rust
unsafe fn FD_CLR(fd: c_int, set: *mut fd_set)
```

### `FD_ISSET`

```rust
unsafe fn FD_ISSET(fd: c_int, set: *const fd_set) -> bool
```

### `FD_SET`

```rust
unsafe fn FD_SET(fd: c_int, set: *mut fd_set)
```

### `FD_ZERO`

```rust
unsafe fn FD_ZERO(set: *mut fd_set)
```

### `SIGRTMAX`

```rust
fn SIGRTMAX() -> c_int
```

### `SIGRTMIN`

```rust
fn SIGRTMIN() -> c_int
```

### `WIFSTOPPED`

```rust
const fn WIFSTOPPED(status: c_int) -> bool
```

### `WSTOPSIG`

```rust
const fn WSTOPSIG(status: c_int) -> c_int
```

### `WIFCONTINUED`

```rust
const fn WIFCONTINUED(status: c_int) -> bool
```

### `WIFSIGNALED`

```rust
const fn WIFSIGNALED(status: c_int) -> bool
```

### `WTERMSIG`

```rust
const fn WTERMSIG(status: c_int) -> c_int
```

### `WIFEXITED`

```rust
const fn WIFEXITED(status: c_int) -> bool
```

### `WEXITSTATUS`

```rust
const fn WEXITSTATUS(status: c_int) -> c_int
```

### `WCOREDUMP`

```rust
const fn WCOREDUMP(status: c_int) -> bool
```

### `W_EXITCODE`

```rust
const fn W_EXITCODE(ret: c_int, sig: c_int) -> c_int
```

### `W_STOPCODE`

```rust
const fn W_STOPCODE(sig: c_int) -> c_int
```

### `QCMD`

```rust
const fn QCMD(cmd: c_int, type_: c_int) -> c_int
```

### `IPOPT_COPIED`

```rust
const fn IPOPT_COPIED(o: u8) -> u8
```

### `IPOPT_CLASS`

```rust
const fn IPOPT_CLASS(o: u8) -> u8
```

### `IPOPT_NUMBER`

```rust
const fn IPOPT_NUMBER(o: u8) -> u8
```

### `IPTOS_ECN`

```rust
const fn IPTOS_ECN(x: u8) -> u8
```

### `KERNEL_VERSION`

```rust
const fn KERNEL_VERSION(a: u32, b: u32, c: u32) -> u32
```

### `getspnam_r`

```rust
unsafe fn getspnam_r(name: *const c_char, spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```

### `mq_open`

```rust
unsafe fn mq_open(name: *const c_char, oflag: c_int) -> mqd_t
```

### `mq_close`

```rust
unsafe fn mq_close(mqd: mqd_t) -> c_int
```

### `mq_unlink`

```rust
unsafe fn mq_unlink(name: *const c_char) -> c_int
```

### `mq_receive`

```rust
unsafe fn mq_receive(mqd: mqd_t, msg_ptr: *mut c_char, msg_len: size_t, msg_prio: *mut c_uint) -> ssize_t
```

### `mq_timedreceive`

```rust
unsafe fn mq_timedreceive(mqd: mqd_t, msg_ptr: *mut c_char, msg_len: size_t, msg_prio: *mut c_uint, abs_timeout: *const crate::timespec) -> ssize_t
```

### `mq_send`

```rust
unsafe fn mq_send(mqd: mqd_t, msg_ptr: *const c_char, msg_len: size_t, msg_prio: c_uint) -> c_int
```

### `mq_timedsend`

```rust
unsafe fn mq_timedsend(mqd: mqd_t, msg_ptr: *const c_char, msg_len: size_t, msg_prio: c_uint, abs_timeout: *const crate::timespec) -> c_int
```

### `mq_getattr`

```rust
unsafe fn mq_getattr(mqd: mqd_t, attr: *mut crate::mq_attr) -> c_int
```

### `mq_setattr`

```rust
unsafe fn mq_setattr(mqd: mqd_t, newattr: *const crate::mq_attr, oldattr: *mut crate::mq_attr) -> c_int
```

### `mrand48`

```rust
unsafe fn mrand48() -> c_long
```

### `seed48`

```rust
unsafe fn seed48(xseed: *mut c_ushort) -> *mut c_ushort
```

### `lcong48`

```rust
unsafe fn lcong48(p: *mut c_ushort)
```

### `lutimes`

```rust
unsafe fn lutimes(file: *const c_char, times: *const crate::timeval) -> c_int
```

### `shm_open`

```rust
unsafe fn shm_open(name: *const c_char, oflag: c_int, mode: mode_t) -> c_int
```

### `shm_unlink`

```rust
unsafe fn shm_unlink(name: *const c_char) -> c_int
```

### `ftok`

```rust
unsafe fn ftok(pathname: *const c_char, proj_id: c_int) -> crate::key_t
```

### `semget`

```rust
unsafe fn semget(key: crate::key_t, nsems: c_int, semflag: c_int) -> c_int
```

### `semop`

```rust
unsafe fn semop(semid: c_int, sops: *mut crate::sembuf, nsops: size_t) -> c_int
```

### `semctl`

```rust
unsafe fn semctl(semid: c_int, semnum: c_int, cmd: c_int) -> c_int
```

### `msgctl`

```rust
unsafe fn msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds) -> c_int
```

### `msgget`

```rust
unsafe fn msgget(key: crate::key_t, msgflg: c_int) -> c_int
```

### `msgrcv`

```rust
unsafe fn msgrcv(msqid: c_int, msgp: *mut c_void, msgsz: size_t, msgtyp: c_long, msgflg: c_int) -> ssize_t
```

### `msgsnd`

```rust
unsafe fn msgsnd(msqid: c_int, msgp: *const c_void, msgsz: size_t, msgflg: c_int) -> c_int
```

### `fallocate`

```rust
unsafe fn fallocate(fd: c_int, mode: c_int, offset: off_t, len: off_t) -> c_int
```

### `posix_fallocate`

```rust
unsafe fn posix_fallocate(fd: c_int, offset: off_t, len: off_t) -> c_int
```

### `readahead`

```rust
unsafe fn readahead(fd: c_int, offset: off64_t, count: size_t) -> ssize_t
```

### `getxattr`

```rust
unsafe fn getxattr(path: *const c_char, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t
```

### `lgetxattr`

```rust
unsafe fn lgetxattr(path: *const c_char, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t
```

### `fgetxattr`

```rust
unsafe fn fgetxattr(filedes: c_int, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t
```

### `setxattr`

```rust
unsafe fn setxattr(path: *const c_char, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int
```

### `lsetxattr`

```rust
unsafe fn lsetxattr(path: *const c_char, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int
```

### `fsetxattr`

```rust
unsafe fn fsetxattr(filedes: c_int, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int
```

### `listxattr`

```rust
unsafe fn listxattr(path: *const c_char, list: *mut c_char, size: size_t) -> ssize_t
```

### `llistxattr`

```rust
unsafe fn llistxattr(path: *const c_char, list: *mut c_char, size: size_t) -> ssize_t
```

### `flistxattr`

```rust
unsafe fn flistxattr(filedes: c_int, list: *mut c_char, size: size_t) -> ssize_t
```

### `removexattr`

```rust
unsafe fn removexattr(path: *const c_char, name: *const c_char) -> c_int
```

### `lremovexattr`

```rust
unsafe fn lremovexattr(path: *const c_char, name: *const c_char) -> c_int
```

### `fremovexattr`

```rust
unsafe fn fremovexattr(filedes: c_int, name: *const c_char) -> c_int
```

### `signalfd`

```rust
unsafe fn signalfd(fd: c_int, mask: *const crate::sigset_t, flags: c_int) -> c_int
```

### `timerfd_create`

```rust
unsafe fn timerfd_create(clockid: crate::clockid_t, flags: c_int) -> c_int
```

### `timerfd_gettime`

```rust
unsafe fn timerfd_gettime(fd: c_int, curr_value: *mut crate::itimerspec) -> c_int
```

### `timerfd_settime`

```rust
unsafe fn timerfd_settime(fd: c_int, flags: c_int, new_value: *const crate::itimerspec, old_value: *mut crate::itimerspec) -> c_int
```

### `quotactl`

```rust
unsafe fn quotactl(cmd: c_int, special: *const c_char, id: c_int, data: *mut c_char) -> c_int
```

### `epoll_pwait`

```rust
unsafe fn epoll_pwait(epfd: c_int, events: *mut crate::epoll_event, maxevents: c_int, timeout: c_int, sigmask: *const crate::sigset_t) -> c_int
```

### `dup3`

```rust
unsafe fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int
```

### `sigtimedwait`

```rust
unsafe fn sigtimedwait(set: *const sigset_t, info: *mut siginfo_t, timeout: *const crate::timespec) -> c_int
```

### `sigwaitinfo`

```rust
unsafe fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> c_int
```

### `accept4`

```rust
unsafe fn accept4(fd: c_int, addr: *mut crate::sockaddr, len: *mut socklen_t, flg: c_int) -> c_int
```

### `reboot`

```rust
unsafe fn reboot(how_to: c_int) -> c_int
```

### `setfsgid`

```rust
unsafe fn setfsgid(gid: crate::gid_t) -> c_int
```

### `setfsuid`

```rust
unsafe fn setfsuid(uid: crate::uid_t) -> c_int
```

### `mkfifoat`

```rust
unsafe fn mkfifoat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int
```

### `sync_file_range`

```rust
unsafe fn sync_file_range(fd: c_int, offset: off64_t, nbytes: off64_t, flags: c_uint) -> c_int
```

### `posix_madvise`

```rust
unsafe fn posix_madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int
```

### `remap_file_pages`

```rust
unsafe fn remap_file_pages(addr: *mut c_void, size: size_t, prot: c_int, pgoff: size_t, flags: c_int) -> c_int
```

### `mkstemps`

```rust
unsafe fn mkstemps(template: *mut c_char, suffixlen: c_int) -> c_int
```

### `vhangup`

```rust
unsafe fn vhangup() -> c_int
```

### `sync`

```rust
unsafe fn sync()
```

### `syncfs`

```rust
unsafe fn syncfs(fd: c_int) -> c_int
```

### `syscall`

```rust
unsafe fn syscall(num: c_long) -> c_long
```

### `sched_setaffinity`

```rust
unsafe fn sched_setaffinity(pid: crate::pid_t, cpusetsize: size_t, cpuset: *const crate::cpu_set_t) -> c_int
```

### `epoll_create`

```rust
unsafe fn epoll_create(size: c_int) -> c_int
```

### `epoll_create1`

```rust
unsafe fn epoll_create1(flags: c_int) -> c_int
```

### `epoll_wait`

```rust
unsafe fn epoll_wait(epfd: c_int, events: *mut crate::epoll_event, maxevents: c_int, timeout: c_int) -> c_int
```

### `epoll_ctl`

```rust
unsafe fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut crate::epoll_event) -> c_int
```

### `unshare`

```rust
unsafe fn unshare(flags: c_int) -> c_int
```

### `umount`

```rust
unsafe fn umount(target: *const c_char) -> c_int
```

### `tee`

```rust
unsafe fn tee(fd_in: c_int, fd_out: c_int, len: size_t, flags: c_uint) -> ssize_t
```

### `splice`

```rust
unsafe fn splice(fd_in: c_int, off_in: *mut loff_t, fd_out: c_int, off_out: *mut loff_t, len: size_t, flags: c_uint) -> ssize_t
```

### `eventfd`

```rust
unsafe fn eventfd(initval: c_uint, flags: c_int) -> c_int
```

### `eventfd_read`

```rust
unsafe fn eventfd_read(fd: c_int, value: *mut eventfd_t) -> c_int
```

### `eventfd_write`

```rust
unsafe fn eventfd_write(fd: c_int, value: eventfd_t) -> c_int
```

### `sched_rr_get_interval`

```rust
unsafe fn sched_rr_get_interval(pid: crate::pid_t, tp: *mut crate::timespec) -> c_int
```

### `sched_setparam`

```rust
unsafe fn sched_setparam(pid: crate::pid_t, param: *const crate::sched_param) -> c_int
```

### `setns`

```rust
unsafe fn setns(fd: c_int, nstype: c_int) -> c_int
```

### `swapoff`

```rust
unsafe fn swapoff(path: *const c_char) -> c_int
```

### `vmsplice`

```rust
unsafe fn vmsplice(fd: c_int, iov: *const crate::iovec, nr_segs: size_t, flags: c_uint) -> ssize_t
```

### `personality`

```rust
unsafe fn personality(persona: c_ulong) -> c_int
```

### `sched_getparam`

```rust
unsafe fn sched_getparam(pid: crate::pid_t, param: *mut crate::sched_param) -> c_int
```

### `clone`

```rust
unsafe fn clone(cb: fn(*mut c_void) -> c_int, child_stack: *mut c_void, flags: c_int, arg: *mut c_void) -> c_int
```

### `sched_getscheduler`

```rust
unsafe fn sched_getscheduler(pid: crate::pid_t) -> c_int
```

### `clock_nanosleep`

```rust
unsafe fn clock_nanosleep(clk_id: crate::clockid_t, flags: c_int, rqtp: *const crate::timespec, rmtp: *mut crate::timespec) -> c_int
```

### `umount2`

```rust
unsafe fn umount2(target: *const c_char, flags: c_int) -> c_int
```

### `swapon`

```rust
unsafe fn swapon(path: *const c_char, swapflags: c_int) -> c_int
```

### `sched_setscheduler`

```rust
unsafe fn sched_setscheduler(pid: crate::pid_t, policy: c_int, param: *const crate::sched_param) -> c_int
```

### `sendfile`

```rust
unsafe fn sendfile(out_fd: c_int, in_fd: c_int, offset: *mut off_t, count: size_t) -> ssize_t
```

### `sigaltstack`

```rust
unsafe fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> c_int
```

### `getdtablesize`

```rust
unsafe fn getdtablesize() -> c_int
```

### `getgrouplist`

```rust
unsafe fn getgrouplist(user: *const c_char, group: crate::gid_t, groups: *mut crate::gid_t, ngroups: *mut c_int) -> c_int
```

### `posix_spawn`

```rust
unsafe fn posix_spawn(pid: *mut crate::pid_t, path: *const c_char, file_actions: *const crate::posix_spawn_file_actions_t, attrp: *const crate::posix_spawnattr_t, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int
```

### `posix_spawnp`

```rust
unsafe fn posix_spawnp(pid: *mut crate::pid_t, file: *const c_char, file_actions: *const crate::posix_spawn_file_actions_t, attrp: *const crate::posix_spawnattr_t, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int
```

### `posix_spawnattr_init`

```rust
unsafe fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> c_int
```

### `posix_spawnattr_destroy`

```rust
unsafe fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> c_int
```

### `posix_spawnattr_getsigdefault`

```rust
unsafe fn posix_spawnattr_getsigdefault(attr: *const posix_spawnattr_t, default: *mut crate::sigset_t) -> c_int
```

### `posix_spawnattr_setsigdefault`

```rust
unsafe fn posix_spawnattr_setsigdefault(attr: *mut posix_spawnattr_t, default: *const crate::sigset_t) -> c_int
```

### `posix_spawnattr_getsigmask`

```rust
unsafe fn posix_spawnattr_getsigmask(attr: *const posix_spawnattr_t, default: *mut crate::sigset_t) -> c_int
```

### `posix_spawnattr_setsigmask`

```rust
unsafe fn posix_spawnattr_setsigmask(attr: *mut posix_spawnattr_t, default: *const crate::sigset_t) -> c_int
```

### `posix_spawnattr_getflags`

```rust
unsafe fn posix_spawnattr_getflags(attr: *const posix_spawnattr_t, flags: *mut c_short) -> c_int
```

### `posix_spawnattr_setflags`

```rust
unsafe fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t, flags: c_short) -> c_int
```

### `posix_spawnattr_getpgroup`

```rust
unsafe fn posix_spawnattr_getpgroup(attr: *const posix_spawnattr_t, flags: *mut crate::pid_t) -> c_int
```

### `posix_spawnattr_setpgroup`

```rust
unsafe fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t, flags: crate::pid_t) -> c_int
```

### `posix_spawnattr_getschedpolicy`

```rust
unsafe fn posix_spawnattr_getschedpolicy(attr: *const posix_spawnattr_t, flags: *mut c_int) -> c_int
```

### `posix_spawnattr_setschedpolicy`

```rust
unsafe fn posix_spawnattr_setschedpolicy(attr: *mut posix_spawnattr_t, flags: c_int) -> c_int
```

### `posix_spawnattr_getschedparam`

```rust
unsafe fn posix_spawnattr_getschedparam(attr: *const posix_spawnattr_t, param: *mut crate::sched_param) -> c_int
```

### `posix_spawnattr_setschedparam`

```rust
unsafe fn posix_spawnattr_setschedparam(attr: *mut posix_spawnattr_t, param: *const crate::sched_param) -> c_int
```

### `posix_spawn_file_actions_init`

```rust
unsafe fn posix_spawn_file_actions_init(actions: *mut posix_spawn_file_actions_t) -> c_int
```

### `posix_spawn_file_actions_destroy`

```rust
unsafe fn posix_spawn_file_actions_destroy(actions: *mut posix_spawn_file_actions_t) -> c_int
```

### `posix_spawn_file_actions_addopen`

```rust
unsafe fn posix_spawn_file_actions_addopen(actions: *mut posix_spawn_file_actions_t, fd: c_int, path: *const c_char, oflag: c_int, mode: mode_t) -> c_int
```

### `posix_spawn_file_actions_addclose`

```rust
unsafe fn posix_spawn_file_actions_addclose(actions: *mut posix_spawn_file_actions_t, fd: c_int) -> c_int
```

### `posix_spawn_file_actions_adddup2`

```rust
unsafe fn posix_spawn_file_actions_adddup2(actions: *mut posix_spawn_file_actions_t, fd: c_int, newfd: c_int) -> c_int
```

### `fread_unlocked`

```rust
unsafe fn fread_unlocked(buf: *mut c_void, size: size_t, nobj: size_t, stream: *mut crate::FILE) -> size_t
```

### `inotify_rm_watch`

```rust
unsafe fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int
```

### `inotify_init`

```rust
unsafe fn inotify_init() -> c_int
```

### `inotify_init1`

```rust
unsafe fn inotify_init1(flags: c_int) -> c_int
```

### `inotify_add_watch`

```rust
unsafe fn inotify_add_watch(fd: c_int, path: *const c_char, mask: u32) -> c_int
```

### `fanotify_init`

```rust
unsafe fn fanotify_init(flags: c_uint, event_f_flags: c_uint) -> c_int
```

### `gethostid`

```rust
unsafe fn gethostid() -> c_long
```

### `klogctl`

```rust
unsafe fn klogctl(syslog_type: c_int, bufp: *mut c_char, len: c_int) -> c_int
```

### `name_to_handle_at`

```rust
unsafe fn name_to_handle_at(dirfd: c_int, path: *const c_char, handle: *mut file_handle, mount_id: *mut c_int, flags: c_int) -> c_int
```

### `open_by_handle_at`

```rust
unsafe fn open_by_handle_at(mount_fd: c_int, handle: *mut file_handle, flags: c_int) -> c_int
```

### `fallocate64`

```rust
unsafe fn fallocate64(fd: c_int, mode: c_int, offset: off64_t, len: off64_t) -> c_int
```

### `fgetpos64`

```rust
unsafe fn fgetpos64(stream: *mut crate::FILE, ptr: *mut crate::fpos64_t) -> c_int
```

### `fopen64`

```rust
unsafe fn fopen64(filename: *const c_char, mode: *const c_char) -> *mut crate::FILE
```

### `posix_fallocate64`

```rust
unsafe fn posix_fallocate64(fd: c_int, offset: off64_t, len: off64_t) -> c_int
```

### `sendfile64`

```rust
unsafe fn sendfile64(out_fd: c_int, in_fd: c_int, offset: *mut off64_t, count: size_t) -> ssize_t
```

### `tmpfile64`

```rust
unsafe fn tmpfile64() -> *mut crate::FILE
```

### `issecure_mask`

```rust
const fn issecure_mask(x: c_int) -> c_int
```

### `FUTEX_OP`

```rust
fn FUTEX_OP(op: c_int, oparg: c_int, cmp: c_int, cmparg: c_int) -> c_int
```

### `SCTP_PR_INDEX`

```rust
unsafe fn SCTP_PR_INDEX(policy: c_int) -> c_int
```

### `SCTP_PR_POLICY`

```rust
unsafe fn SCTP_PR_POLICY(policy: c_int) -> c_int
```

### `SCTP_PR_SET_POLICY`

```rust
unsafe fn SCTP_PR_SET_POLICY(flags: &mut c_int, policy: c_int)
```

### `SO_EE_OFFENDER`

```rust
unsafe fn SO_EE_OFFENDER(ee: *const crate::sock_extended_err) -> *mut crate::sockaddr
```

### `TPACKET_ALIGN`

```rust
unsafe fn TPACKET_ALIGN(x: usize) -> usize
```

### `BPF_CLASS`

```rust
unsafe fn BPF_CLASS(code: __u32) -> __u32
```

### `BPF_SIZE`

```rust
unsafe fn BPF_SIZE(code: __u32) -> __u32
```

### `BPF_MODE`

```rust
unsafe fn BPF_MODE(code: __u32) -> __u32
```

### `BPF_OP`

```rust
unsafe fn BPF_OP(code: __u32) -> __u32
```

### `BPF_SRC`

```rust
unsafe fn BPF_SRC(code: __u32) -> __u32
```

### `BPF_RVAL`

```rust
unsafe fn BPF_RVAL(code: __u32) -> __u32
```

### `BPF_MISCOP`

```rust
unsafe fn BPF_MISCOP(code: __u32) -> __u32
```

### `BPF_STMT`

```rust
unsafe fn BPF_STMT(code: __u16, k: __u32) -> crate::sock_filter
```

### `BPF_JUMP`

```rust
unsafe fn BPF_JUMP(code: __u16, k: __u32, jt: __u8, jf: __u8) -> crate::sock_filter
```

### `SUN_LEN`

```rust
unsafe fn SUN_LEN(s: crate::sockaddr_un) -> usize
```

### `SCTP_PR_TTL_ENABLED`

```rust
const fn SCTP_PR_TTL_ENABLED(policy: c_int) -> bool
```

### `SCTP_PR_RTX_ENABLED`

```rust
const fn SCTP_PR_RTX_ENABLED(policy: c_int) -> bool
```

### `SCTP_PR_PRIO_ENABLED`

```rust
const fn SCTP_PR_PRIO_ENABLED(policy: c_int) -> bool
```

### `iopl`

```rust
unsafe fn iopl(level: c_int) -> c_int
```

### `ioperm`

```rust
unsafe fn ioperm(from: c_ulong, num: c_ulong, turn_on: c_int) -> c_int
```

### `aio_read`

```rust
unsafe fn aio_read(aiocbp: *mut crate::aiocb) -> c_int
```

### `aio_write`

```rust
unsafe fn aio_write(aiocbp: *mut crate::aiocb) -> c_int
```

### `aio_fsync`

```rust
unsafe fn aio_fsync(op: c_int, aiocbp: *mut crate::aiocb) -> c_int
```

### `aio_error`

```rust
unsafe fn aio_error(aiocbp: *const crate::aiocb) -> c_int
```

### `aio_return`

```rust
unsafe fn aio_return(aiocbp: *mut crate::aiocb) -> ssize_t
```

### `aio_suspend`

```rust
unsafe fn aio_suspend(aiocb_list: *const *const crate::aiocb, nitems: c_int, timeout: *const crate::timespec) -> c_int
```

### `aio_cancel`

```rust
unsafe fn aio_cancel(fd: c_int, aiocbp: *mut crate::aiocb) -> c_int
```

### `lio_listio`

```rust
unsafe fn lio_listio(mode: c_int, aiocb_list: *const *mut crate::aiocb, nitems: c_int, sevp: *mut crate::sigevent) -> c_int
```

### `pwritev`

```rust
unsafe fn pwritev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: crate::off_t) -> ssize_t
```

### `preadv`

```rust
unsafe fn preadv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: crate::off_t) -> ssize_t
```

### `getnameinfo`

```rust
unsafe fn getnameinfo(sa: *const crate::sockaddr, salen: crate::socklen_t, host: *mut c_char, hostlen: crate::socklen_t, serv: *mut c_char, servlen: crate::socklen_t, flags: c_int) -> c_int
```

### `getloadavg`

```rust
unsafe fn getloadavg(loadavg: *mut c_double, nelem: c_int) -> c_int
```

### `process_vm_readv`

```rust
unsafe fn process_vm_readv(pid: crate::pid_t, local_iov: *const crate::iovec, liovcnt: c_ulong, remote_iov: *const crate::iovec, riovcnt: c_ulong, flags: c_ulong) -> isize
```

### `process_vm_writev`

```rust
unsafe fn process_vm_writev(pid: crate::pid_t, local_iov: *const crate::iovec, liovcnt: c_ulong, remote_iov: *const crate::iovec, riovcnt: c_ulong, flags: c_ulong) -> isize
```

### `futimes`

```rust
unsafe fn futimes(fd: c_int, times: *const crate::timeval) -> c_int
```

### `strerror_r`

```rust
unsafe fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```

### `abs`

```rust
unsafe fn abs(i: c_int) -> c_int
```

### `labs`

```rust
unsafe fn labs(i: c_long) -> c_long
```

### `rand`

```rust
unsafe fn rand() -> c_int
```

### `srand`

```rust
unsafe fn srand(seed: c_uint)
```

### `drand48`

```rust
unsafe fn drand48() -> c_double
```

### `erand48`

```rust
unsafe fn erand48(xseed: *mut c_ushort) -> c_double
```

### `lrand48`

```rust
unsafe fn lrand48() -> c_long
```

### `nrand48`

```rust
unsafe fn nrand48(xseed: *mut c_ushort) -> c_long
```

### `jrand48`

```rust
unsafe fn jrand48(xseed: *mut c_ushort) -> c_long
```

### `srand48`

```rust
unsafe fn srand48(seed: c_long)
```

### `setpwent`

```rust
unsafe fn setpwent()
```

### `endpwent`

```rust
unsafe fn endpwent()
```

### `getpwent`

```rust
unsafe fn getpwent() -> *mut passwd
```

### `setgrent`

```rust
unsafe fn setgrent()
```

### `endgrent`

```rust
unsafe fn endgrent()
```

### `getgrent`

```rust
unsafe fn getgrent() -> *mut crate::group
```

### `setspent`

```rust
unsafe fn setspent()
```

### `endspent`

```rust
unsafe fn endspent()
```

### `getspent`

```rust
unsafe fn getspent() -> *mut spwd
```

### `getspnam`

```rust
unsafe fn getspnam(name: *const c_char) -> *mut spwd
```

### `shmget`

```rust
unsafe fn shmget(key: crate::key_t, size: size_t, shmflg: c_int) -> c_int
```

### `shmat`

```rust
unsafe fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void
```

### `shmdt`

```rust
unsafe fn shmdt(shmaddr: *const c_void) -> c_int
```

### `shmctl`

```rust
unsafe fn shmctl(shmid: c_int, cmd: c_int, buf: *mut crate::shmid_ds) -> c_int
```

### `mprotect`

```rust
unsafe fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int
```

### `__errno_location`

```rust
unsafe fn __errno_location() -> *mut c_int
```

### `mremap`

```rust
unsafe fn mremap(addr: *mut c_void, len: size_t, new_len: size_t, flags: c_int) -> *mut c_void
```

### `glob`

```rust
unsafe fn glob(pattern: *const c_char, flags: c_int, errfunc: Option<fn(*const c_char, c_int) -> c_int>, pglob: *mut crate::glob_t) -> c_int
```

### `globfree`

```rust
unsafe fn globfree(pglob: *mut crate::glob_t)
```

### `seekdir`

```rust
unsafe fn seekdir(dirp: *mut crate::DIR, loc: c_long)
```

### `telldir`

```rust
unsafe fn telldir(dirp: *mut crate::DIR) -> c_long
```

### `madvise`

```rust
unsafe fn madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int
```

### `msync`

```rust
unsafe fn msync(addr: *mut c_void, len: size_t, flags: c_int) -> c_int
```

### `recvfrom`

```rust
unsafe fn recvfrom(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int, addr: *mut crate::sockaddr, addrlen: *mut crate::socklen_t) -> ssize_t
```

### `nl_langinfo`

```rust
unsafe fn nl_langinfo(item: crate::nl_item) -> *mut c_char
```

### `nl_langinfo_l`

```rust
unsafe fn nl_langinfo_l(item: crate::nl_item, locale: crate::locale_t) -> *mut c_char
```

### `sched_getaffinity`

```rust
unsafe fn sched_getaffinity(pid: crate::pid_t, cpusetsize: size_t, cpuset: *mut cpu_set_t) -> c_int
```

### `sched_get_priority_max`

```rust
unsafe fn sched_get_priority_max(policy: c_int) -> c_int
```

### `settimeofday`

```rust
unsafe fn settimeofday(tv: *const crate::timeval, tz: *const crate::timezone) -> c_int
```

### `sem_timedwait`

```rust
unsafe fn sem_timedwait(sem: *mut crate::sem_t, abstime: *const crate::timespec) -> c_int
```

### `sem_getvalue`

```rust
unsafe fn sem_getvalue(sem: *mut crate::sem_t, sval: *mut c_int) -> c_int
```

### `mount`

```rust
unsafe fn mount(src: *const c_char, target: *const c_char, fstype: *const c_char, flags: c_ulong, data: *const c_void) -> c_int
```

### `prctl`

```rust
unsafe fn prctl(option: c_int) -> c_int
```

### `ppoll`

```rust
unsafe fn ppoll(fds: *mut crate::pollfd, nfds: crate::nfds_t, timeout: *const crate::timespec, sigmask: *const crate::sigset_t) -> c_int
```

### `sethostname`

```rust
unsafe fn sethostname(name: *const c_char, len: size_t) -> c_int
```

### `sched_get_priority_min`

```rust
unsafe fn sched_get_priority_min(policy: c_int) -> c_int
```

### `sysinfo`

```rust
unsafe fn sysinfo(info: *mut crate::sysinfo) -> c_int
```

### `sigsuspend`

```rust
unsafe fn sigsuspend(mask: *const crate::sigset_t) -> c_int
```

### `getgrgid_r`

```rust
unsafe fn getgrgid_r(gid: crate::gid_t, grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```

### `sem_close`

```rust
unsafe fn sem_close(sem: *mut crate::sem_t) -> c_int
```

### `getgrnam_r`

```rust
unsafe fn getgrnam_r(name: *const c_char, grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```

### `initgroups`

```rust
unsafe fn initgroups(user: *const c_char, group: crate::gid_t) -> c_int
```

### `sem_open`

```rust
unsafe fn sem_open(name: *const c_char, oflag: c_int) -> *mut crate::sem_t
```

### `getgrnam`

```rust
unsafe fn getgrnam(name: *const c_char) -> *mut crate::group
```

### `sem_unlink`

```rust
unsafe fn sem_unlink(name: *const c_char) -> c_int
```

### `daemon`

```rust
unsafe fn daemon(nochdir: c_int, noclose: c_int) -> c_int
```

### `sigwait`

```rust
unsafe fn sigwait(set: *const crate::sigset_t, sig: *mut c_int) -> c_int
```

### `getgrgid`

```rust
unsafe fn getgrgid(gid: crate::gid_t) -> *mut crate::group
```

### `popen`

```rust
unsafe fn popen(command: *const c_char, mode: *const c_char) -> *mut crate::FILE
```

### `faccessat`

```rust
unsafe fn faccessat(dirfd: c_int, pathname: *const c_char, mode: c_int, flags: c_int) -> c_int
```

### `dl_iterate_phdr`

```rust
unsafe fn dl_iterate_phdr(callback: Option<fn(*mut crate::dl_phdr_info, size_t, *mut c_void) -> c_int>, data: *mut c_void) -> c_int
```

### `setmntent`

```rust
unsafe fn setmntent(filename: *const c_char, ty: *const c_char) -> *mut crate::FILE
```

### `getmntent`

```rust
unsafe fn getmntent(stream: *mut crate::FILE) -> *mut crate::mntent
```

### `addmntent`

```rust
unsafe fn addmntent(stream: *mut crate::FILE, mnt: *const crate::mntent) -> c_int
```

### `endmntent`

```rust
unsafe fn endmntent(streamp: *mut crate::FILE) -> c_int
```

### `hasmntopt`

```rust
unsafe fn hasmntopt(mnt: *const crate::mntent, opt: *const c_char) -> *mut c_char
```

### `regcomp`

```rust
unsafe fn regcomp(preg: *mut crate::regex_t, pattern: *const c_char, cflags: c_int) -> c_int
```

### `regexec`

```rust
unsafe fn regexec(preg: *const crate::regex_t, input: *const c_char, nmatch: size_t, pmatch: *mut regmatch_t, eflags: c_int) -> c_int
```

### `regerror`

```rust
unsafe fn regerror(errcode: c_int, preg: *const crate::regex_t, errbuf: *mut c_char, errbuf_size: size_t) -> size_t
```

### `regfree`

```rust
unsafe fn regfree(preg: *mut crate::regex_t)
```

### `iconv_open`

```rust
unsafe fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> iconv_t
```

### `iconv`

```rust
unsafe fn iconv(cd: iconv_t, inbuf: *mut *mut c_char, inbytesleft: *mut size_t, outbuf: *mut *mut c_char, outbytesleft: *mut size_t) -> size_t
```

### `iconv_close`

```rust
unsafe fn iconv_close(cd: iconv_t) -> c_int
```

### `gettid`

```rust
unsafe fn gettid() -> crate::pid_t
```

### `timer_create`

```rust
unsafe fn timer_create(clockid: crate::clockid_t, sevp: *mut crate::sigevent, timerid: *mut crate::timer_t) -> c_int
```

### `timer_delete`

```rust
unsafe fn timer_delete(timerid: crate::timer_t) -> c_int
```

### `timer_getoverrun`

```rust
unsafe fn timer_getoverrun(timerid: crate::timer_t) -> c_int
```

### `timer_gettime`

```rust
unsafe fn timer_gettime(timerid: crate::timer_t, curr_value: *mut crate::itimerspec) -> c_int
```

### `timer_settime`

```rust
unsafe fn timer_settime(timerid: crate::timer_t, flags: c_int, new_value: *const crate::itimerspec, old_value: *mut crate::itimerspec) -> c_int
```

### `memmem`

```rust
unsafe fn memmem(haystack: *const c_void, haystacklen: size_t, needle: *const c_void, needlelen: size_t) -> *mut c_void
```

### `sched_getcpu`

```rust
unsafe fn sched_getcpu() -> c_int
```

### `getopt_long`

```rust
unsafe fn getopt_long(argc: c_int, argv: *const *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int
```

### `copy_file_range`

```rust
unsafe fn copy_file_range(fd_in: c_int, off_in: *mut crate::off64_t, fd_out: c_int, off_out: *mut crate::off64_t, len: size_t, flags: c_uint) -> ssize_t
```

### `freopen64`

```rust
unsafe fn freopen64(filename: *const c_char, mode: *const c_char, file: *mut crate::FILE) -> *mut crate::FILE
```

### `fseeko64`

```rust
unsafe fn fseeko64(stream: *mut crate::FILE, offset: crate::off64_t, whence: c_int) -> c_int
```

### `fsetpos64`

```rust
unsafe fn fsetpos64(stream: *mut crate::FILE, ptr: *const crate::fpos64_t) -> c_int
```

### `ftello64`

```rust
unsafe fn ftello64(stream: *mut crate::FILE) -> crate::off64_t
```

### `CMSG_NXTHDR`

```rust
unsafe fn CMSG_NXTHDR(mhdr: *const crate::msghdr, cmsg: *const crate::cmsghdr) -> *mut crate::cmsghdr
```

### `CPU_ALLOC_SIZE`

```rust
unsafe fn CPU_ALLOC_SIZE(count: c_int) -> size_t
```

### `CPU_ZERO`

```rust
unsafe fn CPU_ZERO(cpuset: &mut cpu_set_t)
```

### `CPU_SET`

```rust
unsafe fn CPU_SET(cpu: usize, cpuset: &mut cpu_set_t)
```

### `CPU_CLR`

```rust
unsafe fn CPU_CLR(cpu: usize, cpuset: &mut cpu_set_t)
```

### `CPU_ISSET`

```rust
unsafe fn CPU_ISSET(cpu: usize, cpuset: &cpu_set_t) -> bool
```

### `CPU_COUNT_S`

```rust
unsafe fn CPU_COUNT_S(size: usize, cpuset: &cpu_set_t) -> c_int
```

### `CPU_COUNT`

```rust
unsafe fn CPU_COUNT(cpuset: &cpu_set_t) -> c_int
```

### `CPU_EQUAL`

```rust
unsafe fn CPU_EQUAL(set1: &cpu_set_t, set2: &cpu_set_t) -> bool
```

### `IPTOS_TOS`

```rust
unsafe fn IPTOS_TOS(tos: u8) -> u8
```

### `IPTOS_PREC`

```rust
unsafe fn IPTOS_PREC(tos: u8) -> u8
```

### `RT_TOS`

```rust
unsafe fn RT_TOS(tos: u8) -> u8
```

### `RT_ADDRCLASS`

```rust
unsafe fn RT_ADDRCLASS(flags: u32) -> u32
```

### `RT_LOCALADDR`

```rust
unsafe fn RT_LOCALADDR(flags: u32) -> bool
```

### `ELF32_R_SYM`

```rust
unsafe fn ELF32_R_SYM(val: Elf32_Word) -> Elf32_Word
```

### `ELF32_R_TYPE`

```rust
unsafe fn ELF32_R_TYPE(val: Elf32_Word) -> Elf32_Word
```

### `ELF32_R_INFO`

```rust
unsafe fn ELF32_R_INFO(sym: Elf32_Word, t: Elf32_Word) -> Elf32_Word
```

### `ELF64_R_SYM`

```rust
unsafe fn ELF64_R_SYM(val: Elf64_Xword) -> Elf64_Xword
```

### `ELF64_R_TYPE`

```rust
unsafe fn ELF64_R_TYPE(val: Elf64_Xword) -> Elf64_Xword
```

### `ELF64_R_INFO`

```rust
unsafe fn ELF64_R_INFO(sym: Elf64_Xword, t: Elf64_Xword) -> Elf64_Xword
```

### `makedev`

```rust
const fn makedev(major: c_uint, minor: c_uint) -> crate::dev_t
```

### `major`

```rust
const fn major(dev: crate::dev_t) -> c_uint
```

### `minor`

```rust
const fn minor(dev: crate::dev_t) -> c_uint
```

## Type Aliases

### `sa_family_t`

```rust
type sa_family_t = u16;
```

### `speed_t`

```rust
type speed_t = c_uint;
```

### `tcflag_t`

```rust
type tcflag_t = c_uint;
```

### `clockid_t`

```rust
type clockid_t = c_int;
```

### `timer_t`

```rust
type timer_t = *mut c_void;
```

### `useconds_t`

```rust
type useconds_t = u32;
```

### `key_t`

```rust
type key_t = c_int;
```

### `id_t`

```rust
type id_t = c_uint;
```

### `dev_t`

```rust
type dev_t = u64;
```

### `socklen_t`

```rust
type socklen_t = u32;
```

### `mode_t`

```rust
type mode_t = u32;
```

### `ino64_t`

```rust
type ino64_t = u64;
```

### `off64_t`

```rust
type off64_t = i64;
```

### `blkcnt64_t`

```rust
type blkcnt64_t = i64;
```

### `rlim64_t`

```rust
type rlim64_t = u64;
```

### `mqd_t`

```rust
type mqd_t = c_int;
```

### `nfds_t`

```rust
type nfds_t = c_ulong;
```

### `nl_item`

```rust
type nl_item = c_int;
```

### `idtype_t`

```rust
type idtype_t = c_uint;
```

### `loff_t`

```rust
type loff_t = c_longlong;
```

### `pthread_key_t`

```rust
type pthread_key_t = c_uint;
```

### `pthread_once_t`

```rust
type pthread_once_t = c_int;
```

### `pthread_spinlock_t`

```rust
type pthread_spinlock_t = c_int;
```

### `__kernel_fsid_t`

```rust
type __kernel_fsid_t = __c_anonymous__kernel_fsid_t;
```

### `__kernel_clockid_t`

```rust
type __kernel_clockid_t = c_int;
```

### `__u8`

```rust
type __u8 = c_uchar;
```

### `__u16`

```rust
type __u16 = c_ushort;
```

### `__s16`

```rust
type __s16 = c_short;
```

### `__u32`

```rust
type __u32 = c_uint;
```

### `__s32`

```rust
type __s32 = c_int;
```

### `sctp_assoc_t`

```rust
type sctp_assoc_t = __s32;
```

### `eventfd_t`

```rust
type eventfd_t = u64;
```

### `pid_type`

```rust
type pid_type = c_uint;
```

### `proc_cn_mcast_op`

```rust
type proc_cn_mcast_op = c_uint;
```

### `proc_cn_event`

```rust
type proc_cn_event = c_uint;
```

### `Elf32_Half`

```rust
type Elf32_Half = u16;
```

### `Elf32_Word`

```rust
type Elf32_Word = u32;
```

### `Elf32_Off`

```rust
type Elf32_Off = u32;
```

### `Elf32_Addr`

```rust
type Elf32_Addr = u32;
```

### `Elf32_Xword`

```rust
type Elf32_Xword = u64;
```

### `Elf32_Sword`

```rust
type Elf32_Sword = i32;
```

### `Elf64_Half`

```rust
type Elf64_Half = u16;
```

### `Elf64_Word`

```rust
type Elf64_Word = u32;
```

### `Elf64_Off`

```rust
type Elf64_Off = u64;
```

### `Elf64_Addr`

```rust
type Elf64_Addr = u64;
```

### `Elf64_Xword`

```rust
type Elf64_Xword = u64;
```

### `Elf64_Sxword`

```rust
type Elf64_Sxword = i64;
```

### `Elf64_Sword`

```rust
type Elf64_Sword = i32;
```

### `Elf32_Section`

```rust
type Elf32_Section = u16;
```

### `Elf64_Section`

```rust
type Elf64_Section = u16;
```

### `Elf32_Relr`

```rust
type Elf32_Relr = Elf32_Word;
```

### `Elf64_Relr`

```rust
type Elf64_Relr = Elf32_Xword;
```

### `Elf32_Rel`

```rust
type Elf32_Rel = __c_anonymous_elf32_rel;
```

### `Elf64_Rel`

```rust
type Elf64_Rel = __c_anonymous_elf64_rel;
```

### `Elf32_Rela`

```rust
type Elf32_Rela = __c_anonymous_elf32_rela;
```

### `Elf64_Rela`

```rust
type Elf64_Rela = __c_anonymous_elf64_rela;
```

### `iconv_t`

```rust
type iconv_t = *mut c_void;
```

## Constants

### `ULONG_SIZE`
```rust
const ULONG_SIZE: usize = 64usize;
```

### `EXIT_FAILURE`
```rust
const EXIT_FAILURE: c_int = 1i32;
```

### `EXIT_SUCCESS`
```rust
const EXIT_SUCCESS: c_int = 0i32;
```

### `RAND_MAX`
```rust
const RAND_MAX: c_int = 2_147_483_647i32;
```

### `EOF`
```rust
const EOF: c_int = -1i32;
```

### `SEEK_SET`
```rust
const SEEK_SET: c_int = 0i32;
```

### `SEEK_CUR`
```rust
const SEEK_CUR: c_int = 1i32;
```

### `SEEK_END`
```rust
const SEEK_END: c_int = 2i32;
```

### `_IOFBF`
```rust
const _IOFBF: c_int = 0i32;
```

### `_IONBF`
```rust
const _IONBF: c_int = 2i32;
```

### `_IOLBF`
```rust
const _IOLBF: c_int = 1i32;
```

### `F_DUPFD`
```rust
const F_DUPFD: c_int = 0i32;
```

### `F_GETFD`
```rust
const F_GETFD: c_int = 1i32;
```

### `F_SETFD`
```rust
const F_SETFD: c_int = 2i32;
```

### `F_GETFL`
```rust
const F_GETFL: c_int = 3i32;
```

### `F_SETFL`
```rust
const F_SETFL: c_int = 4i32;
```

### `F_SETLEASE`
```rust
const F_SETLEASE: c_int = 1_024i32;
```

### `F_GETLEASE`
```rust
const F_GETLEASE: c_int = 1_025i32;
```

### `F_NOTIFY`
```rust
const F_NOTIFY: c_int = 1_026i32;
```

### `F_CANCELLK`
```rust
const F_CANCELLK: c_int = 1_029i32;
```

### `F_DUPFD_CLOEXEC`
```rust
const F_DUPFD_CLOEXEC: c_int = 1_030i32;
```

### `F_SETPIPE_SZ`
```rust
const F_SETPIPE_SZ: c_int = 1_031i32;
```

### `F_GETPIPE_SZ`
```rust
const F_GETPIPE_SZ: c_int = 1_032i32;
```

### `F_ADD_SEALS`
```rust
const F_ADD_SEALS: c_int = 1_033i32;
```

### `F_GET_SEALS`
```rust
const F_GET_SEALS: c_int = 1_034i32;
```

### `F_SEAL_SEAL`
```rust
const F_SEAL_SEAL: c_int = 1i32;
```

### `F_SEAL_SHRINK`
```rust
const F_SEAL_SHRINK: c_int = 2i32;
```

### `F_SEAL_GROW`
```rust
const F_SEAL_GROW: c_int = 4i32;
```

### `F_SEAL_WRITE`
```rust
const F_SEAL_WRITE: c_int = 8i32;
```

### `SIGTRAP`
```rust
const SIGTRAP: c_int = 5i32;
```

### `PTHREAD_CREATE_JOINABLE`
```rust
const PTHREAD_CREATE_JOINABLE: c_int = 0i32;
```

### `PTHREAD_CREATE_DETACHED`
```rust
const PTHREAD_CREATE_DETACHED: c_int = 1i32;
```

### `CLOCK_REALTIME`
```rust
const CLOCK_REALTIME: crate::clockid_t = 0i32;
```

### `CLOCK_MONOTONIC`
```rust
const CLOCK_MONOTONIC: crate::clockid_t = 1i32;
```

### `CLOCK_PROCESS_CPUTIME_ID`
```rust
const CLOCK_PROCESS_CPUTIME_ID: crate::clockid_t = 2i32;
```

### `CLOCK_THREAD_CPUTIME_ID`
```rust
const CLOCK_THREAD_CPUTIME_ID: crate::clockid_t = 3i32;
```

### `CLOCK_MONOTONIC_RAW`
```rust
const CLOCK_MONOTONIC_RAW: crate::clockid_t = 4i32;
```

### `CLOCK_REALTIME_COARSE`
```rust
const CLOCK_REALTIME_COARSE: crate::clockid_t = 5i32;
```

### `CLOCK_MONOTONIC_COARSE`
```rust
const CLOCK_MONOTONIC_COARSE: crate::clockid_t = 6i32;
```

### `CLOCK_BOOTTIME`
```rust
const CLOCK_BOOTTIME: crate::clockid_t = 7i32;
```

### `CLOCK_REALTIME_ALARM`
```rust
const CLOCK_REALTIME_ALARM: crate::clockid_t = 8i32;
```

### `CLOCK_BOOTTIME_ALARM`
```rust
const CLOCK_BOOTTIME_ALARM: crate::clockid_t = 9i32;
```

### `CLOCK_TAI`
```rust
const CLOCK_TAI: crate::clockid_t = 11i32;
```

### `TIMER_ABSTIME`
```rust
const TIMER_ABSTIME: c_int = 1i32;
```

### `RUSAGE_SELF`
```rust
const RUSAGE_SELF: c_int = 0i32;
```

### `O_RDONLY`
```rust
const O_RDONLY: c_int = 0i32;
```

### `O_WRONLY`
```rust
const O_WRONLY: c_int = 1i32;
```

### `O_RDWR`
```rust
const O_RDWR: c_int = 2i32;
```

### `SOCK_CLOEXEC`
```rust
const SOCK_CLOEXEC: c_int = 524_288i32;
```

### `S_IFIFO`
```rust
const S_IFIFO: mode_t = 4_096u32;
```

### `S_IFCHR`
```rust
const S_IFCHR: mode_t = 8_192u32;
```

### `S_IFBLK`
```rust
const S_IFBLK: mode_t = 24_576u32;
```

### `S_IFDIR`
```rust
const S_IFDIR: mode_t = 16_384u32;
```

### `S_IFREG`
```rust
const S_IFREG: mode_t = 32_768u32;
```

### `S_IFLNK`
```rust
const S_IFLNK: mode_t = 40_960u32;
```

### `S_IFSOCK`
```rust
const S_IFSOCK: mode_t = 49_152u32;
```

### `S_IFMT`
```rust
const S_IFMT: mode_t = 61_440u32;
```

### `S_IRWXU`
```rust
const S_IRWXU: mode_t = 448u32;
```

### `S_IXUSR`
```rust
const S_IXUSR: mode_t = 64u32;
```

### `S_IWUSR`
```rust
const S_IWUSR: mode_t = 128u32;
```

### `S_IRUSR`
```rust
const S_IRUSR: mode_t = 256u32;
```

### `S_IRWXG`
```rust
const S_IRWXG: mode_t = 56u32;
```

### `S_IXGRP`
```rust
const S_IXGRP: mode_t = 8u32;
```

### `S_IWGRP`
```rust
const S_IWGRP: mode_t = 16u32;
```

### `S_IRGRP`
```rust
const S_IRGRP: mode_t = 32u32;
```

### `S_IRWXO`
```rust
const S_IRWXO: mode_t = 7u32;
```

### `S_IXOTH`
```rust
const S_IXOTH: mode_t = 1u32;
```

### `S_IWOTH`
```rust
const S_IWOTH: mode_t = 2u32;
```

### `S_IROTH`
```rust
const S_IROTH: mode_t = 4u32;
```

### `F_OK`
```rust
const F_OK: c_int = 0i32;
```

### `R_OK`
```rust
const R_OK: c_int = 4i32;
```

### `W_OK`
```rust
const W_OK: c_int = 2i32;
```

### `X_OK`
```rust
const X_OK: c_int = 1i32;
```

### `SIGHUP`
```rust
const SIGHUP: c_int = 1i32;
```

### `SIGINT`
```rust
const SIGINT: c_int = 2i32;
```

### `SIGQUIT`
```rust
const SIGQUIT: c_int = 3i32;
```

### `SIGILL`
```rust
const SIGILL: c_int = 4i32;
```

### `SIGABRT`
```rust
const SIGABRT: c_int = 6i32;
```

### `SIGFPE`
```rust
const SIGFPE: c_int = 8i32;
```

### `SIGKILL`
```rust
const SIGKILL: c_int = 9i32;
```

### `SIGSEGV`
```rust
const SIGSEGV: c_int = 11i32;
```

### `SIGPIPE`
```rust
const SIGPIPE: c_int = 13i32;
```

### `SIGALRM`
```rust
const SIGALRM: c_int = 14i32;
```

### `SIGTERM`
```rust
const SIGTERM: c_int = 15i32;
```

### `PROT_NONE`
```rust
const PROT_NONE: c_int = 0i32;
```

### `PROT_READ`
```rust
const PROT_READ: c_int = 1i32;
```

### `PROT_WRITE`
```rust
const PROT_WRITE: c_int = 2i32;
```

### `PROT_EXEC`
```rust
const PROT_EXEC: c_int = 4i32;
```

### `XATTR_CREATE`
```rust
const XATTR_CREATE: c_int = 1i32;
```

### `XATTR_REPLACE`
```rust
const XATTR_REPLACE: c_int = 2i32;
```

### `RLIM64_INFINITY`
```rust
const RLIM64_INFINITY: crate::rlim64_t = 18_446_744_073_709_551_615u64;
```

### `LC_CTYPE`
```rust
const LC_CTYPE: c_int = 0i32;
```

### `LC_NUMERIC`
```rust
const LC_NUMERIC: c_int = 1i32;
```

### `LC_TIME`
```rust
const LC_TIME: c_int = 2i32;
```

### `LC_COLLATE`
```rust
const LC_COLLATE: c_int = 3i32;
```

### `LC_MONETARY`
```rust
const LC_MONETARY: c_int = 4i32;
```

### `LC_MESSAGES`
```rust
const LC_MESSAGES: c_int = 5i32;
```

### `LC_ALL`
```rust
const LC_ALL: c_int = 6i32;
```

### `LC_CTYPE_MASK`
```rust
const LC_CTYPE_MASK: c_int = 1i32;
```

### `LC_NUMERIC_MASK`
```rust
const LC_NUMERIC_MASK: c_int = 2i32;
```

### `LC_TIME_MASK`
```rust
const LC_TIME_MASK: c_int = 4i32;
```

### `LC_COLLATE_MASK`
```rust
const LC_COLLATE_MASK: c_int = 8i32;
```

### `LC_MONETARY_MASK`
```rust
const LC_MONETARY_MASK: c_int = 16i32;
```

### `LC_MESSAGES_MASK`
```rust
const LC_MESSAGES_MASK: c_int = 32i32;
```

### `MAP_FILE`
```rust
const MAP_FILE: c_int = 0i32;
```

### `MAP_SHARED`
```rust
const MAP_SHARED: c_int = 1i32;
```

### `MAP_PRIVATE`
```rust
const MAP_PRIVATE: c_int = 2i32;
```

### `MAP_FIXED`
```rust
const MAP_FIXED: c_int = 16i32;
```

### `MAP_FAILED`
```rust
const MAP_FAILED: *mut c_void = {0xffffffffffffffff as *mut core::ffi::c_void};
```

### `MS_ASYNC`
```rust
const MS_ASYNC: c_int = 1i32;
```

### `MS_INVALIDATE`
```rust
const MS_INVALIDATE: c_int = 2i32;
```

### `MS_SYNC`
```rust
const MS_SYNC: c_int = 4i32;
```

### `MS_RDONLY`
```rust
const MS_RDONLY: c_ulong = 1u64;
```

### `MS_NOSUID`
```rust
const MS_NOSUID: c_ulong = 2u64;
```

### `MS_NODEV`
```rust
const MS_NODEV: c_ulong = 4u64;
```

### `MS_NOEXEC`
```rust
const MS_NOEXEC: c_ulong = 8u64;
```

### `MS_SYNCHRONOUS`
```rust
const MS_SYNCHRONOUS: c_ulong = 16u64;
```

### `MS_REMOUNT`
```rust
const MS_REMOUNT: c_ulong = 32u64;
```

### `MS_MANDLOCK`
```rust
const MS_MANDLOCK: c_ulong = 64u64;
```

### `MS_DIRSYNC`
```rust
const MS_DIRSYNC: c_ulong = 128u64;
```

### `MS_NOSYMFOLLOW`
```rust
const MS_NOSYMFOLLOW: c_ulong = 256u64;
```

### `MS_NOATIME`
```rust
const MS_NOATIME: c_ulong = 1_024u64;
```

### `MS_NODIRATIME`
```rust
const MS_NODIRATIME: c_ulong = 2_048u64;
```

### `MS_BIND`
```rust
const MS_BIND: c_ulong = 4_096u64;
```

### `MS_MOVE`
```rust
const MS_MOVE: c_ulong = 8_192u64;
```

### `MS_REC`
```rust
const MS_REC: c_ulong = 16_384u64;
```

### `MS_SILENT`
```rust
const MS_SILENT: c_ulong = 32_768u64;
```

### `MS_POSIXACL`
```rust
const MS_POSIXACL: c_ulong = 65_536u64;
```

### `MS_UNBINDABLE`
```rust
const MS_UNBINDABLE: c_ulong = 131_072u64;
```

### `MS_PRIVATE`
```rust
const MS_PRIVATE: c_ulong = 262_144u64;
```

### `MS_SLAVE`
```rust
const MS_SLAVE: c_ulong = 524_288u64;
```

### `MS_SHARED`
```rust
const MS_SHARED: c_ulong = 1_048_576u64;
```

### `MS_RELATIME`
```rust
const MS_RELATIME: c_ulong = 2_097_152u64;
```

### `MS_KERNMOUNT`
```rust
const MS_KERNMOUNT: c_ulong = 4_194_304u64;
```

### `MS_I_VERSION`
```rust
const MS_I_VERSION: c_ulong = 8_388_608u64;
```

### `MS_STRICTATIME`
```rust
const MS_STRICTATIME: c_ulong = 16_777_216u64;
```

### `MS_LAZYTIME`
```rust
const MS_LAZYTIME: c_ulong = 33_554_432u64;
```

### `MS_ACTIVE`
```rust
const MS_ACTIVE: c_ulong = 1_073_741_824u64;
```

### `MS_MGC_VAL`
```rust
const MS_MGC_VAL: c_ulong = 3_236_757_504u64;
```

### `MS_MGC_MSK`
```rust
const MS_MGC_MSK: c_ulong = 4_294_901_760u64;
```

### `SCM_RIGHTS`
```rust
const SCM_RIGHTS: c_int = 1i32;
```

### `SCM_CREDENTIALS`
```rust
const SCM_CREDENTIALS: c_int = 2i32;
```

### `PROT_GROWSDOWN`
```rust
const PROT_GROWSDOWN: c_int = 16_777_216i32;
```

### `PROT_GROWSUP`
```rust
const PROT_GROWSUP: c_int = 33_554_432i32;
```

### `MAP_TYPE`
```rust
const MAP_TYPE: c_int = 15i32;
```

### `MADV_NORMAL`
```rust
const MADV_NORMAL: c_int = 0i32;
```

### `MADV_RANDOM`
```rust
const MADV_RANDOM: c_int = 1i32;
```

### `MADV_SEQUENTIAL`
```rust
const MADV_SEQUENTIAL: c_int = 2i32;
```

### `MADV_WILLNEED`
```rust
const MADV_WILLNEED: c_int = 3i32;
```

### `MADV_DONTNEED`
```rust
const MADV_DONTNEED: c_int = 4i32;
```

### `MADV_FREE`
```rust
const MADV_FREE: c_int = 8i32;
```

### `MADV_REMOVE`
```rust
const MADV_REMOVE: c_int = 9i32;
```

### `MADV_DONTFORK`
```rust
const MADV_DONTFORK: c_int = 10i32;
```

### `MADV_DOFORK`
```rust
const MADV_DOFORK: c_int = 11i32;
```

### `MADV_MERGEABLE`
```rust
const MADV_MERGEABLE: c_int = 12i32;
```

### `MADV_UNMERGEABLE`
```rust
const MADV_UNMERGEABLE: c_int = 13i32;
```

### `MADV_HUGEPAGE`
```rust
const MADV_HUGEPAGE: c_int = 14i32;
```

### `MADV_NOHUGEPAGE`
```rust
const MADV_NOHUGEPAGE: c_int = 15i32;
```

### `MADV_DONTDUMP`
```rust
const MADV_DONTDUMP: c_int = 16i32;
```

### `MADV_DODUMP`
```rust
const MADV_DODUMP: c_int = 17i32;
```

### `MADV_WIPEONFORK`
```rust
const MADV_WIPEONFORK: c_int = 18i32;
```

### `MADV_KEEPONFORK`
```rust
const MADV_KEEPONFORK: c_int = 19i32;
```

### `MADV_COLD`
```rust
const MADV_COLD: c_int = 20i32;
```

### `MADV_PAGEOUT`
```rust
const MADV_PAGEOUT: c_int = 21i32;
```

### `MADV_HWPOISON`
```rust
const MADV_HWPOISON: c_int = 100i32;
```

### `MADV_POPULATE_READ`
```rust
const MADV_POPULATE_READ: c_int = 22i32;
```

### `MADV_POPULATE_WRITE`
```rust
const MADV_POPULATE_WRITE: c_int = 23i32;
```

### `MADV_DONTNEED_LOCKED`
```rust
const MADV_DONTNEED_LOCKED: c_int = 24i32;
```

### `IFF_UP`
```rust
const IFF_UP: c_int = 1i32;
```

### `IFF_BROADCAST`
```rust
const IFF_BROADCAST: c_int = 2i32;
```

### `IFF_DEBUG`
```rust
const IFF_DEBUG: c_int = 4i32;
```

### `IFF_LOOPBACK`
```rust
const IFF_LOOPBACK: c_int = 8i32;
```

### `IFF_POINTOPOINT`
```rust
const IFF_POINTOPOINT: c_int = 16i32;
```

### `IFF_NOTRAILERS`
```rust
const IFF_NOTRAILERS: c_int = 32i32;
```

### `IFF_RUNNING`
```rust
const IFF_RUNNING: c_int = 64i32;
```

### `IFF_NOARP`
```rust
const IFF_NOARP: c_int = 128i32;
```

### `IFF_PROMISC`
```rust
const IFF_PROMISC: c_int = 256i32;
```

### `IFF_ALLMULTI`
```rust
const IFF_ALLMULTI: c_int = 512i32;
```

### `IFF_MASTER`
```rust
const IFF_MASTER: c_int = 1_024i32;
```

### `IFF_SLAVE`
```rust
const IFF_SLAVE: c_int = 2_048i32;
```

### `IFF_MULTICAST`
```rust
const IFF_MULTICAST: c_int = 4_096i32;
```

### `IFF_PORTSEL`
```rust
const IFF_PORTSEL: c_int = 8_192i32;
```

### `IFF_AUTOMEDIA`
```rust
const IFF_AUTOMEDIA: c_int = 16_384i32;
```

### `IFF_DYNAMIC`
```rust
const IFF_DYNAMIC: c_int = 32_768i32;
```

### `SOL_IP`
```rust
const SOL_IP: c_int = 0i32;
```

### `SOL_TCP`
```rust
const SOL_TCP: c_int = 6i32;
```

### `SOL_UDP`
```rust
const SOL_UDP: c_int = 17i32;
```

### `SOL_IPV6`
```rust
const SOL_IPV6: c_int = 41i32;
```

### `SOL_ICMPV6`
```rust
const SOL_ICMPV6: c_int = 58i32;
```

### `SOL_RAW`
```rust
const SOL_RAW: c_int = 255i32;
```

### `SOL_DECNET`
```rust
const SOL_DECNET: c_int = 261i32;
```

### `SOL_X25`
```rust
const SOL_X25: c_int = 262i32;
```

### `SOL_PACKET`
```rust
const SOL_PACKET: c_int = 263i32;
```

### `SOL_ATM`
```rust
const SOL_ATM: c_int = 264i32;
```

### `SOL_AAL`
```rust
const SOL_AAL: c_int = 265i32;
```

### `SOL_IRDA`
```rust
const SOL_IRDA: c_int = 266i32;
```

### `SOL_NETBEUI`
```rust
const SOL_NETBEUI: c_int = 267i32;
```

### `SOL_LLC`
```rust
const SOL_LLC: c_int = 268i32;
```

### `SOL_DCCP`
```rust
const SOL_DCCP: c_int = 269i32;
```

### `SOL_NETLINK`
```rust
const SOL_NETLINK: c_int = 270i32;
```

### `SOL_TIPC`
```rust
const SOL_TIPC: c_int = 271i32;
```

### `SOL_BLUETOOTH`
```rust
const SOL_BLUETOOTH: c_int = 274i32;
```

### `SOL_ALG`
```rust
const SOL_ALG: c_int = 279i32;
```

### `AF_UNSPEC`
```rust
const AF_UNSPEC: c_int = 0i32;
```

### `AF_UNIX`
```rust
const AF_UNIX: c_int = 1i32;
```

### `AF_LOCAL`
```rust
const AF_LOCAL: c_int = 1i32;
```

### `AF_INET`
```rust
const AF_INET: c_int = 2i32;
```

### `AF_AX25`
```rust
const AF_AX25: c_int = 3i32;
```

### `AF_IPX`
```rust
const AF_IPX: c_int = 4i32;
```

### `AF_APPLETALK`
```rust
const AF_APPLETALK: c_int = 5i32;
```

### `AF_NETROM`
```rust
const AF_NETROM: c_int = 6i32;
```

### `AF_BRIDGE`
```rust
const AF_BRIDGE: c_int = 7i32;
```

### `AF_ATMPVC`
```rust
const AF_ATMPVC: c_int = 8i32;
```

### `AF_X25`
```rust
const AF_X25: c_int = 9i32;
```

### `AF_INET6`
```rust
const AF_INET6: c_int = 10i32;
```

### `AF_ROSE`
```rust
const AF_ROSE: c_int = 11i32;
```

### `AF_DECnet`
```rust
const AF_DECnet: c_int = 12i32;
```

### `AF_NETBEUI`
```rust
const AF_NETBEUI: c_int = 13i32;
```

### `AF_SECURITY`
```rust
const AF_SECURITY: c_int = 14i32;
```

### `AF_KEY`
```rust
const AF_KEY: c_int = 15i32;
```

### `AF_NETLINK`
```rust
const AF_NETLINK: c_int = 16i32;
```

### `AF_ROUTE`
```rust
const AF_ROUTE: c_int = 16i32;
```

### `AF_PACKET`
```rust
const AF_PACKET: c_int = 17i32;
```

### `AF_ASH`
```rust
const AF_ASH: c_int = 18i32;
```

### `AF_ECONET`
```rust
const AF_ECONET: c_int = 19i32;
```

### `AF_ATMSVC`
```rust
const AF_ATMSVC: c_int = 20i32;
```

### `AF_RDS`
```rust
const AF_RDS: c_int = 21i32;
```

### `AF_SNA`
```rust
const AF_SNA: c_int = 22i32;
```

### `AF_IRDA`
```rust
const AF_IRDA: c_int = 23i32;
```

### `AF_PPPOX`
```rust
const AF_PPPOX: c_int = 24i32;
```

### `AF_WANPIPE`
```rust
const AF_WANPIPE: c_int = 25i32;
```

### `AF_LLC`
```rust
const AF_LLC: c_int = 26i32;
```

### `AF_CAN`
```rust
const AF_CAN: c_int = 29i32;
```

### `AF_TIPC`
```rust
const AF_TIPC: c_int = 30i32;
```

### `AF_BLUETOOTH`
```rust
const AF_BLUETOOTH: c_int = 31i32;
```

### `AF_IUCV`
```rust
const AF_IUCV: c_int = 32i32;
```

### `AF_RXRPC`
```rust
const AF_RXRPC: c_int = 33i32;
```

### `AF_ISDN`
```rust
const AF_ISDN: c_int = 34i32;
```

### `AF_PHONET`
```rust
const AF_PHONET: c_int = 35i32;
```

### `AF_IEEE802154`
```rust
const AF_IEEE802154: c_int = 36i32;
```

### `AF_CAIF`
```rust
const AF_CAIF: c_int = 37i32;
```

### `AF_ALG`
```rust
const AF_ALG: c_int = 38i32;
```

### `PF_UNSPEC`
```rust
const PF_UNSPEC: c_int = 0i32;
```

### `PF_UNIX`
```rust
const PF_UNIX: c_int = 1i32;
```

### `PF_LOCAL`
```rust
const PF_LOCAL: c_int = 1i32;
```

### `PF_INET`
```rust
const PF_INET: c_int = 2i32;
```

### `PF_AX25`
```rust
const PF_AX25: c_int = 3i32;
```

### `PF_IPX`
```rust
const PF_IPX: c_int = 4i32;
```

### `PF_APPLETALK`
```rust
const PF_APPLETALK: c_int = 5i32;
```

### `PF_NETROM`
```rust
const PF_NETROM: c_int = 6i32;
```

### `PF_BRIDGE`
```rust
const PF_BRIDGE: c_int = 7i32;
```

### `PF_ATMPVC`
```rust
const PF_ATMPVC: c_int = 8i32;
```

### `PF_X25`
```rust
const PF_X25: c_int = 9i32;
```

### `PF_INET6`
```rust
const PF_INET6: c_int = 10i32;
```

### `PF_ROSE`
```rust
const PF_ROSE: c_int = 11i32;
```

### `PF_DECnet`
```rust
const PF_DECnet: c_int = 12i32;
```

### `PF_NETBEUI`
```rust
const PF_NETBEUI: c_int = 13i32;
```

### `PF_SECURITY`
```rust
const PF_SECURITY: c_int = 14i32;
```

### `PF_KEY`
```rust
const PF_KEY: c_int = 15i32;
```

### `PF_NETLINK`
```rust
const PF_NETLINK: c_int = 16i32;
```

### `PF_ROUTE`
```rust
const PF_ROUTE: c_int = 16i32;
```

### `PF_PACKET`
```rust
const PF_PACKET: c_int = 17i32;
```

### `PF_ASH`
```rust
const PF_ASH: c_int = 18i32;
```

### `PF_ECONET`
```rust
const PF_ECONET: c_int = 19i32;
```

### `PF_ATMSVC`
```rust
const PF_ATMSVC: c_int = 20i32;
```

### `PF_RDS`
```rust
const PF_RDS: c_int = 21i32;
```

### `PF_SNA`
```rust
const PF_SNA: c_int = 22i32;
```

### `PF_IRDA`
```rust
const PF_IRDA: c_int = 23i32;
```

### `PF_PPPOX`
```rust
const PF_PPPOX: c_int = 24i32;
```

### `PF_WANPIPE`
```rust
const PF_WANPIPE: c_int = 25i32;
```

### `PF_LLC`
```rust
const PF_LLC: c_int = 26i32;
```

### `PF_CAN`
```rust
const PF_CAN: c_int = 29i32;
```

### `PF_TIPC`
```rust
const PF_TIPC: c_int = 30i32;
```

### `PF_BLUETOOTH`
```rust
const PF_BLUETOOTH: c_int = 31i32;
```

### `PF_IUCV`
```rust
const PF_IUCV: c_int = 32i32;
```

### `PF_RXRPC`
```rust
const PF_RXRPC: c_int = 33i32;
```

### `PF_ISDN`
```rust
const PF_ISDN: c_int = 34i32;
```

### `PF_PHONET`
```rust
const PF_PHONET: c_int = 35i32;
```

### `PF_IEEE802154`
```rust
const PF_IEEE802154: c_int = 36i32;
```

### `PF_CAIF`
```rust
const PF_CAIF: c_int = 37i32;
```

### `PF_ALG`
```rust
const PF_ALG: c_int = 38i32;
```

### `MSG_OOB`
```rust
const MSG_OOB: c_int = 1i32;
```

### `MSG_PEEK`
```rust
const MSG_PEEK: c_int = 2i32;
```

### `MSG_DONTROUTE`
```rust
const MSG_DONTROUTE: c_int = 4i32;
```

### `MSG_CTRUNC`
```rust
const MSG_CTRUNC: c_int = 8i32;
```

### `MSG_TRUNC`
```rust
const MSG_TRUNC: c_int = 32i32;
```

### `MSG_DONTWAIT`
```rust
const MSG_DONTWAIT: c_int = 64i32;
```

### `MSG_EOR`
```rust
const MSG_EOR: c_int = 128i32;
```

### `MSG_WAITALL`
```rust
const MSG_WAITALL: c_int = 256i32;
```

### `MSG_FIN`
```rust
const MSG_FIN: c_int = 512i32;
```

### `MSG_SYN`
```rust
const MSG_SYN: c_int = 1_024i32;
```

### `MSG_CONFIRM`
```rust
const MSG_CONFIRM: c_int = 2_048i32;
```

### `MSG_RST`
```rust
const MSG_RST: c_int = 4_096i32;
```

### `MSG_ERRQUEUE`
```rust
const MSG_ERRQUEUE: c_int = 8_192i32;
```

### `MSG_NOSIGNAL`
```rust
const MSG_NOSIGNAL: c_int = 16_384i32;
```

### `MSG_MORE`
```rust
const MSG_MORE: c_int = 32_768i32;
```

### `MSG_WAITFORONE`
```rust
const MSG_WAITFORONE: c_int = 65_536i32;
```

### `MSG_FASTOPEN`
```rust
const MSG_FASTOPEN: c_int = 536_870_912i32;
```

### `MSG_CMSG_CLOEXEC`
```rust
const MSG_CMSG_CLOEXEC: c_int = 1_073_741_824i32;
```

### `SCM_TIMESTAMP`
```rust
const SCM_TIMESTAMP: c_int = 29i32;
```

### `SOCK_RAW`
```rust
const SOCK_RAW: c_int = 3i32;
```

### `SOCK_RDM`
```rust
const SOCK_RDM: c_int = 4i32;
```

### `IP_TOS`
```rust
const IP_TOS: c_int = 1i32;
```

### `IP_TTL`
```rust
const IP_TTL: c_int = 2i32;
```

### `IP_HDRINCL`
```rust
const IP_HDRINCL: c_int = 3i32;
```

### `IP_OPTIONS`
```rust
const IP_OPTIONS: c_int = 4i32;
```

### `IP_ROUTER_ALERT`
```rust
const IP_ROUTER_ALERT: c_int = 5i32;
```

### `IP_RECVOPTS`
```rust
const IP_RECVOPTS: c_int = 6i32;
```

### `IP_RETOPTS`
```rust
const IP_RETOPTS: c_int = 7i32;
```

### `IP_PKTINFO`
```rust
const IP_PKTINFO: c_int = 8i32;
```

### `IP_PKTOPTIONS`
```rust
const IP_PKTOPTIONS: c_int = 9i32;
```

### `IP_MTU_DISCOVER`
```rust
const IP_MTU_DISCOVER: c_int = 10i32;
```

### `IP_RECVERR`
```rust
const IP_RECVERR: c_int = 11i32;
```

### `IP_RECVTTL`
```rust
const IP_RECVTTL: c_int = 12i32;
```

### `IP_RECVTOS`
```rust
const IP_RECVTOS: c_int = 13i32;
```

### `IP_MTU`
```rust
const IP_MTU: c_int = 14i32;
```

### `IP_FREEBIND`
```rust
const IP_FREEBIND: c_int = 15i32;
```

### `IP_IPSEC_POLICY`
```rust
const IP_IPSEC_POLICY: c_int = 16i32;
```

### `IP_XFRM_POLICY`
```rust
const IP_XFRM_POLICY: c_int = 17i32;
```

### `IP_PASSSEC`
```rust
const IP_PASSSEC: c_int = 18i32;
```

### `IP_TRANSPARENT`
```rust
const IP_TRANSPARENT: c_int = 19i32;
```

### `IP_ORIGDSTADDR`
```rust
const IP_ORIGDSTADDR: c_int = 20i32;
```

### `IP_RECVORIGDSTADDR`
```rust
const IP_RECVORIGDSTADDR: c_int = 20i32;
```

### `IP_MINTTL`
```rust
const IP_MINTTL: c_int = 21i32;
```

### `IP_NODEFRAG`
```rust
const IP_NODEFRAG: c_int = 22i32;
```

### `IP_CHECKSUM`
```rust
const IP_CHECKSUM: c_int = 23i32;
```

### `IP_BIND_ADDRESS_NO_PORT`
```rust
const IP_BIND_ADDRESS_NO_PORT: c_int = 24i32;
```

### `IP_MULTICAST_IF`
```rust
const IP_MULTICAST_IF: c_int = 32i32;
```

### `IP_MULTICAST_TTL`
```rust
const IP_MULTICAST_TTL: c_int = 33i32;
```

### `IP_MULTICAST_LOOP`
```rust
const IP_MULTICAST_LOOP: c_int = 34i32;
```

### `IP_ADD_MEMBERSHIP`
```rust
const IP_ADD_MEMBERSHIP: c_int = 35i32;
```

### `IP_DROP_MEMBERSHIP`
```rust
const IP_DROP_MEMBERSHIP: c_int = 36i32;
```

### `IP_UNBLOCK_SOURCE`
```rust
const IP_UNBLOCK_SOURCE: c_int = 37i32;
```

### `IP_BLOCK_SOURCE`
```rust
const IP_BLOCK_SOURCE: c_int = 38i32;
```

### `IP_ADD_SOURCE_MEMBERSHIP`
```rust
const IP_ADD_SOURCE_MEMBERSHIP: c_int = 39i32;
```

### `IP_DROP_SOURCE_MEMBERSHIP`
```rust
const IP_DROP_SOURCE_MEMBERSHIP: c_int = 40i32;
```

### `IP_MSFILTER`
```rust
const IP_MSFILTER: c_int = 41i32;
```

### `IP_MULTICAST_ALL`
```rust
const IP_MULTICAST_ALL: c_int = 49i32;
```

### `IP_UNICAST_IF`
```rust
const IP_UNICAST_IF: c_int = 50i32;
```

### `IP_DEFAULT_MULTICAST_TTL`
```rust
const IP_DEFAULT_MULTICAST_TTL: c_int = 1i32;
```

### `IP_DEFAULT_MULTICAST_LOOP`
```rust
const IP_DEFAULT_MULTICAST_LOOP: c_int = 1i32;
```

### `IP_PMTUDISC_DONT`
```rust
const IP_PMTUDISC_DONT: c_int = 0i32;
```

### `IP_PMTUDISC_WANT`
```rust
const IP_PMTUDISC_WANT: c_int = 1i32;
```

### `IP_PMTUDISC_DO`
```rust
const IP_PMTUDISC_DO: c_int = 2i32;
```

### `IP_PMTUDISC_PROBE`
```rust
const IP_PMTUDISC_PROBE: c_int = 3i32;
```

### `IP_PMTUDISC_INTERFACE`
```rust
const IP_PMTUDISC_INTERFACE: c_int = 4i32;
```

### `IP_PMTUDISC_OMIT`
```rust
const IP_PMTUDISC_OMIT: c_int = 5i32;
```

### `IPPROTO_HOPOPTS`
```rust
const IPPROTO_HOPOPTS: c_int = 0i32;
```

Hop-by-hop option header

### `IPPROTO_IGMP`
```rust
const IPPROTO_IGMP: c_int = 2i32;
```

group mgmt protocol

### `IPPROTO_IPIP`
```rust
const IPPROTO_IPIP: c_int = 4i32;
```

for compatibility

### `IPPROTO_EGP`
```rust
const IPPROTO_EGP: c_int = 8i32;
```

exterior gateway protocol

### `IPPROTO_PUP`
```rust
const IPPROTO_PUP: c_int = 12i32;
```

pup

### `IPPROTO_IDP`
```rust
const IPPROTO_IDP: c_int = 22i32;
```

xns idp

### `IPPROTO_TP`
```rust
const IPPROTO_TP: c_int = 29i32;
```

tp-4 w/ class negotiation

### `IPPROTO_DCCP`
```rust
const IPPROTO_DCCP: c_int = 33i32;
```

DCCP

### `IPPROTO_ROUTING`
```rust
const IPPROTO_ROUTING: c_int = 43i32;
```

IP6 routing header

### `IPPROTO_FRAGMENT`
```rust
const IPPROTO_FRAGMENT: c_int = 44i32;
```

IP6 fragmentation header

### `IPPROTO_RSVP`
```rust
const IPPROTO_RSVP: c_int = 46i32;
```

resource reservation

### `IPPROTO_GRE`
```rust
const IPPROTO_GRE: c_int = 47i32;
```

General Routing Encap.

### `IPPROTO_ESP`
```rust
const IPPROTO_ESP: c_int = 50i32;
```

IP6 Encap Sec. Payload

### `IPPROTO_AH`
```rust
const IPPROTO_AH: c_int = 51i32;
```

IP6 Auth Header

### `IPPROTO_NONE`
```rust
const IPPROTO_NONE: c_int = 59i32;
```

IP6 no next header

### `IPPROTO_DSTOPTS`
```rust
const IPPROTO_DSTOPTS: c_int = 60i32;
```

IP6 destination option

### `IPPROTO_MTP`
```rust
const IPPROTO_MTP: c_int = 92i32;
```

### `IPPROTO_ENCAP`
```rust
const IPPROTO_ENCAP: c_int = 98i32;
```

encapsulation header

### `IPPROTO_PIM`
```rust
const IPPROTO_PIM: c_int = 103i32;
```

Protocol indep. multicast

### `IPPROTO_COMP`
```rust
const IPPROTO_COMP: c_int = 108i32;
```

IP Payload Comp. Protocol

### `IPPROTO_SCTP`
```rust
const IPPROTO_SCTP: c_int = 132i32;
```

SCTP

### `IPPROTO_MH`
```rust
const IPPROTO_MH: c_int = 135i32;
```

### `IPPROTO_UDPLITE`
```rust
const IPPROTO_UDPLITE: c_int = 136i32;
```

### `IPPROTO_RAW`
```rust
const IPPROTO_RAW: c_int = 255i32;
```

raw IP packet

### `IPPROTO_BEETPH`
```rust
const IPPROTO_BEETPH: c_int = 94i32;
```

### `IPPROTO_MPLS`
```rust
const IPPROTO_MPLS: c_int = 137i32;
```

### `IPPROTO_MPTCP`
```rust
const IPPROTO_MPTCP: c_int = 262i32;
```

Multipath TCP

### `IPPROTO_ETHERNET`
```rust
const IPPROTO_ETHERNET: c_int = 143i32;
```

Ethernet-within-IPv6 encapsulation.

### `MCAST_EXCLUDE`
```rust
const MCAST_EXCLUDE: c_int = 0i32;
```

### `MCAST_INCLUDE`
```rust
const MCAST_INCLUDE: c_int = 1i32;
```

### `MCAST_JOIN_GROUP`
```rust
const MCAST_JOIN_GROUP: c_int = 42i32;
```

### `MCAST_BLOCK_SOURCE`
```rust
const MCAST_BLOCK_SOURCE: c_int = 43i32;
```

### `MCAST_UNBLOCK_SOURCE`
```rust
const MCAST_UNBLOCK_SOURCE: c_int = 44i32;
```

### `MCAST_LEAVE_GROUP`
```rust
const MCAST_LEAVE_GROUP: c_int = 45i32;
```

### `MCAST_JOIN_SOURCE_GROUP`
```rust
const MCAST_JOIN_SOURCE_GROUP: c_int = 46i32;
```

### `MCAST_LEAVE_SOURCE_GROUP`
```rust
const MCAST_LEAVE_SOURCE_GROUP: c_int = 47i32;
```

### `MCAST_MSFILTER`
```rust
const MCAST_MSFILTER: c_int = 48i32;
```

### `IPV6_ADDRFORM`
```rust
const IPV6_ADDRFORM: c_int = 1i32;
```

### `IPV6_2292PKTINFO`
```rust
const IPV6_2292PKTINFO: c_int = 2i32;
```

### `IPV6_2292HOPOPTS`
```rust
const IPV6_2292HOPOPTS: c_int = 3i32;
```

### `IPV6_2292DSTOPTS`
```rust
const IPV6_2292DSTOPTS: c_int = 4i32;
```

### `IPV6_2292RTHDR`
```rust
const IPV6_2292RTHDR: c_int = 5i32;
```

### `IPV6_2292PKTOPTIONS`
```rust
const IPV6_2292PKTOPTIONS: c_int = 6i32;
```

### `IPV6_CHECKSUM`
```rust
const IPV6_CHECKSUM: c_int = 7i32;
```

### `IPV6_2292HOPLIMIT`
```rust
const IPV6_2292HOPLIMIT: c_int = 8i32;
```

### `IPV6_NEXTHOP`
```rust
const IPV6_NEXTHOP: c_int = 9i32;
```

### `IPV6_AUTHHDR`
```rust
const IPV6_AUTHHDR: c_int = 10i32;
```

### `IPV6_UNICAST_HOPS`
```rust
const IPV6_UNICAST_HOPS: c_int = 16i32;
```

### `IPV6_MULTICAST_IF`
```rust
const IPV6_MULTICAST_IF: c_int = 17i32;
```

### `IPV6_MULTICAST_HOPS`
```rust
const IPV6_MULTICAST_HOPS: c_int = 18i32;
```

### `IPV6_MULTICAST_LOOP`
```rust
const IPV6_MULTICAST_LOOP: c_int = 19i32;
```

### `IPV6_ADD_MEMBERSHIP`
```rust
const IPV6_ADD_MEMBERSHIP: c_int = 20i32;
```

### `IPV6_DROP_MEMBERSHIP`
```rust
const IPV6_DROP_MEMBERSHIP: c_int = 21i32;
```

### `IPV6_ROUTER_ALERT`
```rust
const IPV6_ROUTER_ALERT: c_int = 22i32;
```

### `IPV6_MTU_DISCOVER`
```rust
const IPV6_MTU_DISCOVER: c_int = 23i32;
```

### `IPV6_MTU`
```rust
const IPV6_MTU: c_int = 24i32;
```

### `IPV6_RECVERR`
```rust
const IPV6_RECVERR: c_int = 25i32;
```

### `IPV6_V6ONLY`
```rust
const IPV6_V6ONLY: c_int = 26i32;
```

### `IPV6_JOIN_ANYCAST`
```rust
const IPV6_JOIN_ANYCAST: c_int = 27i32;
```

### `IPV6_LEAVE_ANYCAST`
```rust
const IPV6_LEAVE_ANYCAST: c_int = 28i32;
```

### `IPV6_IPSEC_POLICY`
```rust
const IPV6_IPSEC_POLICY: c_int = 34i32;
```

### `IPV6_XFRM_POLICY`
```rust
const IPV6_XFRM_POLICY: c_int = 35i32;
```

### `IPV6_HDRINCL`
```rust
const IPV6_HDRINCL: c_int = 36i32;
```

### `IPV6_RECVPKTINFO`
```rust
const IPV6_RECVPKTINFO: c_int = 49i32;
```

### `IPV6_PKTINFO`
```rust
const IPV6_PKTINFO: c_int = 50i32;
```

### `IPV6_RECVHOPLIMIT`
```rust
const IPV6_RECVHOPLIMIT: c_int = 51i32;
```

### `IPV6_HOPLIMIT`
```rust
const IPV6_HOPLIMIT: c_int = 52i32;
```

### `IPV6_RECVHOPOPTS`
```rust
const IPV6_RECVHOPOPTS: c_int = 53i32;
```

### `IPV6_HOPOPTS`
```rust
const IPV6_HOPOPTS: c_int = 54i32;
```

### `IPV6_RTHDRDSTOPTS`
```rust
const IPV6_RTHDRDSTOPTS: c_int = 55i32;
```

### `IPV6_RECVRTHDR`
```rust
const IPV6_RECVRTHDR: c_int = 56i32;
```

### `IPV6_RTHDR`
```rust
const IPV6_RTHDR: c_int = 57i32;
```

### `IPV6_RECVDSTOPTS`
```rust
const IPV6_RECVDSTOPTS: c_int = 58i32;
```

### `IPV6_DSTOPTS`
```rust
const IPV6_DSTOPTS: c_int = 59i32;
```

### `IPV6_RECVPATHMTU`
```rust
const IPV6_RECVPATHMTU: c_int = 60i32;
```

### `IPV6_PATHMTU`
```rust
const IPV6_PATHMTU: c_int = 61i32;
```

### `IPV6_DONTFRAG`
```rust
const IPV6_DONTFRAG: c_int = 62i32;
```

### `IPV6_RECVTCLASS`
```rust
const IPV6_RECVTCLASS: c_int = 66i32;
```

### `IPV6_TCLASS`
```rust
const IPV6_TCLASS: c_int = 67i32;
```

### `IPV6_AUTOFLOWLABEL`
```rust
const IPV6_AUTOFLOWLABEL: c_int = 70i32;
```

### `IPV6_ADDR_PREFERENCES`
```rust
const IPV6_ADDR_PREFERENCES: c_int = 72i32;
```

### `IPV6_MINHOPCOUNT`
```rust
const IPV6_MINHOPCOUNT: c_int = 73i32;
```

### `IPV6_ORIGDSTADDR`
```rust
const IPV6_ORIGDSTADDR: c_int = 74i32;
```

### `IPV6_RECVORIGDSTADDR`
```rust
const IPV6_RECVORIGDSTADDR: c_int = 74i32;
```

### `IPV6_TRANSPARENT`
```rust
const IPV6_TRANSPARENT: c_int = 75i32;
```

### `IPV6_UNICAST_IF`
```rust
const IPV6_UNICAST_IF: c_int = 76i32;
```

### `IPV6_PREFER_SRC_TMP`
```rust
const IPV6_PREFER_SRC_TMP: c_int = 1i32;
```

### `IPV6_PREFER_SRC_PUBLIC`
```rust
const IPV6_PREFER_SRC_PUBLIC: c_int = 2i32;
```

### `IPV6_PREFER_SRC_PUBTMP_DEFAULT`
```rust
const IPV6_PREFER_SRC_PUBTMP_DEFAULT: c_int = 256i32;
```

### `IPV6_PREFER_SRC_COA`
```rust
const IPV6_PREFER_SRC_COA: c_int = 4i32;
```

### `IPV6_PREFER_SRC_HOME`
```rust
const IPV6_PREFER_SRC_HOME: c_int = 1_024i32;
```

### `IPV6_PREFER_SRC_CGA`
```rust
const IPV6_PREFER_SRC_CGA: c_int = 8i32;
```

### `IPV6_PREFER_SRC_NONCGA`
```rust
const IPV6_PREFER_SRC_NONCGA: c_int = 2_048i32;
```

### `IPV6_PMTUDISC_DONT`
```rust
const IPV6_PMTUDISC_DONT: c_int = 0i32;
```

### `IPV6_PMTUDISC_WANT`
```rust
const IPV6_PMTUDISC_WANT: c_int = 1i32;
```

### `IPV6_PMTUDISC_DO`
```rust
const IPV6_PMTUDISC_DO: c_int = 2i32;
```

### `IPV6_PMTUDISC_PROBE`
```rust
const IPV6_PMTUDISC_PROBE: c_int = 3i32;
```

### `IPV6_PMTUDISC_INTERFACE`
```rust
const IPV6_PMTUDISC_INTERFACE: c_int = 4i32;
```

### `IPV6_PMTUDISC_OMIT`
```rust
const IPV6_PMTUDISC_OMIT: c_int = 5i32;
```

### `TCP_NODELAY`
```rust
const TCP_NODELAY: c_int = 1i32;
```

### `TCP_MAXSEG`
```rust
const TCP_MAXSEG: c_int = 2i32;
```

### `TCP_CORK`
```rust
const TCP_CORK: c_int = 3i32;
```

### `TCP_KEEPIDLE`
```rust
const TCP_KEEPIDLE: c_int = 4i32;
```

### `TCP_KEEPINTVL`
```rust
const TCP_KEEPINTVL: c_int = 5i32;
```

### `TCP_KEEPCNT`
```rust
const TCP_KEEPCNT: c_int = 6i32;
```

### `TCP_SYNCNT`
```rust
const TCP_SYNCNT: c_int = 7i32;
```

### `TCP_LINGER2`
```rust
const TCP_LINGER2: c_int = 8i32;
```

### `TCP_DEFER_ACCEPT`
```rust
const TCP_DEFER_ACCEPT: c_int = 9i32;
```

### `TCP_WINDOW_CLAMP`
```rust
const TCP_WINDOW_CLAMP: c_int = 10i32;
```

### `TCP_INFO`
```rust
const TCP_INFO: c_int = 11i32;
```

### `TCP_QUICKACK`
```rust
const TCP_QUICKACK: c_int = 12i32;
```

### `TCP_CONGESTION`
```rust
const TCP_CONGESTION: c_int = 13i32;
```

### `TCP_MD5SIG`
```rust
const TCP_MD5SIG: c_int = 14i32;
```

### `TCP_COOKIE_TRANSACTIONS`
```rust
const TCP_COOKIE_TRANSACTIONS: c_int = 15i32;
```

### `TCP_THIN_LINEAR_TIMEOUTS`
```rust
const TCP_THIN_LINEAR_TIMEOUTS: c_int = 16i32;
```

### `TCP_THIN_DUPACK`
```rust
const TCP_THIN_DUPACK: c_int = 17i32;
```

### `TCP_USER_TIMEOUT`
```rust
const TCP_USER_TIMEOUT: c_int = 18i32;
```

### `TCP_REPAIR`
```rust
const TCP_REPAIR: c_int = 19i32;
```

### `TCP_REPAIR_QUEUE`
```rust
const TCP_REPAIR_QUEUE: c_int = 20i32;
```

### `TCP_QUEUE_SEQ`
```rust
const TCP_QUEUE_SEQ: c_int = 21i32;
```

### `TCP_REPAIR_OPTIONS`
```rust
const TCP_REPAIR_OPTIONS: c_int = 22i32;
```

### `TCP_FASTOPEN`
```rust
const TCP_FASTOPEN: c_int = 23i32;
```

### `TCP_TIMESTAMP`
```rust
const TCP_TIMESTAMP: c_int = 24i32;
```

### `TCP_NOTSENT_LOWAT`
```rust
const TCP_NOTSENT_LOWAT: c_int = 25i32;
```

### `TCP_CC_INFO`
```rust
const TCP_CC_INFO: c_int = 26i32;
```

### `TCP_SAVE_SYN`
```rust
const TCP_SAVE_SYN: c_int = 27i32;
```

### `TCP_SAVED_SYN`
```rust
const TCP_SAVED_SYN: c_int = 28i32;
```

### `TCP_REPAIR_WINDOW`
```rust
const TCP_REPAIR_WINDOW: c_int = 29i32;
```

### `TCP_FASTOPEN_CONNECT`
```rust
const TCP_FASTOPEN_CONNECT: c_int = 30i32;
```

### `TCP_ULP`
```rust
const TCP_ULP: c_int = 31i32;
```

### `TCP_MD5SIG_EXT`
```rust
const TCP_MD5SIG_EXT: c_int = 32i32;
```

### `TCP_FASTOPEN_KEY`
```rust
const TCP_FASTOPEN_KEY: c_int = 33i32;
```

### `TCP_FASTOPEN_NO_COOKIE`
```rust
const TCP_FASTOPEN_NO_COOKIE: c_int = 34i32;
```

### `TCP_ZEROCOPY_RECEIVE`
```rust
const TCP_ZEROCOPY_RECEIVE: c_int = 35i32;
```

### `TCP_INQ`
```rust
const TCP_INQ: c_int = 36i32;
```

### `TCP_CM_INQ`
```rust
const TCP_CM_INQ: c_int = 36i32;
```

### `TCP_MD5SIG_MAXKEYLEN`
```rust
const TCP_MD5SIG_MAXKEYLEN: usize = 80usize;
```

### `SO_DEBUG`
```rust
const SO_DEBUG: c_int = 1i32;
```

### `SHUT_RD`
```rust
const SHUT_RD: c_int = 0i32;
```

### `SHUT_WR`
```rust
const SHUT_WR: c_int = 1i32;
```

### `SHUT_RDWR`
```rust
const SHUT_RDWR: c_int = 2i32;
```

### `LOCK_SH`
```rust
const LOCK_SH: c_int = 1i32;
```

### `LOCK_EX`
```rust
const LOCK_EX: c_int = 2i32;
```

### `LOCK_NB`
```rust
const LOCK_NB: c_int = 4i32;
```

### `LOCK_UN`
```rust
const LOCK_UN: c_int = 8i32;
```

### `SS_ONSTACK`
```rust
const SS_ONSTACK: c_int = 1i32;
```

### `SS_DISABLE`
```rust
const SS_DISABLE: c_int = 2i32;
```

### `NAME_MAX`
```rust
const NAME_MAX: c_int = 255i32;
```

### `PATH_MAX`
```rust
const PATH_MAX: c_int = 4_096i32;
```

### `UIO_MAXIOV`
```rust
const UIO_MAXIOV: c_int = 1_024i32;
```

### `FD_SETSIZE`
```rust
const FD_SETSIZE: usize = 1_024usize;
```

### `EPOLLIN`
```rust
const EPOLLIN: c_int = 1i32;
```

### `EPOLLPRI`
```rust
const EPOLLPRI: c_int = 2i32;
```

### `EPOLLOUT`
```rust
const EPOLLOUT: c_int = 4i32;
```

### `EPOLLERR`
```rust
const EPOLLERR: c_int = 8i32;
```

### `EPOLLHUP`
```rust
const EPOLLHUP: c_int = 16i32;
```

### `EPOLLRDNORM`
```rust
const EPOLLRDNORM: c_int = 64i32;
```

### `EPOLLRDBAND`
```rust
const EPOLLRDBAND: c_int = 128i32;
```

### `EPOLLWRNORM`
```rust
const EPOLLWRNORM: c_int = 256i32;
```

### `EPOLLWRBAND`
```rust
const EPOLLWRBAND: c_int = 512i32;
```

### `EPOLLMSG`
```rust
const EPOLLMSG: c_int = 1_024i32;
```

### `EPOLLRDHUP`
```rust
const EPOLLRDHUP: c_int = 8_192i32;
```

### `EPOLLEXCLUSIVE`
```rust
const EPOLLEXCLUSIVE: c_int = 268_435_456i32;
```

### `EPOLLWAKEUP`
```rust
const EPOLLWAKEUP: c_int = 536_870_912i32;
```

### `EPOLLONESHOT`
```rust
const EPOLLONESHOT: c_int = 1_073_741_824i32;
```

### `EPOLLET`
```rust
const EPOLLET: c_int = -2_147_483_648i32;
```

### `EPOLL_CTL_ADD`
```rust
const EPOLL_CTL_ADD: c_int = 1i32;
```

### `EPOLL_CTL_MOD`
```rust
const EPOLL_CTL_MOD: c_int = 3i32;
```

### `EPOLL_CTL_DEL`
```rust
const EPOLL_CTL_DEL: c_int = 2i32;
```

### `MNT_FORCE`
```rust
const MNT_FORCE: c_int = 1i32;
```

### `MNT_DETACH`
```rust
const MNT_DETACH: c_int = 2i32;
```

### `MNT_EXPIRE`
```rust
const MNT_EXPIRE: c_int = 4i32;
```

### `UMOUNT_NOFOLLOW`
```rust
const UMOUNT_NOFOLLOW: c_int = 8i32;
```

### `Q_GETFMT`
```rust
const Q_GETFMT: c_int = 8_388_612i32;
```

### `Q_GETINFO`
```rust
const Q_GETINFO: c_int = 8_388_613i32;
```

### `Q_SETINFO`
```rust
const Q_SETINFO: c_int = 8_388_614i32;
```

### `QIF_BLIMITS`
```rust
const QIF_BLIMITS: u32 = 1u32;
```

### `QIF_SPACE`
```rust
const QIF_SPACE: u32 = 2u32;
```

### `QIF_ILIMITS`
```rust
const QIF_ILIMITS: u32 = 4u32;
```

### `QIF_INODES`
```rust
const QIF_INODES: u32 = 8u32;
```

### `QIF_BTIME`
```rust
const QIF_BTIME: u32 = 16u32;
```

### `QIF_ITIME`
```rust
const QIF_ITIME: u32 = 32u32;
```

### `QIF_LIMITS`
```rust
const QIF_LIMITS: u32 = 5u32;
```

### `QIF_USAGE`
```rust
const QIF_USAGE: u32 = 10u32;
```

### `QIF_TIMES`
```rust
const QIF_TIMES: u32 = 48u32;
```

### `QIF_ALL`
```rust
const QIF_ALL: u32 = 63u32;
```

### `Q_SYNC`
```rust
const Q_SYNC: c_int = 8_388_609i32;
```

### `Q_QUOTAON`
```rust
const Q_QUOTAON: c_int = 8_388_610i32;
```

### `Q_QUOTAOFF`
```rust
const Q_QUOTAOFF: c_int = 8_388_611i32;
```

### `Q_GETQUOTA`
```rust
const Q_GETQUOTA: c_int = 8_388_615i32;
```

### `Q_SETQUOTA`
```rust
const Q_SETQUOTA: c_int = 8_388_616i32;
```

### `TCIOFF`
```rust
const TCIOFF: c_int = 2i32;
```

### `TCION`
```rust
const TCION: c_int = 3i32;
```

### `TCOOFF`
```rust
const TCOOFF: c_int = 0i32;
```

### `TCOON`
```rust
const TCOON: c_int = 1i32;
```

### `TCIFLUSH`
```rust
const TCIFLUSH: c_int = 0i32;
```

### `TCOFLUSH`
```rust
const TCOFLUSH: c_int = 1i32;
```

### `TCIOFLUSH`
```rust
const TCIOFLUSH: c_int = 2i32;
```

### `NL0`
```rust
const NL0: crate::tcflag_t = 0u32;
```

### `NL1`
```rust
const NL1: crate::tcflag_t = 256u32;
```

### `TAB0`
```rust
const TAB0: crate::tcflag_t = 0u32;
```

### `CR0`
```rust
const CR0: crate::tcflag_t = 0u32;
```

### `FF0`
```rust
const FF0: crate::tcflag_t = 0u32;
```

### `BS0`
```rust
const BS0: crate::tcflag_t = 0u32;
```

### `VT0`
```rust
const VT0: crate::tcflag_t = 0u32;
```

### `VERASE`
```rust
const VERASE: usize = 2usize;
```

### `VKILL`
```rust
const VKILL: usize = 3usize;
```

### `VINTR`
```rust
const VINTR: usize = 0usize;
```

### `VQUIT`
```rust
const VQUIT: usize = 1usize;
```

### `VLNEXT`
```rust
const VLNEXT: usize = 15usize;
```

### `IGNBRK`
```rust
const IGNBRK: crate::tcflag_t = 1u32;
```

### `BRKINT`
```rust
const BRKINT: crate::tcflag_t = 2u32;
```

### `IGNPAR`
```rust
const IGNPAR: crate::tcflag_t = 4u32;
```

### `PARMRK`
```rust
const PARMRK: crate::tcflag_t = 8u32;
```

### `INPCK`
```rust
const INPCK: crate::tcflag_t = 16u32;
```

### `ISTRIP`
```rust
const ISTRIP: crate::tcflag_t = 32u32;
```

### `INLCR`
```rust
const INLCR: crate::tcflag_t = 64u32;
```

### `IGNCR`
```rust
const IGNCR: crate::tcflag_t = 128u32;
```

### `ICRNL`
```rust
const ICRNL: crate::tcflag_t = 256u32;
```

### `IXANY`
```rust
const IXANY: crate::tcflag_t = 2_048u32;
```

### `IMAXBEL`
```rust
const IMAXBEL: crate::tcflag_t = 8_192u32;
```

### `OPOST`
```rust
const OPOST: crate::tcflag_t = 1u32;
```

### `CS5`
```rust
const CS5: crate::tcflag_t = 0u32;
```

### `CRTSCTS`
```rust
const CRTSCTS: crate::tcflag_t = 2_147_483_648u32;
```

### `ECHO`
```rust
const ECHO: crate::tcflag_t = 8u32;
```

### `OCRNL`
```rust
const OCRNL: crate::tcflag_t = 8u32;
```

### `ONOCR`
```rust
const ONOCR: crate::tcflag_t = 16u32;
```

### `ONLRET`
```rust
const ONLRET: crate::tcflag_t = 32u32;
```

### `OFILL`
```rust
const OFILL: crate::tcflag_t = 64u32;
```

### `OFDEL`
```rust
const OFDEL: crate::tcflag_t = 128u32;
```

### `CLONE_VM`
```rust
const CLONE_VM: c_int = 256i32;
```

### `CLONE_FS`
```rust
const CLONE_FS: c_int = 512i32;
```

### `CLONE_FILES`
```rust
const CLONE_FILES: c_int = 1_024i32;
```

### `CLONE_SIGHAND`
```rust
const CLONE_SIGHAND: c_int = 2_048i32;
```

### `CLONE_PTRACE`
```rust
const CLONE_PTRACE: c_int = 8_192i32;
```

### `CLONE_VFORK`
```rust
const CLONE_VFORK: c_int = 16_384i32;
```

### `CLONE_PARENT`
```rust
const CLONE_PARENT: c_int = 32_768i32;
```

### `CLONE_THREAD`
```rust
const CLONE_THREAD: c_int = 65_536i32;
```

### `CLONE_NEWNS`
```rust
const CLONE_NEWNS: c_int = 131_072i32;
```

### `CLONE_SYSVSEM`
```rust
const CLONE_SYSVSEM: c_int = 262_144i32;
```

### `CLONE_SETTLS`
```rust
const CLONE_SETTLS: c_int = 524_288i32;
```

### `CLONE_PARENT_SETTID`
```rust
const CLONE_PARENT_SETTID: c_int = 1_048_576i32;
```

### `CLONE_CHILD_CLEARTID`
```rust
const CLONE_CHILD_CLEARTID: c_int = 2_097_152i32;
```

### `CLONE_DETACHED`
```rust
const CLONE_DETACHED: c_int = 4_194_304i32;
```

### `CLONE_UNTRACED`
```rust
const CLONE_UNTRACED: c_int = 8_388_608i32;
```

### `CLONE_CHILD_SETTID`
```rust
const CLONE_CHILD_SETTID: c_int = 16_777_216i32;
```

### `CLONE_NEWCGROUP`
```rust
const CLONE_NEWCGROUP: c_int = 33_554_432i32;
```

### `CLONE_NEWUTS`
```rust
const CLONE_NEWUTS: c_int = 67_108_864i32;
```

### `CLONE_NEWIPC`
```rust
const CLONE_NEWIPC: c_int = 134_217_728i32;
```

### `CLONE_NEWUSER`
```rust
const CLONE_NEWUSER: c_int = 268_435_456i32;
```

### `CLONE_NEWPID`
```rust
const CLONE_NEWPID: c_int = 536_870_912i32;
```

### `CLONE_NEWNET`
```rust
const CLONE_NEWNET: c_int = 1_073_741_824i32;
```

### `CLONE_IO`
```rust
const CLONE_IO: c_int = -2_147_483_648i32;
```

### `WNOHANG`
```rust
const WNOHANG: c_int = 1i32;
```

### `WUNTRACED`
```rust
const WUNTRACED: c_int = 2i32;
```

### `WSTOPPED`
```rust
const WSTOPPED: c_int = 2i32;
```

### `WEXITED`
```rust
const WEXITED: c_int = 4i32;
```

### `WCONTINUED`
```rust
const WCONTINUED: c_int = 8i32;
```

### `WNOWAIT`
```rust
const WNOWAIT: c_int = 16_777_216i32;
```

### `ADDR_NO_RANDOMIZE`
```rust
const ADDR_NO_RANDOMIZE: c_int = 262_144i32;
```

### `MMAP_PAGE_ZERO`
```rust
const MMAP_PAGE_ZERO: c_int = 1_048_576i32;
```

### `ADDR_COMPAT_LAYOUT`
```rust
const ADDR_COMPAT_LAYOUT: c_int = 2_097_152i32;
```

### `READ_IMPLIES_EXEC`
```rust
const READ_IMPLIES_EXEC: c_int = 4_194_304i32;
```

### `ADDR_LIMIT_32BIT`
```rust
const ADDR_LIMIT_32BIT: c_int = 8_388_608i32;
```

### `SHORT_INODE`
```rust
const SHORT_INODE: c_int = 16_777_216i32;
```

### `WHOLE_SECONDS`
```rust
const WHOLE_SECONDS: c_int = 33_554_432i32;
```

### `STICKY_TIMEOUTS`
```rust
const STICKY_TIMEOUTS: c_int = 67_108_864i32;
```

### `ADDR_LIMIT_3GB`
```rust
const ADDR_LIMIT_3GB: c_int = 134_217_728i32;
```

### `PTRACE_O_TRACESYSGOOD`
```rust
const PTRACE_O_TRACESYSGOOD: c_int = 1i32;
```

### `PTRACE_O_TRACEFORK`
```rust
const PTRACE_O_TRACEFORK: c_int = 2i32;
```

### `PTRACE_O_TRACEVFORK`
```rust
const PTRACE_O_TRACEVFORK: c_int = 4i32;
```

### `PTRACE_O_TRACECLONE`
```rust
const PTRACE_O_TRACECLONE: c_int = 8i32;
```

### `PTRACE_O_TRACEEXEC`
```rust
const PTRACE_O_TRACEEXEC: c_int = 16i32;
```

### `PTRACE_O_TRACEVFORKDONE`
```rust
const PTRACE_O_TRACEVFORKDONE: c_int = 32i32;
```

### `PTRACE_O_TRACEEXIT`
```rust
const PTRACE_O_TRACEEXIT: c_int = 64i32;
```

### `PTRACE_O_TRACESECCOMP`
```rust
const PTRACE_O_TRACESECCOMP: c_int = 128i32;
```

### `PTRACE_O_SUSPEND_SECCOMP`
```rust
const PTRACE_O_SUSPEND_SECCOMP: c_int = 2_097_152i32;
```

### `PTRACE_O_EXITKILL`
```rust
const PTRACE_O_EXITKILL: c_int = 1_048_576i32;
```

### `PTRACE_O_MASK`
```rust
const PTRACE_O_MASK: c_int = 3_145_983i32;
```

### `PTRACE_EVENT_FORK`
```rust
const PTRACE_EVENT_FORK: c_int = 1i32;
```

### `PTRACE_EVENT_VFORK`
```rust
const PTRACE_EVENT_VFORK: c_int = 2i32;
```

### `PTRACE_EVENT_CLONE`
```rust
const PTRACE_EVENT_CLONE: c_int = 3i32;
```

### `PTRACE_EVENT_EXEC`
```rust
const PTRACE_EVENT_EXEC: c_int = 4i32;
```

### `PTRACE_EVENT_VFORK_DONE`
```rust
const PTRACE_EVENT_VFORK_DONE: c_int = 5i32;
```

### `PTRACE_EVENT_EXIT`
```rust
const PTRACE_EVENT_EXIT: c_int = 6i32;
```

### `PTRACE_EVENT_SECCOMP`
```rust
const PTRACE_EVENT_SECCOMP: c_int = 7i32;
```

### `__WNOTHREAD`
```rust
const __WNOTHREAD: c_int = 536_870_912i32;
```

### `__WALL`
```rust
const __WALL: c_int = 1_073_741_824i32;
```

### `__WCLONE`
```rust
const __WCLONE: c_int = -2_147_483_648i32;
```

### `SPLICE_F_MOVE`
```rust
const SPLICE_F_MOVE: c_uint = 1u32;
```

### `SPLICE_F_NONBLOCK`
```rust
const SPLICE_F_NONBLOCK: c_uint = 2u32;
```

### `SPLICE_F_MORE`
```rust
const SPLICE_F_MORE: c_uint = 4u32;
```

### `SPLICE_F_GIFT`
```rust
const SPLICE_F_GIFT: c_uint = 8u32;
```

### `RTLD_LOCAL`
```rust
const RTLD_LOCAL: c_int = 0i32;
```

### `RTLD_LAZY`
```rust
const RTLD_LAZY: c_int = 1i32;
```

### `POSIX_FADV_NORMAL`
```rust
const POSIX_FADV_NORMAL: c_int = 0i32;
```

### `POSIX_FADV_RANDOM`
```rust
const POSIX_FADV_RANDOM: c_int = 1i32;
```

### `POSIX_FADV_SEQUENTIAL`
```rust
const POSIX_FADV_SEQUENTIAL: c_int = 2i32;
```

### `POSIX_FADV_WILLNEED`
```rust
const POSIX_FADV_WILLNEED: c_int = 3i32;
```

### `AT_FDCWD`
```rust
const AT_FDCWD: c_int = -100i32;
```

### `AT_SYMLINK_NOFOLLOW`
```rust
const AT_SYMLINK_NOFOLLOW: c_int = 256i32;
```

### `AT_REMOVEDIR`
```rust
const AT_REMOVEDIR: c_int = 512i32;
```

### `AT_SYMLINK_FOLLOW`
```rust
const AT_SYMLINK_FOLLOW: c_int = 1_024i32;
```

### `AT_NO_AUTOMOUNT`
```rust
const AT_NO_AUTOMOUNT: c_int = 2_048i32;
```

### `AT_EMPTY_PATH`
```rust
const AT_EMPTY_PATH: c_int = 4_096i32;
```

### `AT_RECURSIVE`
```rust
const AT_RECURSIVE: c_int = 32_768i32;
```

### `LOG_CRON`
```rust
const LOG_CRON: c_int = 72i32;
```

### `LOG_AUTHPRIV`
```rust
const LOG_AUTHPRIV: c_int = 80i32;
```

### `LOG_FTP`
```rust
const LOG_FTP: c_int = 88i32;
```

### `LOG_PERROR`
```rust
const LOG_PERROR: c_int = 32i32;
```

### `PIPE_BUF`
```rust
const PIPE_BUF: usize = 4_096usize;
```

### `SI_LOAD_SHIFT`
```rust
const SI_LOAD_SHIFT: c_uint = 16u32;
```

### `SI_USER`
```rust
const SI_USER: c_int = 0i32;
```

### `SI_KERNEL`
```rust
const SI_KERNEL: c_int = 128i32;
```

### `SI_QUEUE`
```rust
const SI_QUEUE: c_int = -1i32;
```

### `SI_TIMER`
```rust
const SI_TIMER: c_int = -2i32;
```

### `SI_MESGQ`
```rust
const SI_MESGQ: c_int = -3i32;
```

### `SI_ASYNCIO`
```rust
const SI_ASYNCIO: c_int = -4i32;
```

### `SI_SIGIO`
```rust
const SI_SIGIO: c_int = -5i32;
```

### `SI_TKILL`
```rust
const SI_TKILL: c_int = -6i32;
```

### `SI_ASYNCNL`
```rust
const SI_ASYNCNL: c_int = -60i32;
```

### `BUS_ADRALN`
```rust
const BUS_ADRALN: c_int = 1i32;
```

### `BUS_ADRERR`
```rust
const BUS_ADRERR: c_int = 2i32;
```

### `BUS_OBJERR`
```rust
const BUS_OBJERR: c_int = 3i32;
```

### `BUS_MCEERR_AR`
```rust
const BUS_MCEERR_AR: c_int = 4i32;
```

### `BUS_MCEERR_AO`
```rust
const BUS_MCEERR_AO: c_int = 5i32;
```

### `TRAP_BRKPT`
```rust
const TRAP_BRKPT: c_int = 1i32;
```

### `TRAP_TRACE`
```rust
const TRAP_TRACE: c_int = 2i32;
```

### `TRAP_BRANCH`
```rust
const TRAP_BRANCH: c_int = 3i32;
```

### `TRAP_HWBKPT`
```rust
const TRAP_HWBKPT: c_int = 4i32;
```

### `TRAP_UNK`
```rust
const TRAP_UNK: c_int = 5i32;
```

### `CLD_EXITED`
```rust
const CLD_EXITED: c_int = 1i32;
```

### `CLD_KILLED`
```rust
const CLD_KILLED: c_int = 2i32;
```

### `CLD_DUMPED`
```rust
const CLD_DUMPED: c_int = 3i32;
```

### `CLD_TRAPPED`
```rust
const CLD_TRAPPED: c_int = 4i32;
```

### `CLD_STOPPED`
```rust
const CLD_STOPPED: c_int = 5i32;
```

### `CLD_CONTINUED`
```rust
const CLD_CONTINUED: c_int = 6i32;
```

### `SIGEV_SIGNAL`
```rust
const SIGEV_SIGNAL: c_int = 0i32;
```

### `SIGEV_NONE`
```rust
const SIGEV_NONE: c_int = 1i32;
```

### `SIGEV_THREAD`
```rust
const SIGEV_THREAD: c_int = 2i32;
```

### `P_ALL`
```rust
const P_ALL: idtype_t = 0u32;
```

### `P_PID`
```rust
const P_PID: idtype_t = 1u32;
```

### `P_PGID`
```rust
const P_PGID: idtype_t = 2u32;
```

### `P_PIDFD`
```rust
const P_PIDFD: idtype_t = 3u32;
```

### `UTIME_OMIT`
```rust
const UTIME_OMIT: c_long = 1_073_741_822i64;
```

### `UTIME_NOW`
```rust
const UTIME_NOW: c_long = 1_073_741_823i64;
```

### `POLLIN`
```rust
const POLLIN: c_short = 1i16;
```

### `POLLPRI`
```rust
const POLLPRI: c_short = 2i16;
```

### `POLLOUT`
```rust
const POLLOUT: c_short = 4i16;
```

### `POLLERR`
```rust
const POLLERR: c_short = 8i16;
```

### `POLLHUP`
```rust
const POLLHUP: c_short = 16i16;
```

### `POLLNVAL`
```rust
const POLLNVAL: c_short = 32i16;
```

### `POLLRDNORM`
```rust
const POLLRDNORM: c_short = 64i16;
```

### `POLLRDBAND`
```rust
const POLLRDBAND: c_short = 128i16;
```

### `POLLRDHUP`
```rust
const POLLRDHUP: c_short = 8_192i16;
```

### `IPTOS_LOWDELAY`
```rust
const IPTOS_LOWDELAY: u8 = 16u8;
```

### `IPTOS_THROUGHPUT`
```rust
const IPTOS_THROUGHPUT: u8 = 8u8;
```

### `IPTOS_RELIABILITY`
```rust
const IPTOS_RELIABILITY: u8 = 4u8;
```

### `IPTOS_MINCOST`
```rust
const IPTOS_MINCOST: u8 = 2u8;
```

### `IPTOS_PREC_NETCONTROL`
```rust
const IPTOS_PREC_NETCONTROL: u8 = 224u8;
```

### `IPTOS_PREC_INTERNETCONTROL`
```rust
const IPTOS_PREC_INTERNETCONTROL: u8 = 192u8;
```

### `IPTOS_PREC_CRITIC_ECP`
```rust
const IPTOS_PREC_CRITIC_ECP: u8 = 160u8;
```

### `IPTOS_PREC_FLASHOVERRIDE`
```rust
const IPTOS_PREC_FLASHOVERRIDE: u8 = 128u8;
```

### `IPTOS_PREC_FLASH`
```rust
const IPTOS_PREC_FLASH: u8 = 96u8;
```

### `IPTOS_PREC_IMMEDIATE`
```rust
const IPTOS_PREC_IMMEDIATE: u8 = 64u8;
```

### `IPTOS_PREC_PRIORITY`
```rust
const IPTOS_PREC_PRIORITY: u8 = 32u8;
```

### `IPTOS_PREC_ROUTINE`
```rust
const IPTOS_PREC_ROUTINE: u8 = 0u8;
```

### `IPTOS_ECN_MASK`
```rust
const IPTOS_ECN_MASK: u8 = 3u8;
```

### `IPTOS_ECN_ECT1`
```rust
const IPTOS_ECN_ECT1: u8 = 1u8;
```

### `IPTOS_ECN_ECT0`
```rust
const IPTOS_ECN_ECT0: u8 = 2u8;
```

### `IPTOS_ECN_CE`
```rust
const IPTOS_ECN_CE: u8 = 3u8;
```

### `IPOPT_COPY`
```rust
const IPOPT_COPY: u8 = 128u8;
```

### `IPOPT_CLASS_MASK`
```rust
const IPOPT_CLASS_MASK: u8 = 96u8;
```

### `IPOPT_NUMBER_MASK`
```rust
const IPOPT_NUMBER_MASK: u8 = 31u8;
```

### `IPOPT_CONTROL`
```rust
const IPOPT_CONTROL: u8 = 0u8;
```

### `IPOPT_RESERVED1`
```rust
const IPOPT_RESERVED1: u8 = 32u8;
```

### `IPOPT_MEASUREMENT`
```rust
const IPOPT_MEASUREMENT: u8 = 64u8;
```

### `IPOPT_RESERVED2`
```rust
const IPOPT_RESERVED2: u8 = 96u8;
```

### `IPOPT_END`
```rust
const IPOPT_END: u8 = 0u8;
```

### `IPOPT_NOOP`
```rust
const IPOPT_NOOP: u8 = 1u8;
```

### `IPOPT_SEC`
```rust
const IPOPT_SEC: u8 = 130u8;
```

### `IPOPT_LSRR`
```rust
const IPOPT_LSRR: u8 = 131u8;
```

### `IPOPT_TIMESTAMP`
```rust
const IPOPT_TIMESTAMP: u8 = 68u8;
```

### `IPOPT_RR`
```rust
const IPOPT_RR: u8 = 7u8;
```

### `IPOPT_SID`
```rust
const IPOPT_SID: u8 = 136u8;
```

### `IPOPT_SSRR`
```rust
const IPOPT_SSRR: u8 = 137u8;
```

### `IPOPT_RA`
```rust
const IPOPT_RA: u8 = 148u8;
```

### `IPVERSION`
```rust
const IPVERSION: u8 = 4u8;
```

### `MAXTTL`
```rust
const MAXTTL: u8 = 255u8;
```

### `IPDEFTTL`
```rust
const IPDEFTTL: u8 = 64u8;
```

### `IPOPT_OPTVAL`
```rust
const IPOPT_OPTVAL: u8 = 0u8;
```

### `IPOPT_OLEN`
```rust
const IPOPT_OLEN: u8 = 1u8;
```

### `IPOPT_OFFSET`
```rust
const IPOPT_OFFSET: u8 = 2u8;
```

### `IPOPT_MINOFF`
```rust
const IPOPT_MINOFF: u8 = 4u8;
```

### `MAX_IPOPTLEN`
```rust
const MAX_IPOPTLEN: u8 = 40u8;
```

### `IPOPT_NOP`
```rust
const IPOPT_NOP: u8 = 1u8;
```

### `IPOPT_EOL`
```rust
const IPOPT_EOL: u8 = 0u8;
```

### `IPOPT_TS`
```rust
const IPOPT_TS: u8 = 68u8;
```

### `IPOPT_TS_TSONLY`
```rust
const IPOPT_TS_TSONLY: u8 = 0u8;
```

### `IPOPT_TS_TSANDADDR`
```rust
const IPOPT_TS_TSANDADDR: u8 = 1u8;
```

### `IPOPT_TS_PRESPEC`
```rust
const IPOPT_TS_PRESPEC: u8 = 3u8;
```

### `ARPOP_RREQUEST`
```rust
const ARPOP_RREQUEST: u16 = 3u16;
```

### `ARPOP_RREPLY`
```rust
const ARPOP_RREPLY: u16 = 4u16;
```

### `ARPOP_InREQUEST`
```rust
const ARPOP_InREQUEST: u16 = 8u16;
```

### `ARPOP_InREPLY`
```rust
const ARPOP_InREPLY: u16 = 9u16;
```

### `ARPOP_NAK`
```rust
const ARPOP_NAK: u16 = 10u16;
```

### `ATF_NETMASK`
```rust
const ATF_NETMASK: c_int = 32i32;
```

### `ATF_DONTPUB`
```rust
const ATF_DONTPUB: c_int = 64i32;
```

### `ARPHRD_NETROM`
```rust
const ARPHRD_NETROM: u16 = 0u16;
```

### `ARPHRD_ETHER`
```rust
const ARPHRD_ETHER: u16 = 1u16;
```

### `ARPHRD_EETHER`
```rust
const ARPHRD_EETHER: u16 = 2u16;
```

### `ARPHRD_AX25`
```rust
const ARPHRD_AX25: u16 = 3u16;
```

### `ARPHRD_PRONET`
```rust
const ARPHRD_PRONET: u16 = 4u16;
```

### `ARPHRD_CHAOS`
```rust
const ARPHRD_CHAOS: u16 = 5u16;
```

### `ARPHRD_IEEE802`
```rust
const ARPHRD_IEEE802: u16 = 6u16;
```

### `ARPHRD_ARCNET`
```rust
const ARPHRD_ARCNET: u16 = 7u16;
```

### `ARPHRD_APPLETLK`
```rust
const ARPHRD_APPLETLK: u16 = 8u16;
```

### `ARPHRD_DLCI`
```rust
const ARPHRD_DLCI: u16 = 15u16;
```

### `ARPHRD_ATM`
```rust
const ARPHRD_ATM: u16 = 19u16;
```

### `ARPHRD_METRICOM`
```rust
const ARPHRD_METRICOM: u16 = 23u16;
```

### `ARPHRD_IEEE1394`
```rust
const ARPHRD_IEEE1394: u16 = 24u16;
```

### `ARPHRD_EUI64`
```rust
const ARPHRD_EUI64: u16 = 27u16;
```

### `ARPHRD_INFINIBAND`
```rust
const ARPHRD_INFINIBAND: u16 = 32u16;
```

### `ARPHRD_SLIP`
```rust
const ARPHRD_SLIP: u16 = 256u16;
```

### `ARPHRD_CSLIP`
```rust
const ARPHRD_CSLIP: u16 = 257u16;
```

### `ARPHRD_SLIP6`
```rust
const ARPHRD_SLIP6: u16 = 258u16;
```

### `ARPHRD_CSLIP6`
```rust
const ARPHRD_CSLIP6: u16 = 259u16;
```

### `ARPHRD_RSRVD`
```rust
const ARPHRD_RSRVD: u16 = 260u16;
```

### `ARPHRD_ADAPT`
```rust
const ARPHRD_ADAPT: u16 = 264u16;
```

### `ARPHRD_ROSE`
```rust
const ARPHRD_ROSE: u16 = 270u16;
```

### `ARPHRD_X25`
```rust
const ARPHRD_X25: u16 = 271u16;
```

### `ARPHRD_HWX25`
```rust
const ARPHRD_HWX25: u16 = 272u16;
```

### `ARPHRD_CAN`
```rust
const ARPHRD_CAN: u16 = 280u16;
```

### `ARPHRD_PPP`
```rust
const ARPHRD_PPP: u16 = 512u16;
```

### `ARPHRD_CISCO`
```rust
const ARPHRD_CISCO: u16 = 513u16;
```

### `ARPHRD_HDLC`
```rust
const ARPHRD_HDLC: u16 = 513u16;
```

### `ARPHRD_LAPB`
```rust
const ARPHRD_LAPB: u16 = 516u16;
```

### `ARPHRD_DDCMP`
```rust
const ARPHRD_DDCMP: u16 = 517u16;
```

### `ARPHRD_RAWHDLC`
```rust
const ARPHRD_RAWHDLC: u16 = 518u16;
```

### `ARPHRD_TUNNEL`
```rust
const ARPHRD_TUNNEL: u16 = 768u16;
```

### `ARPHRD_TUNNEL6`
```rust
const ARPHRD_TUNNEL6: u16 = 769u16;
```

### `ARPHRD_FRAD`
```rust
const ARPHRD_FRAD: u16 = 770u16;
```

### `ARPHRD_SKIP`
```rust
const ARPHRD_SKIP: u16 = 771u16;
```

### `ARPHRD_LOOPBACK`
```rust
const ARPHRD_LOOPBACK: u16 = 772u16;
```

### `ARPHRD_LOCALTLK`
```rust
const ARPHRD_LOCALTLK: u16 = 773u16;
```

### `ARPHRD_FDDI`
```rust
const ARPHRD_FDDI: u16 = 774u16;
```

### `ARPHRD_BIF`
```rust
const ARPHRD_BIF: u16 = 775u16;
```

### `ARPHRD_SIT`
```rust
const ARPHRD_SIT: u16 = 776u16;
```

### `ARPHRD_IPDDP`
```rust
const ARPHRD_IPDDP: u16 = 777u16;
```

### `ARPHRD_IPGRE`
```rust
const ARPHRD_IPGRE: u16 = 778u16;
```

### `ARPHRD_PIMREG`
```rust
const ARPHRD_PIMREG: u16 = 779u16;
```

### `ARPHRD_HIPPI`
```rust
const ARPHRD_HIPPI: u16 = 780u16;
```

### `ARPHRD_ASH`
```rust
const ARPHRD_ASH: u16 = 781u16;
```

### `ARPHRD_ECONET`
```rust
const ARPHRD_ECONET: u16 = 782u16;
```

### `ARPHRD_IRDA`
```rust
const ARPHRD_IRDA: u16 = 783u16;
```

### `ARPHRD_FCPP`
```rust
const ARPHRD_FCPP: u16 = 784u16;
```

### `ARPHRD_FCAL`
```rust
const ARPHRD_FCAL: u16 = 785u16;
```

### `ARPHRD_FCPL`
```rust
const ARPHRD_FCPL: u16 = 786u16;
```

### `ARPHRD_FCFABRIC`
```rust
const ARPHRD_FCFABRIC: u16 = 787u16;
```

### `ARPHRD_IEEE802_TR`
```rust
const ARPHRD_IEEE802_TR: u16 = 800u16;
```

### `ARPHRD_IEEE80211`
```rust
const ARPHRD_IEEE80211: u16 = 801u16;
```

### `ARPHRD_IEEE80211_PRISM`
```rust
const ARPHRD_IEEE80211_PRISM: u16 = 802u16;
```

### `ARPHRD_IEEE80211_RADIOTAP`
```rust
const ARPHRD_IEEE80211_RADIOTAP: u16 = 803u16;
```

### `ARPHRD_IEEE802154`
```rust
const ARPHRD_IEEE802154: u16 = 804u16;
```

### `ARPHRD_VOID`
```rust
const ARPHRD_VOID: u16 = 65_535u16;
```

### `ARPHRD_NONE`
```rust
const ARPHRD_NONE: u16 = 65_534u16;
```

### `IFF_TUN`
```rust
const IFF_TUN: c_int = 1i32;
```

### `IFF_TAP`
```rust
const IFF_TAP: c_int = 2i32;
```

### `IFF_NAPI`
```rust
const IFF_NAPI: c_int = 16i32;
```

### `IFF_NAPI_FRAGS`
```rust
const IFF_NAPI_FRAGS: c_int = 32i32;
```

### `IFF_NO_CARRIER`
```rust
const IFF_NO_CARRIER: c_int = 64i32;
```

### `IFF_NO_PI`
```rust
const IFF_NO_PI: c_int = 4_096i32;
```

### `TUN_READQ_SIZE`
```rust
const TUN_READQ_SIZE: c_short = 500i16;
```

### `TUN_TUN_DEV`
```rust
const TUN_TUN_DEV: c_short = 1i16;
```

### `TUN_TAP_DEV`
```rust
const TUN_TAP_DEV: c_short = 2i16;
```

### `TUN_TYPE_MASK`
```rust
const TUN_TYPE_MASK: c_short = 15i16;
```

### `IFF_ONE_QUEUE`
```rust
const IFF_ONE_QUEUE: c_int = 8_192i32;
```

### `IFF_VNET_HDR`
```rust
const IFF_VNET_HDR: c_int = 16_384i32;
```

### `IFF_TUN_EXCL`
```rust
const IFF_TUN_EXCL: c_int = 32_768i32;
```

### `IFF_MULTI_QUEUE`
```rust
const IFF_MULTI_QUEUE: c_int = 256i32;
```

### `IFF_ATTACH_QUEUE`
```rust
const IFF_ATTACH_QUEUE: c_int = 512i32;
```

### `IFF_DETACH_QUEUE`
```rust
const IFF_DETACH_QUEUE: c_int = 1_024i32;
```

### `IFF_PERSIST`
```rust
const IFF_PERSIST: c_int = 2_048i32;
```

### `IFF_NOFILTER`
```rust
const IFF_NOFILTER: c_int = 4_096i32;
```

### `TUN_TX_TIMESTAMP`
```rust
const TUN_TX_TIMESTAMP: c_int = 1i32;
```

### `TUN_F_CSUM`
```rust
const TUN_F_CSUM: c_uint = 1u32;
```

### `TUN_F_TSO4`
```rust
const TUN_F_TSO4: c_uint = 2u32;
```

### `TUN_F_TSO6`
```rust
const TUN_F_TSO6: c_uint = 4u32;
```

### `TUN_F_TSO_ECN`
```rust
const TUN_F_TSO_ECN: c_uint = 8u32;
```

### `TUN_F_UFO`
```rust
const TUN_F_UFO: c_uint = 16u32;
```

### `TUN_F_USO4`
```rust
const TUN_F_USO4: c_uint = 32u32;
```

### `TUN_F_USO6`
```rust
const TUN_F_USO6: c_uint = 64u32;
```

### `TUN_PKT_STRIP`
```rust
const TUN_PKT_STRIP: c_int = 1i32;
```

### `TUN_FLT_ALLMULTI`
```rust
const TUN_FLT_ALLMULTI: c_int = 1i32;
```

### `T_TYPE`
```rust
const T_TYPE: u32 = 84u32;
```

### `TUNSETNOCSUM`
```rust
const TUNSETNOCSUM: c_ulong = 1_074_025_672u64;
```

### `TUNSETDEBUG`
```rust
const TUNSETDEBUG: c_ulong = 1_074_025_673u64;
```

### `TUNSETIFF`
```rust
const TUNSETIFF: c_ulong = 1_074_025_674u64;
```

### `TUNSETPERSIST`
```rust
const TUNSETPERSIST: c_ulong = 1_074_025_675u64;
```

### `TUNSETOWNER`
```rust
const TUNSETOWNER: c_ulong = 1_074_025_676u64;
```

### `TUNSETLINK`
```rust
const TUNSETLINK: c_ulong = 1_074_025_677u64;
```

### `TUNSETGROUP`
```rust
const TUNSETGROUP: c_ulong = 1_074_025_678u64;
```

### `TUNGETFEATURES`
```rust
const TUNGETFEATURES: c_ulong = 2_147_767_503u64;
```

### `TUNSETOFFLOAD`
```rust
const TUNSETOFFLOAD: c_ulong = 1_074_025_680u64;
```

### `TUNSETTXFILTER`
```rust
const TUNSETTXFILTER: c_ulong = 1_074_025_681u64;
```

### `TUNGETIFF`
```rust
const TUNGETIFF: c_ulong = 2_147_767_506u64;
```

### `TUNGETSNDBUF`
```rust
const TUNGETSNDBUF: c_ulong = 2_147_767_507u64;
```

### `TUNSETSNDBUF`
```rust
const TUNSETSNDBUF: c_ulong = 1_074_025_684u64;
```

### `TUNATTACHFILTER`
```rust
const TUNATTACHFILTER: c_ulong = 1_074_812_117u64;
```

### `TUNDETACHFILTER`
```rust
const TUNDETACHFILTER: c_ulong = 1_074_812_118u64;
```

### `TUNGETVNETHDRSZ`
```rust
const TUNGETVNETHDRSZ: c_ulong = 2_147_767_511u64;
```

### `TUNSETVNETHDRSZ`
```rust
const TUNSETVNETHDRSZ: c_ulong = 1_074_025_688u64;
```

### `TUNSETQUEUE`
```rust
const TUNSETQUEUE: c_ulong = 1_074_025_689u64;
```

### `TUNSETIFINDEX`
```rust
const TUNSETIFINDEX: c_ulong = 1_074_025_690u64;
```

### `TUNGETFILTER`
```rust
const TUNGETFILTER: c_ulong = 2_148_553_947u64;
```

### `TUNSETVNETLE`
```rust
const TUNSETVNETLE: c_ulong = 1_074_025_692u64;
```

### `TUNGETVNETLE`
```rust
const TUNGETVNETLE: c_ulong = 2_147_767_517u64;
```

### `TUNSETVNETBE`
```rust
const TUNSETVNETBE: c_ulong = 1_074_025_694u64;
```

### `TUNGETVNETBE`
```rust
const TUNGETVNETBE: c_ulong = 2_147_767_519u64;
```

### `TUNSETSTEERINGEBPF`
```rust
const TUNSETSTEERINGEBPF: c_ulong = 2_147_767_520u64;
```

### `TUNSETFILTEREBPF`
```rust
const TUNSETFILTEREBPF: c_ulong = 2_147_767_521u64;
```

### `TUNSETCARRIER`
```rust
const TUNSETCARRIER: c_ulong = 1_074_025_698u64;
```

### `TUNGETDEVNETNS`
```rust
const TUNGETDEVNETNS: c_ulong = 21_731u64;
```

### `FS_IOC_GETFLAGS`
```rust
const FS_IOC_GETFLAGS: c_ulong = 2_148_034_049u64;
```

### `FS_IOC_SETFLAGS`
```rust
const FS_IOC_SETFLAGS: c_ulong = 1_074_292_226u64;
```

### `FS_IOC_GETVERSION`
```rust
const FS_IOC_GETVERSION: c_ulong = 2_148_038_145u64;
```

### `FS_IOC_SETVERSION`
```rust
const FS_IOC_SETVERSION: c_ulong = 1_074_296_322u64;
```

### `FS_IOC32_GETFLAGS`
```rust
const FS_IOC32_GETFLAGS: c_ulong = 2_147_771_905u64;
```

### `FS_IOC32_SETFLAGS`
```rust
const FS_IOC32_SETFLAGS: c_ulong = 1_074_030_082u64;
```

### `FS_IOC32_GETVERSION`
```rust
const FS_IOC32_GETVERSION: c_ulong = 2_147_776_001u64;
```

### `FS_IOC32_SETVERSION`
```rust
const FS_IOC32_SETVERSION: c_ulong = 1_074_034_178u64;
```

### `FICLONE`
```rust
const FICLONE: c_ulong = 1_074_041_865u64;
```

### `FICLONERANGE`
```rust
const FICLONERANGE: c_ulong = 1_075_876_877u64;
```

### `ADFS_SUPER_MAGIC`
```rust
const ADFS_SUPER_MAGIC: c_long = 44_533i64;
```

### `AFFS_SUPER_MAGIC`
```rust
const AFFS_SUPER_MAGIC: c_long = 44_543i64;
```

### `AFS_SUPER_MAGIC`
```rust
const AFS_SUPER_MAGIC: c_long = 1_397_113_167i64;
```

### `AUTOFS_SUPER_MAGIC`
```rust
const AUTOFS_SUPER_MAGIC: c_long = 391i64;
```

### `BPF_FS_MAGIC`
```rust
const BPF_FS_MAGIC: c_long = 3_405_662_737i64;
```

### `BTRFS_SUPER_MAGIC`
```rust
const BTRFS_SUPER_MAGIC: c_long = 2_435_016_766i64;
```

### `CGROUP2_SUPER_MAGIC`
```rust
const CGROUP2_SUPER_MAGIC: c_long = 1_667_723_888i64;
```

### `CGROUP_SUPER_MAGIC`
```rust
const CGROUP_SUPER_MAGIC: c_long = 2_613_483i64;
```

### `CODA_SUPER_MAGIC`
```rust
const CODA_SUPER_MAGIC: c_long = 1_937_076_805i64;
```

### `CRAMFS_MAGIC`
```rust
const CRAMFS_MAGIC: c_long = 684_539_205i64;
```

### `DEBUGFS_MAGIC`
```rust
const DEBUGFS_MAGIC: c_long = 1_684_170_528i64;
```

### `DEVPTS_SUPER_MAGIC`
```rust
const DEVPTS_SUPER_MAGIC: c_long = 7_377i64;
```

### `ECRYPTFS_SUPER_MAGIC`
```rust
const ECRYPTFS_SUPER_MAGIC: c_long = 61_791i64;
```

### `EFS_SUPER_MAGIC`
```rust
const EFS_SUPER_MAGIC: c_long = 4_278_867i64;
```

### `EXT2_SUPER_MAGIC`
```rust
const EXT2_SUPER_MAGIC: c_long = 61_267i64;
```

### `EXT3_SUPER_MAGIC`
```rust
const EXT3_SUPER_MAGIC: c_long = 61_267i64;
```

### `EXT4_SUPER_MAGIC`
```rust
const EXT4_SUPER_MAGIC: c_long = 61_267i64;
```

### `F2FS_SUPER_MAGIC`
```rust
const F2FS_SUPER_MAGIC: c_long = 4_076_150_800i64;
```

### `FUSE_SUPER_MAGIC`
```rust
const FUSE_SUPER_MAGIC: c_long = 1_702_057_286i64;
```

### `FUTEXFS_SUPER_MAGIC`
```rust
const FUTEXFS_SUPER_MAGIC: c_long = 195_894_762i64;
```

### `HOSTFS_SUPER_MAGIC`
```rust
const HOSTFS_SUPER_MAGIC: c_long = 12_648_430i64;
```

### `HPFS_SUPER_MAGIC`
```rust
const HPFS_SUPER_MAGIC: c_long = 4_187_351_113i64;
```

### `HUGETLBFS_MAGIC`
```rust
const HUGETLBFS_MAGIC: c_long = 2_508_478_710i64;
```

### `ISOFS_SUPER_MAGIC`
```rust
const ISOFS_SUPER_MAGIC: c_long = 38_496i64;
```

### `JFFS2_SUPER_MAGIC`
```rust
const JFFS2_SUPER_MAGIC: c_long = 29_366i64;
```

### `MINIX2_SUPER_MAGIC2`
```rust
const MINIX2_SUPER_MAGIC2: c_long = 9_336i64;
```

### `MINIX2_SUPER_MAGIC`
```rust
const MINIX2_SUPER_MAGIC: c_long = 9_320i64;
```

### `MINIX3_SUPER_MAGIC`
```rust
const MINIX3_SUPER_MAGIC: c_long = 19_802i64;
```

### `MINIX_SUPER_MAGIC2`
```rust
const MINIX_SUPER_MAGIC2: c_long = 5_007i64;
```

### `MINIX_SUPER_MAGIC`
```rust
const MINIX_SUPER_MAGIC: c_long = 4_991i64;
```

### `MSDOS_SUPER_MAGIC`
```rust
const MSDOS_SUPER_MAGIC: c_long = 19_780i64;
```

### `NCP_SUPER_MAGIC`
```rust
const NCP_SUPER_MAGIC: c_long = 22_092i64;
```

### `NFS_SUPER_MAGIC`
```rust
const NFS_SUPER_MAGIC: c_long = 26_985i64;
```

### `NILFS_SUPER_MAGIC`
```rust
const NILFS_SUPER_MAGIC: c_long = 13_364i64;
```

### `OCFS2_SUPER_MAGIC`
```rust
const OCFS2_SUPER_MAGIC: c_long = 1_952_539_503i64;
```

### `OPENPROM_SUPER_MAGIC`
```rust
const OPENPROM_SUPER_MAGIC: c_long = 40_865i64;
```

### `OVERLAYFS_SUPER_MAGIC`
```rust
const OVERLAYFS_SUPER_MAGIC: c_long = 2_035_054_128i64;
```

### `PROC_SUPER_MAGIC`
```rust
const PROC_SUPER_MAGIC: c_long = 40_864i64;
```

### `QNX4_SUPER_MAGIC`
```rust
const QNX4_SUPER_MAGIC: c_long = 47i64;
```

### `QNX6_SUPER_MAGIC`
```rust
const QNX6_SUPER_MAGIC: c_long = 1_746_473_250i64;
```

### `RDTGROUP_SUPER_MAGIC`
```rust
const RDTGROUP_SUPER_MAGIC: c_long = 124_082_209i64;
```

### `REISERFS_SUPER_MAGIC`
```rust
const REISERFS_SUPER_MAGIC: c_long = 1_382_369_651i64;
```

### `SECURITYFS_MAGIC`
```rust
const SECURITYFS_MAGIC: c_long = 1_935_894_131i64;
```

### `SELINUX_MAGIC`
```rust
const SELINUX_MAGIC: c_long = 4_185_718_668i64;
```

### `SMACK_MAGIC`
```rust
const SMACK_MAGIC: c_long = 1_128_357_203i64;
```

### `SMB_SUPER_MAGIC`
```rust
const SMB_SUPER_MAGIC: c_long = 20_859i64;
```

### `SYSFS_MAGIC`
```rust
const SYSFS_MAGIC: c_long = 1_650_812_274i64;
```

### `TMPFS_MAGIC`
```rust
const TMPFS_MAGIC: c_long = 16_914_836i64;
```

### `TRACEFS_MAGIC`
```rust
const TRACEFS_MAGIC: c_long = 1_953_653_091i64;
```

### `UDF_SUPER_MAGIC`
```rust
const UDF_SUPER_MAGIC: c_long = 352_400_198i64;
```

### `USBDEVICE_SUPER_MAGIC`
```rust
const USBDEVICE_SUPER_MAGIC: c_long = 40_866i64;
```

### `XENFS_SUPER_MAGIC`
```rust
const XENFS_SUPER_MAGIC: c_long = 2_881_100_148i64;
```

### `NSFS_MAGIC`
```rust
const NSFS_MAGIC: c_long = 1_853_056_627i64;
```

### `AT_STATX_SYNC_TYPE`
```rust
const AT_STATX_SYNC_TYPE: c_int = 24_576i32;
```

### `AT_STATX_SYNC_AS_STAT`
```rust
const AT_STATX_SYNC_AS_STAT: c_int = 0i32;
```

### `AT_STATX_FORCE_SYNC`
```rust
const AT_STATX_FORCE_SYNC: c_int = 8_192i32;
```

### `AT_STATX_DONT_SYNC`
```rust
const AT_STATX_DONT_SYNC: c_int = 16_384i32;
```

### `STATX_TYPE`
```rust
const STATX_TYPE: c_uint = 1u32;
```

### `STATX_MODE`
```rust
const STATX_MODE: c_uint = 2u32;
```

### `STATX_NLINK`
```rust
const STATX_NLINK: c_uint = 4u32;
```

### `STATX_UID`
```rust
const STATX_UID: c_uint = 8u32;
```

### `STATX_GID`
```rust
const STATX_GID: c_uint = 16u32;
```

### `STATX_ATIME`
```rust
const STATX_ATIME: c_uint = 32u32;
```

### `STATX_MTIME`
```rust
const STATX_MTIME: c_uint = 64u32;
```

### `STATX_CTIME`
```rust
const STATX_CTIME: c_uint = 128u32;
```

### `STATX_INO`
```rust
const STATX_INO: c_uint = 256u32;
```

### `STATX_SIZE`
```rust
const STATX_SIZE: c_uint = 512u32;
```

### `STATX_BLOCKS`
```rust
const STATX_BLOCKS: c_uint = 1_024u32;
```

### `STATX_BASIC_STATS`
```rust
const STATX_BASIC_STATS: c_uint = 2_047u32;
```

### `STATX_BTIME`
```rust
const STATX_BTIME: c_uint = 2_048u32;
```

### `STATX_ALL`
```rust
const STATX_ALL: c_uint = 4_095u32;
```

### `STATX_MNT_ID`
```rust
const STATX_MNT_ID: c_uint = 4_096u32;
```

### `STATX_DIOALIGN`
```rust
const STATX_DIOALIGN: c_uint = 8_192u32;
```

### `STATX__RESERVED`
```rust
const STATX__RESERVED: c_int = -2_147_483_648i32;
```

### `STATX_ATTR_COMPRESSED`
```rust
const STATX_ATTR_COMPRESSED: c_int = 4i32;
```

### `STATX_ATTR_IMMUTABLE`
```rust
const STATX_ATTR_IMMUTABLE: c_int = 16i32;
```

### `STATX_ATTR_APPEND`
```rust
const STATX_ATTR_APPEND: c_int = 32i32;
```

### `STATX_ATTR_NODUMP`
```rust
const STATX_ATTR_NODUMP: c_int = 64i32;
```

### `STATX_ATTR_ENCRYPTED`
```rust
const STATX_ATTR_ENCRYPTED: c_int = 2_048i32;
```

### `STATX_ATTR_AUTOMOUNT`
```rust
const STATX_ATTR_AUTOMOUNT: c_int = 4_096i32;
```

### `STATX_ATTR_MOUNT_ROOT`
```rust
const STATX_ATTR_MOUNT_ROOT: c_int = 8_192i32;
```

### `STATX_ATTR_VERITY`
```rust
const STATX_ATTR_VERITY: c_int = 1_048_576i32;
```

### `STATX_ATTR_DAX`
```rust
const STATX_ATTR_DAX: c_int = 2_097_152i32;
```

### `_IOC_NRBITS`
```rust
const _IOC_NRBITS: u32 = 8u32;
```

### `_IOC_TYPEBITS`
```rust
const _IOC_TYPEBITS: u32 = 8u32;
```

### `_IOC_SIZEBITS`
```rust
const _IOC_SIZEBITS: u32 = 14u32;
```

### `_IOC_DIRBITS`
```rust
const _IOC_DIRBITS: u32 = 2u32;
```

### `_IOC_NONE`
```rust
const _IOC_NONE: u32 = 0u32;
```

### `_IOC_WRITE`
```rust
const _IOC_WRITE: u32 = 1u32;
```

### `_IOC_READ`
```rust
const _IOC_READ: u32 = 2u32;
```

### `_IOC_NRMASK`
```rust
const _IOC_NRMASK: u32 = 255u32;
```

### `_IOC_TYPEMASK`
```rust
const _IOC_TYPEMASK: u32 = 255u32;
```

### `_IOC_SIZEMASK`
```rust
const _IOC_SIZEMASK: u32 = 16_383u32;
```

### `_IOC_DIRMASK`
```rust
const _IOC_DIRMASK: u32 = 3u32;
```

### `_IOC_NRSHIFT`
```rust
const _IOC_NRSHIFT: u32 = 0u32;
```

### `_IOC_TYPESHIFT`
```rust
const _IOC_TYPESHIFT: u32 = 8u32;
```

### `_IOC_SIZESHIFT`
```rust
const _IOC_SIZESHIFT: u32 = 16u32;
```

### `_IOC_DIRSHIFT`
```rust
const _IOC_DIRSHIFT: u32 = 30u32;
```

### `PIDTYPE_PID`
```rust
const PIDTYPE_PID: pid_type = 0u32;
```

### `PIDTYPE_TGID`
```rust
const PIDTYPE_TGID: pid_type = 1u32;
```

### `PIDTYPE_PGID`
```rust
const PIDTYPE_PGID: pid_type = 2u32;
```

### `PIDTYPE_SID`
```rust
const PIDTYPE_SID: pid_type = 3u32;
```

### `PIDTYPE_MAX`
```rust
const PIDTYPE_MAX: pid_type = 4u32;
```

### `POSIX_SPAWN_USEVFORK`
```rust
const POSIX_SPAWN_USEVFORK: c_int = 64i32;
```

### `POSIX_SPAWN_SETSID`
```rust
const POSIX_SPAWN_SETSID: c_int = 128i32;
```

### `F_SEAL_FUTURE_WRITE`
```rust
const F_SEAL_FUTURE_WRITE: c_int = 16i32;
```

### `F_SEAL_EXEC`
```rust
const F_SEAL_EXEC: c_int = 32i32;
```

### `IFF_LOWER_UP`
```rust
const IFF_LOWER_UP: c_int = 65_536i32;
```

### `IFF_DORMANT`
```rust
const IFF_DORMANT: c_int = 131_072i32;
```

### `IFF_ECHO`
```rust
const IFF_ECHO: c_int = 262_144i32;
```

### `AT_EXECVE_CHECK`
```rust
const AT_EXECVE_CHECK: c_int = 65_536i32;
```

### `MAX_HANDLE_SZ`
```rust
const MAX_HANDLE_SZ: c_int = 128i32;
```

### `AT_HANDLE_FID`
```rust
const AT_HANDLE_FID: c_int = 512i32;
```

### `AT_HANDLE_MNT_ID_UNIQUE`
```rust
const AT_HANDLE_MNT_ID_UNIQUE: c_int = 1i32;
```

### `AT_HANDLE_CONNECTABLE`
```rust
const AT_HANDLE_CONNECTABLE: c_int = 2i32;
```

### `IFA_UNSPEC`
```rust
const IFA_UNSPEC: c_ushort = 0u16;
```

### `IFA_ADDRESS`
```rust
const IFA_ADDRESS: c_ushort = 1u16;
```

### `IFA_LOCAL`
```rust
const IFA_LOCAL: c_ushort = 2u16;
```

### `IFA_LABEL`
```rust
const IFA_LABEL: c_ushort = 3u16;
```

### `IFA_BROADCAST`
```rust
const IFA_BROADCAST: c_ushort = 4u16;
```

### `IFA_ANYCAST`
```rust
const IFA_ANYCAST: c_ushort = 5u16;
```

### `IFA_CACHEINFO`
```rust
const IFA_CACHEINFO: c_ushort = 6u16;
```

### `IFA_MULTICAST`
```rust
const IFA_MULTICAST: c_ushort = 7u16;
```

### `IFA_FLAGS`
```rust
const IFA_FLAGS: c_ushort = 8u16;
```

### `IFA_F_SECONDARY`
```rust
const IFA_F_SECONDARY: u32 = 1u32;
```

### `IFA_F_TEMPORARY`
```rust
const IFA_F_TEMPORARY: u32 = 1u32;
```

### `IFA_F_NODAD`
```rust
const IFA_F_NODAD: u32 = 2u32;
```

### `IFA_F_OPTIMISTIC`
```rust
const IFA_F_OPTIMISTIC: u32 = 4u32;
```

### `IFA_F_DADFAILED`
```rust
const IFA_F_DADFAILED: u32 = 8u32;
```

### `IFA_F_HOMEADDRESS`
```rust
const IFA_F_HOMEADDRESS: u32 = 16u32;
```

### `IFA_F_DEPRECATED`
```rust
const IFA_F_DEPRECATED: u32 = 32u32;
```

### `IFA_F_TENTATIVE`
```rust
const IFA_F_TENTATIVE: u32 = 64u32;
```

### `IFA_F_PERMANENT`
```rust
const IFA_F_PERMANENT: u32 = 128u32;
```

### `IFA_F_MANAGETEMPADDR`
```rust
const IFA_F_MANAGETEMPADDR: u32 = 256u32;
```

### `IFA_F_NOPREFIXROUTE`
```rust
const IFA_F_NOPREFIXROUTE: u32 = 512u32;
```

### `IFA_F_MCAUTOJOIN`
```rust
const IFA_F_MCAUTOJOIN: u32 = 1_024u32;
```

### `IFA_F_STABLE_PRIVACY`
```rust
const IFA_F_STABLE_PRIVACY: u32 = 2_048u32;
```

### `RWF_HIPRI`
```rust
const RWF_HIPRI: c_int = 1i32;
```

### `RWF_DSYNC`
```rust
const RWF_DSYNC: c_int = 2i32;
```

### `RWF_SYNC`
```rust
const RWF_SYNC: c_int = 4i32;
```

### `RWF_NOWAIT`
```rust
const RWF_NOWAIT: c_int = 8i32;
```

### `RWF_APPEND`
```rust
const RWF_APPEND: c_int = 16i32;
```

### `RWF_NOAPPEND`
```rust
const RWF_NOAPPEND: c_int = 32i32;
```

### `RWF_ATOMIC`
```rust
const RWF_ATOMIC: c_int = 64i32;
```

### `RWF_DONTCACHE`
```rust
const RWF_DONTCACHE: c_int = 128i32;
```

### `IFLA_UNSPEC`
```rust
const IFLA_UNSPEC: c_ushort = 0u16;
```

### `IFLA_ADDRESS`
```rust
const IFLA_ADDRESS: c_ushort = 1u16;
```

### `IFLA_BROADCAST`
```rust
const IFLA_BROADCAST: c_ushort = 2u16;
```

### `IFLA_IFNAME`
```rust
const IFLA_IFNAME: c_ushort = 3u16;
```

### `IFLA_MTU`
```rust
const IFLA_MTU: c_ushort = 4u16;
```

### `IFLA_LINK`
```rust
const IFLA_LINK: c_ushort = 5u16;
```

### `IFLA_QDISC`
```rust
const IFLA_QDISC: c_ushort = 6u16;
```

### `IFLA_STATS`
```rust
const IFLA_STATS: c_ushort = 7u16;
```

### `IFLA_COST`
```rust
const IFLA_COST: c_ushort = 8u16;
```

### `IFLA_PRIORITY`
```rust
const IFLA_PRIORITY: c_ushort = 9u16;
```

### `IFLA_MASTER`
```rust
const IFLA_MASTER: c_ushort = 10u16;
```

### `IFLA_WIRELESS`
```rust
const IFLA_WIRELESS: c_ushort = 11u16;
```

### `IFLA_PROTINFO`
```rust
const IFLA_PROTINFO: c_ushort = 12u16;
```

### `IFLA_TXQLEN`
```rust
const IFLA_TXQLEN: c_ushort = 13u16;
```

### `IFLA_MAP`
```rust
const IFLA_MAP: c_ushort = 14u16;
```

### `IFLA_WEIGHT`
```rust
const IFLA_WEIGHT: c_ushort = 15u16;
```

### `IFLA_OPERSTATE`
```rust
const IFLA_OPERSTATE: c_ushort = 16u16;
```

### `IFLA_LINKMODE`
```rust
const IFLA_LINKMODE: c_ushort = 17u16;
```

### `IFLA_LINKINFO`
```rust
const IFLA_LINKINFO: c_ushort = 18u16;
```

### `IFLA_NET_NS_PID`
```rust
const IFLA_NET_NS_PID: c_ushort = 19u16;
```

### `IFLA_IFALIAS`
```rust
const IFLA_IFALIAS: c_ushort = 20u16;
```

### `IFLA_NUM_VF`
```rust
const IFLA_NUM_VF: c_ushort = 21u16;
```

### `IFLA_VFINFO_LIST`
```rust
const IFLA_VFINFO_LIST: c_ushort = 22u16;
```

### `IFLA_STATS64`
```rust
const IFLA_STATS64: c_ushort = 23u16;
```

### `IFLA_VF_PORTS`
```rust
const IFLA_VF_PORTS: c_ushort = 24u16;
```

### `IFLA_PORT_SELF`
```rust
const IFLA_PORT_SELF: c_ushort = 25u16;
```

### `IFLA_AF_SPEC`
```rust
const IFLA_AF_SPEC: c_ushort = 26u16;
```

### `IFLA_GROUP`
```rust
const IFLA_GROUP: c_ushort = 27u16;
```

### `IFLA_NET_NS_FD`
```rust
const IFLA_NET_NS_FD: c_ushort = 28u16;
```

### `IFLA_EXT_MASK`
```rust
const IFLA_EXT_MASK: c_ushort = 29u16;
```

### `IFLA_PROMISCUITY`
```rust
const IFLA_PROMISCUITY: c_ushort = 30u16;
```

### `IFLA_NUM_TX_QUEUES`
```rust
const IFLA_NUM_TX_QUEUES: c_ushort = 31u16;
```

### `IFLA_NUM_RX_QUEUES`
```rust
const IFLA_NUM_RX_QUEUES: c_ushort = 32u16;
```

### `IFLA_CARRIER`
```rust
const IFLA_CARRIER: c_ushort = 33u16;
```

### `IFLA_PHYS_PORT_ID`
```rust
const IFLA_PHYS_PORT_ID: c_ushort = 34u16;
```

### `IFLA_CARRIER_CHANGES`
```rust
const IFLA_CARRIER_CHANGES: c_ushort = 35u16;
```

### `IFLA_PHYS_SWITCH_ID`
```rust
const IFLA_PHYS_SWITCH_ID: c_ushort = 36u16;
```

### `IFLA_LINK_NETNSID`
```rust
const IFLA_LINK_NETNSID: c_ushort = 37u16;
```

### `IFLA_PHYS_PORT_NAME`
```rust
const IFLA_PHYS_PORT_NAME: c_ushort = 38u16;
```

### `IFLA_PROTO_DOWN`
```rust
const IFLA_PROTO_DOWN: c_ushort = 39u16;
```

### `IFLA_GSO_MAX_SEGS`
```rust
const IFLA_GSO_MAX_SEGS: c_ushort = 40u16;
```

### `IFLA_GSO_MAX_SIZE`
```rust
const IFLA_GSO_MAX_SIZE: c_ushort = 41u16;
```

### `IFLA_PAD`
```rust
const IFLA_PAD: c_ushort = 42u16;
```

### `IFLA_XDP`
```rust
const IFLA_XDP: c_ushort = 43u16;
```

### `IFLA_EVENT`
```rust
const IFLA_EVENT: c_ushort = 44u16;
```

### `IFLA_NEW_NETNSID`
```rust
const IFLA_NEW_NETNSID: c_ushort = 45u16;
```

### `IFLA_IF_NETNSID`
```rust
const IFLA_IF_NETNSID: c_ushort = 46u16;
```

### `IFLA_TARGET_NETNSID`
```rust
const IFLA_TARGET_NETNSID: c_ushort = 46u16;
```

### `IFLA_CARRIER_UP_COUNT`
```rust
const IFLA_CARRIER_UP_COUNT: c_ushort = 47u16;
```

### `IFLA_CARRIER_DOWN_COUNT`
```rust
const IFLA_CARRIER_DOWN_COUNT: c_ushort = 48u16;
```

### `IFLA_NEW_IFINDEX`
```rust
const IFLA_NEW_IFINDEX: c_ushort = 49u16;
```

### `IFLA_MIN_MTU`
```rust
const IFLA_MIN_MTU: c_ushort = 50u16;
```

### `IFLA_MAX_MTU`
```rust
const IFLA_MAX_MTU: c_ushort = 51u16;
```

### `IFLA_PROP_LIST`
```rust
const IFLA_PROP_LIST: c_ushort = 52u16;
```

### `IFLA_ALT_IFNAME`
```rust
const IFLA_ALT_IFNAME: c_ushort = 53u16;
```

### `IFLA_PERM_ADDRESS`
```rust
const IFLA_PERM_ADDRESS: c_ushort = 54u16;
```

### `IFLA_PROTO_DOWN_REASON`
```rust
const IFLA_PROTO_DOWN_REASON: c_ushort = 55u16;
```

### `IFLA_PARENT_DEV_NAME`
```rust
const IFLA_PARENT_DEV_NAME: c_ushort = 56u16;
```

### `IFLA_PARENT_DEV_BUS_NAME`
```rust
const IFLA_PARENT_DEV_BUS_NAME: c_ushort = 57u16;
```

### `IFLA_GRO_MAX_SIZE`
```rust
const IFLA_GRO_MAX_SIZE: c_ushort = 58u16;
```

### `IFLA_TSO_MAX_SIZE`
```rust
const IFLA_TSO_MAX_SIZE: c_ushort = 59u16;
```

### `IFLA_TSO_MAX_SEGS`
```rust
const IFLA_TSO_MAX_SEGS: c_ushort = 60u16;
```

### `IFLA_ALLMULTI`
```rust
const IFLA_ALLMULTI: c_ushort = 61u16;
```

### `IFLA_INFO_UNSPEC`
```rust
const IFLA_INFO_UNSPEC: c_ushort = 0u16;
```

### `IFLA_INFO_KIND`
```rust
const IFLA_INFO_KIND: c_ushort = 1u16;
```

### `IFLA_INFO_DATA`
```rust
const IFLA_INFO_DATA: c_ushort = 2u16;
```

### `IFLA_INFO_XSTATS`
```rust
const IFLA_INFO_XSTATS: c_ushort = 3u16;
```

### `IFLA_INFO_SLAVE_KIND`
```rust
const IFLA_INFO_SLAVE_KIND: c_ushort = 4u16;
```

### `IFLA_INFO_SLAVE_DATA`
```rust
const IFLA_INFO_SLAVE_DATA: c_ushort = 5u16;
```

### `SEEK_DATA`
```rust
const SEEK_DATA: c_int = 3i32;
```

### `SEEK_HOLE`
```rust
const SEEK_HOLE: c_int = 4i32;
```

### `MPOL_DEFAULT`
```rust
const MPOL_DEFAULT: c_int = 0i32;
```

### `MPOL_PREFERRED`
```rust
const MPOL_PREFERRED: c_int = 1i32;
```

### `MPOL_BIND`
```rust
const MPOL_BIND: c_int = 2i32;
```

### `MPOL_INTERLEAVE`
```rust
const MPOL_INTERLEAVE: c_int = 3i32;
```

### `MPOL_LOCAL`
```rust
const MPOL_LOCAL: c_int = 4i32;
```

### `MPOL_F_NUMA_BALANCING`
```rust
const MPOL_F_NUMA_BALANCING: c_int = 8_192i32;
```

### `MPOL_F_RELATIVE_NODES`
```rust
const MPOL_F_RELATIVE_NODES: c_int = 16_384i32;
```

### `MPOL_F_STATIC_NODES`
```rust
const MPOL_F_STATIC_NODES: c_int = 32_768i32;
```

### `PTHREAD_MUTEX_INITIALIZER`
```rust
const PTHREAD_MUTEX_INITIALIZER: crate::pthread_mutex_t;
```

### `PTHREAD_COND_INITIALIZER`
```rust
const PTHREAD_COND_INITIALIZER: crate::pthread_cond_t;
```

### `PTHREAD_RWLOCK_INITIALIZER`
```rust
const PTHREAD_RWLOCK_INITIALIZER: crate::pthread_rwlock_t;
```

### `RENAME_NOREPLACE`
```rust
const RENAME_NOREPLACE: c_uint = 1u32;
```

### `RENAME_EXCHANGE`
```rust
const RENAME_EXCHANGE: c_uint = 2u32;
```

### `RENAME_WHITEOUT`
```rust
const RENAME_WHITEOUT: c_uint = 4u32;
```

### `MSG_STAT`
```rust
const MSG_STAT: c_int = 11i32;
```

### `MSG_INFO`
```rust
const MSG_INFO: c_int = 12i32;
```

### `MSG_NOTIFICATION`
```rust
const MSG_NOTIFICATION: c_int = 32_768i32;
```

### `MSG_NOERROR`
```rust
const MSG_NOERROR: c_int = 4_096i32;
```

### `MSG_EXCEPT`
```rust
const MSG_EXCEPT: c_int = 8_192i32;
```

### `MSG_ZEROCOPY`
```rust
const MSG_ZEROCOPY: c_int = 67_108_864i32;
```

### `SEM_UNDO`
```rust
const SEM_UNDO: c_int = 4_096i32;
```

### `GETPID`
```rust
const GETPID: c_int = 11i32;
```

### `GETVAL`
```rust
const GETVAL: c_int = 12i32;
```

### `GETALL`
```rust
const GETALL: c_int = 13i32;
```

### `GETNCNT`
```rust
const GETNCNT: c_int = 14i32;
```

### `GETZCNT`
```rust
const GETZCNT: c_int = 15i32;
```

### `SETVAL`
```rust
const SETVAL: c_int = 16i32;
```

### `SETALL`
```rust
const SETALL: c_int = 17i32;
```

### `SEM_STAT`
```rust
const SEM_STAT: c_int = 18i32;
```

### `SEM_INFO`
```rust
const SEM_INFO: c_int = 19i32;
```

### `SEM_STAT_ANY`
```rust
const SEM_STAT_ANY: c_int = 20i32;
```

### `QFMT_VFS_OLD`
```rust
const QFMT_VFS_OLD: c_int = 1i32;
```

### `QFMT_VFS_V0`
```rust
const QFMT_VFS_V0: c_int = 2i32;
```

### `QFMT_VFS_V1`
```rust
const QFMT_VFS_V1: c_int = 4i32;
```

### `EFD_SEMAPHORE`
```rust
const EFD_SEMAPHORE: c_int = 1i32;
```

### `RB_AUTOBOOT`
```rust
const RB_AUTOBOOT: c_int = 19_088_743i32;
```

### `RB_HALT_SYSTEM`
```rust
const RB_HALT_SYSTEM: c_int = -839_974_621i32;
```

### `RB_ENABLE_CAD`
```rust
const RB_ENABLE_CAD: c_int = -1_985_229_329i32;
```

### `RB_DISABLE_CAD`
```rust
const RB_DISABLE_CAD: c_int = 0i32;
```

### `RB_POWER_OFF`
```rust
const RB_POWER_OFF: c_int = 1_126_301_404i32;
```

### `RB_SW_SUSPEND`
```rust
const RB_SW_SUSPEND: c_int = -805_241_630i32;
```

### `RB_KEXEC`
```rust
const RB_KEXEC: c_int = 1_163_412_803i32;
```

### `SYNC_FILE_RANGE_WAIT_BEFORE`
```rust
const SYNC_FILE_RANGE_WAIT_BEFORE: c_uint = 1u32;
```

### `SYNC_FILE_RANGE_WRITE`
```rust
const SYNC_FILE_RANGE_WRITE: c_uint = 2u32;
```

### `SYNC_FILE_RANGE_WAIT_AFTER`
```rust
const SYNC_FILE_RANGE_WAIT_AFTER: c_uint = 4u32;
```

### `MREMAP_MAYMOVE`
```rust
const MREMAP_MAYMOVE: c_int = 1i32;
```

### `MREMAP_FIXED`
```rust
const MREMAP_FIXED: c_int = 2i32;
```

### `MREMAP_DONTUNMAP`
```rust
const MREMAP_DONTUNMAP: c_int = 4i32;
```

### `NSIO`
```rust
const NSIO: c_uint = 183u32;
```

### `NS_GET_USERNS`
```rust
const NS_GET_USERNS: c_ulong = 46_849u64;
```

### `NS_GET_PARENT`
```rust
const NS_GET_PARENT: c_ulong = 46_850u64;
```

### `NS_GET_NSTYPE`
```rust
const NS_GET_NSTYPE: c_ulong = 46_851u64;
```

### `NS_GET_OWNER_UID`
```rust
const NS_GET_OWNER_UID: c_ulong = 46_852u64;
```

### `NS_GET_MNTNS_ID`
```rust
const NS_GET_MNTNS_ID: c_ulong = 2_148_054_789u64;
```

### `NS_GET_PID_FROM_PIDNS`
```rust
const NS_GET_PID_FROM_PIDNS: c_ulong = 2_147_792_646u64;
```

### `NS_GET_TGID_FROM_PIDNS`
```rust
const NS_GET_TGID_FROM_PIDNS: c_ulong = 2_147_792_647u64;
```

### `NS_GET_PID_IN_PIDNS`
```rust
const NS_GET_PID_IN_PIDNS: c_ulong = 2_147_792_648u64;
```

### `NS_GET_TGID_IN_PIDNS`
```rust
const NS_GET_TGID_IN_PIDNS: c_ulong = 2_147_792_649u64;
```

### `MNT_NS_INFO_SIZE_VER0`
```rust
const MNT_NS_INFO_SIZE_VER0: c_ulong = 16u64;
```

### `NS_MNT_GET_INFO`
```rust
const NS_MNT_GET_INFO: c_ulong = 2_148_579_082u64;
```

### `NS_MNT_GET_NEXT`
```rust
const NS_MNT_GET_NEXT: c_ulong = 2_148_579_083u64;
```

### `NS_MNT_GET_PREV`
```rust
const NS_MNT_GET_PREV: c_ulong = 2_148_579_084u64;
```

### `PR_SET_MDWE`
```rust
const PR_SET_MDWE: c_int = 65i32;
```

### `PR_GET_MDWE`
```rust
const PR_GET_MDWE: c_int = 66i32;
```

### `PR_MDWE_REFUSE_EXEC_GAIN`
```rust
const PR_MDWE_REFUSE_EXEC_GAIN: c_uint = 1u32;
```

### `PR_MDWE_NO_INHERIT`
```rust
const PR_MDWE_NO_INHERIT: c_uint = 2u32;
```

### `GRND_NONBLOCK`
```rust
const GRND_NONBLOCK: c_uint = 1u32;
```

### `GRND_RANDOM`
```rust
const GRND_RANDOM: c_uint = 2u32;
```

### `GRND_INSECURE`
```rust
const GRND_INSECURE: c_uint = 4u32;
```

### `SECCOMP_MODE_DISABLED`
```rust
const SECCOMP_MODE_DISABLED: c_uint = 0u32;
```

### `SECCOMP_MODE_STRICT`
```rust
const SECCOMP_MODE_STRICT: c_uint = 1u32;
```

### `SECCOMP_MODE_FILTER`
```rust
const SECCOMP_MODE_FILTER: c_uint = 2u32;
```

### `SECCOMP_SET_MODE_STRICT`
```rust
const SECCOMP_SET_MODE_STRICT: c_uint = 0u32;
```

### `SECCOMP_SET_MODE_FILTER`
```rust
const SECCOMP_SET_MODE_FILTER: c_uint = 1u32;
```

### `SECCOMP_GET_ACTION_AVAIL`
```rust
const SECCOMP_GET_ACTION_AVAIL: c_uint = 2u32;
```

### `SECCOMP_GET_NOTIF_SIZES`
```rust
const SECCOMP_GET_NOTIF_SIZES: c_uint = 3u32;
```

### `SECCOMP_FILTER_FLAG_TSYNC`
```rust
const SECCOMP_FILTER_FLAG_TSYNC: c_ulong = 1u64;
```

### `SECCOMP_FILTER_FLAG_LOG`
```rust
const SECCOMP_FILTER_FLAG_LOG: c_ulong = 2u64;
```

### `SECCOMP_FILTER_FLAG_SPEC_ALLOW`
```rust
const SECCOMP_FILTER_FLAG_SPEC_ALLOW: c_ulong = 4u64;
```

### `SECCOMP_FILTER_FLAG_NEW_LISTENER`
```rust
const SECCOMP_FILTER_FLAG_NEW_LISTENER: c_ulong = 8u64;
```

### `SECCOMP_FILTER_FLAG_TSYNC_ESRCH`
```rust
const SECCOMP_FILTER_FLAG_TSYNC_ESRCH: c_ulong = 16u64;
```

### `SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV`
```rust
const SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV: c_ulong = 32u64;
```

### `SECCOMP_RET_KILL_PROCESS`
```rust
const SECCOMP_RET_KILL_PROCESS: c_uint = 2_147_483_648u32;
```

### `SECCOMP_RET_KILL_THREAD`
```rust
const SECCOMP_RET_KILL_THREAD: c_uint = 0u32;
```

### `SECCOMP_RET_KILL`
```rust
const SECCOMP_RET_KILL: c_uint = 0u32;
```

### `SECCOMP_RET_TRAP`
```rust
const SECCOMP_RET_TRAP: c_uint = 196_608u32;
```

### `SECCOMP_RET_ERRNO`
```rust
const SECCOMP_RET_ERRNO: c_uint = 327_680u32;
```

### `SECCOMP_RET_USER_NOTIF`
```rust
const SECCOMP_RET_USER_NOTIF: c_uint = 2_143_289_344u32;
```

### `SECCOMP_RET_TRACE`
```rust
const SECCOMP_RET_TRACE: c_uint = 2_146_435_072u32;
```

### `SECCOMP_RET_LOG`
```rust
const SECCOMP_RET_LOG: c_uint = 2_147_221_504u32;
```

### `SECCOMP_RET_ALLOW`
```rust
const SECCOMP_RET_ALLOW: c_uint = 2_147_418_112u32;
```

### `SECCOMP_RET_ACTION_FULL`
```rust
const SECCOMP_RET_ACTION_FULL: c_uint = 4_294_901_760u32;
```

### `SECCOMP_RET_ACTION`
```rust
const SECCOMP_RET_ACTION: c_uint = 2_147_418_112u32;
```

### `SECCOMP_RET_DATA`
```rust
const SECCOMP_RET_DATA: c_uint = 65_535u32;
```

### `SECCOMP_USER_NOTIF_FLAG_CONTINUE`
```rust
const SECCOMP_USER_NOTIF_FLAG_CONTINUE: c_ulong = 1u64;
```

### `SECCOMP_ADDFD_FLAG_SETFD`
```rust
const SECCOMP_ADDFD_FLAG_SETFD: c_ulong = 1u64;
```

### `SECCOMP_ADDFD_FLAG_SEND`
```rust
const SECCOMP_ADDFD_FLAG_SEND: c_ulong = 2u64;
```

### `TFD_CLOEXEC`
```rust
const TFD_CLOEXEC: c_int = 524_288i32;
```

### `TFD_NONBLOCK`
```rust
const TFD_NONBLOCK: c_int = 2_048i32;
```

### `TFD_TIMER_ABSTIME`
```rust
const TFD_TIMER_ABSTIME: c_int = 1i32;
```

### `TFD_TIMER_CANCEL_ON_SET`
```rust
const TFD_TIMER_CANCEL_ON_SET: c_int = 2i32;
```

### `FALLOC_FL_KEEP_SIZE`
```rust
const FALLOC_FL_KEEP_SIZE: c_int = 1i32;
```

### `FALLOC_FL_PUNCH_HOLE`
```rust
const FALLOC_FL_PUNCH_HOLE: c_int = 2i32;
```

### `FALLOC_FL_COLLAPSE_RANGE`
```rust
const FALLOC_FL_COLLAPSE_RANGE: c_int = 8i32;
```

### `FALLOC_FL_ZERO_RANGE`
```rust
const FALLOC_FL_ZERO_RANGE: c_int = 16i32;
```

### `FALLOC_FL_INSERT_RANGE`
```rust
const FALLOC_FL_INSERT_RANGE: c_int = 32i32;
```

### `FALLOC_FL_UNSHARE_RANGE`
```rust
const FALLOC_FL_UNSHARE_RANGE: c_int = 64i32;
```

### `ENOATTR`
```rust
const ENOATTR: c_int = 61i32;
```

### `SO_ORIGINAL_DST`
```rust
const SO_ORIGINAL_DST: c_int = 80i32;
```

### `IP_RECVFRAGSIZE`
```rust
const IP_RECVFRAGSIZE: c_int = 25i32;
```

### `IPV6_FLOWINFO`
```rust
const IPV6_FLOWINFO: c_int = 11i32;
```

### `IPV6_FLOWLABEL_MGR`
```rust
const IPV6_FLOWLABEL_MGR: c_int = 32i32;
```

### `IPV6_FLOWINFO_SEND`
```rust
const IPV6_FLOWINFO_SEND: c_int = 33i32;
```

### `IPV6_RECVFRAGSIZE`
```rust
const IPV6_RECVFRAGSIZE: c_int = 77i32;
```

### `IPV6_FREEBIND`
```rust
const IPV6_FREEBIND: c_int = 78i32;
```

### `IPV6_FLOWINFO_FLOWLABEL`
```rust
const IPV6_FLOWINFO_FLOWLABEL: c_int = 1_048_575i32;
```

### `IPV6_FLOWINFO_PRIORITY`
```rust
const IPV6_FLOWINFO_PRIORITY: c_int = 267_386_880i32;
```

### `SK_MEMINFO_RMEM_ALLOC`
```rust
const SK_MEMINFO_RMEM_ALLOC: c_int = 0i32;
```

### `SK_MEMINFO_RCVBUF`
```rust
const SK_MEMINFO_RCVBUF: c_int = 1i32;
```

### `SK_MEMINFO_WMEM_ALLOC`
```rust
const SK_MEMINFO_WMEM_ALLOC: c_int = 2i32;
```

### `SK_MEMINFO_SNDBUF`
```rust
const SK_MEMINFO_SNDBUF: c_int = 3i32;
```

### `SK_MEMINFO_FWD_ALLOC`
```rust
const SK_MEMINFO_FWD_ALLOC: c_int = 4i32;
```

### `SK_MEMINFO_WMEM_QUEUED`
```rust
const SK_MEMINFO_WMEM_QUEUED: c_int = 5i32;
```

### `SK_MEMINFO_OPTMEM`
```rust
const SK_MEMINFO_OPTMEM: c_int = 6i32;
```

### `SK_MEMINFO_BACKLOG`
```rust
const SK_MEMINFO_BACKLOG: c_int = 7i32;
```

### `SK_MEMINFO_DROPS`
```rust
const SK_MEMINFO_DROPS: c_int = 8i32;
```

### `CLOSE_RANGE_UNSHARE`
```rust
const CLOSE_RANGE_UNSHARE: c_uint = 2u32;
```

### `CLOSE_RANGE_CLOEXEC`
```rust
const CLOSE_RANGE_CLOEXEC: c_uint = 4u32;
```

### `SKF_AD_OFF`
```rust
const SKF_AD_OFF: c_int = -4_096i32;
```

### `SKF_AD_PROTOCOL`
```rust
const SKF_AD_PROTOCOL: c_int = 0i32;
```

### `SKF_AD_PKTTYPE`
```rust
const SKF_AD_PKTTYPE: c_int = 4i32;
```

### `SKF_AD_IFINDEX`
```rust
const SKF_AD_IFINDEX: c_int = 8i32;
```

### `SKF_AD_NLATTR`
```rust
const SKF_AD_NLATTR: c_int = 12i32;
```

### `SKF_AD_NLATTR_NEST`
```rust
const SKF_AD_NLATTR_NEST: c_int = 16i32;
```

### `SKF_AD_MARK`
```rust
const SKF_AD_MARK: c_int = 20i32;
```

### `SKF_AD_QUEUE`
```rust
const SKF_AD_QUEUE: c_int = 24i32;
```

### `SKF_AD_HATYPE`
```rust
const SKF_AD_HATYPE: c_int = 28i32;
```

### `SKF_AD_RXHASH`
```rust
const SKF_AD_RXHASH: c_int = 32i32;
```

### `SKF_AD_CPU`
```rust
const SKF_AD_CPU: c_int = 36i32;
```

### `SKF_AD_ALU_XOR_X`
```rust
const SKF_AD_ALU_XOR_X: c_int = 40i32;
```

### `SKF_AD_VLAN_TAG`
```rust
const SKF_AD_VLAN_TAG: c_int = 44i32;
```

### `SKF_AD_VLAN_TAG_PRESENT`
```rust
const SKF_AD_VLAN_TAG_PRESENT: c_int = 48i32;
```

### `SKF_AD_PAY_OFFSET`
```rust
const SKF_AD_PAY_OFFSET: c_int = 52i32;
```

### `SKF_AD_RANDOM`
```rust
const SKF_AD_RANDOM: c_int = 56i32;
```

### `SKF_AD_VLAN_TPID`
```rust
const SKF_AD_VLAN_TPID: c_int = 60i32;
```

### `SKF_AD_MAX`
```rust
const SKF_AD_MAX: c_int = 64i32;
```

### `SKF_NET_OFF`
```rust
const SKF_NET_OFF: c_int = -1_048_576i32;
```

### `SKF_LL_OFF`
```rust
const SKF_LL_OFF: c_int = -2_097_152i32;
```

### `BPF_NET_OFF`
```rust
const BPF_NET_OFF: c_int = -1_048_576i32;
```

### `BPF_LL_OFF`
```rust
const BPF_LL_OFF: c_int = -2_097_152i32;
```

### `BPF_MEMWORDS`
```rust
const BPF_MEMWORDS: c_int = 16i32;
```

### `BPF_MAXINSNS`
```rust
const BPF_MAXINSNS: c_int = 4_096i32;
```

### `BPF_LD`
```rust
const BPF_LD: __u32 = 0u32;
```

### `BPF_LDX`
```rust
const BPF_LDX: __u32 = 1u32;
```

### `BPF_ST`
```rust
const BPF_ST: __u32 = 2u32;
```

### `BPF_STX`
```rust
const BPF_STX: __u32 = 3u32;
```

### `BPF_ALU`
```rust
const BPF_ALU: __u32 = 4u32;
```

### `BPF_JMP`
```rust
const BPF_JMP: __u32 = 5u32;
```

### `BPF_RET`
```rust
const BPF_RET: __u32 = 6u32;
```

### `BPF_MISC`
```rust
const BPF_MISC: __u32 = 7u32;
```

### `BPF_W`
```rust
const BPF_W: __u32 = 0u32;
```

### `BPF_H`
```rust
const BPF_H: __u32 = 8u32;
```

### `BPF_B`
```rust
const BPF_B: __u32 = 16u32;
```

### `BPF_IMM`
```rust
const BPF_IMM: __u32 = 0u32;
```

### `BPF_ABS`
```rust
const BPF_ABS: __u32 = 32u32;
```

### `BPF_IND`
```rust
const BPF_IND: __u32 = 64u32;
```

### `BPF_MEM`
```rust
const BPF_MEM: __u32 = 96u32;
```

### `BPF_LEN`
```rust
const BPF_LEN: __u32 = 128u32;
```

### `BPF_MSH`
```rust
const BPF_MSH: __u32 = 160u32;
```

### `BPF_ADD`
```rust
const BPF_ADD: __u32 = 0u32;
```

### `BPF_SUB`
```rust
const BPF_SUB: __u32 = 16u32;
```

### `BPF_MUL`
```rust
const BPF_MUL: __u32 = 32u32;
```

### `BPF_DIV`
```rust
const BPF_DIV: __u32 = 48u32;
```

### `BPF_OR`
```rust
const BPF_OR: __u32 = 64u32;
```

### `BPF_AND`
```rust
const BPF_AND: __u32 = 80u32;
```

### `BPF_LSH`
```rust
const BPF_LSH: __u32 = 96u32;
```

### `BPF_RSH`
```rust
const BPF_RSH: __u32 = 112u32;
```

### `BPF_NEG`
```rust
const BPF_NEG: __u32 = 128u32;
```

### `BPF_MOD`
```rust
const BPF_MOD: __u32 = 144u32;
```

### `BPF_XOR`
```rust
const BPF_XOR: __u32 = 160u32;
```

### `BPF_JA`
```rust
const BPF_JA: __u32 = 0u32;
```

### `BPF_JEQ`
```rust
const BPF_JEQ: __u32 = 16u32;
```

### `BPF_JGT`
```rust
const BPF_JGT: __u32 = 32u32;
```

### `BPF_JGE`
```rust
const BPF_JGE: __u32 = 48u32;
```

### `BPF_JSET`
```rust
const BPF_JSET: __u32 = 64u32;
```

### `BPF_K`
```rust
const BPF_K: __u32 = 0u32;
```

### `BPF_X`
```rust
const BPF_X: __u32 = 8u32;
```

### `BPF_A`
```rust
const BPF_A: __u32 = 16u32;
```

### `BPF_TAX`
```rust
const BPF_TAX: __u32 = 0u32;
```

### `BPF_TXA`
```rust
const BPF_TXA: __u32 = 128u32;
```

### `RESOLVE_NO_XDEV`
```rust
const RESOLVE_NO_XDEV: crate::__u64 = 1u64;
```

### `RESOLVE_NO_MAGICLINKS`
```rust
const RESOLVE_NO_MAGICLINKS: crate::__u64 = 2u64;
```

### `RESOLVE_NO_SYMLINKS`
```rust
const RESOLVE_NO_SYMLINKS: crate::__u64 = 4u64;
```

### `RESOLVE_BENEATH`
```rust
const RESOLVE_BENEATH: crate::__u64 = 8u64;
```

### `RESOLVE_IN_ROOT`
```rust
const RESOLVE_IN_ROOT: crate::__u64 = 16u64;
```

### `RESOLVE_CACHED`
```rust
const RESOLVE_CACHED: crate::__u64 = 32u64;
```

### `ETH_ALEN`
```rust
const ETH_ALEN: c_int = 6i32;
```

### `ETH_HLEN`
```rust
const ETH_HLEN: c_int = 14i32;
```

### `ETH_ZLEN`
```rust
const ETH_ZLEN: c_int = 60i32;
```

### `ETH_DATA_LEN`
```rust
const ETH_DATA_LEN: c_int = 1_500i32;
```

### `ETH_FRAME_LEN`
```rust
const ETH_FRAME_LEN: c_int = 1_514i32;
```

### `ETH_FCS_LEN`
```rust
const ETH_FCS_LEN: c_int = 4i32;
```

### `ETH_P_LOOP`
```rust
const ETH_P_LOOP: c_int = 96i32;
```

### `ETH_P_PUP`
```rust
const ETH_P_PUP: c_int = 512i32;
```

### `ETH_P_PUPAT`
```rust
const ETH_P_PUPAT: c_int = 513i32;
```

### `ETH_P_IP`
```rust
const ETH_P_IP: c_int = 2_048i32;
```

### `ETH_P_X25`
```rust
const ETH_P_X25: c_int = 2_053i32;
```

### `ETH_P_ARP`
```rust
const ETH_P_ARP: c_int = 2_054i32;
```

### `ETH_P_BPQ`
```rust
const ETH_P_BPQ: c_int = 2_303i32;
```

### `ETH_P_IEEEPUP`
```rust
const ETH_P_IEEEPUP: c_int = 2_560i32;
```

### `ETH_P_IEEEPUPAT`
```rust
const ETH_P_IEEEPUPAT: c_int = 2_561i32;
```

### `ETH_P_BATMAN`
```rust
const ETH_P_BATMAN: c_int = 17_157i32;
```

### `ETH_P_DEC`
```rust
const ETH_P_DEC: c_int = 24_576i32;
```

### `ETH_P_DNA_DL`
```rust
const ETH_P_DNA_DL: c_int = 24_577i32;
```

### `ETH_P_DNA_RC`
```rust
const ETH_P_DNA_RC: c_int = 24_578i32;
```

### `ETH_P_DNA_RT`
```rust
const ETH_P_DNA_RT: c_int = 24_579i32;
```

### `ETH_P_LAT`
```rust
const ETH_P_LAT: c_int = 24_580i32;
```

### `ETH_P_DIAG`
```rust
const ETH_P_DIAG: c_int = 24_581i32;
```

### `ETH_P_CUST`
```rust
const ETH_P_CUST: c_int = 24_582i32;
```

### `ETH_P_SCA`
```rust
const ETH_P_SCA: c_int = 24_583i32;
```

### `ETH_P_TEB`
```rust
const ETH_P_TEB: c_int = 25_944i32;
```

### `ETH_P_RARP`
```rust
const ETH_P_RARP: c_int = 32_821i32;
```

### `ETH_P_ATALK`
```rust
const ETH_P_ATALK: c_int = 32_923i32;
```

### `ETH_P_AARP`
```rust
const ETH_P_AARP: c_int = 33_011i32;
```

### `ETH_P_8021Q`
```rust
const ETH_P_8021Q: c_int = 33_024i32;
```

### `ETH_P_IPX`
```rust
const ETH_P_IPX: c_int = 33_079i32;
```

### `ETH_P_IPV6`
```rust
const ETH_P_IPV6: c_int = 34_525i32;
```

### `ETH_P_PAUSE`
```rust
const ETH_P_PAUSE: c_int = 34_824i32;
```

### `ETH_P_SLOW`
```rust
const ETH_P_SLOW: c_int = 34_825i32;
```

### `ETH_P_WCCP`
```rust
const ETH_P_WCCP: c_int = 34_878i32;
```

### `ETH_P_MPLS_UC`
```rust
const ETH_P_MPLS_UC: c_int = 34_887i32;
```

### `ETH_P_MPLS_MC`
```rust
const ETH_P_MPLS_MC: c_int = 34_888i32;
```

### `ETH_P_ATMMPOA`
```rust
const ETH_P_ATMMPOA: c_int = 34_892i32;
```

### `ETH_P_PPP_DISC`
```rust
const ETH_P_PPP_DISC: c_int = 34_915i32;
```

### `ETH_P_PPP_SES`
```rust
const ETH_P_PPP_SES: c_int = 34_916i32;
```

### `ETH_P_LINK_CTL`
```rust
const ETH_P_LINK_CTL: c_int = 34_924i32;
```

### `ETH_P_ATMFATE`
```rust
const ETH_P_ATMFATE: c_int = 34_948i32;
```

### `ETH_P_PAE`
```rust
const ETH_P_PAE: c_int = 34_958i32;
```

### `ETH_P_AOE`
```rust
const ETH_P_AOE: c_int = 34_978i32;
```

### `ETH_P_8021AD`
```rust
const ETH_P_8021AD: c_int = 34_984i32;
```

### `ETH_P_802_EX1`
```rust
const ETH_P_802_EX1: c_int = 34_997i32;
```

### `ETH_P_TIPC`
```rust
const ETH_P_TIPC: c_int = 35_018i32;
```

### `ETH_P_MACSEC`
```rust
const ETH_P_MACSEC: c_int = 35_045i32;
```

### `ETH_P_8021AH`
```rust
const ETH_P_8021AH: c_int = 35_047i32;
```

### `ETH_P_MVRP`
```rust
const ETH_P_MVRP: c_int = 35_061i32;
```

### `ETH_P_1588`
```rust
const ETH_P_1588: c_int = 35_063i32;
```

### `ETH_P_PRP`
```rust
const ETH_P_PRP: c_int = 35_067i32;
```

### `ETH_P_FCOE`
```rust
const ETH_P_FCOE: c_int = 35_078i32;
```

### `ETH_P_TDLS`
```rust
const ETH_P_TDLS: c_int = 35_085i32;
```

### `ETH_P_FIP`
```rust
const ETH_P_FIP: c_int = 35_092i32;
```

### `ETH_P_80221`
```rust
const ETH_P_80221: c_int = 35_095i32;
```

### `ETH_P_LOOPBACK`
```rust
const ETH_P_LOOPBACK: c_int = 36_864i32;
```

### `ETH_P_QINQ1`
```rust
const ETH_P_QINQ1: c_int = 37_120i32;
```

### `ETH_P_QINQ2`
```rust
const ETH_P_QINQ2: c_int = 37_376i32;
```

### `ETH_P_QINQ3`
```rust
const ETH_P_QINQ3: c_int = 37_632i32;
```

### `ETH_P_EDSA`
```rust
const ETH_P_EDSA: c_int = 56_026i32;
```

### `ETH_P_AF_IUCV`
```rust
const ETH_P_AF_IUCV: c_int = 64_507i32;
```

### `ETH_P_802_3_MIN`
```rust
const ETH_P_802_3_MIN: c_int = 1_536i32;
```

### `ETH_P_802_3`
```rust
const ETH_P_802_3: c_int = 1i32;
```

### `ETH_P_AX25`
```rust
const ETH_P_AX25: c_int = 2i32;
```

### `ETH_P_ALL`
```rust
const ETH_P_ALL: c_int = 3i32;
```

### `ETH_P_802_2`
```rust
const ETH_P_802_2: c_int = 4i32;
```

### `ETH_P_SNAP`
```rust
const ETH_P_SNAP: c_int = 5i32;
```

### `ETH_P_DDCMP`
```rust
const ETH_P_DDCMP: c_int = 6i32;
```

### `ETH_P_WAN_PPP`
```rust
const ETH_P_WAN_PPP: c_int = 7i32;
```

### `ETH_P_PPP_MP`
```rust
const ETH_P_PPP_MP: c_int = 8i32;
```

### `ETH_P_LOCALTALK`
```rust
const ETH_P_LOCALTALK: c_int = 9i32;
```

### `ETH_P_CANFD`
```rust
const ETH_P_CANFD: c_int = 13i32;
```

### `ETH_P_PPPTALK`
```rust
const ETH_P_PPPTALK: c_int = 16i32;
```

### `ETH_P_TR_802_2`
```rust
const ETH_P_TR_802_2: c_int = 17i32;
```

### `ETH_P_MOBITEX`
```rust
const ETH_P_MOBITEX: c_int = 21i32;
```

### `ETH_P_CONTROL`
```rust
const ETH_P_CONTROL: c_int = 22i32;
```

### `ETH_P_IRDA`
```rust
const ETH_P_IRDA: c_int = 23i32;
```

### `ETH_P_ECONET`
```rust
const ETH_P_ECONET: c_int = 24i32;
```

### `ETH_P_HDLC`
```rust
const ETH_P_HDLC: c_int = 25i32;
```

### `ETH_P_ARCNET`
```rust
const ETH_P_ARCNET: c_int = 26i32;
```

### `ETH_P_DSA`
```rust
const ETH_P_DSA: c_int = 27i32;
```

### `ETH_P_TRAILER`
```rust
const ETH_P_TRAILER: c_int = 28i32;
```

### `ETH_P_PHONET`
```rust
const ETH_P_PHONET: c_int = 245i32;
```

### `ETH_P_IEEE802154`
```rust
const ETH_P_IEEE802154: c_int = 246i32;
```

### `ETH_P_CAIF`
```rust
const ETH_P_CAIF: c_int = 247i32;
```

### `POSIX_SPAWN_RESETIDS`
```rust
const POSIX_SPAWN_RESETIDS: c_int = 1i32;
```

### `POSIX_SPAWN_SETPGROUP`
```rust
const POSIX_SPAWN_SETPGROUP: c_int = 2i32;
```

### `POSIX_SPAWN_SETSIGDEF`
```rust
const POSIX_SPAWN_SETSIGDEF: c_int = 4i32;
```

### `POSIX_SPAWN_SETSIGMASK`
```rust
const POSIX_SPAWN_SETSIGMASK: c_int = 8i32;
```

### `POSIX_SPAWN_SETSCHEDPARAM`
```rust
const POSIX_SPAWN_SETSCHEDPARAM: c_int = 16i32;
```

### `POSIX_SPAWN_SETSCHEDULER`
```rust
const POSIX_SPAWN_SETSCHEDULER: c_int = 32i32;
```

### `NFNLGRP_NONE`
```rust
const NFNLGRP_NONE: c_int = 0i32;
```

### `NFNLGRP_CONNTRACK_NEW`
```rust
const NFNLGRP_CONNTRACK_NEW: c_int = 1i32;
```

### `NFNLGRP_CONNTRACK_UPDATE`
```rust
const NFNLGRP_CONNTRACK_UPDATE: c_int = 2i32;
```

### `NFNLGRP_CONNTRACK_DESTROY`
```rust
const NFNLGRP_CONNTRACK_DESTROY: c_int = 3i32;
```

### `NFNLGRP_CONNTRACK_EXP_NEW`
```rust
const NFNLGRP_CONNTRACK_EXP_NEW: c_int = 4i32;
```

### `NFNLGRP_CONNTRACK_EXP_UPDATE`
```rust
const NFNLGRP_CONNTRACK_EXP_UPDATE: c_int = 5i32;
```

### `NFNLGRP_CONNTRACK_EXP_DESTROY`
```rust
const NFNLGRP_CONNTRACK_EXP_DESTROY: c_int = 6i32;
```

### `NFNLGRP_NFTABLES`
```rust
const NFNLGRP_NFTABLES: c_int = 7i32;
```

### `NFNLGRP_ACCT_QUOTA`
```rust
const NFNLGRP_ACCT_QUOTA: c_int = 8i32;
```

### `NFNLGRP_NFTRACE`
```rust
const NFNLGRP_NFTRACE: c_int = 9i32;
```

### `NFNETLINK_V0`
```rust
const NFNETLINK_V0: c_int = 0i32;
```

### `NFNL_SUBSYS_NONE`
```rust
const NFNL_SUBSYS_NONE: c_int = 0i32;
```

### `NFNL_SUBSYS_CTNETLINK`
```rust
const NFNL_SUBSYS_CTNETLINK: c_int = 1i32;
```

### `NFNL_SUBSYS_CTNETLINK_EXP`
```rust
const NFNL_SUBSYS_CTNETLINK_EXP: c_int = 2i32;
```

### `NFNL_SUBSYS_QUEUE`
```rust
const NFNL_SUBSYS_QUEUE: c_int = 3i32;
```

### `NFNL_SUBSYS_ULOG`
```rust
const NFNL_SUBSYS_ULOG: c_int = 4i32;
```

### `NFNL_SUBSYS_OSF`
```rust
const NFNL_SUBSYS_OSF: c_int = 5i32;
```

### `NFNL_SUBSYS_IPSET`
```rust
const NFNL_SUBSYS_IPSET: c_int = 6i32;
```

### `NFNL_SUBSYS_ACCT`
```rust
const NFNL_SUBSYS_ACCT: c_int = 7i32;
```

### `NFNL_SUBSYS_CTNETLINK_TIMEOUT`
```rust
const NFNL_SUBSYS_CTNETLINK_TIMEOUT: c_int = 8i32;
```

### `NFNL_SUBSYS_CTHELPER`
```rust
const NFNL_SUBSYS_CTHELPER: c_int = 9i32;
```

### `NFNL_SUBSYS_NFTABLES`
```rust
const NFNL_SUBSYS_NFTABLES: c_int = 10i32;
```

### `NFNL_SUBSYS_NFT_COMPAT`
```rust
const NFNL_SUBSYS_NFT_COMPAT: c_int = 11i32;
```

### `NFNL_SUBSYS_HOOK`
```rust
const NFNL_SUBSYS_HOOK: c_int = 12i32;
```

### `NFNL_SUBSYS_COUNT`
```rust
const NFNL_SUBSYS_COUNT: c_int = 13i32;
```

### `NFNL_MSG_BATCH_BEGIN`
```rust
const NFNL_MSG_BATCH_BEGIN: c_int = 16i32;
```

### `NFNL_MSG_BATCH_END`
```rust
const NFNL_MSG_BATCH_END: c_int = 17i32;
```

### `NFNL_BATCH_UNSPEC`
```rust
const NFNL_BATCH_UNSPEC: c_int = 0i32;
```

### `NFNL_BATCH_GENID`
```rust
const NFNL_BATCH_GENID: c_int = 1i32;
```

### `NFULNL_MSG_PACKET`
```rust
const NFULNL_MSG_PACKET: c_int = 0i32;
```

### `NFULNL_MSG_CONFIG`
```rust
const NFULNL_MSG_CONFIG: c_int = 1i32;
```

### `NFULA_VLAN_UNSPEC`
```rust
const NFULA_VLAN_UNSPEC: c_int = 0i32;
```

### `NFULA_VLAN_PROTO`
```rust
const NFULA_VLAN_PROTO: c_int = 1i32;
```

### `NFULA_VLAN_TCI`
```rust
const NFULA_VLAN_TCI: c_int = 2i32;
```

### `NFULA_UNSPEC`
```rust
const NFULA_UNSPEC: c_int = 0i32;
```

### `NFULA_PACKET_HDR`
```rust
const NFULA_PACKET_HDR: c_int = 1i32;
```

### `NFULA_MARK`
```rust
const NFULA_MARK: c_int = 2i32;
```

### `NFULA_TIMESTAMP`
```rust
const NFULA_TIMESTAMP: c_int = 3i32;
```

### `NFULA_IFINDEX_INDEV`
```rust
const NFULA_IFINDEX_INDEV: c_int = 4i32;
```

### `NFULA_IFINDEX_OUTDEV`
```rust
const NFULA_IFINDEX_OUTDEV: c_int = 5i32;
```

### `NFULA_IFINDEX_PHYSINDEV`
```rust
const NFULA_IFINDEX_PHYSINDEV: c_int = 6i32;
```

### `NFULA_IFINDEX_PHYSOUTDEV`
```rust
const NFULA_IFINDEX_PHYSOUTDEV: c_int = 7i32;
```

### `NFULA_HWADDR`
```rust
const NFULA_HWADDR: c_int = 8i32;
```

### `NFULA_PAYLOAD`
```rust
const NFULA_PAYLOAD: c_int = 9i32;
```

### `NFULA_PREFIX`
```rust
const NFULA_PREFIX: c_int = 10i32;
```

### `NFULA_UID`
```rust
const NFULA_UID: c_int = 11i32;
```

### `NFULA_SEQ`
```rust
const NFULA_SEQ: c_int = 12i32;
```

### `NFULA_SEQ_GLOBAL`
```rust
const NFULA_SEQ_GLOBAL: c_int = 13i32;
```

### `NFULA_GID`
```rust
const NFULA_GID: c_int = 14i32;
```

### `NFULA_HWTYPE`
```rust
const NFULA_HWTYPE: c_int = 15i32;
```

### `NFULA_HWHEADER`
```rust
const NFULA_HWHEADER: c_int = 16i32;
```

### `NFULA_HWLEN`
```rust
const NFULA_HWLEN: c_int = 17i32;
```

### `NFULA_CT`
```rust
const NFULA_CT: c_int = 18i32;
```

### `NFULA_CT_INFO`
```rust
const NFULA_CT_INFO: c_int = 19i32;
```

### `NFULA_VLAN`
```rust
const NFULA_VLAN: c_int = 20i32;
```

### `NFULA_L2HDR`
```rust
const NFULA_L2HDR: c_int = 21i32;
```

### `NFULNL_CFG_CMD_NONE`
```rust
const NFULNL_CFG_CMD_NONE: c_int = 0i32;
```

### `NFULNL_CFG_CMD_BIND`
```rust
const NFULNL_CFG_CMD_BIND: c_int = 1i32;
```

### `NFULNL_CFG_CMD_UNBIND`
```rust
const NFULNL_CFG_CMD_UNBIND: c_int = 2i32;
```

### `NFULNL_CFG_CMD_PF_BIND`
```rust
const NFULNL_CFG_CMD_PF_BIND: c_int = 3i32;
```

### `NFULNL_CFG_CMD_PF_UNBIND`
```rust
const NFULNL_CFG_CMD_PF_UNBIND: c_int = 4i32;
```

### `NFULA_CFG_UNSPEC`
```rust
const NFULA_CFG_UNSPEC: c_int = 0i32;
```

### `NFULA_CFG_CMD`
```rust
const NFULA_CFG_CMD: c_int = 1i32;
```

### `NFULA_CFG_MODE`
```rust
const NFULA_CFG_MODE: c_int = 2i32;
```

### `NFULA_CFG_NLBUFSIZ`
```rust
const NFULA_CFG_NLBUFSIZ: c_int = 3i32;
```

### `NFULA_CFG_TIMEOUT`
```rust
const NFULA_CFG_TIMEOUT: c_int = 4i32;
```

### `NFULA_CFG_QTHRESH`
```rust
const NFULA_CFG_QTHRESH: c_int = 5i32;
```

### `NFULA_CFG_FLAGS`
```rust
const NFULA_CFG_FLAGS: c_int = 6i32;
```

### `NFULNL_COPY_NONE`
```rust
const NFULNL_COPY_NONE: c_int = 0i32;
```

### `NFULNL_COPY_META`
```rust
const NFULNL_COPY_META: c_int = 1i32;
```

### `NFULNL_COPY_PACKET`
```rust
const NFULNL_COPY_PACKET: c_int = 2i32;
```

### `NFULNL_CFG_F_SEQ`
```rust
const NFULNL_CFG_F_SEQ: c_int = 1i32;
```

### `NFULNL_CFG_F_SEQ_GLOBAL`
```rust
const NFULNL_CFG_F_SEQ_GLOBAL: c_int = 2i32;
```

### `NFULNL_CFG_F_CONNTRACK`
```rust
const NFULNL_CFG_F_CONNTRACK: c_int = 4i32;
```

### `NFQNL_MSG_PACKET`
```rust
const NFQNL_MSG_PACKET: c_int = 0i32;
```

### `NFQNL_MSG_VERDICT`
```rust
const NFQNL_MSG_VERDICT: c_int = 1i32;
```

### `NFQNL_MSG_CONFIG`
```rust
const NFQNL_MSG_CONFIG: c_int = 2i32;
```

### `NFQNL_MSG_VERDICT_BATCH`
```rust
const NFQNL_MSG_VERDICT_BATCH: c_int = 3i32;
```

### `NFQA_UNSPEC`
```rust
const NFQA_UNSPEC: c_int = 0i32;
```

### `NFQA_PACKET_HDR`
```rust
const NFQA_PACKET_HDR: c_int = 1i32;
```

### `NFQA_VERDICT_HDR`
```rust
const NFQA_VERDICT_HDR: c_int = 2i32;
```

### `NFQA_MARK`
```rust
const NFQA_MARK: c_int = 3i32;
```

### `NFQA_TIMESTAMP`
```rust
const NFQA_TIMESTAMP: c_int = 4i32;
```

### `NFQA_IFINDEX_INDEV`
```rust
const NFQA_IFINDEX_INDEV: c_int = 5i32;
```

### `NFQA_IFINDEX_OUTDEV`
```rust
const NFQA_IFINDEX_OUTDEV: c_int = 6i32;
```

### `NFQA_IFINDEX_PHYSINDEV`
```rust
const NFQA_IFINDEX_PHYSINDEV: c_int = 7i32;
```

### `NFQA_IFINDEX_PHYSOUTDEV`
```rust
const NFQA_IFINDEX_PHYSOUTDEV: c_int = 8i32;
```

### `NFQA_HWADDR`
```rust
const NFQA_HWADDR: c_int = 9i32;
```

### `NFQA_PAYLOAD`
```rust
const NFQA_PAYLOAD: c_int = 10i32;
```

### `NFQA_CT`
```rust
const NFQA_CT: c_int = 11i32;
```

### `NFQA_CT_INFO`
```rust
const NFQA_CT_INFO: c_int = 12i32;
```

### `NFQA_CAP_LEN`
```rust
const NFQA_CAP_LEN: c_int = 13i32;
```

### `NFQA_SKB_INFO`
```rust
const NFQA_SKB_INFO: c_int = 14i32;
```

### `NFQA_EXP`
```rust
const NFQA_EXP: c_int = 15i32;
```

### `NFQA_UID`
```rust
const NFQA_UID: c_int = 16i32;
```

### `NFQA_GID`
```rust
const NFQA_GID: c_int = 17i32;
```

### `NFQA_SECCTX`
```rust
const NFQA_SECCTX: c_int = 18i32;
```

### `NFQA_VLAN`
```rust
const NFQA_VLAN: c_int = 19i32;
```

### `NFQA_L2HDR`
```rust
const NFQA_L2HDR: c_int = 20i32;
```

### `NFQA_PRIORITY`
```rust
const NFQA_PRIORITY: c_int = 21i32;
```

### `NFQA_VLAN_UNSPEC`
```rust
const NFQA_VLAN_UNSPEC: c_int = 0i32;
```

### `NFQA_VLAN_PROTO`
```rust
const NFQA_VLAN_PROTO: c_int = 1i32;
```

### `NFQA_VLAN_TCI`
```rust
const NFQA_VLAN_TCI: c_int = 2i32;
```

### `NFQNL_CFG_CMD_NONE`
```rust
const NFQNL_CFG_CMD_NONE: c_int = 0i32;
```

### `NFQNL_CFG_CMD_BIND`
```rust
const NFQNL_CFG_CMD_BIND: c_int = 1i32;
```

### `NFQNL_CFG_CMD_UNBIND`
```rust
const NFQNL_CFG_CMD_UNBIND: c_int = 2i32;
```

### `NFQNL_CFG_CMD_PF_BIND`
```rust
const NFQNL_CFG_CMD_PF_BIND: c_int = 3i32;
```

### `NFQNL_CFG_CMD_PF_UNBIND`
```rust
const NFQNL_CFG_CMD_PF_UNBIND: c_int = 4i32;
```

### `NFQNL_COPY_NONE`
```rust
const NFQNL_COPY_NONE: c_int = 0i32;
```

### `NFQNL_COPY_META`
```rust
const NFQNL_COPY_META: c_int = 1i32;
```

### `NFQNL_COPY_PACKET`
```rust
const NFQNL_COPY_PACKET: c_int = 2i32;
```

### `NFQA_CFG_UNSPEC`
```rust
const NFQA_CFG_UNSPEC: c_int = 0i32;
```

### `NFQA_CFG_CMD`
```rust
const NFQA_CFG_CMD: c_int = 1i32;
```

### `NFQA_CFG_PARAMS`
```rust
const NFQA_CFG_PARAMS: c_int = 2i32;
```

### `NFQA_CFG_QUEUE_MAXLEN`
```rust
const NFQA_CFG_QUEUE_MAXLEN: c_int = 3i32;
```

### `NFQA_CFG_MASK`
```rust
const NFQA_CFG_MASK: c_int = 4i32;
```

### `NFQA_CFG_FLAGS`
```rust
const NFQA_CFG_FLAGS: c_int = 5i32;
```

### `NFQA_CFG_F_FAIL_OPEN`
```rust
const NFQA_CFG_F_FAIL_OPEN: c_int = 1i32;
```

### `NFQA_CFG_F_CONNTRACK`
```rust
const NFQA_CFG_F_CONNTRACK: c_int = 2i32;
```

### `NFQA_CFG_F_GSO`
```rust
const NFQA_CFG_F_GSO: c_int = 4i32;
```

### `NFQA_CFG_F_UID_GID`
```rust
const NFQA_CFG_F_UID_GID: c_int = 8i32;
```

### `NFQA_CFG_F_SECCTX`
```rust
const NFQA_CFG_F_SECCTX: c_int = 16i32;
```

### `NFQA_CFG_F_MAX`
```rust
const NFQA_CFG_F_MAX: c_int = 32i32;
```

### `NFQA_SKB_CSUMNOTREADY`
```rust
const NFQA_SKB_CSUMNOTREADY: c_int = 1i32;
```

### `NFQA_SKB_GSO`
```rust
const NFQA_SKB_GSO: c_int = 2i32;
```

### `NFQA_SKB_CSUM_NOTVERIFIED`
```rust
const NFQA_SKB_CSUM_NOTVERIFIED: c_int = 4i32;
```

### `GENL_NAMSIZ`
```rust
const GENL_NAMSIZ: c_int = 16i32;
```

### `GENL_MIN_ID`
```rust
const GENL_MIN_ID: c_int = 16i32;
```

### `GENL_MAX_ID`
```rust
const GENL_MAX_ID: c_int = 1_023i32;
```

### `GENL_ADMIN_PERM`
```rust
const GENL_ADMIN_PERM: c_int = 1i32;
```

### `GENL_CMD_CAP_DO`
```rust
const GENL_CMD_CAP_DO: c_int = 2i32;
```

### `GENL_CMD_CAP_DUMP`
```rust
const GENL_CMD_CAP_DUMP: c_int = 4i32;
```

### `GENL_CMD_CAP_HASPOL`
```rust
const GENL_CMD_CAP_HASPOL: c_int = 8i32;
```

### `GENL_ID_CTRL`
```rust
const GENL_ID_CTRL: c_int = 16i32;
```

### `CTRL_CMD_UNSPEC`
```rust
const CTRL_CMD_UNSPEC: c_int = 0i32;
```

### `CTRL_CMD_NEWFAMILY`
```rust
const CTRL_CMD_NEWFAMILY: c_int = 1i32;
```

### `CTRL_CMD_DELFAMILY`
```rust
const CTRL_CMD_DELFAMILY: c_int = 2i32;
```

### `CTRL_CMD_GETFAMILY`
```rust
const CTRL_CMD_GETFAMILY: c_int = 3i32;
```

### `CTRL_CMD_NEWOPS`
```rust
const CTRL_CMD_NEWOPS: c_int = 4i32;
```

### `CTRL_CMD_DELOPS`
```rust
const CTRL_CMD_DELOPS: c_int = 5i32;
```

### `CTRL_CMD_GETOPS`
```rust
const CTRL_CMD_GETOPS: c_int = 6i32;
```

### `CTRL_CMD_NEWMCAST_GRP`
```rust
const CTRL_CMD_NEWMCAST_GRP: c_int = 7i32;
```

### `CTRL_CMD_DELMCAST_GRP`
```rust
const CTRL_CMD_DELMCAST_GRP: c_int = 8i32;
```

### `CTRL_CMD_GETMCAST_GRP`
```rust
const CTRL_CMD_GETMCAST_GRP: c_int = 9i32;
```

### `CTRL_ATTR_UNSPEC`
```rust
const CTRL_ATTR_UNSPEC: c_int = 0i32;
```

### `CTRL_ATTR_FAMILY_ID`
```rust
const CTRL_ATTR_FAMILY_ID: c_int = 1i32;
```

### `CTRL_ATTR_FAMILY_NAME`
```rust
const CTRL_ATTR_FAMILY_NAME: c_int = 2i32;
```

### `CTRL_ATTR_VERSION`
```rust
const CTRL_ATTR_VERSION: c_int = 3i32;
```

### `CTRL_ATTR_HDRSIZE`
```rust
const CTRL_ATTR_HDRSIZE: c_int = 4i32;
```

### `CTRL_ATTR_MAXATTR`
```rust
const CTRL_ATTR_MAXATTR: c_int = 5i32;
```

### `CTRL_ATTR_OPS`
```rust
const CTRL_ATTR_OPS: c_int = 6i32;
```

### `CTRL_ATTR_MCAST_GROUPS`
```rust
const CTRL_ATTR_MCAST_GROUPS: c_int = 7i32;
```

### `CTRL_ATTR_OP_UNSPEC`
```rust
const CTRL_ATTR_OP_UNSPEC: c_int = 0i32;
```

### `CTRL_ATTR_OP_ID`
```rust
const CTRL_ATTR_OP_ID: c_int = 1i32;
```

### `CTRL_ATTR_OP_FLAGS`
```rust
const CTRL_ATTR_OP_FLAGS: c_int = 2i32;
```

### `CTRL_ATTR_MCAST_GRP_UNSPEC`
```rust
const CTRL_ATTR_MCAST_GRP_UNSPEC: c_int = 0i32;
```

### `CTRL_ATTR_MCAST_GRP_NAME`
```rust
const CTRL_ATTR_MCAST_GRP_NAME: c_int = 1i32;
```

### `CTRL_ATTR_MCAST_GRP_ID`
```rust
const CTRL_ATTR_MCAST_GRP_ID: c_int = 2i32;
```

### `PACKET_FANOUT`
```rust
const PACKET_FANOUT: c_int = 18i32;
```

### `PACKET_TX_HAS_OFF`
```rust
const PACKET_TX_HAS_OFF: c_int = 19i32;
```

### `PACKET_QDISC_BYPASS`
```rust
const PACKET_QDISC_BYPASS: c_int = 20i32;
```

### `PACKET_ROLLOVER_STATS`
```rust
const PACKET_ROLLOVER_STATS: c_int = 21i32;
```

### `PACKET_FANOUT_DATA`
```rust
const PACKET_FANOUT_DATA: c_int = 22i32;
```

### `PACKET_IGNORE_OUTGOING`
```rust
const PACKET_IGNORE_OUTGOING: c_int = 23i32;
```

### `PACKET_VNET_HDR_SZ`
```rust
const PACKET_VNET_HDR_SZ: c_int = 24i32;
```

### `PACKET_FANOUT_HASH`
```rust
const PACKET_FANOUT_HASH: c_uint = 0u32;
```

### `PACKET_FANOUT_LB`
```rust
const PACKET_FANOUT_LB: c_uint = 1u32;
```

### `PACKET_FANOUT_CPU`
```rust
const PACKET_FANOUT_CPU: c_uint = 2u32;
```

### `PACKET_FANOUT_ROLLOVER`
```rust
const PACKET_FANOUT_ROLLOVER: c_uint = 3u32;
```

### `PACKET_FANOUT_RND`
```rust
const PACKET_FANOUT_RND: c_uint = 4u32;
```

### `PACKET_FANOUT_QM`
```rust
const PACKET_FANOUT_QM: c_uint = 5u32;
```

### `PACKET_FANOUT_CBPF`
```rust
const PACKET_FANOUT_CBPF: c_uint = 6u32;
```

### `PACKET_FANOUT_EBPF`
```rust
const PACKET_FANOUT_EBPF: c_uint = 7u32;
```

### `PACKET_FANOUT_FLAG_ROLLOVER`
```rust
const PACKET_FANOUT_FLAG_ROLLOVER: c_uint = 4_096u32;
```

### `PACKET_FANOUT_FLAG_UNIQUEID`
```rust
const PACKET_FANOUT_FLAG_UNIQUEID: c_uint = 8_192u32;
```

### `PACKET_FANOUT_FLAG_IGNORE_OUTGOING`
```rust
const PACKET_FANOUT_FLAG_IGNORE_OUTGOING: c_uint = 16_384u32;
```

### `PACKET_FANOUT_FLAG_DEFRAG`
```rust
const PACKET_FANOUT_FLAG_DEFRAG: c_uint = 32_768u32;
```

### `TP_STATUS_KERNEL`
```rust
const TP_STATUS_KERNEL: __u32 = 0u32;
```

### `TP_STATUS_USER`
```rust
const TP_STATUS_USER: __u32 = 1u32;
```

### `TP_STATUS_COPY`
```rust
const TP_STATUS_COPY: __u32 = 2u32;
```

### `TP_STATUS_LOSING`
```rust
const TP_STATUS_LOSING: __u32 = 4u32;
```

### `TP_STATUS_CSUMNOTREADY`
```rust
const TP_STATUS_CSUMNOTREADY: __u32 = 8u32;
```

### `TP_STATUS_VLAN_VALID`
```rust
const TP_STATUS_VLAN_VALID: __u32 = 16u32;
```

### `TP_STATUS_BLK_TMO`
```rust
const TP_STATUS_BLK_TMO: __u32 = 32u32;
```

### `TP_STATUS_VLAN_TPID_VALID`
```rust
const TP_STATUS_VLAN_TPID_VALID: __u32 = 64u32;
```

### `TP_STATUS_CSUM_VALID`
```rust
const TP_STATUS_CSUM_VALID: __u32 = 128u32;
```

### `TP_STATUS_AVAILABLE`
```rust
const TP_STATUS_AVAILABLE: __u32 = 0u32;
```

### `TP_STATUS_SEND_REQUEST`
```rust
const TP_STATUS_SEND_REQUEST: __u32 = 1u32;
```

### `TP_STATUS_SENDING`
```rust
const TP_STATUS_SENDING: __u32 = 2u32;
```

### `TP_STATUS_WRONG_FORMAT`
```rust
const TP_STATUS_WRONG_FORMAT: __u32 = 4u32;
```

### `TP_STATUS_TS_SOFTWARE`
```rust
const TP_STATUS_TS_SOFTWARE: __u32 = 536_870_912u32;
```

### `TP_STATUS_TS_SYS_HARDWARE`
```rust
const TP_STATUS_TS_SYS_HARDWARE: __u32 = 1_073_741_824u32;
```

### `TP_STATUS_TS_RAW_HARDWARE`
```rust
const TP_STATUS_TS_RAW_HARDWARE: __u32 = 2_147_483_648u32;
```

### `TP_FT_REQ_FILL_RXHASH`
```rust
const TP_FT_REQ_FILL_RXHASH: __u32 = 1u32;
```

### `TPACKET_ALIGNMENT`
```rust
const TPACKET_ALIGNMENT: usize = 16usize;
```

### `TPACKET_HDRLEN`
```rust
const TPACKET_HDRLEN: usize = 52usize;
```

### `TPACKET2_HDRLEN`
```rust
const TPACKET2_HDRLEN: usize = 52usize;
```

### `TPACKET3_HDRLEN`
```rust
const TPACKET3_HDRLEN: usize = 68usize;
```

### `NF_DROP`
```rust
const NF_DROP: c_int = 0i32;
```

### `NF_ACCEPT`
```rust
const NF_ACCEPT: c_int = 1i32;
```

### `NF_STOLEN`
```rust
const NF_STOLEN: c_int = 2i32;
```

### `NF_QUEUE`
```rust
const NF_QUEUE: c_int = 3i32;
```

### `NF_REPEAT`
```rust
const NF_REPEAT: c_int = 4i32;
```

### `NF_STOP`
```rust
const NF_STOP: c_int = 5i32;
```

### `NF_MAX_VERDICT`
```rust
const NF_MAX_VERDICT: c_int = 5i32;
```

### `NF_VERDICT_MASK`
```rust
const NF_VERDICT_MASK: c_int = 255i32;
```

### `NF_VERDICT_FLAG_QUEUE_BYPASS`
```rust
const NF_VERDICT_FLAG_QUEUE_BYPASS: c_int = 32_768i32;
```

### `NF_VERDICT_QMASK`
```rust
const NF_VERDICT_QMASK: c_int = -65_536i32;
```

### `NF_VERDICT_QBITS`
```rust
const NF_VERDICT_QBITS: c_int = 16i32;
```

### `NF_VERDICT_BITS`
```rust
const NF_VERDICT_BITS: c_int = 16i32;
```

### `NF_INET_PRE_ROUTING`
```rust
const NF_INET_PRE_ROUTING: c_int = 0i32;
```

### `NF_INET_LOCAL_IN`
```rust
const NF_INET_LOCAL_IN: c_int = 1i32;
```

### `NF_INET_FORWARD`
```rust
const NF_INET_FORWARD: c_int = 2i32;
```

### `NF_INET_LOCAL_OUT`
```rust
const NF_INET_LOCAL_OUT: c_int = 3i32;
```

### `NF_INET_POST_ROUTING`
```rust
const NF_INET_POST_ROUTING: c_int = 4i32;
```

### `NF_INET_NUMHOOKS`
```rust
const NF_INET_NUMHOOKS: c_int = 5i32;
```

### `NF_INET_INGRESS`
```rust
const NF_INET_INGRESS: c_int = 5i32;
```

### `NF_NETDEV_INGRESS`
```rust
const NF_NETDEV_INGRESS: c_int = 0i32;
```

### `NF_NETDEV_EGRESS`
```rust
const NF_NETDEV_EGRESS: c_int = 1i32;
```

### `NF_NETDEV_NUMHOOKS`
```rust
const NF_NETDEV_NUMHOOKS: c_int = 2i32;
```

### `NFPROTO_UNSPEC`
```rust
const NFPROTO_UNSPEC: c_int = 0i32;
```

### `NFPROTO_INET`
```rust
const NFPROTO_INET: c_int = 1i32;
```

### `NFPROTO_IPV4`
```rust
const NFPROTO_IPV4: c_int = 2i32;
```

### `NFPROTO_ARP`
```rust
const NFPROTO_ARP: c_int = 3i32;
```

### `NFPROTO_NETDEV`
```rust
const NFPROTO_NETDEV: c_int = 5i32;
```

### `NFPROTO_BRIDGE`
```rust
const NFPROTO_BRIDGE: c_int = 7i32;
```

### `NFPROTO_IPV6`
```rust
const NFPROTO_IPV6: c_int = 10i32;
```

### `NFPROTO_DECNET`
```rust
const NFPROTO_DECNET: c_int = 12i32;
```

### `NFPROTO_NUMPROTO`
```rust
const NFPROTO_NUMPROTO: c_int = 13i32;
```

### `NF_ARP`
```rust
const NF_ARP: c_int = 0i32;
```

### `NF_ARP_IN`
```rust
const NF_ARP_IN: c_int = 0i32;
```

### `NF_ARP_OUT`
```rust
const NF_ARP_OUT: c_int = 1i32;
```

### `NF_ARP_FORWARD`
```rust
const NF_ARP_FORWARD: c_int = 2i32;
```

### `NF_ARP_NUMHOOKS`
```rust
const NF_ARP_NUMHOOKS: c_int = 3i32;
```

### `NF_BR_PRE_ROUTING`
```rust
const NF_BR_PRE_ROUTING: c_int = 0i32;
```

### `NF_BR_LOCAL_IN`
```rust
const NF_BR_LOCAL_IN: c_int = 1i32;
```

### `NF_BR_FORWARD`
```rust
const NF_BR_FORWARD: c_int = 2i32;
```

### `NF_BR_LOCAL_OUT`
```rust
const NF_BR_LOCAL_OUT: c_int = 3i32;
```

### `NF_BR_POST_ROUTING`
```rust
const NF_BR_POST_ROUTING: c_int = 4i32;
```

### `NF_BR_BROUTING`
```rust
const NF_BR_BROUTING: c_int = 5i32;
```

### `NF_BR_NUMHOOKS`
```rust
const NF_BR_NUMHOOKS: c_int = 6i32;
```

### `NF_BR_PRI_FIRST`
```rust
const NF_BR_PRI_FIRST: c_int = -2_147_483_648i32;
```

### `NF_BR_PRI_NAT_DST_BRIDGED`
```rust
const NF_BR_PRI_NAT_DST_BRIDGED: c_int = -300i32;
```

### `NF_BR_PRI_FILTER_BRIDGED`
```rust
const NF_BR_PRI_FILTER_BRIDGED: c_int = -200i32;
```

### `NF_BR_PRI_BRNF`
```rust
const NF_BR_PRI_BRNF: c_int = 0i32;
```

### `NF_BR_PRI_NAT_DST_OTHER`
```rust
const NF_BR_PRI_NAT_DST_OTHER: c_int = 100i32;
```

### `NF_BR_PRI_FILTER_OTHER`
```rust
const NF_BR_PRI_FILTER_OTHER: c_int = 200i32;
```

### `NF_BR_PRI_NAT_SRC`
```rust
const NF_BR_PRI_NAT_SRC: c_int = 300i32;
```

### `NF_BR_PRI_LAST`
```rust
const NF_BR_PRI_LAST: c_int = 2_147_483_647i32;
```

### `NF_IP_PRE_ROUTING`
```rust
const NF_IP_PRE_ROUTING: c_int = 0i32;
```

### `NF_IP_LOCAL_IN`
```rust
const NF_IP_LOCAL_IN: c_int = 1i32;
```

### `NF_IP_FORWARD`
```rust
const NF_IP_FORWARD: c_int = 2i32;
```

### `NF_IP_LOCAL_OUT`
```rust
const NF_IP_LOCAL_OUT: c_int = 3i32;
```

### `NF_IP_POST_ROUTING`
```rust
const NF_IP_POST_ROUTING: c_int = 4i32;
```

### `NF_IP_NUMHOOKS`
```rust
const NF_IP_NUMHOOKS: c_int = 5i32;
```

### `NF_IP_PRI_FIRST`
```rust
const NF_IP_PRI_FIRST: c_int = -2_147_483_648i32;
```

### `NF_IP_PRI_RAW_BEFORE_DEFRAG`
```rust
const NF_IP_PRI_RAW_BEFORE_DEFRAG: c_int = -450i32;
```

### `NF_IP_PRI_CONNTRACK_DEFRAG`
```rust
const NF_IP_PRI_CONNTRACK_DEFRAG: c_int = -400i32;
```

### `NF_IP_PRI_RAW`
```rust
const NF_IP_PRI_RAW: c_int = -300i32;
```

### `NF_IP_PRI_SELINUX_FIRST`
```rust
const NF_IP_PRI_SELINUX_FIRST: c_int = -225i32;
```

### `NF_IP_PRI_CONNTRACK`
```rust
const NF_IP_PRI_CONNTRACK: c_int = -200i32;
```

### `NF_IP_PRI_MANGLE`
```rust
const NF_IP_PRI_MANGLE: c_int = -150i32;
```

### `NF_IP_PRI_NAT_DST`
```rust
const NF_IP_PRI_NAT_DST: c_int = -100i32;
```

### `NF_IP_PRI_FILTER`
```rust
const NF_IP_PRI_FILTER: c_int = 0i32;
```

### `NF_IP_PRI_SECURITY`
```rust
const NF_IP_PRI_SECURITY: c_int = 50i32;
```

### `NF_IP_PRI_NAT_SRC`
```rust
const NF_IP_PRI_NAT_SRC: c_int = 100i32;
```

### `NF_IP_PRI_SELINUX_LAST`
```rust
const NF_IP_PRI_SELINUX_LAST: c_int = 225i32;
```

### `NF_IP_PRI_CONNTRACK_HELPER`
```rust
const NF_IP_PRI_CONNTRACK_HELPER: c_int = 300i32;
```

### `NF_IP_PRI_CONNTRACK_CONFIRM`
```rust
const NF_IP_PRI_CONNTRACK_CONFIRM: c_int = 2_147_483_647i32;
```

### `NF_IP_PRI_LAST`
```rust
const NF_IP_PRI_LAST: c_int = 2_147_483_647i32;
```

### `NF_IP6_PRE_ROUTING`
```rust
const NF_IP6_PRE_ROUTING: c_int = 0i32;
```

### `NF_IP6_LOCAL_IN`
```rust
const NF_IP6_LOCAL_IN: c_int = 1i32;
```

### `NF_IP6_FORWARD`
```rust
const NF_IP6_FORWARD: c_int = 2i32;
```

### `NF_IP6_LOCAL_OUT`
```rust
const NF_IP6_LOCAL_OUT: c_int = 3i32;
```

### `NF_IP6_POST_ROUTING`
```rust
const NF_IP6_POST_ROUTING: c_int = 4i32;
```

### `NF_IP6_NUMHOOKS`
```rust
const NF_IP6_NUMHOOKS: c_int = 5i32;
```

### `NF_IP6_PRI_FIRST`
```rust
const NF_IP6_PRI_FIRST: c_int = -2_147_483_648i32;
```

### `NF_IP6_PRI_RAW_BEFORE_DEFRAG`
```rust
const NF_IP6_PRI_RAW_BEFORE_DEFRAG: c_int = -450i32;
```

### `NF_IP6_PRI_CONNTRACK_DEFRAG`
```rust
const NF_IP6_PRI_CONNTRACK_DEFRAG: c_int = -400i32;
```

### `NF_IP6_PRI_RAW`
```rust
const NF_IP6_PRI_RAW: c_int = -300i32;
```

### `NF_IP6_PRI_SELINUX_FIRST`
```rust
const NF_IP6_PRI_SELINUX_FIRST: c_int = -225i32;
```

### `NF_IP6_PRI_CONNTRACK`
```rust
const NF_IP6_PRI_CONNTRACK: c_int = -200i32;
```

### `NF_IP6_PRI_MANGLE`
```rust
const NF_IP6_PRI_MANGLE: c_int = -150i32;
```

### `NF_IP6_PRI_NAT_DST`
```rust
const NF_IP6_PRI_NAT_DST: c_int = -100i32;
```

### `NF_IP6_PRI_FILTER`
```rust
const NF_IP6_PRI_FILTER: c_int = 0i32;
```

### `NF_IP6_PRI_SECURITY`
```rust
const NF_IP6_PRI_SECURITY: c_int = 50i32;
```

### `NF_IP6_PRI_NAT_SRC`
```rust
const NF_IP6_PRI_NAT_SRC: c_int = 100i32;
```

### `NF_IP6_PRI_SELINUX_LAST`
```rust
const NF_IP6_PRI_SELINUX_LAST: c_int = 225i32;
```

### `NF_IP6_PRI_CONNTRACK_HELPER`
```rust
const NF_IP6_PRI_CONNTRACK_HELPER: c_int = 300i32;
```

### `NF_IP6_PRI_LAST`
```rust
const NF_IP6_PRI_LAST: c_int = 2_147_483_647i32;
```

### `IP6T_SO_ORIGINAL_DST`
```rust
const IP6T_SO_ORIGINAL_DST: c_int = 80i32;
```

### `SIOCSHWTSTAMP`
```rust
const SIOCSHWTSTAMP: c_ulong = 35_248u64;
```

### `SIOCGHWTSTAMP`
```rust
const SIOCGHWTSTAMP: c_ulong = 35_249u64;
```

### `WIRELESS_EXT`
```rust
const WIRELESS_EXT: c_ulong = 22u64;
```

### `SIOCSIWCOMMIT`
```rust
const SIOCSIWCOMMIT: c_ulong = 35_584u64;
```

### `SIOCGIWNAME`
```rust
const SIOCGIWNAME: c_ulong = 35_585u64;
```

### `SIOCSIWNWID`
```rust
const SIOCSIWNWID: c_ulong = 35_586u64;
```

### `SIOCGIWNWID`
```rust
const SIOCGIWNWID: c_ulong = 35_587u64;
```

### `SIOCSIWFREQ`
```rust
const SIOCSIWFREQ: c_ulong = 35_588u64;
```

### `SIOCGIWFREQ`
```rust
const SIOCGIWFREQ: c_ulong = 35_589u64;
```

### `SIOCSIWMODE`
```rust
const SIOCSIWMODE: c_ulong = 35_590u64;
```

### `SIOCGIWMODE`
```rust
const SIOCGIWMODE: c_ulong = 35_591u64;
```

### `SIOCSIWSENS`
```rust
const SIOCSIWSENS: c_ulong = 35_592u64;
```

### `SIOCGIWSENS`
```rust
const SIOCGIWSENS: c_ulong = 35_593u64;
```

### `SIOCSIWRANGE`
```rust
const SIOCSIWRANGE: c_ulong = 35_594u64;
```

### `SIOCGIWRANGE`
```rust
const SIOCGIWRANGE: c_ulong = 35_595u64;
```

### `SIOCSIWPRIV`
```rust
const SIOCSIWPRIV: c_ulong = 35_596u64;
```

### `SIOCGIWPRIV`
```rust
const SIOCGIWPRIV: c_ulong = 35_597u64;
```

### `SIOCSIWSTATS`
```rust
const SIOCSIWSTATS: c_ulong = 35_598u64;
```

### `SIOCGIWSTATS`
```rust
const SIOCGIWSTATS: c_ulong = 35_599u64;
```

### `SIOCSIWSPY`
```rust
const SIOCSIWSPY: c_ulong = 35_600u64;
```

### `SIOCGIWSPY`
```rust
const SIOCGIWSPY: c_ulong = 35_601u64;
```

### `SIOCSIWTHRSPY`
```rust
const SIOCSIWTHRSPY: c_ulong = 35_602u64;
```

### `SIOCGIWTHRSPY`
```rust
const SIOCGIWTHRSPY: c_ulong = 35_603u64;
```

### `SIOCSIWAP`
```rust
const SIOCSIWAP: c_ulong = 35_604u64;
```

### `SIOCGIWAP`
```rust
const SIOCGIWAP: c_ulong = 35_605u64;
```

### `SIOCGIWAPLIST`
```rust
const SIOCGIWAPLIST: c_ulong = 35_607u64;
```

### `SIOCSIWSCAN`
```rust
const SIOCSIWSCAN: c_ulong = 35_608u64;
```

### `SIOCGIWSCAN`
```rust
const SIOCGIWSCAN: c_ulong = 35_609u64;
```

### `SIOCSIWESSID`
```rust
const SIOCSIWESSID: c_ulong = 35_610u64;
```

### `SIOCGIWESSID`
```rust
const SIOCGIWESSID: c_ulong = 35_611u64;
```

### `SIOCSIWNICKN`
```rust
const SIOCSIWNICKN: c_ulong = 35_612u64;
```

### `SIOCGIWNICKN`
```rust
const SIOCGIWNICKN: c_ulong = 35_613u64;
```

### `SIOCSIWRATE`
```rust
const SIOCSIWRATE: c_ulong = 35_616u64;
```

### `SIOCGIWRATE`
```rust
const SIOCGIWRATE: c_ulong = 35_617u64;
```

### `SIOCSIWRTS`
```rust
const SIOCSIWRTS: c_ulong = 35_618u64;
```

### `SIOCGIWRTS`
```rust
const SIOCGIWRTS: c_ulong = 35_619u64;
```

### `SIOCSIWFRAG`
```rust
const SIOCSIWFRAG: c_ulong = 35_620u64;
```

### `SIOCGIWFRAG`
```rust
const SIOCGIWFRAG: c_ulong = 35_621u64;
```

### `SIOCSIWTXPOW`
```rust
const SIOCSIWTXPOW: c_ulong = 35_622u64;
```

### `SIOCGIWTXPOW`
```rust
const SIOCGIWTXPOW: c_ulong = 35_623u64;
```

### `SIOCSIWRETRY`
```rust
const SIOCSIWRETRY: c_ulong = 35_624u64;
```

### `SIOCGIWRETRY`
```rust
const SIOCGIWRETRY: c_ulong = 35_625u64;
```

### `SIOCSIWENCODE`
```rust
const SIOCSIWENCODE: c_ulong = 35_626u64;
```

### `SIOCGIWENCODE`
```rust
const SIOCGIWENCODE: c_ulong = 35_627u64;
```

### `SIOCSIWPOWER`
```rust
const SIOCSIWPOWER: c_ulong = 35_628u64;
```

### `SIOCGIWPOWER`
```rust
const SIOCGIWPOWER: c_ulong = 35_629u64;
```

### `SIOCSIWGENIE`
```rust
const SIOCSIWGENIE: c_ulong = 35_632u64;
```

### `SIOCGIWGENIE`
```rust
const SIOCGIWGENIE: c_ulong = 35_633u64;
```

### `SIOCSIWMLME`
```rust
const SIOCSIWMLME: c_ulong = 35_606u64;
```

### `SIOCSIWAUTH`
```rust
const SIOCSIWAUTH: c_ulong = 35_634u64;
```

### `SIOCGIWAUTH`
```rust
const SIOCGIWAUTH: c_ulong = 35_635u64;
```

### `SIOCSIWENCODEEXT`
```rust
const SIOCSIWENCODEEXT: c_ulong = 35_636u64;
```

### `SIOCGIWENCODEEXT`
```rust
const SIOCGIWENCODEEXT: c_ulong = 35_637u64;
```

### `SIOCSIWPMKSA`
```rust
const SIOCSIWPMKSA: c_ulong = 35_638u64;
```

### `SIOCIWFIRSTPRIV`
```rust
const SIOCIWFIRSTPRIV: c_ulong = 35_808u64;
```

### `SIOCIWLASTPRIV`
```rust
const SIOCIWLASTPRIV: c_ulong = 35_839u64;
```

### `SIOCIWFIRST`
```rust
const SIOCIWFIRST: c_ulong = 35_584u64;
```

### `SIOCIWLAST`
```rust
const SIOCIWLAST: c_ulong = 35_839u64;
```

### `IWEVTXDROP`
```rust
const IWEVTXDROP: c_ulong = 35_840u64;
```

### `IWEVQUAL`
```rust
const IWEVQUAL: c_ulong = 35_841u64;
```

### `IWEVCUSTOM`
```rust
const IWEVCUSTOM: c_ulong = 35_842u64;
```

### `IWEVREGISTERED`
```rust
const IWEVREGISTERED: c_ulong = 35_843u64;
```

### `IWEVEXPIRED`
```rust
const IWEVEXPIRED: c_ulong = 35_844u64;
```

### `IWEVGENIE`
```rust
const IWEVGENIE: c_ulong = 35_845u64;
```

### `IWEVMICHAELMICFAILURE`
```rust
const IWEVMICHAELMICFAILURE: c_ulong = 35_846u64;
```

### `IWEVASSOCREQIE`
```rust
const IWEVASSOCREQIE: c_ulong = 35_847u64;
```

### `IWEVASSOCRESPIE`
```rust
const IWEVASSOCRESPIE: c_ulong = 35_848u64;
```

### `IWEVPMKIDCAND`
```rust
const IWEVPMKIDCAND: c_ulong = 35_849u64;
```

### `IWEVFIRST`
```rust
const IWEVFIRST: c_ulong = 35_840u64;
```

### `IW_PRIV_TYPE_MASK`
```rust
const IW_PRIV_TYPE_MASK: c_ulong = 28_672u64;
```

### `IW_PRIV_TYPE_NONE`
```rust
const IW_PRIV_TYPE_NONE: c_ulong = 0u64;
```

### `IW_PRIV_TYPE_BYTE`
```rust
const IW_PRIV_TYPE_BYTE: c_ulong = 4_096u64;
```

### `IW_PRIV_TYPE_CHAR`
```rust
const IW_PRIV_TYPE_CHAR: c_ulong = 8_192u64;
```

### `IW_PRIV_TYPE_INT`
```rust
const IW_PRIV_TYPE_INT: c_ulong = 16_384u64;
```

### `IW_PRIV_TYPE_FLOAT`
```rust
const IW_PRIV_TYPE_FLOAT: c_ulong = 20_480u64;
```

### `IW_PRIV_TYPE_ADDR`
```rust
const IW_PRIV_TYPE_ADDR: c_ulong = 24_576u64;
```

### `IW_PRIV_SIZE_FIXED`
```rust
const IW_PRIV_SIZE_FIXED: c_ulong = 2_048u64;
```

### `IW_PRIV_SIZE_MASK`
```rust
const IW_PRIV_SIZE_MASK: c_ulong = 2_047u64;
```

### `IW_MAX_FREQUENCIES`
```rust
const IW_MAX_FREQUENCIES: usize = 32usize;
```

### `IW_MAX_BITRATES`
```rust
const IW_MAX_BITRATES: usize = 32usize;
```

### `IW_MAX_TXPOWER`
```rust
const IW_MAX_TXPOWER: usize = 8usize;
```

### `IW_MAX_SPY`
```rust
const IW_MAX_SPY: usize = 8usize;
```

### `IW_MAX_AP`
```rust
const IW_MAX_AP: usize = 64usize;
```

### `IW_ESSID_MAX_SIZE`
```rust
const IW_ESSID_MAX_SIZE: usize = 32usize;
```

### `IW_MODE_AUTO`
```rust
const IW_MODE_AUTO: usize = 0usize;
```

### `IW_MODE_ADHOC`
```rust
const IW_MODE_ADHOC: usize = 1usize;
```

### `IW_MODE_INFRA`
```rust
const IW_MODE_INFRA: usize = 2usize;
```

### `IW_MODE_MASTER`
```rust
const IW_MODE_MASTER: usize = 3usize;
```

### `IW_MODE_REPEAT`
```rust
const IW_MODE_REPEAT: usize = 4usize;
```

### `IW_MODE_SECOND`
```rust
const IW_MODE_SECOND: usize = 5usize;
```

### `IW_MODE_MONITOR`
```rust
const IW_MODE_MONITOR: usize = 6usize;
```

### `IW_MODE_MESH`
```rust
const IW_MODE_MESH: usize = 7usize;
```

### `IW_QUAL_QUAL_UPDATED`
```rust
const IW_QUAL_QUAL_UPDATED: c_ulong = 1u64;
```

### `IW_QUAL_LEVEL_UPDATED`
```rust
const IW_QUAL_LEVEL_UPDATED: c_ulong = 2u64;
```

### `IW_QUAL_NOISE_UPDATED`
```rust
const IW_QUAL_NOISE_UPDATED: c_ulong = 4u64;
```

### `IW_QUAL_ALL_UPDATED`
```rust
const IW_QUAL_ALL_UPDATED: c_ulong = 7u64;
```

### `IW_QUAL_DBM`
```rust
const IW_QUAL_DBM: c_ulong = 8u64;
```

### `IW_QUAL_QUAL_INVALID`
```rust
const IW_QUAL_QUAL_INVALID: c_ulong = 16u64;
```

### `IW_QUAL_LEVEL_INVALID`
```rust
const IW_QUAL_LEVEL_INVALID: c_ulong = 32u64;
```

### `IW_QUAL_NOISE_INVALID`
```rust
const IW_QUAL_NOISE_INVALID: c_ulong = 64u64;
```

### `IW_QUAL_RCPI`
```rust
const IW_QUAL_RCPI: c_ulong = 128u64;
```

### `IW_QUAL_ALL_INVALID`
```rust
const IW_QUAL_ALL_INVALID: c_ulong = 112u64;
```

### `IW_FREQ_AUTO`
```rust
const IW_FREQ_AUTO: c_ulong = 0u64;
```

### `IW_FREQ_FIXED`
```rust
const IW_FREQ_FIXED: c_ulong = 1u64;
```

### `IW_MAX_ENCODING_SIZES`
```rust
const IW_MAX_ENCODING_SIZES: usize = 8usize;
```

### `IW_ENCODING_TOKEN_MAX`
```rust
const IW_ENCODING_TOKEN_MAX: usize = 64usize;
```

### `IW_ENCODE_INDEX`
```rust
const IW_ENCODE_INDEX: c_ulong = 255u64;
```

### `IW_ENCODE_FLAGS`
```rust
const IW_ENCODE_FLAGS: c_ulong = 65_280u64;
```

### `IW_ENCODE_MODE`
```rust
const IW_ENCODE_MODE: c_ulong = 61_440u64;
```

### `IW_ENCODE_DISABLED`
```rust
const IW_ENCODE_DISABLED: c_ulong = 32_768u64;
```

### `IW_ENCODE_ENABLED`
```rust
const IW_ENCODE_ENABLED: c_ulong = 0u64;
```

### `IW_ENCODE_RESTRICTED`
```rust
const IW_ENCODE_RESTRICTED: c_ulong = 16_384u64;
```

### `IW_ENCODE_OPEN`
```rust
const IW_ENCODE_OPEN: c_ulong = 8_192u64;
```

### `IW_ENCODE_NOKEY`
```rust
const IW_ENCODE_NOKEY: c_ulong = 2_048u64;
```

### `IW_ENCODE_TEMP`
```rust
const IW_ENCODE_TEMP: c_ulong = 1_024u64;
```

### `IW_POWER_ON`
```rust
const IW_POWER_ON: c_ulong = 0u64;
```

### `IW_POWER_TYPE`
```rust
const IW_POWER_TYPE: c_ulong = 61_440u64;
```

### `IW_POWER_PERIOD`
```rust
const IW_POWER_PERIOD: c_ulong = 4_096u64;
```

### `IW_POWER_TIMEOUT`
```rust
const IW_POWER_TIMEOUT: c_ulong = 8_192u64;
```

### `IW_POWER_MODE`
```rust
const IW_POWER_MODE: c_ulong = 3_840u64;
```

### `IW_POWER_UNICAST_R`
```rust
const IW_POWER_UNICAST_R: c_ulong = 256u64;
```

### `IW_POWER_MULTICAST_R`
```rust
const IW_POWER_MULTICAST_R: c_ulong = 512u64;
```

### `IW_POWER_ALL_R`
```rust
const IW_POWER_ALL_R: c_ulong = 768u64;
```

### `IW_POWER_FORCE_S`
```rust
const IW_POWER_FORCE_S: c_ulong = 1_024u64;
```

### `IW_POWER_REPEATER`
```rust
const IW_POWER_REPEATER: c_ulong = 2_048u64;
```

### `IW_POWER_MODIFIER`
```rust
const IW_POWER_MODIFIER: c_ulong = 15u64;
```

### `IW_POWER_MIN`
```rust
const IW_POWER_MIN: c_ulong = 1u64;
```

### `IW_POWER_MAX`
```rust
const IW_POWER_MAX: c_ulong = 2u64;
```

### `IW_POWER_RELATIVE`
```rust
const IW_POWER_RELATIVE: c_ulong = 4u64;
```

### `IW_TXPOW_TYPE`
```rust
const IW_TXPOW_TYPE: c_ulong = 255u64;
```

### `IW_TXPOW_DBM`
```rust
const IW_TXPOW_DBM: c_ulong = 0u64;
```

### `IW_TXPOW_MWATT`
```rust
const IW_TXPOW_MWATT: c_ulong = 1u64;
```

### `IW_TXPOW_RELATIVE`
```rust
const IW_TXPOW_RELATIVE: c_ulong = 2u64;
```

### `IW_TXPOW_RANGE`
```rust
const IW_TXPOW_RANGE: c_ulong = 4_096u64;
```

### `IW_RETRY_ON`
```rust
const IW_RETRY_ON: c_ulong = 0u64;
```

### `IW_RETRY_TYPE`
```rust
const IW_RETRY_TYPE: c_ulong = 61_440u64;
```

### `IW_RETRY_LIMIT`
```rust
const IW_RETRY_LIMIT: c_ulong = 4_096u64;
```

### `IW_RETRY_LIFETIME`
```rust
const IW_RETRY_LIFETIME: c_ulong = 8_192u64;
```

### `IW_RETRY_MODIFIER`
```rust
const IW_RETRY_MODIFIER: c_ulong = 255u64;
```

### `IW_RETRY_MIN`
```rust
const IW_RETRY_MIN: c_ulong = 1u64;
```

### `IW_RETRY_MAX`
```rust
const IW_RETRY_MAX: c_ulong = 2u64;
```

### `IW_RETRY_RELATIVE`
```rust
const IW_RETRY_RELATIVE: c_ulong = 4u64;
```

### `IW_RETRY_SHORT`
```rust
const IW_RETRY_SHORT: c_ulong = 16u64;
```

### `IW_RETRY_LONG`
```rust
const IW_RETRY_LONG: c_ulong = 32u64;
```

### `IW_SCAN_DEFAULT`
```rust
const IW_SCAN_DEFAULT: c_ulong = 0u64;
```

### `IW_SCAN_ALL_ESSID`
```rust
const IW_SCAN_ALL_ESSID: c_ulong = 1u64;
```

### `IW_SCAN_THIS_ESSID`
```rust
const IW_SCAN_THIS_ESSID: c_ulong = 2u64;
```

### `IW_SCAN_ALL_FREQ`
```rust
const IW_SCAN_ALL_FREQ: c_ulong = 4u64;
```

### `IW_SCAN_THIS_FREQ`
```rust
const IW_SCAN_THIS_FREQ: c_ulong = 8u64;
```

### `IW_SCAN_ALL_MODE`
```rust
const IW_SCAN_ALL_MODE: c_ulong = 16u64;
```

### `IW_SCAN_THIS_MODE`
```rust
const IW_SCAN_THIS_MODE: c_ulong = 32u64;
```

### `IW_SCAN_ALL_RATE`
```rust
const IW_SCAN_ALL_RATE: c_ulong = 64u64;
```

### `IW_SCAN_THIS_RATE`
```rust
const IW_SCAN_THIS_RATE: c_ulong = 128u64;
```

### `IW_SCAN_TYPE_ACTIVE`
```rust
const IW_SCAN_TYPE_ACTIVE: usize = 0usize;
```

### `IW_SCAN_TYPE_PASSIVE`
```rust
const IW_SCAN_TYPE_PASSIVE: usize = 1usize;
```

### `IW_SCAN_MAX_DATA`
```rust
const IW_SCAN_MAX_DATA: usize = 4_096usize;
```

### `IW_SCAN_CAPA_NONE`
```rust
const IW_SCAN_CAPA_NONE: c_ulong = 0u64;
```

### `IW_SCAN_CAPA_ESSID`
```rust
const IW_SCAN_CAPA_ESSID: c_ulong = 1u64;
```

### `IW_SCAN_CAPA_BSSID`
```rust
const IW_SCAN_CAPA_BSSID: c_ulong = 2u64;
```

### `IW_SCAN_CAPA_CHANNEL`
```rust
const IW_SCAN_CAPA_CHANNEL: c_ulong = 4u64;
```

### `IW_SCAN_CAPA_MODE`
```rust
const IW_SCAN_CAPA_MODE: c_ulong = 8u64;
```

### `IW_SCAN_CAPA_RATE`
```rust
const IW_SCAN_CAPA_RATE: c_ulong = 16u64;
```

### `IW_SCAN_CAPA_TYPE`
```rust
const IW_SCAN_CAPA_TYPE: c_ulong = 32u64;
```

### `IW_SCAN_CAPA_TIME`
```rust
const IW_SCAN_CAPA_TIME: c_ulong = 64u64;
```

### `IW_CUSTOM_MAX`
```rust
const IW_CUSTOM_MAX: c_ulong = 256u64;
```

### `IW_GENERIC_IE_MAX`
```rust
const IW_GENERIC_IE_MAX: c_ulong = 1_024u64;
```

### `IW_MLME_DEAUTH`
```rust
const IW_MLME_DEAUTH: c_ulong = 0u64;
```

### `IW_MLME_DISASSOC`
```rust
const IW_MLME_DISASSOC: c_ulong = 1u64;
```

### `IW_MLME_AUTH`
```rust
const IW_MLME_AUTH: c_ulong = 2u64;
```

### `IW_MLME_ASSOC`
```rust
const IW_MLME_ASSOC: c_ulong = 3u64;
```

### `IW_AUTH_INDEX`
```rust
const IW_AUTH_INDEX: c_ulong = 4_095u64;
```

### `IW_AUTH_FLAGS`
```rust
const IW_AUTH_FLAGS: c_ulong = 61_440u64;
```

### `IW_AUTH_WPA_VERSION`
```rust
const IW_AUTH_WPA_VERSION: usize = 0usize;
```

### `IW_AUTH_CIPHER_PAIRWISE`
```rust
const IW_AUTH_CIPHER_PAIRWISE: usize = 1usize;
```

### `IW_AUTH_CIPHER_GROUP`
```rust
const IW_AUTH_CIPHER_GROUP: usize = 2usize;
```

### `IW_AUTH_KEY_MGMT`
```rust
const IW_AUTH_KEY_MGMT: usize = 3usize;
```

### `IW_AUTH_TKIP_COUNTERMEASURES`
```rust
const IW_AUTH_TKIP_COUNTERMEASURES: usize = 4usize;
```

### `IW_AUTH_DROP_UNENCRYPTED`
```rust
const IW_AUTH_DROP_UNENCRYPTED: usize = 5usize;
```

### `IW_AUTH_80211_AUTH_ALG`
```rust
const IW_AUTH_80211_AUTH_ALG: usize = 6usize;
```

### `IW_AUTH_WPA_ENABLED`
```rust
const IW_AUTH_WPA_ENABLED: usize = 7usize;
```

### `IW_AUTH_RX_UNENCRYPTED_EAPOL`
```rust
const IW_AUTH_RX_UNENCRYPTED_EAPOL: usize = 8usize;
```

### `IW_AUTH_ROAMING_CONTROL`
```rust
const IW_AUTH_ROAMING_CONTROL: usize = 9usize;
```

### `IW_AUTH_PRIVACY_INVOKED`
```rust
const IW_AUTH_PRIVACY_INVOKED: usize = 10usize;
```

### `IW_AUTH_CIPHER_GROUP_MGMT`
```rust
const IW_AUTH_CIPHER_GROUP_MGMT: usize = 11usize;
```

### `IW_AUTH_MFP`
```rust
const IW_AUTH_MFP: usize = 12usize;
```

### `IW_AUTH_WPA_VERSION_DISABLED`
```rust
const IW_AUTH_WPA_VERSION_DISABLED: c_ulong = 1u64;
```

### `IW_AUTH_WPA_VERSION_WPA`
```rust
const IW_AUTH_WPA_VERSION_WPA: c_ulong = 2u64;
```

### `IW_AUTH_WPA_VERSION_WPA2`
```rust
const IW_AUTH_WPA_VERSION_WPA2: c_ulong = 4u64;
```

### `IW_AUTH_CIPHER_NONE`
```rust
const IW_AUTH_CIPHER_NONE: c_ulong = 1u64;
```

### `IW_AUTH_CIPHER_WEP40`
```rust
const IW_AUTH_CIPHER_WEP40: c_ulong = 2u64;
```

### `IW_AUTH_CIPHER_TKIP`
```rust
const IW_AUTH_CIPHER_TKIP: c_ulong = 4u64;
```

### `IW_AUTH_CIPHER_CCMP`
```rust
const IW_AUTH_CIPHER_CCMP: c_ulong = 8u64;
```

### `IW_AUTH_CIPHER_WEP104`
```rust
const IW_AUTH_CIPHER_WEP104: c_ulong = 16u64;
```

### `IW_AUTH_CIPHER_AES_CMAC`
```rust
const IW_AUTH_CIPHER_AES_CMAC: c_ulong = 32u64;
```

### `IW_AUTH_KEY_MGMT_802_1X`
```rust
const IW_AUTH_KEY_MGMT_802_1X: usize = 1usize;
```

### `IW_AUTH_KEY_MGMT_PSK`
```rust
const IW_AUTH_KEY_MGMT_PSK: usize = 2usize;
```

### `IW_AUTH_ALG_OPEN_SYSTEM`
```rust
const IW_AUTH_ALG_OPEN_SYSTEM: c_ulong = 1u64;
```

### `IW_AUTH_ALG_SHARED_KEY`
```rust
const IW_AUTH_ALG_SHARED_KEY: c_ulong = 2u64;
```

### `IW_AUTH_ALG_LEAP`
```rust
const IW_AUTH_ALG_LEAP: c_ulong = 4u64;
```

### `IW_AUTH_ROAMING_ENABLE`
```rust
const IW_AUTH_ROAMING_ENABLE: usize = 0usize;
```

### `IW_AUTH_ROAMING_DISABLE`
```rust
const IW_AUTH_ROAMING_DISABLE: usize = 1usize;
```

### `IW_AUTH_MFP_DISABLED`
```rust
const IW_AUTH_MFP_DISABLED: usize = 0usize;
```

### `IW_AUTH_MFP_OPTIONAL`
```rust
const IW_AUTH_MFP_OPTIONAL: usize = 1usize;
```

### `IW_AUTH_MFP_REQUIRED`
```rust
const IW_AUTH_MFP_REQUIRED: usize = 2usize;
```

### `IW_ENCODE_SEQ_MAX_SIZE`
```rust
const IW_ENCODE_SEQ_MAX_SIZE: usize = 8usize;
```

### `IW_ENCODE_ALG_NONE`
```rust
const IW_ENCODE_ALG_NONE: usize = 0usize;
```

### `IW_ENCODE_ALG_WEP`
```rust
const IW_ENCODE_ALG_WEP: usize = 1usize;
```

### `IW_ENCODE_ALG_TKIP`
```rust
const IW_ENCODE_ALG_TKIP: usize = 2usize;
```

### `IW_ENCODE_ALG_CCMP`
```rust
const IW_ENCODE_ALG_CCMP: usize = 3usize;
```

### `IW_ENCODE_ALG_PMK`
```rust
const IW_ENCODE_ALG_PMK: usize = 4usize;
```

### `IW_ENCODE_ALG_AES_CMAC`
```rust
const IW_ENCODE_ALG_AES_CMAC: usize = 5usize;
```

### `IW_ENCODE_EXT_TX_SEQ_VALID`
```rust
const IW_ENCODE_EXT_TX_SEQ_VALID: c_ulong = 1u64;
```

### `IW_ENCODE_EXT_RX_SEQ_VALID`
```rust
const IW_ENCODE_EXT_RX_SEQ_VALID: c_ulong = 2u64;
```

### `IW_ENCODE_EXT_GROUP_KEY`
```rust
const IW_ENCODE_EXT_GROUP_KEY: c_ulong = 4u64;
```

### `IW_ENCODE_EXT_SET_TX_KEY`
```rust
const IW_ENCODE_EXT_SET_TX_KEY: c_ulong = 8u64;
```

### `IW_MICFAILURE_KEY_ID`
```rust
const IW_MICFAILURE_KEY_ID: c_ulong = 3u64;
```

### `IW_MICFAILURE_GROUP`
```rust
const IW_MICFAILURE_GROUP: c_ulong = 4u64;
```

### `IW_MICFAILURE_PAIRWISE`
```rust
const IW_MICFAILURE_PAIRWISE: c_ulong = 8u64;
```

### `IW_MICFAILURE_STAKEY`
```rust
const IW_MICFAILURE_STAKEY: c_ulong = 16u64;
```

### `IW_MICFAILURE_COUNT`
```rust
const IW_MICFAILURE_COUNT: c_ulong = 96u64;
```

### `IW_ENC_CAPA_WPA`
```rust
const IW_ENC_CAPA_WPA: c_ulong = 1u64;
```

### `IW_ENC_CAPA_WPA2`
```rust
const IW_ENC_CAPA_WPA2: c_ulong = 2u64;
```

### `IW_ENC_CAPA_CIPHER_TKIP`
```rust
const IW_ENC_CAPA_CIPHER_TKIP: c_ulong = 4u64;
```

### `IW_ENC_CAPA_CIPHER_CCMP`
```rust
const IW_ENC_CAPA_CIPHER_CCMP: c_ulong = 8u64;
```

### `IW_ENC_CAPA_4WAY_HANDSHAKE`
```rust
const IW_ENC_CAPA_4WAY_HANDSHAKE: c_ulong = 16u64;
```

### `IW_EVENT_CAPA_K_0`
```rust
const IW_EVENT_CAPA_K_0: c_ulong = 67_108_944u64;
```

### `IW_EVENT_CAPA_K_1`
```rust
const IW_EVENT_CAPA_K_1: c_ulong = 1_024u64;
```

### `IW_PMKSA_ADD`
```rust
const IW_PMKSA_ADD: usize = 1usize;
```

### `IW_PMKSA_REMOVE`
```rust
const IW_PMKSA_REMOVE: usize = 2usize;
```

### `IW_PMKSA_FLUSH`
```rust
const IW_PMKSA_FLUSH: usize = 3usize;
```

### `IW_PMKID_LEN`
```rust
const IW_PMKID_LEN: usize = 16usize;
```

### `IW_PMKID_CAND_PREAUTH`
```rust
const IW_PMKID_CAND_PREAUTH: c_ulong = 1u64;
```

### `IW_EV_LCP_PK_LEN`
```rust
const IW_EV_LCP_PK_LEN: usize = 4usize;
```

### `IW_EV_CHAR_PK_LEN`
```rust
const IW_EV_CHAR_PK_LEN: usize = 20usize;
```

### `IW_EV_UINT_PK_LEN`
```rust
const IW_EV_UINT_PK_LEN: usize = 8usize;
```

### `IW_EV_FREQ_PK_LEN`
```rust
const IW_EV_FREQ_PK_LEN: usize = 12usize;
```

### `IW_EV_PARAM_PK_LEN`
```rust
const IW_EV_PARAM_PK_LEN: usize = 12usize;
```

### `IW_EV_ADDR_PK_LEN`
```rust
const IW_EV_ADDR_PK_LEN: usize = 20usize;
```

### `IW_EV_QUAL_PK_LEN`
```rust
const IW_EV_QUAL_PK_LEN: usize = 8usize;
```

### `IW_EV_POINT_PK_LEN`
```rust
const IW_EV_POINT_PK_LEN: usize = 8usize;
```

### `NUD_NONE`
```rust
const NUD_NONE: u16 = 0u16;
```

### `NUD_INCOMPLETE`
```rust
const NUD_INCOMPLETE: u16 = 1u16;
```

### `NUD_REACHABLE`
```rust
const NUD_REACHABLE: u16 = 2u16;
```

### `NUD_STALE`
```rust
const NUD_STALE: u16 = 4u16;
```

### `NUD_DELAY`
```rust
const NUD_DELAY: u16 = 8u16;
```

### `NUD_PROBE`
```rust
const NUD_PROBE: u16 = 16u16;
```

### `NUD_FAILED`
```rust
const NUD_FAILED: u16 = 32u16;
```

### `NUD_NOARP`
```rust
const NUD_NOARP: u16 = 64u16;
```

### `NUD_PERMANENT`
```rust
const NUD_PERMANENT: u16 = 128u16;
```

### `NTF_USE`
```rust
const NTF_USE: u8 = 1u8;
```

### `NTF_SELF`
```rust
const NTF_SELF: u8 = 2u8;
```

### `NTF_MASTER`
```rust
const NTF_MASTER: u8 = 4u8;
```

### `NTF_PROXY`
```rust
const NTF_PROXY: u8 = 8u8;
```

### `NTF_ROUTER`
```rust
const NTF_ROUTER: u8 = 128u8;
```

### `NDA_UNSPEC`
```rust
const NDA_UNSPEC: c_ushort = 0u16;
```

### `NDA_DST`
```rust
const NDA_DST: c_ushort = 1u16;
```

### `NDA_LLADDR`
```rust
const NDA_LLADDR: c_ushort = 2u16;
```

### `NDA_CACHEINFO`
```rust
const NDA_CACHEINFO: c_ushort = 3u16;
```

### `NDA_PROBES`
```rust
const NDA_PROBES: c_ushort = 4u16;
```

### `NDA_VLAN`
```rust
const NDA_VLAN: c_ushort = 5u16;
```

### `NDA_PORT`
```rust
const NDA_PORT: c_ushort = 6u16;
```

### `NDA_VNI`
```rust
const NDA_VNI: c_ushort = 7u16;
```

### `NDA_IFINDEX`
```rust
const NDA_IFINDEX: c_ushort = 8u16;
```

### `NLM_F_BULK`
```rust
const NLM_F_BULK: c_int = 512i32;
```

### `TCA_UNSPEC`
```rust
const TCA_UNSPEC: c_ushort = 0u16;
```

### `TCA_KIND`
```rust
const TCA_KIND: c_ushort = 1u16;
```

### `TCA_OPTIONS`
```rust
const TCA_OPTIONS: c_ushort = 2u16;
```

### `TCA_STATS`
```rust
const TCA_STATS: c_ushort = 3u16;
```

### `TCA_XSTATS`
```rust
const TCA_XSTATS: c_ushort = 4u16;
```

### `TCA_RATE`
```rust
const TCA_RATE: c_ushort = 5u16;
```

### `TCA_FCNT`
```rust
const TCA_FCNT: c_ushort = 6u16;
```

### `TCA_STATS2`
```rust
const TCA_STATS2: c_ushort = 7u16;
```

### `TCA_STAB`
```rust
const TCA_STAB: c_ushort = 8u16;
```

### `RTM_NEWLINK`
```rust
const RTM_NEWLINK: u16 = 16u16;
```

### `RTM_DELLINK`
```rust
const RTM_DELLINK: u16 = 17u16;
```

### `RTM_GETLINK`
```rust
const RTM_GETLINK: u16 = 18u16;
```

### `RTM_SETLINK`
```rust
const RTM_SETLINK: u16 = 19u16;
```

### `RTM_NEWADDR`
```rust
const RTM_NEWADDR: u16 = 20u16;
```

### `RTM_DELADDR`
```rust
const RTM_DELADDR: u16 = 21u16;
```

### `RTM_GETADDR`
```rust
const RTM_GETADDR: u16 = 22u16;
```

### `RTM_NEWROUTE`
```rust
const RTM_NEWROUTE: u16 = 24u16;
```

### `RTM_DELROUTE`
```rust
const RTM_DELROUTE: u16 = 25u16;
```

### `RTM_GETROUTE`
```rust
const RTM_GETROUTE: u16 = 26u16;
```

### `RTM_NEWNEIGH`
```rust
const RTM_NEWNEIGH: u16 = 28u16;
```

### `RTM_DELNEIGH`
```rust
const RTM_DELNEIGH: u16 = 29u16;
```

### `RTM_GETNEIGH`
```rust
const RTM_GETNEIGH: u16 = 30u16;
```

### `RTM_NEWRULE`
```rust
const RTM_NEWRULE: u16 = 32u16;
```

### `RTM_DELRULE`
```rust
const RTM_DELRULE: u16 = 33u16;
```

### `RTM_GETRULE`
```rust
const RTM_GETRULE: u16 = 34u16;
```

### `RTM_NEWQDISC`
```rust
const RTM_NEWQDISC: u16 = 36u16;
```

### `RTM_DELQDISC`
```rust
const RTM_DELQDISC: u16 = 37u16;
```

### `RTM_GETQDISC`
```rust
const RTM_GETQDISC: u16 = 38u16;
```

### `RTM_NEWTCLASS`
```rust
const RTM_NEWTCLASS: u16 = 40u16;
```

### `RTM_DELTCLASS`
```rust
const RTM_DELTCLASS: u16 = 41u16;
```

### `RTM_GETTCLASS`
```rust
const RTM_GETTCLASS: u16 = 42u16;
```

### `RTM_NEWTFILTER`
```rust
const RTM_NEWTFILTER: u16 = 44u16;
```

### `RTM_DELTFILTER`
```rust
const RTM_DELTFILTER: u16 = 45u16;
```

### `RTM_GETTFILTER`
```rust
const RTM_GETTFILTER: u16 = 46u16;
```

### `RTM_NEWACTION`
```rust
const RTM_NEWACTION: u16 = 48u16;
```

### `RTM_DELACTION`
```rust
const RTM_DELACTION: u16 = 49u16;
```

### `RTM_GETACTION`
```rust
const RTM_GETACTION: u16 = 50u16;
```

### `RTM_NEWPREFIX`
```rust
const RTM_NEWPREFIX: u16 = 52u16;
```

### `RTM_GETMULTICAST`
```rust
const RTM_GETMULTICAST: u16 = 58u16;
```

### `RTM_GETANYCAST`
```rust
const RTM_GETANYCAST: u16 = 62u16;
```

### `RTM_NEWNEIGHTBL`
```rust
const RTM_NEWNEIGHTBL: u16 = 64u16;
```

### `RTM_GETNEIGHTBL`
```rust
const RTM_GETNEIGHTBL: u16 = 66u16;
```

### `RTM_SETNEIGHTBL`
```rust
const RTM_SETNEIGHTBL: u16 = 67u16;
```

### `RTM_NEWNDUSEROPT`
```rust
const RTM_NEWNDUSEROPT: u16 = 68u16;
```

### `RTM_NEWADDRLABEL`
```rust
const RTM_NEWADDRLABEL: u16 = 72u16;
```

### `RTM_DELADDRLABEL`
```rust
const RTM_DELADDRLABEL: u16 = 73u16;
```

### `RTM_GETADDRLABEL`
```rust
const RTM_GETADDRLABEL: u16 = 74u16;
```

### `RTM_GETDCB`
```rust
const RTM_GETDCB: u16 = 78u16;
```

### `RTM_SETDCB`
```rust
const RTM_SETDCB: u16 = 79u16;
```

### `RTM_NEWNETCONF`
```rust
const RTM_NEWNETCONF: u16 = 80u16;
```

### `RTM_GETNETCONF`
```rust
const RTM_GETNETCONF: u16 = 82u16;
```

### `RTM_NEWMDB`
```rust
const RTM_NEWMDB: u16 = 84u16;
```

### `RTM_DELMDB`
```rust
const RTM_DELMDB: u16 = 85u16;
```

### `RTM_GETMDB`
```rust
const RTM_GETMDB: u16 = 86u16;
```

### `RTM_NEWNSID`
```rust
const RTM_NEWNSID: u16 = 88u16;
```

### `RTM_DELNSID`
```rust
const RTM_DELNSID: u16 = 89u16;
```

### `RTM_GETNSID`
```rust
const RTM_GETNSID: u16 = 90u16;
```

### `RTM_F_NOTIFY`
```rust
const RTM_F_NOTIFY: c_uint = 256u32;
```

### `RTM_F_CLONED`
```rust
const RTM_F_CLONED: c_uint = 512u32;
```

### `RTM_F_EQUALIZE`
```rust
const RTM_F_EQUALIZE: c_uint = 1_024u32;
```

### `RTM_F_PREFIX`
```rust
const RTM_F_PREFIX: c_uint = 2_048u32;
```

### `RTA_UNSPEC`
```rust
const RTA_UNSPEC: c_ushort = 0u16;
```

### `RTA_DST`
```rust
const RTA_DST: c_ushort = 1u16;
```

### `RTA_SRC`
```rust
const RTA_SRC: c_ushort = 2u16;
```

### `RTA_IIF`
```rust
const RTA_IIF: c_ushort = 3u16;
```

### `RTA_OIF`
```rust
const RTA_OIF: c_ushort = 4u16;
```

### `RTA_GATEWAY`
```rust
const RTA_GATEWAY: c_ushort = 5u16;
```

### `RTA_PRIORITY`
```rust
const RTA_PRIORITY: c_ushort = 6u16;
```

### `RTA_PREFSRC`
```rust
const RTA_PREFSRC: c_ushort = 7u16;
```

### `RTA_METRICS`
```rust
const RTA_METRICS: c_ushort = 8u16;
```

### `RTA_MULTIPATH`
```rust
const RTA_MULTIPATH: c_ushort = 9u16;
```

### `RTA_PROTOINFO`
```rust
const RTA_PROTOINFO: c_ushort = 10u16;
```

### `RTA_FLOW`
```rust
const RTA_FLOW: c_ushort = 11u16;
```

### `RTA_CACHEINFO`
```rust
const RTA_CACHEINFO: c_ushort = 12u16;
```

### `RTA_SESSION`
```rust
const RTA_SESSION: c_ushort = 13u16;
```

### `RTA_MP_ALGO`
```rust
const RTA_MP_ALGO: c_ushort = 14u16;
```

### `RTA_TABLE`
```rust
const RTA_TABLE: c_ushort = 15u16;
```

### `RTA_MARK`
```rust
const RTA_MARK: c_ushort = 16u16;
```

### `RTA_MFC_STATS`
```rust
const RTA_MFC_STATS: c_ushort = 17u16;
```

### `RTN_UNSPEC`
```rust
const RTN_UNSPEC: c_uchar = 0u8;
```

### `RTN_UNICAST`
```rust
const RTN_UNICAST: c_uchar = 1u8;
```

### `RTN_LOCAL`
```rust
const RTN_LOCAL: c_uchar = 2u8;
```

### `RTN_BROADCAST`
```rust
const RTN_BROADCAST: c_uchar = 3u8;
```

### `RTN_ANYCAST`
```rust
const RTN_ANYCAST: c_uchar = 4u8;
```

### `RTN_MULTICAST`
```rust
const RTN_MULTICAST: c_uchar = 5u8;
```

### `RTN_BLACKHOLE`
```rust
const RTN_BLACKHOLE: c_uchar = 6u8;
```

### `RTN_UNREACHABLE`
```rust
const RTN_UNREACHABLE: c_uchar = 7u8;
```

### `RTN_PROHIBIT`
```rust
const RTN_PROHIBIT: c_uchar = 8u8;
```

### `RTN_THROW`
```rust
const RTN_THROW: c_uchar = 9u8;
```

### `RTN_NAT`
```rust
const RTN_NAT: c_uchar = 10u8;
```

### `RTN_XRESOLVE`
```rust
const RTN_XRESOLVE: c_uchar = 11u8;
```

### `RTPROT_UNSPEC`
```rust
const RTPROT_UNSPEC: c_uchar = 0u8;
```

### `RTPROT_REDIRECT`
```rust
const RTPROT_REDIRECT: c_uchar = 1u8;
```

### `RTPROT_KERNEL`
```rust
const RTPROT_KERNEL: c_uchar = 2u8;
```

### `RTPROT_BOOT`
```rust
const RTPROT_BOOT: c_uchar = 3u8;
```

### `RTPROT_STATIC`
```rust
const RTPROT_STATIC: c_uchar = 4u8;
```

### `RT_SCOPE_UNIVERSE`
```rust
const RT_SCOPE_UNIVERSE: c_uchar = 0u8;
```

### `RT_SCOPE_SITE`
```rust
const RT_SCOPE_SITE: c_uchar = 200u8;
```

### `RT_SCOPE_LINK`
```rust
const RT_SCOPE_LINK: c_uchar = 253u8;
```

### `RT_SCOPE_HOST`
```rust
const RT_SCOPE_HOST: c_uchar = 254u8;
```

### `RT_SCOPE_NOWHERE`
```rust
const RT_SCOPE_NOWHERE: c_uchar = 255u8;
```

### `RT_TABLE_UNSPEC`
```rust
const RT_TABLE_UNSPEC: c_uchar = 0u8;
```

### `RT_TABLE_COMPAT`
```rust
const RT_TABLE_COMPAT: c_uchar = 252u8;
```

### `RT_TABLE_DEFAULT`
```rust
const RT_TABLE_DEFAULT: c_uchar = 253u8;
```

### `RT_TABLE_MAIN`
```rust
const RT_TABLE_MAIN: c_uchar = 254u8;
```

### `RT_TABLE_LOCAL`
```rust
const RT_TABLE_LOCAL: c_uchar = 255u8;
```

### `RTMSG_OVERRUN`
```rust
const RTMSG_OVERRUN: u32 = 4u32;
```

### `RTMSG_NEWDEVICE`
```rust
const RTMSG_NEWDEVICE: u32 = 17u32;
```

### `RTMSG_DELDEVICE`
```rust
const RTMSG_DELDEVICE: u32 = 18u32;
```

### `RTMSG_NEWROUTE`
```rust
const RTMSG_NEWROUTE: u32 = 33u32;
```

### `RTMSG_DELROUTE`
```rust
const RTMSG_DELROUTE: u32 = 34u32;
```

### `RTMSG_NEWRULE`
```rust
const RTMSG_NEWRULE: u32 = 49u32;
```

### `RTMSG_DELRULE`
```rust
const RTMSG_DELRULE: u32 = 50u32;
```

### `RTMSG_CONTROL`
```rust
const RTMSG_CONTROL: u32 = 64u32;
```

### `RTMSG_AR_FAILED`
```rust
const RTMSG_AR_FAILED: u32 = 81u32;
```

### `RTEXT_FILTER_VF`
```rust
const RTEXT_FILTER_VF: c_int = 1i32;
```

### `RTEXT_FILTER_BRVLAN`
```rust
const RTEXT_FILTER_BRVLAN: c_int = 2i32;
```

### `RTEXT_FILTER_BRVLAN_COMPRESSED`
```rust
const RTEXT_FILTER_BRVLAN_COMPRESSED: c_int = 4i32;
```

### `RTEXT_FILTER_SKIP_STATS`
```rust
const RTEXT_FILTER_SKIP_STATS: c_int = 8i32;
```

### `RTEXT_FILTER_MRP`
```rust
const RTEXT_FILTER_MRP: c_int = 16i32;
```

### `RTEXT_FILTER_CFM_CONFIG`
```rust
const RTEXT_FILTER_CFM_CONFIG: c_int = 32i32;
```

### `RTEXT_FILTER_CFM_STATUS`
```rust
const RTEXT_FILTER_CFM_STATUS: c_int = 64i32;
```

### `RTMGRP_LINK`
```rust
const RTMGRP_LINK: c_int = 1i32;
```

### `RTMGRP_NOTIFY`
```rust
const RTMGRP_NOTIFY: c_int = 2i32;
```

### `RTMGRP_NEIGH`
```rust
const RTMGRP_NEIGH: c_int = 4i32;
```

### `RTMGRP_TC`
```rust
const RTMGRP_TC: c_int = 8i32;
```

### `RTMGRP_IPV4_IFADDR`
```rust
const RTMGRP_IPV4_IFADDR: c_int = 16i32;
```

### `RTMGRP_IPV4_MROUTE`
```rust
const RTMGRP_IPV4_MROUTE: c_int = 32i32;
```

### `RTMGRP_IPV4_ROUTE`
```rust
const RTMGRP_IPV4_ROUTE: c_int = 64i32;
```

### `RTMGRP_IPV4_RULE`
```rust
const RTMGRP_IPV4_RULE: c_int = 128i32;
```

### `RTMGRP_IPV6_IFADDR`
```rust
const RTMGRP_IPV6_IFADDR: c_int = 256i32;
```

### `RTMGRP_IPV6_MROUTE`
```rust
const RTMGRP_IPV6_MROUTE: c_int = 512i32;
```

### `RTMGRP_IPV6_ROUTE`
```rust
const RTMGRP_IPV6_ROUTE: c_int = 1_024i32;
```

### `RTMGRP_IPV6_IFINFO`
```rust
const RTMGRP_IPV6_IFINFO: c_int = 2_048i32;
```

### `RTMGRP_DECnet_IFADDR`
```rust
const RTMGRP_DECnet_IFADDR: c_int = 4_096i32;
```

### `RTMGRP_DECnet_ROUTE`
```rust
const RTMGRP_DECnet_ROUTE: c_int = 16_384i32;
```

### `RTMGRP_IPV6_PREFIX`
```rust
const RTMGRP_IPV6_PREFIX: c_int = 131_072i32;
```

### `RTNLGRP_NONE`
```rust
const RTNLGRP_NONE: c_uint = 0u32;
```

### `RTNLGRP_LINK`
```rust
const RTNLGRP_LINK: c_uint = 1u32;
```

### `RTNLGRP_NOTIFY`
```rust
const RTNLGRP_NOTIFY: c_uint = 2u32;
```

### `RTNLGRP_NEIGH`
```rust
const RTNLGRP_NEIGH: c_uint = 3u32;
```

### `RTNLGRP_TC`
```rust
const RTNLGRP_TC: c_uint = 4u32;
```

### `RTNLGRP_IPV4_IFADDR`
```rust
const RTNLGRP_IPV4_IFADDR: c_uint = 5u32;
```

### `RTNLGRP_IPV4_MROUTE`
```rust
const RTNLGRP_IPV4_MROUTE: c_uint = 6u32;
```

### `RTNLGRP_IPV4_ROUTE`
```rust
const RTNLGRP_IPV4_ROUTE: c_uint = 7u32;
```

### `RTNLGRP_IPV4_RULE`
```rust
const RTNLGRP_IPV4_RULE: c_uint = 8u32;
```

### `RTNLGRP_IPV6_IFADDR`
```rust
const RTNLGRP_IPV6_IFADDR: c_uint = 9u32;
```

### `RTNLGRP_IPV6_MROUTE`
```rust
const RTNLGRP_IPV6_MROUTE: c_uint = 10u32;
```

### `RTNLGRP_IPV6_ROUTE`
```rust
const RTNLGRP_IPV6_ROUTE: c_uint = 11u32;
```

### `RTNLGRP_IPV6_IFINFO`
```rust
const RTNLGRP_IPV6_IFINFO: c_uint = 12u32;
```

### `RTNLGRP_DECnet_IFADDR`
```rust
const RTNLGRP_DECnet_IFADDR: c_uint = 13u32;
```

### `RTNLGRP_NOP2`
```rust
const RTNLGRP_NOP2: c_uint = 14u32;
```

### `RTNLGRP_DECnet_ROUTE`
```rust
const RTNLGRP_DECnet_ROUTE: c_uint = 15u32;
```

### `RTNLGRP_DECnet_RULE`
```rust
const RTNLGRP_DECnet_RULE: c_uint = 16u32;
```

### `RTNLGRP_NOP4`
```rust
const RTNLGRP_NOP4: c_uint = 17u32;
```

### `RTNLGRP_IPV6_PREFIX`
```rust
const RTNLGRP_IPV6_PREFIX: c_uint = 18u32;
```

### `RTNLGRP_IPV6_RULE`
```rust
const RTNLGRP_IPV6_RULE: c_uint = 19u32;
```

### `RTNLGRP_ND_USEROPT`
```rust
const RTNLGRP_ND_USEROPT: c_uint = 20u32;
```

### `RTNLGRP_PHONET_IFADDR`
```rust
const RTNLGRP_PHONET_IFADDR: c_uint = 21u32;
```

### `RTNLGRP_PHONET_ROUTE`
```rust
const RTNLGRP_PHONET_ROUTE: c_uint = 22u32;
```

### `RTNLGRP_DCB`
```rust
const RTNLGRP_DCB: c_uint = 23u32;
```

### `RTNLGRP_IPV4_NETCONF`
```rust
const RTNLGRP_IPV4_NETCONF: c_uint = 24u32;
```

### `RTNLGRP_IPV6_NETCONF`
```rust
const RTNLGRP_IPV6_NETCONF: c_uint = 25u32;
```

### `RTNLGRP_MDB`
```rust
const RTNLGRP_MDB: c_uint = 26u32;
```

### `RTNLGRP_MPLS_ROUTE`
```rust
const RTNLGRP_MPLS_ROUTE: c_uint = 27u32;
```

### `RTNLGRP_NSID`
```rust
const RTNLGRP_NSID: c_uint = 28u32;
```

### `RTNLGRP_MPLS_NETCONF`
```rust
const RTNLGRP_MPLS_NETCONF: c_uint = 29u32;
```

### `RTNLGRP_IPV4_MROUTE_R`
```rust
const RTNLGRP_IPV4_MROUTE_R: c_uint = 30u32;
```

### `RTNLGRP_IPV6_MROUTE_R`
```rust
const RTNLGRP_IPV6_MROUTE_R: c_uint = 31u32;
```

### `RTNLGRP_NEXTHOP`
```rust
const RTNLGRP_NEXTHOP: c_uint = 32u32;
```

### `RTNLGRP_BRVLAN`
```rust
const RTNLGRP_BRVLAN: c_uint = 33u32;
```

### `RTNLGRP_MCTP_IFADDR`
```rust
const RTNLGRP_MCTP_IFADDR: c_uint = 34u32;
```

### `RTNLGRP_TUNNEL`
```rust
const RTNLGRP_TUNNEL: c_uint = 35u32;
```

### `RTNLGRP_STATS`
```rust
const RTNLGRP_STATS: c_uint = 36u32;
```

### `PROC_CN_MCAST_LISTEN`
```rust
const PROC_CN_MCAST_LISTEN: proc_cn_mcast_op = 1u32;
```

### `PROC_CN_MCAST_IGNORE`
```rust
const PROC_CN_MCAST_IGNORE: proc_cn_mcast_op = 2u32;
```

### `PROC_EVENT_NONE`
```rust
const PROC_EVENT_NONE: proc_cn_event = 0u32;
```

### `PROC_EVENT_FORK`
```rust
const PROC_EVENT_FORK: proc_cn_event = 1u32;
```

### `PROC_EVENT_EXEC`
```rust
const PROC_EVENT_EXEC: proc_cn_event = 2u32;
```

### `PROC_EVENT_UID`
```rust
const PROC_EVENT_UID: proc_cn_event = 4u32;
```

### `PROC_EVENT_GID`
```rust
const PROC_EVENT_GID: proc_cn_event = 64u32;
```

### `PROC_EVENT_SID`
```rust
const PROC_EVENT_SID: proc_cn_event = 128u32;
```

### `PROC_EVENT_PTRACE`
```rust
const PROC_EVENT_PTRACE: proc_cn_event = 256u32;
```

### `PROC_EVENT_COMM`
```rust
const PROC_EVENT_COMM: proc_cn_event = 512u32;
```

### `PROC_EVENT_NONZERO_EXIT`
```rust
const PROC_EVENT_NONZERO_EXIT: proc_cn_event = 536_870_912u32;
```

### `PROC_EVENT_COREDUMP`
```rust
const PROC_EVENT_COREDUMP: proc_cn_event = 1_073_741_824u32;
```

### `PROC_EVENT_EXIT`
```rust
const PROC_EVENT_EXIT: proc_cn_event = 2_147_483_648u32;
```

### `CN_IDX_PROC`
```rust
const CN_IDX_PROC: c_uint = 1u32;
```

### `CN_VAL_PROC`
```rust
const CN_VAL_PROC: c_uint = 1u32;
```

### `CN_IDX_CIFS`
```rust
const CN_IDX_CIFS: c_uint = 2u32;
```

### `CN_VAL_CIFS`
```rust
const CN_VAL_CIFS: c_uint = 1u32;
```

### `CN_W1_IDX`
```rust
const CN_W1_IDX: c_uint = 3u32;
```

### `CN_W1_VAL`
```rust
const CN_W1_VAL: c_uint = 1u32;
```

### `CN_IDX_V86D`
```rust
const CN_IDX_V86D: c_uint = 4u32;
```

### `CN_VAL_V86D_UVESAFB`
```rust
const CN_VAL_V86D_UVESAFB: c_uint = 1u32;
```

### `CN_IDX_BB`
```rust
const CN_IDX_BB: c_uint = 5u32;
```

### `CN_DST_IDX`
```rust
const CN_DST_IDX: c_uint = 6u32;
```

### `CN_DST_VAL`
```rust
const CN_DST_VAL: c_uint = 1u32;
```

### `CN_IDX_DM`
```rust
const CN_IDX_DM: c_uint = 7u32;
```

### `CN_VAL_DM_USERSPACE_LOG`
```rust
const CN_VAL_DM_USERSPACE_LOG: c_uint = 1u32;
```

### `CN_IDX_DRBD`
```rust
const CN_IDX_DRBD: c_uint = 8u32;
```

### `CN_VAL_DRBD`
```rust
const CN_VAL_DRBD: c_uint = 1u32;
```

### `CN_KVP_IDX`
```rust
const CN_KVP_IDX: c_uint = 9u32;
```

### `CN_KVP_VAL`
```rust
const CN_KVP_VAL: c_uint = 1u32;
```

### `CN_VSS_IDX`
```rust
const CN_VSS_IDX: c_uint = 10u32;
```

### `CN_VSS_VAL`
```rust
const CN_VSS_VAL: c_uint = 1u32;
```

### `MODULE_INIT_IGNORE_MODVERSIONS`
```rust
const MODULE_INIT_IGNORE_MODVERSIONS: c_uint = 1u32;
```

### `MODULE_INIT_IGNORE_VERMAGIC`
```rust
const MODULE_INIT_IGNORE_VERMAGIC: c_uint = 2u32;
```

### `SOF_TIMESTAMPING_TX_HARDWARE`
```rust
const SOF_TIMESTAMPING_TX_HARDWARE: c_uint = 1u32;
```

### `SOF_TIMESTAMPING_TX_SOFTWARE`
```rust
const SOF_TIMESTAMPING_TX_SOFTWARE: c_uint = 2u32;
```

### `SOF_TIMESTAMPING_RX_HARDWARE`
```rust
const SOF_TIMESTAMPING_RX_HARDWARE: c_uint = 4u32;
```

### `SOF_TIMESTAMPING_RX_SOFTWARE`
```rust
const SOF_TIMESTAMPING_RX_SOFTWARE: c_uint = 8u32;
```

### `SOF_TIMESTAMPING_SOFTWARE`
```rust
const SOF_TIMESTAMPING_SOFTWARE: c_uint = 16u32;
```

### `SOF_TIMESTAMPING_SYS_HARDWARE`
```rust
const SOF_TIMESTAMPING_SYS_HARDWARE: c_uint = 32u32;
```

### `SOF_TIMESTAMPING_RAW_HARDWARE`
```rust
const SOF_TIMESTAMPING_RAW_HARDWARE: c_uint = 64u32;
```

### `SOF_TIMESTAMPING_OPT_ID`
```rust
const SOF_TIMESTAMPING_OPT_ID: c_uint = 128u32;
```

### `SOF_TIMESTAMPING_TX_SCHED`
```rust
const SOF_TIMESTAMPING_TX_SCHED: c_uint = 256u32;
```

### `SOF_TIMESTAMPING_TX_ACK`
```rust
const SOF_TIMESTAMPING_TX_ACK: c_uint = 512u32;
```

### `SOF_TIMESTAMPING_OPT_CMSG`
```rust
const SOF_TIMESTAMPING_OPT_CMSG: c_uint = 1_024u32;
```

### `SOF_TIMESTAMPING_OPT_TSONLY`
```rust
const SOF_TIMESTAMPING_OPT_TSONLY: c_uint = 2_048u32;
```

### `SOF_TIMESTAMPING_OPT_STATS`
```rust
const SOF_TIMESTAMPING_OPT_STATS: c_uint = 4_096u32;
```

### `SOF_TIMESTAMPING_OPT_PKTINFO`
```rust
const SOF_TIMESTAMPING_OPT_PKTINFO: c_uint = 8_192u32;
```

### `SOF_TIMESTAMPING_OPT_TX_SWHW`
```rust
const SOF_TIMESTAMPING_OPT_TX_SWHW: c_uint = 16_384u32;
```

### `SOF_TIMESTAMPING_BIND_PHC`
```rust
const SOF_TIMESTAMPING_BIND_PHC: c_uint = 32_768u32;
```

### `SOF_TIMESTAMPING_OPT_ID_TCP`
```rust
const SOF_TIMESTAMPING_OPT_ID_TCP: c_uint = 65_536u32;
```

### `SOF_TIMESTAMPING_OPT_RX_FILTER`
```rust
const SOF_TIMESTAMPING_OPT_RX_FILTER: c_uint = 131_072u32;
```

### `SOF_TXTIME_DEADLINE_MODE`
```rust
const SOF_TXTIME_DEADLINE_MODE: u32 = 1u32;
```

### `SOF_TXTIME_REPORT_ERRORS`
```rust
const SOF_TXTIME_REPORT_ERRORS: u32 = 2u32;
```

### `HWTSTAMP_TX_OFF`
```rust
const HWTSTAMP_TX_OFF: c_uint = 0u32;
```

### `HWTSTAMP_TX_ON`
```rust
const HWTSTAMP_TX_ON: c_uint = 1u32;
```

### `HWTSTAMP_TX_ONESTEP_SYNC`
```rust
const HWTSTAMP_TX_ONESTEP_SYNC: c_uint = 2u32;
```

### `HWTSTAMP_TX_ONESTEP_P2P`
```rust
const HWTSTAMP_TX_ONESTEP_P2P: c_uint = 3u32;
```

### `HWTSTAMP_FILTER_NONE`
```rust
const HWTSTAMP_FILTER_NONE: c_uint = 0u32;
```

### `HWTSTAMP_FILTER_ALL`
```rust
const HWTSTAMP_FILTER_ALL: c_uint = 1u32;
```

### `HWTSTAMP_FILTER_SOME`
```rust
const HWTSTAMP_FILTER_SOME: c_uint = 2u32;
```

### `HWTSTAMP_FILTER_PTP_V1_L4_EVENT`
```rust
const HWTSTAMP_FILTER_PTP_V1_L4_EVENT: c_uint = 3u32;
```

### `HWTSTAMP_FILTER_PTP_V1_L4_SYNC`
```rust
const HWTSTAMP_FILTER_PTP_V1_L4_SYNC: c_uint = 4u32;
```

### `HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ`
```rust
const HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ: c_uint = 5u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L4_EVENT`
```rust
const HWTSTAMP_FILTER_PTP_V2_L4_EVENT: c_uint = 6u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L4_SYNC`
```rust
const HWTSTAMP_FILTER_PTP_V2_L4_SYNC: c_uint = 7u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ`
```rust
const HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ: c_uint = 8u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L2_EVENT`
```rust
const HWTSTAMP_FILTER_PTP_V2_L2_EVENT: c_uint = 9u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L2_SYNC`
```rust
const HWTSTAMP_FILTER_PTP_V2_L2_SYNC: c_uint = 10u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ`
```rust
const HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ: c_uint = 11u32;
```

### `HWTSTAMP_FILTER_PTP_V2_EVENT`
```rust
const HWTSTAMP_FILTER_PTP_V2_EVENT: c_uint = 12u32;
```

### `HWTSTAMP_FILTER_PTP_V2_SYNC`
```rust
const HWTSTAMP_FILTER_PTP_V2_SYNC: c_uint = 13u32;
```

### `HWTSTAMP_FILTER_PTP_V2_DELAY_REQ`
```rust
const HWTSTAMP_FILTER_PTP_V2_DELAY_REQ: c_uint = 14u32;
```

### `HWTSTAMP_FILTER_NTP_ALL`
```rust
const HWTSTAMP_FILTER_NTP_ALL: c_uint = 15u32;
```

### `PTP_MAX_SAMPLES`
```rust
const PTP_MAX_SAMPLES: c_uint = 25u32;
```

### `PTP_CLK_MAGIC`
```rust
const PTP_CLK_MAGIC: u32 = 61u32;
```

### `PTP_CLOCK_GETCAPS`
```rust
const PTP_CLOCK_GETCAPS: c_ulong = 2_152_742_145u64;
```

### `PTP_EXTTS_REQUEST`
```rust
const PTP_EXTTS_REQUEST: c_ulong = 1_074_806_018u64;
```

### `PTP_PEROUT_REQUEST`
```rust
const PTP_PEROUT_REQUEST: c_ulong = 1_077_427_459u64;
```

### `PTP_ENABLE_PPS`
```rust
const PTP_ENABLE_PPS: c_ulong = 1_074_019_588u64;
```

### `PTP_SYS_OFFSET`
```rust
const PTP_SYS_OFFSET: c_ulong = 1_128_283_397u64;
```

### `PTP_PIN_GETFUNC`
```rust
const PTP_PIN_GETFUNC: c_ulong = 3_227_532_550u64;
```

### `PTP_PIN_SETFUNC`
```rust
const PTP_PIN_SETFUNC: c_ulong = 1_080_048_903u64;
```

### `PTP_SYS_OFFSET_PRECISE`
```rust
const PTP_SYS_OFFSET_PRECISE: c_ulong = 3_225_435_400u64;
```

### `PTP_SYS_OFFSET_EXTENDED`
```rust
const PTP_SYS_OFFSET_EXTENDED: c_ulong = 3_300_932_873u64;
```

### `PTP_CLOCK_GETCAPS2`
```rust
const PTP_CLOCK_GETCAPS2: c_ulong = 2_152_742_154u64;
```

### `PTP_EXTTS_REQUEST2`
```rust
const PTP_EXTTS_REQUEST2: c_ulong = 1_074_806_027u64;
```

### `PTP_PEROUT_REQUEST2`
```rust
const PTP_PEROUT_REQUEST2: c_ulong = 1_077_427_468u64;
```

### `PTP_ENABLE_PPS2`
```rust
const PTP_ENABLE_PPS2: c_ulong = 1_074_019_597u64;
```

### `PTP_SYS_OFFSET2`
```rust
const PTP_SYS_OFFSET2: c_ulong = 1_128_283_406u64;
```

### `PTP_PIN_GETFUNC2`
```rust
const PTP_PIN_GETFUNC2: c_ulong = 3_227_532_559u64;
```

### `PTP_PIN_SETFUNC2`
```rust
const PTP_PIN_SETFUNC2: c_ulong = 1_080_048_912u64;
```

### `PTP_SYS_OFFSET_PRECISE2`
```rust
const PTP_SYS_OFFSET_PRECISE2: c_ulong = 3_225_435_409u64;
```

### `PTP_SYS_OFFSET_EXTENDED2`
```rust
const PTP_SYS_OFFSET_EXTENDED2: c_ulong = 3_300_932_882u64;
```

### `PTP_PF_NONE`
```rust
const PTP_PF_NONE: c_uint = 0u32;
```

### `PTP_PF_EXTTS`
```rust
const PTP_PF_EXTTS: c_uint = 1u32;
```

### `PTP_PF_PEROUT`
```rust
const PTP_PF_PEROUT: c_uint = 2u32;
```

### `PTP_PF_PHYSYNC`
```rust
const PTP_PF_PHYSYNC: c_uint = 3u32;
```

### `TLS_TX`
```rust
const TLS_TX: c_int = 1i32;
```

### `TLS_RX`
```rust
const TLS_RX: c_int = 2i32;
```

### `TLS_TX_ZEROCOPY_RO`
```rust
const TLS_TX_ZEROCOPY_RO: c_int = 3i32;
```

### `TLS_RX_EXPECT_NO_PAD`
```rust
const TLS_RX_EXPECT_NO_PAD: c_int = 4i32;
```

### `TLS_1_2_VERSION_MAJOR`
```rust
const TLS_1_2_VERSION_MAJOR: __u8 = 3u8;
```

### `TLS_1_2_VERSION_MINOR`
```rust
const TLS_1_2_VERSION_MINOR: __u8 = 3u8;
```

### `TLS_1_2_VERSION`
```rust
const TLS_1_2_VERSION: __u16 = 771u16;
```

### `TLS_1_3_VERSION_MAJOR`
```rust
const TLS_1_3_VERSION_MAJOR: __u8 = 3u8;
```

### `TLS_1_3_VERSION_MINOR`
```rust
const TLS_1_3_VERSION_MINOR: __u8 = 4u8;
```

### `TLS_1_3_VERSION`
```rust
const TLS_1_3_VERSION: __u16 = 772u16;
```

### `TLS_CIPHER_AES_GCM_128`
```rust
const TLS_CIPHER_AES_GCM_128: __u16 = 51u16;
```

### `TLS_CIPHER_AES_GCM_128_IV_SIZE`
```rust
const TLS_CIPHER_AES_GCM_128_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_AES_GCM_128_KEY_SIZE`
```rust
const TLS_CIPHER_AES_GCM_128_KEY_SIZE: usize = 16usize;
```

### `TLS_CIPHER_AES_GCM_128_SALT_SIZE`
```rust
const TLS_CIPHER_AES_GCM_128_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_AES_GCM_128_TAG_SIZE`
```rust
const TLS_CIPHER_AES_GCM_128_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE`
```rust
const TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_AES_GCM_256`
```rust
const TLS_CIPHER_AES_GCM_256: __u16 = 52u16;
```

### `TLS_CIPHER_AES_GCM_256_IV_SIZE`
```rust
const TLS_CIPHER_AES_GCM_256_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_AES_GCM_256_KEY_SIZE`
```rust
const TLS_CIPHER_AES_GCM_256_KEY_SIZE: usize = 32usize;
```

### `TLS_CIPHER_AES_GCM_256_SALT_SIZE`
```rust
const TLS_CIPHER_AES_GCM_256_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_AES_GCM_256_TAG_SIZE`
```rust
const TLS_CIPHER_AES_GCM_256_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE`
```rust
const TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_AES_CCM_128`
```rust
const TLS_CIPHER_AES_CCM_128: __u16 = 53u16;
```

### `TLS_CIPHER_AES_CCM_128_IV_SIZE`
```rust
const TLS_CIPHER_AES_CCM_128_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_AES_CCM_128_KEY_SIZE`
```rust
const TLS_CIPHER_AES_CCM_128_KEY_SIZE: usize = 16usize;
```

### `TLS_CIPHER_AES_CCM_128_SALT_SIZE`
```rust
const TLS_CIPHER_AES_CCM_128_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_AES_CCM_128_TAG_SIZE`
```rust
const TLS_CIPHER_AES_CCM_128_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE`
```rust
const TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_CHACHA20_POLY1305`
```rust
const TLS_CIPHER_CHACHA20_POLY1305: __u16 = 54u16;
```

### `TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE`
```rust
const TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE: usize = 12usize;
```

### `TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE`
```rust
const TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE: usize = 32usize;
```

### `TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE`
```rust
const TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE: usize = 0usize;
```

### `TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE`
```rust
const TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE`
```rust
const TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_SM4_GCM`
```rust
const TLS_CIPHER_SM4_GCM: __u16 = 55u16;
```

### `TLS_CIPHER_SM4_GCM_IV_SIZE`
```rust
const TLS_CIPHER_SM4_GCM_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_SM4_GCM_KEY_SIZE`
```rust
const TLS_CIPHER_SM4_GCM_KEY_SIZE: usize = 16usize;
```

### `TLS_CIPHER_SM4_GCM_SALT_SIZE`
```rust
const TLS_CIPHER_SM4_GCM_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_SM4_GCM_TAG_SIZE`
```rust
const TLS_CIPHER_SM4_GCM_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE`
```rust
const TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_SM4_CCM`
```rust
const TLS_CIPHER_SM4_CCM: __u16 = 56u16;
```

### `TLS_CIPHER_SM4_CCM_IV_SIZE`
```rust
const TLS_CIPHER_SM4_CCM_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_SM4_CCM_KEY_SIZE`
```rust
const TLS_CIPHER_SM4_CCM_KEY_SIZE: usize = 16usize;
```

### `TLS_CIPHER_SM4_CCM_SALT_SIZE`
```rust
const TLS_CIPHER_SM4_CCM_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_SM4_CCM_TAG_SIZE`
```rust
const TLS_CIPHER_SM4_CCM_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE`
```rust
const TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_ARIA_GCM_128`
```rust
const TLS_CIPHER_ARIA_GCM_128: __u16 = 57u16;
```

### `TLS_CIPHER_ARIA_GCM_128_IV_SIZE`
```rust
const TLS_CIPHER_ARIA_GCM_128_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_ARIA_GCM_128_KEY_SIZE`
```rust
const TLS_CIPHER_ARIA_GCM_128_KEY_SIZE: usize = 16usize;
```

### `TLS_CIPHER_ARIA_GCM_128_SALT_SIZE`
```rust
const TLS_CIPHER_ARIA_GCM_128_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_ARIA_GCM_128_TAG_SIZE`
```rust
const TLS_CIPHER_ARIA_GCM_128_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE`
```rust
const TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_ARIA_GCM_256`
```rust
const TLS_CIPHER_ARIA_GCM_256: __u16 = 58u16;
```

### `TLS_CIPHER_ARIA_GCM_256_IV_SIZE`
```rust
const TLS_CIPHER_ARIA_GCM_256_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_ARIA_GCM_256_KEY_SIZE`
```rust
const TLS_CIPHER_ARIA_GCM_256_KEY_SIZE: usize = 32usize;
```

### `TLS_CIPHER_ARIA_GCM_256_SALT_SIZE`
```rust
const TLS_CIPHER_ARIA_GCM_256_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_ARIA_GCM_256_TAG_SIZE`
```rust
const TLS_CIPHER_ARIA_GCM_256_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE`
```rust
const TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_SET_RECORD_TYPE`
```rust
const TLS_SET_RECORD_TYPE: c_int = 1i32;
```

### `TLS_GET_RECORD_TYPE`
```rust
const TLS_GET_RECORD_TYPE: c_int = 2i32;
```

### `SOL_TLS`
```rust
const SOL_TLS: c_int = 282i32;
```

### `TLS_INFO_UNSPEC`
```rust
const TLS_INFO_UNSPEC: c_int = 0i32;
```

### `TLS_INFO_VERSION`
```rust
const TLS_INFO_VERSION: c_int = 1i32;
```

### `TLS_INFO_CIPHER`
```rust
const TLS_INFO_CIPHER: c_int = 2i32;
```

### `TLS_INFO_TXCONF`
```rust
const TLS_INFO_TXCONF: c_int = 3i32;
```

### `TLS_INFO_RXCONF`
```rust
const TLS_INFO_RXCONF: c_int = 4i32;
```

### `TLS_INFO_ZC_RO_TX`
```rust
const TLS_INFO_ZC_RO_TX: c_int = 5i32;
```

### `TLS_INFO_RX_NO_PAD`
```rust
const TLS_INFO_RX_NO_PAD: c_int = 6i32;
```

### `TLS_CONF_BASE`
```rust
const TLS_CONF_BASE: c_int = 1i32;
```

### `TLS_CONF_SW`
```rust
const TLS_CONF_SW: c_int = 2i32;
```

### `TLS_CONF_HW`
```rust
const TLS_CONF_HW: c_int = 3i32;
```

### `TLS_CONF_HW_RECORD`
```rust
const TLS_CONF_HW_RECORD: c_int = 4i32;
```

### `ALG_SET_KEY`
```rust
const ALG_SET_KEY: c_int = 1i32;
```

### `ALG_SET_IV`
```rust
const ALG_SET_IV: c_int = 2i32;
```

### `ALG_SET_OP`
```rust
const ALG_SET_OP: c_int = 3i32;
```

### `ALG_SET_AEAD_ASSOCLEN`
```rust
const ALG_SET_AEAD_ASSOCLEN: c_int = 4i32;
```

### `ALG_SET_AEAD_AUTHSIZE`
```rust
const ALG_SET_AEAD_AUTHSIZE: c_int = 5i32;
```

### `ALG_SET_DRBG_ENTROPY`
```rust
const ALG_SET_DRBG_ENTROPY: c_int = 6i32;
```

### `ALG_SET_KEY_BY_KEY_SERIAL`
```rust
const ALG_SET_KEY_BY_KEY_SERIAL: c_int = 7i32;
```

### `ALG_OP_DECRYPT`
```rust
const ALG_OP_DECRYPT: c_int = 0i32;
```

### `ALG_OP_ENCRYPT`
```rust
const ALG_OP_ENCRYPT: c_int = 1i32;
```

### `IF_OPER_UNKNOWN`
```rust
const IF_OPER_UNKNOWN: c_int = 0i32;
```

### `IF_OPER_NOTPRESENT`
```rust
const IF_OPER_NOTPRESENT: c_int = 1i32;
```

### `IF_OPER_DOWN`
```rust
const IF_OPER_DOWN: c_int = 2i32;
```

### `IF_OPER_LOWERLAYERDOWN`
```rust
const IF_OPER_LOWERLAYERDOWN: c_int = 3i32;
```

### `IF_OPER_TESTING`
```rust
const IF_OPER_TESTING: c_int = 4i32;
```

### `IF_OPER_DORMANT`
```rust
const IF_OPER_DORMANT: c_int = 5i32;
```

### `IF_OPER_UP`
```rust
const IF_OPER_UP: c_int = 6i32;
```

### `IF_LINK_MODE_DEFAULT`
```rust
const IF_LINK_MODE_DEFAULT: c_int = 0i32;
```

### `IF_LINK_MODE_DORMANT`
```rust
const IF_LINK_MODE_DORMANT: c_int = 1i32;
```

### `IF_LINK_MODE_TESTING`
```rust
const IF_LINK_MODE_TESTING: c_int = 2i32;
```

### `MAP_SHARED_VALIDATE`
```rust
const MAP_SHARED_VALIDATE: c_int = 3i32;
```

### `MAP_DROPPABLE`
```rust
const MAP_DROPPABLE: c_int = 8i32;
```

### `VMADDR_CID_ANY`
```rust
const VMADDR_CID_ANY: c_uint = 4_294_967_295u32;
```

### `VMADDR_CID_HYPERVISOR`
```rust
const VMADDR_CID_HYPERVISOR: c_uint = 0u32;
```

### `VMADDR_CID_RESERVED`
```rust
const VMADDR_CID_RESERVED: c_uint = 1u32;
```

### `VMADDR_CID_LOCAL`
```rust
const VMADDR_CID_LOCAL: c_uint = 1u32;
```

### `VMADDR_CID_HOST`
```rust
const VMADDR_CID_HOST: c_uint = 2u32;
```

### `VMADDR_PORT_ANY`
```rust
const VMADDR_PORT_ANY: c_uint = 4_294_967_295u32;
```

### `IN_ACCESS`
```rust
const IN_ACCESS: u32 = 1u32;
```

### `IN_MODIFY`
```rust
const IN_MODIFY: u32 = 2u32;
```

### `IN_ATTRIB`
```rust
const IN_ATTRIB: u32 = 4u32;
```

### `IN_CLOSE_WRITE`
```rust
const IN_CLOSE_WRITE: u32 = 8u32;
```

### `IN_CLOSE_NOWRITE`
```rust
const IN_CLOSE_NOWRITE: u32 = 16u32;
```

### `IN_CLOSE`
```rust
const IN_CLOSE: u32 = 24u32;
```

### `IN_OPEN`
```rust
const IN_OPEN: u32 = 32u32;
```

### `IN_MOVED_FROM`
```rust
const IN_MOVED_FROM: u32 = 64u32;
```

### `IN_MOVED_TO`
```rust
const IN_MOVED_TO: u32 = 128u32;
```

### `IN_MOVE`
```rust
const IN_MOVE: u32 = 192u32;
```

### `IN_CREATE`
```rust
const IN_CREATE: u32 = 256u32;
```

### `IN_DELETE`
```rust
const IN_DELETE: u32 = 512u32;
```

### `IN_DELETE_SELF`
```rust
const IN_DELETE_SELF: u32 = 1_024u32;
```

### `IN_MOVE_SELF`
```rust
const IN_MOVE_SELF: u32 = 2_048u32;
```

### `IN_UNMOUNT`
```rust
const IN_UNMOUNT: u32 = 8_192u32;
```

### `IN_Q_OVERFLOW`
```rust
const IN_Q_OVERFLOW: u32 = 16_384u32;
```

### `IN_IGNORED`
```rust
const IN_IGNORED: u32 = 32_768u32;
```

### `IN_ONLYDIR`
```rust
const IN_ONLYDIR: u32 = 16_777_216u32;
```

### `IN_DONT_FOLLOW`
```rust
const IN_DONT_FOLLOW: u32 = 33_554_432u32;
```

### `IN_EXCL_UNLINK`
```rust
const IN_EXCL_UNLINK: u32 = 67_108_864u32;
```

### `SECURE_NOROOT`
```rust
const SECURE_NOROOT: c_int = 0i32;
```

### `SECURE_NOROOT_LOCKED`
```rust
const SECURE_NOROOT_LOCKED: c_int = 1i32;
```

### `SECBIT_NOROOT`
```rust
const SECBIT_NOROOT: c_int = 1i32;
```

### `SECBIT_NOROOT_LOCKED`
```rust
const SECBIT_NOROOT_LOCKED: c_int = 2i32;
```

### `SECURE_NO_SETUID_FIXUP`
```rust
const SECURE_NO_SETUID_FIXUP: c_int = 2i32;
```

### `SECURE_NO_SETUID_FIXUP_LOCKED`
```rust
const SECURE_NO_SETUID_FIXUP_LOCKED: c_int = 3i32;
```

### `SECBIT_NO_SETUID_FIXUP`
```rust
const SECBIT_NO_SETUID_FIXUP: c_int = 4i32;
```

### `SECBIT_NO_SETUID_FIXUP_LOCKED`
```rust
const SECBIT_NO_SETUID_FIXUP_LOCKED: c_int = 8i32;
```

### `SECURE_KEEP_CAPS`
```rust
const SECURE_KEEP_CAPS: c_int = 4i32;
```

### `SECURE_KEEP_CAPS_LOCKED`
```rust
const SECURE_KEEP_CAPS_LOCKED: c_int = 5i32;
```

### `SECBIT_KEEP_CAPS`
```rust
const SECBIT_KEEP_CAPS: c_int = 16i32;
```

### `SECBIT_KEEP_CAPS_LOCKED`
```rust
const SECBIT_KEEP_CAPS_LOCKED: c_int = 32i32;
```

### `SECURE_NO_CAP_AMBIENT_RAISE`
```rust
const SECURE_NO_CAP_AMBIENT_RAISE: c_int = 6i32;
```

### `SECURE_NO_CAP_AMBIENT_RAISE_LOCKED`
```rust
const SECURE_NO_CAP_AMBIENT_RAISE_LOCKED: c_int = 7i32;
```

### `SECBIT_NO_CAP_AMBIENT_RAISE`
```rust
const SECBIT_NO_CAP_AMBIENT_RAISE: c_int = 64i32;
```

### `SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED`
```rust
const SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED: c_int = 128i32;
```

### `SECURE_EXEC_RESTRICT_FILE`
```rust
const SECURE_EXEC_RESTRICT_FILE: c_int = 8i32;
```

### `SECURE_EXEC_RESTRICT_FILE_LOCKED`
```rust
const SECURE_EXEC_RESTRICT_FILE_LOCKED: c_int = 9i32;
```

### `SECBIT_EXEC_RESTRICT_FILE`
```rust
const SECBIT_EXEC_RESTRICT_FILE: c_int = 256i32;
```

### `SECBIT_EXEC_RESTRICT_FILE_LOCKED`
```rust
const SECBIT_EXEC_RESTRICT_FILE_LOCKED: c_int = 512i32;
```

### `SECURE_EXEC_DENY_INTERACTIVE`
```rust
const SECURE_EXEC_DENY_INTERACTIVE: c_int = 10i32;
```

### `SECURE_EXEC_DENY_INTERACTIVE_LOCKED`
```rust
const SECURE_EXEC_DENY_INTERACTIVE_LOCKED: c_int = 11i32;
```

### `SECBIT_EXEC_DENY_INTERACTIVE`
```rust
const SECBIT_EXEC_DENY_INTERACTIVE: c_int = 1_024i32;
```

### `SECBIT_EXEC_DENY_INTERACTIVE_LOCKED`
```rust
const SECBIT_EXEC_DENY_INTERACTIVE_LOCKED: c_int = 2_048i32;
```

### `SECUREBITS_DEFAULT`
```rust
const SECUREBITS_DEFAULT: c_int = 0i32;
```

### `SECURE_ALL_BITS`
```rust
const SECURE_ALL_BITS: c_int = 1_365i32;
```

### `SECURE_ALL_LOCKS`
```rust
const SECURE_ALL_LOCKS: c_int = 2_730i32;
```

### `SECURE_ALL_UNPRIVILEGED`
```rust
const SECURE_ALL_UNPRIVILEGED: c_int = 1_280i32;
```

### `IN_MASK_CREATE`
```rust
const IN_MASK_CREATE: u32 = 268_435_456u32;
```

### `IN_MASK_ADD`
```rust
const IN_MASK_ADD: u32 = 536_870_912u32;
```

### `IN_ISDIR`
```rust
const IN_ISDIR: u32 = 1_073_741_824u32;
```

### `IN_ONESHOT`
```rust
const IN_ONESHOT: u32 = 2_147_483_648u32;
```

### `IN_ALL_EVENTS`
```rust
const IN_ALL_EVENTS: u32 = 4_095u32;
```

### `IN_CLOEXEC`
```rust
const IN_CLOEXEC: c_int = 524_288i32;
```

### `IN_NONBLOCK`
```rust
const IN_NONBLOCK: c_int = 2_048i32;
```

### `OPEN_TREE_CLONE`
```rust
const OPEN_TREE_CLONE: c_uint = 1u32;
```

### `OPEN_TREE_CLOEXEC`
```rust
const OPEN_TREE_CLOEXEC: c_uint = 524_288u32;
```

### `NFT_TABLE_MAXNAMELEN`
```rust
const NFT_TABLE_MAXNAMELEN: c_int = 256i32;
```

### `NFT_CHAIN_MAXNAMELEN`
```rust
const NFT_CHAIN_MAXNAMELEN: c_int = 256i32;
```

### `NFT_SET_MAXNAMELEN`
```rust
const NFT_SET_MAXNAMELEN: c_int = 256i32;
```

### `NFT_OBJ_MAXNAMELEN`
```rust
const NFT_OBJ_MAXNAMELEN: c_int = 256i32;
```

### `NFT_USERDATA_MAXLEN`
```rust
const NFT_USERDATA_MAXLEN: c_int = 256i32;
```

### `NFT_REG_VERDICT`
```rust
const NFT_REG_VERDICT: c_int = 0i32;
```

### `NFT_REG_1`
```rust
const NFT_REG_1: c_int = 1i32;
```

### `NFT_REG_2`
```rust
const NFT_REG_2: c_int = 2i32;
```

### `NFT_REG_3`
```rust
const NFT_REG_3: c_int = 3i32;
```

### `NFT_REG_4`
```rust
const NFT_REG_4: c_int = 4i32;
```

### `__NFT_REG_MAX`
```rust
const __NFT_REG_MAX: c_int = 5i32;
```

### `NFT_REG32_00`
```rust
const NFT_REG32_00: c_int = 8i32;
```

### `NFT_REG32_01`
```rust
const NFT_REG32_01: c_int = 9i32;
```

### `NFT_REG32_02`
```rust
const NFT_REG32_02: c_int = 10i32;
```

### `NFT_REG32_03`
```rust
const NFT_REG32_03: c_int = 11i32;
```

### `NFT_REG32_04`
```rust
const NFT_REG32_04: c_int = 12i32;
```

### `NFT_REG32_05`
```rust
const NFT_REG32_05: c_int = 13i32;
```

### `NFT_REG32_06`
```rust
const NFT_REG32_06: c_int = 14i32;
```

### `NFT_REG32_07`
```rust
const NFT_REG32_07: c_int = 15i32;
```

### `NFT_REG32_08`
```rust
const NFT_REG32_08: c_int = 16i32;
```

### `NFT_REG32_09`
```rust
const NFT_REG32_09: c_int = 17i32;
```

### `NFT_REG32_10`
```rust
const NFT_REG32_10: c_int = 18i32;
```

### `NFT_REG32_11`
```rust
const NFT_REG32_11: c_int = 19i32;
```

### `NFT_REG32_12`
```rust
const NFT_REG32_12: c_int = 20i32;
```

### `NFT_REG32_13`
```rust
const NFT_REG32_13: c_int = 21i32;
```

### `NFT_REG32_14`
```rust
const NFT_REG32_14: c_int = 22i32;
```

### `NFT_REG32_15`
```rust
const NFT_REG32_15: c_int = 23i32;
```

### `NFT_REG_SIZE`
```rust
const NFT_REG_SIZE: c_int = 16i32;
```

### `NFT_REG32_SIZE`
```rust
const NFT_REG32_SIZE: c_int = 4i32;
```

### `NFT_CONTINUE`
```rust
const NFT_CONTINUE: c_int = -1i32;
```

### `NFT_BREAK`
```rust
const NFT_BREAK: c_int = -2i32;
```

### `NFT_JUMP`
```rust
const NFT_JUMP: c_int = -3i32;
```

### `NFT_GOTO`
```rust
const NFT_GOTO: c_int = -4i32;
```

### `NFT_RETURN`
```rust
const NFT_RETURN: c_int = -5i32;
```

### `NFT_MSG_NEWTABLE`
```rust
const NFT_MSG_NEWTABLE: c_int = 0i32;
```

### `NFT_MSG_GETTABLE`
```rust
const NFT_MSG_GETTABLE: c_int = 1i32;
```

### `NFT_MSG_DELTABLE`
```rust
const NFT_MSG_DELTABLE: c_int = 2i32;
```

### `NFT_MSG_NEWCHAIN`
```rust
const NFT_MSG_NEWCHAIN: c_int = 3i32;
```

### `NFT_MSG_GETCHAIN`
```rust
const NFT_MSG_GETCHAIN: c_int = 4i32;
```

### `NFT_MSG_DELCHAIN`
```rust
const NFT_MSG_DELCHAIN: c_int = 5i32;
```

### `NFT_MSG_NEWRULE`
```rust
const NFT_MSG_NEWRULE: c_int = 6i32;
```

### `NFT_MSG_GETRULE`
```rust
const NFT_MSG_GETRULE: c_int = 7i32;
```

### `NFT_MSG_DELRULE`
```rust
const NFT_MSG_DELRULE: c_int = 8i32;
```

### `NFT_MSG_NEWSET`
```rust
const NFT_MSG_NEWSET: c_int = 9i32;
```

### `NFT_MSG_GETSET`
```rust
const NFT_MSG_GETSET: c_int = 10i32;
```

### `NFT_MSG_DELSET`
```rust
const NFT_MSG_DELSET: c_int = 11i32;
```

### `NFT_MSG_NEWSETELEM`
```rust
const NFT_MSG_NEWSETELEM: c_int = 12i32;
```

### `NFT_MSG_GETSETELEM`
```rust
const NFT_MSG_GETSETELEM: c_int = 13i32;
```

### `NFT_MSG_DELSETELEM`
```rust
const NFT_MSG_DELSETELEM: c_int = 14i32;
```

### `NFT_MSG_NEWGEN`
```rust
const NFT_MSG_NEWGEN: c_int = 15i32;
```

### `NFT_MSG_GETGEN`
```rust
const NFT_MSG_GETGEN: c_int = 16i32;
```

### `NFT_MSG_TRACE`
```rust
const NFT_MSG_TRACE: c_int = 17i32;
```

### `NFT_MSG_NEWOBJ`
```rust
const NFT_MSG_NEWOBJ: c_int = 18i32;
```

### `NFT_MSG_GETOBJ`
```rust
const NFT_MSG_GETOBJ: c_int = 19i32;
```

### `NFT_MSG_DELOBJ`
```rust
const NFT_MSG_DELOBJ: c_int = 20i32;
```

### `NFT_MSG_GETOBJ_RESET`
```rust
const NFT_MSG_GETOBJ_RESET: c_int = 21i32;
```

### `NFT_MSG_MAX`
```rust
const NFT_MSG_MAX: c_int = 34i32;
```

### `NFT_SET_ANONYMOUS`
```rust
const NFT_SET_ANONYMOUS: c_int = 1i32;
```

### `NFT_SET_CONSTANT`
```rust
const NFT_SET_CONSTANT: c_int = 2i32;
```

### `NFT_SET_INTERVAL`
```rust
const NFT_SET_INTERVAL: c_int = 4i32;
```

### `NFT_SET_MAP`
```rust
const NFT_SET_MAP: c_int = 8i32;
```

### `NFT_SET_TIMEOUT`
```rust
const NFT_SET_TIMEOUT: c_int = 16i32;
```

### `NFT_SET_EVAL`
```rust
const NFT_SET_EVAL: c_int = 32i32;
```

### `NFT_SET_POL_PERFORMANCE`
```rust
const NFT_SET_POL_PERFORMANCE: c_int = 0i32;
```

### `NFT_SET_POL_MEMORY`
```rust
const NFT_SET_POL_MEMORY: c_int = 1i32;
```

### `NFT_SET_ELEM_INTERVAL_END`
```rust
const NFT_SET_ELEM_INTERVAL_END: c_int = 1i32;
```

### `NFT_DATA_VALUE`
```rust
const NFT_DATA_VALUE: c_uint = 0u32;
```

### `NFT_DATA_VERDICT`
```rust
const NFT_DATA_VERDICT: c_uint = 4_294_967_040u32;
```

### `NFT_DATA_RESERVED_MASK`
```rust
const NFT_DATA_RESERVED_MASK: c_uint = 4_294_967_040u32;
```

### `NFT_DATA_VALUE_MAXLEN`
```rust
const NFT_DATA_VALUE_MAXLEN: c_int = 64i32;
```

### `NFT_BYTEORDER_NTOH`
```rust
const NFT_BYTEORDER_NTOH: c_int = 0i32;
```

### `NFT_BYTEORDER_HTON`
```rust
const NFT_BYTEORDER_HTON: c_int = 1i32;
```

### `NFT_CMP_EQ`
```rust
const NFT_CMP_EQ: c_int = 0i32;
```

### `NFT_CMP_NEQ`
```rust
const NFT_CMP_NEQ: c_int = 1i32;
```

### `NFT_CMP_LT`
```rust
const NFT_CMP_LT: c_int = 2i32;
```

### `NFT_CMP_LTE`
```rust
const NFT_CMP_LTE: c_int = 3i32;
```

### `NFT_CMP_GT`
```rust
const NFT_CMP_GT: c_int = 4i32;
```

### `NFT_CMP_GTE`
```rust
const NFT_CMP_GTE: c_int = 5i32;
```

### `NFT_RANGE_EQ`
```rust
const NFT_RANGE_EQ: c_int = 0i32;
```

### `NFT_RANGE_NEQ`
```rust
const NFT_RANGE_NEQ: c_int = 1i32;
```

### `NFT_LOOKUP_F_INV`
```rust
const NFT_LOOKUP_F_INV: c_int = 1i32;
```

### `NFT_DYNSET_OP_ADD`
```rust
const NFT_DYNSET_OP_ADD: c_int = 0i32;
```

### `NFT_DYNSET_OP_UPDATE`
```rust
const NFT_DYNSET_OP_UPDATE: c_int = 1i32;
```

### `NFT_DYNSET_F_INV`
```rust
const NFT_DYNSET_F_INV: c_int = 1i32;
```

### `NFT_PAYLOAD_LL_HEADER`
```rust
const NFT_PAYLOAD_LL_HEADER: c_int = 0i32;
```

### `NFT_PAYLOAD_NETWORK_HEADER`
```rust
const NFT_PAYLOAD_NETWORK_HEADER: c_int = 1i32;
```

### `NFT_PAYLOAD_TRANSPORT_HEADER`
```rust
const NFT_PAYLOAD_TRANSPORT_HEADER: c_int = 2i32;
```

### `NFT_PAYLOAD_CSUM_NONE`
```rust
const NFT_PAYLOAD_CSUM_NONE: c_int = 0i32;
```

### `NFT_PAYLOAD_CSUM_INET`
```rust
const NFT_PAYLOAD_CSUM_INET: c_int = 1i32;
```

### `NFT_META_LEN`
```rust
const NFT_META_LEN: c_int = 0i32;
```

### `NFT_META_PROTOCOL`
```rust
const NFT_META_PROTOCOL: c_int = 1i32;
```

### `NFT_META_PRIORITY`
```rust
const NFT_META_PRIORITY: c_int = 2i32;
```

### `NFT_META_MARK`
```rust
const NFT_META_MARK: c_int = 3i32;
```

### `NFT_META_IIF`
```rust
const NFT_META_IIF: c_int = 4i32;
```

### `NFT_META_OIF`
```rust
const NFT_META_OIF: c_int = 5i32;
```

### `NFT_META_IIFNAME`
```rust
const NFT_META_IIFNAME: c_int = 6i32;
```

### `NFT_META_OIFNAME`
```rust
const NFT_META_OIFNAME: c_int = 7i32;
```

### `NFT_META_IIFTYPE`
```rust
const NFT_META_IIFTYPE: c_int = 8i32;
```

### `NFT_META_OIFTYPE`
```rust
const NFT_META_OIFTYPE: c_int = 9i32;
```

### `NFT_META_SKUID`
```rust
const NFT_META_SKUID: c_int = 10i32;
```

### `NFT_META_SKGID`
```rust
const NFT_META_SKGID: c_int = 11i32;
```

### `NFT_META_NFTRACE`
```rust
const NFT_META_NFTRACE: c_int = 12i32;
```

### `NFT_META_RTCLASSID`
```rust
const NFT_META_RTCLASSID: c_int = 13i32;
```

### `NFT_META_SECMARK`
```rust
const NFT_META_SECMARK: c_int = 14i32;
```

### `NFT_META_NFPROTO`
```rust
const NFT_META_NFPROTO: c_int = 15i32;
```

### `NFT_META_L4PROTO`
```rust
const NFT_META_L4PROTO: c_int = 16i32;
```

### `NFT_META_BRI_IIFNAME`
```rust
const NFT_META_BRI_IIFNAME: c_int = 17i32;
```

### `NFT_META_BRI_OIFNAME`
```rust
const NFT_META_BRI_OIFNAME: c_int = 18i32;
```

### `NFT_META_PKTTYPE`
```rust
const NFT_META_PKTTYPE: c_int = 19i32;
```

### `NFT_META_CPU`
```rust
const NFT_META_CPU: c_int = 20i32;
```

### `NFT_META_IIFGROUP`
```rust
const NFT_META_IIFGROUP: c_int = 21i32;
```

### `NFT_META_OIFGROUP`
```rust
const NFT_META_OIFGROUP: c_int = 22i32;
```

### `NFT_META_CGROUP`
```rust
const NFT_META_CGROUP: c_int = 23i32;
```

### `NFT_META_PRANDOM`
```rust
const NFT_META_PRANDOM: c_int = 24i32;
```

### `NFT_CT_STATE`
```rust
const NFT_CT_STATE: c_int = 0i32;
```

### `NFT_CT_DIRECTION`
```rust
const NFT_CT_DIRECTION: c_int = 1i32;
```

### `NFT_CT_STATUS`
```rust
const NFT_CT_STATUS: c_int = 2i32;
```

### `NFT_CT_MARK`
```rust
const NFT_CT_MARK: c_int = 3i32;
```

### `NFT_CT_SECMARK`
```rust
const NFT_CT_SECMARK: c_int = 4i32;
```

### `NFT_CT_EXPIRATION`
```rust
const NFT_CT_EXPIRATION: c_int = 5i32;
```

### `NFT_CT_HELPER`
```rust
const NFT_CT_HELPER: c_int = 6i32;
```

### `NFT_CT_L3PROTOCOL`
```rust
const NFT_CT_L3PROTOCOL: c_int = 7i32;
```

### `NFT_CT_SRC`
```rust
const NFT_CT_SRC: c_int = 8i32;
```

### `NFT_CT_DST`
```rust
const NFT_CT_DST: c_int = 9i32;
```

### `NFT_CT_PROTOCOL`
```rust
const NFT_CT_PROTOCOL: c_int = 10i32;
```

### `NFT_CT_PROTO_SRC`
```rust
const NFT_CT_PROTO_SRC: c_int = 11i32;
```

### `NFT_CT_PROTO_DST`
```rust
const NFT_CT_PROTO_DST: c_int = 12i32;
```

### `NFT_CT_LABELS`
```rust
const NFT_CT_LABELS: c_int = 13i32;
```

### `NFT_CT_PKTS`
```rust
const NFT_CT_PKTS: c_int = 14i32;
```

### `NFT_CT_BYTES`
```rust
const NFT_CT_BYTES: c_int = 15i32;
```

### `NFT_CT_AVGPKT`
```rust
const NFT_CT_AVGPKT: c_int = 16i32;
```

### `NFT_CT_ZONE`
```rust
const NFT_CT_ZONE: c_int = 17i32;
```

### `NFT_CT_EVENTMASK`
```rust
const NFT_CT_EVENTMASK: c_int = 18i32;
```

### `NFT_CT_SRC_IP`
```rust
const NFT_CT_SRC_IP: c_int = 19i32;
```

### `NFT_CT_DST_IP`
```rust
const NFT_CT_DST_IP: c_int = 20i32;
```

### `NFT_CT_SRC_IP6`
```rust
const NFT_CT_SRC_IP6: c_int = 21i32;
```

### `NFT_CT_DST_IP6`
```rust
const NFT_CT_DST_IP6: c_int = 22i32;
```

### `NFT_LIMIT_PKTS`
```rust
const NFT_LIMIT_PKTS: c_int = 0i32;
```

### `NFT_LIMIT_PKT_BYTES`
```rust
const NFT_LIMIT_PKT_BYTES: c_int = 1i32;
```

### `NFT_LIMIT_F_INV`
```rust
const NFT_LIMIT_F_INV: c_int = 1i32;
```

### `NFT_QUEUE_FLAG_BYPASS`
```rust
const NFT_QUEUE_FLAG_BYPASS: c_int = 1i32;
```

### `NFT_QUEUE_FLAG_CPU_FANOUT`
```rust
const NFT_QUEUE_FLAG_CPU_FANOUT: c_int = 2i32;
```

### `NFT_QUEUE_FLAG_MASK`
```rust
const NFT_QUEUE_FLAG_MASK: c_int = 3i32;
```

### `NFT_QUOTA_F_INV`
```rust
const NFT_QUOTA_F_INV: c_int = 1i32;
```

### `NFT_REJECT_ICMP_UNREACH`
```rust
const NFT_REJECT_ICMP_UNREACH: c_int = 0i32;
```

### `NFT_REJECT_TCP_RST`
```rust
const NFT_REJECT_TCP_RST: c_int = 1i32;
```

### `NFT_REJECT_ICMPX_UNREACH`
```rust
const NFT_REJECT_ICMPX_UNREACH: c_int = 2i32;
```

### `NFT_REJECT_ICMPX_NO_ROUTE`
```rust
const NFT_REJECT_ICMPX_NO_ROUTE: c_int = 0i32;
```

### `NFT_REJECT_ICMPX_PORT_UNREACH`
```rust
const NFT_REJECT_ICMPX_PORT_UNREACH: c_int = 1i32;
```

### `NFT_REJECT_ICMPX_HOST_UNREACH`
```rust
const NFT_REJECT_ICMPX_HOST_UNREACH: c_int = 2i32;
```

### `NFT_REJECT_ICMPX_ADMIN_PROHIBITED`
```rust
const NFT_REJECT_ICMPX_ADMIN_PROHIBITED: c_int = 3i32;
```

### `NFT_NAT_SNAT`
```rust
const NFT_NAT_SNAT: c_int = 0i32;
```

### `NFT_NAT_DNAT`
```rust
const NFT_NAT_DNAT: c_int = 1i32;
```

### `NFT_TRACETYPE_UNSPEC`
```rust
const NFT_TRACETYPE_UNSPEC: c_int = 0i32;
```

### `NFT_TRACETYPE_POLICY`
```rust
const NFT_TRACETYPE_POLICY: c_int = 1i32;
```

### `NFT_TRACETYPE_RETURN`
```rust
const NFT_TRACETYPE_RETURN: c_int = 2i32;
```

### `NFT_TRACETYPE_RULE`
```rust
const NFT_TRACETYPE_RULE: c_int = 3i32;
```

### `NFT_NG_INCREMENTAL`
```rust
const NFT_NG_INCREMENTAL: c_int = 0i32;
```

### `NFT_NG_RANDOM`
```rust
const NFT_NG_RANDOM: c_int = 1i32;
```

### `FF_MAX`
```rust
const FF_MAX: __u16 = 127u16;
```

### `FF_CNT`
```rust
const FF_CNT: usize = 128usize;
```

### `INPUT_PROP_POINTER`
```rust
const INPUT_PROP_POINTER: __u16 = 0u16;
```

### `INPUT_PROP_DIRECT`
```rust
const INPUT_PROP_DIRECT: __u16 = 1u16;
```

### `INPUT_PROP_BUTTONPAD`
```rust
const INPUT_PROP_BUTTONPAD: __u16 = 2u16;
```

### `INPUT_PROP_SEMI_MT`
```rust
const INPUT_PROP_SEMI_MT: __u16 = 3u16;
```

### `INPUT_PROP_TOPBUTTONPAD`
```rust
const INPUT_PROP_TOPBUTTONPAD: __u16 = 4u16;
```

### `INPUT_PROP_POINTING_STICK`
```rust
const INPUT_PROP_POINTING_STICK: __u16 = 5u16;
```

### `INPUT_PROP_ACCELEROMETER`
```rust
const INPUT_PROP_ACCELEROMETER: __u16 = 6u16;
```

### `INPUT_PROP_MAX`
```rust
const INPUT_PROP_MAX: __u16 = 31u16;
```

### `INPUT_PROP_CNT`
```rust
const INPUT_PROP_CNT: usize = 32usize;
```

### `EV_MAX`
```rust
const EV_MAX: __u16 = 31u16;
```

### `EV_CNT`
```rust
const EV_CNT: usize = 32usize;
```

### `SYN_MAX`
```rust
const SYN_MAX: __u16 = 15u16;
```

### `SYN_CNT`
```rust
const SYN_CNT: usize = 16usize;
```

### `KEY_MAX`
```rust
const KEY_MAX: __u16 = 767u16;
```

### `KEY_CNT`
```rust
const KEY_CNT: usize = 768usize;
```

### `REL_MAX`
```rust
const REL_MAX: __u16 = 15u16;
```

### `REL_CNT`
```rust
const REL_CNT: usize = 16usize;
```

### `ABS_MAX`
```rust
const ABS_MAX: __u16 = 63u16;
```

### `ABS_CNT`
```rust
const ABS_CNT: usize = 64usize;
```

### `SW_MAX`
```rust
const SW_MAX: __u16 = 16u16;
```

### `SW_CNT`
```rust
const SW_CNT: usize = 17usize;
```

### `MSC_MAX`
```rust
const MSC_MAX: __u16 = 7u16;
```

### `MSC_CNT`
```rust
const MSC_CNT: usize = 8usize;
```

### `LED_MAX`
```rust
const LED_MAX: __u16 = 15u16;
```

### `LED_CNT`
```rust
const LED_CNT: usize = 16usize;
```

### `REP_MAX`
```rust
const REP_MAX: __u16 = 1u16;
```

### `REP_CNT`
```rust
const REP_CNT: usize = 2usize;
```

### `SND_MAX`
```rust
const SND_MAX: __u16 = 7u16;
```

### `SND_CNT`
```rust
const SND_CNT: usize = 8usize;
```

### `UINPUT_VERSION`
```rust
const UINPUT_VERSION: c_uint = 5u32;
```

### `UINPUT_MAX_NAME_SIZE`
```rust
const UINPUT_MAX_NAME_SIZE: usize = 80usize;
```

### `FAN_ACCESS`
```rust
const FAN_ACCESS: u64 = 1u64;
```

### `FAN_MODIFY`
```rust
const FAN_MODIFY: u64 = 2u64;
```

### `FAN_ATTRIB`
```rust
const FAN_ATTRIB: u64 = 4u64;
```

### `FAN_CLOSE_WRITE`
```rust
const FAN_CLOSE_WRITE: u64 = 8u64;
```

### `FAN_CLOSE_NOWRITE`
```rust
const FAN_CLOSE_NOWRITE: u64 = 16u64;
```

### `FAN_OPEN`
```rust
const FAN_OPEN: u64 = 32u64;
```

### `FAN_MOVED_FROM`
```rust
const FAN_MOVED_FROM: u64 = 64u64;
```

### `FAN_MOVED_TO`
```rust
const FAN_MOVED_TO: u64 = 128u64;
```

### `FAN_CREATE`
```rust
const FAN_CREATE: u64 = 256u64;
```

### `FAN_DELETE`
```rust
const FAN_DELETE: u64 = 512u64;
```

### `FAN_DELETE_SELF`
```rust
const FAN_DELETE_SELF: u64 = 1_024u64;
```

### `FAN_MOVE_SELF`
```rust
const FAN_MOVE_SELF: u64 = 2_048u64;
```

### `FAN_OPEN_EXEC`
```rust
const FAN_OPEN_EXEC: u64 = 4_096u64;
```

### `FAN_Q_OVERFLOW`
```rust
const FAN_Q_OVERFLOW: u64 = 16_384u64;
```

### `FAN_FS_ERROR`
```rust
const FAN_FS_ERROR: u64 = 32_768u64;
```

### `FAN_OPEN_PERM`
```rust
const FAN_OPEN_PERM: u64 = 65_536u64;
```

### `FAN_ACCESS_PERM`
```rust
const FAN_ACCESS_PERM: u64 = 131_072u64;
```

### `FAN_OPEN_EXEC_PERM`
```rust
const FAN_OPEN_EXEC_PERM: u64 = 262_144u64;
```

### `FAN_EVENT_ON_CHILD`
```rust
const FAN_EVENT_ON_CHILD: u64 = 134_217_728u64;
```

### `FAN_RENAME`
```rust
const FAN_RENAME: u64 = 268_435_456u64;
```

### `FAN_ONDIR`
```rust
const FAN_ONDIR: u64 = 1_073_741_824u64;
```

### `FAN_CLOSE`
```rust
const FAN_CLOSE: u64 = 24u64;
```

### `FAN_MOVE`
```rust
const FAN_MOVE: u64 = 192u64;
```

### `FAN_CLOEXEC`
```rust
const FAN_CLOEXEC: c_uint = 1u32;
```

### `FAN_NONBLOCK`
```rust
const FAN_NONBLOCK: c_uint = 2u32;
```

### `FAN_CLASS_NOTIF`
```rust
const FAN_CLASS_NOTIF: c_uint = 0u32;
```

### `FAN_CLASS_CONTENT`
```rust
const FAN_CLASS_CONTENT: c_uint = 4u32;
```

### `FAN_CLASS_PRE_CONTENT`
```rust
const FAN_CLASS_PRE_CONTENT: c_uint = 8u32;
```

### `FAN_UNLIMITED_QUEUE`
```rust
const FAN_UNLIMITED_QUEUE: c_uint = 16u32;
```

### `FAN_UNLIMITED_MARKS`
```rust
const FAN_UNLIMITED_MARKS: c_uint = 32u32;
```

### `FAN_ENABLE_AUDIT`
```rust
const FAN_ENABLE_AUDIT: c_uint = 64u32;
```

### `FAN_REPORT_PIDFD`
```rust
const FAN_REPORT_PIDFD: c_uint = 128u32;
```

### `FAN_REPORT_TID`
```rust
const FAN_REPORT_TID: c_uint = 256u32;
```

### `FAN_REPORT_FID`
```rust
const FAN_REPORT_FID: c_uint = 512u32;
```

### `FAN_REPORT_DIR_FID`
```rust
const FAN_REPORT_DIR_FID: c_uint = 1_024u32;
```

### `FAN_REPORT_NAME`
```rust
const FAN_REPORT_NAME: c_uint = 2_048u32;
```

### `FAN_REPORT_TARGET_FID`
```rust
const FAN_REPORT_TARGET_FID: c_uint = 4_096u32;
```

### `FAN_REPORT_DFID_NAME`
```rust
const FAN_REPORT_DFID_NAME: c_uint = 3_072u32;
```

### `FAN_REPORT_DFID_NAME_TARGET`
```rust
const FAN_REPORT_DFID_NAME_TARGET: c_uint = 7_680u32;
```

### `FAN_MARK_ADD`
```rust
const FAN_MARK_ADD: c_uint = 1u32;
```

### `FAN_MARK_REMOVE`
```rust
const FAN_MARK_REMOVE: c_uint = 2u32;
```

### `FAN_MARK_DONT_FOLLOW`
```rust
const FAN_MARK_DONT_FOLLOW: c_uint = 4u32;
```

### `FAN_MARK_ONLYDIR`
```rust
const FAN_MARK_ONLYDIR: c_uint = 8u32;
```

### `FAN_MARK_IGNORED_MASK`
```rust
const FAN_MARK_IGNORED_MASK: c_uint = 32u32;
```

### `FAN_MARK_IGNORED_SURV_MODIFY`
```rust
const FAN_MARK_IGNORED_SURV_MODIFY: c_uint = 64u32;
```

### `FAN_MARK_FLUSH`
```rust
const FAN_MARK_FLUSH: c_uint = 128u32;
```

### `FAN_MARK_EVICTABLE`
```rust
const FAN_MARK_EVICTABLE: c_uint = 512u32;
```

### `FAN_MARK_IGNORE`
```rust
const FAN_MARK_IGNORE: c_uint = 1_024u32;
```

### `FAN_MARK_INODE`
```rust
const FAN_MARK_INODE: c_uint = 0u32;
```

### `FAN_MARK_MOUNT`
```rust
const FAN_MARK_MOUNT: c_uint = 16u32;
```

### `FAN_MARK_FILESYSTEM`
```rust
const FAN_MARK_FILESYSTEM: c_uint = 256u32;
```

### `FAN_MARK_IGNORE_SURV`
```rust
const FAN_MARK_IGNORE_SURV: c_uint = 1_088u32;
```

### `FANOTIFY_METADATA_VERSION`
```rust
const FANOTIFY_METADATA_VERSION: u8 = 3u8;
```

### `FAN_EVENT_INFO_TYPE_FID`
```rust
const FAN_EVENT_INFO_TYPE_FID: u8 = 1u8;
```

### `FAN_EVENT_INFO_TYPE_DFID_NAME`
```rust
const FAN_EVENT_INFO_TYPE_DFID_NAME: u8 = 2u8;
```

### `FAN_EVENT_INFO_TYPE_DFID`
```rust
const FAN_EVENT_INFO_TYPE_DFID: u8 = 3u8;
```

### `FAN_EVENT_INFO_TYPE_PIDFD`
```rust
const FAN_EVENT_INFO_TYPE_PIDFD: u8 = 4u8;
```

### `FAN_EVENT_INFO_TYPE_ERROR`
```rust
const FAN_EVENT_INFO_TYPE_ERROR: u8 = 5u8;
```

### `FAN_EVENT_INFO_TYPE_OLD_DFID_NAME`
```rust
const FAN_EVENT_INFO_TYPE_OLD_DFID_NAME: u8 = 10u8;
```

### `FAN_EVENT_INFO_TYPE_NEW_DFID_NAME`
```rust
const FAN_EVENT_INFO_TYPE_NEW_DFID_NAME: u8 = 12u8;
```

### `FAN_RESPONSE_INFO_NONE`
```rust
const FAN_RESPONSE_INFO_NONE: u8 = 0u8;
```

### `FAN_RESPONSE_INFO_AUDIT_RULE`
```rust
const FAN_RESPONSE_INFO_AUDIT_RULE: u8 = 1u8;
```

### `FAN_ALLOW`
```rust
const FAN_ALLOW: u32 = 1u32;
```

### `FAN_DENY`
```rust
const FAN_DENY: u32 = 2u32;
```

### `FAN_AUDIT`
```rust
const FAN_AUDIT: u32 = 16u32;
```

### `FAN_INFO`
```rust
const FAN_INFO: u32 = 32u32;
```

### `FAN_NOFD`
```rust
const FAN_NOFD: c_int = -1i32;
```

### `FAN_NOPIDFD`
```rust
const FAN_NOPIDFD: c_int = -1i32;
```

### `FAN_EPIDFD`
```rust
const FAN_EPIDFD: c_int = -2i32;
```

### `FUTEX_WAIT`
```rust
const FUTEX_WAIT: c_int = 0i32;
```

### `FUTEX_WAKE`
```rust
const FUTEX_WAKE: c_int = 1i32;
```

### `FUTEX_FD`
```rust
const FUTEX_FD: c_int = 2i32;
```

### `FUTEX_REQUEUE`
```rust
const FUTEX_REQUEUE: c_int = 3i32;
```

### `FUTEX_CMP_REQUEUE`
```rust
const FUTEX_CMP_REQUEUE: c_int = 4i32;
```

### `FUTEX_WAKE_OP`
```rust
const FUTEX_WAKE_OP: c_int = 5i32;
```

### `FUTEX_LOCK_PI`
```rust
const FUTEX_LOCK_PI: c_int = 6i32;
```

### `FUTEX_UNLOCK_PI`
```rust
const FUTEX_UNLOCK_PI: c_int = 7i32;
```

### `FUTEX_TRYLOCK_PI`
```rust
const FUTEX_TRYLOCK_PI: c_int = 8i32;
```

### `FUTEX_WAIT_BITSET`
```rust
const FUTEX_WAIT_BITSET: c_int = 9i32;
```

### `FUTEX_WAKE_BITSET`
```rust
const FUTEX_WAKE_BITSET: c_int = 10i32;
```

### `FUTEX_WAIT_REQUEUE_PI`
```rust
const FUTEX_WAIT_REQUEUE_PI: c_int = 11i32;
```

### `FUTEX_CMP_REQUEUE_PI`
```rust
const FUTEX_CMP_REQUEUE_PI: c_int = 12i32;
```

### `FUTEX_LOCK_PI2`
```rust
const FUTEX_LOCK_PI2: c_int = 13i32;
```

### `FUTEX_PRIVATE_FLAG`
```rust
const FUTEX_PRIVATE_FLAG: c_int = 128i32;
```

### `FUTEX_CLOCK_REALTIME`
```rust
const FUTEX_CLOCK_REALTIME: c_int = 256i32;
```

### `FUTEX_CMD_MASK`
```rust
const FUTEX_CMD_MASK: c_int = -385i32;
```

### `FUTEX_WAITERS`
```rust
const FUTEX_WAITERS: u32 = 2_147_483_648u32;
```

### `FUTEX_OWNER_DIED`
```rust
const FUTEX_OWNER_DIED: u32 = 1_073_741_824u32;
```

### `FUTEX_TID_MASK`
```rust
const FUTEX_TID_MASK: u32 = 1_073_741_823u32;
```

### `FUTEX_BITSET_MATCH_ANY`
```rust
const FUTEX_BITSET_MATCH_ANY: c_int = -1i32;
```

### `FUTEX_OP_SET`
```rust
const FUTEX_OP_SET: c_int = 0i32;
```

### `FUTEX_OP_ADD`
```rust
const FUTEX_OP_ADD: c_int = 1i32;
```

### `FUTEX_OP_OR`
```rust
const FUTEX_OP_OR: c_int = 2i32;
```

### `FUTEX_OP_ANDN`
```rust
const FUTEX_OP_ANDN: c_int = 3i32;
```

### `FUTEX_OP_XOR`
```rust
const FUTEX_OP_XOR: c_int = 4i32;
```

### `FUTEX_OP_OPARG_SHIFT`
```rust
const FUTEX_OP_OPARG_SHIFT: c_int = 8i32;
```

### `FUTEX_OP_CMP_EQ`
```rust
const FUTEX_OP_CMP_EQ: c_int = 0i32;
```

### `FUTEX_OP_CMP_NE`
```rust
const FUTEX_OP_CMP_NE: c_int = 1i32;
```

### `FUTEX_OP_CMP_LT`
```rust
const FUTEX_OP_CMP_LT: c_int = 2i32;
```

### `FUTEX_OP_CMP_LE`
```rust
const FUTEX_OP_CMP_LE: c_int = 3i32;
```

### `FUTEX_OP_CMP_GT`
```rust
const FUTEX_OP_CMP_GT: c_int = 4i32;
```

### `FUTEX_OP_CMP_GE`
```rust
const FUTEX_OP_CMP_GE: c_int = 5i32;
```

### `KEXEC_ON_CRASH`
```rust
const KEXEC_ON_CRASH: c_int = 1i32;
```

### `KEXEC_PRESERVE_CONTEXT`
```rust
const KEXEC_PRESERVE_CONTEXT: c_int = 2i32;
```

### `KEXEC_ARCH_MASK`
```rust
const KEXEC_ARCH_MASK: c_int = -65_536i32;
```

### `KEXEC_FILE_UNLOAD`
```rust
const KEXEC_FILE_UNLOAD: c_int = 1i32;
```

### `KEXEC_FILE_ON_CRASH`
```rust
const KEXEC_FILE_ON_CRASH: c_int = 2i32;
```

### `KEXEC_FILE_NO_INITRAMFS`
```rust
const KEXEC_FILE_NO_INITRAMFS: c_int = 4i32;
```

### `LINUX_REBOOT_MAGIC1`
```rust
const LINUX_REBOOT_MAGIC1: c_int = -18_751_827i32;
```

### `LINUX_REBOOT_MAGIC2`
```rust
const LINUX_REBOOT_MAGIC2: c_int = 672_274_793i32;
```

### `LINUX_REBOOT_MAGIC2A`
```rust
const LINUX_REBOOT_MAGIC2A: c_int = 85_072_278i32;
```

### `LINUX_REBOOT_MAGIC2B`
```rust
const LINUX_REBOOT_MAGIC2B: c_int = 369_367_448i32;
```

### `LINUX_REBOOT_MAGIC2C`
```rust
const LINUX_REBOOT_MAGIC2C: c_int = 537_993_216i32;
```

### `LINUX_REBOOT_CMD_RESTART`
```rust
const LINUX_REBOOT_CMD_RESTART: c_int = 19_088_743i32;
```

### `LINUX_REBOOT_CMD_HALT`
```rust
const LINUX_REBOOT_CMD_HALT: c_int = -839_974_621i32;
```

### `LINUX_REBOOT_CMD_CAD_ON`
```rust
const LINUX_REBOOT_CMD_CAD_ON: c_int = -1_985_229_329i32;
```

### `LINUX_REBOOT_CMD_CAD_OFF`
```rust
const LINUX_REBOOT_CMD_CAD_OFF: c_int = 0i32;
```

### `LINUX_REBOOT_CMD_POWER_OFF`
```rust
const LINUX_REBOOT_CMD_POWER_OFF: c_int = 1_126_301_404i32;
```

### `LINUX_REBOOT_CMD_RESTART2`
```rust
const LINUX_REBOOT_CMD_RESTART2: c_int = -1_582_119_980i32;
```

### `LINUX_REBOOT_CMD_SW_SUSPEND`
```rust
const LINUX_REBOOT_CMD_SW_SUSPEND: c_int = -805_241_630i32;
```

### `LINUX_REBOOT_CMD_KEXEC`
```rust
const LINUX_REBOOT_CMD_KEXEC: c_int = 1_163_412_803i32;
```

### `SO_EE_ORIGIN_NONE`
```rust
const SO_EE_ORIGIN_NONE: u8 = 0u8;
```

### `SO_EE_ORIGIN_LOCAL`
```rust
const SO_EE_ORIGIN_LOCAL: u8 = 1u8;
```

### `SO_EE_ORIGIN_ICMP`
```rust
const SO_EE_ORIGIN_ICMP: u8 = 2u8;
```

### `SO_EE_ORIGIN_ICMP6`
```rust
const SO_EE_ORIGIN_ICMP6: u8 = 3u8;
```

### `SO_EE_ORIGIN_TXSTATUS`
```rust
const SO_EE_ORIGIN_TXSTATUS: u8 = 4u8;
```

### `SO_EE_ORIGIN_TIMESTAMPING`
```rust
const SO_EE_ORIGIN_TIMESTAMPING: u8 = 4u8;
```

### `SCTP_FUTURE_ASSOC`
```rust
const SCTP_FUTURE_ASSOC: c_int = 0i32;
```

### `SCTP_CURRENT_ASSOC`
```rust
const SCTP_CURRENT_ASSOC: c_int = 1i32;
```

### `SCTP_ALL_ASSOC`
```rust
const SCTP_ALL_ASSOC: c_int = 2i32;
```

### `SCTP_RTOINFO`
```rust
const SCTP_RTOINFO: c_int = 0i32;
```

### `SCTP_ASSOCINFO`
```rust
const SCTP_ASSOCINFO: c_int = 1i32;
```

### `SCTP_INITMSG`
```rust
const SCTP_INITMSG: c_int = 2i32;
```

### `SCTP_NODELAY`
```rust
const SCTP_NODELAY: c_int = 3i32;
```

### `SCTP_AUTOCLOSE`
```rust
const SCTP_AUTOCLOSE: c_int = 4i32;
```

### `SCTP_SET_PEER_PRIMARY_ADDR`
```rust
const SCTP_SET_PEER_PRIMARY_ADDR: c_int = 5i32;
```

### `SCTP_PRIMARY_ADDR`
```rust
const SCTP_PRIMARY_ADDR: c_int = 6i32;
```

### `SCTP_ADAPTATION_LAYER`
```rust
const SCTP_ADAPTATION_LAYER: c_int = 7i32;
```

### `SCTP_DISABLE_FRAGMENTS`
```rust
const SCTP_DISABLE_FRAGMENTS: c_int = 8i32;
```

### `SCTP_PEER_ADDR_PARAMS`
```rust
const SCTP_PEER_ADDR_PARAMS: c_int = 9i32;
```

### `SCTP_DEFAULT_SEND_PARAM`
```rust
const SCTP_DEFAULT_SEND_PARAM: c_int = 10i32;
```

### `SCTP_EVENTS`
```rust
const SCTP_EVENTS: c_int = 11i32;
```

### `SCTP_I_WANT_MAPPED_V4_ADDR`
```rust
const SCTP_I_WANT_MAPPED_V4_ADDR: c_int = 12i32;
```

### `SCTP_MAXSEG`
```rust
const SCTP_MAXSEG: c_int = 13i32;
```

### `SCTP_STATUS`
```rust
const SCTP_STATUS: c_int = 14i32;
```

### `SCTP_GET_PEER_ADDR_INFO`
```rust
const SCTP_GET_PEER_ADDR_INFO: c_int = 15i32;
```

### `SCTP_DELAYED_ACK_TIME`
```rust
const SCTP_DELAYED_ACK_TIME: c_int = 16i32;
```

### `SCTP_DELAYED_ACK`
```rust
const SCTP_DELAYED_ACK: c_int = 16i32;
```

### `SCTP_DELAYED_SACK`
```rust
const SCTP_DELAYED_SACK: c_int = 16i32;
```

### `SCTP_CONTEXT`
```rust
const SCTP_CONTEXT: c_int = 17i32;
```

### `SCTP_FRAGMENT_INTERLEAVE`
```rust
const SCTP_FRAGMENT_INTERLEAVE: c_int = 18i32;
```

### `SCTP_PARTIAL_DELIVERY_POINT`
```rust
const SCTP_PARTIAL_DELIVERY_POINT: c_int = 19i32;
```

### `SCTP_MAX_BURST`
```rust
const SCTP_MAX_BURST: c_int = 20i32;
```

### `SCTP_AUTH_CHUNK`
```rust
const SCTP_AUTH_CHUNK: c_int = 21i32;
```

### `SCTP_HMAC_IDENT`
```rust
const SCTP_HMAC_IDENT: c_int = 22i32;
```

### `SCTP_AUTH_KEY`
```rust
const SCTP_AUTH_KEY: c_int = 23i32;
```

### `SCTP_AUTH_ACTIVE_KEY`
```rust
const SCTP_AUTH_ACTIVE_KEY: c_int = 24i32;
```

### `SCTP_AUTH_DELETE_KEY`
```rust
const SCTP_AUTH_DELETE_KEY: c_int = 25i32;
```

### `SCTP_PEER_AUTH_CHUNKS`
```rust
const SCTP_PEER_AUTH_CHUNKS: c_int = 26i32;
```

### `SCTP_LOCAL_AUTH_CHUNKS`
```rust
const SCTP_LOCAL_AUTH_CHUNKS: c_int = 27i32;
```

### `SCTP_GET_ASSOC_NUMBER`
```rust
const SCTP_GET_ASSOC_NUMBER: c_int = 28i32;
```

### `SCTP_GET_ASSOC_ID_LIST`
```rust
const SCTP_GET_ASSOC_ID_LIST: c_int = 29i32;
```

### `SCTP_AUTO_ASCONF`
```rust
const SCTP_AUTO_ASCONF: c_int = 30i32;
```

### `SCTP_PEER_ADDR_THLDS`
```rust
const SCTP_PEER_ADDR_THLDS: c_int = 31i32;
```

### `SCTP_RECVRCVINFO`
```rust
const SCTP_RECVRCVINFO: c_int = 32i32;
```

### `SCTP_RECVNXTINFO`
```rust
const SCTP_RECVNXTINFO: c_int = 33i32;
```

### `SCTP_DEFAULT_SNDINFO`
```rust
const SCTP_DEFAULT_SNDINFO: c_int = 34i32;
```

### `SCTP_AUTH_DEACTIVATE_KEY`
```rust
const SCTP_AUTH_DEACTIVATE_KEY: c_int = 35i32;
```

### `SCTP_REUSE_PORT`
```rust
const SCTP_REUSE_PORT: c_int = 36i32;
```

### `SCTP_PEER_ADDR_THLDS_V2`
```rust
const SCTP_PEER_ADDR_THLDS_V2: c_int = 37i32;
```

### `SCTP_PR_SCTP_NONE`
```rust
const SCTP_PR_SCTP_NONE: c_int = 0i32;
```

### `SCTP_PR_SCTP_TTL`
```rust
const SCTP_PR_SCTP_TTL: c_int = 16i32;
```

### `SCTP_PR_SCTP_RTX`
```rust
const SCTP_PR_SCTP_RTX: c_int = 32i32;
```

### `SCTP_PR_SCTP_PRIO`
```rust
const SCTP_PR_SCTP_PRIO: c_int = 48i32;
```

### `SCTP_PR_SCTP_MAX`
```rust
const SCTP_PR_SCTP_MAX: c_int = 48i32;
```

### `SCTP_PR_SCTP_MASK`
```rust
const SCTP_PR_SCTP_MASK: c_int = 48i32;
```

### `SCTP_ENABLE_RESET_STREAM_REQ`
```rust
const SCTP_ENABLE_RESET_STREAM_REQ: c_int = 1i32;
```

### `SCTP_ENABLE_RESET_ASSOC_REQ`
```rust
const SCTP_ENABLE_RESET_ASSOC_REQ: c_int = 2i32;
```

### `SCTP_ENABLE_CHANGE_ASSOC_REQ`
```rust
const SCTP_ENABLE_CHANGE_ASSOC_REQ: c_int = 4i32;
```

### `SCTP_ENABLE_STRRESET_MASK`
```rust
const SCTP_ENABLE_STRRESET_MASK: c_int = 7i32;
```

### `SCTP_STREAM_RESET_INCOMING`
```rust
const SCTP_STREAM_RESET_INCOMING: c_int = 1i32;
```

### `SCTP_STREAM_RESET_OUTGOING`
```rust
const SCTP_STREAM_RESET_OUTGOING: c_int = 2i32;
```

### `SCTP_INIT`
```rust
const SCTP_INIT: c_int = 0i32;
```

### `SCTP_SNDRCV`
```rust
const SCTP_SNDRCV: c_int = 1i32;
```

### `SCTP_SNDINFO`
```rust
const SCTP_SNDINFO: c_int = 2i32;
```

### `SCTP_RCVINFO`
```rust
const SCTP_RCVINFO: c_int = 3i32;
```

### `SCTP_NXTINFO`
```rust
const SCTP_NXTINFO: c_int = 4i32;
```

### `SCTP_PRINFO`
```rust
const SCTP_PRINFO: c_int = 5i32;
```

### `SCTP_AUTHINFO`
```rust
const SCTP_AUTHINFO: c_int = 6i32;
```

### `SCTP_DSTADDRV4`
```rust
const SCTP_DSTADDRV4: c_int = 7i32;
```

### `SCTP_DSTADDRV6`
```rust
const SCTP_DSTADDRV6: c_int = 8i32;
```

### `SCTP_UNORDERED`
```rust
const SCTP_UNORDERED: c_int = 1i32;
```

### `SCTP_ADDR_OVER`
```rust
const SCTP_ADDR_OVER: c_int = 2i32;
```

### `SCTP_ABORT`
```rust
const SCTP_ABORT: c_int = 4i32;
```

### `SCTP_SACK_IMMEDIATELY`
```rust
const SCTP_SACK_IMMEDIATELY: c_int = 8i32;
```

### `SCTP_SENDALL`
```rust
const SCTP_SENDALL: c_int = 64i32;
```

### `SCTP_PR_SCTP_ALL`
```rust
const SCTP_PR_SCTP_ALL: c_int = 128i32;
```

### `SCTP_NOTIFICATION`
```rust
const SCTP_NOTIFICATION: c_int = 32_768i32;
```

### `SCTP_EOF`
```rust
const SCTP_EOF: c_int = 512i32;
```

### `DCCP_SOCKOPT_PACKET_SIZE`
```rust
const DCCP_SOCKOPT_PACKET_SIZE: c_int = 1i32;
```

### `DCCP_SOCKOPT_SERVICE`
```rust
const DCCP_SOCKOPT_SERVICE: c_int = 2i32;
```

### `DCCP_SOCKOPT_CHANGE_L`
```rust
const DCCP_SOCKOPT_CHANGE_L: c_int = 3i32;
```

### `DCCP_SOCKOPT_CHANGE_R`
```rust
const DCCP_SOCKOPT_CHANGE_R: c_int = 4i32;
```

### `DCCP_SOCKOPT_GET_CUR_MPS`
```rust
const DCCP_SOCKOPT_GET_CUR_MPS: c_int = 5i32;
```

### `DCCP_SOCKOPT_SERVER_TIMEWAIT`
```rust
const DCCP_SOCKOPT_SERVER_TIMEWAIT: c_int = 6i32;
```

### `DCCP_SOCKOPT_SEND_CSCOV`
```rust
const DCCP_SOCKOPT_SEND_CSCOV: c_int = 10i32;
```

### `DCCP_SOCKOPT_RECV_CSCOV`
```rust
const DCCP_SOCKOPT_RECV_CSCOV: c_int = 11i32;
```

### `DCCP_SOCKOPT_AVAILABLE_CCIDS`
```rust
const DCCP_SOCKOPT_AVAILABLE_CCIDS: c_int = 12i32;
```

### `DCCP_SOCKOPT_CCID`
```rust
const DCCP_SOCKOPT_CCID: c_int = 13i32;
```

### `DCCP_SOCKOPT_TX_CCID`
```rust
const DCCP_SOCKOPT_TX_CCID: c_int = 14i32;
```

### `DCCP_SOCKOPT_RX_CCID`
```rust
const DCCP_SOCKOPT_RX_CCID: c_int = 15i32;
```

### `DCCP_SOCKOPT_QPOLICY_ID`
```rust
const DCCP_SOCKOPT_QPOLICY_ID: c_int = 16i32;
```

### `DCCP_SOCKOPT_QPOLICY_TXQLEN`
```rust
const DCCP_SOCKOPT_QPOLICY_TXQLEN: c_int = 17i32;
```

### `DCCP_SOCKOPT_CCID_RX_INFO`
```rust
const DCCP_SOCKOPT_CCID_RX_INFO: c_int = 128i32;
```

### `DCCP_SOCKOPT_CCID_TX_INFO`
```rust
const DCCP_SOCKOPT_CCID_TX_INFO: c_int = 192i32;
```

### `DCCP_SERVICE_LIST_MAX_LEN`
```rust
const DCCP_SERVICE_LIST_MAX_LEN: c_int = 32i32;
```

maximum number of services provided on the same listening port

### `CTL_KERN`
```rust
const CTL_KERN: c_int = 1i32;
```

### `CTL_VM`
```rust
const CTL_VM: c_int = 2i32;
```

### `CTL_NET`
```rust
const CTL_NET: c_int = 3i32;
```

### `CTL_FS`
```rust
const CTL_FS: c_int = 5i32;
```

### `CTL_DEBUG`
```rust
const CTL_DEBUG: c_int = 6i32;
```

### `CTL_DEV`
```rust
const CTL_DEV: c_int = 7i32;
```

### `CTL_BUS`
```rust
const CTL_BUS: c_int = 8i32;
```

### `CTL_ABI`
```rust
const CTL_ABI: c_int = 9i32;
```

### `CTL_CPU`
```rust
const CTL_CPU: c_int = 10i32;
```

### `CTL_BUS_ISA`
```rust
const CTL_BUS_ISA: c_int = 1i32;
```

### `INOTIFY_MAX_USER_INSTANCES`
```rust
const INOTIFY_MAX_USER_INSTANCES: c_int = 1i32;
```

### `INOTIFY_MAX_USER_WATCHES`
```rust
const INOTIFY_MAX_USER_WATCHES: c_int = 2i32;
```

### `INOTIFY_MAX_QUEUED_EVENTS`
```rust
const INOTIFY_MAX_QUEUED_EVENTS: c_int = 3i32;
```

### `KERN_OSTYPE`
```rust
const KERN_OSTYPE: c_int = 1i32;
```

### `KERN_OSRELEASE`
```rust
const KERN_OSRELEASE: c_int = 2i32;
```

### `KERN_OSREV`
```rust
const KERN_OSREV: c_int = 3i32;
```

### `KERN_VERSION`
```rust
const KERN_VERSION: c_int = 4i32;
```

### `KERN_SECUREMASK`
```rust
const KERN_SECUREMASK: c_int = 5i32;
```

### `KERN_PROF`
```rust
const KERN_PROF: c_int = 6i32;
```

### `KERN_NODENAME`
```rust
const KERN_NODENAME: c_int = 7i32;
```

### `KERN_DOMAINNAME`
```rust
const KERN_DOMAINNAME: c_int = 8i32;
```

### `KERN_PANIC`
```rust
const KERN_PANIC: c_int = 15i32;
```

### `KERN_REALROOTDEV`
```rust
const KERN_REALROOTDEV: c_int = 16i32;
```

### `KERN_SPARC_REBOOT`
```rust
const KERN_SPARC_REBOOT: c_int = 21i32;
```

### `KERN_CTLALTDEL`
```rust
const KERN_CTLALTDEL: c_int = 22i32;
```

### `KERN_PRINTK`
```rust
const KERN_PRINTK: c_int = 23i32;
```

### `KERN_NAMETRANS`
```rust
const KERN_NAMETRANS: c_int = 24i32;
```

### `KERN_PPC_HTABRECLAIM`
```rust
const KERN_PPC_HTABRECLAIM: c_int = 25i32;
```

### `KERN_PPC_ZEROPAGED`
```rust
const KERN_PPC_ZEROPAGED: c_int = 26i32;
```

### `KERN_PPC_POWERSAVE_NAP`
```rust
const KERN_PPC_POWERSAVE_NAP: c_int = 27i32;
```

### `KERN_MODPROBE`
```rust
const KERN_MODPROBE: c_int = 28i32;
```

### `KERN_SG_BIG_BUFF`
```rust
const KERN_SG_BIG_BUFF: c_int = 29i32;
```

### `KERN_ACCT`
```rust
const KERN_ACCT: c_int = 30i32;
```

### `KERN_PPC_L2CR`
```rust
const KERN_PPC_L2CR: c_int = 31i32;
```

### `KERN_RTSIGNR`
```rust
const KERN_RTSIGNR: c_int = 32i32;
```

### `KERN_RTSIGMAX`
```rust
const KERN_RTSIGMAX: c_int = 33i32;
```

### `KERN_SHMMAX`
```rust
const KERN_SHMMAX: c_int = 34i32;
```

### `KERN_MSGMAX`
```rust
const KERN_MSGMAX: c_int = 35i32;
```

### `KERN_MSGMNB`
```rust
const KERN_MSGMNB: c_int = 36i32;
```

### `KERN_MSGPOOL`
```rust
const KERN_MSGPOOL: c_int = 37i32;
```

### `KERN_SYSRQ`
```rust
const KERN_SYSRQ: c_int = 38i32;
```

### `KERN_MAX_THREADS`
```rust
const KERN_MAX_THREADS: c_int = 39i32;
```

### `KERN_RANDOM`
```rust
const KERN_RANDOM: c_int = 40i32;
```

### `KERN_SHMALL`
```rust
const KERN_SHMALL: c_int = 41i32;
```

### `KERN_MSGMNI`
```rust
const KERN_MSGMNI: c_int = 42i32;
```

### `KERN_SEM`
```rust
const KERN_SEM: c_int = 43i32;
```

### `KERN_SPARC_STOP_A`
```rust
const KERN_SPARC_STOP_A: c_int = 44i32;
```

### `KERN_SHMMNI`
```rust
const KERN_SHMMNI: c_int = 45i32;
```

### `KERN_OVERFLOWUID`
```rust
const KERN_OVERFLOWUID: c_int = 46i32;
```

### `KERN_OVERFLOWGID`
```rust
const KERN_OVERFLOWGID: c_int = 47i32;
```

### `KERN_SHMPATH`
```rust
const KERN_SHMPATH: c_int = 48i32;
```

### `KERN_HOTPLUG`
```rust
const KERN_HOTPLUG: c_int = 49i32;
```

### `KERN_IEEE_EMULATION_WARNINGS`
```rust
const KERN_IEEE_EMULATION_WARNINGS: c_int = 50i32;
```

### `KERN_S390_USER_DEBUG_LOGGING`
```rust
const KERN_S390_USER_DEBUG_LOGGING: c_int = 51i32;
```

### `KERN_CORE_USES_PID`
```rust
const KERN_CORE_USES_PID: c_int = 52i32;
```

### `KERN_TAINTED`
```rust
const KERN_TAINTED: c_int = 53i32;
```

### `KERN_CADPID`
```rust
const KERN_CADPID: c_int = 54i32;
```

### `KERN_PIDMAX`
```rust
const KERN_PIDMAX: c_int = 55i32;
```

### `KERN_CORE_PATTERN`
```rust
const KERN_CORE_PATTERN: c_int = 56i32;
```

### `KERN_PANIC_ON_OOPS`
```rust
const KERN_PANIC_ON_OOPS: c_int = 57i32;
```

### `KERN_HPPA_PWRSW`
```rust
const KERN_HPPA_PWRSW: c_int = 58i32;
```

### `KERN_HPPA_UNALIGNED`
```rust
const KERN_HPPA_UNALIGNED: c_int = 59i32;
```

### `KERN_PRINTK_RATELIMIT`
```rust
const KERN_PRINTK_RATELIMIT: c_int = 60i32;
```

### `KERN_PRINTK_RATELIMIT_BURST`
```rust
const KERN_PRINTK_RATELIMIT_BURST: c_int = 61i32;
```

### `KERN_PTY`
```rust
const KERN_PTY: c_int = 62i32;
```

### `KERN_NGROUPS_MAX`
```rust
const KERN_NGROUPS_MAX: c_int = 63i32;
```

### `KERN_SPARC_SCONS_PWROFF`
```rust
const KERN_SPARC_SCONS_PWROFF: c_int = 64i32;
```

### `KERN_HZ_TIMER`
```rust
const KERN_HZ_TIMER: c_int = 65i32;
```

### `KERN_UNKNOWN_NMI_PANIC`
```rust
const KERN_UNKNOWN_NMI_PANIC: c_int = 66i32;
```

### `KERN_BOOTLOADER_TYPE`
```rust
const KERN_BOOTLOADER_TYPE: c_int = 67i32;
```

### `KERN_RANDOMIZE`
```rust
const KERN_RANDOMIZE: c_int = 68i32;
```

### `KERN_SETUID_DUMPABLE`
```rust
const KERN_SETUID_DUMPABLE: c_int = 69i32;
```

### `KERN_SPIN_RETRY`
```rust
const KERN_SPIN_RETRY: c_int = 70i32;
```

### `KERN_ACPI_VIDEO_FLAGS`
```rust
const KERN_ACPI_VIDEO_FLAGS: c_int = 71i32;
```

### `KERN_IA64_UNALIGNED`
```rust
const KERN_IA64_UNALIGNED: c_int = 72i32;
```

### `KERN_COMPAT_LOG`
```rust
const KERN_COMPAT_LOG: c_int = 73i32;
```

### `KERN_MAX_LOCK_DEPTH`
```rust
const KERN_MAX_LOCK_DEPTH: c_int = 74i32;
```

### `KERN_NMI_WATCHDOG`
```rust
const KERN_NMI_WATCHDOG: c_int = 75i32;
```

### `KERN_PANIC_ON_NMI`
```rust
const KERN_PANIC_ON_NMI: c_int = 76i32;
```

### `VM_OVERCOMMIT_MEMORY`
```rust
const VM_OVERCOMMIT_MEMORY: c_int = 5i32;
```

### `VM_PAGE_CLUSTER`
```rust
const VM_PAGE_CLUSTER: c_int = 10i32;
```

### `VM_DIRTY_BACKGROUND`
```rust
const VM_DIRTY_BACKGROUND: c_int = 11i32;
```

### `VM_DIRTY_RATIO`
```rust
const VM_DIRTY_RATIO: c_int = 12i32;
```

### `VM_DIRTY_WB_CS`
```rust
const VM_DIRTY_WB_CS: c_int = 13i32;
```

### `VM_DIRTY_EXPIRE_CS`
```rust
const VM_DIRTY_EXPIRE_CS: c_int = 14i32;
```

### `VM_NR_PDFLUSH_THREADS`
```rust
const VM_NR_PDFLUSH_THREADS: c_int = 15i32;
```

### `VM_OVERCOMMIT_RATIO`
```rust
const VM_OVERCOMMIT_RATIO: c_int = 16i32;
```

### `VM_PAGEBUF`
```rust
const VM_PAGEBUF: c_int = 17i32;
```

### `VM_HUGETLB_PAGES`
```rust
const VM_HUGETLB_PAGES: c_int = 18i32;
```

### `VM_SWAPPINESS`
```rust
const VM_SWAPPINESS: c_int = 19i32;
```

### `VM_LOWMEM_RESERVE_RATIO`
```rust
const VM_LOWMEM_RESERVE_RATIO: c_int = 20i32;
```

### `VM_MIN_FREE_KBYTES`
```rust
const VM_MIN_FREE_KBYTES: c_int = 21i32;
```

### `VM_MAX_MAP_COUNT`
```rust
const VM_MAX_MAP_COUNT: c_int = 22i32;
```

### `VM_LAPTOP_MODE`
```rust
const VM_LAPTOP_MODE: c_int = 23i32;
```

### `VM_BLOCK_DUMP`
```rust
const VM_BLOCK_DUMP: c_int = 24i32;
```

### `VM_HUGETLB_GROUP`
```rust
const VM_HUGETLB_GROUP: c_int = 25i32;
```

### `VM_VFS_CACHE_PRESSURE`
```rust
const VM_VFS_CACHE_PRESSURE: c_int = 26i32;
```

### `VM_LEGACY_VA_LAYOUT`
```rust
const VM_LEGACY_VA_LAYOUT: c_int = 27i32;
```

### `VM_SWAP_TOKEN_TIMEOUT`
```rust
const VM_SWAP_TOKEN_TIMEOUT: c_int = 28i32;
```

### `VM_DROP_PAGECACHE`
```rust
const VM_DROP_PAGECACHE: c_int = 29i32;
```

### `VM_PERCPU_PAGELIST_FRACTION`
```rust
const VM_PERCPU_PAGELIST_FRACTION: c_int = 30i32;
```

### `VM_ZONE_RECLAIM_MODE`
```rust
const VM_ZONE_RECLAIM_MODE: c_int = 31i32;
```

### `VM_MIN_UNMAPPED`
```rust
const VM_MIN_UNMAPPED: c_int = 32i32;
```

### `VM_PANIC_ON_OOM`
```rust
const VM_PANIC_ON_OOM: c_int = 33i32;
```

### `VM_VDSO_ENABLED`
```rust
const VM_VDSO_ENABLED: c_int = 34i32;
```

### `VM_MIN_SLAB`
```rust
const VM_MIN_SLAB: c_int = 35i32;
```

### `NET_CORE`
```rust
const NET_CORE: c_int = 1i32;
```

### `NET_ETHER`
```rust
const NET_ETHER: c_int = 2i32;
```

### `NET_802`
```rust
const NET_802: c_int = 3i32;
```

### `NET_UNIX`
```rust
const NET_UNIX: c_int = 4i32;
```

### `NET_IPV4`
```rust
const NET_IPV4: c_int = 5i32;
```

### `NET_IPX`
```rust
const NET_IPX: c_int = 6i32;
```

### `NET_ATALK`
```rust
const NET_ATALK: c_int = 7i32;
```

### `NET_NETROM`
```rust
const NET_NETROM: c_int = 8i32;
```

### `NET_AX25`
```rust
const NET_AX25: c_int = 9i32;
```

### `NET_BRIDGE`
```rust
const NET_BRIDGE: c_int = 10i32;
```

### `NET_ROSE`
```rust
const NET_ROSE: c_int = 11i32;
```

### `NET_IPV6`
```rust
const NET_IPV6: c_int = 12i32;
```

### `NET_X25`
```rust
const NET_X25: c_int = 13i32;
```

### `NET_TR`
```rust
const NET_TR: c_int = 14i32;
```

### `NET_DECNET`
```rust
const NET_DECNET: c_int = 15i32;
```

### `NET_ECONET`
```rust
const NET_ECONET: c_int = 16i32;
```

### `NET_SCTP`
```rust
const NET_SCTP: c_int = 17i32;
```

### `NET_LLC`
```rust
const NET_LLC: c_int = 18i32;
```

### `NET_NETFILTER`
```rust
const NET_NETFILTER: c_int = 19i32;
```

### `NET_DCCP`
```rust
const NET_DCCP: c_int = 20i32;
```

### `NET_IRDA`
```rust
const NET_IRDA: c_int = 412i32;
```

### `PF_VCPU`
```rust
const PF_VCPU: c_int = 1i32;
```

I'm a virtual CPU.

### `PF_IDLE`
```rust
const PF_IDLE: c_int = 2i32;
```

I am an IDLE thread.

### `PF_EXITING`
```rust
const PF_EXITING: c_int = 4i32;
```

Getting shut down.

### `PF_POSTCOREDUMP`
```rust
const PF_POSTCOREDUMP: c_int = 8i32;
```

Coredumps should ignore this task.

### `PF_IO_WORKER`
```rust
const PF_IO_WORKER: c_int = 16i32;
```

Task is an IO worker.

### `PF_WQ_WORKER`
```rust
const PF_WQ_WORKER: c_int = 32i32;
```

I'm a workqueue worker.

### `PF_FORKNOEXEC`
```rust
const PF_FORKNOEXEC: c_int = 64i32;
```

Forked but didn't exec.

### `PF_MCE_PROCESS`
```rust
const PF_MCE_PROCESS: c_int = 128i32;
```

Process policy on mce errors.

### `PF_SUPERPRIV`
```rust
const PF_SUPERPRIV: c_int = 256i32;
```

Used super-user privileges.

### `PF_DUMPCORE`
```rust
const PF_DUMPCORE: c_int = 512i32;
```

Dumped core.

### `PF_SIGNALED`
```rust
const PF_SIGNALED: c_int = 1_024i32;
```

Killed by a signal.

### `PF_MEMALLOC`
```rust
const PF_MEMALLOC: c_int = 2_048i32;
```

Allocating memory to free memory.

See `memalloc_noreclaim_save()`.

### `PF_NPROC_EXCEEDED`
```rust
const PF_NPROC_EXCEEDED: c_int = 4_096i32;
```

`set_user()` noticed that `RLIMIT_NPROC` was exceeded.

### `PF_USED_MATH`
```rust
const PF_USED_MATH: c_int = 8_192i32;
```

If unset the fpu must be initialized before use.

### `PF_USER_WORKER`
```rust
const PF_USER_WORKER: c_int = 16_384i32;
```

Kernel thread cloned from userspace thread.

### `PF_NOFREEZE`
```rust
const PF_NOFREEZE: c_int = 32_768i32;
```

This thread should not be frozen.

### `PF_KSWAPD`
```rust
const PF_KSWAPD: c_int = 131_072i32;
```

I am `kswapd`.

### `PF_MEMALLOC_NOFS`
```rust
const PF_MEMALLOC_NOFS: c_int = 262_144i32;
```

All allocations inherit `GFP_NOFS`.

See `memalloc_nfs_save()`.

### `PF_MEMALLOC_NOIO`
```rust
const PF_MEMALLOC_NOIO: c_int = 524_288i32;
```

All allocations inherit `GFP_NOIO`.

See `memalloc_noio_save()`.

### `PF_LOCAL_THROTTLE`
```rust
const PF_LOCAL_THROTTLE: c_int = 1_048_576i32;
```

Throttle writes only against the bdi I write to, I am cleaning
dirty pages from some other bdi.

### `PF_KTHREAD`
```rust
const PF_KTHREAD: c_int = 2_097_152i32;
```

I am a kernel thread.

### `PF_RANDOMIZE`
```rust
const PF_RANDOMIZE: c_int = 4_194_304i32;
```

Randomize virtual address space.

### `PF_NO_SETAFFINITY`
```rust
const PF_NO_SETAFFINITY: c_int = 67_108_864i32;
```

Userland is not allowed to meddle with `cpus_mask`.

### `PF_MCE_EARLY`
```rust
const PF_MCE_EARLY: c_int = 134_217_728i32;
```

Early kill for mce process policy.

### `PF_MEMALLOC_PIN`
```rust
const PF_MEMALLOC_PIN: c_int = 268_435_456i32;
```

Allocations constrained to zones which allow long term pinning.

See `memalloc_pin_save()`.

### `PF_BLOCK_TS`
```rust
const PF_BLOCK_TS: c_int = 536_870_912i32;
```

Plug has ts that needs updating.

### `PF_SUSPEND_TASK`
```rust
const PF_SUSPEND_TASK: c_int = -2_147_483_648i32;
```

This thread called `freeze_processes()` and should not be frozen.

### `PF_SUSPEND_TASK_UINT`
```rust
const PF_SUSPEND_TASK_UINT: c_uint = 2_147_483_648u32;
```

### `CLONE_PIDFD`
```rust
const CLONE_PIDFD: c_int = 4_096i32;
```

### `SCHED_FLAG_RESET_ON_FORK`
```rust
const SCHED_FLAG_RESET_ON_FORK: c_int = 1i32;
```

### `SCHED_FLAG_RECLAIM`
```rust
const SCHED_FLAG_RECLAIM: c_int = 2i32;
```

### `SCHED_FLAG_DL_OVERRUN`
```rust
const SCHED_FLAG_DL_OVERRUN: c_int = 4i32;
```

### `SCHED_FLAG_KEEP_POLICY`
```rust
const SCHED_FLAG_KEEP_POLICY: c_int = 8i32;
```

### `SCHED_FLAG_KEEP_PARAMS`
```rust
const SCHED_FLAG_KEEP_PARAMS: c_int = 16i32;
```

### `SCHED_FLAG_UTIL_CLAMP_MIN`
```rust
const SCHED_FLAG_UTIL_CLAMP_MIN: c_int = 32i32;
```

### `SCHED_FLAG_UTIL_CLAMP_MAX`
```rust
const SCHED_FLAG_UTIL_CLAMP_MAX: c_int = 64i32;
```

### `XDP_SHARED_UMEM`
```rust
const XDP_SHARED_UMEM: crate::__u16 = 1u16;
```

### `XDP_COPY`
```rust
const XDP_COPY: crate::__u16 = 2u16;
```

### `XDP_ZEROCOPY`
```rust
const XDP_ZEROCOPY: crate::__u16 = 4u16;
```

### `XDP_USE_NEED_WAKEUP`
```rust
const XDP_USE_NEED_WAKEUP: crate::__u16 = 8u16;
```

### `XDP_USE_SG`
```rust
const XDP_USE_SG: crate::__u16 = 16u16;
```

### `XDP_UMEM_UNALIGNED_CHUNK_FLAG`
```rust
const XDP_UMEM_UNALIGNED_CHUNK_FLAG: crate::__u32 = 1u32;
```

### `XDP_RING_NEED_WAKEUP`
```rust
const XDP_RING_NEED_WAKEUP: crate::__u32 = 1u32;
```

### `XDP_MMAP_OFFSETS`
```rust
const XDP_MMAP_OFFSETS: c_int = 1i32;
```

### `XDP_RX_RING`
```rust
const XDP_RX_RING: c_int = 2i32;
```

### `XDP_TX_RING`
```rust
const XDP_TX_RING: c_int = 3i32;
```

### `XDP_UMEM_REG`
```rust
const XDP_UMEM_REG: c_int = 4i32;
```

### `XDP_UMEM_FILL_RING`
```rust
const XDP_UMEM_FILL_RING: c_int = 5i32;
```

### `XDP_UMEM_COMPLETION_RING`
```rust
const XDP_UMEM_COMPLETION_RING: c_int = 6i32;
```

### `XDP_STATISTICS`
```rust
const XDP_STATISTICS: c_int = 7i32;
```

### `XDP_OPTIONS`
```rust
const XDP_OPTIONS: c_int = 8i32;
```

### `XDP_OPTIONS_ZEROCOPY`
```rust
const XDP_OPTIONS_ZEROCOPY: crate::__u32 = 1u32;
```

### `XDP_PGOFF_RX_RING`
```rust
const XDP_PGOFF_RX_RING: crate::off_t = 0i64;
```

### `XDP_PGOFF_TX_RING`
```rust
const XDP_PGOFF_TX_RING: crate::off_t = 2_147_483_648i64;
```

### `XDP_UMEM_PGOFF_FILL_RING`
```rust
const XDP_UMEM_PGOFF_FILL_RING: crate::c_ulonglong = 4_294_967_296u64;
```

### `XDP_UMEM_PGOFF_COMPLETION_RING`
```rust
const XDP_UMEM_PGOFF_COMPLETION_RING: crate::c_ulonglong = 6_442_450_944u64;
```

### `XSK_UNALIGNED_BUF_OFFSET_SHIFT`
```rust
const XSK_UNALIGNED_BUF_OFFSET_SHIFT: crate::c_int = 48i32;
```

### `XSK_UNALIGNED_BUF_ADDR_MASK`
```rust
const XSK_UNALIGNED_BUF_ADDR_MASK: crate::c_ulonglong = 281_474_976_710_655u64;
```

### `XDP_PKT_CONTD`
```rust
const XDP_PKT_CONTD: crate::__u32 = 1u32;
```

### `XDP_UMEM_TX_SW_CSUM`
```rust
const XDP_UMEM_TX_SW_CSUM: crate::__u32 = 2u32;
```

### `XDP_UMEM_TX_METADATA_LEN`
```rust
const XDP_UMEM_TX_METADATA_LEN: crate::__u32 = 4u32;
```

### `XDP_TXMD_FLAGS_TIMESTAMP`
```rust
const XDP_TXMD_FLAGS_TIMESTAMP: crate::__u32 = 1u32;
```

### `XDP_TXMD_FLAGS_CHECKSUM`
```rust
const XDP_TXMD_FLAGS_CHECKSUM: crate::__u32 = 2u32;
```

### `XDP_TX_METADATA`
```rust
const XDP_TX_METADATA: crate::__u32 = 2u32;
```

### `SOL_XDP`
```rust
const SOL_XDP: c_int = 283i32;
```

### `MOUNT_ATTR_RDONLY`
```rust
const MOUNT_ATTR_RDONLY: crate::__u64 = 1u64;
```

### `MOUNT_ATTR_NOSUID`
```rust
const MOUNT_ATTR_NOSUID: crate::__u64 = 2u64;
```

### `MOUNT_ATTR_NODEV`
```rust
const MOUNT_ATTR_NODEV: crate::__u64 = 4u64;
```

### `MOUNT_ATTR_NOEXEC`
```rust
const MOUNT_ATTR_NOEXEC: crate::__u64 = 8u64;
```

### `MOUNT_ATTR__ATIME`
```rust
const MOUNT_ATTR__ATIME: crate::__u64 = 112u64;
```

### `MOUNT_ATTR_RELATIME`
```rust
const MOUNT_ATTR_RELATIME: crate::__u64 = 0u64;
```

### `MOUNT_ATTR_NOATIME`
```rust
const MOUNT_ATTR_NOATIME: crate::__u64 = 16u64;
```

### `MOUNT_ATTR_STRICTATIME`
```rust
const MOUNT_ATTR_STRICTATIME: crate::__u64 = 32u64;
```

### `MOUNT_ATTR_NODIRATIME`
```rust
const MOUNT_ATTR_NODIRATIME: crate::__u64 = 128u64;
```

### `MOUNT_ATTR_IDMAP`
```rust
const MOUNT_ATTR_IDMAP: crate::__u64 = 1_048_576u64;
```

### `MOUNT_ATTR_NOSYMFOLLOW`
```rust
const MOUNT_ATTR_NOSYMFOLLOW: crate::__u64 = 2_097_152u64;
```

### `MOUNT_ATTR_SIZE_VER0`
```rust
const MOUNT_ATTR_SIZE_VER0: c_int = 32i32;
```

### `SCHED_FLAG_KEEP_ALL`
```rust
const SCHED_FLAG_KEEP_ALL: c_int = 24i32;
```

### `SCHED_FLAG_UTIL_CLAMP`
```rust
const SCHED_FLAG_UTIL_CLAMP: c_int = 96i32;
```

### `SCHED_FLAG_ALL`
```rust
const SCHED_FLAG_ALL: c_int = 127i32;
```

### `EPIOCSPARAMS`
```rust
const EPIOCSPARAMS: c_ulong = 1_074_301_441u64;
```

### `EPIOCGPARAMS`
```rust
const EPIOCGPARAMS: c_ulong = 2_148_043_266u64;
```

### `SI_DETHREAD`
```rust
const SI_DETHREAD: c_int = -7i32;
```

### `TRAP_PERF`
```rust
const TRAP_PERF: c_int = 6i32;
```

### `DATE_BASE`
```rust
const DATE_BASE: crate::nl_item = 131_072i32;
```

### `ABDAY_1`
```rust
const ABDAY_1: crate::nl_item = 131_072i32;
```

### `ABDAY_2`
```rust
const ABDAY_2: crate::nl_item = 131_073i32;
```

### `ABDAY_3`
```rust
const ABDAY_3: crate::nl_item = 131_074i32;
```

### `ABDAY_4`
```rust
const ABDAY_4: crate::nl_item = 131_075i32;
```

### `ABDAY_5`
```rust
const ABDAY_5: crate::nl_item = 131_076i32;
```

### `ABDAY_6`
```rust
const ABDAY_6: crate::nl_item = 131_077i32;
```

### `ABDAY_7`
```rust
const ABDAY_7: crate::nl_item = 131_078i32;
```

### `DAY_1`
```rust
const DAY_1: crate::nl_item = 131_079i32;
```

### `DAY_2`
```rust
const DAY_2: crate::nl_item = 131_080i32;
```

### `DAY_3`
```rust
const DAY_3: crate::nl_item = 131_081i32;
```

### `DAY_4`
```rust
const DAY_4: crate::nl_item = 131_082i32;
```

### `DAY_5`
```rust
const DAY_5: crate::nl_item = 131_083i32;
```

### `DAY_6`
```rust
const DAY_6: crate::nl_item = 131_084i32;
```

### `DAY_7`
```rust
const DAY_7: crate::nl_item = 131_085i32;
```

### `ABMON_1`
```rust
const ABMON_1: crate::nl_item = 131_086i32;
```

### `ABMON_2`
```rust
const ABMON_2: crate::nl_item = 131_087i32;
```

### `ABMON_3`
```rust
const ABMON_3: crate::nl_item = 131_088i32;
```

### `ABMON_4`
```rust
const ABMON_4: crate::nl_item = 131_089i32;
```

### `ABMON_5`
```rust
const ABMON_5: crate::nl_item = 131_090i32;
```

### `ABMON_6`
```rust
const ABMON_6: crate::nl_item = 131_091i32;
```

### `ABMON_7`
```rust
const ABMON_7: crate::nl_item = 131_092i32;
```

### `ABMON_8`
```rust
const ABMON_8: crate::nl_item = 131_093i32;
```

### `ABMON_9`
```rust
const ABMON_9: crate::nl_item = 131_094i32;
```

### `ABMON_10`
```rust
const ABMON_10: crate::nl_item = 131_095i32;
```

### `ABMON_11`
```rust
const ABMON_11: crate::nl_item = 131_096i32;
```

### `ABMON_12`
```rust
const ABMON_12: crate::nl_item = 131_097i32;
```

### `MON_1`
```rust
const MON_1: crate::nl_item = 131_098i32;
```

### `MON_2`
```rust
const MON_2: crate::nl_item = 131_099i32;
```

### `MON_3`
```rust
const MON_3: crate::nl_item = 131_100i32;
```

### `MON_4`
```rust
const MON_4: crate::nl_item = 131_101i32;
```

### `MON_5`
```rust
const MON_5: crate::nl_item = 131_102i32;
```

### `MON_6`
```rust
const MON_6: crate::nl_item = 131_103i32;
```

### `MON_7`
```rust
const MON_7: crate::nl_item = 131_104i32;
```

### `MON_8`
```rust
const MON_8: crate::nl_item = 131_105i32;
```

### `MON_9`
```rust
const MON_9: crate::nl_item = 131_106i32;
```

### `MON_10`
```rust
const MON_10: crate::nl_item = 131_107i32;
```

### `MON_11`
```rust
const MON_11: crate::nl_item = 131_108i32;
```

### `MON_12`
```rust
const MON_12: crate::nl_item = 131_109i32;
```

### `AM_STR`
```rust
const AM_STR: crate::nl_item = 131_110i32;
```

### `PM_STR`
```rust
const PM_STR: crate::nl_item = 131_111i32;
```

### `D_T_FMT`
```rust
const D_T_FMT: crate::nl_item = 131_112i32;
```

### `D_FMT`
```rust
const D_FMT: crate::nl_item = 131_113i32;
```

### `T_FMT`
```rust
const T_FMT: crate::nl_item = 131_114i32;
```

### `T_FMT_AMPM`
```rust
const T_FMT_AMPM: crate::nl_item = 131_115i32;
```

### `ERA`
```rust
const ERA: crate::nl_item = 131_116i32;
```

### `ERA_D_FMT`
```rust
const ERA_D_FMT: crate::nl_item = 131_118i32;
```

### `ALT_DIGITS`
```rust
const ALT_DIGITS: crate::nl_item = 131_119i32;
```

### `ERA_D_T_FMT`
```rust
const ERA_D_T_FMT: crate::nl_item = 131_120i32;
```

### `ERA_T_FMT`
```rust
const ERA_T_FMT: crate::nl_item = 131_121i32;
```

### `CODESET`
```rust
const CODESET: crate::nl_item = 14i32;
```

### `CRNCYSTR`
```rust
const CRNCYSTR: crate::nl_item = 262_159i32;
```

### `RADIXCHAR`
```rust
const RADIXCHAR: crate::nl_item = 65_536i32;
```

### `THOUSEP`
```rust
const THOUSEP: crate::nl_item = 65_537i32;
```

### `YESEXPR`
```rust
const YESEXPR: crate::nl_item = 327_680i32;
```

### `NOEXPR`
```rust
const NOEXPR: crate::nl_item = 327_681i32;
```

### `YESSTR`
```rust
const YESSTR: crate::nl_item = 327_682i32;
```

### `NOSTR`
```rust
const NOSTR: crate::nl_item = 327_683i32;
```

### `RUSAGE_CHILDREN`
```rust
const RUSAGE_CHILDREN: c_int = -1i32;
```

### `L_tmpnam`
```rust
const L_tmpnam: c_uint = 20u32;
```

### `_PC_LINK_MAX`
```rust
const _PC_LINK_MAX: c_int = 0i32;
```

### `_PC_MAX_CANON`
```rust
const _PC_MAX_CANON: c_int = 1i32;
```

### `_PC_MAX_INPUT`
```rust
const _PC_MAX_INPUT: c_int = 2i32;
```

### `_PC_NAME_MAX`
```rust
const _PC_NAME_MAX: c_int = 3i32;
```

### `_PC_PATH_MAX`
```rust
const _PC_PATH_MAX: c_int = 4i32;
```

### `_PC_PIPE_BUF`
```rust
const _PC_PIPE_BUF: c_int = 5i32;
```

### `_PC_CHOWN_RESTRICTED`
```rust
const _PC_CHOWN_RESTRICTED: c_int = 6i32;
```

### `_PC_NO_TRUNC`
```rust
const _PC_NO_TRUNC: c_int = 7i32;
```

### `_PC_VDISABLE`
```rust
const _PC_VDISABLE: c_int = 8i32;
```

### `_PC_SYNC_IO`
```rust
const _PC_SYNC_IO: c_int = 9i32;
```

### `_PC_ASYNC_IO`
```rust
const _PC_ASYNC_IO: c_int = 10i32;
```

### `_PC_PRIO_IO`
```rust
const _PC_PRIO_IO: c_int = 11i32;
```

### `_PC_SOCK_MAXBUF`
```rust
const _PC_SOCK_MAXBUF: c_int = 12i32;
```

### `_PC_FILESIZEBITS`
```rust
const _PC_FILESIZEBITS: c_int = 13i32;
```

### `_PC_REC_INCR_XFER_SIZE`
```rust
const _PC_REC_INCR_XFER_SIZE: c_int = 14i32;
```

### `_PC_REC_MAX_XFER_SIZE`
```rust
const _PC_REC_MAX_XFER_SIZE: c_int = 15i32;
```

### `_PC_REC_MIN_XFER_SIZE`
```rust
const _PC_REC_MIN_XFER_SIZE: c_int = 16i32;
```

### `_PC_REC_XFER_ALIGN`
```rust
const _PC_REC_XFER_ALIGN: c_int = 17i32;
```

### `_PC_ALLOC_SIZE_MIN`
```rust
const _PC_ALLOC_SIZE_MIN: c_int = 18i32;
```

### `_PC_SYMLINK_MAX`
```rust
const _PC_SYMLINK_MAX: c_int = 19i32;
```

### `_PC_2_SYMLINKS`
```rust
const _PC_2_SYMLINKS: c_int = 20i32;
```

### `_SC_ARG_MAX`
```rust
const _SC_ARG_MAX: c_int = 0i32;
```

### `_SC_CHILD_MAX`
```rust
const _SC_CHILD_MAX: c_int = 1i32;
```

### `_SC_CLK_TCK`
```rust
const _SC_CLK_TCK: c_int = 2i32;
```

### `_SC_NGROUPS_MAX`
```rust
const _SC_NGROUPS_MAX: c_int = 3i32;
```

### `_SC_OPEN_MAX`
```rust
const _SC_OPEN_MAX: c_int = 4i32;
```

### `_SC_STREAM_MAX`
```rust
const _SC_STREAM_MAX: c_int = 5i32;
```

### `_SC_TZNAME_MAX`
```rust
const _SC_TZNAME_MAX: c_int = 6i32;
```

### `_SC_JOB_CONTROL`
```rust
const _SC_JOB_CONTROL: c_int = 7i32;
```

### `_SC_SAVED_IDS`
```rust
const _SC_SAVED_IDS: c_int = 8i32;
```

### `_SC_REALTIME_SIGNALS`
```rust
const _SC_REALTIME_SIGNALS: c_int = 9i32;
```

### `_SC_PRIORITY_SCHEDULING`
```rust
const _SC_PRIORITY_SCHEDULING: c_int = 10i32;
```

### `_SC_TIMERS`
```rust
const _SC_TIMERS: c_int = 11i32;
```

### `_SC_ASYNCHRONOUS_IO`
```rust
const _SC_ASYNCHRONOUS_IO: c_int = 12i32;
```

### `_SC_PRIORITIZED_IO`
```rust
const _SC_PRIORITIZED_IO: c_int = 13i32;
```

### `_SC_SYNCHRONIZED_IO`
```rust
const _SC_SYNCHRONIZED_IO: c_int = 14i32;
```

### `_SC_FSYNC`
```rust
const _SC_FSYNC: c_int = 15i32;
```

### `_SC_MAPPED_FILES`
```rust
const _SC_MAPPED_FILES: c_int = 16i32;
```

### `_SC_MEMLOCK`
```rust
const _SC_MEMLOCK: c_int = 17i32;
```

### `_SC_MEMLOCK_RANGE`
```rust
const _SC_MEMLOCK_RANGE: c_int = 18i32;
```

### `_SC_MEMORY_PROTECTION`
```rust
const _SC_MEMORY_PROTECTION: c_int = 19i32;
```

### `_SC_MESSAGE_PASSING`
```rust
const _SC_MESSAGE_PASSING: c_int = 20i32;
```

### `_SC_SEMAPHORES`
```rust
const _SC_SEMAPHORES: c_int = 21i32;
```

### `_SC_SHARED_MEMORY_OBJECTS`
```rust
const _SC_SHARED_MEMORY_OBJECTS: c_int = 22i32;
```

### `_SC_AIO_LISTIO_MAX`
```rust
const _SC_AIO_LISTIO_MAX: c_int = 23i32;
```

### `_SC_AIO_MAX`
```rust
const _SC_AIO_MAX: c_int = 24i32;
```

### `_SC_AIO_PRIO_DELTA_MAX`
```rust
const _SC_AIO_PRIO_DELTA_MAX: c_int = 25i32;
```

### `_SC_DELAYTIMER_MAX`
```rust
const _SC_DELAYTIMER_MAX: c_int = 26i32;
```

### `_SC_MQ_OPEN_MAX`
```rust
const _SC_MQ_OPEN_MAX: c_int = 27i32;
```

### `_SC_MQ_PRIO_MAX`
```rust
const _SC_MQ_PRIO_MAX: c_int = 28i32;
```

### `_SC_VERSION`
```rust
const _SC_VERSION: c_int = 29i32;
```

### `_SC_PAGESIZE`
```rust
const _SC_PAGESIZE: c_int = 30i32;
```

### `_SC_PAGE_SIZE`
```rust
const _SC_PAGE_SIZE: c_int = 30i32;
```

### `_SC_RTSIG_MAX`
```rust
const _SC_RTSIG_MAX: c_int = 31i32;
```

### `_SC_SEM_NSEMS_MAX`
```rust
const _SC_SEM_NSEMS_MAX: c_int = 32i32;
```

### `_SC_SEM_VALUE_MAX`
```rust
const _SC_SEM_VALUE_MAX: c_int = 33i32;
```

### `_SC_SIGQUEUE_MAX`
```rust
const _SC_SIGQUEUE_MAX: c_int = 34i32;
```

### `_SC_TIMER_MAX`
```rust
const _SC_TIMER_MAX: c_int = 35i32;
```

### `_SC_BC_BASE_MAX`
```rust
const _SC_BC_BASE_MAX: c_int = 36i32;
```

### `_SC_BC_DIM_MAX`
```rust
const _SC_BC_DIM_MAX: c_int = 37i32;
```

### `_SC_BC_SCALE_MAX`
```rust
const _SC_BC_SCALE_MAX: c_int = 38i32;
```

### `_SC_BC_STRING_MAX`
```rust
const _SC_BC_STRING_MAX: c_int = 39i32;
```

### `_SC_COLL_WEIGHTS_MAX`
```rust
const _SC_COLL_WEIGHTS_MAX: c_int = 40i32;
```

### `_SC_EXPR_NEST_MAX`
```rust
const _SC_EXPR_NEST_MAX: c_int = 42i32;
```

### `_SC_LINE_MAX`
```rust
const _SC_LINE_MAX: c_int = 43i32;
```

### `_SC_RE_DUP_MAX`
```rust
const _SC_RE_DUP_MAX: c_int = 44i32;
```

### `_SC_2_VERSION`
```rust
const _SC_2_VERSION: c_int = 46i32;
```

### `_SC_2_C_BIND`
```rust
const _SC_2_C_BIND: c_int = 47i32;
```

### `_SC_2_C_DEV`
```rust
const _SC_2_C_DEV: c_int = 48i32;
```

### `_SC_2_FORT_DEV`
```rust
const _SC_2_FORT_DEV: c_int = 49i32;
```

### `_SC_2_FORT_RUN`
```rust
const _SC_2_FORT_RUN: c_int = 50i32;
```

### `_SC_2_SW_DEV`
```rust
const _SC_2_SW_DEV: c_int = 51i32;
```

### `_SC_2_LOCALEDEF`
```rust
const _SC_2_LOCALEDEF: c_int = 52i32;
```

### `_SC_UIO_MAXIOV`
```rust
const _SC_UIO_MAXIOV: c_int = 60i32;
```

### `_SC_IOV_MAX`
```rust
const _SC_IOV_MAX: c_int = 60i32;
```

### `_SC_THREADS`
```rust
const _SC_THREADS: c_int = 67i32;
```

### `_SC_THREAD_SAFE_FUNCTIONS`
```rust
const _SC_THREAD_SAFE_FUNCTIONS: c_int = 68i32;
```

### `_SC_GETGR_R_SIZE_MAX`
```rust
const _SC_GETGR_R_SIZE_MAX: c_int = 69i32;
```

### `_SC_GETPW_R_SIZE_MAX`
```rust
const _SC_GETPW_R_SIZE_MAX: c_int = 70i32;
```

### `_SC_LOGIN_NAME_MAX`
```rust
const _SC_LOGIN_NAME_MAX: c_int = 71i32;
```

### `_SC_TTY_NAME_MAX`
```rust
const _SC_TTY_NAME_MAX: c_int = 72i32;
```

### `_SC_THREAD_DESTRUCTOR_ITERATIONS`
```rust
const _SC_THREAD_DESTRUCTOR_ITERATIONS: c_int = 73i32;
```

### `_SC_THREAD_KEYS_MAX`
```rust
const _SC_THREAD_KEYS_MAX: c_int = 74i32;
```

### `_SC_THREAD_STACK_MIN`
```rust
const _SC_THREAD_STACK_MIN: c_int = 75i32;
```

### `_SC_THREAD_THREADS_MAX`
```rust
const _SC_THREAD_THREADS_MAX: c_int = 76i32;
```

### `_SC_THREAD_ATTR_STACKADDR`
```rust
const _SC_THREAD_ATTR_STACKADDR: c_int = 77i32;
```

### `_SC_THREAD_ATTR_STACKSIZE`
```rust
const _SC_THREAD_ATTR_STACKSIZE: c_int = 78i32;
```

### `_SC_THREAD_PRIORITY_SCHEDULING`
```rust
const _SC_THREAD_PRIORITY_SCHEDULING: c_int = 79i32;
```

### `_SC_THREAD_PRIO_INHERIT`
```rust
const _SC_THREAD_PRIO_INHERIT: c_int = 80i32;
```

### `_SC_THREAD_PRIO_PROTECT`
```rust
const _SC_THREAD_PRIO_PROTECT: c_int = 81i32;
```

### `_SC_THREAD_PROCESS_SHARED`
```rust
const _SC_THREAD_PROCESS_SHARED: c_int = 82i32;
```

### `_SC_NPROCESSORS_CONF`
```rust
const _SC_NPROCESSORS_CONF: c_int = 83i32;
```

### `_SC_NPROCESSORS_ONLN`
```rust
const _SC_NPROCESSORS_ONLN: c_int = 84i32;
```

### `_SC_PHYS_PAGES`
```rust
const _SC_PHYS_PAGES: c_int = 85i32;
```

### `_SC_AVPHYS_PAGES`
```rust
const _SC_AVPHYS_PAGES: c_int = 86i32;
```

### `_SC_ATEXIT_MAX`
```rust
const _SC_ATEXIT_MAX: c_int = 87i32;
```

### `_SC_PASS_MAX`
```rust
const _SC_PASS_MAX: c_int = 88i32;
```

### `_SC_XOPEN_VERSION`
```rust
const _SC_XOPEN_VERSION: c_int = 89i32;
```

### `_SC_XOPEN_XCU_VERSION`
```rust
const _SC_XOPEN_XCU_VERSION: c_int = 90i32;
```

### `_SC_XOPEN_UNIX`
```rust
const _SC_XOPEN_UNIX: c_int = 91i32;
```

### `_SC_XOPEN_CRYPT`
```rust
const _SC_XOPEN_CRYPT: c_int = 92i32;
```

### `_SC_XOPEN_ENH_I18N`
```rust
const _SC_XOPEN_ENH_I18N: c_int = 93i32;
```

### `_SC_XOPEN_SHM`
```rust
const _SC_XOPEN_SHM: c_int = 94i32;
```

### `_SC_2_CHAR_TERM`
```rust
const _SC_2_CHAR_TERM: c_int = 95i32;
```

### `_SC_2_UPE`
```rust
const _SC_2_UPE: c_int = 97i32;
```

### `_SC_XOPEN_XPG2`
```rust
const _SC_XOPEN_XPG2: c_int = 98i32;
```

### `_SC_XOPEN_XPG3`
```rust
const _SC_XOPEN_XPG3: c_int = 99i32;
```

### `_SC_XOPEN_XPG4`
```rust
const _SC_XOPEN_XPG4: c_int = 100i32;
```

### `_SC_NZERO`
```rust
const _SC_NZERO: c_int = 109i32;
```

### `_SC_XBS5_ILP32_OFF32`
```rust
const _SC_XBS5_ILP32_OFF32: c_int = 125i32;
```

### `_SC_XBS5_ILP32_OFFBIG`
```rust
const _SC_XBS5_ILP32_OFFBIG: c_int = 126i32;
```

### `_SC_XBS5_LP64_OFF64`
```rust
const _SC_XBS5_LP64_OFF64: c_int = 127i32;
```

### `_SC_XBS5_LPBIG_OFFBIG`
```rust
const _SC_XBS5_LPBIG_OFFBIG: c_int = 128i32;
```

### `_SC_XOPEN_LEGACY`
```rust
const _SC_XOPEN_LEGACY: c_int = 129i32;
```

### `_SC_XOPEN_REALTIME`
```rust
const _SC_XOPEN_REALTIME: c_int = 130i32;
```

### `_SC_XOPEN_REALTIME_THREADS`
```rust
const _SC_XOPEN_REALTIME_THREADS: c_int = 131i32;
```

### `_SC_ADVISORY_INFO`
```rust
const _SC_ADVISORY_INFO: c_int = 132i32;
```

### `_SC_BARRIERS`
```rust
const _SC_BARRIERS: c_int = 133i32;
```

### `_SC_CLOCK_SELECTION`
```rust
const _SC_CLOCK_SELECTION: c_int = 137i32;
```

### `_SC_CPUTIME`
```rust
const _SC_CPUTIME: c_int = 138i32;
```

### `_SC_THREAD_CPUTIME`
```rust
const _SC_THREAD_CPUTIME: c_int = 139i32;
```

### `_SC_MONOTONIC_CLOCK`
```rust
const _SC_MONOTONIC_CLOCK: c_int = 149i32;
```

### `_SC_READER_WRITER_LOCKS`
```rust
const _SC_READER_WRITER_LOCKS: c_int = 153i32;
```

### `_SC_SPIN_LOCKS`
```rust
const _SC_SPIN_LOCKS: c_int = 154i32;
```

### `_SC_REGEXP`
```rust
const _SC_REGEXP: c_int = 155i32;
```

### `_SC_SHELL`
```rust
const _SC_SHELL: c_int = 157i32;
```

### `_SC_SPAWN`
```rust
const _SC_SPAWN: c_int = 159i32;
```

### `_SC_SPORADIC_SERVER`
```rust
const _SC_SPORADIC_SERVER: c_int = 160i32;
```

### `_SC_THREAD_SPORADIC_SERVER`
```rust
const _SC_THREAD_SPORADIC_SERVER: c_int = 161i32;
```

### `_SC_TIMEOUTS`
```rust
const _SC_TIMEOUTS: c_int = 164i32;
```

### `_SC_TYPED_MEMORY_OBJECTS`
```rust
const _SC_TYPED_MEMORY_OBJECTS: c_int = 165i32;
```

### `_SC_2_PBS`
```rust
const _SC_2_PBS: c_int = 168i32;
```

### `_SC_2_PBS_ACCOUNTING`
```rust
const _SC_2_PBS_ACCOUNTING: c_int = 169i32;
```

### `_SC_2_PBS_LOCATE`
```rust
const _SC_2_PBS_LOCATE: c_int = 170i32;
```

### `_SC_2_PBS_MESSAGE`
```rust
const _SC_2_PBS_MESSAGE: c_int = 171i32;
```

### `_SC_2_PBS_TRACK`
```rust
const _SC_2_PBS_TRACK: c_int = 172i32;
```

### `_SC_SYMLOOP_MAX`
```rust
const _SC_SYMLOOP_MAX: c_int = 173i32;
```

### `_SC_STREAMS`
```rust
const _SC_STREAMS: c_int = 174i32;
```

### `_SC_2_PBS_CHECKPOINT`
```rust
const _SC_2_PBS_CHECKPOINT: c_int = 175i32;
```

### `_SC_V6_ILP32_OFF32`
```rust
const _SC_V6_ILP32_OFF32: c_int = 176i32;
```

### `_SC_V6_ILP32_OFFBIG`
```rust
const _SC_V6_ILP32_OFFBIG: c_int = 177i32;
```

### `_SC_V6_LP64_OFF64`
```rust
const _SC_V6_LP64_OFF64: c_int = 178i32;
```

### `_SC_V6_LPBIG_OFFBIG`
```rust
const _SC_V6_LPBIG_OFFBIG: c_int = 179i32;
```

### `_SC_HOST_NAME_MAX`
```rust
const _SC_HOST_NAME_MAX: c_int = 180i32;
```

### `_SC_TRACE`
```rust
const _SC_TRACE: c_int = 181i32;
```

### `_SC_TRACE_EVENT_FILTER`
```rust
const _SC_TRACE_EVENT_FILTER: c_int = 182i32;
```

### `_SC_TRACE_INHERIT`
```rust
const _SC_TRACE_INHERIT: c_int = 183i32;
```

### `_SC_TRACE_LOG`
```rust
const _SC_TRACE_LOG: c_int = 184i32;
```

### `_SC_IPV6`
```rust
const _SC_IPV6: c_int = 235i32;
```

### `_SC_RAW_SOCKETS`
```rust
const _SC_RAW_SOCKETS: c_int = 236i32;
```

### `_SC_V7_ILP32_OFF32`
```rust
const _SC_V7_ILP32_OFF32: c_int = 237i32;
```

### `_SC_V7_ILP32_OFFBIG`
```rust
const _SC_V7_ILP32_OFFBIG: c_int = 238i32;
```

### `_SC_V7_LP64_OFF64`
```rust
const _SC_V7_LP64_OFF64: c_int = 239i32;
```

### `_SC_V7_LPBIG_OFFBIG`
```rust
const _SC_V7_LPBIG_OFFBIG: c_int = 240i32;
```

### `_SC_SS_REPL_MAX`
```rust
const _SC_SS_REPL_MAX: c_int = 241i32;
```

### `_SC_TRACE_EVENT_NAME_MAX`
```rust
const _SC_TRACE_EVENT_NAME_MAX: c_int = 242i32;
```

### `_SC_TRACE_NAME_MAX`
```rust
const _SC_TRACE_NAME_MAX: c_int = 243i32;
```

### `_SC_TRACE_SYS_MAX`
```rust
const _SC_TRACE_SYS_MAX: c_int = 244i32;
```

### `_SC_TRACE_USER_EVENT_MAX`
```rust
const _SC_TRACE_USER_EVENT_MAX: c_int = 245i32;
```

### `_SC_XOPEN_STREAMS`
```rust
const _SC_XOPEN_STREAMS: c_int = 246i32;
```

### `_SC_THREAD_ROBUST_PRIO_INHERIT`
```rust
const _SC_THREAD_ROBUST_PRIO_INHERIT: c_int = 247i32;
```

### `_SC_THREAD_ROBUST_PRIO_PROTECT`
```rust
const _SC_THREAD_ROBUST_PRIO_PROTECT: c_int = 248i32;
```

### `_CS_PATH`
```rust
const _CS_PATH: c_int = 0i32;
```

### `_CS_POSIX_V6_WIDTH_RESTRICTED_ENVS`
```rust
const _CS_POSIX_V6_WIDTH_RESTRICTED_ENVS: c_int = 1i32;
```

### `_CS_POSIX_V5_WIDTH_RESTRICTED_ENVS`
```rust
const _CS_POSIX_V5_WIDTH_RESTRICTED_ENVS: c_int = 4i32;
```

### `_CS_POSIX_V7_WIDTH_RESTRICTED_ENVS`
```rust
const _CS_POSIX_V7_WIDTH_RESTRICTED_ENVS: c_int = 5i32;
```

### `_CS_POSIX_V6_ILP32_OFF32_CFLAGS`
```rust
const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: c_int = 1_116i32;
```

### `_CS_POSIX_V6_ILP32_OFF32_LDFLAGS`
```rust
const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: c_int = 1_117i32;
```

### `_CS_POSIX_V6_ILP32_OFF32_LIBS`
```rust
const _CS_POSIX_V6_ILP32_OFF32_LIBS: c_int = 1_118i32;
```

### `_CS_POSIX_V6_ILP32_OFF32_LINTFLAGS`
```rust
const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: c_int = 1_119i32;
```

### `_CS_POSIX_V6_ILP32_OFFBIG_CFLAGS`
```rust
const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: c_int = 1_120i32;
```

### `_CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS`
```rust
const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: c_int = 1_121i32;
```

### `_CS_POSIX_V6_ILP32_OFFBIG_LIBS`
```rust
const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: c_int = 1_122i32;
```

### `_CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS`
```rust
const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: c_int = 1_123i32;
```

### `_CS_POSIX_V6_LP64_OFF64_CFLAGS`
```rust
const _CS_POSIX_V6_LP64_OFF64_CFLAGS: c_int = 1_124i32;
```

### `_CS_POSIX_V6_LP64_OFF64_LDFLAGS`
```rust
const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: c_int = 1_125i32;
```

### `_CS_POSIX_V6_LP64_OFF64_LIBS`
```rust
const _CS_POSIX_V6_LP64_OFF64_LIBS: c_int = 1_126i32;
```

### `_CS_POSIX_V6_LP64_OFF64_LINTFLAGS`
```rust
const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: c_int = 1_127i32;
```

### `_CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS`
```rust
const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: c_int = 1_128i32;
```

### `_CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS`
```rust
const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: c_int = 1_129i32;
```

### `_CS_POSIX_V6_LPBIG_OFFBIG_LIBS`
```rust
const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: c_int = 1_130i32;
```

### `_CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS`
```rust
const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: c_int = 1_131i32;
```

### `_CS_POSIX_V7_ILP32_OFF32_CFLAGS`
```rust
const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: c_int = 1_132i32;
```

### `_CS_POSIX_V7_ILP32_OFF32_LDFLAGS`
```rust
const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: c_int = 1_133i32;
```

### `_CS_POSIX_V7_ILP32_OFF32_LIBS`
```rust
const _CS_POSIX_V7_ILP32_OFF32_LIBS: c_int = 1_134i32;
```

### `_CS_POSIX_V7_ILP32_OFF32_LINTFLAGS`
```rust
const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: c_int = 1_135i32;
```

### `_CS_POSIX_V7_ILP32_OFFBIG_CFLAGS`
```rust
const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: c_int = 1_136i32;
```

### `_CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS`
```rust
const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: c_int = 1_137i32;
```

### `_CS_POSIX_V7_ILP32_OFFBIG_LIBS`
```rust
const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: c_int = 1_138i32;
```

### `_CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS`
```rust
const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: c_int = 1_139i32;
```

### `_CS_POSIX_V7_LP64_OFF64_CFLAGS`
```rust
const _CS_POSIX_V7_LP64_OFF64_CFLAGS: c_int = 1_140i32;
```

### `_CS_POSIX_V7_LP64_OFF64_LDFLAGS`
```rust
const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: c_int = 1_141i32;
```

### `_CS_POSIX_V7_LP64_OFF64_LIBS`
```rust
const _CS_POSIX_V7_LP64_OFF64_LIBS: c_int = 1_142i32;
```

### `_CS_POSIX_V7_LP64_OFF64_LINTFLAGS`
```rust
const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: c_int = 1_143i32;
```

### `_CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS`
```rust
const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: c_int = 1_144i32;
```

### `_CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS`
```rust
const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: c_int = 1_145i32;
```

### `_CS_POSIX_V7_LPBIG_OFFBIG_LIBS`
```rust
const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: c_int = 1_146i32;
```

### `_CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS`
```rust
const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: c_int = 1_147i32;
```

### `RLIM_SAVED_MAX`
```rust
const RLIM_SAVED_MAX: crate::rlim_t = 18_446_744_073_709_551_615u64;
```

### `RLIM_SAVED_CUR`
```rust
const RLIM_SAVED_CUR: crate::rlim_t = 18_446_744_073_709_551_615u64;
```

### `EI_NIDENT`
```rust
const EI_NIDENT: usize = 16usize;
```

### `EI_MAG0`
```rust
const EI_MAG0: usize = 0usize;
```

### `ELFMAG0`
```rust
const ELFMAG0: u8 = 127u8;
```

### `EI_MAG1`
```rust
const EI_MAG1: usize = 1usize;
```

### `ELFMAG1`
```rust
const ELFMAG1: u8 = 69u8;
```

### `EI_MAG2`
```rust
const EI_MAG2: usize = 2usize;
```

### `ELFMAG2`
```rust
const ELFMAG2: u8 = 76u8;
```

### `EI_MAG3`
```rust
const EI_MAG3: usize = 3usize;
```

### `ELFMAG3`
```rust
const ELFMAG3: u8 = 70u8;
```

### `SELFMAG`
```rust
const SELFMAG: usize = 4usize;
```

### `EI_CLASS`
```rust
const EI_CLASS: usize = 4usize;
```

### `ELFCLASSNONE`
```rust
const ELFCLASSNONE: u8 = 0u8;
```

### `ELFCLASS32`
```rust
const ELFCLASS32: u8 = 1u8;
```

### `ELFCLASS64`
```rust
const ELFCLASS64: u8 = 2u8;
```

### `ELFCLASSNUM`
```rust
const ELFCLASSNUM: usize = 3usize;
```

### `EI_DATA`
```rust
const EI_DATA: usize = 5usize;
```

### `ELFDATANONE`
```rust
const ELFDATANONE: u8 = 0u8;
```

### `ELFDATA2LSB`
```rust
const ELFDATA2LSB: u8 = 1u8;
```

### `ELFDATA2MSB`
```rust
const ELFDATA2MSB: u8 = 2u8;
```

### `ELFDATANUM`
```rust
const ELFDATANUM: usize = 3usize;
```

### `EI_VERSION`
```rust
const EI_VERSION: usize = 6usize;
```

### `EI_OSABI`
```rust
const EI_OSABI: usize = 7usize;
```

### `ELFOSABI_NONE`
```rust
const ELFOSABI_NONE: u8 = 0u8;
```

### `ELFOSABI_SYSV`
```rust
const ELFOSABI_SYSV: u8 = 0u8;
```

### `ELFOSABI_HPUX`
```rust
const ELFOSABI_HPUX: u8 = 1u8;
```

### `ELFOSABI_NETBSD`
```rust
const ELFOSABI_NETBSD: u8 = 2u8;
```

### `ELFOSABI_GNU`
```rust
const ELFOSABI_GNU: u8 = 3u8;
```

### `ELFOSABI_LINUX`
```rust
const ELFOSABI_LINUX: u8 = 3u8;
```

### `ELFOSABI_SOLARIS`
```rust
const ELFOSABI_SOLARIS: u8 = 6u8;
```

### `ELFOSABI_AIX`
```rust
const ELFOSABI_AIX: u8 = 7u8;
```

### `ELFOSABI_IRIX`
```rust
const ELFOSABI_IRIX: u8 = 8u8;
```

### `ELFOSABI_FREEBSD`
```rust
const ELFOSABI_FREEBSD: u8 = 9u8;
```

### `ELFOSABI_TRU64`
```rust
const ELFOSABI_TRU64: u8 = 10u8;
```

### `ELFOSABI_MODESTO`
```rust
const ELFOSABI_MODESTO: u8 = 11u8;
```

### `ELFOSABI_OPENBSD`
```rust
const ELFOSABI_OPENBSD: u8 = 12u8;
```

### `ELFOSABI_ARM`
```rust
const ELFOSABI_ARM: u8 = 97u8;
```

### `ELFOSABI_STANDALONE`
```rust
const ELFOSABI_STANDALONE: u8 = 255u8;
```

### `EI_ABIVERSION`
```rust
const EI_ABIVERSION: usize = 8usize;
```

### `EI_PAD`
```rust
const EI_PAD: usize = 9usize;
```

### `ET_NONE`
```rust
const ET_NONE: u16 = 0u16;
```

### `ET_REL`
```rust
const ET_REL: u16 = 1u16;
```

### `ET_EXEC`
```rust
const ET_EXEC: u16 = 2u16;
```

### `ET_DYN`
```rust
const ET_DYN: u16 = 3u16;
```

### `ET_CORE`
```rust
const ET_CORE: u16 = 4u16;
```

### `ET_NUM`
```rust
const ET_NUM: u16 = 5u16;
```

### `ET_LOOS`
```rust
const ET_LOOS: u16 = 65_024u16;
```

### `ET_HIOS`
```rust
const ET_HIOS: u16 = 65_279u16;
```

### `ET_LOPROC`
```rust
const ET_LOPROC: u16 = 65_280u16;
```

### `ET_HIPROC`
```rust
const ET_HIPROC: u16 = 65_535u16;
```

### `EM_NONE`
```rust
const EM_NONE: u16 = 0u16;
```

### `EM_M32`
```rust
const EM_M32: u16 = 1u16;
```

### `EM_SPARC`
```rust
const EM_SPARC: u16 = 2u16;
```

### `EM_386`
```rust
const EM_386: u16 = 3u16;
```

### `EM_68K`
```rust
const EM_68K: u16 = 4u16;
```

### `EM_88K`
```rust
const EM_88K: u16 = 5u16;
```

### `EM_860`
```rust
const EM_860: u16 = 7u16;
```

### `EM_MIPS`
```rust
const EM_MIPS: u16 = 8u16;
```

### `EM_S370`
```rust
const EM_S370: u16 = 9u16;
```

### `EM_MIPS_RS3_LE`
```rust
const EM_MIPS_RS3_LE: u16 = 10u16;
```

### `EM_PARISC`
```rust
const EM_PARISC: u16 = 15u16;
```

### `EM_VPP500`
```rust
const EM_VPP500: u16 = 17u16;
```

### `EM_SPARC32PLUS`
```rust
const EM_SPARC32PLUS: u16 = 18u16;
```

### `EM_960`
```rust
const EM_960: u16 = 19u16;
```

### `EM_PPC`
```rust
const EM_PPC: u16 = 20u16;
```

### `EM_PPC64`
```rust
const EM_PPC64: u16 = 21u16;
```

### `EM_S390`
```rust
const EM_S390: u16 = 22u16;
```

### `EM_V800`
```rust
const EM_V800: u16 = 36u16;
```

### `EM_FR20`
```rust
const EM_FR20: u16 = 37u16;
```

### `EM_RH32`
```rust
const EM_RH32: u16 = 38u16;
```

### `EM_RCE`
```rust
const EM_RCE: u16 = 39u16;
```

### `EM_ARM`
```rust
const EM_ARM: u16 = 40u16;
```

### `EM_FAKE_ALPHA`
```rust
const EM_FAKE_ALPHA: u16 = 41u16;
```

### `EM_SH`
```rust
const EM_SH: u16 = 42u16;
```

### `EM_SPARCV9`
```rust
const EM_SPARCV9: u16 = 43u16;
```

### `EM_TRICORE`
```rust
const EM_TRICORE: u16 = 44u16;
```

### `EM_ARC`
```rust
const EM_ARC: u16 = 45u16;
```

### `EM_H8_300`
```rust
const EM_H8_300: u16 = 46u16;
```

### `EM_H8_300H`
```rust
const EM_H8_300H: u16 = 47u16;
```

### `EM_H8S`
```rust
const EM_H8S: u16 = 48u16;
```

### `EM_H8_500`
```rust
const EM_H8_500: u16 = 49u16;
```

### `EM_IA_64`
```rust
const EM_IA_64: u16 = 50u16;
```

### `EM_MIPS_X`
```rust
const EM_MIPS_X: u16 = 51u16;
```

### `EM_COLDFIRE`
```rust
const EM_COLDFIRE: u16 = 52u16;
```

### `EM_68HC12`
```rust
const EM_68HC12: u16 = 53u16;
```

### `EM_MMA`
```rust
const EM_MMA: u16 = 54u16;
```

### `EM_PCP`
```rust
const EM_PCP: u16 = 55u16;
```

### `EM_NCPU`
```rust
const EM_NCPU: u16 = 56u16;
```

### `EM_NDR1`
```rust
const EM_NDR1: u16 = 57u16;
```

### `EM_STARCORE`
```rust
const EM_STARCORE: u16 = 58u16;
```

### `EM_ME16`
```rust
const EM_ME16: u16 = 59u16;
```

### `EM_ST100`
```rust
const EM_ST100: u16 = 60u16;
```

### `EM_TINYJ`
```rust
const EM_TINYJ: u16 = 61u16;
```

### `EM_X86_64`
```rust
const EM_X86_64: u16 = 62u16;
```

### `EM_PDSP`
```rust
const EM_PDSP: u16 = 63u16;
```

### `EM_FX66`
```rust
const EM_FX66: u16 = 66u16;
```

### `EM_ST9PLUS`
```rust
const EM_ST9PLUS: u16 = 67u16;
```

### `EM_ST7`
```rust
const EM_ST7: u16 = 68u16;
```

### `EM_68HC16`
```rust
const EM_68HC16: u16 = 69u16;
```

### `EM_68HC11`
```rust
const EM_68HC11: u16 = 70u16;
```

### `EM_68HC08`
```rust
const EM_68HC08: u16 = 71u16;
```

### `EM_68HC05`
```rust
const EM_68HC05: u16 = 72u16;
```

### `EM_SVX`
```rust
const EM_SVX: u16 = 73u16;
```

### `EM_ST19`
```rust
const EM_ST19: u16 = 74u16;
```

### `EM_VAX`
```rust
const EM_VAX: u16 = 75u16;
```

### `EM_CRIS`
```rust
const EM_CRIS: u16 = 76u16;
```

### `EM_JAVELIN`
```rust
const EM_JAVELIN: u16 = 77u16;
```

### `EM_FIREPATH`
```rust
const EM_FIREPATH: u16 = 78u16;
```

### `EM_ZSP`
```rust
const EM_ZSP: u16 = 79u16;
```

### `EM_MMIX`
```rust
const EM_MMIX: u16 = 80u16;
```

### `EM_HUANY`
```rust
const EM_HUANY: u16 = 81u16;
```

### `EM_PRISM`
```rust
const EM_PRISM: u16 = 82u16;
```

### `EM_AVR`
```rust
const EM_AVR: u16 = 83u16;
```

### `EM_FR30`
```rust
const EM_FR30: u16 = 84u16;
```

### `EM_D10V`
```rust
const EM_D10V: u16 = 85u16;
```

### `EM_D30V`
```rust
const EM_D30V: u16 = 86u16;
```

### `EM_V850`
```rust
const EM_V850: u16 = 87u16;
```

### `EM_M32R`
```rust
const EM_M32R: u16 = 88u16;
```

### `EM_MN10300`
```rust
const EM_MN10300: u16 = 89u16;
```

### `EM_MN10200`
```rust
const EM_MN10200: u16 = 90u16;
```

### `EM_PJ`
```rust
const EM_PJ: u16 = 91u16;
```

### `EM_OPENRISC`
```rust
const EM_OPENRISC: u16 = 92u16;
```

### `EM_ARC_A5`
```rust
const EM_ARC_A5: u16 = 93u16;
```

### `EM_XTENSA`
```rust
const EM_XTENSA: u16 = 94u16;
```

### `EM_AARCH64`
```rust
const EM_AARCH64: u16 = 183u16;
```

### `EM_TILEPRO`
```rust
const EM_TILEPRO: u16 = 188u16;
```

### `EM_TILEGX`
```rust
const EM_TILEGX: u16 = 191u16;
```

### `EM_RISCV`
```rust
const EM_RISCV: u16 = 243u16;
```

### `EM_ALPHA`
```rust
const EM_ALPHA: u16 = 36_902u16;
```

### `EV_NONE`
```rust
const EV_NONE: u32 = 0u32;
```

### `EV_CURRENT`
```rust
const EV_CURRENT: u32 = 1u32;
```

### `EV_NUM`
```rust
const EV_NUM: u32 = 2u32;
```

### `PT_NULL`
```rust
const PT_NULL: u32 = 0u32;
```

### `PT_LOAD`
```rust
const PT_LOAD: u32 = 1u32;
```

### `PT_DYNAMIC`
```rust
const PT_DYNAMIC: u32 = 2u32;
```

### `PT_INTERP`
```rust
const PT_INTERP: u32 = 3u32;
```

### `PT_NOTE`
```rust
const PT_NOTE: u32 = 4u32;
```

### `PT_SHLIB`
```rust
const PT_SHLIB: u32 = 5u32;
```

### `PT_PHDR`
```rust
const PT_PHDR: u32 = 6u32;
```

### `PT_TLS`
```rust
const PT_TLS: u32 = 7u32;
```

### `PT_NUM`
```rust
const PT_NUM: u32 = 8u32;
```

### `PT_LOOS`
```rust
const PT_LOOS: u32 = 1_610_612_736u32;
```

### `PT_GNU_EH_FRAME`
```rust
const PT_GNU_EH_FRAME: u32 = 1_685_382_480u32;
```

### `PT_GNU_STACK`
```rust
const PT_GNU_STACK: u32 = 1_685_382_481u32;
```

### `PT_GNU_RELRO`
```rust
const PT_GNU_RELRO: u32 = 1_685_382_482u32;
```

### `PT_LOSUNW`
```rust
const PT_LOSUNW: u32 = 1_879_048_186u32;
```

### `PT_SUNWBSS`
```rust
const PT_SUNWBSS: u32 = 1_879_048_186u32;
```

### `PT_SUNWSTACK`
```rust
const PT_SUNWSTACK: u32 = 1_879_048_187u32;
```

### `PT_HISUNW`
```rust
const PT_HISUNW: u32 = 1_879_048_191u32;
```

### `PT_HIOS`
```rust
const PT_HIOS: u32 = 1_879_048_191u32;
```

### `PT_LOPROC`
```rust
const PT_LOPROC: u32 = 1_879_048_192u32;
```

### `PT_HIPROC`
```rust
const PT_HIPROC: u32 = 2_147_483_647u32;
```

### `PF_X`
```rust
const PF_X: u32 = 1u32;
```

### `PF_W`
```rust
const PF_W: u32 = 2u32;
```

### `PF_R`
```rust
const PF_R: u32 = 4u32;
```

### `PF_MASKOS`
```rust
const PF_MASKOS: u32 = 267_386_880u32;
```

### `PF_MASKPROC`
```rust
const PF_MASKPROC: u32 = 4_026_531_840u32;
```

### `AT_NULL`
```rust
const AT_NULL: c_ulong = 0u64;
```

### `AT_IGNORE`
```rust
const AT_IGNORE: c_ulong = 1u64;
```

### `AT_EXECFD`
```rust
const AT_EXECFD: c_ulong = 2u64;
```

### `AT_PHDR`
```rust
const AT_PHDR: c_ulong = 3u64;
```

### `AT_PHENT`
```rust
const AT_PHENT: c_ulong = 4u64;
```

### `AT_PHNUM`
```rust
const AT_PHNUM: c_ulong = 5u64;
```

### `AT_PAGESZ`
```rust
const AT_PAGESZ: c_ulong = 6u64;
```

### `AT_BASE`
```rust
const AT_BASE: c_ulong = 7u64;
```

### `AT_FLAGS`
```rust
const AT_FLAGS: c_ulong = 8u64;
```

### `AT_ENTRY`
```rust
const AT_ENTRY: c_ulong = 9u64;
```

### `AT_NOTELF`
```rust
const AT_NOTELF: c_ulong = 10u64;
```

### `AT_UID`
```rust
const AT_UID: c_ulong = 11u64;
```

### `AT_EUID`
```rust
const AT_EUID: c_ulong = 12u64;
```

### `AT_GID`
```rust
const AT_GID: c_ulong = 13u64;
```

### `AT_EGID`
```rust
const AT_EGID: c_ulong = 14u64;
```

### `AT_PLATFORM`
```rust
const AT_PLATFORM: c_ulong = 15u64;
```

### `AT_HWCAP`
```rust
const AT_HWCAP: c_ulong = 16u64;
```

### `AT_CLKTCK`
```rust
const AT_CLKTCK: c_ulong = 17u64;
```

### `AT_SECURE`
```rust
const AT_SECURE: c_ulong = 23u64;
```

### `AT_BASE_PLATFORM`
```rust
const AT_BASE_PLATFORM: c_ulong = 24u64;
```

### `AT_RANDOM`
```rust
const AT_RANDOM: c_ulong = 25u64;
```

### `AT_HWCAP2`
```rust
const AT_HWCAP2: c_ulong = 26u64;
```

### `AT_HWCAP3`
```rust
const AT_HWCAP3: c_ulong = 29u64;
```

### `AT_HWCAP4`
```rust
const AT_HWCAP4: c_ulong = 30u64;
```

### `AT_EXECFN`
```rust
const AT_EXECFN: c_ulong = 31u64;
```

### `AT_SYSINFO_EHDR`
```rust
const AT_SYSINFO_EHDR: c_ulong = 33u64;
```

### `AT_MINSIGSTKSZ`
```rust
const AT_MINSIGSTKSZ: c_ulong = 51u64;
```

### `GLOB_ERR`
```rust
const GLOB_ERR: c_int = 1i32;
```

### `GLOB_MARK`
```rust
const GLOB_MARK: c_int = 2i32;
```

### `GLOB_NOSORT`
```rust
const GLOB_NOSORT: c_int = 4i32;
```

### `GLOB_DOOFFS`
```rust
const GLOB_DOOFFS: c_int = 8i32;
```

### `GLOB_NOCHECK`
```rust
const GLOB_NOCHECK: c_int = 16i32;
```

### `GLOB_APPEND`
```rust
const GLOB_APPEND: c_int = 32i32;
```

### `GLOB_NOESCAPE`
```rust
const GLOB_NOESCAPE: c_int = 64i32;
```

### `GLOB_NOSPACE`
```rust
const GLOB_NOSPACE: c_int = 1i32;
```

### `GLOB_ABORTED`
```rust
const GLOB_ABORTED: c_int = 2i32;
```

### `GLOB_NOMATCH`
```rust
const GLOB_NOMATCH: c_int = 3i32;
```

### `POSIX_MADV_NORMAL`
```rust
const POSIX_MADV_NORMAL: c_int = 0i32;
```

### `POSIX_MADV_RANDOM`
```rust
const POSIX_MADV_RANDOM: c_int = 1i32;
```

### `POSIX_MADV_SEQUENTIAL`
```rust
const POSIX_MADV_SEQUENTIAL: c_int = 2i32;
```

### `POSIX_MADV_WILLNEED`
```rust
const POSIX_MADV_WILLNEED: c_int = 3i32;
```

### `POSIX_MADV_DONTNEED`
```rust
const POSIX_MADV_DONTNEED: c_int = 4i32;
```

### `S_IEXEC`
```rust
const S_IEXEC: crate::mode_t = 64u32;
```

### `S_IWRITE`
```rust
const S_IWRITE: crate::mode_t = 128u32;
```

### `S_IREAD`
```rust
const S_IREAD: crate::mode_t = 256u32;
```

### `F_LOCK`
```rust
const F_LOCK: c_int = 1i32;
```

### `F_TEST`
```rust
const F_TEST: c_int = 3i32;
```

### `F_TLOCK`
```rust
const F_TLOCK: c_int = 2i32;
```

### `F_ULOCK`
```rust
const F_ULOCK: c_int = 0i32;
```

### `ST_RDONLY`
```rust
const ST_RDONLY: c_ulong = 1u64;
```

### `ST_NOSUID`
```rust
const ST_NOSUID: c_ulong = 2u64;
```

### `ST_NODEV`
```rust
const ST_NODEV: c_ulong = 4u64;
```

### `ST_NOEXEC`
```rust
const ST_NOEXEC: c_ulong = 8u64;
```

### `ST_SYNCHRONOUS`
```rust
const ST_SYNCHRONOUS: c_ulong = 16u64;
```

### `ST_MANDLOCK`
```rust
const ST_MANDLOCK: c_ulong = 64u64;
```

### `ST_WRITE`
```rust
const ST_WRITE: c_ulong = 128u64;
```

### `ST_APPEND`
```rust
const ST_APPEND: c_ulong = 256u64;
```

### `ST_IMMUTABLE`
```rust
const ST_IMMUTABLE: c_ulong = 512u64;
```

### `ST_NOATIME`
```rust
const ST_NOATIME: c_ulong = 1_024u64;
```

### `ST_NODIRATIME`
```rust
const ST_NODIRATIME: c_ulong = 2_048u64;
```

### `RTLD_NEXT`
```rust
const RTLD_NEXT: *mut c_void = {0xffffffffffffffff as *mut core::ffi::c_void};
```

### `RTLD_DEFAULT`
```rust
const RTLD_DEFAULT: *mut c_void = {0x0 as *mut core::ffi::c_void};
```

### `RTLD_NODELETE`
```rust
const RTLD_NODELETE: c_int = 4_096i32;
```

### `RTLD_NOW`
```rust
const RTLD_NOW: c_int = 2i32;
```

### `AT_EACCESS`
```rust
const AT_EACCESS: c_int = 512i32;
```

### `PTHREAD_BARRIER_SERIAL_THREAD`
```rust
const PTHREAD_BARRIER_SERIAL_THREAD: c_int = -1i32;
```

### `PTHREAD_ONCE_INIT`
```rust
const PTHREAD_ONCE_INIT: crate::pthread_once_t = 0i32;
```

### `PTHREAD_MUTEX_NORMAL`
```rust
const PTHREAD_MUTEX_NORMAL: c_int = 0i32;
```

### `PTHREAD_MUTEX_RECURSIVE`
```rust
const PTHREAD_MUTEX_RECURSIVE: c_int = 1i32;
```

### `PTHREAD_MUTEX_ERRORCHECK`
```rust
const PTHREAD_MUTEX_ERRORCHECK: c_int = 2i32;
```

### `PTHREAD_MUTEX_DEFAULT`
```rust
const PTHREAD_MUTEX_DEFAULT: c_int = 0i32;
```

### `PTHREAD_MUTEX_STALLED`
```rust
const PTHREAD_MUTEX_STALLED: c_int = 0i32;
```

### `PTHREAD_MUTEX_ROBUST`
```rust
const PTHREAD_MUTEX_ROBUST: c_int = 1i32;
```

### `PTHREAD_PRIO_NONE`
```rust
const PTHREAD_PRIO_NONE: c_int = 0i32;
```

### `PTHREAD_PRIO_INHERIT`
```rust
const PTHREAD_PRIO_INHERIT: c_int = 1i32;
```

### `PTHREAD_PRIO_PROTECT`
```rust
const PTHREAD_PRIO_PROTECT: c_int = 2i32;
```

### `PTHREAD_PROCESS_PRIVATE`
```rust
const PTHREAD_PROCESS_PRIVATE: c_int = 0i32;
```

### `PTHREAD_PROCESS_SHARED`
```rust
const PTHREAD_PROCESS_SHARED: c_int = 1i32;
```

### `PTHREAD_INHERIT_SCHED`
```rust
const PTHREAD_INHERIT_SCHED: c_int = 0i32;
```

### `PTHREAD_EXPLICIT_SCHED`
```rust
const PTHREAD_EXPLICIT_SCHED: c_int = 1i32;
```

### `__SIZEOF_PTHREAD_COND_T`
```rust
const __SIZEOF_PTHREAD_COND_T: usize = 48usize;
```

### `IPPROTO_MAX`
```rust
const IPPROTO_MAX: c_int = 256i32;
```

### `IPC_PRIVATE`
```rust
const IPC_PRIVATE: crate::key_t = 0i32;
```

### `IPC_CREAT`
```rust
const IPC_CREAT: c_int = 512i32;
```

### `IPC_EXCL`
```rust
const IPC_EXCL: c_int = 1_024i32;
```

### `IPC_NOWAIT`
```rust
const IPC_NOWAIT: c_int = 2_048i32;
```

### `IPC_RMID`
```rust
const IPC_RMID: c_int = 0i32;
```

### `IPC_SET`
```rust
const IPC_SET: c_int = 1i32;
```

### `IPC_STAT`
```rust
const IPC_STAT: c_int = 2i32;
```

### `IPC_INFO`
```rust
const IPC_INFO: c_int = 3i32;
```

### `SHM_R`
```rust
const SHM_R: c_int = 256i32;
```

### `SHM_W`
```rust
const SHM_W: c_int = 128i32;
```

### `SHM_RDONLY`
```rust
const SHM_RDONLY: c_int = 4_096i32;
```

### `SHM_RND`
```rust
const SHM_RND: c_int = 8_192i32;
```

### `SHM_REMAP`
```rust
const SHM_REMAP: c_int = 16_384i32;
```

### `SHM_LOCK`
```rust
const SHM_LOCK: c_int = 11i32;
```

### `SHM_UNLOCK`
```rust
const SHM_UNLOCK: c_int = 12i32;
```

### `SHM_HUGETLB`
```rust
const SHM_HUGETLB: c_int = 2_048i32;
```

### `SHM_NORESERVE`
```rust
const SHM_NORESERVE: c_int = 4_096i32;
```

### `LOG_NFACILITIES`
```rust
const LOG_NFACILITIES: c_int = 24i32;
```

### `SEM_FAILED`
```rust
const SEM_FAILED: *mut crate::sem_t = {0x0 as *mut unix::linux_like::linux::gnu::sem_t};
```

### `AI_PASSIVE`
```rust
const AI_PASSIVE: c_int = 1i32;
```

### `AI_CANONNAME`
```rust
const AI_CANONNAME: c_int = 2i32;
```

### `AI_NUMERICHOST`
```rust
const AI_NUMERICHOST: c_int = 4i32;
```

### `AI_V4MAPPED`
```rust
const AI_V4MAPPED: c_int = 8i32;
```

### `AI_ALL`
```rust
const AI_ALL: c_int = 16i32;
```

### `AI_ADDRCONFIG`
```rust
const AI_ADDRCONFIG: c_int = 32i32;
```

### `AI_NUMERICSERV`
```rust
const AI_NUMERICSERV: c_int = 1_024i32;
```

### `EAI_BADFLAGS`
```rust
const EAI_BADFLAGS: c_int = -1i32;
```

### `EAI_NONAME`
```rust
const EAI_NONAME: c_int = -2i32;
```

### `EAI_AGAIN`
```rust
const EAI_AGAIN: c_int = -3i32;
```

### `EAI_FAIL`
```rust
const EAI_FAIL: c_int = -4i32;
```

### `EAI_NODATA`
```rust
const EAI_NODATA: c_int = -5i32;
```

### `EAI_FAMILY`
```rust
const EAI_FAMILY: c_int = -6i32;
```

### `EAI_SOCKTYPE`
```rust
const EAI_SOCKTYPE: c_int = -7i32;
```

### `EAI_SERVICE`
```rust
const EAI_SERVICE: c_int = -8i32;
```

### `EAI_MEMORY`
```rust
const EAI_MEMORY: c_int = -10i32;
```

### `EAI_SYSTEM`
```rust
const EAI_SYSTEM: c_int = -11i32;
```

### `EAI_OVERFLOW`
```rust
const EAI_OVERFLOW: c_int = -12i32;
```

### `NI_NUMERICHOST`
```rust
const NI_NUMERICHOST: c_int = 1i32;
```

### `NI_NUMERICSERV`
```rust
const NI_NUMERICSERV: c_int = 2i32;
```

### `NI_NOFQDN`
```rust
const NI_NOFQDN: c_int = 4i32;
```

### `NI_NAMEREQD`
```rust
const NI_NAMEREQD: c_int = 8i32;
```

### `NI_DGRAM`
```rust
const NI_DGRAM: c_int = 16i32;
```

### `NI_IDN`
```rust
const NI_IDN: c_int = 32i32;
```

### `AIO_CANCELED`
```rust
const AIO_CANCELED: c_int = 0i32;
```

### `AIO_NOTCANCELED`
```rust
const AIO_NOTCANCELED: c_int = 1i32;
```

### `AIO_ALLDONE`
```rust
const AIO_ALLDONE: c_int = 2i32;
```

### `LIO_READ`
```rust
const LIO_READ: c_int = 0i32;
```

### `LIO_WRITE`
```rust
const LIO_WRITE: c_int = 1i32;
```

### `LIO_NOP`
```rust
const LIO_NOP: c_int = 2i32;
```

### `LIO_WAIT`
```rust
const LIO_WAIT: c_int = 0i32;
```

### `LIO_NOWAIT`
```rust
const LIO_NOWAIT: c_int = 1i32;
```

### `RUSAGE_THREAD`
```rust
const RUSAGE_THREAD: c_int = 1i32;
```

### `MSG_COPY`
```rust
const MSG_COPY: c_int = 16_384i32;
```

### `SHM_EXEC`
```rust
const SHM_EXEC: c_int = 32_768i32;
```

### `IPV6_MULTICAST_ALL`
```rust
const IPV6_MULTICAST_ALL: c_int = 29i32;
```

### `IPV6_ROUTER_ALERT_ISOLATE`
```rust
const IPV6_ROUTER_ALERT_ISOLATE: c_int = 30i32;
```

### `PACKET_MR_UNICAST`
```rust
const PACKET_MR_UNICAST: c_int = 3i32;
```

### `PTRACE_EVENT_STOP`
```rust
const PTRACE_EVENT_STOP: c_int = 128i32;
```

### `UDP_SEGMENT`
```rust
const UDP_SEGMENT: c_int = 103i32;
```

### `UDP_GRO`
```rust
const UDP_GRO: c_int = 104i32;
```

### `PR_SET_PDEATHSIG`
```rust
const PR_SET_PDEATHSIG: c_int = 1i32;
```

### `PR_GET_PDEATHSIG`
```rust
const PR_GET_PDEATHSIG: c_int = 2i32;
```

### `PR_GET_DUMPABLE`
```rust
const PR_GET_DUMPABLE: c_int = 3i32;
```

### `PR_SET_DUMPABLE`
```rust
const PR_SET_DUMPABLE: c_int = 4i32;
```

### `PR_GET_UNALIGN`
```rust
const PR_GET_UNALIGN: c_int = 5i32;
```

### `PR_SET_UNALIGN`
```rust
const PR_SET_UNALIGN: c_int = 6i32;
```

### `PR_UNALIGN_NOPRINT`
```rust
const PR_UNALIGN_NOPRINT: c_int = 1i32;
```

### `PR_UNALIGN_SIGBUS`
```rust
const PR_UNALIGN_SIGBUS: c_int = 2i32;
```

### `PR_GET_KEEPCAPS`
```rust
const PR_GET_KEEPCAPS: c_int = 7i32;
```

### `PR_SET_KEEPCAPS`
```rust
const PR_SET_KEEPCAPS: c_int = 8i32;
```

### `PR_GET_FPEMU`
```rust
const PR_GET_FPEMU: c_int = 9i32;
```

### `PR_SET_FPEMU`
```rust
const PR_SET_FPEMU: c_int = 10i32;
```

### `PR_FPEMU_NOPRINT`
```rust
const PR_FPEMU_NOPRINT: c_int = 1i32;
```

### `PR_FPEMU_SIGFPE`
```rust
const PR_FPEMU_SIGFPE: c_int = 2i32;
```

### `PR_GET_FPEXC`
```rust
const PR_GET_FPEXC: c_int = 11i32;
```

### `PR_SET_FPEXC`
```rust
const PR_SET_FPEXC: c_int = 12i32;
```

### `PR_FP_EXC_SW_ENABLE`
```rust
const PR_FP_EXC_SW_ENABLE: c_int = 128i32;
```

### `PR_FP_EXC_DIV`
```rust
const PR_FP_EXC_DIV: c_int = 65_536i32;
```

### `PR_FP_EXC_OVF`
```rust
const PR_FP_EXC_OVF: c_int = 131_072i32;
```

### `PR_FP_EXC_UND`
```rust
const PR_FP_EXC_UND: c_int = 262_144i32;
```

### `PR_FP_EXC_RES`
```rust
const PR_FP_EXC_RES: c_int = 524_288i32;
```

### `PR_FP_EXC_INV`
```rust
const PR_FP_EXC_INV: c_int = 1_048_576i32;
```

### `PR_FP_EXC_DISABLED`
```rust
const PR_FP_EXC_DISABLED: c_int = 0i32;
```

### `PR_FP_EXC_NONRECOV`
```rust
const PR_FP_EXC_NONRECOV: c_int = 1i32;
```

### `PR_FP_EXC_ASYNC`
```rust
const PR_FP_EXC_ASYNC: c_int = 2i32;
```

### `PR_FP_EXC_PRECISE`
```rust
const PR_FP_EXC_PRECISE: c_int = 3i32;
```

### `PR_GET_TIMING`
```rust
const PR_GET_TIMING: c_int = 13i32;
```

### `PR_SET_TIMING`
```rust
const PR_SET_TIMING: c_int = 14i32;
```

### `PR_TIMING_STATISTICAL`
```rust
const PR_TIMING_STATISTICAL: c_int = 0i32;
```

### `PR_TIMING_TIMESTAMP`
```rust
const PR_TIMING_TIMESTAMP: c_int = 1i32;
```

### `PR_SET_NAME`
```rust
const PR_SET_NAME: c_int = 15i32;
```

### `PR_GET_NAME`
```rust
const PR_GET_NAME: c_int = 16i32;
```

### `PR_GET_ENDIAN`
```rust
const PR_GET_ENDIAN: c_int = 19i32;
```

### `PR_SET_ENDIAN`
```rust
const PR_SET_ENDIAN: c_int = 20i32;
```

### `PR_ENDIAN_BIG`
```rust
const PR_ENDIAN_BIG: c_int = 0i32;
```

### `PR_ENDIAN_LITTLE`
```rust
const PR_ENDIAN_LITTLE: c_int = 1i32;
```

### `PR_ENDIAN_PPC_LITTLE`
```rust
const PR_ENDIAN_PPC_LITTLE: c_int = 2i32;
```

### `PR_GET_SECCOMP`
```rust
const PR_GET_SECCOMP: c_int = 21i32;
```

### `PR_SET_SECCOMP`
```rust
const PR_SET_SECCOMP: c_int = 22i32;
```

### `PR_CAPBSET_READ`
```rust
const PR_CAPBSET_READ: c_int = 23i32;
```

### `PR_CAPBSET_DROP`
```rust
const PR_CAPBSET_DROP: c_int = 24i32;
```

### `PR_GET_TSC`
```rust
const PR_GET_TSC: c_int = 25i32;
```

### `PR_SET_TSC`
```rust
const PR_SET_TSC: c_int = 26i32;
```

### `PR_TSC_ENABLE`
```rust
const PR_TSC_ENABLE: c_int = 1i32;
```

### `PR_TSC_SIGSEGV`
```rust
const PR_TSC_SIGSEGV: c_int = 2i32;
```

### `PR_GET_SECUREBITS`
```rust
const PR_GET_SECUREBITS: c_int = 27i32;
```

### `PR_SET_SECUREBITS`
```rust
const PR_SET_SECUREBITS: c_int = 28i32;
```

### `PR_SET_TIMERSLACK`
```rust
const PR_SET_TIMERSLACK: c_int = 29i32;
```

### `PR_GET_TIMERSLACK`
```rust
const PR_GET_TIMERSLACK: c_int = 30i32;
```

### `PR_TASK_PERF_EVENTS_DISABLE`
```rust
const PR_TASK_PERF_EVENTS_DISABLE: c_int = 31i32;
```

### `PR_TASK_PERF_EVENTS_ENABLE`
```rust
const PR_TASK_PERF_EVENTS_ENABLE: c_int = 32i32;
```

### `PR_MCE_KILL`
```rust
const PR_MCE_KILL: c_int = 33i32;
```

### `PR_MCE_KILL_CLEAR`
```rust
const PR_MCE_KILL_CLEAR: c_int = 0i32;
```

### `PR_MCE_KILL_SET`
```rust
const PR_MCE_KILL_SET: c_int = 1i32;
```

### `PR_MCE_KILL_LATE`
```rust
const PR_MCE_KILL_LATE: c_int = 0i32;
```

### `PR_MCE_KILL_EARLY`
```rust
const PR_MCE_KILL_EARLY: c_int = 1i32;
```

### `PR_MCE_KILL_DEFAULT`
```rust
const PR_MCE_KILL_DEFAULT: c_int = 2i32;
```

### `PR_MCE_KILL_GET`
```rust
const PR_MCE_KILL_GET: c_int = 34i32;
```

### `PR_SET_MM`
```rust
const PR_SET_MM: c_int = 35i32;
```

### `PR_SET_MM_START_CODE`
```rust
const PR_SET_MM_START_CODE: c_int = 1i32;
```

### `PR_SET_MM_END_CODE`
```rust
const PR_SET_MM_END_CODE: c_int = 2i32;
```

### `PR_SET_MM_START_DATA`
```rust
const PR_SET_MM_START_DATA: c_int = 3i32;
```

### `PR_SET_MM_END_DATA`
```rust
const PR_SET_MM_END_DATA: c_int = 4i32;
```

### `PR_SET_MM_START_STACK`
```rust
const PR_SET_MM_START_STACK: c_int = 5i32;
```

### `PR_SET_MM_START_BRK`
```rust
const PR_SET_MM_START_BRK: c_int = 6i32;
```

### `PR_SET_MM_BRK`
```rust
const PR_SET_MM_BRK: c_int = 7i32;
```

### `PR_SET_MM_ARG_START`
```rust
const PR_SET_MM_ARG_START: c_int = 8i32;
```

### `PR_SET_MM_ARG_END`
```rust
const PR_SET_MM_ARG_END: c_int = 9i32;
```

### `PR_SET_MM_ENV_START`
```rust
const PR_SET_MM_ENV_START: c_int = 10i32;
```

### `PR_SET_MM_ENV_END`
```rust
const PR_SET_MM_ENV_END: c_int = 11i32;
```

### `PR_SET_MM_AUXV`
```rust
const PR_SET_MM_AUXV: c_int = 12i32;
```

### `PR_SET_MM_EXE_FILE`
```rust
const PR_SET_MM_EXE_FILE: c_int = 13i32;
```

### `PR_SET_MM_MAP`
```rust
const PR_SET_MM_MAP: c_int = 14i32;
```

### `PR_SET_MM_MAP_SIZE`
```rust
const PR_SET_MM_MAP_SIZE: c_int = 15i32;
```

### `PR_SET_PTRACER`
```rust
const PR_SET_PTRACER: c_int = 1_499_557_217i32;
```

### `PR_SET_PTRACER_ANY`
```rust
const PR_SET_PTRACER_ANY: c_ulong = 18_446_744_073_709_551_615u64;
```

### `PR_SET_CHILD_SUBREAPER`
```rust
const PR_SET_CHILD_SUBREAPER: c_int = 36i32;
```

### `PR_GET_CHILD_SUBREAPER`
```rust
const PR_GET_CHILD_SUBREAPER: c_int = 37i32;
```

### `PR_SET_NO_NEW_PRIVS`
```rust
const PR_SET_NO_NEW_PRIVS: c_int = 38i32;
```

### `PR_GET_NO_NEW_PRIVS`
```rust
const PR_GET_NO_NEW_PRIVS: c_int = 39i32;
```

### `PR_GET_TID_ADDRESS`
```rust
const PR_GET_TID_ADDRESS: c_int = 40i32;
```

### `PR_SET_THP_DISABLE`
```rust
const PR_SET_THP_DISABLE: c_int = 41i32;
```

### `PR_GET_THP_DISABLE`
```rust
const PR_GET_THP_DISABLE: c_int = 42i32;
```

### `PR_MPX_ENABLE_MANAGEMENT`
```rust
const PR_MPX_ENABLE_MANAGEMENT: c_int = 43i32;
```

### `PR_MPX_DISABLE_MANAGEMENT`
```rust
const PR_MPX_DISABLE_MANAGEMENT: c_int = 44i32;
```

### `PR_SET_FP_MODE`
```rust
const PR_SET_FP_MODE: c_int = 45i32;
```

### `PR_GET_FP_MODE`
```rust
const PR_GET_FP_MODE: c_int = 46i32;
```

### `PR_FP_MODE_FR`
```rust
const PR_FP_MODE_FR: c_int = 1i32;
```

### `PR_FP_MODE_FRE`
```rust
const PR_FP_MODE_FRE: c_int = 2i32;
```

### `PR_CAP_AMBIENT`
```rust
const PR_CAP_AMBIENT: c_int = 47i32;
```

### `PR_CAP_AMBIENT_IS_SET`
```rust
const PR_CAP_AMBIENT_IS_SET: c_int = 1i32;
```

### `PR_CAP_AMBIENT_RAISE`
```rust
const PR_CAP_AMBIENT_RAISE: c_int = 2i32;
```

### `PR_CAP_AMBIENT_LOWER`
```rust
const PR_CAP_AMBIENT_LOWER: c_int = 3i32;
```

### `PR_CAP_AMBIENT_CLEAR_ALL`
```rust
const PR_CAP_AMBIENT_CLEAR_ALL: c_int = 4i32;
```

### `PR_SET_VMA`
```rust
const PR_SET_VMA: c_int = 1_398_164_801i32;
```

### `PR_SET_VMA_ANON_NAME`
```rust
const PR_SET_VMA_ANON_NAME: c_int = 0i32;
```

### `PR_SCHED_CORE`
```rust
const PR_SCHED_CORE: c_int = 62i32;
```

### `PR_SCHED_CORE_GET`
```rust
const PR_SCHED_CORE_GET: c_int = 0i32;
```

### `PR_SCHED_CORE_CREATE`
```rust
const PR_SCHED_CORE_CREATE: c_int = 1i32;
```

### `PR_SCHED_CORE_SHARE_TO`
```rust
const PR_SCHED_CORE_SHARE_TO: c_int = 2i32;
```

### `PR_SCHED_CORE_SHARE_FROM`
```rust
const PR_SCHED_CORE_SHARE_FROM: c_int = 3i32;
```

### `PR_SCHED_CORE_MAX`
```rust
const PR_SCHED_CORE_MAX: c_int = 4i32;
```

### `PR_SCHED_CORE_SCOPE_THREAD`
```rust
const PR_SCHED_CORE_SCOPE_THREAD: c_int = 0i32;
```

### `PR_SCHED_CORE_SCOPE_THREAD_GROUP`
```rust
const PR_SCHED_CORE_SCOPE_THREAD_GROUP: c_int = 1i32;
```

### `PR_SCHED_CORE_SCOPE_PROCESS_GROUP`
```rust
const PR_SCHED_CORE_SCOPE_PROCESS_GROUP: c_int = 2i32;
```

### `ITIMER_REAL`
```rust
const ITIMER_REAL: c_int = 0i32;
```

### `ITIMER_VIRTUAL`
```rust
const ITIMER_VIRTUAL: c_int = 1i32;
```

### `ITIMER_PROF`
```rust
const ITIMER_PROF: c_int = 2i32;
```

### `_POSIX_VDISABLE`
```rust
const _POSIX_VDISABLE: crate::cc_t = 0u8;
```

### `IPV6_RTHDR_LOOSE`
```rust
const IPV6_RTHDR_LOOSE: c_int = 0i32;
```

### `IPV6_RTHDR_STRICT`
```rust
const IPV6_RTHDR_STRICT: c_int = 1i32;
```

### `IUTF8`
```rust
const IUTF8: crate::tcflag_t = 16_384u32;
```

### `CMSPAR`
```rust
const CMSPAR: crate::tcflag_t = 1_073_741_824u32;
```

### `MFD_CLOEXEC`
```rust
const MFD_CLOEXEC: c_uint = 1u32;
```

### `MFD_ALLOW_SEALING`
```rust
const MFD_ALLOW_SEALING: c_uint = 2u32;
```

### `MFD_HUGETLB`
```rust
const MFD_HUGETLB: c_uint = 4u32;
```

### `MFD_NOEXEC_SEAL`
```rust
const MFD_NOEXEC_SEAL: c_uint = 8u32;
```

### `MFD_EXEC`
```rust
const MFD_EXEC: c_uint = 16u32;
```

### `MFD_HUGE_64KB`
```rust
const MFD_HUGE_64KB: c_uint = 1_073_741_824u32;
```

### `MFD_HUGE_512KB`
```rust
const MFD_HUGE_512KB: c_uint = 1_275_068_416u32;
```

### `MFD_HUGE_1MB`
```rust
const MFD_HUGE_1MB: c_uint = 1_342_177_280u32;
```

### `MFD_HUGE_2MB`
```rust
const MFD_HUGE_2MB: c_uint = 1_409_286_144u32;
```

### `MFD_HUGE_8MB`
```rust
const MFD_HUGE_8MB: c_uint = 1_543_503_872u32;
```

### `MFD_HUGE_16MB`
```rust
const MFD_HUGE_16MB: c_uint = 1_610_612_736u32;
```

### `MFD_HUGE_32MB`
```rust
const MFD_HUGE_32MB: c_uint = 1_677_721_600u32;
```

### `MFD_HUGE_256MB`
```rust
const MFD_HUGE_256MB: c_uint = 1_879_048_192u32;
```

### `MFD_HUGE_512MB`
```rust
const MFD_HUGE_512MB: c_uint = 1_946_157_056u32;
```

### `MFD_HUGE_1GB`
```rust
const MFD_HUGE_1GB: c_uint = 2_013_265_920u32;
```

### `MFD_HUGE_2GB`
```rust
const MFD_HUGE_2GB: c_uint = 2_080_374_784u32;
```

### `MFD_HUGE_16GB`
```rust
const MFD_HUGE_16GB: c_uint = 2_281_701_376u32;
```

### `MFD_HUGE_MASK`
```rust
const MFD_HUGE_MASK: c_uint = 63u32;
```

### `MFD_HUGE_SHIFT`
```rust
const MFD_HUGE_SHIFT: c_uint = 26u32;
```

### `PACKET_HOST`
```rust
const PACKET_HOST: c_uchar = 0u8;
```

### `PACKET_BROADCAST`
```rust
const PACKET_BROADCAST: c_uchar = 1u8;
```

### `PACKET_MULTICAST`
```rust
const PACKET_MULTICAST: c_uchar = 2u8;
```

### `PACKET_OTHERHOST`
```rust
const PACKET_OTHERHOST: c_uchar = 3u8;
```

### `PACKET_OUTGOING`
```rust
const PACKET_OUTGOING: c_uchar = 4u8;
```

### `PACKET_LOOPBACK`
```rust
const PACKET_LOOPBACK: c_uchar = 5u8;
```

### `PACKET_USER`
```rust
const PACKET_USER: c_uchar = 6u8;
```

### `PACKET_KERNEL`
```rust
const PACKET_KERNEL: c_uchar = 7u8;
```

### `PACKET_ADD_MEMBERSHIP`
```rust
const PACKET_ADD_MEMBERSHIP: c_int = 1i32;
```

### `PACKET_DROP_MEMBERSHIP`
```rust
const PACKET_DROP_MEMBERSHIP: c_int = 2i32;
```

### `PACKET_RECV_OUTPUT`
```rust
const PACKET_RECV_OUTPUT: c_int = 3i32;
```

### `PACKET_RX_RING`
```rust
const PACKET_RX_RING: c_int = 5i32;
```

### `PACKET_STATISTICS`
```rust
const PACKET_STATISTICS: c_int = 6i32;
```

### `PACKET_COPY_THRESH`
```rust
const PACKET_COPY_THRESH: c_int = 7i32;
```

### `PACKET_AUXDATA`
```rust
const PACKET_AUXDATA: c_int = 8i32;
```

### `PACKET_ORIGDEV`
```rust
const PACKET_ORIGDEV: c_int = 9i32;
```

### `PACKET_VERSION`
```rust
const PACKET_VERSION: c_int = 10i32;
```

### `PACKET_HDRLEN`
```rust
const PACKET_HDRLEN: c_int = 11i32;
```

### `PACKET_RESERVE`
```rust
const PACKET_RESERVE: c_int = 12i32;
```

### `PACKET_TX_RING`
```rust
const PACKET_TX_RING: c_int = 13i32;
```

### `PACKET_LOSS`
```rust
const PACKET_LOSS: c_int = 14i32;
```

### `PACKET_VNET_HDR`
```rust
const PACKET_VNET_HDR: c_int = 15i32;
```

### `PACKET_TX_TIMESTAMP`
```rust
const PACKET_TX_TIMESTAMP: c_int = 16i32;
```

### `PACKET_TIMESTAMP`
```rust
const PACKET_TIMESTAMP: c_int = 17i32;
```

### `PACKET_MR_MULTICAST`
```rust
const PACKET_MR_MULTICAST: c_int = 0i32;
```

### `PACKET_MR_PROMISC`
```rust
const PACKET_MR_PROMISC: c_int = 1i32;
```

### `PACKET_MR_ALLMULTI`
```rust
const PACKET_MR_ALLMULTI: c_int = 2i32;
```

### `SIOCADDRT`
```rust
const SIOCADDRT: c_ulong = 35_083u64;
```

### `SIOCDELRT`
```rust
const SIOCDELRT: c_ulong = 35_084u64;
```

### `SIOCGIFNAME`
```rust
const SIOCGIFNAME: c_ulong = 35_088u64;
```

### `SIOCSIFLINK`
```rust
const SIOCSIFLINK: c_ulong = 35_089u64;
```

### `SIOCGIFCONF`
```rust
const SIOCGIFCONF: c_ulong = 35_090u64;
```

### `SIOCGIFFLAGS`
```rust
const SIOCGIFFLAGS: c_ulong = 35_091u64;
```

### `SIOCSIFFLAGS`
```rust
const SIOCSIFFLAGS: c_ulong = 35_092u64;
```

### `SIOCGIFADDR`
```rust
const SIOCGIFADDR: c_ulong = 35_093u64;
```

### `SIOCSIFADDR`
```rust
const SIOCSIFADDR: c_ulong = 35_094u64;
```

### `SIOCGIFDSTADDR`
```rust
const SIOCGIFDSTADDR: c_ulong = 35_095u64;
```

### `SIOCSIFDSTADDR`
```rust
const SIOCSIFDSTADDR: c_ulong = 35_096u64;
```

### `SIOCGIFBRDADDR`
```rust
const SIOCGIFBRDADDR: c_ulong = 35_097u64;
```

### `SIOCSIFBRDADDR`
```rust
const SIOCSIFBRDADDR: c_ulong = 35_098u64;
```

### `SIOCGIFNETMASK`
```rust
const SIOCGIFNETMASK: c_ulong = 35_099u64;
```

### `SIOCSIFNETMASK`
```rust
const SIOCSIFNETMASK: c_ulong = 35_100u64;
```

### `SIOCGIFMETRIC`
```rust
const SIOCGIFMETRIC: c_ulong = 35_101u64;
```

### `SIOCSIFMETRIC`
```rust
const SIOCSIFMETRIC: c_ulong = 35_102u64;
```

### `SIOCGIFMEM`
```rust
const SIOCGIFMEM: c_ulong = 35_103u64;
```

### `SIOCSIFMEM`
```rust
const SIOCSIFMEM: c_ulong = 35_104u64;
```

### `SIOCGIFMTU`
```rust
const SIOCGIFMTU: c_ulong = 35_105u64;
```

### `SIOCSIFMTU`
```rust
const SIOCSIFMTU: c_ulong = 35_106u64;
```

### `SIOCSIFNAME`
```rust
const SIOCSIFNAME: c_ulong = 35_107u64;
```

### `SIOCSIFHWADDR`
```rust
const SIOCSIFHWADDR: c_ulong = 35_108u64;
```

### `SIOCGIFENCAP`
```rust
const SIOCGIFENCAP: c_ulong = 35_109u64;
```

### `SIOCSIFENCAP`
```rust
const SIOCSIFENCAP: c_ulong = 35_110u64;
```

### `SIOCGIFHWADDR`
```rust
const SIOCGIFHWADDR: c_ulong = 35_111u64;
```

### `SIOCGIFSLAVE`
```rust
const SIOCGIFSLAVE: c_ulong = 35_113u64;
```

### `SIOCSIFSLAVE`
```rust
const SIOCSIFSLAVE: c_ulong = 35_120u64;
```

### `SIOCADDMULTI`
```rust
const SIOCADDMULTI: c_ulong = 35_121u64;
```

### `SIOCDELMULTI`
```rust
const SIOCDELMULTI: c_ulong = 35_122u64;
```

### `SIOCGIFINDEX`
```rust
const SIOCGIFINDEX: c_ulong = 35_123u64;
```

### `SIOGIFINDEX`
```rust
const SIOGIFINDEX: c_ulong = 35_123u64;
```

### `SIOCSIFPFLAGS`
```rust
const SIOCSIFPFLAGS: c_ulong = 35_124u64;
```

### `SIOCGIFPFLAGS`
```rust
const SIOCGIFPFLAGS: c_ulong = 35_125u64;
```

### `SIOCDIFADDR`
```rust
const SIOCDIFADDR: c_ulong = 35_126u64;
```

### `SIOCSIFHWBROADCAST`
```rust
const SIOCSIFHWBROADCAST: c_ulong = 35_127u64;
```

### `SIOCGIFCOUNT`
```rust
const SIOCGIFCOUNT: c_ulong = 35_128u64;
```

### `SIOCGIFBR`
```rust
const SIOCGIFBR: c_ulong = 35_136u64;
```

### `SIOCSIFBR`
```rust
const SIOCSIFBR: c_ulong = 35_137u64;
```

### `SIOCGIFTXQLEN`
```rust
const SIOCGIFTXQLEN: c_ulong = 35_138u64;
```

### `SIOCSIFTXQLEN`
```rust
const SIOCSIFTXQLEN: c_ulong = 35_139u64;
```

### `SIOCETHTOOL`
```rust
const SIOCETHTOOL: c_ulong = 35_142u64;
```

### `SIOCGMIIPHY`
```rust
const SIOCGMIIPHY: c_ulong = 35_143u64;
```

### `SIOCGMIIREG`
```rust
const SIOCGMIIREG: c_ulong = 35_144u64;
```

### `SIOCSMIIREG`
```rust
const SIOCSMIIREG: c_ulong = 35_145u64;
```

### `SIOCWANDEV`
```rust
const SIOCWANDEV: c_ulong = 35_146u64;
```

### `SIOCOUTQNSD`
```rust
const SIOCOUTQNSD: c_ulong = 35_147u64;
```

### `SIOCGSKNS`
```rust
const SIOCGSKNS: c_ulong = 35_148u64;
```

### `SIOCDARP`
```rust
const SIOCDARP: c_ulong = 35_155u64;
```

### `SIOCGARP`
```rust
const SIOCGARP: c_ulong = 35_156u64;
```

### `SIOCSARP`
```rust
const SIOCSARP: c_ulong = 35_157u64;
```

### `SIOCDRARP`
```rust
const SIOCDRARP: c_ulong = 35_168u64;
```

### `SIOCGRARP`
```rust
const SIOCGRARP: c_ulong = 35_169u64;
```

### `SIOCSRARP`
```rust
const SIOCSRARP: c_ulong = 35_170u64;
```

### `SIOCGIFMAP`
```rust
const SIOCGIFMAP: c_ulong = 35_184u64;
```

### `SIOCSIFMAP`
```rust
const SIOCSIFMAP: c_ulong = 35_185u64;
```

### `IPTOS_TOS_MASK`
```rust
const IPTOS_TOS_MASK: u8 = 30u8;
```

### `IPTOS_PREC_MASK`
```rust
const IPTOS_PREC_MASK: u8 = 224u8;
```

### `IPTOS_ECN_NOT_ECT`
```rust
const IPTOS_ECN_NOT_ECT: u8 = 0u8;
```

### `RTF_UP`
```rust
const RTF_UP: c_ushort = 1u16;
```

### `RTF_GATEWAY`
```rust
const RTF_GATEWAY: c_ushort = 2u16;
```

### `RTF_HOST`
```rust
const RTF_HOST: c_ushort = 4u16;
```

### `RTF_REINSTATE`
```rust
const RTF_REINSTATE: c_ushort = 8u16;
```

### `RTF_DYNAMIC`
```rust
const RTF_DYNAMIC: c_ushort = 16u16;
```

### `RTF_MODIFIED`
```rust
const RTF_MODIFIED: c_ushort = 32u16;
```

### `RTF_MTU`
```rust
const RTF_MTU: c_ushort = 64u16;
```

### `RTF_MSS`
```rust
const RTF_MSS: c_ushort = 64u16;
```

### `RTF_WINDOW`
```rust
const RTF_WINDOW: c_ushort = 128u16;
```

### `RTF_IRTT`
```rust
const RTF_IRTT: c_ushort = 256u16;
```

### `RTF_REJECT`
```rust
const RTF_REJECT: c_ushort = 512u16;
```

### `RTF_STATIC`
```rust
const RTF_STATIC: c_ushort = 1_024u16;
```

### `RTF_XRESOLVE`
```rust
const RTF_XRESOLVE: c_ushort = 2_048u16;
```

### `RTF_NOFORWARD`
```rust
const RTF_NOFORWARD: c_ushort = 4_096u16;
```

### `RTF_THROW`
```rust
const RTF_THROW: c_ushort = 8_192u16;
```

### `RTF_NOPMTUDISC`
```rust
const RTF_NOPMTUDISC: c_ushort = 16_384u16;
```

### `RTF_DEFAULT`
```rust
const RTF_DEFAULT: u32 = 65_536u32;
```

### `RTF_ALLONLINK`
```rust
const RTF_ALLONLINK: u32 = 131_072u32;
```

### `RTF_ADDRCONF`
```rust
const RTF_ADDRCONF: u32 = 262_144u32;
```

### `RTF_LINKRT`
```rust
const RTF_LINKRT: u32 = 1_048_576u32;
```

### `RTF_NONEXTHOP`
```rust
const RTF_NONEXTHOP: u32 = 2_097_152u32;
```

### `RTF_CACHE`
```rust
const RTF_CACHE: u32 = 16_777_216u32;
```

### `RTF_FLOW`
```rust
const RTF_FLOW: u32 = 33_554_432u32;
```

### `RTF_POLICY`
```rust
const RTF_POLICY: u32 = 67_108_864u32;
```

### `RTCF_VALVE`
```rust
const RTCF_VALVE: u32 = 2_097_152u32;
```

### `RTCF_MASQ`
```rust
const RTCF_MASQ: u32 = 4_194_304u32;
```

### `RTCF_NAT`
```rust
const RTCF_NAT: u32 = 8_388_608u32;
```

### `RTCF_DOREDIRECT`
```rust
const RTCF_DOREDIRECT: u32 = 16_777_216u32;
```

### `RTCF_LOG`
```rust
const RTCF_LOG: u32 = 33_554_432u32;
```

### `RTCF_DIRECTSRC`
```rust
const RTCF_DIRECTSRC: u32 = 67_108_864u32;
```

### `RTF_LOCAL`
```rust
const RTF_LOCAL: u32 = 2_147_483_648u32;
```

### `RTF_INTERFACE`
```rust
const RTF_INTERFACE: u32 = 1_073_741_824u32;
```

### `RTF_MULTICAST`
```rust
const RTF_MULTICAST: u32 = 536_870_912u32;
```

### `RTF_BROADCAST`
```rust
const RTF_BROADCAST: u32 = 268_435_456u32;
```

### `RTF_NAT`
```rust
const RTF_NAT: u32 = 134_217_728u32;
```

### `RTF_ADDRCLASSMASK`
```rust
const RTF_ADDRCLASSMASK: u32 = 4_160_749_568u32;
```

### `RT_CLASS_UNSPEC`
```rust
const RT_CLASS_UNSPEC: u8 = 0u8;
```

### `RT_CLASS_DEFAULT`
```rust
const RT_CLASS_DEFAULT: u8 = 253u8;
```

### `RT_CLASS_MAIN`
```rust
const RT_CLASS_MAIN: u8 = 254u8;
```

### `RT_CLASS_LOCAL`
```rust
const RT_CLASS_LOCAL: u8 = 255u8;
```

### `RT_CLASS_MAX`
```rust
const RT_CLASS_MAX: u8 = 255u8;
```

### `MAX_ADDR_LEN`
```rust
const MAX_ADDR_LEN: usize = 7usize;
```

### `ARPD_UPDATE`
```rust
const ARPD_UPDATE: c_ushort = 1u16;
```

### `ARPD_LOOKUP`
```rust
const ARPD_LOOKUP: c_ushort = 2u16;
```

### `ARPD_FLUSH`
```rust
const ARPD_FLUSH: c_ushort = 3u16;
```

### `ATF_MAGIC`
```rust
const ATF_MAGIC: c_int = 128i32;
```

### `UDP_CORK`
```rust
const UDP_CORK: c_int = 1i32;
```

### `UDP_ENCAP`
```rust
const UDP_ENCAP: c_int = 100i32;
```

### `UDP_NO_CHECK6_TX`
```rust
const UDP_NO_CHECK6_TX: c_int = 101i32;
```

### `UDP_NO_CHECK6_RX`
```rust
const UDP_NO_CHECK6_RX: c_int = 102i32;
```

### `MAP_FIXED_NOREPLACE`
```rust
const MAP_FIXED_NOREPLACE: c_int = 1_048_576i32;
```

### `MLOCK_ONFAULT`
```rust
const MLOCK_ONFAULT: c_uint = 1u32;
```

### `REG_EXTENDED`
```rust
const REG_EXTENDED: c_int = 1i32;
```

### `REG_ICASE`
```rust
const REG_ICASE: c_int = 2i32;
```

### `REG_NEWLINE`
```rust
const REG_NEWLINE: c_int = 4i32;
```

### `REG_NOSUB`
```rust
const REG_NOSUB: c_int = 8i32;
```

### `REG_NOTBOL`
```rust
const REG_NOTBOL: c_int = 1i32;
```

### `REG_NOTEOL`
```rust
const REG_NOTEOL: c_int = 2i32;
```

### `REG_ENOSYS`
```rust
const REG_ENOSYS: c_int = -1i32;
```

### `REG_NOMATCH`
```rust
const REG_NOMATCH: c_int = 1i32;
```

### `REG_BADPAT`
```rust
const REG_BADPAT: c_int = 2i32;
```

### `REG_ECOLLATE`
```rust
const REG_ECOLLATE: c_int = 3i32;
```

### `REG_ECTYPE`
```rust
const REG_ECTYPE: c_int = 4i32;
```

### `REG_EESCAPE`
```rust
const REG_EESCAPE: c_int = 5i32;
```

### `REG_ESUBREG`
```rust
const REG_ESUBREG: c_int = 6i32;
```

### `REG_EBRACK`
```rust
const REG_EBRACK: c_int = 7i32;
```

### `REG_EPAREN`
```rust
const REG_EPAREN: c_int = 8i32;
```

### `REG_EBRACE`
```rust
const REG_EBRACE: c_int = 9i32;
```

### `REG_BADBR`
```rust
const REG_BADBR: c_int = 10i32;
```

### `REG_ERANGE`
```rust
const REG_ERANGE: c_int = 11i32;
```

### `REG_ESPACE`
```rust
const REG_ESPACE: c_int = 12i32;
```

### `REG_BADRPT`
```rust
const REG_BADRPT: c_int = 13i32;
```

### `EPERM`
```rust
const EPERM: c_int = 1i32;
```

### `ENOENT`
```rust
const ENOENT: c_int = 2i32;
```

### `ESRCH`
```rust
const ESRCH: c_int = 3i32;
```

### `EINTR`
```rust
const EINTR: c_int = 4i32;
```

### `EIO`
```rust
const EIO: c_int = 5i32;
```

### `ENXIO`
```rust
const ENXIO: c_int = 6i32;
```

### `E2BIG`
```rust
const E2BIG: c_int = 7i32;
```

### `ENOEXEC`
```rust
const ENOEXEC: c_int = 8i32;
```

### `EBADF`
```rust
const EBADF: c_int = 9i32;
```

### `ECHILD`
```rust
const ECHILD: c_int = 10i32;
```

### `EAGAIN`
```rust
const EAGAIN: c_int = 11i32;
```

### `ENOMEM`
```rust
const ENOMEM: c_int = 12i32;
```

### `EACCES`
```rust
const EACCES: c_int = 13i32;
```

### `EFAULT`
```rust
const EFAULT: c_int = 14i32;
```

### `ENOTBLK`
```rust
const ENOTBLK: c_int = 15i32;
```

### `EBUSY`
```rust
const EBUSY: c_int = 16i32;
```

### `EEXIST`
```rust
const EEXIST: c_int = 17i32;
```

### `EXDEV`
```rust
const EXDEV: c_int = 18i32;
```

### `ENODEV`
```rust
const ENODEV: c_int = 19i32;
```

### `ENOTDIR`
```rust
const ENOTDIR: c_int = 20i32;
```

### `EISDIR`
```rust
const EISDIR: c_int = 21i32;
```

### `EINVAL`
```rust
const EINVAL: c_int = 22i32;
```

### `ENFILE`
```rust
const ENFILE: c_int = 23i32;
```

### `EMFILE`
```rust
const EMFILE: c_int = 24i32;
```

### `ENOTTY`
```rust
const ENOTTY: c_int = 25i32;
```

### `ETXTBSY`
```rust
const ETXTBSY: c_int = 26i32;
```

### `EFBIG`
```rust
const EFBIG: c_int = 27i32;
```

### `ENOSPC`
```rust
const ENOSPC: c_int = 28i32;
```

### `ESPIPE`
```rust
const ESPIPE: c_int = 29i32;
```

### `EROFS`
```rust
const EROFS: c_int = 30i32;
```

### `EMLINK`
```rust
const EMLINK: c_int = 31i32;
```

### `EPIPE`
```rust
const EPIPE: c_int = 32i32;
```

### `EDOM`
```rust
const EDOM: c_int = 33i32;
```

### `ERANGE`
```rust
const ERANGE: c_int = 34i32;
```

### `EWOULDBLOCK`
```rust
const EWOULDBLOCK: c_int = 11i32;
```

### `CSIGNAL`
```rust
const CSIGNAL: c_int = 255i32;
```

### `SCHED_NORMAL`
```rust
const SCHED_NORMAL: c_int = 0i32;
```

### `SCHED_OTHER`
```rust
const SCHED_OTHER: c_int = 0i32;
```

### `SCHED_FIFO`
```rust
const SCHED_FIFO: c_int = 1i32;
```

### `SCHED_RR`
```rust
const SCHED_RR: c_int = 2i32;
```

### `SCHED_BATCH`
```rust
const SCHED_BATCH: c_int = 3i32;
```

### `SCHED_IDLE`
```rust
const SCHED_IDLE: c_int = 5i32;
```

### `SCHED_DEADLINE`
```rust
const SCHED_DEADLINE: c_int = 6i32;
```

### `SCHED_RESET_ON_FORK`
```rust
const SCHED_RESET_ON_FORK: c_int = 1_073_741_824i32;
```

### `NT_PRSTATUS`
```rust
const NT_PRSTATUS: c_int = 1i32;
```

### `NT_PRFPREG`
```rust
const NT_PRFPREG: c_int = 2i32;
```

### `NT_FPREGSET`
```rust
const NT_FPREGSET: c_int = 2i32;
```

### `NT_PRPSINFO`
```rust
const NT_PRPSINFO: c_int = 3i32;
```

### `NT_PRXREG`
```rust
const NT_PRXREG: c_int = 4i32;
```

### `NT_TASKSTRUCT`
```rust
const NT_TASKSTRUCT: c_int = 4i32;
```

### `NT_PLATFORM`
```rust
const NT_PLATFORM: c_int = 5i32;
```

### `NT_AUXV`
```rust
const NT_AUXV: c_int = 6i32;
```

### `NT_GWINDOWS`
```rust
const NT_GWINDOWS: c_int = 7i32;
```

### `NT_ASRS`
```rust
const NT_ASRS: c_int = 8i32;
```

### `NT_PSTATUS`
```rust
const NT_PSTATUS: c_int = 10i32;
```

### `NT_PSINFO`
```rust
const NT_PSINFO: c_int = 13i32;
```

### `NT_PRCRED`
```rust
const NT_PRCRED: c_int = 14i32;
```

### `NT_UTSNAME`
```rust
const NT_UTSNAME: c_int = 15i32;
```

### `NT_LWPSTATUS`
```rust
const NT_LWPSTATUS: c_int = 16i32;
```

### `NT_LWPSINFO`
```rust
const NT_LWPSINFO: c_int = 17i32;
```

### `NT_PRFPXREG`
```rust
const NT_PRFPXREG: c_int = 20i32;
```

### `MS_NOUSER`
```rust
const MS_NOUSER: c_ulong = 18_446_744_071_562_067_968u64;
```

