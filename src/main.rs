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
    use nugget::memory::translate_addr;
    use x86_64::VirtAddr;

    println!("Hello World, this is {}: a basic operating system for learning.", "NUGGET");
    // panic!("Oops! Something went terribly wrong. Please restart the machine.");

    nugget::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    let addresses = [
        // identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
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
