use sel4::{
    ObjectBlueprint, cap_type, debug_println,
    init_thread::Slot,
};

#[derive(Copy, Clone)]
pub enum PoolSize {
    Small,
    Large,
    Huge,
}

impl PoolSize {
    pub const fn frame_size_bits(self) -> usize {
        match self {
            Self::Small => 12,
            Self::Large => 21,
            Self::Huge => 30,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Large => "large",
            Self::Huge => "huge",
        }
    }
}

#[derive(Copy, Clone)]
pub struct Pool {
    pub size: PoolSize,
    pub cnode_slot: Slot<cap_type::CNode>,
    pub cnode_radix: usize,
}

impl Pool {
    pub fn new(
        root_cnode: Slot<cap_type::CNode>,
        pool_cnode_untyped: Slot<cap_type::Untyped>,
        size: PoolSize,
        cnode_slot: Slot<cap_type::Null>,
        cnode_radix: usize,
    ) -> sel4::Result<Self> {
        debug_println!(
            "[chord] creating {} pool CNode: frame_size_bits={} radix={} slot={}",
            size.name(),
            size.frame_size_bits(),
            cnode_radix,
            cnode_slot.index()
        );

        let dst = root_cnode.cap().absolute_cptr_for_self();
        pool_cnode_untyped.cap().untyped_retype(
            &ObjectBlueprint::CNode {
                size_bits: cnode_radix,
            },
            &dst,
            cnode_slot.index(),
            1,
        )?;

        Ok(Self {
            size,
            cnode_slot: cnode_slot.cast::<cap_type::CNode>(),
            cnode_radix,
        })
    }
}
