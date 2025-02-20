#![no_main]
#![no_std]

use num::FromPrimitive;
use riscv_benchmarks::*;
use riscv_rt::entry;

use modulo_n_tools::montgomery::*;
use core::fmt::Write;

/// Computes `a * b * R^-1 mod m` using Montgomery multiplication.
/// Assumes that `m` is an odd modulus and `m' = -m^-1 mod 2^64` is precomputed.
fn montgomery_mul(a: u64, b: u64, m: u64, m_prime: u64) -> u64 {
    let t = (a as u128) * (b as u128);
    let m_0 = t as u64; // lower 64 bits
    let u = (m_0.wrapping_mul(m_prime)) as u64;
    let t = (m_0 as u128 + (u as u128) * (m as u128)) >> 64;

    // Ensure result is in range
    let mut r = t as u64;
    if r >= m {
        r -= m;
    }
    r
}

/// Computes `m' = -m^-1 mod 2^64`, required for Montgomery multiplication.
fn montgomery_inverse(m: u64) -> u64 {
    let mut y = 1u64;
    for _ in 0..6 {
        y = y.wrapping_mul(2u64.wrapping_sub(m.wrapping_mul(y)));
    }
    y.wrapping_neg()
}

/// Converts a value to Montgomery form: `x * R mod m`, where R = 2^64 mod m.
fn to_montgomery(x: u64, m: u64, r_mod_m: u64) -> u64 {
    montgomery_mul(x, r_mod_m, m, montgomery_inverse(m))
}

/// Converts a Montgomery-form value back to normal form: `x * R⁻¹ mod m`.
fn from_montgomery(x: u64, m: u64, m_prime: u64) -> u64 {
    montgomery_mul(x, 1, m, m_prime)
}
#[entry]
fn main() -> ! {
    let benchmark_data = start_benchmark();
    let in_m = 0xfae849273928f89f;
    let in_b:u64 = 0x14736defb9330573;
    let in_a = 0x0549372187237fef;
    for _ in 0..423 {
        let m_prime = montgomery_inverse(in_m);
        let r_mod_m = ((1u128 << 64) % in_m as u128) as u64;
        let x1 = to_montgomery(in_a, in_m, r_mod_m);
        let x2 = to_montgomery(in_b, in_m, r_mod_m);
        let x = montgomery_mul(x1, x2, in_m, m_prime);
        let x = from_montgomery(x, in_m, m_prime);
        // let x = montgomery_mul(x, x, in_m, m_prime);
        // let x = montgomery_mul(x, x, in_m, m_prime);

        let r2 = multiply(in_a, in_b, in_m);
        // let r2 = multiply(r2, r2, in_m);
        // let r2 = multiply(r2, r2, in_m);
        writeln!(htif::HostFile::stdout(), "{}, {}", r2, x);
    }
    verify_and_end_benchmark(&[1], &[1], benchmark_data);
}

fn multiply(x: u64, y: u64, m: u64) -> u64 {
    return ((x as u128) * (y as u128) % (m as u128)) as u64;
}
