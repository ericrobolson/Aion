use std::{
    ops::{Add, AddAssign},
    u64,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame(pub(crate) u64);
impl Frame {
    /// Returns the raw value of the frame.
    pub fn raw_value(&self) -> u64 {
        self.0
    }

    /// Returns true if self is greater than other.
    /// This function handles wrap arounds.
    pub fn greater_than(&self, other: Frame) -> bool {
        // Deal with wrap arounds.
        // https://gafferongames.com/post/reliability_ordering_and_congestion_avoidance_over_udp/
        // Pretty unlikely to happen in practice, but want to be sure.
        let s1 = self.0;
        let s2 = other.0;
        const HALF_MAX_VALUE: u64 = u64::MAX / 2;

        ((s1 > s2) && (s1 - s2 <= HALF_MAX_VALUE)) || ((s1 < s2) && (s2 - s1 > HALF_MAX_VALUE))
    }
}
impl From<u64> for Frame {
    fn from(value: u64) -> Self {
        Frame(value)
    }
}
impl Default for Frame {
    fn default() -> Self {
        Frame(0)
    }
}
impl Add<u64> for Frame {
    type Output = Frame;

    fn add(self, rhs: u64) -> Frame {
        if let Some(result) = self.0.checked_add(rhs) {
            Frame(result)
        } else {
            Frame(0)
        }
    }
}

impl AddAssign<u64> for Frame {
    fn add_assign(&mut self, rhs: u64) {
        if let Some(result) = self.0.checked_add(rhs) {
            self.0 = result;
        } else {
            self.0 = 0;
        }
    }
}
