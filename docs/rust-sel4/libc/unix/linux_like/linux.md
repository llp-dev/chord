**libc > unix > linux_like > linux**

# Module: unix::linux_like::linux

## Contents

**Structs**

- [`__c_anonymous__kernel_fsid_t`](#__c_anonymous__kernel_fsid_t)
- [`af_alg_iv`](#af_alg_iv) - WARNING: The `PartialEq`, `Eq` and `Hash` implementations of this
- [`dmabuf_cmsg`](#dmabuf_cmsg)
- [`dmabuf_token`](#dmabuf_token)
- [`dqblk`](#dqblk)
- [`epoll_params`](#epoll_params)
- [`fanotify_event_info_fid`](#fanotify_event_info_fid)
- [`fanotify_event_info_header`](#fanotify_event_info_header)
- [`fanotify_event_metadata`](#fanotify_event_metadata)
- [`fanotify_response`](#fanotify_response)
- [`fanout_args`](#fanout_args)
- [`ff_condition_effect`](#ff_condition_effect)
- [`ff_constant_effect`](#ff_constant_effect)
- [`ff_effect`](#ff_effect)
- [`ff_envelope`](#ff_envelope)
- [`ff_periodic_effect`](#ff_periodic_effect)
- [`ff_ramp_effect`](#ff_ramp_effect)
- [`ff_replay`](#ff_replay)
- [`ff_rumble_effect`](#ff_rumble_effect)
- [`ff_trigger`](#ff_trigger)
- [`file_handle`](#file_handle)
- [`genlmsghdr`](#genlmsghdr)
- [`hwtstamp_config`](#hwtstamp_config)
- [`in6_ifreq`](#in6_ifreq)
- [`inotify_event`](#inotify_event)
- [`input_absinfo`](#input_absinfo)
- [`input_event`](#input_event)
- [`input_id`](#input_id)
- [`input_keymap_entry`](#input_keymap_entry)
- [`input_mask`](#input_mask)
- [`iw_discarded`](#iw_discarded)
- [`iw_encode_ext`](#iw_encode_ext)
- [`iw_event`](#iw_event)
- [`iw_freq`](#iw_freq)
- [`iw_michaelmicfailure`](#iw_michaelmicfailure)
- [`iw_missed`](#iw_missed)
- [`iw_mlme`](#iw_mlme)
- [`iw_param`](#iw_param)
- [`iw_pmkid_cand`](#iw_pmkid_cand)
- [`iw_pmksa`](#iw_pmksa)
- [`iw_point`](#iw_point)
- [`iw_priv_args`](#iw_priv_args)
- [`iw_quality`](#iw_quality)
- [`iw_range`](#iw_range)
- [`iw_scan_req`](#iw_scan_req)
- [`iw_statistics`](#iw_statistics)
- [`iw_thrspy`](#iw_thrspy)
- [`iwreq`](#iwreq)
- [`mnt_ns_info`](#mnt_ns_info)
- [`mount_attr`](#mount_attr)
- [`mq_attr`](#mq_attr)
- [`msginfo`](#msginfo)
- [`open_how`](#open_how)
- [`posix_spawn_file_actions_t`](#posix_spawn_file_actions_t)
- [`posix_spawnattr_t`](#posix_spawnattr_t)
- [`pthread_barrier_t`](#pthread_barrier_t)
- [`pthread_barrierattr_t`](#pthread_barrierattr_t)
- [`pthread_cond_t`](#pthread_cond_t)
- [`pthread_condattr_t`](#pthread_condattr_t)
- [`pthread_mutex_t`](#pthread_mutex_t)
- [`pthread_mutexattr_t`](#pthread_mutexattr_t)
- [`pthread_rwlock_t`](#pthread_rwlock_t)
- [`pthread_rwlockattr_t`](#pthread_rwlockattr_t)
- [`ptp_clock_caps`](#ptp_clock_caps)
- [`ptp_clock_time`](#ptp_clock_time)
- [`ptp_extts_event`](#ptp_extts_event)
- [`ptp_extts_request`](#ptp_extts_request)
- [`ptp_perout_request`](#ptp_perout_request)
- [`ptp_pin_desc`](#ptp_pin_desc)
- [`ptp_sys_offset`](#ptp_sys_offset)
- [`ptp_sys_offset_extended`](#ptp_sys_offset_extended)
- [`ptp_sys_offset_precise`](#ptp_sys_offset_precise)
- [`sched_attr`](#sched_attr)
- [`sctp_authinfo`](#sctp_authinfo)
- [`sctp_initmsg`](#sctp_initmsg)
- [`sctp_nxtinfo`](#sctp_nxtinfo)
- [`sctp_prinfo`](#sctp_prinfo)
- [`sctp_rcvinfo`](#sctp_rcvinfo)
- [`sctp_sndinfo`](#sctp_sndinfo)
- [`sctp_sndrcvinfo`](#sctp_sndrcvinfo)
- [`seccomp_data`](#seccomp_data)
- [`seccomp_notif`](#seccomp_notif)
- [`seccomp_notif_addfd`](#seccomp_notif_addfd)
- [`seccomp_notif_resp`](#seccomp_notif_resp)
- [`seccomp_notif_sizes`](#seccomp_notif_sizes)
- [`signalfd_siginfo`](#signalfd_siginfo)
- [`sock_extended_err`](#sock_extended_err)
- [`sock_txtime`](#sock_txtime)
- [`sockaddr_alg`](#sockaddr_alg)
- [`sockaddr_pkt`](#sockaddr_pkt)
- [`sockaddr_vm`](#sockaddr_vm)
- [`sockaddr_xdp`](#sockaddr_xdp)
- [`tls12_crypto_info_aes_ccm_128`](#tls12_crypto_info_aes_ccm_128)
- [`tls12_crypto_info_aes_gcm_128`](#tls12_crypto_info_aes_gcm_128)
- [`tls12_crypto_info_aes_gcm_256`](#tls12_crypto_info_aes_gcm_256)
- [`tls12_crypto_info_aria_gcm_128`](#tls12_crypto_info_aria_gcm_128)
- [`tls12_crypto_info_aria_gcm_256`](#tls12_crypto_info_aria_gcm_256)
- [`tls12_crypto_info_chacha20_poly1305`](#tls12_crypto_info_chacha20_poly1305)
- [`tls12_crypto_info_sm4_ccm`](#tls12_crypto_info_sm4_ccm)
- [`tls12_crypto_info_sm4_gcm`](#tls12_crypto_info_sm4_gcm)
- [`tls_crypto_info`](#tls_crypto_info)
- [`tpacket2_hdr`](#tpacket2_hdr)
- [`tpacket3_hdr`](#tpacket3_hdr)
- [`tpacket_auxdata`](#tpacket_auxdata)
- [`tpacket_bd_ts`](#tpacket_bd_ts)
- [`tpacket_block_desc`](#tpacket_block_desc)
- [`tpacket_hdr`](#tpacket_hdr)
- [`tpacket_hdr_v1`](#tpacket_hdr_v1)
- [`tpacket_hdr_variant1`](#tpacket_hdr_variant1)
- [`tpacket_req`](#tpacket_req)
- [`tpacket_req3`](#tpacket_req3)
- [`tpacket_rollover_stats`](#tpacket_rollover_stats)
- [`tpacket_stats`](#tpacket_stats)
- [`tpacket_stats_v3`](#tpacket_stats_v3)
- [`uinput_abs_setup`](#uinput_abs_setup)
- [`uinput_ff_erase`](#uinput_ff_erase)
- [`uinput_ff_upload`](#uinput_ff_upload)
- [`uinput_setup`](#uinput_setup)
- [`uinput_user_dev`](#uinput_user_dev)
- [`xdp_desc`](#xdp_desc)
- [`xdp_mmap_offsets`](#xdp_mmap_offsets)
- [`xdp_mmap_offsets_v1`](#xdp_mmap_offsets_v1)
- [`xdp_options`](#xdp_options)
- [`xdp_ring_offset`](#xdp_ring_offset)
- [`xdp_ring_offset_v1`](#xdp_ring_offset_v1)
- [`xdp_statistics`](#xdp_statistics)
- [`xdp_statistics_v1`](#xdp_statistics_v1)
- [`xdp_umem_reg`](#xdp_umem_reg)
- [`xdp_umem_reg_v1`](#xdp_umem_reg_v1)
- [`xsk_tx_metadata`](#xsk_tx_metadata)
- [`xsk_tx_metadata_completion`](#xsk_tx_metadata_completion)
- [`xsk_tx_metadata_request`](#xsk_tx_metadata_request)

**Unions**

- [`__c_anonymous_iwreq`](#__c_anonymous_iwreq)
- [`__c_anonymous_ptp_perout_request_1`](#__c_anonymous_ptp_perout_request_1)
- [`__c_anonymous_ptp_perout_request_2`](#__c_anonymous_ptp_perout_request_2)
- [`__c_anonymous_xsk_tx_metadata_union`](#__c_anonymous_xsk_tx_metadata_union)
- [`iwreq_data`](#iwreq_data)
- [`tpacket_bd_header_u`](#tpacket_bd_header_u)
- [`tpacket_req_u`](#tpacket_req_u)

**Enums**

- [`tpacket_versions`](#tpacket_versions)

**Functions**

- [`BPF_CLASS`](#bpf_class)
- [`BPF_JUMP`](#bpf_jump)
- [`BPF_MISCOP`](#bpf_miscop)
- [`BPF_MODE`](#bpf_mode)
- [`BPF_OP`](#bpf_op)
- [`BPF_RVAL`](#bpf_rval)
- [`BPF_SIZE`](#bpf_size)
- [`BPF_SRC`](#bpf_src)
- [`BPF_STMT`](#bpf_stmt)
- [`FUTEX_OP`](#futex_op)
- [`SCTP_PR_INDEX`](#sctp_pr_index)
- [`SCTP_PR_POLICY`](#sctp_pr_policy)
- [`SCTP_PR_PRIO_ENABLED`](#sctp_pr_prio_enabled)
- [`SCTP_PR_RTX_ENABLED`](#sctp_pr_rtx_enabled)
- [`SCTP_PR_SET_POLICY`](#sctp_pr_set_policy)
- [`SCTP_PR_TTL_ENABLED`](#sctp_pr_ttl_enabled)
- [`SO_EE_OFFENDER`](#so_ee_offender)
- [`SUN_LEN`](#sun_len)
- [`TPACKET_ALIGN`](#tpacket_align)
- [`accept4`](#accept4)
- [`clock_nanosleep`](#clock_nanosleep)
- [`clone`](#clone)
- [`dup3`](#dup3)
- [`epoll_create`](#epoll_create)
- [`epoll_create1`](#epoll_create1)
- [`epoll_ctl`](#epoll_ctl)
- [`epoll_pwait`](#epoll_pwait)
- [`epoll_wait`](#epoll_wait)
- [`eventfd`](#eventfd)
- [`eventfd_read`](#eventfd_read)
- [`eventfd_write`](#eventfd_write)
- [`fallocate`](#fallocate)
- [`fallocate64`](#fallocate64)
- [`fanotify_init`](#fanotify_init)
- [`fgetpos64`](#fgetpos64)
- [`fgetxattr`](#fgetxattr)
- [`flistxattr`](#flistxattr)
- [`fopen64`](#fopen64)
- [`fread_unlocked`](#fread_unlocked)
- [`fremovexattr`](#fremovexattr)
- [`fsetxattr`](#fsetxattr)
- [`ftok`](#ftok)
- [`getdtablesize`](#getdtablesize)
- [`getgrouplist`](#getgrouplist)
- [`gethostid`](#gethostid)
- [`getspnam_r`](#getspnam_r)
- [`getxattr`](#getxattr)
- [`inotify_add_watch`](#inotify_add_watch)
- [`inotify_init`](#inotify_init)
- [`inotify_init1`](#inotify_init1)
- [`inotify_rm_watch`](#inotify_rm_watch)
- [`klogctl`](#klogctl)
- [`lcong48`](#lcong48)
- [`lgetxattr`](#lgetxattr)
- [`listxattr`](#listxattr)
- [`llistxattr`](#llistxattr)
- [`lremovexattr`](#lremovexattr)
- [`lsetxattr`](#lsetxattr)
- [`lutimes`](#lutimes)
- [`mkfifoat`](#mkfifoat)
- [`mkstemps`](#mkstemps)
- [`mq_close`](#mq_close)
- [`mq_getattr`](#mq_getattr)
- [`mq_open`](#mq_open)
- [`mq_receive`](#mq_receive)
- [`mq_send`](#mq_send)
- [`mq_setattr`](#mq_setattr)
- [`mq_timedreceive`](#mq_timedreceive)
- [`mq_timedsend`](#mq_timedsend)
- [`mq_unlink`](#mq_unlink)
- [`mrand48`](#mrand48)
- [`msgctl`](#msgctl)
- [`msgget`](#msgget)
- [`msgrcv`](#msgrcv)
- [`msgsnd`](#msgsnd)
- [`name_to_handle_at`](#name_to_handle_at)
- [`open_by_handle_at`](#open_by_handle_at)
- [`personality`](#personality)
- [`posix_fallocate`](#posix_fallocate)
- [`posix_fallocate64`](#posix_fallocate64)
- [`posix_madvise`](#posix_madvise)
- [`posix_spawn`](#posix_spawn)
- [`posix_spawn_file_actions_addclose`](#posix_spawn_file_actions_addclose)
- [`posix_spawn_file_actions_adddup2`](#posix_spawn_file_actions_adddup2)
- [`posix_spawn_file_actions_addopen`](#posix_spawn_file_actions_addopen)
- [`posix_spawn_file_actions_destroy`](#posix_spawn_file_actions_destroy)
- [`posix_spawn_file_actions_init`](#posix_spawn_file_actions_init)
- [`posix_spawnattr_destroy`](#posix_spawnattr_destroy)
- [`posix_spawnattr_getflags`](#posix_spawnattr_getflags)
- [`posix_spawnattr_getpgroup`](#posix_spawnattr_getpgroup)
- [`posix_spawnattr_getschedparam`](#posix_spawnattr_getschedparam)
- [`posix_spawnattr_getschedpolicy`](#posix_spawnattr_getschedpolicy)
- [`posix_spawnattr_getsigdefault`](#posix_spawnattr_getsigdefault)
- [`posix_spawnattr_getsigmask`](#posix_spawnattr_getsigmask)
- [`posix_spawnattr_init`](#posix_spawnattr_init)
- [`posix_spawnattr_setflags`](#posix_spawnattr_setflags)
- [`posix_spawnattr_setpgroup`](#posix_spawnattr_setpgroup)
- [`posix_spawnattr_setschedparam`](#posix_spawnattr_setschedparam)
- [`posix_spawnattr_setschedpolicy`](#posix_spawnattr_setschedpolicy)
- [`posix_spawnattr_setsigdefault`](#posix_spawnattr_setsigdefault)
- [`posix_spawnattr_setsigmask`](#posix_spawnattr_setsigmask)
- [`posix_spawnp`](#posix_spawnp)
- [`quotactl`](#quotactl)
- [`readahead`](#readahead)
- [`reboot`](#reboot)
- [`remap_file_pages`](#remap_file_pages)
- [`removexattr`](#removexattr)
- [`sched_getparam`](#sched_getparam)
- [`sched_getscheduler`](#sched_getscheduler)
- [`sched_rr_get_interval`](#sched_rr_get_interval)
- [`sched_setaffinity`](#sched_setaffinity)
- [`sched_setparam`](#sched_setparam)
- [`sched_setscheduler`](#sched_setscheduler)
- [`seed48`](#seed48)
- [`semctl`](#semctl)
- [`semget`](#semget)
- [`semop`](#semop)
- [`sendfile`](#sendfile)
- [`sendfile64`](#sendfile64)
- [`setfsgid`](#setfsgid)
- [`setfsuid`](#setfsuid)
- [`setns`](#setns)
- [`setxattr`](#setxattr)
- [`shm_open`](#shm_open)
- [`shm_unlink`](#shm_unlink)
- [`sigaltstack`](#sigaltstack)
- [`signalfd`](#signalfd)
- [`sigtimedwait`](#sigtimedwait)
- [`sigwaitinfo`](#sigwaitinfo)
- [`splice`](#splice)
- [`swapoff`](#swapoff)
- [`swapon`](#swapon)
- [`sync`](#sync)
- [`sync_file_range`](#sync_file_range)
- [`syncfs`](#syncfs)
- [`syscall`](#syscall)
- [`tee`](#tee)
- [`timerfd_create`](#timerfd_create)
- [`timerfd_gettime`](#timerfd_gettime)
- [`timerfd_settime`](#timerfd_settime)
- [`tmpfile64`](#tmpfile64)
- [`umount`](#umount)
- [`umount2`](#umount2)
- [`unshare`](#unshare)
- [`vhangup`](#vhangup)
- [`vmsplice`](#vmsplice)

**Constants**

- [`ABS_CNT`](#abs_cnt)
- [`ABS_MAX`](#abs_max)
- [`ALG_OP_DECRYPT`](#alg_op_decrypt)
- [`ALG_OP_ENCRYPT`](#alg_op_encrypt)
- [`ALG_SET_AEAD_ASSOCLEN`](#alg_set_aead_assoclen)
- [`ALG_SET_AEAD_AUTHSIZE`](#alg_set_aead_authsize)
- [`ALG_SET_DRBG_ENTROPY`](#alg_set_drbg_entropy)
- [`ALG_SET_IV`](#alg_set_iv)
- [`ALG_SET_KEY`](#alg_set_key)
- [`ALG_SET_KEY_BY_KEY_SERIAL`](#alg_set_key_by_key_serial)
- [`ALG_SET_OP`](#alg_set_op)
- [`AT_EXECVE_CHECK`](#at_execve_check)
- [`AT_HANDLE_CONNECTABLE`](#at_handle_connectable)
- [`AT_HANDLE_FID`](#at_handle_fid)
- [`AT_HANDLE_MNT_ID_UNIQUE`](#at_handle_mnt_id_unique)
- [`BPF_A`](#bpf_a)
- [`BPF_ABS`](#bpf_abs)
- [`BPF_ADD`](#bpf_add)
- [`BPF_ALU`](#bpf_alu)
- [`BPF_AND`](#bpf_and)
- [`BPF_B`](#bpf_b)
- [`BPF_DIV`](#bpf_div)
- [`BPF_H`](#bpf_h)
- [`BPF_IMM`](#bpf_imm)
- [`BPF_IND`](#bpf_ind)
- [`BPF_JA`](#bpf_ja)
- [`BPF_JEQ`](#bpf_jeq)
- [`BPF_JGE`](#bpf_jge)
- [`BPF_JGT`](#bpf_jgt)
- [`BPF_JMP`](#bpf_jmp)
- [`BPF_JSET`](#bpf_jset)
- [`BPF_K`](#bpf_k)
- [`BPF_LD`](#bpf_ld)
- [`BPF_LDX`](#bpf_ldx)
- [`BPF_LEN`](#bpf_len)
- [`BPF_LL_OFF`](#bpf_ll_off)
- [`BPF_LSH`](#bpf_lsh)
- [`BPF_MAXINSNS`](#bpf_maxinsns)
- [`BPF_MEM`](#bpf_mem)
- [`BPF_MEMWORDS`](#bpf_memwords)
- [`BPF_MISC`](#bpf_misc)
- [`BPF_MOD`](#bpf_mod)
- [`BPF_MSH`](#bpf_msh)
- [`BPF_MUL`](#bpf_mul)
- [`BPF_NEG`](#bpf_neg)
- [`BPF_NET_OFF`](#bpf_net_off)
- [`BPF_OR`](#bpf_or)
- [`BPF_RET`](#bpf_ret)
- [`BPF_RSH`](#bpf_rsh)
- [`BPF_ST`](#bpf_st)
- [`BPF_STX`](#bpf_stx)
- [`BPF_SUB`](#bpf_sub)
- [`BPF_TAX`](#bpf_tax)
- [`BPF_TXA`](#bpf_txa)
- [`BPF_W`](#bpf_w)
- [`BPF_X`](#bpf_x)
- [`BPF_XOR`](#bpf_xor)
- [`CLONE_PIDFD`](#clone_pidfd)
- [`CLOSE_RANGE_CLOEXEC`](#close_range_cloexec)
- [`CLOSE_RANGE_UNSHARE`](#close_range_unshare)
- [`CN_DST_IDX`](#cn_dst_idx)
- [`CN_DST_VAL`](#cn_dst_val)
- [`CN_IDX_BB`](#cn_idx_bb)
- [`CN_IDX_CIFS`](#cn_idx_cifs)
- [`CN_IDX_DM`](#cn_idx_dm)
- [`CN_IDX_DRBD`](#cn_idx_drbd)
- [`CN_IDX_PROC`](#cn_idx_proc)
- [`CN_IDX_V86D`](#cn_idx_v86d)
- [`CN_KVP_IDX`](#cn_kvp_idx)
- [`CN_KVP_VAL`](#cn_kvp_val)
- [`CN_VAL_CIFS`](#cn_val_cifs)
- [`CN_VAL_DM_USERSPACE_LOG`](#cn_val_dm_userspace_log)
- [`CN_VAL_DRBD`](#cn_val_drbd)
- [`CN_VAL_PROC`](#cn_val_proc)
- [`CN_VAL_V86D_UVESAFB`](#cn_val_v86d_uvesafb)
- [`CN_VSS_IDX`](#cn_vss_idx)
- [`CN_VSS_VAL`](#cn_vss_val)
- [`CN_W1_IDX`](#cn_w1_idx)
- [`CN_W1_VAL`](#cn_w1_val)
- [`CTL_ABI`](#ctl_abi)
- [`CTL_BUS`](#ctl_bus)
- [`CTL_BUS_ISA`](#ctl_bus_isa)
- [`CTL_CPU`](#ctl_cpu)
- [`CTL_DEBUG`](#ctl_debug)
- [`CTL_DEV`](#ctl_dev)
- [`CTL_FS`](#ctl_fs)
- [`CTL_KERN`](#ctl_kern)
- [`CTL_NET`](#ctl_net)
- [`CTL_VM`](#ctl_vm)
- [`CTRL_ATTR_FAMILY_ID`](#ctrl_attr_family_id)
- [`CTRL_ATTR_FAMILY_NAME`](#ctrl_attr_family_name)
- [`CTRL_ATTR_HDRSIZE`](#ctrl_attr_hdrsize)
- [`CTRL_ATTR_MAXATTR`](#ctrl_attr_maxattr)
- [`CTRL_ATTR_MCAST_GROUPS`](#ctrl_attr_mcast_groups)
- [`CTRL_ATTR_MCAST_GRP_ID`](#ctrl_attr_mcast_grp_id)
- [`CTRL_ATTR_MCAST_GRP_NAME`](#ctrl_attr_mcast_grp_name)
- [`CTRL_ATTR_MCAST_GRP_UNSPEC`](#ctrl_attr_mcast_grp_unspec)
- [`CTRL_ATTR_OPS`](#ctrl_attr_ops)
- [`CTRL_ATTR_OP_FLAGS`](#ctrl_attr_op_flags)
- [`CTRL_ATTR_OP_ID`](#ctrl_attr_op_id)
- [`CTRL_ATTR_OP_UNSPEC`](#ctrl_attr_op_unspec)
- [`CTRL_ATTR_UNSPEC`](#ctrl_attr_unspec)
- [`CTRL_ATTR_VERSION`](#ctrl_attr_version)
- [`CTRL_CMD_DELFAMILY`](#ctrl_cmd_delfamily)
- [`CTRL_CMD_DELMCAST_GRP`](#ctrl_cmd_delmcast_grp)
- [`CTRL_CMD_DELOPS`](#ctrl_cmd_delops)
- [`CTRL_CMD_GETFAMILY`](#ctrl_cmd_getfamily)
- [`CTRL_CMD_GETMCAST_GRP`](#ctrl_cmd_getmcast_grp)
- [`CTRL_CMD_GETOPS`](#ctrl_cmd_getops)
- [`CTRL_CMD_NEWFAMILY`](#ctrl_cmd_newfamily)
- [`CTRL_CMD_NEWMCAST_GRP`](#ctrl_cmd_newmcast_grp)
- [`CTRL_CMD_NEWOPS`](#ctrl_cmd_newops)
- [`CTRL_CMD_UNSPEC`](#ctrl_cmd_unspec)
- [`DCCP_SERVICE_LIST_MAX_LEN`](#dccp_service_list_max_len) - maximum number of services provided on the same listening port
- [`DCCP_SOCKOPT_AVAILABLE_CCIDS`](#dccp_sockopt_available_ccids)
- [`DCCP_SOCKOPT_CCID`](#dccp_sockopt_ccid)
- [`DCCP_SOCKOPT_CCID_RX_INFO`](#dccp_sockopt_ccid_rx_info)
- [`DCCP_SOCKOPT_CCID_TX_INFO`](#dccp_sockopt_ccid_tx_info)
- [`DCCP_SOCKOPT_CHANGE_L`](#dccp_sockopt_change_l)
- [`DCCP_SOCKOPT_CHANGE_R`](#dccp_sockopt_change_r)
- [`DCCP_SOCKOPT_GET_CUR_MPS`](#dccp_sockopt_get_cur_mps)
- [`DCCP_SOCKOPT_PACKET_SIZE`](#dccp_sockopt_packet_size)
- [`DCCP_SOCKOPT_QPOLICY_ID`](#dccp_sockopt_qpolicy_id)
- [`DCCP_SOCKOPT_QPOLICY_TXQLEN`](#dccp_sockopt_qpolicy_txqlen)
- [`DCCP_SOCKOPT_RECV_CSCOV`](#dccp_sockopt_recv_cscov)
- [`DCCP_SOCKOPT_RX_CCID`](#dccp_sockopt_rx_ccid)
- [`DCCP_SOCKOPT_SEND_CSCOV`](#dccp_sockopt_send_cscov)
- [`DCCP_SOCKOPT_SERVER_TIMEWAIT`](#dccp_sockopt_server_timewait)
- [`DCCP_SOCKOPT_SERVICE`](#dccp_sockopt_service)
- [`DCCP_SOCKOPT_TX_CCID`](#dccp_sockopt_tx_ccid)
- [`EFD_SEMAPHORE`](#efd_semaphore)
- [`ENOATTR`](#enoattr)
- [`EPIOCGPARAMS`](#epiocgparams)
- [`EPIOCSPARAMS`](#epiocsparams)
- [`ETH_ALEN`](#eth_alen)
- [`ETH_DATA_LEN`](#eth_data_len)
- [`ETH_FCS_LEN`](#eth_fcs_len)
- [`ETH_FRAME_LEN`](#eth_frame_len)
- [`ETH_HLEN`](#eth_hlen)
- [`ETH_P_1588`](#eth_p_1588)
- [`ETH_P_8021AD`](#eth_p_8021ad)
- [`ETH_P_8021AH`](#eth_p_8021ah)
- [`ETH_P_8021Q`](#eth_p_8021q)
- [`ETH_P_80221`](#eth_p_80221)
- [`ETH_P_802_2`](#eth_p_802_2)
- [`ETH_P_802_3`](#eth_p_802_3)
- [`ETH_P_802_3_MIN`](#eth_p_802_3_min)
- [`ETH_P_802_EX1`](#eth_p_802_ex1)
- [`ETH_P_AARP`](#eth_p_aarp)
- [`ETH_P_AF_IUCV`](#eth_p_af_iucv)
- [`ETH_P_ALL`](#eth_p_all)
- [`ETH_P_AOE`](#eth_p_aoe)
- [`ETH_P_ARCNET`](#eth_p_arcnet)
- [`ETH_P_ARP`](#eth_p_arp)
- [`ETH_P_ATALK`](#eth_p_atalk)
- [`ETH_P_ATMFATE`](#eth_p_atmfate)
- [`ETH_P_ATMMPOA`](#eth_p_atmmpoa)
- [`ETH_P_AX25`](#eth_p_ax25)
- [`ETH_P_BATMAN`](#eth_p_batman)
- [`ETH_P_BPQ`](#eth_p_bpq)
- [`ETH_P_CAIF`](#eth_p_caif)
- [`ETH_P_CANFD`](#eth_p_canfd)
- [`ETH_P_CONTROL`](#eth_p_control)
- [`ETH_P_CUST`](#eth_p_cust)
- [`ETH_P_DDCMP`](#eth_p_ddcmp)
- [`ETH_P_DEC`](#eth_p_dec)
- [`ETH_P_DIAG`](#eth_p_diag)
- [`ETH_P_DNA_DL`](#eth_p_dna_dl)
- [`ETH_P_DNA_RC`](#eth_p_dna_rc)
- [`ETH_P_DNA_RT`](#eth_p_dna_rt)
- [`ETH_P_DSA`](#eth_p_dsa)
- [`ETH_P_ECONET`](#eth_p_econet)
- [`ETH_P_EDSA`](#eth_p_edsa)
- [`ETH_P_FCOE`](#eth_p_fcoe)
- [`ETH_P_FIP`](#eth_p_fip)
- [`ETH_P_HDLC`](#eth_p_hdlc)
- [`ETH_P_IEEE802154`](#eth_p_ieee802154)
- [`ETH_P_IEEEPUP`](#eth_p_ieeepup)
- [`ETH_P_IEEEPUPAT`](#eth_p_ieeepupat)
- [`ETH_P_IP`](#eth_p_ip)
- [`ETH_P_IPV6`](#eth_p_ipv6)
- [`ETH_P_IPX`](#eth_p_ipx)
- [`ETH_P_IRDA`](#eth_p_irda)
- [`ETH_P_LAT`](#eth_p_lat)
- [`ETH_P_LINK_CTL`](#eth_p_link_ctl)
- [`ETH_P_LOCALTALK`](#eth_p_localtalk)
- [`ETH_P_LOOP`](#eth_p_loop)
- [`ETH_P_LOOPBACK`](#eth_p_loopback)
- [`ETH_P_MACSEC`](#eth_p_macsec)
- [`ETH_P_MOBITEX`](#eth_p_mobitex)
- [`ETH_P_MPLS_MC`](#eth_p_mpls_mc)
- [`ETH_P_MPLS_UC`](#eth_p_mpls_uc)
- [`ETH_P_MVRP`](#eth_p_mvrp)
- [`ETH_P_PAE`](#eth_p_pae)
- [`ETH_P_PAUSE`](#eth_p_pause)
- [`ETH_P_PHONET`](#eth_p_phonet)
- [`ETH_P_PPPTALK`](#eth_p_ppptalk)
- [`ETH_P_PPP_DISC`](#eth_p_ppp_disc)
- [`ETH_P_PPP_MP`](#eth_p_ppp_mp)
- [`ETH_P_PPP_SES`](#eth_p_ppp_ses)
- [`ETH_P_PRP`](#eth_p_prp)
- [`ETH_P_PUP`](#eth_p_pup)
- [`ETH_P_PUPAT`](#eth_p_pupat)
- [`ETH_P_QINQ1`](#eth_p_qinq1)
- [`ETH_P_QINQ2`](#eth_p_qinq2)
- [`ETH_P_QINQ3`](#eth_p_qinq3)
- [`ETH_P_RARP`](#eth_p_rarp)
- [`ETH_P_SCA`](#eth_p_sca)
- [`ETH_P_SLOW`](#eth_p_slow)
- [`ETH_P_SNAP`](#eth_p_snap)
- [`ETH_P_TDLS`](#eth_p_tdls)
- [`ETH_P_TEB`](#eth_p_teb)
- [`ETH_P_TIPC`](#eth_p_tipc)
- [`ETH_P_TRAILER`](#eth_p_trailer)
- [`ETH_P_TR_802_2`](#eth_p_tr_802_2)
- [`ETH_P_WAN_PPP`](#eth_p_wan_ppp)
- [`ETH_P_WCCP`](#eth_p_wccp)
- [`ETH_P_X25`](#eth_p_x25)
- [`ETH_ZLEN`](#eth_zlen)
- [`EV_CNT`](#ev_cnt)
- [`EV_MAX`](#ev_max)
- [`FALLOC_FL_COLLAPSE_RANGE`](#falloc_fl_collapse_range)
- [`FALLOC_FL_INSERT_RANGE`](#falloc_fl_insert_range)
- [`FALLOC_FL_KEEP_SIZE`](#falloc_fl_keep_size)
- [`FALLOC_FL_PUNCH_HOLE`](#falloc_fl_punch_hole)
- [`FALLOC_FL_UNSHARE_RANGE`](#falloc_fl_unshare_range)
- [`FALLOC_FL_ZERO_RANGE`](#falloc_fl_zero_range)
- [`FANOTIFY_METADATA_VERSION`](#fanotify_metadata_version)
- [`FAN_ACCESS`](#fan_access)
- [`FAN_ACCESS_PERM`](#fan_access_perm)
- [`FAN_ALLOW`](#fan_allow)
- [`FAN_ATTRIB`](#fan_attrib)
- [`FAN_AUDIT`](#fan_audit)
- [`FAN_CLASS_CONTENT`](#fan_class_content)
- [`FAN_CLASS_NOTIF`](#fan_class_notif)
- [`FAN_CLASS_PRE_CONTENT`](#fan_class_pre_content)
- [`FAN_CLOEXEC`](#fan_cloexec)
- [`FAN_CLOSE`](#fan_close)
- [`FAN_CLOSE_NOWRITE`](#fan_close_nowrite)
- [`FAN_CLOSE_WRITE`](#fan_close_write)
- [`FAN_CREATE`](#fan_create)
- [`FAN_DELETE`](#fan_delete)
- [`FAN_DELETE_SELF`](#fan_delete_self)
- [`FAN_DENY`](#fan_deny)
- [`FAN_ENABLE_AUDIT`](#fan_enable_audit)
- [`FAN_EPIDFD`](#fan_epidfd)
- [`FAN_EVENT_INFO_TYPE_DFID`](#fan_event_info_type_dfid)
- [`FAN_EVENT_INFO_TYPE_DFID_NAME`](#fan_event_info_type_dfid_name)
- [`FAN_EVENT_INFO_TYPE_ERROR`](#fan_event_info_type_error)
- [`FAN_EVENT_INFO_TYPE_FID`](#fan_event_info_type_fid)
- [`FAN_EVENT_INFO_TYPE_NEW_DFID_NAME`](#fan_event_info_type_new_dfid_name)
- [`FAN_EVENT_INFO_TYPE_OLD_DFID_NAME`](#fan_event_info_type_old_dfid_name)
- [`FAN_EVENT_INFO_TYPE_PIDFD`](#fan_event_info_type_pidfd)
- [`FAN_EVENT_ON_CHILD`](#fan_event_on_child)
- [`FAN_FS_ERROR`](#fan_fs_error)
- [`FAN_INFO`](#fan_info)
- [`FAN_MARK_ADD`](#fan_mark_add)
- [`FAN_MARK_DONT_FOLLOW`](#fan_mark_dont_follow)
- [`FAN_MARK_EVICTABLE`](#fan_mark_evictable)
- [`FAN_MARK_FILESYSTEM`](#fan_mark_filesystem)
- [`FAN_MARK_FLUSH`](#fan_mark_flush)
- [`FAN_MARK_IGNORE`](#fan_mark_ignore)
- [`FAN_MARK_IGNORED_MASK`](#fan_mark_ignored_mask)
- [`FAN_MARK_IGNORED_SURV_MODIFY`](#fan_mark_ignored_surv_modify)
- [`FAN_MARK_IGNORE_SURV`](#fan_mark_ignore_surv)
- [`FAN_MARK_INODE`](#fan_mark_inode)
- [`FAN_MARK_MOUNT`](#fan_mark_mount)
- [`FAN_MARK_ONLYDIR`](#fan_mark_onlydir)
- [`FAN_MARK_REMOVE`](#fan_mark_remove)
- [`FAN_MODIFY`](#fan_modify)
- [`FAN_MOVE`](#fan_move)
- [`FAN_MOVED_FROM`](#fan_moved_from)
- [`FAN_MOVED_TO`](#fan_moved_to)
- [`FAN_MOVE_SELF`](#fan_move_self)
- [`FAN_NOFD`](#fan_nofd)
- [`FAN_NONBLOCK`](#fan_nonblock)
- [`FAN_NOPIDFD`](#fan_nopidfd)
- [`FAN_ONDIR`](#fan_ondir)
- [`FAN_OPEN`](#fan_open)
- [`FAN_OPEN_EXEC`](#fan_open_exec)
- [`FAN_OPEN_EXEC_PERM`](#fan_open_exec_perm)
- [`FAN_OPEN_PERM`](#fan_open_perm)
- [`FAN_Q_OVERFLOW`](#fan_q_overflow)
- [`FAN_RENAME`](#fan_rename)
- [`FAN_REPORT_DFID_NAME`](#fan_report_dfid_name)
- [`FAN_REPORT_DFID_NAME_TARGET`](#fan_report_dfid_name_target)
- [`FAN_REPORT_DIR_FID`](#fan_report_dir_fid)
- [`FAN_REPORT_FID`](#fan_report_fid)
- [`FAN_REPORT_NAME`](#fan_report_name)
- [`FAN_REPORT_PIDFD`](#fan_report_pidfd)
- [`FAN_REPORT_TARGET_FID`](#fan_report_target_fid)
- [`FAN_REPORT_TID`](#fan_report_tid)
- [`FAN_RESPONSE_INFO_AUDIT_RULE`](#fan_response_info_audit_rule)
- [`FAN_RESPONSE_INFO_NONE`](#fan_response_info_none)
- [`FAN_UNLIMITED_MARKS`](#fan_unlimited_marks)
- [`FAN_UNLIMITED_QUEUE`](#fan_unlimited_queue)
- [`FF_CNT`](#ff_cnt)
- [`FF_MAX`](#ff_max)
- [`FUTEX_BITSET_MATCH_ANY`](#futex_bitset_match_any)
- [`FUTEX_CLOCK_REALTIME`](#futex_clock_realtime)
- [`FUTEX_CMD_MASK`](#futex_cmd_mask)
- [`FUTEX_CMP_REQUEUE`](#futex_cmp_requeue)
- [`FUTEX_CMP_REQUEUE_PI`](#futex_cmp_requeue_pi)
- [`FUTEX_FD`](#futex_fd)
- [`FUTEX_LOCK_PI`](#futex_lock_pi)
- [`FUTEX_LOCK_PI2`](#futex_lock_pi2)
- [`FUTEX_OP_ADD`](#futex_op_add)
- [`FUTEX_OP_ANDN`](#futex_op_andn)
- [`FUTEX_OP_CMP_EQ`](#futex_op_cmp_eq)
- [`FUTEX_OP_CMP_GE`](#futex_op_cmp_ge)
- [`FUTEX_OP_CMP_GT`](#futex_op_cmp_gt)
- [`FUTEX_OP_CMP_LE`](#futex_op_cmp_le)
- [`FUTEX_OP_CMP_LT`](#futex_op_cmp_lt)
- [`FUTEX_OP_CMP_NE`](#futex_op_cmp_ne)
- [`FUTEX_OP_OPARG_SHIFT`](#futex_op_oparg_shift)
- [`FUTEX_OP_OR`](#futex_op_or)
- [`FUTEX_OP_SET`](#futex_op_set)
- [`FUTEX_OP_XOR`](#futex_op_xor)
- [`FUTEX_OWNER_DIED`](#futex_owner_died)
- [`FUTEX_PRIVATE_FLAG`](#futex_private_flag)
- [`FUTEX_REQUEUE`](#futex_requeue)
- [`FUTEX_TID_MASK`](#futex_tid_mask)
- [`FUTEX_TRYLOCK_PI`](#futex_trylock_pi)
- [`FUTEX_UNLOCK_PI`](#futex_unlock_pi)
- [`FUTEX_WAIT`](#futex_wait)
- [`FUTEX_WAITERS`](#futex_waiters)
- [`FUTEX_WAIT_BITSET`](#futex_wait_bitset)
- [`FUTEX_WAIT_REQUEUE_PI`](#futex_wait_requeue_pi)
- [`FUTEX_WAKE`](#futex_wake)
- [`FUTEX_WAKE_BITSET`](#futex_wake_bitset)
- [`FUTEX_WAKE_OP`](#futex_wake_op)
- [`F_SEAL_EXEC`](#f_seal_exec)
- [`F_SEAL_FUTURE_WRITE`](#f_seal_future_write)
- [`GENL_ADMIN_PERM`](#genl_admin_perm)
- [`GENL_CMD_CAP_DO`](#genl_cmd_cap_do)
- [`GENL_CMD_CAP_DUMP`](#genl_cmd_cap_dump)
- [`GENL_CMD_CAP_HASPOL`](#genl_cmd_cap_haspol)
- [`GENL_ID_CTRL`](#genl_id_ctrl)
- [`GENL_MAX_ID`](#genl_max_id)
- [`GENL_MIN_ID`](#genl_min_id)
- [`GENL_NAMSIZ`](#genl_namsiz)
- [`GETALL`](#getall)
- [`GETNCNT`](#getncnt)
- [`GETPID`](#getpid)
- [`GETVAL`](#getval)
- [`GETZCNT`](#getzcnt)
- [`GRND_INSECURE`](#grnd_insecure)
- [`GRND_NONBLOCK`](#grnd_nonblock)
- [`GRND_RANDOM`](#grnd_random)
- [`HWTSTAMP_FILTER_ALL`](#hwtstamp_filter_all)
- [`HWTSTAMP_FILTER_NONE`](#hwtstamp_filter_none)
- [`HWTSTAMP_FILTER_NTP_ALL`](#hwtstamp_filter_ntp_all)
- [`HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ`](#hwtstamp_filter_ptp_v1_l4_delay_req)
- [`HWTSTAMP_FILTER_PTP_V1_L4_EVENT`](#hwtstamp_filter_ptp_v1_l4_event)
- [`HWTSTAMP_FILTER_PTP_V1_L4_SYNC`](#hwtstamp_filter_ptp_v1_l4_sync)
- [`HWTSTAMP_FILTER_PTP_V2_DELAY_REQ`](#hwtstamp_filter_ptp_v2_delay_req)
- [`HWTSTAMP_FILTER_PTP_V2_EVENT`](#hwtstamp_filter_ptp_v2_event)
- [`HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ`](#hwtstamp_filter_ptp_v2_l2_delay_req)
- [`HWTSTAMP_FILTER_PTP_V2_L2_EVENT`](#hwtstamp_filter_ptp_v2_l2_event)
- [`HWTSTAMP_FILTER_PTP_V2_L2_SYNC`](#hwtstamp_filter_ptp_v2_l2_sync)
- [`HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ`](#hwtstamp_filter_ptp_v2_l4_delay_req)
- [`HWTSTAMP_FILTER_PTP_V2_L4_EVENT`](#hwtstamp_filter_ptp_v2_l4_event)
- [`HWTSTAMP_FILTER_PTP_V2_L4_SYNC`](#hwtstamp_filter_ptp_v2_l4_sync)
- [`HWTSTAMP_FILTER_PTP_V2_SYNC`](#hwtstamp_filter_ptp_v2_sync)
- [`HWTSTAMP_FILTER_SOME`](#hwtstamp_filter_some)
- [`HWTSTAMP_TX_OFF`](#hwtstamp_tx_off)
- [`HWTSTAMP_TX_ON`](#hwtstamp_tx_on)
- [`HWTSTAMP_TX_ONESTEP_P2P`](#hwtstamp_tx_onestep_p2p)
- [`HWTSTAMP_TX_ONESTEP_SYNC`](#hwtstamp_tx_onestep_sync)
- [`IFA_ADDRESS`](#ifa_address)
- [`IFA_ANYCAST`](#ifa_anycast)
- [`IFA_BROADCAST`](#ifa_broadcast)
- [`IFA_CACHEINFO`](#ifa_cacheinfo)
- [`IFA_FLAGS`](#ifa_flags)
- [`IFA_F_DADFAILED`](#ifa_f_dadfailed)
- [`IFA_F_DEPRECATED`](#ifa_f_deprecated)
- [`IFA_F_HOMEADDRESS`](#ifa_f_homeaddress)
- [`IFA_F_MANAGETEMPADDR`](#ifa_f_managetempaddr)
- [`IFA_F_MCAUTOJOIN`](#ifa_f_mcautojoin)
- [`IFA_F_NODAD`](#ifa_f_nodad)
- [`IFA_F_NOPREFIXROUTE`](#ifa_f_noprefixroute)
- [`IFA_F_OPTIMISTIC`](#ifa_f_optimistic)
- [`IFA_F_PERMANENT`](#ifa_f_permanent)
- [`IFA_F_SECONDARY`](#ifa_f_secondary)
- [`IFA_F_STABLE_PRIVACY`](#ifa_f_stable_privacy)
- [`IFA_F_TEMPORARY`](#ifa_f_temporary)
- [`IFA_F_TENTATIVE`](#ifa_f_tentative)
- [`IFA_LABEL`](#ifa_label)
- [`IFA_LOCAL`](#ifa_local)
- [`IFA_MULTICAST`](#ifa_multicast)
- [`IFA_UNSPEC`](#ifa_unspec)
- [`IFF_DORMANT`](#iff_dormant)
- [`IFF_ECHO`](#iff_echo)
- [`IFF_LOWER_UP`](#iff_lower_up)
- [`IFLA_ADDRESS`](#ifla_address)
- [`IFLA_AF_SPEC`](#ifla_af_spec)
- [`IFLA_ALLMULTI`](#ifla_allmulti)
- [`IFLA_ALT_IFNAME`](#ifla_alt_ifname)
- [`IFLA_BROADCAST`](#ifla_broadcast)
- [`IFLA_CARRIER`](#ifla_carrier)
- [`IFLA_CARRIER_CHANGES`](#ifla_carrier_changes)
- [`IFLA_CARRIER_DOWN_COUNT`](#ifla_carrier_down_count)
- [`IFLA_CARRIER_UP_COUNT`](#ifla_carrier_up_count)
- [`IFLA_COST`](#ifla_cost)
- [`IFLA_EVENT`](#ifla_event)
- [`IFLA_EXT_MASK`](#ifla_ext_mask)
- [`IFLA_GROUP`](#ifla_group)
- [`IFLA_GRO_MAX_SIZE`](#ifla_gro_max_size)
- [`IFLA_GSO_MAX_SEGS`](#ifla_gso_max_segs)
- [`IFLA_GSO_MAX_SIZE`](#ifla_gso_max_size)
- [`IFLA_IFALIAS`](#ifla_ifalias)
- [`IFLA_IFNAME`](#ifla_ifname)
- [`IFLA_IF_NETNSID`](#ifla_if_netnsid)
- [`IFLA_INFO_DATA`](#ifla_info_data)
- [`IFLA_INFO_KIND`](#ifla_info_kind)
- [`IFLA_INFO_SLAVE_DATA`](#ifla_info_slave_data)
- [`IFLA_INFO_SLAVE_KIND`](#ifla_info_slave_kind)
- [`IFLA_INFO_UNSPEC`](#ifla_info_unspec)
- [`IFLA_INFO_XSTATS`](#ifla_info_xstats)
- [`IFLA_LINK`](#ifla_link)
- [`IFLA_LINKINFO`](#ifla_linkinfo)
- [`IFLA_LINKMODE`](#ifla_linkmode)
- [`IFLA_LINK_NETNSID`](#ifla_link_netnsid)
- [`IFLA_MAP`](#ifla_map)
- [`IFLA_MASTER`](#ifla_master)
- [`IFLA_MAX_MTU`](#ifla_max_mtu)
- [`IFLA_MIN_MTU`](#ifla_min_mtu)
- [`IFLA_MTU`](#ifla_mtu)
- [`IFLA_NET_NS_FD`](#ifla_net_ns_fd)
- [`IFLA_NET_NS_PID`](#ifla_net_ns_pid)
- [`IFLA_NEW_IFINDEX`](#ifla_new_ifindex)
- [`IFLA_NEW_NETNSID`](#ifla_new_netnsid)
- [`IFLA_NUM_RX_QUEUES`](#ifla_num_rx_queues)
- [`IFLA_NUM_TX_QUEUES`](#ifla_num_tx_queues)
- [`IFLA_NUM_VF`](#ifla_num_vf)
- [`IFLA_OPERSTATE`](#ifla_operstate)
- [`IFLA_PAD`](#ifla_pad)
- [`IFLA_PARENT_DEV_BUS_NAME`](#ifla_parent_dev_bus_name)
- [`IFLA_PARENT_DEV_NAME`](#ifla_parent_dev_name)
- [`IFLA_PERM_ADDRESS`](#ifla_perm_address)
- [`IFLA_PHYS_PORT_ID`](#ifla_phys_port_id)
- [`IFLA_PHYS_PORT_NAME`](#ifla_phys_port_name)
- [`IFLA_PHYS_SWITCH_ID`](#ifla_phys_switch_id)
- [`IFLA_PORT_SELF`](#ifla_port_self)
- [`IFLA_PRIORITY`](#ifla_priority)
- [`IFLA_PROMISCUITY`](#ifla_promiscuity)
- [`IFLA_PROP_LIST`](#ifla_prop_list)
- [`IFLA_PROTINFO`](#ifla_protinfo)
- [`IFLA_PROTO_DOWN`](#ifla_proto_down)
- [`IFLA_PROTO_DOWN_REASON`](#ifla_proto_down_reason)
- [`IFLA_QDISC`](#ifla_qdisc)
- [`IFLA_STATS`](#ifla_stats)
- [`IFLA_STATS64`](#ifla_stats64)
- [`IFLA_TARGET_NETNSID`](#ifla_target_netnsid)
- [`IFLA_TSO_MAX_SEGS`](#ifla_tso_max_segs)
- [`IFLA_TSO_MAX_SIZE`](#ifla_tso_max_size)
- [`IFLA_TXQLEN`](#ifla_txqlen)
- [`IFLA_UNSPEC`](#ifla_unspec)
- [`IFLA_VFINFO_LIST`](#ifla_vfinfo_list)
- [`IFLA_VF_PORTS`](#ifla_vf_ports)
- [`IFLA_WEIGHT`](#ifla_weight)
- [`IFLA_WIRELESS`](#ifla_wireless)
- [`IFLA_XDP`](#ifla_xdp)
- [`IF_LINK_MODE_DEFAULT`](#if_link_mode_default)
- [`IF_LINK_MODE_DORMANT`](#if_link_mode_dormant)
- [`IF_LINK_MODE_TESTING`](#if_link_mode_testing)
- [`IF_OPER_DORMANT`](#if_oper_dormant)
- [`IF_OPER_DOWN`](#if_oper_down)
- [`IF_OPER_LOWERLAYERDOWN`](#if_oper_lowerlayerdown)
- [`IF_OPER_NOTPRESENT`](#if_oper_notpresent)
- [`IF_OPER_TESTING`](#if_oper_testing)
- [`IF_OPER_UNKNOWN`](#if_oper_unknown)
- [`IF_OPER_UP`](#if_oper_up)
- [`INOTIFY_MAX_QUEUED_EVENTS`](#inotify_max_queued_events)
- [`INOTIFY_MAX_USER_INSTANCES`](#inotify_max_user_instances)
- [`INOTIFY_MAX_USER_WATCHES`](#inotify_max_user_watches)
- [`INPUT_PROP_ACCELEROMETER`](#input_prop_accelerometer)
- [`INPUT_PROP_BUTTONPAD`](#input_prop_buttonpad)
- [`INPUT_PROP_CNT`](#input_prop_cnt)
- [`INPUT_PROP_DIRECT`](#input_prop_direct)
- [`INPUT_PROP_MAX`](#input_prop_max)
- [`INPUT_PROP_POINTER`](#input_prop_pointer)
- [`INPUT_PROP_POINTING_STICK`](#input_prop_pointing_stick)
- [`INPUT_PROP_SEMI_MT`](#input_prop_semi_mt)
- [`INPUT_PROP_TOPBUTTONPAD`](#input_prop_topbuttonpad)
- [`IN_ACCESS`](#in_access)
- [`IN_ALL_EVENTS`](#in_all_events)
- [`IN_ATTRIB`](#in_attrib)
- [`IN_CLOEXEC`](#in_cloexec)
- [`IN_CLOSE`](#in_close)
- [`IN_CLOSE_NOWRITE`](#in_close_nowrite)
- [`IN_CLOSE_WRITE`](#in_close_write)
- [`IN_CREATE`](#in_create)
- [`IN_DELETE`](#in_delete)
- [`IN_DELETE_SELF`](#in_delete_self)
- [`IN_DONT_FOLLOW`](#in_dont_follow)
- [`IN_EXCL_UNLINK`](#in_excl_unlink)
- [`IN_IGNORED`](#in_ignored)
- [`IN_ISDIR`](#in_isdir)
- [`IN_MASK_ADD`](#in_mask_add)
- [`IN_MASK_CREATE`](#in_mask_create)
- [`IN_MODIFY`](#in_modify)
- [`IN_MOVE`](#in_move)
- [`IN_MOVED_FROM`](#in_moved_from)
- [`IN_MOVED_TO`](#in_moved_to)
- [`IN_MOVE_SELF`](#in_move_self)
- [`IN_NONBLOCK`](#in_nonblock)
- [`IN_ONESHOT`](#in_oneshot)
- [`IN_ONLYDIR`](#in_onlydir)
- [`IN_OPEN`](#in_open)
- [`IN_Q_OVERFLOW`](#in_q_overflow)
- [`IN_UNMOUNT`](#in_unmount)
- [`IP6T_SO_ORIGINAL_DST`](#ip6t_so_original_dst)
- [`IPV6_FLOWINFO`](#ipv6_flowinfo)
- [`IPV6_FLOWINFO_FLOWLABEL`](#ipv6_flowinfo_flowlabel)
- [`IPV6_FLOWINFO_PRIORITY`](#ipv6_flowinfo_priority)
- [`IPV6_FLOWINFO_SEND`](#ipv6_flowinfo_send)
- [`IPV6_FLOWLABEL_MGR`](#ipv6_flowlabel_mgr)
- [`IPV6_FREEBIND`](#ipv6_freebind)
- [`IPV6_RECVFRAGSIZE`](#ipv6_recvfragsize)
- [`IP_RECVFRAGSIZE`](#ip_recvfragsize)
- [`IWEVASSOCREQIE`](#iwevassocreqie)
- [`IWEVASSOCRESPIE`](#iwevassocrespie)
- [`IWEVCUSTOM`](#iwevcustom)
- [`IWEVEXPIRED`](#iwevexpired)
- [`IWEVFIRST`](#iwevfirst)
- [`IWEVGENIE`](#iwevgenie)
- [`IWEVMICHAELMICFAILURE`](#iwevmichaelmicfailure)
- [`IWEVPMKIDCAND`](#iwevpmkidcand)
- [`IWEVQUAL`](#iwevqual)
- [`IWEVREGISTERED`](#iwevregistered)
- [`IWEVTXDROP`](#iwevtxdrop)
- [`IW_AUTH_80211_AUTH_ALG`](#iw_auth_80211_auth_alg)
- [`IW_AUTH_ALG_LEAP`](#iw_auth_alg_leap)
- [`IW_AUTH_ALG_OPEN_SYSTEM`](#iw_auth_alg_open_system)
- [`IW_AUTH_ALG_SHARED_KEY`](#iw_auth_alg_shared_key)
- [`IW_AUTH_CIPHER_AES_CMAC`](#iw_auth_cipher_aes_cmac)
- [`IW_AUTH_CIPHER_CCMP`](#iw_auth_cipher_ccmp)
- [`IW_AUTH_CIPHER_GROUP`](#iw_auth_cipher_group)
- [`IW_AUTH_CIPHER_GROUP_MGMT`](#iw_auth_cipher_group_mgmt)
- [`IW_AUTH_CIPHER_NONE`](#iw_auth_cipher_none)
- [`IW_AUTH_CIPHER_PAIRWISE`](#iw_auth_cipher_pairwise)
- [`IW_AUTH_CIPHER_TKIP`](#iw_auth_cipher_tkip)
- [`IW_AUTH_CIPHER_WEP104`](#iw_auth_cipher_wep104)
- [`IW_AUTH_CIPHER_WEP40`](#iw_auth_cipher_wep40)
- [`IW_AUTH_DROP_UNENCRYPTED`](#iw_auth_drop_unencrypted)
- [`IW_AUTH_FLAGS`](#iw_auth_flags)
- [`IW_AUTH_INDEX`](#iw_auth_index)
- [`IW_AUTH_KEY_MGMT`](#iw_auth_key_mgmt)
- [`IW_AUTH_KEY_MGMT_802_1X`](#iw_auth_key_mgmt_802_1x)
- [`IW_AUTH_KEY_MGMT_PSK`](#iw_auth_key_mgmt_psk)
- [`IW_AUTH_MFP`](#iw_auth_mfp)
- [`IW_AUTH_MFP_DISABLED`](#iw_auth_mfp_disabled)
- [`IW_AUTH_MFP_OPTIONAL`](#iw_auth_mfp_optional)
- [`IW_AUTH_MFP_REQUIRED`](#iw_auth_mfp_required)
- [`IW_AUTH_PRIVACY_INVOKED`](#iw_auth_privacy_invoked)
- [`IW_AUTH_ROAMING_CONTROL`](#iw_auth_roaming_control)
- [`IW_AUTH_ROAMING_DISABLE`](#iw_auth_roaming_disable)
- [`IW_AUTH_ROAMING_ENABLE`](#iw_auth_roaming_enable)
- [`IW_AUTH_RX_UNENCRYPTED_EAPOL`](#iw_auth_rx_unencrypted_eapol)
- [`IW_AUTH_TKIP_COUNTERMEASURES`](#iw_auth_tkip_countermeasures)
- [`IW_AUTH_WPA_ENABLED`](#iw_auth_wpa_enabled)
- [`IW_AUTH_WPA_VERSION`](#iw_auth_wpa_version)
- [`IW_AUTH_WPA_VERSION_DISABLED`](#iw_auth_wpa_version_disabled)
- [`IW_AUTH_WPA_VERSION_WPA`](#iw_auth_wpa_version_wpa)
- [`IW_AUTH_WPA_VERSION_WPA2`](#iw_auth_wpa_version_wpa2)
- [`IW_CUSTOM_MAX`](#iw_custom_max)
- [`IW_ENCODE_ALG_AES_CMAC`](#iw_encode_alg_aes_cmac)
- [`IW_ENCODE_ALG_CCMP`](#iw_encode_alg_ccmp)
- [`IW_ENCODE_ALG_NONE`](#iw_encode_alg_none)
- [`IW_ENCODE_ALG_PMK`](#iw_encode_alg_pmk)
- [`IW_ENCODE_ALG_TKIP`](#iw_encode_alg_tkip)
- [`IW_ENCODE_ALG_WEP`](#iw_encode_alg_wep)
- [`IW_ENCODE_DISABLED`](#iw_encode_disabled)
- [`IW_ENCODE_ENABLED`](#iw_encode_enabled)
- [`IW_ENCODE_EXT_GROUP_KEY`](#iw_encode_ext_group_key)
- [`IW_ENCODE_EXT_RX_SEQ_VALID`](#iw_encode_ext_rx_seq_valid)
- [`IW_ENCODE_EXT_SET_TX_KEY`](#iw_encode_ext_set_tx_key)
- [`IW_ENCODE_EXT_TX_SEQ_VALID`](#iw_encode_ext_tx_seq_valid)
- [`IW_ENCODE_FLAGS`](#iw_encode_flags)
- [`IW_ENCODE_INDEX`](#iw_encode_index)
- [`IW_ENCODE_MODE`](#iw_encode_mode)
- [`IW_ENCODE_NOKEY`](#iw_encode_nokey)
- [`IW_ENCODE_OPEN`](#iw_encode_open)
- [`IW_ENCODE_RESTRICTED`](#iw_encode_restricted)
- [`IW_ENCODE_SEQ_MAX_SIZE`](#iw_encode_seq_max_size)
- [`IW_ENCODE_TEMP`](#iw_encode_temp)
- [`IW_ENCODING_TOKEN_MAX`](#iw_encoding_token_max)
- [`IW_ENC_CAPA_4WAY_HANDSHAKE`](#iw_enc_capa_4way_handshake)
- [`IW_ENC_CAPA_CIPHER_CCMP`](#iw_enc_capa_cipher_ccmp)
- [`IW_ENC_CAPA_CIPHER_TKIP`](#iw_enc_capa_cipher_tkip)
- [`IW_ENC_CAPA_WPA`](#iw_enc_capa_wpa)
- [`IW_ENC_CAPA_WPA2`](#iw_enc_capa_wpa2)
- [`IW_ESSID_MAX_SIZE`](#iw_essid_max_size)
- [`IW_EVENT_CAPA_K_0`](#iw_event_capa_k_0)
- [`IW_EVENT_CAPA_K_1`](#iw_event_capa_k_1)
- [`IW_EV_ADDR_PK_LEN`](#iw_ev_addr_pk_len)
- [`IW_EV_CHAR_PK_LEN`](#iw_ev_char_pk_len)
- [`IW_EV_FREQ_PK_LEN`](#iw_ev_freq_pk_len)
- [`IW_EV_LCP_PK_LEN`](#iw_ev_lcp_pk_len)
- [`IW_EV_PARAM_PK_LEN`](#iw_ev_param_pk_len)
- [`IW_EV_POINT_PK_LEN`](#iw_ev_point_pk_len)
- [`IW_EV_QUAL_PK_LEN`](#iw_ev_qual_pk_len)
- [`IW_EV_UINT_PK_LEN`](#iw_ev_uint_pk_len)
- [`IW_FREQ_AUTO`](#iw_freq_auto)
- [`IW_FREQ_FIXED`](#iw_freq_fixed)
- [`IW_GENERIC_IE_MAX`](#iw_generic_ie_max)
- [`IW_MAX_AP`](#iw_max_ap)
- [`IW_MAX_BITRATES`](#iw_max_bitrates)
- [`IW_MAX_ENCODING_SIZES`](#iw_max_encoding_sizes)
- [`IW_MAX_FREQUENCIES`](#iw_max_frequencies)
- [`IW_MAX_SPY`](#iw_max_spy)
- [`IW_MAX_TXPOWER`](#iw_max_txpower)
- [`IW_MICFAILURE_COUNT`](#iw_micfailure_count)
- [`IW_MICFAILURE_GROUP`](#iw_micfailure_group)
- [`IW_MICFAILURE_KEY_ID`](#iw_micfailure_key_id)
- [`IW_MICFAILURE_PAIRWISE`](#iw_micfailure_pairwise)
- [`IW_MICFAILURE_STAKEY`](#iw_micfailure_stakey)
- [`IW_MLME_ASSOC`](#iw_mlme_assoc)
- [`IW_MLME_AUTH`](#iw_mlme_auth)
- [`IW_MLME_DEAUTH`](#iw_mlme_deauth)
- [`IW_MLME_DISASSOC`](#iw_mlme_disassoc)
- [`IW_MODE_ADHOC`](#iw_mode_adhoc)
- [`IW_MODE_AUTO`](#iw_mode_auto)
- [`IW_MODE_INFRA`](#iw_mode_infra)
- [`IW_MODE_MASTER`](#iw_mode_master)
- [`IW_MODE_MESH`](#iw_mode_mesh)
- [`IW_MODE_MONITOR`](#iw_mode_monitor)
- [`IW_MODE_REPEAT`](#iw_mode_repeat)
- [`IW_MODE_SECOND`](#iw_mode_second)
- [`IW_PMKID_CAND_PREAUTH`](#iw_pmkid_cand_preauth)
- [`IW_PMKID_LEN`](#iw_pmkid_len)
- [`IW_PMKSA_ADD`](#iw_pmksa_add)
- [`IW_PMKSA_FLUSH`](#iw_pmksa_flush)
- [`IW_PMKSA_REMOVE`](#iw_pmksa_remove)
- [`IW_POWER_ALL_R`](#iw_power_all_r)
- [`IW_POWER_FORCE_S`](#iw_power_force_s)
- [`IW_POWER_MAX`](#iw_power_max)
- [`IW_POWER_MIN`](#iw_power_min)
- [`IW_POWER_MODE`](#iw_power_mode)
- [`IW_POWER_MODIFIER`](#iw_power_modifier)
- [`IW_POWER_MULTICAST_R`](#iw_power_multicast_r)
- [`IW_POWER_ON`](#iw_power_on)
- [`IW_POWER_PERIOD`](#iw_power_period)
- [`IW_POWER_RELATIVE`](#iw_power_relative)
- [`IW_POWER_REPEATER`](#iw_power_repeater)
- [`IW_POWER_TIMEOUT`](#iw_power_timeout)
- [`IW_POWER_TYPE`](#iw_power_type)
- [`IW_POWER_UNICAST_R`](#iw_power_unicast_r)
- [`IW_PRIV_SIZE_FIXED`](#iw_priv_size_fixed)
- [`IW_PRIV_SIZE_MASK`](#iw_priv_size_mask)
- [`IW_PRIV_TYPE_ADDR`](#iw_priv_type_addr)
- [`IW_PRIV_TYPE_BYTE`](#iw_priv_type_byte)
- [`IW_PRIV_TYPE_CHAR`](#iw_priv_type_char)
- [`IW_PRIV_TYPE_FLOAT`](#iw_priv_type_float)
- [`IW_PRIV_TYPE_INT`](#iw_priv_type_int)
- [`IW_PRIV_TYPE_MASK`](#iw_priv_type_mask)
- [`IW_PRIV_TYPE_NONE`](#iw_priv_type_none)
- [`IW_QUAL_ALL_INVALID`](#iw_qual_all_invalid)
- [`IW_QUAL_ALL_UPDATED`](#iw_qual_all_updated)
- [`IW_QUAL_DBM`](#iw_qual_dbm)
- [`IW_QUAL_LEVEL_INVALID`](#iw_qual_level_invalid)
- [`IW_QUAL_LEVEL_UPDATED`](#iw_qual_level_updated)
- [`IW_QUAL_NOISE_INVALID`](#iw_qual_noise_invalid)
- [`IW_QUAL_NOISE_UPDATED`](#iw_qual_noise_updated)
- [`IW_QUAL_QUAL_INVALID`](#iw_qual_qual_invalid)
- [`IW_QUAL_QUAL_UPDATED`](#iw_qual_qual_updated)
- [`IW_QUAL_RCPI`](#iw_qual_rcpi)
- [`IW_RETRY_LIFETIME`](#iw_retry_lifetime)
- [`IW_RETRY_LIMIT`](#iw_retry_limit)
- [`IW_RETRY_LONG`](#iw_retry_long)
- [`IW_RETRY_MAX`](#iw_retry_max)
- [`IW_RETRY_MIN`](#iw_retry_min)
- [`IW_RETRY_MODIFIER`](#iw_retry_modifier)
- [`IW_RETRY_ON`](#iw_retry_on)
- [`IW_RETRY_RELATIVE`](#iw_retry_relative)
- [`IW_RETRY_SHORT`](#iw_retry_short)
- [`IW_RETRY_TYPE`](#iw_retry_type)
- [`IW_SCAN_ALL_ESSID`](#iw_scan_all_essid)
- [`IW_SCAN_ALL_FREQ`](#iw_scan_all_freq)
- [`IW_SCAN_ALL_MODE`](#iw_scan_all_mode)
- [`IW_SCAN_ALL_RATE`](#iw_scan_all_rate)
- [`IW_SCAN_CAPA_BSSID`](#iw_scan_capa_bssid)
- [`IW_SCAN_CAPA_CHANNEL`](#iw_scan_capa_channel)
- [`IW_SCAN_CAPA_ESSID`](#iw_scan_capa_essid)
- [`IW_SCAN_CAPA_MODE`](#iw_scan_capa_mode)
- [`IW_SCAN_CAPA_NONE`](#iw_scan_capa_none)
- [`IW_SCAN_CAPA_RATE`](#iw_scan_capa_rate)
- [`IW_SCAN_CAPA_TIME`](#iw_scan_capa_time)
- [`IW_SCAN_CAPA_TYPE`](#iw_scan_capa_type)
- [`IW_SCAN_DEFAULT`](#iw_scan_default)
- [`IW_SCAN_MAX_DATA`](#iw_scan_max_data)
- [`IW_SCAN_THIS_ESSID`](#iw_scan_this_essid)
- [`IW_SCAN_THIS_FREQ`](#iw_scan_this_freq)
- [`IW_SCAN_THIS_MODE`](#iw_scan_this_mode)
- [`IW_SCAN_THIS_RATE`](#iw_scan_this_rate)
- [`IW_SCAN_TYPE_ACTIVE`](#iw_scan_type_active)
- [`IW_SCAN_TYPE_PASSIVE`](#iw_scan_type_passive)
- [`IW_TXPOW_DBM`](#iw_txpow_dbm)
- [`IW_TXPOW_MWATT`](#iw_txpow_mwatt)
- [`IW_TXPOW_RANGE`](#iw_txpow_range)
- [`IW_TXPOW_RELATIVE`](#iw_txpow_relative)
- [`IW_TXPOW_TYPE`](#iw_txpow_type)
- [`KERN_ACCT`](#kern_acct)
- [`KERN_ACPI_VIDEO_FLAGS`](#kern_acpi_video_flags)
- [`KERN_BOOTLOADER_TYPE`](#kern_bootloader_type)
- [`KERN_CADPID`](#kern_cadpid)
- [`KERN_COMPAT_LOG`](#kern_compat_log)
- [`KERN_CORE_PATTERN`](#kern_core_pattern)
- [`KERN_CORE_USES_PID`](#kern_core_uses_pid)
- [`KERN_CTLALTDEL`](#kern_ctlaltdel)
- [`KERN_DOMAINNAME`](#kern_domainname)
- [`KERN_HOTPLUG`](#kern_hotplug)
- [`KERN_HPPA_PWRSW`](#kern_hppa_pwrsw)
- [`KERN_HPPA_UNALIGNED`](#kern_hppa_unaligned)
- [`KERN_HZ_TIMER`](#kern_hz_timer)
- [`KERN_IA64_UNALIGNED`](#kern_ia64_unaligned)
- [`KERN_IEEE_EMULATION_WARNINGS`](#kern_ieee_emulation_warnings)
- [`KERN_MAX_LOCK_DEPTH`](#kern_max_lock_depth)
- [`KERN_MAX_THREADS`](#kern_max_threads)
- [`KERN_MODPROBE`](#kern_modprobe)
- [`KERN_MSGMAX`](#kern_msgmax)
- [`KERN_MSGMNB`](#kern_msgmnb)
- [`KERN_MSGMNI`](#kern_msgmni)
- [`KERN_MSGPOOL`](#kern_msgpool)
- [`KERN_NAMETRANS`](#kern_nametrans)
- [`KERN_NGROUPS_MAX`](#kern_ngroups_max)
- [`KERN_NMI_WATCHDOG`](#kern_nmi_watchdog)
- [`KERN_NODENAME`](#kern_nodename)
- [`KERN_OSRELEASE`](#kern_osrelease)
- [`KERN_OSREV`](#kern_osrev)
- [`KERN_OSTYPE`](#kern_ostype)
- [`KERN_OVERFLOWGID`](#kern_overflowgid)
- [`KERN_OVERFLOWUID`](#kern_overflowuid)
- [`KERN_PANIC`](#kern_panic)
- [`KERN_PANIC_ON_NMI`](#kern_panic_on_nmi)
- [`KERN_PANIC_ON_OOPS`](#kern_panic_on_oops)
- [`KERN_PIDMAX`](#kern_pidmax)
- [`KERN_PPC_HTABRECLAIM`](#kern_ppc_htabreclaim)
- [`KERN_PPC_L2CR`](#kern_ppc_l2cr)
- [`KERN_PPC_POWERSAVE_NAP`](#kern_ppc_powersave_nap)
- [`KERN_PPC_ZEROPAGED`](#kern_ppc_zeropaged)
- [`KERN_PRINTK`](#kern_printk)
- [`KERN_PRINTK_RATELIMIT`](#kern_printk_ratelimit)
- [`KERN_PRINTK_RATELIMIT_BURST`](#kern_printk_ratelimit_burst)
- [`KERN_PROF`](#kern_prof)
- [`KERN_PTY`](#kern_pty)
- [`KERN_RANDOM`](#kern_random)
- [`KERN_RANDOMIZE`](#kern_randomize)
- [`KERN_REALROOTDEV`](#kern_realrootdev)
- [`KERN_RTSIGMAX`](#kern_rtsigmax)
- [`KERN_RTSIGNR`](#kern_rtsignr)
- [`KERN_S390_USER_DEBUG_LOGGING`](#kern_s390_user_debug_logging)
- [`KERN_SECUREMASK`](#kern_securemask)
- [`KERN_SEM`](#kern_sem)
- [`KERN_SETUID_DUMPABLE`](#kern_setuid_dumpable)
- [`KERN_SG_BIG_BUFF`](#kern_sg_big_buff)
- [`KERN_SHMALL`](#kern_shmall)
- [`KERN_SHMMAX`](#kern_shmmax)
- [`KERN_SHMMNI`](#kern_shmmni)
- [`KERN_SHMPATH`](#kern_shmpath)
- [`KERN_SPARC_REBOOT`](#kern_sparc_reboot)
- [`KERN_SPARC_SCONS_PWROFF`](#kern_sparc_scons_pwroff)
- [`KERN_SPARC_STOP_A`](#kern_sparc_stop_a)
- [`KERN_SPIN_RETRY`](#kern_spin_retry)
- [`KERN_SYSRQ`](#kern_sysrq)
- [`KERN_TAINTED`](#kern_tainted)
- [`KERN_UNKNOWN_NMI_PANIC`](#kern_unknown_nmi_panic)
- [`KERN_VERSION`](#kern_version)
- [`KEXEC_ARCH_MASK`](#kexec_arch_mask)
- [`KEXEC_FILE_NO_INITRAMFS`](#kexec_file_no_initramfs)
- [`KEXEC_FILE_ON_CRASH`](#kexec_file_on_crash)
- [`KEXEC_FILE_UNLOAD`](#kexec_file_unload)
- [`KEXEC_ON_CRASH`](#kexec_on_crash)
- [`KEXEC_PRESERVE_CONTEXT`](#kexec_preserve_context)
- [`KEY_CNT`](#key_cnt)
- [`KEY_MAX`](#key_max)
- [`LED_CNT`](#led_cnt)
- [`LED_MAX`](#led_max)
- [`LINUX_REBOOT_CMD_CAD_OFF`](#linux_reboot_cmd_cad_off)
- [`LINUX_REBOOT_CMD_CAD_ON`](#linux_reboot_cmd_cad_on)
- [`LINUX_REBOOT_CMD_HALT`](#linux_reboot_cmd_halt)
- [`LINUX_REBOOT_CMD_KEXEC`](#linux_reboot_cmd_kexec)
- [`LINUX_REBOOT_CMD_POWER_OFF`](#linux_reboot_cmd_power_off)
- [`LINUX_REBOOT_CMD_RESTART`](#linux_reboot_cmd_restart)
- [`LINUX_REBOOT_CMD_RESTART2`](#linux_reboot_cmd_restart2)
- [`LINUX_REBOOT_CMD_SW_SUSPEND`](#linux_reboot_cmd_sw_suspend)
- [`LINUX_REBOOT_MAGIC1`](#linux_reboot_magic1)
- [`LINUX_REBOOT_MAGIC2`](#linux_reboot_magic2)
- [`LINUX_REBOOT_MAGIC2A`](#linux_reboot_magic2a)
- [`LINUX_REBOOT_MAGIC2B`](#linux_reboot_magic2b)
- [`LINUX_REBOOT_MAGIC2C`](#linux_reboot_magic2c)
- [`MAP_DROPPABLE`](#map_droppable)
- [`MAP_SHARED_VALIDATE`](#map_shared_validate)
- [`MAX_HANDLE_SZ`](#max_handle_sz)
- [`MNT_NS_INFO_SIZE_VER0`](#mnt_ns_info_size_ver0)
- [`MODULE_INIT_IGNORE_MODVERSIONS`](#module_init_ignore_modversions)
- [`MODULE_INIT_IGNORE_VERMAGIC`](#module_init_ignore_vermagic)
- [`MOUNT_ATTR_IDMAP`](#mount_attr_idmap)
- [`MOUNT_ATTR_NOATIME`](#mount_attr_noatime)
- [`MOUNT_ATTR_NODEV`](#mount_attr_nodev)
- [`MOUNT_ATTR_NODIRATIME`](#mount_attr_nodiratime)
- [`MOUNT_ATTR_NOEXEC`](#mount_attr_noexec)
- [`MOUNT_ATTR_NOSUID`](#mount_attr_nosuid)
- [`MOUNT_ATTR_NOSYMFOLLOW`](#mount_attr_nosymfollow)
- [`MOUNT_ATTR_RDONLY`](#mount_attr_rdonly)
- [`MOUNT_ATTR_RELATIME`](#mount_attr_relatime)
- [`MOUNT_ATTR_SIZE_VER0`](#mount_attr_size_ver0)
- [`MOUNT_ATTR_STRICTATIME`](#mount_attr_strictatime)
- [`MOUNT_ATTR__ATIME`](#mount_attr__atime)
- [`MPOL_BIND`](#mpol_bind)
- [`MPOL_DEFAULT`](#mpol_default)
- [`MPOL_F_NUMA_BALANCING`](#mpol_f_numa_balancing)
- [`MPOL_F_RELATIVE_NODES`](#mpol_f_relative_nodes)
- [`MPOL_F_STATIC_NODES`](#mpol_f_static_nodes)
- [`MPOL_INTERLEAVE`](#mpol_interleave)
- [`MPOL_LOCAL`](#mpol_local)
- [`MPOL_PREFERRED`](#mpol_preferred)
- [`MREMAP_DONTUNMAP`](#mremap_dontunmap)
- [`MREMAP_FIXED`](#mremap_fixed)
- [`MREMAP_MAYMOVE`](#mremap_maymove)
- [`MSC_CNT`](#msc_cnt)
- [`MSC_MAX`](#msc_max)
- [`MSG_EXCEPT`](#msg_except)
- [`MSG_INFO`](#msg_info)
- [`MSG_NOERROR`](#msg_noerror)
- [`MSG_NOTIFICATION`](#msg_notification)
- [`MSG_STAT`](#msg_stat)
- [`MSG_ZEROCOPY`](#msg_zerocopy)
- [`NDA_CACHEINFO`](#nda_cacheinfo)
- [`NDA_DST`](#nda_dst)
- [`NDA_IFINDEX`](#nda_ifindex)
- [`NDA_LLADDR`](#nda_lladdr)
- [`NDA_PORT`](#nda_port)
- [`NDA_PROBES`](#nda_probes)
- [`NDA_UNSPEC`](#nda_unspec)
- [`NDA_VLAN`](#nda_vlan)
- [`NDA_VNI`](#nda_vni)
- [`NET_802`](#net_802)
- [`NET_ATALK`](#net_atalk)
- [`NET_AX25`](#net_ax25)
- [`NET_BRIDGE`](#net_bridge)
- [`NET_CORE`](#net_core)
- [`NET_DCCP`](#net_dccp)
- [`NET_DECNET`](#net_decnet)
- [`NET_ECONET`](#net_econet)
- [`NET_ETHER`](#net_ether)
- [`NET_IPV4`](#net_ipv4)
- [`NET_IPV6`](#net_ipv6)
- [`NET_IPX`](#net_ipx)
- [`NET_IRDA`](#net_irda)
- [`NET_LLC`](#net_llc)
- [`NET_NETFILTER`](#net_netfilter)
- [`NET_NETROM`](#net_netrom)
- [`NET_ROSE`](#net_rose)
- [`NET_SCTP`](#net_sctp)
- [`NET_TR`](#net_tr)
- [`NET_UNIX`](#net_unix)
- [`NET_X25`](#net_x25)
- [`NFNETLINK_V0`](#nfnetlink_v0)
- [`NFNLGRP_ACCT_QUOTA`](#nfnlgrp_acct_quota)
- [`NFNLGRP_CONNTRACK_DESTROY`](#nfnlgrp_conntrack_destroy)
- [`NFNLGRP_CONNTRACK_EXP_DESTROY`](#nfnlgrp_conntrack_exp_destroy)
- [`NFNLGRP_CONNTRACK_EXP_NEW`](#nfnlgrp_conntrack_exp_new)
- [`NFNLGRP_CONNTRACK_EXP_UPDATE`](#nfnlgrp_conntrack_exp_update)
- [`NFNLGRP_CONNTRACK_NEW`](#nfnlgrp_conntrack_new)
- [`NFNLGRP_CONNTRACK_UPDATE`](#nfnlgrp_conntrack_update)
- [`NFNLGRP_NFTABLES`](#nfnlgrp_nftables)
- [`NFNLGRP_NFTRACE`](#nfnlgrp_nftrace)
- [`NFNLGRP_NONE`](#nfnlgrp_none)
- [`NFNL_BATCH_GENID`](#nfnl_batch_genid)
- [`NFNL_BATCH_UNSPEC`](#nfnl_batch_unspec)
- [`NFNL_MSG_BATCH_BEGIN`](#nfnl_msg_batch_begin)
- [`NFNL_MSG_BATCH_END`](#nfnl_msg_batch_end)
- [`NFNL_SUBSYS_ACCT`](#nfnl_subsys_acct)
- [`NFNL_SUBSYS_COUNT`](#nfnl_subsys_count)
- [`NFNL_SUBSYS_CTHELPER`](#nfnl_subsys_cthelper)
- [`NFNL_SUBSYS_CTNETLINK`](#nfnl_subsys_ctnetlink)
- [`NFNL_SUBSYS_CTNETLINK_EXP`](#nfnl_subsys_ctnetlink_exp)
- [`NFNL_SUBSYS_CTNETLINK_TIMEOUT`](#nfnl_subsys_ctnetlink_timeout)
- [`NFNL_SUBSYS_HOOK`](#nfnl_subsys_hook)
- [`NFNL_SUBSYS_IPSET`](#nfnl_subsys_ipset)
- [`NFNL_SUBSYS_NFTABLES`](#nfnl_subsys_nftables)
- [`NFNL_SUBSYS_NFT_COMPAT`](#nfnl_subsys_nft_compat)
- [`NFNL_SUBSYS_NONE`](#nfnl_subsys_none)
- [`NFNL_SUBSYS_OSF`](#nfnl_subsys_osf)
- [`NFNL_SUBSYS_QUEUE`](#nfnl_subsys_queue)
- [`NFNL_SUBSYS_ULOG`](#nfnl_subsys_ulog)
- [`NFPROTO_ARP`](#nfproto_arp)
- [`NFPROTO_BRIDGE`](#nfproto_bridge)
- [`NFPROTO_DECNET`](#nfproto_decnet)
- [`NFPROTO_INET`](#nfproto_inet)
- [`NFPROTO_IPV4`](#nfproto_ipv4)
- [`NFPROTO_IPV6`](#nfproto_ipv6)
- [`NFPROTO_NETDEV`](#nfproto_netdev)
- [`NFPROTO_NUMPROTO`](#nfproto_numproto)
- [`NFPROTO_UNSPEC`](#nfproto_unspec)
- [`NFQA_CAP_LEN`](#nfqa_cap_len)
- [`NFQA_CFG_CMD`](#nfqa_cfg_cmd)
- [`NFQA_CFG_FLAGS`](#nfqa_cfg_flags)
- [`NFQA_CFG_F_CONNTRACK`](#nfqa_cfg_f_conntrack)
- [`NFQA_CFG_F_FAIL_OPEN`](#nfqa_cfg_f_fail_open)
- [`NFQA_CFG_F_GSO`](#nfqa_cfg_f_gso)
- [`NFQA_CFG_F_MAX`](#nfqa_cfg_f_max)
- [`NFQA_CFG_F_SECCTX`](#nfqa_cfg_f_secctx)
- [`NFQA_CFG_F_UID_GID`](#nfqa_cfg_f_uid_gid)
- [`NFQA_CFG_MASK`](#nfqa_cfg_mask)
- [`NFQA_CFG_PARAMS`](#nfqa_cfg_params)
- [`NFQA_CFG_QUEUE_MAXLEN`](#nfqa_cfg_queue_maxlen)
- [`NFQA_CFG_UNSPEC`](#nfqa_cfg_unspec)
- [`NFQA_CT`](#nfqa_ct)
- [`NFQA_CT_INFO`](#nfqa_ct_info)
- [`NFQA_EXP`](#nfqa_exp)
- [`NFQA_GID`](#nfqa_gid)
- [`NFQA_HWADDR`](#nfqa_hwaddr)
- [`NFQA_IFINDEX_INDEV`](#nfqa_ifindex_indev)
- [`NFQA_IFINDEX_OUTDEV`](#nfqa_ifindex_outdev)
- [`NFQA_IFINDEX_PHYSINDEV`](#nfqa_ifindex_physindev)
- [`NFQA_IFINDEX_PHYSOUTDEV`](#nfqa_ifindex_physoutdev)
- [`NFQA_L2HDR`](#nfqa_l2hdr)
- [`NFQA_MARK`](#nfqa_mark)
- [`NFQA_PACKET_HDR`](#nfqa_packet_hdr)
- [`NFQA_PAYLOAD`](#nfqa_payload)
- [`NFQA_PRIORITY`](#nfqa_priority)
- [`NFQA_SECCTX`](#nfqa_secctx)
- [`NFQA_SKB_CSUMNOTREADY`](#nfqa_skb_csumnotready)
- [`NFQA_SKB_CSUM_NOTVERIFIED`](#nfqa_skb_csum_notverified)
- [`NFQA_SKB_GSO`](#nfqa_skb_gso)
- [`NFQA_SKB_INFO`](#nfqa_skb_info)
- [`NFQA_TIMESTAMP`](#nfqa_timestamp)
- [`NFQA_UID`](#nfqa_uid)
- [`NFQA_UNSPEC`](#nfqa_unspec)
- [`NFQA_VERDICT_HDR`](#nfqa_verdict_hdr)
- [`NFQA_VLAN`](#nfqa_vlan)
- [`NFQA_VLAN_PROTO`](#nfqa_vlan_proto)
- [`NFQA_VLAN_TCI`](#nfqa_vlan_tci)
- [`NFQA_VLAN_UNSPEC`](#nfqa_vlan_unspec)
- [`NFQNL_CFG_CMD_BIND`](#nfqnl_cfg_cmd_bind)
- [`NFQNL_CFG_CMD_NONE`](#nfqnl_cfg_cmd_none)
- [`NFQNL_CFG_CMD_PF_BIND`](#nfqnl_cfg_cmd_pf_bind)
- [`NFQNL_CFG_CMD_PF_UNBIND`](#nfqnl_cfg_cmd_pf_unbind)
- [`NFQNL_CFG_CMD_UNBIND`](#nfqnl_cfg_cmd_unbind)
- [`NFQNL_COPY_META`](#nfqnl_copy_meta)
- [`NFQNL_COPY_NONE`](#nfqnl_copy_none)
- [`NFQNL_COPY_PACKET`](#nfqnl_copy_packet)
- [`NFQNL_MSG_CONFIG`](#nfqnl_msg_config)
- [`NFQNL_MSG_PACKET`](#nfqnl_msg_packet)
- [`NFQNL_MSG_VERDICT`](#nfqnl_msg_verdict)
- [`NFQNL_MSG_VERDICT_BATCH`](#nfqnl_msg_verdict_batch)
- [`NFT_BREAK`](#nft_break)
- [`NFT_BYTEORDER_HTON`](#nft_byteorder_hton)
- [`NFT_BYTEORDER_NTOH`](#nft_byteorder_ntoh)
- [`NFT_CHAIN_MAXNAMELEN`](#nft_chain_maxnamelen)
- [`NFT_CMP_EQ`](#nft_cmp_eq)
- [`NFT_CMP_GT`](#nft_cmp_gt)
- [`NFT_CMP_GTE`](#nft_cmp_gte)
- [`NFT_CMP_LT`](#nft_cmp_lt)
- [`NFT_CMP_LTE`](#nft_cmp_lte)
- [`NFT_CMP_NEQ`](#nft_cmp_neq)
- [`NFT_CONTINUE`](#nft_continue)
- [`NFT_CT_AVGPKT`](#nft_ct_avgpkt)
- [`NFT_CT_BYTES`](#nft_ct_bytes)
- [`NFT_CT_DIRECTION`](#nft_ct_direction)
- [`NFT_CT_DST`](#nft_ct_dst)
- [`NFT_CT_DST_IP`](#nft_ct_dst_ip)
- [`NFT_CT_DST_IP6`](#nft_ct_dst_ip6)
- [`NFT_CT_EVENTMASK`](#nft_ct_eventmask)
- [`NFT_CT_EXPIRATION`](#nft_ct_expiration)
- [`NFT_CT_HELPER`](#nft_ct_helper)
- [`NFT_CT_L3PROTOCOL`](#nft_ct_l3protocol)
- [`NFT_CT_LABELS`](#nft_ct_labels)
- [`NFT_CT_MARK`](#nft_ct_mark)
- [`NFT_CT_PKTS`](#nft_ct_pkts)
- [`NFT_CT_PROTOCOL`](#nft_ct_protocol)
- [`NFT_CT_PROTO_DST`](#nft_ct_proto_dst)
- [`NFT_CT_PROTO_SRC`](#nft_ct_proto_src)
- [`NFT_CT_SECMARK`](#nft_ct_secmark)
- [`NFT_CT_SRC`](#nft_ct_src)
- [`NFT_CT_SRC_IP`](#nft_ct_src_ip)
- [`NFT_CT_SRC_IP6`](#nft_ct_src_ip6)
- [`NFT_CT_STATE`](#nft_ct_state)
- [`NFT_CT_STATUS`](#nft_ct_status)
- [`NFT_CT_ZONE`](#nft_ct_zone)
- [`NFT_DATA_RESERVED_MASK`](#nft_data_reserved_mask)
- [`NFT_DATA_VALUE`](#nft_data_value)
- [`NFT_DATA_VALUE_MAXLEN`](#nft_data_value_maxlen)
- [`NFT_DATA_VERDICT`](#nft_data_verdict)
- [`NFT_DYNSET_F_INV`](#nft_dynset_f_inv)
- [`NFT_DYNSET_OP_ADD`](#nft_dynset_op_add)
- [`NFT_DYNSET_OP_UPDATE`](#nft_dynset_op_update)
- [`NFT_GOTO`](#nft_goto)
- [`NFT_JUMP`](#nft_jump)
- [`NFT_LIMIT_F_INV`](#nft_limit_f_inv)
- [`NFT_LIMIT_PKTS`](#nft_limit_pkts)
- [`NFT_LIMIT_PKT_BYTES`](#nft_limit_pkt_bytes)
- [`NFT_LOOKUP_F_INV`](#nft_lookup_f_inv)
- [`NFT_META_BRI_IIFNAME`](#nft_meta_bri_iifname)
- [`NFT_META_BRI_OIFNAME`](#nft_meta_bri_oifname)
- [`NFT_META_CGROUP`](#nft_meta_cgroup)
- [`NFT_META_CPU`](#nft_meta_cpu)
- [`NFT_META_IIF`](#nft_meta_iif)
- [`NFT_META_IIFGROUP`](#nft_meta_iifgroup)
- [`NFT_META_IIFNAME`](#nft_meta_iifname)
- [`NFT_META_IIFTYPE`](#nft_meta_iiftype)
- [`NFT_META_L4PROTO`](#nft_meta_l4proto)
- [`NFT_META_LEN`](#nft_meta_len)
- [`NFT_META_MARK`](#nft_meta_mark)
- [`NFT_META_NFPROTO`](#nft_meta_nfproto)
- [`NFT_META_NFTRACE`](#nft_meta_nftrace)
- [`NFT_META_OIF`](#nft_meta_oif)
- [`NFT_META_OIFGROUP`](#nft_meta_oifgroup)
- [`NFT_META_OIFNAME`](#nft_meta_oifname)
- [`NFT_META_OIFTYPE`](#nft_meta_oiftype)
- [`NFT_META_PKTTYPE`](#nft_meta_pkttype)
- [`NFT_META_PRANDOM`](#nft_meta_prandom)
- [`NFT_META_PRIORITY`](#nft_meta_priority)
- [`NFT_META_PROTOCOL`](#nft_meta_protocol)
- [`NFT_META_RTCLASSID`](#nft_meta_rtclassid)
- [`NFT_META_SECMARK`](#nft_meta_secmark)
- [`NFT_META_SKGID`](#nft_meta_skgid)
- [`NFT_META_SKUID`](#nft_meta_skuid)
- [`NFT_MSG_DELCHAIN`](#nft_msg_delchain)
- [`NFT_MSG_DELOBJ`](#nft_msg_delobj)
- [`NFT_MSG_DELRULE`](#nft_msg_delrule)
- [`NFT_MSG_DELSET`](#nft_msg_delset)
- [`NFT_MSG_DELSETELEM`](#nft_msg_delsetelem)
- [`NFT_MSG_DELTABLE`](#nft_msg_deltable)
- [`NFT_MSG_GETCHAIN`](#nft_msg_getchain)
- [`NFT_MSG_GETGEN`](#nft_msg_getgen)
- [`NFT_MSG_GETOBJ`](#nft_msg_getobj)
- [`NFT_MSG_GETOBJ_RESET`](#nft_msg_getobj_reset)
- [`NFT_MSG_GETRULE`](#nft_msg_getrule)
- [`NFT_MSG_GETSET`](#nft_msg_getset)
- [`NFT_MSG_GETSETELEM`](#nft_msg_getsetelem)
- [`NFT_MSG_GETTABLE`](#nft_msg_gettable)
- [`NFT_MSG_MAX`](#nft_msg_max)
- [`NFT_MSG_NEWCHAIN`](#nft_msg_newchain)
- [`NFT_MSG_NEWGEN`](#nft_msg_newgen)
- [`NFT_MSG_NEWOBJ`](#nft_msg_newobj)
- [`NFT_MSG_NEWRULE`](#nft_msg_newrule)
- [`NFT_MSG_NEWSET`](#nft_msg_newset)
- [`NFT_MSG_NEWSETELEM`](#nft_msg_newsetelem)
- [`NFT_MSG_NEWTABLE`](#nft_msg_newtable)
- [`NFT_MSG_TRACE`](#nft_msg_trace)
- [`NFT_NAT_DNAT`](#nft_nat_dnat)
- [`NFT_NAT_SNAT`](#nft_nat_snat)
- [`NFT_NG_INCREMENTAL`](#nft_ng_incremental)
- [`NFT_NG_RANDOM`](#nft_ng_random)
- [`NFT_OBJ_MAXNAMELEN`](#nft_obj_maxnamelen)
- [`NFT_PAYLOAD_CSUM_INET`](#nft_payload_csum_inet)
- [`NFT_PAYLOAD_CSUM_NONE`](#nft_payload_csum_none)
- [`NFT_PAYLOAD_LL_HEADER`](#nft_payload_ll_header)
- [`NFT_PAYLOAD_NETWORK_HEADER`](#nft_payload_network_header)
- [`NFT_PAYLOAD_TRANSPORT_HEADER`](#nft_payload_transport_header)
- [`NFT_QUEUE_FLAG_BYPASS`](#nft_queue_flag_bypass)
- [`NFT_QUEUE_FLAG_CPU_FANOUT`](#nft_queue_flag_cpu_fanout)
- [`NFT_QUEUE_FLAG_MASK`](#nft_queue_flag_mask)
- [`NFT_QUOTA_F_INV`](#nft_quota_f_inv)
- [`NFT_RANGE_EQ`](#nft_range_eq)
- [`NFT_RANGE_NEQ`](#nft_range_neq)
- [`NFT_REG32_00`](#nft_reg32_00)
- [`NFT_REG32_01`](#nft_reg32_01)
- [`NFT_REG32_02`](#nft_reg32_02)
- [`NFT_REG32_03`](#nft_reg32_03)
- [`NFT_REG32_04`](#nft_reg32_04)
- [`NFT_REG32_05`](#nft_reg32_05)
- [`NFT_REG32_06`](#nft_reg32_06)
- [`NFT_REG32_07`](#nft_reg32_07)
- [`NFT_REG32_08`](#nft_reg32_08)
- [`NFT_REG32_09`](#nft_reg32_09)
- [`NFT_REG32_10`](#nft_reg32_10)
- [`NFT_REG32_11`](#nft_reg32_11)
- [`NFT_REG32_12`](#nft_reg32_12)
- [`NFT_REG32_13`](#nft_reg32_13)
- [`NFT_REG32_14`](#nft_reg32_14)
- [`NFT_REG32_15`](#nft_reg32_15)
- [`NFT_REG32_SIZE`](#nft_reg32_size)
- [`NFT_REG_1`](#nft_reg_1)
- [`NFT_REG_2`](#nft_reg_2)
- [`NFT_REG_3`](#nft_reg_3)
- [`NFT_REG_4`](#nft_reg_4)
- [`NFT_REG_SIZE`](#nft_reg_size)
- [`NFT_REG_VERDICT`](#nft_reg_verdict)
- [`NFT_REJECT_ICMPX_ADMIN_PROHIBITED`](#nft_reject_icmpx_admin_prohibited)
- [`NFT_REJECT_ICMPX_HOST_UNREACH`](#nft_reject_icmpx_host_unreach)
- [`NFT_REJECT_ICMPX_NO_ROUTE`](#nft_reject_icmpx_no_route)
- [`NFT_REJECT_ICMPX_PORT_UNREACH`](#nft_reject_icmpx_port_unreach)
- [`NFT_REJECT_ICMPX_UNREACH`](#nft_reject_icmpx_unreach)
- [`NFT_REJECT_ICMP_UNREACH`](#nft_reject_icmp_unreach)
- [`NFT_REJECT_TCP_RST`](#nft_reject_tcp_rst)
- [`NFT_RETURN`](#nft_return)
- [`NFT_SET_ANONYMOUS`](#nft_set_anonymous)
- [`NFT_SET_CONSTANT`](#nft_set_constant)
- [`NFT_SET_ELEM_INTERVAL_END`](#nft_set_elem_interval_end)
- [`NFT_SET_EVAL`](#nft_set_eval)
- [`NFT_SET_INTERVAL`](#nft_set_interval)
- [`NFT_SET_MAP`](#nft_set_map)
- [`NFT_SET_MAXNAMELEN`](#nft_set_maxnamelen)
- [`NFT_SET_POL_MEMORY`](#nft_set_pol_memory)
- [`NFT_SET_POL_PERFORMANCE`](#nft_set_pol_performance)
- [`NFT_SET_TIMEOUT`](#nft_set_timeout)
- [`NFT_TABLE_MAXNAMELEN`](#nft_table_maxnamelen)
- [`NFT_TRACETYPE_POLICY`](#nft_tracetype_policy)
- [`NFT_TRACETYPE_RETURN`](#nft_tracetype_return)
- [`NFT_TRACETYPE_RULE`](#nft_tracetype_rule)
- [`NFT_TRACETYPE_UNSPEC`](#nft_tracetype_unspec)
- [`NFT_USERDATA_MAXLEN`](#nft_userdata_maxlen)
- [`NFULA_CFG_CMD`](#nfula_cfg_cmd)
- [`NFULA_CFG_FLAGS`](#nfula_cfg_flags)
- [`NFULA_CFG_MODE`](#nfula_cfg_mode)
- [`NFULA_CFG_NLBUFSIZ`](#nfula_cfg_nlbufsiz)
- [`NFULA_CFG_QTHRESH`](#nfula_cfg_qthresh)
- [`NFULA_CFG_TIMEOUT`](#nfula_cfg_timeout)
- [`NFULA_CFG_UNSPEC`](#nfula_cfg_unspec)
- [`NFULA_CT`](#nfula_ct)
- [`NFULA_CT_INFO`](#nfula_ct_info)
- [`NFULA_GID`](#nfula_gid)
- [`NFULA_HWADDR`](#nfula_hwaddr)
- [`NFULA_HWHEADER`](#nfula_hwheader)
- [`NFULA_HWLEN`](#nfula_hwlen)
- [`NFULA_HWTYPE`](#nfula_hwtype)
- [`NFULA_IFINDEX_INDEV`](#nfula_ifindex_indev)
- [`NFULA_IFINDEX_OUTDEV`](#nfula_ifindex_outdev)
- [`NFULA_IFINDEX_PHYSINDEV`](#nfula_ifindex_physindev)
- [`NFULA_IFINDEX_PHYSOUTDEV`](#nfula_ifindex_physoutdev)
- [`NFULA_L2HDR`](#nfula_l2hdr)
- [`NFULA_MARK`](#nfula_mark)
- [`NFULA_PACKET_HDR`](#nfula_packet_hdr)
- [`NFULA_PAYLOAD`](#nfula_payload)
- [`NFULA_PREFIX`](#nfula_prefix)
- [`NFULA_SEQ`](#nfula_seq)
- [`NFULA_SEQ_GLOBAL`](#nfula_seq_global)
- [`NFULA_TIMESTAMP`](#nfula_timestamp)
- [`NFULA_UID`](#nfula_uid)
- [`NFULA_UNSPEC`](#nfula_unspec)
- [`NFULA_VLAN`](#nfula_vlan)
- [`NFULA_VLAN_PROTO`](#nfula_vlan_proto)
- [`NFULA_VLAN_TCI`](#nfula_vlan_tci)
- [`NFULA_VLAN_UNSPEC`](#nfula_vlan_unspec)
- [`NFULNL_CFG_CMD_BIND`](#nfulnl_cfg_cmd_bind)
- [`NFULNL_CFG_CMD_NONE`](#nfulnl_cfg_cmd_none)
- [`NFULNL_CFG_CMD_PF_BIND`](#nfulnl_cfg_cmd_pf_bind)
- [`NFULNL_CFG_CMD_PF_UNBIND`](#nfulnl_cfg_cmd_pf_unbind)
- [`NFULNL_CFG_CMD_UNBIND`](#nfulnl_cfg_cmd_unbind)
- [`NFULNL_CFG_F_CONNTRACK`](#nfulnl_cfg_f_conntrack)
- [`NFULNL_CFG_F_SEQ`](#nfulnl_cfg_f_seq)
- [`NFULNL_CFG_F_SEQ_GLOBAL`](#nfulnl_cfg_f_seq_global)
- [`NFULNL_COPY_META`](#nfulnl_copy_meta)
- [`NFULNL_COPY_NONE`](#nfulnl_copy_none)
- [`NFULNL_COPY_PACKET`](#nfulnl_copy_packet)
- [`NFULNL_MSG_CONFIG`](#nfulnl_msg_config)
- [`NFULNL_MSG_PACKET`](#nfulnl_msg_packet)
- [`NF_ACCEPT`](#nf_accept)
- [`NF_ARP`](#nf_arp)
- [`NF_ARP_FORWARD`](#nf_arp_forward)
- [`NF_ARP_IN`](#nf_arp_in)
- [`NF_ARP_NUMHOOKS`](#nf_arp_numhooks)
- [`NF_ARP_OUT`](#nf_arp_out)
- [`NF_BR_BROUTING`](#nf_br_brouting)
- [`NF_BR_FORWARD`](#nf_br_forward)
- [`NF_BR_LOCAL_IN`](#nf_br_local_in)
- [`NF_BR_LOCAL_OUT`](#nf_br_local_out)
- [`NF_BR_NUMHOOKS`](#nf_br_numhooks)
- [`NF_BR_POST_ROUTING`](#nf_br_post_routing)
- [`NF_BR_PRE_ROUTING`](#nf_br_pre_routing)
- [`NF_BR_PRI_BRNF`](#nf_br_pri_brnf)
- [`NF_BR_PRI_FILTER_BRIDGED`](#nf_br_pri_filter_bridged)
- [`NF_BR_PRI_FILTER_OTHER`](#nf_br_pri_filter_other)
- [`NF_BR_PRI_FIRST`](#nf_br_pri_first)
- [`NF_BR_PRI_LAST`](#nf_br_pri_last)
- [`NF_BR_PRI_NAT_DST_BRIDGED`](#nf_br_pri_nat_dst_bridged)
- [`NF_BR_PRI_NAT_DST_OTHER`](#nf_br_pri_nat_dst_other)
- [`NF_BR_PRI_NAT_SRC`](#nf_br_pri_nat_src)
- [`NF_DROP`](#nf_drop)
- [`NF_INET_FORWARD`](#nf_inet_forward)
- [`NF_INET_INGRESS`](#nf_inet_ingress)
- [`NF_INET_LOCAL_IN`](#nf_inet_local_in)
- [`NF_INET_LOCAL_OUT`](#nf_inet_local_out)
- [`NF_INET_NUMHOOKS`](#nf_inet_numhooks)
- [`NF_INET_POST_ROUTING`](#nf_inet_post_routing)
- [`NF_INET_PRE_ROUTING`](#nf_inet_pre_routing)
- [`NF_IP6_FORWARD`](#nf_ip6_forward)
- [`NF_IP6_LOCAL_IN`](#nf_ip6_local_in)
- [`NF_IP6_LOCAL_OUT`](#nf_ip6_local_out)
- [`NF_IP6_NUMHOOKS`](#nf_ip6_numhooks)
- [`NF_IP6_POST_ROUTING`](#nf_ip6_post_routing)
- [`NF_IP6_PRE_ROUTING`](#nf_ip6_pre_routing)
- [`NF_IP6_PRI_CONNTRACK`](#nf_ip6_pri_conntrack)
- [`NF_IP6_PRI_CONNTRACK_DEFRAG`](#nf_ip6_pri_conntrack_defrag)
- [`NF_IP6_PRI_CONNTRACK_HELPER`](#nf_ip6_pri_conntrack_helper)
- [`NF_IP6_PRI_FILTER`](#nf_ip6_pri_filter)
- [`NF_IP6_PRI_FIRST`](#nf_ip6_pri_first)
- [`NF_IP6_PRI_LAST`](#nf_ip6_pri_last)
- [`NF_IP6_PRI_MANGLE`](#nf_ip6_pri_mangle)
- [`NF_IP6_PRI_NAT_DST`](#nf_ip6_pri_nat_dst)
- [`NF_IP6_PRI_NAT_SRC`](#nf_ip6_pri_nat_src)
- [`NF_IP6_PRI_RAW`](#nf_ip6_pri_raw)
- [`NF_IP6_PRI_RAW_BEFORE_DEFRAG`](#nf_ip6_pri_raw_before_defrag)
- [`NF_IP6_PRI_SECURITY`](#nf_ip6_pri_security)
- [`NF_IP6_PRI_SELINUX_FIRST`](#nf_ip6_pri_selinux_first)
- [`NF_IP6_PRI_SELINUX_LAST`](#nf_ip6_pri_selinux_last)
- [`NF_IP_FORWARD`](#nf_ip_forward)
- [`NF_IP_LOCAL_IN`](#nf_ip_local_in)
- [`NF_IP_LOCAL_OUT`](#nf_ip_local_out)
- [`NF_IP_NUMHOOKS`](#nf_ip_numhooks)
- [`NF_IP_POST_ROUTING`](#nf_ip_post_routing)
- [`NF_IP_PRE_ROUTING`](#nf_ip_pre_routing)
- [`NF_IP_PRI_CONNTRACK`](#nf_ip_pri_conntrack)
- [`NF_IP_PRI_CONNTRACK_CONFIRM`](#nf_ip_pri_conntrack_confirm)
- [`NF_IP_PRI_CONNTRACK_DEFRAG`](#nf_ip_pri_conntrack_defrag)
- [`NF_IP_PRI_CONNTRACK_HELPER`](#nf_ip_pri_conntrack_helper)
- [`NF_IP_PRI_FILTER`](#nf_ip_pri_filter)
- [`NF_IP_PRI_FIRST`](#nf_ip_pri_first)
- [`NF_IP_PRI_LAST`](#nf_ip_pri_last)
- [`NF_IP_PRI_MANGLE`](#nf_ip_pri_mangle)
- [`NF_IP_PRI_NAT_DST`](#nf_ip_pri_nat_dst)
- [`NF_IP_PRI_NAT_SRC`](#nf_ip_pri_nat_src)
- [`NF_IP_PRI_RAW`](#nf_ip_pri_raw)
- [`NF_IP_PRI_RAW_BEFORE_DEFRAG`](#nf_ip_pri_raw_before_defrag)
- [`NF_IP_PRI_SECURITY`](#nf_ip_pri_security)
- [`NF_IP_PRI_SELINUX_FIRST`](#nf_ip_pri_selinux_first)
- [`NF_IP_PRI_SELINUX_LAST`](#nf_ip_pri_selinux_last)
- [`NF_MAX_VERDICT`](#nf_max_verdict)
- [`NF_NETDEV_EGRESS`](#nf_netdev_egress)
- [`NF_NETDEV_INGRESS`](#nf_netdev_ingress)
- [`NF_NETDEV_NUMHOOKS`](#nf_netdev_numhooks)
- [`NF_QUEUE`](#nf_queue)
- [`NF_REPEAT`](#nf_repeat)
- [`NF_STOLEN`](#nf_stolen)
- [`NF_STOP`](#nf_stop)
- [`NF_VERDICT_BITS`](#nf_verdict_bits)
- [`NF_VERDICT_FLAG_QUEUE_BYPASS`](#nf_verdict_flag_queue_bypass)
- [`NF_VERDICT_MASK`](#nf_verdict_mask)
- [`NF_VERDICT_QBITS`](#nf_verdict_qbits)
- [`NF_VERDICT_QMASK`](#nf_verdict_qmask)
- [`NLM_F_BULK`](#nlm_f_bulk)
- [`NS_GET_MNTNS_ID`](#ns_get_mntns_id)
- [`NS_GET_NSTYPE`](#ns_get_nstype)
- [`NS_GET_OWNER_UID`](#ns_get_owner_uid)
- [`NS_GET_PARENT`](#ns_get_parent)
- [`NS_GET_PID_FROM_PIDNS`](#ns_get_pid_from_pidns)
- [`NS_GET_PID_IN_PIDNS`](#ns_get_pid_in_pidns)
- [`NS_GET_TGID_FROM_PIDNS`](#ns_get_tgid_from_pidns)
- [`NS_GET_TGID_IN_PIDNS`](#ns_get_tgid_in_pidns)
- [`NS_GET_USERNS`](#ns_get_userns)
- [`NS_MNT_GET_INFO`](#ns_mnt_get_info)
- [`NS_MNT_GET_NEXT`](#ns_mnt_get_next)
- [`NS_MNT_GET_PREV`](#ns_mnt_get_prev)
- [`NTF_MASTER`](#ntf_master)
- [`NTF_PROXY`](#ntf_proxy)
- [`NTF_ROUTER`](#ntf_router)
- [`NTF_SELF`](#ntf_self)
- [`NTF_USE`](#ntf_use)
- [`NUD_DELAY`](#nud_delay)
- [`NUD_FAILED`](#nud_failed)
- [`NUD_INCOMPLETE`](#nud_incomplete)
- [`NUD_NOARP`](#nud_noarp)
- [`NUD_NONE`](#nud_none)
- [`NUD_PERMANENT`](#nud_permanent)
- [`NUD_PROBE`](#nud_probe)
- [`NUD_REACHABLE`](#nud_reachable)
- [`NUD_STALE`](#nud_stale)
- [`OPEN_TREE_CLOEXEC`](#open_tree_cloexec)
- [`OPEN_TREE_CLONE`](#open_tree_clone)
- [`PACKET_FANOUT`](#packet_fanout)
- [`PACKET_FANOUT_CBPF`](#packet_fanout_cbpf)
- [`PACKET_FANOUT_CPU`](#packet_fanout_cpu)
- [`PACKET_FANOUT_DATA`](#packet_fanout_data)
- [`PACKET_FANOUT_EBPF`](#packet_fanout_ebpf)
- [`PACKET_FANOUT_FLAG_DEFRAG`](#packet_fanout_flag_defrag)
- [`PACKET_FANOUT_FLAG_IGNORE_OUTGOING`](#packet_fanout_flag_ignore_outgoing)
- [`PACKET_FANOUT_FLAG_ROLLOVER`](#packet_fanout_flag_rollover)
- [`PACKET_FANOUT_FLAG_UNIQUEID`](#packet_fanout_flag_uniqueid)
- [`PACKET_FANOUT_HASH`](#packet_fanout_hash)
- [`PACKET_FANOUT_LB`](#packet_fanout_lb)
- [`PACKET_FANOUT_QM`](#packet_fanout_qm)
- [`PACKET_FANOUT_RND`](#packet_fanout_rnd)
- [`PACKET_FANOUT_ROLLOVER`](#packet_fanout_rollover)
- [`PACKET_IGNORE_OUTGOING`](#packet_ignore_outgoing)
- [`PACKET_QDISC_BYPASS`](#packet_qdisc_bypass)
- [`PACKET_ROLLOVER_STATS`](#packet_rollover_stats)
- [`PACKET_TX_HAS_OFF`](#packet_tx_has_off)
- [`PACKET_VNET_HDR_SZ`](#packet_vnet_hdr_sz)
- [`PF_BLOCK_TS`](#pf_block_ts) - Plug has ts that needs updating.
- [`PF_DUMPCORE`](#pf_dumpcore) - Dumped core.
- [`PF_EXITING`](#pf_exiting) - Getting shut down.
- [`PF_FORKNOEXEC`](#pf_forknoexec) - Forked but didn't exec.
- [`PF_IDLE`](#pf_idle) - I am an IDLE thread.
- [`PF_IO_WORKER`](#pf_io_worker) - Task is an IO worker.
- [`PF_KSWAPD`](#pf_kswapd) - I am `kswapd`.
- [`PF_KTHREAD`](#pf_kthread) - I am a kernel thread.
- [`PF_LOCAL_THROTTLE`](#pf_local_throttle) - Throttle writes only against the bdi I write to, I am cleaning
- [`PF_MCE_EARLY`](#pf_mce_early) - Early kill for mce process policy.
- [`PF_MCE_PROCESS`](#pf_mce_process) - Process policy on mce errors.
- [`PF_MEMALLOC`](#pf_memalloc) - Allocating memory to free memory.
- [`PF_MEMALLOC_NOFS`](#pf_memalloc_nofs) - All allocations inherit `GFP_NOFS`.
- [`PF_MEMALLOC_NOIO`](#pf_memalloc_noio) - All allocations inherit `GFP_NOIO`.
- [`PF_MEMALLOC_PIN`](#pf_memalloc_pin) - Allocations constrained to zones which allow long term pinning.
- [`PF_NOFREEZE`](#pf_nofreeze) - This thread should not be frozen.
- [`PF_NO_SETAFFINITY`](#pf_no_setaffinity) - Userland is not allowed to meddle with `cpus_mask`.
- [`PF_NPROC_EXCEEDED`](#pf_nproc_exceeded) - `set_user()` noticed that `RLIMIT_NPROC` was exceeded.
- [`PF_POSTCOREDUMP`](#pf_postcoredump) - Coredumps should ignore this task.
- [`PF_RANDOMIZE`](#pf_randomize) - Randomize virtual address space.
- [`PF_SIGNALED`](#pf_signaled) - Killed by a signal.
- [`PF_SUPERPRIV`](#pf_superpriv) - Used super-user privileges.
- [`PF_SUSPEND_TASK`](#pf_suspend_task) - This thread called `freeze_processes()` and should not be frozen.
- [`PF_USED_MATH`](#pf_used_math) - If unset the fpu must be initialized before use.
- [`PF_USER_WORKER`](#pf_user_worker) - Kernel thread cloned from userspace thread.
- [`PF_VCPU`](#pf_vcpu) - I'm a virtual CPU.
- [`PF_WQ_WORKER`](#pf_wq_worker) - I'm a workqueue worker.
- [`PIDTYPE_MAX`](#pidtype_max)
- [`PIDTYPE_PGID`](#pidtype_pgid)
- [`PIDTYPE_PID`](#pidtype_pid)
- [`PIDTYPE_SID`](#pidtype_sid)
- [`PIDTYPE_TGID`](#pidtype_tgid)
- [`POSIX_SPAWN_RESETIDS`](#posix_spawn_resetids)
- [`POSIX_SPAWN_SETPGROUP`](#posix_spawn_setpgroup)
- [`POSIX_SPAWN_SETSCHEDPARAM`](#posix_spawn_setschedparam)
- [`POSIX_SPAWN_SETSCHEDULER`](#posix_spawn_setscheduler)
- [`POSIX_SPAWN_SETSID`](#posix_spawn_setsid)
- [`POSIX_SPAWN_SETSIGDEF`](#posix_spawn_setsigdef)
- [`POSIX_SPAWN_SETSIGMASK`](#posix_spawn_setsigmask)
- [`POSIX_SPAWN_USEVFORK`](#posix_spawn_usevfork)
- [`PROC_CN_MCAST_IGNORE`](#proc_cn_mcast_ignore)
- [`PROC_CN_MCAST_LISTEN`](#proc_cn_mcast_listen)
- [`PROC_EVENT_COMM`](#proc_event_comm)
- [`PROC_EVENT_COREDUMP`](#proc_event_coredump)
- [`PROC_EVENT_EXEC`](#proc_event_exec)
- [`PROC_EVENT_EXIT`](#proc_event_exit)
- [`PROC_EVENT_FORK`](#proc_event_fork)
- [`PROC_EVENT_GID`](#proc_event_gid)
- [`PROC_EVENT_NONE`](#proc_event_none)
- [`PROC_EVENT_NONZERO_EXIT`](#proc_event_nonzero_exit)
- [`PROC_EVENT_PTRACE`](#proc_event_ptrace)
- [`PROC_EVENT_SID`](#proc_event_sid)
- [`PROC_EVENT_UID`](#proc_event_uid)
- [`PR_GET_MDWE`](#pr_get_mdwe)
- [`PR_MDWE_NO_INHERIT`](#pr_mdwe_no_inherit)
- [`PR_MDWE_REFUSE_EXEC_GAIN`](#pr_mdwe_refuse_exec_gain)
- [`PR_SET_MDWE`](#pr_set_mdwe)
- [`PTHREAD_COND_INITIALIZER`](#pthread_cond_initializer)
- [`PTHREAD_MUTEX_INITIALIZER`](#pthread_mutex_initializer)
- [`PTHREAD_RWLOCK_INITIALIZER`](#pthread_rwlock_initializer)
- [`PTP_CLOCK_GETCAPS`](#ptp_clock_getcaps)
- [`PTP_CLOCK_GETCAPS2`](#ptp_clock_getcaps2)
- [`PTP_ENABLE_PPS`](#ptp_enable_pps)
- [`PTP_ENABLE_PPS2`](#ptp_enable_pps2)
- [`PTP_EXTTS_REQUEST`](#ptp_extts_request)
- [`PTP_EXTTS_REQUEST2`](#ptp_extts_request2)
- [`PTP_MAX_SAMPLES`](#ptp_max_samples)
- [`PTP_PEROUT_REQUEST`](#ptp_perout_request)
- [`PTP_PEROUT_REQUEST2`](#ptp_perout_request2)
- [`PTP_PF_EXTTS`](#ptp_pf_extts)
- [`PTP_PF_NONE`](#ptp_pf_none)
- [`PTP_PF_PEROUT`](#ptp_pf_perout)
- [`PTP_PF_PHYSYNC`](#ptp_pf_physync)
- [`PTP_PIN_GETFUNC`](#ptp_pin_getfunc)
- [`PTP_PIN_GETFUNC2`](#ptp_pin_getfunc2)
- [`PTP_PIN_SETFUNC`](#ptp_pin_setfunc)
- [`PTP_PIN_SETFUNC2`](#ptp_pin_setfunc2)
- [`PTP_SYS_OFFSET`](#ptp_sys_offset)
- [`PTP_SYS_OFFSET2`](#ptp_sys_offset2)
- [`PTP_SYS_OFFSET_EXTENDED`](#ptp_sys_offset_extended)
- [`PTP_SYS_OFFSET_EXTENDED2`](#ptp_sys_offset_extended2)
- [`PTP_SYS_OFFSET_PRECISE`](#ptp_sys_offset_precise)
- [`PTP_SYS_OFFSET_PRECISE2`](#ptp_sys_offset_precise2)
- [`QFMT_VFS_OLD`](#qfmt_vfs_old)
- [`QFMT_VFS_V0`](#qfmt_vfs_v0)
- [`QFMT_VFS_V1`](#qfmt_vfs_v1)
- [`RB_AUTOBOOT`](#rb_autoboot)
- [`RB_DISABLE_CAD`](#rb_disable_cad)
- [`RB_ENABLE_CAD`](#rb_enable_cad)
- [`RB_HALT_SYSTEM`](#rb_halt_system)
- [`RB_KEXEC`](#rb_kexec)
- [`RB_POWER_OFF`](#rb_power_off)
- [`RB_SW_SUSPEND`](#rb_sw_suspend)
- [`REL_CNT`](#rel_cnt)
- [`REL_MAX`](#rel_max)
- [`RENAME_EXCHANGE`](#rename_exchange)
- [`RENAME_NOREPLACE`](#rename_noreplace)
- [`RENAME_WHITEOUT`](#rename_whiteout)
- [`REP_CNT`](#rep_cnt)
- [`REP_MAX`](#rep_max)
- [`RESOLVE_BENEATH`](#resolve_beneath)
- [`RESOLVE_CACHED`](#resolve_cached)
- [`RESOLVE_IN_ROOT`](#resolve_in_root)
- [`RESOLVE_NO_MAGICLINKS`](#resolve_no_magiclinks)
- [`RESOLVE_NO_SYMLINKS`](#resolve_no_symlinks)
- [`RESOLVE_NO_XDEV`](#resolve_no_xdev)
- [`RTA_CACHEINFO`](#rta_cacheinfo)
- [`RTA_DST`](#rta_dst)
- [`RTA_FLOW`](#rta_flow)
- [`RTA_GATEWAY`](#rta_gateway)
- [`RTA_IIF`](#rta_iif)
- [`RTA_MARK`](#rta_mark)
- [`RTA_METRICS`](#rta_metrics)
- [`RTA_MFC_STATS`](#rta_mfc_stats)
- [`RTA_MP_ALGO`](#rta_mp_algo)
- [`RTA_MULTIPATH`](#rta_multipath)
- [`RTA_OIF`](#rta_oif)
- [`RTA_PREFSRC`](#rta_prefsrc)
- [`RTA_PRIORITY`](#rta_priority)
- [`RTA_PROTOINFO`](#rta_protoinfo)
- [`RTA_SESSION`](#rta_session)
- [`RTA_SRC`](#rta_src)
- [`RTA_TABLE`](#rta_table)
- [`RTA_UNSPEC`](#rta_unspec)
- [`RTEXT_FILTER_BRVLAN`](#rtext_filter_brvlan)
- [`RTEXT_FILTER_BRVLAN_COMPRESSED`](#rtext_filter_brvlan_compressed)
- [`RTEXT_FILTER_CFM_CONFIG`](#rtext_filter_cfm_config)
- [`RTEXT_FILTER_CFM_STATUS`](#rtext_filter_cfm_status)
- [`RTEXT_FILTER_MRP`](#rtext_filter_mrp)
- [`RTEXT_FILTER_SKIP_STATS`](#rtext_filter_skip_stats)
- [`RTEXT_FILTER_VF`](#rtext_filter_vf)
- [`RTMGRP_DECnet_IFADDR`](#rtmgrp_decnet_ifaddr)
- [`RTMGRP_DECnet_ROUTE`](#rtmgrp_decnet_route)
- [`RTMGRP_IPV4_IFADDR`](#rtmgrp_ipv4_ifaddr)
- [`RTMGRP_IPV4_MROUTE`](#rtmgrp_ipv4_mroute)
- [`RTMGRP_IPV4_ROUTE`](#rtmgrp_ipv4_route)
- [`RTMGRP_IPV4_RULE`](#rtmgrp_ipv4_rule)
- [`RTMGRP_IPV6_IFADDR`](#rtmgrp_ipv6_ifaddr)
- [`RTMGRP_IPV6_IFINFO`](#rtmgrp_ipv6_ifinfo)
- [`RTMGRP_IPV6_MROUTE`](#rtmgrp_ipv6_mroute)
- [`RTMGRP_IPV6_PREFIX`](#rtmgrp_ipv6_prefix)
- [`RTMGRP_IPV6_ROUTE`](#rtmgrp_ipv6_route)
- [`RTMGRP_LINK`](#rtmgrp_link)
- [`RTMGRP_NEIGH`](#rtmgrp_neigh)
- [`RTMGRP_NOTIFY`](#rtmgrp_notify)
- [`RTMGRP_TC`](#rtmgrp_tc)
- [`RTMSG_AR_FAILED`](#rtmsg_ar_failed)
- [`RTMSG_CONTROL`](#rtmsg_control)
- [`RTMSG_DELDEVICE`](#rtmsg_deldevice)
- [`RTMSG_DELROUTE`](#rtmsg_delroute)
- [`RTMSG_DELRULE`](#rtmsg_delrule)
- [`RTMSG_NEWDEVICE`](#rtmsg_newdevice)
- [`RTMSG_NEWROUTE`](#rtmsg_newroute)
- [`RTMSG_NEWRULE`](#rtmsg_newrule)
- [`RTMSG_OVERRUN`](#rtmsg_overrun)
- [`RTM_DELACTION`](#rtm_delaction)
- [`RTM_DELADDR`](#rtm_deladdr)
- [`RTM_DELADDRLABEL`](#rtm_deladdrlabel)
- [`RTM_DELLINK`](#rtm_dellink)
- [`RTM_DELMDB`](#rtm_delmdb)
- [`RTM_DELNEIGH`](#rtm_delneigh)
- [`RTM_DELNSID`](#rtm_delnsid)
- [`RTM_DELQDISC`](#rtm_delqdisc)
- [`RTM_DELROUTE`](#rtm_delroute)
- [`RTM_DELRULE`](#rtm_delrule)
- [`RTM_DELTCLASS`](#rtm_deltclass)
- [`RTM_DELTFILTER`](#rtm_deltfilter)
- [`RTM_F_CLONED`](#rtm_f_cloned)
- [`RTM_F_EQUALIZE`](#rtm_f_equalize)
- [`RTM_F_NOTIFY`](#rtm_f_notify)
- [`RTM_F_PREFIX`](#rtm_f_prefix)
- [`RTM_GETACTION`](#rtm_getaction)
- [`RTM_GETADDR`](#rtm_getaddr)
- [`RTM_GETADDRLABEL`](#rtm_getaddrlabel)
- [`RTM_GETANYCAST`](#rtm_getanycast)
- [`RTM_GETDCB`](#rtm_getdcb)
- [`RTM_GETLINK`](#rtm_getlink)
- [`RTM_GETMDB`](#rtm_getmdb)
- [`RTM_GETMULTICAST`](#rtm_getmulticast)
- [`RTM_GETNEIGH`](#rtm_getneigh)
- [`RTM_GETNEIGHTBL`](#rtm_getneightbl)
- [`RTM_GETNETCONF`](#rtm_getnetconf)
- [`RTM_GETNSID`](#rtm_getnsid)
- [`RTM_GETQDISC`](#rtm_getqdisc)
- [`RTM_GETROUTE`](#rtm_getroute)
- [`RTM_GETRULE`](#rtm_getrule)
- [`RTM_GETTCLASS`](#rtm_gettclass)
- [`RTM_GETTFILTER`](#rtm_gettfilter)
- [`RTM_NEWACTION`](#rtm_newaction)
- [`RTM_NEWADDR`](#rtm_newaddr)
- [`RTM_NEWADDRLABEL`](#rtm_newaddrlabel)
- [`RTM_NEWLINK`](#rtm_newlink)
- [`RTM_NEWMDB`](#rtm_newmdb)
- [`RTM_NEWNDUSEROPT`](#rtm_newnduseropt)
- [`RTM_NEWNEIGH`](#rtm_newneigh)
- [`RTM_NEWNEIGHTBL`](#rtm_newneightbl)
- [`RTM_NEWNETCONF`](#rtm_newnetconf)
- [`RTM_NEWNSID`](#rtm_newnsid)
- [`RTM_NEWPREFIX`](#rtm_newprefix)
- [`RTM_NEWQDISC`](#rtm_newqdisc)
- [`RTM_NEWROUTE`](#rtm_newroute)
- [`RTM_NEWRULE`](#rtm_newrule)
- [`RTM_NEWTCLASS`](#rtm_newtclass)
- [`RTM_NEWTFILTER`](#rtm_newtfilter)
- [`RTM_SETDCB`](#rtm_setdcb)
- [`RTM_SETLINK`](#rtm_setlink)
- [`RTM_SETNEIGHTBL`](#rtm_setneightbl)
- [`RTNLGRP_BRVLAN`](#rtnlgrp_brvlan)
- [`RTNLGRP_DCB`](#rtnlgrp_dcb)
- [`RTNLGRP_DECnet_IFADDR`](#rtnlgrp_decnet_ifaddr)
- [`RTNLGRP_DECnet_ROUTE`](#rtnlgrp_decnet_route)
- [`RTNLGRP_DECnet_RULE`](#rtnlgrp_decnet_rule)
- [`RTNLGRP_IPV4_IFADDR`](#rtnlgrp_ipv4_ifaddr)
- [`RTNLGRP_IPV4_MROUTE`](#rtnlgrp_ipv4_mroute)
- [`RTNLGRP_IPV4_MROUTE_R`](#rtnlgrp_ipv4_mroute_r)
- [`RTNLGRP_IPV4_NETCONF`](#rtnlgrp_ipv4_netconf)
- [`RTNLGRP_IPV4_ROUTE`](#rtnlgrp_ipv4_route)
- [`RTNLGRP_IPV4_RULE`](#rtnlgrp_ipv4_rule)
- [`RTNLGRP_IPV6_IFADDR`](#rtnlgrp_ipv6_ifaddr)
- [`RTNLGRP_IPV6_IFINFO`](#rtnlgrp_ipv6_ifinfo)
- [`RTNLGRP_IPV6_MROUTE`](#rtnlgrp_ipv6_mroute)
- [`RTNLGRP_IPV6_MROUTE_R`](#rtnlgrp_ipv6_mroute_r)
- [`RTNLGRP_IPV6_NETCONF`](#rtnlgrp_ipv6_netconf)
- [`RTNLGRP_IPV6_PREFIX`](#rtnlgrp_ipv6_prefix)
- [`RTNLGRP_IPV6_ROUTE`](#rtnlgrp_ipv6_route)
- [`RTNLGRP_IPV6_RULE`](#rtnlgrp_ipv6_rule)
- [`RTNLGRP_LINK`](#rtnlgrp_link)
- [`RTNLGRP_MCTP_IFADDR`](#rtnlgrp_mctp_ifaddr)
- [`RTNLGRP_MDB`](#rtnlgrp_mdb)
- [`RTNLGRP_MPLS_NETCONF`](#rtnlgrp_mpls_netconf)
- [`RTNLGRP_MPLS_ROUTE`](#rtnlgrp_mpls_route)
- [`RTNLGRP_ND_USEROPT`](#rtnlgrp_nd_useropt)
- [`RTNLGRP_NEIGH`](#rtnlgrp_neigh)
- [`RTNLGRP_NEXTHOP`](#rtnlgrp_nexthop)
- [`RTNLGRP_NONE`](#rtnlgrp_none)
- [`RTNLGRP_NOP2`](#rtnlgrp_nop2)
- [`RTNLGRP_NOP4`](#rtnlgrp_nop4)
- [`RTNLGRP_NOTIFY`](#rtnlgrp_notify)
- [`RTNLGRP_NSID`](#rtnlgrp_nsid)
- [`RTNLGRP_PHONET_IFADDR`](#rtnlgrp_phonet_ifaddr)
- [`RTNLGRP_PHONET_ROUTE`](#rtnlgrp_phonet_route)
- [`RTNLGRP_STATS`](#rtnlgrp_stats)
- [`RTNLGRP_TC`](#rtnlgrp_tc)
- [`RTNLGRP_TUNNEL`](#rtnlgrp_tunnel)
- [`RTN_ANYCAST`](#rtn_anycast)
- [`RTN_BLACKHOLE`](#rtn_blackhole)
- [`RTN_BROADCAST`](#rtn_broadcast)
- [`RTN_LOCAL`](#rtn_local)
- [`RTN_MULTICAST`](#rtn_multicast)
- [`RTN_NAT`](#rtn_nat)
- [`RTN_PROHIBIT`](#rtn_prohibit)
- [`RTN_THROW`](#rtn_throw)
- [`RTN_UNICAST`](#rtn_unicast)
- [`RTN_UNREACHABLE`](#rtn_unreachable)
- [`RTN_UNSPEC`](#rtn_unspec)
- [`RTN_XRESOLVE`](#rtn_xresolve)
- [`RTPROT_BOOT`](#rtprot_boot)
- [`RTPROT_KERNEL`](#rtprot_kernel)
- [`RTPROT_REDIRECT`](#rtprot_redirect)
- [`RTPROT_STATIC`](#rtprot_static)
- [`RTPROT_UNSPEC`](#rtprot_unspec)
- [`RT_SCOPE_HOST`](#rt_scope_host)
- [`RT_SCOPE_LINK`](#rt_scope_link)
- [`RT_SCOPE_NOWHERE`](#rt_scope_nowhere)
- [`RT_SCOPE_SITE`](#rt_scope_site)
- [`RT_SCOPE_UNIVERSE`](#rt_scope_universe)
- [`RT_TABLE_COMPAT`](#rt_table_compat)
- [`RT_TABLE_DEFAULT`](#rt_table_default)
- [`RT_TABLE_LOCAL`](#rt_table_local)
- [`RT_TABLE_MAIN`](#rt_table_main)
- [`RT_TABLE_UNSPEC`](#rt_table_unspec)
- [`RWF_APPEND`](#rwf_append)
- [`RWF_ATOMIC`](#rwf_atomic)
- [`RWF_DONTCACHE`](#rwf_dontcache)
- [`RWF_DSYNC`](#rwf_dsync)
- [`RWF_HIPRI`](#rwf_hipri)
- [`RWF_NOAPPEND`](#rwf_noappend)
- [`RWF_NOWAIT`](#rwf_nowait)
- [`RWF_SYNC`](#rwf_sync)
- [`SCHED_FLAG_ALL`](#sched_flag_all)
- [`SCHED_FLAG_DL_OVERRUN`](#sched_flag_dl_overrun)
- [`SCHED_FLAG_KEEP_ALL`](#sched_flag_keep_all)
- [`SCHED_FLAG_KEEP_PARAMS`](#sched_flag_keep_params)
- [`SCHED_FLAG_KEEP_POLICY`](#sched_flag_keep_policy)
- [`SCHED_FLAG_RECLAIM`](#sched_flag_reclaim)
- [`SCHED_FLAG_RESET_ON_FORK`](#sched_flag_reset_on_fork)
- [`SCHED_FLAG_UTIL_CLAMP`](#sched_flag_util_clamp)
- [`SCHED_FLAG_UTIL_CLAMP_MAX`](#sched_flag_util_clamp_max)
- [`SCHED_FLAG_UTIL_CLAMP_MIN`](#sched_flag_util_clamp_min)
- [`SCTP_ABORT`](#sctp_abort)
- [`SCTP_ADAPTATION_LAYER`](#sctp_adaptation_layer)
- [`SCTP_ADDR_OVER`](#sctp_addr_over)
- [`SCTP_ALL_ASSOC`](#sctp_all_assoc)
- [`SCTP_ASSOCINFO`](#sctp_associnfo)
- [`SCTP_AUTHINFO`](#sctp_authinfo)
- [`SCTP_AUTH_ACTIVE_KEY`](#sctp_auth_active_key)
- [`SCTP_AUTH_CHUNK`](#sctp_auth_chunk)
- [`SCTP_AUTH_DEACTIVATE_KEY`](#sctp_auth_deactivate_key)
- [`SCTP_AUTH_DELETE_KEY`](#sctp_auth_delete_key)
- [`SCTP_AUTH_KEY`](#sctp_auth_key)
- [`SCTP_AUTOCLOSE`](#sctp_autoclose)
- [`SCTP_AUTO_ASCONF`](#sctp_auto_asconf)
- [`SCTP_CONTEXT`](#sctp_context)
- [`SCTP_CURRENT_ASSOC`](#sctp_current_assoc)
- [`SCTP_DEFAULT_SEND_PARAM`](#sctp_default_send_param)
- [`SCTP_DEFAULT_SNDINFO`](#sctp_default_sndinfo)
- [`SCTP_DELAYED_ACK`](#sctp_delayed_ack)
- [`SCTP_DELAYED_ACK_TIME`](#sctp_delayed_ack_time)
- [`SCTP_DELAYED_SACK`](#sctp_delayed_sack)
- [`SCTP_DISABLE_FRAGMENTS`](#sctp_disable_fragments)
- [`SCTP_DSTADDRV4`](#sctp_dstaddrv4)
- [`SCTP_DSTADDRV6`](#sctp_dstaddrv6)
- [`SCTP_ENABLE_CHANGE_ASSOC_REQ`](#sctp_enable_change_assoc_req)
- [`SCTP_ENABLE_RESET_ASSOC_REQ`](#sctp_enable_reset_assoc_req)
- [`SCTP_ENABLE_RESET_STREAM_REQ`](#sctp_enable_reset_stream_req)
- [`SCTP_ENABLE_STRRESET_MASK`](#sctp_enable_strreset_mask)
- [`SCTP_EOF`](#sctp_eof)
- [`SCTP_EVENTS`](#sctp_events)
- [`SCTP_FRAGMENT_INTERLEAVE`](#sctp_fragment_interleave)
- [`SCTP_FUTURE_ASSOC`](#sctp_future_assoc)
- [`SCTP_GET_ASSOC_ID_LIST`](#sctp_get_assoc_id_list)
- [`SCTP_GET_ASSOC_NUMBER`](#sctp_get_assoc_number)
- [`SCTP_GET_PEER_ADDR_INFO`](#sctp_get_peer_addr_info)
- [`SCTP_HMAC_IDENT`](#sctp_hmac_ident)
- [`SCTP_INIT`](#sctp_init)
- [`SCTP_INITMSG`](#sctp_initmsg)
- [`SCTP_I_WANT_MAPPED_V4_ADDR`](#sctp_i_want_mapped_v4_addr)
- [`SCTP_LOCAL_AUTH_CHUNKS`](#sctp_local_auth_chunks)
- [`SCTP_MAXSEG`](#sctp_maxseg)
- [`SCTP_MAX_BURST`](#sctp_max_burst)
- [`SCTP_NODELAY`](#sctp_nodelay)
- [`SCTP_NOTIFICATION`](#sctp_notification)
- [`SCTP_NXTINFO`](#sctp_nxtinfo)
- [`SCTP_PARTIAL_DELIVERY_POINT`](#sctp_partial_delivery_point)
- [`SCTP_PEER_ADDR_PARAMS`](#sctp_peer_addr_params)
- [`SCTP_PEER_ADDR_THLDS`](#sctp_peer_addr_thlds)
- [`SCTP_PEER_ADDR_THLDS_V2`](#sctp_peer_addr_thlds_v2)
- [`SCTP_PEER_AUTH_CHUNKS`](#sctp_peer_auth_chunks)
- [`SCTP_PRIMARY_ADDR`](#sctp_primary_addr)
- [`SCTP_PRINFO`](#sctp_prinfo)
- [`SCTP_PR_SCTP_ALL`](#sctp_pr_sctp_all)
- [`SCTP_PR_SCTP_MASK`](#sctp_pr_sctp_mask)
- [`SCTP_PR_SCTP_MAX`](#sctp_pr_sctp_max)
- [`SCTP_PR_SCTP_NONE`](#sctp_pr_sctp_none)
- [`SCTP_PR_SCTP_PRIO`](#sctp_pr_sctp_prio)
- [`SCTP_PR_SCTP_RTX`](#sctp_pr_sctp_rtx)
- [`SCTP_PR_SCTP_TTL`](#sctp_pr_sctp_ttl)
- [`SCTP_RCVINFO`](#sctp_rcvinfo)
- [`SCTP_RECVNXTINFO`](#sctp_recvnxtinfo)
- [`SCTP_RECVRCVINFO`](#sctp_recvrcvinfo)
- [`SCTP_REUSE_PORT`](#sctp_reuse_port)
- [`SCTP_RTOINFO`](#sctp_rtoinfo)
- [`SCTP_SACK_IMMEDIATELY`](#sctp_sack_immediately)
- [`SCTP_SENDALL`](#sctp_sendall)
- [`SCTP_SET_PEER_PRIMARY_ADDR`](#sctp_set_peer_primary_addr)
- [`SCTP_SNDINFO`](#sctp_sndinfo)
- [`SCTP_SNDRCV`](#sctp_sndrcv)
- [`SCTP_STATUS`](#sctp_status)
- [`SCTP_STREAM_RESET_INCOMING`](#sctp_stream_reset_incoming)
- [`SCTP_STREAM_RESET_OUTGOING`](#sctp_stream_reset_outgoing)
- [`SCTP_UNORDERED`](#sctp_unordered)
- [`SECBIT_EXEC_DENY_INTERACTIVE`](#secbit_exec_deny_interactive)
- [`SECBIT_EXEC_DENY_INTERACTIVE_LOCKED`](#secbit_exec_deny_interactive_locked)
- [`SECBIT_EXEC_RESTRICT_FILE`](#secbit_exec_restrict_file)
- [`SECBIT_EXEC_RESTRICT_FILE_LOCKED`](#secbit_exec_restrict_file_locked)
- [`SECBIT_KEEP_CAPS`](#secbit_keep_caps)
- [`SECBIT_KEEP_CAPS_LOCKED`](#secbit_keep_caps_locked)
- [`SECBIT_NOROOT`](#secbit_noroot)
- [`SECBIT_NOROOT_LOCKED`](#secbit_noroot_locked)
- [`SECBIT_NO_CAP_AMBIENT_RAISE`](#secbit_no_cap_ambient_raise)
- [`SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED`](#secbit_no_cap_ambient_raise_locked)
- [`SECBIT_NO_SETUID_FIXUP`](#secbit_no_setuid_fixup)
- [`SECBIT_NO_SETUID_FIXUP_LOCKED`](#secbit_no_setuid_fixup_locked)
- [`SECCOMP_ADDFD_FLAG_SEND`](#seccomp_addfd_flag_send)
- [`SECCOMP_ADDFD_FLAG_SETFD`](#seccomp_addfd_flag_setfd)
- [`SECCOMP_FILTER_FLAG_LOG`](#seccomp_filter_flag_log)
- [`SECCOMP_FILTER_FLAG_NEW_LISTENER`](#seccomp_filter_flag_new_listener)
- [`SECCOMP_FILTER_FLAG_SPEC_ALLOW`](#seccomp_filter_flag_spec_allow)
- [`SECCOMP_FILTER_FLAG_TSYNC`](#seccomp_filter_flag_tsync)
- [`SECCOMP_FILTER_FLAG_TSYNC_ESRCH`](#seccomp_filter_flag_tsync_esrch)
- [`SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV`](#seccomp_filter_flag_wait_killable_recv)
- [`SECCOMP_GET_ACTION_AVAIL`](#seccomp_get_action_avail)
- [`SECCOMP_GET_NOTIF_SIZES`](#seccomp_get_notif_sizes)
- [`SECCOMP_MODE_DISABLED`](#seccomp_mode_disabled)
- [`SECCOMP_MODE_FILTER`](#seccomp_mode_filter)
- [`SECCOMP_MODE_STRICT`](#seccomp_mode_strict)
- [`SECCOMP_RET_ACTION`](#seccomp_ret_action)
- [`SECCOMP_RET_ACTION_FULL`](#seccomp_ret_action_full)
- [`SECCOMP_RET_ALLOW`](#seccomp_ret_allow)
- [`SECCOMP_RET_DATA`](#seccomp_ret_data)
- [`SECCOMP_RET_ERRNO`](#seccomp_ret_errno)
- [`SECCOMP_RET_KILL`](#seccomp_ret_kill)
- [`SECCOMP_RET_KILL_PROCESS`](#seccomp_ret_kill_process)
- [`SECCOMP_RET_KILL_THREAD`](#seccomp_ret_kill_thread)
- [`SECCOMP_RET_LOG`](#seccomp_ret_log)
- [`SECCOMP_RET_TRACE`](#seccomp_ret_trace)
- [`SECCOMP_RET_TRAP`](#seccomp_ret_trap)
- [`SECCOMP_RET_USER_NOTIF`](#seccomp_ret_user_notif)
- [`SECCOMP_SET_MODE_FILTER`](#seccomp_set_mode_filter)
- [`SECCOMP_SET_MODE_STRICT`](#seccomp_set_mode_strict)
- [`SECCOMP_USER_NOTIF_FLAG_CONTINUE`](#seccomp_user_notif_flag_continue)
- [`SECUREBITS_DEFAULT`](#securebits_default)
- [`SECURE_ALL_BITS`](#secure_all_bits)
- [`SECURE_ALL_LOCKS`](#secure_all_locks)
- [`SECURE_ALL_UNPRIVILEGED`](#secure_all_unprivileged)
- [`SEEK_DATA`](#seek_data)
- [`SEEK_HOLE`](#seek_hole)
- [`SEM_INFO`](#sem_info)
- [`SEM_STAT`](#sem_stat)
- [`SEM_STAT_ANY`](#sem_stat_any)
- [`SEM_UNDO`](#sem_undo)
- [`SETALL`](#setall)
- [`SETVAL`](#setval)
- [`SIOCGHWTSTAMP`](#siocghwtstamp)
- [`SIOCGIWAP`](#siocgiwap)
- [`SIOCGIWAPLIST`](#siocgiwaplist)
- [`SIOCGIWAUTH`](#siocgiwauth)
- [`SIOCGIWENCODE`](#siocgiwencode)
- [`SIOCGIWENCODEEXT`](#siocgiwencodeext)
- [`SIOCGIWESSID`](#siocgiwessid)
- [`SIOCGIWFRAG`](#siocgiwfrag)
- [`SIOCGIWFREQ`](#siocgiwfreq)
- [`SIOCGIWGENIE`](#siocgiwgenie)
- [`SIOCGIWMODE`](#siocgiwmode)
- [`SIOCGIWNAME`](#siocgiwname)
- [`SIOCGIWNICKN`](#siocgiwnickn)
- [`SIOCGIWNWID`](#siocgiwnwid)
- [`SIOCGIWPOWER`](#siocgiwpower)
- [`SIOCGIWPRIV`](#siocgiwpriv)
- [`SIOCGIWRANGE`](#siocgiwrange)
- [`SIOCGIWRATE`](#siocgiwrate)
- [`SIOCGIWRETRY`](#siocgiwretry)
- [`SIOCGIWRTS`](#siocgiwrts)
- [`SIOCGIWSCAN`](#siocgiwscan)
- [`SIOCGIWSENS`](#siocgiwsens)
- [`SIOCGIWSPY`](#siocgiwspy)
- [`SIOCGIWSTATS`](#siocgiwstats)
- [`SIOCGIWTHRSPY`](#siocgiwthrspy)
- [`SIOCGIWTXPOW`](#siocgiwtxpow)
- [`SIOCIWFIRST`](#siociwfirst)
- [`SIOCIWFIRSTPRIV`](#siociwfirstpriv)
- [`SIOCIWLAST`](#siociwlast)
- [`SIOCIWLASTPRIV`](#siociwlastpriv)
- [`SIOCSHWTSTAMP`](#siocshwtstamp)
- [`SIOCSIWAP`](#siocsiwap)
- [`SIOCSIWAUTH`](#siocsiwauth)
- [`SIOCSIWCOMMIT`](#siocsiwcommit)
- [`SIOCSIWENCODE`](#siocsiwencode)
- [`SIOCSIWENCODEEXT`](#siocsiwencodeext)
- [`SIOCSIWESSID`](#siocsiwessid)
- [`SIOCSIWFRAG`](#siocsiwfrag)
- [`SIOCSIWFREQ`](#siocsiwfreq)
- [`SIOCSIWGENIE`](#siocsiwgenie)
- [`SIOCSIWMLME`](#siocsiwmlme)
- [`SIOCSIWMODE`](#siocsiwmode)
- [`SIOCSIWNICKN`](#siocsiwnickn)
- [`SIOCSIWNWID`](#siocsiwnwid)
- [`SIOCSIWPMKSA`](#siocsiwpmksa)
- [`SIOCSIWPOWER`](#siocsiwpower)
- [`SIOCSIWPRIV`](#siocsiwpriv)
- [`SIOCSIWRANGE`](#siocsiwrange)
- [`SIOCSIWRATE`](#siocsiwrate)
- [`SIOCSIWRETRY`](#siocsiwretry)
- [`SIOCSIWRTS`](#siocsiwrts)
- [`SIOCSIWSCAN`](#siocsiwscan)
- [`SIOCSIWSENS`](#siocsiwsens)
- [`SIOCSIWSPY`](#siocsiwspy)
- [`SIOCSIWSTATS`](#siocsiwstats)
- [`SIOCSIWTHRSPY`](#siocsiwthrspy)
- [`SIOCSIWTXPOW`](#siocsiwtxpow)
- [`SI_DETHREAD`](#si_dethread)
- [`SKF_AD_ALU_XOR_X`](#skf_ad_alu_xor_x)
- [`SKF_AD_CPU`](#skf_ad_cpu)
- [`SKF_AD_HATYPE`](#skf_ad_hatype)
- [`SKF_AD_IFINDEX`](#skf_ad_ifindex)
- [`SKF_AD_MARK`](#skf_ad_mark)
- [`SKF_AD_MAX`](#skf_ad_max)
- [`SKF_AD_NLATTR`](#skf_ad_nlattr)
- [`SKF_AD_NLATTR_NEST`](#skf_ad_nlattr_nest)
- [`SKF_AD_OFF`](#skf_ad_off)
- [`SKF_AD_PAY_OFFSET`](#skf_ad_pay_offset)
- [`SKF_AD_PKTTYPE`](#skf_ad_pkttype)
- [`SKF_AD_PROTOCOL`](#skf_ad_protocol)
- [`SKF_AD_QUEUE`](#skf_ad_queue)
- [`SKF_AD_RANDOM`](#skf_ad_random)
- [`SKF_AD_RXHASH`](#skf_ad_rxhash)
- [`SKF_AD_VLAN_TAG`](#skf_ad_vlan_tag)
- [`SKF_AD_VLAN_TAG_PRESENT`](#skf_ad_vlan_tag_present)
- [`SKF_AD_VLAN_TPID`](#skf_ad_vlan_tpid)
- [`SKF_LL_OFF`](#skf_ll_off)
- [`SKF_NET_OFF`](#skf_net_off)
- [`SK_MEMINFO_BACKLOG`](#sk_meminfo_backlog)
- [`SK_MEMINFO_DROPS`](#sk_meminfo_drops)
- [`SK_MEMINFO_FWD_ALLOC`](#sk_meminfo_fwd_alloc)
- [`SK_MEMINFO_OPTMEM`](#sk_meminfo_optmem)
- [`SK_MEMINFO_RCVBUF`](#sk_meminfo_rcvbuf)
- [`SK_MEMINFO_RMEM_ALLOC`](#sk_meminfo_rmem_alloc)
- [`SK_MEMINFO_SNDBUF`](#sk_meminfo_sndbuf)
- [`SK_MEMINFO_WMEM_ALLOC`](#sk_meminfo_wmem_alloc)
- [`SK_MEMINFO_WMEM_QUEUED`](#sk_meminfo_wmem_queued)
- [`SND_CNT`](#snd_cnt)
- [`SND_MAX`](#snd_max)
- [`SOF_TIMESTAMPING_BIND_PHC`](#sof_timestamping_bind_phc)
- [`SOF_TIMESTAMPING_OPT_CMSG`](#sof_timestamping_opt_cmsg)
- [`SOF_TIMESTAMPING_OPT_ID`](#sof_timestamping_opt_id)
- [`SOF_TIMESTAMPING_OPT_ID_TCP`](#sof_timestamping_opt_id_tcp)
- [`SOF_TIMESTAMPING_OPT_PKTINFO`](#sof_timestamping_opt_pktinfo)
- [`SOF_TIMESTAMPING_OPT_RX_FILTER`](#sof_timestamping_opt_rx_filter)
- [`SOF_TIMESTAMPING_OPT_STATS`](#sof_timestamping_opt_stats)
- [`SOF_TIMESTAMPING_OPT_TSONLY`](#sof_timestamping_opt_tsonly)
- [`SOF_TIMESTAMPING_OPT_TX_SWHW`](#sof_timestamping_opt_tx_swhw)
- [`SOF_TIMESTAMPING_RAW_HARDWARE`](#sof_timestamping_raw_hardware)
- [`SOF_TIMESTAMPING_RX_HARDWARE`](#sof_timestamping_rx_hardware)
- [`SOF_TIMESTAMPING_RX_SOFTWARE`](#sof_timestamping_rx_software)
- [`SOF_TIMESTAMPING_SOFTWARE`](#sof_timestamping_software)
- [`SOF_TIMESTAMPING_SYS_HARDWARE`](#sof_timestamping_sys_hardware)
- [`SOF_TIMESTAMPING_TX_ACK`](#sof_timestamping_tx_ack)
- [`SOF_TIMESTAMPING_TX_HARDWARE`](#sof_timestamping_tx_hardware)
- [`SOF_TIMESTAMPING_TX_SCHED`](#sof_timestamping_tx_sched)
- [`SOF_TIMESTAMPING_TX_SOFTWARE`](#sof_timestamping_tx_software)
- [`SOF_TXTIME_DEADLINE_MODE`](#sof_txtime_deadline_mode)
- [`SOF_TXTIME_REPORT_ERRORS`](#sof_txtime_report_errors)
- [`SOL_TLS`](#sol_tls)
- [`SOL_XDP`](#sol_xdp)
- [`SO_EE_ORIGIN_ICMP`](#so_ee_origin_icmp)
- [`SO_EE_ORIGIN_ICMP6`](#so_ee_origin_icmp6)
- [`SO_EE_ORIGIN_LOCAL`](#so_ee_origin_local)
- [`SO_EE_ORIGIN_NONE`](#so_ee_origin_none)
- [`SO_EE_ORIGIN_TIMESTAMPING`](#so_ee_origin_timestamping)
- [`SO_EE_ORIGIN_TXSTATUS`](#so_ee_origin_txstatus)
- [`SO_ORIGINAL_DST`](#so_original_dst)
- [`SW_CNT`](#sw_cnt)
- [`SW_MAX`](#sw_max)
- [`SYNC_FILE_RANGE_WAIT_AFTER`](#sync_file_range_wait_after)
- [`SYNC_FILE_RANGE_WAIT_BEFORE`](#sync_file_range_wait_before)
- [`SYNC_FILE_RANGE_WRITE`](#sync_file_range_write)
- [`SYN_CNT`](#syn_cnt)
- [`SYN_MAX`](#syn_max)
- [`TCA_FCNT`](#tca_fcnt)
- [`TCA_KIND`](#tca_kind)
- [`TCA_OPTIONS`](#tca_options)
- [`TCA_RATE`](#tca_rate)
- [`TCA_STAB`](#tca_stab)
- [`TCA_STATS`](#tca_stats)
- [`TCA_STATS2`](#tca_stats2)
- [`TCA_UNSPEC`](#tca_unspec)
- [`TCA_XSTATS`](#tca_xstats)
- [`TFD_CLOEXEC`](#tfd_cloexec)
- [`TFD_NONBLOCK`](#tfd_nonblock)
- [`TFD_TIMER_ABSTIME`](#tfd_timer_abstime)
- [`TFD_TIMER_CANCEL_ON_SET`](#tfd_timer_cancel_on_set)
- [`TLS_1_2_VERSION`](#tls_1_2_version)
- [`TLS_1_2_VERSION_MAJOR`](#tls_1_2_version_major)
- [`TLS_1_2_VERSION_MINOR`](#tls_1_2_version_minor)
- [`TLS_1_3_VERSION`](#tls_1_3_version)
- [`TLS_1_3_VERSION_MAJOR`](#tls_1_3_version_major)
- [`TLS_1_3_VERSION_MINOR`](#tls_1_3_version_minor)
- [`TLS_CIPHER_AES_CCM_128`](#tls_cipher_aes_ccm_128)
- [`TLS_CIPHER_AES_CCM_128_IV_SIZE`](#tls_cipher_aes_ccm_128_iv_size)
- [`TLS_CIPHER_AES_CCM_128_KEY_SIZE`](#tls_cipher_aes_ccm_128_key_size)
- [`TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE`](#tls_cipher_aes_ccm_128_rec_seq_size)
- [`TLS_CIPHER_AES_CCM_128_SALT_SIZE`](#tls_cipher_aes_ccm_128_salt_size)
- [`TLS_CIPHER_AES_CCM_128_TAG_SIZE`](#tls_cipher_aes_ccm_128_tag_size)
- [`TLS_CIPHER_AES_GCM_128`](#tls_cipher_aes_gcm_128)
- [`TLS_CIPHER_AES_GCM_128_IV_SIZE`](#tls_cipher_aes_gcm_128_iv_size)
- [`TLS_CIPHER_AES_GCM_128_KEY_SIZE`](#tls_cipher_aes_gcm_128_key_size)
- [`TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE`](#tls_cipher_aes_gcm_128_rec_seq_size)
- [`TLS_CIPHER_AES_GCM_128_SALT_SIZE`](#tls_cipher_aes_gcm_128_salt_size)
- [`TLS_CIPHER_AES_GCM_128_TAG_SIZE`](#tls_cipher_aes_gcm_128_tag_size)
- [`TLS_CIPHER_AES_GCM_256`](#tls_cipher_aes_gcm_256)
- [`TLS_CIPHER_AES_GCM_256_IV_SIZE`](#tls_cipher_aes_gcm_256_iv_size)
- [`TLS_CIPHER_AES_GCM_256_KEY_SIZE`](#tls_cipher_aes_gcm_256_key_size)
- [`TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE`](#tls_cipher_aes_gcm_256_rec_seq_size)
- [`TLS_CIPHER_AES_GCM_256_SALT_SIZE`](#tls_cipher_aes_gcm_256_salt_size)
- [`TLS_CIPHER_AES_GCM_256_TAG_SIZE`](#tls_cipher_aes_gcm_256_tag_size)
- [`TLS_CIPHER_ARIA_GCM_128`](#tls_cipher_aria_gcm_128)
- [`TLS_CIPHER_ARIA_GCM_128_IV_SIZE`](#tls_cipher_aria_gcm_128_iv_size)
- [`TLS_CIPHER_ARIA_GCM_128_KEY_SIZE`](#tls_cipher_aria_gcm_128_key_size)
- [`TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE`](#tls_cipher_aria_gcm_128_rec_seq_size)
- [`TLS_CIPHER_ARIA_GCM_128_SALT_SIZE`](#tls_cipher_aria_gcm_128_salt_size)
- [`TLS_CIPHER_ARIA_GCM_128_TAG_SIZE`](#tls_cipher_aria_gcm_128_tag_size)
- [`TLS_CIPHER_ARIA_GCM_256`](#tls_cipher_aria_gcm_256)
- [`TLS_CIPHER_ARIA_GCM_256_IV_SIZE`](#tls_cipher_aria_gcm_256_iv_size)
- [`TLS_CIPHER_ARIA_GCM_256_KEY_SIZE`](#tls_cipher_aria_gcm_256_key_size)
- [`TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE`](#tls_cipher_aria_gcm_256_rec_seq_size)
- [`TLS_CIPHER_ARIA_GCM_256_SALT_SIZE`](#tls_cipher_aria_gcm_256_salt_size)
- [`TLS_CIPHER_ARIA_GCM_256_TAG_SIZE`](#tls_cipher_aria_gcm_256_tag_size)
- [`TLS_CIPHER_CHACHA20_POLY1305`](#tls_cipher_chacha20_poly1305)
- [`TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE`](#tls_cipher_chacha20_poly1305_iv_size)
- [`TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE`](#tls_cipher_chacha20_poly1305_key_size)
- [`TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE`](#tls_cipher_chacha20_poly1305_rec_seq_size)
- [`TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE`](#tls_cipher_chacha20_poly1305_salt_size)
- [`TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE`](#tls_cipher_chacha20_poly1305_tag_size)
- [`TLS_CIPHER_SM4_CCM`](#tls_cipher_sm4_ccm)
- [`TLS_CIPHER_SM4_CCM_IV_SIZE`](#tls_cipher_sm4_ccm_iv_size)
- [`TLS_CIPHER_SM4_CCM_KEY_SIZE`](#tls_cipher_sm4_ccm_key_size)
- [`TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE`](#tls_cipher_sm4_ccm_rec_seq_size)
- [`TLS_CIPHER_SM4_CCM_SALT_SIZE`](#tls_cipher_sm4_ccm_salt_size)
- [`TLS_CIPHER_SM4_CCM_TAG_SIZE`](#tls_cipher_sm4_ccm_tag_size)
- [`TLS_CIPHER_SM4_GCM`](#tls_cipher_sm4_gcm)
- [`TLS_CIPHER_SM4_GCM_IV_SIZE`](#tls_cipher_sm4_gcm_iv_size)
- [`TLS_CIPHER_SM4_GCM_KEY_SIZE`](#tls_cipher_sm4_gcm_key_size)
- [`TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE`](#tls_cipher_sm4_gcm_rec_seq_size)
- [`TLS_CIPHER_SM4_GCM_SALT_SIZE`](#tls_cipher_sm4_gcm_salt_size)
- [`TLS_CIPHER_SM4_GCM_TAG_SIZE`](#tls_cipher_sm4_gcm_tag_size)
- [`TLS_CONF_BASE`](#tls_conf_base)
- [`TLS_CONF_HW`](#tls_conf_hw)
- [`TLS_CONF_HW_RECORD`](#tls_conf_hw_record)
- [`TLS_CONF_SW`](#tls_conf_sw)
- [`TLS_GET_RECORD_TYPE`](#tls_get_record_type)
- [`TLS_INFO_CIPHER`](#tls_info_cipher)
- [`TLS_INFO_RXCONF`](#tls_info_rxconf)
- [`TLS_INFO_RX_NO_PAD`](#tls_info_rx_no_pad)
- [`TLS_INFO_TXCONF`](#tls_info_txconf)
- [`TLS_INFO_UNSPEC`](#tls_info_unspec)
- [`TLS_INFO_VERSION`](#tls_info_version)
- [`TLS_INFO_ZC_RO_TX`](#tls_info_zc_ro_tx)
- [`TLS_RX`](#tls_rx)
- [`TLS_RX_EXPECT_NO_PAD`](#tls_rx_expect_no_pad)
- [`TLS_SET_RECORD_TYPE`](#tls_set_record_type)
- [`TLS_TX`](#tls_tx)
- [`TLS_TX_ZEROCOPY_RO`](#tls_tx_zerocopy_ro)
- [`TPACKET2_HDRLEN`](#tpacket2_hdrlen)
- [`TPACKET3_HDRLEN`](#tpacket3_hdrlen)
- [`TPACKET_ALIGNMENT`](#tpacket_alignment)
- [`TPACKET_HDRLEN`](#tpacket_hdrlen)
- [`TP_FT_REQ_FILL_RXHASH`](#tp_ft_req_fill_rxhash)
- [`TP_STATUS_AVAILABLE`](#tp_status_available)
- [`TP_STATUS_BLK_TMO`](#tp_status_blk_tmo)
- [`TP_STATUS_COPY`](#tp_status_copy)
- [`TP_STATUS_CSUMNOTREADY`](#tp_status_csumnotready)
- [`TP_STATUS_CSUM_VALID`](#tp_status_csum_valid)
- [`TP_STATUS_KERNEL`](#tp_status_kernel)
- [`TP_STATUS_LOSING`](#tp_status_losing)
- [`TP_STATUS_SENDING`](#tp_status_sending)
- [`TP_STATUS_SEND_REQUEST`](#tp_status_send_request)
- [`TP_STATUS_TS_RAW_HARDWARE`](#tp_status_ts_raw_hardware)
- [`TP_STATUS_TS_SOFTWARE`](#tp_status_ts_software)
- [`TP_STATUS_TS_SYS_HARDWARE`](#tp_status_ts_sys_hardware)
- [`TP_STATUS_USER`](#tp_status_user)
- [`TP_STATUS_VLAN_TPID_VALID`](#tp_status_vlan_tpid_valid)
- [`TP_STATUS_VLAN_VALID`](#tp_status_vlan_valid)
- [`TP_STATUS_WRONG_FORMAT`](#tp_status_wrong_format)
- [`TRAP_PERF`](#trap_perf)
- [`UINPUT_MAX_NAME_SIZE`](#uinput_max_name_size)
- [`UINPUT_VERSION`](#uinput_version)
- [`VMADDR_CID_ANY`](#vmaddr_cid_any)
- [`VMADDR_CID_HOST`](#vmaddr_cid_host)
- [`VMADDR_CID_HYPERVISOR`](#vmaddr_cid_hypervisor)
- [`VMADDR_CID_LOCAL`](#vmaddr_cid_local)
- [`VMADDR_CID_RESERVED`](#vmaddr_cid_reserved)
- [`VMADDR_PORT_ANY`](#vmaddr_port_any)
- [`VM_BLOCK_DUMP`](#vm_block_dump)
- [`VM_DIRTY_BACKGROUND`](#vm_dirty_background)
- [`VM_DIRTY_EXPIRE_CS`](#vm_dirty_expire_cs)
- [`VM_DIRTY_RATIO`](#vm_dirty_ratio)
- [`VM_DIRTY_WB_CS`](#vm_dirty_wb_cs)
- [`VM_DROP_PAGECACHE`](#vm_drop_pagecache)
- [`VM_HUGETLB_GROUP`](#vm_hugetlb_group)
- [`VM_HUGETLB_PAGES`](#vm_hugetlb_pages)
- [`VM_LAPTOP_MODE`](#vm_laptop_mode)
- [`VM_LEGACY_VA_LAYOUT`](#vm_legacy_va_layout)
- [`VM_LOWMEM_RESERVE_RATIO`](#vm_lowmem_reserve_ratio)
- [`VM_MAX_MAP_COUNT`](#vm_max_map_count)
- [`VM_MIN_FREE_KBYTES`](#vm_min_free_kbytes)
- [`VM_MIN_SLAB`](#vm_min_slab)
- [`VM_MIN_UNMAPPED`](#vm_min_unmapped)
- [`VM_NR_PDFLUSH_THREADS`](#vm_nr_pdflush_threads)
- [`VM_OVERCOMMIT_MEMORY`](#vm_overcommit_memory)
- [`VM_OVERCOMMIT_RATIO`](#vm_overcommit_ratio)
- [`VM_PAGEBUF`](#vm_pagebuf)
- [`VM_PAGE_CLUSTER`](#vm_page_cluster)
- [`VM_PANIC_ON_OOM`](#vm_panic_on_oom)
- [`VM_PERCPU_PAGELIST_FRACTION`](#vm_percpu_pagelist_fraction)
- [`VM_SWAPPINESS`](#vm_swappiness)
- [`VM_SWAP_TOKEN_TIMEOUT`](#vm_swap_token_timeout)
- [`VM_VDSO_ENABLED`](#vm_vdso_enabled)
- [`VM_VFS_CACHE_PRESSURE`](#vm_vfs_cache_pressure)
- [`VM_ZONE_RECLAIM_MODE`](#vm_zone_reclaim_mode)
- [`WIRELESS_EXT`](#wireless_ext)
- [`XDP_COPY`](#xdp_copy)
- [`XDP_MMAP_OFFSETS`](#xdp_mmap_offsets)
- [`XDP_OPTIONS`](#xdp_options)
- [`XDP_OPTIONS_ZEROCOPY`](#xdp_options_zerocopy)
- [`XDP_PGOFF_RX_RING`](#xdp_pgoff_rx_ring)
- [`XDP_PGOFF_TX_RING`](#xdp_pgoff_tx_ring)
- [`XDP_PKT_CONTD`](#xdp_pkt_contd)
- [`XDP_RING_NEED_WAKEUP`](#xdp_ring_need_wakeup)
- [`XDP_RX_RING`](#xdp_rx_ring)
- [`XDP_SHARED_UMEM`](#xdp_shared_umem)
- [`XDP_STATISTICS`](#xdp_statistics)
- [`XDP_TXMD_FLAGS_CHECKSUM`](#xdp_txmd_flags_checksum)
- [`XDP_TXMD_FLAGS_TIMESTAMP`](#xdp_txmd_flags_timestamp)
- [`XDP_TX_METADATA`](#xdp_tx_metadata)
- [`XDP_TX_RING`](#xdp_tx_ring)
- [`XDP_UMEM_COMPLETION_RING`](#xdp_umem_completion_ring)
- [`XDP_UMEM_FILL_RING`](#xdp_umem_fill_ring)
- [`XDP_UMEM_PGOFF_COMPLETION_RING`](#xdp_umem_pgoff_completion_ring)
- [`XDP_UMEM_PGOFF_FILL_RING`](#xdp_umem_pgoff_fill_ring)
- [`XDP_UMEM_REG`](#xdp_umem_reg)
- [`XDP_UMEM_TX_METADATA_LEN`](#xdp_umem_tx_metadata_len)
- [`XDP_UMEM_TX_SW_CSUM`](#xdp_umem_tx_sw_csum)
- [`XDP_UMEM_UNALIGNED_CHUNK_FLAG`](#xdp_umem_unaligned_chunk_flag)
- [`XDP_USE_NEED_WAKEUP`](#xdp_use_need_wakeup)
- [`XDP_USE_SG`](#xdp_use_sg)
- [`XDP_ZEROCOPY`](#xdp_zerocopy)
- [`XSK_UNALIGNED_BUF_ADDR_MASK`](#xsk_unaligned_buf_addr_mask)
- [`XSK_UNALIGNED_BUF_OFFSET_SHIFT`](#xsk_unaligned_buf_offset_shift)
- [`__NFT_REG_MAX`](#__nft_reg_max)

**Type Aliases**

- [`__kernel_clockid_t`](#__kernel_clockid_t)
- [`__kernel_fsid_t`](#__kernel_fsid_t)
- [`__s16`](#__s16)
- [`__s32`](#__s32)
- [`__u16`](#__u16)
- [`__u32`](#__u32)
- [`__u8`](#__u8)
- [`blkcnt64_t`](#blkcnt64_t)
- [`dev_t`](#dev_t)
- [`eventfd_t`](#eventfd_t)
- [`idtype_t`](#idtype_t)
- [`ino64_t`](#ino64_t)
- [`loff_t`](#loff_t)
- [`mode_t`](#mode_t)
- [`mqd_t`](#mqd_t)
- [`nfds_t`](#nfds_t)
- [`nl_item`](#nl_item)
- [`off64_t`](#off64_t)
- [`pid_type`](#pid_type)
- [`proc_cn_event`](#proc_cn_event)
- [`proc_cn_mcast_op`](#proc_cn_mcast_op)
- [`pthread_key_t`](#pthread_key_t)
- [`pthread_once_t`](#pthread_once_t)
- [`pthread_spinlock_t`](#pthread_spinlock_t)
- [`rlim64_t`](#rlim64_t)
- [`sctp_assoc_t`](#sctp_assoc_t)
- [`socklen_t`](#socklen_t)

---

## libc::unix::linux_like::linux::ABS_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::ABS_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::ALG_OP_DECRYPT

*Constant*: `c_int`



## libc::unix::linux_like::linux::ALG_OP_ENCRYPT

*Constant*: `c_int`



## libc::unix::linux_like::linux::ALG_SET_AEAD_ASSOCLEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::ALG_SET_AEAD_AUTHSIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::ALG_SET_DRBG_ENTROPY

*Constant*: `c_int`



## libc::unix::linux_like::linux::ALG_SET_IV

*Constant*: `c_int`



## libc::unix::linux_like::linux::ALG_SET_KEY

*Constant*: `c_int`



## libc::unix::linux_like::linux::ALG_SET_KEY_BY_KEY_SERIAL

*Constant*: `c_int`



## libc::unix::linux_like::linux::ALG_SET_OP

*Constant*: `c_int`



## libc::unix::linux_like::linux::AT_EXECVE_CHECK

*Constant*: `c_int`



## libc::unix::linux_like::linux::AT_HANDLE_CONNECTABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux::AT_HANDLE_FID

*Constant*: `c_int`



## libc::unix::linux_like::linux::AT_HANDLE_MNT_ID_UNIQUE

*Constant*: `c_int`



## libc::unix::linux_like::linux::BPF_A

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_ABS

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_ADD

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_ALU

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_AND

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_B

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_CLASS

*Function*

```rust
fn BPF_CLASS(code: __u32) -> __u32
```



## libc::unix::linux_like::linux::BPF_DIV

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_H

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_IMM

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_IND

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_JA

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_JEQ

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_JGE

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_JGT

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_JMP

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_JSET

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_JUMP

*Function*

```rust
fn BPF_JUMP(code: __u16, k: __u32, jt: __u8, jf: __u8) -> crate::sock_filter
```



## libc::unix::linux_like::linux::BPF_K

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_LD

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_LDX

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_LEN

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_LL_OFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::BPF_LSH

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_MAXINSNS

*Constant*: `c_int`



## libc::unix::linux_like::linux::BPF_MEM

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_MEMWORDS

*Constant*: `c_int`



## libc::unix::linux_like::linux::BPF_MISC

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_MISCOP

*Function*

```rust
fn BPF_MISCOP(code: __u32) -> __u32
```



## libc::unix::linux_like::linux::BPF_MOD

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_MODE

*Function*

```rust
fn BPF_MODE(code: __u32) -> __u32
```



## libc::unix::linux_like::linux::BPF_MSH

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_MUL

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_NEG

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_NET_OFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::BPF_OP

*Function*

```rust
fn BPF_OP(code: __u32) -> __u32
```



## libc::unix::linux_like::linux::BPF_OR

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_RET

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_RSH

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_RVAL

*Function*

```rust
fn BPF_RVAL(code: __u32) -> __u32
```



## libc::unix::linux_like::linux::BPF_SIZE

*Function*

```rust
fn BPF_SIZE(code: __u32) -> __u32
```



## libc::unix::linux_like::linux::BPF_SRC

*Function*

```rust
fn BPF_SRC(code: __u32) -> __u32
```



## libc::unix::linux_like::linux::BPF_ST

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_STMT

*Function*

```rust
fn BPF_STMT(code: __u16, k: __u32) -> crate::sock_filter
```



## libc::unix::linux_like::linux::BPF_STX

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_SUB

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_TAX

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_TXA

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_W

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_X

*Constant*: `__u32`



## libc::unix::linux_like::linux::BPF_XOR

*Constant*: `__u32`



## libc::unix::linux_like::linux::CLONE_PIDFD

*Constant*: `c_int`



## libc::unix::linux_like::linux::CLOSE_RANGE_CLOEXEC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CLOSE_RANGE_UNSHARE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_DST_IDX

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_DST_VAL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_IDX_BB

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_IDX_CIFS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_IDX_DM

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_IDX_DRBD

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_IDX_PROC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_IDX_V86D

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_KVP_IDX

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_KVP_VAL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_VAL_CIFS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_VAL_DM_USERSPACE_LOG

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_VAL_DRBD

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_VAL_PROC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_VAL_V86D_UVESAFB

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_VSS_IDX

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_VSS_VAL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_W1_IDX

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CN_W1_VAL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::CTL_ABI

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTL_BUS

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTL_BUS_ISA

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTL_CPU

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTL_DEBUG

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTL_DEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTL_FS

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTL_KERN

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTL_NET

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTL_VM

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_FAMILY_ID

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_FAMILY_NAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_HDRSIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_MAXATTR

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_MCAST_GROUPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_MCAST_GRP_ID

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_MCAST_GRP_NAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_MCAST_GRP_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_OPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_OP_FLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_OP_ID

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_OP_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_ATTR_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_CMD_DELFAMILY

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_CMD_DELMCAST_GRP

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_CMD_DELOPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_CMD_GETFAMILY

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_CMD_GETMCAST_GRP

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_CMD_GETOPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_CMD_NEWFAMILY

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_CMD_NEWMCAST_GRP

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_CMD_NEWOPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::CTRL_CMD_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SERVICE_LIST_MAX_LEN

*Constant*: `c_int`

maximum number of services provided on the same listening port



## libc::unix::linux_like::linux::DCCP_SOCKOPT_AVAILABLE_CCIDS

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_CCID

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_CCID_RX_INFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_CCID_TX_INFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_CHANGE_L

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_CHANGE_R

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_GET_CUR_MPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_PACKET_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_QPOLICY_ID

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_QPOLICY_TXQLEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_RECV_CSCOV

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_RX_CCID

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_SEND_CSCOV

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_SERVER_TIMEWAIT

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_SERVICE

*Constant*: `c_int`



## libc::unix::linux_like::linux::DCCP_SOCKOPT_TX_CCID

*Constant*: `c_int`



## libc::unix::linux_like::linux::EFD_SEMAPHORE

*Constant*: `c_int`



## libc::unix::linux_like::linux::ENOATTR

*Constant*: `c_int`



## libc::unix::linux_like::linux::EPIOCGPARAMS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::EPIOCSPARAMS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::ETH_ALEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_DATA_LEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_FCS_LEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_FRAME_LEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_HLEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_1588

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_8021AD

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_8021AH

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_8021Q

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_80221

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_802_2

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_802_3

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_802_3_MIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_802_EX1

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_AARP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_AF_IUCV

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_ALL

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_AOE

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_ARCNET

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_ARP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_ATALK

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_ATMFATE

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_ATMMPOA

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_AX25

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_BATMAN

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_BPQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_CAIF

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_CANFD

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_CONTROL

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_CUST

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_DDCMP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_DEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_DIAG

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_DNA_DL

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_DNA_RC

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_DNA_RT

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_DSA

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_ECONET

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_EDSA

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_FCOE

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_FIP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_HDLC

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_IEEE802154

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_IEEEPUP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_IEEEPUPAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_IP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_IPV6

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_IPX

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_IRDA

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_LAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_LINK_CTL

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_LOCALTALK

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_LOOP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_LOOPBACK

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_MACSEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_MOBITEX

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_MPLS_MC

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_MPLS_UC

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_MVRP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_PAE

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_PAUSE

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_PHONET

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_PPPTALK

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_PPP_DISC

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_PPP_MP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_PPP_SES

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_PRP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_PUP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_PUPAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_QINQ1

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_QINQ2

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_QINQ3

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_RARP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_SCA

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_SLOW

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_SNAP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_TDLS

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_TEB

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_TIPC

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_TRAILER

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_TR_802_2

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_WAN_PPP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_WCCP

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_P_X25

*Constant*: `c_int`



## libc::unix::linux_like::linux::ETH_ZLEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::EV_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::EV_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::FALLOC_FL_COLLAPSE_RANGE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FALLOC_FL_INSERT_RANGE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FALLOC_FL_KEEP_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FALLOC_FL_PUNCH_HOLE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FALLOC_FL_UNSHARE_RANGE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FALLOC_FL_ZERO_RANGE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FANOTIFY_METADATA_VERSION

*Constant*: `u8`



## libc::unix::linux_like::linux::FAN_ACCESS

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_ACCESS_PERM

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_ALLOW

*Constant*: `u32`



## libc::unix::linux_like::linux::FAN_ATTRIB

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_AUDIT

*Constant*: `u32`



## libc::unix::linux_like::linux::FAN_CLASS_CONTENT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_CLASS_NOTIF

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_CLASS_PRE_CONTENT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_CLOEXEC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_CLOSE

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_CLOSE_NOWRITE

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_CLOSE_WRITE

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_CREATE

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_DELETE

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_DELETE_SELF

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_DENY

*Constant*: `u32`



## libc::unix::linux_like::linux::FAN_ENABLE_AUDIT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_EPIDFD

*Constant*: `c_int`



## libc::unix::linux_like::linux::FAN_EVENT_INFO_TYPE_DFID

*Constant*: `u8`



## libc::unix::linux_like::linux::FAN_EVENT_INFO_TYPE_DFID_NAME

*Constant*: `u8`



## libc::unix::linux_like::linux::FAN_EVENT_INFO_TYPE_ERROR

*Constant*: `u8`



## libc::unix::linux_like::linux::FAN_EVENT_INFO_TYPE_FID

*Constant*: `u8`



## libc::unix::linux_like::linux::FAN_EVENT_INFO_TYPE_NEW_DFID_NAME

*Constant*: `u8`



## libc::unix::linux_like::linux::FAN_EVENT_INFO_TYPE_OLD_DFID_NAME

*Constant*: `u8`



## libc::unix::linux_like::linux::FAN_EVENT_INFO_TYPE_PIDFD

*Constant*: `u8`



## libc::unix::linux_like::linux::FAN_EVENT_ON_CHILD

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_FS_ERROR

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_INFO

*Constant*: `u32`



## libc::unix::linux_like::linux::FAN_MARK_ADD

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_DONT_FOLLOW

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_EVICTABLE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_FILESYSTEM

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_FLUSH

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_IGNORE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_IGNORED_MASK

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_IGNORED_SURV_MODIFY

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_IGNORE_SURV

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_INODE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_MOUNT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_ONLYDIR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MARK_REMOVE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_MODIFY

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_MOVE

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_MOVED_FROM

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_MOVED_TO

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_MOVE_SELF

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_NOFD

*Constant*: `c_int`



## libc::unix::linux_like::linux::FAN_NONBLOCK

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_NOPIDFD

*Constant*: `c_int`



## libc::unix::linux_like::linux::FAN_ONDIR

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_OPEN

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_OPEN_EXEC

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_OPEN_EXEC_PERM

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_OPEN_PERM

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_Q_OVERFLOW

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_RENAME

*Constant*: `u64`



## libc::unix::linux_like::linux::FAN_REPORT_DFID_NAME

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_REPORT_DFID_NAME_TARGET

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_REPORT_DIR_FID

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_REPORT_FID

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_REPORT_NAME

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_REPORT_PIDFD

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_REPORT_TARGET_FID

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_REPORT_TID

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_RESPONSE_INFO_AUDIT_RULE

*Constant*: `u8`



## libc::unix::linux_like::linux::FAN_RESPONSE_INFO_NONE

*Constant*: `u8`



## libc::unix::linux_like::linux::FAN_UNLIMITED_MARKS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FAN_UNLIMITED_QUEUE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::FF_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::FF_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::FUTEX_BITSET_MATCH_ANY

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_CLOCK_REALTIME

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_CMD_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_CMP_REQUEUE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_CMP_REQUEUE_PI

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_FD

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_LOCK_PI

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_LOCK_PI2

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP

*Function*

```rust
fn FUTEX_OP(op: c_int, oparg: c_int, cmp: c_int, cmparg: c_int) -> c_int
```



## libc::unix::linux_like::linux::FUTEX_OP_ADD

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_ANDN

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_CMP_EQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_CMP_GE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_CMP_GT

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_CMP_LE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_CMP_LT

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_CMP_NE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_OPARG_SHIFT

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_OR

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_SET

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OP_XOR

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_OWNER_DIED

*Constant*: `u32`



## libc::unix::linux_like::linux::FUTEX_PRIVATE_FLAG

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_REQUEUE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_TID_MASK

*Constant*: `u32`



## libc::unix::linux_like::linux::FUTEX_TRYLOCK_PI

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_UNLOCK_PI

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_WAIT

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_WAITERS

*Constant*: `u32`



## libc::unix::linux_like::linux::FUTEX_WAIT_BITSET

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_WAIT_REQUEUE_PI

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_WAKE

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_WAKE_BITSET

*Constant*: `c_int`



## libc::unix::linux_like::linux::FUTEX_WAKE_OP

*Constant*: `c_int`



## libc::unix::linux_like::linux::F_SEAL_EXEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::F_SEAL_FUTURE_WRITE

*Constant*: `c_int`



## libc::unix::linux_like::linux::GENL_ADMIN_PERM

*Constant*: `c_int`



## libc::unix::linux_like::linux::GENL_CMD_CAP_DO

*Constant*: `c_int`



## libc::unix::linux_like::linux::GENL_CMD_CAP_DUMP

*Constant*: `c_int`



## libc::unix::linux_like::linux::GENL_CMD_CAP_HASPOL

*Constant*: `c_int`



## libc::unix::linux_like::linux::GENL_ID_CTRL

*Constant*: `c_int`



## libc::unix::linux_like::linux::GENL_MAX_ID

*Constant*: `c_int`



## libc::unix::linux_like::linux::GENL_MIN_ID

*Constant*: `c_int`



## libc::unix::linux_like::linux::GENL_NAMSIZ

*Constant*: `c_int`



## libc::unix::linux_like::linux::GETALL

*Constant*: `c_int`



## libc::unix::linux_like::linux::GETNCNT

*Constant*: `c_int`



## libc::unix::linux_like::linux::GETPID

*Constant*: `c_int`



## libc::unix::linux_like::linux::GETVAL

*Constant*: `c_int`



## libc::unix::linux_like::linux::GETZCNT

*Constant*: `c_int`



## libc::unix::linux_like::linux::GRND_INSECURE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::GRND_NONBLOCK

*Constant*: `c_uint`



## libc::unix::linux_like::linux::GRND_RANDOM

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_ALL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_NONE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_NTP_ALL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V1_L4_EVENT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V1_L4_SYNC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V2_DELAY_REQ

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V2_EVENT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V2_L2_EVENT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V2_L2_SYNC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V2_L4_EVENT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V2_L4_SYNC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_PTP_V2_SYNC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_FILTER_SOME

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_TX_OFF

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_TX_ON

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_TX_ONESTEP_P2P

*Constant*: `c_uint`



## libc::unix::linux_like::linux::HWTSTAMP_TX_ONESTEP_SYNC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::IFA_ADDRESS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFA_ANYCAST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFA_BROADCAST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFA_CACHEINFO

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFA_FLAGS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFA_F_DADFAILED

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_DEPRECATED

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_HOMEADDRESS

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_MANAGETEMPADDR

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_MCAUTOJOIN

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_NODAD

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_NOPREFIXROUTE

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_OPTIMISTIC

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_PERMANENT

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_SECONDARY

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_STABLE_PRIVACY

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_TEMPORARY

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_F_TENTATIVE

*Constant*: `u32`



## libc::unix::linux_like::linux::IFA_LABEL

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFA_LOCAL

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFA_MULTICAST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFA_UNSPEC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFF_DORMANT

*Constant*: `c_int`



## libc::unix::linux_like::linux::IFF_ECHO

*Constant*: `c_int`



## libc::unix::linux_like::linux::IFF_LOWER_UP

*Constant*: `c_int`



## libc::unix::linux_like::linux::IFLA_ADDRESS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_AF_SPEC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_ALLMULTI

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_ALT_IFNAME

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_BROADCAST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_CARRIER

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_CARRIER_CHANGES

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_CARRIER_DOWN_COUNT

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_CARRIER_UP_COUNT

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_COST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_EVENT

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_EXT_MASK

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_GROUP

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_GRO_MAX_SIZE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_GSO_MAX_SEGS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_GSO_MAX_SIZE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_IFALIAS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_IFNAME

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_IF_NETNSID

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_INFO_DATA

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_INFO_KIND

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_INFO_SLAVE_DATA

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_INFO_SLAVE_KIND

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_INFO_UNSPEC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_INFO_XSTATS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_LINK

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_LINKINFO

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_LINKMODE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_LINK_NETNSID

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_MAP

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_MASTER

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_MAX_MTU

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_MIN_MTU

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_MTU

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_NET_NS_FD

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_NET_NS_PID

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_NEW_IFINDEX

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_NEW_NETNSID

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_NUM_RX_QUEUES

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_NUM_TX_QUEUES

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_NUM_VF

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_OPERSTATE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PAD

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PARENT_DEV_BUS_NAME

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PARENT_DEV_NAME

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PERM_ADDRESS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PHYS_PORT_ID

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PHYS_PORT_NAME

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PHYS_SWITCH_ID

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PORT_SELF

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PRIORITY

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PROMISCUITY

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PROP_LIST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PROTINFO

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PROTO_DOWN

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_PROTO_DOWN_REASON

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_QDISC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_STATS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_STATS64

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_TARGET_NETNSID

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_TSO_MAX_SEGS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_TSO_MAX_SIZE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_TXQLEN

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_UNSPEC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_VFINFO_LIST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_VF_PORTS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_WEIGHT

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_WIRELESS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IFLA_XDP

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::IF_LINK_MODE_DEFAULT

*Constant*: `c_int`



## libc::unix::linux_like::linux::IF_LINK_MODE_DORMANT

*Constant*: `c_int`



## libc::unix::linux_like::linux::IF_LINK_MODE_TESTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::IF_OPER_DORMANT

*Constant*: `c_int`



## libc::unix::linux_like::linux::IF_OPER_DOWN

*Constant*: `c_int`



## libc::unix::linux_like::linux::IF_OPER_LOWERLAYERDOWN

*Constant*: `c_int`



## libc::unix::linux_like::linux::IF_OPER_NOTPRESENT

*Constant*: `c_int`



## libc::unix::linux_like::linux::IF_OPER_TESTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::IF_OPER_UNKNOWN

*Constant*: `c_int`



## libc::unix::linux_like::linux::IF_OPER_UP

*Constant*: `c_int`



## libc::unix::linux_like::linux::INOTIFY_MAX_QUEUED_EVENTS

*Constant*: `c_int`



## libc::unix::linux_like::linux::INOTIFY_MAX_USER_INSTANCES

*Constant*: `c_int`



## libc::unix::linux_like::linux::INOTIFY_MAX_USER_WATCHES

*Constant*: `c_int`



## libc::unix::linux_like::linux::INPUT_PROP_ACCELEROMETER

*Constant*: `__u16`



## libc::unix::linux_like::linux::INPUT_PROP_BUTTONPAD

*Constant*: `__u16`



## libc::unix::linux_like::linux::INPUT_PROP_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::INPUT_PROP_DIRECT

*Constant*: `__u16`



## libc::unix::linux_like::linux::INPUT_PROP_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::INPUT_PROP_POINTER

*Constant*: `__u16`



## libc::unix::linux_like::linux::INPUT_PROP_POINTING_STICK

*Constant*: `__u16`



## libc::unix::linux_like::linux::INPUT_PROP_SEMI_MT

*Constant*: `__u16`



## libc::unix::linux_like::linux::INPUT_PROP_TOPBUTTONPAD

*Constant*: `__u16`



## libc::unix::linux_like::linux::IN_ACCESS

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_ALL_EVENTS

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_ATTRIB

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_CLOEXEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::IN_CLOSE

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_CLOSE_NOWRITE

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_CLOSE_WRITE

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_CREATE

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_DELETE

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_DELETE_SELF

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_DONT_FOLLOW

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_EXCL_UNLINK

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_IGNORED

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_ISDIR

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_MASK_ADD

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_MASK_CREATE

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_MODIFY

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_MOVE

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_MOVED_FROM

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_MOVED_TO

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_MOVE_SELF

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_NONBLOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux::IN_ONESHOT

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_ONLYDIR

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_OPEN

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_Q_OVERFLOW

*Constant*: `u32`



## libc::unix::linux_like::linux::IN_UNMOUNT

*Constant*: `u32`



## libc::unix::linux_like::linux::IP6T_SO_ORIGINAL_DST

*Constant*: `c_int`



## libc::unix::linux_like::linux::IPV6_FLOWINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::IPV6_FLOWINFO_FLOWLABEL

*Constant*: `c_int`



## libc::unix::linux_like::linux::IPV6_FLOWINFO_PRIORITY

*Constant*: `c_int`



## libc::unix::linux_like::linux::IPV6_FLOWINFO_SEND

*Constant*: `c_int`



## libc::unix::linux_like::linux::IPV6_FLOWLABEL_MGR

*Constant*: `c_int`



## libc::unix::linux_like::linux::IPV6_FREEBIND

*Constant*: `c_int`



## libc::unix::linux_like::linux::IPV6_RECVFRAGSIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::IP_RECVFRAGSIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::IWEVASSOCREQIE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IWEVASSOCRESPIE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IWEVCUSTOM

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IWEVEXPIRED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IWEVFIRST

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IWEVGENIE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IWEVMICHAELMICFAILURE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IWEVPMKIDCAND

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IWEVQUAL

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IWEVREGISTERED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IWEVTXDROP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_80211_AUTH_ALG

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_ALG_LEAP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_ALG_OPEN_SYSTEM

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_ALG_SHARED_KEY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_CIPHER_AES_CMAC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_CIPHER_CCMP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_CIPHER_GROUP

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_CIPHER_GROUP_MGMT

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_CIPHER_NONE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_CIPHER_PAIRWISE

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_CIPHER_TKIP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_CIPHER_WEP104

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_CIPHER_WEP40

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_DROP_UNENCRYPTED

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_FLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_INDEX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_KEY_MGMT

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_KEY_MGMT_802_1X

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_KEY_MGMT_PSK

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_MFP

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_MFP_DISABLED

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_MFP_OPTIONAL

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_MFP_REQUIRED

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_PRIVACY_INVOKED

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_ROAMING_CONTROL

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_ROAMING_DISABLE

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_ROAMING_ENABLE

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_RX_UNENCRYPTED_EAPOL

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_TKIP_COUNTERMEASURES

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_WPA_ENABLED

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_WPA_VERSION

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_AUTH_WPA_VERSION_DISABLED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_WPA_VERSION_WPA

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_AUTH_WPA_VERSION_WPA2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_CUSTOM_MAX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_ALG_AES_CMAC

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_ENCODE_ALG_CCMP

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_ENCODE_ALG_NONE

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_ENCODE_ALG_PMK

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_ENCODE_ALG_TKIP

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_ENCODE_ALG_WEP

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_ENCODE_DISABLED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_ENABLED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_EXT_GROUP_KEY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_EXT_RX_SEQ_VALID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_EXT_SET_TX_KEY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_EXT_TX_SEQ_VALID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_FLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_INDEX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_MODE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_NOKEY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_OPEN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_RESTRICTED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODE_SEQ_MAX_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_ENCODE_TEMP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENCODING_TOKEN_MAX

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_ENC_CAPA_4WAY_HANDSHAKE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENC_CAPA_CIPHER_CCMP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENC_CAPA_CIPHER_TKIP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENC_CAPA_WPA

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ENC_CAPA_WPA2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_ESSID_MAX_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_EVENT_CAPA_K_0

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_EVENT_CAPA_K_1

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_EV_ADDR_PK_LEN

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_EV_CHAR_PK_LEN

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_EV_FREQ_PK_LEN

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_EV_LCP_PK_LEN

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_EV_PARAM_PK_LEN

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_EV_POINT_PK_LEN

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_EV_QUAL_PK_LEN

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_EV_UINT_PK_LEN

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_FREQ_AUTO

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_FREQ_FIXED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_GENERIC_IE_MAX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_MAX_AP

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MAX_BITRATES

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MAX_ENCODING_SIZES

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MAX_FREQUENCIES

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MAX_SPY

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MAX_TXPOWER

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MICFAILURE_COUNT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_MICFAILURE_GROUP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_MICFAILURE_KEY_ID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_MICFAILURE_PAIRWISE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_MICFAILURE_STAKEY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_MLME_ASSOC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_MLME_AUTH

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_MLME_DEAUTH

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_MLME_DISASSOC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_MODE_ADHOC

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MODE_AUTO

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MODE_INFRA

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MODE_MASTER

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MODE_MESH

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MODE_MONITOR

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MODE_REPEAT

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_MODE_SECOND

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_PMKID_CAND_PREAUTH

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_PMKID_LEN

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_PMKSA_ADD

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_PMKSA_FLUSH

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_PMKSA_REMOVE

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_POWER_ALL_R

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_FORCE_S

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_MAX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_MIN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_MODE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_MODIFIER

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_MULTICAST_R

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_ON

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_PERIOD

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_RELATIVE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_REPEATER

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_TIMEOUT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_TYPE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_POWER_UNICAST_R

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_PRIV_SIZE_FIXED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_PRIV_SIZE_MASK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_PRIV_TYPE_ADDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_PRIV_TYPE_BYTE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_PRIV_TYPE_CHAR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_PRIV_TYPE_FLOAT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_PRIV_TYPE_INT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_PRIV_TYPE_MASK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_PRIV_TYPE_NONE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_QUAL_ALL_INVALID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_QUAL_ALL_UPDATED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_QUAL_DBM

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_QUAL_LEVEL_INVALID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_QUAL_LEVEL_UPDATED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_QUAL_NOISE_INVALID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_QUAL_NOISE_UPDATED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_QUAL_QUAL_INVALID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_QUAL_QUAL_UPDATED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_QUAL_RCPI

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_RETRY_LIFETIME

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_RETRY_LIMIT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_RETRY_LONG

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_RETRY_MAX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_RETRY_MIN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_RETRY_MODIFIER

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_RETRY_ON

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_RETRY_RELATIVE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_RETRY_SHORT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_RETRY_TYPE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_ALL_ESSID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_ALL_FREQ

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_ALL_MODE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_ALL_RATE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_CAPA_BSSID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_CAPA_CHANNEL

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_CAPA_ESSID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_CAPA_MODE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_CAPA_NONE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_CAPA_RATE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_CAPA_TIME

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_CAPA_TYPE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_DEFAULT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_MAX_DATA

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_SCAN_THIS_ESSID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_THIS_FREQ

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_THIS_MODE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_THIS_RATE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_SCAN_TYPE_ACTIVE

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_SCAN_TYPE_PASSIVE

*Constant*: `usize`



## libc::unix::linux_like::linux::IW_TXPOW_DBM

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_TXPOW_MWATT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_TXPOW_RANGE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_TXPOW_RELATIVE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::IW_TXPOW_TYPE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::KERN_ACCT

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_ACPI_VIDEO_FLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_BOOTLOADER_TYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_CADPID

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_COMPAT_LOG

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_CORE_PATTERN

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_CORE_USES_PID

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_CTLALTDEL

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_DOMAINNAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_HOTPLUG

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_HPPA_PWRSW

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_HPPA_UNALIGNED

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_HZ_TIMER

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_IA64_UNALIGNED

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_IEEE_EMULATION_WARNINGS

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_MAX_LOCK_DEPTH

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_MAX_THREADS

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_MODPROBE

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_MSGMAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_MSGMNB

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_MSGMNI

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_MSGPOOL

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_NAMETRANS

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_NGROUPS_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_NMI_WATCHDOG

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_NODENAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_OSRELEASE

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_OSREV

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_OSTYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_OVERFLOWGID

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_OVERFLOWUID

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PANIC

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PANIC_ON_NMI

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PANIC_ON_OOPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PIDMAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PPC_HTABRECLAIM

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PPC_L2CR

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PPC_POWERSAVE_NAP

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PPC_ZEROPAGED

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PRINTK

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PRINTK_RATELIMIT

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PRINTK_RATELIMIT_BURST

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PROF

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_PTY

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_RANDOM

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_RANDOMIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_REALROOTDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_RTSIGMAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_RTSIGNR

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_S390_USER_DEBUG_LOGGING

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SECUREMASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SEM

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SETUID_DUMPABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SG_BIG_BUFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SHMALL

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SHMMAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SHMMNI

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SHMPATH

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SPARC_REBOOT

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SPARC_SCONS_PWROFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SPARC_STOP_A

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SPIN_RETRY

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_SYSRQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_TAINTED

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_UNKNOWN_NMI_PANIC

*Constant*: `c_int`



## libc::unix::linux_like::linux::KERN_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux::KEXEC_ARCH_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::KEXEC_FILE_NO_INITRAMFS

*Constant*: `c_int`



## libc::unix::linux_like::linux::KEXEC_FILE_ON_CRASH

*Constant*: `c_int`



## libc::unix::linux_like::linux::KEXEC_FILE_UNLOAD

*Constant*: `c_int`



## libc::unix::linux_like::linux::KEXEC_ON_CRASH

*Constant*: `c_int`



## libc::unix::linux_like::linux::KEXEC_PRESERVE_CONTEXT

*Constant*: `c_int`



## libc::unix::linux_like::linux::KEY_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::KEY_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::LED_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::LED_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::LINUX_REBOOT_CMD_CAD_OFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_CMD_CAD_ON

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_CMD_HALT

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_CMD_KEXEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_CMD_POWER_OFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_CMD_RESTART

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_CMD_RESTART2

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_CMD_SW_SUSPEND

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_MAGIC1

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_MAGIC2

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_MAGIC2A

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_MAGIC2B

*Constant*: `c_int`



## libc::unix::linux_like::linux::LINUX_REBOOT_MAGIC2C

*Constant*: `c_int`



## libc::unix::linux_like::linux::MAP_DROPPABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux::MAP_SHARED_VALIDATE

*Constant*: `c_int`



## libc::unix::linux_like::linux::MAX_HANDLE_SZ

*Constant*: `c_int`



## libc::unix::linux_like::linux::MNT_NS_INFO_SIZE_VER0

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::MODULE_INIT_IGNORE_MODVERSIONS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::MODULE_INIT_IGNORE_VERMAGIC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::MOUNT_ATTR_IDMAP

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MOUNT_ATTR_NOATIME

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MOUNT_ATTR_NODEV

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MOUNT_ATTR_NODIRATIME

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MOUNT_ATTR_NOEXEC

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MOUNT_ATTR_NOSUID

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MOUNT_ATTR_NOSYMFOLLOW

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MOUNT_ATTR_RDONLY

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MOUNT_ATTR_RELATIME

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MOUNT_ATTR_SIZE_VER0

*Constant*: `c_int`



## libc::unix::linux_like::linux::MOUNT_ATTR_STRICTATIME

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MOUNT_ATTR__ATIME

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::MPOL_BIND

*Constant*: `c_int`



## libc::unix::linux_like::linux::MPOL_DEFAULT

*Constant*: `c_int`



## libc::unix::linux_like::linux::MPOL_F_NUMA_BALANCING

*Constant*: `c_int`



## libc::unix::linux_like::linux::MPOL_F_RELATIVE_NODES

*Constant*: `c_int`



## libc::unix::linux_like::linux::MPOL_F_STATIC_NODES

*Constant*: `c_int`



## libc::unix::linux_like::linux::MPOL_INTERLEAVE

*Constant*: `c_int`



## libc::unix::linux_like::linux::MPOL_LOCAL

*Constant*: `c_int`



## libc::unix::linux_like::linux::MPOL_PREFERRED

*Constant*: `c_int`



## libc::unix::linux_like::linux::MREMAP_DONTUNMAP

*Constant*: `c_int`



## libc::unix::linux_like::linux::MREMAP_FIXED

*Constant*: `c_int`



## libc::unix::linux_like::linux::MREMAP_MAYMOVE

*Constant*: `c_int`



## libc::unix::linux_like::linux::MSC_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::MSC_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::MSG_EXCEPT

*Constant*: `c_int`



## libc::unix::linux_like::linux::MSG_INFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::MSG_NOERROR

*Constant*: `c_int`



## libc::unix::linux_like::linux::MSG_NOTIFICATION

*Constant*: `c_int`



## libc::unix::linux_like::linux::MSG_STAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::MSG_ZEROCOPY

*Constant*: `c_int`



## libc::unix::linux_like::linux::NDA_CACHEINFO

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::NDA_DST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::NDA_IFINDEX

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::NDA_LLADDR

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::NDA_PORT

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::NDA_PROBES

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::NDA_UNSPEC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::NDA_VLAN

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::NDA_VNI

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::NET_802

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_ATALK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_AX25

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_BRIDGE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_CORE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_DCCP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_DECNET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_ECONET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_ETHER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_IPV4

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_IPV6

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_IPX

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_IRDA

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_LLC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_NETFILTER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_NETROM

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_ROSE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_SCTP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_TR

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_UNIX

*Constant*: `c_int`



## libc::unix::linux_like::linux::NET_X25

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNETLINK_V0

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNLGRP_ACCT_QUOTA

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNLGRP_CONNTRACK_DESTROY

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNLGRP_CONNTRACK_EXP_DESTROY

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNLGRP_CONNTRACK_EXP_NEW

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNLGRP_CONNTRACK_EXP_UPDATE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNLGRP_CONNTRACK_NEW

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNLGRP_CONNTRACK_UPDATE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNLGRP_NFTABLES

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNLGRP_NFTRACE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNLGRP_NONE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_BATCH_GENID

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_BATCH_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_MSG_BATCH_BEGIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_MSG_BATCH_END

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_ACCT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_COUNT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_CTHELPER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_CTNETLINK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_CTNETLINK_EXP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_CTNETLINK_TIMEOUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_HOOK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_IPSET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_NFTABLES

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_NFT_COMPAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_NONE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_OSF

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_QUEUE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFNL_SUBSYS_ULOG

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFPROTO_ARP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFPROTO_BRIDGE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFPROTO_DECNET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFPROTO_INET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFPROTO_IPV4

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFPROTO_IPV6

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFPROTO_NETDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFPROTO_NUMPROTO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFPROTO_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CAP_LEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_CMD

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_FLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_F_CONNTRACK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_F_FAIL_OPEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_F_GSO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_F_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_F_SECCTX

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_F_UID_GID

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_PARAMS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_QUEUE_MAXLEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CFG_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_CT_INFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_EXP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_GID

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_HWADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_IFINDEX_INDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_IFINDEX_OUTDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_IFINDEX_PHYSINDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_IFINDEX_PHYSOUTDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_L2HDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_MARK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_PACKET_HDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_PAYLOAD

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_PRIORITY

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_SECCTX

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_SKB_CSUMNOTREADY

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_SKB_CSUM_NOTVERIFIED

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_SKB_GSO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_SKB_INFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_TIMESTAMP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_UID

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_VERDICT_HDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_VLAN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_VLAN_PROTO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_VLAN_TCI

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQA_VLAN_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_CFG_CMD_BIND

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_CFG_CMD_NONE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_CFG_CMD_PF_BIND

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_CFG_CMD_PF_UNBIND

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_CFG_CMD_UNBIND

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_COPY_META

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_COPY_NONE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_COPY_PACKET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_MSG_CONFIG

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_MSG_PACKET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_MSG_VERDICT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFQNL_MSG_VERDICT_BATCH

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_BREAK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_BYTEORDER_HTON

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_BYTEORDER_NTOH

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CHAIN_MAXNAMELEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CMP_EQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CMP_GT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CMP_GTE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CMP_LT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CMP_LTE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CMP_NEQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CONTINUE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_AVGPKT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_BYTES

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_DIRECTION

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_DST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_DST_IP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_DST_IP6

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_EVENTMASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_EXPIRATION

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_HELPER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_L3PROTOCOL

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_LABELS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_MARK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_PKTS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_PROTOCOL

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_PROTO_DST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_PROTO_SRC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_SECMARK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_SRC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_SRC_IP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_SRC_IP6

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_STATE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_STATUS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_CT_ZONE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_DATA_RESERVED_MASK

*Constant*: `c_uint`



## libc::unix::linux_like::linux::NFT_DATA_VALUE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::NFT_DATA_VALUE_MAXLEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_DATA_VERDICT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::NFT_DYNSET_F_INV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_DYNSET_OP_ADD

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_DYNSET_OP_UPDATE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_GOTO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_JUMP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_LIMIT_F_INV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_LIMIT_PKTS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_LIMIT_PKT_BYTES

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_LOOKUP_F_INV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_BRI_IIFNAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_BRI_OIFNAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_CGROUP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_CPU

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_IIF

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_IIFGROUP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_IIFNAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_IIFTYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_L4PROTO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_LEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_MARK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_NFPROTO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_NFTRACE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_OIF

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_OIFGROUP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_OIFNAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_OIFTYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_PKTTYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_PRANDOM

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_PRIORITY

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_PROTOCOL

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_RTCLASSID

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_SECMARK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_SKGID

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_META_SKUID

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_DELCHAIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_DELOBJ

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_DELRULE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_DELSET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_DELSETELEM

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_DELTABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_GETCHAIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_GETGEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_GETOBJ

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_GETOBJ_RESET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_GETRULE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_GETSET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_GETSETELEM

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_GETTABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_NEWCHAIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_NEWGEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_NEWOBJ

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_NEWRULE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_NEWSET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_NEWSETELEM

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_NEWTABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_MSG_TRACE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_NAT_DNAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_NAT_SNAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_NG_INCREMENTAL

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_NG_RANDOM

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_OBJ_MAXNAMELEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_PAYLOAD_CSUM_INET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_PAYLOAD_CSUM_NONE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_PAYLOAD_LL_HEADER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_PAYLOAD_NETWORK_HEADER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_PAYLOAD_TRANSPORT_HEADER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_QUEUE_FLAG_BYPASS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_QUEUE_FLAG_CPU_FANOUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_QUEUE_FLAG_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_QUOTA_F_INV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_RANGE_EQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_RANGE_NEQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_00

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_01

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_02

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_03

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_04

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_05

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_06

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_07

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_08

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_09

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_10

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_11

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_12

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_13

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_14

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_15

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG32_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG_1

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG_2

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG_3

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG_4

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REG_VERDICT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REJECT_ICMPX_ADMIN_PROHIBITED

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REJECT_ICMPX_HOST_UNREACH

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REJECT_ICMPX_NO_ROUTE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REJECT_ICMPX_PORT_UNREACH

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REJECT_ICMPX_UNREACH

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REJECT_ICMP_UNREACH

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_REJECT_TCP_RST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_RETURN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_SET_ANONYMOUS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_SET_CONSTANT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_SET_ELEM_INTERVAL_END

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_SET_EVAL

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_SET_INTERVAL

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_SET_MAP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_SET_MAXNAMELEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_SET_POL_MEMORY

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_SET_POL_PERFORMANCE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_SET_TIMEOUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_TABLE_MAXNAMELEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_TRACETYPE_POLICY

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_TRACETYPE_RETURN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_TRACETYPE_RULE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_TRACETYPE_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFT_USERDATA_MAXLEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_CFG_CMD

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_CFG_FLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_CFG_MODE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_CFG_NLBUFSIZ

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_CFG_QTHRESH

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_CFG_TIMEOUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_CFG_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_CT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_CT_INFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_GID

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_HWADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_HWHEADER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_HWLEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_HWTYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_IFINDEX_INDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_IFINDEX_OUTDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_IFINDEX_PHYSINDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_IFINDEX_PHYSOUTDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_L2HDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_MARK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_PACKET_HDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_PAYLOAD

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_PREFIX

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_SEQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_SEQ_GLOBAL

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_TIMESTAMP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_UID

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_VLAN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_VLAN_PROTO

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_VLAN_TCI

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULA_VLAN_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_CFG_CMD_BIND

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_CFG_CMD_NONE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_CFG_CMD_PF_BIND

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_CFG_CMD_PF_UNBIND

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_CFG_CMD_UNBIND

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_CFG_F_CONNTRACK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_CFG_F_SEQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_CFG_F_SEQ_GLOBAL

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_COPY_META

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_COPY_NONE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_COPY_PACKET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_MSG_CONFIG

*Constant*: `c_int`



## libc::unix::linux_like::linux::NFULNL_MSG_PACKET

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_ACCEPT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_ARP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_ARP_FORWARD

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_ARP_IN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_ARP_NUMHOOKS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_ARP_OUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_BROUTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_FORWARD

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_LOCAL_IN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_LOCAL_OUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_NUMHOOKS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_POST_ROUTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_PRE_ROUTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_PRI_BRNF

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_PRI_FILTER_BRIDGED

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_PRI_FILTER_OTHER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_PRI_FIRST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_PRI_LAST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_PRI_NAT_DST_BRIDGED

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_PRI_NAT_DST_OTHER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_BR_PRI_NAT_SRC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_DROP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_INET_FORWARD

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_INET_INGRESS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_INET_LOCAL_IN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_INET_LOCAL_OUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_INET_NUMHOOKS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_INET_POST_ROUTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_INET_PRE_ROUTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_FORWARD

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_LOCAL_IN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_LOCAL_OUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_NUMHOOKS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_POST_ROUTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRE_ROUTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_CONNTRACK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_CONNTRACK_DEFRAG

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_CONNTRACK_HELPER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_FILTER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_FIRST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_LAST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_MANGLE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_NAT_DST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_NAT_SRC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_RAW

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_RAW_BEFORE_DEFRAG

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_SECURITY

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_SELINUX_FIRST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP6_PRI_SELINUX_LAST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_FORWARD

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_LOCAL_IN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_LOCAL_OUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_NUMHOOKS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_POST_ROUTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRE_ROUTING

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_CONNTRACK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_CONNTRACK_CONFIRM

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_CONNTRACK_DEFRAG

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_CONNTRACK_HELPER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_FILTER

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_FIRST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_LAST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_MANGLE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_NAT_DST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_NAT_SRC

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_RAW

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_RAW_BEFORE_DEFRAG

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_SECURITY

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_SELINUX_FIRST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_IP_PRI_SELINUX_LAST

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_MAX_VERDICT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_NETDEV_EGRESS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_NETDEV_INGRESS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_NETDEV_NUMHOOKS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_QUEUE

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_REPEAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_STOLEN

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_STOP

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_VERDICT_BITS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_VERDICT_FLAG_QUEUE_BYPASS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_VERDICT_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_VERDICT_QBITS

*Constant*: `c_int`



## libc::unix::linux_like::linux::NF_VERDICT_QMASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NLM_F_BULK

*Constant*: `c_int`



## libc::unix::linux_like::linux::NS_GET_MNTNS_ID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_GET_NSTYPE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_GET_OWNER_UID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_GET_PARENT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_GET_PID_FROM_PIDNS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_GET_PID_IN_PIDNS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_GET_TGID_FROM_PIDNS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_GET_TGID_IN_PIDNS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_GET_USERNS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_MNT_GET_INFO

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_MNT_GET_NEXT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NS_MNT_GET_PREV

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::NTF_MASTER

*Constant*: `u8`



## libc::unix::linux_like::linux::NTF_PROXY

*Constant*: `u8`



## libc::unix::linux_like::linux::NTF_ROUTER

*Constant*: `u8`



## libc::unix::linux_like::linux::NTF_SELF

*Constant*: `u8`



## libc::unix::linux_like::linux::NTF_USE

*Constant*: `u8`



## libc::unix::linux_like::linux::NUD_DELAY

*Constant*: `u16`



## libc::unix::linux_like::linux::NUD_FAILED

*Constant*: `u16`



## libc::unix::linux_like::linux::NUD_INCOMPLETE

*Constant*: `u16`



## libc::unix::linux_like::linux::NUD_NOARP

*Constant*: `u16`



## libc::unix::linux_like::linux::NUD_NONE

*Constant*: `u16`



## libc::unix::linux_like::linux::NUD_PERMANENT

*Constant*: `u16`



## libc::unix::linux_like::linux::NUD_PROBE

*Constant*: `u16`



## libc::unix::linux_like::linux::NUD_REACHABLE

*Constant*: `u16`



## libc::unix::linux_like::linux::NUD_STALE

*Constant*: `u16`



## libc::unix::linux_like::linux::OPEN_TREE_CLOEXEC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::OPEN_TREE_CLONE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::PACKET_FANOUT_CBPF

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_CPU

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_DATA

*Constant*: `c_int`



## libc::unix::linux_like::linux::PACKET_FANOUT_EBPF

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_FLAG_DEFRAG

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_FLAG_IGNORE_OUTGOING

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_FLAG_ROLLOVER

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_FLAG_UNIQUEID

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_HASH

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_LB

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_QM

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_RND

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_FANOUT_ROLLOVER

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PACKET_IGNORE_OUTGOING

*Constant*: `c_int`



## libc::unix::linux_like::linux::PACKET_QDISC_BYPASS

*Constant*: `c_int`



## libc::unix::linux_like::linux::PACKET_ROLLOVER_STATS

*Constant*: `c_int`



## libc::unix::linux_like::linux::PACKET_TX_HAS_OFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::PACKET_VNET_HDR_SZ

*Constant*: `c_int`



## libc::unix::linux_like::linux::PF_BLOCK_TS

*Constant*: `c_int`

Plug has ts that needs updating.



## libc::unix::linux_like::linux::PF_DUMPCORE

*Constant*: `c_int`

Dumped core.



## libc::unix::linux_like::linux::PF_EXITING

*Constant*: `c_int`

Getting shut down.



## libc::unix::linux_like::linux::PF_FORKNOEXEC

*Constant*: `c_int`

Forked but didn't exec.



## libc::unix::linux_like::linux::PF_IDLE

*Constant*: `c_int`

I am an IDLE thread.



## libc::unix::linux_like::linux::PF_IO_WORKER

*Constant*: `c_int`

Task is an IO worker.



## libc::unix::linux_like::linux::PF_KSWAPD

*Constant*: `c_int`

I am `kswapd`.



## libc::unix::linux_like::linux::PF_KTHREAD

*Constant*: `c_int`

I am a kernel thread.



## libc::unix::linux_like::linux::PF_LOCAL_THROTTLE

*Constant*: `c_int`

Throttle writes only against the bdi I write to, I am cleaning
dirty pages from some other bdi.



## libc::unix::linux_like::linux::PF_MCE_EARLY

*Constant*: `c_int`

Early kill for mce process policy.



## libc::unix::linux_like::linux::PF_MCE_PROCESS

*Constant*: `c_int`

Process policy on mce errors.



## libc::unix::linux_like::linux::PF_MEMALLOC

*Constant*: `c_int`

Allocating memory to free memory.

See `memalloc_noreclaim_save()`.



## libc::unix::linux_like::linux::PF_MEMALLOC_NOFS

*Constant*: `c_int`

All allocations inherit `GFP_NOFS`.

See `memalloc_nfs_save()`.



## libc::unix::linux_like::linux::PF_MEMALLOC_NOIO

*Constant*: `c_int`

All allocations inherit `GFP_NOIO`.

See `memalloc_noio_save()`.



## libc::unix::linux_like::linux::PF_MEMALLOC_PIN

*Constant*: `c_int`

Allocations constrained to zones which allow long term pinning.

See `memalloc_pin_save()`.



## libc::unix::linux_like::linux::PF_NOFREEZE

*Constant*: `c_int`

This thread should not be frozen.



## libc::unix::linux_like::linux::PF_NO_SETAFFINITY

*Constant*: `c_int`

Userland is not allowed to meddle with `cpus_mask`.



## libc::unix::linux_like::linux::PF_NPROC_EXCEEDED

*Constant*: `c_int`

`set_user()` noticed that `RLIMIT_NPROC` was exceeded.



## libc::unix::linux_like::linux::PF_POSTCOREDUMP

*Constant*: `c_int`

Coredumps should ignore this task.



## libc::unix::linux_like::linux::PF_RANDOMIZE

*Constant*: `c_int`

Randomize virtual address space.



## libc::unix::linux_like::linux::PF_SIGNALED

*Constant*: `c_int`

Killed by a signal.



## libc::unix::linux_like::linux::PF_SUPERPRIV

*Constant*: `c_int`

Used super-user privileges.



## libc::unix::linux_like::linux::PF_SUSPEND_TASK

*Constant*: `c_int`

This thread called `freeze_processes()` and should not be frozen.



## libc::unix::linux_like::linux::PF_USED_MATH

*Constant*: `c_int`

If unset the fpu must be initialized before use.



## libc::unix::linux_like::linux::PF_USER_WORKER

*Constant*: `c_int`

Kernel thread cloned from userspace thread.



## libc::unix::linux_like::linux::PF_VCPU

*Constant*: `c_int`

I'm a virtual CPU.



## libc::unix::linux_like::linux::PF_WQ_WORKER

*Constant*: `c_int`

I'm a workqueue worker.



## libc::unix::linux_like::linux::PIDTYPE_MAX

*Constant*: `pid_type`



## libc::unix::linux_like::linux::PIDTYPE_PGID

*Constant*: `pid_type`



## libc::unix::linux_like::linux::PIDTYPE_PID

*Constant*: `pid_type`



## libc::unix::linux_like::linux::PIDTYPE_SID

*Constant*: `pid_type`



## libc::unix::linux_like::linux::PIDTYPE_TGID

*Constant*: `pid_type`



## libc::unix::linux_like::linux::POSIX_SPAWN_RESETIDS

*Constant*: `c_int`



## libc::unix::linux_like::linux::POSIX_SPAWN_SETPGROUP

*Constant*: `c_int`



## libc::unix::linux_like::linux::POSIX_SPAWN_SETSCHEDPARAM

*Constant*: `c_int`



## libc::unix::linux_like::linux::POSIX_SPAWN_SETSCHEDULER

*Constant*: `c_int`



## libc::unix::linux_like::linux::POSIX_SPAWN_SETSID

*Constant*: `c_int`



## libc::unix::linux_like::linux::POSIX_SPAWN_SETSIGDEF

*Constant*: `c_int`



## libc::unix::linux_like::linux::POSIX_SPAWN_SETSIGMASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::POSIX_SPAWN_USEVFORK

*Constant*: `c_int`



## libc::unix::linux_like::linux::PROC_CN_MCAST_IGNORE

*Constant*: `proc_cn_mcast_op`



## libc::unix::linux_like::linux::PROC_CN_MCAST_LISTEN

*Constant*: `proc_cn_mcast_op`



## libc::unix::linux_like::linux::PROC_EVENT_COMM

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PROC_EVENT_COREDUMP

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PROC_EVENT_EXEC

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PROC_EVENT_EXIT

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PROC_EVENT_FORK

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PROC_EVENT_GID

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PROC_EVENT_NONE

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PROC_EVENT_NONZERO_EXIT

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PROC_EVENT_PTRACE

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PROC_EVENT_SID

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PROC_EVENT_UID

*Constant*: `proc_cn_event`



## libc::unix::linux_like::linux::PR_GET_MDWE

*Constant*: `c_int`



## libc::unix::linux_like::linux::PR_MDWE_NO_INHERIT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PR_MDWE_REFUSE_EXEC_GAIN

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PR_SET_MDWE

*Constant*: `c_int`



## libc::unix::linux_like::linux::PTHREAD_COND_INITIALIZER

*Constant*: `crate::pthread_cond_t`



## libc::unix::linux_like::linux::PTHREAD_MUTEX_INITIALIZER

*Constant*: `crate::pthread_mutex_t`



## libc::unix::linux_like::linux::PTHREAD_RWLOCK_INITIALIZER

*Constant*: `crate::pthread_rwlock_t`



## libc::unix::linux_like::linux::PTP_CLOCK_GETCAPS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_CLOCK_GETCAPS2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_ENABLE_PPS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_ENABLE_PPS2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_EXTTS_REQUEST

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_EXTTS_REQUEST2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_MAX_SAMPLES

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PTP_PEROUT_REQUEST

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_PEROUT_REQUEST2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_PF_EXTTS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PTP_PF_NONE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PTP_PF_PEROUT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PTP_PF_PHYSYNC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::PTP_PIN_GETFUNC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_PIN_GETFUNC2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_PIN_SETFUNC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_PIN_SETFUNC2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_SYS_OFFSET

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_SYS_OFFSET2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_SYS_OFFSET_EXTENDED

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_SYS_OFFSET_EXTENDED2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_SYS_OFFSET_PRECISE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::PTP_SYS_OFFSET_PRECISE2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::QFMT_VFS_OLD

*Constant*: `c_int`



## libc::unix::linux_like::linux::QFMT_VFS_V0

*Constant*: `c_int`



## libc::unix::linux_like::linux::QFMT_VFS_V1

*Constant*: `c_int`



## libc::unix::linux_like::linux::RB_AUTOBOOT

*Constant*: `c_int`



## libc::unix::linux_like::linux::RB_DISABLE_CAD

*Constant*: `c_int`



## libc::unix::linux_like::linux::RB_ENABLE_CAD

*Constant*: `c_int`



## libc::unix::linux_like::linux::RB_HALT_SYSTEM

*Constant*: `c_int`



## libc::unix::linux_like::linux::RB_KEXEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::RB_POWER_OFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::RB_SW_SUSPEND

*Constant*: `c_int`



## libc::unix::linux_like::linux::REL_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::REL_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::RENAME_EXCHANGE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RENAME_NOREPLACE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RENAME_WHITEOUT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::REP_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::REP_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::RESOLVE_BENEATH

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::RESOLVE_CACHED

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::RESOLVE_IN_ROOT

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::RESOLVE_NO_MAGICLINKS

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::RESOLVE_NO_SYMLINKS

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::RESOLVE_NO_XDEV

*Constant*: `crate::__u64`



## libc::unix::linux_like::linux::RTA_CACHEINFO

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_DST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_FLOW

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_GATEWAY

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_IIF

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_MARK

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_METRICS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_MFC_STATS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_MP_ALGO

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_MULTIPATH

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_OIF

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_PREFSRC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_PRIORITY

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_PROTOINFO

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_SESSION

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_SRC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_TABLE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTA_UNSPEC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::RTEXT_FILTER_BRVLAN

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTEXT_FILTER_BRVLAN_COMPRESSED

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTEXT_FILTER_CFM_CONFIG

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTEXT_FILTER_CFM_STATUS

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTEXT_FILTER_MRP

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTEXT_FILTER_SKIP_STATS

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTEXT_FILTER_VF

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_DECnet_IFADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_DECnet_ROUTE

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_IPV4_IFADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_IPV4_MROUTE

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_IPV4_ROUTE

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_IPV4_RULE

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_IPV6_IFADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_IPV6_IFINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_IPV6_MROUTE

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_IPV6_PREFIX

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_IPV6_ROUTE

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_LINK

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_NEIGH

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_NOTIFY

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMGRP_TC

*Constant*: `c_int`



## libc::unix::linux_like::linux::RTMSG_AR_FAILED

*Constant*: `u32`



## libc::unix::linux_like::linux::RTMSG_CONTROL

*Constant*: `u32`



## libc::unix::linux_like::linux::RTMSG_DELDEVICE

*Constant*: `u32`



## libc::unix::linux_like::linux::RTMSG_DELROUTE

*Constant*: `u32`



## libc::unix::linux_like::linux::RTMSG_DELRULE

*Constant*: `u32`



## libc::unix::linux_like::linux::RTMSG_NEWDEVICE

*Constant*: `u32`



## libc::unix::linux_like::linux::RTMSG_NEWROUTE

*Constant*: `u32`



## libc::unix::linux_like::linux::RTMSG_NEWRULE

*Constant*: `u32`



## libc::unix::linux_like::linux::RTMSG_OVERRUN

*Constant*: `u32`



## libc::unix::linux_like::linux::RTM_DELACTION

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELADDR

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELADDRLABEL

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELLINK

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELMDB

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELNEIGH

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELNSID

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELQDISC

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELROUTE

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELRULE

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELTCLASS

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_DELTFILTER

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_F_CLONED

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTM_F_EQUALIZE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTM_F_NOTIFY

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTM_F_PREFIX

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTM_GETACTION

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETADDR

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETADDRLABEL

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETANYCAST

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETDCB

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETLINK

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETMDB

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETMULTICAST

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETNEIGH

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETNEIGHTBL

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETNETCONF

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETNSID

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETQDISC

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETROUTE

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETRULE

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETTCLASS

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_GETTFILTER

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWACTION

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWADDR

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWADDRLABEL

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWLINK

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWMDB

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWNDUSEROPT

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWNEIGH

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWNEIGHTBL

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWNETCONF

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWNSID

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWPREFIX

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWQDISC

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWROUTE

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWRULE

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWTCLASS

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_NEWTFILTER

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_SETDCB

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_SETLINK

*Constant*: `u16`



## libc::unix::linux_like::linux::RTM_SETNEIGHTBL

*Constant*: `u16`



## libc::unix::linux_like::linux::RTNLGRP_BRVLAN

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_DCB

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_DECnet_IFADDR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_DECnet_ROUTE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_DECnet_RULE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV4_IFADDR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV4_MROUTE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV4_MROUTE_R

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV4_NETCONF

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV4_ROUTE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV4_RULE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV6_IFADDR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV6_IFINFO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV6_MROUTE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV6_MROUTE_R

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV6_NETCONF

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV6_PREFIX

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV6_ROUTE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_IPV6_RULE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_LINK

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_MCTP_IFADDR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_MDB

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_MPLS_NETCONF

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_MPLS_ROUTE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_ND_USEROPT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_NEIGH

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_NEXTHOP

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_NONE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_NOP2

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_NOP4

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_NOTIFY

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_NSID

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_PHONET_IFADDR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_PHONET_ROUTE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_STATS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_TC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTNLGRP_TUNNEL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::RTN_ANYCAST

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_BLACKHOLE

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_BROADCAST

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_LOCAL

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_MULTICAST

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_NAT

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_PROHIBIT

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_THROW

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_UNICAST

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_UNREACHABLE

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_UNSPEC

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTN_XRESOLVE

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTPROT_BOOT

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTPROT_KERNEL

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTPROT_REDIRECT

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTPROT_STATIC

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RTPROT_UNSPEC

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RT_SCOPE_HOST

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RT_SCOPE_LINK

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RT_SCOPE_NOWHERE

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RT_SCOPE_SITE

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RT_SCOPE_UNIVERSE

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RT_TABLE_COMPAT

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RT_TABLE_DEFAULT

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RT_TABLE_LOCAL

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RT_TABLE_MAIN

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RT_TABLE_UNSPEC

*Constant*: `c_uchar`



## libc::unix::linux_like::linux::RWF_APPEND

*Constant*: `c_int`



## libc::unix::linux_like::linux::RWF_ATOMIC

*Constant*: `c_int`



## libc::unix::linux_like::linux::RWF_DONTCACHE

*Constant*: `c_int`



## libc::unix::linux_like::linux::RWF_DSYNC

*Constant*: `c_int`



## libc::unix::linux_like::linux::RWF_HIPRI

*Constant*: `c_int`



## libc::unix::linux_like::linux::RWF_NOAPPEND

*Constant*: `c_int`



## libc::unix::linux_like::linux::RWF_NOWAIT

*Constant*: `c_int`



## libc::unix::linux_like::linux::RWF_SYNC

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCHED_FLAG_ALL

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCHED_FLAG_DL_OVERRUN

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCHED_FLAG_KEEP_ALL

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCHED_FLAG_KEEP_PARAMS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCHED_FLAG_KEEP_POLICY

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCHED_FLAG_RECLAIM

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCHED_FLAG_RESET_ON_FORK

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCHED_FLAG_UTIL_CLAMP

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCHED_FLAG_UTIL_CLAMP_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCHED_FLAG_UTIL_CLAMP_MIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_ABORT

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_ADAPTATION_LAYER

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_ADDR_OVER

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_ALL_ASSOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_ASSOCINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_AUTHINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_AUTH_ACTIVE_KEY

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_AUTH_CHUNK

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_AUTH_DEACTIVATE_KEY

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_AUTH_DELETE_KEY

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_AUTH_KEY

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_AUTOCLOSE

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_AUTO_ASCONF

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_CONTEXT

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_CURRENT_ASSOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_DEFAULT_SEND_PARAM

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_DEFAULT_SNDINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_DELAYED_ACK

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_DELAYED_ACK_TIME

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_DELAYED_SACK

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_DISABLE_FRAGMENTS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_DSTADDRV4

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_DSTADDRV6

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_ENABLE_CHANGE_ASSOC_REQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_ENABLE_RESET_ASSOC_REQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_ENABLE_RESET_STREAM_REQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_ENABLE_STRRESET_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_EOF

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_EVENTS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_FRAGMENT_INTERLEAVE

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_FUTURE_ASSOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_GET_ASSOC_ID_LIST

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_GET_ASSOC_NUMBER

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_GET_PEER_ADDR_INFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_HMAC_IDENT

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_INIT

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_INITMSG

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_I_WANT_MAPPED_V4_ADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_LOCAL_AUTH_CHUNKS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_MAXSEG

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_MAX_BURST

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_NODELAY

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_NOTIFICATION

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_NXTINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PARTIAL_DELIVERY_POINT

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PEER_ADDR_PARAMS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PEER_ADDR_THLDS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PEER_ADDR_THLDS_V2

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PEER_AUTH_CHUNKS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PRIMARY_ADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PRINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PR_INDEX

*Function*

```rust
fn SCTP_PR_INDEX(policy: c_int) -> c_int
```



## libc::unix::linux_like::linux::SCTP_PR_POLICY

*Function*

```rust
fn SCTP_PR_POLICY(policy: c_int) -> c_int
```



## libc::unix::linux_like::linux::SCTP_PR_PRIO_ENABLED

*Function*

```rust
fn SCTP_PR_PRIO_ENABLED(policy: c_int) -> bool
```



## libc::unix::linux_like::linux::SCTP_PR_RTX_ENABLED

*Function*

```rust
fn SCTP_PR_RTX_ENABLED(policy: c_int) -> bool
```



## libc::unix::linux_like::linux::SCTP_PR_SCTP_ALL

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PR_SCTP_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PR_SCTP_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PR_SCTP_NONE

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PR_SCTP_PRIO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PR_SCTP_RTX

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PR_SCTP_TTL

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_PR_SET_POLICY

*Function*

```rust
fn SCTP_PR_SET_POLICY(flags: & mut c_int, policy: c_int)
```



## libc::unix::linux_like::linux::SCTP_PR_TTL_ENABLED

*Function*

```rust
fn SCTP_PR_TTL_ENABLED(policy: c_int) -> bool
```



## libc::unix::linux_like::linux::SCTP_RCVINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_RECVNXTINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_RECVRCVINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_REUSE_PORT

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_RTOINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_SACK_IMMEDIATELY

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_SENDALL

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_SET_PEER_PRIMARY_ADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_SNDINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_SNDRCV

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_STATUS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_STREAM_RESET_INCOMING

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_STREAM_RESET_OUTGOING

*Constant*: `c_int`



## libc::unix::linux_like::linux::SCTP_UNORDERED

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_EXEC_DENY_INTERACTIVE

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_EXEC_DENY_INTERACTIVE_LOCKED

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_EXEC_RESTRICT_FILE

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_EXEC_RESTRICT_FILE_LOCKED

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_KEEP_CAPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_KEEP_CAPS_LOCKED

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_NOROOT

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_NOROOT_LOCKED

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_NO_CAP_AMBIENT_RAISE

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_NO_SETUID_FIXUP

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECBIT_NO_SETUID_FIXUP_LOCKED

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECCOMP_ADDFD_FLAG_SEND

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SECCOMP_ADDFD_FLAG_SETFD

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SECCOMP_FILTER_FLAG_LOG

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SECCOMP_FILTER_FLAG_NEW_LISTENER

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SECCOMP_FILTER_FLAG_SPEC_ALLOW

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SECCOMP_FILTER_FLAG_TSYNC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SECCOMP_FILTER_FLAG_TSYNC_ESRCH

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SECCOMP_GET_ACTION_AVAIL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_GET_NOTIF_SIZES

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_MODE_DISABLED

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_MODE_FILTER

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_MODE_STRICT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_ACTION

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_ACTION_FULL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_ALLOW

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_DATA

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_ERRNO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_KILL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_KILL_PROCESS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_KILL_THREAD

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_LOG

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_TRACE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_TRAP

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_RET_USER_NOTIF

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_SET_MODE_FILTER

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_SET_MODE_STRICT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SECCOMP_USER_NOTIF_FLAG_CONTINUE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SECUREBITS_DEFAULT

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECURE_ALL_BITS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECURE_ALL_LOCKS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SECURE_ALL_UNPRIVILEGED

*Constant*: `c_int`



## libc::unix::linux_like::linux::SEEK_DATA

*Constant*: `c_int`



## libc::unix::linux_like::linux::SEEK_HOLE

*Constant*: `c_int`



## libc::unix::linux_like::linux::SEM_INFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SEM_STAT

*Constant*: `c_int`



## libc::unix::linux_like::linux::SEM_STAT_ANY

*Constant*: `c_int`



## libc::unix::linux_like::linux::SEM_UNDO

*Constant*: `c_int`



## libc::unix::linux_like::linux::SETALL

*Constant*: `c_int`



## libc::unix::linux_like::linux::SETVAL

*Constant*: `c_int`



## libc::unix::linux_like::linux::SIOCGHWTSTAMP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWAP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWAPLIST

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWAUTH

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWENCODE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWENCODEEXT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWESSID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWFRAG

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWFREQ

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWGENIE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWMODE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWNAME

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWNICKN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWNWID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWPOWER

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWPRIV

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWRANGE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWRATE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWRETRY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWRTS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWSCAN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWSENS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWSPY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWSTATS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWTHRSPY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCGIWTXPOW

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCIWFIRST

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCIWFIRSTPRIV

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCIWLAST

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCIWLASTPRIV

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSHWTSTAMP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWAP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWAUTH

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWCOMMIT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWENCODE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWENCODEEXT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWESSID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWFRAG

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWFREQ

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWGENIE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWMLME

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWMODE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWNICKN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWNWID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWPMKSA

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWPOWER

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWPRIV

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWRANGE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWRATE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWRETRY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWRTS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWSCAN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWSENS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWSPY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWSTATS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWTHRSPY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SIOCSIWTXPOW

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::SI_DETHREAD

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_ALU_XOR_X

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_CPU

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_HATYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_IFINDEX

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_MARK

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_NLATTR

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_NLATTR_NEST

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_OFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_PAY_OFFSET

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_PKTTYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_PROTOCOL

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_QUEUE

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_RANDOM

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_RXHASH

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_VLAN_TAG

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_VLAN_TAG_PRESENT

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_AD_VLAN_TPID

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_LL_OFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::SKF_NET_OFF

*Constant*: `c_int`



## libc::unix::linux_like::linux::SK_MEMINFO_BACKLOG

*Constant*: `c_int`



## libc::unix::linux_like::linux::SK_MEMINFO_DROPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SK_MEMINFO_FWD_ALLOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::SK_MEMINFO_OPTMEM

*Constant*: `c_int`



## libc::unix::linux_like::linux::SK_MEMINFO_RCVBUF

*Constant*: `c_int`



## libc::unix::linux_like::linux::SK_MEMINFO_RMEM_ALLOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::SK_MEMINFO_SNDBUF

*Constant*: `c_int`



## libc::unix::linux_like::linux::SK_MEMINFO_WMEM_ALLOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::SK_MEMINFO_WMEM_QUEUED

*Constant*: `c_int`



## libc::unix::linux_like::linux::SND_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::SND_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_BIND_PHC

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_OPT_CMSG

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_OPT_ID

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_OPT_ID_TCP

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_OPT_PKTINFO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_OPT_RX_FILTER

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_OPT_STATS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_OPT_TSONLY

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_OPT_TX_SWHW

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_RAW_HARDWARE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_RX_HARDWARE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_RX_SOFTWARE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_SOFTWARE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_SYS_HARDWARE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_TX_ACK

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_TX_HARDWARE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_TX_SCHED

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TIMESTAMPING_TX_SOFTWARE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SOF_TXTIME_DEADLINE_MODE

*Constant*: `u32`



## libc::unix::linux_like::linux::SOF_TXTIME_REPORT_ERRORS

*Constant*: `u32`



## libc::unix::linux_like::linux::SOL_TLS

*Constant*: `c_int`



## libc::unix::linux_like::linux::SOL_XDP

*Constant*: `c_int`



## libc::unix::linux_like::linux::SO_EE_OFFENDER

*Function*

```rust
fn SO_EE_OFFENDER(ee: *const crate::sock_extended_err) -> *mut crate::sockaddr
```



## libc::unix::linux_like::linux::SO_EE_ORIGIN_ICMP

*Constant*: `u8`



## libc::unix::linux_like::linux::SO_EE_ORIGIN_ICMP6

*Constant*: `u8`



## libc::unix::linux_like::linux::SO_EE_ORIGIN_LOCAL

*Constant*: `u8`



## libc::unix::linux_like::linux::SO_EE_ORIGIN_NONE

*Constant*: `u8`



## libc::unix::linux_like::linux::SO_EE_ORIGIN_TIMESTAMPING

*Constant*: `u8`



## libc::unix::linux_like::linux::SO_EE_ORIGIN_TXSTATUS

*Constant*: `u8`



## libc::unix::linux_like::linux::SO_ORIGINAL_DST

*Constant*: `c_int`



## libc::unix::linux_like::linux::SUN_LEN

*Function*

```rust
fn SUN_LEN(s: crate::sockaddr_un) -> usize
```



## libc::unix::linux_like::linux::SW_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::SW_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::SYNC_FILE_RANGE_WAIT_AFTER

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SYNC_FILE_RANGE_WAIT_BEFORE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SYNC_FILE_RANGE_WRITE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::SYN_CNT

*Constant*: `usize`



## libc::unix::linux_like::linux::SYN_MAX

*Constant*: `__u16`



## libc::unix::linux_like::linux::TCA_FCNT

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::TCA_KIND

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::TCA_OPTIONS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::TCA_RATE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::TCA_STAB

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::TCA_STATS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::TCA_STATS2

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::TCA_UNSPEC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::TCA_XSTATS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::TFD_CLOEXEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::TFD_NONBLOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux::TFD_TIMER_ABSTIME

*Constant*: `c_int`



## libc::unix::linux_like::linux::TFD_TIMER_CANCEL_ON_SET

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_1_2_VERSION

*Constant*: `__u16`



## libc::unix::linux_like::linux::TLS_1_2_VERSION_MAJOR

*Constant*: `__u8`



## libc::unix::linux_like::linux::TLS_1_2_VERSION_MINOR

*Constant*: `__u8`



## libc::unix::linux_like::linux::TLS_1_3_VERSION

*Constant*: `__u16`



## libc::unix::linux_like::linux::TLS_1_3_VERSION_MAJOR

*Constant*: `__u8`



## libc::unix::linux_like::linux::TLS_1_3_VERSION_MINOR

*Constant*: `__u8`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_CCM_128

*Constant*: `__u16`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_CCM_128_IV_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_CCM_128_KEY_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_CCM_128_SALT_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_CCM_128_TAG_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_128

*Constant*: `__u16`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_128_IV_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_128_KEY_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_128_SALT_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_128_TAG_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_256

*Constant*: `__u16`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_256_IV_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_256_KEY_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_256_SALT_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_AES_GCM_256_TAG_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_128

*Constant*: `__u16`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_128_IV_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_128_KEY_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_128_SALT_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_128_TAG_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_256

*Constant*: `__u16`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_256_IV_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_256_KEY_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_256_SALT_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_ARIA_GCM_256_TAG_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_CHACHA20_POLY1305

*Constant*: `__u16`



## libc::unix::linux_like::linux::TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_CCM

*Constant*: `__u16`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_CCM_IV_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_CCM_KEY_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_CCM_SALT_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_CCM_TAG_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_GCM

*Constant*: `__u16`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_GCM_IV_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_GCM_KEY_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_GCM_SALT_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CIPHER_SM4_GCM_TAG_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::TLS_CONF_BASE

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_CONF_HW

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_CONF_HW_RECORD

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_CONF_SW

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_GET_RECORD_TYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_INFO_CIPHER

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_INFO_RXCONF

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_INFO_RX_NO_PAD

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_INFO_TXCONF

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_INFO_UNSPEC

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_INFO_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_INFO_ZC_RO_TX

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_RX

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_RX_EXPECT_NO_PAD

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_SET_RECORD_TYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_TX

*Constant*: `c_int`



## libc::unix::linux_like::linux::TLS_TX_ZEROCOPY_RO

*Constant*: `c_int`



## libc::unix::linux_like::linux::TPACKET2_HDRLEN

*Constant*: `usize`



## libc::unix::linux_like::linux::TPACKET3_HDRLEN

*Constant*: `usize`



## libc::unix::linux_like::linux::TPACKET_ALIGN

*Function*

```rust
fn TPACKET_ALIGN(x: usize) -> usize
```



## libc::unix::linux_like::linux::TPACKET_ALIGNMENT

*Constant*: `usize`



## libc::unix::linux_like::linux::TPACKET_HDRLEN

*Constant*: `usize`



## libc::unix::linux_like::linux::TP_FT_REQ_FILL_RXHASH

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_AVAILABLE

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_BLK_TMO

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_COPY

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_CSUMNOTREADY

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_CSUM_VALID

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_KERNEL

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_LOSING

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_SENDING

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_SEND_REQUEST

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_TS_RAW_HARDWARE

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_TS_SOFTWARE

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_TS_SYS_HARDWARE

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_USER

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_VLAN_TPID_VALID

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_VLAN_VALID

*Constant*: `__u32`



## libc::unix::linux_like::linux::TP_STATUS_WRONG_FORMAT

*Constant*: `__u32`



## libc::unix::linux_like::linux::TRAP_PERF

*Constant*: `c_int`



## libc::unix::linux_like::linux::UINPUT_MAX_NAME_SIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::UINPUT_VERSION

*Constant*: `c_uint`



## libc::unix::linux_like::linux::VMADDR_CID_ANY

*Constant*: `c_uint`



## libc::unix::linux_like::linux::VMADDR_CID_HOST

*Constant*: `c_uint`



## libc::unix::linux_like::linux::VMADDR_CID_HYPERVISOR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::VMADDR_CID_LOCAL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::VMADDR_CID_RESERVED

*Constant*: `c_uint`



## libc::unix::linux_like::linux::VMADDR_PORT_ANY

*Constant*: `c_uint`



## libc::unix::linux_like::linux::VM_BLOCK_DUMP

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_DIRTY_BACKGROUND

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_DIRTY_EXPIRE_CS

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_DIRTY_RATIO

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_DIRTY_WB_CS

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_DROP_PAGECACHE

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_HUGETLB_GROUP

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_HUGETLB_PAGES

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_LAPTOP_MODE

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_LEGACY_VA_LAYOUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_LOWMEM_RESERVE_RATIO

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_MAX_MAP_COUNT

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_MIN_FREE_KBYTES

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_MIN_SLAB

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_MIN_UNMAPPED

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_NR_PDFLUSH_THREADS

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_OVERCOMMIT_MEMORY

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_OVERCOMMIT_RATIO

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_PAGEBUF

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_PAGE_CLUSTER

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_PANIC_ON_OOM

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_PERCPU_PAGELIST_FRACTION

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_SWAPPINESS

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_SWAP_TOKEN_TIMEOUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_VDSO_ENABLED

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_VFS_CACHE_PRESSURE

*Constant*: `c_int`



## libc::unix::linux_like::linux::VM_ZONE_RECLAIM_MODE

*Constant*: `c_int`



## libc::unix::linux_like::linux::WIRELESS_EXT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::XDP_COPY

*Constant*: `crate::__u16`



## libc::unix::linux_like::linux::XDP_MMAP_OFFSETS

*Constant*: `c_int`



## libc::unix::linux_like::linux::XDP_OPTIONS

*Constant*: `c_int`



## libc::unix::linux_like::linux::XDP_OPTIONS_ZEROCOPY

*Constant*: `crate::__u32`



## libc::unix::linux_like::linux::XDP_PGOFF_RX_RING

*Constant*: `crate::off_t`



## libc::unix::linux_like::linux::XDP_PGOFF_TX_RING

*Constant*: `crate::off_t`



## libc::unix::linux_like::linux::XDP_PKT_CONTD

*Constant*: `crate::__u32`



## libc::unix::linux_like::linux::XDP_RING_NEED_WAKEUP

*Constant*: `crate::__u32`



## libc::unix::linux_like::linux::XDP_RX_RING

*Constant*: `c_int`



## libc::unix::linux_like::linux::XDP_SHARED_UMEM

*Constant*: `crate::__u16`



## libc::unix::linux_like::linux::XDP_STATISTICS

*Constant*: `c_int`



## libc::unix::linux_like::linux::XDP_TXMD_FLAGS_CHECKSUM

*Constant*: `crate::__u32`



## libc::unix::linux_like::linux::XDP_TXMD_FLAGS_TIMESTAMP

*Constant*: `crate::__u32`



## libc::unix::linux_like::linux::XDP_TX_METADATA

*Constant*: `crate::__u32`



## libc::unix::linux_like::linux::XDP_TX_RING

*Constant*: `c_int`



## libc::unix::linux_like::linux::XDP_UMEM_COMPLETION_RING

*Constant*: `c_int`



## libc::unix::linux_like::linux::XDP_UMEM_FILL_RING

*Constant*: `c_int`



## libc::unix::linux_like::linux::XDP_UMEM_PGOFF_COMPLETION_RING

*Constant*: `crate::c_ulonglong`



## libc::unix::linux_like::linux::XDP_UMEM_PGOFF_FILL_RING

*Constant*: `crate::c_ulonglong`



## libc::unix::linux_like::linux::XDP_UMEM_REG

*Constant*: `c_int`



## libc::unix::linux_like::linux::XDP_UMEM_TX_METADATA_LEN

*Constant*: `crate::__u32`



## libc::unix::linux_like::linux::XDP_UMEM_TX_SW_CSUM

*Constant*: `crate::__u32`



## libc::unix::linux_like::linux::XDP_UMEM_UNALIGNED_CHUNK_FLAG

*Constant*: `crate::__u32`



## libc::unix::linux_like::linux::XDP_USE_NEED_WAKEUP

*Constant*: `crate::__u16`



## libc::unix::linux_like::linux::XDP_USE_SG

*Constant*: `crate::__u16`



## libc::unix::linux_like::linux::XDP_ZEROCOPY

*Constant*: `crate::__u16`



## libc::unix::linux_like::linux::XSK_UNALIGNED_BUF_ADDR_MASK

*Constant*: `crate::c_ulonglong`



## libc::unix::linux_like::linux::XSK_UNALIGNED_BUF_OFFSET_SHIFT

*Constant*: `crate::c_int`



## libc::unix::linux_like::linux::__NFT_REG_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::__c_anonymous__kernel_fsid_t

*Struct*

**Fields:**
- `val: [c_int; 2]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous__kernel_fsid_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::__c_anonymous_iwreq

*Union*

**Fields:**
- `ifrn_name: [c_char; 16]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_iwreq`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::unix::linux_like::linux::__c_anonymous_ptp_perout_request_1

*Union*

**Fields:**
- `start: ptp_clock_time`
- `phase: ptp_clock_time`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_ptp_perout_request_1`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::unix::linux_like::linux::__c_anonymous_ptp_perout_request_2

*Union*

**Fields:**
- `on: ptp_clock_time`
- `rsv: [c_uint; 4]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_ptp_perout_request_2`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::unix::linux_like::linux::__c_anonymous_xsk_tx_metadata_union

*Union*

**Fields:**
- `request: xsk_tx_metadata_request`
- `completion: xsk_tx_metadata_completion`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_xsk_tx_metadata_union`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::unix::linux_like::linux::__kernel_clockid_t

*Type Alias*: `c_int`



## libc::unix::linux_like::linux::__kernel_fsid_t

*Type Alias*: `__c_anonymous__kernel_fsid_t`



## libc::unix::linux_like::linux::__s16

*Type Alias*: `c_short`



## libc::unix::linux_like::linux::__s32

*Type Alias*: `c_int`



## libc::unix::linux_like::linux::__u16

*Type Alias*: `c_ushort`



## libc::unix::linux_like::linux::__u32

*Type Alias*: `c_uint`



## libc::unix::linux_like::linux::__u8

*Type Alias*: `c_uchar`



## libc::unix::linux_like::linux::accept4

*Function*

```rust
fn accept4(fd: c_int, addr: *mut crate::sockaddr, len: *mut socklen_t, flg: c_int) -> c_int
```



## libc::unix::linux_like::linux::af_alg_iv

*Struct*

WARNING: The `PartialEq`, `Eq` and `Hash` implementations of this
type are unsound and will be removed in the future.

**Fields:**
- `ivlen: u32`
- `iv: [c_uchar; 0]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> af_alg_iv`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::blkcnt64_t

*Type Alias*: `i64`



## libc::unix::linux_like::linux::clock_nanosleep

*Function*

```rust
fn clock_nanosleep(clk_id: crate::clockid_t, flags: c_int, rqtp: *const crate::timespec, rmtp: *mut crate::timespec) -> c_int
```



## libc::unix::linux_like::linux::clone

*Function*

```rust
fn clone(cb: fn(...), child_stack: *mut c_void, flags: c_int, arg: *mut c_void) -> c_int
```



## libc::unix::linux_like::linux::dev_t

*Type Alias*: `u64`



## libc::unix::linux_like::linux::dmabuf_cmsg

*Struct*

**Fields:**
- `frag_offset: crate::__u64`
- `frag_size: crate::__u32`
- `frag_token: crate::__u32`
- `dmabuf_id: crate::__u32`
- `flags: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> dmabuf_cmsg`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::dmabuf_token

*Struct*

**Fields:**
- `token_start: crate::__u32`
- `token_count: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> dmabuf_token`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::dqblk

*Struct*

**Fields:**
- `dqb_bhardlimit: u64`
- `dqb_bsoftlimit: u64`
- `dqb_curspace: u64`
- `dqb_ihardlimit: u64`
- `dqb_isoftlimit: u64`
- `dqb_curinodes: u64`
- `dqb_btime: u64`
- `dqb_itime: u64`
- `dqb_valid: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> dqblk`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::dup3

*Function*

```rust
fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::epoll_create

*Function*

```rust
fn epoll_create(size: c_int) -> c_int
```



## libc::unix::linux_like::linux::epoll_create1

*Function*

```rust
fn epoll_create1(flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::epoll_ctl

*Function*

```rust
fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut crate::epoll_event) -> c_int
```



## libc::unix::linux_like::linux::epoll_params

*Struct*

**Fields:**
- `busy_poll_usecs: u32`
- `busy_poll_budget: u16`
- `prefer_busy_poll: u8`
- `__pad: u8`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> epoll_params`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::epoll_pwait

*Function*

```rust
fn epoll_pwait(epfd: c_int, events: *mut crate::epoll_event, maxevents: c_int, timeout: c_int, sigmask: *const crate::sigset_t) -> c_int
```



## libc::unix::linux_like::linux::epoll_wait

*Function*

```rust
fn epoll_wait(epfd: c_int, events: *mut crate::epoll_event, maxevents: c_int, timeout: c_int) -> c_int
```



## libc::unix::linux_like::linux::eventfd

*Function*

```rust
fn eventfd(initval: c_uint, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::eventfd_read

*Function*

```rust
fn eventfd_read(fd: c_int, value: *mut eventfd_t) -> c_int
```



## libc::unix::linux_like::linux::eventfd_t

*Type Alias*: `u64`



## libc::unix::linux_like::linux::eventfd_write

*Function*

```rust
fn eventfd_write(fd: c_int, value: eventfd_t) -> c_int
```



## libc::unix::linux_like::linux::fallocate

*Function*

```rust
fn fallocate(fd: c_int, mode: c_int, offset: off_t, len: off_t) -> c_int
```



## libc::unix::linux_like::linux::fallocate64

*Function*

```rust
fn fallocate64(fd: c_int, mode: c_int, offset: off64_t, len: off64_t) -> c_int
```



## libc::unix::linux_like::linux::fanotify_event_info_fid

*Struct*

**Fields:**
- `hdr: fanotify_event_info_header`
- `fsid: __kernel_fsid_t`
- `handle: [c_uchar; 0]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fanotify_event_info_fid`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::fanotify_event_info_header

*Struct*

**Fields:**
- `info_type: __u8`
- `pad: __u8`
- `len: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fanotify_event_info_header`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::fanotify_event_metadata

*Struct*

**Fields:**
- `event_len: __u32`
- `vers: __u8`
- `reserved: __u8`
- `metadata_len: __u16`
- `mask: __u64`
- `fd: c_int`
- `pid: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fanotify_event_metadata`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::fanotify_init

*Function*

```rust
fn fanotify_init(flags: c_uint, event_f_flags: c_uint) -> c_int
```



## libc::unix::linux_like::linux::fanotify_response

*Struct*

**Fields:**
- `fd: c_int`
- `response: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fanotify_response`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::fanout_args

*Struct*

**Fields:**
- `id: __u16`
- `type_flags: __u16`
- `max_num_members: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fanout_args`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ff_condition_effect

*Struct*

**Fields:**
- `right_saturation: __u16`
- `left_saturation: __u16`
- `right_coeff: __s16`
- `left_coeff: __s16`
- `deadband: __u16`
- `center: __s16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ff_condition_effect`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ff_constant_effect

*Struct*

**Fields:**
- `level: __s16`
- `envelope: ff_envelope`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ff_constant_effect`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ff_effect

*Struct*

**Fields:**
- `type_: __u16`
- `id: __s16`
- `direction: __u16`
- `trigger: ff_trigger`
- `replay: ff_replay`
- `u: [u64; 4]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ff_effect`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ff_envelope

*Struct*

**Fields:**
- `attack_length: __u16`
- `attack_level: __u16`
- `fade_length: __u16`
- `fade_level: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ff_envelope`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ff_periodic_effect

*Struct*

**Fields:**
- `waveform: __u16`
- `period: __u16`
- `magnitude: __s16`
- `offset: __s16`
- `phase: __u16`
- `envelope: ff_envelope`
- `custom_len: __u32`
- `custom_data: *mut __s16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ff_periodic_effect`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ff_ramp_effect

*Struct*

**Fields:**
- `start_level: __s16`
- `end_level: __s16`
- `envelope: ff_envelope`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ff_ramp_effect`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ff_replay

*Struct*

**Fields:**
- `length: __u16`
- `delay: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ff_replay`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ff_rumble_effect

*Struct*

**Fields:**
- `strong_magnitude: __u16`
- `weak_magnitude: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ff_rumble_effect`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ff_trigger

*Struct*

**Fields:**
- `button: __u16`
- `interval: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ff_trigger`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::fgetpos64

*Function*

```rust
fn fgetpos64(stream: *mut crate::FILE, ptr: *mut crate::fpos64_t) -> c_int
```



## libc::unix::linux_like::linux::fgetxattr

*Function*

```rust
fn fgetxattr(filedes: c_int, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t
```



## libc::unix::linux_like::linux::file_handle

*Struct*

**Fields:**
- `handle_bytes: c_uint`
- `handle_type: c_int`
- `f_handle: [c_uchar; 0]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> file_handle`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::flistxattr

*Function*

```rust
fn flistxattr(filedes: c_int, list: *mut c_char, size: size_t) -> ssize_t
```



## libc::unix::linux_like::linux::fopen64

*Function*

```rust
fn fopen64(filename: *const c_char, mode: *const c_char) -> *mut crate::FILE
```



## libc::unix::linux_like::linux::fread_unlocked

*Function*

```rust
fn fread_unlocked(buf: *mut c_void, size: size_t, nobj: size_t, stream: *mut crate::FILE) -> size_t
```



## libc::unix::linux_like::linux::fremovexattr

*Function*

```rust
fn fremovexattr(filedes: c_int, name: *const c_char) -> c_int
```



## libc::unix::linux_like::linux::fsetxattr

*Function*

```rust
fn fsetxattr(filedes: c_int, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::ftok

*Function*

```rust
fn ftok(pathname: *const c_char, proj_id: c_int) -> crate::key_t
```



## libc::unix::linux_like::linux::genlmsghdr

*Struct*

**Fields:**
- `cmd: u8`
- `version: u8`
- `reserved: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> genlmsghdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::getdtablesize

*Function*

```rust
fn getdtablesize() -> c_int
```



## libc::unix::linux_like::linux::getgrouplist

*Function*

```rust
fn getgrouplist(user: *const c_char, group: crate::gid_t, groups: *mut crate::gid_t, ngroups: *mut c_int) -> c_int
```



## libc::unix::linux_like::linux::gethostid

*Function*

```rust
fn gethostid() -> c_long
```



## libc::unix::linux_like::linux::getspnam_r

*Function*

```rust
fn getspnam_r(name: *const c_char, spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```



## libc::unix::linux_like::linux::getxattr

*Function*

```rust
fn getxattr(path: *const c_char, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t
```



## libc::unix::linux_like::linux::hwtstamp_config

*Struct*

**Fields:**
- `flags: c_int`
- `tx_type: c_int`
- `rx_filter: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> hwtstamp_config`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::idtype_t

*Type Alias*: `c_uint`



## libc::unix::linux_like::linux::in6_ifreq

*Struct*

**Fields:**
- `ifr6_addr: crate::in6_addr`
- `ifr6_prefixlen: u32`
- `ifr6_ifindex: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> in6_ifreq`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ino64_t

*Type Alias*: `u64`



## libc::unix::linux_like::linux::inotify_add_watch

*Function*

```rust
fn inotify_add_watch(fd: c_int, path: *const c_char, mask: u32) -> c_int
```



## libc::unix::linux_like::linux::inotify_event

*Struct*

**Fields:**
- `wd: c_int`
- `mask: u32`
- `cookie: u32`
- `len: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> inotify_event`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::inotify_init

*Function*

```rust
fn inotify_init() -> c_int
```



## libc::unix::linux_like::linux::inotify_init1

*Function*

```rust
fn inotify_init1(flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::inotify_rm_watch

*Function*

```rust
fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int
```



## libc::unix::linux_like::linux::input_absinfo

*Struct*

**Fields:**
- `value: __s32`
- `minimum: __s32`
- `maximum: __s32`
- `fuzz: __s32`
- `flat: __s32`
- `resolution: __s32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> input_absinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::input_event

*Struct*

**Fields:**
- `time: crate::timeval`
- `type_: __u16`
- `code: __u16`
- `value: __s32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> input_event`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::input_id

*Struct*

**Fields:**
- `bustype: __u16`
- `vendor: __u16`
- `product: __u16`
- `version: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> input_id`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::input_keymap_entry

*Struct*

**Fields:**
- `flags: __u8`
- `len: __u8`
- `index: __u16`
- `keycode: __u32`
- `scancode: [__u8; 32]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> input_keymap_entry`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::input_mask

*Struct*

**Fields:**
- `type_: __u32`
- `codes_size: __u32`
- `codes_ptr: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> input_mask`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_discarded

*Struct*

**Fields:**
- `nwid: __u32`
- `code: __u32`
- `fragment: __u32`
- `retries: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_discarded`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_encode_ext

*Struct*

**Fields:**
- `ext_flags: __u32`
- `tx_seq: [__u8; 8]`
- `rx_seq: [__u8; 8]`
- `addr: crate::sockaddr`
- `alg: __u16`
- `key_len: __u16`
- `key: [__u8; 0]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_encode_ext`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_event

*Struct*

**Fields:**
- `len: __u16`
- `cmd: __u16`
- `u: iwreq_data`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_event`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_freq

*Struct*

**Fields:**
- `m: __s32`
- `e: __s16`
- `i: __u8`
- `flags: __u8`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_freq`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_michaelmicfailure

*Struct*

**Fields:**
- `flags: __u32`
- `src_addr: crate::sockaddr`
- `tsc: [__u8; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_michaelmicfailure`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_missed

*Struct*

**Fields:**
- `beacon: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_missed`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_mlme

*Struct*

**Fields:**
- `cmd: __u16`
- `reason_code: __u16`
- `addr: crate::sockaddr`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_mlme`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_param

*Struct*

**Fields:**
- `value: __s32`
- `fixed: __u8`
- `disabled: __u8`
- `flags: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_param`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_pmkid_cand

*Struct*

**Fields:**
- `flags: __u32`
- `index: __u32`
- `bssid: crate::sockaddr`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_pmkid_cand`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_pmksa

*Struct*

**Fields:**
- `cmd: __u32`
- `bssid: crate::sockaddr`
- `pmkid: [__u8; 16]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_pmksa`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_point

*Struct*

**Fields:**
- `pointer: *mut c_void`
- `length: __u16`
- `flags: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_point`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_priv_args

*Struct*

**Fields:**
- `cmd: __u32`
- `set_args: __u16`
- `get_args: __u16`
- `name: [c_char; 16]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_priv_args`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_quality

*Struct*

**Fields:**
- `qual: __u8`
- `level: __u8`
- `noise: __u8`
- `updated: __u8`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_quality`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_range

*Struct*

**Fields:**
- `throughput: __u32`
- `min_nwid: __u32`
- `max_nwid: __u32`
- `old_num_channels: __u16`
- `old_num_frequency: __u8`
- `scan_capa: __u8`
- `event_capa: [__u32; 6]`
- `sensitivity: __s32`
- `max_qual: iw_quality`
- `avg_qual: iw_quality`
- `num_bitrates: __u8`
- `bitrate: [__s32; 32]`
- `min_rts: __s32`
- `max_rts: __s32`
- `min_frag: __s32`
- `max_frag: __s32`
- `min_pmp: __s32`
- `max_pmp: __s32`
- `min_pmt: __s32`
- `max_pmt: __s32`
- `pmp_flags: __u16`
- `pmt_flags: __u16`
- `pm_capa: __u16`
- `encoding_size: [__u16; 8]`
- `num_encoding_sizes: __u8`
- `max_encoding_tokens: __u8`
- `encoding_login_index: __u8`
- `txpower_capa: __u16`
- `num_txpower: __u8`
- `txpower: [__s32; 8]`
- `we_version_compiled: __u8`
- `we_version_source: __u8`
- `retry_capa: __u16`
- `retry_flags: __u16`
- `r_time_flags: __u16`
- `min_retry: __s32`
- `max_retry: __s32`
- `min_r_time: __s32`
- `max_r_time: __s32`
- `num_channels: __u16`
- `num_frequency: __u8`
- `freq: [iw_freq; 32]`
- `enc_capa: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_range`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_scan_req

*Struct*

**Fields:**
- `scan_type: __u8`
- `essid_len: __u8`
- `num_channels: __u8`
- `flags: __u8`
- `bssid: crate::sockaddr`
- `essid: [__u8; 32]`
- `min_channel_time: __u32`
- `max_channel_time: __u32`
- `channel_list: [iw_freq; 32]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_scan_req`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_statistics

*Struct*

**Fields:**
- `status: __u16`
- `qual: iw_quality`
- `discard: iw_discarded`
- `miss: iw_missed`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_statistics`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iw_thrspy

*Struct*

**Fields:**
- `addr: crate::sockaddr`
- `qual: iw_quality`
- `low: iw_quality`
- `high: iw_quality`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iw_thrspy`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iwreq

*Struct*

**Fields:**
- `ifr_ifrn: __c_anonymous_iwreq`
- `u: iwreq_data`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iwreq`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::iwreq_data

*Union*

**Fields:**
- `name: [c_char; 16]`
- `essid: iw_point`
- `nwid: iw_param`
- `freq: iw_freq`
- `sens: iw_param`
- `bitrate: iw_param`
- `txpower: iw_param`
- `rts: iw_param`
- `frag: iw_param`
- `mode: __u32`
- `retry: iw_param`
- `encoding: iw_point`
- `power: iw_param`
- `qual: iw_quality`
- `ap_addr: crate::sockaddr`
- `addr: crate::sockaddr`
- `param: iw_param`
- `data: iw_point`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iwreq_data`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::unix::linux_like::linux::klogctl

*Function*

```rust
fn klogctl(syslog_type: c_int, bufp: *mut c_char, len: c_int) -> c_int
```



## libc::unix::linux_like::linux::lcong48

*Function*

```rust
fn lcong48(p: *mut c_ushort)
```



## libc::unix::linux_like::linux::lgetxattr

*Function*

```rust
fn lgetxattr(path: *const c_char, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t
```



## libc::unix::linux_like::linux::listxattr

*Function*

```rust
fn listxattr(path: *const c_char, list: *mut c_char, size: size_t) -> ssize_t
```



## libc::unix::linux_like::linux::llistxattr

*Function*

```rust
fn llistxattr(path: *const c_char, list: *mut c_char, size: size_t) -> ssize_t
```



## libc::unix::linux_like::linux::loff_t

*Type Alias*: `c_longlong`



## libc::unix::linux_like::linux::lremovexattr

*Function*

```rust
fn lremovexattr(path: *const c_char, name: *const c_char) -> c_int
```



## libc::unix::linux_like::linux::lsetxattr

*Function*

```rust
fn lsetxattr(path: *const c_char, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::lutimes

*Function*

```rust
fn lutimes(file: *const c_char, times: *const crate::timeval) -> c_int
```



## libc::unix::linux_like::linux::mkfifoat

*Function*

```rust
fn mkfifoat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int
```



## libc::unix::linux_like::linux::mkstemps

*Function*

```rust
fn mkstemps(template: *mut c_char, suffixlen: c_int) -> c_int
```



## libc::unix::linux_like::linux::mnt_ns_info

*Struct*

**Fields:**
- `size: crate::__u32`
- `nr_mounts: crate::__u32`
- `mnt_ns_id: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> mnt_ns_info`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::mode_t

*Type Alias*: `u32`



## libc::unix::linux_like::linux::mount_attr

*Struct*

**Fields:**
- `attr_set: crate::__u64`
- `attr_clr: crate::__u64`
- `propagation: crate::__u64`
- `userns_fd: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> mount_attr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::mq_attr

*Struct*

**Fields:**
- `mq_flags: c_long`
- `mq_maxmsg: c_long`
- `mq_msgsize: c_long`
- `mq_curmsgs: c_long`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> mq_attr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::mq_close

*Function*

```rust
fn mq_close(mqd: mqd_t) -> c_int
```



## libc::unix::linux_like::linux::mq_getattr

*Function*

```rust
fn mq_getattr(mqd: mqd_t, attr: *mut crate::mq_attr) -> c_int
```



## libc::unix::linux_like::linux::mq_open

*Function*

```rust
fn mq_open(name: *const c_char, oflag: c_int) -> mqd_t
```



## libc::unix::linux_like::linux::mq_receive

*Function*

```rust
fn mq_receive(mqd: mqd_t, msg_ptr: *mut c_char, msg_len: size_t, msg_prio: *mut c_uint) -> ssize_t
```



## libc::unix::linux_like::linux::mq_send

*Function*

```rust
fn mq_send(mqd: mqd_t, msg_ptr: *const c_char, msg_len: size_t, msg_prio: c_uint) -> c_int
```



## libc::unix::linux_like::linux::mq_setattr

*Function*

```rust
fn mq_setattr(mqd: mqd_t, newattr: *const crate::mq_attr, oldattr: *mut crate::mq_attr) -> c_int
```



## libc::unix::linux_like::linux::mq_timedreceive

*Function*

```rust
fn mq_timedreceive(mqd: mqd_t, msg_ptr: *mut c_char, msg_len: size_t, msg_prio: *mut c_uint, abs_timeout: *const crate::timespec) -> ssize_t
```



## libc::unix::linux_like::linux::mq_timedsend

*Function*

```rust
fn mq_timedsend(mqd: mqd_t, msg_ptr: *const c_char, msg_len: size_t, msg_prio: c_uint, abs_timeout: *const crate::timespec) -> c_int
```



## libc::unix::linux_like::linux::mq_unlink

*Function*

```rust
fn mq_unlink(name: *const c_char) -> c_int
```



## libc::unix::linux_like::linux::mqd_t

*Type Alias*: `c_int`



## libc::unix::linux_like::linux::mrand48

*Function*

```rust
fn mrand48() -> c_long
```



## libc::unix::linux_like::linux::msgctl

*Function*

```rust
fn msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds) -> c_int
```



## libc::unix::linux_like::linux::msgget

*Function*

```rust
fn msgget(key: crate::key_t, msgflg: c_int) -> c_int
```



## libc::unix::linux_like::linux::msginfo

*Struct*

**Fields:**
- `msgpool: c_int`
- `msgmap: c_int`
- `msgmax: c_int`
- `msgmnb: c_int`
- `msgmni: c_int`
- `msgssz: c_int`
- `msgtql: c_int`
- `msgseg: c_ushort`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> msginfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::msgrcv

*Function*

```rust
fn msgrcv(msqid: c_int, msgp: *mut c_void, msgsz: size_t, msgtyp: c_long, msgflg: c_int) -> ssize_t
```



## libc::unix::linux_like::linux::msgsnd

*Function*

```rust
fn msgsnd(msqid: c_int, msgp: *const c_void, msgsz: size_t, msgflg: c_int) -> c_int
```



## libc::unix::linux_like::linux::name_to_handle_at

*Function*

```rust
fn name_to_handle_at(dirfd: c_int, path: *const c_char, handle: *mut file_handle, mount_id: *mut c_int, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::nfds_t

*Type Alias*: `c_ulong`



## libc::unix::linux_like::linux::nl_item

*Type Alias*: `c_int`



## libc::unix::linux_like::linux::off64_t

*Type Alias*: `i64`



## libc::unix::linux_like::linux::open_by_handle_at

*Function*

```rust
fn open_by_handle_at(mount_fd: c_int, handle: *mut file_handle, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::open_how

*Struct*

**Fields:**
- `flags: crate::__u64`
- `mode: crate::__u64`
- `resolve: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> open_how`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::personality

*Function*

```rust
fn personality(persona: c_ulong) -> c_int
```



## libc::unix::linux_like::linux::pid_type

*Type Alias*: `c_uint`



## libc::unix::linux_like::linux::posix_fallocate

*Function*

```rust
fn posix_fallocate(fd: c_int, offset: off_t, len: off_t) -> c_int
```



## libc::unix::linux_like::linux::posix_fallocate64

*Function*

```rust
fn posix_fallocate64(fd: c_int, offset: off64_t, len: off64_t) -> c_int
```



## libc::unix::linux_like::linux::posix_madvise

*Function*

```rust
fn posix_madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int
```



## libc::unix::linux_like::linux::posix_spawn

*Function*

```rust
fn posix_spawn(pid: *mut crate::pid_t, path: *const c_char, file_actions: *const crate::posix_spawn_file_actions_t, attrp: *const crate::posix_spawnattr_t, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int
```



## libc::unix::linux_like::linux::posix_spawn_file_actions_addclose

*Function*

```rust
fn posix_spawn_file_actions_addclose(actions: *mut posix_spawn_file_actions_t, fd: c_int) -> c_int
```



## libc::unix::linux_like::linux::posix_spawn_file_actions_adddup2

*Function*

```rust
fn posix_spawn_file_actions_adddup2(actions: *mut posix_spawn_file_actions_t, fd: c_int, newfd: c_int) -> c_int
```



## libc::unix::linux_like::linux::posix_spawn_file_actions_addopen

*Function*

```rust
fn posix_spawn_file_actions_addopen(actions: *mut posix_spawn_file_actions_t, fd: c_int, path: *const c_char, oflag: c_int, mode: mode_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawn_file_actions_destroy

*Function*

```rust
fn posix_spawn_file_actions_destroy(actions: *mut posix_spawn_file_actions_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawn_file_actions_init

*Function*

```rust
fn posix_spawn_file_actions_init(actions: *mut posix_spawn_file_actions_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawn_file_actions_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> posix_spawn_file_actions_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::posix_spawnattr_destroy

*Function*

```rust
fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_getflags

*Function*

```rust
fn posix_spawnattr_getflags(attr: *const posix_spawnattr_t, flags: *mut c_short) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_getpgroup

*Function*

```rust
fn posix_spawnattr_getpgroup(attr: *const posix_spawnattr_t, flags: *mut crate::pid_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_getschedparam

*Function*

```rust
fn posix_spawnattr_getschedparam(attr: *const posix_spawnattr_t, param: *mut crate::sched_param) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_getschedpolicy

*Function*

```rust
fn posix_spawnattr_getschedpolicy(attr: *const posix_spawnattr_t, flags: *mut c_int) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_getsigdefault

*Function*

```rust
fn posix_spawnattr_getsigdefault(attr: *const posix_spawnattr_t, default: *mut crate::sigset_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_getsigmask

*Function*

```rust
fn posix_spawnattr_getsigmask(attr: *const posix_spawnattr_t, default: *mut crate::sigset_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_init

*Function*

```rust
fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_setflags

*Function*

```rust
fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t, flags: c_short) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_setpgroup

*Function*

```rust
fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t, flags: crate::pid_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_setschedparam

*Function*

```rust
fn posix_spawnattr_setschedparam(attr: *mut posix_spawnattr_t, param: *const crate::sched_param) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_setschedpolicy

*Function*

```rust
fn posix_spawnattr_setschedpolicy(attr: *mut posix_spawnattr_t, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_setsigdefault

*Function*

```rust
fn posix_spawnattr_setsigdefault(attr: *mut posix_spawnattr_t, default: *const crate::sigset_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_setsigmask

*Function*

```rust
fn posix_spawnattr_setsigmask(attr: *mut posix_spawnattr_t, default: *const crate::sigset_t) -> c_int
```



## libc::unix::linux_like::linux::posix_spawnattr_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> posix_spawnattr_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::posix_spawnp

*Function*

```rust
fn posix_spawnp(pid: *mut crate::pid_t, file: *const c_char, file_actions: *const crate::posix_spawn_file_actions_t, attrp: *const crate::posix_spawnattr_t, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int
```



## libc::unix::linux_like::linux::proc_cn_event

*Type Alias*: `c_uint`



## libc::unix::linux_like::linux::proc_cn_mcast_op

*Type Alias*: `c_uint`



## libc::unix::linux_like::linux::pthread_barrier_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> pthread_barrier_t`



## libc::unix::linux_like::linux::pthread_barrierattr_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> pthread_barrierattr_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::pthread_cond_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> pthread_cond_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::pthread_condattr_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> pthread_condattr_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::pthread_key_t

*Type Alias*: `c_uint`



## libc::unix::linux_like::linux::pthread_mutex_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> pthread_mutex_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::pthread_mutexattr_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> pthread_mutexattr_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::pthread_once_t

*Type Alias*: `c_int`



## libc::unix::linux_like::linux::pthread_rwlock_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> pthread_rwlock_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::pthread_rwlockattr_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> pthread_rwlockattr_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::pthread_spinlock_t

*Type Alias*: `c_int`



## libc::unix::linux_like::linux::ptp_clock_caps

*Struct*

**Fields:**
- `max_adj: c_int`
- `n_alarm: c_int`
- `n_ext_ts: c_int`
- `n_per_out: c_int`
- `pps: c_int`
- `n_pins: c_int`
- `cross_timestamping: c_int`
- `adjust_phase: c_int`
- `max_phase_adj: c_int`
- `rsv: [c_int; 11]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptp_clock_caps`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ptp_clock_time

*Struct*

**Fields:**
- `sec: crate::__s64`
- `nsec: __u32`
- `reserved: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptp_clock_time`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ptp_extts_event

*Struct*

**Fields:**
- `t: ptp_clock_time`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ptp_extts_event`



## libc::unix::linux_like::linux::ptp_extts_request

*Struct*

**Fields:**
- `index: c_uint`
- `flags: c_uint`
- `rsv: [c_uint; 2]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptp_extts_request`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ptp_perout_request

*Struct*

**Fields:**
- `anonymous_1: __c_anonymous_ptp_perout_request_1`
- `period: ptp_clock_time`
- `index: c_uint`
- `flags: c_uint`
- `anonymous_2: __c_anonymous_ptp_perout_request_2`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptp_perout_request`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ptp_pin_desc

*Struct*

**Fields:**
- `name: [c_char; 64]`
- `index: c_uint`
- `func: c_uint`
- `chan: c_uint`
- `rsv: [c_uint; 5]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptp_pin_desc`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ptp_sys_offset

*Struct*

**Fields:**
- `n_samples: c_uint`
- `rsv: [c_uint; 3]`
- `ts: [ptp_clock_time; 51]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptp_sys_offset`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ptp_sys_offset_extended

*Struct*

**Fields:**
- `n_samples: c_uint`
- `clockid: __kernel_clockid_t`
- `rsv: [c_uint; 2]`
- `ts: [[ptp_clock_time; 3]; 25]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptp_sys_offset_extended`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::ptp_sys_offset_precise

*Struct*

**Fields:**
- `device: ptp_clock_time`
- `sys_realtime: ptp_clock_time`
- `sys_monoraw: ptp_clock_time`
- `rsv: [c_uint; 4]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptp_sys_offset_precise`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::quotactl

*Function*

```rust
fn quotactl(cmd: c_int, special: *const c_char, id: c_int, data: *mut c_char) -> c_int
```



## libc::unix::linux_like::linux::readahead

*Function*

```rust
fn readahead(fd: c_int, offset: off64_t, count: size_t) -> ssize_t
```



## libc::unix::linux_like::linux::reboot

*Function*

```rust
fn reboot(how_to: c_int) -> c_int
```



## libc::unix::linux_like::linux::remap_file_pages

*Function*

```rust
fn remap_file_pages(addr: *mut c_void, size: size_t, prot: c_int, pgoff: size_t, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::removexattr

*Function*

```rust
fn removexattr(path: *const c_char, name: *const c_char) -> c_int
```



## libc::unix::linux_like::linux::rlim64_t

*Type Alias*: `u64`



## libc::unix::linux_like::linux::sched_attr

*Struct*

**Fields:**
- `size: __u32`
- `sched_policy: __u32`
- `sched_flags: crate::__u64`
- `sched_nice: __s32`
- `sched_priority: __u32`
- `sched_runtime: crate::__u64`
- `sched_deadline: crate::__u64`
- `sched_period: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sched_attr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sched_getparam

*Function*

```rust
fn sched_getparam(pid: crate::pid_t, param: *mut crate::sched_param) -> c_int
```



## libc::unix::linux_like::linux::sched_getscheduler

*Function*

```rust
fn sched_getscheduler(pid: crate::pid_t) -> c_int
```



## libc::unix::linux_like::linux::sched_rr_get_interval

*Function*

```rust
fn sched_rr_get_interval(pid: crate::pid_t, tp: *mut crate::timespec) -> c_int
```



## libc::unix::linux_like::linux::sched_setaffinity

*Function*

```rust
fn sched_setaffinity(pid: crate::pid_t, cpusetsize: size_t, cpuset: *const crate::cpu_set_t) -> c_int
```



## libc::unix::linux_like::linux::sched_setparam

*Function*

```rust
fn sched_setparam(pid: crate::pid_t, param: *const crate::sched_param) -> c_int
```



## libc::unix::linux_like::linux::sched_setscheduler

*Function*

```rust
fn sched_setscheduler(pid: crate::pid_t, policy: c_int, param: *const crate::sched_param) -> c_int
```



## libc::unix::linux_like::linux::sctp_assoc_t

*Type Alias*: `__s32`



## libc::unix::linux_like::linux::sctp_authinfo

*Struct*

**Fields:**
- `auth_keynumber: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sctp_authinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sctp_initmsg

*Struct*

**Fields:**
- `sinit_num_ostreams: __u16`
- `sinit_max_instreams: __u16`
- `sinit_max_attempts: __u16`
- `sinit_max_init_timeo: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sctp_initmsg`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sctp_nxtinfo

*Struct*

**Fields:**
- `nxt_sid: __u16`
- `nxt_flags: __u16`
- `nxt_ppid: __u32`
- `nxt_length: __u32`
- `nxt_assoc_id: crate::sctp_assoc_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sctp_nxtinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sctp_prinfo

*Struct*

**Fields:**
- `pr_policy: __u16`
- `pr_value: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sctp_prinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sctp_rcvinfo

*Struct*

**Fields:**
- `rcv_sid: __u16`
- `rcv_ssn: __u16`
- `rcv_flags: __u16`
- `rcv_ppid: __u32`
- `rcv_tsn: __u32`
- `rcv_cumtsn: __u32`
- `rcv_context: __u32`
- `rcv_assoc_id: crate::sctp_assoc_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sctp_rcvinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sctp_sndinfo

*Struct*

**Fields:**
- `snd_sid: __u16`
- `snd_flags: __u16`
- `snd_ppid: __u32`
- `snd_context: __u32`
- `snd_assoc_id: crate::sctp_assoc_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sctp_sndinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sctp_sndrcvinfo

*Struct*

**Fields:**
- `sinfo_stream: __u16`
- `sinfo_ssn: __u16`
- `sinfo_flags: __u16`
- `sinfo_ppid: __u32`
- `sinfo_context: __u32`
- `sinfo_timetolive: __u32`
- `sinfo_tsn: __u32`
- `sinfo_cumtsn: __u32`
- `sinfo_assoc_id: crate::sctp_assoc_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sctp_sndrcvinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::seccomp_data

*Struct*

**Fields:**
- `nr: c_int`
- `arch: __u32`
- `instruction_pointer: crate::__u64`
- `args: [crate::__u64; 6]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seccomp_data`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::seccomp_notif

*Struct*

**Fields:**
- `id: crate::__u64`
- `pid: __u32`
- `flags: __u32`
- `data: seccomp_data`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seccomp_notif`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::seccomp_notif_addfd

*Struct*

**Fields:**
- `id: crate::__u64`
- `flags: __u32`
- `srcfd: __u32`
- `newfd: __u32`
- `newfd_flags: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seccomp_notif_addfd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::seccomp_notif_resp

*Struct*

**Fields:**
- `id: crate::__u64`
- `val: crate::__s64`
- `error: __s32`
- `flags: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seccomp_notif_resp`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::seccomp_notif_sizes

*Struct*

**Fields:**
- `seccomp_notif: __u16`
- `seccomp_notif_resp: __u16`
- `seccomp_data: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seccomp_notif_sizes`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::seed48

*Function*

```rust
fn seed48(xseed: *mut c_ushort) -> *mut c_ushort
```



## libc::unix::linux_like::linux::semctl

*Function*

```rust
fn semctl(semid: c_int, semnum: c_int, cmd: c_int) -> c_int
```



## libc::unix::linux_like::linux::semget

*Function*

```rust
fn semget(key: crate::key_t, nsems: c_int, semflag: c_int) -> c_int
```



## libc::unix::linux_like::linux::semop

*Function*

```rust
fn semop(semid: c_int, sops: *mut crate::sembuf, nsops: size_t) -> c_int
```



## libc::unix::linux_like::linux::sendfile

*Function*

```rust
fn sendfile(out_fd: c_int, in_fd: c_int, offset: *mut off_t, count: size_t) -> ssize_t
```



## libc::unix::linux_like::linux::sendfile64

*Function*

```rust
fn sendfile64(out_fd: c_int, in_fd: c_int, offset: *mut off64_t, count: size_t) -> ssize_t
```



## libc::unix::linux_like::linux::setfsgid

*Function*

```rust
fn setfsgid(gid: crate::gid_t) -> c_int
```



## libc::unix::linux_like::linux::setfsuid

*Function*

```rust
fn setfsuid(uid: crate::uid_t) -> c_int
```



## libc::unix::linux_like::linux::setns

*Function*

```rust
fn setns(fd: c_int, nstype: c_int) -> c_int
```



## libc::unix::linux_like::linux::setxattr

*Function*

```rust
fn setxattr(path: *const c_char, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::shm_open

*Function*

```rust
fn shm_open(name: *const c_char, oflag: c_int, mode: mode_t) -> c_int
```



## libc::unix::linux_like::linux::shm_unlink

*Function*

```rust
fn shm_unlink(name: *const c_char) -> c_int
```



## libc::unix::linux_like::linux::sigaltstack

*Function*

```rust
fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> c_int
```



## libc::unix::linux_like::linux::signalfd

*Function*

```rust
fn signalfd(fd: c_int, mask: *const crate::sigset_t, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::signalfd_siginfo

*Struct*

**Fields:**
- `ssi_signo: u32`
- `ssi_errno: i32`
- `ssi_code: i32`
- `ssi_pid: u32`
- `ssi_uid: u32`
- `ssi_fd: i32`
- `ssi_tid: u32`
- `ssi_band: u32`
- `ssi_overrun: u32`
- `ssi_trapno: u32`
- `ssi_status: i32`
- `ssi_int: i32`
- `ssi_ptr: u64`
- `ssi_utime: u64`
- `ssi_stime: u64`
- `ssi_addr: u64`
- `ssi_addr_lsb: u16`
- `ssi_syscall: i32`
- `ssi_call_addr: u64`
- `ssi_arch: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> signalfd_siginfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sigtimedwait

*Function*

```rust
fn sigtimedwait(set: *const sigset_t, info: *mut siginfo_t, timeout: *const crate::timespec) -> c_int
```



## libc::unix::linux_like::linux::sigwaitinfo

*Function*

```rust
fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> c_int
```



## libc::unix::linux_like::linux::sock_extended_err

*Struct*

**Fields:**
- `ee_errno: u32`
- `ee_origin: u8`
- `ee_type: u8`
- `ee_code: u8`
- `ee_pad: u8`
- `ee_info: u32`
- `ee_data: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sock_extended_err`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sock_txtime

*Struct*

**Fields:**
- `clockid: crate::clockid_t`
- `flags: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sock_txtime`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sockaddr_alg

*Struct*

**Fields:**
- `salg_family: crate::sa_family_t`
- `salg_type: [c_uchar; 14]`
- `salg_feat: u32`
- `salg_mask: u32`
- `salg_name: [c_uchar; 64]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_alg`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sockaddr_pkt

*Struct*

**Fields:**
- `spkt_family: c_ushort`
- `spkt_device: [c_uchar; 14]`
- `spkt_protocol: c_ushort`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_pkt`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sockaddr_vm

*Struct*

**Fields:**
- `svm_family: crate::sa_family_t`
- `svm_reserved1: c_ushort`
- `svm_port: c_uint`
- `svm_cid: c_uint`
- `svm_zero: [u8; 4]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_vm`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::sockaddr_xdp

*Struct*

**Fields:**
- `sxdp_family: crate::__u16`
- `sxdp_flags: crate::__u16`
- `sxdp_ifindex: crate::__u32`
- `sxdp_queue_id: crate::__u32`
- `sxdp_shared_umem_fd: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_xdp`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::socklen_t

*Type Alias*: `u32`



## libc::unix::linux_like::linux::splice

*Function*

```rust
fn splice(fd_in: c_int, off_in: *mut loff_t, fd_out: c_int, off_out: *mut loff_t, len: size_t, flags: c_uint) -> ssize_t
```



## libc::unix::linux_like::linux::swapoff

*Function*

```rust
fn swapoff(path: *const c_char) -> c_int
```



## libc::unix::linux_like::linux::swapon

*Function*

```rust
fn swapon(path: *const c_char, swapflags: c_int) -> c_int
```



## libc::unix::linux_like::linux::sync

*Function*

```rust
fn sync()
```



## libc::unix::linux_like::linux::sync_file_range

*Function*

```rust
fn sync_file_range(fd: c_int, offset: off64_t, nbytes: off64_t, flags: c_uint) -> c_int
```



## libc::unix::linux_like::linux::syncfs

*Function*

```rust
fn syncfs(fd: c_int) -> c_int
```



## libc::unix::linux_like::linux::syscall

*Function*

```rust
fn syscall(num: c_long) -> c_long
```



## libc::unix::linux_like::linux::tee

*Function*

```rust
fn tee(fd_in: c_int, fd_out: c_int, len: size_t, flags: c_uint) -> ssize_t
```



## libc::unix::linux_like::linux::timerfd_create

*Function*

```rust
fn timerfd_create(clockid: crate::clockid_t, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::timerfd_gettime

*Function*

```rust
fn timerfd_gettime(fd: c_int, curr_value: *mut crate::itimerspec) -> c_int
```



## libc::unix::linux_like::linux::timerfd_settime

*Function*

```rust
fn timerfd_settime(fd: c_int, flags: c_int, new_value: *const crate::itimerspec, old_value: *mut crate::itimerspec) -> c_int
```



## libc::unix::linux_like::linux::tls12_crypto_info_aes_ccm_128

*Struct*

**Fields:**
- `info: tls_crypto_info`
- `iv: [c_uchar; 8]`
- `key: [c_uchar; 16]`
- `salt: [c_uchar; 4]`
- `rec_seq: [c_uchar; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tls12_crypto_info_aes_ccm_128`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tls12_crypto_info_aes_gcm_128

*Struct*

**Fields:**
- `info: tls_crypto_info`
- `iv: [c_uchar; 8]`
- `key: [c_uchar; 16]`
- `salt: [c_uchar; 4]`
- `rec_seq: [c_uchar; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tls12_crypto_info_aes_gcm_128`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tls12_crypto_info_aes_gcm_256

*Struct*

**Fields:**
- `info: tls_crypto_info`
- `iv: [c_uchar; 8]`
- `key: [c_uchar; 32]`
- `salt: [c_uchar; 4]`
- `rec_seq: [c_uchar; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tls12_crypto_info_aes_gcm_256`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tls12_crypto_info_aria_gcm_128

*Struct*

**Fields:**
- `info: tls_crypto_info`
- `iv: [c_uchar; 8]`
- `key: [c_uchar; 16]`
- `salt: [c_uchar; 4]`
- `rec_seq: [c_uchar; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tls12_crypto_info_aria_gcm_128`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tls12_crypto_info_aria_gcm_256

*Struct*

**Fields:**
- `info: tls_crypto_info`
- `iv: [c_uchar; 8]`
- `key: [c_uchar; 32]`
- `salt: [c_uchar; 4]`
- `rec_seq: [c_uchar; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tls12_crypto_info_aria_gcm_256`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tls12_crypto_info_chacha20_poly1305

*Struct*

**Fields:**
- `info: tls_crypto_info`
- `iv: [c_uchar; 12]`
- `key: [c_uchar; 32]`
- `salt: [c_uchar; 0]`
- `rec_seq: [c_uchar; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tls12_crypto_info_chacha20_poly1305`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tls12_crypto_info_sm4_ccm

*Struct*

**Fields:**
- `info: tls_crypto_info`
- `iv: [c_uchar; 8]`
- `key: [c_uchar; 16]`
- `salt: [c_uchar; 4]`
- `rec_seq: [c_uchar; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tls12_crypto_info_sm4_ccm`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tls12_crypto_info_sm4_gcm

*Struct*

**Fields:**
- `info: tls_crypto_info`
- `iv: [c_uchar; 8]`
- `key: [c_uchar; 16]`
- `salt: [c_uchar; 4]`
- `rec_seq: [c_uchar; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tls12_crypto_info_sm4_gcm`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tls_crypto_info

*Struct*

**Fields:**
- `version: __u16`
- `cipher_type: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tls_crypto_info`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tmpfile64

*Function*

```rust
fn tmpfile64() -> *mut crate::FILE
```



## libc::unix::linux_like::linux::tpacket2_hdr

*Struct*

**Fields:**
- `tp_status: __u32`
- `tp_len: __u32`
- `tp_snaplen: __u32`
- `tp_mac: __u16`
- `tp_net: __u16`
- `tp_sec: __u32`
- `tp_nsec: __u32`
- `tp_vlan_tci: __u16`
- `tp_vlan_tpid: __u16`
- `tp_padding: [__u8; 4]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket2_hdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket3_hdr

*Struct*

**Fields:**
- `tp_next_offset: __u32`
- `tp_sec: __u32`
- `tp_nsec: __u32`
- `tp_snaplen: __u32`
- `tp_len: __u32`
- `tp_status: __u32`
- `tp_mac: __u16`
- `tp_net: __u16`
- `hv1: crate::tpacket_hdr_variant1`
- `tp_padding: [__u8; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket3_hdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_auxdata

*Struct*

**Fields:**
- `tp_status: __u32`
- `tp_len: __u32`
- `tp_snaplen: __u32`
- `tp_mac: __u16`
- `tp_net: __u16`
- `tp_vlan_tci: __u16`
- `tp_vlan_tpid: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_auxdata`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_bd_header_u

*Union*

**Fields:**
- `bh1: crate::tpacket_hdr_v1`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_bd_header_u`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::unix::linux_like::linux::tpacket_bd_ts

*Struct*

**Fields:**
- `ts_sec: c_uint`
- `ts_usec: c_uint`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_bd_ts`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_block_desc

*Struct*

**Fields:**
- `version: __u32`
- `offset_to_priv: __u32`
- `hdr: crate::tpacket_bd_header_u`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_block_desc`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_hdr

*Struct*

**Fields:**
- `tp_status: c_ulong`
- `tp_len: c_uint`
- `tp_snaplen: c_uint`
- `tp_mac: c_ushort`
- `tp_net: c_ushort`
- `tp_sec: c_uint`
- `tp_usec: c_uint`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_hdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_hdr_v1

*Struct*

**Fields:**
- `block_status: __u32`
- `num_pkts: __u32`
- `offset_to_first_pkt: __u32`
- `blk_len: __u32`
- `seq_num: crate::__u64`
- `ts_first_pkt: crate::tpacket_bd_ts`
- `ts_last_pkt: crate::tpacket_bd_ts`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_hdr_v1`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_hdr_variant1

*Struct*

**Fields:**
- `tp_rxhash: __u32`
- `tp_vlan_tci: __u32`
- `tp_vlan_tpid: __u16`
- `tp_padding: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_hdr_variant1`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_req

*Struct*

**Fields:**
- `tp_block_size: c_uint`
- `tp_block_nr: c_uint`
- `tp_frame_size: c_uint`
- `tp_frame_nr: c_uint`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_req`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_req3

*Struct*

**Fields:**
- `tp_block_size: c_uint`
- `tp_block_nr: c_uint`
- `tp_frame_size: c_uint`
- `tp_frame_nr: c_uint`
- `tp_retire_blk_tov: c_uint`
- `tp_sizeof_priv: c_uint`
- `tp_feature_req_word: c_uint`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_req3`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_req_u

*Union*

**Fields:**
- `req: crate::tpacket_req`
- `req3: crate::tpacket_req3`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_req_u`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::unix::linux_like::linux::tpacket_rollover_stats

*Struct*

**Fields:**
- `tp_all: crate::__u64`
- `tp_huge: crate::__u64`
- `tp_failed: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_rollover_stats`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_stats

*Struct*

**Fields:**
- `tp_packets: c_uint`
- `tp_drops: c_uint`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_stats`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_stats_v3

*Struct*

**Fields:**
- `tp_packets: c_uint`
- `tp_drops: c_uint`
- `tp_freeze_q_cnt: c_uint`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_stats_v3`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::tpacket_versions

*Enum*

**Variants:**
- `TPACKET_V1`
- `TPACKET_V2`
- `TPACKET_V3`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tpacket_versions`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::uinput_abs_setup

*Struct*

**Fields:**
- `code: __u16`
- `absinfo: input_absinfo`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> uinput_abs_setup`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::uinput_ff_erase

*Struct*

**Fields:**
- `request_id: __u32`
- `retval: __s32`
- `effect_id: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> uinput_ff_erase`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::uinput_ff_upload

*Struct*

**Fields:**
- `request_id: __u32`
- `retval: __s32`
- `effect: ff_effect`
- `old: ff_effect`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> uinput_ff_upload`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::uinput_setup

*Struct*

**Fields:**
- `id: input_id`
- `name: [c_char; 80]`
- `ff_effects_max: __u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> uinput_setup`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::uinput_user_dev

*Struct*

**Fields:**
- `name: [c_char; 80]`
- `id: input_id`
- `ff_effects_max: __u32`
- `absmax: [__s32; 64]`
- `absmin: [__s32; 64]`
- `absfuzz: [__s32; 64]`
- `absflat: [__s32; 64]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> uinput_user_dev`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::umount

*Function*

```rust
fn umount(target: *const c_char) -> c_int
```



## libc::unix::linux_like::linux::umount2

*Function*

```rust
fn umount2(target: *const c_char, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::unshare

*Function*

```rust
fn unshare(flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::vhangup

*Function*

```rust
fn vhangup() -> c_int
```



## libc::unix::linux_like::linux::vmsplice

*Function*

```rust
fn vmsplice(fd: c_int, iov: *const crate::iovec, nr_segs: size_t, flags: c_uint) -> ssize_t
```



## libc::unix::linux_like::linux::xdp_desc

*Struct*

**Fields:**
- `addr: crate::__u64`
- `len: crate::__u32`
- `options: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xdp_desc`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xdp_mmap_offsets

*Struct*

**Fields:**
- `rx: xdp_ring_offset`
- `tx: xdp_ring_offset`
- `fr: xdp_ring_offset`
- `cr: xdp_ring_offset`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xdp_mmap_offsets`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xdp_mmap_offsets_v1

*Struct*

**Fields:**
- `rx: xdp_ring_offset_v1`
- `tx: xdp_ring_offset_v1`
- `fr: xdp_ring_offset_v1`
- `cr: xdp_ring_offset_v1`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xdp_mmap_offsets_v1`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xdp_options

*Struct*

**Fields:**
- `flags: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xdp_options`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xdp_ring_offset

*Struct*

**Fields:**
- `producer: crate::__u64`
- `consumer: crate::__u64`
- `desc: crate::__u64`
- `flags: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xdp_ring_offset`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xdp_ring_offset_v1

*Struct*

**Fields:**
- `producer: crate::__u64`
- `consumer: crate::__u64`
- `desc: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xdp_ring_offset_v1`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xdp_statistics

*Struct*

**Fields:**
- `rx_dropped: crate::__u64`
- `rx_invalid_descs: crate::__u64`
- `tx_invalid_descs: crate::__u64`
- `rx_ring_full: crate::__u64`
- `rx_fill_ring_empty_descs: crate::__u64`
- `tx_ring_empty_descs: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xdp_statistics`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xdp_statistics_v1

*Struct*

**Fields:**
- `rx_dropped: crate::__u64`
- `rx_invalid_descs: crate::__u64`
- `tx_invalid_descs: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xdp_statistics_v1`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xdp_umem_reg

*Struct*

**Fields:**
- `addr: crate::__u64`
- `len: crate::__u64`
- `chunk_size: crate::__u32`
- `headroom: crate::__u32`
- `flags: crate::__u32`
- `tx_metadata_len: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xdp_umem_reg`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xdp_umem_reg_v1

*Struct*

**Fields:**
- `addr: crate::__u64`
- `len: crate::__u64`
- `chunk_size: crate::__u32`
- `headroom: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xdp_umem_reg_v1`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xsk_tx_metadata

*Struct*

**Fields:**
- `flags: crate::__u64`
- `xsk_tx_metadata_union: __c_anonymous_xsk_tx_metadata_union`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xsk_tx_metadata`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xsk_tx_metadata_completion

*Struct*

**Fields:**
- `tx_timestamp: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xsk_tx_metadata_completion`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::xsk_tx_metadata_request

*Struct*

**Fields:**
- `csum_start: __u16`
- `csum_offset: __u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> xsk_tx_metadata_request`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



