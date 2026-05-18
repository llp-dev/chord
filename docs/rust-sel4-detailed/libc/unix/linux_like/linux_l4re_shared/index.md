*[libc](../../../index.md) / [unix](../../index.md) / [linux_like](../index.md) / [linux_l4re_shared](index.md)*

---

# Module `linux_l4re_shared`

## Contents

- [Structs](#structs)
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
- [Functions](#functions)
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

## Structs

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

- <span id="glob-t-clone"></span>`fn clone(&self) -> glob_t` — [`glob_t`](../index.md#glob-t)

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

- <span id="passwd-clone"></span>`fn clone(&self) -> passwd` — [`passwd`](../index.md#passwd)

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

- <span id="spwd-clone"></span>`fn clone(&self) -> spwd` — [`spwd`](../index.md#spwd)

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

- <span id="itimerspec-clone"></span>`fn clone(&self) -> itimerspec` — [`itimerspec`](../index.md#itimerspec)

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

- <span id="fsid-t-clone"></span>`fn clone(&self) -> fsid_t` — [`fsid_t`](../index.md#fsid-t)

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

- <span id="packet-mreq-clone"></span>`fn clone(&self) -> packet_mreq` — [`packet_mreq`](../index.md#packet-mreq)

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

- <span id="cpu-set-t-clone"></span>`fn clone(&self) -> cpu_set_t` — [`cpu_set_t`](../index.md#cpu-set-t)

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

- <span id="sembuf-clone"></span>`fn clone(&self) -> sembuf` — [`sembuf`](../index.md#sembuf)

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

- <span id="dl-phdr-info-clone"></span>`fn clone(&self) -> dl_phdr_info` — [`dl_phdr_info`](../index.md#dl-phdr-info)

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

- <span id="elf32-ehdr-clone"></span>`fn clone(&self) -> Elf32_Ehdr` — [`Elf32_Ehdr`](../index.md#elf32-ehdr)

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

- <span id="elf64-ehdr-clone"></span>`fn clone(&self) -> Elf64_Ehdr` — [`Elf64_Ehdr`](../index.md#elf64-ehdr)

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

- <span id="elf32-sym-clone"></span>`fn clone(&self) -> Elf32_Sym` — [`Elf32_Sym`](../index.md#elf32-sym)

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

- <span id="elf64-sym-clone"></span>`fn clone(&self) -> Elf64_Sym` — [`Elf64_Sym`](../index.md#elf64-sym)

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

- <span id="elf32-phdr-clone"></span>`fn clone(&self) -> Elf32_Phdr` — [`Elf32_Phdr`](../index.md#elf32-phdr)

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

- <span id="elf64-phdr-clone"></span>`fn clone(&self) -> Elf64_Phdr` — [`Elf64_Phdr`](../index.md#elf64-phdr)

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

- <span id="elf32-shdr-clone"></span>`fn clone(&self) -> Elf32_Shdr` — [`Elf32_Shdr`](../index.md#elf32-shdr)

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

- <span id="elf64-shdr-clone"></span>`fn clone(&self) -> Elf64_Shdr` — [`Elf64_Shdr`](../index.md#elf64-shdr)

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

- <span id="c-anonymous-elf32-rel-clone"></span>`fn clone(&self) -> __c_anonymous_elf32_rel` — [`__c_anonymous_elf32_rel`](../index.md#c-anonymous-elf32-rel)

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

- <span id="c-anonymous-elf64-rel-clone"></span>`fn clone(&self) -> __c_anonymous_elf64_rel` — [`__c_anonymous_elf64_rel`](../index.md#c-anonymous-elf64-rel)

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

- <span id="ucred-clone"></span>`fn clone(&self) -> ucred` — [`ucred`](../index.md#ucred)

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

- <span id="mntent-clone"></span>`fn clone(&self) -> mntent` — [`mntent`](../index.md#mntent)

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

- <span id="in6-pktinfo-clone"></span>`fn clone(&self) -> in6_pktinfo` — [`in6_pktinfo`](../index.md#in6-pktinfo)

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

- <span id="arpd-request-clone"></span>`fn clone(&self) -> arpd_request` — [`arpd_request`](../index.md#arpd-request)

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

- <span id="regmatch-t-clone"></span>`fn clone(&self) -> regmatch_t` — [`regmatch_t`](../index.md#regmatch-t)

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

- <span id="option-clone"></span>`fn clone(&self) -> option` — [`option`](../index.md#option)

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

- <span id="rlimit64-clone"></span>`fn clone(&self) -> rlimit64` — [`rlimit64`](../index.md#rlimit64)

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

- <span id="c-anonymous-ifru-map-clone"></span>`fn clone(&self) -> __c_anonymous_ifru_map` — [`__c_anonymous_ifru_map`](../index.md#c-anonymous-ifru-map)

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

- <span id="dirent-clone"></span>`fn clone(&self) -> dirent` — [`dirent`](../index.md#dirent)

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

- <span id="dirent64-clone"></span>`fn clone(&self) -> dirent64` — [`dirent64`](../index.md#dirent64)

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

- <span id="c-anonymous-elf32-rela-clone"></span>`fn clone(&self) -> __c_anonymous_elf32_rela` — [`__c_anonymous_elf32_rela`](../index.md#c-anonymous-elf32-rela)

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

- <span id="c-anonymous-elf64-rela-clone"></span>`fn clone(&self) -> __c_anonymous_elf64_rela` — [`__c_anonymous_elf64_rela`](../index.md#c-anonymous-elf64-rela)

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

- <span id="ifreq-clone"></span>`fn clone(&self) -> ifreq` — [`ifreq`](../index.md#ifreq)

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

- <span id="ifconf-clone"></span>`fn clone(&self) -> ifconf` — [`ifconf`](../index.md#ifconf)

##### `impl Copy for ifconf`

##### `impl Debug for ifconf`

- <span id="ifconf-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

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

