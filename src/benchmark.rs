use crate::benchmark::interval::Interval;
pub(crate) use interval::*;
pub(crate) use precise_duration::*;
pub(crate) use statistics::*;
use std::cmp;
use std::collections::BinaryHeap;
use std::fmt::Display;
use std::hint::black_box;
use std::time::Instant;

mod interval;
mod precise_duration;
mod statistics;

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

pub(crate) trait Sampler {
    fn collect_samples(&self, sample: impl FnMut() -> PreciseDuration) -> Vec<PreciseDuration>;
}

pub(crate) struct PrecisionSampler {
    pub(crate) desired_precision: PreciseDuration,
    pub(crate) desired_samples_within_precision: usize,
    pub(crate) min_samples: usize,
    pub(crate) max_samples: usize,
    pub(crate) min_kept_samples: usize,
    pub(crate) max_kept_samples: usize,
}

impl PrecisionSampler {
    pub(crate) fn with_defaults(desired_precision: PreciseDuration) -> Self {
        Self {
            desired_precision,
            desired_samples_within_precision: 50,
            min_samples: 100,
            max_samples: 1000,
            min_kept_samples: 50,
            max_kept_samples: 100,
        }
    }
}

fn heap_pop_while<T: Ord>(heap: &mut BinaryHeap<T>, condition: impl Fn(&T) -> bool) {
    loop {
        let top = heap.peek();
        if top.is_some_and(&condition) {
            heap.pop();
        } else {
            break;
        }
    }
}

impl Sampler for PrecisionSampler {
    fn collect_samples(&self, mut sample: impl FnMut() -> PreciseDuration) -> Vec<PreciseDuration> {
        assert!(0 < self.desired_samples_within_precision);
        assert!(0 < self.min_samples);
        assert!(self.desired_samples_within_precision <= self.min_kept_samples);
        assert!(self.min_samples <= self.max_samples);
        assert!(self.min_kept_samples <= self.max_kept_samples);

        let first_sample = sample();

        let mut min = first_sample;

        let mut precision_bounded_heap =
            BinaryHeap::with_capacity(self.desired_samples_within_precision + 1);
        let mut number_bounded_heap = BinaryHeap::with_capacity(self.max_kept_samples + 1);

        precision_bounded_heap.push(first_sample);
        number_bounded_heap.push(first_sample);

        for _ in 1..self.min_samples {
            let sample = sample();
            precision_bounded_heap.push(sample);
            number_bounded_heap.push(sample);
            if number_bounded_heap.len() > self.max_kept_samples {
                number_bounded_heap.pop();
            }
            min = cmp::min(min, sample);
        }

        heap_pop_while(&mut precision_bounded_heap, |value| {
            *value > min + self.desired_precision
        });

        let mut samples_collected = self.min_samples;

        while precision_bounded_heap.len() < self.desired_samples_within_precision
            && samples_collected < self.max_samples
        {
            let sample = sample();
            samples_collected += 1;
            min = cmp::min(min, sample);

            precision_bounded_heap.push(sample);
            number_bounded_heap.push(sample);
            if number_bounded_heap.len() > self.max_kept_samples {
                number_bounded_heap.pop();
            }

            heap_pop_while(&mut precision_bounded_heap, |value| {
                *value > min + self.desired_precision
            });
        }

        if precision_bounded_heap.len() >= self.min_kept_samples {
            let mut vec = precision_bounded_heap.into_sorted_vec();
            vec.truncate(self.max_kept_samples);
            vec
        } else {
            number_bounded_heap.into_sorted_vec()
        }
    }
}

pub(crate) struct FixedCountSampler {
    pub(crate) samples: usize,
    pub(crate) kept_samples: usize,
}

impl FixedCountSampler {
    pub(crate) fn with_defaults(samples: usize) -> Self {
        Self { samples, kept_samples: (samples / 10).max(10) }
    }
}

impl Sampler for FixedCountSampler {
    fn collect_samples(&self, mut sample: impl FnMut() -> PreciseDuration) -> Vec<PreciseDuration> {
        assert!(0 < self.samples);
        assert!(self.kept_samples <= self.samples);

        let mut samples: Vec<_> = (0..self.samples).map(move |_| sample()).collect();
        samples.sort();
        samples.truncate(self.kept_samples);
        samples
    }
}

pub(crate) fn bench<T>(
    sampler: &impl Sampler,
    mut t: impl FnMut() -> T,
    mut f: impl FnMut(T) -> T,
) -> BenchmarkStatistics<PreciseDuration> {
    let iterations =
        choose_iterations_per_sample(PREFERRED_SAMPLE_DURATION, &mut t, &mut f);
    let sample = || sample(iterations, t(), &mut f) / iterations;
    let samples = sampler.collect_samples(sample);
    BenchmarkStatistics::new(samples)
}

const PREFERRED_BENCHMARKS: u32 = 1000;
const PREFERRED_SAMPLE_DURATION: PreciseDuration = PreciseDuration::from_micros(100);

fn choose_iterations_per_sample<T>(
    preferred_duration: PreciseDuration,
    mut t: impl FnMut() -> T,
    mut f: impl FnMut(T) -> T,
) -> u32 {
    let mut iterations = 1;

    for _ in 0..10 {
        let duration = sample(iterations, t(), &mut f);
        let scale = (preferred_duration.total_attos_f64() / duration.total_attos_f64()).min(1000.0);
        if iterations as f64 * scale < 5.0 {
            iterations = 5;
            break;
        }
        iterations = (iterations as f64 * scale) as u32;
    }

    iterations
}
