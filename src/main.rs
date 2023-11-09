#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    println!("Hello World, this is {}: a basic operating system for learning.", "NUGGET");
    panic!("Oops! Something went terribly wrong. Please restart the machine.");

    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
