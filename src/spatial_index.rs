use std::fmt;

/// `Axis` identifies a specific axis of a Weave.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Axis(String);

impl Axis {
    /// Return the human-readable name of the axis.
    ///
    /// # Example
    /// ```rust
    /// use atomweave::spatial_index::Axis;
    /// let axis = Axis::from("x");
    /// assert_eq!(axis.name(), "x");
    /// ```
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Axis {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_name() {
        let axis = Axis::from("x");
        assert_eq!(axis.to_string(), "x");
    }
}