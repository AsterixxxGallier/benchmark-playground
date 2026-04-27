use crate::benchmark::interval::Interval;
pub(crate) use precise_duration::*;
use rand::random;
use std::fmt::Display;
use std::hint::black_box;
use std::time::Instant;

mod interval;
mod precise_duration;

// returns total, unadjusted picos
fn sample<T>(samples: u32, first_t: T, mut f: impl FnMut(T) -> T) -> PreciseDuration {
    let mut t = first_t;

    // warm-up
    for _ in 0..samples {
        t = black_box(f(t));
    }

    let start = Instant::now();

    for _ in 0..samples {
        t = black_box(f(t));
    }

    let end = Instant::now();

    PreciseDuration::from(end - start)
}

#[must_use]
pub(crate) struct BenchmarkStatistics {
    pub(crate) outliers: usize,
    pub(crate) int: Interval<PreciseDuration>,
    pub(crate) min: PreciseDuration,
    pub(crate) avg: PreciseDuration,
    pub(crate) std: PreciseDuration,
}

impl BenchmarkStatistics {
    fn new(mut samples: Vec<PreciseDuration>) -> Self {
        let avg = samples.iter().sum::<PreciseDuration>() / samples.len();

        let previous_len = samples.len();
        samples.retain(|&duration| duration - avg <= avg / 10u32);

        if samples.is_empty() {
            panic!("no samples, something went wrong");
        }

        let outliers = previous_len - samples.len();

        let int = Interval::from_iter(samples.iter().copied()).unwrap();
        let min = *samples.iter().min().unwrap();
        let avg = samples.iter().sum::<PreciseDuration>() / samples.len();
        let std = (samples
            .iter()
            .map(|&duration| duration.abs_diff(avg).square())
            .sum::<PreciseDuration>()
            / (samples.len() - 1))
            .isqrt();

        Self {
            outliers,
            int,
            min,
            avg,
            std,
        }
    }

    pub(crate) fn report(&self) {
        _ = self.outliers;

        if self.avg.total_attos() < 1000 {
            println!("min:     {:>3} as", self.min.total_attos());
            println!("avg:     {:>3} as", self.avg.total_attos());
            println!("std:     {:>3} as", self.std.total_attos());
        } else if self.avg.total_femtos() < 1000 {
            println!(
                "min: {:>3}.{:0>3} fs",
                self.min.total_femtos(),
                self.min.part_attos()
            );
            println!(
                "avg: {:>3}.{:0>3} fs",
                self.avg.total_femtos(),
                self.avg.part_attos()
            );
            println!(
                "std: {:>3}.{:0>3} fs",
                self.std.total_femtos(),
                self.std.part_attos()
            );
        } else if self.avg.total_picos() < 1000 {
            println!(
                "min: {:>3}.{:0>3} ps",
                self.min.total_picos(),
                self.min.part_femtos()
            );
            println!(
                "avg: {:>3}.{:0>3} ps",
                self.avg.total_picos(),
                self.avg.part_femtos()
            );
            println!(
                "std: {:>3}.{:0>3} ps",
                self.std.total_picos(),
                self.std.part_femtos()
            );
        } else if self.avg.total_nanos() < 1000 {
            println!(
                "min: {:>3}.{:0>3} ns",
                self.min.total_nanos(),
                self.min.part_picos()
            );
            println!(
                "avg: {:>3}.{:0>3} ns",
                self.avg.total_nanos(),
                self.avg.part_picos()
            );
            println!(
                "std: {:>3}.{:0>3} ns",
                self.std.total_nanos(),
                self.std.part_picos()
            );
        } else if self.avg.total_micros() < 1000 {
            println!(
                "min: {:>3}.{:0>3} µs",
                self.min.total_micros(),
                self.min.part_nanos()
            );
            println!(
                "avg: {:>3}.{:0>3} µs",
                self.avg.total_micros(),
                self.avg.part_nanos()
            );
            println!(
                "std: {:>3}.{:0>3} µs",
                self.std.total_micros(),
                self.std.part_nanos()
            );
        } else if self.avg.total_millis() < 1000 {
            println!(
                "min: {:>3}.{:0>3} ms",
                self.min.total_millis(),
                self.min.part_micros()
            );
            println!(
                "avg: {:>3}.{:0>3} ms",
                self.avg.total_millis(),
                self.avg.part_micros()
            );
            println!(
                "std: {:>3}.{:0>3} ms",
                self.std.total_millis(),
                self.std.part_micros()
            );
        } else {
            println!(
                "min: {:>3}.{:0>3} s",
                self.min.total_seconds(),
                self.min.part_millis()
            );
            println!(
                "avg: {:>3}.{:0>3} s",
                self.avg.total_seconds(),
                self.avg.part_millis()
            );
            println!(
                "std: {:>3}.{:0>3} s",
                self.std.total_seconds(),
                self.std.part_millis()
            );
        }
    }

    pub(crate) fn report_as(&self, name: impl Display) {
        println!("# {name}");
        self.report();
        println!();
    }
}

pub(crate) struct Bencher {
    overhead: Interval<PreciseDuration>,
    cycle: Interval<PreciseDuration>,
}

impl Bencher {
    pub(crate) fn new() -> Self {
        let zero_bencher = Self {
            overhead: Interval::point(PreciseDuration::zero()),
            cycle: Interval::point(PreciseDuration::zero()),
        };

        let overhead = zero_bencher.benchmark(|| random(), |value: u64| value).int;

        let overhead_bencher = Self {
            overhead,
            cycle: Interval::point(PreciseDuration::zero()),
        };

        let cycle = overhead_bencher
            .benchmark(|| random(), |value: u64| value + 1)
            .int;

        Self { overhead, cycle }
    }

    fn benchmark_with_counts<T>(
        &self,
        benchmarks: u32,
        samples: u32,
        mut t: impl FnMut() -> T,
        mut f: impl FnMut(T) -> T,
    ) -> BenchmarkStatistics {
        let samples: Vec<_> = (0..benchmarks)
            .map(move |_| sample(samples, t(), &mut f) / samples as u128 - self.overhead.min)
            .collect();

        BenchmarkStatistics::new(samples)
    }

    pub(crate) fn benchmark<T>(
        &self,
        mut t: impl FnMut() -> T,
        mut f: impl FnMut(T) -> T,
    ) -> BenchmarkStatistics {
        let benchmarks = PREFERRED_BENCHMARKS;
        let samples = choose_sample_count(PREFERRED_SAMPLE_DURATION, &mut t, &mut f);

        self.benchmark_with_counts(benchmarks, samples, t, f)
    }

    pub(crate) fn cycles<T>(&self, t: impl FnMut() -> T, f: impl FnMut(T) -> T) -> Interval<f64> {
        let stats = self.benchmark(t, f);
        stats.int.map(PreciseDuration::total_attos_f64)
            / self.cycle.map(PreciseDuration::total_attos_f64)
    }
}

const PREFERRED_BENCHMARKS: u32 = 1000;
const PREFERRED_SAMPLE_DURATION: PreciseDuration = PreciseDuration::from_micros(100);

fn choose_sample_count<T>(
    preferred_duration: PreciseDuration,
    mut t: impl FnMut() -> T,
    mut f: impl FnMut(T) -> T,
) -> u32 {
    let mut samples = 1;

    for _ in 0..10 {
        let duration = sample(samples, t(), &mut f);
        let scale = (preferred_duration.total_attos_f64() / duration.total_attos_f64()).min(1000.0);
        if samples as f64 * scale < 5.0 {
            samples = 5;
            println!(
                "estimated time to collect samples: {:?}",
                duration * samples * 2u32 * PREFERRED_BENCHMARKS / 1000u32
            );
            break;
        }
        samples = (samples as f64 * scale) as u32;
    }

    samples
}
