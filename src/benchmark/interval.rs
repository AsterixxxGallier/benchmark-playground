use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Interval<T: PartialOrd> {
    pub(crate) min: T,
    pub(crate) max: T,
}

#[allow(unused)]
impl<T: PartialOrd> Interval<T> {
    pub(crate) fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub(crate) fn point(value: T) -> Self
    where
        T: Clone,
    {
        Self::new(value.clone(), value)
    }

    pub(crate) fn with(self, value: T) -> Self {
        if value < self.min {
            Self {
                min: value,
                max: self.max,
            }
        } else if value > self.max {
            Self {
                min: self.min,
                max: value,
            }
        } else {
            self
        }
    }

    pub(crate) fn from_iter(mut iter: impl Iterator<Item = T>) -> Option<Self>
    where
        T: Clone,
    {
        let first = iter.next()?;
        let mut min = first.clone();
        let mut max = first;

        for value in iter {
            if value < min {
                min = value;
            } else if value > max {
                max = value;
            }
        }

        Some(Self::new(min, max))
    }

    pub(crate) fn contains(&self, value: T) -> bool {
        self.min <= value && value <= self.max
    }

    pub(crate) fn size(&self) -> <T as Sub>::Output
    where
        T: Sub + Copy,
    {
        self.max - self.min
    }

    pub(crate) fn map<U: PartialOrd>(self, f: impl Fn(T) -> U) -> Interval<U> {
        Interval::new(f(self.min), f(self.max))
    }
}

impl<T: PartialOrd> Add for Interval<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.min + rhs.min, self.max + rhs.max)
    }
}

impl<T: PartialOrd> Sub for Interval<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.min - rhs.max, self.max - rhs.min)
    }
}

impl<T: PartialOrd> Mul for Interval<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.min * rhs.min, self.max * rhs.max)
    }
}

impl<T: PartialOrd> Div for Interval<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(self.min / rhs.max, self.max / rhs.min)
    }
}
