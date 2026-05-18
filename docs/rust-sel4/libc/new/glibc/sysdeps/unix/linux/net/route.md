**libc > new > glibc > sysdeps > unix > linux > net > route**

# Module: new::glibc::sysdeps::unix::linux::net::route

## Contents

**Structs**

- [`rtentry`](#rtentry)

---

## libc::new::glibc::sysdeps::unix::linux::net::route::rtentry

*Struct*

**Fields:**
- `rt_pad1: c_ulong`
- `rt_dst: crate::sockaddr`
- `rt_gateway: crate::sockaddr`
- `rt_genmask: crate::sockaddr`
- `rt_flags: c_ushort`
- `rt_pad2: c_short`
- `rt_pad3: c_ulong`
- `rt_tos: c_uchar`
- `rt_class: c_uchar`
- `rt_pad4: [c_short; 3]`
- `rt_metric: c_short`
- `rt_dev: *mut c_char`
- `rt_mtu: c_ulong`
- `rt_window: c_ulong`
- `rt_irtt: c_ushort`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> rtentry`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



