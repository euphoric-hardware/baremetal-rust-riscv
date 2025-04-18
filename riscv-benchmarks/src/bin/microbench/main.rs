#![no_main]
#![no_std]

use bencher::Bencher;
use riscv_benchmarks::{exit, init_heap};
use riscv_rt::entry;

mod bencher;
mod vec_bench;
use vec_bench::*;

const SCALE_FACTOR: usize = 78;

#[entry]
fn main() -> ! {
    init_heap();

    let mut b = Bencher{};

    binary_search_l1(&mut b);
    binary_search_l2(&mut b);
    binary_search_l3(&mut b);
    binary_search_l1_with_dups(&mut b);
    binary_search_l2_with_dups(&mut b);
    binary_search_l3_with_dups(&mut b);
    binary_search_l1_worst_case(&mut b);
    binary_search_l2_worst_case(&mut b);
    binary_search_l3_worst_case(&mut b);

    rotate_u8(&mut b);
    rotate_rgb(&mut b);
    rotate_usize(&mut b);
    rotate_16_usize_4(&mut b);
    rotate_16_usize_5(&mut b);
    rotate_64_usize_4(&mut b);
    rotate_64_usize_5(&mut b);

    swap_with_slice_u8_30(&mut b);
    swap_with_slice_u8_3000(&mut b);
    swap_with_slice_rgb_30(&mut b);
    swap_with_slice_rgb_3000(&mut b);
    swap_with_slice_usize_30(&mut b);
    swap_with_slice_usize_3000(&mut b);
    swap_with_slice_4x_usize_30(&mut b);
    swap_with_slice_4x_usize_3000(&mut b);
    swap_with_slice_5x_usize_30(&mut b);
    swap_with_slice_5x_usize_3000(&mut b);

    fill_byte_sized(&mut b);
    fold_to_last(&mut b);

    exit();
}

