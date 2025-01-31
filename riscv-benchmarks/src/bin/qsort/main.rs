// Generated with chatGPT based on qsort from riscv-tests
// https://chatgpt.com/share/679c49c3-1308-8004-8138-f620d1b203e1

#![no_main]
#![no_std]

use htif::HostFile;
use sort_data::*;
use riscv_benchmarks::*;
use riscv_rt::entry;

use core::cmp::Ordering;
use core::mem::swap;
use core::fmt::Write;

const INSERTION_THRESHOLD: usize = 16;
const NSTACK: usize = 64;

type Type = i64; // Change this to desired type

fn swap_if_greater(arr: &mut [Type], i: usize, j: usize) {
    if arr[i] > arr[j] {
        arr.swap(i, j);
    }
}

fn insertion_sort(arr: &mut [Type]) {
    for i in 1..arr.len() {
        let value = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > value {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = value;
    }
}

fn selection_sort(arr: &mut [Type]) {
    let len = arr.len();
    for i in 0..len - 1 {
        for j in i + 1..len {
            swap_if_greater(arr, i, j);
        }
    }
}

// Quicksort function
pub fn sort(arr: &mut [Type]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mut stack = [(0, 0); NSTACK];
    let mut stackp = 0;

    let (mut left, mut right): (usize, usize) = (0, len - 1);

    loop {
        // TODO: inspect and verify
        // writeln!(HostFile::stdout(), "{} {}", left, right);
        // Insertion sort when subarray is small enough
        if right as i64 - (left as i64) < INSERTION_THRESHOLD as i64 {
            insertion_sort(&mut arr[left..=right]);
            if stackp == 0 {
                break;
            }
            stackp -= 1;
            right = stack[stackp].1;
            left = stack[stackp].0;
        } else {
            // Choose median of left, center, and right elements as partitioning element
            // TODO: fix me (l+r)/2-1
            // let mid = (left+right) / 2-1;
            let mid = left + (right - left) / 2;
            arr.swap(mid, left);
            swap_if_greater(arr, left, right);
            swap_if_greater(arr, left, right - 1);
            swap_if_greater(arr, right - 1, right);

            // Partitioning element
            let pivot = arr[left];
            let mut i = left + 1;
            let mut j = right;

            // Partitioning loop
            loop {
                while i <= right && arr[i] < pivot {
                    i += 1;
                }
                while j > left && arr[j] > pivot {
                    j -= 1;
                }
                if i >= j {
                    break;
                }
                arr.swap(i, j);
                // i += 1;
                // j -= 1;
            }

            // Insert partitioning element
            arr.swap(left, j);
            
            // Push pointers to larger subarray on stack, process smaller subarray immediately
            if right - i >= j - left {
                stack[stackp] = (i, right);
                stackp += 1;
                right = j - 1;
            } else {
                stack[stackp] = (left, j - 1);
                stackp += 1;
                left = i;
            }
        }
    }
}

#[entry]
fn main() -> ! {
    let mut result = INPUT_DATA.clone();
    let benchmark_data = start_benchmark();
    sort(&mut result);
    verify_and_end_benchmark(&result, &VERIFY_DATA, benchmark_data);
}
