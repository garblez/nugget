#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nugget::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use nugget::println;

entry_point!(kernel_main); // Type-check the entry point for the signature expected by the bootloader.

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use nugget::memory::active_level_4_table;
    use x86_64::{
        structures::paging::PageTable,
        VirtAddr,
    };

    println!("Hello World, this is {}: a basic operating system for learning.", "NUGGET");
    // panic!("Oops! Something went terribly wrong. Please restart the machine.");

    nugget::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() { // Only print non-empties so to not fill up the screen
            println!("L4 Entry {}: {:?}", i, entry);

            // Get the physical address from the entry and convert it
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            // Print non-empties of level 3 page table
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("  L3 Entry {}: {:?}", i, entry);

                    // Get the physical address from the entry and convert it
                    let phys = entry.frame().unwrap().start_address();
                    let virt = phys.as_u64() + boot_info.physical_memory_offset;
                    let ptr = VirtAddr::new(virt).as_mut_ptr();
                    let l2_table: &PageTable = unsafe { &*ptr };

                    // Print non-empties of level 2 page table
                    for (i, entry) in l2_table.iter().enumerate() {
                        if !entry.is_unused() {
                            println!("      L2 Entry {}: {:?}", i, entry);

                            // Get the physical address from the entry and convert it
                            let phys = entry.frame().unwrap().start_address();
                            let virt = phys.as_u64() + boot_info.physical_memory_offset;
                            let ptr = VirtAddr::new(virt).as_mut_ptr();
                            let l1_table: &PageTable = unsafe { &*ptr };

                            // Print non-empties of level 1 page table
                            for (i, entry) in l1_table.iter().enumerate() {
                                if !entry.is_unused() {
                                    println!("          L1 Entry {}: {:?}", i, entry);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(test)]
    test_main(); // Run tests conditionally in testing contexts

    println!("It did not crash!");
    nugget::hlt_loop();
}

#[cfg(not(test))] // Normal panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    nugget::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nugget::test_panic_handler(info)
}
