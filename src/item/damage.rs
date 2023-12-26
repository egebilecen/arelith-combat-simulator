use crate::dice::Dice;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[allow(unused)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Magical,
    Acid,
    Cold,
    Divine,
    Electrical,
    Fire,
    Negative,
    Positive,
    Sonic,
    Entropy,
    Force,
    Psychic,
    Poison,
    Unknown,
}

impl DamageType {
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }

    pub fn is_physical(&self) -> bool {
        match self {
            Self::Slashing | Self::Piercing | Self::Bludgeoning => true,
            _ => false,
        }
    }
}

impl From<&str> for DamageType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "slashing" => Self::Slashing,
            "piercing" => Self::Piercing,
            "bludgeoning" => Self::Bludgeoning,
            "magical" => Self::Magical,
            "acid" => Self::Acid,
            "cold" => Self::Cold,
            "divine" => Self::Divine,
            "electrical" => Self::Electrical,
            "fire" => Self::Fire,
            "negative" => Self::Negative,
            "positive" => Self::Positive,
            "sonic" => Self::Sonic,
            "entropy" => Self::Entropy,
            "force" => Self::Force,
            "psychic" => Self::Psychic,
            "poison" => Self::Poison,
            _ => Self::Unknown,
        }
    }
}

impl std::fmt::Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Damage {
    amount: Dice,
    pub type_: DamageType,
    pub is_resistable: bool,
    pub can_crit: bool,
}

impl std::fmt::Display for Damage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.amount.to_string())
    }
}

impl Damage {
    pub fn new(type_: DamageType, amount: Dice, is_resistable: bool, can_crit: bool) -> Self {
        Damage {
            type_,
            amount,
            is_resistable,
            can_crit,
        }
    }

    #[allow(unused)]
    pub fn roll(&self) -> i32 {
        self.amount.roll()
    }

    pub fn roll_m(&self, count: i32) -> i32 {
        self.amount.roll_m(count)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DamageResult(RefCell<HashMap<DamageType, i32>>);

impl DamageResult {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(unused)]
    pub fn set(&self, type_: DamageType, amount: i32) {
        *self.0.borrow_mut().entry(type_).or_insert(0) += amount;
    }

    pub fn get(&self, type_: DamageType) -> i32 {
        self.0.borrow().get(&type_).unwrap_or(&0).to_owned()
    }

    pub fn get_types(&self) -> Vec<DamageType> {
        self.0.borrow().keys().cloned().collect::<Vec<DamageType>>()
    }

    pub fn get_types_sorted(&self) -> Vec<DamageType> {
        let mut types = self.get_types();
        types.sort();

        types
    }

    pub fn add(&self, type_: DamageType, amount: i32) -> i32 {
        let mut hashmap = self.0.borrow_mut();
        let current_dmg = hashmap.entry(type_).or_insert(0);
        *current_dmg += amount;

        *current_dmg
    }

    pub fn sub(&self, type_: DamageType, amount: i32) -> i32 {
        let mut hashmap = self.0.borrow_mut();

        if hashmap.contains_key(&type_) {
            let current_dmg = hashmap.get_mut(&type_);

            if let Some(current_dmg) = current_dmg {
                *current_dmg -= amount;

                if *current_dmg < 0 {
                    *current_dmg = 0;
                }

                return *current_dmg;
            }
        }

        0
    }

    pub fn total_dmg(&self) -> i32 {
        self.0.borrow().iter().map(|(_, v)| v).sum()
    }

    pub fn add_from(&mut self, other: &DamageResult) {
        for type_ in other.get_types() {
            self.add(type_, other.get(type_));
        }
    }
}

#[cfg(test)]
mod test {
    use super::DamageResult;
    use crate::item::DamageType;

    #[test]
    fn damage_result() {
        let dmg_result = DamageResult::new();
        assert_eq!(dmg_result.get(DamageType::Acid), 0);
        assert_eq!(dmg_result.get(DamageType::Bludgeoning), 0);

        dmg_result.set(DamageType::Acid, 4);
        assert_eq!(dmg_result.get(DamageType::Acid), 4);

        dmg_result.add(DamageType::Bludgeoning, 8);
        assert_eq!(dmg_result.get(DamageType::Bludgeoning), 8);

        dmg_result.add(DamageType::Cold, 3);
        assert_eq!(dmg_result.get(DamageType::Cold), 3);

        dmg_result.sub(DamageType::Cold, 1);
        assert_eq!(dmg_result.get(DamageType::Cold), 2);

        assert_eq!(dmg_result.total_dmg(), 14);

        assert_eq!(dmg_result.get_types().len(), 3);
        assert_eq!(dmg_result.get_types().contains(&DamageType::Acid), true);
        assert_eq!(
            dmg_result.get_types().contains(&DamageType::Bludgeoning),
            true
        );
        assert_eq!(dmg_result.get_types().contains(&DamageType::Cold), true);
        assert_eq!(
            dmg_result.get_types().contains(&DamageType::Slashing),
            false
        );
    }
}
