//! Bootstrap module — root-task capability pool construction and `CSpace`
//! configuration.
//!
//! # Overview
//!
//! After the seL4 kernel boots the root task, the initial thread's `CSpace` is a
//! flat, single-level [`CNode`](sel4::cap_type::CNode) populated with the
//! capabilities the kernel placed at boot time. This module transforms that
//! initial layout into a two-level `CSpace` where pool `CNode` slots are
//! directly addressable via a compact `CPtr` encoding.
//!
//! # Module structure
//!
//! - [`bootstrap`] — the [`Bootstrap`] struct (root-task state) and its
//!   constructor [`Bootstrap::new`].
//! - [`cspace`] — `CSpace` guard configuration and `CPtr` encoding functions.
//! - [`error`] — [`BootstrapError`] enum for fallible bootstrap operations.
//! - [`pool`] — [`Pool`] and [`PoolSize`] types for capability pool management.
//!
//! # `CSpace` layout after bootstrap
//!
//! ```text
//! CPtr bits (WORD_SIZE wide):
//! ┌──────────────┬──────────────┬──────────────────┐
//! │  guard (0s)  │  root slot   │  pool-internal   │
//! │  guard_size  │  root_radix  │  pool_slot_radix │
//! └──────────────┴──────────────┴──────────────────┘
//! ```
//!
//! The guard is always zero, so the kernel skips `guard_size` high bits and
//! resolves the remaining bits as: root `CNode` slot → pool `CNode` internal
//! slot.

#[expect(
    clippy::module_inception,
    reason = "bootstrap.rs defines the Bootstrap type"
)]
pub mod bootstrap;
pub mod cspace;
pub mod error;
pub mod pool;

pub use bootstrap::Bootstrap;
#[expect(unused_imports, reason = "public API surface for external consumers")]
pub use error::BootstrapError;
#[expect(unused_imports, reason = "public API surface for external consumers")]
pub use pool::{Pool, PoolSize};
