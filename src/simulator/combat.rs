use crate::simulator::string::align_string;
use super::{
    character::Character,
    dice::Dice,
    item::{DamageResult, ItemProperty},
};
use std::cmp::max;

#[derive(Debug, PartialEq)]
pub enum HitResult {
    Hit,
    CriticalHit,
    Miss,
    TargetConcealed,
    EpicDodged,
}

impl Default for HitResult {
    fn default() -> Self {
        Self::Miss
    }
}

impl HitResult {
    pub fn is_missed(&self) -> bool {
        match *self {
            Self::Miss | Self::TargetConcealed | Self::EpicDodged => true,
            _ => false,
        }
    }
    pub fn is_crit(&self) -> bool {
        if *self == Self::CriticalHit {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AttackType {
    MainHand,
    OffHand,
    Extra,
}

impl std::fmt::Display for AttackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AttackType::{}",
            match self {
                Self::MainHand => "MainHand",
                Self::OffHand => "OffHand",
                Self::Extra => "Extra",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct AttackInfo {
    pub ab: i32,
    pub type_: AttackType,
}

impl AttackInfo {
    pub fn new(ab: i32, type_: AttackType) -> Self {
        Self { ab, type_ }
    }
}

#[derive(Default, Debug)]
pub struct CombatStatistics {
    pub total_hits: i64,
    pub critical_hits: i64,
    pub total_misses: i64,
    pub concealed_attacks: i64,
    pub epic_dodged_attacks: i64,
    pub dmg_dealt: DamageResult,
}

impl CombatStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total_attacks(&self) -> i64 {
        self.total_hits + self.total_misses
    }
}

impl ToString for CombatStatistics {
    fn to_string(&self) -> String {
        let mut string_list: Vec<String> = vec![];

        string_list.push(align_string(
            "TOTAL ATTACK",
            self.total_attacks().to_string(),
        ));
        string_list.push(align_string("TOTAL HIT", self.total_hits.to_string()));
        string_list.push(align_string(
            "    * CRITICAL HIT",
            self.critical_hits.to_string(),
        ));
        string_list.push("".into());
        string_list.push(align_string("TOTAL MISS", self.total_misses.to_string()));
        string_list.push(align_string(
            "    * CONCEALED",
            self.concealed_attacks.to_string(),
        ));
        string_list.push(align_string(
            "    * EPIC DODGED",
            self.epic_dodged_attacks.to_string(),
        ));
        string_list.push("".into());
        string_list.push(align_string(
            "TOTAL DAMAGE",
            self.dmg_dealt.total_dmg().to_string(),
        ));

        for type_ in self.dmg_dealt.get_types_sorted() {
            string_list.push(align_string(
                format!("    * {}", type_.to_string().to_uppercase()).as_str(),
                self.dmg_dealt.get(type_).to_string(),
            ));
        }

        string_list.join("\n")
    }
}

pub struct Combat<'a> {
    attacker: &'a Character,
    defender: &'a Character,
}

impl<'a> Combat<'a> {
    pub fn new(attacker: &'a Character, defender: &'a Character) -> Self {
        Self { attacker, defender }
    }

    // Returns the final concealment of defender after various
    // factors are considered.
    fn resolve_concealment(attacker: &Character, defender: &Character) -> f32 {
        if attacker.has_blind_fight() {
            (defender.concealment.pow(2) as f32) / 100.0
        } else {
            defender.concealment as f32
        }
    }

    fn resolve_damage(
        attacker: &Character,
        defender: &Character,
        atk_info: AttackInfo,
        is_crit: bool,
    ) -> DamageResult {
        let dmg_result = DamageResult::new();

        let multiplier = if !is_crit {
            1
        } else {
            attacker.weapon_crit_multiplier()
        };

        // TODO: Get the damage type of weapon that defender has less immunity against
        //       if weapon has multiple damage types.
        // TODO: Add unarmed support. Currently if there is no weapon provided to character,
        //       Rust panics because of unwrapping weapon damage type which is null.
        let weapon_base_dmg_type = *attacker.weapon.base.damage_type.first().unwrap();

        // STR mod
        let str_mod_bonus = ((attacker.abilities.str.get_mod()
            + if attacker.is_weapon_twohanded() {
                let str_mod = attacker.abilities.str.get_mod();
                max(0, ((str_mod as f32 * 1.5) as i32) - str_mod)
            } else {
                0
            })
            / if atk_info.type_ == AttackType::OffHand {
                2
            } else {
                1
            })
            * multiplier;

        dmg_result.add(weapon_base_dmg_type, str_mod_bonus);

        // TODO: Add weapon spec., epic weapon spec. feats and implement them in here.

        // Weapon base damage
        let weapon_base_dmg = attacker.weapon.base.damage.roll_m(multiplier);
        dmg_result.add(weapon_base_dmg_type, weapon_base_dmg);

        // Weapon damage bonuses
        let _ = attacker
            .weapon
            .item_properties
            .iter()
            .filter(|x| match x {
                ItemProperty::EnchantmentBonus(_) => true,
                ItemProperty::DamageBonus(_) => true,
                ItemProperty::MassiveCrit(_) => {
                    if is_crit {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            })
            .map(|x| match x {
                ItemProperty::EnchantmentBonus(bonus) => {
                    dmg_result.add(weapon_base_dmg_type, bonus * multiplier);
                }
                ItemProperty::DamageBonus(dmg) => {
                    dmg_result.add(dmg.type_, dmg.roll_m(multiplier));
                }
                ItemProperty::MassiveCrit(dice) => {
                    dmg_result.add(weapon_base_dmg_type, dice.roll());
                }
                _ => (),
            })
            .collect::<Vec<_>>();

        // Bane of Enemies
        if attacker.has_bane_of_enemies() {
            dmg_result.add(weapon_base_dmg_type, Dice::from("2d6").roll_m(multiplier));
        }

        // Apply damage immunity and reduction
        let dmg_types = dmg_result.get_types();

        for dmg_type in dmg_types {
            let defender_dmg_immunity = defender.damage_immunity(dmg_type);
            let defender_dmg_reduction = defender.damage_reduction(dmg_type);

            if defender_dmg_immunity > 0 {
                dmg_result.sub(
                    dmg_type,
                    dmg_result.get(dmg_type) * defender_dmg_immunity / 100,
                );
            }

            if defender_dmg_reduction > 0 {
                dmg_result.sub(dmg_type, defender_dmg_reduction);
            }
        }

        dmg_result
    }

    pub fn resolve_round(&self) -> CombatStatistics {
        let mut round_statistics = CombatStatistics::default();
        let mut defender_can_epic_dodge = true;

        for atk_no in 1..=self.attacker.total_apr() {
            let atk_info = if let Some(atk_info) = self.attacker.atk_ab(atk_no) {
                atk_info
            } else {
                println!("Combat::round() - Attack info is none!");
                continue;
            };

            let defender_concealment = Self::resolve_concealment(self.attacker, self.defender);

            // Concealment check
            if defender_concealment > 0.0
                && (Dice::from("1d100").roll() as f32) < defender_concealment
            {
                round_statistics.concealed_attacks += 1;
                round_statistics.total_misses += 1;

                continue;
            }

            let hit_roll = Dice::from("1d20").roll();

            if hit_roll != 1 && (hit_roll == 20 || (atk_info.ab + hit_roll >= self.defender.ac)) {
                if self.defender.has_epic_dodge() && defender_can_epic_dodge {
                    defender_can_epic_dodge = false;

                    round_statistics.epic_dodged_attacks += 1;
                    round_statistics.total_misses += 1;

                    continue;
                }

                // Critical check
                let is_crit = if !self.defender.is_crit_immune()
                    && hit_roll >= self.attacker.weapon_threat_range()
                    && atk_info.ab + Dice::from("1d20").roll() >= self.defender.ac
                {
                    round_statistics.critical_hits += 1;
                    true
                } else {
                    false
                };

                round_statistics.total_hits += 1;

                // Calculate damage
                let dmg_result =
                    Self::resolve_damage(self.attacker, self.defender, atk_info, is_crit);

                round_statistics.dmg_dealt.add_from(&dmg_result);
            } else {
                round_statistics.total_misses += 1;
            }
        }

        round_statistics
    }
}

#[cfg(test)]
mod test {
    use crate::simulator::{
        character::{AbilityList, Character, CharacterBuilder},
        combat::{AttackInfo, AttackType, Combat},
        dice::Dice,
        feat::feat_db::get_feat,
        item::{Damage, DamageResult, DamageType, ItemProperty, Weapon, WeaponBase},
        size::SizeCategory,
    };

    #[test]
    fn combat() {
        let character: Character = Character::builder()
            .ab(50)
            .base_apr(4)
            .extra_apr(1)
            .feats(vec![])
            .build();

        assert_eq!(
            character.atk_ab(1).unwrap(),
            AttackInfo::new(50, AttackType::MainHand)
        );
        assert_eq!(
            character.atk_ab(2).unwrap(),
            AttackInfo::new(45, AttackType::MainHand)
        );
        assert_eq!(
            character.atk_ab(3).unwrap(),
            AttackInfo::new(40, AttackType::MainHand)
        );
        assert_eq!(
            character.atk_ab(4).unwrap(),
            AttackInfo::new(35, AttackType::MainHand)
        );
        assert_eq!(
            character.atk_ab(5).unwrap(),
            AttackInfo::new(50, AttackType::Extra)
        );
        assert_eq!(character.atk_ab(6), None);

        let character2 = CharacterBuilder::from(character)
            .ab(48)
            .feats(vec![get_feat("Dual Wielding")])
            .build();

        assert_eq!(
            character2.atk_ab(1).unwrap(),
            AttackInfo::new(48, AttackType::MainHand)
        );
        assert_eq!(
            character2.atk_ab(2).unwrap(),
            AttackInfo::new(43, AttackType::MainHand)
        );
        assert_eq!(
            character2.atk_ab(3).unwrap(),
            AttackInfo::new(38, AttackType::MainHand)
        );
        assert_eq!(
            character2.atk_ab(4).unwrap(),
            AttackInfo::new(33, AttackType::MainHand)
        );
        assert_eq!(
            character2.atk_ab(5).unwrap(),
            AttackInfo::new(50, AttackType::Extra)
        );
        assert_eq!(
            character2.atk_ab(6).unwrap(),
            AttackInfo::new(48, AttackType::OffHand)
        );
        assert_eq!(
            character2.atk_ab(7).unwrap(),
            AttackInfo::new(43, AttackType::OffHand)
        );
        assert_eq!(character2.atk_ab(8), None);

        let attacker = Character::builder()
            .ab(50)
            .feats(vec![get_feat("Blind Fight")])
            .build();

        let defender = Character::builder().concealment(50).build();
        assert_eq!(Combat::resolve_concealment(&attacker, &defender), 25.0);

        let defender = Character::builder().concealment(25).build();
        assert_eq!(Combat::resolve_concealment(&attacker, &defender), 6.25);

        let defender = Character::builder().concealment(0).build();
        assert_eq!(Combat::resolve_concealment(&attacker, &defender), 0.0);
    }

    #[test]
    fn damage() {
        let attacker = Character::builder()
            .abilities(AbilityList::builder().str(38).build())
            .weapon(Weapon::new(
                "".into(),
                WeaponBase::new(
                    "".into(),
                    SizeCategory::Medium,
                    Dice::from(6),
                    18,
                    2,
                    vec![DamageType::Slashing],
                ),
                vec![
                    ItemProperty::Keen,
                    ItemProperty::EnchantmentBonus(4),
                    ItemProperty::DamageBonus(Damage::new(
                        DamageType::Divine,
                        Dice::from(4),
                        true,
                        true,
                    )),
                    ItemProperty::MassiveCrit(Dice::from(6)),
                ],
            ))
            .feats(vec![get_feat("Increased Multiplier")])
            .build();

        let defender = Character::builder()
            .physical_immunity(10)
            .physical_damage_reduction(5)
            .build();

        let round_result = Combat::resolve_damage(
            &attacker,
            &defender,
            AttackInfo::new(50, AttackType::MainHand),
            false,
        );

        assert_eq!(round_result.get(DamageType::Slashing), 17);
        assert_eq!(round_result.get(DamageType::Divine), 4);
        assert_eq!(round_result.total_dmg(), 21);

        let round_result = Combat::resolve_damage(
            &attacker,
            &defender,
            AttackInfo::new(50, AttackType::MainHand),
            true,
        );

        assert_eq!(round_result.get(DamageType::Slashing), 66);
        assert_eq!(round_result.get(DamageType::Divine), 12);
        assert_eq!(round_result.total_dmg(), 78);

        // Test twohand weapon damage bonus
        let attacker = Character::builder()
            .abilities(AbilityList::builder().str(38).build())
            .weapon(Weapon::new(
                "".into(),
                WeaponBase::new(
                    "".into(),
                    SizeCategory::Large,
                    Dice::from(6),
                    18,
                    2,
                    vec![DamageType::Slashing],
                ),
                vec![
                    ItemProperty::Keen,
                    ItemProperty::EnchantmentBonus(4),
                    ItemProperty::DamageBonus(Damage::new(
                        DamageType::Divine,
                        Dice::from(4),
                        true,
                        true,
                    )),
                    ItemProperty::MassiveCrit(Dice::from(6)),
                ],
            ))
            .feats(vec![get_feat("Increased Multiplier")])
            .build();

        let defender = Character::builder()
            .physical_immunity(0)
            .physical_damage_reduction(0)
            .build();

        let round_result = Combat::resolve_damage(
            &attacker,
            &defender,
            AttackInfo::new(50, AttackType::MainHand),
            false,
        );

        assert_eq!(round_result.get(DamageType::Slashing), 31);

        // Test offhand damage penalty
        let round_result = Combat::resolve_damage(
            &attacker,
            &defender,
            AttackInfo::new(50, AttackType::OffHand),
            false,
        );

        assert_eq!(round_result.get(DamageType::Slashing), 20);

        let mut dmg1 = DamageResult::new();
        dmg1.add(DamageType::Acid, 4);
        dmg1.add(DamageType::Bludgeoning, 6);

        assert_eq!(dmg1.total_dmg(), 10);

        let dmg2 = DamageResult::new();
        dmg2.add(DamageType::Cold, 2);
        dmg2.add(DamageType::Divine, 1);

        dmg1.add_from(&dmg2);

        assert_eq!(dmg1.get(DamageType::Cold), 2);
        assert_eq!(dmg1.get(DamageType::Divine), 1);
        assert_eq!(dmg1.total_dmg(), 13);
    }
}
