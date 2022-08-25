use std::ops::{Add, AddAssign, SubAssign};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
#[serde(from = "f64", into = "f64")]
pub struct FixedPrecision<const PRECISION_MULTIPLIER: usize> {
    number: isize,
}

impl<const PRECISION_MULTIPLIER: usize> From<f64> for FixedPrecision<PRECISION_MULTIPLIER> {
    fn from(number: f64) -> Self {
        FixedPrecision {
            number: (number * PRECISION_MULTIPLIER as f64) as isize,
        }
    }
}

impl<const PRECISION_MULTIPLIER: usize> From<FixedPrecision<PRECISION_MULTIPLIER>> for f64 {
    fn from(precise_number: FixedPrecision<PRECISION_MULTIPLIER>) -> Self {
        precise_number.number as f64 / PRECISION_MULTIPLIER as f64
    }
}

impl<const PRECISION_MULTIPLIER: usize> SubAssign for FixedPrecision<PRECISION_MULTIPLIER> {
    fn sub_assign(&mut self, other: Self) {
        self.number -= other.number
    }
}

impl<const PRECISION_MULTIPLIER: usize> AddAssign for FixedPrecision<PRECISION_MULTIPLIER> {
    fn add_assign(&mut self, rhs: Self) {
        self.number += rhs.number
    }
}

impl<const PRECISION_MULTIPLIER: usize> Add for FixedPrecision<PRECISION_MULTIPLIER> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        FixedPrecision {
            number: self.number + rhs.number,
        }
    }
}

pub type FixedPrecision4 = FixedPrecision<10000>;
