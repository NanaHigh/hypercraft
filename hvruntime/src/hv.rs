use axalloc::global_allocator;
use axhal::mem::PAGE_SIZE_4K;
use hypercraft::{HostPhysAddr, HyperCraftHal, VmExitInfo};

pub struct HyperCraftHalImpl;

impl HyperCraftHal for HyperCraftHalImpl {
    fn alloc_pages(num_pages: usize) -> Option<hypercraft::HostPhysAddr> {
        global_allocator()
            .alloc_pages(num_pages, PAGE_SIZE_4K)
            .map(|pa| pa as HostPhysAddr)
            .ok()
    }

    fn dealloc_pages(pa: HostPhysAddr, num_pages: usize) {
        global_allocator().dealloc_pages(pa as usize, num_pages);
    }

    // implement by app or runtime
    fn vmexit_handler(vcpu: &mut hypercraft::VCpu<Self>, vm_exit_info: hypercraft::VmExitInfo) {
        match vm_exit_info {
            VmExitInfo::Ecall(sbi_msg) => {
                if let Some(sbi_msg) = sbi_msg {
                    match sbi_msg {
                        hypercraft::HyperCallMsg::PutChar(c) => {
                            axhal::console::putchar(c as u8);
                            vcpu.advance_pc(4);
                        }
                        hypercraft::HyperCallMsg::Reset(_) => axhal::misc::terminate(),
                        _ => todo!(),
                    }
                } else {
                    panic!()
                }
            }
            _ => todo!(),
        }
    }
}
