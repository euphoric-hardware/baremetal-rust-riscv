#![no_main]
#![no_std]

use riscv_benchmarks::*;
use riscv_rt::entry;
use md5::{Md5, Digest};
use core::fmt::Write;
use core::hint::black_box;

const MSG_SIZE: usize = 10;
const SCALE_FACTOR: usize = 10;

fn compute_hash(data: [u8; MSG_SIZE]) -> aes::cipher::generic_array::GenericArray<u8, aes::cipher::typenum::UInt<aes::cipher::typenum::UInt<aes::cipher::typenum::UInt<aes::cipher::typenum::UInt<aes::cipher::typenum::UInt<aes::cipher::typenum::UTerm, aes::cipher::consts::B1>, aes::cipher::consts::B0>, aes::cipher::consts::B0>, aes::cipher::consts::B0>, aes::cipher::consts::B0>> {
    let mut hasher = Md5::new();
    hasher.update(data);
    hasher.finalize()
}

#[entry]
fn main() -> ! {
    let mut data: [u8; MSG_SIZE] = [0; MSG_SIZE];
    for i in 0..MSG_SIZE {
        data[i] = i as u8;
    }
    for _ in 0..SCALE_FACTOR {
        let benchmark_data = start_benchmark();
        black_box(compute_hash(black_box(data)));
        print_benchmark_data(benchmark_data);
        // for h in hash {
        //     write!(htif::HostFile::stdout(), "{:02x?}", h).unwrap();
        // }
        // writeln!(htif::HostFile::stdout(), "").unwrap();
    }
    // verify_and_end_benchmark(&[1], &[1], benchmark_data);
    exit();
}
