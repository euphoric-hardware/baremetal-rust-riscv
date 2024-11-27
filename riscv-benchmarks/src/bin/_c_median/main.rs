#![no_main]
#![no_std]

mod bindings;

use riscv_benchmarks::*;
use riscv_rt::entry;

const DATA_SIZE: usize = 400;

const INPUT_DATA: [i32; DATA_SIZE] = core::include!("../median/data.in");

const VERIFY_DATA: [i32; DATA_SIZE] = core::include!("../median/verify.in");

#[entry]
fn run_median() -> ! {
    let mut result: [i32; 400] = [0; 400];
    let benchmark_data = start_benchmark();
    unsafe { bindings::median(DATA_SIZE.try_into().unwrap(), INPUT_DATA.as_mut_ptr(), result.as_mut_ptr()); }
    verify_and_end_benchmark(&result, &VERIFY_DATA, benchmark_data);
}
