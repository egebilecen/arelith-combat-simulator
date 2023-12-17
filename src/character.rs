use super::{
    combat::{AttackInfo, AttackType},
    feat::{feat_db::get_feat, Feat},
    item::{get_keen_increase, DamageType, Weapon},
    rules::CONSECUTIVE_ATTACK_AB_PENALTY,
    size::SizeCategory,
};

#[derive(Debug)]
pub struct AbilityScore(i32);

impl AbilityScore {
    pub fn get_mod(&self) -> i32 {
        let score = self.0 - if self.0 < 10 { 1 } else { 0 };

        (score - 10) / 2
    }
}

impl Default for AbilityScore {
    fn default() -> Self {
        AbilityScore(0)
    }
}

impl From<i32> for AbilityScore {
    fn from(value: i32) -> Self {
        AbilityScore(value)
    }
}

#[derive(Default, Debug)]
pub struct AbilityList {
    pub str: AbilityScore,
    pub dex: AbilityScore,
    pub con: AbilityScore,
    pub int: AbilityScore,
    pub wis: AbilityScore,
    pub cha: AbilityScore,
}

impl AbilityList {
    pub fn builder() -> AbilityListBuilder {
        AbilityListBuilder::new()
    }
}

pub struct AbilityListBuilder {
    abilities: AbilityList,
}

#[allow(unused)]
impl AbilityListBuilder {
    pub fn new() -> Self {
        Self {
            abilities: AbilityList::default(),
        }
    }

    pub fn str(mut self, value: i32) -> Self {
        self.abilities.str = value.into();
        self
    }

    pub fn dex(mut self, value: i32) -> Self {
        self.abilities.dex = value.into();
        self
    }

    pub fn con(mut self, value: i32) -> Self {
        self.abilities.con = value.into();
        self
    }

    pub fn int(mut self, value: i32) -> Self {
        self.abilities.int = value.into();
        self
    }

    pub fn wis(mut self, value: i32) -> Self {
        self.abilities.wis = value.into();
        self
    }

    pub fn cha(mut self, value: i32) -> Self {
        self.abilities.cha = value.into();
        self
    }

    pub fn build(self) -> AbilityList {
        AbilityList { ..self.abilities }
    }
}

#[derive(Default)]
pub struct Character {
    pub name: String,
    pub size: SizeCategory,
    pub abilities: AbilityList,

    pub ac: i32,
    pub ab: i32,

    pub base_apr: i32,
    pub extra_apr: i32,

    pub concealment: i32,
    pub defensive_essence: i32,
    pub physical_immunity: i32,
    pub physical_dmg_reduction: i32,

    pub weapon: Weapon,
    pub feats: Vec<Feat>,
}

impl Character {
    pub fn builder() -> CharacterBuilder {
        CharacterBuilder::new()
    }

    pub fn total_apr(&self) -> i32 {
        self.base_apr + self.extra_apr + if self.is_dual_wielding() { 2 } else { 0 }
    }

    pub fn has_feat(&self, feat: Feat) -> bool {
        self.feats.contains(&feat)
    }

    pub fn has_blind_fight(&self) -> bool {
        self.has_feat(get_feat("Blind Fight"))
    }

    pub fn has_epic_dodge(&self) -> bool {
        self.has_feat(get_feat("Epic Dodge"))
    }

    pub fn has_bane_of_enemies(&self) -> bool {
        self.has_feat(get_feat("Bane of Enemies"))
    }

    pub fn has_weapon_spec(&self) -> bool {
        self.has_feat(get_feat("Weapon Specialization"))
    }

    pub fn has_epic_weapon_spec(&self) -> bool {
        self.has_feat(get_feat("Epic Weapon Specialization"))
    }

    pub fn is_dual_wielding(&self) -> bool {
        self.has_feat(get_feat("Dual Wielding"))
    }

    pub fn is_crit_immune(&self) -> bool {
        self.has_feat(get_feat("Critical Immunity"))
    }

    pub fn atk_ab(&self, atk_no: i32) -> Option<AttackInfo> {
        if atk_no < 1 || atk_no > self.total_apr() {
            return None;
        }

        if atk_no <= self.base_apr {
            return Some(AttackInfo::new(
                self.ab - (CONSECUTIVE_ATTACK_AB_PENALTY * (atk_no - 1)),
                AttackType::MainHand,
            ));
        }

        if self.extra_apr > 0 && atk_no <= self.base_apr + self.extra_apr {
            let extra_atk_no = atk_no - self.base_apr;
            let extra_atk_ab = self.ab - ((extra_atk_no - 1) * CONSECUTIVE_ATTACK_AB_PENALTY)
                + if self.is_dual_wielding() { 2 } else { 0 };

            return Some(AttackInfo::new(extra_atk_ab, AttackType::Extra));
        }

        if self.is_dual_wielding() && atk_no <= self.total_apr() {
            let dw_atk_no = atk_no - self.total_apr() + 2;

            return Some(AttackInfo::new(
                self.ab - ((dw_atk_no - 1) * CONSECUTIVE_ATTACK_AB_PENALTY),
                AttackType::OffHand,
            ));
        }

        None
    }

    pub fn weapon_crit_multiplier(&self) -> i32 {
        if let Some(override_val) = self.weapon.crit_multiplier_override() {
            return override_val;
        }

        self.weapon.crit_multiplier()
            + if self.has_feat(get_feat("Increased Multiplier")) {
                1
            } else {
                0
            }
    }

    pub fn weapon_threat_range(&self) -> i32 {
        if let Some(override_val) = self.weapon.threat_range_override() {
            return override_val;
        }

        self.weapon.threat_range()
            - if self.has_feat(get_feat("Improved Critical")) {
                get_keen_increase(self.weapon.base.threat_range)
            } else {
                0
            }
            - if self.has_feat(get_feat("Ki Critical")) {
                2
            } else {
                0
            }
    }

    pub fn is_weapon_twohanded(&self) -> bool {
        if self.weapon.base.size > self.size {
            true
        } else {
            false
        }
    }

    pub fn damage_immunity(&self, dmg_type: DamageType) -> i32 {
        if dmg_type.is_physical() {
            return self.physical_immunity;
        }

        0
    }

    pub fn damage_reduction(&self, dmg_type: DamageType) -> i32 {
        if dmg_type.is_physical() {
            return self.physical_dmg_reduction;
        }

        0
    }

    #[allow(unused)]
    pub fn damage_resistance(&self, dmg_type: DamageType) -> i32 {
        unimplemented!()
    }

    #[allow(unused)]
    pub fn weapon_string(&self) -> String {
        format!(
            "{} ({} x{})",
            self.weapon.name,
            if self.weapon.threat_range() < 20 {
                format!("{}-{}", self.weapon_threat_range(), 20)
            } else {
                "20".to_string()
            },
            self.weapon_crit_multiplier()
        )
    }
}

#[derive(Default)]
pub struct CharacterBuilder {
    character: Character,
}

#[allow(unused)]
impl CharacterBuilder {
    pub fn new() -> Self {
        Self {
            character: Character::default(),
        }
    }

    pub fn standard_dummy(ac: i32) -> Self {
        Self::new()
            .name("Standard Dummy".into())
            .ac(ac)
            .concealment(50)
            .physical_immunity(10)
            .defensive_essence(5)
    }

    pub fn name(mut self, name: String) -> Self {
        self.character.name = name;
        self
    }

    pub fn size(mut self, size: SizeCategory) -> Self {
        self.character.size = size;
        self
    }

    pub fn abilities(mut self, abilities: AbilityList) -> Self {
        self.character.abilities = abilities;
        self
    }

    pub fn ac(mut self, ac: i32) -> Self {
        self.character.ac = ac;
        self
    }

    pub fn ab(mut self, ab: i32) -> Self {
        self.character.ab = ab;
        self
    }

    pub fn base_apr(mut self, base_apr: i32) -> Self {
        self.character.base_apr = base_apr;
        self
    }

    pub fn extra_apr(mut self, extra_apr: i32) -> Self {
        self.character.extra_apr = extra_apr;
        self
    }

    pub fn concealment(mut self, concealment: i32) -> Self {
        self.character.concealment = concealment;
        self
    }

    pub fn defensive_essence(mut self, defensive_essence: i32) -> Self {
        self.character.defensive_essence = defensive_essence;
        self
    }

    pub fn physical_immunity(mut self, physical_immunity: i32) -> Self {
        self.character.physical_immunity = physical_immunity;
        self
    }

    pub fn physical_damage_reduction(mut self, physical_damage_reduction: i32) -> Self {
        self.character.physical_dmg_reduction = physical_damage_reduction;
        self
    }

    pub fn weapon(mut self, weapon: Weapon) -> Self {
        self.character.weapon = weapon;
        self
    }

    pub fn feats(mut self, feats: Vec<Feat>) -> Self {
        self.character.feats = feats;
        self
    }

    pub fn add_feat(mut self, feat: Feat) -> Self {
        self.character.feats.push(feat);
        self
    }

    pub fn build(self) -> Character {
        Character { ..self.character }
    }
}

impl From<Character> for CharacterBuilder {
    fn from(value: Character) -> Self {
        Self { character: value }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        character::{AbilityList, Character, CharacterBuilder},
        dice::Dice,
        feat::feat_db::get_feat,
        item::{weapon_db::get_weapon_base, DamageType, ItemProperty, Weapon, WeaponBase},
        size::SizeCategory,
    };

    #[test]
    fn character() {
        let character: Character = Character::builder()
            .abilities(
                AbilityList::builder()
                    .str(38)
                    .dex(20)
                    .con(28)
                    .int(14)
                    .wis(8)
                    .cha(6)
                    .build(),
            )
            .ac(30)
            .ab(50)
            .base_apr(4)
            .extra_apr(1)
            .concealment(50)
            .defensive_essence(5)
            .physical_immunity(0)
            .physical_damage_reduction(0)
            .weapon(Weapon::new(
                "".into(),
                get_weapon_base("Rapier".into()),
                vec![ItemProperty::Keen],
            ))
            .feats(vec![get_feat("Blind Fight")])
            .build();

        assert_eq!(character.abilities.str.get_mod(), 14);
        assert_eq!(character.abilities.dex.get_mod(), 5);
        assert_eq!(character.abilities.con.get_mod(), 9);
        assert_eq!(character.abilities.int.get_mod(), 2);
        assert_eq!(character.abilities.wis.get_mod(), -1);
        assert_eq!(character.abilities.cha.get_mod(), -2);

        assert_eq!(character.total_apr(), 5);
        assert_eq!(character.has_blind_fight(), true);
        assert_eq!(character.is_weapon_twohanded(), false);

        // Keen + Improved Critical test: 18-20
        let character = CharacterBuilder::from(character)
            .weapon(Weapon::new(
                "".into(),
                WeaponBase::new(
                    "".into(),
                    SizeCategory::Large,
                    Dice::from(0),
                    18,
                    2,
                    vec![DamageType::Slashing],
                ),
                vec![ItemProperty::Keen],
            ))
            .feats(vec![get_feat("Improved Critical")])
            .build();
        assert_eq!(character.weapon_threat_range(), 12);

        // Keen + Improved Critical test: 19-20
        let character = CharacterBuilder::from(character)
            .weapon(Weapon::new(
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
            ))
            .feats(vec![get_feat("Improved Critical")])
            .build();
        assert_eq!(character.weapon_threat_range(), 15);

        // Keen + Improved Critical test: 20
        let character = CharacterBuilder::from(character)
            .weapon(Weapon::new(
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
            ))
            .feats(vec![get_feat("Improved Critical")])
            .build();
        assert_eq!(character.weapon_threat_range(), 18);

        // Keen + Improved Critical + Ki Critical test: 18-20
        let character = CharacterBuilder::from(character)
            .weapon(Weapon::new(
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
            ))
            .feats(vec![get_feat("Improved Critical"), get_feat("Ki Critical")])
            .build();
        assert_eq!(character.weapon_threat_range(), 10);

        // Keen + Improved Critical + Ki Critical test: 19-20
        let character = CharacterBuilder::from(character)
            .weapon(Weapon::new(
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
            ))
            .feats(vec![get_feat("Improved Critical"), get_feat("Ki Critical")])
            .build();
        assert_eq!(character.weapon_threat_range(), 13);

        // Keen + Improved Critical + Ki Critical test: 20
        let character = CharacterBuilder::from(character)
            .weapon(Weapon::new(
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
            ))
            .feats(vec![get_feat("Improved Critical"), get_feat("Ki Critical")])
            .build();
        assert_eq!(character.weapon_threat_range(), 16);

        let character: Character = Character::builder()
            .weapon(Weapon::new(
                "".into(),
                get_weapon_base("Greatsword"),
                vec![],
            ))
            .base_apr(4)
            .extra_apr(2)
            .feats(vec![
                get_feat("Dual Wielding"),
                get_feat("Critical Immunity"),
                get_feat("Increased Multiplier"),
            ])
            .build();

        assert_eq!(character.is_dual_wielding(), true);
        assert_eq!(character.is_crit_immune(), true);
        assert_eq!(character.total_apr(), 8);
        assert_eq!(character.is_weapon_twohanded(), true);
        assert_eq!(character.weapon_crit_multiplier(), 3);
    }
}
