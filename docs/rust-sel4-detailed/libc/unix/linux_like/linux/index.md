*[libc](../../../index.md) / [unix](../../index.md) / [linux_like](../index.md) / [linux](index.md)*

---

# Module `linux`

Linux-specific definitions for linux-like values

## Contents

- [Modules](#modules)
  - [`gnu`](#gnu)
  - [`arch`](#arch)
  - [`b64`](#b64)
  - [`generic`](#generic)
- [Structs](#structs)
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
- [Enums](#enums)
  - [`tpacket_versions`](#tpacket-versions)
- [Functions](#functions)
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
  - [`pthread_t`](#pthread-t)
  - [`__priority_which_t`](#priority-which-t)
  - [`__rlimit_resource_t`](#rlimit-resource-t)
  - [`Lmid_t`](#lmid-t)
  - [`regoff_t`](#regoff-t)
  - [`__kernel_rwf_t`](#kernel-rwf-t)
  - [`Ioctl`](#ioctl)
- [Constants](#constants)
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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`gnu`](#gnu) | mod |  |
| [`arch`](#arch) | mod |  |
| [`b64`](#b64) | mod | 64-bit specific definitions for linux-like values |
| [`generic`](#generic) | mod |  |
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
| [`tpacket_versions`](#tpacket-versions) | enum |  |
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
| [`pthread_t`](#pthread-t) | type |  |
| [`__priority_which_t`](#priority-which-t) | type |  |
| [`__rlimit_resource_t`](#rlimit-resource-t) | type |  |
| [`Lmid_t`](#lmid-t) | type |  |
| [`regoff_t`](#regoff-t) | type |  |
| [`__kernel_rwf_t`](#kernel-rwf-t) | type |  |
| [`Ioctl`](#ioctl) | type |  |
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

## Modules

- [`gnu`](gnu/index.md)
- [`arch`](arch/index.md)
- [`b64`](b64/index.md) — 64-bit specific definitions for linux-like values
- [`generic`](generic/index.md)

## Structs

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

- <span id="dqblk-clone"></span>`fn clone(&self) -> dqblk` — [`dqblk`](../index.md#dqblk)

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

- <span id="signalfd-siginfo-clone"></span>`fn clone(&self) -> signalfd_siginfo` — [`signalfd_siginfo`](../index.md#signalfd-siginfo)

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

- <span id="fanout-args-clone"></span>`fn clone(&self) -> fanout_args` — [`fanout_args`](../index.md#fanout-args)

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

- <span id="sockaddr-pkt-clone"></span>`fn clone(&self) -> sockaddr_pkt` — [`sockaddr_pkt`](../index.md#sockaddr-pkt)

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

- <span id="tpacket-auxdata-clone"></span>`fn clone(&self) -> tpacket_auxdata` — [`tpacket_auxdata`](../index.md#tpacket-auxdata)

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

- <span id="tpacket-hdr-clone"></span>`fn clone(&self) -> tpacket_hdr` — [`tpacket_hdr`](../index.md#tpacket-hdr)

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

- <span id="tpacket-hdr-variant1-clone"></span>`fn clone(&self) -> tpacket_hdr_variant1` — [`tpacket_hdr_variant1`](../index.md#tpacket-hdr-variant1)

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

- <span id="tpacket2-hdr-clone"></span>`fn clone(&self) -> tpacket2_hdr` — [`tpacket2_hdr`](../index.md#tpacket2-hdr)

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

- <span id="tpacket-req-clone"></span>`fn clone(&self) -> tpacket_req` — [`tpacket_req`](../index.md#tpacket-req)

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

- <span id="tpacket-req3-clone"></span>`fn clone(&self) -> tpacket_req3` — [`tpacket_req3`](../index.md#tpacket-req3)

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

- <span id="tpacket-rollover-stats-clone"></span>`fn clone(&self) -> tpacket_rollover_stats` — [`tpacket_rollover_stats`](../index.md#tpacket-rollover-stats)

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

- <span id="tpacket-stats-clone"></span>`fn clone(&self) -> tpacket_stats` — [`tpacket_stats`](../index.md#tpacket-stats)

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

- <span id="tpacket-stats-v3-clone"></span>`fn clone(&self) -> tpacket_stats_v3` — [`tpacket_stats_v3`](../index.md#tpacket-stats-v3)

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

- <span id="tpacket3-hdr-clone"></span>`fn clone(&self) -> tpacket3_hdr` — [`tpacket3_hdr`](../index.md#tpacket3-hdr)

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

- <span id="tpacket-bd-ts-clone"></span>`fn clone(&self) -> tpacket_bd_ts` — [`tpacket_bd_ts`](../index.md#tpacket-bd-ts)

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

- <span id="tpacket-hdr-v1-clone"></span>`fn clone(&self) -> tpacket_hdr_v1` — [`tpacket_hdr_v1`](../index.md#tpacket-hdr-v1)

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

- <span id="msginfo-clone"></span>`fn clone(&self) -> msginfo` — [`msginfo`](../index.md#msginfo)

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

- <span id="input-event-clone"></span>`fn clone(&self) -> input_event` — [`input_event`](../index.md#input-event)

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

- <span id="input-id-clone"></span>`fn clone(&self) -> input_id` — [`input_id`](../index.md#input-id)

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

- <span id="input-absinfo-clone"></span>`fn clone(&self) -> input_absinfo` — [`input_absinfo`](../index.md#input-absinfo)

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

- <span id="input-keymap-entry-clone"></span>`fn clone(&self) -> input_keymap_entry` — [`input_keymap_entry`](../index.md#input-keymap-entry)

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

- <span id="input-mask-clone"></span>`fn clone(&self) -> input_mask` — [`input_mask`](../index.md#input-mask)

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

- <span id="ff-replay-clone"></span>`fn clone(&self) -> ff_replay` — [`ff_replay`](../index.md#ff-replay)

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

- <span id="ff-trigger-clone"></span>`fn clone(&self) -> ff_trigger` — [`ff_trigger`](../index.md#ff-trigger)

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

- <span id="ff-envelope-clone"></span>`fn clone(&self) -> ff_envelope` — [`ff_envelope`](../index.md#ff-envelope)

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

- <span id="ff-constant-effect-clone"></span>`fn clone(&self) -> ff_constant_effect` — [`ff_constant_effect`](../index.md#ff-constant-effect)

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

- <span id="ff-ramp-effect-clone"></span>`fn clone(&self) -> ff_ramp_effect` — [`ff_ramp_effect`](../index.md#ff-ramp-effect)

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

- <span id="ff-condition-effect-clone"></span>`fn clone(&self) -> ff_condition_effect` — [`ff_condition_effect`](../index.md#ff-condition-effect)

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

- <span id="ff-periodic-effect-clone"></span>`fn clone(&self) -> ff_periodic_effect` — [`ff_periodic_effect`](../index.md#ff-periodic-effect)

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

- <span id="ff-rumble-effect-clone"></span>`fn clone(&self) -> ff_rumble_effect` — [`ff_rumble_effect`](../index.md#ff-rumble-effect)

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

- <span id="ff-effect-clone"></span>`fn clone(&self) -> ff_effect` — [`ff_effect`](../index.md#ff-effect)

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

- <span id="uinput-ff-upload-clone"></span>`fn clone(&self) -> uinput_ff_upload` — [`uinput_ff_upload`](../index.md#uinput-ff-upload)

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

- <span id="uinput-ff-erase-clone"></span>`fn clone(&self) -> uinput_ff_erase` — [`uinput_ff_erase`](../index.md#uinput-ff-erase)

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

- <span id="uinput-abs-setup-clone"></span>`fn clone(&self) -> uinput_abs_setup` — [`uinput_abs_setup`](../index.md#uinput-abs-setup)

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

- <span id="c-anonymous-kernel-fsid-t-clone"></span>`fn clone(&self) -> __c_anonymous__kernel_fsid_t` — [`__c_anonymous__kernel_fsid_t`](../index.md#c-anonymous-kernel-fsid-t)

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

- <span id="posix-spawn-file-actions-t-clone"></span>`fn clone(&self) -> posix_spawn_file_actions_t` — [`posix_spawn_file_actions_t`](../index.md#posix-spawn-file-actions-t)

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

- <span id="posix-spawnattr-t-clone"></span>`fn clone(&self) -> posix_spawnattr_t` — [`posix_spawnattr_t`](../index.md#posix-spawnattr-t)

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

- <span id="genlmsghdr-clone"></span>`fn clone(&self) -> genlmsghdr` — [`genlmsghdr`](../index.md#genlmsghdr)

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

- <span id="inotify-event-clone"></span>`fn clone(&self) -> inotify_event` — [`inotify_event`](../index.md#inotify-event)

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

- <span id="fanotify-response-clone"></span>`fn clone(&self) -> fanotify_response` — [`fanotify_response`](../index.md#fanotify-response)

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

- <span id="fanotify-event-info-header-clone"></span>`fn clone(&self) -> fanotify_event_info_header` — [`fanotify_event_info_header`](../index.md#fanotify-event-info-header)

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

- <span id="fanotify-event-info-fid-clone"></span>`fn clone(&self) -> fanotify_event_info_fid` — [`fanotify_event_info_fid`](../index.md#fanotify-event-info-fid)

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

- <span id="sockaddr-vm-clone"></span>`fn clone(&self) -> sockaddr_vm` — [`sockaddr_vm`](../index.md#sockaddr-vm)

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

- <span id="sock-extended-err-clone"></span>`fn clone(&self) -> sock_extended_err` — [`sock_extended_err`](../index.md#sock-extended-err)

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

- <span id="seccomp-data-clone"></span>`fn clone(&self) -> seccomp_data` — [`seccomp_data`](../index.md#seccomp-data)

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

- <span id="seccomp-notif-sizes-clone"></span>`fn clone(&self) -> seccomp_notif_sizes` — [`seccomp_notif_sizes`](../index.md#seccomp-notif-sizes)

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

- <span id="seccomp-notif-clone"></span>`fn clone(&self) -> seccomp_notif` — [`seccomp_notif`](../index.md#seccomp-notif)

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

- <span id="seccomp-notif-resp-clone"></span>`fn clone(&self) -> seccomp_notif_resp` — [`seccomp_notif_resp`](../index.md#seccomp-notif-resp)

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

- <span id="seccomp-notif-addfd-clone"></span>`fn clone(&self) -> seccomp_notif_addfd` — [`seccomp_notif_addfd`](../index.md#seccomp-notif-addfd)

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

- <span id="in6-ifreq-clone"></span>`fn clone(&self) -> in6_ifreq` — [`in6_ifreq`](../index.md#in6-ifreq)

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

- <span id="open-how-clone"></span>`fn clone(&self) -> open_how` — [`open_how`](../index.md#open-how)

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

- <span id="ptp-clock-time-clone"></span>`fn clone(&self) -> ptp_clock_time` — [`ptp_clock_time`](../index.md#ptp-clock-time)

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

- <span id="ptp-extts-request-clone"></span>`fn clone(&self) -> ptp_extts_request` — [`ptp_extts_request`](../index.md#ptp-extts-request)

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

- <span id="ptp-sys-offset-extended-clone"></span>`fn clone(&self) -> ptp_sys_offset_extended` — [`ptp_sys_offset_extended`](../index.md#ptp-sys-offset-extended)

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

- <span id="ptp-sys-offset-precise-clone"></span>`fn clone(&self) -> ptp_sys_offset_precise` — [`ptp_sys_offset_precise`](../index.md#ptp-sys-offset-precise)

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

- <span id="ptp-extts-event-clone"></span>`fn clone(&self) -> ptp_extts_event` — [`ptp_extts_event`](../index.md#ptp-extts-event)

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

- <span id="sctp-initmsg-clone"></span>`fn clone(&self) -> sctp_initmsg` — [`sctp_initmsg`](../index.md#sctp-initmsg)

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

- <span id="sctp-sndrcvinfo-clone"></span>`fn clone(&self) -> sctp_sndrcvinfo` — [`sctp_sndrcvinfo`](../index.md#sctp-sndrcvinfo)

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

- <span id="sctp-sndinfo-clone"></span>`fn clone(&self) -> sctp_sndinfo` — [`sctp_sndinfo`](../index.md#sctp-sndinfo)

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

- <span id="sctp-rcvinfo-clone"></span>`fn clone(&self) -> sctp_rcvinfo` — [`sctp_rcvinfo`](../index.md#sctp-rcvinfo)

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

- <span id="sctp-nxtinfo-clone"></span>`fn clone(&self) -> sctp_nxtinfo` — [`sctp_nxtinfo`](../index.md#sctp-nxtinfo)

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

- <span id="sctp-prinfo-clone"></span>`fn clone(&self) -> sctp_prinfo` — [`sctp_prinfo`](../index.md#sctp-prinfo)

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

- <span id="sctp-authinfo-clone"></span>`fn clone(&self) -> sctp_authinfo` — [`sctp_authinfo`](../index.md#sctp-authinfo)

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

- <span id="tls-crypto-info-clone"></span>`fn clone(&self) -> tls_crypto_info` — [`tls_crypto_info`](../index.md#tls-crypto-info)

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

- <span id="tls12-crypto-info-aes-gcm-128-clone"></span>`fn clone(&self) -> tls12_crypto_info_aes_gcm_128` — [`tls12_crypto_info_aes_gcm_128`](../index.md#tls12-crypto-info-aes-gcm-128)

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

- <span id="tls12-crypto-info-aes-gcm-256-clone"></span>`fn clone(&self) -> tls12_crypto_info_aes_gcm_256` — [`tls12_crypto_info_aes_gcm_256`](../index.md#tls12-crypto-info-aes-gcm-256)

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

- <span id="tls12-crypto-info-aes-ccm-128-clone"></span>`fn clone(&self) -> tls12_crypto_info_aes_ccm_128` — [`tls12_crypto_info_aes_ccm_128`](../index.md#tls12-crypto-info-aes-ccm-128)

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

- <span id="tls12-crypto-info-chacha20-poly1305-clone"></span>`fn clone(&self) -> tls12_crypto_info_chacha20_poly1305` — [`tls12_crypto_info_chacha20_poly1305`](../index.md#tls12-crypto-info-chacha20-poly1305)

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

- <span id="tls12-crypto-info-sm4-gcm-clone"></span>`fn clone(&self) -> tls12_crypto_info_sm4_gcm` — [`tls12_crypto_info_sm4_gcm`](../index.md#tls12-crypto-info-sm4-gcm)

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

- <span id="tls12-crypto-info-sm4-ccm-clone"></span>`fn clone(&self) -> tls12_crypto_info_sm4_ccm` — [`tls12_crypto_info_sm4_ccm`](../index.md#tls12-crypto-info-sm4-ccm)

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

- <span id="tls12-crypto-info-aria-gcm-128-clone"></span>`fn clone(&self) -> tls12_crypto_info_aria_gcm_128` — [`tls12_crypto_info_aria_gcm_128`](../index.md#tls12-crypto-info-aria-gcm-128)

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

- <span id="tls12-crypto-info-aria-gcm-256-clone"></span>`fn clone(&self) -> tls12_crypto_info_aria_gcm_256` — [`tls12_crypto_info_aria_gcm_256`](../index.md#tls12-crypto-info-aria-gcm-256)

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

- <span id="iw-param-clone"></span>`fn clone(&self) -> iw_param` — [`iw_param`](../index.md#iw-param)

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

- <span id="iw-point-clone"></span>`fn clone(&self) -> iw_point` — [`iw_point`](../index.md#iw-point)

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

- <span id="iw-freq-clone"></span>`fn clone(&self) -> iw_freq` — [`iw_freq`](../index.md#iw-freq)

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

- <span id="iw-quality-clone"></span>`fn clone(&self) -> iw_quality` — [`iw_quality`](../index.md#iw-quality)

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

- <span id="iw-discarded-clone"></span>`fn clone(&self) -> iw_discarded` — [`iw_discarded`](../index.md#iw-discarded)

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

- <span id="iw-missed-clone"></span>`fn clone(&self) -> iw_missed` — [`iw_missed`](../index.md#iw-missed)

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

- <span id="iw-scan-req-clone"></span>`fn clone(&self) -> iw_scan_req` — [`iw_scan_req`](../index.md#iw-scan-req)

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

- <span id="iw-encode-ext-clone"></span>`fn clone(&self) -> iw_encode_ext` — [`iw_encode_ext`](../index.md#iw-encode-ext)

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

- <span id="iw-pmksa-clone"></span>`fn clone(&self) -> iw_pmksa` — [`iw_pmksa`](../index.md#iw-pmksa)

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

- <span id="iw-pmkid-cand-clone"></span>`fn clone(&self) -> iw_pmkid_cand` — [`iw_pmkid_cand`](../index.md#iw-pmkid-cand)

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

- <span id="iw-statistics-clone"></span>`fn clone(&self) -> iw_statistics` — [`iw_statistics`](../index.md#iw-statistics)

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

- <span id="iw-range-clone"></span>`fn clone(&self) -> iw_range` — [`iw_range`](../index.md#iw-range)

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

- <span id="iw-priv-args-clone"></span>`fn clone(&self) -> iw_priv_args` — [`iw_priv_args`](../index.md#iw-priv-args)

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

- <span id="epoll-params-clone"></span>`fn clone(&self) -> epoll_params` — [`epoll_params`](../index.md#epoll-params)

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

- <span id="pthread-mutexattr-t-clone"></span>`fn clone(&self) -> pthread_mutexattr_t` — [`pthread_mutexattr_t`](../index.md#pthread-mutexattr-t)

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

- <span id="pthread-rwlockattr-t-clone"></span>`fn clone(&self) -> pthread_rwlockattr_t` — [`pthread_rwlockattr_t`](../index.md#pthread-rwlockattr-t)

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

- <span id="pthread-condattr-t-clone"></span>`fn clone(&self) -> pthread_condattr_t` — [`pthread_condattr_t`](../index.md#pthread-condattr-t)

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

- <span id="pthread-barrierattr-t-clone"></span>`fn clone(&self) -> pthread_barrierattr_t` — [`pthread_barrierattr_t`](../index.md#pthread-barrierattr-t)

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

- <span id="fanotify-event-metadata-clone"></span>`fn clone(&self) -> fanotify_event_metadata` — [`fanotify_event_metadata`](../index.md#fanotify-event-metadata)

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

- <span id="ptp-sys-offset-clone"></span>`fn clone(&self) -> ptp_sys_offset` — [`ptp_sys_offset`](../index.md#ptp-sys-offset)

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

- <span id="ptp-pin-desc-clone"></span>`fn clone(&self) -> ptp_pin_desc` — [`ptp_pin_desc`](../index.md#ptp-pin-desc)

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

- <span id="ptp-clock-caps-clone"></span>`fn clone(&self) -> ptp_clock_caps` — [`ptp_clock_caps`](../index.md#ptp-clock-caps)

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

- <span id="sockaddr-xdp-clone"></span>`fn clone(&self) -> sockaddr_xdp` — [`sockaddr_xdp`](../index.md#sockaddr-xdp)

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

- <span id="xdp-ring-offset-clone"></span>`fn clone(&self) -> xdp_ring_offset` — [`xdp_ring_offset`](../index.md#xdp-ring-offset)

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

- <span id="xdp-mmap-offsets-clone"></span>`fn clone(&self) -> xdp_mmap_offsets` — [`xdp_mmap_offsets`](../index.md#xdp-mmap-offsets)

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

- <span id="xdp-ring-offset-v1-clone"></span>`fn clone(&self) -> xdp_ring_offset_v1` — [`xdp_ring_offset_v1`](../index.md#xdp-ring-offset-v1)

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

- <span id="xdp-mmap-offsets-v1-clone"></span>`fn clone(&self) -> xdp_mmap_offsets_v1` — [`xdp_mmap_offsets_v1`](../index.md#xdp-mmap-offsets-v1)

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

- <span id="xdp-umem-reg-clone"></span>`fn clone(&self) -> xdp_umem_reg` — [`xdp_umem_reg`](../index.md#xdp-umem-reg)

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

- <span id="xdp-umem-reg-v1-clone"></span>`fn clone(&self) -> xdp_umem_reg_v1` — [`xdp_umem_reg_v1`](../index.md#xdp-umem-reg-v1)

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

- <span id="xdp-statistics-clone"></span>`fn clone(&self) -> xdp_statistics` — [`xdp_statistics`](../index.md#xdp-statistics)

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

- <span id="xdp-statistics-v1-clone"></span>`fn clone(&self) -> xdp_statistics_v1` — [`xdp_statistics_v1`](../index.md#xdp-statistics-v1)

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

- <span id="xdp-options-clone"></span>`fn clone(&self) -> xdp_options` — [`xdp_options`](../index.md#xdp-options)

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

- <span id="xdp-desc-clone"></span>`fn clone(&self) -> xdp_desc` — [`xdp_desc`](../index.md#xdp-desc)

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

- <span id="xsk-tx-metadata-completion-clone"></span>`fn clone(&self) -> xsk_tx_metadata_completion` — [`xsk_tx_metadata_completion`](../index.md#xsk-tx-metadata-completion)

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

- <span id="xsk-tx-metadata-request-clone"></span>`fn clone(&self) -> xsk_tx_metadata_request` — [`xsk_tx_metadata_request`](../index.md#xsk-tx-metadata-request)

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

- <span id="mount-attr-clone"></span>`fn clone(&self) -> mount_attr` — [`mount_attr`](../index.md#mount-attr)

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

- <span id="mnt-ns-info-clone"></span>`fn clone(&self) -> mnt_ns_info` — [`mnt_ns_info`](../index.md#mnt-ns-info)

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

- <span id="dmabuf-cmsg-clone"></span>`fn clone(&self) -> dmabuf_cmsg` — [`dmabuf_cmsg`](../index.md#dmabuf-cmsg)

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

- <span id="dmabuf-token-clone"></span>`fn clone(&self) -> dmabuf_token` — [`dmabuf_token`](../index.md#dmabuf-token)

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

- <span id="sockaddr-alg-clone"></span>`fn clone(&self) -> sockaddr_alg` — [`sockaddr_alg`](../index.md#sockaddr-alg)

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

- <span id="pthread-cond-t-clone"></span>`fn clone(&self) -> pthread_cond_t` — [`pthread_cond_t`](../index.md#pthread-cond-t)

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

- <span id="pthread-mutex-t-clone"></span>`fn clone(&self) -> pthread_mutex_t` — [`pthread_mutex_t`](../index.md#pthread-mutex-t)

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

- <span id="pthread-rwlock-t-clone"></span>`fn clone(&self) -> pthread_rwlock_t` — [`pthread_rwlock_t`](../index.md#pthread-rwlock-t)

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

- <span id="pthread-barrier-t-clone"></span>`fn clone(&self) -> pthread_barrier_t` — [`pthread_barrier_t`](../index.md#pthread-barrier-t)

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

- <span id="uinput-setup-clone"></span>`fn clone(&self) -> uinput_setup` — [`uinput_setup`](../index.md#uinput-setup)

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

- <span id="uinput-user-dev-clone"></span>`fn clone(&self) -> uinput_user_dev` — [`uinput_user_dev`](../index.md#uinput-user-dev)

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

- <span id="mq-attr-clone"></span>`fn clone(&self) -> mq_attr` — [`mq_attr`](../index.md#mq-attr)

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

- <span id="hwtstamp-config-clone"></span>`fn clone(&self) -> hwtstamp_config` — [`hwtstamp_config`](../index.md#hwtstamp-config)

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

- <span id="sched-attr-clone"></span>`fn clone(&self) -> sched_attr` — [`sched_attr`](../index.md#sched-attr)

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

- <span id="file-handle-clone"></span>`fn clone(&self) -> file_handle` — [`file_handle`](../index.md#file-handle)

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

- <span id="iw-thrspy-clone"></span>`fn clone(&self) -> iw_thrspy` — [`iw_thrspy`](../index.md#iw-thrspy)

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

- <span id="iw-mlme-clone"></span>`fn clone(&self) -> iw_mlme` — [`iw_mlme`](../index.md#iw-mlme)

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

- <span id="iw-michaelmicfailure-clone"></span>`fn clone(&self) -> iw_michaelmicfailure` — [`iw_michaelmicfailure`](../index.md#iw-michaelmicfailure)

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

- <span id="af-alg-iv-clone"></span>`fn clone(&self) -> af_alg_iv` — [`af_alg_iv`](../index.md#af-alg-iv)

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

- <span id="tpacket-block-desc-clone"></span>`fn clone(&self) -> tpacket_block_desc` — [`tpacket_block_desc`](../index.md#tpacket-block-desc)

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

- <span id="sock-txtime-clone"></span>`fn clone(&self) -> sock_txtime` — [`sock_txtime`](../index.md#sock-txtime)

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

- <span id="iw-event-clone"></span>`fn clone(&self) -> iw_event` — [`iw_event`](../index.md#iw-event)

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

- <span id="iwreq-clone"></span>`fn clone(&self) -> iwreq` — [`iwreq`](../index.md#iwreq)

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

- <span id="ptp-perout-request-clone"></span>`fn clone(&self) -> ptp_perout_request` — [`ptp_perout_request`](../index.md#ptp-perout-request)

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

- <span id="xsk-tx-metadata-clone"></span>`fn clone(&self) -> xsk_tx_metadata` — [`xsk_tx_metadata`](../index.md#xsk-tx-metadata)

##### `impl Copy for xsk_tx_metadata`

##### `impl Debug for xsk_tx_metadata`

- <span id="xsk-tx-metadata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="aiocb-clone"></span>`fn clone(&self) -> aiocb` — [`aiocb`](#aiocb)

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

- <span id="exit-status-clone"></span>`fn clone(&self) -> __exit_status` — [`__exit_status`](#exit-status)

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

- <span id="timeval-clone"></span>`fn clone(&self) -> __timeval` — [`__timeval`](#timeval)

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

- <span id="glob64-t-clone"></span>`fn clone(&self) -> glob64_t` — [`glob64_t`](#glob64-t)

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

- <span id="msghdr-clone"></span>`fn clone(&self) -> msghdr` — [`msghdr`](#msghdr)

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

- <span id="cmsghdr-clone"></span>`fn clone(&self) -> cmsghdr` — [`cmsghdr`](#cmsghdr)

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

- <span id="termios-clone"></span>`fn clone(&self) -> termios` — [`termios`](#termios)

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

- <span id="mallinfo-clone"></span>`fn clone(&self) -> mallinfo` — [`mallinfo`](#mallinfo)

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

- <span id="mallinfo2-clone"></span>`fn clone(&self) -> mallinfo2` — [`mallinfo2`](#mallinfo2)

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

- <span id="ntptimeval-clone"></span>`fn clone(&self) -> ntptimeval` — [`ntptimeval`](#ntptimeval)

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

- <span id="regex-t-clone"></span>`fn clone(&self) -> regex_t` — [`regex_t`](#regex-t)

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

- <span id="elf64-chdr-clone"></span>`fn clone(&self) -> Elf64_Chdr` — [`Elf64_Chdr`](#elf64-chdr)

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

- <span id="elf32-chdr-clone"></span>`fn clone(&self) -> Elf32_Chdr` — [`Elf32_Chdr`](#elf32-chdr)

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

- <span id="seminfo-clone"></span>`fn clone(&self) -> seminfo` — [`seminfo`](#seminfo)

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

- <span id="ptrace-peeksiginfo-args-clone"></span>`fn clone(&self) -> ptrace_peeksiginfo_args` — [`ptrace_peeksiginfo_args`](#ptrace-peeksiginfo-args)

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

- <span id="c-anonymous-ptrace-syscall-info-entry-clone"></span>`fn clone(&self) -> __c_anonymous_ptrace_syscall_info_entry` — [`__c_anonymous_ptrace_syscall_info_entry`](#c-anonymous-ptrace-syscall-info-entry)

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

- <span id="c-anonymous-ptrace-syscall-info-exit-clone"></span>`fn clone(&self) -> __c_anonymous_ptrace_syscall_info_exit` — [`__c_anonymous_ptrace_syscall_info_exit`](#c-anonymous-ptrace-syscall-info-exit)

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

- <span id="c-anonymous-ptrace-syscall-info-seccomp-clone"></span>`fn clone(&self) -> __c_anonymous_ptrace_syscall_info_seccomp` — [`__c_anonymous_ptrace_syscall_info_seccomp`](#c-anonymous-ptrace-syscall-info-seccomp)

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

- <span id="ptrace-syscall-info-clone"></span>`fn clone(&self) -> ptrace_syscall_info` — [`ptrace_syscall_info`](#ptrace-syscall-info)

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

- <span id="ptrace-sud-config-clone"></span>`fn clone(&self) -> ptrace_sud_config` — [`ptrace_sud_config`](#ptrace-sud-config)

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

- <span id="iocb-clone"></span>`fn clone(&self) -> iocb` — [`iocb`](#iocb)

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

- <span id="tcp-info-clone"></span>`fn clone(&self) -> tcp_info` — [`tcp_info`](#tcp-info)

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

- <span id="fanotify-event-info-pidfd-clone"></span>`fn clone(&self) -> fanotify_event_info_pidfd` — [`fanotify_event_info_pidfd`](#fanotify-event-info-pidfd)

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

- <span id="fanotify-event-info-error-clone"></span>`fn clone(&self) -> fanotify_event_info_error` — [`fanotify_event_info_error`](#fanotify-event-info-error)

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

- <span id="sem-t-clone"></span>`fn clone(&self) -> sem_t` — [`sem_t`](#sem-t)

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

- <span id="mbstate-t-clone"></span>`fn clone(&self) -> mbstate_t` — [`mbstate_t`](#mbstate-t)

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

- <span id="fpos64-t-clone"></span>`fn clone(&self) -> fpos64_t` — [`fpos64_t`](#fpos64-t)

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

- <span id="fpos-t-clone"></span>`fn clone(&self) -> fpos_t` — [`fpos_t`](#fpos-t)

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

- <span id="timespec-clone"></span>`fn clone(&self) -> timespec` — [`timespec`](#timespec)

##### `impl Copy for timespec`

##### `impl Debug for timespec`

- <span id="timespec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for timespec`

- <span id="timespec-default"></span>`fn default() -> timespec` — [`timespec`](#timespec)

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

- <span id="utmpx-clone"></span>`fn clone(&self) -> utmpx` — [`utmpx`](#utmpx)

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

- <span id="sifields-sigchld-clone"></span>`fn clone(&self) -> sifields_sigchld` — [`sifields_sigchld`](gnu/index.md#sifields-sigchld)

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

- <span id="siginfo-f-clone"></span>`fn clone(&self) -> siginfo_f` — [`siginfo_f`](gnu/index.md#siginfo-f)

##### `impl Copy for siginfo_f`

##### `impl Debug for siginfo_f`

- <span id="siginfo-f-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

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

- <span id="tpacket-versions-clone"></span>`fn clone(&self) -> tpacket_versions` — [`tpacket_versions`](../index.md#tpacket-versions)

##### `impl Copy for tpacket_versions`

##### `impl Debug for tpacket_versions`

- <span id="tpacket-versions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

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

## Constants

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

