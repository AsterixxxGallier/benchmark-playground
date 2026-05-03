#![allow(unused)]
#![feature(portable_simd)]

use crate::benchmark::{bench, PreciseDuration, PrecisionSampler};
use std::simd::num::SimdUint;
use std::simd::Simd;

mod benchmark;

fn main() {
    let sampler = PrecisionSampler::with_defaults(PreciseDuration::zero());

    let baseline = bench(&sampler, || 0, |x| x)
        .statistics()
        .report_as("baseline")
        .average;
    let cycle = bench(&sampler, || 0, |x| x + 1)
        .with_baseline(baseline)
        .statistics()
        .report_as("cycle")
        .average;

    let sampler = PrecisionSampler::with_defaults(PreciseDuration::from_picos(5));

    let scalar_mul = bench(&sampler, || 12341234, |x: u64| x ^ 151515)
        .with_baseline(baseline)
        .statistics()
        .report_as("scalar multiplication")
        .average;

    loop {
        const N: usize = 64;

        bench(
            &sampler,
            || Simd::splat(12341234),
            |x: Simd<u64, N>| x ^ Simd::splat(151515),
        )
        .with_baseline(baseline)
        .with_unit(scalar_mul * N)
        .recip()
        .statistics()
        .report_as("SIMD speedup");
    }
}
