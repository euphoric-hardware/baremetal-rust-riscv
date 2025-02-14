#![no_main]
#![no_std]

use sort_data::*;
use riscv_benchmarks::*;
use riscv_rt::entry;
use radsort;
// use rdx;

#[entry]
fn main() -> ! {
    let mut result = INPUT_DATA.clone();
    let benchmark_data = start_benchmark();
    radsort::sort(&mut result);
    verify_and_end_benchmark(&result, &VERIFY_DATA, benchmark_data);
}
