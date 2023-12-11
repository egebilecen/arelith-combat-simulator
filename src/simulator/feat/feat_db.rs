use super::Feat;
use std::collections::HashMap;

// Not used at the moment for anything.
// I left it as a reference of which feats are implemented.
pub fn get_feat_list() -> HashMap<&'static str, Feat> {
    HashMap::from([
        ("Critical Immunity", Feat("Critical Immunity".into())),
        (
            "Overwhelming Critical",
            Feat("Overwhelming Critical".into()),
        ),
        ("Blind Fight", Feat("Blind Fight".into())),
        ("Bane of Enemies", Feat("Bane of Enemies".into())),
        ("Dual Wielding", Feat("Dual Wielding".into())),
        ("Epic Dodge", Feat("Epic Dodge".into())),
        ("Increased Multiplier", Feat("Increased Multiplier".into())),
        ("Improved Critical", Feat("Improved Critical".into())),
        ("Ki Critical", Feat("Ki Critical".into())),
    ])
}

pub fn get_feat(name: &str) -> Feat {
    get_feat_list().get(name).unwrap().to_owned()
}
