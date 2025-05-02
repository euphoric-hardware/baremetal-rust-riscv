#![no_main]
#![no_std]

use riscv_benchmarks::*;
use riscv_rt::entry;
use md5::{Md5, Digest};
use core::fmt::Write;

const MSG_SIZE: usize = 10;
const SCALE_FACTOR: usize = 1;

#[entry]
fn main() -> ! {
    let mut data: [u8; MSG_SIZE] = [0; MSG_SIZE];
    let benchmark_data = start_benchmark();
    for _ in 0..SCALE_FACTOR {
        for i in 0..MSG_SIZE {
            data[i] = i as u8;
        }
        let mut hasher = Md5::new();
        hasher.update(data);
        let hash = hasher.finalize();
        // for h in hash {
        //     write!(htif::HostFile::stdout(), "{:02x?}", h).unwrap();
        // }
        // writeln!(htif::HostFile::stdout(), "").unwrap();
    }
    verify_and_end_benchmark(&[1], &[1], benchmark_data);
}
