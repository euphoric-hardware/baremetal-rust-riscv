#![no_main]
#![no_std]


#[cfg(feature = "std")]
extern crate std;

use core::{fmt::Write, panic::PanicInfo};
use htif::{panic_htif_print, HostFile};
use riscv::register;

pub mod sort_data;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
const BENCHMARK_DATA_COUNT: usize = 2;

#[cfg(any(target_arch = "x86_64"))]
const BENCHMARK_DATA_COUNT: usize = 1;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
const BENCHMARK_CSR: [(fn() -> usize, &str); BENCHMARK_DATA_COUNT] = [
    (register::mcycle::read, "mcycle"),
    (register::minstret::read, "minstret"),
];

#[cfg(any(target_arch = "x86_64"))]
const BENCHMARK_CSR: [(fn() -> usize, &str); BENCHMARK_DATA_COUNT] = [
    (|| unsafe { core::arch::x86_64::_rdtsc() } as usize, "rdtsc")
];

pub type BenchmarkData = [usize; BENCHMARK_DATA_COUNT];

pub fn start_benchmark() -> BenchmarkData {
    let mut ret = [0; BENCHMARK_DATA_COUNT];
    for (i, csr_op) in BENCHMARK_CSR.iter().enumerate() {
        ret[i] = unsafe { csr_op.0() }
    }
    ret
}

#[cfg(not(feature = "std"))]
pub fn print_benchmark_data(mut data: BenchmarkData) {
    for (i, csr_op) in BENCHMARK_CSR.iter().enumerate() {
        data[i] = csr_op.0() - data[i];
    }
    for (i, csr_op) in BENCHMARK_CSR.iter().enumerate() {
        writeln!(HostFile::stdout(), "{}: {}", csr_op.1, data[i]).unwrap();
    }
}

#[cfg(feature = "std")]
pub fn print_benchmark_data(mut data: BenchmarkData) {
    for (i, csr_op) in BENCHMARK_CSR.iter().enumerate() {
        data[i] = csr_op.0() - data[i];
    }
    for (i, csr_op) in BENCHMARK_CSR.iter().enumerate() {
        std::println!("{}: {}", csr_op.1, data[i]);
    }
}

#[cfg(not(feature = "std"))]
pub fn exit() -> ! {
    htif::exit(0);
}

#[cfg(feature = "std")]
pub fn exit() -> ! {
    std::process::exit(0);
}

pub fn end_benchmark(benchmark_data: BenchmarkData) -> ! {
    print_benchmark_data(benchmark_data);
    exit();
}

// Trait alias
// trait Data = core::fmt::Display + core::cmp::PartialEq;

pub fn verify_data<T: core::fmt::Debug + core::cmp::PartialEq>(result: &[T], expected: &[T]) {
    for i in 0..result.len() {
        if result[i] != expected[i] {
            panic!(
                "\tVerification failed; array different at index {}\n\
                \texpected: {:?}\n\
                \tactual: {:?}\n",
                i, expected[i], result[i]
            );
        }
    }
    assert!(
        result.len() == expected.len(),
        "\tVerification failed; array length differs\n\
                \texpected len: {}\n\
                \tactual len: {}\n",
        expected.len(),
        result.len()
    );
}

pub fn verify_and_end_benchmark<T: core::fmt::Debug + core::cmp::PartialEq>(
    result: &[T],
    expected: &[T],
    benchmark_data: BenchmarkData,
) -> ! {
    print_benchmark_data(benchmark_data);
    verify_data(result, expected);
    exit();
}

#[cfg(not(feature = "std"))]
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    panic_htif_print::panic(_info);
}

// Allocator
// Uses critical-section-single-hart provided by the riscv crate.
//
// TODO: multi hart

use embedded_alloc::LlffHeap as Heap;

#[cfg(not(feature = "std"))]
// #[global_allocator]
// static HEAP: Heap = Heap::empty();

pub fn init_heap() {
    // Initialize the allocator BEFORE you use it
    #[cfg(not(feature = "std"))]
    {
        // use core::mem::MaybeUninit;
        // const HEAP_SIZE: usize = 12800*1024;
        // static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        // unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
}
