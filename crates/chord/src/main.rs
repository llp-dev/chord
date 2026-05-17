#![no_std]
#![no_main]

extern crate alloc;

mod bootstrap;
mod debug;

use bootstrap::Bootstrap;
use sel4::{BootInfoPtr, debug_println};
use sel4_root_task::root_task;

#[root_task(heap_size = 5 * 1024 * 1024)]
fn main(bootinfo: &BootInfoPtr) -> ! {
    debug::dump_bootinfo(bootinfo);

    let bootstrap = Bootstrap::from_bootinfo(bootinfo)
        .expect("Failed to create bootstrap pool CNodes");
    debug_println!(
        "[chord] bootstrap cnode backing untyped idx={}",
        bootstrap.pool_cnode_untyped_idx
    );

    sel4::init_thread::suspend_self()
}
