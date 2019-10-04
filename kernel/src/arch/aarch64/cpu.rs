use crate::consts::SMP_CORES;
use crate::memory::{kernel_offset, phys_to_virt};
use aarch64::asm;
use core::{cmp, mem};

pub use super::board::{CPU_NUM, CPU_SPIN_TABLE};

pub fn halt() {
    asm::wfi();
}

pub fn id() -> usize {
    asm::cpuid()
}

pub unsafe fn start_others() {
    extern "C" {
        fn slave_startup();
    }
    for i in 0..cmp::min(CPU_NUM, *SMP_CORES) {
        if i == 0 {
            continue;
        }
        let release_addr = phys_to_virt(CPU_SPIN_TABLE[i]) as *mut usize;
        let entry_addr = kernel_offset(slave_startup as usize);
        *release_addr = entry_addr;
        asm::flush_dcache_range(
            release_addr as usize,
            release_addr as usize + mem::size_of::<usize>(),
        );
        asm::sev();
    }
}

pub unsafe fn exit_in_qemu(_error_code: u8) -> ! {
    unimplemented!()
}

pub unsafe fn reboot() -> ! {
    unimplemented!()
}
