use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Feat(pub String);

impl Feat {
    fn name(&self) -> &String {
        &self.0
    }
}

impl std::fmt::Display for Feat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
