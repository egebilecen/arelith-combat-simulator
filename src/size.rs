use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum SizeCategory {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Unknown,
}

impl Default for SizeCategory {
    fn default() -> Self {
        Self::Medium
    }
}

impl From<&str> for SizeCategory {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "tiny" => Self::Tiny,
            "small" => Self::Small,
            "medium" => Self::Medium,
            "large" => Self::Large,
            "huge" => Self::Huge,
            _ => Self::Unknown,
        }
    }
}
