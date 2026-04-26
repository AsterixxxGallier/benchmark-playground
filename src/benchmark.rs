use rand::random;
use std::hint::black_box;
use std::time::Instant;

const OVERHEAD_PICOS: u128 = 1755;

#[allow(unused)]
fn calculate_overhead_picos() {
    // set `OVERHEAD_PICOS` to 0
    // take `min` as `OVERHEAD_PICOS` value
    benchmark(|| random(), |secret: u64| secret);
}

// returns total, unadjusted picos
fn sample<T>(samples: u32, first_t: T, mut f: impl FnMut(T) -> T) -> u128 {
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

    (end - start).as_nanos() * 1000
}

pub(crate) struct BenchmarkStatistics {
    outliers: usize,
    min: u128,
    avg: u128,
    std: u128,
}

impl BenchmarkStatistics {
    fn new(mut samples: Vec<u128>) -> Self {
        let avg = samples.iter().sum::<u128>() / samples.len() as u128;

        let previous_len = samples.len();
        samples.retain(|&duration| (duration.saturating_sub(avg)) <= avg / 10);

        let outliers = previous_len - samples.len();

        let min = *samples.iter().min().unwrap();
        let avg = samples.iter().sum::<u128>() / samples.len() as u128;
        let std = (samples
            .iter()
            .map(|&duration| duration.abs_diff(avg) * duration.abs_diff(avg))
            .sum::<u128>()
            / (samples.len() as u128 - 1))
            .isqrt();

        Self {
            outliers,
            min,
            avg,
            std,
        }
    }

    pub(crate) fn report(&self) {
        _ = self.outliers;
        println!("min: {:>3}.{:0>3} ns", self.min / 1000, self.min % 1000);
        println!("avg: {:>3}.{:0>3} ns", self.avg / 1000, self.avg % 1000);
        println!("std: {:>3}.{:0>3} ns", self.std / 1000, self.std % 1000);
    }
}

fn benchmark_with_counts<T>(
    benchmarks: u32,
    samples: u32,
    mut t: impl FnMut() -> T,
    mut f: impl FnMut(T) -> T,
) -> BenchmarkStatistics {
    let samples: Vec<_> = (0..benchmarks)
        .map(move |_| sample(samples, t(), &mut f) / samples as u128 - OVERHEAD_PICOS)
        .collect();

    BenchmarkStatistics::new(samples)
}

const PREFERRED_BENCHMARKS: u32 = 100;
const PREFERRED_SAMPLE_PICOS: u128 = 1_000_000_000;

fn choose_sample_count<T>(
    preferred_picos: u128,
    mut t: impl FnMut() -> T,
    mut f: impl FnMut(T) -> T,
) -> u32 {
    let mut samples = 10;

    for _ in 0..10 {
        let picos = sample(samples, t(), &mut f);
        let scale = (preferred_picos as f64 / picos as f64).min(1000.0);
        samples = (samples as f64 * scale) as u32;
    }

    samples
}

pub(crate) fn benchmark<T>(
    mut t: impl FnMut() -> T,
    mut f: impl FnMut(T) -> T,
) -> BenchmarkStatistics {
    let benchmarks = PREFERRED_BENCHMARKS;
    let samples = choose_sample_count(PREFERRED_SAMPLE_PICOS, &mut t, &mut f);

    benchmark_with_counts(benchmarks, samples, t, f)
}
