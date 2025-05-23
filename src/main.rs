#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use rusty_os::test_trait::Tests;
use rusty_os::{QemuExitCode, exit_qemu, println, test_runner};
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    /*
        * NOTE: From my understanding
        * the index starts from 0 so
        * the 0xb8000 should be the first u8 of Hello
        * 0 * 2 + 1 = 1, to put the color code infront
        * then continue on from there

        for (i,&byte) in HELLO.iter().enumerate(){
            let step: isize = i as isize * 2;
            unsafe {
                *vga_buffer.offset(step) = byte;
                *vga_buffer.offset(step + 1) = 0xb;

            };
        }
    */

    println!("UnitOs");

    rusty_os::init();

    // NOTE: Breakpoint exception
    interrupts::int3();

    println!("here");

    // Running all tests
    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
