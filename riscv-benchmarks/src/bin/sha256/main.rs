#![no_main]
#![no_std]

use data::*;
use riscv_benchmarks::*;
use riscv_rt::entry;
use sha2::{Sha256, Digest};

mod data;

const SCALE_FACTOR: usize = 475;

// Very dumb AES, encrypts each block directly, no mode of operation

#[entry]
fn main() -> ! {
    // block_size() is not a const fn
    let benchmark_data = start_benchmark();
    
    for _ in 0..SCALE_FACTOR {
        let mut hasher = Sha256::new();
        hasher.update(MSG);
        let hash = hasher.finalize();
        verify_data(&hash, &EXPECTED_HASH);
    }

    end_benchmark(benchmark_data);
}

