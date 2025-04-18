use core::hint::black_box;

use riscv_benchmarks::*;
use riscv_rt::entry;

/// Manager of the benchmarking runs.
///
/// Modified from https://doc.rust-lang.org/test/bench/struct.Bencher.html
#[derive(Clone)]
pub struct Bencher {
    // mode: BenchMode,
    // summary: Option<stats::Summary>,
    // pub bytes: u64,
}

impl Bencher {
    /// Callback for benchmark functions to run in their body.
    pub fn iter<T, F>(&mut self, mut inner: F)
    where
        F: FnMut() -> T,
    {
        let benchmark_data = start_benchmark();
        for _ in 0..100 {
            black_box(inner());
        }
        print_benchmark_data(benchmark_data);
    }

    // pub fn bench<F>(&mut self, mut f: F) -> Result<Option<stats::Summary>, String>
    // where
    //     F: FnMut(&mut Bencher) -> Result<(), String>,
    // {
    //     let result = f(self);
    //     result.map(|_| self.summary)
    // }
}
