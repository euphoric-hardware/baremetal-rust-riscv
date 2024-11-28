#![no_main]
#![no_std]

mod data;

use data::*;
use riscv_benchmarks::*;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let mut result = INPUT_DATA.clone();
    let benchmark_data = start_benchmark();
    result.sort_unstable();
    verify_and_end_benchmark(&result, &VERIFY_DATA, benchmark_data);
}
