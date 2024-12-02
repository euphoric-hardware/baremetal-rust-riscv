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
    vvadd(DATA_SIZE, &INPUT_DATA1, &INPUT_DATA2, &mut result);
    verify_data(&result, &VERIFY_DATA);
    print_benchmark_data(benchmark_data);

    let benchmark_data = start_benchmark();
    vvadd_unrolled(DATA_SIZE, &INPUT_DATA1, &INPUT_DATA2, &mut result);
    verify_and_end_benchmark(&result, &VERIFY_DATA, benchmark_data);
}

fn vvadd(n: usize, input1: &[i64], input2: &[i64], result: &mut [i64]) {
    for i in 0..n {
        result[i] = input1[i] + input2[i];
    }
    return
}

fn vvadd_unrolled(n: usize, input1: &[i64], input2: &[i64], result: &mut [i64]) {
    for i in (0..DATA_SIZE-7).step_by(8) {
        result[i] = input1[i] + input2[i];
        result[i+1] = input1[i+1] + input2[i+1];
        result[i+2] = input1[i+2] + input2[i+2];
        result[i+3] = input1[i+3] + input2[i+3];
        result[i+4] = input1[i+4] + input2[i+4];
        result[i+5] = input1[i+5] + input2[i+5];
        result[i+6] = input1[i+6] + input2[i+6];
        result[i+7] = input1[i+7] + input2[i+7];
    }
    for i in (DATA_SIZE/8*8)..(DATA_SIZE) {
        result[i] = input1[i] + input2[i];
    }
    return
}
