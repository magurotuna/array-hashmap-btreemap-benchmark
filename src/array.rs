use crate::data::DATA;

#[inline]
pub fn array_contains(key: &str) -> bool {
    DATA.iter().any(|(name, _)| *name == key)
}

#[inline]
pub fn array_contains_binary_search(key: &str) -> bool {
    let uppercased_key = key.to_ascii_uppercase();
    let result = DATA.binary_search_by(|probe| probe.0.to_ascii_uppercase().cmp(&uppercased_key));
    match result {
        Err(_) => false,
        Ok(idx) => key == DATA[idx].0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert!(array_contains_binary_search("AbortController"));
        assert!(array_contains_binary_search("MessageEvent"));
        assert!(array_contains_binary_search("WritableStream"));
        assert!(!array_contains_binary_search("WRITABLESTREAM"));
        assert!(!array_contains_binary_search("UNEXISTENT_ELEMENT"));
    }
}
