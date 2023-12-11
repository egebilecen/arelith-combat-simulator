use std::collections::HashMap;

use super::{
    character::{Character, CharacterBuilder},
    combat::{Combat, CombatStatistics}, feat::feat_db::get_feat,
};

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

    pub fn damage_test(&self, attacker: &Character, target_ac_list: Vec<i32>, target_has_epic_dodge: bool) -> HashMap<i32, CombatStatistics> {
        let mut statistics = HashMap::new();

        for target_ac in target_ac_list {
            let mut dummy = CharacterBuilder::standard_dummy(target_ac);

            if target_has_epic_dodge {
                dummy = dummy.add_feat(get_feat("Epic Dodge"));
            }

            let dummy = dummy.build();
            let round_statistics = self.begin(attacker, &dummy);

            statistics.insert(target_ac, round_statistics);
        }

        statistics
    }
}
