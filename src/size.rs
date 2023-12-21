use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
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
