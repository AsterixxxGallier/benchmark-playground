use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use std::time::Duration;

const ATTOS_PER_FEMTO: u128 = 1_000;
const ATTOS_PER_PICO: u128 = 1_000_000;
const ATTOS_PER_NANO: u128 = 1_000_000_000;
const ATTOS_PER_MICRO: u128 = 1_000_000_000_000;
const ATTOS_PER_MILLI: u128 = 1_000_000_000_000_000;
const ATTOS_PER_SECOND: u128 = 1_000_000_000_000_000_000;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct PreciseDuration {
    // duration in attoseconds (10^18 attos = 10^9 nanos = 1 second)
    attos: u128,
}

#[allow(unused)]
impl PreciseDuration {
    pub(crate) const fn zero() -> Self {
        Self::from_attos(0)
    }

    pub(crate) const fn from_attos(attos: u128) -> Self {
        Self { attos }
    }

    pub(crate) const fn from_femtos(femtos: u128) -> Self {
        Self::from_attos(femtos * ATTOS_PER_FEMTO)
    }

    pub(crate) const fn from_picos(picos: u128) -> Self {
        Self::from_attos(picos * ATTOS_PER_PICO)
    }

    pub(crate) const fn from_nanos(nanos: u128) -> Self {
        Self::from_attos(nanos * ATTOS_PER_NANO)
    }

    pub(crate) const fn from_micros(micros: u128) -> Self {
        Self::from_attos(micros * ATTOS_PER_MICRO)
    }

    pub(crate) const fn from_millis(millis: u128) -> Self {
        Self::from_attos(millis * ATTOS_PER_MILLI)
    }

    pub(crate) const fn from_seconds(seconds: u128) -> Self {
        Self::from_attos(seconds * ATTOS_PER_SECOND)
    }

    pub(crate) const fn total_attos_f64(self) -> f64 {
        self.attos as f64
    }

    pub(crate) const fn total_femtos_f64(self) -> f64 {
        self.attos as f64 / ATTOS_PER_FEMTO as f64
    }

    pub(crate) const fn total_picos_f64(self) -> f64 {
        self.attos as f64 / ATTOS_PER_PICO as f64
    }

    pub(crate) const fn total_nanos_f64(self) -> f64 {
        self.attos as f64 / ATTOS_PER_NANO as f64
    }

    pub(crate) const fn total_micros_f64(self) -> f64 {
        self.attos as f64 / ATTOS_PER_MICRO as f64
    }

    pub(crate) const fn total_millis_f64(self) -> f64 {
        self.attos as f64 / ATTOS_PER_MILLI as f64
    }

    pub(crate) const fn total_seconds_f64(self) -> f64 {
        self.attos as f64 / ATTOS_PER_SECOND as f64
    }

    pub(crate) const fn total_attos(self) -> u128 {
        self.attos
    }

    pub(crate) const fn total_femtos(self) -> u128 {
        self.attos / ATTOS_PER_FEMTO
    }

    pub(crate) const fn total_picos(self) -> u128 {
        self.attos / ATTOS_PER_PICO
    }

    pub(crate) const fn total_nanos(self) -> u128 {
        self.attos / ATTOS_PER_NANO
    }

    pub(crate) const fn total_micros(self) -> u128 {
        self.attos / ATTOS_PER_MICRO
    }

    pub(crate) const fn total_millis(self) -> u128 {
        self.attos / ATTOS_PER_MILLI
    }

    pub(crate) const fn total_seconds(self) -> u128 {
        self.attos / ATTOS_PER_SECOND
    }

    pub(crate) const fn part_attos(self) -> u16 {
        (self.attos % 1000) as u16
    }

    pub(crate) const fn part_femtos(self) -> u16 {
        (self.attos / ATTOS_PER_FEMTO % 1000) as u16
    }

    pub(crate) const fn part_picos(self) -> u16 {
        (self.attos / ATTOS_PER_PICO % 1000) as u16
    }

    pub(crate) const fn part_nanos(self) -> u16 {
        (self.attos / ATTOS_PER_NANO % 1000) as u16
    }

    pub(crate) const fn part_micros(self) -> u16 {
        (self.attos / ATTOS_PER_MICRO % 1000) as u16
    }

    pub(crate) const fn part_millis(self) -> u16 {
        (self.attos / ATTOS_PER_MILLI % 1000) as u16
    }
}

impl PreciseDuration {
    pub(crate) const fn abs_diff(self, other: Self) -> Self {
        Self::from_attos(self.attos.abs_diff(other.attos))
    }

    pub(super) const fn square(self) -> Self {
        Self::from_attos(self.attos * self.attos)
    }

    pub(super) const fn isqrt(self) -> Self {
        Self::from_attos(self.attos.isqrt())
    }
}

impl From<Duration> for PreciseDuration {
    fn from(value: Duration) -> Self {
        Self {
            attos: value.as_nanos() * ATTOS_PER_NANO,
        }
    }
}

impl Add for PreciseDuration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::from_attos(self.attos + rhs.attos)
    }
}

impl Sub for PreciseDuration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::from_attos(self.attos.saturating_sub(rhs.attos))
    }
}

impl AddAssign for PreciseDuration {
    fn add_assign(&mut self, rhs: Self) {
        self.attos += rhs.attos;
    }
}

impl SubAssign for PreciseDuration {
    fn sub_assign(&mut self, rhs: Self) {
        self.attos = self.attos.saturating_sub(rhs.attos);
    }
}

impl Mul<u16> for PreciseDuration {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self {
        self * rhs as u128
    }
}

impl Div<u8> for PreciseDuration {
    type Output = Self;

    fn div(self, rhs: u8) -> Self {
        self / rhs as u128
    }
}

impl Mul<u8> for PreciseDuration {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self {
        self * rhs as u128
    }
}

impl Div<u16> for PreciseDuration {
    type Output = Self;

    fn div(self, rhs: u16) -> Self {
        self / rhs as u128
    }
}

impl Mul<u32> for PreciseDuration {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        self * rhs as u128
    }
}

impl Div<u32> for PreciseDuration {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        self / rhs as u128
    }
}

impl Mul<u64> for PreciseDuration {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        self * rhs as u128
    }
}

impl Div<u64> for PreciseDuration {
    type Output = Self;

    fn div(self, rhs: u64) -> Self {
        self / rhs as u128
    }
}

impl Div<usize> for PreciseDuration {
    type Output = Self;

    fn div(self, rhs: usize) -> Self {
        self / rhs as u128
    }
}

impl Mul<usize> for PreciseDuration {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        self * rhs as u128
    }
}

impl Mul<u128> for PreciseDuration {
    type Output = Self;

    fn mul(self, rhs: u128) -> Self {
        Self::from_attos(self.attos * rhs)
    }
}

impl Div<u128> for PreciseDuration {
    type Output = Self;

    fn div(self, rhs: u128) -> Self {
        Self::from_attos(self.attos / rhs)
    }
}

impl Sum<Self> for PreciseDuration {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl<'a> Sum<&'a Self> for PreciseDuration {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + *b)
    }
}