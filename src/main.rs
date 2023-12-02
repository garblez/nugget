#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nugget::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use nugget::{println, memory::BootInfoFrameAllocator};

entry_point!(kernel_main); // Type-check the entry point for the signature expected by the bootloader.

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use nugget::memory;
    use x86_64::{
        structures::paging::Page,
        VirtAddr,
    };

    println!("Hello World, this is {}: a basic operating system for learning.", "NUGGET");
    // panic!("Oops! Something went terribly wrong. Please restart the machine.");

    nugget::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);


    // initialise a memory Mapper
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    
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
