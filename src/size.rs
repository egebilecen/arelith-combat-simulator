use serde::Serialize;

#[derive(Clone, PartialEq, PartialOrd, Serialize)]
pub enum SizeCategory {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

impl Default for SizeCategory {
    fn default() -> Self {
        Self::Medium
    }
}
