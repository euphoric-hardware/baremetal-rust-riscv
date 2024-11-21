#![no_main]
#![no_std]

use core::fmt::Write;
use core::writeln;
use riscv::register;
// use riscv_rust_baremetal::benchmark;

use riscv_rust_baremetal::htif::HostFile;

pub fn run_median() {
    let mut result: [i32; 400] = [0; 400];
    median(DATA_SIZE, &INPUT_DATA, &mut result);
    let x = register::mcycle::read();
    writeln!(HostFile::stdout(), "{}", x).unwrap();

    // Verifiy the data
    for i in 0..DATA_SIZE {
        if result[i] != VERIFY_DATA[i] {
            panic!("BAD {} {} {}", i, result[i], VERIFY_DATA[i]);
        }
    }
}

pub fn median(n: usize, input: &[i32], results: &mut [i32]) {
    // Zero the ends
    results[0] = 0;
    results[n - 1] = 0;

    // Do the filter
    for i in 1..(n - 1) {
        let a = input[i - 1];
        let b = input[i];
        let c = input[i + 1];

        results[i] = if a < b {
            if b < c {
                b
            } else if c < a {
                a
            } else {
                c
            }
        } else {
            if a < c {
                a
            } else if c < b {
                b
            } else {
                c
            }
        };
    }
}

const DATA_SIZE: usize = 400;

const INPUT_DATA: [i32; DATA_SIZE] = core::include!("data.in");

const VERIFY_DATA: [i32; DATA_SIZE] = core::include!("verify.in");
#[no_mangle]
pub extern "C" fn _init() {
    main();

    loop{}
}

fn main() {
    run_median();

    // TODO: exit properly

    // Panic!
    // let x = 1 / (x-y*2);

    // unsafe { let src = z as *const (); core::ptr::read_volatile(src) }
    // for c in b"Hello from Rust!".iter() {
    //     unsafe {
    //         *uart = *c as u8;
    //     }
    // }
}
