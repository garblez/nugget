#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nugget::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use nugget::println;

#[no_mangle] // each integration test is its own binary so we need to give it a start point (unmangled obviously)
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nugget::test_panic_handler(info)
}


#[test_case]
fn test_println() {
    println!("test_println output"); // verify that println works after booting
}
