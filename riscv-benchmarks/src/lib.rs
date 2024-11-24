#![no_main]
#![no_std]

use core::fmt::Write;
use htif::HostFile;
use riscv::register;

// Unsafe because there is no synchronization
static mut MCYCLE: usize = 0;
pub unsafe fn start_benchmark() {
    let x = register::mcycle::read();
    MCYCLE = x;
}

pub unsafe fn end_benchmark() -> ! {
    let x = register::mcycle::read();
    writeln!(HostFile::stdout(), "{}", x - MCYCLE).unwrap();
    // TODO: exit properly
    loop {}
}

pub unsafe fn verify_and_end_benchmark<T: core::fmt::Display + core::cmp::PartialEq>(
    result: &[T],
    expected: &[T],
) -> ! {
    let x = register::mcycle::read();
    writeln!(HostFile::stdout(), "{}", x - MCYCLE).unwrap();
    for i in 0..result.len() {
        if result[i] != expected[i] {
            panic!("BAD {} {} {}", i, result[i], expected[i]);
        }
    }
    // TODO: exit properly
    loop {}
}
