use x86_64::{
    structures::paging::{
        PageTable,
        OffsetPageTable,
    },
    VirtAddr,
    PhysAddr,
};

/// Return a mutable reference to the active level 4 page table.
/// 
/// Unsafe as the caller must guarantee that the complete
/// physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this must be called once only
/// to avoid aliasing any mutable references `&mut` as this is 
/// undefined behaviour.
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt =  physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // Unsafe!
}


/// Translates the given virtual address to the
/// mapped physical address, `None` if no address map exists.
/// 
/// Unsafe as the caller must guarantee the complete physical
/// memory is mapped to virtual memory at the passed
/// `physical_memory_offset`.
pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}


/// Private inner translation called by `translate_addr`
/// 
/// This function is safe to limit the scope of `unsafe` since
/// Rust treats the whole body of unsafe functions as an unsafe
/// block. This function must only be reachable through
/// `unsafe fn` from outside of this module!
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::{
        structures::paging::page_table::FrameError,
        registers::control::Cr3,
    };

    // Read active level 4 frame from Cr3
    let (level_4_table_frame, _) = Cr3::read();

    let table_indices = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];

    let mut frame = level_4_table_frame;

    // Traverse the multi-level page table
    for &index in &table_indices {
        // Convert the frame into a page table reference
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        // Read the page table and update `frame`
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages are not supported"),
        };
    }

    // Calculate the physical address by adding the page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}


/// Initialise a new OffsetPageTable
/// 
/// This is unsafe as the caller must guarantee that the complete
/// physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must only be
/// called once to avoid aliasing `&mut` references.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}
