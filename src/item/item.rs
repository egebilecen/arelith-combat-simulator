use crate::dice::Dice;
use super::Damage;

pub fn get_keen_increase(threat_range: i32) -> i32 {
    20 - threat_range + 1
}

#[derive(PartialEq)]
#[allow(unused)]
pub enum ItemProperty {
    AttackBonus(i32),
    EnchantmentBonus(i32),
    MassiveCrit(Dice),
    DamageBonus(Damage),
    Keen,
    ThreatRangeOverride(i32),
    CriticalMultiplierOverride(i32),
}
