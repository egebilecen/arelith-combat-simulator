use super::Feat;
use std::collections::HashMap;

const FEAT_LIST: &'static [&'static str] = &[
    "Critical Immunity",
    "Overwhelming Critical",
    "Blind Fight",
    "Bane of Enemies",
    "Dual Wielding",
    "Epic Dodge",
    "Increased Multiplier",
    "Improved Critical",
    "Ki Critical",
    "Weapon Specialization",
    "Epic Weapon Specialization",
];

pub fn get_feat_list() -> HashMap<&'static str, Feat> {
    let mut hashmap = HashMap::new();

    for feat_str in FEAT_LIST {
        hashmap.insert(*feat_str, Feat(feat_str.to_string()));
    }

    hashmap
}

pub fn get_feat(name: &str) -> Feat {
    get_feat_list().get(name).unwrap().to_owned()
}
