pub mod pool;

use alloc::vec::Vec;

use sel4::{
    BootInfoPtr, debug_println, cap_type,
    init_thread::{Slot, SlotRegion},
};

pub use pool::{Pool, PoolSize};

pub fn cnode_radix_for_slots(slots: usize) -> usize {
    (slots.next_power_of_two().ilog2() as usize).max(1)
}

pub struct Bootstrap {
    pub root_cnode: Slot<cap_type::CNode>,
    pub pool_cnode_untyped_idx: usize,
    pub pool_cnode_untyped: Slot<cap_type::Untyped>,
    pub free_slots: Vec<Slot<cap_type::Null>>,
    pub small_pool: Pool,
    pub large_pool: Pool,
    pub huge_pool: Pool,
}

impl Bootstrap {
    pub fn from_bootinfo(bootinfo: &BootInfoPtr) -> sel4::Result<Self> {
        debug_println!("[chord] === Pool Bootstrap ===");

        let mut small_slots: usize = 0;
        let mut large_slots: usize = 0;
        let mut huge_slots: usize = 0;

        for ut_desc in bootinfo.untyped_list().iter() {
            if !ut_desc.is_device() {
                let bits = ut_desc.size_bits();

                match bits {
                    30..=usize::MAX => {
                        huge_slots += 1usize << (bits - PoolSize::Huge.frame_size_bits());
                    }
                    21..=29 => {
                        large_slots += 1usize << (bits - PoolSize::Large.frame_size_bits());
                    }
                    12..=20 => {
                        small_slots += 1usize << (bits - PoolSize::Small.frame_size_bits());
                    }
                    _ => (),
                }
            }
        }

        debug_println!("[chord] slots small - {}", small_slots);
        debug_println!("[chord] slots large - {}", large_slots);
        debug_println!("[chord] slots huge - {}", huge_slots);

        let small_cnode_radix = cnode_radix_for_slots(small_slots);
        let large_cnode_radix = cnode_radix_for_slots(large_slots);
        let huge_cnode_radix = cnode_radix_for_slots(huge_slots);
        debug_println!("[chord] cnode radix small - {}", small_cnode_radix);
        debug_println!("[chord] cnode radix large - {}", large_cnode_radix);
        debug_println!("[chord] cnode radix huge - {}", huge_cnode_radix);

        let untyped_list = bootinfo.untyped_list();
        let idx: usize = bootinfo
            .kernel_untyped_range()
            .find(|&i| {
                let ut = &untyped_list[i];
                !ut.is_device() && ut.size_bits() == 21
            })
            .expect("Failed to find a kernel untyped region with size_bits == 21");

        let untyped_slot = bootinfo.untyped().index(idx);
        let untyped_desc = &untyped_list[idx];
        debug_println!(
            "[chord] pool cnode untyped idx={} paddr={:#x} size_bits={} is_device={}",
            idx,
            untyped_desc.paddr(),
            untyped_desc.size_bits(),
            untyped_desc.is_device()
        );

        let root_cnode = sel4::init_thread::slot::CNODE;
        Self::new(
            root_cnode,
            idx,
            untyped_slot,
            bootinfo.empty(),
            small_cnode_radix,
            large_cnode_radix,
            huge_cnode_radix,
        )
    }

    fn new(
        root_cnode: Slot<cap_type::CNode>,
        pool_cnode_untyped_idx: usize,
        pool_cnode_untyped: Slot<cap_type::Untyped>,
        empty_slots: SlotRegion<cap_type::Null>,
        small_cnode_radix: usize,
        large_cnode_radix: usize,
        huge_cnode_radix: usize,
    ) -> sel4::Result<Self> {
        let mut free_slots: Vec<Slot<cap_type::Null>> =
            empty_slots.range().rev().map(Slot::from_index).collect();

        let small_cnode_slot = free_slots.pop().expect("Out of empty CSlots!");
        let large_cnode_slot = free_slots.pop().expect("Out of empty CSlots!");
        let huge_cnode_slot = free_slots.pop().expect("Out of empty CSlots!");

        Ok(Self {
            root_cnode,
            pool_cnode_untyped_idx,
            pool_cnode_untyped,
            free_slots,
            small_pool: Pool::new(
                root_cnode,
                pool_cnode_untyped,
                PoolSize::Small,
                small_cnode_slot,
                small_cnode_radix,
            )?,
            large_pool: Pool::new(
                root_cnode,
                pool_cnode_untyped,
                PoolSize::Large,
                large_cnode_slot,
                large_cnode_radix,
            )?,
            huge_pool: Pool::new(
                root_cnode,
                pool_cnode_untyped,
                PoolSize::Huge,
                huge_cnode_slot,
                huge_cnode_radix,
            )?,
        })
    }

    pub fn pop_free_slot(&mut self) -> Option<Slot<cap_type::Null>> {
        self.free_slots.pop()
    }

    pub fn pool(&self, size: PoolSize) -> &Pool {
        match size {
            PoolSize::Small => &self.small_pool,
            PoolSize::Large => &self.large_pool,
            PoolSize::Huge => &self.huge_pool,
        }
    }
}
