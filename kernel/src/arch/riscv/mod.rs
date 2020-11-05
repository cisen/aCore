pub mod context;
pub mod cpu;
pub mod io;
pub mod memory;
mod paging;
mod sbi;
mod traps;
pub mod syscall_ids;

global_asm!(include_str!("boot/entry.S"));

pub fn boot_cpu_id() -> usize {
    context::read_tls()
}

pub fn primary_init_early(_hartid: usize, _device_tree_paddr: usize) {}

pub fn primary_init(_hartid: usize, _device_tree_paddr: usize) {}

pub fn secondary_init(_hartid: usize, _device_tree_paddr: usize) {}
