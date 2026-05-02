#![allow(unused)]
#![feature(portable_simd)]

use crate::benchmark::{bench, PreciseDuration, PrecisionSampler};
use rand::random;
use std::simd::num::SimdUint;
use std::simd::Simd;

mod benchmark;

fn main() {
    let sampler = PrecisionSampler::with_defaults(PreciseDuration::zero());
    let baseline = bench(&sampler, || 0, |x| x);

    baseline.report_as("baseline");

    let cycle = bench(&sampler, || 0, |x| x + 1).with_baseline(&baseline);

    cycle.report_as("cycle");

    let cycle = cycle.average;

    let sampler = PrecisionSampler::with_defaults(PreciseDuration::from_picos(5));

    let scalar_mul = bench(&sampler, || 12341234, |x: u64| x ^ 151515)
        .with_baseline(&baseline)
        .average;

    loop {
        const N: usize = 64;

        bench(&sampler, || Simd::splat(12341234), |x: Simd<u64, N>| x ^ Simd::splat(151515))
            .with_baseline(&baseline)
            .with_unit(scalar_mul * N)
            .report();
    }

    bench(&sampler, || 0, |x: u64| 0)
        .with_baseline(&baseline)
        .report();
    bench(&sampler, || 0, |x: u64| x)
        .with_baseline(&baseline)
        .report();
    bench(&sampler, || 0, |x: u64| x + 1)
        .with_baseline(&baseline)
        .report();
    bench(&sampler, || 0, |x: u64| x % 3)
        .with_baseline(&baseline)
        .report();

    let sampler = PrecisionSampler::with_defaults(PreciseDuration::from_nanos(5));
    bench(
        &sampler,
        || random(),
        |mut x: [u64; 200]| {
            x.sort();
            random()
        },
    )
    .with_baseline(&baseline)
    .report();
    bench(
        &sampler,
        || random(),
        |mut x: [u64; 200]| {
            x.sort_unstable();
            random()
        },
    )
    .with_baseline(&baseline)
    .report();
    // bencher
    //     .benchmark_precise(
    //         PreciseDuration::from_femtos(1000),
    //         100,
    //         || 0,
    //         |x: u64| x % 3,
    //     )
    //     .report_as("x % 3");
    // bencher
    //     .benchmark_precise(
    //         PreciseDuration::from_femtos(1000),
    //         100,
    //         || 0,
    //         |x: u64| x + 3,
    //     )
    //     .report_as("x + 3");
    // bencher.benchmark(|| 0, |x: u64| x).report_as("x");
    // bencher.benchmark(|| 0, |x: u64| x + 1).report_as("x + 1");
    // bencher.benchmark(|| 0, |x: u64| (x + 1) >> 1).report_as("(x + 1) >> 1");
    // bencher.benchmark(|| 0, |x: u64| x / 3).report_as("x / 3");
    // bencher.benchmark(|| 0, |x: u64| x / 5).report_as("x / 5");
    // bencher.benchmark(|| 0, |x: u64| x * 0).report_as("x * 0");
    // bencher.benchmark(|| 0, |x: u64| x * 1).report_as("x * 1");
    // bencher.benchmark(|| 0, |x: u64| x * 2).report_as("x * 2");
    // bencher.benchmark(|| 0, |x: u64| x * 3).report_as("x * 3");
    // bencher.benchmark(|| 0, |x: u64| x * 4).report_as("x * 4");
    // bencher.benchmark(|| 0, |x: u64| x * 5).report_as("x * 5");
    // bencher.benchmark(|| 0, |x: u64| x * 6).report_as("x * 6");
    // bencher.benchmark(|| 0, |x: u64| x * 7).report_as("x * 7");
    // bencher.benchmark(|| 0, |x: u64| x * 8).report_as("x * 8");
    // bencher.benchmark(|| 0, |x: u64| x * x).report_as("x * x");
    // bencher.benchmark(|| 0, |x: u64| x % 3).report_as("x % 3");
    // println!("{:?}", bencher.cycles(|| 0, |x: u64| x % 3));
    // bencher.benchmark(|| 0, |x: u32| x.count_zeros()).report_as("x.count_zeros()");
    // bencher.benchmark(|| 0, |x: u32| x.count_ones()).report_as("x.count_ones()");
    // bencher.benchmark(|| 0, |x: u32| x.leading_zeros()).report_as("x.leading_zeros()");
    // bencher.benchmark(|| 0, |x: u32| x.leading_ones()).report_as("x.leading_ones()");
    // bencher.benchmark(|| 0, |x: u32| x.trailing_zeros()).report_as("x.trailing_zeros()");
    // bencher.benchmark(|| 0, |x: u32| x.trailing_ones()).report_as("x.trailing_ones()");
    // bencher.benchmark(|| 0, |x: u128| x + 1).report_as("x + 1 (u128)");
    // bencher.benchmark(|| u8x64::splat(0), |x: u8x64| x + u8x64::splat(1)).report_as("x + 1 (u8x64)");
    // bencher.benchmark(|| u64x64::splat(0), |x: u64x64| x + u64x64::splat(1)).report_as("x + 1 (u64x64)");
    // bencher.benchmark(|| u64x64::splat(0), |x: u64x64| x + x).report_as("x + x (u64x64)");
    // bencher.benchmark(|| u64x64::splat(3), |x: u64x64| x * x).report_as("x * x (u64x64)");
    // bencher.benchmark(|| u64x64::splat(0), |x: u64x64| Simd::splat(x.reduce_sum())).report_as("splat(x.reduce_sum()) (u64x64)");
    // bencher.benchmark(|| u64x8::splat(0), |x: u64x8| Simd::splat(x.reduce_sum())).report_as("splat(x.reduce_sum()) (u64x8)");
    // bencher.benchmark(|| u64x8::splat(0), |x: u64x8| {
    //     let x = x.to_array();
    //     Simd::splat(x[0] + x[1] + x[2] + x[3] + x[4] + x[5] + x[6] + x[7])
    // }).report_as("splat(x.iter().sum()) (u64x8)");
    // assert!((benchmark(|| 0, |x: u64| x * 4).min / 1).abs_diff(cycle_picos) <= 2);
    // assert!((benchmark(|| 0, |x: u64| x * 5).min / 2).abs_diff(cycle_picos) <= 2);
    // assert!((benchmark(|| 0, |x: u64| x * 6).min / 3).abs_diff(cycle_picos) <= 2);
    // assert!((benchmark(|| 0, |x: u64| x * x).min / 3).abs_diff(cycle_picos) <= 2);
    // assert!((benchmark(|| 0, |x: u64| x / 3).min / 4).abs_diff(cycle_picos) <= 2);
    // assert!((benchmark(|| 0, |x: u64| x % 3).min / 7).abs_diff(cycle_picos) <= 2);
}
