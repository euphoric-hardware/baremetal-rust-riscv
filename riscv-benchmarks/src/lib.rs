#![no_main]
#![no_std]

use core::fmt::Write;
use htif::HostFile;
use riscv::register;

const BENCHMARK_DATA_COUNT: usize = 2;
const BENCHMARK_CSR: [(fn()->usize, &str); BENCHMARK_DATA_COUNT] = [
    (register::mcycle::read, "mcycle"),
    (register::minstret::read, "minstret"),
];

pub type BenchmarkData = [usize; BENCHMARK_DATA_COUNT];

pub fn start_benchmark() -> BenchmarkData {
    let mut ret = [0; BENCHMARK_DATA_COUNT];
    for (i, csr_op) in BENCHMARK_CSR.iter().enumerate() {
        ret[i] = csr_op.0();
    }
    ret
}

fn print_benchmark_data(mut data: BenchmarkData) {
    for (i, csr_op) in BENCHMARK_CSR.iter().enumerate() {
        data[i] = csr_op.0() - data[i];
    }
    for (i, csr_op) in BENCHMARK_CSR.iter().enumerate() {
        writeln!(HostFile::stdout(), "{}: {}", csr_op.1, data[i]).unwrap();
    }
}

pub fn end_benchmark(benchmark_data: BenchmarkData) -> ! {
    print_benchmark_data(benchmark_data);
    htif::exit(0);
}

pub fn verify_and_end_benchmark<T: core::fmt::Display + core::cmp::PartialEq>(
    result: &[T],
    expected: &[T],
    benchmark_data: BenchmarkData,
) -> ! {
    print_benchmark_data(benchmark_data);
    for i in 0..result.len() {
        if result[i] != expected[i] {
            panic!("BAD {} {} {}", i, result[i], expected[i]);
        }
    }
    htif::exit(0);
}
