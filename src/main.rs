#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    // do something here
    loop { }
}
