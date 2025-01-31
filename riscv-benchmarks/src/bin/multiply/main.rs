#![no_main]
#![no_std]

use data::*;
use riscv_benchmarks::*;
use riscv_rt::entry;

mod data;

#[entry]
fn main() -> ! {
    let mut result = [0; DATA_SIZE];
    let benchmark_data = start_benchmark();
    for i in 0..DATA_SIZE {
        result[i] = multiply(INPUT_DATA1[i], INPUT_DATA2[i]);
    }
    verify_and_end_benchmark(&result, &VERIFY_DATA, benchmark_data);
}

fn multiply(mut x: i64, mut y: i64) -> i64 {
    let mut result = 0;

    // while x != 0 {
    for _i in 0..32 {
        if (x & 0x1) == 1 {
            result += y;
        }
        x >>= 1;
        y <<= 1;
    }

    result
}
