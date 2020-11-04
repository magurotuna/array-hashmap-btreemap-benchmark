use crate::data::DATA;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;

pub fn btreemap_contains(key: &str) -> bool {
    static BTREEMAP: Lazy<BTreeMap<&str, bool>> = Lazy::new(|| DATA.iter().copied().collect());
    BTREEMAP.contains_key(key)
}
