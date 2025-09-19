#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(luki_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use luki_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("[+] SETUP - CAUSING INTERRUPT");

    luki_os::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("NO CRASH ;)");

    loop {}
}

// function is called withing normal operation should a panic occur
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    luki_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
