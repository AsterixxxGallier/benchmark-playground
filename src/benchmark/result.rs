use crate::benchmark::PreciseDuration;
use std::ops::SubAssign;
use crate::benchmark::statistics::BenchmarkStatistics;

pub(crate) struct BenchmarkResult<T: PartialOrd> {
    /// must be sorted
    pub(crate) samples: Vec<T>,
}

impl<T: PartialOrd> BenchmarkResult<T> {
    pub(crate) fn with_baseline(mut self, baseline: T) -> Self
    where
        T: Copy + SubAssign,
    {
        for sample in &mut self.samples {
            *sample -= baseline;
        }
        self
    }
}

impl BenchmarkResult<PreciseDuration> {
    pub(crate) fn with_unit(mut self, unit_duration: PreciseDuration) -> BenchmarkResult<f64> {
        BenchmarkResult {
            samples: self
                .samples
                .into_iter()
                .map(|sample| sample.total_attos_f64() / unit_duration.total_attos_f64())
                .collect(),
        }
    }

    pub(crate) fn statistics(&self) -> BenchmarkStatistics<PreciseDuration> {
        BenchmarkStatistics::<PreciseDuration>::new(self)
    }
}

impl BenchmarkResult<f64> {
    pub(crate) fn recip(mut self) -> Self {
        for sample in &mut self.samples {
            *sample = sample.recip();
        }
        self
    }

    pub(crate) fn statistics(&self) -> BenchmarkStatistics<f64> {
        BenchmarkStatistics::<f64>::new(self)
    }
}
