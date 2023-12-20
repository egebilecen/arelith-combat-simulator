use super::Damage;
use crate::dice::Dice;
use serde::Serialize;

pub fn get_keen_increase(threat_range: i32) -> i32 {
    20 - threat_range + 1
}

#[derive(PartialEq, Serialize)]
#[allow(unused)]
pub enum ItemProperty {
    AttackBonus(i32),      // Not implemented, it won't increase the character's AB.
    EnchantmentBonus(i32), // Not implemented, it won't increase the character's AB but it will increase the damage.
    DamageBonus(Damage),
    MassiveCrit(Dice),
    Keen,
    ThreatRangeOverride(i32),
    CriticalMultiplierOverride(i32),
}
