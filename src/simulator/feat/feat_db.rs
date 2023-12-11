use super::Feat;
use std::collections::HashMap;

// Not used at the moment for anything. 
// I left it as a reference of which feats are implemented.
pub fn get_feat_list() -> HashMap<String, Feat> {
    HashMap::from([
        ("Critical Immunity".into(), Feat("Critical Immunity".into())),
        (
            "Overwhelming Critical".into(),
            Feat("Overwhelming Critical".into()),
        ),
        ("Blind Fight".into(), Feat("Blind Fight".into())),
        ("Bane of Enemies".into(), Feat("Bane of Enemies".into())),
        ("Dual Wielding".into(), Feat("Dual Wielding".into())),
        ("Epic Dodge".into(), Feat("Epic Dodge".into())),
        ("Increased Multiplier".into(), Feat("Increased Multiplier".into())),
        ("Improved Critical".into(), Feat("Improved Critical".into())),
        ("Ki Critical".into(), Feat("Ki Critical".into())),
    ])
}

pub fn get_feat(name: &str) -> Feat {
    get_feat_list().get(name).unwrap().to_owned()
}
