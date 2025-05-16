#![no_main]
#![no_std]

use riscv_benchmarks::*;
use riscv_rt::entry;

use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;
use core::fmt::Write;
use core::hint::black_box;
use core::iter;

const DATA_SIZE: usize = 1000; // total size: 5345280;
const SCALE_FACTOR: usize = 10;

const DATA: &[u8] = include_bytes!("xml");

#[entry]
fn main() -> ! {
    init_heap();
    let data = &DATA[..DATA_SIZE];
    for _ in 0..SCALE_FACTOR {
        let benchmark_data = start_benchmark();
        let compressed_data = black_box(compress_to_vec(black_box(data), 7));
        let decompressed_data = black_box(decompress_to_vec(black_box(compressed_data.as_slice()))).unwrap();
        print_benchmark_data(benchmark_data);
        verify_data(decompressed_data.as_slice(), data);

        writeln!(htif::HostFile::stdout(), "Compressed size: {}", compressed_data.len()).unwrap();
    }
    // verify_and_end_benchmark(&[1], &[1], benchmark_data);
    exit();
}
