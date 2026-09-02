use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use regex::Regex;

#[derive(Debug, Clone)]
pub(crate) struct Regexes {
    pub normal: Regex,
    pub whole_word: Regex,
    pub ignore_case: Regex,
    pub whole_word_ignore_case: Regex,
}

/// Static cache where the regexes for each pattern are stored
static CACHE: LazyLock<Mutex<HashMap<String, Regexes>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// It retrieves the regexes from the cache, and if it doesn't have them, it creates them
pub(crate) fn cache_get_or_create(key: &str) -> Regexes {
    CACHE
        .lock()
        .unwrap()
        .entry(key.to_string())
        .or_insert_with(|| {
            let escaped = regex::escape(key);

            let normal = Regex::new(&escaped).unwrap();

            let whole_word = Regex::new(&format!(r"\b{}\b", escaped)).unwrap();

            let ignore_case = Regex::new(&format!(r"(?i){escaped}")).unwrap();

            let whole_word_ignore_case = Regex::new(&format!(r"(?i)\b{}\b", escaped)).unwrap();

            Regexes {
                normal,
                whole_word,
                ignore_case,
                whole_word_ignore_case,
            }
        })
        .clone()
}
