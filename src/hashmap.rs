use crate::data::DATA;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub fn hashmap_contains(key: &str) -> bool {
    static HASHMAP: Lazy<HashMap<&str, bool>> = Lazy::new(|| DATA.iter().copied().collect());
    HASHMAP.contains_key(key)
}
