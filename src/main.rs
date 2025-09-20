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

    #[cfg(test)]
    test_main();

    println!("NO CRASH ;)");

    luki_os::hlt_loop();
}

// function is called withing normal operation should a panic occur
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    luki_os::hlt_loop();
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
