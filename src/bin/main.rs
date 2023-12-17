use arelith::{
    character::{AbilityList, Character},
    dice::Dice,
    feat::feat_db::get_feat,
    item::{weapon_db::get_weapon_base, Damage, DamageType, ItemProperty, Weapon},
    simulator::CombatSimulator,
};

fn main() {
    let attacker = Character::builder()
        .name("10 sw / 15 f / 5 wm".into())
        .ab(49)
        .base_apr(4)
        .extra_apr(1)
        .abilities(AbilityList::builder().str(42).build())
        .feats(vec![
            get_feat("Blind Fight"),
            get_feat("Dual Wielding"),
            get_feat("Improved Critical"),
            get_feat("Increased Multiplier"),
            get_feat("Weapon Specialization"),
            get_feat("Epic Weapon Specialization"),
        ])
        .weapon(Weapon::new(
            "M. Damask Scimitar".into(),
            get_weapon_base("Scimitar"),
            vec![
                ItemProperty::Keen,
                ItemProperty::DamageBonus(Damage::new(
                    DamageType::Slashing,
                    Dice::from(6),
                    true,
                    true,
                )),
                ItemProperty::DamageBonus(Damage::new(
                    DamageType::Slashing,
                    Dice::from(7),
                    true,
                    true,
                )),
                ItemProperty::DamageBonus(Damage::new(
                    DamageType::Sonic,
                    Dice::from("1d6"),
                    true,
                    true,
                )),
                ItemProperty::DamageBonus(Damage::new(
                    DamageType::Positive,
                    Dice::from("1d6"),
                    true,
                    true,
                )),
            ],
        ))
        .build();

    let simulator = CombatSimulator::new(10);
    let result = simulator.damage_test(&attacker, vec![35, 40, 45, 50, 55, 60, 65], false);
    println!("{}", result.to_string());
}
