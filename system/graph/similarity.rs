#[must_use]
pub fn nearest(target: &str, candidates: &[String]) -> Option<String> {
    candidates
        .iter()
        .map(|candidate| (candidate, strsim::damerau_levenshtein(target, candidate)))
        .min_by_key(|(_, distance)| *distance)
        .filter(|(_, distance)| *distance <= 3)
        .map(|(candidate, _)| format!("\n\nMaybe: \"{candidate}\"?"))
}
