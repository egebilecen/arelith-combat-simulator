use super::{get_keen_increase, DamageType, ItemProperty};
use crate::{dice::Dice, size::SizeCategory};

#[derive(Clone, Default)]
pub struct WeaponBase {
    pub name: String,
    pub size: SizeCategory,
    pub damage: Dice,
    pub threat_range: i32,
    pub crit_multiplier: i32,
    pub damage_type: Vec<DamageType>,
}

impl WeaponBase {
    pub fn new(
        name: String,
        size: SizeCategory,
        damage: Dice,
        threat_range: i32,
        crit_multiplier: i32,
        damage_type: Vec<DamageType>,
    ) -> Self {
        WeaponBase {
            name,
            size,
            damage,
            threat_range,
            crit_multiplier,
            damage_type,
        }
    }
}

#[derive(Default)]
pub struct Weapon {
    pub name: String,
    pub base: WeaponBase,
    pub item_properties: Vec<ItemProperty>,
}

impl Weapon {
    pub fn new(name: String, base: WeaponBase, item_properties: Vec<ItemProperty>) -> Self {
        Weapon {
            name,
            base,
            item_properties,
        }
    }

    pub fn is_keen(&self) -> bool {
        self.item_properties.contains(&ItemProperty::Keen)
    }

    pub fn threat_range(&self) -> i32 {
        self.base.threat_range
            - if self.is_keen() {
                get_keen_increase(self.base.threat_range)
            } else {
                0
            }
    }

    pub fn crit_multiplier(&self) -> i32 {
        self.base.crit_multiplier
    }

    pub fn crit_multiplier_override(&self) -> Option<i32> {
        for override_val in self
            .item_properties
            .iter()
            .map(|x| match x {
                ItemProperty::CriticalMultiplierOverride(value) => *value,
                _ => 0,
            })
            .filter(|x| *x > 0)
        {
            return Some(override_val);
        }

        None
    }

    pub fn threat_range_override(&self) -> Option<i32> {
        for override_val in self
            .item_properties
            .iter()
            .map(|x| match x {
                ItemProperty::ThreatRangeOverride(value) => *value,
                _ => 0,
            })
            .filter(|x| *x > 0)
        {
            return Some(override_val);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::{
        character::Character,
        dice::Dice,
        item::{weapon_db::get_weapon_base, DamageType, ItemProperty, Weapon, WeaponBase},
        size::SizeCategory,
    };

    #[test]
    fn weapon_base() {
        let weapon_base = get_weapon_base("Rapier");
        assert_eq!(weapon_base.name, "Rapier");
        assert_eq!(weapon_base.threat_range, 18);
        assert_eq!(weapon_base.crit_multiplier, 2);
        assert_eq!(weapon_base.damage.to_string(), "1d6");
        assert_eq!(weapon_base.damage_type[0], DamageType::Piercing);

        let weapon_base = get_weapon_base("Double Axe");
        assert_eq!(weapon_base.name, "Double Axe");
        assert_eq!(weapon_base.threat_range, 20);
        assert_eq!(weapon_base.crit_multiplier, 3);
        assert_eq!(weapon_base.damage.to_string(), "3d4");
        assert_eq!(weapon_base.damage_type[0], DamageType::Slashing);
    }

    #[test]
    fn weapon() {
        let weapon = Weapon::new("".into(), get_weapon_base("Rapier"), vec![]);
        assert_eq!(weapon.is_keen(), false);
        assert_eq!(weapon.threat_range(), 18);

        // Keen test: 18-20
        let weapon = Weapon::new(
            "".into(),
            WeaponBase::new(
                "".into(),
                SizeCategory::Medium,
                Dice::from(0),
                18,
                2,
                vec![DamageType::Slashing],
            ),
            vec![ItemProperty::Keen],
        );
        assert_eq!(weapon.is_keen(), true);
        assert_eq!(weapon.threat_range(), 15);

        // Keen test: 19-20
        let weapon = Weapon::new(
            "".into(),
            WeaponBase::new(
                "".into(),
                SizeCategory::Medium,
                Dice::from(0),
                19,
                2,
                vec![DamageType::Slashing],
            ),
            vec![ItemProperty::Keen],
        );
        assert_eq!(weapon.is_keen(), true);
        assert_eq!(weapon.threat_range(), 17);

        // Keen test: 20
        let weapon = Weapon::new(
            "".into(),
            WeaponBase::new(
                "".into(),
                SizeCategory::Medium,
                Dice::from(0),
                20,
                2,
                vec![DamageType::Slashing],
            ),
            vec![ItemProperty::Keen],
        );
        assert_eq!(weapon.is_keen(), true);
        assert_eq!(weapon.threat_range(), 19);

        // Threat range and critical multiplier override test
        let weapon = Weapon::new(
            "".into(),
            WeaponBase::new(
                "".into(),
                SizeCategory::Medium,
                Dice::from(0),
                20,
                2,
                vec![DamageType::Slashing],
            ),
            vec![
                ItemProperty::Keen,
                ItemProperty::ThreatRangeOverride(99),
                ItemProperty::CriticalMultiplierOverride(50),
            ],
        );

        let character = Character::builder().weapon(weapon).build();

        assert_eq!(character.weapon_threat_range(), 99);
        assert_eq!(character.weapon_crit_multiplier(), 50);
    }
}
