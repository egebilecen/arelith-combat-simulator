#[derive(PartialEq, Debug, Clone)]
pub struct Feat(pub &'static str);

impl Feat {
    fn name(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Feat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
