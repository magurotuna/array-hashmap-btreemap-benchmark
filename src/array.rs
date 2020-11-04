use crate::data::DATA;

#[inline]
pub fn array_contains(key: &str) -> bool {
    DATA.iter().any(|(name, _)| *name == key)
}
