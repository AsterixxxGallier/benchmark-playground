use crate::benchmark::interval::Interval;
use crate::benchmark::{BenchmarkResult, PreciseDuration};
use std::fmt::Display;

#[must_use]
pub(crate) struct BenchmarkStatistics<T: PartialOrd> {
    pub(crate) interval: Interval<T>,
    pub(crate) average: T,
    pub(crate) median: T,
    pub(crate) standard_deviation: T,
}

impl BenchmarkStatistics<PreciseDuration> {
    pub(crate) fn new(results: &BenchmarkResult<PreciseDuration>) -> Self {
        let samples = &results.samples;

        assert!(!samples.is_empty(), "no samples, something went wrong");

        let avg = samples.iter().sum::<PreciseDuration>() / samples.len();
        let interval = Interval::from_iter(samples.iter().copied()).unwrap();
        let average = samples.iter().sum::<PreciseDuration>() / samples.len();
        let median = samples[samples.len() / 2];
        let standard_deviation = (samples
            .iter()
            .map(|&duration| duration.abs_diff(average).square())
            .sum::<PreciseDuration>()
            / (samples.len() - 1))
            .isqrt();

        Self {
            interval,
            average,
            median,
            standard_deviation,
        }
    }

    pub(crate) fn report(&self) -> &Self {
        let biggest_value = self.average.max(self.standard_deviation);

        if biggest_value.total_attos() < 1000 {
            println!("min:     {:>3} as", self.interval.min.total_attos());
            println!("avg:     {:>3} as", self.average.total_attos());
            println!("std:     {:>3} as", self.standard_deviation.total_attos());
        } else if biggest_value.total_femtos() < 1000 {
            println!(
                "min: {:>3}.{:0>3} fs",
                self.interval.min.total_femtos(),
                self.interval.min.part_attos()
            );
            println!(
                "avg: {:>3}.{:0>3} fs",
                self.average.total_femtos(),
                self.average.part_attos()
            );
            println!(
                "std: {:>3}.{:0>3} fs",
                self.standard_deviation.total_femtos(),
                self.standard_deviation.part_attos()
            );
        } else if biggest_value.total_picos() < 1000 {
            println!(
                "min: {:>3}.{:0>3} ps",
                self.interval.min.total_picos(),
                self.interval.min.part_femtos()
            );
            println!(
                "avg: {:>3}.{:0>3} ps",
                self.average.total_picos(),
                self.average.part_femtos()
            );
            println!(
                "std: {:>3}.{:0>3} ps",
                self.standard_deviation.total_picos(),
                self.standard_deviation.part_femtos()
            );
        } else if biggest_value.total_nanos() < 1000 {
            println!(
                "min: {:>3}.{:0>3} ns",
                self.interval.min.total_nanos(),
                self.interval.min.part_picos()
            );
            println!(
                "avg: {:>3}.{:0>3} ns",
                self.average.total_nanos(),
                self.average.part_picos()
            );
            println!(
                "std: {:>3}.{:0>3} ns",
                self.standard_deviation.total_nanos(),
                self.standard_deviation.part_picos()
            );
        } else if biggest_value.total_micros() < 1000 {
            println!(
                "min: {:>3}.{:0>3} µs",
                self.interval.min.total_micros(),
                self.interval.min.part_nanos()
            );
            println!(
                "avg: {:>3}.{:0>3} µs",
                self.average.total_micros(),
                self.average.part_nanos()
            );
            println!(
                "std: {:>3}.{:0>3} µs",
                self.standard_deviation.total_micros(),
                self.standard_deviation.part_nanos()
            );
        } else if biggest_value.total_millis() < 1000 {
            println!(
                "min: {:>3}.{:0>3} ms",
                self.interval.min.total_millis(),
                self.interval.min.part_micros()
            );
            println!(
                "avg: {:>3}.{:0>3} ms",
                self.average.total_millis(),
                self.average.part_micros()
            );
            println!(
                "std: {:>3}.{:0>3} ms",
                self.standard_deviation.total_millis(),
                self.standard_deviation.part_micros()
            );
        } else {
            println!(
                "min: {:>3}.{:0>3} s",
                self.interval.min.total_seconds(),
                self.interval.min.part_millis()
            );
            println!(
                "avg: {:>3}.{:0>3} s",
                self.average.total_seconds(),
                self.average.part_millis()
            );
            println!(
                "std: {:>3}.{:0>3} s",
                self.standard_deviation.total_seconds(),
                self.standard_deviation.part_millis()
            );
        }

        println!();

        self
    }

    pub(crate) fn report_as(&self, name: impl Display) -> &Self {
        println!("# {name}");
        self.report();
        self
    }
}

impl BenchmarkStatistics<f64> {
    pub(crate) fn new(results: &BenchmarkResult<f64>) -> Self {
        let samples = &results.samples;

        assert!(!samples.is_empty(), "no samples, something went wrong");

        let avg = samples.iter().sum::<f64>() / samples.len() as f64;
        let interval = Interval::from_iter(samples.iter().copied()).unwrap();
        let average = samples.iter().sum::<f64>() / samples.len() as f64;
        let median = samples[samples.len() / 2];
        let standard_deviation = (samples
            .iter()
            .map(|&duration| (duration - average) * (duration - average))
            .sum::<f64>()
            / (samples.len() - 1) as f64)
            .sqrt();

        Self {
            interval,
            average,
            median,
            standard_deviation,
        }
    }

    pub(crate) fn report(&self) -> &Self {
        println!("min: {:>7.3}", self.interval.min);
        println!("avg: {:>7.3}", self.average);
        println!("std: {:>7.3}", self.standard_deviation);
        println!();
        self
    }

    pub(crate) fn report_as(&self, name: impl Display) -> &Self {
        println!("# {name}");
        self.report();
        self
    }
}
