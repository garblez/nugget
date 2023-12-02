use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};

/// Return a mutable reference to the active level 4 page table.
/// 
/// Unsafe as the caller must guarantee that the complete
/// physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this must be called once only
/// to avoid aliasing any mutable references `&mut` as this is 
/// undefined behaviour.
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt =  physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // Unsafe!
}