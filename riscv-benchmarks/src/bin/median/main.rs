#![no_main]
#![no_std]

use riscv_benchmarks::*;
use riscv_rt::entry;

const DATA_SIZE: usize = 400;

const INPUT_DATA: [i32; DATA_SIZE] = core::include!("data.in");

const VERIFY_DATA: [i32; DATA_SIZE] = core::include!("verify.in");

#[entry]
fn run_median() -> ! {
    let mut result: [i32; 400] = [0; 400];
    unsafe { start_benchmark(); };
    median(DATA_SIZE, &INPUT_DATA, &mut result);
    unsafe { verify_and_end_benchmark(&result, &VERIFY_DATA); };
}

fn median(n: usize, input: &[i32], results: &mut [i32]) {
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
