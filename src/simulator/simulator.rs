use std::collections::HashMap;

use super::{
    character::{Character, CharacterBuilder},
    combat::{Combat, CombatStatistics},
    feat::feat_db::get_feat, string::align_string,
};

#[derive(Default, Debug)]
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
        string_list.push(align_string("AVERAGE DAMAGE PER ROUND", format!("{:.2}", target.dmg_dealt.total_dmg() / self.total_rounds)));

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
pub struct CombatSimulator {
    total_rounds: i32,
}

impl CombatSimulator {
    pub fn new(total_rounds: i32) -> Self {
        Self { total_rounds }
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
        target_has_epic_dodge: bool,
    ) -> DamageTestResult {
        let mut result = DamageTestResult::new();

        for target_ac in target_ac_list {
            let mut dummy = CharacterBuilder::standard_dummy(target_ac);

            if target_has_epic_dodge {
                dummy = dummy.add_feat(get_feat("Epic Dodge"));
            }

            let dummy = dummy.build();
            let round_statistics = self.begin(attacker, &dummy);

            result.statistics.insert(target_ac, round_statistics);
        }

        result.total_rounds = self.total_rounds;
        result
    }
}
