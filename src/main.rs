#![feature(portable_simd)]

use std::simd::{u64x64, u64x8, u8x64, Simd};
use std::simd::num::SimdUint;
use crate::benchmark::Bencher;

mod benchmark;
mod aes;

fn main() {
    let bencher = Bencher::new();

    bencher.benchmark(|| 0, |x: u64| x).report_as("x");
    bencher.benchmark(|| 0, |x: u64| x + 1).report_as("x + 1");
    bencher.benchmark(|| 0, |x: u64| (x + 1) >> 1).report_as("(x + 1) >> 1");
    bencher.benchmark(|| 0, |x: u64| x / 3).report_as("x / 3");
    bencher.benchmark(|| 0, |x: u64| x / 5).report_as("x / 5");
    bencher.benchmark(|| 0, |x: u64| x * 0).report_as("x * 0");
    bencher.benchmark(|| 0, |x: u64| x * 1).report_as("x * 1");
    bencher.benchmark(|| 0, |x: u64| x * 2).report_as("x * 2");
    bencher.benchmark(|| 0, |x: u64| x * 3).report_as("x * 3");
    bencher.benchmark(|| 0, |x: u64| x * 4).report_as("x * 4");
    bencher.benchmark(|| 0, |x: u64| x * 5).report_as("x * 5");
    bencher.benchmark(|| 0, |x: u64| x * 6).report_as("x * 6");
    bencher.benchmark(|| 0, |x: u64| x * 7).report_as("x * 7");
    bencher.benchmark(|| 0, |x: u64| x * 8).report_as("x * 8");
    bencher.benchmark(|| 0, |x: u64| x * x).report_as("x * x");
    bencher.benchmark(|| 0, |x: u64| x % 3).report_as("x % 3");
    bencher.benchmark(|| 0, |x: u32| x.count_zeros()).report_as("x.count_zeros()");
    bencher.benchmark(|| 0, |x: u32| x.count_ones()).report_as("x.count_ones()");
    bencher.benchmark(|| 0, |x: u32| x.leading_zeros()).report_as("x.leading_zeros()");
    bencher.benchmark(|| 0, |x: u32| x.leading_ones()).report_as("x.leading_ones()");
    bencher.benchmark(|| 0, |x: u32| x.trailing_zeros()).report_as("x.trailing_zeros()");
    bencher.benchmark(|| 0, |x: u32| x.trailing_ones()).report_as("x.trailing_ones()");
    bencher.benchmark(|| 0, |x: u128| x + 1).report_as("x + 1 (u128)");
    bencher.benchmark(|| u8x64::splat(0), |x: u8x64| x + u8x64::splat(1)).report_as("x + 1 (u8x64)");
    bencher.benchmark(|| u64x64::splat(0), |x: u64x64| x + u64x64::splat(1)).report_as("x + 1 (u64x64)");
    bencher.benchmark(|| u64x64::splat(0), |x: u64x64| x + x).report_as("x + x (u64x64)");
    bencher.benchmark(|| u64x64::splat(3), |x: u64x64| x * x).report_as("x * x (u64x64)");
    bencher.benchmark(|| u64x64::splat(0), |x: u64x64| Simd::splat(x.reduce_sum())).report_as("splat(x.reduce_sum()) (u64x64)");
    bencher.benchmark(|| u64x8::splat(0), |x: u64x8| Simd::splat(x.reduce_sum())).report_as("splat(x.reduce_sum()) (u64x8)");
    bencher.benchmark(|| u64x8::splat(0), |x: u64x8| {
        let x = x.to_array();
        Simd::splat(x[0] + x[1] + x[2] + x[3] + x[4] + x[5] + x[6] + x[7])
    }).report_as("splat(x.iter().sum()) (u64x8)");
    // assert!((benchmark(|| 0, |x: u64| x * 4).min / 1).abs_diff(cycle_picos) <= 2);
    // assert!((benchmark(|| 0, |x: u64| x * 5).min / 2).abs_diff(cycle_picos) <= 2);
    // assert!((benchmark(|| 0, |x: u64| x * 6).min / 3).abs_diff(cycle_picos) <= 2);
    // assert!((benchmark(|| 0, |x: u64| x * x).min / 3).abs_diff(cycle_picos) <= 2);
    // assert!((benchmark(|| 0, |x: u64| x / 3).min / 4).abs_diff(cycle_picos) <= 2);
    // assert!((benchmark(|| 0, |x: u64| x % 3).min / 7).abs_diff(cycle_picos) <= 2);

    // aes::main();
}
