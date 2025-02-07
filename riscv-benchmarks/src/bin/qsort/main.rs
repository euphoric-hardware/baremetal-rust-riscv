// Generated with chatGPT based on qsort from riscv-tests
// https://chatgpt.com/share/679c49c3-1308-8004-8138-f620d1b203e1

#![no_main]
#![no_std]

use sort_data::*;
use riscv_benchmarks::*;
use riscv_rt::entry;

const INSERTION_THRESHOLD: usize = 10;
const NSTACK: usize = 32;

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

fn insertion_sort_unsafe(arr: &mut [Type]) {
    unsafe {
        for i in 1..arr.len() {
            let value = *arr.get_unchecked(i);
            let mut j = i;
            while j > 0 && *arr.get_unchecked(j - 1) > value {
                *arr.get_unchecked_mut(j) = *arr.get_unchecked(j - 1);
                j -= 1;
            }
            *arr.get_unchecked_mut(j) = value;
        }
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
    let mut stack = [(0, 0); NSTACK];
    let mut stackp = 0;

    let (mut left, mut right): (usize, usize) = (1, len);

    loop {
        // Used to verify that algorithm matches C implementation
        // writeln!(HostFile::stdout(), "{} {}", left, right);
        // Insertion sort when subarray is small enough
        if right as i64 - (left as i64) < INSERTION_THRESHOLD as i64 {
            if right >= left-1 {
                insertion_sort(&mut arr[left-1..right]);
            }
            if stackp == 0 {
                break;
            }
            // pop the stack
            stackp -= 1;
            right = stack[stackp].1;
            left = stack[stackp].0;
        } else {
            // Choose median of left, center, and right elements
            // as partitioning element. Also rearrange so that
            // arr[l-1] <= arr[l] <= arr[ir-1].

            let mid = (left+right) / 2-1;
            arr.swap(mid, left);
            swap_if_greater(arr, left - 1, right-1);
            swap_if_greater(arr, left, right-1);
            swap_if_greater(arr, left - 1, left);

            // Partitioning element
            let pivot = arr[left];
            let mut i = left + 1;
            let mut j = right;

            // Partitioning loop
            loop {
                // Find element > pivot
                loop {
                    i += 1;
                    // unsafe {
                    //     if arr.get_unchecked(i-1) >= &pivot { break; }
                    // }
                    if arr[i-1] >= pivot { break; }
                }
                // Find element < pivot
                loop {
                    j -= 1;
                    // unsafe {
                    //     if arr.get_unchecked(j-1) <= &pivot { break; }
                    // }
                    if arr[j-1] <= pivot { break; }
                }
                if j < i {
                    break;
                }
                arr.swap(i-1, j-1);
            }

            // Insert partitioning element
            arr[left] = arr[j-1];
            arr[j-1] = pivot;

            // Push pointers to larger subarray on stack, process smaller subarray immediately
            if right as i32 - i as i32 + 1 >= j as i32 - left as i32 {
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
