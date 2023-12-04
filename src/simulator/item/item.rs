use super::Damage;
use crate::simulator::dice::Dice;

pub fn get_keen_increase(threat_range: i32) -> i32 {
    20 - threat_range + 1
}

#[derive(PartialEq)]
pub enum ItemProperty {
    AttackBonus(i32),
    EnchantmentBonus(i32),
    MassiveCrit(Dice),
    DamageBonus(Damage),
    Keen,
}
