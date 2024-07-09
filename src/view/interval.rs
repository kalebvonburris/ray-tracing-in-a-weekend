use crate::Float;

/// A closed time interval.
pub struct Interval {
    /// The minimum value allowed in the interval.
    pub min: Float,
    /// The maximum value allowed in the interval.
    pub max: Float,
}

impl Default for Interval {
    /// Creates a new `Interval` with -Infinity and +Infinity as the minimum and maximum values, respectively.
    fn default() -> Self {
        Self {
            min: Float::NEG_INFINITY,
            max: Float::INFINITY,
        }
    }
}

impl Interval {
    #[must_use]
    /// Creates a new `Interval` with the given minimum and maximum values.
    pub const fn new(min: Float, max: Float) -> Self {
        Self { min, max }
    }

    /// Returns the size of the interval.
    #[must_use]
    pub fn size(&self) -> Float {
        self.max - self.min
    }

    /// Checks if a value, `t`, is within the interval.
    #[must_use]
    pub fn contains(&self, t: Float) -> bool {
        t <= self.max && self.min <= t
    }

    /// Checks if a value, `t`, is completely within the interval, with no overlap at the ends.
    #[must_use]
    pub fn surrounds(&self, t: Float) -> bool {
        t < self.max && self.min < t
    }

    /// Clamps a value, `t`, to the interval.
    #[must_use]
    pub fn clamp(&self, t: Float) -> Float {
        t.min(self.max).max(self.min)
    }
}
