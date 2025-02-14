#![no_main]
#![no_std]

use riscv_benchmarks::*;
use riscv_rt::entry;

use modulo_n_tools::montgomery::*;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let benchmark_data = start_benchmark();
    let in_m = 0xfae849273928f89f;
    let in_b = 0x14736defb9330573;
    let in_a = 0x0549372187237fef;
    for _ in 0..423 {
        let m = Montgomery64::new(in_m);
        let x = m.powmod(in_a, in_b);
        // writeln!(htif::HostFile::stdout(), "{}, {}", multiply(in_a, in_b) % (in_m as u128), x);
    }
    verify_and_end_benchmark(&[1], &[1], benchmark_data);
}

fn multiply(x: u64, y: u64) -> u128 {
    return (x as u128) * (y as u128)
}
