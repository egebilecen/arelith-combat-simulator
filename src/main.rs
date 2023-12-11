use simulator::{
    character::AbilityList,
    dice::Dice,
    feat::Feat,
    item::{weapon_db::get_weapon_base, Damage, DamageType, ItemProperty, Weapon},
    simulator::CombatSimulator,
};

use crate::simulator::character::Character;

mod simulator;

fn main() {
    let attacker = Character::builder()
        .name("10 sw / 15 f / 5 wm".into())
        .ab(49)
        .base_apr(4)
        .extra_apr(1)
        .abilities(AbilityList::builder().str(42).build())
        .feats(vec![
            Feat("Blind Fight".into()),
            Feat("Dual Wielding".into()),
            Feat("Improved Critical".into()),
            Feat("Increased Multiplier".into()),
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

    let simulator = CombatSimulator::new(500_000);
    let result = simulator.damage_test(&attacker, vec![35, 40, 45, 50, 55, 60, 65]);
    println!("{:#?}", result);
}
