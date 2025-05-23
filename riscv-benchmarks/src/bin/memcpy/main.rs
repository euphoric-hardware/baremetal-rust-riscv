#![no_main]
#![no_std]

mod data;

use core::array;

use data::*;
use htif::exit;
use riscv_benchmarks::*;
use riscv_rt::entry;

fn copy1(dest: &mut [i64], src: &[i64]) {
    for i in (0..src.len()-7).step_by(8) {
        dest[i] = src[i];
        dest[i + 1] = src[i + 1];
        dest[i + 2] = src[i + 2];
        dest[i + 3] = src[i + 3];
        dest[i + 4] = src[i + 4];
        dest[i + 5] = src[i + 5];
        dest[i + 6] = src[i + 6];
        dest[i + 7] = src[i + 7];
    }
    for i in (src.len() / 8 * 8)..(src.len()) {
        dest[i] = src[i];
    }
}

fn copy2(dest: &mut [i64], src: &[i64]) {
    let mut d = dest.iter_mut();
    let mut s = src.iter();
    while d.len() >= 8 {
        *d.next().unwrap() = *s.next().unwrap();
        *d.next().unwrap() = *s.next().unwrap();
        *d.next().unwrap() = *s.next().unwrap();
        *d.next().unwrap() = *s.next().unwrap();
        *d.next().unwrap() = *s.next().unwrap();
        *d.next().unwrap() = *s.next().unwrap();
        *d.next().unwrap() = *s.next().unwrap();
        *d.next().unwrap() = *s.next().unwrap();
    }
    for (d_rest, s_rest) in d.zip(s) {
        *d_rest = *s_rest;
    }
}

#[entry]
fn main() -> ! {
    // let benchmark_data = start_benchmark();

    let mut result: [i64; DATA_SIZE] = [0; DATA_SIZE];
    let benchmark_data = start_benchmark();
    copy2(&mut result, &INPUT_DATA);

    // calls the memcpy generated by rust; see https://doc.rust-lang.org/core/
    // let result = INPUT_DATA;

    // let mut result = [42; DATA_SIZE];
    // copy(&mut result, &INPUT_DATA);
    verify_and_end_benchmark(&result, &INPUT_DATA, benchmark_data);
}
