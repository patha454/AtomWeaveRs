use std::fmt;

/// `Axis` identifies a specific axis of a Weave.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Axis(u64);

impl Axis {
    fn new(index: u64) -> Self {
        Self(index)
    }

    fn index(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = Axis(self.0);
        write!(f, "axis {}", self.0)
    }
}
