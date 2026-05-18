use core::fmt;

/// Errors that can occur during bootstrap initialization.
///
/// Each variant represents a distinct failure mode encountered while
/// constructing the [`Bootstrap`](super::Bootstrap) state from the kernel
/// bootinfo. The [`From<sel4::Error>`](impl-From<sel4::Error>) impl allows
/// using `?` to propagate kernel errors directly.
///
/// # Examples
///
/// ```ignore
/// let bootstrap = Bootstrap::new(bootinfo)?;
/// ```
#[derive(Debug)]
pub enum BootstrapError {
    /// No suitable kernel untyped region found to back pool
    /// [`CNodes`](super::Pool).
    ///
    /// The bootstrap process requires a non-device kernel untyped region with
    /// `size_bits == 21` (exactly 2 MiB) to create the three pool `CNode`
    /// capabilities. If no such region exists in the kernel untyped range,
    /// this error is returned.
    NoBackingUntyped,

    /// Not enough empty [`CSlots`](sel4::init_thread::Slot) in the root
    /// [`CNode`](sel4::cap_type::CNode) for pool [`CNodes`](super::Pool).
    ///
    /// Three empty slots are needed in the root `CNode` to place the Small,
    /// Large, and Huge pool `CNode` capabilities. If fewer than three empty
    /// slots remain, this error is returned.
    NoEmptyCSlots,

    /// A seL4 kernel operation failed.
    ///
    /// Wraps the original [`sel4::Error`] returned by the kernel for
    /// operations like `untyped_retype` or `tcb_set_space`. The `From` impl
    /// allows automatic conversion with `?`.
    Sel4(sel4::Error),
}

impl From<sel4::Error> for BootstrapError {
    /// Converts a kernel error into [`BootstrapError::Sel4`].
    ///
    /// This enables using the `?` operator on any `sel4::Result` within
    /// bootstrap code:
    ///
    /// ```ignore
    /// pool_cnode_untyped.cap().untyped_retype(...)?; // auto-converts
    /// ```
    fn from(err: sel4::Error) -> Self {
        Self::Sel4(err)
    }
}

impl fmt::Display for BootstrapError {
    /// Formats the error in a human-readable form for diagnostics.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoBackingUntyped => {
                write!(f, "no kernel untyped region with size_bits == 21 found")
            }
            Self::NoEmptyCSlots => write!(f, "not enough empty CSlots for pool CNodes"),
            Self::Sel4(err) => write!(f, "seL4 error: {err}"),
        }
    }
}
