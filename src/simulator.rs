use super::{
    character::Character,
    combat::{Combat, CombatStatistics},
    feat::feat_db::get_feat,
    string::align_string,
};
use serde::{Deserialize, Serialize};
use std::{cell::Cell, collections::HashMap};

type CombatCallbackFn = dyn Fn(&Character, &i32, &CombatStatistics) -> ();

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct DamageTestResult {
    total_rounds: i32,
    statistics: HashMap<i32, CombatStatistics>,
}

impl DamageTestResult {
    pub fn new() -> Self {
        Self::default()
    }

    fn target_ac_result_string(&self, target_ac: i32) -> String {
        let target = self.statistics.get(&target_ac);

        if target.is_none() {
            return "".into();
        }

        let target = target.unwrap();
        let mut string_list: Vec<String> = vec![];

        string_list.push(align_string("TARGET AC", target_ac.to_string()));
        string_list.push("".into());
        string_list.push(target.to_string());
        string_list.push("".into());
        string_list.push(align_string(
            "AVERAGE DAMAGE PER ROUND",
            format!("{:.2}", target.dmg_dealt.total_dmg() / self.total_rounds),
        ));

        string_list.join("\n")
    }
}

impl ToString for DamageTestResult {
    fn to_string(&self) -> String {
        let mut string_list: Vec<String> = vec![];

        let mut ac_list = self.statistics.keys().collect::<Vec<&i32>>();
        ac_list.sort();

        for (i, &&ac) in ac_list.iter().enumerate() {
            string_list.push(self.target_ac_result_string(ac));
            string_list.push("".into());

            if i != ac_list.len() - 1 {
                string_list.push("=".repeat(50));
                string_list.push("".into());
            }
        }

        string_list.join("\n")
    }
}

#[derive(Default)]
pub struct CombatSimulator<'a> {
    total_rounds: i32,
    damage_test_notifier: Cell<Option<&'a CombatCallbackFn>>,
}

impl<'a> CombatSimulator<'a> {
    pub fn new(total_rounds: i32) -> Self {
        Self {
            total_rounds,
            damage_test_notifier: Cell::new(None),
        }
    }

    pub fn begin(&self, attacker: &Character, defender: &Character) -> CombatStatistics {
        let mut statistics = CombatStatistics::new();
        let combat = Combat::new(attacker, defender);

        for _ in 1..=self.total_rounds {
            let round_statistics = combat.resolve_round();

            statistics.total_hits += round_statistics.total_hits;
            statistics.total_misses += round_statistics.total_misses;
            statistics.concealed_attacks += round_statistics.concealed_attacks;
            statistics.epic_dodged_attacks += round_statistics.epic_dodged_attacks;
            statistics.critical_hits += round_statistics.critical_hits;
            statistics.dmg_dealt.add_from(&round_statistics.dmg_dealt);
        }

        statistics
    }

    pub fn damage_test(
        &self,
        attacker: &Character,
        target_ac_list: Vec<i32>,
        target_concealment: i32,
        target_physical_immunity: i32,
        target_defensive_essence: i32,
        target_has_epic_dodge: bool,
    ) -> DamageTestResult {
        let mut result = DamageTestResult::new();

        for target_ac in target_ac_list {
            let mut dummy = Character::builder()
                .name("Combat Dummy".into())
                .ac(target_ac)
                .concealment(target_concealment)
                .physical_immunity(target_physical_immunity)
                .defensive_essence(target_defensive_essence);

            if target_has_epic_dodge {
                dummy = dummy.add_feat(get_feat("Epic Dodge"));
            }

            let dummy = dummy.build();
            let combat_statistics = self.begin(attacker, &dummy);

            if let Some(f) = self.damage_test_notifier.get() {
                f(&attacker, &target_ac, &combat_statistics);
            }

            result.statistics.insert(target_ac, combat_statistics);
        }

        result.total_rounds = self.total_rounds;
        result
    }

    pub fn set_damage_test_notifier(&self, f: &'a CombatCallbackFn) {
        self.damage_test_notifier.set(Some(f));
    }
}
