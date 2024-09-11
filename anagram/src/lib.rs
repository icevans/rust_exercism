use std::collections::HashSet;

use itertools::Itertools;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    let target_case_normalized = word.chars().map(|c| c.to_lowercase()).flatten();
    let target_sorted_case_normalized = target_case_normalized.clone().sorted();

    for candidate in possible_anagrams {
        let candidate_case_normalized = candidate.chars().map(|c| c.to_lowercase()).flatten();

        if itertools::equal(
            target_case_normalized.clone(),
            candidate_case_normalized.clone(),
        ) {
            continue;
        }

        let candidate_sorted_case_normalized = candidate_case_normalized.sorted();

        if itertools::equal(
            target_sorted_case_normalized.clone(),
            candidate_sorted_case_normalized,
        ) {
            anagrams.insert(*candidate);
        }
    }

    anagrams
}
