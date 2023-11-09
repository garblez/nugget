#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"] // Make sure our test harness' entry point is called test_main to avoid passing over testing

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    // Now that we've run all our tests, quit from QEMU
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("testing harness... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    println!("Hello World, this is {}: a basic operating system for learning.", "NUGGET");
    // panic!("Oops! Something went terribly wrong. Please restart the machine.");

    #[cfg(test)]
    test_main(); // Run tests conditionally in testing contexts

    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
